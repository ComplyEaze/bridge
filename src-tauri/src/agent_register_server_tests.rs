//! `Server::purchase_register` against a scripted transport replaying the bytes of one live run.
//!
//! The responses are Tally's own bytes from one read of the disposable GST lab book (2025-09-03,
//! one Purchase voucher); the three company-listing responses are trimmed to the target book's
//! row (see the PROVENANCE table beside the fixtures). The scripted order and the request
//! fingerprints are the ones recorded on the wire, so a change to the order or shape of Bridge's
//! reads fails here.
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

use super::*;

const COMPANY_GUID: &str = "ae1490be-52c5-4544-9ffc-4b7da85f9797";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Extent,
    Status,
    BookExtent,
    Currencies,
    LedgersCompliance,
    LedgersPaired,
    Groups,
    Census,
    Window,
    Marks,
}

impl Kind {
    fn of(letter: char) -> Self {
        match letter {
            'e' => Self::Extent,
            's' => Self::Status,
            'b' => Self::BookExtent,
            'c' => Self::Currencies,
            'L' => Self::LedgersCompliance,
            'P' => Self::LedgersPaired,
            'g' => Self::Groups,
            'n' => Self::Census,
            'w' => Self::Window,
            'm' => Self::Marks,
            other => panic!("unknown kind {other}"),
        }
    }
}

/// The request kinds Bridge sent, in order, for one call: e company list, s status probe,
/// b book extent, c currencies, L the compliance ledger listing, P the paired ledger listing,
/// g groups, n voucher census, w voucher window, m company marks.
const RECORDED_ORDER: &str = "esesebsbscscsbsbseseebsbsLsLsPsPsgsgsbsbseseensnseewswseemsmseebsbscscsbsbseseebsbsLsLsPsPsgsgsbsbsese";

