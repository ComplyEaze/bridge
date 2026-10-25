//! A voucher type's numbering method, its sub-method under Automatic, and its
//! duplicate setting, read at the series level (bridge#724).
//!
//! Evidence: one synthetic book on licensed `TallyPrime` 7.1 Silver on Windows,
//! 9 October 2026 (`tests/fixtures/VOUCHER_TYPE_NUMBERING_CAPTURE_PROVENANCE.md`;
//! PARTIAL). Journal (Automatic), Payment (Manual, duplicates prevented) and
//! Receipt (Manual, duplicates not prevented) were set on Tally's screen, and the
//! series-level `NUMBERINGMETHOD` and `PREVENTDUPLICATES` matched each screen. The
//! type-level `NUMBERINGMETHOD` read `None` for all three and the type-level
//! `PREVENTDUPLICATES` read `No` on all 24 types, so neither is the method: this
//! module never reads them. All 24 types held exactly one series, named
//! `Default`; three of them (Journal, Payment, Receipt) were checked against
//! Tally's screen. Automatic (Manual Override) is mapped from the series text
//! measured on a synthetic book on 13 October 2026
//! (`docs/tally/TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md` §9.8) and is on no
//! committed test fixture. Not measured: Multi-user Auto, more than one series, a
//! single series not named `Default`, a type created new, other releases, macOS.
//!
//! The sub-method (`NUMBERINGSUBMETHOD`, a direct child of the series list) is
//! evidence of a second synthetic book on the same release and edition,
//! 13 October 2026 (`tests/fixtures/VOUCHER_TYPE_NUMBERING_M8R_CAPTURE_PROVENANCE.md`;
//! PARTIAL): one voucher type, Receipt, read `Auto Retain`, then `Auto Renumber`
//! after its screen was set to renumber vouchers, then `Auto Retain` after it was
//! set back, and with it on renumber, numbers already in the book changed, at the
//! switch and on a back-dated hand entry. A later run on the same book saw the
//! same on a back-dated voucher sent by import without a number (the provenance
//! file says where that run is kept). It is read only under `Automatic`: a
//! Manual series printed `Auto Retain` on both books, which is not the numbering
//! in force there. Not measured: any wording other than those two, a series that
//! does not print the field, a batch of several imported vouchers, other voucher
//! types, editions and releases.
//!
//! Row-level shapes that are not measured are typed observations, never a
//! default: a type with no series or several, a method outside the measured set,
//! a missing or unrecognised sub-method or duplicate setting. An envelope that is
//! not the closed shape is an error. This parser does not bind rows to a company,
//! and the numbering request computes no company GUID onto its rows: the binding rests on
//! [`join_series_numbering`], which joins every row to a masters row whose GUID
//! carries the company's prefix. Each row keeps the type's `GUID` as printed, the
//! join key; a row without one, a repeated one, or one repeated by another row is
//! a typed error. The committed capture carries it on all 24 rows.

use crate::native_masters::{
    name_attribute, path_is, read_text, refuse_error_element, refuse_stray_text, skip_subtree,
    upper, within_name_bound, NativeMasterRow, NativeMastersError,
};
use crate::native_outstandings::render_native_voucher_type_export_request;
use crate::tolerant_xml::sanitize_invalid_numeric_references;
use quick_xml::{events::Event, Reader};
use std::collections::{HashMap, HashSet};

/// The `FETCH` that was sent live: the type's identity and, at both levels, the
/// two numbering fields it names, with every field of the series list (the
/// sub-method among them).
pub const VOUCHER_TYPE_NUMBERING_FETCH: &str = "NAME, PARENT, GUID, MASTERID, ALTERID, NUMBERINGMETHOD, PREVENTDUPLICATES, VOUCHERNUMBERSERIES.*";

/// Renders the voucher-type export with the numbering fields added: the
/// production builder's request with only its `FETCH` list replaced, byte for
/// byte the request that was sent live for the committed capture.
pub fn render_voucher_type_numbering_request(company: &str) -> String {
    render_native_voucher_type_export_request(company).replacen(
        "<FETCH>NAME, PARENT, GUID, MASTERID, ALTERID</FETCH>",
        &format!("<FETCH>{VOUCHER_TYPE_NUMBERING_FETCH}</FETCH>"),
        1,
    )
}

