//! Master journal tests: local files and synthetic identifiers only.
use super::*;
use crate::agent::agent_import::tests::{sentinel_batch_id, sentinel_uuid};
use bridge_tally_core::master_binding::{twin_fold_keys, MasterNameKeys};
use serde_json::json;
use std::io::Cursor;
use std::path::Path;
use uuid::Uuid;

/// The merge gate's one registered synthetic company GUID.
const COMPANY_GUID: &str = "00000000-0000-4000-8000-000000000002";

fn batch_id() -> String {
    format!("bridge-{}", Uuid::new_v4())
}

fn digest(seed: char) -> String {
    seed.to_string().repeat(64)
}

fn ledger(name: &str, kind: StoredLedgerKind) -> StoredLedgerCreate {
    StoredLedgerCreate {
        name: name.into(),
        kind,
        bill_wise: false,
        party_gstin: None,
        gst_duty_head: matches!(kind, StoredLedgerKind::GstDuty).then(|| "Integrated Tax".into()),
    }
}

fn batch_with(id: &str, sha: &str, items: Vec<StoredLedgerCreate>) -> MasterBatch {
    MasterBatch {
        batch_id: id.into(),
        batch_sha256: sha.into(),
        class: MasterClass::LedgerCreate,
        company: MasterCompany {
            guid: COMPANY_GUID.into(),
            name: "Synthetic Company".into(),
            number: "1".into(),
            books_from: "20260401".into(),
        },
        items,
    }
}

fn batch(id: &str) -> MasterBatch {
    batch_with(
        id,
        &digest('a'),
        vec![
            ledger("Synthetic Debtor", StoredLedgerKind::PartySundryDebtors),
            ledger("Synthetic Duty", StoredLedgerKind::GstDuty),
        ],
    )
}

fn intent_for(batch: &MasterBatch) -> MasterDispatchIntent {
    MasterDispatchIntent {
        batch_id: batch.batch_id.clone(),
        batch_sha256: batch.batch_sha256.clone(),
        native_request_sha256: digest('b'),
        items: batch
            .items
            .iter()
            .map(|item| IntentItem::of(&item.name))
            .collect(),
        pre_image_sha256: digest('c'),
        pre_image_count: 2,
        pre_post_master_marks: vec![MasterMark {
            company_guid: batch.company.guid.clone(),
            altmstid: 7,
        }],
    }
}

fn response_for(batch: &MasterBatch) -> MasterDispatchResponse {
    MasterDispatchResponse {
        batch_id: batch.batch_id.clone(),
        batch_sha256: batch.batch_sha256.clone(),
        response_sha256: digest('d'),
        counters: Some(MasterCounters {
            created: 2,
            altered: 0,
            deleted: 0,
            ignored: 0,
            errors: 0,
            cancelled: 0,
            exceptions: 0,
        }),
    }
}

fn verdict_for(batch: &MasterBatch, state: MasterVerdictState) -> MasterVerdict {
    MasterVerdict::for_test(
        batch.batch_id.clone(),
        batch.batch_sha256.clone(),
        state,
        batch
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| VerdictItem {
                name: item.name.clone(),
                // A ledger confirmed absent has no GUID to name.
                guid: (state != MasterVerdictState::ConfirmedAbsent)
                    .then(|| format!("0000000{index}-aaaa-bbbb-cccc-dddddddddddd")),
                reason: None,
            })
            .collect(),
    )
}

fn lock() -> ExclusiveAdmission {
    ExclusiveAdmission::standing_in(tempfile::tempfile().unwrap())
}

/// `append` for any record: an intent goes through `append_intent`, as a caller's does.
fn put(
    directory: &Path,
    lock: &ExclusiveAdmission,
    record: MasterRecord,
) -> Result<(), MasterLedgerError> {
    match record {
        MasterRecord::DispatchIntent(intent) => {
            append_intent(directory, lock, intent).map(|_recorded| ())
        }
        other => append(directory, lock, other),
    }
}

fn write(directory: &Path, id: &str, image: &PreImage) -> Result<WrittenPreImage, PreImageError> {
    write_pre_image(directory, &lock(), &batch(id), image)
}

fn line(record: &MasterRecord) -> String {
    format!("{}\n", serde_json::to_string(record).unwrap())
}

fn read(text: &str) -> Result<MasterJournal, MasterLedgerError> {
    MasterJournal::read(Cursor::new(text.as_bytes()))
}

fn invalid(reason: Invalid) -> MasterLedgerError {
    MasterLedgerError::Invalid(reason)
}

/// A batch, its intent and an answer that counted nothing created or altered:
/// what a terminal `confirmed_absent` may follow.
fn quiet_prefix(batch: &MasterBatch) -> String {
    let mut response = response_for(batch);
    response.counters = Some(MasterCounters {
        created: 0,
        altered: 0,
        deleted: 0,
        ignored: 0,
        errors: 0,
        cancelled: 0,
        exceptions: 0,
    });
    journal_text(&[
        MasterRecord::Batch(batch.clone()),
        MasterRecord::DispatchIntent(intent_for(batch)),
        MasterRecord::DispatchResponse(response),
    ])
}

fn journal_text(records: &[MasterRecord]) -> String {
    records.iter().map(line).collect()
}

fn full_records(batch: &MasterBatch) -> Vec<MasterRecord> {
    vec![
        MasterRecord::Batch(batch.clone()),
        MasterRecord::DispatchIntent(intent_for(batch)),
        MasterRecord::DispatchResponse(response_for(batch)),
        MasterRecord::Verdict(verdict_for(batch, MasterVerdictState::PostedVerified)),
    ]
}

#[test]
fn every_record_round_trips_and_the_journal_reports_its_state() {
    let id = batch_id();
    let batch = batch(&id);
    let records = full_records(&batch);
    for record in &records {
        let text = serde_json::to_string(record).unwrap();
        assert_eq!(&parse_record(&text).unwrap(), record);
    }
    let journal = read(&journal_text(&records)).unwrap();
    assert!(journal.dispatched(&id));
    assert!(!journal.dispatched("bridge-absent"));
    let state = journal.state(&id).unwrap();
    assert_eq!(state.batch, batch);
    assert_eq!(state.intent, Some(intent_for(&batch)));
    assert_eq!(state.response, Some(response_for(&batch)));
    assert_eq!(
        journal.verdict(&id).map(|verdict| verdict.state),
        Some(MasterVerdictState::PostedVerified)
    );
}

#[test]
fn records_are_tagged_on_record_type_in_snake_case() {
    let batch = batch(&batch_id());
    let tags = full_records(&batch)
        .iter()
        .map(|record| serde_json::to_value(record).unwrap()["record_type"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        tags,
        [
            "master_batch",
            "master_dispatch_intent",
            "master_dispatch_response",
            "master_verdict"
        ]
    );
    let value = serde_json::to_value(&batch.items[0]).unwrap();
    assert_eq!(value["kind"], "party_sundry_debtors");
}

#[test]
fn a_new_record_variant_must_be_named_here() {
    // No wildcard: a fifth variant stops this test compiling until it is listed.
    fn name(record: &MasterRecord) -> &'static str {
        match record {
            MasterRecord::Batch(_) => "batch",
            MasterRecord::DispatchIntent(_) => "intent",
            MasterRecord::DispatchResponse(_) => "response",
            MasterRecord::Verdict(_) => "verdict",
        }
    }
    let names = full_records(&batch(&batch_id()))
        .iter()
        .map(name)
        .collect::<Vec<_>>();
    assert_eq!(names, ["batch", "intent", "response", "verdict"]);
}

#[test]
fn an_unknown_field_is_refused_on_every_record_and_on_nested_values() {
    let batch = batch(&batch_id());
    for record in full_records(&batch) {
        let mut value = serde_json::to_value(&record).unwrap();
        value["extra"] = json!(1);
        assert_eq!(
            read(&format!("{value}\n")).unwrap_err(),
            invalid(Invalid::Parse)
        );
    }
    let mut value = serde_json::to_value(MasterRecord::Batch(batch)).unwrap();
    value["items"][0]["extra"] = json!(true);
    assert_eq!(
        read(&format!("{value}\n")).unwrap_err(),
        invalid(Invalid::Parse)
    );
}

#[test]
fn an_unknown_or_missing_record_type_is_refused() {
    let mut value = serde_json::to_value(MasterRecord::Batch(batch(&batch_id()))).unwrap();
    value["record_type"] = json!("dispatch_intent");
    assert_eq!(
        read(&format!("{value}\n")).unwrap_err(),
        invalid(Invalid::Parse)
    );
    value.as_object_mut().unwrap().remove("record_type");
    assert_eq!(
        read(&format!("{value}\n")).unwrap_err(),
        invalid(Invalid::Parse)
    );
}

#[test]
fn a_record_before_its_batch_is_refused() {
    let batch = batch(&batch_id());
    for record in [
        MasterRecord::DispatchIntent(intent_for(&batch)),
        MasterRecord::DispatchResponse(response_for(&batch)),
        MasterRecord::Verdict(verdict_for(&batch, MasterVerdictState::PostedVerified)),
    ] {
        assert_eq!(read(&line(&record)).unwrap_err(), invalid(Invalid::NoBatch));
    }
}

