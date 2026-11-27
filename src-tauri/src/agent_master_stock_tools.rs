//! `prepare_stock_item` (#1381): the stock twin of `prepare_ledger_creates`.
//! Like the ledger pair, both stock tools are listed only while master writes
//! are in force, and refuse `master_writes_disabled` otherwise.
//!
//! It verifies the company, reads the company's stock reads once in one queued
//! execution, builds the stock-item draft from those reads, builds the preview
//! from the same draft and records the batch in the stock journal. It asks no
//! one and writes nothing to Tally: as for a ledger, the ask and the hold come
//! with the post.
//!
//! Under `mask_parties` the returned preview masks the item's name, its
//! sibling's source name, its parent and its HSN description, as a stock read
//! masks a stock item's name, parent and HSN description (a person can edit
//! that description). The base unit stays whole because unit names are
//! configuration labels, and the HSN code and the rates stay whole because they
//! are not names. Each is masked with `mask`, since the masters read's own
//! marking works on a JSON field, not a plain string. The recorded batch is the
//! same either way.
//!
//! The answer's evidence is the company-list read of the identity check only:
//! the stock reads keep none, and no declared reason code says so.
//!
//! `post_stock_item` is the stock twin of the ledger post's pass. It reads the
//! batch from the stock journal, verifies the company, shows the item again
//! from the book as it stands through the same read as above, refuses unless the batch that item makes is the stored one,
//! and refuses a name an open stock dispatch holds. Then it asks and holds the
//! person's approval as the ledger post does, takes it, and sends the one
//! Create through the shared queue with the stock recheck and the stock intent
//! writer. Then, as the ledger post does, it records Tally's answer, reads the
//! book back through the same read, judges it (the stock journal's `verdict`
//! module holds the table), records the verdict and answers by it. A later call
//! on a batch whose send is recorded reads back and judges the same way, under
//! the observation lease, and never verifies; a settled batch is answered from
//! the journal alone. A send that failed is answered with no read.
use super::super::master_ledger::read_stock_item_journal;
use super::super::master_ledger::stock_item_journal::{
    record_stock_item_intent, Answer, StockIntentRefusal, StockItemBatch, StockJudgement,
    StockReadBack,
};
use super::super::master_ledger::{
    JournalBatchState, JournalRecord, MasterVerdict, MasterVerdictState,
};
use super::super::master_post::aim_master_marks;
use super::super::master_post::StockItemRecheckError;
use super::super::master_post::{recheck_stock_item_for_dispatch, stock_company_marks};
use super::super::master_review::{show_stock_item_draft, ShownStockItem};
use super::*;
use crate::agent::failure_code::{
    create_render_refusal_code, draft_refusal_code, item_name_refusal_code, not_goods_refusal_code,
    sibling_refusal_code, stock_item_catalogue_refusal_code, MasterPostCode, StockItemCreateCode,
};
use crate::tally::agent_read_request::AgentReadRequest;
use crate::tally::approved_import::ApprovedImport;
use crate::tally::runtime::{
    AdmittedCandidateRead, AdmittedStockGroups, AdmittedStockUnits, MastersReadError,
};
use crate::tally::stock_item_create::{
    CompleteStockItemCatalogue, GoodsSibling, SiblingGst, StockItemDraft,
};
use crate::tally::stock_item_verified::StockItemImportOutcome;

/// What a caller is told after a stock item is prepared.
const PREPARED_STOCK_NEXT_STEP: &str = "Nothing was written to Tally. To create this stock item, call post_stock_item with this batch_id: the local user must approve it in a native dialog first.";

/// What a caller is told after the one stock-item Create was answered and the
/// book could not be read back at all.
const POSTED_STOCK_NOTE: &str = "Tally answered the one stock item create (any counters it gave are here), but ComplyEaze Bridge has not read the stock item back: it is NOT verified. Check the stock item in Tally before relying on it, and never post this batch again.";
/// Added when Tally's answer could not be recorded either.
const STOCK_ANSWER_UNRECORDED: &str = " Tally's answer is not on file: keep the counters this answer reports, the only record of them.";
/// Added when the verdict could not be recorded.
const STOCK_VERDICT_UNRECORDED: &str = " ComplyEaze Bridge could not record this verdict: a later call for this batch_id reads the book back again and sends nothing.";
const VERIFIED_NOTE: &str = "ComplyEaze Bridge read the book back after the one create: exactly one new stock item of this name, with the parent, unit, GST details and HSN details as sent; no stock item that existed before changed; the company's master mark moved by exactly one; and no other loaded company's master mark moved.";
const VERIFIED_STEP: &str =
    "Nothing more is needed for this stock item. Do not post this batch again.";
