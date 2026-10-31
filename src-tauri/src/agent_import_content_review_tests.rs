//! A person's review of a native post matched by content only (#1039),
//! against a journal on disk and the tool call. The state a review is
//! admitted on is the one the lab's refused batch answered with, read from
//! its capture; the tool runs against the captured book after a native post.
use super::*;
use crate::agent::agent_import::content_review::{journaled_state, review_preview, shown_state};
use crate::tally::approved_import::test_seam::review_fits;
use crate::tally::approved_import::{ReviewKind, VoucherCount};
use ledger::ReviewedBinding::{Refused, ResponseNotRecorded};

/// The `result` of the lab's `verify_import` of its refused batch (#1039 row
/// R1, `refused-span-binding-lab/PROVENANCE.md`).
fn lab_result() -> Value {
    let exchange: Value = serde_json::from_str(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/refused-span-binding-lab/013-s8-verify_import-b1.json"
    ))
    .unwrap();
    exchange["response"]["result"]["structuredContent"]["result"].clone()
}

fn review_args(line: &ImportLedgerLine) -> Value {
    json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"content_only"})
}

/// The batch as sent by a native post whose answer from Tally was lost: its
/// dispatch intent with the pre-POST mark, and nothing after it. Returns the
/// request's digest, which an answer recorded later must name.
fn sent_unanswered(server: &Server, line: &ImportLedgerLine) -> String {
    let ids = line.vouchers.iter().map(|_| Uuid::new_v4()).collect();
    let native = native_post_request(line, RemoteIds::from_ids(ids)).unwrap();
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
            line,
            &native,
            Some(8),
        ))
        .unwrap();
    native.request_sha256
}

/// Tally's answer to `line`'s post, recorded late, with an outcome.
fn answered_late(server: &Server, line: &ImportLedgerLine, request_sha256: &str) {
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::response(
            line,
            ledger::DispatchResponse {
                request_sha256: request_sha256.into(),
                ..super::super::tests::dispatch_response("success", 1, 0)
            },
        ))
        .unwrap();
}

/// Tally's answer to `line`'s post, recorded late, with no outcome.
fn answered_with_no_outcome(server: &Server, line: &ImportLedgerLine, request_sha256: &str) {
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::response(
            line,
            ledger::DispatchResponse {
                request_sha256: request_sha256.into(),
                outcome: None,
                ..super::super::tests::dispatch_response("success", 1, 0)
            },
        ))
        .unwrap();
}

/// `line`'s span binding refused by `code`, recorded under the lock.
fn binding_refused(server: &Server, line: &ImportLedgerLine, code: &str) {
    let _lock = server.lock_import_admission().unwrap();
    server
        .append_import_record_while_admitted(&ledger::StatusRecord::post_span_verdict(
            line,
            &ledger::PostSpanVerdict::Refused(code.into()),
        ))
        .unwrap();
}

/// [`posted`], then the binding of its post refused by `code`.
fn refused_with(server: &Server, line: &ImportLedgerLine, code: &str) {
    posted(server, line);
    binding_refused(server, line, code);
}

/// The journal's content review records, as written.
fn review_records(server: &Server) -> Vec<Value> {
    fs::read_to_string(server.settings.data_dir.join("agent-import-ledger.jsonl"))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == "content_review")
        .collect()
}

fn settled(server: &Server) -> ledger::Settlement {
    let journal =
        fs::File::open(server.settings.data_dir.join("agent-import-ledger.jsonl")).unwrap();
    ledger::settlement(std::io::BufReader::new(journal)).unwrap()
}

