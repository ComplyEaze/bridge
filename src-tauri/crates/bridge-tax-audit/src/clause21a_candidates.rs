// SPDX-License-Identifier: Apache-2.0
//! Port of the reference engine's `clause21a_candidates`: Form 3CD clause 21(a), the P&L debits that
//! may be one of the clause's items, listed for the CA and never judged.
//!
//! Clause 21(a) asks for amounts debited to P&L that are capital, personal, a political party's
//! advertisement, club fees or services, penalties or fines, compounding fees, a benefit breaching
//! the recipient's conduct rules, or a settlement under a notified law. None of these is a books
//! fact. The test lists candidates by the words of the keyword table ([`KEYWORDS`], data, each table
//! citing its source; the client's `[clause21a].extra_terms` add words, never remove them), over
//! every debit line on a ledger under Purchase Accounts, Direct Expenses or Indirect Expenses, and
//! asks each item's question once.
//!
//! Two rules need no word: a large entry (`[ledger_scrutiny].large_entry_paise` from the rules, or
//! `ledger_scrutiny`'s own default when they carry none, as the reference reads it) on a repairs, maintenance, software, computer or office expenses
//! ledger is a capital candidate, and a debit in a voucher that credits a Capital Account ledger is a
//! personal candidate, except on a partner's interest or remuneration ledger (the s.40(b) booking).
//! Credits on the same ledgers that match are shown beside, never netted. Late-fee and
//! statutory-interest words are their own items; a line that also carries a penalty word is listed
//! under both. Nothing is judged and nothing is left out for being small.
//!
//! One divergence in output from the reference, a refusal: a candidate or credit total past the
//! 64-bit range is refused with a typed error, where the reference's unbounded integers print it.
//! A second sits in the rules loader, not here, and cannot be reached with the vendored rules: a
//! `[ledger_scrutiny]` table that lacks `large_entry_paise` is refused, where the reference falls
//! back to its default.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::book::{Book, Voucher};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::rules::Rules;
use crate::support::{overflow, py_lower, py_strip, rupees, voucher_label};
use crate::PartnersConfig;

pub const TEST_ID: &str = "clause21a_candidates";
pub const VERSION: &str = "1";

/// The keyword table: the reference's own file at its commit `ccc8f8a0`, byte for byte
/// (`parity/python_golden.py` refuses to run the reference unless this copy equals its file).
pub const KEYWORDS: &str = include_str!("../rules/clause21a_keywords.toml");
/// sha256 of [`KEYWORDS`], also recorded in `tests/fixtures/provenance/clause21a-candidates-787.md`.
#[cfg(test)]
const KEYWORDS_SHA256: &str = "28cb01475ab4ca72b8f9fb1cfb29ef4dcace3f56614d6fb64703d6685b4721b3";

const PL_GROUPS: [&str; 3] = ["Purchase Accounts", "Direct Expenses", "Indirect Expenses"];
const CAPITAL_ACCOUNT_GROUP: &str = "Capital Account";

/// One item of clause 21(a) the keyword table has words for, declared in the table's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bucket {
    Capital,
    Personal,
    PoliticalAd,
    ClubFee,
    ClubService,
    PenaltyOrFine,
    LateFee,
    StatutoryInterest,
    Compounding,
    BenefitPerquisite,
    Settlement,
}

impl Bucket {
    pub const ALL: [Self; 11] = [
        Self::Capital,
        Self::Personal,
        Self::PoliticalAd,
        Self::ClubFee,
        Self::ClubService,
        Self::PenaltyOrFine,
        Self::LateFee,
        Self::StatutoryInterest,
        Self::Compounding,
        Self::BenefitPerquisite,
        Self::Settlement,
    ];

    /// The table's name for the item, which also names its figures and finding.
    pub fn name(self) -> &'static str {
        match self {
            Self::Capital => "capital",
            Self::Personal => "personal",
            Self::PoliticalAd => "political_ad",
            Self::ClubFee => "club_fee",
            Self::ClubService => "club_service",
            Self::PenaltyOrFine => "penalty_or_fine",
            Self::LateFee => "late_fee",
            Self::StatutoryInterest => "statutory_interest",
            Self::Compounding => "compounding",
            Self::BenefitPerquisite => "benefit_perquisite",
            Self::Settlement => "settlement",
        }
    }

    /// The item with exactly this name, if the table has one.
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.name() == name)
    }
}

