//! Voucher parse for the local MCP adapter.
use super::*;

#[path = "agent_voucher_bank_allocations.rs"]
mod bank_allocations;
#[path = "agent_voucher_cost_centres.rs"]
mod cost_centres;
#[path = "agent_voucher_entry_gst.rs"]
mod entry_gst;
#[path = "agent_voucher_goods_lines.rs"]
mod goods_lines;
#[path = "agent_voucher_scalars.rs"]
mod scalars;
use bank_allocations::BankAllocationCollector;
use bridge_tally_core::ExactDecimal;
use bridge_tally_protocol::currency_composite::{
    is_balance_composite, parse_currency_composite, CurrencyComposite, Side,
};
use bridge_tally_protocol::native_outstandings::IdentifiedBaseCurrency;
use cost_centres::CostCentreCollector;
pub(super) use cost_centres::CostCentreProblem;
use entry_gst::EntryGstCollector;
use goods_lines::{GoodsLineCollector, GoodsLineList, GoodsLineProblem, STOCK_LINE_LIST};
pub(super) use scalars::*;

/// Shares the native collection boundary used by the protocol crate: CMPINFO
/// contains identically named counters outside BODY/DATA/COLLECTION.
#[derive(Default)]
pub(super) struct NativeCollectionScope {
    path: Vec<String>,
    collection_seen: bool,
    repeated_collection: bool,
}

impl NativeCollectionScope {
    pub(super) fn collection(&self) -> bool {
        self.path == ["ENVELOPE", "BODY", "DATA", "COLLECTION"]
    }
    pub(super) fn row(&self, row: &str) -> bool {
        self.path.len() == 5
            && self.path[..4] == ["ENVELOPE", "BODY", "DATA", "COLLECTION"]
            && self.path[4] == row
    }
    pub(super) fn field(&self, row: &str) -> bool {
        self.path.len() == 6
            && self.path[..4] == ["ENVELOPE", "BODY", "DATA", "COLLECTION"]
            && self.path[4] == row
    }
    pub(super) fn child(&self, row: &str, child: &str) -> bool {
        self.field(row) && self.path[5] == child
    }
    pub(super) fn entry_field(&self) -> bool {
        self.path.len() == 7
            && self.path[..5] == ["ENVELOPE", "BODY", "DATA", "COLLECTION", "VOUCHER"]
            && self.path[5] == "ALLLEDGERENTRIES.LIST"
    }
    /// The path below the current `ALLLEDGERENTRIES.LIST`, including the element being
    /// read; `None` outside a ledger entry.
    pub(super) fn entry_tail(&self) -> Option<&[String]> {
        (self.path.len() >= 6
            && self.path[..5] == ["ENVELOPE", "BODY", "DATA", "COLLECTION", "VOUCHER"]
            && self.path[5] == "ALLLEDGERENTRIES.LIST")
            .then(|| &self.path[6..])
    }
    /// Whether the element being read lies inside a `CATEGORYALLOCATIONS.LIST`.
    pub(super) fn in_cost_centres(&self) -> bool {
        self.entry_tail().is_some_and(|tail| {
            tail.first()
                .is_some_and(|c| c == "CATEGORYALLOCATIONS.LIST")
        })
    }
    /// Whether the element being read lies inside a `BANKALLOCATIONS.LIST`.
    pub(super) fn in_bank_allocations(&self) -> bool {
        self.entry_tail()
            .is_some_and(|tail| tail.first().is_some_and(|c| c == "BANKALLOCATIONS.LIST"))
    }
    /// The path below the current `VOUCHER`, for a voucher's own stock lines: `None` outside a
    /// voucher and inside any other list of it, such as a Stock Journal's in and out sides, which
    /// repeat the same lines without their godowns (protocol reference §8.2i).
    pub(super) fn stock_line_tail(&self) -> Option<&[String]> {
        (self.path.len() >= 5
            && self.path[..5] == ["ENVELOPE", "BODY", "DATA", "COLLECTION", "VOUCHER"]
            && self.path.get(5).is_none_or(|c| c == STOCK_LINE_LIST))
        .then(|| &self.path[5..])
    }
    /// Whether the element being read lies inside an `INVENTORYALLOCATIONS.LIST`.
    pub(super) fn in_goods_lines(&self) -> bool {
        self.entry_tail().is_some_and(|tail| {
            tail.first()
                .is_some_and(|c| c == "INVENTORYALLOCATIONS.LIST")
        })
    }
    /// Whether the element being read is a child of a voucher's Delivery Note entry.
    fn delivery_note_field(&self) -> bool {
        self.path.len() == 7
            && self.path[..5] == ["ENVELOPE", "BODY", "DATA", "COLLECTION", "VOUCHER"]
            && self.path[5] == DELIVERY_NOTE_LIST
    }
    pub(super) fn bill_allocation(&self) -> bool {
        self.path.len() == 7
            && self.path[..5] == ["ENVELOPE", "BODY", "DATA", "COLLECTION", "VOUCHER"]
            && self.path[5] == "ALLLEDGERENTRIES.LIST"
            && self.path[6] == "BILLALLOCATIONS.LIST"
    }
    pub(super) fn bill_allocation_field(&self) -> bool {
        self.path.len() == 8
            && self.path[..5] == ["ENVELOPE", "BODY", "DATA", "COLLECTION", "VOUCHER"]
            && self.path[5] == "ALLLEDGERENTRIES.LIST"
            && self.path[6] == "BILLALLOCATIONS.LIST"
    }
    pub(super) fn voucher_scalar(&self) -> bool {
        self.path.last().is_some_and(|field| {
            (self.field("VOUCHER") && is_voucher_scalar(field))
                || (self.entry_field() && is_voucher_entry_scalar(field))
                || (self.bill_allocation_field() && is_voucher_bill_allocation_scalar(field))
        })
    }
    pub(super) fn start(&mut self, name: String) {
        if self.path == ["ENVELOPE", "BODY", "DATA"] && name == "COLLECTION" {
            self.repeated_collection |= self.collection_seen;
            self.collection_seen = true;
        }
        self.path.push(name);
    }
    pub(super) fn end(&mut self, name: &str) -> Result<(), String> {
        if self.path.pop().as_deref() != Some(name) {
            return Err("agent_read_protocol_invalid".to_string());
        }
        Ok(())
    }
    pub(super) fn finish(&self) -> Result<(), String> {
        if self.collection_seen && !self.repeated_collection && self.path.is_empty() {
            Ok(())
        } else {
            Err("agent_read_protocol_invalid".to_string())
        }
    }
}

pub(super) fn parse_agent_rows(xml: &str, company_guid: &str) -> Result<Vec<Value>, String> {
    parse_agent_rows_with_accounting_state(xml, false, company_guid)
}

pub(super) fn parse_agent_changed_rows(
    xml: &str,
    company_guid: &str,
) -> Result<Vec<Value>, String> {
    parse_agent_rows_with_accounting_state(xml, true, company_guid)
}

/// [`parse_agent_changed_rows`] under [`CompositePolicy::NameLedger`]: an entry
/// whose amount is a currency composite carries `"currency_composite": true`
/// and its amount as Tally sent it, which no caller may sum. Only
/// `ledger_movement`'s first-day read of a several-currency book parses this
/// way (#683).
pub(super) fn parse_agent_changed_rows_naming_composite_ledgers(
    xml: &str,
    company_guid: &str,
) -> Result<Vec<Value>, String> {
    refused_composites(parse_voucher_rows(
        xml,
        true,
        false,
        company_guid,
        CompositePolicy::NameLedger,
        GstDetailsShown::Withheld,
        (NamedVoucherFields::NOT_ASKED, StockLines::NotAsked),
    )?)
}

/// Changed rows plus `effective_date`, which only import verification reads.
/// The public voucher tools do not fetch it and their rows do not carry it.
pub(super) fn parse_import_verification_rows(
    xml: &str,
    company_guid: &str,
) -> Result<Vec<Value>, String> {
    refused_composites(parse_voucher_rows(
        xml,
        true,
        true,
        company_guid,
        CompositePolicy::Refuse,
        GstDetailsShown::Withheld,
        (NamedVoucherFields::NOT_ASKED, StockLines::NotAsked),
    )?)
}

pub(super) fn parse_agent_rows_with_accounting_state(
    xml: &str,
    require_change_identity: bool,
    company_guid: &str,
) -> Result<Vec<Value>, String> {
    refused_composites(parse_voucher_rows(
        xml,
        require_change_identity,
        false,
        company_guid,
        CompositePolicy::Refuse,
        GstDetailsShown::Withheld,
        (NamedVoucherFields::NOT_ASKED, StockLines::NotAsked),
    )?)
}

