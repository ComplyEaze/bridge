//! s.40A(3) on bank payments made by an instrument that is not an account-payee cheque, draft or
//! electronic transfer: a bearer or self cheque encashed across the bank counter by the payee,
//! or cash drawn by card and spent, where the voucher is booked straight to an expense. A port of
//! the reference Python implementation's `counter_cheques_40a3` test module, version 1.
//!
//! `cash_payments_40a3` tests only vouchers that credit a CASH ledger. A bearer cheque paid
//! through the bank credits the BANK ledger, so a wage paid that way never reaches any s.40A(3)
//! test there. Reported in Form 3CD clause 21(d)(A).
//!
//! What is not decided here:
//!   * Which bank narrations mean "encashed across the counter / not account payee" is client
//!     data: the terms arrive as `[roles].counter_cheque_narration_terms`. No term is hardcoded.
//!   * The payee is named only in the bank narration, which this module does not parse, so rows
//!     are per voucher line. A single instrument over the limit is itself over the limit, so a
//!     per-line row never overstates a breach; it can understate one, where two sub-limit
//!     instruments to one payee on one day together exceed it. Every finding says so.
//!   * A voucher that also credits a cash ledger belongs to `cash_payments_40a3` and is excluded
//!     here, so no payment is counted by both tests (CCQ-3).
//!   * Whether the instrument really was a bearer cheque needs the cheque or the bank's paid-
//!     cheque record: confidence is `NeedsDocument`, never computed.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::{Book, Voucher};
use crate::cash_payments_40a3::{classify_kind, group_for_kind};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::ledger_ids::stable_ledger_tag;
use crate::read::iso;
use crate::rules::Rules;
use crate::support::{self, hash8, py_upper, voucher_label};

pub const TEST_ID: &str = "counter_cheques_40a3";
pub const VERSION: &str = "1";

/// `[roles].counter_cheque_narration_terms` as this test reads it: absent is no terms; otherwise a
/// list of strings, or a configuration error for this test alone.
///
/// Divergence, deliberate, and not parity: the reference passes the value to `frozenset(...)`
/// unchecked, so a single string becomes the set of its characters and a table the set of its
/// keys, and a list holding a non-string raises inside the test. Here all three are refused, and
/// only this test fails (the same rule as `cash_book_integrity::own_account_terms`). A repeated
/// term collapses, as a Python set does.
pub fn narration_terms(raw: Option<&toml::Value>) -> Result<BTreeSet<String>> {
    let Some(raw) = raw else {
        return Ok(BTreeSet::new());
    };
    let key = "[roles].counter_cheque_narration_terms";
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

/// One expenditure debit line on a matched bank payment.
struct Row<'a> {
    voucher: &'a Voucher,
    ledger: &'a str,
    paise: i64,
    terms: Vec<&'a str>,
}

fn matched_terms<'a>(narration: &str, terms: &'a BTreeSet<String>) -> Vec<&'a str> {
    let text = py_upper(narration);
    // A BTreeSet iterates in code-point order, which is what Python's `sorted` of str gives.
    terms
        .iter()
        .filter(|t| !t.is_empty() && text.contains(py_upper(t).as_str()))
        .map(String::as_str)
        .collect()
}

