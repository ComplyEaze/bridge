//! The tool through `call_tool`, against the simulator replaying the reads of a
//! whole call: the company list (paired) for the identity, the mode probe and
//! identity bracket, the opening extent (paired), the Company collection
//! (paired), the closing extent (paired) and the closing identity and mode
//! checks. The Company collection is the live capture of `BRIDGE SHAPE LAB`
//! with its identity rewritten to the test company's, so it binds. These tests
//! prove the tool glue: the argument, what each setting says it rests on, and
//! what a refusal looks like; the parser's refusals are proven in
//! `native_company_features`.
use super::super::*;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";
const CAPTURE_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";

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

fn extents() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    )
    .to_string()
}

/// The captured Company collection of `BRIDGE SHAPE LAB`, rewritten to the test
/// company ("Bridge Ageing Lab", number 100002, books from 20260401).
fn features() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/company_features_shape_lab_live.utf16le.xml"
    ))
    .replace(CAPTURE_GUID, GUID)
    .replace("BRIDGE SHAPE LAB", "Bridge Ageing Lab")
    .replace("> 100021<", "> 100002<")
    .replace(
        "<BOOKSFROM TYPE=\"Date\">20250401<",
        "<BOOKSFROM TYPE=\"Date\">20260401<",
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

/// The company list for the identity, then the mode probe and the opening
/// identity bracket.
fn plans_to_the_opening_extent() -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = Vec::new();
    pair(&mut plans, companies.clone());
    plans.extend([status(), companies.clone(), companies]);
    plans
}

