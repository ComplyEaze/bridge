//! Purchase and sales registers for the local MCP adapter (#969).
//!
//! Lists the vouchers that touch a ledger under Duties & Taxes and says, per entry, what the
//! books record: tax only on a ledger whose master carries a recognised GST duty head, never
//! from a name and never from arithmetic. What the books do not say is listed as such. The two
//! registers read the same way and differ only in `RegisterKind`: which voucher classes are
//! the register, and which predefined group holds the taxable ledgers.
//!
//! This file holds the classifier, which has no I/O: it takes the ledger masters as read and
//! the voucher rows as parsed, and returns rows. The reads and their snapshot binding are in
//! `Server::register`.
use bridge_tally_protocol::group_ancestry::{AncestryChain, AncestryGap, GroupIndex};
use bridge_tally_protocol::{GstDutyHeadObservation, PartyLedgerMasterRecord};
use std::collections::{BTreeMap, BTreeSet};

use super::*;
use bridge_tally_core::book_presence::WindowRead;

const DUTIES_AND_TAXES: &str = "Duties & Taxes";
const PURCHASE_ACCOUNTS: &str = "Purchase Accounts";
const SALES_ACCOUNTS: &str = "Sales Accounts";

/// Which register a call builds. A purchase register lists Purchase and Debit Note vouchers
/// and takes the ledgers under Purchase Accounts as the taxable ones; a sales register lists
/// Sales and Credit Note vouchers and takes those under Sales Accounts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RegisterKind {
    Purchase,
    Sales,
}

impl RegisterKind {
    pub(super) fn of_tool(name: &str) -> Option<Self> {
        match name {
            "purchase_register" => Some(Self::Purchase),
            "sales_register" => Some(Self::Sales),
            _ => None,
        }
    }

    fn classes(self) -> [&'static str; 2] {
        match self {
            Self::Purchase => ["Purchase", "Debit Note"],
            Self::Sales => ["Sales", "Credit Note"],
        }
    }

    fn taxable_group(self) -> LedgerGroup {
        match self {
            Self::Purchase => LedgerGroup::PurchaseAccounts,
            Self::Sales => LedgerGroup::SalesAccounts,
        }
    }

    fn profile(self) -> &'static str {
        match self {
            Self::Purchase => "agent_purchase_register_v1",
            Self::Sales => "agent_sales_register_v1",
        }
    }

    fn without_entry_key(self) -> &'static str {
        match self {
            Self::Purchase => "purchase_vouchers_without_duties_taxes_entry",
            Self::Sales => "sales_vouchers_without_duties_taxes_entry",
        }
    }

    /// What the register's `coverage` says about a cancelled voucher in its without-entry list.
    /// The purchase sentence rests on cancelled vouchers read back with no ledger entries
    /// (protocol reference §11c.5 and §9.14); no cancelled sale has been read, so the sales one
    /// says only what the classification does.
    fn cancelled_note(self) -> &'static str {
        match self {
            Self::Purchase => "; a cancelled voucher is listed there too, with cancelled true, because the cancelled vouchers measured came back from Tally with no ledger entries, so being listed there does not mean a cancelled purchase was untaxed",
            Self::Sales => "; a cancelled sale that Tally returns with no ledger entries is listed there too, with cancelled true, so being listed there does not mean it was untaxed; no cancelled sale has been read",
        }
    }

    /// What the register's `coverage` says is not measured. The purchase register states it in
    /// its description only; the sales register has fewer captures behind it, so it says it again
    /// in each response.
    fn not_measured_note(self) -> &'static str {
        match self {
            Self::Purchase => "",
            Self::Sales => "; measured for sales so far: live runs on two synthetic companies for one taxed Sales item invoice (a row), one untaxed one (counted apart; read once by an earlier build: its voucher window is committed, its masters and the tool's answer are not) and one Credit Note in voucher view booked on account (a row, with its signs reversed as Tally sends them: nothing is netted or flipped, so sum signed amounts), and one Sales accounting voucher classified against its book's ledger masters; the state-side head is state_tax on one measured book and sgst_utgst on another, both recognised for the same side, so look for neither alone; not shown by any run: an invoice-view Credit Note, an inter-state (IGST) line, a cancelled or optional sales voucher, an unrecognised or missing duty head on a sale, more than one voucher in a window, paging, a company with a registration, a tax Tally computes itself, a sale typed on Tally's screen, accounting-invoice mode, a post-dated sale, a REFERENCE or a populated PARTYGSTIN on a sale, REFERENCEDATE (not returned) and a ledger or voucher kept in a currency other than the book's base; a row carries `not_measured_live` naming the kind only where the row itself shows it, and kinds a row cannot show are not marked and not vouched for (a sale typed on Tally's screen in voucher view, a tax Tally computed itself, a duty head no sales capture has such as cess, an invoice of another shape than the one run, a ledger or voucher kept in a currency other than the book's base), so an unmarked row is not a measured one in those respects; a row is marked inter_state_line only when a tax entry's ledger master carries a recognised IGST head, and an IGST ledger with no head or an unrecognised head is listed under the without-head or unrecognised list with a status other than complete; the response `state` follows the rule `vouchers` uses: `complete` only when the window's rows were admitted against a separate voucher count (or the window was empty and its corroboration read confirmed it), otherwise `partial` with `nonempty_window_unqualified` and the rows still returned; a row's `status` of complete means every entry it touches classified",
        }
    }

    /// The tool's description, without the receipt sentence the catalogue appends to every read.
    pub(super) fn description(self) -> String {
        match self {
            Self::Purchase => PURCHASE_REGISTER_DESCRIPTION.to_string(),
            Self::Sales => SALES_REGISTER_DESCRIPTION.to_string(),
        }
    }
}

