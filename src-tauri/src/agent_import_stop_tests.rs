//! The Sales stop and its release (ADR 0004, slice 4), against a journal on
//! disk and the tool call. Tally is not reachable in these tests (port 9): a
//! release must not need it, and a read that cannot run is shown, never fatal.
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
    let id = format!("bridge-{}", Uuid::new_v4());
    let voucher = if voucher_type == "Sales" {
        json!({"bridge_txn_id":"t1","date":"20260801","voucher_type":"Sales",
            "voucher_number":"BP/26-27/0010",
            "invoice":{"voucher_type_name":"Sales Manual","place_of_supply":"Rajasthan"},
            "narration":null,"reference":null,
            "entries":[{"ledger":"Customer","amount":"118.00","side":"Dr"},
                {"ledger":"Sales","amount":"118.00","side":"Cr"}]})
    } else {
        json!({"bridge_txn_id":"t1","date":"20260801","voucher_type":voucher_type,
            "narration":null,"reference":null,"voucher_number":null,
            "entries":[{"ledger":"Cash","amount":"118.00","side":"Dr"},
                {"ledger":"Bank","amount":"118.00","side":"Cr"}]})
    };
    let line: ImportLedgerLine = serde_json::from_value(json!({
        "batch_id":id, "identity_scheme":"batch_v1", "company_guid":GUID,
        "endpoint_origin":origin,
        "company":{"name":"WR2 Unicode Lab","guid":GUID,"company_number":"100004","books_from":"20260401"},
        "txn_ids":["t1"],"date_from":"20260801","date_to":"20260801",
        "sha256":"a".repeat(64),"built_at":"2026-08-01T00:00:00Z","status":"built",
        "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":7},
        "vouchers":[voucher]
    }))
    .unwrap();
    server.append_import_ledger(&line).unwrap();
    line
}

/// The batch as the post sent it: a dispatch intent on top of `saved`.
fn dispatched(server: &Server, line: &ImportLedgerLine) {
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_native(
            line,
            "c".repeat(64),
            Uuid::new_v4(),
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

fn result(response: &Value) -> &Value {
    &response["structuredContent"]["result"]
}

/// The one way past a stop is a person's click: it appends a release bound to
/// the batch, shows what it is for, records that Tally could not be read, and
/// survives a restart.
#[tokio::test]
async fn a_release_needs_the_person_works_without_tally_and_survives_a_restart() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
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

    // Approved: the dialog says what is released, names the batch and says
    // Tally could not be read; the release is journaled.
    let approval = ScriptedApproval::approving();
    let response = release(&server, args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "stop_released", "{response}");
    assert_eq!(result(&response)["invoice_found_at_release"], false);
    assert_eq!(approval.review_counts(), [1]);
    let shown = &approval.reviews()[0];
    assert!(shown.contains("RELEASES THE INVOICE STOP"), "{shown}");
    assert!(shown.contains(&line.batch_id), "{shown}");
    assert!(shown.contains("Tally could not be read"), "{shown}");
    assert!(approval.previews().is_empty(), "not a post dialog");
    assert_eq!(stop(&server), None);

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
    let server = Arc::new(server_without_tally(directory.path()));
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
