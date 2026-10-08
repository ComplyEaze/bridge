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
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[path = "agent_import_invoice_wire.rs"]
mod wire;

/// The request builders the sealed read profiles wrap.
pub(in crate::agent) use wire::{
    render_company_state_request, render_invoice_number_request, render_invoice_readback_request,
    render_voucher_types_request,
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

/// The GST slab rates an invoice's tax may be, in percent.
const SLAB_RATES: &[i128] = &[5, 12, 18, 28, 40];

/// What a tax leg may differ from taxable x rate / 2 by, in paise: a few
/// paise of per-line rounding on a multi-line bill, never a rupee.
const TAX_LEG_TOLERANCE_PAISE: i128 = 5;

/// The two tax legs of an intra-state invoice, in paise. They are equal, or
/// differ by one paisa: a bill whose total tax is odd cannot split evenly, and
/// the return reports the odd paisa on one head. Built only by `new`, so a
/// pair that differs by more is not a value of this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TaxLegs {
    cgst: i128,
    state: i128,
}

impl TaxLegs {
    fn new(cgst: i128, state: i128) -> Option<Self> {
        (cgst.abs_diff(state) <= 1).then_some(Self { cgst, state })
    }

    /// Whether EACH leg is half the tax at `rate` percent of `taxable`, to
    /// within `TAX_LEG_TOLERANCE_PAISE`. Checked arithmetic: an amount too
    /// large to multiply matches no rate.
    fn is_half_of(&self, taxable: i128, rate: i128) -> bool {
        [self.cgst, self.state].into_iter().all(|leg| {
            leg.checked_mul(200)
                .zip(taxable.checked_mul(rate))
                .and_then(|(left, right)| left.checked_sub(right))
                .is_some_and(|difference| difference.abs() <= TAX_LEG_TOLERANCE_PAISE * 200)
        })
    }
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
    // The taxable value is the sum of the sales legs, all at the one tax pair's rate.
    let taxable = sales.iter().try_fold(0_i128, |sum, index| {
        amount(*index).and_then(|leg| sum.checked_add(leg))
    });
    let (Some(party_amount), Some(taxable), Some(cgst_amount), Some(state_amount)) =
        (amount(party), taxable, amount(cgst), amount(state_tax))
    else {
        return Err(vec![refuse("invoice_amount_invalid")]);
    };
    let Some(legs) = TaxLegs::new(cgst_amount, state_amount) else {
        return Err(vec![refuse("invoice_cgst_and_state_tax_differ")]);
    };
    let Some(base) = taxable
        .checked_add(legs.cgst)
        .and_then(|sum| sum.checked_add(legs.state))
    else {
        return Err(vec![refuse("invoice_amount_invalid")]);
    };
    // Each leg is half the tax at one slab rate, to within a few paise: a
    // 10.00 sale carrying 0.02 of tax is not 5 percent.
    if !SLAB_RATES
        .iter()
        .any(|rate| legs.is_half_of(taxable, *rate))
    {
        return Err(vec![refuse("invoice_tax_matches_no_slab_rate")]);
    }
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
    InvoiceAdmission::Failed(super::super::ToolFailure::from(code.to_string()))
}

impl super::super::Server {
    /// Read the invoice's masters again. The invoice must be admitted again
    /// (every leg's group and duty head, the type's series, the number still
    /// unused: a refusal there comes back under its own code), and what the
    /// build recorded must be unchanged (`import_invoice_masters_changed`): the
    /// party's GSTIN, state, registration and bill-wise flag, the type's GUID
    /// and the company's state. Run before the approval
    /// dialog and again, after it is answered, before the post is dispatched,
    /// because a dialog can stay open and a book can change under it.
    pub(super) async fn recheck_sales_invoice(
        &self,
        identity: &super::super::VerifiedCompanyIdentity,
        company: &bridge_tally_protocol::TallyCompany,
        saved: &ImportVoucher,
        catalogue: &bridge_tally_protocol::StandardLedgerCatalogV2,
    ) -> Result<super::super::Evidence, super::super::ToolFailure> {
        let mut fresh = saved.clone();
        let recorded = fresh
            .invoice
            .as_mut()
            .and_then(|detail| detail.observed.take());
        let changed =
            || super::super::ToolFailure::from("import_invoice_masters_changed".to_string());
        let evidence = match self
            .admit_sales_invoice(identity, company, &mut fresh, catalogue)
            .await
        {
            Ok(evidence) => evidence,
            Err(InvoiceAdmission::Failed(failure)) => return Err(failure),
            // The refusal's own code is the answer: a number now in use is not
            // "a master changed", and a rebuild would only be refused again.
            Err(InvoiceAdmission::Refused(refusals)) => {
                return Err(match refusals.first() {
                    Some(first) => super::super::ToolFailure::from(first.code.to_string()),
                    None => changed(),
                });
            }
        };
        let now = fresh
            .invoice
            .as_ref()
            .and_then(|detail| detail.observed.as_ref());
        if !observation_unchanged(recorded.as_ref(), now) {
            return Err(changed());
        }
        Ok(evidence)
    }
}

