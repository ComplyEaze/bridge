use super::*;

#[test]
fn large_pages_use_logarithmically_bounded_serialization_probes() {
    let mut response = json!({"result":{"offset":0,"items":
        (0..10_000).map(|id| json!({"id":id,"padding":"x".repeat(64)})).collect::<Vec<_>>()}});
    let mut probes = 0;
    assert!(fit_response(&mut response, "", 512, |value| {
        probes += 1;
        value.to_string().len()
    })
    .unwrap());
    assert!(probes <= 15, "serialization probes: {probes}");
    assert!(response.to_string().len() <= 512);
    let kept = response["result"]["items"].as_array().unwrap();
    assert!(!kept.is_empty());
    assert_eq!(response["result"]["next_offset"], kept.len());
    assert_eq!(kept[0]["id"], 0);
    assert_eq!(kept.last().unwrap()["id"], kept.len() - 1);
}

#[test]
fn a_paged_masters_result_is_trimmed_with_a_cursor_and_an_unpaged_one_is_not() {
    // The `masters` tool pages by `offset`, so an over-budget page is cut and
    // resumes where it stopped.
    let mut paged = json!({"result":{"offset":0,"next_offset":null,
        "masters":[{"name":"A"},{"name":"B"},{"name":"C"}]}});
    assert!(truncate_response_items(&mut paged).expect("trims a masters page"));
    assert_eq!(paged["result"]["masters"].as_array().unwrap().len(), 2);
    assert_eq!(paged["result"]["next_offset"], 2);
    // validate_masters and build_import_xml also key `masters`, with no
    // `offset`: a cursor into them would resume nothing, so they are not paged.
    let mut unpaged = json!({"result":{"masters":[{"name":"A"},{"name":"B"}]}});
    assert!(!truncate_response_items(&mut unpaged).expect("no page shape"));
    assert_eq!(unpaged["result"]["masters"].as_array().unwrap().len(), 2);
    assert!(unpaged["result"].get("next_offset").is_none());
}

#[test]
fn standalone_master_and_status_rows_are_counted_in_final_receipts() {
    for (tool, axis) in [
        ("validate_masters", "masters"),
        ("tally_status", "loaded_companies"),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(Settings {
            endpoint: TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port: 9,
            },
            data_dir: directory.path().to_path_buf(),
            max_rows: 10,
            max_bytes: 200_000,
            redaction: Redaction::None,
            import_enabled: false,
            writes_enabled: false,
            batch_post_enabled: false,
        });
        let mut structured = json!({"result": {}});
        structured["result"][axis] = json!([{"name":"A"}, {"name":"B"}]);
        let mut response = json!({"jsonrpc":"2.0", "id":1, "result":{
            "content":[{"type":"text","text":""}], "structuredContent":structured,
            "isError":false}});
        set_mcp_content_json(&mut response["result"]);
        let wire = format!("{response}\n");
        server
            .append_framed_egress(
                EgressContext {
                    evidence: None,
                    tool: tool.into(),
                    args_sha256: sha256_json(&json!({})),
                    company_guid: None,
                    request_trail: None,
                },
                &response,
                &wire,
            )
            .unwrap();
        let receipt: Value = serde_json::from_str(
            &fs::read_to_string(directory.path().join("agent-egress.jsonl")).unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["rows_prepared"], 2, "{tool}");
        assert_eq!(receipt["response_sha256"], sha256_hex(wire.as_bytes()));
    }
}

fn outstandings_page(offset: usize) -> Value {
    let rows = |count| {
        (0..count)
            .map(|id| json!({"id":id,"padding":"x".repeat(120)}))
            .collect()
    };
    let (bills, _, bill_next) = paginate_open_bills(rows(3), offset, 3);
    let (parties, _, party_next) = paginate_open_bills(rows(5), offset, 3);
    json!({"truncated":bill_next.is_some() || party_next.is_some(), "result":{
        "offset":offset,"open_bills":bills,"next_offset":bill_next,
        "unallocated":{"count":5,"parties":parties,"next_offset":party_next,
        "truncated":party_next.is_some()}}})
}