/// The purchase register's description. The sales register's is the text below it; a test
/// derives the sales text from this one and fails when the two stop agreeing about the shared
/// read, so an edit to one cannot silently leave the other behind.
const PURCHASE_REGISTER_DESCRIPTION: &str = "Read-only: a register of what the books record, not a GST return. It does not decide input tax credit eligibility or blocked credit, matches nothing against GSTR-2B or any portal, checks no GSTIN (`party_gstin` is returned only when the voucher carries one), does not return REFERENCEDATE yet (`reference` is returned only when the voucher carries one), does not classify an item invoice's purchase as taxable (`has_taxable_entry` is false when no entry sits on a Purchase Accounts ledger), and never sums tax across heads or vouchers. It does not treat reverse-charge journals, imports (IGST paid at customs) or input service distribution specially: a voucher that touches a Duties & Taxes ledger is listed by the rule below and nothing more. A GST duty head does not say whether a ledger is input or output, and a Debit Note can be a purchase return or a debit note issued to a customer: each row carries `party_group` (the voucher party's predefined group, for example Sundry Creditors or Sundry Debtors, when it resolves) and the tool does not guess which it is. It inherits the compliance read's refusals (an INR base currency is required; a book too large to list is refused; see `ledger_masters`) and refuses with `register_master_mark_unavailable` when Tally does not report the master-alteration mark. Each page re-reads the masters and the window, so rows can shift between pages. Return the Purchase and Debit Note vouchers of a date window that touch a ledger under Duties & Taxes, with the tax each entry carries taken only from the GST duty head recorded on that ledger's master -- never from a ledger name and never from an amount. Reads the full voucher window before pagination (use narrow dates) and the ledger masters twice, before and after it. Per row: `tax_in_books` lists each entry on a ledger whose head ComplyEaze Bridge recognises as {ledger, head, raw_head, amount}; `duties_taxes_entries_without_gst_head` lists entries on Duties & Taxes ledgers that carry no GST head and never assigns them one: `observation` `not_tax_ledger` is a ledger whose own tax type is not GST (usually TDS or another payable), `absent` is a ledger with no head whose tax type is GST or was not reported, which may be a GST ledger whose head is missing (`tax_type` says which); `duties_taxes_entries_with_unrecognised_head` lists entries whose head is not in the recognised vocabulary or contradicts the ledger's tax type, with the raw spelling and its observation; `entries_on_ledgers_with_unresolved_group` lists entries on ledgers whose group chain could not be resolved; `taxable_entries` are entries on Purchase Accounts ledgers only (a GST purchase booked to a fixed-asset or expense ledger has `has_taxable_entry` false); `party_entries` are the voucher party's own; `other_entries` is everything else (round-off included) with no role inferred. `status` is the first that applies of head_conflict, has_unrecognised_head, has_unresolved_group, has_entries_without_gst_head, has_other_entries, complete. The response `state` follows the rule `vouchers` uses: `complete` only when every voucher read was checked against a separate count of the window (a census, which ComplyEaze Bridge sends unless the book's voucher high-water mark alone proves it small, a few dozen vouchers), otherwise `partial` with `reason` `nonempty_window_unqualified` and the rows still returned; an empty window is `complete` when its corroboration read confirms it. A row's `status` is separate: it says whether every entry the voucher touches classified, and the `state` does not change it. Amounts are as the books state them (negative is a debit), never re-signed and never summed across heads; there is no input-credit or direction field. `reference`, `party_gstin`, `is_invoice`, `post_dated` follow `vouchers`: absent means not observed, and `cancelled`, `optional` and `post_dated` vouchers are returned flagged, not excluded. Every other voucher type that touches Duties & Taxes (Sales, Journal, Payment and so on) is listed apart in `other_voucher_types_touching_duties_taxes`, not in `items`: whether it belongs in a return is the CA's call. A voucher with no resolved class is listed under `unclassified_voucher_type`; a voucher that touches only unplaceable ledgers under `vouchers_with_unplaced_ledgers`; a Purchase or Debit Note voucher with no entry on a Duties & Taxes ledger under `purchase_vouchers_without_duties_taxes_entry` (exempt or unregistered purchases, tax booked to a ledger filed elsewhere, or a cancelled voucher). A cancelled voucher is listed there with `cancelled` true whether or not it was taxed, because the cancelled vouchers measured came back from Tally with no ledger entries; a cancelled voucher that keeps its entries is not measured. Rows are in `items` (paged by offset and limit like `vouchers`); each has `has_taxable_entry`, false when no entry sits on a Purchase Accounts ledger (an item invoice may hold it in an inventory allocation). The side lists carry exact counts (`total`) and at most 100 items (`listed`); every ledger name in the response is masked like `vouchers` masks it. A voucher that names a ledger the masters do not list, a master or voucher that changed while the window was read, or a ledger set aside for its currency, refuses (`ledger_snapshot_drifted`, `voucher_window_changed_during_read`, `register_ledger_currency_excluded`) and releases no rows; a row dated outside the window refuses as `window_not_honoured`. A `sgst_utgst` head is a state-side head that a consumer summing state tax must include alongside `state_tax`. Not measured: REFERENCEDATE (not returned), item invoices whose purchase ledger sits in an inventory allocation, and books with several currencies.";

