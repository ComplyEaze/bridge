//! Replay observed source shapes, then inject a concurrent voucher edit.
use super::*;
use tally_protocol_simulator::{
    Delivery, Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator,
    WireEncoding,
};

fn captured(bytes: &[u8]) -> ScenarioPlan {
    let words = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    ScenarioPlan::new(Fixture::SyntheticXml(String::from_utf16(&words).unwrap()))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

fn captured_utf8(body: &str) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(body.to_owned()))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

#[tokio::test]
async fn movement_refuses_voucher_changes_even_when_period_openings_match() {
    let company = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ));
    let extent = captured_utf8(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    ));
    let ledger = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-period-opening.utf16le.xml"
    ));
    // The movement read proves the book keeps one Currency master (#716).
    let currency = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let voucher = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    ));
    // A small synthetic mark for this company, in the shape the import tests
    // already replay: three vouchers cannot exceed the window budget.
    let high_water = captured_utf8(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY><GUID>61c6de69-1748-461c-ad3f-162cb949df9f</GUID><ALTVCHID>3</ALTVCHID><ALTMSTID>7</ALTMSTID></COMPANY></COLLECTION></DATA></BODY></ENVELOPE>",
    );
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    let opening = || {
        vec![
            status.clone(),
            company.clone(),
            company.clone(),
            extent.clone(),
            status.clone(),
            extent.clone(),
            status.clone(),
            currency.clone(),
            status.clone(),
            currency.clone(),
            status.clone(),
            ledger.clone(),
            status.clone(),
            ledger.clone(),
            status.clone(),
            extent.clone(),
            status.clone(),
            extent.clone(),
            status.clone(),
            company.clone(),
            status.clone(),
            company.clone(),
        ]
    };
    let voucher_read = |body: ScenarioPlan| {
        vec![
            company.clone(),
            body.clone(),
            status.clone(),
            body,
            status.clone(),
            company.clone(),
        ]
    };
    let original = voucher.fixture.body();
    let start = original.find("<VOUCHER ").unwrap();
    let end = start + original[start..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let mut reduced = original.to_string();
    reduced.replace_range(start..end, "");
    // The `stable` read's result, which the `zero` read must equal (#1251).
    let mut stable_result = None;
    for change in [
        "stable",
        "zero",
        "edit",
        "posting",
        "deletion",
        "incomplete",
    ] {
        let changed = !matches!(change, "stable" | "zero");
        let mut before = voucher.clone();
        let mut after = voucher.clone();
        // Inject concurrent changes into captured rows in memory. The source
        // captures stay byte-exact; no live concurrency experiment is claimed.
        match change {
            "zero" => {
                // One more entry on the selected ledger whose amount is `-0.00` and which Tally flags
                // deemed-positive, in both paired reads. A zero adds nothing to either column and
                // nothing to the voucher's balance, so the read must succeed with the same figures.
                let zero_entry = "<ALLLEDGERENTRIES.LIST><LEDGERNAME TYPE=\"String\">WR2 Sales</LEDGERNAME><ISDEEMEDPOSITIVE TYPE=\"Logical\">Yes</ISDEEMEDPOSITIVE><AMOUNT TYPE=\"Amount\">-0.00</AMOUNT></ALLLEDGERENTRIES.LIST>";
                let at = original.find("</ALLLEDGERENTRIES.LIST>").unwrap()
                    + "</ALLLEDGERENTRIES.LIST>".len();
                let mut with_zero = original.to_string();
                with_zero.insert_str(at, zero_entry);
                assert_ne!(with_zero, original);
                before.fixture = Fixture::SyntheticXml(with_zero.clone());
                after.fixture = Fixture::SyntheticXml(with_zero);
            }
            "edit" => {
                let altered = original.replace("-101.01", "-201.01").replace(
                    "<AMOUNT TYPE=\"Amount\">101.01</AMOUNT>",
                    "<AMOUNT TYPE=\"Amount\">201.01</AMOUNT>",
                );
                assert_ne!(altered, original);
                after.fixture = Fixture::SyntheticXml(altered);
            }
            "posting" => before.fixture = Fixture::SyntheticXml(reduced.clone()),
            "deletion" => after.fixture = Fixture::SyntheticXml(reduced.clone()),
            "incomplete" => {
                // Omit one balancing side in both paired responses. Keep the
                // selected WR2 Sales entry, so selection cannot hide refusal.
                let mut omitted = original.to_string();
                let start = omitted.find("<ALLLEDGERENTRIES.LIST>").unwrap();
                let end = start
                    + omitted[start..].find("</ALLLEDGERENTRIES.LIST>").unwrap()
                    + "</ALLLEDGERENTRIES.LIST>".len();
                omitted.replace_range(start..end, "");
                before.fixture = Fixture::SyntheticXml(omitted);
            }
            _ => {}
        }
        let mut plans = vec![
            company.clone(),
            status.clone(),
            company.clone(),
            status.clone(),
        ];
        plans.extend(opening());
        // The pre-flight high-water read (protocol reference §11c) precedes the
        // first window read only; the closing read replays its ranges.
        plans.extend(voucher_read(high_water.clone()));
        plans.extend(voucher_read(before));
        if change != "incomplete" {
            plans.extend(opening());
            plans.extend(voucher_read(after));
        }
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(Settings {
            endpoint: TallyEndpointConfig {
                host: simulator.address().ip().to_string(),
                port: simulator.address().port(),
            },
            data_dir: directory.path().to_path_buf(),
            max_rows: 10,
            max_bytes: 200_000,
            redaction: Redaction::None,
            import_enabled: false,
            writes_enabled: false,
            batch_post_enabled: false,
        });
        let response = server
            .call_tool(
                "ledger_movement",
                json!({
                    "company_guid":"61c6de69-1748-461c-ad3f-162cb949df9f",
                    "from":"20260801", "to":"20260802", "ledger":"WR2 Sales"
                }),
            )
            .await;
        if changed {
            assert_eq!(response["isError"], true, "{response}");
            assert_eq!(
                response["structuredContent"]["result"]["error"]["code"],
                if change == "incomplete" {
                    "voucher_entries_unbalanced"
                } else {
                    "voucher_snapshot_drifted"
                },
                "{response}"
            );
            assert_eq!(
                response["structuredContent"]["evidence"]["state"],
                "partial"
            );
            assert!(
                response["structuredContent"]["evidence"]["bytes"]
                    .as_u64()
                    .unwrap()
                    > 0
            );
        } else {
            assert_eq!(response["isError"], false, "{response}");
            assert_eq!(
                response["structuredContent"]["evidence"]["state"],
                "complete"
            );
            assert_eq!(
                response["structuredContent"]["result"]["voucher_rows_observed"],
                3
            );
            // #1076: the answer says which ledger it read and how.
            let ledger_match = &response["structuredContent"]["result"]["ledger_match"];
            assert_eq!(ledger_match["ledger"], "WR2 Sales", "{ledger_match}");
            assert_eq!(ledger_match["matched"], "exact", "{ledger_match}");
            let result = response["structuredContent"]["result"].clone();
            match change {
                "stable" => stable_result = Some(result),
                // A zero entry on the selected ledger changes no figure.
                _ => assert_eq!(Some(&result), stable_result.as_ref(), "{change}"),
            }
        }
        let observations = simulator.finish().unwrap();
        assert_eq!(
            observations.len(),
            // Each opening read now includes its paired currency read (#716).
            if change == "incomplete" { 38 } else { 66 }
        );
    }
}

