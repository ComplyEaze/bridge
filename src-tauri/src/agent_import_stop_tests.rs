//! The Sales stop and its release (ADR 0004, slice 4), against a journal on
//! disk and the tool call. Every batch is sent as a production post sends it:
//! a native dispatch intent with its pre-POST mark. Most tests answer the
//! release's read with the captured book, which cannot attribute the invoice
//! to its post; others reach no Tally (port 9), or a book too large to read.
use super::super::super::stop::Seen;
use super::*;
use crate::agent::agent_import::tests::{sentinel_batch_id, sentinel_uuid};
use std::sync::Arc;
use tally_protocol_simulator::DeliveryGate;

const ACK: &str = "acknowledge_post_review";

fn server_without_tally(directory: &std::path::Path) -> Server {
    server_at("127.0.0.1:9".parse().unwrap(), directory)
}

/// One invoice batch (or, with `voucher_type`, another type) of the company,
/// as recorded against the Tally endpoint `origin` names.
fn batch(voucher_type: &str, origin: &str) -> ImportLedgerLine {
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
    serde_json::from_value(json!({
        "batch_id":id, "identity_scheme":"batch_v1", "company_guid":GUID,
        "endpoint_origin":origin,
        "company":{"name":"WR2 Unicode Lab","guid":GUID,"company_number":"100004","books_from":"20260401"},
        "txn_ids":["t1"],"date_from":"20260907","date_to":"20260907",
        "sha256":"a".repeat(64),"built_at":"2026-08-01T00:00:00Z","status":"built",
        "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":7},
        "vouchers":[voucher]
    }))
    .unwrap()
}

/// [`batch`] for the server's own endpoint, saved in the journal.
fn saved(server: &Server, voucher_type: &str) -> ImportLedgerLine {
    let origin =
        crate::tally::connection::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let line = batch(voucher_type, &origin);
    server.append_import_ledger(&line).unwrap();
    line
}

/// The batch as a production post sends it and Tally answered: a native
/// dispatch intent carrying the pre-POST mark, and an answer that created
/// the voucher.
fn posted(server: &Server, line: &ImportLedgerLine) {
    let native = native_post_request(line, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
            line,
            &native,
            Some(8),
        ))
        .unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::response(
            line,
            ledger::DispatchResponse {
                request_sha256: native.request_sha256.clone(),
                ..super::super::tests::dispatch_response("success", 1, 0)
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

fn control(server: &Server) -> ledger::NumberControl {
    server
        .import_invoice_number_control(GUID, ("20260401", "20270331"))
        .unwrap()
}

fn args(line: &ImportLedgerLine) -> Value {
    json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"invoice_stop"})
}

async fn release(server: &Server, args: Value, scripted: ScriptedApproval) -> Value {
    SCRIPTED_APPROVAL
        .scope(scripted, server.call_tool(ACK, args))
        .await
}

fn result(response: &Value) -> &Value {
    &response["structuredContent"]["result"]
}

/// The whole dialog for the test batch `line` of "WR2 Unicode Lab", saying
/// `seen` of its invoice.
fn dialog(line: &ImportLedgerLine, seen: &str) -> String {
    format!(
        "THIS RELEASES THE INVOICE STOP FOR THE COMPANY.\n\n\
         Company: WR2 Unicode Lab\n\
         ComplyEaze Bridge sent this invoice to Tally and cannot confirm it as posted:\n  \
         BP/26-27/0010 dated 20260907\n\
         Batch: {}\n\
         What ComplyEaze Bridge saw just now: {seen}.\n\n\
         Click the button only if you have checked this invoice in Tally yourself. \
         ComplyEaze Bridge will then build and post invoices for this company again. \
         This changes nothing in Tally.",
        line.batch_id
    )
}

