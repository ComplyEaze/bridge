//! Native `stock_summary` reads: the company's inventory flags and its own
//! stock item count, its stock items with their closing value, and Tally's own
//! Stock Summary report. Items are returned only when their closing values add
//! up to the report's top-level lines and are not fewer than the count
//! ([`gate_stock_summary`]). A quantity is read and never returned: nothing
//! checks it.
//!
//! Evidence: one synthetic book on one licensed `TallyPrime` 7.1
//! (`tests/fixtures/STOCK_CAPTURE_PROVENANCE.md`; PARTIAL), and, by role, one
//! client book on which the report total equalled the items' closing-value sum
//! (protocol reference §12a.13). How another release, another period end or a
//! book whose inventory is not integrated answers is unmeasured.
//!
//! The parsers are closed and fail closed with a typed error (AGENTS.md P3),
//! and read their collections with the masters parser's element helpers. The
//! stock-item row is bound to its company by the GUID's company prefix and a
//! valid suffix alone: this request has no computed `BRIDGECOMPANYGUID`, and
//! nothing in the response otherwise names the company. Codes are prefixed
//! `stock_`, except the company-flags refusals.
use crate::native_ledger_guid_has_company_prefix;
use crate::native_masters::{
    name_attribute, path_is, read_language_names, read_text, refuse_error_element,
    refuse_stray_text, skip_subtree, upper, within_name_bound, NativeMastersError,
    MASTERS_ASSUMED_ALIASES, MASTERS_ASSUMED_NAME_CHARS, MASTERS_RESPONSE_BUDGET_BYTES,
};
use crate::native_outstandings::NativeLedgerSnapshotPeriod;
use crate::native_statement_reports::render_built_in_report_request;
use crate::native_trial_balance::guid_suffix_is_valid;
use crate::outstandings_shared::COMPANY_EXTENT_V2_FETCH;
use crate::tolerant_xml::sanitize_invalid_numeric_references;
use crate::xml_text::escape_text as xml_escape;
use bridge_tally_primitives::{ExactDecimal, TallyDate};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Characters of one stock-item row that do not grow with its names. The
/// largest SHAPE LAB row, counted from its opening to its closing tag with the
/// capture's line ends and its names included, is 654 characters, so this
/// carries headroom over that one synthetic book (PARTIAL).
const STOCK_ITEM_FIXED_CHARS: usize = 700;

/// Name-bearing texts of a stock-item row before its aliases, each assumed at
/// most [`MASTERS_ASSUMED_NAME_CHARS`] and checked against that bound when
/// read: the `NAME` and `RESERVEDNAME` attributes, a `NAME` element, `PARENT`,
/// `BASEUNITS`, and the unit texts inside `OPENINGBALANCE` and
/// `CLOSINGBALANCE`. Aliases in the language lists are counted separately, per
/// row.
const STOCK_ITEM_NAME_SLOTS: usize = 7;

/// An assumed worst-case stock-item row, in UTF-16 bytes: the masters formula
/// (`2 * (fixed + 6 * names * (slots + aliases))`) with this row's fixed
/// characters and slots. The runtime multiplies it by the master mark to size a
/// read before it, and the per-row span check counts UTF-16 units against half
/// of it after.
pub const fn stock_item_worst_row_bytes() -> usize {
    2 * (STOCK_ITEM_FIXED_CHARS
        + 6 * MASTERS_ASSUMED_NAME_CHARS * (STOCK_ITEM_NAME_SLOTS + MASTERS_ASSUMED_ALIASES))
}

const _: () = assert!(stock_item_worst_row_bytes() < MASTERS_RESPONSE_BUDGET_BYTES);

/// The Company collection with its `FETCH` extended by the inventory flags and
/// the company's counts, and one single-term filter, `$GUID = "<GUID>"`: a
/// `Company` collection ignores `SVCURRENTCOMPANY` and returns every loaded
/// company (§12a.7), so the filter is what makes the response this company's
/// alone. Of the counts it fetches, only the stock item count is read
/// ([`NativeStockItemCount`]); the rest stay in the request so that it is
/// byte-equal to the committed capture. A GUID that could break the formula's
/// string is refused, not escaped.
pub fn render_company_inventory_flags_request(
    company: &str,
    company_guid: &str,
) -> Result<String, NativeStockError> {
    if company_guid.is_empty()
        || !company_guid
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    {
        return Err(NativeStockError::CompanyFlagsGuidUnsupported);
    }
    Ok(format!(
        r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>BridgeCompanyBookExtentV2</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE="Formulae" NAME="BridgeCompanyGuidFilter">$GUID = "{company_guid}"</SYSTEM><COLLECTION NAME="BridgeCompanyBookExtentV2" ISMODIFY="No"><TYPE>Company</TYPE><FETCH>{COMPANY_EXTENT_V2_FETCH}, ISINTEGRATED, ISINVENTORYON, ISBATCHWISEON, NUMLEDGERS, NUMGROUPS, NUMSTOCKITEMS, NUMSTOCKCATEGORIES, NUMGODOWNS, NUMUNITS, NUMVOUCHERTYPES</FETCH><FILTERS>BridgeCompanyGuidFilter</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"#,
        company = xml_escape(company),
    ))
}

/// The Stock Summary by report name, in the built-in statements' envelope
/// (§12a.1), over the already-admitted period.
pub fn render_native_stock_summary_request(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_built_in_report_request("Stock Summary", company, period)
}