/// `count` copies of the captured response's first voucher, AlterIDs `first..`,
/// each with its own GUID, `REMOTEID`, AlterID and master ID, on `date`.
fn many_vouchers(original: &str, ids: std::ops::RangeInclusive<u64>, date: &str) -> String {
    let start = original.find("<VOUCHER ").unwrap();
    let end = start + original[start..].find("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let last = original.rfind("</VOUCHER>").unwrap() + "</VOUCHER>".len();
    let template = &original[start..end];
    let guid = "61c6de69-1748-461c-ad3f-162cb949df9f";
    let body = ids
        .map(|id| {
            template
                .replace(&format!("{guid}-00000001"), &format!("{guid}-{id:08x}"))
                .replace(
                    "<ALTERID TYPE=\"Number\"> 1</ALTERID>",
                    &format!("<ALTERID TYPE=\"Number\"> {id}</ALTERID>"),
                )
                .replace(
                    "<MASTERID TYPE=\"Number\"> 1</MASTERID>",
                    &format!("<MASTERID TYPE=\"Number\"> {id}</MASTERID>"),
                )
                .replace(
                    "<DATE TYPE=\"Date\">20260801</DATE>",
                    &format!("<DATE TYPE=\"Date\">{date}</DATE>"),
                )
        })
        .collect::<String>();
    format!("{}{body}{}", &original[..start], &original[last..])
}

#[tokio::test]
async fn a_divided_movement_refuses_a_posting_above_the_first_reads_ceiling() {
    // #520 P1, end to end. Day one holds 200 vouchers, more than one read at the
    // movement default carries, so the window (that one day) is read in AlterID
    // spans of it, `(0,170]` and `(170,200]`. Between the first read and
    // its replay a voucher is posted on day one and takes AlterID 201, above
    // the replayed spans' ceiling: the replayed responses are byte-identical to
    // the first read's and the snapshots match. Before the fix the replay read
    // no mark and the movement was returned with the posting in neither read.
    let company = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ));
    let extent = captured_utf8(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    ));
    let ledger = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-period-opening.utf16le.xml"
    ));
    // The movement read proves the book keeps one Currency master (#716).
    let currency = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let voucher = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    ));
    let marks = |vouchers: u64| {
        captured_utf8(&format!(
            "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY><GUID>61c6de69-1748-461c-ad3f-162cb949df9f</GUID><ALTVCHID>{vouchers}</ALTVCHID><ALTMSTID>7</ALTMSTID></COMPANY></COLLECTION></DATA></BODY></ENVELOPE>"
        ))
    };
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    let opening = || {
        vec![
            status.clone(),
            company.clone(),
            company.clone(),
            extent.clone(),
            status.clone(),
            extent.clone(),
            status.clone(),
            currency.clone(),
            status.clone(),
            currency.clone(),
            status.clone(),
            ledger.clone(),
            status.clone(),
            ledger.clone(),
            status.clone(),
            extent.clone(),
            status.clone(),
            extent.clone(),
            status.clone(),
            company.clone(),
            status.clone(),
            company.clone(),
        ]
    };
    let voucher_read = |body: ScenarioPlan| {
        vec![
            company.clone(),
            body.clone(),
            status.clone(),
            body,
            status.clone(),
            company.clone(),
        ]
    };
    let original = voucher.fixture.body().into_owned();
    let with = |xml: String| {
        let mut plan = voucher.clone();
        plan.fixture = Fixture::SyntheticXml(xml);
        plan
    };
    let census = with(many_vouchers(&original, 1..=200, "20260801"));
    let first_span = with(many_vouchers(&original, 1..=170, "20260801"));
    let second_span = with(many_vouchers(&original, 171..=200, "20260801"));
    for (replay_mark, expect_ok) in [(200, true), (201, false)] {
        let mut plans = vec![
            company.clone(),
            status.clone(),
            company.clone(),
            status.clone(),
        ];
        plans.extend(opening());
        plans.extend(voucher_read(marks(200)));
        plans.extend(voucher_read(census.clone()));
        let parts = [first_span.clone(), second_span.clone()];
        for part in &parts {
            plans.extend(voucher_read(part.clone()));
        }
        plans.extend(voucher_read(marks(200)));
        plans.extend(opening());
        for part in &parts {
            plans.extend(voucher_read(part.clone()));
        }
        plans.extend(voucher_read(marks(replay_mark)));
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(Settings {
            endpoint: TallyEndpointConfig {
                host: simulator.address().ip().to_string(),
                port: simulator.address().port(),
            },
            data_dir: directory.path().to_path_buf(),
            max_rows: 10,
            max_bytes: 16_000_000,
            redaction: Redaction::None,
            import_enabled: false,
            writes_enabled: false,
            batch_post_enabled: false,
        });
        let response = server
            .call_tool(
                "ledger_movement",
                json!({
                    "company_guid":"61c6de69-1748-461c-ad3f-162cb949df9f",
                    "from":"20260801", "to":"20260801", "ledger":"WR2 Sales"
                }),
            )
            .await;
        if expect_ok {
            assert_eq!(response["isError"], false, "{response}");
            assert_eq!(
                response["structuredContent"]["result"]["voucher_rows_observed"],
                200
            );
            // A quick call's result is unchanged: no block, nothing left out (#1239).
            let result = &response["structuredContent"]["result"];
            assert!(result.get("read_cost").is_none(), "{result}");
            assert!(result.get("read_cost_left_out").is_none(), "{result}");
        } else {
            assert_eq!(response["isError"], true, "{response}");
            assert_eq!(
                response["structuredContent"]["result"]["error"]["code"],
                "voucher_window_changed_during_read",
                "{response}"
            );
            assert_eq!(
                response["structuredContent"]["evidence"]["state"],
                "partial"
            );
        }
        simulator.finish().unwrap();
    }
}

