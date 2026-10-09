//! The Sales stop and its release (ADR 0004, slice 4), against a journal on
//! disk and the tool call. A release needs a Tally it can read: most tests
//! answer the readback with the captured one (which holds none of these
//! invoices), one reaches no Tally (port 9) and is refused.
use super::*;
use std::sync::Arc;

const ACK: &str = "acknowledge_post_review";

fn server_without_tally(directory: &std::path::Path) -> Server {
    server_at("127.0.0.1:9".parse().unwrap(), directory)
}

/// One invoice batch (or, with `voucher_type`, another type) of the company,
/// saved in the journal.
fn saved(server: &Server, voucher_type: &str) -> ImportLedgerLine {
    let origin =
        crate::tally::connection::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    saved_for(server, voucher_type, &origin)
}

/// As [`saved`], recorded against the Tally endpoint `origin` names.
fn saved_for(server: &Server, voucher_type: &str, origin: &str) -> ImportLedgerLine {
    let id = format!("bridge-{}", Uuid::new_v4());
    let voucher = if voucher_type == "Sales" {
        json!({"bridge_txn_id":"t1","date":"20260907","voucher_type":"Sales",
            "voucher_number":"BP/26-27/0010",
            "invoice":{"voucher_type_name":"Sales Manual","place_of_supply":"Rajasthan"},
            "narration":null,"reference":null,
            "entries":[{"ledger":"Customer","amount":"118.00","side":"Dr"},
                {"ledger":"Sales","amount":"118.00","side":"Cr"}]})
    } else {
        json!({"bridge_txn_id":"t1","date":"20260907","voucher_type":voucher_type,
            "narration":null,"reference":null,"voucher_number":null,
            "entries":[{"ledger":"Cash","amount":"118.00","side":"Dr"},
                {"ledger":"Bank","amount":"118.00","side":"Cr"}]})
    };
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":id, "identity_scheme":"batch_v1", "company_guid":GUID,
        "endpoint_origin":origin,
        "company":{"name":"WR2 Unicode Lab","guid":GUID,"company_number":"100004","books_from":"20260401"},
        "txn_ids":["t1"],"date_from":"20260907","date_to":"20260907",
        "sha256":"a".repeat(64),"built_at":"2026-08-01T00:00:00Z","status":"built",
        "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":7},
        "vouchers":[voucher]
    }))
    .unwrap();
    server.append_import_ledger(&line).unwrap();
    line
}

/// The batch as the post sent it and Tally answered: a dispatch intent and a
/// response on top of `saved`.
fn dispatched(server: &Server, line: &ImportLedgerLine) {
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_native(
            line,
            "c".repeat(64),
            Uuid::new_v4(),
        ))
        .unwrap();
    // Tally answered, and created nothing: the shape of a declined post.
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::response(
            line,
            ledger::DispatchResponse {
                request_sha256: "c".repeat(64),
                ..super::super::tests::dispatch_response("success", 0, 0)
            },
        ))
        .unwrap();
}

fn status(server: &Server, line: &ImportLedgerLine, status: &str) {
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(
            &serde_json::from_value::<ledger::StatusRecord>(json!({
                "record_type":"verification_status","batch_id":line.batch_id,
                "batch_sha256":line.sha256,"status":status
            }))
            .unwrap(),
        )
        .unwrap();
}

fn stop(server: &Server) -> Option<String> {
    server.import_invoice_stop(GUID).unwrap()
}

fn args(line: &ImportLedgerLine) -> Value {
    json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"invoice_stop"})
}

async fn release(server: &Server, args: Value, scripted: ScriptedApproval) -> Value {
    SCRIPTED_APPROVAL
        .scope(scripted, server.call_tool(ACK, args))
        .await
}

/// The release records the import journal holds, as written.
fn releases(server: &Server) -> Vec<String> {
    let _lock = server.lock_import_admission_shared().unwrap();
    let reader = server.import_journal_while_admitted().unwrap().unwrap();
    std::io::BufRead::lines(reader)
        .map(Result::unwrap)
        .filter(|line| line.contains("\"release_invoice_found\""))
        .collect()
}

fn result(response: &Value) -> &Value {
    &response["structuredContent"]["result"]
}

