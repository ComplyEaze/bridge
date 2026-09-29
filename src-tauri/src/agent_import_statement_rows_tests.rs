//! Two same-day, same-amount payments of one statement are two statement rows,
//! not one voucher posted twice (#865). Each test asserts the verdict the
//! admission reads, never a message substring.
use super::*;

const OTHER_COMPANY: &str = "00000000-0000-4000-8000-0000000000ff";

fn key(statement: &str, balance: &str) -> StatementRowKey {
    StatementRowKey {
        statement: statement.into(),
        balance: balance.into(),
    }
}

/// A build of one payment whose statement row is `key`, or an inline one.
fn build_of(txn: &str, row: Option<StatementRowKey>) -> ImportLedgerLine {
    build_many(&[(txn, row)])
}

/// A build of one identical payment per entry, each with its own row key.
fn build_many(rows: &[(&str, Option<StatementRowKey>)]) -> ImportLedgerLine {
    let vouchers: Vec<ImportVoucher> = rows
        .iter()
        .map(|(txn, _)| {
            let mut voucher = payload().vouchers.remove(0);
            voucher.bridge_txn_id = (*txn).into();
            voucher
        })
        .collect();
    let keys: BTreeMap<String, StatementRowKey> = rows
        .iter()
        .filter_map(|(txn, row)| Some((txn.to_string(), row.clone()?)))
        .collect();
    ImportLedgerLine {
        ledger_identities: None,
        statement_rows: (!keys.is_empty()).then_some(keys),
        endpoint_origin: None,
        identity_scheme: None,
        amends_batch_id: None,
        batch_id: format!("batch-{}", rows[0].0),
        company_guid: GUID.into(),
        company: None,
        txn_ids: vouchers.iter().map(|v| v.bridge_txn_id.clone()).collect(),
        date_from: "20260901".into(),
        date_to: "20260901".into(),
        sha256: "hash".into(),
        built_at: now(),
        status: "built".into(),
        pre_import_mark: PreImportMark {
            kind: "company_high_water".into(),
            value: Some(10),
            master_value: Some(10),
        },
        vouchers,
    }
}

/// `line` as a build from a statement writes it: its tag is the batch identity,
/// not the bare `bridge_txn_id`.
fn batch_v1(mut line: ImportLedgerLine) -> ImportLedgerLine {
    line.identity_scheme = Some(ImportIdentityScheme::BatchV1);
    line
}

/// The voucher Tally holds after `line` posted, as a verification read shows it.
fn posted(line: &ImportLedgerLine, index: u64) -> ReadVoucher {
    let voucher = &line.vouchers[0];
    ReadVoucher {
        remote_id: Some(format!("remote-{index}")),
        guid: Some(format!("guid-{index}")),
        master_id: Some(index.to_string()),
        alter_id: Some(20 + index),
        date: Some(normalized_date(&voucher.date).unwrap()),
        voucher_type: Some(voucher.voucher_type.as_str().into()),
        narration: Some(format!("[BRIDGE:{}]", line.attribution_tag(voucher))),
        voucher_number: None,
        cancelled: Some(false),
        optional: Some(false),
        effective_date: None,
        entries: voucher
            .entries
            .iter()
            .map(|entry| ReadEntry {
                ledger: entry.ledger.clone(),
                amount: match entry.side {
                    EntrySide::Dr => format!("-{}", entry.amount),
                    EntrySide::Cr => entry.amount.clone(),
                },
                is_deemed_positive: entry.side.tally_positive().into(),
            })
            .collect(),
    }
}

/// What the journal read returns for these builds, all of company `GUID`.
fn journal(lines: &[&ImportLedgerLine]) -> ledger::StatementRows {
    let mut rows = ledger::StatementRows::new();
    for line in lines {
        ledger::note_statement_rows(&mut rows, line);
    }
    rows
}

fn verify(
    line: &ImportLedgerLine,
    observed: Vec<ReadVoucher>,
    rows: &ledger::StatementRows,
) -> Value {
    verify_batch(line, &ImportReadSource::admit(observed).unwrap(), rows).unwrap()
}

fn status_of(result: &Value) -> &str {
    result["vouchers"][0]["status"].as_str().unwrap()
}

