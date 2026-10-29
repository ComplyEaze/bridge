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
            unsettled_binding_refused: 0,
            unsettled_reviewed: 0,
            unsettled_restored: 0,
            settled_by_person: 0,
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
            unsettled_binding_refused: 0,
            unsettled_reviewed: 0,
            unsettled_restored: 0,
            settled_by_person: 0,
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
    later.vouchers[0].date = bridge_tally_core::TallyDate::parse("20260902").unwrap();
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
/// verified invoice of the company (its batch, number and date), preferring one of the
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
    assert_eq!(control(vec![]), NumberControl::NoneStanding);
    // Only built.
    assert_eq!(control(vec![record(&first)]), NumberControl::NoneStanding);
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
        NumberControl::NoneStanding
    );
    // In the build's year before a newer one outside it; the newest inside it.
    let dated = |id: &str, sha: char, number: &str, date: &str| {
        let mut line = invoice_batch(id, sha, (number, "Sales Manual"), SALE);
        line.vouchers[0].date = bridge_tally_core::TallyDate::parse(date).unwrap();
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

fn released(batch: &ImportLedgerLine, seen: ReleaseSeen) -> Vec<u8> {
    record(&StatusRecord::stop_release(batch, seen))
}

const EVERY_SEEN: [ReleaseSeen; 3] = [
    ReleaseSeen::Found,
    ReleaseSeen::NotEstablished,
    ReleaseSeen::Unreadable,
];

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

/// The fold fails closed: an invoice whose number cannot be read still stops
/// the company, though it can never be a number control. (Its date is a
/// `TallyDate`, so a saved batch's date is always readable.)
#[test]
fn an_unreadable_invoice_still_stops_its_company() {
    let mut nameless = invoice_batch("nameless", 'a', ("INV/1", "Sales Manual"), SALE);
    nameless.vouchers[0].voucher_number = None;
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
    assert_eq!(
        stops(with(vec![released(&first, ReleaseSeen::NotEstablished)])),
        ["second"]
    );
    // A re-read that stays unverified keeps the release.
    assert_eq!(
        stops(with(vec![
            released(&first, ReleaseSeen::NotEstablished),
            incomplete(&first)
        ])),
        ["second"]
    );
    // Verified after the release voids it: the next unverified read stops again.
    assert_eq!(
        stops(with(vec![
            released(&first, ReleaseSeen::NotEstablished),
            found(&first),
            incomplete(&first)
        ])),
        ["first", "second"]
    );
}

/// The journal admits a release only after the batch's dispatch intent, for a
/// batch it knows and that is not verified, bound to that batch's sha256,
/// carrying nothing else.
#[test]
fn the_journal_admits_a_release_only_for_a_sent_batch_bound_to_its_sha() {
    let first = invoice_batch("first", 'a', ("INV/1", "Sales Manual"), SALE);
    let read =
        |journal: Vec<Vec<u8>>| invoice_stops(Cursor::new(journal.concat()), "synthetic-guid");
    // Before the dispatch intent.
    assert_eq!(
        read(vec![
            record(&first),
            released(&first, ReleaseSeen::NotEstablished)
        ]),
        Err("import_ledger_invalid".to_string())
    );
    // Another batch's sha.
    let other = invoice_batch("first", 'b', ("INV/1", "Sales Manual"), SALE);
    assert_eq!(
        read(vec![
            record(&first),
            sent(&first),
            released(&other, ReleaseSeen::NotEstablished)
        ]),
        Err("import_ledger_invalid".to_string())
    );
    // A release with no recorded `release_seen`, one with a value this build
    // does not know, and the field on a record that is not a release.
    let release = serde_json::to_value(StatusRecord::stop_release(
        &first,
        ReleaseSeen::NotEstablished,
    ))
    .unwrap();
    let mut bare = release.clone();
    bare.as_object_mut().unwrap().remove("release_seen");
    let mut unknown = release.clone();
    unknown["release_seen"] = json!("possibly_found");
    let mut stray = release;
    stray["record_type"] = json!("verification_status");
    stray["status"] = json!("verification_incomplete");
    for refused in [bare, unknown, stray] {
        assert_eq!(
            read(vec![record(&first), sent(&first), record(&refused)]),
            Err("import_ledger_invalid".to_string()),
            "{refused}"
        );
    }
    // A release is admitted only while its batch is not verified, whether the
    // latest status is a status record's or the batch line's own.
    let mut verified_line = first.clone();
    verified_line.status = "posted_verified".to_string();
    for (journal, admitted) in [
        (vec![record(&first), sent(&first), found(&first)], false),
        (
            vec![record(&first), sent(&first), record(&verified_line)],
            false,
        ),
        (
            vec![
                record(&first),
                sent(&first),
                found(&first),
                incomplete(&first),
            ],
            true,
        ),
    ] {
        let outcome = read([journal, vec![released(&first, ReleaseSeen::Found)]].concat());
        if admitted {
            assert_eq!(outcome, Ok(Vec::<String>::new()));
        } else {
            assert_eq!(outcome, Err("import_ledger_invalid".to_string()));
        }
    }
    // A release sets no status: the batch is still not verified. Each value
    // is recorded by its own name.
    for seen in EVERY_SEEN {
        assert_eq!(
            read(vec![record(&first), sent(&first), released(&first, seen)]),
            Ok(vec![])
        );
    }
    assert_eq!(
        [
            json!(ReleaseSeen::Found),
            json!(ReleaseSeen::NotEstablished),
            json!(ReleaseSeen::Unreadable)
        ],
        [
            json!("found"),
            json!("not_established"),
            json!("unreadable")
        ]
    );
}

/// A released batch is never the number control and no longer counts as sent,
/// whatever its release read saw: that read cannot show the number read works
/// on this Tally. So a company whose only sent invoice was released stands
/// as if none was sent, an earlier verified invoice stays the control across a
/// release of a later one, and a later verified read voids the release.
#[test]
fn a_released_invoice_is_never_the_number_control() {
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
    let release = |seen| [sent_only.clone(), vec![released(&first, seen)]].concat();
    for seen in EVERY_SEEN {
        assert_eq!(
            control(release(seen)),
            NumberControl::NoneStanding,
            "{seen:?}"
        );
    }
    // A later verified read voids the release, and the invoice is verified.
    // An unverified read after that is none verified again.
    let known_first = NumberControl::Known {
        batch_id: "first".to_string(),
        number: "INV/1".to_string(),
        date: "20260901".to_string(),
    };
    assert_eq!(
        control([release(ReleaseSeen::Found), vec![found(&first)]].concat()),
        known_first
    );
    assert_eq!(
        control(
            [
                release(ReleaseSeen::Found),
                vec![found(&first), incomplete(&first)]
            ]
            .concat()
        ),
        NumberControl::NoneVerified
    );
    // An earlier verified invoice stays the control across the release of a
    // later, newer one.
    let second = invoice_batch("second", 'b', ("INV/2", "Sales Manual"), SALE);
    let both = vec![
        record(&first),
        sent(&first),
        found(&first),
        record(&second),
        sent(&second),
        incomplete(&second),
    ];
    for seen in EVERY_SEEN {
        assert_eq!(
            control([both.clone(), vec![released(&second, seen)]].concat()),
            known_first,
            "{seen:?}"
        );
    }
}

/// A verified control whose invoice is gone from the book (deleted or
/// re-dated in Tally, or the books restored): the verification that no longer
/// finds it ends its use as the control and stops the company, an earlier
/// verified invoice becomes the control, and one release lifts the stop.
#[test]
fn a_control_that_no_longer_verifies_stops_its_company_until_released() {
    let control = |journal: &[Vec<u8>]| {
        invoice_number_control(
            Cursor::new(journal.concat()),
            "synthetic-guid",
            ("20260401", "20270331"),
        )
        .unwrap()
    };
    let older = invoice_batch("older", 'a', ("INV/1", "Sales Manual"), SALE);
    let gone = invoice_batch("gone", 'b', ("INV/2", "Sales Manual"), SALE);
    let verified = vec![
        record(&older),
        sent(&older),
        found(&older),
        record(&gone),
        sent(&gone),
        found(&gone),
    ];
    assert_eq!(
        control(&verified),
        NumberControl::Known {
            batch_id: "gone".to_string(),
            number: "INV/2".to_string(),
            date: "20260901".to_string(),
        }
    );
    assert!(stops(verified.clone()).is_empty());
    let demoted = [verified, vec![incomplete(&gone)]].concat();
    let older_known = NumberControl::Known {
        batch_id: "older".to_string(),
        number: "INV/1".to_string(),
        date: "20260901".to_string(),
    };
    assert_eq!(control(&demoted), older_known);
    assert_eq!(stops(demoted.clone()), vec!["gone".to_string()]);
    for seen in EVERY_SEEN {
        let after_release = [demoted.clone(), vec![released(&gone, seen)]].concat();
        assert_eq!(control(&after_release), older_known, "{seen:?}");
        assert!(stops(after_release).is_empty(), "{seen:?}");
    }
}

/// The batch size the tool descriptions and the README tell a caller (#865) is
/// the code's own batch-post limit, so a change of the constant cannot leave
/// the text stale.
#[test]
fn the_batch_size_the_texts_state_is_the_batch_post_limit() {
    let phrase =
        format!("cut a batch only between days, at most {MAX_BATCH_POST_VOUCHERS} vouchers");
    let definitions = crate::agent::catalog::registered_tool_definitions(true, true);
    for tool in ["build_import_xml", "parse_bank_statement"] {
        let description = definitions
            .as_array()
            .expect("tool list")
            .iter()
            .find(|definition| definition["name"] == tool)
            .and_then(|definition| definition["description"].as_str())
            .unwrap_or_else(|| panic!("{tool} has no description"));
        assert!(
            description.contains(&phrase),
            "{tool}'s description does not say {phrase:?}"
        );
    }
    let clause = "With posting on, a saved batch of up to 50 posts after one approval, \
         so same-day twins post together in one batch or through a hand import of the built file.";
    for tool in ["build_import_xml", "parse_bank_statement"] {
        let description = definitions
            .as_array()
            .expect("tool list")
            .iter()
            .find(|definition| definition["name"] == tool)
            .and_then(|definition| definition["description"].as_str())
            .unwrap_or_else(|| panic!("{tool} has no description"));
        assert!(
            description.contains(clause),
            "{tool}'s description lost the same-day twins clause"
        );
    }
    let readme = include_str!("../../docs/agent/README.md");
    let flat = readme.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains(clause),
        "the README lost the same-day twins clause"
    );
    let sentence = format!(
        "cut a batch only between days, at most {MAX_BATCH_POST_VOUCHERS} vouchers (the batch-post limit)"
    );
    assert!(
        readme.contains(&sentence),
        "the README does not say {sentence:?}"
    );
}

