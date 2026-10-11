//! A group name index over the existing `List of Groups` snapshot answer.
//!
//! The request is [`super::render_native_group_snapshot_request`], unchanged:
//! it already fetches `GUID`, `ALTERID` and `PARENT`, and the response carries
//! each group's `LANGUAGENAME.LIST`, whose `NAME.LIST` holds the stored name
//! first and any aliases after it (measured on the Shape Lab capture: `Bank OD
//! A/c` lists `Bank OCC A/c` as a second `NAME`). Names are read only from
//! `NAME.LIST`; the row's `NAME=` attribute is cross-checked against the stored
//! name and a disagreement is refused. Every capture agrees, so a disagreement
//! is a shape not yet observed.
//!
//! The envelope, `STATUS`, collection scoping and the per-row company binding
//! are the snapshot parser's own ([`super::wire::walk_group_snapshot`], in its
//! stricter `NameIndex` mode); this module adds the identity checks the index
//! needs and refuses, with a typed variant, every row or shape it cannot admit.
//! The snapshot readers are not made stricter by any of this.

use std::collections::HashMap;

use super::model::NativeOutstandingsError;
use super::wire::{
    walk_group_snapshot, GroupRowIdentity, GroupWalkMode, GROUP_COLLECTION_EMPTY,
    GROUP_COMPANY_GUID_MISMATCH, GROUP_COMPANY_GUID_MISSING, GROUP_OUTSIDE_COLLECTION,
};
use crate::PartyLedgerMasterFieldObservation;

/// One group of the index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupNameRow {
    /// Every `NAME` of every `NAME.LIST` under the row's `LANGUAGENAME.LIST`
    /// elements, in document order: the stored name first, then the aliases.
    /// Never empty, and no entry is empty. The names are verbatim, as Tally
    /// wrote them: the caller folds case and whitespace when it matches.
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
    /// A row's request-computed `BRIDGECOMPANYGUID` is absent or empty.
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
    /// The row's `NAME=` attribute is not its first (stored) name.
    NameAttributeDisagrees { row: usize },
    /// The row holds names in a shape no capture shows, where reading around
    /// it would drop a name silently.
    UnrecognisedNameShape { row: usize, shape: NameShape },
    /// An element no capture shows holds a `GROUP`, `GUID`, `NAME`,
    /// `NAME.LIST` or `LANGUAGENAME.LIST` somewhere inside it (in the row or in
    /// its language list), where a name or identity could be read around; or a
    /// `GROUP` sits directly inside the row.
    HiddenInUnknownElement { row: usize },
    /// A child the captures show only as a leaf (`RESERVEDNAME`,
    /// `LANGUAGEID` and the like) holds elements, which the index does not read.
    CapturedChildNotLeaf { row: usize },
    /// A name holds U+FFFD. The parser replaces a forbidden numeric character
    /// reference with it, so the stored text is not what Tally wrote.
    NameNotRepresentable { row: usize },
    /// A `GROUP` element sits outside `ENVELOPE/BODY/DATA/COLLECTION` (the
    /// `CMPINFO` counter excepted), so the rows read would be fewer than sent.
    GroupOutsideCollection,
    /// The collection holds no `GROUP` rows at all.
    EmptyCollection,
}

/// A name-bearing shape the index refuses rather than skips.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameShape {
    /// A `NAME` directly under `GROUP`.
    NameDirectlyUnderRow,
    /// A `NAME.LIST` directly under `GROUP`, outside any `LANGUAGENAME.LIST`.
    NameListOutsideLanguageList,
    /// A `NAME` directly under `LANGUAGENAME.LIST`, outside any `NAME.LIST`.
    NameDirectlyUnderLanguageList,
    /// An element other than `NAME` inside a `NAME.LIST`.
    ElementOtherThanNameInNameList,
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
    let walked = walk_group_snapshot(xml, expected_company_guid, GroupWalkMode::NameIndex)
        .map_err(|error| match error {
            NativeOutstandingsError::InvalidResponse(GROUP_COMPANY_GUID_MISSING) => {
                GroupNameIndexError::CompanyGuidMissing
            }
            NativeOutstandingsError::InvalidResponse(GROUP_COMPANY_GUID_MISMATCH) => {
                GroupNameIndexError::CompanyGuidMismatch
            }
            NativeOutstandingsError::InvalidResponse(GROUP_OUTSIDE_COLLECTION) => {
                GroupNameIndexError::GroupOutsideCollection
            }
            NativeOutstandingsError::InvalidResponse(GROUP_COLLECTION_EMPTY) => {
                GroupNameIndexError::EmptyCollection
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
            unrecognised_name_shape,
            hidden_in_unknown_element,
            non_leaf_captured_child,
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

        if hidden_in_unknown_element {
            return Err(GroupNameIndexError::HiddenInUnknownElement { row });
        }
        if non_leaf_captured_child {
            return Err(GroupNameIndexError::CapturedChildNotLeaf { row });
        }
        if let Some(shape) = unrecognised_name_shape {
            return Err(GroupNameIndexError::UnrecognisedNameShape { row, shape });
        }
        if names.is_empty() {
            return Err(GroupNameIndexError::NoNames { row });
        }
        if names.iter().any(|name| name.trim().is_empty()) {
            return Err(GroupNameIndexError::EmptyName { row });
        }
        if names.iter().any(|name| name.contains('\u{fffd}')) {
            return Err(GroupNameIndexError::NameNotRepresentable { row });
        }
        if names[0] != walked_row.row.record.name {
            return Err(GroupNameIndexError::NameAttributeDisagrees { row });
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
