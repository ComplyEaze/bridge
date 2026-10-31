//! One user-approved voucher attempt (a Journal, Payment, Receipt or Contra);
//! subsequent calls only reconcile its identity.
use super::approval::{self, ApprovalBinding, Begin, HeldRequest, Joined, PreviewForm};
use super::*;
use crate::agent::evidence_from_runtime_read;
use crate::tally::approved_import::{
    Answered, ApprovedImport, ApprovedImportAdmissionError, BeforeDispatchError,
    PendingPostApproval, QueuedAdmission, UnderLockRefusal,
};
use bridge_tally_protocol::native_outstandings::{parse_company_currency, BaseCurrencyName};
use bridge_tally_protocol::{parse_import_outcome, TallyImportApplicationStatus};

/// The batch's verification window is one the pre-flight bound (§11c) would
/// read in parts, but the pre-post check inside the dispatch lease sends it as
/// one request. Refused before approval; the batch can be posted over a
/// narrower window.
pub(super) const IMPORT_POST_WINDOW_NOT_BOUNDED: &str = "import_post_window_not_bounded";

/// Admit the whole-window pre-post request on what the verification read of
/// the same window measured: an undivided read was that request, and a divided
/// read's parts together are its size. A verification that reported nothing is
/// refused, not assumed small.
/// Which saved batches a caller may post. The agent's `post_import` posts one
/// Journal, Payment, Receipt or Contra (ADR 0004, amended 2026-09-22); the
/// desktop's review and approval were built for one Journal and stay so.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::agent) enum PostScope {
    JournalOnly,
    Vouchers,
}

impl PostScope {
    fn admits(self, voucher_type: &VoucherType) -> bool {
        match self {
            Self::JournalOnly => *voucher_type == VoucherType::Journal,
            Self::Vouchers => matches!(
                voucher_type,
                VoucherType::Journal
                    | VoucherType::Payment
                    | VoucherType::Receipt
                    | VoucherType::Contra
                    | VoucherType::Sales
            ),
        }
    }

    fn refusal(self) -> &'static str {
        match self {
            Self::JournalOnly => "import_post_requires_one_journal",
            Self::Vouchers => "import_post_requires_one_voucher",
        }
    }
}

/// How many bytes the in-queue classification recheck may spend describing a
/// refusal it never returns: only whether a leg failed is used.
const RECHECK_REFUSAL_BUDGET: usize = 4_096;

/// Where a post stands with its person, when it returns without posting.
pub(super) enum ApprovalState {
    /// The dialog is open, with this much of its time limit left when known.
    Pending(Option<std::time::Duration>),
    Approved,
}

/// How a post call enters (#725 slice 2.0).
pub(in crate::agent) enum Entry {
    /// A call from the agent or the desktop: it may ask, join or redeem.
    Fresh,
    /// The second pass of a call, from the top, to redeem the approval its
    /// own Join just found, keeping that call's evidence. It may only
    /// redeem: it never asks, so it can never show a second dialog. It also
    /// keeps that call's start, so every budget in the second pass is
    /// measured from the call and not from the pass.
    RedeemOnly {
        evidence: Evidence,
        call_started: std::time::Instant,
    },
}

/// What a post's checked body came to: an answer, or an approval its Join
/// found that the same call redeems by entering again from the top, so every
/// check before the wait is made again after it.
pub(super) enum Step {
    Done(Box<ToolOutcome>),
    Redeem,
}

/// What an agent post's approval steps need to know about the post they serve,
/// whichever kind it is: the tool a caller is told to call again, the batch,
/// the company, and how many vouchers (one for a master) it would send.
pub(super) struct ApprovalTarget<'a> {
    pub(super) tool: &'a str,
    pub(super) batch_id: &'a str,
    pub(super) guid: &'a str,
    pub(super) vouchers: usize,
}

/// What one pass of a post call came to: its answer, or a click its Join
/// found already made, which the same call redeems in a second, redeem-only
/// pass (#725 slice 2.0). The passes run one after the other, never nested.
pub(in crate::agent) enum Pass {
    Done(ToolOutcome),
    Redeem {
        evidence: Evidence,
        call_started: std::time::Instant,
    },
}

/// What waiting on a dialog came to.
pub(super) enum Waited {
    /// Boxed: an approval carries an invoice's read plan (#1337), and the two
    /// other cases are a handle and a unit.
    Answered(Box<(Result<ApprovedImport, String>, Answered)>),
    Open(PendingPostApproval),
    /// The call was withdrawn while it waited. The dialog was dropped, which
    /// closes it.
    Cancelled,
}

/// What is left of the call's budget for waiting on the person, never less
/// than enough to have the dialog up before the call returns.
pub(super) fn call_budget(call_started: std::time::Instant) -> std::time::Duration {
    approval::CALL_BUDGET
        .saturating_sub(call_started.elapsed())
        .max(approval::MIN_DIALOG_WAIT)
}

/// Kept back from the call's ceiling for what a post still does after its
/// marks readback: the verification readback took 12.1 s at 200 vouchers, the
/// largest batch measured live, plus the spacing between the requests
/// (`approval::MEASURED_POST`).
/// Only lock waits are bounded: the retry's own send, and the wire waits of the
/// checks after it (each at most the policy total), are not counted, so the
/// call can pass its ceiling by about that much and stay under the host limit.
/// Nor is a drain met by the post's admission reads after a cut request
/// (#2142): at most `DrainPolicy::worst_per_call`, which a test holds under the
/// host limit with the ceiling. The marks readback's retry keeps a drain's
/// worst case back from the ceiling as well.
const AFTER_READ_REST_OF_POST: std::time::Duration = std::time::Duration::from_secs(13);
/// A retry with less to wait than this is not worth its send.
const AFTER_READ_MIN_RETRY_WAIT: std::time::Duration = std::time::Duration::from_secs(1);

/// The wire wait for the one retry of the marks readback after a sent post
/// (#884): the policy's own total, cut to what the call has left of its
/// ceiling once the rest of the post is kept back. `None` when too little is
/// left to be worth another wait, so a slow call is never stretched past the
/// ceiling by the retry.
fn after_read_retry_budget(
    elapsed: std::time::Duration,
    policy_total: std::time::Duration,
) -> Option<std::time::Duration> {
    let left = approval::CALL_CEILING
        .saturating_sub(elapsed)
        .saturating_sub(AFTER_READ_REST_OF_POST);
    (left >= AFTER_READ_MIN_RETRY_WAIT).then(|| left.min(policy_total))
}

/// The wire wait a redeem-only pass may spend (#893): what the call's shared
/// budget has left, cut so that the redeem measured live still fits under the
/// call's ceiling from here. A pass with no time left waits for nothing: an
/// admission read that meets a held lock is refused at once as
/// `tally_endpoint_busy`, before the attempt is recorded and before anything is
/// sent, rather than holding the call past its ceiling.
pub(super) fn redeem_pass_wire_budget(
    elapsed: std::time::Duration,
    shared_left: std::time::Duration,
) -> std::time::Duration {
    approval::CALL_CEILING
        .saturating_sub(elapsed)
        .saturating_sub(approval::MEASURED_REDEEM)
        .min(shared_left)
}

/// Why the marks readback after a sent post failed: the transport's own safe
/// code when it has one (a busy wire lock is `tally_endpoint_busy`), so a
/// doubt says the read was held back and never that the step moved wrongly.
fn after_read_cause(error: &anyhow::Error) -> &'static str {
    error
        .chain()
        .find_map(|cause| {
            cause
                .downcast_ref::<bridge_tally_transport::TallyTransportError>()
                .map(bridge_tally_transport::TallyTransportError::safe_code)
        })
        .unwrap_or("marks_readback_failed")
}

/// Wait up to `budget` for the person's answer, stopping at once if the call
/// is withdrawn (#554): nothing is sent to Tally while waiting.
pub(super) async fn wait_for_answer(
    dialog: PendingPostApproval,
    budget: std::time::Duration,
) -> Waited {
    let withdrawn = crate::tally::runtime::TOOL_CANCELLATION
        .try_with(Clone::clone)
        .ok();
    let withdrawal = async {
        match &withdrawn {
            Some(token) => token.cancelled().await,
            None => std::future::pending().await,
        }
    };
    // Biased: a withdrawal ready in the same poll as an answer wins, so an
    // answer never outlives the cancellation of the call that asked (#725).
    tokio::select! {
        biased;
        () = withdrawal => Waited::Cancelled,
        waited = dialog.answer_within(budget) => match waited {
            Ok(answer) => Waited::Answered(Box::new(answer)),
            Err(dialog) => Waited::Open(dialog),
        },
    }
}

pub(super) fn admit_post_window(served: Option<crate::agent::WindowServed>) -> Result<(), String> {
    match served {
        Some(served) if served.fits_one_request() => Ok(()),
        _ => Err(IMPORT_POST_WINDOW_NOT_BOUNDED.to_string()),
    }
}

