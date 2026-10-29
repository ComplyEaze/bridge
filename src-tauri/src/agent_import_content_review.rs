//! A person's record that they checked in Tally a native post matched by
//! content only (#1039).
//!
//! A native post whose span binding was refused, or whose answer from Tally
//! was never recorded, is final in that state: no later check binds its
//! vouchers, so the batch reads `reconciliation_required` for good. After a
//! person has checked its vouchers in Tally, this records that they did,
//! beside that verdict. It changes nothing in Tally, changes no verdict and no
//! status, binds no voucher, and makes no claim about which Tally vouchers the
//! post created. It lifts no invoice stop (the stop release does that) and
//! releases no row (#876), and neither the transfer facts nor the invoice
//! number control reads it. Its one effect is where the batch is counted:
//! `verify_import` reports the review, and `local_data_report` counts the
//! batch in its own bucket. It is written only after the native review
//! dialog, which the model cannot answer.
//!
//! It is admitted only while the journal holds the content-only state, and the
//! fresh read for the dialog shows it: never for a binding refused because
//! another batch's voucher was bound in this post's span
//! (`span_identity_reused`), a book rolled back after the post, a binding not
//! yet settled, or a batch that reads `posted_verified`. A later
//! `posted_verified` of the batch, a later answer with an outcome, or any
//! rollback cover voids it (`ledger::content_review`).
use super::*;
use crate::tally::approved_import::{ReviewAcknowledged, ReviewKind, VoucherCount};

/// The refusal for a batch whose state is not content-only and final.
const NOT_CONTENT_ONLY: &str = "ack_content_review_not_content_only";

/// The content-only state the journal holds for a batch, if any: its native
/// post recorded a pre-POST mark, it is not verified, and either its span
/// binding was refused by a code a review can cover or no outcome of Tally's
/// answer was recorded and no binding decided.
pub(super) fn journaled_state(snapshot: &ledger::BatchSnapshot) -> Option<ledger::ReviewedBinding> {
    // Only a native dispatch intent records a pre-POST mark.
    if snapshot.pre_post_voucher_mark.is_none() || snapshot.batch.status == "posted_verified" {
        return None;
    }
    let answered = snapshot
        .response
        .as_ref()
        .is_some_and(|response| response.outcome.is_some());
    match &snapshot.span_verdict {
        Some(ledger::PostSpanVerdict::Refused(code)) if code != ledger::IDENTITY_REUSED => {
            Some(ledger::ReviewedBinding::Refused)
        }
        None if !answered => Some(ledger::ReviewedBinding::ResponseNotRecorded),
        _ => None,
    }
}

/// The content-only state a verification's `result` shows, if any: the same
/// two states, as `post_span_binding` reports them, in a result that does not
/// read `posted_verified`. A book rolled back after the post reports
/// `book_rolled_back` and is never one.
pub(super) fn shown_state(result: &Value) -> Option<ledger::ReviewedBinding> {
    if result["verification_status"] == "posted_verified" {
        return None;
    }
    let binding = &result["post_span_binding"];
    match (binding["state"].as_str(), binding["code"].as_str()) {
        (Some("refused"), Some(code)) if code != ledger::IDENTITY_REUSED => {
            Some(ledger::ReviewedBinding::Refused)
        }
        (Some("not_bound"), Some("binding_response_not_recorded")) => {
            Some(ledger::ReviewedBinding::ResponseNotRecorded)
        }
        _ => None,
    }
}

