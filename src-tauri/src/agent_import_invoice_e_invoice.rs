//! An invoice's e-invoice details (#2162 item 9): the IRN, acknowledgement number and
//! acknowledgement date that the e-invoice portal has already registered for it. ComplyEaze
//! Bridge generates none of them and sends nothing to the portal; it writes them into the
//! import as given and reads them back after the post.
//!
//! Tally keeps them as three direct children of the voucher, `IRN`, `IRNACKNO` and `IRNACKDATE`
//! (`YYYYMMDD`), and stores them as sent when an import carries them right after `ISINVOICE`
//! (reads-and-dates reference §8.2j; voucher-writes reference §9.16, row 2162-IRN9). Each value
//! is parsed here once, into a closed alphabet, so nothing a caller types can end the element
//! it is written into.

use super::super::xml_escape;
use crate::agent::failure_code::EInvoiceReadbackCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The three details, each parsed at the edge: an invoice either carries all three or none.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(in crate::agent) struct EInvoiceDetails {
    irn: Irn,
    ack_no: AckNumber,
    ack_date: AckDate,
}

/// The IRN: 64 lower-case hexadecimal characters, the form the portal issues.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
struct Irn(String);

impl TryFrom<String> for Irn {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() == 64
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
        .then_some(Self(value))
        .ok_or("the IRN must be 64 lower-case hexadecimal characters")
    }
}

impl From<Irn> for String {
    fn from(value: Irn) -> Self {
        value.0
    }
}

/// The acknowledgement number: 1 to 20 ASCII digits.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
struct AckNumber(String);

impl TryFrom<String> for AckNumber {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        ((1..=20).contains(&value.len()) && value.bytes().all(|byte| byte.is_ascii_digit()))
            .then_some(Self(value))
            .ok_or("the acknowledgement number must be 1 to 20 digits")
    }
}

impl From<AckNumber> for String {
    fn from(value: AckNumber) -> Self {
        value.0
    }
}

/// The acknowledgement date, a real calendar date given as `YYYY-MM-DD` and kept in that form;
/// Tally's own form, `YYYYMMDD`, is derived from it.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
struct AckDate(String);

impl AckDate {
    fn tally(&self) -> String {
        self.0.replace('-', "")
    }
}

impl TryFrom<String> for AckDate {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        let shaped = bytes.len() == 10
            && bytes.iter().enumerate().all(|(index, byte)| match index {
                4 | 7 => *byte == b'-',
                _ => byte.is_ascii_digit(),
            });
        (shaped && bridge_tally_core::TallyDate::parse(value.replace('-', "")).is_ok())
            .then_some(Self(value))
            .ok_or("the acknowledgement date must be a date written YYYY-MM-DD")
    }
}

impl From<AckDate> for String {
    fn from(value: AckDate) -> Self {
        value.0
    }
}

impl EInvoiceDetails {
    /// The three elements as the import writes them, right after `ISINVOICE`: the shape row
    /// 2162-IRN9 imported and Tally stored as sent. Each value is escaped once, although its
    /// alphabet holds nothing XML would read.
    pub(super) fn render(&self) -> String {
        format!(
            "<IRN>{}</IRN><IRNACKNO>{}</IRNACKNO><IRNACKDATE>{}</IRNACKDATE>",
            xml_escape(&self.irn.0),
            xml_escape(&self.ack_no.0),
            xml_escape(&self.ack_date.tally()),
        )
    }

    /// The approval text's two parts for the details: the IRN line (79 characters) and the
    /// acknowledgement (at most 49), which the dialog writes after the bill allocation.
    pub(in crate::agent) fn review_lines(&self) -> [String; 2] {
        [
            format!("E-invoice IRN: {}", self.irn.0),
            format!("E-invoice ack: {} of {}", self.ack_no.0, self.ack_date.0),
        ]
    }

    /// The bytes the approval digest binds, in a fixed order.
    pub(super) fn digest_fields(&self) -> [&[u8]; 3] {
        [
            self.irn.0.as_bytes(),
            self.ack_no.0.as_bytes(),
            self.ack_date.0.as_bytes(),
        ]
    }

    /// What the e-invoice read says of the posted invoice, found by the GUID its read-back
    /// returned: one difference per element whose text is not the one sent. An element Tally
    /// did not send, or sent empty, is a difference, never equal; no row with that GUID, or
    /// more than one, is a difference of its own.
    pub(super) fn readback_differences(&self, rows: &[Value], guid: &str) -> Vec<String> {
        let mut found = rows.iter().filter(|row| {
            row["guid"]
                .as_str()
                .is_some_and(|read| read.eq_ignore_ascii_case(guid))
        });
        let (Some(row), None) = (found.next(), found.next()) else {
            let code = if rows.iter().any(|row| {
                row["guid"]
                    .as_str()
                    .is_some_and(|read| read.eq_ignore_ascii_case(guid))
            }) {
                EInvoiceReadbackCode::NotUnique
            } else {
                EInvoiceReadbackCode::NotFound
            };
            return vec![code.as_str().to_string()];
        };
        let details = &row["e_invoice"];
        [
            ("IRN", "irn", self.irn.0.clone()),
            ("IRNACKNO", "ack_no", self.ack_no.0.clone()),
            ("IRNACKDATE", "ack_date", self.ack_date.tally()),
        ]
        .into_iter()
        .filter(|(_, key, sent)| {
            details[*key]["state"] != "present" || details[*key]["value"] != sent.as_str()
        })
        .map(|(element, _, _)| element.to_string())
        .collect()
    }
}

#[cfg(test)]
#[path = "agent_import_invoice_e_invoice_tests.rs"]
mod tests;