/// The lab's refused batch is admitted as `refused`; every other state the
/// verification reports is not, and neither is one that reads verified.
#[test]
fn the_lab_s_refused_batch_is_the_state_a_review_admits() {
    let lab = lab_result();
    assert_eq!(lab["post_span_binding"]["code"], "span_step_not_created");
    assert_eq!(lab["counts"]["matching_content_observed"], 50);
    assert_eq!(shown_state(&lab), Some(Refused));
    let with = |binding: Value, status: &str| {
        let mut result = lab.clone();
        result["post_span_binding"] = binding;
        result["verification_status"] = json!(status);
        shown_state(&result)
    };
    let incomplete = "verification_incomplete";
    assert_eq!(
        with(
            json!({"state":"not_bound","code":"binding_response_not_recorded"}),
            incomplete
        ),
        Some(ResponseNotRecorded)
    );
    for refused in [
        json!({"state":"refused","code":"span_identity_reused"}),
        json!({"state":"refused"}),
        json!({"state":"not_bound","code":"binding_not_recorded"}),
        json!({"state":"unsettled","code":"binding_effective_date_not_observed"}),
        json!({"state":"unsettled","code":"binding_journal_unreadable"}),
        json!({"state":"book_rolled_back","current_mark":4,"expected_at_least":9}),
        json!({"state":"bound"}),
        json!({"state":"not_applicable"}),
        Value::Null,
    ] {
        assert_eq!(with(refused.clone(), incomplete), None, "{refused}");
    }
    assert_eq!(
        with(lab["post_span_binding"].clone(), "posted_verified"),
        None
    );
}

/// A batch of `count` Receipts of the lab's company, dated as the lab's.
fn receipts(count: usize, company: &str) -> ImportLedgerLine {
    let mut line = serde_json::to_value(batch("Receipt", "http://127.0.0.1:9001")).unwrap();
    let voucher = line["vouchers"][0].clone();
    line["vouchers"] = (0..count)
        .map(|index| {
            let mut voucher = voucher.clone();
            voucher["bridge_txn_id"] = json!(format!("R-1-{:02}", index + 1));
            voucher["date"] = json!("20250421");
            voucher
        })
        .collect();
    line["company"]["name"] = json!(company);
    serde_json::from_value(line).unwrap()
}

