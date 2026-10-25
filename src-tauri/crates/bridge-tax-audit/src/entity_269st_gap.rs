// SPDX-License-Identifier: Apache-2.0
//! Port of the reference engine's `entity_269st_gap`: s.269ST(a) binds cash received "from a
//! person" in a day, and `cash_payments_40a3` keys that population by party LEDGER, so one person
//! holding two ledgers who pays 1.5 lakh into each on one day shows two under-limit rows and no
//! finding. This test does not re-key the population. It takes the same one
//! (`high_value_register::cash_269st_rows`), re-aggregates it by the PAN each ledger carries and
//! reports only the residue: an entity whose same-day total reaches the limit while no single
//! constituent ledger did. Every finding is judgement-required: the books show that ledgers share
//! one PAN, only the CA can confirm they are one person.
//!
//! A figure id carries an 8-hex tag of the entity's PAN (`entity_day_total_<day>_<tag>`), as every
//! other per-party id does (`support::hash8` of the PAN as written, no normalisation), so the PAN
//! never prints in an id or in an error that quotes one (bridge#1145 item 9). The PAN itself
//! travels once, as the finding's `pan` evidence ref, as the reference's does.

use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_primitives::TallyDate;

use crate::book::{Book, Voucher};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::high_value_register::{cash_269st_rows, Direction};
use crate::party_identity::{EntityBinding, PanSource, PartyIndex};
use crate::read::iso;
use crate::rules::Rules;
use crate::support::{count, hash8};

pub const TEST_ID: &str = "entity_269st_gap";
pub const VERSION: &str = "1.0.0";

/// The reference hard-codes this limit in its invariant (equal to the rules table's number).
const GAP_1_LIMIT_PAISE: i64 = 20_000_000;

fn overflow() -> AuditError {
    AuditError::Config("entity_269st_gap: a total overflowed i64 paise".to_string())
}

/// One (day, PAN): the cash received by every ledger that carries it.
struct Slot<'a> {
    paise: i64,
    vouchers: BTreeMap<String, &'a Voucher>,
    ledgers: BTreeMap<String, i64>,
    binding: &'a EntityBinding,
}

fn voucher_evidence(vouchers: &BTreeMap<String, &Voucher>) -> Vec<EvidenceRef> {
    vouchers
        .keys()
        .map(|g| EvidenceRef::new("voucher", g))
        .collect()
}

