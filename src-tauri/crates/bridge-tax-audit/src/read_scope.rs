// SPDX-License-Identifier: Apache-2.0
//! Port of the reference engine's `read_scope`: what this read of the books covers, stated where
//! the CA reads it.

use crate::book::Book;
use crate::error::Result;
use crate::findings::TestResult;
use crate::rules::Rules;

pub const TEST_ID: &str = "read_scope";
pub const VERSION: &str = "1";

pub fn run(_book: &Book, rules: &Rules) -> Result<TestResult> {
    Ok(TestResult::new(TEST_ID, VERSION, &rules.version))
}
