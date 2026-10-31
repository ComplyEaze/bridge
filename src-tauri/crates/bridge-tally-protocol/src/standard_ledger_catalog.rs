//! Tally's standard List of Ledgers: the catalogue rows, their company binding,
//! and the identity observation, with display-name safety checks on every name.

use std::collections::HashSet;

use quick_xml::{events::Event, Reader};

use crate::{
    attr_value, configured_reader, normalized_standard_company_guid, normalized_standard_value,
    path_eq, pop_expected_path, read_identifier_text, read_optional_text, read_required_text,
    validate_export_response, validate_only_attributes, PartyLedgerMasterFieldObservation,
    TallyLedger,
};

/// Rows either parser in this file will hold from one `List of Ledgers`
/// response. Both parse the same request, and it returns every ledger in the
/// book, so this bound refuses a whole company, never one row.
///
/// It was 1,000, which refused every book over a thousand ledgers: a book of
/// about 9,500 ledgers lost the `vouchers` ledger filter and import validation
/// together (bridge#634). The response's real limit is the transport's 32 MiB
/// cap, counted on the wire, where Tally's XML is UTF-16LE: at most about 16.7
/// million characters. The smallest row the identity parser accepts is about
/// 121 characters plus the company name, so no admitted response can carry
/// more than about 140,000 rows. This bound is above that, and fires only if
/// the transport cap is raised without revisiting it.
///
/// It is not the limit a large book meets first. The master-binding catalogue
/// holds at most `bridge_tally_core::master_binding::MAX_CATALOG_ENTRIES`
/// names, and the desktop catalogue keeps the old 1,000 of its own.
pub const MAX_STANDARD_LEDGER_IDENTITY_ROWS: usize = 250_000;

/// A failed standard-ledger catalog is never a usable catalog. Keep the
/// failure class at the XML boundary so callers can retain their fail-closed
/// behavior without interpreting parser text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardLedgerCatalogError {
    MalformedResponse,
    /// A row's ledger name was refused on its own merits -- blank, past the
    /// length bound, or carrying a character that makes it read as a different
    /// name. Split out from `MalformedResponse` because the two say different
    /// things to whoever has to act: a malformed response is Tally or the
    /// transport, and this is one master in an otherwise well-formed export.
    ///
    /// Diagnosing a real instance of this took four rounds of instrumentation
    /// precisely because it arrived as `MalformedResponse` -- the response was
    /// not malformed at all, and every hypothesis started from the wrong half
    /// of the system.
    LedgerNameUnusable,
    CompanyIdentityMismatch,
    DuplicateIdentity,
    BoundsViolation,
    /// A V2 catalogue row carried no `ISBILLWISEON`. A V1 body, which has none,
    /// fails V2 here and nowhere else: the flag is never defaulted.
    BillWiseFlagMissing,
    /// A V2 catalogue row's `ISBILLWISEON` was empty or neither `Yes` nor `No`.
    BillWiseFlagInvalid,
    /// A V2 catalogue row carried `ISBILLWISEON` more than once.
    BillWiseFlagRepeated,
    /// A name-index row's `ALTERID` was absent, empty, repeated or not an
    /// unsigned decimal number.
    AlterIdInvalid,
}

impl std::fmt::Display for StandardLedgerCatalogError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::MalformedResponse => "standard ledger catalog response was malformed",
            Self::LedgerNameUnusable => "standard ledger catalog held an unusable ledger name",
            Self::CompanyIdentityMismatch => {
                "standard ledger catalog did not confirm the selected company"
            }
            Self::DuplicateIdentity => "standard ledger catalog contained a duplicate identity",
            Self::BoundsViolation => "standard ledger catalog exceeded a safety bound",
            Self::BillWiseFlagMissing => "standard ledger catalog row had no bill-wise flag",
            Self::BillWiseFlagInvalid => {
                "standard ledger catalog row had an unusable bill-wise flag"
            }
            Self::BillWiseFlagRepeated => "standard ledger catalog row repeated its bill-wise flag",
            Self::AlterIdInvalid => "standard ledger catalog row had an unusable alter id",
        })
    }
}

impl std::error::Error for StandardLedgerCatalogError {}

