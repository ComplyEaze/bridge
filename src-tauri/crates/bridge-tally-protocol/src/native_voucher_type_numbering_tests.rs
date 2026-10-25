//! The series-level numbering parser and request against the live captures
//! (`tests/fixtures/voucher_type_numbering_series_*` and, for the sub-method,
//! `tests/fixtures/voucher_type_numbering_m8r_*`). Every parser test uses the
//! captured text, and every negative case is that text with one edit; none has a
//! hand-written response. The `join` module parses the captured numbering rows and
//! builds the masters rows by hand from their names and GUIDs (no masters answer
//! of that book was captured), then edits those rows.
use super::*;
use crate::native_masters::MASTERS_ASSUMED_NAME_CHARS;
use sha2::{Digest, Sha256};

const COMPANY: &str = "Bridge Lab Win Numbering";

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

fn request_bytes() -> &'static [u8] {
    include_bytes!("../tests/fixtures/voucher_type_numbering_series_request.utf16le.xml")
}

fn response() -> String {
    utf16le(include_bytes!(
        "../tests/fixtures/voucher_type_numbering_series_live.utf16le.xml"
    ))
}

fn row<'a>(rows: &'a [VoucherTypeNumberingRow], name: &str) -> &'a VoucherTypeNumberingRow {
    rows.iter().find(|row| row.name == name).expect(name)
}

fn observed(method: SeriesNumberingMethod, duplicates: DuplicateSetting) -> VoucherTypeNumbering {
    VoucherTypeNumbering::Observed { method, duplicates }
}

/// Automatic with the sub-method the 9 October capture prints on every series.
fn retain() -> SeriesNumberingMethod {
    SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRetain)
}

/// The second book's answer before its Receipt type was changed.
fn before_renumber() -> String {
    utf16le(include_bytes!(
        "../tests/fixtures/voucher_type_numbering_m8r_x0_series.utf16le.xml"
    ))
}

/// The second book's answer with its Receipt type set to renumber vouchers.
fn set_to_renumber() -> String {
    utf16le(include_bytes!(
        "../tests/fixtures/voucher_type_numbering_m8r_x4_series.utf16le.xml"
    ))
}

const RENUMBER: &str = "<NUMBERINGSUBMETHOD>Auto Renumber</NUMBERINGSUBMETHOD>";
const RETAIN: &str = "<NUMBERINGSUBMETHOD>Auto Retain</NUMBERINGSUBMETHOD>";

/// `text` with the first `from` in the row of the type named `name` replaced by `to`.
fn edited_row(text: &str, name: &str, from: &str, to: &str) -> String {
    let row = text
        .find(&format!("<VOUCHERTYPE NAME=\"{name}\""))
        .expect(name);
    let end = row + text[row..].find("</VOUCHERTYPE>").unwrap();
    let at = row + text[row..end].find(from).expect(from);
    format!("{}{}{}", &text[..at], to, &text[at + from.len()..])
}

/// The Receipt row of the answer taken with the type set to renumber, its
/// sub-method element replaced by `to`.
fn receipt_sub_method(to: &str) -> Result<VoucherTypeNumbering, NativeMastersError> {
    let text = edited_row(&set_to_renumber(), "Receipt", RENUMBER, to);
    parse_voucher_type_numbering(&text).map(|rows| row(&rows, "Receipt").numbering.clone())
}

/// The Payment row of the capture, to edit one field of.
fn payment_series() -> String {
    let text = response();
    let start = text.find("<VOUCHERTYPE NAME=\"Payment\"").unwrap();
    let series = text[start..].find("<VOUCHERNUMBERSERIES.LIST>").unwrap() + start;
    let end = text[series..].find("</VOUCHERNUMBERSERIES.LIST>").unwrap()
        + series
        + "</VOUCHERNUMBERSERIES.LIST>".len();
    text[series..end].to_string()
}

fn edited(from: &str, to: &str) -> String {
    let text = response();
    let payment = text.find("<VOUCHERTYPE NAME=\"Payment\"").unwrap();
    let at = text[payment..].find(from).expect(from) + payment;
    format!("{}{}{}", &text[..at], to, &text[at + from.len()..])
}

