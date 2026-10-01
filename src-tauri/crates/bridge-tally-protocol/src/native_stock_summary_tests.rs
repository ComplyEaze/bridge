//! The stock summary parsers and request builders against the live captures
//! (`tests/fixtures/stock_*`, `company_inventory_flags_*`). Every negative case
//! edits captured text, or the parsed rows of a capture, inside the test; none
//! uses a hand-written response.
use super::*;
use crate::native_statement_reports::{render_native_statement_request, NativeStatementKind};
use crate::outstandings_shared::DateBoundaryProfile;
use crate::xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName, ValidatedDateRange};

const COMPANY: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const FOREIGN_COMPANY: &str = "ffffffff-b835-4bff-89dd-8a6af138c346";
const LAB: &str = "BRIDGE SHAPE LAB";

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

fn items_response() -> String {
    utf16le(&include_bytes!("../tests/fixtures/stock_items_shape_lab_fy_live.utf16le.xml")[..])
}

fn report_response() -> String {
    utf16le(
        &include_bytes!("../tests/fixtures/stock_summary_report_shape_lab_fy_live.utf16le.xml")[..],
    )
}

fn flags_response() -> String {
    utf16le(
        &include_bytes!("../tests/fixtures/company_inventory_flags_shape_lab_live.utf16le.xml")[..],
    )
}

fn parse(text: &str) -> Result<NativeStockItems, NativeStockError> {
    parse_native_stock_items(text, COMPANY)
}

/// `text` with the first `from` at or after its `DATA` element, where the rows
/// are (the header carries `<STOCKITEM>0</STOCKITEM>` too), replaced by `to`.
fn edited(text: &str, from: &str, to: &str) -> String {
    let data = text.find("<DATA>").expect("the capture has a DATA element");
    let at = data
        + text[data..]
            .find(from)
            .unwrap_or_else(|| panic!("the capture has no {from}"));
    format!("{}{}{}", &text[..at], to, &text[at + from.len()..])
}

/// `text` with the first `from` replaced by `to`, which must be there.
fn replaced(text: &str, from: &str, to: &str) -> String {
    assert!(text.contains(from), "the capture has no {from}");
    text.replacen(from, to, 1)
}

/// The item capture with the first row's (Carton Box Small) closing quantity
/// element as given: `Some(text)` for its text, `None` to leave it out.
fn carton_closing_quantity(text: Option<&str>) -> String {
    replaced(
        &items_response(),
        "<CLOSINGBALANCE TYPE=\"Quantity\"> 100 Box</CLOSINGBALANCE>",
        &text.map_or(String::new(), |text| {
            format!("<CLOSINGBALANCE TYPE=\"Quantity\">{text}</CLOSINGBALANCE>")
        }),
    )
}

/// As [`carton_closing_quantity`], for the first row's closing value.
fn carton_closing_value(text: Option<&str>) -> String {
    replaced(
        &items_response(),
        "<CLOSINGVALUE TYPE=\"Amount\">2500.00</CLOSINGVALUE>",
        &text.map_or(String::new(), |text| {
            format!("<CLOSINGVALUE TYPE=\"Amount\">{text}</CLOSINGVALUE>")
        }),
    )
}

fn decimal(text: &str) -> ExactDecimal {
    ExactDecimal::parse(text).expect("a plain decimal")
}

fn quantity_of(amount: &str, unit: &str) -> Option<NativeStockQuantity> {
    Some(NativeStockQuantity {
        amount: decimal(amount),
        unit: unit.to_string(),
    })
}

fn item<'a>(items: &'a NativeStockItems, name: &str) -> &'a NativeStockItem {
    items
        .rows
        .iter()
        .find(|row| row.name == name)
        .unwrap_or_else(|| panic!("no item {name}"))
}

fn report_of(text: &str) -> Result<NativeStockReport, NativeStockError> {
    parse_native_stock_summary_report(text)
}

fn gate(items: &str, report: &str) -> NativeStockGate {
    gate_stock_summary(parse(items).unwrap().rows, &report_of(report).unwrap()).unwrap()
}

#[test]
fn every_request_is_byte_equal_to_its_committed_fixture() {
    let period = NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20250401").unwrap(),
        TallyDate::parse("20260331").unwrap(),
    )
    .unwrap();
    let report_request = render_native_stock_summary_request(LAB, &period);
    assert_eq!(
        report_request,
        utf16le(
            &include_bytes!("../tests/fixtures/stock_summary_report_fy_request.utf16le.xml")[..]
        )
    );
    // The same envelope as the built-in statements, by construction: only the
    // report name differs.
    assert_eq!(
        report_request,
        render_native_statement_request(NativeStatementKind::BalanceSheet, LAB, &period)
            .replace("Balance Sheet", "Stock Summary")
    );
    let company = ValidatedCompanyName::new(LAB).unwrap();
    let range = ValidatedDateRange::new("20250401", "20260331").unwrap();
    assert_eq!(
        ReadOnlyProfile::AuditStockItemsV1 {
            company: &company,
            period: &range
        }
        .render(),
        utf16le(&include_bytes!("../tests/fixtures/stock_items_fy_request.utf16le.xml")[..])
    );
    assert_eq!(
        render_company_inventory_flags_request(LAB, COMPANY).unwrap(),
        utf16le(
            &include_bytes!("../tests/fixtures/company_inventory_flags_request.utf16le.xml")[..]
        )
    );
}

#[test]
fn the_flags_request_refuses_a_guid_that_could_break_its_formula_and_escapes_the_name() {
    for guid in ["", "abc\" OR 1", "a b", "ab;c", "ab\u{fc}", "a\"b"] {
        assert_eq!(
            render_company_inventory_flags_request(LAB, guid),
            Err(NativeStockError::CompanyFlagsGuidUnsupported),
            "{guid:?}"
        );
    }
    let request = render_company_inventory_flags_request("A & <B>", COMPANY).unwrap();
    assert!(request.contains("<SVCURRENTCOMPANY>A &amp; &lt;B&gt;</SVCURRENTCOMPANY>"));
}

#[test]
fn the_flags_capture_reads_each_flag_from_its_own_element() {
    assert_eq!(
        parse_company_inventory_flags(&flags_response(), COMPANY),
        Ok(NativeInventoryFlags {
            integrated: NativeFlag::Yes,
            inventory_on: NativeFlag::Yes,
            batchwise: NativeFlag::Yes,
        })
    );
    // Every flag differs, so a flag read from another's element shows.
    let text = replaced(
        &flags_response(),
        "<ISINTEGRATED TYPE=\"Logical\">Yes</ISINTEGRATED>",
        "<ISINTEGRATED TYPE=\"Logical\">No</ISINTEGRATED>",
    );
    let text = replaced(
        &text,
        "<ISBATCHWISEON TYPE=\"Logical\">Yes</ISBATCHWISEON>",
        "",
    );
    assert_eq!(
        parse_company_inventory_flags(&text, COMPANY),
        Ok(NativeInventoryFlags {
            integrated: NativeFlag::No,
            inventory_on: NativeFlag::Yes,
            batchwise: NativeFlag::Unknown,
        })
    );
    // An empty element is unknown too, and the counts are fetched but never read.
    let text = replaced(
        &flags_response(),
        "<ISINVENTORYON TYPE=\"Logical\">Yes</ISINVENTORYON>",
        "<ISINVENTORYON TYPE=\"Logical\"></ISINVENTORYON>",
    );
    let text = replaced(
        &text,
        "<NUMSTOCKITEMS TYPE=\"Number\"> 11</NUMSTOCKITEMS>",
        "<NUMSTOCKITEMS TYPE=\"Number\">not a number</NUMSTOCKITEMS>",
    );
    assert_eq!(
        parse_company_inventory_flags(&text, COMPANY)
            .unwrap()
            .inventory_on,
        NativeFlag::Unknown
    );
    // The company GUID is compared ignoring ASCII case.
    assert!(parse_company_inventory_flags(&flags_response(), &COMPANY.to_uppercase()).is_ok());
}

