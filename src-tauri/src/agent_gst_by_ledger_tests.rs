use bridge_tally_protocol::native_outstandings::parse_native_group_snapshot;
use bridge_tally_protocol::parse_native_party_ledger_master_records_with_evidence;

use super::*;

/// The synthetic lab book the captures below were read from (the `gst-ledger-l1` PROVENANCE table).
const COMPANY_GUID: &str = "aae372c5-a894-491c-8843-5532e43d2d2f";

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// TallyPrime 7.2's answers to Bridge's compliance master and group requests on the lab book,
/// whose masters and vouchers were typed on Tally's screen. Every expected figure below is the one
/// the lab item expected of the voucher keyed, which the live run confirmed.
fn captured_index() -> MasterIndex {
    let masters = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-L-1.utf16le.xml"
    ));
    let groups = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-g-1.utf16le.xml"
    ));
    let records = parse_native_party_ledger_master_records_with_evidence(&masters, COMPANY_GUID)
        .expect("the live master capture parses")
        .records
        .into_iter()
        .map(|row| row.record)
        .collect::<Vec<_>>();
    let groups = GroupIndex::build(
        parse_native_group_snapshot(&groups, COMPANY_GUID).expect("the live group capture parses"),
    );
    MasterIndex::build(records.iter(), &groups).expect("masters index")
}

/// The one voucher of each captured day, in day order: V1, V4, V5, V6, V7, V4b, V8, V9, V10,
/// V11, V2, V13, V14, V3.
fn captured_rows() -> Vec<Value> {
    let windows: [&[u8]; 14] = [
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-04nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-05nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-06nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-07nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-08nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-10nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-11nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-12nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-13nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-14nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-15nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-16nov-w-1.utf16le.xml"
        ),
        include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-17nov-w-1.utf16le.xml"
        ),
    ];
    windows
        .iter()
        .flat_map(|bytes| {
            let rows =
                parse_agent_rows(&utf16(bytes), COMPANY_GUID).expect("the live window parses");
            assert_eq!(rows.len(), 1, "one voucher a day");
            rows
        })
        .collect()
}

/// The voucher dated that day of November 2025.
fn on(day: &str) -> Value {
    let date = format!("202511{day}");
    captured_rows()
        .into_iter()
        .find(|row| row["date"] == date)
        .unwrap_or_else(|| panic!("no voucher on {date}"))
}

fn heads(totals: &HeadTotals) -> Vec<(String, String)> {
    totals
        .0
        .iter()
        .map(|(head, amount)| (head.clone(), amount.as_str().to_string()))
        .collect()
}

fn pairs(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|(head, amount)| ((*head).to_string(), (*amount).to_string()))
        .collect()
}

/// The state-side head this book's ledgers carry is SGST/UTGST.
fn both(amount: &str) -> Vec<(String, String)> {
    pairs(&[("cgst", amount), ("sgst_utgst", amount)])
}

fn reason(index: &MasterIndex, row: &Value) -> SetAside {
    match dispose(index, row).unwrap() {
        Disposition::SetAside { reason, .. } => reason,
        other => panic!("expected a set-aside voucher, got {other:?}"),
    }
}

fn attributed(index: &MasterIndex, row: &Value) -> (String, HeadTotals, HeadTotals) {
    match dispose(index, row).unwrap() {
        Disposition::Attributed {
            ledger,
            debit,
            credit,
        } => (ledger, debit, credit),
        other => panic!("expected an attributed voucher, got {other:?}"),
    }
}

