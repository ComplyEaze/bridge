//! Tally's own month-wise Sales Register and Purchase Register, requested by
//! report name over whole months.
//!
//! Each answer has exactly the grammar of the Cash Flow's (protocol reference
//! §12a.16): one `DSPPERIOD` (a month name, **without a year**) then one
//! `DSPACCINFO` per month with a `DSPDRAMT`, a `DSPCRAMT` and a `DSPCLAMT`. The
//! closed parser and the window that places each row in its year are the Cash
//! Flow's, used as they are ([`crate::native_cash_flow`]), as the Funds Flow's
//! are; this module adds the two requests and keeps Tally's three columns under
//! neutral names.
//!
//! Captured once each, on one synthetic book's financial year (BRIDGE PILOT
//! LAB, 13 Oct 2026). On that book the Sales Register's `Cr` was the month's
//! sales invoices, tax included, neither cancelled nor optional, and `CL` a
//! running total printed for August and September only, the other months
//! empty; the Purchase Register's `Dr` was the month's purchases as a negative
//! amount, and its `CL` printed for August and, unchanged, for September, a
//! month with no purchase. Nothing here applies that reading or ties
//! the figures to the vouchers: the rows are returned as Tally printed them.
//! The Journal Register answers another shape (a count of vouchers in the
//! debit column, no closing column) and is not read here.
use serde::Serialize;
use std::fmt;

use crate::{
    native_cash_flow::{
        parse_native_cash_flow, CashFlowMonth, NativeCashFlowError, WholeMonthWindow,
    },
    native_statement_reports::{
        render_built_in_report_request, BuiltInReport, NativeStatementAmount,
    },
};

/// Which month-wise register: each is Tally's own report, by its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MonthRegisterKind {
    Sales,
    Purchase,
}

impl MonthRegisterKind {
    /// The report name Tally's gateway accepted (live capture, 13 Oct 2026).
    pub fn report_id(self) -> &'static str {
        BuiltInReport::from(self).id()
    }
}

impl From<MonthRegisterKind> for BuiltInReport {
    fn from(kind: MonthRegisterKind) -> Self {
        match kind {
            MonthRegisterKind::Sales => Self::SalesRegister,
            MonthRegisterKind::Purchase => Self::PurchaseRegister,
        }
    }
}

/// Renders the only supported request: Tally's own register by name over the
/// window, for the named company. No TDL.
pub fn render_native_month_register_request(
    kind: MonthRegisterKind,
    company: &str,
    window: &WholeMonthWindow,
) -> String {
    render_built_in_report_request(kind.into(), company, window.period())
}

/// One month's row, each column exactly as Tally printed it under its own
/// name. An empty amount is not zero.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeMonthRegisterRow {
    pub month: CashFlowMonth,
    /// `DSPDRAMT`.
    pub dr: NativeStatementAmount,
    /// `DSPCRAMT`.
    pub cr: NativeStatementAmount,
    /// `DSPCLAMT`.
    pub cl: NativeStatementAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeMonthRegister {
    pub rows: Vec<NativeMonthRegisterRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeMonthRegisterError {
    /// A `STATUS` or `LINEERROR` in the response: Tally reported failure.
    TallyReportedFailure,
    /// A bare `RESPONSE`: Tally did not recognise the report name (§12a.1).
    UnknownReport,
    /// An empty `ENVELOPE`: treated as a report that was not rendered, never as
    /// "no sales" (the captured registers printed every month, empty ones
    /// included).
    EmptyEnvelope,
    /// An amount that is not a plain signed decimal.
    InvalidAmount,
    /// The rows name other months than the window holds.
    MonthsUnexpected,
    /// Any other refusal of the shared grammar; [`Self::detail`] names it.
    Refused(&'static str),
}

impl NativeMonthRegisterError {
    pub fn code(self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "month_register_tally_reported_failure",
            Self::UnknownReport => "month_register_report_unknown",
            Self::EmptyEnvelope => "month_register_empty_envelope",
            Self::InvalidAmount => "month_register_amount_invalid",
            Self::MonthsUnexpected => "month_register_months_unexpected",
            Self::Refused(_) => "month_register_response_refused",
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

impl fmt::Display for NativeMonthRegisterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "native month register response refused ({})",
            self.code()
        )
    }
}

impl std::error::Error for NativeMonthRegisterError {}

impl From<NativeCashFlowError> for NativeMonthRegisterError {
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

/// Parses a built-in Sales or Purchase Register response against the window it
/// was asked for, with the Cash Flow's closed grammar: every failure, unknown
/// report, empty answer, stray element, repeated or missing column, bad amount
/// and wrong set of months is refused.
pub fn parse_native_month_register(
    xml: &str,
    window: &WholeMonthWindow,
) -> Result<NativeMonthRegister, NativeMonthRegisterError> {
    let parsed = parse_native_cash_flow(xml, window)?;
    Ok(NativeMonthRegister {
        rows: parsed
            .rows
            .into_iter()
            .map(|row| NativeMonthRegisterRow {
                month: row.month,
                dr: row.debit,
                cr: row.credit,
                cl: row.closing,
            })
            .collect(),
    })
}

#[cfg(test)]
#[path = "native_month_register_tests.rs"]
mod tests;