/// The read-back matched, but its verdict could not be recorded.
const UNRECORDED_NOTE: &str = "ComplyEaze Bridge read the stock item back and it matched what was sent, but the verdict could not be recorded, so the stock item is NOT verified: the batch stays open, and a later call for this batch_id cannot verify it.";
const UNCONFIRMED_NOTE: &str =
    "The book could not be read back, so the stock item is NOT verified.";
const OPEN_NOTE: &str =
    "The read-back did not settle the outcome (see the reasons): the stock item is NOT verified.";
/// The step for either open state. A later call cannot verify the item.
const OPEN_STEP: &str = "Check the stock item in Tally against the reasons given. A later call for this batch_id reads the book back and sends nothing, but it cannot verify this stock item, and the name stays held meanwhile. Do not create the stock item in Tally by hand, and never post this batch again.";
/// The step after a send of a stock batch, by this call or an earlier one.
const STOCK_NEXT_STEP: &str = "Check the company's stock items in Tally before relying on this one. Do not post this batch again, and do not create the stock item in Tally by hand meanwhile.";
/// A stock batch this call did not send, a send of which is recorded.
const STOCK_SENT_NOTE: &str = "A send of this batch is recorded: it was sent to Tally once, or may have been. Nothing was sent now and nothing will be sent again from this batch. The stock item is NOT verified.";
/// A failed stock post whose journal could not be read afterwards.
const STOCK_UNKNOWN_NOTE: &str = "Whether this batch was sent to Tally is not known: the journal could not be read just now. The stock item is NOT verified.";
/// A stock item name an unsettled stock dispatch holds.
const STOCK_HOLD_NOTE: &str = "Nothing was sent. A stock item create ComplyEaze Bridge sent earlier for this name, or for a name that may be the same stock item, is not settled, and it holds the name in every company this data folder holds stock batches for. Do not create the stock item in Tally by hand meanwhile: the earlier create may still be waiting in Tally.";
/// A stock batch whose send another call recorded after this call began.
const STOCK_SENT_SINCE_NOTE: &str = "This call sent nothing. A send of this batch has been recorded since by another call; it may have been sent. The stock item is NOT verified.";
/// A stock batch the hold check could not find in the journal it read.
const STOCK_HOLD_UNREAD_NOTE: &str = "This call sent nothing. The journal could not be read just now to say which batch holds this stock item name.";
/// The step after a stock item refused for a name an open dispatch holds.
const STOCK_HOLD_NEXT_STEP: &str = "When the stock item creates that hold the name are settled, post this batch again. Do not create the stock item in Tally by hand meanwhile.";
/// The step after a hold check that could not read the journal.
const STOCK_UNREAD_NEXT_STEP: &str = "Post this batch again later: the journal is read again then. Do not create the stock item in Tally by hand meanwhile.";

/// What a stock post withdrawn by its caller is told, by what the stock
/// journal showed of its batch, in the ledger post's three cases.
pub(super) const STOCK_CANCELLED: CancelledTexts = CancelledTexts {
    not_sent: "The request was cancelled. No send is recorded for this prepared stock item.",
    sent: "The request was cancelled after a send of this batch was recorded. The stock item create may have been sent to Tally: never send this batch again.",
    not_known: "The request was cancelled, and whether a send of this batch was recorded is not known: the journal could not be read just now, or holds no such batch. Check the company's stock items in Tally.",
    sent_step: "The stock item has not been read back. Call post_stock_item with this batch_id again once Tally is idle to read it back (it sends nothing), or look at the company's stock items in Tally. Do not create the stock item in Tally by hand meanwhile: the create may still be waiting in Tally. Never post this batch again.",
    // Not known: a later call is the right one either way.
    not_known_step: "Call post_stock_item with this batch_id again. If a send of this batch was recorded, that call sends nothing: it reads the book back, or reports the outcome already recorded; if none was, it asks for approval again before anything is sent; if ComplyEaze Bridge holds no such batch, it is refused as master_batch_not_found and nothing is sent. Do not create the stock item in Tally by hand meanwhile: a create that has not run yet may still be waiting in Tally.",
};

