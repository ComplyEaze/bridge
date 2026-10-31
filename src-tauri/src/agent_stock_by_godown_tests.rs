//! The tool through `call_tool`, against the simulator replaying the reads of
//! `agent_negative_stock_tests` up to the reports (identity, the status and mode
//! probe, the opening identity bracket, the book extent), then Tally's own Godown
//! Summary, the plain Stock Summary and the exploded Stock Summary, each paired,
//! then the closing extent and identity bracket.
//!
//! The reports are the committed live captures of 30 Oct 2026 at 31 March 2026
//! (the `builtin_godown_closing_*_fy_*` fixtures), moved to this harness's company
//! and date only by being served for them: the answers carry neither. These tests
//! prove the tool glue; the ties are proven in `native_godown_summary`.
use super::super::*;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

/// Inside the harness book, which begins on 1 April 2026, and a month's last day.
const AS_OF: &str = "2026-04-30";

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

/// The company's identifier, read from the captured company list by its name,
/// never written out.
fn guid() -> String {
    let list = companies();
    let at = list
        .find("<COMPANY NAME=\"Bridge Ageing Lab\"")
        .expect("the captured list holds the company");
    let tag = "<GUID TYPE=\"String\">";
    let start = at + list[at..].find(tag).expect("with its identifier") + tag.len();
    let end = start + list[start..].find("</GUID>").expect("closed");
    list[start..end].to_string()
}

fn extents() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    )
    .to_string()
}

fn godowns() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_godown_closing_godown_summary_fy_live.utf16le.xml"
    ))
}

fn stock() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_godown_closing_stock_summary_fy_live.utf16le.xml"
    ))
}

fn exploded() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_godown_closing_stock_summary_exploded_fy_live.utf16le.xml"
    ))
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

/// A whole call with the three reports as Tally's answers: identity, the status and mode
/// probe, the opening bracket, the extent pair, the three report pairs, the closing extent
/// pair, the closing bracket.
fn plans(godowns: String, stock: String, exploded: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = Vec::new();
    pair(&mut plans, companies.clone());
    plans.extend([status(), companies.clone(), companies.clone()]);
    pair(&mut plans, xml(extents()));
    for report in [godowns, stock, exploded] {
        pair(&mut plans, xml(report));
    }
    pair(&mut plans, xml(extents()));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

fn captured() -> Vec<ScenarioPlan> {
    plans(godowns(), stock(), exploded())
}

async fn call(
    plans: Vec<ScenarioPlan>,
    as_of: &str,
    redaction: Redaction,
) -> (Value, Vec<tally_protocol_simulator::ObservedRequest>) {
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
        redaction,
        import_enabled: false,
        posting: crate::agent::Posting::Off,
        audit_enabled: false,
        master_writes_enabled: false,
    });
    let response = server
        .call_tool(
            "stock_by_godown",
            json!({"company_guid": guid(), "as_of": as_of}),
        )
        .await;
    (response, simulator.finish().unwrap())
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

fn error(response: &Value) -> &Value {
    assert_eq!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]["error"]
}

#[tokio::test]
async fn tied_godowns_are_returned_with_each_root_item_at_each_godowns_own_cost() {
    let plans = captured();
    let total = plans.len();
    let (response, sent) = call(plans, AS_OF, Redaction::None).await;
    assert_eq!(sent.len(), total, "every planned request served");
    let result = result(&response);
    assert_eq!(result["state"], "tied", "{result}");
    assert_eq!(
        result["tie_out"],
        json!({"godown_total": "2292.01", "stock_summary_total": "2292.01"})
    );
    assert_eq!(
        result["godowns"],
        json!([
            {"godown": "Factory Floor", "quantity": {"state": "empty"}, "value": "-11238.00"},
            {"godown": "Main Location", "quantity": {"state": "empty"}, "value": "13530.01"},
        ])
    );
    assert_eq!(
        result["root_items"][1],
        json!({
            "item": "Godown Split Item",
            "state": "tied",
            "value": "-558.00",
            "quantities": "added_to_the_item_quantity",
            "godowns": [
                {"godown": "Factory Floor", "quantity": {"state": "read", "amount": "8", "unit": "Nos"}, "value": "-88.00"},
                {"godown": "Godown Lab C", "quantity": {"state": "read", "amount": "20", "unit": "Nos"}, "value": "-250.00"},
                {"godown": "Main Location", "quantity": {"state": "read", "amount": "22", "unit": "Nos"}, "value": "-220.00"},
            ],
        })
    );
    assert_eq!(result["root_items"].as_array().unwrap().len(), 2);
    assert_eq!(
        result["checks"]["items_inside_stock_groups"],
        "not_available"
    );
    assert!(result.get("reason").is_none(), "{result}");
}

#[tokio::test]
async fn godown_totals_that_differ_return_both_totals_and_no_godown_or_item() {
    let differs = godowns().replacen("13530.01", "13530.02", 1);
    assert_ne!(differs, godowns(), "the target was not found");
    let (response, _) = call(plans(differs, stock(), exploded()), AS_OF, Redaction::None).await;
    let result = result(&response);
    assert_eq!(result["state"], "not_established", "{result}");
    assert_eq!(result["reason"], "godown_totals_differ");
    assert_eq!(
        result["tie_out"],
        json!({"godown_total": "2292.02", "stock_summary_total": "2292.01"})
    );
    assert_eq!(result["godowns"], Value::Null);
    assert_eq!(result["root_items"], Value::Null);
}

