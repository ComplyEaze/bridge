//! Cash payments/receipts tests keyed off the books' cash ledgers, a port of the reference
//! Python implementation's `cash_payments_40a3` test module, version 1:
//!   (a) s.40A(3): cash paid to one payee in one day, over the per-person-per-day limit.
//!   (b) s.269ST limb (a) limb (i): cash received from one party in one day, at or over the
//!       limit. (Limbs (ii)/(iii) need bill-wise/event linkage the books do not carry; out of
//!       scope here, stated in the finding.)
//!   (b2) s.269ST limb (a), PAYMENT leg: cash paid to one party in one day, at or over the
//!       limit. s.269ST binds the receiver only; a cash payment by the assessee is never a
//!       contravention or a penalty exposure under s.269ST/s.271DA for the assessee -- it is a
//!       reporting duty under Form 3CD clause 31(bc) (limb (a) only). Every payment-side finding
//!       says exactly that, in words that never imply the assessee has breached anything.
//!   (c) s.269SS/269T candidates: a voucher with both a cash line and a Loans (Liability) line,
//!       at or over the limit; direction (loan accepted/repaid) from the cash sign.
//!
//! Double-count fix (mirrors the reference module's own 2026-09-17 note): (b)/(b2) above never
//! carry a Clause 31 tag -- a separate books-wide party-day scan (not yet ported) already tags
//! every cash receipt/payment 3CD-31(ba)/3CD-31(bc), so tagging it here too would double-count
//! the same voucher. (c) above keeps its Clause 31 tag only for a loan ledger NOT in the
//! client's `[loans]` configuration; a configured ledger is assumed already tagged by a
//! per-lender scan (not yet ported) and is an observation only here.
//!
//! Payee/party attribution comes from each non-cash ledger LINE on a voucher, never from a
//! single party field (a voucher paying two people in cash the same day must attribute to
//! both). Contra is excluded outright. s.40A(3) exclusion walks a payee ledger's full group
//! CHAIN, not just its immediate parent, so a ledger one level below "Loans (Liability)" is
//! still excluded.

use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_primitives::TallyDate;
use sha2::Sha256;

use crate::book::{Book, Ledger, Voucher};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::ledger_ids::stable_ledger_tag;
use crate::read::iso;
use crate::rules::Rules;
use crate::support::PrintedNames;

pub const TEST_ID: &str = "cash_payments_40a3";
pub const VERSION: &str = "1";

const POPULATION: &str = "Books population (optional, cancelled and post-dated vouchers \
excluded); Contra excluded throughout.";

/// Not a real ledger name: the sentinel for a cash leg with a tax/round-off line but no
/// identified real party (module docstring).
const UNIDENTIFIED_PARTY: &str = "\u{2039}cash leg with no identified party\u{203a}";

/// How an s.269ST finding on that bucket cites it: a "row" ref, never a "ledger" one -- the
/// placeholder names no ledger in the books (EVID-1; the reference's `UNIDENTIFIED_PARTY_ROW`).
const UNIDENTIFIED_PARTY_ROW: &str = "cash_payments_40a3:unidentified_party";

/// The party an s.269ST row is about, as evidence: its ledger, or the no-party bucket as a row.
fn party_ref(ledger_name: &str, row: &RowAgg<'_>) -> EvidenceRef {
    if ledger_name == UNIDENTIFIED_PARTY {
        EvidenceRef::with_label(
            "row",
            UNIDENTIFIED_PARTY_ROW,
            &row.names.label(UNIDENTIFIED_PARTY),
        )
    } else {
        EvidenceRef::new("ledger", ledger_name)
    }
}

/// Tally's own reserved/standard group names (not client data) that put a payee ledger out of
/// s.40A(3) "expenditure" scope, keyed by the rules-configured role name. A ledger's chain
/// contains at most one of these in practice (they sit under different primary groups).
const GROUP_BY_KIND: [(&str, &str); 5] = [
    ("capital", "Capital Account"),
    ("loans_liability", "Loans (Liability)"),
    ("loans_advances_asset", "Loans & Advances (Asset)"),
    ("fixed_assets", "Fixed Assets"),
    ("duties_taxes", "Duties & Taxes"),
];
/// The s.40A(3) rows whose ledger names what was bought, not whom it was paid to: such a row may
/// pool several people's cash, and says so.
const POOLING_LEDGER_GROUPS: [&str; 3] =
    ["Purchase Accounts", "Direct Expenses", "Indirect Expenses"];
const SALES_ACCOUNTS_GROUP: &str = "Sales Accounts";
const PURCHASE_ACCOUNTS_GROUP: &str = "Purchase Accounts";
const LOANS_LIABILITY_GROUP: &str = "Loans (Liability)";
const DUTIES_TAXES_GROUP: &str = "Duties & Taxes";

fn overflow() -> AuditError {
    AuditError::Config("cash_payments_40a3: a total overflowed i64 paise".to_string())
}

/// A generic transport-name match (heuristic only, stated as such in every finding that uses
/// it): the reference engine's own regex, searched with its `re.I` semantics
/// (`support::py_re_search`). Not a hardcoded list of staff or client names.
const TRANSPORT_NAME_RE: &str = "FREIGHT|TRANSPORT|ROAD\\s?LINES|CARRIER|LOGISTIC|CARGO|ROADWAYS";

fn transport_name_match(name: &str) -> bool {
    static ALTS: std::sync::OnceLock<Vec<Vec<crate::support::ReTok>>> = std::sync::OnceLock::new();
    let alts = ALTS.get_or_init(|| crate::support::re_alternatives(TRANSPORT_NAME_RE));
    let alts: Vec<&[crate::support::ReTok]> = alts.iter().map(Vec::as_slice).collect();
    crate::support::py_re_search(name, &alts)
}

/// The reference implementation's `_hash` used inline for a voucher GUID at 12 hex characters
/// (sha256, not sha1 -- the s.269SS/269T row id mixes both hashes deliberately, matching the
/// Python source exactly).
fn hash12_sha256(text: &str) -> String {
    use sha2::Digest;
    crate::canonical::hex(&Sha256::digest(text.as_bytes()))[..12].to_string()
}

