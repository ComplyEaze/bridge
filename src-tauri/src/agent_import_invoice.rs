//! GST invoices on the import path. Sales first; Purchase takes the same core.
//!
//! An invoice is not a Journal with a name: it is an invoice view whose party
//! leg carries a bill allocation, and Tally files it into the sales register
//! and the GST returns by what the voucher and its ledgers say. Everything
//! here is parsed at the edge into one of two states: the input a caller may
//! send (`InvoiceDetail` without `observed`) and the state the build saved
//! (`observed` filled from Tally). A caller can never supply `observed`.
//!
//! The pure parts live here and touch no Tally: structural validation, the
//! GSTIN check, the classification of each leg from what the masters say
//! (never from a ledger's name: duty heads, reserved groups), the arithmetic,
//! and the rendering. The reads that fill `LedgerFacts` belong to the build.

use super::{xml_escape, EntrySide, ImportEntry, ImportVoucher};
use crate::tally::approved_import::{InvoiceAnswers, InvoiceReadPlan};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[path = "agent_import_invoice_wire.rs"]
mod wire;

/// The request builders the sealed read profiles wrap.
pub(in crate::agent) use wire::{
    render_company_registration_request, render_invoice_number_request,
    render_invoice_readback_request, render_ledger_rates_request, render_voucher_types_request,
};

/// What an invoice voucher carries besides its entries.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct InvoiceDetail {
    /// The voucher type the invoice is filed under, by its display name in
    /// this book. A book can hold several types of one class (a client
    /// sample book on the lab has Sales and Sales Acc), so Bridge never picks one: the
    /// caller names it and the build checks the name against Tally.
    pub(super) voucher_type_name: String,
    /// The state the supply is made in, as Tally names it.
    pub(super) place_of_supply: String,
    /// The ledger carrying a round off, when the invoice total is rounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) round_off_ledger: Option<String>,
    /// Filled by the build from what Tally returned; refused when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) observed: Option<InvoiceObserved>,
}

/// What the build observed in Tally for this invoice, saved so a post
/// re-renders and re-checks from the saved batch alone.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct InvoiceObserved {
    /// The GUID of the named voucher type, as the build read it.
    pub(super) voucher_type_guid: String,
    /// The party's GSTIN in force on the invoice date; none for an
    /// unregistered customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) party_gstin: Option<String>,
    /// The state written on the voucher: the GSTIN's state for a registered
    /// customer, the place of supply for an unregistered one.
    pub(super) party_state: String,
    /// What the voucher says of the party's registration: `Regular` or
    /// `Unregistered/Consumer`.
    pub(super) party_registration_type: String,
    /// Whether the party ledger is bill-wise; a New Ref is written only then.
    pub(super) party_bill_wise: bool,
    /// The company's own state, which the supply must be made in (v0 posts
    /// only the intra-state CGST plus state-tax shape).
    pub(super) company_state: String,
}

/// The tax head a ledger carries, as the compliance read reports it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DutyHead {
    Cgst,
    /// `state_tax` and `sgst_utgst` are two spellings of the state head.
    State,
    Igst,
    Cess,
    /// Any head this build does not take (a union territory tax, say).
    Other,
    NotTax,
}

/// What the ledger's dated GST registration says on the invoice date. Each
/// state is positive evidence of one kind; absence of an answer is its own
/// state and never reads as "unregistered".
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum GstinEvidence {
    /// A GSTIN in force on the date.
    InForce(String),
    /// A registration entry in force on the date that names no GSTIN.
    NoneInForce,
    /// The ledger has dated registration entries and none applies on the date
    /// (the first starts later): nothing is known of that day.
    NoEntryInForce,
    /// Only the flat field names a GSTIN: no dated entry says what kind of
    /// registration it is.
    FlatFieldOnly,
    /// The history was unreadable, or the flat field and the history name
    /// different GSTINs: neither is chosen.
    Unsettled,
    /// Tally returned nothing about registration (no history, no flat field).
    NotReported,
}

/// What the masters say about one ledger an invoice names.
#[derive(Clone, Debug)]
pub(super) struct LedgerFacts {
    /// Reserved names of the ledger's groups, nearest first, only when the
    /// chain resolved to the root. Empty when it did not.
    pub(super) reserved_groups: Vec<String>,
    pub(super) duty_head: DutyHead,
    pub(super) gstin: GstinEvidence,
    pub(super) registration_type: Option<String>,
    /// `None` when the bill-wise read did not return this ledger.
    pub(super) bill_wise: Option<bool>,
    /// What the rate listing says of this ledger; `None` when it did not
    /// return the ledger.
    pub(super) rates: Option<wire::LedgerRateRow>,
}

impl LedgerFacts {
    fn under(&self, group: &str) -> bool {
        self.reserved_groups
            .iter()
            .any(|reserved| normalize(reserved) == normalize(group))
    }
}

fn normalize(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// How each entry of a Sales invoice was classified.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct InvoiceRoles {
    pub(super) party: usize,
    pub(super) sales: Vec<usize>,
    pub(super) cgst: usize,
    pub(super) state_tax: usize,
    pub(super) round_off: Option<usize>,
}

/// Why an invoice was refused: one stable code, and the ledger or value it
/// concerns, so a caller fixes every defect at once.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct InvoiceRefusal {
    pub(super) code: &'static str,
    pub(super) detail: RefusalDetail,
}

/// What a refusal concerns. A ledger's name identifies a party, so it is its
/// own case and is written through the party marker: refusal details follow
/// the masking setting like every other ledger name this tool returns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum RefusalDetail {
    Nothing,
    Ledger(String),
    /// A count, a state, a voucher type or an invoice number: never a ledger.
    Value(String),
}

impl InvoiceRefusal {
    /// The refusal as a caller reads it: its code, what it concerns, and the
    /// next step where the code has one (the invoice-number codes name GST
    /// rule 46(b)).
    pub(super) fn to_json(&self) -> serde_json::Value {
        let detail = match &self.detail {
            RefusalDetail::Nothing => serde_json::json!(""),
            RefusalDetail::Ledger(name) => {
                serde_json::json!(super::super::party_name(name.as_str()))
            }
            RefusalDetail::Value(text) => serde_json::json!(text),
        };
        let mut value = serde_json::json!({"code": self.code, "detail": detail});
        if let Some(remediation) = super::super::refusal_remediation(self.code) {
            value["remediation"] = serde_json::json!(remediation);
        }
        value
    }
}

fn refuse(code: &'static str) -> InvoiceRefusal {
    InvoiceRefusal {
        code,
        detail: RefusalDetail::Nothing,
    }
}

fn refuse_ledger(code: &'static str, ledger: &str) -> InvoiceRefusal {
    InvoiceRefusal {
        code,
        detail: RefusalDetail::Ledger(ledger.to_string()),
    }
}

fn refuse_value(code: &'static str, value: impl Into<String>) -> InvoiceRefusal {
    InvoiceRefusal {
        code,
        detail: RefusalDetail::Value(value.into()),
    }
}

/// GST state codes and the names Tally uses for them.
const STATES: &[(&str, &str)] = &[
    ("01", "Jammu & Kashmir"),
    ("02", "Himachal Pradesh"),
    ("03", "Punjab"),
    ("04", "Chandigarh"),
    ("05", "Uttarakhand"),
    ("06", "Haryana"),
    ("07", "Delhi"),
    ("08", "Rajasthan"),
    ("09", "Uttar Pradesh"),
    ("10", "Bihar"),
    ("11", "Sikkim"),
    ("12", "Arunachal Pradesh"),
    ("13", "Nagaland"),
    ("14", "Manipur"),
    ("15", "Mizoram"),
    ("16", "Tripura"),
    ("17", "Meghalaya"),
    ("18", "Assam"),
    ("19", "West Bengal"),
    ("20", "Jharkhand"),
    ("21", "Odisha"),
    ("22", "Chhattisgarh"),
    ("23", "Madhya Pradesh"),
    ("24", "Gujarat"),
    ("26", "Dadra & Nagar Haveli and Daman & Diu"),
    ("27", "Maharashtra"),
    ("29", "Karnataka"),
    ("30", "Goa"),
    ("31", "Lakshadweep"),
    ("32", "Kerala"),
    ("33", "Tamil Nadu"),
    ("34", "Puducherry"),
    ("35", "Andaman & Nicobar Islands"),
    ("36", "Telangana"),
    ("37", "Andhra Pradesh"),
    ("38", "Ladakh"),
];

pub(super) fn state_name_for_code(code: &str) -> Option<&'static str> {
    STATES
        .iter()
        .find(|(known, _)| *known == code)
        .map(|(_, name)| *name)
}

pub(super) fn is_state_name(name: &str) -> bool {
    STATES.iter().any(|(_, known)| *known == name)
}

const GSTIN_ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Format and check digit of a GSTIN (the public mod-36 scheme).
pub(super) fn gstin_valid(gstin: &str) -> bool {
    let bytes = gstin.as_bytes();
    if bytes.len() != 15
        || !bytes
            .iter()
            .all(|b| b.is_ascii_digit() || b.is_ascii_uppercase())
    {
        return false;
    }
    let digit = |i: usize| bytes[i].is_ascii_digit();
    let upper = |i: usize| bytes[i].is_ascii_uppercase();
    if !(digit(0)
        && digit(1)
        && (2..7).all(upper)
        && (7..11).all(digit)
        && upper(11)
        && bytes[12] != b'0'
        && bytes[13] == b'Z')
    {
        return false;
    }
    let mut total = 0usize;
    for (i, ch) in gstin[..14].chars().enumerate() {
        let Some(value) = GSTIN_ALPHABET.find(ch) else {
            return false;
        };
        let product = value * if i % 2 == 0 { 1 } else { 2 };
        total += product / 36 + product % 36;
    }
    let expected = (36 - total % 36) % 36;
    GSTIN_ALPHABET.as_bytes()[expected] == bytes[14]
}

pub(super) fn gstin_state(gstin: &str) -> Option<&'static str> {
    state_name_for_code(gstin.get(..2)?)
}

/// Paise of a plain two-decimal amount (`valid_2dp_amount` has run).
fn paise(amount: &str) -> Option<i128> {
    let (whole, fraction) = amount.split_once('.')?;
    if fraction.len() != 2 {
        return None;
    }
    let whole: i128 = whole.parse().ok()?;
    let fraction: i128 = fraction.parse().ok()?;
    whole.checked_mul(100)?.checked_add(fraction)
}

/// The largest company master mark (an upper bound on its ledgers) a v0
/// invoice is built or posted on. Each invoice reads the whole compliance
/// listing three times; above this the read is refused rather than risk a
/// large book's gateway. A small firm's sample book on the lab sits near 1,600.
const INVOICE_MAX_MASTER_MARK: u64 = 5_000;