/// The pre-post absence check the admission runs, as its typed verdict.
fn admission(result: &Value) -> Result<(), String> {
    post::require_absent_verification_result(result, 1)
}

#[test]
fn a_twin_from_another_statement_row_does_not_block_the_next_batch() {
    let earlier = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00")));
    let rows = journal(&[&earlier, &later]);
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(status_of(&result), "not_found");
    assert_eq!(admission(&result), Ok(()));
}

#[test]
fn the_same_statement_row_imported_again_is_still_refused() {
    let earlier = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let again = build_of("st-20260901-cccc", Some(key("stmt-1", "900.00")));
    let rows = journal(&[&earlier, &again]);
    let result = verify(&again, vec![posted(&earlier, 1)], &rows);
    assert_eq!(status_of(&result), "matching_content_observed");
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_twin_read_from_another_statement_file_is_still_refused() {
    // A running balance depends on where a row sits in the day's order, and a
    // second export of one account can order it differently: the same row can
    // print two balances, so balances of two files prove nothing.
    let earlier = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("stmt-2", "800.00")));
    let rows = journal(&[&earlier, &later]);
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(status_of(&result), "matching_content_observed");
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_statement_without_a_balance_column_is_still_refused() {
    for (earlier_balance, later_balance) in [("", "800.00"), ("900.00", ""), ("", "")] {
        let earlier = build_of("st-20260901-aaaa", Some(key("stmt-1", earlier_balance)));
        let later = build_of("st-20260901-bbbb", Some(key("stmt-1", later_balance)));
        let rows = journal(&[&earlier, &later]);
        let result = verify(&later, vec![posted(&earlier, 1)], &rows);
        assert_eq!(
            admission(&result),
            Err("import_preexisting_identity".into()),
            "{earlier_balance:?} / {later_balance:?}"
        );
    }
}

