//! A party ledger's dated mailing details: address, PIN code, state and country
//! (bridge#1490).
//!
//! Tally keeps a party's mailing address in `LEDMAILINGDETAILS.LIST` entries, each
//! dated by `APPLICABLEFROM`. Measured on TallyPrime 7.1 (one synthetic book,
//! licensed Silver): a `List of Ledgers` read whose FETCH names only the flat
//! `LEDADDRESS.LIST`, `LEDPINCODE`, `LEDGSTPINCODE` and `STATENAME` gets those
//! back empty or absent while the party holds an address, and the dated list
//! comes back only when the FETCH names `LEDMAILINGDETAILS.LIST`. The measured
//! shape is recorded in `docs/tally/TALLY_PROTOCOL_REFERENCE.md` §8.4.
//!
//! The history is parsed once, here, and only [`MailingDetailsHistory::in_force`]
//! answers "which address applies on a date": there is no "first entry"
//! accessor, because the first entry can be an old address.
use bridge_tally_primitives::TallyDate;
use serde::{Deserialize, Serialize};

/// One dated mailing entry, as Tally returned it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MailingDetailsEntry {
    /// `APPLICABLEFROM`, a real `YYYYMMDD` calendar date.
    pub applicable_from: TallyDate,
    /// `MAILINGNAME`, when returned and not empty.
    pub mailing_name: Option<String>,
    /// The `ADDRESS` lines of `ADDRESS.LIST`, in the order Tally sent them.
    pub address_lines: Vec<String>,
    /// `STATE`, when returned and not empty.
    pub state: Option<String>,
    /// `COUNTRY`, when returned and not empty.
    pub country: Option<String>,
    /// `PINCODE`, when returned and not empty.
    pub pin_code: Option<String>,
    /// The entry held a child, or text inside `ADDRESS.LIST`, that this reader
    /// does not take: the address lines shown may be missing some of what Tally
    /// holds. Set, never inferred away (the precedent is `gst_details`).
    #[serde(default)]
    pub other_content_skipped: bool,
}

/// Why a returned mailing history cannot be read as one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MailingDetailsDefect {
    /// An entry carries content but no date.
    EntryWithoutDate,
    /// An `APPLICABLEFROM` that is not a real `YYYYMMDD` calendar date.
    DateInvalid,
    /// One entry carried the same field twice.
    EntryRepeatsAField,
    /// Two entries share a date but say different things.
    ConflictingEntriesOnOneDate,
}

impl MailingDetailsDefect {
    /// The spelling the defect is serialised under, for text that names it.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EntryWithoutDate => "entry_without_date",
            Self::DateInvalid => "date_invalid",
            Self::EntryRepeatsAField => "entry_repeats_a_field",
            Self::ConflictingEntriesOnOneDate => "conflicting_entries_on_one_date",
        }
    }
}

/// What a party-master read observed of the mailing history.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "observation", rename_all = "snake_case")]
pub enum MailingDetailsHistory {
    /// The response carried no `LEDMAILINGDETAILS.LIST` element at all, so the
    /// history was not read. Not the same as an empty history.
    #[default]
    NotObserved,
    /// The history as returned: sorted by `applicable_from`, strictly
    /// increasing. Empty when Tally sent only an empty placeholder, which is
    /// what a ledger with no mailing details gets.
    Entries { entries: Vec<MailingDetailsEntry> },
    /// The history was returned but cannot be relied on.
    Unreadable { defect: MailingDetailsDefect },
}

/// One `LEDMAILINGDETAILS.LIST` element's fields before validation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawMailingDetailsEntry {
    pub applicable_from: Option<String>,
    pub mailing_name: Option<String>,
    pub address_lines: Vec<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub pin_code: Option<String>,
    /// The element carried one of these single fields (or `ADDRESS.LIST`) more
    /// than once.
    pub repeated_field: bool,
    /// A child, or text inside `ADDRESS.LIST`, that is not one of the fields above.
    pub other_content_skipped: bool,
    /// The element held any child or any text, whether or not it was read or
    /// empty. Only an element that held nothing is a placeholder.
    pub has_content: bool,
}

impl RawMailingDetailsEntry {
    fn is_placeholder(&self) -> bool {
        !self.repeated_field
            && !self.has_content
            && self.applicable_from.is_none()
            && self.mailing_name.is_none()
            && self.address_lines.is_empty()
            && self.state.is_none()
            && self.country.is_none()
            && self.pin_code.is_none()
    }
}

impl MailingDetailsHistory {
    /// Validate the entries of one ledger. A defect here fails closed for this
    /// ledger only, so it never refuses the rest of a book (bridge#490). XML
    /// that is not well formed still fails the whole read, as for every field.
    pub fn from_raw(raw: Vec<RawMailingDetailsEntry>) -> Self {
        let mut entries = Vec::new();
        for entry in raw.into_iter().filter(|entry| !entry.is_placeholder()) {
            if entry.repeated_field {
                return Self::unreadable(MailingDetailsDefect::EntryRepeatsAField);
            }
            let Some(applicable_from) = entry.applicable_from else {
                return Self::unreadable(MailingDetailsDefect::EntryWithoutDate);
            };
            let Ok(applicable_from) = TallyDate::parse(applicable_from) else {
                return Self::unreadable(MailingDetailsDefect::DateInvalid);
            };
            entries.push(MailingDetailsEntry {
                applicable_from,
                mailing_name: entry.mailing_name,
                address_lines: entry.address_lines,
                state: entry.state,
                country: entry.country,
                pin_code: entry.pin_code,
                other_content_skipped: entry.other_content_skipped,
            });
        }
        // Document order is not assumed to be date order.
        entries.sort_by(|a, b| a.applicable_from.cmp(&b.applicable_from));
        let mut deduplicated: Vec<MailingDetailsEntry> = Vec::with_capacity(entries.len());
        for entry in entries {
            match deduplicated.last() {
                Some(last) if last.applicable_from == entry.applicable_from => {
                    if *last != entry {
                        return Self::unreadable(MailingDetailsDefect::ConflictingEntriesOnOneDate);
                    }
                }
                _ => deduplicated.push(entry),
            }
        }
        Self::Entries {
            entries: deduplicated,
        }
    }

    fn unreadable(defect: MailingDetailsDefect) -> Self {
        Self::Unreadable { defect }
    }

    /// The entry in force on `as_of`: the one with the latest
    /// `applicable_from` on or before it. `None` when no entry applies yet, or
    /// when the history was not observed or is unreadable.
    pub fn in_force(&self, as_of: &TallyDate) -> Option<&MailingDetailsEntry> {
        match self {
            Self::Entries { entries } => entries
                .iter()
                .rev()
                .find(|entry| entry.applicable_from <= *as_of),
            Self::NotObserved | Self::Unreadable { .. } => None,
        }
    }
}

#[cfg(test)]
#[path = "mailing_details_tests.rs"]
mod tests;
