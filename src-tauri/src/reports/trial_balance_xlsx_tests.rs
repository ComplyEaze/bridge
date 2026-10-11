use std::{
    collections::BTreeMap,
    io::{Cursor, Read},
};

use bridge_tally_core::TallyDate;
use bridge_tally_protocol::{
    native_outstandings::CompanyCurrency, native_trial_balance::parse_native_trial_balance,
};
use quick_xml::{events::Event, Reader};
use zip::ZipArchive;

use super::*;

pub(crate) fn captured_read() -> TrialBalanceRead {
    let report = parse_native_trial_balance(
        include_str!(
            "../../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_known_lab.xml"
        ),
        "eebb9a9f-1679-4468-9e8f-814c729674cb",
    )
    .unwrap();
    TrialBalanceRead {
        company_guid: "eebb9a9f-1679-4468-9e8f-814c729674cb".into(),
        company_name: "Captured Books".into(),
        from: TallyDate::parse("20260401").unwrap(),
        to: TallyDate::parse("20260902").unwrap(),
        currency: CompanyCurrency {
            symbol: "₹".into(),
            mailing_name: "INR".into(),
            currency_count: 1,
            decimal_places: 3,
            is_inr: true,
            names: Vec::new(),
        },
        totals: crate::reports::trial_balance::observed_totals(&report).unwrap(),
        report,
        read_at: "2026-09-08T00:00:00Z".into(),
        evidence: crate::tally::runtime::RuntimeReadEvidence {
            request_sha256: "a".repeat(64),
            response_sha256: "b".repeat(64),
            bytes: 42,
        },
        ledger_scope: Default::default(),
    }
}

fn workbook_text(bytes: &[u8]) -> String {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut text = String::new();
    for name in [
        "xl/worksheets/sheet1.xml",
        "xl/sharedStrings.xml",
        "xl/styles.xml",
    ] {
        archive
            .by_name(name)
            .unwrap()
            .read_to_string(&mut text)
            .unwrap();
    }
    text
}

fn worksheet_xml(bytes: &[u8]) -> String {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("xl/worksheets/sheet1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}

fn worksheet_numeric_cells(bytes: &[u8]) -> BTreeMap<String, String> {
    let xml = worksheet_xml(bytes);

    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut cells = BTreeMap::new();
    let mut cell = None;
    let mut in_value = false;
    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) if tag.name().as_ref() == "c" => {
                cell = tag.attributes().find_map(|attribute| {
                    let attribute = attribute.ok()?;
                    (attribute.key.as_ref() == "r").then(|| attribute.value.into_owned())
                });
            }
            Event::Start(tag) if tag.name().as_ref() == "v" => in_value = true,
            Event::Text(text) if in_value => {
                if let Some(reference) = cell.as_ref() {
                    cells.insert(reference.clone(), text.to_string());
                }
            }
            Event::End(tag) if tag.name().as_ref() == "v" => in_value = false,
            Event::End(tag) if tag.name().as_ref() == "c" => cell = None,
            Event::Eof => break,
            _ => {}
        }
    }
    cells
}

#[test]
fn captured_export_preserves_empty_and_writes_formula_shaped_ledger_as_text() {
    let mut read = captured_read();
    read.report.rows[0].name = "=SUM(A1:A2)".into();
    let bytes = render_trial_balance_xlsx(&read).unwrap();
    let text = workbook_text(&bytes);
    assert!(text.contains("=SUM(A1:A2)"));
    assert!(!text.contains("<f>SUM(A1:A2)</f>"));
    assert!(!text.contains(">-</t>"));
    assert!(text.contains("formatCode=\"##,##,##0.000\""));
}

#[test]
fn captured_export_retains_native_signed_debit_and_credit_cells() {
    let bytes = render_trial_balance_xlsx(&captured_read()).unwrap();
    let cells = worksheet_numeric_cells(&bytes);

    // The second captured ledger has a negative debit and positive credit.
    // These cells must retain the source signs; desktop-only magnitude
    // presentation is not an export transformation.
    assert_eq!(cells.get("E14"), Some(&"-4777".to_string()));
    assert_eq!(cells.get("F14"), Some(&"4500".to_string()));
    assert!(!cells.contains_key("E13"));
    assert!(worksheet_xml(&bytes).contains("<autoFilter ref=\"A12:G18\"/>"));
}

