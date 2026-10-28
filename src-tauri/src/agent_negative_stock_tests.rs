//! The tool through `call_tool`, against the simulator replaying the reads of
//! `agent_cash_flow_tests` up to the report (identity, the status and mode probe,
//! the opening identity bracket, the book extent), then Tally's own Negative
//! Stock, then the closing extent and identity bracket.
//!
//! The reports are the committed live captures of 21 Oct 2026 (the
//! `builtin_negative_stock_negstock_lab_2_*` fixtures), moved to this harness's
//! company only by being served for it: the answer carries no company. These
//! tests prove the tool glue: arguments, the payload, what is withheld, and that
//! no request is sent where the call is refused first. The parser's refusals are
//! proven in `native_negative_stock`.
use super::super::*;
use bridge_tally_core::TallyDate;
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

/// Tally's Negative Stock for a whole year on a book with one item sold before it was bought, as received.
fn captured() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_negative_stock_negstock_lab_2_fy_live.utf16le.xml"
    ))
}

/// The same book to 31 May: the quantity as at that date, rate and value empty.
fn captured_to_may() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_negative_stock_negstock_lab_2_to_20250531_live.utf16le.xml"
    ))
}

/// A book with stock items and none short: an empty envelope.
fn captured_empty() -> String {
    decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/builtin_negative_stock_tieout_c_fy_empty_live.utf16le.xml"
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
        audit_enabled: false,
        master_writes_enabled: false,
    });
    let response = server
        .call_tool(
            "negative_stock",
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
    assert_eq!(result["basis"], "tally_native_negative_stock");
    assert_eq!(result["count"], 1);
    assert_eq!(
        result["listed_items"],
        json!([{
            "item": "Lab Short Item",
            "quantity": {"state": "read", "amount": "-3", "unit": "Nos"},
            "rate": {"state": "present", "value": "120.00"},
            "value": {"state": "present", "value": "360.00"},
        }])
    );
    assert_eq!(
        result["checks"],
        json!({"list_complete": "not_checked", "quantities_against_stock_summary": "not_checked"})
    );
    assert_eq!(result["unmeasured_in_this_answer"], json!([]));
    assert!(result.get("reason").is_none(), "{result}");
}

/// The sign a value carries is stated in the result and the tool text (#2051): the captured
/// item's value 360.00 is positive, and the screen shows it as (-)360.00.
#[tokio::test]
async fn the_sign_a_value_carries_is_stated_beside_the_captured_positive_value() {
    let (response, _) = call(plans(captured()), "2026-04-01", "2027-03-31").await;
    let result = result(&response);
    assert_eq!(result["listed_items"][0]["value"]["value"], "360.00");
    let limitations = result["limitations"].as_array().unwrap();
    let sentence = limitations
        .iter()
        .filter_map(Value::as_str)
        .find(|line| line.starts_with("A value keeps the sign Tally's export carries"))
        .expect("the sign is stated in the limitations");
    for part in [
        "a debit is negative and a credit is positive",
        "(-)360.00",
        "section 12a.13",
    ] {
        assert!(sentence.contains(part), "{part}: {sentence}");
    }
    let definitions = crate::agent::catalog::registered_tool_definitions(true, true);
    let description = definitions
        .as_array()
        .and_then(|tools| tools.iter().find(|tool| tool["name"] == "negative_stock"))
        .and_then(|tool| tool["description"].as_str())
        .expect("negative_stock is registered");
    assert!(
        description
            .contains("a value keeps the export's sign (a debit negative, a credit positive)"),
        "{description}"
    );
}

#[tokio::test]
async fn an_empty_rate_and_value_stay_empty_and_the_quantity_keeps_its_sign() {
    let (response, _) = call(plans(captured_to_may()), "2026-04-01", "2026-05-31").await;
    let result = result(&response);
    assert_eq!(result["state"], "observed", "{result}");
    assert_eq!(
        result["listed_items"],
        json!([{
            "item": "Lab Short Item",
            "quantity": {"state": "read", "amount": "-5", "unit": "Nos"},
            "rate": {"state": "empty"},
            "value": {"state": "empty"},
        }])
    );
    assert_eq!(result["unmeasured_in_this_answer"], json!([]));
}

