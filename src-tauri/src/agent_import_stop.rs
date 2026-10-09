//! A person's release of the Sales stop (ADR 0004, slice 4).
//!
//! A Sales invoice that was sent to Tally and is not verified posted stops
//! every further invoice of its company (`ledger::invoice_stops`). The stop
//! lifts by itself when a later verification reads that batch `posted_verified`.
//! Otherwise only a person lifts it, and this is the one way: after they have
//! checked the invoice in Tally, they answer a native dialog the model cannot
//! answer, and a `stop_release` record is appended to the import journal,
//! bound to the batch and its sha256. A later `posted_verified` of that batch
//! voids the release, so a divergence found after it stops the company again.
//!
//! It needs no voucher row: a declined or deleted invoice reads back nothing,
//! and that is a release. It tries a fresh readback so the dialog can say what
//! ComplyEaze Bridge sees, and what it saw is recorded: an invoice it found
//! stays the control for the next invoice number read, and one it did not find
//! no longer counts as sent. A read that fails for a cause waiting cures is waited
//! for; a failure no wait cures is shown and recorded as the invoice possibly
//! being there, so a stop is never unreleasable by such a failure.
//!
//! A batch released as found is still the number control, and the control read
//! cannot find an invoice that is gone: such a batch can be released once more,
//! and only when a fresh read now shows its invoice gone (`InvoiceHold`). That
//! second release is the way out of `invoice_number_control_missing`, and it
//! never relies on the number read it is meant to check.
use super::*;
use crate::tally::approved_import::{ReviewAcknowledged, VoucherCount};
use ledger::InvoiceHold;

/// What the read made for the dialog saw of the batch's invoice.
pub(super) enum Seen {
    /// At least one voucher of the batch's invoice was found in Tally, however
    /// it reads (verified, divergent, or cancelled or optional).
    Found,
    /// The read ran and found none.
    NotFound,
    /// The read could not be made, by a failure that no wait will cure (the
    /// batch's endpoint or company no longer matches what was recorded, for
    /// one). Recorded as found, so the number control stays strict: the
    /// invoice may be in the book.
    Unreadable(String),
}

impl Seen {
    /// Absent only when every row of the batch reads a state that says the
    /// voucher is not in the book for the dates it was posted: `not_found`,
    /// `tally_reported_not_created` (Tally's own answer said it created none),
    /// and the two states a verification gives a voucher it knows was posted
    /// and the window does not hold, `bound_not_in_window` (found by its GUID
    /// at the post, so deleted or re-dated since) and `book_rolled_back` (the
    /// books are older than the post). A deleted native post can never read
    /// `not_found`, so reading those two as found would keep a vanished
    /// invoice as the number control for good: the control read cannot find
    /// it, and nothing could release it. A re-dated invoice is still seen by
    /// the number read over its financial year, which is what the control
    /// guards. Every other state, and a result with no rows or with
    /// duplicates, reads as found: `sent_not_attributed` (matched by content
    /// alone, so an edited voucher looks the same), `matching_content_observed`,
    /// `not_attributable`, `duplicate_fingerprint` and
    /// `cancelled_with_effective_copy` all say a voucher may be there.
    pub(super) fn of(result: &Value) -> Self {
        let Some(rows) = result["vouchers"]
            .as_array()
            .filter(|rows| !rows.is_empty())
        else {
            return Self::Found;
        };
        let duplicates = result["duplicates"]
            .as_array()
            .is_some_and(|duplicates| !duplicates.is_empty());
        let absent = |row: &Value| {
            matches!(
                row["status"].as_str(),
                Some(
                    "not_found"
                        | "tally_reported_not_created"
                        | "bound_not_in_window"
                        | "book_rolled_back"
                )
            )
        };
        if duplicates || !rows.iter().all(absent) {
            Self::Found
        } else {
            Self::NotFound
        }
    }

    pub(super) fn found(&self) -> bool {
        !matches!(self, Self::NotFound)
    }

    fn words(&self) -> String {
        match self {
            Self::Found => "found in Tally".into(),
            Self::NotFound => {
                "not in Tally's book for the dates it was posted (deleted or re-dated there, \
                 the books restored from a backup, or never created)"
                    .into()
            }
            Self::Unreadable(code) => format!(
                "not established: Tally could not be read ({code}), so it is recorded as possibly in the book"
            ),
        }
    }
}

