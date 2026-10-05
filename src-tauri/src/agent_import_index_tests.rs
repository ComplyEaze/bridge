use super::*;

#[test]
fn captured_derived_large_verification_preserves_tag_and_fallback_multiplicity() {
    // An in-memory workload derived from a captured accounting row. Repeated
    // identities, tags and Journal type below are synthetic, not live evidence.
    let mut template = parse_import_vouchers(&boundary_tests::captured_vouchers(), CAPTURED_GUID)
        .unwrap()
        .rows
        .remove(0);
    template.voucher_type = Some("Journal".into());
    template.voucher_number = None;
    let date = template.date.as_deref().unwrap();
    let expected = ImportVoucher {
        bridge_txn_id: String::new(),
        date: format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..]),
        voucher_type: VoucherType::Journal,
        narration: None,
        reference: None,
        voucher_number: None,
        entries: template
            .entries
            .iter()
            .map(|entry| ImportEntry {
                ledger: entry.ledger.clone(),
                amount: entry.amount.trim_start_matches('-').into(),
                side: if entry.is_deemed_positive == "Yes" {
                    EntrySide::Dr
                } else {
                    EntrySide::Cr
                },
            })
            .collect(),
    };
    let vouchers = (0..1000)
        .map(|index| {
            let mut row = expected.clone();
            row.bridge_txn_id = format!("scale-{index}");
            row
        })
        .collect::<Vec<_>>();
    let line = ImportLedgerLine {
        ledger_identities: None,
        cash_in_hand_ledgers: Some(Vec::new()),
        endpoint_origin: None,
        identity_scheme: None,
        amends_batch_id: None,
        batch_id: "scale-batch".into(),
        company_guid: GUID.into(),
        company: None,
        txn_ids: vouchers
            .iter()
            .map(|row| row.bridge_txn_id.clone())
            .collect(),
        date_from: date.into(),
        date_to: date.into(),
        sha256: "synthetic".into(),
        built_at: now(),
        status: "built".into(),
        pre_import_mark: PreImportMark {
            kind: "company_high_water".into(),
            value: Some(10),
            master_value: Some(10),
        },
        vouchers,
    };
    for tagged in [false, true] {
        let observed = (0..10000)
            .map(|index| {
                let mut row = template.clone();
                row.guid = Some(format!("scale-guid-{index}"));
                row.master_id = Some(index.to_string());
                row.remote_id = None;
                row.alter_id = Some(11);
                row.narration = (tagged && index < 1000).then(|| format!("[BRIDGE:scale-{index}]"));
                row
            })
            .collect::<Vec<_>>();
        let started = std::time::Instant::now();
        let result = verify_observed_batch(&line, &observed).unwrap();
        eprintln!(
            "scaled verification: expected=1000 observed=10000 tagged={tagged} elapsed_ms={}",
            started.elapsed().as_millis()
        );
        assert_eq!(
            result["counts"]["posted_verified"],
            if tagged { 1000 } else { 0 }
        );
        assert_eq!(
            result["counts"]["duplicate_fingerprint"],
            if tagged { 0 } else { 1000 }
        );
        assert_eq!(result["duplicates"].as_array().unwrap().len(), 1);
        assert_eq!(
            verification_status(&result, 1000),
            "verification_incomplete"
        );
        assert!(result["duplicates"][0]["fingerprint"].is_null());
        assert!(result["duplicates"][0]["fingerprint_sha256"].is_string());
    }
}