#[tokio::test]
async fn a_quantity_that_is_empty_or_not_a_number_and_a_unit_is_named_so_and_is_not_zero() {
    for (sent, expected) in [
        ("<DSPCLQTY></DSPCLQTY>", json!({"state": "empty"})),
        (
            "<DSPCLQTY>-3 Nos extra</DSPCLQTY>",
            json!({"state": "unread"}),
        ),
    ] {
        let report = captured().replacen("<DSPCLQTY>-3 Nos</DSPCLQTY>", sent, 1);
        assert_ne!(report, captured(), "the target was not found");
        let (response, _) = call(plans(report), "2026-04-01", "2027-03-31").await;
        assert_eq!(result(&response)["listed_items"][0]["quantity"], expected);
    }
}

#[tokio::test]
async fn a_window_that_does_not_start_in_april_or_leaves_the_year_is_named_unmeasured() {
    for (from, to) in [
        ("2026-05-01", "2027-03-31"),
        ("2026-04-02", "2027-03-31"),
        ("2026-04-01", "2027-04-01"),
        ("2026-04-01", "2028-03-31"),
    ] {
        let (response, _) = call(plans(captured()), from, to).await;
        assert_eq!(
            result(&response)["unmeasured_in_this_answer"],
            json!(["window_other_than_from_1_april_within_one_financial_year"]),
            "{from}..{to}"
        );
    }
}

#[tokio::test]
async fn an_empty_envelope_is_not_established_and_is_not_a_finding() {
    let (response, _) = call(plans(captured_empty()), "2026-04-01", "2027-03-31").await;
    let result = result(&response);
    assert_eq!(result["state"], "not_established", "{result}");
    assert_eq!(
        result["reason"],
        "negative_stock_empty_answer_is_not_a_finding"
    );
    assert_eq!(result["count"], 0);
    assert_eq!(result["listed_items"], json!([]));
}

#[tokio::test]
async fn an_answer_that_is_refused_by_the_parser_names_its_own_cause() {
    // The one listed item listed twice.
    let text = captured();
    let start = text.find("<DSPACCNAME>").expect("the item's name block");
    let end = text.find("</ENVELOPE>").expect("the closing tag");
    let twice = format!("{}{}{}", &text[..end], &text[start..end], &text[end..]);
    let plans = plans_ending_at_the_closing_extent(plans(twice));
    let total = plans.len();
    let (response, sent) = call(plans, "2026-04-01", "2027-03-31").await;
    assert_eq!(sent, total);
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "negative_stock_read_failed", "{response}");
    assert_eq!(
        error["cause"], "negative_stock_item_duplicated",
        "{response}"
    );
}

#[tokio::test]
async fn a_report_with_more_rows_than_the_books_mark_allows_is_refused_not_returned() {
    // The mark of the harness's book is 224: 225 listed items cannot be, whatever the report says.
    let text = captured();
    let start = text.find("<DSPACCNAME>").expect("the item's name block");
    let end = text.find("</ENVELOPE>").expect("the closing tag");
    let block = &text[start..end];
    let rows: String = (0..225)
        .map(|n| block.replace("Lab Short Item", &format!("Lab Short Item {n}")))
        .collect();
    let report = format!("{}{}{}", &text[..start], rows, &text[end..]);
    let plans = plans_ending_at_the_closing_extent(plans(report));
    let total = plans.len();
    let (response, sent) = call(plans, "2026-04-01", "2027-03-31").await;
    assert_eq!(sent, total);
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(
        error["code"], "negative_stock_bound_premise_violated",
        "{response}"
    );
    assert_eq!(
        error["cause"], "stock_rows_exceed_master_mark",
        "{response}"
    );
}

#[tokio::test]
async fn a_report_that_changes_between_its_two_reads_is_refused_as_its_own_drift() {
    let mut plans = plans(captured());
    // The report pair sits just before the closing extent pair (four) and the
    // closing identity bracket (three).
    let second = plans.len() - 7 - 2;
    plans[second] = xml(captured().replace("360.00", "361.00"));
    plans.truncate(plans.len() - 7);
    let total = plans.len();
    let (response, sent) = call(plans, "2026-04-01", "2027-03-31").await;
    assert_eq!(sent, total);
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "negative_stock_read_failed", "{response}");
    assert_eq!(
        error["cause"], "native_negative_stock_changed",
        "{response}"
    );
}

#[tokio::test]
async fn a_book_that_moves_during_the_read_is_reported_as_moved_not_as_what_its_answer_broke() {
    // The closing extent differs from the opening one, and the answer is one the parser
    // would refuse: the move is what is reported.
    let broken = captured().replacen("<DSPCLQTY>", "<DSPCLXTY>", 1);
    assert_ne!(broken, captured(), "the target was not found");
    let mut plans = plans_ending_at_the_closing_extent(plans(broken));
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
    assert_eq!(cause, "negative_stock_extent_changed", "{response}");
}

