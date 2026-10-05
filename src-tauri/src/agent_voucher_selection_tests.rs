//! Captured-source regression cases for selected-ledger reads.
use super::*;

#[tokio::test]
async fn voucher_boundary_refusals_retain_exact_source_commitments() {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    );
    let words = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    let captured = String::from_utf16(&words).unwrap();
    for (from, to, code) in [
        (
            "<AMOUNT TYPE=\"Amount\">-101.01</AMOUNT>",
            "<AMOUNT TYPE=\"Amount\">Maybe</AMOUNT>",
            "voucher_amount_invalid",
        ),
        (
            "\n      <ISDEEMEDPOSITIVE TYPE=\"Logical\">Yes</ISDEEMEDPOSITIVE>",
            "\n      <ISDEEMEDPOSITIVE TYPE=\"Logical\">Maybe</ISDEEMEDPOSITIVE>",
            "voucher_accounting_state_not_observed",
        ),
        (
            "<GUID>61c6de69-1748-461c-ad3f-162cb949df9f-00000001</GUID>",
            "<GUID>71c6de69-1748-461c-ad3f-162cb949df9f-00000001</GUID>",
            "voucher_company_identity_invalid",
        ),
        ("20260801", "2026080A", "voucher_date_invalid"),
        ("20260801", "20260803", "window_not_honoured"),
    ] {
        let damaged = captured.replacen(from, to, 1);
        assert_ne!(damaged, captured);
        let vouchers = ScenarioPlan::new(Fixture::SyntheticXml(damaged))
            .with_encoding(WireEncoding::Utf16Le)
            .with_framing(ResponseFraming::ContentLength);
        let cycle = import_cycle_plans();
        let mut plans = cycle[..4].to_vec();
        // The pre-flight high-water read (protocol reference §11c): ten
        // vouchers cannot exceed the budget, so the window is read whole.
        plans.extend(cycle[10..16].iter().cloned());
        plans.extend([
            cycle[0].clone(),
            vouchers.clone(),
            cycle[1].clone(),
            vouchers.clone(),
            cycle[1].clone(),
            cycle[0].clone(),
        ]);
        let company_body = response_bytes(&plans[0]);
        let high_water_body = response_bytes(&plans[5]);
        let voucher_body = response_bytes(&plans[11]);
        // The window read's own evidence folds its pre-flight first.
        let expected_hash = join_hashes(
            &sha256_hex(&company_body),
            &join_hashes(&sha256_hex(&high_water_body), &sha256_hex(&voucher_body)),
        );
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let response = server_for(simulator.address(), directory.path())
            .call_tool(
                "vouchers",
                json!({"company_guid":CAPTURED_GUID,
                "from":"20260801","to":"20260802"}),
            )
            .await;
        assert_eq!(response["isError"], true);
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"],
            code
        );
        let evidence = &response["structuredContent"]["evidence"];
        assert_eq!(evidence["state"], "partial");
        assert_eq!(evidence["reason_code"], code);
        assert_eq!(evidence["response_sha256"], expected_hash);
        assert_eq!(
            evidence["bytes"],
            2 * (company_body.len() + high_water_body.len() + voucher_body.len())
        );
        let observed = simulator.finish().unwrap();
        assert_eq!(observed.len(), 16);
        assert_eq!(
            evidence["request_sha256"],
            join_hashes(
                &observed[0].request_body_sha256,
                &join_hashes(
                    &observed[5].request_body_sha256,
                    &observed[11].request_body_sha256
                )
            )
        );
    }
}

#[tokio::test]
async fn selected_ledger_rename_or_unknown_entry_refuses_complete_selection() {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    );
    let words = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    let captured = String::from_utf16(&words).unwrap();
    // Negative fault: the returned entries use a renamed ledger. Either the
    // second catalogue observes it or an intervening rename has reverted.
    let renamed = captured.replace("WR2 Sales", "WR2 Renamed Sales");
    assert_ne!(renamed, captured);
    let vouchers = ScenarioPlan::new(Fixture::SyntheticXml(renamed))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength);
    for catalogue_changed in [false, true] {
        let cycle = import_cycle_plans();
        let mut plans = cycle[..10].to_vec();
        plans.extend(cycle[10..16].iter().cloned());
        plans.extend([
            cycle[0].clone(),
            vouchers.clone(),
            cycle[1].clone(),
            vouchers.clone(),
            cycle[1].clone(),
            cycle[0].clone(),
        ]);
        let mut after = cycle[4..10].to_vec();
        if catalogue_changed {
            for index in [1, 3] {
                let original = after[index].fixture.body();
                let altered = original.replace("WR2 Sales", "WR2 Renamed Sales");
                assert_ne!(altered, original);
                after[index].fixture = Fixture::SyntheticXml(altered);
            }
        }
        plans.extend(after);
        // Company, catalogue, high water, window, repeated catalogue. The
        // window read folds its own pre-flight before it joins the chain.
        let source_bytes = [0, 5, 11, 17, 23].map(|index| response_bytes(&plans[index]));
        let hashes = source_bytes.each_ref().map(|body| sha256_hex(body));
        let expected_hash = join_hashes(
            &join_hashes(
                &join_hashes(&hashes[0], &hashes[1]),
                &join_hashes(&hashes[2], &hashes[3]),
            ),
            &hashes[4],
        );
        let expected_bytes = 2 * source_bytes.iter().map(Vec::len).sum::<usize>();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let response = server_for(simulator.address(), directory.path())
            .call_tool(
                "vouchers",
                json!({"company_guid":CAPTURED_GUID,
                "from":"20260801","to":"20260802","ledger":"WR2 Sales"}),
            )
            .await;
        assert_eq!(response["isError"], true);
        assert_eq!(
            response["structuredContent"]["result"]["error"]["code"],
            "ledger_snapshot_drifted"
        );
        assert_eq!(
            response["structuredContent"]["evidence"]["state"],
            "partial"
        );
        let evidence = &response["structuredContent"]["evidence"];
        assert_eq!(evidence["response_sha256"], expected_hash);
        assert_eq!(evidence["bytes"], expected_bytes);
        let observed = simulator.finish().unwrap();
        assert_eq!(observed.len(), 28);
        let requests = [0, 5, 11, 17, 23].map(|index| observed[index].request_body_sha256.clone());
        let request_hash = join_hashes(
            &join_hashes(
                &join_hashes(&requests[0], &requests[1]),
                &join_hashes(&requests[2], &requests[3]),
            ),
            &requests[4],
        );
        assert_eq!(evidence["request_sha256"], request_hash);
    }
}

