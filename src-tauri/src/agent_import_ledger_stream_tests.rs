//! Local journal tests; no authored Tally wire responses.
use super::*;
use std::cell::Cell;
use std::io::{BufReader, Cursor, Read};
use std::rc::Rc;

fn batch(id: &str, narration: &str) -> ImportLedgerLine {
    serde_json::from_value(json!({
        "batch_id":id,"company_guid":"synthetic-guid","company":null,
        "txn_ids":["txn"],"date_from":"20260901","date_to":"20260901",
        "sha256":"a".repeat(64),"built_at":"2026-09-07T00:00:00Z","status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":1,"master_value":1},
        "vouchers":[{"bridge_txn_id":"txn","date":"20260901","voucher_type":"Journal",
            "narration":narration,"reference":null,"voucher_number":null,"entries":[
                {"ledger":"Cash","amount":"1","side":"Dr"},
                {"ledger":"Synthetic Ledger","amount":"1","side":"Cr"}]}]
    }))
    .unwrap()
}

fn record(value: &impl Serialize) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(value).unwrap();
    bytes.push(b'\n');
    bytes
}

/// Generates one physical record at a time and forbids whole-stream Read APIs.
struct Records<I> {
    records: I,
    current: Vec<u8>,
    offset: usize,
    consumed: Rc<Cell<usize>>,
    largest_record: Rc<Cell<usize>>,
}

impl<I: Iterator<Item = Vec<u8>>> Read for Records<I> {
    fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
        panic!("journal admission must consume bounded records through BufRead")
    }
}

impl<I: Iterator<Item = Vec<u8>>> BufRead for Records<I> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.offset == self.current.len() {
            self.current = self.records.next().unwrap_or_default();
            self.offset = 0;
            self.largest_record
                .set(self.largest_record.get().max(self.current.len()));
        }
        Ok(&self.current[self.offset..])
    }
    fn consume(&mut self, amount: usize) {
        self.offset += amount;
        self.consumed.set(self.consumed.get() + amount);
    }
}

#[test]
fn streaming_admission_retains_only_target_payload_and_latest_physical_generation() {
    let target = batch("target", &"target narration ".repeat(50_000));
    let mut updated = batch("target", "latest payload");
    updated.status = "posted_verified".into();
    let large_unrelated = batch("unrelated", &"unrelated narration ".repeat(2_000));
    let full_records = 1000;
    let status_records = 20_000;
    let mut physical = 0;
    let rows = std::iter::from_fn(|| {
        let bytes = match physical {
            0 => record(&target),
            i if i <= full_records => record(&large_unrelated),
            i if i == full_records + 1 => record(&updated),
            i if i <= full_records + 1 + status_records => record(&StatusRecord::from(&updated)),
            i if i == full_records + 2 + status_records => record(&large_unrelated),
            _ => return None,
        };
        physical += 1;
        Some(bytes)
    });
    let consumed = Rc::new(Cell::new(0));
    let largest_record = Rc::new(Cell::new(0));
    let reader = Records {
        records: rows,
        current: Vec::new(),
        offset: 0,
        consumed: consumed.clone(),
        largest_record: largest_record.clone(),
    };
    let selected = read_snapshot(reader, Some("target")).unwrap().unwrap();
    assert!(consumed.get() > MAX_RECORD_BYTES);
    assert!(largest_record.get() < 1_000_000);
    assert_eq!(
        selected.batch.vouchers[0].narration.as_deref(),
        Some("latest payload")
    );
    assert_eq!(selected.batch.status, "posted_verified");
    assert!(selected.generation == VerificationGeneration(full_records + 1 + status_records));
}

#[test]
fn streaming_admission_checks_corruption_after_target_and_unrelated_status_bindings() {
    let target = batch("target", "target");
    let unrelated = batch("unrelated", "other");
    let prefix = [record(&target), record(&unrelated)].concat();
    for ending in [
        b"{\"batch_id\":".to_vec(),
        b"\n".to_vec(),
        record(
            &json!({"record_type":"verification_status","batch_id":"missing",
            "batch_sha256":unrelated.sha256,"status":"posted_verified"}),
        ),
        record(
            &json!({"record_type":"verification_status","batch_id":"unrelated",
            "batch_sha256":"wrong","status":"posted_verified"}),
        ),
        record(&json!({"record_type":"unknown"})),
    ] {
        let bytes = [prefix.clone(), ending].concat();
        for wanted in [None, Some("target"), Some("missing")] {
            assert_eq!(
                read_snapshot(Cursor::new(&bytes), wanted).err(),
                Some("import_ledger_invalid".into())
            );
        }
    }
    let invalid_utf8 = [prefix.clone(), vec![0xff, b'\n']].concat();
    assert_eq!(
        read_snapshot(Cursor::new(invalid_utf8), Some("target")).err(),
        Some("import_ledger_unavailable".into())
    );
    // A complete JSON object is still an incomplete JSONL append without LF.
    let final_record = serde_json::to_vec(&target).unwrap();
    assert_eq!(
        read_snapshot(Cursor::new(final_record), Some("target")).err(),
        Some("import_ledger_invalid".into())
    );
    assert!(read_snapshot(Cursor::new(Vec::<u8>::new()), None)
        .unwrap()
        .is_none());
    assert!(read_snapshot(Cursor::new(record(&target)), Some("target"))
        .unwrap()
        .is_some());
}

#[test]
fn oversized_record_stops_at_the_line_bound_without_reading_the_remaining_stream() {
    let mut reader = BufReader::with_capacity(8192, std::io::repeat(b' '));
    let mut line = Vec::new();
    assert_eq!(
        read_record(&mut reader, &mut line).err(),
        Some("import_ledger_record_too_large".into())
    );
    assert_eq!(line.len(), MAX_RECORD_BYTES);
    assert!(line.capacity() <= MAX_RECORD_BYTES);
    // The next chunk stays unread; there is no unbounded read-to-end fallback.
    assert_eq!(reader.buffer().len(), 8192);
}

