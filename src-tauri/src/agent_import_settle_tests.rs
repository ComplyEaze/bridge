//! A person's settlement of a native post Tally answered as creating none of
//! its vouchers (#1709), through `acknowledge_post_review` with `doubt`
//! `not_created`. Each batch is posted as production posts it, answered by a
//! captured whole-rejection answer, then settled on a later server, as a
//! later session would.
use super::*;
use crate::agent::agent_import::settle::{settle_preview, shown_open};
use crate::tally::approved_import::test_seam::review_fits;
use crate::tally::approved_import::{ReviewKind, VoucherCount};

fn settle_args(line: &ImportLedgerLine) -> Value {
    json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"not_created"})
}

async fn settle(server: &Server, args: Value, scripted: ScriptedApproval) -> Value {
    SCRIPTED_APPROVAL
        .scope(scripted, server.call_tool("acknowledge_post_review", args))
        .await
}

fn result(response: &Value) -> &Value {
    &response["structuredContent"]["result"]
}

/// The journal's person settlement records, as written.
fn settlement_records(directory: &std::path::Path) -> Vec<Value> {
    String::from_utf8(journal(directory))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == "person_settlement")
        .collect()
}

fn counted(directory: &std::path::Path) -> ledger::Settlement {
    ledger::settlement(std::io::Cursor::new(journal(directory))).unwrap()
}

/// A batch posted with `answer` as Tally's answer, the voucher mark after the
/// POST at `after_mark` (10 before) and an empty window, then a later server
/// on the same data folder whose Tally answers `reads` verifications of the
/// book at `later_mark`. A batch of three Journals with `batch`, else one.
struct Posted {
    directory: tempfile::TempDir,
    server: Server,
    simulator: SequenceSimulator,
    line: ImportLedgerLine,
}

async fn posted_then_later(
    answer: String,
    batch: bool,
    after_mark: u64,
    later_mark: u64,
    reads: usize,
) -> Posted {
    let later = (0..reads)
        .flat_map(|_| span_readback(empty_collection(), later_mark))
        .collect();
    posted_then_read(answer, batch, after_mark, later).await
}

/// As `posted_then_later`, with the later Tally answering `later`.
async fn posted_then_read(
    answer: String,
    batch: bool,
    after_mark: u64,
    later: Vec<ScenarioPlan>,
) -> Posted {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(answer)));
    plans.push(xml(company_marks(after_mark, 50, "WR2 Unicode Lab")));
    plans.extend(span_readback(empty_collection(), after_mark));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let (line, args) = if batch {
        saved_partial_batch(&server)
    } else {
        let line = saved_captured_line(&server);
        let args = json!({"company_guid":GUID,"batch_id":line.batch_id});
        (line, args)
    };
    let _ = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", args),
        )
        .await;
    let _ = sent(simulator);
    let simulator = SequenceSimulator::spawn(with_sentinel(later)).unwrap();
    let later_server = server_at(simulator.address(), directory.path());
    let origin =
        super::super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let later_origin =
        super::super::super::super::canonical_loopback_origin(&later_server.settings.endpoint)
            .unwrap();
    let text = String::from_utf8(journal(directory.path())).unwrap();
    fs::write(
        directory.path().join("agent-import-ledger.jsonl"),
        text.replace(&origin, &later_origin),
    )
    .unwrap();
    Posted {
        directory,
        server: later_server,
        simulator,
        line,
    }
}