#[test]
fn a_flag_other_than_yes_or_no_refuses_naming_the_flag() {
    for (element, label) in [
        ("ISINTEGRATED", "is_integrated"),
        ("ISINVENTORYON", "is_inventory_on"),
        ("ISBATCHWISEON", "is_batchwise_on"),
    ] {
        let text = replaced(
            &flags_response(),
            &format!("<{element} TYPE=\"Logical\">Yes</{element}>"),
            &format!("<{element} TYPE=\"Logical\">Maybe</{element}>"),
        );
        assert_eq!(
            parse_company_inventory_flags(&text, COMPANY),
            Err(NativeStockError::FlagInvalid(label)),
            "{element}"
        );
    }
    assert_eq!(
        NativeStockError::FlagInvalid("is_inventory_on").code(),
        "stock_flag_invalid:is_inventory_on"
    );
}

#[test]
fn the_flags_read_is_one_row_of_this_company_or_nothing() {
    let text = flags_response();
    // The header's counts carry `<COMPANY>0</COMPANY>`, so the row is searched
    // for from its own opening.
    let start = text.find("<COMPANY NAME=").unwrap();
    let end = start + text[start..].find("</COMPANY>").unwrap() + "</COMPANY>".len();
    let row = &text[start..end];
    let not_one_row = Err(NativeStockError::CompanyFlagsNotOneRow);
    // Zero rows: a present, empty collection.
    assert_eq!(
        parse_company_inventory_flags(&format!("{}{}", &text[..start], &text[end..]), COMPANY),
        not_one_row
    );
    // Two rows: a year-split sibling shares the GUID (§9.11b).
    assert_eq!(
        parse_company_inventory_flags(&replaced(&text, row, &format!("{row}{row}")), COMPANY),
        not_one_row
    );
    // A row that is another company's, whichever way that shows.
    assert_eq!(
        parse_company_inventory_flags(&text, FOREIGN_COMPANY),
        not_one_row
    );
    assert_eq!(
        parse_company_inventory_flags(
            &replaced(&text, COMPANY, "ffffffff-0000-4000-8000-000000000000"),
            COMPANY
        ),
        not_one_row
    );
    // A row with no GUID names no company.
    assert_eq!(
        parse_company_inventory_flags(
            &replaced(
                &text,
                &format!("<GUID TYPE=\"String\">{COMPANY}</GUID>"),
                ""
            ),
            COMPANY
        ),
        not_one_row
    );
    // Another company's row beside this one is not exactly one row.
    let other = row.replace(COMPANY, FOREIGN_COMPANY);
    assert_eq!(
        parse_company_inventory_flags(&replaced(&text, row, &format!("{row}{other}")), COMPANY),
        not_one_row
    );
}

#[test]
fn the_flags_envelope_is_as_closed_as_the_collection_envelope() {
    let text = flags_response();
    assert_eq!(
        parse_company_inventory_flags(
            &replaced(&text, "<STATUS>1</STATUS>", "<STATUS>0</STATUS>"),
            COMPANY
        ),
        Err(NativeStockError::TallyReportedFailure)
    );
    let start = text.find("<COLLECTION").unwrap();
    let end = text.find("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    assert_eq!(
        parse_company_inventory_flags(&format!("{}{}", &text[..start], &text[end..]), COMPANY),
        Err(NativeStockError::CollectionAbsent)
    );
    assert_eq!(
        parse_company_inventory_flags(&replaced(&text, "<COMPANY NAME=", "<LEDGER NAME="), COMPANY),
        Err(NativeStockError::ForeignChild)
    );
}

#[test]
fn the_capture_reads_eleven_rows_bound_by_guid_prefix_with_quantities_and_signed_values() {
    let items = parse(&items_response()).unwrap();
    assert_eq!(items.rows.len(), 11);
    for row in &items.rows {
        assert!(row.guid.starts_with(&format!("{COMPANY}-")), "{}", row.name);
    }
    // The same capture read for another company binds none of its rows.
    assert_eq!(
        parse_native_stock_items(&items_response(), FOREIGN_COMPANY),
        Err(NativeStockError::RowGuidForeign)
    );
    let carton = item(&items, "Carton Box Small");
    assert_eq!(carton.guid, format!("{COMPANY}-00000110"));
    assert_eq!(carton.parent.as_deref(), Some("Packaging"));
    assert_eq!(carton.base_unit.as_deref(), Some("Box"));
    assert_eq!(carton.closing.quantity, quantity_of("100", "Box"));
    assert_eq!(carton.closing.value, Some(decimal("2500.00")));
    // Opening and closing are read from their own elements, and the sign of a
    // value is kept, never flipped.
    let caustic = item(&items, "Caustic Soda Flakes");
    assert_eq!(caustic.opening.quantity, quantity_of("200.000", "Kgs"));
    assert_eq!(caustic.opening.value, Some(decimal("9000.00")));
    assert_eq!(caustic.closing.quantity, quantity_of("400.000", "Kgs"));
    assert_eq!(caustic.closing.value, Some(decimal("-1000.00")));
    let kit = item(&items, "Cleaning Kit A");
    assert_eq!(kit.opening.quantity, quantity_of("25", "Nos"));
    assert_eq!(kit.closing.quantity, quantity_of("12", "Nos"));
    // An empty element is None; a present zero value is not.
    let empty = item(&items, "Cleaning Kit B");
    assert_eq!(empty.closing.quantity, None);
    assert_eq!(empty.closing.value, None);
    assert_eq!(empty.opening.quantity, None);
    assert_eq!(empty.opening.value, Some(decimal("0.00")));
    let negative = item(&items, "Zero Stock Item");
    assert_eq!(negative.closing.quantity, quantity_of("-50.000", "Kgs"));
    assert_eq!(negative.closing.value, None);
}

#[test]
fn a_quantity_is_a_number_a_single_space_and_a_unit_or_nothing() {
    let closing = |text: Option<&str>| {
        parse(&carton_closing_quantity(text)).map(|items| items.rows[0].closing.quantity.clone())
    };
    let refused = Err(NativeStockError::QuantityUnparseable);
    // Absent, empty and whitespace are no quantity, not zero.
    assert_eq!(closing(None), Ok(None));
    assert_eq!(closing(Some("")), Ok(None));
    assert_eq!(closing(Some("   ")), Ok(None));
    // As Tally sends it (a leading space), and the shapes seen or admitted.
    assert_eq!(closing(Some(" 100 Box")), Ok(quantity_of("100", "Box")));
    assert_eq!(closing(Some("5 U.")), Ok(quantity_of("5", "U.")));
    assert_eq!(closing(Some("-2.5 Kgs")), Ok(quantity_of("-2.5", "Kgs")));
    assert_eq!(closing(Some("0 Box")), Ok(quantity_of("0", "Box")));
    // Everything else refuses the read. Two spaces are pinned as a refusal: the
    // grammar is one space, and a unit has none.
    for text in [
        "abc",
        "5",
        "5  Kgs",
        "5 Kgs extra",
        "1,000 Nos",
        "+5 Kgs",
        ".5 Kgs",
        "5. Kgs",
        "Kgs 5",
        "5 \u{a0}Kgs",
    ] {
        assert_eq!(closing(Some(text)), refused, "{text:?}");
    }
    // A unit is a name-bearing text and is held to the name bound.
    let unit = |length: usize| closing(Some(format!("5 {}", "u".repeat(length)).as_str()));
    assert!(unit(MASTERS_ASSUMED_NAME_CHARS).is_ok());
    assert_eq!(
        unit(MASTERS_ASSUMED_NAME_CHARS + 1),
        Err(NativeStockError::RowExceedsBound)
    );
}

