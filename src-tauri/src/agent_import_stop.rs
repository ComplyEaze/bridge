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
//! It stands on no read. It tries a fresh readback so the dialog can say what
//! ComplyEaze Bridge sees of the invoice (`Seen`), and records that beside the
//! release as evidence nothing reads back: a released batch is never the
//! control for the next invoice number read, whatever was seen. A read that
//! fails, for any cause, is shown by its code and the release goes ahead,
//! because no wait cures many of those causes and nothing later depends on
//! this read; only a cancelled call is refused, since no one is there to
//! answer the dialog.
//!
//! The check under the lock before the record compares the batch's sha256 and
//! the stop, not the verification's generation: a read that changes while the
//! dialog is open makes only the dialog's words stale. The stop lives in this
//! data folder's journal; another install posting into the same company is not
//! stopped by it.
use super::*;
use crate::tally::approved_import::{ReviewAcknowledged, ReviewKind, VoucherCount};

/// What the read made for the dialog saw of the batch's invoice, decided on
/// the invoice's own row alone (the one carrying its `bridge_txn_id`), by its
/// status and, where a status has several, its `reason`. Duplicates only add
/// a clause to a verified row.
pub(super) enum Seen {
    /// A voucher of this invoice was found in Tally, and why it is not
    /// confirmed.
    Found(Why),
    /// What was read does not establish whether the invoice is in the book.
    /// Every status or reason not placed below reads so, never as found.
    NotEstablished(Why),
    /// The read did not complete, by its code: it reports nothing.
    Unreadable {
        code: String,
        answer: Option<&'static str>,
    },
}

/// The invoice row's status (or, for several rows, `ROWS_REPEATED`) and the
/// dialog's words for it.
pub(super) struct Why {
    status: String,
    words: String,
}

/// A batch whose invoice is not exactly one voucher (the build refuses one),
/// or whose read holds no row for it, or one without a status: a read that
/// cannot be placed, never a sighting.
const ROWS_MISSING: &str = "import_verification_rows_missing";

/// A completed read holding more than one row for the batch's one invoice.
/// The verification writes one row per voucher the batch expects (several
/// vouchers matching one are `duplicate_fingerprint`), so this is a batch or
/// a result out of shape, and no row can be told to be the invoice's.
const ROWS_REPEATED: &str = "invoice_rows_repeated";

impl Seen {
    pub(super) fn of(result: &Value, line: &ImportLedgerLine) -> Self {
        let missing = || Self::Unreadable {
            code: ROWS_MISSING.into(),
            answer: None,
        };
        let invoices = line
            .vouchers
            .iter()
            .filter(|voucher| voucher.voucher_type.is_invoice())
            .collect::<Vec<_>>();
        let [invoice] = invoices.as_slice() else {
            return missing();
        };
        let rows = result["vouchers"].as_array().map_or_else(Vec::new, |rows| {
            rows.iter()
                .filter(|row| row["bridge_txn_id"] == invoice.bridge_txn_id.as_str())
                .collect()
        });
        let row = match rows.as_slice() {
            [] => return missing(),
            [row] => row,
            [..] => {
                return Self::NotEstablished(Why {
                    status: ROWS_REPEATED.into(),
                    words: "not established: the read gave this invoice more than one row".into(),
                })
            }
        };
        let Some(status) = row["status"].as_str() else {
            return missing();
        };
        let why = |words: &str| Why {
            status: status.to_string(),
            words: words.to_string(),
        };
        match (status, row["reason"].as_str()) {
            ("posted_verified", _) => Self::Found(Why {
                status: status.to_string(),
                words: verified_words(result),
            }),
            ("posted_divergent", _) => Self::Found(why("found in Tally, not confirmed as built")),
            ("posted_not_effective", Some("voucher_cancelled")) => {
                Self::Found(why("found in Tally, cancelled"))
            }
            ("posted_not_effective", Some("voucher_optional")) => {
                Self::Found(why("found in Tally, optional"))
            }
            // After an answer that was not clean the verification drops the
            // edit example, by the next step it writes; so do these words.
            ("sent_not_attributed", _)
                if row["next_step"]
                    == super::verification::SENT_NOT_ATTRIBUTED_AFTER_UNCLEAN_ANSWER_NEXT_STEP =>
            {
                Self::NotEstablished(why(
                    "not established: no voucher matching this invoice was found, and one may be in the book",
                ))
            }
            ("sent_not_attributed", _) => Self::NotEstablished(why(
                "not established: no voucher matching this invoice was found, and one may be in the book, changed or entered in Tally",
            )),
            ("matching_content_observed", _) => Self::NotEstablished(why(
                "a voucher with this invoice's content was found, but not shown to be the one sent",
            )),
            ("duplicate_fingerprint", _) => Self::NotEstablished(why(
                "vouchers with this invoice's content were found, none shown to be the one sent",
            )),
            (
                "not_attributable",
                Some("tag_precedes_pre_import_voucher_mark" | "fingerprint_precedes_pre_import_voucher_mark"),
            ) => Self::NotEstablished(why(
                "a matching voucher was found, but it was in the book before this post",
            )),
            ("not_attributable", Some("observed_voucher_already_attributed")) => Self::NotEstablished(why(
                "a matching voucher was found, but it is already matched to another voucher",
            )),
            ("not_attributable", Some("bound_master_id_changed")) => Self::NotEstablished(why(
                "a voucher carrying the identity of the one posted is in the book, but with another internal number; how is not established, for example the company's data were rewritten or copied",
            )),
            ("bound_not_in_window", _) => Self::NotEstablished(why(
                "the voucher posted for this invoice is not in the book for these dates now",
            )),
            ("book_rolled_back", _) => {
                Self::NotEstablished(why("the company's books were restored after this post"))
            }
            // The release cannot tell absence: these say only what the row
            // says, never a bare "not found".
            ("not_found", _) => {
                Self::NotEstablished(why("not established: no voucher of it was found"))
            }
            ("tally_reported_not_created", _) => {
                Self::NotEstablished(why("Tally reported it not created"))
            }
            _ => Self::NotEstablished(why(
                "not established: the read gave this invoice a state ComplyEaze Bridge does not place",
            )),
        }
    }

