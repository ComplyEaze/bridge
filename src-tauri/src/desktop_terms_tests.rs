// SPDX-License-Identifier: Apache-2.0

use super::{AcceptRefusal, DesktopTerms, DESKTOP_SOURCE};
use crate::agent::terms::TERMS_VERSION;
use crate::tally::runtime::TermsRefusal;
use crate::tally::{TallyConfig, TallyRuntime};
use std::path::Path;
use std::sync::Arc;

const RECORD: &str = "terms-acceptance.jsonl";

/// A port nothing listens on (the discard port): a call that got past the gate comes back with a
/// connection error, not a terms refusal, so the refusal is also the proof that nothing was sent.
fn config() -> TallyConfig {
    TallyConfig {
        host: "127.0.0.1".to_string(),
        port: 9,
    }
}

fn record_lines(directory: &Path) -> Vec<serde_json::Value> {
    match std::fs::read_to_string(directory.join(RECORD)) {
        Ok(text) => text
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn refusal_code(error: &anyhow::Error) -> Option<&'static str> {
    error
        .chain()
        .find_map(|cause| cause.downcast_ref::<TermsRefusal>())
        .map(|refusal| refusal.code)
}

#[tokio::test]
async fn the_app_reads_and_posts_nothing_until_the_terms_are_accepted_and_does_after() {
    let directory = tempfile::tempdir().unwrap();
    let terms = Arc::new(DesktopTerms::new(directory.path().to_path_buf()));
    let runtime = TallyRuntime::desktop(Arc::clone(&terms));

    // Before: the session-opening paths (a read, a probe) stop with the refusal.
    let read = runtime.fetch_companies(config()).await.unwrap_err();
    assert_eq!(refusal_code(&read), Some("terms_not_accepted"));
    let probe = runtime
        .probe_with_observation(config())
        .await
        .map(|_| ())
        .unwrap_err();
    assert_eq!(refusal_code(&probe), Some("terms_not_accepted"));
    assert!(runtime.snapshots().unwrap().is_empty());
    assert!(
        record_lines(directory.path()).is_empty(),
        "a refusal records nothing"
    );

    // After: the same calls get as far as Tally, so they fail on the connection, not the terms.
    let status = terms.accept(TERMS_VERSION).unwrap();
    assert!(status.accepted && status.state == "accepted");
    let read = runtime.fetch_companies(config()).await.unwrap_err();
    assert_eq!(refusal_code(&read), None, "{read:#}");
}

#[tokio::test]
async fn a_runtime_without_the_gate_is_not_refused() {
    let read = TallyRuntime::ungated()
        .fetch_companies(config())
        .await
        .unwrap_err();
    assert_eq!(refusal_code(&read), None, "{read:#}");
}

#[test]
fn acceptance_is_recorded_with_the_version_the_time_and_that_it_was_made_in_the_app() {
    let directory = tempfile::tempdir().unwrap();
    let terms = DesktopTerms::new(directory.path().to_path_buf());
    terms.accept(TERMS_VERSION).unwrap();
    terms.accept(TERMS_VERSION).unwrap();
    let lines = record_lines(directory.path());
    assert_eq!(lines.len(), 1, "one line per version and source");
    assert_eq!(lines[0]["terms_version"], TERMS_VERSION);
    assert_eq!(lines[0]["source"], DESKTOP_SOURCE);
    assert!(lines[0]["accepted_at"]
        .as_str()
        .is_some_and(|time| time.ends_with('Z')));
}

#[test]
fn a_record_of_another_version_or_another_source_does_not_open_the_gate() {
    let directory = tempfile::tempdir().unwrap();
    let lines = [
        r#"{"terms_version":"2026-09.9","accepted_at":"2026-09-01T00:00:00.000Z","source":"desktop_app"}"#,
        &format!(
            r#"{{"terms_version":"{TERMS_VERSION}","accepted_at":"2026-10-01T00:00:00.000Z","source":"setting"}}"#
        ),
    ];
    std::fs::write(directory.path().join(RECORD), lines.join("\n") + "\n").unwrap();
    let terms = DesktopTerms::new(directory.path().to_path_buf());
    assert_eq!(terms.refusal(), Some("terms_not_accepted"));
    assert_eq!(terms.status().state, "not_accepted");
    // Accepting adds the app's own line beside the others, which stay.
    terms.accept(TERMS_VERSION).unwrap();
    assert_eq!(record_lines(directory.path()).len(), 3);
    assert_eq!(terms.refusal(), None);
}

#[test]
fn an_earlier_acceptance_of_this_version_in_the_app_opens_the_gate_at_start() {
    let directory = tempfile::tempdir().unwrap();
    DesktopTerms::new(directory.path().to_path_buf())
        .accept(TERMS_VERSION)
        .unwrap();
    let restarted = DesktopTerms::new(directory.path().to_path_buf());
    assert_eq!(restarted.refusal(), None);
    assert!(restarted.status().accepted);
}

#[test]
fn a_window_showing_other_terms_cannot_accept_and_nothing_is_recorded() {
    let directory = tempfile::tempdir().unwrap();
    let terms = DesktopTerms::new(directory.path().to_path_buf());
    for version in ["", "2026-09.9", "2026-10.2", " 2026-10.1", "2026_10_1"] {
        assert_eq!(
            terms.accept(version),
            Err(AcceptRefusal::StaleVersion),
            "{version:?}"
        );
    }
    assert_eq!(terms.refusal(), Some("terms_not_accepted"));
    assert!(record_lines(directory.path()).is_empty());
}

/// A torn last line (#2172), even a whole acceptance cut just before its newline, is never an
/// acceptance; accepting again repairs it, keeping its bytes, and is then recorded.
#[test]
fn a_torn_last_line_is_not_an_acceptance_and_accepting_repairs_it() {
    let whole = format!(
        "{{\"terms_version\":\"{TERMS_VERSION}\",\"accepted_at\":\"2026-10-30T00:00:00.000Z\",\"source\":\"desktop_app\"}}"
    );
    for torn in ["{\"terms_version\":\"2026-10".to_string(), whole] {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join(RECORD), &torn).unwrap();
        let terms = DesktopTerms::new(directory.path().to_path_buf());
        assert_eq!(terms.refusal(), Some("terms_not_accepted"), "{torn}");
        assert!(terms.accept(TERMS_VERSION).is_ok());
        assert_eq!(terms.refusal(), None);
        let bytes = std::fs::read(directory.path().join(RECORD)).unwrap();
        assert!(bytes.starts_with(torn.as_bytes()), "nothing is removed");
        let text = String::from_utf8(bytes).unwrap();
        let whole_lines = text
            .lines()
            .filter(|line| serde_json::from_str::<serde_json::Value>(line).is_ok())
            .count();
        assert_eq!(whole_lines, 2, "the marker and the acceptance: {text}");
    }
}

