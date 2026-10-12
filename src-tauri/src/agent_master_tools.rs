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
//! After the POST this piece appends the [`MasterDispatchResponse`] and answers
//! `posted_unverified`. It does not read the book back, so it never says the
//! ledger is verified; the read-back and the verdict are the next piece.
use super::approval::{ApprovalBinding, HeldRequest};
use super::master_ledger::{
    append, append_intent, read_journal, remove_pre_image, write_pre_image, BatchState,
    ExclusiveAdmission, MasterBatch, MasterClass, MasterCompany, MasterCounters,
    MasterDispatchIntent, MasterDispatchResponse, MasterJournal, MasterLedgerError, MasterRecord,
    PreImage, RecordedIntent, StoredLedgerCreate, StoredLedgerKind,
};
use super::master_post::{
    check_name_free, recheck_master_for_dispatch, MasterAim, MasterPostRequest,
};
use super::master_review::master_review_text;
use super::post::{refusal_cause, refusal_code, ApprovalTarget, Entry, Pass, Step};
use super::*;
use crate::agent::evidence_from_runtime_read;
use crate::agent::read_profiles::{ledger_name_index_read, native_group_snapshot_read};
use crate::tally::approved_import::{
    ApprovedImportAdmissionError, BeforeDispatchError, PendingPostApproval, PostKind,
    UnderLockRefusal,
};
use crate::tally::ledger_create_item::{
    AdmittedDutyHead, AdmittedLedgerKind, GstinError, LedgerCreateItem, MasterName,
    MasterNameError, PartySide, VerifiedGstin,
};
use bridge_tally_protocol::GstDutyHead;
use std::sync::Mutex;

/// What a caller is told after a ledger is prepared.
const PREPARED_NEXT_STEP: &str = "Nothing was written to Tally. To create this ledger, call post_ledger_creates with this batch_id: the local user must approve it in a native dialog first.";

/// What a caller is told after the one POST. The ledger is not read back yet.
const POSTED_UNVERIFIED_NOTE: &str = "Tally answered the one create and ComplyEaze Bridge recorded the answer, but has not read the ledger back: it is NOT verified. Check the ledger in Tally before relying on it. Never post this batch again.";

/// What a caller is told when a batch has a recorded intent and no recorded
/// answer: the create may or may not have reached Tally.
const MAY_HAVE_BEEN_SENT_NOTE: &str = "This batch may have been sent to Tally once: an intent to send it is recorded, and no answer from Tally is. Nothing was sent now and nothing will be sent again from this batch. The ledger is NOT verified: check the company's ledgers in Tally.";

/// What a caller is told when a batch was sent and Tally's answer is recorded.
const ALREADY_SENT_NOTE: &str = "This batch was sent to Tally once and its answer is recorded. Nothing was sent now and nothing will be sent again from this batch. This is what ComplyEaze Bridge recorded; the ledger is NOT verified.";

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

/// The one item of a stored batch, parsed through the same constructors as a
/// caller's input: a record that no longer parses as an admitted ledger is
/// refused, not sent.
fn item_from_batch(batch: &MasterBatch) -> Result<LedgerCreateItem, String> {
    let refused = || "master_batch_invalid".to_string();
    let [stored] = batch.items.as_slice() else {
        return Err(refused());
    };
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

/// Enforces the published `inputSchema` on the nested `ledger` object, so its
/// bounds live in one place.
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
    if let Some(value) = args.get("ledger") {
        super::super::catalog::validate_against_schema(
            value,
            &schema["properties"]["ledger"],
            "ledger",
        )?;
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
    }
}

