//! A post approval that outlives the MCP call which asked for it (#725).
//!
//! `post_import` waits for the person at the native dialog only while the call
//! can afford to: an MCP host abandons a call after about a minute (#485,
//! #703). A dialog still open when the call returns stays open, held here, and
//! a later `post_import` of the same batch joins it or redeems its approval.
//!
//! Everything here is in memory, in the one process that showed the dialog. A
//! restarted process holds nothing, so no approval can be redeemed that a
//! person did not give to this process. An approval is bound to what the
//! person was shown, must be taken for posting within a fixed time of the
//! click, is redeemed once, and is spent under the import admission lock before
//! the dispatch intent is written. A cancelled call, a refused redemption, or a
//! redemption that ends before its intent lapses it. An approval that lapses
//! unredeemed leaves a note saying so, and nothing can post from a note. A No,
//! a timeout or an unanswered end is kept as its code and returned once to its
//! batch's next call, which does not ask the person again: for as long as an
//! approval would be kept, at most `MAX_KEPT_REFUSALS` at once, and not across
//! a withdrawal of the batch or a restart.
use super::post::NativePostRequest;
use super::{sha256_hex, write_private, ImportCompanyTuple, ImportLedgerLine};
use crate::tally::approved_import::{
    Answered, ApprovedImport, PendingPostApproval, UnderLockRefusal,
};
use bridge_tally_protocol::StandardLedgerCatalogBinding;
use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::{json, Value};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// How long a call waits on its dialog, counted from the call's start, before
/// returning `pending`. Unit tests wait on scripted dialogs, where a long
/// budget only slows them.
#[cfg(not(test))]
pub(super) const CALL_BUDGET: Duration = Duration::from_secs(40);
#[cfg(test)]
pub(super) const CALL_BUDGET: Duration = Duration::from_millis(300);
/// The most a call that answers and posts may take in all: three quarters of
/// the roughly 60-second host timeout observed once (#485, #703).
pub(super) const CALL_CEILING: Duration = Duration::from_secs(45);
/// What a post took from its approval to its result at the largest batch
/// measured live: 200 Journals (D4, 26 Sep 2026, bridge#725), from the answer
/// to the end of the call, 20.95 s: the queue's re-checks 3.0 s, the POST 5.2 s,
/// the readback 12.1 s, and the spacing between them. A concurrent build ran,
/// so it overstates a quiet book; it is not measured above 200.
pub(super) const MEASURED_POST: Duration = Duration::from_millis(20_950);
pub(super) const MEASURED_POST_VOUCHERS: usize = 200;

/// What a redeem took from the start of its call to its result: the call that
/// finds an approval, checks the book afresh, posts and reads back. Measured
/// live once, 50 Journals on a small synthetic book (L1-a, 28 Sep 2026,
/// bridge#725): 18.09 s. A joining call redeems only a click it finds already
/// made, so it starts the redeem within moments of its own start: this check
/// then limits the batch to the size measured, and bounds nothing on a large
/// book, where a redeem costs what a fresh redeeming call does (#852).
pub(super) const MEASURED_REDEEM: Duration = Duration::from_millis(18_090);
pub(super) const MEASURED_REDEEM_VOUCHERS: usize = 50;

/// Whether work that costs `measured`, measured live at up to
/// `measured_vouchers`, fits under the ceiling when it starts `elapsed` into
/// its call.
fn fits_within(
    measured: Duration,
    measured_vouchers: usize,
    elapsed: Duration,
    vouchers: usize,
) -> bool {
    vouchers <= measured_vouchers
        && elapsed
            .checked_add(measured)
            .is_some_and(|total| total <= CALL_CEILING)
}

/// Whether an approval answered `elapsed` into its call may be posted in that
/// same call, as before #725: only when a post that costs `measured` still fits
/// under the ceiling. Otherwise the next call redeems it, after checking the
/// book again.
fn fits_with(measured: Duration, elapsed: Duration, vouchers: usize) -> bool {
    fits_within(measured, MEASURED_POST_VOUCHERS, elapsed, vouchers)
}

