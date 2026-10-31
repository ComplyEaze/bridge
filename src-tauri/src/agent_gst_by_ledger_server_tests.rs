//! `Server::gst_by_counter_ledger` against a scripted transport replaying the bytes of a live run.
//!
//! The responses are Tally's own bytes from the 14 one-day calls of a lab sitting on a synthetic
//! book whose vouchers were typed on TallyPrime 7.2's screen (see the PROVENANCE table beside the
//! fixtures). Every answer but the census and the window was byte-equal across the 14 calls, so it
//! is kept once. The order and the request fingerprints are the ones recorded on the wire, so a
//! change to the order or shape of the reads fails here, and each day's answer is the one the lab
//! item expected of that voucher.
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

use super::*;

const COMPANY_GUID: &str = "aae372c5-a894-491c-8843-5532e43d2d2f";

/// One call's requests, in order: e company list, s status probe, b book extent, c currencies,
/// L the compliance ledger listing, P the paired ledger listing, g groups, n voucher census,
/// w voucher window, m company marks. The same for each of the 14 calls.
const RECORDED_ORDER: &str = "esesebsbscscsbsbseseebsbsLsLsPsPsgsgsbsbseseensnseewswseemsmseebsbscscsbsbseseebsbsLsLsPsPsgsgsbsbsese";

/// The request fingerprints that are the same in every call.
fn recorded_request_sha256(letter: char) -> &'static str {
    match letter {
        'e' => "9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8",
        's' => "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        'b' => "901d0bb233609b7624cc5b769949f7b8c543a1b86eff17eec7fa59b85b0d05f0",
        'c' => "503b60c62d24438b6df782f9a438f5eb79227f52b4727116f71c4e53fee12e6a",
        'L' => "073be178727d46a46ffeb10fcc9c556a1503dba388b849ec814cd974c638d52d",
        'P' => "6c8e5b96f83eefacac46f350ca5c3cb57fd5b0ecf6ef385c743af8e792aac9c2",
        'g' => "f2ca16e6a52141f81e384f20ce7d1fb2265d1eb608108bbf6f4ce80062c3db3a",
        'm' => "7a298350bd3c2c1f9809881b07fad6047195bb2108e12c2b67c6271d5d69f74a",
        other => panic!("kind {other} differs by day"),
    }
}

/// One recorded day: its census and window answers and the fingerprints of the two requests.
struct Day {
    date: &'static str,
    census: &'static [u8],
    window: &'static [u8],
    census_request: &'static str,
    window_request: &'static str,
}

