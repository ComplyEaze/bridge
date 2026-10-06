use super::*;
use crate::text_encoding::{
    decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
};
use bridge_tally_primitives::TallyDate;

use crate::outstandings_shared::DateBoundaryProfile;

const CASH_FLOW_FY: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_probe_b_fy_live.utf16le.xml");
const CASH_FLOW_APR_JUN: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_probe_b_apr_jun_live.utf16le.xml");
const CASH_FLOW_JUNE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_probe_b_june_live.utf16le.xml");
const EMPTY_ENVELOPE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_ledgers_probe_b_fy_empty_live.utf16le.xml");
const UNKNOWN_REPORT: &[u8] =
    include_bytes!("../tests/fixtures/builtin_unknown_report_refusal_live.utf16le.xml");
const FY_REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_fy_request.utf16le.xml");
const APR_JUN_REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_apr_jun_request.utf16le.xml");
const JUNE_REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_june_request.utf16le.xml");

const LAB: &str = "BRIDGE PROBE B SANDBOX";

/// A captured request: UTF-16LE with a BOM.
fn text(bytes: &[u8]) -> String {
    decode_xml_bytes(bytes).unwrap()
}

/// A captured response, decoded as production decodes it: BOM-less UTF-16LE
/// under the `charset=utf-16` content type Tally sent.
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

fn date(value: &str) -> TallyDate {
    TallyDate::parse(value).unwrap()
}

fn window(from: &str, to: &str) -> WholeMonthWindow {
    WholeMonthWindow::new(DateBoundaryProfile::ModeAgnostic, date(from), date(to)).unwrap()
}

fn present(value: &str) -> NativeStatementAmount {
    NativeStatementAmount::Present(ExactDecimal::parse(value).unwrap())
}

fn dec(value: &str) -> ExactDecimal {
    ExactDecimal::parse(value).unwrap()
}

fn month(year: u16, month: u8) -> CashFlowMonth {
    CashFlowMonth { year, month }
}

// ---- the window ----

#[test]
fn a_financial_year_window_names_twelve_months_in_order_across_the_new_year() {
    let months: Vec<_> = window("20250401", "20260331")
        .months()
        .iter()
        .map(|m| (m.year, m.month))
        .collect();
    let expected = vec![
        (2025, 4),
        (2025, 5),
        (2025, 6),
        (2025, 7),
        (2025, 8),
        (2025, 9),
        (2025, 10),
        (2025, 11),
        (2025, 12),
        (2026, 1),
        (2026, 2),
        (2026, 3),
    ];
    assert_eq!(months, expected);
}

#[test]
fn a_window_must_start_on_the_first_day_of_a_month() {
    assert_eq!(
        WholeMonthWindow::new(
            DateBoundaryProfile::ModeAgnostic,
            date("20250415"),
            date("20250630")
        ),
        Err(WholeMonthWindowError::NotMonthStart)
    );
}

#[test]
fn a_window_must_end_on_the_last_day_of_a_month_including_a_leap_february() {
    assert_eq!(
        WholeMonthWindow::new(
            DateBoundaryProfile::ModeAgnostic,
            date("20250401"),
            date("20250629")
        ),
        Err(WholeMonthWindowError::NotMonthEnd)
    );
    // 2024 is a leap year: 29 February is a month end, 28 February is not.
    assert!(WholeMonthWindow::new(
        DateBoundaryProfile::ModeAgnostic,
        date("20240201"),
        date("20240229")
    )
    .is_ok());
    assert_eq!(
        WholeMonthWindow::new(
            DateBoundaryProfile::ModeAgnostic,
            date("20240201"),
            date("20240228")
        ),
        Err(WholeMonthWindowError::NotMonthEnd)
    );
}

#[test]
fn a_window_of_thirteen_months_is_refused_and_twelve_is_not() {
    assert_eq!(
        WholeMonthWindow::new(
            DateBoundaryProfile::ModeAgnostic,
            date("20250401"),
            date("20260430")
        ),
        Err(WholeMonthWindowError::TooManyMonths)
    );
    assert!(WholeMonthWindow::new(
        DateBoundaryProfile::ModeAgnostic,
        date("20250501"),
        date("20260430")
    )
    .is_ok());
}

