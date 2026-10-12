//! The recheck a master post makes inside the endpoint queue, after approval
//! and before the intent: the one place that decides whether a ledger name is
//! already taken.
//!
//! A master post reads the company's ledger index and group index twice each,
//! between the all-company mark read as the binding reads begin and the aim
//! snapshot read last before the POST. This module takes those answers and
//! refuses unless they are consistent, parse, show the proposed name resolves
//! to no existing ledger or group (stored name or alias, ledgers and groups
//! sharing one namespace), and the company's master mark (ALTMSTID) did not
//! move across the reads.
//!
//! Why a new file: the voucher recheck in `agent_import_post.rs` is a pinned
//! admission file already over three thousand lines, and shares nothing with
//! this decision but the aim check, which it exposes and this module calls.
//!
//! [`recheck_master_admission`] is the closure body a masters caller passes to
//! the queue. It reads nothing and sends nothing; [`admit_master_create`]
//! below it is pure over parsed indexes, so it is tested on constructed ones.
use super::master_ledger::MasterMark;
use super::post::admit_queued_aim;
use crate::agent::change_parse::parse_all_company_marks;
use crate::tally::approved_import::{ApprovedImportAdmissionError, QueuedAdmission};
use crate::tally::ledger_create_item::{LedgerCreateItem, RenderedLedgerCreate};
use crate::tally::VerifiedCompanyIdentity;
use bridge_tally_core::master_name_index::{
    IndexedMaster, MasterKind, MasterNameIndex, MasterNameIndexError,
};
use bridge_tally_protocol::native_outstandings::{
    parse_group_name_index, GroupNameIndexError, GroupNameRow,
};
use bridge_tally_protocol::StandardLedgerCatalogError;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The request a master approval is held with: the item and the request
/// rendered from it, once. It is the masters twin of `NativePostRequest` in the
/// approval slot, and carries no company: the rendered bytes name it, and the
/// approval binding holds the verified tuple.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MasterPostRequest {
    item: LedgerCreateItem,
    rendered: RenderedLedgerCreate,
}

impl MasterPostRequest {
    /// Renders `item` for `company`, once.
    pub(super) fn render(item: LedgerCreateItem, company: &VerifiedCompanyIdentity) -> Self {
        let rendered = item.render_import(company);
        Self { item, rendered }
    }

    #[cfg(test)]
    pub(super) fn item(&self) -> &LedgerCreateItem {
        &self.item
    }

    /// The rendered request, as the POST will send it.
    pub(super) fn xml(&self) -> &str {
        self.rendered.xml()
    }

    /// Lowercase hex digest of [`Self::xml`].
    pub(super) fn sha256(&self) -> &str {
        self.rendered.sha256()
    }
}

/// There is no parser for the ledger name index answer. The request exists
/// (the V3 catalogue profile), but no V3 answer has been captured from a real
/// Tally, so a parser would be written against a shape nobody has seen
/// (AGENTS.md P1). Until the capture exists a master post is always refused
/// here, before the intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LedgerIndexNotCaptured;

/// Parses a V3 ledger catalogue answer into the ledgers of the name index.
/// Always [`LedgerIndexNotCaptured`] today; see its documentation.
pub(super) fn parse_ledger_name_index(
    _xml: &str,
    _expected_company_guid: &str,
) -> Result<Vec<IndexedMaster>, LedgerIndexNotCaptured> {
    // The one seam: a unit test that must get past the missing parser scopes
    // the ledgers it wants the book to hold. It is compiled under bare
    // `cfg(test)`, so no shipped build holds it (`scripts/check-no-test-seam.mjs`
    // scans the executables).
    #[cfg(test)]
    if let Ok(injected) = TEST_LEDGER_INDEX.try_with(Clone::clone) {
        return Ok(injected);
    }
    Err(LedgerIndexNotCaptured)
}

#[cfg(test)]
tokio::task_local! {
    /// The ledgers a test says the book holds, standing in for the parser that
    /// does not exist yet. Scoped around the call under test; absent, the
    /// parser refuses as it does in a shipped build.
    pub(super) static TEST_LEDGER_INDEX: Vec<IndexedMaster>;
}

