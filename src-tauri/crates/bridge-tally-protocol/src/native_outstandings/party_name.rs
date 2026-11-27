//! The party name a `Ledger Outstandings` request carries (#2480).
//!
//! That report takes the name as the text of the `LedgerName` static
//! variable, escaped once as XML text, not inside a `CHILDOF` formula, so it
//! has its own admission, apart from [`crate::ledger_scope::LedgerScopeName`].
//! A name that type admits is admitted here unchanged. Any other name is
//! admitted only in a shape built from the names of the kept captures that
//! show Tally reading it through this report as the named ledger:
//!
//! - every character is an ASCII letter or digit, the space, one of
//!   [`MEASURED_MARKS`], or a character of [`MEASURED_DEVANAGARI_WORD`];
//! - spaces are single and inner, and the first character is an ASCII letter;
//! - each mark stands between neighbours [`MEASURED_MARK_PLACES`] lists for it,
//!   each such placing at most once, and each mark at most as often as
//!   [`MEASURED_MARK_MOST`] says;
//! - a `"` or a bracket that has a space on one side opens and closes a pair
//!   within one space-separated word, never inside another pair; a bracket
//!   inside a word stands alone, the name's only bracket or quote;
//! - the one formula word read, lower-case `and`, stands once between two
//!   other words, and no other formula word appears;
//! - Devanagari appears only once, as [`MEASURED_DEVANAGARI_WORD`] whole;
//! - each two neighbouring space-separated words are a pair of kinds
//!   [`MEASURED_WORD_PAIRS`] lists;
//! - the name is at most [`MARKED_NAME_MAX_CHARS`] characters.
//!
//! Anything else is refused by a typed error before anything is sent; nothing
//! is escaped into admission. What was measured, on which book and with what
//! confidence, is protocol reference §12a.34; the captures are in
//! `tests/fixtures/agent/party-bills/`, and a test holds the tables above to
//! the names those captures sent.
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
/// Devanagari, or at the start of a name was never read. No kept name holds
/// one placing twice, so a name may hold each at most once.
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

/// The most times one kept name holds each mark (§12a.34).
pub const MEASURED_MARK_MOST: [(char, usize); 17] = [
    ('!', 1),
    ('"', 2),
    ('#', 1),
    ('%', 1),
    ('&', 1),
    ('\'', 1),
    ('(', 1),
    (')', 1),
    ('*', 1),
    ('+', 1),
    (',', 1),
    ('-', 2),
    ('.', 1),
    ('/', 1),
    (':', 1),
    ('@', 1),
    ('_', 1),
];

/// What a space-separated word of a name is, for [`MEASURED_WORD_PAIRS`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WordKind {
    /// Before the first word or after the last.
    Edge,
    /// A word holding an ASCII letter or digit, other than `and`.
    Word,
    /// Lower-case `and`.
    And,
    /// [`MEASURED_DEVANAGARI_WORD`].
    Devanagari,
    /// A word that is one mark alone.
    Mark(char),
}

/// Each pair of neighbouring words, by kind, that some kept capture's name
/// holds (§12a.34), and no other pair: two marks standing alone side by side,
/// `and` beside a mark, or a mark before the Devanagari word was never read.
pub const MEASURED_WORD_PAIRS: [(WordKind, WordKind); 16] = {
    use WordKind::{And, Devanagari, Edge, Mark, Word};
    [
        (Edge, Word),
        (Word, Edge),
        (Word, Word),
        (Word, And),
        (And, Word),
        (Word, Devanagari),
        (Devanagari, Edge),
        (Devanagari, Mark('&')),
        (Word, Mark('&')),
        (Mark('&'), Word),
        (Word, Mark('*')),
        (Mark('*'), Word),
        (Word, Mark('+')),
        (Mark('+'), Word),
        (Word, Mark('@')),
        (Mark('@'), Word),
    ]
};

/// The words of [`FORMULA_WORDS`] the lab read in a `LedgerName` (§12a.34),
/// in the case read, each only as a space-separated word between two others,
/// and once.
pub const MEASURED_FORMULA_WORDS: [&str; 1] = ["and"];

/// The one Devanagari word the lab read in a `LedgerName` (§12a.34). Its
/// characters are admitted only as this whole word, once.
pub const MEASURED_DEVANAGARI_WORD: &str = "\u{0909}\u{0926}\u{093E}\u{0939}\u{0930}\u{0923}";

/// The longest name, in characters, among the kept captures that hold a mark,
/// `and` or Devanagari (§12a.34). No longer `LedgerName` has been read through
/// this report, so a marked name longer than this is refused; a name
/// [`LedgerScopeName`] admits keeps that type's limit.
pub const MARKED_NAME_MAX_CHARS: usize = 72;

/// Which limit on what a name holds together was passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combination {
    /// The mark more often than [`MEASURED_MARK_MOST`] allows, or one of its
    /// placings twice.
    Repeated(char),
    /// A quote or bracket not paired within one word, closed with none open,
    /// opened inside another pair, or inside a word beside another.
    Unpaired(char),
    /// `and` or the Devanagari word more than once.
    RepeatedWord,
    /// Two neighbouring words whose kinds [`MEASURED_WORD_PAIRS`] does not list.
    Adjacent,
}