const DAYS: [Day; 14] = [
    Day {
        date: "20251103",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-w-1.utf16le.xml"
        ),
        census_request: "f1d00a414e67ecadf79ef940a6d2fb5655e5979ed5f1b89f2473ea882fee31e7",
        window_request: "ef037e7d1430b2400c8ac71f32430d0187bb96487eec49da9438dd066bf835cb",
    },
    Day {
        date: "20251104",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-04nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-04nov-w-1.utf16le.xml"
        ),
        census_request: "2b4d7db0c8ae9927c54aa27ab75548bbd3bc4fdfb48c12b88d33fd55d3a473b6",
        window_request: "2c3a278b46acf9fa80937a4d190fcb49aed3689a4eb4f640a58db4519dc42bd6",
    },
    Day {
        date: "20251105",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-05nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-05nov-w-1.utf16le.xml"
        ),
        census_request: "4524cd012cee5c27e74bd5666c7962069d143ae2fa287273f1b59b501796a746",
        window_request: "f211010e3762b68157e91cecdbbfa6a0aa4dffb3ea0e4659657036af3ee02c52",
    },
    Day {
        date: "20251106",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-06nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-06nov-w-1.utf16le.xml"
        ),
        census_request: "3ccc9f76bb4013ea4aef23e6b62e9470cb0a469ee85279dbcba1bc8e690e21b8",
        window_request: "7710b53a5d652b9eba36b94784ed87185b177bf941ef25d7ce1e25e0444b1d80",
    },
    Day {
        date: "20251107",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-07nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-07nov-w-1.utf16le.xml"
        ),
        census_request: "e128563c16739d4b18423a15031e5f4a3bd0563141ffc477fd68bc2aeb23a5b8",
        window_request: "fee1dcba3d4ad33dbd7da06dc12f2a5bd809036e53d7fc16447ebfffc33be641",
    },
    Day {
        date: "20251108",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-08nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-08nov-w-1.utf16le.xml"
        ),
        census_request: "b9b54a3d0f382a679b457b82142c2e51fc491cb1d6f12463e8ccd4447ab10f84",
        window_request: "e7916596e128d596e151861cb5bf36b65dfaa904452a4dee53d95f247c58c02b",
    },
    Day {
        date: "20251110",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-10nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-10nov-w-1.utf16le.xml"
        ),
        census_request: "e21ae27586e5bf728fcc79483849efda3c3c1f75239e00dd3092a87c074435ad",
        window_request: "77712fe7af5bb7bf47200159a228cadd4c3b31b04e02b0180c950cf0adc463c1",
    },
    Day {
        date: "20251111",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-11nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-11nov-w-1.utf16le.xml"
        ),
        census_request: "aaf4c6a046e16d234f97fbb1c6005884703acbae901f0d33e348540e01cea3fe",
        window_request: "3ee126e3f2d8f94fc0ab862d3f58a28fbc121fb8cc75eb5731d5b78613a893e1",
    },
    Day {
        date: "20251112",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-12nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-12nov-w-1.utf16le.xml"
        ),
        census_request: "80bdb1914af4375cac19b3a1fcdd20d6a2992b9e34453bf2390fdf8b002706c6",
        window_request: "41d67b0a603653832be1f86efb143284c502aaeae025e7b44fb07d6d04bdf3f5",
    },
    Day {
        date: "20251113",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-13nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-13nov-w-1.utf16le.xml"
        ),
        census_request: "df2375100f4bc895f2069d288fb239874d947387dbc5ab8c366771117a5f266e",
        window_request: "f821cd7178d275639d128d60e1d1b966289e55ce1c44ab5e7f9f7f4e0ee97e95",
    },
    Day {
        date: "20251114",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-14nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-14nov-w-1.utf16le.xml"
        ),
        census_request: "58eb1809a808df4ab3d73b608c9a14d7027714979bc8f35d48b2e46f42ec2872",
        window_request: "9328b689ae8d325950fed577e2125919087fc3a9d0fd1b024c96b944c7c48000",
    },
    Day {
        date: "20251115",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-15nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-15nov-w-1.utf16le.xml"
        ),
        census_request: "f4d7c65190446e9075536ce2b729a9f3a7a0d8e042aad06994dc183f43ebfbc9",
        window_request: "efed621d9721576a76af65081522aa6c1db06f6f9d424fa8d6809b83a3459d1f",
    },
    Day {
        date: "20251116",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-16nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-16nov-w-1.utf16le.xml"
        ),
        census_request: "d8fc61f234141946e62a9aa9661553f64ad4c7558e2864e2d9364f54cb4a2de1",
        window_request: "15df0de11bd3d06b031f6fa8559f06119fd22e8ab180196c42498dba72327d47",
    },
    Day {
        date: "20251117",
        census: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-17nov-n-1.utf16le.xml"
        ),
        window: include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-17nov-w-1.utf16le.xml"
        ),
        census_request: "cf33a7bc03efc87d7dcbe454a48258b6cc95007a29b6708a7673759d2707096c",
        window_request: "9da0cbf78e8ba62546d689f4d3a27c8af2d5530e208edaab8bcf455b4806cf16",
    },
];

