//! A person's release of the rows of a native post whose book was rolled back
//! (#1038), through `acknowledge_post_review` with `doubt` `rolled_back`. Each
//! batch is posted as production posts it and read back verified, then read
//! on a later server below the mark the post left, as a later run of the app would
//! after a backup was restored over the company.
use super::*;
use crate::agent::agent_import::release::{release_preview, shown_rolled_back, NEXT_STEP};
use crate::tally::approved_import::test_seam::review_fits;
use crate::tally::approved_import::{ReviewKind, VoucherCount};

/// The captured Journal's one row.
const ROW: &str = "BRIDGE_MCP_LIVE_20260906_A1";

fn release_args(line: &ImportLedgerLine) -> Value {
    json!({"company_guid":GUID,"batch_id":line.batch_id,"doubt":"rolled_back"})
}

async fn release(server: &Server, args: Value, scripted: ScriptedApproval) -> Value {
    SCRIPTED_APPROVAL
        .scope(scripted, server.call_tool("acknowledge_post_review", args))
        .await
}

fn result(response: &Value) -> &Value {
    &response["structuredContent"]["result"]
}

/// The journal's records of `kind`, as written.
fn records_of(directory: &std::path::Path, kind: &str) -> Vec<Value> {
    String::from_utf8(journal(directory))
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .filter(|record| record["record_type"] == kind)
        .collect()
}

fn counted(directory: &std::path::Path) -> ledger::Settlement {
    ledger::settlement(std::io::Cursor::new(journal(directory))).unwrap()
}

fn origin(server: &Server) -> String {
    super::super::super::super::canonical_loopback_origin(&server.settings.endpoint).unwrap()
}

/// A batch posted on one server, and a later server on the same data folder.
pub(super) struct Restored {
    pub(super) directory: tempfile::TempDir,
    pub(super) server: Server,
    pub(super) simulator: SequenceSimulator,
    pub(super) line: ImportLedgerLine,
}

/// `reads` verifications of the book at mark 10, below the 11 the post left:
/// a backup restored over the company.
pub(super) fn rolled_back_reads(reads: usize) -> Vec<ScenarioPlan> {
    (0..reads)
        .flat_map(|_| span_readback(empty_collection(), 10))
        .collect()
}

/// The captured Journal posted and read back verified, its voucher mark 10
/// before and 11 after, then a later server on the same data folder whose
/// Tally answers `later`.
pub(super) async fn posted_then_read(later: Vec<ScenarioPlan>) -> Restored {
    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(company_marks(11, 50, "WR2 Unicode Lab")));
    plans.extend(span_readback(untagged_posted_journal(), 11));
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let line = saved_captured_line(&server);
    let posted = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool(
                "post_import",
                json!({"company_guid":GUID,"batch_id":line.batch_id}),
            ),
        )
        .await;
    assert_eq!(
        result(&posted)["dispatch"]["state"],
        "posted_verified",
        "{posted}"
    );
    let _ = sent(simulator);
    let simulator = SequenceSimulator::spawn(with_sentinel(later)).unwrap();
    let server = moved_to(&server, simulator.address(), directory.path());
    Restored {
        directory,
        server,
        simulator,
        line,
    }
}

/// A server on `address` over the same data folder, the journal's origin moved
/// to it: a dispatched batch verifies only on the origin it recorded.
pub(super) fn moved_to(
    from: &Server,
    address: std::net::SocketAddr,
    directory: &std::path::Path,
) -> Server {
    let server = server_at(address, directory);
    let path = directory.join("agent-import-ledger.jsonl");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replace(&origin(from), &origin(&server))).unwrap();
    server
}

/// Journals `line` as sent natively, its pre-POST mark `mark` (none: a post
/// recorded before that mark was).
fn sent_natively(server: &Server, line: &ImportLedgerLine, mark: Option<u64>) {
    let record = match mark {
        Some(mark) => {
            let ids = line.vouchers.iter().map(|_| Uuid::new_v4()).collect();
            let native = native_post_request(line, RemoteIds::from_ids(ids)).unwrap();
            ledger::StatusRecord::dispatch_for(line, &native, Some(mark))
        }
        None => ledger::StatusRecord::dispatch_native(line, "c".repeat(64), Uuid::new_v4()),
    };
    let _lock = server.lock_import_admission().unwrap();
    server.append_import_record_while_admitted(&record).unwrap();
}