#[test]
fn a_reversed_window_is_refused() {
    assert_eq!(
        WholeMonthWindow::new(
            DateBoundaryProfile::ModeAgnostic,
            date("20250701"),
            date("20250430")
        ),
        Err(WholeMonthWindowError::Reversed)
    );
}

#[test]
fn the_education_profile_refuses_a_day_30_month_end() {
    // Education admits only days 1, 2 and 31; a 30 June end is not one of them.
    assert_eq!(
        WholeMonthWindow::new(
            DateBoundaryProfile::EducationRestricted,
            date("20250401"),
            date("20250630")
        ),
        Err(WholeMonthWindowError::UnsupportedBoundary)
    );
    assert!(WholeMonthWindow::new(
        DateBoundaryProfile::EducationRestricted,
        date("20250401"),
        date("20250731")
    )
    .is_ok());
}

// ---- the request ----

#[test]
fn every_request_is_byte_equal_to_its_committed_capture() {
    for (window, committed) in [
        (window("20250401", "20260331"), FY_REQUEST),
        (window("20250401", "20250630"), APR_JUN_REQUEST),
        (window("20250601", "20250630"), JUNE_REQUEST),
    ] {
        assert_eq!(
            render_native_cash_flow_request(LAB, &window),
            text(committed)
        );
    }
}

// ---- the captured responses ----

#[test]
fn the_captured_year_parses_to_twelve_rows_with_empty_months_kept_as_rows() {
    let parsed =
        parse_native_cash_flow(&response(CASH_FLOW_FY), &window("20250401", "20260331")).unwrap();
    assert_eq!(parsed.rows.len(), 12);
    let april = &parsed.rows[0];
    assert_eq!(april.month, month(2025, 4));
    assert_eq!(april.debit, present("-3864.02"));
    assert_eq!(april.credit, NativeStatementAmount::Empty);
    assert_eq!(april.closing, present("-3864.02"));
    // September to March were printed with every amount empty: a present row,
    // not a missing one, and not zero.
    for row in &parsed.rows[5..] {
        assert_eq!(row.debit, NativeStatementAmount::Empty);
        assert_eq!(row.credit, NativeStatementAmount::Empty);
        assert_eq!(row.closing, NativeStatementAmount::Empty);
    }
    assert_eq!(parsed.rows[5].month, month(2025, 9));
    assert_eq!(parsed.rows[11].month, month(2026, 3));
}

#[test]
fn a_months_closing_is_its_own_figure_not_a_running_one() {
    // May's closing equals May's debit alone, not April's and May's together.
    let parsed =
        parse_native_cash_flow(&response(CASH_FLOW_FY), &window("20250401", "20260331")).unwrap();
    let may = &parsed.rows[1];
    assert_eq!(may.debit, present("-1786801.00"));
    assert_eq!(may.closing, present("-1786801.00"));
}

#[test]
fn the_year_debits_add_up_to_the_cash_and_bank_debits_of_the_trial_balance() {
    // The tie measured on 6 Oct 2026 (provenance note): Cash 5,500.00 plus a
    // bank ledger's 17,970,481.22, read by trial_balance for the same window.
    let parsed =
        parse_native_cash_flow(&response(CASH_FLOW_FY), &window("20250401", "20260331")).unwrap();
    assert!(
        parsed
            .total_debit()
            .unwrap()
            .numeric_eq(&dec("-17975981.22")),
        "sum of the twelve rows"
    );
}

#[test]
fn a_three_month_window_and_a_one_month_window_parse_to_their_own_rows() {
    let quarter = parse_native_cash_flow(
        &response(CASH_FLOW_APR_JUN),
        &window("20250401", "20250630"),
    )
    .unwrap();
    assert_eq!(
        quarter.rows.iter().map(|r| r.month).collect::<Vec<_>>(),
        vec![month(2025, 4), month(2025, 5), month(2025, 6)]
    );
    assert!(quarter
        .total_debit()
        .unwrap()
        .numeric_eq(&dec("-7263013.22")));
    let june =
        parse_native_cash_flow(&response(CASH_FLOW_JUNE), &window("20250601", "20250630")).unwrap();
    assert_eq!(june.rows.len(), 1);
    assert_eq!(june.rows[0].month, month(2025, 6));
    assert_eq!(june.rows[0].debit, present("-5472348.20"));
}