/// The words of the dialog: what the record does and does not do first, then
/// the batch, why it is matched by content only, and what the read just now
/// found. Every line is at most 100 characters, as the review box requires.
pub(super) fn review_preview(
    line: &ImportLedgerLine,
    reviewed: ledger::ReviewedBinding,
    result: &Value,
) -> String {
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
    let why = match reviewed {
        ledger::ReviewedBinding::Refused => format!(
            "its match of the post to the Tally vouchers it created was refused ({})",
            result["post_span_binding"]["code"]
                .as_str()
                .unwrap_or("refused")
        ),
        ledger::ReviewedBinding::ResponseNotRecorded => {
            "it has no readable answer from Tally to the post".to_string()
        }
    };
    let matched = result["counts"]["matching_content_observed"]
        .as_u64()
        .unwrap_or(0);
    let (noun, these) = if count == 1 {
        ("voucher", "this voucher")
    } else {
        ("vouchers", "these vouchers")
    };
    // A batch whose answer was lost may never have reached Tally: its intent
    // is recorded before the send.
    let (sent, of_them) = match reviewed {
        ledger::ReviewedBinding::Refused => ("Sent to Tally", "sent"),
        ledger::ReviewedBinding::ResponseNotRecorded => (
            "Attempted to post, no answer from Tally recorded",
            "in the batch",
        ),
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
    [
        wrap(format!(
            "THIS RECORDS ONLY THAT YOU CHECKED {} IN TALLY.",
            these.to_uppercase()
        )),
        wrap(
            "It changes nothing in Tally, does not mark the batch posted, and does not let its \
             rows be posted again."
                .into(),
        ),
        String::new(),
        wrap(format!("Company: {}", stop::shown(company))),
        wrap(format!("Batch: {}", stop::shown(&line.batch_id))),
        wrap(format!(
            "{sent}: {count} {noun} ({}), {}.",
            stop::shown(&types),
            stop::shown(&dated)
        )),
        wrap(format!(
            "ComplyEaze Bridge cannot tell which Tally vouchers this post created: {why}."
        )),
        wrap(format!(
            "The read just now found, for {matched} of the {count} {noun} {of_them}, a voucher in \
             the book with the same date, voucher type and ledger entries."
        )),
        String::new(),
        wrap(format!(
            "Confirm only if you have checked {these} in Tally yourself."
        )),
    ]
    .join("\n")
}

/// What `verify_import` reports for a standing review, beside the verdict.
pub(super) fn report(reviewed: ledger::ReviewedBinding) -> Value {
    json!({
        "state": "recorded",
        "reviewed_binding": reviewed.code(),
        "next_step": NEXT_STEP,
    })
}

const NEXT_STEP: &str = "A person recorded that they checked this batch's vouchers in Tally. \
    ComplyEaze Bridge still cannot tell which Tally vouchers this post created: the batch is not \
    marked posted and stays open, and its rows are not posted again.";

impl Server {
    /// The tool `acknowledge_post_review` with `doubt` set to `content_only`.
    pub(in crate::agent) async fn review_content_only(
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
        let Some(reviewed) = journaled_state(&snapshot) else {
            return Err(NOT_CONTENT_ONLY.to_string().into());
        };
        if self.import_content_review(&line.batch_id)?.is_some() {
            return Err("ack_already_recorded".to_string().into());
        }

        // The read the dialog shows. It must show the same content-only state:
        // a book rolled back, or a batch that reads verified, is refused.
        let outcome = self.verify_for_review(args, &mut None).await?;
        let evidence = outcome.evidence.clone();
        let fail =
            |code: &str| ToolFailure::from(code.to_string()).with_prior_evidence(evidence.clone());
        let result = &outcome.payload["result"];
        if shown_state(result) != Some(reviewed) {
            return Err(fail(NOT_CONTENT_ONLY));
        }
        let held = self
            .latest_import_snapshot(batch_id)
            .map_err(|code| fail(&code))?
            .ok_or_else(|| fail("import_batch_not_found"))?;
        let preview = review_preview(&line, reviewed, result);
        let count =
            VoucherCount::new(line.vouchers.len()).ok_or_else(|| fail("ack_batch_not_posted"))?;
        let _acknowledged = ReviewAcknowledged::confirm(ReviewKind::Review, count, &preview)
            .await
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;

        // Recorded under the exclusive lock, and only if nothing of the batch
        // was recorded since the read the person was shown, and the journal
        // would admit the record and hold it standing: the same content-only
        // state, no review standing, not verified, and no rollback covering
        // the batch (which another batch's check can record).
        let _lock = self.lock_import_admission()?;
        let current = self
            .import_snapshot_while_admitted(Some(&line.batch_id))
            .map_err(|code| fail(&code))?
            .ok_or_else(|| fail("import_batch_not_found"))?;
        let record = ledger::StatusRecord::content_review(&line, reviewed);
        let admitted = match self
            .import_journal_while_admitted()
            .map_err(|code| fail(&code))?
        {
            Some(reader) => ledger::content_review_after(reader, &record).ok().flatten(),
            None => None,
        };
        if current.generation != held.generation || admitted != Some(reviewed) {
            return Err(fail("ack_changed_while_reviewing"));
        }
        self.append_import_record_while_admitted(&record)
            .map_err(|code| fail(&code))?;
        Ok(ToolOutcome {
            payload: json!({"result": {
                "batch_id": line.batch_id,
                "state": "content_reviewed",
                "reviewed_binding": reviewed.code(),
                "verification_status": result["verification_status"],
                "next_step": NEXT_STEP,
            }}),
            evidence,
            company_guid: Some(line.company_guid.clone()),
            truncated: false,
        })
    }

    /// The batch's standing content review, under a shared lock.
    pub(super) fn import_content_review(
        &self,
        batch_id: &str,
    ) -> Result<Option<ledger::ReviewedBinding>, String> {
        let _lock = self.lock_import_admission_shared()?;
        self.import_content_review_while_admitted(batch_id)
    }

    fn import_content_review_while_admitted(
        &self,
        batch_id: &str,
    ) -> Result<Option<ledger::ReviewedBinding>, String> {
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::content_review(reader, batch_id),
            None => Ok(None),
        }
    }
}