/// Whether a character is in the reference pattern's `[0-9a-z]`: the only neighbours that stop a
/// term matching. Narrower than `\b`: a term after "é" matches.
fn bounding(c: char) -> bool {
    c.is_ascii_digit() || c.is_ascii_lowercase()
}

/// The reference's `_matcher`: its terms Python-stripped and lowered, blank ones dropped, each
/// matched literally on lowered text where neither neighbour is in `[0-9a-z]`.
#[derive(Debug, Clone)]
struct Matcher(Vec<Vec<char>>);

impl Matcher {
    /// `None` when no term is left, as the reference's `_matcher` returns None.
    fn new<'a>(terms: impl IntoIterator<Item = &'a String>) -> Option<Self> {
        let terms: Vec<Vec<char>> = terms
            .into_iter()
            .map(|t| py_strip(t))
            .filter(|t| !t.is_empty())
            .map(|t| py_lower(t).chars().collect())
            .collect();
        (!terms.is_empty()).then_some(Self(terms))
    }

    /// `search` on text already lowered.
    fn search(&self, text: &[char]) -> bool {
        (0..text.len()).any(|at| {
            (at == 0 || !bounding(text[at - 1]))
                && self.0.iter().any(|t| {
                    let end = at + t.len();
                    end <= text.len()
                        && text[at..end] == t[..]
                        && text.get(end).is_none_or(|&c| !bounding(c))
                })
        })
    }
}

/// One item's row of the keyword table.
#[derive(Debug, Clone)]
struct Item {
    /// The clause's item as the finding names it.
    item: String,
    source: String,
    question: String,
    /// The words read in the ledger name, the narration and the party field.
    terms: Vec<String>,
    /// Words that count only beside one of these, anywhere in the same text.
    club: Option<Matcher>,
    /// Words read in the ledger name or the narration, never the party field.
    own_only: Option<Matcher>,
    /// Words that count only with one of their qualifiers in the same ledger name or narration.
    qualified: Option<(Matcher, Matcher)>,
}

/// The keyword table, typed: every item of [`Bucket`] once, each list of words present or empty.
#[derive(Debug, Clone)]
pub struct KeywordTable {
    items: BTreeMap<Bucket, Item>,
    /// The capital item's ledgers whose large entries are candidates without a word.
    large_entry_ledgers: Option<Matcher>,
}

fn table_error(detail: impl std::fmt::Display) -> AuditError {
    AuditError::Config(format!("{TEST_ID}: keyword table: {detail}"))
}

impl KeywordTable {
    /// The vendored table.
    pub fn vendored() -> Result<Self> {
        Self::parse(KEYWORDS)
    }