/// A server whose Tally answers `cycles` readbacks with the captured book, in
/// which none of these invoices is found.
fn server_reading(directory: &std::path::Path, cycles: usize) -> (SequenceSimulator, Server) {
    // One readback, and the closing profile probe its verification ends with.
    let plans = (0..cycles)
        .flat_map(|_| reconcile_readback().into_iter().chain(probe()))
        .collect();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let server = server_at(simulator.address(), directory);
    (simulator, server)
}

/// A book that cannot be read is refused, before any dialog: a release made
/// blind would drop a possibly posted invoice from the number control.
#[tokio::test]
async fn a_release_is_refused_while_tally_cannot_be_read() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let line = saved(&server, "Sales");
    dispatched(&server, &line);
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_stop_tally_unreadable",
        "{response}"
    );
    assert!(approval.reviews().is_empty(), "no dialog: {response}");
    assert_eq!(stop(&server), Some(line.batch_id));
}

/// The one way past a stop is a person's click: it appends a release bound to
/// the batch, shows what it is for and what was read, journals that nothing was
/// found, and survives a restart.
#[tokio::test]
async fn a_release_needs_the_person_and_survives_a_restart() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 2);
    let line = saved(&server, "Sales");
    dispatched(&server, &line);
    status(&server, &line, "verification_incomplete");
    assert_eq!(stop(&server), Some(line.batch_id.clone()));

    // Declined: nothing is recorded and the stop stands.
    let declined = ScriptedApproval::declining();
    let response = release(&server, args(&line), declined.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_review_declined",
        "{response}"
    );
    assert_eq!(stop(&server), Some(line.batch_id.clone()));

    // Approved: the dialog says what is released, names the batch and what
    // was read; the release is journaled.
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "stop_released", "{response}");
    assert_eq!(result(&response)["invoice_found_at_release"], false);
    assert_eq!(approval.review_counts(), [1]);
    let shown = &approval.reviews()[0];
    assert!(shown.contains("RELEASES THE INVOICE STOP"), "{shown}");
    assert!(shown.contains(&line.batch_id), "{shown}");
    assert!(shown.contains("not in Tally's book"), "{shown}");
    assert!(approval.previews().is_empty(), "not a post dialog");
    assert_eq!(stop(&server), None);
    // What the read found is journaled: nothing found, so the invoice is no
    // longer counted as sent (a found one would be the control instead).
    assert_eq!(
        server
            .import_invoice_number_control(GUID, ("20260401", "20270331"))
            .unwrap(),
        ledger::NumberControl::NeverSent
    );

    // A new server over the same directory reads the same journal.
    let restarted = server_without_tally(directory.path());
    assert_eq!(stop(&restarted), None);

    // Nothing is left to release.
    let again = release(&restarted, args(&line), ScriptedApproval::approving()).await;
    assert_eq!(
        result(&again)["error"]["code"],
        "ack_stop_not_held",
        "{again}"
    );
}

/// A release records the invoice as absent only when every row reads a state
/// that means absent; every other state the verification can give (including
/// the ones it rewrites `not_found` into after a native post) reads as found,
/// as do no rows and duplicates.
#[test]
fn a_release_records_absent_only_for_states_that_mean_absent() {
    use super::super::super::stop::Seen;
    let result = |statuses: &[&str]| {
        json!({"vouchers": statuses.iter().map(|status| json!({"status": status})).collect::<Vec<_>>(),
            "duplicates": []})
    };
    // A native post that is gone can never read `not_found`: it reads one of
    // the two states a verification gives a posted voucher the window does not
    // hold, and a release must be able to record that as absent.
    for absent in [
        "not_found",
        "tally_reported_not_created",
        "bound_not_in_window",
        "book_rolled_back",
    ] {
        assert!(!Seen::of(&result(&[absent])).found(), "{absent}");
    }
    assert!(!Seen::of(&result(&["bound_not_in_window", "not_found"])).found());
    for present in [
        "posted_verified",
        "posted_divergent",
        "posted_not_effective",
        "matching_content_observed",
        "not_attributable",
        "duplicate_fingerprint",
        "cancelled_with_effective_copy",
        "sent_not_attributed",
        "a_state_not_yet_invented",
    ] {
        assert!(Seen::of(&result(&[present])).found(), "{present}");
        // One present row among absent ones is still found.
        assert!(
            Seen::of(&result(&["not_found", present])).found(),
            "{present}"
        );
    }
    assert!(Seen::of(&json!({})).found(), "no rows");
    assert!(Seen::of(&result(&[])).found(), "an empty row list");
    let mut duplicated = result(&["not_found"]);
    duplicated["duplicates"] = json!([["a", "b"]]);
    assert!(Seen::of(&duplicated).found(), "duplicates");
}

