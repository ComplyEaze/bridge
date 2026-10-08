//! Form 3CD clause 13 (and its clause 14 stock link): four questions the books cannot answer,
//! each raised as a finding with an answer figure that stays `not answered`, two of them pointing
//! at something in the books worth looking at first. A port of the reference Python
//! implementation's `questionnaire_cl13` test module, version 1; its contract is the spec pack in
//! `docs/tax-audit/spec-packs/questionnaire_cl13/`.
//!
//! The test reaches no conclusion: the year-end journal count is a hint, never evidence of a
//! method of accounting, and the stock pointer names the stock test's figure without reading it.

use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_primitives::TallyDate;

use crate::book::{Book, Voucher, VoucherStatus};
use crate::error::Result;
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::read::Window;
use crate::rules::Rules;
use crate::support::{self, py_repr_str, voucher_label};

pub const TEST_ID: &str = "questionnaire_cl13";
pub const VERSION: &str = "1";

/// The stock test's figure the closing-stock finding points at, when a stock result carrying it
/// is supplied. Only its presence is read, never its value (pack README §2.2).
pub const STOCK_FIGURE: &str = "stock.stock_in_hand_voucher_count";

/// Matched exactly, case included, anywhere in a ledger's chain (pack README §2.3, §10).
const EXPENSE_GROUPS: [&str; 2] = ["Direct Expenses", "Indirect Expenses"];
/// The base type a counted voucher has, compared exactly; its type name does not matter.
const JOURNAL: &str = "Journal";
const NOT_ANSWERED: &str = "not answered";
const COUNT: &str = "hint_accrual_journal_count";
const STOCK_FACT: &str = "hint_closing_stock_typed_in";

const COUNT_DEFINITION: &str =
    "Population Journal vouchers dated the last day of the books period, \
     with at least one line under a Direct Expenses or Indirect Expenses ledger -- a hint that \
     accrual entries were passed at year end, never proof of the method of accounting.";

/// Shared word for word by all four findings.
const LIMIT: &str = "The books alone cannot answer this question; any figure cited above is a \
     hint drawn from the books, never an answer, and is not a substitute for the client's own \
     confirmation.";

const POPULATION_NOTE: &str = "Books population (optional, cancelled and post-dated vouchers \
     excluded). Every question below is a placeholder for the CA's own answer; the books-derived \
     figures cited are hints, never answers.";

/// What a question's finding cites besides its own answer figure.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pointer {
    None,
    AccrualJournalCount,
    StockFigure,
}

struct Question {
    key: &'static str,
    /// The reference's own mapping, in the order the dump carries it (pack README §2.5).
    clauses: &'static [&'static str],
    title: &'static str,
    ask_client: [&'static str; 2],
    pointer: Pointer,
}

const QUESTIONS: [Question; 4] = [
    Question {
        key: "method_of_accounting",
        clauses: &["3CD-13(a)", "3CD-13(b)"],
        title: "State the method of accounting employed during the previous year (mercantile, \
                cash or hybrid) and confirm it matches the basis of accounting disclosed in the \
                financial statements.",
        ask_client: [
            "Signed confirmation from the client of the method of accounting employed.",
            "The financial statements' own accounting-policy note on the basis of accounting.",
        ],
        pointer: Pointer::AccrualJournalCount,
    },
    Question {
        key: "change_in_method",
        clauses: &["3CD-13(c)", "3CD-13(d)"],
        title: "State whether there has been any change in the method of accounting employed \
                compared with the immediately preceding previous year and, if so, state the \
                effect of the change on profit.",
        ask_client: [
            "The immediately preceding year's financial statements or tax audit report, for \
             comparison.",
            "The client's own note on any change in accounting method, with its computed effect \
             on profit.",
        ],
        pointer: Pointer::None,
    },
    Question {
        key: "icds_deviation",
        clauses: &["3CD-13(e)", "3CD-13(f)"],
        title: "State whether the profit computation complies with each notified Income \
                Computation and Disclosure Standard, and list any deviation together with its \
                effect on profit.",
        ask_client: [
            "The client's own ICDS-wise disclosure statement.",
            "A computation reconciling accounting profit to the ICDS-adjusted profit for each \
             deviation stated.",
        ],
        pointer: Pointer::None,
    },
    Question {
        key: "closing_stock_valuation",
        clauses: &["3CD-13(f)", "3CD-14(a)", "3CD-14(b)"],
        title: "State the method of valuation of closing stock used (see the linked Clause 14 \
                figures) and whether that method has changed from the method employed in the \
                immediately preceding previous year.",
        ask_client: [
            "A signed statement from the client of the stock valuation method (cost, net \
             realisable value, or the lower of cost and net realisable value, and the cost \
             formula used).",
            "Where the method changed, the client's own note on its effect on profit.",
        ],
        pointer: Pointer::StockFigure,
    },
];

