//! The rechecks a master post makes after approval and before the intent: the
//! ledger path decides whether a ledger or group name is already taken, and the
//! stock twin rebuilds an approved stock draft from fresh admitted stock reads,
//! which [`recheck_shown_stock_item`] has the runtime's stock coordinator make
//! in one queued execution. Nothing shipped calls the stock twin yet.
//!
//! A master post reads the company's ledger index and group index twice each,
//! between the all-company mark read as the binding reads begin and the aim
//! snapshot read last before the POST. This module takes those answers and
//! refuses unless they are consistent, parse, show the proposed name resolves
//! to no existing ledger or group (stored name, alias or the row's own
//! spelling; a ledger named like a group is refused by choice, though Tally
//! would create it), and the company's master mark (ALTMSTID) did not move
//! across the reads.
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
use crate::agent::change_parse::{parse_all_company_marks, LoadedCompanyMarks};
use crate::tally::approved_import::{ApprovedImportAdmissionError, QueuedAdmission};
use crate::tally::ledger_create_item::{LedgerCreateBatch, LedgerCreateItem, RenderedLedgerCreate};
use crate::tally::runtime::{
    AdmittedStockGroups, AdmittedStockUnits, StockItemReadKind, StockItemRecheckReads, TallyRuntime,
};
use crate::tally::stock_item_create::{
    CompleteStockItemCatalogue, DraftRefusal, GoodsSibling, ItemNameNotAbsent, NotGoodsSibling,
    ShownDraft, SiblingGst, SiblingRefusal, StockItemCatalogueError, StockItemDraft,
};
use crate::tally::{TallyConfig, VerifiedCompanyIdentity};
use bridge_tally_core::master_name_index::{
    IndexedMaster, MasterKind, MasterNameIndex, MasterNameIndexError,
};
use bridge_tally_protocol::native_outstandings::{
    parse_group_name_index, GroupNameIndexError, GroupNameRow,
};
use bridge_tally_protocol::{parse_ledger_name_index, StandardLedgerCatalogError};
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The fresh stock reads and marks for one already approved stock draft.
struct StockItemRecheckInput<'a, 'draft> {
    shown: &'a ShownDraft<'draft>,
    candidate: &'a CompleteStockItemCatalogue,
    groups: &'a AdmittedStockGroups,
    units: &'a AdmittedStockUnits,
    company_marks_at_binding: &'a str,
    company_marks: &'a str,
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum StockItemRecheckError {
    #[error("stock reads are for another company")]
    OtherCompany,
    #[error("stock item name is not absent")]
    NameNotAbsent(ItemNameNotAbsent),
    #[error("stock item sibling is refused")]
    Sibling(SiblingRefusal),
    #[error("stock item sibling is not the measured goods case")]
    NotGoods(NotGoodsSibling),
    #[error("stock item draft is refused")]
    Draft(DraftRefusal),
    #[error("stock item catalogue is refused")]
    Catalogue(StockItemCatalogueError),
    #[error("stock item draft changed")]
    DraftChanged,
    #[error("stock item read was sized at another master mark")]
    SizedAtAnotherMark {
        read: StockItemReadKind,
        sized: u64,
        binding: u64,
    },
}

/// Reads the stock reads for `shown` in one queued execution
/// ([`TallyRuntime::read_and_recheck_stock_item`]) and rechecks it on them
/// ([`recheck_stock_item_admission`]). The reads are sized at the target's own
/// binding mark ([`target_binding_mark`]). It returns no approval or dispatch
/// value, sends no import, and nothing shipped calls it.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) async fn recheck_shown_stock_item(
    runtime: &TallyRuntime,
    config: TallyConfig,
    shown: &ShownDraft<'_>,
) -> anyhow::Result<()> {
    let company = shown.draft().sibling().sibling().catalogue().company();
    let marks = crate::tally::agent_read_request::AgentReadRequest::parse(
        crate::agent::read_profiles::render_agent_company_high_water(company.display_name()),
    )?;
    let binding_mark = |text: &str| {
        let rows = parse_all_company_marks(text)
            .map_err(|_| ApprovedImportAdmissionError::MastersUnconfirmed)?;
        target_binding_mark(&rows, company)
    };
    let recheck = |reads: StockItemRecheckReads| {
        let (candidate, groups, units, at_binding, at_aim) = reads.into_parts();
        let candidate = CompleteStockItemCatalogue::from_admitted(candidate)
            .map_err(StockItemRecheckError::Catalogue)?;
        recheck_stock_item_admission(StockItemRecheckInput {
            shown,
            candidate: &candidate,
            groups: &groups,
            units: &units,
            company_marks_at_binding: &at_binding,
            company_marks: &at_aim,
        })
    };
    runtime
        .read_and_recheck_stock_item(config, company, marks, binding_mark, recheck)
        .await
}