/// Only the failures no wait cures let a release go ahead of its readback;
/// every other, and any code not listed, refuses it.
#[test]
fn a_release_goes_ahead_of_a_failed_read_only_for_the_incurable_ones() {
    use super::super::super::stop::failure_waiting_cannot_cure;
    for code in [
        "import_post_endpoint_mismatch",
        "company_identity_mismatch",
        "host_setting_invalid",
        "verification_mode_unqualified",
        "verification_too_large_to_report",
    ] {
        assert!(
            failure_waiting_cannot_cure(&ToolFailure::from(code.to_string())),
            "{code}"
        );
    }
    for code in [
        "import_mode_probe_failed",
        "tally_endpoint_busy",
        "company_identity_not_found",
        "request_cancelled",
        "import_batch_company_mismatch",
        "a_code_not_yet_invented",
    ] {
        assert!(
            !failure_waiting_cannot_cure(&ToolFailure::from(code.to_string())),
            "{code}"
        );
    }
}

/// The dialog shows names from the journal and Tally with anything that could
/// reorder or hide text, and any line break, as `?`, and still shows them.
#[test]
fn the_release_dialog_does_not_show_text_that_could_mislead() {
    use super::super::super::stop::{release_preview, Seen};
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let line = saved(&server, "Sales");
    let preview = release_preview(
        &line,
        "Acme\u{202e}Ltd\nPaid",
        &Seen::NotFound,
        ledger::InvoiceHold::Stopping,
    );
    assert!(preview.contains("Acme?Ltd?Paid"), "{preview}");
    assert!(!preview.contains('\u{202e}'));
    assert!(preview.contains(&line.batch_id));
    assert!(preview.contains("BP/26-27/0010"));
}

/// What is not a stop cannot be released: a batch never sent, one holding no
/// invoice, one that reads verified, and no other doubt kind ever selects it.
#[tokio::test]
async fn only_a_batch_that_stops_the_company_can_be_released() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let built = saved(&server, "Sales");
    let journal = saved(&server, "Journal");
    dispatched(&server, &journal);
    let verified = saved(&server, "Sales");
    dispatched(&server, &verified);
    status(&server, &verified, "posted_verified");
    for (line, code) in [
        (&built, "ack_stop_batch_not_sent"),
        (&journal, "ack_stop_batch_not_sent"),
        (&verified, "ack_stop_not_held"),
    ] {
        let approval = ScriptedApproval::approving();
        let response = release(&server, args(line), approval.clone()).await;
        assert_eq!(result(&response)["error"]["code"], code, "{response}");
        assert!(approval.reviews().is_empty(), "no dialog: {response}");
    }

    // A stop is never released by an acknowledgement that names no doubt, or
    // another one: the review path does not know it.
    let stopped = saved(&server, "Sales");
    dispatched(&server, &stopped);
    for doubt in [None, Some("masters"), Some("batch_step")] {
        let mut named = json!({"company_guid":GUID,"batch_id":stopped.batch_id});
        if let Some(doubt) = doubt {
            named["doubt"] = json!(doubt);
        }
        let approval = ScriptedApproval::approving();
        let response = release(&server, named, approval.clone()).await;
        assert!(
            result(&response)["error"].is_object(),
            "{doubt:?}: {response}"
        );
        assert!(approval.reviews().is_empty(), "{doubt:?}: {response}");
    }
    assert_eq!(stop(&server), Some(stopped.batch_id));
}

