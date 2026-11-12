// SPDX-License-Identifier: Apache-2.0
//! Form 3CD clause 42, the cash receipts that may be Form 61A rows under portal code SFT-013, listed
//! for the CA and never judged. A crate-first test: the reference engine has no such module, so
//! there is no reference golden to compare it with (`docs/tax-audit/parity-spec-v1.md`, section 12).
//!
//! Each voucher of the books population, Contra excluded, whose debits to the engagement's cash
//! ledgers come to strictly more than the rules file's limit ([`RULES`]) is one candidate. The sum
//! is taken on that one voucher only: nothing is added across vouchers, days, parties or bills, and
//! no counterparty is left out. Whether a receipt is for a sale of goods or services, and whether
//! several receipts pay one sale, the books do not say; each candidate is the CA's to decide.

use std::collections::BTreeSet;

use crate::book::{Book, Voucher};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, Finding, TestResult, Unit, Value};
use crate::rules::Rules;
use crate::support::{count, overflow, voucher_label, voucher_refs};

pub const TEST_ID: &str = "clause42_sft";
pub const VERSION: &str = "1";

/// The rules file, data: the limit and its source.
pub const RULES: &str = include_str!("../rules/clause42_sft.toml");
/// sha256 of [`RULES`].
#[cfg(test)]
const RULES_SHA256: &str = "71568f6237ef4c6167ed0ab2045337fc80ecc66a0faf53aac2be7684de503536";

const POPULATION: &str =
    "Books population (optional, cancelled and post-dated vouchers excluded); Contra excluded";

/// The limit a voucher's cash debits must be strictly over, in paise; positive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limit(i64);

impl Limit {
    pub fn paise(self) -> i64 {
        self.0
    }

    /// The vendored rules file's limit.
    pub fn vendored() -> Result<Self> {
        Self::parse(RULES)
    }

    /// Parse a rules file. Refused: a table or key not read here, a limit that is not a positive
    /// integer, and a source that is not text.
    pub fn parse(text: &str) -> Result<Self> {
        let error = |detail: &str| AuditError::Config(format!("{TEST_ID}: rules: {detail}"));
        let table = text
            .parse::<toml::Table>()
            .map_err(|e| error(&e.to_string()))?;
        if let Some(name) = table.keys().find(|k| k.as_str() != "sft_013") {
            return Err(error(&format!("[{name}] is not read")));
        }
        let row = table
            .get("sft_013")
            .and_then(toml::Value::as_table)
            .ok_or_else(|| error("[sft_013] is not a table"))?;
        if let Some(key) = row
            .keys()
            .find(|k| !["source", "limit_paise"].contains(&k.as_str()))
        {
            return Err(error(&format!("[sft_013].{key} is not read")));
        }
        row.get("source")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| error("[sft_013].source is not text"))?;
        match row.get("limit_paise").and_then(toml::Value::as_integer) {
            Some(paise) if paise > 0 => Ok(Self(paise)),
            _ => Err(error("[sft_013].limit_paise is not a positive integer")),
        }
    }
}

/// The voucher's debits to the cash ledgers, summed; `None` past the 64-bit range.
fn cash_debits(v: &Voucher, cash: &BTreeSet<String>) -> Option<i64> {
    v.lines
        .iter()
        .filter(|l| l.amount_paise > 0 && cash.contains(&l.ledger))
        .try_fold(0i64, |sum, l| sum.checked_add(l.amount_paise))
}

