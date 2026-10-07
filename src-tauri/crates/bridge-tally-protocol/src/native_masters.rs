//! Native `masters` collections: voucher types, godowns, units, stock
//! groups, cost centres and cost categories, each read as one Collection with the company GUID computed onto
//! every row.
//!
//! Evidence: one synthetic book on one licensed `TallyPrime` 7.1 for the first
//! four kinds, and three synthetic books for cost centres (two for cost categories)
//! (`tests/fixtures/MASTERS_CAPTURE_PROVENANCE.md`; PARTIAL). Every row of
//! every kind carried the computed `BRIDGECOMPANYGUID`; `NUMBERINGMETHOD`
//! took `Default`, `Automatic` and `Manual`. How another release or a larger
//! book answers is unmeasured.
//!
//! The parser is closed and fails closed with a typed error (AGENTS.md P3):
//! it is not the tolerant generic collection reader. Groups are not read
//! here; they stay on the group snapshot.
//!
//! The element-reading helpers at the end are `pub(crate)`: `native_stock_summary`
//! reads its collections with them rather than with a second copy.
use crate::native_ledger_guid_has_company_prefix;
use crate::native_trial_balance::guid_suffix_is_valid;
use crate::tolerant_xml::sanitize_invalid_numeric_references;
use crate::xml_text::escape_text as xml_escape;
use quick_xml::{
    events::{BytesStart, BytesText, Event},
    name::QName,
    Reader,
};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// The longest name, in characters, a master is assumed to carry. ASSUMED,
/// until #917 measures the real limit: a row whose
/// name, parent, alias or other name-bearing text is longer is refused as
/// `masters_row_exceeds_bound`.
pub const MASTERS_ASSUMED_NAME_CHARS: usize = 128;

/// The most alias names a master is assumed to carry beside its own name.
/// ASSUMED, unmeasured: a row with more, counted across every language list
/// it carries, is refused as `masters_row_exceeds_bound`.
pub const MASTERS_ASSUMED_ALIASES: usize = 4;

/// The most response bytes (UTF-16) one masters read is admitted for. Bridge's
/// own bound, under the transport's response cap; not a measured limit. The
/// compliance master read's budget is this constant too.
pub const MASTERS_RESPONSE_BUDGET_BYTES: usize = 16_000_000;

/// The kinds `masters` reads as their own collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeMasterKind {
    VoucherTypes,
    Godowns,
    Units,
    StockGroups,
    CostCentres,
    CostCategories,
}

impl NativeMasterKind {
    pub const ALL: [Self; 6] = [
        Self::VoucherTypes,
        Self::Godowns,
        Self::Units,
        Self::StockGroups,
        Self::CostCentres,
        Self::CostCategories,
    ];

