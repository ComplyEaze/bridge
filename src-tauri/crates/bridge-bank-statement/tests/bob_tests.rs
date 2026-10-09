//! The Bank of Baroda layout: columns, one visual line per amount, the newest row first.
//!
//! There is no Python reference for this profile. Its geometry and shapes were
//! measured on two real statements, held privately; every line here is synthetic, laid out at the measured
//! column positions.

mod common;

use bridge_bank_statement::bank::{Bank, UNRESOLVED};
use bridge_bank_statement::cash::{CashAnswers, CashMovement, CASH_DEPOSIT};
use bridge_bank_statement::date::{parse_day_month_year_slashed, Date};
use bridge_bank_statement::geometry::{Page, Word};
use bridge_bank_statement::mapping::{Mapping, MappingRow};
use bridge_bank_statement::money::Controls;
use bridge_bank_statement::parse::{parse_statement, require_account_match, Row};
use bridge_bank_statement::pipeline::{prepare, StatementRequest};
use bridge_bank_statement::proposals::{format_amount, Disposition};
use bridge_bank_statement::Refusal;
use common::*;

const CHAR: f64 = 4.2;
const BANK_CHARGES: &str = "BANK CHARGES";

/// A text laid out as monospaced words from `x`, split at spaces.
fn put(words: &mut Vec<Word>, x: f64, y: f64, text: &str) {
    let mut at = x;
    for token in text.split(' ').filter(|token| !token.is_empty()) {
        let width = CHAR * token.chars().count() as f64;
        words.push(Word::new(at, y, at + width, y + 7.0, token));
        at += width + CHAR;
    }
}

/// A text whose last glyph ends at `edge`.
fn put_ending(words: &mut Vec<Word>, edge: f64, y: f64, text: &str) {
    put(words, edge - CHAR * text.chars().count() as f64, y, text);
}

/// One printed row. `narration` is the printed fragments, `dy` the amount line's
/// distance below the date line.
struct Printed {
    date: &'static str,
    narration: &'static [&'static str],
    debit: &'static str,
    credit: &'static str,
    balance: &'static str,
    dy: f64,
    cheque: &'static str,
}

const fn printed(
    date: &'static str,
    narration: &'static [&'static str],
    debit: &'static str,
    credit: &'static str,
    balance: &'static str,
    dy: f64,
) -> Printed {
    Printed {
        date,
        narration,
        debit,
        credit,
        balance,
        dy,
        cheque: "",
    }
}

// newest first, as printed; opening balance 10,000.00 Cr
const SMS: Printed = printed(
    "07/08/2026",
    &["SMS Charges for AUG 26"],
    "11.80",
    "",
    "11,635.26Dr",
    0.0,
);
const LOAN: Printed = printed(
    "06/08/2026",
    &["Loan Recovery For00000000000001"],
    "30,000.00",
    "",
    "11,623.46Dr",
    0.0,
);
const ACME: Printed = printed(
    "05/08/2026",
    &["NEFT-ZZZZZ00000000001-ACME", "INDUSTRIES-EAST"],
    "2,250.50",
    "",
    "18,376.54Cr",
    6.0,
);
const UPI: Printed = printed(
    "03/08/2026",
    &[
        // 50 characters: ends exactly at the narration cell's wrap edge, mid-token
        "UPI/600000000002/11:20:45/UPI/northwind.traders.sy",
        "nthetic@okzzzzzz/SYNTH",
    ],
    "",
    "9,876.54",
    "20,627.04Cr",
    6.0,
);
const CASH: Printed = printed("03/08/2026", &["BY CASH"], "", "750.50", "10,750.50Cr", 0.0);

fn table_header(words: &mut Vec<Word>) {
    for (x, text) in [
        (10.0, "TRAN"),
        (41.0, "DATE"),
        (82.0, "VALUE"),
        (119.0, "DATE"),
        (160.0, "NARRATION"),
        (365.0, "CHQ.NO."),
        (457.0, "WITHDRAWAL(DR)"),
        (602.0, "DEPOSIT(CR)"),
        (719.0, "BALANCE(INR)"),
    ] {
        put(words, x, 324.0, text);
    }
}

/// The identity line of the measured statements: the label, the code, and a right-hand
/// label pair on the same visual line (all values invented).
fn put_identity(words: &mut Vec<Word>) {
    for (x, text) in [
        (10.0, "IFSC"),
        (38.0, "Code:"),
        (110.0, IFSC_VALUE),
        (485.0, "Product"),
        (534.0, "Cur:"),
        (600.0, "IN"),
    ] {
        put(words, x, 211.0, text);
    }
}

