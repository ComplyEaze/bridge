//! The series-level numbering parser and request against the live capture
//! (`tests/fixtures/voucher_type_numbering_series_*`). Every negative case is the
//! captured text with one edit; none has a hand-written response.
use super::*;
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
fn the_capture_reads_each_type_at_the_series_level() {
    let rows = parse_voucher_type_numbering(&response()).unwrap();
    assert_eq!(rows.len(), 24);
    // The three types set on the screen: method and duplicate setting match it.
    assert_eq!(
        row(&rows, "Journal").numbering,
        observed(
            SeriesNumberingMethod::Automatic,
            DuplicateSetting::NotPrevented
        )
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
            observed(
                SeriesNumberingMethod::Automatic,
                DuplicateSetting::NotPrevented
            ),
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
        observed(
            SeriesNumberingMethod::Automatic,
            DuplicateSetting::NotPrevented
        )
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
        // Seen on a real book on 28 September 2026, not on this capture.
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
            observed(
                SeriesNumberingMethod::Automatic,
                DuplicateSetting::NotPrevented
            )
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
