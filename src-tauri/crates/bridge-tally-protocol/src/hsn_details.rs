//! A stock item's dated HSN details (`HSNDETAILS.LIST`).
//!
//! Captured on licensed TallyPrime 7.1 (2026-10, one synthetic company, fixtures
//! `masters_stock_items_shape_lab_*_live.utf16le.xml`, provenance in
//! `MASTERS_CAPTURE_PROVENANCE.md`): an item without details carries an empty
//! placeholder element; one with details carries one list element per dated
//! entry, each with `APPLICABLEFROM`, `HSNCODE`, `HSN` (a description Tally fills
//! from its own list) and `SRCOFHSNDETAILS`. The entry Tally adds on an item's
//! first save has a date and a source but no code.
//!
//! The three states and the per-item fail-closed rule are those of
//! [`crate::gst_details`], whose dated-entry sort and conflict check this module
//! shares: a defect in one item's list makes that item's observation
//! `unreadable` and nothing else; XML that is not well formed, a nested tag in a
//! scalar and Tally's own `ERROR` elements are not defects of an entry, they
//! refuse the read (see `stock_tax_details`).
//!
//! The code is text exactly as sent: never trimmed, never cut to digits or to a
//! length, never read as a number. An absent code (Tally's own first row) and an
//! empty one are different observations.
use crate::gst_details::{sorted_without_repeats, DatedEntriesConflict};
use bridge_tally_primitives::TallyDate;
use serde::{Deserialize, Serialize};

/// What a master read observed of one stock item's HSN details.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "observation", rename_all = "snake_case")]
pub enum HsnDetailsObservation {
    /// The response carried no `HSNDETAILS.LIST` element for the item.
    #[default]
    NotObserved,
    /// The entries as returned: sorted by `applicable_from`, strictly
    /// increasing. Empty when Tally sent only an empty placeholder, which is
    /// not a statement that the item has no HSN: a stock group's or the
    /// company's code is not on the item's list.
    Entries { entries: Vec<HsnDetailsEntry> },
    /// The details were returned but cannot be relied on.
    Unreadable { defect: HsnDetailsDefect },
}

/// One dated entry of `HSNDETAILS.LIST`. Text fields are as Tally sent them.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct HsnDetailsEntry {
    /// `YYYYMMDD`, a real calendar date. Always present: a missing or invalid
    /// date is a defect, never an entry.
    pub applicable_from: String,
    /// `HSNCODE` verbatim; absent when the entry has none (Tally's own row).
    pub hsn_code: Option<String>,
    /// `HSN`, the description Tally fills in. Text a person can edit; verbatim.
    pub description: Option<String>,
    /// `SRCOFHSNDETAILS`.
    pub source: Option<String>,
    /// True when the element held content this parser does not model.
    pub other_content_skipped: bool,
}

/// Why an item's HSN details cannot be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HsnDetailsDefect {
    EntryWithoutDate,
    DateInvalid,
    /// An entry carried the same field twice.
    EntryRepeatsAField,
    ConflictingEntriesOnOneDate,
    /// An entry whose only content is children this parser does not model, or
    /// text, a reference or CDATA directly in the list.
    UnrecognisedContent,
}

/// One `HSNDETAILS.LIST` element before validation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawHsnDetailsEntry {
    pub applicable_from: Option<String>,
    pub hsn_code: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub repeated_field: bool,
    /// An unrecognised child, or text outside any field, was skipped.
    pub skipped: bool,
}

impl RawHsnDetailsEntry {
    /// An empty element: nothing recognised and nothing skipped.
    fn is_placeholder(&self) -> bool {
        self.has_no_recognised_content() && !self.skipped
    }

    fn has_no_recognised_content(&self) -> bool {
        self.applicable_from.is_none()
            && self.hsn_code.is_none()
            && self.description.is_none()
            && self.source.is_none()
    }
}

impl HsnDetailsObservation {
    /// Validate the entries of one item. A defect fails closed for this item
    /// only; the read stands.
    pub fn from_raw(raw: Vec<RawHsnDetailsEntry>) -> Self {
        match Self::try_from_raw(raw) {
            Ok(entries) => Self::Entries { entries },
            Err(defect) => Self::Unreadable { defect },
        }
    }

    fn try_from_raw(
        raw: Vec<RawHsnDetailsEntry>,
    ) -> Result<Vec<HsnDetailsEntry>, HsnDetailsDefect> {
        let mut entries = Vec::new();
        for entry in raw.into_iter().filter(|entry| !entry.is_placeholder()) {
            entries.push(validate_entry(entry)?);
        }
        sorted_without_repeats(entries, |entry| entry.applicable_from.as_str())
            .map_err(|DatedEntriesConflict| HsnDetailsDefect::ConflictingEntriesOnOneDate)
    }
}

fn validate_entry(entry: RawHsnDetailsEntry) -> Result<HsnDetailsEntry, HsnDetailsDefect> {
    if entry.repeated_field {
        return Err(HsnDetailsDefect::EntryRepeatsAField);
    }
    if entry.skipped && entry.has_no_recognised_content() {
        return Err(HsnDetailsDefect::UnrecognisedContent);
    }
    let Some(applicable_from) = entry.applicable_from else {
        return Err(HsnDetailsDefect::EntryWithoutDate);
    };
    let applicable_from = applicable_from.trim().to_owned();
    if TallyDate::parse(applicable_from.as_str()).is_err() {
        return Err(HsnDetailsDefect::DateInvalid);
    }
    Ok(HsnDetailsEntry {
        applicable_from,
        hsn_code: entry.hsn_code,
        description: entry.description,
        source: entry.source,
        other_content_skipped: entry.skipped,
    })
}

#[cfg(test)]
#[path = "hsn_details_tests.rs"]
mod tests;