#[tokio::test]
async fn empty_ledger_selection_does_not_replace_source_emptiness() {
    fn captured_plan(bytes: &[u8]) -> ScenarioPlan {
        let words = bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>();
        ScenarioPlan::new(Fixture::SyntheticXml(String::from_utf16(&words).unwrap()))
            .with_encoding(WireEncoding::Utf16Le)
            .with_framing(ResponseFraming::ContentLength)
    }
    let populated = captured_plan(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    ));
    let empty = captured_plan(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-empty-collection.utf16le.xml"
    ));
    assert_eq!(
        parse_agent_rows(&populated.fixture.body(), CAPTURED_GUID)
            .unwrap()
            .len(),
        3
    );
    assert!(parse_agent_rows(&empty.fixture.body(), CAPTURED_GUID)
        .unwrap()
        .is_empty());
    for source_is_empty in [false, true] {
        let cycle = import_cycle_plans();
        let mut plans = cycle[..10].to_vec();
        // The pre-flight high-water read before the window. The widened read
        // that corroborates an empty window reuses that mark, and ten vouchers
        // keep both windows whole.
        plans.extend(cycle[10..16].iter().cloned());
        let paired_read = |source: &ScenarioPlan| {
            [
                cycle[0].clone(),
                source.clone(),
                cycle[1].clone(),
                source.clone(),
                cycle[1].clone(),
                cycle[0].clone(),
            ]
        };
        plans.extend(paired_read(if source_is_empty {
            &empty
        } else {
            &populated
        }));
        if source_is_empty {
            // The same captured vouchers in the widened read contradict true
            // source emptiness, even though none touch the selected Cash ledger.
            plans.extend(paired_read(&populated));
        } else {
            plans.extend(cycle[4..10].iter().cloned());
        }
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let response = server_for(simulator.address(), directory.path())
            .call_tool(
                "vouchers",
                json!({"company_guid":CAPTURED_GUID,
                "from":"20260801","to":"20260802","ledger":"Cash"}),
            )
            .await;
        if source_is_empty {
            assert_eq!(response["isError"], true);
            assert_eq!(
                response["structuredContent"]["result"]["error"]["code"],
                "window_contradicted"
            );
        } else {
            // #985: the selection inherits its window's label. This book's
            // mark of 10 proves it small without a census, so nothing counted
            // the window and the zero is not a checked one.
            assert_eq!(response["isError"], false);
            assert_eq!(response["structuredContent"]["result"]["state"], "partial");
            assert_eq!(
                response["structuredContent"]["result"]["reason"],
                "nonempty_window_unqualified"
            );
            assert_eq!(response["structuredContent"]["result"]["items"], json!([]));
            assert_eq!(response["structuredContent"]["result"]["total"], 0);
            assert_eq!(
                response["structuredContent"]["evidence"]["state"],
                "partial"
            );
        }
        assert_eq!(simulator.finish().unwrap().len(), 28);
    }
}