/// The dialog for the lab's batch says first what the record does and does
/// not do, then the batch, why it is matched by content only and what the
/// read found; it fits the review box for 50 vouchers, and for one voucher of
/// a company with a long name.
#[test]
fn the_review_dialog_says_what_it_records_and_fits_the_box() {
    let lab = lab_result();
    let line = receipts(50, "BRIDGE CONCUR LAB");
    let preview = review_preview(&line, Refused, &lab);
    assert_eq!(
        preview,
        format!(
            "THIS RECORDS ONLY THAT YOU CHECKED THESE VOUCHERS IN TALLY.\n\
             It changes nothing in Tally, does not mark the batch posted, and does not let its rows be posted\n\
             again.\n\
             \n\
             Company: BRIDGE CONCUR LAB\n\
             Batch: {}\n\
             Sent to Tally: 50 vouchers (50 Receipt), dated 20250421.\n\
             ComplyEaze Bridge cannot tell which Tally vouchers this post created: its match of the post to the\n\
             Tally vouchers it created was refused (span_step_not_created).\n\
             The read just now found, for 50 of the 50 vouchers sent, a voucher in the book with the same date,\n\
             voucher type and ledger entries.\n\
             \n\
             Confirm only if you have checked these vouchers in Tally yourself.",
            line.batch_id
        )
    );
    let fifty = VoucherCount::new(50).unwrap();
    assert!(
        review_fits(ReviewKind::Review, fifty, &preview),
        "{preview}"
    );
    let unanswered = review_preview(&line, ResponseNotRecorded, &lab).replace('\n', " ");
    for said in [
        "Attempted to post, no answer from Tally recorded: 50 vouchers (50 Receipt), dated 20250421.",
        "ComplyEaze Bridge cannot tell which Tally vouchers this post created: it has no readable answer from Tally to the post.",
        "for 50 of the 50 vouchers in the batch,",
    ] {
        assert!(unanswered.contains(said), "{said}: {unanswered}");
    }
    let unanswered = review_preview(&line, ResponseNotRecorded, &lab);
    assert!(review_fits(ReviewKind::Review, fifty, &unanswered));
    // One voucher has the smaller box; a long name wraps, and a character
    // that could mislead the reader shows as `?`.
    let long = format!("{} \u{202e}LAB", "BRIDGE CONCUR ".repeat(14));
    let one = receipts(1, &long);
    let preview = review_preview(&one, Refused, &lab);
    assert!(preview.contains("?LAB"), "{preview}");
    assert!(preview.lines().all(|text| text.chars().count() <= 100));
    assert!(review_fits(
        ReviewKind::Review,
        VoucherCount::new(1).unwrap(),
        &preview
    ));
    // One voucher is named as one.
    assert!(preview.starts_with("THIS RECORDS ONLY THAT YOU CHECKED THIS VOUCHER IN TALLY.\n"));
    assert!(preview.contains("Sent to Tally: 1 voucher (1 Receipt), dated 20250421."));
    assert!(preview.ends_with("Confirm only if you have checked this voucher in Tally yourself."));
    // A name with no space is broken where a line ends, so the box still holds it.
    let unbroken = receipts(1, &"X".repeat(250));
    let preview = review_preview(&unbroken, Refused, &lab);
    assert!(
        preview.lines().all(|text| text.chars().count() <= 100),
        "{preview}"
    );
    assert_eq!(preview.matches('X').count(), 250);
    assert!(review_fits(
        ReviewKind::Review,
        VoucherCount::new(1).unwrap(),
        &preview
    ));
    // Fifty vouchers of every type a native post sends, over several dates.
    let mut mixed = serde_json::to_value(receipts(50, "BRIDGE CONCUR LAB")).unwrap();
    for (index, voucher) in mixed["vouchers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        voucher["voucher_type"] = json!(["Receipt", "Payment", "Journal", "Contra"][index % 4]);
        voucher["date"] = json!(format!("202504{:02}", 1 + index % 28));
    }
    let mixed: ImportLedgerLine = serde_json::from_value(mixed).unwrap();
    let preview = review_preview(&mixed, Refused, &lab);
    assert!(
        preview.replace('\n', " ").contains("Sent to Tally: 50 vouchers (13 Receipt, 13 Payment, 12 Journal, 12 Contra), dated 20250401 to 20250428."),
        "{preview}"
    );
    assert!(review_fits(ReviewKind::Review, fifty, &preview));
}

/// The one way to record the review is a person's click on the review box:
/// declined, nothing is written; approved, the record is journaled, survives
/// a restart, is reported by `verify_import` beside an unchanged verdict and
/// counted in its own bucket, and a second is refused before any read.
#[tokio::test]
async fn a_review_needs_the_person_and_is_reported_beside_the_verdict() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 3);
    let line = saved(&server, "Journal");
    sent_unanswered(&server, &line);
    assert_eq!(settled(&server).unsettled_no_response, 1);

    let declined = ScriptedApproval::declining();
    let response = release(&server, review_args(&line), declined.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_review_declined",
        "{response}"
    );
    assert_eq!(declined.review_kinds(), [ReviewKind::Review]);
    assert_eq!(review_records(&server), Vec::<Value>::new());

    let approval = ScriptedApproval::approving();
    let response = release(&server, review_args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "content_reviewed", "{response}");
    assert_eq!(
        result(&response)["reviewed_binding"],
        "response_not_recorded"
    );
    assert_eq!(
        result(&response)["verification_status"],
        "verification_incomplete"
    );
    assert_eq!(approval.review_kinds(), [ReviewKind::Review]);
    assert_eq!(approval.review_counts(), [1]);
    let shown = &approval.reviews()[0];
    assert!(
        shown.starts_with("THIS RECORDS ONLY THAT YOU CHECKED"),
        "{shown}"
    );
    assert!(
        shown
            .replace('\n', " ")
            .contains("it has no readable answer from Tally to the post."),
        "{shown}"
    );
    let records = review_records(&server);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["reviewed_binding"], "response_not_recorded");
    assert_eq!(records[0]["batch_sha256"], line.sha256);
    let counted = settled(&server);
    assert_eq!(
        (
            counted.unsettled,
            counted.unsettled_no_response,
            counted.unsettled_reviewed
        ),
        (1, 0, 1)
    );

    // Reported by the next verification, whose verdict is unchanged.
    let restarted = server_at(_simulator.address(), directory.path());
    let verified = restarted
        .call_tool(
            "verify_import",
            json!({"company_guid":GUID,"batch_id":line.batch_id}),
        )
        .await;
    let verified = result(&verified);
    assert_eq!(verified["verification_status"], "verification_incomplete");
    assert_eq!(verified["post_span_binding"]["state"], "not_bound");
    assert_eq!(
        verified["content_review"],
        json!({"state":"recorded","reviewed_binding":"response_not_recorded",
            "next_step":"A person recorded that they checked this batch's vouchers in Tally. ComplyEaze Bridge still cannot tell which Tally vouchers this post created: the batch is not marked posted and stays open, and its rows are not posted again."})
    );

    // A second review is refused before any read or dialog.
    let again = ScriptedApproval::approving();
    let response = release(&restarted, review_args(&line), again.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_already_recorded",
        "{response}"
    );
    assert!(again.reviews().is_empty());
    assert_eq!(review_records(&restarted).len(), 1);
}