/// The date a stock summary is read as of. Only a 31 March is constructible,
/// because only a 31 March period end (a financial-year end) has been measured
/// for stock, the report tie-out included. Any other date, one that the boundary
/// rule of Bridge's other reads would admit included, is refused as
/// [`NativeStockError::AsOfNotMeasured`]. Of the 31 Marches, only the period
/// ending 31 March 2026 has been measured: another year's 31 March is admitted
/// here, sharing the request shape but not the measurement. Other dates will be
/// admitted in a later change, once captures back them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockSummaryAsOf(TallyDate);

impl StockSummaryAsOf {
    pub fn new(date: TallyDate) -> Result<Self, NativeStockError> {
        if matches!(&date.as_str()[4..8], "0331") {
            Ok(Self(date))
        } else {
            Err(NativeStockError::AsOfNotMeasured)
        }
    }

    pub fn date(&self) -> &TallyDate {
        &self.0
    }
}

/// Tally's `Yes` or `No` for a company flag, or neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeFlag {
    Yes,
    No,
    /// Absent or empty: not read as `No`.
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct NativeInventoryFlags {
    pub integrated: NativeFlag,
    pub inventory_on: NativeFlag,
    pub batchwise: NativeFlag,
}

/// Tally's own count of the company's stock items (`NUMSTOCKITEMS` in the flags
/// answer). A count that is missing, empty or not a plain number is
/// `Unavailable`: it is never read as zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeStockItemCount {
    Reported(u64),
    Unavailable,
}

/// The company's inventory flags with its own stock item count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeCompanyInventory {
    pub flags: NativeInventoryFlags,
    pub item_count: NativeStockItemCount,
}

/// A quantity as Tally writes it, `<number> <unit>`. The amount is signed: it
/// keeps the sign Tally sent, so a negative stock is a negative amount.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeStockQuantity {
    pub amount: ExactDecimal,
    pub unit: String,
}

/// What a quantity element held. No quantity leaves Bridge while nothing checks
/// it, so one Bridge cannot read (a compound unit, or a unit with a space in
/// it) is counted and does not refuse the read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeQuantityRead {
    /// An empty or absent element: not zero.
    Empty,
    Read(NativeStockQuantity),
    /// Text that is not `<number> <unit>` with a unit free of spaces.
    Unread,
}

impl NativeQuantityRead {
    pub fn read(&self) -> Option<&NativeStockQuantity> {
        match self {
            Self::Read(quantity) => Some(quantity),
            Self::Empty | Self::Unread => None,
        }
    }
}

/// A quantity and a value. The value is `None` where Tally sent an empty or
/// absent element: not zero. The quantity is read but never serialized: nothing
/// checks it, so it is withheld.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeStockPosition {
    #[serde(skip)]
    pub quantity: NativeQuantityRead,
    /// A plain signed decimal exactly as Tally sends it: the sign is kept and
    /// never flipped. A negative value is a debit, which is stock held
    /// (§12a.13).
    pub value: Option<ExactDecimal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeStockItem {
    pub name: String,
    pub guid: String,
    /// The `PARENT` text as read, the reserved-root marker included exactly as
    /// the group snapshot keeps it; absent or blank is `None`.
    pub parent: Option<String>,
    pub base_unit: Option<String>,
    /// Read and validated (a malformed opening value still refuses), but
    /// never serialized: nothing checks it, and its as-at date is
    /// unmeasured (the only capture's books start where its period starts).
    #[serde(skip)]
    pub opening: NativeStockPosition,
    pub closing: NativeStockPosition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeStockItems {
    pub rows: Vec<NativeStockItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeStockError {
    /// A `STATUS` other than 1, or an `ERROR` or `LINEERROR` element.
    TallyReportedFailure,
    /// The envelope, a row or the report is not the closed shape; the code
    /// names where.
    Malformed(&'static str),
    /// No `COLLECTION` at all. An empty one is a valid zero-row answer.
    CollectionAbsent,
    ForeignChild,
    RowWithoutName,
    /// The GUID is absent, or is not `<company GUID>-<eight hex digits>`.
    RowGuidForeign,
    DuplicateGuid,
    DuplicateName,
    /// A name or alias count, or a row's length, over the assumed bounds.
    RowExceedsBound,
    /// The GUID-filtered Company collection did not return exactly one row, or
    /// its row is not this company's.
    CompanyFlagsNotOneRow,
    CompanyFlagsGuidUnsupported,
    FlagInvalid(&'static str),
    QuantityUnparseable,
    ValueUnparseable,
    ReportAmountInvalid,
    SumInvalid,
    /// A bare `RESPONSE`: Tally did not recognise the Stock Summary by name.
    ReportUnknown,
    /// The company's stock item count is missing, empty or not a number, so
    /// nothing says whether every item was read.
    ItemCountUnavailable,
    /// The `as_of` is not a 31 March, the only date measured for stock.
    AsOfNotMeasured,
}

impl NativeStockError {
    /// The refusal's stable, data-free code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "stock_tally_reported_failure",
            Self::Malformed(code) => code,
            Self::CollectionAbsent => "stock_collection_absent",
            Self::ForeignChild => "stock_foreign_child",
            Self::RowWithoutName => "stock_row_without_name",
            Self::RowGuidForeign => "stock_row_guid_foreign",
            Self::DuplicateGuid => "stock_row_duplicate_guid",
            Self::DuplicateName => "stock_row_duplicate_name",
            Self::RowExceedsBound => "stock_row_exceeds_bound",
            Self::CompanyFlagsNotOneRow => "company_flags_not_one_row",
            Self::CompanyFlagsGuidUnsupported => "company_flags_guid_unsupported",
            Self::FlagInvalid(field) => match *field {
                "is_integrated" => "stock_flag_invalid:is_integrated",
                "is_inventory_on" => "stock_flag_invalid:is_inventory_on",
                "is_batchwise_on" => "stock_flag_invalid:is_batchwise_on",
                _ => "stock_flag_invalid",
            },
            Self::QuantityUnparseable => "stock_quantity_unparseable",
            Self::ValueUnparseable => "stock_value_unparseable",
            Self::ReportAmountInvalid => "stock_report_amount_invalid",
            Self::SumInvalid => "stock_value_sum_invalid",
            Self::ReportUnknown => "stock_report_unknown",
            Self::ItemCountUnavailable => "stock_item_count_unavailable",
            Self::AsOfNotMeasured => "stock_summary_as_of_not_measured",
        }
    }
}

impl fmt::Display for NativeStockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "native stock response invalid ({})", self.code())
    }
}

