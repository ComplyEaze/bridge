//! The masters parser and request builders against the live captures
//! (`tests/fixtures/masters_*`). Every negative case is the captured text with
//! one edit; none has a hand-written response.
use super::*;
use crate::native_outstandings::render_native_voucher_type_export_request;
use crate::{is_tally_reserved_root, TALLY_SANITIZED_ROOT_MARKER};

const COMPANY: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const FOREIGN_COMPANY: &str = "ffffffff-b835-4bff-89dd-8a6af138c346";
const LAB: &str = "BRIDGE SHAPE LAB";
/// The GUID of the second synthetic book (`BRIDGE READS LAB`), whose captures
/// hold the zero-row answers.
const READS_LAB_COMPANY: &str = "de2e15f2-6d42-4715-b6e7-b7a95a68abe8";

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

fn response(kind: NativeMasterKind) -> String {
    utf16le(match kind {
        NativeMasterKind::VoucherTypes => {
            &include_bytes!("../tests/fixtures/masters_voucher_types_shape_lab_live.utf16le.xml")[..]
        }
        NativeMasterKind::Godowns => {
            &include_bytes!("../tests/fixtures/masters_godowns_shape_lab_live.utf16le.xml")[..]
        }
        NativeMasterKind::Units => {
            &include_bytes!("../tests/fixtures/masters_units_shape_lab_live.utf16le.xml")[..]
        }
        NativeMasterKind::StockGroups => {
            &include_bytes!("../tests/fixtures/masters_stock_groups_shape_lab_live.utf16le.xml")[..]
        }
        // Captured 7 Oct 2026 on a book whose Cost Centres flag reads No and that holds two centres.
        NativeMasterKind::CostCentres => &include_bytes!(
            "../tests/fixtures/masters_cost_centres_shape_lab_flag_no_live.utf16le.xml"
        )[..],
        NativeMasterKind::CostCategories => {
            &include_bytes!("../tests/fixtures/masters_cost_categories_shape_lab_live.utf16le.xml")
                [..]
        }
    })
}

fn request_fixture(kind: NativeMasterKind) -> String {
    utf16le(match kind {
        NativeMasterKind::VoucherTypes => {
            &include_bytes!("../tests/fixtures/masters_voucher_types_request.utf16le.xml")[..]
        }
        NativeMasterKind::Godowns => {
            &include_bytes!("../tests/fixtures/masters_godowns_request.utf16le.xml")[..]
        }
        NativeMasterKind::Units => {
            &include_bytes!("../tests/fixtures/masters_units_request.utf16le.xml")[..]
        }
        NativeMasterKind::StockGroups => {
            &include_bytes!("../tests/fixtures/masters_stock_groups_request.utf16le.xml")[..]
        }
        NativeMasterKind::CostCentres => {
            &include_bytes!("../tests/fixtures/masters_cost_centres_request.utf16le.xml")[..]
        }
        NativeMasterKind::CostCategories => {
            &include_bytes!("../tests/fixtures/masters_cost_categories_request.utf16le.xml")[..]
        }
    })
}

/// A `BRIDGE READS LAB` capture: a book without inventory, so units and stock
/// groups answer with an empty collection and godowns with one row.
fn reads_lab_response(kind: NativeMasterKind) -> String {
    utf16le(match kind {
        NativeMasterKind::Units => {
            &include_bytes!("../tests/fixtures/masters_units_reads_lab_live.utf16le.xml")[..]
        }
        NativeMasterKind::StockGroups => {
            &include_bytes!("../tests/fixtures/masters_stock_groups_reads_lab_live.utf16le.xml")[..]
        }
        NativeMasterKind::Godowns => {
            &include_bytes!("../tests/fixtures/masters_godowns_reads_lab_live.utf16le.xml")[..]
        }
        NativeMasterKind::VoucherTypes
        | NativeMasterKind::CostCentres
        | NativeMasterKind::CostCategories => {
            panic!("no READS LAB capture of this kind")
        }
    })
}

/// The rows the capture holds, counted independently of the parser.
fn captured_rows(kind: NativeMasterKind) -> usize {
    match kind {
        NativeMasterKind::VoucherTypes => 26,
        NativeMasterKind::Godowns => 2,
        NativeMasterKind::Units => 4,
        NativeMasterKind::StockGroups => 3,
        NativeMasterKind::CostCentres | NativeMasterKind::CostCategories => 2,
    }
}

fn parse(kind: NativeMasterKind, text: &str) -> Result<NativeMasters, NativeMastersError> {
    parse_native_masters(kind, text, COMPANY)
}

/// The capture with the first `from` at or after its `DATA` element, where
/// its rows are, replaced by `to`.
fn edited(kind: NativeMasterKind, from: &str, to: &str) -> String {
    let text = response(kind);
    let data = text.find("<DATA>").expect("the capture has a DATA element");
    let at = data
        + text[data..]
            .find(from)
            .unwrap_or_else(|| panic!("the capture has no {from}"));
    format!("{}{}{}", &text[..at], to, &text[at + from.len()..])
}

/// The byte range of the capture's `COLLECTION` element, and where its
/// opening tag ends.
fn collection_span(text: &str) -> (usize, usize, usize) {
    let start = text
        .find("<COLLECTION")
        .expect("the capture has a COLLECTION");
    let open_end = start + text[start..].find('>').expect("an opening tag") + 1;
    let end = text.find("</COLLECTION>").expect("a closing tag") + "</COLLECTION>".len();
    (start, open_end, end)
}

fn first_godown_guid() -> String {
    format!("{COMPANY}-000000d4")
}

fn bound_element(company: &str) -> String {
    format!("<BRIDGECOMPANYGUID TYPE=\"String\">{company}</BRIDGECOMPANYGUID>")
}

#[test]
fn every_request_is_byte_equal_to_its_committed_fixture() {
    for kind in NativeMasterKind::ALL {
        assert_eq!(
            render_native_masters_request(kind, LAB),
            request_fixture(kind),
            "{kind:?}"
        );
    }
}

#[test]
fn the_voucher_type_request_is_the_existing_builder_with_the_masters_fields() {
    let existing = render_native_voucher_type_export_request(LAB);
    let derived = existing.replace(
        "ALTERID</FETCH>",
        "ALTERID, ISACTIVE, ISOPTIONAL, NUMBERINGMETHOD</FETCH><COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>",
    );
    assert_ne!(derived, existing, "the derivation must change something");
    assert_eq!(
        render_native_masters_request(NativeMasterKind::VoucherTypes, LAB),
        derived
    );
}

#[test]
fn every_capture_parses_with_its_row_count_and_binds_every_row_to_the_company() {
    for kind in NativeMasterKind::ALL {
        let parsed = parse(kind, &response(kind)).expect("the capture parses");
        assert_eq!(parsed.rows.len(), captured_rows(kind), "{kind:?}");
        for row in &parsed.rows {
            assert!(
                row.guid
                    .to_ascii_lowercase()
                    .starts_with(&format!("{COMPANY}-")),
                "{kind:?} {}",
                row.name
            );
        }
        // The same capture read for another company binds none of its rows.
        assert_eq!(
            parse_native_masters(kind, &response(kind), FOREIGN_COMPANY),
            Err(NativeMastersError::RowGuidForeign),
            "{kind:?}"
        );
    }
}