#[test]
fn a_record_that_cannot_be_read_or_written_refuses_and_says_so() {
    // A whole line Bridge did not write: the record cannot be read, so it is neither accepted nor
    // writable.
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join(RECORD), "not a record\n{\"a\":1}\n").unwrap();
    let terms = DesktopTerms::new(directory.path().to_path_buf());
    assert_eq!(terms.refusal(), Some("terms_record_unavailable"));
    assert_eq!(terms.status().state, "record_unavailable");
    assert_eq!(
        terms.accept(TERMS_VERSION),
        Err(AcceptRefusal::RecordUnavailable)
    );
    assert_eq!(terms.refusal(), Some("terms_record_unavailable"));

    // A data folder that is a file: nothing can be written.
    let blocker = tempfile::NamedTempFile::new().unwrap();
    let terms = DesktopTerms::new(blocker.path().to_path_buf());
    assert_eq!(
        terms.accept(TERMS_VERSION),
        Err(AcceptRefusal::RecordUnavailable)
    );
    assert_eq!(terms.refusal(), Some("terms_record_unavailable"));
}

#[test]
fn a_failed_write_does_not_close_a_gate_that_is_already_open() {
    let parent = tempfile::tempdir().unwrap();
    let folder = parent.path().join("data");
    let terms = DesktopTerms::new(folder.clone());
    terms.accept(TERMS_VERSION).unwrap();
    // The folder is replaced by a file, so a second acceptance cannot be written or read.
    std::fs::remove_dir_all(&folder).unwrap();
    std::fs::write(&folder, "not a folder").unwrap();
    assert_eq!(
        terms.accept(TERMS_VERSION),
        Err(AcceptRefusal::RecordUnavailable)
    );
    assert_eq!(terms.refusal(), None, "an open gate stays open");
}

#[test]
fn a_second_instance_is_seen_while_closed_and_an_open_gate_stays_open() {
    let directory = tempfile::tempdir().unwrap();
    let first = DesktopTerms::new(directory.path().to_path_buf());
    let second = DesktopTerms::new(directory.path().to_path_buf());
    assert!(!first.status().accepted);
    second.accept(TERMS_VERSION).unwrap();
    assert!(
        first.status().accepted,
        "the status re-reads the record while the gate is closed"
    );
    assert_eq!(first.refusal(), None);
    std::fs::remove_file(directory.path().join(RECORD)).unwrap();
    assert!(
        first.status().accepted,
        "an open gate does not close within the process"
    );
}

#[test]
fn the_status_names_the_terms_version_of_the_build() {
    let directory = tempfile::tempdir().unwrap();
    let status = DesktopTerms::new(directory.path().to_path_buf()).status();
    assert_eq!(status.terms_version, TERMS_VERSION);
    assert_eq!(status.state, "not_accepted");
}

#[test]
fn only_the_mcp_server_builds_a_runtime_without_the_gate() {
    // The desktop app obtains its runtime from `TallyRuntime::desktop` in `.setup`. The runtime
    // without the gate is for the MCP server, whose own gate is `agent/terms`; a new caller of
    // either must be added here on purpose, so a desktop path cannot skip the gate unseen.
    fn walk(directory: &Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push((
                    path.file_name().unwrap().to_string_lossy().to_string(),
                    std::fs::read_to_string(&path).unwrap(),
                ));
            }
        }
    }
    let mut files = Vec::new();
    walk(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut files,
    );
    let callers = |needle: &str| {
        let mut names: Vec<String> = files
            .iter()
            .filter(|(name, text)| !name.ends_with("_tests.rs") && text.contains(needle))
            .map(|(name, _)| name.clone())
            .collect();
        names.sort();
        names
    };
    assert_eq!(callers("ungated()"), ["agent.rs", "runtime.rs"]);
    assert_eq!(callers("Server::new("), ["agent_lab.rs"]);
    assert_eq!(callers("TallyRuntime::desktop("), ["lib.rs"]);
}
