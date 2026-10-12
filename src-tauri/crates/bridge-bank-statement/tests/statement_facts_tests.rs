//! What a statement build records for the transfer pairing view: which way
//! each row moved money, whether the statement reversed it the same day, and
//! the days the statement read in full.

mod common;

use bridge_bank_statement::bank::Bank;
use bridge_bank_statement::cash::CashAnswers;
use bridge_bank_statement::date::Date;
use bridge_bank_statement::mapping::{Mapping, MappingRow};
use bridge_bank_statement::parse::Row;
use bridge_bank_statement::proposals::{
    build, coverage, BuildOptions, Direction, Disposition, VoucherType,
};
use common::*;

fn options<'a>() -> BuildOptions<'a> {
    BuildOptions {
        bank_ledger: "Bank",
        suspense_ledger: "SUSPENSE ACC",
        account_label: "ACC",
        account_number: "00000000001234",
        date_from: None,
        date_to: None,
        cash_answers: CashAnswers::none(),
    }
}

fn mapping(rows: &[(&str, &str, &str)]) -> Mapping {
    Mapping::from_rows(
        rows.iter()
            .enumerate()
            .map(|(index, (party, ledger, treatment))| MappingRow {
                origin: format!("row {index}"),
                party: party.to_string(),
                ledger: ledger.to_string(),
                treatment: Some(treatment.to_string()),
            }),
    )
    .unwrap()
}

fn upi(date: &str, name: &str, reference: &str, dr: &str, cr: &str, bal: &str) -> Row {
    row(&[
        ("date", date),
        ("narr", &format!("UPI-{name}-9@x-ABCD0001-{reference}-P")),
        ("ref", "1"),
        ("dr", dr),
        ("cr", cr),
        ("bal", bal),
    ])
}

fn iso(text: &str) -> Date {
    Date::parse_iso(text).unwrap()
}

#[test]
fn a_record_names_which_way_money_moved_on_the_statements_own_account() {
    let rows = [
        upi("01/08/26", "ALPHA", "111111111111", "10.00", "", "990.00"),
        upi("02/08/26", "BETA", "222222222222", "", "20.00", "1010.00"),
        upi(
            "03/08/26",
            "OWN ACCT",
            "333333333333",
            "30.00",
            "",
            "980.00",
        ),
        upi("04/08/26", "OWN IN", "444444444444", "", "40.00", "1020.00"),
    ];
    let built = build(
        &rows,
        Bank::Hdfc,
        &mapping(&[
            ("ALPHA", "Alpha Ledger", "contra"),
            ("OWN ACCT", "Other Bank", "skip"),
            ("OWN IN", "Other Bank", "skip"),
        ]),
        &options(),
    )
    .unwrap();
    let seen: Vec<_> = built
        .records
        .iter()
        .map(|record| (record.disposition, record.direction))
        .collect();
    assert_eq!(
        seen,
        [
            (
                Disposition::Voucher(VoucherType::Contra),
                Direction::Outflow
            ),
            (
                Disposition::Voucher(VoucherType::Receipt),
                Direction::Inflow
            ),
            (Disposition::Skipped, Direction::Outflow),
            (Disposition::Skipped, Direction::Inflow),
        ]
    );
    assert_eq!(
        serde_json::to_value(Direction::Outflow).unwrap(),
        serde_json::json!("outflow")
    );
    assert_eq!(
        serde_json::to_value(Direction::Inflow).unwrap(),
        serde_json::json!("inflow")
    );
}

