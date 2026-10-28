//! The ledger name a ledger-scoped voucher request is built on (#485).
//!
//! A ledger-scoped request asks Tally for the vouchers of one ledger with
//! `<TYPE>Vouchers : Ledger</TYPE><CHILDOF>name</CHILDOF>` instead of reading
//! the whole book's window. `CHILDOF` takes a formula, not a quoted literal
//! (Tally's own form is `$$GroupSundryDebtors`), so there is no literal whose end
//! a closed character set could guard: the only names known to be read as names
//! are the ones observed, plain words and digits with single spaces (`DN Sales`,
//! `DN Party 001`, protocol reference section 11c.6). The admission is therefore
//! the observed alphabet and nothing wider: ASCII letters, ASCII digits and
//! single spaces. Everything else, including `& . , - _`, other scripts and
//! every symbol, is refused, never escaped. A name that is also a TDL word
//! operator is not a character problem and this type does not refuse it: the
//! caller builds the type only from a ledger it resolved against Tally's own
//! catalogue, so what is sent is a name the book holds, but that does not
//! establish that Tally reads such a name as a name (not established, section
//! 11c.6).
//! A request builder takes this type, never a string (AGENTS.md P2, P3, P7).
//!
//! The alphabet can be widened only by a lab run that reads a ledger whose name
//! carries each added mark, a word operator, digits with a dash, and another
//! script, and compares the answers with the book-wide read.

/// The longest ledger name admitted: the longest master name the import accepts.
pub const LEDGER_SCOPE_NAME_MAX_CHARS: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerScopeNameError {
    Empty,
    TooLong,
    /// Leading or trailing space, or two spaces in a row.
    Spacing,
    /// A character outside the admitted alphabet.
    Character,
}

impl LedgerScopeNameError {
    pub fn code(self) -> &'static str {
        match self {
            Self::Empty => "ledger_scope_name_empty",
            Self::TooLong => "ledger_scope_name_too_long",
            Self::Spacing => "ledger_scope_name_spacing",
            Self::Character => "ledger_scope_name_character",
        }
    }
}

/// A ledger name admitted to the `CHILDOF` position of a ledger-scoped request: ASCII
/// letters and digits, and single spaces between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerScopeName(String);

impl LedgerScopeName {
    pub fn new(name: &str) -> Result<Self, LedgerScopeNameError> {
        if name.is_empty() {
            return Err(LedgerScopeNameError::Empty);
        }
        if name.chars().count() > LEDGER_SCOPE_NAME_MAX_CHARS {
            return Err(LedgerScopeNameError::TooLong);
        }
        let mut previous_space = true;
        for character in name.chars() {
            let admitted = character.is_ascii_alphanumeric() || character == ' ';
            if !admitted {
                return Err(LedgerScopeNameError::Character);
            }
            if character == ' ' && previous_space {
                return Err(LedgerScopeNameError::Spacing);
            }
            previous_space = character == ' ';
        }
        if previous_space {
            return Err(LedgerScopeNameError::Spacing);
        }
        Ok(Self(name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two names the lab read ledger-scoped (protocol reference section 11c.6) are admitted as
    /// they are.
    #[test]
    fn the_names_the_lab_observed_are_admitted_unchanged() {
        for name in ["DN Sales", "DN Party 001"] {
            assert_eq!(LedgerScopeName::new(name).unwrap().as_str(), name);
        }
    }

    #[test]
    fn plain_words_digits_and_single_spaces_are_admitted() {
        for name in ["Cash", "ABC", "7", "Sales 2024", "a b c"] {
            assert!(LedgerScopeName::new(name).is_ok(), "{name}");
        }
    }

    /// Nothing outside ASCII letters, digits and spaces is admitted, whether or not it looks
    /// harmless: the marks never observed in this position, every quote, bracket and operator, the
    /// control characters, and every non-ASCII character, the look-alikes of quotes and brackets
    /// among them. A caller will read the whole window for such a name.
    #[test]
    fn everything_outside_the_observed_alphabet_is_refused_not_escaped() {
        for name in [
            "Ram & Sons",
            "M.S. Traders",
            "Sales 2024-25",
            "Sales_Export",
            "A,B",
            "A\"B",
            "A'B",
            "A$B",
            "A<B",
            "A>B",
            "A{B}",
            "A=B",
            "A;B",
            "A\\B",
            "A|B",
            "A(B)",
            "A/B",
            "A:B",
            "A\nB",
            "A\tB",
            "A\u{0}B",
            "A\u{fffd}B",
            "A\u{200b}B",
            "A\u{4}B",
            "A\u{2bc}B",
            "A\u{ff02}B",
            "A\u{ff1c}B",
            "A\u{2460}B",
            "\u{ff21}BC",
            "भारत",
            "ट्रेडर्स",
            "caf\u{e9}",
            "</CHILDOF>",
            "DN Sales\"/><FILTERS>",
        ] {
            assert_eq!(
                LedgerScopeName::new(name),
                Err(LedgerScopeNameError::Character),
                "{name:?}"
            );
        }
    }

    #[test]
    fn empty_edge_spaced_doubled_and_overlong_names_are_refused_by_their_own_reason() {
        assert_eq!(LedgerScopeName::new(""), Err(LedgerScopeNameError::Empty));
        for name in [" A", "A ", "A  B", " "] {
            assert_eq!(
                LedgerScopeName::new(name),
                Err(LedgerScopeNameError::Spacing),
                "{name:?}"
            );
        }
        let longest = "a".repeat(LEDGER_SCOPE_NAME_MAX_CHARS);
        assert!(LedgerScopeName::new(&longest).is_ok());
        assert_eq!(
            LedgerScopeName::new(&format!("{longest}a")),
            Err(LedgerScopeNameError::TooLong)
        );
    }

    #[test]
    fn each_refusal_has_its_own_code() {
        let codes = [
            LedgerScopeNameError::Empty,
            LedgerScopeNameError::TooLong,
            LedgerScopeNameError::Spacing,
            LedgerScopeNameError::Character,
        ]
        .map(LedgerScopeNameError::code);
        let mut sorted = codes.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 4);
    }
}
