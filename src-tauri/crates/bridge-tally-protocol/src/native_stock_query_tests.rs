use super::*;
use crate::native_stock_summary::{parse_native_stock_items, NativeQuantityRead};
use crate::outstandings_shared::DateBoundaryProfile;
use crate::xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName, ValidatedDateRange};
use bridge_tally_primitives::TallyDate;

const LAB: &str = "BRIDGE SHAPE LAB";
const LAB_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";

/// The committed captures of #2018 on BRIDGE SHAPE LAB (`tests/fixtures/stock-query/PROVENANCE.md`).
fn capture(name: &str) -> Vec<u8> {
    std::fs::read(format!(
        "{}/tests/fixtures/stock-query/{name}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("a committed capture")
}

fn answer(name: &str) -> String {
    let bytes = capture(&format!("{name}.answer.utf16le.xml"));
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();
    String::from_utf16(&units).expect("BOM-less UTF-16LE")
}

fn period(from: &str, to: &str) -> NativeLedgerSnapshotPeriod {
    NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse(from).unwrap(),
        TallyDate::parse(to).unwrap(),
    )
    .unwrap()
}

/// Every captured request naming an item is the builder's rendering, byte for byte, as the
/// product would send it.
#[test]
fn every_captured_request_is_the_builders_bytes() {
    for (file, item, from, to) in [
        (
            "r-4-stock-query-stockitemname",
            "Hydrochloric Acid 33pc",
            "20250401",
            "20260331",
        ),
        (
            "r-01-stock-query-carton-box-small",
            "Carton Box Small",
            "20250401",
            "20260331",
        ),
        (
            "r-02-stock-query-caustic-soda-flakes",
            "Caustic Soda Flakes",
            "20250401",
            "20260331",
        ),
        (
            "r-03-stock-query-cleaning-kit-a",
            "Cleaning Kit A",
            "20250401",
            "20260331",
        ),
        (
            "r-09-stock-query-soda-ash-light",
            "Soda Ash Light",
            "20250401",
            "20260331",
        ),
        (
            "r-11-stock-query-zero-stock-item",
            "Zero Stock Item",
            "20250401",
            "20260331",
        ),
        (
            "r-12-stock-query-hcl-to-20250401",
            "Hydrochloric Acid 33pc",
            "20250401",
            "20250401",
        ),
        (
            "r-14-stock-query-cleaning-kit-a-to-20250401",
            "Cleaning Kit A",
            "20250401",
            "20250401",
        ),
        (
            "r-15-stock-query-unknown-item",
            "Bridge Probe Item",
            "20250401",
            "20260331",
        ),
        (
            "w-02-stock-query-kit-co-item-f1",
            "Kit & Co Item",
            "20250401",
            "20260331",
        ),
        (
            "w-03-stock-query-gauge-test-50l-f1",
            "Gauge (Test) 50L",
            "20250401",
            "20260331",
        ),
        (
            "w-05-stock-query-owner-s-item-f2",
            "Owner's Item",
            "20250401",
            "20260331",
        ),
        (
            "w-07-stock-query-quote-q-item-f2",
            "Quote \"Q\" Item",
            "20250401",
            "20260331",
        ),
        (
            "w-08-stock-query-alt-unit-item-f1",
            "Alt Unit Item",
            "20250401",
            "20260331",
        ),
        (
            "w-09-stock-query-mix-5-set-large-f1",
            "Mix 5% / Set, Large",
            "20250401",
            "20260331",
        ),
        (
            "w-10-stock-query-dash-item-7-f1",
            "Dash-Item 7",
            "20250401",
            "20260331",
        ),
        (
            "alt-a-04-stock-query-alt-unit-probe-a",
            "Alt Unit Probe A",
            "20250401",
            "20260331",
        ),
        (
            "alt-a-05-stock-query-alt-unit-probe-b",
            "Alt Unit Probe B",
            "20250401",
            "20260331",
        ),
    ] {
        let rendered = render_stock_query_request(
            LAB,
            &MeasuredItemName::new(item).unwrap(),
            &period(from, to),
        );
        assert_eq!(
            crate::encode_tally_xml_request_utf16le(&rendered),
            capture(&format!("{file}.request.utf16le.xml")),
            "{file}"
        );
    }
}

/// The bare request (no item) is the built-in report envelope, so the builder's template is the
/// one the control capture sent, less its one added variable.
#[test]
fn the_bare_capture_is_the_envelope_without_the_item() {
    let bare = crate::native_statement_reports::render_report_envelope(
        "Stock Query",
        LAB,
        &period("20250401", "20260331"),
        "",
    );
    assert_eq!(
        crate::encode_tally_xml_request_utf16le(&bare),
        capture("r-3-stock-query.request.utf16le.xml")
    );
}

fn line(godown: &str, batch: Option<&str>, quantity: &str) -> (String, StockQueryBatch, String) {
    (
        godown.to_string(),
        batch.map_or(StockQueryBatch::Default, |name| {
            StockQueryBatch::Named(name.to_string())
        }),
        quantity.to_string(),
    )
}

fn lines(name: &str) -> Vec<(String, StockQueryBatch, String)> {
    match parse_stock_query(&answer(name)).expect(name) {
        StockQueryAnswer::NoLines => Vec::new(),
        StockQueryAnswer::Lines(lines) => lines
            .into_iter()
            .map(|line| {
                (
                    line.godown,
                    line.batch,
                    format!("{} {}", line.quantity.amount.as_str(), line.quantity.unit),
                )
            })
            .collect(),
    }
}

/// Each captured answer reads as its lines, in Tally's order, with each quantity signed and in
/// the unit Tally wrote; the last-purchase and last-sale groups before them are skipped.
#[test]
fn every_captured_answer_reads_as_its_lines() {
    assert_eq!(
        lines("r-4-stock-query-stockitemname"),
        [
            line("Main Location", None, "150.000 Kgs"),
            line("Main Location", Some("H-2504-A"), "300.000 Kgs"),
            line("Factory Floor", Some("H-2504-B"), "366.667 Kgs"),
        ]
    );
    assert_eq!(
        lines("r-01-stock-query-carton-box-small"),
        [line("Main Location", None, "100 Box")]
    );
    assert_eq!(
        lines("r-02-stock-query-caustic-soda-flakes"),
        [
            line("Main Location", None, "200.000 Kgs"),
            line("Main Location", Some("B-2504-A"), "200.000 Kgs"),
        ]
    );
    assert_eq!(
        lines("r-03-stock-query-cleaning-kit-a"),
        [
            line("Main Location", None, "25 Nos"),
            line("Main Location", Some("K-2504-A"), "-13 Nos"),
        ]
    );
    assert_eq!(
        lines("r-09-stock-query-soda-ash-light"),
        [
            line("Main Location", None, "1350.000 Kgs"),
            line("Factory Floor", None, "-50.000 Kgs"),
        ]
    );
    assert_eq!(
        lines("r-11-stock-query-zero-stock-item"),
        [line("Main Location", None, "-50.000 Kgs")]
    );
    assert_eq!(
        lines("r-12-stock-query-hcl-to-20250401"),
        [line("Main Location", None, "150.000 Kgs")]
    );
    assert_eq!(
        lines("r-14-stock-query-cleaning-kit-a-to-20250401"),
        [line("Main Location", None, "25 Nos")]
    );
    for (name, quantity) in [
        ("w-02-stock-query-kit-co-item-f1", "11 Nos"),
        ("w-03-stock-query-gauge-test-50l-f1", "12 Nos"),
        ("w-04-stock-query-owner-s-item-f1", "13 Nos"),
        ("w-05-stock-query-owner-s-item-f2", "13 Nos"),
        ("w-06-stock-query-quote-q-item-f1", "14 Nos"),
        ("w-07-stock-query-quote-q-item-f2", "14 Nos"),
        ("w-08-stock-query-alt-unit-item-f1", "30 Nos"),
        ("w-09-stock-query-mix-5-set-large-f1", "16 Nos"),
        ("w-10-stock-query-dash-item-7-f1", "17 Nos"),
        ("alt-a-04-stock-query-alt-unit-probe-a", "30 Nos"),
        ("alt-a-05-stock-query-alt-unit-probe-b", "36 Nos"),
    ] {
        assert_eq!(
            lines(name),
            [line("Main Location", None, quantity)],
            "{name}"
        );
    }
}

/// The empty answer, for an unknown item and for the bare request alike, is no lines, not zero.
#[test]
fn the_empty_answer_is_no_lines() {
    for name in ["r-15-stock-query-unknown-item", "r-3-stock-query"] {
        assert_eq!(
            parse_stock_query(&answer(name)),
            Ok(StockQueryAnswer::NoLines),
            "{name}"
        );
    }
}

/// `text` with `from` replaced by `to`, which must occur `times` times: a damaged capture that did
/// not change would let its test pass for nothing.
fn damaged(text: &str, from: &str, to: &str, times: usize) -> String {
    assert_eq!(text.matches(from).count(), times, "{from:?}");
    text.replace(from, to)
}

/// Each refusal fires on a capture damaged in one place, under its own code.
#[test]
fn a_damaged_answer_refuses_under_its_own_code() {
    let kit = answer("r-03-stock-query-cleaning-kit-a");
    let soda = answer("r-09-stock-query-soda-ash-light");
    let cases = [
        (
            damaged(
                &kit,
                "<STQBATCHNAME>K-2504-A</STQBATCHNAME>",
                "<STQEXPIRY>K</STQEXPIRY>",
                1,
            ),
            StockQueryError::UnknownElement,
        ),
        (
            damaged(&kit, " <STQGODOWNQTY>-13 Nos</STQGODOWNQTY>\r\n", "", 1),
            StockQueryError::PartialLine,
        ),
        (
            damaged(&kit, " <STQSALESVCHDISC></STQSALESVCHDISC>\r\n", "", 1),
            StockQueryError::PartialGroup,
        ),
        (
            damaged(&soda, "-50.000 Kgs", "(-)50.000 Kgs", 1),
            StockQueryError::QuantityUnparseable,
        ),
        (
            damaged(&soda, "1350.000 Kgs", "1,350.000 Kgs", 1),
            StockQueryError::QuantityUnparseable,
        ),
        (
            damaged(&soda, "-50.000 Kgs", "", 1),
            StockQueryError::QuantityUnparseable,
        ),
        (
            damaged(
                &soda,
                "<STQGODOWNNAME>Factory Floor</STQGODOWNNAME>",
                "<STQGODOWNNAME> </STQGODOWNNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &kit,
                "<STQGODOWNNAME>Main Location</STQGODOWNNAME>\r\n <STQBATCHNAME>K-2504-A",
                "<STQSALESDATE>23-Apr-25</STQSALESDATE>\r\n <STQBATCHNAME>K-2504-A",
                1,
            ),
            StockQueryError::PartialLine,
        ),
        (
            damaged(&kit, "</ENVELOPE>", "", 1),
            StockQueryError::Malformed,
        ),
        (
            damaged(
                &kit,
                "<STQBATCHNAME>K-2504-A</STQBATCHNAME>",
                "<STQBATCHNAME><ERROR>x</ERROR></STQBATCHNAME>",
                1,
            ),
            StockQueryError::TallyReportedFailure,
        ),
    ];
    for (text, expected) in cases {
        assert_eq!(
            parse_stock_query(&text),
            Err(expected),
            "{}",
            expected.code()
        );
    }
}

/// The name alphabet is the captured one: letters, digits, the measured punctuation and single
/// inner spaces, at most 60 characters. Everything else is refused before any request.
#[test]
fn an_item_name_outside_the_captured_alphabet_is_refused() {
    for admitted in [
        "Hydrochloric Acid 33pc",
        "Carton Box Small",
        "A",
        &"x".repeat(60),
        "Kit & Co Item",
        "Gauge (Test) 50L",
        "Owner's Item",
        "Quote \"Q\" Item",
        "Mix 5% / Set, Large",
        "Dash-Item 7",
    ] {
        assert_eq!(
            MeasuredItemName::new(admitted).map(|name| name.as_str().to_string()),
            Ok(admitted.to_string())
        );
    }
    for refused in [
        "",
        " Leading",
        "Trailing ",
        "Two  Spaces",
        "Dot.Item",
        "Semi;colon",
        "Colon:",
        "Hash#",
        "Plus+",
        "Star*",
        "At@",
        "Back\\slash",
        "Equals=",
        "[Bracket]",
        "{Brace}",
        "Less<",
        "More>",
        "Under_score",
        "Line\nBreak",
        "Carriage\rReturn",
        "Tab\tItem",
        "Caf\u{e9}",
        &"x".repeat(61),
    ] {
        assert_eq!(
            MeasuredItemName::new(refused),
            Err(StockQueryError::ItemNameUnmeasured),
            "{refused:?}"
        );
    }
}

#[test]
fn every_refusal_has_its_own_code() {
    let codes = [
        StockQueryError::ItemNameUnmeasured,
        StockQueryError::Malformed,
        StockQueryError::TallyReportedFailure,
        StockQueryError::UnknownElement,
        StockQueryError::PartialGroup,
        StockQueryError::PartialLine,
        StockQueryError::NameInvalid,
        StockQueryError::QuantityUnparseable,
        StockQueryError::LineRepeated,
    ]
    .map(StockQueryError::code);
    let distinct: std::collections::HashSet<_> = codes.iter().collect();
    assert_eq!(distinct.len(), codes.len());
    assert!(codes.iter().all(|code| code.starts_with("stock_query_")));
}

/// More shapes outside the captured one, each refused under its code: a whole group after a line,
/// two elements swapped inside a group or a triple, an over-long name, an element after the
/// envelope, text between elements, and Tally's failure element directly under the root. An
/// entity in a name is read as its character.
#[test]
fn other_shapes_outside_the_capture_refuse_and_an_entity_is_read() {
    let kit = answer("r-03-stock-query-cleaning-kit-a");
    let soda = answer("r-09-stock-query-soda-ash-light");
    let sale = "<STQSALESDATE>23-Apr-25</STQSALESDATE>\r\n <STQSALESVCHNO>Shape Buyer 1</STQSALESVCHNO>\r\n <STQSALESVCHQTY>13 Nos</STQSALESVCHQTY>\r\n <STQSALESVCHRATE>750.00</STQSALESVCHRATE>\r\n <STQSALESVCHDISC></STQSALESVCHDISC>\r\n <STQSALESVCHAMOUNT>10000.00</STQSALESVCHAMOUNT>\r\n";
    let cases = [
        (
            damaged(&kit, "</ENVELOPE>", &format!(" {sale}</ENVELOPE>"), 1),
            StockQueryError::PartialLine,
        ),
        (
            damaged(
                &soda,
                "<STQPURCVCHRATE>25.00</STQPURCVCHRATE>\r\n <STQPURCVCHDISC></STQPURCVCHDISC>",
                "<STQPURCVCHDISC></STQPURCVCHDISC>\r\n <STQPURCVCHRATE>25.00</STQPURCVCHRATE>",
                5,
            ),
            StockQueryError::PartialGroup,
        ),
        (
            damaged(
                &kit,
                "<STQGODOWNNAME>Main Location</STQGODOWNNAME>\r\n <STQBATCHNAME>K-2504-A</STQBATCHNAME>",
                "<STQBATCHNAME>K-2504-A</STQBATCHNAME>\r\n <STQGODOWNNAME>Main Location</STQGODOWNNAME>",
                1,
            ),
            StockQueryError::PartialLine,
        ),
        (
            damaged(&kit, "<STQBATCHNAME>K-2504-A</STQBATCHNAME>", &format!("<STQBATCHNAME>{}</STQBATCHNAME>", "K".repeat(200)), 1),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(&kit, "</ENVELOPE>", "</ENVELOPE>\r\n<STQGODOWNNAME>Main Location</STQGODOWNNAME>", 1),
            StockQueryError::Malformed,
        ),
        (
            damaged(&kit, "<STQBATCHNAME>K-2504-A</STQBATCHNAME>", "stray <STQBATCHNAME>K-2504-A</STQBATCHNAME>", 1),
            StockQueryError::Malformed,
        ),
        (
            damaged(&kit, "</ENVELOPE>", "<ERROR>x</ERROR>\r\n</ENVELOPE>", 1),
            StockQueryError::TallyReportedFailure,
        ),
    ];
    for (text, expected) in cases {
        assert_eq!(
            parse_stock_query(&text),
            Err(expected),
            "{}",
            expected.code()
        );
    }
    let entity = damaged(
        &soda,
        "<STQGODOWNNAME>Factory Floor</STQGODOWNNAME>",
        "<STQGODOWNNAME>Floor &amp; Bay</STQGODOWNNAME>",
        1,
    );
    match parse_stock_query(&entity) {
        Ok(StockQueryAnswer::Lines(lines)) => assert_eq!(lines[1].godown, "Floor & Bay"),
        other => panic!("{other:?}"),
    }
}

/// Shapes no capture holds, refused under their codes: the sanitiser's marker in a name or a unit
/// (Tally's `&#4;`), a name or quantity padded at either end, a godown and batch read twice, a kind of group
/// coming back after the other, an element in another case or with an attribute, a comment and a
/// declaration. Tally's failure element is its failure in any case and with an attribute. Either
/// kind of group may come first, and an empty element with no attribute
/// reads as one with no text.
#[test]
fn unmeasured_shapes_refuse_and_either_group_order_reads() {
    let kit = answer("r-03-stock-query-cleaning-kit-a");
    let soda = answer("r-09-stock-query-soda-ash-light");
    let sale = "<STQSALESDATE>23-Apr-25</STQSALESDATE>\r\n <STQSALESVCHNO>Shape Buyer 1</STQSALESVCHNO>\r\n <STQSALESVCHQTY>13 Nos</STQSALESVCHQTY>\r\n <STQSALESVCHRATE>750.00</STQSALESVCHRATE>\r\n <STQSALESVCHDISC></STQSALESVCHDISC>\r\n <STQSALESVCHAMOUNT>10000.00</STQSALESVCHAMOUNT>\r\n";
    let batch = "<STQBATCHNAME>K-2504-A</STQBATCHNAME>";
    let cases = [
        (
            damaged(&kit, batch, "<STQBATCHNAME>&#4; Primary Batch</STQBATCHNAME>", 1),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &soda,
                "<STQGODOWNNAME>Factory Floor</STQGODOWNNAME>",
                "<STQGODOWNNAME>&#4; Factory Floor</STQGODOWNNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &kit,
                "<STQGODOWNQTY>-13 Nos</STQGODOWNQTY>",
                "<STQGODOWNQTY>-13 &#4;Nos</STQGODOWNQTY>",
                1,
            ),
            StockQueryError::QuantityUnparseable,
        ),
        (
            damaged(&kit, batch, "<STQBATCHNAME> K-2504-A</STQBATCHNAME>", 1),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &kit,
                "<STQBATCHNAME>Primary Batch</STQBATCHNAME>",
                "<STQBATCHNAME>Primary Batch </STQBATCHNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(&soda, "Factory Floor</STQGODOWNNAME>", "Factory Floor </STQGODOWNNAME>", 1),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &soda,
                "</ENVELOPE>",
                " <STQGODOWNNAME>Main Location</STQGODOWNNAME>\r\n <STQBATCHNAME>Primary Batch</STQBATCHNAME>\r\n <STQGODOWNQTY>1.000 Kgs</STQGODOWNQTY>\r\n</ENVELOPE>",
                1,
            ),
            StockQueryError::LineRepeated,
        ),
        (
            damaged(
                &soda,
                "<STQPURCDATE>14-Apr-25</STQPURCDATE>",
                &format!("{sale} <STQPURCDATE>14-Apr-25</STQPURCDATE>"),
                1,
            ),
            StockQueryError::PartialGroup,
        ),
        (
            damaged(&kit, batch, "<stqbatchname>K-2504-A</stqbatchname>", 1),
            StockQueryError::UnknownElement,
        ),
        (
            damaged(&kit, "ENVELOPE>", "envelope>", 2),
            StockQueryError::Malformed,
        ),
        (
            damaged(&kit, batch, "<STQBATCHNAME TYPE=\"String\">K-2504-A</STQBATCHNAME>", 1),
            StockQueryError::Malformed,
        ),
        (
            damaged(&kit, "<ENVELOPE>", "<ENVELOPE A=\"1\">", 1),
            StockQueryError::Malformed,
        ),
        (
            damaged(
                &kit,
                "<STQSALESVCHDISC></STQSALESVCHDISC>",
                "<STQSALESVCHDISC TYPE=\"Number\"/>",
                1,
            ),
            StockQueryError::Malformed,
        ),
        (
            damaged(&kit, "</ENVELOPE>", "<!-- c -->\r\n</ENVELOPE>", 1),
            StockQueryError::Malformed,
        ),
        (
            format!("<?xml version=\"1.0\"?>\r\n{kit}"),
            StockQueryError::Malformed,
        ),
        (
            damaged(&kit, "</ENVELOPE>", "<error>x</error>\r\n</ENVELOPE>", 1),
            StockQueryError::TallyReportedFailure,
        ),
        (
            damaged(&kit, "</ENVELOPE>", "<LINEERROR X=\"1\">x</LINEERROR>\r\n</ENVELOPE>", 1),
            StockQueryError::TallyReportedFailure,
        ),
        (
            damaged(&kit, "</ENVELOPE>", "<LINEERROR X=\"1\"/>\r\n</ENVELOPE>", 1),
            StockQueryError::TallyReportedFailure,
        ),
        (
            damaged(
                &soda,
                "<STQGODOWNNAME>Factory Floor</STQGODOWNNAME>",
                "<STQGODOWNNAME> Factory Floor</STQGODOWNNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &kit,
                "<STQGODOWNQTY>-13 Nos</STQGODOWNQTY>",
                "<STQGODOWNQTY> -13 Nos</STQGODOWNQTY>",
                1,
            ),
            StockQueryError::QuantityUnparseable,
        ),
        (
            damaged(
                &kit,
                "<STQGODOWNQTY>-13 Nos</STQGODOWNQTY>",
                "<STQGODOWNQTY>-13 Nos </STQGODOWNQTY>",
                1,
            ),
            StockQueryError::QuantityUnparseable,
        ),
    ];
    for (text, expected) in cases {
        assert_eq!(
            parse_stock_query(&text),
            Err(expected),
            "{}",
            expected.code()
        );
    }
    for read in [
        damaged(
            &soda,
            "<STQPURCDATE>15-Apr-25</STQPURCDATE>",
            &format!("{sale} <STQPURCDATE>15-Apr-25</STQPURCDATE>"),
            1,
        ),
        damaged(
            &soda,
            "<STQGODOWNNAME>Main Location</STQGODOWNNAME>",
            &format!("{sale} <STQGODOWNNAME>Main Location</STQGODOWNNAME>"),
            1,
        ),
    ] {
        assert_eq!(parse_stock_query(&read), parse_stock_query(&soda));
    }
    assert_eq!(
        parse_stock_query(&damaged(
            &kit,
            "<STQSALESVCHDISC></STQSALESVCHDISC>",
            "<STQSALESVCHDISC/>",
            1
        )),
        parse_stock_query(&kit)
    );
    assert!(
        matches!(parse_stock_query(&soda), Ok(StockQueryAnswer::Lines(lines)) if lines.len() == 2)
    );
}