#[test]
fn dispatch_admission_survives_verification_and_legacy_full_records() {
    let initial = batch("dispatch-test", "local journal test");
    let mut bytes = record(&initial);
    bytes.extend(record(&StatusRecord::dispatch(&initial)));
    let mut verified = initial.clone();
    verified.status = "verification_incomplete".into();
    bytes.extend(record(&StatusRecord::from(&verified)));
    // Legacy full records must not erase a previous attempt either.
    bytes.extend(record(&verified));
    let snapshot = read_snapshot(Cursor::new(bytes.clone()), Some("dispatch-test"))
        .unwrap()
        .unwrap();
    assert!(snapshot.dispatched);
    assert_eq!(snapshot.batch.status, "verification_incomplete");
    bytes.extend(record(&StatusRecord::dispatch(&initial)));
    assert_eq!(
        read_snapshot(Cursor::new(bytes), Some("dispatch-test"))
            .err()
            .as_deref(),
        Some("import_ledger_duplicate_dispatch")
    );
}

#[test]
fn dispatched_batch_cannot_change_its_commitment() {
    let initial = batch("dispatch-test", "local journal test");
    let mut bytes = record(&initial);
    bytes.extend(record(&StatusRecord::dispatch(&initial)));
    let mut changed = initial;
    changed.sha256 = "b".repeat(64);
    bytes.extend(record(&changed));
    assert_eq!(
        read_snapshot(Cursor::new(bytes), None).err().as_deref(),
        Some("import_ledger_invalid")
    );
}

#[test]
fn native_dispatch_response_must_match_its_durable_wire_commitment() {
    let initial = batch("native-dispatch", "local journal test");
    let hash = "c".repeat(64);
    let intent = StatusRecord::dispatch_native(&initial, hash.clone(), Uuid::new_v4());
    let mut bytes = record(&initial);
    bytes.extend(record(&intent));
    for (response_hash, admitted) in [(hash, true), ("d".repeat(64), false)] {
        let response = StatusRecord::response(
            &initial,
            DispatchResponse {
                request_sha256: response_hash,
                response_sha256: "e".repeat(64),
                bytes: 1,
                outcome: None,
            },
        );
        let mut with_response = bytes.clone();
        with_response.extend(record(&response));
        let result = read_snapshot(Cursor::new(with_response), Some("native-dispatch"));
        if admitted {
            assert!(result.unwrap().unwrap().dispatched);
        } else {
            assert_eq!(result.err().as_deref(), Some("import_ledger_invalid"));
        }
    }
    for invalid in ["", "not-a-hash"] {
        let mut bytes = record(&initial);
        bytes.extend(record(&StatusRecord::dispatch_native(
            &initial,
            invalid.into(),
            Uuid::new_v4(),
        )));
        assert_eq!(
            read_snapshot(Cursor::new(bytes), None).err().as_deref(),
            Some("import_ledger_invalid")
        );
    }
    let mut misplaced = serde_json::to_value(StatusRecord::from(&initial)).unwrap();
    misplaced["native_request_sha256"] = serde_json::json!("c".repeat(64));
    let mut bytes = record(&initial);
    bytes.extend(record(&misplaced));
    assert_eq!(
        read_snapshot(Cursor::new(bytes), None).err().as_deref(),
        Some("import_ledger_invalid")
    );
}

/// bridge#579. The REMOTEID recorded with a native dispatch intent survives a
/// journal read, and only in that record, beside its request hash.
#[test]
fn a_native_dispatch_intent_keeps_its_remoteid_and_nothing_else_may_carry_one() {
    let initial = batch("native-remote", "local journal test");
    let hash = "c".repeat(64);
    let remote_id = Uuid::new_v4();
    let mut bytes = record(&initial);
    bytes.extend(record(&StatusRecord::dispatch_native(
        &initial,
        hash.clone(),
        remote_id,
    )));
    let snapshot = read_snapshot(Cursor::new(bytes.clone()), Some("native-remote"))
        .unwrap()
        .unwrap();
    assert!(snapshot.dispatched);
    assert_eq!(
        snapshot.native_remote_id.as_deref(),
        Some(remote_id.hyphenated().to_string().as_str())
    );
    let history = read_history(Cursor::new(bytes.clone())).unwrap();
    assert_eq!(
        history[0].native_remote_id.as_deref(),
        Some(remote_id.hyphenated().to_string().as_str())
    );
    let lineage = read_lineage(Cursor::new(bytes), "native-remote").unwrap();
    assert_eq!(lineage.len(), 1);
    assert_eq!(
        lineage[0].native_remote_id.as_deref(),
        Some(remote_id.hyphenated().to_string().as_str())
    );

    let mut intent =
        serde_json::to_value(StatusRecord::dispatch_native(&initial, hash, remote_id)).unwrap();
    for (field, value) in [
        ("native_remote_id", json!(remote_id.simple().to_string())),
        ("native_remote_id", json!("not-a-uuid")),
        (
            "native_remote_id",
            json!(Uuid::nil().hyphenated().to_string()),
        ),
        (
            "native_remote_id",
            json!(remote_id.hyphenated().to_string().to_uppercase()),
        ),
        ("native_request_sha256", Value::Null),
    ] {
        let mut changed = intent.clone();
        if value.is_null() {
            changed.as_object_mut().unwrap().remove(field);
        } else {
            changed[field] = value;
        }
        let mut bytes = record(&initial);
        bytes.extend(serde_json::to_vec(&changed).unwrap());
        bytes.push(b'\n');
        assert_eq!(
            read_snapshot(Cursor::new(bytes), None).err().as_deref(),
            Some("import_ledger_invalid"),
            "{changed}"
        );
    }
    intent.as_object_mut().unwrap().remove("native_remote_id");
    let mut legacy = record(&initial);
    legacy.extend(serde_json::to_vec(&intent).unwrap());
    legacy.push(b'\n');
    let snapshot = read_snapshot(Cursor::new(legacy), Some("native-remote"))
        .unwrap()
        .unwrap();
    assert!(snapshot.dispatched);
    assert_eq!(snapshot.native_remote_id, None);
}

