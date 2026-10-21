//! `audit_test_run` through the MCP adapter, replayed from the tax-audit
//! crate's synthetic read (cut to thirty vouchers so the production window
//! limits read it as one request).
use super::super::audit_read::tests::{companies_xml, plan, Replay, FIXTURE_GUID};
use super::super::*;
use super::*;
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator,
};

struct OneServer {
    simulator: Option<SequenceSimulator>,
    server: Server,
    _directory: tempfile::TempDir,
}

fn serve(plans: Vec<tally_protocol_simulator::ScenarioPlan>, redaction: Redaction) -> OneServer {
    serve_with(plans, redaction, true)
}

/// A server with the tax-audit setting as given (the tool's own tests turn it on).
fn serve_with(
    plans: Vec<tally_protocol_simulator::ScenarioPlan>,
    redaction: Redaction,
    audit_enabled: bool,
) -> OneServer {
    let simulator = (!plans.is_empty()).then(|| SequenceSimulator::spawn(plans).unwrap());
    // With no plan the port is one nothing listens on: a request sent would
    // fail as a refused connection, not as the refusal a test looks for.
    let port = simulator
        .as_ref()
        .map_or(9, |simulator| simulator.address().port());
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port,
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction,
        import_enabled: false,
        posting: Posting::Off,
        audit_enabled,
    });
    OneServer {
        simulator,
        server,
        _directory: directory,
    }
}

/// The paired company list the tool reads first, then the audit read's requests.
fn replayed(replay: &Replay, redaction: Redaction) -> OneServer {
    let status = || {
        ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
            .with_framing(ResponseFraming::ContentLength)
    };
    let mut plans = vec![
        plan(companies_xml(FIXTURE_GUID)),
        status(),
        plan(companies_xml(FIXTURE_GUID)),
        status(),
    ];
    plans.extend(replay.plans());
    serve(plans, redaction)
}

fn args() -> Value {
    json!({
        "company_guid": FIXTURE_GUID,
        "from": "20250401",
        "to": "20260331",
        "test": "cash_44ab",
        "cash_groups": ["Cash-in-Hand"],
        "bank_groups": ["Bank Accounts", "Bank OD A/c"],
    })
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

fn error_code(response: &Value) -> &str {
    assert_eq!(response["isError"], true, "{response}");
    response["structuredContent"]["result"]["error"]["code"]
        .as_str()
        .unwrap()
}

/// Every string in a JSON value, however deep, keys included.
fn strings(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(text) => out.push(text.clone()),
        Value::Array(items) => items.iter().for_each(|item| strings(item, out)),
        Value::Object(map) => {
            for (key, item) in map {
                out.push(key.clone());
                strings(item, out);
            }
        }
        _ => {}
    }
}

/// The names the book holds: every group and ledger, and the company.
fn book_names() -> Vec<String> {
    let names_in = |bytes: &[u8], element: &str| {
        let text = super::super::audit_read::tests::text_of(bytes);
        let open = format!("<{element} NAME=\"");
        text.match_indices(&open)
            .map(|(at, _)| {
                let start = at + open.len();
                text[start..start + text[start..].find('"').unwrap()].to_string()
            })
            .collect::<Vec<_>>()
    };
    let mut names = names_in(
        include_bytes!(
            "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/ledgers.xml"
        ),
        "LEDGER",
    );
    names.extend(names_in(
        include_bytes!("../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/groups.xml"),
        "GROUP",
    ));
    names.push(super::super::audit_read::tests::FIXTURE_NAME.to_string());
    // The XML spells `&` as `&amp;`; the book's name is the unescaped text.
    let mut names: Vec<String> = names
        .into_iter()
        .map(|name| name.replace("&amp;", "&"))
        .collect();
    // The vouchers' own numbers and GUIDs, narrations aside.
    let vouchers = super::super::audit_read::tests::text_of(include_bytes!(
        "../crates/bridge-tax-audit/tests/fixtures/synthetic-read/parts/vouchers_h1.xml"
    ));
    for element in ["VOUCHERNUMBER", "GUID"] {
        let open = format!("<{element}>");
        let close = format!("</{element}>");
        for (at, _) in vouchers.match_indices(&open) {
            let start = at + open.len();
            names
                .push(vouchers[start..start + vouchers[start..].find(&close).unwrap()].to_string());
        }
    }
    assert!(names.len() > 60, "the book's names were not found");
    names
}

