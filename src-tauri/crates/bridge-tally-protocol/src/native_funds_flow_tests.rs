use super::*;
use crate::{
    outstandings_shared::DateBoundaryProfile,
    text_encoding::{
        decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
    },
};
use bridge_tally_primitives::{ExactDecimal, TallyDate};

const LIVE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_funds_flow_probe_b_fy_live.utf16le.xml");
const REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_funds_flow_fy_request.utf16le.xml");
// A stand-in: no empty Funds Flow answer was captured, so the empty envelope of another
// built-in report (Negative Ledgers) exercises the shared empty-answer path.
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

fn year() -> WholeMonthWindow {
    window("20250401", "20260331")
}

fn present(value: &str) -> NativeStatementAmount {
    NativeStatementAmount::Present(ExactDecimal::parse(value).unwrap())
}

/// The capture on one line, so a mutation matches one string.
fn compact() -> String {
    response(LIVE)
        .split('\n')
        .map(str::trim)
        .collect::<String>()
}

fn mutate(from: &str, to: &str) -> String {
    let text = compact();
    assert!(text.contains(from), "mutation target not found: {from}");
    text.replacen(from, to, 1)
}

#[test]
fn the_request_is_byte_equal_to_its_committed_capture() {
    assert_eq!(
        render_native_funds_flow_request("BRIDGE PROBE B SANDBOX", &year()),
        decode_xml_bytes(REQUEST).unwrap()
    );
}

#[test]
fn the_captured_year_parses_to_twelve_month_rows_in_order() {
    let parsed = parse_native_funds_flow(&response(LIVE), &year()).unwrap();
    assert_eq!(parsed.rows.len(), 12);
    let months: Vec<_> = parsed
        .rows
        .iter()
        .map(|row| (row.month.year, row.month.month))
        .collect();
    assert_eq!(months.first(), Some(&(2025, 4)));
    assert_eq!(months.last(), Some(&(2026, 3)));
    let april = &parsed.rows[0];
    assert_eq!(april.dr, present("-1.00"));
    assert_eq!(april.cr, present("-7292109.25"));
    assert_eq!(april.cl, present("-7292108.25"));
    let july = &parsed.rows[3];
    assert_eq!(july.cl, present("14509.81"));
}

#[test]
fn an_empty_column_stays_empty_and_is_not_zero() {
    // September's three figures: Dr and Cr equal, and CL printed empty.
    let parsed = parse_native_funds_flow(&response(LIVE), &year()).unwrap();
    let september = &parsed.rows[5];
    assert_eq!(september.dr, present("-63117592.83"));
    assert_eq!(september.cr, present("-63117592.83"));
    assert_eq!(september.cl, NativeStatementAmount::Empty);
}

#[test]
fn a_window_that_holds_other_months_is_refused_with_its_own_variant() {
    let quarter = window("20250401", "20250630");
    assert_eq!(
        parse_native_funds_flow(&response(LIVE), &quarter),
        Err(NativeFundsFlowError::MonthsUnexpected)
    );
}

#[test]
fn each_shared_failure_keeps_its_own_variant() {
    assert_eq!(
        parse_native_funds_flow(&response(UNKNOWN_REPORT), &year()),
        Err(NativeFundsFlowError::TallyReportedFailure)
    );
    assert_eq!(
        parse_native_funds_flow(&response(EMPTY_ENVELOPE), &year()),
        Err(NativeFundsFlowError::EmptyEnvelope)
    );
    // A typed string: no capture of a bare `RESPONSE` is committed (seen on another build).
    assert_eq!(
        parse_native_funds_flow("<RESPONSE>Unknown Request</RESPONSE>", &year()),
        Err(NativeFundsFlowError::UnknownReport)
    );
    assert_eq!(
        parse_native_funds_flow(
            &mutate(
                "<DSPDRAMTA>-1.00</DSPDRAMTA>",
                "<DSPDRAMTA>-1,00</DSPDRAMTA>"
            ),
            &year()
        ),
        Err(NativeFundsFlowError::InvalidAmount)
    );
}

#[test]
fn any_other_refusal_is_one_variant_that_names_the_shared_grammars_code() {
    let refused = parse_native_funds_flow("<ENVELOPE><DSPEXTRA/></ENVELOPE>", &year());
    assert_eq!(
        refused,
        Err(NativeFundsFlowError::Refused(
            "cash_flow_unexpected_empty_element"
        ))
    );
    let error = refused.unwrap_err();
    assert_eq!(error.code(), "funds_flow_response_refused");
    assert_eq!(error.detail(), Some("cash_flow_unexpected_empty_element"));
    // A column repeated inside a row, and a row without its period, reach the same variant.
    let repeated = mutate(
        "<DSPDRAMT><DSPDRAMTA>-1.00</DSPDRAMTA></DSPDRAMT>",
        "<DSPDRAMT><DSPDRAMTA>-1.00</DSPDRAMTA></DSPDRAMT><DSPDRAMT><DSPDRAMTA>2.00</DSPDRAMTA></DSPDRAMT>",
    );
    assert_eq!(
        parse_native_funds_flow(&repeated, &year()),
        Err(NativeFundsFlowError::Refused("cash_flow_duplicate_column"))
    );
    let no_row = mutate("<DSPPERIOD>April</DSPPERIOD>", "");
    assert_eq!(
        parse_native_funds_flow(&no_row, &year()),
        Err(NativeFundsFlowError::Refused(
            "cash_flow_row_without_period"
        ))
    );
}

#[test]
fn every_variant_has_its_own_stable_code() {
    assert_eq!(
        [
            NativeFundsFlowError::TallyReportedFailure.code(),
            NativeFundsFlowError::UnknownReport.code(),
            NativeFundsFlowError::EmptyEnvelope.code(),
            NativeFundsFlowError::InvalidAmount.code(),
            NativeFundsFlowError::MonthsUnexpected.code(),
            NativeFundsFlowError::Refused("x").code(),
        ],
        [
            "funds_flow_tally_reported_failure",
            "funds_flow_report_unknown",
            "funds_flow_empty_envelope",
            "funds_flow_amount_invalid",
            "funds_flow_months_unexpected",
            "funds_flow_response_refused",
        ]
    );
    assert_eq!(NativeFundsFlowError::EmptyEnvelope.detail(), None);
}