#[test]
fn the_voucher_type_capture_reads_numbering_activity_and_parent() {
    let rows = parse(
        NativeMasterKind::VoucherTypes,
        &response(NativeMasterKind::VoucherTypes),
    )
    .unwrap()
    .rows;
    let (mut default, mut automatic, mut manual) = (0, 0, 0);
    for row in &rows {
        let NativeMasterDetail::VoucherType {
            optional,
            numbering,
            ..
        } = &row.detail
        else {
            panic!("a voucher type row carries voucher type detail");
        };
        assert_eq!(*optional, Some(false), "{}", row.name);
        match numbering {
            Some(NativeNumberingMethod::Default) => default += 1,
            Some(NativeNumberingMethod::Automatic) => automatic += 1,
            Some(NativeNumberingMethod::Manual) => manual += 1,
            other => panic!("{}: {other:?}", row.name),
        }
    }
    assert_eq!((default, automatic, manual), (24, 1, 1));
    let attendance = &rows[0];
    assert_eq!(attendance.name, "Attendance");
    assert_eq!(attendance.parent.as_deref(), Some("Attendance"));
    assert_eq!((attendance.master_id, attendance.alter_id), (78, 80));
    assert!(matches!(
        attendance.detail,
        NativeMasterDetail::VoucherType {
            active: Some(false),
            ..
        }
    ));
    let contra = rows.iter().find(|row| row.name == "Contra").unwrap();
    assert!(matches!(
        contra.detail,
        NativeMasterDetail::VoucherType {
            active: Some(true),
            ..
        }
    ));
}

#[test]
fn the_godown_capture_keeps_its_reserved_root_parent_as_the_group_snapshot_does() {
    let rows = parse(
        NativeMasterKind::Godowns,
        &response(NativeMasterKind::Godowns),
    )
    .unwrap()
    .rows;
    assert_eq!(rows[0].name, "Factory Floor");
    assert_eq!(rows[0].guid, first_godown_guid());
    assert_eq!((rows[0].master_id, rows[0].alter_id), (212, 213));
    assert_eq!(rows[0].detail, NativeMasterDetail::Plain);
    assert_eq!(
        rows[0].parent.as_deref(),
        Some(format!("{TALLY_SANITIZED_ROOT_MARKER} Primary").as_str())
    );
    assert!(is_tally_reserved_root(rows[0].parent.as_deref().unwrap()));
}

#[test]
fn the_unit_capture_reads_decimal_places_and_the_simple_flag() {
    let rows = parse(NativeMasterKind::Units, &response(NativeMasterKind::Units))
        .unwrap()
        .rows;
    let unit = |name: &str| rows.iter().find(|row| row.name == name).unwrap();
    assert_eq!(
        unit("Kgs").detail,
        NativeMasterDetail::Unit {
            decimal_places: 3,
            simple: true,
        }
    );
    assert!(matches!(
        unit("Nos of 10 Box").detail,
        NativeMasterDetail::Unit {
            decimal_places: 0,
            simple: false,
            ..
        }
    ));
    assert_eq!(unit("Box").parent, None, "a unit has no parent");
    // A parent a unit row carries anyway is not read as one.
    let kind = NativeMasterKind::Units;
    let injected = edited(
        kind,
        "<ISSIMPLEUNIT TYPE=\"Logical\">Yes</ISSIMPLEUNIT>",
        "<ISSIMPLEUNIT TYPE=\"Logical\">Yes</ISSIMPLEUNIT><PARENT TYPE=\"String\">Somewhere</PARENT>",
    );
    assert_eq!(parse(kind, &injected).unwrap().rows[0].parent, None);
}

#[test]
fn a_tally_failure_or_a_missing_status_is_refused() {
    let kind = NativeMasterKind::Godowns;
    // STATUS lies before DATA, so these edit the whole text.
    let text = response(kind);
    assert_eq!(
        parse(
            kind,
            &text.replacen("<STATUS>1</STATUS>", "<STATUS>0</STATUS>", 1)
        ),
        Err(NativeMastersError::TallyReportedFailure)
    );
    assert_eq!(
        parse(
            kind,
            &text.replacen("<STATUS>1</STATUS>", "<STATUS></STATUS>", 1)
        ),
        Err(NativeMastersError::Malformed("masters_status_absent"))
    );
    assert_eq!(
        parse(kind, &text.replacen("<STATUS>1</STATUS>", "", 1)),
        Err(NativeMastersError::Malformed("masters_status_absent"))
    );
    assert_eq!(
        parse(
            kind,
            &text.replacen(
                "<STATUS>1</STATUS>",
                "<STATUS>1</STATUS><STATUS>1</STATUS>",
                1
            )
        ),
        Err(NativeMastersError::Malformed("masters_status_repeated"))
    );
}

#[test]
fn an_error_element_anywhere_refuses() {
    let godowns = NativeMasterKind::Godowns;
    assert_eq!(
        parse(
            godowns,
            &edited(godowns, "</GODOWN>", "<LINEERROR>x</LINEERROR></GODOWN>")
        ),
        Err(NativeMastersError::TallyReportedFailure)
    );
    let voucher_types = NativeMasterKind::VoucherTypes;
    assert_eq!(
        parse(
            voucher_types,
            &edited(
                voucher_types,
                "<ALIAS TYPE=\"String\"></ALIAS>",
                "<ALIAS TYPE=\"String\"><ERROR>x</ERROR></ALIAS>"
            )
        ),
        Err(NativeMastersError::TallyReportedFailure)
    );
    let text = response(godowns);
    assert_eq!(
        parse(
            godowns,
            &text.replacen("<BODY>", "<BODY><LINEERROR>x</LINEERROR>", 1)
        ),
        Err(NativeMastersError::TallyReportedFailure)
    );
}

#[test]
fn an_absent_collection_is_not_an_empty_list() {
    for kind in NativeMasterKind::ALL {
        let text = response(kind);
        let (start, _, end) = collection_span(&text);
        let removed = format!("{}{}", &text[..start], &text[end..]);
        assert_eq!(
            parse(kind, &removed),
            Err(NativeMastersError::CollectionAbsent),
            "{kind:?}"
        );
    }
    let text = response(NativeMasterKind::Godowns);
    assert_eq!(
        parse(
            NativeMasterKind::Godowns,
            &text.replacen("</COLLECTION>", "</COLLECTION><COLLECTION></COLLECTION>", 1)
        ),
        Err(NativeMastersError::Malformed("masters_collection_repeated"))
    );
}

/// A kind with no master answers with a present, empty `COLLECTION` (captured
/// on a book without inventory): zero rows, not an absent collection.
#[test]
fn a_zero_row_capture_is_an_empty_collection_and_a_one_row_capture_is_one_bound_row() {
    for kind in [NativeMasterKind::Units, NativeMasterKind::StockGroups] {
        let text = reads_lab_response(kind);
        assert!(text.contains("<COLLECTION "), "{kind:?}: a present element");
        assert_eq!(
            parse_native_masters(kind, &text, READS_LAB_COMPANY),
            Ok(NativeMasters { rows: Vec::new() }),
            "{kind:?}"
        );
    }
    let godowns = parse_native_masters(
        NativeMasterKind::Godowns,
        &reads_lab_response(NativeMasterKind::Godowns),
        READS_LAB_COMPANY,
    )
    .unwrap();
    assert_eq!(godowns.rows.len(), 1);
    assert_eq!(godowns.rows[0].name, "Main Location");
    assert_eq!(
        godowns.rows[0].guid,
        format!("{READS_LAB_COMPANY}-00000063")
    );
    assert_eq!(
        parse(
            NativeMasterKind::Godowns,
            &reads_lab_response(NativeMasterKind::Godowns)
        ),
        Err(NativeMastersError::RowGuidForeign),
        "bound to its own company only"
    );
}

#[test]
fn a_truncated_response_is_malformed_not_an_empty_answer() {
    let text = response(NativeMasterKind::Godowns);
    let cut = &text[..text.rfind("</ENVELOPE>").unwrap()];
    assert_eq!(
        parse(NativeMasterKind::Godowns, cut),
        Err(NativeMastersError::Malformed(
            "masters_envelope_unterminated"
        ))
    );
    // Cut after the second row's last complete child: a row left open.
    let mid_row = &text[..text.rfind("</PARENT>").unwrap() + "</PARENT>".len()];
    assert_eq!(
        parse(NativeMasterKind::Godowns, mid_row),
        Err(NativeMastersError::Malformed("masters_row_unterminated"))
    );
}

