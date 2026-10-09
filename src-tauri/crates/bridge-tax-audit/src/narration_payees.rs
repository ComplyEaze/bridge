//! Payees named only in the bank narration, against the s.194C limits: the bank payments debited to
//! the ledgers the client lists (and to the 194C ledgers whose payees the TDS payee test could not
//! name), the payee the bank printed on each, payments joined into payees by a shared printed name
//! or UPI handle, and each payee's largest payment and total against the two limits. A port of the
//! reference Python implementation's `narration_payees` test module, version 1; its contract is the
//! spec pack in `docs/tax-audit/spec-packs/narration_payees/` (cited below as "README section N").
//!
//! It reaches no conclusion: every finding is a question. Its narration reader (README section 3)
//! is this module's own. `bridge-bank-statement`'s `hdfc_party`/`sbi_party` read statement rows and
//! differ on purpose (#1428).

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::book::{Book, Voucher};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Figure, Finding, TestResult, Unit, Value};
use crate::read::iso;
use crate::rules::Rules;
use crate::support::{count, hash8, overflow, py_is_decimal, py_repr_str, py_split, py_upper};
use crate::tds_payees;

pub const TEST_ID: &str = "narration_payees";
pub const VERSION: &str = "1";

/// Where forms 1 to 3 print a name, these words alone are no name (README section 3.1).
const CHANNEL_WORDS: [&str; 4] = ["UPI", "IMPS", "NEFT", "RTGS"];
/// Printed in the second UPI layout's name field in place of a payee (README section 3.2).
const PLACEHOLDERS: [&str; 2] = ["BANKACC", "PHONEPE"];
/// A cheque name beginning with one of these words is the assessee's own (README section 3.2).
const SELF_CHEQUES: [&str; 2] = ["SELF", "CASH PAID TO"];
/// Debits to ledgers under these groups are not debits to other ledgers (README section 6.4).
const NOT_OTHER_GROUPS: [&str; 4] = [
    "Bank Accounts",
    "Bank OD A/c",
    "Cash-in-Hand",
    "Duties & Taxes",
];

const POPULATION_NOTE: &str = "Books population (optional, cancelled and post-dated vouchers \
excluded): debits to the ledgers the client lists, and any 194C-mapped ledger whose payees the TDS \
payee test could not name, when that test's finding lists them; bank payments read for the printed \
payee.";

const DEDUCTOR_LIMIT: &str = "Tax was deductible only if the assessee was a deductor for the year \
and the payee a contractor, not an employee.";

/// How the bank paid a payee, as a figure value names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Channel {
    Upi,
    Imps,
    Neft,
    Rtgs,
    ChequeCounter,
}

impl Channel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Upi => "UPI",
            Self::Imps => "IMPS",
            Self::Neft => "NEFT",
            Self::Rtgs => "RTGS",
            Self::ChequeCounter => "CHEQUE_COUNTER",
        }
    }
}

/// A UPI handle as the layout printed it (README section 3.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Handle {
    Whole(String),
    /// The layout printed only the handle's beginning.
    Cut(String),
}

/// A payee read out of a narration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Printed {
    pub name: String,
    pub channel: Channel,
    pub handle: Option<Handle>,
}

/// What a narration gives (README section 3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reading {
    Payee(Printed),
    /// The second UPI layout printed `Bank Acc` or `PhonePe` in place of the payee.
    Placeholder,
    NoPayee,
}

impl Reading {
    fn payee(&self) -> Option<&Printed> {
        match self {
            Self::Payee(p) => Some(p),
            _ => None,
        }
    }
}

fn az(c: char) -> bool {
    c.is_ascii_uppercase()
}

/// `lit` at `i`: the index after it.
fn lit(c: &[char], i: usize, lit: &str) -> Option<usize> {
    let mut j = i;
    for want in lit.chars() {
        if c.get(j) != Some(&want) {
            return None;
        }
        j += 1;
    }
    Some(j)
}

/// One optional `ch` at `i`.
fn opt(c: &[char], i: usize, ch: char) -> usize {
    if c.get(i) == Some(&ch) {
        i + 1
    } else {
        i
    }
}

/// The end of a run of one or more characters from `i` that `keep` accepts.
fn run1(c: &[char], i: usize, keep: impl Fn(char) -> bool) -> Option<usize> {
    let end = i + c[i.min(c.len())..].iter().take_while(|&&x| keep(x)).count();
    (end > i).then_some(end)
}

/// The name of forms 1 to 3 at `i`, and the index after its closing hyphen: A to Z, then A to Z,
/// 0 to 9, a space, `.`, `&` or `'`, then the hyphen; a space left at its end is removed (README
/// section 3.1).
fn hyphen_name(c: &[char], i: usize) -> Option<(String, usize)> {
    if !c.get(i).is_some_and(|&x| az(x)) {
        return None;
    }
    let end = i + c[i..]
        .iter()
        .take_while(|&&x| az(x) || x.is_ascii_digit() || matches!(x, ' ' | '.' | '&' | '\''))
        .count();
    if c.get(end) != Some(&'-') {
        return None;
    }
    let mut name: String = c[i..end].iter().collect();
    if name.ends_with(' ') {
        name.pop();
    }
    Some((name, end + 1))
}

/// Form 1's handle, from directly after the name's hyphen: A to Z, 0 to 9, `.` or `_`, an optional
/// hyphen and decimal digits, then `@` (README section 3.3).
fn form1_handle(c: &[char], i: usize) -> Option<Handle> {
    let mut end = run1(c, i, |x| {
        az(x) || x.is_ascii_digit() || matches!(x, '.' | '_')
    })?;
    if c.get(end) == Some(&'-') {
        end = run1(c, end + 1, py_is_decimal)?;
    }
    (c.get(end) == Some(&'@')).then(|| Handle::Whole(c[i..end].iter().collect()))
}

/// Forms 1 to 3 after their opening: the name (a channel word alone names no one, and no other
/// form is tried), and form 1's handle.
fn named(c: &[char], i: usize, channel: Channel) -> Option<Reading> {
    let (name, after) = hyphen_name(c, i)?;
    if CHANNEL_WORDS.contains(&name.as_str()) {
        return Some(Reading::NoPayee);
    }
    let handle = if channel == Channel::Upi {
        form1_handle(c, after)
    } else {
        None
    };
    Some(Reading::Payee(Printed {
        name,
        channel,
        handle,
    }))
}

fn form1(c: &[char]) -> Option<Reading> {
    named(c, lit(c, 0, "UPI-")?, Channel::Upi)
}

fn form2(c: &[char]) -> Option<Reading> {
    let i = run1(c, lit(c, 0, "IMPS-")?, py_is_decimal)?;
    named(c, lit(c, i, "-")?, Channel::Imps)
}

fn form3(c: &[char]) -> Option<Reading> {
    let (i, channel) = match lit(c, 0, "NEFT") {
        Some(i) => (i, Channel::Neft),
        None => (lit(c, 0, "RTGS")?, Channel::Rtgs),
    };
    let i = lit(c, opt(c, i, ' '), "DR-")?;
    let i = run1(c, i, |x| az(x) || x.is_ascii_digit())?;
    named(c, lit(c, i, "-")?, channel)
}

/// The text of a `/`-closed field of one or more characters from `i`, every space removed, and the
/// index after its `/`.
fn slash_field(c: &[char], i: usize) -> Option<(String, usize)> {
    let end = run1(c, i, |x| x != '/')?;
    (c.get(end) == Some(&'/')).then(|| (c[i..end].iter().filter(|&&x| x != ' ').collect(), end + 1))
}

/// Form 4's handle, after the name field's `/`: a field that may be empty, then the handle field
/// (README section 3.3).
fn form4_handle(c: &[char], i: usize) -> Option<Handle> {
    let skip = i + c[i..].iter().take_while(|&&x| x != '/').count();
    let (field, _) = slash_field(c, lit(c, skip, "/")?)?;
    let handle = match field.split_once('@') {
        Some((before, _)) => Handle::Whole(before.to_string()),
        None => Handle::Cut(field),
    };
    match &handle {
        Handle::Whole(h) | Handle::Cut(h) if h.is_empty() => None,
        _ => Some(handle),
    }
}

fn form4(c: &[char]) -> Option<Reading> {
    let i = lit(c, opt(c, lit(c, 0, "TO TRANSFER-")?, ' '), "UPI/DR/")?;
    let i = lit(c, run1(c, i, |x| x == ' ' || py_is_decimal(x))?, "/")?;
    let (name, after) = slash_field(c, i)?;
    if PLACEHOLDERS.contains(&name.as_str()) {
        return Some(Reading::Placeholder);
    }
    if !name.chars().any(az) {
        return Some(Reading::NoPayee);
    }
    Some(Reading::Payee(Printed {
        handle: form4_handle(c, after),
        name,
        channel: Channel::Upi,
    }))
}

/// The end of the cheque prefix `WITHDR[A]W[A]L BY` and the spaces after it, when the text starts
/// with it and no letter A to Z follows it.
fn cheque_prefix(c: &[char]) -> Option<usize> {
    let i = lit(c, opt(c, lit(c, 0, "WITHDR")?, 'A'), "W")?;
    let i = lit(c, opt(c, i, 'A'), "L BY")?;
    if c.get(i).is_some_and(|&x| az(x)) {
        return None;
    }
    Some(i + c[i..].iter().take_while(|&&x| x == ' ').count())
}