/// The journal's stop release records, as written.
fn release_records(server: &Server) -> Vec<Value> {
    fs::read_to_string(server.settings.data_dir.join("agent-import-ledger.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == "stop_release")
        .collect()
}

/// A server whose Tally answers `cycles` release reads with the captured book
/// after a native post, which cannot attribute the invoice to it.
fn server_reading(directory: &std::path::Path, cycles: usize) -> (SequenceSimulator, Server) {
    let plans = (0..cycles).flat_map(|_| posted_readback()).collect();
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let server = server_at(simulator.address(), directory);
    (simulator, server)
}

const NOT_ATTRIBUTED: &str = "not established: no voucher matching this invoice was found, and one may be in the book, changed or entered in Tally";

/// The one way past a stop is a person's click: it appends a release bound to
/// the batch, shows what it is for and what was read, and survives a restart.
/// After a native post the read cannot attribute the invoice, so it is not
/// established, the release records that, and the released invoice is not the
/// number control.
#[tokio::test]
async fn a_release_needs_the_person_and_survives_a_restart() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 2);
    let line = saved(&server, "Sales");
    posted(&server, &line);
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
    assert_eq!(release_records(&server), Vec::<Value>::new());

    // Approved: the dialog says what is released, names the batch and what
    // was read; the release is journaled with it.
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "stop_released", "{response}");
    assert_eq!(result(&response)["seen_at_release"], "not_established");
    assert_eq!(result(&response)["seen_detail"], "sent_not_attributed");
    assert_eq!(approval.review_counts(), [1]);
    // Asked as the release, not as a review: its dialog says what Y does to the
    // stop, not that a voucher was reviewed (#1780).
    assert_eq!(
        approval.review_kinds(),
        [crate::tally::approved_import::ReviewKind::StopRelease]
    );
    assert_eq!(approval.reviews(), [dialog(&line, NOT_ATTRIBUTED)]);
    assert!(approval.previews().is_empty(), "not a post dialog");
    assert_eq!(stop(&server), None);
    let records = release_records(&server);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["release_seen"], "not_established");
    assert_eq!(records[0]["batch_sha256"], line.sha256);
    assert_eq!(control(&server), ledger::NumberControl::NoneStanding);

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

/// A release whose read cannot reach Tally goes ahead: the dialog says the
/// read did not complete, by its code and that Tally did not answer, and
/// claims nothing about the invoice.
#[tokio::test]
async fn a_release_goes_ahead_while_tally_cannot_be_read() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let line = saved(&server, "Sales");
    posted(&server, &line);
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "stop_released", "{response}");
    assert_eq!(result(&response)["seen_at_release"], "unreadable");
    assert_eq!(result(&response)["seen_detail"], "import_mode_probe_failed");
    assert_eq!(
        approval.reviews(),
        [dialog(
            &line,
            "ComplyEaze Bridge could not complete its read (import_mode_probe_failed: Tally did not answer), so it reports nothing about this invoice"
        )]
    );
    assert_eq!(release_records(&server)[0]["release_seen"], "unreadable");
    assert_eq!(stop(&server), None);
    assert_eq!(control(&server), ledger::NumberControl::NoneStanding);
}

/// A failure that repeats on every retry (a book whose voucher mark needs more
/// census requests than a read may send) does not leave the company stopped
/// for good: the release goes ahead with the code shown.
#[tokio::test]
async fn a_release_on_a_failure_that_repeats_goes_ahead_with_its_code() {
    let directory = tempfile::tempdir().unwrap();
    let mut plans = probe();
    plans.extend(verified_company());
    plans.extend(paired(marks_at(1_000_000_000)));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = saved(&server, "Sales");
    posted(&server, &line);
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "stop_released", "{response}");
    assert_eq!(result(&response)["seen_at_release"], "unreadable");
    assert_eq!(
        result(&response)["seen_detail"],
        "voucher_window_book_too_large"
    );
    assert_eq!(
        approval.reviews(),
        [dialog(
            &line,
            "ComplyEaze Bridge could not complete its read (voucher_window_book_too_large), so it reports nothing about this invoice"
        )]
    );
    assert_eq!(stop(&server), None);
}

