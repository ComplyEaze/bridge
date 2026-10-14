//! Retiring a batch that was never sent: what a read of Tally has to show
//! before ComplyEaze Bridge stops counting the batch's Contras, and the window
//! that read covers.
//!
//! A batch that was built and never sent may have been imported by hand from
//! its saved file. Until Tally has been read for it, it counts as carrying its
//! transfers: the other account's statement cannot withdraw a skip it may
//! carry. `verify_import` with `if_absent` set to `retire` reads Tally for one
//! named batch and, when the batch is absent, removes its saved file and
//! records it retired, which takes it out of the transfer facts and out of
//! `post_import`. The decision is made here, from the verification's rows.
use super::statement_facts::TransferFacts;
use super::*;
use crate::agent::transfer_pairing::{ContraKey, SkipState, TransferView};
use chrono::{Duration, NaiveDate};

/// What the read of Tally is judged on, and what it is judged against.
pub(super) struct Request {
    pub(super) company_guid: String,
    /// The lineage of the batch being read for.
    pub(super) own_lineage: String,
    /// The batch's statement rows: the only ones a sibling can account for.
    pub(super) statement_rows: BTreeSet<String>,
    /// The verification's row for each voucher of the batch.
    pub(super) rows: Vec<Value>,
}

/// What a retirement did, besides recording it.
pub(super) struct Retirement {
    /// The batch's Contra rows that another statement's skipped row is waiting
    /// on, or `None` when the transfer facts could not be read.
    pub(super) carried_rows: Option<Vec<String>>,
}

/// The Contra rows of `batch` that a skipped row of another statement is
/// paired with, may be carried by, or waits on. Retiring the batch leaves each
/// of them to that other statement, so the batch's own statement is built again
/// with them skipped.
pub(super) fn carried_rows(facts: &TransferFacts, view: &TransferView, batch: &str) -> Vec<String> {
    let ours = |key: &ContraKey| key.batch.as_str() == batch;
    let mut rows = BTreeSet::new();
    for (key, state) in &view.skips {
        match state {
            SkipState::Paired { contra, .. } | SkipState::MatchedUnconfirmed { contra }
                if ours(contra) =>
            {
                rows.insert(contra.txn.as_str().to_string());
            }
            SkipState::Ambiguous { candidates }
            | SkipState::ReversedSameDay { candidates }
            | SkipState::ReferenceDiffers { candidates } => {
                rows.extend(
                    candidates
                        .iter()
                        .filter(|candidate| ours(candidate))
                        .map(|candidate| candidate.txn.as_str().to_string()),
                );
            }
            // the view names the batch and not the Contra: the batch's Contra of
            // the skipped row's day and amount
            SkipState::OtherSideUnverified { batch: carrier } if carrier.as_str() == batch => {
                if let Some(skip) = facts.skips.iter().find(|skip| &skip.key == key) {
                    rows.extend(
                        facts
                            .contras
                            .iter()
                            .filter(|contra| {
                                ours(&contra.key)
                                    && contra.date == skip.date
                                    && contra.amount.numeric_eq(&skip.amount)
                            })
                            .map(|contra| contra.key.txn.as_str().to_string()),
                    );
                }
            }
            _ => {}
        }
    }
    rows.into_iter().collect()
}

pub(super) const RETIRED_CARRIED_STEP: &str = "This batch is retired: ComplyEaze Bridge read Tally, found none of its vouchers, removed its saved file and will not send it. Build this statement again with the rows in carried_rows skipped (treatment skip): the other account's statement books them, and posting them from this side would book each transfer twice.";
pub(super) const RETIRED_UNKNOWN_STEP: &str = "This batch is retired: ComplyEaze Bridge read Tally, found none of its vouchers, removed its saved file and will not send it. Whether another account's statement carries any of its rows could not be established. Run verify_import on the other account's batches first, then build this statement again with every row they carry skipped.";
pub(super) const RETIRED_STEP: &str = "This batch is retired: ComplyEaze Bridge read Tally, found none of its vouchers, removed its saved file and will not send it. Build this statement again if its rows are still to be posted.";
pub(super) const KEPT_STEP: &str = "Tally holds a voucher of this batch, so it was not retired and still counts. Its saved file is removed, so ComplyEaze Bridge will not send it.";
const UNFINISHED_STEP: &str = "The read of Tally did not finish, so nothing was retired and the batch still counts. Its saved file is removed, so the batch can no longer be sent from it. Call verify_import on it again with if_absent set to retire.";

