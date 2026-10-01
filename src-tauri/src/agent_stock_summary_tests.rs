//! The `stock_summary` tool through the MCP adapter, replayed from the live
//! captures (`tests/fixtures/stock_*`, `company_inventory_flags_*`) with their
//! book's GUID replaced by the test double's company. The runtime read's own
//! faults are `runtime_stock_summary_tests`.
use super::super::*;
use bridge_tally_protocol::native_stock_summary::stock_item_worst_row_bytes;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
/// The test company's books start on 20260401 (the captured extent).
const AS_OF: &str = "20260731";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn companies() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ))
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
    let mut company = extent[start..end].to_string();
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
    // Only day 1, 2 or 31 is admitted: an arbitrary SVTODATE is unmeasured.
    for day in ["03", "15", "30"] {
        refused_with(
            one.call(json!({"company_guid":GUID,"as_of":format!("202606{day}")}))
                .await,
            "stock_summary_as_of_unsupported",
        );
    }
    refused_with(
        one.call(json!({"company_guid":GUID,"as_of":"2026-06-15"}))
            .await,
        "stock_summary_as_of_unsupported",
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
    ] {
        refused_with(
            one.call(json!({"company_guid":GUID,"as_of":AS_OF,"items":items}))
                .await,
            "argument_invalid:items",
        );
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
        "day 1, 2 or 31",
        "stock_summary_too_large",
        "Typical stock-heavy client books refuse today",
        "company totals only",
        "Empty is not zero",
        "not reconciled",
        "Education mode is refused",
        "tally_stock_summary_differs",
        "stock_not_enabled",
        "negative_closing_quantity_count",
    ] {
        assert!(description.contains(phrase), "{phrase}");
    }
}

#[tokio::test]
async fn a_page_holds_one_read_and_the_next_continues_it_under_either_date_form() {
    let mut plans = Book::captured().first_page(14, MARK);
    plans.extend(continuation_plans(14));
    plans.extend(continuation_plans(14));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(args("2026-07-31", 0, 5, None)).await;
    let page = result(&first);
    assert_eq!(page["state"], "observed");
    assert_eq!(page["company_guid"], GUID);
    assert_eq!(page["as_of"], "20260731");
    assert_eq!(page["period"], json!({"from":"20260401","to":"20260731"}));
    assert_eq!(
        page["inventory"],
        json!({"integrated":"yes","inventory_on":"yes","batchwise":"yes"})
    );
    assert!(page["basis"]
        .as_str()
        .unwrap()
        .starts_with("the books carry this item valuation as closing stock"));
    assert_eq!(
        page["tie_out"],
        json!({"state":"matched","total":"3000.01","report_empty_amounts":0})
    );
    assert_eq!(page["total"], 11);
    assert_eq!(page["offset"], 0);
    assert_eq!(page["next_offset"], 5);
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
            "opening": {"quantity": {"magnitude": "100", "unit": "Box"}, "value": "2500.00"},
            "closing": {"quantity": {"magnitude": "100", "unit": "Box"}, "value": "2500.00"},
        })
    );
    // The totals cover the book: Zero Stock Item holds -50 Kgs with no value.
    assert_eq!(page["totals"]["item_count"], 11);
    assert_eq!(page["totals"]["negative_closing_quantity_count"], 1);
    let id = page["snapshot"]["id"].as_str().unwrap().to_string();

    // The same date in the other form continues the same read.
    let second = one.call(args(AS_OF, 5, 5, Some(&id))).await;
    let page = result(&second);
    assert_eq!(page["snapshot"]["reused"], true);
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
    let other = one.call(args("2026-06-01", 5, 5, Some(&id))).await;
    assert_eq!(error(&other)["code"], "listing_snapshot_changed");
    assert_eq!(error(&other)["cause"], "snapshot_not_held");
    assert_eq!(one.requests(), total);
}

/// A book whose Zero Stock Item (-50 Kgs, no value) has its quantity blanked:
/// nothing that holds stock lacks a value.
fn unstocked_items() -> String {
    replaced(
        &items(),
        "<CLOSINGBALANCE TYPE=\"Quantity\">-50.000 Kgs</CLOSINGBALANCE>",
        "<CLOSINGBALANCE TYPE=\"Quantity\"></CLOSINGBALANCE>",
    )
}

