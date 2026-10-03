// SPDX-License-Identifier: Apache-2.0

//! A due date Tally printed with a four-digit year (bridge#1091), on the capture that showed it.
//!
//! The two fixtures are one live read each of the synthetic company BRIDGE PROBE B SANDBOX
//! (TallyPrime Silver 7.1, 2 Oct 2026): the Bills Receivable report (1,477 open bills, the request
//! rendered by `render_native_bills_request` from 20250401 to 20260331) and the Bills Payable report
//! (21). Their provenance files give both hashes. Before this change the receivable report refused
//! as a whole, with no cause, because ONE row printed its due date as `1-Dec-2108`.
//!
//! The bill is the project's own probe, dated 1-Aug-25 with a credit period of about 83 years, so
//! the printed date is exactly 1000 months after its bill date (an inference from the arithmetic,
//! not something the report states). The expectations below come from that arithmetic and from the
//! dates the report prints, not from the parser's output.

use bridge_tally_primitives::TallyDate;
use bridge_tally_protocol::native_outstandings::{parse_native_bill_rows, NativeOutstandingsError};

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/agent");

fn fixture(name: &str) -> String {
    let bytes = std::fs::read(format!("{FIXTURES}/{name}.utf16le.xml"))
        .unwrap_or_else(|error| panic!("fixture {name} unreadable: {error}"));
    assert_eq!(bytes.len() % 2, 0, "a UTF-16LE fixture has an even length");
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    String::from_utf16(&units).expect("fixture is UTF-16LE")
}

fn receivable() -> String {
    fixture("native-outstandings-probe-b-bills-receivable")
}

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

fn parse(
    xml: &str,
) -> Result<Vec<bridge_tally_protocol::native_outstandings::NativeBillRow>, NativeOutstandingsError>
{
    parse_native_bill_rows(xml, &date("20250401"), &date("20260331"))
}

/// The 1-based row of the long-credit bill with the four-digit due year in Tally's answer.
const LONG_ROW: usize = 1467;

#[test]
fn the_captured_receivable_report_reads_whole_with_its_four_digit_year_due_date() {
    let rows = parse(&receivable()).expect("the report reads");
    assert_eq!(rows.len(), 1477);
    let by_reference = |reference: &str| {
        rows.iter()
            .find(|row| row.reference == reference)
            .unwrap_or_else(|| panic!("no bill {reference}"))
    };
    // 1-Aug-25 plus 1000 months, read exactly as the printed year says.
    let long = by_reference("CPX-05");
    assert_eq!(long.bill_date.as_str(), "20250801");
    assert_eq!(long.due_date.as_str(), "21081201");
    // The report's row order is Tally's: that bill is its 1,467th.
    assert_eq!(rows[LONG_ROW - 1].reference, "CPX-05");
    // The other long due dates of the same capture print two digits and keep reading as before.
    for (reference, due) in [
        ("CPX-01", "20260801"),
        ("CPX-02", "20350730"),
        ("CPX-04", "20331201"),
        ("CPY-01", "20521217"),
        ("ME-31MAR-2M", "20260531"),
    ] {
        assert_eq!(
            by_reference(reference).due_date.as_str(),
            due,
            "{reference}"
        );
    }
}

#[test]
fn the_captured_payable_report_reads() {
    let rows =
        parse(&fixture("native-outstandings-probe-b-bills-payable")).expect("the report reads");
    assert_eq!(rows.len(), 21);
}

#[test]
fn a_year_that_is_not_the_measured_form_still_refuses_and_names_its_row() {
    let with_due = |due: &str| {
        receivable().replacen(
            "<BILLDUE>1-Dec-2108</BILLDUE>",
            &format!("<BILLDUE>{due}</BILLDUE>"),
            1,
        )
    };
    for (due, code) in [
        // A four-digit year a two-digit year could have carried: not the observed shape.
        ("1-Dec-2026", "native_date_year_invalid"),
        ("1-Dec-0099", "native_date_year_invalid"),
        // Three digits, or a year of letters.
        ("1-Dec-208", "native_date_year_invalid"),
        ("1-Dec-21O8", "native_date_year_invalid"),
        // In full but beyond the window a due date may reach (a hundred years after its bill).
        ("1-Dec-2216", "native_date_year_outside_due_window"),
        // Not a calendar date.
        ("30-Feb-2108", "native_date_calendar_invalid"),
    ] {
        let xml = with_due(due);
        assert_ne!(xml, receivable(), "{due}: the capture has the row");
        match parse(&xml) {
            Err(NativeOutstandingsError::BillRow { report, row, cause }) => {
                assert_eq!(
                    report, None,
                    "{due}: the report is named by the caller that knows it"
                );
                assert_eq!(row, u32::try_from(LONG_ROW).unwrap(), "{due}");
                assert_eq!(*cause, NativeOutstandingsError::InvalidDate(code), "{due}");
            }
            other => panic!("{due}: {other:?}"),
        }
    }
}

