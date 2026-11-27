//! One native masters read, inside the bracket the Trial Balance read uses.
use super::*;
use crate::tally::connection::compliance_estimate;
#[cfg(test)]
use bridge_tally_protocol::native_masters::NativeMastersError;
use bridge_tally_protocol::native_masters::{
    masters_worst_row_bytes, parse_native_masters, render_candidate_stock_items_request,
    render_native_masters_request, NativeMasterKind, NativeMasterRow, NativeMasters,
    MASTERS_RESPONSE_BUDGET_BYTES,
};
use bridge_tally_protocol::native_stock_summary::{
    parse_company_inventory_flags, render_company_inventory_flags_request, NativeStockError,
    NativeStockItemCount,
};
use bridge_tally_protocol::native_voucher_type_numbering::{
    join_series_numbering, parse_voucher_type_numbering, render_voucher_type_numbering_request,
    NumberingJoinError, VoucherTypeNumbering, VoucherTypeNumberingRow,
};
use bridge_tally_protocol::outstandings_shared::OutstandingsError;
use bridge_tally_protocol::TallyNamedMaster;
use std::collections::HashSet;

/// The masters a read can return: one native collection per kind, or the
/// account groups, which stay on the group snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MastersKind {
    Native(NativeMasterKind),
    Groups,
}

impl MastersKind {
    pub(crate) fn parse(name: &str) -> Option<Self> {
        match name {
            "voucher_types" => Some(Self::Native(NativeMasterKind::VoucherTypes)),
            "godowns" => Some(Self::Native(NativeMasterKind::Godowns)),
            "units" => Some(Self::Native(NativeMasterKind::Units)),
            "stock_groups" => Some(Self::Native(NativeMasterKind::StockGroups)),
            "stock_items" => Some(Self::Native(NativeMasterKind::StockItems)),
            "cost_centres" => Some(Self::Native(NativeMasterKind::CostCentres)),
            "cost_categories" => Some(Self::Native(NativeMasterKind::CostCategories)),
            "groups" => Some(Self::Groups),
            _ => None,
        }
    }

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Native(NativeMasterKind::VoucherTypes) => "voucher_types",
            Self::Native(NativeMasterKind::Godowns) => "godowns",
            Self::Native(NativeMasterKind::Units) => "units",
            Self::Native(NativeMasterKind::StockGroups) => "stock_groups",
            Self::Native(NativeMasterKind::StockItems) => "stock_items",
            Self::Native(NativeMasterKind::CostCentres) => "cost_centres",
            Self::Native(NativeMasterKind::CostCategories) => "cost_categories",
            Self::Groups => "groups",
        }
    }
}

pub(crate) enum MastersRows {
    /// The rows of one native kind other than voucher types.
    Native(NativeMasters),
    /// The voucher types, each paired with the series-level numbering joined to
    /// it: there is no such read without its numbering.
    VoucherTypes(Vec<(NativeMasterRow, VoucherTypeNumbering)>),
    Groups(Vec<TallyNamedMaster>),
}

/// A completed observation of one kind of master.
pub(crate) struct MastersRead {
    pub(crate) rows: MastersRows,
    pub(crate) evidence: RuntimeReadEvidence,
    /// The first voucher-types pair, when it disagreed and a second pair
    /// agreed (#2160): its evidence, which `evidence` also folds in.
    pub(crate) first_pair_differed: Option<RuntimeReadEvidence>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum MastersReadError {
    #[error("masters_education_unqualified")]
    EducationUnqualified,
    /// The master mark, times an assumed worst-case row of this kind, is over
    /// the response budget, so nothing was requested.
    #[error("masters_too_large")]
    TooLarge {
        master_alter_id: u64,
        estimated_bytes: u64,
        limit_bytes: u64,
        /// The largest mark this kind is admitted at: the budget over its worst row.
        limit_master_alter_id: u64,
    },
    /// The collection came back and broke what its size was admitted on (the
    /// master mark bounds the rows; the response fits the estimate).
    #[error("masters_bound_premise_violated")]
    PremiseViolated(&'static str),
    /// The voucher-type numbering answer does not join the masters answer.
    #[error("{}", .0.code())]
    NumberingJoin(NumberingJoinError),
}

impl MastersReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "masters_education_unqualified",
            Self::TooLarge { .. } => "masters_too_large",
            Self::PremiseViolated(_) => "masters_bound_premise_violated",
            Self::NumberingJoin(join) => join.code(),
        }
    }
}

/// The response budget, as a byte count the estimate arithmetic takes.
fn budget_bytes() -> u64 {
    u64::try_from(MASTERS_RESPONSE_BUDGET_BYTES).unwrap_or(u64::MAX)
}

/// Whether a read of `kind` is sized by the master mark before its request.
/// Voucher types are not: production already sends the same request unbounded
/// (the `vouchers` path), so they keep that policy and are held only by the
/// checks after the read, as groups are held by none before it.
pub(crate) const fn sized_before_the_read(kind: NativeMasterKind) -> bool {
    !matches!(kind, NativeMasterKind::VoucherTypes)
}

