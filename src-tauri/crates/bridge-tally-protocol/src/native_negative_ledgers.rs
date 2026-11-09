//! Tally's own Negative Ledgers list, requested by report name.
//!
//! A built-in report like the statements (protocol reference §12a.1): a success
//! carries no `HEADER` or `STATUS`, and nothing in the response identifies the
//! company, so a caller must bracket the read with identity and extent reads.
//! The observed shape on licensed TallyPrime 7.1 is in the provenance note of the
//! `builtin_negative_ledgers_*` fixtures: one `DSPACCNAME` (the ledger's display
//! name) followed by one `DSPACCINFO` holding a debit closing and a credit closing
//! column, per ledger, directly under the root.
//!
//! A debit closing is carried as a **negative** number in the debit column and a
//! credit closing as a **positive** one in the credit column, the other column
//! empty (the first book as reported against Tally's screen and trial balance, two
//! further books agreeing on the credit half; protocol reference §12a.16). The list
//! is Tally's own and is returned as it is: which ledgers Tally lists is not
//! derived here (a Suspense ledger with a credit balance was not listed on the
//! first book measured), and an **empty** answer cannot be told from a report that
//! was not rendered, so it is returned as empty, never as "no negative ledger".
//!
//! The parser is closed: anything other than well-formed entries of the expected
//! shape is refused, and a failure, an unknown report, a name repeated and an
//! amount that is not a plain decimal each have their own error; an entry with both
//! columns empty, with both columns filled, with a zero or with a sign that is not
//! the one measured is the one `BalanceUnexpected`.
use bridge_tally_primitives::ExactDecimal;
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use serde::Serialize;
use std::collections::BTreeSet;
use std::fmt;

use crate::{
    native_outstandings::NativeLedgerSnapshotPeriod,
    native_statement_reports::{
        amount, read_text, render_built_in_report_request, BuiltInReport, NativeStatementAmount,
    },
    tolerant_xml::sanitize_invalid_numeric_references,
};

/// Renders the only supported request: Tally's own Negative Ledgers by name over
/// the period, for the named company. No TDL.
pub fn render_native_negative_ledgers_request(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_built_in_report_request(BuiltInReport::NegativeLedgers, company, period)
}

/// A ledger's closing balance as Tally listed it: the side and the amount without
/// a sign.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "side", content = "amount", rename_all = "snake_case")]
pub enum NegativeLedgerBalance {
    Debit(ExactDecimal),
    Credit(ExactDecimal),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeNegativeLedger {
    /// The ledger's display name, as Tally shows it. Report entries carry no GUID.
    pub name: String,
    pub balance: NegativeLedgerBalance,
}

/// Tally's list. `entries` may be empty: see the module note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeNegativeLedgers {
    pub entries: Vec<NativeNegativeLedger>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeNegativeLedgersError {
    /// A `STATUS` or `LINEERROR` in the response: Tally reported failure.
    TallyReportedFailure,
    /// A bare `RESPONSE`: Tally did not recognise the report name (§12a.1).
    UnknownReport,
    /// An amount that is not a plain signed decimal (grouped, suffixed...).
    InvalidAmount,
    /// The same display name twice.
    LedgerRepeated,
    /// An entry that is not exactly one side: both columns, neither, a zero, or a
    /// sign other than the one measured (debit negative, credit positive).
    BalanceUnexpected,
    InvalidResponse(&'static str),
}

impl NativeNegativeLedgersError {
    pub fn code(self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "negative_ledgers_tally_reported_failure",
            Self::UnknownReport => "negative_ledgers_report_unknown",
            Self::InvalidAmount => "negative_ledgers_amount_invalid",
            Self::LedgerRepeated => "negative_ledgers_ledger_repeated",
            Self::BalanceUnexpected => "negative_ledgers_balance_unexpected",
            Self::InvalidResponse(code) => code,
        }
    }
}

impl fmt::Display for NativeNegativeLedgersError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "native negative ledgers response refused ({})",
            self.code()
        )
    }
}

impl std::error::Error for NativeNegativeLedgersError {}

