//! Admission tests for locally generated batches, not authored Tally responses.
#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::*;
use bridge_tally_transport::TallyEndpointConfig;

fn batch() -> (ImportLedgerLine, TallyEndpointConfig) {
    let endpoint = TallyEndpointConfig {
        host: "127.0.0.1".into(),
        port: 9001,
    };
    let mut line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000000001", "identity_scheme":"batch_v1",
        "company_guid":"00000000-0000-4000-8000-000000000002",
        "endpoint_origin":super::super::super::canonical_loopback_origin(&endpoint).unwrap(),
        "company":{"name":"Synthetic Accounts","guid":"00000000-0000-4000-8000-000000000002","company_number":"100001","books_from":"20260401"},
        "txn_ids":["journal-test"],"date_from":"20260901","date_to":"20260901",
        "sha256":"", "built_at":"2026-09-07T00:00:00Z", "status":"built",
        "pre_import_mark":{"kind":"company_high_water","value":1,"master_value":1},
        "vouchers":[{"bridge_txn_id":"journal-test","date":"20260901","voucher_type":"Journal",
            "narration":"Synthetic test only","reference":"REF-1","entries":[
                {"ledger":"Expense","amount":"12.50","side":"Dr"},
                {"ledger":"Cash","amount":"12.50","side":"Cr"}]}]
    })).unwrap();
    line.sha256 = sha256_hex(
        render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id).as_bytes(),
    );
    (line, endpoint)
}

/// The product is named "ComplyEaze Bridge" in every line a person reads in a
/// dialog: each "Bridge" in `text` follows "ComplyEaze ".
fn every_bridge_is_the_brand(text: &str) {
    assert!(
        text.contains("ComplyEaze Bridge"),
        "the product is not named at all: {text}"
    );
    for (at, _) in text.match_indices("Bridge") {
        assert!(
            text[..at].ends_with("ComplyEaze "),
            "a bare Bridge at {at}: {text}"
        );
    }
}

#[test]
fn native_preview_contains_all_accounting_inputs_and_pinned_destination() {
    let (line, endpoint) = batch();
    let (_, preview) = admit_saved_journal(&line, &endpoint).unwrap();
    for field in [
        "Synthetic Accounts",
        "9001",
        "20260901",
        "Expense",
        "Cash",
        "12.50",
        "Dr",
        "Cr",
        "REF-1",
        "Synthetic test only",
        "Pause other edits/imports; keep this company and Tally mode as is until ComplyEaze Bridge finishes.",
        "Ledgers checked by identity against the build; narrations sent as prepared, nothing added.",
        &line.batch_id,
    ] {
        assert!(preview.contains(field), "missing {field}");
    }
    every_bridge_is_the_brand(&preview);
}

#[test]
fn contended_post_returns_without_waiting_or_recording_a_dispatch() {
    let directory = tempfile::tempdir().unwrap();
    let (line, mut endpoint) = batch();
    // A regressed blocking lock is released below to let the worker exit.
    // Keep its endpoint different from the saved batch so that fallback path
    // fails local admission before any network access or native approval.
    endpoint.port = 9;
    let server = Server::new(crate::agent::Settings {
        endpoint,
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    });
    server.append_import_ledger(&line).unwrap();
    let journal = directory.path().join("agent-import-ledger.jsonl");
    let before = fs::read(&journal).unwrap();
    let admission = server.lock_import_admission().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let result = runtime.block_on(server.post_import(&json!({
            "company_guid":line.company_guid,"batch_id":line.batch_id
        })));
        tx.send(result.err().map(|failure| failure.code)).unwrap();
    });
    let result = rx.recv_timeout(std::time::Duration::from_secs(2));
    drop(admission);
    worker.join().unwrap();
    assert_eq!(result.unwrap(), Some("import_admission_busy".into()));
    assert_eq!(fs::read(journal).unwrap(), before);
}

#[test]
fn old_or_changed_batches_cannot_be_posted() {
    let (mut line, mut endpoint) = batch();
    endpoint.port = 9002;
    assert_eq!(
        admit_saved_journal_integrity(&line, &endpoint).unwrap_err(),
        "import_post_endpoint_mismatch"
    );
    endpoint.port = 9001;
    line.endpoint_origin = None;
    assert_eq!(
        admit_saved_journal_integrity(&line, &endpoint).unwrap_err(),
        "import_post_endpoint_mismatch"
    );
    let (mut line, endpoint) = batch();
    line.vouchers[0].narration = Some("Changed after build".into());
    assert_eq!(
        admit_saved_journal_integrity(&line, &endpoint).unwrap_err(),
        "import_batch_changed"
    );
}

#[test]
fn saved_integrity_allows_dispatched_recovery_without_reopening_native_preview_checks() {
    let (mut line, endpoint) = batch();
    line.vouchers[0].narration = Some("safe\u{200b}hidden".into());
    refresh_batch_sha256(&mut line);
    assert!(admit_saved_journal_integrity(&line, &endpoint).is_ok());
    assert_eq!(
        admit_saved_journal(&line, &endpoint).unwrap_err(),
        "import_review_format_text"
    );
}

#[test]
fn multiple_or_unqualified_vouchers_require_manual_workflow() {
    let (mut line, endpoint) = batch();
    line.vouchers.push(line.vouchers[0].clone());
    assert_eq!(
        admit_saved_journal(&line, &endpoint).unwrap_err(),
        "import_post_requires_one_journal"
    );
    line.vouchers.pop();
    line.vouchers[0].voucher_type = VoucherType::Payment;
    assert_eq!(
        admit_saved_journal(&line, &endpoint).unwrap_err(),
        "import_post_requires_one_journal"
    );
}

fn refresh_batch_sha256(line: &mut ImportLedgerLine) {
    line.sha256 = sha256_hex(
        render_import_xml(
            &line.company.as_ref().unwrap().name,
            &line.vouchers,
            &line.batch_id,
        )
        .as_bytes(),
    );
}

#[test]
fn native_preview_refuses_line_and_paragraph_separators_in_every_operator_text_field() {
    for separator in ['\u{2028}', '\u{2029}'] {
        for field in ["narration", "reference", "ledger"] {
            let (mut line, endpoint) = batch();
            let value = format!("safe{separator}hidden");
            match field {
                "narration" => line.vouchers[0].narration = Some(value),
                "reference" => line.vouchers[0].reference = Some(value),
                "ledger" => line.vouchers[0].entries[0].ledger = value,
                _ => unreachable!(),
            }
            refresh_batch_sha256(&mut line);
            assert_eq!(
                admit_saved_journal(&line, &endpoint).unwrap_err(),
                "import_review_layout_text",
                "{field} {separator:?}"
            );
        }
    }
}

#[test]
fn native_preview_refuses_unreviewable_formatting_in_every_operator_text_field() {
    for mark in [
        '\u{061c}',
        '\u{200e}',
        '\u{200f}',
        '\u{202a}',
        '\u{202b}',
        '\u{202c}',
        '\u{202d}',
        '\u{202e}',
        '\u{2066}',
        '\u{2067}',
        '\u{2068}',
        '\u{2069}',
        '\u{200b}',
        '\u{200c}',
        '\u{200d}',
        '\u{2060}',
        '\u{206a}',
        '\u{206b}',
        '\u{206c}',
        '\u{206d}',
        '\u{206e}',
        '\u{206f}',
        '\u{feff}',
        '\u{00ad}',
        '\u{034f}',
        '\u{115f}',
        '\u{180e}',
        '\u{fe0f}',
        '\u{e0100}',
        '\u{e007f}',
        '\u{0600}',
        '\u{fff0}',
    ] {
        for field in ["company", "number", "narration", "reference", "ledger"] {
            let (mut line, endpoint) = batch();
            let value = format!("safe{mark}hidden");
            match field {
                "company" => line.company.as_mut().unwrap().name = value,
                "number" => line.vouchers[0].voucher_number = Some(value),
                "narration" => line.vouchers[0].narration = Some(value),
                "reference" => line.vouchers[0].reference = Some(value),
                "ledger" => line.vouchers[0].entries[0].ledger = value,
                _ => unreachable!(),
            }
            refresh_batch_sha256(&mut line);
            assert_eq!(
                admit_saved_journal(&line, &endpoint).unwrap_err(),
                "import_review_format_text",
                "{field} {mark:?}"
            );
        }
    }
}

#[test]
fn native_preview_preserves_visible_multilingual_text() {
    let (mut line, endpoint) = batch();
    line.company.as_mut().unwrap().name = "मराठी खाते".into();
    line.vouchers[0].entries[0].ledger = "किराया".into();
    line.vouchers[0].narration = Some("Cafe\u{0301} – مصروف ₹12.50".into());
    refresh_batch_sha256(&mut line);
    let (_, preview) = admit_saved_journal(&line, &endpoint).unwrap();
    for text in ["मराठी खाते", "किराया", "Cafe\u{0301} – مصروف ₹12.50"]
    {
        assert!(preview.contains(text));
    }
}

pub(super) fn dispatch_response(
    application_status: &str,
    created: u64,
    altered: u64,
) -> ledger::DispatchResponse {
    ledger::DispatchResponse {
        request_sha256: "a".repeat(64),
        response_sha256: "b".repeat(64),
        bytes: 1,
        outcome: Some(
            serde_json::from_value(json!({
                "application_status":application_status,
                "counters": {
                    "created":created,
                    "altered":altered,
                    "deleted":0,
                    "ignored":0,
                    "errors":0,
                    "cancelled":0,
                    "exceptions":0,
                    "line_error_count":0,
                    "counter_presence": {
                        "created":true, "altered":true, "deleted":true,
                        "ignored":true, "errors":true, "cancelled":true, "exceptions":true
                    }
                },
                "exceptions_were_reported":true
            }))
            .unwrap(),
        ),
    }
}

#[test]
fn exact_readback_requires_a_clean_persisted_response_to_reconcile() {
    let clean = dispatch_response("success", 1, 0);
    let altered = dispatch_response("success", 0, 1);
    for (response, response_state, expected_state) in [
        (
            Some(&clean),
            "response_clean",
            "previous_attempt_reconciled",
        ),
        (
            Some(&altered),
            "response_not_clean",
            "reconciliation_required",
        ),
        (None, "response_missing", "reconciliation_required"),
    ] {
        let mut payload = json!({
            "result": {"counts": {"posted_verified": 1}, "duplicates": []}
        });
        finalize_previous_attempt_reconciliation(&mut payload, response, None, 1);
        assert_eq!(payload["result"]["dispatch"]["state"], expected_state);
        assert_eq!(
            payload["result"]["dispatch"]["response_state"],
            response_state
        );
        if expected_state == "previous_attempt_reconciled" {
            assert!(payload["result"].get("error").is_none());
        } else {
            assert_eq!(
                payload["result"]["error"]["code"],
                "import_reconciliation_required"
            );
        }
        // verify_import records this status before the proof is rendered.
        let status =
            final_verification_status(Some(&payload["result"]["dispatch"]), &payload["result"], 1);
        payload["result"]["verification_status"] = json!(status.as_str());
        let markdown = render_proof_markdown(&payload["result"]);
        assert!(markdown.contains(&format!("Dispatch verdict: `{expected_state}`")));
        assert!(markdown.contains(&format!("Response state: `{response_state}`")));
        assert!(markdown.contains("Readback counts: matching 1"));
        assert!(!markdown.contains("Proof-of-Post"));
        assert_eq!(
            markdown.contains("this report does not confirm posting"),
            expected_state == "reconciliation_required"
        );
        assert_eq!(
            markdown.contains("Error: `import_reconciliation_required`"),
            expected_state == "reconciliation_required"
        );
    }
}

#[test]
fn recovery_failure_retains_the_saved_dispatch_response() {
    let response = dispatch_response("success", 1, 0);
    let payload = reconciliation_failure_payload(
        "bridge-test",
        Some(true),
        Some(&response),
        "verification_transport_failed",
    );
    let result = &payload["result"];
    assert_eq!(result["attempt_recorded"], true);
    assert_eq!(
        result["dispatch_response"]["request_sha256"],
        response.request_sha256
    );
    assert_eq!(
        result["dispatch_response"]["response_sha256"],
        response.response_sha256
    );
    assert_eq!(
        result["dispatch_response"]["outcome"]["application_status"],
        "success"
    );
    assert_eq!(
        result["dispatch_response"]["outcome"]["counters"]["created"],
        1
    );
    assert_eq!(result["error"]["code"], "verification_transport_failed");
}

#[test]
fn a_busy_readback_after_a_recorded_send_names_verify_import_never_a_rebuild() {
    let response = dispatch_response("success", 1, 0);
    let after = reconciliation_failure_payload(
        "bridge-test",
        Some(true),
        Some(&response),
        "tally_endpoint_busy",
    );
    let error = &after["result"]["error"];
    assert_eq!(error["code"], "tally_endpoint_busy");
    assert_eq!(
        error["retry_after_s"],
        bridge_tally_transport::WIRE_BUSY_RETRY_AFTER.as_secs()
    );
    assert_eq!(
        error["next_step"],
        "The post was already sent and only its readback was held back. Call verify_import with this original batch after retry_after_s seconds. Never rebuild the batch and never call post_import again."
    );
    // Held back before any attempt was recorded: it says when to try again,
    // and that the person approves again, never a verify_import step (#869).
    let before =
        reconciliation_failure_payload("bridge-test", Some(false), None, "tally_endpoint_busy");
    assert_eq!(
        before["result"]["error"]["next_step"],
        "Nothing was posted: Tally's port was busy. Call post_import with this same batch again after retry_after_s seconds, once Tally is free. Any approval already given has lapsed, so the person is asked to approve it again. Do not rebuild the batch."
    );
    assert_eq!(
        before["result"]["error"]["retry_after_s"],
        bridge_tally_transport::WIRE_BUSY_RETRY_AFTER.as_secs()
    );
    // An attempt that could not be observed: the verify_import step, worded
    // without claiming a send.
    let unknown = reconciliation_failure_payload("bridge-test", None, None, "tally_endpoint_busy");
    assert_eq!(
        unknown["result"]["error"]["retry_after_s"],
        bridge_tally_transport::WIRE_BUSY_RETRY_AFTER.as_secs()
    );
    let step = unknown["result"]["error"]["next_step"].as_str().unwrap();
    assert!(step.contains("verify_import") && step.contains("could not be observed"));
    assert!(!step.contains("already sent"));
    // No other code gains the fields.
    let other = reconciliation_failure_payload(
        "bridge-test",
        Some(true),
        Some(&response),
        "verification_transport_failed",
    );
    assert!(other["result"]["error"].get("next_step").is_none());
}

