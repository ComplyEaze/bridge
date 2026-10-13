//! A company GUID parsed once, at the boundary, into a value that cannot be
//! malformed.
//!
//! A company GUID is **not** company identity by itself: a year-end split
//! copies the parent's GUID onto the child book
//! (`docs/tally/TALLY_PROTOCOL_REFERENCE_COMPANY_IDENTITY_AND_CREATION.md`
//! §9.11b). The complete observed company tuple, held by
//! `VerifiedCompanyIdentity`, stays the identity; this type only guarantees
//! that the GUID component of that tuple has the one accepted spelling.

use std::fmt;

/// Length of the accepted spelling: 32 hex digits and four hyphens.
const GUID_BYTES: usize = 36;
/// Byte offsets of the hyphens in the accepted `8-4-4-4-12` spelling.
const HYPHEN_OFFSETS: [usize; 4] = [8, 13, 18, 23];

/// Why a string is not an accepted company GUID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CompanyGuidError {
    /// The value is not exactly 36 bytes long.
    #[error("a company GUID must be exactly 36 bytes")]
    WrongLength,
    /// A hyphen is missing at one of the four fixed offsets, or one stands
    /// anywhere else.
    #[error("a company GUID must have hyphens at byte offsets 8, 13, 18 and 23 only")]
    MisplacedHyphen,
    /// A byte that is not an ASCII hexadecimal digit stands where one is
    /// required.
    #[error("a company GUID must hold ASCII hexadecimal digits between its hyphens")]
    NotHexadecimal,
}

/// A company GUID in the strict `8-4-4-4-12` spelling, stored lowercase.
///
/// The field is private and the type has no `Default`, no `From<String>` and no
/// `Deserialize`: the only way to hold one is [`CompanyGuid::parse`], so a
/// value that bypassed the parse cannot exist.
///
/// The parse never trims, and refuses braces, the un-hyphenated 32-digit form,
/// the `urn:uuid:` form and non-ASCII text. Case is accepted in either form and
/// normalised, so equality is exact on the stored lowercase text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompanyGuid(String);

impl CompanyGuid {
    /// Accepts exactly 36 bytes: an ASCII hexadecimal digit at every offset
    /// except 8, 13, 18 and 23, which must be a hyphen.
    pub fn parse(value: &str) -> Result<Self, CompanyGuidError> {
        let bytes = value.as_bytes();
        if bytes.len() != GUID_BYTES {
            return Err(CompanyGuidError::WrongLength);
        }
        for (offset, byte) in bytes.iter().enumerate() {
            let hyphen_here = HYPHEN_OFFSETS.contains(&offset);
            if hyphen_here != (*byte == b'-') {
                return Err(CompanyGuidError::MisplacedHyphen);
            }
            if !hyphen_here && !byte.is_ascii_hexdigit() {
                return Err(CompanyGuidError::NotHexadecimal);
            }
        }
        Ok(Self(value.to_ascii_lowercase()))
    }

    /// The lowercase spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether an observed string is this GUID, ignoring ASCII case. For
    /// checking a Tally response against the GUID that was asked for; the
    /// observed string is never trusted to be well-formed.
    pub fn matches_observed(&self, other: &str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl fmt::Display for CompanyGuid {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
#[path = "company_guid_tests.rs"]
mod tests;