/// The answer when the saved file is gone and the read, or the record, did not
/// complete: nothing is retired, nothing was verified, and the call may be
/// repeated.
pub(super) fn unfinished_outcome(
    company: Value,
    batch_id: &str,
    code: &str,
    accumulated: Evidence,
    guid: &str,
) -> ToolOutcome {
    ToolOutcome {
        payload: json!({"company": company, "result": {"batch_id": batch_id, "retire": {
            "state": "not_retired", "reason": code, "file_removed": true,
            "verification_recorded": false, "next_step": UNFINISHED_STEP}}}),
        evidence: Evidence {
            state: "partial",
            reason_code: Some(code.to_string()),
            ..accumulated
        },
        company_guid: Some(guid.to_string()),
        truncated: false,
    }
}

/// Whether a retirement ended without a verification, so there is no proof to
/// serve.
pub(super) fn unfinished(payload: &Value) -> bool {
    payload["result"]["retire"]["verification_recorded"] == json!(false)
}

/// How many days either side of the batch's own dates the read looks, so a
/// voucher a person imported by hand and then re-dated a little is still seen
/// by its narration tag.
pub(super) const RETIRE_WIDEN_DAYS: i64 = 7;

/// Why a read that found the batch did not retire it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Keep {
    /// A voucher in Tally carries this batch's own narration tag: its file was
    /// imported, in whole or part.
    OwnTag,
    /// A voucher in Tally has this batch's content, or is the batch's under
    /// some other mark, and nothing accounts for it.
    VoucherInTally,
}

impl Keep {
    pub(super) fn reason(self) -> &'static str {
        match self {
            Self::OwnTag => "own_tag_in_tally",
            Self::VoucherInTally => "voucher_in_tally",
        }
    }
}

/// Whether the rows of a verification of a never-sent batch show it absent.
///
/// No voucher carries the batch's own tag, and every voucher of the batch is
/// either not found, or matches by content alone a voucher that a batch of
/// another lineage, found posted, holds for the same statement row:
/// that batch's own post accounts for the match, and it is not this batch's
/// voucher. Anything else (a content match nothing accounts for, a voucher
/// found by its tag, a duplicate) keeps the batch counting.
pub(super) fn judge(rows: &[Value], siblings: &BTreeSet<String>) -> Result<(), Keep> {
    if rows.iter().any(|row| row["marker"] == "narration_tag") {
        return Err(Keep::OwnTag);
    }
    for row in rows {
        let accounted_for = || {
            row["bridge_txn_id"].as_str().is_some_and(|row| {
                bridge_bank_statement::proposals::is_statement_txn_id(row) && siblings.contains(row)
            })
        };
        match row["status"].as_str() {
            Some("not_found") => {}
            Some("matching_content_observed") if accounted_for() => {}
            _ => return Err(Keep::VoucherInTally),
        }
    }
    Ok(())
}

/// Whether any voucher row of a verification carries the batch's own tag.
pub(super) fn sees_its_own_tag(rows: &[Value]) -> bool {
    rows.iter().any(|row| row["marker"] == "narration_tag")
}