/// One voucher Tally rejected: the person is asked in the settle dialog, a
/// decline writes nothing, and a yes writes the record. The batch then leaves
/// every `not_settled` count, `verify_import` reports the settlement beside an
/// unchanged verdict, a second settlement is refused before any read, and the
/// row stays held.
#[tokio::test]
async fn a_rejected_voucher_is_settled_only_by_the_person_and_its_row_stays_held() {
    let Posted {
        directory,
        server,
        simulator,
        line,
    } = posted_then_later(rejected_one_silver(), false, 10, 10, 3).await;
    let before = counted(directory.path());
    assert_eq!((before.unsettled, before.settled_by_person), (1, 0));

    let declined = ScriptedApproval::declining();
    let response = settle(&server, settle_args(&line), declined.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_review_declined",
        "{response}"
    );
    assert_eq!(declined.review_kinds(), [ReviewKind::Settle]);
    assert_eq!(settlement_records(directory.path()), Vec::<Value>::new());

    // The company's GUID is matched without regard to case.
    let mut upper = settle_args(&line);
    upper["company_guid"] = json!(GUID.to_uppercase());
    let approval = ScriptedApproval::approving();
    let response = settle(&server, upper, approval.clone()).await;
    assert_eq!(
        result(&response)["state"],
        "settled_by_person",
        "{response}"
    );
    let status = result(&response)["verification_status"].clone();
    assert_ne!(status, "posted_verified", "{response}");
    assert_eq!(approval.review_kinds(), [ReviewKind::Settle]);
    assert_eq!(approval.review_counts(), [1]);
    let shown = &approval.reviews()[0];
    assert!(
        shown.starts_with(
            "THIS CLOSES THE BATCH. COMPLYEAZE BRIDGE NEVER SENDS THIS VOUCHER AGAIN."
        ),
        "{shown}"
    );
    let records = settlement_records(directory.path());
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["batch_sha256"], line.sha256);
    let after = counted(directory.path());
    assert_eq!(
        (
            after.unsettled,
            after.unsettled_binding_refused,
            after.settled_by_person
        ),
        (0, 0, 1)
    );

    // Reported by the next verification, whose verdict is unchanged.
    let verified = server
        .call_tool(
            "verify_import",
            json!({"company_guid":GUID,"batch_id":line.batch_id}),
        )
        .await;
    let verified = result(&verified);
    assert_eq!(verified["verification_status"], status, "{verified}");
    assert_eq!(verified["counts"]["sent_not_attributed"], 1, "{verified}");
    assert_eq!(
        verified["person_settlement"],
        json!({"state": "settled", "next_step": super::super::super::settle::NEXT_STEP}),
        "{verified}"
    );
    assert_eq!(
        verified["post_span_binding"]["summary"],
        super::super::super::settle::SUMMARY
    );
    // The voucher's own line no longer asks for the settlement it has.
    assert_eq!(
        verified["unverified_vouchers"][0]["next_step"],
        super::super::super::settle::NEXT_STEP,
        "{verified}"
    );
    assert_eq!(
        sent(simulator).len(),
        3 * span_readback(empty_collection(), 10).len()
    );

    // A second settlement is refused before any read or dialog.
    let again = ScriptedApproval::approving();
    let response = settle(&server, settle_args(&line), again.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_already_recorded",
        "{response}"
    );
    assert!(again.reviews().is_empty());

    // The row stays held: nothing of it is built or posted again (#876).
    let journal_file = fs::File::open(directory.path().join("agent-import-ledger.jsonl")).unwrap();
    assert_eq!(
        ledger::vouchers_already_posted(
            std::io::BufReader::new(journal_file),
            GUID,
            None,
            &line.vouchers
        )
        .unwrap(),
        Some(line.batch_id.clone())
    );
}

