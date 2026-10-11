//! A group name index over the existing `List of Groups` snapshot answer.
//!
//! The request is [`super::render_native_group_snapshot_request`], unchanged:
//! it already fetches `GUID`, `ALTERID` and `PARENT`, and the response carries
//! each group's `LANGUAGENAME.LIST`, whose `NAME.LIST` holds the stored name
//! first and any aliases after it (measured on the Shape Lab capture: `Bank OD
//! A/c` lists `Bank OCC A/c` as a second `NAME`). The row's `NAME=` attribute is
//! deliberately not read, because it can differ from the stored name.
//!
//! The envelope, `STATUS`, collection scoping and the per-row company binding
//! are the snapshot parser's own ([`super::wire::walk_group_snapshot`]); this
//! module only adds the identity checks the index needs and refuses, with a
//! typed variant, every row it cannot admit.

use std::collections::HashMap;

use super::model::NativeOutstandingsError;
use super::wire::{
    walk_group_snapshot, GroupRowIdentity, GROUP_COMPANY_GUID_MISMATCH, GROUP_COMPANY_GUID_MISSING,
};
use crate::PartyLedgerMasterFieldObservation;

/// One group of the index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupNameRow {
    /// Every `NAME` of every `NAME.LIST` under the row's `LANGUAGENAME.LIST`
    /// elements, in document order: the stored name first, then the aliases.
    /// Never empty, and no entry is empty.
    pub names: Vec<String>,
    /// The group's own `GUID`, non-empty and unique across the index.
    pub guid: String,
    /// The group's `ALTERID`.
    pub alter_id: u64,
    /// The group's `PARENT` as the snapshot parser reads it; `None` when
    /// Tally wrote it empty. A top-level group's parent is Tally's reserved
    /// primary marker, which the sanitiser renders with a leading
    /// replacement character, exactly as `parse_native_group_snapshot` does.
    pub parent: Option<String>,
}

/// Why a group answer cannot be admitted as a name index. `row` is the 1-based
/// position of the `GROUP` row in document order: a number, never a name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupNameIndexError {
    /// The response is not a usable Group collection answer (malformed,
    /// unterminated, `STATUS` failure or absent, no collection, or a row
    /// without its `NAME` or `PARENT`): the snapshot parser's own refusal.
    Response(NativeOutstandingsError),
    /// A row's request-computed `BRIDGECOMPANYGUID` is absent or empty, or
    /// the collection holds no rows to carry one.
    CompanyGuidMissing,
    /// A row's `BRIDGECOMPANYGUID` is not the requested company's.
    CompanyGuidMismatch,
    /// The row has no `GUID` child, or it is empty.
    GuidMissing { row: usize },
    /// The row has more than one `GUID` child.
    GuidRepeatedInRow { row: usize },
    /// The `GUID` of row `row` is the one row `first_row` already carries
    /// (compared ignoring ASCII case).
    DuplicateGuid { first_row: usize, row: usize },
    /// The row has no `ALTERID` child, or it is empty.
    AlterIdMissing { row: usize },
    /// The row has more than one `ALTERID` child.
    AlterIdRepeatedInRow { row: usize },
    /// The row's `ALTERID` is not an unsigned decimal integer that fits `u64`.
    AlterIdNotNumeric { row: usize },
    /// The row has no `NAME` under any `NAME.LIST` of a `LANGUAGENAME.LIST`.
    NoNames { row: usize },
    /// A `NAME` of the row is empty or whitespace.
    EmptyName { row: usize },
}

/// Parses a `List of Groups` answer into the group name index.
///
/// `expected_company_guid` binds the response exactly as in
/// [`super::parse_native_group_snapshot`]. Every row must carry a unique
/// non-empty `GUID`, a numeric `ALTERID` and at least one non-empty name;
/// otherwise the whole answer is refused, because an index with a hole would
/// let a caller conclude that a group does not exist.
pub fn parse_group_name_index(
    xml: &str,
    expected_company_guid: &str,
) -> Result<Vec<GroupNameRow>, GroupNameIndexError> {
    let walked = walk_group_snapshot(xml, expected_company_guid).map_err(|error| match error {
        NativeOutstandingsError::InvalidResponse(GROUP_COMPANY_GUID_MISSING) => {
            GroupNameIndexError::CompanyGuidMissing
        }
        NativeOutstandingsError::InvalidResponse(GROUP_COMPANY_GUID_MISMATCH) => {
            GroupNameIndexError::CompanyGuidMismatch
        }
        other => GroupNameIndexError::Response(other),
    })?;

    let mut first_row_of_guid: HashMap<String, usize> = HashMap::with_capacity(walked.len());
    let mut index = Vec::with_capacity(walked.len());
    for (position, walked_row) in walked.into_iter().enumerate() {
        let row = position + 1;
        let GroupRowIdentity {
            guids,
            alter_ids,
            names,
        } = walked_row.row.identity;

        let guid = single(guids, row, |row| GroupNameIndexError::GuidRepeatedInRow {
            row,
        })?;
        if guid.is_empty() {
            return Err(GroupNameIndexError::GuidMissing { row });
        }
        if let Some(&first_row) = first_row_of_guid.get(&guid.to_ascii_lowercase()) {
            return Err(GroupNameIndexError::DuplicateGuid { first_row, row });
        }
        first_row_of_guid.insert(guid.to_ascii_lowercase(), row);

        let alter_id = single(alter_ids, row, |row| {
            GroupNameIndexError::AlterIdRepeatedInRow { row }
        })?;
        if alter_id.is_empty() {
            return Err(GroupNameIndexError::AlterIdMissing { row });
        }
        let alter_id =
            parse_alter_id(&alter_id).ok_or(GroupNameIndexError::AlterIdNotNumeric { row })?;

        if names.is_empty() {
            return Err(GroupNameIndexError::NoNames { row });
        }
        if names.iter().any(|name| name.trim().is_empty()) {
            return Err(GroupNameIndexError::EmptyName { row });
        }

        let parent = match walked_row.row.record.parent {
            PartyLedgerMasterFieldObservation::Returned(text) if !text.is_empty() => Some(text),
            _ => None,
        };
        index.push(GroupNameRow {
            names,
            guid,
            alter_id,
            parent,
        });
    }
    Ok(index)
}

/// The one value of a child that must appear at most once; an absent child is
/// the empty string, which the caller refuses as missing.
fn single(
    mut values: Vec<String>,
    row: usize,
    repeated: impl FnOnce(usize) -> GroupNameIndexError,
) -> Result<String, GroupNameIndexError> {
    if values.len() > 1 {
        return Err(repeated(row));
    }
    Ok(values.pop().unwrap_or_default())
}

/// An unsigned decimal integer, as Tally writes it (a leading space is part of
/// the capture): digits only, so `+5`, `-5`, `5.0` and `NaN` are refused.
fn parse_alter_id(text: &str) -> Option<u64> {
    let digits = text.trim();
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}