/// The journal answers whether any intent records a REMOTEID, whichever batch
/// it belongs to. It does not refuse to read a journal that holds one twice:
/// that would stop every batch's reads, and the post path already refuses to
/// send a recorded REMOTEID as it writes the intent, under the exclusive lock.
#[test]
fn any_intent_recording_a_remoteid_is_found_and_a_repeat_does_not_block_reads() {
    let (first, second) = (batch("first", "local"), batch("second", "local"));
    let (shared, other) = (Uuid::new_v4(), Uuid::new_v4());
    let mut bytes = record(&first);
    bytes.extend(record(&second));
    assert!(!remote_ids_recorded(Cursor::new(bytes.clone()), &[shared]).unwrap());
    bytes.extend(record(&StatusRecord::dispatch_native(
        &first,
        "c".repeat(64),
        shared,
    )));
    assert!(remote_ids_recorded(Cursor::new(bytes.clone()), &[shared]).unwrap());
    assert!(!remote_ids_recorded(Cursor::new(bytes.clone()), &[other]).unwrap());

    bytes.extend(record(&StatusRecord::dispatch_native(
        &second,
        "c".repeat(64),
        shared,
    )));
    assert!(remote_ids_recorded(Cursor::new(bytes.clone()), &[shared]).unwrap());
    assert!(
        read_snapshot(Cursor::new(bytes), Some("second"))
            .unwrap()
            .unwrap()
            .dispatched
    );
}

/// A batch's dispatch intent records one REMOTEID per voucher: at least two,
/// at most the batch cap, distinct, canonical, beside the request hash and
/// never beside a single post's REMOTEID. The journal finds each of them, so
/// no later post, single or batch, can send one again.
#[test]
fn a_batch_intent_records_distinct_remoteids_that_the_journal_finds() {
    let mut initial = batch("native-batch", "local journal test");
    let mut second = initial.vouchers[0].clone();
    second.bridge_txn_id = "txn-2".into();
    initial.vouchers.push(second);
    let ids = [Uuid::new_v4(), Uuid::new_v4()];
    let mut intent = serde_json::to_value(StatusRecord::dispatch_native(
        &initial,
        "c".repeat(64),
        ids[0],
    ))
    .unwrap();
    intent.as_object_mut().unwrap().remove("native_remote_id");
    intent["native_remote_ids"] = json!(ids
        .iter()
        .map(|id| id.hyphenated().to_string())
        .collect::<Vec<_>>());
    let journal = |intent: &Value| {
        let mut bytes = record(&initial);
        bytes.extend(serde_json::to_vec(intent).unwrap());
        bytes.push(b'\n');
        bytes
    };
    let bytes = journal(&intent);
    assert!(
        read_snapshot(Cursor::new(bytes.clone()), Some("native-batch"))
            .unwrap()
            .unwrap()
            .dispatched
    );
    for id in ids {
        assert!(remote_ids_recorded(Cursor::new(bytes.clone()), &[id]).unwrap());
    }
    assert!(!remote_ids_recorded(Cursor::new(bytes), &[Uuid::new_v4()]).unwrap());

    let id = |uuid: Uuid| json!(uuid.hyphenated().to_string());
    let too_many = (0..=MAX_BATCH_POST_VOUCHERS)
        .map(|_| id(Uuid::new_v4()))
        .collect::<Vec<_>>();
    for (field, value) in [
        ("native_remote_ids", json!([id(ids[0])])),
        // Three ids for a batch of two vouchers.
        (
            "native_remote_ids",
            json!([id(ids[0]), id(ids[1]), id(Uuid::new_v4())]),
        ),
        ("native_remote_ids", json!([id(ids[0]), id(ids[0])])),
        ("native_remote_ids", json!(too_many)),
        (
            "native_remote_ids",
            json!([id(ids[0]), ids[1].simple().to_string()]),
        ),
        ("native_remote_ids", json!([id(ids[0]), id(Uuid::nil())])),
        ("native_remote_id", id(Uuid::new_v4())),
        ("native_request_sha256", Value::Null),
    ] {
        let mut changed = intent.clone();
        if value.is_null() {
            changed.as_object_mut().unwrap().remove(field);
        } else {
            changed[field] = value;
        }
        assert_eq!(
            read_snapshot(Cursor::new(journal(&changed)), None)
                .err()
                .as_deref(),
            Some("import_ledger_invalid"),
            "{changed}"
        );
    }
    // Only a dispatch intent may carry them.
    let mut status = serde_json::to_value(StatusRecord::from(&initial)).unwrap();
    status["native_request_sha256"] = json!("c".repeat(64));
    status["native_remote_ids"] = intent["native_remote_ids"].clone();
    assert_eq!(
        read_snapshot(Cursor::new(journal(&status)), None)
            .err()
            .as_deref(),
        Some("import_ledger_invalid")
    );
}