/// The most bytes one collection response may take, checked after the read:
/// the estimate the read was admitted at, or for a kind not sized before the
/// read the whole budget.
pub(crate) fn limit_for(kind: NativeMasterKind, admitted_estimate: Option<u64>) -> u64 {
    match admitted_estimate {
        Some(estimate) if sized_before_the_read(kind) => estimate,
        _ => budget_bytes(),
    }
}

/// Admits a collection read only when the mark, times an assumed worst-case
/// row of its kind, fits the response budget (an estimate exactly at the
/// budget fits). Returns the estimate. Sent before any collection request.
pub(crate) fn admit_masters_size(
    kind: NativeMasterKind,
    master_alter_id: u64,
) -> Result<u64, MastersReadError> {
    let row_bytes = u64::try_from(masters_worst_row_bytes(kind)).unwrap_or(u64::MAX);
    let estimate = compliance_estimate(master_alter_id, row_bytes, budget_bytes());
    if !estimate.fits {
        return Err(MastersReadError::TooLarge {
            master_alter_id,
            estimated_bytes: estimate.estimated_bytes,
            limit_bytes: budget_bytes(),
            limit_master_alter_id: budget_bytes().checked_div(row_bytes).unwrap_or(0),
        });
    }
    Ok(estimate.estimated_bytes)
}

/// The premise a read was admitted on, checked against what came back: every
/// object has its own AlterID at or under the mark, so the rows cannot outnumber
/// the mark, none can sit above it, none can repeat, and the response (one of
/// the two paired reads, checked after both) cannot exceed `limit_bytes` (see
/// [`limit_for`]). A violation is a loud refusal, not a truncated read.
pub(crate) fn check_masters_premise(
    rows: &[NativeMasterRow],
    master_alter_id: u64,
    received_bytes: usize,
    limit_bytes: u64,
) -> Result<(), MastersReadError> {
    if u64::try_from(rows.len()).unwrap_or(u64::MAX) > master_alter_id {
        return Err(MastersReadError::PremiseViolated(
            "masters_rows_exceed_master_mark",
        ));
    }
    if rows.iter().any(|row| row.alter_id > master_alter_id) {
        return Err(MastersReadError::PremiseViolated(
            "masters_alter_id_above_master_mark",
        ));
    }
    let mut seen = HashSet::new();
    if rows.iter().any(|row| !seen.insert(row.alter_id)) {
        return Err(MastersReadError::PremiseViolated(
            "masters_alter_id_repeated",
        ));
    }
    if u64::try_from(received_bytes).unwrap_or(u64::MAX) > limit_bytes {
        return Err(MastersReadError::PremiseViolated(
            "masters_response_over_admitted_bytes",
        ));
    }
    Ok(())
}

/// The three collection reads from which a complete stock-item draft is proved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StockItemReadKind {
    Candidate,
    StockGroups,
    Units,
}

impl StockItemReadKind {
    const fn native(self) -> NativeMasterKind {
        match self {
            Self::Candidate => NativeMasterKind::StockItems,
            Self::StockGroups => NativeMasterKind::StockGroups,
            Self::Units => NativeMasterKind::Units,
        }
    }
}

/// The size admission of one stock-item-related read, made before its fetch.
/// Outside tests only [`read_stock_item_admission`] makes one, from the
/// target's own binding master mark.
pub(crate) struct StockItemReadSize {
    kind: StockItemReadKind,
    master_mark: u64,
    limit_bytes: u64,
}

impl StockItemReadSize {
    pub(crate) fn new(kind: StockItemReadKind, master_mark: u64) -> Result<Self, MastersReadError> {
        let native = kind.native();
        let estimate = admit_masters_size(native, master_mark)?;
        Ok(Self {
            kind,
            master_mark,
            limit_bytes: limit_for(native, Some(estimate)),
        })
    }

    #[cfg(test)]
    pub(crate) fn with_limit_for_test(
        kind: StockItemReadKind,
        master_mark: u64,
        limit_bytes: u64,
    ) -> Self {
        Self {
            kind,
            master_mark,
            limit_bytes,
        }
    }

    #[cfg(test)]
    pub(crate) const fn admitted_bytes(&self) -> u64 {
        self.limit_bytes
    }
}

#[derive(Debug, thiserror::Error)]
#[error("stock item read size token has another kind")]
pub(crate) struct StockItemReadKindError {
    expected: StockItemReadKind,
    actual: StockItemReadKind,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("stock item row count differs from the company count")]
pub(crate) struct StockItemCountMismatch {
    pub(crate) rows: usize,
    pub(crate) tally_count: u64,
}

/// A stable, sized, company-bound candidate answer whose row count Tally's own
/// company count confirms.
#[derive(Debug)]
pub(crate) struct AdmittedCandidateRead {
    company: VerifiedCompanyIdentity,
    body: String,
    sized_mark: u64,
}

