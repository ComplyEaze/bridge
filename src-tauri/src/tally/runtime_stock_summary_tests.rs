//! Stock summary reads replayed from captures, with one admission, parse or
//! stability fault at a time. The sequence is the masters read's with three
//! paired reads between its extents: mode probe and identity (3), opening
//! extent (4), company flags (4), stock items (4), Stock Summary (4), closing
//! extent (4), identity and mode again (3).
use super::stock_summary::{
    admit_stock_summary_size, check_stock_premise, stock_summary_period, StockSummaryRead,
};
use super::trial_balance_tests::{
    companies as captured_companies, config, decode, education, extents as captured_extents, pair,
    status, xml, GUID,
};
use super::*;
use bridge_tally_protocol::native_masters::MASTERS_RESPONSE_BUDGET_BYTES;
use bridge_tally_protocol::native_stock_summary::{
    parse_native_stock_summary_report, render_company_inventory_flags_request,
    render_native_stock_summary_request, stock_item_worst_row_bytes, NativeFlag,
    NativeItemCountCrossCheck, NativeItemCountStatus, NativeStockError, NativeStockGate,
    NativeStockReport, StockSummaryAsOf,
};
use bridge_tally_protocol::xml_read_profiles::{ValidatedCompanyName, ValidatedDateRange};
use tally_protocol_simulator::{ObservedRequest, ScenarioPlan, SequenceSimulator};

/// The synthetic book the stock captures came from. Its GUID is replaced by the
/// test double's company, so a captured collection binds to it.
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
/// The only date stock has been measured at: the end of the financial year the
/// captures were taken over, 20250401 to 20260331.
const AS_OF: &str = "20260331";
const TODAY: &str = "20260930";
/// The test company's books start here. The captured extent says 20260401, which
/// would put the captures' period before the books; its row in the companies
/// list and its extent are moved a year back together, as the identity bracket
/// compares them.
const BOOKS_FROM: &str = "20250401";

/// `text` with the test company's `BOOKSFROM` moved to [`BOOKS_FROM`], inside
/// that company's own row only.
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
        GUID.into(),
        row.company_number.clone().unwrap(),
        row.books_from.clone().unwrap(),
        &companies,
    )
    .unwrap()
}

fn items() -> String {
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/stock_items_shape_lab_fy_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
}

fn report() -> String {
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/stock_summary_report_shape_lab_fy_live.utf16le.xml"
    ))
}

