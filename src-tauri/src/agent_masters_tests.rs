//! The `masters` tool through the MCP adapter, replayed from the live captures
//! (`tests/fixtures/masters_*`) with their book's GUID replaced by the test
//! double's company. The runtime read's own faults are `runtime_masters_tests`.
use super::super::*;
use bridge_tally_protocol::native_masters::{
    masters_worst_row_bytes, parse_native_masters, NativeMasterKind, MASTERS_RESPONSE_BUDGET_BYTES,
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
        // Twelve items, one with two dated HSN and two dated GST entries.
        "stock_items" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_stock_items_shape_lab_two_dates_live.utf16le.xml"
        ),
        // The first answer of the sitting: eleven items, GST content on one only.
        "stock_items_baseline" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_stock_items_shape_lab_baseline_live.utf16le.xml"
        ),
        // The item as first saved: Tally's own entries, no code and no rate.
        "stock_items_created" => include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/masters_stock_items_shape_lab_item_created_live.utf16le.xml"
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

/// The real series-level numbering capture's book, replaced by the test double's company.
const NUMBERING_CAPTURE_GUID: &str = "3479ef95-3ecd-49b3-9349-d3a8e24b0db2";

/// A numbering answer for a voucher-type masters answer: the real numbering
/// capture's envelope, and for each masters row the capture's row of the same
/// name (or its `Contra` row where the capture has none) with only the row's
/// name, GUID and ids set to the masters row's.
fn numbering_for(masters: &str) -> String {
    let real = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/voucher_type_numbering_series_live.utf16le.xml"
    ))
    .trim_start_matches('\u{feff}')
    .replace(NUMBERING_CAPTURE_GUID, GUID);
    let mut templates = Vec::new();
    let mut rest = real.as_str();
    while let Some(start) = rest.find("<VOUCHERTYPE ") {
        let end = start + rest[start..].find("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
        templates.push(&rest[start..end]);
        rest = &rest[end..];
    }
    let name_of = |block: &str| {
        let at = block.find("NAME=\"").unwrap() + "NAME=\"".len();
        block[at..at + block[at..].find('"').unwrap()].to_string()
    };
    let set = |block: &str, tag: &str, value: &str| {
        let open = block.find(&format!("<{tag} ")).unwrap();
        let text = open + block[open..].find('>').unwrap() + 1;
        let end = text + block[text..].find(&format!("</{tag}>")).unwrap();
        format!("{}{}{}", &block[..text], value, &block[end..])
    };
    let rows = parse_native_masters(NativeMasterKind::VoucherTypes, masters, GUID)
        .unwrap()
        .rows
        .iter()
        .map(|row| {
            assert!(!row.name.contains(['&', '"', '<']), "{}", row.name);
            let block = templates
                .iter()
                .find(|block| name_of(block) == row.name)
                .or_else(|| templates.iter().find(|block| name_of(block) == "Contra"))
                .unwrap();
            let block = format!(
                "<VOUCHERTYPE NAME=\"{0}\" RESERVEDNAME=\"{0}\"{1}",
                row.name,
                &block[block.find('>').unwrap()..]
            );
            let block = set(&block, "GUID", &row.guid);
            let block = set(&block, "ALTERID", &format!(" {}", row.alter_id));
            set(&block, "MASTERID", &format!(" {}", row.master_id))
        })
        .collect::<String>();
    let start = real.find("<VOUCHERTYPE ").unwrap();
    let end = real.rfind("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
    format!("{}{}{}", &real[..start], rows, &real[end..])
}

/// `text` with the first `from` at or after `anchor` replaced by `to`.
fn edit_after(text: &str, anchor: &str, from: &str, to: &str) -> String {
    let anchor = text.find(anchor).expect(anchor);
    let at = anchor + text[anchor..].find(from).expect(from);
    format!("{}{}{}", &text[..at], to, &text[at + from.len()..])
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

/// The same for the voucher types, which read a second collection pair, the
/// series-level numbering, between the first and the closing extent.
const VOUCHER_TYPE_PAGE_REQUESTS: usize = FIRST_PAGE_REQUESTS + 4;

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

/// A whole first-page call of the voucher types: the collection pair, then the
/// numbering pair read after it, inside the same two extents.
fn voucher_type_plans(
    collection: String,
    numbering: String,
    vouchers: u64,
    mark: u64,
) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = voucher_type_plans_to_closing_extent(collection, numbering, vouchers, mark);
    plans.extend([companies.clone(), status(), companies]);
    plans
}

/// The same up to and including the closing extent: all a read sends that is
/// refused after it (a refusal held until the extent was read).
fn voucher_type_plans_to_closing_extent(
    collection: String,
    numbering: String,
    vouchers: u64,
    mark: u64,
) -> Vec<ScenarioPlan> {
    let mut plans = through_opening_extent(vouchers, mark);
    pair(&mut plans, xml(collection));
    pair(&mut plans, xml(numbering));
    pair(&mut plans, xml(extents(vouchers, mark)));
    plans
}

/// The captured voucher types and the numbering answer made for them.
fn captured_voucher_type_plans(vouchers: u64, mark: u64) -> Vec<ScenarioPlan> {
    let masters = capture("voucher_types");
    let numbering = numbering_for(&masters);
    voucher_type_plans(masters, numbering, vouchers, mark)
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
            posting: Posting::Off,
            master_writes_enabled: false,
            audit_enabled: false,
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
    for kind in [
        "ledgers",
        "Godowns",
        "stock_item",
        "Stock_Items",
        "",
        "voucher_type",
    ] {
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
fn the_tool_definition_admits_only_the_eight_kinds_and_states_the_size_limits() {
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
            "stock_items",
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
    // Stock items are admitted at a lower mark, because their row is allowed for
    // several dated entries: written without a thousands separator, and in the
    // item limitation of the answer as well.
    let items =
        MASTERS_RESPONSE_BUDGET_BYTES / masters_worst_row_bytes(NativeMasterKind::StockItems);
    assert_eq!(items, 489);
    assert!(description.contains(&format!("{items} for stock items")));
    // The ITC element goes out under the name of its meaning, with what is and
    // is not known; the old claim and the ledger's key are not used for items.
    assert!(description.contains("`ineligible_itc` (Tally's `GSTINELIGIBLEITC` as sent"));
    assert!(description.contains(
        "which fits an ineligibility flag; PARTIAL, one book, two rows, screen against wire"
    ));
    assert!(description.contains("what an absent element means for the tax is not known"));
    assert!(!description.contains("not the screen's setting"));
    assert!(!description.contains("`itc_eligible` (true"));
    // The kind's reach, its cut-off and what is unmeasured.
    assert!(description.contains("refused past 16,344 UTF-16 units"));
    assert!(description.contains(
        "a limit of 489 refuses, by arithmetic, both client books measured, not tried on them"
    ));
    assert!(!description.contains("stock item count instead"));
    assert!(description.contains("a zero-row stock-item answer are not measured"));
    assert!(description.contains("such as text, a reference or CDATA directly in the list"));
    assert!(description.contains("is masked like an item name"));
    // The inheritance sentence says only what was measured, in the tool text and in the README.
    let scoped = "the group's four other items still carried empty lists";
    assert!(description.contains(scoped));
    assert!(description.contains(
        "not measured: a GST rate on a group or on the company, an HSN code on the company, and whether an item that defers to a coded group inherits its code"
    ));
    assert!(!description.contains("does not carry a code or rate set on its stock group"));
    let readme = include_str!("../../docs/agent/README.md");
    // CHANGELOG.md is not read here: a test that includes it would put it in the native CI scope.
    let flat = readme.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(flat.contains(scoped), "README");
    assert!(
        flat.contains(
            "whether an item that defers to a coded group inherits its code, were not measured"
        ),
        "README"
    );
    assert!(
        !flat.contains("did not appear on the items' lists"),
        "README"
    );
    assert!(!description.contains("under every redaction setting"));
    assert!(description.contains("`ineligible_itc_withheld` is `tally_default_entry`"));
    assert!(description.contains("so recognise it by its source"));
    assert!(description.contains(
        "a flag Tally did not send stays null with no marker, and an entry with any other source is sent as it is"
    ));
    assert!(!description.contains("; On an entry"));
    assert!(!description.contains("dated 1 April 2025, source"));
    assert!(description.contains("`voucher_types` and `groups` have no size check"));
    assert!(
        !description.contains("for voucher types"),
        "no cap is claimed for them"
    );
    assert!(description.contains("masters_too_large"));
    assert!(description.contains("Education mode is refused"));
    // What `mask_parties` masks, and why the rest is not masked.
    assert!(description.contains(
        "godown, stock-group, stock-item, cost-centre and cost-category names, their parents and a cost centre's category are masked"
    ));
    assert!(description.contains("configuration labels, not counterparties"));
    assert!(description.contains("masters_voucher_types_empty"));
    // The series-level numbering, its refusals and what was measured.
    assert!(description.contains("`series_numbering`"));
    assert!(description.contains("masters_numbering_names_differ"));
    assert!(description.contains("masters_numbering_identity_differs"));
    assert!(description.contains("masters_numbering_collection_changed"));
    assert!(description.contains("which `series_numbering` returns instead"));
    assert!(
        !description.contains("which this read does not return"),
        "the series level is now returned"
    );
    assert!(description.contains("a single series not named `Default`"));
    // The sub-method: where it is returned, and what was not measured.
    assert!(description.contains("`sub_method`"));
    assert!(description.contains("when one was sent by import without a number"));
    assert!(description.contains("the same through ComplyEaze Bridge's own post are not measured"));
    assert!(description.contains("was run live on two synthetic books on 14 October 2026"));
    assert!(!description.contains("has not yet been run on a live Tally"));
    assert!(!description.contains("seen once on a real book"));
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
    assert_eq!(row["parent"], "&#4; Primary");
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
    let one = OneServer::spawn(captured_voucher_type_plans(14, MARK));
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
    assert_eq!(one.requests(), VOUCHER_TYPE_PAGE_REQUESTS);
}

#[tokio::test]
async fn an_unrecognised_numbering_method_is_returned_raw() {
    let unknown = capture("voucher_types").replacen(
        "<NUMBERINGMETHOD TYPE=\"String\">Default</NUMBERINGMETHOD>",
        "<NUMBERINGMETHOD TYPE=\"String\">Serial</NUMBERINGMETHOD>",
        1,
    );
    let numbering = numbering_for(&unknown);
    let one = OneServer::spawn(voucher_type_plans(unknown, numbering, 14, MARK));
    let response = one.call(args("voucher_types", 0, 500, None)).await;
    let rows = result(&response)["masters"].as_array().unwrap().clone();
    assert_eq!(
        rows[0]["numbering_method"],
        json!({"unrecognised":"Serial"})
    );
    assert_eq!(rows[1]["numbering_method"], "default");
    assert_eq!(one.requests(), VOUCHER_TYPE_PAGE_REQUESTS);
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
    // A group directly under Tally's root shows the root as Tally sent it.
    let capital = page["masters"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == "Capital Account")
        .unwrap();
    assert_eq!(capital["parent"], "&#4; Primary");
    assert!(!response.to_string().contains('\u{fffd}'));
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
    assert_eq!(packaging["parent"], "&#4; Primary");
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
    assert_eq!(assembly["parent"], "&#4; Primary");
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
        .all(|row| row["parent"] == json!("&#4; Primary")));

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
            (mask("Parity CC A"), json!("&#4; Primary")),
            (mask("Parity CC A1"), json!(mask("Parity CC A"))),
            (mask("Parity CC B"), json!("&#4; Primary")),
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
    let one = OneServer::spawn(captured_voucher_type_plans(14, 50_000_000));
    let response = one.call(args("voucher_types", 0, 500, None)).await;
    assert_eq!(result(&response)["total"], 26);
    assert_eq!(one.requests(), VOUCHER_TYPE_PAGE_REQUESTS);
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
    plans.extend(captured_voucher_type_plans(14, MARK));
    let one = OneServer::spawn_with(plans, Redaction::MaskParties);

    let response = one.call(args("godowns", 0, 500, None)).await;
    let page = result(&response);
    let masked_root = json!("&#4; Primary");
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
    assert_eq!(
        one.requests(),
        FIRST_PAGE_REQUESTS + VOUCHER_TYPE_PAGE_REQUESTS
    );
}

#[tokio::test]
async fn stock_group_names_and_parents_are_masked_and_unit_names_are_not() {
    let groups = with_last_parent(&capture("stock_groups"), "Packaging");
    let mut plans = first_page_plans(groups, 14);
    plans.extend(first_page_plans(capture("units"), 14));
    let one = OneServer::spawn_with(plans, Redaction::MaskParties);

    let response = one.call(args("stock_groups", 0, 500, None)).await;
    let masked_root = json!("&#4; Primary");
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
            ("Factory Floor".to_string(), json!("&#4; Primary")),
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

/// The tool's numbering of the row named `name`.
fn series_numbering<'a>(rows: &'a [Value], name: &str) -> &'a Value {
    let row = rows.iter().find(|row| row["name"] == name).expect(name);
    &row["series_numbering"]
}

#[tokio::test]
async fn each_voucher_type_row_carries_the_numbering_of_its_own_series() {
    let masters = capture("voucher_types");
    let numbering = numbering_for(&masters);
    // Contra: two series. Credit Note: a series with no method.
    // Debit Note: a method Tally printed outside the measured set.
    // Delivery Note: the override method, seen on a real book but not captured.
    // Memorandum: a duplicate setting that is neither Yes nor No.
    // Sales: edited to the sub-method a type set to renumber prints.
    // Purchase: a series with no sub-method. Stock Journal: a sub-method
    // outside the two measured texts. Payment: Manual, with its sub-method
    // changed, which is not returned.
    let series_of = |name: &str| {
        let at = numbering
            .find(&format!("<VOUCHERTYPE NAME=\"{name}\""))
            .unwrap();
        let from = at + numbering[at..].find("<VOUCHERNUMBERSERIES.LIST>").unwrap();
        let to = from
            + numbering[from..]
                .find("</VOUCHERNUMBERSERIES.LIST>")
                .unwrap()
            + "</VOUCHERNUMBERSERIES.LIST>".len();
        numbering[from..to].to_string()
    };
    let contra = series_of("Contra");
    let numbering = edit_after(
        &numbering,
        "<VOUCHERTYPE NAME=\"Contra\"",
        &contra,
        &format!("{contra}{contra}"),
    );
    let numbering = edit_after(
        &numbering,
        "<VOUCHERTYPE NAME=\"Credit Note\"",
        "<NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD>",
        "",
    );
    let numbering = edit_after(
        &numbering,
        "<VOUCHERTYPE NAME=\"Debit Note\"",
        "<NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD>",
        "<NUMBERINGMETHOD>Serial</NUMBERINGMETHOD>",
    );
    let numbering = edit_after(
        &numbering,
        "<VOUCHERTYPE NAME=\"Delivery Note\"",
        "<NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD>",
        "<NUMBERINGMETHOD>Automatic (Manual Override)</NUMBERINGMETHOD>",
    );
    let numbering = edit_after(
        &numbering,
        "<VOUCHERTYPE NAME=\"Memorandum\"",
        "<PREVENTDUPLICATES>No</PREVENTDUPLICATES>",
        "<PREVENTDUPLICATES>Maybe</PREVENTDUPLICATES>",
    );
    const RETAIN: &str = "<NUMBERINGSUBMETHOD>Auto Retain</NUMBERINGSUBMETHOD>";
    let mut numbering = numbering;
    for (name, to) in [
        (
            "Sales",
            "<NUMBERINGSUBMETHOD>Auto Renumber</NUMBERINGSUBMETHOD>",
        ),
        ("Purchase", ""),
        (
            "Stock Journal",
            "<NUMBERINGSUBMETHOD>Renumber &amp; Retain</NUMBERINGSUBMETHOD>",
        ),
        (
            "Payment",
            "<NUMBERINGSUBMETHOD>Auto Renumber</NUMBERINGSUBMETHOD>",
        ),
    ] {
        numbering = edit_after(
            &numbering,
            &format!("<VOUCHERTYPE NAME=\"{name}\""),
            RETAIN,
            to,
        );
    }
    let one = OneServer::spawn(voucher_type_plans(masters, numbering, 14, MARK));
    let response = one.call(args("voucher_types", 0, 500, None)).await;
    let page = result(&response);
    let rows = page["masters"].as_array().unwrap();
    assert_eq!(rows.len(), 26);
    // The three types the capture's book had set on the screen. Under
    // `automatic` the row says which sub-method the series printed.
    assert_eq!(
        series_numbering(rows, "Journal"),
        &json!({"observation":"observed","method":"automatic","sub_method":"auto_retain","duplicates":"not_prevented"})
    );
    assert_eq!(
        series_numbering(rows, "Sales"),
        &json!({"observation":"observed","method":"automatic","sub_method":"auto_renumber","duplicates":"not_prevented"})
    );
    assert_eq!(
        series_numbering(rows, "Purchase"),
        &json!({"observation":"observed","method":"automatic","sub_method":"absent","duplicates":"not_prevented"})
    );
    // Kept raw, as an unrecognised method is.
    assert_eq!(
        series_numbering(rows, "Stock Journal"),
        &json!({"observation":"observed","method":"automatic","sub_method":{"unrecognised":"Renumber & Retain"},"duplicates":"not_prevented"})
    );
    // No other method carries one: Payment and Receipt below are Manual, and
    // the two rows with another method follow them.
    for name in ["Payment", "Receipt", "Debit Note", "Delivery Note"] {
        assert!(
            series_numbering(rows, name).get("sub_method").is_none(),
            "{name}"
        );
    }
    assert_eq!(
        series_numbering(rows, "Payment"),
        &json!({"observation":"observed","method":"manual","duplicates":"prevented"})
    );
    assert_eq!(
        series_numbering(rows, "Receipt"),
        &json!({"observation":"observed","method":"manual","duplicates":"not_prevented"})
    );
    // A row whose series count is not one: nothing is read from it.
    assert_eq!(
        series_numbering(rows, "Contra"),
        &json!({"observation":"series_count","count":2})
    );
    // A series without a method, and one with an unreadable duplicate setting.
    assert_eq!(
        series_numbering(rows, "Credit Note"),
        &json!({"observation":"field_invalid","code":"series_method"})
    );
    assert_eq!(
        series_numbering(rows, "Memorandum"),
        &json!({"observation":"field_invalid","code":"series_duplicates"})
    );
    // An unrecognised method is kept raw and is never `manual`.
    assert_eq!(
        series_numbering(rows, "Debit Note"),
        &json!({"observation":"observed","method":{"unrecognised":"Serial"},"duplicates":"not_prevented"})
    );
    assert_eq!(
        series_numbering(rows, "Delivery Note"),
        &json!({"observation":"observed","method":"automatic_manual_override","duplicates":"not_prevented"})
    );
    // Every row has one, and the type-level field and its text are unchanged:
    // it still reads `default` on 24 rows, `automatic` on one and `manual` on one.
    assert!(rows.iter().all(|row| row["series_numbering"].is_object()));
    let method = |value: &str| {
        rows.iter()
            .filter(|row| row["numbering_method"] == value)
            .count()
    };
    assert_eq!(
        (method("default"), method("automatic"), method("manual")),
        (24, 1, 1)
    );
    let limitations = page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|line| line.as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(limitations
        .iter()
        .any(|line| line.starts_with("numbering_method is the type-level value")));
    let shipped = |needle: &str| limitations.iter().any(|line| line.contains(needle));
    assert!(shipped(
        "series_numbering is the numbering a voucher type's series prints"
    ));
    assert!(shipped(
        "one synthetic book (TallyPrime 7.1 Silver, Windows)"
    ));
    assert!(shipped("Multi-user Auto"));
    assert!(shipped("a single series not named `Default`"));
    assert!(shipped("a voucher type created new"));
    assert!(shipped("An `unrecognised` series method is never `manual`"));
    assert!(shipped(
        "Under `automatic`, and under no other method, series_numbering carries `sub_method`"
    ));
    assert!(shipped("when one was sent by import without a number"));
    assert!(shipped("the same through ComplyEaze Bridge's own post"));
    assert!(shipped(
        "was run live on 14 October 2026 on two synthetic books"
    ));
    assert!(!shipped("has not yet been run on a live Tally"));
    assert!(shipped("Automatic (Manual Override) through this tool"));
    assert!(!shipped("seen once on a real book"));
    assert_eq!(one.requests(), VOUCHER_TYPE_PAGE_REQUESTS);
}

#[tokio::test]
async fn no_other_kind_carries_a_series_numbering() {
    let one = OneServer::spawn(first_page_plans(capture("godowns"), 14));
    let response = one.call(args("godowns", 0, 500, None)).await;
    let page = result(&response);
    assert!(!page["masters"].as_array().unwrap().is_empty());
    for row in page["masters"].as_array().unwrap() {
        assert!(row.get("series_numbering").is_none(), "{row}");
    }
    assert!(!page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line.as_str().unwrap().contains("series_numbering")));
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn a_numbering_answer_that_does_not_join_is_a_tool_refusal_with_no_rows() {
    let masters = capture("voucher_types");
    let numbering = numbering_for(&masters);
    // Payroll is in the masters answer and not in the numbering answer.
    let at = numbering.find("<VOUCHERTYPE NAME=\"Payroll\"").unwrap();
    let end = at + numbering[at..].find("</VOUCHERTYPE>").unwrap() + "</VOUCHERTYPE>".len();
    let missing = format!("{}{}", &numbering[..at], &numbering[end..]);
    let one = OneServer::spawn(voucher_type_plans_to_closing_extent(
        masters.clone(),
        missing,
        14,
        MARK,
    ));
    let refused = one.call(args("voucher_types", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_numbering_names_differ");
    assert!(refused["structuredContent"]["result"]["masters"].is_null());
    // Held until the closing extent was read, and it had not moved.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4 + 4 + 4);

    // One GUID under another name: an identity difference.
    let renamed = numbering.replacen(
        "<VOUCHERTYPE NAME=\"Payroll\" RESERVEDNAME=\"Payroll\"",
        "<VOUCHERTYPE NAME=\"Payrolls\" RESERVEDNAME=\"Payrolls\"",
        1,
    );
    assert_ne!(renamed, numbering);
    let one = OneServer::spawn(voucher_type_plans_to_closing_extent(
        masters, renamed, 14, MARK,
    ));
    let refused = one.call(args("voucher_types", 0, 500, None)).await;
    assert_eq!(
        error(&refused)["code"],
        "masters_numbering_identity_differs"
    );
    assert!(refused["structuredContent"]["result"]["masters"].is_null());
    assert_eq!(one.requests(), 4 + 3 + 4 + 4 + 4 + 4);
}

#[tokio::test]
async fn a_numbering_answer_that_does_not_parse_is_a_tool_refusal_with_its_cause() {
    let masters = capture("voucher_types");
    let numbering = numbering_for(&masters);
    let at = numbering.find("<VOUCHERTYPE NAME=\"Payment\"").unwrap();
    let from = at + numbering[at..].find("<GUID ").unwrap();
    let to = from + numbering[from..].find("</GUID>").unwrap() + "</GUID>".len();
    let no_guid = format!("{}{}", &numbering[..from], &numbering[to..]);
    let mut plans = through_opening_extent(14, MARK);
    pair(&mut plans, xml(masters));
    pair(&mut plans, xml(no_guid));
    let one = OneServer::spawn(plans);
    let refused = one.call(args("voucher_types", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_read_failed");
    assert_eq!(error(&refused)["cause"], "masters_numbering_guid_missing");
    // Refused at once: no closing extent, identity or mode read.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4 + 4);
}

#[tokio::test]
async fn a_numbering_answer_that_repeats_a_guid_is_a_tool_refusal_with_no_rows() {
    let masters = capture("voucher_types");
    let numbering = numbering_for(&masters);
    // Payment's GUID in Journal's row too: the same GUID on two types.
    let guid_of = |name: &str| {
        let at = numbering
            .find(&format!("<VOUCHERTYPE NAME=\"{name}\""))
            .unwrap();
        let open = at + numbering[at..].find("<GUID ").unwrap();
        let text = open + numbering[open..].find('>').unwrap() + 1;
        numbering[text..text + numbering[text..].find("</GUID>").unwrap()].to_string()
    };
    let repeated = numbering.replacen(&guid_of("Payment"), &guid_of("Journal"), 1);
    assert_ne!(repeated, numbering);
    let mut plans = through_opening_extent(14, MARK);
    pair(&mut plans, xml(masters));
    pair(&mut plans, xml(repeated));
    let one = OneServer::spawn(plans);
    let refused = one.call(args("voucher_types", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_read_failed");
    assert_eq!(error(&refused)["cause"], "masters_row_duplicate_guid");
    assert!(refused["structuredContent"]["result"]["masters"].is_null());
    // Refused at once, before the closing extent.
    assert_eq!(one.requests(), 4 + 3 + 4 + 4 + 4);
}

// Stock items: their own dated HSN and GST lists.

/// A mark inside the stock-item admission (489) and above every captured AlterID (294).
const ITEM_MARK: u64 = 400;
const ITEM: &str = "HSN Two Dates Test Item";

fn item_plans(collection: String) -> Vec<ScenarioPlan> {
    first_page_plans_at(collection, 14, ITEM_MARK)
}

fn item_row<'a>(page: &'a Value, name: &str) -> &'a Value {
    page["masters"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == name)
        .unwrap_or_else(|| panic!("no row {name}"))
}

#[tokio::test]
async fn stock_items_read_end_to_end_with_their_lists_exactly_as_sent() {
    let one = OneServer::spawn(item_plans(capture("stock_items")));
    let response = one.call(args("stock_items", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["kind"], "stock_items");
    assert_eq!(page["total"], 12);
    let item = item_row(page, ITEM);
    assert_eq!(item["parent"], "Raw Chemicals");
    assert_eq!(item["master_id"], 282);
    assert_eq!(item["alter_id"], 294);
    assert_eq!(item["inherited_from_group_or_company"], "not_read");
    assert_eq!(
        item["hsn"],
        json!({"observation": "entries", "entries": [
            {"applicable_from": "20250401", "hsn_code": "28362010",
             "description": "Disodium Carbonate, Dense",
             "source": "Specify Details Here", "other_content_skipped": false},
            {"applicable_from": "20250922", "hsn_code": "28362090",
             "description": "Other",
             "source": "Specify Details Here", "other_content_skipped": false},
        ]})
    );
    let gst = item["gst"]["entries"].as_array().unwrap();
    assert_eq!(item["gst"]["observation"], "entries");
    assert_eq!(gst.len(), 2);
    assert_eq!(gst[0]["applicable_from"], "20250401");
    // Tally's `GSTINELIGIBLEITC`, as sent, under the name of its meaning: Yes on
    // the first entry, absent (null, never false) on the second.
    assert_eq!(gst[0]["ineligible_itc"], true);
    assert_eq!(gst[1]["ineligible_itc"], Value::Null);
    // Null is an emitted key, not a missing one.
    assert!(gst[1].as_object().unwrap().contains_key("ineligible_itc"));
    // Rows a person keyed or edited are not Tally's own entries: nothing is withheld.
    assert!(gst
        .iter()
        .all(|entry| entry.get("ineligible_itc_withheld").is_none()));
    // The key 0.6.0 shipped for the ledger output is not on a stock item's entries.
    assert!(gst.iter().all(|entry| entry.get("itc_eligible").is_none()));
    let igst = |entry: &Value| entry["states"][0]["rates"][2].clone();
    assert_eq!(
        igst(&gst[0]),
        json!({"duty_head": "IGST", "valuation_type": "Based on Value",
               "rate": {"kind": "value", "raw": " 18", "decimal": "18"}})
    );
    assert_eq!(
        igst(&gst[1])["rate"],
        json!({"kind": "value", "raw": " 5", "decimal": "5"})
    );
    // The reserved character arrives as the marker, and a head without a rate is absent.
    assert_eq!(gst[0]["states"][0]["state_name"], "\u{fffd}#4; Any");
    assert_eq!(
        gst[0]["states"][0]["rates"][3],
        json!({"duty_head": "Cess", "valuation_type": "\u{fffd}#4; Not Applicable",
               "rate": {"kind": "absent"}})
    );
    assert_eq!(
        gst[0]["states"][0]["rates"][4]["rate"],
        json!({"kind": "absent"})
    );
    assert_eq!(one.requests(), FIRST_PAGE_REQUESTS);
}

#[tokio::test]
async fn an_item_without_entries_says_that_its_group_and_the_company_were_not_read() {
    let one = OneServer::spawn(item_plans(capture("stock_items_baseline")));
    let response = one.call(args("stock_items", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["total"], 11);
    let rows = page["masters"].as_array().unwrap();
    let empty = rows
        .iter()
        .filter(|row| row["hsn"] == json!({"observation": "entries", "entries": []}))
        .count();
    assert_eq!(empty, 11, "the baseline book has no HSN entry on any item");
    // Not one row can be read as "no HSN": every row names what was not read.
    for row in rows {
        assert_eq!(row["inherited_from_group_or_company"], "not_read", "{row}");
    }
    let limitations = page["limitations"].as_array().unwrap();
    let mentions = |text: &str| {
        limitations
            .iter()
            .any(|line| line.as_str().unwrap().contains(text))
    };
    assert!(mentions(
        "is not a statement that the item has no HSN code or no GST rate"
    ));
    assert!(mentions(
        "what its stock group or the company holds is not read here"
    ));
    assert!(mentions(
        "the group's four other items still carried empty lists"
    ));
    assert!(mentions(
        "Not measured: a GST rate on a group or on the company, an HSN code on the company, and whether an item that defers to a coded group inherits its code"
    ));
    // The earlier general sentences, which covered cases nobody measured, are gone.
    assert!(!mentions("is not on the item's list"));
    assert!(!mentions("its items' lists did not change"));
    assert!(mentions("inherited_from_group_or_company: not_read"));
    assert!(mentions("U+FFFD `#4;`"));
    assert!(mentions(
        "ineligible_itc is Tally's GSTINELIGIBLEITC as sent"
    ));
    assert!(mentions(
        "fits an ineligibility flag (PARTIAL: one book, two rows, screen against wire)"
    ));
    assert!(mentions(
        "What an absent element means for the tax is not known"
    ));
    assert!(mentions(
        "ineligible_itc is null and ineligible_itc_withheld is `tally_default_entry`"
    ));
    assert!(mentions("recognise it by its source, not by that date"));
    assert!(mentions(
        "a flag Tally did not send stays null with no marker, and an entry with any other source is sent as it is"
    ));
    assert!(mentions(
        "under mask_parties it is masked like an item name"
    ));
    assert!(!mentions("returned as sent under every redaction setting"));
    assert!(!mentions("not the screen's setting"));
    assert!(mentions("a zero-row stock-item answer"));
    assert!(mentions(
        "so a limit of 489 refuses, by arithmetic, both client books measured"
    ));
    assert!(!mentions("stock item count"));
    assert!(mentions("past 16,344 UTF-16 units"));
    assert!(mentions("PARTIAL"));
    assert!(mentions(
        "Tally adds an HSN entry and a GST entry of its own"
    ));
    // The mark above which the kind is refused is stated, and is the constant's.
    let mark =
        MASTERS_RESPONSE_BUDGET_BYTES / masters_worst_row_bytes(NativeMasterKind::StockItems);
    assert!(mentions(&format!("master mark is at most {mark}")));
}

#[tokio::test]
async fn the_entries_tally_adds_on_a_first_save_are_returned_as_they_are() {
    let one = OneServer::spawn(item_plans(capture("stock_items_created")));
    let response = one.call(args("stock_items", 0, 500, None)).await;
    let item = item_row(result(&response), ITEM).clone();
    assert_eq!(
        item["hsn"]["entries"],
        json!([{"applicable_from": "20250401", "hsn_code": null, "description": null,
                "source": "As per Company/Stock Group", "other_content_skipped": false}])
    );
    let gst = &item["gst"]["entries"][0];
    assert_eq!(gst["source"], "As per Company/Stock Group");
    assert_eq!(gst["taxability"], Value::Null);
    // The Yes Tally sent on its own entry was never compared with the screen:
    // it is not passed on, and the entry says why.
    assert_eq!(gst["ineligible_itc"], Value::Null);
    assert_eq!(gst["ineligible_itc_withheld"], "tally_default_entry");
    assert!(gst.get("itc_eligible").is_none());
    assert!(gst["states"][0]["rates"]
        .as_array()
        .unwrap()
        .iter()
        .all(|rate| rate["rate"] == json!({"kind": "absent"})));
}

#[tokio::test]
async fn an_absent_flag_on_a_default_entry_stays_null_and_is_not_marked_as_withheld() {
    // The same first-save capture with the credit element taken out of Tally's own entry:
    // nothing was sent, so nothing is withheld.
    let element = "<GSTINELIGIBLEITC TYPE=\"Logical\">Yes</GSTINELIGIBLEITC>\r\n      ";
    let captured = capture("stock_items_created");
    assert_eq!(captured.matches(element).count(), 1);
    let one = OneServer::spawn(item_plans(captured.replace(element, "")));
    let response = one.call(args("stock_items", 0, 500, None)).await;
    let gst = &item_row(result(&response), ITEM)["gst"]["entries"][0];
    assert_eq!(gst["source"], "As per Company/Stock Group");
    assert!(gst.as_object().unwrap().contains_key("ineligible_itc"));
    assert_eq!(gst["ineligible_itc"], Value::Null);
    assert!(gst.get("ineligible_itc_withheld").is_none());
}

#[tokio::test]
async fn no_other_kind_carries_the_stock_item_keys() {
    let mut plans = first_page_plans(capture("stock_groups"), 14);
    plans.extend(first_page_plans(capture("godowns"), 14));
    plans.extend(first_page_plans(capture("units"), 14));
    let one = OneServer::spawn(plans);
    for kind in ["stock_groups", "godowns", "units"] {
        let response = one.call(args(kind, 0, 500, None)).await;
        let rows = result(&response)["masters"].as_array().unwrap().clone();
        assert!(!rows.is_empty(), "{kind}");
        for row in &rows {
            for key in ["hsn", "gst", "inherited_from_group_or_company"] {
                assert!(row.get(key).is_none(), "{kind} {key}");
            }
        }
        if kind == "stock_groups" {
            // A stock group's row is the five fields it always had.
            let keys = rows[0]
                .as_object()
                .unwrap()
                .keys()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(
                keys.into_iter().collect::<Vec<_>>(),
                ["alter_id", "guid", "master_id", "name", "parent"]
            );
        }
    }
}

#[tokio::test]
async fn item_names_parents_and_hsn_descriptions_are_masked_and_codes_and_rates_are_not() {
    let one = OneServer::spawn_with(item_plans(capture("stock_items")), Redaction::MaskParties);
    let response = one.call(args("stock_items", 0, 500, None)).await;
    let page = result(&response);
    let text = response.to_string();
    // Names and parents go out masked: the positive control that masking is on.
    for plain in [ITEM, "Soda Ash Light", "Carton Box Small", "Raw Chemicals"] {
        assert!(!text.contains(plain), "{plain}");
    }
    let masked = item_row(page, &mask(ITEM));
    assert_eq!(masked["parent"], json!(mask("Raw Chemicals")));
    // HSN codes, rates and the state row are not names and stay as sent.
    for kept in ["28362010", "28362090", "\" 18\"", "\" 2.50\""] {
        assert!(text.contains(kept), "{kept}");
    }
    assert_eq!(masked["hsn"]["entries"][0]["hsn_code"], "28362010");
    // An HSN description is text a person can edit, so it is masked like a name.
    assert!(!text.contains("Disodium Carbonate"));
    assert_ne!(
        mask("Disodium Carbonate, Dense"),
        "Disodium Carbonate, Dense"
    );
    assert_eq!(
        masked["hsn"]["entries"][0]["description"],
        json!(mask("Disodium Carbonate, Dense"))
    );
    assert_eq!(
        masked["hsn"]["entries"][1]["description"],
        json!(mask("Other"))
    );
    assert!(page["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|line| line
            .as_str()
            .unwrap()
            .contains("godown, stock-group, stock-item")));
}

#[tokio::test]
async fn an_unreadable_list_is_marked_on_its_item_and_the_read_and_the_other_rows_stand() {
    // The first HSN entry of the item undated: that item's HSN list is unreadable.
    let captured = capture("stock_items");
    let row = captured
        .find(&format!("<STOCKITEM NAME=\"{ITEM}\""))
        .unwrap();
    let date = "<APPLICABLEFROM TYPE=\"Date\">20250401</APPLICABLEFROM>\r\n      <HSNCODE";
    let at = row + captured[row..].find(date).unwrap();
    let changed = format!(
        "{}<HSNCODE{}",
        &captured[..at],
        &captured[at + date.len()..]
    );
    let one = OneServer::spawn(item_plans(changed));
    let response = one.call(args("stock_items", 0, 500, None)).await;
    let page = result(&response);
    assert_eq!(page["total"], 12);
    let item = item_row(page, ITEM);
    assert_eq!(
        item["hsn"],
        json!({"observation": "unreadable", "defect": "entry_without_date"})
    );
    assert_eq!(item["gst"]["observation"], "entries");
    assert_eq!(
        item_row(page, "Soda Ash Light")["gst"]["observation"],
        "entries"
    );
}

#[tokio::test]
async fn a_tally_error_inside_a_list_refuses_the_read_through_the_tool() {
    let captured = capture("stock_items");
    let at = captured.find("<GSTRATE ").unwrap();
    let failed = format!(
        "{}<LINEERROR>x</LINEERROR>{}",
        &captured[..at],
        &captured[at..]
    );
    let mut plans = through_opening_extent(14, ITEM_MARK);
    pair(&mut plans, xml(failed));
    let one = OneServer::spawn(plans);
    let refused = one.call(args("stock_items", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_read_failed");
    assert_eq!(error(&refused)["cause"], "masters_tally_reported_failure");
    assert!(refused["structuredContent"]["result"]["masters"].is_null());
    assert_eq!(one.requests(), REFUSED_AT_ONCE);
}

#[tokio::test]
async fn a_book_over_the_stock_item_mark_is_refused_with_the_mark_it_is_read_at() {
    let largest = u64::try_from(
        MASTERS_RESPONSE_BUDGET_BYTES / masters_worst_row_bytes(NativeMasterKind::StockItems),
    )
    .unwrap();
    assert_eq!(largest, 489);
    let one = OneServer::spawn(through_opening_extent(14, largest + 1));
    let refused = one.call(args("stock_items", 0, 500, None)).await;
    let refusal = error(&refused);
    assert_eq!(refusal["code"], "masters_too_large");
    assert_eq!(refusal["size"]["master_alter_id"], largest + 1);
    assert_eq!(refusal["size"]["limit_master_alter_id"], largest);
    assert_eq!(one.requests(), 4 + 3 + 4);
    // At the mark itself the kind is read.
    let one = OneServer::spawn(first_page_plans_at(capture("stock_items"), 14, largest));
    let response = one.call(args("stock_items", 0, 500, None)).await;
    assert_eq!(result(&response)["total"], 12);
}

#[tokio::test]
async fn a_stock_item_collection_that_does_not_carry_its_type_is_refused_through_the_tool() {
    let untyped = capture("stock_items").replacen(" MSTDEPTYPE=\"512\"", "", 1);
    let mut plans = through_opening_extent(14, ITEM_MARK);
    pair(&mut plans, xml(untyped));
    let one = OneServer::spawn(plans);
    let refused = one.call(args("stock_items", 0, 500, None)).await;
    assert_eq!(error(&refused)["code"], "masters_read_failed");
    assert_eq!(
        error(&refused)["cause"],
        "masters_collection_type_unexpected"
    );
    assert_eq!(one.requests(), REFUSED_AT_ONCE);
}