#[test]
fn delimiter_bearing_ledger_names_do_not_create_accounting_duplicates() {
    // Pure matching fault case; these rows do not claim a live Tally capture.
    let mut first = parse_import_vouchers(&boundary_tests::captured_vouchers(), CAPTURED_GUID)
        .unwrap()
        .rows
        .remove(0);
    first.entries = vec![
        ReadEntry {
            ledger: "A".into(),
            amount: "1".into(),
            is_deemed_positive: "No".into(),
        },
        ReadEntry {
            ledger: "B".into(),
            amount: "1".into(),
            is_deemed_positive: "No".into(),
        },
    ];
    first.remote_id = None;
    let mut second = first.clone();
    second.guid = Some("different-guid".into());
    second.master_id = Some("999".into());
    second.entries = vec![ReadEntry {
        ledger: "A|1|No,B".into(),
        amount: "1".into(),
        is_deemed_positive: "No".into(),
    }];
    let rows = vec![first, second];
    let fingerprints = rows.iter().map(observed_fingerprint).collect::<Vec<_>>();
    assert_ne!(fingerprints[0], fingerprints[1]);
    assert_eq!(fingerprints[0].2.join(","), fingerprints[1].2.join(","));
    let identities = rows
        .iter()
        .map(observed_voucher_identity)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let expected = BTreeMap::from([(&fingerprints[0], 1)]);
    let (batch, unrelated) = batch_duplicate_sets(
        &rows,
        &identities,
        &fingerprints,
        &expected,
        &[None, None],
        &BTreeSet::new(),
        &BTreeSet::new(),
    );
    assert!(
        batch.is_empty(),
        "distinct entry vectors must not block a verified batch"
    );
    assert!(unrelated.is_empty());
}

/// The verification rows of the captured D3 readback in which a person
/// cancelled D3-003 at Tally's screen (fixtures `d3-cancelled-*`).
fn d3_cancelled_rows() -> Vec<ReadVoucher> {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/d3-cancelled-import-verification.utf16le.xml"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    parse_import_vouchers(&xml, "17a10910-773c-42c6-bd66-7bba9a392536")
        .unwrap()
        .rows
}

/// A second voucher derived from a captured row: another identity and
/// REMOTEID, no Bridge marker, the same date, type and entries.
fn another_voucher(row: &ReadVoucher, id: &str) -> ReadVoucher {
    let mut other = row.clone();
    other.guid = Some(format!("derived-guid-{id}"));
    other.master_id = Some(format!("9{id}"));
    other.remote_id = Some(format!("derived-remote-{id}"));
    other.narration = None;
    other
}

#[test]
fn cancelled_vouchers_of_one_date_and_type_are_not_accounting_duplicates() {
    // Tally drops a cancelled voucher's entries from this read, so every cancel
    // of one date and type fingerprints alike (bridge#767). The cancelled row is
    // D3-003 as captured; its partners are derived from it in memory.
    let rows = d3_cancelled_rows();
    let cancelled = rows
        .iter()
        .filter(|row| row.cancelled == Some(true))
        .collect::<Vec<_>>();
    assert_eq!(cancelled.len(), 1, "the capture holds one cancel");
    let cancelled = cancelled[0].clone();
    assert!(cancelled.entries.is_empty());
    let kinds = |found: Vec<Value>| {
        found
            .iter()
            .map(|item| item["kind"].as_str().unwrap().to_string())
            .collect::<Vec<_>>()
    };
    let other = another_voucher(&cancelled, "1");
    assert_eq!(
        kinds(test_duplicates(&[cancelled.clone(), other.clone()]).unwrap()),
        Vec::<String>::new()
    );
    // The REMOTEID map is unchanged. (A real read does not produce this shape:
    // a readback's REMOTEID derives from its GUID, which admission keeps unique.)
    let mut same_remote = other;
    same_remote.remote_id = cancelled.remote_id.clone();
    assert_eq!(
        kinds(test_duplicates(&[cancelled, same_remote]).unwrap()),
        ["remote_id"]
    );
    // A row not read as cancelled keeps its fingerprint: an effective or
    // optional voucher keeps its entries.
    let effective = rows
        .iter()
        .find(|row| row.cancelled == Some(false) && !row.entries.is_empty())
        .unwrap()
        .clone();
    for (cancelled, optional) in [(Some(false), Some(false)), (Some(false), Some(true))] {
        let mut first = effective.clone();
        first.cancelled = cancelled;
        first.optional = optional;
        let second = another_voucher(&first, "2");
        assert_eq!(
            kinds(test_duplicates(&[first, second]).unwrap()),
            ["accounting_fingerprint"],
            "{cancelled:?} {optional:?}"
        );
    }
    // The cancel flag, not an empty entry list, leaves a row out: two rows not
    // read as cancelled pair on no entries, without the cancel beside them.
    let captured_cancel = rows.iter().find(|row| row.cancelled == Some(true)).unwrap();
    let mut empty = another_voucher(captured_cancel, "4");
    empty.cancelled = Some(false);
    let empty_twin = another_voucher(&empty, "5");
    let found =
        test_duplicates(&[captured_cancel.clone(), empty.clone(), empty_twin.clone()]).unwrap();
    let paired = found
        .iter()
        .filter(|item| item["kind"] == "accounting_fingerprint")
        .collect::<Vec<_>>();
    assert_eq!(paired.len(), 1, "{found:?}");
    assert_eq!(
        paired[0]["voucher_ids"],
        serde_json::json!(["guid:derived-guid-4", "guid:derived-guid-5"]),
        "{found:?}"
    );
    // Only a cancel whose entries the read dropped is left out: entry-dropping
    // is measured for Journals only, so a cancelled row that comes back with
    // its entries keeps its fingerprint.
    let mut with_entries = effective;
    with_entries.cancelled = Some(true);
    let twin = another_voucher(&with_entries, "6");
    assert_eq!(
        kinds(test_duplicates(&[with_entries, twin]).unwrap()),
        ["accounting_fingerprint"]
    );
}

