//! A voucher posted to a mapped ledger names the statement's party in a last
//! segment, `| Statement party: <name>`, after the date (#1430). A line that
//! goes to suspense already names its party where the ledger would be, and a
//! cash line has no counterparty, so neither gets the segment.
//!
//! The narrations are synthetic twins of the SBI UPI shape
//! `UPI/DR/<reference>/<name>/<bank>`.

mod common;

use bridge_bank_statement::bank::{Bank, UNRESOLVED};
use bridge_bank_statement::cash::{CashAnswerRow, CashAnswers, PURPOSE_NOT_CONFIRMED};
use bridge_bank_statement::mapping::{Mapping, MappingRow};
use bridge_bank_statement::parse::Row;
use bridge_bank_statement::proposals::{
    build, suspense_tag, Build, BuildOptions, MAX_NARRATION_CHARS, STATEMENT_PARTY_LABEL,
    UNIDENTIFIED,
};
use common::*;

fn options(answers: &CashAnswers) -> BuildOptions<'_> {
    BuildOptions {
        bank_ledger: "Bank",
        suspense_ledger: "Suspense",
        account_label: "ACC xx1234",
        account_number: "00000000001234",
        date_from: None,
        date_to: None,
        cash_answers: answers,
    }
}

fn mapping(rows: &[(&str, &str, Option<&str>)]) -> Mapping {
    Mapping::from_rows(
        rows.iter()
            .enumerate()
            .map(|(index, (party, ledger, treatment))| MappingRow {
                origin: format!("mapping record {}", index + 2),
                party: party.to_string(),
                ledger: ledger.to_string(),
                treatment: treatment.map(str::to_string),
            }),
    )
    .unwrap()
}

fn sbi(narration: &str, dr: &str, cr: &str) -> Row {
    row(&[
        ("date", "03Aug2026"),
        ("narr", narration),
        ("narr_spaced", narration),
        ("ref", ""),
        ("ref_spaced", ""),
        ("dr", dr),
        ("cr", cr),
        ("bal", "12000.00"),
    ])
}

fn built(rows: &[Row], mapping: &Mapping) -> Build {
    build(rows, Bank::Sbi, mapping, &options(CashAnswers::none())).unwrap()
}

#[test]
fn a_mapped_payment_receipt_and_contra_name_the_statement_party_last() {
    let rows = [
        sbi("TO TRANSFER-UPI/DR/5/RAVI KUMAR/ZZBK", "100.00", ""),
        sbi("TO TRANSFER-UPI/CR/6/NORTH WIND/ZZBK", "", "250.00"),
        sbi("TO TRANSFER-UPI/DR/7/OWN SAVINGS/ZZBK", "75.00", ""),
    ];
    let mapped = mapping(&[
        ("RAVI KUMAR", "Wages", None),
        ("NORTH WIND", "Customers", None),
        ("OWN SAVINGS", "Savings", Some("contra")),
    ]);
    let built = built(&rows, &mapped);
    let narrations: Vec<&str> = built
        .proposals
        .iter()
        .map(|proposal| proposal.narration.as_str())
        .collect();
    assert_eq!(
        narrations,
        [
            "UPI 5 to Wages | ACC xx1234 | 03-Aug-2026 | Statement party: RAVI KUMAR",
            "UPI 6 from Customers | ACC xx1234 | 03-Aug-2026 | Statement party: NORTH WIND",
            "UPI 7 to Savings | ACC xx1234 | 03-Aug-2026 | Statement party: OWN SAVINGS",
        ]
    );
    assert!(narrations
        .iter()
        .all(|narration| narration.contains(&format!(" | {STATEMENT_PARTY_LABEL} "))));
}

#[test]
fn a_line_sent_to_suspense_is_written_exactly_as_before() {
    let unmapped = mapping(&[]);
    let unresolved = built(&[sbi("MISC DEBIT SYNTHETIC", "100.00", "")], &unmapped);
    let text = &unresolved.proposals[0].narration;
    assert!(
        text.ends_with(&format!(
            " to {UNRESOLVED} | ACC xx1234 | 03-Aug-2026 | {UNIDENTIFIED} Suspense"
        )),
        "{text}"
    );
    let unknown = built(
        &[sbi("TO TRANSFER-UPI/DR/5/RAVI KUMAR/ZZBK", "100.00", "")],
        &unmapped,
    );
    assert_eq!(
        unknown.proposals[0].narration,
        format!("UPI 5 to RAVI KUMAR | ACC xx1234 | 03-Aug-2026 | {UNIDENTIFIED} Suspense")
    );
    // a party mapped to the suspense ledger itself is still a suspense line
    let to_suspense = mapping(&[("RAVI KUMAR", "Suspense", None)]);
    let same = built(
        &[sbi("TO TRANSFER-UPI/DR/5/RAVI KUMAR/ZZBK", "100.00", "")],
        &to_suspense,
    );
    assert_eq!(same.proposals[0].narration, unknown.proposals[0].narration);
}