fn group_masters(rows: Vec<GroupNameRow>) -> Vec<IndexedMaster> {
    rows.into_iter()
        .map(|row| IndexedMaster {
            kind: MasterKind::Group,
            guid: row.guid,
            alter_id: row.alter_id,
            names: row.names,
        })
        .collect()
}

/// The cause, with no book data in it, a group index refusal carries: the snapshot parser's own
/// code when the answer was not a usable collection, else one code for every
/// row-level refusal (their variants carry row numbers, never names).
fn group_cause(error: &GroupNameIndexError) -> Option<&'static str> {
    match error {
        GroupNameIndexError::Response(inner) => {
            crate::tally::approved_import::group_snapshot_cause(inner)
        }
        _ => Some("group_name_index_invalid"),
    }
}

/// The queue's recheck of a master post. Returns the pre-image (every indexed
/// master's lowercased GUID with its `ALTERID`) for the verification that
/// follows the POST.
///
/// Refused with a typed error, never a panic: a voucher admission handed to
/// this recheck is [`ApprovedImportAdmissionError::AdmissionInconsistent`]; a
/// pair of reads that differ, or an index that cannot be built, is
/// `CatalogueUnreadable`; an unparsable group answer is `GroupExportInvalid`;
/// the ledger index is always `MasterLedgerIndexUnavailable` until its parser
/// exists.
pub(super) fn recheck_master_admission(
    queued: QueuedAdmission<'_>,
) -> anyhow::Result<BTreeMap<String, u64>> {
    let QueuedAdmission::Masters {
        item,
        company,
        ledgers_first,
        ledgers_second,
        groups_first,
        groups_second,
        company_marks_at_binding,
        company_marks,
    } = queued
    else {
        return Err(ApprovedImportAdmissionError::AdmissionInconsistent.into());
    };
    let company_guid = company.company_guid();
    if ledgers_first != ledgers_second || groups_first != groups_second {
        return Err(ApprovedImportAdmissionError::CatalogueUnreadable(
            StandardLedgerCatalogError::MalformedResponse,
        )
        .into());
    }
    let (ledgers, groups) = parse_indexes(ledgers_first, groups_first, company_guid)?;
    admit_master_create(
        ledgers,
        groups,
        item,
        company_marks_at_binding,
        company_marks,
        company,
    )
}

/// Parses the two index answers of `company_guid`: the groups first, so an
/// unusable group answer is named before the missing ledger parser is.
fn parse_indexes(
    ledgers_xml: &str,
    groups_xml: &str,
    company_guid: &str,
) -> anyhow::Result<(Vec<IndexedMaster>, Vec<IndexedMaster>)> {
    let groups = parse_group_name_index(groups_xml, company_guid).map_err(|error| {
        ApprovedImportAdmissionError::GroupExportInvalid {
            cause: group_cause(&error),
        }
    })?;
    let ledgers =
        parse_ledger_name_index(ledgers_xml, company_guid).map_err(|LedgerIndexNotCaptured| {
            ApprovedImportAdmissionError::MasterLedgerIndexUnavailable
        })?;
    Ok((ledgers, group_masters(groups)))
}

/// The check a master tool makes before it saves or asks anything: the proposed
/// name resolves to no ledger or group in the index answers just read. It is
/// the queue's name decision without the aim, which only the queue can make.
/// Typed refusals as [`recheck_master_admission`] gives them.
pub(super) fn check_name_free(
    ledgers_xml: &str,
    groups_xml: &str,
    item: &LedgerCreateItem,
    company_guid: &str,
) -> anyhow::Result<()> {
    let (ledgers, groups) = parse_indexes(ledgers_xml, groups_xml, company_guid)?;
    admit_free_name(ledgers, groups, item).map(|_| ())
}

/// What the queue's recheck leaves for `before_dispatch` to record: the books
/// as the aim snapshot showed them, the last read before the POST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MasterAim {
    /// Every indexed master's lowercased GUID with its `ALTERID`.
    pub(super) pre_image: BTreeMap<String, u64>,
    /// Every loaded company's master mark (`ALTMSTID`) in that snapshot.
    pub(super) marks: Vec<MasterMark>,
}