/// A refused binding is reviewed the same way; the dialog names its code.
/// The outcome is whole, and names the batch's company for the egress record.
#[tokio::test]
async fn a_refused_binding_is_reviewed_with_its_code_shown() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 1);
    let line = saved(&server, "Journal");
    refused_with(&server, &line, "span_step_not_created");
    assert_eq!(settled(&server).unsettled_binding_refused, 1);
    let approval = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(
            approval.clone(),
            server.call_tool_response(ACK, review_args(&line)),
        )
        .await;
    assert_eq!(response.egress.company_guid.as_deref(), Some(GUID));
    let response = response.value;
    assert_eq!(response["structuredContent"]["truncated"], false);
    assert_eq!(result(&response)["state"], "content_reviewed", "{response}");
    assert_eq!(result(&response)["reviewed_binding"], "refused");
    assert!(approval.reviews()[0].contains("was refused (span_step_not_created)."));
    let counted = settled(&server);
    assert_eq!(
        (
            counted.unsettled_binding_refused,
            counted.unsettled_reviewed
        ),
        (0, 1)
    );
}

/// Only a batch whose journal holds a content-only state can be reviewed,
/// and the others are refused before any read or dialog: a batch never sent,
/// one whose answer is on file with no binding decided, one refused as
/// `span_identity_reused`, one that reads verified, and one sent with no pre-POST
/// mark.
#[tokio::test]
async fn only_a_batch_matched_by_content_only_can_be_reviewed() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let built = saved(&server, "Journal");
    let answered = saved(&server, "Journal");
    posted(&server, &answered);
    let reused = saved(&server, "Journal");
    refused_with(&server, &reused, "span_identity_reused");
    let verified = saved(&server, "Journal");
    refused_with(&server, &verified, "span_step_not_created");
    status(&server, &verified, "posted_verified");
    let unmarked = saved(&server, "Journal");
    {
        let native =
            native_post_request(&unmarked, RemoteIds::from_ids(vec![Uuid::new_v4()])).unwrap();
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::dispatch_for(
                &unmarked, &native, None,
            ))
            .unwrap();
    }
    for (args, code) in [
        (review_args(&built), "ack_batch_not_posted"),
        (
            review_args(&answered),
            "ack_content_review_not_content_only",
        ),
        (review_args(&reused), "ack_content_review_not_content_only"),
        (
            review_args(&verified),
            "ack_content_review_not_content_only",
        ),
        (
            review_args(&unmarked),
            "ack_content_review_not_content_only",
        ),
    ] {
        let approval = ScriptedApproval::approving();
        let response = release(&server, args.clone(), approval.clone()).await;
        assert_eq!(
            result(&response)["error"]["code"],
            code,
            "{args}: {response}"
        );
        assert!(approval.reviews().is_empty(), "no dialog: {response}");
    }
    assert_eq!(review_records(&server), Vec::<Value>::new());
}