impl Server {
    /// The response path runs only after its interrupted post future is gone.
    /// It may therefore use the endpoint lease to distinguish a locally empty
    /// journal from another connector's in-flight pre-intent admission.
    pub(in crate::agent) fn cancelled_import_for_response(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let snapshot = self.recorded_import_snapshot(args)?;
        let batch_id = required_string(args, "batch_id")?;
        let guid = required_string(args, "company_guid")?;
        let attempted =
            self.post_failure_attempt_observation(batch_id, guid, snapshot.as_ref(), None);
        let mut evidence =
            evidence_from_runtime_read(crate::tally::runtime::RuntimeReadEvidence::empty());
        evidence.state = "partial";
        evidence.reason_code = Some("request_cancelled".into());
        Ok(ToolOutcome {
            payload: json!({"result":{
                "batch_id":batch_id,"attempt_recorded":attempted,
                "dispatch":{"state":if attempted == Some(false) { "not_dispatched" } else { "reconciliation_required" },"resent":false},
                "error":{"code":"request_cancelled","message":if attempted == Some(false) {
                    "The request was cancelled. No posting attempt is recorded for this saved batch."
                } else {
                    "The request was cancelled with an uncertain outcome. Reconcile this original batch with verify_import; never rebuild or resend it."
                }}
            }}),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }

    pub(in crate::agent) fn recorded_import_attempt(
        &self,
        args: &Value,
    ) -> Result<Option<bool>, ToolFailure> {
        self.recorded_import_snapshot(args)
            .map(|snapshot| snapshot.map(|snapshot| snapshot.dispatched))
    }

    fn recorded_import_snapshot(
        &self,
        args: &Value,
    ) -> Result<Option<ledger::BatchSnapshot>, ToolFailure> {
        let batch_id = required_string(args, "batch_id")?;
        let guid = required_string(args, "company_guid")?;
        let uuid = batch_id
            .strip_prefix("bridge-")
            .and_then(|id| uuid::Uuid::parse_str(id).ok())
            .ok_or_else(|| "import_batch_identifier_invalid".to_string())?;
        if batch_id != format!("bridge-{uuid}") {
            return Err("import_batch_identifier_invalid".to_string().into());
        }
        match self.latest_import_snapshot(batch_id) {
            Ok(snapshot) => {
                Ok(snapshot
                    .filter(|snapshot| batch_guid_matches(&snapshot.batch.company_guid, guid)))
            }
            Err(error) if error == "import_admission_busy" => Err(error.into()),
            Err(_) => Ok(None),
        }
    }

    /// The marks readback after a sent post. It gets a wire wait of its own:
    /// the admission reads may have spent the call's budget, and a marks read
    /// refused as busy after a sent post would record a lasting doubt on a
    /// clean batch (#697). A busy refusal has waited that whole budget, so it
    /// is tried once more with a second, cut to what the call has left of its
    /// ceiling (#884); a refusal that survives is returned as it is. A read
    /// refused because Tally is still finishing a request cut for time (#2142)
    /// is tried once more after the refusal's own wait, when the call's
    /// ceiling has room for both.
    pub(super) async fn read_marks_after_post(
        &self,
        request: crate::tally::agent_read_request::AgentReadRequest,
        call_started: std::time::Instant,
    ) -> anyhow::Result<String> {
        #[cfg(test)]
        let _ = CALL_STARTS.try_with(|starts| starts.lock().unwrap().push(call_started));
        #[cfg(test)]
        let _ = MARKS_READS.try_with(|reads| reads.lock().unwrap().push(None));
        let first = crate::tally::runtime::with_operation_wire_budget(
            self.runtime
                .read_company_marks_once(self.tally_config(), request.clone()),
        )
        .await;
        let pause = match first
            .as_ref()
            .err()
            .and_then(crate::endpoint_wire::wire_refusal)
        {
            Some(bridge_tally_transport::WireRefusal::Busy) => Some(std::time::Duration::ZERO),
            Some(bridge_tally_transport::WireRefusal::Draining { retry_after_ms }) => {
                Some(std::time::Duration::from_millis(retry_after_ms))
            }
            _ => None,
        };
        let policy_total = self.runtime.wire_gate_config().retry().total();
        // The retried read may meet a drain of its own, so its worst case is
        // kept back from the ceiling too.
        let drain = self
            .runtime
            .wire_gate_config()
            .drain()
            .map_or(std::time::Duration::ZERO, |policy| policy.worst_per_call());
        let Some((pause, budget)) = pause.and_then(|pause| {
            after_read_retry_budget(call_started.elapsed() + pause + drain, policy_total)
                .map(|budget| (pause, budget))
        }) else {
            return first;
        };
        tokio::time::sleep(pause).await;
        #[cfg(test)]
        let _ = MARKS_READS.try_with(|reads| reads.lock().unwrap().push(Some(budget)));
        crate::tally::runtime::with_operation_wire_budget_of(
            budget,
            self.runtime
                .read_company_marks_once(self.tally_config(), request),
        )
        .await
    }

    /// The company's masters across the post (#239). Only when the snapshots
    /// either side of the POST prove the target's master mark unchanged is
    /// nothing read. Otherwise (the mark moved, or either snapshot could not
    /// be read or did not hold exactly one target row) the ledger catalogue is
    /// read again and each approved ledger's name resolved to its GUID: a
    /// posted voucher's lines carry names, not GUIDs (#239, lab comment of
    /// 2026-09-23).
    async fn masters_after_post(
        &self,
        marks_before: &str,
        marks_after: Option<&[location::LoadedCompanyMarks]>,
        identity: &super::super::VerifiedCompanyIdentity,
        company_name: &str,
        approved: &bridge_tally_protocol::StandardLedgerCatalogBinding,
        accumulated: &mut Evidence,
    ) -> Value {
        let unchanged = location::parse_all_company_marks(marks_before)
            .ok()
            .zip(marks_after)
            .and_then(|(before, after)| {
                location::target_masters_unchanged(
                    &before,
                    after,
                    identity.company_guid(),
                    company_name,
                )
            });
        let trigger = match unchanged {
            Some(true) => return json!({"state":"not_checked","reason":"masters_unmoved"}),
            Some(false) => "masters_moved",
            None => "masters_unconfirmed",
        };
        let approved = approved
            .pairs()
            .map(|(name, guid)| (name.to_string(), guid.to_string()))
            .collect::<Vec<_>>();
        self.ledgers_still_approved(identity, company_name, &approved, trigger, accumulated)
            .await
    }

    /// Whether each approved (name, GUID) still resolves by name to its GUID,
    /// as a masters check verdict with `trigger` saying why it ran (#239).
    pub(super) async fn ledgers_still_approved(
        &self,
        identity: &super::super::VerifiedCompanyIdentity,
        company_name: &str,
        approved: &[(String, String)],
        trigger: &str,
        accumulated: &mut Evidence,
    ) -> Value {
        match self
            .read_import_ledger_catalogue(identity, company_name)
            .await
        {
            Err(_) => json!({"state":"check_unavailable","trigger":trigger}),
            Ok((_, catalogue, _, evidence)) => {
                *accumulated = combine_evidence(accumulated.clone(), evidence);
                let changed = approved
                    .iter()
                    .filter(|(name, guid)| {
                        catalogue
                            .catalog()
                            .bind_selected([name.clone()])
                            .ok()
                            .and_then(|now| {
                                now.pairs()
                                    .next()
                                    .map(|(_, current)| current.eq_ignore_ascii_case(guid))
                            })
                            != Some(true)
                    })
                    .map(|(name, _)| name.clone())
                    .collect::<Vec<_>>();
                if changed.is_empty() {
                    json!({"state":"unchanged","trigger":trigger})
                } else {
                    json!({"state":"posted_under_changed_masters","trigger":trigger,"ledgers":changed})
                }
            }
        }
    }

    /// How many vouchers one post may send: up to the batch cap on the MCP path
    /// whenever posting is on, since one setting governs every post (#1090);
    /// the desktop posts one Journal.
    pub(super) fn post_voucher_limit(&self, scope: PostScope) -> usize {
        if self.settings.posting.enabled() {
            Self::post_voucher_limit_when_enabled(scope)
        } else {
            1
        }
    }

    /// The limit `post_voucher_limit` gives once posting is on; a build asks admission at it
    /// whatever the setting, to say whether turning posting on would give the batch a route.
    pub(super) fn post_voucher_limit_when_enabled(scope: PostScope) -> usize {
        if scope == PostScope::Vouchers {
            ledger::MAX_BATCH_POST_VOUCHERS
        } else {
            1
        }
    }

    /// A post that is waiting for its person, not failing (#725): the dialog is
    /// open (`pending`) or answered and not yet posted (`approved`). Nothing
    /// was sent to Tally; calling `post_import` again with the same batch
    /// waits on the dialog or posts the approved batch.
    pub(super) fn approval_outcome(
        &self,
        tool: &str,
        batch_id: &str,
        guid: &str,
        state: ApprovalState,
        evidence: Evidence,
    ) -> ToolOutcome {
        let approval = match state {
            ApprovalState::Pending(remaining) => json!({
                "state": "pending",
                "dialog_remaining_s": remaining.map(|remaining| remaining.as_secs()),
                "retry_after_s": approval::RETRY_AFTER.as_secs(),
            }),
            ApprovalState::Approved => json!({
                "state": "approved",
                "expires_in_s": self
                    .post_approvals
                    .approval_remaining(batch_id)
                    .map(|remaining| remaining.as_secs()),
                "retry_after_s": 0,
                "next_step": format!("Call {tool} again now with the same batch: nothing is posted until you do, and the approval lapses after expires_in_s."),
            }),
        };
        ToolOutcome {
            payload: json!({"result":{
                "batch_id": batch_id,
                "attempt_recorded": false,
                "approval": approval,
                "previous_approval": self.post_approvals.lapse_note(batch_id),
                "dispatch": {"state": "not_dispatched", "resent": false},
            }}),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        }
    }

    /// What a call finds held for its batch, before it checks anything else
    /// (#725): the one place an agent post's Pending, Redeem and Join are
    /// decided, shared by the voucher and the master tools so neither forks it.
    /// `Ok(Ok((redeeming, form)))` goes on, with the form the ask shows or the
    /// approval is bound to; `Ok(Err(step))` is the call's answer, or a click
    /// found that a second, redeem-only pass takes.
    pub(super) async fn begin_agent_approval(
        &self,
        target: &ApprovalTarget<'_>,
        redeem_only: bool,
        call_started: std::time::Instant,
        accumulated: &Evidence,
    ) -> Result<Result<(bool, PreviewForm), Step>, ToolFailure> {
        let ApprovalTarget {
            tool,
            batch_id,
            guid,
            vouchers,
        } = *target;
        // Re-entered to redeem what its Join found: only that approval lets it
        // go on, and it never asks (#725 slice 2.0).
        if redeem_only {
            return Ok(Ok((true, self.post_approvals.begin_redeem(batch_id)?)));
        }
        match self.post_approvals.begin(batch_id) {
            Begin::Ask(form) => Ok(Ok((false, form))),
            Begin::Redeem(form) => Ok(Ok((true, form))),
            Begin::Busy(code) => Err(code.to_string().into()),
            Begin::Refused(code) => Err(code.into()),
            Begin::Waiting => Ok(Err(Step::Done(Box::new(self.approval_outcome(
                tool,
                batch_id,
                guid,
                ApprovalState::Pending(None),
                accumulated.clone(),
            ))))),
            Begin::Join(dialog) => {
                // Only a click that landed while no call waited is posted by the
                // call that finds it (#725 slice 2.0): posting after this call's
                // own wait could run past the host's timeout on a large book
                // (#852).
                let answered_at_join = dialog.answered().is_some();
                let waited = match wait_for_answer(dialog, call_budget(call_started)).await {
                    Waited::Cancelled => return Err("request_cancelled".to_string().into()),
                    Waited::Open(dialog) => Err(dialog),
                    Waited::Answered(answer) => Ok(*answer),
                };
                match self.post_approvals.settle_join(batch_id, waited) {
                    Joined::StillOpen { remaining } => {
                        Ok(Err(Step::Done(Box::new(self.approval_outcome(
                            tool,
                            batch_id,
                            guid,
                            ApprovalState::Pending(Some(remaining)),
                            accumulated.clone(),
                        )))))
                    }
                    // Posted by this call when it found the click already made
                    // and the redeem fits in it, entering again from the top so
                    // every check is made anew.
                    Joined::Approved
                        if answered_at_join
                            && self
                                .post_approvals
                                .redeem_fits_in_call(call_started.elapsed(), vouchers) =>
                    {
                        Ok(Err(Step::Redeem))
                    }
                    Joined::Approved => Ok(Err(Step::Done(Box::new(self.approval_outcome(
                        tool,
                        batch_id,
                        guid,
                        ApprovalState::Approved,
                        accumulated.clone(),
                    ))))),
                    Joined::Refused(code) => Err(code.into()),
                }
            }
        }
    }

    /// Ask the person in `asked`, wait for the answer within the call's budget,
    /// and hold what came of it for a later call: the shared tail of every
    /// agent post's approval. `Ok(Some(step))` is the call's answer (the dialog
    /// is still open, or was answered too late in the call for the post to
    /// fit); `Ok(None)` is an approval held for this call to take at once.
    pub(super) async fn ask_and_hold(
        &self,
        target: &ApprovalTarget<'_>,
        asked: PendingPostApproval,
        binding: &ApprovalBinding,
        held: impl Into<HeldRequest>,
        call_started: std::time::Instant,
        accumulated: &Evidence,
    ) -> Result<Option<Step>, ToolFailure> {
        let ApprovalTarget {
            tool,
            batch_id,
            guid,
            vouchers,
        } = *target;
        match wait_for_answer(asked, call_budget(call_started)).await {
            Waited::Cancelled => Err("request_cancelled".to_string().into()),
            Waited::Open(dialog) => {
                let remaining = dialog.remaining();
                self.post_approvals
                    .hold_pending(batch_id, binding.clone(), dialog, held)?;
                Ok(Some(Step::Done(Box::new(self.approval_outcome(
                    tool,
                    batch_id,
                    guid,
                    ApprovalState::Pending(Some(remaining)),
                    accumulated.clone(),
                )))))
            }
            Waited::Answered(answer) => {
                let (answer, answered) = *answer;
                self.post_approvals.hold_approved(
                    batch_id,
                    binding.clone(),
                    answer?,
                    held,
                    answered,
                )?;
                // Answered too late in the call for the post to fit: the next
                // call redeems it, after checking the book again.
                if self
                    .post_approvals
                    .fits_in_call(call_started.elapsed(), vouchers)
                {
                    Ok(None)
                } else {
                    Ok(Some(Step::Done(Box::new(self.approval_outcome(
                        tool,
                        batch_id,
                        guid,
                        ApprovalState::Approved,
                        accumulated.clone(),
                    )))))
                }
            }
        }
    }

    pub(in crate::agent) async fn post_import(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        self.post_import_checked(args, None, PostScope::Vouchers)
            .await
    }

    pub(in crate::agent) async fn post_import_checked(
        &self,
        args: &Value,
        expected_sha256: Option<&str>,
        scope: PostScope,
    ) -> Result<ToolOutcome, ToolFailure> {
        match self
            .post_import_entry(args, expected_sha256, scope, Entry::Fresh)
            .await?
        {
            Pass::Done(outcome) => Ok(outcome),
            // At most one more pass: a redeem-only pass never joins, so it
            // never hands on another redeem.
            Pass::Redeem {
                evidence,
                call_started,
            } => {
                #[cfg(test)]
                let _ = BETWEEN_PASSES.try_with(|between| between());
                match self
                    .post_import_entry(
                        args,
                        expected_sha256,
                        scope,
                        Entry::RedeemOnly {
                            evidence,
                            call_started,
                        },
                    )
                    .await?
                {
                    Pass::Done(outcome) => Ok(outcome),
                    // Unreachable: only a Join hands on a redeem.
                    Pass::Redeem { .. } => Err("import_approval_revoked".to_string().into()),
                }
            }
        }
    }

    /// One pass of a post call: the whole post, or up to a Join that found a
    /// click already made, which the caller then redeems in a second pass. A
    /// redeem-only pass runs on a wire-wait budget cut to what the call has
    /// left (#893): its admission reads would otherwise wait on whatever the
    /// call's shared budget still holds, past the call's ceiling.
    pub(in crate::agent) async fn post_import_entry(
        &self,
        args: &Value,
        expected_sha256: Option<&str>,
        scope: PostScope,
        entry: Entry,
    ) -> Result<Pass, ToolFailure> {
        let Entry::RedeemOnly { call_started, .. } = &entry else {
            return self
                .post_import_pass(args, expected_sha256, scope, entry)
                .await;
        };
        let shared_left = crate::tally::runtime::operation_wire_budget_remaining()
            .unwrap_or_else(|| self.runtime.wire_gate_config().retry().total());
        let elapsed = call_started.elapsed();
        #[cfg(test)]
        let elapsed = REDEEM_PASS_ELAPSED
            .try_with(|scripted| *scripted)
            .unwrap_or(elapsed);
        let budget = redeem_pass_wire_budget(elapsed, shared_left);
        #[cfg(test)]
        let _ = REDEEM_WIRE_BUDGETS.try_with(|budgets| budgets.lock().unwrap().push(budget));
        crate::tally::runtime::with_operation_wire_budget_of(
            budget,
            self.post_import_pass(args, expected_sha256, scope, entry),
        )
        .await
    }

    async fn post_import_pass(
        &self,
        args: &Value,
        expected_sha256: Option<&str>,
        scope: PostScope,
        entry: Entry,
    ) -> Result<Pass, ToolFailure> {
        let (carried, call_started) = match entry {
            Entry::Fresh => (None, std::time::Instant::now()),
            Entry::RedeemOnly {
                evidence,
                call_started,
            } => (Some(evidence), call_started),
        };
        #[cfg(test)]
        let _ = CALL_STARTS.try_with(|starts| starts.lock().unwrap().push(call_started));
        let redeem_only = carried.is_some();
        let guid = required_string(args, "company_guid")?;
        let batch_id = required_string(args, "batch_id")?;
        let admitted: Result<_, ToolFailure> = (|| {
            let snapshot = self
                .latest_import_snapshot(batch_id)?
                .ok_or_else(|| "import_batch_not_found".to_string())?;
            if expected_sha256.is_some_and(|expected| snapshot.batch.sha256 != expected) {
                return Err("import_batch_changed".to_string().into());
            }
            if !batch_guid_matches(&snapshot.batch.company_guid, guid) {
                return Err("import_batch_company_mismatch".to_string().into());
            }
            Ok(snapshot)
        })();
        let snapshot = match admitted {
            Ok(snapshot) => snapshot,
            // A refusal before the checked body withdraws an approval given
            // for this batch just as one inside it does, so it cannot hold the
            // slot against other batches for its whole limit (#857). The
            // desktop's journal-only post holds none.
            Err(failure) => {
                if scope == PostScope::Vouchers {
                    self.post_approvals
                        .revoke_unredeemed(batch_id, "post_refused_before_intent");
                }
                return Err(failure);
            }
        };
        let line = snapshot.batch.clone();
        // A re-entered call keeps what its first pass read (#725 slice 2.0).
        let mut accumulated = carried.unwrap_or_else(|| {
            evidence_from_runtime_read(crate::tally::runtime::RuntimeReadEvidence::empty())
        });
        let mut received_response = None;
        // Where the voucher went, once a POST has been sent (#574).
        let mut post_location: Option<Value> = None;
        // The Currency masters of a book refused for having several (#551).
        let mut currencies_seen: Option<Vec<String>> = None;
        // The masters check across the post, kept for a failed readback (#239).
        let mut masters_verdict: Option<Value> = None;
        // The ledgers whose GUID changed since the build (#239).
        let mut ledgers_changed: Option<Vec<String>> = None;
        // The recorded cash-in-hand ledgers no longer under Cash-in-Hand, as
        // refusal rows, and how many were left out (#815).
        let mut cash_in_hand_refused: Option<(Vec<Value>, usize)> = None;
        // The batch's own transaction ids found already in the book (#901).
        let mut preexisting_txn_ids: Option<Vec<String>> = None;
        // Set when the batch that met that refusal holds an invoice: the
        // unsettled batch of this machine with its figures, if there is one.
        let mut preexisting_invoice: Option<Result<Option<String>, String>> = None;
        // The reads of the invoice's masters the last re-read made, for the
        // queue to make again under its lock (#1337).
        let mut invoice_plan: Option<crate::tally::approved_import::InvoiceReadPlan> = None;
        // The voucher types refused for their voucher numbering (bridge#724).
        let mut numbering_refused: Option<(Value, String)> = None;
        // What the transfer check at send listed and said, for the refusal.
        let mut send_transfer_refusal: Option<(Value, String)> = None;
        // The transfer check under the admission lock refused: its code only.
        let mut transfers_changed_under_lock = false;
        let operation: Result<Step, ToolFailure> = async {
            let xml = admit_saved_voucher_integrity(
                &line,
                &self.settings.endpoint,
                scope,
                self.post_voucher_limit(scope),
            )?;
            if snapshot.dispatched {
                // Posted by any route: whatever was held for it can never be
                // posted, and must not keep other batches waiting (#725).
                self.post_approvals
                    .revoke(batch_id, "batch_already_dispatched");
                // The desktop does not go on to check a sent Journal that
                // names a ledger whose name ends in a line break: that
                // check's messages name ledgers as they are (bridge#626).
                // One not yet sent is refused where its text is made.
                if scope == PostScope::JournalOnly {
                    refuse_a_line_break_name_on_the_desktop(&line)?;
                }
                return self
                    .verify_import(args)
                    .await
                    .map(|outcome| Step::Done(Box::new(outcome)));
            }
            // A batch saved before the build refused its narration: its
            // untagged post could never be bound, so it is never sent.
            refuse_rewritten_narration(&line.vouchers)?;
            // The record's own hash only proves the record agrees with itself.
            // The file Bridge built must hold exactly the XML this record
            // renders (bridge#575). The native post below re-renders the same
            // vouchers from the same record with two differences: a fresh
            // private REMOTEID, and narrations without the `[BRIDGE:…]` marker
            // this file carries (#864). A narration's text is itself checked
            // here, so every accounting field and narration it sends is the
            // one checked.
            if self.read_persisted_import_xml(batch_id)? != xml.as_bytes() {
                return Err("import_batch_changed".to_string().into());
            }
            // Decidable from the record alone, so refused after the file check
            // and before any Tally request (#239): a batch saved before Bridge
            // recorded its ledgers' GUIDs has nothing to check them against.
            if line.ledger_identities.is_none() {
                return Err(BuildBindingRefusal::Unbound.code().to_string().into());
            }
            // Nor can a batch saved before Bridge recorded the ledgers its bank
            // cash answers named as cash in hand: nothing to check again (#815).
            if line.cash_in_hand_ledgers.is_none() {
                return Err(CASH_LEDGERS_NOT_RECORDED.to_string().into());
            }
            // Nor one saved before the build recorded which bill-wise ledgers a
            // person approved to receive entries On Account (#1234).
            if line.on_account_approved.is_none() {
                return Err(super::bill_wise::BILL_WISE_NOT_RECORDED.to_string().into());
            }
            // Nor while a Store package's copy of the data folder keeps a
            // second journal this one cannot see (#2233): before any dialog.
            super::split_journal::admit(&self.settings.data_dir)?;
            // A dialog or approval an earlier call left for this batch (#725).
            // The desktop waits for its dialog in one call, as before.
            let (redeeming, form) = match scope {
                // Never built by the flow: only an agent's Join hands on a
                // redeem, and a second pass keeps its scope. The types allow
                // this pair, and the desktop's post asks through its own
                // dialog below, so it is refused before anything could ask
                // (#725 slice 2.0).
                PostScope::JournalOnly if redeem_only => {
                    return Err("import_approval_revoked".to_string().into())
                }
                PostScope::JournalOnly => (false, PreviewForm::Listed),
                // Re-entered to redeem what its Join found: only that
                // approval lets it go on, and it never asks (#725 slice 2.0).
                PostScope::Vouchers => {
                    match self
                        .begin_agent_approval(
                            &ApprovalTarget {
                                tool: "post_import",
                                batch_id,
                                guid,
                                vouchers: line.vouchers.len(),
                            },
                            redeem_only,
                            call_started,
                            &accumulated,
                        )
                        .await?
                    {
                        Ok(asked) => asked,
                        Err(step) => return Ok(step),
                    }
                }
            };
            // Tally may have seen any REMOTEID the journal records, and resending
            // one can undo a person's cancel or delete (protocol reference §9.3).
            // Refused before any Tally request; checked again as the intent is
            // written, under the exclusive lock.
            let remote_ids = RemoteIds::mint(line.vouchers.len())?;
            let recorded = {
                let _lock = self.lock_import_admission_shared()?;
                self.import_remote_ids_recorded_while_admitted(remote_ids.as_slice())?
            };
            if recorded {
                return Err("import_remote_id_reused".to_string().into());
            }
            // A row another batch already sent to Tally is refused here too,
            // before the person is asked to approve it (#876); the check that
            // binds runs under the exclusive lock as the intent is written.
            let row_posted = {
                let _lock = self.lock_import_admission_shared()?;
                self.import_rows_already_posted_while_admitted(&line)?
            };
            if row_posted.is_some() {
                return Err("import_txn_already_posted".to_string().into());
            }
            // The transfer view, as the build judged it, against the journal as it
            // is now: refused before the person is asked, with the build's code.
            // The check that binds runs under the exclusive lock below.
            let transfers_refused = {
                let _lock = self.lock_import_admission_shared()?;
                self.transfers_refusal_for_send_while_admitted(&line)?
            };
            if let Some(refusal) = transfers_refused {
                let (code, detail, step) = refusal.into_parts();
                send_transfer_refusal = Some((detail, step));
                return Err(code.to_string().into());
            }
            let ShownPreview {
                text: preview,
                lists_vouchers: preview_lists_vouchers,
            } = review_preview_for_form(&line, &self.settings.endpoint, scope, form)?;
            // Number matching precedence is not qualified for native Create.
            // Previously dispatched numbered batches remain reconcilable above.
            // Also admits, on this read's measurement, the whole-window request
            // the lease sends before posting (§11c).
            let before = self.verify_import_for_post(args).await?;
            accumulated = combine_evidence(accumulated.clone(), before.evidence);
            require_absent_verification_result(&before.payload["result"], line.vouchers.len())
                .inspect_err(|_| {
                    preexisting_txn_ids = Some(present_txn_ids(&before.payload["result"]));
                    preexisting_invoice = self.import_unsettled_invoice_twin_for_refusal(&line);
                })?;
            let payload = ImportPayload {
                company_guid: line.company_guid.clone(),
                vouchers: line.vouchers.clone(),
                amends_batch_id: None,
            };
            // Every voucher's date, so the queue's Education recheck covers
            // them all, not only the first.
            let voucher_dates = line
                .vouchers
                .iter()
                .map(|voucher| voucher.date.clone())
                .collect::<Vec<_>>();
            let native = native_post_request(&line, remote_ids)?;
            let xml = native.xml.clone();
            let verification_request = crate::tally::agent_read_request::AgentReadRequest::parse(
                render_import_verification_read(
                    &line
                        .company
                        .as_ref()
                        .ok_or_else(|| "import_post_company_missing".to_string())?
                        .name,
                    &line.date_from,
                    &line.date_to,
                ),
            )
            .map_err(|error| error.to_string())?;
            let (company, identity, identity_evidence) = self.verified_company(guid).await?;
            accumulated = combine_evidence(accumulated.clone(), identity_evidence);
            if line.company.as_ref() != Some(&import_company_tuple(&company)?) {
                return Err("company_identity_mismatch".to_string().into());
            }
            validate_dates(&payload, company.books_from.as_deref())?;
            let (catalogue, catalogue_identities, ledger_catalogue_request, evidence) = self
                .read_import_ledger_catalogue(&identity, &company.name)
                .await?;
            accumulated = combine_evidence(accumulated.clone(), evidence);
            if masters_for_payload(&payload, &catalogue)?
                .iter()
                .any(|item| item["match_state"] != "exact")
            {
                return Err("import_masters_changed".to_string().into());
            }
            // A ledger that now folds equal to another live ledger could be
            // taken for it by Tally's import lookup (bridge#626). Refused as the
            // build refuses it, including for a batch built before the twin
            // appeared or before the build checked for one.
            if !folded_twins(
                &requested_ledger_names(&payload),
                catalogue_identities.catalog().parents(),
                catalogue_identities.catalog().aliases(),
            )
            .is_empty()
            {
                return Err("ledger_has_folded_twin".to_string().into());
            }
            let ledger_binding = catalogue_identities
                .catalog()
                .bind_selected(requested_ledger_names(&payload))
                .map_err(|_| "import_masters_changed".to_string())?;
            // The same ledgers must still carry the GUIDs the build bound them
            // to (#239): a name alone cannot tell a ledger renamed and replaced
            // under its old name between build and post.
            admit_build_binding(line.ledger_identities.as_deref(), &ledger_binding).map_err(
                |refusal| {
                    if let BuildBindingRefusal::Changed(ledgers) = &refusal {
                        // Before the list is cut to the few a refusal names.
                        ledgers_changed = Some(in_batch_order(
                            ledgers.clone(),
                            &payload.vouchers,
                            |ledger| ledger.as_str(),
                        ));
                    }
                    ToolFailure::from(refusal.code().to_string())
                },
            )?;
            // A ledger switched to bill-wise since the build would take an entry
            // On Account unseen: only the ledgers the person approved may be
            // bill-wise now. Judged after the identity binding, so a ledger
            // replaced under its old name is reported as that, not as a flag.
            if !super::bill_wise::flags_still_as_approved(
                &catalogue_identities,
                &super::bill_wise::named_ledgers(&payload),
                line.on_account_approved.as_deref().unwrap_or_default(),
            ) {
                return Err("import_bill_wise_changed".to_string().into());
            }
            // A Payment, Receipt or Contra is only the right type while every
            // leg classifies as its build found it. The build's own check is
            // stale by now, so classify again before approval from this
            // catalogue's parents and a fresh group collection, and send the
            // group request with the approval so the queue re-reads both.
            // An invoice is only the right voucher while the masters it was
            // classified from still say the same. Read again before approval
            // (and again after it, before dispatch); refused unless the
            // observation equals the build's.
            if line
                .vouchers
                .iter()
                .any(|voucher| voucher.voucher_type.is_invoice())
            {
                let (evidence, plan) = Box::pin(self.recheck_sales_invoice(
                    &identity,
                    &company,
                    &line.vouchers[0],
                    &catalogue_identities,
                ))
                .await
                .map_err(|failure| failure.with_prior_evidence(accumulated.clone()))?;
                accumulated = combine_evidence(accumulated.clone(), evidence);
                invoice_plan = Some(plan);
            }
            let group_collection_request = if renders_bank_shape(&payload.vouchers) {
                let (groups, evidence) =
                    self.read_group_collection(&identity, &company.name).await?;
                accumulated = combine_evidence(accumulated.clone(), evidence);
                let observed =
                    ObservedMasters::new(catalogue_identities.catalog().parents(), groups);
                if cash_bank_refusals(&payload, &observed, RECHECK_REFUSAL_BUDGET).is_refused() {
                    return Err("import_bank_classification_changed".to_string().into());
                }
                // A ledger a cash answer named as cash in hand must still be
                // one: under Bank Accounts its Contra moves the cash bank to
                // bank, which passes the gate above (#815).
                // Bounded as the queue bounds them, so both answers list the
                // same rows for one regroup.
                if let Some(refused) = super::cash_in_hand_refusals(
                    line.cash_in_hand_ledgers.as_deref().unwrap_or_default(),
                    &observed,
                    RECHECK_REFUSAL_BUDGET,
                ) {
                    cash_in_hand_refused = Some(refused);
                    return Err("cash_ledger_not_cash_in_hand".to_string().into());
                }
                Some(
                    crate::tally::agent_read_request::AgentReadRequest::parse(
                        native_group_snapshot_read(&company.name).into_xml(),
                    )
                    .map_err(|error| error.to_string())?,
                )
            } else if line
                .cash_in_hand_ledgers
                .as_deref()
                .is_some_and(|recorded| !recorded.is_empty())
            {
                // A cash-in-hand ledger rides only on a business-cash Contra,
                // a bank voucher: one recorded without one would skip the
                // check above, so the record is refused as inconsistent (#815).
                return Err("import_post_admission_inconsistent".to_string().into());
            } else {
                None
            };
            // Bridge's amounts are plain base-currency figures, so a post goes
            // only into a book with exactly one Currency master (bridge#551).
            // Read before approval, so a refused book is never asked about, and
            // sent with the approval so the queue reads it again.
            let (currencies, evidence) = self
                .post_read(&identity, company_currency_read(&company.name))
                .await?;
            accumulated = combine_evidence(accumulated.clone(), evidence);
            admit_post_currency(&currencies).map_err(|refusal| {
                currencies_seen = refused_currencies(&refusal);
                ToolFailure::from(refusal.to_string())
            })?;
            let currency_request = crate::tally::agent_read_request::AgentReadRequest::parse(
                company_currency_read(&company.name).into_xml(),
            )
            .map_err(|error| error.to_string())?;
            let mode = self.qualified_import_profile().await?;
            validate_post_profile_with_evidence(&payload, &mode, &mut accumulated)?;
            // Every voucher goes without a number, so each voucher type must be
            // numbered Automatic with existing numbers retained (bridge#724).
            // Read on every pass, a redeem included, after the master mark it
            // is bounded by; the queue compares that mark again before the POST.
            let numbering_admission = self
                .admit_numbering_for_post(&company, &identity, &payload, &mut accumulated)
                .await
                .map_err(|refusal| {
                    if let numbering::NumberingFailure::Refused(refused) = &refusal {
                        numbering_refused = Some((refused.refused_types(), refused.message()));
                    }
                    ToolFailure::from(refusal)
                })?;
            // Kept for the check across the post (#239).
            let approved_binding = ledger_binding.clone();
            let company_marks_request = crate::tally::agent_read_request::AgentReadRequest::parse(
                super::super::read_profiles::render_agent_company_high_water(&company.name),
            )
            .map_err(|error| error.to_string())?;
            // The approval this call spends, if it is one held across calls. It
            // lapses when this drops unspent, whatever way the call ends (#725).
            let mut redemption = None;
            let (request, native) = match scope {
                PostScope::JournalOnly => (
                    ApprovedImport::confirm_vouchers(
                        xml,
                        &preview,
                        voucher_dates,
                        verification_request,
                        ledger_catalogue_request,
                        ledger_binding.clone(),
                        group_collection_request,
                        currency_request,
                        company_marks_request.clone(),
                    )
                    .await?
                    .with_invoice(invoice_plan.clone()),
                    native,
                ),
                PostScope::Vouchers => {
                    // What the person is shown, and what it is about: an
                    // approval is redeemed only for exactly this (#725).
                    let binding = ApprovalBinding::new(&line, &preview, &ledger_binding, form);
                    if !redeeming {
                        let asked = PendingPostApproval::ask(
                            xml,
                            preview.clone(),
                            voucher_dates,
                            verification_request,
                            ledger_catalogue_request,
                            ledger_binding.clone(),
                            group_collection_request,
                            currency_request,
                            company_marks_request.clone(),
                            // A batch's box with the voucher lines that does
                            // not fit the screen is noted, so its next ask
                            // shows the totals (#1832). Nothing else is: not
                            // a totals box, nor a listed ask whose text
                            // already stood in for the voucher lines.
                            (form == PreviewForm::Listed && preview_lists_vouchers)
                                .then(|| self.post_approvals.note_listed_did_not_fit(batch_id)),
                        );
                        if let Some(step) = self
                            .ask_and_hold(
                                &ApprovalTarget {
                                    tool: "post_import",
                                    batch_id,
                                    guid,
                                    vouchers: line.vouchers.len(),
                                },
                                asked,
                                &binding,
                                native,
                                call_started,
                                &accumulated,
                            )
                            .await?
                        {
                            return Ok(step);
                        }
                    }
                    // The dialog may have stayed open: an invoice's masters are
                    // read once more before the approval is spent.
                    if line
                        .vouchers
                        .iter()
                        .any(|voucher| voucher.voucher_type.is_invoice())
                    {
                        // The catalogue read before the dialog is as old as the
                        // dialog: read it again, so the party's bill-wise flag
                        // is read after the answer too.
                        let (_, fresh_catalogue, _, catalogue_read) = self
                            .read_import_ledger_catalogue(&identity, &company.name)
                            .await
                            .map_err(|failure| failure.with_prior_evidence(accumulated.clone()))?;
                        accumulated = combine_evidence(accumulated.clone(), catalogue_read);
                        let (evidence, plan) = Box::pin(self.recheck_sales_invoice(
                            &identity,
                            &company,
                            &line.vouchers[0],
                            &fresh_catalogue,
                        ))
                        .await
                        .map_err(|failure| failure.with_prior_evidence(accumulated.clone()))?;
                        accumulated = combine_evidence(accumulated.clone(), evidence);
                        invoice_plan = Some(plan);
                    }
                    let (taken, request, held) =
                        self.post_approvals.take_for_dispatch(batch_id, &binding)?;
                    redemption = Some(taken);
                    // The binding was a voucher binding, so only a voucher
                    // request can come back; a master request here is a wiring
                    // fault, refused as a changed binding with the approval
                    // unspent (the redemption lapses as this returns).
                    let HeldRequest::Vouchers(native) = held else {
                        return Err("import_approval_binding_changed".to_string().into());
                    };
                    (request.with_invoice(invoice_plan.clone()), native)
                }
            };
            let redemption_id = redemption.as_ref().map(approval::Redemption::id);
            // The cross-process lease starts only after the independent native
            // approval. It covers intent, the one POST, its response append and
            // immediate readback; recovery remains the durable batch journal.
            let _endpoint_dispatch_lease = dispatch_lease::acquire(&self.settings.endpoint)?;
            // Before anything can be sent, so a crash, a concurrent reader or a
            // failed later write reads a doubt, never an absent record (#239).
            self.record_post_checks_pending(batch_id, line.vouchers.len() > 1)?;
            // The target's voucher mark in the aim snapshot, the last read before
            // the POST. The queued recheck sees that snapshot; the intent records
            // the mark, so binding the post to its own span never rests on a mark
            // read after the POST (agent_import_span_identity.rs).
            let aimed_mark = std::sync::Mutex::new(None::<u64>);
            // Read inside the endpoint's dispatch lease, so no other post of
            // this machine to this endpoint can be sent between this read and
            // the queued recheck that uses it.
            let invoice_identity = self.import_invoice_identity(&line)?;
            // And the number's control the journal offers now, inside the same
            // lease: the queue's judge reads the number against that control,
            // and a control that moved since the plan is a change (#1337).
            let invoice_control = self.queued_invoice_control(&line)?;
            let posted = self
                .runtime
                .post_approved_import(
                    self.tally_config(),
                    &identity,
                    request,
                    |queued| {
                        recheck_voucher_queue(
                            queued,
                            &line,
                            &identity,
                            &aimed_mark,
                            invoice_identity,
                            invoice_control.as_ref(),
                            &numbering_admission,
                        )
                    },
                    || {
                        // The file lock covers only the admission+synced append. It is not
                        // held over approval or network I/O. A competing process loses here.
                        let _lock = self
                            .lock_import_admission()
                            .map_err(BeforeDispatchError::Other)?;
                        let current = self
                            .import_snapshot_while_admitted(Some(batch_id))
                            .map_err(BeforeDispatchError::Other)?
                            .ok_or(BeforeDispatchError::Refused(
                                UnderLockRefusal::BatchNotFound,
                            ))?;
                        if current.dispatched {
                            return Err(BeforeDispatchError::Refused(
                                UnderLockRefusal::AlreadyAttempted,
                            ));
                        }
                        if self
                            .import_remote_ids_recorded_while_admitted(native.remote_ids.as_slice())
                            .map_err(BeforeDispatchError::Other)?
                        {
                            return Err(BeforeDispatchError::Refused(
                                UnderLockRefusal::RemoteIdReused,
                            ));
                        }
                        if current.batch.sha256 != line.sha256
                            || current.batch.endpoint_origin != line.endpoint_origin
                        {
                            return Err(BeforeDispatchError::Refused(
                                UnderLockRefusal::BatchChanged,
                            ));
                        }
                        if self
                            .import_rows_already_posted_while_admitted(&line)
                            .map_err(BeforeDispatchError::Other)?
                            .is_some()
                        {
                            return Err(BeforeDispatchError::Refused(
                                UnderLockRefusal::TxnAlreadyPosted,
                            ));
                        }
                        // An invoice waits for a sent invoice of this company that is
                        // not verified posted (ADR 0004, slice 4). Asked again here,
                        // under the lock, because the approval dialog may have stood
                        // open while another batch went out.
                        if self
                            .stopped_company_while_admitted(&line)
                            .map_err(BeforeDispatchError::Other)?
                        {
                            return Err(BeforeDispatchError::Refused(
                                UnderLockRefusal::CompanyStopped,
                            ));
                        }
                        // The transfer records again, between the check before the
                        // dialog and the intent: the approval is withdrawn and the
                        // next call gives the whole reason before asking.
                        if let Some(refusal) = self
                            .transfers_refusal_for_send_while_admitted(&current.batch)
                            .map_err(BeforeDispatchError::Other)?
                        {
                            return Err(BeforeDispatchError::Refused(UnderLockRefusal::Transfers(
                                refusal.reason(),
                            )));
                        }
                        // Before the approval is spent: a refusal here withdraws it,
                        // and the next call asks the person again.
                        let pre_post_voucher_mark =
                            aimed_mark.lock().ok().and_then(|aimed| *aimed).ok_or(
                                BeforeDispatchError::Refused(UnderLockRefusal::MarkUnrecorded),
                            )?;
                        // Spent here, once, under this lock and before the
                        // intent: a revoked approval (the call was cancelled)
                        // or a second redemption writes no intent (#725).
                        match (scope, redemption_id) {
                            (_, Some(id)) => self
                                .post_approvals
                                .spend(batch_id, id)
                                .map_err(BeforeDispatchError::Refused)?,
                            // Every agent post redeems an approval; one that
                            // reached here without one writes no intent.
                            (PostScope::Vouchers, None) => {
                                return Err(BeforeDispatchError::Refused(
                                    UnderLockRefusal::ApprovalRevoked,
                                ))
                            }
                            (PostScope::JournalOnly, None) => {}
                        }
                        // The append itself keeps the catch-all: it may have
                        // recorded an intent (#711).
                        self.append_import_record_while_admitted(
                            &ledger::StatusRecord::dispatch_for(
                                &line,
                                &native,
                                Some(pre_post_voucher_mark),
                            ),
                        )
                        .map_err(BeforeDispatchError::Other)
                    },
                )
                .await;
            // An approval this post did not spend lapses: it is never offered
            // again (#725).
            drop(redemption);
            // Whatever the outcome, the book may have changed: no ledger
            // listing of this company is continued from before it (#630).
            self.drop_listing_snapshots(identity.company_guid());
            let posted = posted.map_err(|error| {
                transfers_changed_under_lock = error.chain().any(|cause| {
                    matches!(
                        cause.downcast_ref::<UnderLockRefusal>(),
                        Some(UnderLockRefusal::Transfers(_))
                    )
                });
                currencies_seen = error
                    .chain()
                    .find_map(|cause| cause.downcast_ref::<ApprovedImportAdmissionError>())
                    .and_then(refused_currencies);
                cash_in_hand_refused = error
                    .chain()
                    .find_map(|cause| cause.downcast_ref::<ApprovedImportAdmissionError>())
                    .and_then(refused_cash_in_hand);
                // An invoice's masters, judged in the queue (#1337): the code the
                // judge refused under, or the master that moved.
                let invoice_refusal = error.chain().find_map(|cause| {
                    match cause.downcast_ref::<ApprovedImportAdmissionError>() {
                        Some(
                            ApprovedImportAdmissionError::InvoiceRefused { code }
                            | ApprovedImportAdmissionError::InvoiceReadUnusable { code },
                        ) => Some(code.clone()),
                        _ => None,
                    }
                });
                let invoice_field = error.chain().find_map(|cause| {
                    match cause.downcast_ref::<ApprovedImportAdmissionError>() {
                        Some(ApprovedImportAdmissionError::InvoiceMastersChanged { field }) => {
                            Some(*field)
                        }
                        _ => None,
                    }
                });
                let code = refusal_code(&error);
                let code = invoice_refusal.as_deref().unwrap_or(code);
                // A queue read that failed in transport has no typed cause of
                // its own; the transport's safe code names it.
                let transport = (code == "post_queue_read_failed")
                    .then(|| {
                        error.chain().find_map(|cause| {
                            cause
                                .downcast_ref::<bridge_tally_transport::TallyTransportError>()
                                .map(bridge_tally_transport::TallyTransportError::safe_code)
                        })
                    })
                    .flatten();
                // The queue's group re-read names why, as the read before
                // approval does (bridge#717).
                let group = error.chain().find_map(|cause| {
                    match cause.downcast_ref::<ApprovedImportAdmissionError>() {
                        Some(ApprovedImportAdmissionError::GroupExportInvalid { cause }) => *cause,
                        _ => None,
                    }
                });
                // A wire-gate refusal before the intent (#697) becomes the code
                // itself in `from_runtime`; it is not repeated as the cause.
                // The base currency a queued invoice read refused on (#1337),
                // which would otherwise come back as the generic queue read.
                let currency_cause = error.chain().find_map(|cause| {
                    cause
                        .downcast_ref::<crate::tally::runtime::CurrencyAdmissionRefusal>()
                        .map(|refusal| refusal.0)
                });
                let mut failure = ToolFailure::from_runtime(code, error);
                if failure.cause.is_none() {
                    failure.cause = refusal_cause(
                        &failure.code,
                        group.or(invoice_field).or(currency_cause),
                        transport,
                    );
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
            let parsed_outcome = parse_import_outcome(&posted.body).ok();
            let reported_created = parsed_outcome
                .as_ref()
                .map(|outcome| outcome.counters().created);
            let response = ledger::DispatchResponse {
                request_sha256: posted.response_evidence.request_sha256,
                response_sha256: posted.response_evidence.response_sha256,
                bytes: posted.response_evidence.bytes,
                outcome: parsed_outcome,
            };
            received_response = Some(response.clone());
            // The journal write is attempted first; its result is held, not
            // returned yet, so a local failure to journal still leaves the
            // location of a post that was sent (#574).
            let journaled = self.lock_import_admission().and_then(|_lock| {
                self.append_import_record_while_admitted(&ledger::StatusRecord::response(
                    &line, response,
                ))
            });
            // Where the voucher went (#574), read only once the journal write has
            // been attempted, so a slow or failed read delays nothing that records
            // the post. A failed read is reported, never guessed.
            let marks_read = self
                .read_marks_after_post(company_marks_request.clone(), call_started)
                .await;
            // The after mark the span is bound with, read as the aim mark is (#1945): an omitted
            // mark is none, so the span falls back to the post's own answer, never a zero.
            let after_post_mark = marks_read.as_ref().ok().and_then(|marks| {
                location::aimed_voucher_mark(marks, identity.company_guid(), &company.name)
            });
            let (marks_after, after_read_cause) = match marks_read {
                Ok(marks) => match location::parse_all_company_marks(&marks) {
                    Ok(_) if location::omits_voucher_mark(&marks, identity.company_guid()) => {
                        (None, Some("marks_readback_voucher_mark_omitted"))
                    }
                    Ok(rows) => (Some(rows), None),
                    Err(_) => (None, Some("marks_readback_unparsed")),
                },
                Err(error) => (None, Some(after_read_cause(&error))),
            };
            post_location = Some(location::classify_post_location(
                &location::parse_all_company_marks(&posted.company_marks_before)
                    .unwrap_or_default(),
                marks_after.as_deref(),
                identity.company_guid(),
                &company.name,
                reported_created,
            ));
            if let (Some(cause), Some(location)) = (after_read_cause, post_location.as_mut()) {
                location["after_read_failure"] = json!(cause);
            }
            // A batch is clean only if the target's voucher mark moved by
            // exactly what Tally created; recorded durably, before anything
            // else can fail, so no later readback can lose it.
            if line.vouchers.len() > 1 {
                if let Some(located) = &post_location {
                    self.record_batch_step_verdict_caused(
                        batch_id,
                        &located["target_voucher_step"],
                        located["after_read_failure"].as_str(),
                    );
                }
            }
            journaled?;
            // A valid counter response is evidence, never proof that Tally preserved
            // the requested ledger/amount/date semantics. Readback is mandatory.
            let masters_after_post = self
                .masters_after_post(
                    &posted.company_marks_before,
                    marks_after.as_deref(),
                    &identity,
                    &company.name,
                    &approved_binding,
                    &mut accumulated,
                )
                .await;
            // The verdict is recorded, beside the pending mark, before the readback,
            // so no later reconcile, which compares by name, can clear a doubt
            // (#239, #911).
            let masters_after_post = self.record_masters_verdict_for(
                batch_id,
                masters_after_post,
                line.vouchers.len() > 1,
            );
            // Ordered here for the answer of a failed readback; a verification
            // that succeeds orders its own.
            masters_verdict = Some(served_masters_verdict(
                masters_after_post.clone(),
                &line.vouchers,
            ));
            let mut proof = self
                .verify_import_after_current_dispatch(args, masters_after_post, after_post_mark)
                .await?;
            accumulated = combine_evidence(accumulated.clone(), proof.evidence.clone());
            proof.evidence = accumulated.clone();
            if let Some(located) = post_location.clone() {
                proof.payload["result"]["post_location"] = located;
            }
            // The send happened and was read back: say how the batch ended.
            if let Some(headline) = super::verification::batch_headline(&proof.payload["result"]) {
                proof.payload["headline"] = json!(headline);
            }
            Ok(Step::Done(Box::new(proof)))
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
        // later call asks the person again. One taken by this call has already
        // lapsed; a dialog still open or declined, one a call waits on, and another
        // batch's hold are left as they are.
        if scope == PostScope::Vouchers && operation.is_err() {
            self.post_approvals
                .revoke_unredeemed(batch_id, "post_refused_before_intent");
        }
        // Even failure after a lost response carries the saved identity. The next
        // call must reconcile that batch, never create a replacement business event.
        match operation {
            Ok(result) => Ok(Pass::Done(result)),
            Err(failure) => {
                let snapshot = self.latest_import_snapshot(batch_id).ok().flatten();
                let attempted = self.post_failure_attempt_observation(
                    batch_id,
                    guid,
                    snapshot.as_ref(),
                    received_response.as_ref(),
                );
                let cause = failure.cause;
                let mut outcome = post_failure_outcome(
                    batch_id,
                    guid,
                    failure,
                    accumulated,
                    snapshot.as_ref(),
                    received_response.as_ref(),
                    attempted,
                );
                // The typed, data-free reason, under the generic refusal's
                // budget rule; a post refusal used to drop it (bridge#634).
                if let Some(cause) = cause {
                    if self.settings.max_bytes >= crate::agent::REMEDIATION_MIN_RESPONSE_BUDGET {
                        outcome.payload["result"]["error"]["cause"] = json!(cause);
                    }
                }
                // The code's next step, under the same rule, and only when the
                // journal shows nothing was sent (#1573).
                // A numbering refusal's step, and its message, need more
                // room than the general floor gives.
                let guidance_floor = if numbering_refused.is_some() {
                    numbering::GUIDANCE_MIN_RESPONSE_BUDGET
                } else {
                    crate::agent::REMEDIATION_MIN_RESPONSE_BUDGET
                };
                let guidance_fits = self.settings.max_bytes >= guidance_floor;
                if let Some(nothing_sent) = NothingSent::observed(attempted) {
                    if guidance_fits {
                        attach_next_step(&mut outcome.payload, cause, nothing_sent);
                    }
                }
                // Name the earlier batch to verify. Both refusal paths land
                // here; the journal only grows, so a later read finds one unless a later
                // record of that batch replaced its row set, or the lock or the read failed
                // (both swallowed here), and then no batch is named.
                if outcome.payload["result"]["error"]["code"] == "import_txn_already_posted" {
                    let blocking = snapshot.as_ref().and_then(|current| {
                        let _lock = self.lock_import_admission_shared().ok()?;
                        self.import_rows_already_posted_while_admitted(&current.batch)
                            .ok()
                            .flatten()
                    });
                    name_blocking_batch(&mut outcome.payload, blocking.as_deref());
                }
                // The same for a stopped company, from the pre-dialog recheck
                // and the lock alike: name the batch the person must decide about.
                if outcome.payload["result"]["error"]["code"] == "invoice_company_stopped" {
                    let blocking = snapshot.as_ref().and_then(|current| {
                        self.import_invoice_stop(&current.batch.company_guid)
                            .ok()
                            .flatten()
                    });
                    if let Some(id) = blocking {
                        outcome.payload["result"]["error"]["blocking_batch_id"] = json!(id);
                    }
                }
                if let Some(located) = post_location {
                    outcome.payload["result"]["post_location"] = located;
                }
                if let Some(masters) = masters_verdict {
                    outcome.payload["result"]["masters_after_post"] = masters;
                    mark_verification_names(&mut outcome.payload["result"]);
                }
                if let Some(currencies) = currencies_seen {
                    name_refused_currencies(&mut outcome.payload, &currencies);
                }
                if let Some((refused, omitted)) = cash_in_hand_refused {
                    name_refused_cash_ledgers(&mut outcome.payload, refused, omitted);
                }
                if let Some((refused, message)) = numbering_refused {
                    let message = guidance_fits.then_some(message);
                    name_refused_voucher_types(&mut outcome.payload, refused, message);
                }
                // Withheld under the remediation budget, as `cause` is. That is
                // not a guarantee against the oversize answer just above it.
                if let Some(ids) = preexisting_txn_ids {
                    if self.settings.max_bytes >= crate::agent::REMEDIATION_MIN_RESPONSE_BUDGET {
                        name_preexisting_rows(
                            &mut outcome.payload,
                            &ids,
                            preexisting_invoice.as_ref(),
                        );
                    }
                }
                if self.settings.max_bytes >= crate::agent::REMEDIATION_MIN_RESPONSE_BUDGET {
                    if let Some((detail, step)) = &send_transfer_refusal {
                        name_transfer_refusal(&mut outcome.payload, detail, step);
                    }
                    if transfers_changed_under_lock {
                        outcome.payload["result"]["error"]["next_step"] =
                            json!(transfers::TRANSFERS_CHANGED_UNDER_LOCK_STEP);
                    }
                }
                if let Some(ledgers) = ledgers_changed {
                    name_changed_ledgers(&mut outcome.payload, &ledgers);
                } else {
                    explain_unbound_batch(&mut outcome.payload);
                }
                Ok(Pass::Done(outcome))
            }
        }
    }

    /// Reconciliation is read-only. Unlike `post_import`, it never reaches the
    /// approval dialog or dispatch path when a concurrent history change means
    /// there is no durable attempt to reconcile.
    pub(in crate::agent) async fn reconcile_import(
        &self,
        args: &Value,
        expected_sha256: &str,
    ) -> Result<ToolOutcome, ToolFailure> {
        let batch_id = required_string(args, "batch_id")?;
        let snapshot = {
            // An empty journal cannot rule out another process's pre-intent
            // admission. Observe it only while that dispatch lane is idle.
            let _lease = dispatch_lease::acquire_observation(&self.settings.endpoint)?;
            self.latest_import_snapshot(batch_id)?
                .ok_or_else(|| "import_batch_not_found".to_string())?
        };
        if snapshot.batch.sha256 != expected_sha256 {
            return Err("import_batch_changed".to_string().into());
        }
        if !snapshot.dispatched {
            let origin = super::super::canonical_loopback_origin(&self.settings.endpoint)
                .map_err(|_| "host_setting_invalid".to_string())?;
            if snapshot.batch.endpoint_origin.as_deref() != Some(origin.as_str()) {
                return Err("import_post_endpoint_mismatch".to_string().into());
            }
            return Err("import_not_dispatched".to_string().into());
        }
        self.reconcile_dispatched_import(args, snapshot).await
    }

    async fn reconcile_dispatched_import(
        &self,
        args: &Value,
        snapshot: ledger::BatchSnapshot,
    ) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        if !batch_guid_matches(&snapshot.batch.company_guid, guid) {
            return Err("import_batch_company_mismatch".to_string().into());
        }
        // Retain batch and endpoint integrity without applying approval-only
        // display restrictions or constructing an approval request.
        let _ = admit_saved_journal_integrity(&snapshot.batch, &self.settings.endpoint)?;
        // This entry is the desktop's alone, and so is this refusal.
        refuse_a_line_break_name_on_the_desktop(&snapshot.batch)?;
        self.verify_import(args).await
    }

    pub(super) fn post_failure_attempt_observation(
        &self,
        batch_id: &str,
        company_guid: &str,
        snapshot: Option<&ledger::BatchSnapshot>,
        received_response: Option<&ledger::DispatchResponse>,
    ) -> Option<bool> {
        if received_response.is_some() || snapshot.is_some_and(|snapshot| snapshot.dispatched) {
            return Some(true);
        }
        let snapshot = snapshot?;
        if !batch_guid_matches(&snapshot.batch.company_guid, company_guid) {
            return None;
        }
        let origin = super::super::canonical_loopback_origin(&self.settings.endpoint).ok()?;
        if snapshot.batch.endpoint_origin.as_deref() != Some(origin.as_str()) {
            return None;
        }
        let _lease = dispatch_lease::acquire_observation(&self.settings.endpoint).ok()?;
        self.latest_import_snapshot(batch_id)
            .ok()
            .flatten()
            .filter(|snapshot| {
                batch_guid_matches(&snapshot.batch.company_guid, company_guid)
                    && (snapshot.dispatched
                        || snapshot.batch.endpoint_origin.as_deref() == Some(origin.as_str()))
            })
            .map(|snapshot| snapshot.dispatched)
    }
}

/// The queue's recheck of a voucher post: the batch's vouchers against the
/// reads the queue made, then the aim, remembering the target's voucher mark
/// from the aim snapshot for the intent. A master admission handed to it is a
/// wiring fault, refused with a typed error and never posted.
#[allow(clippy::too_many_arguments)]
fn recheck_voucher_queue(
    queued: QueuedAdmission<'_>,
    line: &ImportLedgerLine,
    identity: &crate::tally::VerifiedCompanyIdentity,
    aimed_mark: &std::sync::Mutex<Option<u64>>,
    invoice_identity: InvoiceIdentity,
    invoice_control: Option<&ledger::NumberControl>,
    numbering: &numbering::NumberingAdmission,
) -> anyhow::Result<()> {
    let QueuedAdmission::Vouchers {
        first,
        second,
        catalogue,
        groups,
        currencies,
        company_marks_at_binding,
        company_marks,
        ledger_binding,
        invoice,
    } = queued
    else {
        return Err(ApprovedImportAdmissionError::AdmissionInconsistent.into());
    };
    let (company_guid, company_name) = (identity.company_guid(), identity.display_name());
    recheck_import_admission(
        line,
        identity,
        first,
        second,
        catalogue,
        groups,
        currencies,
        ledger_binding,
        invoice_identity,
        invoice,
        invoice_control,
    )?;
    admit_queued_voucher_aim(
        company_marks_at_binding,
        company_marks,
        company_guid,
        company_name,
        numbering,
    )?;
    let mark = location::aimed_voucher_mark(company_marks, company_guid, company_name);
    if let Ok(mut aimed) = aimed_mark.lock() {
        *aimed = mark;
    }
    Ok(())
}

/// The code a refused or failed approved post reports, from the typed cause
/// in its error chain. The first cause that has a rung names it; a refusal
/// that was made before the intent and has no rung of its own is
/// `post_queue_read_failed`, and anything else is an unknown outcome, so a
/// cause added without a rung here is reported as one of those two rather
/// than as itself. Both a voucher post and a master post end here.
pub(super) fn refusal_code(error: &anyhow::Error) -> &'static str {
    if error
        .chain()
        .any(|cause| cause.is::<crate::tally::approved_import::AmbiguousImportCompany>())
    {
        "import_company_scope_ambiguous"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::EducationVoucherDateUnsupported)
        )
    }) {
        "education_voucher_date_unsupported"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::PreexistingIdentity)
        )
    }) {
        "import_preexisting_identity"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::LedgerIdentityChanged)
        )
    }) {
        "import_masters_changed"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::BankClassificationChanged)
        )
    }) {
        "import_bank_classification_changed"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::InvoiceMastersChanged { .. })
        )
    }) {
        "import_invoice_masters_changed"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::CashLedgerNotCashInHand { .. })
        )
    }) {
        "cash_ledger_not_cash_in_hand"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::CashLedgersNotRecorded)
        )
    }) {
        CASH_LEDGERS_NOT_RECORDED
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::BillWiseNotRecorded)
        )
    }) {
        super::bill_wise::BILL_WISE_NOT_RECORDED
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::BillWiseChanged)
        )
    }) {
        "import_bill_wise_changed"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::LedgerFoldedTwin)
        )
    }) {
        "ledger_has_folded_twin"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::AdmissionInconsistent)
        )
    }) {
        "import_post_admission_inconsistent"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::CompanyScopeChanged)
        )
    }) {
        "post_company_scope_changed"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::CompanyScopeUnconfirmed)
        )
    }) {
        "post_company_scope_unconfirmed"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::MultiCurrencyBook { .. })
        )
    }) {
        "import_multi_currency_unsupported"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::BaseCurrencyUndetermined)
        )
    }) {
        "import_base_currency_undetermined"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::MastersMoved)
        )
    }) {
        "post_masters_moved"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::MastersUnconfirmed)
        )
    }) {
        "post_masters_unconfirmed"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::CatalogueUnreadable(_))
        )
    }) {
        "post_catalogue_unreadable"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::GroupExportInvalid { .. })
        )
    }) {
        "group_export_invalid"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::MasterWritesNeedLicensedMode)
        )
    }) {
        "master_writes_need_licensed_mode"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::MasterNameTaken { .. })
        )
    }) {
        "master_name_taken"
    } else if error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::MasterLedgerIndexUnavailable { .. })
        )
    }) {
        "master_ledger_index_unavailable"
    } else if let Some(refusal) = error
        .chain()
        .find_map(|cause| cause.downcast_ref::<UnderLockRefusal>())
    {
        // Refused under the admission lock before the intent was
        // appended (#711): nothing was recorded or sent.
        refusal.code()
    } else if error
        .chain()
        .any(|cause| cause.is::<crate::tally::approved_import::PreIntentQueueRefusal>())
    {
        // Refused in the queue before the intent (#656): nothing
        // was sent, so the outcome is known.
        "post_queue_read_failed"
    } else {
        "import_dispatch_outcome_unknown"
    }
}

