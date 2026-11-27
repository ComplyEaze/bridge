//! `prepare_stock_item` (#1381): the stock twin of `prepare_ledger_creates`.
//! No tool is registered for it yet, so nothing shipped calls it.
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
//! `post_stock_item` is the stock twin of the ledger post's pass, and nothing
//! registers it either. It reads the batch from the stock journal, verifies the
//! company, shows the item again from the book as it stands through the same
//! read as above, refuses unless the batch that item makes is the stored one,
//! and refuses a name an open stock dispatch holds. Then it asks and holds the
//! person's approval as the ledger post does, takes it, and sends the one
//! Create through the shared queue with the stock recheck and the stock intent
//! writer. It reads nothing back: its answer is NOT verified, and it records no
//! answer from Tally.
use super::super::approval::Redemption;
use super::super::master_ledger::read_stock_item_journal;
use super::super::master_ledger::stock_item_journal::{
    record_stock_item_intent, StockIntentRefusal, StockItemBatch,
};
use super::super::master_ledger::JournalRecord;
use super::super::master_post::StockItemRecheckError;
use super::super::master_post::{
    recheck_stock_item_for_dispatch, stock_company_marks, StockItemRecheckPassed,
};
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

/// What a caller is told after a stock item is prepared. It names no tool: none
/// posts a stock item yet.
const PREPARED_STOCK_NEXT_STEP: &str = "Nothing was written to Tally. This stock item is recorded for approval; the local user must approve it in a native dialog before it is created.";

/// What a caller is told after the one stock-item Create was sent: nothing
/// reads it back yet.
const POSTED_STOCK_NOTE: &str = "Tally answered the one stock item create (any counters it gave are here), but ComplyEaze Bridge has not read the stock item back: it is NOT verified. Check the stock item in Tally before relying on it, and never post this batch again.";
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

impl Server {
    /// Prepare ONE stock item, a copy of the sibling stock item `sibling` under
    /// `name`, in the company `company_guid`. Nothing is written to Tally.
    #[cfg_attr(not(test), allow(dead_code))]
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
    #[cfg_attr(not(test), allow(dead_code))]
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
        let state = (self.read_kind_journal_shared(read_stock_item_journal)?)
            .state(batch_id)
            .cloned()
            .ok_or(MasterPostCode::BatchNotFound)?;
        let batch = state.batch;
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
                let code = UnderLockRefusal::AlreadyAttempted.code();
                let sent = stock_sent(batch_id, &guid, accumulated.clone(), code, Some(true));
                return Ok(Step::Done(Box::new(sent)));
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
                    |passed| record_under_lock(data_dir, &batch, passed, &taken),
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
            let outcome = bridge_tally_protocol::parse_import_outcome(&posted.body).ok();
            let counters = reported_counters(outcome.as_ref());
            Ok(Step::Done(Box::new(ToolOutcome {
                payload: json!({"result": {
                    "batch_id": batch_id,
                    "state": "posted_unverified",
                    "attempt_recorded": true,
                    "dispatch": {"state": "dispatched", "resent": false},
                    "counters": counters_json(counters.as_ref()),
                    "verification": {"attempted": false},
                    "preview": preview,
                    "note": POSTED_STOCK_NOTE,
                    "next_step": STOCK_NEXT_STEP,
                }}),
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

/// The stock intent, written under the admission lock. A lock it cannot take
/// is refused by name before any byte is written, as the ledger post's is.
fn record_under_lock(
    data_dir: &Path,
    batch: &StockItemBatch,
    passed: StockItemRecheckPassed,
    taken: &Redemption<'_>,
) -> Result<RecordedIntent, BeforeDispatchError> {
    let lock = ExclusiveAdmission::acquire_or_refuse(data_dir)
        .map_err(|refusal| BeforeDispatchError::Refused(UnderLockRefusal::Admission(refusal)))?;
    record_stock_item_intent(data_dir, &lock, batch, passed, taken).map_err(stock_intent_refusal)
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
