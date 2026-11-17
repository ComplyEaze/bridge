//! One party's open bills through Tally's `Ledger Outstandings` report (#2377),
//! inside the identity and book-extent bracket the other report reads use.
//!
//! The ledger snapshot is the one the lab tied the answers to: `List of Ledgers`
//! from the books' first day to the same to-date. Both reads are paired, and the
//! answer is told only when its bills sum to the ledger's closing
//! ([`PartyBillsAnswer::tie`](bridge_tally_protocol::native_outstandings::PartyBillsAnswer::tie)).
use super::*;
use bridge_tally_protocol::ledger_scope::LedgerScopeName;
use bridge_tally_protocol::native_outstandings::{
    parse_ledger_outstandings_answer, parse_native_ledger_snapshot_for_company,
    render_ledger_outstandings_request, render_native_ledger_snapshot_request,
    NativeLedgerSnapshotPeriod, NativeLedgerSnapshotPeriodError, PartyBillsAnswerError,
    PartyBillsRead, PartyBillsRefusal,
};

/// A party's open bills, tied to its ledger at `to`.
pub(crate) struct PartyBillsLoad {
    pub(crate) books_from: TallyDate,
    pub(crate) read: PartyBillsRead,
    pub(crate) evidence: RuntimeReadEvidence,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum PartyBillsReadError {
    #[error("party_bills_education_unqualified")]
    EducationUnqualified,
    #[error("party_bills_period_not_honoured")]
    Period(NativeLedgerSnapshotPeriodError),
    /// The resolved ledger is not in the snapshot: renamed or deleted between the reads.
    #[error("party_not_in_book")]
    NotInSnapshot,
    #[error("party_not_billwise")]
    NotBillWise,
    #[error("{}", .0.code())]
    Answer(PartyBillsAnswerError),
    #[error("{}", .0.code())]
    Refused(PartyBillsRefusal),
}

impl PartyBillsReadError {
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::EducationUnqualified => "party_bills_education_unqualified",
            Self::Period(_) => "party_bills_period_not_honoured",
            Self::NotInSnapshot => "party_not_in_book",
            Self::NotBillWise => "party_not_billwise",
            Self::Answer(error) => error.code(),
            Self::Refused(refusal) => refusal.code(),
        }
    }
}

impl TallyRuntime {
    /// `party`'s open bills at `to`: the ledger snapshot, then the party's report,
    /// each paired, between two equal book extents and two identity brackets.
    pub(crate) async fn fetch_party_bills(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        party: LedgerScopeName,
        to: TallyDate,
    ) -> anyhow::Result<PartyBillsLoad> {
        let _lease = self.begin_ordinary_read(&config)?;
        self.execute(config, ReadRetryPolicy::SINGLE_ATTEMPT, move |client| {
            let identity = identity.clone();
            let party = party.clone();
            let to = to.clone();
            async move {
                let mut evidence = RuntimeReadEvidence::empty();
                let result = async {
                    let (profile, mode_evidence) = observe_read_boundary(&client).await?;
                    evidence = mode_evidence;
                    if profile == DateBoundaryProfile::EducationRestricted {
                        return Err(PartyBillsReadError::EducationUnqualified.into());
                    }
                    bracket_verified_company_identity(&client, &identity).await?;
                    let extent = client.fetch_company_book_extent(&identity).await?;
                    let books_from = extent.books_from().clone();
                    let period =
                        NativeLedgerSnapshotPeriod::new(profile, books_from.clone(), to.clone())
                            .map_err(PartyBillsReadError::Period)?;

                    let request =
                        render_native_ledger_snapshot_request(identity.company_name(), &period);
                    let (xml, bytes, hash) = client
                        .fetch_native_report_paired(request.clone())
                        .await?
                        .require_stable(PairedReadValidationError::NativeLedgerCollection)?;
                    evidence = evidence
                        .clone()
                        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                    let snapshot =
                        parse_native_ledger_snapshot_for_company(&xml, identity.company_guid());

                    let request =
                        render_ledger_outstandings_request(identity.display_name(), &party, &to);
                    let (xml, bytes, hash) = client
                        .fetch_native_report_paired(request.clone())
                        .await?
                        .require_stable(PairedReadValidationError::PartyBills)?;
                    evidence = evidence
                        .clone()
                        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                    // Held until the closing extent is read: a book that moved
                    // mid-read is reported as moved, not as what its answer broke.
                    let answer = parse_ledger_outstandings_answer(&xml, &books_from, &to);

                    let closing_extent = client.fetch_company_book_extent(&identity).await?;
                    if closing_extent != extent {
                        return Err(PairedReadValidationError::NativeLedgerExtent.into());
                    }
                    let ledger = snapshot?
                        .into_iter()
                        .find(|entry| entry.name == party.as_str())
                        .ok_or(PartyBillsReadError::NotInSnapshot)?;
                    if !ledger.bill_wise_on {
                        return Err(PartyBillsReadError::NotBillWise.into());
                    }
                    let read = answer
                        .map_err(PartyBillsReadError::Answer)?
                        .tie(ledger.closing_balance.as_ref())
                        .map_err(PartyBillsReadError::Refused)?;
                    bracket_verified_company_identity(&client, &identity).await?;
                    evidence = evidence
                        .clone()
                        .combine(confirm_read_boundary(&client, profile).await?);
                    Ok(PartyBillsLoad {
                        books_from,
                        read,
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

#[cfg(test)]
#[path = "runtime_party_bills_tests.rs"]
mod tests;