fn post_failure_outcome(
    batch_id: &str,
    guid: &str,
    failure: ToolFailure,
    accumulated: Evidence,
    snapshot: Option<&ledger::BatchSnapshot>,
    received_response: Option<&ledger::DispatchResponse>,
    attempted: Option<bool>,
) -> ToolOutcome {
    let persisted_response = snapshot.and_then(|snapshot| snapshot.response.as_ref());
    let response = received_response.or(persisted_response);
    let mut evidence = failure
        .evidence
        .map(|item| combine_evidence(accumulated.clone(), *item))
        .unwrap_or(accumulated);
    evidence.state = "partial";
    evidence.reason_code = Some(failure.code.clone());
    ToolOutcome {
        payload: reconciliation_failure_payload(batch_id, attempted, response, &failure.code),
        evidence,
        company_guid: Some(guid.to_string()),
        truncated: false,
    }
}
/// The cause a failure adds to its code: none when it would only repeat the
/// code, as a wire refusal's transport code does once `from_runtime` made it the
/// code itself (#697).
pub(super) fn refusal_cause(
    code: &str,
    group: Option<&'static str>,
    transport: Option<&'static str>,
) -> Option<&'static str> {
    group.or(transport).filter(|cause| code != *cause)
}

/// What a caller does when the port was busy for the readback after its own
/// send (#697). Tally may or may not have accepted it; only the proof is missing.
const BUSY_AFTER_POST_NEXT_STEP: &str = "The post was already sent and only its readback was held back. Call verify_import with this original batch after retry_after_s seconds. Never rebuild the batch and never call post_import again.";