/// What the person was shown must still hold when the release is recorded: a
/// batch that reads verified while the dialog is open, or a stop that moved,
/// is refused and nothing is written.
#[tokio::test]
async fn a_stop_that_changes_while_the_dialog_is_open_is_not_released() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 1);
    let server = Arc::new(server);
    let line = saved(&server, "Sales");
    dispatched(&server, &line);
    let (other, other_line) = (Arc::clone(&server), line.clone());
    let approval = ScriptedApproval::approving_after(move || {
        // A verification reads the batch posted while the person looks.
        status(&other, &other_line, "posted_verified");
    });
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_stop_changed_while_reviewing",
        "{response}"
    );
    // Nothing was appended by the release: the stop is gone only because the
    // batch verified, and a later unverified read stops the company again.
    assert_eq!(stop(&server), None);
    status(&server, &line, "verification_incomplete");
    assert_eq!(stop(&server), Some(line.batch_id));
}

/// The under-lock check a post makes judges only an invoice: a stopped company
/// stops no Journal, Payment, Receipt or Contra.
#[test]
fn the_stop_judges_only_an_invoice_post() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let sent = saved(&server, "Sales");
    dispatched(&server, &sent);
    let invoice = saved(&server, "Sales");
    let payment = saved(&server, "Payment");
    assert!(server.stopped_company_while_admitted(&invoice).unwrap());
    assert!(!server.stopped_company_while_admitted(&payment).unwrap());
}

/// The post checks the stop in the closure that records its dispatch intent,
/// under the admission lock: no Sales post runs end to end before Sales is
/// qualified, so this pins the call where it stands, between the duplicate
/// check and the spending of the approval, for an invoice only.
#[test]
fn the_post_asks_the_stop_under_its_lock_before_it_spends_the_approval() {
    let source = include_str!("agent_import_post.rs");
    let closure = &source[source
        .find("UnderLockRefusal::TxnAlreadyPosted,")
        .expect("the duplicate check of the before-dispatch closure")..];
    let asked = closure
        .find(".stopped_company_while_admitted(&line)")
        .expect("the closure asks the stop");
    let refused = closure
        .find("UnderLockRefusal::CompanyStopped")
        .expect("and refuses it by its own code");
    let spent = closure
        .find(".spend(batch_id, id)")
        .expect("spends the approval");
    let intent = closure
        .find("ledger::StatusRecord::dispatch_for(")
        .expect("appends the intent");
    assert!(asked < refused && refused < spent && spent < intent);
}

/// The build asks the stop again under its admission lock, before it checks
/// the rows already posted and writes a file, for an invoice batch only. No
/// Sales build runs end to end before Sales is qualified, so this pins the
/// call where it stands.
#[test]
fn the_build_asks_the_stop_under_its_lock_before_it_writes_a_file() {
    let source = include_str!("agent_import.rs");
    let build = &source[source
        .find("let _admission_lock = self.lock_import_admission()?;\n            // Admit the journal before publication")
        .expect("the build's exclusive admission lock")..];
    let asked = build
        .find(".import_invoice_stop_while_admitted(&payload.company_guid)")
        .expect("the build asks the stop");
    let invoice_only = build[..asked]
        .rfind("voucher.voucher_type.is_invoice()")
        .expect("and only for an invoice batch");
    let refused = build
        .find("\"invoice_company_stopped\"")
        .expect("and refuses it by its own code");
    let lineage = build
        .find("let lineage = match payload.amends_batch_id")
        .expect("before the lineage and the written file");
    assert!(invoice_only < asked && asked < refused && refused < lineage);
}

/// A post refused for a stopped company names its reason and what to do, and
/// is the refusal of a post that sent nothing.
#[test]
fn a_post_refused_for_a_stopped_company_says_what_to_do() {
    let payload = super::super::reconciliation_failure_payload(
        "bridge-00000000-0000-4000-8000-000000000001",
        Some(false),
        None,
        "invoice_company_stopped",
    );
    let error = &payload["result"]["error"];
    assert_eq!(error["code"], "invoice_company_stopped");
    assert!(error["message"]
        .as_str()
        .unwrap()
        .starts_with("Nothing was sent"));
    assert!(error["next_step"]
        .as_str()
        .unwrap()
        .contains("invoice_stop"));
}

