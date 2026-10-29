//! Tally's own Stock Query report for one stock item (#2018): the item's closing quantity in each
//! godown and batch at a period end, read as `godown, batch, quantity` triples. Quantity only: the
//! report carries no value, rate, expiry or manufacturing date for a batch. What was measured, and
//! what was not, is in the protocol reference, measurements part, §12a.23.
//!
//! No read sends this request yet: the tool that does comes later (#2018). The parser is closed
//! on the elements captured and fails closed with a typed error (AGENTS.md P3); an item name
//! enters a request only as a [`MeasuredItemName`], whose alphabet is the one captured (P2, P7).
use crate::native_masters::{
    read_text, refuse_error_element, upper, within_name_bound, NativeMastersError,
};
use crate::native_outstandings::NativeLedgerSnapshotPeriod;
use crate::native_statement_reports::render_report_envelope;
use crate::native_stock_summary::{quantity, NativeStockQuantity};
use crate::tolerant_xml::sanitize_invalid_numeric_references;
use crate::xml_text::escape_text as xml_escape;
use quick_xml::{events::Event, Reader};
use serde::Serialize;

/// The longest item name admitted, in characters: a choice, not Tally's limit. The longest
/// captured is 22 (`Hydrochloric Acid 33pc`).
pub const STOCK_QUERY_ITEM_NAME_MAX_CHARS: usize = 60;

/// The batch Tally reports for stock not kept in a named batch.
const DEFAULT_BATCH: &str = "Primary Batch";

/// The character the sanitiser writes for a forbidden numeric reference, such as Tally's `&#4;`
/// prefix (crate docs): a value holding it is not text Tally meant as a name or quantity.
const TALLY_SANITIZED_MARKER: char = '\u{fffd}';

/// The characters besides ASCII letters, digits and the space that a capture showed Stock Query
/// selecting an item by, each escaped once as XML text (`lab/2018-stock-query-names-units-dates-answers`
/// at `86ea5211`, committed under `tests/fixtures/stock-query/`):
/// - `&`: `Kit & Co Item` (`w-02`);
/// - `(` and `)`: `Gauge (Test) 50L` (`w-03`);
/// - `'`: `Owner's Item` (`w-04` raw, `w-05` escaped as the builder sends it);
/// - `"`: `Quote "Q" Item` (`w-06` raw, `w-07` escaped as the builder sends it);
/// - `%`, `/` and `,`: `Mix 5% / Set, Large` (`w-09`);
/// - `-`: `Dash-Item 7` (`w-10`).
///
/// A character joins only with a capture of its own.
pub const STOCK_QUERY_MEASURED_PUNCTUATION: [char; 9] =
    ['&', '(', ')', '\'', '"', '%', '/', ',', '-'];

/// Why a Stock Query name could not be sent, or an answer could not be read. No variant
/// carries data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StockQueryError {
    /// The item name holds a character, or a spacing, that no capture showed Stock Query
    /// selecting the item by.
    ItemNameUnmeasured,
    /// The answer is not well-formed XML, not one `ENVELOPE`, or holds markup inside a value.
    Malformed,
    /// Tally's own failure element.
    TallyReportedFailure,
    /// An element outside the captured set.
    UnknownElement,
    /// A last-purchase or last-sale group without its six elements in their order, or a kind
    /// of group appearing again after the other kind.
    PartialGroup,
    /// A godown, batch and quantity triple cut short, out of order, or followed by a group.
    PartialLine,
    /// A godown or batch name that is empty, has space at either end, holds the sanitiser's
    /// marker, or is past the assumed name bound.
    NameInvalid,
    /// A quantity that is empty, holds the sanitiser's marker, or is not `<number> <unit>`:
    /// never read as zero.
    QuantityUnparseable,
    /// A second line for a godown and batch already read.
    LineRepeated,
}

impl StockQueryError {
    /// A stable name for a refusal's cause, carrying no data.
    pub const fn code(self) -> &'static str {
        match self {
            Self::ItemNameUnmeasured => "stock_query_item_name_unmeasured",
            Self::Malformed => "stock_query_malformed",
            Self::TallyReportedFailure => "stock_query_tally_reported_failure",
            Self::UnknownElement => "stock_query_unknown_element",
            Self::PartialGroup => "stock_query_partial_group",
            Self::PartialLine => "stock_query_partial_line",
            Self::NameInvalid => "stock_query_name_invalid",
            Self::QuantityUnparseable => "stock_query_quantity_unparseable",
            Self::LineRepeated => "stock_query_line_repeated",
        }
    }
}

impl std::fmt::Display for StockQueryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for StockQueryError {}