/// The largest voucher mark (an upper bound on the book's vouchers) a v0
/// invoice is built or posted on. PROVISIONAL: formula-filtered voucher reads
/// have been timed in seconds on books of a few thousand vouchers, and this
/// read on none; the lab run that times it on a larger book moves the bound.
const INVOICE_MAX_VOUCHER_MARK: u64 = 25_000;

/// Why a book is refused on its voucher mark, or `None` when the mark admits
/// it. A mark Tally did not give never admits.
fn voucher_mark_refusal(mark: Option<u64>) -> Option<InvoiceRefusal> {
    match mark {
        Some(vouchers) if vouchers <= INVOICE_MAX_VOUCHER_MARK => None,
        Some(vouchers) => Some(refuse_value(
            "invoice_book_too_many_vouchers",
            format!("voucher mark {vouchers}"),
        )),
        None => Some(refuse("invoice_book_size_unknown")),
    }
}

/// Whether the master mark alone admits a book.
fn mark_admits(mark: u64) -> bool {
    mark <= INVOICE_MAX_MASTER_MARK
}

/// What a book above that mark may hold, by the company's own ledger count,
/// and still be built on: the whole compliance listing is a few kilobytes a
/// ledger, so this keeps one read of it to a few megabytes. The listing's own
/// size gate stays in front of it; a follow-up scopes the read to the named
/// ledgers' parent groups so large books can be admitted.
const INVOICE_MAX_LEDGERS: u64 = 2_000;

/// Why a book over the mark is refused on the company's own ledger count, or
/// `None` when the count admits it. A count Tally did not give never admits,
/// and is refused as unknown, not as large.
fn ledger_count_refusal(ledgers: Option<u64>, mark: u64) -> Option<InvoiceRefusal> {
    match ledgers {
        Some(ledgers) if ledgers <= INVOICE_MAX_LEDGERS => None,
        Some(ledgers) => Some(refuse_value(
            "invoice_book_too_large",
            format!("{ledgers} ledgers"),
        )),
        // Not known to be large: the count did not come back, and the mark alone
        // does not admit the book.
        None => Some(refuse_value(
            "invoice_book_size_unknown",
            format!("master mark {mark}"),
        )),
    }
}

/// The most legs an invoice may carry: the customer, up to three Sales Accounts
/// ledgers, the two tax heads (or two sales ledgers, the heads and a round off).
/// What the approval window can show, with its blank line dropped when nothing
/// stands under the entries.
const MAX_INVOICE_ENTRIES: usize = 6;

/// The IGST rates GST has, in thousandths of a percent: the slab list the check
/// admitted before it read ledgers (5, 12, 18, 28 and 40 percent). A ledger at
/// any other rate (a mistyped one, or 3 or 0.25 percent) is refused, though the
/// invoice and the ledger agree: the lab measured 5 percent and one exact 18
/// percent bill only, so the list bounds which rates are worked out and says
/// nothing of how Tally treats the rest.
const SLAB_IGST_MILLI: &[i128] = &[5_000, 12_000, 18_000, 28_000, 40_000];

/// The most sales lines an invoice may carry. Tax was measured on one and two
/// lines (the lab, 10 Oct 2026); a third waits for a measured three-line bill.
const MAX_SALES_LINES: usize = 2;

/// A rate in thousandths of a percent: at most three decimals, as Tally writes
/// a rate (`"2.50"`, `"5"`, after the text is cleaned). `None` for anything
/// else, a zero or a negative.
fn rate_milli(text: &str) -> Option<i128> {
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    if whole.is_empty()
        || fraction.len() > 3
        || !whole.bytes().all(|byte| byte.is_ascii_digit())
        || !fraction.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    let fraction = format!("{fraction:0<3}");
    let milli = whole
        .parse::<i128>()
        .ok()?
        .checked_mul(1000)?
        .checked_add(fraction.parse::<i128>().ok()?)?;
    (milli > 0).then_some(milli)
}

/// The state tax rate of a sales ledger in force on `as_of` (`YYYYMMDD`), in
/// thousandths of a percent, from the rows the rate listing returned. Only
/// what the lab measured is read: one dated row, in force, ledger-specified
/// and taxable, made of nothing but the elements the lab's rows carried, one
/// all-states block that gives CGST, SGST/UTGST and IGST rates valued on value
/// (the state heads equal, IGST twice one of them) and no cess. Each other
/// shape is its own refusal, never a guess.
fn sales_rate_milli(row: &wire::LedgerRateRow, as_of: &str) -> Result<i128, &'static str> {
    let [gst] = row.gst_rows.as_slice() else {
        return Err(match row.gst_rows.len() {
            0 => "invoice_sales_ledger_rate_unknown",
            _ => "invoice_sales_ledger_rate_history_unmeasured",
        });
    };
    if !gst.applicable_from.as_deref().is_some_and(|from| {
        from.len() == 8 && from.bytes().all(|byte| byte.is_ascii_digit()) && from <= as_of
    }) {
        return Err("invoice_sales_ledger_rate_not_in_force");
    }
    if gst.unmeasured {
        return Err("invoice_sales_ledger_rate_shape_unmeasured");
    }
    if gst.taxability.as_deref() != Some("Taxable") {
        return Err("invoice_sales_ledger_not_taxable");
    }
    if gst.source.as_deref() != Some("Specify Details Here") {
        return Err("invoice_sales_ledger_rate_not_ledger_specified");
    }
    let [block] = gst.states.as_slice() else {
        return Err("invoice_sales_ledger_rate_state_wise_unmeasured");
    };
    if block.state.as_deref() != Some("Any") {
        return Err("invoice_sales_ledger_rate_state_wise_unmeasured");
    }
    let unreadable = "invoice_sales_ledger_rate_heads_unreadable";
    let mut cgst = None;
    let mut state = None;
    let mut igst = None;
    for head in &block.heads {
        match head.head.as_str() {
            "CGST" | "SGST/UTGST" | "IGST" => {
                if head.valuation.as_deref() != Some("Based on Value") {
                    return Err(unreadable);
                }
                let milli = head
                    .rate
                    .as_deref()
                    .and_then(rate_milli)
                    .ok_or(unreadable)?;
                let slot = match head.head.as_str() {
                    "CGST" => &mut cgst,
                    "SGST/UTGST" => &mut state,
                    _ => &mut igst,
                };
                if slot.replace(milli).is_some() {
                    return Err(unreadable);
                }
            }
            // No cess at all, as the lab's rows read: Cess "Not Applicable"
            // and State Cess "Based on Value", neither with a rate. A head
            // with a rate, or valued any other way, is a tax this build does
            // not work out.
            "Cess"
                if head.rate.is_none() && head.valuation.as_deref() == Some("Not Applicable") => {}
            "State Cess"
                if head.rate.is_none() && head.valuation.as_deref() == Some("Based on Value") => {}
            "Cess" | "State Cess" => return Err("invoice_cess_rate_not_supported"),
            _ => return Err(unreadable),
        }
    }
    let (Some(cgst), Some(state), Some(igst)) = (cgst, state, igst) else {
        return Err(unreadable);
    };
    if cgst != state {
        return Err("invoice_sales_ledger_rate_heads_unequal");
    }
    if cgst.checked_mul(2) != Some(igst) {
        return Err("invoice_sales_ledger_rate_igst_not_twice_state");
    }
    if !SLAB_IGST_MILLI.contains(&igst) {
        return Err("invoice_sales_ledger_rate_not_a_slab");
    }
    Ok(cgst)
}

/// What a tax ledger must say for the invoice to be checked against its rate:
/// its own rate of tax is the one the sales ledgers give, and it rounds
/// nothing (the measured ledgers read "Not Applicable" and 0). A ledger that
/// reads anything else, or nothing, was not measured.
fn tax_ledger_refusal(row: &wire::LedgerRateRow, half_milli: i128) -> Option<&'static str> {
    let own = row.rate_of_tax_calculation.as_deref().and_then(rate_milli);
    if own != Some(half_milli) {
        return Some("invoice_tax_ledger_rate_mismatch");
    }
    if row.rounding_method.as_deref() != Some("Not Applicable")
        || row.rounding_limit.as_deref() != Some("0")
    {
        return Some("invoice_tax_ledger_rounding_unsupported");
    }
    None
}

/// The tax Tally's GSTR-1 expects on the invoice for each state head: the sum
/// over the sales lines of the line's amount at the head's rate, rounded
/// half-up to the paisa, line by line (the lab, 10 Oct 2026: sixteen vouchers,
/// "included" exactly when both heads equal it). An error code when a figure
/// does not fit, and when a line's tax is not a whole paisa at a rate other
/// than 2.5 percent: the lab measured the rounding of inexact figures only at
/// 2.5 percent (the other rates were exact), so elsewhere only exact lines are
/// admitted.
fn expected_tax_paise(lines: &[i128], half_milli: i128) -> Result<i128, &'static str> {
    let mut total = 0_i128;
    for line in lines {
        let scaled = line
            .checked_mul(half_milli)
            .ok_or("invoice_amount_invalid")?;
        if scaled % 100_000 != 0 && half_milli != 2_500 {
            return Err("invoice_tax_rounding_unmeasured");
        }
        let tax = scaled.checked_add(50_000).ok_or("invoice_amount_invalid")? / 100_000;
        total = total.checked_add(tax).ok_or("invoice_amount_invalid")?;
    }
    Ok(total)
}

/// A paise figure as Tally writes it: `25.03`.
fn paise_text(paise: i128) -> String {
    format!("{}.{:02}", paise / 100, paise % 100)
}

/// The alphabet of an invoice number, as GST rule 46(b) allows it: at most 16
/// characters, letters and digits and the two characters hyphen and slash. A
/// number outside it is rejected by the GSTR-1 upload, so it is refused here,
/// and it is also a closed alphabet for the TDL literal the duplicate check
/// puts it in.
pub(super) fn invoice_number_safe(number: &str) -> bool {
    !number.is_empty()
        && number.chars().count() <= 16
        && number
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '-'))
}

/// What Tally returned is the build's to record, never the caller's: refused
/// on the way in. A saved batch carries it (and `admit_saved` requires it), so
/// this runs only on a caller's payload.
pub(super) fn refuse_supplied_observed(vouchers: &[ImportVoucher]) -> Result<(), String> {
    if vouchers.iter().any(|voucher| {
        voucher
            .invoice
            .as_ref()
            .is_some_and(|detail| detail.observed.is_some())
    }) {
        return Err("invoice_observed_not_input".to_string());
    }
    Ok(())
}

