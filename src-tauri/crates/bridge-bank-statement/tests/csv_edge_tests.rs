//! The CSV edge on a private bank's export layout. Every file here is
//! SYNTHETIC: rows invented for these tests in the layout a real export has
//! (a preamble, nine columns, a closing-balance trailer); no value of
//! the real file the layout was measured on is kept. The people, accounts and
//! references below do not exist.

mod common;

use bridge_bank_statement::bank::{BALANCE, CREDIT, DATE, DEBIT, NARRATION};
use bridge_bank_statement::csv_edge::{read_statement, CHQ_REF, MAX_FILE_BYTES};
use bridge_bank_statement::money::reconcile;
use common::{refuses, rep};

const HEADER: &str = "\"Sl. No.\",\"Transaction Date\",\"Value Date\",\"Description\",\"Chq /Ref No.\",\"Amount\",\"Dr / Cr\",\"Balance\",\"Dr / Cr\"";

/// The preamble of an export: ragged rows, key/value pairs beside the address.
fn preamble() -> Vec<String> {
    vec![
        "\"\",\"\",\"Account Statement\"".to_string(),
        format!(
            "\"SYNTHETIC LAB CUSTOMER\",\"\",\"\",\"\",\"Cust. Reln. No.\",\"{}\"",
            rep('0', 9)
        ),
        format!(
            "\"1 SYNTHETIC ROAD\",\"\",\"\",\"\",\"Account No.\",\"{}\"",
            rep('0', 12)
        ),
        "\"SYNTHETIC CITY 000000\",\"\",\"\",\"\",\"Period\",\"From 01/04/2025 To 31/03/2026\""
            .to_string(),
        "\"\",\"\",\"\",\"\",\"Currency\",\"INR\"".to_string(),
        String::new(),
    ]
}

type Txn<'a> = (
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
);

/// `(transaction date, value date, description, ref, amount, amount side, balance, balance side)`
/// without the serial: it is added in order.
fn transaction(serial: usize, t: &Txn<'_>, balance_side: &str) -> String {
    format!(
        "\"{serial}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{balance_side}\"",
        t.0, t.1, t.2, t.3, t.4, t.5, t.6
    )
}

fn trailer(closing: &str) -> Vec<String> {
    vec![
        format!("\"Closing Balance\",\"as on 31/03/2026 INR {closing}\""),
        String::new(),
        "\"Important Note:\"".to_string(),
        "\"The transaction date is the posting time.\"".to_string(),
        String::new(),
    ]
}

const FIRST: Txn<'static> = (
    "02-04-2025 10:15:00",
    "02-04-2025",
    "UPI/SYNTHETIC PAYER/lab",
    "SYN 000001  A",
    "1,500.00",
    "CR",
    "11,500.00",
);
const SECOND: Txn<'static> = (
    "03-04-2025 09:00:01",
    "03-04-2025",
    "NEFT/SYNTHETIC, SUPPLIER/lab",
    "",
    "250.50",
    "DR",
    "11,249.50",
);
/// Interest posted after midnight takes the previous day as its value date.
const THIRD: Txn<'static> = (
    "01-07-2025 00:00:05",
    "30-06-2025",
    "INT.PD SYNTHETIC",
    "SYN 000003",
    "99,999.50",
    "CR",
    "1,11,249.00",
);

