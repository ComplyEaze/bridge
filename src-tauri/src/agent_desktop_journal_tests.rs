#![allow(
    clippy::disallowed_methods,
    reason = "test doubles: local sockets, servers and processes"
)]
use super::desktop_journal::{DesktopJournalOperation, DesktopJournalService};
use super::*;
use bridge_tally_transport::TallyEndpointConfig;

#[test]
fn action_ipc_keeps_recovery_state_without_unbounded_voucher_details() {
    let large = "x".repeat(5_000_001);
    for state in [
        "posted_verified",
        "previous_attempt_reconciled",
        "reconciliation_required",
    ] {
        let mut payload = json!({"result":{
            "dispatch":{"state":state,"resent":false,"response":{"unused":large}},
            "attempt_recorded":true,
            "unrelated_duplicates_in_window":[large],
        }});
        if state == "reconciliation_required" {
            payload["result"]["error"] = json!({
                "code":"import_reconciliation_required",
                "message":"Reconcile the original batch without resending it.",
                "remediation":large,
            });
        }
        let operation = DesktopJournalOperation::from_outcome(ToolOutcome {
            payload,
            evidence: Evidence {
                request_sha256: "a".repeat(64),
                response_sha256: "b".repeat(64),
                bytes: large.len(),
                state: "complete",
                read_at: None,
                duration_ms: None,
                reason_code: None,
            },
            company_guid: None,
            truncated: false,
        });
        let result = &operation.result["result"];
        assert_eq!(result["dispatch"]["state"], state);
        assert_eq!(result["dispatch"]["resent"], false);
        assert_eq!(result["attempt_recorded"], true);
        assert!(result.get("unrelated_duplicates_in_window").is_none());
        assert!(result["dispatch"].get("response").is_none());
        if state == "reconciliation_required" {
            assert_eq!(result["error"]["code"], "import_reconciliation_required");
            assert_eq!(
                result["error"]["message"],
                "Reconcile the original batch without resending it."
            );
            assert!(result["error"]["remediation"].is_null());
        } else {
            assert!(result["error"].is_null());
        }
        assert!(serde_json::to_vec(&operation.result).unwrap().len() < 1024);
    }
}

#[test]
fn action_ipc_keeps_bounded_response_evidence_and_drops_invalid_metadata() {
    let large = "x".repeat(5_000_001);
    let response = json!({
        "request_sha256": "a".repeat(64),
        "response_sha256": "b".repeat(64),
        "bytes": 538,
        "outcome": bridge_tally_protocol::parse_import_outcome(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/live_education_w4_voucher_sanitized.xml"
        )).unwrap(),
    });
    let mut payload = json!({"result":{
        "dispatch": {"state":"reconciliation_required","resent":false},
        "attempt_recorded":true,
        "dispatch_response":response,
        "proof":large.clone(),
        "vouchers":[large],
    }});
    let operation = DesktopJournalOperation::from_outcome(ToolOutcome {
        payload: payload.clone(),
        evidence: Evidence {
            request_sha256: "a".repeat(64),
            response_sha256: "b".repeat(64),
            bytes: 538,
            state: "partial",
            read_at: None,
            duration_ms: None,
            reason_code: Some("import_ledger_append_failed".into()),
        },
        company_guid: None,
        truncated: false,
    });
    let result = &operation.result["result"];
    assert_eq!(
        result["dispatch_response"]["outcome"]["counters"]["counter_presence"]["deleted"],
        true
    );
    assert_eq!(
        result["dispatch_response"]["request_sha256"],
        "a".repeat(64)
    );
    assert_eq!(
        result["dispatch_response"]["response_sha256"],
        "b".repeat(64)
    );
    assert_eq!(result["dispatch_response"]["bytes"], 538);
    assert_eq!(
        result["dispatch_response"]["outcome"]["application_status"],
        "not_reported"
    );
    assert_eq!(
        result["dispatch_response"]["outcome"]["counters"]["created"],
        1
    );
    assert_eq!(
        result["dispatch_response"]["outcome"]["counters"]["line_error_count"],
        0
    );
    assert!(result.get("proof").is_none());
    assert!(result.get("vouchers").is_none());
    assert!(serde_json::to_vec(&operation.result).unwrap().len() < 2_000);

    let mut null_payload = json!({"result":{"dispatch_response":response.clone()}});
    null_payload["result"]["dispatch_response"]["outcome"] = Value::Null;
    let operation = DesktopJournalOperation::from_outcome(ToolOutcome {
        payload: null_payload,
        evidence: Evidence {
            request_sha256: "a".repeat(64),
            response_sha256: "b".repeat(64),
            bytes: 538,
            state: "partial",
            read_at: None,
            duration_ms: None,
            reason_code: Some("import_ledger_append_failed".into()),
        },
        company_guid: None,
        truncated: false,
    });
    assert_eq!(
        operation.result["result"]["dispatch_response"]["bytes"],
        538
    );
    assert!(operation.result["result"]["dispatch_response"]["outcome"].is_null());

    payload["result"]["dispatch_response"]["request_sha256"] = json!("not-a-sha256");
    let operation = DesktopJournalOperation::from_outcome(ToolOutcome {
        payload,
        evidence: Evidence {
            request_sha256: "a".repeat(64),
            response_sha256: "b".repeat(64),
            bytes: 538,
            state: "partial",
            read_at: None,
            duration_ms: None,
            reason_code: Some("import_ledger_append_failed".into()),
        },
        company_guid: None,
        truncated: false,
    });
    assert!(operation.result["result"]
        .get("dispatch_response")
        .is_none());
}