    /// A failed read, shown by its code, and by what Tally did not do when
    /// that is known.
    pub(super) fn unreadable(failure: &ToolFailure) -> Self {
        let unanswered = failure.unanswered.map(|unanswered| unanswered.0);
        let answer = if failure.code == "request_deadline_exceeded"
            || unanswered == Some("request_deadline_exceeded")
        {
            Some("Tally did not answer in time")
        } else if unanswered == Some("endpoint_unreachable") {
            Some("Tally did not answer")
        } else {
            None
        };
        Self::Unreadable {
            code: failure.code.clone(),
            answer,
        }
    }

    /// What the journal records.
    pub(super) fn recorded(&self) -> ledger::ReleaseSeen {
        match self {
            Self::Found(_) => ledger::ReleaseSeen::Found,
            Self::NotEstablished(_) => ledger::ReleaseSeen::NotEstablished,
            Self::Unreadable { .. } => ledger::ReleaseSeen::Unreadable,
        }
    }

    /// The tool result's `seen_at_release` and `seen_detail`: codes only.
    pub(super) fn reported(&self) -> (&'static str, &str) {
        match self {
            Self::Found(why) => ("found", &why.status),
            Self::NotEstablished(why) => ("not_established", &why.status),
            Self::Unreadable { code, .. } => ("unreadable", code),
        }
    }

    pub(super) fn words(&self) -> String {
        match self {
            Self::Found(why) | Self::NotEstablished(why) => why.words.clone(),
            Self::Unreadable { code, answer } => format!(
                "ComplyEaze Bridge could not complete its read ({code}{}), so it reports nothing about this invoice",
                answer.map_or_else(String::new, |answer| format!(": {answer}"))
            ),
        }
    }
}

/// Why a row that reads verified still stops its company: a duplicate of it
/// (by kind), or a dispatch needing reconciliation, which is Tally's answer to
/// the post unless that answer was clean, when it is the masters check (or
/// the duplicate already named).
fn verified_words(result: &Value) -> String {
    let mut clauses = Vec::new();
    for duplicate in result["duplicates"].as_array().into_iter().flatten() {
        let clause = match duplicate["kind"].as_str() {
            Some("accounting_fingerprint") => "another voucher has the same content",
            Some("remote_id") => "another voucher has the same remote id",
            _ => "another voucher reads as its duplicate",
        };
        if !clauses.contains(&clause) {
            clauses.push(clause);
        }
    }
    // A duplicate alone makes the dispatch need reconciliation, so with a
    // clean answer the masters check is the cause only when none was found.
    let dispatch = &result["dispatch"];
    if dispatch["state"] == "reconciliation_required" {
        if dispatch["response_state"] != "response_clean" {
            clauses.push("Tally's answer to the post needs a check");
        } else if clauses.is_empty() {
            clauses.push("its ledgers are not confirmed as the masters approved");
        }
    }
    if clauses.is_empty() {
        "found in Tally as built".into()
    } else {
        format!(
            "found in Tally as built, but not confirmed: {}",
            clauses.join("; ")
        )
    }
}