fn file(rows: &[(Txn<'_>, &str)], closing: &str) -> Vec<u8> {
    let mut lines = preamble();
    lines.push(HEADER.to_string());
    for (index, (row, balance_side)) in rows.iter().enumerate() {
        lines.push(transaction(index + 1, row, balance_side));
    }
    lines.extend(trailer(closing));
    lines.join("\n").into_bytes()
}

fn good() -> Vec<u8> {
    file(
        &[(FIRST, "CR"), (SECOND, "CR"), (THIRD, "CR")],
        "1,11,249.00",
    )
}

fn decimal(text: &str) -> bridge_tally_primitives::ExactDecimal {
    bridge_tally_primitives::ExactDecimal::parse(text.to_string()).unwrap()
}

#[test]
fn a_matching_file_becomes_rows_on_the_value_date_with_plain_two_place_amounts() {
    let statement = read_statement(&good()).unwrap();
    assert_eq!(statement.rows.len(), 3);
    let first = &statement.rows[0];
    assert_eq!(first.get(DATE), "2025-04-02");
    assert_eq!(first.get(NARRATION), "UPI/SYNTHETIC PAYER/lab");
    // The reference keeps its internal run of spaces, apart from the description.
    assert_eq!(first.get(CHQ_REF), "SYN 000001  A");
    assert_eq!(
        (first.get(DEBIT), first.get(CREDIT), first.get(BALANCE)),
        ("", "1500.00", "11500.00")
    );
    let second = &statement.rows[1];
    // A comma inside a quoted description is part of it; an empty reference is empty.
    assert_eq!(second.get(NARRATION), "NEFT/SYNTHETIC, SUPPLIER/lab");
    assert_eq!(second.get(CHQ_REF), "");
    assert_eq!((second.get(DEBIT), second.get(CREDIT)), ("250.50", ""));
    // The value date, not the posting date, is the row's date.
    assert_eq!(statement.rows[2].get(DATE), "2025-06-30");
    assert_eq!(statement.rows[2].get(BALANCE), "111249.00");
    assert!(statement.opening.numeric_eq(&decimal("10000.00")));
    assert!(statement.closing.numeric_eq(&decimal("111249.00")));
}

#[test]
fn the_rows_run_through_the_running_balance_proof_unchanged() {
    let statement = read_statement(&good()).unwrap();
    reconcile(&statement.rows, &statement.opening, &statement.closing).unwrap();
    // A balance one paisa off breaks the chain the proof replays.
    let bytes = String::from_utf8(good())
        .unwrap()
        .replace("\"11,249.50\"", "\"11,249.51\"");
    let broken = read_statement(bytes.as_bytes()).unwrap();
    refuses(
        reconcile(&broken.rows, &broken.opening, &broken.closing),
        "balance_chain_broken",
    );
}

#[test]
fn a_debit_balance_is_negative_by_its_own_indicator() {
    // synthetic: an overdrawn account after the second row.
    let overdrawn: Txn<'static> = (
        "04-04-2025 10:00:00",
        "04-04-2025",
        "CHQ CLEARING SYNTHETIC",
        "000123",
        "12,000.00",
        "DR",
        "750.50",
    );
    let recovered: Txn<'static> = (
        "05-04-2025 10:00:00",
        "05-04-2025",
        "NEFT/SYNTHETIC PAYER/lab",
        "SYN 000002",
        "1,00,000.00",
        "CR",
        "99,249.50",
    );
    let bytes = file(
        &[
            (FIRST, "CR"),
            (SECOND, "CR"),
            (overdrawn, "DR"),
            (recovered, "CR"),
        ],
        "99,249.50",
    );
    let statement = read_statement(&bytes).unwrap();
    // The debit balance is negative; the credit balance after it is positive again.
    assert_eq!(statement.rows[2].get(BALANCE), "-750.50");
    assert_eq!(statement.rows[3].get(BALANCE), "99249.50");
    reconcile(&statement.rows, &statement.opening, &statement.closing).unwrap();
    // A file that ends on a debit balance has a closing line of a spelling that
    // has not been seen: its plain positive form does not equal the balance, so
    // it is refused, not guessed at.
    let overdrawn_end = file(
        &[(FIRST, "CR"), (SECOND, "CR"), (overdrawn, "DR")],
        "750.50",
    );
    refuses(read_statement(&overdrawn_end), "closing_balance_differs");
}

#[test]
fn encodings_the_export_does_not_use_are_refused() {
    let utf16: Vec<u8> = "\"Sl. No.\"\n"
        .encode_utf16()
        .flat_map(|unit| unit.to_le_bytes())
        .collect();
    refuses(read_statement(&utf16), "encoding_unsupported");
    let mut with_bom = vec![0xEF, 0xBB, 0xBF];
    with_bom.extend(good());
    refuses(read_statement(&with_bom), "encoding_unsupported");
    let crlf = String::from_utf8(good()).unwrap().replace('\n', "\r\n");
    refuses(read_statement(crlf.as_bytes()), "encoding_unsupported");
    let mut nul = good();
    nul.push(0);
    refuses(read_statement(&nul), "encoding_unsupported");
}

#[test]
fn a_header_that_is_not_exactly_the_exports_is_layout_unknown() {
    let text = String::from_utf8(good()).unwrap();
    // A renamed cell.
    let renamed = text.replace("\"Chq /Ref No.\"", "\"Cheque No.\"");
    refuses(read_statement(renamed.as_bytes()), "layout_unknown");
    // A space-padded cell.
    let padded = text.replace("\"Balance\"", "\"Balance \"");
    refuses(read_statement(padded.as_bytes()), "layout_unknown");
    // No header at all.
    let missing = text.replace(HEADER, "\"x\"");
    refuses(read_statement(missing.as_bytes()), "layout_unknown");
    // The header twice.
    let twice = text.replace(HEADER, &format!("{HEADER}\n{HEADER}"));
    refuses(read_statement(twice.as_bytes()), "layout_unknown");
    // A preamble line that is not quoted fields.
    let loose = text.replace("\"Account Statement\"", "Account Statement");
    refuses(read_statement(loose.as_bytes()), "layout_unknown");
}

