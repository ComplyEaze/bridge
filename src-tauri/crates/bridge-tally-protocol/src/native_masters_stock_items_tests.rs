//! The `stock_items` kind of `masters` against the live captures
//! (`tests/fixtures/masters_stock_items_*`). Every negative case is the captured
//! text with one edit; none has a hand-written response. Faults are asserted by
//! their typed variant, never by message text.
use super::*;
use crate::gst_details::{GstDetailsDefect, GstDetailsEntry, GstRate};
use crate::hsn_details::{HsnDetailsDefect, HsnDetailsEntry};

const COMPANY: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";

/// Another company's identifier: the first group of the captured one changed.
fn foreign_company() -> String {
    COMPANY.replacen("3a6bd6e1", "ffffffff", 1)
}
const KIND: NativeMasterKind = NativeMasterKind::StockItems;
/// The item the last capture holds with two dated entries in each list.
const ITEM: &str = "HSN Two Dates Test Item";

fn utf16le(bytes: &[u8]) -> String {
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    String::from_utf16(&units)
        .expect("captured UTF-16LE decodes")
        .trim_start_matches('\u{feff}')
        .to_string()
}

fn baseline() -> String {
    utf16le(include_bytes!(
        "../tests/fixtures/masters_stock_items_shape_lab_baseline_live.utf16le.xml"
    ))
}

fn item_created() -> String {
    utf16le(include_bytes!(
        "../tests/fixtures/masters_stock_items_shape_lab_item_created_live.utf16le.xml"
    ))
}

fn two_dates() -> String {
    utf16le(include_bytes!(
        "../tests/fixtures/masters_stock_items_shape_lab_two_dates_live.utf16le.xml"
    ))
}

fn tieout_b() -> String {
    utf16le(include_bytes!(
        "../tests/fixtures/masters_stock_items_tieout_b_live.utf16le.xml"
    ))
}

const TIEOUT_B_COMPANY: &str = "d64eda55-da7c-4106-a171-3e3cb6cdd331";

fn parse(text: &str) -> Result<NativeMasters, NativeMastersError> {
    parse_native_masters(KIND, text, COMPANY)
}

fn row<'a>(masters: &'a NativeMasters, name: &str) -> &'a NativeMasterRow {
    masters
        .rows
        .iter()
        .find(|row| row.name == name)
        .unwrap_or_else(|| panic!("no row {name}"))
}

fn lists(row: &NativeMasterRow) -> (&HsnDetailsObservation, &GstDetailsObservation) {
    match &row.detail {
        NativeMasterDetail::StockItem { hsn, gst } => (hsn, gst),
        other => panic!("{other:?}"),
    }
}

fn hsn_entries(observation: &HsnDetailsObservation) -> &[HsnDetailsEntry] {
    match observation {
        HsnDetailsObservation::Entries { entries } => entries,
        other => panic!("{other:?}"),
    }
}

fn gst_entries(observation: &GstDetailsObservation) -> &[GstDetailsEntry] {
    match observation {
        GstDetailsObservation::Entries { entries } => entries,
        other => panic!("{other:?}"),
    }
}

/// The capture with the first `from` inside the item's row replaced by `to`.
fn edited(text: &str, from: &str, to: &str) -> String {
    let row = text
        .find(&format!("<STOCKITEM NAME=\"{ITEM}\""))
        .expect("the capture has the item");
    let at = row
        + text[row..]
            .find(from)
            .unwrap_or_else(|| panic!("the item has no {from}"));
    format!("{}{}{}", &text[..at], to, &text[at + from.len()..])
}

/// The item's row of the last capture, parsed after one edit.
fn item_after(
    from: &str,
    to: &str,
) -> Result<(HsnDetailsObservation, GstDetailsObservation), NativeMastersError> {
    let masters = parse(&edited(&two_dates(), from, to))?;
    let (hsn, gst) = lists(row(&masters, ITEM));
    Ok((hsn.clone(), gst.clone()))
}

fn refusal(from: &str, to: &str) -> NativeMastersError {
    parse(&edited(&two_dates(), from, to)).expect_err("the edit must refuse the read")
}

const FIRST_HSN_CODE: &str = "<HSNCODE TYPE=\"String\">28362010</HSNCODE>";
const FIRST_HSN_DATE: &str =
    "<HSNDETAILS.LIST>\r\n      <APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>";
const FIRST_GST_RATE: &str = "<GSTRATE TYPE=\"Number\"> 18</GSTRATE>";
const FIRST_RATE_ROW: &str = "<RATEDETAILS.LIST>";

// The captures.

#[test]
fn the_baseline_capture_has_eleven_items_each_with_both_lists_and_gst_content_on_one() {
    let masters = parse(&baseline()).unwrap();
    assert_eq!(masters.rows.len(), 11);
    let mut with_gst = Vec::new();
    for row in &masters.rows {
        let (hsn, gst) = lists(row);
        // Every item carries both list elements, so none is "not observed"; an
        // empty one is Tally's whitespace-only placeholder.
        assert_eq!(
            *hsn,
            HsnDetailsObservation::Entries { entries: vec![] },
            "{}",
            row.name
        );
        if !gst_entries(gst).is_empty() {
            with_gst.push(row.name.as_str());
        }
    }
    assert_eq!(with_gst, ["Soda Ash Light"]);
}

