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

#[tokio::test]
async fn the_servers_redaction_setting_masks_what_the_sales_register_lists() {
    // Read as a sales register, the recorded day's Purchase is listed apart with the ledger names
    // it touches; with parties masked none of them may survive, and unmasked they are there.
    for (redaction, expect_names) in [(Redaction::None, true), (Redaction::MaskParties, false)] {
        let simulator = SequenceSimulator::spawn(recorded_plans()).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = server_for(&simulator, directory.path(), redaction);
        let response = server
            .call_tool(
                "sales_register",
                json!({"company_guid": COMPANY_GUID, "from": "20250903", "to": "20250903"}),
            )
            .await;
        assert_eq!(response["isError"], false, "{response}");
        let text = response["structuredContent"]["result"].to_string();
        for name in ["Input CGST", "Input SGST"] {
            assert_eq!(text.contains(name), expect_names, "{name}: {text}");
        }
        let _ = simulator.finish();
    }
}

/// The two captured sales-day window requests are the request the code sends for a one-day
/// window, with the lab company and the day substituted: hashed like the recorded purchase
/// window request (UTF-16LE with a byte-order mark), they equal its fingerprint.
#[test]
fn the_captured_sales_day_requests_are_the_window_request_the_code_sends() {
    use sha2::{Digest, Sha256};
    for (bytes, day) in [
        (
            &include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/sales-day/register_window_sales_day_request.utf16le.xml"
            )[..],
            "20250420",
        ),
        (
            &include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/sales-day/register_window_taxed_sales_day_request.utf16le.xml"
            )[..],
            "20250421",
        ),
    ] {
        assert_eq!(&bytes[..2], &[0xff, 0xfe], "a byte-order mark, as sent");
        let text = utf16(&bytes[2..]);
        assert!(text.contains("<SVCURRENTCOMPANY>BRIDGE STOCK LAB</SVCURRENTCOMPANY>"));
        let substituted = text
            .replace("BRIDGE STOCK LAB", "BRIDGE GST RECON LAB")
            .replace(day, "20250903");
        let mut wire = vec![0xff, 0xfe];
        for unit in substituted.encode_utf16() {
            wire.extend(unit.to_le_bytes());
        }
        assert_eq!(
            Sha256::digest(&wire)
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>(),
            recorded_request_sha256(Kind::Window),
            "the {day} request is not the window request the code sends"
        );
    }
}

// A Credit Note day through `sales_register` and a Debit Note day through `purchase_register`,
// each replayed from a live call on one synthetic company (see `note-days/PROVENANCE.md`). The
// scripted transport follows the call's own sequence record: it answers each request, in order,
// with the response bytes the record names, and the fingerprint of every request the code sends
// must be the recorded one.

const NOTE_DAYS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/crates/bridge-tally-protocol/tests/fixtures/note-days"
);

