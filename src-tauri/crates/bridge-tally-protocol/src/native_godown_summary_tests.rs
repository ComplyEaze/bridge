use super::*;
use crate::{
    native_stock_summary::{
        parse_native_stock_summary_report, NativeQuantityRead, NativeReportLine,
        NativeStockQuantity, NativeStockReport,
    },
    outstandings_shared::DateBoundaryProfile,
    text_encoding::{
        decode_tally_xml_response_bytes_limited, decode_xml_bytes, ExpectedTallyTextEncoding,
    },
};
use bridge_tally_primitives::{ExactDecimal, TallyDate};

const LIVE: &[u8] =
    include_bytes!("../tests/fixtures/builtin_godown_summary_shape_lab_fy_live.utf16le.xml");
const REQUEST: &[u8] =
    include_bytes!("../tests/fixtures/builtin_godown_summary_shape_lab_fy_request.utf16le.xml");
/// The Stock Summary read in the same sitting: byte-equal to this committed capture.
const STOCK_SUMMARY: &[u8] =
    include_bytes!("../tests/fixtures/stock_summary_report_shape_lab_fy_live.utf16le.xml");

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

fn decimal(value: &str) -> ExactDecimal {
    ExactDecimal::parse(value).unwrap()
}

fn total_of(report: &NativeStockReport) -> Option<ExactDecimal> {
    match report {
        NativeStockReport::Lines { total, .. } => total.clone(),
        NativeStockReport::Empty => None,
    }
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
        render_native_godown_summary_request("BRIDGE SHAPE LAB", &period),
        decode_xml_bytes(REQUEST).unwrap()
    );
}

#[test]
fn the_captured_answer_is_one_line_per_godown_with_an_empty_quantity_kept_empty() {
    assert_eq!(
        parse_native_stock_summary_report(&response(LIVE)),
        Ok(NativeStockReport::Lines {
            total: Some(decimal("3000.01")),
            present: 2,
            empty: 0,
            lines: vec![
                NativeReportLine {
                    name: "Factory Floor".to_string(),
                    quantity: NativeQuantityRead::Read(NativeStockQuantity {
                        amount: decimal("316.667"),
                        unit: "Kgs".to_string(),
                    }),
                    amount: Some(decimal("-11000.00")),
                },
                NativeReportLine {
                    name: "Main Location".to_string(),
                    quantity: NativeQuantityRead::Empty,
                    amount: Some(decimal("14000.01")),
                },
            ],
        })
    );
}

#[test]
fn the_godown_amounts_sum_to_the_stock_summary_total_of_the_same_sitting() {
    let godowns = parse_native_stock_summary_report(&response(LIVE)).unwrap();
    let stock = parse_native_stock_summary_report(&response(STOCK_SUMMARY)).unwrap();
    assert!(total_of(&godowns).is_some());
    assert_eq!(total_of(&godowns), total_of(&stock));
}