/// #595, through the tool call: a `vouchers` result carries what each request
/// of its window read cost, and so does a refusal of that read.
#[tokio::test]
async fn the_vouchers_tool_reports_its_window_timings_on_success_and_refusal() {
    let bytes = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    );
    let words = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    let vouchers = ScenarioPlan::new(Fixture::SyntheticXml(String::from_utf16(&words).unwrap()))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength);
    let call = |plans: Vec<ScenarioPlan>| async move {
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let response = server_for(simulator.address(), directory.path())
            .call_tool(
                "vouchers",
                json!({"company_guid":CAPTURED_GUID, "from":"20260801","to":"20260802"}),
            )
            .await;
        simulator.cancel();
        simulator.finish().unwrap();
        response
    };

    // Ten vouchers at the mark: the window is read whole, in one part.
    let cycle = import_cycle_plans();
    let mut plans = cycle[..4].to_vec();
    plans.extend(cycle[10..16].iter().cloned());
    plans.extend([
        cycle[0].clone(),
        vouchers.clone(),
        cycle[1].clone(),
        vouchers.clone(),
        cycle[1].clone(),
        cycle[0].clone(),
    ]);
    let body = response_bytes(&plans[11]);
    let response = call(plans).await;
    assert_eq!(response["isError"], false, "{response}");
    let window = &response["structuredContent"]["result"]["window"];
    assert_eq!(window["marks"]["requests"], 1);
    assert_eq!(window["census"]["requests"], 0);
    let parts = window["parts"].as_array().unwrap();
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0]["from"], "20260801");
    assert_eq!(parts[0]["to"], "20260802");
    assert_eq!(parts[0]["served"], true);
    assert_eq!(parts[0]["rows"], 3);
    assert_eq!(parts[0]["bytes"], body.len());
    assert!(parts[0]["ms"].is_u64());
    assert_eq!(
        (&window["from"], &window["to"]),
        (&json!("20260801"), &json!("20260802"))
    );
    assert!(window.get("failed").is_none());

    // A mark far over the budget needs a census, and Tally declares the census
    // response over the transport cap: the refusal carries the timings too.
    let heavy = format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY>\
         <GUID>{CAPTURED_GUID}</GUID><ALTVCHID>5000</ALTVCHID><ALTMSTID>7</ALTMSTID>\
         </COMPANY></COLLECTION></DATA></BODY></ENVELOPE>"
    );
    let heavy = ScenarioPlan::new(Fixture::SyntheticXml(heavy))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength);
    let mut plans = cycle[..4].to_vec();
    plans.extend(cycle[10..16].iter().cloned());
    plans[5] = heavy.clone();
    plans[7] = heavy;
    plans.extend([
        cycle[0].clone(),
        vouchers.with_framing(ResponseFraming::DeclaredContentLength {
            bytes: bridge_tally_transport::XML_RESPONSE_MAX_BYTES + 1,
        }),
    ]);
    let response = call(plans).await;
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], crate::agent::VOLUME_UNESTIMATED);
    let window = &error["window"];
    assert_eq!(window["marks"]["requests"], 1);
    assert_eq!(window["census"]["requests"], 1);
    assert_eq!(window["parts"], json!([]));
    assert_eq!(window["failed"]["kind"], "census");
    assert!(window["failed"]["ms"].is_u64());
}

/// #595: the window timings are the `vouchers` tool's alone. Another tool
/// whose window read is refused the same way keeps its refusal shape.
#[tokio::test]
async fn only_the_vouchers_tool_reports_window_timings_on_a_refusal() {
    let heavy = format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><COMPANY>\
         <GUID>{CAPTURED_GUID}</GUID><ALTVCHID>5000</ALTVCHID><ALTMSTID>7</ALTMSTID>\
         </COMPANY></COLLECTION></DATA></BODY></ENVELOPE>"
    );
    let heavy = ScenarioPlan::new(Fixture::SyntheticXml(heavy))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength);
    let cycle = import_cycle_plans();
    // The company, the ledger catalogue, the heavy mark, then a census Tally
    // declares over the transport cap.
    let mut plans = cycle[..10].to_vec();
    plans.extend(cycle[10..16].iter().cloned());
    plans[11] = heavy.clone();
    plans[13] = heavy;
    plans.extend([
        cycle[0].clone(),
        cycle[11]
            .clone()
            .with_framing(ResponseFraming::DeclaredContentLength {
                bytes: bridge_tally_transport::XML_RESPONSE_MAX_BYTES + 1,
            }),
    ]);
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let response = server_for(simulator.address(), directory.path())
        .call_tool(
            "voucher_presence",
            json!({
                "company_guid": CAPTURED_GUID,
                "from": "20260801",
                "to": "20260802",
                "numbering": [{"voucher_type": "Journal", "numbering_method": "manual"}],
                "vouchers": [{
                    "date": "20260801", "voucher_type": "Journal", "voucher_number": "JV-1",
                    "party": "Cash",
                    "entries": [{"ledger": "Cash", "amount": "-12.50"},
                                {"ledger": "WR2 Sales", "amount": "12.50"}],
                }],
            }),
        )
        .await;
    simulator.cancel();
    simulator.finish().unwrap();
    let error = &response["structuredContent"]["result"]["error"];
    // The same refusal the `vouchers` test reaches, so the window read ran.
    assert_eq!(
        error["code"],
        crate::agent::VOLUME_UNESTIMATED,
        "{response}"
    );
    assert!(error.get("window").is_none(), "{error}");
}

/// Plans for a `vouchers` call whose requested window is empty, so that the
/// wider corroboration read runs next with `corroboration` as its paired legs.
fn empty_window_then(corroboration: Vec<ScenarioPlan>) -> Vec<ScenarioPlan> {
    let decode = |bytes: &[u8]| {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    };
    let empty = ScenarioPlan::new(Fixture::SyntheticXml(decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-empty-collection.utf16le.xml"
    ))))
    .with_encoding(WireEncoding::Utf16Le)
    .with_framing(ResponseFraming::ContentLength);
    let cycle = import_cycle_plans();
    let mut plans = cycle[..4].to_vec();
    plans.extend(cycle[10..16].iter().cloned());
    plans.extend([
        cycle[0].clone(),
        empty.clone(),
        cycle[1].clone(),
        empty,
        cycle[1].clone(),
        cycle[0].clone(),
    ]);
    plans.extend(corroboration);
    plans
}