/// A stock item's name in the alphabet Stock Query was captured selecting by: ASCII letters,
/// digits, [`STOCK_QUERY_MEASURED_PUNCTUATION`] and single spaces, none at either end, at most
/// [`STOCK_QUERY_ITEM_NAME_MAX_CHARS`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasuredItemName(String);

impl MeasuredItemName {
    pub fn new(name: &str) -> Result<Self, StockQueryError> {
        let admitted = !name.is_empty()
            && name.chars().count() <= STOCK_QUERY_ITEM_NAME_MAX_CHARS
            && name.chars().all(|c| {
                c.is_ascii_alphanumeric()
                    || c == ' '
                    || STOCK_QUERY_MEASURED_PUNCTUATION.contains(&c)
            })
            && !name.starts_with(' ')
            && !name.ends_with(' ')
            && !name.contains("  ");
        if admitted {
            Ok(Self(name.to_string()))
        } else {
            Err(StockQueryError::ItemNameUnmeasured)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The Stock Query request for one item over `period`: the built-in report envelope with the
/// report `Stock Query` and one more static variable, `STOCKITEMNAME`, after the period's.
pub fn render_stock_query_request(
    company: &str,
    item: &MeasuredItemName,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_report_envelope(
        "Stock Query",
        company,
        period,
        &format!(
            "<STOCKITEMNAME>{}</STOCKITEMNAME>",
            xml_escape(item.as_str())
        ),
    )
}

/// The batch of one line: Tally's `Primary Batch`, which stands for stock not kept in a named
/// batch, or a batch's own name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "name", rename_all = "snake_case")]
pub enum StockQueryBatch {
    Default,
    Named(String),
}

/// One line of the answer: an item's quantity in one godown and batch, signed as Tally sent it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockQueryLine {
    pub godown: String,
    pub batch: StockQueryBatch,
    pub quantity: NativeStockQuantity,
}

/// What an answer held. `NoLines` is the empty answer: Tally sends it for an item with no stock
/// at the period end and for a name it does not know alike, so it is never read as zero on its
/// own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StockQueryAnswer {
    NoLines,
    Lines(Vec<StockQueryLine>),
}

const PURCHASE_GROUP: [&[u8]; 6] = [
    b"STQPURCDATE",
    b"STQPURCPARTY",
    b"STQPURCVCHQTY",
    b"STQPURCVCHRATE",
    b"STQPURCVCHDISC",
    b"STQPURCVCHAMOUNT",
];
const SALE_GROUP: [&[u8]; 6] = [
    b"STQSALESDATE",
    b"STQSALESVCHNO",
    b"STQSALESVCHQTY",
    b"STQSALESVCHRATE",
    b"STQSALESVCHDISC",
    b"STQSALESVCHAMOUNT",
];
const LINE: [&[u8]; 3] = [b"STQGODOWNNAME", b"STQBATCHNAME", b"STQGODOWNQTY"];

fn captured(name: &[u8]) -> bool {
    PURCHASE_GROUP
        .iter()
        .chain(SALE_GROUP.iter())
        .chain(LINE.iter())
        .any(|known| *known == name)
}

/// Parses a Stock Query answer. The captured order is: any last-purchase and last-sale groups,
/// six elements each, which are checked and skipped (they are not returned); then one triple per
/// godown and batch. No capture holds both kinds of group, so either kind may come first, but
/// the kinds do not interleave. Each of these refuses the whole answer: an element outside that
/// set, in another case or with an attribute; a comment or declaration; a group or triple cut
/// short; a group after a line; an empty or unreadable quantity; a name that is empty, padded,
/// marked by the sanitiser or over-long; and a godown and batch read twice.
pub fn parse_stock_query(xml: &str) -> Result<StockQueryAnswer, StockQueryError> {
    let sanitized = sanitize_invalid_numeric_references(xml);
    let mut reader = Reader::from_str(&sanitized);
    // Not trimmed: whitespace between elements is checked below, and each value is read whole.
    reader.config_mut().trim_text(false);
    let mut root_seen = false;
    let mut root_closed = false;
    let mut fields = Vec::<(Vec<u8>, String)>::new();
    loop {
        match reader
            .read_event()
            .map_err(|_| StockQueryError::Malformed)?
        {
            Event::Start(element) => {
                if element.attributes().next().is_some() {
                    return Err(StockQueryError::Malformed);
                }
                let name = element.name().as_ref().as_bytes().to_vec();
                if !root_seen {
                    if name.as_slice() != b"ENVELOPE" {
                        return Err(StockQueryError::Malformed);
                    }
                    root_seen = true;
                    continue;
                }
                if root_closed {
                    return Err(StockQueryError::Malformed);
                }
                refuse_error_element(&upper(element.name()))
                    .map_err(|_| StockQueryError::TallyReportedFailure)?;
                let text = read_text(&mut reader, element.name()).map_err(|error| match error {
                    NativeMastersError::TallyReportedFailure => {
                        StockQueryError::TallyReportedFailure
                    }
                    _ => StockQueryError::Malformed,
                })?;
                fields.push((name, text));
            }
            Event::Empty(element) if root_seen && !root_closed => {
                refuse_error_element(&upper(element.name()))
                    .map_err(|_| StockQueryError::TallyReportedFailure)?;
                if element.attributes().next().is_some() {
                    return Err(StockQueryError::Malformed);
                }
                fields.push((element.name().as_ref().as_bytes().to_vec(), String::new()));
            }
            Event::End(element)
                if root_seen && !root_closed && element.name().as_ref() == "ENVELOPE" =>
            {
                root_closed = true;
            }
            Event::Text(text) if text.as_ref().as_bytes().iter().all(u8::is_ascii_whitespace) => {}
            Event::Eof => break,
            _ => return Err(StockQueryError::Malformed),
        }
    }
    if !root_closed {
        return Err(StockQueryError::Malformed);
    }
    read_fields(&fields)
}

fn read_fields(fields: &[(Vec<u8>, String)]) -> Result<StockQueryAnswer, StockQueryError> {
    let mut at = 0;
    // The kinds of group read so far, in order: a kind may not come back after the other.
    let mut kinds = Vec::<&[&[u8]; 6]>::new();
    while let Some((name, _)) = fields.get(at) {
        let group = if name.as_slice() == PURCHASE_GROUP[0] {
            &PURCHASE_GROUP
        } else if name.as_slice() == SALE_GROUP[0] {
            &SALE_GROUP
        } else {
            break;
        };
        if kinds.last() != Some(&group) {
            if kinds.contains(&group) {
                return Err(StockQueryError::PartialGroup);
            }
            kinds.push(group);
        }
        expect(fields, at, group, StockQueryError::PartialGroup)?;
        at += group.len();
    }
    let mut lines = Vec::new();
    let mut seen = std::collections::HashSet::new();
    while at < fields.len() {
        expect(fields, at, &LINE, StockQueryError::PartialLine)?;
        let godown = line_name(&fields[at].1)?;
        let batch = line_name(&fields[at + 1].1)?;
        if !seen.insert((godown.clone(), batch.clone())) {
            return Err(StockQueryError::LineRepeated);
        }
        let amount = &fields[at + 2].1;
        if amount.contains(TALLY_SANITIZED_MARKER) {
            return Err(StockQueryError::QuantityUnparseable);
        }
        lines.push(StockQueryLine {
            godown,
            batch: if batch == DEFAULT_BATCH {
                StockQueryBatch::Default
            } else {
                StockQueryBatch::Named(batch)
            },
            quantity: quantity(Some(amount.clone()))
                .ok()
                .flatten()
                .ok_or(StockQueryError::QuantityUnparseable)?,
        });
        at += LINE.len();
    }
    Ok(if lines.is_empty() {
        StockQueryAnswer::NoLines
    } else {
        StockQueryAnswer::Lines(lines)
    })
}

/// `fields[at..]` start with `expected`, in order. A captured element out of place is `cut`; an
/// element never captured is [`StockQueryError::UnknownElement`].
fn expect(
    fields: &[(Vec<u8>, String)],
    at: usize,
    expected: &[&[u8]],
    cut: StockQueryError,
) -> Result<(), StockQueryError> {
    for (offset, want) in expected.iter().enumerate() {
        match fields.get(at + offset) {
            Some((name, _)) if name.as_slice() == *want => {}
            Some((name, _)) if !captured(name) => return Err(StockQueryError::UnknownElement),
            _ => return Err(cut),
        }
    }
    Ok(())
}

/// A godown or batch name as Tally sent it: never empty, with no space at either end and no
/// sanitiser's marker (a forbidden reference such as `&#4;` was in it), within the name bound.
fn line_name(text: &str) -> Result<String, StockQueryError> {
    if text.trim().is_empty() || text.trim() != text || text.contains(TALLY_SANITIZED_MARKER) {
        return Err(StockQueryError::NameInvalid);
    }
    within_name_bound(text).map_err(|_| StockQueryError::NameInvalid)?;
    Ok(text.to_string())
}

#[cfg(test)]
#[path = "native_stock_query_tests.rs"]
mod tests;
