// SPDX-License-Identifier: Apache-2.0
//! Loans and deposits: s.194A interest TDS, Form 3CD Clause 31(a)/(c) (loans and deposits taken
//! and repaid in the year) and the s.269SS/269T candidates within them, with the interest on a
//! declared-shared interest ledger that no configured loan accounts for, and a notice for a loan
//! left out of the client's list. A line-for-line port of the reference at its current head; the
//! reference module's docstring is the design record, and only what differs here is stated:
//!
//! * Every amount is summed in i128 and refused if a figure does not fit i64, where the
//!   reference's integers are unbounded.
//! * A loan entry's `lender` and `lender_type` must be strings, and its `interest_ledger` a name
//!   or a list of non-empty names, each refused when the loans are read. The reference refuses a
//!   non-text `lender` (its per-lender check strips it) and a malformed `interest_ledger` too, but
//!   only when the test runs; it runs with a `lender_type` of any type, which is refused here.
//! * The s.194A coverage arithmetic (the rate times the interest, in paise x 10,000) is checked:
//!   a product past i128, which only an unvendored rate could reach, is refused.
//! * A figure id the reference would repeat (two vouchers with one GUID on one loan, in the same
//!   direction, say) is refused with an error, as the reference's `fig` raises, never a panic.
//! * Voucher identity (the reference's `id(v)`) is the voucher's position in the population.
//! * The shared-ledger rule is switched by `net_reversals`: [`run`] and [`check_invariants`] pass
//!   [`NET_REVERSALS`], and the tests reach the dormant reversal rule through [`run_with`] and
//!   [`check_invariants_with`] instead of rebinding a module global.
//! * Where the reference sorts evidence by id alone, refs sharing an id (a blank GUID) are ordered
//!   by label too; the canonical dump sorts evidence either way.
//!
//! A voucher that both credits and debits a loan ledger (#779 Phase A) is listed as the books hold
//! it and never netted into one row: from its date on, the loan's entries are tested on their own
//! amount only, its maximum outstanding is not computed, and its s.194A verdict is not computed
//! where the voucher's interest or TDS leaves it open.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use bridge_tally_primitives::TallyDate;
use sha1::{Digest, Sha1};

use crate::book::{Book, Voucher};
use crate::cash_book_integrity::EXPENSE_GROUPS;
use crate::depreciation::civil_day_number;
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::ledger_ids::stable_ledger_tag;
use crate::rules::Rules;
use crate::support::{
    count, ledgers_by_tag, overflow, py_casefold, py_ci_eq, py_is_decimal, py_lower, py_repr_str,
    py_split, py_strip, rupees, voucher_label,
};
use crate::tds_payees::{deductor_question, deductor_status, DeductorActivity};

pub const TEST_ID: &str = "loans_interest";
pub const VERSION: &str = "1";

const MODE_CASH: &str = "cash";
const MODE_BANK: &str = "bank";
const MODE_JOURNAL: &str = "journal";
const MODE_OTHER: &str = "other";
const NON_ACCOUNT_PAYEE_MODES: [&str; 3] = [MODE_CASH, MODE_JOURNAL, MODE_OTHER];

const DEFAULT_269_EXEMPT_LENDER_TYPES: [&str; 2] = ["bank", "cooperative_bank"];
/// Clause 31(c)-(e)'s note (repayments): Government is excluded too.
const DEFAULT_269_REPORTING_EXEMPT_LENDER_TYPES: [&str; 4] = [
    "government",
    "government_company",
    "bank",
    "statutory_corporation",
];
/// Clause 31(a)/(b)'s note (loans taken or accepted): not bare Government.
const DEFAULT_269_REPORTING_EXEMPT_ACCEPTED_LENDER_TYPES: [&str; 3] =
    ["government_company", "bank", "statutory_corporation"];

/// Stated on a taken row walked from a principal balance below zero. It names no balance: the
/// walk leaves out interest credited and TDS journals and floors a debit opening.
pub const WALKED_BELOW_ZERO_NOTE: &str = "Before this entry, the principal balance this test \
walks on this loan ledger was below zero (it walks principal only, from an opening floored at nil, \
and leaves out journals against ledgers configured as TDS payable), so it may differ from the \
ledger's balance. Part of this credit may be the lender returning an overpayment, or earlier \
repayments may have included interest: the books do not say which. It is read as a loan taken and \
tested on its own amount; the CA determines.";

/// The note the unattributed figure on a shared interest ledger carries while no credit reduces
/// it (owner decision, 2026-09-23).
pub const NEVER_NET_NOTE: &str = "No credit reduces it: every credit to this ledger on these \
vouchers (a reversal, interest received, interest charged on to a debtor, a rectification) is shown \
beside it as a separate figure, never netted.";

/// The earlier reversal rule's note (owner decision (b), 2026-09-22), kept with the rule.
pub const REVERSAL_RULE_NOTE: &str = "Only a credit that looks like the reversal of a specific \
earlier debit reduces it: a credit on another voucher, not a Receipt, of the same amount, with the \
same other ledgers, dated on or after the debit, each debit reversed at most once and matched \
earliest first. Every other credit (interest received on a Receipt, interest charged on to a \
debtor, an unmatched rectification) is shown beside it but not netted. The match is read from the \
amounts and ledgers, not from the vouchers' purpose: a credit that is not in fact a reversal but \
meets every condition (for example interest received booked as a Journal against the same bank) \
would reduce it.";

/// Whether a credit may reduce the unattributed figure at all. False by owner decision,
/// 2026-09-23 (never subtract); true restores the reversal rule.
pub const NET_REVERSALS: bool = false;

/// The note for the rule in force.
pub fn shared_netting_note(net_reversals: bool) -> &'static str {
    if net_reversals {
        REVERSAL_RULE_NOTE
    } else {
        NEVER_NET_NOTE
    }
}

/// The most days a bank credit may follow a bank debit and still be named as its possible return.
const REVERSAL_PAIR_MAX_DAYS: i64 = 7;

/// R2's texts, shared by the listed record and the flag-not-computed note.
const TWO_WALKS_TEXT: &str = "The principal balance this test walks on this loan ledger \
(principal only: interest credited and TDS journals are not walked) went below zero before this \
entry. That can be an overpayment the lender owes back, or earlier repayments that included \
interest: the books do not say which. So this test reads the principal credits after it two ways: \
from the opening as the books show it, netting them against that balance; or from an opening \
floored at nil, reading each as a fresh loan.";
const SAME_DAY_TEXT: &str = "Entries dated the same day are walked in voucher-id order, which the \
books do not record, so a balance below zero within a day may exist only in that order.";
const REFUND_OR_LOAN_ASK: &str = "How earlier repayments to this loan were applied (to principal \
or to interest), and whether each later credit was the lender returning an overpayment or a new \
loan, with the amounts.";

/// Tally's reserved group names, not client data.
const LOANS_GROUP: &str = "Loans (Liability)";
const BANK_OD_GROUP: &str = "Bank OD A/c";
const DUTIES_TAXES_GROUP: &str = "Duties & Taxes";
/// The only groups the repayment pattern scans: where a loan is found misbooked.
const PATTERN_GROUPS: [&str; 3] = ["Current Liabilities", "Sundry Creditors", "Suspense A/c"];
const PATTERN_MIN_MONTHS: usize = 3;
const NOTICE_ONLY_ADDS: &str = "This notice removes and changes no clause 31 row or amount. It \
adds a question for the CA under clause 31, and a draft that answers clause 31 'Nil' is asked about \
it. A notice that turns out not to be a loan costs a question, not a wrong figure.";

/// One `[loans.loan_ledgers]` entry, typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoanConfig {
    pub lender: String,
    pub lender_type: String,
    /// The loan's configured interest ledgers: empty for an interest-free loan (no
    /// `interest_ledger` key, or `""`), else one name or several (s.2(28A): a lender's charge
    /// booked to its own ledger, named by the CA).
    pub interest_ledgers: BTreeSet<String>,
}

/// Type the bound `[loans.loan_ledgers]` entries, as the reference's `run()` reads them and its
/// `interest_ledgers_of` parses each `interest_ledger`.
pub fn loan_config(
    entries: &BTreeMap<String, toml::Value>,
) -> Result<BTreeMap<String, LoanConfig>> {
    entries
        .iter()
        .map(|(ledger, entry)| {
            let t = entry.as_table().ok_or_else(|| {
                AuditError::Config(format!("[loans.loan_ledgers].{ledger:?} is not a table"))
            })?;
            let text = |key: &str| -> Result<String> {
                t.get(key)
                    .ok_or_else(|| {
                        AuditError::Config(format!("[loans.loan_ledgers].{ledger:?} has no {key}"))
                    })?
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| {
                        AuditError::Config(format!(
                            "[loans.loan_ledgers].{ledger:?}.{key} is not a string"
                        ))
                    })
            };
            let malformed = |v: &toml::Value| {
                AuditError::Config(format!(
                    "[loans.loan_ledgers].{ledger:?}.interest_ledger must be a ledger name or a \
list of ledger names, got {v}"
                ))
            };
            let interest_ledgers = match t.get("interest_ledger") {
                None => BTreeSet::new(),
                Some(toml::Value::String(s)) if s.is_empty() => BTreeSet::new(),
                Some(toml::Value::String(s)) => [s.clone()].into_iter().collect(),
                Some(v @ toml::Value::Array(items)) => {
                    if items.is_empty() {
                        return Err(malformed(v));
                    }
                    items
                        .iter()
                        .map(|x| match x.as_str() {
                            Some(s) if !s.is_empty() => Ok(s.to_string()),
                            _ => Err(malformed(v)),
                        })
                        .collect::<Result<_>>()?
                }
                Some(v) => return Err(malformed(v)),
            };
            Ok((
                ledger.clone(),
                LoanConfig {
                    lender: text("lender")?,
                    lender_type: text("lender_type")?,
                    interest_ledgers,
                },
            ))
        })
        .collect()
}

/// What the reference's pack passes `run()` besides the book, the rules and the loans.
pub struct Inputs<'a> {
    /// `[tds].previous_year_turnover_paise`.
    pub previous_year_turnover_paise: Option<i64>,
    pub cash: &'a BTreeSet<String>,
    pub bank: &'a BTreeSet<String>,
    /// `[loans].shared_interest_ledgers`.
    pub shared_interest_ledgers: &'a BTreeSet<String>,
    /// The ledgers `[statutory_dues].nature_by_ledger` classifies as `tds_payable`.
    pub tds_payable_ledgers: &'a BTreeSet<String>,
    /// `[deductor].activity`.
    pub deductor_activity: Option<DeductorActivity>,
    /// `[tds].previous_year_turnover_status` is a placeholder.
    pub turnover_is_placeholder: bool,
}

fn hash8(text: &str) -> String {
    crate::canonical::hex(&Sha1::digest(text.as_bytes()))[..8].to_string()
}

/// Tally's Receipt base type, whatever its case or surrounding space.
fn is_receipt(base_type: &str) -> bool {
    py_lower(py_strip(base_type)) == "receipt"
}

fn to_i64(n: i128) -> Result<i64> {
    i64::try_from(n).map_err(|_| overflow(TEST_ID))
}

fn net(v: &Voucher, ledger: &str) -> i128 {
    v.lines
        .iter()
        .filter(|l| l.ledger == ledger)
        .map(|l| i128::from(l.amount_paise))
        .sum()
}

fn net_on(v: &Voucher, ledgers: &BTreeSet<String>) -> i128 {
    v.lines
        .iter()
        .filter(|l| ledgers.contains(&l.ledger))
        .map(|l| i128::from(l.amount_paise))
        .sum()
}

fn others_than<'a>(v: &'a Voucher, ledger: &str) -> BTreeSet<&'a str> {
    v.lines
        .iter()
        .filter(|l| l.ledger != ledger)
        .map(|l| l.ledger.as_str())
        .collect()
}

/// [`others_than`] without the ledgers on nil lines: a ledger on a nil line is no posting
/// (bridge#1259 item 3, as `two_sided` and the loan listing read it).
fn posted_others_than<'a>(v: &'a Voucher, ledger: &str) -> BTreeSet<&'a str> {
    v.lines
        .iter()
        .filter(|l| l.ledger != ledger && l.amount_paise != 0)
        .map(|l| l.ledger.as_str())
        .collect()
}

/// A Python list of strings as `repr()` prints it.
fn py_repr_list(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|s| py_repr_str(s)).collect();
    format!("[{}]", inner.join(", "))
}

fn voucher_ref(v: &Voucher) -> EvidenceRef {
    EvidenceRef::with_label("voucher", &v.guid, &voucher_label(v))
}

/// Voucher refs, each once: the reference's `sorted({EvidenceRef(...)})`.
fn voucher_refs<'a>(vs: impl Iterator<Item = &'a Voucher>) -> Vec<EvidenceRef> {
    let set: BTreeSet<(String, String)> = vs.map(|v| (v.guid.clone(), voucher_label(v))).collect();
    set.into_iter()
        .map(|(id, label)| EvidenceRef::with_label("voucher", &id, &label))
        .collect()
}

/// A narration with its whitespace runs as single spaces: `" ".join(n.split())`.
fn normalised(text: &str) -> String {
    py_split(text).join(" ")
}

/// Python's `re.findall(r"\d{6,}", text)`: each maximal run of six or more of Python's decimal
/// digits.
fn bank_numbers(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut run = String::new();
    let mut len = 0;
    for c in text.chars().chain(std::iter::once(' ')) {
        if py_is_decimal(c) {
            run.push(c);
            len += 1;
        } else {
            if len >= 6 {
                out.push(std::mem::take(&mut run));
            }
            run.clear();
            len = 0;
        }
    }
    out
}

/// Whether a character matches `[A-Z]` under Python's `re.I`: an ASCII letter or one of the few
/// characters `re.I` folds onto one.
fn is_ci_letter(c: char) -> bool {
    ('A'..='Z').any(|p| py_ci_eq(c, p))
}

/// Python's `re.search(r"(?<![A-Z])(?:N?ACH|ECS|EMI)(?![A-Z])", text, re.I)`: a bank mandate or
/// instalment word standing alone.
fn has_mandate_word(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    (0..chars.len()).any(|at| {
        (at == 0 || !is_ci_letter(chars[at - 1]))
            && ["NACH", "ACH", "ECS", "EMI"].iter().any(|word| {
                let end = at + word.len();
                end <= chars.len()
                    && word
                        .chars()
                        .zip(&chars[at..end])
                        .all(|(p, &c)| py_ci_eq(c, p))
                    && (end == chars.len() || !is_ci_letter(chars[end]))
            })
    })
}

fn mode(
    counter: &BTreeSet<&str>,
    base_type: &str,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
) -> &'static str {
    if counter.iter().any(|l| cash.contains(*l)) {
        MODE_CASH
    } else if counter.iter().any(|l| bank.contains(*l)) {
        MODE_BANK
    } else if base_type == "Journal" {
        MODE_JOURNAL
    } else {
        MODE_OTHER
    }
}

/// The Form 3CD utility's Note 1 code for a mode and direction; bank carries none.
pub fn mode_code(mode: &str, direction: &str) -> &'static str {
    match (mode, direction) {
        ("cash", "taken") => "B",
        ("cash", "repaid") => "A",
        ("journal", "taken") => "J",
        ("journal", "repaid") => "I",
        ("other", "taken") => "L",
        ("other", "repaid") => "K",
        _ => "",
    }
}

/// A line on an interest ledger: ((date, population position, line index), population position,
/// amount, the voucher's other ledgers). The key's order is the reference's (date, entry order).
type InterestLine<'a> = (
    (&'a TallyDate, usize, usize),
    usize,
    i128,
    BTreeSet<&'a str>,
);

/// A taken or repaid row on one loan ledger.
struct PrincipalRow<'a> {
    /// The voucher's population position: its identity.
    at: usize,
    voucher: &'a Voucher,
    amount: i128,
    mode: &'static str,
    counter: BTreeSet<&'a str>,
}

/// The reference's `compute_loan_rows` for one loan ledger.
struct LoanRows<'a> {
    /// (population position, the loan ledger's own net line).
    interest: Vec<(usize, i128)>,
    taken: Vec<PrincipalRow<'a>>,
    repaid: Vec<PrincipalRow<'a>>,
    /// (population position, the loan's net line) of the taken/repaid rows booked against only
    /// another loan's interest ledger or a shared one, besides TDS lines.
    misposted: Vec<(usize, i128)>,
    /// The population positions of the vouchers that both credit and debit the loan
    /// ([`two_sided`]), in population order: in none of the lists above, never netted into one
    /// row nor skipped when their loan lines net to nil (#779 Phase A). The reference's
    /// `two_sided_vouchers`.
    listed: Vec<usize>,
}

/// The ledgers a loan's other lines are read against.
struct LoanLedgers<'a> {
    interest: &'a BTreeSet<String>,
    tds: &'a BTreeSet<String>,
    other_loans: &'a BTreeSet<&'a str>,
    foreign_interest: &'a BTreeSet<String>,
}

/// The reference's `_two_sided` (#779 Phase A): the voucher both credits and debits the loan
/// ledger (non-zero lines only), and is not an interest or TDS entry. A voucher whose every other
/// non-zero line is one of the loan's interest ledgers or a TDS ledger keeps the interest-journal
/// reading (an accrual and its reversal, say), unless it also carries another loan's line. A
/// voucher on the loan alone is listed only if it does not balance (it would be netted); a
/// balanced one moves no money and keeps the earlier reading.
fn two_sided(v: &Voucher, loan_ledger: &str, ledgers: &LoanLedgers) -> bool {
    let on_loan = || {
        v.lines
            .iter()
            .filter(|l| l.ledger == loan_ledger && l.amount_paise != 0)
    };
    if !(on_loan().any(|l| l.amount_paise > 0) && on_loan().any(|l| l.amount_paise < 0)) {
        return false;
    }
    let others: BTreeSet<&str> = v
        .lines
        .iter()
        .filter(|l| l.ledger != loan_ledger && l.amount_paise != 0)
        .map(|l| l.ledger.as_str())
        .collect();
    if others.is_empty() {
        return v
            .lines
            .iter()
            .map(|l| i128::from(l.amount_paise))
            .sum::<i128>()
            != 0;
    }
    others.iter().any(|o| ledgers.other_loans.contains(o))
        || !others
            .iter()
            .all(|o| ledgers.interest.contains(*o) || ledgers.tds.contains(*o))
}

fn compute_loan_rows<'a>(
    pop: &[&'a Voucher],
    loan_ledger: &str,
    ledgers: &LoanLedgers,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
) -> LoanRows<'a> {
    let ils = ledgers.interest;
    let mut rows = LoanRows {
        interest: Vec::new(),
        taken: Vec::new(),
        repaid: Vec::new(),
        misposted: Vec::new(),
        listed: Vec::new(),
    };
    for (at, v) in pop.iter().copied().enumerate() {
        if v.base_type == "Contra" || !v.lines.iter().any(|l| l.ledger == loan_ledger) {
            continue;
        }
        if two_sided(v, loan_ledger, ledgers) {
            rows.listed.push(at);
            continue; // listed by run(), never netted (#779 Phase A)
        }
        let loan_amt = net(v, loan_ledger);
        if loan_amt == 0 {
            continue;
        }
        // A ledger on a nil line is no posting: a nil line beside an interest journal left it "not
        // only the interest ledger and TDS", and a nil cash line made a bank loan a cash one
        // (bridge#1259 item 3).
        let others = posted_others_than(v, loan_ledger);
        // rest = others - (tds - ils): the other lines, besides TDS ledgers that are not also this
        // loan's interest ledgers.
        let rest: BTreeSet<&str> = others
            .iter()
            .copied()
            .filter(|o| !ledgers.tds.contains(*o) || ils.contains(*o))
            .collect();
        if !ils.is_empty() && !rest.is_empty() && rest.iter().all(|o| ils.contains(*o)) {
            rows.interest.push((at, loan_amt));
            continue;
        }
        // A TDS journal is Loan Dr [/ other loans Dr] / TDS Payable Cr. Another loan on the
        // opposite side makes it a transfer between loans, a row on each
        // (bridge#803: read as TDS, both rows were lost). The sign product is taken on signs, so
        // it cannot overflow.
        let opposite_loan = v.lines.iter().any(|l| {
            ledgers.other_loans.contains(l.ledger.as_str())
                && i128::from(l.amount_paise).signum() * loan_amt.signum() < 0
        });
        if others.iter().any(|o| ledgers.tds.contains(*o))
            && others
                .iter()
                .all(|o| ledgers.tds.contains(*o) || ledgers.other_loans.contains(o))
            && !opposite_loan
        {
            continue; // a TDS journal: neither taken nor repaid
        }
        let m = mode(&others, &v.base_type, cash, bank);
        let non_tds: Vec<&str> = others
            .iter()
            .copied()
            .filter(|o| !ledgers.tds.contains(*o))
            .collect();
        if !non_tds.is_empty()
            && non_tds
                .iter()
                .all(|o| ledgers.foreign_interest.contains(*o))
        {
            rows.misposted.push((at, loan_amt));
        }
        let row = PrincipalRow {
            at,
            voucher: v,
            amount: loan_amt.abs(),
            mode: m,
            counter: others,
        };
        if loan_amt < 0 {
            rows.taken.push(row);
        } else {
            rows.repaid.push(row);
        }
    }
    rows
}

/// One walked taken/repaid row, with every balance the reference's walk carries.
struct WalkedRow<'a> {
    at: usize,
    voucher: &'a Voucher,
    amount: i128,
    mode: &'static str,
    direction: &'static str,
    prior_outstanding: i128,
    after_outstanding: i128,
    prior_breach: i128,
    prior_outstanding_w2: i128,
    after_outstanding_w2: i128,
    prior_breach_w2: i128,
    prior_outstanding_net: i128,
    after_outstanding_net: i128,
    prior_breach_net: i128,
    after_breach: i128,
    /// bridge#802: a principal repayment on or after the date of the first interest credited to
    /// the loan comes before this row in the walk, so it may have paid that interest first.
    repaid_since_interest: bool,
}