impl StandardLedgerCatalogError {
    /// A stable, data-free name for the failure, for a refusal's `cause`
    /// (bridge#634). None of these names a ledger.
    pub const fn safe_code(self) -> &'static str {
        match self {
            Self::MalformedResponse => "ledger_catalogue_malformed_response",
            Self::LedgerNameUnusable => "ledger_catalogue_name_unusable",
            Self::CompanyIdentityMismatch => "ledger_catalogue_identity_mismatch",
            Self::DuplicateIdentity => "ledger_catalogue_duplicate_identity",
            Self::BoundsViolation => "ledger_catalogue_bounds_exceeded",
            Self::BillWiseFlagMissing => "ledger_catalogue_bill_wise_flag_missing",
            Self::BillWiseFlagInvalid => "ledger_catalogue_bill_wise_flag_invalid",
            Self::BillWiseFlagRepeated => "ledger_catalogue_bill_wise_flag_repeated",
            Self::AlterIdInvalid => "ledger_catalogue_alter_id_invalid",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardLedgerIdentityObservation {
    pub company_guid: String,
    pub ledger_count: u64,
}

/// Validates the fixed, documented `List of Ledgers` collection used only to
/// bootstrap a scoped company identity on responders that reject Bridge's
/// custom report profile. Ledger names, balances, and identities are inspected
/// in memory only and never returned by this parser.
pub fn parse_standard_ledger_identity_observation(
    xml: &str,
    expected_company_name: &str,
) -> anyhow::Result<StandardLedgerIdentityObservation> {
    // The one rule every Tally read applies first (§1.1(d)).
    let marked = crate::mark_forbidden_numeric_references(xml);
    let xml = marked.as_ref();
    validate_export_response(xml)?;
    let expected_company_name = normalized_standard_value(expected_company_name, "company name")?;
    let mut reader = configured_reader(xml);
    let mut path = Vec::<Vec<u8>>::new();
    let mut ledger_count = 0_usize;
    let mut company_guid = None::<String>;
    loop {
        match reader.read_event()? {
            Event::Start(element)
                if path_eq(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) =>
            {
                if ledger_count >= MAX_STANDARD_LEDGER_IDENTITY_ROWS {
                    anyhow::bail!(
                        "standard ledger identity collection exceeded the safe row limit"
                    );
                }
                let observed =
                    parse_standard_ledger_identity_row(&mut reader, &element, false, RowShape::V1)?;
                if observed.company_name != expected_company_name {
                    anyhow::bail!(
                        "standard ledger identity collection did not confirm the requested company"
                    );
                }
                if let Some(previous) = &company_guid {
                    if previous != &observed.company_guid {
                        anyhow::bail!(
                            "standard ledger identity collection contained inconsistent company context"
                        );
                    }
                } else {
                    company_guid = Some(observed.company_guid);
                }
                ledger_count += 1;
            }
            Event::Start(element) => {
                path.push(element.name().as_ref().as_bytes().to_ascii_uppercase())
            }
            Event::Empty(_element)
                if path_eq(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) =>
            {
                anyhow::bail!("standard ledger identity collection contained an empty row");
            }
            Event::End(element) => {
                pop_expected_path(&mut path, element.name().as_ref().as_bytes())?
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if !path.is_empty() {
        anyhow::bail!("standard ledger identity collection ended before its root closed");
    }
    Ok(StandardLedgerIdentityObservation {
        company_guid: company_guid.ok_or_else(|| {
            anyhow::anyhow!(
                "standard ledger identity collection did not return a usable ledger row"
            )
        })?,
        ledger_count: ledger_count as u64,
    })
}

/// Opaque identities from one validated standard catalog. GUIDs remain
/// internal to the admission path and are never serialized into a tool result
/// or desktop review.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardLedgerCatalog {
    entries: Vec<StandardLedgerCatalogEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StandardLedgerCatalogEntry {
    /// The spelling of the row's `NAME` attribute: what every voucher row of
    /// this ledger carries.
    name: String,
    /// The ledger's own name when `LANGUAGENAME.LIST` gives a usable one that
    /// differs from `name` (in case or symbols: 26 of 4,017 ledgers in a separate census of
    /// 13 books, reference 9.4h, not reproducible from this repository); `None` otherwise. The identity is the GUID; neither
    /// spelling is.
    stored_name: Option<String>,
    /// The names after the first in the first `LANGUAGENAME.LIST`: Tally's
    /// aliases of this ledger (reference 9.4e, 9.4h), verbatim, each usable as
    /// a ledger name and different from `name` and `stored_name`. They name
    /// this ledger but are never its identity, and no request is built from
    /// one: they only let a caller say which ledger a name belongs to.
    aliases: Vec<String>,
    guid: String,
    /// The immediate `PARENT` group Tally returned for this ledger, or `None`
    /// when it returned none. A ledger exposes no `PARENTSTRUCTURE`, so this
    /// single hop is all the ancestry one catalog response carries; a caller
    /// that needs the group's own identity must read the Group collection.
    parent: Option<String>,
    /// The catalogue returned a `PARENT` this parse would not carry (see
    /// `safe_standard_ledger_parent`), so `parent` is `None` although the
    /// ledger has one.
    parent_unsupported: bool,
}

impl StandardLedgerCatalog {
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|entry| entry.name.as_str())
    }

    /// Each ledger's row spelling and, only when `LANGUAGENAME.LIST` gave a
    /// different usable one, its stored name. A ledger without a stored name
    /// is known by its row spelling alone, as before.
    pub fn spellings(&self) -> impl Iterator<Item = (&str, Option<&str>)> {
        self.entries
            .iter()
            .map(|entry| (entry.name.as_str(), entry.stored_name.as_deref()))
    }

    /// Every alias of every ledger, each paired with the row spelling of the
    /// ledger that carries it. A ledger with no alias yields nothing.
    pub fn aliases(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().flat_map(|entry| {
            entry
                .aliases
                .iter()
                .map(|alias| (entry.name.as_str(), alias.as_str()))
        })
    }

    /// Each ledger paired with the immediate parent group Tally returned for
    /// it. `None` is an unobserved parent, never an empty group name.
    pub fn parents(&self) -> impl Iterator<Item = (&str, Option<&str>)> {
        self.entries
            .iter()
            .map(|entry| (entry.name.as_str(), entry.parent.as_deref()))
    }

    /// Each ledger's name, GUID and immediate parent, for planning parent
    /// parts and checking the parts' rows against this catalogue (bridge#679).
    pub fn identified_parents(
        &self,
    ) -> impl Iterator<Item = (&str, &str, crate::parent_partition::ParentObservation<'_>)> {
        use crate::parent_partition::ParentObservation;
        self.entries.iter().map(|entry| {
            let parent = if entry.parent_unsupported {
                ParentObservation::Unsupported
            } else {
                entry.parent.as_deref().into()
            };
            (entry.name.as_str(), entry.guid.as_str(), parent)
        })
    }

    pub fn bind_selected(
        &self,
        requested_names: impl IntoIterator<Item = String>,
    ) -> anyhow::Result<StandardLedgerCatalogBinding> {
        let mut requested = requested_names.into_iter().collect::<Vec<_>>();
        requested.sort();
        requested.dedup();
        let entries = requested
            .into_iter()
            .map(|name| {
                let entry = self
                    .entries
                    .iter()
                    .find(|candidate| candidate.name == name)
                    .ok_or_else(|| {
                        anyhow::anyhow!("standard ledger catalog omitted requested ledger")
                    })?;
                Ok((name, entry.guid.clone()))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(StandardLedgerCatalogBinding { entries })
    }
}

/// A ledger's `ISBILLWISEON`: maintained bill by bill, or not. Two states and no default: a row that does not say is refused, so
/// "not maintained bill by bill" is never what an absent or unreadable flag turns into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillWiseFlag {
    On,
    Off,
}

impl BillWiseFlag {
    fn parse(text: &str) -> Result<Self, StandardLedgerCatalogError> {
        // After the reader's trimming, exactly the two spellings observed live
        // (§12a.15); any other is refused until a capture shows Tally uses it (P1).
        if text == "Yes" {
            Ok(Self::On)
        } else if text == "No" {
            Ok(Self::Off)
        } else {
            Err(StandardLedgerCatalogError::BillWiseFlagInvalid)
        }
    }
}

/// One validated `StandardLedgerCatalogV2` answer: the V1 catalogue plus every
/// ledger's [`BillWiseFlag`], from the one row. Only
/// [`parse_standard_ledger_catalog_v2_with_identities`] makes one, so every
/// ledger here has a flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardLedgerCatalogV2 {
    catalog: StandardLedgerCatalog,
    /// `flags[i]` is the flag of `catalog.entries[i]`.
    flags: Vec<BillWiseFlag>,
}

impl StandardLedgerCatalogV2 {
    /// The names, GUIDs and parents, exactly as a V1 answer would give them.
    pub fn catalog(&self) -> &StandardLedgerCatalog {
        &self.catalog
    }

    /// Each ledger's name, GUID and bill-wise flag, in response order.
    pub fn bill_wise_flags(&self) -> impl Iterator<Item = (&str, &str, BillWiseFlag)> {
        self.catalog
            .entries
            .iter()
            .zip(&self.flags)
            .map(|(entry, flag)| (entry.name.as_str(), entry.guid.as_str(), *flag))
    }
}

/// [`parse_standard_ledger_catalog_with_identities`] for the V2 request: the
/// same checks, and in addition exactly one `ISBILLWISEON` per row, `Yes` or
/// `No`. A V1 body has none and is refused ([`StandardLedgerCatalogError::BillWiseFlagMissing`]).
pub fn parse_standard_ledger_catalog_v2_with_identities(
    xml: &str,
    expected_company_name: &str,
    expected_company_guid: &str,
) -> Result<StandardLedgerCatalogV2, StandardLedgerCatalogError> {
    let rows = parse_standard_ledger_catalog_rows(
        xml,
        Some(expected_company_name),
        expected_company_guid,
        RowShape::V2,
    )?;
    let flags = rows
        .iter()
        .map(|row| {
            row.bill_wise
                .ok_or(StandardLedgerCatalogError::BillWiseFlagMissing)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(StandardLedgerCatalogV2 {
        catalog: catalog_from_rows(rows),
        flags,
    })
}

/// Opaque selected-master identities from one validated standard catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardLedgerCatalogBinding {
    entries: Vec<(String, String)>,
}

impl StandardLedgerCatalogBinding {
    /// Each selected ledger's observed name with the GUID it was bound to, in
    /// name order. For recording a build's binding, so a later post can tell
    /// a ledger renamed and replaced under its old name (bridge#239).
    pub fn pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(name, guid)| (name.as_str(), guid.as_str()))
    }

    /// True when every selected pair is still present in an already-parsed
    /// catalog.
    ///
    /// TALLY_PROTOCOL_REFERENCE.md §12a.9: Tally can retain a GUID while
    /// changing a visible ledger name, so admission binds the selected pair
    /// rather than either half.
    ///
    /// Callers holding a parsed response should prefer this: a caller checking
    /// several bindings against one response would otherwise reparse it once
    /// per binding.
    pub fn matches_catalog(&self, current: &StandardLedgerCatalog) -> bool {
        self.entries.iter().all(|(name, guid)| {
            current.entries.iter().any(|candidate| {
                &candidate.name == name && candidate.guid.eq_ignore_ascii_case(guid)
            })
        })
    }

    /// [`Self::matches_catalog`] for a caller that holds only the response body.
    pub fn matches(
        &self,
        xml: &str,
        expected_company_name: &str,
        expected_company_guid: &str,
    ) -> Result<bool, StandardLedgerCatalogError> {
        let current = parse_standard_ledger_catalog_with_identities(
            xml,
            expected_company_name,
            expected_company_guid,
        )?;
        Ok(self.matches_catalog(&current))
    }
}

/// Parses the standard catalog once, retaining source GUIDs only in an opaque
/// in-memory value so an admission caller can select bindings without a second
/// parse of the same response.
pub fn parse_standard_ledger_catalog_with_identities(
    xml: &str,
    expected_company_name: &str,
    expected_company_guid: &str,
) -> Result<StandardLedgerCatalog, StandardLedgerCatalogError> {
    let rows = parse_standard_ledger_catalog_rows(
        xml,
        Some(expected_company_name),
        expected_company_guid,
        RowShape::V1,
    )?;
    Ok(catalog_from_rows(rows))
}

fn catalog_from_rows(rows: Vec<StandardLedgerCatalogRow>) -> StandardLedgerCatalog {
    StandardLedgerCatalog {
        entries: rows
            .into_iter()
            .map(|row| {
                let aliases =
                    distinct_aliases(row.aliases, &row.ledger.name, row.stored_name.as_deref());
                StandardLedgerCatalogEntry {
                    stored_name: row.stored_name.filter(|stored| *stored != row.ledger.name),
                    aliases,
                    name: row.ledger.name,
                    guid: row.guid,
                    parent: row
                        .ledger
                        .parent
                        .nonempty_returned_text()
                        .map(str::to_string),
                    parent_unsupported: row.parent_unsupported,
                }
            })
            .collect(),
    }
}

/// The aliases a ledger keeps: each once, in the order Tally listed them, and
/// none that is the ledger's row spelling or its own name (a name the ledger
/// already answers to is not an alias of it).
fn distinct_aliases(listed: Vec<String>, name: &str, stored_name: Option<&str>) -> Vec<String> {
    let mut kept = Vec::<String>::new();
    for alias in listed {
        if alias != name && Some(alias.as_str()) != stored_name && !kept.contains(&alias) {
            kept.push(alias);
        }
    }
    kept
}

/// Parses the documented `List of Ledgers` collection as a deliberately
/// limited interactive catalog. The source GUIDs prove row uniqueness and
/// company scope in memory only; callers receive no GUIDs or raw XML.
pub fn parse_standard_ledger_catalog(
    xml: &str,
    expected_company_name: &str,
    expected_company_guid: &str,
) -> Result<Vec<TallyLedger>, StandardLedgerCatalogError> {
    Ok(parse_standard_ledger_catalog_rows(
        xml,
        Some(expected_company_name),
        expected_company_guid,
        RowShape::V1,
    )?
    .into_iter()
    .map(|row| row.ledger)
    .collect())
}

/// One ledger of the name index a ledger Create is checked against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerNameRow {
    /// Every `NAME` of every `NAME.LIST` under the row's `LANGUAGENAME.LIST`
    /// elements, in document order: the stored name first, then the aliases.
    /// Never empty. Verbatim, as Tally wrote them; the caller folds.
    pub names: Vec<String>,
    /// The row's `NAME` attribute when it is none of `names`. Tally leaves it
    /// at the old spelling, until it restarts, when a Create it resolved to
    /// this ledger rewrites the stored name
    /// (`tests/fixtures/agent/masters-lab.PROVENANCE.md`), so the book may
    /// still answer to it.
    pub row_spelling: Option<String>,
    /// A reserved ledger's built-in name (`RESERVEDNAME`) when it is neither
    /// one of `names` nor the row spelling: the name a user renamed it from,
    /// which the book may still answer to (not measured).
    pub reserved_name: Option<String>,
    /// The ledger's own `GUID`, unique across the answer.
    pub guid: String,
    /// The ledger's `ALTERID`.
    pub alter_id: u64,
}

/// Parses the V3 catalogue answer ([`crate::xml_read_profiles::ReadOnlyProfile::StandardLedgerCatalogV3`])
/// into the ledgers a new ledger's name is checked against, refusing what the
/// V1 and V2 parsers would pass where a gap could hide a name: a row without an
/// `ALTERID` or a `LANGUAGENAME.LIST`, an empty or unusable name or alias, and
/// anything but names or markup inside a `NAME.LIST`. A refused answer is no
/// index, so the caller refuses the write. The company is bound by its GUID on
/// every row. What it cannot check is that the answer holds every ledger:
/// nothing in it states a row count (its `CMPINFO` `LEDGER` figure is not one),
/// so a ledger Tally left out of the collection would read as absent.
pub fn parse_ledger_name_index(
    xml: &str,
    expected_company_guid: &str,
) -> Result<Vec<LedgerNameRow>, StandardLedgerCatalogError> {
    parse_standard_ledger_catalog_rows(xml, None, expected_company_guid, RowShape::NameIndex)?
        .into_iter()
        .map(|row| {
            row.bill_wise
                .ok_or(StandardLedgerCatalogError::BillWiseFlagMissing)?;
            let alter_id = row
                .alter_id
                .ok_or(StandardLedgerCatalogError::AlterIdInvalid)?;
            if row.listed_names.is_empty() {
                return Err(StandardLedgerCatalogError::LedgerNameUnusable);
            }
            let row_spelling =
                (!row.listed_names.contains(&row.ledger.name)).then_some(row.ledger.name);
            let reserved_name = row.reserved_name.filter(|reserved| {
                !row.listed_names.contains(reserved) && row_spelling.as_ref() != Some(reserved)
            });
            Ok(LedgerNameRow {
                names: row.listed_names,
                row_spelling,
                reserved_name,
                guid: row.guid,
                alter_id,
            })
        })
        .collect()
}

/// The ledger GUIDs of one slice of the census (`crate::ledger_census`): the
/// response to [`crate::ledger_census::render_ledger_census_slice_request`].
/// A row carries the ledger's GUID and the company's GUID and nothing else that
/// is read (its name arrives in the row's attribute and in `LANGUAGENAME.LIST`
/// whatever is fetched, and is skipped here: only the count matters). A row
/// holding any other field is refused, so a response that is not the slice
/// that was asked for cannot be counted.
///
/// `max_rows` bounds what the parser will hold ([`StandardLedgerCatalogError::BoundsViolation`]
/// past it); it is not the slice's span. AlterIDs are distinct, so a slice
/// `(after, through]` holds at most `through - after` ledgers, and the census
/// (`crate::ledger_census::LedgerCensus::accept`) refuses a slice holding more.
/// No rows is a well-formed answer (a slice past every ledger); the census
/// decides what an empty count means.
pub fn parse_ledger_census_slice(
    xml: &str,
    expected_company_guid: &str,
    max_rows: u64,
) -> Result<Vec<String>, StandardLedgerCatalogError> {
    parse_census_slice(xml, expected_company_guid, max_rows, b"LEDGER")
}

/// The stock-item GUIDs of one slice of a stock-item census: the response to
/// [`crate::ledger_census::render_stock_item_census_slice_request`], read by the ledger census's
/// rules with `STOCKITEM` rows (#1233). Tested on that request's answers as a lab script captured
/// them (`STOCK_CAPTURE_PROVENANCE.md`); a read past the single-read mark has not yet run through
/// Bridge.
pub fn parse_stock_item_census_slice(
    xml: &str,
    expected_company_guid: &str,
    max_rows: u64,
) -> Result<Vec<String>, StandardLedgerCatalogError> {
    parse_census_slice(xml, expected_company_guid, max_rows, b"STOCKITEM")
}

/// The stock-group GUIDs of one slice of a stock-group census: the response to
/// [`crate::ledger_census::render_stock_group_census_slice_request`], read by the ledger census's
/// rules with `STOCKGROUP` rows (#1233). Tested on a captured answer, like the stock-item parser.
pub fn parse_stock_group_census_slice(
    xml: &str,
    expected_company_guid: &str,
    max_rows: u64,
) -> Result<Vec<String>, StandardLedgerCatalogError> {
    parse_census_slice(xml, expected_company_guid, max_rows, b"STOCKGROUP")
}

/// A census slice whose every row is a `row_tag` element; a row of any other tag is a response to
/// another request and is refused.
fn parse_census_slice(
    xml: &str,
    expected_company_guid: &str,
    max_rows: u64,
    row_tag: &[u8],
) -> Result<Vec<String>, StandardLedgerCatalogError> {
    let marked = crate::mark_forbidden_numeric_references(xml);
    let xml = marked.as_ref();
    validate_export_response(xml).map_err(|_| StandardLedgerCatalogError::MalformedResponse)?;
    let expected_company_guid = normalized_standard_company_guid(expected_company_guid)
        .map_err(|_| StandardLedgerCatalogError::BoundsViolation)?;
    let mut reader = configured_reader(xml);
    let mut path = Vec::<Vec<u8>>::new();
    let mut guids = Vec::<String>::new();
    let mut seen = HashSet::new();
    loop {
        match reader
            .read_event()
            .map_err(|_| StandardLedgerCatalogError::MalformedResponse)?
        {
            Event::Start(element)
                if path_eq(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) =>
            {
                if !element
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(row_tag)
                {
                    return Err(StandardLedgerCatalogError::MalformedResponse);
                }
                if guids.len() as u64 >= max_rows {
                    return Err(StandardLedgerCatalogError::BoundsViolation);
                }
                let row = parse_ledger_census_row(&mut reader, &element).map_err(|error| {
                    error
                        .downcast_ref::<StandardLedgerCatalogError>()
                        .copied()
                        .unwrap_or(StandardLedgerCatalogError::MalformedResponse)
                })?;
                if !row
                    .company_guid
                    .eq_ignore_ascii_case(&expected_company_guid)
                {
                    return Err(StandardLedgerCatalogError::CompanyIdentityMismatch);
                }
                let guid = row.ledger_guid.to_ascii_lowercase();
                if !seen.insert(guid.clone()) {
                    return Err(StandardLedgerCatalogError::DuplicateIdentity);
                }
                guids.push(guid);
            }
            Event::Start(element) => {
                path.push(element.name().as_ref().as_bytes().to_ascii_uppercase())
            }
            Event::Empty(_) if path_eq(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) => {
                return Err(StandardLedgerCatalogError::MalformedResponse);
            }
            Event::End(element) => pop_expected_path(&mut path, element.name().as_ref().as_bytes())
                .map_err(|_| StandardLedgerCatalogError::MalformedResponse)?,
            Event::Eof => break,
            _ => {}
        }
    }
    if path.is_empty() {
        Ok(guids)
    } else {
        Err(StandardLedgerCatalogError::MalformedResponse)
    }
}

struct LedgerCensusRow {
    company_guid: String,
    ledger_guid: String,
}

fn parse_ledger_census_row(
    reader: &mut Reader<&[u8]>,
    element: &quick_xml::events::BytesStart<'_>,
) -> anyhow::Result<LedgerCensusRow> {
    validate_only_attributes(element, &[b"NAME", b"RESERVEDNAME"])?;
    let row_name = element.name().as_ref().as_bytes().to_ascii_uppercase();
    let mut company_guid = None;
    let mut ledger_guid = None;
    loop {
        match reader.read_event()? {
            Event::Start(child) => {
                let child_name = child.name().as_ref().as_bytes().to_ascii_uppercase();
                match child_name.as_slice() {
                    b"NAME" => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        skip_standard_ledger_identity_child(reader, child_name)?;
                    }
                    b"LANGUAGENAME.LIST" => {
                        skip_standard_ledger_identity_child(reader, child_name)?
                    }
                    b"BRIDGECOMPANYGUID" => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        set_bootstrap_context_once(
                            &mut company_guid,
                            normalized_standard_company_guid(&read_required_text(
                                reader,
                                child.name(),
                            )?)?,
                            "company GUID",
                        )?;
                    }
                    b"GUID" => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        if ledger_guid
                            .replace(normalized_standard_value(
                                &read_required_text(reader, child.name())?,
                                "ledger GUID",
                            )?)
                            .is_some()
                        {
                            anyhow::bail!("ledger census row repeated ledger GUID");
                        }
                    }
                    _ => anyhow::bail!("ledger census row contained an unexpected field"),
                }
            }
            Event::End(end)
                if end
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(&row_name) =>
            {
                break
            }
            Event::Empty(_) => anyhow::bail!("ledger census row contained an empty field"),
            Event::Text(text) if !text.trim().is_empty() => {
                anyhow::bail!("ledger census row contained unexpected text")
            }
            Event::CData(_) | Event::DocType(_) | Event::PI(_) => {
                anyhow::bail!("ledger census row contained a forbidden XML construct")
            }
            Event::Eof => anyhow::bail!("ledger census row ended before closing"),
            _ => {}
        }
    }
    Ok(LedgerCensusRow {
        company_guid: company_guid
            .ok_or_else(|| anyhow::anyhow!("ledger census row omitted computed company GUID"))?,
        ledger_guid: ledger_guid
            .ok_or_else(|| anyhow::anyhow!("ledger census row omitted ledger GUID"))?,
    })
}