/// A page: the page-1 header block when `header` is given, the rows, and a footer.
fn bob_page(header: Option<&[&str]>, rows: &[Printed], footer: Option<&str>) -> Page {
    let mut words = Vec::new();
    let mut top = 68.0;
    if let Some(tokens) = header {
        put(
            &mut words,
            10.0,
            71.0,
            "Your Account Statement as on 09/10/2026",
        );
        for (index, token) in tokens.iter().enumerate() {
            put(&mut words, 640.0, 90.0 + 12.0 * index as f64, token);
        }
        put(
            &mut words,
            10.0,
            268.0,
            "Statement of transactions in Current Account in INR",
        );
        put_identity(&mut words);
        table_header(&mut words);
        top = 342.0;
    }
    for row in rows {
        put(&mut words, 15.0, top, row.date);
        put(&mut words, 87.0, top, row.date);
        put(&mut words, 149.5, top, row.narration[0]);
        put_ending(&mut words, 787.14, top, row.balance);
        if !row.cheque.is_empty() {
            put(&mut words, 415.5, top, row.cheque);
        }
        for (text, edge) in [(row.debit, 543.0), (row.credit, 662.0)] {
            if !text.is_empty() {
                put_ending(&mut words, edge, top + row.dy, text);
            }
        }
        for (index, fragment) in row.narration.iter().enumerate().skip(1) {
            put(&mut words, 149.5, top + 11.0 * index as f64, fragment);
        }
        top += 18.0 + 11.0 * (row.narration.len() - 1) as f64;
    }
    if let Some(text) = footer {
        put(&mut words, 10.0, 1165.0, text);
        put(
            &mut words,
            10.0,
            1185.0,
            "*This is computer-generated statement.No signature is required.",
        );
    }
    words
}

const MASKED: &[&str] = &["123XXXXXXXX456"];
/// An invented code with the bank's prefix.
const IFSC_VALUE: &str = "BARB0SYNTH1";
const FOOTER_ONE: &str = "09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 2";
const FOOTER_TWO: &str = "09/10/2026 10:30 SYNTH-ID/00000 Page 2 of 2";

fn pages() -> Vec<Page> {
    vec![
        bob_page(Some(MASKED), &[SMS, LOAN, ACME], Some(FOOTER_ONE)),
        bob_page(None, &[UPI, CASH], Some(FOOTER_TWO)),
    ]
}

fn rows() -> Vec<Row> {
    parse_statement(&pages(), Bank::Bob).unwrap()
}

#[test]
fn the_bank_is_named_bob() {
    assert_eq!(Bank::from_name("bob"), Some(Bank::Bob));
    assert_eq!(Bank::Bob.name(), "bob");
    assert!(!Bank::Bob.prints_totals());
}

#[test]
fn rows_read_oldest_first_with_signed_balances_and_the_amount_on_its_own_line() {
    let rows = rows();
    let cells: Vec<[&str; 6]> = rows
        .iter()
        .map(|row| {
            [
                row.get("date"),
                row.get("vdt"),
                row.get("narr"),
                row.get("dr"),
                row.get("cr"),
                row.get("bal"),
            ]
        })
        .collect();
    assert_eq!(
        cells,
        [
            [
                "03/08/2026",
                "03/08/2026",
                "BY CASH",
                "",
                "750.50",
                "10750.50"
            ],
            [
                "03/08/2026",
                "03/08/2026",
                // the first fragment ends at the wrap edge: the break was inside the token
                "UPI/600000000002/11:20:45/UPI/northwind.traders.synthetic@okzzzzzz/SYNTH",
                "",
                "9876.54",
                "20627.04"
            ],
            [
                "05/08/2026",
                "05/08/2026",
                // the first fragment ended well short of it: the break was at a space
                "NEFT-ZZZZZ00000000001-ACME INDUSTRIES-EAST",
                "2250.50",
                "",
                "18376.54"
            ],
            [
                "06/08/2026",
                "06/08/2026",
                "Loan Recovery For00000000000001",
                "30000.00",
                "",
                "-11623.46"
            ],
            [
                "07/08/2026",
                "07/08/2026",
                "SMS Charges for AUG 26",
                "11.80",
                "",
                "-11635.26"
            ],
        ]
    );
    // a spaced reading keeps the words a wrap would weld
    assert_eq!(
        rows[1].get("narr_spaced"),
        "UPI/600000000002/11:20:45/UPI/northwind.traders.sy nthetic@okzzzzzz/SYNTH"
    );
}

#[test]
fn a_cheque_number_stays_out_of_the_narration_and_the_amounts() {
    let mut cheque = SMS;
    cheque.cheque = "000417";
    let page = bob_page(
        Some(MASKED),
        &[cheque],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    );
    let rows = parse_statement(&[page], Bank::Bob).unwrap();
    assert_eq!(rows[0].get("chq"), "000417");
    assert_eq!(rows[0].get("narr"), "SMS Charges for AUG 26");
    assert_eq!(rows[0].get("dr"), "11.80");
    assert_eq!(rows[0].get("cr"), "");
}