fn answer_definition(key: &str) -> String {
    format!(
        "CA's recorded answer to the Clause 13 question {}. The books cannot supply this; it \
         stays unanswered until the CA's own review records it.",
        py_repr_str(key)
    )
}

/// Ledgers whose master's chain holds an expense group anywhere. A line on a ledger with no
/// master is never an expense line.
fn expense_ledgers(book: &Book) -> BTreeSet<&str> {
    book.ledgers
        .values()
        .filter(|l| EXPENSE_GROUPS.iter().any(|g| l.under(g)))
        .map(|l| l.name.as_str())
        .collect()
}

/// The three conditions of pack README §3, on one voucher.
fn is_year_end_expense_journal(
    v: &Voucher,
    last_day: &TallyDate,
    expense: &BTreeSet<&str>,
) -> bool {
    v.date == *last_day
        && v.base_type == JOURNAL
        && v.lines.iter().any(|l| expense.contains(l.ledger.as_str()))
}

/// Run the test. `stock` is the `stock` test's result on the same book, or `None` (only the edge
/// harness passes none); a result without [`STOCK_FIGURE`] is treated as none.
///
/// # Errors
///
/// `UnknownVoucherStatus` when the population cannot be formed.
pub fn run(
    book: &Book,
    rules: &Rules,
    period: &Window,
    stock: Option<&TestResult>,
) -> Result<TestResult> {
    let population = book.population()?;
    let expense = expense_ledgers(book);
    // One entry per GUID; the later of two counted vouchers sharing one gives the label.
    let mut counted: BTreeMap<&str, &Voucher> = BTreeMap::new();
    for v in population {
        if is_year_end_expense_journal(v, &period.to, &expense) {
            counted.insert(v.guid.as_str(), v);
        }
    }
    let refs: Vec<EvidenceRef> = counted
        .values()
        .map(|v| EvidenceRef::with_label("voucher", &v.guid, &voucher_label(v)))
        .collect();

    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let count_id = r.fig(
        COUNT,
        support::count(TEST_ID, counted.len())?,
        Unit::Count,
        COUNT_DEFINITION,
        refs.clone(),
    )?;
    let stock_supplied = stock.is_some_and(|s| s.figures.iter().any(|f| f.id == STOCK_FIGURE));
    for q in &QUESTIONS {
        let answer_id = r.fig(
            &format!("answer_{}", q.key),
            Value::Text(NOT_ANSWERED.to_string()),
            Unit::Text,
            &answer_definition(q.key),
            Vec::new(),
        )?;
        let mut facts = vec![("answer".to_string(), answer_id)];
        let mut evidence = Vec::new();
        match q.pointer {
            Pointer::AccrualJournalCount => {
                facts.push((COUNT.to_string(), count_id.clone()));
                evidence.clone_from(&refs);
            }
            Pointer::StockFigure if stock_supplied => {
                facts.push((STOCK_FACT.to_string(), STOCK_FIGURE.to_string()));
            }
            Pointer::StockFigure | Pointer::None => {}
        }
        r.findings.push(Finding {
            id: format!("{TEST_ID}/{}", q.key),
            clauses: q.clauses.iter().map(|c| (*c).to_string()).collect(),
            title: q.title.to_string(),
            facts,
            evidence,
            confidence: Confidence::JudgementRequired,
            limits: vec![LIMIT.to_string()],
            ask_client: q.ask_client.iter().map(|a| (*a).to_string()).collect(),
        });
    }
    r.population_note = POPULATION_NOTE.to_string();
    Ok(r)
}

