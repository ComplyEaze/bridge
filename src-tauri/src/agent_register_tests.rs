use bridge_tally_protocol::native_outstandings::parse_native_group_snapshot;
use bridge_tally_protocol::parse_native_party_ledger_master_records_with_evidence;

use super::*;

/// The purchase register's classification, which most tests below read; the sales register's
/// tests name `RegisterKind::Sales` and call `super::classify_register` directly.
fn classify_register(index: &MasterIndex, rows: &[Value]) -> Result<RegisterPage, String> {
    super::classify_register(RegisterKind::Purchase, index, rows)
}

/// The disposable GST lab book the captures below were read from.
const COMPANY_GUID: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// Bytes TallyPrime 7.1 Silver sent for Bridge's own compliance master request, group
/// request and class-shaped voucher window on the lab book (sidecars name the binary and
/// the times). Nothing here is hand-written: the ledgers and vouchers were created in the
/// lab book and read back, and the tests read what came back.
fn captured_masters() -> Vec<PartyLedgerMasterRecord> {
    let xml = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-ledger-masters.utf16le.xml"
    ));
    parse_native_party_ledger_master_records_with_evidence(&xml, COMPANY_GUID)
        .expect("the live master capture parses")
        .records
        .into_iter()
        .map(|row| row.record)
        .collect()
}

fn captured_groups() -> GroupIndex {
    let xml = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-groups.utf16le.xml"
    ));
    GroupIndex::build(
        parse_native_group_snapshot(&xml, COMPANY_GUID).expect("the live group capture parses"),
    )
}

fn captured_rows() -> Vec<Value> {
    let xml = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-vouchers-class.utf16le.xml"
    ));
    parse_agent_rows(&xml, COMPANY_GUID).expect("the live voucher window parses")
}

fn captured_index() -> MasterIndex {
    MasterIndex::build(captured_masters().iter(), &captured_groups(), Vec::new())
        .expect("masters index")
}

fn row_on<'a>(rows: &'a [Value], date: &str) -> &'a Value {
    rows.iter()
        .find(|row| row["date"] == date)
        .unwrap_or_else(|| panic!("no register row dated {date}"))
}

fn ledgers(entries: &Value) -> Vec<String> {
    entries
        .as_array()
        .expect("a list")
        .iter()
        .map(|entry| entry["ledger"].as_str().unwrap().to_string())
        .collect()
}

