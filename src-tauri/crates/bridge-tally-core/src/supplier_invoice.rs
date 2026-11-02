//! A supplier's invoice as the books record it, and whether two of them are the same invoice
//! (#1904): the rule a check reads before a purchase is posted, so input tax credit is not
//! claimed twice. Pure: it reads no book. Which vouchers are compared, and how they are read, is
//! the caller's.
//!
//! The rules (CGST Rules r.46(b), r.36, r.54; the issue's design notes):
//! - A number is never refused for its form: an invoice number that does not meet rule 46(b),
//!   or a document of rule 54(2) to (4) that need not be serially numbered, still carries credit.
//!   [`SupplierInvoiceRef::fits_gstr1`] says only whether it could be a GSTR-1 number, so a
//!   caller can warn that a copy keyed with the supplier's own number would not be found.
//! - Rule 46(b) makes a number unique within a financial year. Two invoices of one supplier and
//!   one document type with the same number and both invoice dates in one financial year are the
//!   same invoice, whatever their dates ([`InvoiceMatch::Booked`]); in different financial years
//!   they are different invoices.
//! - A match that rests on a financial year taken from a voucher date (no invoice date in the
//!   books), on a document type that is not known, or on a loose form of the number, is never
//!   more than [`InvoiceMatch::PossiblyBooked`], which only a person may release. A loose form
//!   counts only with a second signal: the same invoice date or the same invoice value.
//! - A financial year written into a number (`2025-26`, `25/26`, `FY26`, a leading or trailing
//!   `2025`) is left out of the loose form and the serial, so a copy keyed without it, or with the
//!   year written another way, is still found.
//! - A number with no digit but zero (`NA`, `Cash`, `0`) is a placeholder, not a supplier's
//!   number: it is never [`InvoiceMatch::Booked`], and matches only as a loose form does.
//! - The characters folded (separators, characters that draw nothing, digits other than ASCII)
//!   are closed lists: Devanagari and full-width digits only, and the dashes and slashes named
//!   in `SEPARATORS`. Anything else is kept as it is, so a number differing only there is
//!   not found as the same.
use crate::{ExactDecimal, TallyDate};

/// The separators a supplier writes between the parts of a number, folded to one form: the ASCII
/// ones, and the dashes, minus sign and full-width forms a number copied from a PDF or a word
/// processor carries in their place.
const SEPARATORS: [char; 13] = [
    '-', '/', '\\', '.', '_', '\u{2010}', '\u{2011}', '\u{2012}', '\u{2013}', '\u{2014}',
    '\u{2212}', '\u{FF0D}', '\u{FF0F}',
];
/// Rule 46(b): at most sixteen characters.
const GSTR1_MAX_CHARS: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplierInvoiceRefError {
    /// Nothing but separators and whitespace: no number to compare.
    Empty,
}

/// A supplier invoice number with the forms a duplicate check compares. Built only by
/// [`SupplierInvoiceRef::parse`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplierInvoiceRef {
    raw: String,
    /// Upper case with `-` and `/` as typed; only what cannot be in a GSTR-1 number (whitespace,
    /// `.`, `_`, a backslash) is dropped. Two numbers the portal could hold apart differ here.
    strict: String,
    exact: String,
    loose: String,
    serial: Option<String>,
}

impl SupplierInvoiceRef {
    /// Upper case; every run of whitespace and separators folded to one `/`, none at either end.
    /// Refused only when nothing else is left.
    pub fn parse(raw: &str) -> Result<Self, SupplierInvoiceRefError> {
        let mut exact = String::new();
        let mut pending_separator = false;
        for ch in raw.chars().filter(|ch| !is_invisible(*ch)).map(ascii_digit) {
            if ch.is_whitespace() || SEPARATORS.contains(&ch) {
                pending_separator = !exact.is_empty();
                continue;
            }
            if pending_separator {
                exact.push('/');
                pending_separator = false;
            }
            exact.extend(ch.to_uppercase());
        }
        if exact.is_empty() {
            return Err(SupplierInvoiceRefError::Empty);
        }
        let core = without_year(&exact);
        let loose = loose_form(&core);
        let serial = serial_form(&core);
        let strict = raw
            .chars()
            .filter(|ch| !is_invisible(*ch))
            .map(ascii_digit)
            .filter_map(|ch| match ch {
                '-' | '\u{2010}'..='\u{2014}' | '\u{2212}' | '\u{FF0D}' => Some('-'),
                '/' | '\u{FF0F}' => Some('/'),
                ch if ch.is_whitespace() || SEPARATORS.contains(&ch) => None,
                ch => Some(ch),
            })
            .flat_map(char::to_uppercase)
            .collect();
        Ok(Self {
            raw: raw.to_string(),
            strict,
            exact,
            loose,
            serial,
        })
    }

    /// The number as the books hold it.
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// The form two numbers are the same invoice in.
    pub fn exact(&self) -> &str {
        &self.exact
    }