/// `(in_scope, excluded)`: one in-scope row per expenditure debit line on a matched voucher, and
/// the out-of-scope debit lines' amounts per excluded role (one entry per role, even a role with
/// no lines).
fn compute_rows<'a>(
    pop: &[&'a Voucher],
    book: &Book,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
    terms: &'a BTreeSet<String>,
    excluded_roles: &[String],
) -> Result<(Vec<Row<'a>>, BTreeMap<String, i64>)> {
    let mut in_scope = Vec::new();
    let mut excluded: BTreeMap<String, i64> =
        excluded_roles.iter().map(|k| (k.clone(), 0)).collect();
    for &v in pop {
        if v.base_type == "Contra" {
            continue;
        }
        if !v
            .lines
            .iter()
            .any(|l| bank.contains(&l.ledger) && l.amount_paise < 0)
        {
            continue; // money did not leave a bank ledger on this voucher
        }
        if v.lines
            .iter()
            .any(|l| cash.contains(&l.ledger) && l.amount_paise < 0)
        {
            continue; // a cash leg: cash_payments_40a3 owns this voucher
        }
        let hit = matched_terms(&v.narration, terms);
        if hit.is_empty() {
            continue;
        }
        for l in &v.lines {
            if bank.contains(&l.ledger) || cash.contains(&l.ledger) || l.amount_paise <= 0 {
                continue;
            }
            let kind = classify_kind(book.ledgers.get(&l.ledger));
            if let Some(total) = excluded.get_mut(kind) {
                *total = total
                    .checked_add(l.amount_paise)
                    .ok_or_else(|| support::overflow(TEST_ID))?;
                continue;
            }
            in_scope.push(Row {
                voucher: v,
                ledger: &l.ledger,
                paise: l.amount_paise,
                terms: hit.clone(),
            });
        }
    }
    Ok((in_scope, excluded))
}