#[test]
fn the_captured_window_splits_into_the_register_and_the_other_types_touching_duties_taxes() {
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    assert_eq!(page.vouchers_observed, 8);
    // Four Purchase vouchers and the Debit Note are the register.
    let classes: Vec<_> = page
        .rows
        .iter()
        .map(|row| row["voucher_class"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        classes,
        ["Purchase", "Purchase", "Purchase", "Purchase", "Debit Note"]
    );
    // The Sales, Journal and Payment vouchers touch Duties & Taxes too: listed, not dropped.
    let other: Vec<_> = page
        .other_voucher_types
        .iter()
        .map(|row| row["voucher_class"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(other, ["Sales", "Journal", "Payment"]);
    assert!(page.unclassified_voucher_type.is_empty());
    assert!(page.vouchers_with_unplaced_ledgers.is_empty());
}

#[test]
fn an_intra_state_purchase_carries_tax_per_entry_on_recognised_heads() {
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250903");
    assert_eq!(row["status"], "complete");
    assert_eq!(row["party"], "SYN Supplier Intra (M2)");
    assert!(
        row.get("reference").is_none(),
        "absent, never an empty string"
    );
    assert_eq!(row["tax_in_books"][0]["ledger"], "Input CGST");
    assert_eq!(row["tax_in_books"][0]["head"], "cgst");
    assert_eq!(row["tax_in_books"][0]["amount"], "-900.00");
    assert_eq!(row["tax_in_books"][1]["ledger"], "Input SGST");
    assert_eq!(row["tax_in_books"][1]["head"], "state_tax");
    assert_eq!(row["tax_in_books"][1]["amount"], "-900.00");
    assert_eq!(ledgers(&row["taxable_entries"]), ["Purchase - Goods"]);
    assert_eq!(row["taxable_entries"][0]["amount"], "-10000.00");
    assert_eq!(ledgers(&row["party_entries"]), ["SYN Supplier Intra (M2)"]);
    assert!(row["other_entries"].as_array().unwrap().is_empty());
    assert!(row["duties_taxes_entries_without_gst_head"]
        .as_array()
        .unwrap()
        .is_empty());
}

#[test]
fn an_inter_state_purchase_carries_its_reference_and_one_igst_entry() {
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250905");
    assert_eq!(row["reference"], "SYN-INV-002");
    assert_eq!(row["tax_in_books"].as_array().unwrap().len(), 1);
    assert_eq!(row["tax_in_books"][0]["head"], "igst");
    assert_eq!(row["tax_in_books"][0]["amount"], "-3600.00");
}

#[test]
fn an_entry_on_a_duties_ledger_without_a_head_is_listed_and_never_assigned_one() {
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250911");
    assert_eq!(row["status"], "has_entries_without_gst_head");
    assert_eq!(
        ledgers(&row["duties_taxes_entries_without_gst_head"]),
        ["TDS Payable (M2)"]
    );
    assert_eq!(
        row["duties_taxes_entries_without_gst_head"][0]["observation"],
        "not_tax_ledger"
    );
    assert_eq!(
        row["duties_taxes_entries_without_gst_head"][0]["amount"],
        "10.00"
    );
    // The two headed ledgers of the same voucher are still tax.
    assert_eq!(ledgers(&row["tax_in_books"]), ["Input CGST", "Input SGST"]);
}

#[test]
fn an_sgst_utgst_ledger_is_tax_under_its_own_head_and_never_folded_into_state_tax() {
    // The ledger `Input SGST 9% (M2)` was created with the head `SGST/UTGST`, which the
    // classifier recognises as its own head. It is tax, under that head, with the spelling
    // Tally returned; it is not listed as unrecognised and it is not `state_tax`.
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250909");
    assert_eq!(row["status"], "complete");
    assert!(row["duties_taxes_entries_with_unrecognised_head"]
        .as_array()
        .unwrap()
        .is_empty());
    let entry = row["tax_in_books"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["ledger"] == "Input SGST 9% (M2)")
        .expect("the SGST/UTGST ledger is in tax_in_books");
    assert_eq!(entry["head"], "sgst_utgst");
    assert_eq!(entry["raw_head"], "SGST/UTGST");
    assert_eq!(entry["amount"], "-450.00");
    assert_eq!(row["tax_in_books"][0]["ledger"], "Input CGST");
    // The ledger with the other state spelling keeps its own head.
    let other = row_on(&page.rows, "20250903");
    assert_eq!(other["tax_in_books"][1]["head"], "state_tax");
}

#[test]
fn a_debit_note_keeps_the_amounts_and_signs_the_books_state() {
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250916");
    assert_eq!(row["voucher_class"], "Debit Note");
    // The Debit Note reverses the purchase: its tax entries are positive as the books state
    // them. The register never re-signs from the deemed-positive flag.
    assert_eq!(row["tax_in_books"][0]["amount"], "90.00");
    assert_eq!(row["tax_in_books"][1]["amount"], "90.00");
    assert_eq!(row["taxable_entries"][0]["amount"], "1000.00");
}

#[test]
fn the_other_voucher_types_name_the_duties_taxes_ledgers_they_touch() {
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    let journal = page
        .other_voucher_types
        .iter()
        .find(|row| row["voucher_class"] == "Journal")
        .unwrap();
    let touched = ledgers(&json!(journal["duties_taxes_ledgers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| json!({"ledger": name}))
        .collect::<Vec<_>>()));
    assert_eq!(
        touched,
        ["Input CGST", "Input SGST", "Output CGST", "Output SGST"]
    );
}

#[test]
fn a_voucher_entry_naming_a_ledger_the_masters_do_not_list_refuses() {
    let mut rows = captured_rows();
    rows[0]["amounts"][1]["ledger"] = json!("A ledger created after the masters were read");
    assert_eq!(
        classify_register(&captured_index(), &rows),
        Err("ledger_snapshot_drifted".to_string())
    );
}

#[test]
fn a_ledger_whose_head_and_tax_type_disagree_makes_the_voucher_a_head_conflict() {
    let mut masters = captured_masters();
    let input_cgst = masters
        .iter_mut()
        .find(|record| record.ledger.name == "Input CGST")
        .unwrap();
    input_cgst.fields.gst_duty_head = GstDutyHeadObservation::Contradictory {
        tax_type: "Others".to_string(),
        raw: "CGST".to_string(),
    };
    let index = MasterIndex::build(masters.iter(), &captured_groups(), Vec::new()).unwrap();
    let page = classify_register(&index, &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250903");
    assert_eq!(row["status"], "head_conflict");
    let conflicted = &row["duties_taxes_entries_with_unrecognised_head"][0];
    assert_eq!(conflicted["ledger"], "Input CGST");
    assert_eq!(conflicted["observation"], "contradictory");
    assert!(
        !ledgers(&row["tax_in_books"]).contains(&"Input CGST".to_string()),
        "a contradiction is not released as tax"
    );
}

#[test]
fn a_ledger_whose_group_cannot_be_resolved_is_named_not_classified() {
    // The group collection without Duties & Taxes: the tax ledgers lose their place.
    let xml = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-groups.utf16le.xml"
    ));
    let groups: Vec<_> = parse_native_group_snapshot(&xml, COMPANY_GUID)
        .unwrap()
        .into_iter()
        .filter(|group| group.name != "Duties & Taxes")
        .collect();
    let index = MasterIndex::build(
        captured_masters().iter(),
        &GroupIndex::build(groups),
        Vec::new(),
    )
    .unwrap();
    let page = classify_register(&index, &captured_rows()).unwrap();
    // No ledger is placed under Duties & Taxes, so nothing is in the register, and the
    // vouchers that touch the unplaced ledgers are named rather than silently absent.
    assert!(page.rows.is_empty());
    assert_eq!(page.vouchers_with_unplaced_ledgers.len(), 8);
    assert_eq!(
        page.vouchers_with_unplaced_ledgers[0]["unplaced_ledgers"][0]["ancestry_gap"],
        "group_absent"
    );
}

#[test]
fn a_voucher_without_a_resolved_class_is_listed_as_unclassified() {
    let mut rows = captured_rows();
    rows[0]["voucher_class"] = Value::Null;
    let page = classify_register(&captured_index(), &rows).unwrap();
    assert_eq!(page.unclassified_voucher_type.len(), 1);
    assert_eq!(page.unclassified_voucher_type[0]["date"], "20250903");
    assert_eq!(page.rows.len(), 4);
}

#[test]
fn cancelled_optional_and_post_dated_vouchers_are_flagged_and_still_returned() {
    let mut rows = captured_rows();
    rows[0]["cancelled"] = json!(true);
    rows[1]["optional"] = json!(true);
    rows[2]["post_dated"] = json!(true);
    let page = classify_register(&captured_index(), &rows).unwrap();
    assert_eq!(row_on(&page.rows, "20250903")["cancelled"], true);
    assert_eq!(row_on(&page.rows, "20250905")["optional"], true);
    assert_eq!(row_on(&page.rows, "20250909")["post_dated"], true);
    assert_eq!(page.rows.len(), 5);
}

#[test]
fn an_entry_off_the_tax_and_purchase_ledgers_lands_in_other_entries() {
    let mut rows = captured_rows();
    rows[0]["amounts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"ledger": "Cash", "amount": "0.40", "is_deemed_positive": "No", "bill_allocations": []}));
    let page = classify_register(&captured_index(), &rows).unwrap();
    let row = row_on(&page.rows, "20250903");
    assert_eq!(row["status"], "has_other_entries");
    assert_eq!(ledgers(&row["other_entries"]), ["Cash"]);
}

#[test]
fn two_listings_of_the_same_masters_are_equal_and_a_changed_head_is_not() {
    let first = captured_index();
    assert_eq!(first, captured_index());
    assert_eq!(first.len(), 41);

    let mut masters = captured_masters();
    masters
        .iter_mut()
        .find(|record| record.ledger.name == "Input IGST")
        .unwrap()
        .fields
        .gst_duty_head = GstDutyHeadObservation::Absent;
    let changed = MasterIndex::build(masters.iter(), &captured_groups(), Vec::new()).unwrap();
    assert_ne!(first, changed, "a re-headed ledger is drift");

    let mut reparented = captured_masters();
    reparented
        .iter_mut()
        .find(|record| record.ledger.name == "Input IGST")
        .unwrap()
        .ledger
        .parent = bridge_tally_protocol::PartyLedgerMasterFieldObservation::Returned(
        "Indirect Expenses".to_string(),
    );
    let moved = MasterIndex::build(reparented.iter(), &captured_groups(), Vec::new()).unwrap();
    assert_ne!(first, moved, "a re-parented ledger is drift");
}

#[test]
fn a_repeated_ledger_name_refuses_as_drift() {
    let mut masters = captured_masters();
    masters.push(masters[0].clone());
    assert_eq!(
        MasterIndex::build(masters.iter(), &captured_groups(), Vec::new()),
        Err("ledger_snapshot_drifted".to_string())
    );
}

#[test]
fn a_voucher_touching_a_ledger_the_compliance_read_set_aside_refuses_with_its_own_code() {
    // The compliance read leaves out ledgers kept in another currency; a voucher that names
    // one cannot be classified and must not fall into "drift" or "other".
    let masters: Vec<_> = captured_masters()
        .into_iter()
        .filter(|record| record.ledger.name != "Input CGST")
        .collect();
    let index = MasterIndex::build(
        masters.iter(),
        &captured_groups(),
        vec!["Input CGST".to_string()],
    )
    .unwrap();
    assert_eq!(
        classify_register(&index, &captured_rows()),
        Err("register_ledger_currency_excluded".to_string())
    );
    // Without the exclusion the same absence is drift.
    let index = MasterIndex::build(masters.iter(), &captured_groups(), Vec::new()).unwrap();
    assert_eq!(
        classify_register(&index, &captured_rows()),
        Err("ledger_snapshot_drifted".to_string())
    );
}

#[test]
fn a_side_list_carries_its_exact_total_and_at_most_the_cap() {
    let items: Vec<Value> = (0..MAX_LISTED_SIDE_ITEMS + 50)
        .map(|number| json!({"n": number}))
        .collect();
    let bounded = bounded_list(&items);
    assert_eq!(bounded["total"], MAX_LISTED_SIDE_ITEMS + 50);
    assert_eq!(
        bounded["listed"].as_array().unwrap().len(),
        MAX_LISTED_SIDE_ITEMS
    );
    assert_eq!(bounded["listed_truncated"], true);
    assert_eq!(bounded["listed"][0]["n"], 0);
    let small = bounded_list(&items[..3]);
    assert_eq!(small["total"], 3);
    assert_eq!(small["listed_truncated"], false);
}

#[test]
fn every_entry_ledger_is_marked_so_it_is_masked_like_vouchers_masks_it() {
    let row = mark_register_row(json!({
        "party": "Customer One",
        "tax_in_books": [{"ledger": "Input CGST"}],
        "taxable_entries": [{"ledger": "Supplier Two"}],
        "party_entries": [{"ledger": "Customer One"}],
        "other_entries": [{"ledger": "Supplier Two"}],
    }));
    let redacted = redact_value(row, Redaction::MaskParties);
    let text = redacted.to_string();
    for name in ["Customer One", "Supplier Two", "Input CGST"] {
        assert!(!text.contains(name), "{name} survived: {text}");
    }
    assert_eq!(
        redacted["tax_in_books"][0]["ledger"],
        json!(mask("Input CGST"))
    );
}

#[test]
fn a_register_voucher_with_an_unplaced_purchase_ledger_is_returned_flagged_not_complete() {
    // Without Purchase Accounts in the group collection the purchase ledgers cannot be placed.
    // The voucher still touches Duties & Taxes, so it stays in the register, says so, and lists
    // the entry with the reason it could not be placed.
    let xml = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-groups.utf16le.xml"
    ));
    let groups: Vec<_> = parse_native_group_snapshot(&xml, COMPANY_GUID)
        .unwrap()
        .into_iter()
        .filter(|group| group.name != "Purchase Accounts")
        .collect();
    let index = MasterIndex::build(
        captured_masters().iter(),
        &GroupIndex::build(groups),
        Vec::new(),
    )
    .unwrap();
    let page = classify_register(&index, &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250903");
    assert_eq!(row["status"], "has_unresolved_group");
    assert_eq!(
        ledgers(&row["entries_on_ledgers_with_unresolved_group"]),
        ["Purchase - Goods"]
    );
    assert_eq!(
        row["entries_on_ledgers_with_unresolved_group"][0]["ancestry_gap"],
        "group_absent"
    );
    assert!(row["taxable_entries"].as_array().unwrap().is_empty());
    assert_eq!(ledgers(&row["tax_in_books"]), ["Input CGST", "Input SGST"]);
}

#[test]
fn a_side_list_of_exactly_the_cap_is_not_truncated() {
    let items: Vec<Value> = (0..MAX_LISTED_SIDE_ITEMS)
        .map(|n| json!({"n": n}))
        .collect();
    let bounded = bounded_list(&items);
    assert_eq!(bounded["total"], MAX_LISTED_SIDE_ITEMS);
    assert_eq!(
        bounded["listed"].as_array().unwrap().len(),
        MAX_LISTED_SIDE_ITEMS
    );
    assert_eq!(bounded["listed_truncated"], false);
}

fn masters_read(marks: CompanyMarks, index: MasterIndex) -> RegisterMasters {
    RegisterMasters {
        index,
        marks,
        evidence: Evidence {
            request_sha256: String::new(),
            response_sha256: String::new(),
            bytes: 0,
            state: "complete",
            read_at: None,
            duration_ms: None,
            reason_code: None,
        },
    }
}

const PINNED: CompanyMarks = CompanyMarks {
    vouchers: 54,
    masters: 120,
};

#[test]
fn a_window_whose_marks_are_unchanged_is_stable() {
    assert_eq!(window_drift(PINNED, PINNED), None);
}

#[test]
fn a_window_is_refused_when_either_mark_moved_after_it() {
    let moved_voucher = CompanyMarks {
        vouchers: 55,
        ..PINNED
    };
    let moved_master = CompanyMarks {
        masters: 121,
        ..PINNED
    };
    for closing in [moved_voucher, moved_master] {
        assert_eq!(
            window_drift(PINNED, closing),
            Some("voucher_window_changed_during_read"),
            "{closing:?}"
        );
    }
}

#[test]
fn masters_read_again_must_match_in_classification_and_marks() {
    let first = masters_read(PINNED, captured_index());
    assert_eq!(
        masters_drifted(&first, &masters_read(PINNED, captured_index())),
        None
    );
    // A master mark that moved, or a classification input that changed under the same marks,
    // is the masters drifting.
    assert_eq!(
        masters_drifted(
            &first,
            &masters_read(
                CompanyMarks {
                    masters: 121,
                    ..PINNED
                },
                captured_index()
            )
        ),
        Some("ledger_snapshot_drifted")
    );
    let mut masters = captured_masters();
    masters
        .iter_mut()
        .find(|record| record.ledger.name == "Input CGST")
        .unwrap()
        .fields
        .gst_duty_head = GstDutyHeadObservation::Absent;
    let rehead = MasterIndex::build(masters.iter(), &captured_groups(), Vec::new()).unwrap();
    assert_eq!(
        masters_drifted(&first, &masters_read(PINNED, rehead)),
        Some("ledger_snapshot_drifted")
    );
    // A voucher posted after the closing marks were read is the window's drift, not the masters'.
    assert_eq!(
        masters_drifted(
            &first,
            &masters_read(
                CompanyMarks {
                    vouchers: 55,
                    ..PINNED
                },
                captured_index()
            )
        ),
        Some("voucher_window_changed_during_read")
    );
}

#[test]
fn a_page_says_where_the_next_one_starts() {
    let rows: Vec<Value> = (0..5).map(|n| json!({"n": n})).collect();
    let (page, truncated, next) = paginate(rows.clone(), 0, 2);
    assert_eq!((page.len(), truncated, next), (2, true, Some(2)));
    let (page, truncated, next) = paginate(rows.clone(), 4, 2);
    assert_eq!((page.len(), truncated, next), (1, false, None));
    let (page, truncated, next) = paginate(rows.clone(), 5, 2);
    assert_eq!((page.len(), truncated, next), (0, false, None));
    let (page, truncated, next) = paginate(rows, 2, 3);
    assert_eq!(page[0]["n"], 2);
    assert_eq!((page.len(), truncated, next), (3, false, None));
}

fn result_for(
    rows: Vec<Value>,
    index: &MasterIndex,
    redaction: Redaction,
) -> Result<RegisterResult, String> {
    register_result(
        RegisterKind::Purchase,
        index,
        rows,
        ("20250901", "20250930"),
        (0, 500),
        redaction,
    )
}

#[test]
fn a_row_dated_outside_the_window_is_refused_not_returned() {
    let mut rows = captured_rows();
    rows[1]["date"] = json!("20251015");
    assert_eq!(
        result_for(rows, &captured_index(), Redaction::None).map(|_| ()),
        Err("window_not_honoured".to_string())
    );
}

#[test]
fn the_result_carries_items_and_every_list_the_response_promises() {
    let result = result_for(captured_rows(), &captured_index(), Redaction::None).unwrap();
    assert!(!result.truncated);
    let body = &result.result;
    assert_eq!(body["items"].as_array().unwrap().len(), 5);
    assert_eq!(body["total"], 5);
    assert_eq!(body["vouchers_observed"], 8);
    assert_eq!(body["ledger_masters_observed"], 41);
    assert_eq!(
        body["other_voucher_types_touching_duties_taxes"]["total"],
        3
    );
    assert_eq!(body["unclassified_voucher_type"]["total"], 0);
    assert_eq!(body["vouchers_with_unplaced_ledgers"]["total"], 0);
    assert_eq!(
        body["purchase_vouchers_without_duties_taxes_entry"]["total"],
        0
    );
    assert_eq!(body["items"][0]["has_taxable_entry"], true);
}

#[test]
fn a_purchase_with_no_duties_taxes_entry_is_counted_not_dropped() {
    let mut rows = captured_rows();
    let entries = rows[0]["amounts"].as_array_mut().unwrap();
    entries.retain(|entry| {
        let name = entry["ledger"].as_str().unwrap();
        name != "Input CGST" && name != "Input SGST"
    });
    let result = result_for(rows, &captured_index(), Redaction::None).unwrap();
    let body = &result.result;
    assert_eq!(body["items"].as_array().unwrap().len(), 4);
    assert_eq!(
        body["purchase_vouchers_without_duties_taxes_entry"]["total"],
        1
    );
    assert_eq!(
        body["purchase_vouchers_without_duties_taxes_entry"]["listed"][0]["date"],
        "20250903"
    );
}

#[test]
fn an_entry_on_a_ledger_with_no_head_and_a_gst_tax_type_says_so() {
    // `absent` is a ledger whose TAXTYPE is GST (or not reported) with no head: unlike a
    // not_tax_ledger, it may be a GST ledger whose head is missing, and the row carries the
    // tax type so a reader can tell.
    let mut masters = captured_masters();
    masters
        .iter_mut()
        .find(|record| record.ledger.name == "Input CGST")
        .unwrap()
        .fields
        .gst_duty_head = GstDutyHeadObservation::Absent;
    let index = MasterIndex::build(masters.iter(), &captured_groups(), Vec::new()).unwrap();
    let page = classify_register(&index, &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250903");
    let entry = &row["duties_taxes_entries_without_gst_head"][0];
    assert_eq!(entry["ledger"], "Input CGST");
    assert_eq!(entry["observation"], "absent");
    assert_eq!(entry["tax_type"], "GST");
    assert_eq!(row["status"], "has_entries_without_gst_head");
}

#[test]
fn no_party_name_survives_redaction_in_any_list_of_the_real_response() {
    // With Sundry Creditors missing from the group collection the suppliers' own ledgers
    // cannot be placed, so their names sit in `entries_on_ledgers_with_unresolved_group`; the
    // vouchers with no Duties & Taxes ledger go to the side lists. Masked, none of the
    // supplier names may remain anywhere.
    let xml = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-groups.utf16le.xml"
    ));
    let groups: Vec<_> = parse_native_group_snapshot(&xml, COMPANY_GUID)
        .unwrap()
        .into_iter()
        .filter(|group| group.name != "Sundry Creditors")
        .collect();
    let index = MasterIndex::build(
        captured_masters().iter(),
        &GroupIndex::build(groups),
        Vec::new(),
    )
    .unwrap();
    let mut rows = captured_rows();
    // A voucher whose only tax-free ledgers are unplaced lands in the unplaced side list.
    rows[0]["amounts"].as_array_mut().unwrap().retain(|entry| {
        let name = entry["ledger"].as_str().unwrap();
        name != "Input CGST" && name != "Input SGST"
    });
    rows[0]["voucher_class"] = json!("Sales");
    let plain = result_for(rows.clone(), &index, Redaction::None).unwrap();
    assert!(
        plain.result.to_string().contains("SYN Supplier Intra (M2)"),
        "the unmasked response does name the supplier, so the assertion below can fail"
    );
    assert!(
        plain.result.to_string().contains("SYN Supplier Inter (M2)"),
        "and the second supplier, so its masking is tested too"
    );
    let masked = result_for(rows, &index, Redaction::MaskParties).unwrap();
    let text = masked.result.to_string();
    assert!(!text.contains("SYN Supplier Intra (M2)"), "{text}");
    assert!(!text.contains("SYN Supplier Inter (M2)"), "{text}");
    assert!(
        !text.contains("Input CGST") && !text.contains("Purchase - Goods"),
        "a ledger is masked wherever it appears, tax and purchase ledgers included: {text}"
    );
}

#[test]
fn a_row_without_a_purchase_ledger_entry_says_it_has_no_taxable_entry() {
    // An item invoice may hold its purchase ledger in an inventory allocation the parser does
    // not read, so the row says so instead of reporting a complete voucher with nothing taxable.
    let mut rows = captured_rows();
    rows[0]["amounts"]
        .as_array_mut()
        .unwrap()
        .retain(|entry| entry["ledger"] != "Purchase - Goods");
    let page = classify_register(&captured_index(), &rows).unwrap();
    let row = row_on(&page.rows, "20250903");
    assert_eq!(row["has_taxable_entry"], false);
    assert!(row["taxable_entries"].as_array().unwrap().is_empty());
    assert_eq!(row_on(&page.rows, "20250905")["has_taxable_entry"], true);
}

#[test]
fn a_cancelled_purchase_with_no_tax_entry_is_listed_with_its_flag() {
    let mut rows = captured_rows();
    rows[0]["amounts"]
        .as_array_mut()
        .unwrap()
        .retain(|entry| entry["ledger"] != "Input CGST" && entry["ledger"] != "Input SGST");
    rows[0]["cancelled"] = json!(true);
    let result = result_for(rows, &captured_index(), Redaction::None).unwrap();
    let listed = &result.result["purchase_vouchers_without_duties_taxes_entry"]["listed"][0];
    assert_eq!(listed["date"], "20250903");
    assert_eq!(listed["cancelled"], true);
    assert_eq!(listed["optional"], false);
}

#[test]
fn a_flag_tally_did_not_report_is_absent_from_a_listed_voucher_not_null() {
    let mut rows = captured_rows();
    rows[0].as_object_mut().unwrap().remove("post_dated");
    rows[0]["amounts"]
        .as_array_mut()
        .unwrap()
        .retain(|entry| entry["ledger"] != "Input CGST" && entry["ledger"] != "Input SGST");
    let result = result_for(rows, &captured_index(), Redaction::None).unwrap();
    let listed = &result.result["purchase_vouchers_without_duties_taxes_entry"]["listed"][0];
    assert!(listed.get("post_dated").is_none(), "{listed}");
}

#[test]
fn a_ledger_is_masked_the_same_way_by_the_register_and_by_vouchers() {
    let result = result_for(captured_rows(), &captured_index(), Redaction::MaskParties).unwrap();
    let first = &result.result["items"][0];
    let by_vouchers = redact_value(
        mark_voucher_party_names(json!({"amounts": [{"ledger": "Input CGST"}]})),
        Redaction::MaskParties,
    );
    assert_eq!(
        first["tax_in_books"][0]["ledger"],
        by_vouchers["amounts"][0]["ledger"]
    );
    assert_eq!(
        first["taxable_entries"][0]["ledger"],
        json!(mask("Purchase - Goods"))
    );
    // No ledger that appears in any row survives anywhere in the masked response.
    let text = result.result.to_string();
    for row in captured_rows() {
        for entry in row["amounts"].as_array().unwrap() {
            let name = entry["ledger"].as_str().unwrap();
            if name.chars().count() > 4 {
                assert!(!text.contains(name), "{name} survived: {text}");
            }
        }
    }
}

#[test]
fn the_other_types_list_masks_the_duties_taxes_ledgers_it_names() {
    let result = result_for(captured_rows(), &captured_index(), Redaction::MaskParties).unwrap();
    let listed = &result.result["other_voucher_types_touching_duties_taxes"]["listed"][0];
    assert_eq!(
        listed["duties_taxes_ledgers"][0],
        json!(mask("Output CGST"))
    );
}

#[test]
fn a_debit_note_to_a_customer_is_told_from_a_purchase_return_by_its_party_group() {
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    assert_eq!(
        row_on(&page.rows, "20250916")["party_group"],
        "Sundry Creditors"
    );
    let mut rows = captured_rows();
    let debit_note = rows
        .iter_mut()
        .find(|row| row["voucher_class"] == "Debit Note")
        .unwrap();
    debit_note["party"] = json!("Bengaluru Distributors");
    for entry in debit_note["amounts"].as_array_mut().unwrap() {
        if entry["ledger"] == "SYN Supplier Intra (M2)" {
            entry["ledger"] = json!("Bengaluru Distributors");
        }
    }
    let page = classify_register(&captured_index(), &rows).unwrap();
    assert_eq!(
        row_on(&page.rows, "20250916")["party_group"],
        "Sundry Debtors"
    );
    // The register does not guess: the tax heads and status are the same either way.
    assert_eq!(row_on(&page.rows, "20250916")["status"], "complete");
    // A party the masters do not list carries no group, not a guess.
    let mut rows = captured_rows();
    rows[0]["party"] = json!("A party created after the masters were read");
    let page = classify_register(&captured_index(), &rows).unwrap();
    assert!(row_on(&page.rows, "20250903").get("party_group").is_none());
}

#[test]
fn an_unrecognised_head_sits_only_in_the_unrecognised_list() {
    // Exactly one outcome: a head the classifier does not recognise is listed with its raw
    // spelling and never appears as tax, whatever else changes about the vocabulary.
    let mut masters = captured_masters();
    masters
        .iter_mut()
        .find(|record| record.ledger.name == "Input CGST")
        .unwrap()
        .fields
        .gst_duty_head = GstDutyHeadObservation::Unrecognized {
        raw: "Central Tax".to_string(),
    };
    let index = MasterIndex::build(masters.iter(), &captured_groups(), Vec::new()).unwrap();
    let page = classify_register(&index, &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250903");
    assert_eq!(row["status"], "has_unrecognised_head");
    assert_eq!(
        ledgers(&row["duties_taxes_entries_with_unrecognised_head"]),
        ["Input CGST"]
    );
    assert_eq!(
        row["duties_taxes_entries_with_unrecognised_head"][0]["raw_head"],
        "Central Tax"
    );
    assert_eq!(ledgers(&row["tax_in_books"]), ["Input SGST"]);
}

// The sales register. It reads and classifies exactly as the purchase register does (the tests
// above run through `RegisterKind::Purchase`); what differs is the register's classes and the
// group the taxable ledgers sit under. The lab book of the captures above holds one Sales
// voucher, so it is the sales register's only capture with the ledger masters beside it.

fn sales_page() -> RegisterPage {
    super::classify_register(RegisterKind::Sales, &captured_index(), &captured_rows()).unwrap()
}

#[test]
fn the_captured_sales_voucher_is_the_sales_register_and_every_purchase_type_is_listed_apart() {
    let page = sales_page();
    assert_eq!(page.vouchers_observed, 8);
    assert_eq!(page.rows.len(), 1);
    let other: Vec<_> = page
        .other_voucher_types
        .iter()
        .map(|row| row["voucher_class"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        other,
        [
            "Purchase",
            "Purchase",
            "Purchase",
            "Purchase",
            "Debit Note",
            "Journal",
            "Payment"
        ]
    );
    assert!(page.register_class_without_duties_taxes_entry.is_empty());
    assert!(page.unclassified_voucher_type.is_empty());
    assert!(page.vouchers_with_unplaced_ledgers.is_empty());
}

#[test]
fn a_sales_row_takes_tax_from_the_duty_head_and_the_sales_ledger_as_its_taxable_entry() {
    let page = sales_page();
    let row = row_on(&page.rows, "20250918");
    assert_eq!(row["voucher_class"], "Sales");
    assert_eq!(row["status"], "complete");
    assert_eq!(row["party"], "Counter Sales - Unregistered");
    assert_eq!(row["party_group"], "Sundry Debtors");
    assert_eq!(
        ledgers(&row["tax_in_books"]),
        ["Output CGST", "Output SGST"]
    );
    assert_eq!(row["tax_in_books"][0]["head"], "cgst");
    assert_eq!(row["tax_in_books"][1]["head"], "state_tax");
    // Amounts are as the books state them: a sale's tax is a credit, which is positive.
    assert_eq!(row["tax_in_books"][0]["amount"], "900.00");
    assert_eq!(row["has_taxable_entry"], true);
    assert_eq!(ledgers(&row["taxable_entries"]), ["Sales - Goods"]);
    assert_eq!(row["taxable_entries"][0]["amount"], "10000.00");
    assert_eq!(
        ledgers(&row["party_entries"]),
        ["Counter Sales - Unregistered"]
    );
    assert_eq!(row["party_entries"][0]["amount"], "-11800.00");
    // The captured Sales voucher is an accounting voucher, not an invoice: reported as read.
    assert_eq!(row["is_invoice"], false);
}

#[test]
fn neither_register_lists_the_others_vouchers_as_rows() {
    let purchases = classify_register(&captured_index(), &captured_rows()).unwrap();
    assert!(purchases.rows.iter().all(|row| matches!(
        row["voucher_class"].as_str(),
        Some("Purchase" | "Debit Note")
    )));
    assert!(sales_page()
        .rows
        .iter()
        .all(|row| matches!(row["voucher_class"].as_str(), Some("Sales" | "Credit Note"))));
}

#[test]
fn the_register_classes_are_the_registers_own() {
    assert!(is_register_class(RegisterKind::Sales, "Sales"));
    assert!(is_register_class(RegisterKind::Sales, "Credit Note"));
    assert!(!is_register_class(RegisterKind::Sales, "Purchase"));
    assert!(!is_register_class(RegisterKind::Sales, "Debit Note"));
    assert!(is_register_class(RegisterKind::Purchase, "Purchase"));
    assert!(is_register_class(RegisterKind::Purchase, "Debit Note"));
    assert!(!is_register_class(RegisterKind::Purchase, "Sales"));
    assert!(!is_register_class(RegisterKind::Purchase, "Credit Note"));
}

#[test]
fn the_sales_result_names_its_own_profile_classes_and_list() {
    let result = register_result(
        RegisterKind::Sales,
        &captured_index(),
        captured_rows(),
        ("20250901", "20250930"),
        (0, 500),
        Redaction::None,
    )
    .unwrap();
    let body = &result.result;
    assert_eq!(body["profile"], "agent_sales_register_v1");
    assert_eq!(body["register_classes"], json!(["Sales", "Credit Note"]));
    assert_eq!(body["total"], 1);
    assert_eq!(
        body["sales_vouchers_without_duties_taxes_entry"]["total"],
        0
    );
    assert!(body
        .get("purchase_vouchers_without_duties_taxes_entry")
        .is_none());
    let coverage = body["coverage"].as_str().unwrap();
    assert!(coverage.starts_with("items are the Sales and Credit Note vouchers"));
    // What no capture covers is said in the response, not only in the tool text.
    for named in [
        "measured for sales so far",
        "live runs on two synthetic companies",
        "not shown by any run: an invoice-view Credit Note",
        "its signs reversed as Tally sends them",
        "look for neither alone",
        "only where the row itself shows it",
        "not vouched for",
        "a recognised IGST head",
        "REFERENCEDATE (not returned)",
        "`not_measured_live`",
    ] {
        assert!(coverage.contains(named), "coverage lacks {named}");
    }
    // The purchase register's response is unchanged by this note.
    let purchase = result_for(captured_rows(), &captured_index(), Redaction::None).unwrap();
    assert!(!purchase.result["coverage"]
        .as_str()
        .unwrap()
        .contains("not measured"));
}

#[test]
fn the_sales_register_masks_every_ledger_and_the_party_like_the_purchase_register() {
    let result = register_result(
        RegisterKind::Sales,
        &captured_index(),
        captured_rows(),
        ("20250901", "20250930"),
        (0, 500),
        Redaction::MaskParties,
    )
    .unwrap();
    let text = result.result.to_string();
    for name in [
        "Counter Sales - Unregistered",
        "Sales - Goods",
        "Output CGST",
    ] {
        assert!(!text.contains(name), "{name} leaked");
    }
}

/// The voucher window Bridge's own builder sent for one day and Tally's answer, from a lab
/// company whose ledger masters were not read alongside (see the provenance file).
fn sales_day_rows(name: &str) -> Vec<Value> {
    let bytes: &[u8] = match name {
        "sales_day" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/sales-day/register_window_sales_day_live.utf16le.xml"
        ),
        _ => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/sales-day/register_window_taxed_sales_day_live.utf16le.xml"
        ),
    };
    parse_agent_rows(&utf16(bytes), "1edb9b05-8d35-4c0c-9959-dce655730463")
        .expect("the live voucher window parses")
}

#[test]
fn a_sales_item_invoice_arrives_with_the_party_the_sales_ledger_and_its_tax_ledgers_as_entries() {
    let plain = sales_day_rows("sales_day");
    assert_eq!(plain.len(), 1);
    assert_eq!(plain[0]["voucher_class"], "Sales");
    assert_eq!(plain[0]["is_invoice"], true);
    // The goods line is nested under the sales ledger's entry and is not an entry of its own.
    assert_eq!(
        plain[0]["amounts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| (
                entry["ledger"].as_str().unwrap(),
                entry["amount"].as_str().unwrap()
            ))
            .collect::<Vec<_>>(),
        [("Lab Customer One", "-24.00"), ("Lab Sales", "24.00")]
    );
    let taxed = sales_day_rows("taxed_sales_day");
    assert_eq!(taxed.len(), 1);
    assert_eq!(
        taxed[0]["amounts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| (
                entry["ledger"].as_str().unwrap(),
                entry["amount"].as_str().unwrap()
            ))
            .collect::<Vec<_>>(),
        [
            ("Lab Customer One", "-118.00"),
            ("Lab Sales", "100.00"),
            ("Lab Output CGST", "9.00"),
            ("Lab Output SGST", "9.00"),
        ]
    );
}