/// Whether a failed read is one no wait cures, because it fails before any
/// Tally read, or independently of what Tally holds: the batch was sent to
/// another endpoint than the one now set, the company now open is not the one
/// the batch recorded, the host setting is unusable, the product, release or
/// licence is not one a vouchers-absent verdict is qualified for, or the
/// window is too large to report. Refusing the release for these would leave the company stopped for
/// good. Every other failure (Tally not answering, the company not open, a
/// cancelled or cut-short read) is cured by waiting or by opening Tally, so the
/// release is refused and made once the read runs; a code not listed here
/// defaults to that refusal.
pub(super) fn failure_waiting_cannot_cure(failure: &ToolFailure) -> bool {
    matches!(
        failure.code.as_str(),
        "import_post_endpoint_mismatch"
            | "company_identity_mismatch"
            | "host_setting_invalid"
            | "verification_mode_unqualified"
            | "verification_too_large_to_report"
    )
}

/// The words of the dialog. They say what is being released and what it
/// allows, before anything about the batch, because the title of the shared
/// review dialog names only a review.
/// `text` as the dialog shows it: a character that could reorder or hide what
/// the person reads (the same ones the review dialog refuses) shows as `?`.
/// The text comes from the journal and Tally, and a release must not become
/// impossible for a name that carries one.
fn shown(text: &str) -> String {
    text.chars()
        .map(|character| {
            let one = character.to_string();
            if post::has_unsafe_review_layout_character(&one)
                || post::has_unreviewable_format_character(&one)
            {
                '?'
            } else {
                character
            }
        })
        .collect()
}