/// A call withdrawn by its caller is refused before any dialog: no one is
/// there to answer it, and nothing is recorded.
#[tokio::test]
async fn a_cancelled_release_is_refused_with_no_dialog() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let line = saved(&server, "Sales");
    posted(&server, &line);
    let approval = ScriptedApproval::approving();
    let withdrawn = tokio_util::sync::CancellationToken::new();
    withdrawn.cancel();
    let response = crate::tally::runtime::TOOL_CANCELLATION
        .scope(withdrawn, release(&server, args(&line), approval.clone()))
        .await;
    assert_eq!(
        result(&response)["error"]["code"],
        "request_cancelled",
        "{response}"
    );
    assert!(approval.reviews().is_empty(), "no dialog: {response}");
    assert_eq!(release_records(&server), Vec::<Value>::new());
    assert_eq!(stop(&server), Some(line.batch_id));
}

/// The stop can lift while the release reads: here another verification
/// records the batch verified while the read's last answer is held, so the
/// release's own read then fails to record (the journal moved under it) and
/// the stop is found gone. Nothing is asked or recorded, and the result says
/// so. The last answer is held until that record is written, not for a set
/// time: with a timer, a slow runner let the record's write still hold the
/// journal's lock when the release went on, and the release answered
/// `import_admission_busy` instead.
#[tokio::test]
async fn a_release_whose_read_finds_the_stop_gone_asks_nothing() {
    let directory = tempfile::tempdir().unwrap();
    let mut plans = posted_readback();
    let held = plans.len();
    let gate = DeliveryGate::new();
    let last = plans
        .pop()
        .unwrap()
        .with_delivery(Delivery::Held(gate.clone()));
    plans.push(last);
    let simulator = Arc::new(SequenceSimulator::spawn(with_sentinel(plans)).unwrap());
    let server = Arc::new(server_at(simulator.address(), directory.path()));
    let line = saved(&server, "Sales");
    posted(&server, &line);
    let (watched, other, other_line) = (Arc::clone(&simulator), Arc::clone(&server), line.clone());
    let verifier = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + Duration::from_secs(30);
        while watched.received() < held {
            assert!(
                std::time::Instant::now() < deadline,
                "the read's last request never arrived"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
        status(&other, &other_line, "posted_verified");
        // Only now, with the record written and its lock released, does the
        // read's last answer go.
        gate.open();
    });
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    verifier.join().unwrap();
    assert_eq!(result(&response)["state"], "not_held", "{response}");
    assert_eq!(result(&response)["seen_at_release"], Value::Null);
    assert_eq!(result(&response)["seen_detail"], Value::Null);
    assert!(approval.reviews().is_empty(), "no dialog: {response}");
    assert_eq!(release_records(&server), Vec::<Value>::new());
}

