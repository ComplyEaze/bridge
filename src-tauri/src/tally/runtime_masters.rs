//! One native masters read, inside the bracket the Trial Balance read uses.
use super::*;
use crate::tally::connection::compliance_estimate;
use bridge_tally_protocol::native_masters::{
    masters_worst_row_bytes, parse_native_masters, render_native_masters_request, NativeMasterKind,
    NativeMasterRow, NativeMasters, MASTERS_RESPONSE_BUDGET_BYTES,
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
) -> anyhow::Result<Result<NativeMasters, MastersReadError>> {
    let estimate = sized_before_the_read(native)
        .then(|| admit_masters_size(native, mark))
        .transpose()?;
    let limit = limit_for(native, estimate);
    let request = render_native_masters_request(native, identity.display_name());
    let (xml, bytes, hash) = client
        .fetch_native_report_paired(request.clone())
        .await?
        .require_stable(PairedReadValidationError::MastersCollection)?;
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
    let request = render_voucher_type_numbering_request(identity.display_name());
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
                    let held: Result<MastersRows, MastersReadError> = match kind {
                        MastersKind::Native(native) => {
                            let mark = extent
                                .master_alter_id_high_water()
                                .ok_or(OutstandingsError::MasterWitnessAbsent)?
                                .get();
                            match read_native_rows(&client, &identity, native, mark, &mut evidence)
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
                                render_native_group_snapshot_request(identity.display_name());
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
                    let masters = read_native_rows(
                        &client,
                        &identity,
                        NativeMasterKind::VoucherTypes,
                        master_mark,
                        &mut evidence,
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
}