    /// Whether the number as keyed could be a GSTR-1 invoice number: at most sixteen characters,
    /// only letters, digits, `-` and `/` (rule 46(b)). Never a reason to refuse.
    pub fn fits_gstr1(&self) -> bool {
        let trimmed = self.raw.trim();
        trimmed.chars().count() <= GSTR1_MAX_CHARS
            && trimmed
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '/')
    }

    /// The same number without separators and leading zeros, or the same serial where one of the
    /// two is the serial alone (a copy keyed with only the supplier's running number). A serial
    /// shared by two full numbers is often the year (`001/2025-26`), so it is no match by itself.
    fn loose_equal(&self, other: &Self) -> bool {
        let serial_alone = self.is_digits_only() || other.is_digits_only();
        self.loose == other.loose
            || (serial_alone
                && matches!((&self.serial, &other.serial), (Some(left), Some(right)) if left == right))
    }

    fn is_digits_only(&self) -> bool {
        self.exact.chars().all(|ch| ch.is_ascii_digit())
    }

    /// No digit but zero: `NA`, `Cash`, `-0-` and the like stand in for a number.
    fn is_placeholder(&self) -> bool {
        !self
            .exact
            .chars()
            .any(|ch| ch.is_ascii_digit() && ch != '0')
    }
}

/// The exact form without the parts that name a financial year: `FY` with or without its year
/// (`FY26`, `FY25-26`), two runs of two or four digits a year apart (`2025/26`, `25/26`,
/// `2025/2026`), and a run of
/// four digits from 2000 to 2099 at either end. The exact form itself when that leaves no digit,
/// so `INV/2025` keeps its year as its number.
fn without_year(exact: &str) -> String {
    let parts: Vec<&str> = exact.split('/').collect();
    let year_digits = |part: &str| {
        (part.len() == 2 || part.len() == 4) && part.chars().all(|ch| ch.is_ascii_digit())
    };
    let a_year_apart = |first: &str, second: &str| {
        let first: u32 = first.parse().expect("digits");
        let second: u32 = second.parse().expect("digits");
        (first % 100 + 1) % 100 == second % 100
    };
    let mut year = vec![false; parts.len()];
    for (index, part) in parts.iter().enumerate() {
        let fy_year = part.strip_prefix("FY").filter(|digits| year_digits(digits));
        if *part == "FY" || fy_year.is_some() {
            year[index] = true;
        }
        if let Some(next) = parts.get(index + 1) {
            // `FY25-26`: the year after an `FY` part is part of it.
            let first = fy_year.or_else(|| year_digits(part).then_some(*part));
            if let Some(first) = first {
                if year_digits(next) && a_year_apart(first, next) {
                    year[index] = true;
                    year[index + 1] = true;
                }
            }
        }
        let at_end = index == 0 || index + 1 == parts.len();
        if at_end && part.len() == 4 && part.starts_with("20") && year_digits(part) {
            year[index] = true;
        }
    }
    let core = parts
        .iter()
        .zip(&year)
        .filter(|(_, is_year)| !**is_year)
        .map(|(part, _)| *part)
        .collect::<Vec<_>>()
        .join("/");
    if core.chars().any(|ch| ch.is_ascii_digit()) {
        core
    } else {
        exact.to_string()
    }
}

/// A character that draws nothing (a soft hyphen, a zero-width space or joiner, a byte-order
/// mark): dropped, so a copied number reads as it was printed.
fn is_invisible(ch: char) -> bool {
    matches!(
        ch,
        '\u{00AD}' | '\u{200B}'..='\u{200D}' | '\u{2060}' | '\u{FEFF}'
    )
}

/// A Devanagari or full-width digit as its ASCII digit; any other character as it is.
fn ascii_digit(ch: char) -> char {
    let zero = match ch {
        '\u{0966}'..='\u{096F}' => 0x0966,
        '\u{FF10}'..='\u{FF19}' => 0xFF10,
        _ => return ch,
    };
    char::from(b'0' + (u32::from(ch) - zero) as u8)
}

/// The exact form without separators, each run of digits without its leading zeros (`0` for a
/// run of zeros only). A separator ends a run, so `25-26/0001` folds to `25261`.
fn loose_form(exact: &str) -> String {
    let mut out = String::new();
    let mut digits = String::new();
    let flush = |digits: &mut String, out: &mut String| {
        if !digits.is_empty() {
            let trimmed = digits.trim_start_matches('0');
            out.push_str(if trimmed.is_empty() { "0" } else { trimmed });
            digits.clear();
        }
    };
    for ch in exact.chars() {
        if ch == '/' {
            flush(&mut digits, &mut out);
        } else if ch.is_ascii_digit() {
            digits.push(ch);
        } else {
            flush(&mut digits, &mut out);
            out.push(ch);
        }
    }
    flush(&mut digits, &mut out);
    out
}