pub(super) fn release_preview(
    line: &ImportLedgerLine,
    company_name: &str,
    seen: &Seen,
    hold: InvoiceHold,
) -> String {
    let (company_name, batch_id) = (shown(company_name), shown(&line.batch_id));
    let invoices = line
        .vouchers
        .iter()
        .filter(|voucher| voucher.voucher_type.is_invoice())
        .map(|voucher| {
            format!(
                "  {} dated {}",
                shown(voucher.voucher_number.as_deref().unwrap_or("(no number)")),
                shown(&voucher.date)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let (headline, what_it_allows) = match hold {
        InvoiceHold::Stopping => (
            "THIS RELEASES THE INVOICE STOP FOR THE COMPANY.",
            "ComplyEaze Bridge will then build and post invoices for this company again.",
        ),
        InvoiceHold::ReleasedAsFound => (
            "THIS STOPS USING AN INVOICE TO TEST THE INVOICE-NUMBER CHECK.",
            "ComplyEaze Bridge will no longer use this invoice to test that its check for \
             used invoice numbers works on this book. A new invoice's number is still checked \
             against Tally's book; if no other invoice remains as that test, the check stands \
             alone, as for a company's first invoice, and if it misses a used number the \
             duplicate would show only after the post.",
        ),
    };
    format!(
        "{headline}\n\n\
         Company: {company_name}\n\
         ComplyEaze Bridge sent this invoice to Tally and cannot confirm it as posted:\n{invoices}\n\
         Batch: {}\n\
         What ComplyEaze Bridge saw just now: {}.\n\n\
         Click the button only if you have checked this invoice in Tally yourself. \
         {what_it_allows} \
         This changes nothing in Tally.",
        batch_id,
        seen.words()
    )
}

impl Server {
    /// The tool `acknowledge_post_review` with `doubt` set to `invoice_stop`.
    pub(in crate::agent) async fn release_invoice_stop(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let batch_id = required_string(args, "batch_id")?;
        let ledger::BatchSnapshot {
            batch: line,
            dispatched,
            ..
        } = self
            .latest_import_snapshot(batch_id)?
            .ok_or_else(|| "import_batch_not_found".to_string())?;
        // Refused before any read or dialog: nothing of this batch was sent,
        // or it holds no invoice, so it stops nothing.
        if !dispatched || !holds_an_invoice(&line) {
            return Err("ack_stop_batch_not_sent".to_string().into());
        }
        // The company the caller names is the batch's own: a mismatch is the
        // caller's to correct, never a way onto the path taken for a read that
        // cannot be made.
        if !line
            .company_guid
            .eq_ignore_ascii_case(required_string(args, "company_guid")?)
        {
            return Err("import_batch_company_mismatch".to_string().into());
        }
        self.hold_of(&line)?
            .ok_or_else(|| "ack_stop_not_held".to_string())?;

        // What Tally holds now, to show and to record. A read that fails for a
        // cause waiting cures (Tally not answering, the company not open) is
        // waited for: a release made blind could drop a possibly posted invoice
        // from the number control, and such a Tally posts nothing anyway. A
        // failure waiting cannot cure does not refuse the release (the stop
        // would never lift); it is shown and recorded as the invoice possibly
        // being in the book.
        let mut rows = None;
        let (seen, evidence) = match self.verify_for_review(args, &mut rows).await {
            Ok(outcome) => (Seen::of(&outcome.payload["result"]), outcome.evidence),
            Err(failure) if !failure_waiting_cannot_cure(&failure) => {
                let mut refused = ToolFailure::from("ack_stop_tally_unreadable".to_string());
                refused.evidence = failure.evidence;
                return Err(refused);
            }
            Err(failure) => (
                Seen::Unreadable(failure.code.clone()),
                failure.evidence.map_or_else(
                    || {
                        super::super::evidence_from_runtime_read(
                            crate::tally::runtime::RuntimeReadEvidence::empty(),
                        )
                    },
                    |evidence| *evidence,
                ),
            ),
        };
        // The read itself may have verified the batch: the stop is then gone,
        // and nothing is asked.
        let Some((hold, held)) = self.hold_of(&line)? else {
            return Ok(self.stop_outcome(&line, "not_held", None, evidence));
        };
        // A release that was made as found is dropped only by a read that now
        // shows the invoice gone: any other answer leaves it as it stands, and
        // nothing is asked or written.
        if hold == InvoiceHold::ReleasedAsFound && seen.found() {
            return Err(ToolFailure::from("ack_stop_release_stands".to_string())
                .with_prior_evidence(evidence));
        }
        let company_name = held
            .company
            .as_ref()
            .map(|company| company.name.clone())
            .unwrap_or_default();
        let preview = release_preview(&held, &company_name, &seen, hold);
        let count = VoucherCount::new(1).ok_or_else(|| "ack_stop_batch_not_sent".to_string())?;
        let _acknowledged = ReviewAcknowledged::confirm(count, &preview)
            .await
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;

        // Recorded under the exclusive lock, and only if the batch still
        // stands as the person was shown.
        let _lock = self.lock_import_admission()?;
        let current = self
            .import_snapshot_while_admitted(Some(&line.batch_id))?
            .ok_or_else(|| "import_batch_not_found".to_string())?;
        if current.batch.sha256 != line.sha256
            || self
                .import_invoice_holds_while_admitted(&line.company_guid)?
                .get(&line.batch_id)
                != Some(&hold)
        {
            return Err(
                ToolFailure::from("ack_stop_changed_while_reviewing".to_string())
                    .with_prior_evidence(evidence),
            );
        }
        self.append_import_record_while_admitted(&ledger::StatusRecord::stop_release(
            &line,
            seen.found(),
        ))?;
        if self
            .import_invoice_holds_while_admitted(&line.company_guid)?
            .get(&line.batch_id)
            == Some(&hold)
        {
            return Err(ToolFailure::from("ack_stop_not_recorded".to_string())
                .with_prior_evidence(evidence));
        }
        let state = match hold {
            InvoiceHold::Stopping => "stop_released",
            InvoiceHold::ReleasedAsFound => "control_dropped",
        };
        Ok(self.stop_outcome(&line, state, Some(seen.found()), evidence))
    }

    /// How the batch stands now, with the batch as the journal holds it: it
    /// stops its company, or it was released as found and is still the number
    /// control. None for a batch that reads verified, was released as not
    /// found, or was never sent.
    fn hold_of(
        &self,
        line: &ImportLedgerLine,
    ) -> Result<Option<(InvoiceHold, ImportLedgerLine)>, String> {
        let _lock = self.lock_import_admission_shared()?;
        let Some(hold) = self
            .import_invoice_holds_while_admitted(&line.company_guid)?
            .get(&line.batch_id)
            .copied()
        else {
            return Ok(None);
        };
        Ok(self
            .import_snapshot_while_admitted(Some(&line.batch_id))?
            .map(|snapshot| (hold, snapshot.batch)))
    }

    fn stop_outcome(
        &self,
        line: &ImportLedgerLine,
        state: &str,
        invoice_found: Option<bool>,
        evidence: Evidence,
    ) -> ToolOutcome {
        let next_step = match state {
            "stop_released" => {
                "The stop is released. Invoices for this company can be built and posted again."
            }
            "control_dropped" => {
                "This invoice no longer checks new invoice numbers. Build the invoice again."
            }
            _ => "This batch no longer stands: nothing was released.",
        };
        ToolOutcome {
            payload: json!({"result": {
                "batch_id": line.batch_id,
                "state": state,
                "invoice_found_at_release": invoice_found,
                "next_step": next_step,
            }}),
            evidence,
            company_guid: Some(line.company_guid.clone()),
            truncated: false,
        }
    }
}