#[test]
fn a_second_book_shows_the_same_entries_tally_added_itself_on_all_three_of_its_items() {
    let masters = parse_native_masters(KIND, &tieout_b(), TIEOUT_B_COMPANY).unwrap();
    let names: Vec<_> = masters.rows.iter().map(|row| row.name.as_str()).collect();
    assert_eq!(names, ["Lab Widget A", "Lab Widget B", "Lab Widget C"]);
    for row in &masters.rows {
        let (hsn, gst) = lists(row);
        let hsn = hsn_entries(hsn);
        assert_eq!(hsn.len(), 1, "{}", row.name);
        assert_eq!(hsn[0].applicable_from, "20250401");
        assert_eq!(hsn[0].hsn_code, None);
        assert_eq!(hsn[0].source.as_deref(), Some("As per Company/Stock Group"));
        let gst = gst_entries(gst);
        assert_eq!(gst.len(), 1, "{}", row.name);
        assert_eq!(gst[0].source.as_deref(), Some("As per Company/Stock Group"));
        assert_eq!(gst[0].ineligible_itc, Some(true));
        assert_eq!(gst[0].states.len(), 1);
        // The empty slab-rate list Tally prints is not a slab rate.
        assert!(!gst[0].states[0].slab_rates_present);
        assert!(gst[0].states[0]
            .rates
            .iter()
            .all(|rate| rate.rate == GstRate::Absent));
    }
}

#[test]
fn the_item_created_capture_shows_the_entries_tally_added_on_the_first_save() {
    let masters = parse(&item_created()).unwrap();
    assert_eq!(masters.rows.len(), 12);
    let (hsn, gst) = lists(row(&masters, ITEM));
    let hsn = hsn_entries(hsn);
    assert_eq!(hsn.len(), 1);
    assert_eq!(hsn[0].applicable_from, "20250401");
    assert_eq!(hsn[0].hsn_code, None);
    assert_eq!(hsn[0].description, None);
    assert_eq!(hsn[0].source.as_deref(), Some("As per Company/Stock Group"));
    let gst = gst_entries(gst);
    assert_eq!(gst.len(), 1);
    assert_eq!(gst[0].applicable_from, "20250401");
    assert_eq!(gst[0].taxability, None);
    assert_eq!(gst[0].source.as_deref(), Some("As per Company/Stock Group"));
    assert_eq!(gst[0].ineligible_itc, Some(true));
    // No rate on any head: absent, not empty and not zero.
    assert_eq!(gst[0].states.len(), 1);
    assert!(gst[0].states[0]
        .rates
        .iter()
        .all(|rate| rate.rate == GstRate::Absent));
    assert!(!gst[0].other_content_skipped);
}

#[test]
fn the_two_date_capture_reads_two_hsn_and_two_gst_entries_as_sent() {
    let masters = parse(&two_dates()).unwrap();
    assert_eq!(masters.rows.len(), 12);
    let item = row(&masters, ITEM);
    assert_eq!(item.parent.as_deref(), Some("Raw Chemicals"));
    assert_eq!((item.master_id, item.alter_id), (282, 294));
    let (hsn, gst) = lists(item);

    let hsn = hsn_entries(hsn);
    assert_eq!(
        hsn,
        [
            HsnDetailsEntry {
                applicable_from: "20250401".into(),
                hsn_code: Some("28362010".into()),
                description: Some("Disodium Carbonate, Dense".into()),
                source: Some("Specify Details Here".into()),
                other_content_skipped: false,
            },
            HsnDetailsEntry {
                applicable_from: "20250922".into(),
                hsn_code: Some("28362090".into()),
                description: Some("Other".into()),
                source: Some("Specify Details Here".into()),
                other_content_skipped: false,
            },
        ]
    );

    let gst = gst_entries(gst);
    assert_eq!(gst.len(), 2);
    assert_eq!(gst[0].applicable_from, "20250401");
    assert_eq!(gst[1].applicable_from, "20250922");
    // `GSTINELIGIBLEITC` is present on one entry and absent on the other, and
    // neither is filled in. (It was Yes where the screen read `Eligible for
    // Input Tax Credit: No`, so the tool names it `ineligible_itc`.)
    assert_eq!(gst[0].ineligible_itc, Some(true));
    assert_eq!(gst[1].ineligible_itc, None);
    let rates = |entry: &GstDetailsEntry| {
        entry.states[0]
            .rates
            .iter()
            .map(|rate| {
                (
                    rate.duty_head.clone().unwrap(),
                    match &rate.rate {
                        GstRate::Value { raw, decimal } => Some((raw.clone(), decimal.clone())),
                        GstRate::Absent => None,
                        other => panic!("{other:?}"),
                    },
                )
            })
            .collect::<Vec<_>>()
    };
    let value = |raw: &str, decimal: &str| Some((raw.to_string(), decimal.to_string()));
    assert_eq!(
        rates(&gst[0]),
        [
            ("CGST".to_string(), value(" 9", "9")),
            ("SGST/UTGST".to_string(), value(" 9", "9")),
            ("IGST".to_string(), value(" 18", "18")),
            ("Cess".to_string(), None),
            ("State Cess".to_string(), None),
        ]
    );
    assert_eq!(
        rates(&gst[1]),
        [
            ("CGST".to_string(), value(" 2.50", "2.50")),
            ("SGST/UTGST".to_string(), value(" 2.50", "2.50")),
            ("IGST".to_string(), value(" 5", "5")),
            ("Cess".to_string(), None),
            ("State Cess".to_string(), None),
        ]
    );
    // The reserved character arrives as the sanitiser's marker, in the state
    // and in the Cess valuation type.
    assert_eq!(
        gst[0].states[0].state_name.as_deref(),
        Some("\u{fffd}#4; Any")
    );
    assert_eq!(
        gst[0].states[0].rates[3].valuation_type.as_deref(),
        Some("\u{fffd}#4; Not Applicable")
    );
    assert!(!gst[0].states[0].slab_rates_present);
    assert!(gst.iter().all(|entry| !entry.other_content_skipped));
}