/// An amendment alters a voucher in place: it replaces the entries and drops
/// their allocations, and the amendment check compares neither an invoice's
/// party nor its reference nor its bill name. So an invoice is never amended.
pub(super) fn refuse_invoice_amendment(vouchers: &[ImportVoucher]) -> Result<(), String> {
    if vouchers
        .iter()
        .any(|voucher| voucher.voucher_type.is_invoice())
    {
        return Err("invoice_amendment_not_supported".to_string());
    }
    Ok(())
}

/// Structural validation of a Sales invoice, before any read. Everything that
/// needs a master waits for `classify_sales_invoice`.
pub(super) fn validate_invoice_voucher(voucher: &ImportVoucher) -> Result<(), String> {
    if !voucher.voucher_type.is_invoice() {
        // A non-invoice voucher must not carry invoice detail.
        return match voucher.invoice {
            None => Ok(()),
            Some(_) => Err("invoice_detail_on_non_invoice".to_string()),
        };
    }
    let Some(detail) = voucher.invoice.as_ref() else {
        return Err("invoice_detail_required".to_string());
    };
    let plain = |text: &str, max: usize| {
        !text.trim().is_empty()
            && text.chars().count() <= max
            && !text.chars().any(char::is_control)
            && text == text.trim()
    };
    if !plain(&detail.voucher_type_name, super::MAX_MASTER_NAME_CHARS)
        || super::reads_back_as_other_text(&detail.voucher_type_name)
    {
        return Err("invoice_voucher_type_name_invalid".to_string());
    }
    if !is_state_name(&detail.place_of_supply) {
        return Err("invoice_place_of_supply_unknown".to_string());
    }
    // The invoice number is the document number GSTR-1 reports, in the
    // alphabet GST rule 46(b) allows and no other (a space is refused). It
    // goes into a TDL string literal for the duplicate check, so a closed
    // alphabet, not a deny-list.
    let Some(number) = voucher.voucher_number.as_deref() else {
        return Err("invoice_number_required".to_string());
    };
    if !invoice_number_safe(number) {
        return Err("invoice_number_invalid".to_string());
    }
    // The supplier reference is a purchase notion; a Sales invoice has none.
    if voucher.reference.is_some() {
        return Err("invoice_reference_not_for_sales".to_string());
    }
    let round_off = detail.round_off_ledger.as_deref();
    if let Some(name) = round_off {
        if !plain(name, super::MAX_MASTER_NAME_CHARS)
            || !voucher.entries.iter().any(|entry| entry.ledger == name)
        {
            return Err("invoice_round_off_ledger_invalid".to_string());
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    if !voucher
        .entries
        .iter()
        .all(|entry| seen.insert(entry.ledger.as_str()))
    {
        return Err("invoice_ledger_repeated".to_string());
    }
    let counted = voucher
        .entries
        .iter()
        .filter(|entry| Some(entry.ledger.as_str()) != round_off);
    let debits = counted.clone().filter(|e| e.side == EntrySide::Dr).count();
    let credits = counted.filter(|e| e.side == EntrySide::Cr).count();
    // One party debit; the sales legs and one tax leg per head are credits.
    if debits != 1 || credits < 3 {
        return Err("invoice_entry_shape_invalid".to_string());
    }
    // The approval window holds this many legs and no more (24 lines, 1,600
    // characters); an invoice that would not fit is refused here, before any read.
    if voucher.entries.len() > MAX_INVOICE_ENTRIES {
        return Err("invoice_too_many_entries".to_string());
    }
    Ok(())
}

/// Classify every entry of a Sales invoice from what the masters say, and
/// check the arithmetic. `facts` is keyed by ledger name.
pub(super) fn classify_sales_invoice(
    voucher: &ImportVoucher,
    facts: &BTreeMap<String, LedgerFacts>,
    company_state: &str,
    as_of: &str,
) -> Result<InvoiceRoles, Vec<InvoiceRefusal>> {
    let Some(detail) = voucher.invoice.as_ref() else {
        return Err(vec![refuse("invoice_detail_required")]);
    };
    let round_off_name = detail.round_off_ledger.as_deref();
    let mut refusals = Vec::new();
    let mut party = None;
    let mut sales = Vec::new();
    let mut cgst = Vec::new();
    let mut state_tax = Vec::new();
    let mut round_off = None;
    for (index, entry) in voucher.entries.iter().enumerate() {
        let Some(fact) = facts.get(&entry.ledger) else {
            refusals.push(refuse_ledger("invoice_ledger_not_observed", &entry.ledger));
            continue;
        };
        if fact.reserved_groups.is_empty() {
            refusals.push(refuse_ledger(
                "invoice_ledger_group_unresolved",
                &entry.ledger,
            ));
            continue;
        }
        if Some(entry.ledger.as_str()) == round_off_name {
            if !(fact.under("Indirect Expenses") || fact.under("Indirect Incomes")) {
                refusals.push(refuse_ledger(
                    "invoice_round_off_ledger_group",
                    &entry.ledger,
                ));
            }
            round_off = Some(index);
        } else if fact.under("Sundry Debtors") {
            if entry.side != EntrySide::Dr {
                refusals.push(refuse_ledger("invoice_party_must_be_debit", &entry.ledger));
            }
            if party.replace(index).is_some() {
                refusals.push(refuse_ledger("invoice_more_than_one_party", &entry.ledger));
            }
        } else if fact.under("Sales Accounts") {
            if entry.side != EntrySide::Cr {
                refusals.push(refuse_ledger("invoice_sales_must_be_credit", &entry.ledger));
            }
            sales.push(index);
        } else if fact.under("Duties & Taxes") {
            if entry.side != EntrySide::Cr {
                refusals.push(refuse_ledger("invoice_tax_must_be_credit", &entry.ledger));
            }
            match fact.duty_head {
                DutyHead::Cgst => cgst.push(index),
                DutyHead::State => state_tax.push(index),
                DutyHead::Igst => {
                    refusals.push(refuse_ledger("invoice_igst_not_supported", &entry.ledger))
                }
                DutyHead::Cess => {
                    refusals.push(refuse_ledger("invoice_cess_not_supported", &entry.ledger))
                }
                DutyHead::Other | DutyHead::NotTax => refusals.push(refuse_ledger(
                    "invoice_tax_ledger_head_unknown",
                    &entry.ledger,
                )),
            }
        } else {
            refusals.push(refuse_ledger("invoice_ledger_role_unknown", &entry.ledger));
        }
    }
    let one = |found: &[usize], code: &'static str, refusals: &mut Vec<InvoiceRefusal>| {
        if found.len() == 1 {
            Some(found[0])
        } else {
            refusals.push(refuse_value(code, format!("{} found", found.len())));
            None
        }
    };
    if sales.is_empty() {
        refusals.push(refuse_value("invoice_needs_a_sales_ledger", "0 found"));
    }
    let cgst_index = one(
        &cgst,
        "invoice_needs_exactly_one_cgst_ledger",
        &mut refusals,
    );
    let state_index = one(
        &state_tax,
        "invoice_needs_exactly_one_state_tax_ledger",
        &mut refusals,
    );
    if party.is_none() {
        refusals.push(refuse("invoice_party_missing"));
    }
    // The shape posted is CGST plus state tax, so the supply must be made in
    // the company's own state. A registered customer's GSTIN must agree.
    if detail.place_of_supply != company_state {
        refusals.push(refuse_value(
            "invoice_place_of_supply_not_company_state",
            &detail.place_of_supply,
        ));
    }
    if let Some(party_index) = party {
        let name = &voucher.entries[party_index].ledger;
        if let Some(fact) = facts.get(name) {
            // Each state is refused under its own code. A type Tally did not
            // report is never read as Regular or as unregistered.
            let type_not_reported = "invoice_party_registration_type_not_reported";
            match (&fact.gstin, fact.registration_type.as_deref()) {
                (GstinEvidence::Unsettled, _) => {
                    refusals.push(refuse_ledger("invoice_party_gstin_unsettled", name))
                }
                (GstinEvidence::NotReported, _) => refusals.push(refuse_ledger(
                    "invoice_party_registration_not_reported",
                    name,
                )),
                (GstinEvidence::NoEntryInForce, _) => refusals.push(refuse_ledger(
                    "invoice_party_registration_not_in_force",
                    name,
                )),
                (GstinEvidence::FlatFieldOnly, _) => {
                    refusals.push(refuse_ledger(type_not_reported, name))
                }
                (GstinEvidence::InForce(gstin), _) if !gstin_valid(gstin) => {
                    refusals.push(refuse_ledger("invoice_party_gstin_invalid", name))
                }
                (GstinEvidence::InForce(gstin), kind) => {
                    if gstin_state(gstin) != Some(detail.place_of_supply.as_str()) {
                        refusals.push(refuse_ledger(
                            "invoice_place_of_supply_not_party_state",
                            name,
                        ));
                    }
                    match kind {
                        Some("Regular") => {}
                        None => refusals.push(refuse_ledger(type_not_reported, name)),
                        Some(_) => refusals.push(refuse_ledger("invoice_party_not_regular", name)),
                    }
                }
                // An entry in force that names no GSTIN: an unregistered
                // customer only when the entry says so. One that says it is
                // registered has its number missing.
                (GstinEvidence::NoneInForce, Some("Unregistered/Consumer" | "Unregistered")) => {}
                (GstinEvidence::NoneInForce, None) => {
                    refusals.push(refuse_ledger(type_not_reported, name))
                }
                (GstinEvidence::NoneInForce, Some(_)) => {
                    refusals.push(refuse_ledger("invoice_party_gstin_missing", name))
                }
            }
            // A New Ref is written exactly when the party is bill-wise and never
            // otherwise (an allocation on a non-bill-wise ledger is dropped
            // silently). So the flag must be known, but either value admits.
            if fact.bill_wise.is_none() {
                refusals.push(refuse_ledger("invoice_party_bill_wise_unknown", name));
            }
        }
    }
    if !refusals.is_empty() {
        return Err(refusals);
    }
    let (party, sales, cgst, state_tax) = (
        party.expect("checked"),
        sales,
        cgst_index.expect("checked"),
        state_index.expect("checked"),
    );
    let amount = |index: usize| paise(&voucher.entries[index].amount);
    let lines = sales
        .iter()
        .map(|index| amount(*index))
        .collect::<Option<Vec<_>>>();
    let (Some(party_amount), Some(lines), Some(cgst_amount), Some(state_amount)) =
        (amount(party), lines, amount(cgst), amount(state_tax))
    else {
        return Err(vec![refuse("invoice_amount_invalid")]);
    };
    let Some(taxable) = lines
        .iter()
        .try_fold(0_i128, |sum, line| sum.checked_add(*line))
    else {
        return Err(vec![refuse("invoice_amount_invalid")]);
    };
    // The tax Tally's GSTR-1 expects is worked out per sales line from the
    // sales ledger's own rate, never from the tax typed on the invoice (the
    // lab, 10 Oct 2026). Every doubt about the rate is a refusal.
    let mut tax_refusals = Vec::new();
    if sales.len() > MAX_SALES_LINES {
        tax_refusals.push(refuse_value(
            "invoice_too_many_sales_lines",
            format!("{} sales lines, at most {MAX_SALES_LINES}", sales.len()),
        ));
    }
    let mut rates = std::collections::BTreeSet::new();
    for index in &sales {
        let name = &voucher.entries[*index].ledger;
        match facts.get(name).and_then(|fact| fact.rates.as_ref()) {
            None => tax_refusals.push(refuse_ledger("invoice_sales_ledger_rate_unknown", name)),
            Some(row) => match sales_rate_milli(row, as_of) {
                Ok(milli) => {
                    rates.insert(milli);
                }
                Err(code) => tax_refusals.push(refuse_ledger(code, name)),
            },
        }
    }
    if !tax_refusals.is_empty() {
        return Err(tax_refusals);
    }
    let mut rates = rates.into_iter();
    let (Some(half_milli), None) = (rates.next(), rates.next()) else {
        return Err(vec![refuse("invoice_sales_ledgers_rates_differ")]);
    };
    for index in [cgst, state_tax] {
        let name = &voucher.entries[index].ledger;
        let code = match facts.get(name).and_then(|fact| fact.rates.as_ref()) {
            None => Some("invoice_tax_ledger_rate_mismatch"),
            Some(row) => tax_ledger_refusal(row, half_milli),
        };
        if let Some(code) = code {
            tax_refusals.push(refuse_ledger(code, name));
        }
    }
    if !tax_refusals.is_empty() {
        return Err(tax_refusals);
    }
    let expected = expected_tax_paise(&lines, half_milli).map_err(|code| vec![refuse(code)])?;
    for (label, entered) in [("CGST", cgst_amount), ("State tax", state_amount)] {
        if entered != expected {
            tax_refusals.push(refuse_value(
                "invoice_tax_head_not_expected",
                format!(
                    "{label} {} expected {}",
                    paise_text(entered),
                    paise_text(expected)
                ),
            ));
        }
    }
    if !tax_refusals.is_empty() {
        return Err(tax_refusals);
    }
    let Some(base) = taxable
        .checked_add(cgst_amount)
        .and_then(|sum| sum.checked_add(state_amount))
    else {
        return Err(vec![refuse("invoice_amount_invalid")]);
    };
    let round = match round_off {
        None => 0,
        Some(index) => {
            let Some(value) = amount(index) else {
                return Err(vec![refuse("invoice_amount_invalid")]);
            };
            // A credit round off raises what the customer owes, a debit lowers it.
            match voucher.entries[index].side {
                EntrySide::Cr => value,
                EntrySide::Dr => -value,
            }
        }
    };
    if round.abs() >= 100 {
        return Err(vec![refuse("invoice_round_off_too_large")]);
    }
    if base.checked_add(round) != Some(party_amount) {
        return Err(vec![refuse("invoice_party_amount_does_not_close")]);
    }
    Ok(InvoiceRoles {
        party,
        sales,
        cgst,
        state_tax,
        round_off,
    })
}

/// Why a build admitted no invoice: Bridge refused it, or a read failed.
pub(super) enum InvoiceAdmission {
    Refused(Vec<InvoiceRefusal>),
    Failed(super::super::ToolFailure),
}

impl From<super::super::ToolFailure> for InvoiceAdmission {
    fn from(failure: super::super::ToolFailure) -> Self {
        Self::Failed(failure)
    }
}

fn refused(refusal: InvoiceRefusal) -> InvoiceAdmission {
    InvoiceAdmission::Refused(vec![refusal])
}

fn failed(code: &'static str) -> InvoiceAdmission {
    failed_with(code.to_string())
}

fn failed_with(code: String) -> InvoiceAdmission {
    InvoiceAdmission::Failed(super::super::ToolFailure::from(code))
}

impl super::super::Server {
    /// Read the invoice's masters again. The invoice must be admitted again
    /// (every leg's group and duty head, the type's series, the number still
    /// unused: a refusal there comes back under its own code), and what the
    /// build recorded must be unchanged (`import_invoice_masters_changed`, its
    /// `cause` naming the first field that differs): the
    /// party's GSTIN, state, registration and bill-wise flag, the type's GUID
    /// and the company's state (its registration's, ADR 0004 slice 1). Run
    /// before the approval
    /// dialog and again, after it is answered, before the post is dispatched,
    /// because a dialog can stay open and a book can change under it. The post
    /// reads them once more in the endpoint queue (#1337).
    /// Also returns the plan of the reads it made, for the queue to make again
    /// (#1337).
    pub(super) async fn recheck_sales_invoice(
        &self,
        identity: &super::super::VerifiedCompanyIdentity,
        company: &bridge_tally_protocol::TallyCompany,
        saved: &ImportVoucher,
        catalogue: &bridge_tally_protocol::StandardLedgerCatalogV2,
    ) -> Result<(super::super::Evidence, InvoiceReadPlan), super::super::ToolFailure> {
        let mut fresh = saved.clone();
        let recorded = fresh
            .invoice
            .as_mut()
            .and_then(|detail| detail.observed.take());
        // Boxed, with the other large futures of this path: the post's own
        // future holds this one twice, and a thread's stack is small (#1337).
        let (evidence, plan) = match Box::pin(
            self.admit_sales_invoice_planned(identity, company, &mut fresh, catalogue),
        )
        .await
        {
            Ok(admitted) => admitted,
            Err(InvoiceAdmission::Failed(failure)) => return Err(failure),
            // The refusal's own code is the answer: a number now in use is not
            // "a master changed", and a rebuild would only be refused again.
            Err(InvoiceAdmission::Refused(refusals)) => {
                return Err(match refusals.first() {
                    Some(first) => super::super::ToolFailure::from(first.code.to_string()),
                    None => masters_changed(NOT_RECORDED),
                });
            }
        };
        let now = fresh
            .invoice
            .as_ref()
            .and_then(|detail| detail.observed.as_ref());
        match first_difference(recorded.as_ref(), now) {
            None => Ok((evidence, plan)),
            Some(field) => Err(masters_changed(field)),
        }
    }
}

impl super::super::Server {
    /// The number control the journal offers now for the invoice of `line`, or
    /// none for a batch with no invoice. Read inside the endpoint's dispatch
    /// lease, for the queue's judge (#1337).
    pub(super) fn queued_invoice_control(
        &self,
        line: &super::ImportLedgerLine,
    ) -> Result<Option<super::ledger::NumberControl>, String> {
        let Some(voucher) = line
            .vouchers
            .iter()
            .find(|voucher| voucher.voucher_type.is_invoice())
        else {
            return Ok(None);
        };
        let as_of = super::super::normalized_date(&voucher.date)
            .map_err(|_| "invoice_date_invalid".to_string())?;
        let year = financial_year_window(as_of.as_str())
            .ok_or_else(|| "invoice_date_invalid".to_string())?;
        self.import_invoice_number_control(&line.company_guid, (&year.0, &year.1))
            .map(Some)
    }
}

/// Whether the queue's reads and the batch agree on there being an invoice: a
/// batch with none carries no reads and no control, and a batch with exactly
/// one invoice carries its reads and its control. Anything else is a wiring
/// fault, refused before any request is sent and never skipped (#1337).
pub(super) fn queued_invoice_for<'a>(
    vouchers: &'a [ImportVoucher],
    invoice: Option<crate::tally::approved_import::QueuedInvoice<'a>>,
    control: Option<&'a super::ledger::NumberControl>,
) -> Result<
    Option<(
        &'a ImportVoucher,
        crate::tally::approved_import::QueuedInvoice<'a>,
        &'a super::ledger::NumberControl,
    )>,
    crate::tally::approved_import::ApprovedImportAdmissionError,