/// Form 5, the last: a text it does not fit names no one. After the prefix it is read only from
/// the prefix's end.
fn form5(c: &[char]) -> Reading {
    let i = cheque_prefix(c).unwrap_or(0);
    if !c.get(i).is_some_and(|&x| az(x)) {
        return Reading::NoPayee;
    }
    let end = i + c[i..]
        .iter()
        .take_while(|&&x| az(x) || matches!(x, ' ' | '.'))
        .count();
    if lit(c, end, "-")
        .and_then(|j| lit(c, opt(c, j, ' '), "CHQ PAID"))
        .is_none()
    {
        return Reading::NoPayee;
    }
    let mut name: String = c[i..end].iter().collect();
    if name.ends_with(' ') {
        name.pop();
    }
    let refused = SELF_CHEQUES.iter().any(|word| {
        name.strip_prefix(word)
            .is_some_and(|rest| !rest.starts_with(az))
    });
    if refused {
        return Reading::NoPayee;
    }
    Reading::Payee(Printed {
        name,
        channel: Channel::ChequeCounter,
        handle: None,
    })
}

/// The payee a narration names (README section 3): the text upper-cased with Python's tables, its
/// whitespace runs made one space, then the five forms tried in order from its first character.
pub fn read_narration(narration: &str) -> Reading {
    let text = py_split(&py_upper(narration)).join(" ");
    let c: Vec<char> = text.chars().collect();
    form1(&c)
        .or_else(|| form2(&c))
        .or_else(|| form3(&c))
        .or_else(|| form4(&c))
        .unwrap_or_else(|| form5(&c))
}

/// `tds_payees`' finding for 194C payments it could tie to no named payee (README section 2.4).
fn unnamed_finding_id(book: &Book) -> Result<String> {
    Ok(format!(
        "{}/194C_{}",
        tds_payees::TEST_ID,
        tds_payees::row_hash(book, "194C", tds_payees::PAYEE_NOT_NAMED)?
    ))
}

/// The ledgers read besides those listed (README section 2.4): on every population voucher whose
/// GUID `tds_payees`' finding for unnamed 194C payees cites, each ledger mapped exactly `194C` with
/// a debit line.
///
/// # Errors
///
/// When the population cannot be formed, or a tag cannot be derived.
pub fn unnamed_194c_ledgers(
    book: &Book,
    tds: &TestResult,
    nature_by_ledger: &BTreeMap<String, String>,
) -> Result<BTreeSet<String>> {
    let id = unnamed_finding_id(book)?;
    let Some(finding) = tds.findings.iter().find(|f| f.id == id) else {
        return Ok(BTreeSet::new());
    };
    let cited: BTreeSet<&str> = finding
        .evidence
        .iter()
        .filter(|e| e.kind == "voucher")
        .map(|e| e.id.as_str())
        .collect();
    Ok(book
        .population()?
        .into_iter()
        .filter(|v| cited.contains(v.guid.as_str()))
        .flat_map(|v| &v.lines)
        .filter(|l| l.amount_paise > 0)
        .filter(|l| nature_by_ledger.get(&l.ledger).map(String::as_str) == Some("194C"))
        .map(|l| l.ledger.clone())
        .collect())
}

/// A voucher's citation, `<type name> <number> on <date>`, its number as is (README section 6.6).
fn voucher_ref(v: &Voucher) -> EvidenceRef {
    EvidenceRef::with_label(
        "voucher",
        &v.guid,
        &format!("{} {} on {}", v.vtype, v.number, iso(&v.date)),
    )
}

/// The citations of `vouchers`, one per GUID and label, in first-cited order.
fn voucher_refs<'a>(vouchers: impl IntoIterator<Item = &'a Voucher>) -> Vec<EvidenceRef> {
    let mut seen = BTreeSet::new();
    vouchers
        .into_iter()
        .map(voucher_ref)
        .filter(|r| seen.insert((r.id.clone(), r.label.clone())))
        .collect()
}

fn name_ref(shown: &str) -> EvidenceRef {
    EvidenceRef::with_label("payee_name", shown, shown)
}

fn add(a: i64, b: i64) -> Result<i64> {
    a.checked_add(b).ok_or_else(|| overflow(TEST_ID))
}

fn total<'a>(amounts: impl IntoIterator<Item = &'a i64>) -> Result<i64> {
    amounts.into_iter().try_fold(0, |acc, &a| add(acc, a))
}

/// The ledgers and bank set a walk reads with.
struct Ledgers<'a> {
    bank: &'a BTreeSet<String>,
    read: BTreeSet<String>,
}

impl Ledgers<'_> {
    /// The voucher's debits to other ledgers, and those ledgers (README section 6.4).
    fn other_debits<'v>(&self, book: &Book, v: &'v Voucher) -> Result<(i64, Vec<&'v str>)> {
        if v.base_type == "Contra" {
            return Ok((0, Vec::new()));
        }
        let mut sum = 0;
        let mut ledgers = Vec::new();
        for l in v.lines.iter().filter(|l| l.amount_paise > 0) {
            let excluded = self.read.contains(&l.ledger)
                || self.bank.contains(&l.ledger)
                || book
                    .ledgers
                    .get(&l.ledger)
                    .is_some_and(|m| NOT_OTHER_GROUPS.iter().any(|g| m.under(g)));
            if !excluded {
                sum = add(sum, l.amount_paise)?;
                ledgers.push(l.ledger.as_str());
            }
        }
        Ok((sum, ledgers))
    }

    fn amount(&self, v: &Voucher) -> Result<i64> {
        total(
            v.lines
                .iter()
                .filter(|l| l.amount_paise > 0 && self.read.contains(&l.ledger))
                .map(|l| &l.amount_paise),
        )
    }

    fn bank_leg(&self, v: &Voucher) -> bool {
        v.lines
            .iter()
            .any(|l| l.amount_paise < 0 && self.bank.contains(&l.ledger))
    }
}

/// An outgoing bank payment whose narration gives a payee.
struct Payment<'a> {
    v: &'a Voucher,
    /// Zero when it debits no ledger read: it joins payees but is in no row.
    amount: i64,
    printed: Printed,
    /// The handle it joins by: a whole one, or a cut one completed (README section 5.2).
    joins_by: Option<String>,
    /// Its cut handle, when completion failed.
    cut_unjoined: Option<String>,
}

/// One payee: payments joined by a shared name or handle.
struct Payee {
    key: String,
    tag: String,
    /// Indices into the walk's payments, in population order.
    members: Vec<usize>,
}

/// A payee's row: its payments with an amount (README section 5.4).
struct Row {
    payee: usize,
    payments: Vec<usize>,
    total: i64,
    max: i64,
    shown: String,
}

/// What a walk of the population finds (README sections 4 and 5).
struct Walk<'a> {
    unresolved: Vec<&'a Voucher>,
    placeholder: Vec<&'a Voucher>,
    not_through: Vec<&'a Voucher>,
    unresolved_total: i64,
    placeholder_total: i64,
    not_through_total: i64,
    payments: Vec<Payment<'a>>,
    /// Every whole handle printed on a payment, with the names printed with it.
    whole: BTreeMap<String, BTreeSet<String>>,
    payees: Vec<Payee>,
    /// Payment index -> payee index.
    payee_of: Vec<usize>,
    /// By total, largest first, ties by key.
    rows: Vec<Row>,
    /// Population position -> payment index, for every outgoing bank payment with a payee.
    payment_at: HashMap<usize, usize>,
    /// Population positions of the outgoing bank payments (a bank leg), with or without a payee.
    outgoing: BTreeSet<usize>,
}

/// Whether two printed names agree once their spaces are removed: one starts with the other.
fn names_agree(a: &str, b: &str) -> bool {
    let (a, b): (String, String) = (
        a.chars().filter(|&c| c != ' ').collect(),
        b.chars().filter(|&c| c != ' ').collect(),
    );
    a.starts_with(&b) || b.starts_with(&a)
}

fn find(parent: &mut [usize], i: usize) -> usize {
    let mut r = i;
    while parent[r] != r {
        r = parent[r];
    }
    let mut j = i;
    while parent[j] != r {
        let next = parent[j];
        parent[j] = r;
        j = next;
    }
    r
}

fn union(parent: &mut [usize], a: usize, b: usize) {
    let (ra, rb) = (find(parent, a), find(parent, b));
    if ra != rb {
        parent[ra.max(rb)] = ra.min(rb);
    }
}

/// The whole handles starting with `cut`: its candidates (README section 5.2).
fn candidates<'p>(whole: &'p BTreeMap<String, BTreeSet<String>>, cut: &str) -> Vec<&'p String> {
    whole.keys().filter(|w| w.starts_with(cut)).collect()
}

