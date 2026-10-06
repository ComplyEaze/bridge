//! Tally's own Cash Flow, requested by report name over whole months.
//!
//! A built-in report like the statements (protocol reference §12a.1): a success
//! carries no `HEADER` or `STATUS`, and nothing in the response identifies the
//! company, so a caller must bracket the read with identity and extent reads.
//! Its observed shape on licensed TallyPrime 7.1 is in the provenance note of
//! the `builtin_cash_flow_*` fixtures.
//!
//! The response is one `DSPPERIOD` (a month name, **without a year**) followed
//! by one `DSPACCINFO` per month, holding a debit, a credit and a closing
//! amount. A month with no activity is a row whose amounts are empty, not a
//! missing row, and an empty amount is not zero. Amounts are plain signed
//! decimals, a debit negative. Because the label carries no year, a window is at
//! most twelve whole months and the answer must name exactly the months the
//! window holds, in order: [`WholeMonthWindow`] fixes the years, so a response is
//! never matched to months by guesswork.
//!
//! The parser is closed: anything other than well-formed rows of the expected
//! shape is refused, and a failure, an unknown report, an empty answer and a
//! different set of months each have their own error.
use bridge_tally_primitives::{ExactDecimal, TallyDate};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use serde::Serialize;
use std::fmt;

use crate::{
    native_outstandings::NativeLedgerSnapshotPeriod,
    native_statement_reports::{
        amount, read_text, render_built_in_report_request, NativeStatementAmount,
    },
    outstandings_shared::DateBoundaryProfile,
    tolerant_xml::sanitize_invalid_numeric_references,
};

/// The report name Tally's gateway accepts (live capture, 6 Oct 2026).
const REPORT_ID: &str = "Cash Flow";

/// The most months one window may hold: the label has no year, so beyond twelve
/// two rows could carry the same name.
const MAX_MONTHS: u32 = 12;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// A calendar month, with the year Bridge assigned from the window it sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CashFlowMonth {
    pub year: u16,
    /// 1 for January to 12 for December.
    pub month: u8,
}

impl CashFlowMonth {
    fn name(self) -> &'static str {
        MONTH_NAMES[usize::from(self.month) - 1]
    }
}

/// Whole calendar months: from the first day of one month to the last day of
/// the same or a later month, at most twelve of them. Anything else cannot be
/// matched to the rows Tally returns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WholeMonthWindow {
    period: NativeLedgerSnapshotPeriod,
    months: Vec<CashFlowMonth>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WholeMonthWindowError {
    NotMonthStart,
    NotMonthEnd,
    Reversed,
    TooManyMonths,
    /// A date the mode's boundary profile does not admit (Education mode).
    UnsupportedBoundary,
}

impl WholeMonthWindowError {
    pub fn code(self) -> &'static str {
        match self {
            Self::NotMonthStart => "cash_flow_window_not_month_start",
            Self::NotMonthEnd => "cash_flow_window_not_month_end",
            Self::Reversed => "cash_flow_window_reversed",
            Self::TooManyMonths => "cash_flow_window_too_many_months",
            Self::UnsupportedBoundary => "cash_flow_window_unsupported_boundary",
        }
    }
}

fn year_and_month(date: &TallyDate) -> Option<(u16, u8)> {
    let text = date.as_str();
    Some((text.get(0..4)?.parse().ok()?, text.get(4..6)?.parse().ok()?))
}