#[test]
fn a_value_is_a_plain_signed_decimal_or_nothing() {
    let closing = |text: Option<&str>| {
        parse(&carton_closing_value(text)).map(|items| items.rows[0].closing.value.clone())
    };
    assert_eq!(closing(None), Ok(None));
    assert_eq!(closing(Some("")), Ok(None));
    assert_eq!(closing(Some("  ")), Ok(None));
    assert_eq!(closing(Some("2500.00")), Ok(Some(decimal("2500.00"))));
    assert_eq!(closing(Some(" -15500.00 ")), Ok(Some(decimal("-15500.00"))));
    assert_eq!(closing(Some("0.00")), Ok(Some(decimal("0.00"))));
    for text in ["1,000.00", "12.5 Dr", "abc", "+5", "1e3", "5.", ".5", "--5"] {
        assert_eq!(
            closing(Some(text)),
            Err(NativeStockError::ValueUnparseable),
            "{text:?}"
        );
    }
}

#[test]
fn a_repeated_guid_or_a_name_differing_only_in_case_refuses() {
    let text = items_response();
    // Caustic Soda Flakes takes Soda Ash Light's GUID, in the other case.
    assert_eq!(
        parse(&replaced(
            &text,
            &format!("{COMPANY}-0000010c"),
            &format!("{COMPANY}-0000010E")
        )),
        Err(NativeStockError::DuplicateGuid)
    );
    assert_eq!(
        parse(&replaced(
            &text,
            "NAME=\"Caustic Soda Flakes\"",
            "NAME=\"CARTON BOX SMALL\""
        )),
        Err(NativeStockError::DuplicateName)
    );
}

#[test]
fn a_guid_that_is_not_this_companys_or_has_no_valid_suffix_refuses() {
    let text = items_response();
    let carton = format!("{COMPANY}-00000110");
    for guid in [
        format!("{FOREIGN_COMPANY}-00000110"),
        format!("{COMPANY}-0000011"),
        format!("{COMPANY}-0000011g"),
        format!("{COMPANY}-000001100"),
        format!("{COMPANY}x00000110"),
        COMPANY.to_string(),
        String::new(),
    ] {
        assert_eq!(
            parse(&replaced(&text, &carton, &guid)),
            Err(NativeStockError::RowGuidForeign),
            "{guid:?}"
        );
    }
    // No GUID element at all binds the row to no company.
    assert_eq!(
        parse(&replaced(
            &text,
            &format!("<GUID TYPE=\"String\">{carton}</GUID>"),
            ""
        )),
        Err(NativeStockError::RowGuidForeign)
    );
}

fn collection_span(text: &str) -> (usize, usize, usize) {
    let start = text
        .find("<COLLECTION")
        .expect("the capture has a COLLECTION");
    let open_end = start + text[start..].find('>').expect("an opening tag") + 1;
    let end = text.find("</COLLECTION>").expect("a closing tag") + "</COLLECTION>".len();
    (start, open_end, end)
}

#[test]
fn an_absent_collection_is_not_an_empty_list_and_a_present_empty_one_is() {
    let text = items_response();
    let (start, open_end, end) = collection_span(&text);
    assert_eq!(
        parse(&format!("{}{}", &text[..start], &text[end..])),
        Err(NativeStockError::CollectionAbsent)
    );
    let close = text.find("</COLLECTION>").unwrap();
    assert_eq!(
        parse(&format!("{}{}", &text[..open_end], &text[close..])),
        Ok(NativeStockItems { rows: Vec::new() })
    );
    assert_eq!(
        parse(&replaced(
            &text,
            "</COLLECTION>",
            "</COLLECTION><COLLECTION></COLLECTION>"
        )),
        Err(NativeStockError::Malformed("stock_collection_repeated"))
    );
}

#[test]
fn a_tally_failure_a_missing_status_or_an_error_element_refuses() {
    let text = items_response();
    let malformed = |code: &'static str| Err(NativeStockError::Malformed(code));
    assert_eq!(
        parse(&replaced(&text, "<STATUS>1</STATUS>", "<STATUS>0</STATUS>")),
        Err(NativeStockError::TallyReportedFailure)
    );
    assert_eq!(
        parse(&replaced(&text, "<STATUS>1</STATUS>", "<STATUS></STATUS>")),
        malformed("stock_status_absent")
    );
    assert_eq!(
        parse(&replaced(&text, "<STATUS>1</STATUS>", "")),
        malformed("stock_status_absent")
    );
    assert_eq!(
        parse(&replaced(
            &text,
            "<STATUS>1</STATUS>",
            "<STATUS>1</STATUS><STATUS>1</STATUS>"
        )),
        malformed("stock_status_repeated")
    );
    for element in ["<LINEERROR>x</LINEERROR>", "<ERROR/>"] {
        for from in ["</STOCKITEM>", "<NAME>Carton Box Small</NAME>"] {
            let to = if from == "</STOCKITEM>" {
                format!("{element}</STOCKITEM>")
            } else {
                format!("{from}{element}")
            };
            assert_eq!(
                parse(&edited(&text, from, &to)),
                Err(NativeStockError::TallyReportedFailure),
                "{element} before {from}"
            );
        }
    }
    assert_eq!(
        parse(&replaced(&text, "<BODY>", "<BODY><LINEERROR>x</LINEERROR>")),
        Err(NativeStockError::TallyReportedFailure)
    );
    // The tag is matched exactly: `<ERRORS>` is not Tally's failure element.
    assert_eq!(
        parse(&edited(
            &text,
            "<PARENT TYPE=\"String\">Packaging</PARENT>",
            "<PARENT TYPE=\"String\"><ERRORS>x</ERRORS></PARENT>"
        )),
        malformed("stock_scalar_not_text_only")
    );
}