    /// The element Tally names each row of this kind by.
    const fn element(self) -> &'static [u8] {
        match self {
            Self::VoucherTypes => b"VOUCHERTYPE",
            Self::Godowns => b"GODOWN",
            Self::Units => b"UNIT",
            Self::StockGroups => b"STOCKGROUP",
            Self::CostCentres => b"COSTCENTRE",
            Self::CostCategories => b"COSTCATEGORY",
        }
    }

    /// Collection name and `ISMODIFY`, master `TYPE` and `FETCH`. Voucher
    /// types extend Tally's own `List of VoucherTypes`, the others define
    /// their own collection (`tests/fixtures/masters_*_request*`). A unit's
    /// `ORIGINALNAME` is fetched, so the request stays byte-equal to the
    /// committed one, but it is not exposed until a capture shows it
    /// populated: it was absent on every captured unit.
    const fn request_shape(self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::VoucherTypes => (
                "List of VoucherTypes",
                "Yes",
                "VoucherType",
                "NAME, PARENT, GUID, MASTERID, ALTERID, ISACTIVE, ISOPTIONAL, NUMBERINGMETHOD",
            ),
            Self::Godowns => (
                "Bridge Master Godowns",
                "No",
                "Godown",
                "NAME, PARENT, GUID, MASTERID, ALTERID",
            ),
            Self::Units => (
                "Bridge Master Units",
                "No",
                "Unit",
                "NAME, GUID, MASTERID, ALTERID, ORIGINALNAME, DECIMALPLACES, ISSIMPLEUNIT",
            ),
            Self::StockGroups => (
                "Bridge Master Stock Groups",
                "No",
                "StockGroup",
                "NAME, PARENT, GUID, MASTERID, ALTERID",
            ),
            Self::CostCentres => (
                "Bridge Master Cost Centres",
                "No",
                "CostCentre",
                "NAME, PARENT, CATEGORY, GUID, MASTERID, ALTERID",
            ),
            Self::CostCategories => (
                "Bridge Master Cost Categories",
                "No",
                "CostCategory",
                "NAME, GUID, MASTERID, ALTERID, ALLOCATEREVENUE, ALLOCATENONREVENUE, AFFECTSSTOCK",
            ),
        }
    }

    /// The `MSTDEPTYPE` Tally printed on the `COLLECTION` element of every
    /// captured cost-centre (32) and cost-category (16) answer, including the
    /// zero-row one. It is required for those two kinds, so that an empty
    /// collection that was not resolved to the type asked for is not read as
    /// "none defined". The other kinds carry no such requirement.
    const fn collection_type(self) -> Option<&'static str> {
        match self {
            Self::CostCentres => Some("32"),
            Self::CostCategories => Some("16"),
            Self::VoucherTypes | Self::Godowns | Self::Units | Self::StockGroups => None,
        }
    }

    /// Characters of one row that do not grow with its names: 1,200, 800, 700
    /// and 750 for voucher types, godowns, units and stock groups, and 800 and
    /// 950 for cost centres and cost categories (their largest captured rows,
    /// counted the same way, are 636 and 723 characters). The largest
    /// SHAPE LAB row, counted from its opening to its closing tag with the
    /// capture's line ends and its names included, is 900, 567, 474 and 524
    /// characters, so these carry headroom over that one synthetic book
    /// (PARTIAL). Names are counted here and again in
    /// [`masters_worst_row_bytes`], which only adds to the headroom.
    const fn fixed_chars(self) -> usize {
        match self {
            Self::VoucherTypes => 1_200,
            Self::Godowns => 800,
            Self::Units => 700,
            Self::StockGroups => 750,
            Self::CostCentres => 800,
            Self::CostCategories => 950,
        }
    }

    /// Name-bearing texts of a row before its aliases, each assumed at most
    /// [`MASTERS_ASSUMED_NAME_CHARS`] and checked against that bound when read:
    /// the `NAME` and `RESERVEDNAME` attributes, the row's own name element and
    /// one more (`PARENT`; `ORIGINALNAME` for a unit). A voucher type has a
    /// fifth, its `ALIAS` element. Aliases in the language lists are counted
    /// separately, per row.
    const fn name_slots(self) -> usize {
        match self {
            Self::VoucherTypes => 5,
            Self::Godowns | Self::Units | Self::StockGroups => 4,
            Self::CostCentres => 5,
            Self::CostCategories => 3,
        }
    }
}

/// An assumed worst-case row of `kind`, in UTF-16 bytes, given the assumed name
/// and alias bounds. Six UTF-16 units per character is the assumption (`&quot;`
/// is six; a numeric reference can be longer), enforced by the per-row span
/// check, which counts UTF-16 units against half of this after the read, and by
/// the response byte check. The runtime multiplies it by the master mark to
/// size a read before it: the estimate holds only while the mark bounds the row
/// count, which the runtime also checks.
pub const fn masters_worst_row_bytes(kind: NativeMasterKind) -> usize {
    2 * (kind.fixed_chars()
        + 6 * MASTERS_ASSUMED_NAME_CHARS * (kind.name_slots() + MASTERS_ASSUMED_ALIASES))
}

const _: () = {
    let mut index = 0;
    while index < NativeMasterKind::ALL.len() {
        let bytes = masters_worst_row_bytes(NativeMasterKind::ALL[index]);
        assert!(bytes > 0 && bytes < MASTERS_RESPONSE_BUDGET_BYTES);
        index += 1;
    }
};