/// Every committed request that names an item, read from the folder, with the name as sent, its
/// entities decoded once. Requests with no `STOCKITEMNAME` (the bare one, the collection's) are
/// not names.
fn captured_item_names() -> Vec<(String, String)> {
    let folder = format!("{}/tests/fixtures/stock-query", env!("CARGO_MANIFEST_DIR"));
    let mut names = Vec::new();
    for entry in std::fs::read_dir(&folder).expect("the capture folder") {
        let file = entry
            .expect("a folder entry")
            .file_name()
            .into_string()
            .expect("ASCII");
        if !file.ends_with(".request.utf16le.xml") {
            continue;
        }
        let bytes = capture(&file);
        let units: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect();
        let text = String::from_utf16(&units).expect("UTF-16LE");
        let Some((_, rest)) = text.split_once("<STOCKITEMNAME>") else {
            continue;
        };
        let (name, _) = rest.split_once("</STOCKITEMNAME>").expect(&file);
        names.push((
            file.clone(),
            quick_xml::escape::unescape(name).expect(&file).into_owned(),
        ));
    }
    names.sort();
    names
}

/// Every character the alphabet admits beyond letters, digits and the space is in a captured name,
/// and every such character in a captured name is admitted: the alphabet is exactly the measured
/// one. For each, a capture holding it is the builder's bytes (the first test).
#[test]
fn the_admitted_punctuation_is_exactly_the_captured_punctuation() {
    let named = captured_item_names();
    // The twenty requests that name an item: a request the folder lost would not fail quietly.
    assert_eq!(named.len(), 20);
    let names: Vec<String> = named.into_iter().map(|(_, name)| name).collect();
    let captured: std::collections::BTreeSet<char> = names
        .iter()
        .flat_map(|name| name.chars())
        .filter(|c| !c.is_ascii_alphanumeric() && *c != ' ')
        .collect();
    let admitted: std::collections::BTreeSet<char> =
        STOCK_QUERY_MEASURED_PUNCTUATION.into_iter().collect();
    assert_eq!(captured, admitted);
    for name in &names {
        assert!(MeasuredItemName::new(name).is_ok(), "{name:?}");
    }
}

