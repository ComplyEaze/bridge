//! Settlements through another party (may bear on Form 3CD clauses 31 and 21(d)): journals that
//! move a balance between two parties with no bank or cash line (T1), and bank or cash vouchers
//! whose narration names, word for word, a party other than the ones booked (T2, or T2 embedded
//! when the name only sits inside a longer name). A port of the reference Python implementation's
//! `knock_off_candidates` test module, version 1; its contract is the spec pack in
//! `docs/tax-audit/spec-packs/knock_off_candidates/`.
//!
//! Every voucher that fits is listed, whatever its size: the test reaches no conclusion, its
//! totals are sums of candidates, and its findings carry no clause. It has no module check.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use unicode_normalization::UnicodeNormalization;

use crate::book::{Book, Voucher};
use crate::error::Result;
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::ledger_ids::stable_ledger_tag;
use crate::rules::Rules;
use crate::support::{self, py_casefold, py_repr_str, py_split, rupees, voucher_label};
use crate::unicode_tables::{LATE, LETTER_OR_NUMBER, MARK};

pub const TEST_ID: &str = "knock_off_candidates";
pub const VERSION: &str = "1";

/// Always party groups, besides the engagement's `[party_identity].party_groups` (pack README §2.5).
const PARTY_GROUPS: [&str; 4] = [
    "Sundry Debtors",
    "Sundry Creditors",
    "Loans (Liability)",
    "Loans & Advances (Asset)",
];
const MONEY_GROUPS: [&str; 3] = ["Bank Accounts", "Bank OD A/c", "Cash-in-Hand"];
/// Any run of these cuts a narration into parts; `.`, `&`, `'`, `#` and `_` do not.
const SEPARATORS: [char; 9] = ['-', '/', ',', ';', '|', ':', '(', ')', '\\'];
/// Dropped while a token is read: they neither end a token nor stay in it.
const DROPPED: [char; 3] = ['\u{200C}', '\u{200D}', '\u{00AD}'];
/// Words that are never name-like beside a name (pack README §3.4).
const NEVER_NAME_LIKE: [&str; 55] = [
    "to",
    "from",
    "by",
    "for",
    "paid",
    "pay",
    "payment",
    "pymt",
    "neft",
    "rtgs",
    "imps",
    "upi",
    "ref",
    "cr",
    "dr",
    "towards",
    "being",
    "via",
    "and",
    "of",
    "the",
    "in",
    "on",
    "at",
    "with",
    "mr",
    "mrs",
    "ms",
    "inb",
    "transfer",
    "trf",
    "chq",
    "cheque",
    "cash",
    "deposit",
    "withdrawal",
    "settled",
    "adjustment",
    "bill",
    "gr",
    "inv",
    "invoice",
    "against",
    "pvt",
    "ltd",
    "limited",
    "llp",
    "private",
    "rs",
    "inr",
    "amt",
    "amount",
    "no",
    "a",
    "an",
];
/// The shown narration's length, in code points.
const SHOWN_CHARS: usize = 80;
const CONTRA: &str = "Contra";

const CANDIDATE: &str = " -- a candidate total, not a reportable amount.";
const ONCE: &str =
    " A Contra, or any voucher whose every line with an amount is on a bank or cash \
     ledger, counts the larger side of its bank and cash lines once.";

const LIMIT_LISTED: &str = "Listed, never judged: the candidate total is not a reportable \
     amount, and nothing is left out for being small or looking harmless. Whether a row is a \
     payment under s.40A(3), a Rule 6DD case, a loan or deposit, a trade set-off, or one person \
     under two ledgers is the CA's to determine; no clause 31 or 21(d) row is written.";
const LIMIT_RECALL: &str = "Recall is not measured on real books.";
const ASK_T2: &str = "The narration names someone other than the booked party. Who was paid (or \
     who paid)? If an agent paid on the payee's behalf, give the agency basis.";

const POPULATION_NOTE: &str = "Every voucher in the books population. Party ledgers are those \
     under Sundry Debtors, Sundry Creditors, Loans (Liability) or Loans & Advances (Asset); bank \
     and cash, those under Bank Accounts, Bank OD A/c or Cash-in-Hand.";

