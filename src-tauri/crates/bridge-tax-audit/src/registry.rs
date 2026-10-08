// SPDX-License-Identifier: Apache-2.0
//! Every ported test, one entry each, sorted by test id: how to run it on a book, and the
//! least number of figures a parity comparison of it must see. Porting a test adds one entry
//! here and one runner in `parity/python_golden.py`'s `RUNNERS`; `examples/local_parity`,
//! `compare::default_min_figures` and the registry-wide CI parity (`tests/registry.rs`) read
//! this table instead of listing tests themselves.

use std::collections::BTreeSet;

use crate::applicability_44ab::{ComparisonTurnover, TurnoverInputs};
use crate::book::Book;
use crate::books_examined::DocumentRead;
use crate::error::{AuditError, Result};
use crate::financial_statements::ReportTotals;
use crate::rules::Rules;
use crate::Engagement;

/// Data a test takes from its caller rather than from the book: Tally's own Profit & Loss report
/// totals (`financial_statements`), the comparison turnover (`applicability_44ab`) and the documents
/// loaded with the read (`books_examined`). Empty is "none supplied", which each of those tests
/// handles itself.
#[derive(Debug, Clone, Default)]
pub struct CallerData {
    pub report_totals: Option<ReportTotals>,
    pub turnover_inputs: TurnoverInputs,
    /// The assessee's Form 26AS/AIS/TIS rows (`tds_tcs_26as`, `twentysixas_receipts`; the AIS rows
    /// also `high_value_register`).
    pub traces: crate::documents::TracesDocuments,
    /// A bank statement (`bank_reconciliation`, which refuses without one; `high_value_register`,
    /// whose s.194N section reports none supplied).
    pub bank_statement: Option<crate::documents::BankStatementDoc>,
    /// The reader's plain-words reason a supplied statement was refused (`bank_reconciliation`
    /// gives its `refused` result; `high_value_register`'s s.194N coverage says so); `None` when
    /// none was supplied or it was read.
    pub bank_statement_refused: Option<String>,
    /// The documents loaded with the read (`books_examined`).
    pub documents_read: BTreeSet<DocumentRead>,
}

/// One ported test.
pub struct PortedTest {
    pub id: &'static str,
    /// The fewest figures a parity comparison of this test accepts (see `compare`).
    pub min_figures: usize,
    /// Run the test on an already-loaded book and return its canonical parity dump.
    pub run_on: fn(&Engagement, &Book, &Rules, &CallerData) -> Result<serde_json::Value>,
}

