//! The `stock_summary` tool through the MCP adapter, replayed from the live
//! captures (`tests/fixtures/stock_*`, `company_inventory_flags_*`) with their
//! book's GUID replaced by the test double's company. The runtime read's own
//! faults are `runtime_stock_summary_tests`.
use super::super::*;
use bridge_tally_protocol::native_stock_summary::{
    parse_native_stock_items, stock_item_worst_row_bytes,
};
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
/// The only date stock has been measured at: the end of the financial year the
/// captures were taken over, 20250401 to 20260331.
const AS_OF: &str = "20260331";
/// The test company's books start here. The captured extent says 20260401, which
/// would put the captures' period before the books; the company's row in the
/// companies list and in the extent are moved a year back together, as the
/// identity bracket compares them.
const BOOKS_FROM: &str = "20250401";
/// The basis sentence for a read whose report lines summed to the items' sum.
const MATCHED_SENTENCE: &str = "The items' closing values equal the sum of the top-level lines of Tally's own Stock Summary.";
/// The start of the basis sentence for a read that was not compared.
const UNCHECKED_SENTENCE: &str =
    "They were NOT checked against the sum of the top-level lines of Tally's own Stock Summary";
/// The part of a basis that is the same whatever Tally reported.
const UNMEASURED_USE: &str =
    "how the books use them (as closing stock, or against a Stock-in-Hand ledger) is not measured";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// `row` (a company's own text) with its captured `BOOKSFROM` moved to
/// [`BOOKS_FROM`].
fn with_books_from(row: &str) -> String {
    let from = "<BOOKSFROM TYPE=\"Date\">20260401</BOOKSFROM>";
    assert_eq!(row.matches(from).count(), 1);
    row.replace(
        from,
        &format!("<BOOKSFROM TYPE=\"Date\">{BOOKS_FROM}</BOOKSFROM>"),
    )
}

fn companies() -> String {
    let text = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ));
    let at = text.find(GUID).unwrap();
    let start = text[..at].rfind("<COMPANY ").unwrap();
    let end = at + text[at..].find("</COMPANY>").unwrap();
    format!(
        "{}{}{}",
        &text[..start],
        with_books_from(&text[start..end]),
        &text[end..]
    )
}

fn items() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/stock_items_shape_lab_fy_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
}

fn report() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/stock_summary_report_shape_lab_fy_live.utf16le.xml"
    ))
}

fn flags() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_shape_lab_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
}

/// `text` with the first `from` replaced by `to`, which must be there.
fn replaced(text: &str, from: &str, to: &str) -> String {
    assert!(text.contains(from), "the capture has no {from}");
    text.replacen(from, to, 1)
}

/// The captured extents, with only this company's voucher and master marks
/// changed from the captured 14 and 224.
fn extents(vouchers: u64, masters: u64) -> String {
    let extent = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    );
    let at = extent.find(GUID).unwrap();
    let start = extent[..at].rfind("<COMPANY ").unwrap();
    let end = at + extent[at..].find("</COMPANY>").unwrap();
    let mut company = with_books_from(&extent[start..end]);
    for (from, to) in [
        (
            "<ALTVCHID TYPE=\"Number\"> 14</ALTVCHID>",
            format!("<ALTVCHID TYPE=\"Number\"> {vouchers}</ALTVCHID>"),
        ),
        (
            "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>",
            format!("<ALTMSTID TYPE=\"Number\"> {masters}</ALTMSTID>"),
        ),
    ] {
        assert_eq!(company.matches(from).count(), 1);
        company = company.replace(from, &to);
    }
    format!("{}{}{}", &extent[..start], company, &extent[end..])
}

fn xml(text: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(text))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

fn status() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
}

fn pair(plans: &mut Vec<ScenarioPlan>, response: ScenarioPlan) {
    plans.extend([response.clone(), status(), response, status()]);
}

fn identity_plans() -> Vec<ScenarioPlan> {
    let mut plans = Vec::new();
    pair(&mut plans, xml(companies()));
    plans
}

/// The master mark the books carry in these tests: above the rows' count and
/// inside the admission cap (874).
const MARK: u64 = 500;

/// What the three reads are scripted to answer; the default is the captures.
struct Book {
    flags: String,
    items: String,
    report: String,
}

impl Book {
    fn captured() -> Self {
        Self {
            flags: flags(),
            items: items(),
            report: report(),
        }
    }

    /// The identity read, then the runtime read's mode probe, identity and
    /// opening extent: everything a read sends before it reads the flags.
    fn through_opening_extent(vouchers: u64, mark: u64) -> Vec<ScenarioPlan> {
        let companies = xml(companies());
        let mut plans = identity_plans();
        plans.extend([status(), companies.clone(), companies]);
        pair(&mut plans, xml(extents(vouchers, mark)));
        plans
    }

    /// Through the flags pair, where an inventory that is off refuses.
    fn through_flags(&self, vouchers: u64, mark: u64) -> Vec<ScenarioPlan> {
        let mut plans = Self::through_opening_extent(vouchers, mark);
        pair(&mut plans, xml(self.flags.clone()));
        plans
    }

    /// Through the closing extent, where a refusal of the mark's premise is
    /// held until.
    fn through_closing_extent(&self, vouchers: u64, mark: u64) -> Vec<ScenarioPlan> {
        let mut plans = self.through_flags(vouchers, mark);
        pair(&mut plans, xml(self.items.clone()));
        pair(&mut plans, xml(self.report.clone()));
        pair(&mut plans, xml(extents(vouchers, mark)));
        plans
    }

