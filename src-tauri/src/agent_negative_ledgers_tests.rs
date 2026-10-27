//! The tool through `call_tool`, against the simulator replaying the reads of
//! `agent_cash_flow_tests` up to the report (identity, the status and mode probe,
//! the opening identity bracket, the book extent), then Tally's own Negative
//! Ledgers, then the closing extent and identity bracket.
//!
//! The report is the committed live capture of 9 Oct 2026 (the
//! `builtin_negative_ledgers_bridge_lab_win_fy_live` fixture), moved to this
//! harness's company only by being served for it: the answer carries no company.
//! These tests prove the tool glue: arguments, the payload, what is withheld, and
//! that no request is sent where the call is refused first. The parser's refusals
//! are proven in `native_negative_ledgers`.
use super::super::*;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

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

/// Tally's Negative Ledgers of 9 Oct 2026: a creditor with a debit balance and a
/// debtor with a credit balance, as received.
fn captured() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_negative_ledgers_bridge_lab_win_fy_live.utf16le.xml"
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

/// The reads up to the date boundary: identity, then the status and mode probe.
fn plans_to_the_boundary() -> Vec<ScenarioPlan> {
    plans_to_the_boundary_probing(companies())
}

fn plans_to_the_boundary_probing(probed: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = Vec::new();
    pair(&mut plans, companies);
    plans.extend([status(), xml(probed)]);
    plans
}

/// A whole call with `report` as Tally's answer: identity, the opening bracket,
/// the extent pair, the report pair, the closing extent pair, the closing bracket.
fn plans(report: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = plans_to_the_boundary();
    plans.push(companies.clone());
    pair(&mut plans, xml(extents()));
    pair(&mut plans, xml(report));
    pair(&mut plans, xml(extents()));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

/// A call that ends once the closing extent has been read: the closing identity
/// bracket (three requests) is never sent, and an unsent plan is a hang.
fn plans_ending_at_the_closing_extent(mut plans: Vec<ScenarioPlan>) -> Vec<ScenarioPlan> {
    plans.truncate(plans.len() - 3);
    plans
}

async fn call(plans: Vec<ScenarioPlan>, from: &str, to: &str) -> (Value, usize) {
    let (response, observed) = call_observed(plans, from, to, Redaction::None).await;
    (response, observed.len())
}

async fn call_observed(
    plans: Vec<ScenarioPlan>,
    from: &str,
    to: &str,
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
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let response = server
        .call_tool(
            "negative_ledgers",
            json!({"company_guid": guid(), "from": from, "to": to}),
        )
        .await;
    (response, simulator.finish().unwrap())
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

#[tokio::test]
async fn the_captured_list_is_returned_as_tallys_own_and_says_what_was_not_checked() {
    let plans = plans(captured());
    let total = plans.len();
    let (response, sent) = call(plans, "2026-04-01", "2027-03-31").await;
    let result = result(&response);
    assert_eq!(sent, total, "every planned request served");
    assert_eq!(result["state"], "observed", "{result}");
    assert_eq!(result["basis"], "tally_native_negative_ledgers");
    assert_eq!(result["count"], 2);
    assert_eq!(
        result["ledgers"],
        json!([
            {"ledger": "Synthetic Creditor", "side": "debit", "amount": "2500.50"},
            {"ledger": "Synthetic Debtor", "side": "credit", "amount": "10000.00"},
        ])
    );
    assert_eq!(
        result["checks"],
        json!({"list_complete": "not_checked", "amounts_against_trial_balance": "not_checked"})
    );
    assert_eq!(result["unmeasured_in_this_answer"], json!([]));
    assert!(result.get("reason").is_none(), "{result}");
}

#[tokio::test]
async fn a_window_that_is_not_a_whole_financial_year_is_named_unmeasured() {
    for (from, to) in [
        ("2026-04-01", "2026-09-30"),
        ("2026-05-01", "2027-03-31"),
        ("2026-04-01", "2027-09-30"),
    ] {
        let (response, _) = call(plans(captured()), from, to).await;
        assert_eq!(
            result(&response)["unmeasured_in_this_answer"],
            json!(["window_other_than_a_whole_financial_year"]),
            "{from}..{to}"
        );
    }
}

#[tokio::test]
async fn an_empty_answer_is_returned_empty_and_is_not_a_finding() {
    let (response, _) = call(
        plans("<ENVELOPE></ENVELOPE>".to_string()),
        "2026-04-01",
        "2027-03-31",
    )
    .await;
    let result = result(&response);
    assert_eq!(result["state"], "not_established", "{result}");
    assert_eq!(
        result["reason"],
        "negative_ledgers_empty_answer_is_not_a_finding"
    );
    assert_eq!(result["count"], 0);
    assert_eq!(result["ledgers"], json!([]));
}

#[tokio::test]
async fn an_answer_that_is_refused_by_the_parser_names_its_own_cause() {
    // The creditor's credit column filled as well as its debit column: not one side.
    let both = captured().replacen(
        "<DSPCLCRAMTA></DSPCLCRAMTA>",
        "<DSPCLCRAMTA>5.00</DSPCLCRAMTA>",
        1,
    );
    assert_ne!(both, captured(), "the target was not found");
    let plans = plans_ending_at_the_closing_extent(plans(both));
    let total = plans.len();
    let (response, sent) = call(plans, "2026-04-01", "2027-03-31").await;
    assert_eq!(sent, total);
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "negative_ledgers_read_failed", "{response}");
    assert_eq!(
        error["cause"], "negative_ledgers_balance_unexpected",
        "{response}"
    );
}

#[tokio::test]
async fn a_report_that_changes_between_its_two_reads_is_refused_as_its_own_drift() {
    let mut plans = plans(captured());
    // The report pair sits just before the closing extent pair (four) and the
    // closing identity bracket (three).
    let second = plans.len() - 7 - 2;
    plans[second] = xml(captured().replace("10000.00", "10000.01"));
    plans.truncate(plans.len() - 7);
    let total = plans.len();
    let (response, sent) = call(plans, "2026-04-01", "2027-03-31").await;
    assert_eq!(sent, total);
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "negative_ledgers_read_failed", "{response}");
    assert_eq!(
        error["cause"], "native_negative_ledgers_changed",
        "{response}"
    );
}