impl From<crate::native_statement_reports::NativeStatementError> for NativeNegativeLedgersError {
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

fn invalid(code: &'static str) -> NativeNegativeLedgersError {
    NativeNegativeLedgersError::InvalidResponse(code)
}

fn malformed() -> NativeNegativeLedgersError {
    invalid("negative_ledgers_xml_malformed")
}

fn is_failure_signal(name: &[u8]) -> bool {
    matches!(name, b"STATUS" | b"LINEERROR" | b"ERROR")
}

/// The elements Tally wraps a failure in. They are admitted only so the failure
/// signal inside is reached; a wrapper with no signal is refused.
fn is_wrapper(name: &[u8]) -> bool {
    matches!(name, b"HEADER" | b"BODY" | b"DATA")
}

/// Parses a built-in Negative Ledgers response. Refuses a failure signal, an
/// unknown report, any element outside the expected entries, a name without its
/// entry (or an entry without its name), a name twice, an entry whose columns are
/// not exactly one signed amount of the measured sign, and an amount that is not a
/// plain signed decimal. A self-closed or empty root is an empty list.
pub fn parse_native_negative_ledgers(
    xml: &str,
) -> Result<NativeNegativeLedgers, NativeNegativeLedgersError> {
    let sanitized = sanitize_invalid_numeric_references(xml);
    let mut reader = Reader::from_str(&sanitized);
    reader.config_mut().trim_text(false);
    let mut root_seen = false;
    let mut envelope_closed = false;
    let mut wrapper_depth = 0_usize;
    let mut wrapper_seen = false;
    let mut pending_name: Option<String> = None;
    let mut entries: Vec<NativeNegativeLedger> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();

    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element) => {
                let name = element.name().as_ref().as_bytes().to_ascii_uppercase();
                if !root_seen {
                    match name.as_slice() {
                        b"ENVELOPE" => root_seen = true,
                        b"RESPONSE" => return Err(NativeNegativeLedgersError::UnknownReport),
                        _ => return Err(invalid("negative_ledgers_root_not_envelope")),
                    }
                    continue;
                }
                if envelope_closed {
                    return Err(invalid("negative_ledgers_trailing_content"));
                }
                if is_failure_signal(&name) {
                    return Err(NativeNegativeLedgersError::TallyReportedFailure);
                }
                if is_wrapper(&name) {
                    wrapper_depth += 1;
                    wrapper_seen = true;
                } else if wrapper_depth > 0 && name.as_slice() == b"VERSION" {
                    read_text(&mut reader, element.name())?;
                } else if wrapper_depth > 0 {
                    return Err(invalid("negative_ledgers_unexpected_element"));
                } else if name.as_slice() == b"DSPACCNAME" {
                    if pending_name.is_some() {
                        return Err(invalid("negative_ledgers_name_without_entry"));
                    }
                    pending_name = Some(read_name(&mut reader)?);
                } else if name.as_slice() == b"DSPACCINFO" {
                    let ledger = pending_name
                        .take()
                        .ok_or(invalid("negative_ledgers_entry_without_name"))?;
                    // Tally keeps ledger names unique regardless of case and padding.
                    if !seen.insert(ledger.trim().to_lowercase()) {
                        return Err(NativeNegativeLedgersError::LedgerRepeated);
                    }
                    let balance = read_entry(&mut reader, &element)?;
                    entries.push(NativeNegativeLedger {
                        name: ledger,
                        balance,
                    });
                } else {
                    return Err(invalid("negative_ledgers_unexpected_element"));
                }
            }
            Event::Empty(element) => {
                let name = element.name().as_ref().as_bytes().to_ascii_uppercase();
                if !root_seen {
                    // A self-closed root is the same empty list as an empty pair.
                    if name.as_slice() == b"ENVELOPE" {
                        root_seen = true;
                        envelope_closed = true;
                        continue;
                    }
                    return Err(invalid("negative_ledgers_root_not_envelope"));
                }
                if is_failure_signal(&name) {
                    return Err(NativeNegativeLedgersError::TallyReportedFailure);
                }
                return Err(invalid("negative_ledgers_unexpected_empty_element"));
            }
            Event::End(element) => {
                let name = element.name().as_ref().as_bytes().to_ascii_uppercase();
                if name.as_slice() == b"ENVELOPE" && wrapper_depth == 0 && !envelope_closed {
                    envelope_closed = true;
                } else if is_wrapper(&name) && wrapper_depth > 0 {
                    wrapper_depth -= 1;
                } else {
                    return Err(malformed());
                }
            }
            Event::Text(text) => {
                if !text.as_ref().as_bytes().iter().all(u8::is_ascii_whitespace) {
                    return Err(invalid("negative_ledgers_stray_text"));
                }
            }
            Event::Decl(_) | Event::Comment(_) => {}
            Event::Eof => break,
            _ => return Err(invalid("negative_ledgers_unexpected_content")),
        }
    }
    if !root_seen || !envelope_closed {
        return Err(invalid("negative_ledgers_envelope_unterminated"));
    }
    if wrapper_seen {
        return Err(invalid("negative_ledgers_wrapper_without_failure_signal"));
    }
    if pending_name.is_some() {
        return Err(invalid("negative_ledgers_name_without_entry"));
    }
    Ok(NativeNegativeLedgers { entries })
}