pub const PORTED: &[PortedTest] = &[
    PortedTest {
        id: "applicability_44ab",
        min_figures: 9,
        run_on: |e, b, r, c| crate::applicability_44ab_on(e, b, r, &c.turnover_inputs),
    },
    PortedTest {
        id: "bank_reconciliation",
        min_figures: 40,
        run_on: |e, b, r, c| {
            crate::bank_reconciliation_on(
                e,
                b,
                r,
                c.bank_statement.as_ref(),
                c.bank_statement_refused.as_deref(),
            )
        },
    },
    PortedTest {
        id: "book_keeping_quality",
        min_figures: 12,
        run_on: |e, b, r, _| crate::book_keeping_quality_on(e, b, r),
    },
    PortedTest {
        id: "books_examined",
        // `books_maintained` and `books_examined`, on any book.
        min_figures: 2,
        run_on: |_, b, r, c| crate::books_examined_on(b, r, &c.documents_read),
    },
    PortedTest {
        id: "cash_44ab",
        min_figures: 7,
        run_on: |e, b, r, _| crate::cash_44ab_on(e, b, r),
    },
    PortedTest {
        id: "cash_book_integrity",
        min_figures: 1,
        run_on: |e, b, r, _| crate::cash_book_integrity_on(e, b, r),
    },
    PortedTest {
        id: "cash_payments_40a3",
        min_figures: 18,
        run_on: |e, b, r, _| crate::cash_payments_40a3_on(e, b, r),
    },
    PortedTest {
        id: "clause21a_candidates",
        // Nothing structural: two figures per item with a candidate.
        min_figures: 2,
        run_on: |e, b, r, _| crate::clause21a_candidates_on(e, b, r),
    },
    PortedTest {
        id: "counter_cheques_40a3",
        // `configured_terms_count`, the six totals and counts, and one total per excluded role:
        // twelve with no term configured, as the reference's own pack runs a client without one.
        min_figures: 12,
        run_on: |e, b, r, _| crate::counter_cheques_40a3_on(e, b, r),
    },
    PortedTest {
        id: "creditor_ageing_43bh",
        min_figures: 12,
        run_on: |e, b, r, _| crate::creditor_ageing_43bh_on(e, b, r),
    },
    PortedTest {
        id: "depreciation",
        min_figures: 2,
        run_on: |e, b, r, _| crate::depreciation_on(e, b, r),
    },
    PortedTest {
        id: "entity_269st_gap",
        min_figures: 4,
        run_on: |e, b, r, _| crate::entity_269st_gap_on(e, b, r),
    },
    PortedTest {
        id: "financial_statements",
        min_figures: 18,
        run_on: |e, b, r, c| crate::financial_statements_on(e, b, r, c.report_totals.as_ref()),
    },
    PortedTest {
        id: "high_value_register",
        // 38 on any book: no row figures, no statement and an unknown recipient type.
        min_figures: 38,
        run_on: |e, b, r, c| {
            crate::high_value_register_on(
                e,
                b,
                r,
                c.bank_statement.as_ref(),
                c.bank_statement_refused.as_deref(),
                &c.traces.ais,
            )
        },
    },
    PortedTest {
        id: "knock_off_candidates",
        // The seven fixed figures, on every book.
        min_figures: 7,
        run_on: |e, b, r, _| crate::knock_off_candidates_on(e, b, r),
    },
    PortedTest {
        id: "ledger_scrutiny",
        min_figures: 1,
        run_on: |e, b, r, _| crate::ledger_scrutiny_on(e, b, r),
    },
    PortedTest {
        id: "loans_interest",
        // Five figures on any book, six more per configured loan: fewer than eleven is a vacuous run.
        min_figures: 11,
        run_on: |e, b, r, _| crate::loans_interest_on(e, b, r),
    },
    PortedTest {
        id: "partners_40b_194t",
        // `applicable` alone, on any book that is not a firm's or an LLP's.
        min_figures: 1,
        run_on: |e, b, r, _| crate::partners_40b_194t_on(e, b, r),
    },
    PortedTest {
        id: "party_monthly",
        // 16 with all four groups and no voucher (edge book pm_empty): per block the total row's
        // year and voucher count, and the Trial Balance movement and difference.
        min_figures: 16,
        run_on: |e, b, r, _| crate::party_monthly_on(e, b, r),
    },
    PortedTest {
        id: "questionnaire_cl13",
        // The accrual-journal count and the four answer figures, on every book.
        min_figures: 5,
        run_on: |e, b, r, _| crate::questionnaire_cl13_on(e, b, r),
    },
    PortedTest {
        id: "read_scope",
        min_figures: 1,
        run_on: |_, b, r, _| crate::read_scope_on(b, r),
    },
    PortedTest {
        id: "related_parties_cl23",
        // `applicable` alone when no person is confirmed, as on the synthetic read.
        min_figures: 1,
        run_on: |e, b, r, _| crate::related_parties_cl23_on(e, b, r),
    },
    PortedTest {
        id: "specified_persons_40a2b",
        // `applicable` alone when no person is confirmed, as on the synthetic read.
        min_figures: 1,
        run_on: |e, b, r, _| crate::specified_persons_40a2b_on(e, b, r),
    },
    PortedTest {
        id: "stale_balances_41_1",
        min_figures: 1,
        run_on: |e, b, r, _| crate::stale_balances_41_1_on(e, b, r),
    },
    PortedTest {
        id: "statutory_dues_43b",
        min_figures: 4,
        run_on: |e, b, r, _| crate::statutory_dues_43b_on(e, b, r),
    },
    PortedTest {
        id: "stock",
        // 30 on any book; one more when a Stock-in-Hand ledger has no Trial Balance row.
        min_figures: 30,
        run_on: |e, b, r, _| crate::stock_on(e, b, r),
    },
    PortedTest {
        id: "tds_payees",
        min_figures: 30,
        run_on: |e, b, r, _| crate::tds_payees_on(e, b, r),
    },
    PortedTest {
        id: "tds_tcs_26as",
        min_figures: 30,
        run_on: |e, b, r, c| crate::tds_tcs_26as_on(e, b, r, &c.traces),
    },
    PortedTest {
        id: "trial_balance",
        min_figures: 1,
        run_on: |e, b, r, _| crate::trial_balance_on(e, b, r),
    },
    PortedTest {
        id: "twentysixas_receipts",
        min_figures: 1,
        run_on: |e, b, r, c| crate::twentysixas_receipts_on(e, b, r, &c.traces),
    },
];

