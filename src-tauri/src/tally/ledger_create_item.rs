//! One ledger `Create` that Bridge renders from types, never from caller XML.
//!
//! The request is built from [`LedgerCreateItem`], whose parts are constructed
//! once and cannot be wrong afterwards (AGENTS.md P2, P3):
//!
//! - [`MasterName`] refuses a name Bridge has not measured Tally to match
//!   exactly. Non-ASCII is refused on purpose: a non-ASCII name was seen to
//!   read back as mojibake, and how Tally matches such a name is unmeasured.
//! - [`AdmittedLedgerKind`] has three kinds, a party, a suspense ledger and a
//!   GST duty ledger, and **no opening balance field**, no caller-chosen parent and no
//!   other kind. The parent is derived from the kind.
//! - [`AdmittedDutyHead`] is built only from a head in
//!   [`ADMITTED_DUTY_HEADS`], the heads whose `Create` has been measured as a
//!   sent value. Tally silently drops a spelling outside its vocabulary while
//!   reporting success (`docs/tally/TALLY_PROTOCOL_REFERENCE.md` §8.3), so an
//!   unmeasured head is refused here rather than sent.
//!
//! The master tools build it (`agent_master_tools.rs`); nothing else does.

use super::VerifiedCompanyIdentity;
use bridge_tally_core::master_binding::MAX_NAME_CHARS;
use bridge_tally_protocol::import_envelope::{render_import_envelope, ImportReport};
use bridge_tally_protocol::ledger_create::LedgerCreate;
use bridge_tally_protocol::{sha256_hex, GstDutyHead};

/// The duty heads a `Create` may carry. Only `SGST/UTGST` has been measured as
/// a sent value (a lab-created master read back with the head as sent); the
/// others wait for a lab read-back and are added here, one at a time, after it.
const ADMITTED_DUTY_HEADS: &[GstDutyHead] = &[GstDutyHead::SgstUtgst];

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

/// A GSTIN whose shape and check character both verify: two digits, five
/// capital letters, four digits, a capital letter, a letter or digit, `Z`, then
/// the check character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedGstin(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GstinError {
    Shape,
    CheckCharacter,
}