fn service(root: PathBuf) -> (DesktopJournalService, ImportLedgerLine) {
    service_with_voucher_number(root, None)
}

fn service_with_voucher_number(
    root: PathBuf,
    voucher_number: Option<&str>,
) -> (DesktopJournalService, ImportLedgerLine) {
    service_at_endpoint(
        root,
        voucher_number,
        TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9001,
        },
    )
}

fn service_at_endpoint(
    root: PathBuf,
    voucher_number: Option<&str>,
    endpoint: TallyEndpointConfig,
) -> (DesktopJournalService, ImportLedgerLine) {
    service_recording(root, voucher_number, endpoint, json!([]))
}

/// A saved Journal whose build recorded `on_account` as its bill-wise
/// approvals (#1234); `null` for a record written before the field.
fn service_recording(
    root: PathBuf,
    voucher_number: Option<&str>,
    endpoint: TallyEndpointConfig,
    on_account: Value,
) -> (DesktopJournalService, ImportLedgerLine) {
    service_debiting(root, voucher_number, endpoint, on_account, "Expense")
}

/// A service holding one saved Journal that debits `debit_ledger`.
fn service_debiting(
    root: PathBuf,
    voucher_number: Option<&str>,
    endpoint: TallyEndpointConfig,
    on_account: Value,
    debit_ledger: &str,
) -> (DesktopJournalService, ImportLedgerLine) {
    let mut line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000000001", "identity_scheme":"batch_v1", "company_guid":"00000000-0000-4000-8000-000000000002", "endpoint_origin":super::super::canonical_loopback_origin(&endpoint).unwrap(),
        "company":{"name":"Synthetic Accounts","guid":"00000000-0000-4000-8000-000000000002","company_number":"100001","books_from":"20260401"}, "txn_ids":["journal-test"],"date_from":"20260901","date_to":"20260901","sha256":"","built_at":"2026-09-07T00:00:00Z","status":"built", "on_account_approved":on_account,"pre_import_mark":{"kind":"company_high_water","value":1,"master_value":1},
        "vouchers":[{"bridge_txn_id":"journal-test","date":"20260901","voucher_type":"Journal","voucher_number":voucher_number,"entries":[{"ledger":debit_ledger,"amount":"12.50","side":"Dr"},{"ledger":"Cash","amount":"12.50","side":"Cr"}]}]
    })).unwrap();
    line.sha256 = sha256_hex(
        render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id).as_bytes(),
    );
    if line.on_account_approved.is_none() {
        // A record from before the bill-wise field had the older fields: the
        // post reaches its bill-wise check only past theirs.
        line.ledger_identities = Some(Vec::new());
        line.cash_in_hand_ledgers = Some(Vec::new());
    }
    super::super::ensure_private_directory(&root).unwrap();
    let server = Server::new(super::super::Settings {
        endpoint,
        data_dir: root,
        max_rows: 500,
        max_bytes: 5_000_000,
        redaction: super::super::Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    });
    // Fixture construction exclusively owns this temporary directory. Avoid a
    // setup lock that another parallel test's fork can transiently inherit.
    server.append_import_ledger_while_admitted(&line).unwrap();
    (DesktopJournalService { server }, line)
}