/// The settled summary is built where `verify_import` reports it, and keeps
/// what follows the binding's first line (#1709, #1050). Here a second post of
/// the company, sent after this one, proved a mark the book no longer reaches:
/// the verification records that restore, and the settled line is followed by
/// the sentence that names the post to verify again.
#[tokio::test]
async fn a_settled_batch_s_verification_keeps_the_restore_line_after_its_own() {
    let Posted {
        directory,
        server,
        simulator: _simulator,
        line,
    } = posted_then_later(rejected_one_silver(), false, 10, 10, 2).await;
    let response = settle(&server, settle_args(&line), ScriptedApproval::approving()).await;
    assert_eq!(
        result(&response)["state"],
        "settled_by_person",
        "{response}"
    );
    let later = crate::agent::agent_import::tests::sentinel_batch_id(0x1709);
    let journal = directory.path().join("agent-import-ledger.jsonl");
    let text = fs::read_to_string(&journal).unwrap();
    fs::write(&journal, append_later_post(text, &later)).unwrap();
    let verified = server
        .call_tool(
            "verify_import",
            json!({"company_guid":GUID,"batch_id":line.batch_id}),
        )
        .await;
    let verified = result(&verified);
    let binding = &verified["post_span_binding"];
    assert_eq!(
        verified["person_settlement"]["state"], "settled",
        "{verified}"
    );
    assert_eq!(binding["rollback_recorded"], true, "{verified}");
    assert_eq!(
        binding["other_batches_covered"],
        json!({"count": 1, "first": [later]}),
        "{verified}"
    );
    assert_eq!(
        binding["summary"],
        format!(
            "{} This check also found the company's books restored from a backup after other posts of it, and recorded that: verify the other post named in other_batches_covered again, and check their vouchers in Tally.",
            super::super::super::settle::SUMMARY
        ),
        "{verified}"
    );
}

/// A batch Tally rejected whole is settled when the post recorded its voucher
/// mark moving by what Tally reported creating; when the mark moved, or the
/// record of that step is gone, it is refused before any read or dialog.
#[tokio::test]
async fn a_rejected_batch_is_settled_only_when_its_post_recorded_the_step_matched() {
    let Posted {
        directory,
        server,
        simulator: _simulator,
        line,
    } = posted_then_later(rejected_all_three(), true, 10, 10, 1).await;
    let approval = ScriptedApproval::approving();
    let response = settle(&server, settle_args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["state"],
        "settled_by_person",
        "{response}"
    );
    assert_eq!(approval.review_counts(), [3]);

    for remove in [false, true] {
        let Posted {
            directory,
            server,
            simulator,
            line,
        } = posted_then_later(
            rejected_all_three(),
            true,
            if remove { 10 } else { 11 },
            11,
            1,
        )
        .await;
        if remove {
            let imports = server.imports_dir().unwrap();
            for record in ["masters_check", "masters_verdict"] {
                fs::remove_file(imports.join(format!("{}.{record}.json", line.batch_id))).unwrap();
            }
        }
        let approval = ScriptedApproval::approving();
        let response = settle(&server, settle_args(&line), approval.clone()).await;
        assert_eq!(
            result(&response)["error"]["code"],
            "ack_not_created_not_established",
            "{remove}: {response}"
        );
        assert!(approval.reviews().is_empty());
        assert_eq!(sent(simulator).len(), 0, "no request: {response}");
        assert_eq!(settlement_records(directory.path()), Vec::<Value>::new());
    }
    assert_eq!(settlement_records(directory.path()).len(), 1);
}

/// Refused before any read: a batch never sent, one whose answer created its
/// voucher, and one named under another company. No request is sent.
#[tokio::test]
async fn a_batch_not_on_record_as_rejected_is_refused_before_any_read() {
    let directory = tempfile::tempdir().unwrap();
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let server = server_at(simulator.address(), directory.path());
    let unsent = saved_captured_line(&server);
    let response = settle(&server, settle_args(&unsent), ScriptedApproval::approving()).await;
    assert_eq!(result(&response)["error"]["code"], "ack_batch_not_posted");

    let created = saved_captured_line(&server);
    let native =
        native_post_request(&created, RemoteIds::from_ids(vec![uuid::Uuid::new_v4()])).unwrap();
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &created,
                &native,
                Some(10),
            ))
            .unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::response(
                &created,
                ledger::DispatchResponse {
                    request_sha256: native.request_sha256.clone(),
                    ..super::super::tests::dispatch_response("success", 1, 0)
                },
            ))
            .unwrap();
    }
    let response = settle(
        &server,
        settle_args(&created),
        ScriptedApproval::approving(),
    )
    .await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_not_created_not_established"
    );

    // The company is checked first: here the answer would refuse it too.
    let mut elsewhere = settle_args(&created);
    elsewhere["company_guid"] = json!("00000000-0000-4000-8000-000000000002");
    let response = settle(&server, elsewhere, ScriptedApproval::approving()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "import_batch_company_mismatch",
        "{response}"
    );
    assert_eq!(sent(simulator).len(), 0);
}

