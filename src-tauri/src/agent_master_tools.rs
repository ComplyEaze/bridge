//! The two master tools: `prepare_ledger_creates` and `post_ledger_creates`.
//!
//! They are listed only while `BRIDGE_AGENT_ENABLE_MASTER_WRITES` is in force
//! (with posting on) and refuse `master_writes_disabled` otherwise.
//!
//! **prepare** parses the caller's ledger into a [`LedgerCreateItem`] at the
//! boundary (AGENTS.md P3), verifies the company, reads the company's ledger
//! and group name indexes once, refuses a name that already exists, and saves a
//! [`MasterBatch`] in the masters journal. It writes nothing to Tally.
//!
//! **post** is the voucher post's approval flow for one ledger: the same
//! Pending, Redeem and Join steps (`begin_agent_approval`, `ask_and_hold`), the
//! same single approval slot, then `post_approved_import` with the masters
//! kind. Its `before_dispatch`, under the exclusive admission lock, refuses a
//! batch the journal shows dispatched or changed, spends the approval, writes
//! the pre-image sidecar and appends the dispatch intent. The intent comes back
//! as a [`RecordedIntent`] that the runtime requires before it sends: a master
//! POST is not built without an intent recorded for the very bytes it carries.
//!
//! After the POST this piece appends the [`MasterDispatchResponse`], reads the
//! book back and records a verdict (`agent_master_verify.rs`, which reads, and
//! `agent_master_verdict.rs`, which decides over parsed values). A call for a
//! batch the journal shows dispatched runs the same read-back as a reconcile:
//! it reads and never sends. Only when the read-back cannot be made at all
//! (the company cannot be verified, the journal cannot describe the batch)
//! does the answer fall back to the journal's own `posted_unverified`. When
//! Tally's answer itself cannot be appended, the read-back and the verdict go
//! on with the counters the call holds: the answer is appended again under the
//! verdict's lock, before the verdict, and no verdict is written without it.
//! Where the journal cannot be read after the POST, the call answers
//! `posted_unverified` with Tally's counters and `answer_recorded` saying
//! whether the answer is on file. The call that got Tally's answer never says
//! the create may have been sent; a later call that finds an intent and no
//! recorded answer cannot know Tally answered, and says so.
use super::approval::{ApprovalBinding, HeldRequest};
use super::master_ledger::{
    append, append_intent, read_journal, remove_pre_image, write_pre_image, BatchState,
    BlockingBatch, ExclusiveAdmission, MasterBatch, MasterClass, MasterCompany, MasterCounters,
    MasterDispatchIntent, MasterDispatchResponse, MasterJournal, MasterLedgerError, MasterRecord,
    PreImage, RecordedIntent, StoredLedgerCreate, StoredLedgerKind,
};
use super::master_post::{
    check_name_free, recheck_master_for_dispatch, MasterAim, MasterPostRequest,
};
use super::master_review::{masked_review_text, master_review_text};
use super::post::{refusal_cause, refusal_code, ApprovalTarget, Entry, Pass, Step};
use super::*;
use crate::agent::evidence_from_runtime_read;
use crate::agent::read_profiles::{ledger_name_index_read, native_group_snapshot_read};
use crate::tally::approved_import::{
    ApprovedImportAdmissionError, BeforeDispatchError, PendingPostApproval, PostKind,
    UnderLockRefusal,
};
use crate::tally::ledger_create_item::{
    AdmittedDutyHead, AdmittedLedgerKind, GstinError, LedgerCreateBatch, LedgerCreateBatchError,
    LedgerCreateItem, MasterName, MasterNameError, PartySide, VerifiedGstin,
};
use bridge_tally_protocol::GstDutyHead;
use std::sync::Mutex;

#[path = "agent_master_verdict.rs"]
pub(super) mod verdict;
#[path = "agent_master_verify.rs"]
mod verify;
use verdict::Moment;
use verify::{
    not_read_back_next_step, settled_alert, settled_next_step, settled_note, PostedAnswer,
    NEXT_ANSWER_NOT_RECORDED, NEXT_NOT_READ_BACK_AFTER_ANSWER, NEXT_SEND_NOT_KNOWN,
    NEXT_UNCONFIRMED,
};

/// What a caller is told after a ledger is prepared.
const PREPARED_NEXT_STEP: &str = "Nothing was written to Tally. To create this ledger, call post_ledger_creates with this batch_id: the local user must approve it in a native dialog first.";
/// The same after several ledgers are prepared in one batch.
const PREPARED_BATCH_NEXT_STEP: &str = "Nothing was written to Tally. To create these ledgers, call post_ledger_creates with this batch_id: the local user must approve them, all together, in a native dialog first. They are sent to Tally in one request.";

/// What a caller is told after the one POST when Tally answered and the answer
/// could not be appended to the journal, whatever stopped the append (its code
/// is beside the note): the ledger is not verified.
const ANSWER_UNRECORDED_NOTE: &str = "Tally answered the one create (any counters it gave are here), but ComplyEaze Bridge could not record that answer and has not read the ledger back. It is NOT verified. Never post this batch again: check the ledger in Tally before relying on it.";
/// What a caller is told after the one POST when the read-back could not be
/// attempted at all: the ledger is not verified.
const POSTED_UNVERIFIED_NOTE: &str = "Tally answered the one create and ComplyEaze Bridge recorded the answer, but has not read the ledger back: it is NOT verified. Check the ledger in Tally before relying on it. Never post this batch again.";

/// What a caller is told when a batch has a recorded intent and no recorded
/// answer: the create may or may not have reached Tally.
const MAY_HAVE_BEEN_SENT_NOTE: &str = "This batch may have been sent to Tally once: an intent to send it is recorded, and no answer from Tally is. Nothing was sent now and nothing will be sent again from this batch. The ledger is NOT verified: check the company's ledgers in Tally.";

/// What a caller is told when a batch was sent and Tally's answer is recorded.
const ALREADY_SENT_NOTE: &str = "This batch was sent to Tally once and its answer is recorded. Nothing was sent now and nothing will be sent again from this batch. This is what ComplyEaze Bridge recorded; the ledger is NOT verified.";

/// What a post withdrawn by its caller is told, by what the journal showed of
/// its batch. Nothing was sent: the batch can be posted again.
const CANCELLED_NOT_SENT_MESSAGE: &str =
    "The request was cancelled. No send is recorded for this prepared ledger.";
/// A send is recorded: the create may have reached Tally, and a later call reads
/// the book back and sends nothing.
const CANCELLED_SEND_RECORDED_MESSAGE: &str = "The request was cancelled after a send of this batch was recorded. The create may have been sent to Tally: never send this batch again.";
/// The journal could not be read, or does not hold the batch, so whether a send
/// was recorded is not known.
const CANCELLED_SEND_NOT_KNOWN_MESSAGE: &str = "The request was cancelled, and whether a send of this batch was recorded is not known: the journal could not be read just now, or holds no such batch. Check the company's ledgers in Tally.";

/// The refusal code of a [`MasterNameError`].
fn master_name_code(error: MasterNameError) -> &'static str {
    match error {
        MasterNameError::Empty => "master_name_empty",
        MasterNameError::NonAscii => "master_name_not_ascii",
        MasterNameError::ControlCharacter => "master_name_control_character",
        MasterNameError::SurroundingWhitespace => "master_name_surrounding_whitespace",
        MasterNameError::RepeatedSpaces => "master_name_repeated_spaces",
        MasterNameError::TooLong => "master_name_too_long",
    }
}

/// The refusal code of a [`GstinError`].
fn gstin_code(error: GstinError) -> &'static str {
    match error {
        GstinError::Shape => "gstin_shape_invalid",
        GstinError::CheckCharacter => "gstin_check_character_invalid",
    }
}

