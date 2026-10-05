// SPDX-License-Identifier: Apache-2.0
//! Port of the reference engine's `books_examined`: Form 3CD clause 11(b)/(c), the books of account
//! kept and examined, from what this read actually holds.
//!
//! 11(b) asks for the books of account kept and the address at which they are kept; 11(c) for the
//! books and the documents examined. The test can state only what was read: the day book by voucher
//! type, the cash and bank books by ledger count, the ledger accounts, the Trial Balance, and the
//! documents loaded for the engagement. Books kept outside Tally and the address are not in the
//! read, so one question asks the CA for both. It computes nothing.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::Book;
use crate::error::Result;
use crate::findings::{Confidence, Finding, TestResult, Unit, Value};
use crate::rules::Rules;

pub const TEST_ID: &str = "books_examined";
pub const VERSION: &str = "1";

const CASH_GROUP: &str = "Cash-in-Hand";
const BANK_GROUPS: [&str; 2] = ["Bank Accounts", "Bank OD A/c"];
/// The day book's voucher types in the order a CA lists them; any other type follows, by name.
const VOUCHER_ORDER: [&str; 8] = [
    "Sales",
    "Purchase",
    "Receipt",
    "Payment",
    "Contra",
    "Journal",
    "Credit Note",
    "Debit Note",
];

/// A document the reference's pack can load with the read, declared in the order the pack names
/// them, so a set of these iterates in that order and holds each document once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DocumentRead {
    Gstr2b,
    Gstr1,
    Form26as,
    ProfitAndLossReport,
    Ais,
    Tis,
    Gstr3b,
    Gstr3bVs2b,
    BankStatement,
    DraftForm3cd,
}

impl DocumentRead {
    const ALL: [Self; 10] = [
        Self::Gstr2b,
        Self::Gstr1,
        Self::Form26as,
        Self::ProfitAndLossReport,
        Self::Ais,
        Self::Tis,
        Self::Gstr3b,
        Self::Gstr3bVs2b,
        Self::BankStatement,
        Self::DraftForm3cd,
    ];

    /// The name the reference's pack gives the document.
    pub fn name(self) -> &'static str {
        match self {
            Self::Gstr2b => "GSTR-2B",
            Self::Gstr1 => "GSTR-1",
            Self::Form26as => "Form 26AS",
            Self::ProfitAndLossReport => "Tally Profit & Loss report export",
            Self::Ais => "AIS",
            Self::Tis => "TIS",
            Self::Gstr3b => "GSTR-3B",
            Self::Gstr3bVs2b => "GSTR-3B vs 2B workbook",
            Self::BankStatement => "Bank statement",
            Self::DraftForm3cd => "Draft Form 3CD",
        }
    }

    /// The document with exactly this name, if the pack loads one.
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|d| d.name() == name)
    }
}

fn plural(n: usize, word: &str) -> String {
    format!("{n} {word}{}", if n == 1 { "" } else { "s" })
}

/// The books this read of Tally holds, as one line for clause 11. Refuses, as the population does,
/// while any voucher's status is unknown.
pub fn books_read(book: &Book) -> Result<String> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for v in book.population()? {
        *counts.entry(v.base_type.as_str()).or_default() += 1;
    }
    // The CA's order first, then every other type by name (a byte order, which is Python's
    // code-point order).
    let order: Vec<String> = VOUCHER_ORDER
        .iter()
        .filter_map(|t| counts.get(t).map(|n| format!("{t} {n}")))
        .chain(
            counts
                .iter()
                .filter(|(t, _)| !VOUCHER_ORDER.contains(*t))
                .map(|(t, n)| format!("{t} {n}")),
        )
        .collect();
    let mut parts = Vec::new();
    if !order.is_empty() {
        parts.push(format!("day book ({})", order.join(", ")));
    }
    let under = |groups: &[&str]| {
        book.ledgers
            .values()
            .filter(|l| groups.iter().any(|g| l.under(g)))
            .count()
    };
    let cash = under(&[CASH_GROUP]);
    if cash > 0 {
        parts.push(format!("cash book ({})", plural(cash, "cash ledger")));
    }
    let bank = under(&BANK_GROUPS);
    if bank > 0 {
        parts.push(format!("bank books ({})", plural(bank, "bank ledger")));
    }
    parts.push(format!("ledger ({} ledger accounts)", book.ledgers.len()));
    if !book.tb.is_empty() {
        parts.push("trial balance".to_string());
    }
    Ok(format!(
        "Books kept in Tally, as read: {}.",
        parts.join("; ")
    ))
}

