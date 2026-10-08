use super::*;
use crate::{
    native_outstandings::NativeLedgerSnapshotPeriod,
    outstandings_shared::DateBoundaryProfile,
    text_encoding::{
        decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
    },
};
use bridge_tally_primitives::{ExactDecimal, TallyDate};

const LIVE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_stock_shape_lab_fy_live.utf16le.xml");
const REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_stock_shape_lab_fy_request.utf16le.xml");
const EMPTY_ENVELOPE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_negative_ledgers_probe_b_fy_empty_live.utf16le.xml");
const UNKNOWN_REPORT: &[u8] =
    include_bytes!("../tests/fixtures/builtin_unknown_report_refusal_live.utf16le.xml");

/// A captured response, decoded as production decodes it.
fn response(bytes: &[u8]) -> String {
    decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap()
    .text
}

fn live() -> String {
    response(LIVE)
}

fn present(value: &str) -> NativeStatementAmount {
    NativeStatementAmount::Present(ExactDecimal::parse(value).unwrap())
}

fn invalid_response(
    code: &'static str,
) -> Result<NativeNegativeStockListing, NativeNegativeStockError> {
    Err(NativeNegativeStockError::InvalidResponse(code))
}

/// The captured answer compacted to one line, so a mutation matches one string.
fn compact(xml: &str) -> String {
    xml.split('\n').map(str::trim).collect::<String>()
}

fn mutate(from: &str, to: &str) -> String {
    let compact = compact(&live());
    assert!(compact.contains(from), "mutation target not found: {from}");
    compact.replacen(from, to, 1)
}

#[test]
fn the_request_is_byte_equal_to_its_committed_capture() {
    let period = NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20250401").unwrap(),
        TallyDate::parse("20260331").unwrap(),
    )
    .unwrap();
    assert_eq!(
        render_native_negative_stock_request("BRIDGE SHAPE LAB", &period),
        decode_xml_bytes(REQUEST).unwrap()
    );
}

#[test]
fn the_captured_answer_parses_to_its_five_items_in_tallys_order() {
    let parsed = parse_native_negative_stock(&live()).unwrap();
    let names: Vec<_> = parsed.items.iter().map(|item| item.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "Carton Box Small",
            "Cleaning Kit A",
            "HDPE Drum 50L",
            "Sulphuric Acid 98pc",
            "Zero Stock Item"
        ]
    );
    let first = &parsed.items[0];
    assert_eq!(first.rate, present("25.00"));
    assert_eq!(first.value, present("2500.00"));
    let quantity = first.quantity.read().expect("a quantity was read");
    assert_eq!(quantity.amount.as_str(), "100");
    assert_eq!(quantity.unit, "Box");
}

#[test]
fn a_negative_quantity_keeps_its_sign_and_empty_rate_and_value_stay_empty_not_zero() {
    let parsed = parse_native_negative_stock(&live()).unwrap();
    let last = parsed.items.last().unwrap();
    let quantity = last.quantity.read().unwrap();
    assert_eq!(quantity.amount.as_str(), "-50.000");
    assert_eq!(quantity.unit, "Kgs");
    assert_eq!(last.rate, NativeStatementAmount::Empty);
    assert_eq!(last.value, NativeStatementAmount::Empty);
}

#[test]
fn a_value_that_is_a_credit_is_a_positive_amount_as_tally_sent_it() {
    // Four of the five captured items have a positive quantity and a positive (credit) value (§12a.13).
    let parsed = parse_native_negative_stock(&live()).unwrap();
    assert_eq!(parsed.items[3].value, present("5000.00"));
}

#[test]
fn an_empty_envelope_is_refused_because_it_cannot_be_told_from_an_unrendered_report() {
    assert_eq!(
        parse_native_negative_stock(&response(EMPTY_ENVELOPE)),
        Err(NativeNegativeStockError::EmptyEnvelope)
    );
    assert_eq!(
        parse_native_negative_stock("<ENVELOPE/>"),
        Err(NativeNegativeStockError::EmptyEnvelope)
    );
}

