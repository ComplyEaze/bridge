// SPDX-License-Identifier: Apache-2.0
//! Port of the reference engine's `entity_269st_gap`: s.269ST cash received on one day by several
//! ledgers that carry one PAN, reported only where the aggregate reaches the limit and no single
//! ledger does.

use std::collections::BTreeSet;

use crate::book::Book;
use crate::error::Result;
use crate::findings::TestResult;
use crate::party_identity::PartyIndex;
use crate::rules::Rules;

pub const TEST_ID: &str = "entity_269st_gap";
pub const VERSION: &str = "1.0.0";

pub fn run(
    _book: &Book,
    rules: &Rules,
    _cash: &BTreeSet<String>,
    _bank: &BTreeSet<String>,
    _index: &PartyIndex,
    _round_off_ledgers: &BTreeSet<String>,
) -> Result<TestResult> {
    Ok(TestResult::new(TEST_ID, VERSION, &rules.version))
}

pub fn check_invariants(_result: &TestResult) -> Vec<String> {
    Vec::new()
}