impl AdmittedCandidateRead {
    pub(crate) fn company(&self) -> &VerifiedCompanyIdentity {
        &self.company
    }

    pub(crate) fn body(&self) -> &str {
        &self.body
    }

    pub(crate) const fn sized_mark(&self) -> u64 {
        self.sized_mark
    }
}

/// The parsed stock groups of one stable, sized read of the same company.
#[derive(Debug)]
pub(crate) struct AdmittedStockGroups {
    company: VerifiedCompanyIdentity,
    rows: Vec<NativeMasterRow>,
    sized_mark: u64,
}

impl AdmittedStockGroups {
    pub(crate) fn company(&self) -> &VerifiedCompanyIdentity {
        &self.company
    }

    pub(crate) fn rows(&self) -> &[NativeMasterRow] {
        &self.rows
    }

    pub(crate) const fn sized_mark(&self) -> u64 {
        self.sized_mark
    }

    #[cfg(test)]
    pub(crate) fn from_capture_for_test(
        company: &VerifiedCompanyIdentity,
        body: &str,
        sized_mark: u64,
    ) -> Result<Self, NativeMastersError> {
        let rows =
            parse_native_masters(NativeMasterKind::StockGroups, body, company.company_guid())?.rows;
        Ok(Self {
            company: company.clone(),
            rows,
            sized_mark,
        })
    }
}

/// The parsed units of one stable, sized read of the same company.
#[derive(Debug)]
pub(crate) struct AdmittedStockUnits {
    company: VerifiedCompanyIdentity,
    rows: Vec<NativeMasterRow>,
    sized_mark: u64,
}

impl AdmittedStockUnits {
    pub(crate) fn company(&self) -> &VerifiedCompanyIdentity {
        &self.company
    }

    pub(crate) fn rows(&self) -> &[NativeMasterRow] {
        &self.rows
    }

    pub(crate) const fn sized_mark(&self) -> u64 {
        self.sized_mark
    }

    #[cfg(test)]
    pub(crate) fn from_capture_for_test(
        company: &VerifiedCompanyIdentity,
        body: &str,
        sized_mark: u64,
    ) -> Result<Self, NativeMastersError> {
        let rows =
            parse_native_masters(NativeMasterKind::Units, body, company.company_guid())?.rows;
        Ok(Self {
            company: company.clone(),
            rows,
            sized_mark,
        })
    }
}

fn require_stock_item_read_kind(
    size: &StockItemReadSize,
    expected: StockItemReadKind,
) -> Result<(), StockItemReadKindError> {
    if size.kind != expected {
        return Err(StockItemReadKindError {
            expected,
            actual: size.kind,
        });
    }
    Ok(())
}

/// Admits the candidate and company-count answers as one complete candidate
/// read. Outside tests its pairs are the two that
/// [`read_stock_item_admission`] sends back to back.
pub(crate) fn admit_candidate_stock_item_read(
    size: StockItemReadSize,
    identity: &VerifiedCompanyIdentity,
    candidate_pair: NativePairedRead,
    inventory_pair: NativePairedRead,
) -> anyhow::Result<AdmittedCandidateRead> {
    require_stock_item_read_kind(&size, StockItemReadKind::Candidate)?;
    let (body, encoded_bytes, _) =
        candidate_pair.require_stable(PairedReadValidationError::MastersCollection)?;
    let (count_body, _, _) =
        inventory_pair.require_stable(PairedReadValidationError::StockSummaryCollection)?;
    let candidate =
        parse_native_masters(NativeMasterKind::StockItems, &body, identity.company_guid())?;
    check_masters_premise(
        &candidate.rows,
        size.master_mark,
        encoded_bytes,
        size.limit_bytes,
    )?;
    let count = parse_company_inventory_flags(&count_body, identity.company_guid())?.item_count;
    match count {
        NativeStockItemCount::Unavailable => {
            return Err(NativeStockError::ItemCountUnavailable.into())
        }
        NativeStockItemCount::Reported(tally_count)
            if tally_count != u64::try_from(candidate.rows.len()).unwrap_or(u64::MAX) =>
        {
            return Err(StockItemCountMismatch {
                rows: candidate.rows.len(),
                tally_count,
            }
            .into());
        }
        NativeStockItemCount::Reported(_) => {}
    }
    Ok(AdmittedCandidateRead {
        company: identity.clone(),
        body,
        sized_mark: size.master_mark,
    })
}

