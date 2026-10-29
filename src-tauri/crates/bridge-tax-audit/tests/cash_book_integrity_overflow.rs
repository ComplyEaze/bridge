// SPDX-License-Identifier: Apache-2.0
//! `cash_book_integrity` refuses a cash balance that would overflow i64 paise, rather than report
//! a wrapped (sign-flipped) balance as a negative cash day. No realistic book reaches the bound, so
//! the book below is invented to sit at it; every name and amount is invented.
//!
//! This file carries its own small builders rather than adding them to `tests/common` or this test
//! to `module_invariants_batch1.rs`: an edit to either makes the mutation runner select every
//! mutation their tests are recorded as killing.

use std::collections::BTreeSet;

use bridge_tally_primitives::TallyDate;
use bridge_tax_audit::book::{Book, Ledger, LedgerLine, TbRow, Voucher, VoucherStatus};
use bridge_tax_audit::findings::Value;
use bridge_tax_audit::rules::Rules;
use bridge_tax_audit::{cash_book_integrity, AuditError};

fn ledger(name: &str, group: &str) -> Ledger {
    Ledger {
        name: name.to_string(),
        parent: group.to_string(),
        chain: vec![group.to_string()],
        chain_complete: true,
        master_opening_paise: 0,
        pan: String::new(),
        gstin: String::new(),
        guid: String::new(),
        masterid: None,
    }
}

fn opening(paise: i64) -> TbRow {
    TbRow {
        opening_paise: paise,
        debit_paise: 0,
        credit_paise: 0,
        closing_paise: paise,
    }
}

/// A cash ledger opening at `cash_opening`, and one receipt of 100 paise into it.
fn book(cash_opening: i64) -> Book {
    let receipt = Voucher {
        guid: "r1".to_string(),
        date: TallyDate::parse("20250601").unwrap(),
        vtype: "Receipt".to_string(),
        base_type: "Receipt".to_string(),
        number: "r1".to_string(),
        status: VoucherStatus::Regular,
        lines: [("Cash", 100), ("Invented Debtor", -100)]
            .iter()
            .map(|(l, a)| LedgerLine {
                ledger: (*l).to_string(),
                amount_paise: *a,
            })
            .collect(),
        ..Default::default()
    };
    Book {
        company_name: "Invented Traders".to_string(),
        company_guid: "invented-guid".to_string(),
        ledgers: [
            ledger("Cash", "Cash-in-Hand"),
            ledger("Invented Debtor", "Sundry Debtors"),
        ]
        .into_iter()
        .map(|l| (l.name.clone(), l))
        .collect(),
        vouchers: vec![receipt],
        tb: [
            ("Cash".to_string(), opening(cash_opening)),
            ("Invented Debtor".to_string(), opening(0)),
        ]
        .into_iter()
        .collect(),
        ..Default::default()
    }
}

fn run(b: &Book) -> bridge_tax_audit::Result<bridge_tax_audit::findings::TestResult> {
    let cash: BTreeSet<String> = ["Cash".to_string()].into();
    cash_book_integrity::run(b, &Rules::vendored().unwrap(), &cash, &BTreeSet::new(), &[])
}

#[test]
fn a_cash_balance_past_i64_paise_is_refused_not_wrapped() {
    // The control: the receipt brings the balance to exactly i64::MAX, which fits.
    let fits = run(&book(i64::MAX - 100)).unwrap();
    let best_case_negative_days = fits
        .figures
        .iter()
        .find(|f| {
            f.id.starts_with("cash_book_integrity.negative_days_best_case_")
        })
        .unwrap();
    assert_eq!(best_case_negative_days.value, Value::Int(0));

    // One paisa more and the day's closing cannot be held; the test must refuse.
    match run(&book(i64::MAX - 99)) {
        Err(AuditError::Config(detail)) => assert_eq!(
            detail,
            format!(
                "{}: a total overflowed i64 paise",
                cash_book_integrity::TEST_ID
            )
        ),
        other => panic!("expected the overflow refusal, got {other:?}"),
    }
}