/// The sales register's description: its own opening, then the purchase text's shared read with
/// the sales classes, group, list name and measurements swapped in. A test derives everything
/// after the opening from `PURCHASE_REGISTER_DESCRIPTION` and compares it with this text.
const SALES_REGISTER_DESCRIPTION: &str = "Read-only: a register of what the books record, not a GST return. It does not decide the place of supply, the tax rate, whether tax is payable or which part of a return a sale belongs in, matches nothing against any portal, checks no GSTIN (`party_gstin` is returned only when the voucher carries one), does not return REFERENCEDATE yet (`reference` is returned only when the voucher carries one), and never sums tax across heads or vouchers. It does not treat exports, sales under reverse charge or advances specially: a voucher that touches a Duties & Taxes ledger is listed by the rule below and nothing more. A GST duty head does not say whether a ledger is input or output, and a Credit Note can be a sales return or a credit note issued to a supplier: each row carries `party_group` (the voucher party's predefined group, for example Sundry Debtors or Sundry Creditors, when it resolves) and the tool does not guess which it is. A Debit Note, including one issued to a customer, is not a sales row: it is listed apart by identity and ledger names, with no amount. It inherits the compliance read's refusals (an INR base currency is required; a book too large to list is refused; see `ledger_masters`) and refuses with `register_master_mark_unavailable` when Tally does not report the master-alteration mark. Each page re-reads the masters and the window, so rows can shift between pages. Return the Sales and Credit Note vouchers of a date window that touch a ledger under Duties & Taxes, with the tax each entry carries taken only from the GST duty head recorded on that ledger's master -- never from a ledger name and never from an amount. Reads the full voucher window before pagination (use narrow dates) and the ledger masters twice, before and after it. Per row: `tax_in_books` lists each entry on a ledger whose head ComplyEaze Bridge recognises as {ledger, head, raw_head, amount}; `duties_taxes_entries_without_gst_head` lists entries on Duties & Taxes ledgers that carry no GST head and never assigns them one: `observation` `not_tax_ledger` is a ledger whose own tax type is not GST (usually TDS or another payable), `absent` is a ledger with no head whose tax type is GST or was not reported, which may be a GST ledger whose head is missing (`tax_type` says which); `duties_taxes_entries_with_unrecognised_head` lists entries whose head is not in the recognised vocabulary or contradicts the ledger's tax type, with the raw spelling and its observation; `entries_on_ledgers_with_unresolved_group` lists entries on ledgers whose group chain could not be resolved; `taxable_entries` are entries on Sales Accounts ledgers only (a sale booked to another ledger, or whose sales ledger sits in an inventory allocation, has `has_taxable_entry` false); `party_entries` are the voucher party's own; `other_entries` is everything else (round-off included) with no role inferred. `status` is the first that applies of head_conflict, has_unrecognised_head, has_unresolved_group, has_entries_without_gst_head, has_other_entries, complete. The response `state` follows the rule `vouchers` uses: `complete` only when every voucher read was checked against a separate count of the window (a census, which ComplyEaze Bridge sends unless the book's voucher high-water mark alone proves it small, a few dozen vouchers), otherwise `partial` with `reason` `nonempty_window_unqualified` and the rows still returned; an empty window is `complete` when its corroboration read confirms it. A row's `status` is separate: it says whether every entry the voucher touches classified, and the `state` does not change it. Amounts are as the books state them (negative is a debit), never re-signed and never summed across heads; there is no input-credit or direction field. `reference`, `party_gstin`, `is_invoice`, `post_dated` follow `vouchers`: absent means not observed, and `cancelled`, `optional` and `post_dated` vouchers are returned flagged, not excluded. Every other voucher type that touches Duties & Taxes (Purchase, Journal, Payment and so on) is listed apart in `other_voucher_types_touching_duties_taxes`, not in `items`: whether it belongs in a return is the CA's call. A voucher with no resolved class is listed under `unclassified_voucher_type`; a voucher that touches only unplaceable ledgers under `vouchers_with_unplaced_ledgers`; a Sales or Credit Note voucher with no entry on a Duties & Taxes ledger under `sales_vouchers_without_duties_taxes_entry` (listed by identity only; the tool does not say why such a voucher carries no tax entry). A cancelled sale that Tally returns with no ledger entries is listed there too, with `cancelled` true; no cancelled sale has been read, so whether one keeps its entries is not measured. Rows are in `items` (paged by offset and limit like `vouchers`); each has `has_taxable_entry`, false when no entry sits on a Sales Accounts ledger (a sale typed on Tally's screen, or an item invoice of another shape than the imported one that was measured, may hold the sales ledger in an inventory allocation instead; not measured). The side lists carry exact counts (`total`) and at most 100 items (`listed`); every ledger name in the response is masked like `vouchers` masks it. A voucher that names a ledger the masters do not list, a master or voucher that changed while the window was read, or a ledger set aside for its currency, refuses (`ledger_snapshot_drifted`, `voucher_window_changed_during_read`, `register_ledger_currency_excluded`) and releases no rows; a row dated outside the window refuses as `window_not_honoured`. A `sgst_utgst` head is a state-side head that a consumer summing state tax must include alongside `state_tax`. Measured so far: `sales_register` was run against a live Tally on two synthetic companies. On the first, once per day, for one taxed Sales item invoice and one untaxed one: the taxed sale came back as one row with its CGST and SGST/UTGST heads taken from the ledger masters and its sales ledger as the taxable entry, and the untaxed one (read once by an earlier build; its voucher window is committed, its masters and the tool's answer are not) was counted under `sales_vouchers_without_duties_taxes_entry`. On the second, which has 44 ledgers, for one Credit Note in voucher view booked on account: one row, with its CGST and state-tax heads and its sales ledger as the taxable entry. One Sales accounting voucher (not an invoice) was also classified, in tests, against the ledger masters of the purchase register's lab book. A Credit Note is returned as a row with its signs reversed as Tally sends them: the tool neither nets nor flips, so a caller that sums tax over a window must add signed amounts. The measured Credit Note of 1,000.00 with 90.00 CGST and 90.00 State Tax came back with the sales entry -1000.00, each tax entry -90.00 and the party entry 1180.00, where a Sales row has the sales and tax entries positive and the party entry negative. The state-side tax head is `state_tax` (raw State Tax) on one measured book and `sgst_utgst` (raw SGST/UTGST) on another; both are recognised heads for the same side of the tax, so a caller must not look for one of them only. The cost of a call varies by book: 96 requests on a book with 8 ledgers and one currency, 118 on one with 44 ledgers and two currencies, which adds a voucher census and base-currency reads; the result does not report the cost. Not shown by any run: an invoice-view Credit Note; an inter-state (IGST) line; a cancelled or optional sales voucher; an unrecognised or missing duty head on a sale; more than one voucher in a window; paging; a company with a registration; a tax that Tally computes itself; a sale typed on Tally's screen; accounting-invoice mode; a post-dated sale; a REFERENCE or a populated PARTYGSTIN on a sale; REFERENCEDATE (not returned); and a ledger or voucher kept in a currency other than the book's base. A row of such a kind is returned, not withheld, and carries `not_measured_live` naming why (invoice_view_credit_note, inter_state_line, sales_ledger_not_an_entry, cancelled, optional, post_dated, party_gstin_present, reference_present) only where the row itself shows the kind. Kinds a row cannot show are never marked and are not vouched for: a sale typed on Tally's screen in voucher view, a tax Tally computed itself, a duty head no sales capture has (such as cess), an invoice of another shape than the one run (for example several goods lines), and a ledger or voucher kept in a currency other than the book's base; an unmarked row is not a measured one in those respects. A row is marked `inter_state_line` only when a tax entry's ledger master carries a recognised IGST head; an IGST ledger with no head, or an unrecognised head, is listed under the without-head or unrecognised list and the status is not complete.";