/// What an Automatic series says Tally does with the numbers of vouchers
/// already in the book when one is inserted or deleted: the series list's
/// `NUMBERINGSUBMETHOD`, matched as printed and never trimmed or case-folded.
/// Only the two measured texts are named; neither of the other two readings is
/// ever read as `AutoRetain`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberingSubMethod {
    /// `Auto Retain`: the screen's "Retain Original Voucher No.".
    AutoRetain,
    /// `Auto Renumber`: the screen's "Renumber Vouchers".
    AutoRenumber,
    /// Any other text, blank included, kept raw.
    Unrecognised(String),
    /// The series printed no such element.
    Absent,
}

/// The series-level numbering method, as Tally printed it. A value outside the
/// measured set is kept raw and is never read as `Manual`. `Automatic` carries
/// its sub-method, so no reader can take a series as Automatic without saying
/// what it does with each sub-method; under every other method the sub-method
/// is not read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeriesNumberingMethod {
    Automatic(NumberingSubMethod),
    Manual,
    /// Its series text was measured on a synthetic book on 13 October 2026
    /// (reference §9.8, bridge#724); it is on no committed test fixture.
    AutomaticManualOverride,
    Unrecognised(String),
}

/// The series-level duplicate setting: `Yes` is prevented, `No` is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateSetting {
    Prevented,
    NotPrevented,
}