#[tokio::test]
async fn a_completed_run_returns_figures_and_findings_and_no_name_of_the_book() {
    let one = replayed(&Replay::small(), Redaction::None);
    let response = one.server.call_tool("audit_test_run", args()).await;
    let run = result(&response);
    // The book is cut to thirty vouchers against the whole year's trial
    // balance, so the checks that tie vouchers to it fail, and the state says
    // so rather than `completed`.
    assert_eq!(run["state"], "completed_with_failed_book_checks");
    assert_eq!(
        run["book_checks"]["failed"],
        json!([
            {"count": 1, "invariant": "MAP-1"},
            {"count": 27, "invariant": "POP-1"},
            {"count": 1, "invariant": "POP-2"},
            {"count": 1, "invariant": "POP-3"},
        ])
    );
    assert_eq!(run["test"]["test_id"], "cash_44ab");

    // The same figures the crate's own golden holds for this test.
    let golden: Value = serde_json::from_slice(include_bytes!(
        "../crates/bridge-tax-audit/tests/fixtures/golden/synthetic.cash_44ab.json"
    ))
    .unwrap();
    let ids = |figures: &Value| {
        figures
            .as_array()
            .unwrap()
            .iter()
            .map(|figure| figure["id"].as_str().unwrap().to_string())
            .collect::<std::collections::BTreeSet<_>>()
    };
    assert_eq!(ids(&run["figures"]), ids(&golden["figures"]));
    // The values are computed, not zero: the cut book holds cash and bank
    // receipts, so both are positive and the share is between 0 and 10,000 bp.
    let value_of = |id: &str| {
        run["figures"]
            .as_array()
            .unwrap()
            .iter()
            .find(|figure| figure["id"] == id)
            .unwrap_or_else(|| panic!("no figure {id}"))["value"]
            .as_i64()
            .unwrap_or_else(|| panic!("figure {id} has no integer value"))
    };
    assert!(value_of("cash_44ab.cash_receipts") > 0);
    assert!(value_of("cash_44ab.bank_receipts") > 0);
    assert!((0..=10_000).contains(&value_of("cash_44ab.cash_share_receipts")));
    assert_eq!(run["findings"][0]["id"], "cash_44ab/applicability");
    assert!(run["findings"][0].get("evidence").is_none());
    assert!(run["figures"][0].get("evidence").is_none());

    // No string of the result is, or holds, a name from the book. The company
    // object beside the result is the user's own company, named on purpose.
    let mut held = Vec::new();
    strings(run, &mut held);
    for name in book_names() {
        assert!(
            !held
                .iter()
                .any(|text| text == &name || (name.len() >= 10 && text.contains(&name))),
            "the result carries a name of the book"
        );
    }
    // The paired company list (four requests), then ten parts (the window's
    // census included), each between two company checks.
    assert_eq!(one.simulator.unwrap().finish().unwrap().len(), 34);
}

#[tokio::test]
async fn a_redaction_setting_is_refused_before_any_request() {
    for redaction in [Redaction::MaskParties, Redaction::DropNarration] {
        let one = serve(Vec::new(), redaction);
        let response = one.server.call_tool("audit_test_run", args()).await;
        assert_eq!(error_code(&response), REDACTION_UNSUPPORTED);
        assert_eq!(response["structuredContent"]["evidence"]["bytes"], 0);
        // The refusal is the whole answer: no figure, finding or name.
        assert!(response["structuredContent"]["result"]
            .get("figures")
            .is_none());
        let mut held = Vec::new();
        strings(&response["structuredContent"], &mut held);
        for name in book_names() {
            assert!(!held.iter().any(|text| text == &name));
        }
    }
}