impl Server {
    /// The masters journal as it stands, read under the shared admission lock.
    fn read_master_journal(&self) -> Result<MasterJournal, String> {
        let _lock = self.lock_import_admission_shared()?;
        read_journal(&self.settings.data_dir).map_err(|error| error.code().to_string())
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
        Ok(ToolOutcome {
            payload: json!({"result":{
                "batch_id":batch_id,"attempt_recorded":attempted,
                "dispatch":{"state":if attempted == Some(false) { "not_dispatched" } else { "reconciliation_required" },"resent":false},
                "error":{"code":"request_cancelled","message":if attempted == Some(false) {
                    "The request was cancelled. No send is recorded for this prepared ledger."
                } else {
                    "The request was cancelled with an uncertain outcome. The batch may have been sent: never send it again."
                }}
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
        let item = parse_ledger_item(
            args.get("ledger")
                .ok_or_else(|| "ledger_required".to_string())?,
        )?;
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
        check_name_free(&ledgers, &groups, &item, identity.company_guid())
            .map_err(|error| name_check_failure(&error))?;
        let request = MasterPostRequest::render(item.clone(), &identity);
        let preview = master_review_text(&item, &identity, &origin)
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
            items: vec![stored_from_item(&item)],
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
                "parent": item.parent(),
                "next_step": PREPARED_NEXT_STEP,
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
                    self.reconcile_master_batch(&state, accumulated.clone()),
                )));
            }
            let item = item_from_batch(&batch)?;
            let target = ApprovalTarget {
                tool: "post_ledger_creates",
                batch_id,
                guid: &guid,
                vouchers: 1,
            };
            // A dialog or approval an earlier call left for this batch (#725).
            let redeeming = match self
                .begin_agent_approval(&target, redeem_only, call_started, &accumulated)
                .await?
            {
                Ok(redeeming) => redeeming,
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
            let request = MasterPostRequest::render(item.clone(), &identity);
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
            check_name_free(&ledgers, &groups, &item, identity.company_guid())
                .map_err(|error| name_check_failure(&error))?;
            let preview = master_review_text(&item, &identity, &origin)
                .map_err(|error| ToolFailure::from(error.code().to_string()))?;
            let company_marks_request = crate::tally::agent_read_request::AgentReadRequest::parse(
                super::super::read_profiles::render_agent_company_high_water(&company.name),
            )
            .map_err(|error| error.to_string())?;
            let binding =
                ApprovalBinding::masters(&batch.batch_sha256, &origin, &identity, &item, &preview);
            if !redeeming {
                let asked = PendingPostApproval::ask_masters(
                    item.clone(),
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
                    || self.record_master_intent(&batch, &request, taken.id(), &aim),
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
            {
                // Known limit, shared with the voucher path's response append:
                // the lock is tried once, never waited for, so another process
                // holding it at this instant fails the append. The send is
                // already recorded by its intent; only Tally's answer goes
                // unrecorded, and the batch is then reported for reconciliation.
                let lock = ExclusiveAdmission::acquire(&self.settings.data_dir)?;
                append(
                    &self.settings.data_dir,
                    &lock,
                    MasterRecord::DispatchResponse(response.clone()),
                )
                .map_err(|error| error.code().to_string())?;
            }
            Ok(Step::Done(Box::new(ToolOutcome {
                payload: json!({"result":{
                    "batch_id": batch.batch_id,
                    "state": "posted_unverified",
                    "attempt_recorded": true,
                    "dispatch": {"state": "dispatched", "resent": false},
                    "counters": counters_json(response.counters.as_ref()),
                    "note": POSTED_UNVERIFIED_NOTE,
                }}),
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
                // an unreadable journal is not "not sent".
                let attempted = self
                    .read_master_journal()
                    .ok()
                    .map(|journal| journal.dispatched(batch_id));
                Ok(Pass::Done(master_failure_outcome(
                    batch_id,
                    &guid,
                    failure,
                    accumulated,
                    attempted,
                )))
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
    ) -> Result<RecordedIntent, BeforeDispatchError> {
        use BeforeDispatchError::{Other, Refused};
        let data_dir = &self.settings.data_dir;
        let lock = ExclusiveAdmission::acquire(data_dir).map_err(Other)?;
        let journal = read_journal(data_dir)
            .map_err(|_| Refused(UnderLockRefusal::MasterJournalUnavailable))?;
        let current = journal
            .state(&batch.batch_id)
            .ok_or(Refused(UnderLockRefusal::BatchNotFound))?;
        if current.intent.is_some() {
            return Err(Refused(UnderLockRefusal::AlreadyAttempted));
        }
        if current.batch.batch_sha256 != batch.batch_sha256 || current.batch != *batch {
            return Err(Refused(UnderLockRefusal::BatchChanged));
        }
        // What the queue's recheck saw last before the POST; none is a fault.
        let aimed = aim
            .lock()
            .ok()
            .and_then(|mut aimed| aimed.take())
            .ok_or(Refused(UnderLockRefusal::PreImageUnrecorded))?;
        let pre_image = PreImage::new(aimed.pre_image)
            .map_err(|_| Refused(UnderLockRefusal::PreImageUnrecorded))?;
        // Every refusal the journal would raise for this intent, decided against
        // the journal just read under this lock, before the approval is spent
        // and before any sidecar is written: a refused intent leaves the
        // approval for the person's next decision and nothing on disk.
        journal
            .check_intent_admissible(&current.batch, &aimed.marks)
            .map_err(|error| match journal_refusal(error) {
                Ok(refusal) => Refused(refusal),
                Err(code) => Other(code),
            })?;
        // Spent here, once, under this lock and before the intent: a revoked
        // approval (the call was cancelled) or a second redemption writes no
        // intent (#725).
        self.post_approvals
            .spend(&batch.batch_id, approval)
            .map_err(Refused)?;
        let written = write_pre_image(data_dir, &lock, &current.batch, &pre_image)
            .map_err(|_| Refused(UnderLockRefusal::PreImageWriteFailed))?;
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
                Ok(()) => Refused(refusal),
                Err(_) => Refused(UnderLockRefusal::PreImageRemoveFailed),
            },
            // The intent may be in the file: the batch is treated as sent, and
            // the sidecar its digest names is kept.
            Err(code) => Other(code),
        })
    }

    /// The answer for a batch the journal shows dispatched: read-only, from the
    /// journal alone. Nothing is sent and nothing is read from Tally; the
    /// reconcile that reads the book back is the next piece.
    fn reconcile_master_batch(&self, state: &BatchState, evidence: Evidence) -> ToolOutcome {
        let verdict = state.verdict.as_ref();
        let reported = if let Some(verdict) = verdict {
            verdict_state_code(verdict.state)
        } else if state.response.is_some() {
            "posted_unverified"
        } else {
            "reconciliation_required"
        };
        ToolOutcome {
            payload: json!({"result":{
                "batch_id": state.batch.batch_id,
                "state": reported,
                "attempt_recorded": true,
                "dispatch": {"state": "dispatched", "resent": false},
                "journal": {
                    "response_recorded": state.response.is_some(),
                    "counters": counters_json(
                        state.response.as_ref().and_then(|response| response.counters.as_ref())
                    ),
                    "verdict": verdict.map(|verdict| verdict_state_code(verdict.state)),
                },
                "note": if state.response.is_some() { ALREADY_SENT_NOTE } else { MAY_HAVE_BEEN_SENT_NOTE },
            }}),
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

/// What a caller does after a refusal that has a remedy of its own, where the
/// generic message would send them to the wrong one.
fn master_next_step(code: &str) -> Option<&'static str> {
    match code {
        "master_ledger_open_dispatch_overlaps" => Some(
            "Another batch for the same ledger name was sent and its outcome is not settled. Nothing was sent. Check the company's ledgers in Tally, then call prepare_ledger_creates for a new batch; this one cannot be sent.",
        ),
        "master_ledger_invalid" | "master_pre_image_write_failed" | "master_pre_image_unrecorded" => Some(
            "Nothing was sent and nothing was recorded for this batch. Call prepare_ledger_creates for a new batch and ask for approval again.",
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
/// reconciled, never repeated.
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
    let mut error = json!({"code": failure.code, "message": if attempted == Some(false) {
        "Nothing was sent. Review the error before requesting approval again."
    } else {
        "The create may have been sent to Tally. Do not send this batch again: check the company's ledgers in Tally."
    }});
    if let Some(cause) = failure.cause {
        error["cause"] = json!(cause);
    }
    if failure.code == "tally_endpoint_busy" {
        error["retry_after_s"] = json!(bridge_tally_transport::WIRE_BUSY_RETRY_AFTER.as_secs());
    }
    if attempted == Some(false) {
        if let Some(next_step) = master_next_step(&failure.code) {
            error["next_step"] = json!(next_step);
        }
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