#[test]
fn a_four_digit_year_on_a_bill_date_is_not_the_measured_form_and_refuses() {
    // Only a due date was seen printed in full: a bill date's year in full stays refused.
    let xml = receivable().replacen(
        "<BILLDATE>1-Apr-25</BILLDATE>",
        "<BILLDATE>1-Apr-2125</BILLDATE>",
        1,
    );
    match parse(&xml) {
        Err(NativeOutstandingsError::BillRow { row: 1, cause, .. }) => {
            assert_eq!(
                *cause,
                NativeOutstandingsError::InvalidDate("native_date_year_invalid")
            );
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_row_refusal_names_the_report_and_never_the_bill() {
    let xml = receivable().replacen(
        "<BILLDUE>1-Dec-2108</BILLDUE>",
        "<BILLDUE>1-Dec-2026</BILLDUE>",
        1,
    );
    let error = parse(&xml).unwrap_err().in_report("receivable");
    assert_eq!(error.bill_row(), Some((Some("receivable"), 1467)));
    assert_eq!(error.code(), "native_date_year_invalid");
    // The text names the rule, the report and the row; no party, reference or date.
    let text = error.to_string();
    assert!(
        text.contains("native_date_year_invalid") && text.contains("receivable row 1467"),
        "{text}"
    );
    for secret in ["CPX-05", "Debtor", "PROBE", "2026", "2108"] {
        assert!(
            !text.contains(secret),
            "the error text carries {secret:?}: {text}"
        );
    }
    // An error that is not a row refusal is unchanged by naming a report.
    assert_eq!(
        NativeOutstandingsError::InvalidAmount.in_report("payable"),
        NativeOutstandingsError::InvalidAmount
    );
}

#[test]
fn every_error_has_a_typed_code() {
    use bridge_tally_protocol::native_outstandings::NativeOutstandingsError as E;
    for (error, code) in [
        (
            E::InvalidDate("native_date_shape_invalid"),
            "native_date_shape_invalid",
        ),
        (
            E::InvalidResponse("bills_xml_malformed"),
            "bills_xml_malformed",
        ),
        (E::InvalidAmount, "native_amount_invalid"),
        (E::ArithmeticOverflow, "native_arithmetic_overflow"),
        (E::TallyReportedFailure, "native_tally_reported_failure"),
        (E::StatusAbsent, "native_status_absent"),
        (
            E::ForeignCurrencyLedgerBalance {
                ledger_name: "L".to_string(),
            },
            "native_foreign_currency_ledger_balance",
        ),
    ] {
        assert_eq!(error.code(), code);
        assert_eq!(error.bill_row(), None);
        // A row refusal reports its inner cause's code.
        let wrapped = E::BillRow {
            report: Some("payable"),
            row: 3,
            cause: Box::new(error),
        };
        assert_eq!(wrapped.code(), code);
        assert_eq!(wrapped.bill_row(), Some((Some("payable"), 3)));
    }
}

#[test]
fn the_row_number_is_the_place_of_the_bill_in_the_raw_answer() {
    // Counted from the bytes, not from the parser's own row list: the 1,467th <BILLFIXED> opens
    // the row whose due date is the four-digit one.
    let xml = receivable();
    let at = xml
        .find("<BILLDUE>1-Dec-2108</BILLDUE>")
        .expect("the capture has the row");
    assert_eq!(xml[..at].matches("<BILLFIXED>").count(), LONG_ROW);
    assert_eq!(xml.matches("<BILLDUE>1-Dec-2108</BILLDUE>").count(), 1);
}

#[test]
fn the_four_digit_year_edges_are_exact() {
    // The bill is dated 1-Aug-25, so a due date may fall up to 1-Aug-2115 and from year 2100 in full.
    let with_due = |due: &str| {
        receivable().replacen(
            "<BILLDUE>1-Dec-2108</BILLDUE>",
            &format!("<BILLDUE>{due}</BILLDUE>"),
            1,
        )
    };
    let due_of = |due: &str| -> Result<String, NativeOutstandingsError> {
        parse(&with_due(due)).map(|rows| rows[LONG_ROW - 1].due_date.as_str().to_string())
    };
    let refusal = |due: &str| match parse(&with_due(due)) {
        Err(NativeOutstandingsError::BillRow { cause, .. }) => cause.code(),
        other => panic!("{due}: {other:?}"),
    };
    assert_eq!(due_of("1-Jan-2100").unwrap(), "21000101");
    assert_eq!(due_of("1-Aug-2115").unwrap(), "21150801");
    assert_eq!(due_of("29-Feb-2104").unwrap(), "21040229");
    assert_eq!(refusal("2-Aug-2115"), "native_date_year_outside_due_window");
    assert_eq!(refusal("31-Dec-2099"), "native_date_year_invalid");
    // 2100 is not a leap year.
    assert_eq!(refusal("29-Feb-2100"), "native_date_calendar_invalid");
}

#[test]
fn a_refusal_about_the_book_window_is_not_pinned_on_a_row() {
    // The same window applies to every bill, so no row is to blame for it.
    let xml = receivable();
    for (from, as_of) in [("20260331", "20250401"), ("20250401", "21260401")] {
        match parse_native_bill_rows(&xml, &date(from), &date(as_of)) {
            Err(NativeOutstandingsError::InvalidDate(code)) => assert!(
                code == "native_date_book_window_invalid"
                    || code == "native_date_year_ambiguous_book_window",
                "{code}"
            ),
            other => panic!("{from}..{as_of}: {other:?}"),
        }
    }
}