#[test]
fn a_narration_word_that_reaches_the_cell_edge_stays_in_the_narration() {
    // 50 characters, the most the bank prints: the last word ends at x 359.5 and its
    // centre is at 353, inside the narration column and a hair short of the cheque zone
    let edge = Printed {
        narration: &["PAYMENT ADVICE 0000000000000000000000000000000 123"],
        ..SMS
    };
    let page = bob_page(
        Some(MASKED),
        &[edge],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    );
    let rows = parse_statement(&[page], Bank::Bob).unwrap();
    assert_eq!(
        rows[0].get("narr"),
        "PAYMENT ADVICE 0000000000000000000000000000000 123"
    );
    assert_eq!(rows[0].get("chq"), "");
}

#[test]
fn the_header_block_and_footers_are_not_rows_and_do_not_wrap_into_one() {
    // the header block prints a date, and the footer a date and a time: neither opens a
    // row, and the last row's narration does not pick up the footer or the disclaimer
    let rows = rows();
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0].get("narr"), "BY CASH");
    assert_eq!(rows[4].get("narr"), "SMS Charges for AUG 26");
    assert_eq!(rows[4].get("bal"), "-11635.26");
}

#[test]
fn a_narration_that_says_page_n_of_m_is_not_a_footer() {
    let mut page_words = Printed {
        narration: &["INVOICE 7", "Page 1 of 2"],
        ..SMS
    };
    page_words.dy = 6.0;
    let pages = vec![
        bob_page(Some(MASKED), &[page_words, LOAN], Some(FOOTER_ONE)),
        bob_page(None, &[ACME], Some(FOOTER_TWO)),
    ];
    let rows = parse_statement(&pages, Bank::Bob).unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[2].get("narr"), "INVOICE 7 Page 1 of 2");
}

#[test]
fn a_later_page_must_start_with_a_row() {
    // every later page starts with a row; anything else is not read into the row above
    // it, so a banner, a note or a row split across the break refuses the statement
    for stray in [
        // the continuation of a narration, at the page top
        (149.5, "INDUSTRIES-EAST"),
        // a banner at the left margin
        (15.0, "STATEMENT CONTINUED"),
    ] {
        let mut first = bob_page(Some(MASKED), &[SMS, LOAN, carried_row()], Some(FOOTER_ONE));
        first.retain(|word| word.text != "INDUSTRIES-EAST");
        let mut second = bob_page(None, &[UPI, CASH], Some(FOOTER_TWO));
        put(&mut second, stray.0, 60.0, stray.1);
        let refusal = refuses(
            parse_statement(&[first, second], Bank::Bob),
            "unexpected_line_in_table",
        );
        assert_eq!(refusal.row, None);
    }
    // the same statement without the stray line is read
    let first = bob_page(Some(MASKED), &[SMS, LOAN, ACME], Some(FOOTER_ONE));
    let second = bob_page(None, &[UPI, CASH], Some(FOOTER_TWO));
    assert_eq!(
        parse_statement(&[first, second], Bank::Bob).unwrap().len(),
        5
    );
}

fn carried_row() -> Printed {
    Printed {
        narration: &["NEFT-ZZZZZ00000000001-ACME", "INDUSTRIES-EAST"],
        ..ACME
    }
}

#[test]
fn every_page_must_print_its_own_footer() {
    assert_eq!(parse_statement(&pages(), Bank::Bob).unwrap().len(), 5);

    // page 2 without a footer
    let mut no_footer = pages();
    no_footer[1] = bob_page(None, &[UPI, CASH], None);
    refuses(
        parse_statement(&no_footer, Bank::Bob),
        "page_sequence_unproven",
    );

    // a missing page: the remaining pages still say "of 2"
    let mut missing = pages();
    missing.remove(1);
    refuses(
        parse_statement(&missing, Bank::Bob),
        "page_sequence_unproven",
    );

    // out of order
    let mut swapped = pages();
    swapped.swap(0, 1);
    // (the identity line is on the first page only, so it is put back on the new first page)
    put_identity(&mut swapped[0]);
    refuses(
        parse_statement(&swapped, Bank::Bob),
        "page_sequence_unproven",
    );

    // a footer that counts the wrong number of pages
    let mut wrong_count = pages();
    wrong_count[1] = bob_page(
        None,
        &[UPI, CASH],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 2 of 3"),
    );
    refuses(
        parse_statement(&wrong_count, Bank::Bob),
        "page_sequence_unproven",
    );

    // a line that is only "Page 2 of 2" is not the footer
    let mut bare = pages();
    bare[1] = bob_page(None, &[UPI, CASH], Some("Page 2 of 2"));
    refuses(parse_statement(&bare, Bank::Bob), "page_sequence_unproven");
}

