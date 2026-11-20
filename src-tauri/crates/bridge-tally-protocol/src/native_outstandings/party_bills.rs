//! One party's open bills, read through Tally's `Ledger Outstandings` report
//! (#2377), and tied to that ledger's own closing balance before any figure is
//! told.
//!
//! The request, the answer's grammar and the tie were measured on TallyPrime 7.2
//! against one synthetic book: see the provenance note beside the captures in
//! `tests/fixtures/agent/party-bills/`.
//!
//! - **The request** takes a to-date only, the party as the text of the
//!   `LedgerName` static variable, and no fetch list. The name is admitted
//!   through [`LedgerScopeName`] and nothing wider: a name it refuses is never
//!   sent.
//! - **The answer** is the flat grammar of the Bills reports without a party
//!   per row: a `BILLFIXED` opens each bill, its `BILLOP`, `BILLCL`, `BILLDUE`
//!   and `BILLOVERDUE` follow as siblings, then zero or more groups of
//!   `BILLVCH*` elements naming the vouchers that made it. A debit bill is
//!   negative. An On Account row carries the reference text `On Account`.
//! - **An empty body is not an answer.** A name that matches no ledger was
//!   answered with HTTP 200, no bytes and no `Content-Type`, twice. The
//!   transport refuses that by its content type before this parser sees it; an
//!   empty text that did reach it is an unclosed envelope, never "no bills".
//! - **The tie.** Σ `BILLCL`, On Account included, equalled the ledger
//!   snapshot's closing at two to-dates, in the same sign. Only a tied answer
//!   becomes a [`PartyBillsRead`].

use bridge_tally_primitives::{ExactDecimal, TallyDate};
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::ledger_scope::LedgerScopeName;
use crate::xml_read_profiles::compatibility::static_variables_for_unvalidated_company;
use crate::xml_text::escape_text as xml_escape;

use super::date::{parse_native_bill_date, parse_native_due_date};

/// The reference text Tally gives an On Account row.
const ON_ACCOUNT_REFERENCE: &str = "On Account";

/// The `Ledger Outstandings` request for one party at one to-date.
pub fn render_ledger_outstandings_request(
    company: &str,
    party: &LedgerScopeName,
    to: &TallyDate,
) -> String {
    let static_variables = static_variables_for_unvalidated_company(
        company,
        "",
        &format!(
            r#"<LedgerName>{party}</LedgerName><SVTODATE TYPE="Date">{to}</SVTODATE><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><EXPLODEFLAG>Yes</EXPLODEFLAG>"#,
            party = xml_escape(party.as_str()),
            to = to.as_str(),
        ),
    );
    format!(
        r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>Ledger Outstandings</ID></HEADER><BODY><DESC>{static_variables}</DESC></BODY></ENVELOPE>"#
    )
}

/// What a bill row's reference is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartyBillReference {
    OnAccount,
    Bill(String),
}

/// One row of the answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyBill {
    pub reference: PartyBillReference,
    pub bill_date: TallyDate,
    /// The bill's balance at the to-date, as Tally signs it: a debit is negative.
    pub closing: ExactDecimal,
    /// `None` when Tally left `BILLDUE` empty, as it does on some On Account rows.
    pub due_date: Option<TallyDate>,
    /// `None` when Tally left `BILLOVERDUE` empty.
    pub overdue_days: Option<i64>,
}

/// An answer parsed, not yet tied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyBillsAnswer {
    bills: Vec<PartyBill>,
}

impl PartyBillsAnswer {
    pub fn rows(&self) -> &[PartyBill] {
        &self.bills
    }

    /// Ties the rows to the ledger's closing balance at the same to-date.
    ///
    /// `ledger_closing` is the snapshot's `CLOSINGBALANCE`; `None` is an empty
    /// element, which is not zero and refuses.
    pub fn tie(
        self,
        ledger_closing: Option<&ExactDecimal>,
    ) -> Result<PartyBillsRead, PartyBillsRefusal> {
        let ledger_closing = ledger_closing.ok_or(PartyBillsRefusal::LedgerClosingNotRead)?;
        let mut total = ExactDecimal::zero();
        for bill in &self.bills {
            total = total
                .checked_add(&bill.closing)
                .map_err(|_| PartyBillsRefusal::ArithmeticOverflow)?;
        }
        if !total.numeric_eq(ledger_closing) {
            return Err(PartyBillsRefusal::Untied {
                bills_total: total,
                ledger_closing: ledger_closing.clone(),
            });
        }
        // An envelope with no rows has not been seen, so it is not told as no
        // open bills (P1). Tally's answer with no bytes at all, which it sends a
        // party it reports nothing for, never reaches this parse: the runtime
        // tells it against the snapshot (protocol reference §12a.27).
        if self.bills.is_empty() {
            return Err(PartyBillsRefusal::NoRowsNotEstablished);
        }
        let (on_account, bills) = self
            .bills
            .into_iter()
            .partition(|bill| bill.reference == PartyBillReference::OnAccount);
        Ok(PartyBillsRead { bills, on_account })
    }
}