pub fn run(
    book: &Book,
    rules: &Rules,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
    index: &PartyIndex,
    round_off_ledgers: &BTreeSet<String>,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let limit = rules.s269st_limit_per_person_per_day_paise;
    let pop = book.population()?;
    let rows = cash_269st_rows(
        &pop,
        book,
        cash,
        bank,
        Direction::Receipt,
        round_off_ledgers,
    )?;

    // (day, entity PAN) -> constituent ledger rows, for ledgers that bind to an entity at all.
    let mut by_entity: BTreeMap<(TallyDate, &str), Slot> = BTreeMap::new();
    let mut unbound_ledgers: BTreeSet<&str> = BTreeSet::new();
    for ((day, ledger_name), data) in &rows {
        let Some(binding) = index.entity_for_ledger(ledger_name) else {
            unbound_ledgers.insert(ledger_name);
            continue;
        };
        let slot = by_entity
            .entry((day.clone(), binding.pan.as_str()))
            .or_insert_with(|| Slot {
                paise: 0,
                vouchers: BTreeMap::new(),
                ledgers: BTreeMap::new(),
                binding,
            });
        slot.paise = slot.paise.checked_add(data.paise).ok_or_else(overflow)?;
        // One unlabelled ref per distinct GUID: a row's vouchers are keyed by GUID and label (#1195),
        // and a ref with no label cannot tell two vouchers sharing a GUID apart.
        for ((g, _), v) in &data.vouchers {
            slot.vouchers.insert(g.clone(), v);
        }
        slot.ledgers.insert(ledger_name.clone(), data.paise);
    }

    let multi_ledger_days = by_entity.values().filter(|s| s.ledgers.len() > 1).count();
    let (mut gap_total, mut gap_rows) = (0_i64, 0_usize);
    for ((day, pan), slot) in &by_entity {
        if slot.paise < limit {
            continue; // not at the limit even aggregated
        }
        // Already reported by cash_payments_40a3, never twice. A single ledger at the limit is
        // such a row, so this also leaves out an entity of one ledger.
        if slot.ledgers.values().any(|amt| *amt >= limit) {
            continue;
        }
        let binding = slot.binding;
        gap_total = gap_total.checked_add(slot.paise).ok_or_else(overflow)?;
        gap_rows += 1;
        let day_text = iso(day);
        let rid = format!("{day_text}_{}", hash8(pan));
        let ledger_count = slot.ledgers.len();
        let f_amt = r.fig(
            &format!("entity_day_total_{rid}"),
            Value::Int(slot.paise),
            Unit::Paise,
            &format!(
                "Cash received on {day_text} across the {ledger_count} ledgers that carry one PAN, \
summed over the same population cash_payments_40a3 tests per ledger."
            ),
            voucher_evidence(&slot.vouchers),
        )?;
        r.fig(
            &format!("entity_ledger_count_{rid}"),
            count(TEST_ID, ledger_count)?,
            Unit::Count,
            &format!("Ledgers carrying this PAN that received cash on {day_text}."),
            Vec::new(),
        )?;
        r.fig(
            &format!("entity_largest_single_ledger_{rid}"),
            Value::Int(slot.ledgers.values().copied().max().unwrap_or(0)),
            Unit::Paise,
            &format!(
                "Largest single-ledger receipt on {day_text} for this PAN -- below the limit, \
which is why the per-ledger test reports nothing."
            ),
            Vec::new(),
        )?;
        let mut limits = vec![
            "The books show these ledgers share one PAN; whether they are one person is the CA's \
determination. A shared PAN with unrelated ledger names is either a mis-entered GSTIN or one \
proprietor trading under two names."
                .to_string(),
        ];
        if binding.pan_sources.contains(&PanSource::DerivedFromGstin) {
            limits.push(
                "At least one PAN here is derived from the ledger's GSTIN, not recorded on the \
ledger master."
                    .to_string(),
            );
        }
        if !binding.names_agree {
            limits.push(
                "The ledger names for this PAN share no common word -- confirm before treating \
them as one person."
                    .to_string(),
            );
        }
        let mut evidence = voucher_evidence(&slot.vouchers);
        evidence.extend(slot.ledgers.keys().map(|l| EvidenceRef::new("ledger", l)));
        evidence.push(EvidenceRef::new("pan", &binding.pan));
        r.findings.push(Finding {
            id: format!("{TEST_ID}/{rid}"),
            clauses: vec!["s.269ST(a)".to_string(), "3CD-31(ba)".to_string()],
            title: format!(
                "Cash received from ledgers sharing one PAN reaches the s.269ST(a) limit on \
{day_text} only when aggregated -- no single ledger does"
            ),
            facts: vec![("amount".to_string(), f_amt)],
            evidence,
            confidence: Confidence::JudgementRequired,
            limits,
            ask_client: vec![
                "Confirm whether these ledgers are one person for s.269ST.".to_string()
            ],
        });
    }

    r.fig(
        "gap_rows_count",
        count(TEST_ID, gap_rows)?,
        Unit::Count,
        "Person-days where the aggregate reaches the s.269ST(a) limit but no single ledger does, \
and which the per-ledger test therefore does not report.",
        Vec::new(),
    )?;
    r.fig(
        "gap_amount_total",
        Value::Int(gap_total),
        Unit::Paise,
        "Sum of those person-day aggregates. Not a contravention total: each rests on the CA \
confirming the ledgers are one person.",
        Vec::new(),
    )?;
    r.fig(
        "multi_ledger_person_days_count",
        count(TEST_ID, multi_ledger_days)?,
        Unit::Count,
        "Person-days on which MORE THAN ONE ledger carrying the same PAN received cash. This is \
the population the gap can exist in at all: a zero gap with a zero here means the coincidence never \
arose, NOT that per-ledger keying is safe.",
        Vec::new(),
    )?;
    r.fig(
        "unbound_ledger_count",
        count(TEST_ID, unbound_ledgers.len())?,
        Unit::Count,
        "Ledgers in the cash-receipt population with no PAN to bind on, so they cannot be \
aggregated with any other ledger here at all.",
        Vec::new(),
    )?;
    // The reference's own text, compared byte for byte by the goldens: it still names the function the
    // walk had there.
    r.population_note =
        "Population is cash_payments_40a3.compute_269st_rows, unchanged; this test \
only re-aggregates it by PAN and reports the residue no single ledger already triggers."
            .to_string();
    Ok(r)
}