#[test]
fn final_framing_caps_advance_both_outstandings_axes_without_losing_rows() {
    let mut offset = 0;
    let mut bills = Vec::new();
    let mut parties = Vec::new();
    loop {
        let mut response = json!({"jsonrpc":"2.0","id":1,"result":{
            "content":[{"type":"text","text":""}], "isError":false,
            "structuredContent":outstandings_page(offset)}});
        set_mcp_content_json(&mut response["result"]);
        enforce_jsonrpc_response_byte_cap(&mut response, 1200).unwrap();
        assert!(response.to_string().len() < 1200);
        let payload = &response["result"]["structuredContent"];
        assert_eq!(
            serde_json::from_str::<Value>(
                response["result"]["content"][0]["text"].as_str().unwrap()
            )
            .unwrap(),
            *payload
        );
        bills.extend(
            payload["result"]["open_bills"]
                .as_array()
                .unwrap()
                .iter()
                .map(|row| row["id"].as_u64().unwrap()),
        );
        parties.extend(
            payload["result"]["unallocated"]["parties"]
                .as_array()
                .unwrap()
                .iter()
                .map(|row| row["id"].as_u64().unwrap()),
        );
        assert_eq!(payload["result"]["unallocated"]["count"], 5);
        let cursors = [
            payload["result"]["next_offset"].as_u64(),
            payload["result"]["unallocated"]["next_offset"].as_u64(),
        ];
        if offset >= 3 {
            assert!(cursors[0].is_none());
        }
        let continuing = cursors.into_iter().flatten().collect::<Vec<_>>();
        let Some(next) = continuing.first() else {
            break;
        };
        assert!(continuing.iter().all(|cursor| cursor == next));
        assert!(*next > offset as u64);
        offset = *next as usize;
        assert!(offset <= 5);
    }
    assert_eq!(bills, [0, 1, 2]);
    assert_eq!(parties, [0, 1, 2, 3, 4]);
}

#[test]
fn every_cap_layer_refuses_a_page_whose_first_rows_cannot_fit() {
    let mut structured = outstandings_page(0);
    structured["result"]["open_bills"][0]["padding"] = json!("x".repeat(2000));
    assert_eq!(
        enforce_response_byte_cap(structured.clone(), 500).unwrap_err(),
        "agent_response_too_large"
    );
    let mcp = json!({"content":[{"type":"text","text":""}], "isError":false,"structuredContent":structured});
    assert_eq!(
        enforce_mcp_result_byte_cap(&mut mcp.clone(), 500, "outstandings", 6).unwrap_err(),
        "agent_response_too_large"
    );
    let mut framed = json!({"jsonrpc":"2.0","id":1,"result":mcp});
    set_mcp_content_json(&mut framed["result"]);
    assert_eq!(
        enforce_jsonrpc_response_byte_cap(&mut framed, 500).unwrap_err(),
        "agent_response_too_large"
    );
    for axis in ["items", "ledgers"] {
        let mut page = json!({"result":{"offset":7}});
        page["result"][axis] = json!([{"padding":"x".repeat(2000)}]);
        assert_eq!(
            enforce_response_byte_cap(page, 500).unwrap_err(),
            "agent_response_too_large"
        );
    }
}

