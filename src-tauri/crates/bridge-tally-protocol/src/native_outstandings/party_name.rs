//! The party name a `Ledger Outstandings` request carries (#2480).
//!
//! That report takes the name as the text of the `LedgerName` static
//! variable, escaped once as XML text, not inside a `CHILDOF` formula, so it
//! has its own admission, apart from [`crate::ledger_scope::LedgerScopeName`].
//! A name that type admits is admitted here unchanged. Any other name is
//! admitted only in a shape a kept capture shows Tally reading through this
//! report as the named ledger:
//!
//! - every character is an ASCII letter or digit, the space, one of
//!   [`MEASURED_MARKS`], or a character of [`MEASURED_DEVANAGARI_WORD`];
//! - spaces are single and inner, and the first character is an ASCII letter;
//! - each mark stands between neighbours [`MEASURED_MARK_PLACES`] lists for it;
//! - the one formula word read, lower-case `and`, stands alone between two
//!   other space-separated words, and no other formula word appears;
//! - Devanagari appears only as [`MEASURED_DEVANAGARI_WORD`] whole, as a
//!   space-separated word that is not the first;
//! - the name is at most [`MARKED_NAME_MAX_CHARS`] characters.
//!
//! Anything else is refused by a typed error before anything is sent; nothing
//! is escaped into admission. What was measured, on which book and with what
//! confidence, is protocol reference §12a.34; the captures are in
//! `tests/fixtures/agent/party-bills/`, and a test holds this rule to the
//! names those captures sent.
//!
//! The lab read each name only where no other ledger of the book shared its
//! letters-and-digits key. This type cannot see the book, so that condition is
//! the caller's: the app checks it against the ledger catalogue before the
//! read.

use crate::ledger_scope::{LedgerScopeName, FORMULA_WORDS, LEDGER_SCOPE_NAME_MAX_CHARS};

/// The marks besides letters, digits and the space that the lab read in a
/// `LedgerName` (§12a.34), each only where [`MEASURED_MARK_PLACES`] says.
pub const MEASURED_MARKS: [char; 17] = [
    '-', '.', '&', '/', '(', ')', '\'', ',', '_', '"', '+', '@', '#', ':', '%', '*', '!',
];

/// What stands beside a mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Neighbour {
    Letter,
    Digit,
    Space,
    /// The start or the end of the name.
    Edge,
}

/// Each mark with the neighbours before and after it in some kept capture's
/// name (§12a.34), and no other placing: a mark beside another mark, beside
/// Devanagari, or at the start of a name was never read.
pub const MEASURED_MARK_PLACES: [(char, Neighbour, Neighbour); 26] = {
    use Neighbour::{Digit, Edge, Letter, Space};
    [
        ('!', Letter, Edge),
        ('!', Letter, Space),
        ('"', Letter, Space),
        ('"', Space, Letter),
        ('#', Space, Digit),
        ('%', Digit, Space),
        ('&', Letter, Letter),
        ('&', Space, Space),
        ('\'', Letter, Letter),
        ('(', Letter, Letter),
        ('(', Space, Letter),
        (')', Letter, Letter),
        (')', Letter, Space),
        ('*', Space, Space),
        ('+', Space, Space),
        (',', Letter, Space),
        ('-', Digit, Digit),
        ('-', Letter, Letter),
        ('.', Letter, Edge),
        ('.', Letter, Letter),
        ('.', Letter, Space),
        ('/', Letter, Letter),
        (':', Digit, Space),
        (':', Letter, Space),
        ('@', Space, Space),
        ('_', Letter, Letter),
    ]
};

/// The words of [`FORMULA_WORDS`] the lab read in a `LedgerName` (§12a.34),
/// in the case read, each only as a space-separated word between two others.
pub const MEASURED_FORMULA_WORDS: [&str; 1] = ["and"];

/// The one Devanagari word the lab read in a `LedgerName` (§12a.34). Its
/// characters are admitted only as this whole word.
pub const MEASURED_DEVANAGARI_WORD: &str = "\u{0909}\u{0926}\u{093E}\u{0939}\u{0930}\u{0923}";

/// The longest name, in characters, among the kept captures that hold a mark,
/// `and` or Devanagari (§12a.34). No longer `LedgerName` has been read through
/// this report, so a marked name longer than this is refused; a name
/// [`LedgerScopeName`] admits keeps that type's limit.
pub const MARKED_NAME_MAX_CHARS: usize = 72;