/// How long an answered approval may wait to be taken for posting, from the
/// click. The owner approved 15 minutes (#725) after it was compared with 5, 10
/// and 30 minutes, a limit scaled by voucher count, and one renewed per chunk
/// (bridge#792): a late take re-runs every check, and 15 is the smallest round
/// value the projected chunked runs also fit.
pub(super) const APPROVAL_TTL: Duration = Duration::from_secs(15 * 60);
/// A dialog is given at least this long in the call that starts it, so the
/// dialog is up before the call returns even when the checks used the budget.
#[cfg(not(test))]
pub(super) const MIN_DIALOG_WAIT: Duration = Duration::from_secs(1);
#[cfg(test)]
pub(super) const MIN_DIALOG_WAIT: Duration = Duration::from_millis(50);
/// How long a caller is asked to wait before calling again about a dialog
/// still open, so that no agent calls in a tight loop.
pub(super) const RETRY_AFTER: Duration = Duration::from_secs(15);
/// A lapse note is a few hundred bytes; anything larger is not one.
const MAX_LAPSE_NOTE_BYTES: u64 = 4_096;

/// Whether the calling tool call has been withdrawn (#554, #725).
fn call_withdrawn() -> bool {
    crate::tally::runtime::TOOL_CANCELLATION
        .try_with(|token| token.is_cancelled())
        .unwrap_or(false)
}

/// What the person was shown and what it is about, fixed when the dialog is
/// asked. A redemption whose fresh checks produce a different binding is
/// refused: the approval covered something else.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ApprovalBinding {
    batch_sha256: String,
    endpoint_origin: Option<String>,
    company: Option<ImportCompanyTuple>,
    voucher_count: usize,
    preview_sha256: String,
    ledgers: Vec<(String, String)>,
}

impl ApprovalBinding {
    pub(super) fn new(
        line: &ImportLedgerLine,
        preview: &str,
        ledgers: &StandardLedgerCatalogBinding,
    ) -> Self {
        Self {
            batch_sha256: line.sha256.clone(),
            endpoint_origin: line.endpoint_origin.clone(),
            company: line.company.clone(),
            voucher_count: line.vouchers.len(),
            preview_sha256: sha256_hex(preview.as_bytes()),
            ledgers: ledgers
                .pairs()
                .map(|(name, guid)| (name.to_string(), guid.to_ascii_lowercase()))
                .collect(),
        }
    }
}

enum Held {
    /// The dialog is open, or answered with no call yet to collect it.
    /// `dialog` is `None` while a call waits on it.
    Pending {
        binding: ApprovalBinding,
        dialog: Option<PendingPostApproval>,
        native: NativePostRequest,
    },
    /// The person approved, and no call has redeemed it yet.
    Approved {
        binding: ApprovalBinding,
        id: Uuid,
        request: Box<ApprovedImport>,
        native: NativePostRequest,
        answered: Answered,
    },
    /// Taken by the call posting it; spent when that call's intent is admitted.
    Redeeming { id: Uuid, answered: Answered },
}

/// What a `post_import` call finds held for its batch.
pub(super) enum Begin {
    /// Nothing held: check the batch and ask.
    Ask,
    /// A dialog for this batch is open and this call may wait on it.
    Join(PendingPostApproval),
    /// Another call is already waiting on this batch's dialog.
    Waiting,
    /// This batch is approved: check it afresh and redeem.
    Redeem,
    /// Something else is held; refused with this code.
    Busy(&'static str),
    /// This batch's dialog ended in a refusal no call has read: returned now.
    Refused(String),
}

/// What waiting on a joined dialog came to.
pub(super) enum Joined {
    StillOpen { remaining: Duration },
    Approved,
    Refused(String),
}

/// An approval taken by the call about to post it. Whatever way that call
/// ends, an approval it did not spend lapses when this drops: it is never
/// offered again, and it never blocks another post.
pub(super) struct Redemption<'a> {
    approvals: &'a PostApprovals,
    batch_id: String,
    id: Uuid,
}

impl Redemption<'_> {
    pub(super) fn id(&self) -> Uuid {
        self.id
    }
}

impl Drop for Redemption<'_> {
    fn drop(&mut self) {
        self.approvals
            .release_unspent(&self.batch_id, self.id, "post_refused_before_intent");
    }
}