/// A read that finds the book rolled back below the post asks nothing.
#[tokio::test]
async fn a_read_that_shows_the_book_rolled_back_asks_nothing() {
    let Posted {
        directory,
        server,
        simulator: _simulator,
        line,
    } = posted_then_later(rejected_one_silver(), false, 10, 5, 1).await;
    let approval = ScriptedApproval::approving();
    let response = settle(&server, settle_args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_not_created_not_established",
        "{response}"
    );
    assert!(approval.reviews().is_empty());
    assert_eq!(settlement_records(directory.path()), Vec::<Value>::new());
}

/// What the person was shown must still hold when the record is written: a
/// batch that reads verified, is answered again, or is verified again while
/// the dialog is open, or that another batch's check finds rolled back, is
/// refused, and nothing is written.
#[tokio::test]
async fn a_change_while_the_dialog_is_open_writes_nothing() {
    for change in ["verified", "answered", "verified again", "rolled back"] {
        let Posted {
            directory,
            server,
            simulator: _simulator,
            line,
        } = posted_then_later(rejected_one_silver(), false, 10, 10, 1).await;
        let request = String::from_utf8(journal(directory.path()))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .find_map(|record| record["native_request_sha256"].as_str().map(str::to_string))
            .unwrap();
        // Another batch of the same company and address, whose check reads
        // the mark below this batch's floor.
        let mut seer = line.clone();
        seer.batch_id = format!("bridge-{}", uuid::Uuid::new_v4());
        seer.endpoint_origin = Some(
            super::super::super::super::canonical_loopback_origin(&server.settings.endpoint)
                .unwrap(),
        );
        server.append_import_ledger(&seer).unwrap();
        let seer_native =
            native_post_request(&seer, RemoteIds::from_ids(vec![uuid::Uuid::new_v4()])).unwrap();
        {
            let _lock = server.lock_import_admission().unwrap();
            server
                .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                    &seer,
                    &seer_native,
                    Some(10),
                ))
                .unwrap();
        }
        let ledger_path = directory.path().join("agent-import-ledger.jsonl");
        let server = std::sync::Arc::new(server);
        let (other, other_line) = (std::sync::Arc::clone(&server), line.clone());
        let approval = ScriptedApproval::approving_after(move || {
            let records = fs::read_to_string(&ledger_path).unwrap().lines().count() as u64;
            let record = match change {
                "rolled back" => ledger::StatusRecord::rollback_seen(&seer, 1, records),
                "answered" => ledger::StatusRecord::response(
                    &other_line,
                    ledger::DispatchResponse {
                        request_sha256: request.clone(),
                        ..super::super::tests::dispatch_response("success", 1, 0)
                    },
                ),
                status => serde_json::from_value::<ledger::StatusRecord>(json!({
                    "record_type":"verification_status","batch_id":other_line.batch_id,
                    "batch_sha256":other_line.sha256,
                    "status": if status == "verified" { "posted_verified" } else { "reconciliation_required" }
                }))
                .unwrap(),
            };
            let _lock = other.lock_import_admission().unwrap();
            other.append_import_record_while_admitted(&record).unwrap();
        });
        let response = settle(&server, settle_args(&line), approval.clone()).await;
        assert_eq!(
            result(&response)["error"]["code"],
            "ack_changed_while_reviewing",
            "{change}: {response}"
        );
        assert_eq!(approval.review_counts(), [1], "{change}");
        assert_eq!(
            settlement_records(directory.path()),
            Vec::<Value>::new(),
            "{change}"
        );
    }
}