/// Parses the caller's `ledger` object into the one typed item, or refuses with
/// a code naming what was wrong. The schema (`validate_against_schema`) has
/// already held the object to its published shape; this is the typed boundary
/// under it. A field that belongs to another kind of ledger is refused, never
/// ignored.
fn parse_ledger_item(ledger: &Value) -> Result<LedgerCreateItem, String> {
    let invalid = || "argument_invalid:ledger".to_string();
    let object = ledger.as_object().ok_or_else(invalid)?;
    let text = |key: &str| -> Result<Option<&str>, String> {
        match object.get(key) {
            None => Ok(None),
            Some(Value::String(text)) => Ok(Some(text.as_str())),
            Some(_) => Err(invalid()),
        }
    };
    let bill_wise = match object.get("bill_wise") {
        None => None,
        Some(Value::Bool(flag)) => Some(*flag),
        Some(_) => return Err(invalid()),
    };
    let kind = text("kind")?.ok_or_else(invalid)?;
    let (side, gstin, duty_head) = (text("side")?, text("gstin")?, text("duty_head")?);
    let kind = match kind {
        "party" | "suspense" | "gst_duty" => kind,
        _ => return Err("master_kind_not_admitted".into()),
    };
    let name = MasterName::new(text("name")?.ok_or_else(invalid)?)
        .map_err(|error| master_name_code(error).to_string())?;
    let not_for_kind = || "ledger_field_not_for_kind".to_string();
    let kind = match kind {
        "party" => {
            if duty_head.is_some() {
                return Err(not_for_kind());
            }
            let side = match side {
                Some("sundry_debtors") => PartySide::SundryDebtors,
                Some("sundry_creditors") => PartySide::SundryCreditors,
                Some(_) => return Err("party_side_invalid".into()),
                None => return Err("party_side_required".into()),
            };
            let gstin = gstin
                .map(VerifiedGstin::new)
                .transpose()
                .map_err(|error| gstin_code(error).to_string())?;
            AdmittedLedgerKind::Party {
                side,
                gstin,
                bill_wise: bill_wise.unwrap_or(false),
            }
        }
        "suspense" => {
            if side.is_some() || gstin.is_some() || bill_wise.is_some() || duty_head.is_some() {
                return Err(not_for_kind());
            }
            AdmittedLedgerKind::Suspense
        }
        _ => {
            if side.is_some() || gstin.is_some() || bill_wise.is_some() {
                return Err(not_for_kind());
            }
            let spelled = duty_head.ok_or_else(|| "gst_duty_head_required".to_string())?;
            let head = GstDutyHead::ALL
                .into_iter()
                .find(|head| head.tally_spelling() == spelled)
                .ok_or_else(|| "gst_duty_head_not_admitted".to_string())?;
            AdmittedLedgerKind::GstDuty {
                head: AdmittedDutyHead::new(head)
                    .map_err(|_| "gst_duty_head_not_admitted".to_string())?,
            }
        }
    };
    Ok(LedgerCreateItem::new(name, kind))
}

/// The code a refused batch answers with. `TooMany` cannot come from the tool,
/// whose published schema caps `ledgers` at the same limit; it stays for every
/// other way a batch is built, a stored one re-read included.
fn batch_code(error: LedgerCreateBatchError) -> &'static str {
    match error {
        LedgerCreateBatchError::Empty => "argument_invalid:ledgers",
        LedgerCreateBatchError::TooMany => "master_batch_too_large",
        LedgerCreateBatchError::MixedParents => "master_batch_mixed_parents",
        LedgerCreateBatchError::NamesMayBeOneMaster => "master_batch_names_may_be_one_master",
    }
}

/// The caller's ledgers, `ledger` (one) or `ledgers` (one or more) and never
/// both, parsed item by item and then admitted as one batch.
fn parse_ledger_creates(args: &Value) -> Result<LedgerCreateBatch, String> {
    let items = match (args.get("ledger"), args.get("ledgers")) {
        (Some(one), None) => vec![parse_ledger_item(one)?],
        (None, Some(Value::Array(list))) => list
            .iter()
            .map(parse_ledger_item)
            .collect::<Result<Vec<_>, _>>()?,
        (None, None) => return Err("ledger_required".into()),
        _ => return Err("argument_invalid:ledgers".into()),
    };
    LedgerCreateBatch::new(items).map_err(|error| batch_code(error).to_string())
}

/// The record of `item` the journal keeps.
fn stored_from_item(item: &LedgerCreateItem) -> StoredLedgerCreate {
    let (kind, bill_wise, party_gstin, gst_duty_head) = match item.kind() {
        AdmittedLedgerKind::Party {
            side,
            gstin,
            bill_wise,
        } => (
            match side {
                PartySide::SundryDebtors => StoredLedgerKind::PartySundryDebtors,
                PartySide::SundryCreditors => StoredLedgerKind::PartySundryCreditors,
            },
            *bill_wise,
            gstin.as_ref().map(|gstin| gstin.as_str().to_string()),
            None,
        ),
        AdmittedLedgerKind::Suspense => (StoredLedgerKind::Suspense, false, None, None),
        AdmittedLedgerKind::GstDuty { head } => (
            StoredLedgerKind::GstDuty,
            false,
            None,
            Some(head.head().tally_spelling().to_string()),
        ),
    };
    StoredLedgerCreate {
        name: item.name().as_str().to_string(),
        kind,
        bill_wise,
        party_gstin,
        gst_duty_head,
    }
}

/// Every item of a stored batch, parsed through the same constructors as a
/// caller's input and admitted as one batch again: a record that no longer
/// parses as an admitted batch is refused, not sent.
fn creates_from_batch(batch: &MasterBatch) -> Result<LedgerCreateBatch, String> {
    let items = batch
        .items
        .iter()
        .map(item_from_stored)
        .collect::<Result<Vec<_>, _>>()?;
    LedgerCreateBatch::new(items).map_err(|_| "master_batch_invalid".to_string())
}

/// One stored item, parsed as a caller's input is.
fn item_from_stored(stored: &StoredLedgerCreate) -> Result<LedgerCreateItem, String> {
    let refused = || "master_batch_invalid".to_string();
    let name = MasterName::new(&stored.name).map_err(|_| refused())?;
    let party = |side| -> Result<AdmittedLedgerKind, String> {
        Ok(AdmittedLedgerKind::Party {
            side,
            gstin: stored
                .party_gstin
                .as_deref()
                .map(VerifiedGstin::new)
                .transpose()
                .map_err(|_| refused())?,
            bill_wise: stored.bill_wise,
        })
    };
    let kind = match stored.kind {
        StoredLedgerKind::PartySundryDebtors => party(PartySide::SundryDebtors)?,
        StoredLedgerKind::PartySundryCreditors => party(PartySide::SundryCreditors)?,
        StoredLedgerKind::Suspense => AdmittedLedgerKind::Suspense,
        StoredLedgerKind::GstDuty => {
            let spelled = stored.gst_duty_head.as_deref().ok_or_else(refused)?;
            let head = GstDutyHead::ALL
                .into_iter()
                .find(|head| head.tally_spelling() == spelled)
                .ok_or_else(refused)?;
            AdmittedLedgerKind::GstDuty {
                head: AdmittedDutyHead::new(head).map_err(|_| refused())?,
            }
        }
    };
    let item = LedgerCreateItem::new(name, kind);
    // The record must be exactly what the item stores back: a stored field the
    // typed item would not carry is a record someone edited.
    if stored_from_item(&item) != *stored {
        return Err(refused());
    }
    Ok(item)
}

/// Enforces the published `inputSchema` on the nested `ledger` object and the
/// `ledgers` list, each item of which is held to the `ledger` object's schema,
/// so their bounds live in one place.
fn enforce_published_schema(args: &Value) -> Result<(), String> {
    let definitions = super::super::catalog::registered_tool_definitions(true, true);
    let schema = definitions
        .as_array()
        .and_then(|tools| {
            tools
                .iter()
                .find(|tool| tool["name"] == "prepare_ledger_creates")
        })
        .map(|tool| tool["inputSchema"].clone())
        .ok_or_else(|| "tool_not_found".to_string())?;
    for key in ["ledger", "ledgers"] {
        if let Some(value) = args.get(key) {
            super::super::catalog::validate_against_schema(value, &schema["properties"][key], key)?;
        }
    }
    if let Some(Value::Array(items)) = args.get("ledgers") {
        for item in items {
            super::super::catalog::validate_against_schema(
                item,
                &schema["properties"]["ledger"],
                "ledgers",
            )?;
        }
    }
    Ok(())
}

fn counters_json(counters: Option<&MasterCounters>) -> Value {
    counters.map_or(Value::Null, |counters| {
        json!({
            "created": counters.created,
            "altered": counters.altered,
            "deleted": counters.deleted,
            "ignored": counters.ignored,
            "errors": counters.errors,
            "cancelled": counters.cancelled,
            "exceptions": counters.exceptions,
        })
    })
}

