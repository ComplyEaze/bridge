use std::io::Cursor;

use bridge_tally_core::{ExactDecimal, TallyDate};
use zip::ZipArchive;

use super::*;
use crate::reports::party_ledger_master::{
    build_party_ledger_master_workbook, BalanceDateBasis, PartyLedgerMasterRow,
    PartyLedgerMasterSource,
};
use bridge_tally_protocol::mailing_details::{
    MailingDetailsDefect, MailingDetailsEntry, MailingDetailsHistory,
};
use bridge_tally_protocol::{PartyLedgerMasterFieldObservation, PartyLedgerMasterFields};

fn source_with_precision(decimal_places: u8) -> PartyLedgerMasterSource {
    PartyLedgerMasterSource {
        company: "Synthetic Books".to_string(),
        company_guid: "company-guid".to_string(),
        currency_assertion: OutstandingsCurrencyAssertion::Inr,
        currency_decimal_places: decimal_places,
        from: TallyDate::parse("20260401").unwrap(),
        to: TallyDate::parse("20260731").unwrap(),
        balance_date_basis: BalanceDateBasis::HostToday(TallyDate::parse("20260731").unwrap()),
        last_voucher_date: TallyDate::parse("20260731").unwrap(),
        rows: vec![PartyLedgerMasterRow {
            name: "Three decimal customer".to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned("Sundry Debtors".to_string()),
            party_gstin: PartyLedgerMasterFieldObservation::NotObserved,
            fields: PartyLedgerMasterFields::default(),
            guid: "ledger-guid".to_string(),
            master_id: "7".to_string(),
            alter_id: "9".to_string(),
            opening_balance: ExactDecimal::parse("-1.234".to_string()).unwrap(),
            closing_balance: Some(ExactDecimal::parse("1.234".to_string()).unwrap()),
        }],
        request_sha256: "0".repeat(64),
        master_response_sha256: "a".repeat(64),
        balance_response_sha256: "b".repeat(64),
        group_response_sha256: "c".repeat(64),
        master_response_bytes: 100,
        balance_response_bytes: 200,
        group_response_bytes: 300,
        groups: vec![],
        foreign_currency_ledgers_excluded: Vec::new(),
        mixed_currency_ledgers_excluded: Vec::new(),
        count_cross_check: None,
    }
}

#[test]
fn three_decimal_currency_renders_1234_without_a_two_decimal_format() {
    let workbook = build_party_ledger_master_workbook(source_with_precision(3)).unwrap();
    let bytes = render_party_ledger_master_xlsx(&workbook).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut sheet = String::new();
    std::io::Read::read_to_string(
        &mut archive.by_name("xl/worksheets/sheet1.xml").unwrap(),
        &mut sheet,
    )
    .unwrap();
    let mut styles = String::new();
    std::io::Read::read_to_string(&mut archive.by_name("xl/styles.xml").unwrap(), &mut styles)
        .unwrap();

    assert!(sheet.contains(">1.234</v>"));
    assert!(!sheet.contains(">1.23</v>"));
    assert!(styles.contains("formatCode=\"##,##,##0.000\""));
    assert!(!styles.contains("formatCode=\"##,##,##0.00\""));
}

#[test]
fn unrenderable_currency_precision_withholds_the_workbook() {
    assert!(matches!(
        build_party_ledger_master_workbook(source_with_precision(16)),
        Err(super::super::party_ledger_master::PartyLedgerMasterError::UnrenderableCurrencyPrecision(16))
    ));
}