// -- bridge#716: a named ledger's movement names no currency ----------------

async fn movement_call(plans: Vec<ScenarioPlan>, guid: &str, ledger: &str) -> (Value, usize) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    let response = server
        .call_tool(
            "ledger_movement",
            json!({"company_guid": guid, "from":"20260915", "to":"20260915", "ledger": ledger}),
        )
        .await;
    (response, simulator.finish().unwrap().len())
}

/// Identity, then the movement's first read up to and including its paired
/// currency read, which the refusal ends on.
fn opening_through_currency(extent: ScenarioPlan, currency: ScenarioPlan) -> Vec<ScenarioPlan> {
    let company = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ));
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    vec![
        company.clone(),
        status.clone(),
        company.clone(),
        status.clone(),
        status.clone(),
        company.clone(),
        company,
        extent.clone(),
        status.clone(),
        extent,
        status.clone(),
        currency.clone(),
        status.clone(),
        currency,
        status,
    ]
}

/// A movement on the captured several-currency book is refused after its
/// currency read, before any ledger or voucher request: an opening and a
/// movement name no currency, so a dollar ledger's figures would carry
/// nothing to say they are not rupees (#716). Named here is a dollar ledger
/// of that book; a rupee ledger is refused the same way.
#[tokio::test]
async fn a_movement_on_a_several_currency_book_is_refused_before_any_ledger() {
    let plans = opening_through_currency(
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/company_extents_forex_live.utf16le.xml"
        )),
        captured(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_multi_live.utf16le.xml"
        )),
    );
    let total = plans.len();
    let (response, requests) = movement_call(
        plans,
        "b14e9b2d-8a63-4779-804d-25d59eb787eb",
        "FX USD Debtor 01",
    )
    .await;
    assert_eq!(requests, total, "no ledger or voucher request was sent");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(error["code"], "ledger_movement_read_failed");
    assert_eq!(error["cause"], "company_several_currency_masters");
    let remediation = error["remediation"].as_str().unwrap();
    assert!(remediation.contains("ledger_movement"), "{error}");
}