#[tokio::test]
async fn the_value_sum_is_withheld_when_an_item_that_holds_stock_has_no_value() {
    // The capture itself: -50 Kgs with an empty value.
    let one = OneServer::spawn(Book::captured().first_page(14, MARK));
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    let totals = &result(&response)["totals"];
    assert_eq!(totals["value_sum"], Value::Null);
    assert_eq!(totals["partial"], true);
    assert_eq!(totals["empty_closing_value_count"], 4);
    assert_eq!(totals["empty_closing_quantity_count"], 3);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // Without that quantity the sum is the tie's.
    let book = Book {
        items: unstocked_items(),
        ..Book::captured()
    };
    let one = OneServer::spawn(book.first_page(14, MARK));
    let response = one.call(args(AS_OF, 0, 500, None)).await;
    let totals = &result(&response)["totals"];
    assert_eq!(totals["value_sum"], "3000.01");
    assert_eq!(totals["partial"], false);
    assert_eq!(totals["negative_closing_quantity_count"], 0);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);

    // One stocked item's value blanked, with the report's Packaging amount
    // lowered by that value so the tie-out still matches: a sum that leaves out
    // an item that holds stock is withheld whatever the tie says.
    let book = Book {
        items: replaced(
            &unstocked_items(),
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
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_report_that_differs_withholds_every_item_and_holds_nothing() {
    let book = Book {
        report: replaced(
            &report(),
            "<DSPCLAMTA>18750.00</DSPCLAMTA>",
            "<DSPCLAMTA>18750.01</DSPCLAMTA>",
        ),
        ..Book::captured()
    };
    let mut plans = book.first_page(14, MARK);
    plans.extend(continuation_plans(14));
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
    assert_eq!(page["as_of"], "20260731");
    assert_eq!(response["structuredContent"]["truncated"], false);
    // A later page has nothing to continue from.
    let next = one.call(args(AS_OF, 5, 5, Some("any"))).await;
    assert_eq!(error(&next)["cause"], "snapshot_not_held");
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
        assert_eq!(
            page["tie_out"],
            json!({"state":"not_checked","reason":reason})
        );
        let basis = page["basis"].as_str().unwrap();
        assert!(basis.contains("NOT checked against Tally's own Stock Summary total"));
        assert!(basis.contains(reason), "{basis}");
        assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
    }
}

#[tokio::test]
async fn the_basis_says_whether_the_books_carry_this_valuation_as_closing_stock() {
    for (integrated, expected) in [
        (
            Some("No"),
            "the Balance Sheet uses the Stock-in-Hand ledger's entered closing stock; this is Tally's item valuation and is NOT reconciled to it",
        ),
        (
            None,
            "Tally did not say whether inventory is integrated with the accounts",
        ),
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
        assert!(
            page["basis"].as_str().unwrap().starts_with(expected),
            "{page}"
        );
        assert_eq!(
            page["inventory"]["integrated"],
            if integrated.is_some() { "no" } else { "unknown" }
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
    // The books start on 20260401.
    let one = OneServer::spawn(Book::through_opening_extent(14, MARK));
    let refused = one.call(args("2026-03-31", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "stock_summary_as_of_before_books");
    assert_eq!(one.requests(), 4 + 3 + 4);
    let one = OneServer::spawn(Book::through_opening_extent(14, MARK));
    let refused = one.call(args("9999-12-31", 0, 500, None)).await;
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
async fn item_names_are_not_masked_under_mask_parties() {
    // They are not party names: a masked read returns them as they are.
    let one = OneServer::spawn_with(
        Book::captured().first_page(14, MARK),
        Redaction::MaskParties,
    );
    let response = one.call(args(AS_OF, 0, 3, None)).await;
    assert_eq!(
        names(result(&response)),
        ["Carton Box Small", "Caustic Soda Flakes", "Cleaning Kit A"]
    );
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}
