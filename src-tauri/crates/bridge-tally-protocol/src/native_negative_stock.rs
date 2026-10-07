//! Tally's own Negative Stock report, requested by report name.
//!
//! A built-in report like the statements (protocol reference §12a.1): a success
//! carries no `HEADER` or `STATUS`, and nothing in the response identifies the
//! company, so a caller must bracket the read with identity and extent reads.
//! The answer is the Stock Summary's item grammar: top-level children alternate
//! `DSPACCNAME` (one `DSPDISPNAME`) and `DSPSTKINFO` (one `DSPSTKCL` holding
//! `DSPCLQTY`, `DSPCLRATE` and `DSPCLAMTA`, all three present on the capture, so
//! all three are required here; a present element may be empty). On the one book measured it listed
//! exactly the items whose closing quantity is below zero or whose closing value
//! is a credit (§12a.16); that rule was observed on one book and is not applied
//! here: this module only reads the list, closed, and says what it could not.
//!
//! An empty answer cannot be told from a report that was not rendered, so it is
//! an error here, never "no negative stock".
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use serde::Serialize;
use std::{collections::HashSet, fmt};

use crate::{
    native_outstandings::NativeLedgerSnapshotPeriod,
    native_statement_reports::{
        amount, read_text, render_built_in_report_request, NativeStatementAmount,
        NativeStatementError,
    },
    native_stock_summary::{quantity, NativeQuantityRead},
    tolerant_xml::sanitize_invalid_numeric_references,
};

/// The report name Tally's gateway accepts (live capture, 6 Oct 2026).
const REPORT_ID: &str = "Negative Stock";

/// Renders the only supported request: Tally's own Negative Stock by name over
/// the period, for the named company. No TDL.
pub fn render_native_negative_stock_request(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_built_in_report_request(REPORT_ID, company, period)
}

/// One item Tally listed in this report, each rate and value exactly as Tally
/// printed it (an empty one is not zero). Being listed does not make it negative
/// by itself: four of the five captured items have a positive quantity (and a
/// credit value), so the quantity is kept for the caller's own check and is not
/// serialized, and no serialized form of an item says "negative".
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeListedStockItem {
    pub name: String,
    /// The closing quantity, signed as Tally sent it (`-50.000 Kgs` is a negative stock).
    #[serde(skip)]
    pub quantity: NativeQuantityRead,
    pub rate: NativeStatementAmount,
    pub value: NativeStatementAmount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeNegativeStockListing {
    pub items: Vec<NativeListedStockItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeNegativeStockError {
    /// A `STATUS` or `LINEERROR` in the response: Tally reported failure (the
    /// text differs by build, so the structure is what is classified).
    TallyReportedFailure,
    /// A bare `RESPONSE`: Tally did not recognise the report name (§12a.1).
    UnknownReport,
    /// An empty `ENVELOPE`: not told apart from a report that was not rendered
    /// (§12a.11), so never read as "no item is negative".
    EmptyEnvelope,
    /// A rate or value that is not a plain signed decimal.
    InvalidAmount,
    /// The same item name listed twice.
    DuplicateItem,
    InvalidResponse(&'static str),
}

impl NativeNegativeStockError {
    pub fn code(self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "negative_stock_tally_reported_failure",
            Self::UnknownReport => "negative_stock_report_unknown",
            Self::EmptyEnvelope => "negative_stock_empty_envelope",
            Self::InvalidAmount => "negative_stock_amount_invalid",
            Self::DuplicateItem => "negative_stock_item_duplicated",
            Self::InvalidResponse(code) => code,
        }
    }
}

impl fmt::Display for NativeNegativeStockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "native negative stock response refused ({})",
            self.code()
        )
    }
}

impl std::error::Error for NativeNegativeStockError {}

impl From<NativeStatementError> for NativeNegativeStockError {
    fn from(error: NativeStatementError) -> Self {
        match error {
            NativeStatementError::TallyReportedFailure => Self::TallyReportedFailure,
            NativeStatementError::UnknownReport => Self::UnknownReport,
            NativeStatementError::InvalidAmount => Self::InvalidAmount,
            NativeStatementError::InvalidResponse(code) => Self::InvalidResponse(code),
        }
    }
}

/// Element text, with a failure of the shared reader named in this module's codes.
fn text(
    reader: &mut Reader<&[u8]>,
    name: quick_xml::name::QName<'_>,
) -> Result<String, NativeNegativeStockError> {
    read_text(reader, name).map_err(|_| invalid("negative_stock_text_invalid"))
}

