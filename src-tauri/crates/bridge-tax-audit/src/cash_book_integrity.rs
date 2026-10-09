//! Cash-book integrity: whether the books' cash and opening position can be what they say. A
//! port of the reference Python implementation's `cash_book_integrity` test module, version 1.
//!
//! Five structural facts, each a question for the CA, never a conclusion:
//!   1. negative cash, walked day by day from each cash ledger's TB opening, as a best case
//!      (same-day receipts first) and a worst case (payments first);
//!   2. opening balances that do not sum to zero;
//!   3. Contra entries whose narration names the assessee's own account, booked as cash;
//!   4. money received into cash or bank credited to an expense ledger;
//!   5. cash paid through Journal entries to an expense ledger, with repeated and missing
//!      narrations among them.

use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_primitives::TallyDate;

use crate::book::{Book, OpeningStock, Voucher};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::ledger_ids::stable_ledger_tag;
use crate::read::iso;
use crate::rules::Rules;
use crate::support;
use crate::xml::{is_py_space, py_strip};

pub const TEST_ID: &str = "cash_book_integrity";
pub const VERSION: &str = "1";

/// Tally's reserved group names, not client data.
pub(crate) const EXPENSE_GROUPS: [&str; 2] = ["Direct Expenses", "Indirect Expenses"];

fn overflow() -> AuditError {
    support::overflow(TEST_ID)
}

/// The reference's `_ev`: one reference per distinct voucher, ordered by id.
fn ev<'a>(vouchers: impl IntoIterator<Item = &'a Voucher>) -> Vec<EvidenceRef> {
    let refs: BTreeSet<(String, String)> = vouchers
        .into_iter()
        .map(|v| (v.guid.clone(), support::voucher_label(v)))
        .collect();
    refs.into_iter()
        .map(|(id, l)| EvidenceRef::with_label("voucher", &id, &l))
        .collect()
}

fn count(n: usize) -> Result<Value> {
    support::count(TEST_ID, n)
}

fn add(a: i64, b: i64) -> Result<i64> {
    a.checked_add(b).ok_or_else(overflow)
}

fn total<'a>(rows: impl IntoIterator<Item = &'a (&'a Voucher, i64)>) -> Result<i64> {
    rows.into_iter().try_fold(0i64, |acc, (_, a)| add(acc, *a))
}

/// Python 3.13's `str.upper()`, pinned to the reference's Unicode 15.1 by `support::py_upper`:
/// Rust 1.96's own tables (Unicode 17.0) upper-case 55 code points differently, and a narration
/// holding one would otherwise change parts 3 and 5 (edge book `cash_book_unicode`).
fn upper(text: &str) -> String {
    crate::support::py_upper(text)
}

/// Python's `" ".join(text.split()).upper()`: whitespace runs collapsed, then upper-cased.
fn narration_key(text: &str) -> String {
    upper(
        &text
            .split(is_py_space)
            .filter(|w| !w.is_empty())
            .collect::<Vec<_>>()
            .join(" "),
    )
}

struct Day<'a> {
    date: TallyDate,
    close: i64,
    worst: i64,
    vouchers: Vec<&'a Voucher>,
}

/// One cash ledger walked day by day: each day's best-case closing (ordering cannot change it)
/// and worst-case low (opening minus the day's payments), in date order.
fn daily_walk<'a>(pop: &[&'a Voucher], ledger: &str, opening: i64) -> Result<Vec<Day<'a>>> {
    let mut days: BTreeMap<TallyDate, (i64, i64, Vec<&'a Voucher>)> = BTreeMap::new();
    for v in pop {
        let amt = v
            .lines
            .iter()
            .filter(|l| l.ledger == ledger)
            .try_fold(0i64, |acc, l| add(acc, l.amount_paise))?;
        if amt == 0 {
            continue;
        }
        let d = days.entry(v.date.clone()).or_insert((0, 0, Vec::new()));
        if amt > 0 {
            d.0 = add(d.0, amt)?;
        } else {
            d.1 = add(d.1, amt.checked_neg().ok_or_else(overflow)?)?;
        }
        d.2.push(v);
    }
    let mut out = Vec::new();
    let mut bal = opening;
    for (date, (inflow, outflow, vouchers)) in days {
        let worst = bal.checked_sub(outflow).ok_or_else(overflow)?;
        bal = add(bal, inflow)?
            .checked_sub(outflow)
            .ok_or_else(overflow)?;
        out.push(Day {
            date,
            close: bal,
            worst,
            vouchers,
        });
    }
    Ok(out)
}