/// #876: a refused row says not to rebuild it, even when the blocking batch
/// could not be named.
#[test]
fn a_row_refusal_without_a_named_blocker_still_says_not_to_rebuild() {
    let mut payload = reconciliation_failure_payload(
        "bridge-test",
        Some(false),
        None,
        "import_txn_already_posted",
    );
    name_blocking_batch(&mut payload, None);
    let error = &payload["result"]["error"];
    assert!(error["message"]
        .as_str()
        .unwrap()
        .contains("Nothing was sent"));
    let step = error["next_step"].as_str().unwrap();
    assert!(step.contains("Never rebuild a row"), "{step}");
    assert!(step.contains("recent batches"), "{step}");
    assert!(step.contains("whatever ledger it names"), "{step}");
    assert!(step.contains("Do not decide which yourself"), "{step}");
    assert!(step.contains("and say which"), "{step}");
    assert!(step.contains("Never rename a statement row"), "{step}");
    assert!(error.get("blocking_batch_id").is_none());
}

/// A queued read the transport did not start because the call was withdrawn
/// (#778) names the withdrawal, `request_cancelled`, as a call withdrawn
/// before its operation started does, never the operation's read failure.
#[test]
fn a_send_withdrawn_before_the_intent_names_the_withdrawal() {
    let withdrawn = || {
        anyhow::Error::from(crate::tally::approved_import::PreIntentQueueRefusal {
            source: anyhow::Error::new(bridge_tally_transport::TallyTransportError::SendWithdrawn),
        })
    };
    assert_eq!(
        ToolFailure::from_runtime("post_queue_read_failed", withdrawn()).code,
        "request_cancelled"
    );
}

/// A wire refusal replaces a generic failure code with the refusal's own, but
/// never the code that tells the caller not to rebuild an unknown post.
#[test]
fn a_wire_refusal_never_replaces_the_unknown_post_outcome_code() {
    let refused = || {
        anyhow::Error::new(bridge_tally_transport::TallyTransportError::WireRefused {
            refusal: bridge_tally_transport::WireRefusal::Busy,
        })
    };
    assert_eq!(
        ToolFailure::from_runtime("import_dispatch_outcome_unknown", refused()).code,
        "import_dispatch_outcome_unknown"
    );
    assert_eq!(
        ToolFailure::from_runtime("status_probe_unavailable", refused()).code,
        "tally_endpoint_busy"
    );
}

/// A cause is added only when it says something the code does not.
#[test]
fn a_failure_cause_never_repeats_its_own_code() {
    assert_eq!(
        refusal_cause("tally_endpoint_busy", None, Some("tally_endpoint_busy")),
        None
    );
    assert_eq!(
        refusal_cause(
            "post_queue_read_failed",
            None,
            Some("tally_connection_failed")
        ),
        Some("tally_connection_failed")
    );
    assert_eq!(
        refusal_cause("post_queue_read_failed", Some("group"), Some("other")),
        Some("group")
    );
}

/// The marks readback after a sent post draws on a wait budget of its own. A
/// call whose admission reads have spent the shared budget would otherwise be
/// refused as busy at once, and the refusal reads as an unconfirmed step: a
/// lasting doubt on a batch that posted cleanly (#697). The lock is never free
/// here, so the wait itself is what is measured; nothing is sent.
#[tokio::test]
async fn the_marks_readback_after_a_post_waits_on_a_budget_of_its_own() {
    use crate::endpoint_wire::{wire_refusal, FileWireGate};
    use bridge_tally_transport::{TallyWireGate, WireRefusal, WireRetryPolicy};
    use std::time::{Duration, Instant};
    let directory = tempfile::tempdir().unwrap();
    let mut server = records_server(directory.path());
    let endpoint = server.settings.endpoint.clone();
    let budget = Duration::from_millis(600);
    let wire = crate::tally::TallyRuntime::default()
        .wire_gate_config()
        .clone()
        .with_retry(WireRetryPolicy::new(Duration::from_millis(50), budget).unwrap());
    let _other = FileWireGate::new(wire.root().clone(), endpoint.clone())
        .try_acquire()
        .unwrap();
    server.runtime = crate::tally::TallyRuntime::default().with_wire_gate_config(wire);
    let request = || {
        crate::tally::agent_read_request::AgentReadRequest::parse(
            super::super::super::read_profiles::render_agent_company_high_water("Test Co"),
        )
        .unwrap()
    };
    let busy = |result: anyhow::Result<String>| {
        wire_refusal(&result.expect_err("the wire lock is held")) == Some(WireRefusal::Busy)
    };
    crate::tally::runtime::with_operation_wire_budget(async {
        // The call's admission reads spend its whole budget.
        let spent = server
            .runtime
            .read_company_marks_once(server.tally_config(), request())
            .await;
        assert!(busy(spent));
        let started = Instant::now();
        // A call with no time left, so this measures one budget.
        let after = server
            .read_marks_after_post(
                request(),
                Instant::now()
                    .checked_sub(Duration::from_secs(44))
                    .unwrap_or_else(Instant::now),
            )
            .await;
        assert!(busy(after));
        assert!(
            started.elapsed() >= budget,
            "the readback must wait on a fresh budget, not inherit the spent one"
        );
    })
    .await;
}

/// The retry of the marks readback after a sent post (#884) never stretches
/// the call past its ceiling: whatever has elapsed, what elapsed, the wait and
/// the rest of the post fit under it, and a call with little left retries not
/// at all. The slowest measured live post took about 21 s from its answer.
#[test]
fn the_after_read_retry_budget_keeps_the_call_under_its_ceiling() {
    use std::time::Duration;
    let policy = bridge_tally_transport::WireRetryPolicy::DEFAULT.total();
    assert_eq!(
        after_read_retry_budget(Duration::from_secs(18), policy),
        Some(policy)
    );
    assert_eq!(
        after_read_retry_budget(Duration::from_secs(25), policy),
        Some(Duration::from_secs(7))
    );
    assert_eq!(
        after_read_retry_budget(Duration::from_secs(31), policy),
        Some(Duration::from_secs(1))
    );
    for elapsed in [Duration::from_millis(31_001), Duration::from_secs(90)] {
        assert_eq!(after_read_retry_budget(elapsed, policy), None);
    }
    for millis in (0..=60_000).step_by(50) {
        let elapsed = Duration::from_millis(millis);
        if let Some(budget) = after_read_retry_budget(elapsed, policy) {
            assert!(budget <= policy && budget >= AFTER_READ_MIN_RETRY_WAIT);
            assert!(elapsed + budget + AFTER_READ_REST_OF_POST <= approval::CALL_CEILING);
        }
    }
    assert!(approval::CALL_CEILING + Duration::from_secs(10) <= Duration::from_secs(60));
}

/// A redeem-only pass waits on the call's shared budget only as far as the
/// redeem measured live still fits under the ceiling (#893): the budget is
/// never more than the shared one, and a pass that starts past the point where
/// the redeem fits waits for nothing.
#[test]
fn the_redeem_pass_wire_budget_keeps_the_redeem_under_the_ceiling() {
    use std::time::Duration;
    let shared = bridge_tally_transport::WireRetryPolicy::DEFAULT.total();
    assert_eq!(redeem_pass_wire_budget(Duration::ZERO, shared), shared);
    assert_eq!(
        redeem_pass_wire_budget(Duration::from_secs(20), shared),
        Duration::from_millis(6_910)
    );
    for elapsed in [Duration::from_millis(26_910), Duration::from_secs(40)] {
        assert_eq!(redeem_pass_wire_budget(elapsed, shared), Duration::ZERO);
    }
    assert_eq!(
        redeem_pass_wire_budget(Duration::ZERO, Duration::from_millis(300)),
        Duration::from_millis(300)
    );
    for millis in (0..=60_000).step_by(50) {
        let elapsed = Duration::from_millis(millis);
        let budget = redeem_pass_wire_budget(elapsed, shared);
        assert!(budget <= shared);
        if budget > Duration::ZERO {
            assert!(elapsed + budget + approval::MEASURED_REDEEM <= approval::CALL_CEILING);
        }
    }
}

/// A busy wire lock on the marks readback after a sent post is tried once
/// more, on a fresh wait budget (#884): a lock freed after the first budget
/// but within the second is not a lasting doubt. A lock that stays held is
/// still refused, once, after both waits; a call with no time left retries
/// nothing. The port has nothing listening, so a read that gets through the
/// wire fails as a connection failure, which is how it shows here.
#[tokio::test]
async fn a_busy_marks_readback_after_a_post_is_retried_once_within_the_call() {
    use crate::endpoint_wire::{wire_refusal, FileWireGate};
    use bridge_tally_transport::{TallyWireGate, WireRefusal, WireRetryPolicy};
    use std::time::{Duration, Instant};
    let directory = tempfile::tempdir().unwrap();
    let mut server = records_server(directory.path());
    let endpoint = server.settings.endpoint.clone();
    let budget = Duration::from_millis(600);
    let wire = crate::tally::TallyRuntime::default()
        .wire_gate_config()
        .clone()
        .with_retry(WireRetryPolicy::new(Duration::from_millis(50), budget).unwrap());
    server.runtime = crate::tally::TallyRuntime::default().with_wire_gate_config(wire.clone());
    let request = || {
        crate::tally::agent_read_request::AgentReadRequest::parse(
            super::super::super::read_profiles::render_agent_company_high_water("Test Co"),
        )
        .unwrap()
    };
    let hold = || {
        FileWireGate::new(wire.root().clone(), endpoint.clone())
            .try_acquire()
            .unwrap()
    };
    let busy = |result: &anyhow::Result<String>| {
        wire_refusal(result.as_ref().expect_err("the read sends nothing"))
            == Some(WireRefusal::Busy)
    };

    // How many reads a call made is counted, not timed (#998): an upper bound on
    // elapsed time fails on a loaded runner although the code is right. The
    // lower bounds stay, because a wait can only add to elapsed time.
    async fn reads_of(
        server: &Server,
        request: crate::tally::agent_read_request::AgentReadRequest,
        call_started: Instant,
    ) -> (anyhow::Result<String>, Vec<Option<Duration>>) {
        let reads = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let result = MARKS_READS
            .scope(
                std::sync::Arc::clone(&reads),
                server.read_marks_after_post(request, call_started),
            )
            .await;
        let reads = reads.lock().unwrap().clone();
        (result, reads)
    }

    // Freed after the first budget, inside the second: the retry gets through.
    let held = hold();
    let release = tokio::spawn(async move {
        tokio::time::sleep(budget + Duration::from_millis(300)).await;
        drop(held);
    });
    let started = Instant::now();
    let (result, reads) = reads_of(&server, request(), Instant::now()).await;
    assert!(started.elapsed() >= budget);
    assert!(!busy(&result), "the retry must reach the wire: {result:?}");
    assert_eq!(reads, [None, Some(budget)]);
    release.await.unwrap();

    // Held throughout: refused once, after both budgets, never a third read.
    let _held = hold();
    let started = Instant::now();
    let (result, reads) = reads_of(&server, request(), Instant::now()).await;
    assert!(busy(&result));
    assert!(started.elapsed() >= budget * 2);
    assert_eq!(reads, [None, Some(budget)], "exactly one retry");

    // A call with no time left waits its first budget and retries nothing.
    let spent = Instant::now()
        .checked_sub(Duration::from_secs(44))
        .unwrap_or_else(Instant::now);
    let started = Instant::now();
    let (result, reads) = reads_of(&server, request(), spent).await;
    assert!(busy(&result));
    assert!(started.elapsed() >= budget);
    assert_eq!(reads, [None], "no retry");
}