    /// Parse a keyword table. Refused: a table the reference does not name, an item missing, an
    /// unknown key, a value of the wrong type, and a list of words that is given but strips to
    /// nothing where the reference would then call a method on None.
    pub fn parse(text: &str) -> Result<Self> {
        let table: toml::Table = text.parse().map_err(table_error)?;
        let mut items = BTreeMap::new();
        let mut large_entry_ledgers = None;
        for (name, row) in &table {
            let bucket = Bucket::parse(name)
                .ok_or_else(|| table_error(format!("[{name}] is not an item")))?;
            let row = row
                .as_table()
                .ok_or_else(|| table_error(format!("[{name}] is not a table")))?;
            const KEYS: [&str; 9] = [
                "source",
                "item",
                "question",
                "terms",
                "club_terms",
                "ledger_or_narration_terms",
                "qualified_terms",
                "qualifiers",
                "large_entry_ledger_terms",
            ];
            if let Some(key) = row.keys().find(|k| !KEYS.contains(&k.as_str())) {
                return Err(table_error(format!("[{name}].{key} is not read")));
            }
            let text = |key: &str| {
                row.get(key)
                    .and_then(toml::Value::as_str)
                    .map(str::to_string)
                    .ok_or_else(|| table_error(format!("[{name}].{key} is not text")))
            };
            let list = |key: &str| -> Result<Vec<String>> {
                row.get(key).map_or(Ok(Vec::new()), |v| {
                    v.as_array()
                        .and_then(|a| a.iter().map(|t| t.as_str().map(str::to_string)).collect())
                        .ok_or_else(|| {
                            table_error(format!("[{name}].{key} is not a list of words"))
                        })
                })
            };
            // A list the reference reads only when it is non-empty, and then needs a matcher from.
            let required = |key: &str| -> Result<Option<Matcher>> {
                let words = list(key)?;
                if words.is_empty() {
                    return Ok(None);
                }
                Matcher::new(&words)
                    .map(Some)
                    .ok_or_else(|| table_error(format!("[{name}].{key} has only blank words")))
            };
            let qualified = match required("qualified_terms")? {
                None => None,
                Some(q) => {
                    let w = required("qualifiers")?.ok_or_else(|| {
                        table_error(format!("[{name}] has qualified terms and no qualifiers"))
                    })?;
                    Some((q, w))
                }
            };
            if bucket == Bucket::Capital {
                large_entry_ledgers = Matcher::new(&list("large_entry_ledger_terms")?);
            }
            items.insert(
                bucket,
                Item {
                    item: text("item")?,
                    source: text("source")?,
                    question: text("question")?,
                    terms: list("terms")?,
                    club: required("club_terms")?,
                    own_only: required("ledger_or_narration_terms")?,
                    qualified,
                },
            );
        }
        if let Some(missing) = Bucket::ALL.iter().find(|b| !items.contains_key(b)) {
            return Err(table_error(format!("no [{}] table", missing.name())));
        }
        Ok(Self {
            items,
            large_entry_ledgers,
        })
    }
}

/// The client's `[clause21a].extra_terms`: words added to an item's, never removing any.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtraTerms(BTreeMap<Bucket, Vec<String>>);

/// `[clause21a].extra_terms` as the reference's `clause21a_extra_terms` reads it, given the
/// `[clause21a]` value: absent is none; otherwise a table of item name -> list of words. Refused, as
/// the reference refuses them: `[clause21a]` that is not a table, `extra_terms` that is not a table
/// of lists of words, and an item the keyword table does not have.
pub fn extra_terms(clause21a: Option<&toml::Value>) -> Result<ExtraTerms> {
    let Some(clause21a) = clause21a else {
        return Ok(ExtraTerms::default());
    };
    let table = clause21a
        .as_table()
        .ok_or_else(|| AuditError::Config("[clause21a] is not a table".to_string()))?;
    let Some(raw) = table.get("extra_terms") else {
        return Ok(ExtraTerms::default());
    };
    let malformed = || {
        AuditError::Config(
            "[clause21a].extra_terms must map a bucket name to a list of words".to_string(),
        )
    };
    let mut out = BTreeMap::new();
    let mut unknown = BTreeSet::new();
    for (name, words) in raw.as_table().ok_or_else(malformed)? {
        let words: Vec<String> = words
            .as_array()
            .and_then(|a| a.iter().map(|t| t.as_str().map(str::to_string)).collect())
            .ok_or_else(malformed)?;
        match Bucket::parse(name) {
            Some(b) => {
                out.insert(b, words);
            }
            None => {
                unknown.insert(name.clone());
            }
        }
    }
    if !unknown.is_empty() {
        return Err(AuditError::Config(format!(
            "[clause21a].extra_terms names buckets the keyword table does not have: {:?}",
            unknown.into_iter().collect::<Vec<_>>()
        )));
    }
    Ok(ExtraTerms(out))
}