impl WholeMonthWindow {
    pub fn new(
        boundary_profile: DateBoundaryProfile,
        from: TallyDate,
        to: TallyDate,
    ) -> Result<Self, WholeMonthWindowError> {
        if from.as_str().get(6..8) != Some("01") {
            return Err(WholeMonthWindowError::NotMonthStart);
        }
        // The last day of a month is the day before a first.
        let is_month_end = to
            .next_day()
            .is_ok_and(|next| next.as_str().get(6..8) == Some("01"));
        if !is_month_end {
            return Err(WholeMonthWindowError::NotMonthEnd);
        }
        if from > to {
            return Err(WholeMonthWindowError::Reversed);
        }
        let (from_year, from_month) =
            year_and_month(&from).ok_or(WholeMonthWindowError::NotMonthStart)?;
        let (to_year, to_month) = year_and_month(&to).ok_or(WholeMonthWindowError::NotMonthEnd)?;
        let span = (u32::from(to_year) - u32::from(from_year)) * 12 + u32::from(to_month)
            - u32::from(from_month)
            + 1;
        if span > MAX_MONTHS {
            return Err(WholeMonthWindowError::TooManyMonths);
        }
        let period = NativeLedgerSnapshotPeriod::new(boundary_profile, from, to)
            .map_err(|_| WholeMonthWindowError::UnsupportedBoundary)?;
        let months = (0..span)
            .map(|offset| {
                let index = u32::from(from_month) - 1 + offset;
                CashFlowMonth {
                    year: from_year + u16::try_from(index / 12).unwrap_or(0),
                    month: u8::try_from(index % 12 + 1).unwrap_or(1),
                }
            })
            .collect();
        Ok(Self { period, months })
    }

    pub fn period(&self) -> &NativeLedgerSnapshotPeriod {
        &self.period
    }

    /// The months the window holds, first to last.
    pub fn months(&self) -> &[CashFlowMonth] {
        &self.months
    }
}

/// Renders the only supported request: Tally's own Cash Flow by name over the
/// window, for the named company. No TDL.
pub fn render_native_cash_flow_request(company: &str, window: &WholeMonthWindow) -> String {
    render_built_in_report_request(REPORT_ID, company, window.period())
}

/// One month's row, each amount exactly as Tally printed it. The closing figure
/// is the month's own, not a running one (May's equalled May's debit alone).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeCashFlowRow {
    pub month: CashFlowMonth,
    pub debit: NativeStatementAmount,
    pub credit: NativeStatementAmount,
    pub closing: NativeStatementAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeCashFlow {
    pub rows: Vec<NativeCashFlowRow>,
}

impl NativeCashFlow {
    /// The debits added up, counting only the amounts present.
    pub fn total_debit(&self) -> Result<ExactDecimal, NativeCashFlowError> {
        sum_present(self.rows.iter().map(|row| &row.debit))
    }

    /// The credits added up, counting only the amounts present.
    pub fn total_credit(&self) -> Result<ExactDecimal, NativeCashFlowError> {
        sum_present(self.rows.iter().map(|row| &row.credit))
    }

    /// How many rows printed no debit, so a total can say what it left out.
    pub fn empty_debit_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.debit == NativeStatementAmount::Empty)
            .count()
    }
}

fn sum_present<'a>(
    amounts: impl Iterator<Item = &'a NativeStatementAmount>,
) -> Result<ExactDecimal, NativeCashFlowError> {
    let mut sum = ExactDecimal::zero();
    for amount in amounts {
        if let NativeStatementAmount::Present(value) = amount {
            sum = sum
                .checked_add(value)
                .map_err(|_| NativeCashFlowError::InvalidResponse("cash_flow_sum_invalid"))?;
        }
    }
    Ok(sum)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeCashFlowError {
    /// A `STATUS` or `LINEERROR` in the response: Tally reported failure.
    /// Tally 7.1 answers an unknown report this way (`STATUS 0` and a
    /// `LINEERROR` inside HEADER, BODY and DATA); the text differs by build, so
    /// the structure is what is classified.
    TallyReportedFailure,
    /// A bare `RESPONSE`: Tally did not recognise the report name (§12a.1).
    UnknownReport,
    /// An empty `ENVELOPE`. Cash Flow always prints its rows, so an empty
    /// answer is a report that was not rendered, never "no activity".
    EmptyEnvelope,
    /// An amount that is not a plain signed decimal (grouped, suffixed...).
    InvalidAmount,
    /// The rows name other months than the window holds, or the same month
    /// twice, or a name that is not a month.
    MonthsUnexpected,
    InvalidResponse(&'static str),
}

impl NativeCashFlowError {
    pub fn code(self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "cash_flow_tally_reported_failure",
            Self::UnknownReport => "cash_flow_report_unknown",
            Self::EmptyEnvelope => "cash_flow_empty_envelope",
            Self::InvalidAmount => "cash_flow_amount_invalid",
            Self::MonthsUnexpected => "cash_flow_months_unexpected",
            Self::InvalidResponse(code) => code,
        }
    }
}

impl fmt::Display for NativeCashFlowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "native cash flow response refused ({})",
            self.code()
        )
    }
}