/// #1234: a Journal saved before the build recorded its bill-wise approvals is
/// refused for review in plain words and at the post, and a dispatched one
/// stays reviewable for reconciliation.
#[tokio::test]
async fn a_saved_journal_without_bill_wise_approvals_is_refused_in_plain_words() {
    let directory = tempfile::tempdir().unwrap();
    let (service, line) = service_recording(
        directory.path().join("agent"),
        None,
        TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9001,
        },
        Value::Null,
    );
    assert_eq!(line.on_account_approved, None);
    let xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
    std::fs::write(
        service
            .server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        &xml,
    )
    .unwrap();
    let code = service.review_selected_xml(xml.as_bytes()).unwrap_err();
    assert_eq!(code, "import_batch_predates_bill_wise_record");

    let operation = service
        .post(&line.batch_id, &line.sha256, &line.company_guid)
        .await;
    let error = &operation.result["result"]["error"];
    assert_eq!(error["code"], "import_batch_predates_bill_wise_record");
    assert_eq!(
        error["message"],
        "This batch was built before ComplyEaze Bridge began checking ledgers that keep bills in Tally, so it cannot be checked. Nothing was posted. First check in Tally whether its file was already imported by hand, since posting the rebuilt batch would import it a second time. Then build the batch again and post the new batch."
    );
    assert_eq!(operation.result["result"]["attempt_recorded"], false);

    service
        .server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_native(
            &line,
            "a".repeat(64),
            uuid::Uuid::new_v4(),
        ))
        .unwrap();
    assert!(
        service
            .review_selected_xml(xml.as_bytes())
            .unwrap()
            .dispatched
    );
}

#[tokio::test]
async fn descriptor_company_mismatch_is_refused_before_any_tally_work() {
    let directory = tempfile::tempdir().unwrap();
    let (service, line) = service(directory.path().join("agent"));
    let operation = service
        .post(&line.batch_id, &line.sha256, "other-company")
        .await;
    assert_eq!(
        operation.result["result"]["error"]["code"],
        "import_batch_company_mismatch"
    );
    assert_eq!(
        operation.result["result"]["dispatch"]["state"],
        "admission_refused"
    );
}

/// A Journal saved with a narration the readers would rewrite is refused
/// before approval, with no attempt recorded. Asking for approval again can
/// never succeed, so the message says to build it again (#1055).
#[tokio::test]
async fn a_saved_journal_refused_for_its_text_says_to_build_it_again() {
    let directory = tempfile::tempdir().unwrap();
    let (service, mut line) = service(directory.path().join("agent"));
    line.batch_id = "bridge-00000000-0000-4000-8000-000000000003".into();
    line.vouchers[0].narration = Some("Synthetic \u{fffd}#4; test only".into());
    line.sha256 = sha256_hex(
        render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id).as_bytes(),
    );
    service
        .server
        .append_import_ledger_while_admitted(&line)
        .unwrap();
    let operation = service
        .post(&line.batch_id, &line.sha256, &line.company_guid)
        .await;
    let result = &operation.result["result"];
    assert_eq!(result["error"]["code"], "voucher_text_invalid", "{result}");
    assert_eq!(result["attempt_recorded"], false, "{result}");
    assert_eq!(
        result["error"]["message"],
        "Nothing was sent. This saved batch has a narration, reference or voucher number \
         that cannot be posted, and a saved batch cannot be changed. Correct the text, build \
         the batch again, then post the new one."
    );
}

#[tokio::test]
async fn missing_or_unreadable_history_refuses_admission_without_claiming_no_prior_attempt() {
    for unreadable in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let (service, line) = service(directory.path().join("agent"));
        let journal = service
            .server
            .settings
            .data_dir
            .join("agent-import-ledger.jsonl");
        std::fs::remove_file(&journal).unwrap();
        if unreadable {
            std::fs::create_dir(&journal).unwrap();
        }
        let operation = service
            .post(&line.batch_id, &line.sha256, &line.company_guid)
            .await;
        assert_eq!(
            operation.result["result"]["dispatch"]["state"],
            "admission_refused"
        );
        assert_eq!(
            operation.result["result"]["error"]["code"],
            if unreadable {
                "import_ledger_unavailable"
            } else {
                "import_batch_not_found"
            }
        );
        assert!(operation.result["result"]["attempt_recorded"].is_null());
    }
}