/// A whole call.
fn plans(features: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = plans_to_the_opening_extent();
    pair(&mut plans, xml(extents()));
    pair(&mut plans, xml(features));
    pair(&mut plans, xml(extents()));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

/// The plans of a call that ends at the Company collection pair: the closing
/// extent pair (four requests) and the closing identity bracket (three) are
/// never sent, and an unsent plan is a hang.
fn plans_ending_at_the_collection(features: String) -> Vec<ScenarioPlan> {
    let mut plans = plans(features);
    plans.truncate(plans.len() - 7);
    plans
}

async fn call(plans: Vec<ScenarioPlan>, args: Value) -> (Value, usize, usize) {
    let expected = plans.len();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let response = server.call_tool("company_features", args).await;
    (response, simulator.finish().unwrap().len(), expected)
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

fn lead(response: &Value) -> String {
    response["structuredContent"]["headline"]["lead"]
        .as_str()
        .expect("a company features read carries a headline")
        .to_string()
}

fn error(response: &Value) -> &Value {
    assert_eq!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]["error"]
}

#[tokio::test]
async fn a_read_returns_three_settings_each_saying_what_it_rests_on_and_the_symbol() {
    let (response, sent, expected) = call(plans(features()), json!({"company_guid": GUID})).await;
    let result = result(&response);
    assert_eq!(sent, expected, "every planned request was served");
    assert_eq!(result["state"], "observed");
    assert_eq!(result["basis"], "tally_company_collection_settings_today");
    assert_eq!(
        result["settings"],
        json!({
            "cost_centres": {"value": "no", "evidence": "compared_with_tally_screen_on_synthetic_books"},
            "gst": {"value": "yes", "evidence": "differed_on_an_uncommitted_book_not_compared_with_screen"},
            "batch_wise": {"value": "yes", "evidence": "differed_across_committed_books_not_compared_with_screen"},
        })
    );
    assert_eq!(
        result["base_currency"],
        json!({"state": "reported", "symbol": "\u{20b9}", "kind": "symbol_not_iso_code"})
    );
    assert_eq!(
        lead(&response),
        "The settings Tally's company record holds today for \u{201c}Bridge Ageing Lab\u{201d}: cost centres off, GST on, batch-wise stock on. They are not what the books contain or what they were during a year, and only the cost-centre setting has been compared with Tally's own screen, on synthetic books. The company's currency symbol is \u{20b9}: a symbol, not a currency code."
    );
    let limitations = result["limitations"].as_array().expect("limitations");
    for needle in [
        "a cost-centre allocation can be stored while the cost-centre setting reads No",
        "which is not the same as no",
        "not an ISO code",
        "are not returned",
        "Nothing this tool returns changes what another tool reads or refuses",
        "zero-width characters in it are not refused",
    ] {
        assert!(
            limitations
                .iter()
                .any(|line| line.as_str().is_some_and(|line| line.contains(needle))),
            "no limitation says {needle:?}: {limitations:?}"
        );
    }
    assert_eq!(response["structuredContent"]["company"]["guid"], GUID);
}

#[tokio::test]
async fn a_setting_tally_did_not_send_makes_the_result_partial_and_is_never_no() {
    let live = features();
    let start = live.find("<ISGSTON ").unwrap();
    let end = start + live[start..].find("</ISGSTON>").unwrap() + "</ISGSTON>".len();
    let without_gst = format!("{}{}", &live[..start], &live[end..]);
    let (response, sent, expected) = call(plans(without_gst), json!({"company_guid": GUID})).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert_eq!(result["state"], "partial");
    assert_eq!(
        response["structuredContent"]["evidence"]["state"],
        "partial"
    );
    assert_eq!(result["settings"]["gst"]["value"], "not_reported");
    assert_eq!(
        result["settings"]["gst"]["evidence"],
        "not_reported_by_tally"
    );
    assert_eq!(
        result["settings"]["cost_centres"]["evidence"],
        "compared_with_tally_screen_on_synthetic_books"
    );
    assert_eq!(result["settings"]["cost_centres"]["value"], "no");
    let lead = lead(&response);
    assert!(lead.contains("GST not sent by Tally"), "{lead}");
    assert!(!lead.contains("GST off"), "{lead}");
}

#[tokio::test]
async fn a_blank_currency_symbol_is_not_reported() {
    let blank = features().replace(
        "<CURRENCYNAME TYPE=\"String\">\u{20b9}</CURRENCYNAME>",
        "<CURRENCYNAME TYPE=\"String\"></CURRENCYNAME>",
    );
    let (response, _, _) = call(plans(blank), json!({"company_guid": GUID})).await;
    let result = result(&response);
    assert_eq!(result["base_currency"], json!({"state": "not_reported"}));
    assert!(lead(&response).ends_with("Tally sent no currency symbol."));
}

#[tokio::test]
async fn a_row_for_another_company_is_refused_with_its_code_and_cause() {
    let other = features().replace("Bridge Ageing Lab", "Bridge Ageing Lab 2");
    let (response, sent, expected) = call(
        plans_ending_at_the_collection(other),
        json!({"company_guid": GUID}),
    )
    .await;
    assert_eq!(sent, expected, "every planned request was served");
    let error = error(&response);
    assert_eq!(error["code"], "company_features_read_failed", "{response}");
    assert_eq!(
        error["cause"], "company_features_company_mismatch",
        "{response}"
    );
}

#[tokio::test]
async fn an_empty_setting_is_refused_naming_it_and_is_never_read_as_no() {
    let empty = features().replace(
        "<ISCOSTCENTRESON TYPE=\"Logical\">No</ISCOSTCENTRESON>",
        "<ISCOSTCENTRESON TYPE=\"Logical\"></ISCOSTCENTRESON>",
    );
    assert_ne!(empty, features());
    let (response, _, _) = call(
        plans_ending_at_the_collection(empty),
        json!({"company_guid": GUID}),
    )
    .await;
    let error = error(&response);
    assert_eq!(error["code"], "company_features_read_failed", "{response}");
    assert_eq!(
        error["cause"], "company_features_setting_invalid:cost_centres",
        "{response}"
    );
}

#[tokio::test]
async fn a_book_that_moved_during_the_read_is_refused_after_the_collection() {
    let moved = extents().replace(
        "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>",
        "<ALTMSTID TYPE=\"Number\"> 225</ALTMSTID>",
    );
    assert_ne!(moved, extents());
    let mut plans = plans_to_the_opening_extent();
    pair(&mut plans, xml(extents()));
    pair(&mut plans, xml(features()));
    pair(&mut plans, xml(moved));
    let (response, sent, expected) = call(plans, json!({"company_guid": GUID})).await;
    assert_eq!(sent, expected, "every planned request was served");
    let error = error(&response);
    assert_eq!(error["code"], "company_features_read_failed", "{response}");
    assert_eq!(
        error["cause"], "company_features_extent_changed",
        "{response}"
    );
}

#[tokio::test]
async fn education_mode_is_refused_with_its_own_code_before_any_collection_request() {
    let education = companies().replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    );
    assert_ne!(education, companies());
    // The identity read, then the status and mode probe, which sees Education.
    let mut plans = Vec::new();
    pair(&mut plans, xml(companies()));
    plans.extend([status(), xml(education)]);
    let (response, sent, expected) = call(plans, json!({"company_guid": GUID})).await;
    assert_eq!(sent, expected, "every planned request was served");
    assert_eq!(
        error(&response)["code"],
        "company_features_education_unqualified",
        "{response}"
    );
}

#[tokio::test]
async fn the_company_is_required_and_no_other_argument_is_admitted() {
    // Admission refuses before any Tally request: the endpoint is never dialled.
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    for (args, code) in [
        (json!({}), "company_guid_required"),
        (
            json!({"company_guid": GUID, "from": "20260401"}),
            "argument_unknown",
        ),
    ] {
        let response = server.call_tool("company_features", args).await;
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"], code,
            "{response}"
        );
    }
}
