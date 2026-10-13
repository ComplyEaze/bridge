//! The `masters` tool through the MCP adapter, replayed from the live captures
//! (`tests/fixtures/masters_*`) with their book's GUID replaced by the test
//! double's company. The runtime read's own faults are `runtime_masters_tests`.
use super::super::*;
use bridge_tally_protocol::native_masters::{
    masters_worst_row_bytes, NativeMasterKind, MASTERS_RESPONSE_BUDGET_BYTES,
};
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const GROUPS_GUID: &str = "bb8ad19e-6aef-4239-a917-87fec0c6215e";

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

fn capture(kind: &str) -> String {
    let bytes: &[u8] = match kind {
        "voucher_types" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_voucher_types_shape_lab_live.utf16le.xml"
        ),
        "godowns" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_godowns_shape_lab_live.utf16le.xml"
        ),
        "units" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_units_shape_lab_live.utf16le.xml"
        ),
        // The second synthetic book, without inventory: a present, empty collection.
        "units_empty" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_units_reads_lab_live.utf16le.xml"
        ),
        "stock_groups" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_stock_groups_shape_lab_live.utf16le.xml"
        ),
        "cost_centres" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_cost_centres_shape_lab_flag_no_live.utf16le.xml"
        ),
        "cost_categories" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_cost_categories_shape_lab_live.utf16le.xml"
        ),
        // The parity book (setting at Yes), scrubbed: three centres, one under another.
        "cost_centres_parity" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_cost_centres_parity_flag_yes_live.utf16le.xml"
        ),
        _ => panic!("no capture for {kind}"),
    };
    decode(bytes)
        .replace(CAPTURE_GUID, GUID)
        .replace("7c0de000-0000-4000-8000-0000000000a1", GUID)
        .replace("de2e15f2-6d42-4715-b6e7-b7a95a68abe8", GUID)
}

fn groups() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/native/group_snapshot_aarav_with_computed_company_guid.xml"
    )
    .replace(GROUPS_GUID, GUID)
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

/// The identity read, then the runtime read up to and including its opening
/// extent: everything a read sends before it decides to send a collection.
fn through_opening_extent(vouchers: u64, masters: u64) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = identity_plans();
    plans.extend([status(), companies.clone(), companies]);
    pair(&mut plans, xml(extents(vouchers, masters)));
    plans
}

/// The master mark the books carry in these tests: above every captured row's
/// AlterID (the largest is 269) and inside every admission cap.
const MARK: u64 = 1_000;

/// What a whole first-page call sends: identity (4), the runtime read's mode
/// probe and identity (3), opening extent (4), the collection pair (4),
/// closing extent (4), identity and mode again (3).
const FIRST_PAGE_REQUESTS: usize = 4 + 3 + 4 + 4 + 4 + 3;

/// A whole first-page call: identity, then the runtime read's brackets
/// around `collection`, as `runtime_masters_tests` replays them.
fn first_page_plans(collection: String, vouchers: u64) -> Vec<ScenarioPlan> {
    first_page_plans_at(collection, vouchers, MARK)
}

fn first_page_plans_at(collection: String, vouchers: u64, mark: u64) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = through_opening_extent(vouchers, mark);
    pair(&mut plans, xml(collection));
    pair(&mut plans, xml(extents(vouchers, mark)));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

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
        self.server.call_tool("masters", args).await
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

fn args(kind: &str, offset: usize, limit: usize, snapshot_id: Option<&str>) -> Value {
    let mut args = json!({"company_guid":GUID,"kind":kind,"offset":offset,"limit":limit});
    if let Some(id) = snapshot_id {
        args["snapshot_id"] = json!(id);
    }
    args
}