#[allow(clippy::too_many_lines)]
fn walk<'a>(pop: &[&'a Voucher], ledgers: &Ledgers<'_>) -> Result<Walk<'a>> {
    let mut w = Walk {
        unresolved: Vec::new(),
        placeholder: Vec::new(),
        not_through: Vec::new(),
        unresolved_total: 0,
        placeholder_total: 0,
        not_through_total: 0,
        payments: Vec::new(),
        whole: BTreeMap::new(),
        payees: Vec::new(),
        payee_of: Vec::new(),
        rows: Vec::new(),
        payment_at: HashMap::new(),
        outgoing: BTreeSet::new(),
    };
    for (at, &v) in pop.iter().enumerate() {
        let amount = ledgers.amount(v)?;
        let bank_leg = ledgers.bank_leg(v);
        if !bank_leg {
            if amount != 0 {
                w.not_through_total = add(w.not_through_total, amount)?;
                w.not_through.push(v);
            }
            continue;
        }
        w.outgoing.insert(at);
        match read_narration(&v.narration) {
            Reading::Payee(printed) => {
                w.payment_at.insert(at, w.payments.len());
                w.payments.push(Payment {
                    v,
                    amount,
                    printed,
                    joins_by: None,
                    cut_unjoined: None,
                });
            }
            reading if amount != 0 => {
                w.unresolved_total = add(w.unresolved_total, amount)?;
                w.unresolved.push(v);
                if reading == Reading::Placeholder {
                    w.placeholder_total = add(w.placeholder_total, amount)?;
                    w.placeholder.push(v);
                }
            }
            _ => {}
        }
    }

    // Every whole handle printed, with the names printed with it (README section 5.2).
    for p in &w.payments {
        if let Some(Handle::Whole(h)) = &p.printed.handle {
            w.whole
                .entry(h.clone())
                .or_default()
                .insert(p.printed.name.clone());
        }
    }
    for p in &mut w.payments {
        match &p.printed.handle {
            Some(Handle::Whole(h)) => p.joins_by = Some(h.clone()),
            Some(Handle::Cut(c)) => match candidates(&w.whole, c).as_slice() {
                [only]
                    if w.whole[*only]
                        .iter()
                        .any(|n| names_agree(n, &p.printed.name)) =>
                {
                    p.joins_by = Some((*only).clone());
                }
                _ => p.cut_unjoined = Some(c.clone()),
            },
            None => {}
        }
    }

    // Join by name and by handle, through chains (README section 5.1).
    let n = w.payments.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut first_by: HashMap<(bool, &str), usize> = HashMap::new();
    for (i, p) in w.payments.iter().enumerate() {
        let keys = std::iter::once((false, p.printed.name.as_str()))
            .chain(p.joins_by.as_deref().map(|h| (true, h)));
        for k in keys {
            match first_by.get(&k) {
                Some(&j) => union(&mut parent, i, j),
                None => {
                    first_by.insert(k, i);
                }
            }
        }
    }
    let mut payee_of_root: HashMap<usize, usize> = HashMap::new();
    w.payee_of = vec![0; n];
    for i in 0..n {
        let root = find(&mut parent, i);
        let payee = *payee_of_root.entry(root).or_insert_with(|| {
            w.payees.push(Payee {
                key: String::new(),
                tag: String::new(),
                members: Vec::new(),
            });
            w.payees.len() - 1
        });
        w.payees[payee].members.push(i);
        w.payee_of[i] = payee;
    }
    for payee in &mut w.payees {
        let members = || payee.members.iter().map(|&i| &w.payments[i]);
        payee.key = match members().filter_map(|p| p.joins_by.as_ref()).min() {
            Some(h) => format!("upi:{h}"),
            None => members()
                .map(|p| p.printed.name.clone())
                .min()
                .unwrap_or_default(),
        };
        payee.tag = hash8(&payee.key);
    }

    // Rows (README section 5.4).
    for (pi, payee) in w.payees.iter().enumerate() {
        let payments: Vec<usize> = payee
            .members
            .iter()
            .copied()
            .filter(|&i| w.payments[i].amount != 0)
            .collect();
        if payments.is_empty() {
            continue;
        }
        let total = total(payments.iter().map(|&i| &w.payments[i].amount))?;
        let max = payments
            .iter()
            .map(|&i| w.payments[i].amount)
            .max()
            .unwrap_or(0);
        let names: BTreeSet<&str> = payments
            .iter()
            .map(|&i| w.payments[i].printed.name.as_str())
            .collect();
        w.rows.push(Row {
            payee: pi,
            total,
            max,
            shown: names.into_iter().collect::<Vec<_>>().join(" / "),
            payments,
        });
    }
    let payees = &w.payees;
    w.rows.sort_by(|a, b| {
        b.total
            .cmp(&a.total)
            .then_with(|| payees[a.payee].key.cmp(&payees[b.payee].key))
    });
    Ok(w)
}

/// A finding of this test: its id under `narration_payees/`, its facts naming this test's figures.
#[allow(clippy::too_many_arguments)]
fn finding(
    id: &str,
    clauses: &[&str],
    title: &str,
    facts: &[(&str, &str)],
    evidence: Vec<EvidenceRef>,
    confidence: Confidence,
    limits: &[&str],
    ask_client: &[&str],
) -> Finding {
    Finding {
        id: format!("{TEST_ID}/{id}"),
        clauses: clauses.iter().map(|c| (*c).to_string()).collect(),
        title: title.to_string(),
        facts: facts
            .iter()
            .map(|(n, f)| ((*n).to_string(), format!("{TEST_ID}.{f}")))
            .collect(),
        evidence,
        confidence,
        limits: limits.iter().map(|l| (*l).to_string()).collect(),
        ask_client: ask_client.iter().map(|a| (*a).to_string()).collect(),
    }
}