/// A batch whose book reads rolled back is released only by the person, in
/// the release dialog: a decline writes nothing, and a yes writes the one
/// record, on the cover the read recorded. The batch then moves from
/// `not_settled_restored` to `released_by_person`, `verify_import` reports the
/// release beside an unchanged `book_rolled_back`, a second release is
/// refused before any read, and the batch no longer holds its row: another
/// batch of it, refused before the release, then goes on to Tally.
#[tokio::test]
async fn a_rolled_back_batch_s_rows_are_released_only_by_the_person() {
    let Restored {
        directory,
        server,
        simulator,
        line,
    } = posted_then_read(rolled_back_reads(3)).await;
    let again = saved_line_as(&server, &format!("bridge-{}", Uuid::new_v4()), ROW);
    let again_args = json!({"company_guid":GUID,"batch_id":again.batch_id});
    let refused = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", again_args.clone()),
        )
        .await;
    assert_eq!(
        result(&refused)["error"]["code"],
        "import_txn_already_posted",
        "{refused}"
    );
    assert_eq!(
        result(&refused)["error"]["blocking_batch_id"],
        line.batch_id.as_str(),
        "{refused}"
    );

    let declined = ScriptedApproval::declining();
    let response = release(&server, release_args(&line), declined.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_review_declined",
        "{response}"
    );
    assert_eq!(declined.review_kinds(), [ReviewKind::RowRelease]);
    assert_eq!(
        records_of(directory.path(), "row_release"),
        Vec::<Value>::new()
    );
    // The read the dialog showed recorded the restore.
    let seen = records_of(directory.path(), "rollback_seen");
    assert_eq!(seen.len(), 1, "{seen:?}");
    assert_eq!(seen[0]["mark_read"], 10);
    let before = counted(directory.path());
    assert_eq!(
        (
            before.unsettled,
            before.unsettled_restored,
            before.released_by_person
        ),
        (1, 1, 0),
        "{before:?}"
    );

    // The company's GUID is matched without regard to case.
    let mut upper = release_args(&line);
    upper["company_guid"] = json!(GUID.to_uppercase());
    let approval = ScriptedApproval::approving();
    let response = SCRIPTED_APPROVAL
        .scope(
            approval.clone(),
            server.call_tool_response("acknowledge_post_review", upper),
        )
        .await;
    assert_eq!(
        response.egress.company_guid.as_deref(),
        Some(line.company_guid.as_str())
    );
    let response = response.value;
    let released = result(&response);
    assert_eq!(released["state"], "rows_released", "{response}");
    assert_eq!(released["batch_id"], line.batch_id.as_str(), "{response}");
    assert_eq!(released["next_step"], NEXT_STEP, "{response}");
    assert_ne!(
        released["verification_status"], "posted_verified",
        "{response}"
    );
    let status = released["verification_status"].clone();
    assert!(status.is_string(), "{response}");
    assert_eq!(approval.review_kinds(), [ReviewKind::RowRelease]);
    assert_eq!(approval.review_counts(), [1]);
    let listed = super::super::listed_voucher_lines(&line).unwrap().unwrap();
    assert_eq!(approval.reviews(), [release_preview(&line, 10, &listed)]);
    let records = records_of(directory.path(), "row_release");
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(records[0]["batch_id"], line.batch_id.as_str());
    assert_eq!(records[0]["status"], "rows_released");
    let cover = &records[0]["release_cover"];
    assert_eq!(cover["mark_read"], seen[0]["mark_read"]);
    assert_eq!(cover["seen_by"], seen[0]["batch_id"]);
    assert_eq!(
        cover["company"],
        serde_json::to_value(&line.company).unwrap()
    );
    assert_eq!(cover["endpoint_origin"], origin(&server));
    let after = counted(directory.path());
    assert_eq!(
        (
            after.unsettled,
            after.unsettled_restored,
            after.released_by_person,
            after.settled_by_person
        ),
        (0, 0, 1, 0),
        "{after:?}"
    );

    // A second release is refused before any read or dialog: the one read
    // left is the verification below.
    let twice = ScriptedApproval::approving();
    let response = release(&server, release_args(&line), twice.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_already_recorded",
        "{response}"
    );
    assert!(twice.reviews().is_empty());

    // Reported by the next verification, whose state is unchanged.
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
    assert_eq!(
        verified["row_release"],
        json!({"state": "rows_released", "next_step": NEXT_STEP}),
        "{verified}"
    );
    assert_ne!(
        verified["verification_status"], "posted_verified",
        "{verified}"
    );
    assert_eq!(verified["verification_status"], status, "{verified}");
    let _ = sent(simulator);

    // The row is no longer held: the other batch of it goes on to Tally.
    let journal_file = fs::File::open(directory.path().join("agent-import-ledger.jsonl")).unwrap();
    assert_eq!(
        ledger::vouchers_already_posted(
            std::io::BufReader::new(journal_file),
            GUID,
            None,
            &line.vouchers
        )
        .unwrap(),
        None
    );
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let server = moved_to(&server, simulator.address(), directory.path());
    let response = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool("post_import", again_args),
        )
        .await;
    assert_ne!(
        result(&response)["error"]["code"],
        "import_txn_already_posted",
        "{response}"
    );
    assert!(!sent(simulator).is_empty(), "{response}");
}