/// Renders the collection request for one kind of master, binding the
/// response to its company by the computed `BRIDGECOMPANYGUID`.
pub fn render_native_masters_request(kind: NativeMasterKind, company: &str) -> String {
    let (collection, is_modify, master_type, fetch) = kind.request_shape();
    format!(
        r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>{collection}</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME="{collection}" ISMODIFY="{is_modify}"><TYPE>{master_type}</TYPE><FETCH>{fetch}</FETCH><COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"#,
        company = xml_escape(company),
    )
}

/// How a voucher type numbers its vouchers, as Tally reported it. A value
/// other than the three seen is kept raw (trimmed), refused only past the name
/// bound: live values differ by book. `Default` is Tally's reported value, not
/// evidence that the type numbers automatically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeNumberingMethod {
    Automatic,
    Manual,
    Default,
    Unrecognised(String),
}

/// What a kind reports beyond the fields every row has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeMasterDetail {
    Plain,
    VoucherType {
        active: Option<bool>,
        optional: Option<bool>,
        numbering: Option<NativeNumberingMethod>,
    },
    Unit {
        decimal_places: u8,
        simple: bool,
    },
    /// The category a cost centre belongs to, as the text Tally sent. Every
    /// captured centre carried one (the default `Primary Cost Category` where
    /// none was chosen), so an absent or blank one is refused, not read as none.
    CostCentre {
        category: String,
    },
    CostCategory {
        allocates_revenue: bool,
        allocates_non_revenue: bool,
        affects_stock: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeMasterRow {
    pub name: String,
    pub guid: String,
    pub master_id: u64,
    pub alter_id: u64,
    /// The `PARENT` text as read, the reserved-root marker included exactly as
    /// the group snapshot keeps it; absent or blank is `None`.
    pub parent: Option<String>,
    pub detail: NativeMasterDetail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeMasters {
    pub rows: Vec<NativeMasterRow>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeMastersError {
    TallyReportedFailure,
    /// The envelope or a row is not the closed shape; the code names where.
    Malformed(&'static str),
    /// No `COLLECTION` at all. An empty one is a valid zero-row answer.
    CollectionAbsent,
    ForeignChild,
    RowWithoutName,
    /// The GUID is absent, or is not `<company GUID>-<eight hex digits>`.
    RowGuidForeign,
    /// `BRIDGECOMPANYGUID` is absent or names another company.
    RowCompanyMismatch,
    RowFieldInvalid(&'static str),
    DuplicateGuid,
    DuplicateName,
    /// A name or alias count over the assumed bounds.
    RowExceedsBound,
    /// A present, empty `COLLECTION` for voucher types. Every company has
    /// predefined voucher types, so none is no answer, not a zero-row one.
    VoucherTypesEmpty,
    /// A present, empty `COLLECTION` for cost categories. The predefined Primary
    /// Cost Category is expected in every book (UNVERIFIED for a book with no
    /// centre), so none is treated as no answer, not a zero-row one.
    CostCategoriesEmpty,
}

impl NativeMastersError {
    /// The refusal's stable, data-free code. A field that could not be read
    /// names itself after a colon, as `argument_invalid:<name>` does.
    pub fn code(&self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "masters_tally_reported_failure",
            Self::Malformed(code) => code,
            Self::CollectionAbsent => "masters_collection_absent",
            Self::ForeignChild => "masters_foreign_child",
            Self::RowWithoutName => "masters_row_without_name",
            Self::RowGuidForeign => "masters_row_guid_foreign",
            Self::RowCompanyMismatch => "masters_row_company_mismatch",
            Self::RowFieldInvalid(field) => match *field {
                "master_id" => "masters_row_field_invalid:master_id",
                "alter_id" => "masters_row_field_invalid:alter_id",
                "is_active" => "masters_row_field_invalid:is_active",
                "is_optional" => "masters_row_field_invalid:is_optional",
                "decimal_places" => "masters_row_field_invalid:decimal_places",
                "is_simple_unit" => "masters_row_field_invalid:is_simple_unit",
                "allocate_revenue" => "masters_row_field_invalid:allocate_revenue",
                "allocate_non_revenue" => "masters_row_field_invalid:allocate_non_revenue",
                "affects_stock" => "masters_row_field_invalid:affects_stock",
                "category" => "masters_row_field_invalid:category",
                _ => "masters_row_field_invalid",
            },
            Self::DuplicateGuid => "masters_row_duplicate_guid",
            Self::DuplicateName => "masters_row_duplicate_name",
            Self::RowExceedsBound => "masters_row_exceeds_bound",
            Self::VoucherTypesEmpty => "masters_voucher_types_empty",
            Self::CostCategoriesEmpty => "masters_cost_categories_empty",
        }
    }
}

impl fmt::Display for NativeMastersError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "native masters response invalid ({})",
            self.code()
        )
    }
}

impl std::error::Error for NativeMastersError {}

/// The scalar children a row's fields are read from, by upper-case name. One
/// occurrence each; a repeat refuses. `ORIGINALNAME`, `ALIAS` and `NAME` are
/// read only to check them against the name bound.
const ROW_FIELDS: [&str; 17] = [
    "GUID",
    "PARENT",
    "BRIDGECOMPANYGUID",
    "MASTERID",
    "ALTERID",
    "ISACTIVE",
    "ISOPTIONAL",
    "NUMBERINGMETHOD",
    "ORIGINALNAME",
    "DECIMALPLACES",
    "ISSIMPLEUNIT",
    "ALIAS",
    "NAME",
    "CATEGORY",
    "ALLOCATEREVENUE",
    "ALLOCATENONREVENUE",
    "AFFECTSSTOCK",
];

/// Parses one masters collection of `kind`, bound to `company_guid`.
pub fn parse_native_masters(
    kind: NativeMasterKind,
    response: &str,
    company_guid: &str,
) -> Result<NativeMasters, NativeMastersError> {
    let sanitized = sanitize_invalid_numeric_references(response);
    let mut reader = Reader::from_str(&sanitized);
    reader.config_mut().trim_text(false);
    let mut path = Vec::<Vec<u8>>::new();
    let mut root_seen = false;
    let mut status: Option<String> = None;
    let mut collections = 0_usize;
    let mut rows = Vec::new();
    let mut guids = HashSet::new();
    let mut names = HashSet::new();
    loop {
        let event_start = reader.buffer_position();
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(element) => {
                let name = upper(element.name());
                if path.is_empty() && (root_seen || name != b"ENVELOPE") {
                    return Err(NativeMastersError::Malformed("masters_root_not_envelope"));
                }
                refuse_error_element(&name)?;
                if path_is(&path, &[b"ENVELOPE", b"HEADER"]) && name == b"STATUS" {
                    let text = read_text(&mut reader, element.name())?;
                    let text = text.trim();
                    // An empty STATUS is no answer: the envelope's end refuses it.
                    if !text.is_empty() && text != "1" {
                        return Err(NativeMastersError::TallyReportedFailure);
                    }
                    record_status(&mut status, text.to_string())?;
                    continue;
                }
                if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA"]) && name == b"COLLECTION" {
                    require_collection_type(&element, kind)?;
                    collections += 1;
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) {
                    if name != kind.element() {
                        return Err(NativeMastersError::ForeignChild);
                    }
                    let row = parse_row(
                        &mut reader,
                        &element,
                        kind,
                        company_guid,
                        RowSpan {
                            text: &sanitized,
                            start: event_start,
                        },
                    )?;
                    if !guids.insert(row.guid.to_ascii_lowercase()) {
                        return Err(NativeMastersError::DuplicateGuid);
                    }
                    if !names.insert(row.name.to_ascii_lowercase()) {
                        return Err(NativeMastersError::DuplicateName);
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
                    return Err(NativeMastersError::Malformed("masters_root_not_envelope"));
                }
                if path_is(&path, &[b"ENVELOPE", b"HEADER"]) && name == b"STATUS" {
                    record_status(&mut status, String::new())?;
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA"]) && name == b"COLLECTION"
                {
                    require_collection_type(&element, kind)?;
                    collections += 1;
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) {
                    return Err(if name == kind.element() {
                        NativeMastersError::Malformed("masters_row_empty")
                    } else {
                        NativeMastersError::ForeignChild
                    });
                }
            }
            Event::End(_) => {
                if path.pop().is_none() {
                    return Err(NativeMastersError::Malformed("masters_unexpected_close"));
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
                return Err(NativeMastersError::Malformed("masters_unexpected_text"));
            }
            Event::DocType(_) => {
                return Err(NativeMastersError::Malformed("masters_doctype_forbidden"))
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if !path.is_empty() {
        return Err(NativeMastersError::Malformed(
            "masters_envelope_unterminated",
        ));
    }
    if !root_seen {
        return Err(NativeMastersError::Malformed("masters_envelope_missing"));
    }
    if status.as_deref() != Some("1") {
        return Err(NativeMastersError::Malformed("masters_status_absent"));
    }
    match collections {
        0 => Err(NativeMastersError::CollectionAbsent),
        // Every company has predefined voucher types, so a voucher-type answer
        // with no row is anomalous. The other kinds may legitimately have none
        // (a zero-row unit and stock-group answer was captured live).
        1 if rows.is_empty() && kind == NativeMasterKind::VoucherTypes => {
            Err(NativeMastersError::VoucherTypesEmpty)
        }
        // The predefined Primary Cost Category is expected to always exist (UNVERIFIED for a book with no centre); cost centres may be none
        // (a zero-row answer was captured live on a book with none defined).
        1 if rows.is_empty() && kind == NativeMasterKind::CostCategories => {
            Err(NativeMastersError::CostCategoriesEmpty)
        }
        1 => Ok(NativeMasters { rows }),
        _ => Err(NativeMastersError::Malformed("masters_collection_repeated")),
    }
}

/// Text directly under the collection, or under a row outside its fields, is
/// not part of the closed shape: only XML whitespace (space, tab, carriage
/// return, line feed) may sit between elements. `str::trim` is not that set: it
/// also drops U+00A0 and other Unicode spaces, which are text.
pub(crate) fn refuse_stray_text(text: &BytesText<'_>) -> Result<(), NativeMastersError> {
    let decoded = text
        .decode()
        .map_err(|_| NativeMastersError::Malformed("masters_xml_invalid_encoding"))?;
    if decoded.trim_matches([' ', '\t', '\r', '\n']).is_empty() {
        Ok(())
    } else {
        Err(NativeMastersError::Malformed("masters_unexpected_text"))
    }
}

fn record_status(slot: &mut Option<String>, value: String) -> Result<(), NativeMastersError> {
    if slot.replace(value).is_some() {
        return Err(NativeMastersError::Malformed("masters_status_repeated"));
    }
    Ok(())
}

/// Where a row starts in the text being read, so its length can be measured.
struct RowSpan<'a> {
    text: &'a str,
    start: u64,
}

fn parse_row(
    reader: &mut Reader<&[u8]>,
    element: &BytesStart<'_>,
    kind: NativeMasterKind,
    company_guid: &str,
    span: RowSpan<'_>,
) -> Result<NativeMasterRow, NativeMastersError> {
    let name = name_attribute(element)?;
    let mut fields = HashMap::<&'static str, String>::new();
    let mut language_names = 0_usize;
    loop {
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(child) => {
                let child_name = upper(child.name());
                refuse_error_element(&child_name)?;
                if child_name == b"LANGUAGENAME.LIST" {
                    read_language_names(reader, &mut language_names)?;
                } else if let Some(field) = row_field(&child_name) {
                    let text = read_text(reader, child.name())?;
                    if fields.insert(field, text).is_some() {
                        return Err(NativeMastersError::Malformed("masters_row_field_repeated"));
                    }
                } else {
                    skip_subtree(reader)?;
                }
            }
            Event::Empty(child) => {
                let child_name = upper(child.name());
                refuse_error_element(&child_name)?;
                if let Some(field) = row_field(&child_name) {
                    if fields.insert(field, String::new()).is_some() {
                        return Err(NativeMastersError::Malformed("masters_row_field_repeated"));
                    }
                }
            }
            Event::End(end) if end.name().as_ref().eq_ignore_ascii_case(kind.element()) => break,
            Event::Text(text) => refuse_stray_text(&text)?,
            Event::CData(_) | Event::GeneralRef(_) => {
                return Err(NativeMastersError::Malformed("masters_unexpected_text"))
            }
            Event::Eof => return Err(NativeMastersError::Malformed("masters_row_unterminated")),
            _ => {}
        }
    }
    // The row as a whole, opening to closing tag, in UTF-16 units (the unit the
    // response is measured in): the bound on every text a row can carry,
    // whether or not this parser reads it.
    let row_chars = usize::try_from(span.start)
        .ok()
        .zip(usize::try_from(reader.buffer_position()).ok())
        .and_then(|(start, end)| span.text.get(start..end))
        .map_or(usize::MAX, |row| row.encode_utf16().count());
    if row_chars > masters_worst_row_bytes(kind) / 2 {
        return Err(NativeMastersError::RowExceedsBound);
    }
    for field in ["PARENT", "ALIAS", "NAME", "ORIGINALNAME", "CATEGORY"] {
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
        .ok_or(NativeMastersError::RowGuidForeign)?;
    match fields.remove("BRIDGECOMPANYGUID") {
        Some(bound) if bound.trim().eq_ignore_ascii_case(company_guid) => {}
        _ => return Err(NativeMastersError::RowCompanyMismatch),
    }
    let master_id = parsed(&mut fields, "MASTERID", "master_id")?;
    let alter_id = parsed(&mut fields, "ALTERID", "alter_id")?;
    let parent = match kind {
        NativeMasterKind::Units | NativeMasterKind::CostCategories => None,
        _ => fields
            .remove("PARENT")
            .filter(|parent| !parent.trim().is_empty()),
    };
    let detail = match kind {
        NativeMasterKind::VoucherTypes => NativeMasterDetail::VoucherType {
            active: optional_yes_no(&mut fields, "ISACTIVE", "is_active")?,
            optional: optional_yes_no(&mut fields, "ISOPTIONAL", "is_optional")?,
            numbering: fields
                .remove("NUMBERINGMETHOD")
                .map(|text| numbering_method(&text))
                .transpose()?,
        },
        // `ORIGINALNAME` is fetched and bound-checked above, not exposed.
        NativeMasterKind::Units => NativeMasterDetail::Unit {
            decimal_places: parsed(&mut fields, "DECIMALPLACES", "decimal_places")?,
            simple: optional_yes_no(&mut fields, "ISSIMPLEUNIT", "is_simple_unit")?
                .ok_or(NativeMastersError::RowFieldInvalid("is_simple_unit"))?,
        },
        NativeMasterKind::Godowns | NativeMasterKind::StockGroups => NativeMasterDetail::Plain,
        NativeMasterKind::CostCentres => NativeMasterDetail::CostCentre {
            category: fields
                .remove("CATEGORY")
                .map(|text| text.trim().to_string())
                .filter(|text| !text.is_empty())
                .ok_or(NativeMastersError::RowFieldInvalid("category"))?,
        },
        NativeMasterKind::CostCategories => NativeMasterDetail::CostCategory {
            allocates_revenue: required_yes_no(&mut fields, "ALLOCATEREVENUE", "allocate_revenue")?,
            allocates_non_revenue: required_yes_no(
                &mut fields,
                "ALLOCATENONREVENUE",
                "allocate_non_revenue",
            )?,
            affects_stock: required_yes_no(&mut fields, "AFFECTSSTOCK", "affects_stock")?,
        },
    };
    Ok(NativeMasterRow {
        name,
        guid,
        master_id,
        alter_id,
        parent,
        detail,
    })
}

/// Counts the names under `LANGUAGENAME.LIST/NAME.LIST/NAME`, which are the
/// row's aliases, into `names` (one count per row, across every language
/// list it carries), and refuses one past the assumed bounds. Aliases are
/// not kept.
pub(crate) fn read_language_names(
    reader: &mut Reader<&[u8]>,
    names: &mut usize,
) -> Result<(), NativeMastersError> {
    let mut path = Vec::<Vec<u8>>::new();
    loop {
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(element) => {
                let name = upper(element.name());
                refuse_error_element(&name)?;
                if path_is(&path, &[b"NAME.LIST"]) && name == b"NAME" {
                    *names += 1;
                    within_name_bound(&read_text(reader, element.name())?)?;
                    if *names > 1 + MASTERS_ASSUMED_ALIASES {
                        return Err(NativeMastersError::RowExceedsBound);
                    }
                } else {
                    path.push(name);
                }
            }
            Event::Empty(element) => {
                let name = upper(element.name());
                refuse_error_element(&name)?;
                if path_is(&path, &[b"NAME.LIST"]) && name == b"NAME" {
                    *names += 1;
                    if *names > 1 + MASTERS_ASSUMED_ALIASES {
                        return Err(NativeMastersError::RowExceedsBound);
                    }
                }
            }
            Event::End(_) => {
                if path.pop().is_none() {
                    return Ok(());
                }
            }
            Event::Eof => return Err(NativeMastersError::Malformed("masters_row_unterminated")),
            _ => {}
        }
    }
}

pub(crate) fn skip_subtree(reader: &mut Reader<&[u8]>) -> Result<(), NativeMastersError> {
    let mut depth = 1_u32;
    loop {
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(element) => {
                refuse_error_element(&upper(element.name()))?;
                depth += 1;
            }
            Event::Empty(element) => refuse_error_element(&upper(element.name()))?,
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    return Ok(());
                }
            }
            Event::Eof => return Err(NativeMastersError::Malformed("masters_row_unterminated")),
            _ => {}
        }
    }
}