/// The dialog says first what the settlement does and does not do, then the
/// batch, what Tally answered and what the read found; a Sales batch says its
/// stop stays. It fits the box for one voucher and for 50.
#[test]
fn the_settle_dialog_says_what_it_closes_and_fits_the_box() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let line = saved_captured_line(&server);
    let read = json!({"counts":{"matching_content_observed":0}});
    let preview = settle_preview(&line, &read);
    assert_eq!(
        preview,
        format!(
            "THIS CLOSES THE BATCH. COMPLYEAZE BRIDGE NEVER SENDS THIS VOUCHER AGAIN.\n\
             It changes nothing in Tally and does not mark the batch posted. Its rows stay held: they cannot be\n\
             built or posted again.\n\
             \n\
             Company: WR2 Unicode Lab\n\
             Batch: {}\n\
             Sent to Tally: 1 voucher (1 Journal), dated {}.\n\
             When it was sent, Tally answered that it did not create it.\n\
             The read just now found, for 0 of the 1 voucher sent, a voucher in the book with the same date,\n\
             voucher type and ledger entries.\n\
             \n\
             Confirm only if you have checked in Tally that the voucher is in the book, or you have decided it\n\
             will not be posted.",
            line.batch_id, line.vouchers[0].date.as_str()
        )
    );
    assert!(review_fits(
        ReviewKind::Settle,
        VoucherCount::new(1).unwrap(),
        &preview
    ));

    let mut fifty = line.clone();
    fifty.vouchers = (0..50)
        .map(|index| {
            let mut voucher = line.vouchers[0].clone();
            voucher.bridge_txn_id = format!("R-1-{index:02}");
            voucher
        })
        .collect();
    let preview = settle_preview(&fifty, &read);
    assert!(
        preview.contains("NEVER SENDS THESE VOUCHERS AGAIN."),
        "{preview}"
    );
    assert!(
        preview
            .replace('\n', " ")
            .contains("When they were sent, Tally answered that it created none of them."),
        "{preview}"
    );
    assert!(review_fits(
        ReviewKind::Settle,
        VoucherCount::new(50).unwrap(),
        &preview
    ));
    assert!(!preview.contains("invoice stop"));

    let mut sales: Value = serde_json::to_value(&line).unwrap();
    sales["vouchers"][0]["voucher_type"] = json!("Sales");
    sales["vouchers"][0]["voucher_number"] = json!("BP/26-27/0010");
    sales["vouchers"][0]["invoice"] =
        json!({"voucher_type_name":"Sales Manual","place_of_supply":"Rajasthan"});
    let sales: ImportLedgerLine = serde_json::from_value(sales).unwrap();
    assert!(settle_preview(&sales, &read)
        .contains("Its invoice stop stays until you release it on its own."));
}

/// Only a result that still shows the batch open is settled.
#[test]
fn only_a_read_that_shows_the_batch_open_is_settled() {
    for (status, state, open) in [
        ("reconciliation_required", "refused", true),
        ("reconciliation_required", "not_bound", true),
        ("reconciliation_required", "book_rolled_back", false),
        ("reconciliation_required", "unsettled", false),
        ("reconciliation_required", "bound", false),
        ("posted_verified", "refused", false),
    ] {
        let read = json!({"verification_status":status,"post_span_binding":{"state":state}});
        assert_eq!(shown_open(&read), open, "{status} {state}");
    }
}