/// The counters of a response Bridge could read all of. A counter Tally did not
/// report is not a zero (AGENTS.md P3), so a response missing any carries none.
fn reported_counters(
    outcome: Option<&bridge_tally_protocol::TallyImportOutcome>,
) -> Option<MasterCounters> {
    let counters = outcome?.counters();
    counters
        .counter_presence
        .all_reported()
        .then_some(MasterCounters {
            created: counters.created,
            altered: counters.altered,
            deleted: counters.deleted,
            ignored: counters.ignored,
            errors: counters.errors,
            cancelled: counters.cancelled,
            exceptions: counters.exceptions,
        })
}

fn empty_evidence() -> Evidence {
    evidence_from_runtime_read(crate::tally::runtime::RuntimeReadEvidence::empty())
}

fn verdict_state_code(state: super::master_ledger::MasterVerdictState) -> &'static str {
    use super::master_ledger::MasterVerdictState;
    match state {
        MasterVerdictState::PostedVerified => "posted_verified",
        MasterVerdictState::ReconciliationRequired => "reconciliation_required",
        MasterVerdictState::PostedIntoExistingMaster => "posted_into_existing_master",
        MasterVerdictState::MastersAfterPostUnconfirmed => "masters_after_post_unconfirmed",
        MasterVerdictState::MasterWriteLocatedElsewhere => "master_write_located_elsewhere",
        MasterVerdictState::ConfirmedAbsent => "confirmed_absent",
        MasterVerdictState::PostedUnattributed => "posted_unattributed",
    }
}

impl Server {
    /// The masters journal as it stands, read under the shared admission lock.
    fn read_master_journal(&self) -> Result<MasterJournal, String> {
        let _lock = self.lock_import_admission_shared()?;
        read_journal(&self.settings.data_dir).map_err(|error| error.code().to_string())
    }

    /// The journal read of the check that precedes the dialog.
    fn read_master_journal_for_hold_check(&self) -> Result<MasterJournal, String> {
        #[cfg(test)]
        if super::master_ledger::test_fault_fires("hold_check_read_busy") {
            return Err("import_admission_busy".to_string());
        }
        self.read_master_journal()
    }

    /// The one journal read of a failed post, which says whether a send was
    /// recorded and names the batches holding a name. A busy or unreadable
    /// journal is an answer too: a refusal under the lock made after that check
    /// read the journal then answers from what it read (#2083), and any other
    /// failure leaves the fields it would fill out.
    fn read_master_journal_for_failure(&self) -> Result<MasterJournal, String> {
        #[cfg(test)]
        if super::master_ledger::test_fault_fires("failure_read_busy") {
            return Err("import_admission_busy".to_string());
        }
        self.read_master_journal()
    }

    /// The masters journal after Tally answered the one create: the shared read
    /// as [`Self::read_master_journal`], and, when that read fails for any
    /// reason (the admission lock is held by another process or cannot be
    /// opened, or the settled-state check of the imports folder refuses), the
    /// read with no lock. A lockless read of an append in progress fails closed
    /// on a torn last line. Nothing here may fail the call: the caller answers
    /// from what it cannot read.
    fn read_master_journal_after_answer(&self) -> Result<MasterJournal, String> {
        #[cfg(test)]
        if super::master_ledger::test_fault_fires("answer_read_failed") {
            return Err("master_journal_unavailable".to_string());
        }
        let shared = self.read_master_journal();
        #[cfg(test)]
        let shared = if super::master_ledger::test_fault_fires("answer_read_busy") {
            Err("import_admission_busy".to_string())
        } else if super::master_ledger::test_fault_fires("answer_read_lock_unavailable") {
            Err("import_admission_lock_unavailable".to_string())
        } else {
            shared
        };
        match shared {
            Err(_) => {
                read_journal(&self.settings.data_dir).map_err(|error| error.code().to_string())
            }
            other => other,
        }
    }

    /// Whether the masters journal records a dispatch for the batch `args`
    /// names: `Some(false)` is known not sent. Anything not known is `None`, and
    /// a cancellation then waits for the post rather than assume it unsent.
    pub(in crate::agent) fn recorded_master_attempt(
        &self,
        args: &Value,
    ) -> Result<Option<bool>, ToolFailure> {
        let batch_id = required_string(args, "batch_id")?;
        if !valid_batch_id(batch_id) {
            return Err("import_batch_identifier_invalid".to_string().into());
        }
        match self.read_master_journal() {
            Ok(journal) => Ok(journal.state(batch_id).map(|state| state.intent.is_some())),
            Err(error) if error == "import_admission_busy" => Err(error.into()),
            Err(_) => Ok(None),
        }
    }

    /// [`Self::recorded_import_attempt`] for whichever post tool is running.
    pub(in crate::agent) fn recorded_post_attempt(
        &self,
        tool: &str,
        args: &Value,
    ) -> Result<Option<bool>, ToolFailure> {
        if tool == "post_ledger_creates" {
            self.recorded_master_attempt(args)
        } else {
            self.recorded_import_attempt(args)
        }
    }

    /// The answer to a post withdrawn before its intent, by tool.
    pub(in crate::agent) fn cancelled_post_for_response(
        &self,
        tool: &str,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        if tool != "post_ledger_creates" {
            return self.cancelled_import_for_response(args);
        }
        let batch_id = required_string(args, "batch_id")?;
        let attempted = self.recorded_master_attempt(args)?;
        let company_guid = self.read_master_journal().ok().and_then(|journal| {
            journal
                .state(batch_id)
                .map(|state| state.batch.company.guid.clone())
        });
        let mut evidence = empty_evidence();
        evidence.state = "partial";
        evidence.reason_code = Some("request_cancelled".into());
        // The three cases a failure answer has: nothing sent, a send recorded,
        // and a journal that could not be read, which does not know.
        let mut error = json!({"code": "request_cancelled", "message": match attempted {
            Some(false) => CANCELLED_NOT_SENT_MESSAGE,
            Some(true) => CANCELLED_SEND_RECORDED_MESSAGE,
            None => CANCELLED_SEND_NOT_KNOWN_MESSAGE,
        }});
        match attempted {
            Some(false) => {}
            Some(true) => error["next_step"] = json!(NEXT_UNCONFIRMED),
            None => error["next_step"] = json!(NEXT_SEND_NOT_KNOWN),
        }
        Ok(ToolOutcome {
            payload: json!({"result":{
                "batch_id":batch_id,"attempt_recorded":attempted,
                "dispatch":{"state":if attempted == Some(false) { "not_dispatched" } else { "reconciliation_required" },"resent":false},
                "error": error
            }}),
            evidence,
            company_guid,
            truncated: false,
        })
    }

    /// Prepare ONE ledger for creation. Nothing is written to Tally.
    pub(in crate::agent) async fn prepare_ledger_creates(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        enforce_published_schema(args)?;
        let creates = parse_ledger_creates(args)?;
        let (company, identity, identity_evidence) = self.verified_company(guid).await?;
        let mut accumulated = identity_evidence;
        let origin = super::super::canonical_loopback_origin(&self.settings.endpoint)
            .map_err(|_| "host_setting_invalid".to_string())?;
        // The proposed name against the book as it stands, once. The queue
        // makes the binding check again, twice, at the post.
        let (ledgers, evidence, profile) = self
            .post_read_observing_boundary(
                &identity,
                ledger_name_index_read(&company.name).map_err(|error| error.to_string())?,
            )
            .await?;
        accumulated = combine_evidence(accumulated, evidence);
        let (groups, evidence) = self
            .post_read(&identity, native_group_snapshot_read(&company.name))
            .await?;
        accumulated = combine_evidence(accumulated, evidence);
        require_licensed_mode(profile)?;
        check_name_free(&ledgers, &groups, &creates, identity.company_guid())
            .map_err(|error| name_check_failure(&error))?;
        let request = MasterPostRequest::render(creates.clone(), &identity);
        let preview = if self.settings.redaction == super::super::Redaction::MaskParties {
            masked_review_text(&creates, &identity, &origin)
        } else {
            master_review_text(&creates, &identity, &origin)
        }
        .map_err(|error| ToolFailure::from(error.code().to_string()))?;
        let batch = MasterBatch {
            batch_id: format!("bridge-{}", Uuid::new_v4()),
            batch_sha256: request.sha256().to_string(),
            class: MasterClass::LedgerCreate,
            company: MasterCompany {
                guid: identity.company_guid().to_string(),
                name: identity.display_name().to_string(),
                number: identity.company_number().to_string(),
                books_from: identity.books_from_yyyymmdd().to_string(),
            },
            items: creates.items().iter().map(stored_from_item).collect(),
        };
        {
            let lock = ExclusiveAdmission::acquire(&self.settings.data_dir)?;
            append(
                &self.settings.data_dir,
                &lock,
                MasterRecord::Batch(batch.clone()),
            )
            .map_err(|error| error.code().to_string())?;
        }
        Ok(ToolOutcome {
            payload: json!({"result":{
                "batch_id": batch.batch_id,
                "preview": preview,
                "parent": creates.parent(),
                "ledger_count": creates.len(),
                "next_step": if creates.len() == 1 {
                    PREPARED_NEXT_STEP
                } else {
                    PREPARED_BATCH_NEXT_STEP
                },
            }}),
            evidence: accumulated,
            company_guid: Some(batch.company.guid),
            truncated: false,
        })
    }