impl std::error::Error for NativeStockError {}

/// The masters helpers' refusals, under this module's codes.
impl From<NativeMastersError> for NativeStockError {
    fn from(error: NativeMastersError) -> Self {
        match error {
            NativeMastersError::TallyReportedFailure => Self::TallyReportedFailure,
            NativeMastersError::RowWithoutName => Self::RowWithoutName,
            NativeMastersError::RowExceedsBound => Self::RowExceedsBound,
            NativeMastersError::Malformed(code) => Self::Malformed(stock_code(code)),
            // Raised by the masters parser's own row rules, never by the
            // helpers this module calls.
            NativeMastersError::CollectionAbsent
            | NativeMastersError::ForeignChild
            | NativeMastersError::RowGuidForeign
            | NativeMastersError::RowCompanyMismatch
            | NativeMastersError::RowFieldInvalid(_)
            | NativeMastersError::DuplicateGuid
            | NativeMastersError::DuplicateName
            | NativeMastersError::VoucherTypesEmpty
            | NativeMastersError::CostCategoriesEmpty => {
                Self::Malformed("stock_response_malformed")
            }
        }
    }
}

/// Every `Malformed` code the shared helpers can raise, under this module's
/// prefix.
fn stock_code(masters_code: &'static str) -> &'static str {
    match masters_code {
        "masters_xml_malformed" => "stock_xml_malformed",
        "masters_xml_invalid_encoding" => "stock_xml_invalid_encoding",
        "masters_xml_invalid_escape" => "stock_xml_invalid_escape",
        "masters_scalar_not_text_only" => "stock_scalar_not_text_only",
        "masters_unexpected_text" => "stock_unexpected_text",
        "masters_attribute_malformed" => "stock_attribute_malformed",
        "masters_row_unterminated" => "stock_row_unterminated",
        _ => "stock_response_malformed",
    }
}

fn malformed_xml() -> NativeStockError {
    NativeStockError::Malformed("stock_xml_malformed")
}

/// Reads one `COLLECTION` of `row_element` rows under a `STATUS` 1 envelope, as
/// [`crate::native_masters::parse_native_masters`] does: an absent `COLLECTION`
/// refuses and an empty one is zero rows; only `row_element` children; stray
/// text refuses; an `ERROR` or `LINEERROR` anywhere keeps its meaning. `read_row`
/// reads one row, up to and including its closing tag; when `max_row_units` is
/// set, a row longer than that many UTF-16 units (opening to closing tag)
/// refuses.
fn read_collection<T>(
    response: &str,
    row_element: &[u8],
    max_row_units: Option<usize>,
    mut read_row: impl FnMut(&mut Reader<&[u8]>, &BytesStart<'_>) -> Result<T, NativeStockError>,
) -> Result<Vec<T>, NativeStockError> {
    let sanitized = sanitize_invalid_numeric_references(response);
    let mut reader = Reader::from_str(&sanitized);
    reader.config_mut().trim_text(false);
    let mut path = Vec::<Vec<u8>>::new();
    let mut root_seen = false;
    let mut status: Option<String> = None;
    let mut collections = 0_usize;
    let mut rows = Vec::new();
    loop {
        let event_start = reader.buffer_position();
        match reader.read_event().map_err(|_| malformed_xml())? {
            Event::Start(element) => {
                let name = upper(element.name());
                if path.is_empty() && (root_seen || name != b"ENVELOPE") {
                    return Err(NativeStockError::Malformed("stock_root_not_envelope"));
                }
                refuse_error_element(&name)?;
                if path_is(&path, &[b"ENVELOPE", b"HEADER"]) && name == b"STATUS" {
                    let text = read_text(&mut reader, element.name())?;
                    let text = text.trim();
                    // An empty STATUS is no answer: the envelope's end refuses it.
                    if !text.is_empty() && text != "1" {
                        return Err(NativeStockError::TallyReportedFailure);
                    }
                    record_status(&mut status, text.to_string())?;
                    continue;
                }
                if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA"]) && name == b"COLLECTION" {
                    collections += 1;
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) {
                    if name != row_element {
                        return Err(NativeStockError::ForeignChild);
                    }
                    let row = read_row(&mut reader, &element)?;
                    if let Some(limit) = max_row_units {
                        if row_units(&sanitized, event_start, reader.buffer_position()) > limit {
                            return Err(NativeStockError::RowExceedsBound);
                        }
                    }
                    rows.push(row);
                    continue;
                }
                root_seen = true;
                path.push(name);
            }
            Event::Empty(element) => {
                let name = upper(element.name());
                refuse_error_element(&name)?;
                if path.is_empty() {
                    return Err(NativeStockError::Malformed("stock_root_not_envelope"));
                }
                if path_is(&path, &[b"ENVELOPE", b"HEADER"]) && name == b"STATUS" {
                    record_status(&mut status, String::new())?;
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA"]) && name == b"COLLECTION"
                {
                    collections += 1;
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) {
                    return Err(if name == row_element {
                        NativeStockError::Malformed("stock_row_empty")
                    } else {
                        NativeStockError::ForeignChild
                    });
                }
            }
            Event::End(_) => {
                if path.pop().is_none() {
                    return Err(NativeStockError::Malformed("stock_unexpected_close"));
                }
            }
            Event::Text(text)
                if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) =>
            {
                refuse_stray_text(&text)?;
            }
            Event::CData(_) | Event::GeneralRef(_)
                if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) =>
            {
                return Err(NativeStockError::Malformed("stock_unexpected_text"));
            }
            Event::DocType(_) => {
                return Err(NativeStockError::Malformed("stock_doctype_forbidden"))
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if !path.is_empty() {
        return Err(NativeStockError::Malformed("stock_envelope_unterminated"));
    }
    if !root_seen {
        return Err(NativeStockError::Malformed("stock_envelope_missing"));
    }
    if status.as_deref() != Some("1") {
        return Err(NativeStockError::Malformed("stock_status_absent"));
    }
    match collections {
        0 => Err(NativeStockError::CollectionAbsent),
        1 => Ok(rows),
        _ => Err(NativeStockError::Malformed("stock_collection_repeated")),
    }
}

fn record_status(slot: &mut Option<String>, value: String) -> Result<(), NativeStockError> {
    if slot.replace(value).is_some() {
        return Err(NativeStockError::Malformed("stock_status_repeated"));
    }
    Ok(())
}

/// The row from its opening tag to its closing tag, in UTF-16 units (the unit
/// the response is measured in); `usize::MAX` if the positions are not a span
/// of `text`.
fn row_units(text: &str, start: u64, end: u64) -> usize {
    usize::try_from(start)
        .ok()
        .zip(usize::try_from(end).ok())
        .and_then(|(start, end)| text.get(start..end))
        .map_or(usize::MAX, |row| row.encode_utf16().count())
}

/// The scalar children a stock-item row's fields are read from, by upper-case
/// name. One occurrence each; a repeat refuses. `NAME` is read only to check it
/// against the name bound.
const ITEM_FIELDS: [&str; 8] = [
    "GUID",
    "PARENT",
    "BASEUNITS",
    "OPENINGBALANCE",
    "OPENINGVALUE",
    "CLOSINGBALANCE",
    "CLOSINGVALUE",
    "NAME",
];

const FLAG_FIELDS: [&str; 5] = [
    "GUID",
    "ISINTEGRATED",
    "ISINVENTORYON",
    "ISBATCHWISEON",
    "NUMSTOCKITEMS",
];

/// The `wanted` children of the row now open, by upper-case name, up to the
/// row's closing tag. Aliases under `LANGUAGENAME.LIST` are counted per row and
/// refused past the assumed bound; any other child is skipped.
fn read_row_fields(
    reader: &mut Reader<&[u8]>,
    row_element: &[u8],
    wanted: &[&'static str],
) -> Result<HashMap<&'static str, String>, NativeStockError> {
    let field_of = |name: &[u8]| {
        wanted
            .iter()
            .copied()
            .find(|field| field.as_bytes() == name)
    };
    let mut fields = HashMap::<&'static str, String>::new();
    let mut language_names = 0_usize;
    loop {
        match reader.read_event().map_err(|_| malformed_xml())? {
            Event::Start(child) => {
                let child_name = upper(child.name());
                refuse_error_element(&child_name)?;
                if child_name == b"LANGUAGENAME.LIST" {
                    read_language_names(reader, &mut language_names)?;
                } else if let Some(field) = field_of(child_name.as_slice()) {
                    let text = read_text(reader, child.name())?;
                    if fields.insert(field, text).is_some() {
                        return Err(NativeStockError::Malformed("stock_row_field_repeated"));
                    }
                } else {
                    skip_subtree(reader)?;
                }
            }
            Event::Empty(child) => {
                let child_name = upper(child.name());
                refuse_error_element(&child_name)?;
                if let Some(field) = field_of(child_name.as_slice()) {
                    if fields.insert(field, String::new()).is_some() {
                        return Err(NativeStockError::Malformed("stock_row_field_repeated"));
                    }
                }
            }
            Event::End(end)
                if end
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(row_element) =>
            {
                break
            }
            Event::Text(text) => refuse_stray_text(&text)?,
            Event::CData(_) | Event::GeneralRef(_) => {
                return Err(NativeStockError::Malformed("stock_unexpected_text"))
            }
            Event::Eof => return Err(NativeStockError::Malformed("stock_row_unterminated")),
            _ => {}
        }
    }
    Ok(fields)
}

/// Parses the GUID-filtered Company collection of
/// [`render_company_inventory_flags_request`]. Exactly one `COMPANY` row whose
/// `GUID` equals `company_guid` is admitted: none, several (a year-split
/// sibling shares the GUID, §9.11b) or another company's refuse
/// `company_flags_not_one_row`. A flag is `Yes`, `No`, or `Unknown` when absent
/// or empty; any other text refuses. The stock item count is read as
/// [`item_count`] reads it; the other `NUM*` counts are not read.
pub fn parse_company_inventory_flags(
    response: &str,
    company_guid: &str,
) -> Result<NativeCompanyInventory, NativeStockError> {
    let mut rows = read_collection(response, b"COMPANY", None, |reader, _element| {
        read_row_fields(reader, b"COMPANY", &FLAG_FIELDS)
    })?;
    if rows.len() != 1 {
        return Err(NativeStockError::CompanyFlagsNotOneRow);
    }
    let mut fields = rows.remove(0);
    let bound = fields
        .remove("GUID")
        .is_some_and(|guid| guid.trim().eq_ignore_ascii_case(company_guid));
    if !bound {
        return Err(NativeStockError::CompanyFlagsNotOneRow);
    }
    Ok(NativeCompanyInventory {
        flags: NativeInventoryFlags {
            integrated: flag(&mut fields, "ISINTEGRATED", "is_integrated")?,
            inventory_on: flag(&mut fields, "ISINVENTORYON", "is_inventory_on")?,
            batchwise: flag(&mut fields, "ISBATCHWISEON", "is_batchwise_on")?,
        },
        item_count: item_count(fields.remove("NUMSTOCKITEMS")),
    })
}

/// Tally's own stock item count (§12a.13): plain digits after trimming (the
/// committed captures write a non-zero count with a leading space, ` 11`, and
/// zero as `0`). Missing,
/// empty, signed, grouped, fractional or too large for a count: unavailable,
/// never zero. It does not refuse here: the gate decides what a read without a
/// count is.
fn item_count(text: Option<String>) -> NativeStockItemCount {
    text.as_deref()
        .map(str::trim)
        .filter(|text| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|text| text.parse::<u64>().ok())
        .map_or(
            NativeStockItemCount::Unavailable,
            NativeStockItemCount::Reported,
        )
}

fn flag(
    fields: &mut HashMap<&'static str, String>,
    key: &'static str,
    label: &'static str,
) -> Result<NativeFlag, NativeStockError> {
    match fields.remove(key).as_deref().map(str::trim) {
        None | Some("") => Ok(NativeFlag::Unknown),
        Some("Yes") => Ok(NativeFlag::Yes),
        Some("No") => Ok(NativeFlag::No),
        Some(_) => Err(NativeStockError::FlagInvalid(label)),
    }
}

/// Parses the stock-items collection of `AuditStockItemsV1`. Refuses a row with
/// no name, a GUID that is not `company_guid`'s (the row's only binding to its
/// company), a repeated GUID or a name repeated ignoring ASCII case, a name,
/// alias count or row length over the assumed bounds, and a quantity or value
/// that does not read exactly (the whole read refuses, not the row).
pub fn parse_native_stock_items(
    response: &str,
    company_guid: &str,
) -> Result<NativeStockItems, NativeStockError> {
    let mut guids = HashSet::new();
    let mut names = HashSet::new();
    let rows = read_collection(
        response,
        b"STOCKITEM",
        Some(stock_item_worst_row_bytes() / 2),
        |reader, element| {
            let name = name_attribute(element)?;
            let mut fields = read_row_fields(reader, b"STOCKITEM", &ITEM_FIELDS)?;
            for field in ["NAME", "PARENT", "BASEUNITS"] {
                if let Some(text) = fields.get(field) {
                    within_name_bound(text)?;
                }
            }
            let guid = fields
                .remove("GUID")
                .filter(|guid| {
                    native_ledger_guid_has_company_prefix(guid, company_guid)
                        && guid_suffix_is_valid(guid, company_guid)
                })
                .ok_or(NativeStockError::RowGuidForeign)?;
            if !guids.insert(guid.to_ascii_lowercase()) {
                return Err(NativeStockError::DuplicateGuid);
            }
            if !names.insert(name.to_ascii_lowercase()) {
                return Err(NativeStockError::DuplicateName);
            }
            let non_blank = |text: String| Some(text).filter(|text| !text.trim().is_empty());
            Ok(NativeStockItem {
                name,
                guid,
                parent: fields.remove("PARENT").and_then(non_blank),
                base_unit: fields.remove("BASEUNITS").and_then(non_blank),
                opening: position(&mut fields, "OPENINGBALANCE", "OPENINGVALUE")?,
                closing: position(&mut fields, "CLOSINGBALANCE", "CLOSINGVALUE")?,
            })
        },
    )?;
    Ok(NativeStockItems { rows })
}

fn position(
    fields: &mut HashMap<&'static str, String>,
    quantity_key: &'static str,
    value_key: &'static str,
) -> Result<NativeStockPosition, NativeStockError> {
    Ok(NativeStockPosition {
        quantity: match quantity(fields.remove(quantity_key)) {
            Ok(Some(quantity)) => NativeQuantityRead::Read(quantity),
            Ok(None) => NativeQuantityRead::Empty,
            // Outside the grammar (a compound unit, a unit with a space in
            // it): unread, and the row is still read. No quantity is returned.
            Err(NativeStockError::QuantityUnparseable) => NativeQuantityRead::Unread,
            // A unit over the name bound breaks the row-size premise, which
            // is not about the grammar: it still refuses.
            Err(error) => return Err(error),
        },
        value: value(fields.remove(value_key))?,
    })
}

/// The strict quantity grammar: `^-?[0-9]+(\.[0-9]+)? <unit>$` after trimming,
/// where the unit is one or more non-space characters (letters and dots: `U.`
/// was seen live). An empty or absent element is `None`; anything else that
/// does not match, a double space or a unit with a space in it included,
/// refuses. The row reader turns that refusal into an unread quantity
/// ([`position`]); a quantity must pass this grammar before one is ever
/// returned.
fn quantity(text: Option<String>) -> Result<Option<NativeStockQuantity>, NativeStockError> {
    let Some(text) = text else {
        return Ok(None);
    };
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    let (amount, unit) = text
        .split_once(' ')
        .ok_or(NativeStockError::QuantityUnparseable)?;
    if unit.is_empty() || unit.chars().any(char::is_whitespace) {
        return Err(NativeStockError::QuantityUnparseable);
    }
    within_name_bound(unit)?;
    let amount = ExactDecimal::parse(amount).map_err(|_| NativeStockError::QuantityUnparseable)?;
    Ok(Some(NativeStockQuantity {
        amount,
        unit: unit.to_string(),
    }))
}

/// A plain signed decimal after trimming; an empty or absent element is `None`.
fn value(text: Option<String>) -> Result<Option<ExactDecimal>, NativeStockError> {
    match text.as_deref().map(str::trim) {
        None | Some("") => Ok(None),
        Some(text) => ExactDecimal::parse(text)
            .map(Some)
            .map_err(|_| NativeStockError::ValueUnparseable),
    }
}

/// Tally's own Stock Summary, as far as the gate needs it: the sum of its
/// top-level closing amounts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeStockReport {
    Lines {
        /// The sum of the amounts present; `None` when every amount was empty.
        total: Option<ExactDecimal>,
        present: usize,
        /// Amount elements that were empty: counted, never read as zero.
        empty: usize,
    },
    /// An empty `ENVELOPE`: not told apart from a report Tally did not render
    /// (§12a.11), so never read as zero.
    Empty,
}

/// Parses the Stock Summary response (§12a.1, §12a.13): no `HEADER` or `STATUS`;
/// top-level children alternate `DSPACCNAME` then `DSPSTKINFO` and nothing else;
/// each `DSPSTKINFO` holds one `DSPSTKCL` whose `DSPCLAMTA` is a plain signed
/// decimal or empty. A `LINEERROR` or `ERROR` refuses, and so does a bare
/// `RESPONSE` (Tally did not recognise the report name, §12a.1), as the
/// statements parser refuses it; an empty envelope is an answer
/// ([`NativeStockReport::Empty`]). The total is written at the scale of the
/// amounts it adds.
pub fn parse_native_stock_summary_report(xml: &str) -> Result<NativeStockReport, NativeStockError> {
    let sanitized = sanitize_invalid_numeric_references(xml);
    let mut reader = Reader::from_str(&sanitized);
    // Not trimmed: an entity splits a name into several text events.
    reader.config_mut().trim_text(false);
    let mut root_seen = false;
    let mut envelope_closed = false;
    let mut expect_info = false;
    let mut pairs = 0_usize;
    let mut amounts = Vec::<ExactDecimal>::new();
    let mut empty = 0_usize;
    loop {
        match reader.read_event().map_err(|_| malformed_xml())? {
            Event::Start(element) => {
                let name = upper(element.name());
                if !root_seen {
                    match name.as_slice() {
                        b"ENVELOPE" => root_seen = true,
                        b"RESPONSE" => return Err(NativeStockError::ReportUnknown),
                        _ => return Err(report_shape("stock_report_root_not_envelope")),
                    }
                    continue;
                }
                if envelope_closed {
                    return Err(report_shape("stock_report_trailing_content"));
                }
                refuse_error_element(&name)?;
                match name.as_slice() {
                    b"DSPACCNAME" if !expect_info => {
                        read_report_name(&mut reader)?;
                        expect_info = true;
                    }
                    b"DSPSTKINFO" if expect_info => {
                        match read_report_info(&mut reader)? {
                            Some(amount) => amounts.push(amount),
                            None => empty += 1,
                        }
                        pairs += 1;
                        expect_info = false;
                    }
                    _ => return Err(report_shape("stock_report_unexpected_element")),
                }
            }
            Event::Empty(element) => {
                let name = upper(element.name());
                if !root_seen {
                    match name.as_slice() {
                        b"ENVELOPE" => {
                            root_seen = true;
                            envelope_closed = true;
                        }
                        b"RESPONSE" => return Err(NativeStockError::ReportUnknown),
                        _ => return Err(report_shape("stock_report_root_not_envelope")),
                    }
                    continue;
                }
                refuse_error_element(&name)?;
                return Err(report_shape("stock_report_unexpected_empty_element"));
            }
            Event::End(element) => {
                if upper(element.name()) == b"ENVELOPE" && root_seen && !envelope_closed {
                    envelope_closed = true;
                } else {
                    return Err(malformed_xml());
                }
            }
            Event::Text(text) => refuse_stray_text(&text)?,
            Event::Decl(_) | Event::Comment(_) => {}
            Event::Eof => break,
            _ => return Err(report_shape("stock_report_unexpected_content")),
        }
    }
    if !root_seen || !envelope_closed {
        return Err(report_shape("stock_report_envelope_unterminated"));
    }
    if expect_info {
        return Err(report_shape("stock_report_name_without_info"));
    }
    Ok(if pairs == 0 {
        NativeStockReport::Empty
    } else {
        NativeStockReport::Lines {
            total: if amounts.is_empty() {
                None
            } else {
                Some(sum_at_scale(amounts.iter())?)
            },
            present: amounts.len(),
            empty,
        }
    })
}

fn report_shape(code: &'static str) -> NativeStockError {
    NativeStockError::Malformed(code)
}

/// `DSPACCNAME` holding exactly one non-blank `DSPDISPNAME` text element.
fn read_report_name(reader: &mut Reader<&[u8]>) -> Result<(), NativeStockError> {
    let mut named = false;
    loop {
        match reader.read_event().map_err(|_| malformed_xml())? {
            Event::Start(element) => {
                let name = upper(element.name());
                refuse_error_element(&name)?;
                if name != b"DSPDISPNAME" || named {
                    return Err(report_shape("stock_report_name_shape"));
                }
                if read_text(reader, element.name())?.trim().is_empty() {
                    return Err(report_shape("stock_report_name_empty"));
                }
                named = true;
            }
            Event::Empty(element) => {
                refuse_error_element(&upper(element.name()))?;
                return Err(report_shape("stock_report_name_shape"));
            }
            Event::End(element) if upper(element.name()) == b"DSPACCNAME" => {
                return if named {
                    Ok(())
                } else {
                    Err(report_shape("stock_report_name_missing"))
                };
            }
            Event::Text(text) => refuse_stray_text(&text)?,
            _ => return Err(report_shape("stock_report_name_shape")),
        }
    }
}

/// `DSPSTKINFO` holding exactly one `DSPSTKCL`; returns its closing amount, or
/// `None` where that amount was empty.
fn read_report_info(reader: &mut Reader<&[u8]>) -> Result<Option<ExactDecimal>, NativeStockError> {
    let mut amount = None;
    let mut closings = 0_usize;
    loop {
        match reader.read_event().map_err(|_| malformed_xml())? {
            Event::Start(element) => {
                let name = upper(element.name());
                refuse_error_element(&name)?;
                closings += 1;
                if name != b"DSPSTKCL" || closings > 1 {
                    return Err(report_shape("stock_report_info_shape"));
                }
                amount = read_report_closing(reader)?;
            }
            Event::Empty(element) => {
                refuse_error_element(&upper(element.name()))?;
                return Err(report_shape("stock_report_info_shape"));
            }
            Event::End(element) if upper(element.name()) == b"DSPSTKINFO" => {
                return if closings == 1 {
                    Ok(amount)
                } else {
                    Err(report_shape("stock_report_info_shape"))
                };
            }
            Event::Text(text) => refuse_stray_text(&text)?,
            _ => return Err(report_shape("stock_report_info_shape")),
        }
    }
}

/// `DSPSTKCL` holding `DSPCLQTY`, `DSPCLRATE` and `DSPCLAMTA`, each at most
/// once and `DSPCLAMTA` exactly once. Only the amount is read.
fn read_report_closing(
    reader: &mut Reader<&[u8]>,
) -> Result<Option<ExactDecimal>, NativeStockError> {
    let mut amount = None;
    let (mut amount_seen, mut quantity_seen, mut rate_seen) = (false, false, false);
    loop {
        let (name, text) = match reader.read_event().map_err(|_| malformed_xml())? {
            Event::Start(element) => {
                let name = upper(element.name());
                refuse_error_element(&name)?;
                (name, read_text(reader, element.name())?)
            }
            Event::Empty(element) => {
                let name = upper(element.name());
                refuse_error_element(&name)?;
                (name, String::new())
            }
            Event::End(element) if upper(element.name()) == b"DSPSTKCL" => {
                return if amount_seen {
                    Ok(amount)
                } else {
                    Err(report_shape("stock_report_amount_missing"))
                };
            }
            Event::Text(text) => {
                refuse_stray_text(&text)?;
                continue;
            }
            _ => return Err(report_shape("stock_report_closing_shape")),
        };
        match name.as_slice() {
            b"DSPCLAMTA" if !amount_seen => {
                amount_seen = true;
                amount = match text.trim() {
                    "" => None,
                    text => Some(
                        ExactDecimal::parse(text)
                            .map_err(|_| NativeStockError::ReportAmountInvalid)?,
                    ),
                };
            }
            b"DSPCLQTY" if !quantity_seen => quantity_seen = true,
            b"DSPCLRATE" if !rate_seen => rate_seen = true,
            _ => return Err(report_shape("stock_report_closing_shape")),
        }
    }
}

/// What the items add up to, as a caller reports it beside them. Quantities
/// are withheld; the one count that concerns them says how many Bridge could
/// not read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeStockTotals {
    pub item_count: usize,
    pub empty_closing_value_count: usize,
    /// Closing quantities Bridge could not read (a compound unit, or a unit
    /// with a space in it). They are withheld like every quantity.
    pub closing_quantity_unread_count: usize,
    /// The sum of the closing values, at the scale of the values it adds, or
    /// `None` (with `partial`) whenever any item's closing value is empty: an
    /// empty value is not zero. A present `0.00` is a value.
    pub value_sum: Option<ExactDecimal>,
    pub partial: bool,
}

impl NativeStockTotals {
    pub fn of(items: &[NativeStockItem]) -> Result<Self, NativeStockError> {
        let empty_closing_value_count = items
            .iter()
            .filter(|item| item.closing.value.is_none())
            .count();
        let partial = empty_closing_value_count > 0;
        Ok(Self {
            item_count: items.len(),
            empty_closing_value_count,
            closing_quantity_unread_count: items
                .iter()
                .filter(|item| item.closing.quantity == NativeQuantityRead::Unread)
                .count(),
            value_sum: if partial {
                None
            } else {
                Some(present_closing_value_sum(items)?)
            },
            partial,
        })
    }
}

/// The sum of the closing values that are present (an empty one is left out,
/// not counted as zero), at the scale of the values it adds.
fn present_closing_value_sum(items: &[NativeStockItem]) -> Result<ExactDecimal, NativeStockError> {
    sum_at_scale(items.iter().filter_map(|item| item.closing.value.as_ref()))
}

/// The signed sum of `values`, written with as many decimal places as the
/// widest of them: `2500.00` and `500.00` are `3000.00`, not `3000`. No value
/// at all sums to `0`.
fn sum_at_scale<'a>(
    values: impl Iterator<Item = &'a ExactDecimal>,
) -> Result<ExactDecimal, NativeStockError> {
    let decimals = |text: &str| {
        text.split_once('.')
            .map_or(0, |(_, fraction)| fraction.len())
    };
    let mut scale = 0_usize;
    let mut sum = ExactDecimal::zero();
    for value in values {
        scale = scale.max(decimals(value.as_str()));
        sum = sum
            .checked_add(value)
            .map_err(|_| NativeStockError::SumInvalid)?;
    }
    let text = sum.as_str();
    let missing = scale - decimals(text).min(scale);
    if missing == 0 {
        return Ok(sum);
    }
    let point = if text.contains('.') { "" } else { "." };
    ExactDecimal::parse(format!("{text}{point}{}", "0".repeat(missing)))
        .map_err(|_| NativeStockError::SumInvalid)
}

