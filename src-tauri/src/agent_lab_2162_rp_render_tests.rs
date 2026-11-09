//! LAB ONLY: row 2162-RP's part A request, rendered by the build. This file and its one `mod` line in
//! `agent_import_w18_replay_tests.rs` live on the lab branch `lab/2162-rp-sheet` and are never part of a
//! pull request.
//!
//! The build is run as the W18 replay runs it (`agent_import_w18_replay_tests.rs`), on the pilot lab's own
//! answers, then posted with the approval given, and the one write it sends is kept. The three reads that
//! name the invoice's ledgers (`L`, `P`, `Q`) are the W18 and 21 Oct 2026 whole answers cut to the invoice's
//! ledgers by `rates_cut_to`, as the W18 replay's are. The same route as row 2162-H3's (never pushed), with
//! a remote id no earlier lab row has sent.
//!
//! The test renders `BP/26-27/0010`'s two lines under the number of the W18 N answer, asserts the write
//! equals the request that created `BP/26-27/0010` (the `pilot-lab` fixture) with only that number and the
//! remote id's tail changed, and writes the bytes to `$RP_OUT/I-0040.xml` for the sheet.
use super::*;

/// What the build's number read is answered for on W18's answers (`w18-number-0040`).
const RP_NUMBER: &str = "BP/26-27/0040";
/// A remote id no earlier lab row has sent: the 9 Oct pilot post's, `...0777`, with the tail `0940`.
const RP_REMOTE_ID: &str = "00000000-0000-4000-8000-000000000940";

const TWO_LEGS: [&str; 6] = [
    "BRIDGE Walk-in",
    "BRIDGE Svc 998313 5%",
    "BRIDGE Svc 998314 5%",
    "BRIDGE CGST 2.5%",
    "BRIDGE SGST 2.5%",
    "BRIDGE Round Off",
];

/// `BP/26-27/0010`'s entries with `extra` sales lines added after its two.
fn rp_args(extra: &[(&str, &str)], party: &str, head: &str, round_off: bool) -> Value {
    let mut entries = vec![
        json!({"ledger":"BRIDGE Walk-in","amount":party,"side":"Dr"}),
        json!({"ledger":"BRIDGE Svc 998313 5%","amount":"600.40","side":"Cr"}),
        json!({"ledger":"BRIDGE Svc 998314 5%","amount":"400.40","side":"Cr"}),
    ];
    for (ledger, amount) in extra {
        entries.push(json!({"ledger":ledger,"amount":amount,"side":"Cr"}));
    }
    entries.push(json!({"ledger":"BRIDGE CGST 2.5%","amount":head,"side":"Cr"}));
    entries.push(json!({"ledger":"BRIDGE SGST 2.5%","amount":head,"side":"Cr"}));
    let mut invoice = json!({"voucher_type_name":"BRIDGE Sales","place_of_supply":"Rajasthan"});
    if round_off {
        entries.push(json!({"ledger":"BRIDGE Round Off","amount":"0.16","side":"Cr"}));
        invoice["round_off_ledger"] = json!("BRIDGE Round Off");
    }
    json!({"company_guid":W18_GUID, "vouchers":[{
    "bridge_txn_id":"t1", "date":"2026-08-01", "voucher_type":"Sales", "voucher_number":RP_NUMBER,
    "invoice":invoice, "entries":entries}]})
}

