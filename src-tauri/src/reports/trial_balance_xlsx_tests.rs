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

/// A captured book with a keyed Closing Stock ledger (#2068), held in and taken from the export
/// store as the desktop does.
fn stock_ledger_workbooks() -> Vec<Vec<u8>> {
    crate::reports::trial_balance::tests::stock_ledger_captures()
        .into_iter()
        .map(|(report, company)| {
            let mut read = captured_read();
            read.company_guid = company.into();
            read.currency.decimal_places = 2;
            read.totals = crate::reports::trial_balance::observed_totals(&report).unwrap();
            read.report = report;
            let store = crate::reports::trial_balance_store::TrialBalanceExportStore::default();
            let id = store.insert(read).unwrap();
            render_trial_balance_xlsx(&store.get(&id).unwrap()).unwrap()
        })
        .collect()
}

#[test]
fn a_keyed_closing_stock_ledger_is_marked_in_the_workbook_with_its_gap() {
    for bytes in stock_ledger_workbooks() {
        let text = workbook_text(&bytes);
        assert!(text.contains("Ledgers that do not add up"), "{text}");
        assert!(text.contains(
            "1 ledger(s) carry movements_differ_by: their closing is not their opening plus their debit and credit totals"
        ));
        assert!(
            text.contains(">movements_differ_by (source signed)<"),
            "the column header"
        );
        // The gap is written on the Closing Stock row only, as a number in Tally's sign; the
        // column's other cell is its header, a shared string.
        let xml = worksheet_xml(&bytes);
        let is_text = |cell: &str| {
            let start = xml.find(&format!("<c r=\"{cell}\"")).unwrap();
            xml[start..start + xml[start..].find('>').unwrap()].contains("t=\"s\"")
        };
        let gaps: Vec<_> = worksheet_numeric_cells(&bytes)
            .into_iter()
            .filter(|(cell, _)| cell.starts_with('H') && !is_text(cell))
            .collect();
        assert_eq!(gaps.len(), 1, "{gaps:?}");
        assert_eq!(gaps[0].1, "400");
    }
}

#[test]
fn a_workbook_whose_rows_add_up_has_no_gap_line_or_column() {
    let bytes = render_trial_balance_xlsx(&captured_read()).unwrap();
    let text = workbook_text(&bytes);
    assert!(!text.contains("movements_differ_by"));
    assert!(!text.contains("Ledgers that do not add up"));
    assert!(
        !worksheet_xml(&bytes).contains(" r=\"H"),
        "no eighth column"
    );
}

/// Each element named `name` in the worksheet, with its attributes.
fn worksheet_elements(bytes: &[u8], name: &str) -> Vec<BTreeMap<String, String>> {
    let xml = worksheet_xml(bytes);
    let mut reader = Reader::from_str(&xml);
    let mut elements = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) | Event::Empty(tag) if tag.name().as_ref() == name => {
                elements.push(
                    tag.attributes()
                        .map(|attribute| {
                            let attribute = attribute.unwrap();
                            (
                                attribute.key.as_ref().to_string(),
                                attribute.value.into_owned(),
                            )
                        })
                        .collect(),
                );
            }
            Event::Eof => break,
            _ => {}
        }
    }
    elements
}

/// Whether some `<col>` range covers the 1-based `column` with the amount columns' width.
fn has_amount_width(bytes: &[u8], column: u32) -> bool {
    worksheet_elements(bytes, "col").iter().any(|col| {
        let bound = |key: &str| col[key].parse::<u32>().unwrap();
        (bound("min")..=bound("max")).contains(&column)
            && col["width"].parse::<f64>().unwrap().floor() == 22.0
    })
}

/// The filter and the amount widths reach column H only when a ledger does not add up: on each
/// stock capture the filter spans A to H over the header and every ledger row, and H has the
/// amount width; on the known-lab capture they stop at G.
#[test]
fn the_filter_and_amount_widths_reach_the_gap_column_only_when_it_is_written() {
    for ((report, _), bytes) in crate::reports::trial_balance::tests::stock_ledger_captures()
        .into_iter()
        .zip(stock_ledger_workbooks())
    {
        // Nine lines, the limitation, the gap line and a blank row sit above the header.
        let filter = worksheet_elements(&bytes, "autoFilter");
        assert_eq!(filter.len(), 1);
        assert_eq!(filter[0]["ref"], format!("A13:H{}", 13 + report.rows.len()));
        assert!(has_amount_width(&bytes, 7));
        assert!(has_amount_width(&bytes, 8));
    }
    let bytes = render_trial_balance_xlsx(&captured_read()).unwrap();
    assert_eq!(
        worksheet_elements(&bytes, "autoFilter")[0]["ref"],
        "A12:G18"
    );
    assert!(has_amount_width(&bytes, 7));
    assert!(!has_amount_width(&bytes, 8));
}

/// The gap line's row is tall enough for its text at the limitation row's calibration (45
/// characters, 15 points a line).
#[test]
fn the_gap_line_row_holds_every_line_of_its_text() {
    let lines = crate::reports::trial_balance::MOVEMENTS_DIFFER_LIMITATION
        .replace("{count}", "1")
        .chars()
        .count()
        .div_ceil(45);
    for bytes in stock_ledger_workbooks() {
        // The gap line is the eleventh row: nine lines and the limitation above it.
        let rows = worksheet_elements(&bytes, "row");
        let row = rows.iter().find(|row| row["r"] == "11").unwrap();
        let height: f64 = row["ht"].parse().unwrap();
        assert!(height >= (lines * 15) as f64, "{height} for {lines} lines");
    }
}

#[test]
fn a_wrapped_row_height_covers_its_lines_with_one_to_spare_up_to_excels_limit() {
    assert_eq!(wrapped_row_height(0), 15);
    assert_eq!(wrapped_row_height(45), 30);
    assert_eq!(wrapped_row_height(46), 45);
    assert_eq!(wrapped_row_height(271), 120);
    assert_eq!(wrapped_row_height(1_215), 409);
    assert_eq!(wrapped_row_height(usize::MAX), 409);
}
