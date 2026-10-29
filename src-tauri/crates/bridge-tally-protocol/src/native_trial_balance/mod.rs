//! Native Trial Balance source contract and closed request profile.
//! See TALLY_PROTOCOL_REFERENCE section 5.6 for observed field semantics.
use crate::xml_text::escape_text as xml_escape;
use crate::{native_outstandings::NativeLedgerSnapshotPeriod, PartyLedgerMasterFieldObservation};
use bridge_tally_primitives::ExactDecimal;
use serde::Serialize;
use std::fmt;
mod scalar;
mod wire;
// One classifier for every reader, shared with `vouchers` (bridge#674).
pub(crate) use crate::currency_composite::is_balance_composite;
#[cfg(test)]
pub(crate) use crate::currency_composite::is_currency_composite;
pub(crate) use wire::guid_suffix_is_valid;
pub use wire::{parse_native_trial_balance, parse_native_trial_balance_with_currency};

/// One amount exactly as the native collection exposed it.
///
/// An empty `TYPE="Amount"` element is source evidence and is never coerced
/// to numeric zero. A missing element is a malformed Trial Balance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", content = "value", rename_all = "snake_case")]
pub enum NativeTrialBalanceAmount {
    Present(ExactDecimal),
    PresentEmpty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTrialBalanceRow {
    pub name: String,
    pub guid: String,
    pub parent: PartyLedgerMasterFieldObservation,
    pub opening: NativeTrialBalanceAmount,
    pub debit: NativeTrialBalanceAmount,
    pub credit: NativeTrialBalanceAmount,
    pub closing: NativeTrialBalanceAmount,
}

/// Whether a row's closing is its opening plus its debit and credit totals (§5.6).
///
/// Tally holds a ledger's closing and its movements as separate fields, and on
/// every captured Trial Balance they agree for every ledger but one kind: a
/// Closing Stock ledger whose closing balance was keyed on Tally's screen keeps
/// its opening as `TBALCLOSING` and shows the keyed change only in
/// `DEBITTOTALS` (two synthetic TallyPrime 7.2 books, one integrated with
/// inventory and one not). The row is reported as it was sent; this says only
/// that its own columns do not add up.
///
/// An empty amount is taken as zero here, and only here and in the statements:
/// the check can flag or refuse a read but never produces a figure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeTrialBalanceMovements {
    Tie,
    /// `closing - (opening + debit + credit)`, exactly, in Tally's sign, at the
    /// largest number of decimal places among the row's amounts.
    Differ(String),
}

impl NativeTrialBalanceRow {
    pub fn movements(&self) -> NativeTrialBalanceMovements {
        use bridge_tally_primitives::exact_arithmetic::ExactDecimalAccumulator;
        let amounts = [&self.opening, &self.debit, &self.credit, &self.closing];
        let mut gap = ExactDecimalAccumulator::default();
        let mut places = 0;
        for (index, amount) in amounts.into_iter().enumerate() {
            if let NativeTrialBalanceAmount::Present(value) = amount {
                places = places.max(value.as_str().split_once('.').map_or(0, |(_, f)| f.len()));
                // The closing is added; the opening and both movements are taken off it.
                if index == 3 {
                    gap.add(value.as_str());
                } else {
                    gap.subtract(value.as_str());
                }
            }
        }
        if gap.is_zero() {
            return NativeTrialBalanceMovements::Tie;
        }
        let mut text = gap.canonical_string();
        let have = text.split_once('.').map_or(0, |(_, f)| f.len());
        if have < places {
            if have == 0 {
                text.push('.');
            }
            text.push_str(&"0".repeat(places - have));
        }
        NativeTrialBalanceMovements::Differ(text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTrialBalance {
    pub rows: Vec<NativeTrialBalanceRow>,
}

/// A several-currency book's Trial Balance (bridge#551): the plain
/// base-currency rows, which alone are read, and the rows set aside by name.
/// Totals over `report` cover plain base-currency ledgers only and are not
/// expected to balance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrencyScopedTrialBalance {
    pub report: NativeTrialBalance,
    /// Ledgers kept in another currency, with their `CURRENCYNAME`.
    pub foreign_currency_ledgers: Vec<crate::native_outstandings::ForeignCurrencyLedger>,
    /// Base-currency ledgers with a value Tally wrote as a currency composite.
    pub mixed_currency_ledgers: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeTrialBalanceError {
    InvalidAmount,
    InvalidResponse(&'static str),
    TallyReportedFailure,
}

impl NativeTrialBalanceError {
    /// The reason a refusal names, as the other native reports' errors do.
    pub fn code(self) -> &'static str {
        match self {
            Self::InvalidAmount => "trial_balance_amount_invalid",
            Self::InvalidResponse(code) => code,
            Self::TallyReportedFailure => "trial_balance_tally_reported_failure",
        }
    }
}

impl fmt::Display for NativeTrialBalanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAmount => {
                formatter.write_str("Tally returned an invalid Trial Balance amount")
            }
            Self::InvalidResponse(code) => {
                write!(formatter, "native Trial Balance response invalid ({code})")
            }
            Self::TallyReportedFailure => {
                formatter.write_str("Tally reported failure for the native Trial Balance request")
            }
        }
    }
}

impl std::error::Error for NativeTrialBalanceError {}

/// Renders the only supported native Trial Balance read. Both dates come from
/// the already-admitted snapshot period: `TBALOPENING` and `TBALCLOSING` are
/// meaningful only for the same validated window.
pub fn render_native_trial_balance_request(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_trial_balance_collection(
        company,
        period,
        "NAME, GUID, PARENT, TBALOPENING, DEBITTOTALS, CREDITTOTALS, TBALCLOSING",
    )
}

/// [`render_native_trial_balance_request`] with `CURRENCYNAME` appended to
/// its `FETCH`, so each row names its ledger's currency and a foreign-currency
/// ledger can be set aside by name (bridge#551). Sent only when the company
/// defines several Currency masters, so a single-currency book's request is
/// byte-for-byte unchanged. On the captured several-currency book every row
/// carried it (`trial_balance_currency_forex_live`; one book, one run: PARTIAL).
pub fn render_native_trial_balance_request_with_currency(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_trial_balance_collection(
        company,
        period,
        "NAME, GUID, PARENT, TBALOPENING, DEBITTOTALS, CREDITTOTALS, TBALCLOSING, CURRENCYNAME",
    )
}

fn render_trial_balance_collection(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
    fetch: &str,
) -> String {
    format!(
        r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>List of Ledgers</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><SVFROMDATE TYPE="Date">{from}</SVFROMDATE><SVTODATE TYPE="Date">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME="List of Ledgers" ISMODIFY="Yes"><FETCH>{fetch}</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"#,
        company = xml_escape(company),
        from = period.from().as_str(),
        to = period.to().as_str(),
    )
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod unspaced_tests;