/// A settled Sales batch keeps its invoice stop until the separate release,
/// and its row stays held: the settlement is read by neither.
#[test]
fn a_settled_invoice_keeps_its_stop_and_its_row() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    // With no journal yet, no batch is settled.
    assert_eq!(server.import_person_settlement("bridge-none"), Ok(false));
    let mut line: Value = serde_json::to_value(saved_captured_line(&server)).unwrap();
    line["batch_id"] = json!(format!("bridge-{}", uuid::Uuid::new_v4()));
    line["vouchers"][0]["voucher_type"] = json!("Sales");
    line["vouchers"][0]["voucher_number"] = json!("BP/26-27/0010");
    line["vouchers"][0]["invoice"] =
        json!({"voucher_type_name":"Sales Manual","place_of_supply":"Rajasthan"});
    let line: ImportLedgerLine = serde_json::from_value(line).unwrap();
    server.append_import_ledger(&line).unwrap();
    let native =
        native_post_request(&line, RemoteIds::from_ids(vec![uuid::Uuid::new_v4()])).unwrap();
    let mut rejected = super::super::tests::dispatch_response("success", 0, 0);
    rejected.request_sha256 = native.request_sha256.clone();
    let mut outcome = serde_json::to_value(rejected.outcome.take().unwrap()).unwrap();
    outcome["counters"]["exceptions"] = json!(1);
    rejected.outcome = Some(serde_json::from_value(outcome).unwrap());
    {
        let _lock = server.lock_import_admission().unwrap();
        for record in [
            ledger::StatusRecord::dispatch_for(&line, &native, Some(10)),
            ledger::StatusRecord::response(&line, rejected),
            ledger::StatusRecord::person_settlement(&line),
        ] {
            server.append_import_record_while_admitted(&record).unwrap();
        }
    }
    assert_eq!(server.import_person_settlement(&line.batch_id), Ok(true));
    assert_eq!(
        server.import_invoice_stop(GUID).unwrap(),
        Some(line.batch_id.clone())
    );
    let journal_file = fs::File::open(directory.path().join("agent-import-ledger.jsonl")).unwrap();
    assert_eq!(
        ledger::vouchers_already_posted(
            std::io::BufReader::new(journal_file),
            GUID,
            None,
            &line.vouchers
        )
        .unwrap(),
        Some(line.batch_id.clone())
    );
}

/// The journal decides before any read: a native post with a pre-POST mark,
/// answered as creating none of its vouchers, neither verified nor bound (a
/// refused binding does not stop it). For a batch of two or more, the post
/// must also have recorded its step matched.
#[test]
fn the_journaled_answer_is_read_from_the_batch_s_records() {
    use crate::agent::agent_import::settle::journaled_not_created;
    let batch = |pre: Option<u64>, amounts: &[&str], after: &[&str], matched: bool| {
        let directory = tempfile::tempdir().unwrap();
        let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
        let (line, _) = saved_journal_batch(&server, amounts);
        let ids = line.vouchers.iter().map(|_| uuid::Uuid::new_v4()).collect();
        let native = native_post_request(&line, RemoteIds::from_ids(ids)).unwrap();
        let mut rejected = super::super::tests::dispatch_response("success", 0, 0);
        rejected.request_sha256 = native.request_sha256.clone();
        let mut outcome = serde_json::to_value(rejected.outcome.take().unwrap()).unwrap();
        outcome["counters"]["exceptions"] = json!(amounts.len());
        rejected.outcome = Some(serde_json::from_value(outcome).unwrap());
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &line, &native, pre,
            ))
            .unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::response(&line, rejected))
            .unwrap();
        for more in after {
            let record = match *more {
                "bound" => ledger::StatusRecord::post_span_verdict(
                    &line,
                    &ledger::PostSpanVerdict::Bound(vec![
                        super::super::super::span_identity::PostedVoucherIdentity {
                            bridge_txn_id: line.vouchers[0].bridge_txn_id.clone(),
                            guid: "aaaa-0000000b".into(),
                            master_id: 11,
                        },
                    ]),
                ),
                "refused" => ledger::StatusRecord::post_span_verdict(
                    &line,
                    &ledger::PostSpanVerdict::Refused("span_content_mismatch".into()),
                ),
                status => serde_json::from_value(json!({
                    "record_type":"verification_status","batch_id":line.batch_id,
                    "batch_sha256":line.sha256,"status":status
                }))
                .unwrap(),
            };
            server.append_import_record_while_admitted(&record).unwrap();
        }
        let snapshot = server
            .import_snapshot_while_admitted(Some(&line.batch_id))
            .unwrap()
            .unwrap();
        journaled_not_created(&snapshot, || matched)
    };
    let case = |pre: Option<u64>, after: &[&str]| batch(pre, &["12.61"], after, false);
    assert!(case(Some(10), &[]));
    assert!(case(Some(10), &["reconciliation_required"]));
    assert!(!case(None, &[]));
    assert!(!case(Some(10), &["posted_verified"]));
    assert!(!case(Some(10), &["bound"]));
    assert!(case(Some(10), &["refused"]));
    let two = ["12.61", "13.61"];
    assert!(batch(Some(10), &two, &[], true));
    assert!(!batch(Some(10), &two, &[], false));
}

