use super::*;
use crate::outstandings_shared::DateBoundaryProfile;
use bridge_tally_primitives::TallyDate;

const LAB: &str = "BRIDGE SHAPE LAB";

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

/// The name alphabet is the captured one: letters, digits and single inner spaces, at most 60
/// characters. Everything else is refused before any request.
#[test]
fn an_item_name_outside_the_captured_alphabet_is_refused() {
    for admitted in [
        "Hydrochloric Acid 33pc",
        "Carton Box Small",
        "A",
        &"x".repeat(60),
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
        "Kit & Co Item",
        "Owner's Item",
        "Mix 5% / Set, Large",
        "Dash-Item 7",
        "Dot.Item",
        "(Paren",
        "Paren)",
        "Quote\"Q",
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
        StockQueryError::GroupsWithoutLines,
        StockQueryError::UnitsDiffer,
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

/// A name or quantity holding a raw control character, or one that hides or reorders the text
/// around it, raw or as a legal reference, is refused: the sanitiser marks only the reference form
/// of a forbidden character, so the raw form reached the parser before. A joiner, which Indic
/// spelling needs, is read.
#[test]
fn a_hidden_character_in_a_name_or_quantity_refuses() {
    let kit = answer("r-03-stock-query-cleaning-kit-a");
    let soda = answer("r-09-stock-query-soda-ash-light");
    let batch = "<STQBATCHNAME>K-2504-A</STQBATCHNAME>";
    let godown = "<STQGODOWNNAME>Factory Floor</STQGODOWNNAME>";
    let quantity = "<STQGODOWNQTY>-13 Nos</STQGODOWNQTY>";
    let cases = [
        (
            damaged(
                &kit,
                batch,
                "<STQBATCHNAME>\u{4} Primary Batch</STQBATCHNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &soda,
                godown,
                "<STQGODOWNNAME>Factory\u{200B} Floor</STQGODOWNNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &soda,
                godown,
                "<STQGODOWNNAME>Factory&#8203; Floor</STQGODOWNNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &kit,
                batch,
                "<STQBATCHNAME>\u{FEFF}K-2504-A</STQBATCHNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &soda,
                godown,
                "<STQGODOWNNAME>Factory\u{202E} Floor</STQGODOWNNAME>",
                1,
            ),
            StockQueryError::NameInvalid,
        ),
        (
            damaged(
                &kit,
                quantity,
                "<STQGODOWNQTY>-13\u{FEFF} Nos</STQGODOWNQTY>",
                1,
            ),
            StockQueryError::QuantityUnparseable,
        ),
        (
            damaged(
                &kit,
                quantity,
                "<STQGODOWNQTY>-13 Nos\u{7}</STQGODOWNQTY>",
                1,
            ),
            StockQueryError::QuantityUnparseable,
        ),
    ];
    for (text, expected) in cases {
        assert_eq!(parse_stock_query(&text), Err(expected), "{text:?}");
    }
    let joined = "\u{915}\u{94D}\u{200D}\u{937} Godown";
    let read = parse_stock_query(&damaged(
        &soda,
        godown,
        &format!("<STQGODOWNNAME>{joined}</STQGODOWNNAME>"),
        1,
    ));
    assert!(
        matches!(&read, Ok(StockQueryAnswer::Lines(lines)) if lines.iter().any(|line| line.godown == joined)),
        "{read:?}"
    );
}

/// A group with no line after it, and lines in two units, are refused: no capture holds either.
/// Read as no lines, a group alone could not be told from a name Tally does not know; summed, lines
/// in two units are no quantity.
#[test]
fn a_group_without_a_line_and_lines_in_two_units_refuse() {
    let zero = answer("r-11-stock-query-zero-stock-item");
    let kit = answer("r-03-stock-query-cleaning-kit-a");
    let line = " <STQGODOWNNAME>Main Location</STQGODOWNNAME>\r\n <STQBATCHNAME>Primary Batch</STQBATCHNAME>\r\n <STQGODOWNQTY>-50.000 Kgs</STQGODOWNQTY>\r\n";
    assert_eq!(
        parse_stock_query(&damaged(&zero, line, "", 1)),
        Err(StockQueryError::GroupsWithoutLines)
    );
    assert_eq!(
        parse_stock_query(&damaged(&kit, "-13 Nos", "-13 Box", 1)),
        Err(StockQueryError::UnitsDiffer)
    );
    assert!(
        matches!(parse_stock_query(&zero), Ok(StockQueryAnswer::Lines(lines)) if lines.len() == 1)
    );
}