/// [`parse_agent_rows`], reading each entry's GST details (#2027) only when `gst` asks for them,
/// so that every read that does not ask holds and returns exactly what it did before.
pub(super) fn parse_agent_rows_reading_gst(
    xml: &str,
    company_guid: &str,
    gst: GstDetailsShown,
    named: NamedVoucherFields,
    stock: StockLines,
) -> Result<Vec<Value>, String> {
    refused_composites(parse_voucher_rows(
        xml,
        false,
        false,
        company_guid,
        CompositePolicy::Refuse,
        gst,
        (named, stock),
    )?)
}

/// The `vouchers` rows (#674): a voucher whose amount Tally stored as a
/// foreign-currency composite is withheld whole, as a [`VoucherRow::Withheld`],
/// instead of failing the window. Every other check still applies to it, and
/// any other bad amount still fails the window. Nothing that sums, matches or
/// verifies amounts reads through here.
#[cfg(test)]
pub(super) fn parse_agent_rows_withholding(
    xml: &str,
    company_guid: &str,
) -> Result<Vec<VoucherRow>, String> {
    parse_agent_rows_withholding_reading_gst(
        xml,
        company_guid,
        GstDetailsShown::Withheld,
        NamedVoucherFields::NOT_ASKED,
        StockLines::NotAsked,
    )
}

/// The `vouchers` rows, withheld as `parse_agent_rows_withholding` withholds them, with each
/// entry's GST details read only when `gst` shows them (#2027).
pub(super) fn parse_agent_rows_withholding_reading_gst(
    xml: &str,
    company_guid: &str,
    gst: GstDetailsShown,
    named: NamedVoucherFields,
    stock: StockLines,
) -> Result<Vec<VoucherRow>, String> {
    Ok(parse_voucher_rows(
        xml,
        false,
        false,
        company_guid,
        CompositePolicy::Withhold,
        gst,
        (named, stock),
    )?
    .into_iter()
    .map(|(row, withheld)| {
        if withheld {
            VoucherRow::Withheld(WithheldVoucher::from_parsed(row))
        } else {
            VoucherRow::Read(row)
        }
    })
    .collect())
}

/// One detail of a voucher that its read named, as that read found it.
enum VoucherDetail<'row> {
    Present(&'row str),
    /// Tally sent the element, empty: the voucher does not carry this detail.
    Absent,
    /// Tally did not send the element, so this book or release does not say.
    NotReported,
}

impl<'row> VoucherDetail<'row> {
    fn of(row: &'row BTreeMap<String, String>, element: &str) -> Self {
        match row.get(element).map(|value| value.trim()) {
            None => Self::NotReported,
            Some("") => Self::Absent,
            Some(value) => Self::Present(value),
        }
    }

    /// A present value of an element Tally types as a date must be one, or the read is refused.
    fn require_date(&self, refusal: &str) -> Result<(), String> {
        if let Self::Present(date) = self {
            bridge_tally_core::TallyDate::parse((*date).to_string())
                .map_err(|_| refusal.to_string())?;
        }
        Ok(())
    }

    fn to_json(&self) -> Value {
        match self {
            Self::Present(value) => json!({"state": "present", "value": value}),
            Self::Absent => json!({"state": "absent"}),
            Self::NotReported => json!({"state": "not_reported"}),
        }
    }
}

/// A row says named details only for an invoice, or a voucher that carries one of them.
fn row_says_details(row: &BTreeMap<String, String>, details: &[&VoucherDetail]) -> bool {
    details
        .iter()
        .any(|detail| matches!(detail, VoucherDetail::Present(_)))
        || row.get("ISINVOICE").map(|value| value.trim()) == Some("Yes")
}

/// Whether a read's fetch list named the e-invoice details (#1378). Absent is said only when they
/// were asked for, so a read that did not ask returns exactly the rows it did before.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EInvoiceFields {
    Asked,
    NotAsked,
}

impl EInvoiceFields {
    /// From the `include_e_invoice` argument: absent or false does not ask.
    pub(super) fn from_args(args: &Value) -> Result<Self, String> {
        Ok(match optional_bool(args, "include_e_invoice")? {
            Some(true) => Self::Asked,
            Some(false) | None => Self::NotAsked,
        })
    }

    /// An invoice's details, or a voucher's if it carries any (protocol reference §8.2j).
    fn details(self, row: &BTreeMap<String, String>) -> Result<Option<Value>, String> {
        if self == Self::NotAsked {
            return Ok(None);
        }
        let irn = VoucherDetail::of(row, "IRN");
        let ack_no = VoucherDetail::of(row, "IRNACKNO");
        let ack_date = VoucherDetail::of(row, "IRNACKDATE");
        ack_date.require_date("voucher_irn_ack_date_invalid")?;
        if !row_says_details(row, &[&irn, &ack_no, &ack_date]) {
            return Ok(None);
        }
        Ok(Some(json!({
            "irn": irn.to_json(),
            "ack_no": ack_no.to_json(),
            "ack_date": ack_date.to_json(),
        })))
    }
}

/// Whether a `vouchers` call read each voucher's stock lines in place of its ledger entries
/// (#2016).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StockLines {
    Asked,
    NotAsked,
}

/// What needs a voucher's ledger entries, which the stock-line request does not return: each
/// would answer empty or wrong without saying so, so it is refused with this argument.
const NEEDS_LEDGER_ENTRIES: [&str; 8] = [
    "ledger",
    "amount",
    "suspense_tagged",
    "summarise_by",
    "bank_allocations",
    "goods_lines",
    "include_gst_details",
    "include_e_invoice",
];

/// The voucher types whose stock lines were captured on this request (protocol reference §8.2i).
/// A type the user made under one has no reserved name, so it is not among them.
const STOCK_LINE_RESERVED_TYPES: [&str; 4] = [
    "Stock Journal",
    "Delivery Note",
    "Receipt Note",
    CREDIT_NOTE_RESERVED_TYPE,
];
/// The one read type that can be accounts only, so it alone may answer no stock line.
const CREDIT_NOTE_RESERVED_TYPE: &str = "Credit Note";

impl StockLines {
    /// From the `stock_lines` argument: absent or false does not ask.
    pub(super) fn from_args(args: &Value) -> Result<Self, String> {
        if optional_bool(args, "stock_lines")? != Some(true) {
            return Ok(Self::NotAsked);
        }
        match NEEDS_LEDGER_ENTRIES.iter().find(|name| {
            args.get(**name)
                .is_some_and(|value| !value.is_null() && *value != Value::Bool(false))
        }) {
            Some(name) => Err(format!("voucher_stock_lines_excludes_{name}")),
            None => Ok(Self::Asked),
        }
    }

    /// Whether this request's lines are read for a voucher of `kind`, resolved by Tally in the
    /// same response. A kind no capture covers is named, never read on trust.
    fn covers(kind: Option<&ResolvedVoucherType>) -> bool {
        kind.is_some_and(|kind| STOCK_LINE_RESERVED_TYPES.contains(&kind.reserved_name.as_str()))
    }

    /// Whether a voucher of `kind`, read here, must hold a line: one that exists only to move
    /// stock and answers none is a read that failed, never a voucher that moved nothing. A
    /// cancelled one is taken to keep no line: a cancelled Delivery Note answered an empty list
    /// (protocol reference §8.2i); a cancelled Stock Journal or Receipt Note is unverified.
    fn needs_a_line(kind: Option<&ResolvedVoucherType>, cancelled: bool) -> bool {
        !cancelled && kind.is_some_and(|kind| kind.reserved_name != CREDIT_NOTE_RESERVED_TYPE)
    }
}

/// Whether a read's fetch list named a voucher's GST and note details (#2027). Absent is said only
/// when they were asked for, as for [`EInvoiceFields`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum VoucherGstFields {
    Asked,
    NotAsked,
}

impl VoucherGstFields {
    /// A call that shows each entry's GST details asks for these as well.
    pub(super) const fn following(shown: GstDetailsShown) -> Self {
        match shown {
            GstDetailsShown::Shown => Self::Asked,
            GstDetailsShown::Withheld => Self::NotAsked,
        }
    }

    /// An invoice's details, or a voucher's if it carries any (protocol reference §8.2k).
    fn details(self, row: &BTreeMap<String, String>) -> Result<Option<Value>, String> {
        if self == Self::NotAsked {
            return Ok(None);
        }
        let place_of_supply = VoucherDetail::of(row, "PLACEOFSUPPLY");
        let party_state = VoucherDetail::of(row, "STATENAME");
        let registration_type = VoucherDetail::of(row, "GSTREGISTRATIONTYPE");
        let note_reason = VoucherDetail::of(row, "GSTNATUREOFRETURN");
        let party_note_number = VoucherDetail::of(row, "VATPARTYTRANSRETURNNUMBER");
        let party_note_date = VoucherDetail::of(row, "VATPARTYTRANSRETURNDATE");
        party_note_date.require_date("voucher_party_note_date_invalid")?;
        let details = [
            &place_of_supply,
            &party_state,
            &registration_type,
            &note_reason,
            &party_note_number,
            &party_note_date,
        ];
        if !row_says_details(row, &details) {
            return Ok(None);
        }
        Ok(Some(json!({
            "place_of_supply": place_of_supply.to_json(),
            "party_state": party_state.to_json(),
            "registration_type": registration_type.to_json(),
            "note_reason": note_reason.to_json(),
            "party_note_number": party_note_number.to_json(),
            "party_note_date": party_note_date.to_json(),
        })))
    }