/// What a caller does when the port was busy before any attempt was recorded
/// (#869): nothing was sent, and an approval taken for this post has lapsed, so
/// the next call asks the person again.
const BUSY_BEFORE_ATTEMPT_NEXT_STEP: &str = "Nothing was posted: Tally's port was busy. Call post_import with this same batch again after retry_after_s seconds, once Tally is free. Any approval already given has lapsed, so the person is asked to approve it again. Do not rebuild the batch.";

/// The same, when Tally was still busy with an earlier request cut for time
/// (#2142): nothing was posted, and the approval has lapsed.
const DRAINING_BEFORE_ATTEMPT_NEXT_STEP: &str = "Nothing was posted: Tally was still busy with an earlier request that was stopped for time. Call post_import with this same batch again after retry_after_s seconds; this clears by itself within 10 minutes of the earlier request. Any approval already given has lapsed, so the person is asked to approve it again. Do not rebuild the batch.";

/// What a caller does when the port was busy and whether the post was sent
/// could not be observed.
const BUSY_UNKNOWN_ATTEMPT_NEXT_STEP: &str = "Whether the post was sent could not be observed. Call verify_import with this original batch after retry_after_s seconds. Never rebuild the batch and never call post_import again before it says the batch is not in Tally.";

/// What a caller does when an earlier invoice of the company stops this one
/// (ADR 0004, slice 4): only a verified read or a person's release lifts it.
const STOPPED_NEXT_STEP: &str = "Nothing was sent and the approval is withdrawn. blocking_batch_id names the earlier invoice batch (when it is absent, call verify_import on the company's recent invoice batches). Call verify_import on it: if it reads posted and verified, post this invoice again (a batch whose answer was lost or was not clean may never read so). If it does not, ask the user to check that invoice in Tally; once they have, call acknowledge_post_review with that batch_id and doubt invoice_stop, which asks them in a dialog you cannot answer. Never rebuild an invoice to get past this.";

/// What a caller does when another batch already sent, or was found to have
/// posted, a row of this one (#876). Tally's counters for a rejected send are
/// not proof that the row is absent now, so Bridge never lifts the block itself.
const TXN_ALREADY_POSTED_NEXT_STEP: &str = "Nothing was sent. Another batch of this company already went to Tally with this row, or was found posted. Call verify_import with that earlier batch (blocking_batch_id names it; when it is absent, verify the company's recent batches). If it finds the voucher, a row with a statement id (st-, from a bank-statement build) is the same bank row whatever ledger it names: do not post it again, and correct the posted voucher in Tally if its ledger is wrong. A hand-typed id can repeat: this row matched because the id, date and amounts are equal (or an amount could not be read), and that is either the same transaction, already in the book, or a different real transaction that shares them. Do not decide which yourself: ask the user to open the existing voucher in Tally, compare it with this row, and say which. If it is the same transaction and its ledger or narration is wrong, correct the posted voucher in Tally (or amend a batch that was imported by hand); if it is a second real transaction that is not in the book, rebuild that voucher under a new bridge_txn_id. ComplyEaze Bridge does not check the user's answer, and for a posted_verified voucher verify_import returns no date, amounts, ledgers or narration. Never rename a statement row this way: a statement row entered under any other id is not seen. For an invoice, its number identifies it, not its bridge_txn_id. If the blocking batch was released (acknowledge_post_review, doubt invoice_stop) and that release reported seen_at_release found, the invoice is in Tally: do not build it again. If it was released and its read did not report it found, build the invoice again under a new bridge_txn_id and the SAME invoice number: before the approval ComplyEaze Bridge refuses the post if Tally holds a voucher of the invoice's type with that number on its date or, while the earlier batch is unverified, one with those figures under any number, and the build refuses the number where its read of the financial year works on that book. Never also tell the user to enter that invoice in Tally by hand: a retry after a hand entry is how a sale is booked twice. For any other voucher: if Tally rejected that batch and the voucher is not in Tally, ComplyEaze Bridge cannot post this row again, and nothing in ComplyEaze Bridge puts it in the book yet: tell the user so. The same holds when verify_import reports that batch book_rolled_back: the company's voucher mark was seen below that batch's, which a book restored from a backup or replaced by another copy shows, and its voucher may be gone, but ComplyEaze Bridge cannot tell whether it is, so tell the user that too. For an overlapping statement, rebuild without the rows already posted. Never rebuild a row to retry it, except an invoice released as described above.";

/// What a caller is told when `post_import` refuses a saved batch's own text
/// before any attempt (#1055).
const TEXT_REFUSED_MESSAGE: &str = "Nothing was sent. This saved batch has a narration, reference or voucher number that cannot be posted, and a saved batch cannot be changed. Correct the text, build the batch again, then post the new one.";

/// What a caller is told when the approval window for a saved batch was too
/// large to show whole, by the batch's length or for the screen it would open
/// on, before any attempt (#1780). The desktop's Journal screen shows the
/// message alone.
const REVIEW_TOO_LARGE_MESSAGE: &str = "Nothing was sent. The approval window for this saved batch is too large to show whole, either because of its length or because of the screen it would open on, so it was not shown. A larger or less magnified screen may help. If the window listed the batch's vouchers one line each, asking again soon shows its totals only, which is smaller. If the length is the cause, a batch can be split into parts; a single voucher this long cannot be posted from here.";

fn name_blocking_batch(payload: &mut Value, blocking: Option<&str>) {
    let Some(id) = blocking else { return };
    payload["result"]["error"]["blocking_batch_id"] = json!(id);
    payload["result"]["error"]["next_step"] = json!(format!(
        "{TXN_ALREADY_POSTED_NEXT_STEP} The earlier batch is {id}."
    ));
}

fn reconciliation_failure_payload(
    batch_id: &str,
    attempted: Option<bool>,
    response: Option<&ledger::DispatchResponse>,
    code: &str,
) -> Value {
    let mut payload = json!({"result":{"batch_id":batch_id,"attempt_recorded":attempted,"dispatch_response":response,"error":{"code":code,
        "message":if attempted == Some(false) { "No posting attempt was recorded. Review the error before requesting approval again." }
        else { "The saved batch requires reconciliation. Use verify_import with this original batch; never rebuild it to retry." }}}});
    // Every busy refusal says when to retry. A recorded attempt makes it the
    // readback after a send; an unknown attempt (`None`) gets the same step in
    // words that do not claim a send, since verify_import is right either way.
    // A recorded non-attempt (`Some(false)`) offers no step: its message says
    // no attempt was recorded.
    if code == "import_txn_already_posted" {
        payload["result"]["error"]["message"] = json!("Nothing was sent: another batch of this company already sent, or was found to have posted, a row of this batch.");
        payload["result"]["error"]["next_step"] = json!(TXN_ALREADY_POSTED_NEXT_STEP);
    }
    if code == "invoice_company_stopped" {
        payload["result"]["error"]["message"] = json!("Nothing was sent: an earlier invoice of this company was sent to Tally and is not verified posted, so no further invoice is posted until a person decides about it.");
        payload["result"]["error"]["next_step"] = json!(STOPPED_NEXT_STEP);
    }
    // The saved text itself is refused, and a saved batch cannot be changed,
    // so asking for approval again always fails the same way: it has to be
    // built again (#1055). Only with no attempt recorded: a batch that may
    // have been sent is reconciled, never rebuilt.
    if code == "voucher_text_invalid" && attempted == Some(false) {
        payload["result"]["error"]["message"] = json!(TEXT_REFUSED_MESSAGE);
    }
    // The window for the batch was too large to show whole. Asking again soon
    // shows a window that listed the vouchers as totals only (#1832), and does
    // not shrink any other, so the message says what does.
    if code == "import_review_too_large" && attempted == Some(false) {
        payload["result"]["error"]["message"] = json!(REVIEW_TOO_LARGE_MESSAGE);
    }
    // A transfer check refused the batch, and it refuses again on every request
    // until the batch is built again: the desktop is told so, not to review.
    if attempted == Some(false) && transfers::SEND_REFUSALS.contains(&code) {
        payload["result"]["error"]["message"] = json!(if code == "transfer_view_not_established" {
            transfers::TRANSFER_VIEW_REFUSED_MESSAGE
        } else {
            transfers::TRANSFER_REFUSED_MESSAGE
        });
    }
    if code == DESKTOP_LEDGER_LINE_BREAK {
        payload["result"]["error"]["message"] = json!(DESKTOP_LEDGER_LINE_BREAK_ACTION);
    }
    // A second journal (#2233) refuses every post from this copy, so the
    // message says where to post from rather than to review the error.
    if code == super::split_journal::IMPORT_JOURNAL_SPLIT && attempted == Some(false) {
        payload["result"]["error"]["message"] = json!(super::split_journal::REFUSED_MESSAGE);
    }
    // A draining endpoint (#2142) is held back like a busy one: nothing was
    // sent by the refused send, and the same next steps hold.
    if let Some(after) = crate::endpoint_wire::wire_retry_after_s(code) {
        payload["result"]["error"]["retry_after_s"] = json!(after);
        match attempted {
            Some(true) => {
                payload["result"]["error"]["next_step"] = json!(BUSY_AFTER_POST_NEXT_STEP)
            }
            None => payload["result"]["error"]["next_step"] = json!(BUSY_UNKNOWN_ATTEMPT_NEXT_STEP),
            Some(false) => {
                payload["result"]["error"]["next_step"] = json!(match code {
                    "tally_endpoint_draining" => DRAINING_BEFORE_ATTEMPT_NEXT_STEP,
                    _ => BUSY_BEFORE_ATTEMPT_NEXT_STEP,
                })
            }
        }
    }
    payload
}

/// That a refused post sent nothing to Tally: the batch's journal, re-read by
/// [`Server::post_failure_attempt_observation`] under the dispatch
/// observation lease, records no dispatch of it (#1573). A next step is
/// attached only with one, so no refusal after a send, or after a send that
/// could not be ruled out, is ever told what to try next.
#[derive(Clone, Copy, Debug)]
pub(super) struct NothingSent(());

impl NothingSent {
    /// One only from the observation's `Some(false)`; a send (`Some(true)`)
    /// or an unknown (`None`) gives none.
    pub(super) fn observed(attempted: Option<bool>) -> Option<Self> {
        (attempted == Some(false)).then_some(Self(()))
    }
}

/// Adds the general next step, the code's else its cause's (as every other
/// tool's refusal is given), as `error.next_step` to a post refusal that sent
/// nothing, unless the post already set its own (a busy port, a row
/// another batch posted) or its message already is the step (a saved text that
/// cannot be posted, #1055). Before #1573 a code raised inside the post had a
/// next step no caller was ever shown.
pub(super) fn attach_next_step(
    payload: &mut Value,
    cause: Option<&str>,
    _nothing_sent: NothingSent,
) {
    let error = &mut payload["result"]["error"];
    if error.get("next_step").is_some() {
        return;
    }
    let Some(code) = error["code"].as_str() else {
        return;
    };
    if code == "voucher_text_invalid" {
        return;
    }
    if let Some(step) = crate::agent::remediation_for(code, cause) {
        error["next_step"] = json!(step);
    }
}

fn mark_reconciliation_required(payload: &mut Value) {
    payload["result"]["error"] = json!({"code":"import_reconciliation_required", "message":"The saved attempt has not been confirmed as the intended new voucher. Reconcile this original batch without resending it."});
}