#[test]
fn rows_printed_oldest_first_are_refused() {
    let pages = vec![
        bob_page(Some(MASKED), &[CASH, UPI, ACME], Some(FOOTER_ONE)),
        bob_page(None, &[LOAN, SMS], Some(FOOTER_TWO)),
    ];
    let refusal = refuses(parse_statement(&pages, Bank::Bob), "rows_not_newest_first");
    // numbered in the reversed (oldest first) order: the second of five
    assert_eq!(refusal.row, Some(2));
    // rows of the same day are not out of order
    assert_eq!(rows()[0].get("date"), rows()[1].get("date"));
}

#[test]
fn a_balance_must_be_an_amount_with_its_marker() {
    for bad in [
        "10,750.50",
        "10,750.50cr",
        "10,750.50CR",
        "Cr",
        "10,750.5.0Cr",
        "",
    ] {
        let row = Printed {
            balance: bad,
            ..CASH
        };
        let page = bob_page(
            Some(MASKED),
            &[row],
            Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
        );
        let refusal = refuses(parse_statement(&[page], Bank::Bob), "malformed_balance");
        assert_eq!(refusal.row, Some(1), "{bad:?}");
    }
    // a zero balance marked Dr is zero, not negative zero
    let zero = Printed {
        balance: "0.00Dr",
        ..CASH
    };
    let page = bob_page(
        Some(MASKED),
        &[zero],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    );
    let rows = parse_statement(&[page], Bank::Bob).unwrap();
    assert_eq!(rows[0].get("bal"), "0.00");
}

#[test]
fn a_date_cell_that_is_not_a_date_is_refused() {
    let row = Printed {
        date: "31/02/2026",
        ..CASH
    };
    let page = bob_page(
        Some(MASKED),
        &[row],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    );
    let refusal = refuses(parse_statement(&[page], Bank::Bob), "unparseable_date");
    assert_eq!(refusal.row, Some(1));
}

#[test]
fn a_page_one_without_the_column_header_reads_no_rows_from_it() {
    // the table's start is the header: without it nothing is read and no later page is taken
    // for the table either; the statement is refused, never read in part
    let mut no_header = pages();
    no_header[0] = bob_page(None, &[SMS, LOAN, ACME], Some(FOOTER_ONE));
    put_identity(&mut no_header[0]);
    assert!(parse_statement(&no_header, Bank::Bob).unwrap().is_empty());
    // the header block is there but the column header's words are not: still no table
    let mut no_anchor = pages();
    no_anchor[0].retain(|word| !["TRAN", "VALUE", "NARRATION"].contains(&word.text.as_str()));
    assert!(parse_statement(&no_anchor, Bank::Bob).unwrap().is_empty());
    let mapping = no_mapping();
    let controls = Controls::parse_optional("10,000.00", "-11,635.26", None, None).unwrap();
    // no table means no header block above it either, so the account cannot be bound
    refuses(
        prepare(&no_header, &request(&controls, &mapping)),
        "no_account_number_line",
    );
}

#[test]
fn a_line_above_the_column_header_is_never_a_row() {
    // a header-block line with a date in each date column would be a row if the table
    // started anywhere but below its column header
    let mut page = bob_page(
        Some(MASKED),
        &[SMS, LOAN],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    );
    put(&mut page, 15.0, 200.0, "01/08/2026");
    put(&mut page, 87.0, 200.0, "31/08/2026");
    assert_eq!(parse_statement(&[page], Bank::Bob).unwrap().len(), 2);
}

#[test]
fn an_amount_eleven_points_below_a_three_line_narration_stays_on_its_row() {
    let mut charges = Printed {
        narration: &[
            "CHARGES FOR :IMPS/P2A/600000000003/XXXXXXXX",
            "0000",
            "0001",
        ],
        ..SMS
    };
    charges.dy = 11.0;
    let page = bob_page(
        Some(MASKED),
        &[charges, LOAN],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    );
    let rows = parse_statement(&[page], Bank::Bob).unwrap();
    assert_eq!(rows[1].get("dr"), "11.80");
    assert_eq!(
        rows[1].get("narr"),
        "CHARGES FOR :IMPS/P2A/600000000003/XXXXXXXX 0000 0001"
    );
    assert_eq!(Bank::Bob.party(&rows[1]), BANK_CHARGES);
    assert_eq!(rows[0].get("dr"), "30000.00");
}

