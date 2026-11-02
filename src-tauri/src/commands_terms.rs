// SPDX-License-Identifier: Apache-2.0

//! The commands behind the window's Terms-of-Use acceptance (bridge#1106) and the refusal every
//! Tally command returns until it is made.
use super::{tally_command_error, TallyCommandError};
use crate::desktop_terms::{AcceptRefusal, DesktopTerms, TermsStatus};
use std::sync::Arc;
use tauri::State;

/// The refusal for `terms_not_accepted` and `terms_record_unavailable`, in plain words.
pub(crate) fn refusal_command_error(code: &'static str) -> TallyCommandError {
    if code == "terms_record_unavailable" {
        return record_unavailable();
    }
    tally_command_error(
        "terms_not_accepted",
        "Terms of Use",
        "ComplyEaze Bridge reads nothing from Tally and posts nothing until you accept the Terms of Use.",
        "after_change",
        false,
        "Accept the Terms of Use in the Bridge window, then try again.",
    )
}

fn record_unavailable() -> TallyCommandError {
    tally_command_error(
        "terms_record_unavailable",
        "Terms of Use",
        "ComplyEaze Bridge could not read or write its record of the Terms of Use, so it reads nothing from Tally and posts nothing.",
        "after_change",
        false,
        "Check that the Bridge data folder (com.complyeaze.bridge, in your Application Support or AppData folder) can be written, then accept the Terms again. If that fails too, its terms-acceptance.jsonl holds a line Bridge did not write: move that file aside and accept the Terms again.",
    )
}

#[tauri::command]
pub fn desktop_terms_status(terms: State<'_, Arc<DesktopTerms>>) -> TermsStatus {
    terms.status()
}

#[tauri::command]
pub fn desktop_accept_terms(
    version: String,
    terms: State<'_, Arc<DesktopTerms>>,
) -> Result<TermsStatus, TallyCommandError> {
    terms.accept(&version).map_err(|refusal| match refusal {
        AcceptRefusal::StaleVersion => tally_command_error(
            "terms_version_changed",
            "Terms of Use",
            "The window showed a different version of the Terms of Use than this build carries, so nothing was recorded.",
            "after_change",
            false,
            "Reload the Bridge window and read the Terms of Use it shows now.",
        ),
        AcceptRefusal::RecordUnavailable => record_unavailable(),
    })
}

#[cfg(test)]
mod tests {
    use super::super::tally_runtime_command_error;
    use super::refusal_command_error;
    use crate::tally::runtime::TermsRefusal;

    #[test]
    fn a_runtime_refusal_reaches_the_window_as_a_typed_error_in_plain_words() {
        for (code, plain) in [
            ("terms_not_accepted", "until you accept the Terms of Use"),
            (
                "terms_record_unavailable",
                "could not read or write its record",
            ),
        ] {
            let error = tally_runtime_command_error(
                anyhow::Error::new(TermsRefusal { code }).context("the read could not start"),
            );
            assert_eq!(error.code, code);
            assert!(error.message.contains(plain), "{}", error.message);
            assert!(!error.local_state_changed && !error.tally_state_may_have_changed);
            assert_eq!(error.category, refusal_command_error(code).category);
        }
    }
}
