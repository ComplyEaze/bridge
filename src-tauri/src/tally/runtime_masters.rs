//! One native masters read, inside the bracket the Trial Balance read uses.
use super::*;
use crate::tally::connection::compliance_estimate;
use bridge_tally_protocol::native_masters::{
    masters_worst_row_bytes, parse_native_masters, render_native_masters_request, NativeMasterKind,
    NativeMasterRow, NativeMasters, MASTERS_RESPONSE_BUDGET_BYTES,
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
            Self::Groups => "groups",
        }
    }
}

pub(crate) enum MastersRows {
    Native(NativeMasters),
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
}

impl MastersReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "masters_education_unqualified",
            Self::TooLarge { .. } => "masters_too_large",
            Self::PremiseViolated(_) => "masters_bound_premise_violated",
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
        self.execute(
            config,
            ReadOperation::MasterExport,
            ReadRetryPolicy::SINGLE_ATTEMPT,
            move |client| {
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
                        // broken size premise is held until the closing extent
                        // is read: a book that moved mid-read is reported as
                        // moved, not as the rows its new masters added.
                        let held: Result<MastersRows, MastersReadError> = match kind {
                            MastersKind::Native(native) => {
                                let mark = extent
                                    .master_alter_id_high_water()
                                    .ok_or(OutstandingsError::MasterWitnessAbsent)?
                                    .get();
                                let estimate = sized_before_the_read(native)
                                    .then(|| admit_masters_size(native, mark))
                                    .transpose()?;
                                let limit = limit_for(native, estimate);
                                let request =
                                    render_native_masters_request(native, identity.display_name());
                                let (xml, bytes, hash) = client
                                    .fetch_native_report_paired(request.clone())
                                    .await?
                                    .require_stable(PairedReadValidationError::MastersCollection)?;
                                evidence = evidence
                                    .clone()
                                    .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                                let masters =
                                    parse_native_masters(native, &xml, identity.company_guid())?;
                                check_masters_premise(&masters.rows, mark, bytes, limit)
                                    .map(|()| MastersRows::Native(masters))
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
            },
        )
        .await
    }
}