/// Why a name is not sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerOutstandingsPartyError {
    Empty,
    TooLong,
    /// A space at an edge, or two spaces in a row.
    Spacing,
    /// The first character no row has read in a `LedgerName`.
    UnmeasuredMark(char),
    /// The first character that stands where no kept capture shows it: a mark
    /// beside neighbours [`MEASURED_MARK_PLACES`] does not list for it, a first
    /// character that is not an ASCII letter, or Devanagari outside
    /// [`MEASURED_DEVANAGARI_WORD`] as a later word.
    UnmeasuredPosition(char),
    /// A word of [`FORMULA_WORDS`] no row has read where it stands.
    UnmeasuredFormulaWord(&'static str),
    /// A name holding a mark, `and` or Devanagari, longer than
    /// [`MARKED_NAME_MAX_CHARS`].
    MarkedNameTooLong,
}

impl LedgerOutstandingsPartyError {
    pub fn code(self) -> &'static str {
        match self {
            Self::Empty => "party_name_empty",
            Self::TooLong => "party_name_too_long",
            Self::Spacing => "party_name_spacing",
            Self::UnmeasuredMark(_) => "party_name_mark_not_measured",
            Self::UnmeasuredPosition(_) => "party_name_position_not_measured",
            Self::UnmeasuredFormulaWord(_) => "party_name_formula_word_not_measured",
            Self::MarkedNameTooLong => "party_name_marked_too_long",
        }
    }
}

/// A ledger name admitted to the `LedgerName` static variable of a
/// `Ledger Outstandings` request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerOutstandingsParty(String);

impl LedgerOutstandingsParty {
    pub fn new(name: &str) -> Result<Self, LedgerOutstandingsPartyError> {
        use LedgerOutstandingsPartyError as Refusal;
        if name.is_empty() {
            return Err(Refusal::Empty);
        }
        if name.chars().count() > LEDGER_SCOPE_NAME_MAX_CHARS {
            return Err(Refusal::TooLong);
        }
        if LedgerScopeName::new(name).is_ok() {
            return Ok(Self(name.to_string()));
        }
        if let Some(mark) = name.chars().find(|character| !is_measured(*character)) {
            return Err(Refusal::UnmeasuredMark(mark));
        }
        if name.starts_with(' ') || name.ends_with(' ') || name.contains("  ") {
            return Err(Refusal::Spacing);
        }
        if name.chars().count() > MARKED_NAME_MAX_CHARS {
            return Err(Refusal::MarkedNameTooLong);
        }
        let characters = name.chars().collect::<Vec<_>>();
        if !characters[0].is_ascii_alphabetic() {
            return Err(Refusal::UnmeasuredPosition(characters[0]));
        }
        for (at, mark) in characters.iter().copied().enumerate() {
            if !MEASURED_MARKS.contains(&mark) {
                continue;
            }
            let before = at.checked_sub(1).map(|before| characters[before]);
            let after = characters.get(at + 1).copied();
            let placed = neighbour(before)
                .zip(neighbour(after))
                .is_some_and(|(before, after)| {
                    MEASURED_MARK_PLACES.contains(&(mark, before, after))
                });
            if !placed {
                return Err(Refusal::UnmeasuredPosition(mark));
            }
        }
        let words = name.split(' ').collect::<Vec<_>>();
        let last = words.len() - 1;
        for (at, word) in words.iter().copied().enumerate() {
            let devanagari = word.chars().find(|character| !character.is_ascii());
            if let Some(character) = devanagari {
                if word != MEASURED_DEVANAGARI_WORD || at == 0 {
                    return Err(Refusal::UnmeasuredPosition(character));
                }
            }
            // A formula word is a run of letters and digits: every mark parts
            // runs as a space does, so `Sales/or` holds `or`. That reads more
            // words than a space alone would, and so refuses more, never less.
            let between = at != 0 && at != last;
            for run in word.split(|character: char| !character.is_ascii_alphanumeric()) {
                let Some(operator) = FORMULA_WORDS
                    .iter()
                    .copied()
                    .find(|operator| run.eq_ignore_ascii_case(operator))
                else {
                    continue;
                };
                let read =
                    MEASURED_FORMULA_WORDS.contains(&operator) && word == operator && between;
                if !read {
                    return Err(Refusal::UnmeasuredFormulaWord(operator));
                }
            }
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
        || MEASURED_DEVANAGARI_WORD.contains(character)
}

/// What a neighbour is, or `None` for one no placing lists (a mark or
/// Devanagari).
fn neighbour(character: Option<char>) -> Option<Neighbour> {
    match character {
        None => Some(Neighbour::Edge),
        Some(' ') => Some(Neighbour::Space),
        Some(character) if character.is_ascii_alphabetic() => Some(Neighbour::Letter),
        Some(character) if character.is_ascii_digit() => Some(Neighbour::Digit),
        Some(_) => None,
    }
}

#[cfg(test)]
#[path = "party_name_tests.rs"]
mod tests;