/// Admits one stock-group answer, as [`admit_candidate_stock_item_read`] its pairs.
pub(crate) fn admit_stock_groups(
    size: StockItemReadSize,
    identity: &VerifiedCompanyIdentity,
    pair: NativePairedRead,
) -> anyhow::Result<AdmittedStockGroups> {
    require_stock_item_read_kind(&size, StockItemReadKind::StockGroups)?;
    let (body, encoded_bytes, _) =
        pair.require_stable(PairedReadValidationError::MastersCollection)?;
    let parsed = parse_native_masters(
        NativeMasterKind::StockGroups,
        &body,
        identity.company_guid(),
    )?;
    check_masters_premise(
        &parsed.rows,
        size.master_mark,
        encoded_bytes,
        size.limit_bytes,
    )?;
    Ok(AdmittedStockGroups {
        company: identity.clone(),
        rows: parsed.rows,
        sized_mark: size.master_mark,
    })
}

/// Admits one unit answer, as [`admit_candidate_stock_item_read`] its pairs.
pub(crate) fn admit_stock_units(
    size: StockItemReadSize,
    identity: &VerifiedCompanyIdentity,
    pair: NativePairedRead,
) -> anyhow::Result<AdmittedStockUnits> {
    require_stock_item_read_kind(&size, StockItemReadKind::Units)?;
    let (body, encoded_bytes, _) =
        pair.require_stable(PairedReadValidationError::MastersCollection)?;
    let parsed = parse_native_masters(NativeMasterKind::Units, &body, identity.company_guid())?;
    check_masters_premise(
        &parsed.rows,
        size.master_mark,
        encoded_bytes,
        size.limit_bytes,
    )?;
    Ok(AdmittedStockUnits {
        company: identity.clone(),
        rows: parsed.rows,
        sized_mark: size.master_mark,
    })
}

/// What [`TallyRuntime::read_and_recheck_stock_item`] read in its one queued
/// execution: the three admitted stock reads, each sized at the target's binding
/// master mark, and the binding and aim marks answers it sent. The fields are
/// private and only that coordinator makes one, so a recheck that takes it runs
/// on that coordinator's own reads.
#[derive(Debug)]
pub(crate) struct StockItemRecheckReads {
    candidate: AdmittedCandidateRead,
    groups: AdmittedStockGroups,
    units: AdmittedStockUnits,
    company_marks_at_binding: String,
    company_marks: String,
}

impl StockItemRecheckReads {
    /// The candidate, stock-group and unit reads, then the binding and aim
    /// marks answers.
    pub(crate) fn into_parts(
        self,
    ) -> (
        AdmittedCandidateRead,
        AdmittedStockGroups,
        AdmittedStockUnits,
        String,
        String,
    ) {
        (
            self.candidate,
            self.groups,
            self.units,
            self.company_marks_at_binding,
            self.company_marks,
        )
    }
}

/// One loaded company's row of the all-company marks answer, as far as the
/// stock coordinator needs it. The agent's marks parser makes these rows; the
/// runtime chooses the target among them ([`target_binding_mark`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CompanyMasterMark {
    pub(crate) name: String,
    pub(crate) guid: String,
    pub(crate) masters: u64,
}

/// The target's master mark in a marks snapshot: that of the one row whose
/// GUID and trimmed name both match `identity`, not the first row with its
/// GUID, because a second loaded company can share a GUID under another name.
/// No such row, or two, is `MastersUnconfirmed`. The stock coordinator sizes
/// its reads at it and the stock recheck compares them to it.
pub(crate) fn target_binding_mark(
    rows: &[CompanyMasterMark],
    identity: &VerifiedCompanyIdentity,
) -> anyhow::Result<u64> {
    let mut targets = rows.iter().filter(|row| {
        row.guid.eq_ignore_ascii_case(identity.company_guid())
            && row
                .name
                .trim()
                .eq_ignore_ascii_case(identity.display_name().trim())
    });
    match (targets.next(), targets.next()) {
        (Some(row), None) => Ok(row.masters),
        _ => Err(
            crate::tally::approved_import::ApprovedImportAdmissionError::MastersUnconfirmed.into(),
        ),
    }
}

/// The premise of the numbering read, checked against its answer: its rows
/// cannot outnumber the master mark and the response fits `limit_bytes`.
/// Voucher types are unbounded before the read, so the limit is the whole
/// budget.
pub(crate) fn check_numbering_premise(
    rows: &[VoucherTypeNumberingRow],
    master_alter_id: u64,
    received_bytes: usize,
    limit_bytes: u64,
) -> Result<(), MastersReadError> {
    if u64::try_from(rows.len()).unwrap_or(u64::MAX) > master_alter_id {
        return Err(MastersReadError::PremiseViolated(
            "masters_numbering_rows_exceed_master_mark",
        ));
    }
    if u64::try_from(received_bytes).unwrap_or(u64::MAX) > limit_bytes {
        return Err(MastersReadError::PremiseViolated(
            "masters_numbering_response_over_admitted_bytes",
        ));
    }
    Ok(())
}