/// The trailing run of digits without its leading zeros, `0` for a run of zeros only; none when
/// the number does not end in a digit.
fn serial_form(exact: &str) -> Option<String> {
    let digits = exact
        .chars()
        .rev()
        .take_while(char::is_ascii_digit)
        .collect::<Vec<_>>();
    if digits.is_empty() {
        return None;
    }
    let serial = digits.iter().rev().collect::<String>();
    let trimmed = serial.trim_start_matches('0');
    Some(if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    })
}

/// An Indian financial year, named by the calendar year it begins in (1 April).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FinancialYear(u16);

impl FinancialYear {
    pub fn of(date: &TallyDate) -> Self {
        let text = date.as_str();
        let year: u16 = text[0..4].parse().expect("a TallyDate holds YYYYMMDD");
        let month: u8 = text[4..6].parse().expect("a TallyDate holds YYYYMMDD");
        Self(if month >= 4 { year } else { year - 1 })
    }

    pub fn starting_year(self) -> u16 {
        self.0
    }
}

/// The document a supplier issued, as far as the books show it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplierDocument {
    Invoice,
    /// A supplier's debit note (s.34(3)): its own series may share numbers with the invoices.
    DebitNote,
}

/// A purchase about to be posted: its supplier invoice date is required (rule 46(c)); a purchase
/// without one is not a tax invoice and is refused before this type is built.
#[derive(Debug, Clone)]
pub struct IncomingInvoice {
    pub reference: SupplierInvoiceRef,
    pub invoice_date: TallyDate,
    pub document: SupplierDocument,
    pub value: Option<ExactDecimal>,
}

/// A voucher already in the books with the same supplier. Its invoice date is optional: without
/// one its financial year is taken from the voucher date, which makes any match only possible.
#[derive(Debug, Clone)]
pub struct BookedInvoice {
    pub reference: SupplierInvoiceRef,
    pub invoice_date: Option<TallyDate>,
    pub voucher_date: TallyDate,
    /// `None` when the books do not say which document it is.
    pub document: Option<SupplierDocument>,
    pub value: Option<ExactDecimal>,
}

impl BookedInvoice {
    /// The financial years the copy can belong to. From the invoice date, one. From the voucher
    /// date, also the year before when the voucher falls between April and November (an invoice
    /// of the previous year can still be booked until 30 November, s.16(4)).
    fn financial_years(&self) -> (Vec<FinancialYear>, bool) {
        match &self.invoice_date {
            Some(date) => (vec![FinancialYear::of(date)], false),
            None => {
                let year = FinancialYear::of(&self.voucher_date);
                let month: u8 = self.voucher_date.as_str()[4..6]
                    .parse()
                    .expect("a TallyDate holds YYYYMMDD");
                let mut years = vec![year];
                if (4..=11).contains(&month) && year.0 > 0 {
                    years.push(FinancialYear(year.0 - 1));
                }
                (years, true)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvoiceMatch {
    /// The same invoice: refused.
    Booked,
    /// Perhaps the same invoice: refused unless a person releases it, naming the copy.
    PossiblyBooked,
    /// A different invoice.
    Different,
}

/// Whether `booked` is `incoming` already in the books. Both are of the same supplier: the caller
/// compares only vouchers it tied to the supplier's registration.
pub fn compare(incoming: &IncomingInvoice, booked: &BookedInvoice) -> InvoiceMatch {
    // A supplier's debit note and invoice are different documents, whatever their numbers.
    if matches!(booked.document, Some(document) if document != incoming.document) {
        return InvoiceMatch::Different;
    }
    let year = FinancialYear::of(&incoming.invoice_date);
    let (years, from_voucher_date) = booked.financial_years();
    if !years.contains(&year) {
        return InvoiceMatch::Different;
    }
    // A placeholder in both is no supplier number to be the same on: only a loose match.
    if incoming.reference.exact != booked.reference.exact || incoming.reference.is_placeholder() {
        let second_signal = booked.invoice_date.as_ref() == Some(&incoming.invoice_date)
            || matches!((&incoming.value, &booked.value),
                (Some(left), Some(right))
                    if !left.is_zero() && left.magnitude().numeric_eq(&right.magnitude()));
        return if incoming.reference.loose_equal(&booked.reference) && second_signal {
            InvoiceMatch::PossiblyBooked
        } else {
            InvoiceMatch::Different
        };
    }
    // Booked only on the number as the portal could hold it, the invoice's own year and a known
    // document; a match that folding `-` and `/` together made, or that rests on a voucher date
    // or an unknown document, is only possible.
    if booked.document.is_some()
        && !from_voucher_date
        && incoming.reference.strict == booked.reference.strict
    {
        InvoiceMatch::Booked
    } else {
        InvoiceMatch::PossiblyBooked
    }
}

#[cfg(test)]
#[path = "supplier_invoice_tests.rs"]
mod tests;