/// The recheck closure body of a master post: [`recheck_master_admission`], then
/// the aim it left, set into `aim` for the intent. The queue's recheck returns
/// `()` and its signature stays that way; this is the side channel, as the
/// voucher path's `aimed_mark` is. `aim` is written only when the recheck
/// admitted the post, and a recheck that refuses leaves it as it was.
pub(super) fn recheck_master_for_dispatch(
    queued: QueuedAdmission<'_>,
    aim: &Mutex<Option<MasterAim>>,
) -> anyhow::Result<()> {
    let QueuedAdmission::Masters { company_marks, .. } = &queued else {
        return Err(ApprovedImportAdmissionError::AdmissionInconsistent.into());
    };
    let marks_text = (*company_marks).to_string();
    let pre_image = recheck_master_admission(queued)?;
    let marks = aim_master_marks(&marks_text)?;
    *aim.lock()
        .map_err(|_| ApprovedImportAdmissionError::AdmissionInconsistent)? =
        Some(MasterAim { pre_image, marks });
    Ok(())
}

/// Every loaded company's master mark in the aim snapshot. A snapshot that does
/// not parse, or whose GUIDs are not distinct, cannot be recorded: the recheck
/// has already confirmed the same text for the target, so this is a fault, not
/// a refusal a caller can act on.
fn aim_master_marks(company_marks: &str) -> anyhow::Result<Vec<MasterMark>> {
    let rows = parse_all_company_marks(company_marks)
        .map_err(|_| ApprovedImportAdmissionError::CompanyScopeUnconfirmed)?;
    Ok(rows
        .into_iter()
        .map(|row| MasterMark {
            company_guid: row.guid.to_ascii_lowercase(),
            altmstid: row.masters,
        })
        .collect())
}

/// The decision, over parsed indexes: no master holds the name in the one namespace
/// ledgers and groups share, then the company's master mark did not move
/// between the binding snapshot and the aim snapshot (`admit_queued_aim`).
/// The first collision, in ledger-then-group order, names the refusal.
pub(super) fn admit_master_create(
    ledgers: Vec<IndexedMaster>,
    groups: Vec<IndexedMaster>,
    item: &LedgerCreateItem,
    company_marks_at_binding: &str,
    company_marks: &str,
    company: &VerifiedCompanyIdentity,
) -> anyhow::Result<BTreeMap<String, u64>> {
    let index = admit_free_name(ledgers, groups, item)?;
    admit_queued_aim(
        company_marks_at_binding,
        company_marks,
        company.company_guid(),
        company.display_name(),
    )?;
    // The pre-image is returned, not kept: [`recheck_master_for_dispatch`]
    // carries it from the queue's recheck closure to `before_dispatch`, which
    // writes it as the sidecar its intent names.
    Ok(index.pre_image())
}

/// The name decision over parsed indexes: the one namespace ledgers and groups
/// share holds no master the item's name could resolve to. Returns the index,
/// whose pre-image the dispatch records.
fn admit_free_name(
    ledgers: Vec<IndexedMaster>,
    groups: Vec<IndexedMaster>,
    item: &LedgerCreateItem,
) -> anyhow::Result<MasterNameIndex> {
    let mut masters = ledgers;
    masters.extend(groups);
    let index = MasterNameIndex::new(masters).map_err(|error| {
        ApprovedImportAdmissionError::CatalogueUnreadable(match error {
            MasterNameIndexError::DuplicateGuid { .. } => {
                StandardLedgerCatalogError::DuplicateIdentity
            }
            _ => StandardLedgerCatalogError::MalformedResponse,
        })
    })?;
    // The name checked is the item's own typed name, the one the POST carries.
    // A blank candidate is the only error, and a `MasterName` is never blank:
    // reaching it is a wiring fault.
    let collisions = index
        .collisions(item.name().as_str())
        .map_err(|_| ApprovedImportAdmissionError::AdmissionInconsistent)?;
    if let Some(taken) = collisions.first() {
        return Err(ApprovedImportAdmissionError::MasterNameTaken {
            kind: taken.kind,
            via: taken.via,
        }
        .into());
    }
    Ok(index)
}

#[cfg(test)]
#[path = "agent_master_post_tests.rs"]
mod tests;