/// The voucher-type collection as a pair, read as one more pair when the first
/// disagrees with itself (#2160). Tally leaves an element out of the first
/// voucher-types read after a company is loaded and sends it on every later
/// one (protocol reference section 12a.25, PARTIAL), so a first pair can
/// disagree on a book that has not changed.
///
/// **At most one more pair is read, here and nowhere else.** The call is
/// straight-line, with no loop: a second pair that disagrees too is refused as
/// `masters_collection_changed`, as a first one was before, so a book that
/// keeps changing is not read until it settles. The bytes used are still a pair
/// of byte-identical answers. Only what the checks after it see is still
/// refused: a moved book extent (`masters_extent_changed`, the masters tool),
/// or a type saved above the master mark (`masters_bound_premise_violated`).
/// A type deleted between the pairs leaves a second pair that agrees and is
/// accepted; whether a deletion moves the master mark is not measured.
///
/// The disagreeing pair's evidence is added to `evidence`, so a refusal keeps
/// all four reads, and is also set in `first_pair_differed`, so an answer that
/// follows can show the first disagreement without having refused.
async fn read_voucher_type_pair(
    client: &TallyClient,
    request: &str,
    evidence: &mut RuntimeReadEvidence,
    first_pair_differed: &mut Option<RuntimeReadEvidence>,
) -> anyhow::Result<(String, usize, String)> {
    let disagreed = match client
        .fetch_native_report_paired(request.to_owned())
        .await?
    {
        NativePairedRead::Stable {
            body,
            encoded_bytes,
            encoded_sha256,
        } => return Ok((body, encoded_bytes, encoded_sha256)),
        NativePairedRead::Drifted(disagreed) => disagreed,
    };
    *evidence = evidence.clone().combine(disagreed.clone());
    *first_pair_differed = Some(disagreed);
    // The one more pair (section 12a.25). Its disagreement is the refusal.
    client
        .fetch_native_report_paired(request.to_owned())
        .await?
        .require_stable(PairedReadValidationError::MastersCollection)
}

/// One native collection read as a pair against the master mark `mark`: sized
/// by the mark first where its kind is, then held to the premise its size was
/// admitted on. A refusal before or during the request, or an answer Bridge
/// cannot read, is the outer error; a broken premise is returned inside, so a
/// caller that reads a closing extent can report a moved book first.
async fn read_native_rows(
    client: &TallyClient,
    identity: &VerifiedCompanyIdentity,
    native: NativeMasterKind,
    mark: u64,
    evidence: &mut RuntimeReadEvidence,
    first_pair_differed: &mut Option<RuntimeReadEvidence>,
) -> anyhow::Result<Result<NativeMasters, MastersReadError>> {
    let estimate = sized_before_the_read(native)
        .then(|| admit_masters_size(native, mark))
        .transpose()?;
    let limit = limit_for(native, estimate);
    let request = render_native_masters_request(native, identity.display_name());
    let (xml, bytes, hash) = if native == NativeMasterKind::VoucherTypes {
        // Voucher types only, and one more pair at most (protocol reference
        // section 12a.25): every other kind is one pair, as it was.
        read_voucher_type_pair(client, &request, evidence, first_pair_differed).await?
    } else {
        client
            .fetch_native_report_paired(request.clone())
            .await?
            .require_stable(PairedReadValidationError::MastersCollection)?
    };
    *evidence = evidence
        .clone()
        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
    let masters = parse_native_masters(native, &xml, identity.company_guid())?;
    Ok(check_masters_premise(&masters.rows, mark, bytes, limit).map(|()| masters))
}

/// The series-level numbering of the voucher types `rows`, read as a second
/// pair against the same mark and joined to them: one result per row, in the
/// rows' order. Errors split as [`read_native_rows`] splits them.
async fn read_series_numbering(
    client: &TallyClient,
    identity: &VerifiedCompanyIdentity,
    mark: u64,
    rows: Vec<NativeMasterRow>,
    evidence: &mut RuntimeReadEvidence,
) -> anyhow::Result<Result<Vec<(NativeMasterRow, VoucherTypeNumbering)>, MastersReadError>> {
    let request = render_voucher_type_numbering_request(identity.company_name());
    let (xml, bytes, hash) = client
        .fetch_native_report_paired(request.clone())
        .await?
        .require_stable(PairedReadValidationError::MastersNumberingCollection)?;
    *evidence = evidence
        .clone()
        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
    let numbering = parse_voucher_type_numbering(&xml)?;
    Ok(
        check_numbering_premise(&numbering, mark, bytes, budget_bytes())
            .and_then(|()| {
                join_series_numbering(&rows, &numbering).map_err(MastersReadError::NumberingJoin)
            })
            // The join returns one result for each masters row.
            .map(|joined| rows.into_iter().zip(joined).collect()),
    )
}

/// Each voucher type's series-level numbering, read after the company's
/// master mark, its masters rows bounded by it (bridge#724). Built only by
/// [`TallyRuntime::fetch_voucher_type_numbering`], so a value of this type is
/// always a completed read.
#[derive(Debug)]
pub(crate) struct VoucherNumberingRead {
    company_guid: String,
    master_mark: u64,
    types: Vec<(String, VoucherTypeNumbering)>,
    evidence: RuntimeReadEvidence,
}