/// What the response says about one voucher type's numbering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoucherTypeNumbering {
    /// Exactly one series, with a method and a duplicate setting.
    Observed {
        method: SeriesNumberingMethod,
        duplicates: DuplicateSetting,
    },
    /// No series, or more than one: not measured, so nothing is read from it.
    SeriesCount(usize),
    /// The one series lacks its method or duplicate setting, repeats one of
    /// its three fields, or prints a duplicate setting that is not `Yes` or
    /// `No`; the code names which.
    FieldInvalid(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoucherTypeNumberingRow {
    pub name: String,
    /// The type's `GUID` as printed (the join key); never defaulted: a row
    /// without one, with it repeated, or sharing it with another row is a
    /// typed parse error.
    pub guid: String,
    pub numbering: VoucherTypeNumbering,
}

/// Parses the voucher-type export of [`render_voucher_type_numbering_request`].
pub fn parse_voucher_type_numbering(
    response: &str,
) -> Result<Vec<VoucherTypeNumberingRow>, NativeMastersError> {
    let sanitized = sanitize_invalid_numeric_references(response);
    let mut reader = Reader::from_str(&sanitized);
    reader.config_mut().trim_text(false);
    let mut path = Vec::<Vec<u8>>::new();
    let mut root_seen = false;
    let mut status: Option<String> = None;
    let mut collections = 0_usize;
    let mut rows = Vec::new();
    let mut names = HashSet::new();
    let mut guids = HashSet::new();
    loop {
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
                    if !text.is_empty() && text != "1" {
                        return Err(NativeMastersError::TallyReportedFailure);
                    }
                    if status.replace(text.to_string()).is_some() {
                        return Err(NativeMastersError::Malformed("masters_status_repeated"));
                    }
                    continue;
                }
                if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA"]) && name == b"COLLECTION" {
                    collections += 1;
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) {
                    if name != b"VOUCHERTYPE" {
                        return Err(NativeMastersError::ForeignChild);
                    }
                    let row = parse_row(&mut reader, &name_attribute(&element)?)?;
                    if !names.insert(row.name.to_ascii_lowercase()) {
                        return Err(NativeMastersError::DuplicateName);
                    }
                    if !guids.insert(row.guid.to_ascii_lowercase()) {
                        return Err(NativeMastersError::DuplicateGuid);
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
                    if status.replace(String::new()).is_some() {
                        return Err(NativeMastersError::Malformed("masters_status_repeated"));
                    }
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA"]) && name == b"COLLECTION"
                {
                    collections += 1;
                } else if path_is(&path, &[b"ENVELOPE", b"BODY", b"DATA", b"COLLECTION"]) {
                    return Err(NativeMastersError::ForeignChild);
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
        // Every company has predefined voucher types: none is no answer.
        1 if rows.is_empty() => Err(NativeMastersError::VoucherTypesEmpty),
        1 => Ok(rows),
        _ => Err(NativeMastersError::Malformed("masters_collection_repeated")),
    }
}

/// One series list's three fields as printed: `None` is absent, a repeat is
/// recorded so it can be refused.
#[derive(Default)]
struct SeriesFields {
    method: Option<String>,
    sub_method: Option<String>,
    duplicates: Option<String>,
    repeated: bool,
}

impl SeriesFields {
    /// Where the series list's direct child `child` is kept, if it is read.
    fn slot(&mut self, child: &[u8]) -> Option<&mut Option<String>> {
        match child {
            b"NUMBERINGMETHOD" => Some(&mut self.method),
            b"NUMBERINGSUBMETHOD" => Some(&mut self.sub_method),
            b"PREVENTDUPLICATES" => Some(&mut self.duplicates),
            _ => None,
        }
    }
}

fn parse_row(
    reader: &mut Reader<&[u8]>,
    name: &str,
) -> Result<VoucherTypeNumberingRow, NativeMastersError> {
    let mut series = Vec::<SeriesFields>::new();
    let mut guid = None::<String>;
    loop {
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(element) => {
                let child = upper(element.name());
                refuse_error_element(&child)?;
                if child == b"VOUCHERNUMBERSERIES.LIST" {
                    series.push(parse_series(reader)?);
                } else if child == b"GUID" {
                    let text = read_text(reader, element.name())?;
                    if guid.replace(text).is_some() {
                        return Err(NativeMastersError::Malformed(
                            "masters_numbering_guid_repeated",
                        ));
                    }
                } else {
                    skip_subtree(reader)?;
                }
            }
            Event::Empty(element) => {
                let child = upper(element.name());
                refuse_error_element(&child)?;
                // A self-closed series list is still a series, with no method.
                if child == b"VOUCHERNUMBERSERIES.LIST" {
                    series.push(SeriesFields::default());
                } else if child == b"GUID" && guid.replace(String::new()).is_some() {
                    return Err(NativeMastersError::Malformed(
                        "masters_numbering_guid_repeated",
                    ));
                }
            }
            Event::Text(text) => refuse_stray_text(&text)?,
            Event::CData(_) | Event::GeneralRef(_) => {
                return Err(NativeMastersError::Malformed("masters_unexpected_text"))
            }
            Event::End(_) => break,
            Event::Eof => return Err(NativeMastersError::Malformed("masters_row_unterminated")),
            _ => {}
        }
    }
    // Identity is the join key and is never defaulted: an absent or blank
    // GUID refuses the whole answer.
    let guid = guid
        .filter(|guid| !guid.trim().is_empty())
        .ok_or(NativeMastersError::Malformed(
            "masters_numbering_guid_missing",
        ))?;
    within_name_bound(&guid)?;
    Ok(VoucherTypeNumberingRow {
        name: name.to_string(),
        guid,
        numbering: classify(&series)?,
    })
}

/// A series list's direct `NUMBERINGMETHOD`, `NUMBERINGSUBMETHOD` and
/// `PREVENTDUPLICATES`; every other child (its own lists included) is skipped.
fn parse_series(reader: &mut Reader<&[u8]>) -> Result<SeriesFields, NativeMastersError> {
    let mut fields = SeriesFields::default();
    loop {
        match reader
            .read_event()
            .map_err(|_| NativeMastersError::Malformed("masters_xml_malformed"))?
        {
            Event::Start(element) => {
                let child = upper(element.name());
                refuse_error_element(&child)?;
                match fields.slot(&child) {
                    Some(slot) => {
                        let text = read_text(reader, element.name())?;
                        let repeated = slot.replace(text).is_some();
                        fields.repeated |= repeated;
                    }
                    None => skip_subtree(reader)?,
                }
            }
            Event::Empty(element) => {
                let child = upper(element.name());
                refuse_error_element(&child)?;
                if let Some(slot) = fields.slot(&child) {
                    let repeated = slot.replace(String::new()).is_some();
                    fields.repeated |= repeated;
                }
            }
            Event::Text(text) => refuse_stray_text(&text)?,
            Event::CData(_) | Event::GeneralRef(_) => {
                return Err(NativeMastersError::Malformed("masters_unexpected_text"))
            }
            Event::End(_) => return Ok(fields),
            Event::Eof => return Err(NativeMastersError::Malformed("masters_row_unterminated")),
            _ => {}
        }
    }
}

/// One type's reading from its series. A repeat of any of the three fields is
/// `series_field_repeated` under every method, the sub-method's included: which
/// of two printed values holds is not known. Otherwise the sub-method is read
/// under `Automatic` only.
fn classify(series: &[SeriesFields]) -> Result<VoucherTypeNumbering, NativeMastersError> {
    let [one] = series else {
        return Ok(VoucherTypeNumbering::SeriesCount(series.len()));
    };
    if one.repeated {
        return Ok(VoucherTypeNumbering::FieldInvalid("series_field_repeated"));
    }
    let method = match one.method.as_deref() {
        None => return Ok(VoucherTypeNumbering::FieldInvalid("series_method")),
        Some(blank) if blank.trim().is_empty() => {
            return Ok(VoucherTypeNumbering::FieldInvalid("series_method"))
        }
        Some("Automatic") => SeriesNumberingMethod::Automatic(match one.sub_method.as_deref() {
            None => NumberingSubMethod::Absent,
            Some("Auto Retain") => NumberingSubMethod::AutoRetain,
            Some("Auto Renumber") => NumberingSubMethod::AutoRenumber,
            Some(other) => {
                within_name_bound(other)?;
                NumberingSubMethod::Unrecognised(other.to_string())
            }
        }),
        Some("Manual") => SeriesNumberingMethod::Manual,
        Some("Automatic (Manual Override)") => SeriesNumberingMethod::AutomaticManualOverride,
        Some(other) => {
            within_name_bound(other)?;
            SeriesNumberingMethod::Unrecognised(other.to_string())
        }
    };
    let duplicates = match one.duplicates.as_deref() {
        Some("Yes") => DuplicateSetting::Prevented,
        Some("No") => DuplicateSetting::NotPrevented,
        _ => return Ok(VoucherTypeNumbering::FieldInvalid("series_duplicates")),
    };
    Ok(VoucherTypeNumbering::Observed { method, duplicates })
}

/// Why the masters answer and the numbering answer cannot be joined. Each is a
/// refusal of the whole read, never a row value: a type missing a result would
/// read as "no numbering".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberingJoinError {
    /// A type is in one answer, and the other holds it by neither GUID nor name.
    TypeOnOneSideOnly,
    /// One GUID names two different types in the two answers.
    SameGuidDifferentNames,
    /// One name carries two different GUIDs in the two answers.
    SameNameDifferentGuids,
}

