//! GST booked per counter ledger (read-only).
//!
//! Reads what the purchase and sales registers read (the same masters, window and drift checks)
//! and totals, per voucher, the entries on ledgers whose master carries a recognised GST duty
//! head against the one ledger on that voucher under an admitted group. A voucher that has no
//! single such ledger, or that the rules below cannot read with certainty, is set aside under a
//! named reason with its own totals: nothing is apportioned, netted or guessed. This is the
//! books side a CA starts from for expense-head-wise input tax credit; it is not a return
//! figure and says nothing about eligibility or about what was availed.
//!
//! Every total is folded from one `Disposition` per voucher, so a voucher's tax is counted in
//! exactly one place by construction.
use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_core::ExactDecimal;

use super::*;

const INDIRECT_EXPENSES: &str = "Indirect Expenses";
const DIRECT_EXPENSES: &str = "Direct Expenses";
const FIXED_ASSETS: &str = "Fixed Assets";

/// The groups whose single ledger on a voucher takes that voucher's GST. Only groups a live
/// capture has shown a GST voucher on are admitted: Purchase Accounts (purchases, a debit note
/// and an item invoice), Indirect Expenses (a purchase and a journal), Direct Expenses and Fixed
/// Assets (a purchase each, typed on Tally's screen; the `gst-ledger-l1` captures).
const COUNTER_GROUPS: [&str; 4] = [
    PURCHASE_ACCOUNTS,
    INDIRECT_EXPENSES,
    DIRECT_EXPENSES,
    FIXED_ASSETS,
];

const SUNDRY_CREDITORS: &str = "Sundry Creditors";

/// The groups whose ledgers settle a voucher (the party, cash, a bank) and so never stand in the
/// way of attributing its tax.
const SETTLEMENT_GROUPS: [&str; 5] = [
    "Sundry Creditors",
    "Sundry Debtors",
    "Cash-in-Hand",
    "Bank Accounts",
    "Bank OD A/c",
];

/// The voucher classes whose GST is attributed, and only when a ledger under Sundry Creditors is
/// on the voucher: a duty head does not say input or output, so a sale, a credit note (a discount
/// allowed with output tax) or a tax payment with no supplier on it (a GST challan that also pays
/// interest) would otherwise put output tax on an expense ledger. Any other class, a memorandum
/// or reversing journal among them, is set aside. Purchase and Debit Note are the classes a
/// capture shows attributed; Journal rests on a capture with a supplier on it.
const ADMITTED_CLASSES: [&str; 4] = ["Purchase", "Debit Note", "Journal", "Payment"];

/// Non-GST duty types a voucher may carry beside its GST without stopping the attribution: a
/// ledger typed TDS or TCS cannot hold GST. A Duties & Taxes ledger of any other type (`Others`
/// is what an imported ledger reads by default) may be a GST ledger left untyped, so it sets the
/// voucher aside.
const NON_GST_DUTY_TYPES: [&str; 2] = ["TDS", "TCS"];

/// Why a voucher's tax was not attributed to a ledger. The first that applies, in this order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SetAside {
    ClassNotAdmitted,
    HeadConflict,
    UnrecognisedHead,
    HeadAbsent,
    DutyLedgerWithoutGstType,
    UnresolvedGroup,
    SignAndFlagDisagree,
    EntriesDoNotBalance,
    SalesLedgerPresent,
    GstOnBothSides,
    OtherLedgersPresent,
    NoCreditorParty,
    NoCounterLedger,
    SeveralCounterLedgers,
    /// A Purchase that would be attributed, with no bill of its own, entered against a supplier
    /// bill that another attributed Purchase of the window opened for the same supplier ledger,
    /// with the same tax per head: it may be the same invoice keyed twice (#2069). Decided over
    /// the whole window, after every other reason.
    RepeatsAPurchaseBill,
}

impl SetAside {
    fn code(self) -> &'static str {
        match self {
            Self::ClassNotAdmitted => "class_not_admitted",
            Self::HeadConflict => "head_conflict",
            Self::UnrecognisedHead => "unrecognised_head",
            Self::HeadAbsent => "head_absent",
            Self::DutyLedgerWithoutGstType => "duty_ledger_without_gst_type",
            Self::UnresolvedGroup => "unresolved_group",
            Self::SignAndFlagDisagree => "sign_and_flag_disagree",
            Self::EntriesDoNotBalance => "entries_do_not_balance",
            Self::SalesLedgerPresent => "sales_ledger_present",
            Self::GstOnBothSides => "gst_on_both_sides",
            Self::OtherLedgersPresent => "other_ledgers_present",
            Self::NoCreditorParty => "no_creditor_party",
            Self::NoCounterLedger => "no_counter_ledger",
            Self::SeveralCounterLedgers => "several_counter_ledgers",
            Self::RepeatsAPurchaseBill => "repeats_a_purchase_bill",
        }
    }
}