/// The days the retire's read covers: the batch's own dates widened by
/// [`RETIRE_WIDEN_DAYS`] either side, limited by what a book can hold (not
/// before it began, not past today) and by the boundaries the licence admits.
///
/// An Education licence admits a read boundary only on day 1, 2 or 31 of a
/// month that has one (IMPLEMENTATION_GUIDE §2.7, I5). A widened date that
/// the profile rejects is moved outward, to the nearest admitted date, never
/// inward; if that date is before the book began or past today, no admitted
/// window covers the widened one and there is none (the retire is refused and
/// the batch keeps counting).
pub(super) fn widened_window(
    from: &TallyDate,
    to: &TallyDate,
    profile: DateBoundaryProfile,
    today: &TallyDate,
    books_from: Option<&TallyDate>,
) -> Option<(TallyDate, TallyDate)> {
    let date = |date: &TallyDate| NaiveDate::parse_from_str(date.as_str(), "%Y%m%d").ok();
    let tally = |date: NaiveDate| TallyDate::parse(date.format("%Y%m%d").to_string()).ok();
    let (from, to, today) = (date(from)?, date(to)?, date(today)?);
    let first = match books_from {
        Some(first) => Some(date(first)?),
        None => None,
    };
    let mut low = from - Duration::days(RETIRE_WIDEN_DAYS);
    if let Some(first) = first {
        low = low.max(first);
    }
    let mut high = (to + Duration::days(RETIRE_WIDEN_DAYS)).min(today.max(to));
    // outward only: earlier for the start, later for the end
    while !profile.accepts_boundary(&tally(low)?) {
        low -= Duration::days(1);
        if (from - low).num_days() > 62 {
            return None;
        }
    }
    while !profile.accepts_boundary(&tally(high)?) {
        high += Duration::days(1);
        if (high - to).num_days() > 62 {
            return None;
        }
    }
    if first.is_some_and(|first| low < first) || high > today.max(to) {
        return None;
    }
    Some((tally(low)?, tally(high)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(text: &str) -> TallyDate {
        TallyDate::parse(text).unwrap()
    }

    fn row(txn: &str, status: &str, marker: &str) -> Value {
        json!({"bridge_txn_id": txn, "status": status, "marker": marker})
    }

    const ROW: &str = "st-20260901-00000000000000a1";

    #[test]
    fn only_absence_or_a_sibling_accounted_match_retires() {
        let none = BTreeSet::new();
        let siblings = BTreeSet::from([ROW.to_string()]);
        let absent = row(ROW, "not_found", "");
        assert_eq!(judge(std::slice::from_ref(&absent), &none), Ok(()));
        let content = row(ROW, "matching_content_observed", "accounting_fingerprint");
        // nothing accounts for a content match
        assert_eq!(
            judge(std::slice::from_ref(&content), &none),
            Err(Keep::VoucherInTally)
        );
        // a found batch of another lineage holds the same row
        assert_eq!(judge(&[content.clone(), absent.clone()], &siblings), Ok(()));
        // a row that is not a statement row is never accounted for that way
        let by_hand = row(
            "hand-1",
            "matching_content_observed",
            "accounting_fingerprint",
        );
        let hand = BTreeSet::from(["hand-1".to_string()]);
        assert_eq!(judge(&[by_hand], &hand), Err(Keep::VoucherInTally));
    }

    #[test]
    fn a_voucher_with_the_batchs_own_tag_blocks_whatever_else_is_accounted_for() {
        let siblings = BTreeSet::from([ROW.to_string()]);
        for status in [
            "posted_verified",
            "posted_divergent",
            "not_attributable",
            "duplicate_fingerprint",
        ] {
            let tagged = row(ROW, status, "narration_tag");
            assert_eq!(
                judge(std::slice::from_ref(&tagged), &siblings),
                Err(Keep::OwnTag),
                "{status}"
            );
            assert!(sees_its_own_tag(&[tagged]));
        }
        assert!(!sees_its_own_tag(&[row(ROW, "not_found", "")]));
        // any other status keeps the batch counting
        for status in [
            "posted_not_effective",
            "duplicate_fingerprint",
            "bound_not_in_window",
        ] {
            assert_eq!(
                judge(&[row(ROW, status, "accounting_fingerprint")], &siblings),
                Err(Keep::VoucherInTally),
                "{status}"
            );
        }
    }

    #[test]
    fn the_window_is_widened_a_week_either_side_within_the_book() {
        let any = DateBoundaryProfile::ModeAgnostic;
        let wide = widened_window(
            &day("20260910"),
            &day("20260915"),
            any,
            &day("20261201"),
            None,
        );
        assert_eq!(wide, Some((day("20260903"), day("20260922"))));
        // not before the book began, and not past today
        let wide = widened_window(
            &day("20260403"),
            &day("20260915"),
            any,
            &day("20260918"),
            Some(&day("20260401")),
        );
        assert_eq!(wide, Some((day("20260401"), day("20260918"))));
        // a batch dated today reads to today
        let wide = widened_window(
            &day("20260915"),
            &day("20260915"),
            any,
            &day("20260915"),
            None,
        );
        assert_eq!(wide, Some((day("20260908"), day("20260915"))));
    }

    #[test]
    fn an_education_window_moves_outward_to_an_admitted_day_or_is_refused() {
        let education = DateBoundaryProfile::EducationRestricted;
        // 3 Sept is not admitted: the start moves back to the 2nd; 22 Sept
        // moves on to 1 October (30 September has no 31st)
        let wide = widened_window(
            &day("20260910"),
            &day("20260915"),
            education,
            &day("20261201"),
            None,
        );
        assert_eq!(wide, Some((day("20260902"), day("20261001"))));
        // the admitted day after the widened end is past today: no window
        let wide = widened_window(
            &day("20260910"),
            &day("20260915"),
            education,
            &day("20260925"),
            None,
        );
        assert_eq!(wide, None);
        // the start would have to leave the book
        let wide = widened_window(
            &day("20260405"),
            &day("20260406"),
            education,
            &day("20261201"),
            Some(&day("20260403")),
        );
        assert_eq!(wide, None);
    }
}