/// `text` as the dialog shows it: a character that could reorder or hide what
/// the person reads (the same ones the review dialog refuses) shows as `?`.
/// The text comes from the journal and Tally, and a release must not become
/// impossible for a name that carries one.
pub(super) fn shown(text: &str) -> String {
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

/// The words of the dialog. They say what is being released and what it
/// allows, before anything about the batch, because the title of the shared
/// review dialog names only a review. The company is named by the batch's
/// recorded name, or by its GUID when it recorded none.
pub(super) fn release_preview(line: &ImportLedgerLine, seen: &Seen) -> String {
    let company = line
        .company
        .as_ref()
        .map(|company| company.name.as_str())
        .filter(|name| !name.is_empty())
        .unwrap_or(&line.company_guid);
    let (company_name, batch_id) = (shown(company), shown(&line.batch_id));
    let invoices = line
        .vouchers
        .iter()
        .filter(|voucher| voucher.voucher_type.is_invoice())
        .map(|voucher| {
            format!(
                "  {} dated {}",
                shown(voucher.voucher_number.as_deref().unwrap_or("(no number)")),
                shown(voucher.date.as_str())
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
        self.require_stop(&line)?;

        // What Tally holds now, to show. A cancelled call is refused: the
        // caller has gone. On this read only the runtime's typed withdrawal
        // (`ToolFailure::from_runtime`) gives that code; the post's approval
        // holds, which also give it, are not on this path.
        let mut rows = None;
        let (seen, evidence) = match self.verify_for_review(args, &mut rows).await {
            Ok(outcome) => (
                Seen::of(&outcome.payload["result"], &line),
                outcome.evidence,
            ),
            Err(failure) if failure.code == "request_cancelled" => return Err(failure),
            Err(failure) => (
                Seen::unreadable(&failure),
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
        // The stop may have lifted while the read ran (the batch read verified
        // or was released meanwhile, whether or not this read completed):
        // nothing is then asked.
        let Some(held) = self.stop_held(&line)? else {
            return Ok(self.stop_outcome(&line, "not_held", None, evidence));
        };
        let preview = release_preview(&held.batch, &seen);
        let count = VoucherCount::new(1).ok_or_else(|| "ack_stop_batch_not_sent".to_string())?;
        let _acknowledged = ReviewAcknowledged::confirm(ReviewKind::StopRelease, count, &preview)
            .await
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;

        // Recorded under the exclusive lock, and only if the batch still
        // stops the company as the person was shown: no record of it since
        // (a stop that lifted and came back while the dialog was open is
        // another stop than the one shown).
        let _lock = self.lock_import_admission()?;
        let current = self
            .import_snapshot_while_admitted(Some(&line.batch_id))?
            .ok_or_else(|| "import_batch_not_found".to_string())?;
        if current.generation != held.generation
            || current.batch.sha256 != line.sha256
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
            seen.recorded(),
        ))?;
        if self
            .import_invoice_stops_while_admitted(&line.company_guid)?
            .contains(&line.batch_id)
        {
            return Err(ToolFailure::from("ack_stop_not_recorded".to_string())
                .with_prior_evidence(evidence));
        }
        Ok(self.stop_outcome(&line, "stop_released", Some(&seen), evidence))
    }

    /// Refuses a batch that does not stop its company now: it reads verified,
    /// or a person already released it.
    fn require_stop(&self, line: &ImportLedgerLine) -> Result<(), ToolFailure> {
        self.stop_held(line)?
            .map(|_| ())
            .ok_or_else(|| "ack_stop_not_held".to_string().into())
    }

    /// The batch as the journal holds it now, with the generation of its
    /// latest record, when it stops its company.
    fn stop_held(&self, line: &ImportLedgerLine) -> Result<Option<ledger::BatchSnapshot>, String> {
        let _lock = self.lock_import_admission_shared()?;
        if !self
            .import_invoice_stops_while_admitted(&line.company_guid)?
            .contains(&line.batch_id)
        {
            return Ok(None);
        }
        self.import_snapshot_while_admitted(Some(&line.batch_id))
    }

    fn stop_outcome(
        &self,
        line: &ImportLedgerLine,
        state: &str,
        seen: Option<&Seen>,
        evidence: Evidence,
    ) -> ToolOutcome {
        let next_step = if state == "stop_released" {
            "The stop is released. Invoices for this company can be built and posted again."
        } else {
            "This batch no longer stops the company: nothing was released."
        };
        let (seen_at_release, seen_detail) = seen.map(Seen::reported).unzip();
        ToolOutcome {
            payload: json!({"result": {
                "batch_id": line.batch_id,
                "state": state,
                "seen_at_release": seen_at_release,
                "seen_detail": seen_detail,
                "next_step": next_step,
            }}),
            evidence,
            company_guid: Some(line.company_guid.clone()),
            truncated: false,
        }
    }
}