/// What a read result says of the batch's invoice, by its row's status and
/// reason: every status the verification declares (its counts, less the one
/// count that is no status) and every status its rewrites write, so a status
/// added later fails here until it is placed. Duplicates change no state.
#[test]
fn what_was_seen_is_placed_for_every_status_the_verification_writes() {
    use super::super::super::verification;
    let line = batch("Sales", "http://127.0.0.1:9000");
    let empty = verification::verify_batch(
        &line,
        &ImportReadSource::admit(Vec::new()).unwrap(),
        verification::Attribution::Tag,
    )
    .unwrap();
    let mut declared = empty["counts"]
        .as_object()
        .unwrap()
        .keys()
        .filter(|status| *status != "cancelled_with_effective_copy")
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let rewritten = |rewrite: fn(&mut Value)| {
        let mut result = empty.clone();
        rewrite(&mut result);
        result["vouchers"][0]["status"]
            .as_str()
            .unwrap()
            .to_string()
    };
    for rewrite in [
        verification::mark_book_rolled_back as fn(&mut Value),
        verification::mark_sent_not_attributed,
        verification::mark_reported_not_created,
        verification::mark_sent_after_reported_not_created,
        verification::mark_sent_not_attributed_after_unclean_answer,
    ] {
        declared.insert(rewritten(rewrite));
    }
    let placed: &[(&str, Option<&str>, &str, &str)] = &[
        ("posted_verified", None, "found", "found in Tally as built"),
        ("posted_divergent", None, "found", "found in Tally, not confirmed as built"),
        ("posted_not_effective", Some("voucher_cancelled"), "found", "found in Tally, cancelled"),
        ("posted_not_effective", Some("voucher_optional"), "found", "found in Tally, optional"),
        ("sent_not_attributed", None, "not_established", NOT_ATTRIBUTED),
        ("matching_content_observed", None, "not_established", "a voucher with this invoice's content was found, but not shown to be the one sent"),
        ("duplicate_fingerprint", None, "not_established", "vouchers with this invoice's content were found, none shown to be the one sent"),
        ("not_attributable", Some("tag_precedes_pre_import_voucher_mark"), "not_established", "a matching voucher was found, but it was in the book before this post"),
        ("not_attributable", Some("fingerprint_precedes_pre_import_voucher_mark"), "not_established", "a matching voucher was found, but it was in the book before this post"),
        ("not_attributable", Some("observed_voucher_already_attributed"), "not_established", "a matching voucher was found, but it is already matched to another voucher"),
        ("not_attributable", Some("bound_master_id_changed"), "not_established", "a voucher carrying the identity of the one posted is in the book, but with another internal number; how is not established, for example the company's data were rewritten or copied"),
        ("bound_not_in_window", None, "not_established", "the voucher posted for this invoice is not in the book for these dates now"),
        ("book_rolled_back", None, "not_established", "the company's books were restored after this post"),
        ("not_found", None, "not_established", "not established: no voucher of it was found"),
        ("tally_reported_not_created", None, "not_established", "Tally reported it not created"),
    ];
    let unplaced =
        "not established: the read gave this invoice a state ComplyEaze Bridge does not place";
    assert_eq!(
        placed
            .iter()
            .map(|(status, ..)| status.to_string())
            .collect::<std::collections::BTreeSet<_>>(),
        declared,
        "each status the verification writes is placed, and no other"
    );
    let seen = |row: Value, duplicates: Value| {
        let seen = Seen::of(&json!({"vouchers": [row], "duplicates": duplicates}), &line);
        let (kind, detail) = seen.reported();
        (kind.to_string(), detail.to_string(), seen.words())
    };
    for (status, reason, kind, words) in placed {
        let row = json!({"bridge_txn_id": "t1", "status": status, "reason": reason});
        let expected = (kind.to_string(), status.to_string(), words.to_string());
        assert_eq!(
            seen(row.clone(), json!([])),
            expected,
            "{status} {reason:?}"
        );
        if *status != "posted_verified" {
            let duplicated = json!([{"kind": "accounting_fingerprint"}]);
            assert_eq!(seen(row, duplicated), expected, "{status} {reason:?}");
        }
    }
    // A reason or a status not placed is not established, never found.
    for (status, reason) in [
        ("posted_not_effective", Some("a_reason_not_yet_invented")),
        ("posted_not_effective", None),
        ("not_attributable", Some("a_reason_not_yet_invented")),
        ("a_state_not_yet_invented", None),
    ] {
        let row = json!({"bridge_txn_id": "t1", "status": status, "reason": reason});
        assert_eq!(
            seen(row, json!([])),
            (
                "not_established".to_string(),
                status.to_string(),
                unplaced.to_string()
            ),
            "{status} {reason:?}"
        );
    }
}

/// An invoice its post's content cannot find, as each of the verification's
/// rewrites writes it: after an answer that was not clean the words drop the
/// edit example, as the verification's own next step does; otherwise they
/// keep it.
#[test]
fn an_unattributed_invoice_after_an_unclean_answer_drops_the_edit_example() {
    use super::super::super::verification;
    let line = batch("Sales", "http://127.0.0.1:9000");
    let empty = verification::verify_batch(
        &line,
        &ImportReadSource::admit(Vec::new()).unwrap(),
        verification::Attribution::Tag,
    )
    .unwrap();
    for (rewrite, words) in [
        (
            verification::mark_sent_not_attributed_after_unclean_answer as fn(&mut Value),
            "not established: no voucher matching this invoice was found, and one may be in the book",
        ),
        (verification::mark_sent_not_attributed, NOT_ATTRIBUTED),
        (verification::mark_sent_after_reported_not_created, NOT_ATTRIBUTED),
    ] {
        let mut result = empty.clone();
        rewrite(&mut result);
        let seen = Seen::of(&result, &line);
        assert_eq!(seen.reported(), ("not_established", "sent_not_attributed"));
        assert_eq!(seen.words(), words);
    }
}

