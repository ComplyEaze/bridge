//! The CSV edge, on a layout invented for these tests. No shipped layout
//! exists yet: a real one takes its header from a real export.

mod common;

use bridge_bank_statement::bank::{BALANCE, CREDIT, DATE, DEBIT, NARRATION};
use bridge_bank_statement::csv_edge::{read_rows, CsvLayout, DateFormat, Grouping, RowOrder};
use bridge_bank_statement::money::reconcile;
use bridge_tally_primitives::ExactDecimal;
use common::refuses;

const HEADER: &str = "Date,Narration,Withdrawal,Deposit,Balance";

const OLDEST_FIRST: CsvLayout = CsvLayout {
    id: "test_oldest_first",
    header: &["Date", "Narration", "Withdrawal", "Deposit", "Balance"],
    date: 0,
    date_format: DateFormat::DayMonthYearHyphen,
    order: RowOrder::OldestFirst,
    narration: 1,
    debit: 2,
    credit: 3,
    balance: 4,
    grouping: Grouping::Western,
};

fn file(lines: &[&str]) -> Vec<u8> {
    let mut text = String::from(HEADER);
    for line in lines {
        text.push('\n');
        text.push_str(line);
    }
    text.into_bytes()
}

const TWO_ROWS: [&str; 2] = [
    "01-08-2026,UPI/SYNTHETIC PAYEE,,\"1,500.00\",\"11,500.00\"",
    "02-08-2026,NEFT/SYNTHETIC SUPPLIER,250.5,,\"11,249.50\"",
];

#[test]
fn a_matching_file_becomes_rows_with_iso_dates_and_plain_two_place_amounts() {
    let rows = read_rows(&file(&TWO_ROWS), &OLDEST_FIRST).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get(DATE), "2026-08-01");
    assert_eq!(rows[0].get(NARRATION), "UPI/SYNTHETIC PAYEE");
    assert_eq!(
        (
            rows[0].get(DEBIT),
            rows[0].get(CREDIT),
            rows[0].get(BALANCE)
        ),
        ("", "1500.00", "11500.00")
    );
    // 250.5 is read as 250.50
    assert_eq!(rows[1].get(DEBIT), "250.50");
    // the rows run through the existing running-balance proof unchanged
    let opening = ExactDecimal::parse("10000.00".to_string()).unwrap();
    let closing = ExactDecimal::parse("11249.50".to_string()).unwrap();
    reconcile(&rows, &opening, &closing).unwrap();
}

#[test]
fn a_header_that_is_not_the_layouts_is_refused_and_never_guessed() {
    for header in [
        "Date,Narration,Withdrawal,Deposit",
        "Narration,Date,Withdrawal,Deposit,Balance",
        "Date,Narration,Withdrawal,Deposit,Balance ",
        "date,narration,withdrawal,deposit,balance",
        "",
    ] {
        let bytes = format!("{header}\n{}\n", TWO_ROWS[0]).into_bytes();
        refuses(read_rows(&bytes, &OLDEST_FIRST), "layout_unknown");
    }
}

#[test]
fn a_utf8_byte_order_mark_is_stripped_and_anything_else_that_is_not_utf8_is_refused() {
    let mut with_mark = b"\xef\xbb\xbf".to_vec();
    with_mark.extend(file(&TWO_ROWS));
    assert_eq!(read_rows(&with_mark, &OLDEST_FIRST).unwrap().len(), 2);

    let text = String::from_utf8(file(&TWO_ROWS)).unwrap();
    let mut le = b"\xff\xfe".to_vec();
    le.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
    let mut be = b"\xfe\xff".to_vec();
    be.extend(text.encode_utf16().flat_map(u16::to_be_bytes));
    let mut nul = file(&TWO_ROWS);
    nul.insert(10, 0);
    let invalid = [file(&TWO_ROWS), vec![0xc3, 0x28]].concat();
    for bytes in [le, be, nul, invalid] {
        refuses(read_rows(&bytes, &OLDEST_FIRST), "encoding_unsupported");
    }
}

#[test]
fn a_ragged_or_stray_line_is_refused_at_its_row() {
    for stray in ["TOTAL,5", "02-08-2026,A,B,C,D,E"] {
        let refusal = refuses(
            read_rows(&file(&[TWO_ROWS[0], stray]), &OLDEST_FIRST),
            "malformed_row",
        );
        assert_eq!(refusal.row, Some(2));
    }
}

#[test]
fn a_quoted_narration_with_a_comma_and_a_line_break_is_one_narration() {
    let rows = read_rows(
        &file(&["01-08-2026,\"PAID, TO\nSYNTHETIC PAYEE\",,\"1,500.00\",\"11,500.00\""]),
        &OLDEST_FIRST,
    )
    .unwrap();
    assert_eq!(rows[0].get(NARRATION), "PAID, TO\nSYNTHETIC PAYEE");
}