    /// An invoice's dispatch details, or a voucher's if it carries any, each named after the field
    /// of Tally's form whose value the lab found in it, and its Delivery Notes as
    /// [`DeliveryNoteCollector::finish`] gives them (protocol reference §8.2l).
    fn dispatch(
        self,
        row: &BTreeMap<String, String>,
        delivery_notes: Option<Vec<Value>>,
    ) -> Result<Option<Value>, String> {
        if self == Self::NotAsked {
            return Ok(None);
        }
        let doc_no = VoucherDetail::of(row, "BASICSHIPDOCUMENTNO");
        let dispatched_through = VoucherDetail::of(row, "BASICSHIPPEDBY");
        let destination = VoucherDetail::of(row, "BASICFINALDESTINATION");
        let carrier = VoucherDetail::of(row, "EICHECKPOST");
        let lading_no = VoucherDetail::of(row, "BILLOFLADINGNO");
        let lading_date = VoucherDetail::of(row, "BILLOFLADINGDATE");
        let vehicle_no = VoucherDetail::of(row, "BASICSHIPVESSELNO");
        lading_date.require_date("voucher_lading_date_invalid")?;
        let details = [
            &doc_no,
            &dispatched_through,
            &destination,
            &carrier,
            &lading_no,
            &lading_date,
            &vehicle_no,
        ];
        let carries_a_note = delivery_notes
            .as_ref()
            .is_some_and(|notes| !notes.is_empty());
        if !carries_a_note && !row_says_details(row, &details) {
            return Ok(None);
        }
        Ok(Some(json!({
            "doc_no": doc_no.to_json(),
            "dispatched_through": dispatched_through.to_json(),
            "destination": destination.to_json(),
            "carrier": carrier.to_json(),
            "lading_no": lading_no.to_json(),
            "lading_date": lading_date.to_json(),
            "vehicle_no": vehicle_no.to_json(),
            "delivery_notes": match delivery_notes {
                None => json!({"state": "not_reported"}),
                Some(notes) if notes.is_empty() => json!({"state": "absent"}),
                Some(notes) => json!({"state": "present", "value": notes}),
            },
        })))
    }
}

/// The list a voucher keeps its Delivery Notes in, one entry each.
const DELIVERY_NOTE_LIST: &str = "INVOICEDELNOTES.LIST";

/// The two children a [`DELIVERY_NOTE_LIST`] entry was measured with: a note's number and its date.
const DELIVERY_NOTE_FIELDS: [&str; 2] = ["BASICSHIPDELIVERYNOTE", "BASICSHIPPINGDATE"];

/// A voucher's Delivery Notes, read only when the GST details are asked for (#2027). Each entry of
/// [`DELIVERY_NOTE_LIST`] holds one note's number in `BASICSHIPDELIVERYNOTE` and its date in
/// `BASICSHIPPINGDATE`, in the order keyed, and a voucher with none sends one entry with no children
/// (protocol reference §8.2l).
#[derive(Default)]
struct DeliveryNoteCollector {
    /// `None` until the voucher sends the list.
    notes: Option<Vec<Value>>,
    entry: Option<BTreeMap<String, String>>,
}

impl DeliveryNoteCollector {
    fn start_entry(&mut self) {
        self.notes.get_or_insert_with(Vec::new);
        self.entry = Some(BTreeMap::new());
    }

    /// A child of an entry is one of the two measured, read once, as a voucher field is. Any other
    /// refuses the read, so a note under an unmeasured element is never said to be no note.
    fn start_field(&mut self, tag: &str) -> Result<(), String> {
        match self.entry.as_mut() {
            Some(entry) if DELIVERY_NOTE_FIELDS.contains(&tag) && !entry.contains_key(tag) => {
                entry.insert(tag.to_string(), String::new());
                Ok(())
            }
            _ => Err("voucher_delivery_note_invalid".to_string()),
        }
    }

    fn text(&mut self, tag: &str, value: String) {
        if let Some(entry) = self.entry.as_mut() {
            append_agent_text(entry, tag, value);
        }
    }

    /// Only an entry with no children is the empty one; any other is a note, each half stated.
    fn end_entry(&mut self) -> Result<(), String> {
        let entry = self.entry.take().unwrap_or_default();
        if entry.is_empty() {
            return Ok(());
        }
        let number = VoucherDetail::of(&entry, DELIVERY_NOTE_FIELDS[0]);
        let date = VoucherDetail::of(&entry, DELIVERY_NOTE_FIELDS[1]);
        date.require_date("voucher_delivery_note_date_invalid")?;
        self.notes
            .get_or_insert_with(Vec::new)
            .push(json!({"number": number.to_json(), "date": date.to_json()}));
        Ok(())
    }

    /// The voucher's notes: `None` if it sent no list, empty if the list held none.
    fn finish(&mut self) -> Option<Vec<Value>> {
        self.entry = None;
        self.notes.take()
    }
}

/// The voucher fields a read's fetch list names beyond the plain list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct NamedVoucherFields {
    pub(super) e_invoice: EInvoiceFields,
    pub(super) gst: VoucherGstFields,
}

impl NamedVoucherFields {
    pub(super) const NOT_ASKED: Self = Self {
        e_invoice: EInvoiceFields::NotAsked,
        gst: VoucherGstFields::NotAsked,
    };

    /// Whether the list names anything: a const `self != Self::NOT_ASKED`.
    pub(super) const fn names_any(self) -> bool {
        matches!(self.e_invoice, EInvoiceFields::Asked)
            || matches!(self.gst, VoucherGstFields::Asked)
    }
}

/// What an amount that is not a plain decimal does to its voucher.
#[derive(Clone, Copy, PartialEq, Eq)]
enum CompositePolicy {
    /// The window fails, as it always has: every caller that uses amounts.
    Refuse,
    /// A Tally composite withholds its voucher; anything else still fails.
    Withhold,
    /// `ledger_movement` on a several-currency book at its first day (#683): a
    /// composite entry is kept with no figure and marked, so its ledger is set
    /// aside by name. The movement's window answers each bill allocation
    /// amount-only, as its fetch names no allocation field (§8.2n), and an
    /// amount-only allocation is a placeholder, skipped under every policy. One
    /// with a `BILLTYPE` and a composite amount, which that window has not been
    /// seen to send, still fails it. Anything else still fails.
    NameLedger,
}

impl CompositePolicy {
    fn withholds(self, amount: &str) -> bool {
        self == Self::Withhold && is_currency_composite(amount)
    }

    fn names_ledger(self, amount: &str) -> bool {
        self == Self::NameLedger && is_currency_composite(amount)
    }
}

/// A composite as a voucher amount carries it: the shared classifier, with one
/// sign on both amounts.
fn is_currency_composite(amount: &str) -> bool {
    is_balance_composite(amount) && composite_signs_agree(amount)
}

/// A voucher with an amount Tally stored as a composite, read with the base amount as `amount`
/// and the foreign amount, its currency and the rate apart under `foreign` (#683). `None`, so the
/// voucher stays withheld, for a composite [`parse_currency_composite`] refuses or that writes no
/// amount, a foreign symbol that is not one of the book's masters, or base entries that do not
/// net to zero.
fn read_composites(row: &Value, book: &IdentifiedBaseCurrency) -> Option<Value> {
    let mut row = row.clone();
    let mut net = ExactDecimal::zero();
    for entry in row["amounts"].as_array_mut()? {
        read_composite_amount(entry, book)?;
        for allocation in entry["bill_allocations"].as_array_mut()? {
            read_composite_amount(allocation, book)?;
        }
        let amount = ExactDecimal::parse(entry["amount"].as_str()?.to_string()).ok()?;
        // A composite parsed to no amount, so the parser could not compare it.
        if !tally_entry_polarity_agrees(&amount, entry["is_deemed_positive"] == "Yes") {
            entry["polarity_disagrees_with_amount"] = Value::Bool(true);
        }
        net = net.checked_add(&amount).ok()?;
    }
    net.is_zero().then_some(row)
}

