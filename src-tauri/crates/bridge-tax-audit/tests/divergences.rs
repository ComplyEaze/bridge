// SPDX-License-Identifier: Apache-2.0
//! The committed divergences from the frozen reference (`fixtures/divergences.json`) all apply:
//! each names a registered test, each golden it patches exists and takes the patch, and each
//! patched golden is still a dump the comparison accepts.

mod common;

use bridge_tax_audit::compare::validate_types;
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