    /// Ask the person and, once approved, send the one create. See the module
    /// documentation for the flow and for what this piece does not do.
    pub(in crate::agent) async fn post_ledger_creates(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        match self.post_ledger_creates_entry(args, Entry::Fresh).await? {
            Pass::Done(outcome) => Ok(outcome),
            // At most one more pass: a redeem-only pass never joins, so it
            // never hands on another redeem.
            Pass::Redeem {
                evidence,
                call_started,
            } => match self
                .post_ledger_creates_entry(
                    args,
                    Entry::RedeemOnly {
                        evidence,
                        call_started,
                    },
                )
                .await?
            {
                Pass::Done(outcome) => Ok(outcome),
                Pass::Redeem { .. } => Err("import_approval_revoked".to_string().into()),
            },
        }
    }

    async fn post_ledger_creates_entry(
        &self,
        args: &Value,
        entry: Entry,
    ) -> Result<Pass, ToolFailure> {
        let Entry::RedeemOnly { call_started, .. } = &entry else {
            return self.post_ledger_creates_pass(args, entry).await;
        };
        // A redeem-only pass runs on the wire-wait budget the call has left, as
        // the voucher post's does (#893).
        let shared_left = crate::tally::runtime::operation_wire_budget_remaining()
            .unwrap_or_else(|| self.runtime.wire_gate_config().retry().total());
        let budget = super::post::redeem_pass_wire_budget(call_started.elapsed(), shared_left);
        crate::tally::runtime::with_operation_wire_budget_of(
            budget,
            self.post_ledger_creates_pass(args, entry),
        )
        .await
    }