/// Tax per recognised head, as the books state it: debit-side amounts are negative.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct HeadTotals(BTreeMap<String, ExactDecimal>);

impl HeadTotals {
    /// Every total starts from zero, so one amount and a sum print alike, in the canonical form
    /// the voucher summaries print (`-3600`, not `-3600.00`).
    fn add(&mut self, head: &str, amount: &ExactDecimal) -> Result<(), String> {
        let total = self
            .0
            .get(head)
            .unwrap_or(&ExactDecimal::zero())
            .checked_add(amount)
            .map_err(|_| "voucher_amount_invalid".to_string())?;
        self.0.insert(head.to_string(), total);
        Ok(())
    }

    fn absorb(&mut self, other: &Self) -> Result<(), String> {
        other
            .0
            .iter()
            .try_for_each(|(head, amount)| self.add(head, amount))
    }

    fn to_json(&self) -> Value {
        Value::Object(
            self.0
                .iter()
                .map(|(head, amount)| (head.clone(), json!(amount.as_str())))
                .collect(),
        )
    }
}

/// What one voucher contributes. Every total in the answer is a fold over these.
#[derive(Debug, PartialEq)]
enum Disposition {
    /// Cancelled, optional or post-dated: outside the books' totals, counted only.
    NotInBooks,
    /// No entry on a Duties & Taxes ledger that carries or may carry a GST head.
    NoGstEntry,
    Attributed {
        ledger: String,
        debit: HeadTotals,
        credit: HeadTotals,
    },
    SetAside {
        reason: SetAside,
        identity: Value,
        debit: HeadTotals,
        credit: HeadTotals,
    },
}

fn flagged(row: &Value, key: &str) -> bool {
    row.get(key).and_then(Value::as_bool) == Some(true)
}