fn read_composite_amount(holder: &mut Value, book: &IdentifiedBaseCurrency) -> Option<()> {
    let amount = holder["amount"].as_str()?;
    if ExactDecimal::parse(amount.to_string()).is_ok() {
        return Some(());
    }
    let CurrencyComposite::Valued(composite) =
        parse_currency_composite(amount, book.base()).ok()?
    else {
        return None;
    };
    if !book.is_foreign_master(composite.foreign_symbol()) {
        return None;
    }
    let sign = match composite.side() {
        Side::Positive => "",
        Side::Negative => "-",
    };
    holder["amount"] = json!(format!("{sign}{}", composite.base().magnitude().as_str()));
    holder["foreign"] = json!({
        "currency": composite.foreign_symbol(),
        "amount": format!("{sign}{}", composite.foreign().magnitude().as_str()),
        "rate": composite.rate().as_str(),
    });
    Some(())
}

/// A voucher entry's composite carries one sign on both amounts, as every
/// captured one does; a zero foreign amount (a base-only adjustment) carries
/// none. The shared classifier does not compare signs, since a ledger balance
/// can pair opposite ones. Only called on a string it accepted.
fn composite_signs_agree(composite: &str) -> bool {
    let Some((foreign, rest)) = composite.split_once(" @ ") else {
        return false;
    };
    let Some((_, base)) = rest.split_once(" = ") else {
        return false;
    };
    // The amount follows its symbol, with a space in the 7.1 layout and none in the 7.2 one.
    let foreign_is_zero = foreign
        .rsplit(|c: char| !c.is_ascii_digit() && c != '.')
        .next()
        .is_some_and(|digits| digits.bytes().all(|b| b == b'0' || b == b'.'));
    foreign_is_zero || foreign.starts_with('-') == base.starts_with('-')
}

/// Under [`CompositePolicy::Refuse`] no row is ever withheld: a composite has
/// already failed the parse with its amount code.
fn refused_composites(rows: Vec<(Value, bool)>) -> Result<Vec<Value>, String> {
    rows.into_iter()
        .map(|(row, withheld)| {
            if withheld {
                Err("agent_read_protocol_invalid".to_string())
            } else {
                Ok(row)
            }
        })
        .collect()
}

/// A `vouchers` row: read in full, or withheld because an amount was stored as
/// a foreign-currency composite (#674). A withheld voucher shows no amount,
/// so no path that sums, matches or verifies amounts can take one, until
/// [`Self::read_under`] has read it.
#[derive(Debug, Clone)]
pub(super) enum VoucherRow {
    Read(Value),
    Withheld(WithheldVoucher),
}

/// A withheld voucher: its identity, type fields and entry ledgers, never an
/// amount. The fields are private; [`Self::filter_view`] is the only way out
/// but for [`VoucherRow::read_under`].
#[derive(Clone)]
pub(super) struct WithheldVoucher {
    view: Value,
    /// The parsed voucher, composites as Tally wrote them.
    parsed: Value,
}

/// The view alone, so no debug output shows an amount this type withholds.
impl std::fmt::Debug for WithheldVoucher {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("WithheldVoucher")
            .field(&self.view)
            .finish()
    }
}

/// Why a voucher is withheld; the only cause so far.
pub(super) const WITHHELD_FOREIGN_CURRENCY: &str = "foreign_currency_amount_unparsed";

/// Marks a row in [`WithheldVoucher::filter_view`]; the parser never emits it.
pub(super) const WITHHELD_MARKER: &str = "withheld_cause";

