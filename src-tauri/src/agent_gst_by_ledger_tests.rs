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

/// The same live captures the purchase register's tests read: TallyPrime 7.1 Silver's answers
/// to Bridge's compliance master, group and class-shaped window requests on the lab book, one
/// month (September 2025) of eight vouchers created by import. Every expected figure below was
/// worked out by hand from those bytes (entry amounts, and each tax ledger's GSTDUTYHEAD in the
/// master capture), not from this code's output.
fn captured_index() -> MasterIndex {
    let masters = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-ledger-masters.utf16le.xml"
    ));
    let groups = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-groups.utf16le.xml"
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
    MasterIndex::build(records.iter(), &groups, Vec::new()).expect("masters index")
}

fn captured_rows() -> Vec<Value> {
    let xml = utf16(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-register-lab-vouchers-class.utf16le.xml"
    ));
    parse_agent_rows(&xml, COMPANY_GUID).expect("the live voucher window parses")
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
        .map(|(head, amount)| (head.to_string(), amount.to_string()))
        .collect()
}

fn row_of_type<'a>(rows: &'a [Value], voucher_type: &str) -> &'a Value {
    rows.iter()
        .find(|row| row["voucher_type"] == voucher_type)
        .unwrap_or_else(|| panic!("no {voucher_type} voucher in the window"))
}

#[test]
fn the_captured_month_attributes_purchases_and_the_debit_note_to_their_purchase_ledgers() {
    let totals = fold(&captured_index(), &captured_rows()).unwrap();
    assert_eq!(totals.observed, 8);
    assert_eq!(totals.not_in_books, 0);
    assert_eq!(totals.without_gst_entry, 0);
    assert_eq!(
        totals.ledgers.keys().collect::<Vec<_>>(),
        ["Purchase - Goods", "Purchase - Services"]
    );
    // Two purchases (intra and inter) and the debit note, whose tax is on the credit side. The
    // third purchase on this ledger carries a TDS entry on a ledger typed Others, which may be a
    // GST ledger left untyped, so it is set aside (next test).
    let goods = &totals.ledgers["Purchase - Goods"];
    assert_eq!(goods.vouchers, 3);
    assert_eq!(
        heads(&goods.debit),
        pairs(&[("cgst", "-900"), ("igst", "-3600"), ("state_tax", "-900")])
    );
    assert_eq!(
        heads(&goods.credit),
        pairs(&[("cgst", "90"), ("state_tax", "90")])
    );
    // The services purchase's state-side ledger carries SGST/UTGST, kept apart from State Tax.
    let services = &totals.ledgers["Purchase - Services"];
    assert_eq!(services.vouchers, 1);
    assert_eq!(
        heads(&services.debit),
        pairs(&[("cgst", "-450"), ("sgst_utgst", "-450")])
    );
    assert!(services.credit.0.is_empty());
}

#[test]
fn the_sale_the_untyped_duty_ledger_the_journal_and_the_tax_payment_are_set_aside_with_their_tax() {
    let totals = fold(&captured_index(), &captured_rows()).unwrap();
    assert_eq!(
        totals.set_aside.keys().copied().collect::<Vec<_>>(),
        [
            SetAside::ClassNotAdmitted,
            SetAside::DutyLedgerWithoutGstType,
            SetAside::GstOnBothSides,
            SetAside::NoCreditorParty
        ]
    );
    // A sale is never attributed: its class is not one whose GST can be input tax.
    let sale = &totals.set_aside[&SetAside::ClassNotAdmitted];
    assert_eq!(sale.vouchers, 1);
    assert_eq!(sale.listed[0]["voucher_class"], "Sales");
    assert!(sale.debit.0.is_empty());
    assert_eq!(
        heads(&sale.credit),
        pairs(&[("cgst", "900"), ("state_tax", "900")])
    );
    // TDS Payable (M2) is typed Others on this book, not TDS.
    let untyped = &totals.set_aside[&SetAside::DutyLedgerWithoutGstType];
    assert_eq!(untyped.vouchers, 1);
    assert_eq!(
        heads(&untyped.debit),
        pairs(&[("cgst", "-90"), ("state_tax", "-90")])
    );
    // Freight Inward under Indirect Expenses is the one counter ledger, but the journal books
    // output tax as well as input tax, so its input tax is not given to the freight ledger.
    let journal = &totals.set_aside[&SetAside::GstOnBothSides];
    assert_eq!(journal.vouchers, 1);
    assert_eq!(journal.listed[0]["voucher_class"], "Journal");
    assert_eq!(
        heads(&journal.debit),
        pairs(&[("cgst", "-90"), ("state_tax", "-90")])
    );
    assert_eq!(
        heads(&journal.credit),
        pairs(&[("cgst", "90"), ("state_tax", "90")])
    );
    // The tax payment against cash has no supplier on it.
    let payment = &totals.set_aside[&SetAside::NoCreditorParty];
    assert_eq!(payment.listed[0]["voucher_class"], "Payment");
    assert_eq!(
        heads(&payment.debit),
        pairs(&[("cgst", "-90"), ("state_tax", "-90")])
    );
}