#[test]
fn a_truncated_response_a_doctype_a_second_root_or_stray_text_refuses() {
    let text = items_response();
    let malformed = |code: &'static str| Err(NativeStockError::Malformed(code));
    assert_eq!(
        parse(&text[..text.rfind("</ENVELOPE>").unwrap()]),
        malformed("stock_envelope_unterminated")
    );
    // Cut after a row's last complete child: a row left open.
    let mid_row = &text[..text.rfind("</CLOSINGVALUE>").unwrap() + "</CLOSINGVALUE>".len()];
    assert_eq!(parse(mid_row), malformed("stock_row_unterminated"));
    assert_eq!(
        parse(&format!("<!DOCTYPE ENVELOPE>{text}")),
        malformed("stock_doctype_forbidden")
    );
    assert_eq!(
        parse(&format!("{text}<ENVELOPE></ENVELOPE>")),
        malformed("stock_root_not_envelope")
    );
    let row = "<STOCKITEM NAME=\"Carton Box Small\"";
    let parent = "<PARENT TYPE=\"String\">Packaging</PARENT>";
    for stray in ["stray", "&amp;", "<![CDATA[x]]>"] {
        for from in [row, parent] {
            assert_eq!(
                parse(&edited(&text, from, &format!("{stray}{from}"))),
                malformed("stock_unexpected_text"),
                "{stray} before {from}"
            );
        }
    }
    // The shared reading helpers answer under this module's codes.
    assert_eq!(
        parse(&edited(
            &text,
            "NAME=\"Carton Box Small\" RESERVEDNAME=\"\"",
            "NAME=Carton RESERVEDNAME=\"\""
        )),
        malformed("stock_attribute_malformed")
    );
    assert_eq!(
        parse(&edited(
            &text,
            parent,
            "<PARENT TYPE=\"String\">&bogus;</PARENT>"
        )),
        malformed("stock_xml_invalid_escape")
    );
    assert_eq!(
        parse(&edited(
            &text,
            parent,
            "<PARENT TYPE=\"String\">a<b/>c</PARENT>"
        )),
        malformed("stock_scalar_not_text_only")
    );
}

#[test]
fn a_child_that_is_not_a_stock_item_a_nameless_row_or_a_repeated_field_refuses() {
    let text = items_response();
    assert_eq!(
        parse(&edited(
            &text,
            "<STOCKITEM NAME=\"Carton Box Small\"",
            "<LEDGER NAME=\"Carton Box Small\""
        )),
        Err(NativeStockError::ForeignChild)
    );
    assert_eq!(
        parse(&edited(
            &text,
            "<STOCKITEM NAME=\"Carton Box Small\"",
            "<STOCKITEM/><STOCKITEM NAME=\"Carton Box Small\""
        )),
        Err(NativeStockError::Malformed("stock_row_empty"))
    );
    for name in ["", "   "] {
        assert_eq!(
            parse(&edited(
                &text,
                "NAME=\"Carton Box Small\"",
                &format!("NAME=\"{name}\"")
            )),
            Err(NativeStockError::RowWithoutName),
            "{name:?}"
        );
    }
    assert_eq!(
        parse(&edited(
            &text,
            "<BASEUNITS TYPE=\"String\">Box</BASEUNITS>",
            "<BASEUNITS TYPE=\"String\">Box</BASEUNITS><BASEUNITS TYPE=\"String\">Box</BASEUNITS>"
        )),
        Err(NativeStockError::Malformed("stock_row_field_repeated"))
    );
}

#[test]
fn a_blank_parent_or_base_unit_is_none_and_the_root_marker_is_kept_as_read() {
    let text = items_response();
    let blank = parse(&edited(
        &text,
        "<PARENT TYPE=\"String\">Packaging</PARENT>",
        "<PARENT TYPE=\"String\">  </PARENT>",
    ))
    .unwrap();
    assert_eq!(blank.rows[0].parent, None);
    let absent = parse(&edited(
        &text,
        "<BASEUNITS TYPE=\"String\">Box</BASEUNITS>",
        "",
    ))
    .unwrap();
    assert_eq!(absent.rows[0].base_unit, None);
    // Tally's reserved root, as the group snapshot keeps it.
    let root = parse(&edited(
        &text,
        "<PARENT TYPE=\"String\">Packaging</PARENT>",
        "<PARENT TYPE=\"String\">&#4; Primary</PARENT>",
    ))
    .unwrap();
    assert_eq!(root.rows[0].parent.as_deref(), Some("\u{fffd}#4; Primary"));
}

#[test]
fn a_name_or_alias_count_over_the_assumed_bounds_refuses() {
    let text = items_response();
    let exceeds = Err(NativeStockError::RowExceedsBound);
    let attribute = |length: usize| {
        edited(
            &text,
            "NAME=\"Carton Box Small\"",
            &format!("NAME=\"{}\"", "x".repeat(length)),
        )
    };
    assert!(parse(&attribute(MASTERS_ASSUMED_NAME_CHARS)).is_ok());
    assert_eq!(parse(&attribute(MASTERS_ASSUMED_NAME_CHARS + 1)), exceeds);
    let alias = |text_of: &str| {
        edited(
            &text,
            "<NAME>Carton Box Small</NAME>",
            &format!("<NAME>{text_of}</NAME>"),
        )
    };
    assert!(parse(&alias(&"y".repeat(MASTERS_ASSUMED_NAME_CHARS))).is_ok());
    assert_eq!(
        parse(&alias(&"y".repeat(MASTERS_ASSUMED_NAME_CHARS + 1))),
        exceeds
    );
    // Characters, not bytes: a multi-byte name at the bound is admitted.
    assert!(parse(&alias(&"\u{e9}".repeat(MASTERS_ASSUMED_NAME_CHARS))).is_ok());
    let names = |count: usize| {
        edited(
            &text,
            "<NAME>Carton Box Small</NAME>",
            &"<NAME>a</NAME>".repeat(count),
        )
    };
    assert!(parse(&names(1 + MASTERS_ASSUMED_ALIASES)).is_ok());
    assert_eq!(parse(&names(2 + MASTERS_ASSUMED_ALIASES)), exceeds);
    // Empty alias names count too.
    let empties = |count: usize| {
        edited(
            &text,
            "<NAME>Carton Box Small</NAME>",
            &"<NAME/>".repeat(count),
        )
    };
    assert!(parse(&empties(1 + MASTERS_ASSUMED_ALIASES)).is_ok());
    assert_eq!(parse(&empties(2 + MASTERS_ASSUMED_ALIASES)), exceeds);
    // The count is per row, across every language list the row carries: two
    // lists of three names are one past the bound, though neither is.
    let two_lists = |per_list: usize| {
        let names = "<NAME>a</NAME>".repeat(per_list);
        replaced(
            &edited(
                &text,
                "<LANGUAGENAME.LIST>",
                &format!(
                    "<LANGUAGENAME.LIST><NAME.LIST>{names}</NAME.LIST></LANGUAGENAME.LIST><LANGUAGENAME.LIST>"
                ),
            ),
            "<NAME>Carton Box Small</NAME>",
            &names,
        )
    };
    assert!(parse(&two_lists(2)).is_ok());
    assert_eq!(parse(&two_lists(3)), exceeds);
}

#[test]
fn a_parent_base_unit_reserved_name_or_name_element_over_the_bound_refuses() {
    let text = items_response();
    let cases: [(&str, &str, &str); 4] = [
        (
            "<PARENT TYPE=\"String\">Packaging</PARENT>",
            "<PARENT TYPE=\"String\">{}</PARENT>",
            "PARENT",
        ),
        (
            "<BASEUNITS TYPE=\"String\">Box</BASEUNITS>",
            "<BASEUNITS TYPE=\"String\">{}</BASEUNITS>",
            "BASEUNITS",
        ),
        ("RESERVEDNAME=\"\"", "RESERVEDNAME=\"{}\"", "RESERVEDNAME"),
        (
            "<CLOSINGVALUE TYPE=\"Amount\">2500.00</CLOSINGVALUE>",
            "<CLOSINGVALUE TYPE=\"Amount\">2500.00</CLOSINGVALUE><NAME TYPE=\"String\">{}</NAME>",
            "NAME element",
        ),
    ];
    for (from, template, label) in cases {
        let with =
            |length: usize| edited(&text, from, &template.replace("{}", &"n".repeat(length)));
        assert!(
            parse(&with(MASTERS_ASSUMED_NAME_CHARS)).is_ok(),
            "{label} at the bound"
        );
        assert_eq!(
            parse(&with(MASTERS_ASSUMED_NAME_CHARS + 1)),
            Err(NativeStockError::RowExceedsBound),
            "{label} past the bound"
        );
    }
}