fn rp_plan(letter: char, legs: &'static [&'static str]) -> ScenarioPlan {
    let cut = |bytes: &[u8]| {
        pilot_answer(super::super::super::super::tests::rates_cut_to(
            &captured(bytes),
            legs,
        ))
    };
    match letter {
        'L' => cut(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-ledgers-compliance.utf16le.xml")),
        'P' => cut(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab-w18/w18-ledgers-paired.utf16le.xml")),
        'Q' => cut(include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates-scoped-hsn.utf16le.xml")),
        'G' | 'Z' => pilot_plan(letter),
        other => w18_plan(other),
    }
}

/// The queue's re-read of the invoice's masters after the approval: `pilot_queue_block!` in the parent
/// module, copied here because that macro is defined after this module's parent is declared.
const RP_QUEUE_BLOCK: &str = concat!(
    "ebsbscscsbsbse",
    "see",
    "bsbsLsLsPsPsgsgsbsbs",
    "ese",
    "eQsQse",
    "eTsTse",
    "eNsNse",
    "eCsCse",
    "eOsOse",
    "eGsGse"
);

/// The admission's reads once more after the approval, before the queue's: `PILOT_ORDER` past its build and
/// its reads before the dialog (which equal `W18_BUILD` and `W18_POST_BEFORE_DIALOG`), up to `pilot_queue_block!`.
const RP_AFTER_DIALOG: &str = concat!(
    "ekskseemsmseebsbscscsbsbseseebsbsLsLsPsPsgsgsbsbseseeQsQseeTsTseeNsNseeCsCseeOsOse",
    "seemekskseecscse"
);

/// The build, the post's reads, the approval and the write. The answers after the write do not matter here.
fn rp_order() -> String {
    format!("{W18_BUILD}{W18_POST_BEFORE_DIALOG}{RP_AFTER_DIALOG}{RP_QUEUE_BLOCK}seeewswseewswsemZ")
}

/// Builds `args`, posts it approved, and returns every request sent and the body of the one write.
async fn rp_write(args: Value, legs: &'static [&'static str]) -> (String, Vec<u8>) {
    let order = rp_order();
    let simulator =
        SequenceSimulator::spawn(with_sentinel(order.chars().map(|l| rp_plan(l, legs)).collect()))
            .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address(), directory.path());
    let remote_id = Uuid::parse_str(RP_REMOTE_ID).unwrap();
    let response = TEST_QUALIFIED_VOUCHER_TYPES
        .scope(PILOT_QUALIFIED, async {
            let built = built_or_panic(server.build_import_xml(&args).await);
            let result = &built.payload["result"];
            assert_ne!(result["state"], "refused", "{}", built.payload);
            let batch_id = result["batch_id"].as_str().expect("a batch").to_string();
            SCRIPTED_REMOTE_IDS
                .scope(
                    vec![remote_id],
                    SCRIPTED_APPROVAL.scope(
                        ScriptedApproval::approving(),
                        server.call_tool(
                            "post_import",
                            json!({"company_guid":W18_GUID,"batch_id":batch_id}),
                        ),
                    ),
                )
                .await
        })
        .await;
    let observed = sent(simulator);
    let writes = observed
        .iter()
        .filter(|request| {
            let text = captured(request.request_body.get(2..).unwrap_or_default());
            text.contains("<TALLYREQUEST>Import")
        })
        .collect::<Vec<_>>();
    let kinds = w18_kinds(&observed);
    let code = &response["structuredContent"]["result"]["error"]["code"];
    assert_eq!(writes.len(), 1, "one write; asked {kinds}\nserved {order}\nerror {code}");
    (kinds, writes[0].request_body.clone())
}

#[tokio::test]
#[ignore = "lab only: row 2162-RP"]
async fn rp_the_build_renders_0010_under_the_w18_number_and_a_new_remote_id() {
    let (kinds, body) = rp_write(rp_args(&[], "1051.00", "25.02", true), &TWO_LEGS).await;
    let fixture: &[u8] = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-invoice-post-request.utf16le.xml"
    );
    assert_eq!(sha256_hex(fixture), PILOT_REQUEST_SHA256);
    assert_eq!(&fixture[..2], b"\xff\xfe");
    let expected = captured(&fixture[2..]);
    assert_eq!(expected.matches(">BP/26-27/0010<").count(), 2);
    assert_eq!(expected.matches("00000000-0000-4000-8000-000000000777").count(), 1);
    let expected = expected
        .replace(">BP/26-27/0010<", &format!(">{RP_NUMBER}<"))
        .replace("00000000-0000-4000-8000-000000000777", RP_REMOTE_ID);
    assert_eq!(&body[..2], b"\xff\xfe", "{kinds}");
    assert_eq!(captured(&body[2..]), expected, "{kinds}");
    let out = std::path::PathBuf::from(std::env::var("RP_OUT").expect("RP_OUT names the sheet folder"));
    std::fs::write(out.join("I-0040.xml"), &body).unwrap();
    println!("kinds {kinds}\nsha256 {} bytes {}", sha256_hex(&body), body.len());
}
