//! The Bills reports of the lab's foreign-currency book as TallyPrime 7.2 sent them (#2071). A
//! dollar bill settled in full at another rate is sent with an empty `BILLCL`: the customer's
//! sales bill in the payable report, and the supplier's purchase bill in the receivable report.
//! Such a party is set aside by the ledger snapshot, so its rows are dropped before any closing
//! amount is admitted; for a party that is not set aside, the empty amount still refuses. The
//! rupee control party's bill is read as before.
use super::*;
use crate::native_outstandings::{
    compute_native_outstandings_with_exclusions, AgeingAnchor, BaseCurrencyName, BillClosing,
    NativeGroupSnapshot, NativeMasterSnapshot, NativeOverdueCrosscheck,
};
use bridge_tally_primitives::TallyDate;

const COMPANY: &str = "1bb48be5-baba-45b2-90d8-7593244915a9";
const CUSTOMER: &str = "BRIDGE USD Customer";
const SUPPLIER: &str = "BRIDGE USD Supplier";
const RUPEE_PARTY: &str = "BRIDGE INR Customer";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The same 476 bytes in all four reads over two sittings: the customer's bill 2, settled in dollars.
fn payable() -> String {
    decode(include_bytes!(
        "../../tests/fixtures/native/bills_payable_forex_settled_empty_closing.utf16le.xml"
    ))
}

/// After the rupee party's part-settled bill R1 was keyed: R1 and the customer's open bill 1.
fn receivable_rupee_control() -> String {
    decode(include_bytes!(
        "../../tests/fixtures/native/bills_receivable_forex_rupee_control.utf16le.xml"
    ))
}

/// After the dollar purchases and payments: R1, bill 1, and the supplier's P1, paid in full at
/// another rate and sent with an empty `BILLCL`.
fn receivable_settled_supplier() -> String {
    decode(include_bytes!(
        "../../tests/fixtures/native/bills_receivable_forex_settled_supplier.utf16le.xml"
    ))
}

fn date(value: &str) -> TallyDate {
    TallyDate::parse(value).unwrap()
}

/// The window the lab's `outstandings` read sent: books from 1 April 2025, as of 30 April 2025.
fn rows(xml: &str) -> UnadmittedBillRows {
    parse_native_bill_rows_unadmitted(xml, &date("20250401"), &date("20250430"))
        .expect("every row reads apart from its closing amount")
}

/// The base of the lab book, identified from the lab's own reads on 7.2.
fn rupee() -> BaseCurrencyName {
    let masters = crate::native_outstandings::parse_currency_master_list(&decode(include_bytes!(
        "../../tests/fixtures/native/currency_masters_originalname_forex_72.utf16le.xml"
    )))
    .expect("the captured masters");
    let company = crate::native_outstandings::parse_company_currency_name(
        &decode(include_bytes!(
            "../../tests/fixtures/native/company_base_currency_forex_72.utf16le.xml"
        )),
        COMPANY,
    )
    .expect("the captured company read");
    masters
        .identify_base(Some(&company))
        .expect("the base is identified")
        .base()
        .clone()
}

/// The ledger snapshot read beside `receivable_rupee_control`, classified.
fn rupee_control_snapshot() -> ClassifiedLedgerSnapshot {
    let snapshot = decode(include_bytes!(
        "../../tests/fixtures/native/ledger_snapshot_forex_rupee_control.utf16le.xml"
    ));
    parse_native_ledger_snapshot_classified(&snapshot, &rupee()).expect("the snapshot classifies")
}

fn is_invalid_amount_in_row(error: &NativeOutstandingsError, expected: u32) -> bool {
    matches!(
        error,
        NativeOutstandingsError::BillRow { row, cause, .. }
            if *row == expected && **cause == NativeOutstandingsError::InvalidAmount
    )
}

