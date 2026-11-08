//! Stock-by-batch reads replayed from captures, one fault at a time. The sequence is the stock
//! summary's up to its items: mode probe and identity (3), opening extent (4), company flags (4),
//! stock items (4); then one Stock Query per item sent, read once (1 each); then the closing
//! extent (4) and identity and mode again (3). The items are the 30 October sitting's collection
//! at the year end (`c-a-03`), and the Stock Query answers are the committed year-end answers
//! that sitting repeated byte for byte (`stock-query/PROVENANCE.md`).
use super::stock_by_batch::{StockByBatchReadError, STOCK_QUERY_ANSWER_MAX_BYTES};
use super::stock_summary::{stock_summary_period, StockSummaryReadError};
use super::trial_balance_tests::{
    companies as captured_companies, config, decode, education, extents as captured_extents, pair,
    status, xml, GUID,
};
use super::*;
use bridge_tally_protocol::native_stock_by_batch::{GodownSubtotal, StockQueryTie};
use bridge_tally_protocol::native_stock_query::{render_stock_query_request, MeasuredItemName};
use bridge_tally_protocol::native_stock_summary::StockSummaryAsOf;
use tally_protocol_simulator::{Delivery, ObservedRequest, ScenarioPlan, SequenceSimulator};

/// The synthetic book the captures came from; its GUID is replaced by the test double's company.
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const AS_OF: &str = "20260331";
const TODAY: &str = "20260930";
/// The captured extent starts the books a year after the captures' period; the test company's
/// row in the companies list and its extent are moved back together, as the identity bracket
/// compares them (as `runtime_stock_summary_tests` does).
const BOOKS_FROM: &str = "20250401";

/// `text` with the test company's `BOOKSFROM` moved to [`BOOKS_FROM`], in its own row only.
fn with_books_from(text: String) -> String {
    let from = "<BOOKSFROM TYPE=\"Date\">20260401</BOOKSFROM>";
    let at = text.find(GUID).expect("the test company is listed");
    let start = text[..at].rfind("<COMPANY ").unwrap();
    let end = at + text[at..].find("</COMPANY>").unwrap();
    let row = &text[start..end];
    assert_eq!(row.matches(from).count(), 1);
    format!(
        "{}{}{}",
        &text[..start],
        row.replace(
            from,
            &format!("<BOOKSFROM TYPE=\"Date\">{BOOKS_FROM}</BOOKSFROM>")
        ),
        &text[end..]
    )
}

fn companies() -> String {
    with_books_from(captured_companies())
}

fn extents() -> String {
    with_books_from(captured_extents())
}

/// The test company as its identity bracket verifies it.
fn identity() -> VerifiedCompanyIdentity {
    let companies = parse_companies_from_collection(&companies()).unwrap();
    let row = companies
        .iter()
        .find(|row| row.guid.as_deref() == Some(GUID))
        .unwrap();
    VerifiedCompanyIdentity::from_observed_companies(
        row.name.clone(),
        crate::tally::test_company_guid(GUID),
        row.company_number.clone().unwrap(),
        row.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}

fn flags() -> String {
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_shape_lab_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
}

/// The mode probe, the identity bracket and the opening extent.
fn opening(extent: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    pair(&mut plans, xml(extent));
    plans
}

fn cause<T: std::error::Error + 'static>(error: &anyhow::Error) -> Option<&T> {
    error.chain().find_map(|cause| cause.downcast_ref::<T>())
}

fn body_hash(request: &str) -> String {
    sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        request,
    ))
}

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
    through_items_at(extents(), items)
}

fn through_items_at(extent: String, items: String) -> Vec<ScenarioPlan> {
    let mut plans = opening(extent);
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

/// Every refusal the shared admission and period can give is this read's own code: a person is
/// never shown stock_summary's name for a stock_by_batch refusal.
#[test]
fn no_shared_refusal_is_shown_under_stock_summarys_name() {
    for shared in [
        StockSummaryReadError::EducationUnqualified,
        StockSummaryReadError::NotEnabled,
        StockSummaryReadError::AsOfBeforeBooks,
        StockSummaryReadError::AsOfInFuture,
        StockSummaryReadError::PeriodNotHonoured,
        StockSummaryReadError::TooLarge {
            master_alter_id: 1,
            estimated_bytes: 1,
            limit_bytes: 1,
            limit_master_alter_id: 1,
        },
        StockSummaryReadError::TooManyItems {
            master_alter_id: 1,
            counted_items: 1,
            estimated_bytes: 1,
            limit_bytes: 1,
            limit_items: 1,
        },
        StockSummaryReadError::ReportTooLarge {
            counted_lines: 1,
            estimated_bytes: 1,
            limit_bytes: 1,
        },
        StockSummaryReadError::ItemCountCompanyHigher {
            company: 2,
            census: 1,
        },
        StockSummaryReadError::CensusRefused("stock_census_slice_invalid"),
        StockSummaryReadError::PremiseViolated("stock_rows_exceed_master_mark"),
    ] {
        let own = StockByBatchReadError(shared);
        let code = own.safe_code();
        assert!(!code.contains("stock_summary"), "{code}");
        assert!(
            code.starts_with("stock_by_batch_") || code == "stock_not_enabled",
            "{code}"
        );
        assert_eq!(own.to_string(), code);
        // Not reachable through the chain: nothing finds the shared code under this one.
        assert!(std::error::Error::source(&own).is_none());
    }
}

/// Raised in the read, each reaches the caller as this read's own: the shared one is nowhere in
/// the chain. Education mode refuses before the company; rows past the master mark refuse after
/// the closing extent, with no Stock Query sent.
#[tokio::test]
async fn a_shared_refusal_reaches_the_caller_under_this_reads_code() {
    let (result, observed) = run(
        vec![status(), xml(education(&companies()))],
        &[CAUSTIC_SODA],
    )
    .await;
    let error = result.err().expect("education mode refuses");
    assert_eq!(
        cause::<StockByBatchReadError>(&error).map(StockByBatchReadError::safe_code),
        Some("stock_by_batch_education_unqualified")
    );
    assert!(cause::<StockSummaryReadError>(&error).is_none());
    assert_eq!(observed.len(), 2);

    // The capture's book holds 11 items; a mark of 5 admits the read, and the rows break it.
    let at_mark = |mark: u64| {
        extents().replace(
            "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>",
            &format!("<ALTMSTID TYPE=\"Number\"> {mark}</ALTMSTID>"),
        )
    };
    let mut plans = through_items_at(at_mark(5), items());
    pair(&mut plans, xml(at_mark(5)));
    let (result, observed) = run(plans, &[CAUSTIC_SODA]).await;
    let error = result.err().expect("rows past the mark refuse");
    let own = cause::<StockByBatchReadError>(&error).expect("this read's own refusal");
    assert_eq!(own.safe_code(), "stock_by_batch_bound_premise_violated");
    assert!(matches!(
        own.0,
        StockSummaryReadError::PremiseViolated("stock_rows_exceed_master_mark")
    ));
    assert!(cause::<StockSummaryReadError>(&error).is_none());
    assert_eq!(observed.len(), AT_ITEMS + 4);
}
