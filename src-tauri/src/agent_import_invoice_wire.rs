//! The three small reads an invoice build adds, and how their answers are read.
//!
//! Each answer is parsed at the edge into a value that cannot be wrong later:
//! a voucher type with its series-level numbering, a ledger's bill-wise flag,
//! the company's GST registration. Nothing here guesses: a missing element, a repeated
//! one, or a name two rows claim is an error, never a default.
//!
//! What each read was measured to return (TallyPrime 7.1 Silver, lab, 6 Oct
//! 2026, a client sample book): the voucher-type collection with no field
//! list returns every type in 0.1 s (26 types, about 12 KB each) with the
//! type's reserved name as an attribute, its PARENT, its GUID, a top-level
//! NUMBERINGMETHOD and, inside `VOUCHERNUMBERSERIES.LIST`, the series-level
//! one that decides what an import's supplied number does. A `Ledger` row
//! returns `ISBILLWISEON` as Yes or No. The tax-unit collection was read once
//! on a second synthetic lab company (TallyPrime 7.1 Silver, 9 Oct 2026; typed
//! by hand, sent as UTF-8, before any voucher was keyed): the
//! Default Tax Unit and one GST registration, each GUID the company's GUID
//! with a suffix, the registration's dated rows under
//! `GSTREGISTRATIONDETAILS.LIST`.

use bridge_tally_protocol::xml_text::escape_text as xml_escape;
use quick_xml::events::Event;
use std::collections::BTreeMap;

/// One element of a collection answer: its attributes and the text of every
/// descendant, keyed by the path from the row ("PARENT",
/// "VOUCHERNUMBERSERIES.LIST/NUMBERINGMETHOD"). A key can repeat.
#[derive(Debug, Default)]
pub(super) struct Row {
    pub(super) attributes: BTreeMap<String, String>,
    pub(super) fields: Vec<(String, String)>,
    /// Each direct child whose tag ends in `.LIST`, as its own row (its fields
    /// are relative to it): the ledger entries of a voucher.
    pub(super) lists: Vec<(String, Row)>,
}

impl Row {
    /// The one value at `path`, `Ok(None)` when absent, an error when repeated.
    pub(super) fn one(&self, path: &str) -> Result<Option<&str>, &'static str> {
        let mut found = self.fields.iter().filter(|(key, _)| key == path);
        let first = found.next().map(|(_, value)| value.as_str());
        if found.next().is_some() {
            return Err("invoice_read_field_repeated");
        }
        Ok(first)
    }

    pub(super) fn all<'a>(&'a self, path: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        self.fields
            .iter()
            .filter(move |(key, _)| key == path)
            .map(|(_, value)| value.as_str())
    }
}

fn entity(name: &str) -> Option<String> {
    Some(
        match name {
            "amp" => "&",
            "lt" => "<",
            "gt" => ">",
            "quot" => "\"",
            "apos" => "'",
            _ => {
                let digits = name.strip_prefix('#')?;
                let code = match digits.strip_prefix(['x', 'X']) {
                    Some(hex) => u32::from_str_radix(hex, 16).ok()?,
                    None => digits.parse().ok()?,
                };
                return char::from_u32(code).map(String::from);
            }
        }
        .to_string(),
    )
}

