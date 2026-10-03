// SPDX-License-Identifier: Apache-2.0
//! `counter_cheques_40a3`: the one `[roles]` key it reads is validated by it alone, the golden of its
//! main edge book is anchored on named values (so an empty or shifted dump cannot stand in for it),
//! and each of its module invariants fires on its own tampering.

mod common;

use bridge_tax_audit::counter_cheques_40a3::{check_invariants, TEST_ID};
use bridge_tax_audit::error::AuditError;
use bridge_tax_audit::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use bridge_tax_audit::{
    counter_cheques_40a3_canonical, rules_for, trial_balance_canonical, Engagement,
};
use serde_json::{json, Value as Json};

/// The synthetic engagement with one `[roles]` line added.
fn engagement_with(terms_line: &str) -> Engagement {
    let text = std::fs::read_to_string(common::fixtures().join("synthetic-engagement.toml"))
        .unwrap()
        .replacen(
            "round_off_ledgers = [\"Round Off\"]\n",
            &format!("round_off_ledgers = [\"Round Off\"]\n{terms_line}\n"),
            1,
        );
    assert!(text.contains(terms_line), "the [roles] anchor moved");
    Engagement::from_toml(&text, &common::fixtures()).unwrap()
}

fn terms_count(doc: &Json) -> Json {
    doc["figures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "counter_cheques_40a3.configured_terms_count")
        .unwrap()["value"]
        .clone()
}

#[test]
fn a_malformed_value_fails_counter_cheques_alone() {
    for line in [
        "counter_cheque_narration_terms = \"COUNTER\"",
        "counter_cheque_narration_terms = [\"COUNTER\", 7]",
        "counter_cheque_narration_terms = { COUNTER = 1 }",
    ] {
        let e = engagement_with(line);
        let rules = rules_for(&e).unwrap();
        assert!(trial_balance_canonical(&e, &rules).is_ok(), "{line}");
        let err = counter_cheques_40a3_canonical(&e, &rules).unwrap_err();
        assert!(matches!(err, AuditError::Config(_)), "{line}: {err:?}");
    }
}

#[test]
fn a_list_of_strings_reaches_the_test_and_repeats_collapse() {
    let e =
        engagement_with("counter_cheque_narration_terms = [\"counter\", \"counter\", \"Bearer\"]");
    let doc = counter_cheques_40a3_canonical(&e, &rules_for(&e).unwrap()).unwrap();
    assert_eq!(terms_count(&doc), 2);
}

#[test]
fn an_absent_key_is_no_terms() {
    let e = engagement_with("");
    let doc = counter_cheques_40a3_canonical(&e, &rules_for(&e).unwrap()).unwrap();
    assert_eq!(terms_count(&doc), 0);
}

fn figure(doc: &Json, name: &str) -> Json {
    doc["figures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == format!("counter_cheques_40a3.{name}"))
        .unwrap_or_else(|| panic!("no figure {name}"))["value"]
        .clone()
}

#[test]
fn the_terms_book_golden_holds_the_scenario_it_was_written_for() {
    let g = common::golden_named("edge.cc_terms.counter_cheques_40a3");
    assert_eq!(g["figures"].as_array().unwrap().len(), 26);
    assert_eq!(g["findings"].as_array().unwrap().len(), 7);
    // Eight terms from nine entries: the repeat collapses, the empty term counts.
    assert_eq!(figure(&g, "configured_terms_count"), 8);
    // 14 matched lines (the term holding U+019B matches nothing: only Rust's own upper-casing would
    // turn it into the narration's U+A7DC); a payment of exactly the limit is under and one paisa
    // more is over.
    assert_eq!(figure(&g, "matched_expenditure_count"), 14);
    assert_eq!(figure(&g, "over_limit_count"), 7);
    assert_eq!(figure(&g, "over_limit_total"), 9_200_001);
    assert_eq!(figure(&g, "at_or_under_limit_count"), 7);
    // One excluded payee per role that has one; the asset-side loans role has none.
    assert_eq!(figure(&g, "excluded_total_capital"), 1_000_000);
    assert_eq!(figure(&g, "excluded_total_loans_liability"), 500_000);
    assert_eq!(figure(&g, "excluded_total_fixed_assets"), 700_000);
    assert_eq!(figure(&g, "excluded_total_duties_taxes"), 300_000);
    assert_eq!(figure(&g, "excluded_total_loans_advances_asset"), 0);
    assert_eq!(json!(g["module_invariant_violations"]), json!([]));
}

#[test]
fn the_unconfigured_book_golden_looks_at_nothing() {
    let g = common::golden_named("edge.cc_unconfigured.counter_cheques_40a3");
    assert_eq!(g["figures"].as_array().unwrap().len(), 12);
    assert!(g["findings"].as_array().unwrap().is_empty());
    assert_eq!(figure(&g, "configured_terms_count"), 0);
    assert_eq!(figure(&g, "matched_expenditure_count"), 0);
    // The role totals still read zero, and no payment is excluded.
    assert_eq!(figure(&g, "excluded_total_capital"), 0);
}

fn result_with(
    over_total: i64,
    over_count: i64,
    rows: &[i64],
    findings: Vec<Finding>,
) -> TestResult {
    let mut r = TestResult::new(TEST_ID, "1", "test");
    r.fig(
        "over_limit_total",
        Value::Int(over_total),
        Unit::Paise,
        "d",
        Vec::new(),
    )
    .unwrap();
    r.fig(
        "over_limit_count",
        Value::Int(over_count),
        Unit::Count,
        "d",
        Vec::new(),
    )
    .unwrap();
    for (i, paise) in rows.iter().enumerate() {
        r.fig(
            &format!("row_amount_{i}"),
            Value::Int(*paise),
            Unit::Paise,
            "d",
            Vec::new(),
        )
        .unwrap();
    }
    r.findings = findings;
    r
}

fn finding(id: &str, voucher: &str) -> Finding {
    Finding {
        id: id.to_string(),
        clauses: Vec::new(),
        title: "t".to_string(),
        facts: Vec::new(),
        evidence: vec![
            EvidenceRef::new("voucher", voucher),
            EvidenceRef::new("ledger", voucher),
        ],
        confidence: Confidence::NeedsDocument,
        limits: Vec::new(),
        ask_client: Vec::new(),
    }
}

#[test]
fn a_consistent_result_has_no_violation() {
    let r = result_with(
        500,
        2,
        &[200, 300],
        vec![finding("a", "v1"), finding("b", "v2")],
    );
    assert!(check_invariants(&r, None).is_empty());
}

#[test]
fn ccq_1_fires_when_the_total_is_not_the_sum_of_the_rows() {
    let r = result_with(
        501,
        2,
        &[200, 300],
        vec![finding("a", "v1"), finding("b", "v2")],
    );
    assert_eq!(
        check_invariants(&r, None),
        vec!["CCQ-1: over_limit_total != sum of row_amount_* figures"]
    );
}

#[test]
fn ccq_2_fires_when_the_count_is_not_the_number_of_findings() {
    let r = result_with(
        500,
        3,
        &[200, 300],
        vec![finding("a", "v1"), finding("b", "v2")],
    );
    assert_eq!(
        check_invariants(&r, None),
        vec!["CCQ-2: over_limit_count != number of findings"]
    );
}

#[test]
fn ccq_3_fires_on_a_voucher_both_s40a3_tests_report_and_only_then() {
    let mine = result_with(
        500,
        2,
        &[200, 300],
        vec![finding("a", "v1"), finding("b", "v2")],
    );
    let mut cash = TestResult::new("cash_payments_40a3", "1", "test");
    cash.findings = vec![finding("cash_payments_40a3/s40a3/x", "v2")];
    assert_eq!(
        check_invariants(&mine, Some(&cash)),
        vec!["CCQ-3: 1 voucher(s) counted by both s.40A(3) tests"]
    );
    // A cash finding of another family does not count, and a disjoint voucher does not either.
    cash.findings = vec![
        finding("cash_payments_40a3/s269st/x", "v2"),
        finding("cash_payments_40a3/s40a3/y", "v9"),
    ];
    assert!(check_invariants(&mine, Some(&cash)).is_empty());
    // A ledger ref carrying the same id is not a voucher.
    let mut cash2 = TestResult::new("cash_payments_40a3", "1", "test");
    let mut f = finding("cash_payments_40a3/s40a3/z", "other");
    f.evidence = vec![EvidenceRef::new("ledger", "v2")];
    cash2.findings = vec![f];
    assert!(check_invariants(&mine, Some(&cash2)).is_empty());
}

fn without(mut r: TestResult, name: &str) -> TestResult {
    let id = format!("{TEST_ID}.{name}");
    r.figures.retain(|f| f.id != id);
    r
}

fn with_value(mut r: TestResult, name: &str, value: Value) -> TestResult {
    let id = format!("{TEST_ID}.{name}");
    r.figures.iter_mut().find(|f| f.id == id).unwrap().value = value;
    r
}

fn consistent() -> TestResult {
    result_with(
        500,
        2,
        &[200, 300],
        vec![finding("a", "v1"), finding("b", "v2")],
    )
}

/// #1121: a result that has lost the figure an invariant checks fails that invariant; it is not
/// reported as holding.
#[test]
fn ccq_1_and_ccq_2_fire_when_the_figure_they_check_is_absent() {
    assert_eq!(
        check_invariants(&without(consistent(), "over_limit_total"), None),
        vec!["CCQ-1: over_limit_total figure absent or not an integer"]
    );
    assert_eq!(
        check_invariants(&without(consistent(), "over_limit_count"), None),
        vec!["CCQ-2: over_limit_count figure absent or not an integer"]
    );
}

#[test]
fn a_total_or_count_that_is_not_an_integer_is_a_violation() {
    for value in [Value::Text("500".to_string()), Value::Undefined] {
        assert_eq!(
            check_invariants(
                &with_value(consistent(), "over_limit_total", value.clone()),
                None
            ),
            vec!["CCQ-1: over_limit_total figure absent or not an integer"]
        );
        assert_eq!(
            check_invariants(&with_value(consistent(), "over_limit_count", value), None),
            vec!["CCQ-2: over_limit_count figure absent or not an integer"]
        );
    }
}

/// A row amount that is not an integer is a violation of its own, not a zero in the sum.
#[test]
fn a_row_amount_that_is_not_an_integer_is_a_violation_not_a_zero() {
    let r = with_value(consistent(), "row_amount_0", Value::Text("200".to_string()));
    assert_eq!(
        check_invariants(&r, None),
        vec!["CCQ-1: 1 row_amount_* figure(s) not an integer"]
    );
}