/// The partners' interest and remuneration ledgers, as the reference's pack takes them from the
/// bound `[partners]` table: each entry's `interest_ledger` and `remuneration_ledger`, kept when it
/// names a ledger. An entry that is not a table is refused, as the reference's `q.get` fails there;
/// so is a value that is not a name, which binding has already refused.
pub fn partner_ledgers(partners: &PartnersConfig) -> Result<BTreeSet<String>> {
    let mut out = BTreeSet::new();
    for (key, entry) in &partners.partners {
        let entry = entry
            .as_table()
            .ok_or_else(|| AuditError::Config(format!("[partners].{key} is not a table")))?;
        for location in ["interest_ledger", "remuneration_ledger"] {
            match entry.get(location) {
                None => {}
                Some(toml::Value::String(name)) => {
                    if !name.is_empty() {
                        out.insert(name.clone());
                    }
                }
                Some(_) => {
                    return Err(AuditError::Config(format!(
                        "[partners].{key}.{location} is not a name"
                    )))
                }
            }
        }
    }
    Ok(out)
}

/// One matched line: the voucher's position in the population and the line's in the voucher.
type Key = (usize, usize);

/// `extra_terms`: the client's words, added to the table's. `partner_ledgers`: the partners'
/// interest and remuneration ledgers ([`partner_ledgers`]); their credit to capital is the s.40(b)
/// booking, so the capital-credit rule never makes them personal candidates, though a word still can.
/// Refuses, as the population does, while any voucher's status is unknown.
pub fn run(
    book: &Book,
    rules: &Rules,
    extra_terms: &ExtraTerms,
    partner_ledgers: &BTreeSet<String>,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.population_note = "Every debit line on a ledger under Purchase Accounts, Direct Expenses or \
Indirect Expenses, in the books population; credits on those ledgers that match are shown beside."
        .to_string();
    let table = KeywordTable::vendored()?;
    let pl = book.ledgers_under_any(&PL_GROUPS.map(str::to_string));
    let capital_ledgers = book.ledgers_under_any(&[CAPITAL_ACCOUNT_GROUP.to_string()]);
    let large = rules
        .ledger_scrutiny_large_entry_paise
        .unwrap_or(crate::ledger_scrutiny::DEFAULT_LARGE_ENTRY_PAISE);
    let words: BTreeMap<Bucket, Option<Matcher>> = table
        .items
        .iter()
        .map(|(b, t)| {
            let extra = extra_terms.0.get(b).into_iter().flatten();
            (*b, Matcher::new(t.terms.iter().chain(extra)))
        })
        .collect();

    // Per item, each matched line once, in the population's order.
    let mut hits: BTreeMap<Bucket, Vec<(Key, &Voucher, &str, i64)>> = BTreeMap::new();
    let population = book.population()?;
    for (vi, v) in population.iter().enumerate() {
        let contra_capital = v
            .lines
            .iter()
            .any(|l| capital_ledgers.contains(&l.ledger) && l.amount_paise < 0);
        for (li, l) in v.lines.iter().enumerate() {
            // The reference passes over a nil line here. This loop need not: a nil line is neither
            // a debit nor a credit below, so it reaches no figure and no count.
            if !pl.contains(&l.ledger) {
                continue;
            }
            let text: Vec<char> =
                py_lower(&format!("{} {} {}", l.ledger, v.narration, v.party_field))
                    .chars()
                    .collect();
            // The ledger name and the narration, never the party.
            let own: [Vec<char>; 2] = [
                py_lower(&l.ledger).chars().collect(),
                py_lower(&v.narration).chars().collect(),
            ];
            for (b, t) in &table.items {
                let mut hit = words[b].as_ref().is_some_and(|m| m.search(&text));
                if !hit {
                    if let Some(m) = &t.own_only {
                        hit = own.iter().any(|f| m.search(f));
                    }
                }
                if !hit {
                    if let Some((q, w)) = &t.qualified {
                        hit = own.iter().any(|f| q.search(f) && w.search(f));
                    }
                }
                if hit {
                    if let Some(c) = &t.club {
                        hit = c.search(&text);
                    }
                }
                if *b == Bucket::Capital
                    && l.amount_paise > large
                    && table
                        .large_entry_ledgers
                        .as_ref()
                        .is_some_and(|m| m.search(&own[0]))
                {
                    hit = true;
                }
                if *b == Bucket::Personal
                    && contra_capital
                    && l.amount_paise > 0
                    && !partner_ledgers.contains(&l.ledger)
                {
                    hit = true;
                }
                if hit {
                    hits.entry(*b).or_default().push((
                        (vi, li),
                        v,
                        l.ledger.as_str(),
                        l.amount_paise,
                    ));
                }
            }
        }
    }

    let mut in_buckets: HashMap<Key, usize> = HashMap::new();
    for key in hits.values().flatten().map(|h| h.0) {
        *in_buckets.entry(key).or_default() += 1;
    }

    for (b, t) in &table.items {
        let Some(matched) = hits.get(b) else {
            continue;
        };
        let mut debits: Vec<_> = matched.iter().filter(|h| h.3 > 0).collect();
        if debits.is_empty() {
            continue;
        }
        // Largest first, then by GUID; a stable sort, as the reference's `sorted`.
        debits.sort_by(|x, y| y.3.cmp(&x.3).then_with(|| x.1.guid.cmp(&y.1.guid)));
        let total = debits
            .iter()
            .try_fold(0i64, |s, h| s.checked_add(h.3))
            .ok_or_else(|| overflow(TEST_ID))?;
        let credit_total = matched
            .iter()
            .filter(|h| h.3 < 0)
            .try_fold(0i64, |s, h| s.checked_sub(h.3))
            .ok_or_else(|| overflow(TEST_ID))?;
        let evidence: Vec<EvidenceRef> = debits
            .iter()
            .map(|(key, v, ledger, amt)| {
                let multiple = if in_buckets[key] > 1 {
                    ", multiple matches"
                } else {
                    ""
                };
                EvidenceRef::with_label(
                    "voucher",
                    &v.guid,
                    &format!(
                        "{}, {ledger}, {}{multiple}",
                        voucher_label(v),
                        rupees(i128::from(*amt))
                    ),
                )
            })
            .collect();
        let name = b.name();
        let item = &t.item;
        let f_total = r.fig(
            &format!("candidate_total_{name}"),
            Value::Int(total),
            Unit::Paise,
            &format!(
                "Clause 21(a) {item}: the P&L debits matched as candidates, summed -- a candidate \
total, not a reportable amount."
            ),
            evidence.clone(),
        )?;
        let f_credit = r.fig(
            &format!("credits_total_{name}"),
            Value::Int(credit_total),
            Unit::Paise,
            &format!(
                "Credits on the same P&L ledgers matching the {item} words: shown beside, never \
netted."
            ),
            Vec::new(),
        )?;
        r.findings.push(Finding {
            id: format!("{TEST_ID}/{name}"),
            clauses: vec!["3CD-21(a)".to_string()],
            title: format!("Clause 21(a) {item}: candidates listed for the CA, not reported"),
            facts: vec![
                ("candidate_total".to_string(), f_total),
                ("credits_total".to_string(), f_credit),
            ],
            evidence,
            confidence: Confidence::JudgementRequired,
            limits: vec![
                format!(
                    "{} P&L debit line(s) match the words for this item ({}); a match is a \
candidate, never a conclusion, and nothing is left out for being small. The CA decides what, if \
anything, is reported; no clause 21(a) row is written.",
                    debits.len(),
                    t.source
                ),
                "Only lines on ledgers under Purchase Accounts, Direct Expenses or Indirect \
Expenses were searched. Keyword recall is not measured: a narration in Hindi, Hinglish or left blank \
is missed, and a penalty booked only in the GST cash ledger, netted in a party account, or on a \
credit-card statement is not in these books."
                    .to_string(),
            ],
            ask_client: vec![t.question.clone()],
        });
    }
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::{
        extra_terms, partner_ledgers, Bucket, ExtraTerms, KeywordTable, Matcher, KEYWORDS,
        KEYWORDS_SHA256,
    };
    use crate::error::AuditError;
    use crate::PartnersConfig;
    use sha2::{Digest, Sha256};

    fn search(terms: &[&str], text: &str) -> bool {
        let terms: Vec<String> = terms.iter().map(|t| (*t).to_string()).collect();
        Matcher::new(&terms)
            .unwrap()
            .search(&crate::support::py_lower(text).chars().collect::<Vec<_>>())
    }

    #[test]
    fn the_vendored_table_matches_its_recorded_hash_and_has_every_item() {
        assert_eq!(
            crate::canonical::hex(&Sha256::digest(KEYWORDS.as_bytes())),
            KEYWORDS_SHA256,
            "rules/clause21a_keywords.toml changed; regenerate the goldens and the provenance row"
        );
        let table = KeywordTable::vendored().unwrap();
        assert_eq!(table.items.keys().copied().collect::<Vec<_>>(), Bucket::ALL);
        assert!(table.large_entry_ledgers.is_some());
        for b in Bucket::ALL {
            assert_eq!(Bucket::parse(b.name()), Some(b));
        }
    }

    #[test]
    fn a_term_matches_only_between_characters_outside_a_z_and_0_9() {
        assert!(search(&["ac"], "AC repair"));
        assert!(!search(&["ac"], "packing"));
        assert!(!search(&["234e"], "ref 1234e"));
        assert!(search(&["234e"], "u/s 234E,"));
        assert!(search(&["penalty"], "épenalty"));
        assert!(!search(&["penalty"], "penaltyx"));
        assert!(search(&["late fee"], "x late fee"));
        assert!(search(&[" Diwali Gifts "], "diwali gifts"));
        assert!(search(&["201(1a)"], "u/s 201(1A)"));
        assert!(Matcher::new(&[String::new(), "  ".to_string()]).is_none());
    }

    #[test]
    fn a_malformed_keyword_table_is_refused() {
        let unknown_key = KEYWORDS.replacen("[settlement]\n", "[settlement]\nwords = []\n", 1);
        let unknown_item = format!("{KEYWORDS}\n[gifts]\nitem = \"x\"\n");
        // [settlement] is the last table.
        let missing_item = KEYWORDS[..KEYWORDS.find("[settlement]").unwrap()].to_string();
        let blank_club = KEYWORDS.replacen(
            "club_terms = [\"club\", \"gymkhana\", \"golf\"]",
            "club_terms = [\" \"]",
            1,
        );
        let no_qualifiers = KEYWORDS
            .lines()
            .filter(|l| !l.starts_with("qualifiers = "))
            .collect::<Vec<_>>()
            .join("\n");
        let not_text = KEYWORDS.replacen("item = \"(vii)", "item = 7 #", 1);
        for bad in [
            unknown_key,
            unknown_item,
            missing_item,
            blank_club,
            no_qualifiers,
            not_text,
        ] {
            assert_ne!(bad, KEYWORDS);
            assert!(matches!(
                KeywordTable::parse(&bad),
                Err(AuditError::Config(_))
            ));
        }
    }

    #[test]
    fn extra_terms_are_a_table_of_lists_of_words_for_known_items() {
        let v = |s: &str| -> toml::Value { toml::from_str(s).unwrap() };
        assert_eq!(extra_terms(None).unwrap(), ExtraTerms::default());
        assert_eq!(
            extra_terms(Some(&v("x = 1"))).unwrap(),
            ExtraTerms::default()
        );
        let read = extra_terms(Some(&v("extra_terms = { personal = [\"gym\"] }"))).unwrap();
        assert_eq!(read.0[&Bucket::Personal], ["gym"]);
        for bad in [
            toml::Value::String("x".to_string()),
            v("extra_terms = [\"gym\"]"),
            v("extra_terms = { personal = \"gym\" }"),
            v("extra_terms = { personal = [1] }"),
            v("extra_terms = { gifts = [\"gym\"] }"),
        ] {
            assert!(
                matches!(extra_terms(Some(&bad)), Err(AuditError::Config(_))),
                "{bad}"
            );
        }
    }

    #[test]
    fn partner_ledgers_are_the_named_interest_and_remuneration_ledgers() {
        let p = |s: &str| PartnersConfig {
            partners: toml::from_str::<toml::Table>(s)
                .unwrap()
                .into_iter()
                .collect(),
            deed: None,
        };
        let got = partner_ledgers(&p(
            "a = { interest_ledger = \"Interest\", capital_ledgers = [\"Cap\"] }\n\
             b = { interest_ledger = \"\", remuneration_ledger = \"Remuneration\" }",
        ))
        .unwrap();
        assert_eq!(
            got.into_iter().collect::<Vec<_>>(),
            ["Interest", "Remuneration"]
        );
        for bad in ["a = 1", "a = { interest_ledger = 5 }"] {
            assert!(matches!(
                partner_ledgers(&p(bad)),
                Err(AuditError::Config(_))
            ));
        }
    }

    /// The rules' own large-entry threshold, where they carry one, replaces the default (the edge
    /// books can only drop the rules' table, and its value equals the default).
    #[test]
    fn the_rules_large_entry_threshold_is_read() {
        use crate::book::{Book, Ledger, LedgerLine, Voucher, VoucherStatus};
        let ledger = |name: &str, group: &str| Ledger {
            name: name.to_string(),
            parent: group.to_string(),
            chain: vec![group.to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: format!("invented-{name}"),
            masterid: None,
        };
        let voucher = |guid: &str, paise: i64| Voucher {
            guid: guid.to_string(),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            number: guid.to_string(),
            status: VoucherStatus::Regular,
            lines: vec![
                LedgerLine {
                    ledger: "Repairs".to_string(),
                    amount_paise: paise,
                },
                LedgerLine {
                    ledger: "Cash".to_string(),
                    amount_paise: -paise,
                },
            ],
            ..Default::default()
        };
        let book = Book {
            ledgers: [
                ledger("Repairs", "Indirect Expenses"),
                ledger("Cash", "Cash-in-Hand"),
            ]
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect(),
            vouchers: vec![voucher("J1", 101), voucher("J2", 100)],
            ..Default::default()
        };
        let mut rules = crate::rules::Rules::vendored().unwrap();
        rules.ledger_scrutiny_large_entry_paise = Some(100);
        let r = super::run(&book, &rules, &ExtraTerms::default(), &Default::default()).unwrap();
        let total = r
            .figures
            .iter()
            .find(|f| f.id == "clause21a_candidates.candidate_total_capital")
            .unwrap();
        assert_eq!(total.value, crate::findings::Value::Int(101));
        assert_eq!(total.evidence.len(), 1);
    }

    /// A voucher of [`penalty_book`]: its GUID, its number and its lines.
    type PenaltyVoucher<'a> = (&'a str, &'a str, &'a [(&'a str, i64)]);

    /// A book of journals on 2025-06-01, each with a penalty word in its narration, every ledger
    /// but Cash under Indirect Expenses.
    fn penalty_book(vouchers: &[PenaltyVoucher<'_>]) -> crate::book::Book {
        use crate::book::{Book, Ledger, LedgerLine, Voucher, VoucherStatus};
        let ledger = |name: &str, group: &str| Ledger {
            name: name.to_string(),
            parent: group.to_string(),
            chain: vec![group.to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: format!("invented-{name}"),
            masterid: None,
        };
        Book {
            ledgers: [
                ledger("Misc Expenses", "Indirect Expenses"),
                ledger("Penalty Paid", "Indirect Expenses"),
                ledger("Cash", "Cash-in-Hand"),
            ]
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect(),
            vouchers: vouchers
                .iter()
                .map(|(guid, number, lines)| Voucher {
                    guid: (*guid).to_string(),
                    date: bridge_tally_primitives::TallyDate::parse("20250601").unwrap(),
                    vtype: "Journal".to_string(),
                    base_type: "Journal".to_string(),
                    number: (*number).to_string(),
                    narration: "penalty".to_string(),
                    status: VoucherStatus::Regular,
                    lines: lines
                        .iter()
                        .map(|(ledger, paise)| LedgerLine {
                            ledger: (*ledger).to_string(),
                            amount_paise: *paise,
                        })
                        .collect(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
    }

    /// The comparer re-sorts evidence, so no golden holds the order a figure and its finding cite
    /// their candidates in: largest first, then by GUID, then the books' order (two lines of one
    /// voucher). Here the largest amount is on the last GUID, the books' order is the reverse of
    /// the GUIDs', the voucher numbers run against the GUIDs too, and the two lines of one voucher
    /// are not in ledger-name order, so each rule is told from its neighbours and from the two
    /// keys taken the other way round. The labels are what the reference prints, in its own
    /// order, for this book.
    #[test]
    fn candidates_are_cited_largest_first_then_by_guid_then_in_the_books_order() {
        let book = penalty_book(&[
            ("v-c", "7", &[("Misc Expenses", 40_000), ("Cash", -40_000)]),
            ("v-b", "1", &[("Misc Expenses", 30_000), ("Cash", -30_000)]),
            (
                "v-a",
                "2",
                &[
                    ("Penalty Paid", 30_000),
                    ("Misc Expenses", 30_000),
                    ("Cash", -60_000),
                ],
            ),
        ]);
        let rules = crate::rules::Rules::vendored().unwrap();
        let r = super::run(&book, &rules, &ExtraTerms::default(), &Default::default()).unwrap();
        let expected = [
            "Journal 7 on 2025-06-01, Misc Expenses, ₹400",
            "Journal 2 on 2025-06-01, Penalty Paid, ₹300",
            "Journal 2 on 2025-06-01, Misc Expenses, ₹300",
            "Journal 1 on 2025-06-01, Misc Expenses, ₹300",
        ];
        let total = r
            .figures
            .iter()
            .find(|f| f.id == "clause21a_candidates.candidate_total_penalty_or_fine")
            .unwrap();
        assert_eq!(total.value, crate::findings::Value::Int(130_000));
        let labels = |e: &[crate::findings::EvidenceRef]| -> Vec<String> {
            e.iter().map(|x| x.label.clone()).collect()
        };
        assert_eq!(labels(&total.evidence), expected);
        let finding = r
            .findings
            .iter()
            .find(|f| f.id == "clause21a_candidates/penalty_or_fine")
            .unwrap();
        assert_eq!(labels(&finding.evidence), expected);
    }

    /// The reference's integers are unbounded; here a total past the 64-bit range is refused with
    /// the crate's typed overflow error, never wrapped: a sum of debits, one credit whose negation
    /// is out of range, and a sum of credits.
    #[test]
    fn a_total_past_the_64_bit_range_is_refused_not_wrapped() {
        let rules = crate::rules::Rules::vendored().unwrap();
        let refused = |book: &crate::book::Book| {
            let err = super::run(book, &rules, &ExtraTerms::default(), &Default::default())
                .expect_err("refused");
            assert!(matches!(err, AuditError::Config(_)), "{err}");
            assert_eq!(
                err.to_string(),
                crate::support::overflow(super::TEST_ID).to_string()
            );
        };
        let half = i64::MAX / 2 + 1;
        refused(&penalty_book(&[
            ("v-1", "1", &[("Penalty Paid", half), ("Cash", -half)]),
            ("v-2", "2", &[("Penalty Paid", half), ("Cash", -half)]),
        ]));
        refused(&penalty_book(&[
            ("v-1", "1", &[("Penalty Paid", 10_000), ("Cash", -10_000)]),
            (
                "v-2",
                "2",
                &[("Penalty Paid", i64::MIN), ("Cash", i64::MAX)],
            ),
        ]));
        refused(&penalty_book(&[
            ("v-1", "1", &[("Penalty Paid", 10_000), ("Cash", -10_000)]),
            ("v-2", "2", &[("Penalty Paid", -half), ("Cash", half)]),
            ("v-3", "3", &[("Penalty Paid", -half), ("Cash", half)]),
        ]));
    }

    #[test]
    fn a_voucher_of_unknown_status_refuses_the_test() {
        use crate::book::{Book, Voucher};
        let book = Book {
            vouchers: vec![Voucher::default()],
            ..Default::default()
        };
        let rules = crate::rules::Rules::vendored().unwrap();
        assert!(matches!(
            super::run(&book, &rules, &ExtraTerms::default(), &Default::default()),
            Err(AuditError::UnknownVoucherStatus(1))
        ));
    }
}