> {
    let invoiced = vouchers
        .iter()
        .any(|voucher| voucher.voucher_type.is_invoice());
    match (invoiced, invoice, control, vouchers) {
        (false, None, None, _) => Ok(None),
        (true, Some(queued), Some(control), [voucher]) => Ok(Some((voucher, queued, control))),
        _ => {
            Err(crate::tally::approved_import::ApprovedImportAdmissionError::AdmissionInconsistent)
        }
    }
}

/// Whether the plan the queue sent is the one this invoice calls for. The
/// requests are a function of the invoice alone, apart from the number's
/// control, which is the journal's: a plan that differs in any other request is
/// a wiring fault, and a control that moved since the plan is a change. The
/// ledger count is not compared: whether it is needed depends on the marks, and
/// the judge asks for it.
pub(super) fn fit_plan(
    wanted: &InvoiceReadPlan,
    carried: &InvoiceReadPlan,
) -> Result<(), crate::tally::approved_import::ApprovedImportAdmissionError> {
    use crate::tally::approved_import::ApprovedImportAdmissionError as Refusal;
    let InvoiceReadPlan {
        listing_as_of: _,
        ledger_count: _,
        rates,
        voucher_types,
        number,
        number_control,
        registration,
    } = wanted;
    if *rates != carried.rates
        || *voucher_types != carried.voucher_types
        || *number != carried.number
        || *registration != carried.registration
    {
        return Err(Refusal::AdmissionInconsistent);
    }
    if *number_control != carried.number_control {
        return Err(Refusal::InvoiceMastersChanged {
            field: "number_control",
        });
    }
    Ok(())
}