/// The rows named `row_tag`, each with the text of its descendants. Tally's
/// numeric references to control characters (`&#4;`) are not legal XML, so
/// they are marked first exactly as the other parsers in this crate do.
pub(super) fn rows(xml: &str, row_tag: &str) -> Result<Vec<Row>, &'static str> {
    let xml = bridge_tally_protocol::mark_forbidden_numeric_references(xml);
    let mut reader = quick_xml::Reader::from_str(&xml);
    let mut path: Vec<String> = Vec::new();
    let mut row_depth: Option<usize> = None;
    let mut list_depth: Option<usize> = None;
    let mut sub = Row::default();
    let mut current = Row::default();
    let mut out = Vec::new();
    let mut text = String::new();
    let mut status: Option<String> = None;
    let mut collection = false;
    loop {
        match reader.read_event().map_err(|_| "invoice_read_malformed")? {
            Event::Start(start) => {
                let name = String::from_utf8_lossy(start.name().as_ref()).into_owned();
                text.clear();
                collection |= name == "COLLECTION";
                // Only a direct child of the COLLECTION is a row: the answer's
                // CMPINFO block carries counters named like the rows
                // (`<VOUCHERTYPE>0</VOUCHERTYPE>`), which are not rows.
                if row_depth.is_none()
                    && name == row_tag
                    && path.last().map(String::as_str) == Some("COLLECTION")
                {
                    row_depth = Some(path.len());
                    current = Row::default();
                    for attribute in start.attributes().with_checks(true) {
                        let attribute = attribute.map_err(|_| "invoice_read_malformed")?;
                        let key = String::from_utf8_lossy(attribute.key.as_ref()).into_owned();
                        let value = attribute
                            .decoded_and_normalized_value(
                                quick_xml::XmlVersion::Implicit1_0,
                                reader.decoder(),
                            )
                            .map_err(|_| "invoice_read_malformed")?;
                        current.attributes.insert(key, value.into_owned());
                    }
                }
                if let Some(depth) = row_depth {
                    if list_depth.is_none() && path.len() == depth + 1 && name.ends_with(".LIST") {
                        list_depth = Some(path.len());
                        sub = Row::default();
                    }
                }
                path.push(name);
            }
            Event::Text(chunk) => {
                text.push_str(&chunk.decode().map_err(|_| "invoice_read_malformed")?);
            }
            Event::GeneralRef(reference) => {
                let name = reference.decode().map_err(|_| "invoice_read_malformed")?;
                text.push_str(&entity(&name).ok_or("invoice_read_malformed")?);
            }
            Event::End(_) => {
                let Some(name) = path.pop() else {
                    return Err("invoice_read_malformed");
                };
                if name == "STATUS"
                    && path.as_slice() == ["ENVELOPE", "HEADER"]
                    && status.replace(text.trim().to_string()).is_some()
                {
                    return Err("invoice_read_status_repeated");
                }
                if let Some(depth) = row_depth {
                    if path.len() == depth && name == row_tag {
                        out.push(std::mem::take(&mut current));
                        row_depth = None;
                    } else if list_depth == Some(path.len()) && name.ends_with(".LIST") {
                        current.lists.push((name, std::mem::take(&mut sub)));
                        list_depth = None;
                    } else if path.len() > depth {
                        if let Some(list) = list_depth {
                            let inner = path[list + 1..]
                                .iter()
                                .cloned()
                                .chain(std::iter::once(name.clone()))
                                .collect::<Vec<_>>()
                                .join("/");
                            let value = text.trim();
                            if !value.is_empty() {
                                sub.fields.push((inner, value.to_string()));
                            }
                        }
                        // A leaf: record its text under the path below the row.
                        let relative = path[depth + 1..]
                            .iter()
                            .cloned()
                            .chain(std::iter::once(name))
                            .collect::<Vec<_>>()
                            .join("/");
                        let value = text.trim();
                        if !value.is_empty() {
                            current.fields.push((relative, value.to_string()));
                        }
                    }
                }
                text.clear();
            }
            Event::Empty(empty) => {
                collection |= empty.name().as_ref() == b"COLLECTION";
                // A row written as an empty element is still a row, with no
                // fields and no attributes: its reader refuses it or reports
                // every field as different, and never counts it as "no rows".
                if row_depth.is_none()
                    && empty.name().as_ref() == row_tag.as_bytes()
                    && path.last().map(String::as_str) == Some("COLLECTION")
                {
                    out.push(Row::default());
                }
                text.clear();
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if !path.is_empty() {
        return Err("invoice_read_malformed");
    }
    // An empty answer must be Tally's success, never its error text read as
    // "no rows".
    if status.as_deref() != Some("1") {
        return Err("invoice_read_status_not_success");
    }
    // "No rows" is an answer only from a collection that came back: a success
    // envelope without one is some other shape, never an empty collection.
    if !collection {
        return Err("invoice_read_collection_absent");
    }
    Ok(out)
}

/// A voucher type as the book defines it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct VoucherTypeRow {
    pub(super) name: String,
    pub(super) guid: String,
    /// Tally's reserved name: set on a predefined type, empty on a user's.
    pub(super) reserved_name: String,
    pub(super) parent: String,
    /// Each number series of the type, with its numbering method and its
    /// "prevent duplicates" flag (`None` when the answer gave none).
    pub(super) series: Vec<(String, String, Option<String>)>,
}

/// What a named voucher type resolves to for an invoice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ResolvedVoucherType {
    pub(super) guid: String,
    /// The reserved class its parent chain reaches ("Sales").
    pub(super) class: String,
}

pub(in crate::agent) fn render_voucher_types_request(company: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Invoice Voucher Types</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME=\"Bridge Invoice Voucher Types\" ISMODIFY=\"No\"><TYPE>VoucherType</TYPE><NATIVEMETHOD>*</NATIVEMETHOD></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        xml_escape(company)
    )
}

pub(super) fn parse_voucher_types(xml: &str) -> Result<Vec<VoucherTypeRow>, &'static str> {
    let mut types = Vec::new();
    for row in rows(xml, "VOUCHERTYPE")? {
        let name = row
            .attributes
            .get("NAME")
            .cloned()
            .ok_or("invoice_voucher_type_row_unnamed")?;
        let guid = row
            .one("GUID")?
            .ok_or("invoice_voucher_type_row_without_guid")?
            .to_string();
        let parent = row.one("PARENT")?.unwrap_or_default().to_string();
        let reserved_name = row
            .attributes
            .get("RESERVEDNAME")
            .cloned()
            .unwrap_or_default();
        let methods = row
            .all("VOUCHERNUMBERSERIES.LIST/NUMBERINGMETHOD")
            .collect::<Vec<_>>();
        let names = row.all("VOUCHERNUMBERSERIES.LIST/NAME").collect::<Vec<_>>();
        // The flag is aligned to its series only when every series has one;
        // none at all reads as unknown, and some but not all is unreadable.
        let guards = row
            .all("VOUCHERNUMBERSERIES.LIST/PREVENTDUPLICATES")
            .map(|flag| Some(flag.to_string()))
            .collect::<Vec<_>>();
        let guards = match guards.len() {
            0 => vec![None; names.len()],
            n if n == names.len() => guards,
            _ => return Err("invoice_voucher_type_series_unreadable"),
        };
        if methods.len() != names.len() {
            return Err("invoice_voucher_type_series_unreadable");
        }
        let series = names
            .into_iter()
            .zip(methods)
            .zip(guards)
            .map(|((name, method), guard)| (name.to_string(), method.to_string(), guard))
            .collect();
        types.push(VoucherTypeRow {
            name,
            guid,
            reserved_name,
            parent,
            series,
        });
    }
    Ok(types)
}