#[tokio::test]
async fn a_book_that_moves_during_the_read_is_reported_as_moved_not_as_what_its_answer_broke() {
    // The closing extent differs from the opening one, and the answer is one the parser
    // would refuse: the move is what is reported.
    let both = captured().replacen(
        "<DSPCLCRAMTA></DSPCLCRAMTA>",
        "<DSPCLCRAMTA>5.00</DSPCLCRAMTA>",
        1,
    );
    let mut plans = plans_ending_at_the_closing_extent(plans(both));
    let closing_first = plans.len() - 4;
    let moved = extents().replace(
        "<ALTVCHID TYPE=\"Number\"> ",
        "<ALTVCHID TYPE=\"Number\"> 9",
    );
    assert_ne!(moved, extents(), "the extent was not changed");
    plans[closing_first] = xml(moved.clone());
    plans[closing_first + 2] = xml(moved);
    let (response, _) = call(plans, "2026-04-01", "2027-03-31").await;
    assert_eq!(response["isError"], true, "{response}");
    let cause = &response["structuredContent"]["result"]["error"]["cause"];
    assert_eq!(cause, "native_ledger_extent_changed", "{response}");
}

#[tokio::test]
async fn the_request_on_the_wire_is_the_negative_ledgers_of_this_company_and_window() {
    // The simulator keeps a hash of each request body: the production call site must have sent
    // Tally's `Negative Ledgers` for the verified company and the window, twice (the paired read).
    use sha2::{Digest, Sha256};
    let (response, observed) = call_observed(
        plans(captured()),
        "2026-04-01",
        "2027-03-31",
        Redaction::None,
    )
    .await;
    assert_ne!(response["isError"], true, "{response}");
    let request = r#"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE><ID>Negative Ledgers</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>Bridge Ageing Lab</SVCURRENTCOMPANY><SVFROMDATE TYPE="Date">20260401</SVFROMDATE><SVTODATE TYPE="Date">20270331</SVTODATE></STATICVARIABLES></DESC></BODY></ENVELOPE>"#;
    let mut body = vec![0xFF, 0xFE];
    body.extend(request.encode_utf16().flat_map(u16::to_le_bytes));
    let sha: String = Sha256::digest(&body)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let sent = observed
        .iter()
        .filter(|request| request.request_body_sha256 == sha)
        .count();
    assert_eq!(sent, 2, "the Negative Ledgers request, read twice");
}