/// What a deletion would have to keep (#local-data): the settlement counts.
#[test]
fn settlement_counts_sent_and_unsettled_batches() {
    let response = |line: &ImportLedgerLine| {
        StatusRecord::response(
            line,
            DispatchResponse {
                request_sha256: "c".repeat(64),
                response_sha256: "d".repeat(64),
                bytes: 1,
                outcome: None,
            },
        )
    };
    let status = |id: &str, status: &str| {
        let mut line = batch(id, "n");
        line.status = status.into();
        StatusRecord::from(&line)
    };
    let mut bytes = Vec::new();
    // Only built: not sent, nothing to settle.
    bytes.extend(record(&batch("built", "n")));
    // Sent, no response recorded: unsettled.
    let sent = batch("sent", "n");
    bytes.extend(record(&sent));
    bytes.extend(record(&StatusRecord::dispatch(&sent)));
    // Sent, answered and verified: settled.
    let settled = batch("settled", "n");
    bytes.extend(record(&settled));
    bytes.extend(record(&StatusRecord::dispatch(&settled)));
    bytes.extend(record(&response(&settled)));
    bytes.extend(record(&status("settled", "posted_verified")));
    // Sent and answered but the readback was incomplete: unsettled.
    let incomplete = batch("incomplete", "n");
    bytes.extend(record(&incomplete));
    bytes.extend(record(&StatusRecord::dispatch(&incomplete)));
    bytes.extend(record(&response(&incomplete)));
    bytes.extend(record(&status("incomplete", "verification_incomplete")));
    // Verified with no response recorded: still unsettled.
    let unanswered = batch("unanswered", "n");
    bytes.extend(record(&unanswered));
    bytes.extend(record(&StatusRecord::dispatch(&unanswered)));
    bytes.extend(record(&status("unanswered", "posted_verified")));
    // A legacy full record already marked posted, never dispatched: found,
    // and nothing left to settle.
    let mut found = batch("found", "n");
    found.status = "posted_verified".into();
    bytes.extend(record(&found));
    // Verified by a hand import first, then posted natively: the dispatch
    // intent and response make the batch unverified again until it is read back.
    let hand_then_posted = batch("hand_then_posted", "n");
    bytes.extend(record(&hand_then_posted));
    bytes.extend(record(&status("hand_then_posted", "posted_verified")));
    bytes.extend(record(&StatusRecord::dispatch(&hand_then_posted)));
    bytes.extend(record(&response(&hand_then_posted)));
    // Found posted by a hand import, then a later readback reads incomplete,
    // never dispatched: still found, so the double-post memory of it counts.
    let found_then_incomplete = batch("found_then_incomplete", "n");
    bytes.extend(record(&found_then_incomplete));
    bytes.extend(record(&status("found_then_incomplete", "posted_verified")));
    bytes.extend(record(&status(
        "found_then_incomplete",
        "verification_incomplete",
    )));

    assert_eq!(
        settlement(Cursor::new(bytes)).unwrap(),
        Settlement {
            batches: 8,
            sent_or_found: 7,
            unsettled: 4,
            unsettled_no_response: 2,
            no_dispatch_never_verified: 1
        }
    );
    assert_eq!(
        settlement(Cursor::new(Vec::new())).unwrap(),
        Settlement {
            batches: 0,
            sent_or_found: 0,
            unsettled: 0,
            unsettled_no_response: 0,
            no_dispatch_never_verified: 0
        }
    );
    assert_eq!(
        settlement(Cursor::new(b"not json\n".to_vec()))
            .err()
            .as_deref(),
        Some("import_ledger_invalid")
    );
}

const STATEMENT_ID: &str = "st-20260901-0123456789abcdef";

/// One voucher, dated 1 Sep 2026, over `entries` (ledger, amount, side).
fn row_batch(
    id: &str,
    company: &str,
    txn_id: &str,
    sha: char,
    entries: &[(&str, &str, &str)],
) -> ImportLedgerLine {
    let mut line = batch(id, "n");
    line.company_guid = company.into();
    line.sha256 = sha.to_string().repeat(64);
    line.txn_ids = vec![txn_id.into()];
    line.vouchers[0].bridge_txn_id = txn_id.into();
    line.vouchers[0].entries = entries
        .iter()
        .map(|(ledger, amount, side)| {
            serde_json::from_value(json!({"ledger":ledger,"amount":amount,"side":side})).unwrap()
        })
        .collect();
    line
}

fn plain(id: &str, txn_id: &str, entries: &[(&str, &str, &str)]) -> ImportLedgerLine {
    row_batch(id, "synthetic-guid", txn_id, 'a', entries)
}

const BANK_TO_A: &[(&str, &str, &str)] = &[("Bank", "500", "Cr"), ("Ledger A", "500", "Dr")];
const BANK_TO_B: &[(&str, &str, &str)] = &[("Bank", "500.00", "Cr"), ("Ledger B", "500", "Dr")];

fn sent(batch: &ImportLedgerLine) -> Vec<u8> {
    record(&StatusRecord::dispatch_native(
        batch,
        "c".repeat(64),
        Uuid::new_v4(),
    ))
}

fn found(batch: &ImportLedgerLine) -> Vec<u8> {
    record(
        &serde_json::from_value::<StatusRecord>(json!({
            "record_type":"verification_status","batch_id":batch.batch_id,
            "batch_sha256":batch.sha256,"status":"posted_verified"
        }))
        .unwrap(),
    )
}

fn blocker(journal: Vec<Vec<u8>>, candidate: &ImportLedgerLine) -> Option<String> {
    rows_already_posted(Cursor::new(journal.concat()), candidate).unwrap()
}

fn already_posted(journal: Vec<Vec<u8>>, candidate: &ImportLedgerLine) -> bool {
    blocker(journal, candidate).is_some()
}

/// #876: a journal written before compact status records holds the verified
/// status on the full batch record itself; that batch's row still counts.
#[test]
fn a_full_record_already_marked_posted_verified_blocks_its_row() {
    let mut old = plain("old", STATEMENT_ID, BANK_TO_A);
    old.status = "posted_verified".into();
    let rebuilt = plain("new", STATEMENT_ID, BANK_TO_B);
    assert!(already_posted(vec![record(&old)], &rebuilt));
    old.status = "built".into();
    assert!(!already_posted(vec![record(&old)], &rebuilt));
}