#[tokio::test]
async fn an_unknown_or_missing_kind_is_refused_before_any_read() {
    // A server whose Tally is not there: a read would fail as a refused
    // connection, not as the argument refusals asserted here.
    let one = OneServer::unreachable();
    let refused_with = |response: Value, code: &str| {
        assert_eq!(error(&response)["code"], code, "{response}");
        assert_eq!(response["structuredContent"]["evidence"]["bytes"], 0);
    };
    for kind in ["ledgers", "Godowns", "stock_items", "", "voucher_type"] {
        refused_with(
            one.call(json!({"company_guid":GUID,"kind":kind})).await,
            "argument_invalid:kind",
        );
    }
    refused_with(
        one.call(json!({"company_guid":GUID,"kind":7})).await,
        "argument_invalid:kind",
    );
    refused_with(
        one.call(json!({"company_guid":GUID})).await,
        "kind_required",
    );
    refused_with(
        one.call(json!({"company_guid":GUID,"kind":"godowns","kinds":["units"]}))
            .await,
        "argument_unknown",
    );
}

#[test]
fn the_tool_definition_admits_only_the_seven_kinds_and_states_the_size_limits() {
    let definitions = tool_definitions(false, false);
    let tool = definitions
        .as_array()
        .and_then(|tools| tools.iter().find(|tool| tool["name"] == "masters"))
        .expect("masters tool definition");
    let schema = &tool["inputSchema"];
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(schema["required"], json!(["company_guid", "kind"]));
    assert_eq!(
        schema["properties"]["kind"]["enum"],
        json!([
            "voucher_types",
            "godowns",
            "units",
            "stock_groups",
            "cost_centres",
            "cost_categories",
            "groups"
        ])
    );
    assert_eq!(schema["properties"]["offset"]["minimum"], 0);
    assert_eq!(schema["properties"]["limit"]["minimum"], 1);
    // The prose's admitted marks are the constants', for the kinds that are
    // sized by the mark; voucher types are not, and say so.
    let description = tool["description"].as_str().unwrap();
    for kind in [
        NativeMasterKind::Godowns,
        NativeMasterKind::Units,
        NativeMasterKind::StockGroups,
        NativeMasterKind::CostCentres,
        NativeMasterKind::CostCategories,
    ] {
        let mark = MASTERS_RESPONSE_BUDGET_BYTES / masters_worst_row_bytes(kind);
        let written = format!("{},{:03}", mark / 1000, mark % 1000);
        assert!(
            mark >= 1000 && description.contains(&written),
            "{kind:?} {written}"
        );
    }
    assert!(description.contains("`voucher_types` and `groups` have no size check"));
    assert!(
        !description.contains("for voucher types"),
        "no cap is claimed for them"
    );
    assert!(description.contains("masters_too_large"));
    assert!(description.contains("Education mode is refused"));
    // What `mask_parties` masks, and why the rest is not masked.
    assert!(description.contains(
        "godown, stock-group, cost-centre and cost-category names, their parents and a cost centre's category are masked"
    ));
    assert!(description.contains("configuration labels, not counterparties"));
    assert!(description.contains("masters_voucher_types_empty"));
}