/// Which predefined group a ledger sits under, from its group chain and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LedgerGroup {
    DutiesAndTaxes,
    PurchaseAccounts,
    SalesAccounts,
    Other,
    /// The chain stopped before any predefined group: the ledger cannot be placed.
    Unresolved(AncestryGap),
}

/// What the classifier keeps of one ledger master. Everything here is an input to a row's
/// classification, so two listings that differ in any of it classify differently.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RegisterLedger {
    group: LedgerGroup,
    /// The nearest predefined group's `RESERVEDNAME` (for example Sundry Debtors), when the
    /// chain resolves to one. Reported for a voucher's party so a Debit Note to a customer can
    /// be told from a purchase return; it says nothing about which tax ledger is input or output.
    reserved: Option<String>,
    tax_type: Option<String>,
    head: GstDutyHeadObservation,
}

/// The ledger masters of one company as the register reads them. Equality is the drift
/// check: a listing read after the vouchers must equal the one read before.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MasterIndex {
    by_name: BTreeMap<String, RegisterLedger>,
    /// Ledgers the compliance read set aside by currency (foreign, or a base-currency
    /// ledger whose balance is a composite). A voucher that touches one cannot be classified.
    excluded: BTreeSet<String>,
}

impl MasterIndex {
    /// A name that repeats cannot be classified (Tally keeps ledger names unique per
    /// company), so it refuses as drift rather than picking one.
    pub(super) fn build<'a>(
        records: impl IntoIterator<Item = &'a PartyLedgerMasterRecord>,
        groups: &GroupIndex,
        excluded: impl IntoIterator<Item = String>,
    ) -> Result<Self, String> {
        let mut by_name = BTreeMap::new();
        for record in records {
            let parent = record.ledger.parent.returned_text().map(str::to_string);
            let chain = groups.ancestry_chain(parent.as_deref());
            let ledger = RegisterLedger {
                group: ledger_group(&chain),
                reserved: nearest_reserved(&chain),
                tax_type: record.fields.tax_type.returned_text().map(str::to_string),
                head: record.fields.gst_duty_head.clone(),
            };
            if by_name.insert(record.ledger.name.clone(), ledger).is_some() {
                return Err("ledger_snapshot_drifted".to_string());
            }
        }
        Ok(Self {
            by_name,
            excluded: excluded.into_iter().collect(),
        })
    }

    pub(super) fn len(&self) -> usize {
        self.by_name.len()
    }
}

/// The nearest predefined group in the chain, by its `RESERVEDNAME`, which a rename of the
/// book's own group does not change and which the walk reaches through user sub-groups.
fn ledger_group(chain: &AncestryChain) -> LedgerGroup {
    match chain
        .hops
        .iter()
        .map(|hop| hop.reserved_name.trim())
        .find(|reserved| !reserved.is_empty())
    {
        Some(DUTIES_AND_TAXES) => LedgerGroup::DutiesAndTaxes,
        Some(PURCHASE_ACCOUNTS) => LedgerGroup::PurchaseAccounts,
        Some(SALES_ACCOUNTS) => LedgerGroup::SalesAccounts,
        Some(_) => LedgerGroup::Other,
        None => match chain.gap {
            Some(gap) => LedgerGroup::Unresolved(gap),
            None => LedgerGroup::Other,
        },
    }
}

fn nearest_reserved(chain: &AncestryChain) -> Option<String> {
    chain
        .hops
        .iter()
        .map(|hop| hop.reserved_name.trim())
        .find(|reserved| !reserved.is_empty())
        .map(str::to_string)
}

