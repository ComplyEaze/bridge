//! Master journal tests: local files and synthetic identifiers only.
use super::*;
use serde_json::json;
use std::io::Cursor;
use uuid::Uuid;

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
            guid: "00000000-0000-0000-0000-000000000001".into(),
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
            .map(|item| IntentItem {
                name: item.name.clone(),
                canonical_key: item.name.to_lowercase(),
                absence_key: format!("absent:{}", item.name.to_lowercase()),
            })
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
    MasterVerdict {
        batch_id: batch.batch_id.clone(),
        batch_sha256: batch.batch_sha256.clone(),
        state,
        items: batch
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| VerdictItem {
                name: item.name.clone(),
                guid: Some(format!("0000000{index}-aaaa-bbbb-cccc-000000000000")),
                reason: None,
            })
            .collect(),
    }
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
        assert_eq!(
            &serde_json::from_str::<MasterRecord>(&text).unwrap(),
            record
        );
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
    // The identical batch written twice is one batch.
    assert!(read(&journal_text(&[
        MasterRecord::Batch(first.clone()),
        MasterRecord::Batch(first)
    ]))
    .is_ok());
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
fn the_latest_verdict_and_response_win() {
    let batch = batch(&batch_id());
    let mut records = full_records(&batch);
    records.insert(
        3,
        MasterRecord::Verdict(verdict_for(
            &batch,
            MasterVerdictState::MastersAfterPostUnconfirmed,
        )),
    );
    let journal = read(&journal_text(&records)).unwrap();
    assert_eq!(
        journal
            .verdict(&batch.batch_id)
            .map(|verdict| verdict.state),
        Some(MasterVerdictState::PostedVerified)
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
    blank_company.company.guid = " ".into();
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
        company_guid: batch.company.guid.to_uppercase(),
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
    upper.items[0].guid = Some("ABCDEF00-0000-0000-0000-000000000000".into());
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
    for record in full_records(&batch) {
        append(directory.path(), record).unwrap();
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
    for record in full_records(&batch) {
        append(directory.path(), record).unwrap();
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
    // Nothing is written for an intent whose batch is not there.
    assert_eq!(
        append(
            directory.path(),
            MasterRecord::DispatchIntent(intent_for(&batch))
        )
        .unwrap_err(),
        invalid(Invalid::NoBatch)
    );
    assert!(!path.exists());
    append(directory.path(), MasterRecord::Batch(batch.clone())).unwrap();
    append(
        directory.path(),
        MasterRecord::DispatchIntent(intent_for(&batch)),
    )
    .unwrap();
    let before = std::fs::read(&path).unwrap();
    let error = append(
        directory.path(),
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
        append(directory.path(), MasterRecord::Batch(batch(&batch_id()))).unwrap_err(),
        invalid(Invalid::Parse)
    );
}

fn pre_image() -> PreImage {
    PreImage::new([
        ("BBBBBBBB-0000-0000-0000-000000000002".to_string(), 12),
        ("aaaaaaaa-0000-0000-0000-000000000001".to_string(), 11),
    ])
    .unwrap()
}

#[test]
fn the_pre_image_is_sorted_lower_case_and_round_trips_through_its_sidecar() {
    let directory = tempfile::tempdir().unwrap();
    let id = batch_id();
    let image = pre_image();
    let sha = write_pre_image(directory.path(), &id, &image).unwrap();
    let path = directory.path().join(format!("master-pre-image-{id}.json"));
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(
        String::from_utf8(bytes.clone()).unwrap(),
        "{\"aaaaaaaa-0000-0000-0000-000000000001\":11,\"bbbbbbbb-0000-0000-0000-000000000002\":12}"
    );
    assert_eq!(sha, sha256_hex(&bytes));
    assert_eq!(
        read_pre_image(directory.path(), &id, &sha, 2).unwrap(),
        image
    );
    // The same write repeated is accepted; different bytes under the name are not.
    assert_eq!(write_pre_image(directory.path(), &id, &image).unwrap(), sha);
    let other = PreImage::new([("cccccccc-0000-0000-0000-000000000003".to_string(), 1)]).unwrap();
    assert_eq!(
        write_pre_image(directory.path(), &id, &other).unwrap_err(),
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
    let sha = write_pre_image(directory.path(), &id, &pre_image()).unwrap();
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
    for (bytes, expected) in [
        (&b"[1,2]"[..], PreImageError::Malformed),
        (
            &b"{\"AAAAAAAA-0000-0000-0000-000000000001\":1}"[..],
            PreImageError::Malformed,
        ),
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
    for id in [
        "../escape",
        "bridge-../../escape",
        "bridge-00000000-0000-4000-8000-000000000000/../../x",
        "",
        "/etc/passwd",
        "bridge-not-a-uuid",
    ] {
        assert_eq!(
            write_pre_image(&nested, id, &pre_image()).unwrap_err(),
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