#[test]
fn the_names_and_parents_of_the_capture_are_unchanged_by_the_lists() {
    let masters = parse(&two_dates()).unwrap();
    let names = masters
        .rows
        .iter()
        .map(|row| row.name.as_str())
        .collect::<Vec<_>>();
    assert!(names.contains(&"Soda Ash Light"));
    assert!(names.contains(&ITEM));
    for row in &masters.rows {
        assert!(row.parent.is_some(), "{}", row.name);
    }
}

#[test]
fn the_row_bound_is_derived_from_the_measured_largest_item_and_admits_a_stated_mark() {
    // The largest captured row is 4,373 UTF-16 units, so the bound a row is held
    // to (half the worst row in bytes) is well above it.
    let bound_units = masters_worst_row_bytes(KIND) / 2;
    assert_eq!(bound_units, 10_200 + 6 * 128 * 8);
    assert!(bound_units > 4_373 + 3 * 1_627 + 3 * 287);
    // The master mark above which the kind is refused is part of the tool text.
    assert_eq!(
        MASTERS_RESPONSE_BUDGET_BYTES / masters_worst_row_bytes(KIND),
        489
    );
}

// A list absent from a row, empty, or empty of text.

/// The capture with every `tag` element of the item's row removed.
fn without_lists(tag: &str) -> String {
    let text = two_dates();
    let start = text.find(&format!("<STOCKITEM NAME=\"{ITEM}\"")).unwrap();
    let end = start + text[start..].find("</STOCKITEM>").unwrap();
    let (open, close) = (format!("<{tag}>"), format!("</{tag}>"));
    let mut row = text[start..end].to_string();
    while let Some(at) = row.find(&open) {
        let to = at + row[at..].find(&close).unwrap() + close.len();
        row.replace_range(at..to, "");
    }
    format!("{}{row}{}", &text[..start], &text[end..])
}

#[test]
fn a_row_with_no_list_element_at_all_is_not_observed_and_one_list_does_not_imply_the_other() {
    // No HSN element: not observed, with the GST entries intact.
    let masters = parse(&without_lists("HSNDETAILS.LIST")).unwrap();
    let (hsn, gst) = lists(row(&masters, ITEM));
    assert_eq!(*hsn, HsnDetailsObservation::NotObserved);
    assert_eq!(gst_entries(gst).len(), 2);
    // No GST element: not observed, with the HSN entries intact.
    let masters = parse(&without_lists("GSTDETAILS.LIST")).unwrap();
    let (hsn, gst) = lists(row(&masters, ITEM));
    assert_eq!(hsn_entries(hsn).len(), 2);
    assert_eq!(*gst, GstDetailsObservation::NotObserved);
    // The other rows still carry both, as empty placeholders.
    let other = row(&masters, "Soda Ash Light");
    assert_eq!(
        *lists(other).0,
        HsnDetailsObservation::Entries { entries: vec![] }
    );
}

#[test]
fn an_element_with_no_content_is_a_placeholder_and_one_with_empty_children_is_not() {
    // `<GSTDETAILS.LIST/>` and the whitespace-only element are placeholders.
    let (_, gst) = item_after(
        "<GSTDETAILS.LIST>\r\n      <APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM>",
        "<GSTDETAILS.LIST/><GSTDETAILS.LIST>\r\n      <APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM>",
    )
    .unwrap();
    assert_eq!(gst_entries(&gst).len(), 2);
    // An entry whose only child is an empty state row is content without a
    // date, not a placeholder.
    let (_, gst) = item_after(
        "<GSTDETAILS.LIST>\r\n      <APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM>",
        "<GSTDETAILS.LIST><STATEWISEDETAILS.LIST/></GSTDETAILS.LIST><GSTDETAILS.LIST>\r\n      <APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM>",
    )
    .unwrap();
    assert_eq!(
        gst,
        GstDetailsObservation::Unreadable {
            defect: GstDetailsDefect::EntryWithoutDate
        }
    );
}

#[test]
fn a_list_element_with_no_content_is_an_empty_list_and_not_a_missing_one() {
    // Both of the item's HSN entries replaced by one empty element.
    let text = two_dates();
    let at = text.find(&format!("<STOCKITEM NAME=\"{ITEM}\"")).unwrap();
    let first = at + text[at..].find("<HSNDETAILS.LIST>").unwrap();
    let row_end = at + text[at..].find("</STOCKITEM>").unwrap();
    let last_close =
        at + text[at..row_end].rfind("</HSNDETAILS.LIST>").unwrap() + "</HSNDETAILS.LIST>".len();
    let emptied = format!(
        "{}<HSNDETAILS.LIST/>{}",
        &text[..first],
        &text[last_close..]
    );
    let masters = parse(&emptied).unwrap();
    let (hsn, gst) = lists(row(&masters, ITEM));
    assert_eq!(*hsn, HsnDetailsObservation::Entries { entries: vec![] });
    assert_eq!(gst_entries(gst).len(), 2);
}

