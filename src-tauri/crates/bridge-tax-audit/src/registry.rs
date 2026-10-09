// SPDX-License-Identifier: Apache-2.0
//! Every ported test, one entry each, sorted by test id: how to run it on a book, and the
//! least number of figures a parity comparison of it must see. Porting a test adds one entry
//! here and one runner in `parity/python_golden.py`'s `RUNNERS`; `examples/local_parity`,
//! `compare::default_min_figures` and the registry-wide CI parity (`tests/registry.rs`) read
//! this table instead of listing tests themselves.

use std::collections::BTreeSet;

use crate::applicability_44ab::{ComparisonTurnover, TurnoverInputs};
use crate::book::Book;
use crate::books_examined::{CallerNamedDocument, DocumentRead};
use crate::error::{AuditError, Result};
use crate::financial_statements::ReportTotals;
use crate::rules::Rules;
use crate::Engagement;

/// Data a test takes from its caller rather than from the book: Tally's own Profit & Loss report
/// totals (`financial_statements`), the comparison turnover (`applicability_44ab`) and the documents
/// loaded with the read (`books_examined`, through [`CallerData::documents_read`]). Empty is "none
/// supplied", which each of those tests handles itself.
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
    /// The documents loaded with the read whose data no field above holds (`books_examined`).
    pub named_documents: BTreeSet<CallerNamedDocument>,
}

impl CallerData {
    /// The documents loaded with the read, for `books_examined`, as the reference's pack lists them:
    /// each document whose file was loaded, rows or none, in the pack's order (#1281). Form 26AS, AIS
    /// and TIS when `traces` holds them, GSTR-1 when its comparison turnover is held (the pack takes
    /// both from the one file), the bank statement when it was read (a refused one is not), and the
    /// caller's `named_documents`. Refuses a bank statement both held and refused.
    pub fn documents_read(&self) -> Result<BTreeSet<DocumentRead>> {
        if self.bank_statement.is_some() && self.bank_statement_refused.is_some() {
            return Err(AuditError::Config(format!(
                "{}: a bank statement was supplied and also refused",
                crate::books_examined::TEST_ID
            )));
        }
        let held = [
            (DocumentRead::Gstr1, self.turnover_inputs.gstr1.is_some()),
            (DocumentRead::Form26as, self.traces.form26as.is_some()),
            (DocumentRead::Ais, self.traces.ais.is_some()),
            (DocumentRead::Tis, self.traces.tis.is_some()),
            (DocumentRead::BankStatement, self.bank_statement.is_some()),
        ];
        Ok(held
            .into_iter()
            .filter_map(|(d, loaded)| loaded.then_some(d))
            .chain(self.named_documents.iter().map(|d| d.document()))
            .collect())
    }
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
        run_on: |_, b, r, c| crate::books_examined_on(b, r, &c.documents_read()?),
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
        id: "clause44",
        // Every column's and reason's count and amount, the total, and the three overlays' pairs:
        // 27 on any book.
        min_figures: 27,
        run_on: |e, b, r, _| crate::clause44_on(e, b, r),
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
                c.traces.ais_rows(),
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
        id: "narration_payees",
        // The fifteen fixed figures, on every book.
        min_figures: 15,
        run_on: |e, b, r, _| crate::narration_payees_on(e, b, r),
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

    /// A bank statement whose contents `documents_read` never looks at.
    fn statement() -> crate::documents::BankStatementDoc {
        let day = bridge_tally_primitives::TallyDate::parse("20250401").unwrap();
        crate::documents::BankStatementDoc {
            doc_id: "bank:x".to_string(),
            source_sha256: String::new(),
            account_ref: "XXXX1".to_string(),
            bank: "Invented Bank".to_string(),
            start: day.clone(),
            end: day,
            opening_balance_paise: 0,
            closing_balance_paise: 0,
            rows: Vec::new(),
        }
    }

    #[test]
    fn a_refused_bank_statement_is_never_listed() {
        use super::CallerData;
        use crate::books_examined::DocumentRead;
        let refused = CallerData {
            bank_statement_refused: Some("no closing balance".to_string()),
            ..Default::default()
        };
        assert!(refused.documents_read().unwrap().is_empty());
        let read = CallerData {
            bank_statement: Some(statement()),
            ..Default::default()
        };
        assert_eq!(
            read.documents_read()
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>(),
            [DocumentRead::BankStatement]
        );
    }

    #[test]
    fn a_statement_both_held_and_refused_is_refused() {
        use super::CallerData;
        use crate::error::AuditError;
        let both = CallerData {
            bank_statement: Some(statement()),
            bank_statement_refused: Some("no closing balance".to_string()),
            ..Default::default()
        };
        assert!(matches!(both.documents_read(), Err(AuditError::Config(_))));
    }

    #[test]
    fn each_document_is_listed_exactly_when_caller_data_holds_its_file() {
        use super::CallerData;
        use crate::applicability_44ab::ComparisonTurnover;
        use crate::books_examined::{CallerNamedDocument as N, DocumentRead as D};
        let turnover = || {
            Some(ComparisonTurnover {
                turnover_paise: 1,
                coverage: "full".to_string(),
            })
        };
        let listed = |c: &CallerData| c.documents_read().unwrap().into_iter().collect::<Vec<_>>();
        assert!(listed(&CallerData::default()).is_empty());
        // Each derived document alone: a TRACES file loaded with no rows is listed.
        let mut c = CallerData::default();
        c.traces.form26as = Some(Vec::new());
        assert_eq!(listed(&c), [D::Form26as]);
        let mut c = CallerData::default();
        c.traces.ais = Some(Vec::new());
        assert_eq!(listed(&c), [D::Ais]);
        let mut c = CallerData::default();
        c.traces.tis = Some(Vec::new());
        assert_eq!(listed(&c), [D::Tis]);
        let mut c = CallerData::default();
        c.turnover_inputs.gstr1 = turnover();
        assert_eq!(listed(&c), [D::Gstr1]);
        // Data that is not a loaded file lists nothing: the pack never fills the GSTR-3B or AIS
        // turnover, and the report totals come from the read.
        let mut c = CallerData::default();
        c.turnover_inputs.gstr3b = turnover();
        c.turnover_inputs.ais = turnover();
        c.turnover_inputs.books_turnover_paise = Some(1);
        c.report_totals = Some(crate::financial_statements::ReportTotals {
            net_profit_paise: 1,
            closing_stock_paise: None,
            source: None,
        });
        assert!(listed(&c).is_empty());
        // Everything at once, in the pack's order whatever the order of the fields.
        let mut all = CallerData {
            traces: crate::documents::TracesDocuments {
                form26as: Some(Vec::new()),
                ais: Some(Vec::new()),
                tis: Some(Vec::new()),
            },
            bank_statement: Some(statement()),
            named_documents: [
                N::DraftForm3cd,
                N::Gstr3bVs2b,
                N::Gstr3b,
                N::ProfitAndLossReport,
                N::Gstr2b,
            ]
            .into(),
            ..c
        };
        all.turnover_inputs.gstr1 = turnover();
        assert_eq!(
            listed(&all),
            [
                D::Gstr2b,
                D::Gstr1,
                D::Form26as,
                D::ProfitAndLossReport,
                D::Ais,
                D::Tis,
                D::Gstr3b,
                D::Gstr3bVs2b,
                D::BankStatement,
                D::DraftForm3cd,
            ]
        );
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