#[test]
fn a_doctype_or_a_second_root_refuses() {
    let kind = NativeMasterKind::Godowns;
    let text = response(kind);
    assert_eq!(
        parse(kind, &format!("<!DOCTYPE ENVELOPE>{text}")),
        Err(NativeMastersError::Malformed("masters_doctype_forbidden"))
    );
    assert_eq!(
        parse(kind, &format!("{text}<ENVELOPE></ENVELOPE>")),
        Err(NativeMastersError::Malformed("masters_root_not_envelope"))
    );
    assert_eq!(
        parse(kind, &format!("{text}<ENVELOPE/>")),
        Err(NativeMastersError::Malformed("masters_root_not_envelope"))
    );
}

#[test]
fn a_child_that_is_not_the_kinds_element_refuses() {
    let kind = NativeMasterKind::Godowns;
    let text = edited(
        kind,
        "<GODOWN NAME=\"Factory Floor\"",
        "<LEDGER NAME=\"Factory Floor\"",
    );
    assert_eq!(parse(kind, &text), Err(NativeMastersError::ForeignChild));
    // Another kind's element is foreign too.
    assert_eq!(
        parse(
            NativeMasterKind::Units,
            &response(NativeMasterKind::StockGroups)
        ),
        Err(NativeMastersError::ForeignChild)
    );
    let empty_row = edited(
        kind,
        "<GODOWN NAME=\"Factory Floor\"",
        "<GODOWN/><GODOWN NAME=\"Factory Floor\"",
    );
    assert_eq!(
        parse(kind, &empty_row),
        Err(NativeMastersError::Malformed("masters_row_empty"))
    );
}

#[test]
fn a_row_without_a_name_refuses() {
    let kind = NativeMasterKind::Godowns;
    for name in ["", "   "] {
        assert_eq!(
            parse(
                kind,
                &edited(kind, "NAME=\"Factory Floor\"", &format!("NAME=\"{name}\""))
            ),
            Err(NativeMastersError::RowWithoutName),
            "{name:?}"
        );
    }
    assert_eq!(
        parse(kind, &edited(kind, "NAME=\"Factory Floor\" ", "")),
        Err(NativeMastersError::RowWithoutName)
    );
}

#[test]
fn a_row_guid_that_is_not_this_companys_refuses() {
    let kind = NativeMasterKind::Godowns;
    let guid = first_godown_guid();
    let element = |guid: &str| format!("<GUID TYPE=\"String\">{guid}</GUID>");
    for replacement in [
        element(&format!("{FOREIGN_COMPANY}-000000d4")),
        element(&format!("{COMPANY}-0000d4")),
        element(&format!("{COMPANY}-000000zz")),
        element(&guid.replace('-', "_")),
        String::new(),
    ] {
        assert_eq!(
            parse(kind, &edited(kind, &element(&guid), &replacement)),
            Err(NativeMastersError::RowGuidForeign),
            "{replacement}"
        );
    }
}

#[test]
fn a_row_not_bound_to_the_company_refuses() {
    let kind = NativeMasterKind::Godowns;
    assert_eq!(
        parse(
            kind,
            &edited(
                kind,
                &bound_element(COMPANY),
                &bound_element(FOREIGN_COMPANY)
            )
        ),
        Err(NativeMastersError::RowCompanyMismatch)
    );
    assert_eq!(
        parse(kind, &edited(kind, &bound_element(COMPANY), "")),
        Err(NativeMastersError::RowCompanyMismatch)
    );
    // Ignoring case, as the group snapshot does.
    assert!(parse(
        kind,
        &edited(
            kind,
            &bound_element(COMPANY),
            &bound_element(&COMPANY.to_uppercase())
        )
    )
    .is_ok());
}

#[test]
fn a_duplicate_guid_or_a_name_differing_only_in_case_refuses() {
    let kind = NativeMasterKind::Godowns;
    assert_eq!(
        parse(kind, &edited(kind, "-00000063</GUID>", "-000000d4</GUID>")),
        Err(NativeMastersError::DuplicateGuid)
    );
    assert_eq!(
        parse(
            kind,
            &edited(kind, "NAME=\"Main Location\"", "NAME=\"FACTORY FLOOR\"")
        ),
        Err(NativeMastersError::DuplicateName)
    );
}

#[test]
fn an_unreadable_identifier_or_repeated_field_refuses() {
    let kind = NativeMasterKind::Godowns;
    for (from, to, field) in [
        (
            "<MASTERID TYPE=\"Number\"> 212</MASTERID>",
            "<MASTERID TYPE=\"Number\"> twelve</MASTERID>",
            "master_id",
        ),
        ("<MASTERID TYPE=\"Number\"> 212</MASTERID>", "", "master_id"),
        (
            "<ALTERID TYPE=\"Number\"> 213</ALTERID>",
            "<ALTERID TYPE=\"Number\">-213</ALTERID>",
            "alter_id",
        ),
    ] {
        assert_eq!(
            parse(kind, &edited(kind, from, to)),
            Err(NativeMastersError::RowFieldInvalid(field)),
            "{to}"
        );
    }
    let parent = "<PARENT TYPE=\"String\">&#4; Primary</PARENT>";
    assert_eq!(
        parse(kind, &edited(kind, parent, &format!("{parent}{parent}"))),
        Err(NativeMastersError::Malformed("masters_row_field_repeated"))
    );
}

#[test]
fn a_blank_or_absent_parent_is_none() {
    let kind = NativeMasterKind::Godowns;
    let parent = "<PARENT TYPE=\"String\">&#4; Primary</PARENT>";
    for replacement in [
        "<PARENT TYPE=\"String\"></PARENT>",
        "<PARENT TYPE=\"String\"/>",
        "<PARENT TYPE=\"String\">  </PARENT>",
        "",
    ] {
        let rows = parse(kind, &edited(kind, parent, replacement))
            .unwrap()
            .rows;
        assert_eq!(rows[0].parent, None, "{replacement}");
        assert!(rows[1].parent.is_some());
    }
}

#[test]
fn a_flag_that_is_not_yes_or_no_refuses_and_an_absent_one_is_none() {
    let kind = NativeMasterKind::VoucherTypes;
    assert_eq!(
        parse(
            kind,
            &edited(
                kind,
                "<ISACTIVE TYPE=\"Logical\">No</ISACTIVE>",
                "<ISACTIVE TYPE=\"Logical\">Maybe</ISACTIVE>"
            )
        ),
        Err(NativeMastersError::RowFieldInvalid("is_active"))
    );
    assert_eq!(
        parse(
            kind,
            &edited(
                kind,
                "<ISOPTIONAL TYPE=\"Logical\">No</ISOPTIONAL>",
                "<ISOPTIONAL TYPE=\"Logical\"></ISOPTIONAL>"
            )
        ),
        Err(NativeMastersError::RowFieldInvalid("is_optional"))
    );
    let rows = parse(
        kind,
        &edited(kind, "<ISACTIVE TYPE=\"Logical\">No</ISACTIVE>", ""),
    )
    .unwrap()
    .rows;
    assert!(matches!(
        rows[0].detail,
        NativeMasterDetail::VoucherType { active: None, .. }
    ));
}