#[test]
fn an_empty_scalar_is_an_empty_value_and_not_an_absent_one() {
    let (hsn, gst) = item_after(FIRST_HSN_CODE, "<HSNCODE/>").unwrap();
    assert_eq!(hsn_entries(&hsn)[0].hsn_code.as_deref(), Some(""));
    assert_eq!(gst_entries(&gst).len(), 2);
    let (_, gst) = item_after(FIRST_GST_RATE, "<GSTRATE TYPE=\"Number\"></GSTRATE>").unwrap();
    assert_eq!(gst_entries(&gst)[0].states[0].rates[2].rate, GstRate::Empty);
    let (_, gst) = item_after(FIRST_GST_RATE, "<GSTRATE/>").unwrap();
    assert_eq!(gst_entries(&gst)[0].states[0].rates[2].rate, GstRate::Empty);
    let (_, gst) = item_after(FIRST_GST_RATE, "<GSTRATE TYPE=\"Number\"> 0.00</GSTRATE>").unwrap();
    assert_eq!(gst_entries(&gst)[0].states[0].rates[2].rate, GstRate::Zero);
}

#[test]
fn the_lists_may_come_in_either_order_and_after_the_language_list() {
    let text = two_dates();
    let start = text.find(&format!("<STOCKITEM NAME=\"{ITEM}\"")).unwrap();
    let gst_at = start + text[start..].find("<GSTDETAILS.LIST>").unwrap();
    let hsn_at = start + text[start..].find("<HSNDETAILS.LIST>").unwrap();
    let language_at = start + text[start..].find("<LANGUAGENAME.LIST>").unwrap();
    let swapped = format!(
        "{}{}{}{}",
        &text[..gst_at],
        &text[hsn_at..language_at],
        &text[gst_at..hsn_at],
        &text[language_at..]
    );
    assert_eq!(parse(&swapped).unwrap(), parse(&text).unwrap());
}

// The field fails, the read and the row stand.

fn assert_hsn_unreadable(from: &str, to: &str, defect: HsnDetailsDefect) {
    let (hsn, gst) = item_after(from, to).expect("the read stands");
    assert_eq!(hsn, HsnDetailsObservation::Unreadable { defect });
    // The other list is untouched by a fault in this one.
    assert_eq!(gst_entries(&gst).len(), 2);
}

fn assert_gst_unreadable(from: &str, to: &str, defect: GstDetailsDefect) {
    let (hsn, gst) = item_after(from, to).expect("the read stands");
    assert_eq!(gst, GstDetailsObservation::Unreadable { defect });
    assert_eq!(hsn_entries(&hsn).len(), 2);
}

#[test]
fn a_repeated_field_fails_that_list_of_that_item_only() {
    assert_hsn_unreadable(
        FIRST_HSN_CODE,
        &format!("{FIRST_HSN_CODE}{FIRST_HSN_CODE}"),
        HsnDetailsDefect::EntryRepeatsAField,
    );
    assert_gst_unreadable(
        FIRST_GST_RATE,
        &format!("{FIRST_GST_RATE}{FIRST_GST_RATE}"),
        GstDetailsDefect::EntryRepeatsAField,
    );
    // A field repeated in a state row, and in an entry.
    assert_gst_unreadable(
        "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>",
        "<STATENAME TYPE=\"String\">&#4; Any</STATENAME><STATENAME TYPE=\"String\">Goa</STATENAME>",
        GstDetailsDefect::EntryRepeatsAField,
    );
    assert_gst_unreadable(
        "<TAXABILITY TYPE=\"String\">Taxable</TAXABILITY>",
        "<TAXABILITY TYPE=\"String\">Taxable</TAXABILITY><TAXABILITY TYPE=\"String\">Exempt</TAXABILITY>",
        GstDetailsDefect::EntryRepeatsAField,
    );
}

#[test]
fn an_undated_entry_fails_that_list() {
    assert_hsn_unreadable(
        FIRST_HSN_DATE,
        "<HSNDETAILS.LIST>",
        HsnDetailsDefect::EntryWithoutDate,
    );
    assert_gst_unreadable(
        "<GSTDETAILS.LIST>\r\n      <APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>",
        "<GSTDETAILS.LIST>",
        GstDetailsDefect::EntryWithoutDate,
    );
}

#[test]
fn a_date_that_is_not_a_real_date_fails_that_list() {
    assert_hsn_unreadable(
        "<APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>\r\n      <HSNCODE",
        "<APPLICABLEFROM TYPE=\"Date\">20250431</APPLICABLEFROM>\r\n      <HSNCODE",
        HsnDetailsDefect::DateInvalid,
    );
    assert_gst_unreadable(
        "<APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>\r\n      <TAXABILITY",
        "<APPLICABLEFROM TYPE=\"Date\">1-Apr-2025</APPLICABLEFROM>\r\n      <TAXABILITY",
        GstDetailsDefect::DateInvalid,
    );
}

#[test]
fn a_rate_that_is_not_a_plain_decimal_fails_the_gst_list() {
    for bad in ["1,8", "-18", "1e1", "18%", "."] {
        assert_gst_unreadable(
            FIRST_GST_RATE,
            &format!("<GSTRATE TYPE=\"Number\"> {bad}</GSTRATE>"),
            GstDetailsDefect::RateNotDecimal,
        );
    }
}

