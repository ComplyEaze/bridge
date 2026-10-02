// SPDX-License-Identifier: Apache-2.0
//! Port of the reference engine's `read_scope`: what this read of the books covers, stated where
//! the CA reads it.
//!
//! Amounts are read as Indian rupees only, and an amount that carries a foreign-currency value is
//! refused when the book is read (FX-1). But a read that does not carry the books' currency settings
//! cannot show that every ledger is kept in rupees. Until it does (`Book::currency_read`) this test
//! says so and asks the client to confirm. It computes nothing.

use crate::book::Book;
use crate::error::Result;
use crate::findings::{Confidence, Finding, TestResult, Unit, Value};
use crate::rules::Rules;

pub const TEST_ID: &str = "read_scope";
pub const VERSION: &str = "1";

pub fn run(book: &Book, rules: &Rules) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.population_note = "The read of the books itself: no voucher is examined.".to_string();
    let f_currency = r.fig(
        "currency_read",
        Value::Text(if book.currency_read { "yes" } else { "no" }.to_string()),
        Unit::Text,
        "Whether this read of the books includes their currency settings.",
        Vec::new(),
    )?;
    if !book.currency_read {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/currency_not_read"),
            clauses: Vec::new(),
            title: "The books' currency was not read: every amount is taken as Indian rupees"
                .to_string(),
            facts: vec![("currency_read".to_string(), f_currency)],
            evidence: Vec::new(),
            confidence: Confidence::NeedsDocument,
            limits: vec![
                "This read of the books does not include their currency settings. Every ledger is \
taken to be kept in Indian rupees. A book kept in another currency, or a ledger kept in a foreign \
currency whose amounts carry no foreign-currency value, would not be detected."
                    .to_string(),
                "Amounts that do carry a foreign-currency value are refused, so no figure mixes \
currencies from them."
                    .to_string(),
            ],
            ask_client: vec![
                "Confirm that the books are kept in Indian rupees and that no ledger is kept in a \
foreign currency."
                    .to_string(),
            ],
        });
    }
    Ok(r)
}