#[test]
fn an_answered_cash_line_has_no_counterparty_to_name() {
    let withdrawal = [sbi("ATM WDL ATM CASH 1234 SYNTHETIC BRANCH", "500.00", "")];
    let id = built(&withdrawal, &mapping(&[])).records[0]
        .bridge_txn_id
        .clone();
    for (answer, ledger, tagged) in [
        ("owner_use", Some("Drawings"), false),
        ("dont_know", None, true),
    ] {
        let given = CashAnswers::from_rows([CashAnswerRow {
            origin: "cash_answers[0]".to_string(),
            bridge_txn_id: id.clone(),
            answer: answer.to_string(),
            ledger: ledger.map(str::to_string),
        }])
        .unwrap();
        let built = build(&withdrawal, Bank::Sbi, &mapping(&[]), &options(&given)).unwrap();
        let narration = &built.proposals[0].narration;
        assert!(!narration.contains(STATEMENT_PARTY_LABEL), "{narration}");
        assert_eq!(
            narration.ends_with(PURPOSE_NOT_CONFIRMED),
            tagged,
            "{narration}"
        );
    }
}

#[test]
fn a_party_spelled_like_a_suspense_tag_is_not_read_as_one() {
    let name = "UNIDENTIFIED - reallocate from Wages";
    let narration = format!("TO TRANSFER-UPI/DR/5/{name}/ZZBK");
    let mapped = mapping(&[(name, "Wages", None)]);
    let built = built(&[sbi(&narration, "100.00", "")], &mapped);
    let written = &built.proposals[0].narration;
    assert!(
        written.ends_with(&format!(" | {STATEMENT_PARTY_LABEL} {name}")),
        "{written}"
    );
    assert_eq!(suspense_tag(written, ["Wages"]), None);
}

#[test]
fn a_party_holding_the_delimiter_or_making_the_narration_too_long_is_refused() {
    let piped = mapping(&[("A|B", "Wages", None)]);
    refuses(
        build(
            &[sbi("TO TRANSFER-UPI/DR/5/A|B/ZZBK", "100.00", "")],
            Bank::Sbi,
            &piped,
            &options(CashAnswers::none()),
        ),
        "party_not_admissible",
    );
    let long_name = "A".repeat(MAX_NARRATION_CHARS);
    let long = mapping(&[(&long_name, "Wages", None)]);
    refuses(
        build(
            &[sbi(
                &format!("TO TRANSFER-UPI/DR/5/{long_name}/ZZBK"),
                "100.00",
                "",
            )],
            Bank::Sbi,
            &long,
            &options(CashAnswers::none()),
        ),
        "party_not_admissible",
    );
}

#[test]
fn the_transaction_id_does_not_depend_on_the_built_narration() {
    let row = sbi("TO TRANSFER-UPI/DR/5/RAVI KUMAR/ZZBK", "100.00", "");
    let to_suspense = built(std::slice::from_ref(&row), &mapping(&[]));
    let to_ledger = built(
        std::slice::from_ref(&row),
        &mapping(&[("RAVI KUMAR", "Wages", None)]),
    );
    assert_ne!(
        to_suspense.proposals[0].narration,
        to_ledger.proposals[0].narration
    );
    assert_eq!(
        to_suspense.records[0].bridge_txn_id,
        to_ledger.records[0].bridge_txn_id
    );
}

/// A Union Bank line the parser names by its wording (a card fee) maps like a
/// party, but the label is the parser's, not a counterparty the statement
/// printed: no segment.
#[test]
fn a_parser_category_is_not_written_as_a_statement_party() {
    let fee = row(&[
        ("date", "03-08-2026"),
        ("narr", "ANN.FEE0000000000000000DATE OF ISSUANCE01-01-2026A"),
        (
            "narr_spaced",
            "ANN.FEE0000000000000000DATE OF ISSUANCE01-01-2026A",
        ),
        ("ref", "A123456"),
        ("dr", "590.00"),
        ("cr", ""),
        ("bal", "9000.00"),
    ]);
    let mapped = mapping(&[("CARD ANNUAL FEE", "Bank Charges", None)]);
    let built = build(&[fee], Bank::Ubi, &mapped, &options(CashAnswers::none())).unwrap();
    let narration = &built.proposals[0].narration;
    assert!(narration.contains(" to Bank Charges | "), "{narration}");
    assert!(!narration.contains(STATEMENT_PARTY_LABEL), "{narration}");
}

#[test]
fn a_party_carrying_the_reserved_marker_is_refused() {
    let name = "X [BRIDGE: Y";
    let marked = mapping(&[(name, "Wages", None)]);
    refuses(
        build(
            &[sbi(
                &format!("TO TRANSFER-UPI/DR/5/{name}/ZZBK"),
                "100.00",
                "",
            )],
            Bank::Sbi,
            &marked,
            &options(CashAnswers::none()),
        ),
        "party_not_admissible",
    );
}