#[test]
fn the_captured_days_attribute_to_five_counter_ledgers_and_set_the_rest_aside() {
    let totals = fold(&captured_index(), &captured_rows()).unwrap();
    assert_eq!(totals.observed, 14);
    assert_eq!(totals.not_in_books, 0);
    assert_eq!(totals.without_gst_entry, 0);
    assert_eq!(
        totals.ledgers.keys().collect::<Vec<_>>(),
        [
            "GS1 Computer",
            "GS1 Freight Direct",
            "GS1 Professional Fees",
            "GS1 Rent",
            "Purchase - Goods"
        ]
    );
    // V1, a purchase, and V8, a debit note whose tax is on the credit side.
    let rent = &totals.ledgers["GS1 Rent"];
    assert_eq!(rent.vouchers, 2);
    assert_eq!(heads(&rent.debit), both("-900"));
    assert_eq!(heads(&rent.credit), both("90"));
    // V4, a journal: an expense with input IGST and the supplier credited.
    let fees = &totals.ledgers["GS1 Professional Fees"];
    assert_eq!(fees.vouchers, 1);
    assert_eq!(heads(&fees.debit), pairs(&[("igst", "-3600")]));
    // V5 and V6: a Fixed Assets and a Direct Expenses ledger.
    assert_eq!(heads(&totals.ledgers["GS1 Computer"].debit), both("-3600"));
    assert_eq!(
        heads(&totals.ledgers["GS1 Freight Direct"].debit),
        both("-180")
    );
    // V2, an item invoice whose purchase ledger is among its entries.
    assert_eq!(
        heads(&totals.ledgers["Purchase - Goods"].debit),
        both("-450")
    );
}

#[test]
fn the_vouchers_no_single_ledger_takes_are_set_aside_with_their_tax() {
    let totals = fold(&captured_index(), &captured_rows()).unwrap();
    assert_eq!(
        totals.set_aside.keys().copied().collect::<Vec<_>>(),
        [
            SetAside::ClassNotAdmitted,
            SetAside::DutyLedgerWithoutGstType,
            SetAside::GstOnBothSides,
            SetAside::NoCreditorParty,
            SetAside::SeveralCounterLedgers
        ]
    );
    // V14, a credit note: its class is never one whose GST is attributed.
    let note = &totals.set_aside[&SetAside::ClassNotAdmitted];
    assert_eq!(note.vouchers, 1);
    assert_eq!(note.listed[0]["voucher_class"], "Credit Note");
    assert_eq!(heads(&note.debit), both("-90"));
    // V4b: GS1 TDS Payable is typed Others, which may be a GST ledger left untyped.
    let untyped = &totals.set_aside[&SetAside::DutyLedgerWithoutGstType];
    assert_eq!(heads(&untyped.debit), pairs(&[("igst", "-1800")]));
    // V9, a set-off journal.
    let set_off = &totals.set_aside[&SetAside::GstOnBothSides];
    assert_eq!(heads(&set_off.debit), pairs(&[("cgst", "-500")]));
    assert_eq!(heads(&set_off.credit), pairs(&[("cgst", "500")]));
    // V10 (a reversal with no supplier), V11 and V13 (tax payments).
    let no_supplier = &totals.set_aside[&SetAside::NoCreditorParty];
    assert_eq!(no_supplier.vouchers, 3);
    assert_eq!(heads(&no_supplier.debit), both("-190"));
    assert_eq!(heads(&no_supplier.credit), both("90"));
    // V7 (two expense ledgers) and V3 (an item invoice with freight).
    let several = &totals.set_aside[&SetAside::SeveralCounterLedgers];
    assert_eq!(several.vouchers, 2);
    assert_eq!(heads(&several.debit), both("-1350"));
}

#[test]
fn a_gst_challan_that_also_pays_interest_is_never_attributed_to_the_interest_ledger() {
    // V13: Output CGST and SGST and GS1 Interest on GST (Indirect Expenses) against Cash.
    assert_eq!(
        reason(&captured_index(), &on("15")),
        SetAside::NoCreditorParty
    );
}

#[test]
fn a_discount_credit_note_with_output_tax_is_never_attributed_to_the_discount_ledger() {
    // V14: GS1 Discount Allowed (Indirect Expenses) and output tax against the customer.
    assert_eq!(
        reason(&captured_index(), &on("16")),
        SetAside::ClassNotAdmitted
    );
}

#[test]
fn a_purchase_whose_gst_ledgers_have_an_unresolved_group_is_set_aside_not_counted_as_untaxed() {
    let mut index = captured_index();
    for name in ["Input CGST", "Input SGST"] {
        index.by_name.get_mut(name).unwrap().group =
            LedgerGroup::Unresolved(AncestryGap::GroupAbsent);
    }
    assert_eq!(reason(&index, &on("03")), SetAside::UnresolvedGroup);
}

