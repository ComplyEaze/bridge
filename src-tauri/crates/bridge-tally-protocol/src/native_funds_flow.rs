//! Tally's own Funds Flow, requested by report name over whole months.
//!
//! Its answer has exactly the grammar of the Cash Flow's (protocol reference
//! §12a.16): one `DSPPERIOD` (a month name, **without a year**) then one
//! `DSPACCINFO` per month with a `DSPDRAMT`, a `DSPCRAMT` and a `DSPCLAMT`. The
//! closed parser and the window that places each row in its year are therefore
//! the Cash Flow's, used as they are ([`crate::native_cash_flow`]); this module
//! adds the request and keeps Tally's three columns under neutral names.
//!
//! Only one answer was captured: the twelve months of one financial year of one
//! book. A shorter or year-crossing window, and a book with no working-capital
//! ledger, were not captured; the window type accepts any whole-month window of
//! up to twelve months because the Cash Flow's does.
//!
//! What the columns mean is NOT decided here. On the one book measured, `Dr`
//! equalled the working capital at the start of the month, `Cr` at its end, and
//! `CL` was `Cr - Dr`, with June tying to the trial balance's debtors, bank and
//! cash (a recorded reading: the trial balance responses were not kept); which
//! groups Tally counts as working capital was not measured. Nothing
//! here applies that reading, and nothing ties the figures to anything.
use serde::Serialize;
use std::fmt;

use crate::{
    native_cash_flow::{
        parse_native_cash_flow, CashFlowMonth, NativeCashFlowError, WholeMonthWindow,
    },
    native_statement_reports::{render_built_in_report_request, NativeStatementAmount},
};

/// The report name Tally's gateway accepts (live capture, 6 Oct 2026).
const REPORT_ID: &str = "Funds Flow";

/// Renders the only supported request: Tally's own Funds Flow by name over the
/// window, for the named company. No TDL.
pub fn render_native_funds_flow_request(company: &str, window: &WholeMonthWindow) -> String {
    render_built_in_report_request(REPORT_ID, company, window.period())
}

/// One month's row, each column exactly as Tally printed it under its own
/// name. An empty amount is not zero. On the one capture an empty `CL` appears
/// in the five months where `Cr` equals `Dr` and in no other; it is still kept
/// as empty, never read as zero.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeFundsFlowRow {
    pub month: CashFlowMonth,
    /// `DSPDRAMT`.
    pub dr: NativeStatementAmount,
    /// `DSPCRAMT`.
    pub cr: NativeStatementAmount,
    /// `DSPCLAMT`.
    pub cl: NativeStatementAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeFundsFlow {
    pub rows: Vec<NativeFundsFlowRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeFundsFlowError {
    /// A `STATUS` or `LINEERROR` in the response: Tally reported failure.
    TallyReportedFailure,
    /// A bare `RESPONSE`: Tally did not recognise the report name (§12a.1).
    UnknownReport,
    /// An empty `ENVELOPE`: treated as a report that was not rendered, never as
    /// "no activity" (no empty Funds Flow answer was captured; the Cash Flow's
    /// months all printed rows, empty ones included).
    EmptyEnvelope,
    /// An amount that is not a plain signed decimal.
    InvalidAmount,
    /// The rows name other months than the window holds.
    MonthsUnexpected,
    /// Any other refusal of the shared grammar; [`Self::detail`] names it.
    Refused(&'static str),
}

impl NativeFundsFlowError {
    pub fn code(self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "funds_flow_tally_reported_failure",
            Self::UnknownReport => "funds_flow_report_unknown",
            Self::EmptyEnvelope => "funds_flow_empty_envelope",
            Self::InvalidAmount => "funds_flow_amount_invalid",
            Self::MonthsUnexpected => "funds_flow_months_unexpected",
            Self::Refused(_) => "funds_flow_response_refused",
        }
    }

    /// The shared grammar's own code for a refusal that has no variant here.
    pub fn detail(self) -> Option<&'static str> {
        match self {
            Self::Refused(code) => Some(code),
            _ => None,
        }
    }
}

impl fmt::Display for NativeFundsFlowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "native funds flow response refused ({})",
            self.code()
        )
    }
}

impl std::error::Error for NativeFundsFlowError {}

impl From<NativeCashFlowError> for NativeFundsFlowError {
    fn from(error: NativeCashFlowError) -> Self {
        match error {
            NativeCashFlowError::TallyReportedFailure => Self::TallyReportedFailure,
            NativeCashFlowError::UnknownReport => Self::UnknownReport,
            NativeCashFlowError::EmptyEnvelope => Self::EmptyEnvelope,
            NativeCashFlowError::InvalidAmount => Self::InvalidAmount,
            NativeCashFlowError::MonthsUnexpected => Self::MonthsUnexpected,
            NativeCashFlowError::InvalidResponse(code) => Self::Refused(code),
        }
    }
}

/// Parses a built-in Funds Flow response against the window it was asked for,
/// with the Cash Flow's closed grammar: every failure, unknown report, empty
/// answer, stray element, repeated or missing column, bad amount and wrong set
/// of months is refused.
pub fn parse_native_funds_flow(
    xml: &str,
    window: &WholeMonthWindow,
) -> Result<NativeFundsFlow, NativeFundsFlowError> {
    let parsed = parse_native_cash_flow(xml, window)?;
    Ok(NativeFundsFlow {
        rows: parsed
            .rows
            .into_iter()
            .map(|row| NativeFundsFlowRow {
                month: row.month,
                dr: row.debit,
                cr: row.credit,
                cl: row.closing,
            })
            .collect(),
    })
}

#[cfg(test)]
#[path = "native_funds_flow_tests.rs"]
mod tests;
