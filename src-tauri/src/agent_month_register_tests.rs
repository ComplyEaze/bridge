//! The tool through `call_tool`, against the simulator replaying the reads of a
//! whole call: the company list (paired) for the identity, the mode probe and
//! identity bracket, the opening extent (paired), the register (paired), the
//! closing extent (paired) and the closing identity and mode checks. The
//! register answers are the live captures of BRIDGE PILOT LAB's Sales and
//! Purchase Registers (13 Oct 2026), served unchanged: a register answer carries
//! no company identity, so the identity and extent reads around it are the
//! captures the company features tests use (another synthetic company, "Bridge
//! Ageing Lab", books from 20260401). These tests prove the tool glue: the
//! arguments, the rows as printed and what a refusal looks like; the parser's
//! refusals are proven in `native_month_register`.
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

fn extents() -> String {
    include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    )
    .to_string()
}

fn sales() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_sales_register_pilot_lab_fy_live.utf16le.xml"
    ))
}

fn purchase() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_purchase_register_pilot_lab_fy_live.utf16le.xml"
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

/// The company list for the identity, then the mode probe (a status and the
/// company list).
fn plans_to_the_mode_probe() -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = Vec::new();
    pair(&mut plans, companies.clone());
    plans.extend([status(), companies]);
    plans
}

/// A whole call, the register answered with `register`.
fn plans(register: String) -> Vec<ScenarioPlan> {
    let companies = xml(companies());
    let mut plans = plans_to_the_mode_probe();
    plans.push(companies.clone());
    pair(&mut plans, xml(extents()));
    pair(&mut plans, xml(register));
    pair(&mut plans, xml(extents()));
    plans.extend([companies.clone(), status(), companies]);
    plans
}

/// The plans of a call that ends at the register pair: the closing extent pair
/// (four requests) and the closing identity bracket (three) are never sent, and
/// an unsent plan is a hang.
fn plans_ending_at_the_register(register: String) -> Vec<ScenarioPlan> {
    let mut plans = plans(register);
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
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let response = server.call_tool("month_register", args).await;
    (response, simulator.finish().unwrap().len(), expected)
}

/// The test company's GUID, read from the captured company list by its name.
fn guid() -> String {
    let companies = bridge_tally_protocol::parse_companies_from_collection(&companies()).unwrap();
    let mut named = companies
        .iter()
        .filter(|row| row.name == "Bridge Ageing Lab");
    let company = named.next().expect("the test company");
    assert!(named.next().is_none(), "one company of that name");
    company.guid.clone().expect("its GUID")
}

fn args(kind: &str, from: &str, to: &str) -> Value {
    json!({"company_guid": guid(), "kind": kind, "from": from, "to": to})
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

fn error(response: &Value) -> &Value {
    assert_eq!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]["error"]
}

fn amount(value: &str) -> Value {
    json!({"state": "present", "value": value})
}

fn empty() -> Value {
    json!({"state": "empty"})
}

#[tokio::test]
async fn the_sales_register_returns_twelve_months_as_tally_printed_them() {
    let (response, sent, expected) =
        call(plans(sales()), args("sales", "20260401", "20270331")).await;
    let result = result(&response);
    assert_eq!(sent, expected, "every planned request was served");
    assert_eq!(result["state"], "observed");
    assert_eq!(result["basis"], "tally_month_wise_register_as_printed");
    assert_eq!(result["kind"], "sales");
    assert_eq!(result["report_name"], "Sales Register");
    let months = result["months"].as_array().expect("months");
    assert_eq!(months.len(), 12);
    assert_eq!(months[0]["month"], "2026-04");
    assert_eq!(months[11]["month"], "2027-03");
    assert_eq!(
        months[4],
        json!({"month": "2026-08", "debit": empty(), "credit": amount("17638.50"), "closing": amount("17638.50")})
    );
    assert_eq!(
        months[5],
        json!({"month": "2026-09", "debit": empty(), "credit": amount("210.20"), "closing": amount("17848.70")})
    );
    assert_eq!(
        months[0],
        json!({"month": "2026-04", "debit": empty(), "credit": empty(), "closing": empty()})
    );
    assert_eq!(
        result["checks"],
        json!({"against_register_screen": "not_checked", "against_vouchers": "not_checked", "against_trial_balance": "not_checked"})
    );
    let limitations = result["limitations"].as_array().expect("limitations");
    for needle in [
        "exactly as Tally printed them",
        "nothing here ties them to the vouchers",
        "An empty amount is not zero",
        "whole months, at most twelve",
        "equality with Tally's own register screens",
        "the Journal Register prints a count of vouchers",
    ] {
        assert!(
            limitations
                .iter()
                .any(|line| line.as_str().is_some_and(|line| line.contains(needle))),
            "no limitation says {needle:?}: {limitations:?}"
        );
    }
    assert_eq!(response["structuredContent"]["company"]["guid"], guid());
}