/// A marks readback that failed after a sent post is named in the recorded
/// step doubt (#884): the doubt says the read was held back, and the verdict
/// stays doubt.
#[test]
fn a_failed_marks_readback_is_named_in_the_step_doubt() {
    let refused = anyhow::Error::new(bridge_tally_transport::TallyTransportError::WireRefused {
        refusal: bridge_tally_transport::WireRefusal::Busy,
    })
    .context("marks readback");
    assert_eq!(after_read_cause(&refused), "tally_endpoint_busy");
    assert_eq!(
        after_read_cause(&anyhow::anyhow!("something else")),
        "marks_readback_failed"
    );
    let directory = tempfile::tempdir().unwrap();
    let server = records_server(directory.path());
    server.record_post_checks_pending("batch-a", true).unwrap();
    server.record_batch_step_verdict_caused("batch-a", &Value::Null, Some("tally_endpoint_busy"));
    let imports = server.imports_dir().unwrap();
    let doubt: Value = serde_json::from_slice(
        &fs::read(super::super::batch_step_doubt_path(&imports, "batch-a")).unwrap(),
    )
    .unwrap();
    assert_eq!(doubt["state"], "unmatched");
    assert_eq!(doubt["cause"], "tally_endpoint_busy");
    let recorded = server.record_masters_verdict_for("batch-a", json!({"state":"unchanged"}), true);
    assert_eq!(
        post_doubt(Some(&recorded), 2).map(|(code, _)| code),
        Some("batch_step_unconfirmed")
    );
    // A verdict with no cause carries none.
    let other = tempfile::tempdir().unwrap();
    let server = records_server(other.path());
    server.record_post_checks_pending("batch-b", true).unwrap();
    server.record_batch_step_verdict_caused("batch-b", &Value::Null, None);
    let doubt: Value = serde_json::from_slice(
        &fs::read(super::super::batch_step_doubt_path(
            &server.imports_dir().unwrap(),
            "batch-b",
        ))
        .unwrap(),
    )
    .unwrap();
    assert!(doubt.get("cause").is_none());
}

#[test]
fn endpoint_lease_contention_keeps_negative_post_and_cancellation_results_uncertain() {
    let directory = tempfile::tempdir().unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let (mut line, mut endpoint) = batch();
    endpoint.port = listener.local_addr().unwrap().port();
    line.endpoint_origin = Some(super::super::super::canonical_loopback_origin(&endpoint).unwrap());
    let server = Server::new(crate::agent::Settings {
        endpoint,
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    });
    server.append_import_ledger_while_admitted(&line).unwrap();
    let args = json!({"batch_id":&line.batch_id,"company_guid":&line.company_guid});
    let snapshot = server
        .latest_import_snapshot(&line.batch_id)
        .unwrap()
        .unwrap();
    let snapshot_lease = dispatch_lease::acquire_snapshot(&server.settings.endpoint).unwrap();

    assert_eq!(server.recorded_import_attempt(&args).unwrap(), Some(false));
    assert_eq!(
        server.post_failure_attempt_observation(&line.batch_id, &line.company_guid, None, None),
        None
    );
    let attempted = server.post_failure_attempt_observation(
        &line.batch_id,
        &line.company_guid,
        Some(&snapshot),
        None,
    );
    assert_eq!(attempted, Some(false));
    for code in ["import_admission_busy", "import_approval_declined"] {
        let outcome = post_failure_outcome(
            &line.batch_id,
            &line.company_guid,
            ToolFailure::from(code.to_string()),
            crate::agent::evidence_from_runtime_read(
                crate::tally::runtime::RuntimeReadEvidence::empty(),
            ),
            Some(&snapshot),
            None,
            attempted,
        );
        assert_eq!(outcome.payload["result"]["attempt_recorded"], false);
    }
    let cancellation = server.cancelled_import_for_response(&args).unwrap();
    assert_eq!(cancellation.payload["result"]["attempt_recorded"], false);
    drop(snapshot_lease);

    let writer_lease = dispatch_lease::acquire(&server.settings.endpoint).unwrap();
    assert!(server
        .post_failure_attempt_observation(&line.batch_id, &line.company_guid, Some(&snapshot), None)
        .is_none());
    let cancellation = server.cancelled_import_for_response(&args).unwrap();
    assert!(cancellation.payload["result"]["attempt_recorded"].is_null());
    drop(writer_lease);
    assert_eq!(
        server.post_failure_attempt_observation(
            &line.batch_id,
            &line.company_guid,
            Some(&snapshot),
            None,
        ),
        Some(false)
    );

    let mut wrong_endpoint = snapshot;
    wrong_endpoint.batch.endpoint_origin = Some("http://127.0.0.1:9".into());
    assert_eq!(
        server.post_failure_attempt_observation(
            &line.batch_id,
            &line.company_guid,
            Some(&wrong_endpoint),
            None,
        ),
        None
    );

    let lease = dispatch_lease::acquire(&server.settings.endpoint).unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&line))
        .unwrap();
    let intent_snapshot = server
        .latest_import_snapshot(&line.batch_id)
        .unwrap()
        .unwrap();
    assert_eq!(
        server.post_failure_attempt_observation(
            &line.batch_id,
            &line.company_guid,
            Some(&intent_snapshot),
            None,
        ),
        Some(true)
    );
    assert_eq!(
        server.post_failure_attempt_observation(
            &line.batch_id,
            &line.company_guid,
            None,
            Some(&dispatch_response("success", 1, 0)),
        ),
        Some(true)
    );
    drop(lease);
}

#[tokio::test]
async fn received_response_survives_an_injected_response_journal_failure() {
    let directory = tempfile::tempdir().unwrap();
    let (line, endpoint) = batch();
    let server = Server::new(crate::agent::Settings {
        endpoint,
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    });
    server.append_import_ledger(&line).unwrap();
    let admission = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch(&line))
        .unwrap();
    drop(admission);

    let journal = directory.path().join("agent-import-ledger.jsonl");
    std::fs::rename(&journal, directory.path().join("preserved-ledger.jsonl")).unwrap();
    std::fs::create_dir(&journal).unwrap();
    let response = dispatch_response("success", 1, 0);
    let admission = server.lock_import_admission().unwrap();
    assert_eq!(
        server.append_import_record_while_admitted(&ledger::StatusRecord::response(
            &line,
            response.clone(),
        )),
        Err("import_ledger_unavailable".into())
    );
    drop(admission);

    let outcome = post_failure_outcome(
        &line.batch_id,
        &line.company_guid,
        ToolFailure::from("import_ledger_unavailable".to_string()),
        crate::agent::evidence_from_runtime_read(
            crate::tally::runtime::RuntimeReadEvidence::empty(),
        ),
        None,
        Some(&response),
        Some(true),
    );
    let formatted = server.finish_tool_response(
        "post_import",
        &json!({"batch_id":line.batch_id,"company_guid":line.company_guid}),
        chrono::Utc::now(),
        Ok(outcome),
    );
    let mut output = Vec::new();
    std::fs::create_dir(server.settings.data_dir.join("agent-egress.jsonl")).unwrap();
    crate::agent::agent_protocol::finish_response(
        &server,
        &mut output,
        json!(7),
        Ok(formatted.value),
        Some(formatted.egress),
        formatted.recovery_batch_id,
        true,
    )
    .await
    .unwrap();
    let framed: Value = serde_json::from_slice(&output).unwrap();
    assert!(output.len() <= server.settings.max_bytes);
    assert_eq!(framed["error"]["message"], "egress_record_write_failed");
    let payload = &framed["error"]["data"];
    assert_eq!(payload["batch_id"], line.batch_id);
    assert_eq!(payload["attempt_recorded"], true);
    assert_eq!(payload["dispatch"]["state"], "reconciliation_required");
    assert_eq!(payload["dispatch"]["resent"], false);
    assert_eq!(
        payload["dispatch_response"]["request_sha256"],
        response.request_sha256
    );
    assert_eq!(
        payload["dispatch_response"]["response_sha256"],
        response.response_sha256
    );
    assert_eq!(
        payload["dispatch_response"]["outcome"]["application_status"],
        "success"
    );
    assert_eq!(
        payload["dispatch_response"]["outcome"]["counters"]["created"],
        1
    );
}

#[test]
fn current_dispatch_finalizer_marks_only_a_clean_response_posted() {
    let response = dispatch_response("success", 1, 0);
    let mut payload = json!({
        "result": {"counts": {"posted_verified": 1}, "duplicates": []}
    });
    finalize_current_dispatch(&mut payload, Some(&response), None, 1);
    assert_eq!(payload["result"]["dispatch"]["state"], "posted_verified");
    assert_eq!(
        payload["result"]["dispatch"]["response_state"],
        "response_clean"
    );
    assert_eq!(payload["result"]["dispatch"]["counters"]["created"], 1);
    assert!(payload["result"].get("error").is_none());
}

/// Tally's LINEERROR text rides in the response for a person to read and
/// changes no verdict: each finalizer gives the same state, response state
/// and error with the text as without it, for the captured partial commit
/// and for a hostile response past every bound. A clean response cannot carry
/// text at all, because a record never keeps more texts than its count.
#[test]
fn line_error_text_changes_no_dispatch_verdict_and_stays_small() {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/import_line_error_partial_commit_live.utf16le.xml"
    );
    let xml = bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .expect("captured BOM-less UTF-16LE import response")
    .text;
    let with_outcome = |xml: &str| ledger::DispatchResponse {
        outcome: Some(bridge_tally_protocol::parse_import_outcome(xml).unwrap()),
        ..dispatch_response("success", 0, 0)
    };
    let partial = with_outcome(&xml);
    // A hostile response: more entries than are kept, each longer than is
    // kept, all of it control characters. Each becomes a three-byte U+FFFD,
    // never a six-byte JSON escape, and the total stays within 4,096 bytes.
    // That the text can never cause a refusal is pinned where the cap is
    // enforced (agent_response_tests.rs).
    let hostile_line_errors =
        format!("<LINEERROR>{}&quot;</LINEERROR>", "&#1;".repeat(600)).repeat(100);
    let hostile = with_outcome(&format!(
        "<RESPONSE>{hostile_line_errors}<CREATED>1</CREATED><ALTERED>0</ALTERED>\
         <DELETED>0</DELETED><IGNORED>0</IGNORED><ERRORS>0</ERRORS><CANCELLED>0</CANCELLED>\
         <EXCEPTIONS>100</EXCEPTIONS></RESPONSE>"
    ));
    let without_text = |response: &ledger::DispatchResponse| {
        let mut saved = serde_json::to_value(response).unwrap();
        saved["outcome"]
            .as_object_mut()
            .unwrap()
            .remove("tally_line_errors");
        serde_json::from_value::<ledger::DispatchResponse>(saved).unwrap()
    };
    let current: fn(&mut Value, Option<&ledger::DispatchResponse>) =
        |payload, response| finalize_current_dispatch(payload, response, None, 1);
    let previous: fn(&mut Value, Option<&ledger::DispatchResponse>) =
        |payload, response| finalize_previous_attempt_reconciliation(payload, response, None, 1);
    for response in [&partial, &hostile] {
        let kept = response.outcome.as_ref().unwrap().tally_line_errors();
        assert!(!kept.is_empty());
        let stripped = without_text(response);
        assert!(stripped
            .outcome
            .as_ref()
            .unwrap()
            .tally_line_errors()
            .is_empty());
        for finalize in [current, previous] {
            let verdict = |response: &ledger::DispatchResponse| {
                let mut payload =
                    json!({"result":{"counts":{"posted_verified":1},"duplicates":[]}});
                finalize(&mut payload, Some(response));
                payload
            };
            let shown = verdict(response);
            let bare = verdict(&stripped);
            for key in ["state", "response_state"] {
                assert_eq!(
                    shown["result"]["dispatch"][key],
                    bare["result"]["dispatch"][key]
                );
            }
            assert_eq!(shown["result"]["error"], bare["result"]["error"]);
            assert_eq!(
                shown["result"]["dispatch"]["state"],
                "reconciliation_required"
            );
            // The text is shown to the caller as kept, and the whole dispatch
            // report stays far under the default 200,000-byte response cap.
            assert_eq!(
                shown["result"]["dispatch"]["response"]["outcome"]["tally_line_errors"],
                serde_json::to_value(kept).unwrap()
            );
            let size = shown["result"]["dispatch"].to_string().len();
            assert!(size < 8 * 1024, "{size}");
        }
    }
}

#[test]
fn missing_counter_evidence_cannot_confirm_current_or_previous_dispatch() {
    // Mutate only the presence marker in saved response evidence. This tests
    // classification, not a newly claimed live response profile.
    for missing in [
        "legacy",
        "created",
        "altered",
        "deleted",
        "ignored",
        "errors",
        "cancelled",
        "exceptions",
        "exception_marker",
    ] {
        let mut saved = serde_json::to_value(dispatch_response("success", 1, 0)).unwrap();
        match missing {
            "legacy" => {
                saved["outcome"]["counters"]
                    .as_object_mut()
                    .unwrap()
                    .remove("counter_presence");
            }
            "exception_marker" => saved["outcome"]["exceptions_were_reported"] = json!(false),
            counter => saved["outcome"]["counters"]["counter_presence"][counter] = json!(false),
        }
        let response: ledger::DispatchResponse = serde_json::from_value(saved).unwrap();
        let current: fn(&mut Value, Option<&ledger::DispatchResponse>) =
            |payload, response| finalize_current_dispatch(payload, response, None, 1);
        let previous: fn(&mut Value, Option<&ledger::DispatchResponse>) = |payload, response| {
            finalize_previous_attempt_reconciliation(payload, response, None, 1)
        };
        for finalize in [current, previous] {
            let mut payload = json!({"result":{"counts":{"posted_verified":1},"duplicates":[]}});
            finalize(&mut payload, Some(&response));
            assert_eq!(
                payload["result"]["dispatch"]["state"],
                "reconciliation_required"
            );
            assert_eq!(
                payload["result"]["dispatch"]["response_state"],
                "response_not_clean"
            );
            assert_eq!(
                payload["result"]["error"]["code"],
                "import_reconciliation_required"
            );
            assert!(render_proof_markdown(&payload["result"]).contains("does not confirm posting"));
        }
    }
}