#[test]
fn a_date_outside_the_layouts_format_is_refused_including_a_spreadsheets_rewrite() {
    let slash = CsvLayout {
        date_format: DateFormat::DayMonthYearSlash,
        ..OLDEST_FIRST
    };
    for (layout, date) in [
        (&OLDEST_FIRST, "5-8-2026"),
        (&OLDEST_FIRST, "01/08/2026"),
        (&OLDEST_FIRST, "31-02-2026"),
        (&OLDEST_FIRST, "01-08-26"),
        (&slash, "5/9/2026"),
        (&slash, "01-08-2026"),
    ] {
        let line = format!("{date},A,,\"1.00\",\"2.00\"");
        let refusal = refuses(read_rows(&file(&[&line]), layout), "unparseable_date");
        assert_eq!(refusal.row, Some(1), "{date}");
    }
    let line = "01/08/2026,A,,\"1.00\",\"2.00\"";
    assert_eq!(
        read_rows(&file(&[line]), &slash).unwrap()[0].get(DATE),
        "2026-08-01"
    );
}

#[test]
fn rows_out_of_the_declared_order_are_refused_and_a_newest_first_file_is_returned_oldest_first() {
    let reversed = [TWO_ROWS[1], TWO_ROWS[0]];
    let refusal = refuses(
        read_rows(&file(&reversed), &OLDEST_FIRST),
        "date_order_broken",
    );
    assert_eq!(refusal.row, Some(2));

    let newest_first = CsvLayout {
        order: RowOrder::NewestFirst,
        ..OLDEST_FIRST
    };
    let refusal = refuses(
        read_rows(&file(&TWO_ROWS), &newest_first),
        "date_order_broken",
    );
    assert_eq!(refusal.row, Some(2));
    let rows = read_rows(&file(&reversed), &newest_first).unwrap();
    assert_eq!(rows[0].get(DATE), "2026-08-01");
    assert_eq!(rows[1].get(DATE), "2026-08-02");

    // equal dates are in order either way (a newest-first file reverses same-day rows)
    let same_day = [TWO_ROWS[0], TWO_ROWS[0]];
    assert_eq!(read_rows(&file(&same_day), &OLDEST_FIRST).unwrap().len(), 2);
    assert_eq!(read_rows(&file(&same_day), &newest_first).unwrap().len(), 2);
}

#[test]
fn an_amount_outside_the_declared_grouping_is_refused() {
    let indian = CsvLayout {
        grouping: Grouping::Indian,
        ..OLDEST_FIRST
    };
    let plain = CsvLayout {
        grouping: Grouping::None,
        ..OLDEST_FIRST
    };
    let amount = |layout: &CsvLayout, cell: &str| {
        let line = format!("01-08-2026,A,,\"{cell}\",\"1.00\"");
        read_rows(&file(&[&line]), layout)
    };
    // the declared grouping reads, to two places
    assert_eq!(
        amount(&OLDEST_FIRST, "1,234,567.5").unwrap()[0].get(CREDIT),
        "1234567.50"
    );
    assert_eq!(
        amount(&indian, "12,34,567.50").unwrap()[0].get(CREDIT),
        "1234567.50"
    );
    assert_eq!(
        amount(&plain, "1234567").unwrap()[0].get(CREDIT),
        "1234567.00"
    );
    // anything else is refused, including an ungrouped or regrouped cell
    for (layout, cell) in [
        (&OLDEST_FIRST, "1000.00"),
        (&OLDEST_FIRST, "12,34,567.50"),
        (&OLDEST_FIRST, "1,0000.00"),
        (&OLDEST_FIRST, "1E+03"),
        (&OLDEST_FIRST, "1.234"),
        (&OLDEST_FIRST, "-5.00"),
        (&indian, "1,234,567.50"),
        (&plain, "1,000.00"),
    ] {
        let refusal = refuses(amount(layout, cell), "malformed_amount");
        assert_eq!(refusal.row, Some(1), "{cell}");
    }
}

#[test]
fn a_negative_amount_is_refused_on_either_side() {
    for line in [
        "01-08-2026,A,\"-5.00\",,\"1.00\"",
        "01-08-2026,A,,\"-5.00\",\"1.00\"",
    ] {
        let refusal = refuses(read_rows(&file(&[line]), &OLDEST_FIRST), "malformed_amount");
        assert_eq!(refusal.row, Some(1));
    }
}

#[test]
fn a_balance_may_be_negative_but_not_blank() {
    let negative = "01-08-2026,A,\"5.00\",,\"-5.00\"";
    assert_eq!(
        read_rows(&file(&[negative]), &OLDEST_FIRST).unwrap()[0].get(BALANCE),
        "-5.00"
    );
    let blank = "01-08-2026,A,\"5.00\",,";
    let refusal = refuses(
        read_rows(&file(&[blank]), &OLDEST_FIRST),
        "malformed_balance",
    );
    assert_eq!(refusal.row, Some(1));
    let bad = "01-08-2026,A,\"5.00\",,\"1,00.00\"";
    let refusal = refuses(
        read_rows(&file(&[TWO_ROWS[0], bad]), &OLDEST_FIRST),
        "malformed_balance",
    );
    assert_eq!(refusal.row, Some(2));
}