/// #876: the row a statement build derived is one row wherever it is posted, so
/// a rebuild that only remaps its ledger, dispatched or found posted before,
/// is refused; a batch only built, another company, and the batch itself are not.
#[test]
fn a_statement_row_another_batch_sent_or_found_posted_is_already_posted() {
    let old = plain("old", STATEMENT_ID, BANK_TO_A);
    let remapped = plain("new", STATEMENT_ID, BANK_TO_B);
    assert!(already_posted(
        vec![record(&old), sent(&old), record(&remapped)],
        &remapped
    ));
    assert!(already_posted(
        vec![record(&old), found(&old), record(&remapped)],
        &remapped
    ));
    // The refusal names the earlier batch to verify, never the candidate.
    assert_eq!(
        blocker(vec![record(&old), sent(&old), record(&remapped)], &remapped),
        Some("old".to_string())
    );
    // Only built, or only a readback that did not match: nothing was posted.
    assert!(!already_posted(
        vec![record(&old), record(&remapped)],
        &remapped
    ));
    let incomplete = serde_json::from_value::<StatusRecord>(json!({
        "record_type":"verification_status","batch_id":"old",
        "batch_sha256":old.sha256,"status":"verification_incomplete"
    }))
    .unwrap();
    assert!(!already_posted(
        vec![record(&old), record(&incomplete), record(&remapped)],
        &remapped
    ));
    // The candidate's own attempt is not a twin of itself.
    assert!(!already_posted(
        vec![record(&remapped), sent(&remapped)],
        &remapped
    ));
    assert!(!already_posted(vec![], &remapped));
}

#[test]
fn a_row_of_another_company_is_not_already_posted_and_a_guid_compares_without_case() {
    let other = row_batch("old", "OTHER-GUID", STATEMENT_ID, 'a', BANK_TO_A);
    let candidate = plain("new", STATEMENT_ID, BANK_TO_B);
    assert!(!already_posted(
        vec![record(&other), sent(&other)],
        &candidate
    ));
    let upper = row_batch("old", "SYNTHETIC-GUID", STATEMENT_ID, 'a', BANK_TO_A);
    assert!(already_posted(
        vec![record(&upper), sent(&upper)],
        &candidate
    ));
}

/// Batch order is irrelevant: the earlier-built batch may be the one sent later.
#[test]
fn the_order_the_batches_were_built_in_does_not_matter() {
    let old = plain("old", STATEMENT_ID, BANK_TO_A);
    let candidate = plain("new", STATEMENT_ID, BANK_TO_B);
    assert!(already_posted(
        vec![record(&candidate), record(&old), sent(&old)],
        &candidate
    ));
    assert!(already_posted(
        vec![record(&old), sent(&old), record(&candidate)],
        &candidate
    ));
}

/// Only the latest record of a batch counts: a batch re-recorded without the
/// row and then sent posted no such row; and of two matching batches it is
/// enough that one was sent.
#[test]
fn only_a_batchs_latest_record_holds_a_row_and_any_sent_match_counts() {
    let candidate = plain("new", STATEMENT_ID, BANK_TO_B);
    let old = plain("old", STATEMENT_ID, BANK_TO_A);
    let rebuilt = row_batch("old", "synthetic-guid", "other-row", 'b', BANK_TO_A);
    assert!(!already_posted(
        vec![record(&old), record(&rebuilt), sent(&rebuilt)],
        &candidate
    ));
    let (first, second) = (
        plain("first", STATEMENT_ID, BANK_TO_A),
        plain("second", STATEMENT_ID, BANK_TO_A),
    );
    assert!(already_posted(
        vec![record(&first), record(&second), sent(&second)],
        &candidate
    ));
}

/// A label an agent typed repeats across batches ("t1"), so it counts only
/// with the same date and amounts; an id no other batch holds never counts.
#[test]
fn a_typed_transaction_id_matches_only_the_same_shape() {
    let old = plain("old", "t1", BANK_TO_A);
    let journal = |old: &ImportLedgerLine| vec![record(old), sent(old)];
    // Same id and shape, other ledger and the entries listed the other way.
    let same = plain(
        "new",
        "t1",
        &[("Ledger B", "500.0", "Dr"), ("Bank", "500", "Cr")],
    );
    assert!(already_posted(journal(&old), &same));
    // Same id, other amount or date: another transaction.
    let amount = plain(
        "new",
        "t1",
        &[("Bank", "600", "Cr"), ("Ledger B", "600", "Dr")],
    );
    assert!(!already_posted(journal(&old), &amount));
    // The shape holds the amounts and their sides, not which ledger takes
    // which, so the same amount reversed is refused: the safe direction.
    let reversed = plain(
        "new",
        "t1",
        &[("Bank", "500", "Dr"), ("Ledger B", "500", "Cr")],
    );
    assert!(already_posted(journal(&old), &reversed));
    // An extra Dr entry changes the sides, so it is not the same row.
    let extra = plain(
        "new",
        "t1",
        &[
            ("Bank", "500", "Cr"),
            ("Ledger B", "300", "Dr"),
            ("Ledger C", "200", "Dr"),
        ],
    );
    assert!(!already_posted(journal(&old), &extra));
    let mut later = same.clone();
    later.vouchers[0].date = "20260902".into();
    assert!(!already_posted(journal(&old), &later));
    // Another id with the same shape is another transaction.
    let renamed = plain("new", "t2", BANK_TO_B);
    assert!(!already_posted(journal(&old), &renamed));
}

/// A statement-derived id is refused on the id alone: a row with a TDS line or
/// split, whose shape changed, is still the row.
#[test]
fn a_statement_row_is_the_same_row_although_its_shape_changed() {
    let old = plain("old", STATEMENT_ID, BANK_TO_A);
    let split = plain(
        "new",
        STATEMENT_ID,
        &[
            ("Bank", "500", "Cr"),
            ("Ledger A", "450", "Dr"),
            ("TDS", "50", "Dr"),
        ],
    );
    assert!(already_posted(vec![record(&old), sent(&old)], &split));
}

/// An amount that cannot be read does not open a way round the check.
#[test]
fn an_unreadable_amount_fails_closed() {
    let old = plain(
        "old",
        "t1",
        &[("Bank", "not-a-number", "Cr"), ("Ledger A", "1", "Dr")],
    );
    let candidate = plain("new", "t1", BANK_TO_B);
    assert!(already_posted(vec![record(&old), sent(&old)], &candidate));
}