impl Server {
    /// Prepare ONE stock item, a copy of the sibling stock item `sibling` under
    /// `name`, in the company `company_guid`. Nothing is written to Tally.
    pub(in crate::agent) async fn prepare_stock_item(
        &self,
        company_guid: &str,
        name: &str,
        sibling: &str,
    ) -> Result<ToolOutcome, ToolFailure> {
        let name = MasterName::new(name).map_err(|error| master_name_code(error).to_string())?;
        let sibling =
            MasterName::new(sibling).map_err(|error| master_name_code(error).to_string())?;
        let (_, identity, identity_evidence) = self.verified_company(company_guid).await?;
        let origin = self.master_preview_origin()?;
        let shown = self
            .show_from_book(&identity, &name, &sibling, &origin)
            .await?;
        let batch_id = format!("bridge-{}", Uuid::new_v4());
        let batch = (shown.batch(batch_id.clone()))
            .map_err(|error| ToolFailure::from(create_render_refusal_code(&error)))?;
        {
            let lock = ExclusiveAdmission::acquire(&self.settings.data_dir)?;
            append(&self.settings.data_dir, &lock, JournalRecord::Batch(batch))
                .map_err(|error| error.code().to_string())?;
        }
        Ok(ToolOutcome {
            payload: json!({"result":{
                "batch_id": batch_id,
                "preview": if self.settings.redaction == crate::agent::Redaction::MaskParties {
                    shown.masked_preview()
                } else {
                    shown.preview().to_string()
                },
                "next_step": PREPARED_STOCK_NEXT_STEP,
            }}),
            evidence: identity_evidence,
            company_guid: Some(identity.company_guid().to_string()),
            truncated: false,
        })
    }

    /// The stock reads, once, in one queued execution, and the draft of `name`
    /// copied from `sibling` built and shown from them there: the prepare's
    /// read and the post's.
    async fn show_from_book(
        &self,
        identity: &VerifiedCompanyIdentity,
        name: &MasterName,
        sibling: &MasterName,
        origin: &str,
    ) -> Result<ShownStockItem, ToolFailure> {
        let shown = self
            .runtime
            .read_and_recheck_stock_item(
                self.tally_config(),
                identity,
                stock_marks(identity)?,
                stock_company_marks,
                |reads| {
                    // As the ledger prepare, no mark or aim check: the queue rechecks.
                    let (candidate, groups, units, _, _) = reads.into_parts();
                    Ok(show_fresh_draft(
                        &candidate, &groups, &units, name, sibling, origin,
                    ))
                },
            )
            .await
            .map_err(stock_read_failure)?;
        shown.map_err(ToolFailure::from)
    }

    /// Ask the person and, once approved, send the one stock-item Create of
    /// `batch_id`, in at most two passes as the ledger post does. See the
    /// module documentation.
    pub(in crate::agent) async fn post_stock_item(
        &self,
        batch_id: &str,
    ) -> Result<ToolOutcome, ToolFailure> {
        self.master_post_passes(move |entry| self.post_stock_item_pass(batch_id, entry))
            .await
    }