/// After a restore the next voucher created can take a removed voucher's GUID
/// (one lab run on 7.1). So the fresh post of a released batch's rows binds
/// the GUID the released batch had bound, and still reads `posted_verified`:
/// the GUIDs a bind refuses leave out those only a released batch bound.
#[tokio::test]
async fn a_fresh_post_of_released_rows_binds_the_guid_the_released_batch_bound() {
    let Restored {
        directory,
        server,
        simulator,
        line,
    } = posted_then_read(rolled_back_reads(1)).await;
    let released = release(&server, release_args(&line), ScriptedApproval::approving()).await;
    assert_eq!(result(&released)["state"], "rows_released", "{released}");
    let _ = sent(simulator);

    let mut plans = before_approval();
    plans.extend(after_approval(xml(created_one())));
    plans.push(xml(company_marks(11, 50, "WR2 Unicode Lab")));
    plans.extend(span_readback(untagged_posted_journal(), 11));
    plans.extend(probe());
    let simulator = SequenceSimulator::spawn(with_sentinel(plans)).unwrap();
    let server = moved_to(&server, simulator.address(), directory.path());
    let fresh = saved_line_as(&server, &format!("bridge-{}", Uuid::new_v4()), ROW);
    let posted = SCRIPTED_APPROVAL
        .scope(
            ScriptedApproval::approving(),
            server.call_tool(
                "post_import",
                json!({"company_guid":GUID,"batch_id":fresh.batch_id}),
            ),
        )
        .await;
    let _ = sent(simulator);
    let guids = |batch_id: &str| -> Vec<Value> {
        records_of(directory.path(), "post_span_verdict")
            .iter()
            .filter(|verdict| verdict["batch_id"] == batch_id)
            .flat_map(|verdict| verdict["bindings"].as_array().cloned().unwrap_or_default())
            .map(|bound| bound["guid"].clone())
            .collect()
    };
    let bound = guids(&line.batch_id);
    assert_eq!(bound.len(), 1, "the released batch was bound");
    assert_eq!(guids(&fresh.batch_id), bound, "{posted}");
    assert_eq!(
        result(&posted)["dispatch"]["state"],
        "posted_verified",
        "{posted}"
    );
}

async fn refused_before_any_read(server: &Server, args: Value, code: &str) {
    let approval = ScriptedApproval::approving();
    let response = release(server, args, approval.clone()).await;
    assert_eq!(result(&response)["error"]["code"], code, "{response}");
    assert!(approval.reviews().is_empty(), "{response}");
}

