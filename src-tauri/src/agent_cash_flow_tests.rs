//! The tool through `call_tool`, against the simulator replaying the same reads
//! as `agent_statements_tests` (identity, the Trial Balance bracket, the group
//! tree), then Tally's own Cash Flow.
//!
//! The Trial Balance and the group tree are captures from two different
//! synthetic companies; the group tree's company GUID is rewritten to the Trial
//! Balance's so the response binds. Tally's Cash Flow is synthetic text in the
//! shape of the live capture (the committed `builtin_cash_flow_*` fixtures),
//! sized to the Trial Balance's one cash ledger. These tests prove the tool
//! glue: arguments, the check's verdict in the payload, and what is withheld.
//! The simulator serves `plans()` in order, so a read the tool skips or
//! reorders fails its response check, and a missing read stalls `finish()`. The
//! check's arithmetic is proven in `reports::cash_flow`, and the parser's
//! refusals in `native_cash_flow`.
use super::super::*;
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

/// Tally's Cash Flow for April to June 2026 in the captured shape: one row per
/// month, April with `april` as its debit and closing and the other months with
/// every amount empty.
fn cash_flow(april: &str) -> String {
    let empty = "<DSPACCINFO><DSPDRAMT><DSPDRAMTA></DSPDRAMTA></DSPDRAMT><DSPCRAMT><DSPCRAMTA></DSPCRAMTA></DSPCRAMT><DSPCLAMT><DSPCLAMTA></DSPCLAMTA></DSPCLAMT></DSPACCINFO>";
    format!(
        "<ENVELOPE><DSPPERIOD>April</DSPPERIOD><DSPACCINFO><DSPDRAMT><DSPDRAMTA>{april}</DSPDRAMTA></DSPDRAMT><DSPCRAMT><DSPCRAMTA></DSPCRAMTA></DSPCRAMT><DSPCLAMT><DSPCLAMTA>{april}</DSPCLAMTA></DSPCLAMT></DSPACCINFO><DSPPERIOD>May</DSPPERIOD>{empty}<DSPPERIOD>June</DSPPERIOD>{empty}</ENVELOPE>"
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

/// The reads up to the date boundary, which is where a window that is not whole
/// months is refused: identity, then the status and mode probe.
fn plans_to_the_boundary() -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = Vec::new();
    pair(&mut plans, companies.clone());
    plans.extend([status(), companies]);
    plans
}

/// A whole call: identity, then the Trial Balance bracket with the group tree
/// and Tally's Cash Flow inside it.
fn plans(april: &str) -> Vec<ScenarioPlan> {
    plans_with(april, |report| report)
}

/// The same call with the Trial Balance changed by `change`, to reach a branch
/// the capture does not: the cash ledger moved to another group.
fn plans_with(april: &str, change: impl Fn(String) -> String) -> Vec<ScenarioPlan> {
    plans_with_report(xml(cash_flow(april)), change)
}

fn plans_with_report(
    cash_flow: ScenarioPlan,
    change: impl Fn(String) -> String,
) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let currency = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let report = change(
        include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_known_lab.xml"
        )
        .to_string(),
    );
    let mut plans = plans_to_the_boundary();
    // The opening identity bracket.
    plans.push(companies.clone());
    pair(&mut plans, xml(extents()));
    pair(&mut plans, xml(currency));
    pair(&mut plans, xml(report));
    pair(&mut plans, xml(groups()));
    pair(&mut plans, cash_flow);
    pair(&mut plans, xml(extents()));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

/// The plans of a call that ends at the Cash Flow pair: the closing extent pair
/// (four requests) and the closing identity bracket (three) are never sent, and
/// an unsent plan is a hang.
fn plans_ending_at_the_cash_flow(mut plans: Vec<ScenarioPlan>) -> Vec<ScenarioPlan> {
    plans.truncate(plans.len() - 7);
    plans
}