#[test]
fn a_statement_whose_rows_share_one_date_printed_oldest_first_fails_the_replay() {
    // the dates cannot show the order, so the balance replay is what refuses it
    let page = bob_page(
        Some(MASKED),
        &[CASH, UPI],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    );
    let mapping = no_mapping();
    let controls = Controls::parse_optional("10,000.00", "20,627.04", None, None).unwrap();
    let refusal = refuses(
        prepare(&[page], &request(&controls, &mapping)),
        "balance_chain_broken",
    );
    assert_eq!(refusal.row, Some(1));
}

#[test]
fn a_row_needs_both_dates_and_a_stray_date_does_not_start_one() {
    let footer = "09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1";
    // a date alone in the transaction date column joins the row above it, which then
    // has no date a layout can produce: refused, not read as a row of its own
    let mut stray = bob_page(Some(MASKED), &[SMS, LOAN], Some(footer));
    put(&mut stray, 15.0, 400.0, "05/08/2026");
    let stray_rows = parse_statement(&[stray], Bank::Bob);
    refuses(stray_rows, "unparseable_date");

    // a date alone in the value date column joins that cell and starts nothing: the row
    // above it now has no value date a layout can produce
    let mut value_only = bob_page(Some(MASKED), &[SMS, LOAN], Some(footer));
    put(&mut value_only, 87.0, 400.0, "05/08/2026");
    let refusal = refuses(
        parse_statement(&[value_only], Bank::Bob),
        "unparseable_value_date",
    );
    assert_eq!(refusal.row, Some(1));
}

fn party_of(narration: &str) -> String {
    Bank::Bob.party(&row(&[("narr", narration), ("narr_spaced", narration)]))
}

#[test]
fn a_bob_party_is_named_only_where_the_shape_settles_it() {
    // UPI: the VPA, when a slash follows it
    assert_eq!(
        party_of("UPI/600000000001/10:15:30/UPI/nw@okzz/ZZZZ BANK"),
        "nw@okzz"
    );
    assert_eq!(
        party_of("UPI/600000000002/11:20:45/UPI/north_traders.x-1@okzzzzzz/S"),
        "north_traders.x-1@okzzzzzz"
    );
    // NEFT and RTGS: everything after the UTR, hyphens inside the name kept
    assert_eq!(
        party_of("NEFT-ZZZZZ00000000001-ACME INDUSTRIES-EAST"),
        "ACME INDUSTRIES-EAST"
    );
    assert_eq!(
        party_of("NEFT-ZZZZZ00000000001-ACME INDUSTRIES-"),
        "ACME INDUSTRIES"
    );
    // a hyphen with no space before it does not move the UTR's end
    assert_eq!(party_of("NEFT-ZZZZZ00000000001-BLUE-RIVER"), "BLUE-RIVER");
    assert_eq!(
        party_of("RTGS-ZZZZZ00000000000000002- GREEN FIELD LTD"),
        "GREEN FIELD LTD"
    );
    // cash, a loan instalment and three kinds of bank charge
    assert_eq!(party_of("BY CASH"), CASH_DEPOSIT);
    // each loan is its own party: the narration, number included, is named as printed
    assert_eq!(
        party_of("Loan Recovery For00000000000001"),
        "Loan Recovery For00000000000001"
    );
    assert_eq!(
        party_of("Loan Recovery For00000000000002"),
        "Loan Recovery For00000000000002"
    );
    for charge in [
        "CHARGES FOR :IMPS/P2A/600000000003/XXXXXXXX0000",
        "Charges for PORD Customer Payment :600000000009",
        "SMS Charges for AUG 26",
    ] {
        assert_eq!(party_of(charge), BANK_CHARGES, "{charge}");
    }
}

#[test]
fn a_neft_name_broken_at_the_cell_edge_is_not_named() {
    // the first fragment ends at the wrap edge: it may have been cut inside a word or at
    // a space that fell there, so the de-wrapped and the spaced readings differ
    let cut = Printed {
        narration: &["NEFT-ZZZZZ00000000001-ACME INDUSTRIES-EAST-WESTERN", "LTD"],
        ..ACME
    };
    let page = bob_page(
        Some(MASKED),
        &[cut],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    );
    let cut_rows = parse_statement(&[page], Bank::Bob).unwrap();
    assert_eq!(
        cut_rows[0].get("narr"),
        "NEFT-ZZZZZ00000000001-ACME INDUSTRIES-EAST-WESTERNLTD"
    );
    assert_eq!(
        cut_rows[0].get("narr_spaced"),
        "NEFT-ZZZZZ00000000001-ACME INDUSTRIES-EAST-WESTERN LTD"
    );
    assert_eq!(Bank::Bob.party(&cut_rows[0]), UNRESOLVED);
    // the same wrap in a UPI narration is whole again: a VPA holds no space
    assert_eq!(
        Bank::Bob.party(&rows()[1]),
        "northwind.traders.synthetic@okzzzzzz"
    );
}