/// Why a name is not sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerOutstandingsPartyError {
    Empty,
    /// Longer than [`LEDGER_SCOPE_NAME_MAX_CHARS`], Bridge's own limit.
    TooLong,
    /// A space at an edge, or two spaces in a row.
    Spacing,
    /// A character no row has read anywhere in a `LedgerName`, wherever it
    /// stands: the first such character of the name. It need not be a mark: a
    /// letter of another script, or a Devanagari letter or sign outside
    /// [`MEASURED_DEVANAGARI_WORD`], is refused here too, first or not.
    UnmeasuredMark(char),
    /// A character read in some `LedgerName`, standing where no kept name has
    /// it: a mark beside neighbours [`MEASURED_MARK_PLACES`] does not list for
    /// it; a first character that is not an ASCII letter (a letter of
    /// [`MEASURED_DEVANAGARI_WORD`] first lands here, another Devanagari
    /// character first is [`Self::UnmeasuredMark`]); or Devanagari other than
    /// that word whole as a later word.
    UnmeasuredPosition(char),
    /// A word of [`FORMULA_WORDS`] no row has read where it stands.
    UnmeasuredFormulaWord(&'static str),
    /// A name holding a mark, `and` or Devanagari, longer than
    /// [`MARKED_NAME_MAX_CHARS`].
    MarkedNameTooLong,
    /// Every character in a measured place, together in a way no kept name
    /// shows.
    UnmeasuredCombination(Combination),
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
            Self::UnmeasuredCombination(_) => "party_name_combination_not_measured",
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
        let placings = mark_placings(&characters).map_err(Refusal::UnmeasuredPosition)?;
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
        combination(&characters, &placings, &words).map_err(Refusal::UnmeasuredCombination)?;
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

/// Each mark of the name in order with its placing, or the first mark placed
/// where [`MEASURED_MARK_PLACES`] does not list.
fn mark_placings(characters: &[char]) -> Result<Vec<(char, Neighbour, Neighbour)>, char> {
    let mut placings = Vec::new();
    for (at, mark) in characters.iter().copied().enumerate() {
        if !MEASURED_MARKS.contains(&mark) {
            continue;
        }
        let before = neighbour(at.checked_sub(1).map(|before| characters[before]));
        let after = neighbour(characters.get(at + 1).copied());
        let placing = before
            .zip(after)
            .map(|(before, after)| (mark, before, after))
            .filter(|placing| MEASURED_MARK_PLACES.contains(placing))
            .ok_or(mark)?;
        placings.push(placing);
    }
    Ok(placings)
}

/// What one space-separated word is.
fn word_kind(word: &str) -> WordKind {
    let mut characters = word.chars();
    match (characters.next(), characters.next()) {
        _ if word == "and" => WordKind::And,
        _ if word == MEASURED_DEVANAGARI_WORD => WordKind::Devanagari,
        (Some(mark), None) if MEASURED_MARKS.contains(&mark) => WordKind::Mark(mark),
        _ => WordKind::Word,
    }
}

/// The limits on what the name holds together, each from the kept names.
fn combination(
    characters: &[char],
    placings: &[(char, Neighbour, Neighbour)],
    words: &[&str],
) -> Result<(), Combination> {
    for (at, placing) in placings.iter().enumerate() {
        if placings[..at].contains(placing) {
            return Err(Combination::Repeated(placing.0));
        }
    }
    for (mark, most) in MEASURED_MARK_MOST {
        if characters
            .iter()
            .filter(|character| **character == mark)
            .count()
            > most
        {
            return Err(Combination::Repeated(mark));
        }
    }
    let kinds = words.iter().map(|word| word_kind(word)).collect::<Vec<_>>();
    for once in [WordKind::And, WordKind::Devanagari] {
        if kinds.iter().filter(|kind| **kind == once).count() > 1 {
            return Err(Combination::RepeatedWord);
        }
    }
    paired(words)?;
    let bounded = std::iter::once(WordKind::Edge)
        .chain(kinds)
        .chain(std::iter::once(WordKind::Edge))
        .collect::<Vec<_>>();
    if bounded
        .windows(2)
        .any(|pair| !MEASURED_WORD_PAIRS.contains(&(pair[0], pair[1])))
    {
        return Err(Combination::Adjacent);
    }
    Ok(())
}

/// Quotes and brackets as the kept names hold them: a `"` or `(` that starts a
/// word opens a pair that the matching mark ending the same word closes; a
/// bracket inside a word is the name's only bracket or quote. Nothing else
/// opens or closes, so no pair spans two words or holds another.
fn paired(words: &[&str]) -> Result<(), Combination> {
    let total = words
        .iter()
        .flat_map(|word| word.chars())
        .filter(|character| matches!(character, '"' | '(' | ')'))
        .count();
    for word in words {
        let characters = word.chars().collect::<Vec<_>>();
        let end = characters.len() - 1;
        let mut open: Option<char> = None;
        for (at, mark) in characters.iter().copied().enumerate() {
            match mark {
                '(' | ')' if at != 0 && at != end => {
                    if total != 1 {
                        return Err(Combination::Unpaired(mark));
                    }
                }
                '"' | '(' if at == 0 => {
                    if open.is_some() {
                        return Err(Combination::Unpaired(mark));
                    }
                    open = Some(mark);
                }
                '"' | ')' if at == end => {
                    let opener = if mark == ')' { '(' } else { '"' };
                    if open != Some(opener) {
                        return Err(Combination::Unpaired(mark));
                    }
                    open = None;
                }
                '"' | '(' | ')' => return Err(Combination::Unpaired(mark)),
                _ => {}
            }
        }
        if let Some(mark) = open {
            return Err(Combination::Unpaired(mark));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "party_name_tests.rs"]
mod tests;
