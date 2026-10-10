//! A ledger master's dated GST rate and taxability details (`GSTDETAILS.LIST`).
//!
//! A `List of Ledgers` read returns the list only when its FETCH names it.
//! Captured on licensed TallyPrime 7.1 (2026-10, synthetic company, fixture
//! `ledger_gst_details_live.utf16le.xml`, provenance in
//! `LEDGER_GST_DETAILS_CAPTURE_PROVENANCE.md`): a ledger without details
//! carries an empty placeholder element; one with details carries dated
//! entries, each with state-wise rows of rate heads whose `GSTRATE` is a
//! number written with leading spaces, and is absent on some heads.
//!
//! Absent, empty and zero rates are three different observations and are
//! never merged: [`GstRate`]. A rate that is not a plain non-negative decimal
//! makes this ledger's details unreadable; it is never read as zero or absent.
//! The per-ledger fail-closed rule covers the typed defects of
//! [`GstDetailsDefect`] only. A scalar with nested markup, or XML that is not
//! well formed, fails the whole read, as for every master scalar.
//!
//! Children this parser does not model are skipped; an entry that holds
//! recognised fields and skipped children is kept and says so in
//! `other_content_skipped`. Nothing here chooses an entry or sums a rate, and
//! nothing calls this observation yet.
use bridge_tally_primitives::TallyDate;
use serde::{Deserialize, Serialize};

/// What a master read observed of one ledger's GST details.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "observation", rename_all = "snake_case")]
pub enum GstDetailsObservation {
    /// The response carried no `GSTDETAILS.LIST` element for the ledger.
    #[default]
    NotObserved,
    /// The entries as returned: sorted by `applicable_from`, strictly
    /// increasing. Empty when Tally sent only an empty placeholder.
    Entries { entries: Vec<GstDetailsEntry> },
    /// The details were returned but cannot be relied on.
    Unreadable { defect: GstDetailsDefect },
}

/// One dated entry of `GSTDETAILS.LIST`. Text fields are as Tally sent them.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GstDetailsEntry {
    /// `YYYYMMDD`, a real calendar date. Always present: a missing or invalid
    /// date is a defect, never an entry.
    pub applicable_from: String,
    pub taxability: Option<String>,
    /// `SRCOFGSTDETAILS`.
    pub source: Option<String>,
    /// `GSTINELIGIBLEITC`, `Yes` or `No`.
    pub itc_eligible: Option<bool>,
    pub states: Vec<GstStateDetails>,
    /// True when the element held content this parser does not model, at the
    /// entry, in any state row or in any rate row. The read is then not the
    /// whole element; a later capture may add children (such as HSN/SAC).
    pub other_content_skipped: bool,
}

/// One `STATEWISEDETAILS.LIST`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GstStateDetails {
    /// `STATENAME` verbatim. Tally's reserved value arrives with a forbidden
    /// control character, which the parser's sanitiser writes as a marker.
    pub state_name: Option<String>,
    pub rates: Vec<GstRateDetails>,
    /// Whether `GSTSLABRATES.LIST` held anything. Its content is not read.
    pub slab_rates_present: bool,
}

/// One `RATEDETAILS.LIST`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GstRateDetails {
    pub duty_head: Option<String>,
    pub valuation_type: Option<String>,
    pub rate: GstRate,
}

/// A `GSTRATE`, with absence, emptiness and zero kept apart.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GstRate {
    /// The element was missing.
    Absent,
    /// The element was present with no text after trimming.
    Empty,
    /// A decimal equal to zero (`0`, `0.00`).
    Zero,
    /// A non-zero decimal. `raw` is exactly as sent, leading spaces kept.
    Value { raw: String, decimal: String },
}

/// Why a ledger's GST details cannot be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GstDetailsDefect {
    EntryWithoutDate,
    DateInvalid,
    /// An entry, state row or rate row carried the same field twice.
    EntryRepeatsAField,
    ConflictingEntriesOnOneDate,
    /// A `GSTRATE` that is not digits with an optional single `.` fraction.
    RateNotDecimal,
    /// A `GSTINELIGIBLEITC` other than `Yes` or `No`.
    ItcNotYesNo,
    /// An entry whose only content is children this parser does not model.
    UnrecognisedContent,
    /// Two rate rows with the same duty head in one state row, or two state
    /// rows with the same state name in one entry (compared verbatim).
    DuplicateRow,
}

