//! The amount of an import entry (#1872), parsed once where a batch is admitted, as
//! `TallyDate` is for the voucher date.
//!
//! An import amount is a positive number with exactly two decimals, written with digits only,
//! at most `MAX_EXACT_DECIMAL_BYTES` long, and not zero: the rule `validate_payload` applied
//! to the text from the first build that saved a batch. The text is kept as it was received
//! (`0012.50` stays `0012.50`), because the renderers write it and a saved batch is bound to
//! its bytes by a hash. Its canonical form (`12.5`) is computed here, once, from that text, so
//! the comparisons that use it have no failure to report.
use super::EntrySide;
use bridge_tally_core::ExactDecimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// The text is not an import amount.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("an import amount is a positive number with two decimals, in at most 256 bytes")]
pub(super) struct NotAnImportAmount;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ImportAmount {
    lexeme: ExactDecimal,
    canonical: ExactDecimal,
}

impl ImportAmount {
    pub(super) fn parse(text: &str) -> Result<Self, NotAnImportAmount> {
        let (whole, fractional) = text.split_once('.').ok_or(NotAnImportAmount)?;
        let digits = |part: &str| part.bytes().all(|byte| byte.is_ascii_digit());
        if whole.is_empty() || !digits(whole) || fractional.len() != 2 || !digits(fractional) {
            return Err(NotAnImportAmount);
        }
        let lexeme = ExactDecimal::parse(text.to_string()).map_err(|_| NotAnImportAmount)?;
        if lexeme.is_zero() {
            return Err(NotAnImportAmount);
        }
        // Adding zero is how the verification reads put an amount in canonical form: no
        // leading zeros, no trailing decimal zeros.
        let canonical = lexeme
            .checked_add(&ExactDecimal::zero())
            .map_err(|_| NotAnImportAmount)?;
        Ok(Self { lexeme, canonical })
    }

    /// The amount as it was received. What a renderer writes and a saved batch hashes.
    pub(super) fn as_str(&self) -> &str {
        self.lexeme.as_str()
    }

    /// The same value without leading zeros or trailing decimal zeros (`0012.50` is `12.5`).
    /// What the comparisons with the book use.
    pub(super) fn canonical(&self) -> &str {
        self.canonical.as_str()
    }

    /// The amount as a decimal, for sums.
    pub(super) fn decimal(&self) -> &ExactDecimal {
        &self.lexeme
    }

    /// The amount as the request carries it for `side`: a debit is the received text behind a
    /// minus sign, a credit is the received text. The renderers and the span identity use this.
    pub(super) fn signed_lexeme(&self, side: &EntrySide) -> String {
        Self::signed(self.as_str(), side)
    }

    /// As [`Self::signed_lexeme`], over the canonical form. The verification fingerprint uses
    /// this.
    pub(super) fn signed_canonical(&self, side: &EntrySide) -> String {
        Self::signed(self.canonical(), side)
    }

    fn signed(text: &str, side: &EntrySide) -> String {
        match side {
            EntrySide::Dr => format!("-{text}"),
            EntrySide::Cr => text.to_string(),
        }
    }
}

impl fmt::Display for ImportAmount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Serialize for ImportAmount {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ImportAmount {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::parse(&text).map_err(serde::de::Error::custom)
    }
}

/// An entry's amount as a stage of the batch holds it: the caller's text before admission,
/// an [`ImportAmount`] after. `validate_payload` is generic over both.
pub(super) trait PayloadAmount: Clone + fmt::Debug {
    fn import_amount(&self) -> Result<ImportAmount, NotAnImportAmount>;
}

impl PayloadAmount for String {
    fn import_amount(&self) -> Result<ImportAmount, NotAnImportAmount> {
        ImportAmount::parse(self)
    }
}

impl PayloadAmount for ImportAmount {
    fn import_amount(&self) -> Result<ImportAmount, NotAnImportAmount> {
        Ok(self.clone())
    }
}

#[cfg(test)]
#[path = "agent_import_amount_tests.rs"]
mod tests;