fn in_table(table: &[(u32, u32)], c: char) -> bool {
    let cp = u32::from(c);
    table
        .binary_search_by(|&(lo, hi)| {
            if hi < cp {
                std::cmp::Ordering::Less
            } else if lo > cp {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

/// Python's NFD of `text`, made with the crate's newer tables: cut at each [`LATE`] code point,
/// which stays as written, and each piece between decomposed, and case-folded as Python 3.13
/// does when `fold` is set. Python does not know a late code point, so it never reorders marks
/// around one; the cut keeps the crate's library from doing so.
fn decomposed(text: &str, fold: bool) -> String {
    let mut out = String::with_capacity(text.len());
    let mut piece = String::new();
    let flush = |piece: &mut String, out: &mut String| {
        let nfd: String = piece.nfd().collect();
        out.push_str(&if fold { py_casefold(&nfd) } else { nfd });
        piece.clear();
    };
    for c in text.chars() {
        if in_table(&LATE, c) {
            flush(&mut piece, &mut out);
            out.push(c);
        } else {
            piece.push(c);
        }
    }
    flush(&mut piece, &mut out);
    out
}

/// README §3.3's tokens of one text: maximal runs of letters, numbers and marks by Python's
/// categories. A mark after `a` to `z`, or one that would start a token, is dropped; so are
/// [`DROPPED`], which do not end a token; anything else ends one.
fn tokens(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut token = String::new();
    for c in decomposed(text, true).chars() {
        if DROPPED.contains(&c) {
            continue;
        }
        if in_table(&LATE, c) {
            // Unassigned at Unicode 15.1, so neither a letter, a number nor a mark.
        } else if in_table(&LETTER_OR_NUMBER, c) {
            token.push(c);
            continue;
        } else if in_table(&MARK, c) {
            if token
                .chars()
                .last()
                .is_some_and(|p| !p.is_ascii_lowercase())
            {
                token.push(c);
            }
            continue;
        }
        if !token.is_empty() {
            out.push(std::mem::take(&mut token));
        }
    }
    if !token.is_empty() {
        out.push(token);
    }
    out
}

/// A narration's parts, as character ranges of the text as written, and its tokens numbered
/// across the whole narration, each with the index of its part.
struct Narration {
    chars: Vec<char>,
    parts: Vec<(usize, usize)>,
    tokens: Vec<(String, usize)>,
}

impl Narration {
    fn new(text: &str) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let mut parts = Vec::new();
        let mut start = None;
        for (i, c) in chars.iter().enumerate() {
            match (SEPARATORS.contains(c), start) {
                (true, Some(s)) => {
                    parts.push((s, i));
                    start = None;
                }
                (false, None) => start = Some(i),
                _ => {}
            }
        }
        if let Some(s) = start {
            parts.push((s, chars.len()));
        }
        let mut toks = Vec::new();
        for (p, &(s, e)) in parts.iter().enumerate() {
            let part: String = chars[s..e].iter().collect();
            toks.extend(tokens(&part).into_iter().map(|t| (t, p)));
        }
        Self {
            chars,
            parts,
            tokens: toks,
        }
    }

    /// The text as written from the start of part `first` to the end of part `last`, separators
    /// included, its whitespace collapsed.
    fn between(&self, first: usize, last: usize) -> String {
        let text: String = self.chars[self.parts[first].0..self.parts[last].1]
            .iter()
            .collect();
        py_split(&text).join(" ")
    }
}

/// A token beside a name that makes it "inside" (README §3.4, condition 4).
fn name_like(token: &str) -> bool {
    token.chars().count() >= 2
        && token.chars().all(|c| c.is_ascii_lowercase())
        && !NEVER_NAME_LIKE.contains(&token)
}

/// The narration as shown: whitespace collapsed, cut to 79 characters and `…` past 80, then
/// Python's `repr()`.
fn shown(narration: &str) -> String {
    let collapsed = py_split(narration).join(" ");
    let text = if collapsed.chars().count() > SHOWN_CHARS {
        let mut cut: String = collapsed.chars().take(SHOWN_CHARS - 1).collect();
        cut.push('\u{2026}');
        cut
    } else {
        collapsed
    };
    py_repr_str(&text)
}

/// Party keys: each party ledger's tokens, bound to it, or to no one when another ledger master
/// of any group has the same tokens (README §3.3). A shared key stays in the index.
fn party_index<'a>(
    book: &'a Book,
    party: &BTreeSet<&'a str>,
) -> HashMap<Vec<String>, Option<&'a str>> {
    let mut masters: HashMap<Vec<String>, usize> = HashMap::new();
    for name in book.ledgers.keys() {
        let key = tokens(name);
        if !key.is_empty() {
            *masters.entry(key).or_default() += 1;
        }
    }
    let mut index = HashMap::new();
    for name in party {
        let key = tokens(name);
        if key.is_empty() {
            continue;
        }
        let owner = (masters[&key] == 1).then_some(*name);
        index.insert(key, owner);
    }
    index
}

/// What one narration names: its plain names, its inside names that are not plain, and the parts
/// those inside names sit in.
#[derive(Default)]
struct Named<'a> {
    plain: BTreeSet<&'a str>,
    inside: BTreeSet<&'a str>,
    parts: BTreeSet<String>,
}