/// The type the caller named, its class by the parent chain, and whether a
/// supplied voucher number survives on it. Every doubt is a refusal.
pub(super) fn resolve_voucher_type(
    types: &[VoucherTypeRow],
    name: &str,
    class: &str,
) -> Result<ResolvedVoucherType, &'static str> {
    let named = types
        .iter()
        .filter(|row| row.name == name)
        .collect::<Vec<_>>();
    let target = match named.as_slice() {
        [] => return Err("invoice_voucher_type_not_found"),
        [one] => *one,
        _ => return Err("invoice_voucher_type_name_ambiguous"),
    };
    // Walk to a predefined type; a type whose parent is itself is the root.
    let mut at = target;
    let mut hops = 0;
    let reached = loop {
        if !at.reserved_name.is_empty() {
            break at.reserved_name.as_str();
        }
        hops += 1;
        if hops > types.len() {
            return Err("invoice_voucher_type_chain_cycles");
        }
        let parents = types
            .iter()
            .filter(|row| row.name == at.parent)
            .collect::<Vec<_>>();
        at = match parents.as_slice() {
            [one] => one,
            [] => return Err("invoice_voucher_type_parent_missing"),
            _ => return Err("invoice_voucher_type_parent_ambiguous"),
        };
    };
    if reached != class {
        return Err("invoice_voucher_type_wrong_class");
    }
    // The GUID goes into the read-back's formula: checked here, at the build,
    // so a post never finds it unusable after the voucher is in the book.
    if !guid_literal_safe(&target.guid) {
        return Err("invoice_voucher_type_guid_unusable");
    }
    // The series-level method decides what an import's supplied number does,
    // not the top-level field (measured, §9.16: one book's predefined Sales
    // type reads Automatic (Manual Override) on top and Manual in its series;
    // the type keyed for the rehearsal reads None on top and Manual in its
    // series).
    // Its "prevent duplicates" flag is required as a further guard, not
    // relied on: §9.8 measured it only for a failed Journal Alter, and an
    // import that creates a Sales voucher under a used number is not measured
    // (implementation guide §3.4). For a company's first invoice the post's
    // readback (`invoice_number_not_unique`) is what catches a duplicate.
    match target.series.as_slice() {
        [(_, method, Some(guard))] if method == "Manual" && guard == "Yes" => {}
        [(_, method, _)] if method == "Manual" => {
            return Err("invoice_voucher_type_duplicates_allowed")
        }
        [] => return Err("invoice_voucher_type_series_missing"),
        [_] => return Err("invoice_voucher_type_numbering_not_manual"),
        _ => return Err("invoice_voucher_type_several_series"),
    }
    Ok(ResolvedVoucherType {
        guid: target.guid.clone(),
        class: reached.to_string(),
    })
}

/// A GUID is put inside a TDL string literal. Its alphabet is closed (hex and
/// hyphen), unlike a ledger's name, so nothing a ledger is called can alter the
/// formula.
pub(super) fn guid_literal_safe(guid: &str) -> bool {
    !guid.is_empty() && guid.len() <= 64 && guid.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

/// The vouchers of the financial year that carry this number, whatever their
/// type, each with the measured per-row class compute (`$$IsSales` of the row's
/// voucher type, bridge#625). GST requires an invoice number to be unique within
/// the year across every series, and a book can hold several Sales types, so the
/// class is decided here, client-side, from that compute. Only measured
/// primitives reach Tally: a date formula on literal dates (SVFROMDATE and
/// SVTODATE do not limit a collection), a voucher-number equality (the form the
/// hand-keyed invoice read used) and the class compute.
pub(in crate::agent) fn render_invoice_number_request(
    company: &str,
    number: &str,
    (from, to): (&str, &str),
) -> Option<String> {
    if !super::invoice_number_safe(number)
        || ![from, to]
            .iter()
            .all(|date| date.len() == 8 && date.bytes().all(|b| b.is_ascii_digit()))
    {
        return None;
    }
    Some(format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Invoice Number</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{from}</SVFROMDATE><SVTODATE TYPE=\"Date\">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeInvoiceNumber\">$Date &gt;= $$Date:\"{from}\" AND $Date &lt;= $$Date:\"{to}\" AND $VoucherNumber = \"{number}\"</SYSTEM><COLLECTION NAME=\"Bridge Invoice Number\" ISMODIFY=\"No\"><TYPE>Voucher</TYPE><FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED</FETCH><COMPUTE>{SALES_CLASS_TAG}:$$IsSales:$VoucherTypeName</COMPUTE><FILTERS>BridgeInvoiceNumber</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        company = xml_escape(company),
    ))
}

/// The element the Sales class compute answers in (the name the voucher reads
/// already use).
const SALES_CLASS_TAG: &str = "BRIDGEVCHISSALES";

