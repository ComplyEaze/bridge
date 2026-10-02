//! s.40A(3) on bank payments made by an instrument that is not an account-payee cheque, a port of
//! the reference Python implementation's `counter_cheques_40a3` test module. The wiring and the
//! tests come first; the port follows in the next commit, so these tests are seen to fail.

use std::collections::BTreeSet;

use crate::book::Book;
use crate::error::Result;
use crate::findings::TestResult;
use crate::rules::Rules;

pub const TEST_ID: &str = "counter_cheques_40a3";
pub const VERSION: &str = "1";

pub fn narration_terms(_raw: Option<&toml::Value>) -> Result<BTreeSet<String>> {
    Ok(BTreeSet::new())
}

pub fn run(
    _book: &Book,
    rules: &Rules,
    _cash: &BTreeSet<String>,
    _bank: &BTreeSet<String>,
    _terms: &BTreeSet<String>,
) -> Result<TestResult> {
    Ok(TestResult::new(TEST_ID, VERSION, &rules.version))
}

pub fn check_invariants(_result: &TestResult, _cash_result: Option<&TestResult>) -> Vec<String> {
    Vec::new()
}