/// A party's open bills that tie to its ledger, with at least one row. Only
/// [`PartyBillsAnswer::tie`] makes one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyBillsRead {
    bills: Vec<PartyBill>,
    on_account: Vec<PartyBill>,
}

impl PartyBillsRead {
    /// The named bills.
    pub fn bills(&self) -> &[PartyBill] {
        &self.bills
    }

    /// The On Account rows.
    pub fn on_account(&self) -> &[PartyBill] {
        &self.on_account
    }
}

/// Why an answer that parsed is not told.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartyBillsRefusal {
    /// The rows do not sum to the ledger's closing. An envelope with no rows
    /// for a ledger that closes non-zero lands here.
    Untied {
        bills_total: ExactDecimal,
        ledger_closing: ExactDecimal,
    },
    /// The snapshot's closing balance was empty.
    LedgerClosingNotRead,
    ArithmeticOverflow,
    /// No row, against a ledger that closes at zero: the answer for a party
    /// with no open bills is not yet captured, so this one is not told.
    NoRowsNotEstablished,
}

impl PartyBillsRefusal {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Untied { .. } => "party_bills_untied",
            Self::LedgerClosingNotRead => "party_ledger_closing_not_read",
            Self::ArithmeticOverflow => "party_bills_overflow",
            Self::NoRowsNotEstablished => "party_bills_no_rows_not_established",
        }
    }
}

/// Why an answer did not parse. Each names the rule broken, never a party or a
/// reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartyBillsAnswerError {
    /// A `STATUS` element: as in the Bills reports, this report carries none
    /// on success.
    TallyReportedFailure,
    /// The answer broke the grammar; the code says which rule, and `row` is the
    /// 1-based row it broke in, when it broke inside one.
    Malformed {
        rule: &'static str,
        row: Option<usize>,
    },
}

impl PartyBillsAnswerError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::TallyReportedFailure => "party_bills_tally_failure",
            Self::Malformed { .. } => "party_bills_answer_malformed",
        }
    }
}

#[derive(Default)]
struct PendingBill {
    reference: Option<String>,
    bill_date: Option<String>,
    closing: Option<String>,
    due_date: Option<String>,
    overdue: Option<String>,
}