#[test]
fn post_date_refusal_retains_the_completed_profile_probe_evidence() {
    let (mut line, _) = batch();
    line.vouchers[0].date = "20260907".into();
    let payload = ImportPayload {
        company_guid: line.company_guid.clone(),
        vouchers: line.vouchers,
        amends_batch_id: None,
    };
    let profile_evidence = Evidence {
        request_sha256: "profile-request".into(),
        response_sha256: "profile-response".into(),
        bytes: 42,
        state: "complete",
        read_at: None,
        duration_ms: None,
        reason_code: None,
    };
    let profile = ImportProfileObservation {
        qualification: Ok(()),
        evidence: profile_evidence.clone(),
        observed_profile: json!({}),
        admission_key: ("tallyprime".into(), "education".into()),
    };
    let mut accumulated = Evidence {
        request_sha256: String::new(),
        response_sha256: String::new(),
        bytes: 0,
        state: "complete",
        read_at: None,
        duration_ms: None,
        reason_code: None,
    };
    let expected = combine_evidence(accumulated.clone(), profile_evidence);
    assert_eq!(
        validate_post_profile_with_evidence(&payload, &profile, &mut accumulated).unwrap_err(),
        "education_voucher_date_unsupported"
    );
    assert_eq!(accumulated.request_sha256, expected.request_sha256);
    assert_eq!(accumulated.response_sha256, expected.response_sha256);
    assert_eq!(accumulated.bytes, expected.bytes);
}

#[test]
fn absence_is_required_before_a_first_attempt() {
    for payload in [
        json!({}),
        json!({"counts":{"posted_verified":1},"vouchers":[{}]}),
        json!({"counts":{"not_found":1},"vouchers":[]}),
        json!({"result":{"counts":{"not_found":1},"vouchers":[{}]}}),
    ] {
        assert_eq!(
            require_absent_verification_result(&payload, 1).unwrap_err(),
            "import_preexisting_identity"
        );
    }
    assert!(require_absent_verification_result(
        &json!({"counts":{"not_found":1},"vouchers":[{}]}),
        1
    )
    .is_ok());
}

/// A native post sends a fresh REMOTEID and a print-ready narration with no
/// `[BRIDGE:…]` tag (owner decision, 2026-09-28); everything else is the
/// reviewed public file byte for byte. The public file keeps its tag.
#[test]
fn native_request_uses_a_private_remote_identity_and_an_untagged_narration() {
    let (line, _) = batch();
    let voucher = &line.vouchers[0];
    let public = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
    let first = render_native_vouchers_xml(
        "Synthetic Accounts",
        std::iter::once((voucher, Uuid::new_v4())),
    );
    let second = render_native_vouchers_xml(
        "Synthetic Accounts",
        std::iter::once((voucher, Uuid::new_v4())),
    );
    let tag = format!(
        " [BRIDGE:{}]",
        import_identity(&line.batch_id, &voucher.bridge_txn_id)
    );
    assert_eq!(
        public.matches(tag.as_str()).count(),
        1,
        "the public file keeps its tag"
    );
    assert!(!first.contains("[BRIDGE:") && !second.contains("[BRIDGE:"));
    let remote_id = |xml: &str| {
        let mut reader = quick_xml::Reader::from_str(xml);
        loop {
            match reader.read_event().unwrap() {
                quick_xml::events::Event::Start(tag) if tag.name().as_ref() == b"VOUCHER" => {
                    let attribute = tag
                        .attributes()
                        .map(Result::unwrap)
                        .find(|attribute| attribute.key.as_ref() == b"REMOTEID")
                        .unwrap();
                    break Uuid::parse_str(std::str::from_utf8(&attribute.value).unwrap()).unwrap();
                }
                quick_xml::events::Event::Eof => panic!("voucher missing"),
                _ => {}
            }
        }
    };
    let public_id = remote_id(&public);
    let first_id = remote_id(&first);
    let second_id = remote_id(&second);
    assert_ne!(first_id, public_id);
    assert_ne!(second_id, public_id);
    assert_ne!(first_id, second_id);
    // Only the client mutation selector and the tag change; accounting,
    // company and narration text remain byte-for-byte the reviewed public file.
    let untagged_public = public.replace(tag.as_str(), "");
    assert_eq!(
        first.replace(
            &format!("REMOTEID=\"{first_id}\""),
            &format!("REMOTEID=\"{public_id}\"")
        ),
        untagged_public
    );
    assert_eq!(
        second.replace(
            &format!("REMOTEID=\"{second_id}\""),
            &format!("REMOTEID=\"{public_id}\"")
        ),
        untagged_public
    );
    assert_eq!(sha256_hex(public.as_bytes()), line.sha256);
}

#[test]
fn native_post_refuses_supplied_numbers_without_disabling_manual_files() {
    let (mut line, endpoint) = batch();
    assert!(require_native_numbering(&line.vouchers[0]).is_ok());
    line.vouchers[0].voucher_number = Some("MANUAL-1".into());
    line.sha256 = sha256_hex(
        render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id).as_bytes(),
    );
    assert_eq!(
        admit_saved_journal(&line, &endpoint).err().as_deref(),
        Some("import_post_numbered_journal_unsupported")
    );
}

/// bridge#626 over a batch: a folded twin of a ledger named only by the
/// SECOND voucher refuses the post, before approval and again in the queue.
/// Both checks read every voucher's ledgers, not the first voucher's. The
/// twin is the same test-local rewrite of the capture as the single-voucher
/// test (an unrelated ledger renamed `Cash` plus CR LF), no evidence of Tally
/// behaviour.
/// #815 (review P3): a cash-in-hand ledger rides only on a bank voucher, so a
/// Journal recording one, which the queue would read with no group collection
/// and so never check, is refused as a wiring fault. The captured reads are
/// the folded-twin test's, which admit the same Journal recording none.
#[test]
fn a_journal_recording_a_cash_in_hand_ledger_is_refused_by_the_queue() {
    let company_guid = "61c6de69-1748-461c-ad3f-162cb949df9f";
    let decode = |bytes: &[u8]| {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    };
    let captured = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-namespaced-journal.utf16le.xml"
    ));
    let catalogue = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
    ));
    let single_currency = captured_currencies(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    // Voucher 1 names no ledger with a twin; only voucher 2 names `Cash`.
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000000626",
        "identity_scheme":"batch_v1", "company_guid":company_guid,
        "txn_ids":["TWIN-1","TWIN-2"],
        "date_from":"20260907", "date_to":"20260907", "sha256":"e39eb3c0bfe53144bdd9c0f4afcb88c3d63a2050214233ee77465d42a54245ef",
        "built_at":"2026-09-06T21:40:26.641Z", "status":"built", "cash_in_hand_ledgers":[{"bridge_txn_id":"TWIN-2","ledger":"Cash"}],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":219},
        "vouchers":[
            {"bridge_txn_id":"TWIN-1","date":"20260907","voucher_type":"Journal",
             "narration":"first","reference":null,"voucher_number":null,
             "entries":[{"ledger":"Bridge Nested Debtor WR4","amount":"3.00","side":"Dr"},
                {"ledger":"Café Naïve Traders","amount":"3.00","side":"Cr"}]},
            {"bridge_txn_id":"TWIN-2","date":"20260907","voucher_type":"Journal",
             "narration":"second","reference":null,"voucher_number":null,
             "entries":[{"ledger":"Bridge Nested Debtor WR4","amount":"5.00","side":"Dr"},
                {"ledger":"Cash","amount":"5.00","side":"Cr"}]}]
    }))
    .unwrap();
    let payload = ImportPayload {
        company_guid: company_guid.into(),
        vouchers: line.vouchers.clone(),
        amends_batch_id: None,
    };
    let requested = requested_ledger_names(&payload);
    assert_eq!(
        requested,
        ["Bridge Nested Debtor WR4", "Café Naïve Traders", "Cash"],
        "every voucher's ledgers"
    );
    let ledger_binding = bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &catalogue,
        "WR2 Unicode Lab",
        company_guid,
    )
    .unwrap()
    .bind_selected(requested.clone())
    .unwrap();
    let recheck = |catalogue: &str| {
        recheck_import_admission(
            &line,
            company_guid,
            "WR2 Unicode Lab",
            &captured,
            &captured,
            catalogue,
            None,
            &single_currency,
            &ledger_binding,
        )
    };
    let error = recheck(&catalogue).expect_err("a recorded cash ledger without a bank voucher");
    assert_eq!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(&ApprovedImportAdmissionError::AdmissionInconsistent)
    );
}

#[test]
fn a_folded_twin_named_only_by_a_later_voucher_refuses_the_batch() {
    let company_guid = "61c6de69-1748-461c-ad3f-162cb949df9f";
    let decode = |bytes: &[u8]| {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    };
    let captured = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-namespaced-journal.utf16le.xml"
    ));
    let catalogue = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
    ));
    let single_currency = captured_currencies(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    // Voucher 1 names no ledger with a twin; only voucher 2 names `Cash`.
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000000626",
        "identity_scheme":"batch_v1", "company_guid":company_guid,
        "txn_ids":["TWIN-1","TWIN-2"],
        "date_from":"20260907", "date_to":"20260907", "sha256":"e39eb3c0bfe53144bdd9c0f4afcb88c3d63a2050214233ee77465d42a54245ef",
        "built_at":"2026-09-06T21:40:26.641Z", "status":"built", "cash_in_hand_ledgers":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":219},
        "vouchers":[
            {"bridge_txn_id":"TWIN-1","date":"20260907","voucher_type":"Journal",
             "narration":"first","reference":null,"voucher_number":null,
             "entries":[{"ledger":"Bridge Nested Debtor WR4","amount":"3.00","side":"Dr"},
                {"ledger":"Café Naïve Traders","amount":"3.00","side":"Cr"}]},
            {"bridge_txn_id":"TWIN-2","date":"20260907","voucher_type":"Journal",
             "narration":"second","reference":null,"voucher_number":null,
             "entries":[{"ledger":"Bridge Nested Debtor WR4","amount":"5.00","side":"Dr"},
                {"ledger":"Cash","amount":"5.00","side":"Cr"}]}]
    }))
    .unwrap();
    let payload = ImportPayload {
        company_guid: company_guid.into(),
        vouchers: line.vouchers.clone(),
        amends_batch_id: None,
    };
    let requested = requested_ledger_names(&payload);
    assert_eq!(
        requested,
        ["Bridge Nested Debtor WR4", "Café Naïve Traders", "Cash"],
        "every voucher's ledgers"
    );
    let ledger_binding = bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &catalogue,
        "WR2 Unicode Lab",
        company_guid,
    )
    .unwrap()
    .bind_selected(requested.clone())
    .unwrap();
    let recheck = |catalogue: &str| {
        recheck_import_admission(
            &line,
            company_guid,
            "WR2 Unicode Lab",
            &captured,
            &captured,
            catalogue,
            None,
            &single_currency,
            &ledger_binding,
        )
    };
    // Control: the captured catalogue holds no twin of any named ledger.
    recheck(&catalogue).expect("no twin, so the queued batch is admitted");
    assert_eq!(
        catalogue.matches("WR2 Sales").count(),
        2,
        "name and NAME.LIST"
    );
    let twinned = catalogue.replace("WR2 Sales", "Cash&#13;&#10;");
    // Before approval, the post checks the names requested across the batch.
    let twinned_parents = bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &twinned,
        "WR2 Unicode Lab",
        company_guid,
    )
    .unwrap();
    assert!(!super::super::folded_twins(&requested, twinned_parents.parents()).is_empty());
    // In the queue, the recheck reads every voucher's ledgers again.
    let error = recheck(&twinned).expect_err("a twin of voucher 2's ledger must refuse the batch");
    assert!(matches!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::LedgerFoldedTwin)
    ));
}

/// #815: the queue refuses a record written before its cash-in-hand ledgers
/// were, by its typed variant, before reading anything it was handed: there is
/// nothing to check again, so the batch must be rebuilt.
#[test]
fn a_record_without_cash_in_hand_ledgers_is_refused_by_the_queue() {
    let company_guid = "61c6de69-1748-461c-ad3f-162cb949df9f";
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000000815",
        "identity_scheme":"batch_v1", "company_guid":company_guid,
        "txn_ids":["contra-815"],
        "date_from":"20260901", "date_to":"20260901", "sha256":"",
        "built_at":"2026-10-05T00:00:00Z", "status":"built",
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":219},
        "vouchers":[{"bridge_txn_id":"contra-815","date":"20260901","voucher_type":"Contra",
            "entries":[{"ledger":"Cash","amount":"5.00","side":"Dr"},
                {"ledger":"WR2 Sales","amount":"5.00","side":"Cr"}]}]
    }))
    .unwrap();
    assert_eq!(line.cash_in_hand_ledgers, None);
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
    );
    let catalogue = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let ledger_binding = bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &catalogue,
        "WR2 Unicode Lab",
        company_guid,
    )
    .unwrap()
    .bind_selected(vec!["Cash".to_string(), "WR2 Sales".to_string()])
    .unwrap();
    // Every read it is handed is empty: the refusal comes before any of them.
    let error = recheck_import_admission(
        &line,
        company_guid,
        "WR2 Unicode Lab",
        "",
        "",
        "",
        None,
        "",
        &ledger_binding,
    )
    .expect_err("a record without the field must be refused");
    assert_eq!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(&ApprovedImportAdmissionError::CashLedgersNotRecorded)
    );
}

