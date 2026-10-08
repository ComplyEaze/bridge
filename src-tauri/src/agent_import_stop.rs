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
//! no longer counts as sent. A Tally that is not answering is waited for; any
//! other failed read is shown and recorded as the invoice possibly being there,
//! so a stop is never unreleasable.
use super::*;
use crate::tally::approved_import::{ReviewAcknowledged, VoucherCount};

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
    pub(super) fn of(result: &Value) -> Self {
        let present = [
            "posted_verified",
            "posted_divergent",
            "posted_not_effective",
        ]
        .iter()
        .map(|state| counted(result, state))
        .sum::<u64>();
        if present > 0 {
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
            Self::NotFound => "not found in Tally".into(),
            Self::Unreadable(code) => format!(
                "not established: Tally could not be read ({code}), so it is recorded as possibly in the book"
            ),
        }
    }
}

/// Whether a failed read is one a person cures by waiting or by opening Tally:
/// the release is then refused, and made once Tally answers. Any other failure
/// is permanent for this batch, and refusing the release would leave the
/// company stopped for good.
fn tally_not_answering(failure: &ToolFailure) -> bool {
    failure.unanswered.is_some()
        || matches!(
            failure.code.as_str(),
            "tally_endpoint_busy" | "import_mode_probe_failed"
        )
}

fn counted(result: &Value, state: &str) -> u64 {
    result["counts"][state].as_u64().unwrap_or(0)
}

/// The words of the dialog. They say what is being released and what it
/// allows, before anything about the batch, because the title of the shared
/// review dialog names only a review.
fn release_preview(line: &ImportLedgerLine, company_name: &str, seen: &Seen) -> String {
    let invoices = line
        .vouchers
        .iter()
        .filter(|voucher| voucher.voucher_type.is_invoice())
        .map(|voucher| {
            format!(
                "  {} dated {}",
                voucher.voucher_number.as_deref().unwrap_or("(no number)"),
                voucher.date
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "THIS RELEASES THE INVOICE STOP FOR THE COMPANY.\n\n\
         Company: {company_name}\n\
         ComplyEaze Bridge sent this invoice to Tally and cannot confirm it as posted:\n{invoices}\n\
         Batch: {}\n\
         What ComplyEaze Bridge saw just now: {}.\n\n\
         Click the button only if you have checked this invoice in Tally yourself. \
         ComplyEaze Bridge will then build and post invoices for this company again. \
         This changes nothing in Tally.",
        line.batch_id,
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
        self.require_stop(&line)?;

        // What Tally holds now, to show and to record. A Tally that is not
        // answering is waited for: a release made blind could drop a possibly
        // posted invoice from the number control, and a Tally that is not
        // answering posts nothing anyway. A failure that waiting cannot cure
        // does not refuse the release (the stop would never lift); it is shown
        // and recorded as the invoice possibly being in the book.
        let mut rows = None;
        let (seen, evidence) = match self.verify_for_review(args, &mut rows).await {
            Ok(outcome) => (Seen::of(&outcome.payload["result"]), outcome.evidence),
            Err(failure) if tally_not_answering(&failure) => {
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
        let Some(held) = self.stop_held(&line)? else {
            return Ok(self.stop_outcome(&line, "not_held", None, evidence));
        };
        let company_name = held
            .company
            .as_ref()
            .map(|company| company.name.clone())
            .unwrap_or_default();
        let preview = release_preview(&held, &company_name, &seen);
        let count = VoucherCount::new(1).ok_or_else(|| "ack_stop_batch_not_sent".to_string())?;
        let _acknowledged = ReviewAcknowledged::confirm(count, &preview)
            .await
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;

        // Recorded under the exclusive lock, and only if the batch still
        // stops the company as the person was shown.
        let _lock = self.lock_import_admission()?;
        let current = self
            .import_snapshot_while_admitted(Some(&line.batch_id))?
            .ok_or_else(|| "import_batch_not_found".to_string())?;
        if current.batch.sha256 != line.sha256
            || !self
                .import_invoice_stops_while_admitted(&line.company_guid)?
                .contains(&line.batch_id)
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
            .import_invoice_stops_while_admitted(&line.company_guid)?
            .contains(&line.batch_id)
        {
            return Err(ToolFailure::from("ack_stop_not_recorded".to_string())
                .with_prior_evidence(evidence));
        }
        Ok(self.stop_outcome(&line, "stop_released", Some(seen.found()), evidence))
    }

    /// Refuses a batch that does not stop its company now: it reads verified,
    /// or a person already released it.
    fn require_stop(&self, line: &ImportLedgerLine) -> Result<(), ToolFailure> {
        self.stop_held(line)?
            .map(|_| ())
            .ok_or_else(|| "ack_stop_not_held".to_string().into())
    }

    /// The batch as the journal holds it now, when it stops its company.
    fn stop_held(&self, line: &ImportLedgerLine) -> Result<Option<ImportLedgerLine>, String> {
        let _lock = self.lock_import_admission_shared()?;
        if !self
            .import_invoice_stops_while_admitted(&line.company_guid)?
            .contains(&line.batch_id)
        {
            return Ok(None);
        }
        Ok(self
            .import_snapshot_while_admitted(Some(&line.batch_id))?
            .map(|snapshot| snapshot.batch))
    }

    fn stop_outcome(
        &self,
        line: &ImportLedgerLine,
        state: &str,
        invoice_found: Option<bool>,
        evidence: Evidence,
    ) -> ToolOutcome {
        let next_step = if state == "stop_released" {
            "The stop is released. Invoices for this company can be built and posted again."
        } else {
            "This batch no longer stops the company: nothing was released."
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