#[test]
fn a_duty_ledger_typed_tds_does_not_stop_the_attribution_and_one_typed_others_does() {
    let index = captured_index();
    let with_tds = on("08");
    assert_eq!(
        reason(&index, &with_tds),
        SetAside::DutyLedgerWithoutGstType
    );
    let mut typed = index.clone();
    typed.by_name.get_mut("GS1 TDS Payable").unwrap().head = GstDutyHeadObservation::NotTaxLedger {
        tax_type: "TDS".to_string(),
    };
    let (ledger, debit, _) = attributed(&typed, &with_tds);
    assert_eq!(ledger, "GS1 Professional Fees");
    assert_eq!(heads(&debit), pairs(&[("igst", "-1800")]));
}

#[test]
fn an_expense_journal_with_input_tax_only_is_attributed_to_the_expense_ledger() {
    let (ledger, debit, credit) = attributed(&captured_index(), &on("04"));
    assert_eq!(ledger, "GS1 Professional Fees");
    assert_eq!(heads(&debit), pairs(&[("igst", "-3600")]));
    assert!(credit.0.is_empty());
}

#[test]
fn a_purchase_whose_ledger_entries_do_not_balance_is_never_attributed() {
    let index = captured_index();
    let mut row = on("03");
    // As an item invoice of a shape not measured would read: the purchase ledger is not among
    // the ledger entries.
    row["amounts"]
        .as_array_mut()
        .unwrap()
        .retain(|entry| entry["ledger"] != "GS1 Rent");
    assert_eq!(reason(&index, &row), SetAside::EntriesDoNotBalance);
    // And with a freight ledger on it, which must not take the invoice's tax.
    row["amounts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"ledger": "GS1 Freight Inward", "amount": "-500.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    assert_eq!(reason(&index, &row), SetAside::EntriesDoNotBalance);
}

#[test]
fn two_counter_ledgers_another_group_or_a_sales_ledger_set_the_voucher_aside() {
    let index = captured_index();
    // V7, typed with two expense ledgers.
    assert_eq!(reason(&index, &on("07")), SetAside::SeveralCounterLedgers);
    // V1 with part of its expense on a ledger outside the counter and settlement groups: this
    // book has none, so one ledger's group is changed in the index (Current Assets here).
    let mut row = on("03");
    let entries = row["amounts"].as_array_mut().unwrap();
    for entry in entries.iter_mut() {
        if entry["ledger"] == "GS1 Rent" {
            entry["amount"] = json!("-9000.00");
        }
    }
    entries.push(json!({"ledger": "GS1 Interest on GST", "amount": "-1000.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    let mut other = index.clone();
    other
        .by_name
        .get_mut("GS1 Interest on GST")
        .unwrap()
        .reserved = Some("Current Assets".to_string());
    assert_eq!(reason(&other, &row), SetAside::OtherLedgersPresent);
    // A sales ledger on a purchase is its own reason, ahead of the others.
    let mut sales = index;
    sales
        .by_name
        .get_mut("GS1 Interest on GST")
        .unwrap()
        .reserved = Some("Sales Accounts".to_string());
    assert_eq!(reason(&sales, &row), SetAside::SalesLedgerPresent);
}

#[test]
fn a_class_outside_the_captured_ones_is_set_aside_first() {
    let index = captured_index();
    let mut row = on("03");
    row["voucher_class"] = json!("Memorandum");
    assert_eq!(reason(&index, &row), SetAside::ClassNotAdmitted);
    row.as_object_mut().unwrap().remove("voucher_class");
    assert_eq!(reason(&index, &row), SetAside::ClassNotAdmitted);
}

#[test]
fn a_tax_entry_whose_sign_and_deemed_positive_flag_disagree_is_set_aside() {
    let index = captured_index();
    let mut row = on("03");
    for entry in row["amounts"].as_array_mut().unwrap() {
        if entry["ledger"] == "Input CGST" {
            entry["is_deemed_positive"] = json!("No");
        }
    }
    assert_eq!(reason(&index, &row), SetAside::SignAndFlagDisagree);
}

#[test]
fn cancelled_optional_and_post_dated_vouchers_are_counted_not_totalled() {
    let index = captured_index();
    let base = on("03");
    for key in ["cancelled", "optional", "post_dated"] {
        let mut row = base.clone();
        row[key] = json!(true);
        assert_eq!(dispose(&index, &row).unwrap(), Disposition::NotInBooks);
    }
}

#[test]
fn a_voucher_naming_a_ledger_the_masters_do_not_list_refuses() {
    let index = captured_index();
    let mut row = on("03");
    row["amounts"][0]["ledger"] = json!("A ledger not in the masters");
    assert_eq!(
        dispose(&index, &row).unwrap_err(),
        "ledger_snapshot_drifted"
    );
}

#[test]
fn an_amount_that_is_not_an_exact_decimal_refuses() {
    let index = captured_index();
    let mut row = on("03");
    row["amounts"][0]["amount"] = json!("1,000.00");
    assert_eq!(dispose(&index, &row).unwrap_err(), "voucher_amount_invalid");
}

#[test]
fn a_partial_window_has_no_totals() {
    let window = RegisterWindow {
        index: captured_index(),
        rows: captured_rows(),
        withheld_reason: WITHHELD_FOREIGN_CURRENCY,
        state: "partial",
        reason: Some("nonempty_window_unqualified"),
    };
    assert_eq!(
        CompleteWindow::admit(window, ("20251101", "20251130")).err(),
        Some("gst_by_ledger_window_partial".to_string())
    );
}

#[test]
fn the_answer_lists_each_counter_ledger_and_each_reason_by_its_code() {
    let index = captured_index();
    let totals = fold(&index, &captured_rows()).unwrap();
    let result = gst_by_ledger_result(&index, &totals, Redaction::None);
    let first = &result["counter_ledgers"][0];
    assert_eq!(first["ledger"], "GS1 Computer");
    assert_eq!(first["group"], "Fixed Assets");
    assert_eq!(first["parent"], "Fixed Assets");
    assert_eq!(first["vouchers"], 1);
    assert_eq!(
        result["set_aside"]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        [
            "class_not_admitted",
            "duty_ledger_without_gst_type",
            "gst_on_both_sides",
            "no_creditor_party",
            "several_counter_ledgers"
        ]
    );
    assert_eq!(
        result["set_aside"]["no_creditor_party"]["listed"]["total"],
        3
    );
}

#[test]
fn under_mask_parties_a_ledger_and_a_parent_the_book_named_are_masked_and_a_predefined_parent_is_not(
) {
    let mut index = captured_index();
    let totals = fold(&index, &captured_rows()).unwrap();
    let masked = gst_by_ledger_result(&index, &totals, Redaction::MaskParties);
    let computer = &masked["counter_ledgers"][0];
    assert_ne!(computer["ledger"], "GS1 Computer");
    assert_eq!(computer["parent"], "Fixed Assets");
    index.by_name.get_mut("GS1 Computer").unwrap().parent =
        Some("SYN Own Assets Group".to_string());
    let masked = gst_by_ledger_result(&index, &totals, Redaction::MaskParties);
    assert_ne!(
        masked["counter_ledgers"][0]["parent"],
        "SYN Own Assets Group"
    );
    assert_eq!(masked["counter_ledgers"][0]["group"], "Fixed Assets");
}

/// A counter ledger directly under Tally's root shows the root as Tally sent it, and
/// `mask_parties` leaves it as it is, where it masked it into a fragment of the marker.
#[test]
fn a_parent_that_is_tally_root_is_shown_as_tally_sent_it_and_is_not_masked() {
    let mut index = captured_index();
    let totals = fold(&index, &captured_rows()).unwrap();
    index.by_name.get_mut("GS1 Computer").unwrap().parent = Some(format!(
        "{} Primary",
        bridge_tally_protocol::TALLY_SANITIZED_ROOT_MARKER
    ));
    for redaction in [Redaction::None, Redaction::MaskParties] {
        let result = gst_by_ledger_result(&index, &totals, redaction);
        assert_eq!(
            result["counter_ledgers"][0]["parent"], "&#4; Primary",
            "{redaction:?}"
        );
    }
}

/// A withheld voucher leaves the totals short, so the window refuses; it is checked against the
/// window's dates first, so one dated outside the window refuses as any row does (#674, #683).
#[test]
fn a_withheld_voucher_refuses_the_totals_and_one_outside_the_window_refuses_as_not_honoured() {
    let window = || RegisterWindow {
        index: captured_index(),
        rows: crate::agent::voucher_parse::captured_72_register_window(),
        withheld_reason: WITHHELD_FOREIGN_CURRENCY,
        state: "complete",
        reason: None,
    };
    assert_eq!(
        CompleteWindow::admit(window(), ("20250402", "20250430")).err(),
        Some("window_not_honoured".to_string())
    );
    assert_eq!(
        CompleteWindow::admit(window(), ("20250401", "20250401")).err(),
        Some("gst_by_ledger_window_partial".to_string())
    );
}

/// The same book's masters and groups as its December read found them, after a second supplier
/// and a second copy of one supplier invoice were typed on Tally's screen (`gst-ledger-l3`
/// PROVENANCE table).
fn december_index() -> MasterIndex {
    let masters = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l3/l3-dec-L-1.utf16le.xml"
    ));
    let groups = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l3/l3-dec-g-1.utf16le.xml"
    ));
    let records = parse_native_party_ledger_master_records_with_evidence(&masters, COMPANY_GUID)
        .expect("the live master capture parses")
        .records
        .into_iter()
        .map(|row| row.record)
        .collect::<Vec<_>>();
    let groups = GroupIndex::build(
        parse_native_group_snapshot(&groups, COMPANY_GUID).expect("the live group capture parses"),
    );
    MasterIndex::build(records.iter(), &groups).expect("masters index")
}