#[test]
fn a_sales_voucher_naming_ledgers_the_masters_do_not_list_refuses_instead_of_being_guessed() {
    // The item invoices come from another company than the masters above, and that company's
    // masters were not captured: the register cannot place their ledgers, and says so.
    for name in ["sales_day", "taxed_sales_day"] {
        assert_eq!(
            super::classify_register(
                RegisterKind::Sales,
                &captured_index(),
                &sales_day_rows(name)
            )
            .map(|_| ()),
            Err("ledger_snapshot_drifted".to_string())
        );
    }
}

/// The sales description after its own opening is the purchase description's shared read with
/// these swaps. This is the only place the swaps live, so an edit to the purchase text that
/// the sales text does not follow fails here, not in the server.
fn derived_sales_tail() -> String {
    let (_, shared) = PURCHASE_REGISTER_DESCRIPTION
        .split_once(" It inherits the compliance read")
        .expect("the purchase description has its shared read");
    let swaps = [
        (
            "Return the Purchase and Debit Note vouchers",
            "Return the Sales and Credit Note vouchers",
        ),
        (
            "`taxable_entries` are entries on Purchase Accounts ledgers only (a GST purchase booked to a fixed-asset or expense ledger has `has_taxable_entry` false)",
            "`taxable_entries` are entries on Sales Accounts ledgers only (a sale booked to another ledger, or whose sales ledger sits in an inventory allocation, has `has_taxable_entry` false)",
        ),
        (
            "(Sales, Journal, Payment and so on)",
            "(Purchase, Journal, Payment and so on)",
        ),
        (
            "a Purchase or Debit Note voucher with no entry on a Duties & Taxes ledger under `purchase_vouchers_without_duties_taxes_entry` (exempt or unregistered purchases, or tax booked to a ledger filed elsewhere)",
            "a Sales or Credit Note voucher with no entry on a Duties & Taxes ledger under `sales_vouchers_without_duties_taxes_entry` (listed by identity only; the tool does not say why such a voucher carries no tax entry)",
        ),
        (
            "false when no entry sits on a Purchase Accounts ledger (an item invoice may hold it in an inventory allocation)",
            "false when no entry sits on a Sales Accounts ledger (a sale typed on Tally's screen or an item invoice may hold the sales ledger in an inventory allocation instead; not measured)",
        ),
        (
            "Not measured: REFERENCEDATE (not returned), item invoices whose purchase ledger sits in an inventory allocation, and books with several currencies.",
            "Measured so far: `sales_register` was run against a live Tally on two synthetic companies. On the first, once per day, for one taxed Sales item invoice and one untaxed one: the taxed sale came back as one row with its CGST and SGST/UTGST heads taken from the ledger masters and its sales ledger as the taxable entry, and the untaxed one was counted under `sales_vouchers_without_duties_taxes_entry`. On the second, which has 44 ledgers, for one Credit Note in voucher view booked on account: one row, with its CGST and state-tax heads and its sales ledger as the taxable entry. One Sales accounting voucher (not an invoice) was also classified, in tests, against the ledger masters of the purchase register's lab book. A Credit Note is returned as a row with its signs reversed as Tally sends them: the tool neither nets nor flips, so a caller that sums tax over a window must add signed amounts. The measured Credit Note of 1,000.00 with 90.00 CGST and 90.00 State Tax came back with the sales entry -1000.00, each tax entry -90.00 and the party entry 1180.00, where a Sales row has the sales and tax entries positive and the party entry negative. The state-side tax head is `state_tax` (raw State Tax) on one measured book and `sgst_utgst` (raw SGST/UTGST) on another; both are recognised heads for the same side of the tax, so a caller must not look for one of them only. The cost of a call varies by book: 96 requests on a book with 8 ledgers and one currency, 118 on one with 44 ledgers and two currencies, which adds a voucher census and base-currency reads; the result does not report the cost. Not shown by any run: an invoice-view Credit Note; an inter-state (IGST) line; a cancelled or optional sales voucher; an unrecognised or missing duty head on a sale; more than one voucher in a window; paging; a company with a registration; a tax that Tally computes itself; a sale typed on Tally's screen; accounting-invoice mode; a post-dated sale; a REFERENCE or a populated PARTYGSTIN on a sale; REFERENCEDATE (not returned); and a ledger or voucher kept in a currency other than the book's base. A row of such a kind is returned, not withheld, and carries `not_measured_live` naming why (invoice_view_credit_note, inter_state_line, sales_ledger_not_an_entry, cancelled, optional, post_dated, party_gstin_present, reference_present) only where the row itself shows the kind. Kinds a row cannot show are never marked and are not vouched for: a sale typed on Tally's screen in voucher view, a tax Tally computed itself, a duty head no sales capture has (such as cess), an invoice of another shape than the one run (for example several goods lines), and a ledger or voucher kept in a currency other than the book's base; an unmarked row is not a measured one in those respects. A row is marked `inter_state_line` only when a tax entry's ledger master carries a recognised IGST head; an IGST ledger with no head, or an unrecognised head, is listed under the without-head or unrecognised list and the status is not complete.",
        ),
    ];
    let mut text = shared.to_string();
    for (from, to) in swaps {
        assert!(text.contains(from), "the purchase text lost: {from}");
        text = text.replace(from, to);
    }
    format!(" It inherits the compliance read{text}")
}