/// A settlement replaces the first line of the binding's summary only. A
/// restore that reached other posts of the company is still named after it,
/// so the person is told to verify them again (#1050).
#[test]
fn a_settled_summary_keeps_what_follows_its_first_line() {
    use super::super::super::settled_post_span_summary;
    let settled = super::super::super::settle::SUMMARY;
    let restored = json!({
        "state": "refused",
        "rollback_recorded": true,
        "other_batches_covered": {"count": 1},
    });
    let summary = settled_post_span_summary(&restored);
    assert!(summary.starts_with(settled), "{summary}");
    assert!(
        summary.ends_with(
            " This check also found the company's books restored from a backup after other posts of it, and recorded that: verify the other post named in other_batches_covered again, and check their vouchers in Tally."
        ),
        "{summary}"
    );
    let two = json!({
        "state": "refused",
        "rollback_recorded": true,
        "other_batches_covered": {"count": 2},
    });
    assert!(settled_post_span_summary(&two)
        .contains("verify the 2 other posts named in other_batches_covered again"),);
    // No restore, no examples to name: the settled line alone.
    assert_eq!(
        settled_post_span_summary(&json!({"state": "not_bound"})),
        settled
    );
}

/// The dialog counts a match as "in the book" only when it is in the accounts:
/// a cancelled or optional voucher with the same content is not (#1039).
#[test]
fn the_settle_dialog_counts_only_matches_in_the_accounts() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let line = saved_captured_line(&server);
    let found = |effective: bool| {
        settle_preview(
            &line,
            &json!({"vouchers": [{
                "status": "matching_content_observed",
                "accounting_effective": effective,
            }]}),
        )
        .replace('\n', " ")
    };
    assert!(
        found(false).contains("found, for 0 of the 1 voucher sent,"),
        "{}",
        found(false)
    );
    assert!(
        found(true).contains("found, for 1 of the 1 voucher sent,"),
        "{}",
        found(true)
    );
}

/// A voucher's next step stops asking for the settlement only where it asked
/// for it: the two lines that end on that call.
#[test]
fn only_the_next_steps_that_ask_for_the_settlement_are_replaced() {
    use super::super::super::settle::{asks_for_settlement, NEXT_STEP};
    use super::super::super::verification::{
        plain_next_step, REPORTED_NOT_CREATED_WHEN_SENT_NEXT_STEP,
    };
    assert!(asks_for_settlement(plain_next_step(
        "tally_reported_not_created"
    )));
    assert!(asks_for_settlement(Some(
        REPORTED_NOT_CREATED_WHEN_SENT_NEXT_STEP
    )));
    assert!(!asks_for_settlement(plain_next_step("sent_not_attributed")));
    assert!(!asks_for_settlement(Some(NEXT_STEP)));
    assert!(!asks_for_settlement(None));

    // In a settled batch's result only those lines change; with no
    // settlement nothing does.
    use super::super::super::settle::settle_voucher_next_steps;
    let asking = plain_next_step("tally_reported_not_created").unwrap();
    let other = plain_next_step("sent_not_attributed").unwrap();
    let rows = json!([
        {"next_step": asking},
        {"next_step": other},
        {"next_step": null},
        {},
    ]);
    let mut settled = json!({"person_settlement": {"state": "settled"}, "vouchers": rows});
    settle_voucher_next_steps(&mut settled);
    assert_eq!(
        settled["vouchers"],
        json!([
            {"next_step": NEXT_STEP},
            {"next_step": other},
            {"next_step": null},
            {},
        ])
    );
    let mut open = json!({"vouchers": rows});
    settle_voucher_next_steps(&mut open);
    assert_eq!(open["vouchers"], rows);
}