fn under_expense(book: &Book, ledger: &str) -> bool {
    book.ledgers
        .get(ledger)
        .is_some_and(|l| EXPENSE_GROUPS.iter().any(|g| l.under(g)))
}

/// `[roles].own_account_narration_terms` as this test reads it: absent is no terms; otherwise a
/// list of strings, or a configuration error for this test alone.
///
/// Divergence, deliberate, and not parity: the reference passes the value to `frozenset(...)`
/// unchecked. A single string (`"SELF"` rather than `["SELF"]`) becomes the set of its characters,
/// so part 3 matches any narration containing any one of them; a table becomes the set of its
/// keys; and a list holding a non-string raises inside the test's `run`, which -- because the
/// reference's pack builds every result in one expression -- fails the whole pack, not this test.
/// Here all three are refused, and only this test fails.
pub fn own_account_terms(raw: Option<&toml::Value>) -> Result<Vec<String>> {
    let Some(raw) = raw else {
        return Ok(Vec::new());
    };
    let key = "[roles].own_account_narration_terms";
    raw.as_array()
        .ok_or_else(|| AuditError::Config(format!("{TEST_ID}: {key} is not a list")))?
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_string)
                .ok_or_else(|| AuditError::Config(format!("{TEST_ID}: {key} holds a non-string")))
        })
        .collect()
}