/// The read the dialog would show must show the journal's state: here the
/// book's voucher mark reads below the post's, a rollback, so nothing is
/// asked or written.
#[tokio::test]
async fn a_read_that_shows_the_book_rolled_back_asks_nothing() {
    let directory = tempfile::tempdir().unwrap();
    let plans = span_readback(untagged_posted_journal(), 5);
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = saved(&server, "Journal");
    sent_unanswered(&server, &line);
    let approval = ScriptedApproval::approving();
    let response = release(&server, review_args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_content_review_not_content_only",
        "{response}"
    );
    assert!(approval.reviews().is_empty(), "no dialog: {response}");
    assert_eq!(review_records(&server), Vec::<Value>::new());
    // The refusal carries the evidence of the read it was refused on.
    assert!(
        response["structuredContent"]["evidence"]["bytes"]
            .as_u64()
            .unwrap()
            > 0,
        "{response}"
    );
}

/// What the person was shown must still hold when the record is written: a
/// batch that reads verified, whose answer is recorded, or that a later
/// verification records again while the dialog is open, or that another
/// batch's check finds rolled back, is refused and nothing is written.
#[tokio::test]
async fn a_change_while_the_dialog_is_open_writes_nothing() {
    for change in ["verified", "answered", "verified again", "rolled back"] {
        let directory = tempfile::tempdir().unwrap();
        let (_simulator, server) = server_reading(directory.path(), 1);
        let server = std::sync::Arc::new(server);
        let line = saved(&server, "Journal");
        let request = sent_unanswered(&server, &line);
        // Another batch of the same company and address, whose check reads
        // the mark below this batch's floor.
        let seer = saved(&server, "Journal");
        sent_unanswered(&server, &seer);
        let (other, other_line) = (std::sync::Arc::clone(&server), line.clone());
        let approval = ScriptedApproval::approving_after(move || match change {
            "verified" => status(&other, &other_line, "posted_verified"),
            "answered" => answered_late(&other, &other_line, &request),
            "verified again" => status(&other, &other_line, "verification_incomplete"),
            _ => {
                let journal = other.settings.data_dir.join("agent-import-ledger.jsonl");
                let records = fs::read_to_string(journal).unwrap().lines().count() as u64;
                let _lock = other.lock_import_admission().unwrap();
                other
                    .append_import_record_while_admitted(&ledger::StatusRecord::rollback_seen(
                        &seer, 1, records,
                    ))
                    .unwrap();
            }
        });
        let response = release(&server, review_args(&line), approval.clone()).await;
        assert_eq!(
            result(&response)["error"]["code"],
            "ack_changed_while_reviewing",
            "{change}: {response}"
        );
        assert_eq!(approval.review_counts(), [1], "{change}");
        assert_eq!(review_records(&server), Vec::<Value>::new(), "{change}");
    }
}

/// The state the journal holds decides admission before any read, and the
/// lab's own shape (a refusal journaled after the answer) is one of them.
#[test]
fn the_journaled_state_is_read_from_the_batch_s_records() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_without_tally(directory.path());
    let line = saved(&server, "Journal");
    let snapshot = |server: &Server| {
        server
            .latest_import_snapshot(&line.batch_id)
            .unwrap()
            .unwrap()
    };
    assert_eq!(journaled_state(&snapshot(&server)), None);
    let request = sent_unanswered(&server, &line);
    assert_eq!(
        journaled_state(&snapshot(&server)),
        Some(ResponseNotRecorded)
    );
    answered_with_no_outcome(&server, &line, &request);
    assert_eq!(
        journaled_state(&snapshot(&server)),
        Some(ResponseNotRecorded),
        "an answer with no outcome"
    );
    answered_late(&server, &line, &request);
    assert_eq!(journaled_state(&snapshot(&server)), None, "an outcome");
    // A binding refused because another batch's voucher was bound in the
    // post's span is never admitted, even when the answer on file last is
    // one with no outcome.
    let reused = saved(&server, "Journal");
    let request = sent_unanswered(&server, &reused);
    answered_late(&server, &reused, &request);
    binding_refused(&server, &reused, ledger::IDENTITY_REUSED);
    answered_with_no_outcome(&server, &reused, &request);
    let reused = server
        .latest_import_snapshot(&reused.batch_id)
        .unwrap()
        .unwrap();
    assert_eq!(journaled_state(&reused), None, "span_identity_reused");
}