/// The refusal of a held dialog that ended without an approval (declined,
/// timed out, or ended with no answer), and when it was given: the click, or
/// now for a dialog that ended with no answer, which has no click.
fn refusal_of(held: &Held) -> Option<(String, Answered)> {
    match held {
        Held::Pending {
            dialog: Some(dialog),
            ..
        } => dialog.refusal().map(|code| {
            let answered = dialog.answered().unwrap_or_else(|| Answered {
                approved: false,
                at: std::time::Instant::now(),
                at_wall: SystemTime::now(),
            });
            (code, answered)
        }),
        _ => None,
    }
}

/// A refusal no call has read yet: the person's No, a timeout, or a dialog that
/// ended unanswered. It is only a code: it approves nothing and holds nothing.
struct KeptRefusal {
    batch_id: String,
    code: String,
    answered: Answered,
}

/// How many batches' unread refusals are kept at once. Past it the oldest is
/// dropped, and that batch's next call asks the person again.
pub(super) const MAX_KEPT_REFUSALS: usize = 32;

/// The approvals one process holds: one batch's dialog or approval at a time,
/// and the unread refusals of other batches.
pub(in crate::agent) struct PostApprovals {
    imports: PathBuf,
    held: Mutex<Option<(String, Held)>>,
    /// Refusals that landed while no call waited, one per batch, oldest first.
    /// Each is returned once, to its own batch's next call, instead of asking
    /// the person again (#725). They block no other batch. Locked only while
    /// `held` is.
    refusals: Mutex<std::collections::VecDeque<KeptRefusal>>,
    ttl: Duration,
    measured_post: Duration,
    measured_redeem: Duration,
}

impl PostApprovals {
    pub(in crate::agent) fn new(data_dir: &Path) -> Self {
        Self {
            imports: data_dir.join("imports"),
            held: Mutex::new(None),
            refusals: Mutex::new(std::collections::VecDeque::new()),
            ttl: APPROVAL_TTL,
            measured_post: MEASURED_POST,
            measured_redeem: MEASURED_REDEEM,
        }
    }

    /// This holder with its redeems measured at `measured`: a test drives the
    /// joined-too-late branch through a real call with it.
    #[cfg(test)]
    pub(in crate::agent) fn with_measured_redeem(mut self, measured: Duration) -> Self {
        self.measured_redeem = measured;
        self
    }

    /// Whether a joining call that finds a click already made `elapsed` into
    /// it may redeem it in that call (#725 slice 2.0): a batch no larger than
    /// the redeem measured live, whose measured cost fits under the ceiling
    /// from here. A measurement, not a bound on what the redeem then takes.
    pub(super) fn redeem_fits_in_call(&self, elapsed: Duration, vouchers: usize) -> bool {
        fits_within(
            self.measured_redeem,
            MEASURED_REDEEM_VOUCHERS,
            elapsed,
            vouchers,
        )
    }

    #[cfg(test)]
    pub(super) fn with_ttl(data_dir: &Path, ttl: Duration) -> Self {
        let mut approvals = Self::new(data_dir);
        approvals.ttl = ttl;
        approvals
    }

    /// Whether a post answered `elapsed` into its call fits in that call, at
    /// this holder's measured post cost.
    pub(super) fn fits_in_call(&self, elapsed: Duration, vouchers: usize) -> bool {
        fits_with(self.measured_post, elapsed, vouchers)
    }

    /// A holder whose posts are measured at `measured`: a test drives the
    /// answered-too-late branch through a real call with it.
    #[cfg(test)]
    pub(in crate::agent) fn with_measured_post(data_dir: &Path, measured: Duration) -> Self {
        let mut approvals = Self::new(data_dir);
        approvals.measured_post = measured;
        approvals
    }

    /// An approval for `batch_id` already taken by a posting call, with no
    /// dialog or request behind it: enough for the cancellation path's tests.
    #[cfg(test)]
    pub(in crate::agent) fn redeeming_for_test(&self, batch_id: &str) -> Uuid {
        let id = Uuid::new_v4();
        *self.slot() = Some((
            batch_id.to_string(),
            Held::Redeeming {
                id,
                answered: Answered {
                    approved: true,
                    at: std::time::Instant::now(),
                    at_wall: SystemTime::now(),
                },
            },
        ));
        id
    }