#[test]
fn an_unknown_report_name_is_a_reported_failure_by_structure_and_a_bare_response_is_unknown() {
    assert_eq!(
        parse_native_negative_stock(&response(UNKNOWN_REPORT)),
        Err(NativeNegativeStockError::TallyReportedFailure)
    );
    assert_eq!(
        parse_native_negative_stock("<RESPONSE>Unknown Request</RESPONSE>"),
        Err(NativeNegativeStockError::UnknownReport)
    );
}

#[test]
fn a_failure_signal_beside_the_rows_beats_them_and_one_inside_a_row_is_a_shape_error() {
    for tail in ["<LINEERROR>x</LINEERROR>", "<STATUS/>", "<ERROR/>"] {
        let xml = mutate("</ENVELOPE>", &format!("{tail}</ENVELOPE>"));
        assert_eq!(
            parse_native_negative_stock(&xml),
            Err(NativeNegativeStockError::TallyReportedFailure),
            "{tail}"
        );
    }
    let in_row = mutate(
        "<DSPCLRATE>25.00</DSPCLRATE>",
        "<LINEERROR>x</LINEERROR><DSPCLRATE>25.00</DSPCLRATE>",
    );
    assert_eq!(
        parse_native_negative_stock(&in_row),
        invalid_response("negative_stock_closing_shape")
    );
}

#[test]
fn an_amount_that_is_not_a_plain_signed_decimal_is_refused() {
    for bad in ["2,500.00", "2500.00 Dr", " 2500.00", "1e3"] {
        let xml = mutate(
            "<DSPCLAMTA>2500.00</DSPCLAMTA>",
            &format!("<DSPCLAMTA>{bad}</DSPCLAMTA>"),
        );
        assert_eq!(
            parse_native_negative_stock(&xml),
            Err(NativeNegativeStockError::InvalidAmount),
            "{bad}"
        );
    }
}

#[test]
fn an_unreadable_quantity_is_read_as_unread_and_does_not_refuse_the_answer() {
    let xml = mutate(
        "<DSPCLQTY>100 Box</DSPCLQTY>",
        "<DSPCLQTY>1 000 Box</DSPCLQTY>",
    );
    let parsed = parse_native_negative_stock(&xml).unwrap();
    assert_eq!(parsed.items[0].quantity, NativeQuantityRead::Unread);
    let empty = mutate("<DSPCLQTY>100 Box</DSPCLQTY>", "<DSPCLQTY/>");
    assert_eq!(
        parse_native_negative_stock(&empty).unwrap().items[0].quantity,
        NativeQuantityRead::Empty
    );
}

#[test]
fn a_name_without_its_row_and_a_row_without_its_name_are_refused() {
    let doubled = mutate(
        "<DSPACCNAME><DSPDISPNAME>Cleaning Kit A</DSPDISPNAME></DSPACCNAME>",
        "<DSPACCNAME><DSPDISPNAME>Cleaning Kit A</DSPDISPNAME></DSPACCNAME><DSPACCNAME><DSPDISPNAME>X</DSPDISPNAME></DSPACCNAME>",
    );
    assert_eq!(
        parse_native_negative_stock(&doubled),
        invalid_response("negative_stock_name_without_row")
    );
    let no_name = mutate(
        "<DSPACCNAME><DSPDISPNAME>Carton Box Small</DSPDISPNAME></DSPACCNAME>",
        "",
    );
    assert_eq!(
        parse_native_negative_stock(&no_name),
        invalid_response("negative_stock_row_without_name")
    );
    let dangling = mutate(
        "</ENVELOPE>",
        "<DSPACCNAME><DSPDISPNAME>Late</DSPDISPNAME></DSPACCNAME></ENVELOPE>",
    );
    assert_eq!(
        parse_native_negative_stock(&dangling),
        invalid_response("negative_stock_name_without_row")
    );
}

#[test]
fn an_item_listed_twice_is_refused() {
    let twice = mutate(
        "<DSPDISPNAME>Cleaning Kit A</DSPDISPNAME>",
        "<DSPDISPNAME>Carton Box Small</DSPDISPNAME>",
    );
    assert_eq!(
        parse_native_negative_stock(&twice),
        Err(NativeNegativeStockError::DuplicateItem)
    );
}