/// A lost answer recorded with no outcome is reviewed the same way, and the
/// review lifts no invoice stop and releases no row: the batch still stops
/// its company's invoices, and its rows still count as sent.
#[tokio::test]
async fn a_review_lifts_no_stop_and_releases_no_row() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 1);
    let line = saved(&server, "Sales");
    let request = sent_unanswered(&server, &line);
    answered_with_no_outcome(&server, &line, &request);
    assert_eq!(stop(&server), Some(line.batch_id.clone()));
    let approval = ScriptedApproval::approving();
    let response = release(&server, review_args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "content_reviewed", "{response}");
    assert_eq!(
        result(&response)["reviewed_binding"],
        "response_not_recorded"
    );
    assert_eq!(review_records(&server).len(), 1);
    assert_eq!(stop(&server), Some(line.batch_id.clone()));
    let journal =
        fs::File::open(server.settings.data_dir.join("agent-import-ledger.jsonl")).unwrap();
    assert_eq!(
        ledger::vouchers_already_posted(
            std::io::BufReader::new(journal),
            GUID,
            None,
            &line.vouchers
        )
        .unwrap(),
        Some(line.batch_id.clone())
    );
}

/// A verification that reads the book rolled back after the post reports no
/// review: the restore voids it.
#[tokio::test]
async fn a_verification_that_reads_the_book_rolled_back_reports_no_review() {
    let directory = tempfile::tempdir().unwrap();
    // The review's read on the book as posted, then a verification on the
    // book with its voucher mark below the post's.
    let mut plans = posted_readback();
    plans.extend(span_readback(untagged_posted_journal(), 5));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = saved(&server, "Journal");
    sent_unanswered(&server, &line);
    let response = release(&server, review_args(&line), ScriptedApproval::approving()).await;
    assert_eq!(result(&response)["state"], "content_reviewed", "{response}");
    let verified = server
        .call_tool(
            "verify_import",
            json!({"company_guid":GUID,"batch_id":line.batch_id}),
        )
        .await;
    let verified = result(&verified);
    assert_eq!(
        verified["post_span_binding"]["state"], "book_rolled_back",
        "{verified}"
    );
    assert_eq!(verified["content_review"], Value::Null, "{verified}");
}

/// A batch named under another company is refused before any request is
/// sent: the mismatch is the caller's to correct.
#[tokio::test]
async fn a_review_naming_another_company_is_refused_before_any_read() {
    let directory = tempfile::tempdir().unwrap();
    let (simulator, server) = server_reading(directory.path(), 1);
    let line = saved(&server, "Journal");
    sent_unanswered(&server, &line);
    let mut elsewhere = review_args(&line);
    elsewhere["company_guid"] = json!("00000000-0000-4000-8000-000000000002");
    let approval = ScriptedApproval::approving();
    let response = release(&server, elsewhere, approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "import_batch_company_mismatch",
        "{response}"
    );
    assert_eq!(simulator.received(), 0, "no request: {response}");
    assert!(approval.reviews().is_empty());
}