#[test]
fn cancelled_vouchers_pair_neither_in_the_batch_nor_in_the_window() {
    // bridge#767's two cases: two cancelled vouchers of the batch, and two
    // cancelled vouchers of the window that are not the batch's.
    let rows = d3_cancelled_rows();
    let cancelled = rows
        .iter()
        .find(|row| row.cancelled == Some(true))
        .unwrap()
        .clone();
    let effective = rows
        .iter()
        .find(|row| row.cancelled == Some(false) && !row.entries.is_empty())
        .unwrap()
        .clone();
    let pair = vec![cancelled.clone(), another_voucher(&cancelled, "3")];
    let identities = pair
        .iter()
        .map(observed_voucher_identity)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let fingerprints = pair.iter().map(observed_fingerprint).collect::<Vec<_>>();
    assert_eq!(fingerprints[0], fingerprints[1]);
    let batch_fingerprint = observed_fingerprint(&effective);
    let expected = BTreeMap::from([(&batch_fingerprint, 1)]);
    for (tags, expected_tags) in [
        (
            vec![Some("batch-1"), Some("batch-2")],
            BTreeSet::from(["batch-1", "batch-2"]),
        ),
        (vec![None, None], BTreeSet::new()),
    ] {
        let (batch, unrelated) = batch_duplicate_sets(
            &pair,
            &identities,
            &fingerprints,
            &expected,
            &tags,
            &expected_tags,
            &BTreeSet::new(),
        );
        assert!(batch.is_empty(), "{tags:?}: {batch:?}");
        assert!(unrelated.is_empty(), "{tags:?}: {unrelated:?}");
    }
}

/// Bridge's own fingerprint of voucher 353, the hand re-entry in the L1
/// capture, equals the fingerprint L1A-050 was built with (#806): the field
/// comparison in `L1_REENTRY_CAPTURE_PROVENANCE.md`, now made by the function
/// verification matches with. The cancelled 352 carries no entries, so its
/// fingerprint does not.
#[test]
fn the_hand_re_entry_carries_the_cancelled_vouchers_fingerprint() {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-import-verification.utf16le.xml"
    );
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let canonical = |mut row: ReadVoucher| {
        for entry in &mut row.entries {
            entry.amount = canonical_verification_amount(&entry.amount).unwrap();
        }
        row
    };
    let rows = parse_import_vouchers(&xml, "17a10910-773c-42c6-bd66-7bba9a392536")
        .unwrap()
        .rows;
    let row_at = |alter_id: u64| {
        canonical(
            rows.iter()
                .find(|row| row.alter_id == Some(alter_id))
                .expect("the captured row")
                .clone(),
        )
    };
    let journal = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/l1-reentry-journal.jsonl"
    );
    let line: ImportLedgerLine =
        serde_json::from_str(journal.lines().next().expect("the built batch")).unwrap();
    let mut built = line
        .vouchers
        .iter()
        .find(|voucher| voucher.bridge_txn_id == "L1A-050")
        .expect("L1A-050")
        .clone();
    for entry in &mut built.entries {
        entry.amount = canonical_verification_amount(&entry.amount).unwrap();
    }
    let expected = expected_fingerprint(&built);
    assert_eq!(observed_fingerprint(&row_at(1790)), expected);
    assert_ne!(observed_fingerprint(&row_at(1789)), expected);
}