fn gap_code(gap: AncestryGap) -> &'static str {
    match gap {
        AncestryGap::NoParent => "no_parent",
        AncestryGap::ReachedRoot => "reached_root",
        AncestryGap::GroupAbsent => "group_absent",
        AncestryGap::GroupNameRepeated => "group_name_repeated",
        AncestryGap::ReservedNameMissing => "reserved_name_missing",
        AncestryGap::Cycle => "cycle",
        AncestryGap::Exhausted => "exhausted",
    }
}

/// Why a sales row is of a kind no capture has covered, as codes a caller can read. A row the
/// captures cover has none and carries no field. The purchase register has no such field: its
/// kinds were measured, and the tool text says what was not.
fn sales_not_measured(
    row: &Value,
    class: Option<&str>,
    tax: &[Value],
    has_taxable_entry: bool,
) -> Vec<&'static str> {
    let flag = |key: &str| row.get(key).and_then(Value::as_bool) == Some(true);
    let mut marks = Vec::new();
    // A Credit Note in voucher view was measured live once; one in the invoice view was not.
    if class == Some("Credit Note") && flag("is_invoice") {
        marks.push("invoice_view_credit_note");
    }
    if tax.iter().any(|entry| entry["head"] == "igst") {
        marks.push("inter_state_line");
    }
    if !has_taxable_entry && !tax.is_empty() {
        marks.push("sales_ledger_not_an_entry");
    }
    for (key, code) in [
        ("cancelled", "cancelled"),
        ("optional", "optional"),
        ("post_dated", "post_dated"),
    ] {
        if flag(key) {
            marks.push(code);
        }
    }
    for (key, code) in [
        ("party_gstin", "party_gstin_present"),
        ("reference", "reference_present"),
    ] {
        if row.get(key).is_some_and(|value| !value.is_null()) {
            marks.push(code);
        }
    }
    marks
}

/// The classifier's output for one window: the register, the other voucher types that touch
/// Duties & Taxes (counted and listed, never dropped), and what has no class.
#[derive(Debug, PartialEq)]
pub(super) struct RegisterPage {
    pub(super) rows: Vec<Value>,
    pub(super) other_voucher_types: Vec<Value>,
    pub(super) unclassified_voucher_type: Vec<Value>,
    /// Vouchers that touch a ledger whose group cannot be resolved and no Duties & Taxes
    /// ledger: they may have nothing to do with tax, so they are named, not classified.
    pub(super) vouchers_with_unplaced_ledgers: Vec<Value>,
    /// Register-class vouchers with no entry on a Duties & Taxes ledger (exempt or unregistered
    /// ones, or tax booked to a ledger filed elsewhere): counted, not dropped.
    pub(super) register_class_without_duties_taxes_entry: Vec<Value>,
    pub(super) vouchers_observed: usize,
}

/// The register's classes are its rows. Any other class that touches Duties & Taxes is listed
/// apart; whether it belongs in a return is the CA's call, not the tool's.
fn is_register_class(kind: RegisterKind, class: &str) -> bool {
    kind.classes().contains(&class)
}

