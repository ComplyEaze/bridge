//! The two tools through `call_tool`, against the simulator replaying the same
//! reads as `agent_trial_balance_tests`, plus the group tree and Tally's own
//! statements.
//!
//! The Trial Balance and the group tree are captures from two different
//! synthetic companies; the group tree's company GUID is rewritten to the
//! Trial Balance's so the response binds. Tally's statements are synthetic
//! text in the captured shape. These tests prove the tool glue: arguments, the
//! gate's verdict in the payload, and masking. The simulator serves `plans()`
//! in order, so a read the tool skips or reorders fails its response check,
//! and a missing read stalls `finish()`; `sent == expected` holds by
//! construction whenever `finish()` succeeds and proves nothing more. The
//! derivation's figures are proven in `reports::statements`.
use super::super::*;
use super::{top_level, weakest};
use crate::reports::statements::Established;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

const GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";
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

fn extents() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    )
    .to_string()
}

fn groups() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/native/group_snapshot_aarav_with_computed_company_guid.xml"
    )
    .replace(GROUPS_GUID, GUID)
}

fn balance_sheet(current_assets: &str) -> String {
    format!(
        "<ENVELOPE><BSNAME><DSPACCNAME><DSPDISPNAME>Current Assets</DSPDISPNAME></DSPACCNAME></BSNAME><BSAMT><BSSUBAMT></BSSUBAMT><BSMAINAMT>{current_assets}</BSMAINAMT></BSAMT><BSNAME><DSPACCNAME><DSPDISPNAME>Profit &amp; Loss A/c</DSPDISPNAME></DSPACCNAME></BSNAME><BSAMT><BSSUBAMT></BSSUBAMT><BSMAINAMT>11027.00</BSMAINAMT></BSAMT></ENVELOPE>"
    )
}

fn profit_and_loss() -> String {
    profit_and_loss_sales("4027.00")
}

fn profit_and_loss_sales(sales: &str) -> String {
    format!("<ENVELOPE><DSPACCNAME><DSPDISPNAME>Sales Accounts</DSPDISPNAME></DSPACCNAME><PLAMT><PLSUBAMT></PLSUBAMT><BSMAINAMT>{sales}</BSMAINAMT></PLAMT></ENVELOPE>")
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

/// A whole call: identity, then the Trial Balance bracket with the group tree,
/// Tally's Balance Sheet and, for a P&L, Tally's Profit and Loss inside it.
fn plans(current_assets: &str, with_profit_and_loss: bool) -> Vec<ScenarioPlan> {
    plans_with(current_assets, with_profit_and_loss.then(profit_and_loss))
}

fn plans_with(current_assets: &str, profit_and_loss: Option<String>) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let currency = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let report = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_known_lab.xml"
    )
    .to_string();
    let mut plans = Vec::new();
    pair(&mut plans, companies.clone());
    plans.extend([status(), companies.clone(), companies.clone()]);
    pair(&mut plans, xml(extents()));
    pair(&mut plans, xml(currency));
    pair(&mut plans, xml(report));
    pair(&mut plans, xml(groups()));
    pair(&mut plans, xml(balance_sheet(current_assets)));
    if let Some(profit_and_loss) = profit_and_loss {
        pair(&mut plans, xml(profit_and_loss));
    }
    pair(&mut plans, xml(extents()));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

async fn call(tool: &str, plans: Vec<ScenarioPlan>) -> (Value, usize, usize) {
    call_with(tool, plans, Redaction::None).await
}