/// A failed read that no wait cures (the batch was sent to another endpoint
/// than the one now set) must not leave the company stopped for good: the
/// release goes through, says the read failed, and is journaled as the
/// invoice possibly being in the book, so it stays the number control.
#[tokio::test]
async fn a_release_is_not_refused_for_a_failure_waiting_cannot_cure() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let line = saved_for(&server, "Sales", "http://127.0.0.1:9000");
    dispatched(&server, &line);
    assert_eq!(stop(&server), Some(line.batch_id.clone()));
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "stop_released", "{response}");
    assert_eq!(result(&response)["invoice_found_at_release"], true);
    let shown = &approval.reviews()[0];
    assert!(shown.contains("Tally could not be read"), "{shown}");
    assert_eq!(stop(&server), None);
    assert_eq!(
        server
            .import_invoice_number_control(GUID, ("20260401", "20270331"))
            .unwrap(),
        ledger::NumberControl::Known {
            batch_id: line.batch_id.clone(),
            number: "BP/26-27/0010".to_string(),
            date: "20260907".to_string()
        }
    );
}

/// The company the caller names is the batch's own: a wrong one is refused
/// before any read or dialog, and is no way onto the path a failed read takes.
#[tokio::test]
async fn a_release_naming_another_company_is_refused_before_any_read() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let line = saved(&server, "Sales");
    dispatched(&server, &line);
    let mut wrong = args(&line);
    wrong["company_guid"] = json!("00000000-0000-4000-8000-0000000000ff");
    let approval = ScriptedApproval::approving();
    let response = release(&server, wrong, approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "import_batch_company_mismatch",
        "{response}"
    );
    assert!(approval.reviews().is_empty(), "no dialog: {response}");
    assert_eq!(stop(&server), Some(line.batch_id));
}

/// After a native post whose readback cannot attribute the voucher to its
/// post, the verification reads the unmatched voucher as sent but not
/// attributed, which says it may be in the book: the release records it as
/// found, and the number control keeps it.
#[tokio::test]
async fn a_release_after_a_native_post_that_reads_not_attributed_records_found() {
    let directory = tempfile::tempdir().unwrap();
    let plans = posted_readback().into_iter().chain(probe()).collect();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = saved(&server, "Sales");
    let native = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &line,
                &native,
                Some(8),
            ))
            .unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::response(
                &line,
                ledger::DispatchResponse {
                    request_sha256: native.request_sha256.clone(),
                    ..super::super::tests::dispatch_response("success", 1, 0)
                },
            ))
            .unwrap();
    }
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "stop_released", "{response}");
    assert_eq!(
        result(&response)["invoice_found_at_release"],
        true,
        "{response}"
    );
    assert!(
        approval.reviews()[0].contains("found in Tally"),
        "{:?}",
        approval.reviews()
    );
    assert_eq!(
        server
            .import_invoice_number_control(GUID, ("20260401", "20270331"))
            .unwrap(),
        ledger::NumberControl::Known {
            batch_id: line.batch_id.clone(),
            number: "BP/26-27/0010".to_string(),
            date: "20260907".to_string()
        }
    );
}

/// A batch released as found is still the number control, and the control read
/// cannot find an invoice that is gone. Released again, it is dropped from the
/// control only when a fresh read shows the invoice gone, the dialog says what
/// that changes, and the record is the existing release record.
#[tokio::test]
async fn a_batch_released_as_found_is_dropped_once_a_read_shows_it_gone() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 2);
    let line = saved(&server, "Sales");
    dispatched(&server, &line);
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::stop_release(&line, true))
            .unwrap();
    }
    // Released as found: it stops nothing and is the control.
    assert_eq!(stop(&server), None);
    let control = |server: &Server| {
        server
            .import_invoice_number_control(GUID, ("20260401", "20270331"))
            .unwrap()
    };
    assert_eq!(
        control(&server),
        ledger::NumberControl::Known {
            batch_id: line.batch_id.clone(),
            number: "BP/26-27/0010".to_string(),
            date: "20260907".to_string()
        }
    );
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "control_dropped", "{response}");
    assert_eq!(result(&response)["invoice_found_at_release"], false);
    let shown = &approval.reviews()[0];
    assert!(shown.contains("STOPS USING AN INVOICE"), "{shown}");
    assert!(shown.contains("first invoice"), "{shown}");
    assert_eq!(control(&server), ledger::NumberControl::NeverSent);
    // Once dropped it is neither a stop nor a control, and is not released
    // a third time.
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["error"]["code"], "ack_stop_not_held");
    assert!(approval.reviews().is_empty());
}

