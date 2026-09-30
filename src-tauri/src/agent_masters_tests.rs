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
        _ => panic!("no capture for {kind}"),
    };
    decode(bytes)
        .replace(CAPTURE_GUID, GUID)
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
fn the_tool_definition_admits_only_the_five_kinds_and_states_the_size_limits() {
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
    pair(&mut plans, xml(extents(14, MARK)));
    let one = OneServer::spawn(plans);
    let refused = one.call(args("godowns", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_read_failed");
    assert_eq!(error(&refused)["cause"], "masters_collection_absent");
    assert_eq!(one.requests(), 4 + 3 + 4 + 4 + 4);
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

#[tokio::test]
async fn master_names_are_not_masked_under_mask_parties() {
    // They are not party names: a masked read returns them as they are.
    let one = OneServer::spawn_with(
        first_page_plans(capture("godowns"), 14),
        Redaction::MaskParties,
    );
    let response = one.call(args("godowns", 0, 500, None)).await;
    let names = result(&response)["masters"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["name"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(names, ["Factory Floor", "Main Location"]);
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}
