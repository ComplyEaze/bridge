use bridge_tally_protocol::native_outstandings::parse_native_group_snapshot;
use bridge_tally_protocol::parse_native_party_ledger_master_records_with_evidence;

use super::*;

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
fn a_head_the_classifier_does_not_recognise_is_listed_with_its_raw_spelling() {
    // The ledger `Input SGST 9% (M2)` was created with the head `SGST/UTGST`. Whether that
    // spelling is recognised depends on a separate change; what must hold either way is that
    // the entry is named with the spelling Tally returned and is never folded into a head.
    let page = classify_register(&captured_index(), &captured_rows()).unwrap();
    let row = row_on(&page.rows, "20250909");
    let in_tax = row["tax_in_books"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["ledger"] == "Input SGST 9% (M2)");
    let unrecognised = row["duties_taxes_entries_with_unrecognised_head"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["ledger"] == "Input SGST 9% (M2)");
    match (in_tax, unrecognised) {
        (Some(entry), None) => {
            assert_eq!(entry["raw_head"], "SGST/UTGST");
            assert_eq!(row["status"], "complete");
        }
        (None, Some(entry)) => {
            assert_eq!(entry["raw_head"], "SGST/UTGST");
            assert_eq!(entry["observation"], "unrecognized");
            assert_eq!(row["status"], "has_unrecognised_head");
        }
        other => panic!("the entry must be in exactly one list, got {other:?}"),
    }
    assert_eq!(row["tax_in_books"][0]["ledger"], "Input CGST");
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
fn party_names_are_marked_for_redaction_but_tax_ledger_names_are_not() {
    let row = mark_register_row(json!({
        "party": "Customer One",
        "tax_in_books": [{"ledger": "Input CGST"}],
        "party_entries": [{"ledger": "Customer One"}],
        "other_entries": [{"ledger": "Supplier Two"}],
    }));
    assert_eq!(row["tax_in_books"][0]["ledger"], "Input CGST");
    let redacted = redact_value(row, Redaction::MaskParties);
    let text = redacted.to_string();
    assert!(
        !text.contains("Customer One") && !text.contains("Supplier Two"),
        "{text}"
    );
    assert!(text.contains("Input CGST"), "{text}");
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
