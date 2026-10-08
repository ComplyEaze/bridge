use super::*;

fn batch() -> ImportLedgerLine {
    ImportLedgerLine {
        ledger_identities: None,
        cash_in_hand_ledgers: Some(Vec::new()),
        on_account_approved: Some(Vec::new()),
        endpoint_origin: None,
        identity_scheme: None,
        amends_batch_id: None,
        batch_id: "bridge-00000000-0000-4000-8000-000000000001".into(),
        company_guid: GUID.into(),
        company: None,
        txn_ids: vec!["txn-001".into()],
        date_from: stored_date("20260901"),
        date_to: stored_date("20260901"),
        sha256: "a".repeat(64),
        built_at: now(),
        status: "built".into(),
        pre_import_mark: PreImportMark {
            kind: "company_high_water".into(),
            value: Some(10),
            master_value: Some(10),
        },
        vouchers: vec![admitted_payload().vouchers.remove(0)],
    }
}

fn server(path: &Path) -> Server {
    Server::new(crate::agent::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: path.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

#[tokio::test]
async fn unterminated_complete_journal_record_refuses_build_before_publication() {
    // Reuse the observed profile, company and catalogue; admission must stop
    // before requesting a pre-import mark or publishing another journal record.
    let simulator =
        SequenceSimulator::spawn(qualified_import_cycle_plans()[..12].to_vec()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server(directory.path());
    server.settings.endpoint.port = simulator.address().port();
    let journal = directory.path().join("agent-import-ledger.jsonl");
    let original = serde_json::to_vec(&batch()).unwrap();
    assert_eq!(original.last(), Some(&b'}'));
    fs::write(&journal, &original).unwrap();
    let imports = server.imports_dir().unwrap();
    let existing = imports.join("existing.xml");
    fs::write(&existing, b"retained local artifact").unwrap();

    let failure = server
        .build_import_xml(&serde_json::to_value(captured_catalogue_payload()).unwrap())
        .await
        .err()
        .expect("unterminated journal is refused");
    assert_eq!(failure.code, "import_ledger_invalid");
    assert!(failure.evidence.is_some_and(|evidence| evidence.bytes > 0));
    assert_eq!(simulator.finish().unwrap().len(), 12);
    assert_eq!(fs::read(&journal).unwrap(), original);
    assert_eq!(fs::read(&existing).unwrap(), b"retained local artifact");
    assert_eq!(fs::read_dir(&imports).unwrap().count(), 1);
}

#[test]
fn repeated_verification_appends_only_compact_status_and_preserves_batch_bytes() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let mut batch = batch();
    batch.vouchers = (0..1000)
        .map(|index| {
            let mut voucher = batch.vouchers[0].clone();
            voucher.bridge_txn_id = format!("txn-{index}");
            voucher.narration = Some("synthetic narration ".repeat(50));
            voucher
        })
        .collect();
    batch.txn_ids = batch
        .vouchers
        .iter()
        .map(|voucher| voucher.bridge_txn_id.clone())
        .collect();
    server.append_import_ledger(&batch).unwrap();
    let path = directory.path().join("agent-import-ledger.jsonl");
    let original = fs::read(&path).unwrap();
    assert!(original.len() > 1_000_000);
    for index in 0..25 {
        let status = if index % 2 == 0 {
            VerificationStatus::PostedVerified
        } else {
            VerificationStatus::VerificationIncomplete
        };
        let proof = json!({"batch_id":batch.batch_id,"company":{"name":"Synthetic Book"}});
        let generation = server
            .latest_import_snapshot(&batch.batch_id)
            .unwrap()
            .unwrap()
            .generation;
        server
            .persist_import_verification(&proof, &batch, status, generation)
            .unwrap();
    }
    let bytes = fs::read(&path).unwrap();
    assert!(bytes.starts_with(&original));
    let added = std::str::from_utf8(&bytes[original.len()..])
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    assert_eq!(added.len(), 25);
    for line in added {
        // Bounded whatever the payload: the record and the name of the
        // proof it makes current (#911).
        assert!(
            line.len() < 400,
            "verification status must not scale with payload"
        );
        let value: Value = serde_json::from_str(line).unwrap();
        assert_eq!(value["record_type"], "verification_status");
        assert_eq!(value["batch_sha256"], batch.sha256);
        assert!(value.get("vouchers").is_none());
        assert!(value.get("txn_ids").is_none());
    }
    let loaded = server
        .latest_import_snapshot(&batch.batch_id)
        .unwrap()
        .unwrap()
        .batch;
    assert_eq!(loaded.status, "posted_verified");
    assert_eq!(
        sha256_json(&serde_json::to_value(&loaded.vouchers).unwrap()),
        sha256_json(&serde_json::to_value(&batch.vouchers).unwrap())
    );
    assert_eq!(loaded.txn_ids, batch.txn_ids);
    assert_eq!(server.import_ledger().unwrap().len(), 1);
}

#[test]
fn compact_status_hydrates_legacy_full_records_and_rejects_unknown_builds() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let original = batch();
    server.append_import_ledger(&original).unwrap();
    let mut legacy_update = original.clone();
    legacy_update.status = "posted_verified".into();
    server.append_import_ledger(&legacy_update).unwrap();
    let mut current = original.clone();
    current.status = "verification_incomplete".into();
    server
        .persist_import_verification(
            &json!({}),
            &current,
            VerificationStatus::VerificationIncomplete,
            server
                .latest_import_snapshot(&current.batch_id)
                .unwrap()
                .unwrap()
                .generation,
        )
        .unwrap();
    let loaded = server
        .latest_import_snapshot(&original.batch_id)
        .unwrap()
        .unwrap()
        .batch;
    assert_eq!(loaded.status, "verification_incomplete");
    assert_eq!(loaded.txn_ids, original.txn_ids);
    assert_eq!(loaded.vouchers.len(), 1);
    assert_eq!(server.import_ledger().unwrap().len(), 2);
    let existing = fs::read_to_string(directory.path().join("agent-import-ledger.jsonl")).unwrap();
    for change in [
        json!({"batch_id":"missing"}),
        json!({"batch_sha256":"mismatch"}),
    ] {
        let mut value = serde_json::to_value(ledger::StatusRecord::from(&current)).unwrap();
        for (key, new_value) in change.as_object().unwrap() {
            value[key] = new_value.clone();
        }
        assert_eq!(
            ledger::parse_snapshots(&format!("{existing}{value}\n")).err(),
            Some("import_ledger_invalid".into())
        );
    }
    let mut unknown_record = serde_json::to_value(&original).unwrap();
    unknown_record["record_type"] = json!("future_record");
    assert_eq!(
        ledger::parse_snapshots(&format!("{unknown_record}\n")).err(),
        Some("import_ledger_invalid".into())
    );
}

#[test]
fn stale_verifier_cannot_replace_a_newer_same_batch_publication() {
    for identical_status in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let older = server(directory.path());
        let newer = server(directory.path());
        let original = batch();
        older.append_import_ledger(&original).unwrap();
        let proof = json!({"batch_id":original.batch_id,"company":{"name":"Synthetic Book"}});
        let generation = older
            .latest_import_snapshot(&original.batch_id)
            .unwrap()
            .unwrap()
            .generation;
        older
            .persist_import_verification(
                &proof,
                &original,
                VerificationStatus::VerificationIncomplete,
                generation,
            )
            .unwrap();
        // Both processes finish admission before either publishes its reads.
        let stale = older
            .latest_import_snapshot(&original.batch_id)
            .unwrap()
            .unwrap();
        let current = newer
            .latest_import_snapshot(&original.batch_id)
            .unwrap()
            .unwrap();
        let newer_status = if identical_status {
            VerificationStatus::VerificationIncomplete
        } else {
            VerificationStatus::PostedVerified
        };
        newer
            .persist_import_verification(&proof, &current.batch, newer_status, current.generation)
            .unwrap();
        let paths = [
            directory.path().join("agent-import-ledger.jsonl"),
            newer.current_proof_paths(&original.batch_id)[0].clone(),
            newer.current_proof_paths(&original.batch_id)[1].clone(),
        ];
        let before = paths
            .iter()
            .map(fs::read)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let stale_proof = json!({"batch_id":original.batch_id,"company":{"name":"Stale result"}});
        assert_eq!(
            older.persist_import_verification(
                &stale_proof,
                &stale.batch,
                VerificationStatus::VerificationIncomplete,
                stale.generation
            ),
            Err("import_verification_conflict_retry".into())
        );
        assert_eq!(
            before,
            paths
                .iter()
                .map(fs::read)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        );
        assert!(!directory.path().join("imports/.proof-publication").exists());
        if identical_status {
            let journal = fs::read_to_string(&paths[0]).unwrap();
            // Each record names its own proof; apart from that they are the same.
            let lines = journal
                .lines()
                .map(|line| {
                    let mut record: Value = serde_json::from_str(line).unwrap();
                    record.as_object_mut().unwrap().remove("proof");
                    record
                })
                .collect::<Vec<_>>();
            assert_eq!(
                lines[lines.len() - 1],
                lines[lines.len() - 2],
                "identical physical status appends still advance generation"
            );
        }
        // Retrying requires a fresh snapshot. An unrelated batch publication
        // between its admission and publication must not invalidate that token.
        // The retry records the other verdict, so the ledger shows it wrote.
        let retry_status = if newer_status == VerificationStatus::PostedVerified {
            VerificationStatus::VerificationIncomplete
        } else {
            VerificationStatus::PostedVerified
        };
        let retry = older
            .latest_import_snapshot(&original.batch_id)
            .unwrap()
            .unwrap();
        let mut other = original.clone();
        other.batch_id = "independent-batch".into();
        newer.append_import_ledger(&other).unwrap();
        let other_generation = newer
            .latest_import_snapshot(&other.batch_id)
            .unwrap()
            .unwrap()
            .generation;
        newer
            .persist_import_verification(
                &json!({"batch_id":other.batch_id}),
                &other,
                VerificationStatus::VerificationIncomplete,
                other_generation,
            )
            .unwrap();
        older
            .persist_import_verification(&stale_proof, &retry.batch, retry_status, retry.generation)
            .unwrap();
        // The current proof is the retry's, carrying the status the ledger
        // records; the newer one it follows is still on disk.
        let recorded = older
            .latest_import_snapshot(&original.batch_id)
            .unwrap()
            .unwrap()
            .batch
            .status;
        assert_eq!(recorded, retry_status.as_str());
        let mut expected = stale_proof.clone();
        expected["verification_status"] = json!(recorded);
        assert_eq!(
            serde_json::from_slice::<Value>(
                &fs::read(&older.current_proof_paths(&original.batch_id)[0]).unwrap()
            )
            .unwrap(),
            expected
        );
        assert_eq!(fs::read(&paths[1]).unwrap(), before[1]);
    }
}