/// The reference's `compute_running_balance_rows`: taken, repaid and interest rows walked together
/// in (date, GUID) order, three ways: floored at the opening (the walk every figure reads); floored
/// before every taken row as well (the fresh-loan reading); and from the opening as the books show
/// it (the netting reading). Each row also carries `repaid_since_interest` (bridge#802, the
/// reference's closing form): whether a principal repayment on or after the date of the first
/// interest credited to the loan comes before it in this walk. Such a repayment may have paid that
/// interest first, which the books do not record; `run` then reads a taken row also on the breach
/// balance, an upper bound on the principal still owed.
fn compute_running_balance_rows<'a>(
    pop: &[&'a Voucher],
    rows: &LoanRows<'a>,
    opening_outstanding: i128,
) -> Vec<WalkedRow<'a>> {
    // (position, amount, mode, direction, is_interest), built in the reference's order before its
    // stable sort.
    let mut combined: Vec<(usize, i128, &'static str, &'static str, bool)> = Vec::new();
    combined.extend(
        rows.taken
            .iter()
            .map(|r| (r.at, r.amount, r.mode, "taken", false)),
    );
    combined.extend(
        rows.repaid
            .iter()
            .map(|r| (r.at, r.amount, r.mode, "repaid", false)),
    );
    combined.extend(rows.interest.iter().map(|&(at, amt)| {
        (
            at,
            amt.abs(),
            "",
            if amt < 0 { "taken" } else { "repaid" },
            true,
        )
    }));
    combined
        .sort_by(|a, b| (&pop[a.0].date, &pop[a.0].guid).cmp(&(&pop[b.0].date, &pop[b.0].guid)));
    let mut principal = opening_outstanding.max(0);
    let mut breach = principal;
    let (mut principal_w2, mut breach_w2) = (principal, breach);
    let (mut principal_net, mut breach_net) = (opening_outstanding, opening_outstanding);
    let first_interest = combined
        .iter()
        .filter(|row| row.4 && row.3 == "taken")
        .map(|row| &pop[row.0].date)
        .min();
    let mut repaid_since_interest = false;
    let mut out = Vec::new();
    for (at, amount, m, direction, is_interest) in combined {
        let (principal_prior, breach_prior) = (principal, breach);
        let sign = if direction == "taken" { 1 } else { -1 };
        if !is_interest && direction == "taken" && principal_w2 < 0 {
            // The second walk: a credit after a debit balance is a fresh loan.
            let lift = -principal_w2;
            principal_w2 = 0;
            breach_w2 += lift;
        }
        let (principal_prior_w2, breach_prior_w2) = (principal_w2, breach_w2);
        let (principal_prior_net, breach_prior_net) = (principal_net, breach_net);
        breach = breach_prior + sign * amount;
        breach_w2 = breach_prior_w2 + sign * amount;
        breach_net = breach_prior_net + sign * amount;
        if is_interest {
            continue; // moves the breach balance only
        }
        principal = principal_prior + sign * amount;
        principal_w2 = principal_prior_w2 + sign * amount;
        principal_net = principal_prior_net + sign * amount;
        out.push(WalkedRow {
            at,
            voucher: pop[at],
            amount,
            mode: m,
            direction,
            prior_outstanding: principal_prior,
            after_outstanding: principal,
            prior_breach: breach_prior,
            prior_outstanding_w2: principal_prior_w2,
            after_outstanding_w2: principal_w2,
            prior_breach_w2: breach_prior_w2,
            prior_outstanding_net: principal_prior_net,
            after_outstanding_net: principal_net,
            prior_breach_net: breach_prior_net,
            after_breach: breach,
            repaid_since_interest,
        });
        if direction == "repaid" && first_interest.is_some_and(|d| pop[at].date >= *d) {
            repaid_since_interest = true;
        }
    }
    out
}

fn days_between(from: &TallyDate, to: &TallyDate) -> i64 {
    civil_day_number(to) - civil_day_number(from)
}

/// A (repaid row, taken row) pair on one loan that may be a bank debit and its return, with the
/// numbers they share (empty for an identical-narration pair): the reference's `reversal_pairs`.
fn reversal_pairs(
    taken: &[PrincipalRow],
    repaid: &[PrincipalRow],
    bank: &BTreeSet<String>,
) -> Vec<(usize, usize, Vec<String>)> {
    let bank_only =
        |x: &PrincipalRow| !x.counter.is_empty() && x.counter.iter().all(|l| bank.contains(*l));
    let key = |x: &PrincipalRow| -> Option<String> {
        let narration = normalised(&x.voucher.narration);
        (bank_only(x) && !bank_numbers(&narration).is_empty()).then_some(narration)
    };
    let keys_taken: Vec<Option<String>> = taken.iter().map(key).collect();
    let keys_repaid: Vec<Option<String>> = repaid.iter().map(key).collect();
    let every: Vec<String> = taken
        .iter()
        .chain(repaid)
        .map(|x| normalised(&x.voucher.narration))
        .collect();
    let in_window = |d: &PrincipalRow, c: &PrincipalRow| {
        let days = days_between(&d.voucher.date, &c.voucher.date);
        c.amount == d.amount && (0..=REVERSAL_PAIR_MAX_DAYS).contains(&days)
    };
    let mut pairs: Vec<(usize, usize, Vec<String>)> = Vec::new();
    for (di, d) in repaid.iter().enumerate() {
        let Some(d_key) = &keys_repaid[di] else {
            continue;
        };
        let taken_with = |k: &Option<String>| k.as_ref() == Some(d_key);
        if every.iter().filter(|n| *n == d_key).count() != 2
            || keys_taken.iter().filter(|k| taken_with(k)).count() != 1
        {
            continue;
        }
        let Some(ci) = keys_taken.iter().position(taken_with) else {
            continue;
        };
        if in_window(d, &taken[ci]) {
            pairs.push((di, ci, Vec::new()));
        }
    }
    let paired_repaid: HashSet<usize> = pairs.iter().map(|p| p.0).collect();
    let paired_taken: HashSet<usize> = pairs.iter().map(|p| p.1).collect();
    let numbers = |x: &PrincipalRow| -> BTreeSet<String> {
        bank_numbers(&format!("{} {}", x.voucher.narration, x.voucher.reference))
            .into_iter()
            .collect()
    };
    let numbers_taken: Vec<BTreeSet<String>> = taken.iter().map(numbers).collect();
    let numbers_repaid: Vec<BTreeSet<String>> = repaid.iter().map(numbers).collect();
    let mut carriers: HashMap<&str, usize> = HashMap::new();
    for n in numbers_taken.iter().chain(&numbers_repaid).flatten() {
        *carriers.entry(n.as_str()).or_insert(0) += 1;
    }
    let mut by_number: Vec<(usize, Vec<usize>, BTreeSet<&str>)> = Vec::new();
    for (di, d) in repaid.iter().enumerate() {
        if paired_repaid.contains(&di) || !bank_only(d) {
            continue;
        }
        let unique: BTreeSet<&str> = numbers_repaid[di]
            .iter()
            .map(String::as_str)
            .filter(|n| carriers[n] == 2)
            .collect();
        let found: Vec<usize> = taken
            .iter()
            .enumerate()
            .filter(|(ci, c)| {
                bank_only(c)
                    && !paired_taken.contains(ci)
                    && numbers_taken[*ci]
                        .iter()
                        .any(|n| unique.contains(n.as_str()))
                    && c.at != d.at
                    && in_window(d, c)
            })
            .map(|(ci, _)| ci)
            .collect();
        by_number.push((di, found, unique));
    }
    let mut claims: HashMap<usize, usize> = HashMap::new();
    for (_, found, _) in &by_number {
        for c in found {
            *claims.entry(*c).or_insert(0) += 1;
        }
    }
    for (di, found, unique) in by_number {
        if let [ci] = found[..] {
            if claims[&ci] == 1 {
                let shared = numbers_taken[ci]
                    .iter()
                    .filter(|n| unique.contains(n.as_str()))
                    .cloned()
                    .collect();
                pairs.push((di, ci, shared));
            }
        }
    }
    pairs
}

fn paise(n: i128) -> Result<Value> {
    Ok(Value::Int(to_i64(n)?))
}

fn text(t: &str) -> Value {
    Value::Text(t.to_string())
}

/// The reference's `_refund_or_loan_record`: a taken or repaid row whose reportability differs
/// between the two walks. Listed as not computed: outside clause31/, no "amount" fact, never
/// summed, and a s.269SS/269T tag only as "possible".
/// `extra_limit`, on a loan with a voucher listed as both crediting and debiting it, says the row
/// was not compared with that voucher.
#[allow(clippy::too_many_arguments)]
fn refund_or_loan_record(
    r: &mut TestResult,
    loan_ledger: &str,
    lender: &str,
    h: &str,
    row: &WalkedRow,
    clause: &str,
    verdict_net: (bool, bool),
    verdict_w2: (bool, bool),
    extra_limit: Option<&str>,
) -> Result<()> {
    let state = |(reportable, flagged): (bool, bool)| {
        if flagged {
            "reportable and flagged under s.269SS/269T"
        } else if reportable {
            "reportable, not flagged"
        } else {
            "not reportable"
        }
    };
    let (direction, v) = (row.direction, row.voucher);
    let vh = hash8(&v.guid);
    let rid = format!("{direction}_{h}_{vh}");
    let f_amt = r.fig(
        &format!("not_computed_entry_amount_{rid}"),
        paise(row.amount)?,
        Unit::Paise,
        &format!(
            "Loan {direction} on voucher (tag {vh}) against loan ledger (tag {h}): the entry's own \
amount. Its reportability is not computed, so it is in no reportable total."
        ),
        vec![voucher_ref(v)],
    )?;
    let f_mode = r.fig(
        &format!("not_computed_entry_mode_{rid}"),
        text(row.mode),
        Unit::Text,
        &format!(
            "Mode of this {direction} entry, read from its counter-line ledger group(s), as for \
every entry."
        ),
        Vec::new(),
    )?;
    let possible = verdict_net.1 || verdict_w2.1;
    let entry = if direction == "taken" {
        "loan taken"
    } else {
        "repayment"
    };
    let mut clauses = vec![clause.to_string()];
    if possible {
        clauses.push(
            if direction == "taken" {
                "s.269SS"
            } else {
                "s.269T"
            }
            .to_string(),
        );
    }
    r.findings.push(Finding {
        id: format!("{TEST_ID}/not_computed/refund_or_loan_{rid}"),
        clauses,
        title: format!(
            "Loan {direction} against {lender} ({} mode): not computed -- the principal balance \
this test walks went below zero before it, and the books do not show how to read the credits after \
that{}",
            row.mode,
            if possible {
                " (possible s.269SS/269T)"
            } else {
                ""
            }
        ),
        facts: vec![
            ("entry_amount".to_string(), f_amt),
            ("mode".to_string(), f_mode),
        ],
        evidence: vec![voucher_ref(v), EvidenceRef::new("ledger", loan_ledger)],
        confidence: Confidence::JudgementRequired,
        limits: vec![
            format!(
                "{TWO_WALKS_TEXT} This {entry} is {} under the first and {} under the second, so \
neither is chosen: it is in no reportable total and not in the s.269SS/269T flag count. \
{SAME_DAY_TEXT}",
                state(verdict_net),
                state(verdict_w2)
            ),
            "The questions this test asks on a computed entry (the lender's charge, a returned \
debit, a repeated narration, the mode read from the ledger group) are not asked on this record, \
though the figures counting such entries may cite it."
                .to_string(),
        ]
        .into_iter()
        .chain(extra_limit.map(str::to_string))
        .collect(),
        ask_client: vec![REFUND_OR_LOAN_ASK.to_string()],
    });
    Ok(())
}

// bridge#802, in the reference's closing form: a loan taken, after a repayment made once interest
// had been credited, that reaches the limit on the breach balance but not on the walked principal.
const INTEREST_FIRST_TEXT: &str = "The principal balance this test walks on this loan ledger is \
principal only: a repayment made after interest was credited to the loan is read as paying \
principal. If it paid that interest first, more principal was still owed before this entry -- at \
most the principal plus the interest credited and not yet paid, and the books do not record which. \
That upper bound leaves out journals against ledgers configured as TDS payable, so it may \
overstate.";
const INTEREST_FIRST_ASK: &str = "How each repayment on this loan before this entry was applied \
(to the interest credited or to principal), and the principal outstanding with the lender before \
this entry.";

/// bridge#802: listed as not computed, as R2's record is: outside `clause31/`, no `amount` fact,
/// never summed, and a s.269SS tag only as "possible". It states no balance: the upper bound may
/// overstate.
#[allow(clippy::too_many_arguments)]
fn interest_first_record(
    r: &mut TestResult,
    loan_ledger: &str,
    lender: &str,
    h: &str,
    row: &WalkedRow,
    clause: &str,
    verdict_p: (bool, bool),
    verdict_b: (bool, bool),
    extra_limit: Option<&str>,
) -> Result<()> {
    let state = |(reportable, flagged): (bool, bool)| {
        if flagged {
            "reportable and flagged under s.269SS"
        } else if reportable {
            "reportable, not flagged"
        } else {
            "not reportable"
        }
    };
    let v = row.voucher;
    let vh = hash8(&v.guid);
    let rid = format!("taken_{h}_{vh}");
    let f_amt = r.fig(
        &format!("not_computed_entry_amount_{rid}"),
        paise(row.amount)?,
        Unit::Paise,
        &format!(
            "Loan taken on voucher (tag {vh}) against loan ledger (tag {h}): the entry's own \
amount. Its reportability is not computed, so it is in no reportable total."
        ),
        vec![voucher_ref(v)],
    )?;
    let f_mode = r.fig(
        &format!("not_computed_entry_mode_{rid}"),
        text(row.mode),
        Unit::Text,
        "Mode of this taken entry, read from its counter-line ledger group(s), as for every entry.",
        Vec::new(),
    )?;
    let possible = verdict_p.1 || verdict_b.1;
    let mut clauses = vec![clause.to_string()];
    if possible {
        clauses.push("s.269SS".to_string());
    }
    r.findings.push(Finding {
        id: format!("{TEST_ID}/not_computed/interest_first_{rid}"),
        clauses,
        title: format!(
            "Loan taken against {lender} ({} mode): not computed -- it may cross the limit if an \
earlier repayment paid the interest credited first, which the books do not show{}",
            row.mode,
            if possible { " (possible s.269SS)" } else { "" }
        ),
        facts: vec![
            ("entry_amount".to_string(), f_amt),
            ("mode".to_string(), f_mode),
        ],
        evidence: vec![voucher_ref(v), EvidenceRef::new("ledger", loan_ledger)],
        confidence: Confidence::JudgementRequired,
        limits: vec![
            format!(
                "{INTEREST_FIRST_TEXT} This loan taken is {} on the walked principal and {} on \
that upper bound, so neither is chosen: it is in no reportable total and not in the s.269SS/269T \
flag count. Entries dated the same day are walked in voucher-id order, which the books do not \
record.",
                state(verdict_p),
                state(verdict_b)
            ),
            "The questions this test asks on a computed entry (the lender's charge, a returned \
debit, a repeated narration, the mode read from the ledger group) are not asked on this record, \
though the figures counting such entries may cite it."
                .to_string(),
        ]
        .into_iter()
        .chain(extra_limit.map(str::to_string))
        .collect(),
        ask_client: vec![INTEREST_FIRST_ASK.to_string()],
    });
    Ok(())
}

// #779 Phase A: a voucher both crediting and debiting a loan is listed, never netted or divided.
const TWO_SIDED_TITLE: &str = "Loan ledger both credited and debited in one voucher: listed as \
the books hold it, not divided into entries by this test";
const LISTED_TOTALS_NOTE: &str = " A voucher listed as both crediting and debiting one loan is in \
neither total, even where one side reaches the limit; from its date on, an entry on that loan is in \
it only if its own amount reaches the limit.";
const TWO_SIDED_ASK: &str = "Entry by entry, what was taken from the lender, repaid to it, \
credited as interest or a charge, and withheld as TDS in this voucher, and by what mode (cash, bank, \
adjustment)? If part of it corrects another part, which?";

/// A voucher listed as both crediting and debiting a loan: its sides on the loan, the sum of its
/// lines, and the facts citing the figures made for them, in the reference's order.
struct ListedVoucher<'a> {
    voucher: &'a Voucher,
    credit: i128,
    debit: i128,
    imbalance: i128,
    facts: Vec<(String, String)>,
}

/// The reference's `_two_sided_record` (kind 1): a voucher both crediting and debiting the loan,
/// listed as the books hold it. Outside clause31/ and with no "amount" fact, so it is never summed.
/// `roles` holds its other non-zero lines by (role, side), summed.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn two_sided_record(
    r: &mut TestResult,
    loan_ledger: &str,
    lender: &str,
    h: &str,
    listed: &ListedVoucher,
    clauses31: &[&str],
    possible_269: &[&str],
    roles: &BTreeMap<(&str, &str), i128>,
    first_dependent: bool,
) -> Result<()> {
    let v = listed.voucher;
    let vh = hash8(&v.guid);
    let vid = format!("{h}_{vh}");
    let mut facts = listed.facts.clone();
    for (&(role, side), &amount) in roles {
        let id = r.fig(
            &format!("two_sided_{role}_{side}_{vid}"),
            paise(amount)?,
            Unit::Paise,
            &format!(
                "This voucher's (tag {vh}) {side} on ledgers other than loan ledger (tag {h}) read \
as {} (from the ledger's group or the client's configuration), summed.",
                role.replace('_', " ")
            ),
            vec![voucher_ref(v)],
        )?;
        facts.push((format!("{role}_{side}"), id));
    }
    let other_debits: i128 = roles
        .iter()
        .filter(|((_, side), _)| *side == "debits")
        .map(|(_, amount)| *amount)
        .sum();
    let set_off = (listed.credit - other_debits).max(0);
    let balanced = listed.imbalance == 0;
    if set_off != 0 && balanced {
        let id = r.fig(
            &format!("two_sided_forced_set_off_{vid}"),
            paise(set_off)?,
            Unit::Paise,
            &format!(
                "At least this much of this voucher's (tag {vh}) credit to loan ledger (tag {h}) is \
set off against its debit to the loan: the voucher's other debit lines total less than that credit."
            ),
            vec![voucher_ref(v)],
        )?;
        facts.push(("forced_set_off".to_string(), id));
    }
    let mut limits = vec![format!(
        "This voucher credits the loan {} and debits it {}. This test does not yet divide a voucher \
with both sides on one loan into its entries, so it lists it as the books hold it: in no reportable \
total and not in the s.269SS/269T flag count. Both sides stay in the loan's credits and debits \
before any filter.",
        rupees(listed.credit),
        rupees(listed.debit)
    )];
    if set_off != 0 && balanced {
        limits.push(format!(
            "By the voucher's own arithmetic at least {} of the credit is set off against the debit: \
its other debit lines total less than the credit.",
            rupees(set_off)
        ));
    }
    if !balanced {
        limits.push(format!(
            "The voucher's lines do not sum to zero (difference {}, debits positive).",
            rupees(listed.imbalance)
        ));
    }
    if first_dependent {
        limits.push(
            "Every entry on this loan dated on or after this voucher is tested on its own amount \
only: its reportability by the running balance is not computed, and neither is the loan's maximum \
balance."
                .to_string(),
        );
    }
    if !possible_269.is_empty() {
        limits.push(format!(
            "Possible {}: the voucher's other {} a ledger that is not a bank account (cash, a \
journal or another ledger). Whether a limit is breached is not computed; this is not in the \
s.269SS/269T flag count.",
            possible_269.join(" and "),
            if possible_269.len() == 2 {
                "debit and credit lines each include"
            } else if possible_269 == ["s.269SS"].as_slice() {
                "debit lines include"
            } else {
                "credit lines include"
            }
        ));
    }
    if roles.keys().any(|(role, _)| *role == "expense") {
        limits.push(
            "An expense ledger is among its other lines: part of it may be the lender's interest or \
charge (s.2(28A)), which is interest, not a loan taken or repaid."
                .to_string(),
        );
    }
    if roles.keys().any(|(role, _)| *role == "other_interest") {
        limits.push(
            "Another loan's interest ledger, or a shared one, is among its other lines: part of it \
may be interest booked to the wrong ledger."
                .to_string(),
        );
    }
    limits.push(
        "It was not compared with this loan's other entries for reversal pairs or repeated \
narrations."
            .to_string(),
    );
    r.findings.push(Finding {
        id: format!("{TEST_ID}/not_computed/two_sided_{vid}"),
        clauses: clauses31
            .iter()
            .chain(possible_269)
            .map(|c| (*c).to_string())
            .collect(),
        title: format!(
            "{TWO_SIDED_TITLE} ({lender}){}",
            if possible_269.is_empty() {
                ""
            } else {
                " (possible s.269SS/269T)"
            }
        ),
        facts,
        evidence: vec![voucher_ref(v), EvidenceRef::new("ledger", loan_ledger)],
        confidence: Confidence::JudgementRequired,
        limits,
        ask_client: vec![TWO_SIDED_ASK.to_string()],
    });
    Ok(())
}