#[test]
fn an_unknown_numbering_method_is_reported_raw_never_refused() {
    let kind = NativeMasterKind::VoucherTypes;
    let default = "<NUMBERINGMETHOD TYPE=\"String\">Default</NUMBERINGMETHOD>";
    let rows = parse(
        kind,
        &edited(
            kind,
            default,
            "<NUMBERINGMETHOD TYPE=\"String\">Serial</NUMBERINGMETHOD>",
        ),
    )
    .unwrap()
    .rows;
    assert!(matches!(
        &rows[0].detail,
        NativeMasterDetail::VoucherType {
            numbering: Some(NativeNumberingMethod::Unrecognised(raw)),
            ..
        } if raw == "Serial"
    ));
    let rows = parse(kind, &edited(kind, default, "")).unwrap().rows;
    assert!(matches!(
        rows[0].detail,
        NativeMasterDetail::VoucherType {
            numbering: None,
            ..
        }
    ));
    // A present but empty element is a value Bridge does not recognise, not an
    // absent one.
    let empty = |replacement: &str| {
        parse(kind, &edited(kind, default, replacement))
            .unwrap()
            .rows[0]
            .detail
            .clone()
    };
    for replacement in [
        "<NUMBERINGMETHOD TYPE=\"String\"/>",
        "<NUMBERINGMETHOD TYPE=\"String\">  </NUMBERINGMETHOD>",
    ] {
        assert_eq!(
            empty(replacement),
            NativeMasterDetail::VoucherType {
                active: Some(false),
                optional: Some(false),
                numbering: Some(NativeNumberingMethod::Unrecognised(String::new())),
            },
            "{replacement}"
        );
    }
    // Trimmed, and bounded like a name.
    let padded = "<NUMBERINGMETHOD TYPE=\"String\">  Serial \r\n</NUMBERINGMETHOD>";
    assert!(matches!(
        empty(padded),
        NativeMasterDetail::VoucherType {
            numbering: Some(NativeNumberingMethod::Unrecognised(raw)),
            ..
        } if raw == "Serial"
    ));
    let numbering = |length: usize| {
        parse(
            kind,
            &edited(
                kind,
                default,
                &format!(
                    "<NUMBERINGMETHOD TYPE=\"String\">{}</NUMBERINGMETHOD>",
                    "z".repeat(length)
                ),
            ),
        )
    };
    assert!(numbering(MASTERS_ASSUMED_NAME_CHARS).is_ok());
    assert_eq!(
        numbering(MASTERS_ASSUMED_NAME_CHARS + 1),
        Err(NativeMastersError::RowExceedsBound)
    );
}

#[test]
fn a_unit_with_an_unreadable_field_refuses() {
    let kind = NativeMasterKind::Units;
    for (from, to, field) in [
        (
            "<DECIMALPLACES TYPE=\"Number\">0</DECIMALPLACES>",
            "<DECIMALPLACES TYPE=\"Number\">many</DECIMALPLACES>",
            "decimal_places",
        ),
        (
            "<DECIMALPLACES TYPE=\"Number\">0</DECIMALPLACES>",
            "",
            "decimal_places",
        ),
        (
            "<ISSIMPLEUNIT TYPE=\"Logical\">Yes</ISSIMPLEUNIT>",
            "<ISSIMPLEUNIT TYPE=\"Logical\">Sometimes</ISSIMPLEUNIT>",
            "is_simple_unit",
        ),
        (
            "<ISSIMPLEUNIT TYPE=\"Logical\">Yes</ISSIMPLEUNIT>",
            "",
            "is_simple_unit",
        ),
    ] {
        assert_eq!(
            parse(kind, &edited(kind, from, to)),
            Err(NativeMastersError::RowFieldInvalid(field)),
            "{to}"
        );
    }
}

#[test]
fn a_name_or_alias_count_over_the_assumed_bounds_refuses() {
    let kind = NativeMasterKind::Godowns;
    let attribute = |length: usize| {
        edited(
            kind,
            "NAME=\"Factory Floor\"",
            &format!("NAME=\"{}\"", "x".repeat(length)),
        )
    };
    assert!(parse(kind, &attribute(MASTERS_ASSUMED_NAME_CHARS)).is_ok());
    assert_eq!(
        parse(kind, &attribute(MASTERS_ASSUMED_NAME_CHARS + 1)),
        Err(NativeMastersError::RowExceedsBound)
    );
    let alias = |text: &str| {
        edited(
            kind,
            "<NAME>Factory Floor</NAME>",
            &format!("<NAME>{text}</NAME>"),
        )
    };
    assert!(parse(kind, &alias(&"y".repeat(MASTERS_ASSUMED_NAME_CHARS))).is_ok());
    assert_eq!(
        parse(kind, &alias(&"y".repeat(MASTERS_ASSUMED_NAME_CHARS + 1))),
        Err(NativeMastersError::RowExceedsBound)
    );
    let names = |count: usize| {
        edited(
            kind,
            "<NAME>Factory Floor</NAME>",
            &"<NAME>a</NAME>".repeat(count),
        )
    };
    assert!(parse(kind, &names(1 + MASTERS_ASSUMED_ALIASES)).is_ok());
    assert_eq!(
        parse(kind, &names(2 + MASTERS_ASSUMED_ALIASES)),
        Err(NativeMastersError::RowExceedsBound)
    );
    // Characters, not bytes: a multi-byte name at the bound is admitted.
    assert!(parse(kind, &alias(&"é".repeat(MASTERS_ASSUMED_NAME_CHARS))).is_ok());
    // The count is per row, across every language list the row carries: two
    // lists of three names each are one past the bound, though neither is.
    let two_lists = |per_list: usize| {
        let names = "<NAME>a</NAME>".repeat(per_list);
        edited(
            kind,
            "<LANGUAGENAME.LIST>",
            &format!(
                "<LANGUAGENAME.LIST><NAME.LIST>{names}</NAME.LIST></LANGUAGENAME.LIST><LANGUAGENAME.LIST>"
            ),
        )
        .replacen("<NAME>Factory Floor</NAME>", &names, 1)
    };
    assert!(parse(kind, &two_lists(2)).is_ok());
    assert_eq!(
        parse(kind, &two_lists(3)),
        Err(NativeMastersError::RowExceedsBound)
    );
}

#[test]
fn a_parent_reserved_name_alias_or_unit_name_over_the_bound_refuses() {
    let name_bound = |length: usize| "n".repeat(length);
    let cases: [(NativeMasterKind, &str, String, String); 4] = [
        (
            NativeMasterKind::Godowns,
            "<PARENT TYPE=\"String\">&#4; Primary</PARENT>",
            "<PARENT TYPE=\"String\">{}</PARENT>".into(),
            "PARENT".into(),
        ),
        (
            NativeMasterKind::Godowns,
            "RESERVEDNAME=\"\"",
            "RESERVEDNAME=\"{}\"".into(),
            "RESERVEDNAME".into(),
        ),
        (
            NativeMasterKind::VoucherTypes,
            "<ALIAS TYPE=\"String\"></ALIAS>",
            "<ALIAS TYPE=\"String\">{}</ALIAS>".into(),
            "ALIAS".into(),
        ),
        (
            NativeMasterKind::Units,
            "<NAME TYPE=\"String\">Box</NAME>",
            "<NAME TYPE=\"String\">{}</NAME>".into(),
            "unit NAME".into(),
        ),
    ];
    for (kind, from, template, label) in cases {
        let with = |length: usize| edited(kind, from, &template.replace("{}", &name_bound(length)));
        assert!(
            parse(kind, &with(MASTERS_ASSUMED_NAME_CHARS)).is_ok(),
            "{label} at the bound"
        );
        assert_eq!(
            parse(kind, &with(MASTERS_ASSUMED_NAME_CHARS + 1)),
            Err(NativeMastersError::RowExceedsBound),
            "{label} past the bound"
        );
    }
    // A unit's ORIGINALNAME is fetched but not exposed: it is still bounded,
    // and it changes nothing about the row.
    let kind = NativeMasterKind::Units;
    let simple = "<ISSIMPLEUNIT TYPE=\"Logical\">Yes</ISSIMPLEUNIT>";
    let original = |length: usize| {
        edited(
            kind,
            simple,
            &format!(
                "{simple}<ORIGINALNAME TYPE=\"String\">{}</ORIGINALNAME>",
                name_bound(length)
            ),
        )
    };
    assert_eq!(
        parse(kind, &original(MASTERS_ASSUMED_NAME_CHARS)).unwrap(),
        parse(kind, &response(kind)).unwrap()
    );
    assert_eq!(
        parse(kind, &original(MASTERS_ASSUMED_NAME_CHARS + 1)),
        Err(NativeMastersError::RowExceedsBound)
    );
}

#[test]
fn an_error_element_inside_a_language_list_refuses() {
    let kind = NativeMasterKind::Godowns;
    for element in ["<LINEERROR>x</LINEERROR>", "<ERROR/>"] {
        assert_eq!(
            parse(
                kind,
                &edited(
                    kind,
                    "<NAME>Factory Floor</NAME>",
                    &format!("<NAME>Factory Floor</NAME>{element}")
                )
            ),
            Err(NativeMastersError::TallyReportedFailure),
            "{element}"
        );
    }
}