#[test]
fn a_credit_flag_that_is_not_yes_or_no_fails_the_gst_list() {
    assert_gst_unreadable(
        "<GSTINELIGIBLEITC TYPE=\"Logical\">Yes</GSTINELIGIBLEITC>",
        "<GSTINELIGIBLEITC TYPE=\"Logical\">Maybe</GSTINELIGIBLEITC>",
        GstDetailsDefect::ItcNotYesNo,
    );
    // `No` is a value, kept as such, and is not the absence of the element.
    let (_, gst) = item_after(
        "<GSTINELIGIBLEITC TYPE=\"Logical\">Yes</GSTINELIGIBLEITC>",
        "<GSTINELIGIBLEITC TYPE=\"Logical\">No</GSTINELIGIBLEITC>",
    )
    .unwrap();
    assert_eq!(gst_entries(&gst)[0].ineligible_itc, Some(false));
}

#[test]
fn two_entries_on_one_date_that_differ_fail_the_list_and_an_exact_repeat_is_kept_once() {
    // The second HSN entry moved onto the first one's date, with its own code.
    assert_hsn_unreadable(
        "<APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM>\r\n      <HSNCODE",
        "<APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>\r\n      <HSNCODE",
        HsnDetailsDefect::ConflictingEntriesOnOneDate,
    );
    assert_gst_unreadable(
        "<APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM>\r\n      <TAXABILITY",
        "<APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>\r\n      <TAXABILITY",
        GstDetailsDefect::ConflictingEntriesOnOneDate,
    );
    // The same HSN entry sent twice is one entry.
    let text = two_dates();
    let start = text.find(&format!("<STOCKITEM NAME=\"{ITEM}\"")).unwrap();
    let first = start + text[start..].find("<HSNDETAILS.LIST>").unwrap();
    let first_end =
        first + text[first..].find("</HSNDETAILS.LIST>").unwrap() + "</HSNDETAILS.LIST>".len();
    let repeated = format!(
        "{}{}{}",
        &text[..first_end],
        &text[first..first_end],
        &text[first_end..]
    );
    let masters = parse(&repeated).unwrap();
    let (hsn, _) = lists(row(&masters, ITEM));
    assert_eq!(hsn_entries(hsn).len(), 2);
}

#[test]
fn a_repeated_state_or_duty_head_row_fails_the_gst_list() {
    // A second state row with the same name.
    let state_row =
        "<STATEWISEDETAILS.LIST>\r\n       <STATENAME TYPE=\"String\">&#4; Any</STATENAME>";
    assert_gst_unreadable(
        state_row,
        &format!("<STATEWISEDETAILS.LIST><STATENAME TYPE=\"String\">&#4; Any</STATENAME></STATEWISEDETAILS.LIST>{state_row}"),
        GstDetailsDefect::DuplicateRow,
    );
    // A second CGST rate row in a state row.
    assert_gst_unreadable(
        FIRST_RATE_ROW,
        &format!("{FIRST_RATE_ROW}<GSTRATEDUTYHEAD TYPE=\"String\">CGST</GSTRATEDUTYHEAD></RATEDETAILS.LIST>{FIRST_RATE_ROW}"),
        GstDetailsDefect::DuplicateRow,
    );
}

#[test]
fn content_this_reader_does_not_model_is_flagged_and_alone_it_fails_the_list() {
    // Beside the recognised fields: kept, and flagged at the level it was seen.
    let (hsn, _) = item_after(
        FIRST_HSN_CODE,
        &format!("{FIRST_HSN_CODE}<HSNEXTRA>x</HSNEXTRA>"),
    )
    .unwrap();
    assert!(hsn_entries(&hsn)[0].other_content_skipped);
    assert!(!hsn_entries(&hsn)[1].other_content_skipped);
    let (_, gst) = item_after(
        FIRST_GST_RATE,
        &format!("{FIRST_GST_RATE}<NEWRATEFIELD>1</NEWRATEFIELD>"),
    )
    .unwrap();
    assert!(gst_entries(&gst)[0].other_content_skipped);
    let (_, gst) = item_after(
        "<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST>",
        "<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST><NEWSTATEFIELD/>",
    )
    .unwrap();
    assert!(gst_entries(&gst)[0].other_content_skipped);
    // An entry that holds nothing else: unreadable.
    assert_hsn_unreadable(
        "<HSNDETAILS.LIST>",
        "<HSNDETAILS.LIST><HSNEXTRA>x</HSNEXTRA></HSNDETAILS.LIST><HSNDETAILS.LIST>",
        HsnDetailsDefect::UnrecognisedContent,
    );
    assert_gst_unreadable(
        "<GSTDETAILS.LIST>",
        "<GSTDETAILS.LIST><NEWFIELD/></GSTDETAILS.LIST><GSTDETAILS.LIST>",
        GstDetailsDefect::UnrecognisedContent,
    );
    // Text in a list is content, and a no-break space is text, not whitespace.
    assert_hsn_unreadable(
        "<HSNDETAILS.LIST>",
        "<HSNDETAILS.LIST>stray</HSNDETAILS.LIST><HSNDETAILS.LIST>",
        HsnDetailsDefect::UnrecognisedContent,
    );
    assert_hsn_unreadable(
        "<HSNDETAILS.LIST>",
        "<HSNDETAILS.LIST>\u{a0}</HSNDETAILS.LIST><HSNDETAILS.LIST>",
        HsnDetailsDefect::UnrecognisedContent,
    );
    // So are an entity reference and CDATA directly in a list.
    for content in ["&amp;", "<![CDATA[x]]>"] {
        assert_hsn_unreadable(
            "<HSNDETAILS.LIST>",
            &format!("<HSNDETAILS.LIST>{content}</HSNDETAILS.LIST><HSNDETAILS.LIST>"),
            HsnDetailsDefect::UnrecognisedContent,
        );
    }
}

