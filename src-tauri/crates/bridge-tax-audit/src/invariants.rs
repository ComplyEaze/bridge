//! Invariants: properties that must hold before a figure leaves the engine (the reference
//! Python implementation's own invariants module). Each re-derives what it needs from the book,
//! never from a test's own computation. Codes and violation strings match the reference exactly,
//! because the parity dump compares them.
//!
//! POL-1 is not here: the reference implementation calls it an observation, not a gate, and the
//! parity dump excludes it.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::{Book, OpeningStock, TbRow, VoucherStatus};
use crate::error::Result;
use crate::findings::TestResult;
use crate::read::iso;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Violation {
    pub invariant: String,
    pub subject: String,
    pub detail: String,
}

fn violation(invariant: &str, subject: &str, detail: String) -> Violation {
    Violation {
        invariant: invariant.to_string(),
        subject: subject.to_string(),
        detail,
    }
}

/// POP-1 tolerance: the TB tie tolerates one rupee.
const TIE_TOLERANCE_PAISE: i64 = 100;

/// The book invariants, in the reference's `BOOK_INVARIANTS` order: the codes evaluated, and
/// every violation. Like the reference, this refuses (through `population`) when a voucher's
/// status is unknown, so POP-0 can only ever report on a book that is already refused.
pub fn book_invariants(book: &Book) -> Result<(Vec<&'static str>, Vec<Violation>)> {
    let mut out = Vec::new();

    // ID-1 (Bridge ADR 0002): the book is bound to a company GUID.
    if book.company_guid.is_empty() {
        out.push(violation(
            "ID-1",
            &book.company_name,
            "no company GUID".to_string(),
        ));
    }

    // POP-0: every voucher's status is established.
    for v in &book.vouchers {
        if v.status == VoucherStatus::Unknown {
            out.push(violation("POP-0", &v.guid, "status unknown".to_string()));
        }
    }

    let population = book.population()?;

    // POP-1: per ledger, the sum of in-books voucher lines equals TB closing minus opening.
    let mut movement: BTreeMap<&str, i64> = BTreeMap::new();
    for v in &population {
        for l in &v.lines {
            *movement.entry(l.ledger.as_str()).or_default() += l.amount_paise;
        }
    }
    let names: BTreeSet<&str> = movement
        .keys()
        .copied()
        .chain(book.tb.keys().map(String::as_str))
        .collect();
    for name in names {
        let want = book.tb.get(name).map_or(0, TbRow::movement_paise);
        let got = movement.get(name).copied().unwrap_or(0);
        if (got - want).abs() > TIE_TOLERANCE_PAISE {
            out.push(violation(
                "POP-1",
                name,
                format!("vouchers {got} vs TB movement {want} paise"),
            ));
        }
    }

    // POP-2: every in-books voucher's lines sum to zero.
    for v in &population {
        let sum: i64 = v.lines.iter().map(|l| l.amount_paise).sum();
        if !v.lines.is_empty() && sum.abs() > 1 {
            out.push(violation(
                "POP-2",
                &v.guid,
                format!("{} {} {} lines sum {sum}", v.vtype, v.number, iso(&v.date)),
            ));
        }
    }

    // POP-3: TB opening and closing each sum to zero once opening stock, which no ledger holds,
    // is added (#1486); what remains is a difference in opening balances, reported, not hidden.
    for (key, sum) in [
        (
            "opening_paise",
            book.tb.values().map(|t| t.opening_paise).sum::<i64>(),
        ),
        (
            "closing_paise",
            book.tb.values().map(|t| t.closing_paise).sum::<i64>(),
        ),
    ] {
        if let Some(detail) = pop3_detail(key, sum, book.opening_stock) {
            out.push(violation("POP-3", key, detail));
        }
    }

    // POP-5: every in-books voucher has a GUID of its own. Evidence names a voucher by its GUID,
    // so a blank or repeated GUID makes a citation ambiguous: one violation for all blank GUIDs,
    // then one per repeated GUID, in GUID order.
    let mut guid_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for v in &population {
        *guid_counts.entry(v.guid.as_str()).or_insert(0) += 1;
    }
    if let Some(blank) = guid_counts.get("") {
        out.push(violation(
            "POP-5",
            "(blank)",
            format!("{blank} in-books voucher(s) have no GUID"),
        ));
    }
    for (guid, count) in &guid_counts {
        if !guid.is_empty() && *count > 1 {
            out.push(violation(
                "POP-5",
                guid,
                format!("{count} in-books vouchers share this GUID"),
            ));
        }
    }

    // MAP-0: every in-books voucher line posts to a ledger the book's masters carry. MAP-1 and
    // every role lookup skip a ledger with no master, and POP-1 is silent when its lines net to
    // zero; one violation per such ledger, with its line count.
    let mut unknown: BTreeMap<&str, usize> = BTreeMap::new();
    for v in &population {
        for l in &v.lines {
            if !book.ledgers.contains_key(&l.ledger) {
                *unknown.entry(l.ledger.as_str()).or_insert(0) += 1;
            }
        }
    }
    for (name, count) in unknown {
        out.push(violation(
            "MAP-0",
            name,
            format!(
                "{count} in-books voucher line(s) post to it; the book has no ledger master for it"
            ),
        ));
    }

    // MAP-1: every ledger with vouchers resolves to a primary group.
    let used: BTreeSet<&str> = population
        .iter()
        .flat_map(|v| v.lines.iter().map(|l| l.ledger.as_str()))
        .collect();
    for name in used {
        if let Some(ledger) = book.ledgers.get(name) {
            if !ledger.chain_complete {
                out.push(violation(
                    "MAP-1",
                    name,
                    format!("chain incomplete: {}", ledger.chain.join(" > ")),
                ));
            }
        }
    }

    Ok((
        vec![
            "ID-1", "POP-0", "POP-1", "POP-2", "POP-3", "POP-5", "MAP-0", "MAP-1",
        ],
        out,
    ))
}