/// How many of the returned vouchers are Sales-class. A row whose class
/// element is missing or is neither Yes nor No is an error: an unknown function
/// omits its element, so a gap is never a No.
pub(super) fn count_sales_vouchers(xml: &str) -> Result<usize, &'static str> {
    let mut sales = 0;
    for row in rows(xml, "VOUCHER")? {
        match row.one(SALES_CLASS_TAG)? {
            Some("Yes") => sales += 1,
            Some("No") => {}
            _ => return Err("invoice_number_class_unread"),
        }
    }
    Ok(sales)
}

/// Whether the number read for a known invoice (the control) found it: a row
/// carrying exactly that number and that date, read as Sales class. Any other
/// row is not the control, and a row whose class cannot be read is an error,
/// as for the read it controls.
pub(super) fn control_row_found(xml: &str, number: &str, date: &str) -> Result<bool, &'static str> {
    let mut found = false;
    for row in rows(xml, "VOUCHER")? {
        let class = match row.one(SALES_CLASS_TAG)? {
            Some("Yes") => true,
            Some("No") => false,
            _ => return Err("invoice_number_class_unread"),
        };
        found |=
            class && row.one("VOUCHERNUMBER")? == Some(number) && row.one("DATE")? == Some(date);
    }
    Ok(found)
}

/// What a posted invoice reads back as: the voucher-level fields and every
/// ledger entry with its bill allocations. The fetch is the entry wildcard the
/// agent's voucher reads use (it returns the legs of an invoice-view voucher
/// as ALLLEDGERENTRIES.LIST, measured on a committed Sales capture) plus the GST
/// header fields. Measured together on the two invoices the rehearsal posted
/// (7 Oct 2026, §9.16): every field below came back.
pub(in crate::agent) fn render_invoice_readback_request(
    company: &str,
    type_guid: &str,
    number: &str,
    (from, to): (&str, &str),
) -> Option<String> {
    if !guid_literal_safe(type_guid)
        || !super::invoice_number_safe(number)
        || ![from, to]
            .iter()
            .all(|date| date.len() == 8 && date.bytes().all(|b| b.is_ascii_digit()))
    {
        return None;
    }
    Some(format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Invoice Readback</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{from}</SVFROMDATE><SVTODATE TYPE=\"Date\">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeInvoiceReadback\">$Date &gt;= $$Date:\"{from}\" AND $Date &lt;= $$Date:\"{to}\" AND $VoucherNumber = \"{number}\" AND $GUID:VoucherType:$VoucherTypeName = \"{type_guid}\"</SYSTEM><COLLECTION NAME=\"Bridge Invoice Readback\" ISMODIFY=\"No\"><TYPE>Voucher</TYPE><FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, REFERENCE, REFERENCEDATE, PARTYLEDGERNAME, PARTYGSTIN, STATENAME, PLACEOFSUPPLY, GSTREGISTRATIONTYPE, ISINVOICE, ISCANCELLED, ISOPTIONAL, GUID, ALTERID, ALLLEDGERENTRIES.*</FETCH><FILTERS>BridgeInvoiceReadback</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        company = xml_escape(company),
    ))
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ReadLeg {
    pub(super) ledger: String,
    pub(super) amount: String,
    pub(super) deemed_positive: Option<String>,
    /// Bill allocations of the leg: name, type, amount.
    pub(super) allocations: Vec<(Option<String>, Option<String>, String)>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ReadInvoice {
    /// The voucher-level scalar fields, by tag.
    pub(super) fields: BTreeMap<String, String>,
    pub(super) legs: Vec<ReadLeg>,
}

/// How many vouchers a read-back request returned: none, the one, or several.
/// The caller decides what an absent or a repeated voucher means.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Readback {
    Absent,
    One(ReadInvoice),
    Several,
}

pub(super) fn parse_invoice_readback(xml: &str) -> Result<Readback, &'static str> {
    let vouchers = rows(xml, "VOUCHER")?;
    let voucher = match vouchers.as_slice() {
        [] => return Ok(Readback::Absent),
        [voucher] => voucher,
        _ => return Ok(Readback::Several),
    };
    let mut fields = BTreeMap::new();
    for key in [
        "DATE",
        "VOUCHERNUMBER",
        "VOUCHERTYPENAME",
        "REFERENCE",
        "REFERENCEDATE",
        "PARTYLEDGERNAME",
        "PARTYGSTIN",
        "STATENAME",
        "PLACEOFSUPPLY",
        "GSTREGISTRATIONTYPE",
        "ISINVOICE",
        "ISCANCELLED",
        "ISOPTIONAL",
        "GUID",
        "ALTERID",
    ] {
        if let Some(value) = voucher.one(key)? {
            fields.insert(key.to_string(), value.to_string());
        }
    }
    let mut legs = Vec::new();
    for (tag, entry) in &voucher.lists {
        if tag != "ALLLEDGERENTRIES.LIST" {
            continue;
        }
        let names = entry
            .all("BILLALLOCATIONS.LIST/NAME")
            .map(str::to_string)
            .collect::<Vec<_>>();
        let kinds = entry
            .all("BILLALLOCATIONS.LIST/BILLTYPE")
            .map(str::to_string)
            .collect::<Vec<_>>();
        let amounts = entry
            .all("BILLALLOCATIONS.LIST/AMOUNT")
            .map(str::to_string)
            .collect::<Vec<_>>();
        // One allocation per index of the longest of the three lists: a name or a
        // type with no amount beside it is a (blank-amount) allocation, which
        // never equals what was written, instead of being dropped from view.
        let count = names.len().max(kinds.len()).max(amounts.len());
        let allocations = (0..count)
            .map(|index| {
                (
                    names.get(index).cloned(),
                    kinds.get(index).cloned(),
                    amounts.get(index).cloned().unwrap_or_default(),
                )
            })
            .collect();
        legs.push(ReadLeg {
            ledger: entry
                .one("LEDGERNAME")?
                .ok_or("invoice_readback_leg_unnamed")?
                .to_string(),
            amount: entry
                .one("AMOUNT")?
                .ok_or("invoice_readback_leg_without_amount")?
                .to_string(),
            deemed_positive: entry.one("ISDEEMEDPOSITIVE")?.map(str::to_string),
            allocations,
        });
    }
    Ok(Readback::One(ReadInvoice { fields, legs }))
}

