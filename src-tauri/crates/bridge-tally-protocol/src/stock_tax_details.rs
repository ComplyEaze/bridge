//! The closed reader of a stock item's `HSNDETAILS.LIST` and `GSTDETAILS.LIST`
//! inside a `masters` row (kind `stock_items`).
//!
//! Evidence: one synthetic book on one licensed `TallyPrime` 7.1 (section 12a.19 of
//! the protocol reference; PARTIAL). The lists sit directly under the row, in the
//! order `GSTDETAILS.LIST`, `HSNDETAILS.LIST`, `LANGUAGENAME.LIST`, one element
//! per dated entry; an item without entries carries one whitespace-only element
//! of each.
//!
//! This reader reads the two lists into the raw entries that
//! [`GstDetailsObservation::from_raw`] and [`HsnDetailsObservation::from_raw`]
//! validate. Which fault fails what is fixed here, once:
//!
//! - **The read is refused** (a typed [`NativeMastersError`]): Tally's `ERROR` or
//!   `LINEERROR` at any depth inside a list, including inside a scalar
//!   (`TallyReportedFailure`); markup nested in a scalar
//!   (`masters_scalar_not_text_only`); an invalid escape
//!   (`masters_xml_invalid_escape`); XML that is not well formed
//!   (`masters_xml_malformed`); a response cut off inside a list
//!   (`masters_row_unterminated`). These are the faults the closed masters
//!   parser refuses everywhere, kept for content it now reads instead of skips.
//! - **The item's field fails, the read and the row stand** (the observation is
//!   `unreadable` with its defect): an undated entry, an invalid date, a field
//!   repeated in an entry, state row or rate row, two entries on one date that
//!   differ, a `GSTRATE` that is not a plain decimal, a `GSTINELIGIBLEITC` that is
//!   not `Yes` or `No`, a repeated state or duty-head row, and an entry whose only
//!   content is children this reader does not model.
//!
//! Children not modelled are skipped through the parser's own `skip_subtree`, which
//! refuses `ERROR` at any depth, and the level records that it skipped one. This
//! is not the ledger path's reader, which skips an unmodelled child without that
//! check.
use crate::gst_details::{
    GstDetailsObservation, RawGstDetailsEntry, RawGstRateDetails, RawGstStateDetails,
};
use crate::hsn_details::{HsnDetailsObservation, RawHsnDetailsEntry};
use crate::native_masters::{
    read_text, refuse_error_element, skip_subtree, upper, NativeMastersError,
};
use quick_xml::{
    events::{BytesStart, BytesText, Event},
    Reader,
};

/// The two lists of one stock item row as read, before validation. `None` is
/// "the row had no such element", which is `NotObserved`; an element, however
/// empty, makes it `Some`.
#[derive(Debug, Default)]
pub(crate) struct StockTaxLists {
    hsn: Option<Vec<RawHsnDetailsEntry>>,
    gst: Option<Vec<RawGstDetailsEntry>>,
}

/// The list elements a stock item row carries, by upper-case name.
pub(crate) const HSN_LIST: &[u8] = b"HSNDETAILS.LIST";
pub(crate) const GST_LIST: &[u8] = b"GSTDETAILS.LIST";

impl StockTaxLists {
    /// Reads the list element `name` (just opened, or empty when `empty`) if it
    /// is one of the two; returns false for any other child.
    pub(crate) fn read_list(
        &mut self,
        reader: &mut Reader<&[u8]>,
        name: &[u8],
        empty: bool,
    ) -> Result<bool, NativeMastersError> {
        if name == HSN_LIST {
            let entry = if empty {
                RawHsnDetailsEntry::default()
            } else {
                read_hsn_entry(reader)?
            };
            self.hsn.get_or_insert_with(Vec::new).push(entry);
        } else if name == GST_LIST {
            let entry = if empty {
                RawGstDetailsEntry::default()
            } else {
                read_gst_entry(reader)?
            };
            self.gst.get_or_insert_with(Vec::new).push(entry);
        } else {
            return Ok(false);
        }
        Ok(true)
    }

    /// The row's two observations: `NotObserved` where the row had no such
    /// element, the validated entries otherwise.
    pub(crate) fn observations(self) -> (HsnDetailsObservation, GstDetailsObservation) {
        (
            self.hsn.map_or(
                HsnDetailsObservation::NotObserved,
                HsnDetailsObservation::from_raw,
            ),
            self.gst.map_or(
                GstDetailsObservation::NotObserved,
                GstDetailsObservation::from_raw,
            ),
        )
    }
}

/// Whether text between elements is only XML whitespace (space, tab, carriage
/// return, line feed), as the masters parser reads it; a non-breaking space is
/// text.
fn is_blank(text: &BytesText<'_>) -> bool {
    text.trim_matches([' ', '\t', '\r', '\n']).is_empty()
}