/// An element's text, refusing nested markup. Not trimmed: a name or parent
/// is matched by exact codepoint, as the group snapshot reads its `PARENT`.
pub(crate) fn read_text(
    reader: &mut Reader<&[u8]>,
    name: QName<'_>,
) -> Result<String, NativeMastersError> {
    let raw = reader
        .read_text(name)
        .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?;
    let decoded = raw
        .decode()
        .map_err(|_| NativeMastersError::Malformed("masters_xml_invalid_encoding"))?;
    if let Some((_, nested)) = decoded.split_once('<') {
        // Tally's own failure elements keep their meaning inside a scalar: the
        // first nested tag's name is compared exactly, so `<ERRORS>` is not one.
        let tag = nested
            .split(|c: char| c.is_whitespace() || c == '/' || c == '>')
            .next()
            .unwrap_or_default();
        return Err(
            if tag.eq_ignore_ascii_case("ERROR") || tag.eq_ignore_ascii_case("LINEERROR") {
                NativeMastersError::TallyReportedFailure
            } else {
                NativeMastersError::Malformed("masters_scalar_not_text_only")
            },
        );
    }
    quick_xml::escape::unescape(&decoded)
        .map(std::borrow::Cow::into_owned)
        .map_err(|_| NativeMastersError::Malformed("masters_xml_invalid_escape"))
}

