//! The party name a `Ledger Outstandings` request carries (#2480).
//!
//! That report takes the name as the text of the `LedgerName` static
//! variable, escaped once as XML text, not inside a `CHILDOF` formula, so it
//! has its own admission, apart from [`crate::ledger_scope::LedgerScopeName`].
//! It admits exactly what the lab has read through this report as the named
//! ledger and no other: ASCII letters and digits, single inner spaces, the
//! marks of [`MEASURED_MARKS`], the formula words of [`MEASURED_FORMULA_WORDS`],
//! and the Devanagari characters of [`MEASURED_DEVANAGARI`]. Every other
//! character, every other formula word, and a space at an edge or doubled, is
//! refused by a typed error before anything is sent; nothing is escaped into
//! admission. What was measured, on which book and with what confidence, is
//! protocol reference §12a.34; the captures are in
//! `tests/fixtures/agent/party-bills/`.
//!
//! The lab read each name only where no other ledger of the book shared its
//! letters-and-digits key. This type cannot see the book, so that condition is
//! the caller's: the app checks it against the ledger catalogue before the
//! read.

use crate::ledger_scope::{FORMULA_WORDS, LEDGER_SCOPE_NAME_MAX_CHARS};

/// The marks besides letters, digits and the space that the lab read in a
/// `LedgerName` (§12a.34): each alone, and in mixes.
pub const MEASURED_MARKS: [char; 17] = [
    '-', '.', '&', '/', '(', ')', '\'', ',', '_', '"', '+', '@', '#', ':', '%', '*', '!',
];

/// The words of [`FORMULA_WORDS`] the lab read in a `LedgerName` (§12a.34),
/// in the case read. A name holding any other of them, or one of these in
/// another case, is refused.
pub const MEASURED_FORMULA_WORDS: [&str; 1] = ["and"];

/// The Devanagari characters of the one Devanagari word the lab read in a
/// `LedgerName` (§12a.34): four consonants, one independent vowel and one
/// vowel sign. No other character of the script is admitted.
pub const MEASURED_DEVANAGARI: [char; 6] = [
    '\u{0909}', '\u{0926}', '\u{093E}', '\u{0939}', '\u{0930}', '\u{0923}',
];

/// Why a name is not sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerOutstandingsPartyError {
    Empty,
    TooLong,
    /// A space at an edge, or two spaces in a row.
    Spacing,
    /// The first character no row has read in a `LedgerName`.
    UnmeasuredMark(char),
    /// A word of [`FORMULA_WORDS`] no row has read in a `LedgerName`.
    UnmeasuredFormulaWord(&'static str),
}

impl LedgerOutstandingsPartyError {
    pub fn code(self) -> &'static str {
        match self {
            Self::Empty => "party_name_empty",
            Self::TooLong => "party_name_too_long",
            Self::Spacing => "party_name_spacing",
            Self::UnmeasuredMark(_) => "party_name_mark_not_measured",
            Self::UnmeasuredFormulaWord(_) => "party_name_formula_word_not_measured",
        }
    }
}

/// A ledger name admitted to the `LedgerName` static variable of a
/// `Ledger Outstandings` request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerOutstandingsParty(String);

impl LedgerOutstandingsParty {
    pub fn new(name: &str) -> Result<Self, LedgerOutstandingsPartyError> {
        if name.is_empty() {
            return Err(LedgerOutstandingsPartyError::Empty);
        }
        if name.chars().count() > LEDGER_SCOPE_NAME_MAX_CHARS {
            return Err(LedgerOutstandingsPartyError::TooLong);
        }
        if let Some(mark) = name.chars().find(|character| !is_measured(*character)) {
            return Err(LedgerOutstandingsPartyError::UnmeasuredMark(mark));
        }
        if name.starts_with(' ') || name.ends_with(' ') || name.contains("  ") {
            return Err(LedgerOutstandingsPartyError::Spacing);
        }
        // A word is a run of letters and digits: every mark parts words as a
        // space does, so `Sales/or` holds `or`. That reads more words than a
        // space alone would, and so refuses more, never less.
        let unmeasured = name
            .split(|character: char| !character.is_ascii_alphanumeric())
            .find_map(|word| {
                FORMULA_WORDS.iter().copied().find(|operator| {
                    word.eq_ignore_ascii_case(operator)
                        && !(word == *operator && MEASURED_FORMULA_WORDS.contains(operator))
                })
            });
        if let Some(word) = unmeasured {
            return Err(LedgerOutstandingsPartyError::UnmeasuredFormulaWord(word));
        }
        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_measured(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || character == ' '
        || MEASURED_MARKS.contains(&character)
        || MEASURED_DEVANAGARI.contains(&character)
}

#[cfg(test)]
#[path = "party_name_tests.rs"]
mod tests;