fn first_row_units(text: &str) -> usize {
    let data = text.find("<DATA>").unwrap();
    let start = data + text[data..].find("<STOCKITEM NAME=").unwrap();
    let end = start + text[start..].find("</STOCKITEM>").unwrap() + "</STOCKITEM>".len();
    text[start..end].encode_utf16().count()
}

/// The row as a whole is bounded, whether or not the parser reads what makes
/// it long: a row of exactly the assumed worst length is admitted, one more
/// character is not.
#[test]
fn a_row_longer_than_the_assumed_worst_row_refuses() {
    let text = items_response();
    let worst = stock_item_worst_row_bytes() / 2;
    let filler = "<FILLER></FILLER>".len();
    let padded = |chars: usize| {
        edited(
            &text,
            "</STOCKITEM>",
            &format!(
                "<FILLER>{}</FILLER></STOCKITEM>",
                "f".repeat(chars - filler)
            ),
        )
    };
    let admitted = worst - first_row_units(&text);
    assert!(admitted > filler);
    assert!(parse(&padded(admitted)).is_ok(), "at the bound");
    assert_eq!(
        parse(&padded(admitted + 1)),
        Err(NativeStockError::RowExceedsBound),
        "past the bound"
    );
}

/// The row is measured in UTF-16 units, the unit the response is read in: an
/// astral character is one `char` and two units.
#[test]
fn a_row_is_measured_in_utf16_units_not_characters() {
    let text = items_response();
    let worst = stock_item_worst_row_bytes() / 2;
    let filler = "<FILLER></FILLER>".len();
    let room = worst - first_row_units(&text) - filler;
    let (emojis, pad) = (room / 2, room % 2);
    let padded = |emojis: usize| {
        edited(
            &text,
            "</STOCKITEM>",
            &format!(
                "<FILLER>{}{}</FILLER></STOCKITEM>",
                "\u{1F600}".repeat(emojis),
                "x".repeat(pad)
            ),
        )
    };
    assert!(parse(&padded(emojis)).is_ok(), "at the bound");
    // One more is two units over, though the row is still under the bound in
    // characters.
    assert!(first_row_units(&text) + filler + emojis + 1 + pad <= worst);
    assert_eq!(
        parse(&padded(emojis + 1)),
        Err(NativeStockError::RowExceedsBound)
    );
}

#[test]
fn every_fixture_row_fits_the_worst_row_bytes() {
    let text = items_response();
    let mut rows = 0;
    let mut rest = &text[text.find("<DATA>").unwrap()..];
    while let Some(start) = rest.find("<STOCKITEM NAME=") {
        let end = start + rest[start..].find("</STOCKITEM>").unwrap() + "</STOCKITEM>".len();
        let bytes = rest[start..end].encode_utf16().count() * 2;
        assert!(bytes <= stock_item_worst_row_bytes(), "row {rows}: {bytes}");
        rows += 1;
        rest = &rest[end..];
    }
    assert_eq!(rows, 11, "rows measured");
}

#[test]
fn the_worst_row_bytes_follow_the_documented_arithmetic() {
    // 2 * (700 + 6 * 128 * (7 + 4)), worked by hand: 6 * 128 = 768, times 11 is
    // 8,448, plus 700 is 9,148, doubled is 18,296.
    assert_eq!(stock_item_worst_row_bytes(), 18_296);
    assert_eq!((STOCK_ITEM_FIXED_CHARS, STOCK_ITEM_NAME_SLOTS), (700, 7));
    assert_eq!(
        (MASTERS_ASSUMED_NAME_CHARS, MASTERS_ASSUMED_ALIASES),
        (128, 4)
    );
    // The largest master mark a read is admitted at: 874 rows of 18,296 bytes
    // are 15,990,704, and 875 are 16,009,000.
    assert_eq!(
        MASTERS_RESPONSE_BUDGET_BYTES / stock_item_worst_row_bytes(),
        874
    );
    const { assert!(874 * 18_296 <= MASTERS_RESPONSE_BUDGET_BYTES) };
    const { assert!(875 * 18_296 > MASTERS_RESPONSE_BUDGET_BYTES) };
}

#[test]
fn the_report_capture_totals_its_amounts_and_equals_the_items_closing_value_sum() {
    assert_eq!(
        report_of(&report_response()),
        Ok(NativeStockReport::Lines {
            total: Some(decimal("3000.01")),
            present: 3,
            empty: 0,
        })
    );
    // The capture's tie: the report total is the items' closing-value sum.
    let items = parse(&items_response()).unwrap();
    assert_eq!(
        present_closing_value_sum(&items.rows),
        Ok(decimal("3000.01"))
    );
    let NativeStockGate::Matched {
        items,
        total,
        report_empty_amounts,
        ..
    } = gate(&items_response(), &report_response())
    else {
        panic!("the capture's report ties to its items");
    };
    assert_eq!(items.len(), 11);
    assert_eq!(total, decimal("3000.01"));
    assert_eq!(report_empty_amounts, 0);
}

#[test]
fn a_report_that_differs_by_a_hundredth_withholds_the_items_and_names_both_totals() {
    let report = replaced(
        &report_response(),
        "<DSPCLAMTA>18750.00</DSPCLAMTA>",
        "<DSPCLAMTA>18750.01</DSPCLAMTA>",
    );
    assert_eq!(
        gate(&items_response(), &report),
        NativeStockGate::Differs {
            items_total: decimal("3000.01"),
            report_total: decimal("3000.02"),
        }
    );
}

#[test]
fn the_totals_are_compared_by_value_not_by_spelling() {
    // 3000.010 is 3000.01: the sums are equal though their text differs.
    let report = NativeStockReport::Lines {
        total: Some(decimal("3000.010")),
        present: 1,
        empty: 0,
    };
    let items = parse(&items_response()).unwrap().rows;
    assert!(matches!(
        gate_stock_summary(items, &report),
        Ok(NativeStockGate::Matched { .. })
    ));
}