/// The dialog names each voucher type with its count, the batch's first and
/// last dates whatever their order, and a company that recorded no name by
/// its GUID. A name too long for a line is broken so the box still takes it.
#[test]
fn the_settle_dialog_counts_each_type_spans_the_dates_and_breaks_a_long_name() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let line = saved_captured_line(&server);
    let read = json!({"counts":{"matching_content_observed":0}});
    let mut mixed: Value = serde_json::to_value(&line).unwrap();
    let template = mixed["vouchers"][0].clone();
    mixed["vouchers"] = [
        ("R-1-01", "20260909", "Journal"),
        ("R-1-02", "20260905", "Payment"),
        ("R-1-03", "20260907", "Journal"),
    ]
    .iter()
    .map(|(id, date, kind)| {
        let mut voucher = template.clone();
        voucher["bridge_txn_id"] = json!(id);
        voucher["date"] = json!(date);
        voucher["voucher_type"] = json!(kind);
        voucher
    })
    .collect();
    let named = |name: Option<&str>| {
        let mut batch = mixed.clone();
        batch["company"] = match name {
            Some(name) => {
                let mut company = mixed["company"].clone();
                company["name"] = json!(name);
                company
            }
            None => Value::Null,
        };
        let batch: ImportLedgerLine = serde_json::from_value(batch).unwrap();
        settle_preview(&batch, &read)
    };
    let three = VoucherCount::new(3).unwrap();
    let preview = named(Some("WR2 Unicode Lab"));
    let lines: Vec<&str> = preview.lines().collect();
    assert_eq!(
        lines[5..7],
        [
            "Batch: bridge-6c79872c-aab6-4be5-a181-18182c8148be",
            "Sent to Tally: 3 vouchers (2 Journal, 1 Payment), dated 20260905 to 20260909.",
        ],
        "{preview}"
    );
    for unnamed in [None, Some("")] {
        let preview = named(unnamed);
        assert_eq!(
            preview.lines().nth(4),
            Some(format!("Company: {GUID}").as_str()),
            "{preview}"
        );
    }
    let long = "Q".repeat(250);
    let preview = named(Some(&long));
    assert_eq!(
        preview.lines().skip(4).take(4).collect::<Vec<_>>(),
        ["Company:", &long[..100], &long[100..200], &long[200..]],
        "{preview}"
    );
    assert!(
        review_fits(ReviewKind::Settle, three, &preview),
        "{preview}"
    );
}

/// The settled lines keep their words: the batch is never sent again, and a
/// voucher wanted in the book is entered in Tally, never imported through
/// Tally's Import menu.
#[test]
fn the_settled_summary_and_next_step_keep_their_words() {
    use super::super::super::settle::{NEXT_STEP, SUMMARY};
    assert_eq!(
        SUMMARY,
        "A person settled this batch after Tally reported that its post created none of its \
         vouchers: ComplyEaze Bridge never sends them again."
    );
    assert_eq!(
        NEXT_STEP,
        "A person settled this batch in ComplyEaze Bridge's own dialog, saying each voucher is in \
         the book or will not be posted. It is closed and counted as settled, but not marked \
         posted, and its rows are never built or posted again. If a voucher is wanted in the book \
         and is not there, it can only be entered in Tally; do not import its file through \
         Tally's Import menu."
    );
}

/// The dialog says what the read just found: a voucher of the same content
/// in the accounts is counted, though the binding is refused.
#[tokio::test]
async fn the_settle_dialog_counts_what_the_read_found() {
    let Posted {
        directory,
        server,
        simulator: _simulator,
        line,
    } = posted_then_read(
        rejected_one_silver(),
        false,
        10,
        span_readback(tagged_capture_in_span(), 11),
    )
    .await;
    let approval = ScriptedApproval::approving();
    let response = settle(&server, settle_args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["state"],
        "settled_by_person",
        "{response}"
    );
    let shown = &approval.reviews()[0];
    assert!(
        shown.lines().any(|line| line
            == "The read just now found, for 1 of the 1 voucher sent, a voucher in the book with the same date,"),
        "{shown}"
    );
    assert_eq!(settlement_records(directory.path()).len(), 1);
}