    async fn post_ledger_creates_pass(
        &self,
        args: &Value,
        entry: Entry,
    ) -> Result<Pass, ToolFailure> {
        let (carried, call_started) = match entry {
            Entry::Fresh => (None, std::time::Instant::now()),
            Entry::RedeemOnly {
                evidence,
                call_started,
            } => (Some(evidence), call_started),
        };
        let redeem_only = carried.is_some();
        let batch_id = required_string(args, "batch_id")?;
        // A batch id the masters journal does not hold has no master approval to
        // withdraw, and must not withdraw a voucher batch's: it is refused
        // without touching the approval slot.
        if !valid_batch_id(batch_id) {
            return Err("import_batch_identifier_invalid".to_string().into());
        }
        let admitted: Result<BatchState, ToolFailure> = (|| {
            let journal = self.read_master_journal()?;
            let state = journal
                .state(batch_id)
                .cloned()
                .ok_or_else(|| "master_batch_not_found".to_string())?;
            Ok(state)
        })();
        let state = admitted?;
        let batch = state.batch.clone();
        let guid = batch.company.guid.clone();
        let mut accumulated = carried.unwrap_or_else(empty_evidence);
        // What the check under the lock read of the journal when it refused,
        // kept for the failure answer below (#2083).
        let under_lock = Mutex::new(None::<UnderLockView>);
        let operation: Result<Step, ToolFailure> = async {
            if state.intent.is_some() {
                // Sent by any route: whatever was held for it can never be
                // sent, and must not keep other batches waiting (#725).
                self.post_approvals.revoke_of(
                    PostKind::Masters,
                    batch_id,
                    "batch_already_dispatched",
                );
                return Ok(Step::Done(Box::new(
                    self.reconcile_master_batch(&state, accumulated.clone())
                        .await,
                )));
            }
            let creates = creates_from_batch(&batch)?;
            let target = ApprovalTarget {
                tool: "post_ledger_creates",
                batch_id,
                guid: &guid,
                vouchers: creates.len(),
            };
            // A dialog or approval an earlier call left for this batch (#725).
            let redeeming = match self
                .begin_agent_approval(&target, redeem_only, call_started, &accumulated)
                .await?
            {
                Ok((redeeming, _form)) => redeeming,
                Err(step) => return Ok(step),
            };
            let (company, identity, identity_evidence) = self.verified_company(&guid).await?;
            accumulated = combine_evidence(accumulated.clone(), identity_evidence);
            if company.name != batch.company.name
                || identity.company_number() != batch.company.number
                || identity.books_from_yyyymmdd() != batch.company.books_from
                || !identity
                    .company_guid()
                    .eq_ignore_ascii_case(&batch.company.guid)
            {
                return Err("company_identity_mismatch".to_string().into());
            }
            // Rendered once. Its digest must be the batch's, so a record the
            // journal holds that no longer renders to its own digest is not sent.
            let request = MasterPostRequest::render(creates.clone(), &identity);
            if request.sha256() != batch.batch_sha256 {
                return Err("import_batch_changed".to_string().into());
            }
            let origin = super::super::canonical_loopback_origin(&self.settings.endpoint)
                .map_err(|_| "host_setting_invalid".to_string())?;
            // The name against the book as it stands, before the person is
            // asked about it; the queue checks again, twice, after the approval.
            let (ledgers, evidence, profile) = self
                .post_read_observing_boundary(
                    &identity,
                    ledger_name_index_read(&company.name).map_err(|error| error.to_string())?,
                )
                .await?;
            accumulated = combine_evidence(accumulated.clone(), evidence);
            let (groups, evidence) = self
                .post_read(&identity, native_group_snapshot_read(&company.name))
                .await?;
            accumulated = combine_evidence(accumulated.clone(), evidence);
            require_licensed_mode(profile)?;
            check_name_free(&ledgers, &groups, &creates, identity.company_guid())
                .map_err(|error| name_check_failure(&error))?;
            // A name an open dispatch holds is refused before the person is
            // asked, whether this pass asks or redeems an approval given
            // earlier. The journal is read again here, after the Tally reads
            // above, which take seconds: a holder that settled since the pass
            // began does not refuse, and one that appeared does. It is an answer
            // of its own, never the failure path below. Nothing is recorded for
            // the batch. The check under the lock in `record_master_intent`
            // stays, and can still refuse after an approval.
            #[cfg(test)]
            let _ = super::master_ledger::BEFORE_HOLD_CHECK.try_with(|before| before());
            // A read that fails here skips this check and goes on as the post
            // did before it existed: the check under the lock decides, and this
            // read adds no failure and withdraws no approval.
            if let Ok(fresh) = self.read_master_journal_for_hold_check() {
                let view = overlap_view(Some(&fresh), batch_id, &guid);
                if !matches!(view, OverlapView::Settled) {
                    self.post_approvals.revoke_unredeemed_of(
                        PostKind::Masters,
                        batch_id,
                        "post_refused_before_intent",
                    );
                    return Ok(Step::Done(Box::new(overlap_refusal_outcome(
                        batch_id,
                        &guid,
                        accumulated.clone(),
                        &view,
                    ))));
                }
            }
            let preview = master_review_text(&creates, &identity, &origin)
                .map_err(|error| ToolFailure::from(error.code().to_string()))?;
            let company_marks_request = crate::tally::agent_read_request::AgentReadRequest::parse(
                super::super::read_profiles::render_agent_company_high_water(&company.name),
            )
            .map_err(|error| error.to_string())?;
            let binding =
                ApprovalBinding::masters(&batch.batch_sha256, &origin, &identity, &creates, &preview);
            if !redeeming {
                let asked = PendingPostApproval::ask_masters(
                    creates.clone(),
                    identity.clone(),
                    preview.clone(),
                    company_marks_request,
                );
                if let Some(step) = self
                    .ask_and_hold(
                        &target,
                        asked,
                        &binding,
                        request.clone(),
                        call_started,
                        &accumulated,
                    )
                    .await?
                {
                    return Ok(step);
                }
            }
            let (taken, approved, held) =
                self.post_approvals.take_for_dispatch(batch_id, &binding)?;
            // The binding was a master binding, so only a master request can
            // come back, and it must be the one just rendered: anything else is
            // a wiring fault, refused with the approval unspent.
            // The licence the send needs names this batch: bind the approval to it.
            let approved = approved.bound_to_batch(batch_id);
            let HeldRequest::Masters(held) = held else {
                return Err("import_approval_binding_changed".to_string().into());
            };
            if held != request || !approved.sends_exactly(request.xml()) {
                return Err("import_approval_binding_changed".to_string().into());
            }
            let _endpoint_dispatch_lease = dispatch_lease::acquire(&self.settings.endpoint)?;
            let aim = Mutex::new(None::<MasterAim>);
            let posted = self
                .runtime
                .post_approved_import(
                    self.tally_config(),
                    &identity,
                    approved,
                    |queued| recheck_master_for_dispatch(queued, &aim),
                    || self.record_master_intent(&batch, &request, taken.id(), &aim, &under_lock),
                )
                .await;
            // An approval this post did not spend lapses: it is never offered
            // again (#725).
            drop(taken);
            // Whatever the outcome, the book may have changed: no ledger
            // listing of this company is continued from before it (#630).
            self.drop_listing_snapshots(identity.company_guid());
            let posted = posted.map_err(|error| {
                let code = refusal_code(&error);
                let transport = (code == "post_queue_read_failed")
                    .then(|| {
                        error.chain().find_map(|cause| {
                            cause
                                .downcast_ref::<bridge_tally_transport::TallyTransportError>()
                                .map(bridge_tally_transport::TallyTransportError::safe_code)
                        })
                    })
                    .flatten();
                let group = error.chain().find_map(|cause| {
                    match cause.downcast_ref::<ApprovedImportAdmissionError>() {
                        Some(ApprovedImportAdmissionError::GroupExportInvalid { cause }) => *cause,
                        Some(ApprovedImportAdmissionError::MasterLedgerIndexUnavailable {
                            cause,
                        }) => Some(*cause),
                        _ => None,
                    }
                });
                let mut failure = ToolFailure::from_runtime(code, error);
                if failure.cause.is_none() {
                    failure.cause = refusal_cause(&failure.code, group, transport);
                }
                failure
            })?;
            accumulated = combine_evidence(
                accumulated.clone(),
                evidence_from_runtime_read(posted.admission_evidence.clone()),
            );
            accumulated = combine_evidence(
                accumulated.clone(),
                evidence_from_runtime_read(posted.response_evidence.clone()),
            );
            let outcome = bridge_tally_protocol::parse_import_outcome(&posted.body).ok();
            let counters = reported_counters(outcome.as_ref());
            let response = MasterDispatchResponse {
                batch_id: batch.batch_id.clone(),
                batch_sha256: batch.batch_sha256.clone(),
                response_sha256: posted.response_evidence.response_sha256.clone(),
                counters,
            };
            // The answer is tried once, here. Known limit, shared with the
            // voucher path's response append: the lock is tried and never
            // waited for, so another process holding it at this instant fails
            // the append, as does any failure of the append itself. The send is
            // already recorded by its intent. Tally did answer, so whatever
            // happens below the call says so, never that the create may have
            // been sent: an answer that failed here is appended again, under
            // the verdict's own lock, before the verdict.
            let appended = ExclusiveAdmission::acquire(&self.settings.data_dir).and_then(|lock| {
                append(
                    &self.settings.data_dir,
                    &lock,
                    MasterRecord::DispatchResponse(response.clone()),
                )
                .map_err(|error| error.code().to_string())
            });
            // The book as it stands now, with the intent recorded and, when the
            // append worked, the answer: read back and judged. A journal that
            // cannot be read, or has no intent for the batch, falls back to
            // the account of what Tally answered.
            let journal = self.read_master_journal_after_answer();
            let payload = match &journal {
                Ok(journal) => match journal.state(batch_id).filter(|fresh| fresh.intent.is_some())
                {
                    Some(fresh) => {
                        self.verify_master_batch(
                            fresh,
                            Moment::JustPosted,
                            &identity,
                            call_started,
                            &mut accumulated,
                            PostedAnswer {
                                counters,
                                unrecorded: appended.is_err().then_some(&response),
                            },
                        )
                        .await
                    }
                    None => Err("master_batch_not_found".to_string()),
                },
                Err(code) => Err(code.clone()),
            };
            let payload = payload.unwrap_or_else(|code| {
                let mut result = json!({
                    "batch_id": batch.batch_id,
                    "state": "posted_unverified",
                    "attempt_recorded": true,
                    "answer_recorded": appended.is_ok(),
                    "dispatch": {"state": "dispatched", "resent": false},
                    "counters": counters_json(response.counters.as_ref()),
                    "verification": {"attempted": false, "error": {"code": code}},
                    "note": if appended.is_ok() { POSTED_UNVERIFIED_NOTE } else { ANSWER_UNRECORDED_NOTE },
                    "next_step": fallback_next_step(appended.is_ok()),
                });
                if let Err(code) = &appended {
                    result["verification"]["answer_record_error"] = json!(code);
                }
                json!({ "result": result })
            });
            Ok(Step::Done(Box::new(ToolOutcome {
                payload,
                evidence: accumulated.clone(),
                company_guid: Some(guid.clone()),
                truncated: false,
            })))
        }
        .await;
        // A Join that found a click it can redeem in this call hands it to a
        // second, redeem-only pass from the top (#725 slice 2.0). The approval
        // stays held for that pass: nothing below revokes it.
        let operation = match operation {
            Ok(Step::Redeem) => {
                return Ok(Pass::Redeem {
                    evidence: accumulated,
                    call_started,
                })
            }
            Ok(Step::Done(outcome)) => Ok(*outcome),
            Err(failure) => Err(failure),
        };
        // A refused redemption withdraws the approval it was to use (#725): a
        // later call asks the person again.
        if operation.is_err() {
            self.post_approvals.revoke_unredeemed_of(
                PostKind::Masters,
                batch_id,
                "post_refused_before_intent",
            );
        }
        match operation {
            Ok(result) => Ok(Pass::Done(result)),
            Err(failure) => {
                // Whether a send was recorded decides what the caller is told:
                // an unreadable journal is not "not sent". One read serves
                // both that and, for the check under the lock, the batches
                // that hold the name.
                #[cfg(test)]
                let _ = super::master_ledger::BEFORE_FAILURE_READ.try_with(|before| before());
                let journal = self.read_master_journal_for_failure().ok();
                // A read that fails now, for any reason (another call holding
                // the journal's lock to write, or a journal that cannot be
                // read), falls back to what the check under the lock read when
                // it refused. A view is kept only by that refusal,
                // which ends the post, so this failure is that refusal: the
                // answer then says what it was decided on, never "not known"
                // (#2083). It is as of the refusal: a send another call
                // recorded since is not in it, so its message says only what is
                // true either way (`as_of_refusal`). A read that works wins, so
                // a holder settled since is told.
                let held = under_lock
                    .into_inner()
                    .ok()
                    .flatten()
                    .filter(|_| journal.is_none());
                let kept = held.is_some();
                if failure.code == OVERLAP_CODE {
                    // The typed refusal: this call recorded nothing and sent
                    // nothing, whatever the journal shows now.
                    let view = match held {
                        Some(held) => held.overlap,
                        None => overlap_view(journal.as_ref(), batch_id, &guid),
                    };
                    let outcome = overlap_refusal_outcome(batch_id, &guid, accumulated, &view);
                    return Ok(Pass::Done(as_of_refusal(outcome, kept, true)));
                }
                let attempted = journal
                    .as_ref()
                    .map(|journal| journal.dispatched(batch_id))
                    .or(held.map(|held| held.attempted));
                let outcome =
                    master_failure_outcome(batch_id, &guid, failure, accumulated, attempted);
                Ok(Pass::Done(as_of_refusal(outcome, kept, false)))
            }
        }
    }