/// bridge#239: a verification that reports a voucher posted records its
/// ALTERID beside the proof, once; a later verification cannot move it.
#[test]
fn a_verification_records_each_vouchers_first_verified_alter_id_once() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let batch = batch();
    server.append_import_ledger(&batch).unwrap();
    let txn_id = batch.vouchers[0].bridge_txn_id.clone();
    let verify = |alter_id: u64| {
        let proof = json!({"batch_id":batch.batch_id,
            "vouchers":[{"bridge_txn_id":txn_id,"status":"posted_verified","alter_id":alter_id}]});
        let generation = server
            .latest_import_snapshot(&batch.batch_id)
            .unwrap()
            .unwrap()
            .generation;
        server
            .persist_import_verification(
                &proof,
                &batch,
                VerificationStatus::PostedVerified,
                generation,
            )
            .unwrap();
    };
    let imports = directory.path().join("imports");
    verify(40);
    assert_eq!(
        read_verified_baseline(&imports, &batch.batch_id)
            .unwrap()
            .vouchers[&txn_id],
        40
    );
    // Verified again after someone edited a field it does not compare.
    verify(41);
    assert_eq!(
        read_verified_baseline(&imports, &batch.batch_id)
            .unwrap()
            .vouchers[&txn_id],
        40
    );
}