fn invalid(code: &'static str) -> NativeNegativeStockError {
    NativeNegativeStockError::InvalidResponse(code)
}

fn malformed() -> NativeNegativeStockError {
    invalid("negative_stock_xml_malformed")
}

fn is_failure_signal(name: &[u8]) -> bool {
    matches!(name, b"STATUS" | b"LINEERROR" | b"ERROR")
}

/// The elements Tally wraps a failure in; admitted only so the signal inside is
/// reached, and a wrapper with no signal is refused.
fn is_wrapper(name: &[u8]) -> bool {
    matches!(name, b"HEADER" | b"BODY" | b"DATA")
}

/// Parses a built-in Negative Stock response. Refuses a failure signal, an
/// unknown report, an empty answer, any element outside the item grammar, a
/// name without its row (or a row without its name), a row without exactly one
/// closing amount element, a rate or value that is not a plain signed decimal,
/// and an item named twice.
pub fn parse_native_negative_stock(
    xml: &str,
) -> Result<NativeNegativeStockListing, NativeNegativeStockError> {
    let sanitized = sanitize_invalid_numeric_references(xml);
    let mut reader = Reader::from_str(&sanitized);
    reader.config_mut().trim_text(false);
    let mut root_seen = false;
    let mut envelope_closed = false;
    let mut wrapper_depth = 0_usize;
    let mut wrapper_seen = false;
    let mut pending_name: Option<String> = None;
    let mut items: Vec<NativeListedStockItem> = Vec::new();
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element) => {
                let name = element.name().as_ref().to_ascii_uppercase();
                if !root_seen {
                    match name.as_slice() {
                        b"ENVELOPE" => root_seen = true,
                        b"RESPONSE" => return Err(NativeNegativeStockError::UnknownReport),
                        _ => return Err(invalid("negative_stock_root_not_envelope")),
                    }
                    continue;
                }
                if envelope_closed {
                    return Err(invalid("negative_stock_trailing_content"));
                }
                if is_failure_signal(&name) {
                    return Err(NativeNegativeStockError::TallyReportedFailure);
                }
                if is_wrapper(&name) {
                    wrapper_depth += 1;
                    wrapper_seen = true;
                } else if wrapper_depth > 0 && name.as_slice() == b"VERSION" {
                    text(&mut reader, element.name())?;
                } else if wrapper_depth > 0 {
                    return Err(invalid("negative_stock_unexpected_element"));
                } else if name.as_slice() == b"DSPACCNAME" {
                    if pending_name.is_some() {
                        return Err(invalid("negative_stock_name_without_row"));
                    }
                    pending_name = Some(read_item_name(&mut reader, &element)?);
                } else if name.as_slice() == b"DSPSTKINFO" {
                    let item_name = pending_name
                        .take()
                        .ok_or(invalid("negative_stock_row_without_name"))?;
                    items.push(read_item_row(&mut reader, &element, item_name)?);
                } else {
                    return Err(invalid("negative_stock_unexpected_element"));
                }
            }
            Event::Empty(element) => {
                let name = element.name().as_ref().to_ascii_uppercase();
                if !root_seen {
                    return Err(if name.as_slice() == b"ENVELOPE" {
                        NativeNegativeStockError::EmptyEnvelope
                    } else {
                        invalid("negative_stock_root_not_envelope")
                    });
                }
                if is_failure_signal(&name) {
                    return Err(NativeNegativeStockError::TallyReportedFailure);
                }
                return Err(invalid("negative_stock_unexpected_empty_element"));
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
                    return Err(invalid("negative_stock_stray_text"));
                }
            }
            Event::Decl(_) | Event::Comment(_) => {}
            Event::Eof => break,
            _ => return Err(invalid("negative_stock_unexpected_content")),
        }
    }
    if !root_seen || !envelope_closed {
        return Err(invalid("negative_stock_envelope_unterminated"));
    }
    if wrapper_seen {
        return Err(invalid("negative_stock_wrapper_without_failure_signal"));
    }
    if pending_name.is_some() {
        return Err(invalid("negative_stock_name_without_row"));
    }
    if items.is_empty() {
        return Err(NativeNegativeStockError::EmptyEnvelope);
    }
    let mut seen = HashSet::new();
    if !items.iter().all(|item| seen.insert(item.name.as_str())) {
        return Err(NativeNegativeStockError::DuplicateItem);
    }
    Ok(NativeNegativeStockListing { items })
}