/// The same holds when it is the candidate's own amount that cannot be read.
#[test]
fn an_unreadable_candidate_amount_fails_closed() {
    let old = plain("old", "t1", BANK_TO_A);
    let candidate = plain(
        "new",
        "t1",
        &[("Bank", "not-a-number", "Cr"), ("Ledger B", "1", "Dr")],
    );
    assert!(already_posted(vec![record(&old), sent(&old)], &candidate));
}

/// One Sales invoice dated 1 Sep 2026 under `number`, over `entries`.
fn invoice_batch(
    id: &str,
    sha: char,
    (number, filed): (&str, &str),
    entries: &[(&str, &str, &str)],
) -> ImportLedgerLine {
    let mut line = row_batch(id, "synthetic-guid", id, sha, entries);
    line.vouchers[0] = serde_json::from_value(json!({
        "bridge_txn_id":id, "date":"20260901", "voucher_type":"Sales", "voucher_number":number,
        "invoice":{"voucher_type_name":filed, "place_of_supply":"Rajasthan"},
        "entries":entries.iter().map(|(ledger, amount, side)|
            json!({"ledger":ledger,"amount":amount,"side":side})).collect::<Vec<_>>()
    }))
    .unwrap();
    line
}

const SALE: &[(&str, &str, &str)] = &[("Customer", "118", "Dr"), ("Sales", "118.00", "Cr")];

/// What the journal offers as the control for a build's number read: a
/// verified invoice of the company (its number and date), preferring one of the
/// build's financial year and then the newest; else whether one was ever sent.
#[test]
fn the_number_control_is_a_verified_invoice_of_the_company() {
    let control = |journal: Vec<Vec<u8>>| {
        invoice_number_control(
            Cursor::new(journal.concat()),
            "synthetic-guid",
            ("20260401", "20270331"),
        )
        .unwrap()
    };
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    assert_eq!(control(vec![]), NumberControl::NeverSent);
    // Only built.
    assert_eq!(control(vec![record(&first)]), NumberControl::NeverSent);
    // Sent, never verified.
    assert_eq!(
        control(vec![record(&first), sent(&first)]),
        NumberControl::NoneVerified
    );
    let known = NumberControl::Known {
        batch_id: "first".to_string(),
        number: "INV/1".to_string(),
        date: "20260901".to_string(),
    };
    assert_eq!(
        control(vec![record(&first), sent(&first), found(&first)]),
        known
    );
    // A later verification that no longer finds it demotes it.
    let demoted = record(
        &serde_json::from_value::<StatusRecord>(json!({
            "record_type":"verification_status","batch_id":"first",
            "batch_sha256":first.sha256,"status":"verification_incomplete"
        }))
        .unwrap(),
    );
    assert_eq!(
        control(vec![record(&first), sent(&first), found(&first), demoted]),
        NumberControl::NoneVerified
    );
    // Another company's verified invoice is not this company's control.
    let mut elsewhere = invoice_batch("elsewhere", 'b', ("INV/9", "Sales Manual"), SALE);
    elsewhere.company_guid = "other-guid".to_string();
    assert_eq!(
        control(vec![
            record(&elsewhere),
            sent(&elsewhere),
            found(&elsewhere)
        ]),
        NumberControl::NeverSent
    );
    // In the build's year before a newer one outside it; the newest inside it.
    let dated = |id: &str, sha: char, number: &str, date: &str| {
        let mut line = invoice_batch(id, sha, (number, "Sales Manual"), SALE);
        line.vouchers[0].date = date.to_string();
        line
    };
    let last_year = dated("last-year", 'c', "INV/2", "20260310");
    let next_year = dated("next-year", 'd', "INV/3", "20270410");
    let later = dated("later", 'e', "INV/4", "20261015");
    let journal = |batches: &[&ImportLedgerLine]| {
        batches
            .iter()
            .flat_map(|batch| [record(batch), sent(batch), found(batch)])
            .collect::<Vec<_>>()
    };
    assert_eq!(
        control(journal(&[&later, &first, &next_year, &last_year])),
        NumberControl::Known {
            batch_id: "later".to_string(),
            number: "INV/4".to_string(),
            date: "20261015".to_string()
        }
    );
    assert_eq!(
        control(journal(&[&next_year, &last_year])),
        NumberControl::Known {
            batch_id: "next-year".to_string(),
            number: "INV/3".to_string(),
            date: "20270410".to_string()
        }
    );
}

/// An invoice has an unsettled twin while another batch of the company holds an
/// invoice with its date, filed type and entries (whatever its number), was
/// sent, and no readback found it posted. A twin only built, one found posted,
/// other figures, another filed type, another company, a Journal with the same
/// entries and the batch itself are not; and a batch with no invoice asks
/// nothing of the journal.
#[test]
fn an_invoice_sent_and_not_found_posted_is_the_unsettled_twin_of_its_figures() {
    let twin = |journal: Vec<Vec<u8>>, candidate: &ImportLedgerLine| {
        unsettled_invoice_twin(Cursor::new(journal.concat()), candidate).unwrap()
    };
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    let second = invoice_batch(
        "second",
        'b',
        ("INV/2", "Sales Manual"),
        &[("Customer", "118.00", "Dr"), ("Sales", "118", "Cr")],
    );
    assert_eq!(
        twin(vec![record(&first), sent(&first), record(&second)], &second),
        Some("first".to_string())
    );
    // Only built; found posted; the batch itself.
    assert_eq!(twin(vec![record(&first), record(&second)], &second), None);
    assert_eq!(
        twin(
            vec![record(&first), sent(&first), found(&first), record(&second)],
            &second
        ),
        None
    );
    assert_eq!(twin(vec![record(&second), sent(&second)], &second), None);
    // Other figures, another filed type, another company, a Journal.
    let dearer = invoice_batch(
        "dearer",
        'c',
        ("INV/1", "Sales Manual"),
        &[("Customer", "119", "Dr"), ("Sales", "119", "Cr")],
    );
    let other_type = invoice_batch("other-type", 'd', ("INV/1", "Sales Export"), SALE);
    let mut elsewhere = invoice_batch("elsewhere", 'e', ("INV/1", "Sales Manual"), SALE);
    elsewhere.company_guid = "another-guid".into();
    let journal = row_batch("journal", "synthetic-guid", "journal", 'f', SALE);
    for other in [&dearer, &other_type, &elsewhere, &journal] {
        assert_eq!(
            twin(vec![record(other), sent(other), record(&second)], &second),
            None,
            "{}",
            other.batch_id
        );
    }
    // A batch with no invoice reads nothing: a journal that is not one is not met.
    assert_eq!(
        unsettled_invoice_twin(Cursor::new(b"not json\n".to_vec()), &journal),
        Ok(None)
    );
}

