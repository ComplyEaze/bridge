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
use crate::reports::statements::{Established, NotEstablishedReason};
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
/// Tally's Balance Sheet and Tally's Profit and Loss inside it (both tools read
/// both: the Profit and Loss is where Tally's closing stock sits).
fn plans(current_assets: &str) -> Vec<ScenarioPlan> {
    plans_with(current_assets, profit_and_loss())
}

fn plans_with(current_assets: &str, profit_and_loss: String) -> Vec<ScenarioPlan> {
    plans_full(balance_sheet(current_assets), profit_and_loss, None)
}

/// The same call with the statements given as text, and the company's inventory
/// flags read after the Profit and Loss when that statement offers a closing stock.
fn plans_full(
    balance_sheet: String,
    profit_and_loss: String,
    inventory_flags: Option<String>,
) -> Vec<ScenarioPlan> {
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
    pair(&mut plans, xml(balance_sheet));
    pair(&mut plans, xml(profit_and_loss));
    if let Some(flags) = inventory_flags {
        pair(&mut plans, xml(flags));
    }
    pair(&mut plans, xml(extents()));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

/// The headline's lead, from the structured content of a response.
fn lead(response: &Value) -> String {
    response["structuredContent"]["headline"]["lead"]
        .as_str()
        .expect("a statement carries a headline")
        .to_string()
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
        posting: Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
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
    let (response, sent, expected) = call("profit_and_loss", plans("-11027.00")).await;
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
    // In words, ahead of the figures: established, the period, and what it ties to.
    let lead = lead(&response);
    assert!(lead.starts_with("Profit and loss for \u{201c}"), "{lead}");
    assert!(
        lead.contains("the gross result and the net result are established"),
        "{lead}"
    );
    assert!(
        lead.contains("passed the comparison with Tally's own Balance Sheet and Profit and Loss"),
        "{lead}"
    );
    assert!(!lead.contains("Not established"), "{lead}");
}

#[tokio::test]
async fn balance_sheet_reads_both_statements_and_reports_the_carried_line() {
    let (response, sent, expected) = call("balance_sheet", plans("-11027.00")).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    let carried = &result["result"]["profit_and_loss"]["carried"];
    assert!(value(carried).numeric_eq(&bridge_tally_core::ExactDecimal::parse("11027.00").unwrap()));
    assert!(result["tie_out"].is_null());
    assert_eq!(result["lines"].as_array().unwrap().len(), 9);
    // Tally's Profit and Loss was read, but no closing stock was offered, so no
    // comparison with it is claimed or shown.
    let lead = lead(&response);
    assert!(lead.starts_with("Balance sheet for \u{201c}"), "{lead}");
    assert!(
        lead.contains("passed the comparison with Tally's own Balance Sheet."),
        "{lead}"
    );
    assert!(!lead.contains("Profit and Loss."), "{lead}");
}

#[tokio::test]
async fn a_balance_sheet_that_does_not_tie_names_the_line_and_establishes_nothing() {
    let (response, sent, expected) = call("profit_and_loss", plans("-11026.00")).await;
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
    // The headline leads with it, for both results, with the count of lines.
    let lead = lead(&response);
    assert!(
        lead.starts_with("Not established: the profit and loss for \u{201c}"),
        "{lead}"
    );
    assert!(lead.contains("The gross result is not established because Tally's own Balance Sheet differs from the derived lines (on 1 line)"), "{lead}");
    assert!(lead.contains("The net result is not established"), "{lead}");
    assert!(lead.contains("The derived lines are withheld"), "{lead}");
    assert!(!lead.contains("established;"), "{lead}");
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
    let (response, sent, expected) = call("balance_sheet", plans("-11026.00")).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    let carried = &result["result"]["profit_and_loss"]["carried"];
    assert_eq!(carried["state"], "not_established", "{carried}");
    assert_eq!(carried["reason"], "tally_balance_sheet_differs");
    assert!(result["lines"].is_null(), "{result}");
    let lead = lead(&response);
    assert!(
        lead.starts_with("Not established: the balance sheet for \u{201c}"),
        "{lead}"
    );
    assert!(
        lead.contains("The profit and loss line of the balance sheet is not established"),
        "{lead}"
    );
}

#[tokio::test]
async fn a_masked_refusal_hides_the_lines_it_names() {
    let (response, sent, expected) = call_with(
        "profit_and_loss",
        plans("-11026.00"),
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
    let (response, sent, expected) =
        call_with("balance_sheet", plans("-11027.00"), Redaction::MaskParties).await;
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
    let (response, _, _) = call("profit_and_loss", plans("-11027.00")).await;
    let result = result(&response);
    assert_top_level(result, "observed", None);
    assert_eq!(result["result"]["net_result"]["state"], "established");
}

#[tokio::test]
async fn an_established_balance_sheet_is_observed_with_no_reason() {
    let (response, _, _) = call("balance_sheet", plans("-11027.00")).await;
    let result = result(&response);
    assert_top_level(result, "observed", None);
    assert_eq!(
        result["result"]["profit_and_loss"]["carried"]["state"],
        "established"
    );
}

#[tokio::test]
async fn a_profit_and_loss_whose_balance_sheet_differs_is_not_established_at_the_top() {
    let (response, _, _) = call("profit_and_loss", plans("-11026.00")).await;
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
    let (response, _, _) = call("balance_sheet", plans("-11026.00")).await;
    let result = result(&response);
    assert_top_level(
        result,
        "not_established",
        Some("tally_balance_sheet_differs"),
    );
    let carried = &result["result"]["profit_and_loss"]["carried"];
    assert_eq!(carried["reason"], result["reason"]);
    // The synthetic statement's two sides differ by one: the tool says so, in the
    // result and in its first line (#2063).
    assert_eq!(
        result["tally_balance_sheet_sides"],
        json!({"credits": "11027", "debits": "11026"})
    );
    let lead = lead(&response);
    assert!(
        lead.contains("credit lines add to 11027 and its debit lines to 11026"),
        "{lead}"
    );
}

/// A statement whose sides agree carries no such key.
#[tokio::test]
async fn a_balance_sheet_that_balances_carries_no_sides() {
    let (response, _, _) = call("balance_sheet", plans("-11027.00")).await;
    assert!(
        result(&response).get("tally_balance_sheet_sides").is_none(),
        "{response}"
    );
}

#[tokio::test]
async fn a_profit_and_loss_that_differs_from_tallys_own_is_not_established_at_the_top() {
    let plans = plans_with("-11027.00", profit_and_loss_sales("4028.00"));
    let (response, _, _) = call("profit_and_loss", plans).await;
    let result = result(&response);
    assert_top_level(
        result,
        "not_established",
        Some("tally_profit_and_loss_differs"),
    );
    assert_eq!(result["result"]["net_result"]["reason"], result["reason"]);
    assert!(result["lines"].is_null(), "{result}");
    let lead = lead(&response);
    assert!(
        lead.contains("Tally's own Profit and Loss differs from the derived lines"),
        "{lead}"
    );
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
        NotEstablishedReason::UnclassifiedLedgerCarriesAnAmount,
        NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance,
        NotEstablishedReason::ProfitAndLossLedgerNotReturned,
        NotEstablishedReason::TallyBalanceSheetDiffers,
        NotEstablishedReason::TallyProfitAndLossDiffers,
        NotEstablishedReason::TallyBalanceSheetDoesNotBalance,
    ] {
        assert_eq!(
            top_level(&not_established(reason)),
            ("not_established", Some(reason.as_str()))
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
        reason: NotEstablishedReason::TallyProfitAndLossDiffers,
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

/// The codes a caller reads are the ones the result has always carried, by
/// both paths that write them (`as_str` and the derived serde form).
#[test]
fn the_not_established_codes_are_unchanged() {
    for (reason, code) in [
        (
            NotEstablishedReason::UnclassifiedLedgerCarriesAnAmount,
            "unclassified_ledger_carries_an_amount",
        ),
        (
            NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance,
            "closing_stock_not_derivable_from_trial_balance",
        ),
        (
            NotEstablishedReason::ProfitAndLossLedgerNotReturned,
            "profit_and_loss_ledger_not_returned",
        ),
        (
            NotEstablishedReason::TallyBalanceSheetDiffers,
            "tally_balance_sheet_differs",
        ),
        (
            NotEstablishedReason::TallyProfitAndLossDiffers,
            "tally_profit_and_loss_differs",
        ),
    ] {
        assert_eq!(reason.as_str(), code);
        assert_eq!(serde_json::to_value(reason).unwrap(), json!(code));
    }
}

// Tally's own closing stock through the tools (#692). The sources are the known
// lab's, as above, with a Profit and Loss and a Balance Sheet in the captured
// stock-book shape that move Current Assets, the carried line and the trading
// result by one closing stock of 500; the company's flags are the captured
// answer of the inventory book with its GUID rewritten to this one's.

fn stock_profit_and_loss() -> String {
    "<ENVELOPE><DSPACCNAME><DSPDISPNAME>Sales Accounts</DSPDISPNAME></DSPACCNAME><PLAMT><PLSUBAMT></PLSUBAMT><BSMAINAMT>4027.00</BSMAINAMT></PLAMT><DSPACCNAME><DSPDISPNAME>Cost of Sales :</DSPDISPNAME></DSPACCNAME><PLAMT><PLSUBAMT></PLSUBAMT><BSMAINAMT>500.00</BSMAINAMT></PLAMT><DSPACCNAME><DSPDISPNAME>Opening Stock</DSPDISPNAME></DSPACCNAME><PLAMT><PLSUBAMT></PLSUBAMT><BSMAINAMT></BSMAINAMT></PLAMT><DSPACCNAME><DSPDISPNAME>Less: Closing Stock</DSPDISPNAME></DSPACCNAME><PLAMT><PLSUBAMT>-500.00</PLSUBAMT><BSMAINAMT></BSMAINAMT></PLAMT></ENVELOPE>".to_string()
}

fn stock_balance_sheet() -> String {
    "<ENVELOPE><BSNAME><DSPACCNAME><DSPDISPNAME>Current Assets</DSPDISPNAME></DSPACCNAME></BSNAME><BSAMT><BSSUBAMT></BSSUBAMT><BSMAINAMT>-11527.00</BSMAINAMT></BSAMT><BSNAME><DSPACCNAME><DSPDISPNAME>Profit &amp; Loss A/c</DSPDISPNAME></DSPACCNAME></BSNAME><BSAMT><BSSUBAMT></BSSUBAMT><BSMAINAMT>11527.00</BSMAINAMT></BSAMT></ENVELOPE>".to_string()
}

fn inventory_flags(integrated: &str) -> String {
    let answer = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_stock_book_live.utf16le.xml"
    ))
    .replace("d64eda55-da7c-4106-a171-3e3cb6cdd331", GUID);
    assert!(answer.contains("<ISINTEGRATED TYPE=\"Logical\">Yes</ISINTEGRATED>"));
    answer.replace(
        "<ISINTEGRATED TYPE=\"Logical\">Yes</ISINTEGRATED>",
        &format!("<ISINTEGRATED TYPE=\"Logical\">{integrated}</ISINTEGRATED>"),
    )
}

fn stock_plans(flags: String) -> Vec<ScenarioPlan> {
    plans_full(stock_balance_sheet(), stock_profit_and_loss(), Some(flags))
}

#[tokio::test]
async fn profit_and_loss_uses_tallys_closing_stock_and_names_it() {
    let plans = stock_plans(inventory_flags("Yes"));
    let (response, sent, expected) = call("profit_and_loss", plans).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert!(value(&result["result"]["net_result"])
        .numeric_eq(&bridge_tally_core::ExactDecimal::parse("4527.00").unwrap()));
    assert_eq!(result["closing_stock"]["state"], "applied");
    assert_eq!(
        result["closing_stock"]["source"],
        "tally_profit_and_loss_less_closing_stock"
    );
    assert!(
        value(&json!({"state": "established", "value": result["closing_stock"]["value"]}))
            .numeric_eq(&bridge_tally_core::ExactDecimal::parse("-500.00").unwrap())
    );
    assert!(
        lead(&response).contains("Closing stock is Tally's own"),
        "{}",
        lead(&response)
    );
}

#[tokio::test]
async fn balance_sheet_uses_tallys_closing_stock_and_names_it() {
    let plans = stock_plans(inventory_flags("Yes"));
    let (response, sent, expected) = call("balance_sheet", plans).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert!(value(&result["result"]["profit_and_loss"]["carried"])
        .numeric_eq(&bridge_tally_core::ExactDecimal::parse("11527.00").unwrap()));
    assert_eq!(result["closing_stock"]["state"], "applied");
    // The comparison with Tally's Profit and Loss is shown once its closing stock is offered.
    assert!(!result["tie_out"].is_null());
}

#[tokio::test]
async fn flags_that_do_not_show_an_integrated_book_decline_the_figure() {
    let plans = stock_plans(inventory_flags("No"));
    let (response, sent, expected) = call("profit_and_loss", plans).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert_eq!(result["state"], "not_established");
    assert_eq!(result["reason"], "tally_balance_sheet_differs");
    assert_eq!(result["closing_stock"]["state"], "declined");
    assert_eq!(result["closing_stock"]["why"], "book_not_shown_integrated");
}

#[tokio::test]
async fn a_book_whose_profit_and_loss_offers_no_closing_stock_reads_no_flags() {
    // The ordinary plans end at the closing extent: a flags read would be one
    // request too many and stall the simulator's sequence.
    let (response, sent, expected) = call("profit_and_loss", plans("-11027.00")).await;
    assert_eq!(sent, expected);
    assert!(result(&response)["closing_stock"].is_null());
}

#[tokio::test]
async fn a_flags_read_that_does_not_bind_to_the_company_declines_the_figure_and_keeps_the_statement(
) {
    // The flags answer names another company: the read is not a failed
    // statement, the book is simply not shown integrated.
    let flags = inventory_flags("Yes").replace(GUID, "00000000-0000-4000-8000-000000000002");
    let (response, sent, expected) = call("profit_and_loss", stock_plans(flags)).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert_eq!(result["closing_stock"]["why"], "inventory_flags_not_read");
    assert_eq!(result["state"], "not_established");
}

fn unreadable_profit_and_loss() -> String {
    // A bare RESPONSE is Tally not recognising the report (§12a.1).
    "<RESPONSE></RESPONSE>".to_string()
}

#[tokio::test]
async fn a_balance_sheet_whose_profit_and_loss_cannot_be_read_is_the_sheet_it_always_was() {
    let plans = plans_full(
        balance_sheet("-11027.00"),
        unreadable_profit_and_loss(),
        None,
    );
    let (response, sent, expected) = call("balance_sheet", plans).await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert!(value(&result["result"]["profit_and_loss"]["carried"])
        .numeric_eq(&bridge_tally_core::ExactDecimal::parse("11027.00").unwrap()));
    assert!(result["closing_stock"].is_null());
    assert!(result["tie_out"].is_null());
}

#[tokio::test]
async fn a_profit_and_loss_whose_own_statement_cannot_be_read_is_refused() {
    // Its gross and net are gated on Tally's Profit and Loss: no answer without it.
    let mut plans = plans_full(
        balance_sheet("-11027.00"),
        unreadable_profit_and_loss(),
        None,
    );
    plans.truncate(plans.len() - 8);
    let (response, _, _) = call("profit_and_loss", plans).await;
    assert_eq!(response["isError"], true, "{response}");
}
