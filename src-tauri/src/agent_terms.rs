//! The Terms-of-Use gate of the MCP server.
//!
//! The extension asks the user to accept the Terms of Use in its settings, and the setting arrives
//! here as `BRIDGE_TERMS_ACCEPTED`. A host cannot be relied on to make a required checkbox mean
//! "ticked" (an unticked box is still a value), so the server is the gate: until the setting is on,
//! every `tools/call` refuses in band, before any argument is read and before any request to
//! Tally. `initialize` and `tools/list` still answer, so the host shows the tools and the user
//! reads the reason in the chat rather than in a generic "server disconnected".
//!
//! An absent, empty or unrecognised value is "not accepted". When it is on, a line per terms
//! version is appended to `terms-acceptance.jsonl` in the data folder (version, time, source); if
//! that line cannot be written the server refuses too, the same fail-closed rule the receipt log
//! has. Nothing here is sent anywhere.
//!
//! This is a local record that the gate opened, not a security boundary and not proof of who
//! accepted, when they ticked the box or which text they saw: anyone who runs `bridge_mcp` by hand
//! can set the variable, and doing so is their act. It is written when the server starts, not at
//! the tick; two servers starting together can each append a line for the same version. A
//! hand-set `true` names no version, so it also opens the gate for a later terms version. `Server::new` (tests and the desktop app's local
//! views, which never dispatch an MCP tool) stays open; only `Server::for_mcp`, which
//! `run_stdio` uses, builds a server that answers an MCP client.
use super::egress::{append_egress_line, read_egress_tail};
use chrono::{SecondsFormat, Utc};
use serde_json::{json, Value};
use std::path::Path;

/// The version of the Terms of Use this build asks the user to accept. The manifest's setting is
/// named after it (`accept_terms_2026_10`), so new terms mean a new setting and a new prompt: an
/// old "true" cannot stand for terms that changed. A test pins the manifest to this constant.
pub(super) const TERMS_VERSION: &str = "2026-10";
/// The environment variable the manifest maps the accept-terms setting to.
pub(super) const TERMS_ENV: &str = "BRIDGE_TERMS_ACCEPTED";
const RECORD_FILE: &str = "terms-acceptance.jsonl";
/// The record file holds a line per terms version; this bounds the tail read.
const RECORD_SCAN_LINES: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TermsState {
    NotAccepted,
    /// Accepted, but the acceptance could not be recorded locally.
    RecordUnavailable,
    Accepted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TermsGate {
    /// Tests and the desktop app's local views: no MCP client, so nothing to accept.
    NotRequired,
    /// The MCP stdio server.
    Required(TermsState),
}

impl TermsGate {
    /// The gate for the server that answers an MCP client. `setting` is the raw value of
    /// [`TERMS_ENV`]; only `true` and `1` accept.
    pub(super) fn for_mcp(setting: Option<&str>, data_dir: &Path) -> Self {
        if !matches!(setting, Some("true" | "1")) {
            return Self::Required(TermsState::NotAccepted);
        }
        match record_acceptance(data_dir) {
            Ok(()) => Self::Required(TermsState::Accepted),
            Err(_) => Self::Required(TermsState::RecordUnavailable),
        }
    }

    /// The refusal code every tool call returns, or `None` when calls may proceed.
    pub(super) fn refusal(self) -> Option<&'static str> {
        match self {
            Self::NotRequired | Self::Required(TermsState::Accepted) => None,
            Self::Required(TermsState::NotAccepted) => Some("terms_not_accepted"),
            Self::Required(TermsState::RecordUnavailable) => Some("terms_record_unavailable"),
        }
    }
}

/// Appends the acceptance of [`TERMS_VERSION`] once. A version already recorded is not recorded
/// again.
fn record_acceptance(data_dir: &Path) -> Result<(), String> {
    let path = data_dir.join(RECORD_FILE);
    let tail = read_egress_tail(&path, RECORD_SCAN_LINES)?;
    let already = tail.records.iter().any(|line| {
        serde_json::from_str::<Value>(line)
            .ok()
            .is_some_and(|record| record["terms_version"] == TERMS_VERSION)
    });
    if already {
        return Ok(());
    }
    let record = json!({
        "terms_version": TERMS_VERSION,
        "accepted_at": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        "source": "setting",
    });
    append_egress_line(&path, &record.to_string())
}

#[cfg(test)]
#[path = "agent_terms_tests.rs"]
mod tests;