async fn call(plans: Vec<ScenarioPlan>, from: &str, to: &str) -> (Value, usize, usize) {
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
    let response = server
        .call_tool(
            "cash_flow",
            json!({"company_guid": GUID, "from": from, "to": to}),
        )
        .await;
    (response, simulator.finish().unwrap().len(), expected)
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

/// The headline's lead, from the structured content of a response.
fn lead(response: &Value) -> String {
    response["structuredContent"]["headline"]["lead"]
        .as_str()
        .expect("a cash flow carries a headline")
        .to_string()
}

#[tokio::test]
async fn a_net_that_ties_returns_the_months_and_says_what_was_checked() {
    let (response, sent, expected) = call(plans("-4950.00"), "2026-04-01", "2026-06-30").await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert_eq!(result["state"], "observed", "{result}");
    assert!(result.get("reason").is_none(), "{result}");
    let months = result["months"].as_array().unwrap();
    assert_eq!(months.len(), 3);
    assert_eq!(months[0]["month"], "2026-04");
    assert_eq!(
        months[0]["closing"],
        json!({"state": "present", "value": "-4950.00"})
    );
    // An empty month stays empty: not zero.
    assert_eq!(months[1]["month"], "2026-05");
    assert_eq!(months[1]["closing"], json!({"state": "empty"}));
    assert_eq!(result["net_total"]["state"], "checked");
    assert_eq!(result["net_total"]["value"], "-4950.00");
    assert_eq!(
        result["basis"],
        "tally_native_cash_flow_checked_against_trial_balance"
    );
    assert_eq!(result["net_total"]["money_ledgers"], 1);
    assert_eq!(
        result["checks"],
        json!({
            "net_total": "checked",
            "month_split": "not_checked",
            "debit_and_credit_columns": "withheld",
        })
    );
    // In words, ahead of the figures: what was read, what was checked, what was not.
    let lead = lead(&response);
    assert!(lead.starts_with("Cash flow for \u{201c}"), "{lead}");
    assert!(
        lead.contains("equals the cash and bank ledgers of its trial balance"),
        "{lead}"
    );
    assert!(lead.contains("has not been checked"), "{lead}");
    assert!(
        lead.contains("debit and credit columns are not shown"),
        "{lead}"
    );
    assert!(
        lead.contains("no month with an outflow has been measured"),
        "{lead}"
    );
    assert!(
        lead.contains("not a cash flow statement under AS 3"),
        "{lead}"
    );
    assert!(!lead.contains("Not established"), "{lead}");
}

#[tokio::test]
async fn a_net_that_differs_withholds_the_months_and_names_both_figures() {
    let (response, sent, expected) = call(plans("-4949.00"), "2026-04-01", "2026-06-30").await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert_eq!(result["state"], "not_established", "{result}");
    assert_eq!(result["reason"], "cash_flow_differs_from_trial_balance");
    assert!(result["months"].is_null(), "{result}");
    let net = &result["net_total"];
    assert_eq!(net["state"], "differs");
    assert_eq!(net["use"], "investigation_only");
    assert_eq!(net["tally_cash_flow_months_added"], "-4949.00");
    assert_eq!(net["trial_balance_cash_and_bank_ledgers"], "-4950.00");
    // Compared, and the figures disagree: not "not checked"; the months go with it.
    assert_eq!(result["checks"]["net_total"], "differs");
    assert_eq!(result["checks"]["month_split"], "withheld");
    assert_eq!(result["basis"], "tally_native_cash_flow_withheld");
    let lead = lead(&response);
    assert!(
        lead.starts_with("Not established: the cash flow for \u{201c}"),
        "{lead}"
    );
    assert!(lead.contains("so the months are withheld"), "{lead}");
    assert!(lead.contains("open Cash Flow in Tally"), "{lead}");
}

/// The captured trial balance with its cash ledger under another group.
fn cash_under(group: &'static str) -> impl Fn(String) -> String {
    move |report| {
        let changed = report.replace(
            "<PARENT TYPE=\"String\">Cash-in-Hand</PARENT>",
            &format!("<PARENT TYPE=\"String\">{group}</PARENT>"),
        );
        assert_ne!(changed, report, "the cash ledger's group was not found");
        changed
    }
}

#[tokio::test]
async fn a_bank_od_ledger_with_movement_refuses_the_result_and_says_why() {
    // synthetic: the cash ledger moved under Bank OD A/c, so it carries movement there.
    let (response, sent, expected) = call(
        plans_with("-4950.00", cash_under("Bank OD A/c")),
        "2026-04-01",
        "2026-06-30",
    )
    .await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert_eq!(result["state"], "not_established", "{result}");
    assert_eq!(result["reason"], "cash_flow_money_group_unmeasured");
    assert!(result["months"].is_null(), "{result}");
    assert_eq!(result["net_total"]["state"], "not_checked");
    assert_eq!(result["net_total"]["ledgers"], 1);
    assert_eq!(result["checks"]["net_total"], "not_checked");
    assert_eq!(result["checks"]["month_split"], "withheld");
    assert_eq!(result["basis"], "tally_native_cash_flow_withheld");
    let lead = lead(&response);
    assert!(
        lead.starts_with("Not established: the cash flow for \u{201c}"),
        "{lead}"
    );
    assert!(lead.contains("Bank OD A/c or Bank OCC A/c"), "{lead}");
    assert!(lead.contains("has to be read in Tally"), "{lead}");
}

#[tokio::test]
async fn a_book_with_no_cash_or_bank_ledger_and_an_empty_cash_flow_is_not_called_checked() {
    // synthetic: the cash ledger moved under Sundry Debtors; Tally's Cash Flow is all empty.
    let (response, sent, expected) = call(
        plans_with("", cash_under("Sundry Debtors")),
        "2026-04-01",
        "2026-06-30",
    )
    .await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert_eq!(result["state"], "not_established", "{result}");
    assert_eq!(result["reason"], "cash_flow_nothing_to_compare");
    assert!(result["months"].is_null(), "{result}");
    assert_eq!(result["net_total"]["state"], "not_checked");
    assert_eq!(result["checks"]["net_total"], "not_checked");
    let lead = lead(&response);
    assert!(lead.contains("nothing was checked"), "{lead}");
    assert!(!lead.contains("equals the cash and bank ledgers"), "{lead}");
}

#[tokio::test]
async fn an_answer_that_is_refused_by_the_parser_names_its_own_cause() {
    // Tally's Cash Flow came back as an empty envelope: a report that was not rendered.
    let plans = plans_ending_at_the_cash_flow(plans_with_report(
        xml("<ENVELOPE></ENVELOPE>".to_string()),
        |report| report,
    ));
    let (response, sent, expected) = call(plans, "2026-04-01", "2026-06-30").await;
    assert_eq!(sent, expected);
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "cash_flow_read_failed", "{response}");
    assert_eq!(error["cause"], "cash_flow_empty_envelope", "{response}");
}