impl std::error::Error for NativeCashFlowError {}

impl From<crate::native_statement_reports::NativeStatementError> for NativeCashFlowError {
    fn from(error: crate::native_statement_reports::NativeStatementError) -> Self {
        use crate::native_statement_reports::NativeStatementError as Shared;
        match error {
            Shared::TallyReportedFailure => Self::TallyReportedFailure,
            Shared::UnknownReport => Self::UnknownReport,
            Shared::InvalidAmount => Self::InvalidAmount,
            Shared::InvalidResponse(code) => Self::InvalidResponse(code),
        }
    }
}

fn invalid(code: &'static str) -> NativeCashFlowError {
    NativeCashFlowError::InvalidResponse(code)
}

fn malformed() -> NativeCashFlowError {
    invalid("cash_flow_xml_malformed")
}

fn is_failure_signal(name: &[u8]) -> bool {
    matches!(name, b"STATUS" | b"LINEERROR" | b"ERROR")
}

/// The elements Tally wraps a failure in. They are admitted only so the failure
/// signal inside is reached; a wrapper with no signal is refused.
fn is_wrapper(name: &[u8]) -> bool {
    matches!(name, b"HEADER" | b"BODY" | b"DATA")
}

/// Parses a built-in Cash Flow response against the window it was asked for.
/// Refuses a failure signal, an unknown report, an empty answer, any element
/// outside the expected rows, a period without its row (or a row without its
/// period), a row without all three amounts or with one twice, an amount that
/// is not a plain signed decimal, and rows whose months are not exactly the
/// window's, in order.
pub fn parse_native_cash_flow(
    xml: &str,
    window: &WholeMonthWindow,
) -> Result<NativeCashFlow, NativeCashFlowError> {
    let sanitized = sanitize_invalid_numeric_references(xml);
    let mut reader = Reader::from_str(&sanitized);
    reader.config_mut().trim_text(false);
    let mut root_seen = false;
    let mut envelope_closed = false;
    let mut wrapper_depth = 0_usize;
    let mut wrapper_seen = false;
    let mut pending_period: Option<String> = None;
    let mut rows: Vec<(String, [NativeStatementAmount; 3])> = Vec::new();

    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element) => {
                let name = element.name().as_ref().to_ascii_uppercase();
                if !root_seen {
                    match name.as_slice() {
                        b"ENVELOPE" => root_seen = true,
                        b"RESPONSE" => return Err(NativeCashFlowError::UnknownReport),
                        _ => return Err(invalid("cash_flow_root_not_envelope")),
                    }
                    continue;
                }
                if envelope_closed {
                    return Err(invalid("cash_flow_trailing_content"));
                }
                if is_failure_signal(&name) {
                    return Err(NativeCashFlowError::TallyReportedFailure);
                }
                if is_wrapper(&name) {
                    wrapper_depth += 1;
                    wrapper_seen = true;
                } else if wrapper_depth > 0 && name.as_slice() == b"VERSION" {
                    read_text(&mut reader, element.name())?;
                } else if wrapper_depth > 0 {
                    return Err(invalid("cash_flow_unexpected_element"));
                } else if name.as_slice() == b"DSPPERIOD" {
                    if pending_period.is_some() {
                        return Err(invalid("cash_flow_period_without_row"));
                    }
                    pending_period = Some(read_text(&mut reader, element.name())?);
                } else if name.as_slice() == b"DSPACCINFO" {
                    let period = pending_period
                        .take()
                        .ok_or(invalid("cash_flow_row_without_period"))?;
                    rows.push((period, read_row(&mut reader, &element)?));
                } else {
                    return Err(invalid("cash_flow_unexpected_element"));
                }
            }
            Event::Empty(element) => {
                let name = element.name().as_ref().to_ascii_uppercase();
                if !root_seen {
                    return Err(invalid("cash_flow_root_not_envelope"));
                }
                if is_failure_signal(&name) {
                    return Err(NativeCashFlowError::TallyReportedFailure);
                }
                return Err(invalid("cash_flow_unexpected_empty_element"));
            }
            Event::End(element) => {
                let name = element.name().as_ref().to_ascii_uppercase();
                if name.as_slice() == b"ENVELOPE" && wrapper_depth == 0 && !envelope_closed {
                    envelope_closed = true;
                } else if is_wrapper(&name) && wrapper_depth > 0 {
                    wrapper_depth -= 1;
                } else {
                    return Err(malformed());
                }
            }
            Event::Text(text) => {
                if !text.as_ref().iter().all(u8::is_ascii_whitespace) {
                    return Err(invalid("cash_flow_stray_text"));
                }
            }
            Event::Decl(_) | Event::Comment(_) => {}
            Event::Eof => break,
            _ => return Err(invalid("cash_flow_unexpected_content")),
        }
    }
    if !root_seen || !envelope_closed {
        return Err(invalid("cash_flow_envelope_unterminated"));
    }
    if wrapper_seen {
        return Err(invalid("cash_flow_wrapper_without_failure_signal"));
    }
    if pending_period.is_some() {
        return Err(invalid("cash_flow_period_without_row"));
    }
    if rows.is_empty() {
        return Err(NativeCashFlowError::EmptyEnvelope);
    }
    let expected = window.months();
    if rows.len() != expected.len()
        || rows
            .iter()
            .zip(expected)
            .any(|((name, _), month)| name != month.name())
    {
        return Err(NativeCashFlowError::MonthsUnexpected);
    }
    Ok(NativeCashFlow {
        rows: rows
            .into_iter()
            .zip(expected)
            .map(|((_, [debit, credit, closing]), month)| NativeCashFlowRow {
                month: *month,
                debit,
                credit,
                closing,
            })
            .collect(),
    })
}