/// Decide one voucher. A ledger the masters do not list refuses the whole read, as in the
/// registers.
fn dispose(index: &MasterIndex, row: &Value) -> Result<Disposition, String> {
    if ["cancelled", "optional", "post_dated"]
        .iter()
        .any(|key| flagged(row, key))
    {
        return Ok(Disposition::NotInBooks);
    }
    let entries = row
        .get("amounts")
        .and_then(Value::as_array)
        .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
    let (mut debit, mut credit) = (HeadTotals::default(), HeadTotals::default());
    let mut found = BTreeSet::new();
    let mut counters = BTreeSet::new();
    let (mut sales, mut others, mut any_gst, mut creditor) = (false, false, false, false);
    let mut balance = ExactDecimal::zero();
    for entry in entries {
        let name = entry
            .get("ledger")
            .and_then(Value::as_str)
            .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
        let amount = ExactDecimal::parse(
            entry
                .get("amount")
                .and_then(Value::as_str)
                .ok_or_else(|| "voucher_amount_invalid".to_string())?,
        )
        .map_err(|_| "voucher_amount_invalid".to_string())?;
        balance = balance
            .checked_add(&amount)
            .map_err(|_| "voucher_amount_invalid".to_string())?;
        let ledger = index.by_name.get(name).ok_or_else(|| {
            if index.excluded.contains(name) {
                "register_ledger_currency_excluded".to_string()
            } else {
                "ledger_snapshot_drifted".to_string()
            }
        })?;
        match (ledger.group, &ledger.head) {
            (LedgerGroup::DutiesAndTaxes, GstDutyHeadObservation::Recognized { head, .. }) => {
                any_gst = true;
                if amount.is_zero() {
                    continue;
                }
                let is_debit = amount.is_negative();
                match entry.get("is_deemed_positive").and_then(Value::as_str) {
                    Some("Yes") if !is_debit => found.insert(SetAside::SignAndFlagDisagree),
                    Some("No") if is_debit => found.insert(SetAside::SignAndFlagDisagree),
                    _ => false,
                };
                let head = serde_json::to_value(head)
                    .ok()
                    .and_then(|value| value.as_str().map(str::to_string))
                    .ok_or_else(|| "agent_read_protocol_invalid".to_string())?;
                let side = if is_debit { &mut debit } else { &mut credit };
                side.add(&head, &amount)?;
            }
            // A TDS or TCS ledger settles part of the voucher; it is not GST.
            (LedgerGroup::DutiesAndTaxes, GstDutyHeadObservation::NotTaxLedger { tax_type })
                if NON_GST_DUTY_TYPES.contains(&tax_type.trim()) => {}
            (LedgerGroup::DutiesAndTaxes, GstDutyHeadObservation::NotTaxLedger { .. }) => {
                any_gst = true;
                found.insert(SetAside::DutyLedgerWithoutGstType);
            }
            (LedgerGroup::DutiesAndTaxes, GstDutyHeadObservation::Absent) => {
                any_gst = true;
                found.insert(SetAside::HeadAbsent);
            }
            (LedgerGroup::DutiesAndTaxes, GstDutyHeadObservation::Unrecognized { .. }) => {
                any_gst = true;
                found.insert(SetAside::UnrecognisedHead);
            }
            (LedgerGroup::DutiesAndTaxes, GstDutyHeadObservation::Contradictory { .. }) => {
                any_gst = true;
                found.insert(SetAside::HeadConflict);
            }
            // A ledger whose group does not resolve may be a GST ledger: the voucher is set aside
            // and listed, never counted as having no GST.
            (LedgerGroup::Unresolved(_), _) => {
                any_gst = true;
                found.insert(SetAside::UnresolvedGroup);
            }
            _ => match ledger.reserved.as_deref() {
                Some(group) if COUNTER_GROUPS.contains(&group) => {
                    counters.insert(name.to_string());
                }
                Some(SALES_ACCOUNTS) => sales = true,
                Some(SUNDRY_CREDITORS) => creditor = true,
                Some(group) if SETTLEMENT_GROUPS.contains(&group) => {}
                _ => others = true,
            },
        }
    }
    if !any_gst {
        return Ok(Disposition::NoGstEntry);
    }
    let class = row.get("voucher_class").and_then(Value::as_str);
    if !class.is_some_and(|class| ADMITTED_CLASSES.contains(&class)) {
        found.insert(SetAside::ClassNotAdmitted);
    }
    // Entries that do not sum to zero mean a ledger sits where this read does not look (an item
    // invoice of a shape not captured may hold its purchase ledger in an inventory allocation);
    // another ledger on it (freight, round-off) must then never take the whole invoice's tax.
    // The item invoices typed on TallyPrime 7.2's screen did list the purchase ledger among
    // their entries (the `gst-ledger-l1` captures of 14 and 17 November).
    if !balance.is_zero() {
        found.insert(SetAside::EntriesDoNotBalance);
    }
    if sales {
        found.insert(SetAside::SalesLedgerPresent);
    }
    if !debit.0.is_empty() && !credit.0.is_empty() {
        found.insert(SetAside::GstOnBothSides);
    }
    if others {
        found.insert(SetAside::OtherLedgersPresent);
    }
    if !creditor {
        found.insert(SetAside::NoCreditorParty);
    }
    match counters.len() {
        0 => found.insert(SetAside::NoCounterLedger),
        1 => false,
        _ => found.insert(SetAside::SeveralCounterLedgers),
    };
    Ok(match found.first() {
        Some(&reason) => Disposition::SetAside {
            reason,
            identity: identity(row),
            debit,
            credit,
        },
        None => Disposition::Attributed {
            ledger: counters
                .pop_first()
                .ok_or_else(|| "agent_read_protocol_invalid".to_string())?,
            debit,
            credit,
        },
    })
}

/// How a set-aside voucher is listed.
fn identity(row: &Value) -> Value {
    json!({
        "date": row.get("date"),
        "voucher_number": row.get("voucher_number"),
        "voucher_type": row.get("voucher_type"),
        "voucher_class": row.get("voucher_class"),
        "guid": row.get("guid"),
    })
}

