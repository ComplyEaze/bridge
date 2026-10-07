use super::*;
use crate::text_encoding::{
    decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
};
use bridge_tally_primitives::{ExactDecimal, TallyDate};

use crate::outstandings_shared::DateBoundaryProfile;

const CASH_FLOW_FY: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_probe_b_fy_live.utf16le.xml");
const CASH_FLOW_APR_JUN: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_probe_b_apr_jun_live.utf16le.xml");
const CASH_FLOW_JUNE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_probe_b_june_live.utf16le.xml");
const AMEND_APR_SEP: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_amend_lab_apr_sep_live.utf16le.xml");
const SHAPE_FY: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_shape_lab_fy_live.utf16le.xml");
const CORPUS_DENSE_FY: &[u8] =
    include_bytes!("../tests/fixtures/builtin_cash_flow_corpus_dense_fy_empty_live.utf16le.xml");
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

/// The debits of a parsed answer added up, in the test: the figure the capture
/// carries, which the check in `reports::cash_flow` compares with the trial balance.
fn debit_sum(parsed: &NativeCashFlow) -> ExactDecimal {
    let mut sum = ExactDecimal::zero();
    for row in &parsed.rows {
        if let NativeStatementAmount::Present(value) = &row.debit {
            sum = sum.checked_add(value).unwrap();
        }
    }
    sum
}

#[test]
fn the_captured_year_debits_add_up_to_the_figure_measured_against_the_trial_balance() {
    // Cash 5,500.00 plus a bank ledger's 17,970,481.22: the totals were read by
    // `trial_balance` for the same window (provenance note); the tie itself is
    // checked in `reports::cash_flow`.
    let parsed =
        parse_native_cash_flow(&response(CASH_FLOW_FY), &window("20250401", "20260331")).unwrap();
    assert!(debit_sum(&parsed).numeric_eq(&dec("-17975981.22")));
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
    assert!(debit_sum(&quarter).numeric_eq(&dec("-7263013.22")));
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
    // Tally 7.1 answered `STATUS 0` in a HEADER and a `LINEERROR` inside BODY and
    // DATA, not the bare RESPONSE of an earlier build; classify by structure.
    assert_eq!(
        parse_native_cash_flow(&response(UNKNOWN_REPORT), &window("20250401", "20260331")),
        Err(NativeCashFlowError::TallyReportedFailure)
    );
}

#[test]
fn a_self_closed_root_is_an_empty_envelope() {
    assert_eq!(
        parse_native_cash_flow("<ENVELOPE/>", &window("20250401", "20260331")),
        Err(NativeCashFlowError::EmptyEnvelope)
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
    assert_eq!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_column_missing"
        ))
    );
}

#[test]
fn a_repeated_column_inside_a_row_is_refused() {
    let xml = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT><DSPDRAMT><DSPDRAMTA>1.00</DSPDRAMTA></DSPDRAMT>",
    );
    assert_eq!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_duplicate_column"
        ))
    );
}

#[test]
fn a_period_without_its_row_and_a_row_without_its_period_are_refused() {
    let without_period = mutate(&year(), "<DSPPERIOD>April</DSPPERIOD>", "");
    assert_eq!(
        parse_year(&without_period),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_row_without_period"
        ))
    );
    let doubled = mutate(
        &year(),
        "<DSPPERIOD>April</DSPPERIOD>",
        "<DSPPERIOD>April</DSPPERIOD><DSPPERIOD>April</DSPPERIOD>",
    );
    assert_eq!(
        parse_year(&doubled),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_period_without_row"
        ))
    );
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
    assert_eq!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_unexpected_element"
        ))
    );
}

#[test]
fn stray_text_and_trailing_content_are_refused() {
    let xml = mutate(&year(), "<DSPPERIOD>April", "stray<DSPPERIOD>April");
    assert_eq!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse("cash_flow_stray_text"))
    );
    // A second root, and a start element after the root has closed.
    let xml = format!("{}<DSPPERIOD>April</DSPPERIOD>", year());
    assert_eq!(
        parse_year(&xml),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_trailing_content"
        ))
    );
}

#[test]
fn a_truncated_response_is_refused_not_read_as_what_was_received() {
    // Cut between elements, after the first row: the root never closes.
    let full = mutate(&year(), "", "");
    let end = full.find("</DSPACCINFO>").unwrap() + "</DSPACCINFO>".len();
    assert_eq!(
        parse_year(&full[..end]),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_envelope_unterminated"
        ))
    );
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
fn a_self_closed_failure_signal_beats_the_rows() {
    for signal in ["<LINEERROR/>", "<STATUS/>", "<ERROR/>"] {
        let xml = mutate(&year(), "</ENVELOPE>", &format!("{signal}</ENVELOPE>"));
        assert_eq!(
            parse_year(&xml),
            Err(NativeCashFlowError::TallyReportedFailure),
            "{signal}"
        );
    }
}