impl WithheldVoucher {
    fn from_parsed(parsed: Value) -> Self {
        let mut row = parsed.clone();
        // Keep each entry's ledger (the ledger filter reads it) and drop
        // everything that carries an amount.
        let ledgers = row["amounts"]
            .as_array()
            .map(|entries| {
                entries
                    .iter()
                    .map(|entry| json!({"ledger": entry["ledger"]}))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        row["amounts"] = Value::Array(ledgers);
        row[WITHHELD_MARKER] = json!(WITHHELD_FOREIGN_CURRENCY);
        Self { view: row, parsed }
    }

    /// The row as the date, ledger and voucher-type filters read it: the parsed
    /// voucher with each entry cut to its ledger, marked withheld, with no
    /// amount. The view never leaves the tool: `vouchers` lists a withheld
    /// voucher by GUID, date, type, number and cause only.
    pub(super) fn filter_view(&self) -> Value {
        self.view.clone()
    }
}

impl super::voucher_window::WindowRow for VoucherRow {
    fn window_date(&self) -> Option<&str> {
        self.row().window_date()
    }
    fn window_alter_id(&self) -> Option<u64> {
        self.row().window_alter_id()
    }
    fn window_guid(&self) -> Option<&str> {
        self.row().window_guid()
    }
    fn window_master_id(&self) -> Result<Option<u64>, String> {
        self.row().window_master_id()
    }
}

impl VoucherRow {
    /// The row the date, ledger and voucher-type filters read: a read voucher
    /// whole, a withheld one as its [`WithheldVoucher::filter_view`].
    pub(super) fn is_withheld(&self) -> bool {
        matches!(self, Self::Withheld(_))
    }

    /// A withheld voucher read in a book whose base and masters are known, or left withheld
    /// where [`read_composites`] will not read it.
    pub(super) fn read_under(self, book: &IdentifiedBaseCurrency) -> Self {
        match self {
            Self::Withheld(withheld) => match read_composites(&withheld.parsed, book) {
                Some(row) => Self::Read(row),
                None => Self::Withheld(withheld),
            },
            read => read,
        }
    }

    pub(super) fn into_filter_row(self) -> Value {
        match self {
            Self::Read(row) => row,
            Self::Withheld(withheld) => withheld.filter_view(),
        }
    }

    fn row(&self) -> &Value {
        match self {
            Self::Read(row) => row,
            Self::Withheld(withheld) => &withheld.view,
        }
    }
}

fn parse_voucher_rows(
    xml: &str,
    require_change_identity: bool,
    include_effective_date: bool,
    company_guid: &str,
    composites: CompositePolicy,
    gst_details: GstDetailsShown,
    (named, stock): (NamedVoucherFields, StockLines),
) -> Result<Vec<(Value, bool)>, String> {
    // An entry's GST details are read only for a call that shows them (#2027): a read that does
    // not ask holds, sizes and returns exactly the rows it did before.
    let read_gst = gst_details == GstDetailsShown::Shown;
    let read_stock = stock == StockLines::Asked;
    let read_notes = named.gst == VoucherGstFields::Asked;
    // Tally's collection XML varies by release; use a deliberately conservative
    // extractor and never infer a missing field. Malformed rows fail before
    // optional selectors can hide them as an apparently complete empty result.
    let marked = mark_agent_xml(xml);
    let xml = marked.as_ref();
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    let mut rows = Vec::new();
    let mut identities = VoucherSourceIdentities::default();
    validate_agent_envelope(xml)?;
    let mut current: Option<BTreeMap<String, String>> = None;
    let mut entry: Option<BTreeMap<String, String>> = None;
    let mut allocation: Option<BTreeMap<String, String>> = None;
    let mut entries = Vec::<Value>::new();
    let mut allocations = Vec::<Value>::new();
    let mut cost = CostCentreCollector::default();
    let mut bank = BankAllocationCollector::default();
    let mut goods = GoodsLineCollector::default();
    let mut stock_lines = GoodsLineCollector::of_list(GoodsLineList::VoucherStockLines);
    let mut gst = EntryGstCollector::default();
    let mut notes = DeliveryNoteCollector::default();
    let mut current_tag = String::new();
    let mut scope = NativeCollectionScope::default();
    // Set by a composite amount under `CompositePolicy::Withhold`, per voucher.
    let mut withheld = false;
    loop {
        match reader.read_event() {
            Ok(quick_xml::events::Event::Start(event)) => {
                let tag = event.name().as_ref().to_ascii_uppercase();
                if scope.voucher_scalar() || (scope.collection() && tag != "VOUCHER") {
                    return Err("agent_read_protocol_invalid".into());
                }
                if tag == "VOUCHER" && scope.collection() {
                    let mut row = BTreeMap::new();
                    for attribute in event.attributes() {
                        let attribute =
                            attribute.map_err(|_| "agent_read_protocol_invalid".to_string())?;
                        if attribute.key.as_ref().eq_ignore_ascii_case("REMOTEID") {
                            claim_agent_scalar(&mut row, "REMOTEID")?;
                            row.insert(
                                "REMOTEID".into(),
                                attribute
                                    .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                                    .map_err(|_| "agent_read_protocol_invalid".to_string())?
                                    .into_owned(),
                            );
                        }
                    }
                    current = Some(row);
                    entries.clear();
                    withheld = false;
                    stock_lines.reset();
                    notes = DeliveryNoteCollector::default();
                }
                if read_notes {
                    start_delivery_note_element(&scope, &tag, &mut notes)?;
                }
                if tag == "ALLLEDGERENTRIES.LIST" && scope.row("VOUCHER") {
                    entry = Some(BTreeMap::new());
                    allocations.clear();
                    cost.reset();
                    bank.reset();
                    goods.reset();
                    gst.reset();
                }
                if let Some(tail) = scope.entry_tail() {
                    cost.start(tail, &tag);
                    bank.start(tail, &tag);
                    goods.start(tail, &tag);
                    gst.start(tail, &tag);
                }
                if let Some(tail) = scope.stock_line_tail().filter(|_| read_stock) {
                    stock_lines.start(tail, &tag);
                }
                if tag == "BILLALLOCATIONS.LIST" && scope.child("VOUCHER", "ALLLEDGERENTRIES.LIST")
                {
                    allocation = Some(BTreeMap::new());
                }
                claim_voucher_scalar(
                    &scope,
                    &tag,
                    current.as_mut(),
                    entry.as_mut(),
                    allocation.as_mut(),
                )?;
                scope.start(tag.clone());
                if scope.repeated_collection {
                    return Err("agent_read_protocol_invalid".into());
                }
                current_tag = tag;
            }
            Ok(quick_xml::events::Event::Text(text)) => {
                if read_notes && scope.delivery_note_field() {
                    notes.text(&current_tag, decoded_agent_text(text.clone())?);
                }
                if let Some(tail) = scope.entry_tail().filter(|_| read_gst) {
                    match decoded_agent_text(text.clone()) {
                        Ok(value) => gst.text(tail, &current_tag, &value),
                        Err(_) => gst.undecodable(tail),
                    }
                }
                if let Some(tail) = scope.stock_line_tail().filter(|_| read_stock) {
                    match decoded_agent_text(text.clone()) {
                        Ok(value) => stock_lines.text(tail, &current_tag, &value),
                        Err(_) => stock_lines.undecodable(),
                    }
                }
                if scope.in_cost_centres() {
                    match decoded_agent_text(text.clone()) {
                        Ok(value) => {
                            cost.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value)
                        }
                        Err(_) => cost.undecodable(),
                    }
                }
                if scope.in_bank_allocations() {
                    match decoded_agent_text(text.clone()) {
                        Ok(value) => {
                            bank.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value)
                        }
                        Err(_) => bank.undecodable(),
                    }
                }
                if scope.in_goods_lines() {
                    match decoded_agent_text(text.clone()) {
                        Ok(value) => {
                            goods.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value)
                        }
                        Err(_) => goods.undecodable(),
                    }
                }
                if let Some(row) = allocation
                    .as_mut()
                    .filter(|_| scope.bill_allocation_field())
                {
                    append_agent_text(row, &current_tag, decoded_agent_text(text)?);
                } else if let Some(row) = entry.as_mut().filter(|_| scope.entry_field()) {
                    append_agent_text(row, &current_tag, decoded_agent_text(text)?);
                } else if let Some(row) = current.as_mut().filter(|_| scope.field("VOUCHER")) {
                    append_agent_text(row, &current_tag, decoded_agent_text(text)?);
                }
            }
            Ok(quick_xml::events::Event::CData(text)) => {
                let value = text.to_string();
                if read_notes && scope.delivery_note_field() {
                    notes.text(&current_tag, value.clone());
                }
                if let Some(tail) = scope.stock_line_tail().filter(|_| read_stock) {
                    stock_lines.text(tail, &current_tag, &value);
                }
                if let Some(tail) = scope.entry_tail().filter(|_| read_gst) {
                    gst.text(tail, &current_tag, &value);
                }
                if scope.in_cost_centres() {
                    cost.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value);
                }
                if scope.in_bank_allocations() {
                    bank.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value);
                }
                if scope.in_goods_lines() {
                    goods.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value);
                }
                if let Some(row) = allocation
                    .as_mut()
                    .filter(|_| scope.bill_allocation_field())
                {
                    append_agent_text(row, &current_tag, value);
                } else if let Some(row) = entry.as_mut().filter(|_| scope.entry_field()) {
                    append_agent_text(row, &current_tag, value);
                } else if let Some(row) = current.as_mut().filter(|_| scope.field("VOUCHER")) {
                    append_agent_text(row, &current_tag, value);
                }
            }
            Ok(quick_xml::events::Event::GeneralRef(reference)) => {
                if read_notes && scope.delivery_note_field() {
                    notes.text(&current_tag, decoded_agent_reference(reference.clone())?);
                }
                if let Some(tail) = scope.stock_line_tail().filter(|_| read_stock) {
                    match decoded_agent_reference(reference.clone()) {
                        Ok(value) => stock_lines.text(tail, &current_tag, &value),
                        Err(_) => stock_lines.undecodable(),
                    }
                }
                if let Some(tail) = scope.entry_tail().filter(|_| read_gst) {
                    match decoded_agent_reference(reference.clone()) {
                        Ok(value) => gst.text(tail, &current_tag, &value),
                        Err(_) => gst.undecodable(tail),
                    }
                }
                if scope.in_cost_centres() {
                    match decoded_agent_reference(reference.clone()) {
                        Ok(value) => {
                            cost.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value)
                        }
                        Err(_) => cost.undecodable(),
                    }
                }
                if scope.in_bank_allocations() {
                    match decoded_agent_reference(reference.clone()) {
                        Ok(value) => {
                            bank.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value)
                        }
                        Err(_) => bank.undecodable(),
                    }
                }
                if scope.in_goods_lines() {
                    match decoded_agent_reference(reference.clone()) {
                        Ok(value) => {
                            goods.text(scope.entry_tail().unwrap_or(&[]), &current_tag, &value)
                        }
                        Err(_) => goods.undecodable(),
                    }
                }
                if let Some(row) = allocation
                    .as_mut()
                    .filter(|_| scope.bill_allocation_field())
                {
                    append_agent_text(row, &current_tag, decoded_agent_reference(reference)?);
                } else if let Some(row) = entry.as_mut().filter(|_| scope.entry_field()) {
                    append_agent_text(row, &current_tag, decoded_agent_reference(reference)?);
                } else if let Some(row) = current.as_mut().filter(|_| scope.field("VOUCHER")) {
                    append_agent_text(row, &current_tag, decoded_agent_reference(reference)?);
                }
            }
            Ok(quick_xml::events::Event::End(event)) => {
                let end = event.name().as_ref().to_ascii_uppercase();
                if read_notes && scope.child("VOUCHER", DELIVERY_NOTE_LIST) {
                    notes.end_entry()?;
                }
                if let Some(tail) = scope.stock_line_tail().filter(|_| read_stock) {
                    stock_lines.end(tail);
                }
                if let Some(tail) = scope.entry_tail() {
                    cost.end(tail);
                    bank.end(tail);
                    goods.end(tail);
                    gst.end(tail);
                }
                if scope.bill_allocation() {
                    if let Some(allocation_row) = allocation.take().filter(|row| !row.is_empty()) {
                        // Tally emits an amount-only container for a ledger entry with no
                        // typed allocation. It is NOT empty, so the filter above admits
                        // it, and requiring BILLTYPE unconditionally aborted the entire
                        // read over a row that carries nothing to record. The rule for
                        // which untyped rows are placeholders lives in
                        // `bill_allocation_without_type_is_placeholder` rather
                        // than being restated here.
                        //
                        // A skipped row must still fall through to `scope.end` below, so
                        // this is an `if let` and not a `continue`: continuing the event
                        // loop would leave the scope stack unbalanced and mis-attribute
                        // every element after it.
                        let bill_type = allocation_row
                            .get("BILLTYPE")
                            .filter(|value| !value.trim().is_empty());
                        if let Some(bill_type) = bill_type {
                            let amount = allocation_row
                                .get("AMOUNT")
                                .filter(|value| !value.trim().is_empty())
                                .ok_or_else(|| "bill_allocation_field_missing".to_string())?;
                            if bridge_tally_core::ExactDecimal::parse(amount.clone()).is_err() {
                                if composites.withholds(amount) {
                                    withheld = true;
                                } else {
                                    return Err("bill_allocation_amount_invalid".to_string());
                                }
                            }
                            let name = allocation_row
                                .get("NAME")
                                .filter(|value| !value.trim().is_empty());
                            let reference = if bill_type.trim() == "On Account" {
                                // On Account is the one bill type with no bill identity.
                                // Keep that absence explicit instead of representing it
                                // as an empty name.
                                //
                                // A NAME here is a contradiction, not a value to drop.
                                // Silently discarding it loses a supplier reference that
                                // a malformed response -- or a request-shape regression
                                // -- is trying to tell us about. Refuse it as
                                // `bill_reference_forbidden`.
                                if name.is_some() {
                                    return Err("bill_reference_forbidden".to_string());
                                }
                                json!({"kind": "on_account"})
                            } else {
                                let name = name.ok_or_else(|| {
                                    "bill_allocation_field_missing".to_string()
                                })?;
                                json!({"kind": "named", "name": name})
                            };
                            let mut parsed_allocation = json!({
                                "reference": reference,
                                "bill_type": bill_type,
                                "amount": amount,
                            });
                            // The bill's own date and credit period, which the wildcard
                            // fetch already returns on New Ref and Agst Ref allocations
                            // (live captures). An empty or absent element is not observed
                            // and the key is omitted, never invented as "" or a zero
                            // period. A present BILLDATE must be a date, as DATE must:
                            // a malformed one refuses the read.
                            if let Some(bill_date) = allocation_row
                                .get("BILLDATE")
                                .map(|value| value.trim())
                                .filter(|value| !value.is_empty())
                            {
                                bridge_tally_core::TallyDate::parse(bill_date.to_string())
                                    .map_err(|_| "bill_allocation_date_invalid".to_string())?;
                                parsed_allocation["bill_date"] = json!(bill_date);
                            }
                            if let Some(period) = allocation_row
                                .get("BILLCREDITPERIOD")
                                .map(|value| value.trim())
                                .filter(|value| !value.is_empty())
                            {
                                parsed_allocation["credit_period"] = credit_period_json(period);
                            }
                            allocations.push(parsed_allocation);
                        } else if !bridge_tally_protocol::outstandings_shared::bill_allocation_without_type_is_placeholder(
                            allocation_row.get("NAME").map(String::as_str),
                        ) {
                            // A named bill with no type is partially populated, not a
                            // placeholder. Guessing the type would invent an allocation.
                            return Err("bill_allocation_field_missing".to_string());
                        }
                    }
                } else if scope.child("VOUCHER", "ALLLEDGERENTRIES.LIST") {
                    if allocation.is_some() {
                        return Err("agent_read_protocol_invalid".to_string());
                    }
                    // A Stock Journal moves inventory and has no accounting effect, so
                    // Tally returns an ALLLEDGERENTRIES.LIST with no children at all.
                    // That is an empty entry list, not a malformed entry, and refusing
                    // it lost the whole window over a voucher that is exactly right.
                    // The same distinction is already drawn one level down for
                    // BILLALLOCATIONS.LIST. A row carrying *some* of the three fields
                    // stays refused below: that is what a truncated response or a
                    // request-shape regression looks like, and it must stay loud.
                    if let (Some(_), Some(entry_row)) =
                        (current.as_mut(), entry.take().filter(|row| !row.is_empty()))
                    {
                        let ledger = entry_row
                            .get("LEDGERNAME")
                            .filter(|value| !value.trim().is_empty())
                            .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
                        let amount = entry_row
                            .get("AMOUNT")
                            .filter(|value| !value.trim().is_empty())
                            .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
                        let mut entry_composite = false;
                        let parsed_amount =
                            match bridge_tally_core::ExactDecimal::parse(amount.clone()) {
                                Ok(parsed) => Some(parsed),
                                Err(_) if composites.withholds(amount) => {
                                    withheld = true;
                                    None
                                }
                                Err(_) if composites.names_ledger(amount) => {
                                    entry_composite = true;
                                    None
                                }
                                Err(_) => return Err("voucher_amount_invalid".to_string()),
                            };

                        let polarity = entry_row
                            .get("ISDEEMEDPOSITIVE")
                            .filter(|value| !value.trim().is_empty())
                            .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
                        let is_deemed_positive = required_tally_bool(Some(polarity))?;
                        // AMOUNT carries the sign; ISDEEMEDPOSITIVE records the column the
                        // entry was made in, and the two legitimately disagree. A rounding
                        // ledger is flagged with the voucher's default side while the
                        // rounding itself goes either way. Refusing the disagreement threw
                        // away the whole window over entries whose arithmetic is right --
                        // summing AMOUNT alone reproduces Tally's own closing balance for
                        // the ledger, and honouring the flag does not.
                        //
                        // The disagreement is still worth saying out loud, so it is carried
                        // on the entry rather than dropped. It is only present when the two
                        // observations conflict, so an ordinary entry is unchanged.
                        let mut parsed_entry = json!({
                            "ledger": ledger,
                            "amount": amount,
                            "is_deemed_positive": if is_deemed_positive { "Yes" } else { "No" },
                            "bill_allocations": std::mem::take(&mut allocations),
                        });
                        // A cost-centre list that cannot be read whole is not shown in
                        // part and does not refuse the read: the entry says why.
                        match cost.finish() {
                            Ok(rows) if rows.is_empty() => {}
                            Ok(rows) => {
                                parsed_entry["cost_centre_allocations"] = Value::Array(rows)
                            }
                            Err(problem) => {
                                parsed_entry["cost_centre_allocations_unreadable"] =
                                    json!(problem.reason());
                            }
                        }
                        // The same for bank allocations (five fields, each absent, empty or a
                        // value); an entry with no populated allocation has no key. Every answer
                        // removes them unless `vouchers` was asked for them (#1231).
                        match bank.finish() {
                            Ok(rows) if rows.is_empty() => {}
                            Ok(rows) => parsed_entry["bank_allocations"] = Value::Array(rows),
                            Err(problem) => {
                                parsed_entry["bank_allocations_unreadable"] =
                                    json!(problem.reason());
                            }
                        }
                        // The same for goods lines (#2015): an invoice's item lines with their
                        // godown and batch split, withheld unless `vouchers` asks for them.
                        match goods.finish() {
                            Ok(rows) if rows.is_empty() => {}
                            Ok(rows) => parsed_entry["goods_lines"] = Value::Array(rows),
                            Err(problem) => {
                                parsed_entry["goods_lines_unreadable"] = json!(problem.reason());
                            }
                        }
                        // The same for the entry's GST details (#2027): its rates, taxability and
                        // type of supply as Tally sent them, withheld unless `vouchers` asks.
                        match gst.finish() {
                            _ if !read_gst => {}
                            Ok(None) => {}
                            Ok(Some(details)) => parsed_entry["gst_details"] = details,
                            Err(problem) => {
                                parsed_entry["gst_details_unreadable"] = json!(problem.reason());
                            }
                        }
                        if parsed_amount.as_ref().is_some_and(|parsed| {
                            !tally_entry_polarity_agrees(parsed, is_deemed_positive)
                        }) {
                            parsed_entry["polarity_disagrees_with_amount"] = Value::Bool(true);
                        }
                        if entry_composite {
                            parsed_entry["currency_composite"] = Value::Bool(true);
                        }
                        entries.push(parsed_entry);
                    }
                } else if scope.row("VOUCHER") {
                    if let Some(row) = current.take() {
                        if ["DATE", "VOUCHERTYPENAME"].iter().any(|field| {
                            row.get(*field).is_none_or(|value| value.trim().is_empty())
                        }) {
                            return Err(if require_change_identity {
                                "change_row_core_field_invalid"
                            } else {
                                "agent_read_protocol_invalid"
                            }
                            .to_string());
                        }
                        if !row.get("GUID").is_some_and(|guid| {
                            bridge_tally_protocol::master_guid_belongs_to_company(
                                guid,
                                company_guid,
                            )
                        }) {
                            return Err("voucher_company_identity_invalid".to_string());
                        }
                        bridge_tally_core::TallyDate::parse(row["DATE"].clone())
                            .map_err(|_| "voucher_date_invalid".to_string())?;
                        let master_id = parse_optional_tally_u64(
                            row.get("MASTERID").map(String::as_str),
                            "voucher_master_id_invalid",
                        )?;
                        identities.admit(row.get("GUID").map(String::as_str), master_id)?;
                        // Emitted as the parsed number's text, never Tally's
                        // padded form (" 1"), so it equals the same id read
                        // elsewhere, as import verification already emits it (#989).
                        let master_id = master_id.map(|id| id.to_string());
                        let amounts = std::mem::take(&mut entries);
                        let mut parsed = json!({"date": row.get("DATE"), "voucher_number": row.get("VOUCHERNUMBER"), "voucher_type": row.get("VOUCHERTYPENAME"), "party": row.get("PARTYLEDGERNAME"), "narration": row.get("NARRATION"), "guid": row.get("GUID"), "alter_id": parse_optional_tally_alter_id(row.get("ALTERID").map(String::as_str))?, "master_id": master_id, "amounts": amounts});
                        // Present only when the read asked Tally to resolve the
                        // row's voucher type (bridge#625).
                        let kind = resolve_row_voucher_type(&row, company_guid)?;
                        let cancelled = required_tally_bool(row.get("ISCANCELLED"))?;
                        if read_stock {
                            if !StockLines::covers(kind.as_ref()) {
                                parsed["stock_lines_not_read"] = Value::Bool(true);
                            } else {
                                match stock_lines.finish() {
                                    Ok(lines)
                                        if lines.is_empty()
                                            && StockLines::needs_a_line(
                                                kind.as_ref(),
                                                cancelled,
                                            ) =>
                                    {
                                        parsed["stock_lines_unreadable"] =
                                            json!(GoodsLineProblem::StockLineMissing.reason());
                                    }
                                    Ok(lines) => parsed["stock_lines"] = Value::Array(lines),
                                    Err(problem) => {
                                        parsed["stock_lines_unreadable"] = json!(problem.reason());
                                    }
                                }
                            }
                        }
                        if let Some(resolved) = kind {
                            parsed["voucher_type_guid"] = json!(resolved.guid);
                            parsed["voucher_type_reserved_name"] = json!(resolved.reserved_name);
                            parsed["voucher_class"] =
                                json!(resolved.class.map(ReservedVoucherClass::name));
                        }
                        if require_change_identity {
                            parsed["remote_id"] = json!(row.get("REMOTEID"));
                        }
                        if include_effective_date {
                            // An empty element is treated like an absent one: not observed.
                            // A present value must be a date, as DATE must: a malformed
                            // one refuses the read rather than passing as unobserved.
                            if let Some(effective) = row
                                .get("EFFECTIVEDATE")
                                .map(|value| value.trim())
                                .filter(|value| !value.is_empty())
                            {
                                bridge_tally_core::TallyDate::parse(effective.to_string())
                                    .map_err(|_| "voucher_effective_date_invalid".to_string())?;
                                parsed["effective_date"] = json!(effective);
                            }
                        }
                        parsed["cancelled"] = Value::Bool(cancelled);
                        parsed["optional"] =
                            Value::Bool(required_tally_bool(row.get("ISOPTIONAL"))?);
                        // Unlike ISCANCELLED/ISOPTIONAL, ISPOSTDATED is left out by a source
                        // that does not report it: one old committed capture did, because it
                        // was taken before the tag was in the fetch list and never asked for
                        // it (protocol reference 8.2c records Tally asserting it on every
                        // voucher once the fetch list asks). required_tally_bool would refuse
                        // the whole read on that shape; that is right for a tag known to always
                        // be present, but wrong here; it would turn "Tally did not say" into a
                        // hard failure instead of a legible unknown. So this field is optional like
                        // EFFECTIVEDATE above: an absent or empty element is not observed and
                        // the key is omitted, never invented as `false`. A present value must
                        // be Yes/No: an unrecognised value refuses the read rather than
                        // guessing, exactly as a malformed EFFECTIVEDATE does.
                        if let Some(post_dated) = row
                            .get("ISPOSTDATED")
                            .map(|value| value.trim())
                            .filter(|value| !value.is_empty())
                        {
                            parsed["post_dated"] = Value::Bool(match post_dated {
                                "Yes" => true,
                                "No" => false,
                                _ => return Err("voucher_post_dated_invalid".to_string()),
                            });
                        }
                        // REFERENCE is TYPE="String", the same shape NARRATION uses, but
                        // unlike NARRATION a blank reference carries no information worth
                        // returning: protocol reference §8.2c observed it empty on most
                        // vouchers and populated with a manual reference number on the
                        // rest. An empty or absent element is not observed and the key is
                        // omitted, never emitted as "".
                        if let Some(reference) = row
                            .get("REFERENCE")
                            .map(|value| value.trim())
                            .filter(|value| !value.is_empty())
                        {
                            parsed["reference"] = json!(reference);
                        }
                        // REFERENCEDATE (#1257) is the date of the supplier's invoice a voucher's
                        // REFERENCE names. Like EFFECTIVEDATE above, an empty or absent element is
                        // not observed and the key is omitted, never invented as ""; a present value
                        // must be a date, as DATE must, or the read is refused. Tally sent the
                        // element on every voucher with `TYPE="Date"`, empty or `YYYYMMDD`, in the
                        // one live read of one synthetic book (protocol reference §8.2f); other
                        // forms are accepted if they hold a date but have not been observed.
                        if let Some(reference_date) = row
                            .get("REFERENCEDATE")
                            .map(|value| value.trim())
                            .filter(|value| !value.is_empty())
                        {
                            bridge_tally_core::TallyDate::parse(reference_date.to_string())
                                .map_err(|_| "voucher_reference_date_invalid".to_string())?;
                            parsed["reference_date"] = json!(reference_date);
                        }
                        // ISINVOICE follows ISPOSTDATED's optional-boolean idiom exactly:
                        // absent or empty is "Tally did not say" and the key is omitted,
                        // never invented as false; a present value must be Yes/No. §8.2c's
                        // capture asserted it (with one of those two values) on every
                        // voucher observed, unlike ISPOSTDATED, but that is one instance on
                        // one release and is not grounds to promote it to
                        // required_tally_bool. §8.2c also notes ISINVOICE is the one
                        // logical here Tally emits without a TYPE="Logical" attribute;
                        // parsing here matches on tag name only, so that is not visible to
                        // this code and changes nothing about it.
                        if let Some(is_invoice) = row
                            .get("ISINVOICE")
                            .map(|value| value.trim())
                            .filter(|value| !value.is_empty())
                        {
                            parsed["is_invoice"] = Value::Bool(match is_invoice {
                                "Yes" => true,
                                "No" => false,
                                _ => return Err("voucher_is_invoice_invalid".to_string()),
                            });
                        }
                        // PARTYGSTIN is TYPE="String", handled like REFERENCE above: an
                        // empty or absent element is not observed. §8.2c's capture proved
                        // the tag round-trips through this FETCH list but never observed a
                        // populated value (this synthetic company's parties carry no
                        // GSTIN) -- presence is verified, population is not, and this
                        // parsing makes no claim about what a populated value looks like.
                        if let Some(party_gstin) = row
                            .get("PARTYGSTIN")
                            .map(|value| value.trim())
                            .filter(|value| !value.is_empty())
                        {
                            parsed["party_gstin"] = json!(party_gstin);
                        }
                        if let Some(details) = named.e_invoice.details(&row)? {
                            parsed["e_invoice"] = details;
                        }
                        if let Some(details) = named.gst.details(&row)? {
                            parsed["voucher_gst"] = details;
                        }
                        if let Some(details) = named.gst.dispatch(&row, notes.finish())? {
                            parsed["dispatch"] = details;
                        }
                        rows.push((parsed, withheld));
                    }
                }
                scope.end(&end)?;
                current_tag.clear();
            }
            Ok(quick_xml::events::Event::Empty(event)) => {
                let name = event.name().as_ref().to_ascii_uppercase();
                if scope.voucher_scalar() {
                    return Err("agent_read_protocol_invalid".into());
                }
                if scope.collection() {
                    return Err("agent_read_protocol_invalid".to_string());
                }
                if read_notes {
                    start_delivery_note_element(&scope, &name, &mut notes)?;
                }
                claim_voucher_scalar(
                    &scope,
                    &name,
                    current.as_mut(),
                    entry.as_mut(),
                    allocation.as_mut(),
                )?;
                if let Some(tail) = scope.entry_tail() {
                    cost.start(tail, &name);
                    bank.start(tail, &name);
                    goods.start(tail, &name);
                    gst.start(tail, &name);
                }
                if let Some(tail) = scope.stock_line_tail().filter(|_| read_stock) {
                    stock_lines.start(tail, &name);
                }
                scope.start(name.clone());
                if scope.repeated_collection {
                    return Err("agent_read_protocol_invalid".into());
                }
                if let Some(tail) = scope.entry_tail() {
                    cost.end(tail);
                    bank.end(tail);
                    goods.end(tail);
                    gst.end(tail);
                }
                if let Some(tail) = scope.stock_line_tail().filter(|_| read_stock) {
                    stock_lines.end(tail);
                }
                if read_notes && scope.child("VOUCHER", DELIVERY_NOTE_LIST) {
                    notes.end_entry()?;
                }
                scope.end(&name)?;
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(_) => return Err("agent_read_protocol_invalid".to_string()),
            _ => {}
        }
    }
    scope.finish()?;
    Ok(rows)
}