/// A clean response creates exactly the batch's `voucher_count` vouchers and
/// nothing else.
pub(super) fn import_outcome_is_clean(
    outcome: Option<&bridge_tally_protocol::TallyImportOutcome>,
    voucher_count: usize,
) -> bool {
    outcome.is_some_and(|outcome| {
        outcome.application_status() != TallyImportApplicationStatus::Failure
            && outcome.exceptions_were_reported()
            && outcome
                .counters()
                .is_clean_success_for(voucher_count as u64, 0, 0)
    })
}

fn persisted_response_is_clean(
    response: Option<&ledger::DispatchResponse>,
    voucher_count: usize,
) -> bool {
    response
        .is_some_and(|response| import_outcome_is_clean(response.outcome.as_ref(), voucher_count))
}

fn persisted_response_state(
    response: Option<&ledger::DispatchResponse>,
    voucher_count: usize,
) -> &'static str {
    match response {
        None => "response_missing",
        Some(response) if import_outcome_is_clean(response.outcome.as_ref(), voucher_count) => {
            "response_clean"
        }
        Some(_) => "response_not_clean",
    }
}

pub(super) fn finalize_previous_attempt_reconciliation(
    payload: &mut Value,
    response: Option<&ledger::DispatchResponse>,
    masters_after_post: Option<&Value>,
    voucher_count: usize,
) {
    let name_verified = verification_status(&payload["result"], voucher_count) == "posted_verified"
        && persisted_response_is_clean(response, voucher_count);
    // A doubt recorded when this batch was posted outlives the readback, which
    // compares by name and cannot clear it (#239).
    let doubt = post_doubt(masters_after_post, voucher_count);
    let reconciled = name_verified && doubt.is_none();
    payload["result"]["dispatch"] = json!({
        "state": if reconciled { "previous_attempt_reconciled" } else { "reconciliation_required" },
        "resent": false,
        "response_state": persisted_response_state(response, voucher_count),
        "response": response,
    });
    if !name_verified {
        mark_reconciliation_required(payload);
    } else if let Some((code, message)) = doubt {
        payload["result"]["error"] = json!({"code": code, "message": message});
    }
}

/// What a person does about a doubt across a post: one text, in each message
/// that asks for it.
macro_rules! masters_review {
    () => {
        "Review the voucher in Tally and correct it there if it went to the wrong ledger. It is already posted, so do not rebuild this event."
    };
}
const MASTERS_REVIEW: &str = masters_review!();

/// The message for a post whose ledgers no longer resolve to the masters
/// approved. The ledgers are named in `masters_after_post.ledgers`, where the
/// response's redaction reaches them, never in this text.
pub(super) const CHANGED_MASTERS_MESSAGE: &str = concat!(
    "Posted to Tally, but the ledgers in masters_after_post.ledgers no longer resolve to the master you approved. ",
    masters_review!()
);

/// Whether the masters check across a post leaves doubt that the voucher went
/// to the ledgers approved (#239), as the refusal code and plain message. Only
/// an unchanged resolution, or a mark proven unmoved, admits: any other state,
/// including one this build does not know, is doubt. `None` means no check was
/// made or recorded, which is not doubt.
pub(super) fn masters_doubt(masters_after_post: Option<&Value>) -> Option<(&'static str, String)> {
    let masters = masters_after_post?;
    let state = masters["state"].as_str().unwrap_or_default();
    if state == "unchanged" || (state == "not_checked" && masters["reason"] == "masters_unmoved") {
        // Clean on the masters; a batch's recorded step verdict may still
        // doubt, independently.
        return masters
            .get("batch_step")
            .and_then(|step| batch_step_doubt(Some(step)));
    }
    Some(if state == "posted_under_changed_masters" {
        (
            "posted_under_changed_masters",
            CHANGED_MASTERS_MESSAGE.to_string(),
        )
    } else {
        let again = if state == super::MASTERS_CHECK_PENDING || state == "check_unavailable" {
            " A later verify_import of this batch checks again."
        } else {
            ""
        };
        (
            "masters_after_post_unconfirmed",
            format!("Posted to Tally, but ComplyEaze Bridge could not confirm that its ledgers are still the masters you approved.{again} {MASTERS_REVIEW}"),
        )
    })
}

/// A batch's step doubt: Bridge has no record of the target's voucher mark
/// moving by exactly Tally's CREATED. It moved otherwise, could not be read,
/// or its verdict was never recorded or cannot be read. Both finalizers show
/// the message only when the saved response is clean (CREATED parsed, and
/// equal to the batch) and every voucher read back verified, so it may say
/// that Tally reported creating the batch and that the batch is posted.
/// Nothing in the step says which voucher, so the review is of the whole
/// batch.
fn batch_step_doubt(step: Option<&Value>) -> Option<(&'static str, String)> {
    if step.is_some_and(|step| step["state"] == "matched") {
        return None;
    }
    Some((
        "batch_step_unconfirmed",
        "Tally reported creating the batch, and every voucher reads back, but ComplyEaze Bridge did not confirm that this company's voucher mark moved by exactly that many: the mark moved by another amount or backwards, it could not be read, or the check did not finish. Another change may have been made in it while the batch was posting. Review the batch's vouchers in Tally. They are already posted, so do not rebuild this batch. Record that review with acknowledge_post_review: the record is kept on this computer and does not change the batch's status, so this notice stays.".to_string(),
    ))
}

/// Every doubt across the post: the masters check, and for a batch its step,
/// which must be recorded as matched.
pub(super) fn post_doubt(
    masters_after_post: Option<&Value>,
    voucher_count: usize,
) -> Option<(&'static str, String)> {
    masters_doubt(masters_after_post).or_else(|| {
        (voucher_count > 1)
            .then(|| {
                batch_step_doubt(masters_after_post.and_then(|masters| masters.get("batch_step")))
            })
            .flatten()
    })
}

pub(super) fn finalize_current_dispatch(
    payload: &mut Value,
    response: Option<&ledger::DispatchResponse>,
    masters_after_post: Option<&Value>,
    voucher_count: usize,
) {
    let verified = verification_status(&payload["result"], voucher_count) == "posted_verified";
    let clean = persisted_response_is_clean(response, voucher_count);
    let masters_doubt = post_doubt(masters_after_post, voucher_count);
    payload["result"]["dispatch"] = json!({
        "state": if clean && verified && masters_doubt.is_none() { "posted_verified" } else { "reconciliation_required" },
        "counters":response.and_then(|response| response.outcome.as_ref().map(|outcome| outcome.counters())),
        "application_status":response.and_then(|response| response.outcome.as_ref().map(|outcome| outcome.application_status())),
        "response_state": persisted_response_state(response, voucher_count),
        "response": response,
        "resent":false, "automatic_retry":false
    });
    if !clean || !verified {
        mark_reconciliation_required(payload);
    } else if let Some((code, message)) = masters_doubt {
        payload["result"]["error"] = json!({"code": code, "message": message});
    }
}

fn validate_post_profile_with_evidence(
    payload: &ImportPayload,
    profile: &ImportProfileObservation,
    accumulated: &mut Evidence,
) -> Result<(), String> {
    *accumulated = combine_evidence(accumulated.clone(), profile.evidence.clone());
    validate_import_dates_for_profile(payload, profile)
}

/// Every one of the batch's `voucher_count` vouchers is absent from the book.
fn require_absent_verification_result(result: &Value, voucher_count: usize) -> Result<(), String> {
    if voucher_count == 0
        || result["counts"]["not_found"].as_u64() != Some(voucher_count as u64)
        || result["vouchers"].as_array().map(Vec::len) != Some(voucher_count)
    {
        return Err("import_preexisting_identity".into());
    }
    Ok(())
}

/// The transaction ids of the batch's vouchers the readback did not find absent
/// (#901): each already matches a voucher in the book that this batch did not
/// post.
fn present_txn_ids(result: &Value) -> Vec<String> {
    result["vouchers"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|voucher| voucher["status"] != "not_found")
        .filter_map(|voucher| voucher["bridge_txn_id"].as_str().map(str::to_owned))
        .collect()
}

/// What to do when a batch's rows are already in the book, or look like rows
/// that are (#901). It names no amount, ledger or narration.
const PREEXISTING_ROWS_NEXT_STEP: &str = "Nothing was sent. The rows listed in error.preexisting_txn_ids each look like a voucher already in the book that this batch did not post: an earlier batch's, or one entered by hand. Rows with the same date, type, ledgers, amounts and sides match the same voucher, so no more of them are in the book than Tally holds vouchers: count them in Tally. Open the matching voucher and confirm it is a regular voucher (if it is optional or post-dated, ask the user what it should be, and leave the row out until then) and the same bank row as the statement's. If it is, the row is in the book: leave it out. If you cannot find the voucher, do not enter the row by hand: build the batch again and ComplyEaze Bridge checks the book again; if the voucher is there it refuses again, and if no voucher with that fingerprint is there it can go on to approval. If it is refused again and you still cannot find the voucher, ask the user, and never change a row (its date, ledger, type or amount) to get it past this check. For a genuinely different transaction (the statement has more rows with this date, ledgers and amount than Tally holds vouchers) that shares the fingerprint of a voucher you have opened and confirmed, ComplyEaze Bridge cannot post the extra row yet: leave it out of this batch and tell the user so. Then build the other rows again without it so those post; a rebuilt batch can be refused again, naming rows this answer did not list. Cut inline batches on whole days, so same-day rows of one amount are not split across batches.";

/// The same refusal for an invoice, which is recognised by its number and not
/// by its figures (ADR 0004). It names no amount, ledger or narration.
const PREEXISTING_INVOICE_NEXT_STEP: &str = "Nothing was sent. A voucher already in the book looks like this invoice, in one or more of three ways. Open it in Tally first and confirm it is a regular voucher (not optional, post-dated or cancelled): if you cannot find it, do not enter the invoice by hand, ask the user. (1) It has this invoice's number under this voucher type. If it is this invoice, the invoice is in the book: do not build it again. If it is another invoice, tell the user the number is in use and ask which invoice number to use; never choose a number yourself. (2) It has no number and the same date, ledgers, amounts and sides. If it is this invoice, it is in the book: do not build it again. If it is another sale, a new number does not get past this check: tell the user and enter the invoice in Tally by hand. (3) error.unsettled_batch_id is present: an earlier batch from this computer with the same date, ledgers, amounts and sides was sent to Tally and has not been confirmed, so a voucher with those figures may be that batch's invoice whatever its number. If the voucher is that batch's invoice, it is in the book: do not build it again. If that batch's answer was received, verify_import on it can confirm it and end this refusal; that is not permission to build this invoice again if it is already in the book. If you have found the voucher and it is not that batch's invoice, or the user says this is another sale, ComplyEaze Bridge cannot post it while that batch is unconfirmed: enter it in Tally by hand. If error.unsettled_batch_unread is present instead, and no voucher in the book is this invoice by case (1) or (2), the check could not read the journal or the invoice, and its value says why: if it is import_admission_busy, another action of ComplyEaze Bridge holds the journal, so build the invoice again once; for any other value tell the user, and do not assume a batch exists. Apart from the new number the user chooses in case (1), never change a number, date, ledger, amount or side to get past this check.";

/// What the transfer check at send listed (the rows it names, never a value
/// from a record) and what to do about it.
fn name_transfer_refusal(payload: &mut Value, detail: &Value, step: &str) {
    let error = &mut payload["result"]["error"];
    if let (Some(error), Some(detail)) = (error.as_object_mut(), detail.as_object()) {
        error.extend(detail.clone());
        error.insert("next_step".into(), json!(step));
    }
}

/// Name the rows of the batch that are already in the book, with the way on.
/// `invoice` is set for a batch that holds an invoice, to the unsettled batch
/// of this machine with its figures when there is one.
fn name_preexisting_rows(
    payload: &mut Value,
    txn_ids: &[String],
    invoice: Option<&Result<Option<String>, String>>,
) {
    if txn_ids.is_empty() {
        return;
    }
    let error = &mut payload["result"]["error"];
    error["preexisting_txn_ids"] = json!(txn_ids);
    error["next_step"] = json!(match invoice {
        Some(_) => PREEXISTING_INVOICE_NEXT_STEP,
        None => PREEXISTING_ROWS_NEXT_STEP,
    });
    match invoice {
        Some(Ok(Some(batch_id))) => error["unsettled_batch_id"] = json!(batch_id),
        // The journal could not be read to say whether a batch is unconfirmed:
        // the refusal says so, never that there is none.
        Some(Err(code)) => error["unsettled_batch_unread"] = json!(code),
        Some(Ok(None)) | None => {}
    }
    // Set only at the check before the dialog, where nothing was sent, so the
    // generic "never rebuild it to retry" of an unobserved attempt would
    // contradict the step.
    error["message"] = json!(
        "Nothing was sent: rows of this batch already match vouchers in the book. \
         See error.next_step."
    );
}

/// The aim check on the snapshot the queue read last before the POST (#574).
/// The aim check (#574), then the master mark (#239): the target's ALTMSTID in
/// the aim snapshot must equal the one read as the queue's binding reads began.
/// A voucher post and a master post both make it; the binding snapshot's rows
/// are returned for the check only a voucher post adds
/// ([`admit_queued_voucher_aim`]).
pub(super) fn admit_queued_aim(
    marks_at_binding: &str,
    marks: &str,
    company_guid: &str,
    company_name: &str,
) -> anyhow::Result<Vec<location::LoadedCompanyMarks>> {
    let rows = location::parse_all_company_marks(marks)
        .map_err(|_| ApprovedImportAdmissionError::CompanyScopeUnconfirmed)?;
    location::admit_post_target(&rows, company_guid, company_name)
        .map_err(|_| ApprovedImportAdmissionError::CompanyScopeChanged)?;
    let at_binding = location::parse_all_company_marks(marks_at_binding)
        .map_err(|_| ApprovedImportAdmissionError::MastersUnconfirmed)?;
    match location::target_masters_unchanged(&at_binding, &rows, company_guid, company_name) {
        Some(true) => {}
        Some(false) => return Err(ApprovedImportAdmissionError::MastersMoved.into()),
        None => return Err(ApprovedImportAdmissionError::MastersUnconfirmed.into()),
    }
    Ok(at_binding)
}

/// [`admit_queued_aim`] for a voucher post, whose binding snapshot must also
/// equal the mark the voucher numbering was read under before the approval
/// (bridge#724), so no master moved from that read to the POST.
fn admit_queued_voucher_aim(
    marks_at_binding: &str,
    marks: &str,
    company_guid: &str,
    company_name: &str,
    numbering: &numbering::NumberingAdmission,
) -> anyhow::Result<()> {
    let at_binding = admit_queued_aim(marks_at_binding, marks, company_guid, company_name)?;
    numbering
        .admit_at_binding(&at_binding, company_guid, company_name)
        .map_err(Into::into)
}