/// For the kinds that name one, the `MSTDEPTYPE` the collection element must
/// carry (see [`NativeMasterKind::collection_type`]).
fn require_collection_type(
    element: &BytesStart<'_>,
    kind: NativeMasterKind,
) -> Result<(), NativeMastersError> {
    let Some(expected) = kind.collection_type() else {
        return Ok(());
    };
    for attribute in element.attributes() {
        let attribute =
            attribute.map_err(|_| NativeMastersError::Malformed("masters_attribute_malformed"))?;
        if attribute.key.as_ref().eq_ignore_ascii_case(b"MSTDEPTYPE") {
            let value = attribute
                .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                .map_err(|_| NativeMastersError::Malformed("masters_attribute_malformed"))?;
            if value.trim() == expected {
                return Ok(());
            }
        }
    }
    Err(NativeMastersError::Malformed(
        "masters_collection_type_unexpected",
    ))
}

/// The row's `NAME`, required and not blank. Its `RESERVEDNAME`, when present,
/// is checked against the name bound too.
pub(crate) fn name_attribute(element: &BytesStart<'_>) -> Result<String, NativeMastersError> {
    let mut name = None;
    for attribute in element.attributes() {
        let attribute =
            attribute.map_err(|_| NativeMastersError::Malformed("masters_attribute_malformed"))?;
        let key = attribute.key.as_ref();
        let is_name = key.eq_ignore_ascii_case(b"NAME");
        if !is_name && !key.eq_ignore_ascii_case(b"RESERVEDNAME") {
            continue;
        }
        let value = attribute
            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
            .map_err(|_| NativeMastersError::Malformed("masters_attribute_malformed"))?
            .into_owned();
        within_name_bound(&value)?;
        if is_name {
            name = Some(value);
        }
    }
    name.filter(|name| !name.trim().is_empty())
        .ok_or(NativeMastersError::RowWithoutName)
}