/// A call that must be refused with `code` once `plans` have been answered, with no report
/// request sent: the plans end before it, and a request past them has nothing to answer it.
async fn assert_refused_before_the_report(
    plans: Vec<ScenarioPlan>,
    from: &str,
    to: &str,
    code: &str,
) {
    let total = plans.len();
    let (response, sent) = call(plans, from, to).await;
    assert_eq!(response["isError"], true, "{code}: {response}");
    assert_eq!(
        response["structuredContent"]["result"]["error"],
        json!({"code": code, "message": "ComplyEaze Bridge refused this operation."}),
        "{response}"
    );
    assert_eq!(sent, total, "{code}: every planned request served");
}

#[tokio::test]
async fn a_reversed_window_is_refused_before_any_request() {
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
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let response = server
        .call_tool(
            "negative_ledgers",
            json!({"company_guid": guid(), "from": "2027-03-31", "to": "2026-04-01"}),
        )
        .await;
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"], "invalid_date_range",
        "{response}"
    );
}

#[tokio::test]
async fn education_mode_is_refused_before_the_report() {
    let education = companies().replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    );
    assert_ne!(education, companies());
    assert_refused_before_the_report(
        plans_to_the_boundary_probing(education),
        "2026-04-01",
        "2027-03-31",
        "negative_ledgers_education_unqualified",
    )
    .await;
}

#[tokio::test]
async fn a_window_before_the_books_is_refused_before_the_report() {
    // The book extent says the books begin on 1 April 2024.
    let mut plans = plans_to_the_boundary();
    plans.push(xml(companies()));
    pair(&mut plans, xml(extents()));
    assert_refused_before_the_report(
        plans,
        "2024-01-01",
        "2024-03-31",
        "negative_ledgers_before_books",
    )
    .await;
}

#[tokio::test]
async fn under_mask_parties_each_ledger_name_is_masked_and_nothing_else_changes() {
    let (plain, _) = call_observed(
        plans(captured()),
        "2026-04-01",
        "2027-03-31",
        Redaction::None,
    )
    .await;
    let (masked, _) = call_observed(
        plans(captured()),
        "2026-04-01",
        "2027-03-31",
        Redaction::MaskParties,
    )
    .await;
    let plain_ledgers = &result(&plain)["ledgers"];
    let masked_ledgers = &result(&masked)["ledgers"];
    // The capture lists two ledgers: with none, the loop below would run zero times.
    assert_eq!(plain_ledgers.as_array().unwrap().len(), 2);
    assert_eq!(masked_ledgers.as_array().unwrap().len(), 2);
    for (plain, masked) in plain_ledgers
        .as_array()
        .unwrap()
        .iter()
        .zip(masked_ledgers.as_array().unwrap())
    {
        let name = plain["ledger"].as_str().unwrap();
        assert_eq!(masked["ledger"], json!(crate::agent::mask(name)));
        assert_ne!(masked["ledger"], plain["ledger"]);
        assert_eq!(masked["side"], plain["side"]);
        assert_eq!(masked["amount"], plain["amount"]);
    }
    assert_eq!(result(&plain)["count"], result(&masked)["count"]);
}

#[test]
fn a_whole_financial_year_is_one_april_to_the_next_thirty_first_of_march() {
    use super::is_a_whole_financial_year as whole;
    assert!(whole("20260401", "20270331"));
    assert!(whole("20240401", "20250331"));
    assert!(!whole("20260401", "20260930"));
    assert!(!whole("20260401", "20280331"));
    assert!(!whole("20260402", "20270331"));
    assert!(!whole("20260401", "20270330"));
    assert!(!whole("2026", "2027"));
}