/// The dialog shows the batch's company by its GUID when the batch records
/// no company name, a character of the batch id that could mislead as `?`,
/// and no matching voucher when the read reports no count of them.
#[test]
fn the_review_dialog_shows_only_what_the_batch_and_the_read_hold() {
    let lab = lab_result();
    let preview = |line: &Value, result: &Value| {
        let line: ImportLedgerLine = serde_json::from_value(line.clone()).unwrap();
        review_preview(&line, Refused, result)
    };
    let shown = |preview: &str, opening: &str| {
        preview
            .lines()
            .find(|text| text.starts_with(opening))
            .map(str::to_string)
    };
    let mut line = serde_json::to_value(receipts(1, "")).unwrap();
    let unnamed = preview(&line, &lab);
    assert_eq!(
        shown(&unnamed, "Company: "),
        Some(format!("Company: {GUID}"))
    );
    line["company"] = Value::Null;
    let untupled = preview(&line, &lab);
    assert_eq!(
        shown(&untupled, "Company: "),
        Some(format!("Company: {GUID}"))
    );
    line["batch_id"] = json!("bridge-\u{202e}1");
    let misleading = preview(&line, &lab);
    assert_eq!(
        shown(&misleading, "Batch: "),
        Some("Batch: bridge-?1".to_string())
    );
    // A line one character over the limit is broken at its last space.
    let wide = receipts(1, &format!("{} {}", "A".repeat(45), "B".repeat(46)));
    let wide = review_preview(&wide, Refused, &lab);
    let company = wide
        .lines()
        .position(|text| text.starts_with("Company: "))
        .unwrap();
    assert_eq!(
        wide.lines().skip(company).take(2).collect::<Vec<_>>(),
        [format!("Company: {}", "A".repeat(45)), "B".repeat(46)]
    );
    let mut uncounted = lab.clone();
    uncounted["counts"] = Value::Null;
    let uncounted = preview(&line, &uncounted).replace('\n', " ");
    assert!(
        uncounted.contains("found, for 0 of the 1 voucher sent, a voucher in the book"),
        "{uncounted}"
    );
}

/// The review box is asked for as many vouchers as the batch holds: its size
/// and wording depend on the count.
#[tokio::test]
async fn the_review_box_is_asked_for_every_voucher_of_the_batch() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 1);
    let origin =
        crate::tally::connection::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let mut two = serde_json::to_value(batch("Journal", &origin)).unwrap();
    let mut second = two["vouchers"][0].clone();
    second["bridge_txn_id"] = json!("t2");
    two["vouchers"].as_array_mut().unwrap().push(second);
    two["txn_ids"] = json!(["t1", "t2"]);
    let line: ImportLedgerLine = serde_json::from_value(two).unwrap();
    server.append_import_ledger(&line).unwrap();
    sent_unanswered(&server, &line);
    let approval = ScriptedApproval::approving();
    let response = release(&server, review_args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "content_reviewed", "{response}");
    assert_eq!(approval.review_counts(), [2]);
    assert_eq!(review_records(&server).len(), 1);
}

/// A company GUID that differs from the batch's only in letter case names
/// the same company: the review is not refused as naming another one.
#[tokio::test]
async fn a_review_naming_the_company_guid_in_capitals_is_recorded() {
    let directory = tempfile::tempdir().unwrap();
    let (_simulator, server) = server_reading(directory.path(), 1);
    let origin =
        crate::tally::connection::canonical_loopback_origin(&server.settings.endpoint).unwrap();
    let lettered = "61c6de69-1748-461c-ad3f-162cb949df9f";
    let mut line = serde_json::to_value(batch("Journal", &origin)).unwrap();
    line["company_guid"] = json!(lettered);
    line["company"]["guid"] = json!(lettered);
    let line: ImportLedgerLine = serde_json::from_value(line).unwrap();
    server.append_import_ledger(&line).unwrap();
    sent_unanswered(&server, &line);
    let mut args = review_args(&line);
    args["company_guid"] = json!(lettered.to_ascii_uppercase());
    let approval = ScriptedApproval::approving();
    let response = release(&server, args, approval.clone()).await;
    assert_eq!(result(&response)["state"], "content_reviewed", "{response}");
    assert_eq!(approval.review_kinds(), [ReviewKind::Review]);
    assert_eq!(review_records(&server).len(), 1);
}