fn utf16(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn shared_body(letter: char) -> String {
    utf16(match letter {
        'e' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-e-1.utf16le.xml").as_slice(),
        'b' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-b-1.utf16le.xml").as_slice(),
        'c' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-c-1.utf16le.xml").as_slice(),
        'L' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-L-1.utf16le.xml").as_slice(),
        'P' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-P-1.utf16le.xml").as_slice(),
        'g' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-g-1.utf16le.xml").as_slice(),
        'm' => include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/gst-ledger-l1/s6-03nov-m-1.utf16le.xml").as_slice(),
        other => panic!("kind {other} differs by day"),
    })
}

fn plans_for(day: &Day) -> Vec<ScenarioPlan> {
    RECORDED_ORDER
        .chars()
        .map(|letter| {
            if letter == 's' {
                return ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
            }
            let body = match letter {
                'n' => utf16(day.census),
                'w' => utf16(day.window),
                _ => shared_body(letter),
            };
            ScenarioPlan::new(Fixture::SyntheticXml(body))
                .with_encoding(WireEncoding::Utf16LeNoBom)
                .with_framing(ResponseFraming::ContentLength)
        })
        .collect()
}

fn request_sha256(day: &Day, letter: char) -> &'static str {
    match letter {
        'n' => day.census_request,
        'w' => day.window_request,
        _ => recorded_request_sha256(letter),
    }
}

/// Replays one day and checks every request against the recorded one.
async fn replay(day: &Day) -> Value {
    let simulator = SequenceSimulator::spawn(plans_for(day)).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        posting: Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let response = server
        .call_tool(
            "gst_by_counter_ledger",
            json!({"company_guid": COMPANY_GUID, "from": day.date, "to": day.date}),
        )
        .await;
    let observed = simulator.finish().unwrap();
    assert_eq!(observed.len(), RECORDED_ORDER.len(), "{}", day.date);
    for (position, (request, letter)) in observed.iter().zip(RECORDED_ORDER.chars()).enumerate() {
        assert_eq!(
            request.request_body_sha256,
            request_sha256(day, letter),
            "{} request {position} ({letter}) is not the recorded one",
            day.date
        );
    }
    assert_eq!(response["isError"], false, "{}: {response}", day.date);
    response["structuredContent"]["result"].clone()
}

fn heads(value: &Value) -> Vec<(String, String)> {
    value
        .as_object()
        .unwrap()
        .iter()
        .map(|(head, amount)| (head.clone(), amount.as_str().unwrap().to_string()))
        .collect()
}

fn pairs(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|(head, amount)| ((*head).to_string(), (*amount).to_string()))
        .collect()
}