impl VoucherNumberingRead {
    pub(crate) fn company_guid(&self) -> &str {
        &self.company_guid
    }

    /// The company's `ALTMSTID` the read was bounded by, read before it.
    pub(crate) const fn master_mark(&self) -> u64 {
        self.master_mark
    }

    /// Each voucher type's name with its numbering, in the masters order.
    pub(crate) fn types(&self) -> &[(String, VoucherTypeNumbering)] {
        &self.types
    }

    pub(crate) fn evidence(&self) -> &RuntimeReadEvidence {
        &self.evidence
    }

    /// A read as a test states it, with no Tally behind it.
    #[cfg(test)]
    pub(crate) fn for_test(
        company_guid: &str,
        master_mark: u64,
        types: Vec<(String, VoucherTypeNumbering)>,
    ) -> Self {
        Self {
            company_guid: company_guid.to_string(),
            master_mark,
            types,
            evidence: RuntimeReadEvidence::empty(),
        }
    }
}

/// What reads the all-company marks answer into the rows
/// [`target_binding_mark`] chooses among: the agent's parser, carried by a stock
/// approval so the queue sizes its reads as the standalone recheck does.
pub(crate) type CompanyMarksParser = fn(&str) -> anyhow::Result<Vec<CompanyMasterMark>>;

/// The evidence of a pair that agreed, added to `evidence`. A drifted pair adds
/// nothing here: its evidence rides in the refusal its admission returns.
fn add_pair_evidence(evidence: &mut RuntimeReadEvidence, request: &str, pair: &NativePairedRead) {
    if let NativePairedRead::Stable {
        encoded_bytes,
        encoded_sha256,
        ..
    } = pair
    {
        *evidence = evidence.clone().combine(RuntimeReadEvidence::paired(
            request,
            encoded_sha256.clone(),
            *encoded_bytes,
        ));
    }
}

/// The reads a stock-item Create is admitted on, after the binding marks
/// answer `binding_marks` (#1381): the target's own mark among the rows
/// `parse_marks` reads from it ([`target_binding_mark`]); the three size
/// admissions at that mark, so a book above an edge sends no collection
/// request; the candidate pair, a drifted one refusing before the count is
/// sent, and the company-count pair, back to back; the stock-group pair; the
/// unit pair. The queue ([`TallyRuntime::post_approved_import`]) and the
/// standalone recheck ([`TallyRuntime::read_and_recheck_stock_item`]) both read
/// through here. Each agreeing pair's evidence is added to `evidence`, a
/// drifted pair's is in its refusal, and errors are returned as they are, for
/// the caller to attach `evidence`, as [`read_native_rows`] returns them.
/// Nothing here sends an import.
pub(super) async fn read_stock_item_admission(
    client: &TallyClient,
    identity: &VerifiedCompanyIdentity,
    binding_marks: &str,
    parse_marks: CompanyMarksParser,
    evidence: &mut RuntimeReadEvidence,
) -> anyhow::Result<(
    AdmittedCandidateRead,
    AdmittedStockGroups,
    AdmittedStockUnits,
)> {
    let mark = target_binding_mark(&parse_marks(binding_marks)?, identity)?;
    let candidate_size = StockItemReadSize::new(StockItemReadKind::Candidate, mark)?;
    let groups_size = StockItemReadSize::new(StockItemReadKind::StockGroups, mark)?;
    let units_size = StockItemReadSize::new(StockItemReadKind::Units, mark)?;
    // Nothing between the candidate pair and the count pair, so the count is of
    // the book the candidate rows were read from as nearly as two serial reads
    // allow.
    let candidate_request = render_candidate_stock_items_request(identity.company_name());
    let candidate_pair = client
        .fetch_native_report_paired(candidate_request.clone())
        .await?;
    add_pair_evidence(evidence, &candidate_request, &candidate_pair);
    // The admission's own drift refusal, taken before the count pair is sent
    // rather than after it.
    let (body, encoded_bytes, encoded_sha256) =
        candidate_pair.require_stable(PairedReadValidationError::MastersCollection)?;
    let candidate_pair = NativePairedRead::Stable {
        body,
        encoded_bytes,
        encoded_sha256,
    };
    let count_request = render_company_inventory_flags_request(
        identity.company_name(),
        identity.company_guid_value(),
    );
    let count_pair = client
        .fetch_native_report_paired(count_request.clone())
        .await?;
    add_pair_evidence(evidence, &count_request, &count_pair);
    let candidate =
        admit_candidate_stock_item_read(candidate_size, identity, candidate_pair, count_pair)?;
    let groups_request =
        render_native_masters_request(NativeMasterKind::StockGroups, identity.display_name());
    let groups_pair = client
        .fetch_native_report_paired(groups_request.clone())
        .await?;
    add_pair_evidence(evidence, &groups_request, &groups_pair);
    let groups = admit_stock_groups(groups_size, identity, groups_pair)?;
    let units_request =
        render_native_masters_request(NativeMasterKind::Units, identity.display_name());
    let units_pair = client
        .fetch_native_report_paired(units_request.clone())
        .await?;
    add_pair_evidence(evidence, &units_request, &units_pair);
    let units = admit_stock_units(units_size, identity, units_pair)?;
    Ok((candidate, groups, units))
}