#[test]
fn a_response_for_other_months_than_the_window_asked_is_refused() {
    // The year's answer against a six-month window, the quarter's against the
    // year, and the quarter's against a window that starts a month later.
    for (bytes, from, to) in [
        (CASH_FLOW_FY, "20250401", "20250930"),
        (CASH_FLOW_APR_JUN, "20250401", "20260331"),
        (CASH_FLOW_APR_JUN, "20250501", "20250731"),
        (CASH_FLOW_JUNE, "20250501", "20250531"),
    ] {
        assert_eq!(
            parse_native_cash_flow(&response(bytes), &window(from, to)),
            Err(NativeCashFlowError::MonthsUnexpected),
            "{from}..{to}"
        );
    }
}

#[test]
fn an_empty_envelope_is_refused_because_cash_flow_always_prints_its_rows() {
    // The captured empty envelope came from Negative Ledgers, but the shape is
    // Tally's: nothing in it can tell no activity from a report not rendered.
    assert_eq!(
        parse_native_cash_flow(&response(EMPTY_ENVELOPE), &window("20250401", "20260331")),
        Err(NativeCashFlowError::EmptyEnvelope)
    );
}

#[test]
fn an_unknown_report_name_is_a_reported_failure_by_structure() {
    // Tally 7.1 answered `STATUS 0` and a `LINEERROR` inside HEADER, BODY and
    // DATA, not the bare RESPONSE of an earlier build; classify by structure.
    assert_eq!(
        parse_native_cash_flow(&response(UNKNOWN_REPORT), &window("20250401", "20260331")),
        Err(NativeCashFlowError::TallyReportedFailure)
    );
}

#[test]
fn a_bare_response_is_an_unknown_report() {
    assert_eq!(
        parse_native_cash_flow(
            "<RESPONSE>Unknown Request, cannot be processed</RESPONSE>",
            &window("20250401", "20260331")
        ),
        Err(NativeCashFlowError::UnknownReport)
    );
}

// ---- refusals on the captured bytes, one change each ----

fn year() -> String {
    response(CASH_FLOW_FY)
}

/// The capture with the layout whitespace between elements removed, with one
/// change made to it. The change must be found: a mutation that matches nothing
/// would leave the capture intact and a refusal test would pass for nothing.
fn mutate(xml: &str, from: &str, to: &str) -> String {
    let compact = compact_layout(xml);
    assert!(compact.contains(from), "mutation target not found: {from}");
    compact.replacen(from, to, 1)
}

fn compact_layout(xml: &str) -> String {
    let mut out = String::with_capacity(xml.len());
    let mut pending = String::new();
    for ch in xml.chars() {
        if ch.is_whitespace() {
            pending.push(ch);
        } else {
            // Whitespace between `>` and `<` is layout; anywhere else it is kept.
            if !(out.ends_with('>') && ch == '<') {
                out.push_str(&pending);
            }
            pending.clear();
            out.push(ch);
        }
    }
    out
}

fn parse_year(xml: &str) -> Result<NativeCashFlow, NativeCashFlowError> {
    parse_native_cash_flow(xml, &window("20250401", "20260331"))
}

#[test]
fn a_digit_grouped_or_suffixed_amount_is_refused() {
    for bad in ["-3,864.02", "3864.02 Dr", "-3864.02 Dr", " -3864.02", "1e3"] {
        let xml = mutate(&year(), "-3864.02", bad);
        assert_eq!(
            parse_year(&xml),
            Err(NativeCashFlowError::InvalidAmount),
            "{bad}"
        );
    }
}

#[test]
fn a_month_that_is_not_a_known_month_name_is_refused() {
    let xml = mutate(&year(), "<DSPPERIOD>April", "<DSPPERIOD>Aprilx");
    assert_eq!(parse_year(&xml), Err(NativeCashFlowError::MonthsUnexpected));
    let xml = mutate(&year(), "<DSPPERIOD>April", "<DSPPERIOD>april");
    assert_eq!(parse_year(&xml), Err(NativeCashFlowError::MonthsUnexpected));
}