/// A movement whose currency collection holds no master is refused after
/// it, before any ledger or voucher request. DERIVED from the captured
/// single-master response with its one `CURRENCY` element removed (#716).
#[tokio::test]
async fn a_movement_with_no_currency_master_is_refused_before_any_ledger() {
    let single = String::from_utf16(
        &include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        )
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>(),
    )
    .unwrap();
    let start = single.find("<CURRENCY ").unwrap();
    let end = start + single[start..].find("</CURRENCY>").unwrap() + "</CURRENCY>".len();
    let mut none = single.clone();
    none.replace_range(start..end, "");
    assert!(!none.contains("<CURRENCY "), "no master left");
    let plans = opening_through_currency(
        captured_utf8(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )),
        captured_utf8(&none),
    );
    let total = plans.len();
    let (response, requests) =
        movement_call(plans, "61c6de69-1748-461c-ad3f-162cb949df9f", "WR2 Sales").await;
    assert_eq!(requests, total, "no ledger or voucher request was sent");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(error["cause"], "company_currency_probe_failed");
}

/// A movement on a book whose one Currency master is not INR is refused after
/// its currency read, before any ledger or voucher request (#716). DERIVED
/// from the captured single-master response with its `MAILINGNAME` changed
/// from `INR`; no non-INR book has been captured.
#[tokio::test]
async fn a_movement_on_a_non_inr_book_is_refused_before_any_ledger() {
    let single = String::from_utf16(
        &include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
        )
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>(),
    )
    .unwrap();
    let inr = "<MAILINGNAME TYPE=\"String\">INR</MAILINGNAME>";
    assert_eq!(single.matches(inr).count(), 1);
    let foreign = single.replace(inr, "<MAILINGNAME TYPE=\"String\">UAE Dirham</MAILINGNAME>");
    let plans = opening_through_currency(
        captured_utf8(include_str!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
        )),
        captured_utf8(&foreign),
    );
    let total = plans.len();
    let (response, requests) =
        movement_call(plans, "61c6de69-1748-461c-ad3f-162cb949df9f", "WR2 Sales").await;
    assert_eq!(requests, total, "no ledger or voucher request was sent");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(error["code"], "ledger_movement_read_failed");
    assert_eq!(error["cause"], "company_base_currency_not_inr");
}