fn flags() -> String {
    decode(include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_shape_lab_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
}

/// `text` with the first `from` replaced by `to`, which must be there.
fn replaced(text: &str, from: &str, to: &str) -> String {
    assert!(text.contains(from), "the capture has no {from}");
    text.replacen(from, to, 1)
}

/// The report with its `ENVELOPE` emptied, as Tally answered on a book not known
/// to hold inventory (§12a.11).
fn hollow_report() -> String {
    let text = report();
    let start = text.find("<ENVELOPE>").unwrap() + "<ENVELOPE>".len();
    format!(
        "{}{}",
        &text[..start],
        &text[text.rfind("</ENVELOPE>").unwrap()..]
    )
}

/// The books' extent with its master mark set to `mark`.
fn extent_with_mark(mark: u64) -> String {
    let captured = extents();
    let mark_element = "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>";
    assert!(captured.contains(mark_element));
    captured.replace(
        mark_element,
        &format!("<ALTMSTID TYPE=\"Number\"> {mark}</ALTMSTID>"),
    )
}

/// The mode probe, the identity bracket and the opening extent.
fn opening(extent: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = vec![status(), companies.clone(), companies];
    pair(&mut plans, xml(extent));
    plans
}

/// What each read is scripted to answer; the default is the captures.
struct Book {
    opening_extent: String,
    flags: String,
    items: String,
    report: String,
    closing_extent: String,
}

impl Book {
    fn captured() -> Self {
        Self {
            opening_extent: extents(),
            flags: flags(),
            items: items(),
            report: report(),
            closing_extent: extents(),
        }
    }

    fn at_mark(mark: u64) -> Self {
        Self {
            opening_extent: extent_with_mark(mark),
            closing_extent: extent_with_mark(mark),
            ..Self::captured()
        }
    }

    /// The opening, then the flags pair.
    fn through_flags(&self) -> Vec<ScenarioPlan> {
        let mut plans = opening(self.opening_extent.clone());
        pair(&mut plans, xml(self.flags.clone()));
        plans
    }

    /// Through the stock items pair.
    fn through_items(&self) -> Vec<ScenarioPlan> {
        let mut plans = self.through_flags();
        pair(&mut plans, xml(self.items.clone()));
        plans
    }

    /// Through the report pair.
    fn through_report(&self) -> Vec<ScenarioPlan> {
        let mut plans = self.through_items();
        pair(&mut plans, xml(self.report.clone()));
        plans
    }

    fn through_closing_extent(&self) -> Vec<ScenarioPlan> {
        let mut plans = self.through_report();
        pair(&mut plans, xml(self.closing_extent.clone()));
        plans
    }

    /// A whole read: also the identity and mode brackets after the closing extent.
    fn complete(&self) -> Vec<ScenarioPlan> {
        let companies = xml(companies());
        let mut plans = self.through_closing_extent();
        plans.extend([companies.clone(), status(), companies]);
        plans
    }
}

/// A whole read's requests: 3 + 4 + 4 + 4 + 4 + 4 + 3.
const WHOLE_READ: usize = 26;
/// Through the opening extent, the flags pair, the items pair, the report pair,
/// and the closing extent.
const AT_OPENING_EXTENT: usize = 7;
const AT_FLAGS: usize = 11;
const AT_ITEMS: usize = 15;
const AT_REPORT: usize = 19;
const AT_CLOSING_EXTENT: usize = 23;

type Outcome = (
    anyhow::Result<(StockSummaryRead, CompanyBookExtent)>,
    Vec<ObservedRequest>,
);

async fn run_at(plans: Vec<ScenarioPlan>, as_of: &str, today: &str) -> Outcome {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let as_of = StockSummaryAsOf::new(TallyDate::parse(as_of).unwrap()).unwrap();
    let result = TallyRuntime::default()
        .fetch_stock_summary_with_extent(
            config(&simulator),
            &identity(),
            as_of,
            TallyDate::parse(today).unwrap(),
        )
        .await;
    (result, simulator.finish().unwrap())
}

async fn run(plans: Vec<ScenarioPlan>) -> Outcome {
    run_at(plans, AS_OF, TODAY).await
}

fn cause<T: std::error::Error + 'static>(error: &anyhow::Error) -> Option<&T> {
    error.chain().find_map(|cause| cause.downcast_ref::<T>())
}

fn body_hash(request: &str) -> String {
    sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        request,
    ))
}

/// The three requests a read of the test company sends over 20250401 to 20260331.
fn sent_requests() -> [String; 3] {
    let name = identity().display_name().to_string();
    let period = stock_summary_period(
        DateBoundaryProfile::ModeAgnostic,
        &StockSummaryAsOf::new(TallyDate::parse(AS_OF).unwrap()).unwrap(),
        &TallyDate::parse(BOOKS_FROM).unwrap(),
        &TallyDate::parse(TODAY).unwrap(),
    )
    .unwrap();
    let company = ValidatedCompanyName::new(name.as_str()).unwrap();
    let range = ValidatedDateRange::new("20250401", "20260331").unwrap();
    [
        render_company_inventory_flags_request(&name, GUID).unwrap(),
        ReadOnlyProfile::AuditStockItemsV1 {
            company: &company,
            period: &range,
        }
        .render(),
        render_native_stock_summary_request(&name, &period),
    ]
}

/// Whether any request sent carried `request`'s body.
fn was_sent(observed: &[ObservedRequest], request: &str) -> bool {
    let hash = body_hash(request);
    observed
        .iter()
        .any(|observed| observed.request_body_sha256 == hash)
}