/// Run the test. `bank` is the bank ledgers (README section 2.2), `listed` the ledgers the client
/// lists and `added` those [`unnamed_194c_ledgers`] adds; both are read alike.
///
/// # Errors
///
/// Rules with no `[s194c]`; a voucher of unknown status; a total that overflows; two payees, or two
/// cut handles, with one tag (`DuplicateFigureId`, as the reference raises).
#[allow(clippy::too_many_lines)]
pub fn run(
    book: &Book,
    rules: &Rules,
    bank: &BTreeSet<String>,
    listed: &BTreeSet<String>,
    added: &BTreeSet<String>,
) -> Result<TestResult> {
    let limits = rules
        .s194c
        .ok_or_else(|| AuditError::Config(format!("{TEST_ID} needs rules [s194c]")))?;
    let ledgers = Ledgers {
        bank,
        read: listed.union(added).cloned().collect(),
    };
    let pop = book.population()?;
    let w = walk(&pop, &ledgers)?;
    let over = |row: &Row| row.max > limits.single_sum_paise || row.total > limits.aggregate_paise;
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.population_note = POPULATION_NOTE.to_string();
    let vouchers_of =
        |ps: &[usize]| -> Vec<&Voucher> { ps.iter().map(|&i| w.payments[i].v).collect() };

    r.fig(
        "configured_ledgers_count",
        count(TEST_ID, listed.len())?,
        Unit::Count,
        "Ledgers the client lists as paid to payees named only in the bank narration.",
        Vec::new(),
    )?;
    r.fig(
        "added_ledgers_count",
        count(TEST_ID, added.difference(listed).count())?,
        Unit::Count,
        "194C-mapped ledgers read besides those the client lists, because the TDS payee test could \
not name the payee of their payments (its finding lists them; on a book whose vouchers share a \
blank or repeated identifier, a ledger debited on another voucher with the same identifier too). \
With no ledger listed or added, nothing was read.",
        Vec::new(),
    )?;

    // Each row's five figures, rows by total, largest first (README section 6.2).
    let mut over_payments: Vec<usize> = Vec::new();
    let mut over_count = 0;
    let mut over_total = 0;
    for row in &w.rows {
        let tag = &w.payees[row.payee].tag;
        let name = name_ref(&row.shown);
        let channels: BTreeSet<&str> = row
            .payments
            .iter()
            .map(|&i| w.payments[i].printed.channel.as_str())
            .collect();
        r.fig(
            &format!("payee_channel_{tag}"),
            Value::Text(channels.into_iter().collect::<Vec<_>>().join(", ")),
            Unit::Text,
            "How the bank paid this payee (UPI, IMPS, NEFT/RTGS, or a cheque encashed at the \
counter).",
            vec![name.clone()],
        )?;
        r.fig(
            &format!("payee_count_{tag}"),
            count(TEST_ID, row.payments.len())?,
            Unit::Count,
            "Bank payments to this payee on the ledgers read.",
            vec![name.clone()],
        )?;
        let mut evidence = vec![name.clone()];
        evidence.extend(voucher_refs(vouchers_of(&row.payments)));
        r.fig(
            &format!("payee_total_{tag}"),
            Value::Int(row.total),
            Unit::Paise,
            "Total of those payments.",
            evidence,
        )?;
        r.fig(
            &format!("payee_max_{tag}"),
            Value::Int(row.max),
            Unit::Paise,
            "Largest single payment.",
            vec![name.clone()],
        )?;
        r.fig(
            &format!("payee_over_194c_{tag}"),
            Value::Text(if over(row) { "yes" } else { "no" }.to_string()),
            Unit::Text,
            "Whether a single payment exceeds the s.194C single-sum limit or the year's total \
exceeds the aggregate limit.",
            vec![name],
        )?;
        if over(row) {
            over_count += 1;
            over_total = add(over_total, row.total)?;
            over_payments.extend(&row.payments);
        }
    }
    r.fig(
        "payees_count",
        count(TEST_ID, w.rows.len())?,
        Unit::Count,
        "Distinct payees printed by the bank: payments sharing a printed name or a whole UPI \
handle are one payee.",
        Vec::new(),
    )?;
    r.fig(
        "payees_over_194c_count",
        count(TEST_ID, over_count)?,
        Unit::Count,
        "Of those, payees over a s.194C threshold.",
        Vec::new(),
    )?;
    r.fig(
        "payees_over_194c_total",
        Value::Int(over_total),
        Unit::Paise,
        "Total paid to the payees over a threshold.",
        voucher_refs(vouchers_of(&over_payments)),
    )?;
    r.fig(
        "resolved_total",
        Value::Int(total(w.rows.iter().map(|row| &row.total))?),
        Unit::Paise,
        "Total bank payments with a printed payee.",
        Vec::new(),
    )?;
    r.fig(
        "unresolved_bank_total",
        Value::Int(w.unresolved_total),
        Unit::Paise,
        "Bank payments on the ledgers read whose narration names no payee that can be read (a \
placeholder in place of the payee included).",
        voucher_refs(w.unresolved.iter().copied()),
    )?;
    r.fig(
        "placeholder_payee_total",
        Value::Int(w.placeholder_total),
        Unit::Paise,
        "Bank payments on the ledgers read where the bank printed a placeholder (such as \"Bank \
Acc\" or \"PhonePe\") in place of the payee: not read as any payee.",
        voucher_refs(w.placeholder.iter().copied()),
    )?;
    r.fig(
        "not_through_bank_total",
        Value::Int(w.not_through_total),
        Unit::Paise,
        "Debits on the ledgers read with no bank leg (cash or journal): no payee is read from a \
bank narration for them.",
        voucher_refs(w.not_through.iter().copied()),
    )?;

    // Same day (README section 6.3).
    let mut day_payments: Vec<usize> = Vec::new();
    let (mut day_count, mut day_total) = (0, 0);
    for row in &w.rows {
        let tag = &w.payees[row.payee].tag;
        let mut days: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for &i in &row.payments {
            days.entry(iso(&w.payments[i].v.date)).or_default().push(i);
        }
        for (day, ps) in days {
            let sum = total(ps.iter().map(|&i| &w.payments[i].amount))?;
            if ps.len() < 2 || sum <= limits.single_sum_paise {
                continue;
            }
            let mut evidence = vec![name_ref(&row.shown)];
            evidence.extend(voucher_refs(vouchers_of(&ps)));
            r.fig(
                &format!("same_day_{tag}_{day}"),
                Value::Int(sum),
                Unit::Paise,
                &format!(
                    "Bank payments to one payee on {day}, together: two or more payments, their \
total over the s.194C single-sum limit."
                ),
                evidence,
            )?;
            day_count += 1;
            day_total = add(day_total, sum)?;
            day_payments.extend(ps);
        }
    }
    r.fig(
        "same_day_count",
        count(TEST_ID, day_count)?,
        Unit::Count,
        "Payees paid two or more times on one day, the day's total over the s.194C single-sum \
limit, counted once per payee and day.",
        Vec::new(),
    )?;
    r.fig(
        "same_day_total",
        Value::Int(day_total),
        Unit::Paise,
        "The day totals of every payee paid more than once on one day, over the s.194C single-sum \
limit, added together.",
        voucher_refs(vouchers_of(&day_payments)),
    )?;

    // Debits to other ledgers (README section 6.4).
    let mut crossing_vouchers: Vec<&Voucher> = Vec::new();
    let mut crossing_evidence: Vec<&Voucher> = Vec::new();
    let (mut crossing_count, mut crossing_total) = (0, 0);
    for row in &w.rows {
        let payee = &w.payees[row.payee];
        let (mut sum, mut largest) = (0, 0);
        let mut vouchers: Vec<&Voucher> = Vec::new();
        let mut debited: BTreeSet<&str> = BTreeSet::new();
        for &i in &payee.members {
            let v = w.payments[i].v;
            let (other, on) = ledgers.other_debits(book, v)?;
            if other > 0 {
                sum = add(sum, other)?;
                largest = largest.max(other);
                vouchers.push(v);
                debited.extend(on);
            }
        }
        if vouchers.is_empty() {
            continue;
        }
        let mut evidence = voucher_refs(vouchers.iter().copied());
        evidence.extend(debited.iter().map(|l| EvidenceRef::new("ledger", l)));
        evidence.push(name_ref(&row.shown));
        r.fig(
            &format!("payee_elsewhere_{}", payee.tag),
            Value::Int(sum),
            Unit::Paise,
            "Debits to other ledgers on outgoing bank payments to this payee -- ledgers other than \
those read, a bank or cash ledger, or a duties ledger (the vouchers, and the ledgers debited): \
listed beside the s.194C limits, not added into them.",
            evidence,
        )?;
        let crossing = !over(row)
            && (add(row.total, sum)? > limits.aggregate_paise || largest > limits.single_sum_paise);
        if crossing {
            crossing_count += 1;
            crossing_total = add(crossing_total, sum)?;
            crossing_vouchers.extend(&vouchers);
            crossing_evidence.extend(&vouchers);
            crossing_evidence.extend(vouchers_of(&row.payments));
        }
    }
    r.fig(
        "elsewhere_over_count",
        count(TEST_ID, crossing_count)?,
        Unit::Count,
        "Payees under both s.194C limits on the ledgers read, whose total with their debits to \
other ledgers exceeds the aggregate limit, or one of whose payments debits other ledgers by more \
than the single-sum limit.",
        Vec::new(),
    )?;
    r.fig(
        "elsewhere_over_total",
        Value::Int(crossing_total),
        Unit::Paise,
        "The debits to other ledgers of every such payee, added together.",
        voucher_refs(crossing_vouchers),
    )?;

    // Cut handles that were not completed (README section 6.5).
    let in_row: BTreeSet<usize> = w
        .rows
        .iter()
        .flat_map(|row| row.payments.iter().copied())
        .collect();
    let row_of: HashMap<usize, &Row> = w.rows.iter().map(|row| (row.payee, row)).collect();
    let unjoined: Vec<usize> = (0..w.payments.len())
        .filter(|i| w.payments[*i].cut_unjoined.is_some() && in_row.contains(i))
        .collect();
    r.fig(
        "cut_handle_unjoined_count",
        count(TEST_ID, unjoined.len())?,
        Unit::Count,
        "Bank payments whose UPI address one bank printed cut short and no single full address \
among the bank payments completes under an agreeing name (none, several, or one printed under \
another name): joined to no other payee by it.",
        voucher_refs(vouchers_of(&unjoined)),
    )?;
    // Each cut handle not completed: the payees of its payments in a row, and of every row
    // payment that carries one of its candidates.
    let mut cuts: BTreeMap<&str, BTreeSet<usize>> = BTreeMap::new();
    for (i, p) in w.payments.iter().enumerate() {
        if let Some(c) = &p.cut_unjoined {
            let payees = cuts.entry(c.as_str()).or_default();
            if in_row.contains(&i) {
                payees.insert(w.payee_of[i]);
            }
        }
    }
    let mut group_payments: Vec<usize> = Vec::new();
    let mut groups = 0;
    for (cut, mut payees) in cuts {
        let cands: BTreeSet<&String> = candidates(&w.whole, cut).into_iter().collect();
        for &i in &in_row {
            if w.payments[i]
                .joins_by
                .as_ref()
                .is_some_and(|h| cands.contains(h))
            {
                payees.insert(w.payee_of[i]);
            }
        }
        if payees.len() < 2 {
            continue;
        }
        let rows: Vec<&Row> = payees.iter().map(|p| row_of[p]).collect();
        let names: BTreeSet<&str> = rows
            .iter()
            .flat_map(|row| {
                row.payments
                    .iter()
                    .map(|&i| w.payments[i].printed.name.as_str())
            })
            .collect();
        let ps: Vec<usize> = rows
            .iter()
            .flat_map(|row| row.payments.iter().copied())
            .collect();
        let mut evidence = vec![name_ref(&names.into_iter().collect::<Vec<_>>().join(" / "))];
        evidence.extend(voucher_refs(vouchers_of(&ps)));
        r.fig(
            &format!("possible_same_{}", hash8(cut)),
            Value::Int(total(rows.iter().map(|row| &row.total))?),
            Unit::Paise,
            "Payees whose payments carry one shortened UPI address, or the one full address that \
completes it under another printed name: not joined. If they are one person, this is that payee's \
total.",
            evidence,
        )?;
        groups += 1;
        group_payments.extend(ps);
    }
    r.fig(
        "possible_same_count",
        count(TEST_ID, groups)?,
        Unit::Count,
        "Groups of payees that a shortened UPI address might join, listed apart and never joined.",
        Vec::new(),
    )?;

    // Findings (README section 6.7).
    if !w.unresolved.is_empty() {
        r.findings.push(finding(
            "recipient_not_read",
            &["s.194C"],
            "Bank payments whose recipient the bank narration does not give: the recipient of \
each is not known",
            &[("amount", "unresolved_bank_total"), ("placeholders", "placeholder_payee_total")],
            voucher_refs(w.unresolved.iter().copied()),
            Confidence::NeedsDocument,
            &["The narration of each payment listed names no one, carries a placeholder in place \
of the payee (such as \"Bank Acc\" for a transfer to an account, or \"PhonePe\" for a bill paid \
through it), or prints the payee in a form not read here. So these payments are counted under no \
payee and no s.194C limit is tested on them; one recipient may have been paid several of them."],
            &["The recipient of each payment listed, and what it was for."],
        ));
    }
    if over_count > 0 {
        r.findings.push(finding(
            "s194c_candidates/all",
            &["s.194C", "s.40(a)(ia)"],
            "Payees paid through the bank beyond a s.194C threshold",
            &[
                ("payees", "payees_over_194c_count"),
                ("amount", "payees_over_194c_total"),
                ("not_through_bank", "not_through_bank_total"),
            ],
            voucher_refs(vouchers_of(&over_payments)),
            Confidence::JudgementRequired,
            &[
                "Tax was deductible only if the assessee was a deductor for the year (for an \
individual, previous-year business turnover above the limit) and the payee was a contractor, not \
an employee.",
                "The payee is what the bank printed: payments sharing a printed name or a whole \
UPI handle are one row, so one handle under two names, or one name on two handles, is one payee, \
and two different people printed with the same name are one row too. A handle one bank prints cut \
short counts as the whole handle only when exactly one whole handle among the bank payments starts \
with it and a name printed with it starts with the cut payment's name (or the reverse); otherwise \
it joins nothing, and the payees it might join are listed apart. A person printed two ways with no \
handle to join them is two rows. Only outgoing bank payments are read, and joined; payments in \
cash or by journal on these ledgers are not read here.",
            ],
            &[
                "Confirm the previous year's business turnover.",
                "For each payee listed, state whether an employee or a contractor, and provide \
PAN, and any TDS deducted and deposited.",
            ],
        ));
    }
    if day_count > 0 {
        r.findings.push(finding(
            "same_day/all",
            &["s.194C", "s.40(a)(ia)"],
            "Payees paid more than once on one day, together over the s.194C single-payment limit",
            &[
                ("payee_days", "same_day_count"),
                ("amount", "same_day_total"),
            ],
            voucher_refs(vouchers_of(&day_payments)),
            Confidence::JudgementRequired,
            &[
                "s.194C tests a single sum credited or paid. Payments on one day may be for one \
contract (one trip, one bill) or for several, and the books do not say which; these day totals are \
listed, not counted as over the limit.",
                DEDUCTOR_LIMIT,
            ],
            &[
                "For each payee and day listed, whether the payments were for one contract (one \
trip, one bill), with the bills.",
            ],
        ));
    }
    if crossing_count > 0 {
        r.findings.push(finding(
            "elsewhere/all",
            &["s.194C", "s.40(a)(ia)"],
            "Payees under the s.194C limits here, but over one with their payments booked to \
other ledgers",
            &[
                ("payees", "elsewhere_over_count"),
                ("amount", "elsewhere_over_total"),
            ],
            voucher_refs(crossing_evidence),
            Confidence::JudgementRequired,
            &[
                "The payments booked to other ledgers (a loan, a creditor, salary) went to the \
same printed payee but are not read as payments under a contract: whether any was for the same \
kind of work, and so counts towards the s.194C aggregate, the books do not say.",
                DEDUCTOR_LIMIT,
            ],
            &["For each payee listed, what the payments booked to the other ledgers were for."],
        ));
    }
    if groups > 0 {
        r.findings.push(finding(
            "possible_same/all",
            &["s.194C"],
            "Payees that may be one person: they share a UPI address the bank printed cut short",
            &[("groups", "possible_same_count")],
            voucher_refs(vouchers_of(&group_payments)),
            Confidence::JudgementRequired,
            &[
                "One bank prints the UPI address cut short. These payees' payments carry the same \
shortened address, and no single full address among the bank payments completes it under an \
agreeing name (none does, several do, or the one that does is printed under another name), so \
they are not joined: two people can share it. If a group is one person, the payee's total is the \
group's, and a s.194C limit may be crossed.",
            ],
            &["For each group listed, whether the payees are one person."],
        ));
    }
    Ok(r)
}