fn named<'a>(
    narration: &str,
    index: &HashMap<Vec<String>, Option<&'a str>>,
    on_voucher: &BTreeSet<&str>,
) -> Named<'a> {
    let n = Narration::new(narration);
    let words: Vec<&str> = n.tokens.iter().map(|(t, _)| t.as_str()).collect();
    // Every key matching in full at every position: (first token, last token, owner).
    let mut found: Vec<(usize, usize, Option<&'a str>)> = Vec::new();
    for (key, owner) in index {
        for start in 0..words.len() {
            let end = start + key.len();
            if end <= words.len() && words[start..end].iter().zip(key).all(|(w, k)| *w == k) {
                found.push((start, end - 1, *owner));
            }
        }
    }
    let kept: Vec<(usize, usize, Option<&'a str>)> = found
        .iter()
        .copied()
        .filter(|&(s, e, _)| {
            !found
                .iter()
                .any(|&(fs, fe, _)| fs <= s && e <= fe && (fs, fe) != (s, e))
        })
        .collect();
    let overlaps = |a: (usize, usize), b: (usize, usize)| a.0 <= b.1 && b.0 <= a.1;
    let mut out = Named::default();
    let mut inside_at: Vec<(&'a str, usize, usize)> = Vec::new();
    for &(s, e, owner) in &kept {
        let Some(name) = owner else { continue };
        if on_voucher.contains(name) {
            continue;
        }
        let beside = |t: Option<usize>, anchor: usize| {
            let Some(t) = t.filter(|t| *t < words.len()) else {
                return false;
            };
            let holding: Vec<_> = kept.iter().filter(|k| k.0 <= t && t <= k.1).collect();
            if !holding.is_empty() && !holding.iter().any(|k| overlaps((k.0, k.1), (s, e))) {
                return false;
            }
            n.tokens[t].1 == n.tokens[anchor].1 && name_like(words[t])
        };
        if beside(s.checked_sub(1), s) || beside(Some(e + 1), e) {
            inside_at.push((name, s, e));
        } else {
            out.plain.insert(name);
        }
    }
    for (name, s, e) in inside_at {
        if !out.plain.contains(name) {
            out.inside.insert(name);
            out.parts.insert(n.between(n.tokens[s].1, n.tokens[e].1));
        }
    }
    out
}

/// What a T2 voucher adds to a total: the larger side of its money lines once for a Contra or a
/// voucher whose every nonzero line is money, else each money line's absolute value.
fn money_amount(v: &Voucher, money: &BTreeSet<&str>) -> i128 {
    let lines = || v.lines.iter().filter(|l| l.amount_paise != 0);
    let money_lines = || lines().filter(|l| money.contains(l.ledger.as_str()));
    if v.vtype == CONTRA || lines().all(|l| money.contains(l.ledger.as_str())) {
        let debit: i128 = money_lines()
            .filter(|l| l.amount_paise > 0)
            .map(|l| i128::from(l.amount_paise))
            .sum();
        let credit: i128 = money_lines()
            .filter(|l| l.amount_paise < 0)
            .map(|l| -i128::from(l.amount_paise))
            .sum();
        debit.max(credit)
    } else {
        money_lines()
            .map(|l| i128::from(l.amount_paise).abs())
            .sum()
    }
}

fn paise(total: i128) -> Result<Value> {
    i64::try_from(total)
        .map(Value::Int)
        .map_err(|_| support::overflow(TEST_ID))
}

#[derive(Default)]
struct PerName {
    rows: usize,
    total: i128,
    refs: Vec<EvidenceRef>,
}

/// Run the test. `party_groups` are the engagement's extra party groups, as bound.
///
/// # Errors
///
/// `UnknownVoucherStatus` when the population cannot be formed; `DuplicateFigureId` when two
/// plainly named ledgers share a tag, as the reference stops; an overflow of a total.
pub fn run(book: &Book, rules: &Rules, party_groups: &[String]) -> Result<TestResult> {
    let population = book.population()?;
    let party: BTreeSet<&str> = book
        .ledgers
        .values()
        .filter(|l| {
            PARTY_GROUPS.iter().any(|g| l.under(g)) || party_groups.iter().any(|g| l.under(g))
        })
        .map(|l| l.name.as_str())
        .collect();
    let money: BTreeSet<&str> = book
        .ledgers
        .values()
        .filter(|l| MONEY_GROUPS.iter().any(|g| l.under(g)))
        .map(|l| l.name.as_str())
        .collect();
    let index = party_index(book, &party);

    let (mut t1_refs, mut t1_total) = (Vec::new(), 0i128);
    let (mut t2_refs, mut t2_total) = (Vec::new(), 0i128);
    let (mut emb_refs, mut emb_total) = (Vec::new(), 0i128);
    let mut per_name: BTreeMap<&str, PerName> = BTreeMap::new();
    let mut inside_rows: BTreeMap<&str, usize> = BTreeMap::new();
    for v in population {
        let on: BTreeSet<&str> = v
            .lines
            .iter()
            .filter(|l| l.amount_paise != 0)
            .map(|l| l.ledger.as_str())
            .collect();
        let has_money = on.iter().any(|l| money.contains(l));
        let parties: Vec<&str> = on.iter().copied().filter(|l| party.contains(l)).collect();
        let label = voucher_label(v);
        if parties.len() >= 2 && !has_money {
            let entries: Vec<String> = parties
                .iter()
                .map(|p| {
                    let sum: i128 = v
                        .lines
                        .iter()
                        .filter(|l| l.ledger == *p)
                        .map(|l| i128::from(l.amount_paise))
                        .sum();
                    format!("{p} {}", rupees(sum))
                })
                .collect();
            t1_total += v
                .lines
                .iter()
                .filter(|l| l.amount_paise > 0 && party.contains(l.ledger.as_str()))
                .map(|l| i128::from(l.amount_paise))
                .sum::<i128>();
            t1_refs.push(EvidenceRef::with_label(
                "voucher",
                &v.guid,
                &format!(
                    "{label}: {}; narration: {}",
                    entries.join("; "),
                    shown(&v.narration)
                ),
            ));
        }
        if !has_money || v.narration.is_empty() {
            continue;
        }
        let found = named(&v.narration, &index, &on);
        if found.plain.is_empty() && found.inside.is_empty() {
            continue;
        }
        let amount = money_amount(v, &money);
        let booked: Vec<&str> = on.iter().copied().filter(|l| !money.contains(l)).collect();
        let booked = if booked.is_empty() {
            "no other ledger".to_string()
        } else {
            booked.join(", ")
        };
        if !found.plain.is_empty() {
            let names: Vec<&str> = found.plain.iter().copied().collect();
            t2_total += amount;
            t2_refs.push(EvidenceRef::with_label(
                "voucher",
                &v.guid,
                &format!(
                    "{label}: booked to {booked}; narration names {}; narration: {}",
                    names.join(", "),
                    shown(&v.narration)
                ),
            ));
            for name in names {
                let entry = per_name.entry(name).or_default();
                entry.rows += 1;
                entry.total += amount;
                entry.refs.push(EvidenceRef::with_label(
                    "voucher",
                    &v.guid,
                    &format!("{label}: narration: {}", shown(&v.narration)),
                ));
            }
        }
        if !found.inside.is_empty() {
            let names: Vec<&str> = found.inside.iter().copied().collect();
            let parts: Vec<String> = found.parts.iter().map(|p| py_repr_str(p)).collect();
            emb_total += amount;
            emb_refs.push(EvidenceRef::with_label(
                "voucher",
                &v.guid,
                &format!(
                    "{label}: booked to {booked}; the name of {} sits inside {}",
                    names.join(", "),
                    parts.join(", ")
                ),
            ));
            for name in names {
                *inside_rows.entry(name).or_default() += 1;
            }
        }
    }

    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let t1_count = r.fig(
        "t1_row_count",
        support::count(TEST_ID, t1_refs.len())?,
        Unit::Count,
        "Vouchers with lines on two or more distinct party ledgers and no bank or cash line.",
        t1_refs.clone(),
    )?;
    let t1_sum = r.fig(
        "t1_candidate_total_paise",
        paise(t1_total)?,
        Unit::Paise,
        &format!(
            "Debits on party ledgers in the vouchers settling one party through another with no \
             bank or cash line, summed{CANDIDATE}"
        ),
        Vec::new(),
    )?;
    let t2_count = r.fig(
        "t2_row_count",
        support::count(TEST_ID, t2_refs.len())?,
        Unit::Count,
        "Bank or cash vouchers whose narration names, as whole words, a party ledger with no \
         amount on the voucher.",
        t2_refs,
    )?;
    let t2_sum = r.fig(
        "t2_candidate_total_paise",
        paise(t2_total)?,
        Unit::Paise,
        &format!(
            "Bank and cash lines of the vouchers whose narration names a party ledger with no \
             amount on the voucher, summed{CANDIDATE}{ONCE}"
        ),
        Vec::new(),
    )?;
    let pairs: usize = per_name.values().map(|p| p.rows).sum();
    let t2_pairs = r.fig(
        "t2_named_pair_count",
        support::count(TEST_ID, pairs)?,
        Unit::Count,
        "Voucher and named-party pairs across the groups: equal to the voucher count unless a \
         narration names more than one party.",
        Vec::new(),
    )?;
    let emb_count = r.fig(
        "t2_embedded_row_count",
        support::count(TEST_ID, emb_refs.len())?,
        Unit::Count,
        "Bank or cash vouchers whose narration holds a party ledger's name only inside a longer \
         name (another name-like word beside it in the same part of the narration); listed apart \
         from the others.",
        emb_refs.clone(),
    )?;
    let emb_sum = r.fig(
        "t2_embedded_candidate_total_paise",
        paise(emb_total)?,
        Unit::Paise,
        &format!(
            "Bank and cash lines of the vouchers whose narration holds a party's name only inside \
             a longer name, summed{CANDIDATE}{ONCE}"
        ),
        Vec::new(),
    )?;
    let mut named_refs = Vec::new();
    for (name, p) in &per_name {
        let tag = stable_ledger_tag(book, name)?;
        r.fig(
            &format!("t2_named_count_{tag}"),
            support::count(TEST_ID, p.rows)?,
            Unit::Count,
            &format!(
                "Bank or cash vouchers whose narration names one party ledger (tag {tag}) with no \
                 amount on the voucher; a voucher naming two parties counts in both groups."
            ),
            p.refs.clone(),
        )?;
        r.fig(
            &format!("t2_named_total_paise_{tag}"),
            paise(p.total)?,
            Unit::Paise,
            &format!(
                "Bank and cash lines of the vouchers naming one party ledger (tag {tag}), \
                 summed{CANDIDATE}{ONCE}"
            ),
            Vec::new(),
        )?;
        named_refs.push(EvidenceRef::with_label(
            "ledger",
            name,
            &format!(
                "named in {} voucher(s), {} (a candidate total, not a reportable amount)",
                p.rows,
                rupees(p.total)
            ),
        ));
    }

    let finding = |key: &str,
                   title: &str,
                   own_limit: &str,
                   ask: &str,
                   facts: Vec<(&str, &String)>,
                   evidence: Vec<EvidenceRef>| Finding {
        id: format!("{TEST_ID}/{key}"),
        clauses: Vec::new(),
        title: title.to_string(),
        facts: facts
            .into_iter()
            .map(|(n, id)| (n.to_string(), id.clone()))
            .collect(),
        evidence,
        confidence: Confidence::JudgementRequired,
        limits: vec![
            LIMIT_LISTED.to_string(),
            LIMIT_RECALL.to_string(),
            own_limit.to_string(),
        ],
        ask_client: vec![ask.to_string()],
    };
    if !t1_refs.is_empty() {
        r.findings.push(finding(
            "t1",
            "Journals settling one party through another, with no bank or cash line: listed for \
             the CA (may bear on clauses 31 and 21(d))",
            "Same-person set-offs, rectifications of a wrong-party posting, opening or migration \
             entries, branch ledgers of one entity, and discount or note journals against one \
             party are listed too.",
            "Journal settling one party through another: whose liability, and who received the \
             value? Is it a loan or deposit taken or repaid by journal (clause 31, codes I/J), a \
             same-person set-off, or a correction?",
            vec![("candidate_total", &t1_sum), ("rows", &t1_count)],
            t1_refs,
        ));
    }
    if !per_name.is_empty() {
        r.findings.push(finding(
            "t2",
            "Bank or cash entries whose narration names a party other than the one booked: listed \
             for the CA (may bear on clause 21(d) and s.68)",
            "A name is matched only whole and in full, and only when no other master shares it; \
             an accent on an English letter is ignored, so two names differing only by one are \
             shared and matched to neither. A name cut short, broken up, abbreviated, spelt \
             differently, written in another alphabet than the ledger's (Devanagari for English \
             letters, or the reverse), or garbled in conversion (a stray character joined to a \
             word) is missed, and a short or common name may be a coincidence (the proprietor, a \
             family member, the operator).",
            ASK_T2,
            vec![
                ("candidate_total", &t2_sum),
                ("named_pairs", &t2_pairs),
                ("rows", &t2_count),
            ],
            named_refs,
        ));
    }
    if !emb_refs.is_empty() {
        let mut evidence: Vec<EvidenceRef> = inside_rows
            .iter()
            .map(|(name, k)| {
                EvidenceRef::with_label(
                    "ledger",
                    name,
                    &format!("inside a longer name in {k} voucher(s)"),
                )
            })
            .collect();
        evidence.extend(emb_refs);
        r.findings.push(finding(
            "t2_embedded",
            "Bank or cash entries whose narration holds a party's name inside a longer name: \
             listed apart for the CA (may bear on clause 21(d) and s.68)",
            "The narration's name is longer than the party's (for example a person's name before \
             a firm's): it may be that party or another one. It is listed apart so that the CA \
             sees the difference; nothing is refused or dropped.",
            ASK_T2,
            vec![("candidate_total", &emb_sum), ("rows", &emb_count)],
            evidence,
        ));
    }
    r.population_note = POPULATION_NOTE.to_string();
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{Ledger, LedgerLine, VoucherStatus};
    use crate::error::AuditError;
    use bridge_tally_primitives::TallyDate;

    fn ledger(name: &str, group: &str, guid: &str) -> (String, Ledger) {
        let l = Ledger {
            name: name.to_string(),
            parent: group.to_string(),
            chain: vec![group.to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: guid.to_string(),
            masterid: None,
        };
        (name.to_string(), l)
    }

    /// A Payment of Rs 100 from `Bank A` to `Rent`, with `narration`.
    fn payment(guid: &str, narration: &str) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse("20250501").unwrap(),
            vtype: "Payment".to_string(),
            base_type: "Payment".to_string(),
            number: guid.to_string(),
            status: VoucherStatus::Regular,
            narration: narration.to_string(),
            lines: vec![
                LedgerLine {
                    ledger: "Rent".to_string(),
                    amount_paise: 10_000,
                },
                LedgerLine {
                    ledger: "Bank A".to_string(),
                    amount_paise: -10_000,
                },
            ],
            ..Default::default()
        }
    }

    /// `Alpha Traders` carries the GUID `beta traders`; `beta traders` has none, so its tag is the
    /// hash of its name, which is the hash of the other's normalised GUID: one tag, `59119993`.
    fn book(narration: &str) -> Book {
        Book {
            ledgers: [
                ledger("Alpha Traders", "Sundry Debtors", "BETA TRADERS"),
                ledger("beta traders", "Sundry Creditors", ""),
                ledger("Bank A", "Bank Accounts", "g-bank"),
                ledger("Rent", "Indirect Expenses", "g-rent"),
            ]
            .into_iter()
            .collect(),
            vouchers: vec![payment("p1", narration)],
            ..Default::default()
        }
    }

    /// The money amount tells a Contra by the voucher's base type, not its type name, as the
    /// reference's `money_of` does: a Contra-based type under another name counts the larger side
    /// of its money lines once, and a type named Contra on another base sums each money line. Each
    /// voucher has one line outside bank and cash, so only the Contra test decides.
    #[test]
    fn a_contra_is_told_by_its_base_type_not_its_name() {
        let money = BTreeSet::from(["Bank A", "Cash"]);
        let voucher = |vtype: &str, base_type: &str| Voucher {
            vtype: vtype.to_string(),
            base_type: base_type.to_string(),
            lines: [
                ("Bank A", 10_000),
                ("Cash", -9_000),
                ("Bank Charges", -1_000),
            ]
            .into_iter()
            .map(|(ledger, amount_paise)| LedgerLine {
                ledger: ledger.to_string(),
                amount_paise,
            })
            .collect(),
            ..payment("c1", "")
        };
        assert_eq!(
            money_amount(&voucher("Cash Deposit", "Contra"), &money),
            10_000
        );
        assert_eq!(money_amount(&voucher("Contra", "Payment"), &money), 19_000);
    }

    #[test]
    fn two_named_ledgers_sharing_a_tag_stop_the_test_with_the_repeated_figure_id() {
        let rules = Rules::vendored().unwrap();
        let err = run(&book("Alpha Traders / beta traders"), &rules, &[]).unwrap_err();
        assert!(
            matches!(&err, AuditError::DuplicateFigureId(id)
                if id == "knock_off_candidates.t2_named_count_59119993"),
            "{err:?}"
        );
        // The control: one of the two named, and the other only inside a longer name.
        for narration in ["Alpha Traders", "Alpha Traders / Omega beta traders"] {
            assert!(run(&book(narration), &rules, &[]).is_ok(), "{narration}");
        }
    }

    #[test]
    fn a_voucher_of_unknown_status_refuses_before_any_figure() {
        let mut b = book("Alpha Traders");
        b.vouchers[0].status = VoucherStatus::Unknown;
        let refused = run(&b, &Rules::vendored().unwrap(), &[]);
        assert!(matches!(refused, Err(AuditError::UnknownVoucherStatus(1))));
    }

    fn text(cps: &[u32]) -> String {
        cps.iter().map(|cp| char::from_u32(*cp).unwrap()).collect()
    }

    fn cps(text: &str) -> Vec<u32> {
        text.chars().map(u32::from).collect()
    }

    /// `tests/fixtures/unicode-probes.json`, made by `parity/unicode_tables.py` from Python 3.13:
    /// each table's edges, the late code points, and, for each sequence, Python's NFD, its
    /// case-fold after the cut, and the generator's model of the tokens of README section 3.3.
    #[test]
    fn the_tables_and_the_reader_agree_with_python_on_every_probe() {
        let probes: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/unicode-probes.json")).unwrap();
        assert!(include_str!("unicode_tables.rs")
            .contains(&format!("; {}.", probes["python"].as_str().unwrap())));
        let ints = |v: &serde_json::Value| -> Vec<u32> {
            v.as_array()
                .unwrap()
                .iter()
                .map(|x| u32::try_from(x.as_u64().unwrap()).unwrap())
                .collect()
        };
        for (kind, letter, mark) in [
            ("letter_or_number", true, false),
            ("mark", false, true),
            ("other", false, false),
        ] {
            for cp in ints(&probes["categories"][kind]) {
                let c = char::from_u32(cp).unwrap();
                assert_eq!(
                    (in_table(&LETTER_OR_NUMBER, c), in_table(&MARK, c)),
                    (letter, mark),
                    "U+{cp:04X}"
                );
            }
        }
        let late: Vec<u32> = LATE.iter().flat_map(|&(a, b)| a..=b).collect();
        assert_eq!(late, ints(&probes["late"]));
        let sequences = probes["sequences"].as_array().unwrap();
        assert!(sequences.len() > 800, "{}", sequences.len());
        for s in sequences {
            let input = text(&ints(&s[0]));
            assert_eq!(cps(&decomposed(&input, false)), ints(&s[1]), "{:?}", s[0]);
            assert_eq!(cps(&decomposed(&input, true)), ints(&s[2]), "{:?}", s[0]);
            let want: Vec<Vec<u32>> = s[3].as_array().unwrap().iter().map(ints).collect();
            let got: Vec<Vec<u32>> = tokens(&input).iter().map(|t| cps(t)).collect();
            assert_eq!(got, want, "{:?}", s[0]);
        }
    }
}