/// How the rows read compare with Tally's own stock item count. Items leave
/// only when the two are equal: a count that differs in either direction is not
/// counting the list that was read ([`NativeStockGate::ItemCountDiffers`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeItemCountStatus {
    Matched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct NativeItemCountCrossCheck {
    pub status: NativeItemCountStatus,
    pub rows: usize,
    pub tally_count: u64,
}

/// What the read established. Items are reachable only from `ValueTotalMatched`:
/// every other outcome carries none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeStockGate {
    /// Tally's count is 0, the item collection is empty and the report is empty.
    NoStockItems,
    /// At least one item carries a value, the report has a total, and the two
    /// sums are numerically equal.
    ValueTotalMatched {
        items: Vec<NativeStockItem>,
        totals: NativeStockTotals,
        /// The report's total, equal to the items' closing-value sum.
        total: ExactDecimal,
        /// The report's amount elements that were empty and left out of it.
        report_empty_amounts: usize,
        item_count: NativeItemCountCrossCheck,
    },
    /// The rows read and Tally's own stock item count differ, either way.
    ItemCountDiffers { rows: usize, tally_count: u64 },
    /// The report has a total that the items do not add up to. `items_total` is
    /// `None` when no item carries a value at all.
    Differs {
        items_total: Option<ExactDecimal>,
        report_total: ExactDecimal,
    },
    /// The items carry a non-zero value and the report shows no amount: an
    /// empty report is not told apart from one Tally did not render.
    ReportShowsNoValue { items_total: ExactDecimal },
    /// Nothing could be compared.
    NotComparable,
}