#[test]
fn the_request_is_byte_for_byte_the_one_sent_live() {
    let rendered = render_voucher_type_numbering_request(COMPANY);
    let mut wire = vec![0xFF, 0xFE];
    wire.extend(rendered.encode_utf16().flat_map(u16::to_le_bytes));
    assert_eq!(wire, request_bytes());
    // The hash the contributor checked before sending it.
    assert_eq!(
        Sha256::digest(&wire)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        "70f1d9cefbfd00bda76fcdad35dc24707ba76d11a921e2ef63d8be317fc61934"
    );
}

#[test]
fn the_request_of_the_second_book_is_byte_for_byte_the_one_sent_live() {
    // The same renderer with the second book's name: the two answers of
    // 13 October 2026 committed here answer the product's own request.
    let rendered = render_voucher_type_numbering_request("BRIDGE MANUAL NUMBERING LAB 2");
    let mut wire = vec![0xFF, 0xFE];
    wire.extend(rendered.encode_utf16().flat_map(u16::to_le_bytes));
    assert_eq!(
        wire,
        include_bytes!("../tests/fixtures/voucher_type_numbering_m8r_series_request.utf16le.xml")
    );
    assert_eq!(
        Sha256::digest(&wire)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        "caf423642547b7673624ddc9a5a400a10bae208fbb99fdaa643aec0e0dbc4c28"
    );
}

#[test]
fn the_capture_reads_each_type_at_the_series_level() {
    let rows = parse_voucher_type_numbering(&response()).unwrap();
    assert_eq!(rows.len(), 24);
    // The three types set on the screen: method and duplicate setting match it.
    assert_eq!(
        row(&rows, "Journal").numbering,
        observed(retain(), DuplicateSetting::NotPrevented)
    );
    assert_eq!(
        row(&rows, "Payment").numbering,
        observed(SeriesNumberingMethod::Manual, DuplicateSetting::Prevented)
    );
    assert_eq!(
        row(&rows, "Receipt").numbering,
        observed(
            SeriesNumberingMethod::Manual,
            DuplicateSetting::NotPrevented
        )
    );
    // The other 21 types were not altered: Automatic, duplicates not prevented.
    let others = rows
        .iter()
        .filter(|row| !["Journal", "Payment", "Receipt"].contains(&row.name.as_str()));
    assert_eq!(others.clone().count(), 21);
    for row in others {
        assert_eq!(
            row.numbering,
            observed(retain(), DuplicateSetting::NotPrevented),
            "{}",
            row.name
        );
    }
}

#[test]
fn the_type_level_values_are_never_read() {
    // The capture's type level reads None for Payment and No for duplicates; the
    // parse above shows Manual and Prevented, so changing the type level changes nothing.
    let text = edited(
        "<NUMBERINGMETHOD TYPE=\"String\">None</NUMBERINGMETHOD>",
        "<NUMBERINGMETHOD TYPE=\"String\">Automatic</NUMBERINGMETHOD>",
    );
    let rows = parse_voucher_type_numbering(&text).unwrap();
    assert_eq!(
        row(&rows, "Payment").numbering,
        observed(SeriesNumberingMethod::Manual, DuplicateSetting::Prevented)
    );
}

#[test]
fn the_type_level_duplicate_setting_is_never_read_either() {
    // Payment's type level reads No for duplicates (its screen says Yes); Journal's series
    // says No. Flipping both type-level settings changes nothing.
    let text = edited(
        "<PREVENTDUPLICATES TYPE=\"Logical\">No</PREVENTDUPLICATES>",
        "<PREVENTDUPLICATES TYPE=\"Logical\">Yes</PREVENTDUPLICATES>",
    );
    let rows = parse_voucher_type_numbering(&text).unwrap();
    assert_eq!(
        row(&rows, "Payment").numbering,
        observed(SeriesNumberingMethod::Manual, DuplicateSetting::Prevented)
    );
    let flipped = response().replacen(
        "<PREVENTDUPLICATES TYPE=\"Logical\">No</PREVENTDUPLICATES>",
        "<PREVENTDUPLICATES TYPE=\"Logical\">Yes</PREVENTDUPLICATES>",
        24,
    );
    let rows = parse_voucher_type_numbering(&flipped).unwrap();
    assert_eq!(
        row(&rows, "Journal").numbering,
        observed(retain(), DuplicateSetting::NotPrevented)
    );
}

