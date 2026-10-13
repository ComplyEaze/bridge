//! Where a parse can be tied to the book: the statement's balance at the
//! edges of a window of whole calendar months, worked out from its own rows by
//! date. The statement is synthetic and spans three months, with rows on the
//! first and last day of a month, so an edge counted on the wrong side moves a
//! figure.

mod common;

use bridge_bank_statement::bank::Bank;
use bridge_bank_statement::cash::CashAnswers;
use bridge_bank_statement::date::Date;
use bridge_bank_statement::mapping::Mapping;
use bridge_bank_statement::parse::Row;
use bridge_bank_statement::proposals::{
    build, format_amount, tie_out_window, BuildOptions, TieOutWindowRefusal,
};
use bridge_tally_primitives::ExactDecimal;
use common::*;

fn upi(date: &str, name: &str, dr: &str, cr: &str, bal: &str) -> Row {
    row(&[
        ("date", date),
        ("narr", &format!("UPI-{name}-9@x-ABCD0001-111111111111-P")),
        ("ref", "1"),
        ("dr", dr),
        ("cr", cr),
        ("bal", bal),
    ])
}

/// Opens at 1000.00 on 1 June 2026 and closes at 1240.25 on 31 August.
/// June nets +100.00 (to 1100.00), July +334.75 (to 1434.75), August -194.50.
fn rows() -> Vec<Row> {
    vec![
        upi("01/06/26", "ALPHA", "100.00", "", "900.00"),
        upi("15/06/26", "BRAVO", "", "250.00", "1150.00"),
        upi("30/06/26", "CHARLIE", "50.00", "", "1100.00"),
        upi("01/07/26", "DELTA", "", "400.00", "1500.00"),
        upi("20/07/26", "ECHO", "75.25", "", "1424.75"),
        upi("31/07/26", "FOXTROT", "", "10.00", "1434.75"),
        upi("03/08/26", "GOLF", "200.00", "", "1234.75"),
        upi("31/08/26", "HOTEL", "", "5.50", "1240.25"),
    ]
}

fn day(year: u16, month: u8, day: u8) -> Option<Date> {
    Some(Date::new(year, month, day).unwrap())
}

/// The window's dates and edge balances, or why it cannot be tied.
fn window_of(
    rows: &[Row],
    from: Option<Date>,
    to: Option<Date>,
) -> Result<(String, String, String, String), TieOutWindowRefusal> {
    let build = build(
        rows,
        Bank::Hdfc,
        &Mapping::default(),
        &BuildOptions {
            bank_ledger: "Bank",
            suspense_ledger: "Suspense",
            account_label: "ACC",
            account_number: "00000000001234",
            date_from: from,
            date_to: to,
            cash_answers: CashAnswers::none(),
        },
    )
    .unwrap();
    let opening = ExactDecimal::parse("1000.00").unwrap();
    let closing = ExactDecimal::parse("1240.25").unwrap();
    tie_out_window(&build, &opening, &closing, from, to)
        .expect("a build with rows has a window")
        .expect("in range")
        .map(|edges| {
            (
                edges.first().iso(),
                edges.last().iso(),
                format_amount(edges.opening()),
                format_amount(edges.closing()),
            )
        })
}

fn established(
    first: &str,
    last: &str,
    opening: &str,
    closing: &str,
) -> Result<(String, String, String, String), TieOutWindowRefusal> {
    Ok((first.into(), last.into(), opening.into(), closing.into()))
}

#[test]
fn the_whole_statement_ties_at_its_row_dates_and_the_balances_supplied() {
    assert_eq!(
        window_of(&rows(), None, None),
        established("2026-06-01", "2026-08-31", "1000.00", "1240.25")
    );
    // A window that leaves no row out is the whole statement, whatever its days.
    assert_eq!(
        window_of(&rows(), day(2026, 5, 20), day(2026, 9, 3)),
        established("2026-06-01", "2026-08-31", "1000.00", "1240.25")
    );
}

#[test]
fn each_month_ties_at_the_statements_own_balance_at_its_edges() {
    assert_eq!(
        window_of(&rows(), day(2026, 6, 1), day(2026, 6, 30)),
        established("2026-06-01", "2026-06-30", "1000.00", "1100.00")
    );
    // Rows on 1 July and 31 July are inside July, and only inside it.
    assert_eq!(
        window_of(&rows(), day(2026, 7, 1), day(2026, 7, 31)),
        established("2026-07-01", "2026-07-31", "1100.00", "1434.75")
    );
    assert_eq!(
        window_of(&rows(), day(2026, 8, 1), day(2026, 8, 31)),
        established("2026-08-01", "2026-08-31", "1434.75", "1240.25")
    );
    assert_eq!(
        window_of(&rows(), day(2026, 6, 1), day(2026, 7, 31)),
        established("2026-06-01", "2026-07-31", "1000.00", "1434.75")
    );
    // A from alone runs to the last row, here the last day of a month.
    assert_eq!(
        window_of(&rows(), day(2026, 7, 1), None),
        established("2026-07-01", "2026-08-31", "1100.00", "1240.25")
    );
}

#[test]
fn a_window_that_is_not_whole_calendar_months_is_refused_before_any_figure() {
    for (from, to) in [
        // one day holding one row
        (day(2026, 7, 20), day(2026, 7, 20)),
        // a month less its first day, and less its last
        (day(2026, 7, 2), day(2026, 7, 31)),
        (day(2026, 7, 1), day(2026, 7, 30)),
        // mid-month to mid-month
        (day(2026, 6, 15), day(2026, 7, 14)),
        // a to alone that is not a month's last day
        (None, day(2026, 7, 15)),
        // a from alone that is not the 1st
        (day(2026, 6, 2), None),
    ] {
        assert_eq!(
            window_of(&rows(), from, to),
            Err(TieOutWindowRefusal::NotWholeCalendarMonths),
            "{from:?} to {to:?}"
        );
    }
}

#[test]
fn a_month_before_the_first_row_or_after_the_last_is_not_established() {
    // May has no row; the statement does not show its balance on 1 May.
    assert_eq!(
        window_of(&rows(), day(2026, 5, 1), day(2026, 6, 30)),
        Err(TieOutWindowRefusal::BeyondStatementRows)
    );
    let mut ends_mid_month = rows();
    ends_mid_month.pop();
    assert_eq!(
        window_of(&ends_mid_month, day(2026, 8, 1), day(2026, 8, 31)),
        Err(TieOutWindowRefusal::BeyondStatementRows)
    );
    assert_eq!(
        TieOutWindowRefusal::BeyondStatementRows.code(),
        "window_beyond_statement_rows"
    );
    assert_eq!(
        TieOutWindowRefusal::NotWholeCalendarMonths.code(),
        "window_not_whole_calendar_months"
    );
}

#[test]
fn the_order_the_bank_printed_the_rows_in_does_not_move_the_edges() {
    // A June row printed among the July rows, as a back-dated entry would be.
    let mut reordered = rows();
    let late = reordered.remove(1);
    reordered.insert(4, late);
    for (from, to) in [
        (day(2026, 6, 1), day(2026, 6, 30)),
        (day(2026, 7, 1), day(2026, 7, 31)),
    ] {
        assert_eq!(
            window_of(&reordered, from, to),
            window_of(&rows(), from, to)
        );
    }
}