struct StandardLedgerCatalogRow {
    ledger: TallyLedger,
    stored_name: Option<String>,
    aliases: Vec<String>,
    guid: String,
    parent_unsupported: bool,
    /// `None` for V1; a V2 parse refuses a row without one in the row parser.
    bill_wise: Option<BillWiseFlag>,
    /// The name index's `ALTERID`; `None` for every other shape.
    alter_id: Option<u64>,
    /// The name index's names, in document order; empty for every other shape.
    listed_names: Vec<String>,
    /// The name index's non-empty `RESERVEDNAME`; `None` for every other shape.
    reserved_name: Option<String>,
}

/// Which catalogue request a body answers, and so which row fields it must
/// carry beyond the V1 identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RowShape {
    V1,
    /// One `ISBILLWISEON` per row.
    V2,
    /// The V3 request ([`parse_ledger_name_index`]): V2, one `ALTERID` per row,
    /// and every name of every `LANGUAGENAME.LIST`, read strictly.
    NameIndex,
}

impl RowShape {
    fn reads_bill_wise(self) -> bool {
        self != Self::V1
    }
}

/// `expected_company_name` is `None` only for the name index, which binds the
/// company by GUID as the group index does.
fn parse_standard_ledger_catalog_rows(
    xml: &str,
    expected_company_name: Option<&str>,
    expected_company_guid: &str,
    shape: RowShape,
) -> Result<Vec<StandardLedgerCatalogRow>, StandardLedgerCatalogError> {
    // The one rule every Tally read applies first (§1.1(d)): ledger names and
    // parents here must spell a forbidden reference exactly as the voucher
    // rows they are matched against do, and `&#4; Primary` must reach
    // `group_ancestry` as the reserved root rather than as a control
    // character `safe_standard_ledger_parent` would discard.
    let marked = crate::mark_forbidden_numeric_references(xml);
    let xml = marked.as_ref();
    validate_export_response(xml).map_err(|_| StandardLedgerCatalogError::MalformedResponse)?;
    let expected_company_name = expected_company_name
        .map(|name| normalized_standard_value(name, "company name"))
        .transpose()
        .map_err(|_| StandardLedgerCatalogError::BoundsViolation)?;
    let expected_company_guid = normalized_standard_company_guid(expected_company_guid)
        .map_err(|_| StandardLedgerCatalogError::BoundsViolation)?;
    let mut reader = configured_reader(xml);
    let mut path = Vec::<Vec<u8>>::new();
    let mut rows = Vec::new();
    let mut seen_names = HashSet::new();
    let mut seen_guids = HashSet::new();
    loop {
        match reader
            .read_event()
            .map_err(|_| StandardLedgerCatalogError::MalformedResponse)?
        {
            Event::Start(element)
                if path_eq(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) =>
            {
                if rows.len() >= MAX_STANDARD_LEDGER_IDENTITY_ROWS {
                    return Err(StandardLedgerCatalogError::BoundsViolation);
                }
                // The row parser reports through `anyhow`, so a class raised
                // inside it arrives boxed. Recover it rather than flattening
                // every failure to "malformed": a refused ledger name is not a
                // malformed response, and saying so sent a previous diagnosis
                // at the transport for three rounds.
                let observed =
                    parse_standard_ledger_identity_row(&mut reader, &element, true, shape)
                        .map_err(|error| {
                            error
                                .downcast_ref::<StandardLedgerCatalogError>()
                                .copied()
                                .unwrap_or(StandardLedgerCatalogError::MalformedResponse)
                        })?;
                if expected_company_name
                    .as_ref()
                    .is_some_and(|expected| observed.company_name != *expected)
                    || !observed
                        .company_guid
                        .eq_ignore_ascii_case(&expected_company_guid)
                {
                    return Err(StandardLedgerCatalogError::CompanyIdentityMismatch);
                }
                let ledger_name = observed
                    .ledger_name
                    .ok_or(StandardLedgerCatalogError::MalformedResponse)?;
                let ledger_guid = observed
                    .ledger_guid
                    .ok_or(StandardLedgerCatalogError::MalformedResponse)?;
                if !seen_names.insert(standard_ledger_name_comparison_key(&ledger_name))
                    || !seen_guids.insert(ledger_guid.to_ascii_lowercase())
                {
                    return Err(StandardLedgerCatalogError::DuplicateIdentity);
                }
                rows.push(StandardLedgerCatalogRow {
                    ledger: TallyLedger {
                        name: ledger_name,
                        parent: observed.parent,
                        party_gstin: PartyLedgerMasterFieldObservation::NotObserved,
                        opening_balance: None,
                    },
                    stored_name: observed.stored_name,
                    aliases: observed.aliases,
                    guid: ledger_guid,
                    parent_unsupported: observed.parent_unsupported,
                    bill_wise: observed.bill_wise,
                    alter_id: observed.alter_id,
                    listed_names: observed.listed_names,
                    reserved_name: observed.reserved_name,
                });
            }
            Event::Start(element) => {
                let name = element.name().as_ref().as_bytes().to_ascii_uppercase();
                if shape == RowShape::NameIndex && ledger_outside_collection(&path, &name) {
                    return Err(StandardLedgerCatalogError::MalformedResponse);
                }
                path.push(name)
            }
            Event::Empty(_) if path_eq(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) => {
                return Err(StandardLedgerCatalogError::MalformedResponse);
            }
            Event::Empty(element)
                if shape == RowShape::NameIndex
                    && ledger_outside_collection(
                        &path,
                        &element.name().as_ref().as_bytes().to_ascii_uppercase(),
                    ) =>
            {
                return Err(StandardLedgerCatalogError::MalformedResponse);
            }
            Event::End(element) => pop_expected_path(&mut path, element.name().as_ref().as_bytes())
                .map_err(|_| StandardLedgerCatalogError::MalformedResponse)?,
            Event::Eof => break,
            _ => {}
        }
    }
    if path.is_empty() && !rows.is_empty() {
        Ok(rows)
    } else {
        Err(StandardLedgerCatalogError::MalformedResponse)
    }
}