#[test]
fn a_filled_slab_list_is_flagged_and_its_content_is_not_read() {
    let (_, gst) = item_after(
        "<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST>",
        "<GSTSLABRATES.LIST><SLAB>x</SLAB></GSTSLABRATES.LIST>",
    )
    .unwrap();
    assert!(gst_entries(&gst)[0].states[0].slab_rates_present);
    assert!(!gst_entries(&gst)[0].other_content_skipped);
}

// The read is refused.

#[test]
fn tallys_error_elements_refuse_the_read_at_every_depth_of_a_list() {
    for element in ["ERROR", "LINEERROR", "error"] {
        let error = format!("<{element}>x</{element}>");
        let empty_error = format!("<{element}/>");
        for (anchor, replacement) in [
            // In an HSN entry, as a child and as an empty child.
            ("<HSNDETAILS.LIST>", format!("<HSNDETAILS.LIST>{error}")),
            (
                "<HSNDETAILS.LIST>",
                format!("<HSNDETAILS.LIST>{empty_error}"),
            ),
            // In a GST entry, a state row, a rate row and a slab list.
            ("<GSTDETAILS.LIST>", format!("<GSTDETAILS.LIST>{error}")),
            (
                "<STATEWISEDETAILS.LIST>",
                format!("<STATEWISEDETAILS.LIST>{error}"),
            ),
            (FIRST_RATE_ROW, format!("{FIRST_RATE_ROW}{error}")),
            (FIRST_RATE_ROW, format!("{FIRST_RATE_ROW}{empty_error}")),
            (
                "<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST>",
                format!("<GSTSLABRATES.LIST>{error}</GSTSLABRATES.LIST>"),
            ),
            // Inside a child this reader does not model, however deep.
            (
                FIRST_RATE_ROW,
                format!("{FIRST_RATE_ROW}<NEWFIELD><INNER>{error}</INNER></NEWFIELD>"),
            ),
            // Inside a scalar, at the entry, state and rate level.
            (
                FIRST_HSN_CODE,
                format!("<HSNCODE TYPE=\"String\">2836{error}</HSNCODE>"),
            ),
            (
                "<TAXABILITY TYPE=\"String\">Taxable</TAXABILITY>",
                format!("<TAXABILITY TYPE=\"String\">{error}</TAXABILITY>"),
            ),
            (
                "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>",
                format!("<STATENAME TYPE=\"String\">{error}</STATENAME>"),
            ),
            (
                FIRST_GST_RATE,
                format!("<GSTRATE TYPE=\"Number\">{error}</GSTRATE>"),
            ),
        ] {
            assert_eq!(
                refusal(anchor, &replacement),
                NativeMastersError::TallyReportedFailure,
                "{replacement}"
            );
        }
    }
}

#[test]
fn markup_nested_in_a_scalar_refuses_the_read() {
    for (anchor, replacement) in [
        (
            FIRST_HSN_CODE,
            "<HSNCODE TYPE=\"String\"><B>28362010</B></HSNCODE>",
        ),
        (
            FIRST_HSN_CODE,
            "<HSNCODE TYPE=\"String\">2836<B/></HSNCODE>",
        ),
        (
            "<HSN TYPE=\"String\">Disodium Carbonate, Dense</HSN>",
            "<HSN TYPE=\"String\"><![CDATA[Dense]]></HSN>",
        ),
        (
            FIRST_GST_RATE,
            "<GSTRATE TYPE=\"Number\"><V>18</V></GSTRATE>",
        ),
        (
            "<APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>\r\n      <TAXABILITY",
            "<APPLICABLEFROM TYPE=\"Date\"><D>20250401</D></APPLICABLEFROM>\r\n      <TAXABILITY",
        ),
    ] {
        assert_eq!(
            refusal(anchor, replacement),
            NativeMastersError::Malformed("masters_scalar_not_text_only"),
            "{replacement}"
        );
    }
}

#[test]
fn an_invalid_escape_in_a_list_scalar_refuses_the_read() {
    assert_eq!(
        refusal(
            FIRST_HSN_CODE,
            "<HSNCODE TYPE=\"String\">28&bogus;</HSNCODE>"
        ),
        NativeMastersError::Malformed("masters_xml_invalid_escape")
    );
    assert_eq!(
        refusal(
            FIRST_GST_RATE,
            "<GSTRATE TYPE=\"Number\"> 1&#xZZ;</GSTRATE>"
        ),
        NativeMastersError::Malformed("masters_xml_invalid_escape")
    );
}

#[test]
fn xml_that_is_not_well_formed_inside_a_list_refuses_the_read() {
    // A closing tag that does not match its opening tag.
    assert_eq!(
        refusal("</HSNDETAILS.LIST>", "</HSNDETAIL.LIST>"),
        NativeMastersError::Malformed("masters_xml_malformed")
    );
}

