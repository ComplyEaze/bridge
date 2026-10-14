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

/// What the read of Tally is judged on.
pub(super) struct Request {
    pub(super) company_guid: String,
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

pub(super) const RETIRED_CARRIED_STEP: &str = "This batch is retired: ComplyEaze Bridge read Tally, found no voucher with its narration tag or its content, removed its saved file and will not send it. Build this statement again with the rows in carried_rows skipped (treatment skip): the other account's statement books them, and posting them from this side would book each transfer twice.";
pub(super) const RETIRED_UNKNOWN_STEP: &str = "This batch is retired: ComplyEaze Bridge read Tally, found no voucher with its narration tag or its content, removed its saved file and will not send it. Whether another account's statement carries any of its rows could not be established. Run verify_import on the other account's batches first, then build this statement again with every row they carry skipped.";
pub(super) const RETIRED_STEP: &str = "This batch is retired: ComplyEaze Bridge read Tally, found no voucher with its narration tag or its content, removed its saved file and will not send it. Build this statement again if its rows are still to be posted.";
pub(super) const KEPT_STEP: &str = "The read of Tally did not find this batch absent: a voucher carries its narration tag, matches its content, or could not be read as absent (`reason` says which), so it was not retired and still counts. Its saved file is removed, so a new post_import call for it is refused; a send already waiting for approval is not stopped by the removal.";
const UNFINISHED_STEP: &str = "The read of Tally did not finish, so nothing was retired and the batch still counts. Its saved file is removed, so a new post_import call for it is refused. Call verify_import on it again with if_absent set to retire.";
const UNFINISHED_FINAL_STEP: &str = "The read of Tally was refused for a reason that a repeat will not change, so nothing was retired and the batch still counts. Its saved file is removed, so a new post_import call for it is refused; a repeat will not help until the read limit or the licence mode that stopped the read changes, and no step inside ComplyEaze Bridge releases the batch yet.";
const CONFLICT_STEP: &str = "Another record of this batch was written meanwhile, so nothing was retired. Call verify_import on it again.";
const RECORD_UNCONFIRMED_STEP: &str = "The read of Tally finished, but ComplyEaze Bridge could not confirm that the record of it was written, so it is not established whether the batch is retired. The import folder is marked as needing recovery, and verify_import is refused for this batch until that is cleared, so there is no call to make on it now. Nothing sends it meanwhile: its saved file is removed. Report this with the reason named here.";

/// The code a failure carries when the record of the retirement may already be
/// in the journal.
pub(super) const RECORD_MAY_EXIST: &str = "retire_record_may_exist";

/// A refusal of the read that a repeat of the call cannot change: the window
/// is over the read limits (the plan refuses it, or one part is over the
/// transport's size, or a part starts or ends on a day the licence does not
/// serve), or the licence does not qualify an absence. The list is the
/// refusals that the plan and the window read can raise, and the closing
/// qualification; a refusal that can only arrive before the read
/// (`import_post_window_not_bounded` belongs to the check before a post) or
/// after the record is written (`verification_too_large_to_report` belongs to
/// serving the page) is not here, nor is a transient one such as a busy port,
/// nor `voucher_window_volume_unestimated`, which a census that timed out or
/// was too large raises and a later census may not.
fn repeat_cannot_help(code: &str) -> bool {
    matches!(
        code,
        "voucher_window_book_too_large"
            | "voucher_window_too_many_reads"
            | "voucher_window_part_over_budget"
            | "verification_window_day_not_readable"
            | "response_size_limit_exceeded"
            | "verification_mode_unqualified"
            | "window_part_boundary_unsupported_in_education"
    )
}

/// A refusal the plan of the window read raises, before any data part is asked
/// for. It arrives before the saved file is removed, so it leaves the batch as
/// it was. The list is the refusals that depend on the book and the licence
/// alone.
pub(super) fn refused_by_the_plan(code: &str) -> bool {
    matches!(
        code,
        "voucher_window_book_too_large"
            | "voucher_window_too_many_reads"
            | "voucher_window_part_over_budget"
            | "voucher_window_volume_unestimated"
            | "window_part_boundary_unsupported_in_education"
    )
}

const PLAN_REFUSED_STEP: &str = "The read of Tally was refused before anything was changed: the saved file of this batch is still in place, it was not retired and it still counts. A repeat will not help until the read limit or the licence mode that stopped the read changes.";
const PLAN_REFUSED_REPEATABLE_STEP: &str = "The read of Tally was refused before anything was changed: the saved file of this batch is still in place, it was not retired and it still counts. Call verify_import on it again with if_absent set to retire.";

/// The answer when the plan of the read refused it: the batch is exactly as it
/// was. Nothing is verified, so there is no page to serve.
pub(super) fn planned_refusal_outcome(
    company: Value,
    batch_id: &str,
    code: &str,
    accumulated: Evidence,
    guid: &str,
) -> ToolOutcome {
    let step = if repeat_cannot_help(code) {
        PLAN_REFUSED_STEP
    } else {
        PLAN_REFUSED_REPEATABLE_STEP
    };
    ToolOutcome {
        payload: json!({"company": company, "result": {"batch_id": batch_id, "retire": {
            "state": "not_retired", "reason": code, "file_removed": false,
            "verification_recorded": false, "next_step": step}}}),
        evidence: Evidence {
            state: "partial",
            reason_code: Some(code.to_string()),
            ..accumulated
        },
        company_guid: Some(guid.to_string()),
        truncated: false,
    }
}

/// The answer when the saved file is gone and the read, or the record, did not
/// complete. Nothing is verified, so there is no page to serve. What it says
/// follows what is known: that nothing was retired (the read, or the record
/// refused before it was appended), that another record of the batch was
/// written meanwhile, that a repeat cannot help, or, when the record may
/// already be in the journal, that it is not established.
pub(super) fn unfinished_outcome(
    company: Value,
    batch_id: &str,
    code: &str,
    record_may_exist: bool,
    accumulated: Evidence,
    guid: &str,
) -> ToolOutcome {
    let (state, step) = if record_may_exist {
        ("not_established", RECORD_UNCONFIRMED_STEP)
    } else if code == "import_verification_conflict_retry" {
        ("not_retired", CONFLICT_STEP)
    } else if repeat_cannot_help(code) {
        ("not_retired", UNFINISHED_FINAL_STEP)
    } else {
        ("not_retired", UNFINISHED_STEP)
    };
    ToolOutcome {
        payload: json!({"company": company, "result": {"batch_id": batch_id, "retire": {
            "state": state, "reason": code, "file_removed": true,
            "verification_recorded": false, "next_step": step}}}),
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
    /// some other mark.
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
/// not found. Anything else (a voucher found by its tag, a content match, a
/// duplicate) keeps the batch counting: a content match is credited to no other
/// batch, because nothing compares the matched voucher with another batch's.
pub(super) fn judge(rows: &[Value]) -> Result<(), Keep> {
    if rows.iter().any(|row| row["marker"] == "narration_tag") {
        return Err(Keep::OwnTag);
    }
    if rows
        .iter()
        .all(|row| row["status"].as_str() == Some("not_found"))
    {
        Ok(())
    } else {
        Err(Keep::VoucherInTally)
    }
}

/// Whether a verification's rows show any voucher of the batch in Tally: any
/// row that is not "not found", whether it was found by the batch's tag or by
/// its content. A row that names no status counts as found.
pub(super) fn found_a_voucher(rows: &[Value]) -> bool {
    rows.iter()
        .any(|row| row["status"].as_str() != Some("not_found"))
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
    fn only_absence_retires() {
        let absent = row(ROW, "not_found", "");
        assert_eq!(judge(std::slice::from_ref(&absent)), Ok(()));
        // a content match is credited to nothing
        let content = row(ROW, "matching_content_observed", "accounting_fingerprint");
        assert_eq!(
            judge(std::slice::from_ref(&content)),
            Err(Keep::VoucherInTally)
        );
        assert_eq!(judge(&[content, absent]), Err(Keep::VoucherInTally));
        // a row that names no status keeps the batch counting
        assert_eq!(
            judge(&[json!({"bridge_txn_id": ROW})]),
            Err(Keep::VoucherInTally)
        );
    }

    #[test]
    fn a_voucher_with_the_batchs_own_tag_blocks_the_retire() {
        for status in [
            "posted_verified",
            "posted_divergent",
            "not_attributable",
            "duplicate_fingerprint",
        ] {
            let tagged = row(ROW, status, "narration_tag");
            assert_eq!(
                judge(std::slice::from_ref(&tagged)),
                Err(Keep::OwnTag),
                "{status}"
            );
            assert!(found_a_voucher(&[tagged]));
        }
        assert!(!found_a_voucher(&[row(ROW, "not_found", "")]));
        // any other status keeps the batch counting
        for status in [
            "posted_not_effective",
            "duplicate_fingerprint",
            "bound_not_in_window",
        ] {
            assert_eq!(
                judge(&[row(ROW, status, "accounting_fingerprint")]),
                Err(Keep::VoucherInTally),
                "{status}"
            );
        }
    }

    /// A batch is seen again by any row that is not "not found": a content match
    /// with no tag counts as a tagged voucher does.
    #[test]
    fn any_row_that_is_not_absent_is_a_find() {
        assert!(!found_a_voucher(&[]));
        assert!(!found_a_voucher(&[row(ROW, "not_found", "")]));
        for status in [
            "matching_content_observed",
            "posted_verified",
            "posted_divergent",
            "not_attributable",
            "duplicate_fingerprint",
            "posted_not_effective",
            "bound_not_in_window",
        ] {
            assert!(
                found_a_voucher(&[row(ROW, "not_found", ""), row(ROW, status, "")]),
                "{status}"
            );
        }
        // a row with no status is not read as absent
        assert!(found_a_voucher(&[json!({"bridge_txn_id": ROW})]));
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

    fn evidence() -> Evidence {
        Evidence {
            request_sha256: "a".repeat(64),
            response_sha256: "b".repeat(64),
            bytes: 0,
            state: "complete",
            read_at: None,
            duration_ms: None,
            reason_code: None,
        }
    }

    fn answer(code: &str, record_may_exist: bool) -> Value {
        let outcome = unfinished_outcome(
            json!({}),
            "bridge-x",
            code,
            record_may_exist,
            evidence(),
            "guid",
        );
        assert!(unfinished(&outcome.payload));
        outcome.payload["result"]["retire"].clone()
    }

    #[test]
    fn a_read_that_did_not_finish_says_the_file_is_gone_and_the_call_may_be_repeated() {
        let retire = answer("window_not_corroborated", false);
        assert_eq!(retire["state"], "not_retired");
        assert_eq!(retire["reason"], "window_not_corroborated");
        assert_eq!(retire["file_removed"], true);
        let step = retire["next_step"].as_str().unwrap();
        assert!(
            step.contains("a new post_import call for it is refused"),
            "{step}"
        );
        assert!(
            step.contains("Call verify_import on it again with if_absent set to retire"),
            "{step}"
        );
    }

    #[test]
    fn a_record_written_meanwhile_does_not_claim_the_batch_cannot_be_sent() {
        let retire = answer("import_verification_conflict_retry", false);
        assert_eq!(retire["state"], "not_retired");
        assert_eq!(retire["reason"], "import_verification_conflict_retry");
        let step = retire["next_step"].as_str().unwrap();
        assert_eq!(
            step,
            "Another record of this batch was written meanwhile, so nothing was retired. Call verify_import on it again."
        );
        assert!(!step.contains("no longer be sent"), "{step}");
    }

    #[test]
    fn a_refusal_a_repeat_cannot_change_says_so() {
        for code in [
            "voucher_window_book_too_large",
            "voucher_window_too_many_reads",
            "voucher_window_part_over_budget",
            "verification_window_day_not_readable",
            "response_size_limit_exceeded",
            "verification_mode_unqualified",
            "window_part_boundary_unsupported_in_education",
        ] {
            let retire = answer(code, false);
            assert_eq!(retire["reason"], code);
            let step = retire["next_step"].as_str().unwrap();
            assert!(
                step.contains("a repeat will not help until"),
                "{code}: {step}"
            );
            assert!(
                !step.contains("Call verify_import on it again with if_absent"),
                "{step}"
            );
            // a batch stranded by such a refusal is told so, with no way out named
            // that does not exist
            assert!(
                step.contains("no step inside ComplyEaze Bridge releases the batch yet"),
                "{code}: {step}"
            );
        }
        // a transient refusal, and two that cannot arrive after the delete, do
        // not claim that a repeat is useless
        for code in [
            "tally_endpoint_busy",
            "import_post_window_not_bounded",
            "verification_too_large_to_report",
        ] {
            let retire = answer(code, false);
            let step = retire["next_step"].as_str().unwrap();
            assert!(!step.contains("will not help"), "{code}: {step}");
        }
    }

    /// A refusal of the plan arrives before the file is removed: the answer
    /// says the batch is unchanged, serves no page, and says a repeat is
    /// useless only for the refusals a repeat cannot clear.
    #[test]
    fn a_refusal_by_the_plan_says_the_batch_is_unchanged() {
        for code in [
            "voucher_window_book_too_large",
            "voucher_window_too_many_reads",
            "voucher_window_part_over_budget",
            "voucher_window_volume_unestimated",
            "window_part_boundary_unsupported_in_education",
        ] {
            assert!(refused_by_the_plan(code), "{code}");
            let outcome = planned_refusal_outcome(json!({}), "bridge-x", code, evidence(), "guid");
            assert!(unfinished(&outcome.payload), "{code}");
            let retire = &outcome.payload["result"]["retire"];
            assert_eq!(retire["state"], "not_retired", "{code}");
            assert_eq!(retire["reason"], code);
            assert_eq!(retire["file_removed"], false, "{code}");
            assert_eq!(retire["verification_recorded"], false, "{code}");
            let step = retire["next_step"].as_str().unwrap();
            assert!(
                step.contains("before anything was changed"),
                "{code}: {step}"
            );
            assert!(step.contains("still in place"), "{code}: {step}");
            assert_eq!(
                step.contains("repeat will not help until"),
                repeat_cannot_help(code),
                "{code}: {step}"
            );
        }
        // a census that was not served may be served on the next call
        assert!(!repeat_cannot_help("voucher_window_volume_unestimated"));
        // neither a transient refusal nor a refusal of the read itself is the plan's
        for code in ["tally_endpoint_busy", "response_size_limit_exceeded"] {
            assert!(!refused_by_the_plan(code), "{code}");
        }
    }

    #[test]
    fn a_record_that_may_have_been_written_is_not_established() {
        let retire = answer("proof_publication_recovery_required", true);
        assert_eq!(retire["state"], "not_established");
        assert_eq!(retire["reason"], "proof_publication_recovery_required");
        let step = retire["next_step"].as_str().unwrap();
        assert!(
            step.contains("could not confirm that the record of it was written"),
            "{step}"
        );
        // the call it used to advise is refused while the marker stands
        assert!(step.contains("verify_import is refused"), "{step}");
        assert!(!step.contains("without if_absent"), "{step}");
        assert!(!step.contains("nothing was retired"), "{step}");
        assert!(!step.contains("no longer be sent"), "{step}");
    }
}