/// The target's master mark in a binding snapshot: that of the one row whose
/// GUID and name both match `company`, not the first row with its GUID, because
/// a second loaded company can share a GUID under another name. The stock
/// coordinator sizes its reads at it and the stock recheck compares them to it.
fn target_binding_mark(
    rows: &[LoadedCompanyMarks],
    company: &VerifiedCompanyIdentity,
) -> anyhow::Result<u64> {
    let mut targets = rows.iter().filter(|row| {
        row.guid.eq_ignore_ascii_case(company.company_guid())
            && row
                .name
                .trim()
                .eq_ignore_ascii_case(company.display_name().trim())
    });
    match (targets.next(), targets.next()) {
        (Some(row), None) => Ok(row.masters),
        _ => Err(ApprovedImportAdmissionError::MastersUnconfirmed.into()),
    }
}

/// Applies the ledger path's unchanged aim check, then rebuilds the shown stock
/// draft from fresh admitted stock reads sized at the target's own binding
/// mark. Step 1 compares the full
/// `VerifiedCompanyIdentity` values the reads carry, which the coordinator
/// attached; the parsers bind rows by GUID only.
fn recheck_stock_item_admission(input: StockItemRecheckInput<'_, '_>) -> anyhow::Result<()> {
    let shown = input.shown.draft();
    let company = shown.sibling().sibling().catalogue().company();
    if input.candidate.catalogue().company() != company
        || input.groups.company() != company
        || input.units.company() != company
    {
        return Err(StockItemRecheckError::OtherCompany.into());
    }

    // The ledger path's aim check first: it admits exactly one target in each
    // snapshot and an unmoved master mark, so the binding mark below is the
    // target's own and not a namesake's or a same-GUID sibling's.
    let binding_rows = admit_queued_aim(
        input.company_marks_at_binding,
        input.company_marks,
        company.company_guid(),
        company.display_name(),
    )?;
    let binding = target_binding_mark(&binding_rows, company)?;
    for (read, sized) in [
        (StockItemReadKind::Candidate, input.candidate.sized_mark()),
        (StockItemReadKind::StockGroups, input.groups.sized_mark()),
        (StockItemReadKind::Units, input.units.sized_mark()),
    ] {
        if sized != binding {
            return Err(StockItemRecheckError::SizedAtAnotherMark {
                read,
                sized,
                binding,
            }
            .into());
        }
    }

    let name = input
        .candidate
        .name_absent(shown.name())
        .map_err(StockItemRecheckError::NameNotAbsent)?;
    let sibling = SiblingGst::named(
        input.candidate.catalogue(),
        shown.sibling().sibling().source_name(),
    )
    .map_err(StockItemRecheckError::Sibling)?;
    let sibling = GoodsSibling::from_sibling(sibling).map_err(StockItemRecheckError::NotGoods)?;
    let fresh = StockItemDraft::new_admitted(name, sibling, input.groups, input.units)
        .map_err(StockItemRecheckError::Draft)?;
    if !fresh.same_create(shown) {
        return Err(StockItemRecheckError::DraftChanged.into());
    }
    Ok(())
}

/// The request a master approval is held with: the batch and the request
/// rendered from it, once. It is the masters twin of `NativePostRequest` in the
/// approval slot, and carries no company: the rendered bytes name it, and the
/// approval binding holds the verified tuple.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MasterPostRequest {
    batch: LedgerCreateBatch,
    rendered: RenderedLedgerCreate,
}

impl MasterPostRequest {
    /// Renders `batch` for `company`, once.
    pub(super) fn render(
        batch: impl Into<LedgerCreateBatch>,
        company: &VerifiedCompanyIdentity,
    ) -> Self {
        let batch = batch.into();
        let rendered = batch.render_import(company);
        Self { batch, rendered }
    }