#[test]
fn empty_alias_names_count_toward_the_alias_bound() {
    let kind = NativeMasterKind::Godowns;
    let names = |count: usize| edited(kind, "<NAME>Factory Floor</NAME>", &"<NAME/>".repeat(count));
    assert!(parse(kind, &names(1 + MASTERS_ASSUMED_ALIASES)).is_ok());
    assert_eq!(
        parse(kind, &names(2 + MASTERS_ASSUMED_ALIASES)),
        Err(NativeMastersError::RowExceedsBound)
    );
}

/// The row as a whole is bounded, whether or not the parser reads what makes
/// it long: a row of exactly the assumed worst length is admitted, one
/// character more is not.
#[test]
fn a_row_longer_than_the_assumed_worst_row_refuses() {
    for kind in NativeMasterKind::ALL {
        let text = response(kind);
        let element = String::from_utf8(kind.element().to_vec()).unwrap();
        let (open, close) = (format!("<{element} NAME="), format!("</{element}>"));
        let data = text.find("<DATA>").unwrap();
        let start = data + text[data..].find(&open).unwrap();
        let end = start + text[start..].find(&close).unwrap() + close.len();
        let row_chars = text[start..end].encode_utf16().count();
        let worst = masters_worst_row_bytes(kind) / 2;
        // Filler in an element the parser does not read, added to the first row.
        let padded = |chars: usize| {
            edited(
                kind,
                &close,
                &format!(
                    "<FILLER>{}</FILLER>{close}",
                    "f".repeat(chars - "<FILLER></FILLER>".len())
                ),
            )
        };
        let admitted = worst - row_chars;
        assert!(admitted > "<FILLER></FILLER>".len(), "{kind:?}");
        assert!(
            parse(kind, &padded(admitted)).is_ok(),
            "{kind:?} at the bound"
        );
        assert_eq!(
            parse(kind, &padded(admitted + 1)),
            Err(NativeMastersError::RowExceedsBound),
            "{kind:?} past the bound"
        );
    }
}

/// The row is measured in UTF-16 units, the unit the response is read in: a
/// name of astral characters is one `char` and two units.
#[test]
fn a_row_is_measured_in_utf16_units_not_characters() {
    let kind = NativeMasterKind::Godowns;
    let text = response(kind);
    let data = text.find("<DATA>").unwrap();
    let start = data + text[data..].find("<GODOWN NAME=").unwrap();
    let end = start + text[start..].find("</GODOWN>").unwrap() + "</GODOWN>".len();
    let row_units = text[start..end].encode_utf16().count();
    let worst = masters_worst_row_bytes(kind) / 2;
    let filler = "<FILLER></FILLER>".len();
    let room = worst - row_units - filler;
    let (emojis, pad) = (room / 2, room % 2);
    let padded = |emojis: usize| {
        edited(
            kind,
            "</GODOWN>",
            &format!(
                "<FILLER>{}{}</FILLER></GODOWN>",
                "\u{1F600}".repeat(emojis),
                "x".repeat(pad)
            ),
        )
    };
    assert!(parse(kind, &padded(emojis)).is_ok(), "at the bound");
    // One more emoji is two units over, though the row is still under the
    // bound in characters.
    assert!(row_units + filler + emojis + 1 + pad <= worst);
    assert_eq!(
        parse(kind, &padded(emojis + 1)),
        Err(NativeMastersError::RowExceedsBound)
    );
}

#[test]
fn text_outside_the_fields_refuses() {
    let kind = NativeMasterKind::Godowns;
    let row = "<GODOWN NAME=\"Factory Floor\"";
    let parent = "<PARENT TYPE=\"String\">&#4; Primary</PARENT>";
    for stray in ["stray", "&amp;", "<![CDATA[x]]>"] {
        for (from, label) in [(row, "under the collection"), (parent, "under a row")] {
            assert_eq!(
                parse(kind, &edited(kind, from, &format!("{stray}{from}"))),
                Err(NativeMastersError::Malformed("masters_unexpected_text")),
                "{stray} {label}"
            );
        }
    }
}

/// Only XML whitespace may sit between elements: a no-break space or another
/// Unicode space is text, which `str::trim` would have hidden.
#[test]
fn only_xml_whitespace_may_sit_between_elements() {
    let kind = NativeMasterKind::Godowns;
    let row = "<GODOWN NAME=\"Factory Floor\"";
    let parent = "<PARENT TYPE=\"String\">&#4; Primary</PARENT>";
    for (from, label) in [(row, "under the collection"), (parent, "under a row")] {
        for space in [" ", "\t", "\r", "\n", " \t\r\n"] {
            let parsed = parse(kind, &edited(kind, from, &format!("{space}{from}")));
            assert_eq!(
                parsed.map(|masters| masters.rows.len()),
                Ok(captured_rows(kind)),
                "{space:?} {label}"
            );
        }
        for not_xml_whitespace in ["\u{a0}", "\u{2003}", "\u{3000}", " \u{a0} "] {
            assert_eq!(
                parse(
                    kind,
                    &edited(kind, from, &format!("{not_xml_whitespace}{from}"))
                ),
                Err(NativeMastersError::Malformed("masters_unexpected_text")),
                "{not_xml_whitespace:?} {label}"
            );
        }
    }
}

/// Every company has predefined voucher types, so a present, empty voucher-type
/// collection is refused; the other kinds may answer with none (an empty cost-category
/// collection is refused by `an_empty_cost_category_answer_is_refused_because_the_primary_category_always_exists`).
#[test]
fn a_voucher_type_collection_with_no_rows_is_refused_and_the_other_kinds_may_be_empty() {
    let kind = NativeMasterKind::VoucherTypes;
    let text = response(kind);
    let (start, open_end, end) = collection_span(&text);
    let closing = end - "</COLLECTION>".len();
    let emptied = [
        // The captured opening and closing tags, every row removed.
        format!("{}{}", &text[..open_end], &text[closing..]),
        format!("{}<COLLECTION/>{}", &text[..start], &text[end..]),
    ];
    for empty in emptied {
        assert!(!empty.contains("<VOUCHERTYPE "));
        assert_eq!(
            parse(kind, &empty),
            Err(NativeMastersError::VoucherTypesEmpty)
        );
    }
    assert_eq!(
        NativeMastersError::VoucherTypesEmpty.code(),
        "masters_voucher_types_empty"
    );
    // The unedited capture still reads, and the other kinds still answer empty.
    assert_eq!(parse(kind, &text).unwrap().rows.len(), captured_rows(kind));
    for other in [NativeMasterKind::Units, NativeMasterKind::StockGroups] {
        assert_eq!(
            parse_native_masters(other, &reads_lab_response(other), READS_LAB_COMPANY),
            Ok(NativeMasters { rows: Vec::new() }),
            "{other:?}"
        );
    }
    let godowns = response(NativeMasterKind::Godowns);
    let (start, _, end) = collection_span(&godowns);
    assert_eq!(
        parse(
            NativeMasterKind::Godowns,
            &format!("{}<COLLECTION/>{}", &godowns[..start], &godowns[end..])
        ),
        Ok(NativeMasters { rows: Vec::new() })
    );
}