#[tokio::test]
async fn reconcile_without_durable_intent_requires_an_idle_dispatch_lane() {
    let directory = tempfile::tempdir().unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = TallyEndpointConfig {
        host: "127.0.0.1".into(),
        port: listener.local_addr().unwrap().port(),
    };
    let (mut service, line) = service_at_endpoint(directory.path().join("agent"), None, endpoint);
    let journal = service
        .server
        .settings
        .data_dir
        .join("agent-import-ledger.jsonl");
    let before = std::fs::read(&journal).unwrap();
    let lease = dispatch_lease::acquire(&service.server.settings.endpoint).unwrap();
    let contended = service
        .reconcile(&line.batch_id, &line.sha256, &line.company_guid)
        .await;
    assert_eq!(
        contended.result["result"]["error"]["code"],
        "import_admission_busy"
    );
    assert!(contended.result["result"]["attempt_recorded"].is_null());
    assert_eq!(std::fs::read(&journal).unwrap(), before);
    drop(lease);
    let snapshot_lease =
        dispatch_lease::acquire_snapshot(&service.server.settings.endpoint).unwrap();
    let operation = service
        .reconcile(&line.batch_id, &line.sha256, &line.company_guid)
        .await;
    assert_eq!(
        operation.result["result"]["error"]["code"],
        "import_not_dispatched"
    );
    assert_eq!(operation.result["result"]["attempt_recorded"], false);
    drop(snapshot_lease);
    let other_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    service.server.settings.endpoint.port = other_listener.local_addr().unwrap().port();
    let changed = service
        .reconcile(&line.batch_id, &line.sha256, &line.company_guid)
        .await;
    assert_eq!(
        changed.result["result"]["error"]["code"],
        "import_post_endpoint_mismatch"
    );
    assert!(changed.result["result"]["attempt_recorded"].is_null());
    assert_eq!(std::fs::read(&journal).unwrap(), before);
}

#[test]
fn review_details_come_from_the_admitted_saved_journal() {
    let directory = tempfile::tempdir().unwrap();
    let (service, line) = service(directory.path().join("agent"));
    let xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
    std::fs::write(
        service
            .server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        &xml,
    )
    .unwrap();

    let review = service.review_selected_xml(xml.as_bytes()).unwrap();
    assert_eq!(review.details.date, "20260901");
    assert_eq!(review.details.total_debit, "12.5");
    assert_eq!(review.details.total_credit, "12.5");
    assert_eq!(
        review
            .details
            .entries
            .iter()
            .map(|entry| (&entry.ledger, &entry.side, &entry.amount))
            .collect::<Vec<_>>(),
        vec![
            (
                &"Expense".to_string(),
                &"Dr".to_string(),
                &"12.50".to_string()
            ),
            (&"Cash".to_string(), &"Cr".to_string(), &"12.50".to_string())
        ]
    );
}

#[test]
fn review_refuses_selected_xml_from_a_superseded_full_record() {
    let directory = tempfile::tempdir().unwrap();
    let (service, mut line) = service(directory.path().join("agent"));
    let original_xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
    let path = service
        .server
        .imports_dir()
        .unwrap()
        .join(format!("{}.xml", line.batch_id));
    std::fs::write(&path, &original_xml).unwrap();

    // Legacy history accepts changed full records before dispatch. The file
    // still contains the original bytes while the latest saved batch changes.
    line.vouchers[0].narration = Some("Updated Journal".into());
    let latest_xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
    line.sha256 = sha256_hex(latest_xml.as_bytes());
    service
        .server
        .append_import_ledger_while_admitted(&line)
        .unwrap();

    assert_eq!(
        service
            .review_selected_xml(original_xml.as_bytes())
            .unwrap_err(),
        "import_batch_changed"
    );

    std::fs::write(&path, &latest_xml).unwrap();
    let review = service.review_selected_xml(latest_xml.as_bytes()).unwrap();
    assert_eq!(review.sha256, line.sha256);
    assert_eq!(review.details.narration.as_deref(), Some("Updated Journal"));
}