#[test]
fn a_response_cut_off_inside_a_list_refuses_the_read() {
    let text = two_dates();
    let start = text.find(&format!("<STOCKITEM NAME=\"{ITEM}\"")).unwrap();
    // After a complete field of the first HSN entry, and after a complete rate row.
    for marker in [FIRST_HSN_CODE, FIRST_GST_RATE, "<GSTSLABRATES.LIST>"] {
        let at = start + text[start..].find(marker).unwrap() + marker.len();
        assert_eq!(
            parse(&text[..at]).expect_err("a cut-off response is refused"),
            NativeMastersError::Malformed("masters_row_unterminated"),
            "{marker}"
        );
    }
    // Cut inside a field's text, and inside a tag.
    let at = start + text[start..].find(FIRST_HSN_CODE).unwrap();
    assert!(matches!(
        parse(&text[..at + 30]),
        Err(NativeMastersError::Malformed(
            "masters_xml_malformed" | "masters_row_unterminated"
        ))
    ));
    assert!(matches!(
        parse(&text[..at + 5]),
        Err(NativeMastersError::Malformed(
            "masters_xml_malformed" | "masters_row_unterminated"
        ))
    ));
}

// The row and the collection.

#[test]
fn the_collection_must_carry_the_stock_item_type() {
    let text = two_dates();
    assert!(text.contains("MSTDEPTYPE=\"512\""));
    for replacement in [
        "MSTDEPTYPE=\"128\"",
        "MSTDEPTYPE=\"\"",
        "MSTDEPTYPE=\"5120\"",
    ] {
        assert_eq!(
            parse(&text.replacen("MSTDEPTYPE=\"512\"", replacement, 1)),
            Err(NativeMastersError::Malformed(
                "masters_collection_type_unexpected"
            )),
            "{replacement}"
        );
    }
    assert_eq!(
        parse(&text.replacen(" MSTDEPTYPE=\"512\"", "", 1)),
        Err(NativeMastersError::Malformed(
            "masters_collection_type_unexpected"
        ))
    );
    // An empty collection that does not name the type is not "no stock items".
    let (start, open_end, end) = collection_span(&text);
    let empty_untyped = format!(
        "{}<COLLECTION></COLLECTION>{}",
        &text[..start],
        &text[end..]
    );
    assert_eq!(
        parse(&empty_untyped),
        Err(NativeMastersError::Malformed(
            "masters_collection_type_unexpected"
        ))
    );
    // The same collection, empty, with the type, is a zero-row answer.
    let empty_typed = format!(
        "{}{}</COLLECTION>{}",
        &text[..start],
        &text[start..open_end],
        &text[end..]
    );
    assert_eq!(parse(&empty_typed).unwrap().rows.len(), 0);
}