// -- The movement's ledger catalogue read: one attempt, a typed cause, and the
// -- ledger named before any voucher is read.

/// Identity, then the movement's first ledger read to its end: the read's own
/// brackets, its paired currency read, its paired export (positions 15 and 17)
/// and its closing brackets. The same sequence the happy-path tests replay.
const FIRST_EXPORT: usize = 15;

fn first_ledger_read() -> Vec<ScenarioPlan> {
    let company = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ));
    let extent = captured_utf8(include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    ));
    let ledger = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-period-opening.utf16le.xml"
    ));
    let currency = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/currency_inr_modern_live.utf16le.xml"
    ));
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    vec![
        company.clone(),
        status.clone(),
        company.clone(),
        status.clone(),
        status.clone(),
        company.clone(),
        company.clone(),
        extent.clone(),
        status.clone(),
        extent.clone(),
        status.clone(),
        currency.clone(),
        status.clone(),
        currency,
        status.clone(),
        ledger.clone(),
        status.clone(),
        ledger,
        status.clone(),
        extent.clone(),
        status.clone(),
        extent,
        status.clone(),
        company.clone(),
        status,
        company,
    ]
}

fn movement_server(
    simulator: &SequenceSimulator,
    policy: bridge_tally_transport::TransportPolicy,
) -> (Server, tempfile::TempDir) {
    let directory = tempfile::tempdir().unwrap();
    let mut server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: simulator.address().ip().to_string(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    server.runtime = TallyRuntime::with_transport_policy(policy);
    (server, directory)
}

/// A ledger catalogue export that outlives its deadline is sent once (a 2 s deadline,
/// a 5 s stall, and a wait past both: a request that is slow enough to fail the
/// deadline on a loaded machine fails this test rather than passing it). The
/// script holds a whole second read behind the stalled one, so a retry would
/// find plans to answer it, and `received` would count them. Agent voucher reads
/// are single-attempt because a request that timed out queues more work behind
/// a gateway still building the abandoned response; this read is the same.
#[tokio::test]
async fn a_movement_catalogue_read_that_times_out_is_sent_once_and_names_why() {
    let busy = std::time::Duration::from_millis(5_000);
    let mut plans = first_ledger_read();
    plans[FIRST_EXPORT] = plans[FIRST_EXPORT]
        .clone()
        .with_delivery(Delivery::SlowHeaders(busy));
    let stalled = FIRST_EXPORT + 1;
    // Directly behind the stalled export: a whole read (its brackets, currency
    // read and export), which is what a retry would send next.
    plans.truncate(stalled);
    plans.extend(first_ledger_read().into_iter().skip(4));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let (server, _directory) = movement_server(
        &simulator,
        bridge_tally_transport::TransportPolicy {
            request_timeout: std::time::Duration::from_millis(2_000),
            ..Default::default()
        },
    );
    let response = server
        .call_tool(
            "ledger_movement",
            json!({
                "company_guid":"61c6de69-1748-461c-ad3f-162cb949df9f",
                "from":"20260801", "to":"20260802", "ledger":"WR2 Sales"
            }),
        )
        .await;
    // Past the stalled response and any retry that would have queued behind it.
    tokio::time::sleep(busy + std::time::Duration::from_millis(3_000)).await;
    assert_eq!(
        simulator.received(),
        stalled,
        "nothing was sent after the export that timed out"
    );
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(error["code"], "ledger_movement_read_failed");
    assert_eq!(error["cause"], "movement_catalogue_deadline_exceeded");
    assert!(
        error["remediation"]
            .as_str()
            .is_some_and(|text| text.contains("narrowing from and to is not known to help")),
        "narrowing the dates is not known to help a whole-book catalogue: {error}"
    );
    simulator.cancel();
}

/// A ledger the first catalogue does not hold is refused as soon as that
/// catalogue is read. One plan sits behind the catalogue's 26 requests (4
/// identity and 22 of the read itself): a request after the catalogue would be
/// answered and counted, so `received` stays at 26 only if none is sent.
#[tokio::test]
async fn a_movement_names_an_unknown_ledger_before_reading_any_voucher() {
    let mut plans = first_ledger_read();
    let total = plans.len();
    plans.push(plans[1].clone());
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let (server, _directory) = movement_server(
        &simulator,
        bridge_tally_transport::TransportPolicy::default(),
    );
    let response = server
        .call_tool(
            "ledger_movement",
            json!({
                "company_guid":"61c6de69-1748-461c-ad3f-162cb949df9f",
                "from":"20260801", "to":"20260802", "ledger":"No Such Ledger"
            }),
        )
        .await;
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"],
        "ledger_not_found"
    );
    assert_eq!(
        simulator.received(),
        total,
        "no request after the catalogue"
    );
    simulator.cancel();
}