fn date(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

fn as_of(text: &str) -> StockSummaryAsOf {
    StockSummaryAsOf::new(date(text)).unwrap()
}

#[tokio::test]
async fn a_whole_read_ties_through_every_bracket_and_sends_the_builders_requests_in_order() {
    let (result, observed) = run(Book::captured().complete()).await;

    let (read, extent) = result.unwrap();
    assert_eq!(observed.len(), WHOLE_READ);
    assert_eq!(read.from, date("20250401"));
    assert_eq!(read.to, date(AS_OF));
    assert_eq!(read.inventory.integrated, NativeFlag::Yes);
    assert_eq!(read.inventory.inventory_on, NativeFlag::Yes);
    assert_eq!(read.inventory.batchwise, NativeFlag::Yes);
    let NativeStockGate::ValueTotalMatched {
        items,
        total,
        report_empty_amounts,
        item_count,
        ..
    } = read.gate
    else {
        panic!("the capture's report ties to its items");
    };
    assert_eq!(items.len(), 11);
    assert_eq!(total.as_str(), "3000.01");
    // The company's own item count, from the flags answer, is the rows read.
    assert_eq!(
        item_count,
        NativeItemCountCrossCheck {
            status: NativeItemCountStatus::Matched,
            rows: 11,
            tally_count: 11,
        }
    );
    assert_eq!(report_empty_amounts, 0);
    assert_eq!(
        extent.master_alter_id_high_water().map(|mark| mark.get()),
        Some(224)
    );
    assert!(!read.evidence.request_sha256.is_empty());
    // The flags, the items and the report, each sent twice, between the opening
    // and closing extents: the builders' requests for this company and period.
    let [flags, items, report] = sent_requests();
    for (first, request) in [(7, &flags), (11, &items), (15, &report)] {
        let hash = body_hash(request);
        assert_eq!(observed[first].request_body_sha256, hash, "{first}");
        assert_eq!(observed[first + 2].request_body_sha256, hash, "{first}");
        assert_eq!(observed[first + 1].method, "GET", "{first}");
    }
}

#[test]
fn the_period_is_the_financial_year_containing_as_of_or_the_books_start_if_later() {
    let period = |profile, as_of_text, books_from, today| {
        stock_summary_period(profile, &as_of(as_of_text), &date(books_from), &date(today))
    };
    let admitted = |as_of_text, books_from, today| {
        let period = period(
            DateBoundaryProfile::ModeAgnostic,
            as_of_text,
            books_from,
            today,
        )
        .unwrap_or_else(|error| panic!("{as_of_text}: {error}"));
        (
            period.from().as_str().to_string(),
            period.to().as_str().to_string(),
        )
    };
    let pair = |from: &str, to: &str| (from.to_string(), to.to_string());
    // A 31 March is the end of the financial year that began the April before.
    assert_eq!(
        admitted("20260331", "20240401", "20260930"),
        pair("20250401", "20260331")
    );
    assert_eq!(
        admitted("20250331", "20240401", "20260930"),
        pair("20240401", "20250331")
    );
    // The books' start, if it is later than the year's start.
    assert_eq!(
        admitted("20260331", "20250515", "20260930"),
        pair("20250515", "20260331")
    );
    // As of the host's today is admitted.
    assert_eq!(
        admitted("20260331", "20240401", "20260331"),
        pair("20250401", "20260331")
    );
    // Before the books, and after today, refuse.
    assert!(matches!(
        period(
            DateBoundaryProfile::ModeAgnostic,
            "20240331",
            "20240401",
            "20260930"
        ),
        Err(StockSummaryReadError::AsOfBeforeBooks)
    ));
    assert!(matches!(
        period(
            DateBoundaryProfile::ModeAgnostic,
            "20270331",
            "20240401",
            "20260930"
        ),
        Err(StockSummaryReadError::AsOfInFuture)
    ));
    // The period is the Trial Balance's type, with its endpoint admission: a
    // start that is not a boundary the profile accepts is not honoured.
    let refused = period(
        DateBoundaryProfile::EducationRestricted,
        "20260331",
        "20250515",
        "20260930",
    );
    assert!(matches!(
        refused,
        Err(StockSummaryReadError::PeriodNotHonoured)
    ));
    assert_eq!(
        refused.err().map(|error| error.safe_code()),
        Some("stock_summary_period_not_honoured")
    );
}

#[test]
fn the_admission_edge_is_the_largest_mark_that_fits_and_the_next_refuses() {
    // 2 * (700 + 6 * 128 * (7 + 4)) is 18,296 bytes a row: 874 rows are
    // 15,990,704 bytes and 875 are 16,009,000.
    let row_bytes = 18_296_u64;
    assert_eq!(
        u64::try_from(stock_item_worst_row_bytes()).unwrap(),
        row_bytes
    );
    let largest = u64::try_from(MASTERS_RESPONSE_BUDGET_BYTES).unwrap() / row_bytes;
    assert_eq!(largest, 874);
    assert_eq!(admit_stock_summary_size(largest).ok(), Some(15_990_704));
    assert!(matches!(
        admit_stock_summary_size(largest + 1),
        Err(StockSummaryReadError::TooLarge {
            master_alter_id: 875,
            estimated_bytes: 16_009_000,
            limit_bytes: 16_000_000,
            limit_master_alter_id: 874,
        })
    ));
    // A mark so large that its product would overflow refuses; it never wraps.
    assert!(matches!(
        admit_stock_summary_size(u64::MAX),
        Err(StockSummaryReadError::TooLarge {
            estimated_bytes: u64::MAX,
            ..
        })
    ));
}

#[test]
fn the_premise_checks_refuse_a_response_that_outgrows_its_mark() {
    let violated = |result: Result<(), StockSummaryReadError>| match result {
        Err(error @ StockSummaryReadError::PremiseViolated(reason)) => {
            assert_eq!(error.safe_code(), "stock_summary_bound_premise_violated");
            reason
        }
        other => panic!("{other:?}"),
    };
    // As many rows as the mark and as many bytes as admitted fit.
    assert!(check_stock_premise(11, 11, 1_000, 1_000).is_ok());
    assert_eq!(
        violated(check_stock_premise(12, 11, 0, u64::MAX)),
        "stock_rows_exceed_master_mark"
    );
    assert_eq!(
        violated(check_stock_premise(11, 11, 1_001, 1_000)),
        "stock_response_over_admitted_bytes"
    );
}

#[tokio::test]
async fn a_book_at_the_edge_reads_and_one_past_it_is_refused_before_the_items_request() {
    let edge = 874;
    let (result, observed) = run(Book::at_mark(edge).complete()).await;
    assert!(matches!(
        result.unwrap().0.gate,
        NativeStockGate::ValueTotalMatched { .. }
    ));
    assert_eq!(observed.len(), WHOLE_READ);

    let mark = edge + 1;
    let (result, observed) = run(Book::at_mark(mark).through_flags()).await;
    let error = result.err().expect("refused");
    assert!(
        matches!(
            cause::<StockSummaryReadError>(&error),
            Some(StockSummaryReadError::TooLarge {
                master_alter_id,
                estimated_bytes: 16_009_000,
                limit_bytes: 16_000_000,
                limit_master_alter_id: 874,
            }) if *master_alter_id == mark
        ),
        "{error:?}"
    );
    assert_eq!(
        cause::<StockSummaryReadError>(&error).map(StockSummaryReadError::safe_code),
        Some("stock_summary_too_large")
    );
    // The mode probe, identity, the opening extent and the flags: no stock item
    // and no report request.
    assert_eq!(observed.len(), AT_FLAGS);
    let [_, items, report] = sent_requests();
    assert!(!was_sent(&observed, &items));
    assert!(!was_sent(&observed, &report));
}

#[tokio::test]
async fn education_is_refused_before_identity_or_extent_dispatch() {
    let (result, observed) = run(vec![status(), xml(education(&companies()))]).await;

    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<StockSummaryReadError>(&error),
        Some(StockSummaryReadError::EducationUnqualified)
    ));
    assert_eq!(observed.len(), 2);
}