#[test]
fn an_empty_or_unknown_report_is_not_a_comparison_and_returns_the_items_unchecked() {
    let text = report_response();
    let start = text.find("<ENVELOPE>").unwrap() + "<ENVELOPE>".len();
    let hollow = format!(
        "{}{}",
        &text[..start],
        &text[text.rfind("</ENVELOPE>").unwrap()..]
    );
    let cases = [
        (
            hollow.clone(),
            NativeStockReport::Empty,
            "stock_report_empty",
        ),
        (
            hollow.replace("<ENVELOPE></ENVELOPE>", "<ENVELOPE/>"),
            NativeStockReport::Empty,
            "stock_report_empty",
        ),
        (
            text.replacen("<ENVELOPE>", "<RESPONSE>", 1)
                .replacen("</ENVELOPE>", "</RESPONSE>", 1),
            NativeStockReport::UnknownReport,
            "stock_unknown_report",
        ),
    ];
    for (xml, expected, reason) in cases {
        assert_eq!(report_of(&xml), Ok(expected.clone()), "{xml}");
        let NativeStockGate::NotChecked {
            items,
            reason: given,
            ..
        } = gate(&items_response(), &xml)
        else {
            panic!("{xml}: not a comparison");
        };
        assert_eq!((items.len(), given), (11, reason));
    }
    // Every amount empty: nothing to compare, and not a total of zero.
    let all_empty =
        ["18750.00", "14500.00", "-30249.99"]
            .iter()
            .fold(text.clone(), |xml, amount| {
                replaced(
                    &xml,
                    &format!("<DSPCLAMTA>{amount}</DSPCLAMTA>"),
                    "<DSPCLAMTA></DSPCLAMTA>",
                )
            });
    assert_eq!(
        report_of(&all_empty),
        Ok(NativeStockReport::Lines {
            total: None,
            present: 0,
            empty: 3
        })
    );
    assert!(matches!(
        gate(&items_response(), &all_empty),
        NativeStockGate::NotChecked {
            reason: "stock_report_amounts_all_empty",
            ..
        }
    ));
}

#[test]
fn an_empty_amount_is_counted_and_left_out_never_read_as_zero() {
    let report = replaced(
        &report_response(),
        "<DSPCLAMTA>14500.00</DSPCLAMTA>",
        "<DSPCLAMTA></DSPCLAMTA>",
    );
    assert_eq!(
        report_of(&report),
        Ok(NativeStockReport::Lines {
            total: Some(decimal("-11499.99")),
            present: 2,
            empty: 1,
        })
    );
    // The two Packaging items' closing values are empty too: the sums agree,
    // and the empty amount is reported beside the match.
    let items = replaced(
        &replaced(
            &items_response(),
            "<CLOSINGVALUE TYPE=\"Amount\">2500.00</CLOSINGVALUE>",
            "<CLOSINGVALUE TYPE=\"Amount\"></CLOSINGVALUE>",
        ),
        "<CLOSINGVALUE TYPE=\"Amount\">12000.00</CLOSINGVALUE>",
        "<CLOSINGVALUE TYPE=\"Amount\"></CLOSINGVALUE>",
    );
    let NativeStockGate::Matched {
        total,
        report_empty_amounts,
        totals,
        ..
    } = gate(&items, &report)
    else {
        panic!("the present amounts agree");
    };
    assert_eq!(total, decimal("-11499.99"));
    assert_eq!(report_empty_amounts, 1);
    assert!(totals.partial, "stocked items with an empty value");
}

#[test]
fn a_report_outside_the_closed_shape_refuses() {
    let text = report_response();
    let shape = |code: &'static str| Err(NativeStockError::Malformed(code));
    let cases: Vec<(&str, String, Result<NativeStockReport, NativeStockError>)> = vec![
        (
            "a failure element",
            replaced(&text, "<DSPACCNAME>", "<LINEERROR>x</LINEERROR><DSPACCNAME>"),
            Err(NativeStockError::TallyReportedFailure),
        ),
        (
            "an error element",
            replaced(&text, "<DSPACCNAME>", "<ERROR/><DSPACCNAME>"),
            Err(NativeStockError::TallyReportedFailure),
        ),
        (
            "an unexpected child",
            replaced(&text, "<DSPACCNAME>", "<DSPFOO></DSPFOO><DSPACCNAME>"),
            shape("stock_report_unexpected_element"),
        ),
        (
            "a STATUS, which the report does not carry",
            replaced(&text, "<DSPACCNAME>", "<STATUS>1</STATUS><DSPACCNAME>"),
            shape("stock_report_unexpected_element"),
        ),
        (
            "a name with no amounts after it, before another name",
            replaced(
                &text,
                "<DSPSTKINFO>",
                "<DSPACCNAME><DSPDISPNAME>x</DSPDISPNAME></DSPACCNAME><DSPSTKINFO>",
            ),
            shape("stock_report_unexpected_element"),
        ),
        (
            "amounts with no name before them",
            replaced(&text, "<DSPACCNAME>", "<DSPSTKINFO><DSPSTKCL><DSPCLAMTA>1</DSPCLAMTA></DSPSTKCL></DSPSTKINFO><DSPACCNAME>"),
            shape("stock_report_unexpected_element"),
        ),
        (
            "a last name with no amounts",
            replaced(
                &text,
                "</ENVELOPE>",
                "<DSPACCNAME><DSPDISPNAME>x</DSPDISPNAME></DSPACCNAME></ENVELOPE>",
            ),
            shape("stock_report_name_without_info"),
        ),
        (
            "a blank name",
            replaced(
                &text,
                "<DSPDISPNAME>Finished Kits</DSPDISPNAME>",
                "<DSPDISPNAME>  </DSPDISPNAME>",
            ),
            shape("stock_report_name_empty"),
        ),
        (
            "a name with no display name",
            replaced(
                &text,
                "<DSPDISPNAME>Finished Kits</DSPDISPNAME>",
                "",
            ),
            shape("stock_report_name_missing"),
        ),
        (
            "an amount that is not a plain decimal",
            replaced(
                &text,
                "<DSPCLAMTA>18750.00</DSPCLAMTA>",
                "<DSPCLAMTA>18,750.00</DSPCLAMTA>",
            ),
            Err(NativeStockError::ReportAmountInvalid),
        ),
        (
            "no amount element",
            replaced(&text, "<DSPCLAMTA>18750.00</DSPCLAMTA>", ""),
            shape("stock_report_amount_missing"),
        ),
        (
            "a repeated amount element",
            replaced(
                &text,
                "<DSPCLAMTA>18750.00</DSPCLAMTA>",
                "<DSPCLAMTA>18750.00</DSPCLAMTA><DSPCLAMTA>1</DSPCLAMTA>",
            ),
            shape("stock_report_closing_shape"),
        ),
        (
            "an element the closing block does not have",
            replaced(
                &text,
                "<DSPCLAMTA>18750.00</DSPCLAMTA>",
                "<DSPCLAMTA>18750.00</DSPCLAMTA><DSPOPAMTA>1</DSPOPAMTA>",
            ),
            shape("stock_report_closing_shape"),
        ),
        (
            "a second closing block",
            replaced(
                &text,
                "</DSPSTKCL>",
                "</DSPSTKCL><DSPSTKCL><DSPCLAMTA>1</DSPCLAMTA></DSPSTKCL>",
            ),
            shape("stock_report_info_shape"),
        ),
        (
            "text between elements",
            replaced(&text, "<DSPACCNAME>", "stray<DSPACCNAME>"),
            shape("stock_unexpected_text"),
        ),
        (
            "an element after the envelope",
            format!("{text}<DSPACCNAME></DSPACCNAME>"),
            shape("stock_report_trailing_content"),
        ),
        (
            "an empty element after the envelope",
            format!("{text}<DSPFOO/>"),
            shape("stock_report_unexpected_empty_element"),
        ),
        (
            "a truncated report",
            text[..text.rfind("</ENVELOPE>").unwrap()].to_string(),
            shape("stock_report_envelope_unterminated"),
        ),
        (
            "a root that is not an envelope",
            text.replacen("<ENVELOPE>", "<REPORT>", 1)
                .replacen("</ENVELOPE>", "</REPORT>", 1),
            shape("stock_report_root_not_envelope"),
        ),
    ];
    for (label, xml, expected) in cases {
        assert_eq!(report_of(&xml), expected, "{label}");
    }
}