#[test]
fn renders_evidence_currency_and_returned_fields_in_the_workbook() {
    let workbook = build_party_ledger_master_workbook(PartyLedgerMasterSource {
        company: "Synthetic Books".to_string(),
        company_guid: "company-guid".to_string(),
        currency_assertion: OutstandingsCurrencyAssertion::Inr,
        currency_decimal_places: 2,
        from: TallyDate::parse("20260401").unwrap(),
        to: TallyDate::parse("20260731").unwrap(),
        balance_date_basis: BalanceDateBasis::HostToday(TallyDate::parse("20260731").unwrap()),
        last_voucher_date: TallyDate::parse("20260731").unwrap(),
        rows: vec![PartyLedgerMasterRow {
            name: "Customer".to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned("Sundry Debtors".to_string()),
            party_gstin: PartyLedgerMasterFieldObservation::Returned("29ABCDE1234F1Z5".to_string()),
            fields: PartyLedgerMasterFields {
                email: PartyLedgerMasterFieldObservation::Returned(
                    "synthetic@example.invalid".to_string(),
                ),
                ..PartyLedgerMasterFields::default()
            },
            guid: "ledger-guid".to_string(),
            master_id: "7".to_string(),
            alter_id: "9".to_string(),
            opening_balance: ExactDecimal::parse("-100.00".to_string()).unwrap(),
            closing_balance: Some(ExactDecimal::parse("125.00".to_string()).unwrap()),
        }],
        request_sha256: "0".repeat(64),
        master_response_sha256: "a".repeat(64),
        balance_response_sha256: "b".repeat(64),
        group_response_sha256: "c".repeat(64),
        master_response_bytes: 100,
        balance_response_bytes: 200,
        group_response_bytes: 300,
        groups: vec![],
        foreign_currency_ledgers_excluded: Vec::new(),
        mixed_currency_ledgers_excluded: Vec::new(),
        count_cross_check: None,
    })
    .unwrap();
    let bytes = render_party_ledger_master_xlsx(&workbook).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut text = String::new();
    for name in [
        "xl/worksheets/sheet1.xml",
        "xl/worksheets/sheet2.xml",
        "xl/sharedStrings.xml",
    ] {
        std::io::Read::read_to_string(&mut archive.by_name(name).unwrap(), &mut text).unwrap();
    }
    assert!(text.contains("Master response SHA-256"));
    assert!(text.contains("Not observed fields"));
    assert!(!text.contains("Unavailable fields"));
    assert!(text.contains(
        "“Not observed” means this Tally response did not return the requested field; it does not establish whether that field is unset in this book or unavailable in this Tally build. Bridge never manufactures master data."
    ));
    assert!(text.contains("Not observed"));
    assert!(!text.contains("was unset in this book"));
    assert!(text.contains("Income Tax number (as returned)"));
    assert!(text.contains("synthetic@example.invalid"));
    assert!(text.contains("Currency"));
    assert!(text.contains("INR"));
    assert!(text.contains(
        "Read period: 20260401 to 20260731. No prior-year values were requested or inferred."
    ));
    assert!(!text.contains("One year read"));
    assert!(text.contains("EXCLUSION LIST (loud)"));
}

#[test]
fn normally_signed_sundry_debtor_renders_as_a_group_subtotal_not_trade_receivables() {
    let workbook = build_party_ledger_master_workbook(PartyLedgerMasterSource {
        company: "Synthetic Books".to_string(),
        company_guid: "company-guid".to_string(),
        currency_assertion: OutstandingsCurrencyAssertion::Inr,
        currency_decimal_places: 2,
        from: TallyDate::parse("20260401").unwrap(),
        to: TallyDate::parse("20260731").unwrap(),
        balance_date_basis: BalanceDateBasis::HostToday(TallyDate::parse("20260731").unwrap()),
        last_voucher_date: TallyDate::parse("20260731").unwrap(),
        rows: vec![PartyLedgerMasterRow {
            name: "Customer balance".to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned("Sundry Debtors".to_string()),
            party_gstin: PartyLedgerMasterFieldObservation::NotObserved,
            fields: PartyLedgerMasterFields::default(),
            guid: "ledger-guid".to_string(),
            master_id: "7".to_string(),
            alter_id: "9".to_string(),
            opening_balance: ExactDecimal::parse("-300.00".to_string()).unwrap(),
            closing_balance: Some(ExactDecimal::parse("-300.00".to_string()).unwrap()),
        }],
        request_sha256: "0".repeat(64),
        master_response_sha256: "a".repeat(64),
        balance_response_sha256: "b".repeat(64),
        group_response_sha256: "c".repeat(64),
        master_response_bytes: 100,
        balance_response_bytes: 200,
        group_response_bytes: 300,
        groups: vec![bridge_tally_protocol::TallyNamedMaster {
            name: "Sundry Debtors".to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned("Primary".to_string()),
            reserved_name: Some("Sundry Debtors".to_string()),
        }],
        foreign_currency_ledgers_excluded: Vec::new(),
        mixed_currency_ledgers_excluded: Vec::new(),
        count_cross_check: None,
    })
    .unwrap();

    let bytes = render_party_ledger_master_xlsx(&workbook).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut text = String::new();
    for name in [
        "xl/worksheets/sheet1.xml",
        "xl/worksheets/sheet2.xml",
        "xl/sharedStrings.xml",
    ] {
        std::io::Read::read_to_string(&mut archive.by_name(name).unwrap(), &mut text).unwrap();
    }

    assert!(text.contains("Sundry Debtors group subtotal"));
    assert!(text.contains("-300.00"));
    assert!(!text.contains("Trade receivables"));
}