#[tokio::test]
async fn inventory_off_is_refused_after_the_flags_and_unknown_is_read_on() {
    let off = Book {
        flags: replaced(
            &flags(),
            "<ISINVENTORYON TYPE=\"Logical\">Yes</ISINVENTORYON>",
            "<ISINVENTORYON TYPE=\"Logical\">No</ISINVENTORYON>",
        ),
        ..Book::captured()
    };
    let (result, observed) = run(off.through_flags()).await;
    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<StockSummaryReadError>(&error),
        Some(StockSummaryReadError::NotEnabled)
    ));
    assert_eq!(
        cause::<StockSummaryReadError>(&error).map(StockSummaryReadError::safe_code),
        Some("stock_not_enabled")
    );
    assert_eq!(observed.len(), AT_FLAGS);
    let [_, items, _] = sent_requests();
    assert!(!was_sent(&observed, &items));

    // Absent is unknown, reported, and does not refuse.
    let unknown = Book {
        flags: replaced(
            &flags(),
            "<ISINVENTORYON TYPE=\"Logical\">Yes</ISINVENTORYON>",
            "",
        ),
        ..Book::captured()
    };
    let (result, observed) = run(unknown.complete()).await;
    let read = result.unwrap().0;
    assert_eq!(read.inventory.inventory_on, NativeFlag::Unknown);
    assert_eq!(read.inventory.integrated, NativeFlag::Yes);
    assert_eq!(observed.len(), WHOLE_READ);
}