#[test]
fn a_row_missing_one_of_its_three_columns_is_refused() {
    let xml = mutate(
        &year(),
        "<DSPCLAMT><DSPCLAMTA>-3864.02</DSPCLAMTA></DSPCLAMT>",
        "",
    );
    assert!(matches!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(_))
    ));
}

#[test]
fn a_repeated_column_inside_a_row_is_refused() {
    let xml = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT><DSPDRAMT><DSPDRAMTA>1.00</DSPDRAMTA></DSPDRAMT>",
    );
    assert!(matches!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(_))
    ));
}

#[test]
fn a_period_without_its_row_and_a_row_without_its_period_are_refused() {
    let without_row = mutate(&year(), "<DSPPERIOD>April</DSPPERIOD>", "");
    assert!(parse_year(&without_row).is_err());
    let doubled = mutate(
        &year(),
        "<DSPPERIOD>April</DSPPERIOD>",
        "<DSPPERIOD>April</DSPPERIOD><DSPPERIOD>April</DSPPERIOD>",
    );
    assert!(matches!(
        parse_year(&doubled),
        Err(NativeCashFlowError::InvalidResponse(_))
    ));
}

#[test]
fn a_period_left_without_a_row_at_the_end_is_refused() {
    // Every month has its row, then one more period with nothing after it.
    let xml = mutate(
        &year(),
        "</ENVELOPE>",
        "<DSPPERIOD>March</DSPPERIOD></ENVELOPE>",
    );
    assert_eq!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_period_without_row"
        ))
    );
}

#[test]
fn an_element_outside_the_known_grammar_is_refused() {
    let xml = mutate(&year(), "</ENVELOPE>", "<DSPEXTRA>1</DSPEXTRA></ENVELOPE>");
    assert!(matches!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(_))
    ));
}

#[test]
fn stray_text_and_trailing_content_are_refused() {
    let xml = mutate(&year(), "<DSPPERIOD>April", "stray<DSPPERIOD>April");
    assert!(matches!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(_))
    ));
    let xml = format!("{}<ENVELOPE/>", year());
    assert!(matches!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(_))
    ));
}

#[test]
fn a_truncated_response_is_refused_not_read_as_what_was_received() {
    let full = year();
    let cut = &full[..full.len() / 2];
    assert!(matches!(
        parse_year(cut),
        Err(NativeCashFlowError::InvalidResponse(_))
    ));
}

#[test]
fn a_failure_signal_anywhere_beats_the_rows() {
    let xml = mutate(
        &year(),
        "</ENVELOPE>",
        "<LINEERROR>x</LINEERROR></ENVELOPE>",
    );
    assert_eq!(
        parse_year(&xml),
        Err(NativeCashFlowError::TallyReportedFailure)
    );
}

#[test]
fn a_wrapper_with_no_failure_signal_is_not_read_as_a_report() {
    let xml =
        "<ENVELOPE><HEADER><VERSION>1</VERSION></HEADER><BODY><DATA></DATA></BODY></ENVELOPE>";
    assert!(matches!(
        parse_year(xml),
        Err(NativeCashFlowError::InvalidResponse(_))
    ));
}

#[test]
fn every_error_has_its_own_stable_code() {
    let codes = [
        NativeCashFlowError::TallyReportedFailure.code(),
        NativeCashFlowError::UnknownReport.code(),
        NativeCashFlowError::EmptyEnvelope.code(),
        NativeCashFlowError::InvalidAmount.code(),
        NativeCashFlowError::MonthsUnexpected.code(),
    ];
    let mut unique = codes.to_vec();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), codes.len());
    assert!(codes.iter().all(|code| code.starts_with("cash_flow_")));
}

#[test]
fn a_totals_sum_with_an_empty_amount_counts_only_the_amounts_present() {
    // The year has seven rows with an empty debit: the total is the five
    // present ones, and `empty_debit_count` says how many were left out.
    let parsed = parse_year(&year()).unwrap();
    assert_eq!(parsed.empty_debit_count(), 7);
}