/// A batch is settled by its LATEST status, as `settlement` takes it: one a
/// readback found posted and that was sent again afterwards is unsettled
/// again, so the guard comes back on.
#[test]
fn a_batch_sent_again_after_it_was_found_posted_is_unsettled_again() {
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    let second = invoice_batch("second", 'b', ("INV/2", "Sales Manual"), SALE);
    let journal = [record(&first), found(&first), sent(&first), record(&second)].concat();
    assert_eq!(
        unsettled_invoice_twin(Cursor::new(journal), &second),
        Ok(Some("first".to_string()))
    );
}

/// The guard fails closed. A saved batch whose invoice cannot be read counts
/// as a twin once sent; an invoice of the batch itself that cannot be read is
/// an error, never "no figures, nothing to match".
#[test]
fn an_invoice_that_cannot_be_read_never_switches_the_guard_off() {
    let unreadable = invoice_batch(
        "unreadable",
        'a',
        ("INV/1", "Sales Manual"),
        &[
            ("Customer", "not-a-number", "Dr"),
            ("Sales", "118.00", "Cr"),
        ],
    );
    let second = invoice_batch("second", 'b', ("INV/2", "Sales Manual"), SALE);
    let journal = [record(&unreadable), sent(&unreadable), record(&second)].concat();
    assert_eq!(
        unsettled_invoice_twin(Cursor::new(journal), &second),
        Ok(Some("unreadable".to_string()))
    );
    assert_eq!(
        unsettled_invoice_twin(Cursor::new(Vec::new()), &unreadable),
        Err("import_invoice_figures_unreadable".to_string())
    );
}

fn incomplete(batch: &ImportLedgerLine) -> Vec<u8> {
    record(
        &serde_json::from_value::<StatusRecord>(json!({
            "record_type":"verification_status","batch_id":batch.batch_id,
            "batch_sha256":batch.sha256,"status":"verification_incomplete"
        }))
        .unwrap(),
    )
}

fn released(batch: &ImportLedgerLine, invoice_found: bool) -> Vec<u8> {
    record(&StatusRecord::stop_release(batch, invoice_found))
}

fn stops(journal: Vec<Vec<u8>>) -> Vec<String> {
    invoice_stops(Cursor::new(journal.concat()), "synthetic-guid").unwrap()
}

/// A sent invoice that is not verified posted stops its company, and nothing
/// else does: not one only built, not one a person declined (no intent), not a
/// verified one, not another company's, not a Journal.
#[test]
fn a_sent_invoice_that_is_not_verified_posted_stops_its_company() {
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    assert!(stops(vec![]).is_empty());
    // Only built (or declined in the dialog): no dispatch intent.
    assert!(stops(vec![record(&first)]).is_empty());
    // Sent and never read back, sent and read back unverified.
    assert_eq!(stops(vec![record(&first), sent(&first)]), ["first"]);
    assert_eq!(
        stops(vec![record(&first), sent(&first), incomplete(&first)]),
        ["first"]
    );
    // Verified: nothing stops, and a later unverified read stops it again.
    assert!(stops(vec![record(&first), sent(&first), found(&first)]).is_empty());
    assert_eq!(
        stops(vec![
            record(&first),
            sent(&first),
            found(&first),
            incomplete(&first)
        ]),
        ["first"]
    );
    // Another company's invoice, and a batch with no invoice.
    let mut elsewhere = invoice_batch("elsewhere", 'b', ("INV/9", "Sales Manual"), SALE);
    elsewhere.company_guid = "other-guid".to_string();
    assert!(stops(vec![record(&elsewhere), sent(&elsewhere)]).is_empty());
    let journal_batch = plain("journal", "t1", BANK_TO_A);
    assert!(stops(vec![record(&journal_batch), sent(&journal_batch)]).is_empty());
    // Case in the company GUID does not matter.
    assert_eq!(
        invoice_stops(
            Cursor::new([record(&first), sent(&first)].concat()),
            "SYNTHETIC-GUID"
        )
        .unwrap(),
        ["first"]
    );
}

/// The fold fails closed: an invoice whose number or date cannot be read still
/// stops the company, though it can never be a number control.
#[test]
fn an_unreadable_invoice_still_stops_its_company() {
    let mut nameless = invoice_batch("nameless", 'a', ("INV/1", "Sales Manual"), SALE);
    nameless.vouchers[0].voucher_number = None;
    nameless.vouchers[0].date = "not-a-date".to_string();
    assert_eq!(
        stops(vec![record(&nameless), sent(&nameless)]),
        ["nameless"]
    );
}

