//! Stock-by-batch reads replayed from captures, one fault at a time. The sequence is the stock
//! summary's up to its items: mode probe and identity (3), opening extent (4), company flags (4),
//! stock items (4); then one Stock Query per item sent, read once (1 each); then the closing
//! extent (4) and identity and mode again (3). The items are the 30 October sitting's collection
//! at the year end (`c-a-03`), and the Stock Query answers are the committed year-end answers
//! that sitting repeated byte for byte (`stock-query/PROVENANCE.md`).
use super::stock_by_batch::STOCK_QUERY_ANSWER_MAX_BYTES;
use super::stock_summary::stock_summary_period;
use super::stock_summary_tests::{
    body_hash, cause, companies, extents, flags, identity, opening, AS_OF, BOOKS_FROM,
    CAPTURE_GUID, TODAY,
};
use super::trial_balance_tests::{config, decode, pair, status, xml, GUID};
use super::*;
use bridge_tally_protocol::native_stock_by_batch::{GodownSubtotal, StockQueryTie};
use bridge_tally_protocol::native_stock_query::{render_stock_query_request, MeasuredItemName};
use bridge_tally_protocol::native_stock_summary::StockSummaryAsOf;
use tally_protocol_simulator::{Delivery, ObservedRequest, ScenarioPlan, SequenceSimulator};

const SODA_ASH: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346-0000010e";
const CAUSTIC_SODA: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346-0000010c";
const CLEANING_KIT_A: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346-00000112";
const LABEL_ROLL: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346-00000111";

/// Through the opening extent, the flags pair and the items pair.
const AT_ITEMS: usize = 15;
/// The closing extent and the brackets after it.
const AFTER_QUERIES: usize = 7;