#[tokio::test]
async fn a_page_holds_one_read_and_the_next_continues_it() {
    let mut plans = first_page_plans(capture("godowns"), 14);
    plans.extend(continuation_plans(14));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(args("godowns", 0, 1, None)).await;
    let page = result(&first);
    assert_eq!(page["kind"], "godowns");
    assert_eq!(page["company_guid"], GUID);
    assert_eq!(page["total"], 2);
    assert_eq!(page["next_offset"], 1);
    assert_eq!(first["structuredContent"]["truncated"], true);
    assert_eq!(page["snapshot"]["reused"], false);
    let row = &page["masters"][0];
    assert_eq!(row["name"], "Factory Floor");
    assert_eq!(row["guid"], format!("{GUID}-000000d4"));
    assert_eq!(
        (row["master_id"].as_u64(), row["alter_id"].as_u64()),
        (Some(212), Some(213))
    );
    assert_eq!(row["parent"], "\u{fffd}#4; Primary");
    assert!(
        row.get("active").is_none(),
        "a godown has no voucher type fields"
    );
    let id = page["snapshot"]["id"].as_str().unwrap().to_string();

    let second = one.call(args("godowns", 1, 1, Some(&id))).await;
    let page = result(&second);
    assert_eq!(page["snapshot"]["reused"], true);
    assert_eq!(page["masters"][0]["name"], "Main Location");
    assert_eq!(page["next_offset"], Value::Null);
    assert_eq!(page["total"], 2);
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn a_moved_book_or_another_snapshot_refuses_a_page_that_names_its_snapshot() {
    let mut plans = first_page_plans(capture("godowns"), 14);
    plans.extend(continuation_plans(15));
    plans.extend(continuation_plans(14));
    plans.extend(continuation_plans(14));
    let total = plans.len();
    let one = OneServer::spawn(plans);
    let first = one.call(args("godowns", 0, 1, None)).await;
    let id = result(&first)["snapshot"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    // A voucher was posted since the first page.
    let moved = one.call(args("godowns", 1, 1, Some(&id))).await;
    assert_eq!(error(&moved)["code"], "listing_snapshot_changed");
    assert_eq!(error(&moved)["cause"], "book_changed_since_first_page");
    // An id this server never held.
    let wrong = one
        .call(args("godowns", 1, 1, Some("not-a-snapshot")))
        .await;
    assert_eq!(error(&wrong)["code"], "listing_snapshot_changed");
    assert_eq!(error(&wrong)["cause"], "snapshot_not_held");
    // Another kind is another listing: the godowns snapshot does not serve it.
    let other = one.call(args("units", 1, 1, Some(&id))).await;
    assert_eq!(error(&other)["code"], "listing_snapshot_changed");
    assert_eq!(error(&other)["cause"], "snapshot_not_held");
    assert_eq!(one.requests(), total);
}

#[tokio::test]
async fn voucher_types_report_their_numbering_and_flags() {
    let one = OneServer::spawn(first_page_plans(capture("voucher_types"), 14));
    let response = one.call(args("voucher_types", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["total"], 26);
    let rows = page["masters"].as_array().unwrap();
    let numbering = |method: &str| {
        rows.iter()
            .filter(|row| row["numbering_method"] == method)
            .count()
    };
    assert_eq!(
        (
            numbering("default"),
            numbering("automatic"),
            numbering("manual")
        ),
        (24, 1, 1)
    );
    let attendance = &rows[0];
    assert_eq!(attendance["name"], "Attendance");
    assert_eq!(attendance["parent"], "Attendance");
    assert_eq!(attendance["active"], false);
    assert_eq!(attendance["optional"], false);
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line.as_str().unwrap().contains("unrecognised")));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn an_unrecognised_numbering_method_is_returned_raw() {
    let unknown = capture("voucher_types").replacen(
        "<NUMBERINGMETHOD TYPE=\"String\">Default</NUMBERINGMETHOD>",
        "<NUMBERINGMETHOD TYPE=\"String\">Serial</NUMBERINGMETHOD>",
        1,
    );
    let one = OneServer::spawn(first_page_plans(unknown, 14));
    let response = one.call(args("voucher_types", 0, 500, None)).await;
    let rows = result(&response)["masters"].as_array().unwrap().clone();
    assert_eq!(
        rows[0]["numbering_method"],
        json!({"unrecognised":"Serial"})
    );
    assert_eq!(rows[1]["numbering_method"], "default");
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
    // The type-level value is not the method (#1485): the limitation a page carries
    // and the tool description both say so, and neither claims the three values
    // are the only ones seen.
    let page = result(&response);
    let limitations: Vec<&str> = page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|line| line.as_str().unwrap())
        .collect();
    assert!(
        limitations
            .iter()
            .any(|line| line.contains("not the voucher type's numbering method")),
        "{limitations:?}"
    );
    assert!(!limitations
        .iter()
        .any(|line| line.contains("only those three seen")));
    assert!(
        limitations.iter().any(|line| line.contains(
            "It can differ from the number series that decides what a supplied number does; do not pass it to voucher_presence's numbering."
        )),
        "{limitations:?}"
    );
    let definitions = catalog::registered_tool_definitions(true, true);
    let description = definitions
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "masters")
        .unwrap()["description"]
        .as_str()
        .unwrap();
    assert!(description.contains("is not the voucher type's numbering method"));
    assert!(!description.contains("the only numbering methods seen"));
    // voucher_presence takes the method the person confirms for the series, not this read's value.
    let presence = definitions
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "voucher_presence")
        .unwrap();
    let numbering = presence["inputSchema"]["properties"]["numbering"]["description"]
        .as_str()
        .unwrap();
    assert!(
        numbering.contains(
            "with its numbering method as the person confirms it for the series used, not the numbering_method of the masters tool:"
        ),
        "{numbering}"
    );
}