#[tokio::test]
async fn arguments_that_cannot_be_honoured_are_refused_before_any_request() {
    let mut cases = Vec::new();
    let mut other_test = args();
    other_test["test"] = json!("clause44");
    cases.push(other_test);
    let mut backwards = args();
    backwards["from"] = json!("20260331");
    backwards["to"] = json!("20250401");
    cases.push(backwards);
    for groups in [
        json!([]),
        json!([""]),
        json!(["a\nb"]),
        json!("Cash-in-Hand"),
    ] {
        let mut bad = args();
        bad["cash_groups"] = groups;
        cases.push(bad);
    }
    let mut missing = args();
    missing.as_object_mut().unwrap().remove("bank_groups");
    cases.push(missing);
    let mut both = args();
    both["bank_groups"] = json!(["Bank Accounts", " cash-in-hand "]);
    cases.push(both.clone());
    for case in cases {
        let one = serve(Vec::new(), Redaction::None);
        let response = one.server.call_tool("audit_test_run", case.clone()).await;
        assert_eq!(response["isError"], true, "{case}");
        assert_eq!(
            response["structuredContent"]["evidence"]["bytes"], 0,
            "{case}"
        );
        let code = error_code(&response);
        assert_ne!(code, "company_collection_invalid", "{case}");
        assert!(
            !code.contains("unreachable") && !code.contains("connection"),
            "{case}: {code}"
        );
    }
    // A group named as cash and as bank (spelt with other case and spaces).
    let one = serve(Vec::new(), Redaction::None);
    let response = one.server.call_tool("audit_test_run", both).await;
    assert_eq!(error_code(&response), "audit_groups_overlap");
}

#[tokio::test]
async fn marks_that_moved_during_the_read_refuse_it_with_a_rule_code() {
    let mut replay = Replay::small();
    replay.marks_before = replay.marks_before.replace("> 30<", "> 29<");
    let one = replayed(&replay, Redaction::None);
    let response = one.server.call_tool("audit_test_run", args()).await;
    assert_eq!(error_code(&response), "C6-moved");
    drop(one.simulator);
    // The refusal says nothing of the book.
    let mut held = Vec::new();
    strings(&response["structuredContent"]["result"]["error"], &mut held);
    for name in book_names() {
        assert!(!held
            .iter()
            .any(|text| text == &name || (name.len() >= 10 && text.contains(&name))));
    }
}

#[test]
fn the_catalog_lists_exactly_the_admitted_tests() {
    let definitions = catalog::registered_tool_definitions(true, true);
    let tool = definitions
        .as_array()
        .unwrap()
        .iter()
        .find(|tool| tool["name"] == "audit_test_run")
        .expect("the tool is registered");
    let listed: Vec<&str> = tool["inputSchema"]["properties"]["test"]["enum"]
        .as_array()
        .unwrap()
        .iter()
        .map(|test| test.as_str().unwrap())
        .collect();
    assert_eq!(listed, ADMITTED);
    assert_eq!(ADMITTED, ["cash_44ab"]);
    for test in ADMITTED {
        assert!(registry::find(test).is_some());
    }
}

fn dump(figures: usize, findings: usize) -> Value {
    json!({
        "test_id": "t", "test_version": "1", "rules_version": "1",
        "population_note_text": "note",
        "figures": (0..figures).map(|n| json!({"id": format!("t.f{n}"), "value": n, "unit": "paise", "definition_text": "d", "evidence": [{"kind": "ledger", "id": "Secret Ledger"}]})).collect::<Vec<_>>(),
        "findings": (0..findings).map(|n| json!({"id": format!("t/f{n}"), "confidence": "computed", "clauses": [], "facts": [], "title_text": "x", "limits_text": [], "ask_client_text": [], "evidence": [{"kind": "ledger", "id": "Secret Ledger"}]})).collect::<Vec<_>>(),
        "book_invariants_evaluated": ["POP-1"],
        "book_invariant_violations": [
            {"invariant": "POP-1", "subject": "Secret Ledger", "detail": "vouchers 1 vs TB 2"},
            {"invariant": "POP-1", "subject": "Another Secret", "detail": "x"},
            {"invariant": "MAP-1", "subject": "Secret Ledger", "detail": "y"},
        ],
        "result_invariant_violations": [],
        "module_invariant_violations": [],
    })
}