/// A row that reads verified still stops its company when another voucher
/// duplicates it or the dispatch needs reconciliation: the words say which.
#[test]
fn a_verified_row_says_why_it_is_not_confirmed() {
    let line = batch("Sales", "http://127.0.0.1:9000");
    let words = |duplicates: Value, dispatch: Value| {
        Seen::of(
            &json!({"vouchers": [{"bridge_txn_id": "t1", "status": "posted_verified"}],
                "duplicates": duplicates, "dispatch": dispatch}),
            &line,
        )
        .words()
    };
    let not_clean =
        json!({"state": "reconciliation_required", "response_state": "response_not_clean"});
    let but = |clauses: &str| format!("found in Tally as built, but not confirmed: {clauses}");
    for (duplicates, dispatch, expected) in [
        (json!([]), json!(null), "found in Tally as built".to_string()),
        (
            json!([]),
            json!({"state": "previous_attempt_reconciled", "response_state": "response_clean"}),
            "found in Tally as built".to_string(),
        ),
        (
            json!([{"kind": "accounting_fingerprint"}, {"kind": "accounting_fingerprint"}]),
            json!(null),
            but("another voucher has the same content"),
        ),
        (
            json!([{"kind": "remote_id"}]),
            json!(null),
            but("another voucher has the same remote id"),
        ),
        (
            json!([{"kind": "a_kind_not_yet_invented"}]),
            json!(null),
            but("another voucher reads as its duplicate"),
        ),
        (json!([]), not_clean.clone(), but("Tally's answer to the post needs a check")),
        (
            json!([]),
            json!({"state": "reconciliation_required", "response_state": "response_clean"}),
            but("its ledgers are not confirmed as the masters approved"),
        ),
        // A duplicate alone makes the dispatch need reconciliation: with a
        // clean answer, the duplicate is the whole reason.
        (
            json!([{"kind": "accounting_fingerprint"}]),
            json!({"state": "reconciliation_required", "response_state": "response_clean"}),
            but("another voucher has the same content"),
        ),
        (
            json!([{"kind": "remote_id"}, {"kind": "accounting_fingerprint"}]),
            not_clean,
            but("another voucher has the same remote id; another voucher has the same content; Tally's answer to the post needs a check"),
        ),
    ] {
        assert_eq!(words(duplicates.clone(), dispatch.clone()), expected, "{duplicates} {dispatch}");
    }
}

/// The release is decided on the batch's one invoice and its one row: any
/// other shape is a read that cannot be placed, never a sighting. A completed
/// read holding several rows for the invoice is not established, by its own
/// detail, never found.
#[test]
fn a_read_that_does_not_hold_the_invoice_s_one_row_is_unreadable() {
    let line = batch("Sales", "http://127.0.0.1:9000");
    let row = json!({"bridge_txn_id": "t1", "status": "posted_divergent"});
    let mut two_invoices = line.clone();
    two_invoices.vouchers.push(line.vouchers[0].clone());
    let journal = batch("Journal", "http://127.0.0.1:9000");
    let missing = (
        "unreadable".to_string(),
        "import_verification_rows_missing".to_string(),
        "ComplyEaze Bridge could not complete its read (import_verification_rows_missing), so it reports nothing about this invoice".to_string(),
    );
    for (result, line) in [
        (json!({}), &line),
        (json!({"vouchers": []}), &line),
        (
            json!({"vouchers": [{"bridge_txn_id": "t2", "status": "posted_divergent"}]}),
            &line,
        ),
        (json!({"vouchers": [{"bridge_txn_id": "t1"}]}), &line),
        (json!({"vouchers": [row.clone()]}), &two_invoices),
        (json!({"vouchers": [row.clone()]}), &journal),
    ] {
        let seen = Seen::of(&result, line);
        let (kind, detail) = seen.reported();
        assert_eq!(
            (kind.to_string(), detail.to_string(), seen.words()),
            missing,
            "{result}"
        );
    }
    let verified = json!({"bridge_txn_id": "t1", "status": "posted_verified"});
    for rows in [
        json!([row.clone(), row.clone()]),
        json!([verified.clone(), row.clone()]),
        json!([verified.clone(), verified.clone(), verified]),
    ] {
        let seen = Seen::of(&json!({"vouchers": rows}), &line);
        let (kind, detail) = seen.reported();
        assert_eq!(
            (kind.to_string(), detail.to_string(), seen.words()),
            (
                "not_established".to_string(),
                "invoice_rows_repeated".to_string(),
                "not established: the read gave this invoice more than one row".to_string()
            ),
            "{rows}"
        );
    }
}