impl TallyRuntime {
    /// One kind of master, with the same queue, mode and identity brackets as
    /// the Trial Balance and its book-extent pair: the extent before and after
    /// must be equal, or the read refuses. Also returns that extent, so a
    /// caller can tell later whether the book has moved.
    pub(crate) async fn fetch_masters_with_extent(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        kind: MastersKind,
    ) -> anyhow::Result<(MastersRead, CompanyBookExtent)> {
        let _lease = self.begin_ordinary_read(&config)?;
        let identity = identity.clone();
        self.execute(config, ReadRetryPolicy::SINGLE_ATTEMPT, move |client| {
            let identity = identity.clone();
            async move {
                let mut evidence = RuntimeReadEvidence::empty();
                let result = async {
                    let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                    evidence = mode_evidence;
                    if profile == DateBoundaryProfile::EducationRestricted {
                        return Err(MastersReadError::EducationUnqualified.into());
                    }
                    bracket_verified_company_identity(&client, &identity).await?;
                    let extent = client.fetch_company_book_extent(&identity).await?;
                    // A response Bridge cannot read refuses at once. Only a
                    // broken size premise, the numbering's included, and a
                    // numbering that does not join are held until the closing
                    // extent is read: a book that moved mid-read is reported
                    // as moved, not as the rows its new masters added.
                    let mut first_pair_differed = None;
                    let held: Result<MastersRows, MastersReadError> = match kind {
                        MastersKind::Native(native) => {
                            let mark = extent
                                .master_alter_id_high_water()
                                .ok_or(OutstandingsError::MasterWitnessAbsent)?
                                .get();
                            match read_native_rows(
                                &client,
                                &identity,
                                native,
                                mark,
                                &mut evidence,
                                &mut first_pair_differed,
                            )
                            .await?
                            {
                                Err(violated) => Err(violated),
                                // Voucher types only: the series-level numbering is
                                // a second collection, read and compared the same
                                // way, inside the same opening and closing extents.
                                Ok(masters) if native == NativeMasterKind::VoucherTypes => {
                                    read_series_numbering(
                                        &client,
                                        &identity,
                                        mark,
                                        masters.rows,
                                        &mut evidence,
                                    )
                                    .await?
                                    .map(MastersRows::VoucherTypes)
                                }
                                Ok(masters) => Ok(MastersRows::Native(masters)),
                            }
                        }
                        // The group snapshot as the statements read it: no
                        // size admission of its own.
                        MastersKind::Groups => {
                            let request =
                                render_native_group_snapshot_request(identity.company_name());
                            let (xml, bytes, hash) = client
                                .fetch_native_report_paired(request.clone())
                                .await?
                                .require_stable(PairedReadValidationError::MastersCollection)?;
                            evidence = evidence
                                .clone()
                                .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                            Ok(MastersRows::Groups(parse_native_group_snapshot(
                                &xml,
                                identity.company_guid(),
                            )?))
                        }
                    };
                    let closing_extent = client.fetch_company_book_extent(&identity).await?;
                    if closing_extent != extent {
                        return Err(PairedReadValidationError::MastersExtent.into());
                    }
                    let rows = held?;
                    bracket_verified_company_identity(&client, &identity).await?;
                    evidence = evidence
                        .clone()
                        .combine(confirm_read_boundary(&client, profile).await?);
                    Ok((
                        MastersRead {
                            rows,
                            evidence: evidence.clone(),
                            first_pair_differed,
                        },
                        extent,
                    ))
                }
                .await;
                result.map_err(|error| with_read_evidence(error, evidence))
            }
        })
        .await
    }