#[test]
fn a_bob_narration_that_does_not_settle_a_party_goes_to_suspense() {
    for unsettled in [
        // a VPA that ends the text may have lost its handle to the bank's cut
        "UPI/600000000005/09:09:09/UPI/cutoff@ok",
        // or its @ and everything after it
        "UPI/600000000005/09:09:09/UPI/northwind.tr",
        // a token that is not a VPA, though a slash follows it
        "UPI/600000000005/09:09:09/UPI/northwind.traders/ZZZZ",
        // the shape, not the prefix, decides
        "UPI/600000000005/09:09:09/IMPS/nw@okzz/ZZZZ",
        "UPI/60000000000/09:09:09/UPI/nw@okzz/ZZZZ",
        "UPI/600000000005/9:09:09/UPI/nw@okzz/ZZZZ",
        // without a UTR-shaped first field, or without a name after it
        "NEFT-ab-ACME INDUSTRIES",
        "NEFT-ZZZZZ00000000001-",
        "NEFT-ZZZZZ00000000001--",
        "RTGS-ZZZZZ00000000000000002-",
        // no counterparty field is settled for these
        "IMPS/P2A/600000000003/ZZZZZZZZZZ1 234/NOTE",
        "MBK/600000000004/12:00:00/SOME NAME",
        "EBANK:WIB/0000000001/order payment",
        "BY SOME NAME",
        "BY CASH DEPOSIT",
        "TO TRANSFER",
        "00000000000001 Disbursement Credit",
        "00000000000001:Int.Coll:01-07-2026 to 30-09-2026",
        // the loan wording as observed has no space before the digits, and nothing else
        "Loan Recovery For 00000000000001",
        "Loan Recovery For00000000000001 X",
        "xLoan Recovery For1",
        "Loan Recovery For",
        "Cheque Book Charges",
        "SOMETHING ELSE",
    ] {
        assert_eq!(party_of(unsettled), UNRESOLVED, "{unsettled}");
    }
}

#[test]
fn a_bob_reference_is_the_bank_reference_the_narration_carries() {
    let reference = |narration: &str, cheque: &str| {
        Bank::Bob.reference(&row(&[("narr", narration), ("chq", cheque)]))
    };
    let pair = |mode: &str, value: &str| (mode.to_string(), value.to_string());
    assert_eq!(
        reference("UPI/600000000001/10:15:30/UPI/nw@okzz/ZZZZ BANK", ""),
        pair("UPI", "600000000001")
    );
    assert_eq!(
        reference("IMPS/P2A/600000000003/ZZZZZZZZZZ1 234/NOTE", ""),
        pair("IMPS", "600000000003")
    );
    assert_eq!(
        reference("MBK/600000000004/12:00:00/SOME NAME", ""),
        pair("MBK", "600000000004")
    );
    assert_eq!(
        reference("NEFT-ZZZZZ00000000001-ACME INDUSTRIES", ""),
        pair("NEFT", "ZZZZZ00000000001")
    );
    assert_eq!(
        reference("RTGS-ZZZZZ00000000000000002-GREEN FIELD LTD", ""),
        pair("RTGS", "ZZZZZ00000000000000002")
    );
    assert_eq!(reference("BY CASH", ""), pair("CASH", ""));
    // a cheque number is the reference of a row with no reference in its narration
    assert_eq!(
        reference("SYNTHETIC SUPPLIES-MICR INWARD CLG (CTS)", "000417"),
        pair("TXN", "000417")
    );
    // a UTR must look like one
    assert_eq!(reference("NEFT-ab-ACME", ""), pair("TXN", ""));
}