    async fn post_stock_item_pass(
        &self,
        batch_id: &str,
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
        // Refused, as the ledger post refuses, without touching the approval slot.
        if !valid_batch_id(batch_id) {
            return Err(MasterPostCode::BatchIdInvalid.into());
        }
        let journal = self.read_kind_journal_shared(read_stock_item_journal)?;
        let state = (journal.state(batch_id).cloned()).ok_or(MasterPostCode::BatchNotFound)?;
        let batch = state.batch.clone();
        let guid = batch.company().guid.clone();
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
                let reconciled = self.reconcile_stock_batch(&journal, &state, accumulated.clone());
                return Ok(Step::Done(Box::new(reconciled.await)));
            }
            let target = ApprovalTarget {
                tool: "post_stock_item",
                batch_id,
                guid: &guid,
                vouchers: 1,
            };
            let redeeming = match self
                .begin_agent_approval(&target, redeem_only, call_started, &accumulated)
                .await?
            {
                Ok((redeeming, _form)) => redeeming,
                Err(step) => return Ok(step),
            };
            let (company, identity, identity_evidence) = self.verified_company(&guid).await?;
            accumulated = combine_evidence(accumulated.clone(), identity_evidence);
            if !same_company(&company.name, &identity, batch.company()) {
                return Err(MasterPostCode::CompanyIdentityMismatch.into());
            }
            // The stored line was admitted only if it rebuilds.
            let stored = (batch.stored_writes()).map_err(|_| MasterPostCode::BatchChanged)?;
            let origin = self.master_preview_origin()?;
            let shown = self
                .show_from_book(
                    &identity,
                    stored.name(),
                    stored.sibling_source_name(),
                    &origin,
                )
                .await?;
            // The batch the item shown now makes, whole: its digest, its company
            // and every stored write, the sibling's GUID among them.
            let fresh = (shown.batch(batch_id.to_string()))
                .map_err(|error| ToolFailure::from(create_render_refusal_code(&error)))?;
            if fresh != batch {
                return Err(MasterPostCode::BatchChanged.into());
            }
            // A name an open stock dispatch holds is refused before the person
            // is asked; the check under the lock in the writer stays. A read
            // that fails here skips this check, as the ledger post's does.
            if let Ok(journal) = self.read_master_journal_for_hold_check(read_stock_item_journal) {
                let view = overlap_view(Some(&journal), batch_id, &guid);
                if let Some(refused) = stock_hold_refusal(batch_id, &guid, &accumulated, &view) {
                    self.post_approvals.revoke_unredeemed_of(
                        PostKind::Masters,
                        batch_id,
                        "post_refused_before_intent",
                    );
                    return Ok(Step::Done(Box::new(refused)));
                }
            }
            let preview = if self.settings.redaction == crate::agent::Redaction::MaskParties {
                shown.masked_preview()
            } else {
                shown.preview().to_string()
            };
            let binding = shown.binding();
            if !redeeming {
                let asked = shown.ask(stock_marks(&identity)?, batch_id.to_string());
                let (_, dialog, held) = asked.into_parts();
                if let Some(step) = self
                    .ask_and_hold(&target, dialog, &binding, held, call_started, &accumulated)
                    .await?
                {
                    return Ok(step);
                }
            }
            let (taken, approved, held) =
                self.post_approvals.take_for_dispatch(batch_id, &binding)?;
            // Only the fresh item may come back, else the approval is left
            // unspent: what the person was shown is the stored item.
            let HeldRequest::StockItem(held) = held else {
                return Err(MasterPostCode::ApprovalBindingChanged.into());
            };
            if held.prepared().company() != &identity || *held.prepared().writes() != stored {
                return Err(MasterPostCode::ApprovalBindingChanged.into());
            }
            owed_digest(&approved, &batch)?;
            let _endpoint_dispatch_lease = dispatch_lease::acquire(&self.settings.endpoint)?;
            let data_dir = &self.settings.data_dir;
            let posted = self
                .runtime
                .post_approved_import(
                    self.tally_config(),
                    &identity,
                    approved,
                    recheck_stock_item_for_dispatch,
                    |passed| {
                        let lock =
                            ExclusiveAdmission::acquire_or_refuse(data_dir).map_err(|refusal| {
                                BeforeDispatchError::Refused(UnderLockRefusal::Admission(refusal))
                            })?;
                        record_stock_item_intent(data_dir, &lock, &batch, passed, &taken)
                            .map_err(stock_intent_refusal)
                    },
                )
                .await;
            // An approval this post did not use lapses: it is never offered
            // again (#725).
            drop(taken);
            self.drop_listing_snapshots(identity.company_guid());
            let posted = posted.map_err(stock_post_failure)?;
            for read in [&posted.admission_evidence, &posted.response_evidence] {
                accumulated = combine_evidence(
                    accumulated.clone(),
                    evidence_from_runtime_read(read.clone()),
                );
            }
            let sha256 = &posted.response_evidence.response_sha256;
            let mut payload =
                (self.answer_stock_send(&batch, &identity, &posted.body, sha256)).await;
            payload["result"]["preview"] = json!(preview);
            Ok(Step::Done(Box::new(ToolOutcome {
                payload,
                evidence: accumulated.clone(),
                company_guid: Some(guid.clone()),
                truncated: false,
            })))
        }
        .await;
        let failure = match operation {
            Ok(Step::Redeem) => {
                return Ok(Pass::Redeem {
                    evidence: accumulated,
                    call_started,
                })
            }
            Ok(Step::Done(outcome)) => return Ok(Pass::Done(*outcome)),
            Err(failure) => failure,
        };
        // A refused redemption withdraws the approval it was to use (#725).
        self.post_approvals.revoke_unredeemed_of(
            PostKind::Masters,
            batch_id,
            "post_refused_before_intent",
        );
        // Whether a send was recorded decides the answer: an unreadable
        // journal is not "not sent".
        let journal = self.read_kind_journal_shared(read_stock_item_journal).ok();
        match journal.map(|journal| journal.dispatched(batch_id)) {
            Some(false) => Err(failure),
            attempted => Ok(Pass::Done(stock_sent(
                batch_id,
                &guid,
                accumulated,
                &failure.code,
                attempted,
            ))),
        }
    }
}