#[test]
fn a_record_with_another_batch_digest_is_refused() {
    let batch = batch(&batch_id());
    let batch_line = line(&MasterRecord::Batch(batch.clone()));
    let mut intent = intent_for(&batch);
    intent.batch_sha256 = digest('e');
    assert_eq!(
        read(&format!(
            "{batch_line}{}",
            line(&MasterRecord::DispatchIntent(intent))
        ))
        .unwrap_err(),
        invalid(Invalid::ShaMismatch)
    );
    let intent_line = line(&MasterRecord::DispatchIntent(intent_for(&batch)));
    let mut response = response_for(&batch);
    response.batch_sha256 = digest('e');
    assert_eq!(
        read(&format!(
            "{batch_line}{intent_line}{}",
            line(&MasterRecord::DispatchResponse(response))
        ))
        .unwrap_err(),
        invalid(Invalid::ShaMismatch)
    );
    let mut verdict = verdict_for(&batch, MasterVerdictState::PostedVerified);
    verdict.batch_sha256 = digest('e');
    assert_eq!(
        read(&format!(
            "{batch_line}{intent_line}{}",
            line(&MasterRecord::Verdict(verdict))
        ))
        .unwrap_err(),
        invalid(Invalid::ShaMismatch)
    );
}

#[test]
fn a_batch_id_reused_with_another_digest_or_content_is_refused() {
    let id = batch_id();
    let first = batch(&id);
    let mut other_sha = first.clone();
    other_sha.batch_sha256 = digest('f');
    assert_eq!(
        read(&journal_text(&[
            MasterRecord::Batch(first.clone()),
            MasterRecord::Batch(other_sha)
        ]))
        .unwrap_err(),
        invalid(Invalid::BatchIdReused)
    );
    let mut other_items = first.clone();
    other_items.items.pop();
    assert_eq!(
        read(&journal_text(&[
            MasterRecord::Batch(first.clone()),
            MasterRecord::Batch(other_items)
        ]))
        .unwrap_err(),
        invalid(Invalid::BatchIdReused)
    );
    // The identical batch again is refused too: the caller reads the state.
    assert_eq!(
        read(&journal_text(&[
            MasterRecord::Batch(first.clone()),
            MasterRecord::Batch(first)
        ]))
        .unwrap_err(),
        invalid(Invalid::BatchIdReused)
    );
}

#[test]
fn a_second_dispatch_intent_is_the_duplicate_dispatch_refusal() {
    let batch = batch(&batch_id());
    let text = journal_text(&[
        MasterRecord::Batch(batch.clone()),
        MasterRecord::DispatchIntent(intent_for(&batch)),
        MasterRecord::DispatchIntent(intent_for(&batch)),
    ]);
    let error = read(&text).unwrap_err();
    assert_eq!(error, MasterLedgerError::DuplicateDispatch);
    assert_eq!(error.code(), "master_ledger_duplicate_dispatch");
}

#[test]
fn a_response_or_verdict_without_an_intent_is_refused() {
    let batch = batch(&batch_id());
    let batch_record = MasterRecord::Batch(batch.clone());
    assert_eq!(
        read(&journal_text(&[
            batch_record.clone(),
            MasterRecord::DispatchResponse(response_for(&batch))
        ]))
        .unwrap_err(),
        invalid(Invalid::ResponseWithoutIntent)
    );
    assert_eq!(
        read(&journal_text(&[
            batch_record,
            MasterRecord::Verdict(verdict_for(&batch, MasterVerdictState::PostedVerified))
        ]))
        .unwrap_err(),
        invalid(Invalid::VerdictWithoutIntent)
    );
}

#[test]
fn an_intent_or_verdict_must_name_the_batchs_items_in_order() {
    let batch = batch(&batch_id());
    let batch_line = line(&MasterRecord::Batch(batch.clone()));
    let mut intent = intent_for(&batch);
    intent.items.reverse();
    assert_eq!(
        read(&format!(
            "{batch_line}{}",
            line(&MasterRecord::DispatchIntent(intent))
        ))
        .unwrap_err(),
        invalid(Invalid::ItemsMismatch)
    );
    let mut verdict = verdict_for(&batch, MasterVerdictState::ReconciliationRequired);
    verdict.items.pop();
    assert_eq!(
        read(&format!(
            "{batch_line}{}{}",
            line(&MasterRecord::DispatchIntent(intent_for(&batch))),
            line(&MasterRecord::Verdict(verdict))
        ))
        .unwrap_err(),
        invalid(Invalid::ItemsMismatch)
    );
}

#[test]
fn a_ledger_kind_and_its_fields_must_agree() {
    let id = batch_id();
    let mut gstin_on_suspense = ledger("Synthetic Suspense", StoredLedgerKind::Suspense);
    gstin_on_suspense.party_gstin = Some("SYNTHETIC-GSTIN".into());
    let mut gstin_on_duty = ledger("Synthetic Duty", StoredLedgerKind::GstDuty);
    gstin_on_duty.party_gstin = Some("SYNTHETIC-GSTIN".into());
    let mut duty_without_head = ledger("Synthetic Duty", StoredLedgerKind::GstDuty);
    duty_without_head.gst_duty_head = None;
    let mut head_on_party = ledger("Synthetic Debtor", StoredLedgerKind::PartySundryDebtors);
    head_on_party.gst_duty_head = Some("Integrated Tax".into());
    let mut head_on_suspense = ledger("Synthetic Suspense", StoredLedgerKind::Suspense);
    head_on_suspense.gst_duty_head = Some("Integrated Tax".into());
    for bad in [
        gstin_on_suspense,
        gstin_on_duty,
        duty_without_head,
        head_on_party,
        head_on_suspense,
    ] {
        let record = MasterRecord::Batch(batch_with(&id, &digest('a'), vec![bad]));
        assert_eq!(
            read(&line(&record)).unwrap_err(),
            invalid(Invalid::KindFieldMismatch)
        );
    }
    // The consistent forms are admitted.
    let mut debtor = ledger("Synthetic Debtor", StoredLedgerKind::PartySundryDebtors);
    debtor.party_gstin = Some("SYNTHETIC-GSTIN".into());
    let mut creditor = ledger("Synthetic Creditor", StoredLedgerKind::PartySundryCreditors);
    creditor.bill_wise = true;
    let good = batch_with(
        &id,
        &digest('a'),
        vec![
            debtor,
            creditor,
            ledger("Synthetic Suspense", StoredLedgerKind::Suspense),
            ledger("Synthetic Duty", StoredLedgerKind::GstDuty),
        ],
    );
    assert!(read(&line(&MasterRecord::Batch(good))).is_ok());
}

#[test]
fn empty_blank_and_repeated_names_and_blank_fields_are_refused() {
    let id = batch_id();
    let record = |items| MasterRecord::Batch(batch_with(&id, &digest('a'), items));
    assert_eq!(
        read(&line(&record(vec![]))).unwrap_err(),
        invalid(Invalid::EmptyItems)
    );
    for name in ["", "   ", "\t"] {
        assert_eq!(
            read(&line(&record(vec![ledger(
                name,
                StoredLedgerKind::Suspense
            )])))
            .unwrap_err(),
            invalid(Invalid::BlankField)
        );
    }
    assert_eq!(
        read(&line(&record(vec![
            ledger("Synthetic One", StoredLedgerKind::Suspense),
            ledger("synthetic one", StoredLedgerKind::Suspense)
        ])))
        .unwrap_err(),
        invalid(Invalid::DuplicateName)
    );
    let mut blank_company = batch(&id);
    blank_company.company.name = " ".into();
    assert_eq!(
        read(&line(&MasterRecord::Batch(blank_company))).unwrap_err(),
        invalid(Invalid::BlankField)
    );
    let mut blank_gstin = ledger("Synthetic Debtor", StoredLedgerKind::PartySundryDebtors);
    blank_gstin.party_gstin = Some(" ".into());
    assert_eq!(
        read(&line(&record(vec![blank_gstin]))).unwrap_err(),
        invalid(Invalid::BlankField)
    );
}

#[test]
fn ids_and_digests_are_checked_on_every_record() {
    let good = batch(&batch_id());
    let mut bad_id = good.clone();
    bad_id.batch_id = "bridge-not-a-uuid".into();
    assert_eq!(
        read(&line(&MasterRecord::Batch(bad_id))).unwrap_err(),
        invalid(Invalid::BadBatchId)
    );
    for bad in ["", "A".repeat(64).as_str(), &"a".repeat(63)] {
        let mut batch = good.clone();
        batch.batch_sha256 = bad.into();
        assert_eq!(
            read(&line(&MasterRecord::Batch(batch))).unwrap_err(),
            invalid(Invalid::BadDigest)
        );
    }
    let batch_line = line(&MasterRecord::Batch(good.clone()));
    let mut intent = intent_for(&good);
    intent.native_request_sha256 = "xyz".into();
    assert_eq!(
        read(&format!(
            "{batch_line}{}",
            line(&MasterRecord::DispatchIntent(intent))
        ))
        .unwrap_err(),
        invalid(Invalid::BadDigest)
    );
    let mut intent = intent_for(&good);
    intent.pre_image_sha256 = "xyz".into();
    assert_eq!(
        read(&format!(
            "{batch_line}{}",
            line(&MasterRecord::DispatchIntent(intent))
        ))
        .unwrap_err(),
        invalid(Invalid::BadDigest)
    );
    let intent_line = line(&MasterRecord::DispatchIntent(intent_for(&good)));
    let mut response = response_for(&good);
    response.response_sha256 = "xyz".into();
    assert_eq!(
        read(&format!(
            "{batch_line}{intent_line}{}",
            line(&MasterRecord::DispatchResponse(response))
        ))
        .unwrap_err(),
        invalid(Invalid::BadDigest)
    );
}

#[test]
fn an_intent_refuses_two_marks_for_one_company() {
    let batch = batch(&batch_id());
    let mut intent = intent_for(&batch);
    intent.pre_post_master_marks.push(MasterMark {
        company_guid: batch.company.guid.clone(),
        altmstid: 8,
    });
    assert_eq!(
        read(&journal_text(&[
            MasterRecord::Batch(batch),
            MasterRecord::DispatchIntent(intent)
        ]))
        .unwrap_err(),
        invalid(Invalid::DuplicateMark)
    );
}

