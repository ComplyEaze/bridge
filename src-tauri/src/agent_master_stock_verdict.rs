//! The verdict on one stock-item read-back (#1381): a pure judge over what one
//! call holds, by the table the #1381 decision set, first match wins:
//!
//! 1. The read-back failed in transport or parse, or its two marks answers
//!    (binding and aim) give the batch company different marks:
//!    `masters_after_post_unconfirmed`, reason `masters_unconfirmed`.
//! 2. The pre-image cannot be read: `reconciliation_required`, reason
//!    `pre_image_unreadable`.
//! 3. Only in the call that sent the Create, with Tally's parsed answer in
//!    hand: the answer clean by `StockItemImportOutcome::of`, the company's
//!    mark one above the intent's aim mark, no pre-image item changed or
//!    missing, the read back `Equal`, and no other loaded company's mark moved
//!    nor the set of loaded companies changed: `posted_verified`, through
//!    `StockItemReadBackBasis::verify`.
//! 4. Anything else: `reconciliation_required`, with one static reason for
//!    each cause found.
//!
//! A later call never verifies: only counters are on file, which cannot show
//! the answer's status or line errors. So a batch whose read-back failed at
//! post time, or whose answer was lost, stays non-terminal and its name stays
//! held until row 1381-B adds a rule. The five other terminal states rest on
//! ledger-only evidence or the unmeasured ALTERID rule, and are never given.
use super::super::{is_canonical_guid, MasterMark, MasterVerdictState};
use super::{BasisRefusal, Journal, StockItemBatch, StockItemReadBackBasis};
use crate::tally::stock_item_create::StockItemCatalogue;
use crate::tally::stock_item_verified::{ReadBackVerdict, StockItemImportOutcome};
use crate::tally::VerifiedCompanyIdentity;
use std::collections::BTreeMap;
use std::path::Path;

/// Tally's answer to the Create, as the judging call holds it.
#[derive(Clone, Debug)]
pub(in crate::agent::agent_import) enum Answer {
    /// The call that sent the Create: the parsed answer, `None` when the body
    /// did not parse.
    Posted(Option<StockItemImportOutcome>),
    /// Any later call: rule 3 is skipped.
    Later,
}

/// The only three states the table gives; the verified item's lower-case GUID.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Verified(String),
    Unconfirmed,
    Open,
}

/// What the judge decided, with static reasons (none when verified).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::agent::agent_import) struct StockJudgement {
    outcome: Outcome,
    reasons: Vec<&'static str>,
}

impl StockJudgement {
    fn open(reasons: Vec<&'static str>) -> Self {
        let outcome = Outcome::Open;
        Self { outcome, reasons }
    }

    pub(in crate::agent::agent_import) fn state(&self) -> MasterVerdictState {
        match self.outcome {
            Outcome::Verified(_) => MasterVerdictState::PostedVerified,
            Outcome::Unconfirmed => MasterVerdictState::MastersAfterPostUnconfirmed,
            Outcome::Open => MasterVerdictState::ReconciliationRequired,
        }
    }

    pub(in crate::agent::agent_import) fn guid(&self) -> Option<&str> {
        match &self.outcome {
            Outcome::Verified(guid) => Some(guid),
            Outcome::Unconfirmed | Outcome::Open => None,
        }
    }

    pub(in crate::agent::agent_import) fn reasons(&self) -> &[&'static str] {
        &self.reasons
    }
}

/// What one stock batch's read-back is judged against, taken from the journal
/// and the company as verified afresh before the read: the basis, the batch
/// company's lower-case GUID and the marks of the intent's aim read.
pub(in crate::agent::agent_import) struct StockReadBack {
    pub(super) basis: Result<StockItemReadBackBasis, BasisRefusal>,
    pub(super) company_guid: String,
    pub(super) stored: Vec<MasterMark>,
}