/// Whether a `LEDGER` element at `path` is somewhere the name-index walk would
/// step over it: anywhere but the collection, whose rows are read, and the
/// `CMPINFO` block, whose bare `<LEDGER>` figure is a counter and not a row. A
/// ledger the walk skipped would read as absent, and a Create of its name would
/// alter it, so the answer is refused instead, as the group index refuses a
/// stray `GROUP`.
fn ledger_outside_collection(path: &[Vec<u8>], name: &[u8]) -> bool {
    name == b"LEDGER"
        && !path_eq(path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"])
        && !path_eq(path, &[b"ENVELOPE", b"BODY", b"DESC", b"CMPINFO"])
}

struct StandardLedgerIdentityRow {
    company_name: String,
    company_guid: String,
    ledger_name: Option<String>,
    /// The first `NAME` of the first `LANGUAGENAME.LIST`, read only when the
    /// row's own name is wanted; see [`walk_standard_ledger_identity_child`].
    stored_name: Option<String>,
    /// Every later `NAME` of the same list: the ledger's aliases, as read.
    aliases: Vec<String>,
    ledger_guid: Option<String>,
    parent: PartyLedgerMasterFieldObservation,
    parent_unsupported: bool,
    bill_wise: Option<BillWiseFlag>,
    alter_id: Option<u64>,
    listed_names: Vec<String>,
    reserved_name: Option<String>,
}

/// `shape` V2 or later: the row must carry one `ISBILLWISEON`.
/// Without it, that element is as unexpected as any other field, so a V2 body
/// fails the V1 parse as surely as a V1 body fails the V2 one.
fn parse_standard_ledger_identity_row(
    reader: &mut Reader<&[u8]>,
    element: &quick_xml::events::BytesStart<'_>,
    include_ledger_name: bool,
    shape: RowShape,
) -> anyhow::Result<StandardLedgerIdentityRow> {
    validate_only_attributes(element, &[b"NAME", b"RESERVEDNAME"])?;
    let mut ledger_name = include_ledger_name
        .then(|| attr_value(element, b"NAME"))
        .flatten()
        .map(|value| observed_standard_ledger_name(&value))
        .transpose()?;
    // The name index also keeps a reserved ledger's built-in name, which the
    // book may still answer to after the ledger is renamed.
    let reserved_name = (shape == RowShape::NameIndex)
        .then(|| attr_value(element, b"RESERVEDNAME"))
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .map(|value| observed_standard_ledger_name(&value))
        .transpose()?;
    let row_name = element.name().as_ref().as_bytes().to_ascii_uppercase();
    let mut company_name = None;
    let mut company_guid = None;
    let mut ledger_guid = None;
    let mut parent = PartyLedgerMasterFieldObservation::NotObserved;
    let mut parent_seen = false;
    let mut parent_unsupported = false;
    let mut language_names = LanguageNames::default();
    let mut language_list_seen = false;
    let mut bill_wise = None::<BillWiseFlag>;
    let mut alter_id = None::<u64>;
    let mut listed_names = Vec::new();
    let name_index = shape == RowShape::NameIndex;
    loop {
        match reader.read_event()? {
            Event::Start(child) => {
                let child_name = child.name().as_ref().as_bytes().to_ascii_uppercase();
                match child_name.as_slice() {
                    b"ISBILLWISEON" if shape.reads_bill_wise() => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        // Every flag seen live is a Logical (§12a.15): a flag of
                        // another type, or with no type, is not read as one.
                        if attr_value(&child, b"TYPE").as_deref() != Some("Logical") {
                            return Err(StandardLedgerCatalogError::BillWiseFlagInvalid.into());
                        }
                        let text = read_optional_text(reader, child.name())?
                            .ok_or(StandardLedgerCatalogError::BillWiseFlagInvalid)?;
                        if bill_wise.replace(BillWiseFlag::parse(&text)?).is_some() {
                            return Err(StandardLedgerCatalogError::BillWiseFlagRepeated.into());
                        }
                    }
                    b"ALTERID" if name_index => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        let value = read_optional_text(reader, child.name())?
                            .and_then(|text| parse_alter_id(&text))
                            .ok_or(StandardLedgerCatalogError::AlterIdInvalid)?;
                        if alter_id.replace(value).is_some() {
                            return Err(StandardLedgerCatalogError::AlterIdInvalid.into());
                        }
                    }
                    b"NAME" if include_ledger_name => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        if ledger_name
                            .replace(observed_standard_ledger_name(&read_required_text(
                                reader,
                                child.name(),
                            )?)?)
                            .is_some()
                        {
                            anyhow::bail!("standard ledger collection repeated ledger name");
                        }
                    }
                    b"NAME" => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        skip_standard_ledger_identity_child(
                            reader,
                            child.name().as_ref().as_bytes().to_ascii_uppercase(),
                        )?;
                    }
                    b"BRIDGECOMPANYNAME" => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        set_bootstrap_context_once(
                            &mut company_name,
                            normalized_standard_value(
                                &read_required_text(reader, child.name())?,
                                "company name",
                            )?,
                            "company name",
                        )?;
                    }
                    b"BRIDGECOMPANYGUID" => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        set_bootstrap_context_once(
                            &mut company_guid,
                            normalized_standard_company_guid(&read_required_text(
                                reader,
                                child.name(),
                            )?)?,
                            "company GUID",
                        )?;
                    }
                    b"GUID" if include_ledger_name => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        if ledger_guid
                            .replace(normalized_standard_value(
                                &read_required_text(reader, child.name())?,
                                "ledger GUID",
                            )?)
                            .is_some()
                        {
                            anyhow::bail!("standard ledger collection repeated ledger GUID");
                        }
                    }
                    b"GUID" => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        skip_standard_ledger_identity_child(
                            reader,
                            child.name().as_ref().as_bytes().to_ascii_uppercase(),
                        )?;
                    }
                    b"PARENT" if include_ledger_name => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        if parent_seen {
                            anyhow::bail!("standard ledger collection repeated ledger parent");
                        }
                        parent_seen = true;
                        // `read_identifier_text`, not `read_optional_text`: the latter
                        // trims, which would hand `safe_standard_ledger_parent` an
                        // already-normalized name and defeat the byte preservation the
                        // function below exists to provide.
                        parent = match read_identifier_text(reader, child.name())? {
                            Some(value) => match safe_standard_ledger_parent(&value) {
                                Some(value) => PartyLedgerMasterFieldObservation::Returned(value),
                                None => {
                                    parent_unsupported = true;
                                    PartyLedgerMasterFieldObservation::NotObserved
                                }
                            },
                            None => PartyLedgerMasterFieldObservation::Returned(String::new()),
                        };
                    }
                    b"PARENT" => {
                        validate_only_attributes(&child, &[b"TYPE"])?;
                        skip_standard_ledger_identity_child(
                            reader,
                            child.name().as_ref().as_bytes().to_ascii_uppercase(),
                        )?;
                    }
                    // The name index takes every name of every list: each is a
                    // spelling a new ledger must not collide with.
                    b"LANGUAGENAME.LIST" if include_ledger_name && name_index => {
                        listed_names.extend(walk_name_index_language_list(
                            reader,
                            child.name().as_ref().as_bytes(),
                        )?);
                    }
                    // Only the first list can name the ledger: a later list, and
                    // every `NAME` after the first in it, is an alias.
                    b"LANGUAGENAME.LIST" if include_ledger_name && !language_list_seen => {
                        language_list_seen = true;
                        language_names = walk_standard_ledger_identity_child(
                            reader,
                            child.name().as_ref().as_bytes().to_ascii_uppercase(),
                            true,
                        )?;
                    }
                    b"LANGUAGENAME.LIST" => skip_standard_ledger_identity_child(
                        reader,
                        child.name().as_ref().as_bytes().to_ascii_uppercase(),
                    )?,
                    _ => anyhow::bail!(
                        "standard ledger identity collection contained an unexpected row field"
                    ),
                }
            }
            Event::End(end)
                if end
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(&row_name) =>
            {
                break
            }
            Event::Empty(child)
                if include_ledger_name && child.name().as_ref().eq_ignore_ascii_case("PARENT") =>
            {
                validate_only_attributes(&child, &[b"TYPE"])?;
                if parent_seen {
                    anyhow::bail!("standard ledger collection repeated ledger parent");
                }
                parent_seen = true;
                parent = PartyLedgerMasterFieldObservation::Returned(String::new());
            }
            Event::Empty(child)
                if shape.reads_bill_wise()
                    && child.name().as_ref().eq_ignore_ascii_case("ISBILLWISEON") =>
            {
                return Err(StandardLedgerCatalogError::BillWiseFlagInvalid.into());
            }
            Event::Empty(child)
                if name_index && child.name().as_ref().eq_ignore_ascii_case("ALTERID") =>
            {
                return Err(StandardLedgerCatalogError::AlterIdInvalid.into());
            }
            Event::Empty(_) => {
                anyhow::bail!("standard ledger identity collection contained an empty row field")
            }
            Event::Text(text) if !text.trim().is_empty() => {
                anyhow::bail!("standard ledger identity collection contained unexpected row text")
            }
            Event::CData(_) | Event::DocType(_) | Event::PI(_) => {
                anyhow::bail!(
                    "standard ledger identity collection contained a forbidden XML construct"
                )
            }
            Event::Eof => {
                anyhow::bail!("standard ledger identity collection row ended before closing")
            }
            _ => {}
        }
    }
    Ok(StandardLedgerIdentityRow {
        company_name: company_name.ok_or_else(|| {
            anyhow::anyhow!("standard ledger identity collection omitted computed company name")
        })?,
        company_guid: company_guid.ok_or_else(|| {
            anyhow::anyhow!("standard ledger identity collection omitted computed company GUID")
        })?,
        ledger_name,
        stored_name: language_names.stored_name,
        aliases: language_names.aliases,
        ledger_guid,
        parent,
        parent_unsupported,
        bill_wise,
        alter_id,
        listed_names,
        reserved_name,
    })
}