/// Refused before any read or dialog, and nothing is recorded: a batch never
/// sent, one sent with no vouchers, one named under another company, one whose post recorded no
/// pre-POST mark (no rollback record can cover it), one holding a Contra, one
/// holding a Journal and then a Contra (every voucher must be admitted, not
/// one), a Sales invoice, and one of more vouchers than the dialog lists. No
/// request is sent.
#[tokio::test]
async fn a_batch_a_release_cannot_rest_on_is_refused_before_any_read() {
    let directory = tempfile::tempdir().unwrap();
    let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
    let server = server_at(simulator.address(), directory.path());
    // With no journal yet, no release stands.
    assert_eq!(server.import_row_release("bridge-none"), Ok(false));
    let fresh = || format!("bridge-{}", Uuid::new_v4());

    let unsent = saved_line_as(&server, &fresh(), ROW);
    refused_before_any_read(&server, release_args(&unsent), "ack_batch_not_posted").await;

    let marked = saved_line_as(&server, &fresh(), ROW);
    sent_natively(&server, &marked, Some(10));
    let mut elsewhere = release_args(&marked);
    elsewhere["company_guid"] = json!("ffffffff-0000-4000-8000-00000000ffff");
    refused_before_any_read(&server, elsewhere, "import_batch_company_mismatch").await;

    let empty = saved_captured_line_edited(&server, |line| {
        line.batch_id = fresh();
        line.vouchers.clear();
        line.txn_ids.clear();
    });
    sent_natively(&server, &empty, None);
    refused_before_any_read(&server, release_args(&empty), "ack_batch_not_posted").await;

    let unmarked = saved_line_as(&server, &fresh(), ROW);
    sent_natively(&server, &unmarked, None);
    refused_before_any_read(
        &server,
        release_args(&unmarked),
        "ack_rolled_back_not_established",
    )
    .await;

    let contra = saved_captured_line_edited(&server, |line| {
        line.batch_id = fresh();
        line.vouchers[0].voucher_type = VoucherType::Contra;
    });
    sent_natively(&server, &contra, Some(10));
    refused_before_any_read(
        &server,
        release_args(&contra),
        "ack_release_type_not_admitted",
    )
    .await;

    let mixed = saved_captured_line_edited(&server, |line| {
        line.batch_id = fresh();
        let mut contra = line.vouchers[0].clone();
        contra.bridge_txn_id = "BRIDGE_RELEASE_MIXED_CONTRA".into();
        contra.voucher_type = VoucherType::Contra;
        line.txn_ids.push(contra.bridge_txn_id.clone());
        line.vouchers.push(contra);
    });
    assert_eq!(mixed.vouchers[0].voucher_type, VoucherType::Journal);
    sent_natively(&server, &mixed, Some(10));
    refused_before_any_read(
        &server,
        release_args(&mixed),
        "ack_release_type_not_admitted",
    )
    .await;

    let sales = saved_captured_line_edited(&server, |line| {
        line.batch_id = fresh();
        line.vouchers[0].voucher_type = VoucherType::Sales;
    });
    sent_natively(&server, &sales, Some(10));
    refused_before_any_read(
        &server,
        release_args(&sales),
        "ack_release_type_not_admitted",
    )
    .await;

    let (eleven, _) = saved_journal_batch(&server, &["12.61"; 11]);
    sent_natively(&server, &eleven, Some(10));
    refused_before_any_read(
        &server,
        release_args(&eleven),
        "ack_release_vouchers_not_listed",
    )
    .await;

    assert_eq!(sent(simulator).len(), 0);
    assert_eq!(
        records_of(directory.path(), "row_release"),
        Vec::<Value>::new()
    );
}

/// A read that finds the book where the post left it asks nothing and
/// records nothing.
#[tokio::test]
async fn a_read_that_does_not_show_the_book_rolled_back_asks_nothing() {
    let Restored {
        directory,
        server,
        simulator: _simulator,
        line,
    } = posted_then_read(span_readback(untagged_posted_journal(), 11)).await;
    let approval = ScriptedApproval::approving();
    let response = release(&server, release_args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_rolled_back_not_established",
        "{response}"
    );
    assert!(approval.reviews().is_empty());
    assert_eq!(
        records_of(directory.path(), "row_release"),
        Vec::<Value>::new()
    );
}

