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