/// A release made as found stands when a fresh read still does not show the
/// invoice gone: nothing is asked, nothing is written.
#[tokio::test]
async fn a_release_as_found_stands_while_the_read_does_not_show_the_invoice_gone() {
    let directory = tempfile::tempdir().unwrap();
    let plans = posted_readback()
        .into_iter()
        .chain(probe())
        // The profile is probed once per server: the second read has no opening probe.
        .chain(posted_readback().into_iter().skip(probe().len()))
        .chain(probe())
        .collect();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = saved(&server, "Sales");
    let native = native_post_request(&line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &line,
                &native,
                Some(8),
            ))
            .unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::response(
                &line,
                ledger::DispatchResponse {
                    request_sha256: native.request_sha256.clone(),
                    ..super::super::tests::dispatch_response("success", 1, 0)
                },
            ))
            .unwrap();
    }
    let approval = ScriptedApproval::approving();
    let first = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&first)["state"], "stop_released", "{first}");
    assert_eq!(result(&first)["invoice_found_at_release"], true);
    let releases_before = releases(&server);
    assert_eq!(
        releases_before.len(),
        1,
        "the first release is on the journal"
    );
    let second_approval = ScriptedApproval::approving();
    let second = release(&server, args(&line), second_approval.clone()).await;
    assert_eq!(
        result(&second)["error"]["code"],
        "ack_stop_release_stands",
        "{second}"
    );
    assert!(second_approval.reviews().is_empty(), "no dialog: {second}");
    // A read that does not show the invoice gone, or that fails for a cause no
    // wait cures, leaves the release as it stands and records no release (the
    // read itself still saves its verification, as any read of a batch does).
    assert_eq!(releases(&server), releases_before, "no release: {second}");
    assert!(matches!(
        server
            .import_invoice_number_control(GUID, ("20260401", "20270331"))
            .unwrap(),
        ledger::NumberControl::Known { .. }
    ));
}

/// The refusal for a control the number read did not find names the control's
/// batch, which is what a person releases. No Sales build runs end to end
/// before Sales is qualified, so this pins the value where it is built.
#[test]
fn the_missing_control_refusal_names_the_control_batch() {
    // Whitespace folded, so a reformat of the call does not break the pin.
    let source = include_str!("agent_import_invoice.rs")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let known = source
        .find("super::ledger::NumberControl::Known { batch_id,")
        .expect("the control the admission reads against, with its batch taken");
    assert!(
        source[known..].contains("\"invoice_number_control_missing\", &batch_id"),
        "the refusal's value is the control's batch"
    );
}

/// An invoice reaches Tally through `post_import` only. The build refuses one
/// that has no native route before it reads anything (posting off) and before
/// it writes a file (a saved invoice `post_import` would refuse). No Sales build
/// runs end to end before Sales is qualified, so both are pinned by position.
#[test]
fn an_invoice_without_a_native_post_route_is_refused_before_a_read_and_before_a_file() {
    let source = include_str!("agent_import.rs");
    let build = &source[source
        .find("pub(super) async fn build_import_xml(")
        .expect("the build")..];
    let unqualified = build
        .find("refuse_unqualified_types(&payload.vouchers")
        .expect("the type check comes first");
    let first_read = build
        .find("self.qualified_import_profile().await")
        .expect("the first read of Tally");
    let off = build
        .find("&& !self.settings.writes_enabled")
        .expect("posting off refuses an invoice");
    // Only an invoice: the guard is on the voucher type, in front of the setting.
    let invoice_only = build[..off]
        .rfind(".any(|voucher| voucher.voucher_type.is_invoice())")
        .expect("and only for an invoice");
    let refused = build[off..]
        .find("INVOICE_POST_NOT_ENABLED")
        .expect("by its own code");
    assert!(unqualified < invoice_only && invoice_only < off && off + refused < first_read);

    // The saved invoice is judged by the post's own admission and refused with
    // the code the post would give, before the file is written.
    let eligible = build
        .find("post::admit_saved_voucher(")
        .expect("the saved invoice is judged by the post's own admission");
    let invoice_batch = build[..eligible]
        .rfind("if holds_an_invoice(&line)")
        .expect("for an invoice batch only");
    let file = build
        .find("persistence::persist_build(")
        .expect("the file is written");
    let refusal = build[eligible..]
        .find("return Err(invoice_route_refusal(code).into())")
        .expect("with the post's own code");
    assert!(invoice_batch < eligible && eligible + refusal < file);

    // The guidance of the saved invoice is the posting route only, whichever
    // branch the other batch kinds would have taken.
    let guidance = build
        .find("let (mut warnings, next_step) = build_import_guidance(")
        .expect("the shared guidance");
    let invoice = build[guidance..]
        .find("invoice_guidance(&mut warnings)")
        .expect("replaced for an invoice batch");
    let amendment = build[guidance..]
        .find("let next_step = match &amendment")
        .expect("before the amendment's own text");
    assert!(invoice < amendment);
}