// Post-span verdict records (agent_import_span_identity.rs): one per batch,
// after its response, never setting the batch's status.

fn record(value: impl serde::Serialize) -> String {
    format!("{}\n", serde_json::to_string(&value).unwrap())
}

/// A batch, its native intent and its response, as a journal.
/// A native post of this build: its intent records the pre-POST mark, and its
/// response carries an outcome parsed from the committed post-span capture.
/// Only such a batch can have a post-span verdict.
fn posted_journal(line: &ImportLedgerLine) -> String {
    posted_journal_with(line, Some(1795), Some(captured_outcome()))
}

fn posted_journal_with(
    line: &ImportLedgerLine,
    mark: Option<u64>,
    outcome: Option<bridge_tally_protocol::TallyImportOutcome>,
) -> String {
    let mut intent = serde_json::to_value(ledger::StatusRecord::dispatch_native(
        line,
        "c".repeat(64),
        Uuid::new_v4(),
    ))
    .unwrap();
    if let Some(mark) = mark {
        intent["pre_post_voucher_mark"] = json!(mark);
    }
    let response = ledger::StatusRecord::response(
        line,
        ledger::DispatchResponse {
            request_sha256: "c".repeat(64),
            response_sha256: "d".repeat(64),
            bytes: 1,
            outcome,
        },
    );
    format!("{}{}{}", record(line), record(intent), record(response))
}