#[test]
fn a_closing_without_its_value_a_repeated_column_and_unknown_elements_are_refused() {
    let no_value = mutate("<DSPCLAMTA>2500.00</DSPCLAMTA>", "");
    assert_eq!(
        parse_native_negative_stock(&no_value),
        invalid_response("negative_stock_value_missing")
    );
    let repeated = mutate(
        "<DSPCLRATE>25.00</DSPCLRATE>",
        "<DSPCLRATE>25.00</DSPCLRATE><DSPCLRATE>26.00</DSPCLRATE>",
    );
    assert_eq!(
        parse_native_negative_stock(&repeated),
        invalid_response("negative_stock_closing_shape")
    );
    let extra = mutate("</ENVELOPE>", "<DSPEXTRA>1</DSPEXTRA></ENVELOPE>");
    assert_eq!(
        parse_native_negative_stock(&extra),
        invalid_response("negative_stock_unexpected_element")
    );
    let two_closings = mutate(
        "<DSPSTKINFO><DSPSTKCL><DSPCLQTY>100 Box</DSPCLQTY>",
        "<DSPSTKINFO><DSPSTKCL><DSPCLQTY>1 Box</DSPCLQTY><DSPCLRATE>1</DSPCLRATE><DSPCLAMTA>1</DSPCLAMTA></DSPSTKCL><DSPSTKCL><DSPCLQTY>100 Box</DSPCLQTY>",
    );
    assert_eq!(
        parse_native_negative_stock(&two_closings),
        invalid_response("negative_stock_row_shape")
    );
}

#[test]
fn a_blank_item_name_a_repeated_value_and_content_after_the_envelope_are_refused() {
    let blank = mutate(
        "<DSPDISPNAME>Cleaning Kit A</DSPDISPNAME>",
        "<DSPDISPNAME>  </DSPDISPNAME>",
    );
    assert_eq!(
        parse_native_negative_stock(&blank),
        invalid_response("negative_stock_item_name_missing")
    );
    let twice = mutate(
        "<DSPCLAMTA>2500.00</DSPCLAMTA>",
        "<DSPCLAMTA>2500.00</DSPCLAMTA><DSPCLAMTA>1.00</DSPCLAMTA>",
    );
    assert_eq!(
        parse_native_negative_stock(&twice),
        invalid_response("negative_stock_closing_shape")
    );
    let after = mutate(
        "</ENVELOPE>",
        "</ENVELOPE><DSPACCNAME><DSPDISPNAME>X</DSPDISPNAME></DSPACCNAME>",
    );
    assert_eq!(
        parse_native_negative_stock(&after),
        invalid_response("negative_stock_trailing_content")
    );
}

#[test]
fn text_a_wrapper_and_a_cut_response_are_refused() {
    assert_eq!(
        parse_native_negative_stock(&mutate("<DSPACCNAME>", "stray<DSPACCNAME>")),
        invalid_response("negative_stock_stray_text")
    );
    assert_eq!(
        parse_native_negative_stock(
            "<ENVELOPE><HEADER><VERSION>1</VERSION></HEADER><BODY><DATA></DATA></BODY></ENVELOPE>"
        ),
        invalid_response("negative_stock_wrapper_without_failure_signal")
    );
    let full = compact(&live());
    let end = full.find("</DSPSTKINFO>").unwrap() + "</DSPSTKINFO>".len();
    assert_eq!(
        parse_native_negative_stock(&full[..end]),
        invalid_response("negative_stock_envelope_unterminated")
    );
    assert_eq!(
        parse_native_negative_stock("<REPORT/>"),
        invalid_response("negative_stock_root_not_envelope")
    );
}