/// The registry entry for `id`, if that test is ported.
pub fn find(id: &str) -> Option<&'static PortedTest> {
    PORTED.iter().find(|t| t.id == id)
}

/// Read, verify and build the book, then run test `id` on it: its canonical parity dump.
pub fn run_canonical(
    id: &str,
    engagement: &Engagement,
    rules: &Rules,
    caller: &CallerData,
) -> Result<serde_json::Value> {
    let test = find(id).ok_or_else(|| AuditError::Config(format!("{id} is not a ported test")))?;
    (test.run_on)(engagement, &crate::load_book(engagement)?, rules, caller)
}

/// `financial_statements`' report totals from the JSON `parity/python_golden.py
/// --emit-report-totals` writes.
pub fn report_totals_from_json(v: &serde_json::Value) -> Result<ReportTotals> {
    let net_profit_paise = v["net_profit_paise"].as_i64().ok_or_else(|| {
        AuditError::Config("report totals: net_profit_paise is not an integer".to_string())
    })?;
    let closing_stock_paise = match &v["closing_stock_paise"] {
        serde_json::Value::Null => None,
        x => Some(x.as_i64().ok_or_else(|| {
            AuditError::Config("report totals: closing_stock_paise is not an integer".to_string())
        })?),
    };
    let source = match &v["source"] {
        serde_json::Value::Null => None,
        x => Some(
            x.as_str()
                .ok_or_else(|| {
                    AuditError::Config("report totals: source is not a string".to_string())
                })?
                .to_string(),
        ),
    };
    Ok(ReportTotals {
        net_profit_paise,
        closing_stock_paise,
        source,
    })
}

/// `applicability_44ab`'s comparison turnover from the JSON `parity/python_golden.py
/// --emit-turnover-inputs` writes. The books turnover is the test's own, never the caller's.
pub fn turnover_inputs_from_json(v: &serde_json::Value) -> Result<TurnoverInputs> {
    let source = |key: &str| -> Result<Option<ComparisonTurnover>> {
        let s = &v[key];
        if s.is_null() {
            return Ok(None);
        }
        match (s["turnover_paise"].as_i64(), s["coverage"].as_str()) {
            (Some(turnover_paise), Some(coverage)) => Ok(Some(ComparisonTurnover {
                turnover_paise,
                coverage: coverage.to_string(),
            })),
            _ => Err(AuditError::Config(format!(
                "turnover inputs: {key} needs turnover_paise and coverage"
            ))),
        }
    };
    Ok(TurnoverInputs {
        books_turnover_paise: None,
        gstr1: source("gstr1")?,
        gstr3b: source("gstr3b")?,
        ais: source("ais")?,
    })
}