/// A person's release lifts the stop for that batch only; a later verified
/// read voids it, so a divergence found afterwards stops the company again,
/// while a re-read that stays unverified does not re-stop it.
#[test]
fn a_release_lifts_one_batch_and_a_later_verified_read_voids_it() {
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    let second = invoice_batch("second", 'b', ("INV/2", "Sales Manual"), SALE);
    let base = vec![
        record(&first),
        sent(&first),
        incomplete(&first),
        record(&second),
        sent(&second),
    ];
    let with = |extra: Vec<Vec<u8>>| [base.clone(), extra].concat();
    assert_eq!(stops(with(vec![])), ["first", "second"]);
    // Releasing one leaves the other.
    assert_eq!(stops(with(vec![released(&first, false)])), ["second"]);
    // A re-read that stays unverified keeps the release.
    assert_eq!(
        stops(with(vec![released(&first, false), incomplete(&first)])),
        ["second"]
    );
    // Verified after the release voids it: the next unverified read stops again.
    assert_eq!(
        stops(with(vec![
            released(&first, false),
            found(&first),
            incomplete(&first)
        ])),
        ["first", "second"]
    );
}

/// The journal admits a release only after the batch's dispatch intent, for a
/// batch it knows, bound to that batch's sha256, carrying nothing else.
#[test]
fn the_journal_admits_a_release_only_for_a_sent_batch_bound_to_its_sha() {
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    let read =
        |journal: Vec<Vec<u8>>| invoice_stops(Cursor::new(journal.concat()), "synthetic-guid");
    // Before the dispatch intent.
    assert_eq!(
        read(vec![record(&first), released(&first, false)]),
        Err("import_ledger_invalid".to_string())
    );
    // Another batch's sha.
    let other = invoice_batch("first", 'b', ("INV/1", "Sales Manual"), SALE);
    assert_eq!(
        read(vec![record(&first), sent(&first), released(&other, false)]),
        Err("import_ledger_invalid".to_string())
    );
    // A release with no recorded `release_invoice_found`, and the field on a
    // record that is not a release.
    let mut bare = serde_json::to_value(StatusRecord::stop_release(&first, true)).unwrap();
    bare.as_object_mut()
        .unwrap()
        .remove("release_invoice_found");
    assert_eq!(
        read(vec![record(&first), sent(&first), record(&bare)]),
        Err("import_ledger_invalid".to_string())
    );
    let mut stray = serde_json::to_value(StatusRecord::stop_release(&first, true)).unwrap();
    stray["record_type"] = json!("verification_status");
    stray["status"] = json!("verification_incomplete");
    assert_eq!(
        read(vec![record(&first), sent(&first), record(&stray)]),
        Err("import_ledger_invalid".to_string())
    );
    // A release sets no status: the batch is still not verified.
    assert_eq!(
        read(vec![record(&first), sent(&first), released(&first, true)]),
        Ok(vec![])
    );
}

/// After a release the number control follows what the read at the release
/// found: an invoice it found stays the control, one it did not find no longer
/// counts as sent (so the build is not refused for a control it cannot have).
#[test]
fn a_release_clears_the_number_control_refusal_by_what_it_found() {
    let control = |journal: Vec<Vec<u8>>| {
        invoice_number_control(
            Cursor::new(journal.concat()),
            "synthetic-guid",
            ("20260401", "20270331"),
        )
        .unwrap()
    };
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    let sent_only = vec![record(&first), sent(&first), incomplete(&first)];
    assert_eq!(control(sent_only.clone()), NumberControl::NoneVerified);
    let release =
        |invoice_found| [sent_only.clone(), vec![released(&first, invoice_found)]].concat();
    assert_eq!(control(release(false)), NumberControl::NeverSent);
    assert_eq!(
        control(release(true)),
        NumberControl::Known {
            batch_id: "first".to_string(),
            number: "INV/1".to_string(),
            date: "20260901".to_string()
        }
    );
    // A later verified read voids the release, and the invoice is verified.
    // An unverified read after that is none verified again.
    assert_eq!(
        control([release(false), vec![found(&first), incomplete(&first)]].concat()),
        NumberControl::NoneVerified
    );
}

/// How a sent, unverified invoice batch stands: stopping until a person
/// releases it; released as found, it stays the number control and can be
/// released again; released as not found, it is out of both; a later verified
/// read voids a release, and the last release is the one that stands.
#[test]
fn a_batch_released_as_found_stands_as_the_control_until_released_again() {
    let holds = |journal: Vec<Vec<u8>>| {
        invoice_holds(Cursor::new(journal.concat()), "synthetic-guid").unwrap()
    };
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    let sent_only = vec![record(&first), sent(&first), incomplete(&first)];
    let with = |more: Vec<Vec<u8>>| [sent_only.clone(), more].concat();
    let only = |hold| BTreeMap::from([("first".to_string(), hold)]);
    assert_eq!(holds(sent_only.clone()), only(InvoiceHold::Stopping));
    assert_eq!(
        holds(with(vec![released(&first, true)])),
        only(InvoiceHold::ReleasedAsFound)
    );
    assert!(holds(with(vec![released(&first, false)])).is_empty());
    // Released as found and then released as not found: out of both.
    assert!(holds(with(vec![released(&first, true), released(&first, false)])).is_empty());
    // Verified: neither.
    assert!(holds(with(vec![released(&first, true), found(&first)])).is_empty());
    // A divergence after a verified read is a new stop, and a release made
    // before the verified read does not survive it.
    assert_eq!(
        holds(with(vec![
            released(&first, true),
            found(&first),
            incomplete(&first)
        ])),
        only(InvoiceHold::Stopping)
    );
    // Another company's batch holds nothing here, and a batch never sent
    // holds nothing.
    let mut elsewhere = invoice_batch("elsewhere", 'b', ("INV/9", "Sales Manual"), SALE);
    elsewhere.company_guid = "other-guid".to_string();
    assert!(holds(vec![record(&elsewhere), sent(&elsewhere)]).is_empty());
    assert!(holds(vec![record(&first)]).is_empty());
    // The stops are the stopping ones only.
    let stops = |journal: Vec<Vec<u8>>| {
        invoice_stops(Cursor::new(journal.concat()), "synthetic-guid").unwrap()
    };
    assert_eq!(stops(sent_only.clone()), ["first"]);
    assert!(stops(with(vec![released(&first, true)])).is_empty());
}