/// The captured tax payment (Output CGST and Output SGST debited against Cash), as a GST challan
/// that also pays interest: Freight Inward (M2), under Indirect Expenses, stands in for an
/// interest ledger, and the cash credit is raised by its amount. Output tax must never be
/// reported as an expense ledger's tax.
#[test]
fn a_gst_challan_that_also_pays_interest_is_never_attributed_to_the_interest_ledger() {
    let index = captured_index();
    let mut row = row_of_type(&captured_rows(), "Payment").clone();
    let entries = row["amounts"].as_array_mut().unwrap();
    for entry in entries.iter_mut() {
        if entry["ledger"] == "Cash" {
            entry["amount"] = json!("230.00");
        }
    }
    entries.push(json!({"ledger": "Freight Inward (M2)", "amount": "-50.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    assert_eq!(reason(&index, &row), SetAside::NoCreditorParty);
}

/// The same payment's entries as a credit note for a discount allowed with output tax: the
/// Indirect Expenses ledger debited and the customer credited.
#[test]
fn a_discount_credit_note_with_output_tax_is_never_attributed_to_the_discount_ledger() {
    let index = captured_index();
    let mut row = row_of_type(&captured_rows(), "Payment").clone();
    row["voucher_class"] = json!("Credit Note");
    let entries = row["amounts"].as_array_mut().unwrap();
    entries.retain(|entry| entry["ledger"] != "Cash");
    entries.push(json!({"ledger": "Freight Inward (M2)", "amount": "-1000.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    entries.push(json!({"ledger": "Counter Sales - Unregistered", "amount": "1180.00", "is_deemed_positive": "No", "bill_allocations": []}));
    assert_eq!(reason(&index, &row), SetAside::ClassNotAdmitted);
}

#[test]
fn a_purchase_whose_gst_ledgers_have_an_unresolved_group_is_set_aside_not_counted_as_untaxed() {
    let mut index = captured_index();
    for name in ["Input CGST", "Input SGST"] {
        index.by_name.get_mut(name).unwrap().group =
            LedgerGroup::Unresolved(AncestryGap::GroupAbsent);
    }
    let rows = captured_rows();
    let intra = rows
        .iter()
        .find(|row| {
            let entries = row["amounts"].as_array().unwrap();
            row["voucher_type"] == "Purchase"
                && entries.len() == 4
                && entries.iter().any(|entry| entry["ledger"] == "Input SGST")
        })
        .expect("the intra-state purchase");
    assert_eq!(reason(&index, intra), SetAside::UnresolvedGroup);
}

#[test]
fn a_duty_ledger_typed_tds_does_not_stop_the_attribution_and_one_typed_others_does() {
    let rows = captured_rows();
    let with_tds = rows
        .iter()
        .find(|row| {
            row["amounts"]
                .as_array()
                .unwrap()
                .iter()
                .any(|entry| entry["ledger"] == "TDS Payable (M2)")
        })
        .expect("the purchase with a TDS entry");
    let index = captured_index();
    assert_eq!(reason(&index, with_tds), SetAside::DutyLedgerWithoutGstType);
    let mut typed = index.clone();
    typed.by_name.get_mut("TDS Payable (M2)").unwrap().head =
        GstDutyHeadObservation::NotTaxLedger {
            tax_type: "TDS".to_string(),
        };
    match dispose(&typed, with_tds).unwrap() {
        Disposition::Attributed { ledger, debit, .. } => {
            assert_eq!(ledger, "Purchase - Goods");
            assert_eq!(
                heads(&debit),
                pairs(&[("cgst", "-90"), ("state_tax", "-90")])
            );
        }
        other => panic!("expected the purchase ledger, got {other:?}"),
    }
}

#[test]
fn an_expense_journal_with_input_tax_only_is_attributed_to_the_expense_ledger() {
    let index = captured_index();
    let rows = captured_rows();
    let mut journal = row_of_type(&rows, "Journal").clone();
    // The captured journal without its two output-tax entries and with the supplier's credit
    // raised by their amount: an expense booked with input tax, which this book has no capture of
    // on its own.
    let entries = journal["amounts"].as_array_mut().unwrap();
    entries.retain(|entry| !entry["ledger"].as_str().unwrap().starts_with("Output"));
    for entry in entries.iter_mut() {
        if entry["ledger"] == "SYN Supplier Intra (M2)" {
            entry["amount"] = json!("1180.00");
        }
    }
    match dispose(&index, &journal).unwrap() {
        Disposition::Attributed {
            ledger,
            debit,
            credit,
        } => {
            assert_eq!(ledger, "Freight Inward (M2)");
            assert_eq!(
                heads(&debit),
                pairs(&[("cgst", "-90"), ("state_tax", "-90")])
            );
            assert!(credit.0.is_empty());
        }
        other => panic!("expected the freight ledger, got {other:?}"),
    }
}

fn reason(index: &MasterIndex, row: &Value) -> SetAside {
    match dispose(index, row).unwrap() {
        Disposition::SetAside { reason, .. } => reason,
        other => panic!("expected a set-aside voucher, got {other:?}"),
    }
}

fn purchase(rows: &[Value]) -> Value {
    rows.iter()
        .find(|row| {
            row["voucher_type"] == "Purchase"
                && row["amounts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|entry| entry["ledger"] == "Input IGST")
        })
        .expect("the inter-state purchase")
        .clone()
}

#[test]
fn a_purchase_whose_ledger_entries_do_not_balance_is_never_attributed() {
    let index = captured_index();
    let mut row = purchase(&captured_rows());
    // As an item invoice reads: the purchase ledger is not among the ledger entries.
    row["amounts"]
        .as_array_mut()
        .unwrap()
        .retain(|entry| entry["ledger"] != "Purchase - Goods");
    assert_eq!(reason(&index, &row), SetAside::EntriesDoNotBalance);
    // And with a freight ledger on it, which must not take the invoice's tax.
    row["amounts"]
        .as_array_mut()
        .unwrap()
        .push(json!({"ledger": "Freight Inward (M2)", "amount": "-500.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    assert_eq!(reason(&index, &row), SetAside::EntriesDoNotBalance);
}

#[test]
fn two_counter_ledgers_or_another_group_set_the_voucher_aside() {
    let index = captured_index();
    let base = purchase(&captured_rows());
    let mut two = base.clone();
    let entries = two["amounts"].as_array_mut().unwrap();
    for entry in entries.iter_mut() {
        if entry["ledger"] == "Purchase - Goods" {
            entry["amount"] = json!("-15000.00");
        }
    }
    entries.push(json!({"ledger": "Purchase - Services", "amount": "-5000.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    assert_eq!(reason(&index, &two), SetAside::SeveralCounterLedgers);
    let mut other = base;
    let entries = other["amounts"].as_array_mut().unwrap();
    for entry in entries.iter_mut() {
        if entry["ledger"] == "Purchase - Goods" {
            entry["amount"] = json!("-19000.00");
        }
    }
    // The book's Suspense ledger stands in for any ledger outside the counter and settlement
    // groups (Direct Expenses, Fixed Assets, Current Assets, Stock-in-Hand and so on).
    entries.push(json!({"ledger": "Suspense", "amount": "-1000.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    assert_eq!(reason(&index, &other), SetAside::OtherLedgersPresent);
    // A sales ledger on a purchase is its own reason, ahead of the others.
    let entries = other["amounts"].as_array_mut().unwrap();
    entries.retain(|entry| entry["ledger"] != "Suspense");
    entries.push(json!({"ledger": "Sales - Goods", "amount": "-1000.00", "is_deemed_positive": "Yes", "bill_allocations": []}));
    assert_eq!(reason(&index, &other), SetAside::SalesLedgerPresent);
}

#[test]
fn a_class_outside_the_captured_ones_is_set_aside_first() {
    let index = captured_index();
    let mut row = purchase(&captured_rows());
    row["voucher_class"] = json!("Memorandum");
    assert_eq!(reason(&index, &row), SetAside::ClassNotAdmitted);
    row.as_object_mut().unwrap().remove("voucher_class");
    assert_eq!(reason(&index, &row), SetAside::ClassNotAdmitted);
}

#[test]
fn a_tax_entry_whose_sign_and_deemed_positive_flag_disagree_is_set_aside() {
    let index = captured_index();
    let mut row = purchase(&captured_rows());
    for entry in row["amounts"].as_array_mut().unwrap() {
        if entry["ledger"] == "Input IGST" {
            entry["is_deemed_positive"] = json!("No");
        }
    }
    assert_eq!(reason(&index, &row), SetAside::SignAndFlagDisagree);
}

#[test]
fn cancelled_optional_and_post_dated_vouchers_are_counted_not_totalled() {
    let index = captured_index();
    let base = purchase(&captured_rows());
    for key in ["cancelled", "optional", "post_dated"] {
        let mut row = base.clone();
        row[key] = json!(true);
        assert_eq!(dispose(&index, &row).unwrap(), Disposition::NotInBooks);
    }
}

#[test]
fn a_voucher_naming_a_ledger_the_masters_do_not_list_refuses() {
    let index = captured_index();
    let mut row = purchase(&captured_rows());
    row["amounts"][0]["ledger"] = json!("A ledger not in the masters");
    assert_eq!(
        dispose(&index, &row).unwrap_err(),
        "ledger_snapshot_drifted"
    );
}

#[test]
fn an_amount_that_is_not_an_exact_decimal_refuses() {
    let index = captured_index();
    let mut row = purchase(&captured_rows());
    row["amounts"][0]["amount"] = json!("1,000.00");
    assert_eq!(dispose(&index, &row).unwrap_err(), "voucher_amount_invalid");
}

#[test]
fn a_partial_window_has_no_totals() {
    let window = RegisterWindow {
        index: captured_index(),
        rows: captured_rows(),
        state: "partial",
        reason: Some("nonempty_window_unqualified"),
    };
    assert_eq!(
        CompleteWindow::admit(window).err(),
        Some("gst_by_ledger_window_partial".to_string())
    );
}

#[test]
fn the_answer_lists_each_counter_ledger_and_each_reason_by_its_code() {
    let index = captured_index();
    let totals = fold(&index, &captured_rows()).unwrap();
    let result = gst_by_ledger_result(&index, &totals, Redaction::None);
    let first = &result["counter_ledgers"][0];
    assert_eq!(first["group"], "Purchase Accounts");
    assert_eq!(first["parent"], "Purchase Accounts");
    assert_eq!(first["vouchers"], 3);
    assert_eq!(first["ledger"], "Purchase - Goods");
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
            "no_creditor_party"
        ]
    );
    assert_eq!(
        result["set_aside"]["no_creditor_party"]["listed"]["total"],
        1
    );
}

#[test]
fn under_mask_parties_a_ledger_and_a_parent_the_book_named_are_masked_and_a_predefined_parent_is_not(
) {
    let mut index = captured_index();
    let totals = fold(&index, &captured_rows()).unwrap();
    let masked = gst_by_ledger_result(&index, &totals, Redaction::MaskParties);
    let goods = &masked["counter_ledgers"][0];
    assert_ne!(goods["ledger"], "Purchase - Goods");
    assert_eq!(goods["parent"], "Purchase Accounts");
    index.by_name.get_mut("Purchase - Goods").unwrap().parent =
        Some("SYN Own Purchases Group".to_string());
    let masked = gst_by_ledger_result(&index, &totals, Redaction::MaskParties);
    assert_ne!(
        masked["counter_ledgers"][0]["parent"],
        "SYN Own Purchases Group"
    );
    assert_eq!(masked["counter_ledgers"][0]["group"], "Purchase Accounts");
}
