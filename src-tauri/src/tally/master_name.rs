//! The name of one master Bridge will send: a ledger today, a stock item next.
//!
//! [`MasterName`] refuses a name Bridge has not measured Tally to match
//! exactly. Non-ASCII is refused on purpose: a non-ASCII name was seen to
//! read back as mojibake, and how Tally matches such a name is unmeasured.

use bridge_tally_core::master_binding::MAX_NAME_CHARS;

/// A ledger name Bridge will send. See the module doc for why non-ASCII is
/// refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MasterName(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MasterNameError {
    Empty,
    NonAscii,
    ControlCharacter,
    SurroundingWhitespace,
    RepeatedSpaces,
    TooLong,
}

impl MasterName {
    /// Refuses, in order: an empty name, any non-ASCII character, any ASCII
    /// control character (0x00-0x1F, 0x7F), leading or trailing whitespace, a
    /// run of two or more spaces, and a name over
    /// `bridge_tally_core::master_binding::MAX_NAME_CHARS`. That cap is the
    /// existing master-name bound for pathological input, not a measured Tally
    /// limit.
    pub(crate) fn new(value: &str) -> Result<Self, MasterNameError> {
        if value.is_empty() {
            return Err(MasterNameError::Empty);
        }
        if !value.is_ascii() {
            return Err(MasterNameError::NonAscii);
        }
        if value.bytes().any(|b| b.is_ascii_control()) {
            return Err(MasterNameError::ControlCharacter);
        }
        if value.starts_with(' ') || value.ends_with(' ') {
            return Err(MasterNameError::SurroundingWhitespace);
        }
        if value.contains("  ") {
            return Err(MasterNameError::RepeatedSpaces);
        }
        if value.len() > MAX_NAME_CHARS {
            return Err(MasterNameError::TooLong);
        }
        Ok(Self(value.to_string()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
#[path = "master_name_tests.rs"]
mod tests;