/// `'` and `"` were sent raw (`f1`) and escaped (`f2`); Tally answered both byte for byte alike.
/// The builder escapes once, so it sends the `f2` form and not the raw one.
#[test]
fn a_quote_raw_or_escaped_selects_the_same_item() {
    for (raw, escaped, item) in [
        (
            "w-04-stock-query-owner-s-item-f1",
            "w-05-stock-query-owner-s-item-f2",
            "Owner's Item",
        ),
        (
            "w-06-stock-query-quote-q-item-f1",
            "w-07-stock-query-quote-q-item-f2",
            "Quote \"Q\" Item",
        ),
    ] {
        assert_eq!(
            capture(&format!("{raw}.answer.utf16le.xml")),
            capture(&format!("{escaped}.answer.utf16le.xml"))
        );
        let rendered = crate::encode_tally_xml_request_utf16le(&render_stock_query_request(
            LAB,
            &MeasuredItemName::new(item).unwrap(),
            &period("20250401", "20260331"),
        ));
        assert_eq!(rendered, capture(&format!("{escaped}.request.utf16le.xml")));
        assert_ne!(rendered, capture(&format!("{raw}.request.utf16le.xml")));
    }
}

/// The stock-item collection requests of the same sittings are the stock read's own builder
/// (`AuditStockItemsV1`), byte for byte.
#[test]
fn the_collection_requests_are_the_stock_reads_bytes() {
    let company = ValidatedCompanyName::new(LAB).unwrap();
    for (file, to) in [
        ("a-01-items-to-20250401", "20250401"),
        ("a-02-items-to-20250501", "20250501"),
        ("a-03-items-to-20260331", "20260331"),
        ("w-01-items-to-20260331", "20260331"),
    ] {
        let range = ValidatedDateRange::new("20250401", to).unwrap();
        let rendered = ReadOnlyProfile::AuditStockItemsV1 {
            company: &company,
            period: &range,
        }
        .render();
        assert_eq!(
            crate::encode_tally_xml_request_utf16le(&rendered),
            capture(&format!("{file}.request.utf16le.xml")),
            "{file}"
        );
    }
}