/// The reference's `_depends_record` (kind 3): an entry dated on or after a voucher listed as both
/// crediting and debiting its loan, below the limit on its own amount, so its reportability depends
/// on the balance, which is not known. Its own voucher, date and amount are shown; outside
/// clause31/, no "amount" fact, never summed. `listed_labels` names the listed vouchers dated on or
/// before it.
fn depends_record(
    r: &mut TestResult,
    c: &RowContext,
    row: &WalkedRow,
    clause: &str,
    possible: bool,
    listed_labels: &str,
    first_listed: &TallyDate,
) -> Result<()> {
    let (direction, v, h) = (row.direction, row.voucher, c.h);
    let vh = hash8(&v.guid);
    let rid = format!("{direction}_{h}_{vh}");
    let f_amt = r.fig(
        &format!("depends_entry_amount_{rid}"),
        paise(row.amount)?,
        Unit::Paise,
        &format!(
            "Loan {direction} on voucher (tag {vh}) against loan ledger (tag {h}): the entry's own \
amount. Its reportability is not computed, so it is in no reportable total."
        ),
        vec![voucher_ref(v)],
    )?;
    let f_mode = r.fig(
        &format!("depends_entry_mode_{rid}"),
        text(row.mode),
        Unit::Text,
        &format!(
            "Mode of this {direction} entry, read from its counter-line ledger group(s), as for \
every entry."
        ),
        Vec::new(),
    )?;
    let mut clauses = vec![clause.to_string()];
    if possible {
        clauses.push(
            if direction == "taken" {
                "s.269SS"
            } else {
                "s.269T"
            }
            .to_string(),
        );
    }
    r.findings.push(Finding {
        id: format!("{TEST_ID}/not_computed/depends_{rid}"),
        clauses,
        title: format!(
            "Loan {direction} against {} ({} mode): not computed -- it depends on the not-computed \
voucher(s) {listed_labels}{}",
            c.lender,
            row.mode,
            if possible {
                " (possible s.269SS/269T)"
            } else {
                ""
            }
        ),
        facts: vec![
            ("entry_amount".to_string(), f_amt),
            ("mode".to_string(), f_mode),
        ],
        evidence: vec![voucher_ref(v), EvidenceRef::new("ledger", c.loan_ledger)],
        confidence: Confidence::JudgementRequired,
        limits: vec![
            format!(
                "Voucher(s) {listed_labels} both credit and debit this loan and are listed as the \
books hold them, not divided into entries, so the balance owed to the lender from {} on is not \
known. This entry's own amount ({}) is below the s.269SS/269T limit ({}), so whether it is \
reportable depends on that balance: not computed. It is in no reportable total and not in the \
s.269SS/269T flag count.",
                crate::read::iso(first_listed),
                rupees(row.amount),
                rupees(c.limit_269)
            ),
            c.not_compared.to_string(),
            "The questions this test asks on a computed entry (the lender's charge, a returned \
debit, a repeated narration) are not asked on this record."
                .to_string(),
        ],
        ask_client: vec![format!(
            "The entries in voucher(s) {listed_labels}, and the balance owed to {} before this \
entry.",
            c.lender
        )],
    });
    Ok(())
}

/// Run the test with the rule in force ([`NET_REVERSALS`]).
pub fn run(
    book: &Book,
    rules: &Rules,
    entity_type: &str,
    loans: &BTreeMap<String, LoanConfig>,
    inputs: &Inputs,
) -> Result<TestResult> {
    run_with(book, rules, entity_type, loans, inputs, NET_REVERSALS)
}

/// The lender-type lists a loan's reportability and flags read.
struct Exemptions<'a> {
    s194a: BTreeSet<&'a str>,
    s269: BTreeSet<&'a str>,
    /// Clause 31(c)-(e)'s note: repayments.
    reporting: BTreeSet<&'a str>,
    /// Clause 31(a)/(b)'s note: loans taken or accepted.
    reporting_accepted: BTreeSet<&'a str>,
}

impl Exemptions<'_> {
    fn reporting_for(&self, direction: &str) -> &BTreeSet<&str> {
        if direction == "taken" {
            &self.reporting_accepted
        } else {
            &self.reporting
        }
    }

    /// (clause, its exemption list) for the three tests a lender's type gates.
    fn by_clause(&self) -> [(&'static str, &BTreeSet<&str>); 3] {
        [
            ("3CD-31(a)", &self.reporting_accepted),
            ("3CD-31(c)", &self.reporting),
            ("s.194A", &self.s194a),
        ]
    }
}

/// Running counts across every loan.
#[derive(Default)]
struct Counts {
    taken_reportable_total: i128,
    repaid_reportable_total: i128,
    flag_count: usize,
    possible_count: usize,
    /// Vouchers listed as both crediting and debiting a loan, once per loan ledger.
    two_sided_count: usize,
    listed_taken: usize,
    listed_repaid: usize,
    flag_not_computed_count: usize,
    tds_over_threshold_count: usize,
}