#[test]
fn a_row_with_no_statement_hash_is_still_refused() {
    let earlier = build_of("st-20260901-aaaa", Some(key("", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("", "800.00")));
    let rows = journal(&[&earlier, &later]);
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn an_inline_batch_colliding_with_a_recorded_one_is_still_refused() {
    let earlier = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let inline = build_of("inline-1", None);
    let rows = journal(&[&earlier, &inline]);
    let result = verify(&inline, vec![posted(&earlier, 1)], &rows);
    assert_eq!(status_of(&result), "matching_content_observed");
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_recorded_batch_colliding_with_an_inline_one_is_still_refused() {
    let inline = build_of("inline-1", None);
    let later = build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00")));
    let rows = journal(&[&inline, &later]);
    let result = verify(&later, vec![posted(&inline, 1)], &rows);
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_legacy_record_with_no_row_key_behaves_as_before() {
    let raw = serde_json::to_value(build_of("st-20260901-aaaa", None)).unwrap();
    assert!(raw.get("statement_rows").is_none(), "{raw}");
    let read: ImportLedgerLine = serde_json::from_value(raw).unwrap();
    assert!(read.statement_rows.is_none());
    let earlier = build_of("st-20260901-aaaa", None);
    let later = build_of("st-20260901-bbbb", None);
    let result = verify(
        &later,
        vec![posted(&earlier, 1)],
        &journal(&[&earlier, &later]),
    );
    assert_eq!(status_of(&result), "matching_content_observed");
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_row_recorded_for_another_company_proves_nothing_here() {
    let earlier = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00")));
    let mut rows = journal(&[&earlier, &later]);
    rows.get_mut("st-20260901-aaaa").unwrap().0 = OTHER_COMPANY.into();
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(status_of(&result), "matching_content_observed");
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_tag_the_journal_never_recorded_is_still_refused() {
    let earlier = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00")));
    let rows = journal(&[&later]);
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn both_rows_verify_after_both_are_posted_and_a_repeat_still_blocks() {
    let first = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let second = build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00")));
    let rows = journal(&[&first, &second]);
    let both = || vec![posted(&first, 1), posted(&second, 2)];
    for line in [&first, &second] {
        let result = verify(line, both(), &rows);
        assert_eq!(status_of(&result), "posted_verified");
        assert_eq!(result["duplicates"], json!([]));
        assert_eq!(verification_status(&result, 1), "posted_verified");
    }
    // Without the journal's keys the same read is a duplicate, as before.
    let unkeyed = verify(&first, both(), &ledger::StatementRows::new());
    assert!(!unkeyed["duplicates"].as_array().unwrap().is_empty());
    assert_eq!(verification_status(&unkeyed, 1), "verification_incomplete");
    // A third voucher of the first row's key is a real duplicate of it.
    let repeat = build_of("st-20260901-cccc", Some(key("stmt-1", "900.00")));
    let rows = journal(&[&first, &second, &repeat]);
    let mut observed = both();
    observed.push(posted(&repeat, 3));
    let result = verify(&second, observed, &rows);
    assert!(!result["duplicates"].as_array().unwrap().is_empty());
    assert_eq!(verification_status(&result, 1), "verification_incomplete");
}

#[test]
fn the_verification_result_never_carries_the_statement_key_or_balance() {
    let earlier = build_of("st-20260901-aaaa", Some(key("stmtsha256-0001", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("stmtsha256-0001", "800.00")));
    let rows = journal(&[&earlier, &later]);
    let observed = vec![posted(&earlier, 1), posted(&later, 2)];
    let result = verify(&later, observed, &rows).to_string();
    for private in ["stmtsha256-0001", "900.00", "800.00"] {
        assert!(!result.contains(private), "{private} in {result}");
    }
}

#[test]
fn a_batch_holding_a_new_row_and_a_repeat_is_still_refused() {
    // The book holds a row printed 900.00. The next batch has a genuinely new
    // row (800.00) and a re-import of the first (900.00): the new row must not
    // clear the repeat, or the repeat posts twice.
    let earlier = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let mixed = build_many(&[
        ("st-20260901-bbbb", Some(key("stmt-1", "800.00"))),
        ("st-20260901-cccc", Some(key("stmt-1", "900.00"))),
    ]);
    let rows = journal(&[&earlier, &mixed]);
    let observed = ImportReadSource::admit(vec![posted(&earlier, 1)]).unwrap();
    let result = verify_batch(&mixed, &observed, &rows).unwrap();
    assert_eq!(
        post::require_absent_verification_result(&result, 2),
        Err("import_preexisting_identity".into())
    );
    // With only the new rows in the batch, the same twin does not block it.
    let fresh = build_many(&[
        ("st-20260901-bbbb", Some(key("stmt-1", "800.00"))),
        ("st-20260901-dddd", Some(key("stmt-1", "700.00"))),
    ]);
    let rows = journal(&[&earlier, &fresh]);
    let result = verify_batch(&fresh, &observed, &rows).unwrap();
    assert_eq!(post::require_absent_verification_result(&result, 2), Ok(()));
}

#[test]
fn a_group_with_one_unkeyed_voucher_is_still_a_duplicate() {
    let first = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let second = build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00")));
    let inline = build_of("inline-1", None);
    let rows = journal(&[&first, &second, &inline]);
    let observed = vec![posted(&first, 1), posted(&second, 2), posted(&inline, 3)];
    let result = verify(&second, observed, &rows);
    assert!(!result["duplicates"].as_array().unwrap().is_empty());
    assert_eq!(verification_status(&result, 1), "verification_incomplete");
}

#[test]
fn three_distinct_rows_all_verify() {
    let first = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let second = build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00")));
    let third = build_of("st-20260901-cccc", Some(key("stmt-1", "700.00")));
    let rows = journal(&[&first, &second, &third]);
    let all = || vec![posted(&first, 1), posted(&second, 2), posted(&third, 3)];
    for line in [&first, &second, &third] {
        let result = verify(line, all(), &rows);
        assert_eq!(status_of(&result), "posted_verified");
        assert_eq!(result["duplicates"], json!([]));
    }
}

#[test]
fn a_build_tagged_by_batch_identity_finds_its_recorded_row() {
    let earlier = batch_v1(build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00"))));
    let later = batch_v1(build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00"))));
    let tag = earlier.attribution_tag(&earlier.vouchers[0]);
    assert_ne!(tag, "st-20260901-aaaa");
    let rows = journal(&[&earlier, &later]);
    assert!(rows.contains_key(&tag));
    assert!(!rows.contains_key("st-20260901-aaaa"));
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(status_of(&result), "not_found");
    assert_eq!(admission(&result), Ok(()));
}

#[test]
fn the_journal_read_returns_every_batchs_rows_with_its_company() {
    let directory = tempfile::tempdir().expect("temporary data directory");
    let server = Server::new(crate::agent::Settings {
        endpoint: bridge_tally_transport::TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: crate::agent::Redaction::None,
        import_enabled: true,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let first = batch_v1(build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00"))));
    let second = batch_v1(build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00"))));
    let inline = batch_v1(build_of("inline-1", None));
    for line in [&first, &second, &inline] {
        server.append_import_ledger(line).expect("saved batch");
    }
    let snapshot = server
        .latest_import_snapshot(&second.batch_id)
        .unwrap()
        .unwrap();
    let first_tag = first.attribution_tag(&first.vouchers[0]);
    let second_tag = second.attribution_tag(&second.vouchers[0]);
    assert_eq!(snapshot.statement_rows.len(), 2);
    assert_eq!(
        snapshot.statement_rows[&first_tag],
        (GUID.to_string(), key("stmt-1", "900.00"))
    );
    assert_eq!(
        snapshot.statement_rows[&second_tag],
        (GUID.to_string(), key("stmt-1", "800.00"))
    );
}

#[test]
fn a_build_records_the_row_key_of_each_voucher_that_has_one() {
    let keyed = build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00")));
    let mut row_keys = BTreeMap::new();
    row_keys.insert(
        keyed.vouchers[0].bridge_txn_id.clone(),
        key("stmt-1", "900.00"),
    );
    row_keys.insert("some-other-txn".to_string(), key("stmt-1", "1.00"));
    let recorded = recorded_statement_rows(&keyed.vouchers, &row_keys).unwrap();
    assert_eq!(recorded.len(), 1);
    assert_eq!(
        recorded[&keyed.vouchers[0].bridge_txn_id],
        key("stmt-1", "900.00")
    );
    assert!(recorded_statement_rows(&keyed.vouchers, &BTreeMap::new()).is_none());
}

#[test]
fn a_rebuilt_batch_that_remaps_a_posted_row_is_still_refused() {
    // Batch A posted the 900.00 row. A rebuild (not an amendment) maps that
    // row to another ledger and adds its 800.00 twin: the twin's exemption
    // must not let the remapped, already-posted row post a second time.
    let earlier = batch_v1(build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00"))));
    let mut rebuilt = batch_v1(build_many(&[
        ("st-20260901-cccc", Some(key("stmt-1", "900.00"))),
        ("st-20260901-bbbb", Some(key("stmt-1", "800.00"))),
    ]));
    rebuilt.vouchers[0].entries[0].ledger = "Rent Paid".into();
    let rows = journal(&[&earlier, &rebuilt]);
    let result = verify(&rebuilt, vec![posted(&earlier, 1)], &rows);
    assert_eq!(
        post::require_absent_verification_result(&result, 2),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_copied_narration_tag_is_not_a_row_identity() {
    // The row was entered by hand by duplicating the posted voucher in Tally,
    // so two observed vouchers carry one tag. The tag then proves nothing.
    let earlier = batch_v1(build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00"))));
    let later = batch_v1(build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00"))));
    let rows = journal(&[&earlier, &later]);
    let original = posted(&earlier, 1);
    let mut copy = posted(&earlier, 2);
    copy.narration = original.narration.clone();
    let result = verify(&later, vec![original, copy], &rows);
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_recorded_company_guid_matches_in_any_letter_case() {
    let earlier = batch_v1(build_of("st-20260901-aaaa", Some(key("stmt-1", "900.00"))));
    let later = batch_v1(build_of("st-20260901-bbbb", Some(key("stmt-1", "800.00"))));
    let mut rows = journal(&[&earlier, &later]);
    for (company, _) in rows.values_mut() {
        *company = company.to_uppercase();
    }
    assert_ne!(rows.values().next().unwrap().0, GUID);
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(admission(&result), Ok(()));
}