/// `ledger_movement` for a ledger spelt without its accents: the refusal keeps
/// its code and lists the catalogue's own spelling for the user to confirm,
/// from the catalogue already read (no request after it), and says what to do.
async fn movement_for_ledger(
    requested: &str,
    redaction: Redaction,
    max_bytes: usize,
) -> (Value, String) {
    let mut plans = first_ledger_read();
    let total = plans.len();
    plans.push(plans[1].clone());
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let (mut server, _directory) = movement_server(
        &simulator,
        bridge_tally_transport::TransportPolicy::default(),
    );
    server.settings.redaction = redaction;
    server.settings.max_bytes = max_bytes;
    let response = server
        .call_tool(
            "ledger_movement",
            json!({
                "company_guid":"61c6de69-1748-461c-ad3f-162cb949df9f",
                "from":"20260801", "to":"20260802", "ledger": requested
            }),
        )
        .await;
    assert_eq!(
        simulator.received(),
        total,
        "no request after the catalogue"
    );
    simulator.cancel();
    let text = response.to_string();
    (response, text)
}

#[tokio::test]
async fn a_misspelt_ledger_lists_the_catalogues_spelling_and_asks_the_user() {
    let (response, _) = movement_for_ledger("Cafe Naive Traders", Redaction::None, 200_000).await;
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "ledger_not_found", "{error}");
    assert_eq!(error["candidates_listing"], "listed", "{error}");
    assert_eq!(error["candidates_total"], 1, "{error}");
    assert_eq!(error["candidates_total_is_lower_bound"], false, "{error}");
    let names = error["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|candidate| candidate["name"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(names, ["Café Naïve Traders"], "{error}");
    assert!(
        error.get("requested").is_none(),
        "the request is not echoed"
    );
    assert!(error["remediation"]
        .as_str()
        .unwrap()
        .contains("ask which one they meant"));
}

/// Under `mask_parties` neither the catalogue's spelling nor the request is in
/// the response, in the structured result or its text copy.
#[tokio::test]
async fn masking_keeps_the_ledger_names_out_of_a_ledger_refusal() {
    let (response, text) =
        movement_for_ledger("Cafe Naive Traders", Redaction::MaskParties, 200_000).await;
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "ledger_not_found", "{error}");
    assert_eq!(error["candidates_listing"], "names_masked", "{error}");
    // Not even a count: it would answer "does a ledger start with this?".
    for key in ["candidates", "candidates_total", "candidates_reason"] {
        assert!(error.get(key).is_none(), "{key} under masking: {error}");
    }
    for name in ["Café Naïve Traders", "Cafe Naive Traders", "Naïve", "Naive"] {
        assert!(!text.contains(name), "{name} in {text}");
    }
}

/// A masked spelling is refused as such, through the tool, and never looked up.
#[tokio::test]
async fn a_masked_ledger_spelling_is_refused_before_it_can_resolve_to_another_ledger() {
    let (response, _) = movement_for_ledger("Ca…fe", Redaction::MaskParties, 200_000).await;
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "ledger_name_masked", "{error}");
    assert!(error["remediation"]
        .as_str()
        .unwrap()
        .contains("type the full ledger name"));
}

/// Below the budget that carries guidance, the refusal is the bare code: no
/// candidates and no listing word, so an absent listing is "no search was
/// made", which the remediation says, and never a claim about the book.
#[tokio::test]
async fn under_the_guidance_budget_the_ledger_refusal_is_the_bare_code() {
    let (response, _) = movement_for_ledger("Cafe Naive Traders", Redaction::None, 4_095).await;
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "ledger_not_found", "{response}");
    for key in ["candidates", "candidates_listing", "remediation"] {
        assert!(
            error.get(key).is_none(),
            "{key} at the small budget: {error}"
        );
    }
}