    /// `before_dispatch` of a master post, under the exclusive admission lock.
    ///
    /// Refuses a batch the journal shows dispatched or changed, spends the
    /// approval, writes the pre-image sidecar and appends the dispatch intent;
    /// the [`RecordedIntent`] it returns is the licence the runtime requires to
    /// send. Every refusal before the intent append is a named one with nothing
    /// recorded; a failure of the append itself keeps the catch-all, because the
    /// intent may have been recorded.
    fn record_master_intent(
        &self,
        batch: &MasterBatch,
        request: &MasterPostRequest,
        approval: Uuid,
        aim: &Mutex<Option<MasterAim>>,
        under_lock: &Mutex<Option<UnderLockView>>,
    ) -> Result<RecordedIntent, BeforeDispatchError> {
        use BeforeDispatchError::{Other, Refused};
        let data_dir = &self.settings.data_dir;
        let lock = ExclusiveAdmission::acquire(data_dir).map_err(Other)?;
        let journal = read_journal(data_dir)
            .map_err(|_| Refused(UnderLockRefusal::MasterJournalUnavailable))?;
        // Each refusal below is made before any byte of the intent is written,
        // so the journal just read is what the refusal was decided on: it is
        // kept for the answer. A failure after which the intent may be in the
        // file keeps nothing.
        let refused = |refusal: UnderLockRefusal| {
            if let Ok(mut held) = under_lock.lock() {
                *held = Some(UnderLockView::of(&journal, batch));
            }
            Refused(refusal)
        };
        let current = journal
            .state(&batch.batch_id)
            .ok_or_else(|| refused(UnderLockRefusal::BatchNotFound))?;
        if current.intent.is_some() {
            return Err(refused(UnderLockRefusal::AlreadyAttempted));
        }
        if current.batch.batch_sha256 != batch.batch_sha256 || current.batch != *batch {
            return Err(refused(UnderLockRefusal::BatchChanged));
        }
        // What the queue's recheck saw last before the POST; none is a fault.
        let aimed = aim
            .lock()
            .ok()
            .and_then(|mut aimed| aimed.take())
            .ok_or_else(|| refused(UnderLockRefusal::PreImageUnrecorded))?;
        let pre_image = PreImage::new(aimed.pre_image)
            .map_err(|_| refused(UnderLockRefusal::PreImageUnrecorded))?;
        // Every refusal the journal would raise for this intent, decided against
        // the journal just read under this lock, before the approval is spent
        // and before any sidecar is written: a refused intent leaves the
        // approval for the person's next decision and nothing on disk.
        journal
            .check_intent_admissible(&current.batch, &aimed.marks)
            .map_err(|error| match journal_refusal(error) {
                Ok(refusal) => refused(refusal),
                Err(code) => Other(code),
            })?;
        // Spent here, once, under this lock and before the intent: a revoked
        // approval (the call was cancelled) or a second redemption writes no
        // intent (#725).
        self.post_approvals
            .spend(&batch.batch_id, approval)
            .map_err(refused)?;
        let written = write_pre_image(data_dir, &lock, &current.batch, &pre_image)
            .map_err(|_| refused(UnderLockRefusal::PreImageWriteFailed))?;
        let intent = MasterDispatchIntent::new(
            &current.batch,
            request.sha256().to_string(),
            written,
            aimed.marks,
        );
        let appended = intent
            .map_err(|_| MasterLedgerError::Invalid(super::master_ledger::Invalid::BadDigest))
            .and_then(|intent| append_intent(data_dir, &lock, intent));
        appended.map_err(|error| match journal_refusal(error) {
            // Refused by the journal before any byte of the intent was written
            // (the pre-check above makes this a fault, not an expected path):
            // the sidecar this call wrote names no intent, so it is removed
            // while the lock is held. A removal that fails is reported.
            Ok(refusal) => match remove_pre_image(data_dir, &lock, &batch.batch_id) {
                Ok(()) => refused(refusal),
                Err(_) => refused(UnderLockRefusal::PreImageRemoveFailed),
            },
            // The intent may be in the file: the batch is treated as sent, and
            // the sidecar its digest names is kept.
            Err(code) => Other(code),
        })
    }

    /// The answer for a batch the journal shows dispatched. A batch with a
    /// terminal verdict is reported from the journal alone. Any other is read
    /// back and judged again (a reconcile): the same function as after the
    /// POST, which reads and never sends. Where the read-back cannot be
    /// attempted at all (the company cannot be verified, or no longer matches
    /// the batch) the journal's account is given, with the reason.
    async fn reconcile_master_batch(
        &self,
        state: &BatchState,
        mut evidence: Evidence,
    ) -> ToolOutcome {
        if state.is_settled() {
            return Self::journal_report(state, evidence, None);
        }
        let started = std::time::Instant::now();
        // A read-back that can say "absent" must not run while another process
        // is dispatching on this endpoint: its POST of this very batch could
        // still be on the way to the wire. The shared observation lease an
        // active dispatch excludes is held for the whole read-back; without it
        // nothing is read and the outcome stays unconfirmed.
        let _observation = match dispatch_lease::acquire_observation(&self.settings.endpoint) {
            Ok(lease) => lease,
            Err(code) => {
                return match self.unconfirmed_while_dispatch_is_busy(state, &code) {
                    Ok(payload) => ToolOutcome {
                        payload,
                        evidence,
                        company_guid: Some(state.batch.company.guid.clone()),
                        truncated: false,
                    },
                    Err(code) => Self::journal_report(state, evidence, Some(code)),
                };
            }
        };
        let verified = match self.verified_company(&state.batch.company.guid).await {
            Ok((company, identity, identity_evidence)) => {
                evidence = combine_evidence(evidence, identity_evidence);
                if company.name != state.batch.company.name
                    || identity.company_number() != state.batch.company.number
                    || identity.books_from_yyyymmdd() != state.batch.company.books_from
                    || !identity
                        .company_guid()
                        .eq_ignore_ascii_case(&state.batch.company.guid)
                {
                    Err("company_identity_mismatch".to_string())
                } else {
                    Ok(identity)
                }
            }
            Err(failure) => Err(failure.code),
        };
        let payload = match verified {
            Ok(identity) => {
                self.verify_master_batch(
                    state,
                    Moment::Reconcile,
                    &identity,
                    started,
                    &mut evidence,
                    PostedAnswer::recorded(state),
                )
                .await
            }
            Err(code) => Err(code),
        };
        match payload {
            Ok(payload) => ToolOutcome {
                payload,
                evidence,
                company_guid: Some(state.batch.company.guid.clone()),
                truncated: false,
            },
            Err(code) => Self::journal_report(state, evidence, Some(code)),
        }
    }

    /// The journal's own account of a dispatched batch. `not_read_back` is the
    /// code of the read-back that could not be attempted, when there was one.
    fn journal_report(
        state: &BatchState,
        evidence: Evidence,
        not_read_back: Option<String>,
    ) -> ToolOutcome {
        let verdict = state.verdict.as_ref();
        let reported = if let Some(verdict) = verdict {
            verdict_state_code(verdict.state())
        } else if state.response.is_some() {
            "posted_unverified"
        } else {
            "reconciliation_required"
        };
        let mut result = json!({
            "batch_id": state.batch.batch_id,
            "state": reported,
            "attempt_recorded": true,
            "dispatch": {"state": "dispatched", "resent": false},
            "journal": {
                "response_recorded": state.response.is_some(),
                "counters": counters_json(
                    state.response.as_ref().and_then(|response| response.counters.as_ref())
                ),
                "verdict": verdict.map(|verdict| verdict_state_code(verdict.state())),
            },
            // The verdict first: it is what the read-back concluded, and a
            // verdict with no answer on file is not "may have been sent".
            "note": match (verdict, &state.response) {
                (Some(verdict), _) => settled_note(verdict.state()),
                (None, Some(_)) => ALREADY_SENT_NOTE,
                (None, None) => MAY_HAVE_BEEN_SENT_NOTE,
            },
        });
        if let Some(verdict) = verdict {
            if let Some(alert) = settled_alert(verdict.state()) {
                result["alert"] = json!(alert);
            }
        }
        // A book that was not read back says what stopped it and what to do:
        // the verdict, when there is one, was judged by an earlier call.
        match (&not_read_back, verdict) {
            (Some(code), _) => {
                result["next_step"] = json!(not_read_back_next_step(code));
                result["verification"] = json!({"attempted": false, "error": {"code": code}});
            }
            (None, Some(verdict)) => {
                result["next_step"] = json!(settled_next_step(verdict.state()))
            }
            (None, None) => {}
        }
        ToolOutcome {
            payload: json!({ "result": result }),
            evidence,
            company_guid: Some(state.batch.company.guid.clone()),
            truncated: false,
        }
    }
}