pub(super) fn append_agent_text(row: &mut BTreeMap<String, String>, tag: &str, value: String) {
    row.entry(tag.to_string()).or_default().push_str(&value);
}

/// The most characters of an unrecognised credit-period text that are carried
/// out. The text comes from the book, so it is not passed on unbounded.
const UNRECOGNISED_CREDIT_PERIOD_TEXT_CHARS: usize = 40;

/// A bill allocation's credit period as typed JSON: `{"value": 30, "unit":
/// "days"}` (units days, weeks, months). The credit period is an annotation on
/// the allocation, so a text that is not `<n> Days|Weeks|Months` does not refuse
/// the window (that would lose every other voucher in it over a label, a read
/// that succeeded before this field was carried): it is carried as
/// `{"unit": "unrecognised", "text": ...}` and never read as a number of days.
/// A text over the cap is cut to it and marked `"truncated": true`.
fn credit_period_json(text: &str) -> Value {
    use bridge_tally_protocol::outstandings_shared::{parse_credit_period, CreditPeriod};
    match parse_credit_period(text) {
        Ok(CreditPeriod::Days(value)) => json!({"value": value, "unit": "days"}),
        Ok(CreditPeriod::Weeks(value)) => json!({"value": value, "unit": "weeks"}),
        Ok(CreditPeriod::Months(value)) => json!({"value": value, "unit": "months"}),
        Err(_) => {
            let carried = text
                .chars()
                .take(UNRECOGNISED_CREDIT_PERIOD_TEXT_CHARS)
                .collect::<String>();
            let mut period = json!({"unit": "unrecognised", "text": carried});
            if text.chars().count() > UNRECOGNISED_CREDIT_PERIOD_TEXT_CHARS {
                period["truncated"] = Value::Bool(true);
            }
            period
        }
    }
}