/// [`run`], with the shared-ledger rule given.
#[allow(clippy::too_many_lines)]
pub fn run_with(
    book: &Book,
    rules: &Rules,
    entity_type: &str,
    loans: &BTreeMap<String, LoanConfig>,
    inputs: &Inputs,
    net_reversals: bool,
) -> Result<TestResult> {
    let missing = |table: &str| AuditError::Config(format!("{TEST_ID} needs rules [{table}]"));
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let pop = book.population()?;
    r.population_note = "Books population (optional, cancelled and post-dated vouchers excluded); \
Contra excluded throughout. A loan ledger's interest journals are vouchers whose only other ledger \
lines are its configured interest ledger or ledgers (the client's list of loans) and any ledger the \
client configures as TDS payable; every other voucher touching the loan ledger is a taken (credit) \
or repaid (debit) transaction, never classified by ledger name -- except one that both credits \
and debits the loan ledger, which is listed as the books hold it, not divided into entries."
        .to_string();

    let s194a = rules.s194a.as_ref().ok_or_else(|| missing("s194a"))?;
    let threshold_194a = i128::from(s194a.threshold_other_than_securities_paise);
    let limit_269 = i128::from(rules.s269ss_269t_limit_paise);
    fn listed<'a>(v: &'a Option<Vec<String>>, default: &[&'static str]) -> BTreeSet<&'a str> {
        match v {
            Some(v) => v.iter().map(String::as_str).collect(),
            None => default.iter().copied().collect(),
        }
    }
    let ex = Exemptions {
        s194a: s194a
            .exempt_lender_types
            .iter()
            .map(String::as_str)
            .collect(),
        s269: listed(
            &rules.s269ss_269t_exempt_lender_types,
            &DEFAULT_269_EXEMPT_LENDER_TYPES,
        ),
        reporting: listed(
            &rules.s269ss_269t_reporting_exempt_lender_types,
            &DEFAULT_269_REPORTING_EXEMPT_LENDER_TYPES,
        ),
        reporting_accepted: listed(
            &rules.s269ss_269t_reporting_exempt_lender_types_accepted,
            &DEFAULT_269_REPORTING_EXEMPT_ACCEPTED_LENDER_TYPES,
        ),
    };
    let rate_194a = rules.tds_rates.as_ref().and_then(|t| t.s194a_bp);

    // ---------------------------------------------------------------- deductor status (reused)
    let status = deductor_status(
        entity_type,
        rules,
        inputs.previous_year_turnover_paise,
        inputs.deductor_activity,
        inputs.turnover_is_placeholder,
    )?;
    let f_status = r.fig(
        "deductor_status",
        text(status),
        Unit::Text,
        "Whether the assessee must deduct TDS under s.194A for the year -- the same rule and figure as \
'TDS on payments made': firm/LLP/company always; individual/HUF only if previous-year business \
turnover exceeded the rules' turnover limit for an individual or HUF.",
        Vec::new(),
    )?;
    if status == "unknown" {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/deductor_status"),
            clauses: vec!["3CD-21(b)".to_string(), "3CD-34(a)".to_string()],
            title: "Deductor status for s.194A depends on previous-year turnover".to_string(),
            facts: vec![("deductor_status".to_string(), f_status)],
            evidence: Vec::new(),
            confidence: Confidence::JudgementRequired,
            limits: vec![deductor_question(
                rules,
                "s.194A",
                inputs.deductor_activity,
                inputs.turnover_is_placeholder,
            )?],
            ask_client: vec![
                "Confirm previous-year turnover or gross receipts, and whether from \
a business or a profession."
                    .to_string(),
            ],
        });
    }

    let mut n = Counts::default();
    // The population positions each interest ledger's paired loans counted as interest journals:
    // by identity, never by GUID (a GUID can be blank or repeated -- POP-5).
    let mut interest_vouchers: HashMap<&str, HashSet<usize>> = HashMap::new();

    // A ledger that is any loan's interest ledger, or a declared shared one, is never a TDS line
    // here, even if the client's statutory dues also classify it as TDS payable.
    let not_tds: BTreeSet<&str> = loans
        .values()
        .flat_map(|c| c.interest_ledgers.iter().map(String::as_str))
        .chain(inputs.shared_interest_ledgers.iter().map(String::as_str))
        .collect();
    let tds_here: BTreeSet<String> = inputs
        .tds_payable_ledgers
        .iter()
        .filter(|l| !not_tds.contains(l.as_str()))
        .cloned()
        .collect();
    r.fig(
        "tds_payable_ledgers",
        text(&tds_here.iter().map(String::as_str).collect::<Vec<_>>().join("\n")),
        Unit::Text,
        "The ledgers the client's statutory dues classify as TDS payable, less every loan's interest \
ledger and every shared one, one per line: the TDS lines this test reads. Its own consistency checks \
read them back.",
        Vec::new(),
    )?;
    for (loan_ledger, cfg) in loans {
        let (lender, lender_type) = (cfg.lender.as_str(), cfg.lender_type.as_str());
        let ils = &cfg.interest_ledgers;
        let h = stable_ledger_tag(book, loan_ledger)?;
        let other_loans: BTreeSet<&str> = loans
            .keys()
            .map(String::as_str)
            .filter(|o| *o != loan_ledger)
            .collect();
        let foreign_interest: BTreeSet<String> = loans
            .iter()
            .filter(|(o, _)| *o != loan_ledger)
            .flat_map(|(_, c)| c.interest_ledgers.iter())
            .chain(inputs.shared_interest_ledgers)
            .filter(|l| !ils.contains(*l) && !loans.contains_key(*l))
            .cloned()
            .collect();
        let rows = compute_loan_rows(
            &pop,
            loan_ledger,
            &LoanLedgers {
                interest: ils,
                tds: &tds_here,
                other_loans: &other_loans,
                foreign_interest: &foreign_interest,
            },
            inputs.cash,
            inputs.bank,
        );
        for il in ils {
            interest_vouchers
                .entry(il.as_str())
                .or_default()
                .extend(rows.interest.iter().map(|&(at, _)| at));
        }
        // TDS deducted on the interest journals themselves: the interest is counted gross.
        let tds_on_journals: i128 = rows
            .interest
            .iter()
            .map(|&(at, _)| net_on(pop[at], &tds_here))
            .sum();
        let interest_total: i128 =
            -rows.interest.iter().map(|&(_, amt)| amt).sum::<i128>() - tds_on_journals;
        // #779 Phase A: a voucher both crediting and debiting the loan is listed as the books hold
        // it, never netted. Its credit and debit sides stay in the totals before any filter (so
        // LOAN-1 ties), in no reportable total.
        let listed: Vec<&Voucher> = rows.listed.iter().map(|&at| pop[at]).collect();
        let side = |v: &Voucher, debit: bool| -> i128 {
            v.lines
                .iter()
                .filter(|l| l.ledger == *loan_ledger && l.amount_paise != 0)
                .filter(|l| (l.amount_paise > 0) == debit)
                .map(|l| i128::from(l.amount_paise).abs())
                .sum()
        };
        let sides: Vec<(i128, i128)> = listed
            .iter()
            .map(|&v| (side(v, false), side(v, true)))
            .collect();
        let listed_credits: i128 = sides.iter().map(|&(a, _)| a).sum();
        let listed_debits: i128 = sides.iter().map(|&(_, b)| b).sum();
        let taken_total: i128 = rows.taken.iter().map(|x| x.amount).sum::<i128>() + listed_credits;
        let repaid_total: i128 = rows.repaid.iter().map(|x| x.amount).sum::<i128>() + listed_debits;

        let f_lender_type = r.fig(
            &format!("lender_type_{h}"),
            text(lender_type),
            Unit::Text,
            &format!(
                "Lender type for loan ledger (tag {h}) from the client's list of loans, \
classified by the loan ledger this interest pairs with -- never by a word in the ledger name."
            ),
            Vec::new(),
        )?;
        let ev_interest = voucher_refs(rows.interest.iter().map(|&(at, _)| pop[at]));
        let f_int = r.fig(
            &format!("interest_total_{h}"),
            paise(interest_total)?,
            Unit::Paise,
            &format!(
                "Interest paid/credited on loan ledger (tag {h}): sum of every voucher whose only \
other lines are its configured interest ledger(s) and any ledger in the client's statutory dues \
classified as TDS payable, counted before that TDS."
            ),
            ev_interest.clone(),
        )?;
        // s.194A coverage, read per loan: the TDS-payable lines on every voucher that posts to the
        // loan. A voucher that also posts to another configured loan counts for neither; a nil line
        // on the other loan is not a posting (bridge#1201; the opposite-loan test gives it no side
        // either), and a nil line on this loan is not one either (bridge#1259: with a nil line on
        // each of two loans, one deduction was read as on both).
        let tds_by_voucher: Vec<(usize, i128)> = pop
            .iter()
            .enumerate()
            .filter(|(_, v)| {
                v.base_type != "Contra"
                    && v.lines
                        .iter()
                        .any(|l| l.ledger == *loan_ledger && l.amount_paise != 0)
                    && !v
                        .lines
                        .iter()
                        .any(|l| other_loans.contains(l.ledger.as_str()) && l.amount_paise != 0)
            })
            .map(|(at, v)| (at, -net_on(v, &tds_here)))
            .filter(|&(_, x)| x != 0)
            .collect();
        let tds_on_loan: i128 = tds_by_voucher.iter().map(|&(_, x)| x).sum();
        let ev_tds = voucher_refs(tds_by_voucher.iter().map(|&(at, _)| pop[at]));
        let f_tds = r.fig(
            &format!("tds_on_loan_{h}"),
            paise(tds_on_loan)?,
            Unit::Paise,
            &format!(
                "TDS deducted on loan ledger (tag {h}): the lines on ledgers in the client's \
statutory dues classified as TDS payable, on every voucher that posts to the loan."
            ),
            ev_tds.clone(),
        )?;
        // A voucher counted in taken or repaid already holds its TDS in its loan line (Loan Dr
        // 10,000 / TDS Cr 1,000 / Bank Cr 9,000 is a repayment of 10,000): LOAN-1 adds
        // tds_on_loan for the deductions outside those totals, so it is told what to leave out.
        // Published only where such a voucher carries TDS (bridge#1259 item 2). A voucher listed
        // as both crediting and debiting the loan is not read this way: LOAN-1 firing on one
        // carrying TDS stays.
        let in_principal: BTreeSet<usize> = rows
            .taken
            .iter()
            .chain(&rows.repaid)
            .map(|x| x.at)
            .collect();
        let principal_tds: Vec<(usize, i128)> = tds_by_voucher
            .iter()
            .copied()
            .filter(|(at, _)| in_principal.contains(at))
            .collect();
        if !principal_tds.is_empty() {
            r.fig(
                &format!("tds_in_principal_{h}"),
                paise(principal_tds.iter().map(|&(_, x)| x).sum())?,
                Unit::Paise,
                &format!(
                    "Of the TDS deducted on loan ledger (tag {h}), the part on the vouchers counted \
in its taken or repaid totals (a repayment booked net of TDS, say), summed. It is inside the TDS \
figure beside it, and is shown so the loan's movement can be checked against the Trial Balance."
                ),
                voucher_refs(principal_tds.iter().map(|&(at, _)| pop[at])),
            )?;
        }
        // (c) of #779 Phase A: a listed voucher carrying the loan's interest ledger or a TDS ledger
        // leaves its interest out of interest_total and its TDS in tds_on_loan. The threshold is
        // open where the interest crosses it in some reading of those interest lines and not in
        // others (below); coverage only where TDS is seen on the loan -- with none, coverage is
        // "none" in every reading (a certain default stays).
        let listed_194a: Vec<&Voucher> = listed
            .iter()
            .copied()
            .filter(|v| {
                v.lines.iter().any(|l| {
                    l.amount_paise != 0 && (ils.contains(&l.ledger) || tds_here.contains(&l.ledger))
                })
            })
            .collect();
        // The threshold is read line by line (6c2d6be2): the least interest is the total with every
        // listed reversal line, the most the total with every listed credit line, so a listed pair
        // netting to nil still spans both.
        let listed_lines: Vec<i128> = listed_194a
            .iter()
            .flat_map(|v| v.lines.iter())
            .filter(|l| ils.contains(&l.ledger))
            .map(|l| i128::from(l.amount_paise))
            .collect();
        let interest_listed: i128 = listed_lines.iter().sum();
        let interest_least = interest_total + listed_lines.iter().filter(|x| **x < 0).sum::<i128>();
        let interest_most = interest_total + listed_lines.iter().filter(|x| **x > 0).sum::<i128>();
        let over_without = interest_total > threshold_194a;
        let threshold_open = !listed_194a.is_empty()
            && (interest_least > threshold_194a) != (interest_most > threshold_194a);
        let coverage_open = !listed_194a.is_empty() && tds_on_loan != 0;
        let s194a_open = threshold_open || (coverage_open && over_without);
        let ev_listed_194a = voucher_refs(listed_194a.iter().copied());
        let f_listed_interest = if listed_194a.is_empty() {
            None
        } else {
            Some(r.fig(
                &format!("interest_on_listed_vouchers_{h}"),
                paise(interest_listed)?,
                Unit::Paise,
                &format!(
                    "The lines on loan ledger (tag {h})'s interest ledger(s) in the vouchers listed \
as both crediting and debiting the loan, summed (debits positive): shown beside the interest total, \
never added to it."
                ),
                ev_listed_194a.clone(),
            )?)
        };
        // The date-aware count, which the by-date limit states.
        let mut covering_read: i128 = 0;
        let coverage = match rate_194a {
            _ if coverage_open => "not computed",
            None => "not judged",
            Some(rate) => {
                let rate = i128::from(rate);
                let expected = interest_total
                    .max(0)
                    .checked_mul(rate)
                    .and_then(|x| x.checked_add(5000))
                    .ok_or_else(|| overflow(TEST_ID))?
                    .div_euclid(10_000);
                r.fig(
                    &format!("s194a_tds_expected_{h}"),
                    paise(expected)?,
                    Unit::Paise,
                    &format!(
                        "TDS at the s.194A rate ({rate} basis points) on loan ledger (tag {h})'s \
interest."
                    ),
                    Vec::new(),
                )?;
                // Date-aware: a deduction covers only interest credited on or before its date.
                // Amounts in paise x 10,000 so the rate introduces no rounding; each voucher's
                // interest and TDS move together; same day, vouchers crediting interest first.
                let mut per_voucher: Vec<(usize, i128, i128)> = Vec::new(); // (at, need, tds)
                let mut slot = |at: usize, need: i128, tds: i128| {
                    if let Some(e) = per_voucher.iter_mut().find(|e| e.0 == at) {
                        e.1 += need;
                        e.2 += tds;
                    } else {
                        per_voucher.push((at, need, tds));
                    }
                };
                for &(at, amt) in &rows.interest {
                    let gross = -(amt + net_on(pop[at], &tds_here));
                    slot(
                        at,
                        gross.checked_mul(rate).ok_or_else(|| overflow(TEST_ID))?,
                        0,
                    );
                }
                for &(at, x) in &tds_by_voucher {
                    slot(
                        at,
                        0,
                        x.checked_mul(10_000).ok_or_else(|| overflow(TEST_ID))?,
                    );
                }
                per_voucher.sort_by(|a, b| {
                    (&pop[a.0].date, u8::from(a.1 <= 0)).cmp(&(&pop[b.0].date, u8::from(b.1 <= 0)))
                });
                let (mut required, mut usable) = (0_i128, 0_i128);
                for &(_, need, tds) in &per_voucher {
                    required = required
                        .checked_add(need)
                        .ok_or_else(|| overflow(TEST_ID))?;
                    usable = usable
                        .checked_add(tds)
                        .ok_or_else(|| overflow(TEST_ID))?
                        .min(required)
                        .max(0);
                }
                let covering = usable.div_euclid(10_000);
                covering_read = covering;
                r.fig(
                    &format!("s194a_tds_covering_{h}"),
                    paise(covering)?,
                    Unit::Paise,
                    &format!(
                        "The TDS on loan ledger (tag {h}) that covers its interest: each deduction \
counts only against interest credited on or before its own date, up to the s.194A rate on that \
interest."
                    ),
                    ev_tds.clone(),
                )?;
                let deductions = tds_by_voucher.iter().filter(|&&(_, x)| x > 0).count();
                // a rupee of rounding per deduction (never per reversal)
                let tolerance =
                    100 * i128::try_from(deductions.max(1)).map_err(|_| overflow(TEST_ID))?;
                // The TDS seen can reach the rate by amount and not by date: a deduction at payment
                // before a later credit of the interest, or one for an earlier year's interest; the
                // books do not say which. Named as such, and still listed; "partly covered" is kept
                // for a shortfall in amount.
                if tds_on_loan > 0 && covering >= expected - tolerance {
                    "covered"
                } else if tds_on_loan >= expected - tolerance && expected - tolerance > 0 {
                    "covered by amount, not by date"
                } else if tds_on_loan > 0 {
                    "partly covered"
                } else {
                    "none"
                }
            }
        };
        r.fig(
            &format!("s194a_tds_coverage_{h}"),
            text(coverage),
            Unit::Text,
            &format!(
                "Whether the TDS on loan ledger (tag {h}) covers the s.194A rate on its interest: \
covered, partly covered, covered by amount but not by date (the TDS seen reaches the rate, while the \
date-aware count, reversals included, credits less), none, not judged where the rules carry no \
s.194A rate, or not computed where a voucher listed as both crediting and debiting the loan carries \
interest or TDS and TDS is seen on the loan. The rate assumes the lender furnished a PAN: s.206AA's \
higher rate is not applied, and the books do not show a PAN."
            ),
            Vec::new(),
        )?;
        let tds_seen_ledgers: BTreeSet<&str> = rows
            .interest
            .iter()
            .flat_map(|&(at, _)| pop[at].lines.iter())
            .filter(|l| tds_here.contains(&l.ledger))
            .map(|l| l.ledger.as_str())
            .collect();
        if !tds_seen_ledgers.is_empty() {
            r.fig(
                &format!("interest_tds_ledgers_{h}"),
                text(&tds_seen_ledgers.into_iter().collect::<Vec<_>>().join("\n")),
                Unit::Text,
                &format!(
                    "The ledgers in the client's statutory dues classified as TDS payable that \
appear on loan ledger (tag {h})'s interest journals -- this test's own consistency checks read them \
back to tie the interest to the loan's lines."
                ),
                Vec::new(),
            )?;
        }
        r.fig(
            &format!("interest_ledger_{h}"),
            text(
                &ils.iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            Unit::Text,
            &format!(
                "The configured interest ledger name(s) for loan ledger (tag {h}), one per line, \
or '' if the loan is interest-free -- this test's own consistency checks read them back to check \
each voucher on this loan and its interest ledger(s) independently of the test's own bucketing.{}",
                if ils.len() > 1 {
                    " Several are named, one per line: naming a ledger here is the CA's \
classification of it as interest (s.2(28A): interest includes any service fee or other charge in \
respect of the money borrowed); it is never inferred from the books."
                } else {
                    ""
                }
            ),
            Vec::new(),
        )?;
        let both = |side: &str| {
            if listed.is_empty() {
                String::new()
            } else {
                format!(
                    " It includes the {side} side of each voucher listed as both crediting and \
debiting the loan."
                )
            }
        };
        r.fig(
            &format!("clause31_taken_total_{h}"),
            paise(taken_total)?,
            Unit::Paise,
            &format!(
                "Credits to loan ledger (tag {h}) other than its interest journals -- amount \
taken/accepted in the year, before any Clause 31/s.269SS lender-exemption or mode filter.{}",
                both("credit")
            ),
            Vec::new(),
        )?;
        r.fig(
            &format!("clause31_repaid_total_{h}"),
            paise(repaid_total)?,
            Unit::Paise,
            &format!(
                "Debits to loan ledger (tag {h}) other than its interest journals -- amount repaid \
in the year, before any Clause 31/s.269T lender-exemption or mode filter.{}",
                both("debit")
            ),
            Vec::new(),
        )?;
        let mut listed_vouchers: Vec<ListedVoucher> = Vec::new();
        for (v, &(credit, debit)) in listed.iter().copied().zip(&sides) {
            let vh = hash8(&v.guid);
            let vid = format!("{h}_{vh}");
            let gross_credit = r.fig(
                &format!("two_sided_gross_credit_{vid}"),
                paise(credit)?,
                Unit::Paise,
                &format!(
                    "The credit lines on loan ledger (tag {h}) in this voucher (tag {vh}), summed: \
the voucher also debits the loan."
                ),
                vec![voucher_ref(v)],
            )?;
            let gross_debit = r.fig(
                &format!("two_sided_gross_debit_{vid}"),
                paise(debit)?,
                Unit::Paise,
                &format!(
                    "The debit lines on loan ledger (tag {h}) in this voucher (tag {vh}), summed: \
the voucher also credits the loan."
                ),
                vec![voucher_ref(v)],
            )?;
            let mut facts = vec![
                ("gross_credit".to_string(), gross_credit),
                ("gross_debit".to_string(), gross_debit),
            ];
            let imbalance: i128 = v.lines.iter().map(|l| i128::from(l.amount_paise)).sum();
            if imbalance != 0 {
                let id = r.fig(
                    &format!("two_sided_imbalance_{vid}"),
                    paise(imbalance)?,
                    Unit::Paise,
                    &format!(
                        "This voucher's lines (tag {vh}) do not sum to zero: the difference, debits \
positive."
                    ),
                    vec![voucher_ref(v)],
                )?;
                facts.push(("imbalance".to_string(), id));
            }
            listed_vouchers.push(ListedVoucher {
                voucher: v,
                credit,
                debit,
                imbalance,
                facts,
            });
        }
        let ev_listed = voucher_refs(listed.iter().copied());
        if !listed.is_empty() {
            r.fig(
                &format!("clause31_not_computed_credits_{h}"),
                paise(listed_credits)?,
                Unit::Paise,
                &format!(
                    "The credit sides of the vouchers listed as both crediting and debiting loan \
ledger (tag {h}), summed: in the loan's credits before any filter, in no reportable total."
                ),
                ev_listed.clone(),
            )?;
            r.fig(
                &format!("clause31_not_computed_debits_{h}"),
                paise(listed_debits)?,
                Unit::Paise,
                &format!(
                    "The debit sides of the vouchers listed as both crediting and debiting loan \
ledger (tag {h}), summed: in the loan's debits before any filter, in no reportable total."
                ),
                ev_listed.clone(),
            )?;
        }

        // FAR defect 5: an entry whose other lines are expense ledgers only, besides TDS, and not
        // all another loan's interest ledger or a shared one (that is the misposted question's).
        let charge_shaped = |v: &Voucher| -> bool {
            let counter: Vec<&str> = others_than(v, loan_ledger)
                .into_iter()
                .filter(|c| !tds_here.contains(*c))
                .collect();
            !counter.is_empty()
                && !counter.iter().all(|c| foreign_interest.contains(*c))
                && counter.iter().all(|c| {
                    book.ledgers
                        .get(*c)
                        .is_some_and(|l| EXPENSE_GROUPS.iter().any(|g| l.under(g)))
                })
        };
        let charge_taken: Vec<&PrincipalRow> = rows
            .taken
            .iter()
            .filter(|x| charge_shaped(x.voucher))
            .collect();
        let charge_entries = charge_taken.len()
            + rows
                .repaid
                .iter()
                .filter(|x| charge_shaped(x.voucher))
                .count();
        if charge_entries > 0 {
            r.fig(
                &format!("clause31_expense_entry_count_{h}"),
                count(TEST_ID, charge_entries)?,
                Unit::Count,
                &format!(
                    "Entries on loan ledger (tag {h}) whose other lines are expense ledgers only, \
besides any TDS: each is read as a loan taken or repaid, not as interest. Clause 31 lists it, asking \
whether it is the lender's charge (or its reversal), where the lender and the running balance, or \
its own amount, make it reportable; an entry whose reportability is not computed is listed as not \
computed instead, without that question."
                ),
                Vec::new(),
            )?;
        }
        let pairs = reversal_pairs(&rows.taken, &rows.repaid, inputs.bank);
        // Each paired voucher's partner, and the numbers matched (none for an identical narration).
        let mut partner: HashMap<usize, (usize, Vec<String>)> = HashMap::new();
        for (di, ci, shared) in &pairs {
            partner.insert(rows.repaid[*di].at, (rows.taken[*ci].at, shared.clone()));
        }
        for (di, ci, shared) in &pairs {
            partner.insert(rows.taken[*ci].at, (rows.repaid[*di].at, shared.clone()));
        }
        if !pairs.is_empty() {
            r.fig(
                &format!("clause31_reversal_pair_count_{h}"),
                count(TEST_ID, pairs.len())?,
                Unit::Count,
                &format!(
                    "Pairs on loan ledger (tag {h}) of a bank debit and a bank credit of one \
amount, 0 to {REVERSAL_PAIR_MAX_DAYS} days apart, with an identical narration, with a number of six \
or more digits in it, that no other taken or repaid entry of the loan carries; or sharing a number \
of six or more digits, in the narration or the voucher's reference, that no other taken or repaid \
entry of the loan carries. Each is named as the other's possible reversal wherever clause 31 lists \
it (where the lender and the running balance, or its own amount, make it reportable, or as not \
computed where that is not decided); nothing is netted."
                ),
                voucher_refs(
                    pairs
                        .iter()
                        .flat_map(|(di, ci, _)| [rows.repaid[*di].voucher, rows.taken[*ci].voucher]),
                ),
            )?;
        }
        // F8c: entries in one direction sharing an identical narration with a number of six or
        // more digits in it, a bank ledger among their other lines, are named to each other.
        let mut by_narration: Vec<((&str, String), Vec<usize>)> = Vec::new();
        for (direction, list) in [("repaid", &rows.repaid), ("taken", &rows.taken)] {
            for x in list {
                let narration = normalised(&x.voucher.narration);
                if !bank_numbers(&narration).is_empty()
                    && x.counter.iter().any(|l| inputs.bank.contains(*l))
                {
                    let k = (direction, narration);
                    match by_narration.iter_mut().find(|(key, _)| *key == k) {
                        Some((_, vs)) => vs.push(x.at),
                        None => by_narration.push((k, vec![x.at])),
                    }
                }
            }
        }
        let mut repeats: HashMap<usize, Vec<usize>> = HashMap::new();
        for (_, vs) in by_narration.iter().filter(|(_, vs)| vs.len() >= 2) {
            for &v in vs {
                repeats.insert(v, vs.iter().copied().filter(|o| *o != v).collect());
            }
        }
        if !repeats.is_empty() {
            r.fig(
                &format!("clause31_repeated_narration_count_{h}"),
                count(TEST_ID, repeats.len())?,
                Unit::Count,
                &format!(
                    "Bank entries on loan ledger (tag {h}) sharing, in one direction, an identical \
narration with a number of six or more digits in it. Each is named to the others wherever clause 31 \
lists it; nothing is netted."
                ),
                voucher_refs(
                    by_narration
                        .iter()
                        .filter(|(_, vs)| vs.len() >= 2)
                        .flat_map(|(_, vs)| vs.iter().map(|&at| pop[at])),
                ),
            )?;
        }
        let f_charges = if charge_taken.is_empty() {
            None
        } else {
            Some(r.fig(
                &format!("expense_credits_{h}"),
                paise(charge_taken.iter().map(|x| x.amount).sum())?,
                Unit::Paise,
                &format!(
                    "Credits to loan ledger (tag {h}) on entries whose other lines are expense \
ledgers only, besides any TDS: the loan's own side of each, so net of any TDS on it. The lender's \
charge or a loan taken, not judged; not added into the interest total, and a reversal to an expense \
ledger is not netted against them."
                ),
                voucher_refs(charge_taken.iter().map(|x| x.voucher)),
            )?)
        };

        if !rows.misposted.is_empty() {
            // R2: an entry against only another loan's interest ledger, or a shared one, may be
            // interest booked to the wrong ledger, or principal: the books do not say.
            let ev_mis = voucher_refs(rows.misposted.iter().map(|&(at, _)| pop[at]));
            let f_mis = r.fig(
                &format!("possible_misposted_interest_{h}"),
                paise(-rows.misposted.iter().map(|&(_, a)| a).sum::<i128>())?,
                Unit::Paise,
                &format!(
                    "Net credit to loan ledger (tag {h}) on its taken/repaid entries whose only \
other lines are an interest ledger configured for another loan or declared shared, and ledgers in \
the client's statutory dues classified as TDS payable: each is read as a loan taken or repaid, and \
clause 31 lists it where the lender and the running balance, or its own amount, make it reportable, \
or as not computed where that is not decided."
                ),
                ev_mis.clone(),
            )?;
            let asked: BTreeSet<&str> = rows
                .misposted
                .iter()
                .filter(|&&(_, a)| {
                    !ex.reporting_for(if a < 0 { "taken" } else { "repaid" })
                        .contains(lender_type)
                })
                .map(|&(_, a)| if a < 0 { "3CD-31(a)" } else { "3CD-31(c)" })
                .collect();
            if !asked.is_empty() {
                let booked_to: BTreeSet<&str> = rows
                    .misposted
                    .iter()
                    .flat_map(|&(at, _)| pop[at].lines.iter())
                    .filter(|l| foreign_interest.contains(&l.ledger))
                    .map(|l| l.ledger.as_str())
                    .collect();
                let tags = booked_to
                    .iter()
                    .map(|x| stable_ledger_tag(book, x))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                let k = rows.misposted.len();
                let mut evidence = ev_mis;
                evidence.push(EvidenceRef::new("ledger", loan_ledger));
                r.findings.push(Finding {
                    id: format!("{TEST_ID}/possible_misposted_interest/{h}"),
                    clauses: asked.iter().map(|c| (*c).to_string()).collect(),
                    title: "Loan entries booked against another loan's interest ledger, or a \
shared one: read as loans taken or repaid; whether they are interest is the CA's to determine"
                        .to_string(),
                    facts: vec![
                        ("possible_misposted".to_string(), f_mis),
                        ("lender_type".to_string(), f_lender_type.clone()),
                    ],
                    evidence,
                    confidence: Confidence::JudgementRequired,
                    limits: vec![
                        format!(
                            "{k} entr{} on this loan ledger booked against only an interest ledger \
configured for another loan or declared shared (tag {tags}), besides any TDS lines: read as a loan \
taken or repaid, with clause 31(a)/(c) and the s.269SS/269T test applied as to any other entry.",
                            if k == 1 { "y" } else { "ies" }
                        ),
                        "If an entry is interest to this lender booked to the wrong interest \
ledger, it is not a loan taken or repaid: it would leave clause 31, and a s.269SS/269T flag on it \
would not apply. The books do not show which; the CA determines."
                            .to_string(),
                    ],
                    ask_client: vec![format!(
                        "For each entry named, whether it is interest to {lender} or a loan taken \
or repaid."
                    )],
                });
            }
        }

        let opening_outstanding: i128 = book
            .tb
            .get(loan_ledger)
            .map_or(0, |t| -i128::from(t.opening_paise));
        let opening_clipped = opening_outstanding < 0;
        let running = compute_running_balance_rows(&pop, &rows, opening_outstanding);
        let max_outstanding = running
            .iter()
            .map(|row| row.after_outstanding)
            .fold(opening_outstanding.max(0), i128::max);
        let walked_below_zero = running
            .iter()
            .any(|row| row.direction == "taken" && row.prior_outstanding < 0);
        let max_outstanding_w2 = running
            .iter()
            .map(|row| row.after_outstanding_w2)
            .fold(opening_outstanding.max(0), i128::max);
        if listed.is_empty() {
            r.fig(
                &format!("max_outstanding_paise_{h}"),
                paise(max_outstanding)?,
                Unit::Paise,
                &format!(
                    "Running maximum of the outstanding balance owed to the lender on loan ledger (tag \
{h}) during the year (opening balance, then after every taken/repaid entry in date order) -- the \
utility's MaxAmtOsAccPy column.{}{}{}",
                    if opening_clipped {
                        " The TB opening on this ledger is a debit balance; walked from 0, not a \
negative outstanding -- confirm the opening figure with the client."
                    } else {
                        ""
                    },
                    if walked_below_zero {
                        " A loan taken on this ledger was walked from a principal balance below zero, \
so this maximum may not be the amount outstanding with the lender: confirm."
                    } else {
                        ""
                    },
                    if max_outstanding_w2 == max_outstanding {
                        String::new()
                    } else {
                        format!(
                            " Read with every principal credit after the principal balance went below \
zero as a fresh loan, the maximum is {}; the books do not show which reading holds: confirm.",
                            rupees(max_outstanding_w2)
                        )
                    }
                ),
                Vec::new(),
            )?;
        } else {
            r.fig(
                &format!("max_outstanding_not_computed_{h}"),
                text(
                    "not computed: a voucher both credits and debits this loan ledger, and this \
test does not divide it into entries, so the balance from its date on is not known",
                ),
                Unit::Text,
                &format!(
                    "The running maximum of the outstanding balance on loan ledger (tag {h}) -- the \
utility's MaxAmtOsAccPy column -- is not computed on a loan with a voucher listed as both crediting \
and debiting it (#779)."
                ),
                ev_listed.clone(),
            )?;
        }

        // ---------------------------------------------------------------- s.194A TDS
        if s194a_open && !ex.s194a.contains(lender_type) && matches!(status, "deductor" | "unknown")
        {
            // Kind 4 (#779 Phase A (c)): one record in place of both s.194A findings, outside
            // s194a/ and with no "interest" fact, so clause 21(b) never sums it.
            let tds_on_listed = listed_194a
                .iter()
                .flat_map(|v| v.lines.iter())
                .any(|l| tds_here.contains(&l.ledger) && l.amount_paise != 0);
            let mut evidence = ev_listed_194a.clone();
            evidence.push(EvidenceRef::new("ledger", loan_ledger));
            r.findings.push(Finding {
                id: format!("{TEST_ID}/not_computed/s194a_{h}"),
                clauses: ["s.194A", "3CD-21(b)", "3CD-34(a)", "3CD-34(c)"]
                    .map(str::to_string)
                    .to_vec(),
                title: format!(
                    "Interest to {lender}: {}",
                    if threshold_open {
                        "whether it crosses the s.194A threshold is not computed -- a voucher that \
both credits and debits the loan carries interest"
                    } else {
                        "whether the TDS seen covers it under s.194A is not computed -- a voucher \
that both credits and debits the loan carries interest or TDS"
                    }
                ),
                facts: vec![
                    ("interest_total".to_string(), f_int.clone()),
                    (
                        "interest_on_listed_vouchers".to_string(),
                        f_listed_interest.clone().unwrap_or_default(),
                    ),
                    ("tds_on_loan".to_string(), f_tds.clone()),
                    ("lender_type".to_string(), f_lender_type.clone()),
                ],
                evidence,
                confidence: Confidence::JudgementRequired,
                limits: vec![
                    format!(
                        "{} voucher(s) both credit and debit this loan and carry its interest \
ledger or a TDS ledger; this test lists each as the books hold it and does not divide it into \
entries. Their interest ({}) is shown beside the interest total, not in it; any TDS on them is in \
the TDS seen on the loan, unless the voucher also posts to another configured loan. {} This loan is \
in no clause 21(b) item from this test.",
                        listed_194a.len(),
                        rupees(interest_listed),
                        if threshold_open {
                            format!(
                                "The interest crosses the s.194A threshold ({}) {}, so whether it \
crosses is not computed.",
                                rupees(threshold_194a),
                                if interest_least == interest_total {
                                    "with that interest and not without it".to_string()
                                } else if interest_most == interest_total {
                                    "without that interest and not with it".to_string()
                                } else {
                                    format!(
                                        "in some readings of the interest lines on those vouchers \
and not in others (from {} to {})",
                                        rupees(interest_least),
                                        rupees(interest_most)
                                    )
                                }
                            )
                        } else {
                            format!(
                                "The interest crosses the s.194A threshold ({}) with or without it, \
but whether the TDS seen covers it is not computed: {}",
                                rupees(threshold_194a),
                                if tds_on_listed {
                                    "part of the TDS may be on those vouchers (unless each also \
posts to another configured loan, whose TDS is not counted here), and which interest it covers is \
not read."
                                } else {
                                    "the interest on those vouchers is not in the interest total, \
so the TDS the rate requires is not known exactly."
                                }
                            )
                        }
                    ),
                    "s.40(a)(ia) disallows 30% of the interest on a TDS default; the second proviso \
removes this if the lender's Form 26A (Rule 31ACB) shows the interest was returned as income."
                        .to_string(),
                ],
                ask_client: vec![format!(
                    "The interest credited to {lender} for the year, the TDS deducted on it under \
s.194A, and the challans showing it was deposited."
                )],
            });
        }
        let over_194a = !s194a_open
            && !ex.s194a.contains(lender_type)
            && interest_total > threshold_194a
            && (status == "deductor" || status == "unknown");
        if over_194a && coverage != "covered" {
            n.tds_over_threshold_count += 1;
            let partly = tds_on_loan > 0;
            let mut facts = vec![
                ("interest".to_string(), f_int.clone()),
                ("lender_type".to_string(), f_lender_type.clone()),
            ];
            let mut limits = Vec::new();
            if status != "deductor" {
                limits.push(
                    "Whether the assessee must deduct under s.194A is not known (see the \
deductor-status question): listed until it is confirmed."
                        .to_string(),
                );
            }
            limits.push(format!(
                "Books only: lender type ({}) comes from the client's setup, not from a \
notification lookup; confirm the lender is not itself a body notified as exempt under \
s.194A(3)(iii) beyond the lender types the rules already exempt.",
                py_repr_str(lender_type)
            ));
            limits.push(
                "s.40(a)(ia) disallows 30% of the interest on TDS default; the second proviso \
removes this if the lender's Form 26A (Rule 31ACB) shows the interest was returned as income -- \
s.201(1A) interest still runs either way."
                    .to_string(),
            );
            let by_date = coverage == "covered by amount, not by date";
            if by_date {
                facts.push(("tds_on_loan".to_string(), f_tds.clone()));
                limits.push(format!(
                    "TDS of {} is seen on this loan, at the s.194A rate on its interest by amount, \
but the date-aware count (each deduction against the interest credited on or before its date, \
reversals included) credits only {} of it. The difference may be a deduction at payment before a \
later credit of the interest (s.194A: at credit or payment, whichever is earlier), a deduction in \
excess on an earlier credit, a deduction later reversed, or one for an earlier year's interest: the \
books do not say which. The whole interest is listed until the CA determines which.",
                    rupees(tds_on_loan),
                    rupees(covering_read)
                ));
            } else if partly {
                facts.push(("tds_on_loan".to_string(), f_tds.clone()));
                limits.push(format!(
                    "Partly covered: TDS of {} is seen on this loan, short of the s.194A rate on \
its interest. The whole interest is listed until the treatment of a short deduction (Form 3CD \
clause 34(a), and clause 21(b) only as a flag) is settled.",
                    rupees(tds_on_loan)
                ));
            }
            if coverage == "not judged" {
                limits.push(
                    "The rules carry no s.194A rate, so whether the TDS seen on this loan covers \
its interest is not judged."
                        .to_string(),
                );
            }
            if let Some(f) = &f_listed_interest {
                facts.push(("interest_on_listed_vouchers".to_string(), f.clone()));
                limits.push(format!(
                    "{} voucher(s) both credit and debit this loan and carry its interest ledger \
or a TDS ledger; the interest on them ({}) is not in the amount above, which crosses the s.194A \
threshold with or without it. No TDS is counted on this loan (TDS on a voucher that also posts to \
another configured loan is not counted), so none of its interest is covered in either reading; the \
listed voucher(s) are named on their own records.",
                    listed_194a.len(),
                    rupees(interest_listed)
                ));
            }
            if let Some(f) = &f_charges {
                facts.push(("expense_credits".to_string(), f.clone()));
                limits.push(
                    "Credits to this loan from expense ledgers only are not in the interest above \
(they are shown net of any TDS on them; the interest is gross). If one is the lender's charge, it \
is interest (s.2(28A)) and belongs in it. Until its ledger is named among the loan's interest \
ledgers it is read as a loan taken, and clause 31 lists it where the lender and the running \
balance, or its own amount, make it reportable, or as not computed where that is not decided."
                        .to_string(),
                );
            }
            let mut evidence = ev_interest.clone();
            evidence.push(EvidenceRef::new("ledger", loan_ledger));
            r.findings.push(Finding {
                id: format!("{TEST_ID}/s194a/{h}"),
                clauses: ["s.194A", "3CD-21(b)", "3CD-34(a)", "3CD-34(c)"]
                    .map(str::to_string)
                    .to_vec(),
                title: if by_date {
                    "Interest to a non-exempt lender over the s.194A threshold, TDS on the loan \
at the s.194A rate by amount, short of it by the date-aware count"
                } else if partly {
                    "Interest to a non-exempt lender over the s.194A threshold, TDS on the loan \
short of the s.194A rate"
                } else {
                    "Interest to a non-exempt lender over the s.194A threshold, no TDS ledger \
evidence"
                }
                .to_string(),
                facts,
                evidence,
                confidence: Confidence::NeedsDocument,
                limits,
                ask_client: vec![
                    if partly {
                        format!(
                            "Confirm the TDS deducted under s.194A on interest to {lender} and the \
interest it covers."
                        )
                    } else {
                        format!(
                            "Confirm whether TDS under s.194A was deducted on interest to {lender}."
                        )
                    },
                    "If not deducted, confirm whether Form 26A is available for this lender."
                        .to_string(),
                ],
            });
        }
        if over_194a && coverage == "covered" {
            // Covered is never silent: the CA sees what was read as covering the interest.
            let mut evidence = ev_tds.clone();
            evidence.push(EvidenceRef::new("ledger", loan_ledger));
            r.findings.push(Finding {
                id: format!("{TEST_ID}/s194a_tds_seen/{h}"),
                clauses: vec!["s.194A".to_string()],
                title:
                    "TDS seen on the loan covers the s.194A rate on its interest; not listed in \
clause 21(b)"
                        .to_string(),
                facts: vec![
                    ("interest".to_string(), f_int.clone()),
                    ("tds_on_loan".to_string(), f_tds.clone()),
                    ("lender_type".to_string(), f_lender_type.clone()),
                ],
                evidence,
                confidence: Confidence::JudgementRequired,
                limits: [
                    "Not checked: that the TDS was deposited by the due date (clause 21(b) also \
covers tax deducted but not paid by the s.139(1) due date).",
                    "Not checked: the period each deduction relates to, beyond its date (a \
deduction counts only against interest credited on or before it).",
                    "Not checked: the lender's PAN. The s.194A(1) rate is used; s.206AA's higher \
rate is not applied.",
                    "Not counted: a deduction, or its reversal, on a voucher that also posts to \
another loan (the books do not split it between them).",
                    "Not checked: that each deduction is s.194A TDS on this interest; TDS-payable \
lines on any voucher posting to this ledger count, including TDS under another section on another \
credit to it (rent, commission).",
                ]
                .map(str::to_string)
                .to_vec(),
                ask_client: vec![format!(
                    "Confirm the TDS deducted on interest to {lender}, the period it covers, and \
the challans showing it was deposited by the due date."
                )],
            });
        }

        // ---------------------------------------------------------------- Clause 31(a)/(c)
        if ex.reporting_accepted.contains(lender_type) && ex.reporting.contains(lender_type) {
            continue; // outside Clause 31 form reporting entirely (31-13)
        }
        // #779 Phase A: each voucher both crediting and debiting the loan is listed (kind 1); from
        // the first one's date on, an entry is tested on its own amount only, and one below the
        // limit is listed as depending on it (kind 3).
        let first_listed: Option<&TallyDate> = listed.iter().map(|v| &v.date).min();
        let mut order: Vec<usize> = (0..listed.len()).collect();
        order.sort_by(|&x, &y| {
            (&listed[x].date, &listed[x].guid).cmp(&(&listed[y].date, &listed[y].guid))
        });
        let listed_sorted: Vec<&Voucher> = order.iter().map(|&i| listed[i]).collect();
        let not_compared = if listed.is_empty() {
            String::new()
        } else {
            format!(
                "Voucher(s) {} both credit and debit this loan and are listed as not computed: they \
were not compared for reversal pairs or repeated narrations with this loan's other entries.",
                listed_sorted
                    .iter()
                    .map(|v| voucher_label(v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let clauses31: Vec<&str> = [("taken", "3CD-31(a)"), ("repaid", "3CD-31(c)")]
            .into_iter()
            .filter(|(direction, _)| !ex.reporting_for(direction).contains(lender_type))
            .map(|(_, clause)| clause)
            .collect();
        for &i in &order {
            let v = listed[i];
            let mut roles: BTreeMap<(&str, &str), i128> = BTreeMap::new();
            for l in &v.lines {
                if l.ledger == *loan_ledger || l.amount_paise == 0 {
                    continue;
                }
                let ledger = l.ledger.as_str();
                let role = if inputs.cash.contains(ledger) {
                    "cash"
                } else if inputs.bank.contains(ledger) {
                    "bank"
                } else if ils.contains(ledger) {
                    "interest"
                } else if tds_here.contains(ledger) {
                    "tds"
                } else if other_loans.contains(ledger) {
                    "other_loan"
                } else if foreign_interest.contains(ledger) {
                    "other_interest"
                } else if book
                    .ledgers
                    .get(ledger)
                    .is_some_and(|x| EXPENSE_GROUPS.iter().any(|g| x.under(g)))
                {
                    "expense"
                } else {
                    "other"
                };
                let side = if l.amount_paise > 0 {
                    "debits"
                } else {
                    "credits"
                };
                *roles.entry((role, side)).or_insert(0) += i128::from(l.amount_paise).abs();
            }
            let possible_269: Vec<&str> = if ex.s269.contains(lender_type) {
                Vec::new()
            } else {
                [("s.269SS", "debits"), ("s.269T", "credits")]
                    .into_iter()
                    .filter(|(_, side)| {
                        roles.keys().any(|(role, sd)| sd == side && *role != "bank")
                    })
                    .map(|(tag, _)| tag)
                    .collect()
            };
            n.possible_count += usize::from(!possible_269.is_empty());
            n.two_sided_count += 1;
            two_sided_record(
                &mut r,
                loan_ledger,
                lender,
                &h,
                &listed_vouchers[i],
                &clauses31,
                &possible_269,
                &roles,
                Some(&v.date) == first_listed,
            )?;
        }
        let ctx = RowContext {
            loan_ledger,
            lender,
            lender_type,
            h: &h,
            limit_269,
            ex: &ex,
            pop: &pop,
            charge_shaped: &charge_shaped,
            repeats: &repeats,
            partner: &partner,
            first_listed,
            listed_sorted: &listed_sorted,
            not_compared: &not_compared,
        };
        for row in &running {
            clause31_row(&mut r, &ctx, row, &mut n)?;
        }
    }

    totals(&mut r, &n, limit_269)?;
    per_lender(&mut r, book, loans, &ex)?;
    shared_interest(
        &mut r,
        book,
        &pop,
        inputs,
        &interest_vouchers,
        net_reversals,
    )?;
    unlisted_loan_notices(&mut r, book, &pop, loans, inputs)?;
    Ok(r)
}

/// What one loan's clause 31 rows read.
struct RowContext<'a> {
    loan_ledger: &'a str,
    lender: &'a str,
    lender_type: &'a str,
    h: &'a str,
    limit_269: i128,
    ex: &'a Exemptions<'a>,
    pop: &'a [&'a Voucher],
    charge_shaped: &'a dyn Fn(&Voucher) -> bool,
    /// A voucher's population position -> the other entries sharing its narration.
    repeats: &'a HashMap<usize, Vec<usize>>,
    /// A voucher's population position -> its possible reversal partner and the numbers shared.
    partner: &'a HashMap<usize, (usize, Vec<String>)>,
    /// The earliest date of a voucher listed as both crediting and debiting the loan.
    first_listed: Option<&'a TallyDate>,
    /// Those vouchers in (date, GUID) order.
    listed_sorted: &'a [&'a Voucher],
    /// On a loan with such a voucher, the limit saying the entry was not compared with it; else
    /// empty.
    not_compared: &'a str,
}

/// One walked taken/repaid row: listed as not computed, skipped, or a clause 31 finding.
#[allow(clippy::too_many_lines)]
fn clause31_row(r: &mut TestResult, c: &RowContext, row: &WalkedRow, n: &mut Counts) -> Result<()> {
    let direction = row.direction;
    let taken = direction == "taken";
    if c.ex.reporting_for(direction).contains(c.lender_type) {
        return Ok(()); // this direction's note excludes the lender (Government: repayments only)
    }
    let clause = if taken { "3CD-31(a)" } else { "3CD-31(c)" };
    let (v, amt, m) = (row.voucher, row.amount, row.mode);
    let (prior, after) = (row.prior_outstanding, row.after_outstanding);
    let limit = c.limit_269;
    let payee_mode = NON_ACCOUNT_PAYEE_MODES.contains(&m) && !c.ex.s269.contains(c.lender_type);
    // From the first listed voucher's date on, only an entry whose own amount reaches the limit is
    // computed: limb (a) makes it reportable, and flags it, on both walks whatever the balance.
    let after_listed = c.first_listed.is_some_and(|d| v.date >= *d);
    // the listed vouchers dated on or before this entry: the ones its text may name
    let labels_here = c
        .listed_sorted
        .iter()
        .filter(|x| x.date <= v.date)
        .map(|x| voucher_label(x))
        .collect::<Vec<_>>()
        .join(", ");
    if after_listed && amt < limit {
        n.possible_count += usize::from(payee_mode);
        if taken {
            n.listed_taken += 1;
        } else {
            n.listed_repaid += 1;
        }
        return match c.first_listed {
            Some(first) => depends_record(r, c, row, clause, payee_mode, &labels_here, first),
            None => Ok(()),
        };
    }
    // 31-11: the reporting window, principal only; taken tests the larger of the balance after
    // this entry and its own amount, repaid the larger of the balance before it and its amount.
    let crosses_reporting_window = if taken {
        amt.max(after)
    } else {
        prior.max(amt)
    } >= limit;
    // 31-12: the s.269SS/s.269T breach test; repaid counts interest credited and not yet paid.
    let crosses_breach_balance = if taken {
        amt.max(after)
    } else {
        row.prior_breach.max(amt)
    } >= limit;
    // R2: the same tests on the netting walk and on the fresh-loan walk.
    let verdict = |prior_: i128, after_: i128, breach_prior_: i128| {
        let window = if taken {
            amt.max(after_)
        } else {
            prior_.max(amt)
        } >= limit;
        let breach = if taken {
            amt.max(after_)
        } else {
            breach_prior_.max(amt)
        } >= limit;
        (window || breach, payee_mode && breach)
    };
    let verdict_net = verdict(
        row.prior_outstanding_net,
        row.after_outstanding_net,
        row.prior_breach_net,
    );
    let verdict_w2 = verdict(
        row.prior_outstanding_w2,
        row.after_outstanding_w2,
        row.prior_breach_w2,
    );
    if verdict_net.0 != verdict_w2.0 {
        n.possible_count += usize::from(verdict_w2.1);
        if taken {
            n.listed_taken += 1;
        } else {
            n.listed_repaid += 1;
        }
        return refund_or_loan_record(
            r,
            c.loan_ledger,
            c.lender,
            c.h,
            row,
            clause,
            verdict_net,
            verdict_w2,
            (!c.not_compared.is_empty()).then_some(c.not_compared),
        );
    }
    if taken && row.repaid_since_interest {
        // bridge#802, closing form: the same test on the breach balance, an upper bound on the
        // principal still owed if earlier repayments paid interest first; listed only where it
        // alone reaches the limit.
        let verdict_p = verdict(prior, after, row.prior_breach);
        let verdict_b = verdict(prior, row.after_breach, row.prior_breach);
        if verdict_b.0 && !verdict_p.0 {
            n.possible_count += usize::from(verdict_b.1);
            n.listed_taken += 1;
            return interest_first_record(
                r,
                c.loan_ledger,
                c.lender,
                c.h,
                row,
                clause,
                verdict_p,
                verdict_b,
                (!c.not_compared.is_empty()).then_some(c.not_compared),
            );
        }
    }
    let flag_undetermined = verdict_net.1 != verdict_w2.1;
    n.possible_count += usize::from(flag_undetermined);
    n.flag_not_computed_count += usize::from(flag_undetermined);
    if !(crosses_reporting_window || crosses_breach_balance) {
        return Ok(());
    }
    if taken {
        n.taken_reportable_total += amt;
    } else {
        n.repaid_reportable_total += amt;
    }
    let h = c.h;
    let vh = hash8(&v.guid);
    let rid = format!("{direction}_{h}_{vh}");
    let f_amt = r.fig(
        &format!("clause31_row_amount_{rid}"),
        paise(amt)?,
        Unit::Paise,
        &format!("Loan {direction} on voucher (tag {vh}) against loan ledger (tag {h})."),
        vec![voucher_ref(v)],
    )?;
    let f_mode = r.fig(
        &format!("clause31_row_mode_{rid}"),
        text(m),
        Unit::Text,
        &format!(
            "Mode of this {direction} transaction, read from its counter-line ledger group(s): \
cash/bank if any counter-line is under that group, else journal if the voucher's own type is \
Journal, else other."
        ),
        Vec::new(),
    )?;
    let code = mode_code(m, direction);
    r.fig(
        &format!("clause31_row_mode_code_{rid}"),
        text(code),
        Unit::Text,
        &format!(
            "Form 3CD utility Note 1 code for this {direction} entry: derived from the mode and \
direction, not read off a specimen utility export -- confirm."
        ),
        Vec::new(),
    )?;
    if !after_listed {
        r.fig(
            &format!("clause31_row_outstanding_after_{rid}"),
            paise(after)?,
            Unit::Paise,
            &format!(
                "Outstanding balance owed to the lender on loan ledger (tag {h}) immediately after \
this entry (prior balance {prior}p {} this entry's amount) -- the GN 55.8/57.2 running-balance \
walk.",
                if taken { "plus" } else { "minus" }
            ),
            Vec::new(),
        )?;
    }
    let flagged = NON_ACCOUNT_PAYEE_MODES.contains(&m)
        && crosses_breach_balance
        && !c.ex.s269.contains(c.lender_type)
        && !flag_undetermined;
    let mut clauses = vec![clause.to_string()];
    if flagged {
        n.flag_count += 1;
        clauses.push(if taken { "s.269SS" } else { "s.269T" }.to_string());
    }

    let mut limits = vec![
        "Books only: confirm the lender's identity (name, address, PAN) for Form 3CD Clause 31, \
and whether this lender is excepted from s.269SS/269T beyond the lender types the rules already \
except (Government, a notified corporation, or another body notified under the Explanation)."
            .to_string(),
        "The mode-and-direction code follows the Form 3CD utility's own Note 1 list (A/B cash \
payment/receipt, I/J journal entry debit/credit, K/L any other mode debit/credit); this pack's own \
mapping from mode and direction to that code is a derivation from the GN's text, not verified \
against a specimen utility export -- confirm."
            .to_string(),
    ];
    let mut ask_client =
        vec!["Confirm lender identity (name, address, PAN) for Clause 31.".to_string()];
    if c.lender_type == "insurer" {
        limits.push(
            "The lender type is 'insurer': exempt from TDS deduction under s.194A(3)(iii), but \
that is a different exemption from the one in the Explanation to s.269SS/269T -- this lender is NOT \
treated as exempt from Clause 31/s.269SS/s.269T here; confirm."
                .to_string(),
        );
    }
    if m == MODE_CASH {
        limits.push(format!(
            "Mode is read from the counter-line ledger's group only (Cash-in-Hand), never from the \
narration: this voucher's narration is {}. A narration naming an electronic instrument (e.g. \
UPI/PhonePe) booked through a cash ledger takes this outside s.269SS/269T even though the ledger \
group says 'cash' -- confirm from the narration and the bank statement before treating this as a \
cash breach.",
            py_repr_str(&v.narration)
        ));
        ask_client.push(
            "Confirm from narration/bank statement whether this was genuinely cash or an \
electronic transfer narrated against a cash ledger."
                .to_string(),
        );
    }
    let is_charge_shaped = (c.charge_shaped)(v);
    if is_charge_shaped {
        limits.push(format!(
            "This entry is {} an expense ledger (its other lines are expense ledgers only, besides \
any TDS); if it {} the lender's interest or charge (s.2(28A)), name each such ledger among the \
loan's interest ledgers and the entry leaves clause 31. A ledger that also carries other payments \
must then be declared shared, and its other debits are listed as interest attributed to no loan.",
            if taken { "credited from" } else { "reversed to" },
            if taken { "is" } else { "reverses" }
        ));
        ask_client.push(
            if taken {
                "Whether this entry is the lender's interest or charge, or a loan taken."
            } else {
                "Whether this entry reverses the lender's interest or charge, or is a loan repaid."
            }
            .to_string(),
        );
    }
    if let Some(same_as) = c.repeats.get(&row.at).filter(|s| !s.is_empty()) {
        let labels: Vec<String> = same_as.iter().map(|&o| voucher_label(c.pop[o])).collect();
        limits.push(format!(
            "This entry's narration is the same as that of {}, a number of six or more digits \
included: one may book the other a second time, or the bank may have repeated the narration. Each \
is listed here as it stands.",
            labels.join(", ")
        ));
        ask_client.push(format!(
            "The bank statement lines for the entries named, to show whether each was a separate {}",
            if taken { "receipt." } else { "payment." }
        ));
    }
    let partner = c.partner.get(&row.at);
    if let Some((other, shared)) = partner {
        let other = c.pop[*other];
        let gap = days_between(&other.date, &v.date).abs();
        let matched = if shared.is_empty() {
            "carry the same narration, with a number of six or more digits in it,".to_string()
        } else {
            format!(
                "share the number {}, which no other loan taken or repaid entry of this loan \
carries,",
                shared.join(" and ")
            )
        };
        limits.push(format!(
            "This entry and {} {matched} and the same amount, {gap} day{} apart, so one may be the \
return of the other (a debit returned unpaid), or they may be a repayment and a fresh loan. \
Neither is removed from clause 31 by this: each is listed where the lender and the running \
balance, or its own amount, make it reportable, or as not computed where that is not decided. If \
the bank statement shows the debit was returned, neither is a loan repaid or taken.",
            voucher_label(other),
            if gap == 1 { "" } else { "s" }
        ));
        ask_client.push(
            "The bank statement lines for both entries, to show whether the debit was returned."
                .to_string(),
        );
    }
    if flagged && amt < limit {
        limits.push(format!(
            "This entry's own amount ({amt}p) is below the s.269SS/269T limit on its own; it is \
flagged only because the running balance with this lender (GN 55.8/57.2) is at or over the limit \
-- report every entry from where the running balance first reaches the limit until it falls back \
below, not only the large ones."
        ));
    }
    if after_listed {
        limits.push(format!(
            "The balance owed to the lender before and after this entry is not computed: \
voucher(s) {labels_here}, listed as both crediting and debiting this loan, come on or before its \
date. It is reportable, and tested for s.269SS/269T, on its own amount."
        ));
    }
    if !c.not_compared.is_empty() {
        limits.push(c.not_compared.to_string());
    }
    if taken && prior < 0 && !after_listed {
        limits.push(WALKED_BELOW_ZERO_NOTE.to_string());
        ask_client.push(
            "Whether any of this credit to the loan returns an overpayment, and whether earlier \
repayments included interest."
                .to_string(),
        );
    }
    if flag_undetermined {
        limits.push(format!(
            "The s.269SS/269T flag on this entry is not computed. {TWO_WALKS_TEXT} This entry is \
reportable under both and flagged only under the second. It is in the reportable total, and not in \
the s.269SS/269T flag count. {SAME_DAY_TEXT}"
        ));
        ask_client.push(REFUND_OR_LOAN_ASK.to_string());
    }
    if flagged {
        limits.push(
            "s.273B reasonable cause (e.g. banking facilities not available in the area) is a CA \
judgement call on the facts, not a books fact."
                .to_string(),
        );
    }
    if !taken && crosses_breach_balance && !crosses_reporting_window {
        limits.push(
            "This repayment's own principal balance never reaches the ordinary GN 55.8/57.2 \
reporting window on its own; it is reportable here only because the balance held with this lender, \
together with interest credited and not yet paid off, reaches the s.269T limit (GN 57.1/57.4: \
report repayments even below ₹20,000 where the loan plus interest is ₹20,000 or more)."
                .to_string(),
        );
    }

    let asks = match partner {
        Some((other, shared)) => {
            let other = voucher_label(c.pop[*other]);
            if shared.is_empty() {
                format!(
                    ": the same narration and amount as {other} -- {}",
                    if taken {
                        "the return of that debit?"
                    } else {
                        "a returned debit?"
                    }
                )
            } else {
                format!(
                    ": the same amount as {other}, sharing the number {} -- {}",
                    shared.join(" and "),
                    if taken {
                        "the return of that debit, or a fresh loan after a repayment?"
                    } else {
                        "a returned debit, or a repayment and a fresh loan?"
                    }
                )
            }
        }
        _ if is_charge_shaped => if taken {
            ": credited from an expense ledger -- the lender's charge, or a loan taken?"
        } else {
            ": reversed to an expense ledger -- a reversal of the lender's charge, or a loan repaid?"
        }
        .to_string(),
        _ if flagged => if after_listed || (taken && (after < limit || prior < 0)) {
            " -- its own amount at or over the s.269SS/269T limit"
        } else {
            " -- at or over the s.269SS/269T running-balance limit"
        }
        .to_string(),
        _ => String::new(),
    };
    r.findings.push(Finding {
        id: format!("{TEST_ID}/clause31/{rid}"),
        clauses,
        title: format!(
            "Loan {direction} against {} ({m} mode, code {}){asks}{}",
            c.lender,
            if code.is_empty() { "n/a" } else { code },
            if flag_undetermined {
                " (the s.269SS/269T flag is not computed)"
            } else {
                ""
            }
        ),
        facts: vec![("amount".to_string(), f_amt), ("mode".to_string(), f_mode)],
        evidence: vec![voucher_ref(v), EvidenceRef::new("ledger", c.loan_ledger)],
        confidence: Confidence::NeedsDocument,
        limits,
        ask_client,
    });
    Ok(())
}

/// The run-wide totals and counts.
fn totals(r: &mut TestResult, n: &Counts, limit_269: i128) -> Result<()> {
    let listed_note = if n.two_sided_count > 0 {
        LISTED_TOTALS_NOTE
    } else {
        ""
    };
    r.fig(
        "clause31_taken_reportable_total",
        paise(n.taken_reportable_total)?,
        Unit::Paise,
        &format!(
            "Sum of the loans taken that Clause 31(a) reports: entries on a loan from a lender \
outside the form's reporting exemption, from where the running balance with that lender reaches the \
s.269SS/269T limit, or whose own amount does.{}{}",
            listed_note,
            if n.listed_taken > 0 {
                format!(
                    " {} loan(s) taken listed as not computed are not in it.",
                    n.listed_taken
                )
            } else {
                String::new()
            }
        ),
        Vec::new(),
    )?;
    r.fig(
        "clause31_repaid_reportable_total",
        paise(n.repaid_reportable_total)?,
        Unit::Paise,
        &format!(
            "Sum of the repayments that Clause 31(c) reports: entries on a loan from a lender \
outside the form's reporting exemption, where the balance being repaid, with or without the \
interest credited and not yet paid, or the repayment itself reaches the s.269SS/269T limit.{}{}",
            listed_note,
            if n.listed_repaid > 0 {
                format!(
                    " {} repayment(s) listed as not computed are not in it.",
                    n.listed_repaid
                )
            } else {
                String::new()
            }
        ),
        Vec::new(),
    )?;
    let listed = n.listed_taken + n.listed_repaid;
    if listed > 0 {
        r.fig(
            "clause31_not_computed_row_count",
            count(TEST_ID, listed)?,
            Unit::Count,
            "Taken/repaid entries listed as not computed -- because the two walks of the \
principal balance disagree on their reportability, or a loan taken may be reportable if earlier \
repayments paid interest first (bridge#802), or because a voucher listed as both crediting and \
debiting their loan comes on or before their date and their own amount is less than the limit: in \
no reportable total and not in the flag count.",
            Vec::new(),
        )?;
    }
    if n.two_sided_count > 0 {
        r.fig(
            "clause31_two_sided_listed_count",
            count(TEST_ID, n.two_sided_count)?,
            Unit::Count,
            "Vouchers listed as both crediting and debiting a loan ledger, counted once per loan \
ledger they are listed on, as the books hold them: not divided into entries, in no reportable total \
and not in the flag count (#779).",
            Vec::new(),
        )?;
    }
    if n.possible_count > 0 {
        r.fig(
            "s269ss_269t_possible_not_computed_count",
            count(TEST_ID, n.possible_count)?,
            Unit::Count,
            "Entries and vouchers whose s.269SS/269T flag is not computed and may apply: rows the \
two walks of the balance disagree on, which the second would flag; loans taken that may cross the \
limit if earlier repayments paid interest first (bridge#802); entries in cash, journal or other \
mode dated on or after a voucher listed as both crediting and debiting their loan; and such \
vouchers whose other lines include a ledger that is not a bank account. Listed as questions or \
noted on the row, never in the flag count.",
            Vec::new(),
        )?;
    }
    r.fig(
        "s269ss_269t_flag_count",
        count(TEST_ID, n.flag_count)?,
        Unit::Count,
        &format!(
            "Reportable taken/repaid rows in cash/journal/other mode at or over the s.269SS/269T \
limit ({}).{}{}",
            rupees(limit_269),
            if listed > 0 || n.flag_not_computed_count > 0 {
                format!(
                    " {listed} entr(ies) listed as not computed and {} whose flag is not computed \
are not counted.",
                    n.flag_not_computed_count
                )
            } else {
                String::new()
            },
            if n.two_sided_count > 0 {
                format!(
                    " {} voucher(s) listed as both crediting and debiting a loan are not counted.",
                    n.two_sided_count
                )
            } else {
                String::new()
            }
        ),
        Vec::new(),
    )?;
    r.fig(
        "s194a_tds_over_threshold_lender_count",
        count(TEST_ID, n.tds_over_threshold_count)?,
        Unit::Count,
        "Loan ledgers with a non-exempt lender whose interest this year exceeds the s.194A \
threshold, where the assessee is a deductor or its deductor status is not known, and the TDS seen \
does not cover the s.194A rate on it (each listed in clause 21(b)); a loan whose s.194A verdict is \
not computed is not counted.",
        Vec::new(),
    )?;
    Ok(())
}

/// R3: the tests run per loan ledger while the law looks at the lender. A lender on two ledgers,
/// and ledgers with no lender configured, are asked about; nothing is recomputed.
fn per_lender(
    r: &mut TestResult,
    book: &Book,
    loans: &BTreeMap<String, LoanConfig>,
    ex: &Exemptions,
) -> Result<()> {
    let mut by_lender: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    let mut unnamed: Vec<&str> = Vec::new();
    for (loan_ledger, cfg) in loans {
        let key = py_casefold(py_strip(&cfg.lender));
        if key.is_empty() {
            unnamed.push(loan_ledger); // no lender configured: never grouped
        } else {
            by_lender.entry(key).or_default().push(loan_ledger);
        }
    }
    let clauses_for = |ledgers: &[&str]| -> Vec<String> {
        let types: BTreeSet<&str> = ledgers
            .iter()
            .map(|x| loans[*x].lender_type.as_str())
            .collect();
        ex.by_clause()
            .iter()
            .filter(|(_, exempt)| types.iter().any(|t| !exempt.contains(t)))
            .map(|(c, _)| (*c).to_string())
            .collect()
    };
    let tags = |ledgers: &[&str]| -> Result<String> {
        Ok(ledgers
            .iter()
            .map(|x| stable_ledger_tag(book, x))
            .collect::<Result<Vec<_>>>()?
            .join(", "))
    };
    let unnamed_clauses = clauses_for(&unnamed);
    if !unnamed.is_empty() && loans.len() > 1 && !unnamed_clauses.is_empty() {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/lender_not_configured"),
            clauses: unnamed_clauses,
            title: "Loan ledgers with no lender configured: whether they share a lender is not \
judged"
                .to_string(),
            facts: Vec::new(),
            evidence: unnamed
                .iter()
                .map(|x| EvidenceRef::new("ledger", x))
                .collect(),
            confidence: Confidence::JudgementRequired,
            limits: vec![format!(
                "{} loan ledger(s) have no lender name configured (tags {}). The limits are tested \
per ledger; whether any of these is with the same person as another loan ledger is not judged, so \
an amount that crosses a limit only on a lender's aggregate is not listed. The CA determines.",
                unnamed.len(),
                tags(&unnamed)?
            )],
            ask_client: vec!["The lender of each loan ledger named.".to_string()],
        });
    }
    for ledgers in by_lender.values().filter(|ls| ls.len() > 1) {
        let clauses = clauses_for(ledgers);
        if clauses.is_empty() {
            continue;
        }
        r.findings.push(Finding {
            id: format!(
                "{TEST_ID}/per_lender/{}",
                stable_ledger_tag(book, ledgers[0])?
            ),
            clauses,
            title: "One lender on more than one loan ledger: the limits are tested per ledger, not \
per lender"
                .to_string(),
            facts: Vec::new(),
            evidence: ledgers
                .iter()
                .map(|x| EvidenceRef::new("ledger", x))
                .collect(),
            confidence: Confidence::JudgementRequired,
            limits: vec![
                format!(
                    "This lender is configured on {} loan ledgers (tags {}). Clause 31 \
reportability, the s.269SS/269T test and the s.194A threshold are tested here ledger by ledger, \
while the law looks at the lender as a whole: an entry or interest that crosses a limit only on the \
lender's aggregate is not listed. The CA determines.",
                    ledgers.len(),
                    tags(ledgers)?
                ),
                "Lenders are matched by the configured lender name only, compared without case or \
surrounding spaces: the same person configured under two different names is not caught, and two \
people under one name are grouped. The CA confirms the lender list."
                    .to_string(),
            ],
            ask_client: vec!["Which loan ledgers are with the same person.".to_string()],
        });
    }
    Ok(())
}