#[test]
fn only_the_masked_number_above_the_table_binds_on_its_clear_digits() {
    let bound = |label: &str| require_account_match(&pages(), Bank::Bob, label);
    assert_eq!(bound("BOB CA 12399999999456").unwrap(), "123XXXXXXXX456");
    // the real length is not known (the mask may or may not keep it): a shorter or a longer
    // number that begins and ends with the clear digits is accepted, and nothing in the
    // middle is compared
    assert_eq!(bound("BOB CA 123999999456").unwrap(), "123XXXXXXXX456");
    assert_eq!(
        bound("BOB CA 1230000000000000456").unwrap(),
        "123XXXXXXXX456"
    );
    // the clear digits must agree: the first three and the last three
    refuses(bound("BOB CA 12499999999456"), "account_not_in_statement");
    refuses(bound("BOB CA 12399999999457"), "account_not_in_statement");
    // a tail cannot bind a number whose middle is masked
    refuses(bound("BOB CA xx0456"), "unbindable_account");
    // the clear digits alone are not a number: something must stand where the mask is
    refuses(bound("BOB CA 123456"), "unbindable_account");
    refuses(bound("BOB CA 1234567"), "account_not_in_statement");
    assert_eq!(bound("BOB CA 1237456").unwrap(), "123XXXXXXXX456");
    refuses(bound("BOB CA 456"), "unbindable_account");

    // a masked number inside a narration is the counterparty's, and is not read
    let mut narrated = pages();
    narrated[0] = bob_page(
        Some(MASKED),
        &[
            Printed {
                narration: &["CHARGES FOR 555XXXXXXXX555"],
                ..SMS
            },
            LOAN,
            ACME,
        ],
        Some(FOOTER_ONE),
    );
    assert_eq!(
        require_account_match(&narrated, Bank::Bob, "BOB CA 12399999999456").unwrap(),
        "123XXXXXXXX456"
    );

    // two different masked numbers in the header block: ambiguous
    let two = vec![bob_page(
        Some(&["123XXXXXXXX456", "123XXXXXXXX457"]),
        &[SMS],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    )];
    refuses(
        require_account_match(&two, Bank::Bob, "BOB CA 12399999999456"),
        "ambiguous_account_match",
    );
    // the same number printed three times is one number
    let thrice = vec![bob_page(
        Some(&["123XXXXXXXX456", "123XXXXXXXX456", "123XXXXXXXX456"]),
        &[SMS],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    )];
    assert_eq!(
        require_account_match(&thrice, Bank::Bob, "BOB CA 12399999999456").unwrap(),
        "123XXXXXXXX456"
    );
    // none at all
    let none = vec![bob_page(
        Some(&[]),
        &[SMS],
        Some("09/10/2026 10:30 SYNTH-ID/00000 Page 1 of 1"),
    )];
    refuses(
        require_account_match(&none, Bank::Bob, "BOB CA 12399999999456"),
        "no_account_number_line",
    );
}

#[test]
fn slashed_dates_are_strict() {
    assert_eq!(
        parse_day_month_year_slashed("29/02/2028"),
        Date::new(2028, 2, 29)
    );
    for bad in [
        "29/02/2026",
        "31/04/2026",
        "1/08/2026",
        "01/8/2026",
        "01/08/26",
        "01-08-2026",
        "01/08/2026 ",
        "00/08/2026",
    ] {
        assert_eq!(parse_day_month_year_slashed(bad), None, "{bad}");
    }
}

fn request<'a>(controls: &'a Controls, mapping: &'a Mapping) -> StatementRequest<'a> {
    StatementRequest {
        bank: Bank::Bob,
        account_label: "BOB CA 12399999999456",
        controls,
        bank_ledger: "Bob Bank",
        suspense_ledger: "Suspense",
        mapping,
        cash_answers: CashAnswers::none(),
        date_from: None,
        date_to: None,
    }
}

fn no_mapping() -> Mapping {
    Mapping::from_rows(std::iter::empty::<MappingRow>()).unwrap()
}

#[test]
fn a_bob_statement_proves_itself_from_the_callers_opening_and_closing_balances() {
    let mapping = no_mapping();
    let controls = Controls::parse_optional("10,000.00", "-11,635.26", None, None).unwrap();
    let parsed = prepare(&pages(), &request(&controls, &mapping)).unwrap();
    assert_eq!(parsed.account_number, "123XXXXXXXX456");
    assert_eq!(parsed.statement_rows, 5);
    assert_eq!(format_amount(&parsed.totals.debits), "32262.30");
    assert_eq!(format_amount(&parsed.totals.credits), "10627.04");
    // the BY CASH deposit is a cash line nobody has answered: no voucher yet
    assert_eq!(parsed.build.proposals.len(), 4);
    let open: Vec<_> = parsed
        .build
        .records
        .iter()
        .filter(|record| record.disposition == Disposition::NeedsAnswer)
        .collect();
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].party, CASH_DEPOSIT);
    assert_eq!(open[0].cash_movement, Some(CashMovement::Deposit));

    // the opening balance ties the oldest row
    let off = Controls::parse_optional("10,000.01", "-11,635.26", None, None).unwrap();
    refuses(
        prepare(&pages(), &request(&off, &mapping)),
        "balance_chain_broken",
    );
    // the closing balance ties the newest: a dropped newest row is seen
    let short = Controls::parse_optional("10,000.00", "-11,623.46", None, None).unwrap();
    refuses(
        prepare(&pages(), &request(&short, &mapping)),
        "extent_unproven",
    );
}

// --- Bank identity (the page-1 IFSC Code line) -----------------------------------

/// The pages with page 1's identity words edited by `edit`.
fn pages_with(edit: impl FnOnce(&mut Page)) -> Vec<Page> {
    let mut pages = pages();
    edit(&mut pages[0]);
    pages
}