/// The stable code of each unreadable field, spelled out: a renamed label, or
/// one that fell through to the bare code, would break a caller that keys on it.
#[test]
fn every_unreadable_field_answers_with_its_own_code() {
    let godowns = NativeMasterKind::Godowns;
    let vouchers = NativeMasterKind::VoucherTypes;
    let units = NativeMasterKind::Units;
    let cases = [
        (
            godowns,
            edited(
                godowns,
                "<MASTERID TYPE=\"Number\"> 212</MASTERID>",
                "<MASTERID TYPE=\"Number\"> twelve</MASTERID>",
            ),
            "masters_row_field_invalid:master_id",
        ),
        (
            godowns,
            edited(
                godowns,
                "<ALTERID TYPE=\"Number\"> 213</ALTERID>",
                "<ALTERID TYPE=\"Number\">-213</ALTERID>",
            ),
            "masters_row_field_invalid:alter_id",
        ),
        (
            vouchers,
            edited(
                vouchers,
                "<ISACTIVE TYPE=\"Logical\">No</ISACTIVE>",
                "<ISACTIVE TYPE=\"Logical\">Maybe</ISACTIVE>",
            ),
            "masters_row_field_invalid:is_active",
        ),
        (
            vouchers,
            edited(
                vouchers,
                "<ISOPTIONAL TYPE=\"Logical\">No</ISOPTIONAL>",
                "<ISOPTIONAL TYPE=\"Logical\">Maybe</ISOPTIONAL>",
            ),
            "masters_row_field_invalid:is_optional",
        ),
        (
            units,
            edited(
                units,
                "<DECIMALPLACES TYPE=\"Number\">0</DECIMALPLACES>",
                "<DECIMALPLACES TYPE=\"Number\">many</DECIMALPLACES>",
            ),
            "masters_row_field_invalid:decimal_places",
        ),
        (
            units,
            edited(
                units,
                "<ISSIMPLEUNIT TYPE=\"Logical\">Yes</ISSIMPLEUNIT>",
                "<ISSIMPLEUNIT TYPE=\"Logical\">Sometimes</ISSIMPLEUNIT>",
            ),
            "masters_row_field_invalid:is_simple_unit",
        ),
    ];
    for (kind, text, code) in cases {
        let refused = parse(kind, &text).expect_err(code);
        assert!(
            matches!(refused, NativeMastersError::RowFieldInvalid(_)),
            "{code}: {refused:?}"
        );
        assert_eq!(refused.code(), code);
    }
}

/// One edit of captured text per refusal, each with its exact variant.
#[test]
fn every_refusal_branch_answers_with_its_exact_variant() {
    let godowns = NativeMasterKind::Godowns;
    let vouchers = NativeMasterKind::VoucherTypes;
    let malformed = |code: &'static str| Err(NativeMastersError::Malformed(code));
    let failure = || Err(NativeMastersError::TallyReportedFailure);
    let parent = "<PARENT TYPE=\"String\">&#4; Primary</PARENT>";
    let can_delete = "<CANDELETE TYPE=\"Logical\">No</CANDELETE>";
    let whole = response(godowns);
    let (start, _, end) = collection_span(&whole);
    // The text up to just after the first `marker` in the rows.
    let cut_after = |kind: NativeMasterKind, marker: &str| {
        let text = response(kind);
        let data = text.find("<DATA>").unwrap();
        let at = data + text[data..].find(marker).unwrap() + marker.len();
        text[..at].to_string()
    };
    let cases: Vec<(
        &str,
        NativeMasterKind,
        String,
        Result<NativeMasters, NativeMastersError>,
    )> = vec![
        // masters_xml_malformed, at each site a read can fail.
        (
            "mismatched end, envelope loop",
            godowns,
            whole.replacen("</BODY>", "</BODYX>", 1),
            malformed("masters_xml_malformed"),
        ),
        (
            "mismatched end, row loop",
            godowns,
            edited(godowns, "</GODOWN>", "</GODOWNX>"),
            malformed("masters_xml_malformed"),
        ),
        (
            "mismatched end, scalar text",
            godowns,
            edited(godowns, parent, "<PARENT TYPE=\"String\">x</PARENTX>"),
            malformed("masters_xml_malformed"),
        ),
        (
            "mismatched end, language list",
            godowns,
            edited(godowns, "</NAME.LIST>", "</NAME.LISTX>"),
            malformed("masters_xml_malformed"),
        ),
        (
            "mismatched end, skipped subtree",
            vouchers,
            edited(vouchers, can_delete, "<CANDELETE>No</CANDELETEX>"),
            malformed("masters_xml_malformed"),
        ),
        (
            "cut inside a tag",
            godowns,
            format!(
                "{}<PARE",
                cut_after(godowns, "<ALTERID TYPE=\"Number\"> 213</ALTERID>")
            ),
            malformed("masters_xml_malformed"),
        ),
        // masters_unexpected_close cannot be reached while quick-xml checks end
        // names: an end tag with nothing open is refused by the reader.
        (
            "end tag with nothing open",
            godowns,
            format!("{whole}</ENVELOPE>"),
            malformed("masters_xml_malformed"),
        ),
        (
            "no envelope",
            godowns,
            String::new(),
            malformed("masters_envelope_missing"),
        ),
        (
            "only whitespace",
            godowns,
            "  \r\n".into(),
            malformed("masters_envelope_missing"),
        ),
        // masters_xml_invalid_encoding cannot be reached: the parser reads a
        // `&str`, already valid UTF-8, so decoding cannot fail.
        (
            "nested markup in a scalar",
            godowns,
            edited(
                godowns,
                "<GUID TYPE=\"String\">",
                "<GUID TYPE=\"String\">a<X/>b",
            ),
            malformed("masters_scalar_not_text_only"),
        ),
        (
            "nested ERRORS is not a failure",
            godowns,
            edited(
                godowns,
                parent,
                "<PARENT TYPE=\"String\"><ERRORS/></PARENT>",
            ),
            malformed("masters_scalar_not_text_only"),
        ),
        (
            "nested ERROR is a failure",
            godowns,
            edited(
                godowns,
                parent,
                "<PARENT TYPE=\"String\">a<ERROR>x</ERROR></PARENT>",
            ),
            failure(),
        ),
        (
            "nested LINEERROR is a failure",
            godowns,
            edited(
                godowns,
                parent,
                "<PARENT TYPE=\"String\"><LINEERROR/></PARENT>",
            ),
            failure(),
        ),
        (
            "unknown entity in a scalar",
            godowns,
            edited(godowns, parent, "<PARENT TYPE=\"String\">&bogus;</PARENT>"),
            malformed("masters_xml_invalid_escape"),
        ),
        (
            "unquoted attribute",
            godowns,
            edited(
                godowns,
                "NAME=\"Factory Floor\" RESERVEDNAME=\"\"",
                "NAME=Factory RESERVEDNAME=\"\"",
            ),
            malformed("masters_attribute_malformed"),
        ),
        (
            "unknown entity in an attribute",
            godowns,
            edited(godowns, "NAME=\"Factory Floor\"", "NAME=\"a&bogus;b\""),
            malformed("masters_attribute_malformed"),
        ),
        (
            "repeated field, two empty",
            godowns,
            edited(
                godowns,
                parent,
                "<PARENT TYPE=\"String\"/><PARENT TYPE=\"String\"/>",
            ),
            malformed("masters_row_field_repeated"),
        ),
        (
            "repeated field, empty after full",
            godowns,
            edited(
                godowns,
                parent,
                &format!("{parent}<PARENT TYPE=\"String\"/>"),
            ),
            malformed("masters_row_field_repeated"),
        ),
        (
            "cut inside a language list",
            godowns,
            cut_after(godowns, "<NAME>Factory Floor</NAME>"),
            malformed("masters_row_unterminated"),
        ),
        (
            "cut inside a skipped subtree",
            vouchers,
            cut_after(vouchers, "<CANDELETE TYPE=\"Logical\">"),
            malformed("masters_row_unterminated"),
        ),
        (
            "error inside a skipped subtree, empty",
            vouchers,
            edited(vouchers, can_delete, "<CANDELETE><LINEERROR/></CANDELETE>"),
            failure(),
        ),
        (
            "error inside a skipped subtree, full",
            vouchers,
            edited(
                vouchers,
                can_delete,
                "<CANDELETE><ERROR>x</ERROR></CANDELETE>",
            ),
            failure(),
        ),
        (
            "empty ERROR in a row",
            godowns,
            edited(godowns, "</GODOWN>", "<ERROR/></GODOWN>"),
            failure(),
        ),
        (
            "empty LINEERROR in a row",
            godowns,
            edited(godowns, "</GODOWN>", "<LINEERROR/></GODOWN>"),
            failure(),
        ),
        (
            "empty LINEERROR at the top",
            godowns,
            whole.replacen("<BODY>", "<BODY><LINEERROR/>", 1),
            failure(),
        ),
        (
            "empty ERROR at the top",
            godowns,
            whole.replacen("<BODY>", "<BODY><ERROR/>", 1),
            failure(),
        ),
        // Present and empty, not absent.
        (
            "self-closing collection",
            godowns,
            format!("{}<COLLECTION/>{}", &whole[..start], &whole[end..]),
            Ok(NativeMasters { rows: Vec::new() }),
        ),
    ];
    for (label, kind, text, expected) in cases {
        assert_eq!(parse(kind, &text), expected, "{label}");
    }
}