#[test]
fn a_field_nested_in_another_list_is_not_the_series_field() {
    // A NUMBERINGMETHOD or PREVENTDUPLICATES inside the series' own restart list, or inside
    // the type's language list, is not the series' field; the series' own is still read.
    let restart = edited(
        "<RESTARTFROMLIST.LIST>",
        "<RESTARTFROMLIST.LIST><NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD><PREVENTDUPLICATES>No</PREVENTDUPLICATES>",
    );
    let language = edited(
        "<LANGUAGENAME.LIST>",
        "<LANGUAGENAME.LIST><NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD>",
    );
    for text in [restart, language] {
        let rows = parse_voucher_type_numbering(&text).unwrap();
        assert_eq!(
            row(&rows, "Payment").numbering,
            observed(SeriesNumberingMethod::Manual, DuplicateSetting::Prevented)
        );
    }
    // The same for the sub-method, on the row captured as `Auto Renumber`: one
    // inside the series' restart list, or at the type's own level, neither
    // replaces the series' own nor repeats it.
    let renumber = set_to_renumber();
    for (from, to) in [
        (
            "<RESTARTFROMLIST.LIST>",
            format!("<RESTARTFROMLIST.LIST>{RETAIN}"),
        ),
        (
            "<LANGUAGENAME.LIST>",
            format!("{RETAIN}<LANGUAGENAME.LIST>"),
        ),
        (
            "<LANGUAGENAME.LIST>",
            format!("<LANGUAGENAME.LIST>{RETAIN}"),
        ),
    ] {
        let rows =
            parse_voucher_type_numbering(&edited_row(&renumber, "Receipt", from, &to)).unwrap();
        assert_eq!(
            row(&rows, "Receipt").numbering,
            observed(
                SeriesNumberingMethod::Automatic(NumberingSubMethod::AutoRenumber),
                DuplicateSetting::NotPrevented
            ),
            "{to}"
        );
    }
}

#[test]
fn a_second_series_or_none_is_not_read() {
    let series = payment_series();
    let two = edited(&series, &format!("{series}{series}"));
    assert_eq!(
        row(&parse_voucher_type_numbering(&two).unwrap(), "Payment").numbering,
        VoucherTypeNumbering::SeriesCount(2)
    );
    let none = edited(&series, "");
    assert_eq!(
        row(&parse_voucher_type_numbering(&none).unwrap(), "Payment").numbering,
        VoucherTypeNumbering::SeriesCount(0)
    );
}

#[test]
fn a_self_closed_series_list_is_still_a_series() {
    let series = payment_series();
    // Only an empty list: one series, with no method.
    let only_empty = edited(&series, "<VOUCHERNUMBERSERIES.LIST/>");
    assert_eq!(
        row(
            &parse_voucher_type_numbering(&only_empty).unwrap(),
            "Payment"
        )
        .numbering,
        VoucherTypeNumbering::FieldInvalid("series_method")
    );
    // A real series and an empty one: two series.
    let both = edited(&series, &format!("{series}<VOUCHERNUMBERSERIES.LIST/>"));
    assert_eq!(
        row(&parse_voucher_type_numbering(&both).unwrap(), "Payment").numbering,
        VoucherTypeNumbering::SeriesCount(2)
    );
}

#[test]
fn a_whitespace_only_method_is_a_blank_method() {
    let text = edited(
        "<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>",
        "<NUMBERINGMETHOD>  </NUMBERINGMETHOD>",
    );
    assert_eq!(
        row(&parse_voucher_type_numbering(&text).unwrap(), "Payment").numbering,
        VoucherTypeNumbering::FieldInvalid("series_method")
    );
}

#[test]
fn a_method_outside_the_measured_set_is_kept_raw_and_never_manual() {
    for (printed, expected) in [
        (
            "Serial",
            SeriesNumberingMethod::Unrecognised("Serial".to_string()),
        ),
        (
            " Manual",
            SeriesNumberingMethod::Unrecognised(" Manual".to_string()),
        ),
        (
            "manual",
            SeriesNumberingMethod::Unrecognised("manual".to_string()),
        ),
        // Measured on a synthetic book on 13 October 2026, not on this capture.
        (
            "Automatic (Manual Override)",
            SeriesNumberingMethod::AutomaticManualOverride,
        ),
    ] {
        let text = edited(
            "<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>",
            &format!("<NUMBERINGMETHOD>{printed}</NUMBERINGMETHOD>"),
        );
        let rows = parse_voucher_type_numbering(&text).unwrap();
        assert_eq!(
            row(&rows, "Payment").numbering,
            observed(expected, DuplicateSetting::Prevented),
            "{printed:?}"
        );
    }
}

