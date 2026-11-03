//! A person's release of the rows of a native post whose book was rolled back
//! (#1038).
//!
//! A backup restored over a company, or another copy put in its place, can
//! remove vouchers ComplyEaze Bridge posted. `verify_import` then reports the
//! batch `book_rolled_back`, and a rollback record covers it for good (#1050).
//! The journal still holds its rows as posted, so a rebuild of them is refused
//! (#876). After a person has checked in Tally that none of the batch's
//! vouchers is in the book, including any keyed again with other figures,
//! this records that they released its rows. It changes nothing in Tally,
//! changes no verdict and no status, removes no record and sends nothing. Its
//! one effect is that the lookup of rows already posted
//! (`ledger::vouchers_already_posted`) no longer counts the batch, so those
//! rows can go through an ordinary fresh build and post, with every check an
//! ordinary batch meets. `local_data_report` counts it as released by a person
//! and `verify_import` reports the release beside `book_rolled_back`. It is
//! written only after the native row release dialog, which the model cannot
//! answer, and it is never undone or voided.
//!
//! The person is the only guard against a voucher keyed again with any
//! difference: the fresh post's book check matches by fingerprint and does
//! not see it. So the dialog lists every voucher, and a batch whose vouchers
//! cannot all be listed is never released. Only a batch of Journal, Payment
//! and Receipt vouchers is released: a Sales invoice or a Contra is also held
//! by readers a release does not reach.
use super::*;
use crate::tally::approved_import::{ReviewAcknowledged, ReviewKind, VoucherCount};

/// The refusal for a batch whose fresh read does not show its book rolled
/// back, or that no rollback record covers.
const ROLLED_BACK_NOT_ESTABLISHED: &str = "ack_rolled_back_not_established";

/// The refusal for a batch holding a voucher of a type a release does not
/// admit.
const TYPE_NOT_ADMITTED: &str = "ack_release_type_not_admitted";

/// The refusal for a batch whose vouchers the dialog cannot all list.
const VOUCHERS_NOT_LISTED: &str = "ack_release_vouchers_not_listed";

/// Whether a release admits a voucher of `voucher_type`. An allow-list: a
/// type added later is refused until it is named here. A Sales invoice is
/// also held by the invoice readers, and a Contra by the transfer view, which
/// a release does not reach.
fn type_admitted(voucher_type: &VoucherType) -> bool {
    matches!(
        voucher_type,
        VoucherType::Journal | VoucherType::Payment | VoucherType::Receipt
    )
}

/// Whether a verification's `result` shows the batch's book rolled back: its
/// binding reads `book_rolled_back`, and it does not read `posted_verified`.
pub(super) fn shown_rolled_back(result: &Value) -> bool {
    result["verification_status"] != "posted_verified"
        && result["post_span_binding"]["state"] == "book_rolled_back"
}

/// The date of a build time as the dialog shows it: its day, as Tally writes
/// one (`14-Sep-2026`), or the value as it is when it is not a time.
fn built_on(built_at: &str) -> String {
    let day = built_at.get(..10).unwrap_or(built_at).replace('-', "");
    post::shown_date(&day)
}

/// The words of the dialog: what the release does and does not do first, then
/// the batch, what the last verification read, every voucher, and what the
/// person confirms. `listed` is the batch's vouchers as
/// [`post::listed_voucher_lines`] gives them. Every line is at most 100
/// characters, as the box requires.
pub(super) fn release_preview(line: &ImportLedgerLine, listed: &[String]) -> String {
    let company = line
        .company
        .as_ref()
        .map(|company| company.name.as_str())
        .filter(|name| !name.is_empty())
        .unwrap_or(&line.company_guid);
    let count = line.vouchers.len();
    let (noun, confirm) = if count == 1 {
        (
            "voucher",
            "Confirm only if you have checked in Tally that this voucher is not in the book, \
             including keyed again with other figures.",
        )
    } else {
        (
            "vouchers",
            "Confirm only if you have checked in Tally that none of these vouchers is in the \
             book, including any keyed again with other figures.",
        )
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
        wrap("THIS RELEASES THIS BATCH'S ROWS, SO THAT THEY CAN BE BUILT AND POSTED AGAIN.".into()),
        wrap(
            "It changes nothing in Tally and removes no record. It cannot be undone: a new batch \
             of these rows can post them again."
                .into(),
        ),
        String::new(),
        wrap(format!("Company: {}", stop::shown(company))),
        wrap(format!("Batch: {}", stop::shown(&line.batch_id))),
        wrap(format!(
            "Built {} and sent to Tally by ComplyEaze Bridge: {count} {noun}.",
            stop::shown(&built_on(&line.built_at))
        )),
        wrap(
            "The last verification read the company's voucher mark below the mark this post left."
                .into(),
        ),
        String::new(),
    ];
    lines.extend(listed.iter().cloned());
    lines.push(String::new());
    lines.push(wrap(confirm.into()));
    lines.join("\n")
}