#[test]
fn captured_export_wraps_total_qualification_text() {
    let bytes = render_trial_balance_xlsx(&captured_read()).unwrap();
    let xml = worksheet_xml(&bytes);

    let text = workbook_text(&bytes);
    assert!(text.contains(r#"<alignment wrapText="1"/>"#));
    for cell in ["E19", "F19", "G19"] {
        assert!(
            xml.contains(&format!(r#"<c r="{cell}" s="1" t="s">"#)),
            "expected wrapped cell style for {cell}"
        );
    }
    assert!(text.contains("Observed numeric total:"));
    assert!(text.contains("empty fields:"));
}

#[test]
fn unsafe_excel_precision_withholds_captured_export() {
    let mut read = captured_read();
    read.report.rows[0].opening = NativeTrialBalanceAmount::Present(
        bridge_tally_core::ExactDecimal::parse("9007199254740993").unwrap(),
    );
    assert!(matches!(
        render_trial_balance_xlsx(&read),
        Err(TrialBalanceXlsxError::InvalidAmount(_))
    ));
}

/// As `captured_read`, as a several-currency book's read: two Currency
/// masters with the first read a dollar master, the rows covering the
/// identified rupee base's plain ledgers, and one ledger of each exclusion.
pub(crate) fn several_currency_read() -> TrialBalanceRead {
    let mut read = captured_read();
    read.currency = CompanyCurrency {
        symbol: "$".into(),
        mailing_name: "US Dollar".into(),
        currency_count: 2,
        decimal_places: 2,
        is_inr: false,
        names: Vec::new(),
    };
    read.ledger_scope = crate::tally::runtime::TrialBalanceLedgerScope::BaseCurrencyLedgersOnly {
        base_name: "I\u{20b9}".into(),
        decimal_places: 3,
        foreign: vec![
            bridge_tally_protocol::native_outstandings::ForeignCurrencyLedger {
                ledger: "Dollar Debtor 01".into(),
                currency: "$".into(),
            },
        ],
        mixed: vec!["Rupee Party 01".into()],
    };
    read
}

fn sheet_text(bytes: &[u8], name: &str) -> String {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut text = String::new();
    archive
        .by_name(name)
        .unwrap()
        .read_to_string(&mut text)
        .unwrap();
    text
}

/// bridge#709: a several-currency book's workbook names the identified base
/// as its currency (not the first master read), states that its totals cover
/// the base-currency ledgers only, never labels the opening net a difference,
/// and lists every ledger left out on its own sheet.
#[test]
fn a_several_currency_export_states_its_scope_and_lists_the_ledgers_left_out() {
    let bytes = render_trial_balance_xlsx(&several_currency_read()).unwrap();
    let text = workbook_text(&bytes);
    assert!(text.contains(crate::tally::runtime::BASE_CURRENCY_LEDGERS_ONLY_LIMITATION));
    // The partial read says so first, with its counts (bridge#709).
    assert!(text.contains(
        "Base-currency ledgers only: 1 ledger kept in another currency and 1 base-currency ledger with a value Tally shows in another currency are excluded and listed."
    ));
    assert!(
        text.contains("I\u{20b9} (the base Tally identified; this book keeps 2 Currency masters)")
    );
    assert!(!text.contains("US Dollar"));
    assert!(text.contains("Opening net, base-currency ledgers only (not a balance check)"));
    assert!(!text.contains("Opening difference"));
    let workbook = sheet_text(&bytes, "xl/workbook.xml");
    assert!(workbook.contains("name=\"Excluded ledgers\""), "{workbook}");
    let excluded = sheet_text(&bytes, "xl/worksheets/sheet2.xml");
    // Two ledgers below one header row, written as shared strings.
    assert_eq!(excluded.matches("<row ").count(), 3, "{excluded}");
    for name in [
        "Dollar Debtor 01",
        "Rupee Party 01",
        "Kept in another currency",
    ] {
        assert!(text.contains(name), "{name}");
    }
}

/// A book with one Currency master keeps its workbook exactly as before: one
/// sheet, its own currency, and the opening difference.
#[test]
fn a_single_currency_export_has_one_sheet_and_its_opening_difference() {
    let bytes = render_trial_balance_xlsx(&captured_read()).unwrap();
    let text = workbook_text(&bytes);
    assert!(text.contains("Opening difference (observed)"));
    assert!(!text.contains(crate::tally::runtime::BASE_CURRENCY_LEDGERS_ONLY_LIMITATION));
    let workbook = sheet_text(&bytes, "xl/workbook.xml");
    assert!(!workbook.contains("Excluded ledgers"), "{workbook}");
}
