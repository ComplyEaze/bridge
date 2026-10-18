//! What the narration and the ledger names of a proposal may carry, and which refusal says so.
//!
//! The Python statement tool (since removed, #1748) held these as `build`
//! assertions; the app refuses a bad narration only at `build_import_xml`, after a person has
//! mapped and answered, so the crate's own per-row refusals are the early warning and need their
//! own guards: control characters, the reserved marker, the exact length boundary, the order
//! in which the party and the narration are judged, and the leniency for a line that never
//! writes a party segment (suspense, skipped).
//!
//! The narrations are synthetic twins of the SBI UPI shape `UPI/DR/<reference>/<name>/<bank>`.

mod common;

use bridge_bank_statement::bank::{Bank, PARSER_CATEGORIES, PARSER_SENTINELS, UNNAMED, UNRESOLVED};
use bridge_bank_statement::cash::{CashAnswers, CASH_DEPOSIT, CASH_WITHDRAWAL};
use bridge_bank_statement::mapping::{Mapping, MappingRow};
use bridge_bank_statement::parse::Row;
use bridge_bank_statement::proposals::{
    build, Build, BuildOptions, Disposition, MAX_NARRATION_CHARS, STATEMENT_PARTY_LABEL,
};
use common::*;

fn options(label: &str) -> BuildOptions<'_> {
    BuildOptions {
        bank_ledger: "Bank",
        suspense_ledger: "Suspense",
        account_label: label,
        account_number: "0000001234",
        date_from: None,
        date_to: None,
        cash_answers: CashAnswers::none(),
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

fn debit_to(party: &str) -> Row {
    row(&[
        ("date", "03Aug2026"),
        ("narr", &format!("TO TRANSFER-UPI/DR/5/{party}/ZZBK")),
        ("narr_spaced", &format!("TO TRANSFER-UPI/DR/5/{party}/ZZBK")),
        ("ref", ""),
        ("ref_spaced", ""),
        ("dr", "100.00"),
        ("cr", ""),
        ("bal", "12000.00"),
    ])
}

fn built(
    party: &str,
    mapped: &Mapping,
    label: &str,
) -> Result<Build, bridge_bank_statement::Refusal> {
    build(&[debit_to(party)], Bank::Sbi, mapped, &options(label))
}

#[test]
fn a_mapped_party_with_a_control_character_or_the_marker_is_refused() {
    // The C0 range at its start (\x00), DEL and the C1 range at its ends (\x80 and \x9f), C0
    // characters the squash does not fold (\x07, \x1b), and the reserved marker in either
    // case. (\x1f and \x85 are whitespace to the squash and never reach the check: see the next test.)
    for party in [
        "A\u{0}B",
        "A\u{7}B",
        "A\u{1b}B",
        "A\u{7f}B",
        "A\u{80}B",
        "A\u{9f}B",
        "X [BRIDGE:1] Y",
        "X [Bridge:1] Y",
    ] {
        let mapped = mapping(&[(party, "Wages", None)]);
        let refusal = built(party, &mapped, "ACC xx1234").err();
        let category = refusal.as_ref().map(|refusal| refusal.category);
        assert!(
            matches!(category, Some("party_not_admissible")),
            "{party:?} was {refusal:?}"
        );
    }
}

#[test]
fn a_whitespace_control_character_is_folded_before_the_party_is_judged() {
    // \x1f and \x85 are whitespace to the squash (as to Python's `str.isspace`), so the party is
    // written with a space and is admissible; the fold, not the check, handles them.
    for party in ["A\u{1f}B", "A\u{85}B"] {
        let mapped = mapping(&[(party, "Wages", None)]);
        let narration = &built(party, &mapped, "ACC xx1234").unwrap().proposals[0].narration;
        assert!(
            narration.ends_with(" | Statement party: A B"),
            "{narration}"
        );
    }
}

#[test]
fn the_narration_limit_is_exact_and_counts_the_party_segment() {
    // The narration without the segment, for a one-character party, fixes the room left.
    let one = built("P", &mapping(&[("P", "Wages", None)]), "ACC xx1234").unwrap();
    let narration = &one.proposals[0].narration;
    let segment = format!(" | {STATEMENT_PARTY_LABEL} P");
    assert!(narration.ends_with(&segment), "{narration}");
    let room = MAX_NARRATION_CHARS - narration.chars().count();
    let exact = "P".repeat(1 + room);
    let at_limit = built(&exact, &mapping(&[(&exact, "Wages", None)]), "ACC xx1234").unwrap();
    assert_eq!(
        at_limit.proposals[0].narration.chars().count(),
        MAX_NARRATION_CHARS
    );
    let over = "P".repeat(2 + room);
    let refusal = built(&over, &mapping(&[(&over, "Wages", None)]), "ACC xx1234").unwrap_err();
    assert_eq!(refusal.category, "party_not_admissible");
}

#[test]
fn a_narration_over_the_limit_before_any_segment_is_the_narrations_refusal() {
    let label = "T".repeat(MAX_NARRATION_CHARS);
    let refusal = built("ALPHA", &mapping(&[("ALPHA", "Wages", None)]), &label).unwrap_err();
    assert_eq!(refusal.category, "narration_not_admissible");
}

#[test]
fn a_party_holding_the_delimiter_is_refused_before_the_narration_is_judged() {
    // The label carries a control character, so the narration is not admissible either; the
    // party is judged first, as the app does.
    let piped = mapping(&[("A|B", "Wages", None)]);
    let refusal = built("A|B", &piped, "AC\u{7}C").unwrap_err();
    assert_eq!(refusal.category, "party_not_admissible");
}

#[test]
fn a_party_holding_the_delimiter_is_not_refused_where_no_segment_is_written() {
    // Sent to suspense (no mapping row) it is written where the ledger would be, once.
    let to_suspense = built("A|B", &mapping(&[]), "ACC xx1234").unwrap();
    assert!(to_suspense.records[0].suspense);
    assert!(to_suspense.proposals[0].narration.contains("A|B"));
    // Skipped, nothing is written at all.
    let skipped = built(
        "A|B",
        &mapping(&[("A|B", "Other Bank", Some("skip"))]),
        "ACC xx1234",
    )
    .unwrap();
    assert_eq!(skipped.records[0].disposition, Disposition::Skipped);
    assert!(skipped.proposals.is_empty());
}

#[test]
fn the_marker_in_a_party_sent_to_suspense_is_refused_because_it_is_written_there() {
    let refusal = built("X [BRIDGE:1] Y", &mapping(&[]), "ACC xx1234").unwrap_err();
    assert_eq!(refusal.category, "narration_not_admissible");
}

#[test]
fn a_long_party_is_refused_when_mapped_and_fits_when_sent_to_suspense() {
    let long_party = "P".repeat(MAX_NARRATION_CHARS - 80);
    let mapped = mapping(&[(&long_party, "A Ledger With A Longer Name", None)]);
    let refusal = built(&long_party, &mapped, "ACC xx1234").unwrap_err();
    assert_eq!(refusal.category, "party_not_admissible");
    // Written once, where the ledger would be, it fits.
    let unmapped = built(&long_party, &mapping(&[]), "ACC xx1234").unwrap();
    assert!(unmapped.records[0].suspense);
    assert!(unmapped.proposals[0].narration.chars().count() <= MAX_NARRATION_CHARS);
}

#[test]
fn the_transaction_id_follows_the_statements_account_not_the_operators_label() {
    let party = "MAPLE QUILL";
    let mapped = mapping(&[(party, "Wages", None)]);
    let ids: Vec<String> = ["ACC xx1234", "main account", "ACC xx1234 (old)"]
        .into_iter()
        .map(|label| {
            built(party, &mapped, label).unwrap().records[0]
                .bridge_txn_id
                .clone()
        })
        .collect();
    assert_eq!(ids[0], ids[1]);
    assert_eq!(ids[0], ids[2]);
    // The statement's own account number does move it.
    let mut other = options("ACC xx1234");
    other.account_number = "0000005678";
    let elsewhere = build(&[debit_to(party)], Bank::Sbi, &mapped, &other).unwrap();
    assert_ne!(ids[0], elsewhere.records[0].bridge_txn_id);
}

#[test]
fn a_bank_or_suspense_ledger_that_is_empty_or_only_whitespace_is_refused() {
    for blank in ["", "  ", "\t", " \t "] {
        let mut bank_blank = options("ACC xx1234");
        bank_blank.bank_ledger = blank;
        let refusal = build(
            &[debit_to("MAPLE QUILL")],
            Bank::Sbi,
            &mapping(&[]),
            &bank_blank,
        )
        .unwrap_err();
        assert_eq!(refusal.category, "blank_ledger", "bank ledger {blank:?}");
        let mut suspense_blank = options("ACC xx1234");
        suspense_blank.suspense_ledger = blank;
        let refusal = build(
            &[debit_to("MAPLE QUILL")],
            Bank::Sbi,
            &mapping(&[]),
            &suspense_blank,
        )
        .unwrap_err();
        assert_eq!(
            refusal.category, "blank_ledger",
            "suspense ledger {blank:?}"
        );
    }
}

#[test]
fn the_labels_the_parser_gives_are_these_and_only_a_printed_name_becomes_a_party() {
    assert_eq!(PARSER_SENTINELS, ["UNRESOLVED", "UNNAMED"]);
    assert_eq!(
        PARSER_CATEGORIES,
        ["EMI", "DEBIT CARD FEE", "BANK CHARGES", "CARD ANNUAL FEE"]
    );
    assert_eq!((UNRESOLVED, UNNAMED), ("UNRESOLVED", "UNNAMED"));
    assert_eq!(CASH_WITHDRAWAL, "ATM CASH WITHDRAWAL");
    assert_ne!(CASH_DEPOSIT, CASH_WITHDRAWAL);
    // Category labels are compared exactly: the mixed-case spelling is a printed name.
    let mapped = mapping(&[("Bank Charges", "Wages", None)]);
    let written = &built("Bank Charges", &mapped, "ACC xx1234")
        .unwrap()
        .proposals[0]
        .narration;
    assert!(
        written.ends_with(&format!(" | {STATEMENT_PARTY_LABEL} Bank Charges")),
        "{written}"
    );
}