#[test]
fn a_failure_signal_inside_a_row_beats_the_row() {
    let xml = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "<LINEERROR>x</LINEERROR>",
    );
    assert_eq!(
        parse_year(&xml),
        Err(NativeCashFlowError::TallyReportedFailure)
    );
}

fn invalid_response(code: &'static str) -> Result<NativeCashFlow, NativeCashFlowError> {
    Err(NativeCashFlowError::InvalidResponse(code))
}

#[test]
fn a_root_that_is_not_an_envelope_is_refused_whether_it_is_empty_or_not() {
    for xml in ["<REPORT/>", "<REPORT></REPORT>"] {
        assert_eq!(
            parse_year(xml),
            invalid_response("cash_flow_root_not_envelope")
        );
    }
}

#[test]
fn an_unexpected_self_closed_element_is_refused() {
    assert_eq!(
        parse_year("<ENVELOPE><DSPEXTRA/></ENVELOPE>"),
        invalid_response("cash_flow_unexpected_empty_element")
    );
}

#[test]
fn cdata_and_a_doctype_are_not_content_of_a_report() {
    for xml in [
        "<ENVELOPE><![CDATA[x]]></ENVELOPE>",
        "<ENVELOPE><!DOCTYPE x></ENVELOPE>",
    ] {
        assert_eq!(
            parse_year(xml),
            invalid_response("cash_flow_unexpected_content")
        );
    }
}

#[test]
fn text_or_an_unknown_element_where_a_column_belongs_is_refused() {
    let in_row = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "stray",
    );
    assert_eq!(parse_year(&in_row), invalid_response("cash_flow_row_shape"));
    let in_column = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "<DSPDRAMT>stray</DSPDRAMT>",
    );
    assert_eq!(
        parse_year(&in_column),
        invalid_response("cash_flow_column_shape")
    );
    // A failure signal is looked for between columns, not inside one: here the
    // column is refused for its shape.
    let signal_in_column = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "<DSPDRAMT><LINEERROR>x</LINEERROR></DSPDRAMT>",
    );
    assert_eq!(
        parse_year(&signal_in_column),
        invalid_response("cash_flow_column_shape")
    );
}

#[test]
fn a_column_without_its_amount_element_is_refused_and_a_self_closed_amount_is_empty() {
    let missing = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "<DSPDRAMT></DSPDRAMT>",
    );
    assert_eq!(
        parse_year(&missing),
        invalid_response("cash_flow_amount_missing")
    );
    let self_closed = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "<DSPDRAMT><DSPDRAMTA/></DSPDRAMT>",
    );
    let parsed = parse_year(&self_closed).unwrap();
    assert_eq!(parsed.rows[0].debit, NativeStatementAmount::Empty);
}

#[test]
fn a_second_amount_element_in_one_column_is_refused_not_last_value_wins() {
    let two = mutate(
        &year(),
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA></DSPDRAMT>",
        "<DSPDRAMT><DSPDRAMTA>-3864.02</DSPDRAMTA><DSPDRAMTA>1.00</DSPDRAMTA></DSPDRAMT>",
    );
    assert_eq!(parse_year(&two), invalid_response("cash_flow_column_shape"));
}

#[test]
fn swapped_and_repeated_months_are_not_the_windows_months() {
    for xml in [
        // April twice, May gone.
        mutate(
            &year(),
            "<DSPPERIOD>May</DSPPERIOD>",
            "<DSPPERIOD>April</DSPPERIOD>",
        ),
        // May and April in each other's place.
        {
            let marked = mutate(
                &year(),
                "<DSPPERIOD>April</DSPPERIOD>",
                "<DSPPERIOD>@@</DSPPERIOD>",
            );
            marked
                .replacen(
                    "<DSPPERIOD>May</DSPPERIOD>",
                    "<DSPPERIOD>April</DSPPERIOD>",
                    1,
                )
                .replacen("<DSPPERIOD>@@</DSPPERIOD>", "<DSPPERIOD>May</DSPPERIOD>", 1)
        },
    ] {
        assert_eq!(parse_year(&xml), Err(NativeCashFlowError::MonthsUnexpected));
    }
}

#[test]
fn a_response_cut_inside_an_element_is_refused() {
    let full = mutate(&year(), "", "");
    let cut = full.find("<DSPDRAMTA>").unwrap() + "<DSPDRAMTA>-38".len();
    assert!(
        matches!(
            parse_year(&full[..cut]),
            Err(NativeCashFlowError::InvalidResponse(_))
        ),
        "{:?}",
        parse_year(&full[..cut])
    );
}