#[tokio::test]
async fn a_flags_answer_that_is_not_one_row_of_this_company_is_refused_before_any_item_read() {
    let row_start = flags().find("<COMPANY NAME=").unwrap();
    let row_end = row_start + flags()[row_start..].find("</COMPANY>").unwrap() + "</COMPANY>".len();
    let row = flags()[row_start..row_end].to_string();
    for (label, text) in [
        ("two rows", replaced(&flags(), &row, &format!("{row}{row}"))),
        ("another company's row", flags().replace(GUID, CAPTURE_GUID)),
        (
            "no rows",
            format!("{}{}", &flags()[..row_start], &flags()[row_end..]),
        ),
    ] {
        let book = Book {
            flags: text,
            ..Book::captured()
        };
        let (result, observed) = run(book.through_flags()).await;
        let error = result.err().expect("refused");
        assert_eq!(
            cause::<NativeStockError>(&error),
            Some(&NativeStockError::CompanyFlagsNotOneRow),
            "{label}"
        );
        assert_eq!(observed.len(), AT_FLAGS, "{label}");
    }
    let maybe = Book {
        flags: replaced(
            &flags(),
            "<ISBATCHWISEON TYPE=\"Logical\">Yes</ISBATCHWISEON>",
            "<ISBATCHWISEON TYPE=\"Logical\">Maybe</ISBATCHWISEON>",
        ),
        ..Book::captured()
    };
    let (result, observed) = run(maybe.through_flags()).await;
    let error = result.err().expect("refused");
    assert_eq!(
        cause::<NativeStockError>(&error),
        Some(&NativeStockError::FlagInvalid("is_batchwise_on"))
    );
    assert_eq!(observed.len(), AT_FLAGS);
}

#[tokio::test]
async fn an_as_of_before_the_books_or_after_today_is_refused_after_the_opening_extent() {
    let (result, observed) = run_at(opening(extents()), "20250331", TODAY).await;
    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<StockSummaryReadError>(&error),
        Some(StockSummaryReadError::AsOfBeforeBooks)
    ));
    assert_eq!(observed.len(), AT_OPENING_EXTENT);

    let (result, observed) = run_at(opening(extents()), AS_OF, "20260301").await;
    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<StockSummaryReadError>(&error),
        Some(StockSummaryReadError::AsOfInFuture)
    ));
    assert_eq!(observed.len(), AT_OPENING_EXTENT);
}

#[tokio::test]
async fn a_book_that_moved_during_the_read_is_refused_and_keeps_the_completed_source() {
    let book = Book {
        closing_extent: extent_with_mark(225),
        ..Book::captured()
    };
    let (result, observed) = run(book.through_closing_extent()).await;

    let error = result.err().expect("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(
        changed,
        PairedReadValidationError::StockSummaryExtent
    ));
    assert_eq!(changed.safe_code(), "stock_summary_extent_changed");
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    assert!(!evidence.request_sha256.is_empty());
    assert_eq!(observed.len(), AT_CLOSING_EXTENT);
}

#[tokio::test]
async fn a_read_that_changed_between_its_paired_reads_is_refused_at_that_read() {
    let drifted = |text: String| {
        [
            xml(text.clone()),
            status(),
            xml(format!("{text}\n")),
            status(),
        ]
    };
    let book = Book::captured();
    // The flags.
    let mut plans = opening(extents());
    plans.extend(drifted(flags()));
    let (result, observed) = run(plans).await;
    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<PairedReadValidationError>(&error),
        Some(PairedReadValidationError::StockSummaryCollection)
    ));
    assert_eq!(observed.len(), AT_FLAGS);
    // The stock items.
    let mut plans = book.through_flags();
    plans.extend(drifted(items()));
    let (result, observed) = run(plans).await;
    let error = result.err().expect("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(
        changed,
        PairedReadValidationError::StockSummaryCollection
    ));
    assert_eq!(changed.safe_code(), "stock_summary_collection_changed");
    assert_eq!(observed.len(), AT_ITEMS);
    // The Stock Summary.
    let mut plans = book.through_items();
    plans.extend(drifted(report()));
    let (result, observed) = run(plans).await;
    let error = result.err().expect("refused");
    let changed = cause::<PairedReadValidationError>(&error).expect("a stability refusal");
    assert!(matches!(
        changed,
        PairedReadValidationError::NativeStatement
    ));
    assert_eq!(changed.safe_code(), "native_statement_changed");
    assert_eq!(observed.len(), AT_REPORT);
}