#[tokio::test]
async fn the_request_on_the_wire_is_the_negative_stock_of_this_company_and_window() {
    // The simulator keeps a hash of each request body: the production call site must have sent
    // Tally's `Negative Stock` for the verified company and the window, twice (the paired read).
    use bridge_tally_protocol::{
        native_negative_stock::render_native_negative_stock_request,
        native_outstandings::NativeLedgerSnapshotPeriod, outstandings_shared::DateBoundaryProfile,
    };
    use sha2::{Digest, Sha256};
    let (response, observed) = call_observed(
        plans(captured()),
        "2026-04-01",
        "2027-03-31",
        Redaction::None,
    )
    .await;
    assert_ne!(response["isError"], true, "{response}");
    let period = NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse("20260401").unwrap(),
        TallyDate::parse("20270331").unwrap(),
    )
    .unwrap();
    let request = render_native_negative_stock_request("Bridge Ageing Lab", &period);
    assert!(request.contains("<ID>Negative Stock</ID>"), "{request}");
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
    assert_eq!(sent, 2, "the Negative Stock request, read twice");
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
        response["structuredContent"]["result"]["error"]["code"], code,
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
        audit_enabled: false,
        master_writes_enabled: false,
    });
    let response = server
        .call_tool(
            "negative_stock",
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
        "negative_stock_education_unqualified",
    )
    .await;
}

#[tokio::test]
async fn a_window_before_the_books_is_refused_before_the_report() {
    // The book extent says the books begin on 1 April 2026.
    let mut plans = plans_to_the_boundary();
    plans.push(xml(companies()));
    pair(&mut plans, xml(extents()));
    assert_refused_before_the_report(
        plans,
        "2025-04-01",
        "2026-03-31",
        "negative_stock_before_books",
    )
    .await;
}

#[tokio::test]
async fn a_book_too_large_for_the_ceiling_is_refused_with_its_size_before_the_report() {
    // The ceiling is the stock summary's, a stated guess: 874 is admitted, 875 is not.
    let over = extents().replace(
        "<ALTMSTID TYPE=\"Number\"> 224</ALTMSTID>",
        "<ALTMSTID TYPE=\"Number\"> 875</ALTMSTID>",
    );
    assert_ne!(over, extents(), "the mark was not changed");
    let mut plans = plans_to_the_boundary();
    plans.push(xml(companies()));
    pair(&mut plans, xml(over));
    let total = plans.len();
    let (response, sent) = call(plans, "2026-04-01", "2027-03-31").await;
    assert_eq!(sent, total, "no report request was sent");
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "negative_stock_too_large", "{response}");
    assert_eq!(error["size"]["master_alter_id"], 875);
    assert_eq!(error["size"]["limit_bytes"], 16_000_000);
    assert_eq!(error["size"]["limit_master_alter_id"], 874);
    assert!(error["remediation"]
        .as_str()
        .unwrap()
        .contains("A larger book refuses"));
}

#[tokio::test]
async fn under_mask_parties_each_item_name_is_masked_and_nothing_else_changes() {
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
    let plain_items = result(&plain)["listed_items"].as_array().unwrap();
    let masked_items = result(&masked)["listed_items"].as_array().unwrap();
    // The capture lists one item: with none, the loop below would run zero times.
    assert_eq!(plain_items.len(), 1);
    assert_eq!(masked_items.len(), 1);
    for (plain, masked) in plain_items.iter().zip(masked_items) {
        let name = plain["item"].as_str().unwrap();
        assert_eq!(masked["item"], json!(crate::agent::mask(name)));
        assert_ne!(masked["item"], plain["item"]);
        assert_eq!(masked["quantity"], plain["quantity"]);
        assert_eq!(masked["rate"], plain["rate"]);
        assert_eq!(masked["value"], plain["value"]);
    }
    assert!(!masked.to_string().contains("Lab Short Item"));
}

#[test]
fn the_measured_window_starts_on_1_april_and_ends_within_that_year() {
    use super::is_measured_window as measured;
    assert!(measured("20260401", "20260531"));
    assert!(measured("20260401", "20270331"));
    assert!(measured("20260401", "20260401"));
    assert!(!measured("20260401", "20270401"));
    assert!(!measured("20260402", "20270331"));
    assert!(!measured("20260501", "20270331"));
    assert!(!measured("20260401", "20250331"));
    assert!(!measured("2026", "2027"));
}