fn captured_outcome() -> bridge_tally_protocol::TallyImportOutcome {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/post-span-import-response.utf16le.xml"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    bridge_tally_protocol::parse_import_outcome(&xml).unwrap()
}

fn bound(line: &ImportLedgerLine) -> ledger::PostSpanVerdict {
    ledger::PostSpanVerdict::Bound(
        line.vouchers
            .iter()
            .enumerate()
            .map(|(index, voucher)| span_identity::PostedVoucherIdentity {
                bridge_txn_id: voucher.bridge_txn_id.clone(),
                guid: format!("{GUID}-{:08x}", 100 + index),
                master_id: 100 + index as u64,
            })
            .collect(),
    )
}

fn verdict_json(line: &ImportLedgerLine, verdict: &ledger::PostSpanVerdict) -> Value {
    serde_json::to_value(ledger::StatusRecord::post_span_verdict(line, verdict)).unwrap()
}

#[test]
fn a_verdict_after_the_response_reads_back_and_sets_no_status() {
    let line = batch();
    for verdict in [
        bound(&line),
        ledger::PostSpanVerdict::Refused("span_step_not_created".into()),
    ] {
        let text = format!(
            "{}{}",
            posted_journal(&line),
            record(verdict_json(&line, &verdict))
        );
        let snapshots = ledger::parse_snapshots(&text).unwrap();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].span_verdict.as_ref(), Some(&verdict));
        assert_eq!(snapshots[0].batch.status, "response_received");
        let settled = ledger::settlement(std::io::Cursor::new(text.as_bytes())).unwrap();
        assert_eq!(settled.unsettled, 1, "a verdict is not a verification");
    }
}

#[test]
fn a_verdict_that_breaks_a_rule_makes_the_journal_invalid() {
    let line = batch();
    let posted = posted_journal(&line);
    let good = verdict_json(&line, &bound(&line));
    let invalid = |text: String| ledger::parse_snapshots(&text).err();
    let expect = Some("import_ledger_invalid".to_string());
    // Before any response.
    let intent_only = posted
        .lines()
        .take(2)
        .map(|line| format!("{line}\n"))
        .collect::<String>();
    assert_eq!(invalid(format!("{intent_only}{}", record(&good))), expect);
    // A batch whose intent recorded no pre-POST mark (an older build's post),
    // and a response with no parsed outcome: the writer binds from neither.
    for unbindable in [
        posted_journal_with(&line, None, Some(captured_outcome())),
        posted_journal_with(&line, Some(1795), None),
    ] {
        assert_eq!(invalid(format!("{unbindable}{}", record(&good))), expect);
        assert!(
            ledger::parse_snapshots(&unbindable).is_ok(),
            "the journal itself is valid"
        );
    }
    // A second verdict for the batch.
    assert_eq!(
        invalid(format!("{posted}{}{}", record(&good), record(&good))),
        expect
    );
    let changed = |edit: &dyn Fn(&mut Value)| {
        let mut value = good.clone();
        edit(&mut value);
        invalid(format!("{posted}{}", record(value)))
    };
    // A transaction id the batch does not hold.
    assert_eq!(
        changed(&|value| value["bindings"][0]["bridge_txn_id"] = json!("txn-999")),
        expect
    );
    // Fewer bindings than vouchers.
    assert_eq!(changed(&|value| value["bindings"] = json!([])), expect);
    // Both bindings and a refusal, then neither.
    assert_eq!(
        changed(&|value| value["binding_refusal"] = json!("span_count_mismatch")),
        expect
    );
    assert_eq!(
        changed(&|value| {
            value.as_object_mut().unwrap().remove("bindings");
        }),
        expect
    );
    // A refusal code outside the code alphabet.
    let refused = verdict_json(
        &line,
        &ledger::PostSpanVerdict::Refused("span_count_mismatch".into()),
    );
    let mut shouting = refused.clone();
    shouting["binding_refusal"] = json!("SPAN COUNT");
    assert_eq!(invalid(format!("{posted}{}", record(shouting))), expect);
    // A verdict claiming to set a status.
    assert_eq!(
        changed(&|value| value["status"] = json!("posted_verified")),
        expect
    );
    // A GUID that is not lowercase hex.
    assert_eq!(
        changed(&|value| value["bindings"][0]["guid"] = json!("NOT-A-GUID")),
        expect
    );
    // The good one, for contrast.
    assert!(ledger::parse_snapshots(&format!("{posted}{}", record(&good))).is_ok());
}

