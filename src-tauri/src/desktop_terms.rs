// SPDX-License-Identifier: Apache-2.0

//! The desktop app's acceptance of the Terms of Use (bridge#1106).
//!
//! The extension makes the user accept the Terms in its settings and its server refuses every tool
//! until they have (`agent/terms`). The desktop app has the same rule: [`TallyRuntime::desktop`]
//! refuses every send to Tally, and so every read and every post, until this record says the user
//! accepted the Terms version the build carries. The window shows the acceptance before anything
//! else (`src/TermsGate.tsx`).
//!
//! The record is the extension's own record format: a line per terms version in
//! `terms-acceptance.jsonl`, here in the app's data folder with `source` `desktop_app`. A line
//! counts for the source that wrote it, so an acceptance in the extension, which starts from
//! another folder and another source, does not open the app. A new `TERMS_VERSION` finds no line
//! and asks again. Nothing is sent anywhere. Local reads (the mirror, saved drafts) are not gated:
//! they send nothing to Tally, and the mirror is filled only through gated paths.
//!
//! Fail closed: a record that cannot be read (a torn last line, a folder that cannot be read) or
//! written is `terms_record_unavailable`, which refuses like `terms_not_accepted` and tells the
//! user the folder to look at. The gate only opens within a process; there is no revoke, and a new
//! terms version arrives with a new build and so a new process.
//!
//! [`TallyRuntime::desktop`]: crate::tally::TallyRuntime::desktop
use crate::agent::terms::{
    acceptance_recorded, record_acceptance, TermsGate, TermsState, TERMS_VERSION,
};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Mutex;

/// The `source` of the record line the app writes.
pub(crate) const DESKTOP_SOURCE: &str = "desktop_app";

/// What the window needs to show the acceptance: the version to name on the control, and whether
/// the gate is open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct TermsStatus {
    pub(crate) terms_version: &'static str,
    /// `accepted`, `not_accepted` or `record_unavailable`.
    pub(crate) state: &'static str,
    pub(crate) accepted: bool,
}

/// Why [`DesktopTerms::accept`] did not record an acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AcceptRefusal {
    /// The window named another Terms version than this build carries: it is showing old text.
    StaleVersion,
    /// The record could not be written or read.
    RecordUnavailable,
}

pub(crate) struct DesktopTerms {
    data_dir: PathBuf,
    gate: Mutex<TermsGate>,
}

impl DesktopTerms {
    /// Reads the record in `data_dir`. A missing record is "not accepted"; one that cannot be read
    /// is "record unavailable".
    pub(crate) fn new(data_dir: PathBuf) -> Self {
        let gate = read_gate(&data_dir);
        Self {
            data_dir,
            gate: Mutex::new(gate),
        }
    }

    /// The refusal code every send returns, or `None` once the terms are accepted. A poisoned lock
    /// refuses.
    pub(crate) fn refusal(&self) -> Option<&'static str> {
        match self.gate.lock() {
            Ok(gate) => gate.refusal(),
            Err(_) => Some("terms_not_accepted"),
        }
    }

    /// The state for the window. While the gate is closed the record is read again, so a second
    /// window (or an acceptance made by another instance of the app) is seen; an open gate stays
    /// open.
    pub(crate) fn status(&self) -> TermsStatus {
        if let Ok(mut gate) = self.gate.lock() {
            if gate.refusal().is_some() {
                *gate = read_gate(&self.data_dir);
            }
        }
        self.snapshot()
    }

    /// Records the acceptance of the Terms version the window showed and opens the gate. A window
    /// that names another version records nothing; a record that cannot be written leaves the
    /// gate closed.
    pub(crate) fn accept(&self, version: &str) -> Result<TermsStatus, AcceptRefusal> {
        if version != TERMS_VERSION {
            return Err(AcceptRefusal::StaleVersion);
        }
        let outcome = prepare_folder(&self.data_dir)
            .and_then(|()| record_acceptance(&self.data_dir, DESKTOP_SOURCE));
        let Ok(mut gate) = self.gate.lock() else {
            return Err(AcceptRefusal::RecordUnavailable);
        };
        match outcome {
            Ok(()) => {
                *gate = TermsGate::Required(TermsState::Accepted);
                drop(gate);
                Ok(self.snapshot())
            }
            Err(_) => {
                // A write that failed does not close a gate another instance's acceptance has
                // just opened (a concurrent status() re-read can have seen it).
                if *gate != TermsGate::Required(TermsState::Accepted) {
                    *gate = TermsGate::Required(TermsState::RecordUnavailable);
                }
                Err(AcceptRefusal::RecordUnavailable)
            }
        }
    }

    fn snapshot(&self) -> TermsStatus {
        let state = match self.gate.lock().map(|gate| *gate) {
            Ok(TermsGate::Required(TermsState::Accepted)) | Ok(TermsGate::NotRequired) => {
                "accepted"
            }
            Ok(TermsGate::Required(TermsState::RecordUnavailable)) => "record_unavailable",
            Ok(TermsGate::Required(TermsState::NotAccepted)) | Err(_) => "not_accepted",
        };
        TermsStatus {
            terms_version: TERMS_VERSION,
            state,
            accepted: state == "accepted",
        }
    }
}

fn read_gate(data_dir: &std::path::Path) -> TermsGate {
    match acceptance_recorded(data_dir, DESKTOP_SOURCE) {
        Ok(true) => TermsGate::Required(TermsState::Accepted),
        Ok(false) => TermsGate::Required(TermsState::NotAccepted),
        Err(_) => TermsGate::Required(TermsState::RecordUnavailable),
    }
}

/// The app's data folder, private to the user, before a record is written into it.
fn prepare_folder(data_dir: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(data_dir).map_err(|error| error.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(data_dir, std::fs::Permissions::from_mode(0o700))
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "desktop_terms_tests.rs"]
mod tests;