#[test]
fn queued_absence_recheck_distinguishes_an_attributed_journal_from_a_new_candidate() {
    let company_guid = "61c6de69-1748-461c-ad3f-162cb949df9f";
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-namespaced-journal.utf16le.xml"
    );
    let captured = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-6c79872c-aab6-4be5-a181-18182c8148be",
        "identity_scheme":"batch_v1", "company_guid":company_guid,
        "txn_ids":["BRIDGE_MCP_LIVE_20260906_A1"],
        "date_from":"20260907", "date_to":"20260907", "sha256":"e39eb3c0bfe53144bdd9c0f4afcb88c3d63a2050214233ee77465d42a54245ef",
        "built_at":"2026-09-06T21:40:26.641Z", "status":"built", "cash_in_hand_ledgers":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":219},
        "vouchers":[{"bridge_txn_id":"BRIDGE_MCP_LIVE_20260906_A1","date":"20260907",
            "voucher_type":"Journal","narration":"Bridge MCP batch namespace qualification",
            "reference":null,"voucher_number":null,
            "entries":[{"ledger":"Bridge Nested Debtor WR4","amount":"12.61","side":"Dr"},
                {"ledger":"Cash","amount":"12.61","side":"Cr"}]}]
    }))
    .unwrap();
    let catalogue_bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
    );
    let catalogue = String::from_utf16(
        &catalogue_bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let single_currency = captured_currencies(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let ledger_binding = bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &catalogue,
        "WR2 Unicode Lab",
        company_guid,
    )
    .unwrap()
    .bind_selected(requested_ledger_names(&ImportPayload {
        company_guid: company_guid.into(),
        vouchers: line.vouchers.clone(),
        amends_batch_id: None,
    }))
    .unwrap();
    let error = recheck_import_admission(
        &line,
        company_guid,
        "WR2 Unicode Lab",
        &captured,
        &captured,
        &catalogue,
        None,
        &single_currency,
        &ledger_binding,
    )
    .expect_err("captured attributed Journal must block the queued native attempt");
    assert!(matches!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::PreexistingIdentity)
    ));

    // Change the expected local batch, not the captured Tally source. Neither
    // attribution nor accounting content now matches the captured Journal.
    let mut absent = line;
    absent.batch_id = "bridge-00000000-0000-4000-8000-000000000003".into();
    for entry in &mut absent.vouchers[0].entries {
        entry.amount = "12.62".into();
    }
    recheck_import_admission(
        &absent,
        company_guid,
        "WR2 Unicode Lab",
        &captured,
        &captured,
        &catalogue,
        None,
        &single_currency,
        &ledger_binding,
    )
    .expect("paired captured source establishes absence of the new candidate");

    // bridge#626: a ledger added since approval that folds equal to a named one
    // refuses the queued post. The catalogue is a test-local rewrite of the
    // capture, an unrelated ledger renamed `Cash` plus CR LF, and no evidence
    // of Tally behaviour. The named ledgers' binding is unchanged, so only the
    // twin check can refuse it.
    assert_eq!(
        catalogue.matches("WR2 Sales").count(),
        2,
        "name and NAME.LIST"
    );
    let twinned = catalogue.replace("WR2 Sales", "Cash&#13;&#10;");
    let error = recheck_import_admission(
        &absent,
        company_guid,
        "WR2 Unicode Lab",
        &captured,
        &captured,
        &twinned,
        None,
        &single_currency,
        &ledger_binding,
    )
    .expect_err("a folded twin added since approval must refuse the queued post");
    assert!(matches!(
        error.downcast_ref::<ApprovedImportAdmissionError>(),
        Some(ApprovedImportAdmissionError::LedgerFoldedTwin)
    ));

    // A bank voucher is classified from the group collection read beside the
    // catalogue. Without that read the queue refuses rather than post on half
    // a check, and a Journal that somehow carries one is a wiring fault too.
    let groups_bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-party-groups.utf16le.xml"
    );
    let groups = String::from_utf16(
        &groups_bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let mut payment = absent.clone();
    payment.vouchers[0].voucher_type = VoucherType::Payment;
    payment.vouchers[0].reference = None;
    let recheck = |line: &ImportLedgerLine, groups: Option<&str>| {
        recheck_import_admission(
            line,
            company_guid,
            "WR2 Unicode Lab",
            &captured,
            &captured,
            &catalogue,
            groups,
            &single_currency,
            &ledger_binding,
        )
    };
    recheck(&payment, Some(&groups)).expect("the captured masters classify this Payment");
    for (line, groups) in [(&payment, None), (&absent, Some(groups.as_str()))] {
        let error = recheck(line, groups).expect_err("a missing or stray group read must refuse");
        assert!(matches!(
            error.downcast_ref::<ApprovedImportAdmissionError>(),
            Some(ApprovedImportAdmissionError::AdmissionInconsistent)
        ));
    }
}

#[test]
fn the_whole_window_pre_post_request_is_admitted_on_the_verification_measurement() {
    // Review of #520. The request sent whole inside the dispatch lease is
    // admitted on what verify_import's read of the same window measured.
    let evidence = |bytes: usize| Evidence {
        request_sha256: String::new(),
        response_sha256: String::new(),
        bytes,
        state: "complete",
        read_at: None,
        duration_ms: None,
        reason_code: None,
    };
    let divided = [
        crate::agent::WindowPart {
            from: "20260801".into(),
            to: "20260815".into(),
            span: None,
        },
        crate::agent::WindowPart {
            from: "20260816".into(),
            to: "20260831".into(),
            span: None,
        },
    ];
    let budget = usize::try_from(crate::agent::WINDOW_READ_BUDGET_BYTES).unwrap();
    let light = crate::agent::WindowServed::of(&divided, &evidence(2 * budget), false);
    let heavy = crate::agent::WindowServed::of(&divided, &evidence(2 * budget + 2), false);
    assert_eq!(admit_post_window(Some(light)), Ok(()));
    assert_eq!(
        admit_post_window(Some(heavy)),
        Err(IMPORT_POST_WINDOW_NOT_BOUNDED.to_string())
    );
    // A verification that reported nothing is refused, not assumed small.
    assert_eq!(
        admit_post_window(None),
        Err(IMPORT_POST_WINDOW_NOT_BOUNDED.to_string())
    );
}

/// bridge#575. The post path used to compare a journal record only with
/// itself; the saved XML file Bridge built was never read. A record and file
/// that disagree must stop the post before anything is sent to Tally.
#[tokio::test]
async fn a_saved_xml_file_that_differs_from_the_record_is_refused_before_tally() {
    let directory = tempfile::tempdir().unwrap();
    let (mut line, mut endpoint) = batch();
    // Port 9 is not a Tally endpoint: a regression that reaches the network
    // fails there with a transport error, never with the codes below.
    endpoint.port = 9;
    line.endpoint_origin = Some(super::super::super::canonical_loopback_origin(&endpoint).unwrap());
    let server = Server::new(crate::agent::Settings {
        endpoint,
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    });
    server.append_import_ledger(&line).unwrap();
    let path = server
        .imports_dir()
        .unwrap()
        .join(format!("{}.xml", line.batch_id));
    let args = json!({"company_guid":line.company_guid,"batch_id":line.batch_id});

    // The record is self-consistent; the file holds a different Journal.
    let mut other = line.vouchers.clone();
    other[0].entries[0].amount = "99.00".into();
    other[0].entries[1].amount = "99.00".into();
    fs::write(
        &path,
        render_import_xml("Synthetic Accounts", &other, &line.batch_id),
    )
    .unwrap();
    assert!(admit_saved_journal_integrity(&line, &server.settings.endpoint).is_ok());
    assert_eq!(post_error(&server, &args).await, "import_batch_changed");

    fs::remove_file(&path).unwrap();
    assert_eq!(
        post_error(&server, &args).await,
        "import_persisted_file_unavailable"
    );

    // The exact file passes this check; the post then fails later, at the
    // network, because port 9 is not Tally.
    fs::write(
        &path,
        render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id),
    )
    .unwrap();
    let later = post_error(&server, &args).await;
    assert!(
        !matches!(
            later.as_str(),
            "import_batch_changed" | "import_persisted_file_unavailable"
        ),
        "{later}"
    );
}

/// The error code a post reports, whether it failed before or inside its
/// operation (the latter comes back as an outcome whose result names the
/// error), and that no attempt was recorded.
async fn post_error(server: &Server, args: &Value) -> String {
    match server.post_import(args).await {
        Ok(outcome) => {
            let result = &outcome.payload["result"];
            assert_ne!(result["attempt_recorded"], json!(true), "{result}");
            result["error"]["code"]
                .as_str()
                .unwrap_or("unexpected_success")
                .to_string()
        }
        Err(failure) => failure.code,
    }
}

/// bridge#579. Tally deletes a voucher only by the client REMOTEID it was
/// created with and exports its own GUID in that attribute, so the native
/// post must record the REMOTEID it sends, with its request hash, before
/// sending. The recorded value must be exactly the one in the request bytes.
#[test]
fn the_dispatch_intent_records_the_remoteid_the_native_request_carries() {
    let (line, _) = batch();
    let remote_id = Uuid::new_v4();
    let request = native_post_request(&line, RemoteIds::from_ids(vec![remote_id])).unwrap();
    assert_eq!(request.remote_ids.as_slice(), [remote_id]);
    let sent = request
        .xml
        .split("<VOUCHER REMOTEID=\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .unwrap();
    assert_eq!(request.xml.matches("REMOTEID=").count(), 1);
    assert_eq!(sent, remote_id.hyphenated().to_string());
    assert_eq!(
        request.request_sha256,
        sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
            &request.xml
        ))
    );

    let intent =
        serde_json::to_value(ledger::StatusRecord::dispatch_for(&line, &request, None)).unwrap();
    assert_eq!(intent["native_remote_id"], json!(sent));
    assert_eq!(
        intent["native_request_sha256"],
        json!(request.request_sha256)
    );

    // Two posts never share a REMOTEID: reuse could make Tally upsert.
    let other = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    assert_ne!(other.remote_ids.as_slice(), request.remote_ids.as_slice());
    assert_ne!(other.request_sha256, request.request_sha256);
}

/// A Payment with one bank credit and `parties` debits named by `name`.
fn payment_with(parties: usize, name: impl Fn(usize) -> String) -> ImportLedgerLine {
    let (mut line, _) = batch();
    let voucher = &mut line.vouchers[0];
    voucher.voucher_type = VoucherType::Payment;
    voucher.reference = None;
    voucher.entries = (0..parties)
        .map(|index| ImportEntry {
            ledger: name(index),
            amount: "1.00".into(),
            side: EntrySide::Dr,
        })
        .chain(std::iter::once(ImportEntry {
            ledger: "Cash".into(),
            amount: format!("{parties}.00"),
            side: EntrySide::Cr,
        }))
        .collect();
    line
}

/// The native dialog cannot scroll, so a preview past its caps is refused,
/// never truncated. Multi-entry bank vouchers reach the caps: seventeen fixed
/// lines plus one per entry, so seven entries fit 24 lines and eight do not.
#[test]
fn a_bank_preview_is_refused_at_each_cap_rather_than_truncated() {
    let (_, endpoint) = batch();
    let seven = admit_fresh_saved_voucher(&payment_with(6, |i| format!("Party {i}")), &endpoint)
        .expect("seven entries fit");
    assert_eq!(seven.lines().count(), 24, "{seven}");
    assert_eq!(
        admit_fresh_saved_voucher(&payment_with(7, |i| format!("Party {i}")), &endpoint)
            .unwrap_err(),
        "import_review_too_large"
    );
    // One entry line is `Dr 1.00  "<name>"`: 11 characters around the name.
    let fits = admit_fresh_saved_voucher(&payment_with(1, |_| "N".repeat(89)), &endpoint)
        .expect("a 100-character line fits");
    assert!(
        fits.lines().any(|line| line.chars().count() == 100),
        "{fits}"
    );
    assert_eq!(
        admit_fresh_saved_voucher(&payment_with(1, |_| "N".repeat(90)), &endpoint).unwrap_err(),
        "import_review_too_large"
    );
}

/// Grow a preview one character at a time with `build(pad)`: it fits up to some
/// size and is refused beyond it, never the other way round. The largest preview
/// that fit, and the refusal that followed it (`None` if none did by `max_pad`).
fn largest_fit(
    build: impl Fn(usize) -> Result<String, String>,
    max_pad: usize,
) -> (String, Option<String>) {
    let (mut fit, mut refusal) = (None, None);
    for pad in 0..=max_pad {
        match build(pad) {
            Ok(preview) => {
                assert!(refusal.is_none(), "a fit after a refusal, at pad {pad}");
                fit = Some(preview);
            }
            Err(code) => {
                refusal.get_or_insert(code);
            }
        }
    }
    (fit.expect("the smallest preview fits"), refusal)
}

/// A batch of two Payments over the same `parties` ledgers named by `names`, in
/// the company `company`.
fn payment_batch_with(names: &[String], company: &str) -> ImportLedgerLine {
    let mut line = payment_with(names.len(), |index| names[index].clone());
    line.company.as_mut().unwrap().name = company.into();
    let mut second = line.vouchers[0].clone();
    second.bridge_txn_id = "journal-test-2".into();
    line.vouchers.push(second);
    line.txn_ids.push("journal-test-2".into());
    refresh_batch_sha256(&mut line);
    line
}

/// The lengths of fields that grow together: `pad` more characters are dealt out
/// round-robin from `start`, and each field stops at its own cap, so the total
/// grows by one, or by nothing, per step.
fn grown_lengths(pad: usize, start: usize, caps: &[usize]) -> Vec<usize> {
    caps.iter()
        .enumerate()
        .map(|(field, cap)| {
            (start + pad / caps.len() + usize::from(field < pad % caps.len())).min(*cap)
        })
        .collect()
}

/// The longest length, up to 200, at which `fits` still holds.
fn widest(fits: impl Fn(usize) -> bool) -> usize {
    (1..=200)
        .take_while(|&length| fits(length))
        .last()
        .expect("fits at one character")
}