pub fn run(
    book: &Book,
    rules: &Rules,
    limit: Limit,
    cash: &BTreeSet<String>,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let mut candidates: Vec<(&Voucher, i64)> = Vec::new();
    for v in book.population()? {
        if v.base_type == "Contra" {
            continue;
        }
        let debits = cash_debits(v, cash).ok_or_else(|| overflow(TEST_ID))?;
        if debits > limit.paise() {
            candidates.push((v, debits));
        }
    }
    candidates.sort_by(|(a, _), (b, _)| {
        (a.date.as_str(), voucher_label(a), &a.guid).cmp(&(
            b.date.as_str(),
            voucher_label(b),
            &b.guid,
        ))
    });
    r.population_note = POPULATION.to_string();
    let limit_id = r.fig(
        "limit",
        Value::Int(limit.paise()),
        Unit::Paise,
        "The limit from rules/clause42_sft.toml; a voucher is listed when its cash debits are strictly more.",
        Vec::new(),
    )?;
    let count_id = r.fig(
        "candidates",
        count(TEST_ID, candidates.len())?,
        Unit::Count,
        &format!("Vouchers whose debits to cash ledgers are over the limit. {POPULATION}"),
        voucher_refs(candidates.iter().map(|(v, _)| *v)),
    )?;
    let total = candidates
        .iter()
        .try_fold(0i64, |sum, (_, debits)| sum.checked_add(*debits))
        .ok_or_else(|| overflow(TEST_ID))?;
    let total_id = r.fig(
        "candidate_cash",
        Value::Int(total),
        Unit::Paise,
        "Debits to cash ledgers on the listed vouchers, each voucher's own.",
        voucher_refs(candidates.iter().map(|(v, _)| *v)),
    )?;
    for (index, (v, debits)) in candidates.iter().enumerate() {
        r.fig(
            &format!("voucher_{}", index + 1),
            Value::Int(*debits),
            Unit::Paise,
            &format!("Debits to cash ledgers on {}.", voucher_label(v)),
            voucher_refs([*v]),
        )?;
    }
    r.findings.push(Finding {
        id: format!("{TEST_ID}/candidates"),
        clauses: vec!["3CD-42".to_string(), "Rule 114E".to_string()],
        title: "Vouchers whose cash debits are over the limit, each on its own".to_string(),
        facts: vec![
            ("candidates".to_string(), count_id),
            ("candidate_cash".to_string(), total_id),
            ("limit".to_string(), limit_id),
        ],
        evidence: voucher_refs(candidates.iter().map(|(v, _)| *v)),
        confidence: Confidence::JudgementRequired,
        limits: vec![
            "Books only: whether a receipt is for a sale of goods or services is not established."
                .to_string(),
            "Bill-wise allocations are not read, so receipts that pay one sale are not told apart."
                .to_string(),
        ],
        ask_client: Vec::new(),
    });
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{LedgerLine, VoucherStatus};
    use bridge_tally_primitives::TallyDate;
    use sha2::{Digest, Sha256};
    use std::collections::BTreeMap;

    const LIMIT: i64 = 20_000_000;

    fn line(ledger: &str, amount_paise: i64) -> LedgerLine {
        LedgerLine {
            ledger: ledger.to_string(),
            amount_paise,
        }
    }

    fn voucher(guid: &str, base_type: &str, lines: Vec<LedgerLine>) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse("20250610").unwrap(),
            vtype: base_type.to_string(),
            base_type: base_type.to_string(),
            number: guid.to_string(),
            status: VoucherStatus::Regular,
            lines,
            ..Voucher::default()
        }
    }

    fn book(vouchers: Vec<Voucher>) -> Book {
        Book {
            company_name: "Synthetic".to_string(),
            company_guid: "test-guid".to_string(),
            read_at: String::new(),
            groups: BTreeMap::new(),
            group_masters: BTreeMap::new(),
            ledgers: BTreeMap::new(),
            stock: None,
            currency_read: false,
            opening_stock: crate::book::OpeningStock::NotApplicable,
            closing_stock: crate::book::ClosingStock::NotApplicable,
            bill_allocations: crate::book::BillAllocations::NotRead,
            vouchers,
            tb: BTreeMap::new(),
        }
    }

    fn rules() -> Rules {
        Rules::vendored().unwrap()
    }

    fn cash() -> BTreeSet<String> {
        BTreeSet::from(["Cash".to_string(), "Petty Cash".to_string()])
    }

    /// The listed vouchers' GUIDs, in the result's order.
    fn listed(book: &Book) -> Vec<String> {
        let r = run(book, &rules(), Limit::vendored().unwrap(), &cash()).unwrap();
        r.findings[0]
            .evidence
            .iter()
            .map(|e| e.id.clone())
            .collect()
    }

    #[test]
    fn the_vendored_rules_match_their_recorded_hash() {
        assert_eq!(
            crate::canonical::hex(&Sha256::digest(RULES.as_bytes())),
            RULES_SHA256,
            "rules/clause42_sft.toml changed; review the limit and its source"
        );
        assert_eq!(Limit::vendored().unwrap().paise(), LIMIT);
    }

    #[test]
    fn a_rules_file_with_anything_else_is_refused() {
        let ok = "[sft_013]\nsource = \"s\"\nlimit_paise = 5\n";
        assert_eq!(Limit::parse(ok).unwrap().paise(), 5);
        for bad in [
            "[sft_013]\nsource = \"s\"\nlimit_paise = 0\n",
            "[sft_013]\nsource = \"s\"\nlimit_paise = -1\n",
            "[sft_013]\nsource = \"s\"\nlimit_paise = \"5\"\n",
            "[sft_013]\nsource = \"s\"\n",
            "[sft_013]\nlimit_paise = 5\n",
            "[sft_013]\nsource = \"s\"\nlimit_paise = 5\naggregate = true\n",
            "[sft_013]\nsource = \"s\"\nlimit_paise = 5\n[sft_012]\nlimit_paise = 5\n",
            "not toml",
        ] {
            let err = Limit::parse(bad).unwrap_err();
            assert!(
                matches!(&err, AuditError::Config(m) if m.starts_with("clause42_sft: rules: ")),
                "{bad:?}: {err}"
            );
        }
    }

    /// Exactly the limit is not listed, one paisa more is.
    #[test]
    fn the_limit_is_strict() {
        let b = book(vec![
            voucher(
                "at",
                "Receipt",
                vec![line("Cash", LIMIT), line("P", -LIMIT)],
            ),
            voucher(
                "over",
                "Receipt",
                vec![line("Cash", LIMIT + 1), line("P", -LIMIT - 1)],
            ),
        ]);
        assert_eq!(listed(&b), ["over"]);
    }

    /// Two vouchers under the limit are never added, however close in date or party; the cash
    /// lines of one voucher are, across cash ledgers; a bank line beside them is not cash; and a
    /// cash credit on the same voucher (change given back) does not reduce its cash debits.
    #[test]
    fn only_one_vouchers_cash_debits_are_summed() {
        let half = LIMIT / 2 + 1;
        let b = book(vec![
            voucher(
                "gross",
                "Receipt",
                vec![
                    line("Cash", LIMIT + 1),
                    line("Cash", -2),
                    line("P", 1 - LIMIT),
                ],
            ),
            voucher("a", "Receipt", vec![line("Cash", half), line("P", -half)]),
            voucher("b", "Receipt", vec![line("Cash", half), line("P", -half)]),
            voucher(
                "two-cash",
                "Receipt",
                vec![
                    line("Cash", half),
                    line("Petty Cash", half),
                    line("P", -2 * half),
                ],
            ),
            voucher(
                "cash-and-bank",
                "Receipt",
                vec![line("Cash", half), line("Bank", half), line("P", -2 * half)],
            ),
        ]);
        assert_eq!(listed(&b), ["gross", "two-cash"]);
    }

    /// A cash credit is a payment, never a receipt; Contra is left out whichever way it moves
    /// cash; and a voucher outside the books population is not listed.
    #[test]
    fn payments_contra_and_excluded_vouchers_are_not_listed() {
        let big = LIMIT * 2;
        let mut optional = voucher(
            "optional",
            "Receipt",
            vec![line("Cash", big), line("P", -big)],
        );
        optional.status = VoucherStatus::Optional;
        let b = book(vec![
            voucher("paid", "Payment", vec![line("Cash", -big), line("P", big)]),
            voucher(
                "withdrawn",
                "Contra",
                vec![line("Cash", big), line("Bank", -big)],
            ),
            optional,
            voucher(
                "sale",
                "Sales",
                vec![line("Cash", big), line("Sales", -big)],
            ),
        ]);
        assert_eq!(listed(&b), ["sale"]);
    }

    /// The finding cites every candidate and its figures, asks nothing and concludes nothing.
    #[test]
    fn the_finding_lists_candidates_for_the_ca_and_answers_no_question() {
        let big = LIMIT * 2;
        let b = book(vec![voucher(
            "one",
            "Receipt",
            vec![line("Cash", big), line("Capital", -big)],
        )]);
        let r = run(&b, &rules(), Limit::vendored().unwrap(), &cash()).unwrap();
        let value = |id: &str| {
            r.figures
                .iter()
                .find(|f| f.id == id)
                .unwrap_or_else(|| panic!("no figure {id}"))
                .value
                .clone()
        };
        assert_eq!(value("clause42_sft.limit"), Value::Int(LIMIT));
        assert_eq!(value("clause42_sft.candidates"), Value::Int(1));
        assert_eq!(value("clause42_sft.candidate_cash"), Value::Int(big));
        assert_eq!(value("clause42_sft.voucher_1"), Value::Int(big));
        let f = &r.findings[0];
        assert_eq!(f.confidence, Confidence::JudgementRequired);
        assert!(f.ask_client.is_empty());
        assert_eq!(f.evidence.len(), 1);
        let (_, violations) = crate::invariants::result_invariants(&b, &r);
        assert!(violations.is_empty(), "{violations:?}");
    }

    /// Candidates are numbered by date, whatever order the book holds them in.
    #[test]
    fn candidates_are_numbered_by_date() {
        let (big, bigger) = (LIMIT + 1, LIMIT + 2);
        let mut late = voucher(
            "a-late",
            "Receipt",
            vec![line("Cash", big), line("P", -big)],
        );
        late.date = TallyDate::parse("20260301").unwrap();
        let early = voucher(
            "b-early",
            "Receipt",
            vec![line("Cash", bigger), line("P", -bigger)],
        );
        let r = run(
            &book(vec![late, early]),
            &rules(),
            Limit::vendored().unwrap(),
            &cash(),
        )
        .unwrap();
        let first = r
            .figures
            .iter()
            .find(|f| f.id == "clause42_sft.voucher_1")
            .unwrap();
        assert_eq!(first.value, Value::Int(bigger));
        assert_eq!(first.evidence[0].id, "b-early");
    }

    #[test]
    fn a_book_with_no_candidate_still_states_the_limit_and_a_zero_count() {
        let r = run(
            &book(Vec::new()),
            &rules(),
            Limit::vendored().unwrap(),
            &cash(),
        )
        .unwrap();
        assert_eq!(r.figures.len(), 3);
        assert_eq!(r.figures[1].value, Value::Int(0));
        assert!(r.findings[0].evidence.is_empty());
    }

    #[test]
    fn a_cash_total_past_the_range_is_refused() {
        let b = book(vec![voucher(
            "huge",
            "Receipt",
            vec![line("Cash", i64::MAX), line("Petty Cash", 1)],
        )]);
        let err = run(&b, &rules(), Limit::vendored().unwrap(), &cash()).unwrap_err();
        assert!(
            matches!(&err, AuditError::Config(m) if m == "clause42_sft: a total overflowed i64 paise"),
            "{err}"
        );
    }
}
