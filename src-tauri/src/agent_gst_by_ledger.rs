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

/// The groups whose single ledger on a voucher takes that voucher's GST. Only groups a live
/// capture has shown a GST voucher on are admitted: Purchase Accounts (purchases and a debit
/// note) and Indirect Expenses (a journal). Direct Expenses and Fixed Assets wait for their own
/// capture; until then a voucher on one of them is set aside, not attributed.
const COUNTER_GROUPS: [&str; 2] = [PURCHASE_ACCOUNTS, INDIRECT_EXPENSES];

/// The groups whose ledgers settle a voucher (the party, cash, a bank) and so never stand in the
/// way of attributing its tax.
const SETTLEMENT_GROUPS: [&str; 5] = [
    "Sundry Creditors",
    "Sundry Debtors",
    "Cash-in-Hand",
    "Bank Accounts",
    "Bank OD A/c",
];

/// The voucher classes the captures show; any other class (a memorandum or reversing journal,
/// or one no capture covers) is set aside, so its tax never reaches a ledger's total.
const ADMITTED_CLASSES: [&str; 6] = [
    "Purchase",
    "Debit Note",
    "Journal",
    "Payment",
    "Sales",
    "Credit Note",
];

/// Why a voucher's tax was not attributed to a ledger. The first that applies, in this order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SetAside {
    ClassNotAdmitted,
    HeadConflict,
    UnrecognisedHead,
    HeadAbsent,
    UnresolvedGroup,
    SignAndFlagDisagree,
    EntriesDoNotBalance,
    SalesLedgerPresent,
    GstOnBothSides,
    OtherLedgersPresent,
    NoCounterLedger,
    SeveralCounterLedgers,
}

impl SetAside {
    fn code(self) -> &'static str {
        match self {
            Self::ClassNotAdmitted => "class_not_admitted",
            Self::HeadConflict => "head_conflict",
            Self::UnrecognisedHead => "unrecognised_head",
            Self::HeadAbsent => "head_absent",
            Self::UnresolvedGroup => "unresolved_group",
            Self::SignAndFlagDisagree => "sign_and_flag_disagree",
            Self::EntriesDoNotBalance => "entries_do_not_balance",
            Self::SalesLedgerPresent => "sales_ledger_present",
            Self::GstOnBothSides => "gst_on_both_sides",
            Self::OtherLedgersPresent => "other_ledgers_present",
            Self::NoCounterLedger => "no_counter_ledger",
            Self::SeveralCounterLedgers => "several_counter_ledgers",
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
    let (mut sales, mut others, mut any_gst) = (false, false, false);
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
            // A TDS or other non-GST duty ledger settles part of the voucher; it is not GST.
            (LedgerGroup::DutiesAndTaxes, GstDutyHeadObservation::NotTaxLedger { .. }) => {}
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
            (LedgerGroup::Unresolved(_), _) => {
                found.insert(SetAside::UnresolvedGroup);
            }
            _ => match ledger.reserved.as_deref() {
                Some(group) if COUNTER_GROUPS.contains(&group) => {
                    counters.insert(name.to_string());
                }
                Some(SALES_ACCOUNTS) => sales = true,
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
    // An item invoice holds its purchase or sales ledger in an inventory allocation this read
    // does not fetch, so its ledger entries do not sum to zero; another ledger on it (freight,
    // round-off) must never take the whole invoice's tax.
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
    match counters.len() {
        0 => found.insert(SetAside::NoCounterLedger),
        1 => false,
        _ => found.insert(SetAside::SeveralCounterLedgers),
    };
    Ok(match found.first() {
        Some(&reason) => Disposition::SetAside {
            reason,
            identity: json!({
                "date": row.get("date"),
                "voucher_number": row.get("voucher_number"),
                "voucher_type": row.get("voucher_type"),
                "voucher_class": class,
                "guid": row.get("guid"),
            }),
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
    for row in rows {
        let (bucket, debit, credit) = match dispose(index, row)? {
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
            let mut item = json!({
                "ledger": name,
                "group": ledger.and_then(|ledger| ledger.reserved.as_deref()),
                "parent": ledger.and_then(|ledger| ledger.parent.as_deref()),
                "vouchers": bucket.vouchers,
                "debit_side_tax": bucket.debit.to_json(),
                "credit_side_tax": bucket.credit.to_json(),
            });
            mark_party_field(&mut item, "ledger");
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

const COVERAGE: &str = "GST booked in the books, not a return figure: per voucher, the entries on Duties & Taxes ledgers whose master carries a recognised GST duty head, totalled against the one ledger on that voucher under Purchase Accounts or Indirect Expenses (the groups a live capture has shown); debit_side_tax is negative as the books state it and credit_side_tax positive, never netted; a voucher with no such single ledger, or one these rules cannot read with certainty, is set aside under its reason with its own totals; nothing is apportioned; GST booked to a ledger outside Duties & Taxes, or to a ledger with no recognised head, is not attributed; eligibility, availment, input versus output and any return table are not decided";

/// The tool's description, without the receipt sentence the catalogue appends to every read.
pub(in crate::agent) const DESCRIPTION: &str = "Read-only: the GST the books record, totalled per counter ledger, for a date window; not a return figure. It does not decide input tax credit eligibility, blocked credit, what was availed, input versus output, or which row of any return a ledger belongs in, and matches nothing against any portal. It reads what purchase_register and sales_register read (the ledger masters twice, the full voucher window, the same drift checks and refusals) and, for every voucher with an entry on a ledger under Duties & Taxes whose master carries a recognised GST duty head, takes that tax per head and attributes it to the voucher's one ledger under Purchase Accounts or Indirect Expenses, when the voucher has exactly one such ledger and nothing but the party, cash, bank and non-GST duty ledgers besides. `counter_ledgers` lists each such ledger with its predefined `group`, its `parent`, `vouchers`, `debit_side_tax` (negative, as the books state it) and `credit_side_tax` (positive), per head (cgst, igst, state_tax, sgst_utgst, ut_tax, cess; state_tax and sgst_utgst are both state-side and kept apart), never netted. Every other voucher with GST is set aside in `set_aside` under the first reason that applies, with its count, its own tax totals and at most 100 vouchers listed: class_not_admitted (a class other than Purchase, Debit Note, Journal, Payment, Sales and Credit Note, such as a memorandum), head_conflict, unrecognised_head, head_absent (a Duties & Taxes ledger that may be GST has no head), unresolved_group, sign_and_flag_disagree, entries_do_not_balance (for example an item invoice, whose purchase or sales ledger sits in an inventory allocation this read does not fetch), sales_ledger_present (sales and credit notes), gst_on_both_sides (reverse-charge and set-off journals), other_ledgers_present (a ledger under any other group, such as Direct Expenses, Fixed Assets, Current Assets or Stock-in-Hand), no_counter_ledger (tax payments) and several_counter_ledgers. Nothing is apportioned. Cancelled, optional and post-dated vouchers are counted, not totalled. A window that is not complete is refused (gst_by_ledger_window_partial). Ledger names are masked like vouchers masks them. Measured on one synthetic book of TallyPrime 7.1 Silver, one month of eight vouchers created by import: purchases on Purchase Accounts ledgers, a debit note, a sale, a journal on an Indirect Expenses ledger and a tax payment; not measured: vouchers typed on Tally's screen, item invoices, Direct Expenses and Fixed Assets ledgers, windows of more than eight vouchers or read in parts, Gold and other releases. A year on a large book may be refused as too large by the window read: use shorter windows.";

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