/// Starts a Delivery Note entry, or a field of one; an element inside a field is refused, as one
/// inside a voucher field is.
fn start_delivery_note_element(
    scope: &NativeCollectionScope,
    tag: &str,
    notes: &mut DeliveryNoteCollector,
) -> Result<(), String> {
    if scope.delivery_note_field() {
        Err("voucher_delivery_note_invalid".to_string())
    } else if scope.row("VOUCHER") && tag == DELIVERY_NOTE_LIST {
        notes.start_entry();
        Ok(())
    } else if scope.child("VOUCHER", DELIVERY_NOTE_LIST) {
        notes.start_field(tag)
    } else {
        Ok(())
    }
}

fn claim_voucher_scalar(
    scope: &NativeCollectionScope,
    field: &str,
    current: Option<&mut BTreeMap<String, String>>,
    entry: Option<&mut BTreeMap<String, String>>,
    allocation: Option<&mut BTreeMap<String, String>>,
) -> Result<(), String> {
    let row = if scope.row("VOUCHER")
        && (is_voucher_scalar(field) || is_voucher_type_class_scalar(field))
    {
        current
    } else if scope.child("VOUCHER", "ALLLEDGERENTRIES.LIST") && is_voucher_entry_scalar(field) {
        entry
    } else if scope.bill_allocation() && is_voucher_bill_allocation_scalar(field) {
        allocation
    } else {
        None
    };
    if let Some(row) = row {
        claim_agent_scalar(row, field)?;
    }
    Ok(())
}