/// One `GSTDETAILS.LIST` element before validation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawGstDetailsEntry {
    pub applicable_from: Option<String>,
    pub taxability: Option<String>,
    pub source: Option<String>,
    pub itc_eligible: Option<String>,
    pub states: Vec<RawGstStateDetails>,
    pub repeated_field: bool,
    /// An unrecognised child was skipped at this level.
    pub skipped: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawGstStateDetails {
    pub state_name: Option<String>,
    pub rates: Vec<RawGstRateDetails>,
    pub slab_rates_present: bool,
    pub repeated_field: bool,
    /// An unrecognised child was skipped at this level.
    pub skipped: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawGstRateDetails {
    pub duty_head: Option<String>,
    pub valuation_type: Option<String>,
    /// `None` when the element is missing; the text exactly as sent otherwise.
    pub rate: Option<String>,
    pub repeated_field: bool,
    /// An unrecognised child was skipped at this level.
    pub skipped: bool,
}

impl RawGstDetailsEntry {
    /// An empty element: nothing recognised and nothing skipped.
    fn is_placeholder(&self) -> bool {
        self.has_no_recognised_content() && !self.skipped
    }

    /// No recognised field and no state row.
    fn has_no_recognised_content(&self) -> bool {
        self.applicable_from.is_none()
            && self.taxability.is_none()
            && self.source.is_none()
            && self.itc_eligible.is_none()
            && self.states.is_empty()
    }
}

impl GstDetailsObservation {
    /// Validate the entries of one ledger. A defect fails closed for this
    /// ledger only (bridge#490); XML that is not well formed still fails the
    /// whole read, as for every field.
    pub fn from_raw(raw: Vec<RawGstDetailsEntry>) -> Self {
        match Self::try_from_raw(raw) {
            Ok(entries) => Self::Entries { entries },
            Err(defect) => Self::Unreadable { defect },
        }
    }

    fn try_from_raw(
        raw: Vec<RawGstDetailsEntry>,
    ) -> Result<Vec<GstDetailsEntry>, GstDetailsDefect> {
        let mut entries = Vec::new();
        for entry in raw.into_iter().filter(|entry| !entry.is_placeholder()) {
            entries.push(validate_entry(entry)?);
        }
        // Document order is not assumed to be date order. Every kept entry has
        // a date, so the key is total.
        entries.sort_by(|a, b| a.applicable_from.cmp(&b.applicable_from));
        let mut deduplicated: Vec<GstDetailsEntry> = Vec::with_capacity(entries.len());
        for entry in entries {
            match deduplicated.last() {
                Some(last) if last.applicable_from == entry.applicable_from => {
                    if *last != entry {
                        return Err(GstDetailsDefect::ConflictingEntriesOnOneDate);
                    }
                }
                _ => deduplicated.push(entry),
            }
        }
        Ok(deduplicated)
    }
}

fn validate_entry(entry: RawGstDetailsEntry) -> Result<GstDetailsEntry, GstDetailsDefect> {
    let repeated = entry.repeated_field
        || entry
            .states
            .iter()
            .any(|s| s.repeated_field || s.rates.iter().any(|r| r.repeated_field));
    if repeated {
        return Err(GstDetailsDefect::EntryRepeatsAField);
    }
    if entry.skipped && entry.has_no_recognised_content() {
        return Err(GstDetailsDefect::UnrecognisedContent);
    }
    let other_content_skipped = entry.skipped
        || entry
            .states
            .iter()
            .any(|s| s.skipped || s.rates.iter().any(|r| r.skipped));
    let Some(applicable_from) = entry.applicable_from else {
        return Err(GstDetailsDefect::EntryWithoutDate);
    };
    let applicable_from = applicable_from.trim().to_owned();
    if TallyDate::parse(applicable_from.as_str()).is_err() {
        return Err(GstDetailsDefect::DateInvalid);
    }
    let itc_eligible = match entry.itc_eligible.as_deref().map(str::trim) {
        None => None,
        Some("Yes") => Some(true),
        Some("No") => Some(false),
        Some(_) => return Err(GstDetailsDefect::ItcNotYesNo),
    };
    let mut states = Vec::with_capacity(entry.states.len());
    let mut seen_states: Vec<&str> = Vec::new();
    for state in &entry.states {
        if let Some(name) = state.state_name.as_deref() {
            if seen_states.contains(&name) {
                return Err(GstDetailsDefect::DuplicateRow);
            }
            seen_states.push(name);
        }
        let mut seen_heads: Vec<&str> = Vec::new();
        for head in state.rates.iter().filter_map(|r| r.duty_head.as_deref()) {
            if seen_heads.contains(&head) {
                return Err(GstDetailsDefect::DuplicateRow);
            }
            seen_heads.push(head);
        }
    }
    for state in entry.states {
        let mut rates = Vec::with_capacity(state.rates.len());
        for rate in state.rates {
            rates.push(GstRateDetails {
                duty_head: rate.duty_head,
                valuation_type: rate.valuation_type,
                rate: classify_rate(rate.rate)?,
            });
        }
        states.push(GstStateDetails {
            state_name: state.state_name,
            rates,
            slab_rates_present: state.slab_rates_present,
        });
    }
    Ok(GstDetailsEntry {
        applicable_from,
        taxability: entry.taxability,
        source: entry.source,
        itc_eligible,
        states,
        other_content_skipped,
    })
}

fn classify_rate(raw: Option<String>) -> Result<GstRate, GstDetailsDefect> {
    let Some(raw) = raw else {
        return Ok(GstRate::Absent);
    };
    let decimal = raw.trim();
    if decimal.is_empty() {
        return Ok(GstRate::Empty);
    }
    if !is_plain_decimal(decimal) {
        return Err(GstDetailsDefect::RateNotDecimal);
    }
    if decimal.bytes().all(|b| b == b'0' || b == b'.') {
        return Ok(GstRate::Zero);
    }
    let decimal = decimal.to_owned();
    Ok(GstRate::Value { raw, decimal })
}

/// Digits, then optionally one `.` and more digits. No sign, exponent or
/// separator.
fn is_plain_decimal(value: &str) -> bool {
    let (whole, fraction) = match value.split_once('.') {
        Some((whole, fraction)) => (whole, Some(fraction)),
        None => (value, None),
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    digits(whole) && fraction.is_none_or(digits)
}

#[cfg(test)]
#[path = "gst_details_tests.rs"]
mod tests;