#[tokio::test]
async fn an_empty_exploded_answer_returns_godowns_and_says_no_item_was_established() {
    let (response, _) = call(
        plans(godowns(), stock(), "<ENVELOPE></ENVELOPE>".to_string()),
        AS_OF,
        Redaction::None,
    )
    .await;
    let result = result(&response);
    assert_eq!(result["state"], "tied", "{result}");
    assert_eq!(result["root_items"], Value::Null);
    assert_eq!(
        result["root_items_reason"],
        "exploded_stock_summary_empty_is_not_a_finding"
    );
    assert_eq!(result["godowns"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn the_three_requests_on_the_wire_are_this_companys_and_periods_each_read_twice() {
    use bridge_tally_protocol::{
        native_godown_summary::{
            render_native_godown_summary_request, render_native_stock_summary_exploded_request,
        },
        native_outstandings::NativeLedgerSnapshotPeriod,
        native_stock_summary::render_native_stock_summary_request,
        outstandings_shared::DateBoundaryProfile,
    };
    use sha2::{Digest, Sha256};
    let (response, observed) = call(captured(), AS_OF, Redaction::None).await;
    assert_ne!(response["isError"], true, "{response}");
    let period = NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        bridge_tally_core::TallyDate::parse("20260401").unwrap(),
        bridge_tally_core::TallyDate::parse("20260430").unwrap(),
    )
    .unwrap();
    for request in [
        render_native_godown_summary_request("Bridge Ageing Lab", &period),
        render_native_stock_summary_request("Bridge Ageing Lab", &period),
        render_native_stock_summary_exploded_request("Bridge Ageing Lab", &period),
    ] {
        let mut body = vec![0xFF, 0xFE];
        body.extend(request.encode_utf16().flat_map(u16::to_le_bytes));
        let sha: String = Sha256::digest(&body)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let sent = observed
            .iter()
            .filter(|observed| observed.request_body_sha256 == sha)
            .count();
        assert_eq!(sent, 2, "{request}");
    }
}

#[tokio::test]
async fn a_date_that_is_not_a_month_end_is_refused_before_any_request() {
    // Refused before Tally is reached at all: the endpoint is a port nothing listens on.
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 1,
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        posting: crate::agent::Posting::Off,
        audit_enabled: false,
        master_writes_enabled: false,
    });
    let response = server
        .call_tool(
            "stock_by_godown",
            json!({"company_guid": guid(), "as_of": "2026-04-29"}),
        )
        .await;
    let error = error(&response);
    assert_eq!(
        error["code"], "stock_by_godown_as_of_not_measured",
        "{response}"
    );
    assert!(error["remediation"]
        .as_str()
        .unwrap()
        .contains("last day of a month"));
}

#[tokio::test]
async fn an_exploded_answer_that_changes_between_its_reads_is_refused_as_its_own_drift() {
    let mut plans = captured();
    // The exploded pair sits before the closing extent pair (four) and the closing
    // identity bracket (three); its second answer is two before its end.
    let second = plans.len() - 7 - 2;
    plans[second] = xml(exploded().replacen("-220.00", "-221.00", 1));
    plans.truncate(plans.len() - 7);
    let (response, _) = call(plans, AS_OF, Redaction::None).await;
    let error = error(&response);
    assert_eq!(error["code"], "stock_by_godown_read_failed", "{response}");
    assert_eq!(error["cause"], "native_stock_summary_exploded_changed");
}

#[tokio::test]
async fn a_book_that_moves_during_the_read_is_reported_as_moved() {
    let mut plans = captured();
    plans.truncate(plans.len() - 3);
    let closing_first = plans.len() - 4;
    let moved = extents().replace(
        "<ALTVCHID TYPE=\"Number\"> ",
        "<ALTVCHID TYPE=\"Number\"> 9",
    );
    assert_ne!(moved, extents(), "the extent was not changed");
    plans[closing_first] = xml(moved.clone());
    plans[closing_first + 2] = xml(moved);
    let (response, _) = call(plans, AS_OF, Redaction::None).await;
    assert_eq!(error(&response)["cause"], "stock_by_godown_extent_changed");
}

#[tokio::test]
async fn under_mask_parties_godown_and_item_names_are_masked_and_figures_are_not() {
    let (plain, _) = call(captured(), AS_OF, Redaction::None).await;
    let (masked, _) = call(captured(), AS_OF, Redaction::MaskParties).await;
    let (plain, masked) = (result(&plain), result(&masked));
    for (plain, masked) in plain["godowns"]
        .as_array()
        .unwrap()
        .iter()
        .zip(masked["godowns"].as_array().unwrap())
    {
        let name = plain["godown"].as_str().unwrap();
        assert_eq!(masked["godown"], json!(crate::agent::mask(name)));
        assert_eq!(masked["value"], plain["value"]);
    }
    let item = &masked["root_items"][1];
    assert_eq!(item["item"], json!(crate::agent::mask("Godown Split Item")));
    assert_eq!(
        item["godowns"][1]["godown"],
        json!(crate::agent::mask("Godown Lab C"))
    );
    assert_eq!(item["value"], "-558.00");
    let text = masked.to_string();
    for name in [
        "Factory Floor",
        "Main Location",
        "Godown Lab C",
        "Godown Split Item",
    ] {
        assert!(!text.contains(name), "{name}");
    }
}

#[tokio::test]
async fn an_exploded_answer_that_is_the_plain_report_says_the_flag_was_not_observed() {
    let (response, _) = call(plans(godowns(), stock(), stock()), AS_OF, Redaction::None).await;
    let result = result(&response);
    assert_eq!(result["state"], "tied", "{result}");
    assert_eq!(result["root_items"], Value::Null);
    assert_eq!(
        result["root_items_reason"],
        "explode_flag_not_observed_is_not_a_finding"
    );
}