/// Classify the vouchers of one window against the ledger masters.
///
/// A voucher entry that names a ledger the masters do not list refuses (the masters moved
/// between the two reads, or the window belongs to another book); it is never put in a
/// catch-all. `rows` are the parsed rows of the class-entry voucher shape, in window order.
pub(super) fn classify_register(
    kind: RegisterKind,
    index: &MasterIndex,
    rows: &[Value],
) -> Result<RegisterPage, String> {
    let mut page = RegisterPage {
        rows: Vec::new(),
        other_voucher_types: Vec::new(),
        unclassified_voucher_type: Vec::new(),
        vouchers_with_unplaced_ledgers: Vec::new(),
        register_class_without_duties_taxes_entry: Vec::new(),
        vouchers_observed: rows.len(),
    };
    for row in rows {
        let entries = row
            .get("amounts")
            .and_then(Value::as_array)
            .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
        let party = row.get("party").and_then(Value::as_str);
        let mut tax = Vec::new();
        let mut without_head = Vec::new();
        let mut unrecognised = Vec::new();
        let mut unresolved = Vec::new();
        let mut taxable = Vec::new();
        let mut party_entries = Vec::new();
        let mut other = Vec::new();
        let mut touched = Vec::new();
        let mut conflict = false;
        for entry in entries {
            let name = entry
                .get("ledger")
                .and_then(Value::as_str)
                .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
            let ledger = index.by_name.get(name).ok_or_else(|| {
                let code = if index.excluded.contains(name) {
                    "register_ledger_currency_excluded"
                } else {
                    "ledger_snapshot_drifted"
                };
                code.to_string()
            })?;
            match ledger.group {
                LedgerGroup::DutiesAndTaxes => {
                    touched.push(json!(name));
                    match &ledger.head {
                        GstDutyHeadObservation::Recognized { raw, head } => {
                            let mut item = entry.clone();
                            item["head"] = serde_json::to_value(head).unwrap_or(Value::Null);
                            item["raw_head"] = json!(raw);
                            tax.push(item);
                        }
                        GstDutyHeadObservation::NotTaxLedger { tax_type } => {
                            let mut item = entry.clone();
                            item["observation"] = json!("not_tax_ledger");
                            item["tax_type"] = json!(tax_type);
                            without_head.push(item);
                        }
                        GstDutyHeadObservation::Absent => {
                            // No head, and the ledger's own TAXTYPE is GST or was not
                            // reported: unlike not_tax_ledger, this may be a GST ledger
                            // whose head is missing.
                            let mut item = entry.clone();
                            item["observation"] = json!("absent");
                            item["tax_type"] = json!(ledger.tax_type);
                            without_head.push(item);
                        }
                        GstDutyHeadObservation::Unrecognized { raw } => {
                            let mut item = entry.clone();
                            item["observation"] = json!("unrecognized");
                            item["raw_head"] = json!(raw);
                            unrecognised.push(item);
                        }
                        GstDutyHeadObservation::Contradictory { tax_type, raw } => {
                            conflict = true;
                            let mut item = entry.clone();
                            item["observation"] = json!("contradictory");
                            item["tax_type"] = json!(tax_type);
                            item["raw_head"] = json!(raw);
                            unrecognised.push(item);
                        }
                    }
                }
                group if group == kind.taxable_group() => taxable.push(entry.clone()),
                LedgerGroup::Unresolved(gap) => {
                    let mut item = entry.clone();
                    item["ancestry_gap"] = json!(gap_code(gap));
                    unresolved.push(item);
                }
                LedgerGroup::PurchaseAccounts | LedgerGroup::SalesAccounts | LedgerGroup::Other => {
                    if party == Some(name) {
                        party_entries.push(entry.clone());
                    } else {
                        other.push(entry.clone());
                    }
                }
            }
        }
        let class = row.get("voucher_class").and_then(Value::as_str);
        let identity = json!({
            "date": row.get("date"),
            "voucher_number": row.get("voucher_number"),
            "voucher_type": row.get("voucher_type"),
            "voucher_class": class,
            "guid": row.get("guid"),
        });
        // Present only when Tally reported them, like `vouchers`: absent is not false.
        let mut identity = identity;
        for key in ["cancelled", "optional", "post_dated"] {
            if let Some(value) = row.get(key) {
                identity[key] = value.clone();
            }
        }
        if touched.is_empty() && unresolved.is_empty() {
            if class.is_some_and(|class| is_register_class(kind, class)) {
                page.register_class_without_duties_taxes_entry
                    .push(identity);
            }
            continue;
        }
        if !touched.is_empty() {
            match class {
                Some(class) if is_register_class(kind, class) => {}
                Some(_) => {
                    let mut item = identity.clone();
                    item["duties_taxes_ledgers"] = Value::Array(touched);
                    page.other_voucher_types.push(item);
                    continue;
                }
                None => {
                    let mut item = identity.clone();
                    item["duties_taxes_ledgers"] = Value::Array(touched);
                    page.unclassified_voucher_type.push(item);
                    continue;
                }
            }
        }
        // A voucher that only touches a ledger that cannot be placed is not in the register
        // (it may be nothing to do with tax), but it is not dropped either: it is named.
        if touched.is_empty() {
            let mut item = identity.clone();
            item["unplaced_ledgers"] = Value::Array(unresolved);
            page.vouchers_with_unplaced_ledgers.push(item);
            continue;
        }
        let status = if conflict {
            "head_conflict"
        } else if !unrecognised.is_empty() {
            "has_unrecognised_head"
        } else if !unresolved.is_empty() {
            "has_unresolved_group"
        } else if !without_head.is_empty() {
            "has_entries_without_gst_head"
        } else if !other.is_empty() {
            "has_other_entries"
        } else {
            "complete"
        };
        let mut out = identity;
        for key in [
            "alter_id",
            "party",
            "party_gstin",
            "reference",
            "is_invoice",
            "cancelled",
            "optional",
            "post_dated",
        ] {
            if let Some(value) = row.get(key) {
                out[key] = value.clone();
            }
        }
        // The party ledger's predefined group, from the same masters read: a Debit Note to a
        // Sundry Debtors party is a customer's debit note, not a purchase return. Absent when
        // the party is unknown or its group did not resolve; the tool never guesses the intent.
        if let Some(reserved) = party
            .and_then(|name| index.by_name.get(name))
            .and_then(|ledger| ledger.reserved.as_ref())
        {
            out["party_group"] = json!(reserved);
        }
        out["status"] = json!(status);
        let not_measured = (kind == RegisterKind::Sales)
            .then(|| sales_not_measured(row, class, &tax, !taxable.is_empty()))
            .filter(|marks| !marks.is_empty());
        out["tax_in_books"] = Value::Array(tax);
        out["duties_taxes_entries_without_gst_head"] = Value::Array(without_head);
        out["duties_taxes_entries_with_unrecognised_head"] = Value::Array(unrecognised);
        out["entries_on_ledgers_with_unresolved_group"] = Value::Array(unresolved);
        if let Some(marks) = not_measured {
            out["not_measured_live"] = json!(marks);
        }
        out["has_taxable_entry"] = json!(!taxable.is_empty());
        out["taxable_entries"] = Value::Array(taxable);
        out["party_entries"] = Value::Array(party_entries);
        out["other_entries"] = Value::Array(other);
        page.rows.push(out);
    }
    Ok(page)
}

/// How many items of the side lists (other voucher types, unclassified, unplaced) one
/// response carries; the totals are exact whatever the cap.
const MAX_LISTED_SIDE_ITEMS: usize = 100;

fn bounded_list(items: &[Value]) -> Value {
    json!({
        "total": items.len(),
        "listed": items.iter().take(MAX_LISTED_SIDE_ITEMS).collect::<Vec<_>>(),
        "listed_truncated": items.len() > MAX_LISTED_SIDE_ITEMS,
    })
}