/// An invoice's masters, read again in the endpoint queue, judged by the rule
/// the build judged them by and held to what the build recorded (#1337).
///
/// Refused, each before any intent: a plan that is not the one this invoice
/// calls for (a wiring fault); a number control that moved since the plan; a
/// book that outgrew the size gate since the build; the judge's own refusal;
/// and an observation that differs from the recorded one, naming the first
/// field that does.
pub(super) fn admit_queued_invoice(
    voucher: &ImportVoucher,
    company_name: &str,
    company_guid: &str,
    catalogue: &bridge_tally_protocol::StandardLedgerCatalogV2,
    control: &super::ledger::NumberControl,
    queued: &crate::tally::approved_import::QueuedInvoice<'_>,
) -> Result<(), crate::tally::approved_import::ApprovedImportAdmissionError> {
    use crate::tally::approved_import::ApprovedImportAdmissionError as Refusal;
    let mut fresh = voucher.clone();
    let recorded = fresh
        .invoice
        .as_mut()
        .and_then(|detail| detail.observed.take());
    let judge = InvoiceJudge {
        company_name,
        company_guid,
        voucher: &fresh,
        catalogue,
        control,
    };
    let refused = |verdict: Verdict| match verdict {
        Verdict::Refused(refusals) => match refusals.first() {
            Some(first) => Refusal::InvoiceRefused {
                code: first.code.to_string(),
            },
            None => Refusal::InvoiceMastersChanged {
                field: NOT_RECORDED,
            },
        },
        Verdict::Failed(code) => Refusal::InvoiceRefused { code },
        // A request the plan does not carry is a plan that does not fit.
        Verdict::Need(_) => Refusal::AdmissionInconsistent,
    };
    // What the queue read must be what this invoice's admission reads: the
    // requests are a function of the invoice alone, apart from the control,
    // which is the journal's.
    let wanted = judge
        .plan(
            queued.plan.listing_as_of.clone(),
            queued.plan.ledger_count.is_some(),
        )
        .map_err(refused)?;
    fit_plan(&wanted, queued.plan)?;
    match judge.judge(queued.answers) {
        Ok(observed) => match first_difference(recorded.as_ref(), Some(&observed)) {
            None => Ok(()),
            Some(field) => Err(Refusal::InvoiceMastersChanged { field }),
        },
        // The book outgrew the size gate while the approval waited: the count
        // the gate now needs was not part of the plan.
        Err(Verdict::Need(Need::LedgerCount)) => {
            Err(Refusal::InvoiceMastersChanged { field: "book_size" })
        }
        Err(verdict) => Err(refused(verdict)),
    }
}

/// The refusal for an invoice whose masters moved, naming the first field that
/// differs as its typed cause.
fn masters_changed(field: &'static str) -> super::super::ToolFailure {
    let mut failure = super::super::ToolFailure::from("import_invoice_masters_changed".to_string());
    failure.cause = Some(field);
    failure
}

/// What `first_difference` names when the build recorded no observation, or
/// the re-read made none.
pub(super) const NOT_RECORDED: &str = "observation";

/// The first field in which what the re-read observed differs from what the
/// build recorded, or none when they are equal whole. An invoice with no
/// observation recorded is never "unchanged". The recorded fields are
/// destructured, so a field added to the observation cannot be left out.
pub(super) fn first_difference(
    recorded: Option<&InvoiceObserved>,
    now: Option<&InvoiceObserved>,
) -> Option<&'static str> {
    let (Some(recorded), Some(now)) = (recorded, now) else {
        return Some(NOT_RECORDED);
    };
    let InvoiceObserved {
        voucher_type_guid,
        party_gstin,
        party_state,
        party_registration_type,
        party_bill_wise,
        company_state,
    } = recorded;
    if *voucher_type_guid != now.voucher_type_guid {
        Some("voucher_type")
    } else if *party_gstin != now.party_gstin {
        Some("party_gstin")
    } else if *party_state != now.party_state {
        Some("party_state")
    } else if *party_registration_type != now.party_registration_type {
        Some("party_registration")
    } else if *party_bill_wise != now.party_bill_wise {
        Some("party_bill_wise")
    } else if *company_state != now.company_state {
        Some("company_state")
    } else {
        None
    }
}

/// Every field of an invoice's detail, one length-prefixed value each, for the
/// approval digest: written out field by field, never serde output, so a
/// serializer change cannot move a digest and a new field cannot be left out
/// (the destructurings are exhaustive).
pub(super) fn encode_detail(field: &mut dyn FnMut(&[u8]), detail: &InvoiceDetail) {
    let InvoiceDetail {
        voucher_type_name,
        place_of_supply,
        round_off_ledger,
        observed,
    } = detail;
    field(voucher_type_name.as_bytes());
    field(place_of_supply.as_bytes());
    match round_off_ledger {
        Some(ledger) => {
            field(b"1");
            field(ledger.as_bytes());
        }
        None => field(b"0"),
    }
    match observed {
        None => field(b"0"),
        Some(InvoiceObserved {
            voucher_type_guid,
            party_gstin,
            party_state,
            party_registration_type,
            party_bill_wise,
            company_state,
        }) => {
            field(b"1");
            field(voucher_type_guid.as_bytes());
            match party_gstin {
                Some(gstin) => {
                    field(b"1");
                    field(gstin.as_bytes());
                }
                None => field(b"0"),
            }
            field(party_state.as_bytes());
            field(party_registration_type.as_bytes());
            field(if *party_bill_wise { b"1" } else { b"0" });
            field(company_state.as_bytes());
        }
    }
}

/// A signed two-decimal amount as read (`-11200.00`, `373175.00`) in paise.
fn signed_paise(amount: &str) -> Option<i128> {
    let (negative, digits) = match amount.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, amount),
    };
    let value = paise(digits)?;
    Some(if negative { -value } else { value })
}

/// Every way a posted invoice reads back differently from what the saved
/// batch carries: the voucher-level fields, each leg, the bill allocation.
/// The codes name fields, never values.
fn invoice_readback_differences(
    expected: &ImportVoucher,
    read: &wire::ReadInvoice,
    date: &str,
) -> Vec<String> {
    let mut diffs = Vec::new();
    let (Some(detail), Some(observed)) = (
        expected.invoice.as_ref(),
        expected
            .invoice
            .as_ref()
            .and_then(|detail| detail.observed.as_ref()),
    ) else {
        return vec!["invoice_not_observed".to_string()];
    };
    let number = expected.voucher_number.as_deref().unwrap_or_default();
    let party = party_entry(expected)
        .map(|entry| entry.ledger.as_str())
        .unwrap_or_default();
    let want = |field: &str, value: Option<&str>, diffs: &mut Vec<String>| {
        if read
            .fields
            .get(field)
            .map(String::as_str)
            .filter(|value| !value.is_empty())
            != value
        {
            diffs.push(field.to_string());
        }
    };
    want("DATE", Some(date), &mut diffs);
    want("VOUCHERNUMBER", Some(number), &mut diffs);
    want("REFERENCE", Some(number), &mut diffs);
    want("REFERENCEDATE", Some(date), &mut diffs);
    want(
        "VOUCHERTYPENAME",
        Some(&detail.voucher_type_name),
        &mut diffs,
    );
    want("PARTYLEDGERNAME", Some(party), &mut diffs);
    want("PARTYGSTIN", observed.party_gstin.as_deref(), &mut diffs);
    want("STATENAME", Some(&observed.party_state), &mut diffs);
    want("PLACEOFSUPPLY", Some(&detail.place_of_supply), &mut diffs);
    want(
        "GSTREGISTRATIONTYPE",
        Some(&observed.party_registration_type),
        &mut diffs,
    );
    want("ISINVOICE", Some("Yes"), &mut diffs);
    want("ISCANCELLED", Some("No"), &mut diffs);
    want("ISOPTIONAL", Some("No"), &mut diffs);
    // Legs: the same multiset of (ledger, signed amount, side).
    let key = |ledger: &str, amount: i128, positive: bool| (ledger.to_string(), amount, positive);
    let mut expected_legs = expected
        .entries
        .iter()
        .map(|entry| {
            let value = paise(&entry.amount).unwrap_or(i128::MIN);
            match entry.side {
                EntrySide::Dr => key(&entry.ledger, -value, true),
                EntrySide::Cr => key(&entry.ledger, value, false),
            }
        })
        .collect::<Vec<_>>();
    let mut read_legs = Vec::new();
    for leg in &read.legs {
        match (signed_paise(&leg.amount), leg.deemed_positive.as_deref()) {
            (Some(amount), Some("Yes")) => read_legs.push(key(&leg.ledger, amount, true)),
            (Some(amount), Some("No")) => read_legs.push(key(&leg.ledger, amount, false)),
            _ => {
                diffs.push("legs_unreadable".to_string());
                read_legs.clear();
                break;
            }
        }
    }
    expected_legs.sort();
    read_legs.sort();
    if !diffs.iter().any(|d| d == "legs_unreadable") && expected_legs != read_legs {
        diffs.push("legs".to_string());
    }
    // Bill allocation: exactly one New Ref named by the number on the party leg
    // of a bill-wise party, for the party leg's amount; none anywhere otherwise.
    let party_amount = expected
        .entries
        .iter()
        .find(|entry| entry.ledger == party)
        .and_then(|entry| paise(&entry.amount))
        .map(|value| -value);
    let allocation_ok = read.legs.iter().all(|leg| {
        if leg.ledger == party && observed.party_bill_wise {
            matches!(
                leg.allocations.as_slice(),
                [(Some(name), Some(kind), amount)]
                    if name == number && kind == "New Ref" && signed_paise(amount) == party_amount
            )
        } else {
            leg.allocations.is_empty()
        }
    });
    if !allocation_ok {
        diffs.push("bill_allocation".to_string());
    }
    diffs
}

impl super::super::Server {
    /// Read the posted invoice back by its type and number over its financial
    /// year and compare every field the build wrote. The fields the standard
    /// readback never sees: party, GST header, reference, allocation. An absent
    /// voucher, an unreadable answer and a repeated one are all differences,
    /// never a pass. The read walks the book's vouchers, so the voucher mark is
    /// read first and bounds it, as it bounds the build: a book that has grown
    /// past the bound since the post is a difference, not a read.
    pub(super) async fn read_back_sales_invoice(
        &self,
        identity: &super::super::VerifiedCompanyIdentity,
        company: &bridge_tally_protocol::TallyCompany,
        voucher: &ImportVoucher,
    ) -> Result<
        (
            Vec<String>,
            Option<String>,
            Option<String>,
            super::super::Evidence,
        ),
        super::super::ToolFailure,
    > {
        let Some(observed) = voucher
            .invoice
            .as_ref()
            .and_then(|detail| detail.observed.as_ref())
        else {
            return Ok((
                vec!["invoice_not_observed".to_string()],
                None,
                None,
                super::local_evidence("invoice_readback"),
            ));
        };
        // Once a post is in the book, nothing here is an error that would hide
        // the post's own verification: each failure is a difference.
        let only =
            |difference: &str, evidence| Ok((vec![difference.to_string()], None, None, evidence));
        let local = || super::local_evidence("invoice_readback");
        let Ok((mark, mark_evidence)) = self.pre_import_mark(company, identity).await else {
            return only("invoice_readback_failed", local());
        };
        if voucher_mark_refusal(mark.value).is_some() {
            return only("invoice_readback_book_too_large", mark_evidence);
        }
        let number = voucher.voucher_number.clone().unwrap_or_default();
        let window = super::super::normalized_date(&voucher.date)
            .ok()
            .map(|date| date.as_str().to_string())
            .and_then(|date| financial_year_window(&date).map(|year| (date, year)));
        let request = window.as_ref().and_then(|(_, year)| {
            super::super::invoice_readback_read(
                &company.name,
                &observed.voucher_type_guid,
                &number,
                (&year.0, &year.1),
            )
        });
        let (Some((date, _)), Some(request)) = (window.as_ref(), request) else {
            return only("invoice_readback_failed", mark_evidence);
        };
        let Ok((xml, evidence)) = self.post_read(identity, request).await else {
            return only("invoice_readback_failed", mark_evidence);
        };
        let evidence = super::super::combine_evidence(mark_evidence, evidence);
        let (differences, alter_id, guid) =
            readback_outcome(wire::parse_invoice_readback(&xml), voucher, date);
        Ok((differences, alter_id, guid, evidence))
    }
}