/// Which s.40A(3) exclusion role (if any) a payee ledger falls under, by its full group chain
/// (not just the immediate parent).
fn classify_kind(ledger: Option<&Ledger>) -> &'static str {
    let Some(ledger) = ledger else {
        return "expenditure";
    };
    for (kind, group) in GROUP_BY_KIND {
        if ledger.under(group) {
            return kind;
        }
    }
    "expenditure"
}

fn group_for_kind(kind: &str) -> Result<&'static str> {
    GROUP_BY_KIND
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, g)| *g)
        .ok_or_else(|| {
            AuditError::Config(format!(
                "cash_payments_40a3: rules.s40a3.excluded_group_roles names unknown role {kind:?}"
            ))
        })
}

use crate::support::{rupees, voucher_label};

/// One (date, ledger) row's aggregate: cash amount and the distinct vouchers that contributed.
pub(crate) struct RowAgg<'a> {
    pub(crate) paise: i64,
    pub(crate) vouchers: BTreeMap<String, &'a Voucher>,
    /// The names its vouchers print: shown with a pooling s.40A(3) row or a pooled s.269ST row
    /// with no party ledger, never used to key or attribute.
    names: PrintedNames,
}

impl<'a> RowAgg<'a> {
    fn new() -> Self {
        Self {
            paise: 0,
            vouchers: BTreeMap::new(),
            names: PrintedNames::default(),
        }
    }

    fn add(&mut self, amount: i64, v: &'a Voucher) -> Result<()> {
        self.paise = self.paise.checked_add(amount).ok_or_else(overflow)?;
        self.vouchers.insert(v.guid.clone(), v);
        Ok(())
    }
}

pub(crate) type RowMap<'a> = BTreeMap<(TallyDate, String), RowAgg<'a>>;

/// Evidence refs for one voucher map: sorted by guid, each carrying the fixed voucher label the
/// canonical serialiser compares.
fn evidence_for_vouchers(vouchers: &BTreeMap<String, &Voucher>) -> Vec<EvidenceRef> {
    vouchers
        .iter()
        .map(|(g, v)| EvidenceRef::with_label("voucher", g, &voucher_label(v)))
        .collect()
}

/// Evidence refs across every voucher touched by any row in `rows`, deduplicated by guid.
fn evidence_for_rows(rows: &RowMap<'_>) -> Vec<EvidenceRef> {
    let mut all: BTreeMap<String, &Voucher> = BTreeMap::new();
    for agg in rows.values() {
        for (g, v) in &agg.vouchers {
            all.insert(g.clone(), v);
        }
    }
    evidence_for_vouchers(&all)
}

/// (in_scope_rows, excluded_rows) keyed by (date, payee ledger name); `excluded_rows` has one
/// entry per role in `excluded_roles`, even a role with no rows.
fn compute_40a3_rows<'a>(
    pop: &[&'a Voucher],
    book: &Book,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
    excluded_roles: &[String],
) -> Result<(RowMap<'a>, BTreeMap<String, RowMap<'a>>)> {
    let mut in_scope: RowMap = BTreeMap::new();
    let mut excluded: BTreeMap<String, RowMap> = excluded_roles
        .iter()
        .map(|k| (k.clone(), BTreeMap::new()))
        .collect();
    for &v in pop {
        if v.base_type == "Contra" {
            continue;
        }
        let cash_paid_out = v
            .lines
            .iter()
            .any(|l| cash.contains(&l.ledger) && l.amount_paise < 0);
        if !cash_paid_out {
            continue; // no cash line credited (cash paid out) on this voucher
        }
        for l in &v.lines {
            if cash.contains(&l.ledger) || bank.contains(&l.ledger) || l.amount_paise <= 0 {
                continue; // only non-cash, non-bank DEBIT lines are payee candidates
            }
            let kind = classify_kind(book.ledgers.get(&l.ledger));
            let key = (v.date.clone(), l.ledger.clone());
            let bucket: &mut RowMap = match excluded.get_mut(kind) {
                Some(bucket) => bucket,
                None => &mut in_scope,
            };
            let row = bucket.entry(key).or_insert_with(RowAgg::new);
            row.add(l.amount_paise, v)?;
            row.names.note(v, [cash, bank]);
        }
    }
    Ok((in_scope, excluded))
}

/// A voucher whose cash leg names no party, pooled by the day with every other such voucher,
/// whatever name each prints: the row shows every usable printed name and asserts no person.
fn pool_unidentified<'a>(
    rows: &mut RowMap<'a>,
    v: &'a Voucher,
    total: i64,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
) -> Result<()> {
    let row = rows
        .entry((v.date.clone(), UNIDENTIFIED_PARTY.to_string()))
        .or_insert_with(RowAgg::new);
    row.add(total, v)?;
    row.names.note(v, [cash, bank]);
    Ok(())
}