    /// A whole first-page call: identity, then the runtime read's brackets
    /// around the flags, items and report, as `runtime_stock_summary_tests`
    /// replays them.
    fn first_page(&self, vouchers: u64, mark: u64) -> Vec<ScenarioPlan> {
        let companies = xml(companies());
        let mut plans = self.through_closing_extent(vouchers, mark);
        plans.extend([companies.clone(), status(), companies]);
        plans
    }
}

/// What a whole first-page call sends: identity (4), mode probe and identity
/// (3), opening extent (4), flags (4), items (4), report (4), closing extent
/// (4), identity and mode again (3).
const FIRST_PAGE_REQUESTS: usize = 4 + 3 + 4 + 4 + 4 + 4 + 4 + 3;

/// A continuation page: identity, then the bracketed, paired extent read.
fn continuation_plans(vouchers: u64) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = identity_plans();
    plans.push(companies.clone());
    pair(&mut plans, xml(extents(vouchers, MARK)));
    plans.push(companies);
    plans
}

struct OneServer {
    simulator: Option<SequenceSimulator>,
    server: Server,
    _directory: tempfile::TempDir,
}

impl OneServer {
    fn spawn(plans: Vec<ScenarioPlan>) -> Self {
        Self::spawn_with(plans, Redaction::None)
    }

    fn spawn_with(plans: Vec<ScenarioPlan>, redaction: Redaction) -> Self {
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let port = simulator.address().port();
        Self::on_port(Some(simulator), port, redaction)
    }

    /// A server whose Tally is not there: any request it sent would fail as a
    /// refused connection, not as the argument refusal a test looks for.
    fn unreachable() -> Self {
        Self::on_port(None, 9, Redaction::None)
    }

    fn on_port(simulator: Option<SequenceSimulator>, port: u16, redaction: Redaction) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(Settings {
            endpoint: TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port,
            },
            data_dir: directory.path().into(),
            max_rows: 500,
            max_bytes: 200_000,
            redaction,
            import_enabled: false,
            writes_enabled: false,
            batch_post_enabled: false,
        });
        Self {
            simulator,
            server,
            _directory: directory,
        }
    }

    async fn call(&self, args: Value) -> Value {
        self.server.call_tool("stock_summary", args).await
    }

    fn requests(self) -> usize {
        self.simulator.unwrap().finish().unwrap().len()
    }
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

fn error(response: &Value) -> &Value {
    assert_eq!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]["error"]
}

fn args(as_of: &str, offset: usize, limit: usize, snapshot_id: Option<&str>) -> Value {
    let mut args = json!({"company_guid":GUID,"as_of":as_of,"offset":offset,"limit":limit});
    if let Some(id) = snapshot_id {
        args["snapshot_id"] = json!(id);
    }
    args
}