#[tokio::test]
async fn a_transport_failure_stays_a_transport_failure_at_every_read() {
    let book = Book::captured();
    for (plans, sent) in [
        (
            {
                let mut plans = opening(extents());
                plans.push(xml(flags()).with_http_status(500));
                plans
            },
            AT_OPENING_EXTENT + 1,
        ),
        (
            {
                let mut plans = book.through_flags();
                plans.push(xml(items()).with_http_status(500));
                plans
            },
            AT_FLAGS + 1,
        ),
        (
            {
                let mut plans = book.through_items();
                plans.push(xml(report()).with_http_status(500));
                plans
            },
            AT_ITEMS + 1,
        ),
    ] {
        let (result, observed) = run(plans).await;
        let error = result.err().expect("refused");
        assert!(matches!(
            cause::<bridge_tally_transport::TallyTransportError>(&error),
            Some(bridge_tally_transport::TallyTransportError::HttpStatus { status: 500 })
        ));
        // Not read as an absent collection, an empty report, nor any refusal of
        // Bridge's own.
        assert!(cause::<NativeStockError>(&error).is_none());
        assert!(cause::<StockSummaryReadError>(&error).is_none());
        assert!(cause::<PairedReadValidationError>(&error).is_none());
        assert_eq!(observed.len(), sent);
    }
}

#[tokio::test]
async fn an_items_response_that_does_not_parse_is_refused_at_once_with_its_typed_cause() {
    let items_text = items();
    let start = items_text.find("<COLLECTION").unwrap();
    let end = items_text.find("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    let absent = format!("{}{}", &items_text[..start], &items_text[end..]);
    let bad_value = replaced(
        &items_text,
        "<CLOSINGVALUE TYPE=\"Amount\">2500.00</CLOSINGVALUE>",
        "<CLOSINGVALUE TYPE=\"Amount\">2,500.00</CLOSINGVALUE>",
    );
    let other_company = items_text.replace(GUID, CAPTURE_GUID);
    // No closing extent is read after these: the answer's shape is wrong
    // whether or not the book moved.
    for (label, text, expected) in [
        (
            "an absent collection",
            absent,
            NativeStockError::CollectionAbsent,
        ),
        (
            "a value that does not read",
            bad_value,
            NativeStockError::ValueUnparseable,
        ),
        (
            "rows that are another company's",
            other_company,
            NativeStockError::RowGuidForeign,
        ),
    ] {
        let book = Book {
            items: text,
            ..Book::captured()
        };
        let (result, observed) = run(book.through_items()).await;
        let error = result.err().expect("refused");
        assert_eq!(
            cause::<NativeStockError>(&error),
            Some(&expected),
            "{label}"
        );
        assert_eq!(observed.len(), AT_ITEMS, "{label}");
    }
}

#[tokio::test]
async fn a_report_that_does_not_parse_or_is_not_recognised_is_refused_after_the_closing_extent() {
    let unknown =
        report()
            .replacen("<ENVELOPE>", "<RESPONSE>", 1)
            .replacen("</ENVELOPE>", "</RESPONSE>", 1);
    for (label, text, expected) in [
        (
            "a report Tally answered without",
            unknown.clone(),
            NativeStockError::ReportUnknown,
        ),
        (
            "a failure element",
            replaced(
                &report(),
                "<DSPACCNAME>",
                "<LINEERROR>x</LINEERROR><DSPACCNAME>",
            ),
            NativeStockError::TallyReportedFailure,
        ),
        (
            "an amount that is not a decimal",
            replaced(
                &report(),
                "<DSPCLAMTA>18750.00</DSPCLAMTA>",
                "<DSPCLAMTA>18,750.00</DSPCLAMTA>",
            ),
            NativeStockError::ReportAmountInvalid,
        ),
    ] {
        let book = Book {
            report: text,
            ..Book::captured()
        };
        // Held until the closing extent was read, and it had not moved.
        let (result, observed) = run(book.through_closing_extent()).await;
        let error = result.err().expect("refused");
        assert_eq!(
            cause::<NativeStockError>(&error),
            Some(&expected),
            "{label}"
        );
        assert_eq!(observed.len(), AT_CLOSING_EXTENT, "{label}");
    }
    // A book that moved says so, not what its report would have refused.
    let book = Book {
        report: unknown,
        closing_extent: extent_with_mark(260),
        ..Book::captured()
    };
    let (result, observed) = run(book.through_closing_extent()).await;
    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<PairedReadValidationError>(&error),
        Some(PairedReadValidationError::StockSummaryExtent)
    ));
    assert!(cause::<NativeStockError>(&error).is_none());
    assert_eq!(observed.len(), AT_CLOSING_EXTENT);
}