#[test]
fn a_report_amount_sum_that_cannot_be_formed_refuses() {
    // 256 bytes is the widest decimal: the sum of two such amounts is wider.
    let wide = "9".repeat(256);
    let report = replaced(
        &replaced(
            &report_response(),
            "<DSPCLAMTA>18750.00</DSPCLAMTA>",
            &format!("<DSPCLAMTA>{wide}</DSPCLAMTA>"),
        ),
        "<DSPCLAMTA>14500.00</DSPCLAMTA>",
        &format!("<DSPCLAMTA>{wide}</DSPCLAMTA>"),
    );
    assert_eq!(report_of(&report), Err(NativeStockError::SumInvalid));
}

fn totals_of(items: &str) -> NativeStockTotals {
    NativeStockTotals::of(&parse(items).unwrap().rows).unwrap()
}

/// The totals of the captured items after `edit` has been applied to each
/// parsed row: the capture's text is untouched, only the parsed quantities and
/// values change.
fn totals_after(edit: impl Fn(&mut NativeStockItem)) -> NativeStockTotals {
    let mut rows = parse(&items_response()).unwrap().rows;
    rows.iter_mut().for_each(edit);
    NativeStockTotals::of(&rows).unwrap()
}

#[test]
fn the_totals_count_each_kind_of_item_and_withhold_a_sum_when_any_closing_value_is_empty() {
    let totals = totals_of(&items_response());
    // Four closing values are empty: Zero Stock Item holds -50 Kgs with none,
    // and three items have neither a quantity nor a value.
    assert_eq!(
        totals,
        NativeStockTotals {
            item_count: 11,
            negative_closing_quantity_count: 1,
            zero_quantity_count: 0,
            empty_closing_quantity_count: 3,
            empty_closing_value_count: 4,
            value_sum: None,
            partial: true,
        }
    );
    // An empty quantity does not change that: blanking Zero Stock Item's
    // quantity too leaves the sum withheld.
    let unstocked = replaced(
        &items_response(),
        "<CLOSINGBALANCE TYPE=\"Quantity\">-50.000 Kgs</CLOSINGBALANCE>",
        "<CLOSINGBALANCE TYPE=\"Quantity\"></CLOSINGBALANCE>",
    );
    assert_eq!(
        totals_of(&unstocked),
        NativeStockTotals {
            item_count: 11,
            negative_closing_quantity_count: 0,
            zero_quantity_count: 0,
            empty_closing_quantity_count: 4,
            empty_closing_value_count: 4,
            value_sum: None,
            partial: true,
        }
    );
    // Nor does a closing quantity that is present and zero: that it makes an
    // empty value zero is unmeasured, so the sum is withheld.
    let zero_quantity = totals_after(|item| {
        if item.closing.value.is_none() {
            item.closing.quantity = quantity_of("0.000", "Nos");
        }
    });
    assert_eq!(
        (
            zero_quantity.partial,
            zero_quantity.value_sum,
            zero_quantity.zero_quantity_count,
            zero_quantity.empty_closing_value_count,
        ),
        (true, None, 4, 4)
    );
}

/// The sum of `values` with every sign dropped, built in the test from the
/// same rows: the number a magnitude comparison would use.
fn sum_of_magnitudes<'a>(values: impl Iterator<Item = &'a ExactDecimal>) -> ExactDecimal {
    values
        .map(|value| value.abs().unwrap())
        .try_fold(ExactDecimal::zero(), |sum, value| sum.checked_add(&value))
        .unwrap()
}

#[test]
fn a_mixed_sign_set_is_summed_algebraically_on_both_sides() {
    let rows = parse(&items_response()).unwrap().rows;
    let report = report_of(&report_response()).unwrap();
    let NativeStockReport::Lines {
        total: Some(report_total),
        ..
    } = &report
    else {
        panic!("the capture's report has amounts");
    };
    let item_values = rows
        .iter()
        .filter_map(|row| row.closing.value.as_ref())
        .collect::<Vec<_>>();
    // Both signs are present among the items' values, and (below) among the
    // report's amounts.
    assert!(item_values.iter().any(|value| value.is_negative()));
    assert!(item_values.iter().any(|value| !value.is_negative()));
    // Each side is its algebraic sum, `3000.01`, and the gate matches them.
    let items_sum = present_closing_value_sum(&rows).unwrap();
    assert!(items_sum.numeric_eq(&decimal("3000.01")));
    assert!(report_total.numeric_eq(&decimal("3000.01")));
    assert!(matches!(
        gate(&items_response(), &report_response()),
        NativeStockGate::Matched { .. }
    ));
    // A sum of magnitudes is a different number on each side, so a comparison
    // of magnitudes would not have matched.
    let items_magnitudes = sum_of_magnitudes(item_values.iter().copied());
    assert!(!items_magnitudes.numeric_eq(&decimal("3000.01")));
    // The report's own amounts, read from the capture's text.
    let report_text = report_response();
    let report_amounts = report_text
        .split("<DSPCLAMTA>")
        .skip(1)
        .map(|part| decimal(part.split("</DSPCLAMTA>").next().unwrap().trim()))
        .collect::<Vec<_>>();
    assert_eq!(report_amounts.len(), 3);
    assert!(report_amounts.iter().any(|amount| amount.is_negative()));
    assert!(report_amounts.iter().any(|amount| !amount.is_negative()));
    let report_magnitudes = sum_of_magnitudes(report_amounts.iter());
    assert!(!report_magnitudes.numeric_eq(&decimal("3000.01")));
    assert!(!items_magnitudes.numeric_eq(&report_magnitudes));
    // An edit of captured text: Caustic Soda Flakes' closing value `-1000.00`
    // has its sign flipped. The items' algebraic sum moves by 2000.00 and no
    // longer equals the report's, so the gate differs.
    let flipped = replaced(
        &items_response(),
        "<CLOSINGVALUE TYPE=\"Amount\">-1000.00</CLOSINGVALUE>",
        "<CLOSINGVALUE TYPE=\"Amount\">1000.00</CLOSINGVALUE>",
    );
    let NativeStockGate::Differs {
        items_total,
        report_total,
    } = gate(&flipped, &report_response())
    else {
        panic!("a flipped sign must not match");
    };
    assert!(items_total.numeric_eq(&decimal("5000.01")));
    assert!(report_total.numeric_eq(&decimal("3000.01")));
}

#[test]
fn a_sum_is_formed_only_when_every_closing_value_is_present() {
    // An edit of the captured text, not a capture: each of the four empty
    // closing values is given an explicit `0.00`, so none is empty.
    let explicit = items_response().replace(
        "<CLOSINGVALUE TYPE=\"Amount\"></CLOSINGVALUE>",
        "<CLOSINGVALUE TYPE=\"Amount\">0.00</CLOSINGVALUE>",
    );
    assert_eq!(
        explicit
            .matches("<CLOSINGVALUE TYPE=\"Amount\">0.00<")
            .count(),
        4
    );
    let totals = totals_of(&explicit);
    assert!(!totals.partial);
    assert_eq!(totals.empty_closing_value_count, 0);
    // Quantities are as captured: three are empty, and one is negative.
    assert_eq!(totals.empty_closing_quantity_count, 3);
    assert_eq!(totals.negative_closing_quantity_count, 1);
    assert!(totals
        .value_sum
        .expect("every value present")
        .numeric_eq(&decimal("3000.01")));
    // One value blanked again (Carton Box Small's): withheld.
    let blanked = replaced(
        &explicit,
        "<CLOSINGVALUE TYPE=\"Amount\">2500.00</CLOSINGVALUE>",
        "<CLOSINGVALUE TYPE=\"Amount\"></CLOSINGVALUE>",
    );
    let totals = totals_of(&blanked);
    assert_eq!(
        (
            totals.partial,
            totals.value_sum,
            totals.empty_closing_value_count
        ),
        (true, None, 1)
    );
}