/// party_rows: (date, party ledger name) -> aggregate, for cash RECEIVED from a party.
pub(crate) fn compute_269st_rows<'a>(
    pop: &[&'a Voucher],
    book: &Book,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
    round_off_ledgers: &BTreeSet<String>,
) -> Result<RowMap<'a>> {
    let mut party_rows: RowMap = BTreeMap::new();
    for &v in pop {
        if v.base_type == "Contra" {
            continue;
        }
        let cash_received = v
            .lines
            .iter()
            .any(|l| cash.contains(&l.ledger) && l.amount_paise > 0);
        if !cash_received {
            continue; // no cash line debited (cash received) on this voucher
        }
        let mut party_amounts: BTreeMap<String, i64> = BTreeMap::new();
        let mut fallback_total: i64 = 0;
        for l in &v.lines {
            if cash.contains(&l.ledger) || bank.contains(&l.ledger) || l.amount_paise >= 0 {
                continue; // only non-cash, non-bank CREDIT lines are party candidates
            }
            let amt = -l.amount_paise;
            let ledger = book.ledgers.get(&l.ledger);
            let is_sales = ledger.is_some_and(|l2| l2.under(SALES_ACCOUNTS_GROUP));
            let is_tax_or_roundoff = ledger.is_some_and(|l2| l2.under(DUTIES_TAXES_GROUP))
                || round_off_ledgers.contains(&l.ledger);
            fallback_total = fallback_total.checked_add(amt).ok_or_else(overflow)?;
            if is_sales || is_tax_or_roundoff {
                continue; // never itself a party; its amount stays in fallback_total only
            }
            let entry = party_amounts.entry(l.ledger.clone()).or_insert(0);
            *entry = entry.checked_add(amt).ok_or_else(overflow)?;
        }
        if party_amounts.len() == 1 {
            let only = party_amounts.keys().next().unwrap().clone();
            party_amounts.insert(only, fallback_total);
        }
        if !party_amounts.is_empty() {
            for (ledger_name, amt) in party_amounts {
                let key = (v.date.clone(), ledger_name);
                party_rows
                    .entry(key)
                    .or_insert_with(RowAgg::new)
                    .add(amt, v)?;
            }
        } else if fallback_total > 0 {
            pool_unidentified(&mut party_rows, v, fallback_total, cash, bank)?;
        }
    }
    Ok(party_rows)
}

/// Mirrors `compute_269st_rows` for the PAYMENT leg: cash paid BY the assessee to a party.
fn compute_269st_payment_rows<'a>(
    pop: &[&'a Voucher],
    book: &Book,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
    round_off_ledgers: &BTreeSet<String>,
) -> Result<RowMap<'a>> {
    let mut party_rows: RowMap = BTreeMap::new();
    for &v in pop {
        if v.base_type == "Contra" {
            continue;
        }
        let cash_paid_out = v
            .lines
            .iter()
            .any(|l| cash.contains(&l.ledger) && l.amount_paise < 0);
        if !cash_paid_out {
            continue;
        }
        let mut party_amounts: BTreeMap<String, i64> = BTreeMap::new();
        let mut fallback_total: i64 = 0;
        for l in &v.lines {
            if cash.contains(&l.ledger) || bank.contains(&l.ledger) || l.amount_paise <= 0 {
                continue;
            }
            let amt = l.amount_paise;
            let ledger = book.ledgers.get(&l.ledger);
            let is_purchase = ledger.is_some_and(|l2| l2.under(PURCHASE_ACCOUNTS_GROUP));
            let is_tax_or_roundoff = ledger.is_some_and(|l2| l2.under(DUTIES_TAXES_GROUP))
                || round_off_ledgers.contains(&l.ledger);
            fallback_total = fallback_total.checked_add(amt).ok_or_else(overflow)?;
            if is_purchase || is_tax_or_roundoff {
                continue;
            }
            let entry = party_amounts.entry(l.ledger.clone()).or_insert(0);
            *entry = entry.checked_add(amt).ok_or_else(overflow)?;
        }
        if party_amounts.len() == 1 {
            let only = party_amounts.keys().next().unwrap().clone();
            party_amounts.insert(only, fallback_total);
        }
        if !party_amounts.is_empty() {
            for (ledger_name, amt) in party_amounts {
                let key = (v.date.clone(), ledger_name);
                party_rows
                    .entry(key)
                    .or_insert_with(RowAgg::new)
                    .add(amt, v)?;
            }
        } else if fallback_total > 0 {
            pool_unidentified(&mut party_rows, v, fallback_total, cash, bank)?;
        }
    }
    Ok(party_rows)
}

/// A loan ledger's sides on one voucher, in the order first met: ("cr", credits) or ("dr", debits).
type LoanSides = Vec<(&'static str, Vec<i64>)>;

/// One s.269SS/269T candidate: a voucher, one loan ledger on it and one side of that ledger
/// (bridge#775).
struct LoanCandidate<'a> {
    voucher: &'a Voucher,
    ledger: String,
    amount_paise: i64,
    direction: &'static str,
    /// How many lines of the ledger, on this side, were summed.
    lines: usize,
    /// Names the voucher: its GUID, or, where the GUID is blank or shared with another voucher of
    /// the population, the voucher's place in it.
    key: String,
    /// Names the side only where the voucher has both sides on the ledger.
    suffix: &'static str,
    /// Which way the voucher's cash moved.
    cash_direction: &'static str,
    /// The loan ledgers carrying the other side on the voucher, so a correction or a transfer
    /// between loans is shown on the row, never silently counted.
    opposite: Vec<String>,
}