/// The number of vouchers an answer holds (the status check included).
#[cfg(test)]
pub(super) fn count_vouchers(xml: &str) -> Result<usize, &'static str> {
    Ok(rows(xml, "VOUCHER")?.len())
}

/// The company's own GST registrations: a `TaxUnit` collection with no field
/// list. The shape was read once, on a synthetic lab company, from the same
/// collection typed by hand and sent as UTF-8 (two units: the Default Tax Unit
/// and one GST registration whose dated rows carry the state, the
/// registration type and whether it is inactive). This request, sent in UTF-16,
/// was answered on 9 Oct 2026 with the same two rows (the `pilot-lab` fixtures).
pub(in crate::agent) fn render_company_registration_request(company: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Invoice Tax Units</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME=\"Bridge Invoice Tax Units\" ISMODIFY=\"No\"><TYPE>TaxUnit</TYPE><NATIVEMETHOD>*</NATIVEMETHOD></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        xml_escape(company)
    )
}

/// The registration an invoice is issued under: the one GST unit in force on
/// the invoice date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CompanyRegistration {
    pub(super) gstin: String,
    /// The state of the registration's dated row in force: the supplier's
    /// state for the place-of-supply rule.
    pub(super) state: String,
}

/// Why a tax-unit answer gives no registration: the book does not hold one
/// this build can issue under (a refusal), or the answer itself cannot be
/// read (a failed read).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RegistrationOutcome {
    Refused(&'static str),
    Failed(&'static str),
}

/// The company's GST registration in force on `as_of` (`YYYYMMDD`), from the
/// tax-unit answer. A GST unit is a row whose `TAXTYPE` attribute, `USEDFOR`,
/// `GSTREGNUMBER` or dated `GSTREGISTRATIONDETAILS.LIST` says so, and all of
/// them must; its dated row in force is the latest one on or before `as_of`
/// (document order is not assumed to be date order). Exactly one unit may be
/// in force, its row not inactive and Regular, with a valid GSTIN of the
/// row's state, equal to the unit's `TAXREGISTRATION`. Nothing is defaulted: a missing field refuses, and the
/// unit-level type, state and dates are never read (empty on a registered
/// unit). A row whose GUID is not the company's refuses the whole answer, so a
/// unit is never dropped from the count; a row with no GUID, and an answer with
/// no unit at all, not even the default one, are failed reads.
pub(super) fn parse_company_registration(
    xml: &str,
    company_guid: &str,
    as_of: &str,
) -> Result<CompanyRegistration, RegistrationOutcome> {
    use RegistrationOutcome::{Failed, Refused};
    let units = rows(xml, "TAXUNIT").map_err(Failed)?;
    if units.is_empty() {
        return Err(Failed("invoice_company_registration_unread"));
    }
    let prefix = format!("{}-", company_guid.to_ascii_lowercase());
    let mut found = 0;
    let mut inactive = false;
    let mut in_force = Vec::new();
    for unit in &units {
        let guid = unit
            .one("GUID")
            .map_err(Failed)?
            .ok_or(Failed("invoice_tax_unit_without_guid"))?;
        if !guid.to_ascii_lowercase().starts_with(&prefix) {
            return Err(Refused("invoice_company_registration_unbound"));
        }
        let gstin = unit.one("GSTREGNUMBER").map_err(Failed)?;
        let dated = unit
            .lists
            .iter()
            .filter(|(name, row)| name == "GSTREGISTRATIONDETAILS.LIST" && !row.fields.is_empty())
            .map(|(_, row)| row)
            .collect::<Vec<_>>();
        let markers = [
            unit.attributes.get("TAXTYPE").map(String::as_str) == Some("GST"),
            unit.one("USEDFOR").map_err(Failed)? == Some("GST"),
            gstin.is_some(),
            !dated.is_empty(),
        ];
        if !markers.contains(&true) {
            continue;
        }
        found += 1;
        if markers.contains(&false) {
            return Err(Refused("invoice_company_registration_inconsistent"));
        }
        let mut rows_by_date = Vec::new();
        for row in dated {
            let fields = (
                row.one("FROMDATE").map_err(Failed)?,
                row.one("STATE").map_err(Failed)?,
                row.one("REGISTRATIONTYPE").map_err(Failed)?,
                row.one("ISINACTIVE").map_err(Failed)?,
            );
            let (Some(from), Some(state), Some(kind), Some(flag @ ("Yes" | "No"))) = fields else {
                return Err(Refused("invoice_company_registration_incomplete"));
            };
            if bridge_tally_core::TallyDate::parse(from).is_err() || !super::is_state_name(state) {
                return Err(Refused("invoice_company_registration_incomplete"));
            }
            rows_by_date.push((from, state, kind, flag == "Yes"));
        }
        rows_by_date.sort_unstable();
        if rows_by_date.windows(2).any(|pair| pair[0].0 == pair[1].0) {
            return Err(Refused("invoice_company_registration_inconsistent"));
        }
        match rows_by_date.iter().rev().find(|row| row.0 <= as_of) {
            None => {}
            Some(&(_, _, _, true)) => inactive = true,
            Some(&(_, state, kind, false)) => in_force.push((unit, gstin, state, kind)),
        }
    }
    let (unit, gstin, state, kind) = match in_force.as_slice() {
        [one] => *one,
        [] if found == 0 => return Err(Refused("invoice_company_registration_absent")),
        [] if inactive => return Err(Refused("invoice_company_registration_inactive")),
        [] => return Err(Refused("invoice_company_registration_not_yet_in_force")),
        _ => return Err(Refused("invoice_company_registration_ambiguous")),
    };
    if kind != "Regular" {
        return Err(Refused("invoice_company_registration_not_regular"));
    }
    let gstin = gstin.unwrap_or_default();
    // A GSTIN is held once per unit and is not dated, so after a change an
    // earlier invoice would be issued under the new one.
    let consistent = unit.one("GSTOLDREGNUMBER").map_err(Failed)?.is_none()
        && unit
            .attributes
            .get("TAXREGISTRATION")
            .is_some_and(|attribute| attribute == gstin)
        && super::gstin_valid(gstin)
        && super::gstin_state(gstin) == Some(state);
    if !consistent {
        return Err(Refused("invoice_company_registration_inconsistent"));
    }
    Ok(CompanyRegistration {
        gstin: gstin.to_string(),
        state: state.to_string(),
    })
}

// ---- The ledger rates read (#1342, W7, 10 Oct 2026) ----

/// The ledger listing the tax check reads: the listing the build already reads
/// for each ledger's group, duty head and GSTIN, with the four fields that
/// hold a ledger's GST rate and rounding (`GSTDETAILS.LIST`,
/// `RATEOFTAXCALCULATION`, `ROUNDINGMETHOD`, `ROUNDINGLIMIT`) added and
/// nothing else changed. Sent in UTF-16 on a licensed TallyPrime 7.1 Silver
/// synthetic company on 10 Oct 2026, it was answered with the fields (the
/// `pilot-lab` fixtures). `window` is the financial year's start and the
/// invoice date, as the request that was measured carried them (a ledger's
/// rate rows came back whole; whether the window changes them was not tried).
/// `None` when a date is not eight digits.
pub(in crate::agent) fn render_ledger_rates_request(
    company: &str,
    window: (&str, &str),
) -> Option<String> {
    render_ledger_rates(company, window, None)
}

/// [`render_ledger_rates_request`] restricted to the ledgers under one
/// [`ParentPart`]'s parents, through the same `SYSTEM` formula and `FILTERS`
/// element the compliance listing's parts carry (#679, #1331). The fetch list
/// is the unfiltered request's, unchanged.
pub(in crate::agent) fn render_ledger_rates_request_for_parents(
    company: &str,
    window: (&str, &str),
    part: &bridge_tally_protocol::parent_partition::ParentPart,
) -> Option<String> {
    render_ledger_rates(company, window, Some(part))
}

fn render_ledger_rates(
    company: &str,
    window: (&str, &str),
    part: Option<&bridge_tally_protocol::parent_partition::ParentPart>,
) -> Option<String> {
    let digits = |date: &str| date.len() == 8 && date.bytes().all(|byte| byte.is_ascii_digit());
    if !digits(window.0) || !digits(window.1) {
        return None;
    }
    let (formula, filters) = bridge_tally_protocol::native_outstandings::parent_filter_parts(part);
    Some(format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>List of Ledgers</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{from}</SVFROMDATE><SVTODATE TYPE=\"Date\">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE>{formula}<COLLECTION NAME=\"List of Ledgers\" ISMODIFY=\"Yes\"><FETCH>NAME, GUID, REMOTEID, MASTERID, ALTERID, PARENT, PARTYGSTIN, INCOMETAXNUMBER, NAMEONPAN, LEDPINCODE, LEDGSTPINCODE, MSMEREGNUMBER, LEDUDYAMREGNUMBER, BANKACCHOLDERNAME, BANKDETAILS, IFSCODE, EMAIL, LEDGERPHONE, STATENAME, LEDADDRESS.LIST, TAXTYPE, GSTDUTYHEAD, OPENINGBALANCE, LEDGSTREGDETAILS.LIST, GSTDETAILS.LIST, RATEOFTAXCALCULATION, ROUNDINGMETHOD, ROUNDINGLIMIT</FETCH><COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>{filters}</COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        company = xml_escape(company),
        from = window.0,
        to = window.1,
    ))
}