async fn call_vouchers(plans: Vec<ScenarioPlan>) -> Value {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let response = server_for(simulator.address(), directory.path())
        .call_tool(
            "vouchers",
            json!({"company_guid":CAPTURED_GUID, "from":"20260801","to":"20260802"}),
        )
        .await;
    simulator.cancel();
    simulator.finish().unwrap();
    response
}

/// #595: an empty requested window is corroborated by a wider read of its
/// own. When Tally refuses that read, the error's window is the wider one, and
/// says so by its own dates, with the part that failed.
#[tokio::test]
async fn a_refused_corroboration_read_reports_its_own_window() {
    let cycle = import_cycle_plans();
    let refused = cycle[11].clone().with_http_status(500);
    let response = call_vouchers(empty_window_then(vec![cycle[0].clone(), refused])).await;
    assert_eq!(response["isError"], true, "{response}");
    // Tally's refusal of the wider read, reported as the generic read failure.
    assert_eq!(
        response["structuredContent"]["result"]["error"]["code"],
        "agent_runtime_read_failed"
    );
    let window = &response["structuredContent"]["result"]["error"]["window"];
    assert_eq!(window["from"], "20260731", "{window}");
    assert_eq!(window["to"], "20260803", "{window}");
    assert_eq!(window["failed"]["kind"], "part", "{window}");
    assert_eq!(window["parts"][0]["served"], false, "{window}");
}

/// #595: a corroboration read Tally served, but whose rows fail validation, is
/// not a window-read failure: the error carries no window.
#[tokio::test]
async fn a_corroboration_refused_after_its_read_carries_no_window() {
    let outside = ScenarioPlan::new(Fixture::SyntheticXml(
        String::from_utf16(
            &include_bytes!(
                "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
            )
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
        )
        .unwrap()
        // Dated outside the wider window too.
        .replace("20260801", "20261001"),
    ))
    .with_encoding(WireEncoding::Utf16Le)
    .with_framing(ResponseFraming::ContentLength);
    let cycle = import_cycle_plans();
    let response = call_vouchers(empty_window_then(vec![
        cycle[0].clone(),
        outside.clone(),
        cycle[1].clone(),
        outside,
        cycle[1].clone(),
        cycle[0].clone(),
    ]))
    .await;
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "window_not_honoured", "{response}");
    assert!(error.get("window").is_none(), "{error}");
}

/// A `vouchers` call filtered to one ledger, with every catalogue read served
/// `catalogue`. The sequence is the rename case's: company, paired catalogue,
/// the window's marks, the window, then the repeated catalogue.
async fn call_filtered_vouchers(catalogue: impl Fn(&str) -> String, ledger: &str) -> Value {
    let words = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    )
    .chunks_exact(2)
    .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
    .collect::<Vec<_>>();
    call_filtered_vouchers_over(String::from_utf16(&words).unwrap(), catalogue, ledger).await
}

/// [`call_filtered_vouchers`] with `window` as the vouchers response.
async fn call_filtered_vouchers_over(
    window: String,
    catalogue: impl Fn(&str) -> String,
    ledger: &str,
) -> Value {
    call_filtered_vouchers_configured(window, catalogue, ledger, 200_000, Redaction::None).await
}

/// [`call_filtered_vouchers_over`] under a response cap and a redaction setting.
async fn call_filtered_vouchers_configured(
    window: String,
    catalogue: impl Fn(&str) -> String,
    ledger: &str,
    max_bytes: usize,
    redaction: Redaction,
) -> Value {
    call_filtered_vouchers_args(
        window,
        catalogue,
        json!({"company_guid":CAPTURED_GUID,
        "from":"20260801","to":"20260802","ledger":ledger}),
        max_bytes,
        redaction,
    )
    .await
}

/// [`call_filtered_vouchers_configured`] with the whole argument object given, for a ledger
/// call that also searches or summarises.
async fn call_filtered_vouchers_args(
    window: String,
    catalogue: impl Fn(&str) -> String,
    args: Value,
    max_bytes: usize,
    redaction: Redaction,
) -> Value {
    let vouchers = ScenarioPlan::new(Fixture::SyntheticXml(window))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength);
    let cycle = import_cycle_plans();
    let mut plans = cycle[..10].to_vec();
    plans.extend(cycle[10..16].iter().cloned());
    plans.extend([
        cycle[0].clone(),
        vouchers.clone(),
        cycle[1].clone(),
        vouchers,
        cycle[1].clone(),
        cycle[0].clone(),
    ]);
    plans.extend(cycle[4..10].iter().cloned());
    for index in [5, 7, 23, 25] {
        let body = catalogue(&plans[index].fixture.body());
        plans[index].fixture = Fixture::SyntheticXml(body);
    }
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut server = server_for(simulator.address(), directory.path());
    server.settings.max_bytes = max_bytes;
    server.settings.redaction = redaction;
    let response = server.call_tool("vouchers", args).await;
    simulator.cancel();
    simulator.finish().unwrap();
    response
}