#[test]
fn a_pre_post_mark_belongs_only_to_a_native_intent() {
    let line = batch();
    let posted = posted_journal(&line);
    let mut lines = posted
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    lines[1]["pre_post_voucher_mark"] = json!(1795);
    let marked_intent = lines.iter().map(record).collect::<String>();
    assert_eq!(
        ledger::parse_snapshots(&marked_intent).unwrap()[0].pre_post_voucher_mark,
        Some(1795)
    );
    lines[1]
        .as_object_mut()
        .unwrap()
        .remove("pre_post_voucher_mark");
    lines[2]["pre_post_voucher_mark"] = json!(1795);
    let marked_response = lines.iter().map(record).collect::<String>();
    assert_eq!(
        ledger::parse_snapshots(&marked_response).err(),
        Some("import_ledger_invalid".to_string())
    );
}

/// A saved proof is named only by a verification record, and the batch's
/// latest verification record decides which proof is current: one an older
/// build wrote, with no name, makes its single legacy file current (#911).
#[test]
fn a_proof_is_named_only_by_a_verification_record_and_the_latest_decides() {
    let line = batch();
    let mut lines = posted_journal(&line)
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    let text = |lines: &[Value]| lines.iter().map(record).collect::<String>();
    let current = |lines: &[Value]| {
        ledger::parse_snapshots(&text(lines)).unwrap()[0]
            .current_proof
            .clone()
    };
    assert_eq!(current(&lines), ledger::CurrentProof::Legacy);
    let name = ledger::ProofName::of(b"proof", chrono::Utc::now());
    let saved = serde_json::to_value(ledger::StatusRecord::verified(&line, name.clone())).unwrap();
    lines.push(saved.clone());
    assert_eq!(current(&lines), ledger::CurrentProof::Saved(name.clone()));
    let mut older = saved.clone();
    older.as_object_mut().unwrap().remove("proof");
    lines.push(older);
    assert_eq!(current(&lines), ledger::CurrentProof::Legacy);
    lines.push(saved.clone());
    assert_eq!(current(&lines), ledger::CurrentProof::Saved(name.clone()));
    // A record of another kind after it names no proof and changes nothing.
    let mut before_the_post = lines[..1].to_vec();
    before_the_post.push(saved.clone());
    before_the_post.extend(lines[1..3].iter().cloned());
    assert_eq!(
        current(&before_the_post),
        ledger::CurrentProof::Saved(name.clone())
    );
    // On any other record the name refuses the journal, as does a malformed one.
    let mut misplaced = lines.clone();
    misplaced[2]["proof"] = json!(String::from(name));
    assert_eq!(
        ledger::parse_snapshots(&text(&misplaced)).err(),
        Some("import_ledger_invalid".to_string())
    );
    let mut malformed = lines;
    let last = malformed.len() - 1;
    malformed[last]["proof"] = json!("../batch.proof.json");
    assert_eq!(
        ledger::parse_snapshots(&text(&malformed)).err(),
        Some("import_ledger_invalid".to_string())
    );
}

#[test]
fn the_aim_mark_is_read_only_from_exactly_one_target_row() {
    use crate::agent::change_parse::LoadedCompanyMarks;
    let row = |name: &str, guid: &str, vouchers| LoadedCompanyMarks {
        name: name.into(),
        guid: guid.into(),
        vouchers,
        masters: 1,
    };
    let target = row("Synthetic Accounts", GUID, 1795);
    let other = row("Other", "22222222-2222-4222-8222-222222222222", 50);
    let mark = |rows: &[LoadedCompanyMarks]| {
        post::location_target_voucher_mark_for_tests(rows, GUID, "Synthetic Accounts")
    };
    assert_eq!(mark(&[target.clone(), other.clone()]), Some(1795));
    assert_eq!(mark(&[other]), None, "no target row");
    assert_eq!(mark(&[target.clone(), target]), None, "two target rows");
}