#[test]
fn nonpageable_results_refuse_byte_trimming_without_inventing_continuations() {
    for (tool, axis) in [
        ("list_companies", "companies"),
        ("tally_status", "loaded_companies"),
        ("validate_masters", "masters"),
        ("read_evidence", "records"),
        ("egress_log", "records"),
    ] {
        // History tools may already report a bounded tail. That does not make
        // the tail offset-pageable, and its existing truncation must survive.
        let mut structured = json!({"truncated":axis == "records", "result":{}});
        structured["result"][axis] = json!((0..5)
            .map(|id| json!({"id":id,"padding":"x".repeat(160)}))
            .collect::<Vec<_>>());
        let (complete, trimmed, rows) =
            enforce_response_byte_cap(structured.clone(), 10_000).unwrap();
        assert_eq!(complete, structured);
        assert!(!trimmed);
        assert_eq!(rows, 5);
        assert!(complete["result"].get("next_offset").is_none());
        assert_eq!(
            enforce_response_byte_cap(structured.clone(), 500).unwrap_err(),
            "agent_response_too_large",
            "{tool}"
        );
        let mut mcp = json!({"content":[{"type":"text","text":""}], "isError":false, "structuredContent":structured});
        assert_eq!(
            enforce_mcp_result_byte_cap(&mut mcp, 500, tool, 5).unwrap_err(),
            "agent_response_too_large",
            "{tool}"
        );
        assert_eq!(mcp["structuredContent"], structured);
        let mut framed = json!({"jsonrpc":"2.0","id":1,"result":mcp});
        assert_eq!(
            enforce_jsonrpc_response_byte_cap(&mut framed, 500).unwrap_err(),
            "agent_response_too_large",
            "{tool}"
        );
        assert_eq!(framed["result"]["structuredContent"], structured);
        let refusal = response_too_large(tool, "agent_response_too_large");
        assert!(refusal.to_string().len() <= 500);
        assert_eq!(refusal["isError"], true);
    }
}

#[test]
fn offset_row_pages_still_advance_after_byte_trimming() {
    for axis in ["items", "ledgers"] {
        let mut structured = json!({"result":{"offset":7}});
        structured["result"][axis] = json!((7..12)
            .map(|id| json!({"id":id,"padding":"x".repeat(160)}))
            .collect::<Vec<_>>());
        let (page, trimmed, count) = enforce_response_byte_cap(structured, 500).unwrap();
        assert!(trimmed);
        assert!(count > 0 && count < 5);
        assert_eq!(page["result"]["next_offset"], 7 + count);
        assert_eq!(page["result"][axis][0]["id"], 7);
        assert_eq!(page["result"][axis][count - 1]["id"], 6 + count);
        assert!(page.to_string().len() <= 500);
    }
}