/// The `DSPACCNAME` block: its `DSPDISPNAME` once, text only, and nothing else.
fn read_name(reader: &mut Reader<&[u8]>) -> Result<String, NativeNegativeLedgersError> {
    let mut found: Option<String> = None;
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element)
                if element
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(b"DSPDISPNAME")
                    && found.is_none() =>
            {
                let text = read_text(reader, element.name())?;
                if text.trim().is_empty() {
                    return Err(invalid("negative_ledgers_name_empty"));
                }
                found = Some(text);
            }
            Event::End(element)
                if element
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(b"DSPACCNAME") =>
            {
                return found.ok_or(invalid("negative_ledgers_name_missing"));
            }
            Event::Text(text) if text.as_ref().as_bytes().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(invalid("negative_ledgers_name_shape")),
        }
    }
}

/// One `DSPACCINFO`: the debit and the credit closing column, each exactly once,
/// and nothing else; the two together must be exactly one amount of the measured
/// sign.
fn read_entry(
    reader: &mut Reader<&[u8]>,
    block: &BytesStart<'_>,
) -> Result<NegativeLedgerBalance, NativeNegativeLedgersError> {
    let block = block.name().as_ref().as_bytes().to_vec();
    let mut debit = None;
    let mut credit = None;
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element) => {
                let name = element.name().as_ref().as_bytes().to_ascii_uppercase();
                if is_failure_signal(&name) {
                    return Err(NativeNegativeLedgersError::TallyReportedFailure);
                }
                let (slot, inner): (&mut Option<NativeStatementAmount>, &[u8]) =
                    match name.as_slice() {
                        b"DSPCLDRAMT" => (&mut debit, b"DSPCLDRAMTA"),
                        b"DSPCLCRAMT" => (&mut credit, b"DSPCLCRAMTA"),
                        _ => return Err(invalid("negative_ledgers_unexpected_element")),
                    };
                if slot.is_some() {
                    return Err(invalid("negative_ledgers_duplicate_column"));
                }
                *slot = Some(read_column(
                    reader,
                    element.name().as_ref().as_bytes(),
                    inner,
                )?);
            }
            Event::End(element) if element.name().as_ref().as_bytes() == block.as_slice() => {
                return match (debit, credit) {
                    (Some(debit), Some(credit)) => balance(debit, credit),
                    _ => Err(invalid("negative_ledgers_column_missing")),
                };
            }
            Event::Text(text) if text.as_ref().as_bytes().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(invalid("negative_ledgers_entry_shape")),
        }
    }
}

/// One column: its outer element holding its amount element once, text or empty.
fn read_column(
    reader: &mut Reader<&[u8]>,
    outer: &[u8],
    inner: &[u8],
) -> Result<NativeStatementAmount, NativeNegativeLedgersError> {
    let outer = outer.to_vec();
    let mut found = None;
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element)
                if element
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(inner)
                    && found.is_none() =>
            {
                found = Some(amount(&read_text(reader, element.name())?)?);
            }
            Event::Empty(element)
                if element
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(inner)
                    && found.is_none() =>
            {
                found = Some(NativeStatementAmount::Empty);
            }
            Event::End(element) if element.name().as_ref().as_bytes() == outer.as_slice() => {
                return found.ok_or(invalid("negative_ledgers_amount_missing"));
            }
            Event::Text(text) if text.as_ref().as_bytes().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(invalid("negative_ledgers_column_shape")),
        }
    }
}

/// Exactly one side, of the sign measured: a debit negative, a credit positive,
/// and never zero.
fn balance(
    debit: NativeStatementAmount,
    credit: NativeStatementAmount,
) -> Result<NegativeLedgerBalance, NativeNegativeLedgersError> {
    match (debit, credit) {
        (NativeStatementAmount::Present(value), NativeStatementAmount::Empty)
            if value.is_negative() && !value.is_zero() =>
        {
            Ok(NegativeLedgerBalance::Debit(value.magnitude()))
        }
        (NativeStatementAmount::Empty, NativeStatementAmount::Present(value))
            if !value.is_negative() && !value.is_zero() =>
        {
            Ok(NegativeLedgerBalance::Credit(value))
        }
        _ => Err(NativeNegativeLedgersError::BalanceUnexpected),
    }
}

#[cfg(test)]
#[path = "native_negative_ledgers_tests.rs"]
mod tests;