#[test]
fn a_book_whose_closing_values_are_all_empty_has_no_value_sum() {
    // Every item with no quantity and no value: the sum is not zero, it is
    // withheld.
    let totals = totals_after(|item| {
        item.closing.quantity = None;
        item.closing.value = None;
    });
    assert_eq!(
        totals,
        NativeStockTotals {
            item_count: 11,
            negative_closing_quantity_count: 0,
            zero_quantity_count: 0,
            empty_closing_quantity_count: 11,
            empty_closing_value_count: 11,
            value_sum: None,
            partial: true,
        }
    );
    let json = serde_json::to_value(&totals).unwrap();
    assert_eq!(json["value_sum"], serde_json::Value::Null);
    assert_eq!(json["partial"], true);
}

#[test]
fn a_book_with_no_items_has_a_sum_of_zero_and_is_not_partial() {
    // A present, empty collection is zero rows (see the zero-row parse test):
    // nothing is missing, so the sum is zero, not withheld.
    let totals = NativeStockTotals::of(&[]).unwrap();
    assert_eq!(totals.item_count, 0);
    assert!(!totals.partial);
    assert_eq!(totals.empty_closing_value_count, 0);
    assert!(totals
        .value_sum
        .expect("no value is missing")
        .numeric_eq(&decimal("0")));
}

#[test]
fn an_opening_position_that_is_not_returned_is_still_validated() {
    // Edits of captured text: the opening is never serialized, but a malformed
    // one still refuses the whole read.
    let quantity = replaced(
        &items_response(),
        "<OPENINGBALANCE TYPE=\"Quantity\"> 100 Box</OPENINGBALANCE>",
        "<OPENINGBALANCE TYPE=\"Quantity\">100</OPENINGBALANCE>",
    );
    assert_eq!(parse(&quantity), Err(NativeStockError::QuantityUnparseable));
    let value = replaced(
        &items_response(),
        "<OPENINGVALUE TYPE=\"Amount\">2500.00</OPENINGVALUE>",
        "<OPENINGVALUE TYPE=\"Amount\">not a value</OPENINGVALUE>",
    );
    assert_eq!(parse(&value), Err(NativeStockError::ValueUnparseable));
}

#[test]
fn the_items_and_totals_serialize_in_the_shape_the_tool_returns() {
    let items = parse(&items_response()).unwrap();
    let json = serde_json::to_value(item(&items, "Caustic Soda Flakes")).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "name": "Caustic Soda Flakes",
            "guid": format!("{COMPANY}-0000010c"),
            "parent": "Raw Chemicals",
            "base_unit": "Kgs",
            "closing": {"quantity": {"amount": "400.000", "unit": "Kgs"}, "value": "-1000.00"},
        })
    );
    // The opening position is read (the parse tests above assert it) but is
    // never serialized, on any item.
    assert!(item(&items, "Caustic Soda Flakes").opening.value.is_some());
    for row in &items.rows {
        assert!(
            serde_json::to_value(row).unwrap().get("opening").is_none(),
            "{}",
            row.name
        );
    }
    let empty = serde_json::to_value(item(&items, "Cleaning Kit B")).unwrap();
    assert_eq!(
        empty["closing"],
        serde_json::json!({"quantity": null, "value": null})
    );
    let totals = serde_json::to_value(totals_of(&items_response())).unwrap();
    assert_eq!(totals["value_sum"], serde_json::Value::Null);
    assert_eq!(totals["partial"], true);
    assert_eq!(
        serde_json::to_value(NativeInventoryFlags {
            integrated: NativeFlag::Yes,
            inventory_on: NativeFlag::No,
            batchwise: NativeFlag::Unknown,
        })
        .unwrap(),
        serde_json::json!({"integrated": "yes", "inventory_on": "no", "batchwise": "unknown"})
    );
}

#[test]
fn only_a_31_march_is_an_as_of_that_can_be_constructed() {
    // The financial-year end is the one date measured for stock, in any year.
    for date in ["20260331", "20250331"] {
        let date = TallyDate::parse(date).unwrap();
        assert_eq!(StockSummaryAsOf::new(date.clone()).unwrap().date(), &date);
    }
    // Every other date is the typed refusal: another month's 31st and the
    // first two days the boundary rule of other reads admits included; and a
    // 31 March look-alike in another month or on another day.
    for date in [
        "20260731", "20260401", "20260402", "20260101", "20260301", "20260330", "20261231",
        "20260930",
    ] {
        assert_eq!(
            StockSummaryAsOf::new(TallyDate::parse(date).unwrap()),
            Err(NativeStockError::AsOfNotMeasured),
            "{date}"
        );
    }
    assert_eq!(
        NativeStockError::AsOfNotMeasured.code(),
        "stock_summary_as_of_not_measured"
    );
}

#[test]
fn every_code_carries_the_stock_prefix_but_the_company_flags_refusals() {
    for error in [
        NativeStockError::TallyReportedFailure,
        NativeStockError::Malformed("stock_x"),
        NativeStockError::CollectionAbsent,
        NativeStockError::ForeignChild,
        NativeStockError::RowWithoutName,
        NativeStockError::RowGuidForeign,
        NativeStockError::DuplicateGuid,
        NativeStockError::DuplicateName,
        NativeStockError::RowExceedsBound,
        NativeStockError::FlagInvalid("is_integrated"),
        NativeStockError::FlagInvalid("other"),
        NativeStockError::QuantityUnparseable,
        NativeStockError::ValueUnparseable,
        NativeStockError::ReportAmountInvalid,
        NativeStockError::SumInvalid,
        NativeStockError::AsOfNotMeasured,
    ] {
        assert!(error.code().starts_with("stock_"), "{error:?}");
    }
    for error in [
        NativeStockError::CompanyFlagsNotOneRow,
        NativeStockError::CompanyFlagsGuidUnsupported,
    ] {
        assert!(error.code().starts_with("company_flags_"), "{error:?}");
    }
    // Every code the shared helpers raise is translated, not defaulted.
    for code in [
        "masters_xml_malformed",
        "masters_xml_invalid_encoding",
        "masters_xml_invalid_escape",
        "masters_scalar_not_text_only",
        "masters_unexpected_text",
        "masters_attribute_malformed",
        "masters_row_unterminated",
    ] {
        assert_eq!(
            NativeStockError::from(NativeMastersError::Malformed(code)).code(),
            code.replacen("masters_", "stock_", 1)
        );
    }
    assert_eq!(
        NativeStockError::from(NativeMastersError::TallyReportedFailure),
        NativeStockError::TallyReportedFailure
    );
    assert_eq!(
        NativeStockError::from(NativeMastersError::RowExceedsBound),
        NativeStockError::RowExceedsBound
    );
    assert_eq!(
        NativeStockError::from(NativeMastersError::RowWithoutName),
        NativeStockError::RowWithoutName
    );
}