async fn call_with(
    tool: &str,
    plans: Vec<ScenarioPlan>,
    redaction: Redaction,
) -> (Value, usize, usize) {
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
        redaction,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let response = server
        .call_tool(
            tool,
            json!({"company_guid": GUID, "from": "2026-04-01", "to": "2026-09-02"}),
        )
        .await;
    (response, simulator.finish().unwrap().len(), expected)
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

fn value(result: &Value) -> bridge_tally_core::ExactDecimal {
    assert_eq!(result["state"], "established", "{result}");
    bridge_tally_core::ExactDecimal::parse(result["value"].as_str().unwrap()).unwrap()
}

#[tokio::test]
async fn profit_and_loss_reads_both_statements_and_reports_gated_results() {
    let (response, sent, expected) = call("profit_and_loss", plans("-11027.00", true)).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert!(value(&result["result"]["net_result"])
        .numeric_eq(&bridge_tally_core::ExactDecimal::parse("4027.00").unwrap()));
    assert!(value(&result["result"]["gross_result"])
        .numeric_eq(&bridge_tally_core::ExactDecimal::parse("4027.00").unwrap()));
    let gate = result["balance_sheet_gate"]["lines"].as_array().unwrap();
    assert_eq!(gate.len(), 2);
    assert!(
        gate.iter().all(|line| line["status"] == "matched"),
        "{gate:?}"
    );
    assert_eq!(result["tie_out"]["lines"][0]["status"], "matched");
    assert_eq!(result["lines"].as_array().unwrap().len(), 6);
}

#[tokio::test]
async fn balance_sheet_reads_only_the_balance_sheet_and_reports_the_carried_line() {
    let (response, sent, expected) = call("balance_sheet", plans("-11027.00", false)).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    let carried = &result["result"]["profit_and_loss"]["carried"];
    assert!(value(carried).numeric_eq(&bridge_tally_core::ExactDecimal::parse("11027.00").unwrap()));
    assert!(result["tie_out"].is_null());
    assert_eq!(result["lines"].as_array().unwrap().len(), 9);
}

#[tokio::test]
async fn a_balance_sheet_that_does_not_tie_names_the_line_and_establishes_nothing() {
    let (response, sent, expected) = call("profit_and_loss", plans("-11026.00", true)).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    for figure in ["gross_result", "net_result"] {
        let refused = &result["result"][figure];
        assert_eq!(refused["state"], "not_established", "{refused}");
        assert_eq!(refused["reason"], "tally_balance_sheet_differs");
        assert_eq!(refused["lines"], json!(["Current Assets"]));
    }
    // The derived lines are withheld, not shown as the statement.
    assert!(result["lines"].is_null(), "{result}");
    assert_eq!(
        result["balance_sheet_gate"]["lines"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
}

#[tokio::test]
async fn a_balance_sheet_that_does_not_tie_withholds_its_lines() {
    let (response, sent, expected) = call("balance_sheet", plans("-11026.00", false)).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    let carried = &result["result"]["profit_and_loss"]["carried"];
    assert_eq!(carried["state"], "not_established", "{carried}");
    assert_eq!(carried["reason"], "tally_balance_sheet_differs");
    assert!(result["lines"].is_null(), "{result}");
}

#[tokio::test]
async fn a_masked_refusal_hides_the_lines_it_names() {
    let (response, sent, expected) = call_with(
        "profit_and_loss",
        plans("-11026.00", true),
        Redaction::MaskParties,
    )
    .await;
    assert_eq!(sent, expected);
    let refused = &result(&response)["result"]["net_result"];
    assert_eq!(
        refused["reason"], "tally_balance_sheet_differs",
        "{refused}"
    );
    assert_eq!(refused["lines"].as_array().unwrap().len(), 1);
    assert_ne!(refused["lines"][0], "Current Assets", "{refused}");
}

#[tokio::test]
async fn a_masked_response_hides_the_line_names_a_ledger_can_carry() {
    let (response, sent, expected) = call_with(
        "balance_sheet",
        plans("-11027.00", false),
        Redaction::MaskParties,
    )
    .await;
    assert_eq!(sent, expected);
    let text = result(&response).to_string();
    assert!(
        text.contains("Current Assets"),
        "group names stay readable: {text}"
    );
    assert!(!text.contains("Profit & Loss A/c"), "{text}");
}

/// The top-level `state` and `reason` on the parsed payload (#984): `observed`
/// only when this tool's result is established, otherwise `not_established`
/// with the reason the nested result carries.
fn assert_top_level(result: &Value, state: &str, reason: Option<&str>) {
    assert_eq!(result["state"], state, "{result}");
    match reason {
        Some(reason) => assert_eq!(result["reason"], reason, "{result}"),
        None => assert!(result.get("reason").is_none(), "{result}"),
    }
}

#[tokio::test]
async fn an_established_profit_and_loss_is_observed_with_no_reason() {
    let (response, _, _) = call("profit_and_loss", plans("-11027.00", true)).await;
    let result = result(&response);
    assert_top_level(result, "observed", None);
    assert_eq!(result["result"]["net_result"]["state"], "established");
}

#[tokio::test]
async fn an_established_balance_sheet_is_observed_with_no_reason() {
    let (response, _, _) = call("balance_sheet", plans("-11027.00", false)).await;
    let result = result(&response);
    assert_top_level(result, "observed", None);
    assert_eq!(
        result["result"]["profit_and_loss"]["carried"]["state"],
        "established"
    );
}

#[tokio::test]
async fn a_profit_and_loss_whose_balance_sheet_differs_is_not_established_at_the_top() {
    let (response, _, _) = call("profit_and_loss", plans("-11026.00", true)).await;
    let result = result(&response);
    assert_top_level(
        result,
        "not_established",
        Some("tally_balance_sheet_differs"),
    );
    assert_eq!(result["result"]["net_result"]["reason"], result["reason"]);
}

#[tokio::test]
async fn a_balance_sheet_that_does_not_tie_is_not_established_at_the_top() {
    let (response, _, _) = call("balance_sheet", plans("-11026.00", false)).await;
    let result = result(&response);
    assert_top_level(
        result,
        "not_established",
        Some("tally_balance_sheet_differs"),
    );
    let carried = &result["result"]["profit_and_loss"]["carried"];
    assert_eq!(carried["reason"], result["reason"]);
}

#[tokio::test]
async fn a_profit_and_loss_that_differs_from_tallys_own_is_not_established_at_the_top() {
    let plans = plans_with("-11027.00", Some(profit_and_loss_sales("4028.00")));
    let (response, _, _) = call("profit_and_loss", plans).await;
    let result = result(&response);
    assert_top_level(
        result,
        "not_established",
        Some("tally_profit_and_loss_differs"),
    );
    assert_eq!(result["result"]["net_result"]["reason"], result["reason"]);
    assert!(result["lines"].is_null(), "{result}");
}

/// Every reason the derivation can refuse with, through the same decision the
/// tool uses. The tool-level tests above reach only the two reasons the
/// captured fixtures produce; the other three need a trial balance with a
/// Stock-in-Hand balance, an unclassified ledger or no Profit & Loss ledger,
/// none of which is captured in the tree.
#[test]
fn every_not_established_reason_is_carried_to_the_top() {
    let not_established = |reason| Established::NotEstablished {
        reason,
        lines: Vec::new(),
    };
    for reason in [
        "unclassified_ledger_carries_an_amount",
        "closing_stock_not_derivable_from_trial_balance",
        "profit_and_loss_ledger_not_returned",
        "tally_balance_sheet_differs",
        "tally_profit_and_loss_differs",
    ] {
        assert_eq!(
            top_level(&not_established(reason)),
            ("not_established", Some(reason))
        );
    }
    let established = Established::Established {
        value: bridge_tally_core::ExactDecimal::parse("1.00").unwrap(),
    };
    assert_eq!(top_level(&established), ("observed", None));
}

/// A tool that carries two results reports the weaker one (#984).
#[test]
fn the_weaker_of_two_results_decides_the_top() {
    let established = Established::Established {
        value: bridge_tally_core::ExactDecimal::parse("1.00").unwrap(),
    };
    let refused = Established::NotEstablished {
        reason: "tally_profit_and_loss_differs",
        lines: Vec::new(),
    };
    assert_eq!(
        top_level(weakest(&[&established, &refused])).0,
        "not_established"
    );
    assert_eq!(
        top_level(weakest(&[&refused, &established])).0,
        "not_established"
    );
    assert_eq!(
        top_level(weakest(&[&established, &established])).0,
        "observed"
    );
}