#[test]
fn every_error_has_its_own_stable_code() {
    assert_eq!(
        [
            NativeNegativeStockError::TallyReportedFailure.code(),
            NativeNegativeStockError::UnknownReport.code(),
            NativeNegativeStockError::EmptyEnvelope.code(),
            NativeNegativeStockError::InvalidAmount.code(),
            NativeNegativeStockError::DuplicateItem.code(),
        ],
        [
            "negative_stock_tally_reported_failure",
            "negative_stock_report_unknown",
            "negative_stock_empty_envelope",
            "negative_stock_amount_invalid",
            "negative_stock_item_duplicated",
        ]
    );
    assert_eq!(NativeNegativeStockError::InvalidResponse("x").code(), "x");
}

#[test]
fn an_absent_quantity_or_rate_is_refused_while_an_empty_one_is_read() {
    // The capture shows all three columns on every row; an absent element is not an empty one.
    let no_rate = mutate("<DSPCLRATE>25.00</DSPCLRATE>", "");
    assert_eq!(
        parse_native_negative_stock(&no_rate),
        invalid_response("negative_stock_column_missing")
    );
    let no_quantity = mutate("<DSPCLQTY>100 Box</DSPCLQTY>", "");
    assert_eq!(
        parse_native_negative_stock(&no_quantity),
        invalid_response("negative_stock_column_missing")
    );
    let empty_rate = mutate("<DSPCLRATE>25.00</DSPCLRATE>", "<DSPCLRATE/>");
    assert_eq!(
        parse_native_negative_stock(&empty_rate).unwrap().items[0].rate,
        NativeStatementAmount::Empty
    );
    let twice = mutate(
        "<DSPCLQTY>100 Box</DSPCLQTY>",
        "<DSPCLQTY>100 Box</DSPCLQTY><DSPCLQTY>1 Box</DSPCLQTY>",
    );
    assert_eq!(
        parse_native_negative_stock(&twice),
        invalid_response("negative_stock_closing_shape")
    );
    let bad_rate = mutate(
        "<DSPCLRATE>25.00</DSPCLRATE>",
        "<DSPCLRATE>25,00</DSPCLRATE>",
    );
    assert_eq!(
        parse_native_negative_stock(&bad_rate),
        Err(NativeNegativeStockError::InvalidAmount)
    );
}

#[test]
fn the_remaining_shape_refusals_each_return_their_own_code() {
    let cases: [(String, &str); 8] = [
        (
            mutate("</ENVELOPE>", "<DSPEXTRA/></ENVELOPE>"),
            "negative_stock_unexpected_empty_element",
        ),
        (
            mutate("</ENVELOPE>", "<![CDATA[x]]></ENVELOPE>"),
            "negative_stock_unexpected_content",
        ),
        (
            mutate(
                "<DSPDISPNAME>Cleaning Kit A</DSPDISPNAME>",
                "<DSPDISPNAME>A</DSPDISPNAME><DSPDISPNAME>B</DSPDISPNAME>",
            ),
            "negative_stock_name_shape",
        ),
        (
            mutate(
                "<DSPSTKINFO><DSPSTKCL><DSPCLQTY>100 Box</DSPCLQTY>",
                "<DSPSTKINFO></DSPSTKINFO><DSPSTKINFO><DSPSTKCL><DSPCLQTY>100 Box</DSPCLQTY>",
            ),
            "negative_stock_closing_missing",
        ),
        (
            "<REPORT></REPORT>".to_string(),
            "negative_stock_root_not_envelope",
        ),
        (
            "<ENVELOPE><HEADER><DSPEXTRA>1</DSPEXTRA></HEADER></ENVELOPE>".to_string(),
            "negative_stock_unexpected_element",
        ),
        (
            mutate("<DSPCLQTY>100 Box</DSPCLQTY>", "<DSPCLQTY><X/></DSPCLQTY>"),
            "negative_stock_text_invalid",
        ),
        (
            mutate("</DSPSTKCL>", "<DSPEXTRA><X/></DSPEXTRA></DSPSTKCL>"),
            "negative_stock_text_invalid",
        ),
    ];
    for (xml, code) in cases {
        assert_eq!(
            parse_native_negative_stock(&xml),
            invalid_response(code),
            "{code}"
        );
    }
}