/// Walks one `LANGUAGENAME.LIST` of a name-index row to its closing tag and
/// returns every `NAME` of its one `NAME.LIST`, verbatim, in document order.
/// It fails closed where the lenient walk below leaves a name unread: an empty
/// or unusable name, a second `NAME.LIST`, or any element but `NAME.LIST` and
/// `LANGUAGEID` in the list, or but `NAME` in the `NAME.LIST`.
fn walk_name_index_language_list(
    reader: &mut Reader<&[u8]>,
    list_name: &[u8],
) -> anyhow::Result<Vec<String>> {
    let mut names = Vec::new();
    let mut name_lists = 0_u32;
    loop {
        match reader.read_event()? {
            Event::Start(child) if child.name().as_ref().eq_ignore_ascii_case("NAME.LIST") => {
                validate_only_attributes(&child, &[b"TYPE"])?;
                name_lists += 1;
                loop {
                    match reader.read_event()? {
                        Event::Start(name) if name.name().as_ref().eq_ignore_ascii_case("NAME") => {
                            validate_only_attributes(&name, &[b"TYPE"])?;
                            // Raw text first: markup or CDATA inside a name would
                            // otherwise become part of a name no key can match.
                            let raw = reader.read_text(name.name())?;
                            if raw.contains('<') {
                                return Err(StandardLedgerCatalogError::LedgerNameUnusable.into());
                            }
                            let value = quick_xml::escape::unescape(&raw)?;
                            if value.trim().is_empty() {
                                return Err(StandardLedgerCatalogError::LedgerNameUnusable.into());
                            }
                            names.push(observed_standard_ledger_name(&value)?);
                        }
                        Event::Empty(name) if name.name().as_ref().eq_ignore_ascii_case("NAME") => {
                            return Err(StandardLedgerCatalogError::LedgerNameUnusable.into());
                        }
                        Event::End(end)
                            if end.name().as_ref().eq_ignore_ascii_case("NAME.LIST") =>
                        {
                            break;
                        }
                        Event::Comment(_) => {}
                        _ => anyhow::bail!("ledger name index NAME.LIST held more than names"),
                    }
                }
            }
            Event::Start(child) if child.name().as_ref().eq_ignore_ascii_case("LANGUAGEID") => {
                validate_only_attributes(&child, &[b"TYPE"])?;
                read_optional_text(reader, child.name())?;
            }
            Event::End(end)
                if end
                    .name()
                    .as_ref()
                    .as_bytes()
                    .eq_ignore_ascii_case(list_name) =>
            {
                break
            }
            Event::Comment(_) => {}
            _ => anyhow::bail!("ledger name index LANGUAGENAME.LIST held an unexpected field"),
        }
    }
    if name_lists != 1 || names.is_empty() {
        return Err(StandardLedgerCatalogError::LedgerNameUnusable.into());
    }
    Ok(names)
}