#[test]
fn the_sales_description_is_the_purchase_one_with_the_sales_swaps() {
    let sales = RegisterKind::Sales.description();
    assert!(sales.starts_with("Read-only: a register of what the books record, not a GST return."));
    assert!(
        sales.ends_with(&derived_sales_tail()),
        "the sales description no longer follows the purchase description"
    );
    assert_eq!(
        RegisterKind::Purchase.description(),
        PURCHASE_REGISTER_DESCRIPTION
    );
}

#[test]
fn the_sales_description_says_nothing_only_the_purchase_register_would() {
    let sales = RegisterKind::Sales.description();
    for purchase_only in [
        "Purchase Accounts",
        "Debit Note vouchers",
        "purchase_vouchers_without_duties_taxes_entry",
        "input tax credit eligibility",
        "GSTR-2B",
        "exempt or unregistered",
    ] {
        assert!(
            !sales.contains(purchase_only),
            "sales text says {purchase_only}"
        );
    }
    for stated in [
        "Return the Sales and Credit Note vouchers of a date window",
        "`sales_vouchers_without_duties_taxes_entry`",
        "entries on Sales Accounts ledgers only",
        "or whose sales ledger sits in an inventory allocation",
        "may hold the sales ledger in an inventory allocation instead; not measured",
        "the tool does not say why such a voucher carries no tax entry",
        "A Debit Note, including one issued to a customer, is not a sales row",
        "run against a live Tally on two synthetic companies",
        "Not shown by any run: an invoice-view Credit Note",
        "its signs reversed as Tally sends them",
        "must not look for one of them only",
        "The cost of a call varies by book",
        "more than one voucher in a window",
        "REFERENCEDATE (not returned)",
        "`not_measured_live`",
        "only where the row itself shows the kind",
        "Kinds a row cannot show are never marked and are not vouched for",
        "a recognised IGST head",
        "its CGST and SGST/UTGST heads",
    ] {
        assert!(sales.contains(stated), "sales text lacks: {stated}");
    }
}