/// The names a response may need to mask: the voucher's party and every entry's ledger.
pub(super) fn mark_register_row(mut row: Value) -> Value {
    mark_party_field(&mut row, "party");
    // Every entry's ledger, as `vouchers` marks them: a ledger can be named for a party (a
    // per-deductee TDS ledger, a purchase ledger per supplier), and the same ledger must be
    // masked the same way by every tool.
    for list in [
        "tax_in_books",
        "duties_taxes_entries_without_gst_head",
        "duties_taxes_entries_with_unrecognised_head",
        "entries_on_ledgers_with_unresolved_group",
        "taxable_entries",
        "party_entries",
        "other_entries",
    ] {
        if let Some(entries) = row.get_mut(list).and_then(Value::as_array_mut) {
            for entry in entries {
                mark_party_field(entry, "ledger");
            }
        }
    }
    row
}

/// A side-list item names ledgers: the ones it could not place, and the Duties & Taxes ledgers
/// the voucher touches; any of them can be named for a party.
fn mark_register_side_item(mut item: Value) -> Value {
    if let Some(entries) = item
        .get_mut("unplaced_ledgers")
        .and_then(Value::as_array_mut)
    {
        for entry in entries {
            mark_party_field(entry, "ledger");
        }
    }
    if let Some(names) = item
        .get_mut("duties_taxes_ledgers")
        .and_then(Value::as_array_mut)
    {
        for name in names {
            if let Value::String(text) = name.take() {
                *name = party_name_value(text);
            }
        }
    }
    item
}

/// The window was planned against the marks the masters were read under; the marks read after
/// it must be those marks, or the book moved while the window was read. (An undivided window
/// read reports no marks of its own, so this closing read is the check.)
fn window_drift(pinned: CompanyMarks, closing: CompanyMarks) -> Option<&'static str> {
    (closing != pinned).then_some("voucher_window_changed_during_read")
}

/// The masters read after the window must be the masters read before it, in everything that
/// decides a classification, and under the same marks. A voucher mark that moved is the
/// window's problem, not the masters'.
fn masters_drifted(first: &RegisterMasters, second: &RegisterMasters) -> Option<&'static str> {
    if second.marks.vouchers != first.marks.vouchers {
        Some("voucher_window_changed_during_read")
    } else if second.index != first.index || second.marks.masters != first.marks.masters {
        Some("ledger_snapshot_drifted")
    } else {
        None
    }
}

/// What the tool returns for one window, except `state` and `reason`, which depend on whether
/// an empty window was corroborated.
pub(super) struct RegisterResult {
    pub(super) result: Value,
    pub(super) truncated: bool,
}

/// Validate the window, classify its vouchers, page the register and mark and redact every
/// party name in every list of the response.
pub(super) fn register_result(
    kind: RegisterKind,
    index: &MasterIndex,
    rows: Vec<Value>,
    (from, to): (&str, &str),
    (offset, limit): (usize, usize),
    redaction: Redaction,
) -> Result<RegisterResult, String> {
    // An undivided window read is admitted by its caller against the window: rows dated
    // outside it would otherwise be returned as register rows.
    let rows = validate_then_filter_voucher_rows(rows, from, to, None)?;
    let page = classify_register(kind, index, &rows)?;
    let total = page.rows.len();
    let (items, truncated, next_offset) = paginate(page.rows, offset, limit);
    let items = items
        .into_iter()
        .map(|row| redact_value(mark_register_row(row), redaction))
        .collect::<Vec<_>>();
    let side = |list: &[Value]| {
        let marked = list
            .iter()
            .cloned()
            .map(mark_register_side_item)
            .collect::<Vec<_>>();
        redact_value(bounded_list(&marked), redaction)
    };
    let [first_class, second_class] = kind.classes();
    let mut result = json!({
        "profile": kind.profile(),
        "register_classes": [first_class, second_class],
        "items": items,
        "offset": offset,
        "next_offset": next_offset,
        "total": total,
        "vouchers_observed": page.vouchers_observed,
        "ledger_masters_observed": index.len(),
        "other_voucher_types_touching_duties_taxes": side(&page.other_voucher_types),
        "unclassified_voucher_type": side(&page.unclassified_voucher_type),
        "vouchers_with_unplaced_ledgers": side(&page.vouchers_with_unplaced_ledgers),
        "coverage": format!(
            "items are the {first_class} and {second_class} vouchers that touch a ledger under Duties & Taxes; tax is taken only from the GST duty head recorded on a ledger master, never from a name or an amount; every other voucher type that touches those ledgers is listed apart (whether it belongs in a return is the CA's call); {first_class} and {second_class} vouchers with no entry on a Duties & Taxes ledger are counted in {}, not returned as items{}{}",
            kind.without_entry_key(),
            kind.cancelled_note(),
            kind.not_measured_note(),
        ),
    });
    result[kind.without_entry_key()] = side(&page.register_class_without_duties_taxes_entry);
    Ok(RegisterResult { result, truncated })
}

/// One page of the register: the rows from `offset`, at most `limit`, whether more remain and
/// where the next page starts.
fn paginate(rows: Vec<Value>, offset: usize, limit: usize) -> (Vec<Value>, bool, Option<usize>) {
    let total = rows.len();
    let page = rows
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();
    let truncated = offset.saturating_add(page.len()) < total;
    let next_offset = truncated.then_some(offset + page.len());
    (page, truncated, next_offset)
}

/// The masters as one read saw them, with the marks the read was pinned under.
struct RegisterMasters {
    index: MasterIndex,
    marks: CompanyMarks,
    evidence: Evidence,
}