/// The details of a review with one entry marked On Account, as they are
/// serialised for the screen. The screen's own test
/// (`scripts/journal-posting-screen.test.tsx`) reads this text from this file
/// and renders it.
const MARKED_REVIEW_DETAILS_SENT: &str = r#"{"date":"20260901","reference":null,"narration":null,"entries":[{"ledger":"Expense","side":"Dr","amount":"12.50","onAccount":true},{"ledger":"Cash","side":"Cr","amount":"12.50","onAccount":false}],"onAccountNote":"On Account: a bill-wise ledger when this batch was built. Its entries carry no bill allocation.","totalDebit":"12.5","totalCredit":"12.5"}"#;

/// The review screen marks each entry on a ledger the batch records as
/// approved to take entries On Account, and carries the native dialog's own
/// sentence for the mark, as that dialog does next (#1234); a batch that
/// records none carries neither.
#[test]
fn review_marks_each_entry_on_a_ledger_approved_on_account() {
    for (approved, flags, note) in [
        (json!([]), [false, false], None),
        (
            json!([{"ledger":"Expense","party_digest":"a".repeat(64)}]),
            [true, false],
            Some(super::post::ON_ACCOUNT_LEGEND),
        ),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let (service, line) = service_recording(
            directory.path().join("agent"),
            None,
            TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port: 9001,
            },
            approved,
        );
        let xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
        std::fs::write(
            service
                .server
                .imports_dir()
                .unwrap()
                .join(format!("{}.xml", line.batch_id)),
            &xml,
        )
        .unwrap();
        let review = service.review_selected_xml(xml.as_bytes()).unwrap();
        assert_eq!(
            review
                .details
                .entries
                .iter()
                .map(|entry| entry.on_account)
                .collect::<Vec<_>>(),
            flags
        );
        assert_eq!(review.details.on_account_note.as_deref(), note);
        if note.is_some() {
            assert_eq!(
                serde_json::to_string(&review.details).unwrap(),
                MARKED_REVIEW_DETAILS_SENT
            );
        }
    }
}

/// The desktop Journal review screen shows the narration the post sends
/// (#1055, after #1223): without outer whitespace, including spaces HTML would
/// keep such as U+00A0, and one of only spaces reads as absent, since both
/// post an empty narration.
#[test]
fn review_shows_each_narration_as_the_post_sends_it() {
    for (saved, shown) in [
        (Some("  Rent  "), Some("Rent")),
        (Some("\u{a0}Rent"), Some("Rent")),
        (Some("Rent"), Some("Rent")),
        (Some("   "), Some("")),
        (None, None),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let (service, mut line) = service(directory.path().join("agent"));
        line.vouchers[0].narration = saved.map(str::to_owned);
        let xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
        line.sha256 = sha256_hex(xml.as_bytes());
        service
            .server
            .append_import_ledger_while_admitted(&line)
            .unwrap();
        let path = service
            .server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id));
        std::fs::write(&path, &xml).unwrap();
        let review = service.review_selected_xml(xml.as_bytes()).unwrap();
        assert_eq!(review.details.narration.as_deref(), shown, "{saved:?}");
    }
}

#[test]
fn review_refuses_fresh_numbered_journal_but_retains_dispatched_reconciliation() {
    let directory = tempfile::tempdir().unwrap();
    let (service, line) = service_with_voucher_number(directory.path().join("agent"), Some("JV-1"));
    let xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
    std::fs::write(
        service
            .server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        &xml,
    )
    .unwrap();

    assert_eq!(
        service.review_selected_xml(xml.as_bytes()).unwrap_err(),
        "import_post_numbered_journal_unsupported"
    );

    service
        .server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_native(
            &line,
            "a".repeat(64),
            uuid::Uuid::new_v4(),
        ))
        .unwrap();
    let review = service.review_selected_xml(xml.as_bytes()).unwrap();
    assert!(review.dispatched);
}