fn int_of(f: &Figure) -> i64 {
    match f.value {
        Value::Int(n) => n,
        _ => 0,
    }
}

/// The module's own check, NP-1 to NP-7 (README section 8), given the book, the result, the bank
/// ledgers and the ledgers read. A cited GUID resolves to the last population voucher carrying it.
///
/// # Errors
///
/// A population that cannot be formed; and where the reference stops with an internal error (README
/// section 13): a `payee_total_` figure citing a GUID whose last population voucher has no bank
/// leg.
#[allow(clippy::too_many_lines)]
pub fn check_invariants(
    book: &Book,
    result: &TestResult,
    bank: &BTreeSet<String>,
    read: &BTreeSet<String>,
) -> Result<Vec<String>> {
    let ledgers = Ledgers {
        bank,
        read: read.clone(),
    };
    let pop = book.population()?;
    let w = walk(&pop, &ledgers)?;
    let mut out = Vec::new();
    let figure = |name: &str| {
        let id = format!("{TEST_ID}.{name}");
        result.figures.iter().find(|f| f.id == id)
    };
    let last_at: HashMap<&str, usize> = pop
        .iter()
        .enumerate()
        .map(|(at, v)| (v.guid.as_str(), at))
        .collect();
    let resolve =
        |e: &EvidenceRef| (e.kind == "voucher").then(|| last_at.get(e.id.as_str()).copied());
    let totals: Vec<&Figure> = result
        .figures
        .iter()
        .filter(|f| f.id.starts_with(&format!("{TEST_ID}.payee_total_")))
        .collect();
    let tag_of = |f: &Figure| f.id[format!("{TEST_ID}.payee_total_").len()..].to_string();

    // NP-1: a fresh walk of the population.
    for (name, fresh) in [
        ("unresolved_bank_total", w.unresolved_total),
        ("not_through_bank_total", w.not_through_total),
    ] {
        if let Some(f) = figure(name) {
            if int_of(f) != fresh {
                out.push(format!(
                    "NP-1: {name} = {} but a fresh population walk finds {fresh}",
                    int_of(f)
                ));
            }
        }
    }
    let fresh: HashMap<&str, i64> = w
        .rows
        .iter()
        .map(|row| (w.payees[row.payee].tag.as_str(), row.total))
        .collect();
    let mut figured = BTreeSet::new();
    for f in &totals {
        let tag = tag_of(f);
        match fresh.get(tag.as_str()) {
            None => out.push(format!(
                "NP-1: {} names a payee a fresh population walk does not find at all",
                f.id
            )),
            Some(&sum) if sum != int_of(f) => out.push(format!(
                "NP-1: {} = {} but a fresh population walk of the same payee finds {sum}",
                f.id,
                int_of(f)
            )),
            Some(_) => {}
        }
        figured.insert(tag);
    }
    for row in &w.rows {
        let tag = &w.payees[row.payee].tag;
        if !figured.contains(tag) {
            out.push(format!(
                "NP-1: a fresh population walk finds payee tag {tag} with no payee_total_{tag} figure"
            ));
        }
    }
    let sum = total(totals.iter().map(|f| match &f.value {
        Value::Int(n) => n,
        _ => &0,
    }))?;
    if let Some(f) = figure("resolved_total") {
        if int_of(f) != sum {
            out.push(format!(
                "NP-1: resolved_total = {} != sum of payee_total_* figures ({sum})",
                int_of(f)
            ));
        }
    }

    // NP-2 and NP-3: each voucher a payee total cites, in the order the test made the figures.
    let mut cited_by: HashMap<&str, String> = HashMap::new();
    for f in &totals {
        let tag = tag_of(f);
        for e in f.evidence.iter().filter(|e| e.kind == "voucher") {
            match resolve(e).flatten() {
                None => out.push(format!(
                    "NP-2: {} evidence voucher {} is not in the books population",
                    f.id, e.id
                )),
                Some(at) if !w.outgoing.contains(&at) => {
                    return Err(AuditError::ModuleInvariant {
                        module: TEST_ID,
                        code: "NP-2",
                        detail: format!("{} evidence voucher {} carries no bank leg", f.id, e.id),
                    });
                }
                Some(at) => {
                    let got = w
                        .payment_at
                        .get(&at)
                        .map_or("none", |&i| w.payees[w.payee_of[i]].tag.as_str());
                    if got != tag {
                        out.push(format!(
                            "NP-2: {} evidence voucher {} narration names payee tag {got}, not {tag}",
                            f.id, e.id
                        ));
                    }
                }
            }
            if let Some(earlier) = cited_by.get(e.id.as_str()) {
                if *earlier != tag {
                    out.push(format!(
                        "NP-3: voucher {} is evidence for both payee tag {earlier} and {tag}",
                        e.id
                    ));
                }
            }
            cited_by.insert(e.id.as_str(), tag.clone());
        }
    }

    // NP-4 to NP-6: each row's payments read again, with the narration reader alone.
    let mut by_name: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut by_handle: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for f in &totals {
        let tag = tag_of(f);
        let readings: Vec<Option<Printed>> = f
            .evidence
            .iter()
            .filter_map(|e| resolve(e).flatten())
            .map(|at| read_narration(&pop[at].narration).payee().cloned())
            .collect();
        for p in readings.iter().flatten() {
            by_name
                .entry(p.name.clone())
                .or_default()
                .insert(tag.clone());
            if let Some(Handle::Whole(h)) = &p.handle {
                by_handle.entry(h.clone()).or_default().insert(tag.clone());
            }
        }
        if readings.len() >= 2 && !connected(&readings) {
            out.push(format!(
                "NP-6: payee row {tag} joins payments that no shared printed name or UPI handle links"
            ));
        }
    }
    for (what, map) in [("printed name", &by_name), ("whole UPI handle", &by_handle)] {
        let code = if what == "printed name" {
            "NP-4"
        } else {
            "NP-5"
        };
        for (text, tags) in map.iter().filter(|(_, t)| t.len() > 1) {
            out.push(format!(
                "{code}: the {what} {} is in {} payee rows ({})",
                py_repr_str(text),
                tags.len(),
                tags.iter().cloned().collect::<Vec<_>>().join(", ")
            ));
        }
    }

    // NP-7: each payee's debits to other ledgers, from the vouchers its figure cites.
    for f in result
        .figures
        .iter()
        .filter(|f| f.id.starts_with(&format!("{TEST_ID}.payee_elsewhere_")))
    {
        let mut sum = 0;
        for at in f.evidence.iter().filter_map(|e| resolve(e).flatten()) {
            sum = add(sum, ledgers.other_debits(book, pop[at])?.0)?;
        }
        if sum != int_of(f) {
            out.push(format!(
                "NP-7: {} = {} but its evidence vouchers debit other ledgers by {sum}",
                f.id,
                int_of(f)
            ));
        }
    }
    Ok(out)
}

