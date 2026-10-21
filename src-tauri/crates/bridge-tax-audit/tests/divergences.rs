// SPDX-License-Identifier: Apache-2.0
//! The committed divergences from the frozen reference (`fixtures/divergences.json`) all apply:
//! each names a registered test, each golden it patches exists and takes the patch, and each
//! patched golden is still a dump the comparison accepts.

mod common;

use bridge_tax_audit::compare::{compare, validate_types};
use bridge_tax_audit::registry;

#[test]
fn every_committed_divergence_applies_to_a_golden_of_its_test() {
    for d in common::divergences() {
        assert!(
            registry::find(&d.test).is_some(),
            "{}: {} is not a registered test",
            d.id,
            d.test
        );
        for stem in d.expect.keys() {
            assert!(
                common::fixtures()
                    .join(format!("golden/{stem}.json"))
                    .is_file(),
                "{}: no golden {stem}",
                d.id
            );
            let patched = common::golden_named(stem);
            validate_types(&patched, stem).unwrap_or_else(|e| panic!("{}: {stem}: {e:?}", d.id));
        }
    }
}

/// A divergence over a real golden: the engine's synthetic `cash_44ab` result compared with the
/// golden patched to another cash-receipts value differs at exactly that figure, and with the
/// patch's value it differs nowhere.
#[test]
fn a_declared_patch_is_what_the_comparison_holds_rust_to() {
    let rust = common::run(&common::fixtures().join("synthetic-read"), false).unwrap();
    let oracle = common::oracle_named("synthetic.cash_44ab");
    let id = "cash_44ab.cash_receipts";
    let actual = oracle["figures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap()["value"]
        .as_i64()
        .unwrap();
    let record = |value: i64| {
        bridge_tax_audit::divergence::parse(
            &serde_json::json!([{
                "id": "DV-T", "issue": 1, "test": "cash_44ab", "why": "test", "ca_answer": "test",
                "expect": {"synthetic.cash_44ab": {format!("figures[{id}].value"): value}}
            }])
            .to_string(),
        )
        .unwrap()
    };
    let other = bridge_tax_audit::divergence::patched(
        &oracle,
        "synthetic.cash_44ab",
        &record(actual + 100),
    )
    .unwrap();
    let differences = compare(&other, &rust, None).unwrap();
    assert_eq!(differences.len(), 1, "{differences:?}");
    assert!(
        differences[0].starts_with(&format!("{id}: value differs")),
        "{differences:?}"
    );
    // Rust's own value cannot be declared: it equals the oracle.
    assert!(
        bridge_tax_audit::divergence::patched(&oracle, "synthetic.cash_44ab", &record(actual))
            .unwrap_err()
            .contains("equals the oracle")
    );
    assert!(compare(&oracle, &rust, None).unwrap().is_empty());
}
