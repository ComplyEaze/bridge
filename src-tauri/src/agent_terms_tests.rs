use super::super::*;
use super::{TermsGate, TermsState, RECORD_FILE, TERMS_ENV, TERMS_VERSION};
use std::path::Path;

/// Settings whose Tally endpoint is a port nothing listens on (the discard port). A tool that got
/// past the gate would come back with a Tally-side error, not `terms_not_accepted`, so a refusal
/// code is also the proof that the call stopped before Tally.
fn settings(data_dir: &Path) -> Settings {
    Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".to_string(),
            port: 9,
        },
        data_dir: data_dir.to_path_buf(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: true,
        writes_enabled: true,
        batch_post_enabled: false,
    }
}

fn error_code(response: &ToolResponse) -> Option<String> {
    response.value["structuredContent"]["result"]["error"]["code"]
        .as_str()
        .map(str::to_string)
}

fn published_tools() -> Vec<String> {
    catalog::registered_tool_definitions(true, true)
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap().to_string())
        .filter(|name| !name.starts_with("lab_"))
        .collect()
}

fn record_lines(directory: &Path) -> Vec<Value> {
    match std::fs::read_to_string(directory.join(RECORD_FILE)) {
        Ok(text) => text
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect(),
        Err(_) => Vec::new(),
    }
}

#[tokio::test]
async fn every_tool_refuses_until_the_terms_are_accepted_and_sends_nothing_to_tally() {
    for setting in [
        None,
        Some(""),
        Some("false"),
        Some("0"),
        Some("maybe"),
        Some("TRUE"),
        Some("yes"),
        Some(" true"),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let server = Server::for_mcp_with(settings(directory.path()), setting);
        let tools = published_tools();
        assert!(tools.len() >= 20, "{tools:?}");
        for name in &tools {
            // Empty arguments: a refusal must come before any argument is read.
            let response = server.call_tool_response(name, json!({})).await;
            assert_eq!(
                error_code(&response).as_deref(),
                Some("terms_not_accepted"),
                "{name} with setting {setting:?}"
            );
            let message = response.value["structuredContent"]["result"]["error"]["remediation"]
                .as_str()
                .unwrap_or_default();
            assert!(
                message.contains("I accept the ComplyEaze Bridge Terms of Use"),
                "{name}"
            );
            assert!(message.contains("Nothing was read from Tally"), "{name}");
        }
        assert!(
            record_lines(directory.path()).is_empty(),
            "nothing is recorded without acceptance"
        );
    }
}

#[tokio::test]
async fn accepting_lets_calls_through_and_records_the_version_once() {
    for setting in ["true", "1"] {
        let directory = tempfile::tempdir().unwrap();
        let server = Server::for_mcp_with(settings(directory.path()), Some(setting));
        let response = server.call_tool_response("voucher_schema", json!({})).await;
        assert_eq!(
            error_code(&response),
            None,
            "{setting}: {:?}",
            response.value
        );
        let lines = record_lines(directory.path());
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["terms_version"], TERMS_VERSION);
        assert_eq!(lines[0]["source"], "setting");
        assert!(lines[0]["accepted_at"].as_str().unwrap().ends_with('Z'));
        // A second server on the same folder records nothing new for the same version.
        let again = Server::for_mcp_with(settings(directory.path()), Some(setting));
        let response = again.call_tool_response("voucher_schema", json!({})).await;
        assert_eq!(error_code(&response), None);
        assert_eq!(record_lines(directory.path()).len(), 1);
    }
}

#[tokio::test]
async fn a_new_terms_version_is_recorded_beside_an_older_one() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join(RECORD_FILE),
        "{\"terms_version\":\"2026-01\",\"accepted_at\":\"2026-01-01T00:00:00.000Z\",\"source\":\"setting\"}\n",
    )
    .unwrap();
    let server = Server::for_mcp_with(settings(directory.path()), Some("true"));
    assert_eq!(
        error_code(&server.call_tool_response("voucher_schema", json!({})).await),
        None
    );
    let versions: Vec<_> = record_lines(directory.path())
        .iter()
        .map(|line| line["terms_version"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(versions, ["2026-01", TERMS_VERSION]);
}

#[tokio::test]
async fn an_acceptance_that_cannot_be_recorded_refuses_every_tool() {
    let directory = tempfile::tempdir().unwrap();
    // A directory where the record file should be: the append fails.
    std::fs::create_dir(directory.path().join(RECORD_FILE)).unwrap();
    let server = Server::for_mcp_with(settings(directory.path()), Some("true"));
    for name in published_tools() {
        let response = server.call_tool_response(&name, json!({})).await;
        assert_eq!(
            error_code(&response).as_deref(),
            Some("terms_record_unavailable"),
            "{name}"
        );
    }
}

#[tokio::test]
async fn initialize_and_the_tool_list_answer_before_the_terms_are_accepted() {
    let directory = tempfile::tempdir().unwrap();
    let server = Server::for_mcp_with(settings(directory.path()), None);
    let input = [
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"terms","version":"1"}}}),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"tally_status","arguments":{}}}),
    ]
    .iter()
    .map(|value| format!("{value}\n"))
    .collect::<String>();
    let mut output = Vec::new();
    serve_stdio(server, BufReader::new(input.as_bytes()), &mut output)
        .await
        .expect("stdio session");
    let responses: Vec<Value> = String::from_utf8(output)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(responses.len(), 3, "{responses:?}");
    assert!(responses[0]["result"]["protocolVersion"].is_string());
    assert!(!responses[1]["result"]["tools"]
        .as_array()
        .unwrap()
        .is_empty());
    let refused = &responses[2]["result"]["structuredContent"]["result"]["error"];
    assert_eq!(refused["code"], "terms_not_accepted");
}