fn identity_refusal(pages: &[Page]) -> Refusal {
    let refusal = refuses(parse_statement(pages, Bank::Bob), "bank_not_recognised");
    // The refusal never carries the code it read.
    assert!(!refusal.message.contains("BARB"), "{refusal}");
    assert!(!refusal.message.contains(IFSC_VALUE), "{refusal}");
    refusal
}

fn value_word(page: &mut Page) -> &mut Word {
    page.iter_mut()
        .find(|word| word.text == IFSC_VALUE)
        .unwrap()
}

#[test]
fn a_statement_whose_page_one_ifsc_code_has_the_bank_prefix_is_read() {
    assert_eq!(rows().len(), 5);
}

#[test]
fn another_banks_code_malformed_codes_and_a_missing_value_refuse() {
    for bad in [
        "HDFC0SYNTH1",  // another bank
        "barb0synth1",  // not capitals
        "BARB0SYNTH",   // ten characters
        "BARB0SYNTH12", // twelve characters
        "BARB1SYNTH1",  // the fifth character is not a zero
        "BARB0SYNT-1",  // a character that is neither a capital letter nor a digit
    ] {
        let pages = pages_with(|page| value_word(page).text = bad.to_string());
        identity_refusal(&pages);
    }
    // "Code:" is the last word on its line: nothing follows it.
    let pages = pages_with(|page| page.retain(|word| word.x0 <= 38.0 || word.y0 != 211.0));
    let refusal = identity_refusal(&pages);
    assert!(refusal.message.contains("no value"), "{refusal}");
    // The value glued to the label, and a non-ASCII value of the right length.
    identity_refusal(&pages_with(|page| {
        page.iter_mut()
            .find(|word| word.text == "Code:")
            .unwrap()
            .text = format!("Code:{IFSC_VALUE}");
        page.retain(|word| word.text != IFSC_VALUE);
    }));
    identity_refusal(&pages_with(|page| {
        value_word(page).text = "BARB0SYNT\u{e9}1".to_string()
    }));
}

#[test]
fn the_identity_is_checked_before_the_footers() {
    // Page 1 names another bank and page 2 has no footer: the first refusal wins.
    let mut both = pages_with(|page| value_word(page).text = "HDFC0SYNTH1".to_string());
    both[1] = bob_page(None, &[UPI, CASH], None);
    identity_refusal(&both);
}

#[test]
fn no_ifsc_line_a_split_label_or_a_repeated_line_refuses() {
    // No line at all.
    identity_refusal(&pages_with(|page| {
        page.retain(|word| !["IFSC", "Code:", IFSC_VALUE].contains(&word.text.as_str()))
    }));
    // The label without "Code:".
    identity_refusal(&pages_with(|page| page.retain(|word| word.text != "Code:")));
    // The label followed by another word where "Code:" belongs, then a valid code.
    identity_refusal(&pages_with(|page| {
        page.iter_mut()
            .find(|word| word.text == "Code:")
            .unwrap()
            .text = "Number:".to_string()
    }));
    // "Code:" first, then "IFSC".
    identity_refusal(&pages_with(|page| {
        for word in page.iter_mut() {
            if word.text == "IFSC" {
                word.text = "Code:".to_string();
            } else if word.text == "Code:" {
                word.text = "IFSC".to_string();
            }
        }
    }));
    // A second identical line.
    identity_refusal(&pages_with(|page| {
        let copy: Vec<Word> = page
            .iter()
            .filter(|word| ["IFSC", "Code:", IFSC_VALUE].contains(&word.text.as_str()))
            .map(|word| {
                Word::new(
                    word.x0,
                    word.y0 + 40.0,
                    word.x1,
                    word.y1 + 40.0,
                    word.text.clone(),
                )
            })
            .collect();
        page.extend(copy);
    }));
}

#[test]
fn the_code_is_read_on_page_one_only_and_a_prefix_in_a_narration_is_not_identity() {
    // The line moved to page 2 and removed from page 1.
    let mut moved = pages();
    let line: Vec<Word> = moved[0]
        .iter()
        .filter(|word| ["IFSC", "Code:", IFSC_VALUE].contains(&word.text.as_str()))
        .cloned()
        .collect();
    moved[0].retain(|word| !["IFSC", "Code:", IFSC_VALUE].contains(&word.text.as_str()));
    moved[1].extend(line);
    identity_refusal(&moved);
    // A narration that carries the bank's prefix is a counterparty's bank, not identity.
    let mut narrated = pages();
    narrated[0].retain(|word| !["IFSC", "Code:", IFSC_VALUE].contains(&word.text.as_str()));
    put(&mut narrated[0], 149.5, 500.0, "NEFT-BARB0SYNTH1-ACME");
    identity_refusal(&narrated);
}