#[tokio::test]
async fn a_row_longer_than_the_assumed_worst_row_is_the_parsers_typed_refusal() {
    // A filler element the parser does not read, in a row: only the row's own
    // length can refuse it.
    let filler = "x".repeat(stock_item_worst_row_bytes());
    // The header's counts carry a `STOCKITEM` element too, so the first two
    // closing tags are the header's and the first row's.
    let book = Book {
        items: items().replacen(
            "</STOCKITEM>",
            &format!("<FILLER>{filler}</FILLER></STOCKITEM>"),
            2,
        ),
        ..Book::captured()
    };
    let (result, observed) = run(book.through_items()).await;

    let error = result.err().expect("refused");
    assert_eq!(
        cause::<NativeStockError>(&error),
        Some(&NativeStockError::RowExceedsBound)
    );
    assert_eq!(observed.len(), AT_ITEMS);
}

#[tokio::test]
async fn a_response_that_breaks_the_marks_premise_is_refused_after_the_closing_extent() {
    // Eleven items, but a mark of two.
    let (result, observed) = run(Book::at_mark(2).through_closing_extent()).await;
    let error = result.err().expect("refused");
    let violated = cause::<StockSummaryReadError>(&error).expect("a premise refusal");
    assert!(matches!(
        violated,
        StockSummaryReadError::PremiseViolated("stock_rows_exceed_master_mark")
    ));
    assert_eq!(violated.safe_code(), "stock_summary_bound_premise_violated");
    // The completed reads are kept as evidence.
    let evidence = &error.downcast_ref::<RuntimeReadFailure>().unwrap().evidence;
    assert!(!evidence.response_sha256.is_empty());
    // Held until the closing extent was read, and it had not moved.
    assert_eq!(observed.len(), AT_CLOSING_EXTENT);

    // The mark fits every row, but the response is larger than was admitted: 224
    // rows of at most 18,296 bytes are 4,098,304 bytes, and this response
    // carries 4.4 MB (2.2 million UTF-16 characters) beside its rows.
    let padded = replaced(
        &items(),
        "<DATA>",
        &format!("<PAD>{}</PAD><DATA>", "x".repeat(2_200_000)),
    );
    let book = Book {
        items: padded,
        ..Book::captured()
    };
    let (result, observed) = run(book.through_closing_extent()).await;
    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<StockSummaryReadError>(&error),
        Some(StockSummaryReadError::PremiseViolated(
            "stock_response_over_admitted_bytes"
        ))
    ));
    assert_eq!(observed.len(), AT_CLOSING_EXTENT);
}

#[tokio::test]
async fn a_book_that_moved_during_the_read_is_reported_as_moved_not_as_what_its_rows_broke() {
    // The mark the read opened at (2) is below the rows it returned, and the
    // closing extent shows the mark moved to 260.
    let book = Book {
        closing_extent: extent_with_mark(260),
        ..Book::at_mark(2)
    };
    let (result, observed) = run(book.through_closing_extent()).await;

    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<PairedReadValidationError>(&error),
        Some(PairedReadValidationError::StockSummaryExtent)
    ));
    assert!(cause::<StockSummaryReadError>(&error).is_none());
    assert_eq!(observed.len(), AT_CLOSING_EXTENT);
}

#[tokio::test]
async fn a_report_that_differs_completes_the_read_and_withholds_the_items() {
    let book = Book {
        report: replaced(
            &report(),
            "<DSPCLAMTA>18750.00</DSPCLAMTA>",
            "<DSPCLAMTA>18750.01</DSPCLAMTA>",
        ),
        ..Book::captured()
    };
    let (result, observed) = run(book.complete()).await;
    let read = result.unwrap().0;
    let NativeStockGate::Differs {
        items_total,
        report_total,
    } = read.gate
    else {
        panic!("the report's total is not the items' sum");
    };
    assert_eq!(items_total.unwrap().as_str(), "3000.01");
    assert_eq!(report_total.as_str(), "3000.02");
    assert_eq!(observed.len(), WHOLE_READ);
}