// --- #1050: a rollback seen once covers the company's earlier native posts ---

const ROLLBACK_RESPONSE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-import-response.utf16le.xml"
);

/// The captured answer to a native post, as the journal records it.
fn captured_outcome() -> bridge_tally_protocol::TallyImportOutcome {
    let xml = bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        ROLLBACK_RESPONSE,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        ROLLBACK_RESPONSE.len(),
    )
    .unwrap()
    .text;
    bridge_tally_protocol::parse_import_outcome(&xml).unwrap()
}

const PLACE_ENDPOINT: &str = "http://127.0.0.1:9001";

/// A native batch of one company at one Tally address.
fn placed(id: &str, company_name: &str, endpoint: Option<&str>) -> ImportLedgerLine {
    let mut line = batch(id, id);
    line.company = Some(
        serde_json::from_value(json!({
            "name": company_name, "guid": "00000000-0000-4000-8000-000000000002",
            "company_number": "100054", "books_from": "20260401"
        }))
        .unwrap(),
    );
    line.endpoint_origin = endpoint.map(str::to_string);
    line
}

/// Its native dispatch intent, recording `pre` as the pre-POST mark.
fn intent(line: &ImportLedgerLine, pre: Option<u64>) -> Vec<u8> {
    let mut update = StatusRecord::dispatch_native(line, "c".repeat(64), Uuid::new_v4());
    update.pre_post_voucher_mark = pre;
    record(&update)
}

/// Its recorded answer, with the captured outcome.
fn answered(line: &ImportLedgerLine) -> Vec<u8> {
    record(&StatusRecord::response(
        line,
        DispatchResponse {
            request_sha256: "c".repeat(64),
            response_sha256: "d".repeat(64),
            bytes: ROLLBACK_RESPONSE.len(),
            outcome: Some(captured_outcome()),
        },
    ))
}

