//! The one fold a narration search compares under (#1230).
//!
//! A search is a question about what a person typed against what Tally stored, so
//! two spellings of the same visible text must meet: a composed and a decomposed
//! accent, any letter case, and a run of spaces or a line break. Nothing else is
//! folded: a different letter, a dash or an apostrophe stays different, because a
//! wider fold would match text the book does not hold. This is a search key only;
//! it never resolves a master and no write depends on it.
use unicode_normalization::UnicodeNormalization;

/// `text` composed (NFC), lower-cased, with every run of whitespace, line breaks
/// included, collapsed to one space and the ends trimmed.
pub fn narration_search_key(text: &str) -> String {
    text.nfc()
        .collect::<String>()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
#[path = "text_search_tests.rs"]
mod tests;