/// bridge#551, #642: a base-currency-ledgers-only result nests its figures
/// under `base_currency_ledgers` and adds both lists of excluded ledgers. The
/// byte cap pages all four collections by the one offset, and never drops a
/// row.
#[test]
fn final_framing_caps_page_a_base_currency_ledgers_only_result() {
    let rows = |count| {
        (0..count)
            .map(|id| json!({"id":id,"padding":"x".repeat(120)}))
            .collect::<Vec<Value>>()
    };
    let page = |offset: usize| {
        let (bills, _, bill_next) = paginate_open_bills(rows(3), offset, 6);
        let (parties, _, party_next) = paginate_open_bills(rows(2), offset, 6);
        let (ledgers, _, ledger_next) = paginate_open_bills(rows(6), offset, 6);
        let (mixed, _, mixed_next) = paginate_open_bills(rows(4), offset, 6);
        json!({"result":{"state":"partial","partial_reason":"currency_ledgers_excluded",
            "partial_reasons":["foreign_currency_ledgers_excluded","mixed_currency_ledgers_excluded"],
            "base_currency_ledgers":{"offset":offset,"open_bills":bills,"next_offset":bill_next,
                "unallocated":{"count":2,"parties":parties,"next_offset":party_next,"truncated":party_next.is_some()}},
            "foreign_currency_ledgers_excluded":{"count":6,"ledgers":ledgers,"next_offset":ledger_next,
                "truncated":ledger_next.is_some()},
            "base_currency_ledgers_mixed_excluded":{"count":4,"reason":"mixed_currency_movement",
                "ledgers":mixed,"next_offset":mixed_next,"truncated":mixed_next.is_some()}}})
    };
    let ids = |rows: &Value| {
        rows.as_array()
            .unwrap()
            .iter()
            .map(|row| row["id"].as_u64().unwrap())
            .collect::<Vec<_>>()
    };
    let (mut offset, mut bills, mut parties, mut ledgers, mut mixed) =
        (0, Vec::new(), Vec::new(), Vec::new(), Vec::new());
    loop {
        let unbounded = page(offset);
        assert_eq!(
            response_row_count(&unbounded),
            Some(15 - 3.min(offset) - 2.min(offset) - 6.min(offset) - 4.min(offset))
        );
        // Four axes at one row each, with the reasons and both excluded
        // lists' framing, need more than the three-axis 1,000 bytes; the 15
        // unbounded rows (about 2,000 bytes) still have to page.
        let (response, _, _) = enforce_response_byte_cap(unbounded, 1400).unwrap();
        assert!(response.to_string().len() <= 1400);
        let result = &response["result"];
        bills.extend(ids(&result["base_currency_ledgers"]["open_bills"]));
        parties.extend(ids(
            &result["base_currency_ledgers"]["unallocated"]["parties"]
        ));
        ledgers.extend(ids(&result["foreign_currency_ledgers_excluded"]["ledgers"]));
        mixed.extend(ids(
            &result["base_currency_ledgers_mixed_excluded"]["ledgers"]
        ));
        let continuing = [
            result["base_currency_ledgers"]["next_offset"].as_u64(),
            result["base_currency_ledgers"]["unallocated"]["next_offset"].as_u64(),
            result["foreign_currency_ledgers_excluded"]["next_offset"].as_u64(),
            result["base_currency_ledgers_mixed_excluded"]["next_offset"].as_u64(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        let Some(next) = continuing.first() else {
            break;
        };
        assert!(continuing.iter().all(|cursor| cursor == next));
        assert!(*next > offset as u64);
        offset = *next as usize;
    }
    assert!(offset > 0, "the cap paged the result");
    assert_eq!(bills, [0, 1, 2]);
    assert_eq!(parties, [0, 1]);
    assert_eq!(ledgers, [0, 1, 2, 3, 4, 5]);
    assert_eq!(mixed, [0, 1, 2, 3]);
}

/// Trimming a complete outstandings result never adds an excluded-ledger
/// list to it: an absent list stays absent (bridge#642).
#[test]
fn final_framing_caps_add_no_excluded_list_to_a_complete_result() {
    let bills = (0..6)
        .map(|id| json!({"id":id,"padding":"x".repeat(120)}))
        .collect::<Vec<Value>>();
    let unbounded =
        json!({"result":{"state":"complete","offset":0,"open_bills":bills,"next_offset":null}});
    let (response, _, _) = enforce_response_byte_cap(unbounded, 600).unwrap();
    let result = &response["result"];
    assert!(
        result["open_bills"].as_array().unwrap().len() < 6,
        "the cap trimmed"
    );
    for key in [
        "foreign_currency_ledgers_excluded",
        "base_currency_ledgers_mixed_excluded",
    ] {
        assert!(result.get(key).is_none(), "{key} added: {result}");
    }
}

fn post_result_with_line_errors(texts: usize) -> Value {
    json!({"jsonrpc":"2.0","id":1,"result":{
        "content":[{"type":"text","text":""}], "isError":false,
        "structuredContent":{"result":{"dispatch":{"state":"reconciliation_required",
            "response":{"outcome":{
                "counters":{"line_error_count":texts + 1},
                "tally_line_errors":(0..texts)
                    .map(|_| json!({"text":"x".repeat(400),"truncated":false}))
                    .collect::<Vec<_>>(),
                "tally_line_errors_omitted":1
            }}}}}
    }})
}

/// Tally's LINEERROR text is cut first: a result that is over its cap only
/// because of the text comes back whole without it, the omission counted,
/// and is never refused. The same result without the text is unchanged.
#[test]
fn line_error_text_is_cut_before_a_result_is_refused() {
    let mut framed = post_result_with_line_errors(4);
    let mut bare = framed.clone();
    assert!(drop_tally_line_error_text(&mut bare));
    let cap = {
        let mut sized = bare.clone();
        set_mcp_content_json(&mut sized["result"]);
        sized.to_string().len() + 1
    };
    enforce_jsonrpc_response_byte_cap(&mut framed, cap).expect("fits without the text");
    let outcome =
        &framed["result"]["structuredContent"]["result"]["dispatch"]["response"]["outcome"];
    assert!(outcome.get("tally_line_errors").is_none(), "{outcome}");
    assert_eq!(outcome["tally_line_errors_omitted"], 5);
    assert_eq!(
        framed["result"]["structuredContent"],
        bare["result"]["structuredContent"]
    );
    // The text copy of the result is rebuilt too: no stale text survives,
    // and the whole frame is within the cap.
    assert!(framed.to_string().len() < cap);
    let text = framed["result"]["content"][0]["text"].as_str().unwrap();
    assert!(!text.contains(&"x".repeat(400)), "{text}");
    // Under a cap the result cannot meet even without the text, it is refused
    // exactly as a result that never had any.
    let mut still_too_large = post_result_with_line_errors(4);
    assert_eq!(
        enforce_jsonrpc_response_byte_cap(&mut still_too_large, 200).unwrap_err(),
        "agent_response_too_large"
    );
}

/// The new step acts only where the key is present: a value without it is
/// left byte-identical and reported as untouched, so every other tool's
/// result goes through the cap exactly as before.
#[test]
fn a_result_without_line_error_text_is_untouched_by_the_text_step() {
    for value in [
        json!({"result":{"items":[{"id":1}],"tally_line_errors_omitted":3,"line_errors":["kept"]}}),
        json!([{"outcome":{"line_error_count":1}}, "tally_line_errors", 7]),
        json!("tally_line_errors"),
    ] {
        let before = value.to_string();
        let mut after = value.clone();
        assert!(!drop_tally_line_error_text(&mut after));
        assert_eq!(after.to_string(), before);
    }
    // And a paged result without it is paged with no trace of the step.
    let mut response = json!({"result":{"offset":0,"items":
        (0..10_000).map(|id| json!({"id":id,"padding":"x".repeat(64)})).collect::<Vec<_>>()}});
    assert!(fit_response(&mut response, "", 512, |value| value.to_string().len()).unwrap());
    assert!(response["result"]
        .get("tally_line_errors_omitted")
        .is_none());
}

/// A result that must be paged anyway loses its LINEERROR text before any
/// row: it keeps exactly the rows the same result without the text keeps.
#[test]
fn a_paged_result_loses_its_line_error_text_before_any_row() {
    let paged = |with_text: bool| {
        let mut outcome = json!({"counters":{"line_error_count":8},"tally_line_errors_omitted":8});
        if with_text {
            outcome["tally_line_errors_omitted"] = json!(4);
            outcome["tally_line_errors"] = json!((0..4)
                .map(|_| json!({"text":"y".repeat(400),"truncated":false}))
                .collect::<Vec<_>>());
        }
        json!({"jsonrpc":"2.0","id":1,"result":{
            "content":[{"type":"text","text":""}], "isError":false,
            "structuredContent":{"result":{"offset":0,
                "items":(0..200).map(|id| json!({"id":id,"padding":"x".repeat(64)})).collect::<Vec<_>>(),
                "dispatch":{"response":{"outcome":outcome}}}}}})
    };
    let (mut with_text, mut without_text) = (paged(true), paged(false));
    enforce_jsonrpc_response_byte_cap(&mut with_text, 8_000).unwrap();
    enforce_jsonrpc_response_byte_cap(&mut without_text, 8_000).unwrap();
    let rows = |framed: &Value| {
        framed["result"]["structuredContent"]["result"]["items"]
            .as_array()
            .unwrap()
            .len()
    };
    assert!(rows(&without_text) < 200, "the cap pages this result");
    assert_eq!(rows(&with_text), rows(&without_text));
    assert_eq!(with_text, without_text);
    assert!(!with_text.to_string().contains("yyyy"));
}

/// Text inside an array is found in every element, each counted on its own.
#[test]
fn line_error_text_is_dropped_from_every_element_of_an_array() {
    let text = json!({"text":"z","truncated":false});
    let mut value = json!({"outcomes":[
        {"tally_line_errors":[text.clone()],"tally_line_errors_omitted":0},
        {"tally_line_errors":[text.clone(), text]}
    ]});
    assert!(drop_tally_line_error_text(&mut value));
    assert_eq!(
        value,
        json!({"outcomes":[{"tally_line_errors_omitted":1},{"tally_line_errors_omitted":2}]})
    );
}

/// A partial trial balance carries lists of excluded ledgers, which the cap
/// trims by their own shape; its `ledgers` page is then as long as before, so
/// its headline's rows sentence must not change.
#[test]
fn a_cap_that_trims_only_the_excluded_lists_leaves_the_rows_sentence_alone() {
    let date = |text: &str| bridge_tally_core::TallyDate::parse(text.to_string()).unwrap();
    let basis = headline::TrialBalanceBasis::new(
        date("20260401"),
        date("20260902"),
        headline::Completeness::from_gaps(vec![headline::Gap::BaseCurrencyLedgersOnly {
            foreign: 3,
            mixed: 0,
        }]),
    );
    let page = headline::Page::new(headline::Rows::Ledgers, 0, 5, 5);
    let headline = basis.headline(&headline::CompanyName::new("Synthetic"), page);
    let before = serde_json::to_value(&headline).unwrap();
    let mut response = json!({
        "headline": headline,
        "result": {
            "offset": 0,
            "ledgers": [1, 2, 3, 4, 5],
            "foreign_currency_ledgers_excluded": {"count": 3, "ledgers": [1, 2, 3]},
        },
    });
    assert!(truncate_response_items(&mut response).unwrap());
    assert_eq!(
        response["result"]["ledgers"].as_array().unwrap().len(),
        5,
        "the excluded lists were trimmed, not the ledgers"
    );
    assert_eq!(response["headline"], before);
}

/// A page the response size cuts keeps `limit` as asked, so the count of the
/// bills shown is restated from the rows that remain, at either nesting, and
/// the total stays the count of every bill. A payload that carries no count
/// gains none.
#[test]
fn a_size_cut_outstandings_page_restates_the_bills_shown() {
    let bills = (0..6)
        .map(|id| json!({"id": id, "padding": "x".repeat(120)}))
        .collect::<Vec<_>>();
    let figures = json!({"offset": 0, "limit": 6, "open_bills_total": 6,
        "open_bills_shown": 6, "open_bills": bills, "next_offset": null,
        "unallocated": {"count": 0, "parties": [], "next_offset": null, "truncated": false}});
    for nested in [false, true] {
        let result = if nested {
            json!({"state": "partial", "base_currency_ledgers": figures.clone(),
                "foreign_currency_ledgers_excluded": {"count": 0, "ledgers": [],
                    "next_offset": null, "truncated": false}})
        } else {
            let mut flat = figures.clone();
            flat["state"] = json!("complete");
            flat
        };
        let (response, _, _) = enforce_response_byte_cap(json!({"result": result}), 700).unwrap();
        let figures = if nested {
            &response["result"]["base_currency_ledgers"]
        } else {
            &response["result"]
        };
        let shown = figures["open_bills"].as_array().unwrap().len();
        assert!(shown > 0 && shown < 6, "the cap must cut the page: {shown}");
        assert_eq!(figures["open_bills_shown"], shown, "nested={nested}");
        assert_eq!(figures["open_bills_total"], 6, "nested={nested}");
        assert_eq!(figures["limit"], 6, "nested={nested}");
        assert_eq!(figures["next_offset"], shown, "nested={nested}");
    }

    let mut without = outstandings_page(0);
    without["result"]["open_bills"][0]["padding"] = json!("x".repeat(300));
    let (response, _, _) = enforce_response_byte_cap(without, 700).unwrap();
    assert!(
        response["result"]["open_bills"].as_array().unwrap().len() < 3,
        "the cap must cut the bills of this page too"
    );
    assert!(response["result"].get("open_bills_shown").is_none());
}

/// A summary's receipt counts its buckets as the rows prepared (#1230); before, it read zero.
#[test]
fn a_summary_receipt_counts_its_buckets() {
    let response = json!({"result": {"buckets": [{"group": "a"}, {"group": "b"}, {"group": "c"}]}});
    assert_eq!(response_row_count(&response), Some(3));
}