fn collection_span(text: &str) -> (usize, usize, usize) {
    let start = text.find("<COLLECTION").unwrap();
    let open_end = start + text[start..].find('>').unwrap() + 1;
    let end = text.find("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    (start, open_end, end)
}

#[test]
fn a_row_is_bound_to_the_company_by_its_guid_and_its_computed_company_guid() {
    let text = two_dates();
    let bound = format!("<BRIDGECOMPANYGUID TYPE=\"String\">{COMPANY}</BRIDGECOMPANYGUID>");
    // Missing: not read as bound.
    assert_eq!(
        parse(&edited(&text, &bound, "")),
        Err(NativeMastersError::RowCompanyMismatch)
    );
    // Another company's.
    assert_eq!(
        parse(&edited(&text, COMPANY, &foreign_company())),
        // The first occurrence is the row's GUID prefix.
        Err(NativeMastersError::RowGuidForeign)
    );
    assert_eq!(
        parse_native_masters(KIND, &text, &foreign_company()),
        Err(NativeMastersError::RowGuidForeign)
    );
    let foreign_bound = bound.replace(COMPANY, &foreign_company());
    assert_eq!(
        parse(&edited(&text, &bound, &foreign_bound)),
        Err(NativeMastersError::RowCompanyMismatch)
    );
}

#[test]
fn a_duplicate_guid_or_name_refuses_the_read() {
    let text = two_dates();
    let item_guid = format!("{COMPANY}-0000011a");
    assert_eq!(
        parse(&edited(&text, &item_guid, &format!("{COMPANY}-0000010e"))),
        Err(NativeMastersError::DuplicateGuid)
    );
    assert_eq!(
        parse(&text.replacen(
            &format!("<STOCKITEM NAME=\"{ITEM}\""),
            "<STOCKITEM NAME=\"soda ash light\"",
            1
        )),
        Err(NativeMastersError::DuplicateName)
    );
}

/// The row of the item as the parser measures it: from its opening tag to its
/// closing tag, in UTF-16 units.
fn item_row_units(text: &str) -> usize {
    let start = text.find(&format!("<STOCKITEM NAME=\"{ITEM}\"")).unwrap();
    let end = start + text[start..].find("</STOCKITEM>").unwrap() + "</STOCKITEM>".len();
    text[start..end].encode_utf16().count()
}

/// The capture with `extra` more GST entries on the item, each a copy of its
/// real 22 September 2025 entry with a new valid date, put after that entry.
fn with_more_gst_entries(extra: usize) -> String {
    let text = two_dates();
    let row = text.find(&format!("<STOCKITEM NAME=\"{ITEM}\"")).unwrap();
    let marker =
        "<GSTDETAILS.LIST>\r\n      <APPLICABLEFROM TYPE=\"Date\">20250922</APPLICABLEFROM>";
    let start = row + text[row..].find(marker).unwrap();
    let end =
        start + text[start..].find("</GSTDETAILS.LIST>").unwrap() + "</GSTDETAILS.LIST>".len();
    // The line break and indent before the entry go with each copy.
    let line = start - "\r\n     ".len();
    assert_eq!(&text[line..start], "\r\n     ");
    let mut copies = String::new();
    for index in 0..extra {
        let date = format!("2026{:02}01", index + 1);
        copies.push_str(&text[line..end].replace("20250922", &date));
    }
    format!("{}{copies}{}", &text[..end], &text[end..])
}

#[test]
fn seven_more_real_gst_entries_are_read_and_the_eighth_refuses_the_read() {
    let bound = masters_worst_row_bytes(KIND) / 2;
    assert_eq!(bound, 16_344);
    // Each copy of the real entry adds 1,576 units (the entry, 1,569, and its line break and indent) to the measured 4,373.
    let seven = with_more_gst_entries(7);
    assert_eq!(item_row_units(&seven), 4_373 + 7 * 1_576);
    let masters = parse(&seven).unwrap();
    let (_, gst) = lists(row(&masters, ITEM));
    assert_eq!(gst_entries(gst).len(), 9);
    // The eighth takes the row past the bound: the typed refusal, not a field.
    let eight = with_more_gst_entries(8);
    assert_eq!(item_row_units(&eight), 4_373 + 8 * 1_576);
    assert!(item_row_units(&eight) > bound);
    assert_eq!(parse(&eight), Err(NativeMastersError::RowExceedsBound));
}

#[test]
fn a_row_exactly_at_the_cut_off_is_read_and_one_unit_more_is_refused() {
    let bound = masters_worst_row_bytes(KIND) / 2;
    // Seven real extra entries, and the item's own taxability text made longer
    // (verbatim text, a valid value) until the row is exactly the bound.
    let base = with_more_gst_entries(7);
    let padded = |extra: usize| {
        edited(
            &base,
            "Taxable</TAXABILITY>",
            &format!("Taxable{}</TAXABILITY>", "t".repeat(extra)),
        )
    };
    let at_bound = padded(bound - item_row_units(&base));
    assert_eq!(item_row_units(&at_bound), bound);
    assert_eq!(parse(&at_bound).unwrap().rows.len(), 12);
    let over = padded(bound - item_row_units(&base) + 1);
    assert_eq!(item_row_units(&over), bound + 1);
    assert_eq!(parse(&over), Err(NativeMastersError::RowExceedsBound));
}

#[test]
fn a_name_longer_than_the_assumed_name_bound_refuses_the_read() {
    let long = "N".repeat(MASTERS_ASSUMED_NAME_CHARS + 1);
    assert_eq!(
        parse(&two_dates().replacen(ITEM, &long, 1)),
        Err(NativeMastersError::RowExceedsBound)
    );
}

#[test]
fn the_hsn_description_is_not_held_to_the_name_bound() {
    // Real tariff descriptions run past the name bound; the row bound still holds.
    let long = "D".repeat(MASTERS_ASSUMED_NAME_CHARS * 3);
    let (hsn, _) = item_after("Disodium Carbonate, Dense", &long).unwrap();
    assert_eq!(
        hsn_entries(&hsn)[0].description.as_deref(),
        Some(long.as_str())
    );
}

#[test]
fn a_code_is_never_trimmed_cut_or_read_as_a_number() {
    let (hsn, _) = item_after(
        FIRST_HSN_CODE,
        "<HSNCODE TYPE=\"String\"> 0028362010 99 </HSNCODE>",
    )
    .unwrap();
    assert_eq!(
        hsn_entries(&hsn)[0].hsn_code.as_deref(),
        Some(" 0028362010 99 ")
    );
}

// The other kinds.

#[test]
fn another_kind_still_skips_a_list_it_does_not_read_and_still_refuses_an_error_inside_it() {
    let groups = utf16le(include_bytes!(
        "../tests/fixtures/masters_stock_groups_shape_lab_live.utf16le.xml"
    ));
    let row_end = groups.find("</STOCKGROUP>").unwrap();
    let with_list = |inner: &str| {
        format!(
            "{}<HSNDETAILS.LIST>{inner}</HSNDETAILS.LIST>{}",
            &groups[..row_end],
            &groups[row_end..]
        )
    };
    let parsed = parse_native_masters(
        NativeMasterKind::StockGroups,
        &with_list("<HSNCODE>2836</HSNCODE>"),
        COMPANY,
    )
    .unwrap();
    assert_eq!(parsed.rows.len(), 3);
    assert!(parsed
        .rows
        .iter()
        .all(|row| row.detail == NativeMasterDetail::Plain));
    assert_eq!(
        parse_native_masters(
            NativeMasterKind::StockGroups,
            &with_list("<ERROR>x</ERROR>"),
            COMPANY
        ),
        Err(NativeMastersError::TallyReportedFailure)
    );
}

#[test]
fn a_state_row_other_than_any_is_read_like_any_other_content() {
    let (_, gst) = item_after(
        "<STATENAME TYPE=\"String\">&#4; Any</STATENAME>",
        "<STATENAME TYPE=\"String\">Goa</STATENAME>",
    )
    .unwrap();
    let gst = gst_entries(&gst);
    assert_eq!(gst[0].states[0].state_name.as_deref(), Some("Goa"));
    assert_eq!(gst[0].states[0].rates.len(), 5);
}