/// `vouchers` for a ledger named by its first words refuses with the code it
/// always had, and lists the ledger it may mean for the user to confirm.
#[tokio::test]
async fn a_ledger_filter_for_a_truncated_name_lists_what_it_may_mean() {
    let response = call_filtered_vouchers(|catalogue| catalogue.to_string(), "wr2 sale").await;
    assert_eq!(response["isError"], true, "{response}");
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "ledger_not_found", "{error}");
    assert_eq!(error["candidates_listing"], "listed", "{error}");
    assert_eq!(error["candidates"][0]["name"], "WR2 Sales", "{error}");
    assert_eq!(error["candidates"][0]["rule"], "catalog_prefix", "{error}");
}

/// A refusal that lists ledgers is framed twice (structured and text copy) and an
/// error object has no page to trim, so a list that did not fit would cost the
/// caller the refusal code. Across caps from the smallest that carries guidance
/// to well above the list floor, the code always survives; the list is
/// attached only from its floor, cut to a sixteenth of the cap, and a cut list
/// says `truncated` with the whole count.
#[tokio::test]
async fn a_long_candidate_list_never_costs_the_refusal_its_code() {
    let window = || {
        let words = include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
        )
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
        String::from_utf16(&words).unwrap()
    };
    let long_names = |catalogue: &str| {
        crate::tally::standard_ledger_catalog::tests::catalogue_with_extra_ledgers(
            catalogue,
            (0..20).map(|index| {
                (
                    format!("Sundry Creditor With A Deliberately Long Name \u{0928}\u{092e}\u{0938}\u{094d}\u{0924}\u{0947} {index:02}"),
                    format!("c{index:07x}"),
                )
            }),
        )
    };
    for max_bytes in [
        4_096, 5_000, 6_000, 8_000, 12_000, 16_383, 16_384, 20_000, 200_000,
    ] {
        let response = call_filtered_vouchers_configured(
            window(),
            long_names,
            "Sundry Creditor With A Deliberately Long Name",
            max_bytes,
            Redaction::None,
        )
        .await;
        let error = &response["structuredContent"]["result"]["error"];
        assert_eq!(error["code"], "ledger_not_found", "{max_bytes}: {response}");
        if max_bytes < 16_384 {
            assert!(
                error.get("candidates_listing").is_none(),
                "{max_bytes}: {error}"
            );
        } else {
            let listing = error["candidates_listing"].as_str().unwrap();
            let listed = error["candidates"].as_array().unwrap().len();
            assert_eq!(error["candidates_total"], 20, "{max_bytes}: {error}");
            assert_eq!(error["candidates_reason"], "master_binding_near_miss");
            // Truncated exactly when the budget cut the list, and the old
            // field agrees with the word.
            assert_eq!(listing == "truncated", listed < 20, "{max_bytes}: {error}");
            assert_eq!(error["candidates_truncated"], listing == "truncated");
            if max_bytes == 16_384 {
                assert_eq!(listing, "truncated", "{error}");
            }
            if max_bytes == 200_000 {
                assert_eq!((listing, listed), ("listed", 20), "{error}");
            }
        }
    }
}

/// Twenty-six ledgers that differ only in their spaces all collide (the
/// catalogue itself refuses a book whose names differ only in case): the
/// refusal is `ledger_ambiguous`, lists the first twenty-five, and says
/// `truncated` with the whole count, in the old field as well as the new word.
#[tokio::test]
async fn more_colliding_ledgers_than_the_cap_are_truncated_with_the_whole_count() {
    let window = {
        let words = include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
        )
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
        String::from_utf16(&words).unwrap()
    };
    let response = call_filtered_vouchers_configured(
        window,
        |catalogue| {
            crate::tally::standard_ledger_catalog::tests::catalogue_with_extra_ledgers(
                catalogue,
                (1..=26_usize).map(|spaces| {
                    (
                        format!("Dup{}xy", " ".repeat(spaces)),
                        format!("d{spaces:07x}"),
                    )
                }),
            )
        },
        "dup xy",
        200_000,
        Redaction::None,
    )
    .await;
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "ledger_ambiguous", "{error}");
    assert_eq!(error["candidates_listing"], "truncated", "{error}");
    assert_eq!(error["candidates_truncated"], true, "{error}");
    assert_eq!(error["candidates_total"], 26, "{error}");
    assert_eq!(error["candidates"].as_array().unwrap().len(), 25, "{error}");
}