#[test]
fn gstin_not_observed_is_labeled_while_an_explicit_empty_gstin_is_not() {
    let rows = vec![
        PartyLedgerMasterRow {
            name: "GSTIN not observed".to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned("Sundry Debtors".to_string()),
            party_gstin: PartyLedgerMasterFieldObservation::NotObserved,
            fields: PartyLedgerMasterFields::default(),
            guid: "ledger-guid-1".to_string(),
            master_id: "7".to_string(),
            alter_id: "9".to_string(),
            opening_balance: ExactDecimal::parse("-100.00".to_string()).unwrap(),
            closing_balance: Some(ExactDecimal::parse("125.00".to_string()).unwrap()),
        },
        PartyLedgerMasterRow {
            name: "GSTIN returned empty".to_string(),
            parent: PartyLedgerMasterFieldObservation::Returned("Sundry Debtors".to_string()),
            party_gstin: PartyLedgerMasterFieldObservation::Returned(String::new()),
            fields: PartyLedgerMasterFields::default(),
            guid: "ledger-guid-2".to_string(),
            master_id: "8".to_string(),
            alter_id: "10".to_string(),
            opening_balance: ExactDecimal::parse("-200.00".to_string()).unwrap(),
            closing_balance: Some(ExactDecimal::parse("250.00".to_string()).unwrap()),
        },
    ];
    let workbook = build_party_ledger_master_workbook(PartyLedgerMasterSource {
        company: "Synthetic Books".to_string(),
        company_guid: "company-guid".to_string(),
        currency_assertion: OutstandingsCurrencyAssertion::Inr,
        currency_decimal_places: 2,
        from: TallyDate::parse("20260401").unwrap(),
        to: TallyDate::parse("20260731").unwrap(),
        balance_date_basis: BalanceDateBasis::HostToday(TallyDate::parse("20260731").unwrap()),
        last_voucher_date: TallyDate::parse("20260731").unwrap(),
        rows,
        request_sha256: "0".repeat(64),
        master_response_sha256: "a".repeat(64),
        balance_response_sha256: "b".repeat(64),
        group_response_sha256: "c".repeat(64),
        master_response_bytes: 100,
        balance_response_bytes: 200,
        group_response_bytes: 300,
        groups: vec![],
        foreign_currency_ledgers_excluded: Vec::new(),
        mixed_currency_ledgers_excluded: Vec::new(),
        count_cross_check: None,
    })
    .unwrap();
    let bytes = render_party_ledger_master_xlsx(&workbook).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut sheet = String::new();
    std::io::Read::read_to_string(
        &mut archive.by_name("xl/worksheets/sheet1.xml").unwrap(),
        &mut sheet,
    )
    .unwrap();
    let mut shared_strings = String::new();
    std::io::Read::read_to_string(
        &mut archive.by_name("xl/sharedStrings.xml").unwrap(),
        &mut shared_strings,
    )
    .unwrap();
    let not_observed_index = shared_strings
        .split("<si>")
        .skip(1)
        .position(|entry| entry.contains("<t>Not observed</t>"))
        .expect("workbook contains the disclosure label");
    let not_observed_cell = format!(r#"r="C15" t="s"><v>{not_observed_index}</v>"#);
    let explicitly_empty_cell = format!(r#"r="C16" t="s"><v>{not_observed_index}</v>"#);

    assert!(
        sheet.contains(&not_observed_cell),
        "an omitted GSTIN must render the observation label"
    );
    assert!(
        !sheet.contains(&explicitly_empty_cell),
        "an explicitly returned empty GSTIN must not be mislabeled as not observed"
    );
}

fn worksheet_with_parent(parent: PartyLedgerMasterFieldObservation) -> (String, usize) {
    let workbook = build_party_ledger_master_workbook(PartyLedgerMasterSource {
        company: "Synthetic Books".to_string(),
        company_guid: "company-guid".to_string(),
        currency_assertion: OutstandingsCurrencyAssertion::Inr,
        currency_decimal_places: 2,
        from: TallyDate::parse("20260401").unwrap(),
        to: TallyDate::parse("20260731").unwrap(),
        balance_date_basis: BalanceDateBasis::HostToday(TallyDate::parse("20260731").unwrap()),
        last_voucher_date: TallyDate::parse("20260731").unwrap(),
        rows: vec![PartyLedgerMasterRow {
            name: "Parent observation".to_string(),
            parent,
            party_gstin: PartyLedgerMasterFieldObservation::Returned(String::new()),
            fields: PartyLedgerMasterFields::default(),
            guid: "ledger-guid-parent".to_string(),
            master_id: "11".to_string(),
            alter_id: "12".to_string(),
            opening_balance: ExactDecimal::parse("-100.00".to_string()).unwrap(),
            closing_balance: Some(ExactDecimal::parse("125.00".to_string()).unwrap()),
        }],
        request_sha256: "0".repeat(64),
        master_response_sha256: "a".repeat(64),
        balance_response_sha256: "b".repeat(64),
        group_response_sha256: "c".repeat(64),
        master_response_bytes: 100,
        balance_response_bytes: 200,
        group_response_bytes: 300,
        groups: vec![],
        foreign_currency_ledgers_excluded: Vec::new(),
        mixed_currency_ledgers_excluded: Vec::new(),
        count_cross_check: None,
    })
    .unwrap();
    let bytes = render_party_ledger_master_xlsx(&workbook).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut sheet = String::new();
    std::io::Read::read_to_string(
        &mut archive.by_name("xl/worksheets/sheet1.xml").unwrap(),
        &mut sheet,
    )
    .unwrap();
    let mut shared_strings = String::new();
    std::io::Read::read_to_string(
        &mut archive.by_name("xl/sharedStrings.xml").unwrap(),
        &mut shared_strings,
    )
    .unwrap();
    let not_observed_index = shared_strings
        .split("<si>")
        .skip(1)
        .position(|entry| entry.contains("<t>Not observed</t>"))
        .expect("workbook contains the disclosure label");
    (sheet, not_observed_index)
}

#[test]
fn parent_not_observed_is_labeled_in_the_group_cell() {
    let (sheet, not_observed_index) =
        worksheet_with_parent(PartyLedgerMasterFieldObservation::NotObserved);
    let group_cell = format!(r#"r="B15" t="s"><v>{not_observed_index}</v>"#);

    assert!(
        sheet.contains(&group_cell),
        "an omitted parent must render Not observed instead of a blank Group cell"
    );
}

#[test]
fn explicitly_empty_parent_renders_an_empty_group_cell() {
    let (sheet, not_observed_index) =
        worksheet_with_parent(PartyLedgerMasterFieldObservation::Returned(String::new()));
    let group_cell = format!(r#"r="B15" t="s"><v>{not_observed_index}</v>"#);

    assert!(
        !sheet.contains(&group_cell),
        "an explicitly empty parent must remain empty rather than being labeled Not observed"
    );
    let cell_has_value = sheet
        .find(r#"<c r="B15""#)
        .and_then(|start| {
            sheet[start..]
                .find("</c>")
                .map(|end| &sheet[start..start + end])
        })
        .is_some_and(|cell| cell.contains("<v>"));
    assert!(
        !cell_has_value,
        "an explicitly empty parent must not write a Group-cell value"
    );
}

fn workbook_text(source: PartyLedgerMasterSource) -> String {
    let workbook = build_party_ledger_master_workbook(source).unwrap();
    let bytes = render_party_ledger_master_xlsx(&workbook).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut text = String::new();
    for name in [
        "xl/worksheets/sheet1.xml",
        "xl/worksheets/sheet2.xml",
        "xl/sharedStrings.xml",
    ] {
        std::io::Read::read_to_string(&mut archive.by_name(name).unwrap(), &mut text).unwrap();
    }
    text
}

#[test]
fn a_later_dated_voucher_is_said_beside_the_balance_date_and_in_the_read_period() {
    let mut source = source_with_precision(2);
    source.to = TallyDate::parse("20261001").unwrap();
    source.last_voucher_date = TallyDate::parse("50261231").unwrap();
    let text = workbook_text(source);
    let note = "Vouchers dated after 20261001 (the book's last voucher date is 50261231) are not included in these balances. Review them in Tally before relying on this workbook. The balances end at the first month boundary (the 1st, 2nd or 31st of a month) on or after this computer's date, or on or after the start of the books if that is later; if this computer's date is wrong, correct it and export again.";
    // Once on the ledger sheet (beside the date), once in the Schedule III read period.
    assert_eq!(text.matches(note).count(), 2, "{text}");
    assert!(text.contains(&format!(
        "Read period: 20260401 to 20261001. No prior-year values were requested or inferred. {note}"
    )));
}

#[test]
fn no_later_dated_voucher_adds_nothing_to_the_workbook() {
    for last in ["20260731", "20260630"] {
        let mut source = source_with_precision(2);
        source.last_voucher_date = TallyDate::parse(last).unwrap();
        let text = workbook_text(source);
        assert!(!text.contains("Vouchers dated after"), "{last}");
        assert!(text.contains(
            "Read period: 20260401 to 20260731. No prior-year values were requested or inferred."
        ));
        assert!(
            !text.contains("No prior-year values were requested or inferred. "),
            "{last}"
        );
    }
}

/// A fingerprint of every part of the package except `docProps/core.xml`,
/// which Excel's writer stamps with the creation time: each part's name,
/// length and decompressed bytes, in name order.
fn package_fingerprint(source: PartyLedgerMasterSource) -> String {
    use sha2::{Digest, Sha256};
    let workbook = build_party_ledger_master_workbook(source).unwrap();
    let bytes = render_party_ledger_master_xlsx(&workbook).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut parts = Vec::new();
    for index in 0..archive.len() {
        let mut part = archive.by_index(index).unwrap();
        if part.name() == "docProps/core.xml" {
            continue;
        }
        let mut contents = Vec::new();
        std::io::Read::read_to_end(&mut part, &mut contents).unwrap();
        parts.push((part.name().to_owned(), contents));
    }
    parts.sort();
    let mut hasher = Sha256::new();
    for (name, contents) in &parts {
        hasher.update(name.as_bytes());
        hasher.update((contents.len() as u64).to_le_bytes());
        hasher.update(contents);
    }
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The package of the host-date workbook. It was taken from master before #903
/// changed anything (with no date typed, the bytes must not change), and was
/// taken again when the five mailing columns and the Covered text changed the
/// package (#1490); nothing since then may change it.
const HOST_DATE_PACKAGE_WITHOUT_NOTE: &str =
    "ee2e97715ac355ac1f1c1a3d1f83c4a48037d3fa24d10385bc169078abcb73e9";
const HOST_DATE_PACKAGE_WITH_LATER_DATED_NOTE: &str =
    "db95374634b59808e2f11cff3d59de437b9064d205abf0df122f30d194fc7de1";

#[test]
fn the_host_date_workbook_is_byte_for_byte_what_it_was_before_a_date_could_be_typed() {
    assert_eq!(
        package_fingerprint(source_with_precision(2)),
        HOST_DATE_PACKAGE_WITHOUT_NOTE
    );
    let mut later = source_with_precision(2);
    later.to = TallyDate::parse("20261001").unwrap();
    later.last_voucher_date = TallyDate::parse("50261231").unwrap();
    assert_eq!(
        package_fingerprint(later),
        HOST_DATE_PACKAGE_WITH_LATER_DATED_NOTE
    );
}

#[test]
fn a_moved_chosen_date_is_said_beside_the_balance_date_and_in_the_read_period() {
    let mut source = source_with_precision(2);
    source.to = TallyDate::parse("20260831").unwrap();
    source.last_voucher_date = TallyDate::parse("20260831").unwrap();
    source.balance_date_basis = BalanceDateBasis::Requested(TallyDate::parse("20260815").unwrap());
    let moved = source.date_moved_note().expect("a moved date is noted");
    let text = workbook_text(source);
    // Once on the ledger sheet (beside the date), once in the Schedule III read period.
    assert_eq!(text.matches(&moved).count(), 2, "{text}");
    assert!(text.contains(&format!(
        "Read period: 20260401 to 20260831. No prior-year values were requested or inferred. {moved}"
    )));
}

#[test]
fn a_chosen_date_that_is_a_boundary_adds_no_moved_note_to_the_workbook() {
    let mut source = source_with_precision(2);
    source.balance_date_basis = BalanceDateBasis::Requested(TallyDate::parse("20260731").unwrap());
    assert_eq!(source.date_moved_note(), None);
    let text = workbook_text(source);
    assert!(!text.contains("is not a date Tally accepts"), "{text}");
    assert!(text.contains(
        "Read period: 20260401 to 20260731. No prior-year values were requested or inferred."
    ));
}

fn mailing_row(name: &str, id: &str, history: MailingDetailsHistory) -> PartyLedgerMasterRow {
    PartyLedgerMasterRow {
        name: name.to_string(),
        parent: PartyLedgerMasterFieldObservation::Returned("Sundry Debtors".to_string()),
        party_gstin: PartyLedgerMasterFieldObservation::NotObserved,
        fields: PartyLedgerMasterFields {
            mailing_details: history,
            ..PartyLedgerMasterFields::default()
        },
        guid: format!("guid-{id}"),
        master_id: id.to_string(),
        alter_id: id.to_string(),
        opening_balance: ExactDecimal::parse("0.00".to_string()).unwrap(),
        closing_balance: Some(ExactDecimal::parse("0.00".to_string()).unwrap()),
    }
}

fn mailing_entry(from: &str, pin: &str) -> MailingDetailsEntry {
    MailingDetailsEntry {
        applicable_from: TallyDate::parse(from).unwrap(),
        mailing_name: Some("Lab Party".to_string()),
        address_lines: vec!["1 Test Street".to_string(), "Test Area".to_string()],
        state: Some("Karnataka".to_string()),
        country: Some("India".to_string()),
        pin_code: Some(pin.to_string()),
        other_content_skipped: false,
    }
}

/// The text of one cell of the ledger sheet, or `None` when it holds nothing.
fn sheet_cell(source: PartyLedgerMasterSource, cell: &str) -> Option<String> {
    let workbook = build_party_ledger_master_workbook(source).unwrap();
    let bytes = render_party_ledger_master_xlsx(&workbook).unwrap();
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut sheet = String::new();
    std::io::Read::read_to_string(
        &mut archive.by_name("xl/worksheets/sheet1.xml").unwrap(),
        &mut sheet,
    )
    .unwrap();
    let mut shared = String::new();
    std::io::Read::read_to_string(
        &mut archive.by_name("xl/sharedStrings.xml").unwrap(),
        &mut shared,
    )
    .unwrap();
    let start = sheet.find(&format!(r#"<c r="{cell}""#))?;
    let element = &sheet[start..start + sheet[start..].find("</c>")?];
    let index: usize = element
        .split("<v>")
        .nth(1)?
        .split("</v>")
        .next()?
        .parse()
        .ok()?;
    let entry = shared.split("<si>").skip(1).nth(index)?;
    let text = entry.split('>').nth(1)?.split('<').next()?;
    Some(text.to_string())
}

#[test]
fn the_mailing_columns_are_appended_after_the_existing_ones_and_named() {
    let source = source_with_precision(2);
    for (cell, label) in [
        ("W14", "Closing balance (exact text)"),
        (
            "X14",
            "Mailing address in force (as of Balances-as-of date)",
        ),
        ("Y14", "Mailing PIN code in force"),
        ("Z14", "Mailing state in force"),
        ("AA14", "Mailing details effective from"),
        ("AB14", "Mailing details status"),
    ] {
        assert_eq!(
            sheet_cell(source.clone(), cell).as_deref(),
            Some(label),
            "{cell}"
        );
    }
}

#[test]
fn a_partys_mailing_entry_in_force_on_the_balances_date_fills_the_five_cells() {
    let mut source = source_with_precision(2);
    source.rows = vec![
        mailing_row(
            "Lab Party",
            "1",
            MailingDetailsHistory::Entries {
                entries: vec![
                    mailing_entry("20260401", "560001"),
                    mailing_entry("20261001", "560099"),
                ],
            },
        ),
        mailing_row("Other", "2", MailingDetailsHistory::NotObserved),
    ];
    let cell = |cell: &str| sheet_cell(source.clone(), cell);
    // `Balances as of` is 20260731: the entry dated 20260401 is in force, the one
    // dated 20261001 is not yet.
    assert_eq!(cell("X15").as_deref(), Some("1 Test Street, Test Area"));
    assert_eq!(cell("Y15").as_deref(), Some("560001"));
    assert_eq!(cell("Z15").as_deref(), Some("Karnataka"));
    assert_eq!(cell("AA15").as_deref(), Some("20260401"));
    assert_eq!(cell("AB15").as_deref(), Some("in force"));
    // A ledger whose response did not carry the list says so and leaves the values blank.
    for blank in ["X16", "Y16", "Z16", "AA16"] {
        assert_eq!(cell(blank), None, "{blank}");
    }
    assert_eq!(cell("AB16").as_deref(), Some("not observed"));
}

#[test]
fn each_mailing_status_says_why_the_values_are_blank() {
    let status = |history: MailingDetailsHistory, as_of: &str| {
        mailing_cells(&history, &TallyDate::parse(as_of).unwrap())[4].clone()
    };
    let one = MailingDetailsHistory::Entries {
        entries: vec![mailing_entry("20260401", "560001")],
    };
    assert_eq!(status(one.clone(), "20260401"), "in force");
    // Not "no address": the first entry starts later, and the status says when.
    assert_eq!(
        status(one.clone(), "20260331"),
        "none in force on that date; first entry from 20260401"
    );
    let mut skipped = mailing_entry("20260401", "560001");
    skipped.other_content_skipped = true;
    assert_eq!(
        status(
            MailingDetailsHistory::Entries {
                entries: vec![skipped]
            },
            "20260401"
        ),
        "in force; some content of the entry was not read"
    );
    assert_eq!(
        status(
            MailingDetailsHistory::Entries { entries: vec![] },
            "20260731"
        ),
        "no mailing details"
    );
    assert_eq!(
        status(MailingDetailsHistory::NotObserved, "20260731"),
        "not observed"
    );
    for (defect, name) in [
        (MailingDetailsDefect::EntryWithoutDate, "entry_without_date"),
        (MailingDetailsDefect::DateInvalid, "date_invalid"),
        (
            MailingDetailsDefect::EntryRepeatsAField,
            "entry_repeats_a_field",
        ),
        (
            MailingDetailsDefect::ConflictingEntriesOnOneDate,
            "conflicting_entries_on_one_date",
        ),
    ] {
        assert_eq!(
            status(MailingDetailsHistory::Unreadable { defect }, "20260731"),
            format!("unreadable: {name}")
        );
    }
}

#[test]
fn an_entry_missing_its_pin_or_state_leaves_those_cells_blank_and_the_rest_filled() {
    let mut entry = mailing_entry("20260401", "560001");
    entry.pin_code = None;
    entry.state = None;
    entry.address_lines = vec!["Only line".to_string()];
    let cells = mailing_cells(
        &MailingDetailsHistory::Entries {
            entries: vec![entry],
        },
        &TallyDate::parse("20260731").unwrap(),
    );
    assert_eq!(cells, ["Only line", "", "", "20260401", "in force"]);
}

#[test]
fn the_covered_text_names_the_dated_mailing_details() {
    let text = workbook_text(source_with_precision(2));
    assert!(text.contains(
        "the dated mailing details (address, PIN code, state) in force on the Balances-as-of date"
    ));
    // The older flat columns can read empty for a party that holds an address: say so.
    assert!(text.contains("read the Mailing columns at the right"));
}