impl Server {
    /// Whether the stock journal records a send of the batch `args` names:
    /// `Some(false)` is known not sent. Only a busy journal is an error, which
    /// a cancellation waits out; anything else not known is `None`, and the
    /// post then drains rather than be taken as unsent.
    pub(super) fn recorded_stock_attempt(&self, args: &Value) -> Result<Option<bool>, ToolFailure> {
        let Some(batch_id) = args.get("batch_id").and_then(Value::as_str) else {
            return Ok(None);
        };
        match self.read_kind_journal_shared(read_stock_item_journal) {
            Ok(journal) => Ok(journal.state(batch_id).map(|state| state.intent.is_some())),
            Err(error) if error == "import_admission_busy" => Err(error.into()),
            Err(_) => Ok(None),
        }
    }

    /// What the stock journal shows of the batch a cancelled stock post
    /// names: whether a send of it is recorded, and its company's GUID.
    pub(super) fn cancelled_stock_facts(
        &self,
        args: &Value,
    ) -> Result<(Option<bool>, Option<String>), ToolFailure> {
        let batch_id = required_string(args, "batch_id")?;
        let attempted = self.recorded_stock_attempt(args)?;
        let company_guid =
            (self.read_kind_journal_shared(read_stock_item_journal).ok()).and_then(|journal| {
                (journal.state(batch_id)).map(|state| state.batch.company().guid.clone())
            });
        Ok((attempted, company_guid))
    }

    /// After the one Create was answered: the answer recorded once, then the
    /// book read back and judged with it in hand. Else NOT verified.
    async fn answer_stock_send(
        &self,
        batch: &StockItemBatch,
        identity: &VerifiedCompanyIdentity,
        body: &str,
        response_sha256: &str,
    ) -> Value {
        let outcome = bridge_tally_protocol::parse_import_outcome(body).ok();
        let counters = reported_counters(outcome.as_ref());
        let response = MasterDispatchResponse {
            batch_id: batch.batch_id().to_string(),
            batch_sha256: batch.batch_sha256().to_string(),
            response_sha256: response_sha256.to_string(),
            counters,
        };
        // Tried once here, and once more under the verdict's lock if it failed.
        let data_dir = &self.settings.data_dir;
        let appended = ExclusiveAdmission::acquire(data_dir).and_then(|lock| {
            let record = JournalRecord::<StockItemBatch>::DispatchResponse(response.clone());
            append(data_dir, &lock, record).map_err(|error| error.code().to_string())
        });
        let answer = Answer::Posted(outcome.as_ref().map(StockItemImportOutcome::of));
        let unrecorded = appended.is_err().then_some(&response);
        let code = match self.read_kind_journal_after_answer(read_stock_item_journal) {
            Ok(journal) => match journal.state(batch.batch_id()) {
                Some(state) if state.intent.is_some() => {
                    let verified =
                        self.verify_stock_batch(&journal, state, identity, &answer, unrecorded);
                    return verified.await;
                }
                _ => MasterPostCode::BatchNotFound.as_str().to_string(),
            },
            Err(code) => code,
        };
        let mut result = json!({
            "batch_id": batch.batch_id(),
            "state": "posted_unverified",
            "attempt_recorded": true,
            "answer_recorded": appended.is_ok(),
            "dispatch": {"state": "dispatched", "resent": false},
            "counters": counters_json(counters.as_ref()),
            "verification": {"attempted": false, "error": {"code": code}},
            "note": POSTED_STOCK_NOTE,
            "next_step": STOCK_NEXT_STEP,
        });
        if let Err(code) = &appended {
            result["verification"]["answer_record_error"] = json!(code);
            result["next_step"] = json!(format!("{STOCK_NEXT_STEP}{STOCK_ANSWER_UNRECORDED}"));
        }
        json!({ "result": result })
    }

    /// A later call on a sent batch: a settled one is answered from the journal
    /// alone; an open one is read back under the observation lease and judged.
    async fn reconcile_stock_batch(
        &self,
        journal: &Journal<StockItemBatch>,
        state: &JournalBatchState<StockItemBatch>,
        mut evidence: Evidence,
    ) -> ToolOutcome {
        let (batch_id, guid) = (state.batch.batch_id(), &state.batch.company().guid);
        if let Some(verdict) = (state.verdict.as_ref()).filter(|_| state.is_settled()) {
            return stock_journal_report(state, verdict, evidence);
        }
        // As the ledger's reconcile: nothing is read while another process is
        // dispatching on this endpoint. Unlike it, no verdict is written then.
        let _observation = match dispatch_lease::acquire_observation(&self.settings.endpoint) {
            Ok(lease) => lease,
            Err(code) => return stock_sent(batch_id, guid, evidence, &code, Some(true)),
        };
        let identity = match self.verified_company(guid).await {
            Ok((company, identity, identity_evidence)) => {
                evidence = combine_evidence(evidence.clone(), identity_evidence);
                if !same_company(&company.name, &identity, state.batch.company()) {
                    let code = MasterPostCode::CompanyIdentityMismatch.as_str();
                    return stock_sent(batch_id, guid, evidence, code, Some(true));
                }
                identity
            }
            Err(failure) => return stock_sent(batch_id, guid, evidence, &failure.code, Some(true)),
        };
        let verified = self.verify_stock_batch(journal, state, &identity, &Answer::Later, None);
        ToolOutcome {
            payload: verified.await,
            evidence,
            company_guid: Some(guid.clone()),
            truncated: false,
        }
    }