/// A requested name the binding rules refuse (a control character) cannot be
/// searched: the refusal keeps its code and says `unavailable`, with the reason
/// and without a count (a 0 would read as "no ledgers").
#[tokio::test]
async fn an_unsearchable_name_says_unavailable_and_gives_no_count() {
    let window = {
        let words = include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
        )
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
        String::from_utf16(&words).unwrap()
    };
    let response = call_filtered_vouchers_configured(
        window,
        |catalogue| catalogue.to_string(),
        "wr2\u{7}sale",
        200_000,
        Redaction::None,
    )
    .await;
    let error = &response["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "ledger_not_found", "{response}");
    assert_eq!(error["candidates_listing"], "unavailable", "{error}");
    assert_eq!(error["candidates_reason"], "master_name_unsafe", "{error}");
    for key in [
        "candidates",
        "candidates_total",
        "candidates_total_is_lower_bound",
    ] {
        assert!(
            error.get(key).is_none(),
            "{key} beside unavailable: {error}"
        );
    }
}

/// A state that carries no list (nothing resembles the name) needs only the
/// guidance floor, not the list floor: it is there from 4,096 bytes, says
/// `none`, and does not say the ledger is absent.
#[tokio::test]
async fn a_name_that_resembles_nothing_says_none_from_the_guidance_floor() {
    let window = {
        let words = include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
        )
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
        String::from_utf16(&words).unwrap()
    };
    for (max_bytes, carries) in [(4_095, false), (4_096, true)] {
        let response = call_filtered_vouchers_configured(
            window.clone(),
            |catalogue| catalogue.to_string(),
            "qzxv",
            max_bytes,
            Redaction::None,
        )
        .await;
        let error = &response["structuredContent"]["result"]["error"];
        assert_eq!(error["code"], "ledger_not_found", "{max_bytes}: {error}");
        assert_eq!(
            error.get("candidates_listing").is_some(),
            carries,
            "{max_bytes}: {error}"
        );
        if carries {
            assert_eq!(error["candidates_listing"], "none", "{error}");
            assert_eq!(error["candidates_total"], 0, "{error}");
            assert_eq!(error["candidates_truncated"], false, "{error}");
            assert!(error.get("candidates").is_none() || error["candidates"] == json!([]));
        }
    }
}

/// Through `vouchers`, under `mask_parties`: no name, no count, and the masked
/// spelling is refused, not resolved to another ledger.
#[tokio::test]
async fn masking_hides_the_candidates_and_refuses_a_masked_spelling_through_vouchers() {
    let window = || {
        let words = include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
        )
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
        String::from_utf16(&words).unwrap()
    };
    let masked = call_filtered_vouchers_configured(
        window(),
        |catalogue| catalogue.to_string(),
        "wr2 sale",
        200_000,
        Redaction::MaskParties,
    )
    .await;
    let error = &masked["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "ledger_not_found", "{error}");
    assert_eq!(error["candidates_listing"], "names_masked", "{error}");
    for key in ["candidates", "candidates_total", "candidates_reason"] {
        assert!(error.get(key).is_none(), "{key}: {error}");
    }
    assert!(!masked.to_string().contains("WR2 Sales"), "{masked}");
    let spelled = call_filtered_vouchers_configured(
        window(),
        |catalogue| catalogue.to_string(),
        "WR…es",
        200_000,
        Redaction::MaskParties,
    )
    .await;
    assert_eq!(
        spelled["structuredContent"]["result"]["error"]["code"], "ledger_name_masked",
        "{spelled}"
    );
}