#[test]
fn a_verified_verdict_must_name_every_guid_and_guids_must_be_canonical() {
    let batch = batch(&batch_id());
    let prefix = format!(
        "{}{}",
        line(&MasterRecord::Batch(batch.clone())),
        line(&MasterRecord::DispatchIntent(intent_for(&batch)))
    );
    let mut missing = verdict_for(&batch, MasterVerdictState::PostedVerified);
    missing.items[1].guid = None;
    assert_eq!(
        read(&format!(
            "{prefix}{}",
            line(&MasterRecord::Verdict(missing))
        ))
        .unwrap_err(),
        invalid(Invalid::VerifiedWithoutGuid)
    );
    // The same gap is a legitimate unverified verdict.
    let mut unverified = verdict_for(&batch, MasterVerdictState::ReconciliationRequired);
    unverified.items[1].guid = None;
    unverified.items[1].reason = Some("not found after the post".into());
    assert!(read(&format!(
        "{prefix}{}",
        line(&MasterRecord::Verdict(unverified))
    ))
    .is_ok());
    let mut upper = verdict_for(&batch, MasterVerdictState::PostedVerified);
    upper.items[0].guid = Some(sentinel_uuid(0xabcd).to_string().to_uppercase());
    assert_eq!(
        read(&format!("{prefix}{}", line(&MasterRecord::Verdict(upper)))).unwrap_err(),
        invalid(Invalid::BadGuid)
    );
    let mut repeated = verdict_for(&batch, MasterVerdictState::PostedVerified);
    repeated.items[1].guid = repeated.items[0].guid.clone();
    assert_eq!(
        read(&format!(
            "{prefix}{}",
            line(&MasterRecord::Verdict(repeated))
        ))
        .unwrap_err(),
        invalid(Invalid::BadGuid)
    );
}

#[test]
fn counters_may_be_absent_and_must_be_whole() {
    let batch = batch(&batch_id());
    let mut response = response_for(&batch);
    response.counters = None;
    let text = journal_text(&[
        MasterRecord::Batch(batch.clone()),
        MasterRecord::DispatchIntent(intent_for(&batch)),
        MasterRecord::DispatchResponse(response),
    ]);
    assert_eq!(
        read(&text)
            .unwrap()
            .state(&batch.batch_id)
            .unwrap()
            .response
            .as_ref()
            .map(|response| response.counters),
        Some(None)
    );
    let mut value =
        serde_json::to_value(MasterRecord::DispatchResponse(response_for(&batch))).unwrap();
    value["counters"].as_object_mut().unwrap().remove("errors");
    let partial = format!(
        "{}{}{value}\n",
        line(&MasterRecord::Batch(batch.clone())),
        line(&MasterRecord::DispatchIntent(intent_for(&batch)))
    );
    assert_eq!(read(&partial).unwrap_err(), invalid(Invalid::Parse));
}

#[test]
fn a_line_without_its_newline_or_not_json_is_refused() {
    let text = journal_text(&[MasterRecord::Batch(batch(&batch_id()))]);
    assert_eq!(
        read(text.trim_end()).unwrap_err(),
        invalid(Invalid::Unterminated)
    );
    assert_eq!(read("not json\n").unwrap_err(), invalid(Invalid::Parse));
    assert_eq!(
        read("{\"record_type\":\"master_batch\"}\n").unwrap_err(),
        invalid(Invalid::Parse)
    );
    assert_eq!(read("").unwrap().state("anything"), None);
    // Every invalid cause reaches the edge as the one code.
    assert_eq!(invalid(Invalid::Parse).code(), "master_ledger_invalid");
}

#[test]
fn a_voucher_journal_line_in_the_master_journal_is_refused() {
    let voucher_batch = json!({
        "batch_id": batch_id(), "company_guid": "synthetic-guid", "company": null,
        "txn_ids": ["txn"], "date_from": "20260901", "date_to": "20260901",
        "sha256": digest('a'), "built_at": "2026-09-07T00:00:00Z", "status": "built",
        "on_account_approved": [],
        "pre_import_mark": {"kind": "company_high_water", "value": 1, "master_value": 1},
        "vouchers": [{"bridge_txn_id": "txn", "date": "20260901", "voucher_type": "Journal",
            "narration": "n", "reference": null, "voucher_number": null, "entries": [
                {"ledger": "Cash", "amount": "1", "side": "Dr"},
                {"ledger": "Synthetic Ledger", "amount": "1", "side": "Cr"}]}]
    });
    assert_eq!(
        read(&format!("{voucher_batch}\n")).unwrap_err(),
        invalid(Invalid::Parse)
    );
    let status = json!({"record_type": "dispatch_intent", "batch_id": batch_id(),
        "batch_sha256": digest('a'), "status": "dispatch_started"});
    assert_eq!(
        read(&format!("{status}\n")).unwrap_err(),
        invalid(Invalid::Parse)
    );
}

#[test]
fn a_master_line_in_the_voucher_journal_is_refused_by_its_reader() {
    // The voucher reader would refuse a master line loudly, never skip it.
    let text = journal_text(&[MasterRecord::Batch(batch(&batch_id()))]);
    assert!(super::ledger::parse_snapshots(&text).is_err());
}

#[test]
fn appending_master_records_leaves_the_voucher_journal_byte_for_byte() {
    let directory = tempfile::tempdir().unwrap();
    let voucher_path = directory.path().join("agent-import-ledger.jsonl");
    let voucher_line = json!({
        "batch_id": batch_id(), "company_guid": "synthetic-guid", "company": null,
        "txn_ids": ["txn"], "date_from": "20260901", "date_to": "20260901",
        "sha256": digest('a'), "built_at": "2026-09-07T00:00:00Z", "status": "built",
        "on_account_approved": [],
        "pre_import_mark": {"kind": "company_high_water", "value": 1, "master_value": 1},
        "vouchers": [{"bridge_txn_id": "txn", "date": "20260901", "voucher_type": "Journal",
            "narration": "n", "reference": null, "voucher_number": null, "entries": [
                {"ledger": "Cash", "amount": "1", "side": "Dr"},
                {"ledger": "Synthetic Ledger", "amount": "1", "side": "Cr"}]}]
    })
    .to_string()
        + "\n";
    std::fs::write(&voucher_path, &voucher_line).unwrap();
    let batch = batch(&batch_id());
    let lock = lock();
    for record in full_records(&batch) {
        put(directory.path(), &lock, record).unwrap();
    }
    assert_eq!(
        std::fs::read_to_string(&voucher_path).unwrap(),
        voucher_line
    );
    // The voucher reader still admits its own journal, with no master record in it.
    let snapshots = super::ledger::parse_snapshots(&voucher_line).unwrap();
    assert_eq!(snapshots.len(), 1);
    assert!(!snapshots[0].dispatched);
    let journal = read_journal(directory.path()).unwrap();
    assert!(journal.dispatched(&batch.batch_id));
}

#[test]
fn a_missing_journal_is_empty_and_append_builds_it_in_order() {
    let directory = tempfile::tempdir().unwrap();
    assert!(!read_journal(directory.path()).unwrap().dispatched("x"));
    let batch = batch(&batch_id());
    let lock = lock();
    for record in full_records(&batch) {
        put(directory.path(), &lock, record).unwrap();
    }
    let on_disk = std::fs::read_to_string(directory.path().join(JOURNAL_FILE)).unwrap();
    assert_eq!(on_disk, journal_text(&full_records(&batch)));
    assert_eq!(on_disk.lines().count(), 4);
}

#[test]
fn append_never_writes_a_record_the_reader_would_refuse() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(JOURNAL_FILE);
    let batch = batch(&batch_id());
    let lock = lock();
    // Nothing is written for an intent whose batch is not there.
    assert_eq!(
        put(
            directory.path(),
            &lock,
            MasterRecord::DispatchIntent(intent_for(&batch))
        )
        .unwrap_err(),
        invalid(Invalid::NoBatch)
    );
    assert!(!path.exists());
    put(directory.path(), &lock, MasterRecord::Batch(batch.clone())).unwrap();
    put(
        directory.path(),
        &lock,
        MasterRecord::DispatchIntent(intent_for(&batch)),
    )
    .unwrap();
    let before = std::fs::read(&path).unwrap();
    let error = put(
        directory.path(),
        &lock,
        MasterRecord::DispatchIntent(intent_for(&batch)),
    )
    .unwrap_err();
    assert_eq!(error, MasterLedgerError::DuplicateDispatch);
    assert_eq!(error.code(), "master_ledger_duplicate_dispatch");
    assert_eq!(std::fs::read(&path).unwrap(), before);
}

#[test]
fn a_corrupt_journal_file_blocks_appends() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join(JOURNAL_FILE), "garbage\n").unwrap();
    assert_eq!(
        put(
            directory.path(),
            &lock(),
            MasterRecord::Batch(batch(&batch_id()))
        )
        .unwrap_err(),
        invalid(Invalid::Parse)
    );
}

fn pre_image() -> PreImage {
    PreImage::new([
        (sentinel_uuid(0xbb02).to_string().to_uppercase(), 12),
        (sentinel_uuid(0xaa01).to_string(), 11),
    ])
    .unwrap()
}

#[test]
fn the_pre_image_is_sorted_lower_case_and_round_trips_through_its_sidecar() {
    let directory = tempfile::tempdir().unwrap();
    let id = batch_id();
    let image = pre_image();
    let written = write(directory.path(), &id, &image).unwrap();
    let sha = written.sha256().to_string();
    assert_eq!(written.count(), 2);
    let path = directory.path().join(format!("master-pre-image-{id}.json"));
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(
        String::from_utf8(bytes.clone()).unwrap(),
        format!(
            "{{\"{}\":11,\"{}\":12}}",
            sentinel_uuid(0xaa01),
            sentinel_uuid(0xbb02)
        )
    );
    assert_eq!(sha, sha256_hex(&bytes));
    assert_eq!(
        read_pre_image(directory.path(), &id, &sha, 2).unwrap(),
        image
    );
    // The same write repeated is accepted; different bytes under the name are not.
    assert_eq!(write(directory.path(), &id, &image).unwrap(), written);
    let other = PreImage::new([(sentinel_uuid(0xcc03).to_string(), 1)]).unwrap();
    assert_eq!(
        write(directory.path(), &id, &other).unwrap_err(),
        PreImageError::ExistsDifferent
    );
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    // No staged file is left behind.
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
}