/// `DSPACCNAME` holding exactly one non-blank `DSPDISPNAME` text element.
fn read_item_name(
    reader: &mut Reader<&[u8]>,
    block: &BytesStart<'_>,
) -> Result<String, NativeNegativeStockError> {
    let block = block.name().as_ref().to_vec();
    let mut name: Option<String> = None;
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element)
                if element.name().as_ref().eq_ignore_ascii_case(b"DSPDISPNAME")
                    && name.is_none() =>
            {
                name = Some(text(reader, element.name())?);
            }
            Event::End(element) if element.name().as_ref() == block.as_slice() => {
                return name
                    .filter(|text| !text.trim().is_empty())
                    .ok_or(invalid("negative_stock_item_name_missing"));
            }
            Event::Text(text) if text.as_ref().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(invalid("negative_stock_name_shape")),
        }
    }
}

/// `DSPSTKINFO` holding exactly one `DSPSTKCL`, and nothing else.
fn read_item_row(
    reader: &mut Reader<&[u8]>,
    block: &BytesStart<'_>,
    name: String,
) -> Result<NativeListedStockItem, NativeNegativeStockError> {
    let block = block.name().as_ref().to_vec();
    let mut item: Option<NativeListedStockItem> = None;
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element)
                if element.name().as_ref().eq_ignore_ascii_case(b"DSPSTKCL") && item.is_none() =>
            {
                item = Some(read_closing(reader, &element, name.clone())?);
            }
            Event::End(element) if element.name().as_ref() == block.as_slice() => {
                return item.ok_or(invalid("negative_stock_closing_missing"));
            }
            Event::Text(text) if text.as_ref().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(invalid("negative_stock_row_shape")),
        }
    }
}

/// `DSPSTKCL` holding `DSPCLQTY`, `DSPCLRATE` and `DSPCLAMTA`, each exactly once
/// and in any order; an element may be empty (an absent one is refused, not read
/// as empty).
fn read_closing(
    reader: &mut Reader<&[u8]>,
    block: &BytesStart<'_>,
    name: String,
) -> Result<NativeListedStockItem, NativeNegativeStockError> {
    let block = block.name().as_ref().to_vec();
    let (mut quantity_text, mut rate, mut value): (Option<String>, Option<String>, Option<String>) =
        (None, None, None);
    let mut quantity_seen = false;
    loop {
        match reader.read_event().map_err(|_| malformed())? {
            Event::Start(element) => {
                let key = element.name().as_ref().to_ascii_uppercase();
                let text = text(reader, element.name())?;
                match key.as_slice() {
                    b"DSPCLQTY" if !quantity_seen => {
                        quantity_seen = true;
                        quantity_text = Some(text);
                    }
                    b"DSPCLRATE" if rate.is_none() => rate = Some(text),
                    b"DSPCLAMTA" if value.is_none() => value = Some(text),
                    _ => return Err(invalid("negative_stock_closing_shape")),
                }
            }
            Event::Empty(element) => {
                let key = element.name().as_ref().to_ascii_uppercase();
                match key.as_slice() {
                    b"DSPCLQTY" if !quantity_seen => quantity_seen = true,
                    b"DSPCLRATE" if rate.is_none() => rate = Some(String::new()),
                    b"DSPCLAMTA" if value.is_none() => value = Some(String::new()),
                    _ => return Err(invalid("negative_stock_closing_shape")),
                }
            }
            Event::End(element) if element.name().as_ref() == block.as_slice() => {
                let (Some(rate), true) = (rate, quantity_seen) else {
                    return Err(invalid("negative_stock_column_missing"));
                };
                let value = value.ok_or(invalid("negative_stock_value_missing"))?;
                return Ok(NativeListedStockItem {
                    name,
                    quantity: match quantity(quantity_text) {
                        Ok(None) => NativeQuantityRead::Empty,
                        Ok(Some(read)) => NativeQuantityRead::Read(read),
                        // Text that is not `<number> <unit>` (an over-long unit included) is
                        // read as unread, never as a value.
                        Err(_) => NativeQuantityRead::Unread,
                    },
                    rate: amount(&rate)?,
                    value: amount(&value)?,
                });
            }
            Event::Text(text) if text.as_ref().iter().all(u8::is_ascii_whitespace) => {}
            _ => return Err(invalid("negative_stock_closing_shape")),
        }
    }
}

#[cfg(test)]
#[path = "native_negative_stock_tests.rs"]
mod tests;