/// The classifier's own markers for hand-set kinds. The rows are the captured Sales voucher
/// with one field set, so these test the marker rule, not any capture of such a voucher.
#[test]
fn a_sales_row_of_a_kind_no_capture_covers_says_so_and_a_covered_one_does_not() {
    let captured = row_on(&sales_page().rows, "20250918").clone();
    assert!(
        captured.get("not_measured_live").is_none(),
        "the captured Sales voucher is a measured kind"
    );
    let mut rows = captured_rows();
    let sale = rows
        .iter_mut()
        .find(|row| row["voucher_class"] == "Sales")
        .unwrap();
    let cases = [
        ("cancelled", json!(true), "cancelled"),
        ("optional", json!(true), "optional"),
        ("post_dated", json!(true), "post_dated"),
        ("party_gstin", json!("SYNTHETIC"), "party_gstin_present"),
        ("reference", json!("SYN-REF"), "reference_present"),
    ];
    for (key, value, code) in cases {
        let mut marked = sale.clone();
        marked[key] = value;
        let page =
            super::classify_register(RegisterKind::Sales, &captured_index(), &[marked]).unwrap();
        assert_eq!(page.rows.len(), 1, "{key}");
        assert_eq!(
            page.rows[0]["not_measured_live"],
            json!([code]),
            "{key} should mark the row"
        );
    }
    // A Credit Note in voucher view was measured live; one in the invoice view was not.
    let mut credit = sale.clone();
    credit["voucher_class"] = json!("Credit Note");
    let page = super::classify_register(RegisterKind::Sales, &captured_index(), &[credit.clone()])
        .unwrap();
    assert!(page.rows[0].get("not_measured_live").is_none());
    credit["is_invoice"] = json!(true);
    let page = super::classify_register(RegisterKind::Sales, &captured_index(), &[credit]).unwrap();
    assert_eq!(
        page.rows[0]["not_measured_live"],
        json!(["invoice_view_credit_note"])
    );
    // An invoice-view voucher is not marked: one taxed and one untaxed item invoice were run live.
    let mut invoice = sale.clone();
    invoice["is_invoice"] = json!(true);
    let page =
        super::classify_register(RegisterKind::Sales, &captured_index(), &[invoice]).unwrap();
    assert!(page.rows[0].get("not_measured_live").is_none());
    // The purchase register's rows never carry the field.
    let purchases = classify_register(&captured_index(), &rows).unwrap();
    assert!(purchases
        .rows
        .iter()
        .all(|row| row.get("not_measured_live").is_none()));
}