/// The three columns of one row, each exactly once and nothing else.
fn read_row(
    reader: &mut Reader<&[u8]>,
    block: &BytesStart<'_>,
) -> Result<[NativeStatementAmount; 3], NativeCashFlowError> {
    let block = block.name().as_ref().to_vec();
    let mut debit = None;
    let mut credit = None;
    let mut closing = None;
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element) => {
                let name = element.name().as_ref().to_ascii_uppercase();
                if is_failure_signal(&name) {
                    return Err(NativeCashFlowError::TallyReportedFailure);
                }
                let (slot, inner): (&mut Option<NativeStatementAmount>, &[u8]) =
                    match name.as_slice() {
                        b"DSPDRAMT" => (&mut debit, b"DSPDRAMTA"),
                        b"DSPCRAMT" => (&mut credit, b"DSPCRAMTA"),
                        b"DSPCLAMT" => (&mut closing, b"DSPCLAMTA"),
                        _ => return Err(invalid("cash_flow_unexpected_element")),
                    };
                if slot.is_some() {
                    return Err(invalid("cash_flow_duplicate_column"));
                }
                *slot = Some(read_column(reader, element.name().as_ref(), inner)?);
            }
            Event::End(element) if element.name().as_ref() == block.as_slice() => {
                return match (debit, credit, closing) {
                    (Some(debit), Some(credit), Some(closing)) => Ok([debit, credit, closing]),
                    _ => Err(invalid("cash_flow_column_missing")),
                };
            }
            Event::Text(text) if text.as_ref().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(invalid("cash_flow_row_shape")),
        }
    }
}

/// One column: its outer element holding its amount element once, text or empty.
fn read_column(
    reader: &mut Reader<&[u8]>,
    outer: &[u8],
    inner: &[u8],
) -> Result<NativeStatementAmount, NativeCashFlowError> {
    let outer = outer.to_vec();
    let mut found = None;
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element)
                if element.name().as_ref().eq_ignore_ascii_case(inner) && found.is_none() =>
            {
                found = Some(amount(&read_text(reader, element.name())?)?);
            }
            Event::Empty(element)
                if element.name().as_ref().eq_ignore_ascii_case(inner) && found.is_none() =>
            {
                found = Some(NativeStatementAmount::Empty);
            }
            Event::End(element) if element.name().as_ref() == outer.as_slice() => {
                return found.ok_or(invalid("cash_flow_amount_missing"));
            }
            Event::Text(text) if text.as_ref().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(invalid("cash_flow_column_shape")),
        }
    }
}

#[cfg(test)]
#[path = "native_cash_flow_tests.rs"]
mod tests;
