//! The three small reads an invoice build adds, and how their answers are read.
//!
//! Each answer is parsed at the edge into a value that cannot be wrong later:
//! a voucher type with its series-level numbering, a ledger's bill-wise flag,
//! the company's state. Nothing here guesses: a missing element, a repeated
//! one, or a name two rows claim is an error, never a default.
//!
//! What each read was measured to return (TallyPrime 7.1 Silver, lab, 6 Oct
//! 2026, a client sample book): the voucher-type collection with no field
//! list returns every type in 0.1 s (26 types, about 12 KB each) with the
//! type's reserved name as an attribute, its PARENT, its GUID, a top-level
//! NUMBERINGMETHOD and, inside `VOUCHERNUMBERSERIES.LIST`, the series-level
//! one that decides what an import's supplied number does. A `Ledger` row
//! returns `ISBILLWISEON` as Yes or No. The company-state read was measured
//! the same day on the synthetic lab company (one row a loaded company, the
//! row chosen by GUID carried its STATENAME); an answer that does not carry a
//! known state name fails the build (`invoice_company_state_unreadable`).

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
    /// Each number series of the type, with its numbering method.
    pub(super) series: Vec<(String, String)>,
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
        if methods.len() != names.len() {
            return Err("invoice_voucher_type_series_unreadable");
        }
        let series = names
            .into_iter()
            .zip(methods)
            .map(|(name, method)| (name.to_string(), method.to_string()))
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
    match target.series.as_slice() {
        [(_, method)] if method == "Manual" => {}
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

/// The company's own state: a `Company` collection with a FETCH list of NAME,
/// GUID and STATENAME (measured 6 Oct 2026 on the lab: one row for each loaded
/// company, each with its GUID, and STATENAME on the rows that have a state).
/// A `Company` collection returns every loaded company whatever
/// SVCURRENTCOMPANY says, so the row is chosen by GUID, never by position.
pub(in crate::agent) fn render_company_state_request(company: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Invoice Company</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME=\"Bridge Invoice Company\" ISMODIFY=\"No\"><TYPE>Company</TYPE><FETCH>NAME, GUID, STATENAME</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>",
        xml_escape(company)
    )
}

pub(super) fn parse_company_state(xml: &str, company_guid: &str) -> Result<String, &'static str> {
    let mut found = None;
    for row in rows(xml, "COMPANY")? {
        let guid = row.one("GUID")?.ok_or("invoice_company_row_without_guid")?;
        if guid.eq_ignore_ascii_case(company_guid) {
            if found.is_some() {
                return Err("invoice_company_row_repeated");
            }
            found = Some(row.one("STATENAME")?.map(str::to_string));
        }
    }
    match found {
        Some(Some(state)) if super::is_state_name(&state) => Ok(state),
        Some(_) => Err("invoice_company_state_unreadable"),
        None => Err("invoice_company_row_missing"),
    }
}

#[cfg(test)]
#[path = "agent_import_invoice_wire_tests.rs"]
mod tests;