#[test]
fn every_fixture_row_fits_the_worst_row_bytes_of_its_kind() {
    for kind in NativeMasterKind::ALL {
        let text = response(kind);
        let element = String::from_utf8(kind.element().to_vec()).unwrap();
        let (open, close) = (format!("<{element} NAME="), format!("</{element}>"));
        let mut rows = 0;
        let mut rest = text.as_str();
        while let Some(start) = rest.find(&open) {
            let end = start + rest[start..].find(&close).unwrap() + close.len();
            let bytes = rest[start..end].encode_utf16().count() * 2;
            assert!(bytes <= masters_worst_row_bytes(kind), "{kind:?}: {bytes}");
            rows += 1;
            rest = &rest[end..];
        }
        assert_eq!(rows, captured_rows(kind), "{kind:?}: rows measured");
    }
}

#[test]
fn the_worst_row_bytes_follow_the_documented_arithmetic() {
    // 2 * (fixed + 6 * 128 * (slots + 4)), worked by hand: fixed 1,200, 800,
    // 700, 750, 800, 950 and slots 5, 4, 4, 4, 5, 3.
    for (kind, bytes) in [
        (NativeMasterKind::VoucherTypes, 16_224),
        (NativeMasterKind::Godowns, 13_888),
        (NativeMasterKind::Units, 13_688),
        (NativeMasterKind::StockGroups, 13_788),
        (NativeMasterKind::CostCentres, 15_424),
        (NativeMasterKind::CostCategories, 12_652),
    ] {
        assert_eq!(masters_worst_row_bytes(kind), bytes, "{kind:?}");
    }
    assert_eq!(MASTERS_ASSUMED_NAME_CHARS, 128);
    assert_eq!(MASTERS_ASSUMED_ALIASES, 4);
    assert_eq!(MASTERS_RESPONSE_BUDGET_BYTES, 16_000_000);
}

// ---- cost centres and cost categories (captured 7 Oct 2026, flag No, rows present) ----

/// `BRIDGE CORPUS FOREX`, a book with no cost centre defined.
const FOREX_COMPANY: &str = "b14e9b2d-8a63-4779-804d-25d59eb787eb";

fn forex_cost_centres() -> String {
    utf16le(
        &include_bytes!(
            "../tests/fixtures/masters_cost_centres_corpus_forex_empty_live.utf16le.xml"
        )[..],
    )
}

#[test]
fn the_cost_centre_capture_reads_both_centres_with_their_category_and_root_parent() {
    let rows = parse(
        NativeMasterKind::CostCentres,
        &response(NativeMasterKind::CostCentres),
    )
    .unwrap()
    .rows;
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "Assembly");
    assert_eq!(rows[0].guid, format!("{COMPANY}-00000109"));
    assert_eq!((rows[0].master_id, rows[0].alter_id), (265, 267));
    assert_eq!(
        rows[0].detail,
        NativeMasterDetail::CostCentre {
            category: "Business Line".to_string()
        }
    );
    assert!(is_tally_reserved_root(rows[0].parent.as_deref().unwrap()));
    assert_eq!(rows[1].name, "Trading");
    assert_eq!((rows[1].master_id, rows[1].alter_id), (217, 218));
}

#[test]
fn a_cost_centre_without_a_category_or_with_a_blank_one_is_refused_not_read_as_none() {
    // Every captured centre carried a category (the default one where none was chosen), so a
    // missing or blank one is an answer that was not read, not a centre with no category.
    for replacement in ["", "<CATEGORY TYPE=\"String\">  </CATEGORY>"] {
        let changed = edited(
            NativeMasterKind::CostCentres,
            "<CATEGORY TYPE=\"String\">Business Line</CATEGORY>",
            replacement,
        );
        assert_eq!(
            parse(NativeMasterKind::CostCentres, &changed),
            Err(NativeMastersError::RowFieldInvalid("category")),
            "{replacement:?}"
        );
    }
    assert_eq!(
        NativeMastersError::RowFieldInvalid("category").code(),
        "masters_row_field_invalid:category"
    );
}

#[test]
fn the_two_cost_collections_must_name_their_own_type_so_an_unresolved_one_is_not_read_as_empty() {
    // The `MSTDEPTYPE` of every captured answer (32 for centres, 16 for categories), the zero-row
    // answer included. A collection that does not carry its own is refused, whether it holds rows
    // or none: an empty answer that was not resolved to the type asked for says nothing.
    let type_error = Err(NativeMastersError::Malformed(
        "masters_collection_type_unexpected",
    ));
    for (kind, own, other) in [
        (NativeMasterKind::CostCentres, "32", "16"),
        (NativeMasterKind::CostCategories, "16", "32"),
    ] {
        let text = response(kind);
        let own_attribute = format!("MSTDEPTYPE=\"{own}\"");
        assert!(text.contains(&own_attribute), "{kind:?}");
        let wrong = text.replace(&own_attribute, &format!("MSTDEPTYPE=\"{other}\""));
        let absent = text.replace(&own_attribute, "");
        assert_eq!(parse(kind, &wrong), type_error.clone(), "{kind:?} wrong");
        assert_eq!(parse(kind, &absent), type_error.clone(), "{kind:?} absent");
        // The right type parses, and it is the capture itself.
        assert!(parse(kind, &text).is_ok(), "{kind:?}");
    }
    // The captured zero-row cost-centre answer carries its type and is a valid answer; the same
    // answer without it is refused, and so is a self-closed collection that names none.
    assert!(forex_cost_centres().contains("MSTDEPTYPE=\"32\""));
    let no_type = forex_cost_centres().replace("MSTDEPTYPE=\"32\"", "");
    assert_eq!(
        parse_native_masters(NativeMasterKind::CostCentres, &no_type, FOREX_COMPANY),
        type_error
    );
    let text = response(NativeMasterKind::CostCategories);
    let (start, _, end) = collection_span(&text);
    let self_closed = format!("{}<COLLECTION/>{}", &text[..start], &text[end..]);
    assert_eq!(
        parse(NativeMasterKind::CostCategories, &self_closed),
        type_error
    );
    // A self-closed collection that does name its type is an empty answer, classified like the
    // open-and-close form: refused for categories, a zero-row answer for centres.
    let typed = format!(
        "{}<COLLECTION MSTDEPTYPE=\"16\"/>{}",
        &text[..start],
        &text[end..]
    );
    assert_eq!(
        parse(NativeMasterKind::CostCategories, &typed),
        Err(NativeMastersError::CostCategoriesEmpty)
    );
    let text = response(NativeMasterKind::CostCentres);
    let (start, _, end) = collection_span(&text);
    let typed = format!(
        "{}<COLLECTION MSTDEPTYPE=\"32\"/>{}",
        &text[..start],
        &text[end..]
    );
    assert!(parse(NativeMasterKind::CostCentres, &typed)
        .unwrap()
        .rows
        .is_empty());
}

#[test]
fn the_cost_category_capture_reads_the_three_allocation_flags() {
    let rows = parse(
        NativeMasterKind::CostCategories,
        &response(NativeMasterKind::CostCategories),
    )
    .unwrap()
    .rows;
    assert_eq!(rows.len(), 2);
    let find = |name: &str| rows.iter().find(|row| row.name == name).unwrap();
    assert_eq!(
        find("Business Line").detail,
        NativeMasterDetail::CostCategory {
            allocates_revenue: true,
            allocates_non_revenue: false,
            affects_stock: false
        }
    );
    assert_eq!(
        find("Primary Cost Category").detail,
        NativeMasterDetail::CostCategory {
            allocates_revenue: true,
            allocates_non_revenue: true,
            affects_stock: false
        }
    );
    // A category has no parent.
    assert_eq!(find("Business Line").parent, None);
}