#[allow(clippy::too_many_arguments)]
fn recheck_import_admission(
    line: &ImportLedgerLine,
    identity: &crate::tally::VerifiedCompanyIdentity,
    first: &str,
    second: &str,
    catalogue: &str,
    groups: Option<&str>,
    currencies: &str,
    ledger_binding: &bridge_tally_protocol::StandardLedgerCatalogBinding,
    invoice_identity: InvoiceIdentity,
    invoice: Option<crate::tally::approved_import::QueuedInvoice<'_>>,
    invoice_control: Option<&ledger::NumberControl>,
) -> anyhow::Result<()> {
    let (company_guid, company_name) = (identity.company_guid(), identity.display_name());
    // The ledgers the build found under Cash-in-Hand (#815): a record without
    // them predates the field and has nothing to check again.
    let cash_in_hand = line
        .cash_in_hand_ledgers
        .as_deref()
        .ok_or(ApprovedImportAdmissionError::CashLedgersNotRecorded)?;
    if line.on_account_approved.is_none() {
        return Err(ApprovedImportAdmissionError::BillWiseNotRecorded.into());
    }
    let observed = parse_import_vouchers(first, company_guid).map_err(anyhow::Error::msg)?;
    let corroboration = parse_import_vouchers(second, company_guid).map_err(anyhow::Error::msg)?;
    corroborate_verification_window(
        &observed,
        &corroboration,
        line.date_from.as_str(),
        line.date_to.as_str(),
    )
    .map_err(anyhow::Error::msg)?;
    let result = verify_batch_as(line, &observed, Attribution::Tag, invoice_identity)
        .map_err(anyhow::Error::msg)?;
    require_absent_verification_result(&result, line.vouchers.len()).map_err(|code| match code
        .as_str()
    {
        "import_preexisting_identity" => ApprovedImportAdmissionError::PreexistingIdentity.into(),
        _ => anyhow::Error::msg(code),
    })?;
    let catalogue = crate::tally::standard_ledger_catalog::parse_import_ledger_catalog_response(
        catalogue,
        company_name,
        company_guid,
    )
    .map_err(ApprovedImportAdmissionError::CatalogueUnreadable)?;
    if !ledger_binding.matches_catalog(catalogue.catalog()) {
        return Err(ApprovedImportAdmissionError::LedgerIdentityChanged.into());
    }
    let parents = catalogue.catalog();
    // Nor can the binding see a ledger added since approval that folds equal to
    // a named one, which Tally's import lookup could take for it (bridge#626).
    let named = line
        .vouchers
        .iter()
        .flat_map(|voucher| &voucher.entries)
        .map(|entry| entry.ledger.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if !folded_twins(&named, parents.parents(), parents.aliases()).is_empty() {
        return Err(ApprovedImportAdmissionError::LedgerFoldedTwin.into());
    }
    // A ledger switched to bill-wise since the build would take an entry On
    // Account unseen: only the ledgers the person approved may be bill-wise
    // now. The same catalogue read carries the flags (#1234).
    let approved = line.on_account_approved.as_deref().unwrap_or_default();
    // An invoice's New Ref party is not judged by the On Account approval (its
    // entry carries an allocation), but it must still be bill-wise: a ledger
    // switched off since the build would have Tally drop the New Ref unseen.
    let bill_named = line
        .vouchers
        .iter()
        .flat_map(|voucher| {
            let allocated = super::invoice::new_ref_party(voucher);
            voucher
                .entries
                .iter()
                .filter(move |entry| Some(entry.ledger.as_str()) != allocated)
        })
        .map(|entry| entry.ledger.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if !super::bill_wise::flags_still_as_approved(&catalogue, &bill_named, approved)
        || line
            .vouchers
            .iter()
            .filter_map(super::invoice::new_ref_party)
            .any(|party| super::invoice::party_bill_wise_in(&catalogue, party) != Some(true))
    {
        return Err(ApprovedImportAdmissionError::BillWiseChanged.into());
    }
    // An invoice's masters, read again in this queue, judged by the rule the
    // build judged them by and held to what the build recorded (#1337). A batch
    // that holds an invoice with no such reads, or reads with no invoice, is a
    // wiring fault: refused, never skipped.
    if let Some((voucher, queued, control)) =
        super::invoice::queued_invoice_for(&line.vouchers, invoice, invoice_control)?
    {
        let today = bridge_tally_core::TallyDate::parse(super::super::tally_host_today()).map_err(
            |_| ApprovedImportAdmissionError::InvoiceReadUnusable {
                code: "current_date_invalid".to_string(),
            },
        )?;
        super::invoice::admit_queued_invoice(
            voucher, identity, &catalogue, control, &today, &queued,
        )?;
    }
    // The binding above compares each ledger's name and GUID, not its parent,
    // so it cannot see a ledger or a group re-parented since approval. A bank
    // voucher's type rests on exactly that, so classify every leg again from
    // this catalogue's parents and the group collection read beside it.
    let bank = renders_bank_shape(&line.vouchers);
    match (bank, groups) {
        // A recorded cash-in-hand ledger without a bank voucher would skip the
        // check below, so it is refused as a wiring fault too (#815).
        (false, None) if cash_in_hand.is_empty() => {}
        (true, Some(groups)) => {
            let groups = parse_native_group_snapshot(groups, company_guid).map_err(|error| {
                ApprovedImportAdmissionError::GroupExportInvalid {
                    cause: crate::tally::approved_import::group_snapshot_cause(&error),
                }
            })?;
            let payload = ImportPayload {
                company_guid: line.company_guid.clone(),
                vouchers: line.vouchers.clone(),
                amends_batch_id: None,
            };
            let observed = ObservedMasters::new(parents.parents(), groups);
            if cash_bank_refusals(&payload, &observed, RECHECK_REFUSAL_BUDGET).is_refused() {
                return Err(ApprovedImportAdmissionError::BankClassificationChanged.into());
            }
            // Re-parenting a cash ledger under Bank Accounts keeps every leg
            // money, so it passes the gate above (#815).
            if let Some((refused, omitted)) =
                super::cash_in_hand_refusals(cash_in_hand, &observed, RECHECK_REFUSAL_BUDGET)
            {
                return Err(ApprovedImportAdmissionError::CashLedgerNotCashInHand {
                    refused,
                    omitted,
                }
                .into());
            }
        }
        // A bank voucher without its group read, or a Journal with one, is a
        // wiring fault; refuse rather than post on half a check.
        _ => return Err(ApprovedImportAdmissionError::AdmissionInconsistent.into()),
    }
    // A Currency master can be added while approval waits (bridge#551).
    admit_post_currency(currencies)?;
    Ok(())
}

/// Admit a post on the company's Currency masters (bridge#551). Bridge's
/// amounts are plain base-currency figures, and a foreign-currency ledger's
/// balance can read as a plain amount too, so only the ledger's own currency
/// tells them apart (TALLY_PROTOCOL_REFERENCE §8.2d). Among several masters
/// only the outstandings reads identify the base (§9.10a.2); the write path
/// does not compare a leg's currency with it yet, so a post goes only into a
/// book with exactly one. That every ledger of such a book is in the base is
/// an inference (a ledger's currency is one of the book's masters), not a
/// measurement. When the write path uses the identified base, each leg's
/// `CURRENCYNAME` is compared with it instead. A response that parses to no master, or
/// does not parse (a master without a NAME does not), is
/// `BaseCurrencyUndetermined`.
pub(super) fn admit_post_currency(currencies: &str) -> Result<(), ApprovedImportAdmissionError> {
    let currency = parse_company_currency(currencies)
        .map_err(|_| ApprovedImportAdmissionError::BaseCurrencyUndetermined)?;
    if BaseCurrencyName::of_single_master(&currency).is_some() {
        Ok(())
    } else if currency.currency_count > 1 {
        Err(ApprovedImportAdmissionError::MultiCurrencyBook {
            currencies: currency.names,
        })
    } else {
        Err(ApprovedImportAdmissionError::BaseCurrencyUndetermined)
    }
}

/// The code refusing a batch recorded before its cash-in-hand ledgers were
/// (#815). Like a batch built before ledger binding (#239), it is rebuilt.
const CASH_LEDGERS_NOT_RECORDED: &str = "import_batch_predates_cash_ledger_record";

fn refused_cash_in_hand(refusal: &ApprovedImportAdmissionError) -> Option<(Vec<Value>, usize)> {
    match refusal {
        ApprovedImportAdmissionError::CashLedgerNotCashInHand { refused, omitted } => {
            Some((refused.clone(), *omitted))
        }
        _ => None,
    }
}

/// List the recorded cash-in-hand ledgers that no longer reach Cash-in-Hand, as
/// the build's refusal lists them, and, where no attempt is recorded, say in
/// plain words what to do (#815). The message names no ledger itself.
fn name_refused_cash_ledgers(payload: &mut Value, refused: Vec<Value>, omitted: usize) {
    let error = &mut payload["result"]["error"];
    error["refused_ledgers"] = Value::Array(refused);
    error["refused_ledgers_omitted"] = json!(omitted);
    if payload["result"]["attempt_recorded"] == json!(false) {
        payload["result"]["error"]["message"] = json!(
            "A ledger named as cash in hand when this batch was built (listed in \
             error.refused_ledgers) is no longer under Cash-in-Hand: its group now reaches the \
             reserved group shown. Nothing was posted. Put the ledger back under Cash-in-Hand, \
             or build the batch again with the cash-in-hand ledger."
        );
    }
}

fn refused_currencies(refusal: &ApprovedImportAdmissionError) -> Option<Vec<String>> {
    match refusal {
        ApprovedImportAdmissionError::MultiCurrencyBook { currencies } => Some(currencies.clone()),
        _ => None,
    }
}

/// Why a saved batch's build-time ledger binding does not admit this post.
#[derive(Debug, PartialEq, Eq)]
enum BuildBindingRefusal {
    /// Built before Bridge recorded ledger identities: nothing to compare.
    Unbound,
    /// These ledgers now resolve to another GUID, or are not in the record.
    Changed(Vec<String>),
}

impl BuildBindingRefusal {
    fn code(&self) -> &'static str {
        match self {
            Self::Unbound => "import_batch_predates_ledger_binding",
            Self::Changed(_) => "import_masters_changed_since_build",
        }
    }
}

/// Every ledger the post binds now must carry the GUID the build bound it to,
/// compared as the catalogue binding compares GUIDs (ASCII case folded).
fn admit_build_binding(
    recorded: Option<&[BoundLedger]>,
    current: &bridge_tally_protocol::StandardLedgerCatalogBinding,
) -> Result<(), BuildBindingRefusal> {
    let recorded = recorded.ok_or(BuildBindingRefusal::Unbound)?;
    let changed = current
        .pairs()
        .filter(|(name, guid)| {
            !recorded
                .iter()
                .any(|bound| bound.name == *name && bound.guid.eq_ignore_ascii_case(guid))
        })
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>();
    if changed.is_empty() {
        Ok(())
    } else {
        Err(BuildBindingRefusal::Changed(changed))
    }
}

/// How many changed ledgers a refusal names; the rest are counted.
pub(super) const REFUSAL_LEDGERS_NAMED: usize = 8;

/// List the ledgers whose GUID changed since the build, each marked as a party
/// name so the response's redaction applies, and, where no attempt is
/// recorded, say in plain words what that means. The message refers to the
/// list and names no ledger itself.
fn name_changed_ledgers(payload: &mut Value, ledgers: &[String]) {
    let named = ledgers
        .iter()
        .take(REFUSAL_LEDGERS_NAMED)
        .map(|ledger| party_name(ledger.clone()))
        .collect::<Vec<_>>();
    let error = &mut payload["result"]["error"];
    error["ledgers_changed"] = json!(named);
    error["ledgers_changed_total"] = json!(ledgers.len());
    if payload["result"]["attempt_recorded"] == json!(false) {
        payload["result"]["error"]["message"] = json!(
            "A ledger this batch names (listed in error.ledgers_changed) is no longer the one it \
             was built against: the name now belongs to a different ledger in Tally. Nothing was \
             posted. Confirm which ledger you meant (it may now have another name) before \
             building the batch again."
        );
    }
}

/// Say plainly that a batch built before ledger identities were recorded must
/// be rebuilt, where no attempt is recorded. So must one built before its
/// cash-in-hand ledgers were recorded (#815).
fn explain_unbound_batch(payload: &mut Value) {
    if payload["result"]["error"]["code"] == json!(CASH_LEDGERS_NOT_RECORDED)
        && payload["result"]["attempt_recorded"] == json!(false)
    {
        payload["result"]["error"]["message"] = json!(
            "This batch was built before ComplyEaze Bridge recorded which of its ledgers must stay \
             under Cash-in-Hand, so it cannot be checked. Nothing was posted. Build the batch \
             again, then post the new batch."
        );
    }
    if payload["result"]["error"]["code"] == json!(super::bill_wise::BILL_WISE_NOT_RECORDED)
        && payload["result"]["attempt_recorded"] == json!(false)
    {
        payload["result"]["error"]["message"] = json!(
            "This batch was built before ComplyEaze Bridge began checking ledgers that keep \
             bills in Tally, so it cannot be checked. Nothing was posted. First check in Tally \
             whether its file was already imported by hand, since posting the rebuilt batch would import it a second time. \
             Then build the batch again and post the new batch."
        );
    }
    if payload["result"]["error"]["code"] == json!("import_bill_wise_changed")
        && payload["result"]["attempt_recorded"] == json!(false)
    {
        payload["result"]["error"]["message"] = json!(
            "A ledger in this batch now keeps bills in Tally, and the person did not approve \
             entries on it going On Account. Nothing was posted. Build the batch again: the \
             new build lists the ledger, and the person is asked whether its entries may post \
             On Account."
        );
    }
    if payload["result"]["error"]["code"] == json!("import_batch_predates_ledger_binding")
        && payload["result"]["attempt_recorded"] == json!(false)
    {
        payload["result"]["error"]["message"] = json!(
            "This batch was built before ComplyEaze Bridge recorded which ledgers it was built against, so \
             it cannot be checked. Nothing was posted. Build the batch again, then post the new \
             batch."
        );
    }
}

/// How many Currency masters a refusal names; the rest are counted.
const REFUSAL_CURRENCIES_NAMED: usize = 8;

/// Names the voucher types a numbering refusal refused (bridge#724), and,
/// while the journal shows nothing was sent, says so in the message: the
/// desktop's Journal screen shows the message alone. `message` is `None`
/// under a budget that does not hold it, and the refusal keeps its own.
fn name_refused_voucher_types(payload: &mut Value, refused: Value, message: Option<String>) {
    payload["result"]["error"]["refused_types"] = refused;
    if payload["result"]["attempt_recorded"] == json!(false) {
        if let Some(message) = message {
            payload["result"]["error"]["message"] = json!(message);
        }
    }
}

/// Say plainly why a multi-currency book was refused, naming the masters seen.
/// The message is replaced only where no attempt is recorded: any other state
/// keeps its instruction to reconcile.
fn name_refused_currencies(payload: &mut Value, currencies: &[String]) {
    let named = currencies
        .iter()
        .take(REFUSAL_CURRENCIES_NAMED)
        .cloned()
        .collect::<Vec<_>>();
    let error = &mut payload["result"]["error"];
    error["currencies_seen"] = json!(named);
    error["currencies_total"] = json!(currencies.len());
    if payload["result"]["attempt_recorded"] == json!(false) {
        let mut list = named.join(", ");
        if currencies.len() > named.len() {
            list.push_str(&format!(" and {} more", currencies.len() - named.len()));
        }
        payload["result"]["error"]["message"] = json!(format!(
            "This company has more than one currency defined ({list}); ComplyEaze Bridge does not post \
             into multi-currency books yet. Nothing was posted."
        ));
    }
}

#[cfg(test)]
tokio::task_local! {
    /// Test-only: the REMOTEID a post mints, so a test can make it one the
    /// journal already records.
    pub(super) static SCRIPTED_REMOTE_ID: Uuid;
    /// Test-only: the REMOTEIDs a batch post mints, one per voucher.
    pub(super) static SCRIPTED_REMOTE_IDS: Vec<Uuid>;
    /// Test-only: run between a call's two passes (#725 slice 2.0), as a
    /// cancel, an expiry or another route's post landing there would.
    pub(super) static BETWEEN_PASSES: std::sync::Arc<dyn Fn() + Send + Sync>;
    /// Test-only: the start each pass of a post call took its budgets from.
    pub(super) static CALL_STARTS: std::sync::Arc<std::sync::Mutex<Vec<std::time::Instant>>>;
    /// Test-only: one entry per marks readback attempt after a post (#998): `None`
    /// for the first read, on the operation's own budget, and the retry's budget.
    pub(super) static MARKS_READS:
        std::sync::Arc<std::sync::Mutex<Vec<Option<std::time::Duration>>>>;
    /// Test-only: the wire-wait budget each redeem-only pass ran on (#893).
    pub(super) static REDEEM_WIRE_BUDGETS:
        std::sync::Arc<std::sync::Mutex<Vec<std::time::Duration>>>;
    /// Test-only: how far into its call a redeem-only pass takes itself to
    /// start, so a test can reach the call's ceiling without waiting it out.
    pub(super) static REDEEM_PASS_ELAPSED: std::time::Duration;
}

/// A fresh random REMOTEID for one native post.
fn mint_remote_id() -> Uuid {
    #[cfg(test)]
    if let Ok(scripted) = SCRIPTED_REMOTE_ID.try_with(|id| *id) {
        return scripted;
    }
    Uuid::new_v4()
}

/// The fresh REMOTEIDs of one native post, one per voucher in order, all
/// distinct. They are minted together at dispatch time and never derived, so
/// none can repeat one Tally may have seen (protocol reference §9.3).
pub(super) struct RemoteIds(Vec<Uuid>);

impl RemoteIds {
    /// At least one id; a repeat, which a v4 UUID makes vanishingly rare, is
    /// discarded, and a run of repeats refuses rather than loop.
    pub(super) fn mint(count: usize) -> Result<Self, String> {
        if count == 0 {
            return Err("import_post_requires_one_voucher".into());
        }
        #[cfg(test)]
        if let Ok(scripted) = SCRIPTED_REMOTE_IDS.try_with(Clone::clone) {
            if scripted.len() == count {
                return Ok(Self(scripted));
            }
        }
        // No more than the journal will admit on read, so an intent is never
        // written that the next journal read would refuse.
        if count > ledger::MAX_BATCH_POST_VOUCHERS {
            return Err("import_post_batch_too_large".into());
        }
        let mut ids = Vec::with_capacity(count);
        let mut repeats = 0;
        while ids.len() < count {
            let id = mint_remote_id();
            if ids.contains(&id) {
                repeats += 1;
                if repeats > 3 {
                    return Err("import_remote_id_reused".into());
                }
            } else {
                ids.push(id);
            }
        }
        Ok(Self(ids))
    }

    #[cfg(test)]
    pub(super) fn from_ids(ids: Vec<Uuid>) -> Self {
        Self(ids)
    }

    pub(super) fn as_slice(&self) -> &[Uuid] {
        &self.0
    }
}

/// One native post: the request bytes, their wire digest, and the fresh
/// REMOTEIDs they carry, one per voucher. The post records `request_sha256`
/// and the REMOTEIDs together in its dispatch intent, before sending
/// (bridge#579).
pub(super) struct NativePostRequest {
    pub(super) xml: String,
    pub(super) request_sha256: String,
    pub(super) remote_ids: RemoteIds,
}

pub(super) fn native_post_request(
    line: &ImportLedgerLine,
    remote_ids: RemoteIds,
) -> Result<NativePostRequest, String> {
    let company = line
        .company
        .as_ref()
        .ok_or_else(|| "import_post_company_missing".to_string())?;
    if remote_ids.as_slice().len() != line.vouchers.len() {
        return Err("import_post_remote_ids_mismatch".into());
    }
    let xml = render_native_vouchers_xml(
        &company.name,
        line.vouchers
            .iter()
            .zip(remote_ids.as_slice().iter().copied()),
    );
    let request_sha256 = sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        &xml,
    ));
    Ok(NativePostRequest {
        xml,
        request_sha256,
        remote_ids,
    })
}

pub(super) fn require_native_numbering(voucher: &ImportVoucher) -> Result<(), String> {
    if voucher.voucher_number.is_some() {
        return Err("import_post_numbered_journal_unsupported".into());
    }
    Ok(())
}

pub(super) fn admit_saved_journal_integrity(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
) -> Result<String, String> {
    admit_saved_voucher_integrity(line, endpoint, PostScope::JournalOnly, 1)
}

/// `max_vouchers` is the batch cap on the MCP path while posting is on, and 1
/// for the desktop (`Server::post_voucher_limit`). Every voucher's type must
/// be one the scope admits.
pub(super) fn admit_saved_voucher_integrity(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
    scope: PostScope,
    max_vouchers: usize,
) -> Result<String, String> {
    if line.vouchers.is_empty()
        || (line.vouchers.len() > 1 && max_vouchers < 2)
        || !line
            .vouchers
            .iter()
            .all(|voucher| scope.admits(&voucher.voucher_type))
        || line.identity_scheme != Some(ImportIdentityScheme::BatchV1)
    {
        return Err(scope.refusal().into());
    }
    if line.vouchers.len() > max_vouchers.min(ledger::MAX_BATCH_POST_VOUCHERS) {
        return Err("import_post_batch_too_large".into());
    }
    // A native post uses a fresh private REMOTEID, so it can only create. An
    // amendment exists to alter a voucher already in the book in place.
    if line.amends_batch_id.is_some() {
        return Err("import_post_amendment_requires_file_import".into());
    }
    let origin =
        super::super::canonical_loopback_origin(endpoint).map_err(|_| "host_setting_invalid")?;
    if line.endpoint_origin.as_deref() != Some(origin.as_str()) {
        return Err("import_post_endpoint_mismatch".into());
    }
    let company = line.company.as_ref().ok_or("import_post_company_missing")?;
    let payload = ImportPayload {
        company_guid: line.company_guid.clone(),
        vouchers: line.vouchers.clone(),
        amends_batch_id: None,
    };
    validate_payload(payload)?;
    // An invoice renders only from what its build observed in Tally: one saved
    // without that is not postable, and is never rendered as an empty message.
    if line.vouchers.iter().any(|voucher| {
        voucher.voucher_type.is_invoice()
            && voucher
                .invoice
                .as_ref()
                .and_then(|detail| detail.observed.as_ref())
                .is_none()
    }) {
        return Err("import_invoice_not_observed".into());
    }
    // A saved voucher of a type that is not qualified is never posted, however
    // it came to be saved: the same list the build refuses on.
    super::refuse_unqualified_types(&line.vouchers, super::qualified_voucher_types())?;
    totals(&line.vouchers)?;
    let xml = render_import_xml(&company.name, &line.vouchers, line.identity_batch_id());
    if sha256_hex(xml.as_bytes()) != line.sha256 {
        return Err("import_batch_changed".into());
    }
    Ok(xml)
}

pub(super) fn admit_saved_journal(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
) -> Result<(String, String), String> {
    admit_saved_voucher(line, endpoint, PostScope::JournalOnly, 1)
}

pub(super) fn admit_saved_voucher(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
    scope: PostScope,
    max_vouchers: usize,
) -> Result<(String, String), String> {
    let xml = admit_saved_voucher_integrity(line, endpoint, scope, max_vouchers)?;
    let preview = review_preview_for(line, endpoint, scope)?;
    // A batch about to be posted, not one only to be reconciled: saved before
    // the build recorded its bill-wise approvals, it is rebuilt (#1234). The
    // integrity check above does not include this, so a dispatched batch of an
    // older build can still be reconciled. Content refusals above win, so a
    // rebuild is never advised for a batch that would be refused again.
    if line.on_account_approved.is_none() {
        return Err(super::bill_wise::BILL_WISE_NOT_RECORDED.into());
    }
    Ok((xml, preview))
}

/// What the approval must show about a bank voucher's legs: which side had to
/// be bank or cash, and that Bridge checks it again just before posting.
fn classification_review_line(voucher_type: &VoucherType) -> Option<&'static str> {
    match voucher_type {
        VoucherType::Journal => None,
        VoucherType::Payment => {
            Some("Checked in Tally: every Cr ledger is bank/cash; every Dr ledger holds no money.")
        }
        VoucherType::Receipt => {
            Some("Checked in Tally: every Dr ledger is bank/cash; every Cr ledger holds no money.")
        }
        VoucherType::Contra => Some("Checked in Tally: every ledger is bank/cash."),
        VoucherType::Sales => Some(INVOICE_CHECKED_LINE),
    }
}

fn admit_fresh_saved_voucher(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
) -> Result<String, String> {
    refuse_a_line_break_name_on_the_desktop(line)?;
    review_preview_with(line, endpoint, &[])
}

/// What the desktop answers for a saved Journal that names a ledger whose
/// name ends in a line break.
pub(super) const DESKTOP_LEDGER_LINE_BREAK: &str = "import_desktop_ledger_line_break";
/// What the desktop's post and reconcile say with that code: the message
/// and the remediation its review gives for the same Journal.
pub(super) const DESKTOP_LEDGER_LINE_BREAK_ACTION: &str = "This saved Journal names a ledger whose name in Tally ends in a line break, and this screen cannot show such a name. Use the assistant for this Journal: its approval dialog shows such a name with the line break written out, and it can check a Journal that was already posted. If this one was already posted, do not post it again.";

/// The desktop Journal screen shows a ledger's name as it is and has no way
/// to show a line break at its end, so the desktop takes no saved Journal
/// that names such a ledger: not to review or post, sent or not, and not
/// to reconcile once it was sent (bridge#626).
pub(super) fn refuse_a_line_break_name_on_the_desktop(
    line: &ImportLedgerLine,
) -> Result<(), String> {
    if line
        .vouchers
        .iter()
        .flat_map(|voucher| &voucher.entries)
        .any(|entry| entry.ledger.ends_with("\r\n"))
    {
        return Err(DESKTOP_LEDGER_LINE_BREAK.into());
    }
    Ok(())
}

/// When the agent's post happens, stated in its dialog (#725): the post may
/// come after the click, on the agent's next call, or not at all.
/// One line, so a multi-line voucher gives up as little of the dialog as can
/// be: every outcome is in it, since a post not made now or on the next ask in
/// the window, or one cancelled, refused or cut off by a restart (the approval
/// is held in memory), is never made. Exactly the 100-character line cap.
pub(super) fn agent_post_timing_lines() -> [String; 1] {
    [format!(
        "ComplyEaze Bridge posts now or if asked again within {} min, unless cancelled, refused or restarted.",
        approval::APPROVAL_TTL.as_secs() / 60
    )]
}

/// The preview a post of `scope` shows, and so whether it fits the dialog at
/// all: the one decision the build's eligibility and the post both use. An
/// agent's post says when it happens (#725); the desktop's, answered and posted
/// in one call, is unchanged.
pub(super) fn review_preview_for(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
    scope: PostScope,
) -> Result<String, String> {
    match scope {
        PostScope::JournalOnly => admit_fresh_saved_voucher(line, endpoint),
        PostScope::Vouchers => agent_review_preview(line, endpoint),
    }
}

/// [`review_preview_for`] in the form an ask shows (#1832). The desktop's post
/// shows one Journal and has no other form.
pub(super) fn review_preview_for_form(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
    scope: PostScope,
    form: PreviewForm,
) -> Result<ShownPreview, String> {
    match scope {
        PostScope::JournalOnly => {
            admit_fresh_saved_voucher(line, endpoint).map(|text| ShownPreview {
                text,
                lists_vouchers: false,
            })
        }
        PostScope::Vouchers => {
            review_text(line, endpoint, &agent_post_timing_lines(), form).map(|shown| {
                let lists_vouchers = shown.lists_vouchers();
                ShownPreview {
                    text: shown.into_string(),
                    lists_vouchers,
                }
            })
        }
    }
}