/// `books_examined`' documents from the JSON list `parity/python_golden.py --documents-read` reads:
/// each a name the reference's pack gives, once, in the pack's order. Anything else is refused
/// rather than read differently from the reference, which prints the names as given.
pub fn documents_read_from_json(v: &serde_json::Value) -> Result<BTreeSet<DocumentRead>> {
    let names = v
        .as_array()
        .ok_or_else(|| AuditError::Config("documents read: not a list".to_string()))?;
    let mut out = BTreeSet::new();
    for name in names {
        let d = name.as_str().and_then(DocumentRead::parse).ok_or_else(|| {
            AuditError::Config(format!(
                "documents read: {name} is not a document the pack loads"
            ))
        })?;
        if out.last().is_some_and(|last| *last >= d) {
            return Err(AuditError::Config(format!(
                "documents read: {name} is repeated or out of the pack's order"
            )));
        }
        out.insert(d);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::PORTED;

    #[test]
    fn find_is_by_exact_id() {
        assert!(super::find("trial_balance").is_some());
        assert!(super::find("trial").is_none());
        assert!(super::find("cash").is_none());
        assert!(super::find("trial_balance ").is_none());
    }

    #[test]
    fn caller_data_is_read_whole_and_refused_when_mistyped() {
        use super::{report_totals_from_json, turnover_inputs_from_json};
        use serde_json::json;
        let t = turnover_inputs_from_json(&json!({
            "gstr1": {"turnover_paise": 5, "coverage": "full"},
            "gstr3b": null,
            "ais": {"turnover_paise": 7, "coverage": "partial"}
        }))
        .unwrap();
        assert_eq!(t.gstr1.unwrap().turnover_paise, 5);
        assert!(t.gstr3b.is_none());
        let ais = t.ais.unwrap();
        assert_eq!((ais.turnover_paise, ais.coverage.as_str()), (7, "partial"));
        for bad in [
            json!({"gstr1": {"turnover_paise": "5", "coverage": "full"}}),
            json!({"ais": {"coverage": "full"}}),
        ] {
            assert!(turnover_inputs_from_json(&bad).is_err(), "{bad}");
        }
        let r = report_totals_from_json(
            &json!({"net_profit_paise": 3, "closing_stock_paise": null, "source": null}),
        )
        .unwrap();
        assert_eq!(
            (r.net_profit_paise, r.closing_stock_paise, r.source),
            (3, None, None)
        );
        for bad in [
            json!({}),
            json!({"net_profit_paise": "3"}),
            json!({"net_profit_paise": 3, "closing_stock_paise": "4"}),
            json!({"net_profit_paise": 3, "source": 9}),
        ] {
            assert!(report_totals_from_json(&bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn documents_read_are_the_packs_names_once_in_its_order() {
        use super::documents_read_from_json;
        use crate::books_examined::DocumentRead;
        use crate::error::AuditError;
        use serde_json::json;
        assert!(documents_read_from_json(&json!([])).unwrap().is_empty());
        let read =
            documents_read_from_json(&json!(["GSTR-2B", "Form 26AS", "Draft Form 3CD"])).unwrap();
        assert_eq!(
            read.into_iter().collect::<Vec<_>>(),
            [
                DocumentRead::Gstr2b,
                DocumentRead::Form26as,
                DocumentRead::DraftForm3cd
            ]
        );
        for bad in [
            json!("GSTR-2B"),
            json!(["GSTR-2B", 3]),
            json!(["GSTR-2B", "GSTR-9"]),
            json!(["GSTR-2B", "GSTR-2B"]),
            json!(["Form 26AS", "GSTR-1"]),
        ] {
            assert!(
                matches!(documents_read_from_json(&bad), Err(AuditError::Config(_))),
                "{bad}"
            );
        }
    }

    #[test]
    fn the_registry_is_sorted_and_unique() {
        let ids: Vec<&str> = PORTED.iter().map(|t| t.id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(ids, sorted);
    }
}