/// December 2025's six vouchers: Purchase 57, which opens supplier invoice GS1/P/0001 (New Ref);
/// Purchase 58, the same invoice keyed again, Agst Ref that bill; an optional Purchase 60 and a
/// cancelled Purchase 59; Journal 5; and Payment 3, Agst Ref the bill with no GST entry.
fn december_rows() -> Vec<Value> {
    let rows = parse_agent_rows(
        &utf16(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l3/l3-dec-w-1.utf16le.xml"
        )),
        COMPANY_GUID,
    )
    .expect("the live window parses");
    assert_eq!(rows.len(), 6);
    rows
}

fn numbered<'a>(rows: &'a mut [Value], number: &str) -> &'a mut Value {
    rows.iter_mut()
        .find(|row| row["voucher_number"] == number)
        .unwrap_or_else(|| panic!("no voucher {number}"))
}

/// Sets every entry on `ledger`, and each bill allocation on it, to `amount`.
fn set_amount(row: &mut Value, ledger: &str, amount: &str) {
    for entry in row["amounts"].as_array_mut().unwrap() {
        if entry["ledger"] == ledger {
            entry["amount"] = json!(amount);
            for allocation in entry["bill_allocations"].as_array_mut().unwrap() {
                allocation["amount"] = json!(amount);
            }
        }
    }
}

