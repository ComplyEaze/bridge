//! Candidate stock-item fields added to the existing masters read.
//!
//! Evidence is the four captures in `tests/fixtures/agent/stock-create-lab/`;
//! see its `PROVENANCE.md`. It is PARTIAL: one book, one release, one sitting.

use crate::{
    gst_details::GstDetailsObservation,
    hsn_details::HsnDetailsObservation,
    native_masters::{
        name_attribute, parse_native_masters, path_is, read_text, upper, NativeMasterDetail,
        NativeMasterKind, NativeMastersError,
    },
    tolerant_xml::sanitize_invalid_numeric_references,
};
use quick_xml::{events::BytesStart, events::Event, Reader};

/// A scalar as Tally returned it, without inferring a value for absence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarObservation {
    Absent,
    Empty,
    Text(String),
}

/// One stock item with the fields needed to consider copying it. Only
/// [`parse_candidate_stock_items`] builds one, so every candidate a caller holds was read
/// from an answer the closed masters parser admitted; callers read it through the accessors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateStockItem {
    name: String,
    guid: String,
    master_id: u64,
    alter_id: u64,
    parent: Option<String>,
    base_units: ScalarObservation,
    gst_applicable: ScalarObservation,
    gst_type_of_supply: ScalarObservation,
    names: Vec<String>,
    hsn: HsnDetailsObservation,
    gst: GstDetailsObservation,
}

impl CandidateStockItem {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn guid(&self) -> &str {
        &self.guid
    }

    pub fn master_id(&self) -> u64 {
        self.master_id
    }

    pub fn alter_id(&self) -> u64 {
        self.alter_id
    }

    pub fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }

    pub fn base_units(&self) -> &ScalarObservation {
        &self.base_units
    }

    pub fn gst_applicable(&self) -> &ScalarObservation {
        &self.gst_applicable
    }

    pub fn gst_type_of_supply(&self) -> &ScalarObservation {
        &self.gst_type_of_supply
    }

    /// Every language name, in document order.
    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn hsn(&self) -> &HsnDetailsObservation {
        &self.hsn
    }

    pub fn gst(&self) -> &GstDetailsObservation {
        &self.gst
    }
}

struct ObservedRow {
    name: String,
    base_units: Option<ScalarObservation>,
    gst_applicable: Option<ScalarObservation>,
    gst_type_of_supply: Option<ScalarObservation>,
    names: Vec<String>,
}

/// Reads candidate fields after the closed masters parser admits the answer.
pub fn parse_candidate_stock_items(
    response: &str,
    company_guid: &str,
) -> Result<Vec<CandidateStockItem>, NativeMastersError> {
    let masters = parse_native_masters(NativeMasterKind::StockItems, response, company_guid)?;
    let sanitized = sanitize_invalid_numeric_references(response);
    let mut reader = Reader::from_str(&sanitized);
    reader.config_mut().trim_text(false);
    let mut path = Vec::<Vec<u8>>::new();
    let mut observed = Vec::new();
    loop {
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(element)
                if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"])
                    && upper(element.name()) == b"STOCKITEM" =>
            {
                observed.push(read_observed_row(&mut reader, &element)?);
            }
            Event::Start(element) => path.push(upper(element.name())),
            Event::End(_) => {
                path.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if masters.rows.len() != observed.len() {
        return Err(rows_differ());
    }
    let mut candidates = Vec::with_capacity(observed.len());
    for (row, extra) in masters.rows.into_iter().zip(observed) {
        if row.name != extra.name {
            return Err(rows_differ());
        }
        let NativeMasterDetail::StockItem { hsn, gst } = row.detail else {
            return Err(rows_differ());
        };
        candidates.push(CandidateStockItem {
            name: row.name,
            guid: row.guid,
            master_id: row.master_id,
            alter_id: row.alter_id,
            parent: row.parent,
            base_units: extra.base_units.unwrap_or(ScalarObservation::Absent),
            gst_applicable: extra.gst_applicable.unwrap_or(ScalarObservation::Absent),
            gst_type_of_supply: extra
                .gst_type_of_supply
                .unwrap_or(ScalarObservation::Absent),
            names: extra.names,
            hsn,
            gst,
        });
    }
    Ok(candidates)
}

fn read_observed_row(
    reader: &mut Reader<&[u8]>,
    element: &BytesStart<'_>,
) -> Result<ObservedRow, NativeMastersError> {
    let mut row = ObservedRow {
        name: name_attribute(element)?,
        base_units: None,
        gst_applicable: None,
        gst_type_of_supply: None,
        names: Vec::new(),
    };
    let mut path = Vec::<Vec<u8>>::new();
    loop {
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(element) => {
                let name = upper(element.name());
                if path.is_empty() && is_scalar(&name) {
                    let text = read_text(reader, element.name())?;
                    set_scalar(&mut row, &name, scalar(text))?;
                } else if path_is(&path, &[b"LANGUAGENAME.LIST", b"NAME.LIST"]) && name == b"NAME" {
                    let text = read_text(reader, element.name())?;
                    if xml_blank(&text) {
                        return Err(NativeMastersError::Malformed("masters_name_entry_empty"));
                    }
                    row.names.push(text);
                } else {
                    path.push(name);
                }
            }
            Event::Empty(element) => {
                let name = upper(element.name());
                if path.is_empty() && is_scalar(&name) {
                    set_scalar(&mut row, &name, ScalarObservation::Empty)?;
                } else if path_is(&path, &[b"LANGUAGENAME.LIST", b"NAME.LIST"]) && name == b"NAME" {
                    return Err(NativeMastersError::Malformed("masters_name_entry_empty"));
                }
            }
            Event::End(_) => {
                if path.pop().is_none() {
                    return Ok(row);
                }
            }
            Event::Eof => {
                return Err(NativeMastersError::Malformed("masters_row_unterminated"));
            }
            _ => {}
        }
    }
}

fn is_scalar(name: &[u8]) -> bool {
    matches!(name, b"BASEUNITS" | b"GSTAPPLICABLE" | b"GSTTYPEOFSUPPLY")
}

fn set_scalar(
    row: &mut ObservedRow,
    name: &[u8],
    value: ScalarObservation,
) -> Result<(), NativeMastersError> {
    let slot = match name {
        b"BASEUNITS" => &mut row.base_units,
        b"GSTAPPLICABLE" => &mut row.gst_applicable,
        _ => &mut row.gst_type_of_supply,
    };
    if slot.replace(value).is_some() {
        return Err(NativeMastersError::Malformed("masters_row_field_repeated"));
    }
    Ok(())
}

fn scalar(text: String) -> ScalarObservation {
    if xml_blank(&text) {
        ScalarObservation::Empty
    } else {
        ScalarObservation::Text(text)
    }
}

fn xml_blank(text: &str) -> bool {
    text.trim_matches([' ', '\t', '\r', '\n']).is_empty()
}

fn rows_differ() -> NativeMastersError {
    NativeMastersError::Malformed("masters_candidate_rows_differ")
}
