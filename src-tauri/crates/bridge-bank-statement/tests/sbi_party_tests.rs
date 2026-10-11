//! The SBI party reader names a counterparty or says it could not: an empty
//! name field must never come back as an empty party.
//!
//! The narrations are synthetic. The UPI shape is the one the SBI parser reads
//! (`UPI/DR/<reference>/<name>/<bank>`); a statement that prints an empty name
//! field has not been seen, so these pin what the reader does if one appears.

mod common;

use bridge_bank_statement::bank::{Bank, PARSER_SENTINELS, UNNAMED};
use bridge_bank_statement::cash::CashAnswers;
use bridge_bank_statement::mapping::{Mapping, MappingRow};
use bridge_bank_statement::parse::Row;
use bridge_bank_statement::proposals::{build, BuildOptions};
use common::*;

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

#[test]
fn an_empty_upi_name_field_is_unnamed_not_an_empty_party() {
    for narration in ["TO TRANSFER-UPI/DR/5//ZZBK", "TO TRANSFER-UPI/DR/5/ /ZZBK"] {
        let party = Bank::Sbi.party(&sbi(narration, "100.00", ""));
        assert_eq!(party, UNNAMED, "{narration:?}");
    }
}

/// The same omission in the NEFT, RTGS and hyphen-only UPI shapes, which the
/// empty UPI field led to: a name field that is empty, blank or only hyphens.
#[test]
fn an_empty_neft_rtgs_or_hyphen_only_name_field_is_unnamed_too() {
    for narration in [
        "TO TRANSFER-NEFT*ZZBK0001*ZZ1**",
        "TO TRANSFER-NEFT*ZZBK0001*ZZ1* *",
        "NEFT*ZZBK0001*ZZ1*-",
        "RTGS UTR NO: ZZBKR1234- ",
        "RTGS UTR NO: ZZBKR1234-   ",
        "TO TRANSFER-UPI/DR/5/-/ZZBK",
        "TO TRANSFER-UPI/DR/5/",
    ] {
        let party = Bank::Sbi.party(&sbi(narration, "100.00", ""));
        assert_eq!(party, UNNAMED, "{narration:?}");
    }
}

/// The reference column names the party for transfer lines; a name that is
/// only spaces there is empty too.
#[test]
fn a_blank_name_in_the_reference_column_is_unnamed_too() {
    for reference in ["TRANSFER TO 123  / 456", "CT0 TRANSFER FROM 123   /"] {
        let line = row(&[
            ("date", "03Aug2026"),
            ("narr", "TO TRANSFER-PAYMENT"),
            ("narr_spaced", "TO TRANSFER-PAYMENT"),
            ("ref", reference),
            ("ref_spaced", reference),
            ("dr", "100.00"),
            ("cr", ""),
            ("bal", "12000.00"),
        ]);
        assert_eq!(Bank::Sbi.party(&line), UNNAMED, "{reference:?}");
    }
}

#[test]
fn a_named_upi_field_is_still_read() {
    let party = Bank::Sbi.party(&sbi("TO TRANSFER-UPI/DR/5/ACME EXPORTS/ZZBK", "100.00", ""));
    assert_eq!(party, "ACME EXPORTS");
    let neft = Bank::Sbi.party(&sbi(
        "TO TRANSFER-NEFT*ZZBK0001*ZZ1*ACME EXPORTS*",
        "100.00",
        "",
    ));
    assert_eq!(neft, "ACME EXPORTS");
}

/// What the empty party did downstream: it is never a mapping key (a row with
/// an empty party is dropped), so the line went to suspense with a narration
/// that named no one. It now reaches suspense naming the sentinel.
#[test]
fn an_unnamed_upi_line_reaches_suspense_and_its_narration_names_the_sentinel() {
    let mapping = Mapping::from_rows(std::iter::empty::<MappingRow>()).unwrap();
    let options = BuildOptions {
        bank_ledger: "Bank",
        suspense_ledger: "Suspense",
        account_label: "ACC xx1234",
        account_number: "00000000001234",
        date_from: None,
        date_to: None,
        cash_answers: CashAnswers::none(),
    };
    let rows = [sbi("TO TRANSFER-UPI/DR/5//ZZBK", "100.00", "")];
    let built = build(&rows, Bank::Sbi, &mapping, &options).unwrap();
    assert_eq!(built.records[0].party, UNNAMED);
    assert!(built.records[0].suspense);
    let narration = &built.proposals[0].narration;
    assert_eq!(
        narration,
        "UPI 5 to UNNAMED | ACC xx1234 | 03-Aug-2026 | UNIDENTIFIED - reallocate from Suspense"
    );
    assert!(PARSER_SENTINELS.contains(&UNNAMED));
}