fn agst_ref_numbers(totals: &GstByLedger) -> Vec<&Value> {
    totals
        .agst_ref_purchases
        .iter()
        .map(|item| &item["voucher_number"])
        .collect()
}

#[test]
fn a_supplier_invoice_keyed_twice_has_its_copy_set_aside_and_its_opener_counted() {
    let index = december_index();
    let totals = fold(&index, &december_rows()).unwrap();
    assert_eq!(totals.observed, 6);
    assert_eq!(totals.not_in_books, 2);
    assert_eq!(totals.without_gst_entry, 1);
    let goods = &totals.ledgers["Purchase - Goods"];
    assert_eq!(goods.vouchers, 1);
    assert_eq!(heads(&goods.debit), both("-900"));
    let copy = &totals.set_aside[&SetAside::BillAlreadyOpened];
    assert_eq!(copy.vouchers, 1);
    assert_eq!(heads(&copy.debit), both("-900"));
    assert!(copy.credit.0.is_empty());
    assert_eq!(copy.listed[0]["voucher_number"], "58");
    assert_eq!(copy.listed[0]["voucher_class"], "Purchase");
    assert_eq!(totals.set_aside.len(), 1);
    // Journal 5's input tax stays on GS1 Rent.
    assert_eq!(heads(&totals.ledgers["GS1 Rent"].debit), both("-450"));
    assert!(totals.agst_ref_purchases.is_empty());
    let result = gst_by_ledger_result(&index, &totals, Redaction::None);
    assert_eq!(
        result["set_aside"]["bill_already_opened"]["listed"]["total"],
        1
    );
    assert_eq!(result["agst_ref_purchase"]["total"], 0);
}