    /// Whether anything is held for `batch_id`.
    #[cfg(test)]
    pub(in crate::agent) fn holds(&self, batch_id: &str) -> bool {
        self.slot()
            .as_ref()
            .is_some_and(|(held_batch, _)| held_batch == batch_id)
    }

    fn slot(&self) -> std::sync::MutexGuard<'_, Option<(String, Held)>> {
        self.held
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn refusals(&self) -> std::sync::MutexGuard<'_, std::collections::VecDeque<KeptRefusal>> {
        let mut refusals = self
            .refusals
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        // A refusal is kept as long as an approval would be, from the click,
        // on both clocks.
        refusals.retain(|kept| !self.expired(&kept.answered));
        refusals
    }

    /// Keep `batch_id`'s refusal, given at `answered`, for its next call, in
    /// place of any earlier one. One already past its time is not kept, so it
    /// never pushes a live one out.
    fn keep_refusal(&self, batch_id: String, code: String, answered: Answered) {
        let mut refusals = self.refusals();
        refusals.retain(|kept| kept.batch_id != batch_id);
        if self.expired(&answered) {
            return;
        }
        refusals.push_back(KeptRefusal {
            batch_id,
            code,
            answered,
        });
        while refusals.len() > MAX_KEPT_REFUSALS {
            refusals.pop_front();
        }
    }

    /// `batch_id`'s kept refusal, removed: it is returned once.
    fn take_refusal(&self, batch_id: &str) -> Option<String> {
        let mut refusals = self.refusals();
        let at = refusals.iter().position(|kept| kept.batch_id == batch_id)?;
        refusals.remove(at).map(|kept| kept.code)
    }

    fn expired(&self, answered: &Answered) -> bool {
        self.remaining(answered).is_zero()
    }

    /// How long an approval given at `answered` may still be redeemed: the
    /// less of what either clock leaves.
    fn remaining(&self, answered: &Answered) -> Duration {
        let wall_age = SystemTime::now()
            .duration_since(answered.at_wall)
            .unwrap_or(Duration::ZERO);
        let age = answered.at.elapsed().max(wall_age);
        self.ttl.saturating_sub(age)
    }

    /// How long `batch_id`'s held approval may still be redeemed, counted from
    /// the click; `None` when no approval of it is held.
    pub(super) fn approval_remaining(&self, batch_id: &str) -> Option<Duration> {
        match self.slot().as_ref() {
            Some((held_batch, Held::Approved { answered, .. })) if held_batch == batch_id => {
                Some(self.remaining(answered))
            }
            _ => None,
        }
    }

    /// Whether `batch_id`'s held dialog has an answer stamped, for tests that
    /// must wait for a person's scripted click to land.
    #[cfg(test)]
    pub(in crate::agent) fn answered_for_test(&self, batch_id: &str) -> bool {
        matches!(
            self.slot().as_ref(),
            Some((held_batch, Held::Pending { dialog: Some(dialog), .. }))
                if held_batch == batch_id && dialog.answered().is_some()
        )
    }

    /// Whether a call is waiting on `batch_id`'s held dialog: it has taken the
    /// dialog out of the slot to wait on it.
    #[cfg(test)]
    pub(in crate::agent) fn joined_for_test(&self, batch_id: &str) -> bool {
        matches!(
            self.slot().as_ref(),
            Some((held_batch, Held::Pending { dialog: None, .. })) if held_batch == batch_id
        )
    }

    /// What a call re-entered to redeem a joined approval finds (#725 slice
    /// 2.0). Only this batch's approval, still in its time, lets it go on:
    /// anything else is refused, and nothing is asked, so such a call can
    /// never show a second dialog.
    pub(super) fn begin_redeem(&self, batch_id: &str) -> Result<(), String> {
        let mut slot = self.slot();
        let expired = matches!(
            slot.as_ref(),
            Some((held_batch, Held::Approved { answered, .. }))
                if held_batch == batch_id && self.expired(answered)
        );
        self.settle(&mut slot);
        if expired {
            return Err("import_approval_expired".into());
        }
        match slot.as_ref() {
            Some((held_batch, Held::Approved { .. })) if held_batch == batch_id => Ok(()),
            Some((held_batch, Held::Redeeming { .. })) if held_batch == batch_id => {
                Err("import_approval_in_use".into())
            }
            _ => Err("import_approval_revoked".into()),
        }
    }

    pub(super) fn begin(&self, batch_id: &str) -> Begin {
        let mut slot = self.slot();
        self.settle(&mut slot);
        // A No, a timeout or an unanswered end this batch has not read is
        // returned to this call, and the person is not asked again (#725).
        if let Some(code) = self.take_refusal(batch_id) {
            return Begin::Refused(code);
        }
        let Some((held_batch, held)) = slot.as_mut() else {
            return Begin::Ask;
        };
        if *held_batch != batch_id {
            return Begin::Busy("post_approval_busy");
        }
        match held {
            Held::Pending { dialog, .. } => match dialog.take() {
                Some(dialog) => Begin::Join(dialog),
                None => Begin::Waiting,
            },
            Held::Approved { .. } => Begin::Redeem,
            Held::Redeeming { .. } => Begin::Busy("import_approval_in_use"),
        }
    }

    /// Let go of what can no longer be redeemed: a dialog that ended without
    /// an approval, and an approval past its time, whether or not a call has
    /// collected it yet. Neither blocks another batch. A refusal's code is
    /// kept for its own batch's next call; the dialog itself is dropped.
    fn settle(&self, slot: &mut Option<(String, Held)>) {
        if let Some((code, answered)) = slot.as_ref().and_then(|(_, held)| refusal_of(held)) {
            if let Some((batch_id, _dialog)) = slot.take() {
                self.keep_refusal(batch_id, code, answered);
            }
            return;
        }
        let finished = match slot.as_ref() {
            Some((
                _,
                Held::Pending {
                    dialog: Some(dialog),
                    ..
                },
            )) => dialog
                .answered()
                .is_some_and(|answered| self.expired(&answered)),
            Some((_, Held::Approved { answered, .. })) => self.expired(answered),
            _ => false,
        };
        if finished {
            if let Some((batch_id, held)) = slot.take() {
                self.lapse(&batch_id, held, "approval_expired");
            }
        }
    }

    /// Hold a dialog the asking call could not wait out. Refused, and the
    /// dialog closed, when that call was withdrawn meanwhile.
    pub(super) fn hold_pending(
        &self,
        batch_id: &str,
        binding: ApprovalBinding,
        dialog: PendingPostApproval,
        native: NativePostRequest,
    ) -> Result<(), String> {
        let mut slot = self.slot();
        // Checked under the lock: a withdrawal cancels before it revokes, so
        // a hold either lands before the revocation or sees the cancellation.
        if call_withdrawn() {
            return Err("request_cancelled".into());
        }
        if slot.is_some() {
            // Dropping `dialog` closes it: nothing can hold two.
            return Err("post_approval_busy".into());
        }
        *slot = Some((
            batch_id.to_string(),
            Held::Pending {
                binding,
                dialog: Some(dialog),
                native,
            },
        ));
        Ok(())
    }

    /// Settle a joined wait: put an open dialog back, keep an approval that
    /// has not run out, and let go of a refusal.
    pub(super) fn settle_join(
        &self,
        batch_id: &str,
        waited: Result<(Result<ApprovedImport, String>, Answered), PendingPostApproval>,
    ) -> Joined {
        let mut slot = self.slot();
        let Some((held_batch, Held::Pending { dialog, .. })) = slot.as_mut() else {
            // Revoked while this call waited: whatever was answered is dropped.
            return Joined::Refused("import_approval_revoked".into());
        };
        if *held_batch != batch_id {
            return Joined::Refused("import_approval_revoked".into());
        }
        match waited {
            Err(open) => {
                let remaining = open.remaining();
                *dialog = Some(open);
                Joined::StillOpen { remaining }
            }
            Ok((Err(code), _)) => {
                *slot = None;
                Joined::Refused(code)
            }
            Ok((Ok(request), answered)) => {
                let Some((
                    _,
                    Held::Pending {
                        binding, native, ..
                    },
                )) = slot.take()
                else {
                    unreachable!("matched as pending above");
                };
                let held = approved(binding, request, native, answered);
                if self.expired(&answered) {
                    self.lapse(batch_id, held, "approval_expired");
                    return Joined::Refused("import_approval_expired".into());
                }
                *slot = Some((batch_id.to_string(), held));
                Joined::Approved
            }
        }
    }

    /// Keep an approval answered in the asking call for a later one, or for
    /// this call to take at once. Refused when the call was withdrawn.
    pub(super) fn hold_approved(
        &self,
        batch_id: &str,
        binding: ApprovalBinding,
        request: ApprovedImport,
        native: NativePostRequest,
        answered: Answered,
    ) -> Result<(), String> {
        let mut slot = self.slot();
        if call_withdrawn() {
            return Err("request_cancelled".into());
        }
        if slot.is_some() {
            return Err("post_approval_busy".into());
        }
        *slot = Some((
            batch_id.to_string(),
            approved(binding, request, native, answered),
        ));
        Ok(())
    }

    /// Take this batch's approval for the call about to post it. Refused, and
    /// the approval lapsed, when the fresh checks bound anything other than
    /// what the person was shown, or the approval is for another count.
    pub(super) fn take_for_dispatch(
        &self,
        batch_id: &str,
        fresh: &ApprovalBinding,
    ) -> Result<(Redemption<'_>, ApprovedImport, NativePostRequest), String> {
        let mut slot = self.slot();
        let expired = matches!(
            slot.as_ref(),
            Some((held_batch, Held::Approved { answered, .. }))
                if held_batch == batch_id && self.expired(answered)
        );
        self.settle(&mut slot);
        if expired {
            return Err("import_approval_expired".into());
        }
        match slot.take() {
            Some((
                held_batch,
                Held::Approved {
                    binding,
                    id,
                    request,
                    native,
                    answered,
                },
            )) if held_batch == batch_id => {
                if binding != *fresh || request.voucher_count() != fresh.voucher_count {
                    self.write_lapse_note(batch_id, answered, "approval_binding_changed");
                    return Err("import_approval_binding_changed".into());
                }
                *slot = Some((batch_id.to_string(), Held::Redeeming { id, answered }));
                Ok((
                    Redemption {
                        approvals: self,
                        batch_id: batch_id.to_string(),
                        id,
                    },
                    *request,
                    native,
                ))
            }
            other => {
                // Another call already took it: in use, not revoked.
                let in_use = matches!(
                    &other,
                    Some((held_batch, Held::Redeeming { .. })) if held_batch == batch_id
                );
                *slot = other;
                Err(if in_use {
                    "import_approval_in_use"
                } else {
                    "import_approval_revoked"
                }
                .into())
            }
        }
    }

    /// Spend the approval, once. Called under the import admission lock just
    /// before the dispatch intent is written; a second call, or one after a
    /// revocation, is refused and writes no intent.
    pub(super) fn spend(&self, batch_id: &str, approval: Uuid) -> Result<(), UnderLockRefusal> {
        let mut slot = self.slot();
        match slot.as_ref() {
            Some((held_batch, Held::Redeeming { id, .. }))
                if held_batch == batch_id && *id == approval =>
            {
                *slot = None;
                Ok(())
            }
            _ => Err(UnderLockRefusal::ApprovalRevoked),
        }
    }

    /// An approval taken and not spent lapses; it is never offered again.
    fn release_unspent(&self, batch_id: &str, approval: Uuid, reason: &str) {
        let mut slot = self.slot();
        if let Some((held_batch, Held::Redeeming { id, answered })) = slot.as_ref() {
            if held_batch == batch_id && *id == approval {
                let answered = *answered;
                *slot = None;
                self.write_lapse_note(batch_id, answered, reason);
            }
        }
    }

    /// Withdraw whatever is held for `batch_id`: an open dialog is closed, an
    /// approval, redeemed or not, can no longer be spent, and an unread refusal
    /// is dropped, so the next call asks again.
    pub(in crate::agent) fn revoke(&self, batch_id: &str, reason: &str) {
        let mut slot = self.slot();
        self.refusals().retain(|kept| kept.batch_id != batch_id);
        if slot
            .as_ref()
            .is_some_and(|(held_batch, _)| held_batch == batch_id)
        {
            if let Some((_, held)) = slot.take() {
                self.lapse(batch_id, held, reason);
            }
        }
    }

    /// Withdraw `batch_id`'s approval if it was given and not taken: a call
    /// that was to redeem it was refused before taking it. That is an
    /// approval a call has collected (`Approved`), and one clicked while no call
    /// waited, whose dialog still sits in the slot with its answer. An open or
    /// declined dialog, one a call is waiting on, a taken approval and another
    /// batch's hold are left as they are.
    pub(super) fn revoke_unredeemed(&self, batch_id: &str, reason: &str) {
        let mut slot = self.slot();
        if matches!(
            slot.as_ref(),
            Some((held_batch, Held::Approved { .. })) if held_batch == batch_id
        ) || matches!(
            slot.as_ref(),
            Some((held_batch, Held::Pending { dialog: Some(dialog), .. }))
                if held_batch == batch_id
                    && dialog.answered().is_some_and(|answered| answered.approved)
        ) {
            if let Some((_, held)) = slot.take() {
                self.lapse(batch_id, held, reason);
            }
        }
    }

    /// The last approval of `batch_id` that lapsed unredeemed, if any. It is a
    /// report only: nothing reads it as an approval.
    pub(super) fn lapse_note(&self, batch_id: &str) -> Option<Value> {
        let file = std::fs::File::open(self.lapse_note_path(batch_id)?).ok()?;
        let mut bytes = Vec::new();
        file.take(MAX_LAPSE_NOTE_BYTES + 1)
            .read_to_end(&mut bytes)
            .ok()?;
        if bytes.len() as u64 > MAX_LAPSE_NOTE_BYTES {
            return None;
        }
        serde_json::from_slice(&bytes).ok()
    }

    /// Let go of `held`: a dialog is closed by dropping it; an approval that
    /// was given, collected or not, leaves a note.
    fn lapse(&self, batch_id: &str, held: Held, reason: &str) {
        let answered = match &held {
            Held::Pending {
                dialog: Some(dialog),
                ..
            } => dialog.answered().filter(|answered| answered.approved),
            Held::Pending { dialog: None, .. } => None,
            Held::Approved { answered, .. } | Held::Redeeming { answered, .. } => Some(*answered),
        };
        drop(held);
        if let Some(answered) = answered {
            self.write_lapse_note(batch_id, answered, reason);
        }
    }

    fn lapse_note_path(&self, batch_id: &str) -> Option<PathBuf> {
        let uuid = batch_id
            .strip_prefix("bridge-")
            .and_then(|id| Uuid::parse_str(id).ok())?;
        Some(
            self.imports
                .join(format!("bridge-{uuid}.approval_lapse.json")),
        )
    }

    /// Best effort: a note that cannot be written loses a report, never an
    /// approval, since none is ever read back from it.
    fn write_lapse_note(&self, batch_id: &str, answered: Answered, reason: &str) {
        let Some(path) = self.lapse_note_path(batch_id) else {
            return;
        };
        let note = json!({
            "batch_id": batch_id,
            "state": "approval_lapsed_unposted",
            "reason": reason,
            "approved_at": DateTime::<Utc>::from(answered.at_wall)
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            "lapsed_at": Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            "redeemable": false,
        });
        if self.imports.is_dir() {
            let _ = write_private(&path, note.to_string().as_bytes());
        }
    }
}

impl Drop for PostApprovals {
    fn drop(&mut self) {
        if let Some((batch_id, held)) = self.slot().take() {
            self.lapse(&batch_id, held, "process_ended");
        }
    }
}

fn approved(
    binding: ApprovalBinding,
    request: ApprovedImport,
    native: NativePostRequest,
    answered: Answered,
) -> Held {
    Held::Approved {
        binding,
        id: Uuid::new_v4(),
        request: Box::new(request),
        native,
        answered,
    }
}