/// What a read-back answer means for the posted invoice: its differences from
/// the saved batch, and the AlterID and GUID of the one voucher found.
fn readback_outcome(
    parsed: Result<wire::Readback, &'static str>,
    voucher: &ImportVoucher,
    date: &str,
) -> (Vec<String>, Option<String>, Option<String>) {
    let only = |difference: &str| (vec![difference.to_string()], None, None);
    match parsed {
        // Two vouchers of this type and number: the number is not unique,
        // which a post must surface, never hide behind a generic error.
        Ok(wire::Readback::Several) => only("invoice_number_not_unique"),
        Ok(wire::Readback::Absent) => only("invoice_not_found"),
        Err(_) => only("invoice_readback_unreadable"),
        Ok(wire::Readback::One(read)) => (
            invoice_readback_differences(voucher, &read, date),
            read.fields.get("ALTERID").cloned(),
            read.fields.get("GUID").cloned(),
        ),
    }
}

/// The voucher an invoice read-back found by type and number must be the one
/// the standard readback attributed to this batch: its GUID and its AlterID
/// must both come back, on both reads, and agree. A value that did not come
/// back is a difference, never a pass.
pub(super) fn readback_identity_differences(
    guid: Option<&str>,
    alter_id: Option<&str>,
    matched: &serde_json::Value,
) -> Vec<String> {
    let mut differences = Vec::new();
    match (guid, matched["guid"].as_str()) {
        (Some(read), Some(attributed)) if read.eq_ignore_ascii_case(attributed) => {}
        _ => differences.push("guid".to_string()),
    }
    match (
        alter_id.and_then(|read| read.trim().parse::<u64>().ok()),
        matched["alter_id"].as_u64(),
    ) {
        (Some(read), Some(attributed)) if read == attributed => {}
        _ => differences.push("alter_id".to_string()),
    }
    differences
}

/// What a ledger's GST registration says on a date, and the type its entry in
/// force names. "No GSTIN in force" from the ledger read covers two things, so
/// they are told apart here: an entry in force that names no GSTIN, and a
/// history whose first entry starts after the date, which says nothing of that
/// day and is never read as unregistered.
pub(super) fn gstin_evidence(
    flat: Option<&str>,
    history: &bridge_tally_protocol::gst_registration::GstRegistrationHistory,
    as_of: &str,
) -> (GstinEvidence, Option<String>) {
    let read = super::super::ledgers::party_gstin_on(flat, history, as_of);
    let evidence = if read.sources_disagree || read.status == "history_unreadable" {
        GstinEvidence::Unsettled
    } else {
        match (read.gstin, read.status) {
            (Some(gstin), "in_force") => GstinEvidence::InForce(gstin),
            (Some(_), "flat_field") => GstinEvidence::FlatFieldOnly,
            (None, "no_gstin_in_force") if history.in_force(as_of).is_some() => {
                GstinEvidence::NoneInForce
            }
            (None, "no_gstin_in_force") => GstinEvidence::NoEntryInForce,
            _ => GstinEvidence::NotReported,
        }
    };
    (evidence, read.registration_type)
}

/// A ledger's bill-wise flag as the V2 ledger catalogue read it: `None` when
/// the catalogue holds the name zero times or more than once.
pub(super) fn party_bill_wise_in(
    catalogue: &bridge_tally_protocol::StandardLedgerCatalogV2,
    ledger: &str,
) -> Option<bool> {
    let mut found = catalogue
        .bill_wise_flags()
        .filter(|(name, _, _)| *name == ledger)
        .map(|(_, _, flag)| flag == bridge_tally_protocol::BillWiseFlag::On);
    let first = found.next()?;
    found.next().is_none().then_some(first)
}

/// The party ledger of an invoice that will carry a New Ref. Its entry is
/// written with a bill allocation, so it does not land On Account and needs
/// no On Account approval (#1234); every other leg is judged as before.
pub(super) fn new_ref_party(voucher: &ImportVoucher) -> Option<&str> {
    // Only an invoice has one, whatever detail another voucher may carry.
    if !voucher.voucher_type.is_invoice() {
        return None;
    }
    let detail = voucher.invoice.as_ref()?;
    if !detail.observed.as_ref()?.party_bill_wise {
        return None;
    }
    party_entry(voucher).map(|entry| entry.ledger.as_str())
}

/// The party's entry of an invoice: the debit that is not the round off (a
/// Sales invoice's customer). One rule, asked by the build, the render, the
/// read-back, the On Account exemption and the approval text.
pub(super) fn party_entry(voucher: &ImportVoucher) -> Option<&ImportEntry> {
    let round_off = voucher.invoice.as_ref()?.round_off_ledger.as_deref();
    voucher
        .entries
        .iter()
        .find(|entry| entry.side == EntrySide::Dr && Some(entry.ledger.as_str()) != round_off)
}

/// The financial year (1 April to 31 March) a YYYYMMDD date falls in.
fn financial_year_window(date: &str) -> Option<(String, String)> {
    let year: i32 = date.get(..4)?.parse().ok()?;
    let month: u32 = date.get(4..6)?.parse().ok()?;
    let start = if month >= 4 { year } else { year - 1 };
    Some((format!("{start}0401"), format!("{}0331", start + 1)))
}

/// Why a build believes that no voucher carries its invoice number. The read
/// answers "not in use" with no row, which a filter that matches nothing on
/// another release also gives, so the answer stands only on one of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberAbsence {
    /// The same read for an invoice the journal holds as verified posted
    /// found it.
    Controlled,
    /// The book's voucher mark is 0: it holds no voucher at all.
    EmptyBook,
    /// No invoice of this company was ever sent: the first post stands on
    /// the voucher type's own duplicate guard and its read-back.
    FirstPost,
}

/// The basis for "not in use" when the journal names no verified invoice of
/// the company to read beside it. Once an invoice was sent and none stands
/// verified, a read that matches nothing could not be told from an unused
/// number, so the build is refused until that post is verified.
fn absence_without_control(
    sent_before: bool,
    voucher_mark: Option<u64>,
) -> Result<NumberAbsence, InvoiceRefusal> {
    match (voucher_mark, sent_before) {
        (Some(0), _) => Ok(NumberAbsence::EmptyBook),
        (_, false) => Ok(NumberAbsence::FirstPost),
        (_, true) => Err(refuse("invoice_number_control_unavailable")),
    }
}

fn duty_head_of(observation: &bridge_tally_protocol::GstDutyHeadObservation) -> DutyHead {
    use bridge_tally_protocol::{GstDutyHead, GstDutyHeadObservation};
    match observation {
        GstDutyHeadObservation::Recognized { head, .. } => match head {
            GstDutyHead::Cgst => DutyHead::Cgst,
            GstDutyHead::StateTax | GstDutyHead::SgstUtgst => DutyHead::State,
            GstDutyHead::Igst => DutyHead::Igst,
            GstDutyHead::Cess => DutyHead::Cess,
            GstDutyHead::UtTax => DutyHead::Other,
        },
        GstDutyHeadObservation::Unrecognized { .. }
        | GstDutyHeadObservation::Contradictory { .. } => DutyHead::Other,
        GstDutyHeadObservation::NotTaxLedger { .. } | GstDutyHeadObservation::Absent => {
            DutyHead::NotTax
        }
    }
}

/// What the admission's judge still needs read before it can decide (#1337).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Need {
    Marks,
    LedgerCount,
    Listing,
    Rates,
    VoucherTypes,
    Number,
    NumberControl,
    Registration,
}

/// Why the judge has no observation yet.
#[derive(Debug)]
pub(super) enum Verdict {
    /// The next answer it needs, which the caller reads and offers again.
    Need(Need),
    Refused(Vec<InvoiceRefusal>),
    /// A read that could not be used, under the code that names it.
    Failed(String),
}

fn refusing(refusal: InvoiceRefusal) -> Verdict {
    Verdict::Refused(vec![refusal])
}

fn unusable(code: &'static str) -> Verdict {
    Verdict::Failed(code.to_string())
}

/// What one Sales invoice's admission is judged on, apart from the answers: the
/// invoice, the catalogue its party's bill-wise flag is read from, and the
/// number control the journal offers.
pub(super) struct InvoiceJudge<'a> {
    pub(super) company_name: &'a str,
    pub(super) company_guid: &'a str,
    pub(super) voucher: &'a ImportVoucher,
    pub(super) catalogue: &'a bridge_tally_protocol::StandardLedgerCatalogV2,
    pub(super) control: &'a super::ledger::NumberControl,
}

fn admitted(
    request: super::super::ReadRequest,
) -> Result<crate::tally::agent_read_request::AgentReadRequest, Verdict> {
    crate::tally::agent_read_request::AgentReadRequest::parse(request.into_xml())
        .map_err(|error| Verdict::Failed(error.to_string()))
}