/// One candidate per voucher, loan ledger and side. The ledger's credits on the voucher are
/// summed, and so are its debits, and the limit is tested on each sum: s.269SS/269T look at the
/// loan taken or repaid, and a receipt split over two lines of one ledger is one acceptance. Within
/// a loan ledger an acceptance and a repayment on one voucher are never netted; only the voucher's
/// cash lines are netted, to decide whether it moved cash at all. The direction is the side's own:
/// a credit to the loan is a loan accepted, a debit a loan repaid.
fn compute_269ss_269t_candidates<'a>(
    pop: &[&'a Voucher],
    book: &Book,
    cash: &BTreeSet<String>,
    limit_ss_t: i64,
) -> Result<Vec<LoanCandidate<'a>>> {
    let mut out = Vec::new();
    let mut guid_count: BTreeMap<&str, usize> = BTreeMap::new();
    for &v in pop {
        *guid_count.entry(v.guid.as_str()).or_insert(0) += 1;
    }
    for (index, &v) in pop.iter().enumerate() {
        if v.base_type == "Contra" {
            continue;
        }
        let mut cash_net: i64 = 0;
        let mut has_cash_line = false;
        for l in &v.lines {
            if cash.contains(&l.ledger) {
                has_cash_line = true;
                cash_net = cash_net.checked_add(l.amount_paise).ok_or_else(overflow)?;
            }
        }
        if !has_cash_line || cash_net == 0 {
            continue;
        }
        // Per ledger, in the order first met, its sides in the order first met: ("cr", credits)
        // or ("dr", debits), each line's amount as booked.
        let mut per_side: Vec<(String, LoanSides)> = Vec::new();
        for l in &v.lines {
            let is_loan = book
                .ledgers
                .get(&l.ledger)
                .is_some_and(|lg| lg.under(LOANS_LIABILITY_GROUP));
            if !is_loan || l.amount_paise == 0 {
                continue;
            }
            let side = if l.amount_paise < 0 { "cr" } else { "dr" };
            let slot = match per_side.iter().position(|(n, _)| *n == l.ledger) {
                Some(i) => i,
                None => {
                    per_side.push((l.ledger.clone(), Vec::new()));
                    per_side.len() - 1
                }
            };
            let sides = &mut per_side[slot].1;
            match sides.iter_mut().find(|(s, _)| *s == side) {
                Some((_, amounts)) => amounts.push(l.amount_paise),
                None => sides.push((side, vec![l.amount_paise])),
            }
        }
        let key = if !v.guid.is_empty() && guid_count[v.guid.as_str()] == 1 {
            v.guid.clone()
        } else {
            format!("{}#{index}", v.guid)
        };
        let cash_direction = if cash_net > 0 { "accepted" } else { "repaid" };
        for (ledger_name, sides) in &per_side {
            for (side, amounts) in sides {
                let sum = amounts
                    .iter()
                    .try_fold(0_i64, |t, a| t.checked_add(*a))
                    .ok_or_else(overflow)?;
                let amount = sum.checked_abs().ok_or_else(overflow)?;
                if amount < limit_ss_t {
                    continue;
                }
                let other = if *side == "cr" { "dr" } else { "cr" };
                let opposite: BTreeSet<&String> = per_side
                    .iter()
                    .filter(|(_, s)| s.iter().any(|(k, _)| *k == other))
                    .map(|(n, _)| n)
                    .collect();
                out.push(LoanCandidate {
                    voucher: v,
                    ledger: ledger_name.clone(),
                    amount_paise: amount,
                    direction: if *side == "cr" { "accepted" } else { "repaid" },
                    lines: amounts.len(),
                    key: key.clone(),
                    suffix: match (sides.len() == 2, *side) {
                        (true, "cr") => "_cr",
                        (true, _) => "_dr",
                        (false, _) => "",
                    },
                    cash_direction,
                    opposite: opposite.into_iter().cloned().collect(),
                });
            }
        }
    }
    Ok(out)
}