#[test]
fn the_projection_keeps_ids_values_and_counts_and_drops_every_name() {
    let projected = project(&dump(2, 1)).unwrap();
    assert_eq!(projected["state"], "completed_with_failed_book_checks");
    let text = projected.to_string();
    assert!(
        !text.contains("Secret") && !text.contains("evidence"),
        "{text}"
    );
    assert_eq!(projected["figures"].as_array().unwrap().len(), 2);
    assert_eq!(
        projected["book_checks"]["failed"],
        json!([{"invariant": "MAP-1", "count": 1}, {"invariant": "POP-1", "count": 2}])
    );
}

#[test]
fn an_id_value_or_unit_that_could_hold_a_name_is_not_projected() {
    let mut text_value = dump(1, 1);
    text_value["figures"][0]["value"] = json!("Secret Ledger");
    let mut named_id = dump(1, 1);
    named_id["figures"][0]["id"] = json!("t.Secret Ledger");
    let mut named_finding = dump(1, 1);
    named_finding["findings"][0]["id"] = json!("t/Secret Ledger");
    let mut named_fact = dump(1, 1);
    named_fact["findings"][0]["facts"] = json!([{"name": "Secret Ledger", "figure_id": "t.f0"}]);
    let mut text_unit = dump(1, 1);
    text_unit["figures"][0]["unit"] = json!("text");
    for case in [text_value, named_id, named_finding, named_fact, text_unit] {
        assert_eq!(
            project(&case).unwrap_err(),
            RESULT_NOT_PROJECTABLE,
            "{case}"
        );
    }
    // A null value (a share with a zero denominator) is a value.
    let mut undefined = dump(1, 1);
    undefined["figures"][0]["value"] = Value::Null;
    assert!(project(&undefined).is_ok());
}

#[tokio::test]
async fn a_period_outside_the_year_the_rules_are_for_is_refused_before_any_request() {
    for (from, to) in [
        ("20240401", "20250331"),
        ("20250331", "20260331"),
        ("20250401", "20260401"),
        ("20250401", "20270331"),
    ] {
        let one = serve(Vec::new(), Redaction::None);
        let mut period = args();
        period["from"] = json!(from);
        period["to"] = json!(to);
        let response = one.server.call_tool("audit_test_run", period).await;
        assert_eq!(error_code(&response), PERIOD_OUTSIDE_RULES, "{from} {to}");
        assert_eq!(response["structuredContent"]["evidence"]["bytes"], 0);
    }
}

#[test]
fn a_result_past_the_bounds_is_refused_and_never_cut_short() {
    assert!(project(&dump(MAX_FIGURES, MAX_FINDINGS)).is_ok());
    assert_eq!(
        project(&dump(MAX_FIGURES + 1, 0)).unwrap_err(),
        RESULT_TOO_LARGE
    );
    assert_eq!(
        project(&dump(0, MAX_FINDINGS + 1)).unwrap_err(),
        RESULT_TOO_LARGE
    );
    assert_eq!(
        project(&json!({"figures": 1})).unwrap_err(),
        RESULT_NOT_PROJECTABLE
    );
}

/// Off by default: with the setting off the tool is not listed, and a call naming it is refused
/// in band before any request; with it on it is listed.
#[tokio::test]
async fn the_tool_is_absent_and_refused_while_its_setting_is_off() {
    let listed = |audit| {
        catalog::listed_tool_definitions(false, false, audit)
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["name"] == "audit_test_run")
    };
    assert!(!listed(false));
    assert!(listed(true));
    let one = serve_with(Vec::new(), Redaction::None, false);
    let response = one.server.call_tool("audit_test_run", args()).await;
    assert_eq!(error_code(&response), "audit_tool_disabled");
    assert_eq!(response["structuredContent"]["evidence"]["bytes"], 0);
}