/// NP-6's link between two payments read again (README section 8).
fn linked(a: &Printed, b: &Printed) -> bool {
    if a.name == b.name {
        return true;
    }
    let cut_of = |whole: &Printed, cut: &Printed| match (&whole.handle, &cut.handle) {
        (Some(Handle::Whole(w)), Some(Handle::Cut(c))) => {
            let strip = |n: &str| n.chars().filter(|&c| c != ' ').collect::<String>();
            let (x, y) = (strip(&whole.name), strip(&cut.name));
            w.starts_with(c.as_str()) && !x.is_empty() && !y.is_empty() && names_agree(&x, &y)
        }
        _ => false,
    };
    match (&a.handle, &b.handle) {
        (Some(Handle::Whole(x)), Some(Handle::Whole(y))) if x == y => true,
        _ => cut_of(a, b) || cut_of(b, a),
    }
}

/// Whether every payment of a row is reached from the first through NP-6's links.
fn connected(readings: &[Option<Printed>]) -> bool {
    let mut reached = vec![false; readings.len()];
    let mut stack = vec![0];
    reached[0] = true;
    while let Some(i) = stack.pop() {
        for j in 0..readings.len() {
            if reached[j] {
                continue;
            }
            if let (Some(a), Some(b)) = (&readings[i], &readings[j]) {
                if linked(a, b) {
                    reached[j] = true;
                    stack.push(j);
                }
            }
        }
    }
    reached.into_iter().all(|r| r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{LedgerLine, VoucherStatus};
    use bridge_tally_primitives::TallyDate;

    fn payee(name: &str, channel: Channel, handle: Option<Handle>) -> Reading {
        Reading::Payee(Printed {
            name: name.to_string(),
            channel,
            handle,
        })
    }

    fn named(name: &str, channel: Channel) -> Reading {
        payee(name, channel, None)
    }

    fn whole(h: &str) -> Option<Handle> {
        Some(Handle::Whole(h.to_string()))
    }

    fn none_of(texts: &[&str]) {
        for t in texts {
            assert_eq!(read_narration(t), Reading::NoPayee, "{t:?}");
        }
    }

    /// README section 3.1: Python's upper-casing, which can lengthen the text; Python's whitespace,
    /// made one space; A to Z only (`np_text`).
    #[test]
    fn the_text_is_upper_cased_and_spaced_as_python_does_it_then_read_in_ascii() {
        use Channel::Upi;
        for (text, name) in [
            ("upi-stra\u{df}e-wages", "STRASSE"),
            ("UPI-\u{17f}IGMA-WAGES", "SIGMA"),
            ("upi-d\u{131}l-wages", "DIL"),
            ("upi-\u{fb01}ne-wages", "FINE"),
            ("UPI-OMEGA\u{a0}MART-WAGES", "OMEGA MART"),
            ("UPI-OMEGA\u{2028}MART-WAGES", "OMEGA MART"),
            ("UPI-OMEGA\u{1c}MART-WAGES", "OMEGA MART"),
            ("UPI-OMEGA\u{85}MART-WAGES", "OMEGA MART"),
            ("\u{a0}UPI-OMEGA \u{3000} MART-WAGES", "OMEGA MART"),
            ("  UPI-ALPHA \t TRADERS -WAGES\n", "ALPHA TRADERS"),
        ] {
            assert_eq!(read_narration(text), named(name, Upi), "{text:?}");
        }
        none_of(&[
            "UPI-\u{212a}APPA-WAGES",
            "upi-caf\u{e9}-wages",
            "UPI-\u{926}\u{947}\u{935}-WAGES",
            "UPI-\u{ff21}\u{ff2c}\u{ff30}\u{ff28}\u{ff21}-WAGES",
            "UPI-OMEGA\u{200b}MART-WAGES",
            "\u{feff}UPI-OMEGA MART-WAGES",
            "UPI\u{2010}OMEGA MART\u{2010}WAGES",
            "UPI-NU \u{967}-WAGES",
            "",
            "   ",
        ]);
    }

    /// README section 3.1: a name of forms 1 to 3 runs to its first hyphen, from A to Z, through A
    /// to Z, 0 to 9, space, `.`, `&` and `'`, and loses one space at its end; every form starts at
    /// the text's first character (`np_forms`, `np_unread`).
    #[test]
    fn a_name_runs_to_its_first_hyphen_from_the_texts_start() {
        use Channel::{Imps, Neft, Upi};
        assert_eq!(
            read_narration("UPI-ALPHA TRADERS-BETA STORES-GAMMA MILLS"),
            named("ALPHA TRADERS", Upi)
        );
        assert_eq!(
            read_narration("UPI-A1 B.C & D'E-WAGES"),
            named("A1 B.C & D'E", Upi)
        );
        assert_eq!(read_narration("UPI-Z-WAGES"), named("Z", Upi));
        assert_eq!(
            read_narration("UPI-ALPHA TRADERS-"),
            named("ALPHA TRADERS", Upi)
        );
        assert_eq!(
            read_narration("IMPS-55-BETA STORES -ZZBK"),
            named("BETA STORES", Imps)
        );
        assert_eq!(
            read_narration("NEFT DR-ZZBK77-GAMMA MILLS -NET"),
            named("GAMMA MILLS", Neft)
        );
        // A placeholder word is a payee in the first layout.
        assert_eq!(read_narration("UPI-PHONEPE-BILLPAY"), named("PHONEPE", Upi));
        none_of(&[
            "UPI-ALPHA TRADERS",
            "UPI--ALPHA.T@ZZBK",
            "UPI-9 STAR-WAGES",
            "UPI- ALPHA-WAGES",
            "UPI -ALPHA-WAGES",
            "UPI-ALPHA_T-WAGES",
            "UPI-ALPHA/T-WAGES",
            "UPI-ALPHA,T-WAGES",
            "PAID UPI-ALPHA-WAGES",
            "UPIALPHA-WAGES",
            "REF IMPS-4021-BETA STORES-ZZBK",
            "BY NEFT DR-ZZBK77-GAMMA MILLS-NET",
            "BY TO TRANSFER-UPI/DR/5/DELTA/ZZBK",
            "7 ZETA - CHQ PAID",
        ]);
    }

    /// README section 3.1: a channel word alone, the trailing space removed, names no one in forms
    /// 1 to 3 and stops the reading; a name that only begins with one is a name, and forms 4 and 5
    /// read `UPI` as a payee.
    #[test]
    fn a_channel_word_alone_names_no_one_in_forms_one_to_three_only() {
        none_of(&[
            "IMPS-55-UPI-ALPHA TRADERS-ZZBK",
            "IMPS-55-UPI -BETA STORES-ZZBK",
            "UPI-IMPS-ETA@ZZBK",
            "NEFT DR-ZZBK77-RTGS-GAMMA MILLS-NET",
            "RTGS DR-ZZBK77-NEFT-GAMMA MILLS-NET",
            // The first layout's channel word stops the reading before the cheque form.
            "UPI-UPI - CHQ PAID",
        ]);
        assert_eq!(
            read_narration("IMPS-55-UPIKA STORES-ZZBK"),
            named("UPIKA STORES", Channel::Imps)
        );
        assert_eq!(
            read_narration("NEFT DR-ZZBK77-UPI STORES-NET"),
            named("UPI STORES", Channel::Neft)
        );
        assert_eq!(
            read_narration("TO TRANSFER-UPI/DR/5/UPI/ZZBK"),
            named("UPI", Channel::Upi)
        );
        assert_eq!(
            read_narration("UPI - CHQ PAID"),
            named("UPI", Channel::ChequeCounter)
        );
    }

    /// README section 3.2 forms 2 and 3: the IMPS reference takes any decimal digit, the NEFT code
    /// A to Z and 0 to 9 only.
    #[test]
    fn imps_neft_and_rtgs() {
        use Channel::{Imps, Neft, Rtgs};
        for (text, name, channel) in [
            ("IMPS-4021-BETA STORES-ZZBK-XX12", "BETA STORES", Imps),
            ("imps-4021-beta stores-zzbk", "BETA STORES", Imps),
            (
                "IMPS-\u{967}\u{968}\u{969}-PI STORES-ZZBK",
                "PI STORES",
                Imps,
            ),
            ("IMPS-\u{ff14}\u{ff10}-PI STORES-ZZBK", "PI STORES", Imps),
            ("NEFT DR-ZZBK77-GAMMA MILLS-NET", "GAMMA MILLS", Neft),
            ("NEFTDR-ZZBK77-GAMMA MILLS-NET", "GAMMA MILLS", Neft),
            ("RTGS DR-ZZBK77-GAMMA MILLS-NET", "GAMMA MILLS", Rtgs),
            ("RTGSDR-Q-GAMMA MILLS-", "GAMMA MILLS", Rtgs),
        ] {
            assert_eq!(read_narration(text), named(name, channel), "{text:?}");
        }
        none_of(&[
            "IMPS-4\u{b2}-PI STORES-ZZBK",
            "IMPS-\u{2460}-PI STORES-ZZBK",
            "IMPS-BETA STORES-ZZBK",
            "IMPS-40 21-BETA STORES-ZZBK",
            "IMPS-A40-BETA STORES-ZZBK",
            "IMPS-4021-BETA STORES",
            "IMPS-4021-7 STAR-ZZBK",
            "IMPS 4021-BETA STORES-ZZBK",
            "IMPS-5-PI \u{967}-ZZBK",
            "NEFT CR-ZZBK77-GAMMA MILLS-NET",
            "NEFT DR--GAMMA MILLS-NET",
            "NEFT DR-ZZ BK-GAMMA MILLS-NET",
            "NEFT DR-ZZBK77-GAMMA MILLS",
            "NEFT-ZZBK77-GAMMA MILLS-NET",
            "NEFT DR -ZZBK77-GAMMA MILLS-NET",
            "IFT DR-ZZBK77-GAMMA MILLS-NET",
            "NEFT DR-ZZ\u{967}-GAMMA MILLS-NET",
        ]);
    }

    /// README section 3.2 form 4: spaces removed from the name field, the two placeholders, and a
    /// field with no letter A to Z naming no one.
    #[test]
    fn the_second_upi_layout() {
        for (text, name) in [
            ("TO TRANSFER-UPI/DR/5 12/DELTA WO/", "DELTAWO"),
            ("to  transfer-upi/dr/5 1 2/delta wo/zzbk/", "DELTAWO"),
            ("TO TRANSFER-UPI/DR/ /DELTAWO/", "DELTAWO"),
            ("TO TRANSFER-UPI/DR/5/D-7 & CO./ZZBK", "D-7&CO."),
            ("TO TRANSFER-UPI/DR/5/PHONEPE1/ZZBK", "PHONEPE1"),
            ("TO TRANSFER-UPI/DR/5/BANK ACCT/ZZBK", "BANKACCT"),
            ("TO TRANSFER-UPI/DR/\u{967}\u{968} \u{969}/RHO/ZZBK", "RHO"),
            (
                "TO TRANSFER-UPI/DR/5/\u{926}\u{947}\u{935}-KA/ZZBK",
                "\u{926}\u{947}\u{935}-KA",
            ),
            ("TO TRANSFER-UPI/DR/5/TAU\u{302}/ZZBK", "TAU\u{302}"),
        ] {
            assert_eq!(read_narration(text), named(name, Channel::Upi), "{text:?}");
        }
        for text in [
            "TO TRANSFER-UPI/DR/5 12/BANK ACC/ZZBK/DELTAWORKS/PAY",
            "to transfer-upi/dr/5 12/Phone Pe/zzbk/billpay/pay",
        ] {
            assert_eq!(read_narration(text), Reading::Placeholder, "{text:?}");
        }
        none_of(&[
            "TO TRANSFER-UPI/DR/5 12/40 217/ZZBK",
            "TO TRANSFER-UPI/DR//DELTA/ZZBK",
            "TO TRANSFER-UPI/DR/5A/DELTA/ZZBK",
            "TO TRANSFER-UPI/CR/5/DELTA/ZZBK",
            "TO TRANSFER-UPI/DR/5/DELTA",
            "TO TRANSFER-UPI/DR/5//ZZBK",
            "TO TRANSFER -UPI/DR/5/DELTA/ZZBK",
            "TO TRANSFER-NEFT/DR/5/DELTA/ZZBK",
            "TO TRANSFER-UPI/DR/5/ /ZZBK",
            "TO TRANSFER-UPI/DR/5/\u{926}\u{947}\u{935}/ZZBK",
            "to transfer-upi/dr/5/\u{3b1}\u{3bb}\u{3c6}\u{3b1}/zzbk",
        ]);
    }

    /// README section 3.2 form 5: the prefix and its four spellings, read only after it; the name's
    /// characters; the hyphen and `CHQ PAID`; the refused words `SELF` and `CASH PAID TO`.
    #[test]
    fn the_cheque_paid_at_the_counter() {
        for (text, name) in [
            ("EPSILON CARRIERS - CHQ PAID - 12", "EPSILON CARRIERS"),
            ("EPSILON CARRIERS-CHQ PAID", "EPSILON CARRIERS"),
            (
                "WITHDRAWAL BY EPSILON CARRIERS - CHQ PAID",
                "EPSILON CARRIERS",
            ),
            (
                "WITHDRWAL BY EPSILON CARRIERS - CHQ PAID",
                "EPSILON CARRIERS",
            ),
            (
                "WITHDRAWL BY EPSILON CARRIERS - CHQ PAID",
                "EPSILON CARRIERS",
            ),
            (
                "WITHDRWL BY EPSILON CARRIERS - CHQ PAID",
                "EPSILON CARRIERS",
            ),
            (
                "withdrawal by  epsilon carriers -chq paid",
                "EPSILON CARRIERS",
            ),
            ("EPSILON CARRIERS- CHQ PAIDX", "EPSILON CARRIERS"),
            ("M.K. ZETA - CHQ PAID", "M.K. ZETA"),
            ("ZETA. - CHQ PAID", "ZETA."),
            ("MYSELF - CHQ PAID", "MYSELF"),
            ("CASH PAID - CHQ PAID", "CASH PAID"),
            ("WITHDRAWALBY ETA - CHQ PAID", "WITHDRAWALBY ETA"),
            ("PAID TO THETA - CHQ PAID", "PAID TO THETA"),
            (
                "WITHDRAWAL BYWAY CARRIERS - CHQ PAID",
                "WITHDRAWAL BYWAY CARRIERS",
            ),
            ("Z-CHQ PAID", "Z"),
            ("SELFMADE MILLS - CHQ PAID", "SELFMADE MILLS"),
            ("CASH PAID TOWER - CHQ PAID", "CASH PAID TOWER"),
        ] {
            assert_eq!(
                read_narration(text),
                named(name, Channel::ChequeCounter),
                "{text:?}"
            );
        }
        none_of(&[
            "SELF - CHQ PAID",
            "WITHDRAWAL BY SELF - CHQ PAID",
            "CASH PAID TO EPSILON - CHQ PAID",
            "SELF. - CHQ PAID",
            "ZETA & SONS - CHQ PAID",
            "ZETA 2 - CHQ PAID",
            "ZETA'S - CHQ PAID",
            "ZETA - ETA - CHQ PAID",
            ".ZETA - CHQ PAID",
            "ZETA - CHQ RETURN",
            "ZETA CHQ PAID",
            "WITHDRAWAL BY 7 - CHQ PAID",
            " - CHQ PAID",
            "WITHDRAWAL BY - CHQ PAID",
            "WITHDRAWAL BY-CHQ PAID",
            "WITHDRAWAL BY. - CHQ PAID",
            "\u{c9}TOILE - CHQ PAID",
            "ACH DEBIT ZETA",
            "NEFT-SELF",
        ]);
    }

    /// README section 3.3 form 1: A to Z, 0 to 9, `.` and `_`, one optional suffix of a hyphen and
    /// decimal digits, then `@` (`np_handles`).
    #[test]
    fn the_first_layouts_handle() {
        for (text, name, handle) in [
            (
                "UPI-ALPHA TRADERS-ALPHA.T@ZZBK-WAGES",
                "ALPHA TRADERS",
                "ALPHA.T",
            ),
            ("upi-alpha traders-alpha.t@zzbk", "ALPHA TRADERS", "ALPHA.T"),
            ("UPI-BETA STORES-BETA_S.9@ZZBK", "BETA STORES", "BETA_S.9"),
            ("UPI-GAMMA MILLS-70412-2@ZZBK", "GAMMA MILLS", "70412-2"),
            ("UPI-NU STORES-NU.S@", "NU STORES", "NU.S"),
            (
                "UPI-XI MART-XI-\u{967}\u{968}@ZZBK",
                "XI MART",
                "XI-\u{967}\u{968}",
            ),
        ] {
            assert_eq!(
                read_narration(text),
                payee(name, Channel::Upi, whole(handle)),
                "{text:?}"
            );
        }
        for (text, name) in [
            ("UPI-DELTA WORKS-DELTA W@ZZBK", "DELTA WORKS"),
            ("UPI-ETA-ETA-X@ZZBK", "ETA"),
            ("UPI-THETA-TH-1-2@ZZBK", "THETA"),
            ("UPI-IOTA-IOTA&CO@ZZBK", "IOTA"),
            ("UPI-KAPPA-@ZZBK", "KAPPA"),
            ("UPI-LAMBDA-LAM.BDA\u{ff20}ZZBK", "LAMBDA"),
            ("UPI-MU-M\u{dc}@ZZBK", "MU"),
            ("UPI-OMEGA--12@ZZBK", "OMEGA"),
            ("UPI-BANYAN ONE-BN\u{967}@ZZBK", "BANYAN ONE"),
        ] {
            assert_eq!(read_narration(text), named(name, Channel::Upi), "{text:?}");
        }
        // Only a UPI payee has a handle.
        assert_eq!(
            read_narration("IMPS-88-PHI THREE-ZZBK"),
            named("PHI THREE", Channel::Imps)
        );
    }

    /// README section 3.3 form 4: the handle field after a field that may be empty, spaces removed,
    /// whole before its first `@` and cut without one, and no handle when empty or unclosed.
    #[test]
    fn the_second_layouts_handle() {
        let upi = |name: &str, handle: Option<Handle>| payee(name, Channel::Upi, handle);
        for (text, reading) in [
            (
                "TO TRANSFER-UPI/DR/5 12/OMICRON/ZZBK/OMI.CRON@ZZ/PAY",
                upi("OMICRON", whole("OMI.CRON")),
            ),
            (
                "TO TRANSFER-UPI/DR/5 12/PI ST/ZZBK/PI ST ORE@Z Z/PAY",
                upi("PIST", whole("PISTORE")),
            ),
            (
                "TO TRANSFER-UPI/DR/5/SIGMA//SIG.MA@ZZ/",
                upi("SIGMA", whole("SIG.MA")),
            ),
            (
                "TO TRANSFER-UPI/DR/5/UPSILON/ZZBK/UPS@Z@Q/PAY",
                upi("UPSILON", whole("UPS")),
            ),
            (
                "TO TRANSFER-UPI/DR/5 12/SAL TWO/ZZBK/SAL COMMON/PAY",
                upi("SALTWO", Some(Handle::Cut("SALCOMMON".to_string()))),
            ),
            (
                "TO TRANSFER-UPI/DR/5/RHO/ZZBK/RHOMART.1@ZZ",
                upi("RHO", None),
            ),
            ("TO TRANSFER-UPI/DR/5/TAU/ZZBK/@ZZ/PAY", upi("TAU", None)),
            ("TO TRANSFER-UPI/DR/5/TAU/ZZBK/ /PAY", upi("TAU", None)),
        ] {
            assert_eq!(read_narration(text), reading, "{text:?}");
        }
    }

    fn voucher(guid: &str, vtype: &str, narration: &str, lines: &[(&str, i64)]) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse("20250502").unwrap(),
            vtype: vtype.to_string(),
            base_type: vtype.to_string(),
            number: "1".to_string(),
            status: VoucherStatus::Regular,
            narration: narration.to_string(),
            lines: lines
                .iter()
                .map(|(l, a)| LedgerLine {
                    ledger: (*l).to_string(),
                    amount_paise: *a,
                })
                .collect(),
            ..Voucher::default()
        }
    }

    fn set(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|n| (*n).to_string()).collect()
    }

    /// README section 13: where a payee total cites a GUID whose last population voucher has no
    /// bank leg, the reference's check stops after forming its NP-2 message; so does this one,
    /// with the code and the message's detail. The control, the same two vouchers the other way
    /// round, runs.
    #[test]
    fn a_cited_guid_whose_last_voucher_has_no_bank_leg_stops_the_check() {
        let rules = Rules::vendored().unwrap();
        let (bank, listed) = (set(&["Bank A"]), set(&["Wages"]));
        let paid = voucher(
            "g1",
            "Payment",
            "UPI-ALPHA-WAGES",
            &[("Wages", 100), ("Bank A", -100)],
        );
        let journal = voucher("g1", "Journal", "", &[("Wages", 200), ("Creditor", -200)]);
        let run_check = |vouchers: Vec<Voucher>| {
            let book = Book {
                vouchers,
                ..Book::default()
            };
            let r = run(&book, &rules, &bank, &listed, &BTreeSet::new()).unwrap();
            check_invariants(&book, &r, &bank, &listed)
        };
        let err = run_check(vec![paid.clone(), journal.clone()]).unwrap_err();
        let want = format!(
            "narration_payees.payee_total_{} evidence voucher g1 carries no bank leg",
            hash8("ALPHA")
        );
        assert!(
            matches!(
                &err,
                AuditError::ModuleInvariant { module: TEST_ID, code: "NP-2", detail } if *detail == want
            ),
            "{err:?}"
        );
        assert_eq!(
            run_check(vec![journal, paid]).unwrap(),
            Vec::<String>::new()
        );
    }

    /// NP-1 (README section 8) walks the population again and compares: a result whose totals or
    /// payee figures disagree with that walk is reported, each by its own message. The untouched
    /// result is the control.
    #[test]
    fn np1_reports_each_total_and_payee_figure_a_fresh_walk_disagrees_with() {
        let rules = Rules::vendored().unwrap();
        let (bank, listed) = (set(&["Bank A"]), set(&["Wages"]));
        let book = Book {
            vouchers: vec![
                voucher(
                    "p1",
                    "Payment",
                    "UPI-ALPHA-WAGES",
                    &[("Wages", 100), ("Bank A", -100)],
                ),
                voucher(
                    "p2",
                    "Payment",
                    "UPI-BETA-WAGES",
                    &[("Wages", 300), ("Bank A", -300)],
                ),
                voucher("p3", "Payment", "WAGES", &[("Wages", 50), ("Bank A", -50)]),
                voucher("j1", "Journal", "", &[("Wages", 70), ("Creditor", -70)]),
            ],
            ..Book::default()
        };
        let r = run(&book, &rules, &bank, &listed, &BTreeSet::new()).unwrap();
        let alpha = format!("{TEST_ID}.payee_total_{}", hash8("ALPHA"));
        let check = |edit: &dyn Fn(&mut TestResult)| {
            let mut tampered = r.clone();
            edit(&mut tampered);
            let mut out = check_invariants(&book, &tampered, &bank, &listed).unwrap();
            out.sort();
            out
        };
        let add_one = |name: String| {
            move |t: &mut TestResult| {
                let f = t.figures.iter_mut().find(|f| f.id == name).unwrap();
                f.value = Value::Int(int_of(f) + 1);
            }
        };
        let figure = |name: &str| format!("{TEST_ID}.{name}");

        assert_eq!(check(&|_| {}), Vec::<String>::new());
        assert_eq!(
            check(&add_one(figure("resolved_total"))),
            ["NP-1: resolved_total = 401 != sum of payee_total_* figures (400)"]
        );
        assert_eq!(
            check(&add_one(figure("unresolved_bank_total"))),
            ["NP-1: unresolved_bank_total = 51 but a fresh population walk finds 50"]
        );
        assert_eq!(
            check(&add_one(figure("not_through_bank_total"))),
            ["NP-1: not_through_bank_total = 71 but a fresh population walk finds 70"]
        );
        assert_eq!(
            check(&add_one(alpha.clone())),
            [
                format!(
                    "NP-1: {alpha} = 101 but a fresh population walk of the same payee finds 100"
                ),
                "NP-1: resolved_total = 400 != sum of payee_total_* figures (401)".to_string(),
            ]
        );
        // ALPHA's total under a tag no payee has: the figure names no payee the walk finds, the
        // walk finds ALPHA with no figure, and NP-2 reads the cited voucher as ALPHA's.
        let stray = format!("{TEST_ID}.payee_total_00000000");
        assert_eq!(
            check(&|t: &mut TestResult| {
                t.figures.iter_mut().find(|f| f.id == alpha).unwrap().id = stray.clone();
            }),
            [
                format!(
                    "NP-1: a fresh population walk finds payee tag {0} with no payee_total_{0} \
                     figure",
                    hash8("ALPHA")
                ),
                format!("NP-1: {stray} names a payee a fresh population walk does not find at all"),
                format!(
                    "NP-2: {stray} evidence voucher p1 narration names payee tag {}, not 00000000",
                    hash8("ALPHA")
                ),
            ]
        );
    }

    /// NP-2 (README section 8): a cited GUID with no population voucher is reported, and nothing
    /// more is checked for it, as the reference skips the rest; so one cited under two tags
    /// gives no NP-3 line. A result the test cannot make, built here by hand.
    #[test]
    fn np2_checks_nothing_more_for_a_cited_voucher_outside_the_population() {
        let rules = Rules::vendored().unwrap();
        let (bank, listed) = (set(&["Bank A"]), set(&["Wages"]));
        let book = Book {
            vouchers: vec![
                voucher(
                    "p1",
                    "Payment",
                    "UPI-ALPHA-WAGES",
                    &[("Wages", 100), ("Bank A", -100)],
                ),
                voucher(
                    "p2",
                    "Payment",
                    "UPI-BETA-WAGES",
                    &[("Wages", 300), ("Bank A", -300)],
                ),
            ],
            ..Book::default()
        };
        let mut r = run(&book, &rules, &bank, &listed, &BTreeSet::new()).unwrap();
        let ids: Vec<String> = ["ALPHA", "BETA"]
            .map(|name| format!("{TEST_ID}.payee_total_{}", hash8(name)))
            .into();
        for f in r.figures.iter_mut().filter(|f| ids.contains(&f.id)) {
            f.evidence.push(EvidenceRef::with_label(
                "voucher",
                "zz",
                "Payment 9 on 2025-05-02",
            ));
        }
        let mut out = check_invariants(&book, &r, &bank, &listed).unwrap();
        out.sort();
        assert_eq!(out, {
            let mut want: Vec<String> = ids
                .iter()
                .map(|id| format!("NP-2: {id} evidence voucher zz is not in the books population"))
                .collect();
            want.sort();
            want
        });
    }

    #[test]
    fn a_voucher_of_unknown_status_refuses_before_any_figure() {
        let rules = Rules::vendored().unwrap();
        let mut v = voucher("g1", "Payment", "UPI-ALPHA-WAGES", &[("Wages", 100)]);
        v.status = VoucherStatus::Unknown;
        let book = Book {
            vouchers: vec![v],
            ..Book::default()
        };
        let err = run(&book, &rules, &set(&[]), &set(&["Wages"]), &set(&[])).unwrap_err();
        assert!(
            matches!(err, AuditError::UnknownVoucherStatus(1)),
            "{err:?}"
        );
    }

    #[test]
    fn rules_without_s194c_refuse() {
        let mut rules = Rules::vendored().unwrap();
        rules.s194c = None;
        let err = run(&Book::default(), &rules, &set(&[]), &set(&[]), &set(&[])).unwrap_err();
        assert!(
            matches!(&err, AuditError::Config(m) if m == "narration_payees needs rules [s194c]"),
            "{err:?}"
        );
    }
}