#[test]
fn the_gate_states_map_to_their_refusals() {
    assert_eq!(TermsGate::NotRequired.refusal(), None);
    assert_eq!(TermsGate::Required(TermsState::Accepted).refusal(), None);
    assert_eq!(
        TermsGate::Required(TermsState::NotAccepted).refusal(),
        Some("terms_not_accepted")
    );
    assert_eq!(
        TermsGate::Required(TermsState::RecordUnavailable).refusal(),
        Some("terms_record_unavailable")
    );
    let directory = tempfile::tempdir().unwrap();
    for setting in [None, Some("false"), Some(""), Some("maybe"), Some("True")] {
        assert_eq!(
            TermsGate::for_mcp(setting, directory.path()),
            TermsGate::Required(TermsState::NotAccepted),
            "{setting:?}"
        );
    }
}

// Server::new stays open (tests, the desktop app's local views); the production path is the only
// place an MCP client is answered, and it must build the closed server. These read the source, so
// a caller that skipped the gate fails here rather than in a release.
fn source_files() -> Vec<(String, String)> {
    fn walk(directory: &Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push((
                    path.file_name().unwrap().to_string_lossy().to_string(),
                    std::fs::read_to_string(&path).unwrap(),
                ));
            }
        }
    }
    let mut out = Vec::new();
    walk(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut out);
    out
}

#[test]
fn the_only_production_entry_that_answers_an_mcp_client_builds_the_closed_server() {
    let files = source_files();
    let agent = &files.iter().find(|(name, _)| name == "agent.rs").unwrap().1;
    let start = agent.find("pub async fn run_stdio()").unwrap();
    let body = &agent[start..agent[start..].find("\n}\n").unwrap() + start];
    assert!(
        body.contains("Server::for_mcp(Settings::from_env()?)"),
        "{body}"
    );
    assert!(!body.contains("Server::new("), "{body}");
}

#[test]
fn nothing_else_dispatches_an_mcp_tool_call() {
    // The desktop app calls Bridge's operations directly and never dispatches a `tools/call`, so
    // it does not need the gate. Only the serving loop (agent_protocol.rs) and agent.rs's own
    // wrappers call the dispatcher; a new caller must be added here on purpose.
    let mut callers = Vec::new();
    for (name, text) in source_files() {
        if name.ends_with("_tests.rs") || name == "agent_terms_tests.rs" {
            continue;
        }
        if text.contains(".call_tool_response(") || text.contains(".tool_payload(") {
            callers.push(name);
        }
    }
    callers.sort();
    assert_eq!(callers, ["agent.rs", "agent_protocol.rs"]);
}

#[test]
fn the_manifest_asks_for_exactly_the_terms_this_build_enforces() {
    let manifest: Value =
        serde_json::from_str(include_str!("../../packaging/mcpb/manifest.json")).unwrap();
    let key = format!("accept_terms_{}", TERMS_VERSION.replace('-', "_"));
    assert_eq!(manifest["manifest_version"], "0.2");
    assert_eq!(
        manifest["privacy_policies"],
        json!(["https://bridge.complyeaze.com/privacy"])
    );
    let setting = &manifest["user_config"][&key];
    assert_eq!(setting["type"], "boolean", "{key}");
    assert_eq!(setting["required"], true);
    assert_eq!(setting["default"], false);
    assert!(setting["title"]
        .as_str()
        .unwrap()
        .starts_with("I accept the ComplyEaze Bridge Terms of Use"));
    let description = setting["description"].as_str().unwrap();
    assert!(description.contains("https://bridge.complyeaze.com/terms"));
    assert!(description.contains("https://bridge.complyeaze.com/privacy"));
    assert_eq!(
        manifest["server"]["mcp_config"]["env"][TERMS_ENV],
        format!("${{user_config.{key}}}")
    );
    // Posting keeps its own separate setting, and turning it on confirms the posting section.
    assert_eq!(manifest["user_config"]["enable_writes"]["default"], false);
    assert!(manifest["user_config"]["enable_writes"]["description"]
        .as_str()
        .unwrap()
        .contains("section 9 of the Terms of Use"));
    // No other setting is required except the accept-terms one.
    for (name, option) in manifest["user_config"].as_object().unwrap() {
        if name != &key {
            assert_ne!(option["required"], true, "{name}");
        }
    }
}