/// Interest ledgers the client config declares shared: the unattributed part is a named figure
/// with its vouchers (the reference's owner decisions of 2026-09-22 and 2026-09-23).
#[allow(clippy::too_many_lines)]
fn shared_interest(
    r: &mut TestResult,
    book: &Book,
    pop: &[&Voucher],
    inputs: &Inputs,
    interest_vouchers: &HashMap<&str, HashSet<usize>>,
    net_reversals: bool,
) -> Result<()> {
    for il in inputs.shared_interest_ledgers {
        let empty = HashSet::new();
        let cited = interest_vouchers.get(il.as_str()).unwrap_or(&empty);
        let mut debits: Vec<InterestLine> = Vec::new();
        let mut credits: Vec<InterestLine> = Vec::new();
        for (at, v) in pop.iter().copied().enumerate() {
            if cited.contains(&at) || v.base_type == "Contra" {
                continue;
            }
            let others = others_than(v, il);
            for (i, l) in v.lines.iter().enumerate() {
                if l.ledger != *il || l.amount_paise == 0 {
                    continue;
                }
                let line = (
                    (&v.date, at, i),
                    at,
                    i128::from(l.amount_paise).abs(),
                    others.clone(),
                );
                if l.amount_paise > 0 {
                    debits.push(line);
                } else {
                    credits.push(line);
                }
            }
        }
        debits.sort_by(|a, b| a.0.cmp(&b.0));
        credits.sort_by(|a, b| a.0.cmp(&b.0));
        let mut taken: HashSet<usize> = HashSet::new();
        let mut reversed_pairs: Vec<(usize, usize)> = Vec::new();
        let mut unmatched: Vec<usize> = Vec::new();
        let (mut reversed_total, mut unmatched_total): (i128, i128) = (0, 0);
        for (ckey, cat, amount, others) in &credits {
            let found = debits
                .iter()
                .enumerate()
                .position(|(n, (dkey, dat, damount, dothers))| {
                    !taken.contains(&n)
                        && net_reversals
                        && dat != cat
                        && !is_receipt(&pop[*cat].base_type)
                        && damount == amount
                        && dothers == others
                        && dkey.0 <= ckey.0
                });
            match found {
                None => {
                    unmatched.push(*cat);
                    unmatched_total += amount;
                }
                Some(n) => {
                    taken.insert(n);
                    reversed_pairs.push((*cat, debits[n].1));
                    reversed_total += amount;
                }
            }
        }
        let unpaired_debits: i128 = debits.iter().map(|d| d.2).sum();
        let debit_on_vouchers: i128 = pop
            .iter()
            .flat_map(|v| v.lines.iter())
            .filter(|l| l.ledger == *il && l.amount_paise > 0)
            .map(|l| i128::from(l.amount_paise))
            .sum();
        let lh = stable_ledger_tag(book, il)?;

        let ev_debits = voucher_refs(debits.iter().map(|d| pop[d.1]));
        let pair_ev: Vec<EvidenceRef> = reversed_pairs
            .iter()
            .map(|&(c, d)| {
                (
                    pop[c].guid.clone(),
                    format!(
                        "{}, reversing {}",
                        voucher_label(pop[c]),
                        voucher_label(pop[d])
                    ),
                )
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|(id, label)| EvidenceRef::with_label("voucher", &id, &label))
            .collect();
        let f_debits = r.fig(
            &format!("shared_interest_unpaired_debits_{lh}"),
            paise(unpaired_debits)?,
            Unit::Paise,
            &format!(
                "Debits to shared interest ledger (tag {lh}) on population vouchers (Contra \
excluded) that no paired loan's interest total counts."
            ),
            ev_debits.clone(),
        )?;
        let f_reversed = r.fig(
            &format!("shared_interest_reversed_credits_{lh}"),
            paise(reversed_total)?,
            Unit::Paise,
            &if net_reversals {
                format!(
                    "Credits to shared interest ledger (tag {lh}), on the same vouchers, each \
matched as the reversal of a specific earlier debit on another voucher counted among the unpaired \
debits (each pair cited); netted against the debits."
                )
            } else {
                format!(
                    "Credits to shared interest ledger (tag {lh}) netted against the debits: none, \
since no credit reduces the figure."
                )
            },
            pair_ev.clone(),
        )?;
        let f_unmatched = r.fig(
            &format!("shared_interest_unmatched_credits_{lh}"),
            paise(unmatched_total)?,
            Unit::Paise,
            &if net_reversals {
                format!(
                    "Every other credit to shared interest ledger (tag {lh}) on the same vouchers: \
shown, not netted."
                )
            } else {
                format!(
                    "Every credit to shared interest ledger (tag {lh}) on the same vouchers: shown, \
never netted."
                )
            },
            voucher_refs(unmatched.iter().map(|&at| pop[at])),
        )?;
        let unattributed = unpaired_debits - reversed_total;
        let un_evidence: Vec<EvidenceRef> = ev_debits
            .iter()
            .chain(pair_ev.iter())
            .map(|e| (e.id.clone(), e.label.clone()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|(id, label)| EvidenceRef::with_label("voucher", &id, &label))
            .collect();
        let f_un = r.fig(
            &format!("shared_interest_unattributed_{lh}"),
            paise(unattributed)?,
            Unit::Paise,
            &format!(
                "Interest on shared interest ledger (tag {lh}) not attributed to any configured \
loan: the debits on population vouchers (Contra excluded) that no paired loan's interest total \
counts{}{}",
                if net_reversals {
                    ", less the credits matched to them as reversals. "
                } else {
                    ". "
                },
                shared_netting_note(net_reversals)
            ),
            un_evidence.clone(),
        )?;
        r.fig(
            &format!("shared_interest_debits_{lh}"),
            paise(debit_on_vouchers)?,
            Unit::Paise,
            &format!("All population debits to shared interest ledger (tag {lh})."),
            Vec::new(),
        )?;
        if unattributed > 0 {
            r.findings.push(Finding {
                id: format!("{TEST_ID}/shared_interest/{lh}"),
                clauses: vec!["s.194A".to_string()],
                title: "Interest on a shared interest ledger that is not attributed to any \
configured loan"
                    .to_string(),
                facts: vec![
                    ("unattributed".to_string(), f_un),
                    ("unpaired_debits".to_string(), f_debits),
                    ("reversed_credits".to_string(), f_reversed),
                    ("unmatched_credits".to_string(), f_unmatched),
                ],
                evidence: un_evidence,
                confidence: Confidence::NeedsDocument,
                limits: vec!["For a voucher that names no loan, the books do not say which lender \
this interest was paid to. Whether s.194A applies depends on the lender: interest paid to a bank, a \
co-operative bank, an insurer or another body s.194A(3)(iii) names is exempt; other lenders may not \
be. A voucher that also posts to a configured loan (an instalment paying principal and interest \
together, or interest net of a TDS ledger not classified as TDS payable) names that loan; it is \
counted here because the loan's interest total does not include it."
                    .to_string()],
                ask_client: vec![
                    "Bank or lender statements identifying the lender for these interest debits."
                        .to_string(),
                ],
            });
        }
    }
    Ok(())
}

/// F10: one NOT COMPUTED notice per ledger that moved in the year and is not in the client's list
/// of loans -- under Loans (Liability), or under a pattern group and debited like an instalment.
/// Only adds: nothing else in this test reads these notices.
#[allow(clippy::too_many_lines)]
fn unlisted_loan_notices(
    r: &mut TestResult,
    book: &Book,
    pop: &[&Voucher],
    loans: &BTreeMap<String, LoanConfig>,
    inputs: &Inputs,
) -> Result<()> {
    let (mut notices, mut pattern_notices, mut chain_incomplete) = (0_usize, 0_usize, 0_usize);
    let ev = |xs: &[(&Voucher, i64)]| voucher_refs(xs.iter().map(|(v, _)| *v));
    for (name, led) in &book.ledgers {
        if loans.contains_key(name)
            || inputs.cash.contains(name)
            || inputs.bank.contains(name)
            || led.under(BANK_OD_GROUP)
        {
            continue;
        }
        let lines: Vec<(&Voucher, i64)> = pop
            .iter()
            .copied()
            .filter(|v| v.base_type != "Contra")
            .flat_map(|v| {
                v.lines
                    .iter()
                    .filter(|l| l.ledger == *name && l.amount_paise != 0)
                    .map(move |l| (v, l.amount_paise))
            })
            .collect();
        if lines.is_empty() {
            continue;
        }
        if !led.chain_complete {
            chain_incomplete += 1;
            continue;
        }
        let h = stable_ledger_tag(book, name)?;
        if led.under(LOANS_GROUP) {
            notices += 1;
            let credits: Vec<(&Voucher, i64)> =
                lines.iter().copied().filter(|&(_, a)| a < 0).collect();
            let debits: Vec<(&Voucher, i64)> =
                lines.iter().copied().filter(|&(_, a)| a > 0).collect();
            let f_cr = r.fig(
                &format!("loan_not_listed_credits_{h}"),
                paise(-credits.iter().map(|&(_, a)| i128::from(a)).sum::<i128>())?,
                Unit::Paise,
                &format!(
                    "Credits in the year to ledger (tag {h}), under Loans (Liability) and not in \
the client's list of loans (Contra vouchers excluded, as throughout this test)."
                ),
                ev(&credits),
            )?;
            let f_dr = r.fig(
                &format!("loan_not_listed_debits_{h}"),
                paise(debits.iter().map(|&(_, a)| i128::from(a)).sum())?,
                Unit::Paise,
                &format!(
                    "Debits in the year to ledger (tag {h}), under Loans (Liability) and not in \
the client's list of loans (Contra vouchers excluded, as throughout this test)."
                ),
                ev(&debits),
            )?;
            r.findings.push(Finding {
                id: format!("{TEST_ID}/loan_not_listed/{h}"),
                clauses: vec!["3CD-31(a)".to_string(), "3CD-31(c)".to_string()],
                title: "A ledger under Loans (Liability) is not in the client's list of loans: \
this test computed no clause 31 rows for it"
                    .to_string(),
                facts: vec![("credits".to_string(), f_cr), ("debits".to_string(), f_dr)],
                evidence: vec![EvidenceRef::new("ledger", name)],
                confidence: Confidence::NeedsDocument,
                limits: vec![
                    "Not computed: this test runs clause 31, the s.269SS/269T test and the s.194A \
test only on the loans in the client's list. This ledger moved in the year and is not in it, so \
this test computed none of them for it (a cash entry on it may still be listed by the cash-payments \
test); its credits and debits are shown as the books hold them."
                        .to_string(),
                    NOTICE_ONLY_ADDS.to_string(),
                ],
                ask_client: vec!["Whether this ledger is a loan; if so, the lender, the lender's \
type (such as a bank, an NBFC or a person) and the loan statement."
                    .to_string()],
            });
            continue;
        }
        if !PATTERN_GROUPS.iter().any(|g| led.under(g)) || led.under(DUTIES_TAXES_GROUP) {
            continue;
        }
        let mut by_amount: Vec<(i64, Vec<&Voucher>)> = Vec::new();
        for &(v, a) in &lines {
            if a > 0
                && has_mandate_word(&v.narration)
                && v.lines
                    .iter()
                    .any(|l| inputs.bank.contains(&l.ledger) && l.amount_paise < 0)
            {
                match by_amount.iter_mut().find(|(k, _)| *k == a) {
                    Some((_, vs)) => vs.push(v),
                    None => by_amount.push((a, vec![v])),
                }
            }
        }
        let hits: Vec<&Voucher> = by_amount
            .iter()
            .filter(|(_, vs)| {
                vs.iter()
                    .map(|v| &v.date.as_str()[..6])
                    .collect::<BTreeSet<_>>()
                    .len()
                    >= PATTERN_MIN_MONTHS
            })
            .flat_map(|(_, vs)| vs.iter().copied())
            .collect();
        if hits.is_empty() {
            continue;
        }
        pattern_notices += 1;
        let f_n = r.fig(
            &format!("loan_pattern_debits_{h}"),
            count(TEST_ID, hits.len())?,
            Unit::Count,
            &format!(
                "Bank debits to ledger (tag {h}), under Current Liabilities, Sundry Creditors or \
Suspense A/c and not in the client's list of loans, whose narration carries ACH, NACH, ECS or EMI, \
of one amount in {PATTERN_MIN_MONTHS} or more different months."
            ),
            voucher_refs(hits.iter().copied()),
        )?;
        r.findings.push(Finding {
            id: format!("{TEST_ID}/loan_pattern_not_listed/{h}"),
            clauses: vec!["3CD-31(a)".to_string(), "3CD-31(c)".to_string()],
            title: "A ledger under current liabilities or suspense is debited like a loan \
instalment and is not in the client's list of loans: this test computed no clause 31 rows for it"
                .to_string(),
            facts: vec![("pattern_debits".to_string(), f_n)],
            evidence: vec![EvidenceRef::new("ledger", name)],
            confidence: Confidence::NeedsDocument,
            limits: vec![
                "Not computed: this test runs clause 31, the s.269SS/269T test and the s.194A test \
only on the loans in the client's list. This ledger is picked because its bank debits look like \
instalments (a mandate or instalment word in the narration, one amount, several months); that is a \
pattern, not a finding that it is a loan, and the lender is not classified from it. Only ledgers \
under Current Liabilities, Sundry Creditors or Suspense A/c are scanned for it; an asset or capital \
ledger is not."
                    .to_string(),
                NOTICE_ONLY_ADDS.to_string(),
            ],
            ask_client: vec!["Whether this ledger is a loan; if so, the lender, the lender's type \
(such as a bank, an NBFC or a person) and the loan statement."
                .to_string()],
        });
    }
    r.fig(
        "loan_not_listed_count",
        count(TEST_ID, notices)?,
        Unit::Count,
        "Ledgers under Loans (Liability), other than bank and cash ledgers, that moved in the year \
and are not in the client's list of loans (each has a notice).",
        Vec::new(),
    )?;
    r.fig(
        "loan_notice_chain_incomplete_count",
        count(TEST_ID, chain_incomplete)?,
        Unit::Count,
        "Ledgers that moved in the year, are not in the client's list of loans and are not bank or \
cash ledgers, whose Tally group chain is incomplete: whether each is under Loans (Liability) cannot \
be told, so neither notice is raised for it.",
        Vec::new(),
    )?;
    r.fig(
        "loan_pattern_not_listed_count",
        count(TEST_ID, pattern_notices)?,
        Unit::Count,
        &format!(
            "Ledgers under Current Liabilities, Sundry Creditors or Suspense A/c, not in the \
client's list of loans, with bank debits of one amount in {PATTERN_MIN_MONTHS} or more months \
narrated with ACH, NACH, ECS or EMI (each has a notice)."
        ),
        Vec::new(),
    )?;
    Ok(())
}

/// LOAN-1 to LOAN-4 with the rule in force ([`NET_REVERSALS`]); the reference's
/// `check_invariants`, whose docstring describes each check and its accepted limits. LOAN-1 ties
/// because a voucher listed as both crediting and debiting a loan has its credit side in the
/// loan's taken total and its debit side in its repaid total; LOAN-4 (#779 Phase A) holds that
/// every such voucher is listed with its own two sides, or is an interest or TDS entry, or a
/// balanced voucher on the loan alone. The tie adds the TDS deducted on the loan, less what the
/// taken and repaid totals already hold of it (`tds_in_principal_<tag>`, published only where
/// there is some): a repayment booked net of TDS carries its deduction in its loan line
/// (bridge#1259 item 2). Its re-derivation reads vouchers by GUID, as the reference does: on a GUID
/// a voucher outside the taken and repaid rows shares, it reads that voucher too and the line can
/// fire while the tie holds. Refs go in (id, label) order; the reference's, per id, is not fixed.
pub fn check_invariants(book: &Book, result: &TestResult) -> Result<Vec<String>> {
    check_invariants_with(book, result, NET_REVERSALS)
}

/// A figure's value as the reference reads a paise figure.
fn int_of(f: &crate::findings::Figure) -> i128 {
    match f.value {
        Value::Int(n) => i128::from(n),
        _ => 0,
    }
}

/// A text figure's lines: the reference's `frozenset(fig.value.split("\n"))`.
fn lines_of(f: &crate::findings::Figure) -> BTreeSet<String> {
    match &f.value {
        Value::Text(t) => t.split('\n').map(str::to_string).collect(),
        _ => BTreeSet::new(),
    }
}

/// The reference's `_names_shown`: one name as `repr()`, several as a sorted list's `repr()`.
fn names_shown(names: &BTreeSet<String>) -> String {
    if names.len() == 1 {
        py_repr_str(names.iter().next().map_or("", String::as_str))
    } else {
        py_repr_list(&names.iter().cloned().collect::<Vec<_>>())
    }
}

/// [`check_invariants`], with the shared-ledger rule given.
#[allow(clippy::too_many_lines)]
pub fn check_invariants_with(
    book: &Book,
    result: &TestResult,
    net_reversals: bool,
) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let prefix = format!("{}.", result.test_id);
    let tol: i128 = 100;
    let hash_to_name = ledgers_by_tag(book)?;
    let figures: BTreeMap<&str, &crate::findings::Figure> =
        result.figures.iter().map(|f| (f.id.as_str(), f)).collect();
    let under_duties = |x: &str| {
        book.ledgers
            .get(x)
            .is_some_and(|l| l.under(DUTIES_TAXES_GROUP))
    };

    // A loan may name several interest ledgers, one per line. Kept in the result's own figure
    // order, as the reference's dict is.
    let marker = format!("{prefix}interest_ledger_");
    let mut interest_ledger_by_tag: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut interest_tags_in_order: Vec<String> = Vec::new();
    for f in &result.figures {
        if let (Some(tag), Value::Text(text)) = (f.id.strip_prefix(&marker), &f.value) {
            if !text.is_empty() {
                interest_ledger_by_tag.insert(tag.to_string(), lines_of(f));
                interest_tags_in_order.push(tag.to_string());
            }
        }
    }

    let pop = book.population()?;
    // The TDS ledgers run() published (tds_payable_ledgers: the configuration, not a group), read
    // back by LOAN-1 and LOAN-4. LOAN-1 reads the figure's lines as they are, an empty text being no
    // ledger at all but an empty name beside another being a ledger; only LOAN-4 leaves the empty
    // name out (`tds_read_loan4`).
    let tds_read: BTreeSet<String> = figures
        .get(format!("{prefix}tds_payable_ledgers").as_str())
        .filter(|f| matches!(&f.value, Value::Text(t) if !t.is_empty()))
        .map(|f| lines_of(f))
        .unwrap_or_default();
    let tds_read_loan4: BTreeSet<String> =
        tds_read.iter().filter(|x| !x.is_empty()).cloned().collect();

    // LOAN-1
    let marker = format!("{prefix}interest_total_");
    for (fid, fig) in &figures {
        let Some(h) = fid.strip_prefix(&marker) else {
            continue;
        };
        let Some(name) = hash_to_name.get(h) else {
            out.push(format!(
                "LOAN-1: cannot resolve a loan ledger for figure {fid} (tag {h})"
            ));
            continue;
        };
        let Some(tb_row) = book.tb.get(name.as_str()) else {
            out.push(format!(
                "LOAN-1: {name} (tag {h}) has no Trial Balance row to reconcile against"
            ));
            continue;
        };
        let value = |id: String| figures.get(id.as_str()).map_or(0, |f| int_of(f));
        let taken_total = value(format!("{prefix}clause31_taken_total_{h}"));
        let repaid_total = value(format!("{prefix}clause31_repaid_total_{h}"));
        // interest_total is gross of the TDS on its journals, and a TDS journal debits the loan
        // outside taken/repaid: the loan moved by the interest net of every TDS deduction on it.
        let tds_on_loan = value(format!("{prefix}tds_on_loan_{h}"));
        let interest = int_of(fig);
        // ... except the TDS on a voucher counted in taken or repaid, whose loan line holds it
        // (bridge#1259 item 2). It is re-derived from the vouchers the figure cites, by GUID, on
        // the TDS ledgers run() published, so a TDS line read wrongly on a taken or repaid voucher
        // cannot cancel inside the tie.
        let principal_id = format!("{prefix}tds_in_principal_{h}");
        let tds_in_principal = value(principal_id.clone());
        if let Some(principal) = figures.get(principal_id.as_str()) {
            let cited: BTreeSet<&str> = principal
                .evidence
                .iter()
                .filter(|e| e.kind == "voucher")
                .map(|e| e.id.as_str())
                .collect();
            let derived: i128 = pop
                .iter()
                .filter(|v| {
                    cited.contains(v.guid.as_str())
                        && v.base_type != "Contra"
                        && v.lines
                            .iter()
                            .any(|l| l.ledger == **name && l.amount_paise != 0)
                })
                .map(|v| -net_on(v, &tds_read))
                .sum();
            if derived != tds_in_principal {
                out.push(format!(
                    "LOAN-1: {name} (tag {h}) tds_in_principal_{h} says {tds_in_principal}p but \
the lines on the TDS ledgers of the vouchers it cites on this loan are {derived}p"
                ));
            }
        }
        let expected_movement =
            -taken_total + repaid_total - interest + tds_on_loan - tds_in_principal;
        let movement = i128::from(tb_row.closing_paise) - i128::from(tb_row.opening_paise);
        let diff = expected_movement - movement;
        if diff.abs() > tol {
            out.push(format!(
                "LOAN-1: {name} (tag {h}) expected FY movement {expected_movement}p (= -taken \
{taken_total}p + repaid {repaid_total}p - interest {interest}p + TDS {}p) does not tie \
the TB movement {movement}p (opening {}p, closing {}p); difference {diff}p -- a voucher on this \
loan ledger was likely dropped from or wrongly added to the population.",
                tds_on_loan - tds_in_principal,
                tb_row.opening_paise,
                tb_row.closing_paise
            ));
        }
    }

    let mut pop_by_guid: HashMap<&str, Vec<&Voucher>> = HashMap::new();
    for v in &pop {
        pop_by_guid.entry(v.guid.as_str()).or_default().push(v);
    }
    let shared_guid = |g: &str| pop_by_guid.get(g).is_some_and(|vs| vs.len() > 1);
    // The vouchers run() listed as both crediting and debiting loan (tag h), by GUID: one side's
    // figure (#779).
    let listed_sides = |h: &str, side: &str| -> BTreeMap<String, i128> {
        let marker = format!("{prefix}two_sided_gross_{side}_{h}_");
        let mut sides = BTreeMap::new();
        for f in result.figures.iter().filter(|f| f.id.starts_with(&marker)) {
            for e in f.evidence.iter().filter(|e| e.kind == "voucher") {
                sides.insert(e.id.clone(), int_of(f));
            }
        }
        sides
    };

    // LOAN-2 (A)-(C)
    for (h, ils) in &interest_ledger_by_tag {
        let fid = format!("{prefix}interest_total_{h}");
        let Some(loan) = hash_to_name.get(h.as_str()) else {
            out.push(format!(
                "LOAN-2: cannot resolve a loan ledger for figure {prefix}interest_ledger_{h} (tag {h})"
            ));
            continue;
        };
        let Some(fig) = figures.get(fid.as_str()) else {
            out.push(format!(
                "LOAN-2: loan ledger {} (tag {h}) has an interest ledger ({}) but no \
interest_total_{h} figure, so its interest is checked nowhere",
                py_repr_str(loan),
                names_shown(ils)
            ));
            continue;
        };
        let cited: BTreeSet<&str> = fig
            .evidence
            .iter()
            .filter(|e| e.kind == "voucher")
            .map(|e| e.id.as_str())
            .collect();
        let tds = figures
            .get(format!("{prefix}interest_tds_ledgers_{h}").as_str())
            .map(|f| lines_of(f))
            .unwrap_or_default();
        let on_loan: Vec<&Voucher> = pop
            .iter()
            .copied()
            .filter(|v| v.lines.iter().any(|l| l.ledger == **loan))
            .collect();
        let duplicated: Vec<String> = on_loan
            .iter()
            .filter(|v| shared_guid(&v.guid))
            .map(|v| v.guid.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        if !duplicated.is_empty() {
            out.push(format!(
                "LOAN-2: vouchers posting to loan ledger {} (tag {h}) share a GUID with another \
books-population voucher, so they cannot be matched to interest_total_{h}'s evidence: {}",
                py_repr_str(loan),
                py_repr_list(&duplicated)
            ));
        }
        let listed_here = listed_sides(h.as_str(), "credit");
        for v in &on_loan {
            if cited.contains(v.guid.as_str()) || duplicated.contains(&v.guid) {
                continue;
            }
            let on_interest = net_on(v, ils);
            if on_interest != 0 {
                // Interest net of an unconfigured ledger: named by shape, never by the ledger's name.
                let extra: Vec<String> = v
                    .lines
                    .iter()
                    .map(|l| l.ledger.as_str())
                    .filter(|l| *l != loan.as_str() && !ils.contains(*l))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .map(str::to_string)
                    .collect();
                let opposite = |x: i128| x.signum() * on_interest.signum() < 0;
                let looks_netted = !extra.is_empty()
                    && opposite(net(v, loan))
                    && extra.iter().all(|x| opposite(net(v, x)) && under_duties(x));
                let hint = if looks_netted {
                    format!(
                        " It looks like interest booked net of {} (under {}): if {} as \
\"tds_payable\" in [statutory_dues.nature_by_ledger] so this voucher is read as interest, not as a \
loan taken or repaid.",
                        extra
                            .iter()
                            .map(|x| py_repr_str(x))
                            .collect::<Vec<_>>()
                            .join(", "),
                        py_repr_str(DUTIES_TAXES_GROUP),
                        if extra.len() == 1 {
                            "that is a TDS ledger, classify it"
                        } else {
                            "those are TDS ledgers, classify each"
                        }
                    )
                } else {
                    String::new()
                };
                let why = if listed_here.contains_key(&v.guid) {
                    format!(
                        "the voucher both credits and debits the loan and is listed as not \
computed, so its interest is shown beside interest_total_{h}, not in it (#779)."
                    )
                } else {
                    format!(
                        "the builder counted the voucher as taken/repaid or skipped it (an \
interest journal's other lines may be only its interest ledger and ledgers classified as TDS \
payable; see this module's docstring).{hint}"
                    )
                };
                out.push(format!(
                    "LOAN-2: voucher {} (guid {}) posts {on_interest}p to interest ledger {} and a \
line to loan ledger {} (tag {h}), but that interest is in no interest_total_{h}: {why}",
                    voucher_label(v),
                    v.guid,
                    names_shown(ils),
                    py_repr_str(loan)
                ));
            }
        }
        let missing: Vec<String> = cited
            .iter()
            .filter(|g| !pop_by_guid.contains_key(**g))
            .map(|g| (*g).to_string())
            .collect();
        if !missing.is_empty() {
            out.push(format!(
                "LOAN-2: interest_total_{h} cites vouchers outside the books population: {}",
                py_repr_list(&missing)
            ));
        }
        let cited_vouchers: Vec<&Voucher> = cited
            .iter()
            .filter(|g| !duplicated.iter().any(|d| d == **g))
            .flat_map(|g| pop_by_guid.get(*g).into_iter().flatten().copied())
            .collect();
        let on_cited: i128 = -cited_vouchers
            .iter()
            .map(|v| net(v, loan) + net_on(v, &tds))
            .sum::<i128>();
        let value = int_of(fig);
        if (on_cited - value).abs() > tol {
            out.push(format!(
                "LOAN-2: interest_total_{h} is {value}p but the loan ledger {}'s own lines on the \
vouchers it cites net {}p (interest {on_cited}p); difference {}p.",
                py_repr_str(loan),
                -on_cited,
                value - on_cited
            ));
        }
        for v in &cited_vouchers {
            let (on_l, on_ils) = (net(v, loan), net_on(v, ils));
            let residue = on_l + on_ils + net_on(v, &tds);
            if residue != 0 {
                out.push(format!(
                    "LOAN-2: interest_total_{h} counts voucher {} (guid {}), whose loan and \
interest-ledger lines do not cancel (loan {on_l}p, interest {on_ils}p): {residue}p of it moves to \
or from another ledger, so part of what is counted as interest is principal or something else.",
                    voucher_label(v),
                    v.guid
                ));
            }
        }
    }

    // LOAN-2 (D)/(E): an entry on a loan against only another loan's interest ledger, or a shared
    // one, is asked about; the interest ledgers are read back from run()'s own figures.
    let names_after = |marker: &str| -> BTreeSet<String> {
        figures
            .keys()
            .filter_map(|f| f.strip_prefix(marker))
            .filter_map(|h| hash_to_name.get(h).map(|n| (*n).clone()))
            .collect()
    };
    let loan_names = names_after(&format!("{prefix}interest_total_"));
    let shared_names = names_after(&format!("{prefix}shared_interest_debits_"));
    let all_interest: BTreeSet<String> = interest_ledger_by_tag
        .values()
        .flatten()
        .cloned()
        .chain(shared_names)
        .filter(|x| !loan_names.contains(x))
        .collect();
    let empty_set = BTreeSet::new();
    for (fid, fig) in &figures {
        let Some(h) = fid.strip_prefix(&format!("{prefix}interest_total_")) else {
            continue;
        };
        let Some(loan) = hash_to_name.get(h) else {
            continue;
        };
        let own = interest_ledger_by_tag.get(h).unwrap_or(&empty_set);
        let foreign: BTreeSet<&str> = all_interest
            .iter()
            .filter(|x| !own.contains(*x))
            .map(String::as_str)
            .collect();
        let mis = figures
            .get(format!("{prefix}possible_misposted_interest_{h}").as_str())
            .copied();
        let voucher_ids = |f: &crate::findings::Figure| -> BTreeSet<String> {
            f.evidence
                .iter()
                .filter(|e| e.kind == "voucher")
                .map(|e| e.id.clone())
                .collect()
        };
        let mis_cited = mis.map(voucher_ids).unwrap_or_default();
        let mut cited = voucher_ids(fig);
        cited.extend(listed_sides(h, "credit").into_keys());
        for v in &pop {
            let others = others_than(v, loan);
            if v.base_type == "Contra"
                || cited.contains(&v.guid)
                || mis_cited.contains(&v.guid)
                || !others.iter().all(|o| foreign.contains(o))
                || others.is_empty()
                || !v.lines.iter().any(|l| l.ledger == **loan)
                || net(v, loan) == 0
            {
                continue;
            }
            out.push(format!(
                "LOAN-2: voucher {} (guid {}) posts to loan ledger {} (tag {h}) against only \
another interest ledger (another loan's, or one declared shared), but \
possible_misposted_interest_{h} does not cite it: the entry is not asked about.",
                voucher_label(v),
                v.guid,
                py_repr_str(loan)
            ));
        }
        let Some(mis) = mis else {
            continue;
        };
        let mut on_loan: i128 = 0;
        for g in &mis_cited {
            if shared_guid(g) {
                continue; // a shared GUID is reported by LOAN-2 above
            }
            for v in pop_by_guid.get(g.as_str()).into_iter().flatten() {
                let stray: Vec<String> = v
                    .lines
                    .iter()
                    .filter(|l| l.amount_paise != 0)
                    .map(|l| l.ledger.as_str())
                    .filter(|l| *l != loan.as_str() && !foreign.contains(l) && !under_duties(l))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .map(str::to_string)
                    .collect();
                if !stray.is_empty() || !v.lines.iter().any(|l| foreign.contains(l.ledger.as_str()))
                {
                    out.push(format!(
                        "LOAN-2: possible_misposted_interest_{h} cites voucher {} (guid {}), which \
moves to or from {}.",
                        voucher_label(v),
                        v.guid,
                        if stray.is_empty() {
                            "no other interest ledger".to_string()
                        } else {
                            py_repr_list(&stray)
                        }
                    ));
                }
                on_loan -= net(v, loan);
            }
        }
        if (on_loan - int_of(mis)).abs() > tol && !mis_cited.iter().any(|g| shared_guid(g)) {
            out.push(format!(
                "LOAN-2: possible_misposted_interest_{h} is {}p but the loan's lines on the \
vouchers it cites net {on_loan}p.",
                int_of(mis)
            ));
        }
    }

    // LOAN-4 (#779 Phase A), per loan ledger: every population voucher (Contra excluded) with both
    // a credit and a debit line on it is either listed -- a two_sided_gross_credit/debit figure
    // pair citing it, equal to the voucher's own credit and debit lines on the loan -- or read as
    // interest: its every other non-zero line is one of the loan's interest ledgers or a TDS ledger
    // run() published (tds_read_loan4, above), with no other loan's
    // line; a voucher on the loan alone is listed only if it does not balance. A listed voucher
    // that does not credit and debit the loan is named too.
    for fid in figures.keys() {
        let Some(h) = fid.strip_prefix(&format!("{prefix}interest_total_")) else {
            continue;
        };
        let Some(loan) = hash_to_name.get(h) else {
            continue;
        };
        let ils = interest_ledger_by_tag.get(h).unwrap_or(&empty_set);
        let (credits, debits) = (listed_sides(h, "credit"), listed_sides(h, "debit"));
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for v in &pop {
            if v.base_type == "Contra" || shared_guid(&v.guid) {
                continue; // a shared GUID is reported by LOAN-2
            }
            let on_loan = |debit: bool| -> i128 {
                v.lines
                    .iter()
                    .filter(|l| l.ledger == **loan && l.amount_paise != 0)
                    .filter(|l| (l.amount_paise > 0) == debit)
                    .map(|l| i128::from(l.amount_paise).abs())
                    .sum()
            };
            let (credit, debit) = (on_loan(false), on_loan(true));
            if credit == 0 || debit == 0 {
                continue;
            }
            seen.insert(v.guid.as_str());
            if let Some(&shown_credit) = credits.get(&v.guid) {
                let shown_debit = debits.get(&v.guid).copied();
                if shown_credit != credit || shown_debit != Some(debit) {
                    out.push(format!(
                        "LOAN-4: voucher {} (guid {}) credits loan ledger {} (tag {h}) {credit}p and \
debits it {debit}p, but its listed record shows {shown_credit}p and {}p.",
                        voucher_label(v),
                        v.guid,
                        py_repr_str(loan),
                        shown_debit.map_or_else(|| "None".to_string(), |x| x.to_string())
                    ));
                }
                continue;
            }
            let others: BTreeSet<&str> = v
                .lines
                .iter()
                .filter(|l| l.ledger != **loan && l.amount_paise != 0)
                .map(|l| l.ledger.as_str())
                .collect();
            let unbalanced = v
                .lines
                .iter()
                .map(|l| i128::from(l.amount_paise))
                .sum::<i128>()
                != 0;
            if (others.is_empty() && unbalanced)
                || others
                    .iter()
                    .any(|o| *o != loan.as_str() && loan_names.contains(*o))
                || !others
                    .iter()
                    .all(|o| ils.contains(*o) || tds_read_loan4.contains(*o))
            {
                out.push(format!(
                    "LOAN-4: voucher {} (guid {}) both credits ({credit}p) and debits ({debit}p) loan \
ledger {} (tag {h}), and is not an interest or TDS entry, but no listed record cites it: it was \
netted into one entry or dropped.",
                    voucher_label(v),
                    v.guid,
                    py_repr_str(loan)
                ));
            }
        }
        for g in credits.keys() {
            if !seen.contains(g.as_str()) && pop_by_guid.get(g.as_str()).map_or(0, Vec::len) <= 1 {
                out.push(format!(
                    "LOAN-4: a listed record on loan ledger {} (tag {h}) cites voucher guid {g}, \
which does not both credit and debit the loan in the books population.",
                    py_repr_str(loan)
                ));
            }
        }
    }

    // LOAN-3
    let mut by_interest_ledger: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for h in &interest_tags_in_order {
        for led in &interest_ledger_by_tag[h] {
            by_interest_ledger
                .entry(led.clone())
                .or_default()
                .push(h.clone());
        }
    }
    let shared_marker = format!("{prefix}shared_interest_debits_");
    for fid in figures.keys() {
        let Some(sh) = fid.strip_prefix(&shared_marker) else {
            continue;
        };
        match hash_to_name.get(sh) {
            None => out.push(format!(
                "LOAN-3: cannot resolve a shared interest ledger for figure {fid} (tag {sh})"
            )),
            Some(name) => {
                by_interest_ledger.entry((*name).clone()).or_default();
            }
        }
    }
    for (interest_ledger, tags) in &by_interest_ledger {
        let lh = stable_ledger_tag(book, interest_ledger)?;
        let shared_fid = format!("{prefix}shared_interest_debits_{lh}");
        let paired: BTreeSet<&str> = tags
            .iter()
            .filter_map(|h| hash_to_name.get(h.as_str()).map(|n| n.as_str()))
            .collect();
        if let Some(shared_fig) = figures.get(shared_fid.as_str()) {
            let debit_pop: i128 = pop
                .iter()
                .flat_map(|v| v.lines.iter())
                .filter(|l| l.ledger == *interest_ledger && l.amount_paise > 0)
                .map(|l| i128::from(l.amount_paise))
                .sum();
            match book.tb.get(interest_ledger) {
                None => out.push(format!(
                    "LOAN-3: shared interest ledger {} has no Trial Balance row to reconcile against",
                    py_repr_str(interest_ledger)
                )),
                Some(tb_row) if (debit_pop - i128::from(tb_row.debit_paise)).abs() > tol => {
                    out.push(format!(
                        "LOAN-3: shared interest ledger {} population debits {debit_pop}p do not \
tie the TB period debit {}p",
                        py_repr_str(interest_ledger),
                        tb_row.debit_paise
                    ));
                }
                Some(_) => {}
            }
            if int_of(shared_fig) != debit_pop {
                out.push(format!(
                    "LOAN-3: shared_interest_debits_{lh} is {}p but the population's debits to {} \
are {debit_pop}p",
                    int_of(shared_fig),
                    py_repr_str(interest_ledger)
                ));
            }
            // Recomputed from the vouchers, never from the builder's figures or evidence: a paired
            // loan's journals are re-derived by interest_total's own rule, its TDS ledgers and
            // several interest ledgers included, held by position.
            let mut tds_of: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
            let mut ils_by_loan: BTreeMap<&str, &BTreeSet<String>> = BTreeMap::new();
            for h in tags {
                if let Some(name) = hash_to_name.get(h.as_str()) {
                    tds_of.insert(
                        name.as_str(),
                        figures
                            .get(format!("{prefix}interest_tds_ledgers_{h}").as_str())
                            .map(|f| lines_of(f))
                            .unwrap_or_default(),
                    );
                    ils_by_loan.insert(name.as_str(), &interest_ledger_by_tag[h]);
                }
            }
            let journals: HashSet<usize> = pop
                .iter()
                .enumerate()
                .filter(|(_, v)| v.base_type != "Contra")
                .filter(|(_, v)| {
                    paired.iter().any(|loan| {
                        let rest: Vec<&str> = posted_others_than(v, loan)
                            .into_iter()
                            .filter(|o| !tds_of.get(loan).is_some_and(|t| t.contains(*o)))
                            .collect();
                        v.lines.iter().any(|l| l.ledger == *loan)
                            && !rest.is_empty()
                            && rest.iter().all(|o| ils_by_loan[loan].contains(*o))
                            && net(v, loan) != 0
                    })
                })
                .map(|(n, _)| n)
                .collect();
            let mut lines: Vec<InterestLine> = Vec::new();
            for (n, v) in pop.iter().enumerate() {
                if journals.contains(&n) || v.base_type == "Contra" {
                    continue;
                }
                for (i, l) in v.lines.iter().enumerate() {
                    if l.ledger == *interest_ledger && l.amount_paise != 0 {
                        lines.push((
                            (&v.date, n, i),
                            n,
                            i128::from(l.amount_paise),
                            others_than(v, interest_ledger),
                        ));
                    }
                }
            }
            let debit_lines: Vec<_> = lines.iter().filter(|x| x.2 > 0).collect();
            let mut credit_lines: Vec<_> = lines.iter().filter(|x| x.2 < 0).collect();
            credit_lines.sort_by(|a, b| a.0.cmp(&b.0));
            let mut used = BTreeSet::new();
            let (mut reversed_total, mut unmatched_total): (i128, i128) = (0, 0);
            for (key, n, amount, others) in credit_lines {
                let candidates: Vec<_> = if !net_reversals || is_receipt(&pop[*n].base_type) {
                    Vec::new()
                } else {
                    debit_lines
                        .iter()
                        .filter(|d| {
                            !used.contains(&d.0)
                                && d.1 != *n
                                && d.2 == -amount
                                && d.3 == *others
                                && d.0 .0 <= key.0
                        })
                        .collect()
                };
                if let Some(first) = candidates.iter().map(|d| d.0).min() {
                    used.insert(first);
                    reversed_total -= amount;
                } else {
                    unmatched_total -= amount;
                }
            }
            let unpaired_debits: i128 = debit_lines.iter().map(|d| d.2).sum();
            let expected = [
                ("unpaired_debits", unpaired_debits),
                ("reversed_credits", reversed_total),
                ("unmatched_credits", unmatched_total),
                ("unattributed", unpaired_debits - reversed_total),
            ];
            for (name, value) in expected {
                let fid = format!("{prefix}shared_interest_{name}_{lh}");
                match figures.get(fid.as_str()) {
                    None => out.push(format!(
                        "LOAN-3: shared interest ledger {} has no shared_interest_{name}_{lh} figure",
                        py_repr_str(interest_ledger)
                    )),
                    Some(f) if int_of(f) != value => out.push(format!(
                        "LOAN-3: shared_interest_{name}_{lh} is {}p but the vouchers no paired \
loan's interest total cites give {value}p",
                        int_of(f)
                    )),
                    Some(_) => {}
                }
            }
            continue;
        }
        let outside: Vec<(&Voucher, i128)> = pop
            .iter()
            .copied()
            .filter(|v| !v.lines.iter().any(|l| paired.contains(l.ledger.as_str())))
            .map(|v| (v, net(v, interest_ledger)))
            .filter(|&(_, a)| a != 0)
            .collect();
        let outside_net: i128 = outside.iter().map(|&(_, a)| a).sum();
        if outside_net > tol {
            let debits: Vec<&(&Voucher, i128)> = outside.iter().filter(|&&(_, a)| a > 0).collect();
            let shown: Vec<String> = debits
                .iter()
                .take(5)
                .map(|&&(v, amt)| format!("{} ({amt}p)", voucher_label(v)))
                .collect();
            out.push(format!(
                "LOAN-3: interest ledger {} (not declared shared) nets {outside_net}p of debits on \
vouchers that post to none of its paired loan ledgers, so that interest is in no loan's \
interest_total; the debits: {}{} -- declare the ledger shared ([loans].shared_interest_ledgers) or \
book the interest against its loan.",
                py_repr_str(interest_ledger),
                shown.join(", "),
                if debits.len() > 5 { " ..." } else { "" }
            ));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{Ledger, LedgerLine, VoucherStatus};

    fn table(text: &str) -> BTreeMap<String, toml::Value> {
        let t: toml::Table = toml::from_str(text).unwrap();
        t.into_iter().collect()
    }

    #[test]
    fn a_loan_entry_is_typed_or_refused_naming_it() {
        let ok = loan_config(&table(
            "[\"Loan A\"]\nlender = \"x\"\nlender_type = \"nbfc\"\ninterest_ledger = \"Int\"\n\
             [\"Loan B\"]\nlender = \"y\"\nlender_type = \"person\"\n\
             [\"Loan C\"]\nlender = \"z\"\nlender_type = \"nbfc\"\n\
             interest_ledger = [\"Int C\", \"Charges C\", \"Int C\"]\n\
             [\"Loan D\"]\nlender = \"w\"\nlender_type = \"person\"\ninterest_ledger = \"\"\n",
        ))
        .unwrap();
        let names =
            |l: &str| -> Vec<&str> { ok[l].interest_ledgers.iter().map(String::as_str).collect() };
        assert_eq!(names("Loan A"), ["Int"]);
        assert!(names("Loan B").is_empty() && names("Loan D").is_empty());
        assert_eq!(names("Loan C"), ["Charges C", "Int C"]);
        for (text, needle) in [
            ("\"Loan A\" = 5\n", "is not a table"),
            ("[\"Loan A\"]\nlender_type = \"nbfc\"\n", "has no lender"),
            ("[\"Loan A\"]\nlender = \"x\"\n", "has no lender_type"),
            (
                "[\"Loan A\"]\nlender = \"x\"\nlender_type = 3\n",
                "lender_type is not a string",
            ),
            (
                "[\"Loan A\"]\nlender = \"x\"\nlender_type = \"nbfc\"\ninterest_ledger = 1\n",
                "interest_ledger must be a ledger name or a list of ledger names",
            ),
            (
                "[\"Loan A\"]\nlender = \"x\"\nlender_type = \"nbfc\"\ninterest_ledger = []\n",
                "interest_ledger must be a ledger name or a list of ledger names",
            ),
            (
                "[\"Loan A\"]\nlender = \"x\"\nlender_type = \"nbfc\"\ninterest_ledger = [\"I\", \"\"]\n",
                "interest_ledger must be a ledger name or a list of ledger names",
            ),
        ] {
            let err = loan_config(&table(text)).unwrap_err();
            assert!(
                matches!(&err, AuditError::Config(m) if m.contains(needle)),
                "{text}: {err}"
            );
        }
    }

    /// Python 3.13's `re.findall(r"\d{6,}", ...)` and the mandate-word search on the cases that
    /// separate them from a naive reading (each expectation measured in Python).
    #[test]
    fn numbers_and_mandate_words_match_as_the_reference_reads_them() {
        assert_eq!(
            bank_numbers("a1234567b\u{661}\u{662}\u{663}\u{664}\u{665}\u{666}x 12345 00123456"),
            [
                "1234567",
                "\u{661}\u{662}\u{663}\u{664}\u{665}\u{666}",
                "00123456"
            ]
        );
        for (text, hit) in [
            ("ach dr", true),
            ("ACHDR", false),
            ("xach", false),
            ("\u{131}ach", false),
            ("\u{17f}ach", false),
            ("ec\u{17f} 1", true),
            ("em\u{130}", true),
            ("NACH-1", true),
            ("nACH", true),
            ("ACH\u{130}", false),
            ("\u{212a}ACH", false),
            ("ACH\u{212a}", false),
            ("EMI", true),
            ("", false),
        ] {
            assert_eq!(has_mandate_word(text), hit, "{text:?}");
        }
    }

    fn book(vouchers: Vec<Voucher>) -> Book {
        let ledger = |name: &str, group: &str| Ledger {
            name: name.to_string(),
            parent: group.to_string(),
            chain: vec![group.to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: String::new(),
            masterid: None,
        };
        Book {
            company_name: "Invented".to_string(),
            company_guid: "invented".to_string(),
            read_at: String::new(),
            groups: BTreeMap::new(),
            group_masters: BTreeMap::new(),
            ledgers: [
                ledger("Loan A", "Unsecured Loans"),
                ledger("Cash", "Cash-in-Hand"),
            ]
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect(),
            vouchers,
            tb: BTreeMap::new(),
            ..Default::default()
        }
    }

    fn taken(guid: &str, day: &str) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse(day.to_string()).unwrap(),
            vtype: "Receipt".to_string(),
            base_type: "Receipt".to_string(),
            number: String::new(),
            status: VoucherStatus::Regular,
            lines: vec![
                LedgerLine {
                    ledger: "Loan A".to_string(),
                    amount_paise: -2_500_000,
                },
                LedgerLine {
                    ledger: "Cash".to_string(),
                    amount_paise: 2_500_000,
                },
            ],
            ..Default::default()
        }
    }

    fn run_on(b: &Book, rules: &Rules) -> Result<TestResult> {
        let loans = loan_config(&table(
            "[\"Loan A\"]\nlender = \"x\"\nlender_type = \"person\"\n",
        ))
        .unwrap();
        let cash: BTreeSet<String> = ["Cash".to_string()].into_iter().collect();
        let none = BTreeSet::new();
        run(
            b,
            rules,
            "firm",
            &loans,
            &Inputs {
                previous_year_turnover_paise: None,
                cash: &cash,
                bank: &none,
                shared_interest_ledgers: &none,
                tds_payable_ledgers: &none,
                deductor_activity: None,
                turnover_is_placeholder: false,
            },
        )
    }

    #[test]
    fn two_vouchers_sharing_a_row_figure_id_are_refused() {
        // Both are reportable cash receipts on one loan with one GUID: the reference raises on the
        // repeated figure id, and this refuses rather than panicking.
        let rules = Rules::vendored().unwrap();
        let dup = book(vec![taken("g1", "20250601"), taken("g1", "20250602")]);
        let err = run_on(&dup, &rules).unwrap_err();
        assert!(
            matches!(&err, AuditError::DuplicateFigureId(id) if id.contains(".clause31_row_amount_")),
            "{err}"
        );
        // The control: distinct GUIDs give two rows.
        let two = book(vec![taken("g1", "20250601"), taken("g2", "20250602")]);
        assert_eq!(run_on(&two, &rules).unwrap().findings.len(), 2);
    }

    /// Without `[tds_rates].s194a_bp` coverage is not judged: no expected or covering figure, the
    /// coverage figure says so, and the s.194A finding carries the reference's limit for it.
    #[test]
    fn coverage_is_not_judged_without_the_s194a_rate() {
        let mut rules = Rules::vendored().unwrap();
        if let Some(t) = rules.tds_rates.as_mut() {
            t.s194a_bp = None;
        }
        let mut b = book(vec![Voucher {
            guid: "i1".to_string(),
            date: TallyDate::parse("20250930".to_string()).unwrap(),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            status: VoucherStatus::Regular,
            lines: vec![
                LedgerLine {
                    ledger: "Interest A".to_string(),
                    amount_paise: 1_200_000,
                },
                LedgerLine {
                    ledger: "Loan A".to_string(),
                    amount_paise: -1_200_000,
                },
            ],
            ..Default::default()
        }]);
        b.ledgers.insert(
            "Interest A".to_string(),
            Ledger {
                name: "Interest A".to_string(),
                parent: "Indirect Expenses".to_string(),
                chain: vec!["Indirect Expenses".to_string()],
                chain_complete: true,
                master_opening_paise: 0,
                pan: String::new(),
                gstin: String::new(),
                guid: String::new(),
                masterid: None,
            },
        );
        let loans = loan_config(&table(
            "[\"Loan A\"]\nlender = \"x\"\nlender_type = \"nbfc\"\ninterest_ledger = \"Interest A\"\n",
        ))
        .unwrap();
        let none = BTreeSet::new();
        let r = run(
            &b,
            &rules,
            "firm",
            &loans,
            &Inputs {
                previous_year_turnover_paise: None,
                cash: &none,
                bank: &none,
                shared_interest_ledgers: &none,
                tds_payable_ledgers: &none,
                deductor_activity: None,
                turnover_is_placeholder: false,
            },
        )
        .unwrap();
        let h = stable_ledger_tag(&b, "Loan A").unwrap();
        let value = |name: &str| {
            r.figures
                .iter()
                .find(|f| f.id == format!("{TEST_ID}.{name}_{h}"))
                .map(|f| f.value.clone())
        };
        assert_eq!(
            value("s194a_tds_coverage"),
            Some(Value::Text("not judged".to_string()))
        );
        assert_eq!(value("s194a_tds_expected"), None);
        assert_eq!(value("s194a_tds_covering"), None);
        let finding = r
            .findings
            .iter()
            .find(|f| f.id == format!("{TEST_ID}/s194a/{h}"))
            .unwrap();
        assert!(finding
            .limits
            .iter()
            .any(|l| l.starts_with("The rules carry no s.194A rate")));
    }

    /// Interest lines the book reads (each within i64) whose total leaves i64 are refused with the
    /// crate's typed overflow error, never a wrapped figure or a panic.
    #[test]
    fn interest_past_i64_is_refused_not_wrapped() {
        let rules = Rules::vendored().unwrap();
        let journal = |guid: &str| Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse("20250930".to_string()).unwrap(),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            status: VoucherStatus::Regular,
            lines: vec![
                LedgerLine {
                    ledger: "Interest A".to_string(),
                    amount_paise: 5_000_000_000_000_000_000,
                },
                LedgerLine {
                    ledger: "Loan A".to_string(),
                    amount_paise: -5_000_000_000_000_000_000,
                },
            ],
            ..Default::default()
        };
        let mut b = book(vec![journal("i1"), journal("i2")]);
        b.ledgers.insert(
            "Interest A".to_string(),
            Ledger {
                name: "Interest A".to_string(),
                parent: "Indirect Expenses".to_string(),
                chain: vec!["Indirect Expenses".to_string()],
                chain_complete: true,
                master_opening_paise: 0,
                pan: String::new(),
                gstin: String::new(),
                guid: String::new(),
                masterid: None,
            },
        );
        let loans = loan_config(&table(
            "[\"Loan A\"]\nlender = \"x\"\nlender_type = \"nbfc\"\ninterest_ledger = \"Interest A\"\n",
        ))
        .unwrap();
        let none = BTreeSet::new();
        let err = run(
            &b,
            &rules,
            "firm",
            &loans,
            &Inputs {
                previous_year_turnover_paise: None,
                cash: &none,
                bank: &none,
                shared_interest_ledgers: &none,
                tds_payable_ledgers: &none,
                deductor_activity: None,
                turnover_is_placeholder: false,
            },
        )
        .unwrap_err();
        assert!(
            matches!(&err, AuditError::Config(m) if m.contains("overflowed")),
            "{err}"
        );
    }

    #[test]
    fn rules_without_s194a_are_refused() {
        let mut rules = Rules::vendored().unwrap();
        rules.s194a = None;
        let err = run_on(&book(vec![]), &rules).unwrap_err();
        assert!(
            matches!(&err, AuditError::Config(m) if m.contains("needs rules [s194a]")),
            "{err}"
        );
    }

    /// A Journal on 2025-06-01 with no number, so its label is "Journal <guid> on 2025-06-01".
    fn journal(guid: &str, lines: &[(&str, i64)]) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse("20250601".to_string()).unwrap(),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            status: VoucherStatus::Regular,
            lines: lines
                .iter()
                .map(|&(ledger, amount_paise)| LedgerLine {
                    ledger: ledger.to_string(),
                    amount_paise,
                })
                .collect(),
            ..Default::default()
        }
    }

    /// Cash Dr 40,000 / Loan A Cr 40,000; Loan A Dr 25,000 / Cash Cr 25,000: one voucher, both
    /// sides of the loan, listed.
    fn both_sides(guid: &str) -> Voucher {
        journal(
            guid,
            &[
                ("Cash", 4_000_000),
                ("Loan A", -4_000_000),
                ("Loan A", 2_500_000),
                ("Cash", -2_500_000),
            ],
        )
    }

    fn loan4(b: &Book, r: &TestResult) -> Vec<String> {
        check_invariants(b, r)
            .unwrap()
            .into_iter()
            .filter(|x| x.starts_with("LOAN-4"))
            .collect()
    }

    fn gross_side<'a>(r: &'a mut TestResult, side: &str) -> &'a mut crate::findings::Figure {
        let prefix = format!("{TEST_ID}.two_sided_gross_{side}_");
        let mut it = r.figures.iter_mut().filter(|f| f.id.starts_with(&prefix));
        let f = it.next().unwrap();
        assert!(it.next().is_none(), "one listed voucher");
        f
    }

    /// The reference's LOAN-4 amount check: a listed record whose sides differ from the voucher's
    /// own lines on the loan is named, with each side as the record shows it (a missing debit
    /// figure reads as Python's `None`).
    #[test]
    fn loan4_names_a_listed_record_whose_sides_differ_from_the_voucher() {
        let rules = Rules::vendored().unwrap();
        let b = book(vec![both_sides("t1")]);
        let clean = run_on(&b, &rules).unwrap();
        assert_eq!(loan4(&b, &clean), Vec::<String>::new());
        let h = stable_ledger_tag(&b, "Loan A").unwrap();

        let mut r = clean.clone();
        gross_side(&mut r, "credit").value = Value::Int(1);
        assert_eq!(
            loan4(&b, &r),
            [format!(
                "LOAN-4: voucher Journal t1 on 2025-06-01 (guid t1) credits loan ledger 'Loan A' \
(tag {h}) 4000000p and debits it 2500000p, but its listed record shows 1p and 2500000p."
            )]
        );

        let mut r = clean.clone();
        let debit_id = gross_side(&mut r, "debit").id.clone();
        r.figures.retain(|f| f.id != debit_id);
        assert_eq!(
            loan4(&b, &r),
            [format!(
                "LOAN-4: voucher Journal t1 on 2025-06-01 (guid t1) credits loan ledger 'Loan A' \
(tag {h}) 4000000p and debits it 2500000p, but its listed record shows 4000000p and Nonep."
            )]
        );
    }

    /// The reference's `test_loan4_names_an_unlisted_unbalanced_voucher_on_the_loan_alone`: with
    /// the listed record's figures removed, the voucher (on the loan alone, not balancing) is named
    /// as cited by no listed record.
    #[test]
    fn loan4_names_an_unlisted_unbalanced_voucher_on_the_loan_alone() {
        let rules = Rules::vendored().unwrap();
        let b = book(vec![journal(
            "l1",
            &[("Loan A", -500_000), ("Loan A", 300_000)],
        )]);
        let mut r = run_on(&b, &rules).unwrap();
        assert_eq!(loan4(&b, &r), Vec::<String>::new());
        let before = r.figures.len();
        r.figures.retain(|f| !f.id.contains("two_sided_gross_"));
        assert_eq!(before - r.figures.len(), 2);
        let h = stable_ledger_tag(&b, "Loan A").unwrap();
        assert_eq!(
            loan4(&b, &r),
            [format!(
                "LOAN-4: voucher Journal l1 on 2025-06-01 (guid l1) both credits (500000p) and \
debits (300000p) loan ledger 'Loan A' (tag {h}), and is not an interest or TDS entry, but no listed \
record cites it: it was netted into one entry or dropped."
            )]
        );
    }

    /// The reference's LOAN-4 orphan check: a listed record that also cites a voucher in the
    /// population which does not both credit and debit the loan is named.
    #[test]
    fn loan4_names_a_listed_record_citing_a_one_sided_voucher() {
        let rules = Rules::vendored().unwrap();
        let b = book(vec![both_sides("t1"), taken("t2", "20250701")]);
        let mut r = run_on(&b, &rules).unwrap();
        assert_eq!(loan4(&b, &r), Vec::<String>::new());
        gross_side(&mut r, "credit")
            .evidence
            .push(EvidenceRef::new("voucher", "t2"));
        let h = stable_ledger_tag(&b, "Loan A").unwrap();
        assert_eq!(
            loan4(&b, &r),
            [format!(
                "LOAN-4: a listed record on loan ledger 'Loan A' (tag {h}) cites voucher guid t2, \
which does not both credit and debit the loan in the books population."
            )]
        );
    }
}