    /// The voucher types and their series-level numbering, read as the masters
    /// tool reads them but bounded by `master_mark`, the company's `ALTMSTID`
    /// the caller read before this call (bridge#724). A type saved after that
    /// mark and before the masters pair carries an AlterID above it and
    /// refuses the read (`masters_bound_premise_violated`, cause
    /// `masters_alter_id_above_master_mark`). The numbering rows carry no
    /// AlterID, so a type saved between the masters pair and the numbering
    /// pair is not seen here; a post sees it as the moved mark it compares
    /// just before sending. No book-extent pair: its equality also covers
    /// vouchers, so a voucher entered during the read would refuse it.
    /// Education is refused, as the masters tool refuses it: the read is not
    /// measured there.
    pub(crate) async fn fetch_voucher_type_numbering(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        master_mark: u64,
    ) -> anyhow::Result<VoucherNumberingRead> {
        let _lease = self.begin_ordinary_read(&config)?;
        let identity = identity.clone();
        self.execute(config, ReadRetryPolicy::SINGLE_ATTEMPT, move |client| {
            let identity = identity.clone();
            async move {
                let mut evidence = RuntimeReadEvidence::empty();
                let result = async {
                    let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                    evidence = mode_evidence;
                    if profile == DateBoundaryProfile::EducationRestricted {
                        return Err(MastersReadError::EducationUnqualified.into());
                    }
                    bracket_verified_company_identity(&client, &identity).await?;
                    // Not shown by the callers of this read: a re-read is in
                    // `evidence`, folded into the chain, and in the refusal if
                    // the second pair disagrees too.
                    let mut first_pair_differed = None;
                    let masters = read_native_rows(
                        &client,
                        &identity,
                        NativeMasterKind::VoucherTypes,
                        master_mark,
                        &mut evidence,
                        &mut first_pair_differed,
                    )
                    .await??;
                    let joined = read_series_numbering(
                        &client,
                        &identity,
                        master_mark,
                        masters.rows,
                        &mut evidence,
                    )
                    .await??;
                    bracket_verified_company_identity(&client, &identity).await?;
                    evidence = evidence
                        .clone()
                        .combine(confirm_read_boundary(&client, profile).await?);
                    Ok(VoucherNumberingRead {
                        company_guid: identity.company_guid().to_string(),
                        master_mark,
                        types: joined
                            .into_iter()
                            .map(|(row, numbering)| (row.name, numbering))
                            .collect(),
                        evidence: evidence.clone(),
                    })
                }
                .await;
                result.map_err(|error| with_read_evidence(error, evidence))
            }
        })
        .await
    }

    /// The reads a stock-item Create is rechecked on (#1381, slice 5b-3b), made
    /// in one queued execution with the masters read's brackets, then `recheck`
    /// on them. In order: the mode probe, Education refused before anything
    /// else is sent (the read is not measured there); the identity bracket;
    /// every loaded company's marks, the binding snapshot; the three size
    /// admissions at the target's own mark in that snapshot
    /// ([`target_binding_mark`] over the rows `parse_marks` reads from it), so a
    /// book above an edge sends no collection request; the candidate pair, a
    /// drifted one refusing before the count is sent, and the company-count
    /// pair, back to back; the stock-group
    /// pair; the unit pair; the closing identity and mode brackets; and the
    /// marks again, last, the aim. Only `recheck`, local work, follows the aim,
    /// and its value is returned. Nothing here sends an import, and no shipped
    /// caller reaches it. The
    /// runtime trusts the rows `parse_marks` returns; the check on them is the
    /// recheck's own re-derivation of the mark from the binding text.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) async fn read_and_recheck_stock_item<R, T>(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        company_marks_request: crate::tally::agent_read_request::AgentReadRequest,
        parse_marks: CompanyMarksParser,
        recheck: R,
    ) -> anyhow::Result<T>
    where
        R: Fn(StockItemRecheckReads) -> anyhow::Result<T>,
    {
        use crate::tally::approved_import::ApprovedImportAdmissionError;
        let _lease = self.begin_ordinary_read(&config)?;
        let marks_xml = company_marks_request.into_xml();
        self.execute(config, ReadRetryPolicy::SINGLE_ATTEMPT, |client| {
            let (recheck, marks_xml) = (&recheck, marks_xml.clone());
            async move {
                let (profile, _) = observe_read_boundary(&client).await?;
                if profile == DateBoundaryProfile::EducationRestricted {
                    return Err(MastersReadError::EducationUnqualified.into());
                }
                bracket_verified_company_identity(&client, identity).await?;
                // As the ledger queue reads its marks: a failed read is the
                // typed refusal, and nothing was sent.
                let binding = client
                    .post_xml_raw(marks_xml.clone())
                    .await
                    .map_err(|error| {
                        unconfirmed_unless_wire_refused(
                            error,
                            ApprovedImportAdmissionError::MastersUnconfirmed,
                        )
                    })?;
                // The standalone recheck keeps no evidence: a refusal is its
                // typed error alone, as it was before the reads were shared.
                let (candidate, groups, units) = read_stock_item_admission(
                    &client,
                    identity,
                    &binding.text,
                    parse_marks,
                    &mut RuntimeReadEvidence::empty(),
                )
                .await?;
                bracket_verified_company_identity(&client, identity).await?;
                confirm_read_boundary(&client, profile).await?;
                let aim = client.post_xml_raw(marks_xml).await.map_err(|error| {
                    unconfirmed_unless_wire_refused(
                        error,
                        ApprovedImportAdmissionError::CompanyScopeUnconfirmed,
                    )
                })?;
                recheck(StockItemRecheckReads {
                    candidate,
                    groups,
                    units,
                    company_marks_at_binding: binding.text,
                    company_marks: aim.text,
                })
            }
        })
        .await
    }
}