/// A catalogue export the transport refuses for its size names that too. A
/// timeout is named already (a request no response answered); an oversized one
/// was answered, so it needs its own naming. The export is the captured one
/// padded past a cap that the identity, extent and currency answers stay under.
#[tokio::test]
async fn a_movement_catalogue_over_the_response_cap_names_why() {
    let mut plans = first_ledger_read();
    let padded = format!(
        "{}{}",
        plans[FIRST_EXPORT].fixture.body(),
        " ".repeat(30_000)
    );
    plans[FIRST_EXPORT] = captured_utf8(&padded);
    plans.truncate(FIRST_EXPORT + 1);
    // One plan behind the export: an extra send would be answered and counted.
    plans.push(plans[1].clone());
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let (server, _directory) = movement_server(
        &simulator,
        bridge_tally_transport::TransportPolicy {
            xml_response_max_bytes: 30_000,
            ..Default::default()
        },
    );
    let response = server
        .call_tool(
            "ledger_movement",
            json!({
                "company_guid":"61c6de69-1748-461c-ad3f-162cb949df9f",
                "from":"20260801", "to":"20260802", "ledger":"WR2 Sales"
            }),
        )
        .await;
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(error["code"], "ledger_movement_read_failed");
    assert_eq!(error["cause"], "movement_catalogue_too_large");
    assert!(
        error["remediation"]
            .as_str()
            .is_some_and(|text| text.contains("do not retry in a loop")),
        "{error}"
    );
    assert_eq!(simulator.received(), FIRST_EXPORT + 1);
    simulator.cancel();
}

/// The corroborating catalogue read (the second, after the voucher window) is
/// sent once as well: it is the same read through the same function, and this
/// pins it rather than inferring it. The script is the stable happy path up to
/// the second export, which stalls, with a whole read behind it for a retry.
#[tokio::test]
async fn a_movements_second_catalogue_read_that_times_out_is_sent_once() {
    let company = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml"
    ));
    let voucher = captured(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    ));
    let high_water = captured_utf8(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY><GUID>61c6de69-1748-461c-ad3f-162cb949df9f</GUID><ALTVCHID>3</ALTVCHID><ALTMSTID>7</ALTMSTID></COMPANY></COLLECTION></DATA></BODY></ENVELOPE>",
    );
    let status = ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime));
    let voucher_read = |body: ScenarioPlan| {
        vec![
            company.clone(),
            body.clone(),
            status.clone(),
            body,
            status.clone(),
            company.clone(),
        ]
    };
    let busy = std::time::Duration::from_millis(5_000);
    let mut plans = first_ledger_read();
    plans.extend(voucher_read(high_water));
    plans.extend(voucher_read(voucher));
    // The second catalogue read is the first one's requests from the read's own
    // brackets on (the identity requests are not repeated); its export is the
    // 12th.
    let export = FIRST_EXPORT - 4;
    let mut second = first_ledger_read().into_iter().skip(4).collect::<Vec<_>>();
    second[export] = second[export]
        .clone()
        .with_delivery(Delivery::SlowHeaders(busy));
    second.truncate(export + 1);
    plans.extend(second);
    let stalled = plans.len();
    plans.extend(first_ledger_read().into_iter().skip(4));
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let (server, _directory) = movement_server(
        &simulator,
        bridge_tally_transport::TransportPolicy {
            request_timeout: std::time::Duration::from_millis(2_000),
            ..Default::default()
        },
    );
    let response = server
        .call_tool(
            "ledger_movement",
            json!({
                "company_guid":"61c6de69-1748-461c-ad3f-162cb949df9f",
                "from":"20260801", "to":"20260802", "ledger":"WR2 Sales"
            }),
        )
        .await;
    tokio::time::sleep(busy + std::time::Duration::from_millis(3_000)).await;
    assert_eq!(
        simulator.received(),
        stalled,
        "nothing was sent after the second export that timed out"
    );
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(response["isError"], true, "{response}");
    assert_eq!(error["code"], "ledger_movement_read_failed");
    assert_eq!(error["cause"], "movement_catalogue_deadline_exceeded");
    simulator.cancel();
}