impl VerifiedGstin {
    pub(crate) fn new(value: &str) -> Result<Self, GstinError> {
        let bytes = value.as_bytes();
        let shaped = bytes.len() == 15
            && bytes[0..2].iter().all(u8::is_ascii_digit)
            && bytes[2..7].iter().all(u8::is_ascii_uppercase)
            && bytes[7..11].iter().all(u8::is_ascii_digit)
            && bytes[11].is_ascii_uppercase()
            && (bytes[12].is_ascii_uppercase() || bytes[12].is_ascii_digit())
            && bytes[13] == b'Z'
            && (bytes[14].is_ascii_uppercase() || bytes[14].is_ascii_digit());
        if !shaped {
            return Err(GstinError::Shape);
        }
        // The shape admits only fifteen ASCII bytes, so the split is safe.
        let (body, check) = value.split_at(14);
        if check.as_bytes()[0] != gstin_check_character(body.as_bytes()) {
            return Err(GstinError::CheckCharacter);
        }
        Ok(Self(value.to_string()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

/// The GSTIN check character: a mod-36 sum over the first fourteen characters,
/// each weighted 1 then 2 alternately, with the quotient and remainder of the
/// weighted value added. `body` is fourteen characters of `0-9A-Z`, which
/// the shape check in [`VerifiedGstin::new`] has already established.
fn gstin_check_character(body: &[u8]) -> u8 {
    const ALPHABET: &[u8; 36] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let sum: usize = body
        .iter()
        .enumerate()
        .map(|(index, byte)| {
            let value = ALPHABET.iter().position(|c| c == byte).unwrap_or(0);
            let weighted = value * if index % 2 == 0 { 1 } else { 2 };
            weighted / 36 + weighted % 36
        })
        .sum();
    ALPHABET[(36 - sum % 36) % 36]
}

/// A duty head that is in [`ADMITTED_DUTY_HEADS`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmittedDutyHead(GstDutyHead);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AdmittedDutyHeadError {
    DutyHeadNotAdmitted(GstDutyHead),
}

impl AdmittedDutyHead {
    pub(crate) fn new(head: GstDutyHead) -> Result<Self, AdmittedDutyHeadError> {
        if ADMITTED_DUTY_HEADS.contains(&head) {
            Ok(Self(head))
        } else {
            Err(AdmittedDutyHeadError::DutyHeadNotAdmitted(head))
        }
    }

    pub(crate) fn head(self) -> GstDutyHead {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PartySide {
    SundryDebtors,
    SundryCreditors,
}

/// The only ledger kinds Bridge will create. There is no opening balance, no
/// caller-chosen parent and no other kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AdmittedLedgerKind {
    Party {
        side: PartySide,
        gstin: Option<VerifiedGstin>,
        bill_wise: bool,
    },
    Suspense,
    GstDuty {
        head: AdmittedDutyHead,
    },
}

impl AdmittedLedgerKind {
    fn parent(&self) -> &'static str {
        match self {
            Self::Party {
                side: PartySide::SundryDebtors,
                ..
            } => "Sundry Debtors",
            Self::Party {
                side: PartySide::SundryCreditors,
                ..
            } => "Sundry Creditors",
            Self::Suspense => "Suspense A/c",
            Self::GstDuty { .. } => "Duties & Taxes",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LedgerCreateItem {
    name: MasterName,
    kind: AdmittedLedgerKind,
}

impl LedgerCreateItem {
    pub(crate) fn new(name: MasterName, kind: AdmittedLedgerKind) -> Self {
        Self { name, kind }
    }

    pub(crate) fn name(&self) -> &MasterName {
        &self.name
    }

    pub(crate) fn kind(&self) -> &AdmittedLedgerKind {
        &self.kind
    }

    pub(crate) fn parent(&self) -> &'static str {
        self.kind.parent()
    }

    /// The masters import request for the verified `company`, with its digest.
    pub(crate) fn render_import(&self, company: &VerifiedCompanyIdentity) -> RenderedLedgerCreate {
        let (is_billwise_on, party_gstin, tax_type, gst_duty_head) = match &self.kind {
            AdmittedLedgerKind::Party {
                gstin, bill_wise, ..
            } => (
                *bill_wise,
                gstin.as_ref().map(VerifiedGstin::as_str),
                None,
                None,
            ),
            AdmittedLedgerKind::Suspense => (false, None, None, None),
            AdmittedLedgerKind::GstDuty { head } => {
                (false, None, Some("GST"), Some(head.head().tally_spelling()))
            }
        };
        let message = LedgerCreate {
            name: self.name.as_str(),
            parent: self.parent(),
            is_billwise_on,
            opening_balance: None,
            party_gstin,
            tax_type,
            gst_duty_head,
        }
        .render_message();
        let xml =
            render_import_envelope(company.display_name(), ImportReport::AllMasters, &message);
        let sha256 = sha256_hex(xml.as_bytes());
        RenderedLedgerCreate { xml, sha256 }
    }
}

/// A rendered request and its digest.
///
/// `sha256` is over the UTF-8 bytes of the string Bridge renders here. The
/// dispatch later sends that string as UTF-16LE
/// (`encode_tally_xml_request_utf16le`), so the digest names the rendered
/// text, not the bytes on the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RenderedLedgerCreate {
    xml: String,
    sha256: String,
}

impl RenderedLedgerCreate {
    pub(crate) fn xml(&self) -> &str {
        &self.xml
    }

    /// Lowercase hex.
    pub(crate) fn sha256(&self) -> &str {
        &self.sha256
    }
}

#[cfg(test)]
#[path = "ledger_create_item_tests.rs"]
mod tests;