#[test]
fn a_missing_blank_repeated_or_unrecognised_series_field_is_not_a_default() {
    let cases: [(&str, &str, &'static str); 6] = [
        (
            "<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>",
            "",
            "series_method",
        ),
        (
            "<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>",
            "<NUMBERINGMETHOD/>",
            "series_method",
        ),
        (
            "<PREVENTDUPLICATES>Yes</PREVENTDUPLICATES>",
            "",
            "series_duplicates",
        ),
        (
            "<PREVENTDUPLICATES>Yes</PREVENTDUPLICATES>",
            "<PREVENTDUPLICATES>yes</PREVENTDUPLICATES>",
            "series_duplicates",
        ),
        (
            "<PREVENTDUPLICATES>Yes</PREVENTDUPLICATES>",
            "<PREVENTDUPLICATES/>",
            "series_duplicates",
        ),
        (
            "<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>",
            "<NUMBERINGMETHOD>Manual</NUMBERINGMETHOD><NUMBERINGMETHOD>Manual</NUMBERINGMETHOD>",
            "series_field_repeated",
        ),
    ];
    for (from, to, code) in cases {
        let rows = parse_voucher_type_numbering(&edited(from, to)).unwrap();
        assert_eq!(
            row(&rows, "Payment").numbering,
            VoucherTypeNumbering::FieldInvalid(code),
            "{to}"
        );
    }
}

/// The second book (13 October 2026): 24 types, Payment numbered Manual, and the
/// Receipt type's screen set to retain, then to renumber.
#[test]
fn the_sub_method_reads_as_the_receipt_screen_was_set_in_each_capture() {
    for (text, receipt) in [
        (before_renumber(), NumberingSubMethod::AutoRetain),
        (set_to_renumber(), NumberingSubMethod::AutoRenumber),
    ] {
        let rows = parse_voucher_type_numbering(&text).unwrap();
        assert_eq!(rows.len(), 24);
        assert_eq!(
            row(&rows, "Receipt").numbering,
            observed(
                SeriesNumberingMethod::Automatic(receipt.clone()),
                DuplicateSetting::NotPrevented
            ),
            "{receipt:?}"
        );
        // Payment's series prints `Auto Retain` too, and is Manual: under
        // Manual the sub-method is not read.
        assert_eq!(
            row(&rows, "Payment").numbering,
            observed(
                SeriesNumberingMethod::Manual,
                DuplicateSetting::NotPrevented
            )
        );
        let others = rows
            .iter()
            .filter(|row| !["Payment", "Receipt"].contains(&row.name.as_str()));
        assert_eq!(others.clone().count(), 22);
        for row in others {
            assert_eq!(
                row.numbering,
                observed(retain(), DuplicateSetting::NotPrevented),
                "{} {receipt:?}",
                row.name
            );
        }
    }
}

#[test]
fn a_series_without_a_sub_method_reads_absent_and_never_retain() {
    assert_eq!(
        receipt_sub_method("").unwrap(),
        observed(
            SeriesNumberingMethod::Automatic(NumberingSubMethod::Absent),
            DuplicateSetting::NotPrevented
        )
    );
}

#[test]
fn a_sub_method_outside_the_two_measured_texts_is_kept_raw() {
    let at_bound = "x".repeat(MASTERS_ASSUMED_NAME_CHARS);
    for (element, raw) in [
        // Neither trimmed nor case-folded into a measured text.
        (
            "<NUMBERINGSUBMETHOD>auto retain</NUMBERINGSUBMETHOD>",
            "auto retain",
        ),
        (
            "<NUMBERINGSUBMETHOD> Auto Retain</NUMBERINGSUBMETHOD>",
            " Auto Retain",
        ),
        (
            "<NUMBERINGSUBMETHOD>Auto Retain </NUMBERINGSUBMETHOD>",
            "Auto Retain ",
        ),
        (
            "<NUMBERINGSUBMETHOD>AUTO RENUMBER</NUMBERINGSUBMETHOD>",
            "AUTO RENUMBER",
        ),
        (
            "<NUMBERINGSUBMETHOD>Renumber</NUMBERINGSUBMETHOD>",
            "Renumber",
        ),
        // Blank is a printed value, not a missing element.
        ("<NUMBERINGSUBMETHOD/>", ""),
        ("<NUMBERINGSUBMETHOD></NUMBERINGSUBMETHOD>", ""),
        ("<NUMBERINGSUBMETHOD>  </NUMBERINGSUBMETHOD>", "  "),
        (
            &format!("<NUMBERINGSUBMETHOD>{at_bound}</NUMBERINGSUBMETHOD>"),
            at_bound.as_str(),
        ),
    ] {
        assert_eq!(
            receipt_sub_method(element).unwrap(),
            observed(
                SeriesNumberingMethod::Automatic(NumberingSubMethod::Unrecognised(raw.to_string())),
                DuplicateSetting::NotPrevented
            ),
            "{element}"
        );
    }
}

#[test]
fn a_sub_method_longer_than_a_name_refuses_the_answer() {
    let over = "x".repeat(MASTERS_ASSUMED_NAME_CHARS + 1);
    assert_eq!(
        receipt_sub_method(&format!("<NUMBERINGSUBMETHOD>{over}</NUMBERINGSUBMETHOD>"))
            .unwrap_err(),
        NativeMastersError::RowExceedsBound
    );
}

#[test]
fn a_repeated_sub_method_is_the_repeated_field_reading() {
    for repeated in [
        format!("{RENUMBER}{RENUMBER}"),
        format!("{RENUMBER}{RETAIN}"),
        format!("{RETAIN}<NUMBERINGSUBMETHOD/>"),
        format!("<NUMBERINGSUBMETHOD/>{RETAIN}"),
    ] {
        assert_eq!(
            receipt_sub_method(&repeated).unwrap(),
            VoucherTypeNumbering::FieldInvalid("series_field_repeated"),
            "{repeated}"
        );
    }
    // Under Manual too: a repeat is refused before the method is looked at.
    let text = edited_row(
        &set_to_renumber(),
        "Payment",
        RETAIN,
        &format!("{RETAIN}{RETAIN}"),
    );
    let rows = parse_voucher_type_numbering(&text).unwrap();
    assert_eq!(
        row(&rows, "Payment").numbering,
        VoucherTypeNumbering::FieldInvalid("series_field_repeated")
    );
}

#[test]
fn the_sub_method_of_a_manual_series_is_not_read() {
    let over = "x".repeat(MASTERS_ASSUMED_NAME_CHARS + 1);
    for to in [
        String::new(),
        RENUMBER.to_string(),
        "<NUMBERINGSUBMETHOD>Serial</NUMBERINGSUBMETHOD>".to_string(),
        format!("<NUMBERINGSUBMETHOD>{over}</NUMBERINGSUBMETHOD>"),
    ] {
        let text = edited_row(&set_to_renumber(), "Payment", RETAIN, &to);
        let rows = parse_voucher_type_numbering(&text).unwrap();
        assert_eq!(
            row(&rows, "Payment").numbering,
            observed(
                SeriesNumberingMethod::Manual,
                DuplicateSetting::NotPrevented
            ),
            "{to}"
        );
    }
}

#[test]
fn an_envelope_that_is_not_the_closed_shape_is_refused() {
    let text = response();
    let refused = |text: &str| parse_voucher_type_numbering(text).unwrap_err();
    assert_eq!(
        refused(&text.replacen("<STATUS>1</STATUS>", "<STATUS>0</STATUS>", 1)),
        NativeMastersError::TallyReportedFailure
    );
    assert_eq!(
        refused(&text.replacen("<STATUS>1</STATUS>", "", 1)),
        NativeMastersError::Malformed("masters_status_absent")
    );
    assert_eq!(
        refused(&text.replacen("</COLLECTION>", "<LINEERROR>x</LINEERROR></COLLECTION>", 1)),
        NativeMastersError::TallyReportedFailure
    );
    assert_eq!(
        refused(&text.replacen(
            "<VOUCHERTYPE NAME=\"Receipt\"",
            "<VOUCHERTYPE NAME=\"Payment\"",
            1
        )),
        NativeMastersError::DuplicateName
    );
    assert_eq!(
        refused(&text.replacen(
            "<VOUCHERTYPE NAME=\"Receipt\"",
            "<LEDGER NAME=\"Receipt\"",
            1
        )),
        NativeMastersError::ForeignChild
    );
    assert_eq!(
        refused(&text.replacen("<COLLECTION", "<NOTCOLLECTION", 1).replacen(
            "</COLLECTION>",
            "</NOTCOLLECTION>",
            1
        )),
        NativeMastersError::CollectionAbsent
    );
    assert_eq!(
        refused("<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION></COLLECTION></DATA></BODY></ENVELOPE>"),
        NativeMastersError::VoucherTypesEmpty
    );
    assert_eq!(
        refused("<RESPONSE>x</RESPONSE>"),
        NativeMastersError::Malformed("masters_root_not_envelope")
    );
}

const PAYMENT_GUID: &str = "3479ef95-3ecd-49b3-9349-d3a8e24b0db2-00000022";
const PAYMENT_GUID_ELEMENT: &str =
    "<GUID TYPE=\"String\">3479ef95-3ecd-49b3-9349-d3a8e24b0db2-00000022</GUID>";

#[test]
fn every_captured_row_keeps_its_guid() {
    let text = response();
    // Counted from the file by element name, independently of the parser.
    assert_eq!(text.matches("<GUID ").count(), 24);
    let rows = parse_voucher_type_numbering(&text).unwrap();
    assert_eq!(rows.len(), 24);
    for row in &rows {
        assert!(
            text.contains(&format!("<GUID TYPE=\"String\">{}</GUID>", row.guid)),
            "{}",
            row.name
        );
    }
    assert_eq!(row(&rows, "Payment").guid, PAYMENT_GUID);
    // 24 distinct GUIDs: the join key is unique in the capture.
    assert_eq!(
        rows.iter()
            .map(|row| row.guid.as_str())
            .collect::<HashSet<_>>()
            .len(),
        24
    );
}

#[test]
fn a_missing_blank_or_repeated_guid_is_a_typed_error_not_a_default() {
    let refused =
        |from: &str, to: &str| parse_voucher_type_numbering(&edited(from, to)).unwrap_err();
    assert_eq!(
        refused(PAYMENT_GUID_ELEMENT, ""),
        NativeMastersError::Malformed("masters_numbering_guid_missing")
    );
    assert_eq!(
        refused(PAYMENT_GUID_ELEMENT, "<GUID/>"),
        NativeMastersError::Malformed("masters_numbering_guid_missing")
    );
    assert_eq!(
        refused(PAYMENT_GUID_ELEMENT, "<GUID TYPE=\"String\">   </GUID>"),
        NativeMastersError::Malformed("masters_numbering_guid_missing")
    );
    let repeated_guid = NativeMastersError::Malformed("masters_numbering_guid_repeated");
    assert_eq!(
        refused(
            PAYMENT_GUID_ELEMENT,
            &format!("{PAYMENT_GUID_ELEMENT}{PAYMENT_GUID_ELEMENT}")
        ),
        repeated_guid
    );
    assert_eq!(
        refused(
            PAYMENT_GUID_ELEMENT,
            &format!("{PAYMENT_GUID_ELEMENT}<GUID/>")
        ),
        repeated_guid
    );
    assert_eq!(
        refused(
            PAYMENT_GUID_ELEMENT,
            &format!("<GUID/>{PAYMENT_GUID_ELEMENT}")
        ),
        repeated_guid
    );
}

#[test]
fn a_guid_shared_by_two_types_is_refused_by_the_parser() {
    let text = response();
    let journal_guid = "3479ef95-3ecd-49b3-9349-d3a8e24b0db2-00000026";
    assert!(text.contains(journal_guid));
    // Payment under Journal's GUID, exactly and differing only in case.
    for shared in [journal_guid.to_string(), journal_guid.to_ascii_uppercase()] {
        let shared_text = edited(
            PAYMENT_GUID_ELEMENT,
            &format!("<GUID TYPE=\"String\">{shared}</GUID>"),
        );
        assert_eq!(
            parse_voucher_type_numbering(&shared_text).unwrap_err(),
            NativeMastersError::DuplicateGuid,
            "{shared}"
        );
    }
}

#[test]
fn a_guid_nested_in_another_list_is_not_the_types() {
    // Only a direct child of the type is its identity: one inside the language
    // list or the series list neither repeats the type's nor stands in for it.
    let nested = edited(
        "<LANGUAGENAME.LIST>",
        "<LANGUAGENAME.LIST><GUID>other</GUID>",
    );
    let rows = parse_voucher_type_numbering(&nested).unwrap();
    assert_eq!(row(&rows, "Payment").guid, PAYMENT_GUID);
    let series = edited(
        "<VOUCHERNUMBERSERIES.LIST>",
        "<VOUCHERNUMBERSERIES.LIST><GUID>other</GUID>",
    );
    let rows = parse_voucher_type_numbering(&series).unwrap();
    assert_eq!(row(&rows, "Payment").guid, PAYMENT_GUID);
    // A type whose only GUID is nested has none of its own.
    let only_nested = edited(
        PAYMENT_GUID_ELEMENT,
        "<LANGUAGENAME.LIST><GUID>other</GUID></LANGUAGENAME.LIST>",
    );
    assert_eq!(
        parse_voucher_type_numbering(&only_nested).unwrap_err(),
        NativeMastersError::Malformed("masters_numbering_guid_missing")
    );
}

mod join {
    use super::*;
    use crate::native_masters::{NativeMasterDetail, NativeMasterRow};

    /// The masters answer for the same book as the captured numbering rows:
    /// each row's name and GUID, in the numbering rows' order.
    fn masters(rows: &[VoucherTypeNumberingRow]) -> Vec<NativeMasterRow> {
        rows.iter()
            .map(|row| NativeMasterRow {
                name: row.name.clone(),
                guid: row.guid.clone(),
                master_id: 1,
                alter_id: 1,
                parent: None,
                detail: NativeMasterDetail::VoucherType {
                    active: Some(true),
                    optional: Some(false),
                    numbering: None,
                },
            })
            .collect()
    }

    fn captured() -> Vec<VoucherTypeNumberingRow> {
        parse_voucher_type_numbering(&response()).unwrap()
    }

    fn index(rows: &[NativeMasterRow], name: &str) -> usize {
        rows.iter().position(|row| row.name == name).expect(name)
    }

    #[test]
    fn equal_answers_join_one_result_per_masters_row() {
        let numbering = captured();
        let joined = join_series_numbering(&masters(&numbering), &numbering).unwrap();
        assert_eq!(joined.len(), 24);
        for (row, result) in numbering.iter().zip(&joined) {
            assert_eq!(&row.numbering, result, "{}", row.name);
        }
    }

    #[test]
    fn the_join_follows_the_masters_order_and_the_key_not_the_position() {
        let numbering = captured();
        // The masters answer in the reverse order: a positional join would
        // hand every row its neighbour's result.
        let mut reversed = masters(&numbering);
        reversed.reverse();
        let joined = join_series_numbering(&reversed, &numbering).unwrap();
        let payment = index(&reversed, "Payment");
        assert_eq!(
            joined[payment],
            observed(SeriesNumberingMethod::Manual, DuplicateSetting::Prevented)
        );
        let journal = index(&reversed, "Journal");
        assert_eq!(
            joined[journal],
            observed(retain(), DuplicateSetting::NotPrevented)
        );
        let receipt = index(&reversed, "Receipt");
        assert_eq!(
            joined[receipt],
            observed(
                SeriesNumberingMethod::Manual,
                DuplicateSetting::NotPrevented
            )
        );
        // And the numbering answer reordered: still by key.
        let mut shuffled = numbering.clone();
        shuffled.reverse();
        let joined = join_series_numbering(&masters(&numbering), &shuffled).unwrap();
        for (row, result) in numbering.iter().zip(&joined) {
            assert_eq!(&row.numbering, result, "{}", row.name);
        }
    }

    #[test]
    fn a_type_on_one_side_only_refuses_the_whole_read() {
        let numbering = captured();
        // In the masters answer only.
        let mut fewer = numbering.clone();
        fewer.retain(|row| row.name != "Payroll");
        let error = join_series_numbering(&masters(&numbering), &fewer).unwrap_err();
        assert_eq!(error, NumberingJoinError::TypeOnOneSideOnly);
        assert_eq!(error.code(), "masters_numbering_names_differ");
        // In the numbering answer only.
        let mut fewer_masters = masters(&numbering);
        fewer_masters.retain(|row| row.name != "Payroll");
        let error = join_series_numbering(&fewer_masters, &numbering).unwrap_err();
        assert_eq!(error, NumberingJoinError::TypeOnOneSideOnly);
        assert_eq!(error.code(), "masters_numbering_names_differ");
        // A different type in each: a name and a GUID found on no other side.
        let mut other = numbering.clone();
        other[0].name = "Another".into();
        other[0].guid = "3479ef95-3ecd-49b3-9349-d3a8e24b0db2-000000ff".into();
        assert_eq!(
            join_series_numbering(&masters(&numbering), &other).unwrap_err(),
            NumberingJoinError::TypeOnOneSideOnly
        );
    }

    #[test]
    fn names_that_differ_only_in_case_join_and_nothing_wider_does() {
        let numbering = captured();
        let mut folded = masters(&numbering);
        let payment = index(&folded, "Payment");
        folded[payment].name = "PAYMENT".into();
        let joined = join_series_numbering(&folded, &numbering).unwrap();
        assert_eq!(joined[payment], numbering[payment].numbering);
        // A GUID differing only in case is the same GUID (both answers key
        // their GUIDs this way).
        let mut upper = numbering.clone();
        upper[payment].guid = upper[payment].guid.to_ascii_uppercase();
        assert!(join_series_numbering(&masters(&numbering), &upper).is_ok());
        // Surrounding space is not folded away.
        let mut spaced = masters(&numbering);
        spaced[payment].name = "Payment ".into();
        assert_eq!(
            join_series_numbering(&spaced, &numbering).unwrap_err(),
            NumberingJoinError::SameGuidDifferentNames
        );
    }

    #[test]
    fn one_guid_with_two_names_is_an_identity_difference() {
        let numbering = captured();
        let mut renamed = masters(&numbering);
        let payment = index(&renamed, "Payment");
        renamed[payment].name = "Payments".into();
        let error = join_series_numbering(&renamed, &numbering).unwrap_err();
        assert_eq!(error, NumberingJoinError::SameGuidDifferentNames);
        assert_eq!(error.code(), "masters_numbering_identity_differs");
    }

    #[test]
    fn one_name_with_two_guids_is_an_identity_difference() {
        let numbering = captured();
        let mut moved = masters(&numbering);
        let payment = index(&moved, "Payment");
        moved[payment].guid = "3479ef95-3ecd-49b3-9349-d3a8e24b0db2-000000ff".into();
        let error = join_series_numbering(&moved, &numbering).unwrap_err();
        assert_eq!(error, NumberingJoinError::SameNameDifferentGuids);
        assert_eq!(error.code(), "masters_numbering_identity_differs");
        // The same, seen from the numbering answer's side.
        let mut other = numbering.clone();
        other[payment].guid = "3479ef95-3ecd-49b3-9349-d3a8e24b0db2-000000ff".into();
        assert_eq!(
            join_series_numbering(&masters(&numbering), &other).unwrap_err(),
            NumberingJoinError::SameNameDifferentGuids
        );
    }

    #[test]
    fn each_observation_is_carried_to_its_own_row() {
        let series = payment_series();
        let two = format!("{series}{series}");
        let text = edited(&series, &two)
            .replacen(
                "<NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD>",
                "<NUMBERINGMETHOD>Serial</NUMBERINGMETHOD>",
                1,
            )
            .replacen(
                "<PREVENTDUPLICATES>No</PREVENTDUPLICATES>",
                "<PREVENTDUPLICATES>maybe</PREVENTDUPLICATES>",
                1,
            );
        let numbering = parse_voucher_type_numbering(&text).unwrap();
        let joined = join_series_numbering(&masters(&numbering), &numbering).unwrap();
        let by_name = |name: &str| &joined[numbering.iter().position(|r| r.name == name).unwrap()];
        assert_eq!(by_name("Payment"), &VoucherTypeNumbering::SeriesCount(2));
        // The first Automatic series in the file belongs to Attendance.
        assert_eq!(
            by_name("Attendance"),
            &VoucherTypeNumbering::FieldInvalid("series_duplicates")
        );
        let unrecognised = text.replacen(
            "<NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD>",
            "<NUMBERINGMETHOD>Serial</NUMBERINGMETHOD>",
            1,
        );
        let numbering = parse_voucher_type_numbering(&unrecognised).unwrap();
        let joined = join_series_numbering(&masters(&numbering), &numbering).unwrap();
        let contra = numbering.iter().position(|r| r.name == "Contra").unwrap();
        assert_eq!(
            joined[contra],
            observed(
                SeriesNumberingMethod::Unrecognised("Serial".into()),
                DuplicateSetting::NotPrevented
            )
        );
    }
}
