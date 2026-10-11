//! One ledger `Create` message.
//!
//! What a live masters import captured on TallyPrime 7.1 shows (a company's
//! masters were created with it): `<NAME>` mirrors the `NAME` attribute,
//! `PARENT` and `ISBILLWISEON` are always present, `OPENINGBALANCE` follows
//! them only when the balance is non-zero, and no `TALLYMESSAGE` declares
//! `xmlns:UDF`. The capture carried no `PARTYGSTIN`, `TAXTYPE` or
//! `GSTDUTYHEAD`: their position after those elements is this renderer's
//! choice and UNVERIFIED as an order, although Tally has accepted Creates
//! carrying them (`docs/tally/TALLY_PROTOCOL_REFERENCE.md` §8.3, §9.4a).
//!
//! This renders the message only. Which fields a caller may set, and the
//! envelope that carries it, belong to the caller.
use crate::xml_text::escape_text;

/// The fields of one ledger `Create`, each emitted exactly as given and
/// escaped once here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedgerCreate<'a> {
    pub name: &'a str,
    pub parent: &'a str,
    pub is_billwise_on: bool,
    /// Omitted when `None`. The capture carries no `OPENINGBALANCE` on a
    /// ledger whose balance is zero.
    pub opening_balance: Option<&'a str>,
    pub party_gstin: Option<&'a str>,
    pub tax_type: Option<&'a str>,
    /// Emitted verbatim: this type does not check it. Tally sets a duty head
    /// at `Create` only, and silently drops a spelling outside its own
    /// vocabulary while reporting success (§8.3), so the caller checks it.
    pub gst_duty_head: Option<&'a str>,
}

impl LedgerCreate<'_> {
    /// The `TALLYMESSAGE` for this ledger.
    pub fn render_message(&self) -> String {
        let name = escape_text(self.name);
        let element = |tag: &str, value: Option<&str>| {
            value
                .map(|value| format!("<{tag}>{}</{tag}>", escape_text(value)))
                .unwrap_or_default()
        };
        format!(
            "<TALLYMESSAGE><LEDGER NAME=\"{name}\" ACTION=\"Create\"><NAME>{name}</NAME>\
<PARENT>{parent}</PARENT><ISBILLWISEON>{billwise}</ISBILLWISEON>{opening}{gstin}{tax_type}{duty_head}</LEDGER></TALLYMESSAGE>",
            parent = escape_text(self.parent),
            billwise = if self.is_billwise_on { "Yes" } else { "No" },
            opening = element("OPENINGBALANCE", self.opening_balance),
            gstin = element("PARTYGSTIN", self.party_gstin),
            tax_type = element("TAXTYPE", self.tax_type),
            duty_head = element("GSTDUTYHEAD", self.gst_duty_head),
        )
    }
}

#[cfg(test)]
#[path = "ledger_create_tests.rs"]
mod tests;
