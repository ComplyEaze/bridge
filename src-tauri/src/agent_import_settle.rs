//! A person's settlement of a native post Tally answered as creating none of
//! its vouchers (#1709).
//!
//! When Tally's own answer to a post reported none of its vouchers created,
//! the batch stays `reconciliation_required` for good: no later check binds
//! it, and its rows stay held, so nothing in ComplyEaze Bridge closes it.
//! Sending it again is not safe: Tally can still finish work its client gave
//! up on (#1713), so a voucher it did not report may still land. After a
//! person has said that each voucher is in the book or will not be posted,
//! this records that they settled the batch. It changes nothing in Tally,
//! changes no verdict and no status, binds no voucher and sends nothing. It
//! releases no row (#876), so the batch's rows are never built or posted
//! again, and it lifts no invoice stop (the stop release does that). Its one
//! effect is where the batch is counted: `local_data_report` counts it as
//! settled by a person and no longer in `not_settled`, and `verify_import`
//! reports the settlement. It is written only after the native settle dialog,
//! which the model cannot answer.
//!
//! It is admitted only while the journal holds that answer and the fresh read
//! for the dialog shows the batch neither verified, bound, still binding nor
//! rolled back. A later `posted_verified` of the batch, a later answer with an
//! outcome, a binding or any rollback cover voids it
//! (`ledger::person_settlement`).
use super::*;
use crate::tally::approved_import::{ReviewAcknowledged, ReviewKind, VoucherCount};

/// The refusal for a batch whose post Tally is not on record as answering
/// that it created none of its vouchers.
const NOT_CREATED_NOT_ESTABLISHED: &str = "ack_not_created_not_established";

/// Whether the journal, and for a batch its post's own check record, hold
/// that Tally answered the post as creating none of its vouchers: the cause a
/// later verification reports as `ReportedNotCreatedWhenSent`. The native
/// post recorded a pre-POST mark, is neither verified nor bound, its saved
/// answer has the whole-rejection shape, and for a batch of two or more the
/// post itself recorded the company's voucher mark moving by what Tally
/// reported creating (`batch_step` `matched`).
pub(super) fn journaled_not_created(
    snapshot: &ledger::BatchSnapshot,
    step_matched: impl FnOnce() -> bool,
) -> bool {
    if snapshot.pre_post_voucher_mark.is_none()
        || snapshot.batch.status == "posted_verified"
        || matches!(
            snapshot.span_verdict,
            Some(ledger::PostSpanVerdict::Bound(_))
        )
    {
        return false;
    }
    let count = snapshot.batch.vouchers.len();
    let answer = snapshot
        .response
        .as_ref()
        .and_then(|response| response.outcome.as_ref())
        .map(|outcome| outcome.counters());
    let step_matched = count > 1 && step_matched();
    verification::later_unmatched_cause(answer, count, step_matched)
        == verification::UnmatchedCause::ReportedNotCreatedWhenSent
}

/// Whether a verification's `result` shows the batch still open: not
/// `posted_verified`, and its binding `refused` or `not_bound`. A book rolled
/// back after the post (`book_rolled_back`), a binding still `unsettled` and a
/// bound post are never settled.
pub(super) fn shown_open(result: &Value) -> bool {
    result["verification_status"] != "posted_verified"
        && matches!(
            result["post_span_binding"]["state"].as_str(),
            Some("refused" | "not_bound")
        )
}