/// One element of an answer with its attributes, text and children, kept
/// whole so a field that is present but empty stays different from one that is
/// absent (`rows` drops empty leaves; the rate rows depend on the difference).
#[derive(Debug, Default)]
struct Node {
    name: String,
    attributes: BTreeMap<String, String>,
    text: String,
    children: Vec<Node>,
}

impl Node {
    /// The text of the one child named `name`: `Ok(None)` when absent, an
    /// error when repeated. Whitespace and the control characters Tally
    /// writes before some values (`&#4; Any`) are not part of the value.
    fn text_of(&self, name: &str) -> Result<Option<String>, &'static str> {
        let mut found = self.children.iter().filter(|child| child.name == name);
        let first = found.next().map(|child| clean(&child.text));
        if found.next().is_some() {
            return Err("invoice_read_field_repeated");
        }
        Ok(first)
    }

    fn all<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Node> + 'a {
        self.children.iter().filter(move |child| child.name == name)
    }
}

/// A value as Tally means it: the numeric reference to a control character it
/// writes before some values (`&#4; Any`), which `mark_forbidden_numeric_references`
/// turns into the replacement character and `#4;`, and the whitespace around
/// it, are not part of the value.
fn clean(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('\u{fffd}') {
        out.push_str(&rest[..at]);
        let after = &rest[at + '\u{fffd}'.len_utf8()..];
        let digits = after
            .strip_prefix('#')
            .map(|tail| tail.bytes().take_while(u8::is_ascii_digit).count());
        match digits {
            Some(count) if count > 0 && after[1 + count..].starts_with(';') => {
                rest = &after[1 + count + 1..];
            }
            _ => {
                out.push('\u{fffd}');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out.trim_matches(|c: char| c.is_whitespace() || c.is_control())
        .to_string()
}

/// The answer as a tree, under the same envelope rules as `rows`: a success
/// status, a collection that came back, numeric references to control
/// characters marked first.
fn parse_tree(xml: &str) -> Result<Node, &'static str> {
    let xml = bridge_tally_protocol::mark_forbidden_numeric_references(xml);
    let mut reader = quick_xml::Reader::from_str(&xml);
    let mut stack: Vec<Node> = vec![Node::default()];
    loop {
        match reader.read_event().map_err(|_| "invoice_read_malformed")? {
            Event::Start(start) => {
                let mut node = Node {
                    name: String::from_utf8_lossy(start.name().as_ref()).into_owned(),
                    ..Node::default()
                };
                for attribute in start.attributes().with_checks(true) {
                    let attribute = attribute.map_err(|_| "invoice_read_malformed")?;
                    let value = attribute
                        .decoded_and_normalized_value(
                            quick_xml::XmlVersion::Implicit1_0,
                            reader.decoder(),
                        )
                        .map_err(|_| "invoice_read_malformed")?;
                    node.attributes.insert(
                        String::from_utf8_lossy(attribute.key.as_ref()).into_owned(),
                        value.into_owned(),
                    );
                }
                stack.push(node);
            }
            Event::Empty(empty) => {
                let node = Node {
                    name: String::from_utf8_lossy(empty.name().as_ref()).into_owned(),
                    ..Node::default()
                };
                stack
                    .last_mut()
                    .ok_or("invoice_read_malformed")?
                    .children
                    .push(node);
            }
            Event::Text(chunk) => {
                let text = chunk.decode().map_err(|_| "invoice_read_malformed")?;
                stack
                    .last_mut()
                    .ok_or("invoice_read_malformed")?
                    .text
                    .push_str(&text);
            }
            Event::GeneralRef(reference) => {
                let name = reference.decode().map_err(|_| "invoice_read_malformed")?;
                let text = entity(&name).ok_or("invoice_read_malformed")?;
                stack
                    .last_mut()
                    .ok_or("invoice_read_malformed")?
                    .text
                    .push_str(&text);
            }
            Event::End(_) => {
                let node = stack.pop().ok_or("invoice_read_malformed")?;
                stack
                    .last_mut()
                    .ok_or("invoice_read_malformed")?
                    .children
                    .push(node);
            }
            Event::Eof => break,
            _ => {}
        }
    }
    let [root] = <[Node; 1]>::try_from(stack).map_err(|_| "invoice_read_malformed")?;
    let [envelope] = <[Node; 1]>::try_from(root.children).map_err(|_| "invoice_read_malformed")?;
    if envelope.name != "ENVELOPE" {
        return Err("invoice_read_malformed");
    }
    let status = envelope
        .all("HEADER")
        .next()
        .and_then(|header| header.text_of("STATUS").ok().flatten());
    if status.as_deref() != Some("1") {
        return Err("invoice_read_status_not_success");
    }
    Ok(envelope)
}

/// A ledger's rate for one head in one state-wise row: the head's name, how
/// Tally values it, and the rate when the row carries one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct HeadRate {
    pub(super) head: String,
    pub(super) valuation: Option<String>,
    pub(super) rate: Option<String>,
}

/// One state-wise block of a dated GST row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct StateRates {
    pub(super) state: Option<String>,
    pub(super) heads: Vec<HeadRate>,
}