/// Voucher evidence for a set of rows: one ref per distinct GUID and label, as the reference's set
/// of refs is, so vouchers sharing a GUID (blank, or one GUID on different days) are each cited
/// (#1134).
fn ev(rows: &[&Row<'_>]) -> Vec<EvidenceRef> {
    let refs: BTreeSet<(&str, String)> = rows
        .iter()
        .map(|row| (row.voucher.guid.as_str(), voucher_label(row.voucher)))
        .collect();
    refs.into_iter()
        .map(|(g, label)| EvidenceRef::with_label("voucher", g, &label))
        .collect()
}

fn sum(rows: &[&Row<'_>]) -> Result<i64> {
    rows.iter().try_fold(0i64, |acc, r| {
        acc.checked_add(r.paise)
            .ok_or_else(|| support::overflow(TEST_ID))
    })
}

pub fn run(
    book: &Book,
    rules: &Rules,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
    terms: &BTreeSet<String>,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.population_note = "Books population (optional, cancelled and post-dated vouchers excluded); \
bank payments whose narration carries a configured counter-encashment term; Contra and any voucher \
with a cash leg excluded."
        .to_string();
    let limit = rules.s40a3_limit_per_person_per_day_paise;
    // The canonical dump sorts figures and findings by id, so no emission order is kept here.
    let excluded_roles = &rules.s40a3_excluded_group_roles;

    r.fig(
        "configured_terms_count",
        support::count(TEST_ID, terms.len())?,
        Unit::Count,
        "Bank-narration terms the client configuration identifies as counter-encashed or \
non-account-payee payments (roles.counter_cheque_narration_terms). Zero means this test was not \
configured and found nothing because it looked at nothing.",
        Vec::new(),
    )?;

    let pop = book.population()?;
    let (in_scope, excluded) = compute_rows(&pop, book, cash, bank, terms, excluded_roles)?;
    let all: Vec<&Row> = in_scope.iter().collect();
    let over: Vec<&Row> = in_scope.iter().filter(|x| x.paise > limit).collect();
    let under: Vec<&Row> = in_scope.iter().filter(|x| x.paise <= limit).collect();

    r.fig(
        "matched_expenditure_count",
        support::count(TEST_ID, all.len())?,
        Unit::Count,
        "Expenditure debit lines on bank payments whose narration carries a configured term, any \
amount.",
        Vec::new(),
    )?;
    r.fig(
        "matched_expenditure_total",
        Value::Int(sum(&all)?),
        Unit::Paise,
        "Sum of those expenditure debit lines, any amount.",
        ev(&all),
    )?;
    r.fig(
        "over_limit_count",
        support::count(TEST_ID, over.len())?,
        Unit::Count,
        &format!(
            "Of those, lines where the single payment exceeds the s.40A(3) limit ({limit} paise)."
        ),
        Vec::new(),
    )?;
    r.fig(
        "over_limit_total",
        Value::Int(sum(&over)?),
        Unit::Paise,
        "Sum of the over-limit lines above.",
        ev(&over),
    )?;
    r.fig(
        "at_or_under_limit_count",
        support::count(TEST_ID, under.len())?,
        Unit::Count,
        "Matched lines at or under the limit. Not tested for same-payee same-day aggregation: the \
payee is named only in the bank narration, so two such payments to one person on one day are not \
joined.",
        Vec::new(),
    )?;
    r.fig(
        "at_or_under_limit_total",
        Value::Int(sum(&under)?),
        Unit::Paise,
        "Sum of the at-or-under-limit matched lines.",
        ev(&under),
    )?;
    for kind in excluded_roles {
        r.fig(
            &format!("excluded_total_{kind}"),
            Value::Int(excluded[kind]),
            Unit::Paise,
            &format!(
                "Matched bank payments debited to ledgers under Tally group '{}' (not expenditure \
for s.40A(3)).",
                group_for_kind(kind)?
            ),
            Vec::new(),
        )?;
    }

    // A row's id is its date and a hash of its GUID and ledger. Two over-limit lines can share all three
    // (two lines to one ledger on one voucher, or two blank-GUID payments on one day); each of those lines
    // takes its place among them, in this stable sort's order (population, then line, order on a tie), as a
    // suffix, so no id repeats and every other id is as before (#1195).
    let mut over = over;
    over.sort_by(|a, b| {
        (&a.voucher.date, &a.voucher.guid, a.ledger).cmp(&(
            &b.voucher.date,
            &b.voucher.guid,
            b.ledger,
        ))
    });
    let mut base_ids = Vec::with_capacity(over.len());
    for row in &over {
        let v = row.voucher;
        let tag = stable_ledger_tag(book, row.ledger)?;
        base_ids.push(format!(
            "{}_{}",
            iso(&v.date),
            hash8(&format!("{}|{tag}", v.guid))
        ));
    }
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for b in &base_ids {
        *seen.entry(b.as_str()).or_insert(0) += 1;
    }
    let repeated: BTreeSet<&str> = seen
        .iter()
        .filter(|(_, n)| **n > 1)
        .map(|(b, _)| *b)
        .collect();
    let mut place: BTreeMap<&str, usize> = BTreeMap::new();
    for (row, base) in over.iter().zip(&base_ids) {
        let v = row.voucher;
        let day = iso(&v.date);
        let rid = if repeated.contains(base.as_str()) {
            let k = place.entry(base.as_str()).or_insert(0);
            *k += 1;
            format!("{base}_{k}")
        } else {
            base.clone()
        };
        let f_amt = r.fig(
            &format!("row_amount_{rid}"),
            Value::Int(row.paise),
            Unit::Paise,
            &format!(
                "One bank payment on {day} debited to an expense ledger, narration carrying a \
configured counter-encashment term."
            ),
            ev(&[row]),
        )?;
        let f_terms = r.fig(
            &format!("row_terms_{rid}"),
            Value::Text(row.terms.join(", ")),
            Unit::Text,
            "The configured narration term(s) this payment's bank narration carries.",
            Vec::new(),
        )?;
        r.findings.push(Finding {
            id: format!("{TEST_ID}/s40a3/{rid}"),
            clauses: vec!["s.40A(3)".to_string(), "3CD-21(d)".to_string()],
            title: format!(
                "Bank payment by a non-account-payee instrument over the s.40A(3) limit on {day}"
            ),
            facts: vec![
                ("amount".to_string(), f_amt),
                ("narration_terms".to_string(), f_terms),
            ],
            evidence: vec![
                EvidenceRef::with_label("voucher", &v.guid, &voucher_label(v)),
                EvidenceRef::new("ledger", row.ledger),
            ],
            confidence: Confidence::NeedsDocument,
            limits: vec![
                "The instrument is inferred from the bank narration only; the cheque itself or the \
bank's paid-cheque record shows whether it was crossed account payee."
                    .to_string(),
                "Rule 6DD exceptions are not visible from the books; wages have no general \
exception."
                    .to_string(),
                "Rows are per payment: sub-limit payments to the same person on the same day are \
not joined, so this list can understate, never overstate, the payments over the limit."
                    .to_string(),
            ],
            ask_client: vec![
                "Provide the cheque (or the bank's paid-cheque image) for this payment and name \
the payee."
                    .to_string(),
                "State whether any Rule 6DD exception applies, with the supporting record."
                    .to_string(),
            ],
        });
    }
    Ok(r)
}

/// CCQ-1 to CCQ-3, checked on the result alone, without calling [`compute_rows`]: the over-limit
/// total equals the sum of the row-amount figures, the over-limit count equals the number of
/// findings, and (when the `cash_payments_40a3` result is given) no voucher is evidence on a finding
/// of both tests. [`run`] always emits the total and the count as integers, so an absent or
/// non-integer total, count or row amount is itself a violation: a check that cannot see what it
/// checks does not report that it holds (#1121, as the reference does since its fix).
///
/// The reference's canonical dump calls this with the result alone, so CCQ-3 is exercised by its
/// pack and by tests here, not by the parity dump.
pub fn check_invariants(result: &TestResult, cash_result: Option<&TestResult>) -> Vec<String> {
    let mut out = Vec::new();
    let int_figure = |id: &str| {
        result
            .figures
            .iter()
            .find(|f| f.id == id)
            .and_then(|f| match &f.value {
                Value::Int(n) => Some(*n),
                _ => None,
            })
    };
    let amount_prefix = format!("{TEST_ID}.row_amount_");
    let rows: Vec<Option<i64>> = result
        .figures
        .iter()
        .filter(|f| f.id.starts_with(&amount_prefix))
        .map(|f| match &f.value {
            Value::Int(n) => Some(*n),
            _ => None,
        })
        .collect();
    let bad_rows = rows.iter().filter(|r| r.is_none()).count();
    if bad_rows > 0 {
        out.push(format!(
            "CCQ-1: {bad_rows} row_amount_* figure(s) not an integer"
        ));
    }
    match int_figure(&format!("{TEST_ID}.over_limit_total")) {
        None => out.push("CCQ-1: over_limit_total figure absent or not an integer".to_string()),
        Some(total) => {
            let sum: i128 = rows.iter().flatten().map(|n| i128::from(*n)).sum();
            if bad_rows == 0 && i128::from(total) != sum {
                out.push("CCQ-1: over_limit_total != sum of row_amount_* figures".to_string());
            }
        }
    }
    match int_figure(&format!("{TEST_ID}.over_limit_count")) {
        None => out.push("CCQ-2: over_limit_count figure absent or not an integer".to_string()),
        Some(count) => {
            if usize::try_from(count).ok() != Some(result.findings.len()) {
                out.push("CCQ-2: over_limit_count != number of findings".to_string());
            }
        }
    }
    if let Some(cash) = cash_result {
        let cash_vouchers: BTreeSet<&str> = cash
            .findings
            .iter()
            .filter(|f| f.id.starts_with("cash_payments_40a3/s40a3/"))
            .flat_map(|f| f.evidence.iter())
            .filter(|e| e.kind == "voucher")
            .map(|e| e.id.as_str())
            .collect();
        let mine: BTreeSet<&str> = result
            .findings
            .iter()
            .flat_map(|f| f.evidence.iter())
            .filter(|e| e.kind == "voucher")
            .map(|e| e.id.as_str())
            .collect();
        let both = cash_vouchers.intersection(&mine).count();
        if both > 0 {
            out.push(format!(
                "CCQ-3: {both} voucher(s) counted by both s.40A(3) tests"
            ));
        }
    }
    out
}