/// A verification records its verdict only against the journal it read: if
/// anything was appended since, it records nothing and retries, and a verdict
/// already recorded is never replaced.
#[test]
fn a_verdict_is_recorded_only_against_the_journal_its_verification_read() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let line = batch();
    let journal = directory.path().join("agent-import-ledger.jsonl");
    fs::write(&journal, posted_journal(&line)).unwrap();
    let read_at = server
        .latest_import_snapshot(&line.batch_id)
        .unwrap()
        .unwrap()
        .generation;
    // Something else writes after the verification read the journal.
    {
        let _lock = server.lock_import_admission().unwrap();
        server
            .append_import_record_while_admitted(&ledger::StatusRecord::from(&line))
            .unwrap();
    }
    let before = fs::read(&journal).unwrap();
    assert_eq!(
        server
            .record_post_span_verdict(&line, &bound(&line), read_at)
            .err(),
        Some("import_verification_conflict_retry".to_string())
    );
    assert_eq!(fs::read(&journal).unwrap(), before, "nothing was recorded");
    // Against the current journal it records, once.
    let current = server
        .latest_import_snapshot(&line.batch_id)
        .unwrap()
        .unwrap()
        .generation;
    let (recorded, after) = server
        .record_post_span_verdict(&line, &bound(&line), current)
        .unwrap();
    assert_eq!(recorded, bound(&line));
    assert!(after != current);
    assert_eq!(
        server
            .record_post_span_verdict(
                &line,
                &ledger::PostSpanVerdict::Refused("span_count_mismatch".into()),
                after
            )
            .err(),
        Some("import_verification_conflict_retry".to_string())
    );
    let snapshot = server
        .latest_import_snapshot(&line.batch_id)
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.span_verdict, Some(bound(&line)));
}

/// A GUID another batch already bound is never bound again: checked under the
/// lock the verdict is recorded under, it is recorded as a refusal instead.
#[test]
fn a_guid_bound_by_another_batch_is_recorded_as_a_refusal() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path());
    let first = batch();
    let mut second = batch();
    second.batch_id = "bridge-00000000-0000-4000-8000-000000000002".into();
    let journal = directory.path().join("agent-import-ledger.jsonl");
    fs::write(
        &journal,
        format!(
            "{}{}{}",
            posted_journal(&first),
            record(verdict_json(&first, &bound(&first))),
            posted_journal(&second)
        ),
    )
    .unwrap();
    let at = server
        .latest_import_snapshot(&second.batch_id)
        .unwrap()
        .unwrap()
        .generation;
    let (recorded, _) = server
        .record_post_span_verdict(&second, &bound(&second), at)
        .unwrap();
    assert_eq!(
        recorded,
        ledger::PostSpanVerdict::Refused("span_identity_reused".into())
    );
}

/// A stored date that is not `YYYYMMDD` refuses the whole journal, as any
/// other malformed record does, for the import tools and the local data
/// report alike. No release has written one: every build stored the
/// normalised date (#1307).
#[test]
fn a_stored_date_that_is_not_yyyymmdd_refuses_the_journal() {
    let mut line = batch();
    line.company = Some(ImportCompanyTuple {
        name: "Synthetic Accounts".into(),
        guid: GUID.into(),
        company_number: "1".into(),
        books_from: stored_date("20260401"),
    });
    let stored = serde_json::to_value(&line).unwrap();
    let text = format!("{stored}\n");
    assert_eq!(ledger::parse_snapshots(&text).unwrap().len(), 1);
    assert!(ledger::settlement(text.as_bytes()).is_ok());
    for pointer in [
        "/date_from",
        "/date_to",
        "/vouchers/0/date",
        "/company/books_from",
    ] {
        for date in ["2026-09-01", "20260931", ""] {
            let mut edited = stored.clone();
            *edited.pointer_mut(pointer).unwrap() = json!(date);
            let text = format!("{edited}\n");
            assert_eq!(
                ledger::parse_snapshots(&text).err(),
                Some("import_ledger_invalid".to_string()),
                "{pointer} {date:?}"
            );
            assert_eq!(
                ledger::settlement(text.as_bytes()).err(),
                Some("import_ledger_invalid".to_string()),
                "{pointer} {date:?}"
            );
        }
    }
}