/// A re-read admits a post only when the build recorded an observation and the
/// re-read made the same one, whole. An invoice with none recorded is never
/// "unchanged".
fn observation_unchanged(
    recorded: Option<&InvoiceObserved>,
    now: Option<&InvoiceObserved>,
) -> bool {
    matches!((recorded, now), (Some(recorded), Some(now)) if recorded == now)
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

impl super::super::Server {
    /// Read what the masters say about one Sales invoice, classify every leg,
    /// and record what was observed on the voucher. The reads: the company's
    /// marks (and its ledger count when the master mark is high), the ledger
    /// compliance listing (reserved group ancestry, duty head, the GSTIN in
    /// force on the invoice date), the voucher types (the named type, its
    /// class and its series-level numbering), the vouchers carrying the number
    /// and the company's state. The party's bill-wise flag comes from the
    /// catalogue the caller already read. Each answer is bound to the verified
    /// company.
    pub(super) async fn admit_sales_invoice(
        &self,
        identity: &super::super::VerifiedCompanyIdentity,
        company: &bridge_tally_protocol::TallyCompany,
        voucher: &mut ImportVoucher,
        catalogue: &bridge_tally_protocol::StandardLedgerCatalogV2,
    ) -> Result<super::super::Evidence, InvoiceAdmission> {
        let company_name = company.name.as_str();
        let Some(detail) = voucher.invoice.as_ref() else {
            return Err(refused(refuse("invoice_detail_required")));
        };
        let as_of = super::super::normalized_date(&voucher.date)
            .map_err(|_| failed("invoice_date_invalid"))?
            .as_str()
            .to_string();
        let type_name = detail.voucher_type_name.clone();
        let party_name = party_entry(voucher)
            .map(|entry| entry.ledger.clone())
            .ok_or_else(|| refused(refuse("invoice_party_missing")))?;

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
        // 0. Refuse large books: the whole ledger compliance listing is read
        // three times for one invoice (build, before the dialog, after it).
        let (mark, mark_evidence) = self
            .pre_import_mark(company, identity)
            .await
            .map_err(InvoiceAdmission::Failed)?;
        // The duplicate-number read and the read-back each walk the book's
        // vouchers (a formula decides the rows, so Tally evaluates every
        // voucher): at the build, at both re-reads, after the post and at
        // every later verification. The voucher mark bounds that walk.
        if let Some(refusal) = voucher_mark_refusal(mark.value) {
            return Err(refused(refusal));
        }
        let mut size_evidence = mark_evidence;
        match mark.master_value {
            Some(value) if mark_admits(value) => {}
            // The mark counts every master alteration, so a long-lived or
            // stock-heavy book is over it whatever its ledger count: the
            // company's own count of its ledgers can admit such a book.
            Some(value) => {
                let (xml, read) = self
                    .post_read(
                        identity,
                        super::super::invoice_ledger_count_read(&company.name),
                    )
                    .await?;
                size_evidence = super::super::combine_evidence(size_evidence, read);
                let count = bridge_tally_protocol::outstandings_shared::parse_company_ledger_count(
                    &xml,
                    &company.name,
                    identity.company_guid(),
                )
                .map_err(|_| failed("invoice_book_size_unreadable"))?;
                if let Some(refusal) = ledger_count_refusal(count.map(|count| count.get()), value) {
                    return Err(refused(refusal));
                }
            }
            None => return Err(refused(refuse("invoice_book_size_unknown"))),
        }
        let mark_evidence = size_evidence;

        // 1. The compliance listing.
        let today = bridge_tally_core::TallyDate::parse(super::super::tally_host_today())
            .map_err(|_| failed("current_date_invalid"))?;
        let listing = self
            .runtime
            .fetch_agent_party_ledger_masters_with_evidence(self.tally_config(), identity, today)
            .await
            .map_err(|error| {
                super::super::ToolFailure::from_runtime("party_ledger_master_read_failed", error)
            })?;
        let mut evidence = super::super::combine_evidence(
            mark_evidence,
            super::super::evidence_from_runtime_read(listing.evidence.clone()),
        );
        let index =
            bridge_tally_protocol::group_ancestry::GroupIndex::build(listing.groups.clone());
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
                },
            );
        }

        // 2. The party's bill-wise flag, from the ledger catalogue the build
        // already reads (the V2 catalogue carries ISBILLWISEON per ledger,
        // section 12a.15): no extra read, and the same answer the bill-wise
        // approval gate judges.
        let bill_wise = party_bill_wise_in(catalogue, &party_name).ok_or_else(|| {
            refused(refuse_ledger(
                "invoice_party_bill_wise_unknown",
                &party_name,
            ))
        })?;
        if let Some(fact) = facts.get_mut(&party_name) {
            fact.bill_wise = Some(bill_wise);
        }

        // 3. The voucher type: named by the caller, never chosen.
        let (xml, read) = self
            .post_read(
                identity,
                super::super::invoice_voucher_types_read(company_name),
            )
            .await?;
        evidence = super::super::combine_evidence(evidence, read);
        let types = wire::parse_voucher_types(&xml).map_err(failed)?;
        let resolved = wire::resolve_voucher_type(&types, &type_name, "Sales")
            .map_err(|code| refused(refuse_value(code, &type_name)))?;

        // 3b. The number must not already be in use in this type and year.
        let number = voucher.voucher_number.clone().unwrap_or_default();
        let year = financial_year_window(&as_of).ok_or_else(|| failed("invoice_date_invalid"))?;
        let request = super::super::invoice_number_read(company_name, &number, (&year.0, &year.1))
            .ok_or_else(|| refused(refuse_value("invoice_number_invalid", &number)))?;
        let (xml, read) = self.post_read(identity, request).await?;
        evidence = super::super::combine_evidence(evidence, read);
        if wire::count_sales_vouchers(&xml).map_err(failed)? != 0 {
            return Err(refused(refuse_value(
                "invoice_number_already_used",
                &number,
            )));
        }
        // 3c. "No voucher carries this number" is believed only beside a
        // control: the same read for an invoice this company is known to hold
        // must find it (a read that matches nothing on this Tally fails
        // here), or the book holds no voucher at all, or this is the
        // company's first invoice sent (whose read-back is then the check).
        let control = self
            .import_invoice_number_control(identity.company_guid(), (&year.0, &year.1))
            .map_err(|_| failed("invoice_number_control_unreadable"))?;
        let _absence = match control {
            super::ledger::NumberControl::Known {
                number: known,
                date,
            } => {
                let window = financial_year_window(&date)
                    .ok_or_else(|| failed("invoice_number_control_unreadable"))?;
                let request =
                    super::super::invoice_number_read(company_name, &known, (&window.0, &window.1))
                        .ok_or_else(|| failed("invoice_number_control_unreadable"))?;
                let (xml, read) = self.post_read(identity, request).await?;
                evidence = super::super::combine_evidence(evidence, read);
                if !wire::control_row_found(&xml, &known, &date).map_err(failed)? {
                    return Err(refused(refuse_value(
                        "invoice_number_control_missing",
                        &known,
                    )));
                }
                NumberAbsence::Controlled
            }
            super::ledger::NumberControl::NeverSent => {
                absence_without_control(false, mark.value).map_err(refused)?
            }
            super::ledger::NumberControl::NoneVerified => {
                absence_without_control(true, mark.value).map_err(refused)?
            }
        };

        // 4. The company's state.
        let (xml, read) = self
            .post_read(
                identity,
                super::super::invoice_company_state_read(company_name),
            )
            .await?;
        evidence = super::super::combine_evidence(evidence, read);
        let company_state =
            wire::parse_company_state(&xml, identity.company_guid()).map_err(failed)?;

        let roles = classify_sales_invoice(voucher, &facts, &company_state)
            .map_err(InvoiceAdmission::Refused)?;
        let party = &voucher.entries[roles.party].ledger;
        let observed = observe(&facts[party].gstin, resolved.guid, bill_wise, company_state);
        if let Some(detail) = voucher.invoice.as_mut() {
            detail.observed = Some(observed);
        }
        Ok(evidence)
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
