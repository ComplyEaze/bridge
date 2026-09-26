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
//! person was shown, redeemed once, and spent under the import admission lock
//! before the dispatch intent is written; a cancelled or refused redemption
//! spends it too. An approval that lapses unredeemed leaves a note saying so,
//! and nothing can post from a note.
use super::post::NativePostRequest;
use super::{sha256_hex, write_private, ImportCompanyTuple, ImportLedgerLine};
use crate::tally::approved_import::{ApprovedImport, PendingPostApproval};
use bridge_tally_protocol::StandardLedgerCatalogBinding;
use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::{json, Value};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// The longest one `post_import` call spends before it returns, well inside
/// the roughly 60-second host timeout observed once under #485. Unit tests
/// wait on scripted dialogs, where a long budget only slows them.
#[cfg(not(test))]
pub(super) const CALL_BUDGET: Duration = Duration::from_secs(40);
#[cfg(test)]
pub(super) const CALL_BUDGET: Duration = Duration::from_millis(300);
/// The most one call may take in all: three quarters of the roughly 60-second
/// host timeout observed once (#485, #703).
pub(super) const CALL_CEILING: Duration = Duration::from_secs(45);
/// What a post took from its approval to its result at the largest batch
/// measured live: 200 Journals (D4, 26 Sep 2026, bridge#725), from the answer
/// to the end of the call, 20.95 s: the queue's re-checks 3.0 s, the POST 5.2 s,
/// the readback 12.1 s, and the spacing between them. A concurrent build ran,
/// so it overstates a quiet book; it is not measured above 200.
const MEASURED_POST: Duration = Duration::from_millis(20_950);
const MEASURED_POST_VOUCHERS: usize = 200;

/// Whether an approval answered `elapsed` into its call may be posted in that
/// same call, as before #725: only when the measured cost of the post still
/// fits under the ceiling. Otherwise the next call redeems it, after checking
/// the book again.
pub(super) fn dispatch_fits_in_call(elapsed: Duration, vouchers: usize) -> bool {
    vouchers <= MEASURED_POST_VOUCHERS
        && elapsed
            .checked_add(MEASURED_POST)
            .is_some_and(|total| total <= CALL_CEILING)
}
/// How long an answered approval may wait to be redeemed. The owner's
/// decision (#725); 15 minutes is the proposal it was put to them with.
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
    /// The dialog is open. `dialog` is `None` while a call waits on it.
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
        approved_at: Instant,
        approved_at_utc: DateTime<Utc>,
    },
    /// Taken by the call posting it; spent when that call's intent is admitted.
    Redeeming {
        id: Uuid,
        approved_at_utc: DateTime<Utc>,
    },
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
}

/// What waiting on a joined dialog came to.
pub(super) enum Joined {
    StillOpen { remaining: Duration },
    Approved,
    Refused(String),
}

/// The approvals one process holds: at most one batch at a time.
pub(in crate::agent) struct PostApprovals {
    imports: PathBuf,
    held: Mutex<Option<(String, Held)>>,
    ttl: Duration,
}

impl PostApprovals {
    pub(in crate::agent) fn new(data_dir: &Path) -> Self {
        Self {
            imports: data_dir.join("imports"),
            held: Mutex::new(None),
            ttl: APPROVAL_TTL,
        }
    }

