//! The ledgers one read is restricted to, named by GUID (#1331).
//!
//! A Sales invoice names a handful of ledgers, but its customer's parent group
//! can hold thousands, so a read restricted by parent (#679,
//! [`crate::parent_partition`]) can still be the whole book. This part names the
//! ledgers themselves, by the GUID the ledger catalogue gave each one.
//!
//! A GUID goes inside a TDL string literal, `$GUID = "…"`. Its alphabet is
//! closed (ASCII hexadecimal digits and the hyphen), so no GUID can end the
//! literal or add a term to the formula (AGENTS.md P7). Whether Tally compares
//! `$GUID` with regard to letter case is not measured (the lab found `$Name`
//! and `$Parent` fold it); the caller sends the GUID the catalogue gave and
//! proves each row of the answer by GUID, name and parent.
//! A request builder takes this type, never a string (P2, P3).
//!
//! What Tally answers to this filter on a ledger collection, and how a GUID the
//! book does not hold answers, is the lab sheet of #1331
//! (`lab/1331-scoped-masters`).

use crate::parent_partition::PartFormula;
use std::collections::BTreeSet;

/// The name the `FILTERS` element refers to the formula by.
pub const LEDGER_GUID_FORMULA_NAME: &str = "BridgeInvoiceLedgers";

/// The longest GUID admitted. A master's GUID is its company's GUID (36
/// characters) and a hyphenated hexadecimal suffix.
pub const LEDGER_GUID_MAX_CHARS: usize = 64;

/// The most GUIDs one part may name: the most ledgers its only caller, a Sales
/// invoice, names (the invoice's entry cap in the app crate is defined as this
/// constant). A `$GUID` filter has been sent live with five terms only (#1331);
/// a six-term `$GUID` filter has not been sent. A caller that needs more terms
/// raises this bound with live evidence for the new count (AGENTS.md P1); the
/// 200-term OR formulas of protocol reference section 11e were parent filters,
/// not `$GUID` ones.
pub const LEDGER_GUID_PART_MAX_GUIDS: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerGuidError {
    Empty,
    TooLong,
    /// A character other than an ASCII hexadecimal digit or a hyphen.
    Character,
}

impl LedgerGuidError {
    pub fn code(self) -> &'static str {
        match self {
            Self::Empty => "ledger_guid_empty",
            Self::TooLong => "ledger_guid_too_long",
            Self::Character => "ledger_guid_character",
        }
    }
}

/// A ledger's GUID as Tally gave it, admitted to a TDL string literal.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LedgerGuid(String);

impl LedgerGuid {
    pub fn parse(guid: &str) -> Result<Self, LedgerGuidError> {
        if guid.is_empty() {
            return Err(LedgerGuidError::Empty);
        }
        if guid.len() > LEDGER_GUID_MAX_CHARS {
            return Err(LedgerGuidError::TooLong);
        }
        if !guid.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            return Err(LedgerGuidError::Character);
        }
        Ok(Self(guid.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerGuidPartError {
    Empty,
    TooMany,
}

impl LedgerGuidPartError {
    pub fn code(self) -> &'static str {
        match self {
            Self::Empty => "ledger_guid_part_empty",
            Self::TooMany => "ledger_guid_part_too_many",
        }
    }
}

/// The distinct ledgers one read names, in GUID order, so that the same
/// ledgers always render the same request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerGuidPart {
    guids: Vec<LedgerGuid>,
}

impl LedgerGuidPart {
    /// A ledger named twice is named once.
    pub fn new(guids: impl IntoIterator<Item = LedgerGuid>) -> Result<Self, LedgerGuidPartError> {
        let guids = guids.into_iter().collect::<BTreeSet<_>>();
        if guids.is_empty() {
            return Err(LedgerGuidPartError::Empty);
        }
        if guids.len() > LEDGER_GUID_PART_MAX_GUIDS {
            return Err(LedgerGuidPartError::TooMany);
        }
        Ok(Self {
            guids: guids.into_iter().collect(),
        })
    }

    pub fn guids(&self) -> &[LedgerGuid] {
        &self.guids
    }

    /// The one formula the part's request carries: `$GUID = "…" OR …`.
    pub fn formula(&self) -> PartFormula {
        PartFormula {
            name: LEDGER_GUID_FORMULA_NAME.to_owned(),
            text: self
                .guids
                .iter()
                .map(|guid| format!("$GUID = \"{}\"", guid.as_str()))
                .collect::<Vec<_>>()
                .join(" OR "),
        }
    }
}

#[cfg(test)]
#[path = "ledger_guid_part_tests.rs"]
mod tests;