fn fixture(name: &str) -> String {
    decode(
        &std::fs::read(format!(
            "{}/crates/bridge-tally-protocol/tests/fixtures/stock-query/{name}",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("a committed capture"),
    )
}

/// The year-end collection, bound to the test company.
fn items() -> String {
    fixture("c-a-03-items-to-20260331.answer.utf16le.xml").replace(CAPTURE_GUID, GUID)
}

fn answer(name: &str) -> ScenarioPlan {
    xml(fixture(&format!("{name}.answer.utf16le.xml")))
}

/// The test company's GUID in place of the capture's, as the items are bound to it.
fn bound(guid: &str) -> String {
    guid.replace(CAPTURE_GUID, GUID)
}

fn through_items(items: String) -> Vec<ScenarioPlan> {
    let mut plans = opening(extents());
    pair(&mut plans, xml(flags()));
    pair(&mut plans, xml(items));
    plans
}

fn after_queries(plans: &mut Vec<ScenarioPlan>, closing_extent: String) {
    let companies = xml(companies());
    pair(plans, xml(closing_extent));
    plans.extend([companies.clone(), status(), companies]);
}

type Outcome = (anyhow::Result<StockByBatchRead>, Vec<ObservedRequest>);

async fn run(plans: Vec<ScenarioPlan>, guids: &[&str]) -> Outcome {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let result = TallyRuntime::default()
        .fetch_stock_by_batch(
            config(&simulator),
            &identity(),
            StockSummaryAsOf::new(TallyDate::parse(AS_OF).unwrap()).unwrap(),
            guids.iter().map(|guid| bound(guid)).collect(),
            TallyDate::parse(TODAY).unwrap(),
        )
        .await;
    (result, simulator.finish().unwrap())
}

/// The Stock Query the read sends for `item` over the test company's year.
fn query_request(item: &str) -> String {
    let period = stock_summary_period(
        DateBoundaryProfile::ModeAgnostic,
        &StockSummaryAsOf::new(TallyDate::parse(AS_OF).unwrap()).unwrap(),
        &TallyDate::parse(BOOKS_FROM).unwrap(),
        &TallyDate::parse(TODAY).unwrap(),
    )
    .unwrap();
    render_stock_query_request(
        identity().display_name(),
        &MeasuredItemName::new(item).unwrap(),
        &period,
    )
}

fn tie(item: &StockByBatchItem) -> &StockQueryTie {
    match item {
        StockByBatchItem::Read { tie, .. } => tie,
        other => panic!("not read: {other:?}"),
    }
}

#[tokio::test]
async fn one_bracket_sends_one_stock_query_per_item_in_the_order_asked() {
    let mut plans = through_items(items());
    plans.extend([
        answer("r-09-stock-query-soda-ash-light"),
        answer("r-02-stock-query-caustic-soda-flakes"),
        answer("r-03-stock-query-cleaning-kit-a"),
    ]);
    after_queries(&mut plans, extents());
    let (result, observed) = run(plans, &[SODA_ASH, CAUSTIC_SODA, CLEANING_KIT_A]).await;

    let read = result.unwrap();
    assert_eq!(observed.len(), AT_ITEMS + 3 + AFTER_QUERIES);
    assert_eq!(read.stopped, None);
    assert_eq!(read.from.as_str(), "20250401");
    assert_eq!(read.to.as_str(), AS_OF);
    // Not alphabetical: the order asked, each item once, one send each.
    for (at, item) in ["Soda Ash Light", "Caustic Soda Flakes", "Cleaning Kit A"]
        .into_iter()
        .enumerate()
    {
        assert_eq!(
            observed[AT_ITEMS + at].request_body_sha256,
            body_hash(&query_request(item)),
            "{item}"
        );
        assert_eq!(tie(&read.items[at]).code(), "tied", "{item}");
    }
    let StockByBatchItem::Read {
        item,
        lines,
        subtotals,
        ..
    } = &read.items[0]
    else {
        panic!("read");
    };
    assert_eq!(item.name, "Soda Ash Light");
    assert_eq!(lines.len(), 2);
    assert_eq!(
        subtotals
            .iter()
            .map(|s| s.lines.clone())
            .collect::<Vec<_>>(),
        vec![vec![0], vec![1]]
    );
    assert!(matches!(
        &subtotals[..],
        [
            GodownSubtotal {
                quantity: Some(_),
                ..
            },
            GodownSubtotal {
                quantity: Some(_),
                ..
            }
        ]
    ));
}

#[tokio::test]
async fn a_book_that_moves_during_the_queries_reports_the_move_and_no_item() {
    let moved = extents().replace(
        "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>",
        "<ALTMSTID TYPE=\"Number\"> 225</ALTMSTID>",
    );
    assert_ne!(moved, extents());
    let mut plans = through_items(items());
    plans.push(answer("r-02-stock-query-caustic-soda-flakes"));
    pair(&mut plans, xml(moved));
    let (result, observed) = run(plans, &[CAUSTIC_SODA]).await;

    let error = result.err().expect("a moved book refuses");
    assert!(matches!(
        cause::<PairedReadValidationError>(&error),
        Some(PairedReadValidationError::StockByBatchExtent)
    ));
    assert_eq!(observed.len(), AT_ITEMS + 1 + 4);
}

#[tokio::test]
async fn a_failed_send_stops_the_reads_and_the_rest_are_not_sent() {
    let mut plans = through_items(items());
    plans.extend([
        answer("r-09-stock-query-soda-ash-light"),
        answer("r-02-stock-query-caustic-soda-flakes").with_delivery(Delivery::ResetBeforeBody),
    ]);
    let (result, observed) = run(plans, &[SODA_ASH, CAUSTIC_SODA, CLEANING_KIT_A]).await;

    let read = result.unwrap();
    // The third item is never dispatched, and neither is the closing extent.
    assert_eq!(observed.len(), AT_ITEMS + 2);
    let code = read.stopped.expect("the reads stopped");
    assert!(matches!(read.items[0], StockByBatchItem::Read { .. }));
    assert!(
        matches!(&read.items[1], StockByBatchItem::NotRead { item, code: at } if item.name == "Caustic Soda Flakes" && *at == code)
    );
    assert!(
        matches!(&read.items[2], StockByBatchItem::NotSent { item } if item.name == "Cleaning Kit A")
    );
    assert_ne!(code, "stock_query_send_failed", "the transport named it");
}

#[tokio::test]
async fn an_unknown_guid_and_an_unmeasured_name_send_nothing() {
    // `Label Roll` renamed with a character no capture showed Stock Query selecting by.
    let items = items().replace("Label Roll", "Label_Roll");
    let mut plans = through_items(items);
    plans.push(answer("r-03-stock-query-cleaning-kit-a"));
    after_queries(&mut plans, extents());
    let unknown = "3a6bd6e1-b835-4bff-89dd-8a6af138c346-00000fff";
    let (result, observed) = run(plans, &[unknown, LABEL_ROLL, CLEANING_KIT_A]).await;

    let read = result.unwrap();
    assert_eq!(observed.len(), AT_ITEMS + 1 + AFTER_QUERIES);
    assert_eq!(
        observed[AT_ITEMS].request_body_sha256,
        body_hash(&query_request("Cleaning Kit A"))
    );
    assert!(
        matches!(&read.items[0], StockByBatchItem::ItemNotFound { guid } if *guid == bound(unknown))
    );
    assert!(
        matches!(&read.items[1], StockByBatchItem::NameUnmeasured { item } if item.name == "Label_Roll")
    );
    assert_eq!(tie(&read.items[2]).code(), "tied");
}

#[tokio::test]
async fn an_answer_refused_or_past_the_limit_refuses_that_item_and_the_reads_go_on() {
    let refusal = decode(
        &std::fs::read(format!(
            "{}/crates/bridge-tally-protocol/tests/fixtures/builtin_unknown_report_refusal_live.utf16le.xml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap(),
    );
    let mut plans = through_items(items());
    plans.extend([
        xml("x".repeat(STOCK_QUERY_ANSWER_MAX_BYTES)),
        xml("<ENVELOPE><STQGODOWNNAME>Main Location</STQGODOWNNAME></ENVELOPE>".to_string()),
        xml(refusal),
        answer("r-03-stock-query-cleaning-kit-a"),
    ]);
    after_queries(&mut plans, extents());
    let (result, observed) =
        run(plans, &[SODA_ASH, CAUSTIC_SODA, LABEL_ROLL, CLEANING_KIT_A]).await;

    let read = result.unwrap();
    assert_eq!(observed.len(), AT_ITEMS + 4 + AFTER_QUERIES);
    assert_eq!(read.stopped, None);
    let codes = read.items[..3]
        .iter()
        .map(|item| match item {
            StockByBatchItem::Refused { code, .. } => *code,
            other => panic!("refused: {other:?}"),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        codes,
        [
            "stock_query_answer_too_large",
            "stock_query_partial_line",
            "stock_query_tally_reported_failure",
        ]
    );
    assert_eq!(tie(&read.items[3]).code(), "tied");
}