/// `documents_read`: the documents the pack loaded for this engagement, never guessed here.
pub fn run(
    book: &Book,
    rules: &Rules,
    documents_read: &BTreeSet<DocumentRead>,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.population_note =
        "The read of the books itself, and the documents loaded with it.".to_string();
    let read = books_read(book)?;
    let f_kept = r.fig(
        "books_maintained",
        Value::Text(format!(
            "{read} The address at which the books are kept: to be confirmed by the CA."
        )),
        Unit::Text,
        "Clause 11(b): the books of account this read of Tally holds; the address is not in the \
books.",
        Vec::new(),
    )?;
    let docs = if documents_read.is_empty() {
        "none loaded with this read".to_string()
    } else {
        documents_read
            .iter()
            .map(|d| d.name())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let f_examined = r.fig(
        "books_examined",
        Value::Text(format!("{read} Documents examined: {docs}.")),
        Unit::Text,
        "Clause 11(c): the books of account read, and the documents loaded for this engagement.",
        Vec::new(),
    )?;
    r.findings.push(Finding {
        id: format!("{TEST_ID}/clause11"),
        clauses: vec!["3CD-11(b)".to_string(), "3CD-11(c)".to_string()],
        title: "Books of account kept and examined: listed from what was read; the CA confirms \
and completes the list"
            .to_string(),
        facts: vec![
            ("books_maintained".to_string(), f_kept),
            ("books_examined".to_string(), f_examined),
        ],
        evidence: Vec::new(),
        confidence: Confidence::JudgementRequired,
        limits: vec![
            "The list is what this read of Tally holds and the documents loaded with it. Books \
kept outside Tally (registers, stock records, a second set of books) and the address at which the \
books are kept are not in the read."
                .to_string(),
        ],
        ask_client: vec![
            "Any books of account kept outside Tally, and the address at which the books are kept."
                .to_string(),
        ],
    });
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::{books_read, DocumentRead};
    use crate::book::{Book, Voucher, VoucherStatus};
    use crate::error::AuditError;

    #[test]
    fn the_document_names_are_unique_and_declared_in_the_packs_order() {
        let names: Vec<&str> = DocumentRead::ALL.iter().map(|d| d.name()).collect();
        assert_eq!(
            names,
            [
                "GSTR-2B",
                "GSTR-1",
                "Form 26AS",
                "Tally Profit & Loss report export",
                "AIS",
                "TIS",
                "GSTR-3B",
                "GSTR-3B vs 2B workbook",
                "Bank statement",
                "Draft Form 3CD",
            ]
        );
        assert!(DocumentRead::ALL.windows(2).all(|w| w[0] < w[1]));
        for d in DocumentRead::ALL {
            assert_eq!(DocumentRead::parse(d.name()), Some(d));
        }
        assert_eq!(DocumentRead::parse("gstr-2b"), None);
        assert_eq!(DocumentRead::parse("GSTR-2B "), None);
    }

    #[test]
    fn a_voucher_of_unknown_status_refuses_the_list() {
        let book = Book {
            vouchers: vec![
                Voucher {
                    status: VoucherStatus::Regular,
                    base_type: "Sales".to_string(),
                    ..Default::default()
                },
                Voucher::default(),
            ],
            ..Default::default()
        };
        assert!(matches!(
            books_read(&book),
            Err(AuditError::UnknownVoucherStatus(1))
        ));
    }
}