    /// Reads the book back, judges it, records the verdict and answers.
    /// `unrecorded` is Tally's answer when its first append failed.
    async fn verify_stock_batch(
        &self,
        journal: &Journal<StockItemBatch>,
        state: &JournalBatchState<StockItemBatch>,
        identity: &VerifiedCompanyIdentity,
        answer: &Answer,
        unrecorded: Option<&MasterDispatchResponse>,
    ) -> Value {
        let read_back = StockReadBack::of(
            &self.settings.data_dir,
            journal,
            state.batch.batch_id(),
            identity,
        );
        let judged = match stock_marks(identity) {
            Ok(marks) => {
                (self.runtime)
                    .read_and_recheck_stock_item(
                        self.tally_config(),
                        identity,
                        marks,
                        stock_company_marks,
                        |reads| {
                            // Judged here, on this execution's own reads.
                            let (candidate, _, _, at_binding, at_aim) = reads.into_parts();
                            let catalogue = CompleteStockItemCatalogue::from_admitted(&candidate);
                            let parsed = (catalogue.ok())
                                .zip(aim_master_marks(&at_binding).ok())
                                .zip(aim_master_marks(&at_aim).ok());
                            Ok(match parsed {
                                Some(((catalogue, binding), aim)) => {
                                    let fresh = (&binding[..], &aim[..], catalogue.catalogue());
                                    ("read", read_back.judge(answer, Some(fresh)))
                                }
                                None => ("unparsed", read_back.judge(answer, None)),
                            })
                        },
                    )
                    .await
            }
            Err(failure) => Err(anyhow::anyhow!(failure.code)),
        };
        let (status, judgement) =
            judged.unwrap_or_else(|_| ("failed", read_back.judge(answer, None)));
        // Tally's answer first when it is still unrecorded, then the verdict.
        let verdict = MasterVerdict::stock(&state.batch, &judgement);
        let mut answer_recorded = state.response.is_some();
        let read = read_stock_item_journal;
        let verdict =
            self.append_kind_under_lock(state, verdict, unrecorded, &mut answer_recorded, read);
        let recorded = (verdict, answer_recorded);
        let counters = match (unrecorded, &state.response) {
            (Some(response), _) | (None, Some(response)) => response.counters,
            (None, None) => None,
        };
        let moment = matches!(answer, Answer::Posted(_)).then_some("after_post");
        let status = json!({ "state": status });
        stock_verification_result(state, &judgement, status, &recorded, moment, counters)
    }
}

/// The answer on a judged stock batch, with the ledger answer's field names;
/// verified only once the verdict is recorded. `moment` is `Some` at the post.
fn stock_verification_result(
    state: &JournalBatchState<StockItemBatch>,
    judgement: &StockJudgement,
    read_back: Value,
    (verdict, answer_recorded): &(Result<bool, String>, bool),
    moment: Option<&str>,
    counters: Option<MasterCounters>,
) -> Value {
    use MasterVerdictState::{MastersAfterPostUnconfirmed as Unconfirmed, PostedVerified};
    let judged = judgement.state();
    let verified = judged == PostedVerified;
    let (reported, note, step) = match judged {
        PostedVerified if verdict.is_ok() => ("posted_verified", VERIFIED_NOTE, VERIFIED_STEP),
        PostedVerified => ("posted_unverified", UNRECORDED_NOTE, OPEN_STEP),
        Unconfirmed => (verdict_state_code(judged), UNCONFIRMED_NOTE, OPEN_STEP),
        _ => (verdict_state_code(judged), OPEN_NOTE, OPEN_STEP),
    };
    let mut next_step = step.to_string();
    if verdict.is_err() {
        next_step.push_str(STOCK_VERDICT_UNRECORDED);
    }
    if moment.is_some() && !answer_recorded {
        next_step.push_str(STOCK_ANSWER_UNRECORDED);
    }
    let mut result = json!({
        "batch_id": state.batch.batch_id(),
        "state": reported,
        "attempt_recorded": true,
        "answer_recorded": answer_recorded,
        "dispatch": {"state": "dispatched", "resent": false},
        "verification": {
            "moment": moment.unwrap_or("reconcile"),
            "settled": verified && verdict.is_ok(),
            "guid": judgement.guid(),
            "reasons": judgement.reasons(),
            "read_back": read_back,
            "verdict_recorded": verdict.is_ok(),
            "verdict_written": matches!(verdict, Ok(true)),
        },
        "counters": counters_json(counters.as_ref()),
        "next_step": next_step,
        "note": note,
    });
    if let Err(code) = verdict {
        result["verification"]["verdict_record_error"] = json!(code);
    }
    json!({ "result": result })
}

