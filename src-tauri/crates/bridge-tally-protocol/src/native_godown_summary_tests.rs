use super::*;
use crate::{
    native_stock_summary::{
        parse_native_stock_summary_exploded, parse_native_stock_summary_report,
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

// The 30 Oct sitting, after two root items and a godown under Main Location were keyed and one
// item was moved between godowns on 15 Sep 2025.
const GODOWNS_FY: &[u8] =
    include_bytes!("../tests/fixtures/builtin_godown_closing_godown_summary_fy_live.utf16le.xml");
const GODOWNS_AUG: &[u8] = include_bytes!(
    "../tests/fixtures/builtin_godown_closing_godown_summary_to_20250831_live.utf16le.xml"
);
const STOCK_FY: &[u8] =
    include_bytes!("../tests/fixtures/builtin_godown_closing_stock_summary_fy_live.utf16le.xml");
const STOCK_AUG: &[u8] = include_bytes!(
    "../tests/fixtures/builtin_godown_closing_stock_summary_to_20250831_live.utf16le.xml"
);
const EXPLODED_FY: &[u8] = include_bytes!(
    "../tests/fixtures/builtin_godown_closing_stock_summary_exploded_fy_live.utf16le.xml"
);
const EXPLODED_AUG: &[u8] = include_bytes!(
    "../tests/fixtures/builtin_godown_closing_stock_summary_exploded_to_20250831_live.utf16le.xml"
);
const EXPLODED_REQUEST: &[u8] = include_bytes!(
    "../tests/fixtures/builtin_godown_closing_stock_summary_exploded_fy_request.utf16le.xml"
);

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

/// The response with `from` replaced by `to` exactly `count` times, so a changed input cannot
/// silently do nothing.
fn damaged(bytes: &[u8], from: &str, to: &str, count: usize) -> String {
    let text = response(bytes);
    assert_eq!(text.matches(from).count(), count, "{from}");
    text.replace(from, to)
}

fn decimal(value: &str) -> ExactDecimal {
    ExactDecimal::parse(value).unwrap()
}

fn quantity(amount: &str, unit: &str) -> NativeStockQuantity {
    NativeStockQuantity {
        amount: decimal(amount),
        unit: unit.to_string(),
    }
}

fn year_end() -> NativeLedgerSnapshotPeriod {
    NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20250401").unwrap(),
        TallyDate::parse("20260331").unwrap(),
    )
    .unwrap()
}

fn godown_line(godown: &str, amount: &str, unit: &str, value: &str) -> NativeItemGodownLine {
    NativeItemGodownLine {
        godown: godown.to_string(),
        quantity: NativeQuantityRead::Read(quantity(amount, unit)),
        value: decimal(value),
    }
}

fn items(exploded: &str, plain: &[u8]) -> Vec<NativeItemGodowns> {
    let exploded = parse_native_stock_summary_exploded(exploded)
        .unwrap()
        .unwrap();
    let plain = parse_native_stock_summary_report(&response(plain)).unwrap();
    match items_by_godown(&exploded, &plain).unwrap() {
        NativeRootItemsByGodown::Lines(items) => items,
        NativeRootItemsByGodown::ExplodeNotObserved => panic!("the captures were exploded"),
    }
}

fn outcome_of(exploded: &str, item: &str) -> NativeItemByGodown {
    items(exploded, STOCK_FY)
        .into_iter()
        .find(|found| found.item == item)
        .unwrap()
        .outcome
}

#[test]
fn the_request_is_text_equal_to_its_committed_capture() {
    assert_eq!(
        render_native_godown_summary_request("BRIDGE SHAPE LAB", &year_end()),
        decode_xml_bytes(REQUEST).unwrap()
    );
}

#[test]
fn the_exploded_request_is_text_equal_to_its_committed_capture() {
    assert_eq!(
        render_native_stock_summary_exploded_request("BRIDGE SHAPE LAB", &year_end()),
        decode_xml_bytes(EXPLODED_REQUEST).unwrap()
    );
}

#[test]
fn the_captured_answer_is_one_line_per_godown_with_a_net_quantity_and_an_empty_one_kept_empty() {
    assert_eq!(
        parse_native_godown_summary(&response(LIVE)),
        Ok(NativeGodownSummary::Lines {
            total: Some(decimal("3000.01")),
            empty: 0,
            lines: vec![
                NativeGodownLine {
                    name: "Factory Floor".to_string(),
                    quantity: GodownNetQuantity::NetAcrossItems(quantity("316.667", "Kgs")),
                    amount: Some(decimal("-11000.00")),
                },
                NativeGodownLine {
                    name: "Main Location".to_string(),
                    quantity: GodownNetQuantity::Empty,
                    amount: Some(decimal("14000.01")),
                },
            ],
        })
    );
}

#[test]
fn the_godowns_tie_to_the_stock_summary_of_the_same_sitting_at_each_date() {
    for (godowns, stock, total) in [
        (LIVE, STOCK_SUMMARY, "3000.01"),
        (GODOWNS_AUG, STOCK_AUG, "2300.01"),
        (GODOWNS_FY, STOCK_FY, "2292.01"),
    ] {
        let godowns = parse_native_godown_summary(&response(godowns)).unwrap();
        let stock = parse_native_stock_summary_report(&response(stock)).unwrap();
        assert_eq!(
            tie_godowns(&godowns, &stock),
            NativeGodownTie::Tied {
                total: decimal(total)
            }
        );
    }
}

#[test]
fn a_godown_under_another_has_no_line_of_its_own_in_the_plain_summary() {
    let NativeGodownSummary::Lines { lines, .. } =
        parse_native_godown_summary(&response(GODOWNS_FY)).unwrap()
    else {
        panic!("lines expected");
    };
    let names: Vec<&str> = lines.iter().map(|line| line.name.as_str()).collect();
    assert_eq!(names, ["Factory Floor", "Main Location"]);
    // Main Location's amount holds Godown Lab C's 250.00 as well as its own.
    assert_eq!(lines[1].amount, Some(decimal("13530.01")));
}

#[test]
fn godown_totals_that_differ_refuse_with_both_sides() {
    let godowns =
        parse_native_godown_summary(&damaged(GODOWNS_FY, "13530.01", "13530.02", 1)).unwrap();
    let stock = parse_native_stock_summary_report(&response(STOCK_FY)).unwrap();
    assert_eq!(
        tie_godowns(&godowns, &stock),
        NativeGodownTie::Differs {
            godown_total: decimal("2292.02"),
            stock_summary_total: decimal("2292.01"),
        }
    );
}

#[test]
fn an_empty_godown_amount_is_not_zero_and_does_not_tie() {
    let godowns = parse_native_godown_summary(&damaged(
        GODOWNS_FY,
        "<DSPCLAMTA>-11238.00</DSPCLAMTA>",
        "<DSPCLAMTA></DSPCLAMTA>",
        1,
    ))
    .unwrap();
    let stock = parse_native_stock_summary_report(&response(STOCK_FY)).unwrap();
    assert_eq!(
        tie_godowns(&godowns, &stock),
        NativeGodownTie::NotComparable {
            godown_total: Some(decimal("13530.01")),
            stock_summary_total: Some(decimal("2292.01")),
            godown_empty_amounts: 1,
        }
    );
}

#[test]
fn an_empty_godown_summary_is_not_a_total_of_zero() {
    let godowns = parse_native_godown_summary("<ENVELOPE></ENVELOPE>").unwrap();
    assert_eq!(godowns, NativeGodownSummary::Empty);
    let stock = parse_native_stock_summary_report(&response(STOCK_FY)).unwrap();
    assert_eq!(
        tie_godowns(&godowns, &stock),
        NativeGodownTie::NotComparable {
            godown_total: None,
            stock_summary_total: Some(decimal("2292.01")),
            godown_empty_amounts: 0,
        }
    );
}

#[test]
fn each_godown_is_valued_at_its_own_cost_and_the_values_add_up_to_the_item() {
    assert_eq!(
        items(&response(EXPLODED_FY), STOCK_FY),
        vec![
            NativeItemGodowns {
                item: "Godown Single Item".to_string(),
                outcome: NativeItemByGodown::Tied {
                    value: decimal("-150.00"),
                    quantity: GodownQuantityCheck::Added,
                    godowns: vec![godown_line("Factory Floor", "15", "Nos", "-150.00")],
                },
            },
            NativeItemGodowns {
                item: "Godown Split Item".to_string(),
                // 22 Nos in Main Location is 220.00 at its own 10.00: not 245.52 at the item's
                // 11.16, nor 242.00 at the 11.00 it had before the transfer. The three add up to
                // the item's 558.00.
                outcome: NativeItemByGodown::Tied {
                    value: decimal("-558.00"),
                    quantity: GodownQuantityCheck::Added,
                    godowns: vec![
                        godown_line("Factory Floor", "8", "Nos", "-88.00"),
                        godown_line("Godown Lab C", "20", "Nos", "-250.00"),
                        godown_line("Main Location", "22", "Nos", "-220.00"),
                    ],
                },
            },
        ]
    );
}

#[test]
fn before_the_transfer_the_split_item_is_in_two_godowns() {
    let found = items(&response(EXPLODED_AUG), STOCK_AUG);
    assert_eq!(found.len(), 2);
    assert_eq!(
        found[1].outcome,
        NativeItemByGodown::Tied {
            value: decimal("-550.00"),
            quantity: GodownQuantityCheck::Added,
            godowns: vec![
                godown_line("Godown Lab C", "20", "Nos", "-250.00"),
                godown_line("Main Location", "30", "Nos", "-300.00"),
            ],
        }
    );
}

#[test]
fn an_item_inside_a_stock_group_has_no_line_per_godown() {
    let exploded = parse_native_stock_summary_exploded(&response(EXPLODED_FY))
        .unwrap()
        .unwrap();
    // Three groups, eight items inside them and two items under the root.
    assert_eq!(exploded.len(), 13);
    let with_godowns: Vec<&str> = exploded
        .iter()
        .filter(|line| !line.sub_lines.is_empty())
        .map(|line| line.line.name.as_str())
        .collect();
    assert_eq!(with_godowns, ["Godown Single Item", "Godown Split Item"]);
}

#[test]
fn the_plain_parser_refuses_the_exploded_answer() {
    assert_eq!(
        parse_native_stock_summary_report(&response(EXPLODED_FY)),
        Err(NativeStockError::Malformed(
            "stock_report_unexpected_element"
        ))
    );
}

#[test]
fn godown_values_that_do_not_add_up_return_no_line() {
    let text = damaged(
        EXPLODED_FY,
        "<DSPCLAMTA>-220.00</DSPCLAMTA>",
        "<DSPCLAMTA>-242.00</DSPCLAMTA>",
        1,
    );
    assert_eq!(
        outcome_of(&text, "Godown Split Item"),
        NativeItemByGodown::ValuesDiffer {
            item_value: decimal("-558.00"),
            godown_sum: decimal("-580.00"),
        }
    );
}

#[test]
fn godown_quantities_that_do_not_add_up_return_no_line() {
    let text = damaged(
        EXPLODED_FY,
        "<DSPCLQTY>22 Nos</DSPCLQTY>",
        "<DSPCLQTY>23 Nos</DSPCLQTY>",
        1,
    );
    assert_eq!(
        outcome_of(&text, "Godown Split Item"),
        NativeItemByGodown::QuantitiesDiffer {
            item_quantity: quantity("50", "Nos"),
            godown_sum: decimal("51"),
        }
    );
}

#[test]
fn a_quantity_in_another_unit_is_not_compared() {
    let text = damaged(
        EXPLODED_FY,
        "<DSPCLQTY>22 Nos</DSPCLQTY>",
        "<DSPCLQTY>22 Box</DSPCLQTY>",
        1,
    );
    assert!(matches!(
        outcome_of(&text, "Godown Split Item"),
        NativeItemByGodown::Tied {
            quantity: GodownQuantityCheck::NotCompared,
            ..
        }
    ));
}

#[test]
fn an_empty_godown_value_is_not_zero() {
    let text = damaged(
        EXPLODED_FY,
        "<DSPCLAMTA>-220.00</DSPCLAMTA>",
        "<DSPCLAMTA></DSPCLAMTA>",
        1,
    );
    assert_eq!(
        outcome_of(&text, "Godown Split Item"),
        NativeItemByGodown::ValueEmpty
    );
}

#[test]
fn a_named_batch_is_not_measured() {
    let text = damaged(
        EXPLODED_FY,
        "<SSBATCH></SSBATCH>\r\n  <SSGODOWN>Main Location</SSGODOWN>",
        "<SSBATCH>B-1</SSBATCH>\r\n  <SSGODOWN>Main Location</SSGODOWN>",
        1,
    );
    assert_eq!(
        outcome_of(&text, "Godown Split Item"),
        NativeItemByGodown::BatchNamed
    );
}

#[test]
fn a_godown_named_twice_returns_no_line() {
    let text = damaged(
        EXPLODED_FY,
        "<SSGODOWN>Godown Lab C</SSGODOWN>",
        "<SSGODOWN>Main Location</SSGODOWN>",
        1,
    );
    assert_eq!(
        outcome_of(&text, "Godown Split Item"),
        NativeItemByGodown::GodownRepeated
    );
}

#[test]
fn a_line_the_plain_report_does_not_show_with_its_value_is_not_tied() {
    let exploded = damaged(
        EXPLODED_FY,
        "<DSPCLAMTA>-558.00</DSPCLAMTA>",
        "<DSPCLAMTA>-559.00</DSPCLAMTA>",
        1,
    );
    assert_eq!(
        outcome_of(&exploded, "Godown Split Item"),
        NativeItemByGodown::NotInPlainReport
    );
}

#[test]
fn a_blank_godown_or_a_sub_line_without_its_figures_refuses() {
    let blank = damaged(
        EXPLODED_FY,
        "<SSGODOWN>Godown Lab C</SSGODOWN>",
        "<SSGODOWN> </SSGODOWN>",
        1,
    );
    assert_eq!(
        parse_native_stock_summary_exploded(&blank),
        Err(NativeStockError::Malformed(
            "stock_report_sub_line_godown_empty"
        ))
    );
    let missing = damaged(EXPLODED_FY, "<SSGODOWN>Godown Lab C</SSGODOWN>", "", 1);
    assert_eq!(
        parse_native_stock_summary_exploded(&missing),
        Err(NativeStockError::Malformed("stock_report_sub_line_shape"))
    );
    let text = response(EXPLODED_FY);
    let cut = &text[..text.rfind("<DSPSTKINFO>").unwrap()];
    assert_eq!(
        parse_native_stock_summary_exploded(&format!("{cut}</ENVELOPE>")),
        Err(NativeStockError::Malformed(
            "stock_report_sub_line_without_info"
        ))
    );
}

#[test]
fn a_sub_line_before_any_line_refuses() {
    let text = response(EXPLODED_FY);
    let start = text.find("<SSBATCHNAME>").unwrap();
    let end = text[start..].find("</DSPSTKINFO>").unwrap() + start + "</DSPSTKINFO>".len();
    let moved = format!("<ENVELOPE>{}</ENVELOPE>", &text[start..end]);
    assert_eq!(
        parse_native_stock_summary_exploded(&moved),
        Err(NativeStockError::Malformed(
            "stock_report_unexpected_element"
        ))
    );
}

#[test]
fn an_empty_exploded_answer_is_not_an_answer_of_no_lines() {
    assert_eq!(
        parse_native_stock_summary_exploded("<ENVELOPE></ENVELOPE>"),
        Ok(None)
    );
}

#[test]
fn an_answer_that_is_the_plain_report_line_for_line_shows_the_flag_was_not_observed() {
    let plain = response(STOCK_FY);
    let exploded = parse_native_stock_summary_exploded(&plain)
        .unwrap()
        .unwrap();
    let report = parse_native_stock_summary_report(&plain).unwrap();
    assert_eq!(
        items_by_godown(&exploded, &report),
        Ok(NativeRootItemsByGodown::ExplodeNotObserved)
    );
}

#[test]
fn an_expanded_answer_with_no_godown_line_is_an_empty_list_not_an_unobserved_flag() {
    // Every line but the groups' and their items' taken out of the exploded capture would be
    // the plain report again; here the two root items' godown lines alone are removed.
    let text = response(EXPLODED_FY);
    let mut kept = String::new();
    let mut rest = text.as_str();
    while let Some(start) = rest.find(" <SSBATCHNAME>") {
        kept.push_str(&rest[..start]);
        let after = &rest[start..];
        let end = after.find("</DSPSTKINFO>").unwrap() + "</DSPSTKINFO>".len();
        rest = after[end..].trim_start_matches(['\r', '\n']);
    }
    kept.push_str(rest);
    let exploded = parse_native_stock_summary_exploded(&kept).unwrap().unwrap();
    assert_eq!(exploded.len(), 13);
    let report = parse_native_stock_summary_report(&response(STOCK_FY)).unwrap();
    assert_eq!(
        items_by_godown(&exploded, &report),
        Ok(NativeRootItemsByGodown::Lines(Vec::new()))
    );
}
