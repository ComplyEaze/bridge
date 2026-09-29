//! Two same-day, same-amount payments of one account are two statement rows,
//! not one voucher posted twice (#865). Each test asserts the verdict the
//! admission reads, never a message substring.
use super::*;

const OTHER_COMPANY: &str = "00000000-0000-4000-8000-0000000000ff";

fn key(account: &str, balance: &str) -> StatementRowKey {
    StatementRowKey {
        account: account.into(),
        balance: balance.into(),
    }
}

/// A build of one payment whose statement row is `key`, or an inline one.
fn build_of(txn: &str, row: Option<StatementRowKey>) -> ImportLedgerLine {
    let mut voucher = payload().vouchers.remove(0);
    voucher.bridge_txn_id = txn.into();
    ImportLedgerLine {
        ledger_identities: None,
        statement_rows: row.map(|row| BTreeMap::from([(txn.to_string(), row)])),
        endpoint_origin: None,
        identity_scheme: None,
        amends_batch_id: None,
        batch_id: format!("batch-{txn}"),
        company_guid: GUID.into(),
        company: None,
        txn_ids: vec![txn.into()],
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
        vouchers: vec![voucher],
    }
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
        narration: Some(format!("[BRIDGE:{}]", voucher.bridge_txn_id)),
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
        for voucher in &line.vouchers {
            if let Some(row) = line
                .statement_rows
                .as_ref()
                .and_then(|rows| rows.get(&voucher.bridge_txn_id))
            {
                rows.insert(
                    line.attribution_tag(voucher),
                    (line.company_guid.clone(), row.clone()),
                );
            }
        }
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
    let earlier = build_of("st-20260901-aaaa", Some(key("acct-1", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("acct-1", "800.00")));
    let rows = journal(&[&earlier, &later]);
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(status_of(&result), "not_found");
    assert_eq!(admission(&result), Ok(()));
}

#[test]
fn the_same_statement_row_imported_again_is_still_refused() {
    let earlier = build_of("st-20260901-aaaa", Some(key("acct-1", "900.00")));
    let again = build_of("st-20260901-cccc", Some(key("acct-1", "900.00")));
    let rows = journal(&[&earlier, &again]);
    let result = verify(&again, vec![posted(&earlier, 1)], &rows);
    assert_eq!(status_of(&result), "matching_content_observed");
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn a_twin_from_another_account_is_still_refused() {
    // A Contra shows as a payment on one account and a receipt on the other,
    // and a different account's balance proves nothing about the same row.
    let earlier = build_of("st-20260901-aaaa", Some(key("acct-1", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("acct-2", "800.00")));
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
        let earlier = build_of("st-20260901-aaaa", Some(key("acct-1", earlier_balance)));
        let later = build_of("st-20260901-bbbb", Some(key("acct-1", later_balance)));
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
fn an_account_with_no_digest_is_still_refused() {
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
    let earlier = build_of("st-20260901-aaaa", Some(key("acct-1", "900.00")));
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
    let later = build_of("st-20260901-bbbb", Some(key("acct-1", "800.00")));
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
    let earlier = build_of("st-20260901-aaaa", Some(key("acct-1", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("acct-1", "800.00")));
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
    let earlier = build_of("st-20260901-aaaa", Some(key("acct-1", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("acct-1", "800.00")));
    let rows = journal(&[&later]);
    let result = verify(&later, vec![posted(&earlier, 1)], &rows);
    assert_eq!(
        admission(&result),
        Err("import_preexisting_identity".into())
    );
}

#[test]
fn both_rows_verify_after_both_are_posted_and_a_repeat_still_blocks() {
    let first = build_of("st-20260901-aaaa", Some(key("acct-1", "900.00")));
    let second = build_of("st-20260901-bbbb", Some(key("acct-1", "800.00")));
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
    let repeat = build_of("st-20260901-cccc", Some(key("acct-1", "900.00")));
    let rows = journal(&[&first, &second, &repeat]);
    let mut observed = both();
    observed.push(posted(&repeat, 3));
    let result = verify(&second, observed, &rows);
    assert!(!result["duplicates"].as_array().unwrap().is_empty());
    assert_eq!(verification_status(&result, 1), "verification_incomplete");
}

#[test]
fn the_verification_result_never_carries_the_account_key_or_balance() {
    let earlier = build_of("st-20260901-aaaa", Some(key("acctdigest0001", "900.00")));
    let later = build_of("st-20260901-bbbb", Some(key("acctdigest0001", "800.00")));
    let rows = journal(&[&earlier, &later]);
    let observed = vec![posted(&earlier, 1), posted(&later, 2)];
    let result = verify(&later, observed, &rows).to_string();
    for private in ["acctdigest0001", "900.00", "800.00"] {
        assert!(!result.contains(private), "{private} in {result}");
    }
}

#[test]
fn the_journal_read_returns_every_batchs_rows_with_its_company() {
    let first = build_of("st-20260901-aaaa", Some(key("acct-1", "900.00")));
    let second = build_of("st-20260901-bbbb", Some(key("acct-1", "800.00")));
    let inline = build_of("inline-1", None);
    let mut bytes = Vec::new();
    for line in [&first, &second, &inline] {
        bytes.extend(serde_json::to_vec(line).unwrap());
        bytes.push(b'\n');
    }
    let snapshot = ledger::read_snapshot(std::io::Cursor::new(bytes), Some(&second.batch_id))
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.statement_rows, journal(&[&first, &second]));
    assert_eq!(snapshot.statement_rows.len(), 2);
    assert_eq!(snapshot.statement_rows["st-20260901-aaaa"].0, GUID);
}