#[test]
fn a_book_with_no_cost_centre_answers_an_empty_collection_that_is_a_zero_row_answer() {
    // No row is read here, so no company binding is exercised: only that a typed, empty answer
    // is a zero-row answer.
    let rows = parse_native_masters(
        NativeMasterKind::CostCentres,
        &forex_cost_centres(),
        FOREX_COMPANY,
    )
    .unwrap()
    .rows;
    assert!(rows.is_empty());
}

#[test]
fn an_empty_cost_category_answer_is_refused_because_the_primary_category_always_exists() {
    let (start, open_end, end) = collection_span(&response(NativeMasterKind::CostCategories));
    let text = response(NativeMasterKind::CostCategories);
    let empty = format!(
        "{}{}",
        &text[..open_end],
        &text[end - "</COLLECTION>".len()..]
    );
    assert!(empty.len() < text.len() && start < open_end);
    assert_eq!(
        parse(NativeMasterKind::CostCategories, &empty),
        Err(NativeMastersError::CostCategoriesEmpty)
    );
    assert_eq!(
        NativeMastersError::CostCategoriesEmpty.code(),
        "masters_cost_categories_empty"
    );
}

#[test]
fn a_cost_category_row_that_carries_a_parent_has_none() {
    // A category has no parent: a PARENT element on the wire is not read as one.
    let text = response(NativeMasterKind::CostCategories);
    let tag = "<COSTCATEGORY NAME=\"Business Line\"";
    let at = text.find(tag).unwrap();
    let open_end = at + text[at..].find('>').unwrap() + 1;
    let changed = format!(
        "{}<PARENT TYPE=\"String\">Business Line</PARENT>{}",
        &text[..open_end],
        &text[open_end..]
    );
    assert_ne!(changed, text);
    let rows = parse(NativeMasterKind::CostCategories, &changed)
        .unwrap()
        .rows;
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| row.parent.is_none()), "{rows:?}");
}

#[test]
fn an_absent_or_unreadable_allocation_flag_is_refused_not_read_as_no() {
    for (field, label, code) in [
        (
            "ALLOCATEREVENUE",
            "allocate_revenue",
            "masters_row_field_invalid:allocate_revenue",
        ),
        (
            "ALLOCATENONREVENUE",
            "allocate_non_revenue",
            "masters_row_field_invalid:allocate_non_revenue",
        ),
        (
            "AFFECTSSTOCK",
            "affects_stock",
            "masters_row_field_invalid:affects_stock",
        ),
    ] {
        let tag = format!("<{field} TYPE=\"Logical\">No</{field}>");
        let tag_yes = format!("<{field} TYPE=\"Logical\">Yes</{field}>");
        let from = if response(NativeMasterKind::CostCategories).contains(&tag) {
            tag.clone()
        } else {
            tag_yes
        };
        let absent = edited(NativeMasterKind::CostCategories, &from, "");
        assert_eq!(
            parse(NativeMasterKind::CostCategories, &absent),
            Err(NativeMastersError::RowFieldInvalid(label)),
            "{field} absent"
        );
        let odd = edited(
            NativeMasterKind::CostCategories,
            &from,
            &format!("<{field} TYPE=\"Logical\">Maybe</{field}>"),
        );
        let error = parse(NativeMasterKind::CostCategories, &odd).unwrap_err();
        assert_eq!(error, NativeMastersError::RowFieldInvalid(label));
        assert_eq!(error.code(), code);
    }
}

#[test]
fn a_cost_centre_row_from_another_company_and_a_repeated_category_are_refused() {
    assert_eq!(
        parse_native_masters(
            NativeMasterKind::CostCentres,
            &response(NativeMasterKind::CostCentres),
            FOREIGN_COMPANY
        ),
        Err(NativeMastersError::RowGuidForeign)
    );
    let repeated = edited(
        NativeMasterKind::CostCentres,
        "<CATEGORY TYPE=\"String\">Business Line</CATEGORY>",
        "<CATEGORY TYPE=\"String\">Business Line</CATEGORY><CATEGORY TYPE=\"String\">X</CATEGORY>",
    );
    assert_eq!(
        parse(NativeMasterKind::CostCentres, &repeated),
        Err(NativeMastersError::Malformed("masters_row_field_repeated"))
    );
}

#[test]
fn a_cost_centre_category_over_the_name_bound_is_refused() {
    let long = "x".repeat(MASTERS_ASSUMED_NAME_CHARS + 1);
    let edited = edited(
        NativeMasterKind::CostCentres,
        "<CATEGORY TYPE=\"String\">Business Line</CATEGORY>",
        &format!("<CATEGORY TYPE=\"String\">{long}</CATEGORY>"),
    );
    assert_eq!(
        parse(NativeMasterKind::CostCentres, &edited),
        Err(NativeMastersError::RowExceedsBound)
    );
}

// ---- the same collections on a book whose Cost Centres setting reads Yes (captured 7 Oct 2026 in a separate sitting, scrubbed) ----

/// The parity book's synthetic company GUID, as scrubbed in the fixtures.
const PARITY_COMPANY: &str = "7c0de000-0000-4000-8000-0000000000a1";

fn parity(bytes: &[u8]) -> String {
    utf16le(bytes)
}

#[test]
fn a_book_with_the_setting_at_yes_reads_its_three_centres_one_under_another() {
    let rows = parse_native_masters(
        NativeMasterKind::CostCentres,
        &parity(
            &include_bytes!(
                "../tests/fixtures/masters_cost_centres_parity_flag_yes_live.utf16le.xml"
            )[..],
        ),
        PARITY_COMPANY,
    )
    .unwrap()
    .rows;
    assert_eq!(
        rows.iter().map(|row| row.name.as_str()).collect::<Vec<_>>(),
        ["Parity CC A", "Parity CC A1", "Parity CC B"]
    );
    // The two top-level centres carry the reserved root marker; the child names its parent centre.
    assert!(is_tally_reserved_root(rows[0].parent.as_deref().unwrap()));
    assert_eq!(rows[1].parent.as_deref(), Some("Parity CC A"));
    assert!(!is_tally_reserved_root(rows[1].parent.as_deref().unwrap()));
    assert!(is_tally_reserved_root(rows[2].parent.as_deref().unwrap()));
    // The default category is a category like any other: kept, not dropped.
    for row in &rows {
        assert_eq!(
            row.detail,
            NativeMasterDetail::CostCentre {
                category: "Primary Cost Category".to_string()
            }
        );
    }
    assert_eq!((rows[0].master_id, rows[0].alter_id), (206, 211));
    assert_eq!((rows[1].master_id, rows[1].alter_id), (208, 213));
}

#[test]
fn the_parity_book_cost_category_is_the_predefined_one_with_its_flags() {
    let rows = parse_native_masters(
        NativeMasterKind::CostCategories,
        &parity(
            &include_bytes!(
                "../tests/fixtures/masters_cost_categories_parity_flag_yes_live.utf16le.xml"
            )[..],
        ),
        PARITY_COMPANY,
    )
    .unwrap()
    .rows;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "Primary Cost Category");
    assert_eq!(rows[0].parent, None);
    assert_eq!(
        rows[0].detail,
        NativeMasterDetail::CostCategory {
            allocates_revenue: true,
            allocates_non_revenue: true,
            affects_stock: false,
        }
    );
}

#[test]
fn the_flag_yes_cost_centre_answer_is_company_bound() {
    let centres = parity(
        &include_bytes!("../tests/fixtures/masters_cost_centres_parity_flag_yes_live.utf16le.xml")
            [..],
    );
    assert_eq!(
        parse_native_masters(NativeMasterKind::CostCentres, &centres, COMPANY),
        Err(NativeMastersError::RowGuidForeign)
    );
}