/// A name of `length` characters: a letter for `field`, so names differ, then
/// `filler`.
fn name_of(field: usize, length: usize, filler: &str) -> String {
    format!(
        "{}{}",
        char::from_u32(0x0915 + field as u32).expect("a Devanagari letter"),
        filler.repeat(length.saturating_sub(1))
    )
}

/// One Payment of six parties and a balancing ledger in a company, whose every
/// free-text field (the company, each entry's ledger, the narration and the
/// reference) has the length `lengths` gives: company, six parties, the balancing
/// ledger, narration, reference.
fn payment_of(lengths: &[usize]) -> ImportLedgerLine {
    let mut line = payment_with(6, |party| name_of(party, lengths[1 + party], "N"));
    line.company.as_mut().unwrap().name = name_of(9, lengths[0], "N");
    let voucher = &mut line.vouchers[0];
    voucher.entries.last_mut().unwrap().ledger = name_of(6, lengths[7], "N");
    voucher.narration = Some(name_of(7, lengths[8], "N"));
    voucher.reference = Some(name_of(8, lengths[9], "N"));
    refresh_batch_sha256(&mut line);
    line
}

/// The single-voucher preview's 1,600-character cap is exact: with every field
/// grown to its own width (the company and the balancing ledger as well as the
/// parties, narration and reference), a preview of exactly 1,600 characters in 24
/// lines fits and one more is refused. Branding the dialog text added characters
/// to every preview, so a preview near the cap is refused loudly, never cut, and
/// the boundary is pinned here.
#[test]
fn a_single_voucher_preview_fits_at_exactly_1600_characters_and_not_one_more() {
    let (_, endpoint) = batch();
    let fits = |lengths: &[usize]| admit_fresh_saved_voucher(&payment_of(lengths), &endpoint);
    let caps: Vec<usize> = (0..10)
        .map(|field| {
            widest(|length| {
                let mut lengths = vec![1; 10];
                lengths[field] = length;
                fits(&lengths).is_ok()
            })
        })
        .collect();
    let (fit, refusal) = largest_fit(|pad| fits(&grown_lengths(pad, 1, &caps)), 10 * 100);
    assert_eq!(fit.chars().count(), 1_600, "{fit}");
    assert_eq!(fit.lines().count(), 24, "{fit}");
    assert!(fit.lines().all(|line| line.chars().count() <= 100), "{fit}");
    assert_eq!(refusal.as_deref(), Some("import_review_too_large"));
}

/// The batch post preview's caps (3,200 characters, 7,000 bytes) against what 40
/// lines allow, with the company, all 22 parties and the balancing ledger grown to
/// their own width: ASCII names and three-byte names. At full width the preview
/// still fits all 40 lines and is under both caps (3,123 characters, 6,725 bytes
/// with three-byte names when measured), so the character and byte caps do not
/// bind for it: the line caps do. Measured, not derived: a change that lets this
/// preview grow past them fails here and needs its own boundary test.
#[test]
fn the_batch_post_preview_at_full_width_against_its_caps() {
    let (_, endpoint) = batch();
    for filler in ["N", "न"] {
        let fits = |lengths: &[usize]| {
            // The company, 22 parties and the balancing ledger.
            let names: Vec<String> = (0..22)
                .map(|party| name_of(party, lengths[1 + party], filler))
                .collect();
            let mut line = payment_batch_with(&names, &name_of(24, lengths[0], filler));
            for voucher in &mut line.vouchers {
                voucher.entries.last_mut().unwrap().ledger = name_of(23, lengths[23], filler);
            }
            refresh_batch_sha256(&mut line);
            review_preview_with(&line, &endpoint, &[])
        };
        let caps: Vec<usize> = (0..24)
            .map(|field| {
                widest(|length| {
                    let mut lengths = vec![1; 24];
                    lengths[field] = length;
                    fits(&lengths).is_ok()
                })
            })
            .collect();
        let (fit, refusal) = largest_fit(|pad| fits(&grown_lengths(pad, 1, &caps)), 24 * 100);
        assert_eq!(refusal, None, "every field at its width still fits");
        assert_eq!(fit.lines().count(), 40, "{fit}");
        assert!(
            fit.chars().count() < 3_200,
            "{} characters",
            fit.chars().count()
        );
        assert!(fit.len() < 7_000, "{} bytes", fit.len());
    }
}

fn captured_currencies(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[test]
fn a_post_is_admitted_only_into_a_book_with_one_currency_master() {
    // Both single-master spellings measured on 7.1 are admitted, whatever the
    // base is called: the gate asks how many masters there are, not which.
    for single in [
        &include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        )[..],
        &include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_legacy_live.utf16le.xml"
        )[..],
    ] {
        admit_post_currency(&captured_currencies(single)).expect("one master admits");
    }
    // The captured two-master book refuses, naming both masters as read.
    let multi = captured_currencies(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
    ));
    let names = bridge_tally_protocol::native_outstandings::parse_company_currency(&multi)
        .unwrap()
        .names;
    assert_eq!(names.len(), 2, "{names:?}");
    assert_eq!(
        admit_post_currency(&multi),
        Err(ApprovedImportAdmissionError::MultiCurrencyBook { currencies: names })
    );
    // Kept out of every serialized output that carries the struct.
    let serialized = serde_json::to_value(
        bridge_tally_protocol::native_outstandings::parse_company_currency(&multi).unwrap(),
    )
    .unwrap();
    assert!(serialized.get("names").is_none(), "{serialized}");
    // No base can be named from a response that does not parse, from one with
    // no master, or from one whose only master has no NAME (which the parser
    // itself refuses). The last two are explicit edits of the one-master
    // capture, not live evidence.
    let single = captured_currencies(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    // The row's own close: CMPINFO's `<CURRENCY>0</CURRENCY>` comes earlier.
    let row_start = single.find("<CURRENCY NAME=").unwrap();
    let row_end =
        row_start + single[row_start..].find("</CURRENCY>").unwrap() + "</CURRENCY>".len();
    let no_master = format!("{}{}", &single[..row_start], &single[row_end..]);
    assert_eq!(
        bridge_tally_protocol::native_outstandings::parse_company_currency(&no_master)
            .unwrap()
            .currency_count,
        0,
        "the edit must leave a readable collection with no master"
    );
    assert_eq!(single.matches(" NAME=\"I₹\"").count(), 1);
    let nameless = single.replace(" NAME=\"I₹\"", " NAME=\"\"");
    assert!(bridge_tally_protocol::native_outstandings::parse_company_currency(&nameless).is_err());
    for undetermined in ["<ENVELOPE/>", no_master.as_str(), nameless.as_str()] {
        assert_eq!(
            admit_post_currency(undetermined),
            Err(ApprovedImportAdmissionError::BaseCurrencyUndetermined),
            "{undetermined}"
        );
    }
}

#[test]
fn a_multi_currency_refusal_names_the_masters_in_plain_words_only_when_nothing_was_attempted() {
    let currencies = (1..=10).map(|n| format!("C{n}")).collect::<Vec<_>>();
    let refused = |attempted: Value| {
        let mut payload = json!({"result":{"attempt_recorded":attempted,
            "error":{"code":"import_multi_currency_unsupported","message":"generic"}}});
        name_refused_currencies(&mut payload, &currencies);
        payload["result"]["error"].clone()
    };
    let error = refused(json!(false));
    assert_eq!(
        error["message"],
        "This company has more than one currency defined (C1, C2, C3, C4, C5, C6, C7, C8 and 2 \
         more); ComplyEaze Bridge does not post into multi-currency books yet. Nothing was posted."
    );
    assert_eq!(error["currencies_seen"].as_array().unwrap().len(), 8);
    assert_eq!(error["currencies_total"], 10);
    // An unknown attempt keeps its instruction to reconcile.
    for attempted in [json!(true), Value::Null] {
        assert_eq!(refused(attempted)["message"], "generic");
    }
}

/// The captured catalogue's binding of `names`, as a post binds them.
fn captured_binding(names: &[&str]) -> bridge_tally_protocol::StandardLedgerCatalogBinding {
    let catalogue = captured_currencies(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml"
    ));
    bridge_tally_protocol::parse_standard_ledger_catalog_with_identities(
        &catalogue,
        "WR2 Unicode Lab",
        "61c6de69-1748-461c-ad3f-162cb949df9f",
    )
    .unwrap()
    .bind_selected(names.iter().map(|name| name.to_string()))
    .unwrap()
}

fn recorded(pairs: &[(&str, &str)]) -> Vec<BoundLedger> {
    pairs
        .iter()
        .map(|(name, guid)| BoundLedger {
            name: name.to_string(),
            guid: guid.to_string(),
        })
        .collect()
}

#[test]
fn a_post_admits_only_the_ledgers_its_build_bound() {
    // The captured catalogue's GUIDs for these two ledgers.
    const A: &str = "61c6de69-1748-461c-ad3f-162cb949df9f-000000d5";
    const B: &str = "61c6de69-1748-461c-ad3f-162cb949df9f-0000001f";
    let now = captured_binding(&["Bridge Nested Debtor WR4", "Cash"]);
    // Same ledgers, same GUIDs (GUID case is not identity).
    assert_eq!(
        admit_build_binding(
            Some(&recorded(&[
                ("Cash", &B.to_uppercase()),
                ("Bridge Nested Debtor WR4", A)
            ])),
            &now
        ),
        Ok(())
    );
    // Cash renamed and a new Cash created since the build: named.
    assert_eq!(
        admit_build_binding(
            Some(&recorded(&[
                ("Bridge Nested Debtor WR4", A),
                ("Cash", "61c6de69-1748-461c-ad3f-162cb949df9f-000000ff")
            ])),
            &now
        ),
        Err(BuildBindingRefusal::Changed(vec!["Cash".into()]))
    );
    // A ledger the record does not hold is not admitted by name alone.
    assert_eq!(
        admit_build_binding(Some(&recorded(&[("Bridge Nested Debtor WR4", A)])), &now),
        Err(BuildBindingRefusal::Changed(vec!["Cash".into()]))
    );
    // Two ledgers that exchanged names since the build: every GUID is still
    // recorded, but under the other name, so both refuse.
    assert_eq!(
        admit_build_binding(
            Some(&recorded(&[("Cash", A), ("Bridge Nested Debtor WR4", B)])),
            &now
        ),
        Err(BuildBindingRefusal::Changed(vec![
            "Bridge Nested Debtor WR4".into(),
            "Cash".into()
        ]))
    );
    // A record built before binding existed has nothing to compare.
    assert_eq!(
        admit_build_binding(None, &now),
        Err(BuildBindingRefusal::Unbound)
    );
}

#[test]
fn a_record_without_ledger_identities_reads_as_built_before_binding() {
    // An older record, exactly as it was serialized: no `ledger_identities`.
    let (mut line, _) = batch();
    line.ledger_identities = None;
    let json = serde_json::to_value(&line).unwrap();
    assert!(json.get("ledger_identities").is_none(), "{json}");
    let reread: ImportLedgerLine = serde_json::from_value(json).unwrap();
    assert_eq!(reread.ledger_identities, None);
    // And a current record keeps them across a round trip.
    line.ledger_identities = Some(recorded(&[("Cash", "g")]));
    let reread: ImportLedgerLine =
        serde_json::from_value(serde_json::to_value(&line).unwrap()).unwrap();
    assert_eq!(reread.ledger_identities, line.ledger_identities);
}