/// No row reported here may also be reported per ledger: the largest single-ledger amount on every
/// gap row must be strictly below the limit (else `cash_payments_40a3` already has it).
pub fn check_invariants(result: &TestResult) -> Vec<String> {
    result
        .figures
        .iter()
        .filter(|f| f.id.contains(".entity_largest_single_ledger_"))
        .filter_map(|f| match f.value {
            Value::Int(v) if v >= GAP_1_LIMIT_PAISE => Some(format!(
                "GAP-1 {}: a single ledger at {v} already reaches the limit, so this row double \
counts cash_payments_40a3",
                f.id
            )),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::findings::Unit;

    /// GAP-1: a gap row whose largest single-ledger amount reaches the limit double counts
    /// `cash_payments_40a3`; one under it does not, and a figure that is not such a row is ignored.
    #[test]
    fn gap_1_fires_on_a_single_ledger_at_the_limit_and_not_under_it() {
        let result_with = |figures: &[(&str, i64)]| {
            let mut r = TestResult::new(TEST_ID, VERSION, "invented");
            for (id, v) in figures {
                r.fig(id, Value::Int(*v), Unit::Paise, "invented", Vec::new())
                    .unwrap();
            }
            r
        };
        let day = "entity_largest_single_ledger_2025-06-01_4d8a6b4a";
        assert!(check_invariants(&result_with(&[(day, GAP_1_LIMIT_PAISE - 1)])).is_empty());
        let fired = check_invariants(&result_with(&[
            (day, GAP_1_LIMIT_PAISE),
            (
                "entity_day_total_2025-06-01_4d8a6b4a",
                GAP_1_LIMIT_PAISE * 3,
            ),
        ]));
        assert_eq!(fired.len(), 1);
        assert!(fired[0].starts_with("GAP-1 entity_269st_gap.entity_largest_single_ledger_"));
    }

    /// Characterises the population this test re-aggregates, the receipt leg of the s.269ST walk
    /// (`high_value_register::cash_269st_rows`), through the production entry: a Sales line stays
    /// in a lone party's total, a round-off line (as bound) does too and is never a party, and a
    /// Contra is no row. Two ledgers of one PAN, Rs 1.5 lakh each, are a gap.
    #[test]
    fn the_entity_total_is_the_walks_rows_summed_by_pan() {
        use crate::book::{Ledger, LedgerLine, VoucherStatus};
        use crate::party_identity::{build_party_index, PartyConfig};
        let ledger = |name: &str, group: &str, pan: &str| Ledger {
            name: name.to_string(),
            parent: group.to_string(),
            chain: vec![group.to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            guid: format!("invented-{name}"),
            masterid: None,
            pan: pan.to_string(),
            gstin: String::new(),
        };
        let voucher = |guid: &str, base_type: &str, lines: &[(&str, i64)]| Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse("20250601").unwrap(),
            base_type: base_type.to_string(),
            status: VoucherStatus::Regular,
            lines: lines
                .iter()
                .map(|(ledger, amount_paise)| LedgerLine {
                    ledger: (*ledger).to_string(),
                    amount_paise: *amount_paise,
                })
                .collect(),
            ..Default::default()
        };
        let book = Book {
            ledgers: [
                ledger("Alpha A", "Sundry Debtors", "PAN-ONE"),
                ledger("Alpha B", "Sundry Debtors", "PAN-ONE"),
                ledger("Sales", "Sales Accounts", ""),
                ledger("Round Off", "Indirect Expenses", ""),
                ledger("Loan Y", "Loans (Liability)", ""),
            ]
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect(),
            vouchers: vec![
                voucher(
                    "e1",
                    "Receipt",
                    &[
                        ("Cash", 15_000_000),
                        ("Alpha A", -10_000_000),
                        ("Sales", -5_000_000),
                    ],
                ),
                voucher(
                    "e2",
                    "Receipt",
                    &[
                        ("Cash", 15_000_100),
                        ("Alpha B", -15_000_000),
                        ("Round Off", -100),
                    ],
                ),
                // A loan line is a ledger with no PAN, and a voucher naming no party is the
                // unidentified bucket: both are unbound, neither is left out.
                voucher(
                    "e4",
                    "Receipt",
                    &[("Cash", 1_000_000), ("Loan Y", -1_000_000)],
                ),
                voucher(
                    "e5",
                    "Receipt",
                    &[("Cash", 2_000_000), ("Sales", -2_000_000)],
                ),
                voucher(
                    "e3",
                    "Contra",
                    &[("Cash", 30_000_000), ("Alpha A", -30_000_000)],
                ),
            ],
            ..Default::default()
        };
        let set = |names: &[&str]| -> BTreeSet<String> {
            names.iter().map(|n| (*n).to_string()).collect()
        };
        let index = build_party_index(&book, &PartyConfig::default()).unwrap();
        let r = run(
            &book,
            &Rules::vendored().unwrap(),
            &set(&["Cash"]),
            &set(&["Bank"]),
            &index,
            &set(&["Round Off"]),
        )
        .unwrap();
        let rid = format!("2025-06-01_{}", hash8("PAN-ONE"));
        let value = |name: &str| {
            let id = format!("{TEST_ID}.{name}");
            match r.figures.iter().find(|f| f.id == id).unwrap().value {
                Value::Int(v) => v,
                _ => panic!("not an integer"),
            }
        };
        assert_eq!(value(&format!("entity_day_total_{rid}")), 30_000_100);
        assert_eq!(value(&format!("entity_ledger_count_{rid}")), 2);
        assert_eq!(
            value(&format!("entity_largest_single_ledger_{rid}")),
            15_000_100
        );
        assert_eq!(value("gap_rows_count"), 1);
        assert_eq!(value("unbound_ledger_count"), 2);
        assert_eq!(r.findings.len(), 1);
        let cited: Vec<(&str, &str)> = r.findings[0]
            .evidence
            .iter()
            .map(|e| (e.kind.as_str(), e.id.as_str()))
            .collect();
        assert_eq!(
            cited,
            [
                ("voucher", "e1"),
                ("voucher", "e2"),
                ("ledger", "Alpha A"),
                ("ledger", "Alpha B"),
                ("pan", "PAN-ONE"),
            ]
        );
    }
}