#[test]
fn an_agst_ref_purchase_whose_amounts_differ_is_counted_and_listed() {
    let index = december_index();
    // A supplementary invoice of half the value, its tax halved with it.
    let mut rows = december_rows();
    let second = numbered(&mut rows, "58");
    set_amount(second, "Purchase - Goods", "-5000.00");
    set_amount(second, "Input CGST", "-450.00");
    set_amount(second, "Input SGST", "-450.00");
    set_amount(second, "SYN GS1 Supplier B", "5900.00");
    let totals = fold(&index, &rows).unwrap();
    let goods = &totals.ledgers["Purchase - Goods"];
    assert_eq!(goods.vouchers, 2);
    assert_eq!(heads(&goods.debit), both("-1350"));
    assert!(totals.set_aside.is_empty());
    assert_eq!(agst_ref_numbers(&totals), ["58"]);
    // The same tax on a different value is not the same invoice either.
    let mut rows = december_rows();
    let second = numbered(&mut rows, "58");
    set_amount(second, "Purchase - Goods", "-10100.00");
    set_amount(second, "SYN GS1 Supplier B", "11900.00");
    let totals = fold(&index, &rows).unwrap();
    assert_eq!(totals.ledgers["Purchase - Goods"].vouchers, 2);
    assert!(totals.set_aside.is_empty());
    assert_eq!(agst_ref_numbers(&totals), ["58"]);
    let result = gst_by_ledger_result(&index, &totals, Redaction::None);
    assert_eq!(result["agst_ref_purchase"]["total"], 1);
    assert_eq!(
        result["agst_ref_purchase"]["listed"][0]["voucher_number"],
        "58"
    );
}

#[test]
fn the_same_bill_on_another_party_is_not_a_copy() {
    let index = december_index();
    let mut rows = december_rows();
    let second = numbered(&mut rows, "58");
    for entry in second["amounts"].as_array_mut().unwrap() {
        if entry["ledger"] == "SYN GS1 Supplier B" {
            entry["ledger"] = json!("SYN GS1 Supplier");
        }
    }
    let totals = fold(&index, &rows).unwrap();
    assert_eq!(totals.ledgers["Purchase - Goods"].vouchers, 2);
    assert!(totals.set_aside.is_empty());
    assert_eq!(agst_ref_numbers(&totals), ["58"]);
}

#[test]
fn a_debit_note_or_journal_against_the_bill_stays_counted_and_unlisted() {
    let index = december_index();
    for class in ["Debit Note", "Journal"] {
        let mut rows = december_rows();
        numbered(&mut rows, "58")["voucher_class"] = json!(class);
        let totals = fold(&index, &rows).unwrap();
        assert_eq!(totals.ledgers["Purchase - Goods"].vouchers, 2, "{class}");
        assert!(totals.set_aside.is_empty(), "{class}");
        assert!(totals.agst_ref_purchases.is_empty(), "{class}");
    }
}

#[test]
fn a_copy_that_opens_a_bill_of_its_own_or_has_no_counted_opener_stays_counted() {
    let index = december_index();
    // No. 58 also opening a bill: an opener is never set aside, so two vouchers can never set
    // each other aside.
    let mut rows = december_rows();
    for entry in numbered(&mut rows, "58")["amounts"].as_array_mut().unwrap() {
        if entry["ledger"] == "SYN GS1 Supplier B" {
            let mut opened = entry["bill_allocations"][0].clone();
            opened["bill_type"] = json!("New Ref");
            opened["reference"]["name"] = json!("GS1/P/0002");
            entry["bill_allocations"]
                .as_array_mut()
                .unwrap()
                .push(opened);
        }
    }
    let totals = fold(&index, &rows).unwrap();
    assert_eq!(totals.ledgers["Purchase - Goods"].vouchers, 2);
    assert!(totals.set_aside.is_empty());
    assert_eq!(agst_ref_numbers(&totals), ["58"]);
    // No. 57 cancelled: the bill's opener is not counted, so No. 58 carries the invoice's tax.
    let mut rows = december_rows();
    numbered(&mut rows, "57")["cancelled"] = json!(true);
    let totals = fold(&index, &rows).unwrap();
    let goods = &totals.ledgers["Purchase - Goods"];
    assert_eq!(goods.vouchers, 1);
    assert_eq!(heads(&goods.debit), both("-900"));
    assert!(totals.set_aside.is_empty());
    assert_eq!(agst_ref_numbers(&totals), ["58"]);
}