#[test]
fn a_changed_ledger_is_named_in_plain_words_only_when_nothing_was_attempted() {
    let ledgers = (1..=9).map(|n| format!("L{n}")).collect::<Vec<_>>();
    let refused = |attempted: Value| {
        let mut payload = json!({"result":{"attempt_recorded":attempted,
            "error":{"code":"import_masters_changed_since_build","message":"generic"}}});
        name_changed_ledgers(&mut payload, &ledgers);
        payload["result"]["error"].clone()
    };
    let error = refused(json!(false));
    let message = error["message"].as_str().unwrap();
    assert!(
        message.starts_with(
            "A ledger this batch names (listed in error.ledgers_changed) is no longer the one it was built against"
        ),
        "{message}"
    );
    assert!(!message.contains("L1"), "{message}");
    // Listed, each marked as a party name, up to the bound; counted in full.
    let listed = (1..=8)
        .map(|n| serde_json::to_value(party_name(format!("L{n}"))).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(error["ledgers_changed"], json!(listed));
    assert_eq!(error["ledgers_changed_total"], 9);
    for attempted in [json!(true), Value::Null] {
        assert_eq!(refused(attempted)["message"], "generic");
    }
    // An unbound batch is told to rebuild, only when nothing was attempted.
    let unbound = |attempted: Value| {
        let mut payload = json!({"result":{"attempt_recorded":attempted,
            "error":{"code":"import_batch_predates_ledger_binding","message":"generic"}}});
        explain_unbound_batch(&mut payload);
        payload["result"]["error"]["message"].clone()
    };
    assert!(unbound(json!(false))
        .as_str()
        .unwrap()
        .contains("Build the batch again"));
    for attempted in [json!(true), Value::Null] {
        assert_eq!(unbound(attempted), "generic");
    }
}

/// bridge#239: a clean, verified post is still not posted_verified when its
/// masters changed across the post; the message says the voucher is in Tally
/// and must not be posted again.
#[test]
fn a_masters_doubt_after_the_post_downgrades_a_clean_verified_post() {
    let response = dispatch_response("success", 1, 0);
    let finalized = |masters: Value| {
        let mut payload = json!({"result": {"counts": {"posted_verified": 1}, "duplicates": []}});
        finalize_current_dispatch(&mut payload, Some(&response), Some(&masters), 1);
        payload["result"].clone()
    };
    for (state, code) in [
        (
            "posted_under_changed_masters",
            "posted_under_changed_masters",
        ),
        ("check_unavailable", "masters_after_post_unconfirmed"),
        ("check_pending", "masters_after_post_unconfirmed"),
        // A state this build does not know, or a not_checked for any other
        // reason than unmoved masters, is a doubt too.
        ("not_checked", "masters_after_post_unconfirmed"),
        ("some_future_state", "masters_after_post_unconfirmed"),
    ] {
        let result = finalized(json!({"state": state}));
        assert_eq!(
            result["dispatch"]["state"], "reconciliation_required",
            "{state}"
        );
        assert_eq!(result["error"]["code"], code);
        let message = result["error"]["message"].as_str().unwrap();
        assert!(message.starts_with("Posted to Tally"), "{message}");
        assert!(message.contains("do not rebuild this event"), "{message}");
        // Only a check that could not finish is promised a later one.
        assert_eq!(
            message.contains("checks again"),
            state == "check_unavailable" || state == "check_pending",
            "{message}"
        );
    }
    for masters in [
        json!({"state": "unchanged"}),
        json!({"state": "not_checked", "reason": "masters_unmoved"}),
    ] {
        let result = finalized(masters.clone());
        assert_eq!(result["dispatch"]["state"], "posted_verified", "{masters}");
        assert!(result.get("error").is_none());
    }
    // A reconcile of an earlier attempt is held back by the same doubt.
    let reconciled = |masters: Option<Value>| {
        let mut payload = json!({"result": {"counts": {"posted_verified": 1}, "duplicates": []}});
        finalize_previous_attempt_reconciliation(
            &mut payload,
            Some(&response),
            masters.as_ref(),
            1,
        );
        payload["result"].clone()
    };
    let doubted = reconciled(Some(
        json!({"state": "posted_under_changed_masters", "ledgers": ["Cash"]}),
    ));
    assert_eq!(
        doubted["dispatch"]["state"], "reconciliation_required",
        "{doubted}"
    );
    assert_eq!(doubted["error"]["code"], "posted_under_changed_masters");
    let clear = reconciled(None);
    assert_eq!(
        clear["dispatch"]["state"], "previous_attempt_reconciled",
        "{clear}"
    );
}

/// bridge#626 slice 1 changes no post code: this pins the refusal that already
/// applies to a ledger name ending in CR LF. Such a name can now be built and
/// imported from the file, but the native dialog cannot yet show it so that an
/// operator can tell it from its twin; slice 2 changes that, and this test.
#[test]
fn native_preview_refuses_a_ledger_name_ending_in_a_line_break() {
    let (mut line, endpoint) = batch();
    line.vouchers[0].entries[0].ledger.push_str("\r\n");
    refresh_batch_sha256(&mut line);
    assert_eq!(
        admit_saved_journal(&line, &endpoint).unwrap_err(),
        "import_review_layout_text"
    );
}

/// A batch of two for the N-voucher paths. Admission still posts one voucher,
/// so these call each generalized piece directly.
fn batch_of_two() -> ImportLedgerLine {
    let (mut line, _) = batch();
    let mut second = line.vouchers[0].clone();
    second.bridge_txn_id = "journal-test-2".into();
    second.narration = Some("Synthetic second".into());
    line.vouchers.push(second);
    line.txn_ids.push("journal-test-2".into());
    line.sha256 = sha256_hex(
        render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id).as_bytes(),
    );
    line
}

/// One fresh id per voucher, all distinct; none for an empty batch.
#[test]
fn remote_ids_are_minted_one_per_voucher_and_distinct() {
    let ids = RemoteIds::mint(3).unwrap();
    let distinct = ids
        .as_slice()
        .iter()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!((ids.as_slice().len(), distinct.len()), (3, 3));
    assert_eq!(
        RemoteIds::mint(0).err().as_deref(),
        Some("import_post_requires_one_voucher")
    );
    // Never more than the journal admits on read.
    assert_eq!(
        RemoteIds::mint(ledger::MAX_BATCH_POST_VOUCHERS)
            .map(|ids| ids.as_slice().len())
            .ok(),
        Some(ledger::MAX_BATCH_POST_VOUCHERS)
    );
    assert_eq!(
        RemoteIds::mint(ledger::MAX_BATCH_POST_VOUCHERS + 1)
            .err()
            .as_deref(),
        Some("import_post_batch_too_large")
    );
}

/// The request carries every voucher, each with its own REMOTEID in batch
/// order, and refuses ids that do not match the vouchers one for one.
#[test]
fn a_native_request_renders_every_voucher_with_its_own_remote_id() {
    let line = batch_of_two();
    let ids = [Uuid::new_v4(), Uuid::new_v4()];
    let request = native_post_request(&line, RemoteIds::from_ids(ids.to_vec())).unwrap();
    let mut reader = quick_xml::Reader::from_str(&request.xml);
    let mut remote_ids = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            quick_xml::events::Event::Start(tag) if tag.name().as_ref() == b"VOUCHER" => {
                let remote_id = tag
                    .attributes()
                    .map(Result::unwrap)
                    .find(|attribute| attribute.key.as_ref() == b"REMOTEID")
                    .map(|attribute| String::from_utf8(attribute.value.to_vec()).unwrap());
                remote_ids.push(remote_id.unwrap());
            }
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(
        remote_ids,
        ids.iter()
            .map(|id| id.hyphenated().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        native_post_request(&line, RemoteIds::from_ids(vec![ids[0]]))
            .err()
            .as_deref(),
        Some("import_post_remote_ids_mismatch")
    );
}

/// One voucher keeps the single-id intent an older binary can read; a batch
/// records every id, and the journal finds each of them.
#[test]
fn the_intent_keeps_the_single_id_shape_for_one_voucher() {
    let (one, _) = batch();
    let id = Uuid::new_v4();
    let single = serde_json::to_value(ledger::StatusRecord::dispatch_for(
        &one,
        &native_post_request(&one, RemoteIds::from_ids(vec![id])).unwrap(),
        None,
    ))
    .unwrap();
    assert_eq!(
        single["native_remote_id"],
        json!(id.hyphenated().to_string())
    );
    assert!(single.get("native_remote_ids").is_none(), "{single}");

    let two = batch_of_two();
    let ids = [Uuid::new_v4(), Uuid::new_v4()];
    let batch = serde_json::to_value(ledger::StatusRecord::dispatch_for(
        &two,
        &native_post_request(&two, RemoteIds::from_ids(ids.to_vec())).unwrap(),
        None,
    ))
    .unwrap();
    assert!(batch.get("native_remote_id").is_none(), "{batch}");
    assert_eq!(
        batch["native_remote_ids"],
        json!(ids
            .iter()
            .map(|id| id.hyphenated().to_string())
            .collect::<Vec<_>>())
    );
}

/// Every voucher must be absent: N not found, over N rows, and never zero.
#[test]
fn absence_is_required_for_every_voucher() {
    let result = |not_found: u64, rows: usize| json!({"counts":{"not_found":not_found},"vouchers":vec![json!({}); rows]});
    assert!(require_absent_verification_result(&result(2, 2), 2).is_ok());
    for (not_found, rows, count) in [(1, 2, 2), (2, 1, 2), (1, 1, 2), (0, 0, 0)] {
        assert_eq!(
            require_absent_verification_result(&result(not_found, rows), count)
                .err()
                .as_deref(),
            Some("import_preexisting_identity"),
            "{not_found} {rows} {count}"
        );
    }
}

/// A batch is clean only when Tally created exactly its N vouchers and the
/// readback verified all N; one short on either side is not clean.
#[test]
fn a_batch_is_clean_only_with_n_creates_and_n_verified() {
    // A batch's masters check found nothing and its step matched.
    let matched = json!({"state":"unchanged","batch_step":{"state":"matched"}});
    let verdict = |created: u64, verified: u64| {
        let mut payload = json!({"result":{"counts":{"posted_verified":verified},"duplicates":[]}});
        finalize_current_dispatch(
            &mut payload,
            Some(&dispatch_response("success", created, 0)),
            Some(&matched),
            2,
        );
        payload["result"]["dispatch"]["state"].clone()
    };
    assert_eq!(verdict(2, 2), "posted_verified");
    assert_eq!(verdict(1, 2), "reconciliation_required");
    assert_eq!(verdict(2, 1), "reconciliation_required");
    assert_eq!(verdict(3, 2), "reconciliation_required");
    // With no step verdict recorded, even N creates and N verified is not clean.
    let mut payload = json!({"result":{"counts":{"posted_verified":2},"duplicates":[]}});
    finalize_current_dispatch(
        &mut payload,
        Some(&dispatch_response("success", 2, 0)),
        Some(&json!({"state":"unchanged"})),
        2,
    );
    assert_eq!(
        payload["result"]["dispatch"]["state"],
        "reconciliation_required"
    );
    assert_eq!(payload["result"]["error"]["code"], "batch_step_unconfirmed");
}

/// A reconciliation of an earlier batch attempt is clean only with N creates
/// and N verified, as for the current dispatch.
#[test]
fn a_previous_batch_attempt_reconciles_only_with_n_creates_and_n_verified() {
    // A batch's masters check found nothing and its step matched.
    let matched = json!({"state":"unchanged","batch_step":{"state":"matched"}});
    let verdict = |created: u64, verified: u64| {
        let mut payload = json!({"result":{"counts":{"posted_verified":verified},"duplicates":[]}});
        finalize_previous_attempt_reconciliation(
            &mut payload,
            Some(&dispatch_response("success", created, 0)),
            Some(&matched),
            2,
        );
        payload["result"]["dispatch"]["state"].clone()
    };
    assert_eq!(verdict(2, 2), "previous_attempt_reconciled");
    assert_eq!(verdict(1, 2), "reconciliation_required");
    assert_eq!(verdict(2, 1), "reconciliation_required");
    assert_eq!(verdict(3, 2), "reconciliation_required");
    // With no step verdict recorded, even N creates and N verified is not clean.
    let mut payload = json!({"result":{"counts":{"posted_verified":2},"duplicates":[]}});
    finalize_previous_attempt_reconciliation(
        &mut payload,
        Some(&dispatch_response("success", 2, 0)),
        Some(&json!({"state":"unchanged"})),
        2,
    );
    assert_eq!(
        payload["result"]["dispatch"]["state"],
        "reconciliation_required"
    );
    assert_eq!(payload["result"]["error"]["code"], "batch_step_unconfirmed");
}

/// A batch with one voucher of each type, for the batch approval text.
fn batch_of_every_type() -> (ImportLedgerLine, TallyEndpointConfig) {
    let (mut line, endpoint) = batch();
    let voucher = |id: &str, voucher_type: &str, entries: Value| -> ImportVoucher {
        serde_json::from_value(json!({"bridge_txn_id":id,"date":"20260902",
            "voucher_type":voucher_type,"narration":"Synthetic test only","entries":entries}))
        .unwrap()
    };
    line.vouchers.push(voucher(
        "receipt-1",
        "Receipt",
        json!([
        {"ledger":"Cash","amount":"40.00","side":"Dr"},
        {"ledger":"Party A","amount":"40.00","side":"Cr"}]),
    ));
    line.vouchers.push(voucher(
        "payment-1",
        "Payment",
        json!([
        {"ledger":"Party B","amount":"15.00","side":"Dr"},
        {"ledger":"Bank","amount":"15.00","side":"Cr"}]),
    ));
    line.vouchers.push(voucher(
        "contra-1",
        "Contra",
        json!([
        {"ledger":"Bank","amount":"5.00","side":"Dr"},
        {"ledger":"Cash","amount":"5.00","side":"Cr"}]),
    ));
    line.txn_ids
        .extend(["receipt-1", "payment-1", "contra-1"].map(String::from));
    line.date_to = "20260902".into();
    line.sha256 = sha256_hex(
        render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id).as_bytes(),
    );
    (line, endpoint)
}

/// Batch admission: one voucher unless batch posting lets more through, and
/// never more than the cap.
#[test]
fn a_batch_is_admitted_only_under_the_batch_limit() {
    let (_, endpoint) = batch();
    let two = batch_of_two();
    assert_eq!(
        admit_saved_voucher_integrity(&two, &endpoint, PostScope::Vouchers, 1)
            .err()
            .as_deref(),
        Some("import_post_requires_one_voucher")
    );
    assert!(admit_saved_voucher_integrity(
        &two,
        &endpoint,
        PostScope::Vouchers,
        ledger::MAX_BATCH_POST_VOUCHERS
    )
    .is_ok());
    let mut many = two.clone();
    while many.vouchers.len() <= ledger::MAX_BATCH_POST_VOUCHERS {
        let mut extra = many.vouchers[0].clone();
        extra.bridge_txn_id = format!("journal-extra-{}", many.vouchers.len());
        many.txn_ids.push(extra.bridge_txn_id.clone());
        many.vouchers.push(extra);
    }
    many.sha256 = sha256_hex(
        render_import_xml("Synthetic Accounts", &many.vouchers, &many.batch_id).as_bytes(),
    );
    assert_eq!(
        admit_saved_voucher_integrity(
            &many,
            &endpoint,
            PostScope::Vouchers,
            ledger::MAX_BATCH_POST_VOUCHERS
        )
        .err()
        .as_deref(),
        Some("import_post_batch_too_large")
    );
    // Every voucher's type is checked, not only the first's.
    let (mixed, _) = batch_of_every_type();
    assert_eq!(
        admit_saved_voucher_integrity(
            &mixed,
            &endpoint,
            PostScope::JournalOnly,
            ledger::MAX_BATCH_POST_VOUCHERS
        )
        .err()
        .as_deref(),
        Some("import_post_requires_one_journal")
    );
    // The desktop posts one Journal, whatever the limit.
    assert_eq!(
        admit_saved_voucher_integrity(&two, &endpoint, PostScope::JournalOnly, 1)
            .err()
            .as_deref(),
        Some("import_post_requires_one_journal")
    );
}