#[tokio::test]
async fn a_month_shows_tallys_closing_column_not_its_debit() {
    // The captured shape has the same figure in both columns, so the debit is changed here:
    // the net total (the closings) still ties, and the month must carry the closing.
    let changed = cash_flow("-4950.00").replace(
        "<DSPDRAMTA>-4950.00</DSPDRAMTA>",
        "<DSPDRAMTA>-1.00</DSPDRAMTA>",
    );
    assert_ne!(changed, cash_flow("-4950.00"), "the debit was not found");
    let (response, sent, expected) = call(
        plans_with_report(xml(changed), |report| report),
        "2026-04-01",
        "2026-06-30",
    )
    .await;
    let result = result(&response);
    assert_eq!(sent, expected);
    assert_eq!(result["state"], "observed", "{result}");
    assert_eq!(
        result["months"][0]["closing"],
        json!({"state": "present", "value": "-4950.00"})
    );
}

#[tokio::test]
async fn a_cash_flow_that_changes_between_its_two_reads_is_refused_as_its_own_drift() {
    let mut plans = plans_with("-4950.00", |report| report);
    // The pair sits just before the closing extent pair (four) and identity bracket (three).
    let second = plans.len() - 7 - 2;
    plans[second] = xml(cash_flow("-4949.00"));
    let (response, sent, expected) = call(
        plans_ending_at_the_cash_flow(plans),
        "2026-04-01",
        "2026-06-30",
    )
    .await;
    assert_eq!(sent, expected);
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "cash_flow_read_failed", "{response}");
    assert_eq!(error["cause"], "native_cash_flow_changed", "{response}");
}

#[tokio::test]
async fn a_window_that_is_not_whole_months_is_refused_before_any_cash_flow_request() {
    for (from, to, code) in [
        (
            "2026-04-15",
            "2026-06-30",
            "cash_flow_window_not_month_start",
        ),
        ("2026-04-01", "2026-06-29", "cash_flow_window_not_month_end"),
        (
            "2026-04-01",
            "2027-04-30",
            "cash_flow_window_too_many_months",
        ),
    ] {
        let plans = plans_to_the_boundary();
        let total = plans.len();
        let (response, sent, _) = call(plans, from, to).await;
        assert_eq!(response["isError"], true, "{from}..{to}: {response}");
        let error = &response["structuredContent"]["result"]["error"];
        assert_eq!(error["code"], code, "{from}..{to}: {response}");
        // Nothing past the date boundary was sent.
        assert_eq!(sent, total, "{from}..{to}");
    }
}