impl InvoiceJudge<'_> {
    fn as_of(&self) -> Result<String, Verdict> {
        super::super::normalized_date(&self.voucher.date)
            .map(|date| date.as_str().to_string())
            .map_err(|_| unusable("invoice_date_invalid"))
    }

    fn year(&self) -> Result<(String, String), Verdict> {
        financial_year_window(&self.as_of()?).ok_or_else(|| unusable("invoice_date_invalid"))
    }

    /// The request that reads what `need` names, fixed from the invoice: the
    /// build sends it and the queue sends it again. The marks and the listing
    /// are not requests of the plan (the queue's binding marks are the marks;
    /// the listing is read as the compliance read reads it).
    pub(super) fn request(
        &self,
        need: Need,
    ) -> Result<crate::tally::agent_read_request::AgentReadRequest, Verdict> {
        let company = self.company_name;
        match need {
            Need::LedgerCount => admitted(super::super::invoice_ledger_count_read(company)),
            Need::VoucherTypes => admitted(super::super::invoice_voucher_types_read(company)),
            Need::Registration => {
                admitted(super::super::invoice_company_registration_read(company))
            }
            Need::Rates => {
                let window = self.year()?;
                let request = super::super::invoice_ledger_rates_read(
                    company,
                    (&window.0, self.as_of()?.as_str()),
                )
                .ok_or_else(|| unusable("invoice_date_invalid"))?;
                admitted(request)
            }
            Need::Number => {
                let number = self.voucher.voucher_number.clone().unwrap_or_default();
                let year = self.year()?;
                let request =
                    super::super::invoice_number_read(company, &number, (&year.0, &year.1))
                        .ok_or_else(|| refusing(refuse_value("invoice_number_invalid", &number)))?;
                admitted(request)
            }
            Need::NumberControl => match self.number_control_request()? {
                Some(request) => Ok(request),
                None => Err(unusable("invoice_number_control_unreadable")),
            },
            Need::Marks | Need::Listing => Err(unusable("invoice_read_plan_inconsistent")),
        }
    }

    /// The read of the invoice the journal offers as the number's control, or
    /// none when it offers none.
    fn number_control_request(
        &self,
    ) -> Result<Option<crate::tally::agent_read_request::AgentReadRequest>, Verdict> {
        let super::ledger::NumberControl::Known { number, date, .. } = self.control else {
            return Ok(None);
        };
        let window = financial_year_window(date)
            .ok_or_else(|| unusable("invoice_number_control_unreadable"))?;
        let request =
            super::super::invoice_number_read(self.company_name, number, (&window.0, &window.1))
                .ok_or_else(|| unusable("invoice_number_control_unreadable"))?;
        admitted(request).map(Some)
    }

    /// The plan an admission that read the answers it did makes: every request
    /// fixed from the invoice, the ledger count only when it was read.
    pub(super) fn plan(
        &self,
        listing_as_of: bridge_tally_core::TallyDate,
        counted: bool,
    ) -> Result<InvoiceReadPlan, Verdict> {
        Ok(InvoiceReadPlan {
            listing_as_of,
            ledger_count: if counted {
                Some(self.request(Need::LedgerCount)?)
            } else {
                None
            },
            rates: self.request(Need::Rates)?,
            voucher_types: self.request(Need::VoucherTypes)?,
            number: self.request(Need::Number)?,
            number_control: self.number_control_request()?,
            registration: self.request(Need::Registration)?,
        })
    }

    /// Classify the invoice on the answers read so far: the observation the
    /// build records, a refusal, or the next answer needed. The same function
    /// runs in the build, in the re-read before the dialog and after it, and in
    /// the queue (#1337); each answer is judged as it arrives, in this order,
    /// so a refusal needs none of the reads after it.
    pub(super) fn judge(&self, answers: &InvoiceAnswers) -> Result<InvoiceObserved, Verdict> {
        let voucher = self.voucher;
        let Some(detail) = voucher.invoice.as_ref() else {
            return Err(refusing(refuse("invoice_detail_required")));
        };
        let as_of = self.as_of()?;
        let type_name = detail.voucher_type_name.clone();
        let party_name = party_entry(voucher)
            .map(|entry| entry.ledger.clone())
            .ok_or_else(|| refusing(refuse("invoice_party_missing")))?;

        // 0. Refuse large books: the whole ledger compliance listing is read
        // three times for one invoice (build, before the dialog, after it).
        let Some(marks) = answers.marks.as_deref() else {
            return Err(Verdict::Need(Need::Marks));
        };
        let mark = super::pre_import_mark_of(marks, self.company_guid).map_err(Verdict::Failed)?;
        // The duplicate-number read and the read-back each walk the book's
        // vouchers (a formula decides the rows, so Tally evaluates every
        // voucher): at the build, at both re-reads, after the post and at
        // every later verification. The voucher mark bounds that walk.
        if let Some(refusal) = voucher_mark_refusal(mark.value) {
            return Err(refusing(refusal));
        }
        match mark.master_value {
            Some(value) if mark_admits(value) => {}
            // The mark counts every master alteration, so a long-lived or
            // stock-heavy book is over it whatever its ledger count: the
            // company's own count of its ledgers can admit such a book.
            Some(value) => {
                let Some(body) = answers.ledger_count.as_deref() else {
                    return Err(Verdict::Need(Need::LedgerCount));
                };
                let count = bridge_tally_protocol::outstandings_shared::parse_company_ledger_count(
                    body,
                    self.company_name,
                    self.company_guid,
                )
                .map_err(|_| unusable("invoice_book_size_unreadable"))?;
                if let Some(refusal) = ledger_count_refusal(count.map(|count| count.get()), value) {
                    return Err(refusing(refusal));
                }
            }
            None => return Err(refusing(refuse("invoice_book_size_unknown"))),
        }

        // 1. The compliance listing.
        let Some(listing) = answers.listing.as_ref() else {
            return Err(Verdict::Need(Need::Listing));
        };
        let index =
            bridge_tally_protocol::group_ancestry::GroupIndex::build(listing.groups.clone());
        // 1b. Each ledger's GST rate and rounding, from the same listing with
        // four fields added (W7, 10 Oct 2026): the tax is worked out from the
        // sales ledger's own rate, so the rate is read, never inferred.
        let Some(rates_body) = answers.rates.as_deref() else {
            return Err(Verdict::Need(Need::Rates));
        };
        let wanted = voucher
            .entries
            .iter()
            .map(|entry| entry.ledger.as_str())
            .collect::<Vec<_>>();
        let rates = wire::parse_ledger_rates(rates_body, self.company_guid, &wanted)
            .map_err(|code| Verdict::Failed(code.to_string()))?;
        let mut facts = BTreeMap::new();
        for entry in &voucher.entries {
            let matching = listing
                .records
                .iter()
                .filter(|record| record.ledger.name == entry.ledger)
                .collect::<Vec<_>>();
            let [record] = matching.as_slice() else {
                // Absent or repeated: classification reports it as not observed.
                continue;
            };
            let parent = record.ledger.parent.returned_text().map(str::to_string);
            let chain = index.ancestry_chain(parent.as_deref());
            let reserved_groups = if chain.is_complete() {
                chain
                    .hops
                    .iter()
                    .map(|hop| hop.reserved_name.clone())
                    .collect()
            } else {
                Vec::new()
            };
            let (gstin, registration_type) = gstin_evidence(
                record.ledger.party_gstin.returned_text(),
                &record.fields.gst_registrations,
                &as_of,
            );
            facts.insert(
                entry.ledger.clone(),
                LedgerFacts {
                    reserved_groups,
                    duty_head: duty_head_of(&record.fields.gst_duty_head),
                    gstin,
                    registration_type,
                    bill_wise: None,
                    rates: rates.get(&entry.ledger).cloned(),
                },
            );
        }

        // 2. The party's bill-wise flag, from the ledger catalogue the build
        // already reads (the V2 catalogue carries ISBILLWISEON per ledger,
        // section 12a.15): no extra read, and the same answer the bill-wise
        // approval gate judges.
        let bill_wise = party_bill_wise_in(self.catalogue, &party_name).ok_or_else(|| {
            refusing(refuse_ledger(
                "invoice_party_bill_wise_unknown",
                &party_name,
            ))
        })?;
        if let Some(fact) = facts.get_mut(&party_name) {
            fact.bill_wise = Some(bill_wise);
        }

        // 3. The voucher type: named by the caller, never chosen.
        let Some(types_body) = answers.voucher_types.as_deref() else {
            return Err(Verdict::Need(Need::VoucherTypes));
        };
        let types = wire::parse_voucher_types(types_body)
            .map_err(|code| Verdict::Failed(code.to_string()))?;
        let resolved = wire::resolve_voucher_type(&types, &type_name, "Sales")
            .map_err(|code| refusing(refuse_value(code, &type_name)))?;

        // 3b. The number must not already be in use in this type and year.
        let number = voucher.voucher_number.clone().unwrap_or_default();
        let Some(number_body) = answers.number.as_deref() else {
            return Err(Verdict::Need(Need::Number));
        };
        if wire::count_sales_vouchers(number_body)
            .map_err(|code| Verdict::Failed(code.to_string()))?
            != 0
        {
            return Err(refusing(refuse_value(
                "invoice_number_already_used",
                &number,
            )));
        }
        // 3c. "No voucher carries this number" is believed only beside a
        // control: the same read for an invoice this company is known to hold
        // must find it (a read that matches nothing on this Tally fails
        // here), or the book holds no voucher at all, or this is the
        // company's first invoice sent (whose read-back is then the check).
        let _absence = match self.control {
            super::ledger::NumberControl::Known {
                batch_id,
                number: known,
                date,
            } => {
                let Some(control_body) = answers.number_control.as_deref() else {
                    return Err(Verdict::Need(Need::NumberControl));
                };
                if !wire::control_row_found(control_body, known, date)
                    .map_err(|code| Verdict::Failed(code.to_string()))?
                {
                    return Err(refusing(refuse_value(
                        "invoice_number_control_missing",
                        batch_id,
                    )));
                }
                NumberAbsence::Controlled
            }
            super::ledger::NumberControl::NeverSent => {
                absence_without_control(false, mark.value).map_err(refusing)?
            }
            super::ledger::NumberControl::NoneVerified => {
                absence_without_control(true, mark.value).map_err(refusing)?
            }
        };

        // 4. The company's own GST registration in force on the invoice date
        // (ADR 0004, slice 1); its state is the supplier's state.
        let Some(registration_body) = answers.registration.as_deref() else {
            return Err(Verdict::Need(Need::Registration));
        };
        let company_state =
            wire::parse_company_registration(registration_body, self.company_guid, &as_of)
                .map_err(|outcome| match outcome {
                    wire::RegistrationOutcome::Refused(code) => refusing(refuse(code)),
                    wire::RegistrationOutcome::Failed(code) => unusable(code),
                })?
                .state;

        let roles = classify_sales_invoice(voucher, &facts, &company_state, &as_of)
            .map_err(Verdict::Refused)?;
        let party = &voucher.entries[roles.party].ledger;
        Ok(observe(
            &facts[party].gstin,
            resolved.guid,
            bill_wise,
            company_state,
        ))
    }
}

impl super::super::Server {
    /// Read what the masters say about one Sales invoice, classify every leg,
    /// and record what was observed on the voucher. The reads: the company's
    /// marks (and its ledger count when the master mark is high), the ledger
    /// compliance listing (reserved group ancestry, duty head, the GSTIN in
    /// force on the invoice date), the same listing with each ledger's GST
    /// rate and rounding (the tax is worked out from it), the voucher types (the named type, its
    /// class and its series-level numbering), the vouchers carrying the number
    /// and the company's GST registration in force on the invoice date. The
    /// party's bill-wise flag comes from the
    /// catalogue the caller already read. Each answer is bound to the verified
    /// company.
    pub(super) async fn admit_sales_invoice(
        &self,
        identity: &super::super::VerifiedCompanyIdentity,
        company: &bridge_tally_protocol::TallyCompany,
        voucher: &mut ImportVoucher,
        catalogue: &bridge_tally_protocol::StandardLedgerCatalogV2,
    ) -> Result<super::super::Evidence, InvoiceAdmission> {
        Box::pin(self.admit_sales_invoice_planned(identity, company, voucher, catalogue))
            .await
            .map(|(evidence, _)| evidence)
    }