fn names(page: &Value) -> Vec<String> {
    page["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["name"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn an_unsupported_date_or_filter_is_refused_before_any_read() {
    // A server whose Tally is not there: a read would fail as a refused
    // connection, not as the argument refusals asserted here.
    let one = OneServer::unreachable();
    let refused_with = |response: Value, code: &str| {
        assert_eq!(error(&response)["code"], code, "{response}");
        assert_eq!(response["structuredContent"]["evidence"]["bytes"], 0);
    };
    // Only a 31 March is admitted: no other date is measured for stock, the
    // days 1 and 2 and another month's 31st that other reads admit included.
    for as_of in [
        "20260731", "20260401", "20260402", "20260101", "20260315", "20260630",
    ] {
        refused_with(
            one.call(json!({"company_guid":GUID,"as_of":as_of})).await,
            "stock_summary_as_of_not_measured",
        );
    }
    refused_with(
        one.call(json!({"company_guid":GUID,"as_of":"2026-07-31"}))
            .await,
        "stock_summary_as_of_not_measured",
    );
    refused_with(
        one.call(json!({"company_guid":GUID,"as_of":"2026-02-30"}))
            .await,
        "invalid_date",
    );
    refused_with(
        one.call(json!({"company_guid":GUID})).await,
        "as_of_required",
    );
    refused_with(
        one.call(json!({"company_guid":GUID,"as_of":AS_OF,"from":AS_OF}))
            .await,
        "argument_unknown",
    );
    // The item filter: one to fifty GUIDs, each nonblank, none repeated.
    let many = (0..51).map(|n| format!("g-{n}")).collect::<Vec<_>>();
    for items in [
        json!([]),
        json!(many),
        json!(["a", "a"]),
        json!(["a", "A"]),
        json!(["a", " "]),
        json!(["a", 7]),
        json!("a"),
        // Over the schema's maxLength of 64.
        json!(["a".repeat(65)]),
    ] {
        refused_with(
            one.call(json!({"company_guid":GUID,"as_of":AS_OF,"items":items}))
                .await,
            "argument_invalid:items",
        );
    }
}

#[tokio::test]
async fn a_date_that_is_not_a_31_march_is_refused_with_its_remediation_before_any_request() {
    // A server whose Tally is not there: any request would fail as a refused
    // connection, not as the typed refusal asserted here, and no bytes of
    // evidence are recorded.
    let one = OneServer::unreachable();
    for as_of in ["2026-07-31", "2026-04-01", "2026-04-02"] {
        let response = one.call(args(as_of, 0, 500, None)).await;
        assert_eq!(error(&response)["code"], "stock_summary_as_of_not_measured");
        assert_eq!(response["structuredContent"]["evidence"]["bytes"], 0);
        let remediation = error(&response)["remediation"].as_str().unwrap();
        assert!(remediation.contains("31 March"), "{remediation}");
    }
}

#[test]
fn the_tool_definition_states_the_date_the_size_and_the_limits() {
    let definitions = tool_definitions(false, false);
    let tool = definitions
        .as_array()
        .and_then(|tools| tools.iter().find(|tool| tool["name"] == "stock_summary"))
        .expect("stock_summary tool definition");
    let schema = &tool["inputSchema"];
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(schema["required"], json!(["company_guid", "as_of"]));
    assert_eq!(schema["properties"]["items"]["minItems"], 1);
    assert_eq!(schema["properties"]["items"]["maxItems"], 50);
    // The schema's bound on each GUID is the one `item_filter` enforces.
    assert_eq!(schema["properties"]["items"]["items"]["maxLength"], 64);
    assert_eq!(
        schema["properties"]["items"]["items"]["maxLength"],
        super::MAX_ITEM_GUID_CHARS
    );
    assert_eq!(schema["properties"]["items"]["uniqueItems"], true);
    assert_eq!(schema["properties"]["offset"]["minimum"], 0);
    assert_eq!(schema["properties"]["limit"]["minimum"], 1);
    let description = tool["description"].as_str().unwrap();
    // It leads with what a caller searches for.
    assert!(description.starts_with("Return the stock summary: closing stock quantity and value"));
    // The prose's admitted mark is the constants'.
    let mark = 16_000_000 / stock_item_worst_row_bytes();
    assert_eq!(mark, 874);
    assert!(description.contains(&format!("{mark}")));
    for phrase in [
        "must be a 31 March",
        "the period ending 31 March 2026",
        "other years' 31 March",
        "admitted but unmeasured",
        "whenever any item's closing value is empty, whatever its quantity",
        "a book with no items has a `value_sum` of zero",
        "stock_summary_too_large",
        "Typical stock-heavy client books refuse today",
        "16,000,000 bytes",
        "company totals only",
        "Empty is not zero",
        "an empty closing quantity or value is returned as null and counted",
        "The opening quantity and value are read but not returned, because their as-at date is unmeasured",
        "what the sign means is unmeasured, and `value_sum` adds the values as sent, signs included",
        "the sum of the top-level lines of Tally's own Stock Summary",
        "The top-level `state` is `observed`",
        "`unchecked`",
        "`totals.value_sum_signs` is always `as_sent_meaning_unmeasured`",
        "what a negative value means is unmeasured",
        "`not_established`",
        "not measured",
        "ISINTEGRATED",
        "Education mode is refused",
        "tally_stock_summary_differs",
        "stock_not_enabled",
        "stock_summary_as_of_not_measured",
        "negative_closing_quantity_count",
        "`amount`",
        "`name` and `parent` are masked under mask_parties",
        "company_flags_not_one_row",
        "stock_quantity_unparseable",
        "`matched` can stand beside `partial` true",
    ] {
        assert!(description.contains(phrase), "{phrase}");
    }
    // What the description no longer says: no accounting claim, no "magnitude",
    // no MB figure for the admitted size.
    for phrase in [
        "magnitude",
        "16 MB",
        "ties to it",
        "NOT reconciled to the Balance",
        "day 1, 2 or 31",
        "stock_summary_as_of_unsupported",
        "unless its closing quantity is present and zero",
        "Tally's own Stock Summary total",
        "item names are not masked",
        "`opening` and `closing`",
    ] {
        assert!(!description.contains(phrase), "{phrase}");
    }
}

#[test]
fn an_item_guid_over_the_schemas_length_is_not_a_filter() {
    let filter = |guid: String| super::item_filter(&json!({"items": [guid]}));
    assert_eq!(filter("a".repeat(64)), Ok(Some(vec!["a".repeat(64)])));
    assert_eq!(
        filter("a".repeat(65)),
        Err("argument_invalid:items".to_string())
    );
    // Characters, as the schema counts them, not bytes.
    assert!(filter("é".repeat(64)).is_ok());
    assert!(filter("é".repeat(65)).is_err());
}

#[tokio::test]
async fn a_page_holds_one_read_and_the_next_continues_it_under_either_date_form() {
    let mut plans = Book::captured().first_page(14, MARK);
    plans.extend(continuation_plans(14));
    plans.extend(continuation_plans(14));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(args("2026-03-31", 0, 5, None)).await;
    let page = result(&first);
    assert_eq!(page["state"], "observed");
    assert_eq!(page["company_guid"], GUID);
    assert_eq!(page["as_of"], "20260331");
    assert_eq!(page["period"], json!({"from":"20250401","to":"20260331"}));
    assert_eq!(
        page["inventory"],
        json!({"integrated":"yes","inventory_on":"yes","batchwise":"yes"})
    );
    let basis = page["basis"].as_str().unwrap();
    assert!(basis.starts_with(
        "Tally reported ISINTEGRATED Yes. These are the stock items' closing values exactly as Tally sends them"
    ));
    assert!(basis.contains(UNMEASURED_USE), "{basis}");
    assert!(basis.contains(MATCHED_SENTENCE), "{basis}");
    assert!(!basis.contains("NOT checked"), "{basis}");
    assert_eq!(
        page["tie_out"],
        json!({"state":"matched","total":"3000.01","report_empty_amounts":0})
    );
    assert_eq!(page["total"], 11);
    assert_eq!(page["offset"], 0);
    assert_eq!(page["next_offset"], 5);
    // Only the 2026 year end was measured; another year's is admitted, said so.
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line.as_str().unwrap().starts_with(
            "Only the period ending 31 March 2026 has been measured: a 31 March of another year is admitted"
        )));
    assert_eq!(first["structuredContent"]["truncated"], true);
    assert_eq!(page["snapshot"]["reused"], false);
    assert_eq!(
        names(page),
        [
            "Carton Box Small",
            "Caustic Soda Flakes",
            "Cleaning Kit A",
            "Cleaning Kit B",
            "Descaler Concentrate"
        ]
    );
    assert_eq!(
        page["items"][0],
        json!({
            "name": "Carton Box Small", "guid": format!("{GUID}-00000110"),
            "parent": "Packaging", "base_unit": "Box",
            "closing": {"quantity": {"amount": "100", "unit": "Box"}, "value": "2500.00"},
        })
    );
    // The note beside `value_sum`, which is null here: always present.
    assert_eq!(page["totals"]["value_sum"], Value::Null);
    assert_eq!(page["totals"]["value_sum_signs"], "as_sent_meaning_unmeasured");
    // No returned item carries `opening`: it is read but not returned.
    assert!(page["items"]
        .as_array()
        .unwrap()
        .iter()
        .all(|item| item.get("opening").is_none()));
    // The limitations pin what the figures are and are not.
    let limitations = page["limitations"].as_array().unwrap();
    for line in [
        "Opening quantity and value are read but not returned, because their as-at date is unmeasured",
        "Values and their signs are exactly as Tally sends them: the one capture had items with a positive quantity and a negative value, and what the sign means is unmeasured; `value_sum` adds the values as sent, signs included (`totals.value_sum_signs` says so)",
    ] {
        assert!(limitations.iter().any(|found| found == line), "{line}");
    }
    // The totals cover the book: Zero Stock Item holds -50 Kgs with no value.
    assert_eq!(page["totals"]["item_count"], 11);
    assert_eq!(page["totals"]["negative_closing_quantity_count"], 1);
    let id = page["snapshot"]["id"].as_str().unwrap().to_string();

    // The same date in the other form continues the same read.
    let second = one.call(args(AS_OF, 5, 5, Some(&id))).await;
    let page = result(&second);
    assert_eq!(page["snapshot"]["reused"], true);
    // A later page of a matched read is `observed`, as its first page was.
    assert_eq!(page["state"], "observed");
    assert!(page["items"]
        .as_array()
        .unwrap()
        .iter()
        .all(|item| item.get("opening").is_none()));
    assert_eq!(page["next_offset"], 10);
    assert_eq!(
        names(page),
        [
            "HDPE Drum 50L",
            "Hydrochloric Acid 33pc",
            "Label Roll",
            "Soda Ash Light",
            "Sulphuric Acid 98pc"
        ]
    );
    let third = one.call(args(AS_OF, 10, 5, Some(&id))).await;
    let page = result(&third);
    assert_eq!(names(page), ["Zero Stock Item"]);
    assert_eq!(page["next_offset"], Value::Null);
    assert_eq!(page["total"], 11);
    assert_eq!(third["structuredContent"]["truncated"], false);
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn an_items_filter_selects_from_the_held_read_and_lists_a_guid_it_did_not_find() {
    let one = OneServer::spawn(Book::captured().first_page(14, MARK));
    let carton = format!("{GUID}-00000110");
    let label = format!("{GUID}-00000111");
    let unknown = format!("{GUID}-deadbeef");
    let response = one
        .call(json!({
            "company_guid": GUID, "as_of": AS_OF,
            // The match ignores ASCII case, and an unknown GUID is reported.
            "items": [label.to_uppercase(), carton, unknown],
        }))
        .await;
    let page = result(&response);
    // In the read's order, not the filter's.
    assert_eq!(names(page), ["Carton Box Small", "Label Roll"]);
    assert_eq!(page["total"], 2);
    assert_eq!(page["items_not_found"], json!([unknown]));
    // Totals and the tie-out are the whole book's.
    assert_eq!(page["totals"]["item_count"], 11);
    assert_eq!(page["tie_out"]["state"], "matched");
    assert_eq!(response["structuredContent"]["truncated"], false);
    // One filter that finds nothing is not an error.
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    let one = OneServer::spawn(Book::captured().first_page(14, MARK));
    let response = one
        .call(json!({"company_guid":GUID,"as_of":AS_OF,"items":[unknown]}))
        .await;
    let page = result(&response);
    assert_eq!(page["items"], json!([]));
    assert_eq!(page["total"], 0);
    assert_eq!(page["items_not_found"], json!([unknown]));
    assert!(page.get("items_not_found").is_some());
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_moved_book_another_date_or_another_snapshot_refuses_a_page_that_names_its_snapshot() {
    let mut plans = Book::captured().first_page(14, MARK);
    plans.extend(continuation_plans(15));
    plans.extend(continuation_plans(14));
    plans.extend(continuation_plans(14));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(args(AS_OF, 0, 5, None)).await;
    let id = result(&first)["snapshot"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    // A voucher was posted since the first page.
    let moved = one.call(args(AS_OF, 5, 5, Some(&id))).await;
    assert_eq!(error(&moved)["code"], "listing_snapshot_changed");
    assert_eq!(error(&moved)["cause"], "book_changed_since_first_page");
    // An id this server never held.
    let wrong = one.call(args(AS_OF, 5, 5, Some("not-a-snapshot"))).await;
    assert_eq!(error(&wrong)["code"], "listing_snapshot_changed");
    assert_eq!(error(&wrong)["cause"], "snapshot_not_held");
    // Another date is another listing: this snapshot does not serve it.
    let other = one.call(args("2025-03-31", 5, 5, Some(&id))).await;
    assert_eq!(error(&other)["code"], "listing_snapshot_changed");
    assert_eq!(error(&other)["cause"], "snapshot_not_held");
    assert_eq!(one.requests(), total);
}

/// A book whose Zero Stock Item (-50 Kgs, no value) has its quantity blanked:
/// its empty value now has no quantity to explain it either.
fn unstocked_items() -> String {
    replaced(
        &items(),
        "<CLOSINGBALANCE TYPE=\"Quantity\">-50.000 Kgs</CLOSINGBALANCE>",
        "<CLOSINGBALANCE TYPE=\"Quantity\"></CLOSINGBALANCE>",
    )
}

/// The captured items with every item that has an empty closing value given a
/// closing quantity of zero: Zero Stock Item's -50 Kgs becomes zero, and the
/// three items with neither a quantity nor a value are given one. An edit of
/// captured text.
fn zeroed_items() -> String {
    replaced(
        &items(),
        "<CLOSINGBALANCE TYPE=\"Quantity\">-50.000 Kgs</CLOSINGBALANCE>",
        "<CLOSINGBALANCE TYPE=\"Quantity\">0.000 Kgs</CLOSINGBALANCE>",
    )
    .replace(
        "<CLOSINGBALANCE TYPE=\"Quantity\"></CLOSINGBALANCE>",
        "<CLOSINGBALANCE TYPE=\"Quantity\">0 Nos</CLOSINGBALANCE>",
    )
}

/// The captured items with every empty closing value given an explicit `0.00`.
/// This is an edit of captured text, not a capture: it is how a book with no
/// empty closing value is made from the one capture there is.
fn explicit_zero_values() -> String {
    let from = "<CLOSINGVALUE TYPE=\"Amount\"></CLOSINGVALUE>";
    let text = items();
    assert_eq!(text.matches(from).count(), 4);
    text.replace(from, "<CLOSINGVALUE TYPE=\"Amount\">0.00</CLOSINGVALUE>")
}

#[tokio::test]
async fn the_value_sum_is_withheld_whenever_any_closing_value_is_empty() {
    // The capture itself: -50 Kgs with an empty value, and three items with
    // neither a quantity nor a value.
    let one = OneServer::spawn(Book::captured().first_page(14, MARK));
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    let totals = &result(&response)["totals"];
    assert_eq!(totals["value_sum"], Value::Null);
    assert_eq!(totals["partial"], true);
    assert_eq!(totals["empty_closing_value_count"], 4);
    assert_eq!(totals["empty_closing_quantity_count"], 3);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // Blanking that quantity too changes nothing.
    let book = Book {
        items: unstocked_items(),
        ..Book::captured()
    };
    let one = OneServer::spawn(book.first_page(14, MARK));
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["totals"]["value_sum"], Value::Null);
    assert_eq!(page["totals"]["partial"], true);
    assert_eq!(page["totals"]["negative_closing_quantity_count"], 0);
    assert_eq!(page["totals"]["empty_closing_quantity_count"], 4);
    assert_eq!(page["tie_out"]["state"], "matched");
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // Nor does a present zero quantity: that it makes an empty value zero is
    // unmeasured. The four items with an empty value are given a zero quantity
    // (an edit of captured text).
    let book = Book {
        items: zeroed_items(),
        ..Book::captured()
    };
    let one = OneServer::spawn(book.first_page(14, MARK));
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    let totals = &result(&response)["totals"];
    assert_eq!(totals["value_sum"], Value::Null);
    assert_eq!(totals["partial"], true);
    assert_eq!(totals["zero_quantity_count"], 4);
    assert_eq!(totals["empty_closing_value_count"], 4);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // With no empty closing value the sum is formed, and is the tie's.
    let book = Book {
        items: explicit_zero_values(),
        ..Book::captured()
    };
    let one = OneServer::spawn(book.first_page(14, MARK));
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["totals"]["value_sum"], "3000.01");
    // Present beside a non-null sum too.
    assert_eq!(page["totals"]["value_sum_signs"], "as_sent_meaning_unmeasured");
    assert_eq!(page["totals"]["partial"], false);
    assert_eq!(page["totals"]["empty_closing_value_count"], 0);
    assert_eq!(page["tie_out"]["state"], "matched");
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // One item's value blanked on top of that, with the report's Packaging
    // amount lowered by that value so the tie-out still matches: the sum is
    // withheld whatever the tie says, and the tie is of the grand total only.
    let book = Book {
        items: replaced(
            &explicit_zero_values(),
            "<CLOSINGVALUE TYPE=\"Amount\">2500.00</CLOSINGVALUE>",
            "<CLOSINGVALUE TYPE=\"Amount\"></CLOSINGVALUE>",
        ),
        report: replaced(
            &report(),
            "<DSPCLAMTA>14500.00</DSPCLAMTA>",
            "<DSPCLAMTA>12000.00</DSPCLAMTA>",
        ),
        ..Book::captured()
    };
    let one = OneServer::spawn(book.first_page(14, MARK));
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["totals"]["value_sum"], Value::Null);
    assert_eq!(page["totals"]["partial"], true);
    assert_eq!(page["tie_out"]["state"], "matched");
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line
            .as_str()
            .unwrap()
            .contains("`matched` can stand beside `partial: true`")));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_report_that_differs_withholds_every_item_and_is_not_held() {
    let book = Book {
        report: replaced(
            &report(),
            "<DSPCLAMTA>18750.00</DSPCLAMTA>",
            "<DSPCLAMTA>18750.01</DSPCLAMTA>",
        ),
        ..Book::captured()
    };
    let mut plans = book.first_page(14, MARK);
    // The later page below names no snapshot: it sends the continuation's
    // identity and extent reads, finds nothing held, and then reads afresh
    // (the captured book, which matches), so a whole first-page call's requests
    // follow, less the identity read the continuation already sent.
    plans.extend(continuation_plans(14));
    plans.extend(Book::captured().first_page(14, MARK).into_iter().skip(4));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    // Not an error call: the read completed and Tally's figures disagree.
    let page = result(&response);
    assert_eq!(page["state"], "not_established");
    assert_eq!(page["reason"], "tally_stock_summary_differs");
    assert_eq!(page["items"], Value::Null);
    assert_eq!(
        page["tie_out"],
        json!({"state":"differs","items_value_sum":"3000.01","report_total":"3000.02"})
    );
    assert!(page.get("totals").is_none(), "no figure derived from them");
    assert!(page.get("snapshot").is_none(), "nothing held");
    assert_eq!(page["as_of"], "20260331");
    assert_eq!(response["structuredContent"]["truncated"], false);
    // The limitation says this read is not held, and does not say that nothing
    // is: an earlier read of the same date that was returned, matched or not
    // checked, may still be.
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line.as_str().unwrap()
            == "This read is not held: a later page continues only from an earlier read of the same date that was returned (matched or not checked), if one is still held; call again with offset 0 to read afresh"));
    // A later page naming no snapshot: the differing read was not held, so
    // this reads afresh and serves its second page from that new read.
    let next = one.call(args(AS_OF, 5, 5, None)).await;
    let page = result(&next);
    assert_eq!(page["state"], "observed");
    assert_eq!(page["snapshot"]["reused"], false);
    assert_eq!(page["tie_out"]["state"], "matched");
    assert_eq!(
        names(page),
        [
            "HDPE Drum 50L",
            "Hydrochloric Acid 33pc",
            "Label Roll",
            "Soda Ash Light",
            "Sulphuric Acid 98pc"
        ]
    );
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn a_not_checked_read_is_held_and_a_later_page_continues_it() {
    // What the differing read's limitation names: a returned read that was not
    // checked against the report is held like a matched one.
    let text = report();
    let start = text.find("<ENVELOPE>").unwrap() + "<ENVELOPE>".len();
    let hollow = format!(
        "{}{}",
        &text[..start],
        &text[text.rfind("</ENVELOPE>").unwrap()..]
    );
    let book = Book {
        report: hollow,
        ..Book::captured()
    };
    let mut plans = book.first_page(14, MARK);
    plans.extend(continuation_plans(14));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(args(AS_OF, 0, 5, None)).await;
    let page = result(&first);
    assert_eq!(page["tie_out"]["state"], "not_checked");
    // Returned without a comparison: not `observed`, as a matched read is.
    assert_eq!(page["state"], "unchecked");
    assert_eq!(page["snapshot"]["reused"], false);
    let id = page["snapshot"]["id"].as_str().unwrap().to_string();
    let second = one.call(args(AS_OF, 5, 5, Some(&id))).await;
    let page = result(&second);
    assert_eq!(page["snapshot"]["reused"], true);
    assert_eq!(page["tie_out"]["state"], "not_checked");
    // A later page served from the held snapshot is as unchecked as its first.
    assert_eq!(page["state"], "unchecked");
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn an_empty_or_unknown_report_returns_the_items_and_says_they_are_unchecked() {
    let text = report();
    let start = text.find("<ENVELOPE>").unwrap() + "<ENVELOPE>".len();
    let hollow = format!(
        "{}{}",
        &text[..start],
        &text[text.rfind("</ENVELOPE>").unwrap()..]
    );
    let unknown =
        text.replacen("<ENVELOPE>", "<RESPONSE>", 1)
            .replacen("</ENVELOPE>", "</RESPONSE>", 1);
    for (report, reason) in [
        (hollow, "stock_report_empty"),
        (unknown, "stock_unknown_report"),
    ] {
        let book = Book {
            report,
            ..Book::captured()
        };
        let one = OneServer::spawn(book.first_page(14, MARK));
        let response = one.call(args(AS_OF, 0, 500, None)).await;
        let page = result(&response);
        assert_eq!(page["total"], 11);
        assert_eq!(page["state"], "unchecked");
        assert_eq!(
            page["totals"]["value_sum_signs"],
            "as_sent_meaning_unmeasured"
        );
        assert_eq!(
            page["tie_out"],
            json!({"state":"not_checked","reason":reason})
        );
        let basis = page["basis"].as_str().unwrap();
        assert!(basis.contains(UNCHECKED_SENTENCE), "{basis}");
        assert!(basis.contains(reason), "{basis}");
        // No tie is claimed of figures that were not compared.
        assert!(!basis.contains(MATCHED_SENTENCE), "{basis}");
        assert!(!basis.contains("equal the sum"), "{basis}");
        assert!(!basis.contains("ties"), "{basis}");
        assert!(basis.contains(UNMEASURED_USE), "{basis}");
        assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
    }
}

#[tokio::test]
async fn the_basis_states_only_what_tally_reported_about_integration() {
    for (integrated, expected) in [
        (Some("No"), "Tally reported ISINTEGRATED No."),
        (None, "Tally did not send ISINTEGRATED."),
    ] {
        let flags = match integrated {
            Some(text) => replaced(
                &flags(),
                "<ISINTEGRATED TYPE=\"Logical\">Yes</ISINTEGRATED>",
                &format!("<ISINTEGRATED TYPE=\"Logical\">{text}</ISINTEGRATED>"),
            ),
            None => replaced(
                &flags(),
                "<ISINTEGRATED TYPE=\"Logical\">Yes</ISINTEGRATED>",
                "",
            ),
        };
        let book = Book {
            flags,
            ..Book::captured()
        };
        let one = OneServer::spawn(book.first_page(14, MARK));
        let response = one.call(args(AS_OF, 0, 1, None)).await;
        let page = result(&response);
        let basis = page["basis"].as_str().unwrap();
        assert!(basis.starts_with(expected), "{page}");
        // Whatever Tally reported, how the books use the values is not claimed,
        // and a matched read says only that the sum equals the report lines' sum.
        assert!(basis.contains(UNMEASURED_USE), "{basis}");
        assert!(basis.ends_with(MATCHED_SENTENCE), "{basis}");
        assert!(!basis.contains("NOT checked"), "{basis}");
        assert!(!basis.contains("reconciled"), "{basis}");
        assert!(!basis.contains("ties"), "{basis}");
        assert_eq!(
            page["inventory"]["integrated"],
            if integrated.is_some() {
                "no"
            } else {
                "unknown"
            }
        );
        assert_eq!(page["tie_out"]["state"], "matched");
        assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
    }
}

#[tokio::test]
async fn a_book_too_large_for_a_whole_read_is_refused_with_its_size_before_any_item_request() {
    let mark = u64::try_from(16_000_000 / stock_item_worst_row_bytes()).unwrap() + 1;
    assert_eq!(mark, 875);
    let one = OneServer::spawn(Book::captured().through_flags(14, mark));
    let refused = one.call(args(AS_OF, 0, 500, None)).await;
    let refusal = error(&refused);
    assert_eq!(refusal["code"], "stock_summary_too_large");
    assert_eq!(refusal["size"]["master_alter_id"], mark);
    assert_eq!(refusal["size"]["limit_bytes"], 16_000_000);
    assert_eq!(refusal["size"]["limit_master_alter_id"], 874);
    assert_eq!(refusal["size"]["estimated_bytes"], 16_009_000);
    assert!(refusal["remediation"]
        .as_str()
        .unwrap()
        .contains("A larger book refuses"));
    assert!(refused["structuredContent"]["result"]["items"].is_null());
    // Identity, then the runtime read's mode probe, identity, opening extent
    // and flags: no stock item and no report request.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4);
}

#[tokio::test]
async fn an_inventory_that_is_off_is_refused_after_the_flags() {
    let book = Book {
        flags: replaced(
            &flags(),
            "<ISINVENTORYON TYPE=\"Logical\">Yes</ISINVENTORYON>",
            "<ISINVENTORYON TYPE=\"Logical\">No</ISINVENTORYON>",
        ),
        ..Book::captured()
    };
    let one = OneServer::spawn(book.through_flags(14, MARK));
    let refused = one.call(args(AS_OF, 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "stock_not_enabled");
    assert_eq!(one.requests(), 4 + 3 + 4 + 4);
}

#[tokio::test]
async fn a_flags_answer_that_is_not_one_row_is_a_typed_cause_of_the_read() {
    let text = flags();
    let start = text.find("<COMPANY NAME=").unwrap();
    let end = start + text[start..].find("</COMPANY>").unwrap() + "</COMPANY>".len();
    let book = Book {
        flags: format!("{}{}", &text[..start], &text[end..]),
        ..Book::captured()
    };
    let one = OneServer::spawn(book.through_flags(14, MARK));
    let refused = one.call(args(AS_OF, 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "stock_summary_read_failed");
    assert_eq!(error(&refused)["cause"], "company_flags_not_one_row");
    assert_eq!(one.requests(), 4 + 3 + 4 + 4);
}

#[tokio::test]
async fn an_as_of_before_the_books_or_after_today_is_refused_after_the_opening_extent() {
    // The books start on 20250401, so the 31 March before them is 20250331.
    let one = OneServer::spawn(Book::through_opening_extent(14, MARK));
    let refused = one.call(args("2025-03-31", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "stock_summary_as_of_before_books");
    assert_eq!(one.requests(), 4 + 3 + 4);
    let one = OneServer::spawn(Book::through_opening_extent(14, MARK));
    let refused = one.call(args("9999-03-31", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "stock_summary_as_of_in_future");
    assert_eq!(one.requests(), 4 + 3 + 4);
}

#[tokio::test]
async fn education_is_refused_before_identity_or_extent_dispatch() {
    let education = companies().replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    );
    let mut plans = identity_plans();
    plans.extend([status(), xml(education)]);
    let one = OneServer::spawn(plans);
    let refused = one.call(args(AS_OF, 0, 500, None)).await;
    assert_eq!(
        error(&refused)["code"],
        "stock_summary_education_unqualified"
    );
    assert_eq!(one.requests(), 4 + 2);
}

#[tokio::test]
async fn a_response_that_breaks_the_marks_premise_is_refused_with_its_cause() {
    // Eleven items against a mark of two: read, then refused whole.
    let one = OneServer::spawn(Book::captured().through_closing_extent(14, 2));
    let refused = one.call(args(AS_OF, 0, 500, None)).await;
    assert_eq!(
        error(&refused)["code"],
        "stock_summary_bound_premise_violated"
    );
    assert_eq!(error(&refused)["cause"], "stock_rows_exceed_master_mark");
    assert!(refused["structuredContent"]["result"]["items"].is_null());
    // The reads completed, so their evidence is kept.
    assert_ne!(refused["structuredContent"]["evidence"]["bytes"], 0);
    // Held until the closing extent was read, and it had not moved.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4 + 4 + 4 + 4);
}

#[tokio::test]
async fn a_parser_refusal_surfaces_as_the_tool_error_with_its_stock_cause() {
    let text = items();
    let start = text.find("<COLLECTION").unwrap();
    let end = text.find("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    let book = Book {
        items: format!("{}{}", &text[..start], &text[end..]),
        ..Book::captured()
    };
    let mut plans = book.through_flags(14, MARK);
    pair(&mut plans, xml(book.items.clone()));
    let one = OneServer::spawn(plans);
    let refused = one.call(args(AS_OF, 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "stock_summary_read_failed");
    assert_eq!(error(&refused)["cause"], "stock_collection_absent");
    // Returned at once: no report and no closing extent.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4 + 4);
}

#[tokio::test]
async fn item_names_are_marked_under_mask_parties_and_plain_without_it() {
    // Masked: the names go out under the party-name marker, and the guid, the
    // identity, stays plain.
    let one = OneServer::spawn_with(
        Book::captured().first_page(14, MARK),
        Redaction::MaskParties,
    );
    let response = one.call(args(AS_OF, 0, 3, None)).await;
    let page = result(&response);
    assert_eq!(
        names(page),
        [
            mask("Carton Box Small"),
            mask("Caustic Soda Flakes"),
            mask("Cleaning Kit A")
        ]
    );
    // A mask that kept the plain text would pass the above if `mask` did.
    assert_ne!(mask("Carton Box Small"), "Carton Box Small");
    for plain in ["Carton Box Small", "Caustic Soda Flakes", "Cleaning Kit A"] {
        assert!(!response.to_string().contains(plain), "{plain}");
    }
    assert_eq!(page["items"][0]["guid"], format!("{GUID}-00000110"));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // The `items` filter is by guid, so masking does not touch it.
    let one = OneServer::spawn_with(
        Book::captured().first_page(14, MARK),
        Redaction::MaskParties,
    );
    let response = one
        .call(json!({
            "company_guid": GUID, "as_of": AS_OF,
            "items": [format!("{GUID}-00000111")],
        }))
        .await;
    let page = result(&response);
    assert_eq!(names(page), [mask("Label Roll")]);
    assert_eq!(page["items"][0]["guid"], format!("{GUID}-00000111"));
    assert_eq!(page["items_not_found"], Value::Null);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // Without it the marker is materialised: the same names, as plain text.
    let one = OneServer::spawn(Book::captured().first_page(14, MARK));
    let response = one.call(args(AS_OF, 0, 3, None)).await;
    assert_eq!(
        names(result(&response)),
        ["Carton Box Small", "Caustic Soda Flakes", "Cleaning Kit A"]
    );
    assert!(!response.to_string().contains(PARTY_NAME_MARKER));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_masked_response_contains_no_item_or_stock_group_name_anywhere() {
    // The names are parsed from the capture, not written here.
    let rows = parse_native_stock_items(&items(), GUID).unwrap().rows;
    let item_names = rows.iter().map(|row| row.name.clone()).collect::<Vec<_>>();
    assert_eq!(item_names.len(), 11, "the check cannot pass vacuously");
    let mut group_names = rows
        .iter()
        .filter_map(|row| row.parent.clone())
        .collect::<Vec<_>>();
    group_names.sort();
    group_names.dedup();
    assert_eq!(group_names.len(), 3);
    let all = item_names.iter().chain(&group_names).collect::<Vec<_>>();

    // Positive control: unmasked, the whole response carries every one of them.
    let one = OneServer::spawn(Book::captured().first_page(14, MARK));
    let plain = one.call(args(AS_OF, 0, 500, None)).await.to_string();
    for name in &all {
        assert!(plain.contains(name.as_str()), "{name} is in the plain read");
    }
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // Masked: the whole serialized tool response, not one field, has none.
    let one = OneServer::spawn_with(
        Book::captured().first_page(14, MARK),
        Redaction::MaskParties,
    );
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    assert_eq!(result(&response)["total"], 11);
    let masked = response.to_string();
    for name in &all {
        assert!(!masked.contains(name.as_str()), "{name} leaked");
    }
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_stock_group_parent_is_marked_under_mask_parties_and_the_reserved_root_is_not() {
    // Carton Box Small's parent (Packaging) becomes Tally's reserved root, as
    // the group snapshot keeps it; the next two items keep their stock groups.
    let book = Book {
        items: replaced(
            &items(),
            "<PARENT TYPE=\"String\">Packaging</PARENT>",
            "<PARENT TYPE=\"String\">&#4; Primary</PARENT>",
        ),
        ..Book::captured()
    };
    let one = OneServer::spawn_with(book.first_page(14, MARK), Redaction::MaskParties);
    let response = one.call(args(AS_OF, 0, 3, None)).await;
    let page = result(&response);
    let names_and_parents = page["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| (item["name"].clone(), item["parent"].clone()))
        .collect::<Vec<_>>();
    assert_eq!(
        names_and_parents,
        [
            (
                json!(mask("Carton Box Small")),
                json!("\u{fffd}#4; Primary")
            ),
            (
                json!(mask("Caustic Soda Flakes")),
                json!(mask("Raw Chemicals"))
            ),
            (json!(mask("Cleaning Kit A")), json!(mask("Finished Kits"))),
        ]
    );
    // A mask that kept the plain text would pass the above if `mask` did.
    assert_ne!(mask("Raw Chemicals"), "Raw Chemicals");
    assert_ne!(mask("Finished Kits"), "Finished Kits");
    for plain in ["Raw Chemicals", "Finished Kits"] {
        assert!(!response.to_string().contains(plain), "{plain}");
    }
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line.as_str().unwrap().contains("`name` and `parent` are masked")));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn an_item_without_a_parent_keeps_a_null_parent_under_mask_parties() {
    let book = Book {
        items: replaced(&items(), "<PARENT TYPE=\"String\">Packaging</PARENT>", ""),
        ..Book::captured()
    };
    let one = OneServer::spawn_with(book.first_page(14, MARK), Redaction::MaskParties);
    let response = one.call(args(AS_OF, 0, 1, None)).await;
    let item = &result(&response)["items"][0];
    assert_eq!(item["name"], mask("Carton Box Small"));
    assert_eq!(item["parent"], Value::Null);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}