/// One dated `GSTDETAILS.LIST` row of a ledger.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct GstRow {
    pub(super) applicable_from: Option<String>,
    pub(super) taxability: Option<String>,
    pub(super) source: Option<String>,
    pub(super) states: Vec<StateRates>,
    /// The row carries an element or a value the lab's rows did not (a child
    /// of the row, of a state-wise block or of a head's rate that is not in
    /// the measured set, or a slab-rate list that is not empty).
    pub(super) unmeasured: bool,
}

/// What the rate listing says of one ledger. Each field is the text Tally
/// returned, or `None` when the element was absent: an absent field, an empty
/// one and a zero are three different answers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::agent::agent_import) struct LedgerRateRow {
    pub(super) gst_rows: Vec<GstRow>,
    pub(super) rate_of_tax_calculation: Option<String>,
    pub(super) rounding_method: Option<String>,
    pub(super) rounding_limit: Option<String>,
}

/// The elements the lab's GST rows carried (10 Oct 2026, fourteen ledgers),
/// at each level of a row; anything else marks the row unmeasured.
const GST_ROW_CHILDREN: &[&str] = &[
    "APPLICABLEFROM",
    "GSTINELIGIBLEITC",
    "SRCOFGSTDETAILS",
    "STATEWISEDETAILS.LIST",
    "TAXABILITY",
];
const STATE_BLOCK_CHILDREN: &[&str] = &["GSTSLABRATES.LIST", "RATEDETAILS.LIST", "STATENAME"];
const HEAD_RATE_CHILDREN: &[&str] = &["GSTRATE", "GSTRATEDUTYHEAD", "GSTRATEVALUATIONTYPE"];