/// The batch approval text: every ledger's totals and entry count, the types,
/// the money Receipts and Payments move, and a line for Contras and Journals.
#[test]
fn a_batch_approval_summarizes_every_ledger_and_the_money_the_types_move() {
    let (line, endpoint) = batch_of_every_type();
    let preview = admit_fresh_saved_voucher(&line, &endpoint).unwrap();
    for expected in [
        "Create 4 vouchers in \"Synthetic Accounts\"",
        "Types: 1 Contra, 1 Journal, 1 Payment, 1 Receipt",
        "Dates: 20260901 to 20260902",
        "Dr 40  Cr 17.5  3 entries  \"Cash\"",
        "Dr 5  Cr 15  2 entries  \"Bank\"",
        "Dr 12.5  Cr 0  1 entry  \"Expense\"",
        "Money in by Receipt vouchers: 40",
        "Money out by Payment vouchers: 15",
        "Contra: moves between cash/bank ledgers, net zero",
        "Journals may also move cash/bank ledgers; see the per-ledger totals",
        "Not shown here: each voucher's own date, narration and reference.",
        "After a timeout, reconcile this batch; do not rebuild or resend it.",
    ] {
        assert!(
            preview.contains(expected),
            "missing {expected:?} in:\n{preview}"
        );
    }
    // One voucher keeps the single-voucher approval.
    let (one, endpoint) = batch();
    assert!(admit_fresh_saved_voucher(&one, &endpoint)
        .unwrap()
        .starts_with("Create ONE Journal"));
}

/// A batch whose summary would not fit one dialog is refused, never cut;
/// and a numbered voucher anywhere in it is refused.
#[test]
fn a_batch_approval_that_does_not_fit_is_refused_and_every_voucher_is_unnumbered() {
    let (mut line, endpoint) = batch_of_every_type();
    for index in 0..40 {
        let mut extra = line.vouchers[0].clone();
        extra.bridge_txn_id = format!("journal-wide-{index}");
        extra.entries[0].ledger = format!("Expense {index:02}");
        line.vouchers.push(extra);
    }
    assert_eq!(
        admit_fresh_saved_voucher(&line, &endpoint).err().as_deref(),
        Some("import_review_too_large")
    );
    let (mut numbered, endpoint) = batch_of_every_type();
    numbered.vouchers[2].voucher_number = Some("7".into());
    assert_eq!(
        admit_fresh_saved_voucher(&numbered, &endpoint)
            .err()
            .as_deref(),
        Some("import_post_numbered_journal_unsupported")
    );
}

fn records_server(directory: &std::path::Path) -> Server {
    Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: true,
    })
}

/// A batch's durable doubts: the masters verdict and the step verdict are
/// kept independently and either one doubting is doubt, in all four
/// combinations, whichever is recorded first; a crash before either is
/// recorded is doubt; a one-voucher record has no step verdict at all.
#[test]
fn a_batch_keeps_its_masters_and_step_verdicts_independently() {
    let unchanged = json!({"state":"unchanged"});
    let changed = json!({"state":"posted_under_changed_masters","trigger":"masters_moved","ledgers":["Cash"]});
    let matched =
        json!({"before":10,"after":12,"step":2,"reported_created":2,"matches_created":true});
    let unmatched =
        json!({"before":10,"after":13,"step":3,"reported_created":2,"matches_created":false});
    for (masters, step, step_first, expected) in [
        (&unchanged, &matched, true, None),
        (&unchanged, &matched, false, None),
        (&unchanged, &unmatched, true, Some("batch_step_unconfirmed")),
        (
            &unchanged,
            &unmatched,
            false,
            Some("batch_step_unconfirmed"),
        ),
        (
            &changed,
            &matched,
            true,
            Some("posted_under_changed_masters"),
        ),
        (
            &changed,
            &unmatched,
            false,
            Some("posted_under_changed_masters"),
        ),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let server = records_server(directory.path());
        server.imports_dir().unwrap();
        server.record_post_checks_pending("batch-a", true).unwrap();
        let recorded = if step_first {
            server.record_batch_step_verdict("batch-a", step);
            server.record_masters_verdict_for("batch-a", masters.clone(), true)
        } else {
            let verdict = server.record_masters_verdict_for("batch-a", masters.clone(), true);
            server.record_batch_step_verdict("batch-a", step);
            let _ = verdict;
            super::super::read_masters_check(&server.imports_dir().unwrap(), "batch-a").unwrap()
        };
        assert_eq!(
            post_doubt(Some(&recorded), 2).map(|(code, _)| code),
            expected,
            "{recorded}"
        );
        // The same doubt reads from the record alone, as an amendment's
        // baseline check reads it.
        assert_eq!(
            masters_doubt(Some(&recorded)).map(|(code, _)| code),
            expected,
            "{recorded}"
        );
        // The step verdict is kept beside whichever masters verdict.
        assert_eq!(
            recorded["batch_step"]["state"],
            if step["matches_created"] == true {
                "matched"
            } else {
                "unmatched"
            },
            "{recorded}"
        );
    }
    // A crash after the pending record and before any verdict: doubt.
    let directory = tempfile::tempdir().unwrap();
    let server = records_server(directory.path());
    server.record_post_checks_pending("batch-a", true).unwrap();
    let pending = super::super::read_masters_check(&server.imports_dir().unwrap(), "batch-a");
    assert!(post_doubt(pending.as_ref(), 2).is_some());
    // The masters check could not finish, but the step matched: still doubt.
    server.record_batch_step_verdict("batch-a", &matched);
    let pending = super::super::read_masters_check(&server.imports_dir().unwrap(), "batch-a");
    assert!(post_doubt(pending.as_ref(), 2).is_some(), "{pending:?}");
    // A one-voucher record has no step verdict, and a clean one is clean;
    // the same record read for a batch is doubt, since it holds no step.
    let directory = tempfile::tempdir().unwrap();
    let server = records_server(directory.path());
    server.record_post_checks_pending("batch-b", false).unwrap();
    let single = server.record_masters_verdict("batch-b", unchanged.clone());
    assert!(single.get("batch_step").is_none(), "{single}");
    assert!(post_doubt(Some(&single), 1).is_none());
    assert_eq!(
        post_doubt(Some(&single), 2).map(|(code, _)| code),
        Some("batch_step_unconfirmed")
    );
}

/// A step Bridge never saw match is doubt, never a match: no step at all (a
/// snapshot without exactly one target row), Tally's CREATED unreadable (a
/// null `matches_created`), a mark that went backwards, and, defensively, a
/// step whose `matches_created` is absent, which `target_voucher_step` never
/// writes.
#[test]
fn a_step_that_was_never_observed_to_match_is_recorded_as_doubt() {
    for step in [
        Value::Null,
        json!({"before":10,"after":12,"step":2,"reported_created":null,"matches_created":null}),
        json!({"before":10,"after":12,"step":2,"reported_created":2}),
        json!({"before":12,"after":10,"step":null,"reported_created":2,"matches_created":false}),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let server = records_server(directory.path());
        server.record_post_checks_pending("batch-a", true).unwrap();
        server.record_batch_step_verdict("batch-a", &step);
        let imports = server.imports_dir().unwrap();
        let doubt: Value = serde_json::from_slice(
            &fs::read(super::super::batch_step_doubt_path(&imports, "batch-a")).unwrap(),
        )
        .unwrap();
        assert_eq!(doubt["state"], "unmatched", "{step}: {doubt}");
        let recorded =
            server.record_masters_verdict_for("batch-a", json!({"state":"unchanged"}), true);
        assert_eq!(
            post_doubt(Some(&recorded), 2).map(|(code, _)| code),
            Some("batch_step_unconfirmed"),
            "{step}: {recorded}"
        );
    }
}

/// A step doubt, once observed, is never cleared by a later matched verdict.
#[test]
fn an_observed_step_doubt_outlives_a_later_verdict() {
    let directory = tempfile::tempdir().unwrap();
    let server = records_server(directory.path());
    server.record_post_checks_pending("batch-a", true).unwrap();
    server.record_batch_step_verdict("batch-a", &json!({"matches_created":false}));
    server.record_batch_step_verdict("batch-a", &json!({"matches_created":true}));
    let recorded = server.record_masters_verdict_for("batch-a", json!({"state":"unchanged"}), true);
    assert_eq!(
        post_doubt(Some(&recorded), 2).map(|(code, _)| code),
        Some("batch_step_unconfirmed"),
        "{recorded}"
    );
}

/// A batch whose check record cannot be read when its masters verdict is
/// written keeps its step pending, so it stays in doubt; and a batch in
/// doubt, or with no step verdict at all, is never an amendment baseline.
#[test]
fn an_unreadable_batch_record_keeps_the_step_pending_and_blocks_the_baseline() {
    let directory = tempfile::tempdir().unwrap();
    let server = records_server(directory.path());
    let imports = server.imports_dir().unwrap();
    server.record_post_checks_pending("batch-a", true).unwrap();
    fs::write(imports.join("batch-a.masters_check.json"), b"not json").unwrap();
    let recorded = server.record_masters_verdict_for("batch-a", json!({"state":"unchanged"}), true);
    assert_eq!(
        recorded["batch_step"]["state"], "check_pending",
        "{recorded}"
    );
    assert!(post_doubt(Some(&recorded), 2).is_some());

    let baseline = serde_json::to_vec(&super::super::amend::VerifiedBaseline::default()).unwrap();
    for batch in ["batch-a", "batch-b", "batch-c"] {
        fs::write(imports.join(format!("{batch}.baseline.json")), &baseline).unwrap();
    }
    assert!(super::super::read_verified_baseline_for(&imports, "batch-a", 2).is_none());
    // A clean single post is a baseline; the same record read as a batch,
    // holding no step verdict, is not.
    server.record_post_checks_pending("batch-b", false).unwrap();
    server.record_masters_verdict("batch-b", json!({"state":"unchanged"}));
    assert!(super::super::read_verified_baseline_for(&imports, "batch-b", 1).is_some());
    assert!(super::super::read_verified_baseline_for(&imports, "batch-b", 2).is_none());
    // A batch whose masters and step both came out clean is a baseline.
    server.record_post_checks_pending("batch-c", true).unwrap();
    server.record_batch_step_verdict("batch-c", &json!({"matches_created":true}));
    server.record_masters_verdict_for("batch-c", json!({"state":"unchanged"}), true);
    assert!(super::super::read_verified_baseline_for(&imports, "batch-c", 2).is_some());
}

/// Only the rows the readback did not find absent are named, by their ids alone.
#[test]
fn only_present_rows_are_named_and_a_clean_result_names_none() {
    let result = json!({"vouchers":[
        {"bridge_txn_id":"t1","status":"not_found"},
        {"bridge_txn_id":"t2","status":"not_attributable","marker":"accounting_fingerprint"},
        {"bridge_txn_id":"t3","status":"duplicate_fingerprint","matches":2},
        {"bridge_txn_id":"t4","status":"matching_content_observed","marker":"accounting_fingerprint"},
        {"status":"posted_verified"},
    ]});
    assert_eq!(present_txn_ids(&result), ["t2", "t3", "t4"]);
    assert!(
        present_txn_ids(&json!({"vouchers":[{"bridge_txn_id":"t1","status":"not_found"}]}))
            .is_empty()
    );
    assert!(present_txn_ids(&json!({})).is_empty());

    let mut payload = json!({"result":{"error":{"code":"import_preexisting_identity"}}});
    name_preexisting_rows(&mut payload, &[]);
    assert!(payload["result"]["error"].get("next_step").is_none());
    // An unobserved attempt's generic message would say never to rebuild it.
    payload["result"]["error"]["message"] =
        json!("The saved batch requires reconciliation. never rebuild it to retry.");
    name_preexisting_rows(&mut payload, &["t2".into(), "t3".into()]);
    assert!(payload["result"]["error"]["message"]
        .as_str()
        .unwrap()
        .starts_with("Nothing was sent"));
    assert_eq!(
        payload["result"]["error"]["preexisting_txn_ids"],
        json!(["t2", "t3"])
    );
    assert!(payload["result"]["error"]["next_step"]
        .as_str()
        .unwrap()
        .contains("error.preexisting_txn_ids"));
}

/// A refusal of the saved batch's own text says to build it again only when no
/// attempt was recorded (#1055). Where an attempt was, or may have been, made,
/// the message stays the reconciliation one, which says never to rebuild; and
/// any other code with no attempt keeps the general message.
#[test]
fn a_text_refusal_says_to_build_again_only_when_no_attempt_was_recorded() {
    let message = |attempted, code| {
        reconciliation_failure_payload("bridge-test", attempted, None, code)["result"]["error"]
            ["message"]
            .clone()
    };
    assert_eq!(
        message(Some(false), "voucher_text_invalid"),
        json!(TEXT_REFUSED_MESSAGE)
    );
    let reconcile = json!("The saved batch requires reconciliation. Use verify_import with this original batch; never rebuild it to retry.");
    assert_eq!(message(Some(true), "voucher_text_invalid"), reconcile);
    assert_eq!(message(None, "voucher_text_invalid"), reconcile);
    assert_eq!(
        message(Some(false), "voucher_entry_invalid"),
        json!(
            "No posting attempt was recorded. Review the error before requesting approval again."
        )
    );
}