/// A failed read is shown by its code, with what Tally did not do when the
/// failure says so; it is recorded as unreadable.
#[test]
fn a_failed_read_is_shown_by_its_code() {
    let failure = |code: &str, unanswered: Option<&'static str>| {
        let mut failure = ToolFailure::from(code.to_string());
        failure.unanswered = unanswered.map(crate::agent::Unanswered);
        failure
    };
    for (failure, words) in [
        (
            failure("request_deadline_exceeded", None),
            "ComplyEaze Bridge could not complete its read (request_deadline_exceeded: Tally did not answer in time), so it reports nothing about this invoice",
        ),
        (
            failure("import_mode_probe_failed", Some("request_deadline_exceeded")),
            "ComplyEaze Bridge could not complete its read (import_mode_probe_failed: Tally did not answer in time), so it reports nothing about this invoice",
        ),
        (
            failure("company_collection_invalid", Some("endpoint_unreachable")),
            "ComplyEaze Bridge could not complete its read (company_collection_invalid: Tally did not answer), so it reports nothing about this invoice",
        ),
        (
            failure("agent_runtime_read_failed", Some("endpoint_circuit_cooldown")),
            "ComplyEaze Bridge could not complete its read (agent_runtime_read_failed), so it reports nothing about this invoice",
        ),
    ] {
        let seen = Seen::unreadable(&failure);
        assert_eq!(seen.words(), words);
        assert_eq!(seen.reported(), ("unreadable", failure.code.as_str()));
        assert_eq!(seen.recorded(), ledger::ReleaseSeen::Unreadable);
    }
}

/// The dialog shows names from the journal and Tally with anything that could
/// reorder or hide text, and any line break, as `?`, and still shows them. A
/// batch that recorded no company name (or an empty one) names its company by
/// GUID, never by nothing.
#[test]
fn the_release_dialog_does_not_show_text_that_could_mislead() {
    use super::super::super::stop::release_preview;
    let mut line = batch("Sales", "http://127.0.0.1:9000");
    let seen = Seen::of(
        &json!({"vouchers": [{"bridge_txn_id": "t1", "status": "posted_divergent"}]}),
        &line,
    );
    let words = dialog(&line, "found in Tally, not confirmed as built");
    line.company.as_mut().unwrap().name = "Acme\u{202e}Ltd\nPaid".to_string();
    assert_eq!(
        release_preview(&line, &seen),
        words.replace("WR2 Unicode Lab", "Acme?Ltd?Paid")
    );
    line.company.as_mut().unwrap().name = String::new();
    assert_eq!(
        release_preview(&line, &seen),
        words.replace("WR2 Unicode Lab", GUID)
    );
    line.company = None;
    assert_eq!(
        release_preview(&line, &seen),
        words.replace("WR2 Unicode Lab", GUID)
    );
}