/// Parses a decoded answer. `books_from` and `as_of` resolve the two-digit
/// years of the bill dates, as for the Bills reports.
pub fn parse_ledger_outstandings_answer(
    xml: &str,
    books_from: &TallyDate,
    as_of: &TallyDate,
) -> Result<PartyBillsAnswer, PartyBillsAnswerError> {
    let malformed = |rule, row| PartyBillsAnswerError::Malformed { rule, row };
    if carries_status(xml) {
        return Err(PartyBillsAnswerError::TallyReportedFailure);
    }
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut envelope = Place::Before;
    let mut in_fixed = false;
    let mut pending: Vec<PendingBill> = Vec::new();
    loop {
        let event = reader
            .read_event()
            .map_err(|_| malformed("party_bills_xml_malformed", None))?;
        let row = pending.len();
        match event {
            Event::Start(element) => {
                let name = element.name().as_ref().as_bytes().to_ascii_uppercase();
                match envelope {
                    Place::Before if name == b"ENVELOPE" => {
                        envelope = Place::Inside;
                        continue;
                    }
                    Place::Before => return Err(malformed("party_bills_root_not_envelope", None)),
                    Place::After => return Err(malformed("party_bills_trailing_content", None)),
                    Place::Inside => {}
                }
                if name == b"STATUS" {
                    return Err(PartyBillsAnswerError::TallyReportedFailure);
                }
                if !in_fixed && name == b"BILLFIXED" {
                    pending.push(PendingBill::default());
                    in_fixed = true;
                    continue;
                }
                let known = if in_fixed {
                    matches!(name.as_slice(), b"BILLDATE" | b"BILLREF")
                } else {
                    is_row_scalar(&name) || is_voucher_scalar(&name)
                };
                if !known {
                    return Err(malformed("party_bills_unknown_element", Some(row)));
                }
                let raw = reader
                    .read_text(element.name())
                    .map_err(|_| malformed("party_bills_xml_malformed", Some(row)))?;
                let text = quick_xml::escape::unescape(&raw)
                    .map_err(|_| malformed("party_bills_invalid_escape", Some(row)))?
                    .trim()
                    .to_string();
                let bill = pending
                    .last_mut()
                    .ok_or(malformed("party_bills_scalar_before_fixed", None))?;
                let slot = match name.as_slice() {
                    b"BILLDATE" => Some(&mut bill.bill_date),
                    b"BILLREF" => Some(&mut bill.reference),
                    b"BILLCL" => Some(&mut bill.closing),
                    b"BILLDUE" => Some(&mut bill.due_date),
                    b"BILLOVERDUE" => Some(&mut bill.overdue),
                    // The opening and the vouchers that made the bill are not
                    // told; they are read only to be placed.
                    _ => None,
                };
                if let Some(slot) = slot {
                    if slot.replace(text).is_some() {
                        return Err(malformed("party_bills_duplicate_element", Some(row)));
                    }
                }
            }
            Event::End(element) => {
                let name = element.name().as_ref().as_bytes().to_ascii_uppercase();
                match (in_fixed, name.as_slice()) {
                    (true, b"BILLFIXED") => in_fixed = false,
                    (false, b"ENVELOPE") if envelope == Place::Inside => envelope = Place::After,
                    _ => return Err(malformed("party_bills_xml_malformed", Some(row))),
                }
            }
            // Tally writes an empty value as an open and a close tag; a
            // self-closed element has not been seen in this report.
            Event::Empty(element) => {
                if element.name().as_ref().eq_ignore_ascii_case("STATUS") {
                    return Err(PartyBillsAnswerError::TallyReportedFailure);
                }
                return Err(malformed("party_bills_empty_element", Some(row)));
            }
            Event::Eof => break,
            Event::Decl(_) | Event::Comment(_) => {}
            _ => return Err(malformed("party_bills_unexpected_content", Some(row))),
        }
    }
    if envelope != Place::After {
        return Err(malformed("party_bills_envelope_unclosed", None));
    }
    let bills = pending
        .into_iter()
        .enumerate()
        .map(|(index, row)| {
            finish_bill(row, books_from, as_of).map_err(|rule| malformed(rule, Some(index + 1)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PartyBillsAnswer { bills })
}

/// Whether a `STATUS` element stands anywhere in the answer, at any depth. As in
/// the Bills reports, this report carries none on success. An answer that does
/// not parse far enough to tell is left to the grammar below.
fn carries_status(xml: &str) -> bool {
    let mut reader = Reader::from_str(xml);
    loop {
        match reader.read_event() {
            Ok(Event::Start(element) | Event::Empty(element))
                if element.name().as_ref().eq_ignore_ascii_case("STATUS") =>
            {
                return true;
            }
            Ok(Event::Eof) | Err(_) => return false,
            Ok(_) => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    Before,
    Inside,
    After,
}

fn is_row_scalar(name: &[u8]) -> bool {
    matches!(name, b"BILLOP" | b"BILLCL" | b"BILLDUE" | b"BILLOVERDUE")
}

fn is_voucher_scalar(name: &[u8]) -> bool {
    matches!(
        name,
        b"BILLVCHDATE" | b"BILLVCHTYPE" | b"BILLVCHNUMBER" | b"BILLVCHAMOUNT" | b"BILLVCHQUALIFIER"
    )
}

fn finish_bill(
    row: PendingBill,
    books_from: &TallyDate,
    as_of: &TallyDate,
) -> Result<PartyBill, &'static str> {
    let reference = row.reference.ok_or("party_bills_reference_missing")?;
    if reference.is_empty() {
        return Err("party_bills_reference_empty");
    }
    let reference = if reference == ON_ACCOUNT_REFERENCE {
        PartyBillReference::OnAccount
    } else {
        PartyBillReference::Bill(reference)
    };
    let bill_date = row.bill_date.ok_or("party_bills_bill_date_missing")?;
    let bill_date = parse_native_bill_date(&bill_date, books_from, as_of)
        .map_err(|_| "party_bills_bill_date_invalid")?;
    // An empty closing is not zero (P3).
    let closing = row.closing.ok_or("party_bills_closing_missing")?;
    if closing.is_empty() {
        return Err("party_bills_closing_empty");
    }
    let closing = ExactDecimal::parse(closing).map_err(|_| "party_bills_closing_invalid")?;
    let due_date = match row.due_date.as_deref() {
        None => return Err("party_bills_due_date_missing"),
        Some("") => None,
        Some(raw) => Some(
            parse_native_due_date(raw, &bill_date).map_err(|_| "party_bills_due_date_invalid")?,
        ),
    };
    let overdue_days = match row.overdue.as_deref() {
        None => return Err("party_bills_overdue_missing"),
        Some("") => None,
        Some(raw) => Some(
            raw.parse::<i64>()
                .map_err(|_| "party_bills_overdue_invalid")?,
        ),
    };
    Ok(PartyBill {
        reference,
        bill_date,
        closing,
        due_date,
        overdue_days,
    })
}

#[cfg(test)]
#[path = "party_bills_tests.rs"]
mod tests;