pub(crate) fn within_name_bound(name: &str) -> Result<(), NativeMastersError> {
    if name.chars().count() > MASTERS_ASSUMED_NAME_CHARS {
        return Err(NativeMastersError::RowExceedsBound);
    }
    Ok(())
}

fn row_field(name: &[u8]) -> Option<&'static str> {
    ROW_FIELDS
        .into_iter()
        .find(|field| field.as_bytes() == name)
}

fn parsed<T: std::str::FromStr>(
    fields: &mut HashMap<&'static str, String>,
    key: &'static str,
    label: &'static str,
) -> Result<T, NativeMastersError> {
    fields
        .remove(key)
        .and_then(|text| text.trim().parse().ok())
        .ok_or(NativeMastersError::RowFieldInvalid(label))
}

fn optional_yes_no(
    fields: &mut HashMap<&'static str, String>,
    key: &'static str,
    label: &'static str,
) -> Result<Option<bool>, NativeMastersError> {
    match fields.remove(key).as_deref().map(str::trim) {
        None => Ok(None),
        Some("Yes") => Ok(Some(true)),
        Some("No") => Ok(Some(false)),
        Some(_) => Err(NativeMastersError::RowFieldInvalid(label)),
    }
}

/// A `Yes` or `No` field that must be present: an absent one is not a `No`.
fn required_yes_no(
    fields: &mut HashMap<&'static str, String>,
    key: &'static str,
    label: &'static str,
) -> Result<bool, NativeMastersError> {
    optional_yes_no(fields, key, label)?.ok_or(NativeMastersError::RowFieldInvalid(label))
}

fn numbering_method(text: &str) -> Result<NativeNumberingMethod, NativeMastersError> {
    let text = text.trim();
    Ok(match text {
        "Automatic" => NativeNumberingMethod::Automatic,
        "Manual" => NativeNumberingMethod::Manual,
        "Default" => NativeNumberingMethod::Default,
        _ => {
            within_name_bound(text)?;
            NativeNumberingMethod::Unrecognised(text.to_string())
        }
    })
}

pub(crate) fn upper(name: QName<'_>) -> Vec<u8> {
    name.as_ref().to_ascii_uppercase()
}

pub(crate) fn refuse_error_element(name: &[u8]) -> Result<(), NativeMastersError> {
    if name == b"LINEERROR" || name == b"ERROR" {
        return Err(NativeMastersError::TallyReportedFailure);
    }
    Ok(())
}

pub(crate) fn path_is(path: &[Vec<u8>], expected: &[&[u8]]) -> bool {
    path.len() == expected.len()
        && path
            .iter()
            .zip(expected)
            .all(|(part, expected)| part.as_slice() == *expected)
}

#[cfg(test)]
#[path = "native_masters_tests.rs"]
mod tests;