/// The stock journal's own account of a settled batch, with no read.
fn stock_journal_report(
    state: &JournalBatchState<StockItemBatch>,
    verdict: &MasterVerdict,
    evidence: Evidence,
) -> ToolOutcome {
    let judged = verdict.state();
    let (note, next_step) = match judged {
        MasterVerdictState::PostedVerified => (VERIFIED_NOTE, VERIFIED_STEP),
        _ => (OPEN_NOTE, OPEN_STEP),
    };
    let counters = (state.response.as_ref()).and_then(|response| response.counters);
    ToolOutcome {
        payload: json!({"result": {
            "batch_id": state.batch.batch_id(),
            "state": verdict_state_code(judged),
            "attempt_recorded": true,
            "dispatch": {"state": "dispatched", "resent": false},
            "journal": {
                "response_recorded": state.response.is_some(),
                "counters": counters_json(counters.as_ref()),
                "verdict": verdict_state_code(judged),
            },
            "note": note,
            "next_step": next_step,
        }}),
        evidence,
        company_guid: Some(state.batch.company().guid.clone()),
        truncated: false,
    }
}

/// The all-company marks request a stock read and a stock approval are sized by.
fn stock_marks(identity: &VerifiedCompanyIdentity) -> Result<AgentReadRequest, ToolFailure> {
    AgentReadRequest::parse(
        crate::agent::read_profiles::render_agent_company_high_water(identity.display_name()),
    )
    .map_err(|error| error.to_string().into())
}

/// The owed check: the bytes the approval sends digest to the batch's own
/// digest. Run once per dispatch, after the take and before the lease and the
/// queue, so a refusal leaves the approval unused and writes nothing.
fn owed_digest(approved: &ApprovedImport, batch: &StockItemBatch) -> Result<(), ToolFailure> {
    if approved.xml_sha256() != batch.batch_sha256() {
        return Err(MasterPostCode::BatchChanged.into());
    }
    Ok(())
}

/// The stock writer's refusal as the queue takes it: a token or an approval of
/// another batch, or another digest, is a changed batch.
fn stock_intent_refusal(refusal: StockIntentRefusal) -> BeforeDispatchError {
    match refusal {
        StockIntentRefusal::Refused(refusal) => BeforeDispatchError::Refused(refusal),
        StockIntentRefusal::Other(code) => BeforeDispatchError::Other(code),
        StockIntentRefusal::OtherBatch | StockIntentRefusal::OtherDigest => {
            BeforeDispatchError::Refused(UnderLockRefusal::BatchChanged)
        }
    }
}

/// The answer for a stock batch this call did not send, when a send of it is
/// recorded (`Some(true)`) or the journal could not say (`None`): `code` is
/// why this call stopped.
fn stock_sent(
    batch_id: &str,
    guid: &str,
    mut evidence: Evidence,
    code: &str,
    attempted: Option<bool>,
) -> ToolOutcome {
    evidence.state = "partial";
    evidence.reason_code = Some(code.to_string());
    let message = if attempted == Some(true) {
        STOCK_SENT_NOTE
    } else {
        STOCK_UNKNOWN_NOTE
    };
    ToolOutcome {
        payload: json!({"result": {
            "batch_id": batch_id,
            "attempt_recorded": attempted,
            "dispatch": {"state": "reconciliation_required", "resent": false},
            "verification": {"attempted": false},
            "error": {"code": code, "message": message, "next_step": STOCK_NEXT_STEP},
        }}),
        evidence,
        company_guid: Some(guid.to_string()),
        truncated: false,
    }
}