/// An IGST entry and a sale whose sales ledger is not an entry are marked by the entries they
/// carry. The first row is the captured inter-state purchase read as a sale (its tax is on the
/// igst head); the second drops the sales ledger entry from the captured Sales voucher.
#[test]
fn an_igst_line_and_a_missing_sales_ledger_entry_are_marked() {
    let mut rows = captured_rows();
    let purchase = rows
        .iter()
        .find(|row| row["date"] == "20250905")
        .cloned()
        .unwrap();
    let mut as_sale = purchase;
    as_sale["voucher_class"] = json!("Sales");
    let page =
        super::classify_register(RegisterKind::Sales, &captured_index(), &[as_sale]).unwrap();
    let marks = page.rows[0]["not_measured_live"]
        .as_array()
        .unwrap()
        .clone();
    assert!(marks.contains(&json!("inter_state_line")));
    // Its Purchase Accounts ledger is not the sales group, so there is no taxable entry.
    assert!(marks.contains(&json!("sales_ledger_not_an_entry")));
    let sale = rows
        .iter_mut()
        .find(|row| row["voucher_class"] == "Sales")
        .unwrap();
    sale["amounts"]
        .as_array_mut()
        .unwrap()
        .retain(|entry| entry["ledger"] != "Sales - Goods");
    let page = super::classify_register(
        RegisterKind::Sales,
        &captured_index(),
        std::slice::from_ref(sale),
    )
    .unwrap();
    assert_eq!(page.rows[0]["has_taxable_entry"], false);
    assert_eq!(
        page.rows[0]["not_measured_live"],
        json!(["sales_ledger_not_an_entry"])
    );
    assert_eq!(page.rows[0]["status"], "complete");
}

#[test]
fn the_purchase_result_keeps_its_profile_and_classes() {
    let result = result_for(captured_rows(), &captured_index(), Redaction::None).unwrap();
    assert_eq!(result.result["profile"], "agent_purchase_register_v1");
    assert_eq!(
        result.result["register_classes"],
        json!(["Purchase", "Debit Note"])
    );
}

// Waiting for the lab (no test stands in for them): the ledger masters and group collection of the company that holds the two
// Sales item invoices (`sales-day/`), read by the same build as the voucher windows. With them
// the first test below classifies both invoices end to end, and the untaxed invoice is the
// positive case for `sales_vouchers_without_duties_taxes_entry` (a Sales voucher with no entry
// on a Duties & Taxes ledger). Neither is written by hand; each reads the captures once they exist.