/// What the person was shown must still hold when the record is written: a
/// batch that reads verified while the dialog is open, whose cover another
/// batch's check moves to a lower restore, or that another verification
/// reads again (a record the journal would still admit a release after), is
/// refused, and nothing is written.
#[tokio::test]
async fn a_change_while_the_dialog_is_open_writes_nothing() {
    for change in ["verified", "rolled back lower", "read again"] {
        let Restored {
            directory,
            server,
            simulator: _simulator,
            line,
        } = posted_then_read(rolled_back_reads(1)).await;
        // Another batch of the same company and address, sent after this one,
        // whose check reads the mark below this batch's cover.
        let mut seer = line.clone();
        seer.batch_id = format!("bridge-{}", Uuid::new_v4());
        seer.endpoint_origin = Some(origin(&server));
        server.append_import_ledger(&seer).unwrap();
        sent_natively(&server, &seer, Some(10));
        let ledger_path = directory.path().join("agent-import-ledger.jsonl");
        let server = std::sync::Arc::new(server);
        let (other, other_line) = (std::sync::Arc::clone(&server), line.clone());
        let approval = ScriptedApproval::approving_after(move || {
            let records = fs::read_to_string(&ledger_path).unwrap().lines().count() as u64;
            let record = match change {
                "rolled back lower" => ledger::StatusRecord::rollback_seen(&seer, 1, records),
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
        let response = release(&server, release_args(&line), approval.clone()).await;
        assert_eq!(
            result(&response)["error"]["code"],
            "ack_changed_while_reviewing",
            "{change}: {response}"
        );
        assert_eq!(approval.review_counts(), [1], "{change}");
        assert_eq!(
            records_of(directory.path(), "row_release"),
            Vec::<Value>::new(),
            "{change}"
        );
    }
}

/// The dialog says first what the release does and does not do, then the
/// batch, the mark the cover's verification read (never that the book reads
/// it now) and every voucher, then what the person confirms. It fits the box
/// for one voucher and for ten; eleven are not listed, so never released; a
/// company name without spaces, or the largest mark, is broken to fit.
#[test]
fn the_release_dialog_lists_every_voucher_and_fits_the_box() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let line = saved_captured_line(&server);
    let listed = super::super::listed_voucher_lines(&line).unwrap().unwrap();
    assert_eq!(
        listed[..2],
        [
            super::super::VOUCHER_LINES_HEADING,
            super::super::VOUCHER_LINES_NARRATION_HEADING
        ]
    );
    assert_eq!(listed.len(), 3, "{listed:?}");
    assert!(
        listed[2].starts_with("Journal 7-Sep-2026  12.61"),
        "{listed:?}"
    );
    let preview = release_preview(&line, 10, &listed);
    assert_eq!(
        preview,
        format!(
            "THIS RELEASES THIS BATCH'S ROWS, SO THAT THEY CAN BE BUILT AND POSTED AGAIN.\n\
             It changes nothing in Tally and removes no record. It cannot be undone: a new batch of these rows\n\
             can post them again.\n\
             \n\
             Company: WR2 Unicode Lab\n\
             Batch: {}\n\
             Built 6-Sep-2026 and sent to Tally by ComplyEaze Bridge: 1 voucher.\n\
             A verification read this company's voucher mark at 10, below the mark this post left.\n\
             It may read higher now.\n\
             \n\
             {}\n\
             \n\
             Confirm only if you have checked in Tally that this voucher is not in the book, including keyed\n\
             again with other figures.",
            line.batch_id,
            listed.join("\n")
        )
    );
    assert!(review_fits(
        ReviewKind::RowRelease,
        VoucherCount::new(1).unwrap(),
        &preview
    ));

    let (ten, _) = saved_journal_batch(&server, &["12.61"; 10]);
    let listed_ten = super::super::listed_voucher_lines(&ten).unwrap().unwrap();
    assert_eq!(listed_ten.len(), 12, "{listed_ten:?}");
    let preview = release_preview(&ten, 10, &listed_ten);
    assert!(
        preview.contains("Built 6-Sep-2026 and sent to Tally by ComplyEaze Bridge: 10 vouchers."),
        "{preview}"
    );
    assert!(
        preview.ends_with(
            "Confirm only if you have checked in Tally that none of these vouchers is in the book, including any\n\
             keyed again with other figures."
        ),
        "{preview}"
    );
    assert!(review_fits(
        ReviewKind::RowRelease,
        VoucherCount::new(10).unwrap(),
        &preview
    ));
    let (eleven, _) = saved_journal_batch(&server, &["12.61"; 11]);
    assert_eq!(super::super::listed_voucher_lines(&eleven), Ok(None));

    let mut long = line.clone();
    long.company.as_mut().unwrap().name = "A".repeat(250);
    let preview = release_preview(&long, u64::MAX, &listed);
    assert!(
        preview.lines().all(|shown| shown.chars().count() <= 100),
        "{preview}"
    );
    assert!(review_fits(
        ReviewKind::RowRelease,
        VoucherCount::new(1).unwrap(),
        &preview
    ));
}

/// A Payment batch and a Receipt batch are admitted as a Journal batch is:
/// each goes on to the read, and is answered as the Journal batch is.
#[tokio::test]
async fn a_payment_or_receipt_batch_goes_on_to_the_read_as_a_journal_batch_does() {
    let mut codes = Vec::new();
    for voucher_type in [
        VoucherType::Journal,
        VoucherType::Payment,
        VoucherType::Receipt,
    ] {
        let directory = tempfile::tempdir().unwrap();
        let simulator = SequenceSimulator::spawn(with_sentinel(Vec::new())).unwrap();
        let server = server_at(simulator.address(), directory.path());
        let line = saved_captured_line_edited(&server, |line| {
            line.batch_id = format!("bridge-{}", Uuid::new_v4());
            line.vouchers[0].voucher_type = voucher_type.clone();
        });
        sent_natively(&server, &line, Some(10));
        let approval = ScriptedApproval::approving();
        let response = release(&server, release_args(&line), approval.clone()).await;
        assert!(approval.reviews().is_empty(), "{response}");
        assert!(!sent(simulator).is_empty(), "{voucher_type:?}: {response}");
        codes.push(result(&response)["error"]["code"].clone());
    }
    assert_ne!(codes[0], "ack_release_type_not_admitted", "{codes:?}");
    assert_eq!(codes[1], codes[0], "{codes:?}");
    assert_eq!(codes[2], codes[0], "{codes:?}");
}

/// A release another call writes while the dialog is open is refused by the
/// journal's own rule, so this one reads as a change and writes nothing more:
/// the journal holds the other call's one release.
#[tokio::test]
async fn a_release_written_while_the_dialog_is_open_is_not_written_twice() {
    let Restored {
        directory,
        server,
        simulator: _simulator,
        line,
    } = posted_then_read(rolled_back_reads(1)).await;
    let server = std::sync::Arc::new(server);
    let (other, other_line) = (std::sync::Arc::clone(&server), line.clone());
    let approval = ScriptedApproval::approving_after(move || {
        let cover = other
            .rollback_scan(&other_line.batch_id)
            .unwrap()
            .covered_by
            .unwrap();
        let record = ledger::StatusRecord::row_release(&other_line, &cover).unwrap();
        let _lock = other.lock_import_admission().unwrap();
        other.append_import_record_while_admitted(&record).unwrap();
    });
    let response = release(&server, release_args(&line), approval.clone()).await;
    assert_eq!(
        result(&response)["error"]["code"],
        "ack_changed_while_reviewing",
        "{response}"
    );
    assert_eq!(approval.review_counts(), [1]);
    assert_eq!(records_of(directory.path(), "row_release").len(), 1);
}

/// The dialog shows the mark the cover's verification read, not the mark
/// recorded before the post: here a read at 9, below the 10 before the post.
#[tokio::test]
async fn the_release_dialog_shows_the_mark_the_cover_read() {
    let Restored {
        directory: _directory,
        server,
        simulator: _simulator,
        line,
    } = posted_then_read(span_readback(empty_collection(), 9)).await;
    let approval = ScriptedApproval::approving();
    let response = release(&server, release_args(&line), approval.clone()).await;
    assert_eq!(result(&response)["state"], "rows_released", "{response}");
    let listed = super::super::listed_voucher_lines(&line).unwrap().unwrap();
    assert_eq!(approval.reviews(), [release_preview(&line, 9, &listed)]);
}

/// A read shows the book rolled back only when its binding reads
/// `book_rolled_back` and its status is not `posted_verified`.
#[test]
fn a_read_shows_the_book_rolled_back_only_by_its_binding_and_its_status() {
    let read = |status: &str, state: &str| {
        shown_rolled_back(&json!({
            "verification_status": status,
            "post_span_binding": {"state": state},
        }))
    };
    assert!(read("reconciliation_required", "book_rolled_back"));
    assert!(!read("posted_verified", "book_rolled_back"));
    assert!(!read("reconciliation_required", "bound"));
    assert!(!read("posted_verified", "bound"));
    assert!(!shown_rolled_back(&json!({})));
}

/// The dialog names the company by its GUID when the batch recorded no name
/// or an empty one, and shows a character that could reorder what the person
/// reads, in the batch id, as `?`.
#[test]
fn the_release_dialog_names_a_nameless_company_by_its_guid_and_shows_no_reordering_character() {
    let directory = tempfile::tempdir().unwrap();
    let server = server_at("127.0.0.1:9".parse().unwrap(), directory.path());
    let line = saved_captured_line(&server);
    let listed = super::super::listed_voucher_lines(&line).unwrap().unwrap();
    let named_by_guid = format!("\nCompany: {}\n", line.company_guid);

    let mut unnamed = line.clone();
    unnamed.company = None;
    let preview = release_preview(&unnamed, 10, &listed);
    assert!(preview.contains(&named_by_guid), "{preview}");

    let mut empty = line.clone();
    empty.company.as_mut().unwrap().name = String::new();
    let preview = release_preview(&empty, 10, &listed);
    assert!(preview.contains(&named_by_guid), "{preview}");

    let mut reordering = line.clone();
    reordering.batch_id = "bridge-\u{202E}a".into();
    let preview = release_preview(&reordering, 10, &listed);
    assert!(preview.contains("\nBatch: bridge-?a\n"), "{preview}");
    assert!(!preview.contains('\u{202E}'), "{preview}");
}
