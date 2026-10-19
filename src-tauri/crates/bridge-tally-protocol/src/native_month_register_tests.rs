use super::*;
use crate::{
    outstandings_shared::DateBoundaryProfile,
    text_encoding::{
        decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
    },
};
use bridge_tally_primitives::{ExactDecimal, TallyDate};

const SALES: &[u8] =
    include_bytes!("../tests/fixtures/builtin_sales_register_pilot_lab_fy_live.utf16le.xml");
const SALES_REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_sales_register_pilot_lab_fy_request.utf16le.xml");
const PURCHASE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_purchase_register_pilot_lab_fy_live.utf16le.xml");
const PURCHASE_REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_purchase_register_pilot_lab_fy_request.utf16le.xml");
// No empty register answer was captured: the empty envelope of another built-in
// report (Negative Ledgers) exercises the shared empty-answer path.
const EMPTY_ENVELOPE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_ledgers_probe_b_fy_empty_live.utf16le.xml");
const UNKNOWN_REPORT: &[u8] =
    include_bytes!("../tests/fixtures/builtin_unknown_report_refusal_live.utf16le.xml");

/// A captured response, decoded as production decodes it.
fn response(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap()
    .text
}

fn window(from: &str, to: &str) -> WholeMonthWindow {
    WholeMonthWindow::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse(from).unwrap(),
        TallyDate::parse(to).unwrap(),
    )
    .unwrap()
}

/// The financial year both captures were asked for.
fn year() -> WholeMonthWindow {
    window("20260401", "20270331")
}

fn present(value: &str) -> NativeStatementAmount {
    NativeStatementAmount::Present(ExactDecimal::parse(value).unwrap())
}

/// The capture on one line, so a mutation matches one string.
fn compact(bytes: &[u8]) -> String {
    response(bytes)
        .split('\n')
        .map(str::trim)
        .collect::<String>()
}

fn mutate(bytes: &[u8], from: &str, to: &str) -> String {
    let text = compact(bytes);
    assert_eq!(text.matches(from).count(), 1, "mutation target: {from}");
    text.replacen(from, to, 1)
}

#[test]
fn each_request_is_byte_equal_to_the_one_the_lab_sent() {
    for (kind, request) in [
        (MonthRegisterKind::Sales, SALES_REQUEST),
        (MonthRegisterKind::Purchase, PURCHASE_REQUEST),
    ] {
        assert_eq!(
            render_native_month_register_request(kind, "BRIDGE PILOT LAB", &year()),
            decode_xml_bytes(request).unwrap(),
            "{kind:?}"
        );
    }
}

#[test]
fn the_report_names_are_the_two_tally_accepted() {
    assert_eq!(MonthRegisterKind::Sales.report_id(), "Sales Register");
    assert_eq!(MonthRegisterKind::Purchase.report_id(), "Purchase Register");
}

/// The pilot lab's Sales Register for its year: twelve month rows, April
/// first; August's sales and the running total after it, then September's,
/// with every other month present and empty (an empty amount is not zero).
#[test]
fn the_captured_sales_register_parses_to_twelve_months_with_a_running_total() {
    let parsed = parse_native_month_register(&response(SALES), &year()).unwrap();
    let months: Vec<_> = parsed
        .rows
        .iter()
        .map(|row| (row.month.year, row.month.month))
        .collect();
    assert_eq!(months.len(), 12);
    assert_eq!(months.first(), Some(&(2026, 4)));
    assert_eq!(months.last(), Some(&(2027, 3)));
    let august = &parsed.rows[4];
    assert_eq!(august.dr, NativeStatementAmount::Empty);
    assert_eq!(august.cr, present("17638.50"));
    assert_eq!(august.cl, present("17638.50"));
    let september = &parsed.rows[5];
    assert_eq!(september.cr, present("210.20"));
    assert_eq!(september.cl, present("17848.70"));
    let with_amounts = parsed
        .rows
        .iter()
        .filter(|row| {
            [&row.dr, &row.cr, &row.cl]
                .iter()
                .any(|amount| **amount != NativeStatementAmount::Empty)
        })
        .count();
    assert_eq!(with_amounts, 2);
}

/// The pilot lab's Purchase Register: August's purchases as a negative debit,
/// and September carrying the closing with no month amount of its own.
#[test]
fn the_captured_purchase_register_carries_its_closing_into_a_month_with_none() {
    let parsed = parse_native_month_register(&response(PURCHASE), &year()).unwrap();
    assert_eq!(parsed.rows.len(), 12);
    let august = &parsed.rows[4];
    assert_eq!(august.dr, present("-2100.00"));
    assert_eq!(august.cr, NativeStatementAmount::Empty);
    assert_eq!(august.cl, present("-2100.00"));
    let september = &parsed.rows[5];
    assert_eq!(september.dr, NativeStatementAmount::Empty);
    assert_eq!(september.cr, NativeStatementAmount::Empty);
    assert_eq!(september.cl, present("-2100.00"));
}

#[test]
fn a_window_that_holds_other_months_is_refused_with_its_own_variant() {
    assert_eq!(
        parse_native_month_register(&response(SALES), &window("20260401", "20260630")),
        Err(NativeMonthRegisterError::MonthsUnexpected)
    );
}

#[test]
fn each_shared_failure_keeps_its_own_variant() {
    assert_eq!(
        parse_native_month_register(&response(UNKNOWN_REPORT), &year()),
        Err(NativeMonthRegisterError::TallyReportedFailure)
    );
    assert_eq!(
        parse_native_month_register(&response(EMPTY_ENVELOPE), &year()),
        Err(NativeMonthRegisterError::EmptyEnvelope)
    );
    // A typed string: no capture of a bare `RESPONSE` is committed (seen on another build).
    assert_eq!(
        parse_native_month_register("<RESPONSE>Unknown Request</RESPONSE>", &year()),
        Err(NativeMonthRegisterError::UnknownReport)
    );
    assert_eq!(
        parse_native_month_register(
            &mutate(
                SALES,
                "<DSPCRAMTA>17638.50</DSPCRAMTA>",
                "<DSPCRAMTA>17,638.50</DSPCRAMTA>"
            ),
            &year()
        ),
        Err(NativeMonthRegisterError::InvalidAmount)
    );
}

#[test]
fn any_other_refusal_is_one_variant_that_names_the_shared_grammars_code() {
    let refused =
        parse_native_month_register(&mutate(SALES, "<DSPPERIOD>August</DSPPERIOD>", ""), &year())
            .unwrap_err();
    assert!(
        matches!(
            refused,
            NativeMonthRegisterError::Refused(_) | NativeMonthRegisterError::MonthsUnexpected
        ),
        "{refused:?}"
    );
    assert_eq!(
        NativeMonthRegisterError::Refused("x").code(),
        "month_register_response_refused"
    );
    assert_eq!(NativeMonthRegisterError::Refused("x").detail(), Some("x"));
    assert_eq!(NativeMonthRegisterError::InvalidAmount.detail(), None);
}

#[test]
fn every_error_has_its_own_code() {
    let codes = [
        NativeMonthRegisterError::TallyReportedFailure,
        NativeMonthRegisterError::UnknownReport,
        NativeMonthRegisterError::EmptyEnvelope,
        NativeMonthRegisterError::InvalidAmount,
        NativeMonthRegisterError::MonthsUnexpected,
        NativeMonthRegisterError::Refused("x"),
    ]
    .map(NativeMonthRegisterError::code);
    let unique: std::collections::BTreeSet<_> = codes.iter().collect();
    assert_eq!(unique.len(), codes.len());
    assert!(codes.iter().all(|code| code.starts_with("month_register_")));
}