#[tokio::test]
async fn review_refuses_fresh_unreviewable_text_but_retains_dispatched_reconciliation() {
    let directory = tempfile::tempdir().unwrap();
    // Reserve a port without listening: recovery must reach a read failure,
    // independently of any live Tally instance or reusable free-port race.
    let socket = tokio::net::TcpSocket::new_v4().unwrap();
    socket.bind("127.0.0.1:0".parse().unwrap()).unwrap();
    let (service, mut line) = service_at_endpoint(
        directory.path().join("agent"),
        None,
        TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: socket.local_addr().unwrap().port(),
        },
    );
    std::fs::remove_file(
        service
            .server
            .settings
            .data_dir
            .join("agent-import-ledger.jsonl"),
    )
    .unwrap();
    line.vouchers[0].entries[0].ledger = "Cash\u{200d}".into();
    let xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
    line.sha256 = sha256_hex(xml.as_bytes());
    service
        .server
        .append_import_ledger_while_admitted(&line)
        .unwrap();
    std::fs::write(
        service
            .server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        &xml,
    )
    .unwrap();

    assert_eq!(
        service.review_selected_xml(xml.as_bytes()).unwrap_err(),
        "import_review_format_text"
    );
    service
        .server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_native(
            &line,
            "a".repeat(64),
            uuid::Uuid::new_v4(),
        ))
        .unwrap();
    assert!(
        service
            .review_selected_xml(xml.as_bytes())
            .unwrap()
            .dispatched
    );
    let reconciliation = service
        .reconcile(&line.batch_id, &line.sha256, &line.company_guid)
        .await;
    assert_eq!(
        reconciliation.result["result"]["error"]["code"],
        "import_mode_probe_failed"
    );
}

/// The local desktop, which shows the message alone, names the ledgers a
/// message refers to by field: marked or plain, up to the bound, then counted.
#[test]
fn the_desktop_message_names_the_ledgers_the_result_lists() {
    let operation = |result: Value| {
        DesktopJournalOperation::from_outcome(ToolOutcome {
            payload: json!({ "result": result }),
            evidence: Evidence {
                request_sha256: "a".repeat(64),
                response_sha256: "b".repeat(64),
                bytes: 0,
                state: "complete",
                read_at: None,
                duration_ms: None,
                reason_code: None,
            },
            company_guid: None,
            truncated: false,
        })
        .result["result"]["error"]["message"]
            .as_str()
            .unwrap()
            .to_string()
    };
    let marked = (1..=10)
        .map(|n| serde_json::to_value(party_name(format!("Ledger {n}"))).unwrap())
        .collect::<Vec<_>>();
    let changed_masters = operation(json!({
        "error": {"code": "posted_under_changed_masters", "message": "Posted."},
        "masters_after_post": {"ledgers": marked},
    }));
    assert_eq!(
        changed_masters,
        "Posted. Ledgers: Ledger 1, Ledger 2, Ledger 3, Ledger 4, Ledger 5, Ledger 6, Ledger 7, Ledger 8 and 2 more."
    );
    let refused = operation(json!({
        "error": {
            "code": "import_masters_changed_since_build",
            "message": "Refused.",
            "ledgers_changed": ["Cash"],
            "ledgers_changed_total": 3,
        },
    }));
    assert_eq!(refused, "Refused. Ledgers: Cash and 2 more.");
    let other = operation(json!({
        "error": {"code": "import_reconciliation_required", "message": "Reconcile."},
        "masters_after_post": {"ledgers": ["Cash"]},
    }));
    assert_eq!(other, "Reconcile.");
}

/// bridge#626: the desktop's review takes no saved Journal that names a
/// ledger whose name ends in a line break, before or after it was posted:
/// its screen shows a name as it is.
#[test]
fn a_journal_naming_a_ledger_that_ends_in_a_line_break_is_refused_on_the_desktop() {
    let directory = tempfile::tempdir().unwrap();
    let (service, line) = service_debiting(
        directory.path().join("agent"),
        None,
        TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9001,
        },
        json!([]),
        "Expense\r\n",
    );
    let xml = render_import_xml("Synthetic Accounts", &line.vouchers, &line.batch_id);
    std::fs::write(
        service
            .server
            .imports_dir()
            .unwrap()
            .join(format!("{}.xml", line.batch_id)),
        &xml,
    )
    .unwrap();
    assert_eq!(
        service.review_selected_xml(xml.as_bytes()).unwrap_err(),
        "import_desktop_ledger_line_break"
    );
    service
        .server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_native(
            &line,
            "a".repeat(64),
            uuid::Uuid::new_v4(),
        ))
        .unwrap();
    assert_eq!(
        service.review_selected_xml(xml.as_bytes()).unwrap_err(),
        "import_desktop_ledger_line_break"
    );
}