#[allow(clippy::too_many_lines)] // one section per fact, as the reference lays them out
pub fn run(
    book: &Book,
    rules: &Rules,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
    own_account_terms: &[String],
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let pop = book.population()?;
    r.population_note =
        "Books population (optional, cancelled and post-dated vouchers excluded).".to_string();

    // 1. negative cash
    for led in cash {
        let opening = book.tb.get(led).map_or(0, |t| t.opening_paise);
        let walk = daily_walk(&pop, led, opening)?;
        let best_neg: Vec<&Day> = walk.iter().filter(|d| d.close < 0).collect();
        let worst_neg = walk.iter().filter(|d| d.worst < 0).count();
        let h = stable_ledger_tag(book, led)?;
        let f_best = r.fig(
            &format!("negative_days_best_case_{h}"),
            count(best_neg.len())?,
            Unit::Count,
            &format!(
                "Days on which cash ledger (tag {h}) closes below zero even if every same-day \
receipt came before every same-day payment."
            ),
            Vec::new(),
        )?;
        let f_worst = r.fig(
            &format!("negative_days_worst_case_{h}"),
            count(worst_neg)?,
            Unit::Count,
            &format!(
                "Days on which cash ledger (tag {h}) is below zero if same-day payments came first."
            ),
            Vec::new(),
        )?;
        // The first day at the lowest closing, as Python's min() returns the first minimum.
        let lowest = best_neg.iter().fold(None::<&&Day>, |acc, d| match acc {
            Some(a) if a.close <= d.close => Some(a),
            _ => Some(d),
        });
        let best_vouchers: Vec<&Voucher> = best_neg
            .iter()
            .flat_map(|d| d.vouchers.iter().copied())
            .collect();
        let f_low = r.fig(
            &format!("lowest_best_case_balance_{h}"),
            Value::Int(lowest.map_or(0, |d| d.close)),
            Unit::Paise,
            &format!(
                "Lowest best-case daily closing of cash ledger (tag {h}); zero when it never goes \
below zero."
            ),
            ev(best_vouchers.iter().copied()),
        )?;
        let f_low_day = r.fig(
            &format!("lowest_best_case_date_{h}"),
            Value::Text(lowest.map_or(String::new(), |d| iso(&d.date))),
            Unit::Text,
            "Date of that lowest best-case closing (blank when none).",
            Vec::new(),
        )?;
        if !best_neg.is_empty() {
            let mut evidence = ev(best_vouchers.iter().copied());
            evidence.push(EvidenceRef::new("ledger", led));
            r.findings.push(Finding {
                id: format!("{TEST_ID}/negative_cash/{h}"),
                clauses: Vec::new(),
                title: "Cash in hand as booked falls below zero".to_string(),
                facts: vec![
                    ("days_best_case".to_string(), f_best),
                    ("days_worst_case".to_string(), f_worst),
                    ("lowest".to_string(), f_low),
                    ("lowest_date".to_string(), f_low_day),
                ],
                evidence,
                confidence: Confidence::Computed,
                limits: vec![
                    "Cash cannot be spent before it is held; the entries on these days cannot all \
carry the dates on which the cash moved. Which entries are misdated is not visible from the books."
                        .to_string(),
                ],
                ask_client: vec![
                    "Provide the cash book or cash vouchers for the negative days, with the dates \
cash was actually paid."
                        .to_string(),
                ],
            });
        }
    }

    // 2. opening difference
    let diff = book
        .tb
        .values()
        .try_fold(0i64, |acc, t| add(acc, t.opening_paise))?;
    let f_diff = r.fig(
        "opening_difference",
        Value::Int(diff),
        Unit::Paise,
        "Sum of every ledger's Trial Balance opening balance (Dr positive). Tally shows a non-zero \
sum as 'Difference in opening balances'.",
        Vec::new(),
    )?;
    // Opening stock is held on the stock items, never on a ledger, so on a book with integrated
    // inventory the ledger sum also carries minus the opening stock (#1497, #1486; the date a
    // stock item's opening value stands at is not measured: TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_
    // AND_OPEN_QUESTIONS.md §12a.13). A known term is added and the remainder is the difference;
    // an unknown one is named, and nothing is added.
    let mut facts = vec![("difference".to_string(), f_diff.clone())];
    let mut limits = vec![
        "The books cannot show which opening is missing or wrong; the prior year's closing \
balance sheet settles it."
            .to_string(),
    ];
    let remainder = match book.opening_stock {
        OpeningStock::Valued(stock) if stock != 0 => {
            let f_stock = r.fig(
                "opening_stock",
                Value::Int(stock),
                Unit::Paise,
                "Opening stock: the stock items' opening values (Dr positive). Tally holds it on \
the stock items, not on any ledger, so the ledgers' Trial Balance openings leave it out.",
                Vec::new(),
            )?;
            let rest = add(diff, stock)?;
            let f_rest = r.fig(
                "opening_difference_after_stock",
                Value::Int(rest),
                Unit::Paise,
                "Sum of every ledger's Trial Balance opening balance with the opening stock added \
(Dr positive): the difference in opening balances once stock, which no ledger holds, is counted.",
                Vec::new(),
            )?;
            facts = vec![
                ("difference".to_string(), f_rest),
                ("ledger_sum".to_string(), f_diff),
                ("opening_stock".to_string(), f_stock),
            ];
            rest
        }
        OpeningStock::Unknown(why) => {
            limits.push(format!(
                "Opening stock is held on the stock items, not on a ledger, and was {}; part of \
this difference may be opening stock.",
                why.as_str()
            ));
            diff
        }
        OpeningStock::NotApplicable | OpeningStock::Valued(_) => diff,
    };
    if remainder != 0 {
        let openings: Vec<EvidenceRef> = book
            .tb
            .iter()
            .filter(|(_, t)| t.opening_paise != 0)
            .map(|(n, _)| EvidenceRef::new("ledger", n))
            .collect();
        r.findings.push(Finding {
            id: format!("{TEST_ID}/opening_difference/all"),
            clauses: Vec::new(),
            title: "Opening balances do not balance".to_string(),
            facts,
            evidence: openings,
            confidence: Confidence::Computed,
            limits,
            ask_client: vec![
                "Provide the balance sheet as at the start of the year (the prior year's closing)."
                    .to_string(),
            ],
        });
    }

    // 3. own-account transfers booked as cash
    let terms: BTreeSet<String> = own_account_terms
        .iter()
        .filter(|t| !t.is_empty())
        .map(|t| upper(t))
        .collect();
    r.fig(
        "own_account_terms_count",
        count(terms.len())?,
        Unit::Count,
        "Own-account narration terms set for this client. Zero means the own-account check looked at \
nothing.",
        Vec::new(),
    )?;
    let mut out_v: Vec<(&Voucher, i64)> = Vec::new();
    let mut in_v: Vec<(&Voucher, i64)> = Vec::new();
    if !terms.is_empty() {
        for v in &pop {
            if v.base_type != "Contra" {
                continue;
            }
            let text = upper(&v.narration);
            if !terms.iter().any(|t| text.contains(t.as_str())) {
                continue;
            }
            let c = v
                .lines
                .iter()
                .filter(|l| cash.contains(&l.ledger))
                .try_fold(0i64, |acc, l| add(acc, l.amount_paise))?;
            if c > 0 {
                out_v.push((v, c));
            } else if c < 0 {
                in_v.push((v, c.checked_neg().ok_or_else(overflow)?));
            }
        }
    }
    let f_out = r.fig(
        "own_account_booked_as_withdrawal_total",
        Value::Int(total(&out_v)?),
        Unit::Paise,
        "Contra entries debiting cash whose bank narration names the assessee's own account \
(money sent to another account, booked as cash in hand).",
        ev(out_v.iter().map(|(v, _)| *v)),
    )?;
    let f_out_n = r.fig(
        "own_account_booked_as_withdrawal_count",
        count(out_v.len())?,
        Unit::Count,
        "Number of Contra entries debiting cash whose bank narration names the assessee's own account \
(money sent to another account, booked as cash in hand).",
        Vec::new(),
    )?;
    let f_in = r.fig(
        "own_account_booked_as_deposit_total",
        Value::Int(total(&in_v)?),
        Unit::Paise,
        "Contra entries crediting cash whose bank narration names the assessee's own account \
(money received from another account, booked as cash deposited).",
        ev(in_v.iter().map(|(v, _)| *v)),
    )?;
    let f_in_n = r.fig(
        "own_account_booked_as_deposit_count",
        count(in_v.len())?,
        Unit::Count,
        "Number of Contra entries crediting cash whose bank narration names the assessee's own \
account (money received from another account, booked as cash deposited).",
        Vec::new(),
    )?;
    if !out_v.is_empty() || !in_v.is_empty() {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/own_account_as_cash/all"),
            clauses: Vec::new(),
            title: "Transfers with the assessee's own account booked as cash".to_string(),
            facts: vec![
                ("out_total".to_string(), f_out),
                ("out_count".to_string(), f_out_n),
                ("in_total".to_string(), f_in),
                ("in_count".to_string(), f_in_n),
            ],
            evidence: ev(out_v.iter().chain(in_v.iter()).map(|(v, _)| *v)),
            confidence: Confidence::JudgementRequired,
            limits: vec![
                "The narration shows the other account's holder, not what the money was then used \
for. Whether those funds paid business expenses or were drawings needs that account's statement."
                    .to_string(),
            ],
            ask_client: vec![
                "Provide the statement of the receiving account for the year and say what these \
transfers were used for."
                    .to_string(),
            ],
        });
    }

    // 4. receipts credited to expense ledgers
    let mut by_ledger: BTreeMap<&str, Vec<(&Voucher, i64)>> = BTreeMap::new();
    for v in &pop {
        if !v
            .lines
            .iter()
            .any(|l| (cash.contains(&l.ledger) || bank.contains(&l.ledger)) && l.amount_paise > 0)
        {
            continue;
        }
        for l in &v.lines {
            if l.amount_paise < 0 && under_expense(book, &l.ledger) {
                by_ledger
                    .entry(l.ledger.as_str())
                    .or_default()
                    .push((v, l.amount_paise.checked_neg().ok_or_else(overflow)?));
            }
        }
    }
    let mut all_credits: i64 = 0;
    for (name, rows) in &by_ledger {
        let h = stable_ledger_tag(book, name)?;
        let amt = total(rows)?;
        all_credits = add(all_credits, amt)?;
        let f_amt = r.fig(
            &format!("expense_credit_total_{h}"),
            Value::Int(amt),
            Unit::Paise,
            &format!("Money received into cash or bank credited to expense ledger (tag {h})."),
            ev(rows.iter().map(|(v, _)| *v)),
        )?;
        let f_n = r.fig(
            &format!("expense_credit_count_{h}"),
            count(rows.len())?,
            Unit::Count,
            &format!("Entries in that total (ledger tag {h})."),
            Vec::new(),
        )?;
        let mut evidence = ev(rows.iter().map(|(v, _)| *v));
        evidence.push(EvidenceRef::new("ledger", name));
        r.findings.push(Finding {
            id: format!("{TEST_ID}/expense_credits/{h}"),
            clauses: vec!["s.68".to_string()],
            title: "Money received booked as a reduction of an expense".to_string(),
            facts: vec![("amount".to_string(), f_amt), ("count".to_string(), f_n)],
            evidence,
            confidence: Confidence::JudgementRequired,
            limits: vec![
                "The books do not show why each sum was received: a refund of an advance reduces \
the expense; any other receipt does not, and an unexplained credit is a s.68 question."
                    .to_string(),
            ],
            ask_client: vec![
                "For each receipt, name the payer and state why the money was received."
                    .to_string(),
            ],
        });
    }
    r.fig(
        "expense_credit_total_all",
        Value::Int(all_credits),
        Unit::Paise,
        "Sum of money received credited to any expense ledger.",
        Vec::new(),
    )?;

    // 5. cash paid through journals, and repeated narrations among them
    let mut jv: BTreeMap<&str, Vec<(&Voucher, i64)>> = BTreeMap::new();
    for v in &pop {
        if v.base_type != "Journal"
            || !v
                .lines
                .iter()
                .any(|l| cash.contains(&l.ledger) && l.amount_paise < 0)
        {
            continue;
        }
        for l in &v.lines {
            if l.amount_paise > 0 && under_expense(book, &l.ledger) {
                jv.entry(l.ledger.as_str())
                    .or_default()
                    .push((v, l.amount_paise));
            }
        }
    }
    let mut all_journal: i64 = 0;
    for (name, rows) in &jv {
        let h = stable_ledger_tag(book, name)?;
        let amt = total(rows)?;
        all_journal = add(all_journal, amt)?;
        let mut by_text: BTreeMap<String, Vec<(&Voucher, i64)>> = BTreeMap::new();
        for (v, a) in rows {
            let key = narration_key(&v.narration);
            if !key.is_empty() {
                by_text.entry(key).or_default().push((v, *a));
            }
        }
        let repeated: Vec<(&Voucher, i64)> = by_text
            .values()
            .filter(|g| g.len() > 1)
            .flatten()
            .copied()
            .collect();
        let unnarrated: Vec<(&Voucher, i64)> = rows
            .iter()
            .filter(|(v, _)| py_strip(&v.narration).is_empty())
            .copied()
            .collect();
        let f_amt = r.fig(
            &format!("journal_cash_total_{h}"),
            Value::Int(amt),
            Unit::Paise,
            &format!("Cash paid through Journal entries to expense ledger (tag {h})."),
            ev(rows.iter().map(|(v, _)| *v)),
        )?;
        let f_n = r.fig(
            &format!("journal_cash_count_{h}"),
            count(rows.len())?,
            Unit::Count,
            &format!("Journal entries paying cash to expense ledger (tag {h})."),
            Vec::new(),
        )?;
        let f_rep = r.fig(
            &format!("journal_cash_repeated_narration_total_{h}"),
            Value::Int(total(&repeated)?),
            Unit::Paise,
            &format!(
                "Cash paid through Journal entries to expense ledger (tag {h}) whose narration text \
is identical to another such entry's."
            ),
            ev(repeated.iter().map(|(v, _)| *v)),
        )?;
        let f_rep_n = r.fig(
            &format!("journal_cash_repeated_narration_count_{h}"),
            count(repeated.len())?,
            Unit::Count,
            &format!(
                "Journal entries paying cash to expense ledger (tag {h}) whose narration text is \
identical to another such entry's."
            ),
            Vec::new(),
        )?;
        let f_un = r.fig(
            &format!("journal_cash_unnarrated_total_{h}"),
            Value::Int(total(&unnarrated)?),
            Unit::Paise,
            &format!(
                "Cash paid through Journal entries to expense ledger (tag {h}) that carry no \
narration at all."
            ),
            ev(unnarrated.iter().map(|(v, _)| *v)),
        )?;
        let f_un_n = r.fig(
            &format!("journal_cash_unnarrated_count_{h}"),
            count(unnarrated.len())?,
            Unit::Count,
            &format!(
                "Journal entries paying cash to expense ledger (tag {h}) that carry no narration at \
all."
            ),
            Vec::new(),
        )?;
        let mut evidence = ev(rows.iter().map(|(v, _)| *v));
        evidence.push(EvidenceRef::new("ledger", name));
        r.findings.push(Finding {
            id: format!("{TEST_ID}/journal_cash/{h}"),
            clauses: Vec::new(),
            title: "Cash paid through journal entries, without a payee".to_string(),
            facts: vec![
                ("amount".to_string(), f_amt),
                ("count".to_string(), f_n),
                ("repeated_amount".to_string(), f_rep),
                ("repeated_count".to_string(), f_rep_n),
                ("unnarrated_amount".to_string(), f_un),
                ("unnarrated_count".to_string(), f_un_n),
            ],
            evidence,
            confidence: Confidence::JudgementRequired,
            limits: vec![
                "A journal names no payee, so the per-person daily limit of s.40A(3) and the TDS \
thresholds cannot be tested from these entries; the payment records behind them decide both."
                    .to_string(),
                "Identical narration text on two entries is a question of duplication, not proof \
of it."
                    .to_string(),
            ],
            ask_client: vec![
                "Provide the wage register or payment records (payee, date, amount) behind each \
entry."
                    .to_string(),
                "Explain each pair of entries that carries the same narration.".to_string(),
            ],
        });
    }
    r.fig(
        "journal_cash_total_all",
        Value::Int(all_journal),
        Unit::Paise,
        "Cash paid through Journal entries to any expense ledger.",
        Vec::new(),
    )?;
    Ok(r)
}