/// The answer to a stock item refused before the ask by what `view` shows of
/// the stock journal, view by view as the ledger post's
/// `overlap_refusal_outcome` answers; `None` when no open dispatch holds its
/// name. This call sent nothing in any case.
fn stock_hold_refusal(
    batch_id: &str,
    guid: &str,
    accumulated: &Evidence,
    view: &OverlapView,
) -> Option<ToolOutcome> {
    let (code, attempted, dispatch, message, next_step) = match view {
        OverlapView::Settled => return None,
        OverlapView::Held(_) => (
            OVERLAP_CODE,
            json!(false),
            "not_dispatched",
            STOCK_HOLD_NOTE,
            STOCK_HOLD_NEXT_STEP,
        ),
        OverlapView::SendRecordedSince => (
            UnderLockRefusal::AlreadyAttempted.code(),
            json!(true),
            "reconciliation_required",
            STOCK_SENT_SINCE_NOTE,
            STOCK_NEXT_STEP,
        ),
        // This call recorded nothing; whether another did is not known.
        OverlapView::Unread => (
            OVERLAP_CODE,
            Value::Null,
            "not_dispatched",
            STOCK_HOLD_UNREAD_NOTE,
            STOCK_UNREAD_NEXT_STEP,
        ),
    };
    let mut evidence = accumulated.clone();
    evidence.state = "partial";
    evidence.reason_code = Some(code.to_string());
    let mut error =
        json!({"code": code, "message": message, "hold": view.code(), "next_step": next_step});
    if let OverlapView::Held(held) = view {
        held.add_to(&mut error);
    }
    Some(ToolOutcome {
        payload: json!({"result": {
            "batch_id": batch_id,
            "attempt_recorded": attempted,
            "dispatch": {"state": dispatch, "resent": false},
            "error": error,
        }}),
        evidence,
        company_guid: Some(guid.to_string()),
        truncated: false,
    })
}

/// The refusal a failed stock dispatch becomes: a stock recheck refused in the
/// queue answers the code the prepare gives the same condition, a draft the
/// book now makes differently the code the post gives it before the ask, and
/// anything else is mapped as the ledger post maps it.
fn stock_post_failure(error: anyhow::Error) -> ToolFailure {
    use StockItemRecheckError::{Catalogue, Draft, DraftChanged, NameNotAbsent, NotGoods, Sibling};
    let refused = (error.chain()).find_map(|cause| cause.downcast_ref::<StockItemRecheckError>());
    let code = match refused {
        Some(DraftChanged) => return MasterPostCode::BatchChanged.into(),
        Some(NameNotAbsent(refusal)) => item_name_refusal_code(refusal),
        Some(Sibling(refusal)) => sibling_refusal_code(refusal),
        Some(NotGoods(refusal)) => not_goods_refusal_code(refusal),
        Some(Draft(refusal)) => draft_refusal_code(refusal),
        Some(Catalogue(refusal)) => stock_item_catalogue_refusal_code(refusal),
        _ => return post_failure(error),
    };
    ToolFailure::from(code)
}

/// The draft of `name` copied from `sibling`, built from the one execution's
/// reads, and its preview; each refusal is its declared stock-item code.
fn show_fresh_draft(
    candidate: &AdmittedCandidateRead,
    groups: &AdmittedStockGroups,
    units: &AdmittedStockUnits,
    name: &MasterName,
    sibling: &MasterName,
    origin: &str,
) -> Result<ShownStockItem, StockItemCreateCode> {
    let catalogue = CompleteStockItemCatalogue::from_admitted(candidate)
        .map_err(|error| stock_item_catalogue_refusal_code(&error))?;
    let name = (catalogue.name_absent(name)).map_err(|error| item_name_refusal_code(&error))?;
    let sibling = SiblingGst::named(catalogue.catalogue(), sibling)
        .map_err(|error| sibling_refusal_code(&error))?;
    let sibling =
        GoodsSibling::from_sibling(sibling).map_err(|error| not_goods_refusal_code(&error))?;
    let draft = StockItemDraft::new_admitted(name, sibling, groups, units)
        .map_err(|error| draft_refusal_code(&error))?;
    show_stock_item_draft(draft, origin).map_err(|error| error.code())
}

/// A refused stock read. The read refuses Education itself, before any stock
/// request and before the reads could hand on a profile, so that refusal is the
/// ledger prepare's licensed-mode refusal; any other is a runtime read failure.
fn stock_read_failure(error: anyhow::Error) -> ToolFailure {
    let education = error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<MastersReadError>(),
            Some(MastersReadError::EducationUnqualified)
        )
    });
    if education {
        if let Err(failure) = require_licensed_mode(DateBoundaryProfile::EducationRestricted) {
            return failure;
        }
    }
    ToolFailure::from_runtime(crate::agent::GENERIC_RUNTIME_READ_FAILURE, error)
}

#[cfg(test)]
#[path = "agent_master_stock_tools_tests.rs"]
mod tests;