impl Server {
    pub(super) async fn register(
        &self,
        kind: RegisterKind,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let from = normalized_date(required_string(args, "from")?)?;
        let to = normalized_date(required_string(args, "to")?)?;
        if from > to {
            return Err("invalid_date_range".to_string().into());
        }
        let (company, identity, mut evidence) = self.verified_company(guid).await?;
        let result: Result<ToolOutcome, ToolFailure> = async {
            // The masters first, pinned under the marks of their own read; the window is then
            // planned against those marks, and the company's marks are read once more after
            // it. An undivided window read takes opening marks only, so this closing read is
            // what shows a master or voucher that changed while the window was read.
            let first = self.read_register_masters(&identity).await?;
            evidence = combine_evidence(evidence.clone(), first.evidence.clone());
            let window = self
                .read_entry_window_shaped(
                    &identity,
                    &company.name,
                    &from,
                    &to,
                    Some(first.marks),
                    VoucherReadShape::ClassEntryWildcard,
                )
                .await?;
            evidence = combine_evidence(evidence.clone(), window.all_evidence());
            let (marks_xml, closing_evidence, _) = self
                .post_read_observing_boundary(&identity, company_high_water_read(&company.name))
                .await?;
            evidence = combine_evidence(evidence.clone(), closing_evidence);
            let (vouchers, masters) = parse_company_marks(&marks_xml, identity.company_guid())?;
            let closing = CompanyMarks { vouchers, masters };
            if let Some(code) = window_drift(first.marks, closing) {
                return Err(code.to_string().into());
            }
            // The two marks can be unchanged by an edit that does not move them (a duty head
            // or a parent changed in Tally's own screens is unmeasured), so the masters are
            // read again and must classify exactly as they did.
            let second = self.read_register_masters(&identity).await?;
            evidence = combine_evidence(evidence.clone(), second.evidence.clone());
            if let Some(code) = masters_drifted(&first, &second) {
                return Err(code.to_string().into());
            }
            // The window's label is the one rule `vouchers` and `voucher_presence` use
            // (`window_read`, #985, #1031): a non-empty window is `complete` only when every
            // row was admitted against a census of the window, and an empty one when its
            // corroboration read confirms it. A row's own `status` is about how its entries
            // classified and is not decided here.
            let counted = window.counted();
            let empty_window = if window.rows.is_empty() {
                let (corroboration, partial, corroboration_reason) = self
                    .corroborate_empty_voucher_read(
                        &identity,
                        &company.name,
                        &from,
                        &to,
                        None,
                        Some(first.marks),
                    )
                    .await?;
                evidence = combine_evidence(evidence.clone(), corroboration);
                Some((partial, corroboration_reason))
            } else {
                None
            };
            let (state, reason) = register_window_label(counted, empty_window);
            if state == "partial" {
                evidence.state = "partial";
                evidence.reason_code = reason.map(str::to_string);
            }
            let offset = arg_usize(args, "offset", 0)?;
            let limit = arg_positive_usize(args, "limit", self.settings.max_rows)?
                .min(self.settings.max_rows);
            let RegisterResult {
                mut result,
                truncated,
            } = register_result(
                kind,
                &first.index,
                window.rows,
                (&from, &to),
                (offset, limit),
                self.settings.redaction,
            )?;
            result["state"] = json!(state);
            result["reason"] = json!(reason);
            let payload = json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            });
            Ok(ToolOutcome {
                payload,
                evidence: evidence.clone(),
                company_guid: Some(guid.to_string()),
                truncated,
            })
        }
        .await;
        result.map_err(|failure| failure.with_prior_evidence(evidence))
    }

    /// One fresh compliance read of the ledger masters, classified, with the marks of the
    /// extent the read was pinned under.
    async fn read_register_masters(
        &self,
        identity: &VerifiedCompanyIdentity,
    ) -> Result<RegisterMasters, ToolFailure> {
        let today = bridge_tally_core::TallyDate::parse(tally_host_today())
            .map_err(|_| ToolFailure::from("current_date_invalid".to_string()))?;
        let listing = self
            .runtime
            .fetch_agent_party_ledger_masters_with_evidence(self.tally_config(), identity, today)
            .await
            .map_err(|error| ToolFailure::from_runtime("party_ledger_master_read_failed", error))?;
        let marks = CompanyMarks {
            vouchers: listing
                .extent
                .voucher_alter_id_high_water()
                .map_or(0, |mark| mark.get()),
            masters: listing
                .extent
                .master_alter_id_high_water()
                .map(|mark| mark.get())
                .ok_or_else(|| ToolFailure::from("register_master_mark_unavailable".to_string()))?,
        };
        let groups = GroupIndex::build(listing.groups);
        let excluded = listing
            .foreign_currency_ledgers_excluded
            .into_iter()
            .map(|ledger| ledger.ledger)
            .chain(listing.mixed_currency_ledgers_excluded);
        let index = MasterIndex::build(listing.records.iter(), &groups, excluded)?;
        Ok(RegisterMasters {
            index,
            marks,
            evidence: evidence_from_runtime_read(listing.evidence),
        })
    }
}

/// The register's `state` and `reason` for its window: the one rule `vouchers` and
/// `voucher_presence` use (`window_read`, #985), so the same window cannot be `complete` in a
/// register and `partial` there (#1031). `counted` is whether every row was admitted against a
/// census of the window; `empty_window` is the empty-window control's `(partial, reason)`, present
/// only when the window held no rows. The reason is given only with `partial`, as before.
fn register_window_label(
    counted: bool,
    empty_window: Option<(bool, Option<&'static str>)>,
) -> (&'static str, Option<&'static str>) {
    match window_read(counted, empty_window) {
        (WindowRead::Complete, _) => ("complete", None),
        (WindowRead::Partial, reason) => ("partial", reason),
    }
}

#[cfg(test)]
#[path = "agent_register_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "agent_register_server_tests.rs"]
mod server_tests;