#[tokio::test]
async fn the_purchase_register_returns_its_purchases_as_a_negative_debit_and_carries_the_closing() {
    let (response, sent, expected) = call(
        plans(purchase()),
        args("purchase", "2026-04-01", "2027-03-31"),
    )
    .await;
    let result = result(&response);
    assert_eq!(sent, expected, "every planned request was served");
    assert_eq!(result["kind"], "purchase");
    assert_eq!(result["report_name"], "Purchase Register");
    let months = result["months"].as_array().expect("months");
    assert_eq!(
        months[4],
        json!({"month": "2026-08", "debit": amount("-2100.00"), "credit": empty(), "closing": amount("-2100.00")})
    );
    assert_eq!(
        months[5],
        json!({"month": "2026-09", "debit": empty(), "credit": empty(), "closing": amount("-2100.00")})
    );
}

/// A window that is not whole months is refused after the mode probe and
/// before the identity bracket, so no register request is sent.
#[tokio::test]
async fn a_window_that_is_not_whole_months_is_refused_before_the_register_is_asked_for() {
    for (from, to, code) in [
        (
            "20260402",
            "20270331",
            "month_register_window_not_month_start",
        ),
        (
            "20260401",
            "20270330",
            "month_register_window_not_month_end",
        ),
        ("20270401", "20260331", "month_register_window_reversed"),
        (
            "20260401",
            "20270430",
            "month_register_window_too_many_months",
        ),
    ] {
        let (response, sent, expected) =
            call(plans_to_the_mode_probe(), args("sales", from, to)).await;
        assert_eq!(sent, expected, "{from} {to}");
        assert_eq!(error(&response)["code"], code, "{response}");
    }
}

/// The start decides: a window wholly before the books, and one that starts
/// before them and ends inside them, are both refused.
#[tokio::test]
async fn a_window_starting_before_the_books_is_refused_before_the_register_is_asked_for() {
    // The test company's books start on 2026-04-01.
    for (from, to) in [("20260301", "20260331"), ("20260301", "20260430")] {
        let mut plans = plans_to_the_mode_probe();
        plans.push(xml(companies()));
        pair(&mut plans, xml(extents()));
        let (response, sent, expected) = call(plans, args("sales", from, to)).await;
        assert_eq!(sent, expected, "{from} {to}");
        assert_eq!(
            error(&response)["code"],
            "month_register_before_books",
            "{from} {to} {response}"
        );
    }
}

#[tokio::test]
async fn education_mode_is_refused_with_its_own_code_before_the_register_is_asked_for() {
    let education = companies().replace(
        "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
        "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
    );
    assert_ne!(education, companies());
    // The identity read, then the status and mode probe, which sees Education.
    let mut plans = Vec::new();
    pair(&mut plans, xml(companies()));
    plans.extend([status(), xml(education)]);
    let (response, sent, expected) = call(plans, args("sales", "20260401", "20270331")).await;
    assert_eq!(sent, expected, "every planned request was served");
    assert_eq!(
        error(&response)["code"],
        "month_register_education_unqualified",
        "{response}"
    );
}

/// Tally's refusal of a report name it does not have (a live capture) is a
/// failure under the register's own code, never an empty year.
#[tokio::test]
async fn a_report_tally_refuses_is_a_failure_and_never_an_empty_year() {
    let unknown = decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_unknown_report_refusal_live.utf16le.xml"
    ));
    let (response, sent, expected) = call(
        plans_ending_at_the_register(unknown),
        args("sales", "20260401", "20270331"),
    )
    .await;
    assert_eq!(sent, expected, "every planned request was served");
    let error = error(&response);
    assert_eq!(error["code"], "month_register_read_failed", "{response}");
    assert_eq!(
        error["cause"], "month_register_tally_reported_failure",
        "{response}"
    );
}

#[tokio::test]
async fn a_book_that_moved_during_the_read_is_refused_after_the_register() {
    let moved = extents().replace(
        "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>",
        "<ALTMSTID TYPE=\"Number\"> 225</ALTMSTID>",
    );
    assert_ne!(moved, extents());
    let mut plans = plans_to_the_mode_probe();
    plans.push(xml(companies()));
    pair(&mut plans, xml(extents()));
    pair(&mut plans, xml(sales()));
    pair(&mut plans, xml(moved));
    let (response, sent, expected) = call(plans, args("sales", "20260401", "20270331")).await;
    assert_eq!(sent, expected, "every planned request was served");
    let error = error(&response);
    assert_eq!(error["code"], "month_register_read_failed", "{response}");
    assert_eq!(
        error["cause"], "month_register_extent_changed",
        "{response}"
    );
}

#[tokio::test]
async fn the_arguments_are_refused_before_any_request() {
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
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    for (args, code) in [
        (
            json!({"kind": "sales", "from": "20260401", "to": "20270331"}),
            "company_guid_required",
        ),
        (
            json!({"company_guid": guid(), "kind": "journal", "from": "20260401", "to": "20270331"}),
            "argument_invalid:kind",
        ),
        (
            json!({"company_guid": guid(), "from": "20260401", "to": "20270331"}),
            "kind_required",
        ),
    ] {
        let response = server.call_tool("month_register", args.clone()).await;
        assert_eq!(error(&response)["code"], code, "{args} {response}");
    }
}