/// The rate listing by ledger name, for the ledgers an invoice names. Every
/// row of the answer must carry the verified company's GUID (the
/// `BRIDGECOMPANYGUID` compute); the rows of the named ledgers are read in
/// full, and a name two of them claim refuses the answer. A row of any other
/// ledger is not read, so one unrelated ledger cannot fail an invoice.
pub(super) fn parse_ledger_rates(
    xml: &str,
    company_guid: &str,
    wanted: &[&str],
) -> Result<BTreeMap<String, LedgerRateRow>, &'static str> {
    let envelope = parse_tree(xml)?;
    let body = envelope
        .all("BODY")
        .next()
        .and_then(|body| body.all("DATA").next())
        .ok_or("invoice_read_collection_absent")?;
    let collection = body
        .all("COLLECTION")
        .next()
        .ok_or("invoice_read_collection_absent")?;
    let mut out = BTreeMap::new();
    for ledger in collection.all("LEDGER") {
        let bound = ledger.text_of("BRIDGECOMPANYGUID");
        if !bound.is_ok_and(|guid| guid.is_some_and(|guid| guid.eq_ignore_ascii_case(company_guid)))
        {
            return Err("invoice_ledger_rates_company_mismatch");
        }
        let Some(name) = ledger.attributes.get("NAME").cloned() else {
            continue;
        };
        if !wanted.contains(&name.as_str()) {
            continue;
        }
        let mut gst_rows = Vec::new();
        for details in ledger.all("GSTDETAILS.LIST") {
            let mut unmeasured = details
                .children
                .iter()
                .any(|child| !GST_ROW_CHILDREN.contains(&child.name.as_str()));
            let mut states = Vec::new();
            for state in details.all("STATEWISEDETAILS.LIST") {
                unmeasured |= state
                    .children
                    .iter()
                    .any(|child| !STATE_BLOCK_CHILDREN.contains(&child.name.as_str()));
                unmeasured |= state
                    .all("GSTSLABRATES.LIST")
                    .any(|slabs| !slabs.children.is_empty() || !clean(&slabs.text).is_empty());
                let mut heads = Vec::new();
                for detail in state.all("RATEDETAILS.LIST") {
                    unmeasured |= detail
                        .children
                        .iter()
                        .any(|child| !HEAD_RATE_CHILDREN.contains(&child.name.as_str()));
                    heads.push(HeadRate {
                        head: detail
                            .text_of("GSTRATEDUTYHEAD")?
                            .ok_or("invoice_ledger_rates_head_unnamed")?,
                        valuation: detail.text_of("GSTRATEVALUATIONTYPE")?,
                        rate: detail.text_of("GSTRATE")?,
                    });
                }
                states.push(StateRates {
                    state: state.text_of("STATENAME")?,
                    heads,
                });
            }
            gst_rows.push(GstRow {
                applicable_from: details.text_of("APPLICABLEFROM")?,
                taxability: details.text_of("TAXABILITY")?,
                source: details.text_of("SRCOFGSTDETAILS")?,
                states,
                unmeasured,
            });
        }
        let row = LedgerRateRow {
            gst_rows,
            rate_of_tax_calculation: ledger.text_of("RATEOFTAXCALCULATION")?,
            rounding_method: ledger.text_of("ROUNDINGMETHOD")?,
            rounding_limit: ledger.text_of("ROUNDINGLIMIT")?,
        };
        if out.insert(name, row).is_some() {
            return Err("invoice_ledger_rates_name_repeated");
        }
    }
    Ok(out)
}

#[cfg(test)]
#[path = "agent_import_invoice_wire_tests.rs"]
mod tests;
