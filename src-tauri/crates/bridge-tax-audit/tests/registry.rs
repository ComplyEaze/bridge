// SPDX-License-Identifier: Apache-2.0
//! CI parity for every registered test at once: each entry in `registry::PORTED` has a synthetic
//! golden (`golden/synthetic.<id>.json`) and matches it over the committed synthetic read, and the
//! Python side's `parity/python_golden.py` `RUNNERS` names exactly the same tests. A newly ported
//! test is covered here by its registry entry and golden alone; its own `parity_<id>.rs` adds the
//! anchored checks (figure counts, named values, a changed value reported).

mod common;

use bridge_tax_audit::compare::compare;
use bridge_tax_audit::registry::{self, CallerData, PORTED};
use bridge_tax_audit::rules_for;

fn caller(id: &str) -> CallerData {
    let json = |name: &str| -> serde_json::Value {
        serde_json::from_str(&std::fs::read_to_string(common::fixtures().join(name)).unwrap())
            .unwrap()
    };
    let mut c = CallerData::default();
    match id {
        "financial_statements" => {
            c.report_totals = Some(
                registry::report_totals_from_json(&json("synthetic-report-totals.json")).unwrap(),
            );
        }
        "applicability_44ab" => {
            c.turnover_inputs =
                registry::turnover_inputs_from_json(&json("synthetic-turnover-inputs.json"))
                    .unwrap();
        }
        _ => {}
    }
    // `high_value_register` takes both documents, optionally; its golden is made with both.
    if matches!(id, "bank_reconciliation" | "high_value_register") {
        c.bank_statement = Some(
            bridge_tax_audit::documents::bank_statement_from_json(&json(
                "synthetic-bank-statement.json",
            ))
            .unwrap(),
        );
    }
    if matches!(
        id,
        "tds_tcs_26as" | "twentysixas_receipts" | "high_value_register"
    ) {
        c.traces = bridge_tax_audit::documents::traces_documents_from_json(&json(
            "synthetic-traces-documents.json",
        ))
        .unwrap();
    }
    c
}

/// Tests the reference itself ends on the synthetic read, with the refusal code the port gives:
/// `entity_269st_gap` (`party_identity.IncompleteLedgerChain`: the read has a ledger whose group
/// chain is incomplete and does not settle whether it is a party). Their parity is the edge books'.
const SYNTHETIC_REFUSALS: [(&str, &str); 1] = [("entity_269st_gap", "PARTY-chain-incomplete")];

#[test]
fn every_registered_test_matches_its_synthetic_golden() {
    let e = common::engagement(&common::fixtures().join("synthetic-read"), false);
    let rules = rules_for(&e).unwrap();
    for test in PORTED {
        if let Some((_, code)) = SYNTHETIC_REFUSALS.iter().find(|(id, _)| *id == test.id) {
            // The reference raises on this read, so there is no golden: the port refuses, typed.
            let err = registry::run_canonical(test.id, &e, &rules, &caller(test.id))
                .expect_err("the reference raises on the synthetic read");
            assert_eq!(err.code(), Some(*code), "{}: {err}", test.id);
            continue;
        }
        let rust = registry::run_canonical(test.id, &e, &rules, &caller(test.id)).unwrap();
        let golden = common::golden_named(&format!("synthetic.{}", test.id));
        let diffs = compare(&golden, &rust, None).unwrap();
        assert!(diffs.is_empty(), "{}:\n{}", test.id, diffs.join("\n"));
    }
}

#[test]
fn the_python_runners_name_the_same_tests() {
    // Read as text: the Python module needs the reference engine on its path to import.
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("parity/python_golden.py"),
    )
    .unwrap();
    let block = text
        .split("\nRUNNERS = {\n")
        .nth(1)
        .and_then(|rest| rest.split("\n}\n").next())
        .expect("python_golden.py has a RUNNERS = { ... } block");
    let python: Vec<&str> = block
        .lines()
        .map(|l| l.trim().split('"').nth(1).expect("one quoted id per line"))
        .collect();
    let rust: Vec<&str> = PORTED.iter().map(|t| t.id).collect();
    assert_eq!(python, rust);
}

#[test]
fn an_unregistered_test_is_refused() {
    let e = common::engagement(&common::fixtures().join("synthetic-read"), false);
    let rules = rules_for(&e).unwrap();
    let err = registry::run_canonical("itr_3cd_tally", &e, &rules, &CallerData::default())
        .unwrap_err()
        .to_string();
    assert!(err.contains("itr_3cd_tally is not a ported test"), "{err}");
}

#[test]
fn the_26as_floors_are_what_each_test_emits_with_no_documents() {
    // The synthetic engagement configures no TDS/TCS ledger and two deductor aliases, so with no
    // document rows `tds_tcs_26as` emits its structural figures plus one per alias, and
    // `twentysixas_receipts` (every figure belongs to a party with a Part I row) emits none.
    let e = common::engagement(&common::fixtures().join("synthetic-read"), false);
    let rules = rules_for(&e).unwrap();
    let figures = |id: &str| {
        registry::run_canonical(id, &e, &rules, &CallerData::default()).unwrap()["figures"]
            .as_array()
            .unwrap()
            .len()
    };
    let floor = |id: &str| registry::find(id).unwrap().min_figures;
    assert_eq!(figures("tds_tcs_26as"), floor("tds_tcs_26as") + 2);
    assert_eq!(figures("twentysixas_receipts"), 0);
    assert_eq!(floor("twentysixas_receipts"), 1);
}

/// `high_value_register`'s s.194N coverage, as the registry passes the caller's two statement
/// inputs: a refused statement (no document) is stated with the reader's reason, never as "not
/// supplied"; a statement that is supplied is read, whatever reason is also given.
#[test]
fn the_registry_passes_a_refused_statement_reason_to_the_coverage_figure() {
    let e = common::engagement(&common::fixtures().join("synthetic-read"), false);
    let rules = rules_for(&e).unwrap();
    let coverage = |c: &CallerData| -> String {
        let dump = registry::run_canonical("high_value_register", &e, &rules, c).unwrap();
        let fig = dump["figures"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["id"] == "high_value_register.s194n_coverage")
            .expect("the coverage figure is always emitted");
        fig["value"].as_str().unwrap().to_string()
    };
    let mut c = CallerData::default();
    assert_eq!(
        coverage(&c),
        "no bank statement supplied for this engagement"
    );
    c.bank_statement_refused = Some("has no rows".to_string());
    assert_eq!(
        coverage(&c),
        "the bank statement supplied was refused (it has no rows)"
    );
    c.bank_statement = caller("high_value_register").bank_statement;
    assert!(
        coverage(&c).contains(" only -- s.194N is an annual test"),
        "a supplied statement is read"
    );
}