    /// [`Self::admit_sales_invoice`], also returning the reads it made as the
    /// plan the queue makes again under its lock (#1337).
    pub(super) async fn admit_sales_invoice_planned(
        &self,
        identity: &super::super::VerifiedCompanyIdentity,
        company: &bridge_tally_protocol::TallyCompany,
        voucher: &mut ImportVoucher,
        catalogue: &bridge_tally_protocol::StandardLedgerCatalogV2,
    ) -> Result<(super::super::Evidence, InvoiceReadPlan), InvoiceAdmission> {
        if voucher.invoice.is_none() {
            return Err(refused(refuse("invoice_detail_required")));
        }
        let as_of = super::super::normalized_date(&voucher.date)
            .map_err(|_| failed("invoice_date_invalid"))?
            .as_str()
            .to_string();
        let year = financial_year_window(&as_of).ok_or_else(|| failed("invoice_date_invalid"))?;

        // A sent invoice of this company that is not verified posted stops
        // every further invoice until a person releases it (ADR 0004, slice
        // 4). Asked first, from the journal alone, so a stopped company costs
        // Tally no request; the post asks again under its lock.
        if let Some(batch_id) = self
            .import_invoice_stop(identity.company_guid())
            .map_err(|_| failed("invoice_stop_unreadable"))?
        {
            return Err(refused(refuse_value("invoice_company_stopped", &batch_id)));
        }
        // The journal's offer of a control for the number read, as it stands now.
        let control = self
            .import_invoice_number_control(identity.company_guid(), (&year.0, &year.1))
            .map_err(|_| failed("invoice_number_control_unreadable"))?;
        let today = bridge_tally_core::TallyDate::parse(super::super::tally_host_today())
            .map_err(|_| failed("current_date_invalid"))?;
        let judge = InvoiceJudge {
            company_name: company.name.as_str(),
            company_guid: identity.company_guid(),
            voucher: &*voucher,
            catalogue,
            control: &control,
        };
        let mut answers = InvoiceAnswers::default();
        let mut evidence: Option<super::super::Evidence> = None;
        let note = |evidence: &mut Option<super::super::Evidence>, read: super::super::Evidence| {
            *evidence = Some(match evidence.take() {
                Some(earlier) => super::super::combine_evidence(earlier, read),
                None => read,
            });
        };
        let observed = loop {
            match judge.judge(&answers) {
                Ok(observed) => break observed,
                Err(Verdict::Refused(refusals)) => return Err(InvoiceAdmission::Refused(refusals)),
                Err(Verdict::Failed(code)) => {
                    let failure = super::super::ToolFailure::from(code);
                    return Err(InvoiceAdmission::Failed(match evidence.clone() {
                        Some(read) => failure.with_prior_evidence(read),
                        None => failure,
                    }));
                }
                Err(Verdict::Need(need)) => match need {
                    Need::Marks => {
                        let (xml, read) = self
                            .post_read(
                                identity,
                                super::super::company_high_water_read(&company.name),
                            )
                            .await?;
                        answers.marks = Some(xml);
                        note(&mut evidence, read);
                    }
                    Need::Listing => {
                        let listing =
                            Box::pin(self.runtime.fetch_agent_party_ledger_masters_with_evidence(
                                self.tally_config(),
                                identity,
                                today.clone(),
                            ))
                            .await
                            .map_err(|error| {
                                super::super::ToolFailure::from_runtime(
                                    "party_ledger_master_read_failed",
                                    error,
                                )
                            })?;
                        note(
                            &mut evidence,
                            super::super::evidence_from_runtime_read(listing.evidence.clone()),
                        );
                        answers.listing = Some(listing);
                    }
                    Need::LedgerCount
                    | Need::Rates
                    | Need::VoucherTypes
                    | Need::Number
                    | Need::NumberControl
                    | Need::Registration => {
                        let request = judge.request(need).map_err(|verdict| match verdict {
                            Verdict::Refused(refusals) => InvoiceAdmission::Refused(refusals),
                            Verdict::Failed(code) => failed_with(code),
                            Verdict::Need(_) => failed("invoice_read_plan_inconsistent"),
                        })?;
                        let (xml, read, _) = self.post_admitted_read(identity, request).await?;
                        let slot = match need {
                            Need::LedgerCount => &mut answers.ledger_count,
                            Need::Rates => &mut answers.rates,
                            Need::VoucherTypes => &mut answers.voucher_types,
                            Need::Number => &mut answers.number,
                            Need::NumberControl => &mut answers.number_control,
                            _ => &mut answers.registration,
                        };
                        *slot = Some(xml);
                        note(&mut evidence, read);
                    }
                },
            }
        };
        let counted = answers.ledger_count.is_some();
        let plan = judge
            .plan(today, counted)
            .map_err(|verdict| match verdict {
                Verdict::Refused(refusals) => InvoiceAdmission::Refused(refusals),
                Verdict::Failed(code) => failed_with(code),
                Verdict::Need(_) => failed("invoice_read_plan_inconsistent"),
            })?;
        if let Some(detail) = voucher.invoice.as_mut() {
            detail.observed = Some(observed);
        }
        match evidence {
            Some(evidence) => Ok((evidence, plan)),
            None => Err(failed("invoice_read_plan_inconsistent")),
        }
    }
}

/// What the build records of an admitted invoice. `classify_sales_invoice` has
/// run: a GSTIN in force is a valid Regular one in the company's state, and no
/// GSTIN means an entry in force that says the customer is unregistered.
fn observe(
    party: &GstinEvidence,
    voucher_type_guid: String,
    party_bill_wise: bool,
    company_state: String,
) -> InvoiceObserved {
    let party_gstin = match party {
        GstinEvidence::InForce(gstin) => Some(gstin.clone()),
        _ => None,
    };
    InvoiceObserved {
        voucher_type_guid,
        party_state: match party_gstin.as_deref() {
            Some(gstin) => gstin_state(gstin).unwrap_or_default().to_string(),
            None => company_state.clone(),
        },
        party_registration_type: if party_gstin.is_some() {
            "Regular".to_string()
        } else {
            "Unregistered/Consumer".to_string()
        },
        party_gstin,
        party_bill_wise,
        company_state,
    }
}

/// The XML of one Sales invoice, in the invoice view, from the saved batch
/// alone: the party leg first with its New Ref, then the credit legs. The
/// header carries the buyer and consignee block a hand-keyed invoice carries
/// (section 9.16): the three names are the party ledger's name, because its
/// mailing name is not read; the consignee's state is the party's state; the
/// dealer type is the value the keyed invoices read.
pub(super) fn render_sales_invoice_xml(
    voucher: &ImportVoucher,
    remote_id: Uuid,
    date: &str,
    narration: &str,
) -> Option<String> {
    let detail = voucher.invoice.as_ref()?;
    let observed = detail.observed.as_ref()?;
    let number = voucher.voucher_number.as_deref()?;
    let party_entry = party_entry(voucher)?;
    let leg = |entry: &ImportEntry, bill: bool| {
        let signed = match entry.side {
            EntrySide::Dr => format!("-{}", entry.amount),
            EntrySide::Cr => entry.amount.clone(),
        };
        let allocation = if bill {
            format!(
                "<BILLALLOCATIONS.LIST><NAME>{}</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>{signed}</AMOUNT></BILLALLOCATIONS.LIST>",
                xml_escape(number)
            )
        } else {
            String::new()
        };
        format!(
            "<LEDGERENTRIES.LIST><LEDGERNAME>{}</LEDGERNAME><ISDEEMEDPOSITIVE>{}</ISDEEMEDPOSITIVE><AMOUNT>{signed}</AMOUNT>{allocation}</LEDGERENTRIES.LIST>",
            xml_escape(&entry.ledger),
            entry.side.tally_positive()
        )
    };
    let mut legs = leg(party_entry, observed.party_bill_wise);
    for entry in voucher
        .entries
        .iter()
        .filter(|entry| !std::ptr::eq(*entry, party_entry))
    {
        legs.push_str(&leg(entry, false));
    }
    let type_name = xml_escape(&detail.voucher_type_name);
    let party = xml_escape(&party_entry.ledger);
    let gstin = observed
        .party_gstin
        .as_deref()
        .map(|gstin| format!("<PARTYGSTIN>{}</PARTYGSTIN>", xml_escape(gstin)))
        .unwrap_or_default();
    Some(format!(
        "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"{remote_id}\" VCHTYPE=\"{type_name}\" ACTION=\"Create\" OBJVIEW=\"Invoice Voucher View\"><DATE>{date}</DATE><EFFECTIVEDATE>{date}</EFFECTIVEDATE><REFERENCEDATE>{date}</REFERENCEDATE><REFERENCE>{}</REFERENCE><VOUCHERTYPENAME>{type_name}</VOUCHERTYPENAME><VOUCHERNUMBER>{}</VOUCHERNUMBER><PARTYLEDGERNAME>{party}</PARTYLEDGERNAME><PARTYNAME>{party}</PARTYNAME><BASICBASEPARTYNAME>{party}</BASICBASEPARTYNAME><PARTYMAILINGNAME>{party}</PARTYMAILINGNAME><BASICBUYERNAME>{party}</BASICBUYERNAME><CONSIGNEEMAILINGNAME>{party}</CONSIGNEEMAILINGNAME>{gstin}<STATENAME>{}</STATENAME><CONSIGNEESTATENAME>{consignee_state}</CONSIGNEESTATENAME><PLACEOFSUPPLY>{}</PLACEOFSUPPLY><GSTREGISTRATIONTYPE>{}</GSTREGISTRATIONTYPE><VATDEALERTYPE>Regular</VATDEALERTYPE><COUNTRYOFRESIDENCE>India</COUNTRYOFRESIDENCE><CONSIGNEECOUNTRYNAME>India</CONSIGNEECOUNTRYNAME><PERSISTEDVIEW>Invoice Voucher View</PERSISTEDVIEW><VCHENTRYMODE>Accounting Invoice</VCHENTRYMODE><ISINVOICE>Yes</ISINVOICE>{narration}{legs}</VOUCHER></TALLYMESSAGE>",
        xml_escape(number),
        xml_escape(number),
        xml_escape(&observed.party_state),
        xml_escape(&detail.place_of_supply),
        xml_escape(&observed.party_registration_type),
        consignee_state = xml_escape(&observed.party_state),
    ))
}

#[cfg(test)]
#[path = "agent_import_invoice_tests.rs"]
mod tests;
