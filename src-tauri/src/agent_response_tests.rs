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
fn a_paged_ledgers_result_is_trimmed_with_a_cursor_and_an_unpaged_one_is_not() {
    let mut paged = json!({"result":{"offset":0,"next_offset":null,
        "ledgers":[{"ledger":"A"},{"ledger":"B"},{"ledger":"C"}]}});
    assert!(truncate_response_items(&mut paged).expect("trims a ledgers page"));
    assert_eq!(paged["result"]["ledgers"].as_array().unwrap().len(), 2);
    assert_eq!(paged["result"]["next_offset"], 2);
    // negative_ledgers keys `ledgers` with no `offset`: a cursor into it would
    // resume nothing, so it is not trimmed.
    let mut unpaged = json!({"result":{"ledgers":[{"ledger":"A"},{"ledger":"B"}]}});
    assert!(!truncate_response_items(&mut unpaged).expect("no page shape"));
    assert_eq!(unpaged["result"]["ledgers"].as_array().unwrap().len(), 2);
    assert!(unpaged["result"].get("next_offset").is_none());
}

#[test]
fn a_paged_items_result_is_trimmed_with_a_cursor_and_an_unpaged_one_is_not() {
    // gst_status pages `items` by `offset`; gst_uncertain_vouchers returns `items` whole,
    // with counts and a line that cover the whole window, and takes no offset: a cursor into
    // it would resume nothing, so an over-budget answer is refused instead.
    let mut paged = json!({"result":{"offset":0,"next_offset":null,
        "items":[{"n":"A"},{"n":"B"},{"n":"C"}]}});
    assert!(truncate_response_items(&mut paged).expect("trims an items page"));
    assert_eq!(paged["result"]["items"].as_array().unwrap().len(), 2);
    assert_eq!(paged["result"]["next_offset"], 2);
    let mut unpaged = json!({"result":{"items":[{"n":"A"},{"n":"B"}]}});
    assert!(!truncate_response_items(&mut unpaged).expect("no page shape"));
    assert_eq!(unpaged["result"]["items"].as_array().unwrap().len(), 2);
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
            posting: Posting::Off,
            master_writes_enabled: false,
            audit_enabled: false,
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

// ---- the transfers section never makes a call fail as too large -----------------

fn transfers_section(rows: usize) -> Value {
    json!({
        "state": "computed", "skips_paired": 2, "skips_open": rows, "retired": 1,
        "open": (0..rows)
            .map(|n| json!({"bridge_txn_id": format!("st-20260105-{n:016x}"),
                "date": "20260105", "state": "not_found"}))
            .collect::<Vec<_>>(),
        "open_omitted": 0,
        "next_steps": {"not_found": "a step text ".repeat(25)},
    })
}

fn len_of(response: &Value) -> usize {
    response.to_string().len()
}

#[test]
fn open_rows_are_cut_before_the_verified_rows_of_a_page() {
    let items: Vec<Value> = (0..20)
        .map(|id| json!({"id": id, "padding": "x".repeat(40)}))
        .collect();
    let mut response = json!({"result": {"offset": 0, "items": items,
        "transfers": transfers_section(300)}});
    let without = {
        let mut bare = response.clone();
        bare["result"]["transfers"]["open"] = json!([]);
        len_of(&bare)
    };
    let cap = without + 2_000;
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert!(len_of(&response) <= cap);
    // every verified row is still there, and the section counts what it cut
    assert_eq!(response["result"]["items"].as_array().unwrap().len(), 20);
    let section = &response["result"]["transfers"];
    let kept = section["open"].as_array().unwrap().len();
    assert!(kept > 0 && kept < 300, "{kept}");
    assert_eq!(section["open_omitted"], 300 - kept);
    assert_eq!(section["skips_open"], 300);
    assert!(section["next_steps"]["open_omitted"]
        .as_str()
        .unwrap()
        .contains("BRIDGE_AGENT_MAX_BYTES"));
    // the rows kept are the first ones
    assert_eq!(
        section["open"][kept - 1]["bridge_txn_id"],
        format!("st-20260105-{:016x}", kept - 1)
    );
}

#[test]
fn a_result_with_no_rows_of_its_own_still_fits_by_cutting_the_section() {
    let mut response = json!({"result": {"batch_id": "bridge-x", "note": "y".repeat(500),
        "transfers": transfers_section(300)}});
    let cap = 3_000;
    assert!(len_of(&response) > cap);
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert!(len_of(&response) <= cap);
    assert_eq!(response["result"]["batch_id"], "bridge-x");
    assert!(
        response["result"]["transfers"]["open_omitted"]
            .as_u64()
            .unwrap()
            > 0
    );
}

#[test]
fn the_section_is_fitted_to_the_text_copy_and_its_escaping_too() {
    let mut mcp = json!({"content": [{"type": "text", "text": ""}],
        "structuredContent": {"result": {"batch_id": "bridge-x",
            "transfers": transfers_section(300)}}});
    let cap = 6_000;
    enforce_mcp_result_byte_cap(&mut mcp, cap, "build_import_xml", 0).unwrap();
    set_mcp_content_json(&mut mcp);
    assert!(len_of(&mcp) <= cap, "{}", len_of(&mcp));
    let section = &mcp["structuredContent"]["result"]["transfers"];
    assert!(section["open_omitted"].as_u64().unwrap() > 0);
    assert_eq!(section["skips_open"], 300);
}

#[test]
fn a_section_that_cannot_keep_its_texts_keeps_its_counts_and_then_only_a_note() {
    let section = transfers_section(300);
    let with = |section: Value| json!({"result": {"batch_id": "bridge-x", "transfers": section}});
    // room for the counts and the one step, not for the step texts
    let cap = len_of(&with(counts_only_section(&section)));
    assert!(len_of(&with(trim_section(&section, 0))) > cap);
    let mut counts = with(section.clone());
    assert!(fit_response(&mut counts, "", cap, |value| len_of(value)).unwrap());
    let kept = &counts["result"]["transfers"];
    assert_eq!(kept["state"], "computed");
    assert_eq!(kept["skips_open"], 300);
    assert_eq!(kept["open"], json!([]));
    assert_eq!(kept["open_omitted"], 300);
    assert_eq!(
        kept["next_steps"]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        ["open_omitted"]
    );
    // room for nothing but a note that the check is to be run again
    let cap = len_of(&with(check_not_run_section(false)));
    let mut note = with(section);
    assert!(fit_response(&mut note, "", cap, |value| len_of(value)).unwrap());
    let kept = &note["result"]["transfers"];
    assert_eq!(kept["state"], "not_established");
    assert!(kept.get("skips_open").is_none());
    assert!(kept["next_steps"]["check_not_run"].is_string());
}

#[test]
fn a_section_that_already_fits_or_is_not_computed_is_left_alone() {
    let mut fits = json!({"result": {"transfers": transfers_section(3)}});
    let before = fits.clone();
    assert!(!fit_response(&mut fits, "", 100_000, |value| len_of(value)).unwrap());
    assert_eq!(fits, before);
    // a note is not cut, so a response too large for its page is the page's
    let mut note = json!({"result": {"items": [], "padding": "x".repeat(2_000),
        "transfers": {"state": "not_established", "next_steps": {}}}});
    assert_eq!(
        fit_response(&mut note, "", 500, |value| len_of(value)),
        Err("agent_response_too_large".to_string())
    );
    // and a page cut around it leaves it as it was
    let items: Vec<Value> = (0..50).map(|id| json!({"id": id})).collect();
    let mut paged = json!({"result": {"offset": 0, "items": items,
        "transfers": {"state": "not_established", "next_steps": {"check_not_run": "again"}}}});
    let before = paged["result"]["transfers"].clone();
    assert!(fit_response(&mut paged, "", 300, |value| len_of(value)).unwrap());
    assert!(paged["result"]["items"].as_array().unwrap().len() < 50);
    assert_eq!(paged["result"]["transfers"], before);
}

// ---- the section is fitted again into what a trimmed page leaves ----------------

fn verify_page(items: usize, section: Value) -> Value {
    json!({"result": {
        "verification_status": "posted_verified", "counts": {"posted_verified": items},
        "offset": 0,
        "items": (0..items)
            .map(|id| json!({"id": id, "padding": "x".repeat(1_500)}))
            .collect::<Vec<_>>(),
        "transfers": section,
    }})
}

/// A page whose own rows exceed the cap: the check did run, so the section
/// stays computed, widened into what the trimmed page leaves.
#[test]
fn a_page_cut_to_the_cap_leaves_its_computed_section_as_wide_as_the_room_allows() {
    let section = transfers_section(40);
    // room for five page rows beside the counts, and for some open rows, not for a sixth page row
    let mut five = verify_page(30, counts_only_section(&section));
    retain_page_width(&mut five, PageShape::Rows("items"), 5).unwrap();
    let cap = len_of(&five) + 700;
    let mut response = verify_page(30, section.clone());
    assert!(len_of(&response) > cap);
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert!(len_of(&response) <= cap);
    // the page is cut to the rows that fit beside the counts, its verdict is not
    assert_eq!(response["result"]["items"].as_array().unwrap().len(), 5);
    assert_eq!(response["result"]["verification_status"], "posted_verified");
    assert_eq!(response["result"]["counts"], json!({"posted_verified": 30}));
    // the check ran: the section is computed, with every open row kept or counted
    let shown = &response["result"]["transfers"];
    assert_eq!(shown["state"], "computed");
    assert_eq!(shown["skips_open"], 40);
    let rows = shown["open"].as_array().unwrap().len();
    assert!(rows > 0 && rows < 40, "{rows}");
    assert_eq!(shown["open_omitted"], 40 - rows);
    // and it is as wide as the trimmed page leaves room for
    let mut wider = response.clone();
    wider["result"]["transfers"] = trim_section(&section, rows + 1);
    assert!(len_of(&wider) > cap);
}

#[test]
fn a_page_cut_to_the_cap_keeps_its_counts_when_no_row_fits_beside_it() {
    let section = transfers_section(40);
    let mut response = verify_page(30, section.clone());
    // one page row and the counts, and no room for an open row
    let mut one = verify_page(1, counts_only_section(&section));
    one["result"]["offset"] = json!(0);
    let cap = len_of(&one) + 40;
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert!(len_of(&response) <= cap);
    assert_eq!(response["result"]["items"].as_array().unwrap().len(), 1);
    let shown = &response["result"]["transfers"];
    assert_eq!(shown["state"], "computed");
    assert_eq!(shown["open"], json!([]));
    assert_eq!(shown["open_omitted"], 40);
}

#[test]
fn a_page_cut_to_the_cap_says_the_check_is_to_be_run_again_only_when_not_even_counts_fit() {
    let section = transfers_section(40);
    let mut response = verify_page(30, section.clone());
    let one = verify_page(1, check_not_run_section(false));
    let cap = len_of(&one) + 40;
    assert!(len_of(&verify_page(1, counts_only_section(&section))) > cap);
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert!(len_of(&response) <= cap);
    assert_eq!(response["result"]["transfers"]["state"], "not_established");
}

/// `truncated` on the whole answer stays true when only the section's rows were
/// cut: something was left out, and the flag is the conservative reading.
#[test]
fn cutting_only_the_section_still_reports_that_something_was_cut() {
    let mut response = json!({"result": {"batch_id": "bridge-x",
        "transfers": transfers_section(300)}});
    let cap = 3_000;
    let (fitted, truncated, _) = enforce_response_byte_cap(response.clone(), cap).unwrap();
    assert!(truncated);
    assert!(
        fitted["result"]["transfers"]["open_omitted"]
            .as_u64()
            .unwrap()
            > 0
    );
    // a section that already fits reports nothing cut
    response["result"]["transfers"] = transfers_section(2);
    let (_, truncated, _) = enforce_response_byte_cap(response, 100_000).unwrap();
    assert!(!truncated);
}

#[test]
fn the_note_that_the_check_did_not_run_names_the_call_that_made_it() {
    let batch = check_not_run_section(false);
    assert_eq!(
        batch["next_steps"]["check_not_run"],
        "Not run, or cut to fit: verify again; if it repeats, raise BRIDGE_AGENT_MAX_BYTES."
    );
    let recorded = check_not_run_section(true);
    assert_eq!(
        recorded["next_steps"]["check_not_run"],
        "The skipped rows are recorded; the transfer check will show on the next build or verification for this company."
    );
}

#[test]
fn a_recorded_statement_that_cannot_keep_its_section_is_told_the_next_call_will_show_it() {
    let section = transfers_section(40);
    let with = |section: Value| json!({"result": {"state": "skips_recorded", "record_id": "skips-x", "transfers": section}});
    let cap = len_of(&with(check_not_run_section(true)));
    assert!(len_of(&with(counts_only_section(&section))) > cap);
    let mut response = with(section);
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert_eq!(
        response["result"]["transfers"]["next_steps"]["check_not_run"],
        "The skipped rows are recorded; the transfer check will show on the next build or verification for this company."
    );
}

fn small_page(items: usize, section: Value) -> Value {
    json!({"result": {"offset": 0, "verification_status": "posted_verified",
        "items": (0..items).map(|id| json!({"id": id})).collect::<Vec<_>>(),
        "transfers": section}})
}

/// A page that fits beside the note at full width is not cut, however many open
/// rows the section has and however little room the counts would need.
#[test]
fn a_page_that_fits_with_the_note_at_full_width_is_not_cut() {
    let section = transfers_section(40);
    let cap = len_of(&small_page(30, check_not_run_section(false))) + 5;
    assert!(len_of(&small_page(30, counts_only_section(&section))) > cap);
    let mut response = small_page(30, section);
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert_eq!(response["result"]["items"].as_array().unwrap().len(), 30);
    assert!(response["result"].get("next_offset").is_none());
    assert_eq!(response["result"]["transfers"]["state"], "not_established");
    assert!(len_of(&response) <= cap);
}

/// A page of one row, or none, has no width to give up: the section is what
/// gives way, down to the note.
#[test]
fn a_page_of_one_row_or_none_keeps_its_rows_and_the_section_gives_way_to_the_note() {
    let section = transfers_section(40);
    for rows in [0, 1] {
        let cap = len_of(&small_page(rows, check_not_run_section(false))) + 5;
        assert!(len_of(&small_page(rows, counts_only_section(&section))) > cap);
        let mut response = small_page(rows, section.clone());
        assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
        assert_eq!(response["result"]["items"].as_array().unwrap().len(), rows);
        assert_eq!(response["result"]["transfers"]["state"], "not_established");
        assert!(len_of(&response) <= cap);
    }
}

/// A page whose rows alone overflow is trimmed with the note beside it, and the
/// section is then widened into what is left. Here the trimmed page leaves room
/// for the note and not the counts.
#[test]
fn an_overflowing_page_is_trimmed_beside_the_note_and_the_section_widened_after() {
    let section = transfers_section(40);
    let mut fifteen = small_page(60, check_not_run_section(false));
    retain_page_width(&mut fifteen, PageShape::Rows("items"), 15).unwrap();
    let cap = len_of(&fifteen) + 8;
    let mut response = small_page(60, section);
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert_eq!(response["result"]["items"].as_array().unwrap().len(), 15);
    assert_eq!(response["result"]["transfers"]["state"], "not_established");
    assert!(len_of(&response) <= cap);
}

/// A section with no open rows has counts shorter than the note: the page is
/// trimmed beside the counts, so it keeps every row the counts leave room for.
#[test]
fn an_overflowing_page_is_trimmed_beside_the_counts_when_they_are_shorter_than_the_note() {
    let section = transfers_section(0);
    let counts = counts_only_section(&section);
    assert!(len_of(&counts) < len_of(&check_not_run_section(false)));
    let mut fifteen = small_page(60, counts.clone());
    retain_page_width(&mut fifteen, PageShape::Rows("items"), 15).unwrap();
    let cap = len_of(&fifteen) + 2;
    let mut with_note = small_page(60, check_not_run_section(false));
    retain_page_width(&mut with_note, PageShape::Rows("items"), 15).unwrap();
    assert!(len_of(&with_note) > cap);
    let mut response = small_page(60, section);
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert_eq!(response["result"]["items"].as_array().unwrap().len(), 15);
    assert_eq!(response["result"]["transfers"], counts);
    assert!(len_of(&response) <= cap);
}

/// With no computed section the page is trimmed exactly as it always was, and
/// a page that cannot fit fails.
#[test]
fn a_result_with_no_computed_section_takes_the_old_path() {
    for transfers in [
        None,
        Some(json!({"state": "not_established", "next_steps": {}})),
    ] {
        let page = |items: usize| {
            let mut page = small_page(items, Value::Null);
            match &transfers {
                Some(section) => page["result"]["transfers"] = section.clone(),
                None => {
                    page["result"].as_object_mut().unwrap().remove("transfers");
                }
            }
            page
        };
        let mut five = page(60);
        retain_page_width(&mut five, PageShape::Rows("items"), 5).unwrap();
        let cap = len_of(&five) + 3;
        let mut response = page(60);
        assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
        assert_eq!(response["result"]["items"].as_array().unwrap().len(), 5);
        assert_eq!(response["result"].get("transfers"), transfers.as_ref());
        // a page of one row that does not fit fails
        let mut one = page(1);
        assert_eq!(
            fit_response(&mut one, "", 20, |value| len_of(value)),
            Err("agent_response_too_large".to_string())
        );
        let mut none = page(0);
        assert_eq!(
            fit_response(&mut none, "", 20, |value| len_of(value)),
            Err("agent_response_too_large".to_string())
        );
    }
}

/// A written build has no page to trim. When it fits without its section but
/// not beside the note that the check did not run, it is answered without the
/// section and reported as cut: it is never turned into an error after its
/// file and journal record are written.
#[test]
fn a_result_with_no_page_that_fits_only_without_its_section_is_answered_without_it() {
    let bare =
        json!({"truncated": false, "result": {"batch_id": "bridge-x", "note": "y".repeat(500)}});
    let mut response = bare.clone();
    response["result"]["transfers"] = transfers_section(40);
    let cap = len_of(&bare) + 10;
    let mut noted = bare.clone();
    noted["result"]["transfers"] = check_not_run_section(false);
    assert!(len_of(&noted) > cap);
    assert!(fit_response(&mut response, "", cap, |value| len_of(value)).unwrap());
    assert!(response["result"].get("transfers").is_none(), "{response}");
    assert_eq!(response["result"]["batch_id"], "bridge-x");
    assert_eq!(response["truncated"], true);
    // through the cap itself: answered, and reported as cut
    let mut with_section = bare.clone();
    with_section["result"]["transfers"] = transfers_section(40);
    let (fitted, truncated, _) = enforce_response_byte_cap(with_section, cap).unwrap();
    assert!(truncated);
    assert!(fitted["result"].get("transfers").is_none(), "{fitted}");
    assert_eq!(fitted["result"]["batch_id"], "bridge-x");
    // a result too large without its section is still too large
    let mut too_big = bare.clone();
    too_big["result"]["transfers"] = transfers_section(40);
    assert_eq!(
        fit_response(&mut too_big, "", 100, |value| len_of(value)),
        Err("agent_response_too_large".to_string())
    );
}

/// Whatever the section is, a note that the check did not run included: a
/// result with no page that fits without it is answered without it.
#[test]
fn a_result_with_no_page_drops_a_note_section_too() {
    let bare =
        json!({"truncated": false, "result": {"batch_id": "bridge-x", "note": "y".repeat(500)}});
    let mut noted = bare.clone();
    noted["result"]["transfers"] = check_not_run_section(false);
    let cap = len_of(&bare) + 5;
    assert!(len_of(&noted) > cap);
    assert!(fit_response(&mut noted, "", cap, |value| len_of(value)).unwrap());
    assert!(noted["result"].get("transfers").is_none(), "{noted}");
    assert_eq!(noted["truncated"], true);
}

/// The same drop through the two other places the cap is applied to a tool
/// result: the MCP result's `structuredContent`, and the JSON-RPC response.
#[test]
fn a_result_with_no_page_drops_its_section_in_the_mcp_and_jsonrpc_envelopes_too() {
    let bare =
        json!({"truncated": false, "result": {"batch_id": "bridge-x", "note": "y".repeat(500)}});
    let with_section = |section: Value| {
        let mut full = bare.clone();
        full["result"]["transfers"] = section;
        full
    };
    // MCP: structuredContent beside its text copy
    let mcp = |structured: Value| {
        let mut mcp =
            json!({"content": [{"type": "text", "text": ""}], "structuredContent": structured});
        set_mcp_content_json(&mut mcp);
        mcp
    };
    let cap = len_of(&mcp(bare.clone())) + 10;
    for section in [transfers_section(40), check_not_run_section(false)] {
        assert!(len_of(&mcp(with_section(section.clone()))) > cap);
        let mut response = mcp(with_section(section));
        enforce_mcp_result_byte_cap(&mut response, cap, "build_import_xml", 0).unwrap();
        let structured = &response["structuredContent"];
        assert!(
            structured["result"].get("transfers").is_none(),
            "{response}"
        );
        assert_eq!(structured["truncated"], true);
        assert!(len_of(&response) <= cap);
    }
    // JSON-RPC: the same under result.structuredContent
    let rpc = |structured: Value| {
        let mut rpc = json!({"jsonrpc": "2.0", "id": 1, "result": {"content": [], "structuredContent": structured}});
        set_mcp_content_json(&mut rpc["result"]);
        rpc
    };
    let cap = len_of(&rpc(bare.clone())) + 11;
    let mut response = rpc(with_section(transfers_section(40)));
    enforce_jsonrpc_response_byte_cap(&mut response, cap).unwrap();
    assert!(response["result"]["structuredContent"]["result"]
        .get("transfers")
        .is_none());
    assert_eq!(response["result"]["structuredContent"]["truncated"], true);
    // a result too large without the section still fails in both
    let mut too_big = mcp(with_section(transfers_section(40)));
    assert_eq!(
        enforce_mcp_result_byte_cap(&mut too_big, 100, "build_import_xml", 0),
        Err("agent_response_too_large".to_string())
    );
}

/// The egress log's answer names its order, so a result over a cap keeps its newest receipts: the
/// cap is checked at three stages, each of which may cut again, and the count left out adds up
/// across them. An answer that does not name its order (the evidence log's) is not trimmed.
#[test]
fn a_newest_first_list_is_trimmed_to_its_newest_and_counts_what_every_stage_cut() {
    let records = (0..8)
        .map(|id| {
            json!(format!(
                "{{\"index\":{id},\"padding\":\"{}\"}}",
                "x".repeat(300)
            ))
        })
        .collect::<Vec<_>>();
    let answer =
        json!({"truncated": false, "result": {"records": records, "order": "newest_first"}});
    let size = answer.to_string().len();
    // First stage: a cap that holds about five of the eight.
    let (first, trimmed, _) = enforce_response_byte_cap(answer.clone(), size * 5 / 8).unwrap();
    assert!(trimmed);
    assert_eq!(first["truncated"], true);
    let kept = first["result"]["records"].as_array().unwrap().len();
    assert!((1..8).contains(&kept));
    assert_eq!(first["result"]["omitted_for_size"], 8 - kept as u64);
    assert!(first.to_string().len() <= size * 5 / 8);
    // The kept receipts are the first of the list, which is the newest.
    assert_eq!(
        first["result"]["records"][0],
        answer["result"]["records"][0]
    );
    // A later stage with a smaller cap cuts again, and the two counts add up.
    let mut mcp =
        json!({"content":[{"type":"text","text":""}], "isError":false, "structuredContent":first});
    enforce_mcp_result_byte_cap(&mut mcp, size, "egress_log", 0).unwrap();
    let again = mcp["structuredContent"]["result"]["records"]
        .as_array()
        .unwrap()
        .len();
    assert!(again < kept);
    assert_eq!(
        mcp["structuredContent"]["result"]["omitted_for_size"],
        8 - again as u64
    );
    // Nothing named its order: the same list is refused, not trimmed.
    let mut unnamed = answer;
    unnamed["result"].as_object_mut().unwrap().remove("order");
    assert_eq!(
        enforce_response_byte_cap(unnamed, size / 4).unwrap_err(),
        "agent_response_too_large"
    );
}