/// How the master journal's refusal of a dispatch intent is told apart: `Ok` for
/// one raised before any byte of the intent was written (nothing is recorded),
/// `Err` with its code for one after which the intent may be in the file.
fn journal_refusal(error: MasterLedgerError) -> Result<UnderLockRefusal, String> {
    match error {
        MasterLedgerError::OpenDispatchOverlaps => Ok(UnderLockRefusal::MasterOverlapsOpenDispatch),
        MasterLedgerError::DuplicateDispatch => Ok(UnderLockRefusal::AlreadyAttempted),
        MasterLedgerError::Invalid(_)
        | MasterLedgerError::SerializationFailed
        | MasterLedgerError::RecordTooLarge
        | MasterLedgerError::UseAppendIntent => Ok(UnderLockRefusal::MasterJournalRefused),
        MasterLedgerError::Unavailable
        | MasterLedgerError::RollbackFailed
        | MasterLedgerError::DirectorySyncFailed => Err(error.code().to_string()),
    }
}

/// The refusal code of a dispatch that an unsettled batch's ledger name blocks.
const OVERLAP_CODE: &str = MasterLedgerError::OpenDispatchOverlaps.code();

/// What a caller does after the answer Tally gave could not be followed by a
/// read-back: a later call reads and sends nothing, and nothing is keyed meanwhile.
fn fallback_next_step(answer_recorded: bool) -> String {
    if answer_recorded {
        NEXT_NOT_READ_BACK_AFTER_ANSWER.to_string()
    } else {
        format!("{NEXT_NOT_READ_BACK_AFTER_ANSWER}{NEXT_ANSWER_NOT_RECORDED}")
    }
}

/// The batches that hold a name, as a refusal tells them: those of the refused
/// batch's own company by id, and those of other companies with the company
/// they belong to, so the person can load that company and read that batch
/// back. The list is not capped, for this reason: the journal admits an open
/// dispatch only when it overlaps no other open dispatch
/// (`MasterJournal::intent_refusal`, on every append and on every read). A
/// batch's name is ASCII (`MasterName`), and for such a name an open dispatch
/// can overlap it in three ways: by the canonical key its intent recorded, by
/// the absence key it recorded, or by its name (for ASCII names, equal letters
/// and digits). Open dispatches that agree on one of these overlap each other,
/// so at most one is listed per way, three in all, and one for a journal this
/// build wrote, whose keys are computed from the names. A fourth holder would
/// have to agree with the new name by one of the three ways and with none of
/// the other three holders, which two holders that agree by the same way
/// already rule out; no journal the reader admits has one.
#[derive(Debug, PartialEq, Eq)]
struct BlockerReport {
    ids: Vec<String>,
    other_companies: Vec<BlockingBatch>,
}

impl BlockerReport {
    fn of(blockers: &[BlockingBatch], company_guid: &str) -> Self {
        let (own, others): (Vec<_>, Vec<_>) = blockers
            .iter()
            .cloned()
            .partition(|blocker| blocker.company_guid.eq_ignore_ascii_case(company_guid));
        Self {
            ids: own.into_iter().map(|blocker| blocker.batch_id).collect(),
            other_companies: others,
        }
    }

    fn add_to(&self, error: &mut Value) {
        error["blocking_batch_ids"] = json!(self.ids);
        error["blocking_batches_other_companies"] = json!(self
            .other_companies
            .iter()
            .map(|blocker| json!({"batch_id": blocker.batch_id, "company_guid": blocker.company_guid}))
            .collect::<Vec<_>>());
    }
}

/// What the check under the lock read of the journal when it refused a batch
/// (#2083): whether a send of the batch was on file, and what holds its name.
/// The failure answer reads the journal again, and falls back to this only
/// when that read fails.
#[derive(Debug)]
struct UnderLockView {
    attempted: bool,
    overlap: OverlapView,
}

impl UnderLockView {
    fn of(journal: &MasterJournal, batch: &MasterBatch) -> Self {
        Self {
            attempted: journal.dispatched(&batch.batch_id),
            overlap: overlap_view(Some(journal), &batch.batch_id, &batch.company.guid),
        }
    }
}

/// The message of a failure answer made from the journal the check under the
/// lock read (`kept`), when it says no send of the batch was on file: another
/// call may have recorded one since, so it says only what is true either way,
/// that this call sent nothing, and what the journal held at the refusal
/// (#2083). Every other answer, and every field, is left as it is.
fn as_of_refusal(mut outcome: ToolOutcome, kept: bool, held_name: bool) -> ToolOutcome {
    let result = &mut outcome.payload["result"];
    if kept && result["attempt_recorded"] == false {
        result["error"]["message"] = json!(if held_name {
            KEPT_HELD_NAME_MESSAGE
        } else {
            KEPT_NOT_SENT_MESSAGE
        });
    }
    outcome
}

const KEPT_NOT_SENT_MESSAGE: &str = "This call sent nothing. When it was refused, no send of this batch was on file. Review the error before requesting approval again.";
const KEPT_HELD_NAME_MESSAGE: &str = "This call sent nothing. When it was refused, an earlier create held this ledger name and no send of this batch was on file.";

/// What a read of the journal showed of a batch refused for its ledger name.
#[derive(Debug, PartialEq, Eq)]
enum OverlapView {
    /// Batches hold the name.
    Held(BlockerReport),
    /// The journal was read and no batch holds the name now.
    Settled,
    /// The journal was read and shows a send of this batch recorded since the
    /// call began, by another call.
    SendRecordedSince,
    /// The journal could not be read.
    Unread,
}

impl OverlapView {
    /// The name of the case, in the answer.
    fn code(&self) -> &'static str {
        match self {
            Self::Held(_) => "held",
            Self::Settled => "settled_meanwhile",
            Self::SendRecordedSince => "send_of_this_batch_recorded_since",
            Self::Unread => "journal_unread",
        }
    }
}

/// What `journal` shows of `batch_id` refused for its ledger name. `None` is a
/// journal that could not be read.
fn overlap_view(
    journal: Option<&MasterJournal>,
    batch_id: &str,
    company_guid: &str,
) -> OverlapView {
    let Some(current) = journal.and_then(|journal| journal.state(batch_id)) else {
        return OverlapView::Unread;
    };
    if current.intent.is_some() {
        return OverlapView::SendRecordedSince;
    }
    let held = journal
        .map(|journal| journal.open_dispatch_blockers(&current.batch))
        .unwrap_or_default();
    if held.is_empty() {
        OverlapView::Settled
    } else {
        OverlapView::Held(BlockerReport::of(&held, company_guid))
    }
}

const OVERLAP_HOLD: &str = "A create ComplyEaze Bridge sent earlier for this ledger name, or for a name that may be the same ledger, is not settled. An unsettled create holds its name in every company this data folder holds batches for. Nothing was sent.";
const OVERLAP_OWN: &str = " Call post_ledger_creates with each batch id in blocking_batch_ids: it reads the book back and sends nothing.";
const OVERLAP_OTHER: &str = " A batch of another company holds the name too: blocking_batches_other_companies lists each with its company_guid. list_companies shows only the companies loaded in Tally, so a company_guid you cannot find there belongs to a company that is not loaded. Call post_ledger_creates with that batch_id: if its company is loaded it reads that book back and sends nothing, and if not the answer says to load the same companies and call again.";
/// The step when a send of this batch was recorded since the call began.
const NEXT_SEND_RECORDED_SINCE: &str = "Call post_ledger_creates with this batch_id again later: it reads the book back and sends nothing. Do not create the ledger in Tally by hand meanwhile, because the send may still be waiting in Tally and would overwrite it. Never post this batch again.";
const OVERLAP_THEN: &str = " When those batches are settled, call post_ledger_creates with this batch_id again. Do not create the ledger in Tally by hand meanwhile: the earlier create may still be waiting in Tally and would overwrite it.";
const OVERLAP_SETTLED: &str = "Nothing was sent. The create that held this ledger name has been settled since this call began. Call post_ledger_creates with this batch_id again to post it.";
const OVERLAP_UNREAD: &str = "Nothing was sent by this call. ComplyEaze Bridge could not read its journal just now to say which batch holds this ledger name. Call post_ledger_creates with this batch_id again: it names the batches, or, if this batch was sent by another call, reads it back and sends nothing. Do not create the ledger in Tally by hand meanwhile: an earlier create may still be waiting in Tally and would overwrite it.";