#[test]
fn a_settled_bill_is_read_with_an_empty_closing_amount_and_nothing_else_deferred() {
    let payable = rows(&payable()).0;
    assert_eq!(payable.len(), 1);
    assert_eq!(payable[0].row, 1);
    assert_eq!(payable[0].party, CUSTOMER);
    assert_eq!(payable[0].reference, "2");
    assert_eq!(payable[0].closing, BillClosing::Empty);
    assert_eq!(payable[0].bill_date, date("20250401"));
    assert_eq!(payable[0].due_date, date("20250401"));
    assert_eq!(payable[0].tally_overdue_days, Some(29));

    let receivable = rows(&receivable_settled_supplier()).0;
    let read = receivable
        .iter()
        .map(|row| {
            (
                row.row,
                row.party.as_str(),
                row.reference.as_str(),
                matches!(row.closing, BillClosing::Amount(_)),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        read,
        [
            (1, RUPEE_PARTY, "R1", true),
            (2, CUSTOMER, "1", true),
            (3, SUPPLIER, "P1", false),
        ]
    );
    assert_eq!(receivable[2].closing, BillClosing::Empty);
}

/// The strict parse every other caller uses refuses as the lab's tool answers did:
/// `native_amount_invalid` at payable row 1, and at receivable row 3 once P1 was settled.
#[test]
fn an_empty_closing_amount_still_refuses_when_its_party_is_not_set_aside() {
    let error = rows(&payable()).admit_all().unwrap_err();
    assert!(is_invalid_amount_in_row(&error, 1), "{error:?}");
    let error =
        parse_native_bill_rows(&payable(), &date("20250401"), &date("20250430")).unwrap_err();
    assert!(is_invalid_amount_in_row(&error, 1), "{error:?}");
    assert_eq!(error.code(), "native_amount_invalid");
    assert_eq!(error.bill_row(), Some((None, 1)));

    let error = parse_native_bill_rows(
        &receivable_settled_supplier(),
        &date("20250401"),
        &date("20250430"),
    )
    .unwrap_err();
    assert!(is_invalid_amount_in_row(&error, 3), "{error:?}");
    // Setting aside the customer alone does not cover the supplier's row.
    let error = rows(&receivable_settled_supplier())
        .admit_except(|party| party == CUSTOMER)
        .unwrap_err();
    assert!(is_invalid_amount_in_row(&error, 3), "{error:?}");
}

/// The book with the rupee control party, read end to end below the runtime: the snapshot sets
/// the dollar customer aside, its rows are dropped, and the rupee bill is the whole report.
#[test]
fn the_rupee_control_book_reads_its_rupee_bill_and_sets_the_dollar_party_aside() {
    let snapshot = rupee_control_snapshot();
    assert!(snapshot.foreign.is_empty());
    assert!(snapshot.mixed.iter().any(|name| name == CUSTOMER));
    let base = snapshot
        .base
        .iter()
        .map(|entry| entry.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(base, ["BRIDGE INR Customer", "Cash"]);

    let set_aside = |party: &str| snapshot.mixed.iter().any(|name| name == party);
    let receivable = rows(&receivable_rupee_control())
        .admit_except(set_aside)
        .unwrap();
    let payable = rows(&payable()).admit_except(set_aside).unwrap();
    assert_eq!(payable, []);
    assert_eq!(receivable.len(), 1);
    assert_eq!(receivable[0].party, RUPEE_PARTY);
    assert_eq!(receivable[0].reference, "R1");

    let groups = parse_native_group_snapshot(
        &decode(include_bytes!(
            "../../tests/fixtures/native/group_snapshot_forexgroup_lab.utf16le.xml"
        )),
        COMPANY,
    )
    .expect("the captured groups");
    let result = compute_native_outstandings_with_exclusions(
        "BRIDGE FOREXGROUP LAB",
        &receivable,
        &payable,
        NativeMasterSnapshot {
            ledgers: &snapshot.base,
            groups: NativeGroupSnapshot::Complete(&groups),
        },
        &snapshot.foreign,
        &snapshot.mixed,
        AgeingAnchor::DueDate,
        &date("20250430"),
        0,
    )
    .expect("the base-currency bills are computed");
    // The total is the rupee bill's own amount (compared by value: the two are written to
    // different scales).
    assert!(result
        .report
        .receivable_total
        .checked_subtract(&receivable[0].closing_balance.magnitude())
        .unwrap()
        .is_zero());
    assert!(result.report.payable_total.is_zero());
    assert_eq!(result.mixed_currency_ledgers_excluded, snapshot.mixed);
    assert_eq!(result.overdue_crosscheck, NativeOverdueCrosscheck::Honored);
}

/// After the dollar purchases, the supplier's settled bill is dropped with its party; the
/// remaining rows are the rupee bill. That book's own snapshot is not classified here: its
/// Profit & Loss balance carries a rate with decimals, which no detector admits yet (#2071).
#[test]
fn the_settled_suppliers_row_is_dropped_with_its_party() {
    let receivable = rows(&receivable_settled_supplier())
        .admit_except(|party| party == CUSTOMER || party == SUPPLIER)
        .unwrap();
    assert_eq!(receivable.len(), 1);
    assert_eq!(receivable[0].party, RUPEE_PARTY);
}

/// A labelled edit of the capture: a closing amount that is present but not a decimal still
/// refuses while the rows are read, before any party is set aside. Only an empty one waits.
#[test]
fn a_closing_amount_that_is_not_a_decimal_still_refuses_at_the_parse() {
    let edited = payable().replace("<BILLCL></BILLCL>", "<BILLCL>-</BILLCL>");
    assert_ne!(edited, payable());
    let error = parse_native_bill_rows_unadmitted(&edited, &date("20250401"), &date("20250430"))
        .unwrap_err();
    assert!(is_invalid_amount_in_row(&error, 1), "{error:?}");
}