/// An unsigned decimal integer, as Tally writes an `ALTERID` (the captures
/// carry a leading space, trimmed by the reader): digits only.
fn parse_alter_id(text: &str) -> Option<u64> {
    let digits = text.trim();
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

fn skip_standard_ledger_identity_child(
    reader: &mut Reader<&[u8]>,
    expected_name: Vec<u8>,
) -> anyhow::Result<()> {
    walk_standard_ledger_identity_child(reader, expected_name, false).map(|_| ())
}

/// The names a first `LANGUAGENAME.LIST` carries: the ledger's own name (its
/// first `NAME`) and the later ones, which are aliases.
#[derive(Debug, Default)]
struct LanguageNames {
    stored_name: Option<String>,
    aliases: Vec<String>,
}

/// Walks one row child to its closing tag, as a skip does. With
/// `capture_stored_name` it also returns the ledger's own name: the first
/// `NAME` of the first `NAME.LIST` under `LANGUAGENAME.LIST`, read verbatim
/// like the row's `NAME` attribute (reference 9.4h). Later names are aliases
/// and are never identity: they are returned beside it, each read like the
/// first and dropped when empty, not decodable or unusable as a ledger name. A
/// first name that is empty, self-closing, not decodable or unusable as a
/// ledger name leaves `None`, so the ledger keeps its row spelling, no alias is
/// promoted, and the walk fails no read the skip used to pass: only the XML
/// structure can fail it, as it could before.
fn walk_standard_ledger_identity_child(
    reader: &mut Reader<&[u8]>,
    expected_name: Vec<u8>,
    capture_stored_name: bool,
) -> anyhow::Result<LanguageNames> {
    let mut depth = 1_u32;
    let mut names = LanguageNames::default();
    let mut first_name_seen = false;
    let mut in_name_list = false;
    loop {
        match reader.read_event()? {
            Event::Start(child)
                if capture_stored_name
                    && depth == 2
                    && in_name_list
                    && !first_name_seen
                    && child.name().as_ref().eq_ignore_ascii_case("NAME") =>
            {
                first_name_seen = true;
                // Not `read_identifier_text`: its `?` on a bad entity would refuse the
                // whole catalogue for a name this walk can simply leave unread.
                let raw = reader.read_text(child.name())?;
                names.stored_name = usable_language_name(&raw);
            }
            Event::Start(child)
                if capture_stored_name
                    && depth == 2
                    && in_name_list
                    && first_name_seen
                    && child.name().as_ref().eq_ignore_ascii_case("NAME") =>
            {
                let raw = reader.read_text(child.name())?;
                names.aliases.extend(usable_language_name(&raw));
            }
            Event::Empty(child)
                if capture_stored_name
                    && depth == 2
                    && in_name_list
                    && child.name().as_ref().eq_ignore_ascii_case("NAME") =>
            {
                // An empty first name is still the first name: an alias after it is not.
                first_name_seen = true;
            }
            Event::Start(child)
                if capture_stored_name
                    && depth == 1
                    && child.name().as_ref().eq_ignore_ascii_case("NAME.LIST") =>
            {
                in_name_list = true;
                depth = depth.checked_add(1).ok_or_else(|| {
                    anyhow::anyhow!("standard ledger identity nesting exceeded limits")
                })?;
            }
            Event::Start(_) => {
                depth = depth.checked_add(1).ok_or_else(|| {
                    anyhow::anyhow!("standard ledger identity nesting exceeded limits")
                })?
            }
            Event::End(end) => {
                depth = depth.checked_sub(1).ok_or_else(|| {
                    anyhow::anyhow!(
                        "standard ledger identity collection closed an unexpected field"
                    )
                })?;
                if depth == 1 {
                    in_name_list = false;
                }
                if depth == 0 {
                    if !end
                        .name()
                        .as_ref()
                        .as_bytes()
                        .eq_ignore_ascii_case(&expected_name)
                    {
                        anyhow::bail!(
                            "standard ledger identity collection closed an unexpected field"
                        );
                    }
                    return Ok(names);
                }
            }
            Event::DocType(_) | Event::PI(_) => {
                anyhow::bail!(
                    "standard ledger identity collection contained a forbidden XML construct"
                )
            }
            Event::Eof => {
                anyhow::bail!("standard ledger identity collection field ended before closing")
            }
            _ => {}
        }
    }
}

/// A language name as text: unescaped, not blank and usable as a ledger name,
/// else `None`. Never an error: a name this walk cannot use is left unread.
fn usable_language_name(raw: &str) -> Option<String> {
    quick_xml::escape::unescape(raw)
        .ok()
        .map(|value| value.into_owned())
        .filter(|value| !value.trim().is_empty())
        .and_then(|value| observed_standard_ledger_name(&value).ok())
}

/// A ledger name is an **identity** value, not a display string: it is matched
/// against proposals by exact codepoint, and echoed back as the import spelling
/// that has to round-trip to Tally. So the only things it can be refused for are
/// the two that make it unusable as an identity -- absent, or past the bound this
/// parser is willing to hold.
///
/// It deliberately does **not** refuse control or bidi characters. Real books
/// hold ledger names with an embedded newline, and books migrated from other
/// software hold names with C1 bytes baked in by a double-encoding import. When
/// this refused them, one such master failed the entire company's catalog and
/// with it every read that needs one -- presence, a ledger-scoped voucher
/// window, and import validation. A malformed name must not remove a real
/// master, and must not be rewritten either: Tally matches by exact codepoint,
/// so a cleaned-up spelling addresses a ledger that does not exist.
///
/// What it still refuses is the set that makes a name *lie about itself*:
/// bidirectional overrides and zero-width characters, which can render one
/// spelling as another. A newline is ugly in a terminal and the renderer's
/// problem; a right-to-left override is a forged name and this parser's.
fn observed_standard_ledger_name(value: &str) -> Result<String, StandardLedgerCatalogError> {
    if value.trim().is_empty()
        || value.len() > 512
        || value.chars().any(deceptive_display_character)
    {
        return Err(StandardLedgerCatalogError::LedgerNameUnusable);
    }
    Ok(value.to_string())
}

fn standard_ledger_name_comparison_key(value: &str) -> String {
    value.to_lowercase()
}

/// Validates a ledger's `PARENT` without normalising it.
///
/// The emptiness test reads the trimmed view, but the value is retained
/// verbatim. A `PARENT` is a foreign reference to a group `NAME`, matched by
/// exact codepoint, so trimming here would silently resolve a pair that
/// [`group_ancestry`] is built to refuse — and it would do so upstream of the
/// walk, where the walk cannot see it.
fn safe_standard_ledger_parent(value: &str) -> Option<String> {
    if value.trim().is_empty() || value.len() > 1024 || value.chars().any(unsafe_display_character)
    {
        return None;
    }
    Some(value.to_string())
}

pub(crate) fn unsafe_display_character(value: char) -> bool {
    value.is_control() || deceptive_display_character(value)
}

/// The half of [`unsafe_display_character`] that is about *deception* rather
/// than about rendering: characters that reorder or hide the text around them,
/// so that the spelling shown is not the spelling stored.
///
/// Split out because the two halves earn different answers on an observed
/// master name. A control character there is a real, if untidy, name a book
/// genuinely holds; one of these is a name forged to read as another, and no
/// book has a legitimate reason to hold one.
fn deceptive_display_character(value: char) -> bool {
    matches!(
        value,
        // Deliberately NOT a single `U+200B..=U+200F` range. That span holds
        // ZWNJ (U+200C) and ZWJ (U+200D), which are **orthography**, not
        // deception: Devanagari and other Indic scripts need them to control
        // conjunct formation, and this repository's own fixtures are full of
        // Indic ledger names. Refusing them would make a legitimately spelled
        // Hindi or Marathi ledger fail the whole catalog -- the exact failure
        // the newline fix existed to remove.
        //
        // The trade-off is real and worth stating: a codepoint filter cannot
        // know that a ZWJ sits between two Devanagari consonants rather than
        // injected into ASCII, so admitting them re-admits a narrow version of
        // the deception this set exists to stop -- `Alpha<ZWJ> Traders` is
        // byte-distinct from `Alpha Traders` and renders the same. It is
        // bounded rather than closed: the fold and the token index keep the
        // joiner verbatim, so such a name tends to fail exact and token
        // matching instead of quietly aliasing a real master. That is a worse
        // guarantee than refusal and a far better one than breaking every
        // Indic book, which is what refusal actually cost.
        '\u{061C}'
            | '\u{200B}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2060}'
            | '\u{2066}'..='\u{206F}'
            | '\u{FEFF}'
    )
}

fn set_bootstrap_context_once(
    slot: &mut Option<String>,
    value: String,
    label: &str,
) -> anyhow::Result<()> {
    if slot.replace(value).is_some() {
        anyhow::bail!("standard ledger identity collection repeated computed {label}");
    }
    Ok(())
}

#[cfg(test)]
#[path = "standard_ledger_catalog_v2_tests.rs"]
mod v2_tests;