/// The named bills of `bill_type` on the Sundry Creditors entries of a Purchase in the books,
/// each as its ledger and the bill's name. Any other voucher has none.
fn purchase_bills(index: &MasterIndex, row: &Value, bill_type: &str) -> Vec<(String, String)> {
    let in_books = !["cancelled", "optional", "post_dated"]
        .iter()
        .any(|key| flagged(row, key));
    if !in_books || row.get("voucher_class").and_then(Value::as_str) != Some("Purchase") {
        return Vec::new();
    }
    let entries = row.get("amounts").and_then(Value::as_array);
    entries
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let ledger = entry.get("ledger").and_then(Value::as_str)?;
            let creditor = index.by_name.get(ledger)?.reserved.as_deref() == Some(SUNDRY_CREDITORS);
            creditor.then_some((ledger, entry.get("bill_allocations")?.as_array()?))
        })
        .flat_map(|(ledger, bills)| {
            bills.iter().filter_map(move |bill| {
                let typed = bill.get("bill_type").and_then(Value::as_str)?.trim() == bill_type;
                let reference = bill.get("reference")?;
                let named = reference.get("kind").and_then(Value::as_str) == Some("named");
                let name = reference.get("name").and_then(Value::as_str)?;
                (typed && named).then(|| (ledger.to_string(), name.to_string()))
            })
        })
        .collect()
}

/// Which row may repeat a supplier invoice, mapped to the row it repeats: an attributed Purchase
/// with no bill of its own (`New Ref`), entered against a bill (`Agst Ref`) that an attributed
/// Purchase in the books of this window opened for the same creditor ledger, with the same tax
/// per head on each side. Only attributed rows take part, so the first copy keeps its tax and
/// nothing already set aside moves. Not seen: a first copy outside the window or not attributed,
/// a copy with a bill of its own or no bill, a copy keyed as another class, and one whose tax
/// differs.
fn repeated_purchase_bills(
    index: &MasterIndex,
    rows: &[Value],
    dispositions: &[Disposition],
) -> BTreeMap<usize, usize> {
    let taxes = |position: usize| match &dispositions[position] {
        Disposition::Attributed { debit, credit, .. } => Some((debit, credit)),
        _ => None,
    };
    // Every Purchase that opened a bill: only an attributed one can match, by its tax.
    let mut opened: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (position, row) in rows.iter().enumerate() {
        for bill in purchase_bills(index, row, "New Ref") {
            opened.entry(bill).or_default().push(position);
        }
    }
    rows.iter()
        .enumerate()
        .filter(|&(position, row)| {
            taxes(position).is_some() && purchase_bills(index, row, "New Ref").is_empty()
        })
        .filter_map(|(position, row)| {
            purchase_bills(index, row, "Agst Ref")
                .iter()
                .filter_map(|bill| opened.get(bill))
                .flatten()
                .copied()
                .find(|&first| taxes(first) == taxes(position))
                .map(|first| (position, first))
        })
        .collect()
}

#[derive(Debug, Default)]
struct Bucket {
    vouchers: usize,
    debit: HeadTotals,
    credit: HeadTotals,
    listed: Vec<Value>,
}

/// The fold of one window's dispositions.
#[derive(Debug, Default)]
struct GstByLedger {
    ledgers: BTreeMap<String, Bucket>,
    set_aside: BTreeMap<SetAside, Bucket>,
    not_in_books: usize,
    without_gst_entry: usize,
    observed: usize,
}

fn fold(index: &MasterIndex, rows: &[Value]) -> Result<GstByLedger, String> {
    let mut out = GstByLedger {
        observed: rows.len(),
        ..GstByLedger::default()
    };
    let dispositions = rows
        .iter()
        .map(|row| dispose(index, row))
        .collect::<Result<Vec<_>, _>>()?;
    let repeats = repeated_purchase_bills(index, rows, &dispositions);
    for (position, (row, disposition)) in rows.iter().zip(dispositions).enumerate() {
        let disposition = match (disposition, repeats.get(&position)) {
            (Disposition::Attributed { debit, credit, .. }, Some(&first)) => {
                let mut identity = identity(row);
                identity["repeats"] = json!({
                    "date": rows[first].get("date"),
                    "voucher_number": rows[first].get("voucher_number"),
                    "guid": rows[first].get("guid"),
                });
                Disposition::SetAside {
                    reason: SetAside::RepeatsAPurchaseBill,
                    identity,
                    debit,
                    credit,
                }
            }
            (disposition, _) => disposition,
        };
        let (bucket, debit, credit) = match disposition {
            Disposition::NotInBooks => {
                out.not_in_books += 1;
                continue;
            }
            Disposition::NoGstEntry => {
                out.without_gst_entry += 1;
                continue;
            }
            Disposition::Attributed {
                ledger,
                debit,
                credit,
            } => (out.ledgers.entry(ledger).or_default(), debit, credit),
            Disposition::SetAside {
                reason,
                identity,
                debit,
                credit,
            } => {
                let bucket = out.set_aside.entry(reason).or_default();
                bucket.listed.push(identity);
                (bucket, debit, credit)
            }
        };
        bucket.vouchers += 1;
        bucket.debit.absorb(&debit)?;
        bucket.credit.absorb(&credit)?;
    }
    Ok(out)
}