fn note_day_file(name: &str) -> Vec<u8> {
    std::fs::read(format!("{NOTE_DAYS}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn note_day_json(name: &str) -> Value {
    serde_json::from_slice(&note_day_file(name)).unwrap()
}

/// Replays one recorded call and returns the tool's response and its observed requests.
async fn replay_note_day(prefix: &str) -> (Value, Value, Vec<String>) {
    let sequence = note_day_json(&format!("{prefix}_sequence.json"));
    let requests = sequence["requests"].as_array().unwrap();
    assert_eq!(
        requests.len(),
        118,
        "the record lists every request of the call"
    );
    let plans = requests
        .iter()
        .map(|request| {
            if request["method"] == "GET" {
                plan(Kind::Status, String::new())
            } else {
                let fixture = request["fixture"].as_str().unwrap();
                plan(Kind::Window, utf16(&note_day_file(fixture)))
            }
        })
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_for(&simulator, directory.path(), Redaction::None);
    let response = server
        .call_tool(
            sequence["tool"].as_str().unwrap(),
            sequence["arguments"].clone(),
        )
        .await;
    let observed = simulator.finish().unwrap();
    assert_eq!(
        observed.len(),
        requests.len(),
        "every recorded request was sent, no more"
    );
    let mut wrong = Vec::new();
    for (position, (sent, recorded)) in observed.iter().zip(requests).enumerate() {
        let want = recorded["request_sha256"]
            .as_str()
            .unwrap_or_else(|| recorded_request_sha256(Kind::Status));
        if sent.request_body_sha256 != want {
            wrong.push(format!("request {position} is not the recorded one"));
        }
    }
    (response, sequence, wrong)
}

#[tokio::test]
async fn a_credit_note_day_replays_through_the_sales_register_with_the_signs_tally_sent() {
    let (response, _, wrong) = replay_note_day("credit_note_day").await;
    assert!(wrong.is_empty(), "{wrong:?}");
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["profile"], "agent_sales_register_v1");
    assert_eq!(result["state"], "complete");
    assert_eq!(result["total"], 1);
    assert_eq!(result["vouchers_observed"], 1);
    assert_eq!(result["ledger_masters_observed"], 44);
    for list in [
        "other_voucher_types_touching_duties_taxes",
        "unclassified_voucher_type",
        "vouchers_with_unplaced_ledgers",
        "sales_vouchers_without_duties_taxes_entry",
    ] {
        assert_eq!(result[list]["total"], 0, "{list}");
    }
    let row = &result["items"][0];
    assert_eq!(row["voucher_class"], "Credit Note");
    assert_eq!(row["voucher_number"], "1");
    assert_eq!(row["is_invoice"], false);
    assert_eq!(row["status"], "complete");
    assert_eq!(row["party"], "Shape Buyer 1");
    assert_eq!(row["party_group"], "Sundry Debtors");
    // The signs are Tally's: the party is positive and the sales and tax entries negative, the
    // reverse of a Sales row, and nothing is netted or flipped.
    assert_eq!(row["party_entries"][0]["amount"], "1180.00");
    assert_eq!(
        row["party_entries"][0]["bill_allocations"][0]["bill_type"],
        "On Account"
    );
    assert_eq!(row["has_taxable_entry"], true);
    assert_eq!(row["taxable_entries"][0]["ledger"], "Sales - Local");
    assert_eq!(row["taxable_entries"][0]["amount"], "-1000.00");
    let tax = row["tax_in_books"].as_array().unwrap();
    assert_eq!(tax.len(), 2);
    assert_eq!(
        (tax[0]["head"].as_str(), tax[0]["amount"].as_str()),
        (Some("cgst"), Some("-90.00"))
    );
    // This book's state-side head is `state_tax`; the other lab book's is `sgst_utgst`.
    assert_eq!(
        (tax[1]["head"].as_str(), tax[1]["raw_head"].as_str()),
        (Some("state_tax"), Some("State Tax"))
    );
    assert_eq!(tax[1]["amount"], "-90.00");
    assert!(row["other_entries"].as_array().unwrap().is_empty());
    // A Credit Note in voucher view, on account, is a measured kind: no marker.
    assert!(row.get("not_measured_live").is_none(), "{row}");
    // What the live call returned, row for row, apart from the marker the first build
    // put on every credit note and this one no longer puts on this kind.
    let answer = note_day_json("credit_note_day_answer.json");
    let mut live_row = answer["result"]["items"][0].clone();
    assert_eq!(live_row["not_measured_live"], json!(["credit_note"]));
    live_row
        .as_object_mut()
        .unwrap()
        .remove("not_measured_live");
    assert_eq!(row, &live_row);
    assert_eq!(answer["result"]["ledger_masters_observed"], 44);
}

#[tokio::test]
async fn a_debit_note_day_replays_through_the_purchase_register_with_the_signs_tally_sent() {
    let (response, _, wrong) = replay_note_day("debit_note_day").await;
    assert!(wrong.is_empty(), "{wrong:?}");
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["profile"], "agent_purchase_register_v1");
    assert_eq!(result["state"], "complete");
    assert_eq!(result["total"], 1);
    let row = &result["items"][0];
    assert_eq!(row["voucher_class"], "Debit Note");
    assert_eq!(row["status"], "complete");
    assert_eq!(row["party_group"], "Sundry Creditors");
    assert_eq!(row["party_entries"][0]["amount"], "-1180.00");
    assert_eq!(row["taxable_entries"][0]["amount"], "1000.00");
    let tax = row["tax_in_books"].as_array().unwrap();
    assert_eq!(tax.len(), 2);
    assert_eq!(
        (tax[0]["head"].as_str(), tax[0]["amount"].as_str()),
        (Some("cgst"), Some("90.00"))
    );
    assert_eq!(
        (tax[1]["head"].as_str(), tax[1]["amount"].as_str()),
        (Some("state_tax"), Some("90.00"))
    );
    assert!(row.get("not_measured_live").is_none());
    // The purchase register returns exactly what the live call did.
    let answer = note_day_json("debit_note_day_answer.json");
    assert_eq!(row, &answer["result"]["items"][0]);
}

/// Every request file of `note-days/` is a request one of the three calls sent, and every request
/// a call sent has its file: the record's fingerprints and the files' own bytes agree.
#[test]
fn the_note_day_request_files_are_exactly_the_requests_the_three_calls_sent() {
    use sha2::{Digest, Sha256};
    use std::collections::BTreeSet;
    let hex = |bytes: &[u8]| {
        Sha256::digest(bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    };
    let files = std::fs::read_dir(NOTE_DAYS)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.ends_with("_request.utf16le.xml"))
        .collect::<Vec<_>>();
    let mut on_disk = BTreeSet::new();
    for name in &files {
        let bytes = note_day_file(name);
        assert_eq!(
            &bytes[..2],
            &[0xff, 0xfe],
            "{name}: a byte-order mark, as sent"
        );
        on_disk.insert(hex(&bytes));
    }
    assert_eq!(
        on_disk.len(),
        files.len(),
        "no two request files are the same request"
    );
    let mut sent = BTreeSet::new();
    for prefix in [
        "credit_note_day",
        "debit_note_day",
        "cancelled_purchase_day",
    ] {
        let sequence = note_day_json(&format!("{prefix}_sequence.json"));
        sent.extend(
            sequence["requests"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|request| request["request_sha256"].as_str().map(str::to_string)),
        );
    }
    assert_eq!(on_disk, sent);
}

#[tokio::test]
async fn a_cancelled_purchase_is_never_a_row_and_is_listed_with_its_cancelled_flag() {
    // A cancelled voucher keeps no entries, so it has no Duties & Taxes entry and lands in the
    // list of register-class vouchers without one, flagged, rather than among the rows. This
    // is a Purchase read through the purchase register; a cancelled SALE is not measured.
    let (response, _, wrong) = replay_note_day("cancelled_purchase_day").await;
    assert!(wrong.is_empty(), "{wrong:?}");
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "complete");
    assert_eq!(result["total"], 0);
    assert!(result["items"].as_array().unwrap().is_empty());
    assert_eq!(result["vouchers_observed"], 1);
    let listed = &result["purchase_vouchers_without_duties_taxes_entry"];
    assert_eq!(listed["total"], 1);
    assert_eq!(listed["listed"][0]["voucher_class"], "Purchase");
    assert_eq!(listed["listed"][0]["cancelled"], true);
    // What the live call returned, side list for side list.
    let answer = note_day_json("cancelled_purchase_day_answer.json");
    assert_eq!(
        listed,
        &answer["result"]["purchase_vouchers_without_duties_taxes_entry"]
    );
}