impl NumberingJoinError {
    /// The refusal's stable code, which carries no data.
    pub const fn code(self) -> &'static str {
        match self {
            Self::TypeOnOneSideOnly => "masters_numbering_names_differ",
            Self::SameGuidDifferentNames | Self::SameNameDifferentGuids => {
                "masters_numbering_identity_differs"
            }
        }
    }
}

impl std::fmt::Display for NumberingJoinError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "voucher-type numbering does not join the masters answer ({})",
            self.code()
        )
    }
}

impl std::error::Error for NumberingJoinError {}

/// A type found by neither GUID: the same name under another GUID is an
/// identity difference, any other is a type on one side only.
fn unmatched<'a>(name: &str, mut other_names: impl Iterator<Item = &'a str>) -> NumberingJoinError {
    if other_names.any(|other| other.eq_ignore_ascii_case(name)) {
        NumberingJoinError::SameNameDifferentGuids
    } else {
        NumberingJoinError::TypeOnOneSideOnly
    }
}

/// Joins the numbering answer to the masters answer by GUID, comparing the
/// names as a second equality (ASCII case fold, as both parsers key their
/// names, and nothing wider). One result per masters row, in the masters
/// order. Nothing is joined by position or by name alone, and every type must
/// be on both sides. Both inputs are parser output (`parse_native_masters` and
/// [`parse_voucher_type_numbering`]), which have already refused a repeated GUID
/// and a repeated name in one answer; this function checks neither again.
pub fn join_series_numbering(
    masters: &[NativeMasterRow],
    numbering: &[VoucherTypeNumberingRow],
) -> Result<Vec<VoucherTypeNumbering>, NumberingJoinError> {
    let by_guid = numbering
        .iter()
        .enumerate()
        .map(|(index, row)| (row.guid.to_ascii_lowercase(), index))
        .collect::<HashMap<_, _>>();
    let mut matched = vec![false; numbering.len()];
    let mut joined = Vec::with_capacity(masters.len());
    for row in masters {
        let Some(&index) = by_guid.get(&row.guid.to_ascii_lowercase()) else {
            return Err(unmatched(
                &row.name,
                numbering.iter().map(|other| other.name.as_str()),
            ));
        };
        let found = &numbering[index];
        if !found.name.eq_ignore_ascii_case(&row.name) {
            return Err(NumberingJoinError::SameGuidDifferentNames);
        }
        matched[index] = true;
        joined.push(found.numbering.clone());
    }
    if let Some(index) = matched.iter().position(|seen| !seen) {
        return Err(unmatched(
            &numbering[index].name,
            masters.iter().map(|other| other.name.as_str()),
        ));
    }
    Ok(joined)
}

#[cfg(test)]
#[path = "native_voucher_type_numbering_tests.rs"]
mod tests;