/// A window that may be totalled: one whose rows were admitted against a census, or an empty one
/// whose corroboration read confirmed it. A partial window has no totals.
struct CompleteWindow {
    index: MasterIndex,
    rows: Vec<Value>,
}

impl CompleteWindow {
    fn admit(window: RegisterWindow) -> Result<Self, String> {
        if window.state != "complete" {
            return Err("gst_by_ledger_window_partial".to_string());
        }
        Ok(Self {
            index: window.index,
            rows: window.rows,
        })
    }
}

fn gst_by_ledger_result(index: &MasterIndex, totals: &GstByLedger, redaction: Redaction) -> Value {
    let counter_ledgers = totals
        .ledgers
        .iter()
        .map(|(name, bucket)| {
            let ledger = index.by_name.get(name);
            let parent = ledger.and_then(|ledger| ledger.parent.as_deref());
            let under_root = parent.is_some_and(bridge_tally_protocol::is_tally_reserved_root);
            let mut item = json!({
                "ledger": name,
                "ledger_guid": index.guid_of(name),
                "group": ledger.and_then(|ledger| ledger.reserved.as_deref()),
                "parent": parent.map(bridge_tally_protocol::tally_root_as_sent),
                "vouchers": bucket.vouchers,
                "debit_side_tax": bucket.debit.to_json(),
                "credit_side_tax": bucket.credit.to_json(),
            });
            mark_party_field(&mut item, "ledger");
            // A parent the book named itself (a sub-group) is masked like a ledger name; a
            // parent that is the predefined group itself, or Tally's root, is not a name of
            // the book's.
            if !under_root && item["parent"] != item["group"] {
                mark_party_field(&mut item, "parent");
            }
            item
        })
        .collect::<Vec<_>>();
    let set_aside = totals
        .set_aside
        .iter()
        .map(|(reason, bucket)| {
            (
                reason.code().to_string(),
                json!({
                    "vouchers": bucket.vouchers,
                    "debit_side_tax": bucket.debit.to_json(),
                    "credit_side_tax": bucket.credit.to_json(),
                    "listed": bounded_list(&bucket.listed),
                }),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    redact_value(
        json!({
            "profile": "agent_gst_by_counter_ledger_v1",
            "counter_ledgers": counter_ledgers,
            "set_aside": set_aside,
            "vouchers_observed": totals.observed,
            "vouchers_without_gst_entry": totals.without_gst_entry,
            "vouchers_cancelled_optional_or_post_dated": totals.not_in_books,
            "ledger_masters_observed": index.len(),
            "coverage": COVERAGE,
        }),
        redaction,
    )
}

const COVERAGE: &str = "GST booked in the books, not a return figure: per voucher, the entries on Duties & Taxes ledgers whose master carries a recognised GST duty head, totalled against the one ledger on that voucher under Purchase Accounts, Direct Expenses, Indirect Expenses or Fixed Assets (groups a live capture has shown on a purchase), only on a Purchase, Debit Note, Journal or Payment that has a Sundry Creditors ledger on it; debit_side_tax is negative as the books state it and credit_side_tax positive, never netted; a voucher with no such single ledger, or one these rules cannot read with certainty, is set aside under its reason with its own totals; nothing is apportioned; GST booked to a ledger outside Duties & Taxes, or to a ledger with no recognised head, is not attributed; eligibility, availment, input versus output and any return table are not decided";

/// The tool's description, without the receipt sentence the catalogue appends to every read.
pub(in crate::agent) const DESCRIPTION: &str = "Read-only: the GST the books record, totalled per counter ledger, for a date window; not a return figure. It does not decide input tax credit eligibility, blocked credit, what was availed, input versus output, or which row of any return a ledger belongs in, and matches nothing against any portal. It reads what purchase_register and sales_register read (the ledger masters twice, the full voucher window, the same drift checks and refusals) and, for every voucher with an entry on a ledger under Duties & Taxes whose master carries a recognised GST duty head, takes that tax per head and attributes it to the voucher's one ledger under Purchase Accounts, Direct Expenses, Indirect Expenses or Fixed Assets, only when the voucher is a Purchase, Debit Note, Journal or Payment with a ledger under Sundry Creditors on it, has exactly one such counter ledger (a round-off ledger under Indirect Expenses counts as one), and nothing besides but the party, cash, bank and TDS or TCS ledgers: a duty head does not say input or output, so a sale, a credit note or a tax payment with no supplier on it is never attributed. `counter_ledgers` lists each such ledger with its `ledger_guid` (Tally's GUID of the ledger, which trial_balance returns as the ledger's `guid`; its movement there covers every voucher on it, set-aside ones included, so it is never a base for this tax), its predefined `group`, its `parent` (masked like a ledger name when it is a group the book named), `vouchers`, `debit_side_tax` (negative, as the books state it) and `credit_side_tax` (positive: tax booked back, which can be a reversal or output tax on an asset or goods returned or sold to a supplier; the tool does not tell them apart), per head (cgst, igst, state_tax, sgst_utgst, ut_tax, cess; state_tax and sgst_utgst are both state-side and kept apart), never netted. Every other voucher with GST is set aside in `set_aside` under the first reason that applies, with its count, its own tax totals and at most 100 vouchers listed: class_not_admitted (any class but Purchase, Debit Note, Journal and Payment: sales, credit notes, memoranda), head_conflict, unrecognised_head, head_absent (a Duties & Taxes ledger that may be GST has no head), duty_ledger_without_gst_type (a Duties & Taxes ledger typed neither GST nor TDS or TCS, such as Others, which may be a GST ledger left untyped), unresolved_group (a ledger whose group does not resolve, which may be a GST ledger), sign_and_flag_disagree, entries_do_not_balance (a ledger sits where this read does not look, such as an item invoice of a shape not measured whose purchase ledger is in an inventory allocation), sales_ledger_present (sales and credit notes), gst_on_both_sides (reverse-charge and set-off journals), other_ledgers_present (a ledger under any other group, such as Current Assets or Stock-in-Hand), no_creditor_party (no ledger under Sundry Creditors, such as a GST challan payment), no_counter_ledger, several_counter_ledgers, and repeats_a_purchase_bill (a Purchase with no bill of its own, against the bill another attributed Purchase of the window opened for that supplier ledger, with equal tax per head: maybe one invoice keyed twice; `repeats` names that one, still attributed). Nothing is apportioned. Cancelled, optional and post-dated vouchers are counted, not totalled (`vouchers_cancelled_optional_or_post_dated`); `vouchers_without_gst_entry` counts the vouchers with no Duties & Taxes entry of any kind but TDS or TCS, and `vouchers_observed` every voucher of the window. A window that is not complete is refused (gst_by_ledger_window_partial). Ledger names are masked like vouchers masks them. Measured on one synthetic book of TallyPrime 7.2 Silver: fourteen vouchers typed on Tally's screen, read one day at a time and, by the build before Direct Expenses and Fixed Assets were admitted, as one month of 64 vouchers that the window read in two parts (the shapes the agent guide lists). A registered book's December, read once by an earlier build, held an invoice keyed twice; the second copy now reads as repeats_a_purchase_bill. Not detected: a first copy outside the window, a copy with its own bill, none, or another class. Not measured: vouchers created by import, Gold and other releases, a registered book on this build, other item-invoice shapes, more than 64 vouchers in a window. A year on a large book may be refused as too large by the window read: use shorter windows.";

impl Server {
    pub(in crate::agent) async fn gst_by_counter_ledger(
        &self,
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
            let window = self
                .read_register_window(&identity, &company.name, (&from, &to), &mut evidence)
                .await?;
            let CompleteWindow { index, rows } = CompleteWindow::admit(window)?;
            let rows = validate_then_filter_voucher_rows(rows, from.as_str(), to.as_str(), None)?;
            let totals = fold(&index, &rows)?;
            let mut result = gst_by_ledger_result(&index, &totals, self.settings.redaction);
            result["state"] = json!("complete");
            let payload = json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            });
            Ok(ToolOutcome {
                payload,
                evidence: evidence.clone(),
                company_guid: Some(guid.to_string()),
                truncated: false,
            })
        }
        .await;
        result.map_err(|failure| failure.with_prior_evidence(evidence))
    }
}

#[cfg(test)]
#[path = "agent_gst_by_ledger_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "agent_gst_by_ledger_server_tests.rs"]
mod server_tests;