/// In the same sitting, each item's one Stock Query line equals the stock-item collection's
/// closing quantity for it. An item with an alternate unit is the exception: Stock Query writes
/// its base unit only, and the collection's compound closing is not read as a quantity.
#[test]
fn each_line_meets_the_same_sittings_collection_closing() {
    let closing = |file: &str, item: &str| {
        parse_native_stock_items(&answer(file), LAB_GUID)
            .expect(file)
            .rows
            .into_iter()
            .find(|row| row.name == item)
            .unwrap_or_else(|| panic!("{item} in {file}"))
            .closing
            .quantity
    };
    for (query, item) in [
        ("w-02-stock-query-kit-co-item-f1", "Kit & Co Item"),
        ("w-03-stock-query-gauge-test-50l-f1", "Gauge (Test) 50L"),
        ("w-05-stock-query-owner-s-item-f2", "Owner's Item"),
        ("w-07-stock-query-quote-q-item-f2", "Quote \"Q\" Item"),
        ("w-08-stock-query-alt-unit-item-f1", "Alt Unit Item"),
        ("w-09-stock-query-mix-5-set-large-f1", "Mix 5% / Set, Large"),
        ("w-10-stock-query-dash-item-7-f1", "Dash-Item 7"),
    ] {
        let Ok(StockQueryAnswer::Lines(lines)) = parse_stock_query(&answer(query)) else {
            panic!("{query}");
        };
        assert_eq!(lines.len(), 1, "{query}");
        assert_eq!(
            closing("w-01-items-to-20260331", item),
            NativeQuantityRead::Read(lines[0].quantity.clone()),
            "{query}"
        );
    }
    for item in ["Alt Unit Probe A", "Alt Unit Probe B"] {
        assert_eq!(
            closing("alt-a-01-items-fy-s4", item),
            NativeQuantityRead::Unread,
            "{item}"
        );
    }
}