/// A preview and whether it lists the batch's vouchers one line each: what the
/// box shows, so that a box that did not fit is noted only when it listed them
/// (#1832).
pub(super) struct ShownPreview {
    pub(super) text: String,
    pub(super) lists_vouchers: bool,
}

/// The preview an agent's post shows: the voucher or batch, and when it is
/// posted, all inside the dialog's caps.
pub(super) fn agent_review_preview(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
) -> Result<String, String> {
    review_preview_with(line, endpoint, &agent_post_timing_lines())
}

#[path = "agent_import_post_on_account.rs"]
mod on_account;
#[path = "agent_import_post_voucher_text.rs"]
mod voucher_text;
pub(super) use on_account::OnAccountMarks;
#[cfg(test)]
pub(super) use on_account::ON_ACCOUNT_LEGEND;
use on_account::{reads_like_the_legend, ReviewText};
#[cfg(test)]
use on_account::{ON_ACCOUNT_MARK, ON_ACCOUNT_UNMARKED};
use voucher_text::VoucherText;
pub(super) use voucher_text::VoucherTextRefusal;

fn review_preview_with(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
    footer: &[String],
) -> Result<String, String> {
    review_preview_with_form(line, endpoint, footer, PreviewForm::Listed)
}

fn review_preview_with_form(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
    footer: &[String],
    form: PreviewForm,
) -> Result<String, String> {
    review_text(line, endpoint, footer, form).map(ReviewText::into_string)
}