/// The guidance of a saved invoice names the posting route and never a hand
/// import; it replaces the first warning the other batch kinds share.
#[test]
fn the_guidance_of_a_saved_invoice_names_the_posting_route_only() {
    use super::super::super::{invoice_guidance, INVOICE_POST_ONLY_WARNING};
    let mut warnings = json!(["generic", "second"]);
    let next_step = invoice_guidance(&mut warnings);
    assert_eq!(warnings[0], INVOICE_POST_ONLY_WARNING);
    assert_eq!(warnings[1], "second");
    assert!(next_step.contains("post_import"));
    for text in [next_step, INVOICE_POST_ONLY_WARNING] {
        assert!(
            !text.contains("verify_import right after importing"),
            "{text}"
        );
        assert!(!text.contains("Gateway of Tally"), "{text}");
    }
    assert!(INVOICE_POST_ONLY_WARNING.contains("Do not import the written file in Tally by hand"));
}

/// After a declined invoice is released, the texts send it again under the
/// same number, and never also tell the person to enter it by hand.
#[test]
fn the_already_posted_texts_send_a_declined_invoice_again_under_its_number() {
    for text in [
        super::super::super::BUILD_TXN_ALREADY_POSTED_NEXT_STEP,
        super::super::TXN_ALREADY_POSTED_NEXT_STEP,
    ] {
        assert!(text.contains("under a new bridge_txn_id and the SAME invoice number"));
        assert!(text.contains("Never also tell the user to enter that invoice in Tally by hand"));
        // The hand-entry advice stays, for any other voucher.
        assert!(text.contains("For any other voucher: if Tally rejected that batch"));
    }
    // The post's text no longer forbids the one rebuild it now advises.
    let post = super::super::TXN_ALREADY_POSTED_NEXT_STEP;
    assert!(post.contains(
        "Never rebuild a row to retry it, except an invoice released as described above."
    ));
}

/// The approval-text refusals of an invoice build carry the invoice's own
/// codes, with advice about an invoice; the post's codes, which also refuse a
/// Journal, Payment, Receipt or Contra, carry no invoice wording.
#[test]
fn the_approval_text_refusals_of_an_invoice_build_do_not_reach_other_vouchers() {
    use super::super::super::invoice_route_refusal;
    for (post_code, invoice_code) in [
        ("import_review_layout_text", "invoice_review_layout_text"),
        ("import_review_format_text", "invoice_review_format_text"),
        ("import_review_too_large", "invoice_review_too_large"),
    ] {
        assert_eq!(invoice_route_refusal(post_code.to_string()), invoice_code);
        let advice = crate::agent::refusal_remediation(invoice_code)
            .unwrap_or_else(|| panic!("{invoice_code} has advice"));
        assert!(advice.contains("invoice"), "{advice}");
        assert!(advice.contains("no file was written"), "{advice}");
        // What a Journal, Payment, Receipt or Contra post is refused with.
        assert!(
            crate::agent::refusal_remediation(post_code)
                .is_none_or(|text| !text.contains("invoice")),
            "{post_code}"
        );
    }
    // Any other code passes through as the post's own.
    for code in [
        "import_post_requires_one_voucher",
        "import_invoice_not_observed",
    ] {
        assert_eq!(invoice_route_refusal(code.to_string()), code);
    }
}