/// What each day's one voucher must come to: attributed to a ledger of a group, or set aside
/// under a reason, with its debit-side and credit-side tax.
enum Expect {
    Attributed(&'static str, &'static str),
    SetAside(&'static str),
}

/// A day's date, what its voucher must come to, and its debit-side and credit-side tax by head.
type DayExpectation = (
    &'static str,
    Expect,
    Vec<(String, String)>,
    Vec<(String, String)>,
);

#[tokio::test]
async fn each_day_typed_on_tallys_screen_reads_as_the_lab_item_expected() {
    use Expect::{Attributed, SetAside};
    let state = |amount| [("cgst", amount), ("sgst_utgst", amount)];
    let expected: [DayExpectation; 14] = [
        // V1, a purchase to an Indirect Expenses ledger.
        (
            "20251103",
            Attributed("GS1 Rent", "Indirect Expenses"),
            pairs(&state("-900")),
            vec![],
        ),
        // V4, a journal: an expense with input IGST and the supplier credited.
        (
            "20251104",
            Attributed("GS1 Professional Fees", "Indirect Expenses"),
            pairs(&[("igst", "-3600")]),
            vec![],
        ),
        // V5 and V6, purchases to a Fixed Assets and a Direct Expenses ledger: admitted on these captures.
        (
            "20251105",
            Attributed("GS1 Computer", "Fixed Assets"),
            pairs(&state("-3600")),
            vec![],
        ),
        (
            "20251106",
            Attributed("GS1 Freight Direct", "Direct Expenses"),
            pairs(&state("-180")),
            vec![],
        ),
        // V7, one purchase to two expense ledgers: nothing is apportioned.
        (
            "20251107",
            SetAside("several_counter_ledgers"),
            pairs(&state("-900")),
            vec![],
        ),
        // V4b, a journal with a TDS ledger typed Others, which may be a GST ledger left untyped.
        (
            "20251108",
            SetAside("duty_ledger_without_gst_type"),
            pairs(&[("igst", "-1800")]),
            vec![],
        ),
        // V8, a debit note: credit-side tax on the purchase ledger.
        (
            "20251110",
            Attributed("GS1 Rent", "Indirect Expenses"),
            vec![],
            pairs(&state("90")),
        ),
        // V9, a set-off journal.
        (
            "20251111",
            SetAside("gst_on_both_sides"),
            pairs(&[("cgst", "-500")]),
            pairs(&[("cgst", "500")]),
        ),
        // V10, a reversal of input tax with no supplier on it.
        (
            "20251112",
            SetAside("no_creditor_party"),
            vec![],
            pairs(&state("90")),
        ),
        // V11, a tax payment.
        (
            "20251113",
            SetAside("no_creditor_party"),
            pairs(&state("-100")),
            vec![],
        ),
        // V2, an item invoice whose purchase ledger is among its entries.
        (
            "20251114",
            Attributed("Purchase - Goods", "Purchase Accounts"),
            pairs(&state("-450")),
            vec![],
        ),
        // V13, a tax payment that also pays interest: never attributed to the interest ledger.
        (
            "20251115",
            SetAside("no_creditor_party"),
            pairs(&state("-90")),
            vec![],
        ),
        // V14, a credit note with output tax: never attributed to the discount ledger.
        (
            "20251116",
            SetAside("class_not_admitted"),
            pairs(&state("-90")),
            vec![],
        ),
        // V3, an item invoice with freight: two counter ledgers.
        (
            "20251117",
            SetAside("several_counter_ledgers"),
            pairs(&state("-450")),
            vec![],
        ),
    ];
    for (day, (date, expect, debit, credit)) in DAYS.iter().zip(expected) {
        assert_eq!(day.date, date);
        let result = replay(day).await;
        assert_eq!(result["state"], "complete", "{date}: {result}");
        assert_eq!(result["vouchers_observed"], 1, "{date}: {result}");
        assert_eq!(result["vouchers_without_gst_entry"], 0, "{date}: {result}");
        let ledgers = result["counter_ledgers"].as_array().unwrap();
        let set_aside = result["set_aside"].as_object().unwrap();
        match expect {
            Attributed(ledger, group) => {
                assert_eq!(ledgers.len(), 1, "{date}: {result}");
                assert!(set_aside.is_empty(), "{date}: {result}");
                let item = &ledgers[0];
                assert_eq!(item["ledger"], ledger, "{date}");
                assert_eq!(item["group"], group, "{date}");
                assert_eq!(item["vouchers"], 1, "{date}");
                assert!(
                    item["ledger_guid"]
                        .as_str()
                        .is_some_and(|guid| guid.starts_with(COMPANY_GUID)),
                    "{date}: a ledger the book created carries the company's GUID prefix: {item}"
                );
                assert_eq!(heads(&item["debit_side_tax"]), debit, "{date}");
                assert_eq!(heads(&item["credit_side_tax"]), credit, "{date}");
            }
            SetAside(reason) => {
                assert!(ledgers.is_empty(), "{date}: {result}");
                assert_eq!(set_aside.len(), 1, "{date}: {result}");
                let bucket = &set_aside[reason];
                assert_eq!(bucket["vouchers"], 1, "{date}: {result}");
                assert_eq!(heads(&bucket["debit_side_tax"]), debit, "{date}");
                assert_eq!(heads(&bucket["credit_side_tax"]), credit, "{date}");
            }
        }
    }
}

#[tokio::test]
async fn the_same_ledger_keeps_its_guid_across_days() {
    // V1 and V8 both land on GS1 Rent: the key a caller joins on is the same in both answers.
    let first = replay(&DAYS[0]).await;
    let eighth = replay(&DAYS[6]).await;
    assert_eq!(DAYS[6].date, "20251110");
    assert_eq!(
        first["counter_ledgers"][0]["ledger_guid"],
        eighth["counter_ledgers"][0]["ledger_guid"]
    );
}