pub fn run(
    book: &Book,
    rules: &Rules,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
    loan_ledgers_configured: &BTreeSet<String>,
    round_off_ledgers: &BTreeSet<String>,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let pop = book.population()?;
    r.population_note = POPULATION.to_string();

    let limit_40a3 = rules.s40a3_limit_per_person_per_day_paise;
    let goods_limit = rules.s40a3_goods_carriage_limit_paise;
    let mut excluded_roles = rules.s40a3_excluded_group_roles.clone();
    excluded_roles.sort();

    let limit_269st = rules.s269st_limit_per_person_per_day_paise;
    let limit_ss_t = rules.s269ss_269t_limit_paise;

    // ---------------------------------------------------------------- (a) s.40A(3)
    let (in_scope_rows, excluded_rows) =
        compute_40a3_rows(&pop, book, cash, bank, &excluded_roles)?;
    let in_scope_count = in_scope_rows.len();
    let in_scope_total: i64 = in_scope_rows
        .values()
        .try_fold(0i64, |acc, d| acc.checked_add(d.paise).ok_or_else(overflow))?;
    let rows_over_limit: Vec<(&(TallyDate, String), &RowAgg)> = in_scope_rows
        .iter()
        .filter(|(_, d)| d.paise > limit_40a3)
        .collect();

    let in_scope_ev = evidence_for_rows(&in_scope_rows);
    r.fig(
        "s40a3_payee_days_any_amount_count",
        Value::Int(in_scope_count as i64),
        Unit::Count,
        "Distinct (date, payee ledger) pairs with a cash payment on a population, non-Contra \
voucher, payee classified as expenditure (not in a group the rules exclude from s.40A(3)).",
        Vec::new(),
    )?;
    r.fig(
        "s40a3_payee_days_any_amount_total",
        Value::Int(in_scope_total),
        Unit::Paise,
        "Sum of cash paid across every (date, payee ledger) pair with a cash payment on a \
population, non-Contra voucher, payee classified as expenditure, any amount.",
        in_scope_ev,
    )?;
    r.fig(
        "s40a3_over_limit_in_scope_count",
        Value::Int(rows_over_limit.len() as i64),
        Unit::Count,
        &format!(
            "In-scope (date, payee) pairs where the day's total exceeds the s.40A(3) limit \
({} per person per day).",
            rupees(i128::from(limit_40a3))
        ),
        Vec::new(),
    )?;

    let mut over_all_kinds = rows_over_limit.len();
    for kind in &excluded_roles {
        over_all_kinds += excluded_rows[kind]
            .values()
            .filter(|d| d.paise > limit_40a3)
            .count();
    }
    r.fig(
        "s40a3_over_limit_all_kinds_count",
        Value::Int(over_all_kinds as i64),
        Unit::Count,
        "(date, payee) pairs over the s.40A(3) limit before excluding capital, loans, fixed \
assets and duties and taxes; the in-scope count is a subset.",
        Vec::new(),
    )?;

    for kind in &excluded_roles {
        let rows = &excluded_rows[kind];
        let total: i64 = rows
            .values()
            .try_fold(0i64, |acc, d| acc.checked_add(d.paise).ok_or_else(overflow))?;
        let ev = evidence_for_rows(rows);
        let group = group_for_kind(kind)?;
        r.fig(
            &format!("s40a3_excluded_total_{kind}"),
            Value::Int(total),
            Unit::Paise,
            &format!(
                "Cash paid to ledgers under Tally group '{group}' (excluded from s.40A(3) \
scope by the rules); {} (date, payee) pairs.",
                rows.len()
            ),
            ev,
        )?;
    }

    // One Finding per in-scope payee-day that is actually over the limit.
    for ((d, ledger_name), data) in rows_over_limit {
        let h = stable_ledger_tag(book, ledger_name)?;
        let rid = format!("{}_{h}", iso(d));
        // A ledger that records purchases or expenses names what was bought, not whom it was paid
        // to: the row is keyed by it, loudly, and may pool several people's payments.
        let ledger = book.ledgers.get(ledger_name);
        let pooling = ledger.is_some_and(|lg| POOLING_LEDGER_GROUPS.iter().any(|g| lg.under(g)));
        let what = if ledger.is_some_and(|lg| lg.under(PURCHASE_ACCOUNTS_GROUP)) {
            "purchase"
        } else {
            "expense"
        };
        let amount_definition = if pooling {
            format!(
                "Debits to one {what} ledger (tag {h}) on {}, on population vouchers with a cash \
payment, summed: the debits, not the cash paid.",
                iso(d)
            )
        } else {
            format!(
                "Cash paid to one payee ledger (tag {h}) on {}, summed across every \
population voucher that day.",
                iso(d)
            )
        };
        let f_amt = r.fig(
            &format!("s40a3_row_amount_{rid}"),
            Value::Int(data.paise),
            Unit::Paise,
            &amount_definition,
            evidence_for_vouchers(&data.vouchers),
        )?;
        let goods_flag = data.paise <= goods_limit && transport_name_match(ledger_name);
        let f_goods = r.fig(
            &format!("s40a3_goods_carriage_candidate_{rid}"),
            Value::Text(if goods_flag { "yes" } else { "no" }.to_string()),
            Unit::Text,
            "Heuristic only: ledger name matches a generic transport-name pattern and the \
day's total is within the \u{20b9}35,000 goods-carriage limit (proviso to s.40A(3)).",
            Vec::new(),
        )?;
        let mut limits = vec![
            "Books only: Rule 6DD exceptions (e.g. bank/cooperative-bank closure, payments to \
government, payments where banking facilities are not available) are not visible from \
vouchers; ask the client whether any exception applies to this payment."
                .to_string(),
            "A single cash entry on this ledger and day may be a lump/year-end total covering \
several smaller payments to different people or on different days that the books do not \
itemise separately; confirm before treating it as one payee-day breach."
                .to_string(),
        ];
        if pooling {
            // Keyed by the debited ledger -- loud rather than split -- so a ledger that records
            // purchases or expenses may pool several people's payments; the names its vouchers
            // print let the CA split it.
            let printed = data.names.quoted();
            let names = if printed.is_empty() {
                "No usable name is printed on its vouchers.".to_string()
            } else if data.names.any_unprinted() {
                format!("Names printed on its vouchers: {printed}, and vouchers printing no usable name.")
            } else {
                format!("Names printed on its vouchers: {printed}.")
            };
            limits.push(format!(
                "This ledger records {what}s and may hold payments to several people on one day; \
the row does not show whether they went to one person. {names} (A printed name is usable when it \
is not blank, a cash or bank ledger, or one of the voucher's own ledgers.)"
            ));
        }
        if goods_flag {
            limits.push(
                "The possible goods-carriage flag is a heuristic match on the ledger name only \
(freight/transport/carrier/logistics keywords), not a finding of fact that the payee operates \
a goods carriage; confirm with the client before relying on the higher \u{20b9}35,000 limit."
                    .to_string(),
            );
        }
        let mut evidence = evidence_for_vouchers(&data.vouchers);
        evidence.push(EvidenceRef::new("ledger", ledger_name));
        r.findings.push(Finding {
            id: format!("{TEST_ID}/s40a3/{rid}"),
            clauses: vec!["s.40A(3)".to_string(), "3CD-21(d)".to_string()],
            title: if pooling {
                format!(
                    "Debits to one {what} ledger on vouchers with a cash payment total over the \
s.40A(3) daily limit on {}: the cash paid, and whether it went to one person, are not shown",
                    iso(d)
                )
            } else {
                format!(
                    "Cash paid to one payee over the s.40A(3) daily limit on {}",
                    iso(d)
                )
            },
            facts: vec![
                ("amount".to_string(), f_amt),
                ("goods_carriage_candidate".to_string(), f_goods),
            ],
            evidence,
            confidence: Confidence::NeedsDocument,
            limits,
            ask_client: vec![
                "Confirm whether any Rule 6DD exception applies to this payment.".to_string(),
                "Confirm whether this ledger/day is a single payee or a lump entry covering \
several payments."
                    .to_string(),
            ],
        });
    }

    // ---------------------------------------------------------------- (b) s.269ST limb (i)
    let party_rows = compute_269st_rows(&pop, book, cash, bank, round_off_ledgers)?;
    let party_total: i64 = party_rows
        .values()
        .try_fold(0i64, |acc, d| acc.checked_add(d.paise).ok_or_else(overflow))?;
    let rows_269st: Vec<(&(TallyDate, String), &RowAgg)> = party_rows
        .iter()
        .filter(|(_, d)| d.paise >= limit_269st)
        .collect();
    let party_ev = evidence_for_rows(&party_rows);
    r.fig(
        "s269st_party_days_any_amount_count",
        Value::Int(party_rows.len() as i64),
        Unit::Count,
        "Distinct (date, party ledger) pairs with a cash receipt on a population, non-Contra \
voucher, party ledger not under 'Sales Accounts'.",
        Vec::new(),
    )?;
    r.fig(
        "s269st_party_days_any_amount_total",
        Value::Int(party_total),
        Unit::Paise,
        "Sum of cash received across every (date, party ledger) pair with a cash receipt on a \
population, non-Contra voucher, party ledger not under 'Sales Accounts', any amount.",
        party_ev,
    )?;
    r.fig(
        "s269st_at_or_over_limit_count",
        Value::Int(rows_269st.len() as i64),
        Unit::Count,
        &format!(
            "(date, party) pairs where the day's total is at or over the s.269ST(a) limb (i) \
limit ({} per person per day).",
            rupees(i128::from(limit_269st))
        ),
        Vec::new(),
    )?;

    for ((d, ledger_name), data) in rows_269st {
        let h = stable_ledger_tag(book, ledger_name)?;
        let rid = format!("{}_{h}", iso(d));
        let unnamed = ledger_name == UNIDENTIFIED_PARTY;
        let f_amt = r.fig(
            &format!("s269st_row_amount_{rid}"),
            Value::Int(data.paise),
            Unit::Paise,
            &if unnamed {
                format!(
                    "Received on {} on cash-receipt vouchers with no party ledger (tag {h}), \
whatever name they print, summed: the payers' side of each voucher, not only its cash.",
                    iso(d)
                )
            } else {
                format!(
                    "Cash received from one party ledger (tag {h}) on {}, summed across every \
population voucher that day.",
                    iso(d)
                )
            },
            evidence_for_vouchers(&data.vouchers),
        )?;
        let mut evidence = evidence_for_vouchers(&data.vouchers);
        evidence.push(party_ref(ledger_name, data));
        r.findings.push(Finding {
            id: format!("{TEST_ID}/s269st/{rid}"),
            clauses: vec!["s.269ST(a)".to_string()],
            title: if unnamed {
                format!(
                    "Cash received on {} on vouchers that do not name the payer: the total is at or \
over the s.269ST(a) limit; whether any one person paid that much is not known",
                    iso(d)
                )
            } else {
                format!(
                    "Cash received from one party at or over the s.269ST(a) limb (i) limit on {}",
                    iso(d)
                )
            },
            facts: vec![("amount".to_string(), f_amt)],
            evidence,
            confidence: Confidence::NeedsDocument,
            limits: vec![
                "Books test covers limb (i) (aggregate per person per day) only. Limbs (ii) (a \
single transaction) and (iii) (receipts relating to one event or occasion from a person) need \
bill-wise/event linkage the books do not carry; a same-party pattern across several days that \
never reaches this limit on one day is not tested here."
                    .to_string(),
                "No Clause 31 tag here: this is the same (party, date) cash-mode receipt that the \
party-day scan of 'High-value transactions' already tags 3CD-31(ba) for every party, so this finding is an observation \
only, never counted a second time in the Clause 31 filing-aid total."
                    .to_string(),
            ],
            ask_client: vec![
                if unnamed {
                    "Who paid on each of these vouchers, and did any one person reach the s.269ST \
limit this day? Supply each payer's name and PAN."
                } else {
                    "Confirm whether this receipt is genuinely from one party."
                }
                .to_string(),
                "Confirm whether limbs (ii)/(iii) are separately breached (documents needed)."
                    .to_string(),
            ],
        });
    }

    // ---------------------------------------------------------------- (b2) s.269ST payment leg
    let party_pay_rows = compute_269st_payment_rows(&pop, book, cash, bank, round_off_ledgers)?;
    let pay_total: i64 = party_pay_rows
        .values()
        .try_fold(0i64, |acc, d| acc.checked_add(d.paise).ok_or_else(overflow))?;
    let rows_269st_pay: Vec<(&(TallyDate, String), &RowAgg)> = party_pay_rows
        .iter()
        .filter(|(_, d)| d.paise >= limit_269st)
        .collect();
    let pay_ev = evidence_for_rows(&party_pay_rows);
    r.fig(
        "s269st_payment_party_days_any_amount_count",
        Value::Int(party_pay_rows.len() as i64),
        Unit::Count,
        "Distinct (date, party ledger) pairs with a cash payment on a population, non-Contra \
voucher, party ledger not under 'Purchase Accounts'.",
        Vec::new(),
    )?;
    r.fig(
        "s269st_payment_party_days_any_amount_total",
        Value::Int(pay_total),
        Unit::Paise,
        "Sum of cash paid across every (date, party ledger) pair with a cash payment on a \
population, non-Contra voucher, party ledger not under 'Purchase Accounts', any amount.",
        pay_ev,
    )?;
    r.fig(
        "s269st_payment_at_or_over_limit_count",
        Value::Int(rows_269st_pay.len() as i64),
        Unit::Count,
        &format!(
            "(date, party) pairs where the day's cash payment total is at or over the \
s.269ST(a) limb (a) threshold ({limit_269st_text} per person per day) -- reportable in clause \
31(bc), not a contravention by the assessee (s.269ST binds the receiver, not the payer).",
            limit_269st_text = rupees(i128::from(limit_269st))
        ),
        Vec::new(),
    )?;

    for ((d, ledger_name), data) in rows_269st_pay {
        let h = stable_ledger_tag(book, ledger_name)?;
        let rid = format!("{}_{h}", iso(d));
        let unnamed = ledger_name == UNIDENTIFIED_PARTY;
        let f_amt = r.fig(
            &format!("s269st_payment_row_amount_{rid}"),
            Value::Int(data.paise),
            Unit::Paise,
            &if unnamed {
                format!(
                    "Paid on {} on cash-payment vouchers with no party ledger (tag {h}), whatever \
name they print, summed: the payees' side of each voucher, not only its cash.",
                    iso(d)
                )
            } else {
                format!(
                    "Cash paid to one party ledger (tag {h}) on {}, summed across every population \
voucher that day.",
                    iso(d)
                )
            },
            evidence_for_vouchers(&data.vouchers),
        )?;
        let mut evidence = evidence_for_vouchers(&data.vouchers);
        evidence.push(party_ref(ledger_name, data));
        r.findings.push(Finding {
            id: format!("{TEST_ID}/s269st_payment/{rid}"),
            clauses: vec!["s.269ST(a)".to_string()],
            title: if unnamed {
                format!(
                    "Cash paid on {} on vouchers that do not name the payee: the total is at or \
over the s.269ST limit; whether any one person was paid that much is not known",
                    iso(d)
                )
            } else {
                format!(
                    "Cash paid to one party at or over the s.269ST person-per-day threshold on {} \
-- reportable in clause 31(bc)",
                    iso(d)
                )
            },
            facts: vec![("amount".to_string(), f_amt)],
            evidence,
            confidence: Confidence::NeedsDocument,
            limits: vec![
                "s.269ST(a) is a duty on the receiver of the cash, not the payer: the section \
reads 'No person shall receive...'. A payment made by the assessee is not itself a \
books-testable exposure for the assessee under s.269ST; it is a Form 3CD clause 31(bc) \
reporting item only (limb (a): aggregate per person per day)."
                    .to_string(),
                "Books test covers limb (a) (aggregate per person per day) only. Limbs (b) (a \
single transaction) and (c) (receipts relating to one event or occasion) need bill-wise/event \
linkage the books do not carry."
                    .to_string(),
                "This payment is counted once for clause 31(bc), in the high-value transactions \
list; it is shown here for the s.40A(3) review only and is not counted a second time."
                    .to_string(),
            ],
            ask_client: if unnamed {
                vec![
                    "Who was paid on each of these vouchers, and was any one person paid at or over \
the s.269ST limit this day? Supply each payee's name and PAN."
                        .to_string(),
                    "Confirm whether limbs (b)/(c) apply (documents needed).".to_string(),
                ]
            } else {
                vec![
                    "Confirm whether this payment is genuinely to one party.".to_string(),
                    "Confirm whether limbs (b)/(c) apply from the party's own records (documents \
needed)."
                        .to_string(),
                ]
            },
        });
    }

    // ---------------------------------------------------------------- (c) s.269SS/269T
    let candidates = compute_269ss_269t_candidates(&pop, book, cash, limit_ss_t)?;
    let mut loan_total: i64 = 0;
    let mut uncovered_count: i64 = 0;
    for c in &candidates {
        let v = c.voucher;
        loan_total = loan_total
            .checked_add(c.amount_paise)
            .ok_or_else(overflow)?;
        let h = stable_ledger_tag(book, &c.ledger)?;
        // Unique by construction: the key names the voucher, the suffix the side where both exist.
        let rid = format!("{}_{h}{}", hash12_sha256(&c.key), c.suffix);
        // A blank GUID names nothing: the voucher's number does.
        let vref = if v.guid.is_empty() {
            format!(
                "number {}",
                if v.number.is_empty() {
                    "(none)"
                } else {
                    v.number.as_str()
                }
            )
        } else {
            crate::support::guid_tail12(&v.guid).to_string()
        };
        let clause = if c.direction == "accepted" {
            "s.269SS"
        } else {
            "s.269T"
        };
        let covered = loan_ledgers_configured.contains(&c.ledger);
        let coverage_tag = if covered { "covered" } else { "uncovered" };
        // The cash may have moved the other way from this loan side (a transfer between loans on a
        // voucher with a small cash leg); the text then says so instead of "cash <direction>".
        let with_cash = c.cash_direction == c.direction;
        let cash_words = if with_cash {
            format!("cash {} in the same voucher", c.direction)
        } else {
            "on a voucher whose cash moved the other way".to_string()
        };
        // An opposite entry on the voucher may be a correction: shown on the row, never hidden.
        let opposite = if c.opposite.is_empty() {
            String::new()
        } else {
            let names: Vec<String> = c.opposite.iter().map(|n| format!("'{n}'")).collect();
            format!(
                " This voucher also has an opposite entry on {}; it may be a correction; check \
before counting.",
                names.join(", ")
            )
        };
        let amount_definition = if c.lines == 1 {
            format!(
                "Loan-ledger line (tag {h}) on voucher {vref}, {cash_words} ({coverage_tag} by the \
client's list of loans).{opposite}"
            )
        } else {
            format!(
                "Loan-ledger lines (tag {h}) on voucher {vref}, {} of them on one side of the \
ledger, summed; {cash_words} ({coverage_tag} by the client's list of loans).{opposite}",
                c.lines
            )
        };
        let f_amt = r.fig(
            &format!("s269ss269t_amount_{coverage_tag}_{rid}"),
            Value::Int(c.amount_paise),
            Unit::Paise,
            &amount_definition,
            vec![EvidenceRef::with_label(
                "voucher",
                &v.guid,
                &voucher_label(v),
            )],
        )?;
        let common_limit = "Books only: confirm the counterparty is not government, a banking \
company, a co-operative bank, or another person/case excepted by s.269SS/269T, and that no \
other exception in the Act applies."
            .to_string();
        let (clauses, limits) = if covered {
            (vec![clause.to_string()], {
                let mut limits = vec![
                    common_limit,
                    "No Clause 31 tag here: this loan ledger is in the client's list of loans, \
so 'Loans and interest' already tags the matching entry 3CD-31(a)/3CD-31(c) -- this finding is an \
observation only, never counted a second time in the Clause 31 filing-aid total."
                        .to_string(),
                ];
                if !c.suffix.is_empty() {
                    limits.push(
                            "'Loans and interest' gives no clause 31 row for a voucher that both \
credits and debits this ledger: it lists it as the books hold it, without dividing it into entries \
or computing its reportability, where the lender is one clause 31 reports. This voucher has both an \
acceptance and a repayment on it."
                                .to_string(),
                        );
                }
                limits
            })
        } else {
            uncovered_count += 1;
            let clause_31 = if c.direction == "accepted" {
                "3CD-31(a)"
            } else {
                "3CD-31(c)"
            };
            (
                vec![clause.to_string(), clause_31.to_string()],
                vec![
                    common_limit,
                    "This loan ledger is NOT in the client's list of loans, so 'Loans and interest' \
never sees it -- this is the only place this entry is reported for Clause 31; add the ledger to \
that list to get the full lender-classification/running-balance test instead."
                        .to_string(),
                ],
            )
        };
        r.findings.push(Finding {
            id: format!("{TEST_ID}/s269ss269t/{rid}"),
            clauses,
            title: if with_cash {
                format!(
                    "Cash {} against a loan ledger on {}, at or over the s.269SS/269T limit",
                    c.direction,
                    iso(&v.date)
                )
            } else {
                format!(
                    "Loan {} against a loan ledger on {} on a voucher whose cash moved the other \
way, at or over the s.269SS/269T limit",
                    c.direction,
                    iso(&v.date)
                )
            },
            facts: vec![("amount".to_string(), f_amt)],
            evidence: vec![
                EvidenceRef::with_label("voucher", &v.guid, &voucher_label(v)),
                EvidenceRef::new("ledger", &c.ledger),
            ],
            confidence: Confidence::NeedsDocument,
            limits,
            ask_client: vec![
                "Confirm the lender/borrower's identity and relationship.".to_string(),
                "Confirm no s.269SS/269T exception applies.".to_string(),
            ],
        });
    }

    r.fig(
        "s269ss269t_candidate_uncovered_by_loans_interest_count",
        Value::Int(uncovered_count),
        Unit::Count,
        "s.269SS/269T candidates (population, non-Contra vouchers with a cash line, where one side of a \
Loans (Liability)-chain ledger -- its credits or its debits, summed -- is at or over the limit) on a \
loan ledger NOT in the client's list of loans -- the only ones this test tags for Clause 31; every \
other candidate is covered by 'Loans and interest' instead.",
        Vec::new(),
    )?;
    r.fig(
        "s269ss269t_candidate_count",
        Value::Int(candidates.len() as i64),
        Unit::Count,
        &format!(
            "(Voucher, loan ledger, side) candidates on population, non-Contra vouchers with a cash \
line, where one side of a Loans (Liability)-chain ledger on the voucher -- its credits (a loan \
accepted) or its debits (a loan repaid), summed -- is at or over the s.269SS/269T limit ({}).",
            rupees(i128::from(limit_ss_t))
        ),
        Vec::new(),
    )?;
    r.fig(
        "s269ss269t_candidate_total",
        Value::Int(loan_total),
        Unit::Paise,
        "Sum of the candidates' amounts: each (voucher, loan ledger, side) candidate's lines summed, \
taken without their sign, across the s.269SS/269T candidates. A voucher with entries on both sides \
of its loan ledgers (a correction, or a loan taken and repaid) may be counted in both directions.",
        Vec::new(),
    )?;

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pattern searched here is the reference's own, byte for byte (the probe file records it
    /// from the reference module).
    #[test]
    fn the_pattern_is_the_reference_s() {
        let v = crate::support::text_probe_tests::probes();
        assert_eq!(
            v["header"]["reference_regexes"]["transport"]
                .as_str()
                .unwrap(),
            TRANSPORT_NAME_RE
        );
    }

    /// The reference's transport-name regex is case-insensitive (`re.I`).
    #[test]
    fn the_transport_name_check_ignores_case() {
        for name in [
            "ABC LOGISTICS",
            "abc logistics",
            "Invented Road Lines",
            "invented roadlines",
            "Cargo Co",
        ] {
            assert!(transport_name_match(name), "{name}");
        }
        assert!(!transport_name_match("Invented Traders"));
    }

    /// The s.269SS/269T figure definition names the voucher by `guid[-12:]` too, in the reference.
    #[test]
    fn a_loan_line_definition_names_the_voucher_by_the_last_12_characters_of_its_guid() {
        use crate::book::{LedgerLine, VoucherStatus};
        let ledger = |name: &str, chain: &[&str]| crate::book::Ledger {
            name: name.to_string(),
            parent: chain[0].to_string(),
            chain: chain.iter().map(|g| (*g).to_string()).collect(),
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: format!("invented-{name}"),
            masterid: None,
        };
        let book = Book {
            company_name: "Invented".to_string(),
            company_guid: "invented-company".to_string(),
            read_at: String::new(),
            groups: BTreeMap::new(),
            group_masters: BTreeMap::new(),
            ledgers: [
                ledger("Cash", &["Cash-in-Hand"]),
                ledger("Lender Loan", &["Loans (Liability)"]),
            ]
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect(),
            vouchers: vec![Voucher {
                guid: "invented-guid-ééééééa".to_string(),
                date: TallyDate::parse("20250601").unwrap(),
                vtype: "Receipt".to_string(),
                base_type: "Receipt".to_string(),
                number: "R/1".to_string(),
                status: VoucherStatus::Regular,
                lines: vec![
                    LedgerLine {
                        ledger: "Cash".to_string(),
                        amount_paise: 3_000_000,
                    },
                    LedgerLine {
                        ledger: "Lender Loan".to_string(),
                        amount_paise: -3_000_000,
                    },
                ],
                narration: String::new(),
                ..Default::default()
            }],
            tb: BTreeMap::new(),
            ..Default::default()
        };
        let cash: BTreeSet<String> = ["Cash".to_string()].into_iter().collect();
        let r = run(
            &book,
            &Rules::vendored().unwrap(),
            &cash,
            &BTreeSet::new(),
            &BTreeSet::new(),
            &BTreeSet::new(),
        )
        .unwrap();
        let def = r
            .figures
            .iter()
            .find(|f| f.id.contains("s269ss269t_amount_uncovered_"))
            .map(|f| f.definition.clone())
            .unwrap();
        assert!(
            def.contains("on voucher guid-ééééééa, cash accepted"),
            "{def}"
        );
    }

    /// The reference labels a voucher with no number by `guid[-12:]`: 12 characters. Here the
    /// 12-byte cut would land inside an 'é' (a panic when byte-sliced); the expected tail is the
    /// reference's own `"invented-guid-ééééééa"[-12:]`.
    #[test]
    fn a_voucher_without_a_number_is_labelled_by_the_last_12_characters_of_its_guid() {
        let v = Voucher {
            guid: "invented-guid-ééééééa".to_string(),
            date: TallyDate::parse("20250601").unwrap(),
            vtype: "Payment".to_string(),
            base_type: "Payment".to_string(),
            number: String::new(),
            status: crate::book::VoucherStatus::Regular,
            lines: Vec::new(),
            narration: String::new(),
            ..Default::default()
        };
        assert_eq!(voucher_label(&v), "Payment guid-ééééééa on 2025-06-01");
    }
}