/// Punctuation sits only where a capture shows it: between letters, digits or spaces, never at
/// either end and never touching other punctuation. And no printable ASCII character outside the
/// measured set is admitted anywhere.
#[test]
fn punctuation_is_admitted_only_where_a_capture_shows_it() {
    for refused in [
        "-Item",
        "Item,",
        "&",
        "--",
        "'",
        "\"Q\" Item",
        "Item (Test)",
        "(Test) Item",
        "Mix 5%",
        "A--B",
        "Kit && Co",
        "A,-B",
        "A (/) B",
        "A '' B",
        "A -",
        "- A",
    ] {
        assert_eq!(
            MeasuredItemName::new(refused),
            Err(StockQueryError::ItemNameUnmeasured),
            "{refused:?}"
        );
    }
    for c in (0x20u8..0x7f).map(char::from) {
        let inner = format!("A{c}B");
        let expected =
            c.is_ascii_alphanumeric() || c == ' ' || STOCK_QUERY_MEASURED_PUNCTUATION.contains(&c);
        assert_eq!(MeasuredItemName::new(&inner).is_ok(), expected, "{inner:?}");
        if !expected {
            assert_eq!(
                MeasuredItemName::new(&inner),
                Err(StockQueryError::ItemNameUnmeasured),
                "{inner:?}"
            );
        }
    }
}