/// Decides what a read established from the rows, Tally's own item count and
/// the report. In order:
///
/// 1. no count: nothing says whether every item was read, so the read refuses;
/// 2. the rows and the count differ, either way: its own outcome, before any
///    comparison. Fewer rows may be an incomplete list; more rows means the
///    count is not counting the list that was read;
/// 3. no rows, a count of zero and an empty report: no stock items;
/// 4. the report has a total: the items' closing values must add up to it, with
///    at least one value on their side (two empty sides never match);
/// 5. the report shows no amount: a non-zero sum on the items' side is not
///    confirmed, and anything else was not comparable. Whether Tally's Stock
///    Summary shows a line for a group worth zero is unmeasured, so a sum of
///    zero against an empty report is not called a contradiction.
///
/// A present `0.00` is a value.
pub fn gate_stock_summary(
    items: Vec<NativeStockItem>,
    item_count: NativeStockItemCount,
    report: &NativeStockReport,
) -> Result<NativeStockGate, NativeStockError> {
    let NativeStockItemCount::Reported(tally_count) = item_count else {
        return Err(NativeStockError::ItemCountUnavailable);
    };
    let rows = items.len();
    let rows_counted = u64::try_from(rows).unwrap_or(u64::MAX);
    if rows_counted != tally_count {
        return Ok(NativeStockGate::ItemCountDiffers { rows, tally_count });
    }
    let item_count = NativeItemCountCrossCheck {
        status: NativeItemCountStatus::Matched,
        rows,
        tally_count,
    };
    // `None` when no item carries a closing value: there is no figure on the
    // items' side, which is not a sum of zero.
    let items_total = if items.iter().any(|item| item.closing.value.is_some()) {
        Some(present_closing_value_sum(&items)?)
    } else {
        None
    };
    Ok(match report {
        NativeStockReport::Lines {
            total: Some(report_total),
            empty,
            ..
        } => match items_total {
            Some(sum) if sum.numeric_eq(report_total) => NativeStockGate::ValueTotalMatched {
                totals: NativeStockTotals::of(&items)?,
                items,
                total: report_total.clone(),
                report_empty_amounts: *empty,
                item_count,
            },
            None if report_total.is_zero() => NativeStockGate::NotComparable,
            items_total => NativeStockGate::Differs {
                items_total,
                report_total: report_total.clone(),
            },
        },
        // No rows, and so a count of zero: a higher count returned above.
        NativeStockReport::Empty if rows == 0 => NativeStockGate::NoStockItems,
        NativeStockReport::Lines { total: None, .. } | NativeStockReport::Empty => {
            match items_total {
                Some(sum) if !sum.is_zero() => {
                    NativeStockGate::ReportShowsNoValue { items_total: sum }
                }
                _ => NativeStockGate::NotComparable,
            }
        }
    })
}

#[cfg(test)]
#[path = "native_stock_summary_tests.rs"]
mod tests;