/// POP-3's detail for one sum, or `None` when it ties. With no opening stock to add the text is
/// the reference's original one, so a book without inventory reads exactly as before.
fn pop3_detail(key: &str, sum: i64, stock: OpeningStock) -> Option<String> {
    match stock {
        OpeningStock::Valued(stock) if stock != 0 => {
            let difference = sum + stock;
            (difference.abs() > TIE_TOLERANCE_PAISE).then(|| {
                format!(
                    "TB {key} sums to {sum} paise; with opening stock of {stock} paise the \
                     difference in opening balances is {difference} paise"
                )
            })
        }
        OpeningStock::Unknown(why) => (sum.abs() > TIE_TOLERANCE_PAISE).then(|| {
            format!(
                "TB {key} sums to {sum} paise; opening stock {}",
                why.as_str()
            )
        }),
        OpeningStock::NotApplicable | OpeningStock::Valued(_) => {
            (sum.abs() > TIE_TOLERANCE_PAISE).then(|| format!("TB {key} sums to {sum} paise"))
        }
    }
}

/// The three result-level invariants, evaluated with this one result (REND-0, EVID-1, POP-4).
pub fn result_invariants(book: &Book, result: &TestResult) -> (Vec<&'static str>, Vec<Violation>) {
    let mut out = Vec::new();

    // REND-0: every fact in a finding points at a figure that exists.
    let figures: BTreeSet<&str> = result.figures.iter().map(|f| f.id.as_str()).collect();
    for finding in &result.findings {
        for (name, figure_id) in &finding.facts {
            if !figures.contains(figure_id.as_str()) {
                out.push(violation(
                    "REND-0",
                    &finding.id,
                    format!("fact {name} -> missing figure {figure_id}"),
                ));
            }
        }
    }

    let refs: Vec<_> = result
        .figures
        .iter()
        .flat_map(|f| f.evidence.iter())
        .chain(result.findings.iter().flat_map(|f| f.evidence.iter()))
        .collect();

    // EVID-1: every voucher (either kind) or ledger evidence ref resolves in the book.
    let guids: BTreeSet<&str> = book.vouchers.iter().map(|v| v.guid.as_str()).collect();
    for e in &refs {
        let unresolved = ((e.kind == "voucher" || e.kind == "excluded_voucher")
            && !guids.contains(e.id.as_str()))
            || (e.kind == "ledger" && !book.ledgers.contains_key(&e.id));
        if unresolved {
            out.push(violation(
                "EVID-1",
                &result.test_id,
                format!("unresolved {}", e.key()),
            ));
        }
    }

    // POP-4: no figure or finding cites a voucher outside the books population -- except through
    // an "excluded_voucher" ref, which states that purpose and must itself point at an excluded
    // voucher. Both directions are checked, so the marker cannot hide a population voucher.
    let excluded: BTreeSet<&str> = book.excluded().map(|v| v.guid.as_str()).collect();
    for e in &refs {
        if e.kind == "voucher" && excluded.contains(e.id.as_str()) {
            out.push(violation("POP-4", &result.test_id, e.key()));
        }
        if e.kind == "excluded_voucher"
            && guids.contains(e.id.as_str())
            && !excluded.contains(e.id.as_str())
        {
            out.push(violation(
                "POP-4",
                &result.test_id,
                format!("{} is in the books population", e.key()),
            ));
        }
    }

    (vec!["REND-0", "EVID-1", "POP-4"], out)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use bridge_tally_primitives::TallyDate;

    use super::*;
    use crate::book::Voucher;
    use crate::findings::{EvidenceRef, Unit, Value};

    fn voucher(guid: &str, status: VoucherStatus) -> Voucher {
        Voucher {
            narration: String::new(),
            party_field: String::new(),
            guid: guid.to_string(),
            date: TallyDate::parse("20250601").unwrap(),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            number: String::new(),
            status,
            lines: Vec::new(),
            ..Default::default()
        }
    }

    fn book() -> Book {
        Book {
            company_name: "Synthetic".to_string(),
            company_guid: "test-guid".to_string(),
            read_at: String::new(),
            groups: BTreeMap::new(),
            group_masters: BTreeMap::new(),
            ledgers: BTreeMap::new(),
            vouchers: vec![
                voucher("in-books", VoucherStatus::Regular),
                voucher("left-out", VoucherStatus::Optional),
            ],
            tb: BTreeMap::new(),
            ..Default::default()
        }
    }

    /// POP-5 (reference `inv_voucher_guids_unique`): one violation for all blank GUIDs, then one
    /// per repeated GUID in GUID order; an excluded voucher is outside it. The texts are the
    /// reference's own.
    #[test]
    fn pop5_names_blank_and_repeated_voucher_guids_in_the_population() {
        let mut b = book();
        b.vouchers = vec![
            voucher("g-2", VoucherStatus::Regular),
            voucher("", VoucherStatus::Regular),
            voucher("g-1", VoucherStatus::Regular),
            voucher("g-2", VoucherStatus::Regular),
            voucher("", VoucherStatus::Regular),
            voucher("g-1", VoucherStatus::Regular),
            voucher("g-1", VoucherStatus::Optional),
            voucher("g-3", VoucherStatus::Regular),
            voucher("g-3", VoucherStatus::Cancelled),
        ];
        let (evaluated, found) = book_invariants(&b).unwrap();
        assert_eq!(
            evaluated,
            vec!["ID-1", "POP-0", "POP-1", "POP-2", "POP-3", "POP-5", "MAP-0", "MAP-1"]
        );
        let pop5: Vec<(String, String)> = found
            .into_iter()
            .filter(|v| v.invariant == "POP-5")
            .map(|v| (v.subject, v.detail))
            .collect();
        assert_eq!(
            pop5,
            vec![
                (
                    "(blank)".to_string(),
                    "2 in-books voucher(s) have no GUID".to_string()
                ),
                (
                    "g-1".to_string(),
                    "2 in-books vouchers share this GUID".to_string()
                ),
                (
                    "g-2".to_string(),
                    "2 in-books vouchers share this GUID".to_string()
                ),
            ]
        );
    }

    fn violations(kind: &str, guid: &str, code: &str) -> Vec<String> {
        let mut r = TestResult::new("t", "1", "r");
        r.fig(
            "f",
            Value::Int(1),
            Unit::Count,
            "d",
            vec![EvidenceRef::new(kind, guid)],
        )
        .unwrap();
        result_invariants(&book(), &r)
            .1
            .into_iter()
            .filter(|v| v.invariant == code)
            .map(|v| v.detail)
            .collect()
    }

    /// Negative control: the marker must not have weakened the ordinary check.
    #[test]
    fn a_plain_voucher_reference_to_an_excluded_voucher_still_fires() {
        assert_eq!(
            violations("voucher", "left-out", "POP-4"),
            vec!["voucher:left-out"]
        );
    }

    #[test]
    fn an_excluded_voucher_reference_to_an_excluded_voucher_is_clean() {
        assert!(violations("excluded_voucher", "left-out", "POP-4").is_empty());
    }

    /// Negative control in the other direction: the marker cannot hide a population voucher.
    #[test]
    fn an_excluded_voucher_reference_to_a_books_voucher_fires() {
        assert_eq!(
            violations("excluded_voucher", "in-books", "POP-4"),
            vec!["excluded_voucher:in-books is in the books population"]
        );
    }

    fn lines(l: &[(&str, i64)]) -> Vec<crate::book::LedgerLine> {
        l.iter()
            .map(|(n, a)| crate::book::LedgerLine {
                ledger: (*n).to_string(),
                amount_paise: *a,
            })
            .collect()
    }

    fn ledger(name: &str) -> crate::book::Ledger {
        crate::book::Ledger {
            name: name.to_string(),
            parent: "Indirect Expenses".to_string(),
            chain: vec!["Indirect Expenses".to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: String::new(),
            masterid: None,
        }
    }

    fn map0(b: &Book) -> Vec<(String, String)> {
        book_invariants(b)
            .unwrap()
            .1
            .into_iter()
            .filter(|v| v.invariant == "MAP-0")
            .map(|v| (v.subject, v.detail))
            .collect()
    }

    /// MAP-0 is neither looser nor stricter than the join every test uses: exact, case-sensitive
    /// lookup in `book.ledgers`. A line naming "Round Off" against a master stored as "ROUND OFF"
    /// is joined by no test, so MAP-0 names it (the reference pins the same case).
    #[test]
    fn map0_matches_the_join_rule_a_case_only_difference_fires() {
        let mut b = book();
        b.ledgers
            .insert("ROUND OFF".to_string(), ledger("ROUND OFF"));
        b.vouchers = vec![Voucher {
            lines: lines(&[("Round Off", 100), ("ROUND OFF", -100)]),
            ..voucher("a", VoucherStatus::Regular)
        }];
        assert!(!b.ledgers.contains_key("Round Off")); // the join misses it...
        assert_eq!(
            map0(&b), // ...so MAP-0 fires, once
            vec![(
                "Round Off".to_string(),
                "1 in-books voucher line(s) post to it; the book has no ledger master for it"
                    .to_string()
            )]
        );
    }

    /// Several unknown ledgers are reported in name order, one each, each with its own count.
    #[test]
    fn map0_reports_several_ledgers_in_name_order() {
        let mut b = book();
        b.vouchers = vec![Voucher {
            lines: lines(&[("Zeta", 100), ("Alpha", -60), ("Alpha", -40)]),
            ..voucher("a", VoucherStatus::Regular)
        }];
        let got: Vec<(String, String)> = map0(&b)
            .into_iter()
            .map(|(s, d)| (s, d[..1].to_string()))
            .collect();
        assert_eq!(
            got,
            vec![
                ("Alpha".to_string(), "2".to_string()),
                ("Zeta".to_string(), "1".to_string())
            ]
        );
    }

    /// MAP-0: a line whose ledger has no master is skipped by MAP-1 and by every role lookup;
    /// when its lines net to zero POP-1 is silent too. MAP-0 names it, once per ledger, counting
    /// in-books lines only. The same book, detail and count as the reference's own test.
    #[test]
    fn map0_names_a_ledger_with_no_master() {
        let mut b = book();
        b.ledgers.insert("X".to_string(), ledger("X"));
        b.tb.insert(
            "X".to_string(),
            TbRow {
                opening_paise: 0,
                debit_paise: 0,
                credit_paise: 0,
                closing_paise: 0,
            },
        );
        b.vouchers = vec![
            Voucher {
                lines: lines(&[("X", 100), ("Ghost", -100)]),
                ..voucher("a", VoucherStatus::Regular)
            },
            Voucher {
                lines: lines(&[("X", -100), ("Ghost", 100)]),
                ..voucher("b", VoucherStatus::Regular)
            },
            Voucher {
                lines: lines(&[("X", 5), ("Phantom", -5)]),
                ..voucher("c", VoucherStatus::Optional)
            },
        ];
        let (codes, viol) = book_invariants(&b).unwrap();
        assert!(codes.contains(&"MAP-0"));
        let got: Vec<(&str, &str, &str)> = viol
            .iter()
            .map(|v| (v.invariant.as_str(), v.subject.as_str(), v.detail.as_str()))
            .collect();
        assert_eq!(
            got,
            vec![(
                "MAP-0",
                "Ghost",
                "2 in-books voucher line(s) post to it; the book has no ledger master for it"
            )]
        );
        // Negative control: with Ghost's master present, nothing fires.
        b.ledgers.insert("Ghost".to_string(), ledger("Ghost"));
        assert!(book_invariants(&b).unwrap().1.is_empty());
    }

    #[test]
    fn evid1_resolves_the_new_kind() {
        assert!(violations("excluded_voucher", "left-out", "EVID-1").is_empty());
        assert_eq!(
            violations("excluded_voucher", "nowhere", "EVID-1"),
            vec!["unresolved excluded_voucher:nowhere"]
        );
        // An unresolvable ref is EVID-1's to report, not POP-4's.
        assert!(violations("excluded_voucher", "nowhere", "POP-4").is_empty());
    }

    /// POP-3's violations on a book whose ledger openings sum to `sum` (closing the same: no
    /// movement), with `stock` as its opening stock.
    fn pop3(sum: i64, stock: OpeningStock) -> Vec<(String, String)> {
        let mut b = book();
        let row = |opening_paise: i64| TbRow {
            opening_paise,
            debit_paise: 0,
            credit_paise: 0,
            closing_paise: opening_paise,
        };
        b.tb.insert("Shop debit".to_string(), row(20_000));
        b.tb.insert("Capital".to_string(), row(sum - 20_000));
        b.opening_stock = stock;
        let (_, found) = book_invariants(&b).unwrap();
        let mut pop3: Vec<(String, String)> = found
            .into_iter()
            .filter(|v| v.invariant == "POP-3")
            .map(|v| (v.subject, v.detail))
            .collect();
        pop3.sort();
        pop3
    }

    /// #1486: opening stock, which no ledger holds, is added before the sums are tested, and what
    /// remains is named as the difference in opening balances.
    #[test]
    fn pop3_adds_opening_stock_and_reports_only_the_difference() {
        assert!(pop3(-80_000, OpeningStock::Valued(80_000)).is_empty());
        let both = |detail: &str| {
            vec![
                (
                    "closing_paise".to_string(),
                    detail.replace("{k}", "closing_paise"),
                ),
                (
                    "opening_paise".to_string(),
                    detail.replace("{k}", "opening_paise"),
                ),
            ]
        };
        assert_eq!(
            pop3(-100_000, OpeningStock::Valued(80_000)),
            both(
                "TB {k} sums to -100000 paise; with opening stock of 80000 paise the \
                 difference in opening balances is -20000 paise"
            )
        );
    }

    /// Without a stock term the text is the reference's original one, so no book without
    /// inventory moves; an unknown term is named.
    #[test]
    fn pop3_without_a_stock_term_reads_as_before_and_names_an_unknown_one() {
        let plain = vec![
            (
                "closing_paise".to_string(),
                "TB closing_paise sums to -100000 paise".to_string(),
            ),
            (
                "opening_paise".to_string(),
                "TB opening_paise sums to -100000 paise".to_string(),
            ),
        ];
        assert_eq!(pop3(-100_000, OpeningStock::NotApplicable), plain);
        assert_eq!(pop3(-100_000, OpeningStock::Valued(0)), plain);
        assert_eq!(
            pop3(
                -100_000,
                OpeningStock::Unknown(crate::book::OpeningStockUnknown::NotRead)
            ),
            vec![
                (
                    "closing_paise".to_string(),
                    "TB closing_paise sums to -100000 paise; opening stock not read".to_string(),
                ),
                (
                    "opening_paise".to_string(),
                    "TB opening_paise sums to -100000 paise; opening stock not read".to_string(),
                ),
            ]
        );
        assert!(pop3(
            0,
            OpeningStock::Unknown(crate::book::OpeningStockUnknown::NotRead)
        )
        .is_empty());
    }

    /// The one-rupee tolerance holds on every arm: 100 paise ties and 101 does not, a positive
    /// remainder is reported with its sign, and ledgers that sum to zero with opening stock
    /// beside them report the whole stock as the difference.
    #[test]
    fn pop3_tolerance_sign_and_a_zero_ledger_sum() {
        let unknown = OpeningStock::Unknown(crate::book::OpeningStockUnknown::Unreadable);
        assert!(pop3(-80_100, OpeningStock::Valued(80_000)).is_empty());
        assert!(pop3(100, unknown).is_empty());
        assert!(pop3(100, OpeningStock::NotApplicable).is_empty());
        let one = |sum: i64, stock: OpeningStock| pop3(sum, stock).len();
        assert_eq!(one(-80_101, OpeningStock::Valued(80_000)), 2);
        assert_eq!(one(101, unknown), 2);
        assert_eq!(one(101, OpeningStock::NotApplicable), 2);
        assert_eq!(
            pop3(0, OpeningStock::Valued(50_000))[1],
            (
                "opening_paise".to_string(),
                "TB opening_paise sums to 0 paise; with opening stock of 50000 paise the \
                 difference in opening balances is 50000 paise"
                    .to_string()
            )
        );
        assert_eq!(
            pop3(30_000, unknown)[1],
            (
                "opening_paise".to_string(),
                "TB opening_paise sums to 30000 paise; opening stock unreadable".to_string()
            )
        );
    }
}
