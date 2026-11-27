//! One party's open bills through Tally's `Ledger Outstandings` report (#2377),
//! inside the identity and book-extent bracket the other report reads use.
//!
//! The ledger snapshot is the one the lab tied the answers to: `List of Ledgers`
//! from the books' first day to the same to-date. Both reads are paired, and the
//! answer is told only when its bills sum to the ledger's closing
//! ([`PartyBillsAnswer::tie`](bridge_tally_protocol::native_outstandings::PartyBillsAnswer::tie)).
//!
//! Tally answers a party it reports nothing for with no body at all, the same for a name no
//! ledger carries and for a bill-wise party with an empty ledger-list closing at the date
//! (protocol reference §12a.27). The answer cannot tell those apart, so the snapshot does, and
//! neither is told as "no open bills".
use super::*;
use crate::tally::connection::PartyBillsAnswerBody;
use bridge_tally_core::master_binding::letters_and_digits_key;
use bridge_tally_protocol::ledger_scope::LedgerScopeName;
use bridge_tally_protocol::native_outstandings::{
    parse_ledger_outstandings_answer, parse_native_ledger_snapshot_for_company,
    render_ledger_outstandings_request, render_native_ledger_snapshot_request,
    LedgerOutstandingsParty, LedgerOutstandingsPartyError, LedgerSnapshotEntry,
    NativeLedgerSnapshotPeriod, NativeLedgerSnapshotPeriodError, PartyBillsAnswer,
    PartyBillsAnswerError, PartyBillsRead, PartyBillsRefusal,
};

/// The party a `Ledger Outstandings` read is sent for (#2480): a name
/// [`LedgerOutstandingsParty`] admits and, when [`LedgerScopeName`] would not admit it, the one
/// ledger of the company's catalogue that has its letters-and-digits key, by any name that
/// ledger answers to. Every name with a mark the lab read through this report was read where no
/// other ledger shared that key (protocol reference §12a.34); a name the narrower type admits is
/// admitted as it was before.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OpenBillsParty(LedgerOutstandingsParty);

/// Why a party is not sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OpenBillsPartyRefusal {
    Name(LedgerOutstandingsPartyError),
    /// The number of catalogue ledgers with the name's letters-and-digits key is not one: another
    /// ledger shares it, or (a catalogue that does not hold the party) none has it.
    KeyShared,
}

impl OpenBillsPartyRefusal {
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::Name(refusal) => refusal.code(),
            Self::KeyShared => "party_name_key_shared",
        }
    }
}

impl OpenBillsParty {
    /// `ledgers` is the company's whole ledger catalogue, each ledger as every name it answers
    /// to: its row spelling, its stored name and its aliases.
    pub(crate) fn admit<'a, L, N>(name: &str, ledgers: L) -> Result<Self, OpenBillsPartyRefusal>
    where
        L: IntoIterator<Item = N>,
        N: IntoIterator<Item = &'a str>,
    {
        let party = LedgerOutstandingsParty::new(name).map_err(OpenBillsPartyRefusal::Name)?;
        if LedgerScopeName::new(name).is_err() {
            let key = letters_and_digits_key(name);
            let sharing = ledgers
                .into_iter()
                .map(|names| {
                    names
                        .into_iter()
                        .any(|spelling| letters_and_digits_key(spelling) == key)
                })
                .filter(|shares| *shares)
                .count();
            if sharing != 1 {
                return Err(OpenBillsPartyRefusal::KeyShared);
            }
        }
        Ok(Self(party))
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

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
    /// The report sent nothing for a bill-wise party whose snapshot closing is empty. Whether it
    /// has open bills is not established: an empty closing is not established to be zero, and even
    /// a zero balance may hold an open bill set against an on-account advance (protocol reference
    /// §12a.27).
    #[error("party_bills_none_reported")]
    NoneReported,
    /// The report sent nothing for a bill-wise party whose snapshot shows a closing: not measured.
    #[error("party_bills_answer_missing")]
    AnswerMissing,
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
            Self::NoneReported => "party_bills_none_reported",
            Self::AnswerMissing => "party_bills_answer_missing",
            Self::Answer(error) => error.code(),
            Self::Refused(refusal) => refusal.code(),
        }
    }
}

/// The party's answer told against its ledger in the snapshot. The snapshot is consulted first,
/// so a name it does not carry and a ledger that keeps no bills refuse by those codes whatever
/// the report answered. `answer` is `None` for the empty answer (protocol reference §12a.27),
/// which is never told as no open bills: a bill-wise party whose closing is empty refuses as
/// `party_bills_none_reported`, and one with a closing as `party_bills_answer_missing`.
fn tie_to_snapshot(
    snapshot: Vec<LedgerSnapshotEntry>,
    party: &str,
    answer: Option<Result<PartyBillsAnswer, PartyBillsAnswerError>>,
) -> Result<PartyBillsRead, PartyBillsReadError> {
    let ledger = snapshot
        .into_iter()
        .find(|entry| entry.name == party)
        .ok_or(PartyBillsReadError::NotInSnapshot)?;
    if !ledger.bill_wise_on {
        return Err(PartyBillsReadError::NotBillWise);
    }
    let Some(answer) = answer else {
        return Err(match ledger.closing_balance {
            None => PartyBillsReadError::NoneReported,
            Some(_) => PartyBillsReadError::AnswerMissing,
        });
    };
    answer
        .map_err(PartyBillsReadError::Answer)?
        .tie(ledger.closing_balance.as_ref())
        .map_err(PartyBillsReadError::Refused)
}

impl TallyRuntime {
    /// `party`'s open bills at `to`: the ledger snapshot, then the party's report,
    /// each paired, between two equal book extents and two identity brackets.
    pub(crate) async fn fetch_party_bills(
        &self,
        config: TallyConfig,
        identity: &VerifiedCompanyIdentity,
        party: OpenBillsParty,
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
                        render_ledger_outstandings_request(identity.display_name(), &party.0, &to);
                    let (body, bytes, hash) =
                        client.fetch_party_bills_paired(request.clone()).await?;
                    evidence = evidence
                        .clone()
                        .combine(RuntimeReadEvidence::paired(&request, hash, bytes));
                    // Held until the closing extent is read: a book that moved
                    // mid-read is reported as moved, not as what its answer broke.
                    let answer = match &body {
                        PartyBillsAnswerBody::Rows(xml) => {
                            Some(parse_ledger_outstandings_answer(xml, &books_from, &to))
                        }
                        PartyBillsAnswerBody::Empty => None,
                    };

                    let closing_extent = client.fetch_company_book_extent(&identity).await?;
                    if closing_extent != extent {
                        return Err(PairedReadValidationError::NativeLedgerExtent.into());
                    }
                    let read = tie_to_snapshot(snapshot?, party.as_str(), answer)?;
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