#[tokio::test]
async fn units_report_their_decimal_places_and_no_parent() {
    let one = OneServer::spawn(first_page_plans(capture("units"), 14));
    let response = one.call(args("units", 0, 500, None)).await;
    let rows = result(&response)["masters"].as_array().unwrap().clone();
    let kgs = rows.iter().find(|row| row["name"] == "Kgs").unwrap();
    assert_eq!(kgs["decimal_places"], 3);
    assert_eq!(kgs["simple"], true);
    assert!(
        kgs.get("formal_name").is_none(),
        "never observed, not exposed"
    );
    assert_eq!(kgs["parent"], Value::Null);
    let compound = rows
        .iter()
        .find(|row| row["name"] == "Nos of 10 Box")
        .unwrap();
    assert_eq!(compound["simple"], false);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn groups_come_from_the_group_snapshot_with_their_own_row_shape() {
    let one = OneServer::spawn(first_page_plans(groups(), 14));
    let response = one.call(args("groups", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["kind"], "groups");
    assert_eq!(page["total"], 28);
    let bank = page["masters"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == "Bank Accounts")
        .unwrap();
    assert_eq!(bank["reserved_name"], "Bank Accounts");
    assert!(bank["parent"].is_string());
    assert!(
        bank.get("guid").is_none(),
        "the group snapshot carries no GUID"
    );
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line.as_str().unwrap().contains("no size check")));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_book_too_large_for_a_whole_read_is_refused_with_its_size() {
    let mark = u64::try_from(
        MASTERS_RESPONSE_BUDGET_BYTES / masters_worst_row_bytes(NativeMasterKind::Godowns),
    )
    .unwrap()
        + 1;
    let one = OneServer::spawn(through_opening_extent(14, mark));
    let refused = one.call(args("godowns", 0, 500, None)).await;
    let refusal = error(&refused);
    assert_eq!(refusal["code"], "masters_too_large");
    assert_eq!(refusal["size"]["master_alter_id"], mark);
    assert_eq!(refusal["size"]["limit_bytes"], 16_000_000);
    assert!(refusal["size"]["estimated_bytes"].as_u64().unwrap() > 16_000_000);
    // The largest mark a godown read is admitted at: one below this refusal.
    assert_eq!(refusal["size"]["limit_master_alter_id"], mark - 1);
    assert!(refusal["remediation"]
        .as_str()
        .unwrap()
        .contains("A larger book refuses"));
    // Identity, then the runtime read's mode probe, identity and extent: no
    // collection request.
    assert_eq!(one.requests(), 4 + 3 + 4);
}

#[tokio::test]
async fn stock_groups_read_end_to_end_with_their_parent() {
    let one = OneServer::spawn(first_page_plans(capture("stock_groups"), 14));
    let response = one.call(args("stock_groups", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["kind"], "stock_groups");
    assert_eq!(page["total"], 3);
    let rows = page["masters"].as_array().unwrap();
    let packaging = rows.iter().find(|row| row["name"] == "Packaging").unwrap();
    assert_eq!(packaging["parent"], "\u{fffd}#4; Primary");
    assert_eq!(packaging["alter_id"], 264);
    assert!(packaging.get("active").is_none());
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn cost_centres_read_end_to_end_with_their_category_although_the_setting_is_off() {
    let one = OneServer::spawn(first_page_plans(capture("cost_centres"), 14));
    let response = one.call(args("cost_centres", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["kind"], "cost_centres");
    assert_eq!(page["total"], 2);
    let rows = page["masters"].as_array().unwrap();
    let assembly = rows.iter().find(|row| row["name"] == "Assembly").unwrap();
    assert_eq!(assembly["category"], "Business Line");
    assert_eq!(assembly["parent"], "\u{fffd}#4; Primary");
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line
            .as_str()
            .unwrap()
            .contains("whether or not the company's Cost Centres setting is on")));
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line
            .as_str()
            .unwrap()
            .contains("does not return how a voucher was allocated")));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn cost_categories_read_end_to_end_with_their_allocation_flags() {
    let one = OneServer::spawn(first_page_plans(capture("cost_categories"), 14));
    let response = one.call(args("cost_categories", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["kind"], "cost_categories");
    assert_eq!(page["total"], 2);
    let rows = page["masters"].as_array().unwrap();
    let line = rows
        .iter()
        .find(|row| row["name"] == "Business Line")
        .unwrap();
    assert_eq!(line["allocates_revenue"], true);
    assert_eq!(line["allocates_non_revenue"], false);
    assert_eq!(line["affects_stock"], false);
    assert!(line.get("parent").is_some(), "the key stays, null");
    assert_eq!(line["parent"], Value::Null);
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line
            .as_str()
            .unwrap()
            .contains("predefined Primary Cost Category always exists")));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_category_that_affects_stock_is_returned_as_such() {
    // The captured answer holds only categories that do not affect stock; the one flag of the last
    // row is changed in the captured bytes, so the output is read from the wire and not defaulted,
    // and all three flags of both rows are asserted so that a swap of two flags shows.
    let captured = capture("cost_categories");
    let flag = "<AFFECTSSTOCK TYPE=\"Logical\">No</AFFECTSSTOCK>";
    let at = captured.rfind(flag).expect("a flag");
    let changed = format!(
        "{}<AFFECTSSTOCK TYPE=\"Logical\">Yes</AFFECTSSTOCK>{}",
        &captured[..at],
        &captured[at + flag.len()..]
    );
    let one = OneServer::spawn(first_page_plans(changed, 14));
    let response = one.call(args("cost_categories", 0, 500, None)).await;
    let rows = result(&response)["masters"].as_array().unwrap().clone();
    let stock_flags = rows
        .iter()
        .map(|row| {
            (
                row["name"].as_str().unwrap().to_string(),
                json!([
                    row["allocates_revenue"],
                    row["allocates_non_revenue"],
                    row["affects_stock"]
                ]),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        stock_flags,
        [
            ("Business Line".to_string(), json!([true, false, false])),
            (
                "Primary Cost Category".to_string(),
                json!([true, true, true])
            ),
        ]
    );
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn cost_centre_and_category_names_and_categories_are_masked_under_mask_parties() {
    // A customer can name a cost centre after itself, so the names and the category a centre carries
    // are masked; the allocation flags of a category are not names and stay.
    let mut plans = first_page_plans(capture("cost_centres"), 14);
    plans.extend(first_page_plans(capture("cost_categories"), 14));
    let one = OneServer::spawn_with(plans, Redaction::MaskParties);

    let response = one.call(args("cost_centres", 0, 500, None)).await;
    let page = result(&response);
    let rows = page["masters"].as_array().unwrap();
    let names = rows
        .iter()
        .map(|row| (row["name"].clone(), row["category"].clone()))
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            (json!(mask("Assembly")), json!(mask("Business Line"))),
            (json!(mask("Trading")), json!(mask("Business Line"))),
        ]
    );
    // A mask that kept the plain text would pass the above if `mask` did.
    assert_ne!(mask("Business Line"), "Business Line");
    for plain in ["Assembly", "Trading", "Business Line"] {
        assert!(!response.to_string().contains(plain), "{plain}");
    }
    assert!(rows
        .iter()
        .all(|row| row["parent"] == json!("\u{fffd}#4; Primary")));

    let response = one.call(args("cost_categories", 0, 500, None)).await;
    let page = result(&response);
    let names = page["masters"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["name"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            json!(mask("Business Line")),
            json!(mask("Primary Cost Category"))
        ]
    );
    // The rows, not the limitations: a limitation names the predefined category in plain text.
    for plain in ["Business Line", "Primary Cost Category"] {
        assert!(!page["masters"].to_string().contains(plain), "{plain}");
    }
    assert_eq!(page["masters"][0]["allocates_revenue"], json!(true));
    // A null parent keeps its key under masking: it is not dropped as if it were text.
    for row in page["masters"].as_array().unwrap() {
        assert!(row.get("parent").is_some(), "{row}");
        assert_eq!(row["parent"], Value::Null);
    }
    assert_eq!(one.requests(), 2 * FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_child_cost_centre_under_mask_parties_has_its_parent_masked_and_the_root_left() {
    // The parity book: two centres at the top level and one under another.
    let one = OneServer::spawn_with(
        first_page_plans(capture("cost_centres_parity"), 14),
        Redaction::MaskParties,
    );
    let response = one.call(args("cost_centres", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(
        names_and_parents(page),
        [
            (mask("Parity CC A"), json!("\u{fffd}#4; Primary")),
            (mask("Parity CC A1"), json!(mask("Parity CC A"))),
            (mask("Parity CC B"), json!("\u{fffd}#4; Primary")),
        ]
    );
    for row in page["masters"].as_array().unwrap() {
        assert_eq!(row["category"], json!(mask("Primary Cost Category")));
    }
    assert!(!response.to_string().contains("Parity CC"));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

/// A refusal of the read at once, after the first page: the same request count as the other
/// refusals of a bad answer (no closing extent, identity or mode read).
const REFUSED_AT_ONCE: usize = 4 + 3 + 4 + 4;

async fn refused_cause(kind: &str, answer: String) -> (String, usize) {
    let mut plans = through_opening_extent(14, MARK);
    pair(&mut plans, xml(answer));
    let one = OneServer::spawn(plans);
    let refused = one.call(args(kind, 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_read_failed", "{refused}");
    (
        error(&refused)["cause"].as_str().unwrap().to_string(),
        one.requests(),
    )
}

#[tokio::test]
async fn a_cost_centre_without_a_category_is_refused_through_the_tool() {
    let changed = capture("cost_centres").replacen(
        "<CATEGORY TYPE=\"String\">Business Line</CATEGORY>",
        "",
        1,
    );
    assert_ne!(changed, capture("cost_centres"));
    let (cause, requests) = refused_cause("cost_centres", changed).await;
    assert_eq!(cause, "masters_row_field_invalid:category");
    assert_eq!(requests, REFUSED_AT_ONCE);
}

#[tokio::test]
async fn a_cost_collection_that_does_not_carry_its_own_type_is_refused_through_the_tool() {
    for (kind, own, other) in [
        ("cost_centres", "MSTDEPTYPE=\"32\"", "MSTDEPTYPE=\"16\""),
        ("cost_categories", "MSTDEPTYPE=\"16\"", "MSTDEPTYPE=\"32\""),
    ] {
        let captured = capture(kind);
        assert!(captured.contains(own), "{kind}");
        let (cause, requests) = refused_cause(kind, captured.replace(own, other)).await;
        assert_eq!(cause, "masters_collection_type_unexpected", "{kind}");
        assert_eq!(requests, REFUSED_AT_ONCE, "{kind}");
    }
}

#[tokio::test]
async fn an_empty_cost_category_answer_is_refused_through_the_tool() {
    let captured = capture("cost_categories");
    let start = captured.find("<COLLECTION").unwrap();
    let open_end = start + captured[start..].find('>').unwrap() + 1;
    let close = captured.find("</COLLECTION>").unwrap();
    let empty = format!("{}{}", &captured[..open_end], &captured[close..]);
    let (cause, requests) = refused_cause("cost_categories", empty).await;
    assert_eq!(cause, "masters_cost_categories_empty");
    assert_eq!(requests, REFUSED_AT_ONCE);
}

#[tokio::test]
async fn a_cost_category_row_that_carries_a_parent_returns_none() {
    // A category has no parent: a PARENT on the wire is not read as one, and the key stays.
    let captured = capture("cost_categories");
    let tag = "<COSTCATEGORY NAME=\"Business Line\"";
    let at = captured.find(tag).unwrap();
    let open_end = at + captured[at..].find('>').unwrap() + 1;
    let changed = format!(
        "{}<PARENT TYPE=\"String\">Business Line</PARENT>{}",
        &captured[..open_end],
        &captured[open_end..]
    );
    let one = OneServer::spawn(first_page_plans(changed, 14));
    let response = one.call(args("cost_categories", 0, 500, None)).await;
    let rows = result(&response)["masters"].as_array().unwrap().clone();
    assert_eq!(rows.len(), 2);
    for row in &rows {
        assert!(row.get("parent").is_some(), "the key stays: {row}");
        assert_eq!(row["parent"], Value::Null, "{row}");
    }
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn voucher_types_have_no_size_check_before_the_read() {
    // A mark no godown, unit or stock group read would be admitted at.
    let one = OneServer::spawn(first_page_plans_at(
        capture("voucher_types"),
        14,
        50_000_000,
    ));
    let response = one.call(args("voucher_types", 0, 500, None)).await;
    assert_eq!(result(&response)["total"], 26);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_response_that_breaks_the_marks_premise_is_refused_with_its_cause() {
    // 26 voucher types against a mark of 25: read, then refused whole.
    let mut plans = through_opening_extent(14, 25);
    pair(&mut plans, xml(capture("voucher_types")));
    pair(&mut plans, xml(extents(14, 25)));
    let one = OneServer::spawn(plans);
    let refused = one.call(args("voucher_types", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_bound_premise_violated");
    assert_eq!(error(&refused)["cause"], "masters_rows_exceed_master_mark");
    assert!(refused["structuredContent"]["result"]["masters"].is_null());
    // The read itself completed, so its evidence is kept.
    assert_ne!(refused["structuredContent"]["evidence"]["bytes"], 0);
    // Held until the closing extent was read, and it had not moved.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4 + 4);
}

#[tokio::test]
async fn a_parser_refusal_surfaces_as_the_tool_error_with_its_masters_cause() {
    let captured = capture("godowns");
    let start = captured.find("<COLLECTION").unwrap();
    let end = captured.find("</COLLECTION>").unwrap() + "</COLLECTION>".len();
    let absent = format!("{}{}", &captured[..start], &captured[end..]);
    let mut plans = through_opening_extent(14, MARK);
    pair(&mut plans, xml(absent));
    let one = OneServer::spawn(plans);
    let refused = one.call(args("godowns", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_read_failed");
    assert_eq!(error(&refused)["cause"], "masters_collection_absent");
    // Refused at once: no closing extent, identity or mode read follows.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4);
}

#[tokio::test]
async fn a_kind_with_no_master_is_an_empty_page() {
    // A present, empty collection (captured on a book without inventory).
    let one = OneServer::spawn(first_page_plans(capture("units_empty"), 14));
    let response = one.call(args("units", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["total"], 0);
    assert_eq!(page["masters"], json!([]));
    assert_eq!(page["next_offset"], Value::Null);
    assert_eq!(response["structuredContent"]["truncated"], false);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

/// The capture with its last `PARENT` (the last row's) set to `parent`, which
/// is user text and not Tally's reserved root.
fn with_last_parent(text: &str, parent: &str) -> String {
    let root = "&#4; Primary</PARENT>";
    let at = text.rfind(root).expect("a root parent");
    format!(
        "{}{parent}</PARENT>{}",
        &text[..at],
        &text[at + root.len()..]
    )
}

fn names_and_parents(page: &Value) -> Vec<(String, Value)> {
    page["masters"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["name"].as_str().unwrap().to_string(),
                row["parent"].clone(),
            )
        })
        .collect()
}

#[tokio::test]
async fn godown_names_and_parents_are_masked_and_voucher_type_names_are_not() {
    // One masking server, two reads: the godown names are masked (the positive
    // control that masking is on), and the voucher-type names on the same
    // server are not.
    let godowns = with_last_parent(&capture("godowns"), "Factory Floor");
    let mut plans = first_page_plans(godowns, 14);
    plans.extend(first_page_plans(capture("voucher_types"), 14));
    let one = OneServer::spawn_with(plans, Redaction::MaskParties);

    let response = one.call(args("godowns", 0, 500, None)).await;
    let page = result(&response);
    let masked_root = json!("\u{fffd}#4; Primary");
    assert_eq!(
        names_and_parents(page),
        [
            (mask("Factory Floor"), masked_root.clone()),
            (mask("Main Location"), json!(mask("Factory Floor"))),
        ]
    );
    // A mask that kept the plain text would pass the above if `mask` did.
    assert_ne!(mask("Factory Floor"), "Factory Floor");
    assert!(!response.to_string().contains("Factory Floor"));
    assert!(!response.to_string().contains("Main Location"));
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line.as_str().unwrap().contains("are masked")));

    let response = one.call(args("voucher_types", 0, 500, None)).await;
    let page = result(&response);
    let names = names_and_parents(page);
    assert_eq!(names.len(), 26);
    assert_eq!(names[0], ("Attendance".to_string(), json!("Attendance")));
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line
            .as_str()
            .unwrap()
            .contains("not masked by mask_parties")));
    assert_eq!(one.requests(), 2 * FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn stock_group_names_and_parents_are_masked_and_unit_names_are_not() {
    let groups = with_last_parent(&capture("stock_groups"), "Packaging");
    let mut plans = first_page_plans(groups, 14);
    plans.extend(first_page_plans(capture("units"), 14));
    let one = OneServer::spawn_with(plans, Redaction::MaskParties);

    let response = one.call(args("stock_groups", 0, 500, None)).await;
    let masked_root = json!("\u{fffd}#4; Primary");
    assert_eq!(
        names_and_parents(result(&response)),
        [
            (mask("Finished Kits"), masked_root.clone()),
            (mask("Packaging"), masked_root),
            (mask("Raw Chemicals"), json!(mask("Packaging"))),
        ]
    );
    for plain in ["Finished Kits", "Packaging", "Raw Chemicals"] {
        assert!(!response.to_string().contains(plain), "{plain}");
    }

    let response = one.call(args("units", 0, 500, None)).await;
    let names = result(&response)["masters"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["name"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert!(names.contains(&"Kgs".to_string()), "{names:?}");
    assert_eq!(names.len(), 4);
    assert_eq!(one.requests(), 2 * FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn masked_godown_rows_are_held_marked_and_not_masked_without_mask_parties() {
    // Under no redaction the marker is materialised: the same names as plain text.
    let godowns = with_last_parent(&capture("godowns"), "Factory Floor");
    let one = OneServer::spawn(first_page_plans(godowns, 14));
    let response = one.call(args("godowns", 0, 500, None)).await;
    assert_eq!(
        names_and_parents(result(&response)),
        [
            ("Factory Floor".to_string(), json!("\u{fffd}#4; Primary")),
            ("Main Location".to_string(), json!("Factory Floor")),
        ]
    );
    assert!(!response.to_string().contains(PARTY_NAME_MARKER));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn account_group_names_are_not_masked() {
    let one = OneServer::spawn_with(first_page_plans(groups(), 14), Redaction::MaskParties);
    let response = one.call(args("groups", 0, 500, None)).await;
    let names = result(&response)["masters"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["name"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert!(names.contains(&"Bank Accounts".to_string()));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_voucher_type_answer_with_no_rows_is_refused_with_its_cause() {
    let captured = capture("voucher_types");
    let start = captured.find("<COLLECTION").unwrap();
    let open_end = start + captured[start..].find('>').unwrap() + 1;
    let close = captured.find("</COLLECTION>").unwrap();
    let empty = format!("{}{}", &captured[..open_end], &captured[close..]);
    let mut plans = through_opening_extent(14, MARK);
    pair(&mut plans, xml(empty));
    let one = OneServer::spawn(plans);
    let refused = one.call(args("voucher_types", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_read_failed");
    assert_eq!(error(&refused)["cause"], "masters_voucher_types_empty");
    // Refused at once: no closing extent, identity or mode read.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4);
}