/// What `verify_import` reports for a standing release, beside the verdict.
pub(super) fn report() -> Value {
    json!({"state": "rows_released", "next_step": NEXT_STEP})
}

pub(super) const NEXT_STEP: &str =
    "A person released this batch's rows in ComplyEaze Bridge's own dialog, saying none of its \
    vouchers is in the book. The batch keeps its record and still reads book_rolled_back. Its rows \
    can be built again as a new batch, which is checked against the book and approved like any \
    other. Do not import this batch's own file through Tally's Import menu.";

impl Server {
    /// The tool `acknowledge_post_review` with `doubt` set to `rolled_back`.
    pub(in crate::agent) async fn release_rolled_back_rows(
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
        // Only a native post that recorded its pre-POST mark can be covered.
        if snapshot.pre_post_voucher_mark.is_none() {
            return Err(ROLLED_BACK_NOT_ESTABLISHED.to_string().into());
        }
        if !line
            .vouchers
            .iter()
            .all(|voucher| type_admitted(&voucher.voucher_type))
        {
            return Err(TYPE_NOT_ADMITTED.to_string().into());
        }
        let Some(listed) = post::listed_voucher_lines(&line)? else {
            return Err(VOUCHERS_NOT_LISTED.to_string().into());
        };
        if self.import_row_release(&line.batch_id)? {
            return Err("ack_already_recorded".to_string().into());
        }

        // The read the dialog shows. It must show the book rolled back, and a
        // rollback record must cover the batch once it is done (the read may
        // write that record itself).
        let outcome = self.verify_for_review(args, &mut None).await?;
        let evidence = outcome.evidence.clone();
        let fail =
            |code: &str| ToolFailure::from(code.to_string()).with_prior_evidence(evidence.clone());
        let result = &outcome.payload["result"];
        if !shown_rolled_back(result) {
            return Err(fail(ROLLED_BACK_NOT_ESTABLISHED));
        }
        let held = self
            .latest_import_snapshot(batch_id)
            .map_err(|code| fail(&code))?
            .ok_or_else(|| fail("import_batch_not_found"))?;
        let cover = self
            .rollback_scan(&held.batch.batch_id)
            .map_err(|code| fail(&code))?
            .covered_by
            .ok_or_else(|| fail(ROLLED_BACK_NOT_ESTABLISHED))?;
        let record = ledger::StatusRecord::row_release(&held.batch, &cover);
        let preview = release_preview(&held.batch, &listed);
        let count =
            VoucherCount::new(line.vouchers.len()).ok_or_else(|| fail("ack_batch_not_posted"))?;
        let _acknowledged = ReviewAcknowledged::confirm(ReviewKind::RowRelease, count, &preview)
            .await
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;

        // Recorded under the exclusive lock, and only if nothing of the batch
        // was recorded since the read the person was shown, and the journal
        // would admit the record: the same cover still standing for the
        // company and endpoint it names, no release standing, and the batch
        // not verified.
        let _lock = self.lock_import_admission()?;
        let current = self
            .import_snapshot_while_admitted(Some(&line.batch_id))
            .map_err(|code| fail(&code))?
            .ok_or_else(|| fail("import_batch_not_found"))?;
        let admitted = match self
            .import_journal_while_admitted()
            .map_err(|code| fail(&code))?
        {
            Some(reader) => ledger::row_release_after(reader, &record).unwrap_or(false),
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
                "state": "rows_released",
                "verification_status": result["verification_status"],
                "next_step": NEXT_STEP,
            }}),
            evidence,
            company_guid: Some(line.company_guid.clone()),
            truncated: false,
        })
    }

    /// Whether a release of the batch's rows stands, under a shared lock.
    pub(super) fn import_row_release(&self, batch_id: &str) -> Result<bool, String> {
        let _lock = self.lock_import_admission_shared()?;
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::row_release(reader, batch_id),
            None => Ok(false),
        }
    }
}