/// The read-back's parsed values: the binding and aim marks and the items.
pub(in crate::agent::agent_import) type Fresh<'a> =
    (&'a [MasterMark], &'a [MasterMark], &'a StockItemCatalogue);

impl StockReadBack {
    pub(in crate::agent::agent_import) fn of(
        data_dir: &Path,
        journal: &Journal<StockItemBatch>,
        batch_id: &str,
        identity: &VerifiedCompanyIdentity,
    ) -> Self {
        let intent = (journal.state(batch_id)).and_then(|state| state.intent.as_ref());
        Self {
            basis: StockItemReadBackBasis::from_journal(data_dir, journal, batch_id, identity),
            company_guid: identity.company_guid().to_ascii_lowercase(),
            stored: intent.map_or_else(Vec::new, |intent| intent.pre_post_master_marks().to_vec()),
        }
    }

    /// The table above, on `fresh`: `None` when the read-back failed in
    /// transport or parse.
    pub(in crate::agent::agent_import) fn judge(
        &self,
        answer: &Answer,
        fresh: Option<Fresh<'_>>,
    ) -> StockJudgement {
        let company = self.company_guid.as_str();
        let unconfirmed = StockJudgement {
            outcome: Outcome::Unconfirmed,
            reasons: vec!["masters_unconfirmed"],
        };
        // Rule 1.
        let Some((Some(at_binding), Some(now), catalogue)) = fresh
            .map(|(binding, aim, catalogue)| (by_company(binding), by_company(aim), catalogue))
        else {
            return unconfirmed;
        };
        let after = match (now.get(company), at_binding.get(company)) {
            (Some(aim), Some(binding)) if aim == binding => *aim,
            _ => return unconfirmed,
        };
        // Rule 2.
        if let Err(BasisRefusal::PreImage(_)) = self.basis {
            return StockJudgement::open(vec!["pre_image_unreadable"]);
        }
        let mut reasons = Vec::new();
        reasons.extend(match answer {
            Answer::Later => Some("later_call_does_not_verify"),
            Answer::Posted(None) => Some("answer_not_parsed"),
            Answer::Posted(Some(outcome)) => match outcome {
                StockItemImportOutcome::CountersClean(_) => None,
                StockItemImportOutcome::ApplicationFailure => Some("tally_reported_failure"),
                StockItemImportOutcome::AlteredExisting { .. } => Some("tally_reported_altered"),
                StockItemImportOutcome::NotClean { .. } => Some("counters_not_clean"),
                StockItemImportOutcome::Unreported => Some("counters_not_reported"),
            },
        });
        // M: the company's mark after, less its mark at the aim read.
        let stored = by_company(&self.stored);
        reasons.extend(
            match stored.as_ref().and_then(|stored| stored.get(company)) {
                None => Some("target_mark_not_recorded"),
                Some(before) if after == *before => Some("target_mark_unmoved"),
                Some(before) if after < *before => Some("target_mark_moved_back"),
                Some(before) if after - before > 1 => Some("target_mark_moved_more_than_once"),
                Some(_) => None,
            },
        );
        // O: the other companies' marks, in both answers, and the set of them.
        match stored {
            Some(stored) if stored.keys().eq(now.keys()) && stored.keys().eq(at_binding.keys()) => {
                if (stored.iter()).any(|(guid, mark)| {
                    guid != company && (now[guid] != *mark || at_binding[guid] != *mark)
                }) {
                    reasons.push("other_company_mark_moved");
                }
            }
            _ => reasons.push("company_scope_changed"),
        }
        let basis = match &self.basis {
            Ok(basis) => basis,
            Err(refusal) => {
                reasons.push(basis_reason(refusal));
                return StockJudgement::open(reasons);
            }
        };
        // E: an earlier item changed, or missing from the fresh read.
        let read: BTreeMap<String, u64> = (catalogue.items().iter())
            .map(|item| (item.guid().to_ascii_lowercase(), item.alter_id()))
            .collect();
        for (guid, alter_id) in basis.pre_image() {
            let cause = match read.get(guid) {
                None => "earlier_item_missing",
                Some(now) if now != alter_id => "earlier_item_changed",
                Some(_) => continue,
            };
            if !reasons.contains(&cause) {
                reasons.push(cause);
            }
        }
        // B.
        reasons.extend(match basis.read_back(catalogue) {
            ReadBackVerdict::Equal(_) => None,
            ReadBackVerdict::Differs { .. } => Some("item_differs"),
            ReadBackVerdict::NotFoundOnce { .. } => Some("item_not_found_once"),
            ReadBackVerdict::NotNew => Some("item_not_new"),
            ReadBackVerdict::OtherCompany => Some("read_of_another_company"),
        });
        // Rule 3: only with a clean answer in hand and no cause found.
        let (Answer::Posted(Some(outcome)), true) = (answer, reasons.is_empty()) else {
            return StockJudgement::open(reasons);
        };
        match basis.verify(outcome.clone(), catalogue) {
            Ok(item) if is_canonical_guid(&item.guid().to_ascii_lowercase()) => StockJudgement {
                outcome: Outcome::Verified(item.guid().to_ascii_lowercase()),
                reasons: Vec::new(),
            },
            Ok(_) => StockJudgement::open(vec!["item_guid_not_canonical"]),
            Err(_) => StockJudgement::open(vec!["item_not_verified"]),
        }
    }
}

/// Each company's mark, by lower-case GUID; `None` when a GUID repeats.
fn by_company(marks: &[MasterMark]) -> Option<BTreeMap<String, u64>> {
    let mut map = BTreeMap::new();
    for mark in marks {
        let guid = mark.company_guid.to_ascii_lowercase();
        if map.insert(guid, mark.altmstid).is_some() {
            return None;
        }
    }
    Some(map)
}

/// The reason a basis refusal gives (the pre-image's is rule 2's).
fn basis_reason(refusal: &BasisRefusal) -> &'static str {
    match refusal {
        BasisRefusal::OtherCompanyName
        | BasisRefusal::OtherCompanyGuid
        | BasisRefusal::OtherCompanyNumber
        | BasisRefusal::OtherBooksFrom => "company_identity_changed",
        BasisRefusal::BeforeOfOtherCompany => "pre_image_of_another_company",
        BasisRefusal::PreImage(_) => "pre_image_unreadable",
        BasisRefusal::NoBatch
        | BasisRefusal::NoIntent
        | BasisRefusal::NotRebuilt(_)
        | BasisRefusal::NotRendered(_)
        | BasisRefusal::DigestMismatch
        | BasisRefusal::RequestDigestMismatch => "batch_record_not_readable",
    }
}