/// QCL-1: the published count against a walk of its own, which counts every regular voucher
/// meeting the three conditions, a repeated GUID included (pack README §6). It finds the expense
/// ledgers and the regular vouchers again rather than reusing [`run`]'s.
///
/// # Errors
///
/// Only when the walk's count does not fit an `i64`.
pub fn check_invariants(book: &Book, period: &Window, result: &TestResult) -> Result<Vec<String>> {
    let id = format!("{TEST_ID}.{COUNT}");
    let Some(figure) = result.figures.iter().find(|f| f.id == id) else {
        return Ok(vec![format!("QCL-1: {id} missing from the result")]);
    };
    let expense: BTreeSet<&str> = book
        .ledgers
        .values()
        .filter(|l| {
            l.chain
                .iter()
                .any(|g| g == "Direct Expenses" || g == "Indirect Expenses")
        })
        .map(|l| l.name.as_str())
        .collect();
    let walked = book
        .vouchers
        .iter()
        .filter(|v| v.status == VoucherStatus::Regular)
        .filter(|v| v.date == period.to && v.base_type == "Journal")
        .filter(|v| v.lines.iter().any(|l| expense.contains(l.ledger.as_str())))
        .count();
    let walked = support::count(TEST_ID, walked)?;
    if figure.value == walked {
        return Ok(Vec::new());
    }
    let shown = |v: &Value| match v {
        Value::Int(n) => n.to_string(),
        Value::Text(t) => t.clone(),
        Value::Undefined => "None".to_string(),
    };
    Ok(vec![format!(
        "QCL-1: {id} = {} but an independent re-walk of the books population gives {}",
        shown(&figure.value),
        shown(&walked)
    )])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{Ledger, LedgerLine};
    use crate::error::AuditError;

    fn date(d: &str) -> TallyDate {
        TallyDate::parse(d).unwrap()
    }

    fn period() -> Window {
        Window {
            from: date("20250401"),
            to: date("20260331"),
        }
    }

    fn ledger(name: &str, chain: &[&str]) -> (String, Ledger) {
        let l = Ledger {
            name: name.to_string(),
            parent: chain[0].to_string(),
            chain: chain.iter().map(|g| (*g).to_string()).collect(),
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: String::new(),
            masterid: None,
        };
        (name.to_string(), l)
    }

    fn journal(guid: &str, number: &str, ledgers: &[&str]) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: date("20260331"),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            number: number.to_string(),
            status: VoucherStatus::Regular,
            lines: ledgers
                .iter()
                .map(|l| LedgerLine {
                    ledger: (*l).to_string(),
                    amount_paise: 0,
                })
                .collect(),
            ..Default::default()
        }
    }

    /// `Freight` sits under Indirect Expenses below a renamed primary, so the group is not last in
    /// its chain; `Ghost` has no master.
    fn book(vouchers: Vec<Voucher>) -> Book {
        Book {
            ledgers: [
                ledger("Freight", &["Indirect Expenses", "Renamed Primary"]),
                ledger("Bank", &["Bank Accounts"]),
            ]
            .into_iter()
            .collect(),
            vouchers,
            ..Default::default()
        }
    }

    fn finding<'a>(r: &'a TestResult, key: &str) -> &'a Finding {
        let id = format!("{TEST_ID}/{key}");
        r.findings.iter().find(|f| f.id == id).unwrap()
    }

    fn count_figure(r: &TestResult) -> (&Value, Vec<String>) {
        let f = r.figures.iter().find(|f| f.id.ends_with(COUNT)).unwrap();
        (&f.value, f.evidence.iter().map(EvidenceRef::key).collect())
    }

    #[test]
    fn an_expense_group_counts_anywhere_in_the_chain_and_a_ledger_with_no_master_never() {
        let b = book(vec![
            journal("g-1", "J1", &["Freight", "Bank"]),
            journal("g-2", "J2", &["Ghost", "Bank"]),
        ]);
        let r = run(&b, &Rules::vendored().unwrap(), &period(), None).unwrap();
        assert_eq!(
            count_figure(&r),
            (&Value::Int(1), vec!["voucher:g-1".to_string()])
        );
        assert_eq!(
            check_invariants(&b, &period(), &r).unwrap(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn the_stock_pointer_needs_a_stock_result_carrying_its_figure() {
        let b = book(Vec::new());
        let rules = Rules::vendored().unwrap();
        let answer = (
            "answer".to_string(),
            format!("{TEST_ID}.answer_closing_stock_valuation"),
        );
        let pointer = (STOCK_FACT.to_string(), STOCK_FIGURE.to_string());
        let mut stock = TestResult::new("stock", "1", &rules.version);
        let facts = |stock: Option<&TestResult>| {
            let r = run(&b, &rules, &period(), stock).unwrap();
            finding(&r, "closing_stock_valuation").facts.clone()
        };
        assert_eq!(facts(None), std::slice::from_ref(&answer));
        assert_eq!(facts(Some(&stock)), std::slice::from_ref(&answer));
        stock
            .fig(
                "stock_in_hand_voucher_count",
                Value::Int(0),
                Unit::Count,
                "",
                Vec::new(),
            )
            .unwrap();
        assert_eq!(facts(Some(&stock)), [answer, pointer]);
    }

    #[test]
    fn a_voucher_of_unknown_status_refuses_before_any_figure() {
        let mut v = journal("g-1", "J1", &["Freight", "Bank"]);
        v.status = VoucherStatus::Unknown;
        let refused = run(&book(vec![v]), &Rules::vendored().unwrap(), &period(), None);
        assert!(matches!(refused, Err(AuditError::UnknownVoucherStatus(1))));
    }

    #[test]
    fn qcl_1_names_a_missing_count_and_shows_a_non_integer_one_as_python_prints_it() {
        let b = book(vec![journal("g-1", "J1", &["Freight", "Bank"])]);
        let mut r = TestResult::new(TEST_ID, VERSION, "v");
        assert_eq!(
            check_invariants(&b, &period(), &r).unwrap(),
            ["QCL-1: questionnaire_cl13.hint_accrual_journal_count missing from the result"]
        );
        r.fig(COUNT, Value::Undefined, Unit::Count, "", Vec::new())
            .unwrap();
        assert_eq!(
            check_invariants(&b, &period(), &r).unwrap(),
            [
                "QCL-1: questionnaire_cl13.hint_accrual_journal_count = None but an independent \
                 re-walk of the books population gives 1"
            ]
        );
    }
}