fn review_text(
    line: &ImportLedgerLine,
    endpoint: &super::super::TallyEndpointConfig,
    footer: &[String],
    form: PreviewForm,
) -> Result<ReviewText, String> {
    let company = line.company.as_ref().ok_or("import_post_company_missing")?;
    let origin =
        super::super::canonical_loopback_origin(endpoint).map_err(|_| "host_setting_invalid")?;
    let (debit, credit) = totals(&line.vouchers)?;
    if line.vouchers.len() > 1 {
        return batch_review_text(line, company, &origin, &debit, &credit, footer, form);
    }
    let voucher = &line.vouchers[0];
    if voucher.voucher_type.is_invoice() {
        return invoice_review_preview(
            line,
            company,
            &origin,
            (debit.as_str(), credit.as_str()),
            footer,
        );
    }
    let ledgers = admit_review_text(
        std::iter::once(company.name.as_str())
            .chain(voucher.voucher_number.iter().map(String::as_str))
            .chain(voucher.reference.iter().map(String::as_str))
            .chain(voucher.narration.iter().map(String::as_str)),
        voucher.entries.iter().map(|entry| entry.ledger.as_str()),
    )
    .map_err(ReviewTextRefusal::import_code)?;
    require_native_numbering(voucher)?;
    let marks = OnAccountMarks::of(line);
    let quoted = |text: &str| serde_json::to_string(text).expect("string serialization");
    let entries = voucher
        .entries
        .iter()
        .zip(&ledgers)
        .map(|(entry, ledger)| {
            format!(
                "{} {}  {}",
                if entry.side == EntrySide::Dr {
                    "Dr"
                } else {
                    "Cr"
                },
                entry.amount,
                marks.named(*ledger)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let legend = marks.legend(voucher.entries.iter().map(|entry| entry.ledger.as_str()));
    let under = lines_under_the_ledgers(legend, line_break_note(&ledgers));
    let classification = classification_review_line(&voucher.voucher_type)
        .map(|line| format!("\n{line}"))
        .unwrap_or_default();
    let narration = approval_narration_lines(super::posted_narration(voucher))?;
    // A narration in parts (#1535): the date line points below the dashed
    // line, and the line above it, which ComplyEaze Bridge writes, counts the
    // parts. A reference is never split: it stays one line, under the caps.
    let parts = (narration.len() > 1).then(|| {
        format!(
            "Below the dashes: the reference on one line, then the narration in {} parts.",
            narration.len()
        )
    });
    let cue = if parts.is_some() {
        VOUCHER_TEXT_CUE_PARTS
    } else {
        VOUCHER_TEXT_CUE
    };
    // The voucher's reference and narration come last, under their heading,
    // after every line ComplyEaze Bridge writes itself, the footer included.
    // The date line says where they are.
    let preview = format!("Create ONE {} in {}\nCompany GUID: {}\nCompany number: {}  Books from: {}\nTally: {origin}\nDate: {}  Voucher number: {}  {cue}\n\n{}\n{under}\nTotal debit: {}  Total credit: {}{classification}\nBatch: {}\nLedgers checked by identity against the build; narrations sent as prepared, nothing added.\nDo not post a file already imported manually.\nPause other edits/imports; keep this company and Tally mode as is until ComplyEaze Bridge finishes.\nAfter a timeout, reconcile this batch; do not rebuild or resend it.",
        voucher.voucher_type.as_str(), quoted(&company.name), company.guid, company.company_number, shown_date(company.books_from.as_str()),
        shown_date(voucher.date.as_str()), voucher.voucher_number.as_deref().map(quoted).unwrap_or_else(|| "Tally assigns it".into()),
        entries, debit.as_str(), credit.as_str(), line.batch_id);
    let preview = std::iter::once(preview)
        .chain(footer.iter().cloned())
        .chain(parts)
        .chain([
            VOUCHER_TEXT_HEADING.to_string(),
            format!(
                "Reference: {}",
                voucher
                    .reference
                    .as_deref()
                    .map(quoted)
                    .unwrap_or_else(|| "(none)".into())
            ),
        ])
        .chain(narration)
        .collect::<Vec<_>>()
        .join("\n");
    // Native message boxes have no portable scrollable review surface. Keep this
    // first posting slice reviewable; longer batches retain the manual file path.
    if preview.chars().count() > ONE_VOUCHER_MAX_CHARS
        || preview.lines().count() > ONE_VOUCHER_MAX_LINES
        || preview.lines().any(|line| line.chars().count() > 100)
    {
        return Err("import_review_too_large".into());
    }
    marks.seal(line, preview)
}

/// What the invoice window says was checked: each tax head against the tax
/// worked out per sales line at the sales ledger's own rate. It is a line of
/// the dialog, so it is no longer than the one it replaced (the window's cap).
const INVOICE_CHECKED_LINE: &str =
    "Checked: customer, Sales Accounts ledgers, CGST and state tax per line at the ledger's rate.";
/// The same line for an invoice with an HSN/SAC warning (never a refusal),
/// with the number of sales ledgers warned about. To fit, it names the tax
/// checked as "tax" where the line above names CGST and state tax. It takes the place of the
/// line above and is no longer while that number has one digit; an invoice
/// has at most `MAX_SALES_LINES` (2) sales ledgers, so the number is at most
/// 2, and ten ledgers would need thirteen entries, more than the window's
/// lines: a warning cannot push a window that fits past its caps.
pub(super) fn invoice_checked_hsn_line(ledgers: usize) -> String {
    format!(
        "Checked: customer, Sales Accounts ledgers, tax per line at ledger rate. HSN/SAC: {ledgers} to check."
    )
}

/// The approval text of one Sales invoice: what a person must see to say yes to
/// a GST document. The party with its GSTIN and registration, the place of
/// supply, the invoice number (which is also the reference), the bill
/// allocation that will be written, and every leg. Same caps and same refusals
/// as the other single-voucher text, and sealed as it is: ledger names are
/// written through the batch's On Account marks, and the voucher's own text
/// (its reference, which is its number, and its narration) stands last, under
/// the heading, after every line of the dialog's own. The cue for it is on
/// the `Tally:` line, a line of the dialog's own that carries none of the
/// voucher's text: the line with the date has no room for it inside a dialog
/// line's 100 characters. The number stands above the entries as well, on the
/// line with the type and the date and as the bill's name; it is of GST rule
/// 46(b)'s alphabet
/// (letters, digits, hyphen and slash, at most 16 characters), checked here,
/// so it can neither wrap nor read as a line of the dialog's own. A record
/// that approves the customer On Account is refused: the customer's entry is
/// written with a New Ref or is not bill-wise, and a dialog that marked it
/// would say both.
fn invoice_review_preview(
    line: &ImportLedgerLine,
    company: &ImportCompanyTuple,
    origin: &str,
    (debit, credit): (&str, &str),
    footer: &[String],
) -> Result<ReviewText, String> {
    let voucher = &line.vouchers[0];
    let detail = voucher
        .invoice
        .as_ref()
        .ok_or("import_invoice_not_observed")?;
    let observed = detail
        .observed
        .as_ref()
        .ok_or("import_invoice_not_observed")?;
    let number = voucher
        .voucher_number
        .as_deref()
        .ok_or("import_invoice_not_observed")?;
    let mut review_text = std::iter::once(company.name.as_str())
        .chain(std::iter::once(number))
        .chain(std::iter::once(detail.voucher_type_name.as_str()))
        .chain(voucher.narration.iter().map(String::as_str))
        .chain(voucher.entries.iter().map(|entry| entry.ledger.as_str()));
    if review_text.clone().any(has_unsafe_review_layout_character) {
        return Err("import_review_layout_text".into());
    }
    if review_text.any(has_unreviewable_format_character) {
        return Err("import_review_format_text".into());
    }
    if super::invoice::InvoiceNumber::parse(number).is_none() {
        return Err("import_review_layout_text".into());
    }
    let marks = OnAccountMarks::of(line);
    let quoted = |text: &str| serde_json::to_string(text).expect("string serialization");
    let party = super::invoice::party_entry(voucher)
        .map(|entry| entry.ledger.as_str())
        .ok_or("import_invoice_not_observed")?;
    if marks.marks(party) {
        return Err("import_invoice_not_observed".into());
    }
    let registration = match observed.party_gstin.as_deref() {
        Some(gstin) => format!("Regular, GSTIN {gstin}"),
        None => "Unregistered, no GSTIN".to_string(),
    };
    let allocation = if observed.party_bill_wise {
        format!("New Ref {}", quoted(number))
    } else {
        "none (customer not bill-wise)".to_string()
    };
    let entries = voucher
        .entries
        .iter()
        .map(|entry| {
            // The names passed the checks above, so each is a dialog's ledger
            // name; one that ends in a line break was refused there.
            let ledger =
                ReviewLedgerName::parse(&entry.ledger).map_err(ReviewTextRefusal::import_code)?;
            Ok(format!(
                "{} {}  {}",
                if entry.side == EntrySide::Dr {
                    "Dr"
                } else {
                    "Cr"
                },
                entry.amount,
                marks.named(ledger)
            ))
        })
        .collect::<Result<Vec<_>, String>>()?
        .join("\n");
    // The line under the entries is the legend of the ledgers approved On Account;
    // with none it is not printed, which keeps a bill of several sales legs inside
    // the window.
    let legend = marks.legend(voucher.entries.iter().map(|entry| entry.ledger.as_str()));
    let under = if legend.is_empty() {
        String::new()
    } else {
        format!("{legend}\n")
    };
    let warned = observed
        .hsn_warnings
        .iter()
        .map(|warning| warning.ledger.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let checked = if warned == 0 {
        INVOICE_CHECKED_LINE.to_string()
    } else {
        invoice_checked_hsn_line(warned)
    };
    let preview = format!("Create ONE Sales invoice in {}\nCompany GUID: {}\nCompany number: {}  Books from: {}\nTally: {origin}  {VOUCHER_TEXT_CUE}\nVoucher type: {}  Date: {}  Number: {}\nCustomer: {}\n{registration}  Place of supply: {}\nBill allocation: {allocation}\n{}\n{under}Total debit: {debit}  Total credit: {credit}\n{checked}\nBatch: {}\nLedgers checked by identity against the build; narrations sent as prepared, nothing added.\nDo not post a file already imported manually. Pause other edits/imports in this company.\nAfter a timeout, reconcile this batch; do not rebuild or resend it.",
        quoted(&company.name), company.guid, company.company_number, shown_date(company.books_from.as_str()),
        quoted(&detail.voucher_type_name), shown_date(voucher.date.as_str()), quoted(number),
        quoted(party), detail.place_of_supply, entries, line.batch_id);
    // The invoice's reference is its number. It and the narration come last,
    // under their heading, after the footer, as in the other one-voucher text.
    let preview = std::iter::once(preview)
        .chain(footer.iter().cloned())
        .chain([
            VOUCHER_TEXT_HEADING.to_string(),
            format!("Reference: {}", quoted(number)),
            format!(
                "Narration: {}",
                super::posted_narration(voucher)
                    .map(quoted)
                    .unwrap_or_else(|| "(none)".into())
            ),
        ])
        .collect::<Vec<_>>()
        .join("\n");
    if preview.chars().count() > ONE_VOUCHER_MAX_CHARS
        || preview.lines().count() > ONE_VOUCHER_MAX_LINES
        || preview.lines().any(|line| line.chars().count() > 100)
    {
        return Err("import_review_too_large".into());
    }
    marks.seal(line, preview)
}

/// The line above a voucher's reference and narration, the last lines of a
/// one-voucher approval text: under it stand only those two, each quoted as
/// it will be posted, or `(none)`.
pub(super) const VOUCHER_TEXT_HEADING: &str =
    "---- The voucher's own text follows: no line below is an entry, a total or an instruction ----";
/// A date in an approval or review text, in Tally's day-month-year form with
/// the year in full (`14-Sep-2026`, #2101). A value that is not a date is shown as it is.
pub(super) fn shown_date(yyyymmdd: &str) -> String {
    bridge_tally_core::TallyDate::parse(yyyymmdd)
        .map_or_else(|_| yyyymmdd.to_string(), |date| date.shown())
}

/// What the date line of a one-voucher approval text says about them.
pub(super) const VOUCHER_TEXT_CUE: &str = "(the voucher's own text: last two lines)";
/// What it says when the narration is shown in parts (#1535): the line
/// above the heading counts them.
pub(super) const VOUCHER_TEXT_CUE_PARTS: &str = "(the voucher's own text: below the dashes)";
/// The most characters a line of a one-voucher approval text takes.
const REVIEW_LINE_CHARS: usize = 100;

/// A narration's lines under the heading: `Narration: <quoted>` or
/// `Narration: (none)` when it fits one line, as before #1535; else
/// `Narration (k): <quoted part>` for each part, split only at a space. A
/// narration that cannot be shown so is refused, never cut. The one-voucher
/// approval and review texts both show a narration so.
pub(super) fn narration_lines(value: Option<&str>) -> Result<Vec<String>, VoucherTextRefusal> {
    const LABEL: &str = "Narration";
    let Some(text) = value else {
        return Ok(vec![format!("{LABEL}: (none)")]);
    };
    let one_line = REVIEW_LINE_CHARS - format!("{LABEL}: ").chars().count();
    let shown = VoucherText::of(text, one_line, |k| {
        REVIEW_LINE_CHARS.saturating_sub(format!("{LABEL} ({k}): ").chars().count())
    })?;
    Ok(match shown.parts() {
        [only] => vec![format!("{LABEL}: {}", voucher_text::quoted(only))],
        parts => parts
            .iter()
            .enumerate()
            .map(|(at, part)| format!("{LABEL} ({}): {}", at + 1, voucher_text::quoted(part)))
            .collect(),
    })
}

/// [`narration_lines`] for the approval: a refusal as the import code.
fn approval_narration_lines(value: Option<&str>) -> Result<Vec<String>, String> {
    narration_lines(value).map_err(|refusal| match refusal {
        VoucherTextRefusal::DoesNotFit => "import_review_too_large".to_string(),
        VoucherTextRefusal::Layout => ReviewTextRefusal::Layout.import_code(),
    })
}

/// The most a one-voucher approval or review text may take: what the native
/// dialog holds. Its own answer text stands above it (#1780) and has room of
/// its own, so a text that fits here is shown whole.
pub(super) const ONE_VOUCHER_MAX_LINES: usize = crate::tally::approved_import::ONE_VOUCHER_LINES;
pub(super) const ONE_VOUCHER_MAX_CHARS: usize = crate::tally::approved_import::ONE_VOUCHER_CHARS;

/// The most a batch's approval text may take: lines, characters, characters
/// a line, and UTF-8 bytes (under the native dialog's 8,000). A batch whose
/// summary does not fit is refused, never cut; the caller posts it in parts.
pub(super) const BATCH_REVIEW_MAX_LINES: usize = crate::tally::approved_import::BATCH_LINES;
pub(super) const BATCH_REVIEW_MAX_CHARS: usize = crate::tally::approved_import::BATCH_CHARS;
pub(super) const BATCH_REVIEW_MAX_LINE_CHARS: usize = 100;
pub(super) const BATCH_REVIEW_MAX_BYTES: usize = 7_000;

/// The most vouchers a batch's dialog lists one line each (#1063).
pub(super) const BATCH_REVIEW_MAX_VOUCHER_LINES: usize = 10;
/// The characters of a narration a voucher's line shows before it marks the
/// cut. Characters, not graphemes: a cut can split a Devanagari conjunct, and
/// the marker says so rather than hiding it (#1063).
pub(super) const BATCH_REVIEW_NARRATION_CHARS: usize = 40;
/// The same for a Journal's line, which names a ledger on each side.
pub(super) const BATCH_REVIEW_JOURNAL_NARRATION_CHARS: usize = 20;
/// The fewest narration characters a Journal's line shows when its cut moves
/// down to fit the line: fewer would identify nothing.
pub(super) const BATCH_REVIEW_JOURNAL_NARRATION_MIN_CHARS: usize = 10;

/// The line above a small batch's voucher lines.
pub(super) const VOUCHER_LINES_HEADING: &str =
    "Each voucher: type, date, amount, ledger, narration (references not shown):";
/// The first heading line of a batch that marks a ledger On Account: a
/// voucher's line has no room for the mark, so the heading says where it is.
pub(super) const VOUCHER_LINES_HEADING_MARKED: &str =
    "Each voucher: type, date, amount, ledger, narration (no references; On Account is marked below):";
/// The second heading line: what the quoted text ending each line is. The
/// dialog wraps long lines, so a narration's tail can begin a row of its own
/// (#1063 follow-up).
pub(super) const VOUCHER_LINES_NARRATION_HEADING: &str =
    "Each line ends with its narration, quoted; a cut one is marked … (+N characters).";
/// What a voucher's line shows in place of a narration that reads like a line
/// of this dialog. It echoes none of the narration.
pub(super) const NARRATION_WITHHELD: &str = "(narration withheld: it reads like a dialog line)";
// Each sentence that stands in for the voucher lines takes the place of the
// line a batch showed before #1063, and is no longer than it, so the
// totals-only text never outgrows the caps a batch passed before.
/// A batch of more than `BATCH_REVIEW_MAX_VOUCHER_LINES` vouchers.
pub(super) const VOUCHER_LINES_OVER_LIMIT: &str =
    "Per-voucher lines are not shown: this batch has over 10 vouchers.";
/// A narration or reference holds a layout or format character; the reason
/// never echoes the text.
pub(super) const VOUCHER_LINES_UNSAFE: &str =
    "Per-voucher lines are not shown: a narration/reference is unsafe.";
/// The voucher lines would break one of the dialog's caps.
pub(super) const VOUCHER_LINES_DO_NOT_FIT: &str =
    "Per-voucher lines are not shown: they do not fit this dialog.";

/// Whether `text` holds a shape a line of the batch dialog begins with, so
/// that a wrapped tail of it could pass for one (#1063 follow-up). Matched
/// case-insensitively anywhere: `\bdr\s+[0-9][0-9.,]*\s+cr\s+[0-9]` (a
/// ledger line's two halves), `total\s+(debit|credit)\s*:` (any whitespace
/// between the words, a no-break space included), `batch\s*:` and
/// `create\s+\d`. Written out rather than with a regex crate: `\s` is any
/// whitespace, `\d` any Unicode decimal digit, and `\b` sees a letter, digit
/// or `_` as a word character. A bank narration with one of `DR`/`CR` and a
/// reference number ("NEFT CR 000123456789") is not such a shape.
pub(super) fn reads_like_a_dialog_line(text: &str) -> bool {
    use icu_properties::{props::GeneralCategory, CodePointMapData};
    let category = CodePointMapData::<GeneralCategory>::new();
    let chars: Vec<char> = text.chars().map(|c| c.to_ascii_lowercase()).collect();
    let word = |c: char| c.is_alphanumeric() || c == '_';
    let literal = |at: usize, expected: &str| -> Option<usize> {
        let mut at = at;
        for wanted in expected.chars() {
            (chars.get(at) == Some(&wanted)).then_some(())?;
            at += 1;
        }
        Some(at)
    };
    let spaces = |at: usize, least: usize| -> Option<usize> {
        let end = at
            + chars
                .get(at..)
                .unwrap_or_default()
                .iter()
                .take_while(|c| c.is_whitespace())
                .count();
        (end - at >= least).then_some(end)
    };
    let ascii_digit = |at: usize| chars.get(at).is_some_and(char::is_ascii_digit);
    let ledger_line = |at: usize| -> Option<()> {
        (at == 0 || !word(chars[at - 1])).then_some(())?;
        let at = spaces(literal(at, "dr")?, 1)?;
        ascii_digit(at).then_some(())?;
        let at = at
            + chars[at..]
                .iter()
                .take_while(|c| c.is_ascii_digit() || matches!(c, '.' | ','))
                .count();
        let at = spaces(literal(spaces(at, 1)?, "cr")?, 1)?;
        ascii_digit(at).then_some(())
    };
    let total = |at: usize| -> Option<()> {
        let at = spaces(literal(at, "total")?, 1)?;
        let at = literal(at, "debit").or_else(|| literal(at, "credit"))?;
        (chars.get(spaces(at, 0)?) == Some(&':')).then_some(())
    };
    let batch = |at: usize| -> Option<()> {
        (chars.get(spaces(literal(at, "batch")?, 0)?) == Some(&':')).then_some(())
    };
    let create = |at: usize| -> Option<()> {
        chars
            .get(spaces(literal(at, "create")?, 1)?)
            .is_some_and(|c| category.get(*c) == GeneralCategory::DecimalNumber)
            .then_some(())
    };
    (0..chars.len()).any(|at| {
        ledger_line(at)
            .or_else(|| total(at))
            .or_else(|| batch(at))
            .or_else(|| create(at))
            .is_some()
    })
}

/// One line per voucher of a small batch (#1063): its type and date, its
/// value (the sum of its debits), the ledger a statement row names (a
/// Receipt's first credit, a Payment's or Contra's first debit, in the batch
/// file's order, with "+N more" for the rest of that side, never a bare
/// "+N" that could read as an amount; a Journal's first debit and first
/// credit, each so, as `Dr … Cr …`) and its narration as posted,
/// quoted and cut at `BATCH_REVIEW_NARRATION_CHARS` (a Journal's at
/// `BATCH_REVIEW_JOURNAL_NARRATION_CHARS`, or lower, down to
/// `BATCH_REVIEW_JOURNAL_NARRATION_MIN_CHARS`, when that makes its line
/// fit) with the cut marked, or
/// `NARRATION_WITHHELD` when it reads like a line of this dialog; the On
/// Account legend counts as one only when this dialog carries it (`legend`).
/// `None` when a narration or reference holds a character the dialog cannot
/// show as it is: nothing is stripped or altered.
fn voucher_review_lines(
    vouchers: &[ImportVoucher],
    legend: bool,
) -> Result<Option<Vec<String>>, String> {
    let shown_text = vouchers.iter().flat_map(|voucher| {
        super::posted_narration(voucher)
            .into_iter()
            .chain(voucher.reference.as_deref())
    });
    if shown_text.clone().any(|text| {
        has_unsafe_review_layout_character(text) || has_unreviewable_format_character(text)
    }) {
        return Ok(None);
    }
    let quoted = |text: &str| serde_json::to_string(text).expect("string serialization");
    let mut lines = Vec::with_capacity(vouchers.len());
    for voucher in vouchers {
        let mut value = ExactDecimal::zero();
        for entry in voucher
            .entries
            .iter()
            .filter(|entry| entry.side == EntrySide::Dr)
        {
            let amount = ExactDecimal::parse(entry.amount.clone())
                .map_err(|_| "voucher_amount_invalid".to_string())?;
            value = value
                .checked_add(&amount)
                .map_err(|_| "voucher_amount_overflow".to_string())?;
        }
        let shown = |name: &str| {
            ReviewLedgerName::parse(name)
                .map(ReviewLedgerName::shown)
                .map_err(ReviewTextRefusal::import_code)
        };
        let side_named = |side: EntrySide| -> Result<String, String> {
            let named: Vec<&str> = voucher
                .entries
                .iter()
                .filter(|entry| entry.side == side)
                .map(|entry| entry.ledger.as_str())
                .collect();
            Ok(match named.as_slice() {
                [] => "(none)".to_string(),
                [only] => shown(only)?,
                [first, rest @ ..] => format!("{} +{} more", shown(first)?, rest.len()),
            })
        };
        // A Journal has no bank side fixed by its type, so its line names
        // the first ledger of each side; its narration gets fewer characters.
        let journal = voucher.voucher_type == VoucherType::Journal;
        let ledger = if journal {
            format!(
                "Dr {}  Cr {}",
                side_named(EntrySide::Dr)?,
                side_named(EntrySide::Cr)?
            )
        } else if voucher.voucher_type == VoucherType::Receipt {
            side_named(EntrySide::Cr)?
        } else {
            side_named(EntrySide::Dr)?
        };
        // Whether the narration is shown at all is decided once, on its whole
        // text: `Err` holds what stands in its place at every cut.
        let posted = match super::posted_narration(voucher) {
            None => Err("(none)"),
            Some(text)
                if reads_like_a_dialog_line(text) || (legend && reads_like_the_legend(text)) =>
            {
                Err(NARRATION_WITHHELD)
            }
            Some(text) => Ok(text),
        };
        let narration = |cut: usize| match posted {
            Err(shown) => shown.to_string(),
            Ok(text) => {
                let length = text.chars().count();
                if length > cut {
                    let kept: String = text.chars().take(cut).collect();
                    format!("{}… (+{} characters)", quoted(&kept), length - cut)
                } else {
                    quoted(text)
                }
            }
        };
        let line_with = |narration: String| {
            format!(
                "{} {}  {}  {ledger}  {narration}",
                voucher.voucher_type.as_str(),
                shown_date(voucher.date.as_str()),
                value.as_str(),
            )
        };
        // A Journal's narration gives way first: when its line would be wider
        // than a line may be, the cut moves down, never below
        // `BATCH_REVIEW_JOURNAL_NARRATION_MIN_CHARS`, until the line fits. A
        // line that fits at no such cut is kept at the usual cut, so the batch
        // leaves the totals as before.
        lines.push(if journal {
            (BATCH_REVIEW_JOURNAL_NARRATION_MIN_CHARS..=BATCH_REVIEW_JOURNAL_NARRATION_CHARS)
                .rev()
                .map(|cut| line_with(narration(cut)))
                .find(|line| line.chars().count() <= BATCH_REVIEW_MAX_LINE_CHARS)
                .unwrap_or_else(|| line_with(narration(BATCH_REVIEW_JOURNAL_NARRATION_CHARS)))
        } else {
            line_with(narration(BATCH_REVIEW_NARRATION_CHARS))
        });
    }
    Ok(Some(lines))
}

/// The approval text for a batch: a summary a person can read in one native
/// dialog. Every ledger's debit and credit totals and entry count, the totals
/// by voucher type, the money the types themselves fix as moving in or out,
/// and the standing cautions; and for a batch of at most
/// `BATCH_REVIEW_MAX_VOUCHER_LINES`, one line per voucher (#1063). When those
/// lines cannot be shown, the summary says why in their place, and a batch is
/// never refused for them. References are not shown; the amounts and ledgers
/// are what the approval binds.
///
/// Order (#746): what a person checks first comes first, in lines this
/// program writes, which hold no narration and no ledger name: the count and
/// company, the company number, the types, the dates, the totals and the
/// money lines. Then the company GUID and the Tally origin, the voucher lines
/// or the sentence in their place, the per-ledger rows, the batch id, and the
/// cautions.
fn batch_review_text(
    line: &ImportLedgerLine,
    company: &ImportCompanyTuple,
    origin: &str,
    debit: &ExactDecimal,
    credit: &ExactDecimal,
    footer: &[String],
    form: PreviewForm,
) -> Result<ReviewText, String> {
    let names = admit_review_text(
        [company.name.as_str()],
        line.vouchers
            .iter()
            .flat_map(|voucher| voucher.entries.iter().map(|entry| entry.ledger.as_str())),
    )
    .map_err(ReviewTextRefusal::import_code)?;
    let note = line_break_note(&names);
    for voucher in &line.vouchers {
        require_native_numbering(voucher)?;
    }
    let add = |total: &mut ExactDecimal, amount: &str| -> Result<(), String> {
        let amount = ExactDecimal::parse(amount.to_string())
            .map_err(|_| "voucher_amount_invalid".to_string())?;
        *total = total
            .checked_add(&amount)
            .map_err(|_| "voucher_amount_overflow".to_string())?;
        Ok(())
    };
    let mut by_type = BTreeMap::<&str, usize>::new();
    let mut ledgers = BTreeMap::<ReviewLedgerName, (ExactDecimal, ExactDecimal, usize)>::new();
    let (mut money_in, mut money_out) = (ExactDecimal::zero(), ExactDecimal::zero());
    // `names` holds every entry's ledger, in this order.
    let mut admitted = names.iter().copied();
    for voucher in &line.vouchers {
        *by_type.entry(voucher.voucher_type.as_str()).or_default() += 1;
        for entry in &voucher.entries {
            let name = admitted.next().ok_or("import_review_layout_text")?;
            let totals = ledgers
                .entry(name)
                .or_insert_with(|| (ExactDecimal::zero(), ExactDecimal::zero(), 0));
            totals.2 += 1;
            match &entry.side {
                EntrySide::Dr => add(&mut totals.0, &entry.amount)?,
                EntrySide::Cr => add(&mut totals.1, &entry.amount)?,
            }
            // The type fixes the cash/bank side: a Receipt's debits and a
            // Payment's credits (checked at build and again in the queue).
            match (&voucher.voucher_type, &entry.side) {
                (VoucherType::Receipt, EntrySide::Dr) => add(&mut money_in, &entry.amount)?,
                (VoucherType::Payment, EntrySide::Cr) => add(&mut money_out, &entry.amount)?,
                _ => {}
            }
        }
    }
    let (first, last) = date_window(line.vouchers.iter().map(|voucher| &voucher.date))
        .ok_or("voucher_count_invalid")?;
    let (first, last) = (shown_date(first.as_str()), shown_date(last.as_str()));
    let quoted = |text: &str| serde_json::to_string(text).expect("string serialization");
    let marks = OnAccountMarks::of(line);
    let mut head = vec![
        format!(
            "Create {} vouchers in {}",
            line.vouchers.len(),
            quoted(&company.name)
        ),
        format!(
            "Company number: {}  Books from: {}",
            company.company_number,
            shown_date(company.books_from.as_str())
        ),
        format!(
            "Types: {}",
            by_type
                .iter()
                .map(|(voucher_type, count)| format!("{count} {voucher_type}"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!("Dates: {first} to {last}  Voucher numbers: Tally assigns them"),
        format!(
            "Total debit: {}  Total credit: {}",
            debit.as_str(),
            credit.as_str()
        ),
    ];
    if by_type.contains_key(VoucherType::Receipt.as_str()) {
        head.push(format!(
            "Money in by Receipt vouchers: {}",
            money_in.as_str()
        ));
    }
    if by_type.contains_key(VoucherType::Payment.as_str()) {
        head.push(format!(
            "Money out by Payment vouchers: {}",
            money_out.as_str()
        ));
    }
    if by_type.contains_key(VoucherType::Contra.as_str()) {
        head.push("Contra: moves between cash/bank ledgers, net zero".into());
    }
    if by_type.contains_key(VoucherType::Journal.as_str()) {
        head.push("Journals may also move cash/bank ledgers; see the per-ledger totals".into());
    }
    head.push(format!("Company GUID: {}", company.guid));
    head.push(format!("Tally: {origin}"));
    let mut text = vec![String::new()];
    for (ledger, (dr, cr, count)) in &ledgers {
        text.push(format!(
            "Dr {}  Cr {}  {count} {}  {}",
            dr.as_str(),
            cr.as_str(),
            if *count == 1 { "entry" } else { "entries" },
            marks.named(*ledger)
        ));
    }
    let legend = marks.legend(ledgers.keys().map(|ledger| ledger.as_str()));
    text.extend(
        lines_under_the_ledgers(legend, note)
            .split('\n')
            .map(str::to_string),
    );
    text.push(format!("Batch: {}", line.batch_id));
    text.push(String::new());
    text.push(
        "Ledgers checked by identity against the build; narrations sent as prepared, nothing added.".into(),
    );
    text.push("Do not post a file already imported manually.".into());
    text.push(
        "Pause other edits/imports; keep this company and Tally mode as is until ComplyEaze Bridge finishes."
            .into(),
    );
    text.push("After a timeout, reconcile this batch; do not rebuild or resend it.".into());
    text.extend(footer.iter().cloned());
    let fits = |lines: &[String]| {
        let preview = lines.join("\n");
        lines.len() <= BATCH_REVIEW_MAX_LINES
            && preview.chars().count() <= BATCH_REVIEW_MAX_CHARS
            && preview.len() <= BATCH_REVIEW_MAX_BYTES
            && lines
                .iter()
                .all(|line| line.chars().count() <= BATCH_REVIEW_MAX_LINE_CHARS)
    };
    let whole = |middle: Vec<String>| [head.clone(), middle, text.clone()].concat();
    let reason = if line.vouchers.len() > BATCH_REVIEW_MAX_VOUCHER_LINES {
        VOUCHER_LINES_OVER_LIMIT
    } else if let Some(vouchers) = voucher_review_lines(&line.vouchers, !legend.is_empty())? {
        // The totals form skips the listed attempt, and keeps every refusal
        // before it: over the voucher limit, and lines that cannot be shown.
        if form == PreviewForm::Listed {
            let listed = whole(
                [
                    if legend.is_empty() {
                        VOUCHER_LINES_HEADING
                    } else {
                        VOUCHER_LINES_HEADING_MARKED
                    },
                    VOUCHER_LINES_NARRATION_HEADING,
                ]
                .map(str::to_string)
                .into_iter()
                .chain(vouchers)
                .collect(),
            );
            if fits(&listed) {
                return marks
                    .seal(line, listed.join("\n"))
                    .map(ReviewText::listing_vouchers);
            }
        }
        VOUCHER_LINES_DO_NOT_FIT
    } else {
        VOUCHER_LINES_UNSAFE
    };
    let summary = whole(vec![reason.to_string()]);
    if !fits(&summary) {
        return Err("import_review_too_large".into());
    }
    marks.seal(line, summary.join("\n"))
}

/// Why a value cannot be shown in a native dialog. A layout character anywhere
/// is reported before a format character anywhere.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum ReviewTextRefusal {
    Layout,
    Format,
}

impl ReviewTextRefusal {
    fn of(value: &str) -> Option<Self> {
        if has_unsafe_review_layout_character(value) {
            Some(Self::Layout)
        } else if has_unreviewable_format_character(value) {
            Some(Self::Format)
        } else {
            None
        }
    }

    /// The code an approval dialog refuses with.
    fn import_code(self) -> String {
        match self {
            Self::Layout => "import_review_layout_text".into(),
            Self::Format => "import_review_format_text".into(),
        }
    }
}

/// A ledger name as a dialog shows it (bridge#626): the only form a dialog
/// line is written from. It is held to the rules for all dialog text, except
/// that it may end in exactly one CR LF when the rest is a name the build
/// admits, the one spelling with a line break that a build binds. It is shown
/// JSON-quoted, so the break is written out and no control character reaches
/// the dialog, with [`LINE_BREAK_MARK`] before the quote
/// (`Line break  "Cash\r\n"`); a text that shows one carries
/// [`LINE_BREAK_NOTE`] once.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ReviewLedgerName<'a>(&'a str);

/// What every line shows before the quoted name of a ledger whose name ends
/// in a line break: on the name's own line and outside its quotes, where no
/// name can write it.
pub(super) const LINE_BREAK_MARK: &str = "Line break  ";
/// What the mark says, once, on a line of its own under the lines that name
/// ledgers.
pub(super) const LINE_BREAK_NOTE: &str =
    "Line break: a name that ends in a line break in Tally (shown as \\r\\n). No other name here has one.";

impl<'a> ReviewLedgerName<'a> {
    pub(super) fn parse(name: &'a str) -> Result<Self, ReviewTextRefusal> {
        let checked = if super::live_spelling_importable(0, name) {
            super::without_trailing_crlf(name)
        } else {
            name
        };
        ReviewTextRefusal::of(checked).map_or(Ok(Self(name)), Err)
    }

    pub(super) fn as_str(self) -> &'a str {
        self.0
    }

    pub(super) fn ends_in_line_break(self) -> bool {
        self.0.ends_with("\r\n")
    }

    /// The name as every dialog line shows it, and the only way one is
    /// written: JSON-quoted, with [`LINE_BREAK_MARK`] before it when it ends
    /// in a line break.
    pub(super) fn shown(self) -> String {
        let quoted = serde_json::to_string(self.0).expect("string serialization");
        if self.ends_in_line_break() {
            format!("{LINE_BREAK_MARK}{quoted}")
        } else {
            quoted
        }
    }
}

/// The check every dialog text passes for the names of `shown` that end in a
/// line break. The text holds no carriage return. [`LINE_BREAK_NOTE`] is a
/// line of it when there is such a name. For each such name a line ends in
/// two spaces, the mark and the quoted name, which is the name's own line.
/// And the characters `\r\n"`, a written-out break before a closing quote,
/// stand in the text exactly as often as those marked names do, so none is
/// shown without its mark. Every value a dialog quotes is refused for a
/// control character unless it is such a name, a typed backslash is written
/// doubled, and a quote inside a value has a backslash before it.
pub(super) fn line_break_names_are_marked<'a>(
    text: &str,
    shown: impl IntoIterator<Item = ReviewLedgerName<'a>>,
) -> bool {
    let marked: BTreeSet<String> = shown
        .into_iter()
        .filter(|name| name.ends_in_line_break())
        .map(ReviewLedgerName::shown)
        .collect();
    !text.contains('\r')
        && (marked.is_empty() || text.lines().any(|line| line == LINE_BREAK_NOTE))
        && marked.iter().all(|name| {
            let own_line = format!("  {name}");
            text.lines().any(|line| line.ends_with(&own_line))
        })
        && text.matches("\\r\\n\"").count()
            == marked
                .iter()
                .map(|name| text.matches(name.as_str()).count())
                .sum::<usize>()
}

/// The line or lines under a text's entry or per-ledger lines: the On Account
/// sentence, the line-break note, both in that order, or one blank line.
fn lines_under_the_ledgers(legend: &str, note: Option<&str>) -> String {
    [legend, note.unwrap_or_default()]
        .into_iter()
        .filter(|shown| !shown.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// [`LINE_BREAK_NOTE`] when any of `names` ends in a line break.
pub(super) fn line_break_note(names: &[ReviewLedgerName<'_>]) -> Option<&'static str> {
    names
        .iter()
        .any(|name| name.ends_in_line_break())
        .then_some(LINE_BREAK_NOTE)
}

/// Checks every value a dialog shows: `text` under the plain rules and
/// `ledgers` as [`ReviewLedgerName`]s, returned in the order given.
pub(super) fn admit_review_text<'a>(
    text: impl IntoIterator<Item = &'a str>,
    ledgers: impl IntoIterator<Item = &'a str>,
) -> Result<Vec<ReviewLedgerName<'a>>, ReviewTextRefusal> {
    let ledgers = ledgers
        .into_iter()
        .map(ReviewLedgerName::parse)
        .collect::<Vec<_>>();
    let refusal = text
        .into_iter()
        .filter_map(ReviewTextRefusal::of)
        .chain(ledgers.iter().filter_map(|ledger| ledger.err()))
        .min();
    match refusal {
        Some(refusal) => Err(refusal),
        None => ledgers.into_iter().collect(),
    }
}

pub(super) fn has_unsafe_review_layout_character(value: &str) -> bool {
    value
        .chars()
        .any(|character| character.is_control() || matches!(character, '\u{2028}' | '\u{2029}'))
}

pub(super) fn has_unreviewable_format_character(value: &str) -> bool {
    use icu_properties::{
        props::{DefaultIgnorableCodePoint, GeneralCategory},
        CodePointMapData, CodePointSetData,
    };
    // Unicode UAX #44: General_Category=Cf and Default_Ignorable_Code_Point.
    // Native approval must not hide characters that distinguish exact XML names.
    let ignorable = CodePointSetData::new::<DefaultIgnorableCodePoint>();
    let category = CodePointMapData::<GeneralCategory>::new();
    value.chars().any(|character| {
        ignorable.contains(character) || category.get(character) == GeneralCategory::Format
    })
}

#[path = "agent_import_post_location.rs"]
mod location;

#[path = "agent_import_numbering.rs"]
pub(super) mod numbering;

#[cfg(test)]
#[path = "agent_import_numbering_tests.rs"]
mod numbering_tests;

/// The aim snapshot's target voucher mark from its answer, for the journal tests: none when the
/// target row omits the mark (#1945), so the post refuses (`post_mark_unrecorded`).
#[cfg(test)]
pub(super) fn location_aimed_voucher_mark_for_tests(
    marks: &str,
    guid: &str,
    name: &str,
) -> Option<u64> {
    location::aimed_voucher_mark(marks, guid, name)
}

/// The target's voucher mark among parsed rows, for the journal tests: present only when exactly
/// one loaded company is the target.
#[cfg(test)]
pub(super) fn location_target_voucher_mark_for_tests(
    rows: &[location::LoadedCompanyMarks],
    guid: &str,
    name: &str,
) -> Option<u64> {
    location::target_voucher_mark(rows, guid, name)
}

#[cfg(test)]
#[path = "agent_import_post_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "agent_import_post_e2e_tests.rs"]
mod e2e_tests;