    #[cfg(test)]
    pub(super) fn batch(&self) -> &LedgerCreateBatch {
        &self.batch
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

/// Parses a V3 ledger catalogue answer into the ledgers of the name index,
/// with [`parse_ledger_name_index`]'s strict reading: an answer it refuses is
/// no index, and the post is refused before the intent.
pub(super) fn ledger_masters(
    xml: &str,
    expected_company_guid: &str,
) -> Result<Vec<IndexedMaster>, StandardLedgerCatalogError> {
    // The one seam: a tool test scopes the ledgers the book holds, until the
    // tool tests read captured answers of one company end to end. It is
    // compiled under bare `cfg(test)`, so no shipped build holds it.
    #[cfg(test)]
    if let Ok(injected) = TEST_LEDGER_INDEX.try_with(Clone::clone) {
        return Ok(injected);
    }
    Ok(parse_ledger_name_index(xml, expected_company_guid)?
        .into_iter()
        .map(|row| IndexedMaster {
            kind: MasterKind::Ledger,
            guid: row.guid,
            alter_id: row.alter_id,
            names: row
                .names
                .into_iter()
                .chain(row.row_spelling)
                .chain(row.reserved_name)
                .collect(),
        })
        .collect())
}

#[cfg(test)]
tokio::task_local! {
    /// The ledgers a test says the book holds, standing in for a captured
    /// answer. Scoped around the call under test; absent, the answer is
    /// parsed as it is in a shipped build.
    pub(super) static TEST_LEDGER_INDEX: Vec<IndexedMaster>;
}

#[cfg(test)]
tokio::task_local! {
    /// The ledgers a test says the book holds AFTER the POST, for the
    /// read-back only ([`parse_readback_indexes`]). Absent, the read-back uses
    /// [`TEST_LEDGER_INDEX`], and without that the parser's refusal.
    pub(super) static TEST_LEDGER_INDEX_AFTER_POST: Vec<IndexedMaster>;
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
/// an unparsable ledger answer is `MasterLedgerIndexUnavailable`.
pub(super) fn recheck_master_admission(
    queued: QueuedAdmission<'_>,
) -> anyhow::Result<BTreeMap<String, u64>> {
    let QueuedAdmission::Masters {
        batch,
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
        batch,
        company_marks_at_binding,
        company_marks,
        company,
    )
}

/// Parses the two index answers of `company_guid`: the groups first, so an
/// unusable group answer is named before an unusable ledger answer is.
pub(super) fn parse_indexes(
    ledgers_xml: &str,
    groups_xml: &str,
    company_guid: &str,
) -> anyhow::Result<(Vec<IndexedMaster>, Vec<IndexedMaster>)> {
    let groups = parse_groups(groups_xml, company_guid)?;
    let ledgers = ledger_masters(ledgers_xml, company_guid).map_err(|error| {
        ApprovedImportAdmissionError::MasterLedgerIndexUnavailable {
            cause: error.safe_code(),
        }
    })?;
    Ok((ledgers, groups))
}

fn parse_groups(groups_xml: &str, company_guid: &str) -> anyhow::Result<Vec<IndexedMaster>> {
    let groups = parse_group_name_index(groups_xml, company_guid).map_err(|error| {
        ApprovedImportAdmissionError::GroupExportInvalid {
            cause: group_cause(&error),
        }
    })?;
    Ok(group_masters(groups))
}

/// [`parse_indexes`] for the read-back after the POST. The same parsers; the
/// one difference is a test seam, so a test can say the book holds the new
/// ledger now that it did not hold before the POST.
pub(super) fn parse_readback_indexes(
    ledgers_xml: &str,
    groups_xml: &str,
    company_guid: &str,
) -> anyhow::Result<(Vec<IndexedMaster>, Vec<IndexedMaster>)> {
    #[cfg(test)]
    if let Ok(injected) = TEST_LEDGER_INDEX_AFTER_POST.try_with(Clone::clone) {
        return Ok((injected, parse_groups(groups_xml, company_guid)?));
    }
    parse_indexes(ledgers_xml, groups_xml, company_guid)
}

/// The check a master tool makes before it saves or asks anything: every
/// proposed name resolves to no ledger or group in the index answers just
/// read. It is the queue's name decision without the aim, which only the queue
/// can make. Typed refusals as [`recheck_master_admission`] gives them.
pub(super) fn check_name_free(
    ledgers_xml: &str,
    groups_xml: &str,
    batch: &LedgerCreateBatch,
    company_guid: &str,
) -> anyhow::Result<()> {
    let (ledgers, groups) = parse_indexes(ledgers_xml, groups_xml, company_guid)?;
    admit_free_names(ledgers, groups, batch).map(|_| ())
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
pub(super) fn aim_master_marks(company_marks: &str) -> anyhow::Result<Vec<MasterMark>> {
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

/// The decision, over parsed indexes: no ledger or group holds any item's
/// name, then the company's master mark did not move
/// between the binding snapshot and the aim snapshot (`admit_queued_aim`).
/// The first collision, in ledger-then-group order, names the refusal.
pub(super) fn admit_master_create(
    ledgers: Vec<IndexedMaster>,
    groups: Vec<IndexedMaster>,
    batch: &LedgerCreateBatch,
    company_marks_at_binding: &str,
    company_marks: &str,
    company: &VerifiedCompanyIdentity,
) -> anyhow::Result<BTreeMap<String, u64>> {
    let index = admit_free_names(ledgers, groups, batch)?;
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

/// The name decision over parsed indexes: no ledger or group holds a spelling
/// any item's name could resolve to. The first collision, in batch order and
/// then ledger-then-group order, names the refusal. Returns the index, whose
/// pre-image the dispatch records.
fn admit_free_names(
    ledgers: Vec<IndexedMaster>,
    groups: Vec<IndexedMaster>,
    batch: &LedgerCreateBatch,
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
    for item in batch.items() {
        admit_free_name(&index, item)?;
    }
    Ok(index)
}

/// One item's name against the index.
fn admit_free_name(index: &MasterNameIndex, item: &LedgerCreateItem) -> anyhow::Result<()> {
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
    Ok(())
}

#[cfg(test)]
#[path = "agent_master_post_tests.rs"]
mod tests;