#[test]
fn the_header_is_found_by_content_whatever_the_length_of_the_address_block() {
    let text = String::from_utf8(good()).unwrap();
    let longer = text.replace(
        "\"1 SYNTHETIC ROAD\"",
        "\"1 SYNTHETIC ROAD\",\"\",\"\",\"\",\"x\",\"y\"\n\"2 SYNTHETIC LANE\",\"\",\"\"\n\"1 SYNTHETIC ROAD\"",
    );
    let statement = read_statement(longer.as_bytes()).unwrap();
    assert_eq!(statement.rows.len(), 3);
}

#[test]
fn a_line_that_is_not_the_exports_quoting_is_a_malformed_row() {
    let text = String::from_utf8(good()).unwrap();
    // An unquoted field.
    let unquoted = text.replace("\"250.50\"", "250.50");
    assert_eq!(
        refuses(read_statement(unquoted.as_bytes()), "malformed_row").row,
        Some(2)
    );
    // A column short.
    let short = text.replace(",\"DR\",\"11,249.50\",\"CR\"", ",\"DR\",\"11,249.50\"");
    assert_eq!(
        refuses(read_statement(short.as_bytes()), "malformed_row").row,
        Some(2)
    );
    // A quoted line break.
    let broken = text.replace("SYNTHETIC, SUPPLIER", "SYNTHETIC,\nSUPPLIER");
    assert_eq!(
        refuses(read_statement(broken.as_bytes()), "malformed_row").row,
        Some(2)
    );
    // A blank line between transactions.
    let blank = text.replace("\n\"2\",", "\n\n\"2\",");
    assert_eq!(
        refuses(read_statement(blank.as_bytes()), "malformed_row").row,
        Some(2)
    );
    // Text after a closing quote.
    let trailing = text.replace("\"250.50\"", "\"250.50\"x");
    assert_eq!(
        refuses(read_statement(trailing.as_bytes()), "malformed_row").row,
        Some(2)
    );
}

#[test]
fn a_doubled_quote_inside_a_description_is_one_quote() {
    let text = String::from_utf8(good()).unwrap();
    let quoted = text.replace("UPI/SYNTHETIC PAYER/lab", "UPI/SYNTHETIC \"\"PAYER\"\"/lab");
    let statement = read_statement(quoted.as_bytes()).unwrap();
    assert_eq!(
        statement.rows[0].get(NARRATION),
        "UPI/SYNTHETIC \"PAYER\"/lab"
    );
}

#[test]
fn the_serial_must_run_one_two_three() {
    let text = String::from_utf8(good()).unwrap();
    let skipped = text.replace("\n\"3\",", "\n\"4\",");
    assert_eq!(
        refuses(read_statement(skipped.as_bytes()), "serial_broken").row,
        Some(3)
    );
    let repeated = text.replace("\n\"2\",", "\n\"1\",");
    let refusal = refuses(read_statement(repeated.as_bytes()), "serial_broken");
    assert_eq!(refusal.row, Some(2));
}

#[test]
fn dates_are_checked_for_shape_validity_and_order() {
    let text = String::from_utf8(good()).unwrap();
    // No time on the transaction date.
    let no_time = text.replace("03-04-2025 09:00:01", "03-04-2025");
    assert_eq!(
        refuses(read_statement(no_time.as_bytes()), "unparseable_date").row,
        Some(2)
    );
    // A time that is not a time.
    let hour = text.replace("03-04-2025 09:00:01", "03-04-2025 24:00:01");
    assert_eq!(
        refuses(read_statement(hour.as_bytes()), "unparseable_date").row,
        Some(2)
    );
    // A date that is not a date.
    let day = text.replace("03-04-2025 09:00:01", "31-04-2025 09:00:01");
    assert_eq!(
        refuses(read_statement(day.as_bytes()), "unparseable_date").row,
        Some(2)
    );
    // A value date a spreadsheet rewrote.
    let value = text.replace("\"03-04-2025\"", "\"3/4/2025\"");
    assert_eq!(
        refuses(read_statement(value.as_bytes()), "unparseable_date").row,
        Some(2)
    );
    // Posting order broken by one second.
    let order = text.replace("03-04-2025 09:00:01", "02-04-2025 10:14:59");
    assert_eq!(
        refuses(read_statement(order.as_bytes()), "date_order_broken").row,
        Some(2)
    );
}