#[test]
fn the_pre_image_constructor_refuses_empty_bad_and_conflicting_entries() {
    assert_eq!(PreImage::new(Vec::new()).unwrap_err(), PreImageError::Empty);
    for guid in ["", "not a guid", "../x", "zz-1"] {
        assert_eq!(
            PreImage::new([(guid.to_string(), 1)]).unwrap_err(),
            PreImageError::BadGuid
        );
    }
    assert_eq!(
        PreImage::new([("AB-1".to_string(), 1), ("ab-1".to_string(), 2)]).unwrap_err(),
        PreImageError::ConflictingEntry
    );
    assert_eq!(
        PreImage::new([("AB-1".to_string(), 1), ("ab-1".to_string(), 1)])
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn a_pre_image_with_the_wrong_digest_or_count_or_no_file_is_refused() {
    let directory = tempfile::tempdir().unwrap();
    let id = batch_id();
    assert_eq!(
        read_pre_image(directory.path(), &id, &digest('a'), 2).unwrap_err(),
        PreImageError::Missing
    );
    let sha = write(directory.path(), &id, &pre_image())
        .unwrap()
        .sha256()
        .to_string();
    assert_eq!(
        read_pre_image(directory.path(), &id, &digest('0'), 2).unwrap_err(),
        PreImageError::ShaMismatch
    );
    assert_eq!(
        read_pre_image(directory.path(), &id, &sha, 3).unwrap_err(),
        PreImageError::CountMismatch
    );
    assert_eq!(
        read_pre_image(directory.path(), &id, &sha, 1).unwrap_err(),
        PreImageError::CountMismatch
    );
    // A changed byte no longer carries the recorded digest.
    let path = directory.path().join(format!("master-pre-image-{id}.json"));
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[40] = b'9';
    std::fs::write(&path, &bytes).unwrap();
    assert_eq!(
        read_pre_image(directory.path(), &id, &sha, 2).unwrap_err(),
        PreImageError::ShaMismatch
    );
}

#[test]
fn a_sidecar_that_is_not_the_sorted_lower_case_map_is_refused_even_with_its_digest() {
    let directory = tempfile::tempdir().unwrap();
    let id = batch_id();
    let path = directory.path().join(format!("master-pre-image-{id}.json"));
    let upper_key = format!(
        "{{\"{}\":1}}",
        sentinel_uuid(0xaa01).to_string().to_uppercase()
    );
    for (bytes, expected) in [
        (&b"[1,2]"[..], PreImageError::Malformed),
        (upper_key.as_bytes(), PreImageError::Malformed),
        (&b"{}"[..], PreImageError::Empty),
        (&b"{\"../x\":1}"[..], PreImageError::BadGuid),
    ] {
        std::fs::write(&path, bytes).unwrap();
        assert_eq!(
            read_pre_image(directory.path(), &id, &sha256_hex(bytes), 1).unwrap_err(),
            expected
        );
    }
}

#[test]
fn a_batch_id_that_could_leave_the_data_dir_is_refused_for_the_sidecar() {
    let directory = tempfile::tempdir().unwrap();
    let nested = directory.path().join("nested");
    std::fs::create_dir(&nested).unwrap();
    let traversal = format!("{}/../../x", sentinel_batch_id(0));
    for id in [
        "../escape",
        "bridge-../../escape",
        traversal.as_str(),
        "",
        "/etc/passwd",
        "bridge-not-a-uuid",
    ] {
        assert_eq!(
            write(&nested, id, &pre_image()).unwrap_err(),
            PreImageError::BadBatchId
        );
        assert_eq!(
            read_pre_image(&nested, id, &digest('a'), 2).unwrap_err(),
            PreImageError::BadBatchId
        );
    }
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
    assert_eq!(std::fs::read_dir(&nested).unwrap().count(), 0);
}

#[test]
fn every_error_has_a_stable_code() {
    assert_eq!(
        MasterLedgerError::Unavailable.code(),
        "master_ledger_unavailable"
    );
    assert_eq!(PreImageError::Missing.code(), "master_pre_image_missing");
    assert_eq!(
        PreImageError::ShaMismatch.code(),
        "master_pre_image_invalid"
    );
}

/// A batch of one ledger, under its own id.
fn single(name: &str) -> MasterBatch {
    batch_with(
        &batch_id(),
        &digest('a'),
        vec![ledger(name, StoredLedgerKind::Suspense)],
    )
}

/// An intent whose recorded keys are overwritten, as a hand-edited line would be.
fn keyed_intent(batch: &MasterBatch, canonical: &str, absence: &str) -> MasterDispatchIntent {
    let mut intent = intent_for(batch);
    for item in &mut intent.items {
        item.canonical_key = canonical.into();
        item.absence_key = absence.into();
    }
    intent
}

fn dispatched_batch(name: &str) -> (MasterBatch, Vec<MasterRecord>) {
    let batch = single(name);
    let records = vec![
        MasterRecord::Batch(batch.clone()),
        MasterRecord::DispatchIntent(intent_for(&batch)),
    ];
    (batch, records)
}

#[test]
fn an_open_dispatch_blocks_a_new_batch_of_the_same_ledgers() {
    let (first, mut records) = dispatched_batch("Synthetic One");
    // The same ledger under a new batch id, by case and either separator.
    for name in ["SYNTHETIC-ONE", "synthetic/one"] {
        let (_, second) = dispatched_batch(name);
        let text = journal_text(&[records.clone(), second].concat());
        assert_eq!(
            read(&text).unwrap_err(),
            MasterLedgerError::OpenDispatchOverlaps
        );
    }
    assert_eq!(
        MasterLedgerError::OpenDispatchOverlaps.code(),
        "master_ledger_open_dispatch_overlaps"
    );
    // Other ledgers are not blocked.
    let (_, other) = dispatched_batch("Synthetic Four");
    assert!(read(&journal_text(&[records.clone(), other.clone()].concat())).is_ok());
    // An outcome left open by a verdict that settles nothing still blocks.
    records.push(MasterRecord::Verdict(verdict_for(
        &first,
        MasterVerdictState::MastersAfterPostUnconfirmed,
    )));
    let (_, again) = dispatched_batch("synthetic one");
    assert_eq!(
        read(&journal_text(&[records.clone(), again.clone()].concat())).unwrap_err(),
        MasterLedgerError::OpenDispatchOverlaps
    );
    // A terminal verdict settles it, and the same ledgers may be sent again.
    // A verified create rests on Tally's answer, recorded first.
    records.push(MasterRecord::DispatchResponse(response_for(&first)));
    records.push(MasterRecord::Verdict(verdict_for(
        &first,
        MasterVerdictState::PostedVerified,
    )));
    assert!(read(&journal_text(&[records, again].concat())).is_ok());
}

#[test]
fn append_refuses_an_overlapping_intent_and_writes_nothing() {
    let directory = tempfile::tempdir().unwrap();
    let lock = lock();
    let (_, first) = dispatched_batch("Synthetic One");
    for record in first {
        put(directory.path(), &lock, record).unwrap();
    }
    let (_, second) = dispatched_batch("Synthetic One");
    put(directory.path(), &lock, second[0].clone()).unwrap();
    let before = std::fs::read(directory.path().join(JOURNAL_FILE)).unwrap();
    assert_eq!(
        put(directory.path(), &lock, second[1].clone()).unwrap_err(),
        MasterLedgerError::OpenDispatchOverlaps
    );
    assert_eq!(
        std::fs::read(directory.path().join(JOURNAL_FILE)).unwrap(),
        before
    );
}

#[test]
fn the_first_terminal_verdict_wins_and_a_later_one_is_refused() {
    let batch = batch(&batch_id());
    let prefix = quiet_prefix(&batch);
    let verdict = |state| line(&MasterRecord::Verdict(verdict_for(&batch, state)));
    // Verdicts that settle nothing may repeat and may be followed by a terminal one.
    let open = [
        MasterVerdictState::MastersAfterPostUnconfirmed,
        MasterVerdictState::ReconciliationRequired,
    ];
    let text = format!(
        "{prefix}{}{}{}",
        verdict(open[0]),
        verdict(open[1]),
        verdict(MasterVerdictState::PostedVerified)
    );
    let journal = read(&text).unwrap();
    assert!(journal.state(&batch.batch_id).unwrap().is_settled());
    assert_eq!(
        journal
            .verdict(&batch.batch_id)
            .map(|verdict| verdict.state),
        Some(MasterVerdictState::PostedVerified)
    );
    // After a terminal verdict nothing follows, of any state.
    for terminal in [
        MasterVerdictState::PostedVerified,
        MasterVerdictState::PostedIntoExistingMaster,
        MasterVerdictState::MasterWriteLocatedElsewhere,
        MasterVerdictState::ConfirmedAbsent,
    ] {
        for later in [terminal, open[0], MasterVerdictState::PostedVerified] {
            assert_eq!(
                read(&format!("{prefix}{}{}", verdict(terminal), verdict(later))).unwrap_err(),
                invalid(Invalid::VerdictAfterTerminal)
            );
        }
    }
    assert!(!MasterVerdictState::ReconciliationRequired.is_terminal());
    assert!(!MasterVerdictState::MastersAfterPostUnconfirmed.is_terminal());
    assert!(MasterVerdictState::ConfirmedAbsent.is_terminal());
}

/// A verified create needs Tally's counters, so `posted_verified` is admitted
/// only after the batch's answer is on record; the other terminal verdicts can
/// rest on a read-back alone (an answer that was lost).
#[test]
fn posted_verified_without_an_answer_on_record_is_refused() {
    let batch = batch(&batch_id());
    let without_answer = journal_text(&full_records(&batch)[..2]);
    let verdict = |state| line(&MasterRecord::Verdict(verdict_for(&batch, state)));
    assert_eq!(
        read(&format!(
            "{without_answer}{}",
            verdict(MasterVerdictState::PostedVerified)
        ))
        .unwrap_err(),
        invalid(Invalid::VerifiedWithoutResponse)
    );
    for state in [
        MasterVerdictState::PostedIntoExistingMaster,
        MasterVerdictState::MasterWriteLocatedElsewhere,
        MasterVerdictState::MastersAfterPostUnconfirmed,
        MasterVerdictState::ReconciliationRequired,
    ] {
        assert!(
            read(&format!("{without_answer}{}", verdict(state))).is_ok(),
            "{state:?}"
        );
    }
    assert!(read(&format!(
        "{without_answer}{}",
        line(&MasterRecord::Verdict(absent_verdict(&batch)))
    ))
    .is_ok());
    let with_answer = journal_text(&full_records(&batch)[..3]);
    assert!(read(&format!(
        "{with_answer}{}",
        verdict(MasterVerdictState::PostedVerified)
    ))
    .is_ok());
}

/// Absence is never concluded against Tally's own count of a write: a
/// `confirmed_absent` verdict after an answer counting a create or an
/// alteration is refused; after an answer counting none, or none on record,
/// it is admitted.
#[test]
fn confirmed_absent_against_counters_that_show_a_write_is_refused() {
    let batch = batch(&batch_id());
    let intent = journal_text(&full_records(&batch)[..2]);
    let absent = line(&MasterRecord::Verdict(absent_verdict(&batch)));
    assert!(read(&format!("{intent}{absent}")).is_ok());
    let answered = |created: u64, altered: u64| {
        let mut response = response_for(&batch);
        response.counters = Some(MasterCounters {
            created,
            altered,
            deleted: 0,
            ignored: 0,
            errors: 0,
            cancelled: 0,
            exceptions: 0,
        });
        line(&MasterRecord::DispatchResponse(response))
    };
    assert!(read(&format!("{intent}{}{absent}", answered(0, 0))).is_ok());
    for (created, altered) in [(1, 0), (0, 1)] {
        assert_eq!(
            read(&format!("{intent}{}{absent}", answered(created, altered))).unwrap_err(),
            invalid(Invalid::AbsentAgainstCounters),
            "{created} {altered}"
        );
    }
}

/// `confirmed_absent` as a journal line: no GUID, an optional reason.
fn absent_verdict(batch: &MasterBatch) -> MasterVerdict {
    let mut verdict = verdict_for(batch, MasterVerdictState::ConfirmedAbsent);
    for item in &mut verdict.items {
        item.reason = Some("not_in_book".into());
    }
    verdict
}

#[test]
fn confirmed_absent_is_a_snake_case_terminal_verdict_without_a_guid() {
    let batch = batch(&batch_id());
    let prefix = quiet_prefix(&batch);
    let encoded = serde_json::to_string(&MasterVerdictState::ConfirmedAbsent).unwrap();
    assert_eq!(encoded, "\"confirmed_absent\"");
    let text = format!(
        "{prefix}{}",
        line(&MasterRecord::Verdict(absent_verdict(&batch)))
    );
    let journal = read(&text).unwrap();
    let state = journal.state(&batch.batch_id).unwrap();
    assert!(state.is_settled());
    assert_eq!(
        state.verdict.as_ref().map(|verdict| verdict.state),
        Some(MasterVerdictState::ConfirmedAbsent)
    );
    // It counts as settled in the local-data report.
    assert_eq!(journal.summary().not_settled, 0);
    // A GUID on an absent ledger is a contradiction the journal refuses.
    let mut named = absent_verdict(&batch);
    named.items[0].guid = Some(crate::synthetic_guid(0xaabb).into());
    assert_eq!(
        read(&format!("{prefix}{}", line(&MasterRecord::Verdict(named)))).unwrap_err(),
        invalid(Invalid::AbsentWithGuid)
    );
    // Nothing follows it, not even a verified verdict.
    for later in [
        MasterVerdictState::PostedVerified,
        MasterVerdictState::ConfirmedAbsent,
        MasterVerdictState::ReconciliationRequired,
    ] {
        let text = format!(
            "{prefix}{}{}",
            line(&MasterRecord::Verdict(absent_verdict(&batch))),
            line(&MasterRecord::Verdict(verdict_for(&batch, later)))
        );
        assert_eq!(
            read(&text).unwrap_err(),
            invalid(Invalid::VerdictAfterTerminal)
        );
    }
}

/// `posted_unattributed` as `MasterVerdict::judged` writes it: every GUID
/// named, and the reason it is not verified.
fn unattributed_verdict(batch: &MasterBatch) -> MasterVerdict {
    let mut verdict = verdict_for(batch, MasterVerdictState::PostedUnattributed);
    for item in &mut verdict.items {
        item.reason = Some("counters_unavailable".into());
    }
    verdict
}

/// The state is for an answer that was lost, so it is admitted only when no
/// answer is on file, and refused after any answer: one whose counters could
/// not be read, and one with counters, even counters that count nothing.
#[test]
fn posted_unattributed_is_admitted_only_with_no_answer_on_file() {
    let batch = batch(&batch_id());
    let intent = journal_text(&full_records(&batch)[..2]);
    let unattributed = line(&MasterRecord::Verdict(unattributed_verdict(&batch)));
    assert!(read(&format!("{intent}{unattributed}")).is_ok());
    let answered = |counters: Option<MasterCounters>| {
        let mut response = response_for(&batch);
        response.counters = counters;
        line(&MasterRecord::DispatchResponse(response))
    };
    assert_eq!(
        read(&format!("{intent}{}{unattributed}", answered(None))).unwrap_err(),
        invalid(Invalid::UnattributedWithAnswer)
    );
    let count = |created: u64, altered: u64, errors: u64| MasterCounters {
        created,
        altered,
        deleted: 0,
        ignored: 0,
        errors,
        cancelled: 0,
        exceptions: 0,
    };
    for counters in [
        count(1, 0, 0),
        count(0, 0, 0),
        count(0, 1, 0),
        count(1, 0, 1),
    ] {
        assert_eq!(
            read(&format!(
                "{intent}{}{unattributed}",
                answered(Some(counters))
            ))
            .unwrap_err(),
            invalid(Invalid::UnattributedWithAnswer),
            "{counters:?}"
        );
    }
}

#[test]
fn posted_unattributed_is_a_snake_case_terminal_verdict_naming_every_guid() {
    let batch = batch(&batch_id());
    let prefix = journal_text(&full_records(&batch)[..2]);
    assert_eq!(
        serde_json::to_string(&MasterVerdictState::PostedUnattributed).unwrap(),
        "\"posted_unattributed\""
    );
    assert!(MasterVerdictState::PostedUnattributed.is_terminal());
    let text = format!(
        "{prefix}{}",
        line(&MasterRecord::Verdict(unattributed_verdict(&batch)))
    );
    let journal = read(&text).unwrap();
    let state = journal.state(&batch.batch_id).unwrap();
    assert!(state.is_settled());
    assert_eq!(journal.summary().not_settled, 0);
    let mut missing = unattributed_verdict(&batch);
    missing.items[1].guid = None;
    assert_eq!(
        read(&format!(
            "{prefix}{}",
            line(&MasterRecord::Verdict(missing))
        ))
        .unwrap_err(),
        invalid(Invalid::VerifiedWithoutGuid)
    );
    for later in [
        MasterVerdictState::PostedVerified,
        MasterVerdictState::PostedUnattributed,
        MasterVerdictState::ReconciliationRequired,
        MasterVerdictState::MastersAfterPostUnconfirmed,
    ] {
        let text = format!(
            "{prefix}{}{}",
            line(&MasterRecord::Verdict(unattributed_verdict(&batch))),
            line(&MasterRecord::Verdict(verdict_for(&batch, later)))
        );
        assert_eq!(
            read(&text).unwrap_err(),
            invalid(Invalid::VerdictAfterTerminal),
            "{later:?}"
        );
    }
}

#[test]
fn posted_unattributed_stops_blocking_a_new_batch_of_the_same_ledger() {
    let (first, mut records) = dispatched_batch("Synthetic One");
    let (_, again) = dispatched_batch("SYNTHETIC-ONE");
    assert_eq!(
        read(&journal_text(&[records.clone(), again.clone()].concat())).unwrap_err(),
        MasterLedgerError::OpenDispatchOverlaps
    );
    records.push(MasterRecord::Verdict(unattributed_verdict(&first)));
    assert!(read(&journal_text(&[records, again].concat())).is_ok());
}

/// A build that does not know a verdict state refuses the whole journal on a
/// line holding it, so every master write and reconcile stops. A build from
/// before `posted_unattributed` reads it as this one reads a state it lacks;
/// that is why the state ships in the same release as the journal itself.
#[test]
fn a_reader_without_a_verdict_state_refuses_the_whole_journal() {
    let batch = batch(&batch_id());
    let text = format!(
        "{}{}",
        journal_text(&full_records(&batch)[..2]),
        line(&MasterRecord::Verdict(unattributed_verdict(&batch)))
    );
    assert!(read(&text).is_ok());
    let needle = "\"state\":\"posted_unattributed\"";
    assert_eq!(text.matches(needle).count(), 1, "{text}");
    let unknown = text.replace(needle, "\"state\":\"posted_somewhere_later\"");
    assert_eq!(read(&unknown).unwrap_err(), invalid(Invalid::Parse));
}

#[test]
fn confirmed_absent_stops_blocking_a_new_batch_of_the_same_ledger() {
    let (first, mut records) = dispatched_batch("Synthetic One");
    let (_, again) = dispatched_batch("SYNTHETIC-ONE");
    assert_eq!(
        read(&journal_text(&[records.clone(), again.clone()].concat())).unwrap_err(),
        MasterLedgerError::OpenDispatchOverlaps
    );
    records.push(MasterRecord::Verdict(absent_verdict(&first)));
    assert!(read(&journal_text(&[records, again].concat())).is_ok());
}

/// The two loud terminal verdicts settle a batch as the quiet ones do, so neither
/// holds its name against a new intent. That is right, and it is the journal's
/// documented rule (see the module notes): the person is told to check Tally for
/// both, and what stops a second Create of a name the book now holds is the
/// absence check against the book, made in the send's own queue, which refuses a
/// name that exists as `master_name_taken`. Holding would not be safer: a name
/// held for good by a loud state could never be released by any later call,
/// because a terminal verdict is the batch's last. Whether a ledger Tally
/// reported as altered should keep its name held until a person has looked is a
/// question for the owner, and is not what this pins. Verdicts that leave the
/// outcome open (`reconciliation_required`, `masters_after_post_unconfirmed`)
/// still hold, so the test fails if a loud state were mistaken for an open one,
/// and also if an open one were mistaken for a settled one.
#[test]
fn a_batch_that_ended_in_a_loud_state_does_not_hold_its_name_but_an_open_one_does() {
    use MasterVerdictState::{
        MasterWriteLocatedElsewhere, MastersAfterPostUnconfirmed, PostedIntoExistingMaster,
        ReconciliationRequired,
    };
    for (state, holds) in [
        (PostedIntoExistingMaster, false),
        (MasterWriteLocatedElsewhere, false),
        (ReconciliationRequired, true),
        (MastersAfterPostUnconfirmed, true),
    ] {
        // As a journal read: the same name under a new batch id, by case and by a
        // separator, is admitted or refused with the open-dispatch overlap.
        let (first, mut records) = dispatched_batch("Synthetic One");
        records.push(MasterRecord::Verdict(verdict_for(&first, state)));
        for name in ["Synthetic One", "SYNTHETIC-ONE"] {
            let (second, again) = dispatched_batch(name);
            let text = journal_text(&[records.clone(), again].concat());
            match read(&text) {
                Ok(journal) => {
                    assert!(!holds, "{state:?} should hold {name}");
                    assert!(journal.state(&first.batch_id).unwrap().is_settled());
                    assert!(journal.open_dispatch_blockers(&second).is_empty());
                }
                Err(error) => {
                    assert!(holds, "{state:?} should not hold {name}");
                    assert_eq!(error, MasterLedgerError::OpenDispatchOverlaps);
                }
            }
        }
        // As the append the post makes under the lock: the new batch's intent is
        // written, or refused with the same error and nothing written.
        let directory = tempfile::tempdir().unwrap();
        let lock = lock();
        for record in &records {
            put(directory.path(), &lock, record.clone()).unwrap();
        }
        let (second, again) = dispatched_batch("Synthetic One");
        put(directory.path(), &lock, again[0].clone()).unwrap();
        let before = std::fs::read(directory.path().join(JOURNAL_FILE)).unwrap();
        let journal = read_journal(directory.path()).unwrap();
        assert_eq!(
            journal.open_dispatch_blockers(&second).is_empty(),
            !holds,
            "{state:?}"
        );
        let appended = put(directory.path(), &lock, again[1].clone());
        if holds {
            assert_eq!(
                appended.unwrap_err(),
                MasterLedgerError::OpenDispatchOverlaps
            );
            assert_eq!(
                std::fs::read(directory.path().join(JOURNAL_FILE)).unwrap(),
                before
            );
        } else {
            appended.unwrap();
            assert!(read_journal(directory.path())
                .unwrap()
                .dispatched(&second.batch_id));
        }
    }
}

#[test]
fn a_batch_has_one_response() {
    let batch = batch(&batch_id());
    let text = journal_text(&[
        MasterRecord::Batch(batch.clone()),
        MasterRecord::DispatchIntent(intent_for(&batch)),
        MasterRecord::DispatchResponse(response_for(&batch)),
        MasterRecord::DispatchResponse(response_for(&batch)),
    ]);
    assert_eq!(read(&text).unwrap_err(), invalid(Invalid::SecondResponse));
}

#[test]
fn an_intent_needs_marks_that_include_the_batchs_own_company() {
    let batch = batch(&batch_id());
    let batch_record = MasterRecord::Batch(batch.clone());
    let with = |marks: Vec<MasterMark>| {
        let mut intent = intent_for(&batch);
        intent.pre_post_master_marks = marks;
        read(&journal_text(&[
            batch_record.clone(),
            MasterRecord::DispatchIntent(intent),
        ]))
    };
    let mark = |guid: &str| MasterMark {
        company_guid: guid.into(),
        altmstid: 3,
    };
    assert_eq!(with(vec![]).unwrap_err(), invalid(Invalid::MarksMissing));
    assert_eq!(
        with(vec![mark(&sentinel_uuid(0x00ff).to_string())]).unwrap_err(),
        invalid(Invalid::MarksMissing)
    );
    assert_eq!(
        with(vec![mark(
            &batch.company.guid.to_uppercase().replace('0', "A")
        )])
        .unwrap_err(),
        invalid(Invalid::BadGuid)
    );
    assert_eq!(with(vec![mark("")]).unwrap_err(), invalid(Invalid::BadGuid));
    // Other loaded companies are recorded beside the batch's own.
    assert!(with(vec![
        mark(&sentinel_uuid(0x00ff).to_string()),
        mark(&batch.company.guid)
    ])
    .is_ok());
    // The company of the batch is a canonical GUID too.
    let mut upper = batch.clone();
    upper.company.guid = sentinel_uuid(0xabce).to_string().to_uppercase();
    assert_eq!(
        read(&line(&MasterRecord::Batch(upper))).unwrap_err(),
        invalid(Invalid::BadGuid)
    );
}

#[test]
fn an_intent_is_built_only_from_the_sidecar_written_for_its_batch() {
    let directory = tempfile::tempdir().unwrap();
    let batch = batch(&batch_id());
    let written = write_pre_image(directory.path(), &lock(), &batch, &pre_image()).unwrap();
    let intent = MasterDispatchIntent::new(
        &batch,
        digest('b'),
        written,
        intent_for(&batch).pre_post_master_marks,
    )
    .unwrap();
    assert_eq!(intent.pre_image_count(), 2);
    // What the intent records is what the sidecar reads back with.
    assert!(read_pre_image(
        directory.path(),
        &batch.batch_id,
        intent.pre_image_sha256(),
        intent.pre_image_count()
    )
    .is_ok());
    // A token written for another batch is refused.
    let other = single("Synthetic Other");
    let foreign = write_pre_image(directory.path(), &lock(), &other, &pre_image()).unwrap();
    assert_eq!(
        MasterDispatchIntent::new(
            &batch,
            digest('b'),
            foreign,
            intent_for(&batch).pre_post_master_marks,
        )
        .unwrap_err(),
        PreImageError::OtherBatch
    );
    // A reader still refuses a zero count, whoever wrote the line.
    let mut zero = intent_for(&batch);
    zero.pre_image_count = 0;
    assert_eq!(
        read(&journal_text(&[
            MasterRecord::Batch(batch),
            MasterRecord::DispatchIntent(zero)
        ]))
        .unwrap_err(),
        invalid(Invalid::ZeroPreImageCount)
    );
}

#[test]
fn a_sidecar_the_reader_would_refuse_for_size_is_never_written() {
    let directory = tempfile::tempdir().unwrap();
    let batch = batch(&batch_id());
    // 128-character GUIDs, about 135 bytes an entry, past the 32 MiB bound.
    let entries = 260_000_u64;
    let huge = PreImage::new((0..entries).map(|index| (format!("{index:0>128x}"), index))).unwrap();
    assert_eq!(
        write_pre_image(directory.path(), &lock(), &batch, &huge).unwrap_err(),
        PreImageError::TooLargeToWrite
    );
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    assert_eq!(
        PreImageError::TooLargeToWrite.code(),
        "master_pre_image_write_failed"
    );
}

#[test]
fn a_directory_that_cannot_be_synced_is_an_error() {
    let directory = tempfile::tempdir().unwrap();
    assert!(sync_directory(directory.path()).is_ok());
    #[cfg(unix)]
    assert!(sync_directory(&directory.path().join("absent")).is_err());
    assert_eq!(
        MasterLedgerError::DirectorySyncFailed.code(),
        "master_ledger_directory_sync_failed"
    );
}

#[test]
fn the_local_data_report_names_exactly_the_files_this_module_writes() {
    let id = batch_id();
    assert!(is_master_file("agent-master-ledger.jsonl"));
    assert!(is_master_file(&format!("master-pre-image-{id}.json")));
    // The stage a sidecar write leaves behind if it stops.
    assert!(is_master_file(&format!(
        "master-pre-image-{id}.{}.next",
        Uuid::new_v4()
    )));
    for name in [
        "master-pre-image-x.json",
        "master-pre-image-.json",
        &format!("master-pre-image-{id}.json.next"),
        &format!("{id}.json"),
        "agent-master-ledger.jsonl.bak",
        &format!("master-pre-image-{id}.not-a-uuid.next"),
        &format!("master-pre-image-x.{}.next", Uuid::new_v4()),
    ] {
        assert!(!is_master_file(name), "{name}");
    }
}

/// Appends with a recording sync, one per call, in a fresh data directory.
struct SyncProbe {
    calls: std::cell::RefCell<Vec<std::path::PathBuf>>,
}

impl SyncProbe {
    fn append(
        &self,
        directory: &Path,
        lock: &ExclusiveAdmission,
        record: MasterRecord,
        result: std::io::Result<()>,
    ) -> Result<(), MasterLedgerError> {
        append_syncing(directory, lock, record, |synced: &Path| {
            self.calls.borrow_mut().push(synced.to_path_buf());
            result
        })
    }

    fn count(&self) -> usize {
        self.calls.borrow().len()
    }
}

fn probe() -> SyncProbe {
    SyncProbe {
        calls: std::cell::RefCell::new(Vec::new()),
    }
}

fn injected() -> std::io::Result<()> {
    Err(std::io::Error::other("injected"))
}

#[test]
fn the_directory_is_synced_for_the_first_append_and_for_every_intent_only() {
    let lock = lock();
    let batch = batch(&batch_id());
    let directory = tempfile::tempdir().unwrap();
    let probe = probe();
    // The append that creates the journal syncs the data directory.
    probe
        .append(
            directory.path(),
            &lock,
            MasterRecord::Batch(batch.clone()),
            Ok(()),
        )
        .unwrap();
    assert_eq!(*probe.calls.borrow(), [directory.path().to_path_buf()]);
    // The intent syncs again, the journal being there.
    probe
        .append(
            directory.path(),
            &lock,
            MasterRecord::DispatchIntent(intent_for(&batch)),
            Ok(()),
        )
        .unwrap();
    assert_eq!(probe.count(), 2);
    // A response or a verdict on a journal that is already durable does not.
    probe
        .append(
            directory.path(),
            &lock,
            MasterRecord::DispatchResponse(response_for(&batch)),
            Ok(()),
        )
        .unwrap();
    probe
        .append(
            directory.path(),
            &lock,
            MasterRecord::Verdict(verdict_for(&batch, MasterVerdictState::PostedVerified)),
            Ok(()),
        )
        .unwrap();
    assert_eq!(probe.count(), 2);
}

#[test]
fn a_failed_first_sync_does_not_excuse_the_intent_that_follows() {
    let lock = lock();
    let batch = batch(&batch_id());
    let directory = tempfile::tempdir().unwrap();
    let probe = probe();
    // The first append creates the file and writes the record, then its sync fails.
    assert_eq!(
        probe
            .append(
                directory.path(),
                &lock,
                MasterRecord::Batch(batch.clone()),
                injected()
            )
            .unwrap_err(),
        MasterLedgerError::DirectorySyncFailed
    );
    assert!(directory.path().join(JOURNAL_FILE).exists());
    assert_eq!(probe.count(), 1);
    // The file now exists and is not empty; the intent still syncs, checked.
    assert_eq!(
        probe
            .append(
                directory.path(),
                &lock,
                MasterRecord::DispatchIntent(intent_for(&batch)),
                injected()
            )
            .unwrap_err(),
        MasterLedgerError::DirectorySyncFailed
    );
    assert_eq!(probe.count(), 2);
    probe
        .append(
            directory.path(),
            &lock,
            MasterRecord::DispatchResponse(response_for(&batch)),
            Ok(()),
        )
        .unwrap();
    // A response after a durable intent needs no sync of its own.
    assert_eq!(probe.count(), 2);
}

#[test]
fn an_empty_journal_file_is_synced_on_any_append() {
    let lock = lock();
    let directory = tempfile::tempdir().unwrap();
    // A file left empty by an earlier append that stopped before writing.
    std::fs::write(directory.path().join(JOURNAL_FILE), b"").unwrap();
    let probe = probe();
    assert_eq!(
        probe
            .append(
                directory.path(),
                &lock,
                MasterRecord::Batch(batch(&batch_id())),
                injected()
            )
            .unwrap_err(),
        MasterLedgerError::DirectorySyncFailed
    );
    assert_eq!(probe.count(), 1);
}

#[test]
fn an_intent_and_a_record_cannot_be_deserialised_outside_the_reader() {
    // Compiles only while neither type implements `Deserialize`: with one
    // impl, the item below is ambiguous. The module's reader parses a line
    // through a private wire type instead.
    trait AmbiguousIfDeserialize<A> {
        fn probe() {}
    }
    impl<T: ?Sized> AmbiguousIfDeserialize<()> for T {}
    impl<T: serde::de::DeserializeOwned> AmbiguousIfDeserialize<u8> for T {}
    <MasterDispatchIntent as AmbiguousIfDeserialize<_>>::probe();
    <MasterRecord as AmbiguousIfDeserialize<_>>::probe();
    // The reader itself still builds an intent from a written line.
    let batch = batch(&batch_id());
    let record = MasterRecord::DispatchIntent(intent_for(&batch));
    assert_eq!(
        parse_record(&serde_json::to_string(&record).unwrap()).unwrap(),
        record
    );
}

#[test]
fn an_intent_takes_its_items_and_keys_from_its_batch() {
    let batch = batch_with(
        &batch_id(),
        &digest('a'),
        vec![
            ledger("J\u{30c}ournal-A", StoredLedgerKind::Suspense),
            ledger("Cafe\u{301} Traders", StoredLedgerKind::Suspense),
        ],
    );
    let directory = tempfile::tempdir().unwrap();
    let written = write_pre_image(directory.path(), &lock(), &batch, &pre_image()).unwrap();
    let intent = MasterDispatchIntent::new(
        &batch,
        digest('b'),
        written,
        intent_for(&batch).pre_post_master_marks,
    )
    .unwrap();
    let names: Vec<&str> = intent.items().iter().map(IntentItem::name).collect();
    assert_eq!(names, batch.names());
    for item in intent.items() {
        assert_eq!(item.canonical_key, canonical_master_key(item.name()));
        assert_eq!(item.absence_key, absence_key(item.name()));
    }
}

#[test]
fn an_open_dispatch_overlaps_on_matching_names_or_on_an_equal_recorded_key() {
    let records = |a: &MasterBatch, a_keys: (&str, &str), b: &MasterBatch, b_keys: (&str, &str)| {
        journal_text(&[
            MasterRecord::Batch(a.clone()),
            MasterRecord::DispatchIntent(keyed_intent(a, a_keys.0, a_keys.1)),
            MasterRecord::Batch(b.clone()),
            MasterRecord::DispatchIntent(keyed_intent(b, b_keys.0, b_keys.1)),
        ])
    };
    let rent = single("Synthetic Rent");
    // Names that may be one master overlap whatever keys the lines record.
    let spaced = single("synthetic   RENT");
    assert_eq!(
        read(&records(
            &rent,
            ("key-a", "absent-a"),
            &spaced,
            ("key-b", "absent-b")
        ))
        .unwrap_err(),
        MasterLedgerError::OpenDispatchOverlaps
    );
    // A key recorded on disk can only add a refusal: either one equal refuses
    // names that no fold relates (a fold narrowed later cannot release them).
    let other = single("Synthetic Water");
    for keys in [("key-a", "absent-b"), ("key-b", "absent-a")] {
        assert_eq!(
            read(&records(&rent, ("key-a", "absent-a"), &other, keys)).unwrap_err(),
            MasterLedgerError::OpenDispatchOverlaps,
            "{keys:?}"
        );
    }
    assert!(read(&records(
        &rent,
        ("key-a", "absent-a"),
        &other,
        ("key-b", "absent-b")
    ))
    .is_ok());
}

#[test]
fn a_pair_agreeing_on_the_canonical_key_only_or_a_twin_fold_only_may_be_one_master() {
    let one = |a: &str, b: &str| MasterNameKeys::of(a).may_be_one_master(&MasterNameKeys::of(b));
    let shares_a_twin_key = |a: &str, b: &str| {
        let b_keys = twin_fold_keys(b);
        twin_fold_keys(a).iter().any(|key| b_keys.contains(key))
    };
    // Capital iota with diaeresis and acute has no precomposed form, the small
    // one does (U+0390): lowercasing first gives one canonical key, while NFC
    // first leaves the twin folds apart in every position.
    let capital = "\u{399}\u{308}\u{301}ta";
    let small = "\u{3b9}\u{308}\u{301}ta";
    assert_eq!(canonical_master_key(capital), canonical_master_key(small));
    assert!(!shares_a_twin_key(capital, small));
    // NFC against NFD: the twin folds agree, the canonical key does not.
    let (nfc, nfd) = ("Caf\u{e9} Traders", "Cafe\u{301} Traders");
    assert_ne!(canonical_master_key(nfc), canonical_master_key(nfd));
    assert!(shares_a_twin_key(nfc, nfd));
    for (a, b) in [(capital, small), (nfc, nfd)] {
        assert!(one(a, b), "{a:?} {b:?}");
        assert!(one(b, a), "{a:?} {b:?}");
        let (_, first) = dispatched_batch(a);
        let (_, second) = dispatched_batch(b);
        assert_eq!(
            read(&journal_text(&[first, second].concat())).unwrap_err(),
            MasterLedgerError::OpenDispatchOverlaps,
            "{a:?} {b:?}"
        );
    }
    assert!(!one("Synthetic Rent", "Synthetic Water"));
}

#[test]
fn only_acquire_makes_an_exclusive_admission_and_a_shared_lock_blocks_it() {
    // By type: nothing converts a `File` into one, so the lock a shared holder
    // has (`lock_import_admission_shared` returns a `File`) cannot reach a write.
    trait AmbiguousIfFromFile<A> {
        fn probe() {}
    }
    impl<T: ?Sized> AmbiguousIfFromFile<()> for T {}
    impl<T: From<std::fs::File>> AmbiguousIfFromFile<u8> for T {}
    <ExclusiveAdmission as AmbiguousIfFromFile<_>>::probe();
    // By behaviour: acquiring takes the lock exclusively.
    let directory = tempfile::tempdir().unwrap();
    let first = ExclusiveAdmission::acquire(directory.path()).unwrap();
    assert!(ExclusiveAdmission::acquire(directory.path()).is_err());
    drop(first);
    let shared =
        local_file::open_local_file(&directory.path().join("agent-import-admission.lock"), true)
            .unwrap();
    shared.try_lock_shared().unwrap();
    assert!(ExclusiveAdmission::acquire(directory.path()).is_err());
    drop(shared);
    assert!(ExclusiveAdmission::acquire(directory.path()).is_ok());
}

#[test]
fn an_intent_is_appended_only_through_append_intent_which_returns_the_token() {
    let directory = tempfile::tempdir().unwrap();
    let lock = lock();
    let batch = batch(&batch_id());
    put(directory.path(), &lock, MasterRecord::Batch(batch.clone())).unwrap();
    let before = std::fs::read(directory.path().join(JOURNAL_FILE)).unwrap();
    // `append` refuses an intent and writes nothing.
    let error = append(
        directory.path(),
        &lock,
        MasterRecord::DispatchIntent(intent_for(&batch)),
    )
    .unwrap_err();
    assert_eq!(error, MasterLedgerError::UseAppendIntent);
    assert_eq!(error.code(), "master_ledger_use_append_intent");
    assert_eq!(
        std::fs::read(directory.path().join(JOURNAL_FILE)).unwrap(),
        before
    );
    let recorded = append_intent(directory.path(), &lock, intent_for(&batch)).unwrap();
    assert_eq!(recorded.batch_id(), batch.batch_id);
    assert_eq!(recorded.native_request_sha256(), digest('b'));
    // A second intent, or one whose directory sync fails, returns no token.
    assert_eq!(
        append_intent(directory.path(), &lock, intent_for(&batch)).unwrap_err(),
        MasterLedgerError::DuplicateDispatch
    );
    let other = single("Synthetic Other");
    put(directory.path(), &lock, MasterRecord::Batch(other.clone())).unwrap();
    assert_eq!(
        append_intent_syncing(directory.path(), &lock, intent_for(&other), |_: &Path| {
            injected()
        })
        .unwrap_err(),
        MasterLedgerError::DirectorySyncFailed
    );
    // The intent reached the file, so the batch reads as dispatched.
    assert!(read_journal(directory.path())
        .unwrap()
        .dispatched(&other.batch_id));
}

#[test]
fn an_intent_is_read_through_accessors_only() {
    let batch = batch(&batch_id());
    let intent = intent_for(&batch);
    assert_eq!(intent.batch_id(), batch.batch_id);
    assert_eq!(intent.batch_sha256(), batch.batch_sha256);
    assert_eq!(intent.native_request_sha256(), digest('b'));
    assert_eq!(intent.items().len(), 2);
    assert_eq!(intent.pre_post_master_marks().len(), 1);
}

#[test]
fn names_that_fold_together_cannot_share_a_batch() {
    let id = batch_id();
    let record = |names: &[&str]| {
        MasterRecord::Batch(batch_with(
            &id,
            &digest('a'),
            names
                .iter()
                .map(|name| ledger(name, StoredLedgerKind::Suspense))
                .collect(),
        ))
    };
    for names in [
        &["Rent  A", "Rent A"][..],
        &["Rent-A", "Rent A"][..],
        &["Rent/A", "Rent A"][..],
        &["Alpha", "Beta", "RENT\tA", "Rent A"][..],
        &["J\u{30c}ournal", "j\u{30c}ournal"][..],
        &["\u{399}\u{308}\u{301}ta", "\u{3b9}\u{308}\u{301}ta"][..],
        &["Caf\u{e9}", "Cafe\u{301}"][..],
        &["Cafe\u{301}/A", "Caf\u{e9} a"][..],
    ] {
        assert_eq!(
            read(&line(&record(names))).unwrap_err(),
            invalid(Invalid::DuplicateName),
            "{names:?}"
        );
    }
    assert!(read(&line(&record(&["Rent A", "Rent B", "Alpha"]))).is_ok());
}

#[test]
fn a_journal_cut_inside_a_record_is_refused_to_read_and_to_append() {
    let directory = tempfile::tempdir().unwrap();
    let lock = lock();
    let batch = batch(&batch_id());
    put(directory.path(), &lock, MasterRecord::Batch(batch.clone())).unwrap();
    put(
        directory.path(),
        &lock,
        MasterRecord::DispatchIntent(intent_for(&batch)),
    )
    .unwrap();
    let path = directory.path().join(JOURNAL_FILE);
    let whole_bytes = std::fs::read(&path).unwrap();
    let whole = whole_bytes.len() as u64;
    // Every cut inside the last record, the delimiter included, is refused.
    for cut in [1, 2, 7, 40] {
        std::fs::OpenOptions::new()
            .write(true)
            .open(&path)
            .unwrap()
            .set_len(whole - cut)
            .unwrap();
        assert!(matches!(
            read_journal(directory.path()).unwrap_err(),
            MasterLedgerError::Invalid(Invalid::Unterminated | Invalid::Parse)
        ));
        let cut_bytes = std::fs::read(&path).unwrap();
        assert!(put(
            directory.path(),
            &lock,
            MasterRecord::DispatchResponse(response_for(&batch))
        )
        .is_err());
        assert_eq!(std::fs::read(&path).unwrap(), cut_bytes);
        std::fs::write(&path, &whole_bytes).unwrap();
    }
    assert!(read_journal(directory.path())
        .unwrap()
        .dispatched(&batch.batch_id));
}

#[test]
fn the_sidecar_directory_sync_is_checked_whether_the_sidecar_is_new_or_there() {
    let directory = tempfile::tempdir().unwrap();
    let lock = lock();
    let batch = batch(&batch_id());
    let synced = std::cell::Cell::new(0);
    let sync = |result: std::io::Result<()>| {
        let synced = &synced;
        move |_: &Path| {
            synced.set(synced.get() + 1);
            result
        }
    };
    assert_eq!(
        write_pre_image_syncing(
            directory.path(),
            &lock,
            &batch,
            &pre_image(),
            sync(injected())
        )
        .unwrap_err(),
        PreImageError::DirectorySyncFailed
    );
    assert_eq!(synced.get(), 1);
    // The sidecar is in place; writing it again syncs again, and a failure is reported.
    assert_eq!(
        write_pre_image_syncing(
            directory.path(),
            &lock,
            &batch,
            &pre_image(),
            sync(injected())
        )
        .unwrap_err(),
        PreImageError::DirectorySyncFailed
    );
    assert_eq!(synced.get(), 2);
    assert!(
        write_pre_image_syncing(directory.path(), &lock, &batch, &pre_image(), sync(Ok(())))
            .is_ok()
    );
    assert_eq!(synced.get(), 3);
}

#[test]
fn a_pre_image_cannot_be_deserialised_outside_the_reader() {
    trait AmbiguousIfDeserialize<A> {
        fn probe() {}
    }
    impl<T: ?Sized> AmbiguousIfDeserialize<()> for T {}
    impl<T: serde::de::DeserializeOwned> AmbiguousIfDeserialize<u8> for T {}
    <PreImage as AmbiguousIfDeserialize<_>>::probe();
}

#[test]
fn the_summary_counts_open_intents() {
    let (first, mut records) = dispatched_batch("Synthetic One");
    let (_, second) = dispatched_batch("Synthetic Two");
    records.extend(second);
    let (_, never_sent) = dispatched_batch("Synthetic Three");
    records.push(never_sent[0].clone());
    records.push(MasterRecord::DispatchResponse(response_for(&first)));
    records.push(MasterRecord::Verdict(verdict_for(
        &first,
        MasterVerdictState::PostedVerified,
    )));
    let summary = read(&journal_text(&records)).unwrap().summary();
    assert_eq!(
        summary,
        MasterSummary {
            batches: 3,
            dispatched: 2,
            not_settled: 1
        }
    );
}

/// Removing a sidecar written for a refused dispatch: present, absent and
/// unremovable are told apart, and a name that could leave the data dir is
/// refused.
#[test]
fn a_sidecar_is_removed_checked() {
    let directory = tempfile::tempdir().unwrap();
    let batch = batch(&batch_id());
    write_pre_image(directory.path(), &lock(), &batch, &pre_image()).unwrap();
    let path = directory
        .path()
        .join(format!("master-pre-image-{}.json", batch.batch_id));
    assert!(path.exists());
    remove_pre_image(directory.path(), &lock(), &batch.batch_id).unwrap();
    assert!(!path.exists());
    // Already absent is already removed.
    remove_pre_image(directory.path(), &lock(), &batch.batch_id).unwrap();
    // A name that is not a batch id reaches no file.
    assert_eq!(
        remove_pre_image(directory.path(), &lock(), "../x"),
        Err(PreImageError::BadBatchId)
    );
    // Not removable (a directory stands where the file would be): reported.
    std::fs::create_dir(&path).unwrap();
    let error = remove_pre_image(directory.path(), &lock(), &batch.batch_id).unwrap_err();
    assert_eq!(error, PreImageError::RemoveFailed);
    assert_eq!(error.code(), "master_pre_image_remove_failed");
}

#[test]
fn two_names_differing_only_by_spaces_or_symbols_may_be_one_master() {
    let one = |a: &str, b: &str| MasterNameKeys::of(a).may_be_one_master(&MasterNameKeys::of(b));
    let (dotted, plain) = ("Synthetic Probe Pvt. Ltd.", "Synthetic Probe Pvt Ltd");
    assert_ne!(canonical_master_key(dotted), canonical_master_key(plain));
    assert!(one(dotted, plain));
    assert!(one("Synthetic Fresh Party", "Synthetic FreshParty"));
    assert!(!one("Synthetic Probe Pvt Ltd", "Synthetic Probe Pvt Ltd 2"));
}