    #[cfg(test)]
    pub(super) fn with_ttl(data_dir: &Path, ttl: Duration) -> Self {
        let mut approvals = Self::new(data_dir);
        approvals.ttl = ttl;
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
                approved_at_utc: Utc::now(),
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

    pub(super) fn begin(&self, batch_id: &str) -> Begin {
        let mut slot = self.slot();
        self.lapse_if_expired(&mut slot);
        let Some((held_batch, held)) = slot.as_mut() else {
            return Begin::Ask;
        };
        if held_batch != batch_id {
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

    /// Hold a dialog the asking call could not wait out.
    pub(super) fn hold_pending(
        &self,
        batch_id: &str,
        binding: ApprovalBinding,
        dialog: PendingPostApproval,
        native: NativePostRequest,
    ) -> Result<(), String> {
        let mut slot = self.slot();
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

    /// Settle a joined wait: put an open dialog back, keep an approval, and
    /// let go of a refusal.
    pub(super) fn settle_join(
        &self,
        batch_id: &str,
        waited: Result<Result<ApprovedImport, String>, PendingPostApproval>,
    ) -> Joined {
        let mut slot = self.slot();
        let Some((held_batch, Held::Pending { dialog, .. })) = slot.as_mut() else {
            // Revoked while this call waited: whatever was answered is dropped.
            return Joined::Refused("import_approval_revoked".into());
        };
        if held_batch != batch_id {
            return Joined::Refused("import_approval_revoked".into());
        }
        match waited {
            Err(open) => {
                let remaining = open.remaining();
                *dialog = Some(open);
                Joined::StillOpen { remaining }
            }
            Ok(Err(code)) => {
                *slot = None;
                Joined::Refused(code)
            }
            Ok(Ok(request)) => {
                let Some((
                    _,
                    Held::Pending {
                        binding, native, ..
                    },
                )) = slot.take()
                else {
                    unreachable!("matched as pending above");
                };
                *slot = Some((batch_id.to_string(), approved(binding, request, native)));
                Joined::Approved
            }
        }
    }

    /// Keep an approval answered in the asking call for a later one.
    pub(super) fn hold_approved(
        &self,
        batch_id: &str,
        binding: ApprovalBinding,
        request: ApprovedImport,
        native: NativePostRequest,
    ) -> Result<(), String> {
        let mut slot = self.slot();
        if slot.is_some() {
            return Err("post_approval_busy".into());
        }
        *slot = Some((batch_id.to_string(), approved(binding, request, native)));
        Ok(())
    }

    /// Take this batch's approval for the call about to post it. Refused, and
    /// the approval lapsed, when the fresh checks bound anything other than
    /// what the person was shown, or the approval is for another count.
    pub(super) fn take_for_dispatch(
        &self,
        batch_id: &str,
        fresh: &ApprovalBinding,
    ) -> Result<(Uuid, ApprovedImport, NativePostRequest), String> {
        let mut slot = self.slot();
        self.lapse_if_expired(&mut slot);
        match slot.take() {
            Some((
                held_batch,
                Held::Approved {
                    binding,
                    id,
                    request,
                    native,
                    approved_at_utc,
                    ..
                },
            )) if held_batch == batch_id => {
                if binding != *fresh || request.voucher_count() != fresh.voucher_count {
                    self.write_lapse_note(batch_id, approved_at_utc, "approval_binding_changed");
                    return Err("import_approval_binding_changed".into());
                }
                *slot = Some((
                    batch_id.to_string(),
                    Held::Redeeming {
                        id,
                        approved_at_utc,
                    },
                ));
                Ok((id, *request, native))
            }
            other => {
                *slot = other;
                Err("import_approval_revoked".into())
            }
        }
    }

    /// Spend the approval, once. Called under the import admission lock just
    /// before the dispatch intent is written; a second call, or one after a
    /// revocation, is refused and writes no intent.
    pub(super) fn spend(&self, batch_id: &str, approval: Uuid) -> Result<(), String> {
        let mut slot = self.slot();
        match slot.as_ref() {
            Some((held_batch, Held::Redeeming { id, .. }))
                if held_batch == batch_id && *id == approval =>
            {
                *slot = None;
                Ok(())
            }
            _ => Err("import_approval_revoked".into()),
        }
    }

    /// After a redeeming call ends: an approval it did not spend (refused
    /// before its intent) lapses; it is never offered again.
    pub(super) fn release_unspent(&self, batch_id: &str, approval: Uuid, reason: &str) {
        let mut slot = self.slot();
        if let Some((
            held_batch,
            Held::Redeeming {
                id,
                approved_at_utc,
            },
        )) = slot.as_ref()
        {
            if held_batch == batch_id && *id == approval {
                let approved_at_utc = *approved_at_utc;
                *slot = None;
                self.write_lapse_note(batch_id, approved_at_utc, reason);
            }
        }
    }

    /// Withdraw whatever is held for `batch_id`: an open dialog is closed and
    /// an approval, redeemed or not, can no longer be spent.
    pub(in crate::agent) fn revoke(&self, batch_id: &str, reason: &str) {
        let mut slot = self.slot();
        if slot
            .as_ref()
            .is_some_and(|(held_batch, _)| held_batch == batch_id)
        {
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

    fn lapse_if_expired(&self, slot: &mut Option<(String, Held)>) {
        let expired = matches!(
            slot.as_ref(),
            Some((_, Held::Approved { approved_at, .. })) if approved_at.elapsed() > self.ttl
        );
        if expired {
            if let Some((batch_id, held)) = slot.take() {
                self.lapse(&batch_id, held, "approval_expired");
            }
        }
    }

    /// Let go of `held`: a dialog is closed by dropping it; an approval that
    /// was given leaves a note.
    fn lapse(&self, batch_id: &str, held: Held, reason: &str) {
        match held {
            Held::Pending { .. } => {}
            Held::Approved {
                approved_at_utc, ..
            }
            | Held::Redeeming {
                approved_at_utc, ..
            } => self.write_lapse_note(batch_id, approved_at_utc, reason),
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
    fn write_lapse_note(&self, batch_id: &str, approved_at: DateTime<Utc>, reason: &str) {
        let Some(path) = self.lapse_note_path(batch_id) else {
            return;
        };
        let note = json!({
            "batch_id": batch_id,
            "state": "approval_lapsed_unposted",
            "reason": reason,
            "approved_at": approved_at.to_rfc3339_opts(SecondsFormat::Secs, true),
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

fn approved(binding: ApprovalBinding, request: ApprovedImport, native: NativePostRequest) -> Held {
    Held::Approved {
        binding,
        id: Uuid::new_v4(),
        request: Box::new(request),
        native,
        approved_at: Instant::now(),
        approved_at_utc: Utc::now(),
    }
}