fn seen(line: &ImportLedgerLine, mark: u64, records_before_read: u64) -> Vec<u8> {
    record(&StatusRecord::rollback_seen(
        line,
        mark,
        records_before_read,
    ))
}

fn cover(journal: &[Vec<u8>], batch_id: &str) -> RollbackScan {
    rollback_cover(Cursor::new(journal.concat()), batch_id).unwrap()
}

fn invalid(journal: &[Vec<u8>]) -> bool {
    rollback_cover(Cursor::new(journal.concat()), "p1").is_err()
}

/// The 12 Oct shape: P1 and P2 posted natively, a backup restored below both,
/// P2's verification sees the mark below its floor and records it. P1, whose
/// own verification would read the mark above P1's floor once a voucher is
/// keyed again, is covered by that record.
#[test]
fn a_rollback_seen_on_one_batch_covers_an_earlier_batch_of_the_company() {
    let created = captured_outcome().counters().created;
    assert!(created > 0);
    let p1 = placed("p1", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let p2 = placed("p2", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let mut journal = vec![
        record(&p1),
        intent(&p1, Some(1)),
        answered(&p1),
        record(&p2),
        intent(&p2, Some(1 + created)),
        answered(&p2),
    ];
    let read = journal.len() as u64;
    journal.push(seen(&p2, 1, read));
    for id in ["p1", "p2"] {
        let scan = cover(&journal, id);
        assert_eq!(
            scan.covered_by,
            Some(RollbackCover {
                mark_read: 1,
                seen_by: "p2".into()
            }),
            "{id}"
        );
        assert_eq!(scan.records, journal.len() as u64);
    }
    // The answer that recorded it names P1 as the batch to verify again.
    assert_eq!(
        covered_by_records_of(Cursor::new(journal.concat()), "p2").unwrap(),
        vec!["p1".to_string()]
    );
    assert!(covered_by_records_of(Cursor::new(journal.concat()), "p1")
        .unwrap()
        .is_empty());
}

/// #1050: in the local data report's count, a native post a recorded restore
/// covers is not settled whatever its latest status reads, and counts in its
/// own bucket and in no other part of `unsettled`.
#[test]
fn a_restored_post_is_not_settled_whatever_its_status_reads() {
    let created = captured_outcome().counters().created;
    assert!(created > 0);
    let p1 = placed("p1", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let p2 = placed("p2", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let verified = |line: &ImportLedgerLine| {
        let mut read = line.clone();
        read.status = "posted_verified".into();
        record(&StatusRecord::from(&read))
    };
    let mut journal = vec![
        record(&p1),
        intent(&p1, Some(1)),
        answered(&p1),
        verified(&p1),
        record(&p2),
        intent(&p2, Some(1 + created)),
        answered(&p2),
    ];
    let before = settlement(Cursor::new(journal.concat())).unwrap();
    // P1 verified and settled; P2 answered, not yet verified.
    assert_eq!((before.unsettled, before.unsettled_restored), (1, 0));
    let read = journal.len() as u64;
    journal.push(seen(&p2, 1, read));
    let after = settlement(Cursor::new(journal.concat())).unwrap();
    assert_eq!(after.unsettled, 2, "{after:?}");
    assert_eq!(after.unsettled_restored, 2, "{after:?}");
    assert_eq!(
        after.unsettled_no_response + after.unsettled_binding_refused + after.unsettled_reviewed,
        0,
        "{after:?}"
    );
    assert_eq!(after.settled_by_person, 0);
    assert_eq!(after.sent_or_found, 2, "the rows stay held");
}

/// #1050: a cover reaches only native posts dispatched before its reading. A
/// hand-imported batch beside it, with no dispatch intent, and a post sent
/// after the record are counted as before, and the parts still sum to
/// `unsettled`.
#[test]
fn a_restore_leaves_a_hand_import_and_a_later_post_where_they_were() {
    let created = captured_outcome().counters().created;
    let p1 = placed("p1", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let hand = placed("hand", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let later = placed("later", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let mut hand_found = hand.clone();
    hand_found.status = "posted_verified".into();
    let mut journal = vec![
        record(&p1),
        intent(&p1, Some(1)),
        answered(&p1),
        record(&hand_found),
    ];
    let read = journal.len() as u64;
    journal.push(seen(&p1, 1, read));
    journal.push(record(&later));
    journal.push(intent(&later, Some(1)));
    let counted = settlement(Cursor::new(journal.concat())).unwrap();
    assert!(created > 0);
    // P1 restored; `later` sent after the record, no answer: no response;
    // the hand import was never dispatched and was found posted.
    assert_eq!(counted.unsettled_restored, 1, "{counted:?}");
    assert_eq!(counted.unsettled_no_response, 1, "{counted:?}");
    assert_eq!(counted.unsettled, 2, "{counted:?}");
    assert_eq!(counted.sent_or_found, 3, "{counted:?}");
    assert_eq!(counted.no_dispatch_never_verified, 0, "{counted:?}");
    assert_eq!(
        counted.unsettled,
        counted.unsettled_no_response
            + counted.unsettled_binding_refused
            + counted.unsettled_reviewed
            + counted.unsettled_restored,
        "{counted:?}"
    );
}

/// A record judges a batch only on what the journal held when its mark was
/// read: a post written after that point, or answered after it, is not
/// covered by a mark its own vouchers had not yet raised.
#[test]
fn a_record_covers_only_what_the_journal_held_before_its_mark_was_read() {
    let created = captured_outcome().counters().created;
    let seer = placed("seer", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let early = placed("early", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let late = placed("late", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let mut journal = vec![
        record(&seer),
        intent(&seer, Some(20)),
        answered(&seer),
        // Dispatched before the read at the mark, answered only after it.
        record(&early),
        intent(&early, Some(5)),
    ];
    let read = journal.len() as u64;
    journal.push(answered(&early));
    journal.push(record(&late));
    journal.push(intent(&late, Some(5)));
    journal.push(answered(&late));
    journal.push(seen(&seer, 5, read));
    assert!(cover(&journal, "seer").covered_by.is_some());
    // Within the prefix `early` had proved only its pre-POST mark, 5, which is
    // not above the mark read: its answer came after.
    assert_eq!(cover(&journal, "early").covered_by, None);
    // `late` was dispatched after the read.
    assert_eq!(cover(&journal, "late").covered_by, None);
    // Had the read come after `early`'s answer, its floor 5 + created is
    // above 5, and it is covered.
    assert!(created > 0);
    let last = journal.len() - 1;
    let mut read_after_answer = journal.clone();
    read_after_answer[last] = seen(&seer, 5, read + 1);
    assert_eq!(
        cover(&read_after_answer, "early")
            .covered_by
            .map(|cover| cover.mark_read),
        Some(5)
    );
}

/// An answer that reported `STATUS=0` failed whatever its counters say
/// (IMPLEMENTATION_GUIDE.md §3.2, I6), so its `CREATED` proves no voucher
/// landed: the post's floor stays its pre-POST mark (#2035).
#[test]
fn an_answer_reporting_status_zero_raises_no_floor() {
    let failed = bridge_tally_protocol::parse_import_outcome(
        "<ENVELOPE><HEADER><STATUS>0</STATUS></HEADER><BODY><DATA><IMPORTRESULT><CREATED>10</CREATED><ALTERED>0</ALTERED><DELETED>0</DELETED><LASTVCHID>11</LASTVCHID><IGNORED>0</IGNORED><ERRORS>0</ERRORS><CANCELLED>0</CANCELLED><EXCEPTIONS>0</EXCEPTIONS></IMPORTRESULT></DATA></BODY></ENVELOPE>",
    )
    .unwrap();
    assert_eq!(failed.counters().created, 10);
    assert_eq!(
        failed.application_status(),
        bridge_tally_protocol::TallyImportApplicationStatus::Failure
    );
    let refused = placed("refused", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let seer = placed("seer", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let mut journal = vec![
        record(&refused),
        intent(&refused, Some(1)),
        record(&StatusRecord::response(
            &refused,
            DispatchResponse {
                request_sha256: "c".repeat(64),
                response_sha256: "d".repeat(64),
                bytes: 1,
                outcome: Some(failed),
            },
        )),
        record(&seer),
        intent(&seer, Some(1)),
    ];
    let read = journal.len() as u64;
    // The mark still reads 1: Tally created nothing for the refused post, so
    // no record would be written for it.
    assert!(!rollback_record_extends(Cursor::new(journal.concat()), "seer", 1, read).unwrap());
    journal.push(seen(&seer, 1, read));
    assert_eq!(cover(&journal, "refused").covered_by, None);
    assert!(covered_by_records_of(Cursor::new(journal.concat()), "seer")
        .unwrap()
        .is_empty());
}

/// Only the same company, by all four fields, at the same Tally address.
#[test]
fn a_record_covers_no_batch_of_another_company_or_address() {
    let seer = placed("seer", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let renamed = placed("renamed", "BRIDGE ROLLBACK LAB COPY", Some(PLACE_ENDPOINT));
    let elsewhere = placed(
        "elsewhere",
        "BRIDGE ROLLBACK LAB",
        Some("http://127.0.0.1:9002"),
    );
    let mut other_number = placed("number", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    other_number.company.as_mut().unwrap().company_number = "100055".into();
    let unplaced = placed("unplaced", "BRIDGE ROLLBACK LAB", None);
    let mut journal = vec![record(&seer), intent(&seer, Some(9))];
    for line in [&renamed, &elsewhere, &other_number, &unplaced] {
        journal.push(record(line));
        journal.push(intent(line, Some(9)));
    }
    let read = journal.len() as u64;
    journal.push(seen(&seer, 3, read));
    assert!(cover(&journal, "seer").covered_by.is_some());
    for id in ["renamed", "elsewhere", "number", "unplaced"] {
        assert_eq!(cover(&journal, id).covered_by, None, "{id}");
    }
}

/// A post with no pre-POST mark (a hand import, or one the journal never
/// recorded a mark for) is never covered, and never carries a record.
#[test]
fn a_batch_without_a_pre_post_mark_is_never_covered_and_carries_no_record() {
    let seer = placed("seer", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let hand = placed("hand", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let journal = vec![
        record(&seer),
        intent(&seer, Some(9)),
        record(&hand),
        intent(&hand, None),
        seen(&seer, 3, 4),
    ];
    assert_eq!(cover(&journal, "hand").covered_by, None);
    let mut on_hand = journal.clone();
    on_hand.push(seen(&hand, 3, 4));
    assert!(rollback_cover(Cursor::new(on_hand.concat()), "hand").is_err());
}

/// What the journal admits of a rollback-seen record: after a native intent
/// with a pre-POST mark, a count no larger than its own position, and nothing
/// else; and no other record carries its fields.
#[test]
fn a_rollback_seen_record_is_admitted_only_in_its_one_shape() {
    let p1 = placed("p1", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let base = vec![record(&p1), intent(&p1, Some(4))];
    let mut fine = base.clone();
    fine.push(seen(&p1, 2, 2));
    assert!(!invalid(&fine));
    // Before the intent.
    assert!(invalid(&[record(&p1), seen(&p1, 2, 1)]));
    // A count beyond its own position.
    let mut beyond = base.clone();
    beyond.push(seen(&p1, 2, 3));
    assert!(invalid(&beyond));
    // Its fields on another record, or another field on it.
    let mut stray = StatusRecord::stop_release(&p1, ReleaseSeen::Found);
    stray.mark_read = Some(2);
    let mut with_stray = base.clone();
    with_stray.push(record(&stray));
    assert!(invalid(&with_stray));
    let mut extra = StatusRecord::rollback_seen(&p1, 2, 2);
    extra.pre_post_voucher_mark = Some(4);
    let mut with_extra = base.clone();
    with_extra.push(record(&extra));
    assert!(invalid(&with_extra));
    // It sets no status: the batch keeps the status it had.
    let snapshot = read_snapshot(Cursor::new(fine.concat()), Some("p1"))
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.batch.status, "dispatch_started");
}

fn extends(journal: &[Vec<u8>], seen_by: &str, mark: u64, read: u64) -> bool {
    rollback_record_extends(Cursor::new(journal.concat()), seen_by, mark, read).unwrap()
}

/// A record is written only when it would cover a post no record covers yet.
/// A post already covered by its own record still records a second restore
/// that reaches a post sent since (the second restore would otherwise be lost
/// and that post could read verified against a voucher keyed by hand); a
/// record that would cover nothing new is not written.
#[test]
fn a_covered_posts_check_still_records_a_second_restore_that_reaches_a_later_post() {
    let created = captured_outcome().counters().created;
    let a = placed("a", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let c = placed("c", "BRIDGE ROLLBACK LAB", Some(PLACE_ENDPOINT));
    let mut journal = vec![record(&a), intent(&a, Some(10)), answered(&a)];
    // A's check sees the first restore, below A's floor.
    let read = journal.len() as u64;
    assert!(extends(&journal, "a", 10, read));
    journal.push(seen(&a, 10, read));
    assert!(cover(&journal, "a").covered_by.is_some());
    // The same restore again: nothing new to cover, nothing written.
    assert!(!extends(&journal, "a", 10, journal.len() as u64));
    // C is posted after the record (its floor above 10), then a second restore
    // to 10 is read by A's check: it reaches C, so it is written.
    journal.push(record(&c));
    journal.push(intent(&c, Some(10 + created)));
    journal.push(answered(&c));
    assert_eq!(cover(&journal, "c").covered_by, None);
    let read = journal.len() as u64;
    assert!(extends(&journal, "a", 10, read));
    journal.push(seen(&a, 10, read));
    assert_eq!(
        cover(&journal, "c").covered_by.map(|cover| cover.seen_by),
        Some("a".to_string())
    );
    // A mark at or above every proved floor is no rollback.
    assert!(!extends(
        &journal,
        "a",
        10 + 2 * created,
        journal.len() as u64
    ));
}

fn refused_as(line: &ImportLedgerLine, code: &str) -> Vec<u8> {
    record(&StatusRecord::post_span_verdict(
        line,
        &PostSpanVerdict::Refused(code.into()),
    ))
}

fn reviewed(line: &ImportLedgerLine, state: ReviewedBinding) -> Vec<u8> {
    record(&StatusRecord::content_review(line, state))
}

fn verified(line: &ImportLedgerLine) -> Vec<u8> {
    record(
        &serde_json::from_value::<StatusRecord>(json!({
            "record_type":"verification_status","batch_id":line.batch_id,
            "batch_sha256":line.sha256,"status":"posted_verified"
        }))
        .unwrap(),
    )
}

/// The review that stands for `batch_id`, or the journal's refusal.
fn review_of(journal: &[Vec<u8>], batch_id: &str) -> Result<Option<ReviewedBinding>, String> {
    content_review(Cursor::new(journal.concat()), batch_id)
}

/// A content review (#1039) is admitted only after a native intent with a
/// pre-POST mark, on a content-only state the journal holds: a refusal by any
/// code but `span_identity_reused`, or no outcome and no verdict. Never while
/// the batch reads verified, never twice while one stands, and never with a
/// field of another record; no other record carries its field. It sets no
/// status.
#[test]
fn a_content_review_is_admitted_only_on_a_content_only_state() {
    use ReviewedBinding::{Refused, ResponseNotRecorded};
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let refused = vec![
        record(&p1),
        intent(&p1, Some(4)),
        answered(&p1),
        refused_as(&p1, "span_step_not_created"),
    ];
    let unanswered = vec![record(&p1), intent(&p1, Some(4))];
    let with = |base: &[Vec<u8>], more: &[Vec<u8>]| [base, more].concat();

    let admitted = with(&refused, &[reviewed(&p1, Refused)]);
    assert_eq!(review_of(&admitted, "p1"), Ok(Some(Refused)));
    let snapshot = read_snapshot(Cursor::new(admitted.concat()), Some("p1"))
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.batch.status, "response_received");
    let admitted = with(&unanswered, &[reviewed(&p1, ResponseNotRecorded)]);
    assert_eq!(review_of(&admitted, "p1"), Ok(Some(ResponseNotRecorded)));
    // An answer whose outcome did not parse is no outcome: the state holds.
    let unparsed = StatusRecord::response(
        &p1,
        DispatchResponse {
            request_sha256: "c".repeat(64),
            response_sha256: "d".repeat(64),
            bytes: 1,
            outcome: None,
        },
    );
    let admitted = with(
        &unanswered,
        &[record(&unparsed), reviewed(&p1, ResponseNotRecorded)],
    );
    assert_eq!(review_of(&admitted, "p1"), Ok(Some(ResponseNotRecorded)));

    let refused_invalid = |journal: Vec<Vec<u8>>, why: &str| {
        assert_eq!(
            review_of(&journal, "p1"),
            Err("import_ledger_invalid".to_string()),
            "{why}"
        );
    };
    refused_invalid(
        vec![
            record(&p1),
            intent(&p1, Some(4)),
            answered(&p1),
            refused_as(&p1, IDENTITY_REUSED),
            reviewed(&p1, Refused),
        ],
        "a refusal no review covers",
    );
    refused_invalid(
        with(&refused[..3], &[reviewed(&p1, Refused)]),
        "no refusal on file",
    );
    refused_invalid(
        with(&refused[..3], &[reviewed(&p1, ResponseNotRecorded)]),
        "an outcome on file",
    );
    refused_invalid(
        with(&refused, &[reviewed(&p1, ResponseNotRecorded)]),
        "the other state",
    );
    refused_invalid(
        vec![
            record(&p1),
            intent(&p1, None),
            reviewed(&p1, ResponseNotRecorded),
        ],
        "an intent with no pre-POST mark",
    );
    refused_invalid(
        vec![record(&p1), reviewed(&p1, ResponseNotRecorded)],
        "never sent",
    );
    refused_invalid(
        with(
            &unanswered,
            &[verified(&p1), reviewed(&p1, ResponseNotRecorded)],
        ),
        "while verified",
    );
    refused_invalid(
        with(&refused, &[reviewed(&p1, Refused), reviewed(&p1, Refused)]),
        "a second while the first stands",
    );
    let mut extra = StatusRecord::content_review(&p1, Refused);
    extra.pre_post_voucher_mark = Some(4);
    refused_invalid(
        with(&refused, &[record(&extra)]),
        "a field of another record",
    );
    let mut stray = StatusRecord::stop_release(&p1, ReleaseSeen::Found);
    stray.reviewed_binding = Some(Refused);
    refused_invalid(
        with(&refused, &[record(&stray)]),
        "its field on another record",
    );
    let mut misnamed = StatusRecord::content_review(&p1, Refused);
    misnamed.status = "posted_verified".into();
    refused_invalid(with(&refused, &[record(&misnamed)]), "another status");
}

/// The record is written as `content_review` with its state in snake case. A
/// build that does not know a record kind or a state refuses the whole
/// journal holding it, as an older build does this one, never skipping it.
#[test]
fn a_record_kind_or_state_the_build_does_not_know_refuses_the_whole_journal() {
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let review = serde_json::to_value(StatusRecord::content_review(
        &p1,
        ReviewedBinding::ResponseNotRecorded,
    ))
    .unwrap();
    assert_eq!(review["record_type"], "content_review");
    assert_eq!(review["status"], "content_reviewed");
    assert_eq!(review["reviewed_binding"], "response_not_recorded");
    for (field, unknown) in [
        ("record_type", "content_review_v2"),
        ("reviewed_binding", "reviewed"),
    ] {
        let mut changed = review.clone();
        changed[field] = json!(unknown);
        let journal = vec![record(&p1), intent(&p1, Some(4)), record(&changed)];
        assert_eq!(
            review_of(&journal, "p1"),
            Err("import_ledger_invalid".to_string()),
            "{field}"
        );
        assert!(
            settlement(Cursor::new(journal.concat())).is_err(),
            "{field}"
        );
    }
}

/// A review stands until the batch reads `posted_verified`, an answer with an
/// outcome is recorded, or a rollback covers the batch; each voids it for good.
/// After a verified read or an answer a new review may be recorded where the
/// state allows; after a cover none is, since the batch then reads rolled back
/// and the journal keeps refusing a second review of it.
#[test]
fn a_content_review_is_voided_by_a_verified_read_an_answer_or_a_rollback() {
    use ReviewedBinding::{Refused, ResponseNotRecorded};
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let mut journal = vec![
        record(&p1),
        intent(&p1, Some(4)),
        reviewed(&p1, ResponseNotRecorded),
    ];
    assert_eq!(review_of(&journal, "p1"), Ok(Some(ResponseNotRecorded)));
    journal.push(verified(&p1));
    assert_eq!(review_of(&journal, "p1"), Ok(None));
    journal.push(incomplete(&p1));
    assert_eq!(review_of(&journal, "p1"), Ok(None), "void for good");
    journal.push(reviewed(&p1, ResponseNotRecorded));
    assert_eq!(review_of(&journal, "p1"), Ok(Some(ResponseNotRecorded)));
    journal.push(answered(&p1));
    assert_eq!(review_of(&journal, "p1"), Ok(None), "an answer voids it");

    // A later answer voids a review of a lost answer, so the refusal its
    // binding then records can be reviewed in turn.
    let mut late = vec![
        record(&p1),
        intent(&p1, Some(4)),
        reviewed(&p1, ResponseNotRecorded),
        answered(&p1),
        refused_as(&p1, "span_step_not_created"),
        reviewed(&p1, Refused),
    ];
    assert_eq!(review_of(&late, "p1"), Ok(Some(Refused)));
    // A full record that reads verified voids it as a status record does.
    let mut found = p1.clone();
    found.status = "posted_verified".into();
    late.push(record(&found));
    assert_eq!(review_of(&late, "p1"), Ok(None));

    // A rollback that covers the batch voids its review; one that does not
    // cover it leaves the review standing.
    let refused = vec![
        record(&p1),
        intent(&p1, Some(4)),
        answered(&p1),
        refused_as(&p1, "span_step_not_created"),
        reviewed(&p1, Refused),
    ];
    let floor = 4 + captured_outcome().counters().created;
    let mut above = refused.clone();
    above.push(seen(&p1, floor, refused.len() as u64));
    assert_eq!(cover(&above, "p1").covered_by, None);
    assert_eq!(review_of(&above, "p1"), Ok(Some(Refused)));
    let mut below = refused.clone();
    below.push(seen(&p1, floor - 1, refused.len() as u64));
    assert!(cover(&below, "p1").covered_by.is_some());
    assert_eq!(review_of(&below, "p1"), Ok(None));
    // Reviews of other batches are their own.
    assert_eq!(review_of(&refused, "p2"), Ok(None));
}

/// The local-data report counts a reviewed batch in its own bucket and in no
/// other part of the unsettled batches; once voided, it is back where it was.
#[test]
fn a_reviewed_batch_is_counted_in_its_own_bucket_until_voided() {
    use ReviewedBinding::{Refused, ResponseNotRecorded};
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let p2 = placed("p2", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let created = captured_outcome().counters().created;
    let journal = vec![
        record(&p1),
        intent(&p1, Some(4)),
        answered(&p1),
        refused_as(&p1, "span_step_not_created"),
        record(&p2),
        intent(&p2, Some(4 + created)),
    ];
    let counts = |journal: &[Vec<u8>]| {
        let counted = settlement(Cursor::new(journal.concat())).unwrap();
        (
            counted.unsettled,
            counted.unsettled_binding_refused,
            counted.unsettled_no_response,
            counted.unsettled_reviewed,
        )
    };
    assert_eq!(counts(&journal), (2, 1, 1, 0));
    let mut both = journal.clone();
    both.push(reviewed(&p1, Refused));
    assert_eq!(counts(&both), (2, 0, 1, 1));
    both.push(reviewed(&p2, ResponseNotRecorded));
    assert_eq!(counts(&both), (2, 0, 0, 2));
    // A restore below both posts' floors voids both reviews, and both posts
    // then count as restored and in no other part (#1050).
    let read = both.len() as u64;
    both.push(seen(&p2, 1, read));
    assert_eq!(counts(&both), (2, 0, 0, 0));
    assert_eq!(
        settlement(Cursor::new(both.concat()))
            .unwrap()
            .unsettled_restored,
        2
    );
}

/// The writer's check: the review a record would stand as if appended, or the
/// journal's refusal of it, so the writer appends only what the reader admits.
#[test]
fn the_writer_appends_only_a_review_the_journal_admits_and_holds_standing() {
    use ReviewedBinding::Refused;
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let refused = vec![
        record(&p1),
        intent(&p1, Some(4)),
        answered(&p1),
        refused_as(&p1, "span_step_not_created"),
    ];
    let after = |journal: &[Vec<u8>]| {
        content_review_after(
            Cursor::new(journal.concat()),
            &StatusRecord::content_review(&p1, Refused),
        )
    };
    assert_eq!(after(&refused), Ok(Some(Refused)));
    let mut standing = refused.clone();
    standing.push(reviewed(&p1, Refused));
    assert!(after(&standing).is_err());
    let mut covered = refused.clone();
    covered.push(seen(&p1, 1, refused.len() as u64));
    assert_eq!(after(&covered), Ok(None));
}

/// The licensed 7.1 Silver answer to a rejected single voucher, committed byte
/// for byte: every counter reported, `CREATED` 0, `EXCEPTIONS` 1.
const REJECTED_ONE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/single-import-missing-ledger.utf16le.xml"
);

/// Its recorded answer: Tally created none of the batch's one voucher.
fn rejected(line: &ImportLedgerLine) -> Vec<u8> {
    let xml = bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        REJECTED_ONE,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        REJECTED_ONE.len(),
    )
    .unwrap()
    .text;
    record(&StatusRecord::response(
        line,
        DispatchResponse {
            request_sha256: "c".repeat(64),
            response_sha256: "d".repeat(64),
            bytes: REJECTED_ONE.len(),
            outcome: Some(bridge_tally_protocol::parse_import_outcome(&xml).unwrap()),
        },
    ))
}

fn settled(line: &ImportLedgerLine) -> Vec<u8> {
    record(&StatusRecord::person_settlement(line))
}

fn bound_as(line: &ImportLedgerLine) -> Vec<u8> {
    let bound = line
        .vouchers
        .iter()
        .map(
            |voucher| super::super::span_identity::PostedVoucherIdentity {
                bridge_txn_id: voucher.bridge_txn_id.clone(),
                guid: "aaaa-00000064".into(),
                master_id: 100,
            },
        )
        .collect();
    record(&StatusRecord::post_span_verdict(
        line,
        &PostSpanVerdict::Bound(bound),
    ))
}

/// Whether a settlement of `batch_id` stands, or the journal's refusal.
fn settlement_of(journal: &[Vec<u8>], batch_id: &str) -> Result<bool, String> {
    person_settlement(Cursor::new(journal.concat()), batch_id)
}

/// A person settlement (#1709) is admitted only after a native intent with a
/// pre-POST mark and an answer that reported none of the batch's vouchers
/// created; never while the batch reads verified or is bound, never twice
/// while one stands, and only with its own status. It sets no status.
#[test]
fn a_person_settlement_is_admitted_only_after_an_answer_that_created_none() {
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let not_created = vec![record(&p1), intent(&p1, Some(4)), rejected(&p1)];
    let with = |base: &[Vec<u8>], more: &[Vec<u8>]| [base, more].concat();
    let invalid = Err("import_ledger_invalid".to_string());

    let admitted = with(&not_created, &[settled(&p1)]);
    assert_eq!(settlement_of(&admitted, "p1"), Ok(true));
    let snapshot = read_snapshot(Cursor::new(admitted.concat()), Some("p1"))
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.batch.status, "response_received");
    // A refused binding leaves the state as it is.
    let refused = with(&not_created, &[refused_as(&p1, "span_step_not_created")]);
    assert_eq!(
        settlement_of(&with(&refused, &[settled(&p1)]), "p1"),
        Ok(true)
    );

    // No pre-POST mark, no answer, an answer that created something, or one
    // whose outcome did not parse.
    let unmarked = vec![record(&p1), intent(&p1, None), rejected(&p1), settled(&p1)];
    assert_eq!(settlement_of(&unmarked, "p1"), invalid);
    let unanswered = vec![record(&p1), intent(&p1, Some(4)), settled(&p1)];
    assert_eq!(settlement_of(&unanswered, "p1"), invalid);
    let created = vec![
        record(&p1),
        intent(&p1, Some(4)),
        answered(&p1),
        settled(&p1),
    ];
    assert_eq!(settlement_of(&created, "p1"), invalid);
    let unparsed = record(&StatusRecord::response(
        &p1,
        DispatchResponse {
            request_sha256: "c".repeat(64),
            response_sha256: "d".repeat(64),
            bytes: 1,
            outcome: None,
        },
    ));
    let unparsed = vec![record(&p1), intent(&p1, Some(4)), unparsed, settled(&p1)];
    assert_eq!(settlement_of(&unparsed, "p1"), invalid);
    // A later answer that created something replaces the rejection.
    let replaced = with(&not_created, &[answered(&p1), settled(&p1)]);
    assert_eq!(settlement_of(&replaced, "p1"), invalid);
    // One exception for a batch of two is not every voucher rejected.
    let mut p2 = placed("p2", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let mut second = p2.vouchers[0].clone();
    second.bridge_txn_id = "txn-2".into();
    p2.vouchers.push(second);
    p2.txn_ids.push("txn-2".into());
    let partial = vec![
        record(&p2),
        intent(&p2, Some(4)),
        rejected(&p2),
        settled(&p2),
    ];
    assert_eq!(settlement_of(&partial, "p2"), invalid);

    // Verified, bound, a second settlement, or another status.
    assert_eq!(
        settlement_of(&with(&not_created, &[verified(&p1), settled(&p1)]), "p1"),
        invalid
    );
    assert_eq!(
        settlement_of(&with(&not_created, &[bound_as(&p1), settled(&p1)]), "p1"),
        invalid
    );
    assert_eq!(
        settlement_of(&with(&admitted, &[settled(&p1)]), "p1"),
        invalid
    );
    let mut renamed = serde_json::to_value(StatusRecord::person_settlement(&p1)).unwrap();
    renamed["status"] = json!("content_reviewed");
    assert_eq!(
        settlement_of(&with(&not_created, &[record(&renamed)]), "p1"),
        invalid
    );
}

/// The record's shape, and a build that does not know it refuses the journal.
#[test]
fn a_person_settlement_kind_the_build_does_not_know_refuses_the_whole_journal() {
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let settlement_json = serde_json::to_value(StatusRecord::person_settlement(&p1)).unwrap();
    assert_eq!(settlement_json["record_type"], "person_settlement");
    assert_eq!(settlement_json["status"], "settled_by_person");
    assert_eq!(settlement_json["batch_sha256"], p1.sha256.as_str());
    let mut changed = settlement_json.clone();
    changed["record_type"] = json!("person_settlement_v2");
    let journal = vec![
        record(&p1),
        intent(&p1, Some(4)),
        rejected(&p1),
        record(&changed),
    ];
    assert_eq!(
        settlement_of(&journal, "p1"),
        Err("import_ledger_invalid".to_string())
    );
    assert!(settlement(Cursor::new(journal.concat())).is_err());
}

/// A settlement stands until the batch reads `posted_verified`, an answer with
/// an outcome is recorded, it is bound, or a rollback covers it; each voids it
/// for good. A new settlement may then be recorded where the state allows.
#[test]
fn a_person_settlement_is_voided_by_a_verified_read_an_answer_a_bind_or_a_rollback() {
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let p2 = placed("p2", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let standing = vec![
        record(&p1),
        intent(&p1, Some(4)),
        rejected(&p1),
        settled(&p1),
    ];
    let with = |more: &[Vec<u8>]| [standing.as_slice(), more].concat();
    assert_eq!(settlement_of(&with(&[verified(&p1)]), "p1"), Ok(false));
    let mut full = p1.clone();
    full.status = "posted_verified".into();
    assert_eq!(settlement_of(&with(&[record(&full)]), "p1"), Ok(false));
    assert_eq!(settlement_of(&with(&[bound_as(&p1)]), "p1"), Ok(false));
    // A later answer voids it, and a new rejection admits a new settlement.
    assert_eq!(settlement_of(&with(&[rejected(&p1)]), "p1"), Ok(false));
    assert_eq!(
        settlement_of(&with(&[rejected(&p1), settled(&p1)]), "p1"),
        Ok(true)
    );
    // A restore below the post's floor voids it; one of another batch that
    // does not cover it does not.
    let read = standing.len() as u64;
    assert_eq!(settlement_of(&with(&[seen(&p1, 1, read)]), "p1"), Ok(false));
    let other = with(&[record(&p2), intent(&p2, Some(9))]);
    let read = other.len() as u64;
    let mut uncovered = other.clone();
    uncovered.push(seen(&p2, 8, read));
    assert_eq!(settlement_of(&uncovered, "p1"), Ok(true));
}

/// A settled batch leaves `unsettled` and every part of it, beside a standing
/// content review too, and comes back once the settlement is voided.
#[test]
fn a_settled_batch_leaves_every_not_settled_count_until_voided() {
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let p2 = placed("p2", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let journal = vec![
        record(&p1),
        intent(&p1, Some(4)),
        rejected(&p1),
        refused_as(&p1, "span_step_not_created"),
        record(&p2),
        intent(&p2, Some(4)),
    ];
    let counts = |journal: &[Vec<u8>]| {
        let counted = settlement(Cursor::new(journal.concat())).unwrap();
        (
            counted.unsettled,
            counted.unsettled_binding_refused,
            counted.unsettled_no_response,
            counted.unsettled_reviewed,
            counted.settled_by_person,
            counted.sent_or_found,
        )
    };
    assert_eq!(counts(&journal), (2, 1, 1, 0, 0, 2));
    let mut settled_journal = journal.clone();
    settled_journal.push(settled(&p1));
    assert_eq!(counts(&settled_journal), (1, 0, 1, 0, 1, 2));
    let mut reviewed_too = journal.clone();
    reviewed_too.push(reviewed(&p1, ReviewedBinding::Refused));
    assert_eq!(counts(&reviewed_too), (2, 0, 1, 1, 0, 2));
    reviewed_too.push(settled(&p1));
    assert_eq!(counts(&reviewed_too), (1, 0, 1, 0, 1, 2));
    // A restore below the post's floor voids the settlement, and both posts
    // then count as restored and in no other part (#1050).
    let read = settled_journal.len() as u64;
    settled_journal.push(seen(&p2, 1, read));
    assert_eq!(counts(&settled_journal), (2, 0, 0, 0, 0, 2));
    assert_eq!(
        settlement(Cursor::new(settled_journal.concat()))
            .unwrap()
            .unsettled_restored,
        2
    );
}

/// The writer's check: whether a record would stand if appended, or the
/// journal's refusal of it, so the writer appends only what the reader admits.
#[test]
fn the_writer_appends_only_a_settlement_the_journal_admits_and_holds_standing() {
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let not_created = vec![record(&p1), intent(&p1, Some(4)), rejected(&p1)];
    let after = |journal: &[Vec<u8>]| {
        person_settlement_after(
            Cursor::new(journal.concat()),
            &StatusRecord::person_settlement(&p1),
        )
    };
    assert_eq!(after(&not_created), Ok(true));
    let mut standing = not_created.clone();
    standing.push(settled(&p1));
    assert!(after(&standing).is_err());
    let mut covered = not_created.clone();
    covered.push(seen(&p1, 1, not_created.len() as u64));
    assert_eq!(after(&covered), Ok(false));
}

/// A verified read voids a settlement for good, by a status record or a full
/// record; once the batch reads unverified again and is still on record as
/// rejected, a new settlement is admitted.
#[test]
fn a_settlement_voided_by_a_verified_read_can_be_made_again() {
    let p1 = placed("p1", "BRIDGE CONCUR LAB", Some(PLACE_ENDPOINT));
    let standing = vec![
        record(&p1),
        intent(&p1, Some(4)),
        rejected(&p1),
        settled(&p1),
    ];
    let unverified = record(
        &serde_json::from_value::<StatusRecord>(json!({
            "record_type":"verification_status","batch_id":p1.batch_id,
            "batch_sha256":p1.sha256,"status":"reconciliation_required"
        }))
        .unwrap(),
    );
    let by_status = [
        standing.clone(),
        vec![verified(&p1), unverified, settled(&p1)],
    ]
    .concat();
    assert_eq!(settlement_of(&by_status, "p1"), Ok(true));
    let mut full = p1.clone();
    full.status = "posted_verified".into();
    let mut open = p1.clone();
    open.status = "reconciliation_required".into();
    let by_record = [standing, vec![record(&full), record(&open), settled(&p1)]].concat();
    assert_eq!(settlement_of(&by_record, "p1"), Ok(true));
}