fn reversed_flags(rows: &[Row], window: impl FnOnce(&mut BuildOptions<'_>)) -> Vec<bool> {
    let mut opts = options();
    window(&mut opts);
    build(rows, Bank::Hdfc, &Mapping::default(), &opts)
        .unwrap()
        .records
        .iter()
        .map(|record| record.reversed_same_day)
        .collect()
}

#[test]
fn a_line_reversed_the_same_day_is_flagged_on_both_rows() {
    let rows = [
        // out and back the same day, equal amount: both flagged
        upi("01/08/26", "PAID", "111111111111", "10.00", "", "990.00"),
        upi("01/08/26", "BACK", "222222222222", "", "10.00", "1000.00"),
        // the amount differs: not a reversal
        upi("02/08/26", "OUT", "333333333333", "5.00", "", "995.00"),
        upi("02/08/26", "IN", "444444444444", "", "6.00", "1001.00"),
        // the next day: not the same day
        upi("03/08/26", "OUT", "555555555555", "7.00", "", "994.00"),
        upi("04/08/26", "IN", "666666666666", "", "7.00", "1001.00"),
        // two payments out, equal: the same direction is not a reversal
        upi("05/08/26", "TWICE", "777777777777", "8.00", "", "993.00"),
        upi("05/08/26", "TWICE", "888888888888", "8.00", "", "985.00"),
        // equal amounts that differ in notation only are equal
        upi("06/08/26", "ODD", "999999999999", "9.5", "", "975.50"),
        upi("06/08/26", "ODD", "101010101010", "", "9.50", "985.00"),
    ];
    assert_eq!(
        reversed_flags(&rows, |_| {}),
        [true, true, false, false, false, false, false, false, true, true]
    );
}

#[test]
fn a_reversal_is_read_over_every_row_not_only_the_window() {
    let rows = [
        upi("01/08/26", "PAID", "111111111111", "10.00", "", "990.00"),
        upi("01/08/26", "BACK", "222222222222", "", "10.00", "1000.00"),
        upi("05/08/26", "LATER", "333333333333", "4.00", "", "996.00"),
        upi("05/08/26", "LATER", "444444444444", "", "4.00", "1000.00"),
    ];
    // a window holding only the second pair flags it as the whole statement does
    assert_eq!(
        reversed_flags(&rows, |opts| opts.date_from = Date::new(2026, 8, 5)),
        [true, true]
    );
    assert_eq!(reversed_flags(&rows, |_| {}), [true, true, true, true]);
}

#[test]
fn coverage_is_the_days_whose_every_row_was_read() {
    let rows = [
        iso("2026-08-03"),
        iso("2026-08-01"),
        iso("2026-08-10"),
        iso("2026-08-07"),
    ];
    // no window: the first and last days may be cut short, so they are left out
    assert_eq!(
        coverage(&rows, None, None),
        Some((iso("2026-08-02"), iso("2026-08-09")))
    );
    // a window inside the rows: rows exist beyond both ends, so its days count
    assert_eq!(
        coverage(&rows, Some(iso("2026-08-03")), Some(iso("2026-08-07"))),
        Some((iso("2026-08-03"), iso("2026-08-07")))
    );
    // a window reaching the statement's own first or last day does not
    assert_eq!(
        coverage(&rows, Some(iso("2026-08-01")), Some(iso("2026-08-10"))),
        Some((iso("2026-08-02"), iso("2026-08-09")))
    );
    assert_eq!(
        coverage(&rows, Some(iso("2026-07-01")), Some(iso("2026-09-01"))),
        Some((iso("2026-08-02"), iso("2026-08-09")))
    );
    // one end given, the other left to the rows
    assert_eq!(
        coverage(&rows, Some(iso("2026-08-05")), None),
        Some((iso("2026-08-05"), iso("2026-08-09")))
    );
    assert_eq!(
        coverage(&rows, None, Some(iso("2026-08-05"))),
        Some((iso("2026-08-02"), iso("2026-08-05")))
    );
}

#[test]
fn coverage_is_none_when_no_day_was_read_in_full() {
    assert_eq!(coverage(&[], None, None), None);
    // every row on one day, or on two adjacent days
    assert_eq!(coverage(&[iso("2026-08-01")], None, None), None);
    assert_eq!(
        coverage(&[iso("2026-08-01"), iso("2026-08-01")], None, None),
        None
    );
    assert_eq!(
        coverage(&[iso("2026-08-01"), iso("2026-08-02")], None, None),
        None
    );
    // a window that lies past the rows, or ends before it starts
    let rows = [iso("2026-08-01"), iso("2026-08-10")];
    assert_eq!(
        coverage(&rows, Some(iso("2026-08-20")), Some(iso("2026-08-25"))),
        None
    );
    assert_eq!(
        coverage(&rows, Some(iso("2026-08-06")), Some(iso("2026-08-04"))),
        None
    );
    // three days apart leaves exactly the middle one
    assert_eq!(
        coverage(&[iso("2026-08-01"), iso("2026-08-03")], None, None),
        Some((iso("2026-08-02"), iso("2026-08-02")))
    );
}

#[test]
fn coverage_steps_across_month_and_year_ends() {
    let rows = [iso("2026-12-31"), iso("2027-01-02"), iso("2026-03-01")];
    assert_eq!(
        coverage(&rows, None, None),
        Some((iso("2026-03-02"), iso("2027-01-01")))
    );
    let rows = [iso("2028-02-28"), iso("2028-03-01")];
    assert_eq!(
        coverage(&rows, None, None),
        Some((iso("2028-02-29"), iso("2028-02-29")))
    );
    let rows = [iso("2028-02-29"), iso("2028-03-01")];
    assert_eq!(coverage(&rows, None, None), None);
    let rows = [iso("2028-02-28"), iso("2028-03-02")];
    assert_eq!(
        coverage(&rows, None, None),
        Some((iso("2028-02-29"), iso("2028-03-01")))
    );
    let rows = [iso("2026-01-31"), iso("2026-03-01")];
    assert_eq!(
        coverage(&rows, None, None),
        Some((iso("2026-02-01"), iso("2026-02-28")))
    );
}

#[test]
fn a_build_keeps_the_date_of_every_row_it_read() {
    let rows = [
        upi("01/08/26", "A", "111111111111", "10.00", "", "990.00"),
        upi("09/08/26", "B", "222222222222", "10.00", "", "980.00"),
    ];
    let mut window = options();
    window.date_from = Date::new(2026, 8, 5);
    let built = build(&rows, Bank::Hdfc, &Mapping::default(), &window).unwrap();
    assert_eq!(built.records.len(), 1);
    assert_eq!(built.dates, [iso("2026-08-01"), iso("2026-08-09")]);
}