fn recorded_request_sha256(kind: Kind) -> &'static str {
    match kind {
        Kind::Extent => "9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8",
        Kind::Status => "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        Kind::BookExtent => "2a51f6b170aa9f6ce75be17f747b339c02c6eff0c021ba1f3494b160d7ecd8ad",
        Kind::Currencies => "e35383713c7c8309288b8372cf05f9b91634053e70aa43f282cbdd07e6d61c5e",
        Kind::LedgersCompliance => {
            "9de7fdf791c57b6aa1c8f41aa0da3d0333224860d8b130291002067ca2c52830"
        }
        Kind::LedgersPaired => "36393cad9e54cca34a235ad210d75f5d0522da3010d6623a75f161ef7bf25fc8",
        Kind::Groups => "935cc2ca841b01c4d34af9015cb6d93051bdf282e86ff92fd633b6c51a994716",
        Kind::Census => "4ece541a6950491ea5417e56dbba6c63e9d5fbef413dd3b4f07f53dfc7cf6e54",
        Kind::Window => "34847acdf64088170371e86f5aff33e991753778290dfe47122ff0498a64b90c",
        Kind::Marks => "162901e7f767920bdfabf1a5cfe5e8045deeeb276011245186cdc00492414c5a",
    }
}

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn body_of(kind: Kind) -> String {
    match kind {
        Kind::Extent => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-extent.utf16le.xml")),
        Kind::Status => String::from_utf8(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-status.txt").to_vec()).unwrap(),
        Kind::BookExtent => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-book-extent.utf16le.xml")),
        Kind::Currencies => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-currencies.utf16le.xml")),
        Kind::LedgersCompliance => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-ledgers-compliance.utf16le.xml")),
        Kind::LedgersPaired => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-ledgers-paired.utf16le.xml")),
        Kind::Groups => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-groups.utf16le.xml")),
        Kind::Census => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-census.utf16le.xml")),
        Kind::Window => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-window.utf16le.xml")),
        Kind::Marks => utf16(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/native-register-e2e-marks.utf16le.xml")),
    }
}

fn plan(kind: Kind, body: String) -> ScenarioPlan {
    if kind == Kind::Status {
        return ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    }
    ScenarioPlan::new(Fixture::SyntheticXml(body))
        .with_encoding(WireEncoding::Utf16LeNoBom)
        .with_framing(ResponseFraming::ContentLength)
}

fn recorded_plans() -> Vec<ScenarioPlan> {
    RECORDED_ORDER
        .chars()
        .map(|letter| {
            let kind = Kind::of(letter);
            plan(kind, body_of(kind))
        })
        .collect()
}

fn server_for(
    simulator: &SequenceSimulator,
    directory: &std::path::Path,
    redaction: Redaction,
) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
        data_dir: directory.to_path_buf(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

async fn run(plans: Vec<ScenarioPlan>, from_to: (&str, &str)) -> (Value, usize) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_for(&simulator, directory.path(), Redaction::None);
    let response = server
        .call_tool(
            "purchase_register",
            json!({"company_guid": COMPANY_GUID, "from": from_to.0, "to": from_to.1}),
        )
        .await;
    // Not every scripted answer is used when a read refuses early, so the count is what the
    // server asked for, and the simulator is stopped rather than waited on.
    let received = simulator.received();
    simulator.cancel();
    (response, received)
}

#[tokio::test]
async fn the_register_reads_masters_then_the_window_then_the_marks_then_the_masters_again() {
    let (response, requests) = run(recorded_plans(), ("20250903", "20250903")).await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "complete");
    assert_eq!(result["total"], 1);
    assert_eq!(result["vouchers_observed"], 1);
    assert_eq!(result["items"][0]["status"], "complete");
    assert_eq!(result["items"][0]["party_group"], "Sundry Creditors");
    assert_eq!(result["items"][0]["tax_in_books"][0]["head"], "cgst");
    assert_eq!(result["ledger_masters_observed"], 43);
    assert_eq!(
        requests,
        RECORDED_ORDER.len(),
        "every recorded request was sent, no more"
    );
}

#[tokio::test]
async fn the_sales_register_sends_the_same_requests_and_lists_the_purchase_apart() {
    // The sales register reads exactly as the purchase register does, so the recorded
    // answers of the purchase register's read serve it: the one voucher of that day is a
    // Purchase, which the sales register does not list as a row but names.
    let simulator = SequenceSimulator::spawn(recorded_plans()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_for(&simulator, directory.path(), Redaction::None);
    let response = server
        .call_tool(
            "sales_register",
            json!({"company_guid": COMPANY_GUID, "from": "20250903", "to": "20250903"}),
        )
        .await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["profile"], "agent_sales_register_v1");
    assert_eq!(result["state"], "complete");
    assert_eq!(result["total"], 0);
    assert_eq!(result["vouchers_observed"], 1);
    let other = &result["other_voucher_types_touching_duties_taxes"];
    assert_eq!(other["total"], 1);
    assert_eq!(other["listed"][0]["voucher_class"], "Purchase");
    let observed = simulator.finish().unwrap();
    assert_eq!(observed.len(), RECORDED_ORDER.len());
    for (position, (request, letter)) in observed.iter().zip(RECORDED_ORDER.chars()).enumerate() {
        assert_eq!(
            request.request_body_sha256,
            recorded_request_sha256(Kind::of(letter)),
            "request {position} ({letter}) is not the recorded one"
        );
    }
}

#[tokio::test]
async fn the_requests_sent_are_the_recorded_ones_in_the_recorded_order() {
    // The scripted transport answers by position, so this is what pins the order and shape of
    // Bridge's reads: each request body's fingerprint must be the recorded one.
    let simulator = SequenceSimulator::spawn(recorded_plans()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_for(&simulator, directory.path(), Redaction::None);
    let response = server
        .call_tool(
            "purchase_register",
            json!({"company_guid": COMPANY_GUID, "from": "20250903", "to": "20250903"}),
        )
        .await;
    assert_eq!(response["isError"], false, "{response}");
    let observed = simulator.finish().unwrap();
    assert_eq!(observed.len(), RECORDED_ORDER.len());
    for (position, (request, letter)) in observed.iter().zip(RECORDED_ORDER.chars()).enumerate() {
        assert_eq!(
            request.request_body_sha256,
            recorded_request_sha256(Kind::of(letter)),
            "request {position} ({letter}) is not the recorded one"
        );
    }
}

/// Every occurrence of the company-marks response, with the voucher mark moved by one.
fn marks_with_moved_voucher_mark() -> String {
    let body = body_of(Kind::Marks);
    let start = body.find("<ALTVCHID").unwrap();
    let open_end = start + body[start..].find('>').unwrap() + 1;
    let close = open_end + body[open_end..].find('<').unwrap();
    let mark: u64 = body[open_end..close].trim().parse().unwrap();
    format!("{}{}{}", &body[..open_end], mark + 1, &body[close..])
}

#[tokio::test]
async fn a_voucher_mark_that_moved_while_the_window_was_read_refuses_and_releases_no_rows() {
    let mut plans = recorded_plans();
    let moved = marks_with_moved_voucher_mark();
    for (position, letter) in RECORDED_ORDER.chars().enumerate() {
        if letter == 'm' {
            plans[position] = plan(Kind::Marks, moved.clone());
        }
    }
    let (response, requests) = run(plans, ("20250903", "20250903")).await;
    assert_eq!(response["isError"], true, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "voucher_window_changed_during_read",
        "{response}"
    );
    assert!(
        result.get("items").is_none(),
        "no row is released: {response}"
    );
    // The second masters read is never sent once the window has drifted: the requests stop
    // before the compliance listing that follows the marks in the recorded order.
    let last_marks = RECORDED_ORDER.rfind('m').unwrap();
    let second_listing = last_marks + RECORDED_ORDER[last_marks..].find('L').unwrap();
    assert!(
        requests > last_marks && requests < second_listing,
        "{requests}"
    );
}

#[tokio::test]
async fn a_duty_head_changed_between_the_two_masters_reads_refuses_and_releases_no_rows() {
    // The marks cannot be trusted alone (whether a head change moves them is unmeasured), so
    // the masters are read again: the second listing names another head for the first ledger
    // that carries one, under the same marks.
    let mut plans = recorded_plans();
    let first_marks = RECORDED_ORDER.find('m').unwrap();
    let altered =
        body_of(Kind::LedgersCompliance).replacen(">CGST</GSTDUTYHEAD>", ">IGST</GSTDUTYHEAD>", 1);
    assert_ne!(altered, body_of(Kind::LedgersCompliance));
    for (position, letter) in RECORDED_ORDER.chars().enumerate() {
        if letter == 'L' && position > first_marks {
            plans[position] = plan(Kind::LedgersCompliance, altered.clone());
        }
    }
    let (response, requests) = run(plans, ("20250903", "20250903")).await;
    assert_eq!(response["isError"], true, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(
        result["error"]["code"], "ledger_snapshot_drifted",
        "{response}"
    );
    assert!(
        result.get("items").is_none(),
        "no row is released: {response}"
    );
    assert_eq!(requests, RECORDED_ORDER.len());
}

#[tokio::test]
async fn a_window_whose_dates_the_caller_did_not_ask_for_is_refused() {
    // The recorded answer is for 2025-09-03; asking for another day makes Tally's answer a
    // window it did not honour (and changes the request, so the order is not asserted).
    let (response, _) = run(recorded_plans(), ("20250910", "20250910")).await;
    assert_eq!(response["isError"], true, "{response}");
    assert!(
        response["structuredContent"]["result"]
            .get("items")
            .is_none(),
        "no row is released for a window that was not the one asked for: {response}"
    );
}

#[tokio::test]
async fn the_servers_redaction_setting_masks_the_party_and_every_ledger_of_a_real_response() {
    let simulator = SequenceSimulator::spawn(recorded_plans()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_for(&simulator, directory.path(), Redaction::MaskParties);
    let response = server
        .call_tool(
            "purchase_register",
            json!({"company_guid": COMPANY_GUID, "from": "20250903", "to": "20250903"}),
        )
        .await;
    assert_eq!(response["isError"], false, "{response}");
    let text = response["structuredContent"]["result"].to_string();
    for name in [
        "SYN Supplier Intra (M2)",
        "Input CGST",
        "Input SGST",
        "Purchase - Goods",
    ] {
        assert!(!text.contains(name), "{name} survived masking: {text}");
    }
    let _ = simulator.finish();
}
