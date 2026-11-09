//! The state table, row by row, from committed captures only (`tests/fixtures/stock-query/`,
//! BRIDGE SHAPE LAB). A row that pairs an answer with its own sitting's closing is the case it
//! names; a row that pairs answers across dates or items holds the rule against real bytes, and
//! says so.
use super::*;
use crate::native_stock_query::StockQueryBatch;
use crate::native_stock_summary::parse_native_stock_items;
use bridge_tally_primitives::ExactDecimal;

const LAB_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";

fn answer(name: &str) -> String {
    let bytes = std::fs::read(format!(
        "{}/tests/fixtures/stock-query/{name}.answer.utf16le.xml",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("a committed capture");
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();
    String::from_utf16(&units).expect("BOM-less UTF-16LE")
}

fn query(name: &str) -> StockQueryAnswer {
    parse_stock_query(&answer(name)).expect(name)
}

fn closing(collection: &str, item: &str) -> NativeQuantityRead {
    parse_native_stock_items(&answer(collection), LAB_GUID)
        .expect(collection)
        .rows
        .into_iter()
        .find(|row| row.name == item)
        .unwrap_or_else(|| panic!("{item} in {collection}"))
        .closing
        .quantity
}

fn quantity(amount: &str, unit: &str) -> NativeStockQuantity {
    NativeStockQuantity {
        amount: ExactDecimal::parse(amount).unwrap(),
        unit: unit.to_string(),
    }
}

/// The 30 October sitting answered these byte for byte as the committed `r-` files, beside the
/// collection at the same dates (`c-a-01`, `c-a-03`): the six pairs at two dates all tie.
#[test]
fn each_answer_with_lines_ties_to_its_sittings_closing() {
    for (answer, collection, item, total) in [
        (
            "r-02-stock-query-caustic-soda-flakes",
            "c-a-03-items-to-20260331",
            "Caustic Soda Flakes",
            "400.000",
        ),
        (
            "r-03-stock-query-cleaning-kit-a",
            "c-a-03-items-to-20260331",
            "Cleaning Kit A",
            "12",
        ),
        (
            "r-09-stock-query-soda-ash-light",
            "c-a-03-items-to-20260331",
            "Soda Ash Light",
            "1300.000",
        ),
        (
            "r-4-stock-query-stockitemname",
            "c-a-03-items-to-20260331",
            "Hydrochloric Acid 33pc",
            "816.667",
        ),
        (
            "r-12-stock-query-hcl-to-20250401",
            "c-a-01-items-to-20250401",
            "Hydrochloric Acid 33pc",
            "150.000",
        ),
        (
            "r-14-stock-query-cleaning-kit-a-to-20250401",
            "c-a-01-items-to-20250401",
            "Cleaning Kit A",
            "25",
        ),
    ] {
        let NativeQuantityRead::Read(read) = closing(collection, item) else {
            panic!("{item} has a closing in {collection}");
        };
        let tie = tie_stock_query(&query(answer), &closing(collection, item)).unwrap();
        let StockQueryTie::Tied(sum) = &tie else {
            panic!("{answer}: {tie:?}");
        };
        // Written as Tally writes the closing, at its scale and in its unit.
        assert_eq!(sum.amount.as_str(), total, "{answer}");
        assert_eq!(*sum, read, "{answer}");
        assert_eq!(tie.code(), "tied");
    }
}

/// `q-r-13` is the empty answer, and the same sitting's collection gives that item `0.000 Kgs`
/// at that date: no stock.
#[test]
fn the_empty_answer_beside_an_explicit_zero_is_no_stock() {
    assert_eq!(
        query("q-r-13-stock-query-zero-stock-item-to-20250401"),
        StockQueryAnswer::NoLines
    );
    let zero = closing("c-a-01-items-to-20250401", "Zero Stock Item");
    assert_eq!(zero, NativeQuantityRead::Read(quantity("0.000", "Kgs")));
    let tie = tie_stock_query(
        &query("q-r-13-stock-query-zero-stock-item-to-20250401"),
        &zero,
    )
    .unwrap();
    assert_eq!(tie, StockQueryTie::NoStock);
    assert_eq!(tie.code(), "no_stock");
}

/// `q-r-04` is the empty answer, and the same sitting's collection gives that item no closing at
/// all: it never moved, which is not a statement that it holds none.
#[test]
fn the_empty_answer_beside_an_empty_closing_is_no_movement() {
    let empty = closing("c-a-03-items-to-20260331", "Cleaning Kit B");
    assert_eq!(empty, NativeQuantityRead::Empty);
    let tie = tie_stock_query(&query("q-r-04-stock-query-cleaning-kit-b"), &empty).unwrap();
    assert_eq!(tie, StockQueryTie::NoMovement);
    assert_eq!(tie.code(), "no_movement");
}

/// No capture shows these disagreements; each pairs real answers across dates or items, so the
/// rule is held against real bytes, not a case Tally was seen to send.
#[test]
fn a_disagreement_of_either_side_is_differs_and_adjusts_neither() {
    // A year-end answer against the opening day's closing: 400 against 200.
    assert_eq!(
        tie_stock_query(
            &query("r-02-stock-query-caustic-soda-flakes"),
            &closing("c-a-01-items-to-20250401", "Caustic Soda Flakes"),
        ),
        Ok(StockQueryTie::Differs {
            lines_total: Some(quantity("400.000", "Kgs")),
        })
    );
    // Lines against an empty closing: stock shown against none.
    assert_eq!(
        tie_stock_query(
            &query("r-02-stock-query-caustic-soda-flakes"),
            &closing("c-a-03-items-to-20260331", "Cleaning Kit B"),
        ),
        Ok(StockQueryTie::Differs {
            lines_total: Some(quantity("400.000", "Kgs")),
        })
    );
    // The empty answer against a closing that is not zero.
    assert_eq!(
        tie_stock_query(
            &query("q-r-04-stock-query-cleaning-kit-b"),
            &closing("c-a-03-items-to-20260331", "Caustic Soda Flakes"),
        ),
        Ok(StockQueryTie::Differs { lines_total: None })
    );
}

/// An item with an alternate unit: Stock Query writes the base unit (`alt-a-04`) and the same
/// sitting's collection writes the compound closing, which is not read. It never ties.
#[test]
fn a_compound_closing_is_not_comparable() {
    let compound = closing("alt-a-01-items-fy-s4", "Alt Unit Probe A");
    assert_eq!(compound, NativeQuantityRead::Unread);
    let tie = tie_stock_query(&query("alt-a-04-stock-query-alt-unit-probe-a"), &compound);
    assert_eq!(
        tie,
        Ok(StockQueryTie::NotComparable(NotComparable::ClosingUnread))
    );
    assert_eq!(
        tie_stock_query(&StockQueryAnswer::NoLines, &compound),
        Ok(StockQueryTie::NotComparable(NotComparable::ClosingUnread))
    );
}

/// Units are compared as sent and never converted: `Box` lines against a `Kgs` closing (two
/// items, paired only to hold the rule), and one answer whose lines are in two units.
#[test]
fn another_unit_is_not_comparable() {
    assert_eq!(
        tie_stock_query(
            &query("r-01-stock-query-carton-box-small"),
            &closing("c-a-03-items-to-20260331", "Caustic Soda Flakes"),
        ),
        Ok(StockQueryTie::NotComparable(NotComparable::UnitsDiffer))
    );
    let StockQueryAnswer::Lines(mut lines) = query("r-02-stock-query-caustic-soda-flakes") else {
        panic!("r-02 has lines");
    };
    lines[1].quantity.unit = "kgs".to_string();
    assert_eq!(
        tie_stock_query(
            &StockQueryAnswer::Lines(lines),
            &closing("c-a-03-items-to-20260331", "Caustic Soda Flakes"),
        ),
        Ok(StockQueryTie::NotComparable(NotComparable::UnitsDiffer))
    );
}

/// A sum longer than `ExactDecimal` holds refuses rather than report a wrong total.
#[test]
fn an_unrepresentable_sum_refuses() {
    let StockQueryAnswer::Lines(mut lines) = query("r-02-stock-query-caustic-soda-flakes") else {
        panic!("r-02 has lines");
    };
    let longest = "9".repeat(bridge_tally_primitives::MAX_EXACT_DECIMAL_BYTES);
    lines[0].quantity.amount = ExactDecimal::parse(longest.clone()).unwrap();
    lines[1].quantity.amount = ExactDecimal::parse(longest).unwrap();
    let error = tie_stock_query(
        &StockQueryAnswer::Lines(lines.clone()),
        &NativeQuantityRead::Empty,
    )
    .unwrap_err();
    assert_eq!(error.code(), "stock_query_sum_unrepresentable");
    assert_eq!(godown_subtotals(&lines), Err(StockSumUnrepresentable));
}

/// Godowns in the order they first appear, each line once, summed per godown; a batch line and a
/// default line of one godown fall together.
#[test]
fn subtotals_group_lines_by_godown_as_sent() {
    let StockQueryAnswer::Lines(lines) = query("r-4-stock-query-stockitemname") else {
        panic!("r-4 has lines");
    };
    assert_eq!(lines[0].batch, StockQueryBatch::Default);
    assert_eq!(
        godown_subtotals(&lines),
        Ok(vec![
            GodownSubtotal {
                lines: vec![0, 1],
                quantity: Some(quantity("450.000", "Kgs")),
            },
            GodownSubtotal {
                lines: vec![2],
                quantity: Some(quantity("366.667", "Kgs")),
            },
        ])
    );
    // A negative line is summed with its sign.
    let StockQueryAnswer::Lines(lines) = query("r-09-stock-query-soda-ash-light") else {
        panic!("r-09 has lines");
    };
    let subtotals = godown_subtotals(&lines).unwrap();
    assert_eq!(subtotals[1].quantity, Some(quantity("-50.000", "Kgs")));
}

/// Two godowns whose names differ only in case are two godowns, as the parser's repeat check
/// holds them.
#[test]
fn godowns_differing_only_in_case_are_not_merged() {
    let StockQueryAnswer::Lines(mut lines) = query("r-4-stock-query-stockitemname") else {
        panic!("r-4 has lines");
    };
    lines[1].godown = lines[0].godown.to_ascii_uppercase();
    let subtotals = godown_subtotals(&lines).unwrap();
    assert_eq!(subtotals.len(), 3);
    assert_eq!(subtotals[0].lines, vec![0]);
    assert_eq!(subtotals[1].lines, vec![1]);
}

/// Tally's shaped failure envelope, as it answered a built-in report it could not find, reads as
/// Tally's failure, not as a malformed answer; the parser alone calls it malformed. An answer
/// that is malformed without saying Tally failed stays malformed.
#[test]
fn tallys_failure_envelope_reads_as_tallys_failure() {
    let bytes = std::fs::read(format!(
        "{}/tests/fixtures/builtin_unknown_report_refusal_live.utf16le.xml",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();
    let failure = String::from_utf16(&units).unwrap();
    assert_eq!(parse_stock_query(&failure), Err(StockQueryError::Malformed));
    assert_eq!(
        read_stock_query(&failure),
        Err(StockQueryError::TallyReportedFailure)
    );
    let cut = answer("r-02-stock-query-caustic-soda-flakes");
    let cut = &cut[..cut.len() / 2];
    assert_eq!(read_stock_query(cut), parse_stock_query(cut));
    assert!(read_stock_query(cut).is_err());
    assert_eq!(
        read_stock_query(&answer("r-02-stock-query-caustic-soda-flakes")),
        parse_stock_query(&answer("r-02-stock-query-caustic-soda-flakes"))
    );
}