/// The words of the dialog: what the settlement does and does not do first,
/// then the batch, what Tally answered when it was sent and what the read just
/// now found. Every line is at most 100 characters, as the box requires.
pub(super) fn settle_preview(line: &ImportLedgerLine, result: &Value) -> String {
    let company = line
        .company
        .as_ref()
        .map(|company| company.name.as_str())
        .filter(|name| !name.is_empty())
        .unwrap_or(&line.company_guid);
    let count = line.vouchers.len();
    let mut types: Vec<(String, usize)> = Vec::new();
    for voucher in &line.vouchers {
        let name = voucher.voucher_type.as_str().to_string();
        match types.iter_mut().find(|(known, _)| *known == name) {
            Some((_, seen)) => *seen += 1,
            None => types.push((name, 1)),
        }
    }
    let types = types
        .iter()
        .map(|(name, seen)| format!("{seen} {name}"))
        .collect::<Vec<_>>()
        .join(", ");
    let dates = line.vouchers.iter().map(|voucher| voucher.date.as_str());
    let (first, last) = (dates.clone().min(), dates.max());
    let dated = match (first, last) {
        (Some(first), Some(last)) if first != last => format!("dated {first} to {last}"),
        (Some(first), _) => format!("dated {first}"),
        _ => String::new(),
    };
    // A match that is cancelled or optional is not in the accounts, so the
    // line never counts it as held, as the binding's summary does not.
    let matched = result["vouchers"].as_array().map_or(0, |vouchers| {
        vouchers
            .iter()
            .filter(|voucher| {
                voucher["status"] == "matching_content_observed"
                    && voucher["accounting_effective"] == true
            })
            .count()
    });
    let (noun, these, each) = if count == 1 {
        ("voucher", "this voucher", "the voucher")
    } else {
        ("vouchers", "these vouchers", "each voucher")
    };
    // Broken at spaces, then any word longer than a line broken where it
    // reaches the limit, so a name without spaces cannot make the box refuse.
    let wrap = |text: String| {
        crate::tally::approved_import::wrapped(&text, 100)
            .lines()
            .flat_map(|line| {
                line.chars()
                    .collect::<Vec<_>>()
                    .chunks(100)
                    .map(|part| part.iter().collect::<String>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let mut lines = vec![
        wrap(format!(
            "THIS CLOSES THE BATCH. COMPLYEAZE BRIDGE NEVER SENDS {} AGAIN.",
            these.to_uppercase()
        )),
        wrap(
            "It changes nothing in Tally and does not mark the batch posted. Its rows stay held: \
             they cannot be built or posted again."
                .into(),
        ),
        String::new(),
        wrap(format!("Company: {}", stop::shown(company))),
        wrap(format!("Batch: {}", stop::shown(&line.batch_id))),
        wrap(format!(
            "Sent to Tally: {count} {noun} ({}), {}.",
            stop::shown(&types),
            stop::shown(&dated)
        )),
        wrap(
            if count == 1 {
                "When it was sent, Tally answered that it did not create it."
            } else {
                "When they were sent, Tally answered that it created none of them."
            }
            .into(),
        ),
        wrap(format!(
            "The read just now found, for {matched} of the {count} {noun} sent, a voucher in the \
             book with the same date, voucher type and ledger entries."
        )),
    ];
    if holds_an_invoice(line) {
        lines.push(wrap(
            "Its invoice stop stays until you release it on its own.".into(),
        ));
    }
    lines.push(String::new());
    lines.push(wrap(format!(
        "Confirm only if you have checked in Tally that {each} is in the book, or you have \
         decided it will not be posted."
    )));
    lines.join("\n")
}

/// What `verify_import` reports for a standing settlement, beside the verdict.
pub(super) fn report() -> Value {
    json!({"state": "settled", "next_step": NEXT_STEP})
}

/// Whether a voucher's next step still asks for the settlement it now has:
/// the lines of `tally_reported_not_created` and of a voucher a later read
/// cannot find after Tally reported it not created end on that call.
pub(super) fn asks_for_settlement(next_step: Option<&str>) -> bool {
    next_step
        .is_some_and(|line| line.contains("call acknowledge_post_review with doubt not_created"))
}

/// In `verify_import`'s result for a batch a person settled, each voucher line
/// that still asks for the settlement gets the settled next step; every other
/// line is kept. A result with no settlement is left as it is.
pub(super) fn settle_voucher_next_steps(result: &mut Value) {
    if !result["person_settlement"].is_object() {
        return;
    }
    if let Some(rows) = result["vouchers"].as_array_mut() {
        for row in rows {
            if asks_for_settlement(row["next_step"].as_str()) {
                row["next_step"] = json!(NEXT_STEP);
            }
        }
    }
}

/// The binding summary of a settled batch, in place of the open one.
pub(super) const SUMMARY: &str = "A person settled this batch after Tally reported that its post \
    created none of its vouchers: ComplyEaze Bridge never sends them again.";

pub(super) const NEXT_STEP: &str =
    "A person settled this batch in ComplyEaze Bridge's own dialog, saying \
    each voucher is in the book or will not be posted. It is closed and counted as settled, but \
    not marked posted, and its rows are never built or posted again. If a voucher is wanted in the \
    book and is not there, it can only be entered in Tally; do not import its file through \
    Tally's Import menu.";

impl Server {
    /// The tool `acknowledge_post_review` with `doubt` set to `not_created`.
    pub(in crate::agent) async fn settle_not_created(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let batch_id = required_string(args, "batch_id")?;
        let snapshot = self
            .latest_import_snapshot(batch_id)?
            .ok_or_else(|| "import_batch_not_found".to_string())?;
        let line = snapshot.batch.clone();
        // Refused before any read or dialog: nothing was sent.
        if !snapshot.dispatched || line.vouchers.is_empty() {
            return Err("ack_batch_not_posted".to_string().into());
        }
        if !line
            .company_guid
            .eq_ignore_ascii_case(required_string(args, "company_guid")?)
        {
            return Err("import_batch_company_mismatch".to_string().into());
        }
        let step_matched = || {
            self.imports_dir()
                .ok()
                .and_then(|imports| read_masters_check(&imports, &line.batch_id))
                .is_some_and(|check| check["batch_step"]["state"] == "matched")
        };
        if !journaled_not_created(&snapshot, step_matched) {
            return Err(NOT_CREATED_NOT_ESTABLISHED.to_string().into());
        }
        if self.import_person_settlement(&line.batch_id)? {
            return Err("ack_already_recorded".to_string().into());
        }

        // The read the dialog shows. It must show the batch still open: a
        // book rolled back, a binding not yet settled, or a batch that reads
        // verified is refused.
        let outcome = self.verify_for_review(args, &mut None).await?;
        let evidence = outcome.evidence.clone();
        let fail =
            |code: &str| ToolFailure::from(code.to_string()).with_prior_evidence(evidence.clone());
        let result = &outcome.payload["result"];
        if !shown_open(result) {
            return Err(fail(NOT_CREATED_NOT_ESTABLISHED));
        }
        let held = self
            .latest_import_snapshot(batch_id)
            .map_err(|code| fail(&code))?
            .ok_or_else(|| fail("import_batch_not_found"))?;
        let preview = settle_preview(&line, result);
        let count =
            VoucherCount::new(line.vouchers.len()).ok_or_else(|| fail("ack_batch_not_posted"))?;
        let _acknowledged = ReviewAcknowledged::confirm(ReviewKind::Settle, count, &preview)
            .await
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;

        // Recorded under the exclusive lock, and only if nothing of the batch
        // was recorded since the read the person was shown, and the journal
        // would admit the record and hold it standing: the same answer on
        // record, no settlement standing, not verified or bound, and no
        // rollback covering the batch (which another batch's check can
        // record).
        let _lock = self.lock_import_admission()?;
        let current = self
            .import_snapshot_while_admitted(Some(&line.batch_id))
            .map_err(|code| fail(&code))?
            .ok_or_else(|| fail("import_batch_not_found"))?;
        let record = ledger::StatusRecord::person_settlement(&line);
        let admitted = match self
            .import_journal_while_admitted()
            .map_err(|code| fail(&code))?
        {
            Some(reader) => ledger::person_settlement_after(reader, &record).unwrap_or(false),
            None => false,
        };
        if current.generation != held.generation || !admitted {
            return Err(fail("ack_changed_while_reviewing"));
        }
        self.append_import_record_while_admitted(&record)
            .map_err(|code| fail(&code))?;
        Ok(ToolOutcome {
            payload: json!({"result": {
                "batch_id": line.batch_id,
                "state": "settled_by_person",
                "verification_status": result["verification_status"],
                "next_step": NEXT_STEP,
            }}),
            evidence,
            company_guid: Some(line.company_guid.clone()),
            truncated: false,
        })
    }

    /// Whether the batch's settlement stands, under a shared lock.
    pub(super) fn import_person_settlement(&self, batch_id: &str) -> Result<bool, String> {
        let _lock = self.lock_import_admission_shared()?;
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::person_settlement(reader, batch_id),
            None => Ok(false),
        }
    }
}