/// The one rule every agent-facing parser applies to a Tally response before
/// reading it: forbidden numeric references are marked, then the XML reader
/// unescapes what is left (`docs/tally/TALLY_PROTOCOL_REFERENCE.md` §1.1(d)).
/// `&#4; Primary` therefore reads as `U+FFFD#4; Primary` here exactly as it
/// does in `bridge-tally-protocol`'s native parsers, never as a raw U+0004,
/// and [`decoded_agent_reference`] only sees references the rule leaves
/// alone. A literal U+FFFD followed by `#`, digits and `;` reads as
/// `U+FFFD#65533;` and the rest, which keeps the rewrite reversible.
pub(super) fn mark_agent_xml(xml: &str) -> std::borrow::Cow<'_, str> {
    bridge_tally_protocol::mark_forbidden_numeric_references(xml)
}

pub(super) fn decoded_agent_text(text: quick_xml::events::BytesText<'_>) -> Result<String, String> {
    quick_xml::escape::unescape(&text)
        .map(|value| value.into_owned())
        .map_err(|_| "agent_read_protocol_invalid".to_string())
}

pub(super) fn decoded_agent_reference(
    reference: quick_xml::events::BytesRef<'_>,
) -> Result<String, String> {
    quick_xml::escape::unescape(&format!("&{};", &*reference))
        .map(|value| value.into_owned())
        .map_err(|_| "agent_read_protocol_invalid".to_string())
}

pub(super) fn required_tally_bool(value: Option<&String>) -> Result<bool, String> {
    match value.map(String::as_str).map(str::trim) {
        Some("Yes") => Ok(true),
        Some("No") => Ok(false),
        _ => Err("voucher_accounting_state_not_observed".to_string()),
    }
}

#[cfg(test)]
pub(super) const FOREX_72_COMPANY_GUID: &str = "1bb48be5-baba-45b2-90d8-7593244915a9";

#[cfg(test)]
pub(super) fn decoded_utf16le(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// The 7.2 lab book's answer of one day: eleven vouchers, eight with composites
/// (fixtures/agent/vouchers-forex-unspaced-20250401.PROVENANCE.md).
#[cfg(test)]
pub(super) fn captured_72_vouchers() -> String {
    decoded_utf16le(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-forex-unspaced-20250401.utf16le.xml"
    ))
}

/// The registers' class-entry window of the same day (`forex-register`, #683), each voucher as
/// the window parse leaves it before the book's base is read, in the shape the date, ledger and
/// class checks read: every voucher with a composite is withheld.
#[cfg(test)]
pub(super) fn captured_72_register_window() -> Vec<Value> {
    let shape = super::voucher_window::VoucherReadShape::ClassEntryWildcard;
    parse_agent_rows_withholding_reading_gst(
        &decoded_utf16le(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/forex-register/forex-register-window.utf16le.xml"
        )),
        FOREX_72_COMPANY_GUID,
        GstDetailsShown::Withheld,
        shape.named_fields(),
        shape.stock_lines(),
    )
    .expect("the captured register window parses")
    .into_iter()
    .map(VoucherRow::into_filter_row)
    .collect()
}

/// That book's base and masters, from its own captured currency reads.
#[cfg(test)]
pub(super) fn captured_72_book() -> IdentifiedBaseCurrency {
    use bridge_tally_protocol::native_outstandings::{
        parse_company_currency_name, parse_currency_master_list,
    };
    let masters = parse_currency_master_list(&decoded_utf16le(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/native/currency_masters_originalname_forex_72.utf16le.xml"
    )))
    .unwrap();
    let company = parse_company_currency_name(
        &decoded_utf16le(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/native/company_base_currency_forex_72.utf16le.xml"
        )),
        FOREX_72_COMPANY_GUID,
    )
    .unwrap();
    masters.identify_base(Some(&company)).unwrap()
}

/// Synthetic mutation for tests (#674): the captured three-voucher window
/// (`native-three-vouchers`) with the first `count` vouchers' amounts replaced
/// by the composites a live read of the several-currency book captured
/// (`vouchers-forex-composite-20260915`): each negative amount by the captured
/// negative composite, each positive one by the positive composite.
#[cfg(test)]
pub(super) fn window_with_composite_vouchers(count: usize) -> String {
    let decode = |bytes: &[u8]| {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    };
    let forex = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-forex-composite-20260915.utf16le.xml"
    ));
    let amounts = |xml: &str| -> Vec<String> {
        xml.split("<AMOUNT")
            .skip(1)
            .filter_map(|tail| Some(tail[tail.find('>')? + 1..tail.find("</AMOUNT>")?].to_string()))
            .collect()
    };
    let composites = amounts(&forex);
    let negative = composites
        .iter()
        .find(|value| value.starts_with('-'))
        .unwrap()
        .clone();
    let positive = composites
        .iter()
        .find(|value| !value.starts_with('-'))
        .unwrap()
        .clone();
    let mut window = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    ));
    // The first `</VOUCHER>` closes CMPINFO's counter, not a voucher.
    let mut from = 0;
    for _ in 0..count {
        let start = from + window[from..].find("<VOUCHER ").unwrap();
        let end = start + window[start..].find("</VOUCHER>").unwrap();
        let mut voucher = window[start..end].to_string();
        for plain in amounts(&voucher) {
            let at = voucher.find(&format!(">{plain}</AMOUNT>")).unwrap();
            let composite = if plain.starts_with('-') {
                &negative
            } else {
                &positive
            };
            voucher.replace_range(at + 1..at + 1 + plain.len(), composite);
        }
        window.replace_range(start..end, &voucher);
        from = start + voucher.len();
    }
    window
}

#[cfg(test)]
#[path = "agent_voucher_parse_tests.rs"]
mod boundary_tests;

#[cfg(test)]
#[path = "agent_voucher_bank_allocations_tests.rs"]
mod bank_allocation_tests;
#[cfg(test)]
#[path = "agent_voucher_cost_centres_tests.rs"]
mod cost_centre_tests;
#[cfg(test)]
#[path = "agent_voucher_entry_gst_tests.rs"]
mod entry_gst_tests;
#[cfg(test)]
#[path = "agent_voucher_goods_lines_tests.rs"]
mod goods_line_tests;
