//! The `stock_by_batch` tool through the MCP adapter, replayed from the 30 October sitting's
//! year-end collection (`stock-query/c-a-03`) and the committed year-end Stock Query answers that
//! sitting repeated byte for byte, with the book's GUID replaced by the test double's company.
//! The runtime read's own faults are `runtime_stock_by_batch_tests`.
use super::super::*;
use tally_protocol_simulator::{
    Delivery, Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator,
    WireEncoding,
};

const GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const AS_OF: &str = "20260331";
/// The captured company row and extent start the books a year later than the captures' period;
/// both are moved back together, as the identity bracket compares them.
const BOOKS_FROM: &str = "20250401";
const HCL: &str = "0000010d";
const CAUSTIC_SODA: &str = "0000010c";
const CLEANING_KIT_A: &str = "00000112";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn fixture(name: &str) -> String {
    decode(
        &std::fs::read(format!(
            "{}/crates/bridge-tally-protocol/tests/fixtures/{name}",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("a committed capture"),
    )
}

/// The company's own row in `text`, with its `BOOKSFROM` moved to [`BOOKS_FROM`].
fn with_books_from(text: &str) -> String {
    let from = "<BOOKSFROM TYPE=\"Date\">20260401</BOOKSFROM>";
    let at = text.find(GUID).unwrap();
    let start = text[..at].rfind("<COMPANY ").unwrap();
    let end = at + text[at..].find("</COMPANY>").unwrap();
    assert_eq!(text[start..end].matches(from).count(), 1);
    format!(
        "{}{}{}",
        &text[..start],
        text[start..end].replace(
            from,
            &format!("<BOOKSFROM TYPE=\"Date\">{BOOKS_FROM}</BOOKSFROM>")
        ),
        &text[end..]
    )
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

fn companies() -> ScenarioPlan {
    xml(with_books_from(&fixture(
        "agent/native-licensed-release-companies.utf16le.xml",
    )))
}

fn extents() -> ScenarioPlan {
    extents_at(224)
}

/// The captured extents with this company's master mark at `mark` (captured: 224).
fn extents_at(mark: u64) -> ScenarioPlan {
    let captured = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    );
    let element = "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>";
    assert_eq!(captured.matches(element).count(), 1);
    xml(with_books_from(&captured.replace(
        element,
        &format!("<ALTMSTID TYPE=\"Number\"> {mark}</ALTMSTID>"),
    )))
}

/// The tool's identity read, then the runtime read up to its first Stock Query, with `flags`
/// as the company's inventory flags.
fn through_items(flags: String) -> Vec<ScenarioPlan> {
    let mut plans = Vec::new();
    pair(&mut plans, companies());
    plans.extend([status(), companies(), companies()]);
    pair(&mut plans, extents());
    pair(&mut plans, xml(flags));
    pair(
        &mut plans,
        xml(
            fixture("stock-query/c-a-03-items-to-20260331.answer.utf16le.xml")
                .replace(CAPTURE_GUID, GUID),
        ),
    );
    plans
}

fn flags() -> String {
    fixture("company_inventory_flags_shape_lab_live.utf16le.xml").replace(CAPTURE_GUID, GUID)
}

fn answer(name: &str) -> ScenarioPlan {
    xml(fixture(&format!("stock-query/{name}.answer.utf16le.xml")))
}

fn closing(plans: &mut Vec<ScenarioPlan>) {
    pair(plans, extents());
    plans.extend([companies(), status(), companies()]);
}

fn server(
    plans: Vec<ScenarioPlan>,
    redaction: Redaction,
) -> (Server, SequenceSimulator, tempfile::TempDir) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let (server, directory) = server_at(simulator.address().port(), redaction);
    (server, simulator, directory)
}

/// A server on `port`. On port 9 no Tally is there: a request sent would fail as a refused
/// connection, not as the refusal a test looks for.
fn server_at(port: u16, redaction: Redaction) -> (Server, tempfile::TempDir) {
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
    (server, directory)
}

fn item(suffix: &str) -> String {
    format!("{GUID}-{suffix}")
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

#[tokio::test]
async fn godown_batch_and_item_names_are_masked_under_mask_parties_and_plain_without_it() {
    for redaction in [Redaction::None, Redaction::MaskParties] {
        let masked = matches!(redaction, Redaction::MaskParties);
        let mut plans = through_items(flags());
        plans.push(answer("r-4-stock-query-stockitemname"));
        closing(&mut plans);
        let (server, simulator, _directory) = server(plans, redaction);
        let response = server
            .call_tool(
                "stock_by_batch",
                json!({"company_guid": GUID, "as_of": AS_OF, "items": [item(HCL)]}),
            )
            .await;
        let page = result(&response);
        let text = response.to_string();
        let row = &page["items"][0];
        assert_eq!(row["state"], "tied");
        assert_eq!(row["guid"], item(HCL));
        assert_eq!(
            row["lines_total"],
            json!({"amount": "816.667", "unit": "Kgs"})
        );
        assert_eq!(
            row["closing"],
            json!({"state": "read", "amount": "816.667", "unit": "Kgs"})
        );
        // Tally's own default batch is fixed text and is never masked.
        assert_eq!(
            row["lines"][0]["batch"],
            json!({"kind": "default", "name": "Primary Batch"})
        );
        for (plain, shown) in [
            ("Hydrochloric Acid 33pc", &row["name"]),
            ("Main Location", &row["lines"][0]["godown"]),
            ("Factory Floor", &row["lines"][2]["godown"]),
            ("H-2504-A", &row["lines"][1]["batch"]["name"]),
        ] {
            let expected = if masked {
                mask(plain)
            } else {
                plain.to_string()
            };
            assert_eq!(shown, &json!(expected), "{plain}");
            assert_eq!(text.contains(plain), !masked, "{plain}");
        }
        // Godown totals are keyed by the lines' positions and carry no name of their own.
        assert_eq!(
            page["items"][0]["godowns"],
            json!([
                {"lines": [0, 1], "quantity": {"amount": "450.000", "unit": "Kgs"}, "source": "stock_query_lines_added"},
                {"lines": [2], "quantity": {"amount": "366.667", "unit": "Kgs"}, "source": "stock_query_lines_added"},
            ])
        );
        assert_eq!(page["state"], "complete");
        assert_eq!(page["batch_wise"], "yes");
        let lead = response["structuredContent"]["headline"]["lead"]
            .as_str()
            .unwrap();
        assert!(lead.contains("1 tie to the closing quantity"), "{lead}");
        assert!(!lead.contains("Hydrochloric"), "{lead}");
        assert_eq!(simulator.finish().unwrap().len(), 4 + 3 + 4 + 4 + 4 + 1 + 7);
    }
}

#[tokio::test]
async fn a_failed_send_is_a_partial_answer_that_says_so_first() {
    let mut plans = through_items(flags());
    plans.extend([
        answer("r-02-stock-query-caustic-soda-flakes"),
        answer("r-03-stock-query-cleaning-kit-a").with_delivery(Delivery::ResetBeforeBody),
    ]);
    let (server, simulator, _directory) = server(plans, Redaction::None);
    let response = server
        .call_tool(
            "stock_by_batch",
            json!({"company_guid": GUID, "as_of": AS_OF, "items": [item(CAUSTIC_SODA), item(CLEANING_KIT_A), item(HCL)]}),
        )
        .await;
    let page = result(&response);
    assert_eq!(page["state"], "partial");
    assert!(page["stopped"]["code"].is_string());
    assert_eq!(page["verification"], "stopped_before_the_closing_reads");
    let states = page["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["state"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(states, ["tied", "not_read", "not_sent"]);
    let lead = response["structuredContent"]["headline"]["lead"]
        .as_str()
        .unwrap();
    assert!(lead.contains("Partial: a send to Tally failed"), "{lead}");
    assert_eq!(simulator.finish().unwrap().len(), 4 + 3 + 4 + 4 + 4 + 2);
}

#[tokio::test]
async fn batch_wise_off_still_answers_per_godown_and_says_so() {
    let off = flags().replace(
        "<ISBATCHWISEON TYPE=\"Logical\">Yes</ISBATCHWISEON>",
        "<ISBATCHWISEON TYPE=\"Logical\">No</ISBATCHWISEON>",
    );
    assert_ne!(off, flags(), "the capture holds the batch-wise flag");
    let mut plans = through_items(off);
    plans.push(answer("r-02-stock-query-caustic-soda-flakes"));
    closing(&mut plans);
    let (server, simulator, _directory) = server(plans, Redaction::None);
    let response = server
        .call_tool(
            "stock_by_batch",
            json!({"company_guid": GUID, "as_of": AS_OF, "items": [item(CAUSTIC_SODA)]}),
        )
        .await;
    let page = result(&response);
    assert_eq!(page["batch_wise"], "no");
    assert_eq!(page["items"][0]["state"], "tied");
    let lead = response["structuredContent"]["headline"]["lead"]
        .as_str()
        .unwrap();
    assert!(lead.contains("Batch-wise stock is off"), "{lead}");
    simulator.finish().unwrap();
}

#[tokio::test]
async fn a_list_or_date_that_cannot_be_honoured_is_refused_before_any_read() {
    let too_many = (0..=super::MAX_STOCK_BY_BATCH_ITEMS)
        .map(|at| item(&format!("{at:08x}")))
        .collect::<Vec<_>>();
    for (args, code) in [
        (
            json!({"company_guid": GUID, "as_of": AS_OF}),
            "items_required",
        ),
        (
            json!({"company_guid": GUID, "as_of": AS_OF, "items": []}),
            "argument_invalid:items",
        ),
        (
            json!({"company_guid": GUID, "as_of": AS_OF, "items": too_many}),
            "argument_invalid:items",
        ),
        (
            json!({"company_guid": GUID, "as_of": AS_OF, "items": [item(HCL), item(HCL).to_uppercase()]}),
            "argument_invalid:items",
        ),
        (
            json!({"company_guid": GUID, "as_of": "20260330", "items": [item(HCL)]}),
            "stock_by_batch_as_of_not_measured",
        ),
    ] {
        let (server, _directory) = server_at(9, Redaction::None);
        let response = server.call_tool("stock_by_batch", args.clone()).await;
        assert_eq!(response["isError"], true, "{args}");
        // Port 9 has no Tally: a request sent would have failed as a connection, not this.
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"], code,
            "{args}"
        );
    }
}

/// A refusal of the stock-item admission it shares with stock_summary reaches the person under
/// this tool's own code, with its cause and remediation, and nowhere names stock_summary. A mark
/// of 5 admits the read, and the book's 11 rows break it.
#[tokio::test]
async fn a_shared_refusal_is_shown_under_this_tools_name() {
    let mut plans = Vec::new();
    pair(&mut plans, companies());
    plans.extend([status(), companies(), companies()]);
    pair(&mut plans, extents_at(5));
    pair(&mut plans, xml(flags()));
    pair(
        &mut plans,
        xml(
            fixture("stock-query/c-a-03-items-to-20260331.answer.utf16le.xml")
                .replace(CAPTURE_GUID, GUID),
        ),
    );
    pair(&mut plans, extents_at(5));
    let (server, simulator, _directory) = server(plans, Redaction::None);
    let response = server
        .call_tool(
            "stock_by_batch",
            json!({"company_guid": GUID, "as_of": AS_OF, "items": [item(HCL)]}),
        )
        .await;
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "stock_by_batch_bound_premise_violated");
    assert_eq!(error["cause"], "stock_rows_exceed_master_mark");
    assert!(
        !response.to_string().contains("stock_summary"),
        "{response}"
    );
    assert_eq!(simulator.finish().unwrap().len(), 4 + 3 + 4 + 4 + 4 + 4);
}

/// The published bound on `items` is the code's, and so is the answer limit the description
/// states.
#[test]
fn the_schema_and_description_state_the_codes_bounds() {
    let tools = super::super::catalog::registered_tool_definitions(false, false);
    let tool = tools
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "stock_by_batch")
        .unwrap();
    assert_eq!(
        tool["inputSchema"]["properties"]["items"]["maxItems"],
        super::MAX_STOCK_BY_BATCH_ITEMS
    );
    let limit = crate::tally::runtime::STOCK_QUERY_ANSWER_MAX_BYTES;
    let shown = limit
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(",");
    assert!(
        tool["description"]
            .as_str()
            .unwrap()
            .contains(&format!("over {shown} bytes")),
        "{shown}"
    );
}