/// What a caller does after the refusal for a held name, by what the journal
/// showed.
fn overlap_next_step(view: &OverlapView) -> String {
    match view {
        OverlapView::Held(held) => {
            let mut step = OVERLAP_HOLD.to_string();
            if !held.ids.is_empty() {
                step.push_str(OVERLAP_OWN);
            }
            if !held.other_companies.is_empty() {
                step.push_str(OVERLAP_OTHER);
            }
            step.push_str(OVERLAP_THEN);
            step
        }
        OverlapView::Settled => OVERLAP_SETTLED.to_string(),
        OverlapView::SendRecordedSince => NEXT_SEND_RECORDED_SINCE.to_string(),
        OverlapView::Unread => OVERLAP_UNREAD.to_string(),
    }
}

/// The answer to a dispatch refused for its ledger name, before the person is
/// asked or under the journal's lock. This call sent nothing in any case; what
/// the journal shows decides the rest.
fn overlap_refusal_outcome(
    batch_id: &str,
    company_guid: &str,
    accumulated: Evidence,
    view: &OverlapView,
) -> ToolOutcome {
    // A send of this batch recorded since is the answer the check under the lock
    // gives a batch already attempted, not the answer for a held name.
    let code = match view {
        OverlapView::SendRecordedSince => UnderLockRefusal::AlreadyAttempted.code(),
        _ => OVERLAP_CODE,
    };
    let mut evidence = accumulated;
    evidence.state = "partial";
    evidence.reason_code = Some(code.to_string());
    let (attempted, dispatch, message) = match view {
        OverlapView::Held(_) => (
            json!(false),
            "not_dispatched",
            "Nothing was sent. An earlier create holds this ledger name.",
        ),
        OverlapView::Settled => (
            json!(false),
            "not_dispatched",
            "Nothing was sent. The create that held this ledger name has been settled since.",
        ),
        OverlapView::SendRecordedSince => (
            json!(true),
            "reconciliation_required",
            "This call sent nothing. A send of this batch has been recorded since by another call; it may have been sent.",
        ),
        // The typed refusal means this call recorded nothing, and `dispatch`
        // says so; whether another call recorded a send is not known, so
        // `attempt_recorded` is not claimed either way.
        OverlapView::Unread => (
            Value::Null,
            "not_dispatched",
            "This call sent nothing. The journal could not be read just now.",
        ),
    };
    let mut error = json!({
        "code": code,
        "message": message,
        "hold": view.code(),
        "next_step": overlap_next_step(view),
    });
    if let OverlapView::Held(held) = view {
        held.add_to(&mut error);
    }
    ToolOutcome {
        payload: json!({"result":{
            "batch_id": batch_id,
            "attempt_recorded": attempted,
            "dispatch": {"state": dispatch, "resent": false},
            "error": error,
        }}),
        evidence,
        company_guid: Some(company_guid.to_string()),
        truncated: false,
    }
}

/// What a caller does after a refusal that has a remedy of its own, where the
/// generic message would send them to the wrong one.
fn master_next_step(code: &str) -> Option<&'static str> {
    match code {
        "master_ledger_invalid" | "master_pre_image_write_failed" | "master_pre_image_unrecorded" => Some(
            "Nothing was sent and nothing was recorded for this batch. Call prepare_ledger_creates for a new batch and ask for approval again.",
        ),
        "master_approval_timed_out" => Some(
            "Nothing was sent: the approval box closed unanswered when its time ran out. To create the ledger, call post_ledger_creates with this batch_id again; that call opens a new approval box.",
        ),
        "master_review_too_large" => Some(
            "Nothing was sent: the approval box's text was over its limit, or on Windows the box did not fit the screen whole. A larger or less scaled screen helps with the second; then call post_ledger_creates with this batch_id again. If it is refused the same way, call prepare_ledger_creates for a new batch with a shorter ledger name.",
        ),
        "master_approval_declined" => Some(
            "Nothing was sent: the approval was declined. If the ledger should still be created, call post_ledger_creates with this batch_id again; that call opens a new approval box.",
        ),
        "master_pre_image_remove_failed" => Some(
            "Nothing was sent. A working file of ComplyEaze Bridge for this batch could not be removed from its local folder; do not retry this batch. Call prepare_ledger_creates for a new batch.",
        ),
        _ => None,
    }
}

/// The refusal a name check's typed error becomes.
fn name_check_failure(error: &anyhow::Error) -> ToolFailure {
    let code = refusal_code(error);
    let group = error.chain().find_map(|cause| {
        match cause.downcast_ref::<ApprovedImportAdmissionError>() {
            Some(ApprovedImportAdmissionError::GroupExportInvalid { cause }) => *cause,
            Some(ApprovedImportAdmissionError::MasterLedgerIndexUnavailable { cause }) => {
                Some(*cause)
            }
            _ => None,
        }
    });
    let mut failure = ToolFailure::from(code.to_string());
    failure.cause = refusal_cause(code, group, None);
    failure
}

/// An Education profile is refused before the person is asked, as the queue
/// refuses it again before the send: no master write has been measured in it.
fn require_licensed_mode(profile: DateBoundaryProfile) -> Result<(), ToolFailure> {
    match profile {
        DateBoundaryProfile::ModeAgnostic => Ok(()),
        DateBoundaryProfile::EducationRestricted => {
            Err("master_writes_need_licensed_mode".to_string().into())
        }
    }
}

/// What a failed post is answered with. `attempted` is what the journal showed:
/// a recorded intent means the create may have been sent and must be
/// reconciled, never repeated; `None` is a journal that could not be read, so
/// whether a send was recorded is not known.
fn master_failure_outcome(
    batch_id: &str,
    company_guid: &str,
    failure: ToolFailure,
    accumulated: Evidence,
    attempted: Option<bool>,
) -> ToolOutcome {
    let mut evidence = failure
        .evidence
        .map(|item| combine_evidence(accumulated.clone(), *item))
        .unwrap_or(accumulated);
    evidence.state = "partial";
    evidence.reason_code = Some(failure.code.clone());
    let mut error = json!({"code": failure.code, "message": match attempted {
        Some(false) => "Nothing was sent. Review the error before requesting approval again.",
        Some(true) => "The create may have been sent to Tally. Do not send this batch again: check the company's ledgers in Tally.",
        None => "Whether the create was sent to Tally is not known: the journal could not be read just now. Check the company's ledgers in Tally.",
    }});
    if let Some(cause) = failure.cause {
        error["cause"] = json!(cause);
    }
    if failure.code == "tally_endpoint_busy" {
        error["retry_after_s"] = json!(bridge_tally_transport::WIRE_BUSY_RETRY_AFTER.as_secs());
    }
    match attempted {
        Some(false) => {
            if let Some(next_step) = master_next_step(&failure.code) {
                error["next_step"] = json!(next_step);
            }
        }
        // A send is recorded: a later call reads the book back and sends
        // nothing, and nothing is keyed by hand meanwhile.
        Some(true) => error["next_step"] = json!(NEXT_UNCONFIRMED),
        // Not known: a later call is true either way, and says which.
        None => error["next_step"] = json!(NEXT_SEND_NOT_KNOWN),
    }
    ToolOutcome {
        payload: json!({"result":{
            "batch_id": batch_id,
            "attempt_recorded": attempted,
            "dispatch": {
                "state": if attempted == Some(false) { "not_dispatched" } else { "reconciliation_required" },
                "resent": false,
            },
            "error": error,
        }}),
        evidence,
        company_guid: Some(company_guid.to_string()),
        truncated: false,
    }
}

#[cfg(test)]
#[path = "agent_master_tools_tests.rs"]
mod tests;