/// Reads the children of an element already opened, up to its closing tag.
/// `known` reads a child it models (its content consumed, through the child's
/// own closing tag unless `empty`) and returns true; any other child is skipped
/// through `skip_subtree`. Returns whether anything not modelled was seen at
/// this level: an unmodelled child, or text, a reference or CDATA outside any
/// field.
fn read_level(
    reader: &mut Reader<&[u8]>,
    mut known: impl FnMut(&mut Reader<&[u8]>, &BytesStart<'_>, bool) -> Result<bool, NativeMastersError>,
) -> Result<bool, NativeMastersError> {
    let mut skipped = false;
    loop {
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(child) => {
                refuse_error_element(&upper(child.name()))?;
                if !known(reader, &child, false)? {
                    skipped = true;
                    skip_subtree(reader)?;
                }
            }
            Event::Empty(child) => {
                refuse_error_element(&upper(child.name()))?;
                if !known(reader, &child, true)? {
                    skipped = true;
                }
            }
            Event::End(_) => return Ok(skipped),
            Event::Text(text) => skipped |= !is_blank(&text),
            Event::GeneralRef(_) | Event::CData(_) => skipped = true,
            Event::Eof => return Err(NativeMastersError::Malformed("masters_row_unterminated")),
            _ => {}
        }
    }
}

/// A field's text: `Some("")` for an empty element, the text exactly as sent
/// otherwise. A second sighting marks `repeated`.
fn read_field(
    reader: &mut Reader<&[u8]>,
    child: &BytesStart<'_>,
    empty: bool,
    slot: &mut Option<String>,
    repeated: &mut bool,
) -> Result<(), NativeMastersError> {
    let text = if empty {
        String::new()
    } else {
        read_text(reader, child.name())?
    };
    if slot.replace(text).is_some() {
        *repeated = true;
    }
    Ok(())
}

fn read_hsn_entry(reader: &mut Reader<&[u8]>) -> Result<RawHsnDetailsEntry, NativeMastersError> {
    let mut entry = RawHsnDetailsEntry::default();
    entry.skipped = read_level(reader, |reader, child, empty| {
        let slot = match upper(child.name()).as_slice() {
            b"APPLICABLEFROM" => &mut entry.applicable_from,
            b"HSNCODE" => &mut entry.hsn_code,
            b"HSN" => &mut entry.description,
            b"SRCOFHSNDETAILS" => &mut entry.source,
            _ => return Ok(false),
        };
        read_field(reader, child, empty, slot, &mut entry.repeated_field)?;
        Ok(true)
    })?;
    Ok(entry)
}

fn read_gst_entry(reader: &mut Reader<&[u8]>) -> Result<RawGstDetailsEntry, NativeMastersError> {
    let mut entry = RawGstDetailsEntry::default();
    entry.skipped = read_level(reader, |reader, child, empty| {
        let slot = match upper(child.name()).as_slice() {
            b"APPLICABLEFROM" => &mut entry.applicable_from,
            b"TAXABILITY" => &mut entry.taxability,
            b"SRCOFGSTDETAILS" => &mut entry.source,
            b"GSTINELIGIBLEITC" => &mut entry.itc_eligible,
            b"STATEWISEDETAILS.LIST" => {
                let state = if empty {
                    RawGstStateDetails::default()
                } else {
                    read_gst_state(reader)?
                };
                entry.states.push(state);
                return Ok(true);
            }
            _ => return Ok(false),
        };
        read_field(reader, child, empty, slot, &mut entry.repeated_field)?;
        Ok(true)
    })?;
    Ok(entry)
}

fn read_gst_state(reader: &mut Reader<&[u8]>) -> Result<RawGstStateDetails, NativeMastersError> {
    let mut state = RawGstStateDetails::default();
    state.skipped = read_level(reader, |reader, child, empty| {
        match upper(child.name()).as_slice() {
            b"STATENAME" => {
                read_field(
                    reader,
                    child,
                    empty,
                    &mut state.state_name,
                    &mut state.repeated_field,
                )?;
            }
            b"RATEDETAILS.LIST" => {
                let rate = if empty {
                    RawGstRateDetails::default()
                } else {
                    read_gst_rate(reader)?
                };
                state.rates.push(rate);
            }
            // Present when it holds any element or non-blank text; its content is
            // not read, and an `ERROR` inside it still refuses the read.
            b"GSTSLABRATES.LIST" => {
                if !empty && read_level(reader, |_, _, _| Ok(false))? {
                    state.slab_rates_present = true;
                }
            }
            _ => return Ok(false),
        }
        Ok(true)
    })?;
    Ok(state)
}

fn read_gst_rate(reader: &mut Reader<&[u8]>) -> Result<RawGstRateDetails, NativeMastersError> {
    let mut rate = RawGstRateDetails::default();
    rate.skipped = read_level(reader, |reader, child, empty| {
        let slot = match upper(child.name()).as_slice() {
            b"GSTRATEDUTYHEAD" => &mut rate.duty_head,
            b"GSTRATEVALUATIONTYPE" => &mut rate.valuation_type,
            b"GSTRATE" => &mut rate.rate,
            _ => return Ok(false),
        };
        read_field(reader, child, empty, slot, &mut rate.repeated_field)?;
        Ok(true)
    })?;
    Ok(rate)
}