#[test]
fn a_wrapper_with_no_failure_signal_is_not_read_as_a_report() {
    let xml =
        "<ENVELOPE><HEADER><VERSION>1</VERSION></HEADER><BODY><DATA></DATA></BODY></ENVELOPE>";
    assert_eq!(
        parse_year(xml),
        Err(NativeCashFlowError::InvalidResponse(
            "cash_flow_wrapper_without_failure_signal"
        ))
    );
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

// ---- the answers of 2026-10-07 on three more lab books ----

/// A row's debit and credit added: what Tally printed as the closing.
fn debit_plus_credit(row: &NativeCashFlowRow) -> ExactDecimal {
    let part = |amount: &NativeStatementAmount| match amount {
        NativeStatementAmount::Present(value) => value.clone(),
        NativeStatementAmount::Empty => ExactDecimal::zero(),
    };
    part(&row.debit).checked_add(&part(&row.credit)).unwrap()
}

#[test]
fn every_captured_row_with_a_credit_closes_at_its_debit_plus_its_credit() {
    // BRIDGE AMEND LAB (April to September 2026) and BRIDGE SHAPE LAB (year 2025-26): months with a
    // credit only, months with both columns, and a positive closing. A debit is negative.
    for (bytes, from, to) in [
        (AMEND_APR_SEP, "20260401", "20260930"),
        (SHAPE_FY, "20250401", "20260331"),
    ] {
        let parsed = parse_native_cash_flow(&response(bytes), &window(from, to)).unwrap();
        let mut with_amounts = 0;
        for row in &parsed.rows {
            if row.closing == NativeStatementAmount::Empty {
                assert_eq!(row.debit, NativeStatementAmount::Empty);
                assert_eq!(row.credit, NativeStatementAmount::Empty);
                continue;
            }
            with_amounts += 1;
            let NativeStatementAmount::Present(closing) = &row.closing else {
                unreachable!()
            };
            assert!(
                closing.numeric_eq(&debit_plus_credit(row)),
                "{:?}",
                row.month
            );
        }
        assert!(with_amounts >= 3, "{with_amounts} rows carried amounts");
        // The equation is only evidence if a credit was there to add.
        assert!(
            parsed
                .rows
                .iter()
                .any(|row| matches!(row.credit, NativeStatementAmount::Present(_))),
            "no credit amount in the capture"
        );
    }
}

#[test]
fn a_captured_credit_only_month_keeps_its_debit_empty_and_a_mixed_month_keeps_both() {
    let amend =
        parse_native_cash_flow(&response(AMEND_APR_SEP), &window("20260401", "20260930")).unwrap();
    // April 2026: a credit and nothing else.
    assert_eq!(amend.rows[0].month, month(2026, 4));
    assert_eq!(amend.rows[0].debit, NativeStatementAmount::Empty);
    assert_eq!(amend.rows[0].credit, present("21371.00"));
    assert_eq!(amend.rows[0].closing, present("21371.00"));
    // July 2026: both, and the closing is their net.
    assert_eq!(amend.rows[3].month, month(2026, 7));
    assert_eq!(amend.rows[3].debit, present("-33501.00"));
    assert_eq!(amend.rows[3].credit, present("15810.00"));
    assert_eq!(amend.rows[3].closing, present("-17691.00"));
    let shape =
        parse_native_cash_flow(&response(SHAPE_FY), &window("20250401", "20260331")).unwrap();
    // June 2025 closes positive: credits above debits.
    assert_eq!(shape.rows[2].month, month(2025, 6));
    assert_eq!(shape.rows[2].closing, present("5500.00"));
}

#[test]
fn a_whole_year_with_every_amount_empty_parses_to_twelve_empty_rows() {
    // BRIDGE CORPUS DENSE: Tally's answer for a year in which the lab's earlier reading found no cash
    // or bank activity. Empty is kept as empty, twelve times: not zero, and not a missing row.
    let parsed =
        parse_native_cash_flow(&response(CORPUS_DENSE_FY), &window("20250401", "20260331"))
            .unwrap();
    assert_eq!(parsed.rows.len(), 12);
    assert_eq!(parsed.rows[0].month, month(2025, 4));
    assert_eq!(parsed.rows[11].month, month(2026, 3));
    for row in &parsed.rows {
        assert_eq!(row.debit, NativeStatementAmount::Empty);
        assert_eq!(row.credit, NativeStatementAmount::Empty);
        assert_eq!(row.closing, NativeStatementAmount::Empty);
    }
}