/// What the release dialog shows (#1780): the statement that the stop is
/// released stays the first line, under it the answer text says what Y does
/// to the stop, and nothing in it says a voucher was reviewed.
#[test]
fn the_release_dialog_opens_with_its_statement_and_then_says_what_yes_does() {
    use crate::tally::approved_import::{review_text, ReviewKind};
    let line = batch("Sales", "http://127.0.0.1:9000");
    let seen = Seen::of(
        &json!({"vouchers": [{"bridge_txn_id": "t1", "status": "posted_divergent"}]}),
        &line,
    );
    let preview = super::super::super::stop::release_preview(&line, &seen);
    let count = crate::tally::approved_import::VoucherCount::new(1).unwrap();
    let shown = review_text(ReviewKind::StopRelease, count, &preview);
    assert!(
        shown.starts_with("THIS RELEASES THE INVOICE STOP FOR THE COMPANY.\n\n"),
        "{shown}"
    );
    let (_, answer) = shown.split_once("\n\n").unwrap();
    assert!(
        answer.contains("release the invoice stop for this company"),
        "{shown}"
    );
    assert!(!shown.contains("record that you reviewed"), "{shown}");
    assert!(
        shown.ends_with(preview.split_once("\n\n").unwrap().1),
        "{shown}"
    );
    // The review of a doubted post is shown with its own words.
    let review = review_text(ReviewKind::Review, count, &preview);
    assert!(
        review.contains("record that you reviewed this voucher"),
        "{review}"
    );
}

/// What is not a stop cannot be released: a batch never sent, one holding no
/// invoice, one that reads verified, and no other doubt kind ever selects it.
#[tokio::test]
async fn only_a_batch_that_stops_the_company_can_be_released() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let built = saved(&server, "Sales");
    let journal = saved(&server, "Journal");
    posted(&server, &journal);
    let verified = saved(&server, "Sales");
    posted(&server, &verified);
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
    posted(&server, &stopped);
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
    posted(&server, &line);
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
    assert_eq!(release_records(&server), Vec::<Value>::new());
    assert_eq!(stop(&server), None);
    status(&server, &line, "verification_incomplete");
    assert_eq!(stop(&server), Some(line.batch_id));
}

/// A stop that lifts and comes back while the dialog is open is another stop
/// than the one the person was shown: the batch reads verified, which voids
/// what came before, then diverges and stops the company again. The release
/// is refused, nothing is written, and the new stop stands.
#[tokio::test]
async fn a_stop_that_lifts_and_returns_while_the_dialog_is_open_is_not_released() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 1);
    let server = Arc::new(server);
    let line = saved(&server, "Sales");
    posted(&server, &line);
    let (other, other_line) = (Arc::clone(&server), line.clone());
    let approval = ScriptedApproval::approving_after(move || {
        status(&other, &other_line, "posted_verified");
        status(&other, &other_line, "verification_incomplete");
    });
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_stop_changed_while_reviewing",
        "{response}"
    );
    assert_eq!(approval.review_counts(), [1]);
    assert_eq!(release_records(&server), Vec::<Value>::new());
    assert_eq!(stop(&server), Some(line.batch_id));
}

/// The under-lock check a post makes judges only an invoice: a stopped company
/// stops no Journal, Payment, Receipt or Contra.
#[test]
fn the_stop_judges_only_an_invoice_post() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let sent = saved(&server, "Sales");
    posted(&server, &sent);
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
        &sentinel_batch_id(1),
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

/// The company the caller names is the batch's own: a wrong one is refused
/// before any read or dialog, and is no way onto the path a failed read takes.
#[tokio::test]
async fn a_release_naming_another_company_is_refused_before_any_read() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let line = saved(&server, "Sales");
    posted(&server, &line);
    let mut wrong = args(&line);
    wrong["company_guid"] = json!(sentinel_uuid(0xff).to_string());
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
        .find("&& !self.settings.posting.enabled()")
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
        // Any other voucher a rejected batch carried has no route yet, and is never
        // rebuilt to get one (#1709).
        assert!(text.contains("For any other voucher: if Tally rejected that batch"));
        assert!(text.contains("nothing in ComplyEaze Bridge puts it in the book yet"));
        assert!(!text.contains("enter the voucher in Tally"));
        // A batch a restore may have emptied is no route either, and is said so (#1038).
        assert!(text.contains("when verify_import reports that batch book_rolled_back"));
    }
    let build = super::super::super::BUILD_TXN_ALREADY_POSTED_NEXT_STEP;
    assert!(build.contains("never build it again under a new bridge_txn_id to retry it"));
    assert!(build.contains("cannot build the rest of that day from the statement yet"));
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

#[path = "agent_import_content_review_tests.rs"]
mod content_review_tests;