#[test]
fn the_same_figures_in_two_groupings_read_to_identical_rows() {
    // the transaction id hashes the row's text, so it must not depend on how
    // the bank grouped the digits
    let western = read_rows(
        &file(&["01-08-2026,A,,\"1,234,567.50\",\"2,000,000.00\""]),
        &OLDEST_FIRST,
    )
    .unwrap();
    let indian = CsvLayout {
        grouping: Grouping::Indian,
        ..OLDEST_FIRST
    };
    let lakh = read_rows(
        &file(&["01-08-2026,A,,\"12,34,567.50\",\"20,00,000.00\""]),
        &indian,
    )
    .unwrap();
    assert_eq!(western, lakh);
}

#[test]
fn each_column_is_read_from_the_layouts_own_index() {
    // balance first, then date, credit, narration, debit
    let permuted = CsvLayout {
        header: &["Balance", "Date", "Deposit", "Narration", "Withdrawal"],
        date: 1,
        narration: 3,
        debit: 4,
        credit: 2,
        balance: 0,
        ..OLDEST_FIRST
    };
    let bytes = "Balance,Date,Deposit,Narration,Withdrawal\n\"11,500.00\",01-08-2026,\"1,500.00\",PAYEE,\n\"11,249.50\",02-08-2026,,SUPPLIER,250.5\n"
        .as_bytes();
    let rows = read_rows(bytes, &permuted).unwrap();
    let read: Vec<[&str; 5]> = rows
        .iter()
        .map(|row| {
            [
                row.get(DATE),
                row.get(NARRATION),
                row.get(DEBIT),
                row.get(CREDIT),
                row.get(BALANCE),
            ]
        })
        .collect();
    assert_eq!(
        read,
        [
            ["2026-08-01", "PAYEE", "", "1500.00", "11500.00"],
            ["2026-08-02", "SUPPLIER", "250.50", "", "11249.50"],
        ]
    );
}

#[test]
fn a_layout_whose_columns_are_not_distinct_columns_of_its_header_is_refused() {
    for broken in [
        CsvLayout {
            balance: 5,
            ..OLDEST_FIRST
        },
        CsvLayout {
            credit: 2,
            ..OLDEST_FIRST
        },
        CsvLayout {
            narration: 0,
            ..OLDEST_FIRST
        },
    ] {
        refuses(read_rows(&file(&TWO_ROWS), &broken), "layout_invalid");
    }
}

#[test]
fn a_leading_zero_is_refused_so_one_value_has_one_spelling() {
    let plain = CsvLayout {
        grouping: Grouping::None,
        ..OLDEST_FIRST
    };
    let indian = CsvLayout {
        grouping: Grouping::Indian,
        ..OLDEST_FIRST
    };
    let amount = |layout: &CsvLayout, cell: &str| {
        read_rows(
            &file(&[&format!("01-08-2026,A,,\"{cell}\",\"1.00\"")]),
            layout,
        )
    };
    for (layout, cell) in [
        (&OLDEST_FIRST, "0,100.00"),
        (&OLDEST_FIRST, "01.50"),
        (&plain, "007.50"),
        (&indian, "0,100.00"),
        (&indian, "01,234.00"),
    ] {
        refuses(amount(layout, cell), "malformed_amount");
    }
    // zero itself, and a fraction below one, are fine
    assert_eq!(amount(&plain, "0").unwrap()[0].get(CREDIT), "0.00");
    assert_eq!(
        amount(&OLDEST_FIRST, "0.50").unwrap()[0].get(CREDIT),
        "0.50"
    );
}

#[test]
fn a_file_with_no_rows_is_not_an_error_at_the_edge_and_an_empty_file_is_no_layout() {
    // the running-balance proof refuses an empty statement; the edge does not decide that
    assert!(read_rows(&file(&[]), &OLDEST_FIRST).unwrap().is_empty());
    refuses(read_rows(b"", &OLDEST_FIRST), "layout_unknown");
}

#[test]
fn whitespace_around_a_date_or_an_amount_cell_is_trimmed() {
    let line = "01-08-2026,  A  ,,\" 5.00 \",\"1.00\"";
    let rows = read_rows(&file(&[line]), &OLDEST_FIRST).unwrap();
    assert_eq!(rows[0].get(CREDIT), "5.00");
    assert_eq!(rows[0].get(NARRATION), "A");
    let padded_date = " 01-08-2026,A,,\"5.00\",\"1.00\"";
    assert_eq!(
        read_rows(&file(&[padded_date]), &OLDEST_FIRST).unwrap()[0].get(DATE),
        "2026-08-01"
    );
}