#[test]
fn amounts_and_balances_are_only_the_exports_spelling() {
    let text = String::from_utf8(good()).unwrap();
    for (from, to) in [
        // One decimal.
        ("\"99,999.50\"", "\"99,999.5\""),
        // No grouping, Western grouping.
        ("\"1,500.00\"", "\"1500.00\""),
        ("\"1,500.00\"", "\"1,234,567.00\""),
        ("\"1,500.00\"", "\"1,500.000\""),
        ("\"1,500.00\"", "\"-1,500.00\""),
        ("\"1,500.00\"", "\"01,500.00\""),
        ("\"1,500.00\"", "\"1,50.00\""),
        ("\"1,500.00\"", "\"\""),
        ("\"1,500.00\"", "\"1,500.00 \""),
    ] {
        let edited = text.replace(from, to);
        let row = if from.contains("99,999.50") { 3 } else { 1 };
        assert_eq!(
            refuses(read_statement(edited.as_bytes()), "malformed_amount").row,
            Some(row),
            "{from} -> {to}"
        );
    }
    let balance = text.replace("\"11,249.50\"", "\"11249.50\"");
    assert_eq!(
        refuses(read_statement(balance.as_bytes()), "malformed_balance").row,
        Some(2)
    );
    let blank = text.replace("\"11,249.50\"", "\"\"");
    assert_eq!(
        refuses(read_statement(blank.as_bytes()), "malformed_balance").row,
        Some(2)
    );
}

#[test]
fn a_direction_other_than_dr_or_cr_is_refused_on_either_side() {
    let text = String::from_utf8(good()).unwrap();
    let amount_side = text.replace("\"250.50\",\"DR\"", "\"250.50\",\"Dr\"");
    assert_eq!(
        refuses(read_statement(amount_side.as_bytes()), "direction_unknown").row,
        Some(2)
    );
    let balance_side = text.replace("\"11,249.50\",\"CR\"", "\"11,249.50\",\"\"");
    assert_eq!(
        refuses(read_statement(balance_side.as_bytes()), "direction_unknown").row,
        Some(2)
    );
}

#[test]
fn the_trailer_must_be_there_close_the_chain_and_hold_no_rows() {
    let text = String::from_utf8(good()).unwrap();
    // No closing line.
    let cut = text.split("\"Closing Balance\"").next().unwrap();
    refuses(read_statement(cut.as_bytes()), "trailer_missing");
    // A closing line that is not the last balance.
    let differs = text.replace("INR 1,11,249.00", "INR 1,11,249.01");
    refuses(
        read_statement(differs.as_bytes()),
        "closing_balance_differs",
    );
    // A closing line in another spelling.
    let spelled = text.replace("as on 31/03/2026 INR 1,11,249.00", "INR 111249.00");
    refuses(
        read_statement(spelled.as_bytes()),
        "closing_balance_unreadable",
    );
    // A row after the closing line.
    let mut after = text.clone();
    after.push_str(&format!("{}\n", transaction(4, &FIRST, "CR")));
    refuses(read_statement(after.as_bytes()), "trailer_unexpected");
    // Nothing between the header and the closing line.
    let empty = file(&[], "0.00");
    refuses(read_statement(&empty), "empty_statement");
}

#[test]
fn a_cell_too_long_to_be_an_amount_is_refused_not_a_panic() {
    // synthetic: a grouping-valid amount of about 270 bytes.
    let long = format!("1{},000.00", ",00".repeat(90));
    let text = String::from_utf8(good()).unwrap();
    let amount = text.replace("\"1,500.00\"", &format!("\"{long}\""));
    assert_eq!(
        refuses(read_statement(amount.as_bytes()), "malformed_amount").row,
        Some(1)
    );
    let balance = text.replace("\"11,249.50\"", &format!("\"{long}\""));
    assert_eq!(
        refuses(read_statement(balance.as_bytes()), "malformed_balance").row,
        Some(2)
    );
    let closing = text.replace("INR 1,11,249.00", &format!("INR {long}"));
    refuses(
        read_statement(closing.as_bytes()),
        "closing_balance_unreadable",
    );
}

#[test]
fn a_closing_line_with_an_impossible_date_is_unreadable() {
    let text = String::from_utf8(good()).unwrap();
    let date = text.replace("as on 31/03/2026", "as on 99/99/9999");
    refuses(
        read_statement(date.as_bytes()),
        "closing_balance_unreadable",
    );
}

#[test]
fn stray_quotes_are_a_malformed_row() {
    let text = String::from_utf8(good()).unwrap();
    // An unterminated quote, a lone quote, and a quote inside a quoted cell.
    for (from, to) in [
        ("\"SYN 000001  A\"", "\"SYN 000001  A"),
        ("\"250.50\"", "\""),
        ("\"250.50\"", "\"250\"50\""),
    ] {
        let edited = text.replace(from, to);
        let row = if from.contains("SYN 000001") { 1 } else { 2 };
        assert_eq!(
            refuses(read_statement(edited.as_bytes()), "malformed_row").row,
            Some(row),
            "{from}"
        );
    }
}

#[test]
fn a_file_larger_than_any_statement_is_refused_before_it_is_read() {
    let mut big = good();
    big.extend(vec![b' '; MAX_FILE_BYTES]);
    refuses(read_statement(&big), "file_too_large");
}