/// CBI-1 and CBI-2, re-derived from the Trial Balance rather than this module's walk: the opening
/// difference figure equals the TB opening sum, and a cash ledger that closes negative in the TB
/// must report at least one negative best-case day.
pub fn check_invariants(book: &Book, result: &TestResult) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let tb_sum = book
        .tb
        .values()
        .try_fold(0i64, |acc, t| add(acc, t.opening_paise))?;
    let prefix = format!("{TEST_ID}.");
    if let Some(f) = result
        .figures
        .iter()
        .find(|f| f.id == format!("{prefix}opening_difference"))
    {
        if f.value != Value::Int(tb_sum) {
            out.push("CBI-1: opening_difference != Trial Balance opening sum".to_string());
        }
    }
    let marker = format!("{prefix}negative_days_best_case_");
    for f in result
        .figures
        .iter()
        .filter(|f| f.id.starts_with(marker.as_str()))
    {
        let h = f.id.rsplit('_').next().unwrap_or_default();
        let mut closing = None;
        for (n, t) in &book.tb {
            if stable_ledger_tag(book, n)? == h {
                closing = Some(t.closing_paise);
                break;
            }
        }
        if closing.is_some_and(|c| c < 0) && f.value == Value::Int(0) {
            out.push(format!(
                "CBI-2: cash ledger tag {h} closes negative in the TB but no negative day was found"
            ));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{OpeningStock, OpeningStockUnknown, TbRow};
    use crate::findings::Figure;

    const ORIGINAL_LIMIT: &str = "The books cannot show which opening is missing or wrong; the \
prior year's closing balance sheet settles it.";

    /// An invented book of two ledgers whose Trial Balance openings sum to `sum`, with `stock` as
    /// its opening stock and no vouchers.
    fn book(sum: i64, stock: OpeningStock) -> Book {
        let row = |opening_paise: i64| TbRow {
            opening_paise,
            debit_paise: 0,
            credit_paise: 0,
            closing_paise: opening_paise,
        };
        let mut b = Book::default();
        b.tb.insert("Owner Capital".to_string(), row(sum - 300_000));
        b.tb.insert("Shop Bank".to_string(), row(300_000));
        b.opening_stock = stock;
        b
    }

    fn run_on(b: &Book) -> TestResult {
        let none = BTreeSet::new();
        run(b, &Rules::vendored().unwrap(), &none, &none, &[]).unwrap()
    }

    /// Section 2's figures (every id under `opening_`), whole, in the order they are made.
    fn opening_figures(r: &TestResult) -> String {
        let figures: Vec<&Figure> = r
            .figures
            .iter()
            .filter(|f| f.id.starts_with("cash_book_integrity.opening_"))
            .collect();
        format!("{figures:?}")
    }

    /// Section 2's finding, whole, or "None".
    fn opening_finding(r: &TestResult) -> String {
        let finding = r
            .findings
            .iter()
            .find(|f| f.id == "cash_book_integrity/opening_difference/all");
        format!("{finding:?}")
    }

    fn figure(name: &str, value: i64, definition: &str) -> Figure {
        Figure {
            id: format!("cash_book_integrity.{name}"),
            value: Value::Int(value),
            unit: Unit::Paise,
            definition: definition.to_string(),
            evidence: Vec::new(),
        }
    }

    fn ledger_sum(value: i64) -> Figure {
        figure(
            "opening_difference",
            value,
            "Sum of every ledger's Trial Balance opening balance (Dr positive). Tally shows a \
non-zero sum as 'Difference in opening balances'.",
        )
    }

    fn stock_figures(sum: i64, stock: i64) -> String {
        format!(
            "{:?}",
            vec![
                &ledger_sum(sum),
                &figure(
                    "opening_stock",
                    stock,
                    "Opening stock: the stock items' opening values (Dr positive). Tally holds it \
on the stock items, not on any ledger, so the ledgers' Trial Balance openings leave it out.",
                ),
                &figure(
                    "opening_difference_after_stock",
                    sum + stock,
                    "Sum of every ledger's Trial Balance opening balance with the opening stock \
added (Dr positive): the difference in opening balances once stock, which no ledger holds, is \
counted.",
                ),
            ]
        )
    }

    fn finding(facts: &[(&str, &str)], limits: &[&str]) -> String {
        let f = Finding {
            id: "cash_book_integrity/opening_difference/all".to_string(),
            clauses: Vec::new(),
            title: "Opening balances do not balance".to_string(),
            facts: facts
                .iter()
                .map(|(n, f)| ((*n).to_string(), format!("cash_book_integrity.{f}")))
                .collect(),
            evidence: vec![
                EvidenceRef::new("ledger", "Owner Capital"),
                EvidenceRef::new("ledger", "Shop Bank"),
            ],
            confidence: Confidence::Computed,
            limits: limits.iter().map(|l| (*l).to_string()).collect(),
            ask_client: vec![
                "Provide the balance sheet as at the start of the year (the prior year's closing)."
                    .to_string(),
            ],
        };
        format!("{:?}", Some(&f))
    }

    const STOCK_FACTS: [(&str, &str); 3] = [
        ("difference", "opening_difference_after_stock"),
        ("ledger_sum", "opening_difference"),
        ("opening_stock", "opening_stock"),
    ];

    /// #1497: ledger openings that opening stock exactly offsets are not a difference.
    #[test]
    fn opening_stock_that_offsets_the_ledger_openings_raises_no_finding() {
        let r = run_on(&book(-800_000, OpeningStock::Valued(800_000)));
        assert_eq!(opening_figures(&r), stock_figures(-800_000, 800_000));
        assert_eq!(opening_finding(&r), "None");
    }

    /// What remains after the stock is the finding's difference; the ledger sum and the stock
    /// stand beside it, so the remainder is traceable.
    #[test]
    fn a_difference_left_after_opening_stock_is_the_findings_difference() {
        let r = run_on(&book(-1_000_000, OpeningStock::Valued(800_000)));
        assert_eq!(opening_figures(&r), stock_figures(-1_000_000, 800_000));
        assert_eq!(
            opening_finding(&r),
            finding(&STOCK_FACTS, &[ORIGINAL_LIMIT])
        );
    }

    /// Stock that more than offsets the ledgers leaves a debit difference, sign kept; ledgers
    /// that sum to zero beside opening stock report the whole stock.
    #[test]
    fn opening_stock_that_more_than_offsets_keeps_its_sign() {
        for (sum, stock) in [(-500_000, 800_000), (0, 800_000), (-800_001, 800_000)] {
            let r = run_on(&book(sum, OpeningStock::Valued(stock)));
            assert_eq!(opening_figures(&r), stock_figures(sum, stock));
            assert_eq!(
                opening_finding(&r),
                finding(&STOCK_FACTS, &[ORIGINAL_LIMIT])
            );
        }
    }

    /// An opening stock that could not be taken is named, with its reason, in the finding's
    /// limits, so the ledger sum is not read as a pure difference; nothing is added to it.
    #[test]
    fn an_unknown_opening_stock_is_named_in_the_findings_limits() {
        for (why, text) in [
            (OpeningStockUnknown::NotRead, "not read"),
            (
                OpeningStockUnknown::NotAtBooksStart,
                "not taken: the read does not show the trial balance starting on the books' \
first day",
            ),
            (OpeningStockUnknown::Unreadable, "unreadable"),
        ] {
            let r = run_on(&book(-1_000_000, OpeningStock::Unknown(why)));
            assert_eq!(
                opening_figures(&r),
                format!("{:?}", vec![&ledger_sum(-1_000_000)])
            );
            let limit = format!(
                "Opening stock is held on the stock items, not on a ledger, and was {text}; part \
of this difference may be opening stock."
            );
            assert_eq!(
                opening_finding(&r),
                finding(
                    &[("difference", "opening_difference")],
                    &[ORIGINAL_LIMIT, &limit]
                )
            );
            let r = run_on(&book(0, OpeningStock::Unknown(why)));
            assert_eq!(opening_finding(&r), "None");
        }
    }

    /// With no stock term the whole result is what it was before #1497: no new figure, the same
    /// finding.
    #[test]
    fn no_stock_term_gives_the_result_as_before() {
        let plain = run_on(&book(-1_000_000, OpeningStock::NotApplicable));
        let zero = run_on(&book(-1_000_000, OpeningStock::Valued(0)));
        assert_eq!(
            format!("{:?}", (&plain.figures, &plain.findings)),
            format!("{:?}", (&zero.figures, &zero.findings))
        );
        assert_eq!(
            opening_figures(&plain),
            format!("{:?}", vec![&ledger_sum(-1_000_000)])
        );
        assert_eq!(
            opening_finding(&plain),
            finding(&[("difference", "opening_difference")], &[ORIGINAL_LIMIT])
        );
        let balanced = run_on(&book(0, OpeningStock::NotApplicable));
        assert_eq!(opening_finding(&balanced), "None");
    }
}