/// bridge#634: the catalogue parser refused any book past 1,000 ledgers, as
/// `ledger_export_invalid` whatever the names, so a book of about 9,500 ledgers lost the
/// ledger filter. Of the capture's three vouchers only one posts to this
/// ledger, so the filter itself is observed, not only the refusal's absence.
#[tokio::test]
async fn a_ledger_filter_reads_a_book_of_more_than_a_thousand_ledgers() {
    let response = call_filtered_vouchers(
        |catalogue| {
            crate::tally::standard_ledger_catalog::tests::catalogue_with_extra_ledgers(
                catalogue,
                (0..1_000)
                    .map(|index| (format!("Bulk Ledger {index:04}"), format!("b{index:07x}"))),
            )
        },
        "Café Naïve Traders",
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    // A mark-10 book: read whole, uncounted (#985).
    assert_eq!(result["state"], "partial", "{result}");
    assert_eq!(result["total"], 1, "{result}");
    let ledgers = result["items"][0]["amounts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["ledger"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(ledgers.contains(&"Café Naïve Traders"), "{ledgers:?}");
}

/// bridge#634: a catalogue refused for one ledger says why in the refusal's
/// cause, and never which ledger.
#[tokio::test]
async fn a_refused_ledger_catalogue_names_its_cause_and_no_ledger() {
    for (rows, cause) in [
        (
            vec![
                ("Twice Named".to_string(), "c0000001".to_string()),
                ("Twice Named".to_string(), "c0000002".to_string()),
            ],
            "ledger_catalogue_duplicate_identity",
        ),
        (
            vec![("Twice\u{202E}Turned".to_string(), "c0000003".to_string())],
            "ledger_catalogue_name_unusable",
        ),
    ] {
        let response = call_filtered_vouchers(
            |catalogue| {
                crate::tally::standard_ledger_catalog::tests::catalogue_with_extra_ledgers(
                    catalogue,
                    rows.clone(),
                )
            },
            "WR2 Sales",
        )
        .await;
        assert_eq!(response["isError"], true, "{response}");
        let content = &response["structuredContent"];
        let error = &content["result"]["error"];
        // The code still names what failed, for any caller keyed on it.
        assert_eq!(error["code"], "ledger_export_invalid", "{response}");
        assert_eq!(error["cause"], cause, "{response}");
        assert_eq!(content["evidence"]["reason_code"], "ledger_export_invalid");
        assert!(!response.to_string().contains("Twice"), "{response}");
    }
}

// -- #674: a foreign-currency composite withholds its voucher, not the window --

use crate::agent::voucher_parse::window_with_composite_vouchers;

fn decoded(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

/// Voucher 1 of the captured three-voucher window made a composite one.
fn window_with_a_composite_voucher() -> String {
    window_with_composite_vouchers(1)
}

/// The whole-window `vouchers` plans of the timings test, serving `window`.
fn vouchers_plans(window: String) -> Vec<ScenarioPlan> {
    let vouchers = ScenarioPlan::new(Fixture::SyntheticXml(window))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength);
    let cycle = import_cycle_plans();
    let mut plans = cycle[..4].to_vec();
    plans.extend(cycle[10..16].iter().cloned());
    plans.extend([
        cycle[0].clone(),
        vouchers.clone(),
        cycle[1].clone(),
        vouchers,
        cycle[1].clone(),
        cycle[0].clone(),
    ]);
    plans
}

async fn call_vouchers_with(plans: Vec<ScenarioPlan>, extra: Value) -> Value {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let mut args = json!({"company_guid":CAPTURED_GUID, "from":"20260801","to":"20260802"});
    for (key, value) in extra.as_object().unwrap() {
        args[key] = value.clone();
    }
    let response = server_for(simulator.address(), directory.path())
        .call_tool("vouchers", args)
        .await;
    simulator.cancel();
    simulator.finish().unwrap();
    response
}

#[tokio::test]
async fn a_composite_voucher_is_withheld_and_the_rest_of_the_window_is_returned() {
    let response =
        call_vouchers_with(vouchers_plans(window_with_a_composite_voucher()), json!({})).await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    // `items` does not cover the window, and the state says so first.
    assert_eq!(result["state"], "partial", "{result}");
    assert_eq!(result["reason"], "vouchers_withheld");
    assert_eq!(
        response["structuredContent"]["evidence"]["state"],
        "partial"
    );
    assert_eq!(result["total"], 2);
    let numbers: Vec<&str> = result["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["voucher_number"].as_str().unwrap())
        .collect();
    assert_eq!(numbers, vec!["2", "3"]);
    assert_eq!(result["withheld_total"], 1);
    assert_eq!(
        result["withheld_vouchers"],
        json!([{
            "guid": "61c6de69-1748-461c-ad3f-162cb949df9f-00000001",
            "date": "20260801", "voucher_type": "Sales", "voucher_number": "1",
            "cause": "foreign_currency_amount_unparsed",
        }])
    );
    assert!(result["coverage"]
        .as_str()
        .unwrap()
        .contains("exclude 1 voucher"));
    // No composite reaches the payload.
    assert!(!result.to_string().contains(" @ "), "{result}");
}

#[tokio::test]
async fn a_withheld_listing_is_the_same_on_every_page() {
    let mut pages = Vec::new();
    for offset in [0, 1] {
        let response = call_vouchers_with(
            vouchers_plans(window_with_a_composite_voucher()),
            json!({"limit": 1, "offset": offset}),
        )
        .await;
        assert_eq!(response["isError"], false, "{response}");
        pages.push(response["structuredContent"]["result"].clone());
    }
    for key in [
        "withheld_total",
        "withheld_vouchers",
        "coverage",
        "total",
        "state",
    ] {
        assert_eq!(pages[0][key], pages[1][key], "{key}");
    }
    assert_ne!(pages[0]["items"], pages[1]["items"]);
}

#[tokio::test]
async fn an_ordinary_window_carries_no_withheld_fields() {
    let response = call_vouchers_with(
        vouchers_plans(decoded(include_bytes!(
            "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
        ))),
        json!({}),
    )
    .await;
    let result = &response["structuredContent"]["result"];
    // A mark-10 book: read whole, uncounted (#985).
    assert_eq!(result["state"], "partial", "{result}");
    assert_eq!(result["reason"], "nonempty_window_unqualified");
    for key in ["withheld_total", "withheld_vouchers", "coverage"] {
        assert!(result.get(key).is_none(), "{key}");
    }
}

#[tokio::test]
async fn a_withheld_voucher_is_listed_under_a_ledger_filter_it_touches() {
    // Voucher 1 posts to this party ledger; its amounts are the composites.
    let touching = call_filtered_vouchers_over(
        window_with_a_composite_voucher(),
        |catalogue| catalogue.to_string(),
        "नमस्ते ट्रेडर्स",
    )
    .await;
    assert_eq!(touching["isError"], false, "{touching}");
    let result = &touching["structuredContent"]["result"];
    assert_eq!(result["total"], 0, "{result}");
    assert_eq!(result["withheld_total"], 1);
    assert_eq!(result["state"], "partial");
    // A ledger it does not touch lists nothing withheld.
    let other = call_filtered_vouchers_over(
        window_with_a_composite_voucher(),
        |catalogue| catalogue.to_string(),
        "Café Naïve Traders",
    )
    .await;
    let result = &other["structuredContent"]["result"];
    assert_eq!(result["total"], 1, "{result}");
    assert!(result.get("withheld_total").is_none(), "{result}");
    // Uncounted, so `partial` for the window, not for a withheld voucher.
    assert_eq!(result["state"], "partial");
    assert_eq!(result["reason"], "nonempty_window_unqualified");
}

/// A window whose every voucher is withheld is not empty: the empty-window
/// corroboration does not run, so these plans hold no corroboration read and
/// `finish` would fail on one.
#[tokio::test]
async fn an_all_withheld_window_is_not_empty_and_is_not_corroborated() {
    let response =
        call_vouchers_with(vouchers_plans(window_with_composite_vouchers(3)), json!({})).await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["total"], 0, "{result}");
    assert_eq!(result["items"], json!([]));
    assert_eq!(result["withheld_total"], 3);
    assert_eq!(result["state"], "partial");
    assert_eq!(result["reason"], "vouchers_withheld");
}

/// #1076: a ledger named in another ASCII case is read, and the answer says
/// which ledger and how the name reached it; the exact spelling says `exact`.
/// Mutant killed: dropping `ledger_match` from the vouchers result.
#[tokio::test]
async fn a_ledger_filter_says_which_ledger_it_read_and_how() {
    for (requested, matched) in [
        ("Café Naïve Traders", "exact"),
        ("CAFé NAïVE TRADERS", "case_or_spacing"),
    ] {
        let response = call_filtered_vouchers(|catalogue| catalogue.to_string(), requested).await;
        assert_eq!(response["isError"], false, "{response}");
        let result = &response["structuredContent"]["result"];
        assert_eq!(result["total"], 1, "{result}");
        let ledger_match = &result["ledger_match"];
        assert_eq!(ledger_match["matched"], matched, "{ledger_match}");
        assert_eq!(
            ledger_match["ledger"], "Café Naïve Traders",
            "{ledger_match}"
        );
        assert_eq!(ledger_match["similar_ledgers"], json!([]), "{ledger_match}");
    }
}

// -- #1230: a summary over the selectors the tool already had --

/// A summary of a window narrowed to one ledger adds only that ledger's entries by month, and
/// says so; the ledger is resolved through the live catalogue as for any `ledger` call.
/// Mutant killed: not carrying the resolved ledger from the operation into the summary.
#[tokio::test]
async fn a_summary_of_a_ledger_window_adds_only_that_ledgers_entries() {
    let words = include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-three-vouchers.utf16le.xml"
    )
    .chunks_exact(2)
    .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
    .collect::<Vec<_>>();
    let response = call_filtered_vouchers_args(
        String::from_utf16(&words).unwrap(),
        |catalogue| catalogue.to_string(),
        json!({"company_guid": CAPTURED_GUID, "from": "20260801", "to": "20260802",
            "ledger": "wr2 sales", "summarise_by": "month"}),
        200_000,
        Redaction::None,
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["entries_counted"], "selected_ledger", "{result}");
    assert_eq!(result["buckets"][0]["group"], "2026-08");
    assert_eq!(result["buckets"][0]["debit"], "0");
    assert_eq!(result["buckets"][0]["credit"], "306.06");
    assert_eq!(result["ledger_match"]["matched"], "case_or_spacing");
}

/// A voucher withheld for a foreign-currency amount is in no bucket, and the result says its
/// totals are short by it. Mutant killed: summing a withheld row, or the listing's coverage text.
#[tokio::test]
async fn a_summary_leaves_a_withheld_voucher_out_and_says_so() {
    let response = call_vouchers_with(
        vouchers_plans(window_with_a_composite_voucher()),
        json!({"summarise_by": "voucher_type"}),
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "partial", "{result}");
    assert_eq!(result["reason"], "vouchers_withheld");
    assert_eq!(result["vouchers_summarised"], 2);
    assert_eq!(result["buckets"][0]["vouchers"], 2);
    assert_eq!(result["buckets"][0]["credit"], "205.05");
    assert_eq!(result["withheld_total"], 1);
    assert!(result["coverage"]
        .as_str()
        .unwrap()
        .starts_with("buckets exclude 1 voucher"));
    assert!(!result.to_string().contains(" @ "), "{result}");
}

/// An amount search keeps a withheld voucher it cannot judge and the result stays partial.
#[tokio::test]
async fn an_amount_search_over_a_window_with_a_withheld_voucher_stays_partial() {
    let response = call_vouchers_with(
        vouchers_plans(window_with_a_composite_voucher()),
        json!({"amount": "103.03"}),
    )
    .await;
    assert_eq!(response["isError"], false, "{response}");
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["state"], "partial", "{result}");
    assert_eq!(result["total"], 1);
    assert_eq!(result["items"][0]["voucher_number"], "3");
    assert_eq!(
        result["items"][0]["matched"],
        json!({"amount_entries": [0, 1]})
    );
    assert_eq!(result["withheld_total"], 1);
    assert!(result["coverage"]
        .as_str()
        .unwrap()
        .contains("an amount search keeps every withheld voucher"));
}