#[tokio::test]
async fn valued_items_against_an_empty_report_complete_the_read_and_return_no_item() {
    let book = Book {
        report: hollow_report(),
        ..Book::captured()
    };
    let (result, observed) = run(book.complete()).await;
    let NativeStockGate::ReportShowsNoValue { items_total } = result.unwrap().0.gate else {
        panic!("the items carry a value the report does not show");
    };
    assert_eq!(items_total.as_str(), "3000.01");
    assert_eq!(observed.len(), WHOLE_READ);
    // The parser's own answer, for the record.
    assert_eq!(
        parse_native_stock_summary_report(&hollow_report()),
        Ok(NativeStockReport::Empty)
    );
}

/// The flags capture with its stock item count element as given: `Some(text)`
/// for its text, `None` to leave the element out.
fn flags_with_count(text: Option<&str>) -> String {
    replaced(
        &flags(),
        "<NUMSTOCKITEMS TYPE=\"Number\"> 11</NUMSTOCKITEMS>",
        &text.map_or(String::new(), |text| {
            format!("<NUMSTOCKITEMS TYPE=\"Number\">{text}</NUMSTOCKITEMS>")
        }),
    )
}

#[tokio::test]
async fn rows_that_differ_from_the_companys_own_item_count_complete_the_read_as_that_outcome() {
    // Twelve counted, eleven read, and a report that ties.
    let book = Book {
        flags: flags_with_count(Some(" 12")),
        ..Book::captured()
    };
    let (result, observed) = run(book.complete()).await;
    assert_eq!(
        result.unwrap().0.gate,
        NativeStockGate::ItemCountDiffers {
            rows: 11,
            tally_count: 12
        }
    );
    assert_eq!(observed.len(), WHOLE_READ);
    // Ten counted, eleven read: the same outcome the other way round.
    let book = Book {
        flags: flags_with_count(Some(" 10")),
        ..Book::captured()
    };
    let (result, observed) = run(book.complete()).await;
    assert_eq!(
        result.unwrap().0.gate,
        NativeStockGate::ItemCountDiffers {
            rows: 11,
            tally_count: 10
        }
    );
    assert_eq!(observed.len(), WHOLE_READ);
}

#[tokio::test]
async fn an_item_count_tally_did_not_give_is_refused_after_the_closing_extent() {
    for flags in [flags_with_count(None), flags_with_count(Some("eleven"))] {
        let book = Book {
            flags,
            ..Book::captured()
        };
        let (result, observed) = run(book.through_closing_extent()).await;
        let error = result.err().expect("refused");
        assert_eq!(
            cause::<NativeStockError>(&error),
            Some(&NativeStockError::ItemCountUnavailable)
        );
        // Held until the closing extent was read, and it had not moved.
        assert_eq!(observed.len(), AT_CLOSING_EXTENT);
    }
}

#[tokio::test]
async fn a_book_that_moved_is_reported_as_moved_not_as_a_missing_item_count() {
    let book = Book {
        flags: flags_with_count(None),
        closing_extent: extent_with_mark(260),
        ..Book::captured()
    };
    let (result, observed) = run(book.through_closing_extent()).await;
    let error = result.err().expect("refused");
    assert!(matches!(
        cause::<PairedReadValidationError>(&error),
        Some(PairedReadValidationError::StockSummaryExtent)
    ));
    assert!(cause::<NativeStockError>(&error).is_none());
    assert_eq!(observed.len(), AT_CLOSING_EXTENT);
}

#[tokio::test]
async fn a_quantity_bridge_cannot_read_does_not_refuse_the_read() {
    // An edit of captured text: a compound unit. The quantity is
    // `unread` and none is returned, so the book is still read, and the totals count
    // the quantity as unread.
    let book = Book {
        items: replaced(
            &items(),
            "<CLOSINGBALANCE TYPE=\"Quantity\"> 100 Box</CLOSINGBALANCE>",
            "<CLOSINGBALANCE TYPE=\"Quantity\"> 2 Box of 10 Nos</CLOSINGBALANCE>",
        ),
        ..Book::captured()
    };
    let (result, observed) = run(book.complete()).await;
    let NativeStockGate::ValueTotalMatched { items, totals, .. } = result.unwrap().0.gate else {
        panic!("the value total still ties");
    };
    assert_eq!(items.len(), 11);
    assert_eq!(totals.closing_quantity_unread_count, 1);
    assert_eq!(observed.len(), WHOLE_READ);
}
