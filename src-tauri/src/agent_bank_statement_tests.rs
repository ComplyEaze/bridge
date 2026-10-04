//! The `parse_bank_statement` tool surface: gating, admission, what leaves the
//! machine, and where the password may and may not appear.
//!
//! Tests that open a PDF need the PDFium library and are ignored by default;
//! run them with `BRIDGE_PDFIUM_LIBRARY=/abs/path/libpdfium.dylib cargo test
//! --lib bank_statement -- --ignored`.
use super::*;

const PASSWORD: &str = "synthetic-user-4321";

fn server(directory: &Path, import_enabled: bool, redaction: Redaction) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".to_string(),
            port: 9,
        },
        data_dir: directory.join("agent"),
        max_rows: 500,
        max_bytes: 200_000,
        redaction,
        import_enabled,
        writes_enabled: false,
        batch_post_enabled: false,
    })
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("crates/bridge-bank-statement/tests/fixtures")
        .join(name)
}

/// A private copy of a fixture and a password file, as an operator would
/// have them.
fn statement_files(directory: &Path, pdf: &str, password: &str) -> (PathBuf, PathBuf) {
    let statement = directory.join("statement.pdf");
    fs::copy(fixture(pdf), &statement).unwrap();
    let password_file = directory.join("statement.password");
    fs::write(&password_file, format!("{password}\n")).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&password_file, fs::Permissions::from_mode(0o600)).unwrap();
    }
    (statement, password_file)
}

fn arguments(statement: &Path, password_file: &Path) -> Value {
    json!({
        "statement_path": statement.to_str().unwrap(),
        "password_file": password_file.to_str().unwrap(),
        "bank": "hdfc",
        "account_label": "Synthetic CA xx4321",
        "opening_balance": "1,000.00",
        "closing_balance": "1,02,200.00",
        "total_debits": "8,800.00",
        "total_credits": "1,10,000.00",
        "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense",
        "mapping": [
            {"party": "NORTHWIND TRADERS", "ledger": "Northwind Traders"},
            {"party": "SILVER OAK MUTUAL", "ledger": "Silver Oak Mutual Fund", "treatment": "auto"}
        ]
    })
}

/// An absolute path on this platform (a Unix-style "/x" is not absolute on
/// Windows) for a file the test never opens.
fn never_opened(name: &str) -> String {
    std::env::temp_dir()
        .join(format!("bridge-never-opened-{name}"))
        .to_string_lossy()
        .into_owned()
}

/// Where a parse result's proposals file is, from its proposals_id, as
/// build_import_xml resolves it.
fn proposals_file(directory: &Path, result: &Value) -> PathBuf {
    directory
        .join("agent")
        .join(PROPOSALS_DIRECTORY)
        .join(format!("{}.json", result["proposals_id"].as_str().unwrap()))
}

fn error_code(response: &Value) -> Option<&str> {
    response["structuredContent"]["result"]["error"]["code"].as_str()
}

fn every_file_under(directory: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![directory.to_path_buf()];
    while let Some(next) = pending.pop() {
        for entry in fs::read_dir(next).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                found.push(path);
            }
        }
    }
    found
}

#[tokio::test]
async fn the_tool_is_hidden_and_refused_without_the_import_opt_in() {
    for (enabled, listed) in [(false, false), (true, true)] {
        assert_eq!(
            tool_definitions(enabled, false)
                .as_array()
                .unwrap()
                .iter()
                .any(|tool| tool["name"] == "parse_bank_statement"),
            listed
        );
    }
    let directory = tempfile::tempdir().unwrap();
    let (statement, password_file) =
        statement_files(directory.path(), "hdfc-synthetic.pdf", PASSWORD);
    let response = server(directory.path(), false, Redaction::None)
        .call_tool(
            "parse_bank_statement",
            arguments(&statement, &password_file),
        )
        .await;
    assert_eq!(
        error_code(&response),
        Some("import_unverified_on_live_tally")
    );
}

#[test]
fn the_published_schema_uses_only_patterns_the_validator_implements() {
    fn patterns(value: &Value, found: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                if let Some(Value::String(pattern)) = map.get("pattern") {
                    found.push(pattern.clone());
                }
                map.values().for_each(|child| patterns(child, found));
            }
            Value::Array(values) => values.iter().for_each(|child| patterns(child, found)),
            _ => {}
        }
    }
    let mut found = Vec::new();
    patterns(&input_schema(), &mut found);
    found.sort();
    found.dedup();
    assert_eq!(found, [r"\S", "^[0-9]{4}-?[0-9]{2}-?[0-9]{2}$"]);
}

#[tokio::test]
async fn inputs_are_refused_before_the_statement_is_opened() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), true, Redaction::None);
    // no statement file exists at all: every refusal below must come first
    let missing = directory.path().join("absent.pdf");
    let base = arguments(&missing, &directory.path().join("absent.password"));
    let cases: [(&str, Value, &str); 8] = [
        (
            "statement_path",
            json!("relative/statement.pdf"),
            "argument_invalid:statement_path",
        ),
        (
            "password_file",
            json!("statement.password"),
            "argument_invalid:password_file",
        ),
        ("bank", json!("icici"), "argument_invalid:bank"),
        (
            "total_debits",
            json!("-8,800.00"),
            "statement_malformed_control_value",
        ),
        (
            "opening_balance",
            json!("1.005"),
            "statement_malformed_control_value",
        ),
        ("from", json!("2026-02-30"), "invalid_date"),
        (
            "mapping",
            json!([{"party": "UNRESOLVED", "ledger": "Anything"}]),
            "statement_mapping_claims_a_sentinel",
        ),
        (
            "mapping",
            json!([{"party": "A", "ledger": "L", "treatment": "transfer"}]),
            "argument_invalid:mapping",
        ),
    ];
    for (key, value, expected) in cases {
        let mut args = base.clone();
        args[key] = value;
        let response = server.call_tool("parse_bank_statement", args).await;
        assert_eq!(error_code(&response), Some(expected), "{key}");
    }
    let mut one_total = base.clone();
    one_total.as_object_mut().unwrap().remove("total_credits");
    assert_eq!(
        error_code(&server.call_tool("parse_bank_statement", one_total).await),
        Some("statement_control_totals_incomplete")
    );
    // HDFC prints its totals, so they cannot be left out; Union Bank prints none
    let mut no_totals = base.clone();
    for key in ["total_debits", "total_credits"] {
        no_totals.as_object_mut().unwrap().remove(key);
    }
    assert_eq!(
        error_code(
            &server
                .call_tool("parse_bank_statement", no_totals.clone())
                .await
        ),
        Some("statement_control_totals_required")
    );
    no_totals["bank"] = json!("ubi");
    assert_eq!(
        error_code(&server.call_tool("parse_bank_statement", no_totals).await),
        Some("statement_file_unreadable")
    );
    let mut reversed = base.clone();
    reversed["from"] = json!("2026-08-31");
    reversed["to"] = json!("2026-08-01");
    assert_eq!(
        error_code(&server.call_tool("parse_bank_statement", reversed).await),
        Some("statement_reversed_date_window")
    );
    let mut collision = base.clone();
    collision["mapping"] = json!([
        {"party": "ZEPHYR MANUFACTURING", "ledger": "One"},
        {"party": "ZEPHYRMANUFACTURING", "ledger": "Two"}
    ]);
    assert_eq!(
        error_code(&server.call_tool("parse_bank_statement", collision).await),
        Some("statement_mapping_key_collision")
    );
    assert_eq!(
        error_code(&server.call_tool("parse_bank_statement", base).await),
        Some("statement_file_unreadable")
    );
}

#[cfg(unix)]
#[tokio::test]
async fn a_password_file_others_can_read_is_refused() {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    let (statement, password_file) =
        statement_files(directory.path(), "hdfc-synthetic.pdf", PASSWORD);
    fs::set_permissions(&password_file, fs::Permissions::from_mode(0o640)).unwrap();
    let response = server(directory.path(), true, Redaction::None)
        .call_tool(
            "parse_bank_statement",
            arguments(&statement, &password_file),
        )
        .await;
    assert_eq!(
        error_code(&response),
        Some("statement_password_file_permissions")
    );
    assert!(!response.to_string().contains(PASSWORD));
}

#[tokio::test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
async fn only_the_summary_leaves_and_the_password_appears_nowhere() {
    assert!(env::var_os("BRIDGE_PDFIUM_LIBRARY").is_some());
    let directory = tempfile::tempdir().unwrap();
    let (statement, password_file) =
        statement_files(directory.path(), "hdfc-synthetic.pdf", PASSWORD);
    let server = server(directory.path(), true, Redaction::None);
    let response = server
        .call_tool(
            "parse_bank_statement",
            arguments(&statement, &password_file),
        )
        .await;
    let result = &response["structuredContent"]["result"];
    assert!(error_code(&response).is_none(), "{response}");
    assert_eq!(result["statement_rows"], 6);
    assert_eq!(result["vouchers"], 6);
    assert_eq!(result["suspense_rows"], 4);
    assert_eq!(result["account_last4"], "4321");
    assert_eq!(result["reconciled"]["running_balance_every_row"], true);
    assert_eq!(result["reconciled"]["totals_match_statement"], true);
    let northwind = result["counterparties"]
        .as_array()
        .unwrap()
        .iter()
        .find(|group| group["party"] == "NORTHWIND TRADERS")
        .expect("northwind group");
    assert_eq!(northwind["ledger"], "Northwind Traders");
    assert_eq!(northwind["disposition"], "Receipt");
    // Its rows, and no amount.
    assert_eq!(northwind["rows"], 1);
    assert!(northwind.get("total").is_none(), "{northwind}");

    // row-level content stays in the file: no reference, narration, label or
    // transaction date reaches the response
    let text = response.to_string();
    for private in [
        PASSWORD,
        "612345678901",
        "45678901",
        "PAYMENT",
        "narration",
        "bridge_txn_id",
        "st-2026",
        "2026-08-01",
        "00000000004321",
    ] {
        assert!(
            !text.contains(private),
            "{private} left the machine: {text}"
        );
    }

    // The result names the file by its id; its path is not returned.
    assert!(result.get("path").is_none(), "{result}");
    let path = proposals_file(directory.path(), result);
    let bytes = fs::read(&path).unwrap();
    assert_eq!(sha256_hex(&bytes), result["sha256"]);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
    let document: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(document["schema"], PROPOSALS_SCHEMA);
    let vouchers = document["vouchers"].as_array().unwrap();
    assert_eq!(vouchers.len(), 6);
    // exactly build_import_xml's voucher fields; no identity of our own
    for voucher in vouchers {
        let mut keys: Vec<&str> = voucher
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "bridge_txn_id",
                "date",
                "entries",
                "narration",
                "voucher_type"
            ]
        );
        assert_eq!(voucher["entries"].as_array().unwrap().len(), 2);
    }
    assert!(!String::from_utf8_lossy(&bytes).contains("REMOTEID"));

    // Through the real MCP framing, which is what writes egress receipts: the
    // password reaches neither the wire nor any file this server wrote.
    let protocol_directory = tempfile::tempdir().unwrap();
    let protocol_server = self::server(protocol_directory.path(), true, Redaction::None);
    let requests = [
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"parse_bank_statement","arguments":arguments(&statement, &password_file)}}),
    ];
    let input: String = requests
        .iter()
        .map(|request| format!("{request}\n"))
        .collect();
    let mut output = Vec::new();
    agent_protocol::serve_stdio(
        protocol_server,
        tokio::io::BufReader::new(input.as_bytes()),
        &mut output,
    )
    .await
    .unwrap();
    let wire = String::from_utf8(output).unwrap();
    assert!(wire.contains("\"proposals_id\""), "{wire}");
    assert!(!wire.contains(PASSWORD));
    let written = every_file_under(&protocol_directory.path().join("agent"));
    assert!(
        written
            .iter()
            .any(|file| file.ends_with("agent-egress.jsonl")),
        "no egress receipt was written: {written:?}"
    );
    for file in written {
        let content = fs::read(&file).unwrap();
        assert!(
            !String::from_utf8_lossy(&content).contains(PASSWORD),
            "password written to {}",
            file.display()
        );
    }

    // a corrected mapping keeps every transaction label
    let mut remapped = arguments(&statement, &password_file);
    remapped["mapping"] = json!([]);
    let second = server.call_tool("parse_bank_statement", remapped).await;
    let second_path = proposals_file(directory.path(), &second["structuredContent"]["result"]);
    let second_document: Value = serde_json::from_slice(&fs::read(second_path).unwrap()).unwrap();
    let labels = |document: &Value| {
        document["vouchers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|voucher| voucher["bridge_txn_id"].clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(labels(&document), labels(&second_document));
    assert_eq!(second["structuredContent"]["result"]["suspense_rows"], 6);
}

#[tokio::test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
async fn mask_parties_masks_every_name_in_the_summary() {
    assert!(env::var_os("BRIDGE_PDFIUM_LIBRARY").is_some());
    let directory = tempfile::tempdir().unwrap();
    let (statement, password_file) =
        statement_files(directory.path(), "hdfc-synthetic.pdf", PASSWORD);
    let response = server(directory.path(), true, Redaction::MaskParties)
        .call_tool(
            "parse_bank_statement",
            arguments(&statement, &password_file),
        )
        .await;
    assert!(error_code(&response).is_none(), "{response}");
    let text = response.to_string();
    for name in [
        "NORTHWIND TRADERS",
        "Northwind Traders",
        "SILVER OAK MUTUAL",
        "BLUE RIVER CO",
        "GREEN-FIELD",
        "Synthetic Bank Ledger",
        "Suspense",
    ] {
        assert!(!text.contains(name), "{name} was not masked: {text}");
    }
    // The evidence describes the result as it is returned.
    let content = &response["structuredContent"];
    assert_eq!(
        content["evidence"]["response_sha256"],
        sha256_json(&content["result"])
    );
    assert_eq!(
        content["evidence"]["bytes"],
        serde_json::to_vec(&content["result"]).unwrap().len()
    );
}

/// The evidence of a parse result is its hash and size as the caller receives
/// it: with mask_parties, the masked names; with drop_narration, no narration.
#[test]
fn the_evidence_describes_the_result_as_returned() {
    let result =
        json!({"party": party_name("SYNTHETIC PAYER"), "rows": 3, "narration": "SYNTHETIC TEXT"});
    let masked = json!({"party": "SY…ER", "rows": 3, "narration": "SYNTHETIC TEXT"});
    let plain = json!({"party": "SYNTHETIC PAYER", "rows": 3, "narration": "SYNTHETIC TEXT"});
    let dropped = json!({"party": "SYNTHETIC PAYER", "rows": 3});
    let size = |value: &Value| serde_json::to_vec(value).unwrap().len();
    assert_eq!(
        returned_evidence(&result, Redaction::MaskParties),
        (sha256_json(&masked), size(&masked))
    );
    assert_eq!(
        returned_evidence(&result, Redaction::None),
        (sha256_json(&plain), size(&plain))
    );
    assert_eq!(
        returned_evidence(&result, Redaction::DropNarration),
        (sha256_json(&dropped), size(&dropped))
    );
}

#[tokio::test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
async fn refusals_name_a_category_and_a_row_never_the_statement() {
    assert!(env::var_os("BRIDGE_PDFIUM_LIBRARY").is_some());
    let directory = tempfile::tempdir().unwrap();
    let (statement, password_file) =
        statement_files(directory.path(), "hdfc-synthetic.pdf", "not-the-password");
    let server = server(directory.path(), true, Redaction::None);
    let wrong = server
        .call_tool(
            "parse_bank_statement",
            arguments(&statement, &password_file),
        )
        .await;
    assert_eq!(error_code(&wrong), Some("statement_unreadable_pdf"));
    assert!(!wrong.to_string().contains("not-the-password"));

    fs::write(&password_file, PASSWORD).unwrap();
    for (key, value, expected) in [
        (
            "account_label",
            "xx9876",
            "statement_account_not_in_statement",
        ),
        (
            "closing_balance",
            "1,02,201.00",
            "statement_extent_unproven",
        ),
        (
            "total_debits",
            "8,801.00",
            "statement_control_total_mismatch",
        ),
        (
            "opening_balance",
            "1,001.00",
            "statement_balance_chain_broken:row_1",
        ),
    ] {
        let mut args = arguments(&statement, &password_file);
        args[key] = json!(value);
        let response = server.call_tool("parse_bank_statement", args).await;
        assert_eq!(error_code(&response), Some(expected), "{key}");
    }
    // nothing was published for a refused run
    assert!(
        !directory.path().join("agent/bank-statements").exists()
            || every_file_under(&directory.path().join("agent/bank-statements")).is_empty()
    );
}

#[tokio::test]
#[ignore = "needs PDFium: set BRIDGE_PDFIUM_LIBRARY and run with --ignored"]
async fn a_union_bank_statement_parses_without_printed_totals() {
    assert!(env::var_os("BRIDGE_PDFIUM_LIBRARY").is_some());
    let directory = tempfile::tempdir().unwrap();
    let (statement, password_file) =
        statement_files(directory.path(), "ubi-synthetic.pdf", "synthetic-user-7788");
    let server = server(directory.path(), true, Redaction::None);
    let args = json!({
        "statement_path": statement.to_str().unwrap(),
        "password_file": password_file.to_str().unwrap(),
        "bank": "ubi",
        "account_label": "UBI SB xx7788",
        "opening_balance": "10,000.00",
        "closing_balance": "500.00",
        "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense",
        "mapping": [{"party": "CASH DEPOSIT", "ledger": "Cash", "treatment": "contra"}]
    });
    // A cash party is answered per line, never mapped.
    assert_eq!(
        error_code(&server.call_tool("parse_bank_statement", args.clone()).await),
        Some("statement_cash_party_in_mapping")
    );
    let mut args = args;
    args.as_object_mut().unwrap().remove("mapping");
    let open = server.call_tool("parse_bank_statement", args.clone()).await;
    let result = &open["structuredContent"]["result"];
    assert_eq!(result["vouchers"], 5, "{open}");
    let questions = result["cash_questions"].as_array().unwrap();
    assert_eq!(questions.len(), 1, "{open}");
    assert_eq!(questions[0]["movement"], "deposit");
    assert_eq!(questions[0]["amount"], "750.50");
    assert_eq!(questions[0]["answers"].as_array().unwrap().len(), 5);
    args["cash_answers"] =
        json!([{"bridge_txn_id": questions[0]["bridge_txn_id"], "answer": "dont_know"}]);
    let response = server.call_tool("parse_bank_statement", args.clone()).await;
    let result = &response["structuredContent"]["result"];
    assert_eq!(result["statement_rows"], 6, "{response}");
    assert_eq!(result["vouchers"], 6);
    assert_eq!(result["cash_questions"], json!([]));
    assert_eq!(
        result["suspense_by_reason"]["cash_purpose_not_confirmed"], 1,
        "{response}"
    );
    // Union Bank prints no totals and none were supplied, so none is echoed.
    assert_eq!(result["reconciled"]["total_debits"], Value::Null);
    assert_eq!(result["reconciled"]["total_credits"], Value::Null);
    assert_eq!(result["reconciled"]["closing_balance"], "500.00");
    assert_eq!(result["reconciled"]["totals_match_statement"], false);

    // a closing balance the rows do not reach is still refused
    let mut short = args;
    short["closing_balance"] = json!("0.00");
    assert_eq!(
        error_code(&server.call_tool("parse_bank_statement", short).await),
        Some("statement_extent_unproven")
    );
}

/// What `build` writes is what build_import_xml reads: a cash line left open
/// by a real build, persisted by this tool, is refused at resolve, and the
/// same statement answered resolves. No PDF is read; the rows are SBI shapes.
#[test]
fn an_open_cash_line_written_by_the_parse_is_refused_where_the_build_reads_it() {
    use bridge_bank_statement::parse::Row;
    use bridge_bank_statement::proposals::{build, group_counterparties, selfcheck, BuildOptions};
    let directory = tempfile::tempdir().unwrap();
    let sbi = |date: &str, narration: &str, dr: &str, cr: &str, bal: &str| {
        Row::from_pairs([
            ("date", date),
            ("narr", narration),
            ("narr_spaced", narration),
            ("ref", ""),
            ("ref_spaced", ""),
            ("dr", dr),
            ("cr", cr),
            ("bal", bal),
        ])
    };
    let rows = [
        sbi(
            "01Aug2026",
            "ATM WDL ATM CASH 1234 SYNTHETIC BRANCH",
            "500.00",
            "",
            "9500.00",
        ),
        sbi(
            "02Aug2026",
            "BY TRANSFER-NEFT*SYNTHETIC SUPPLIER",
            "",
            "100.00",
            "9600.00",
        ),
    ];
    let parsed_with = |answers: &CashAnswers| {
        let build = build(
            &rows,
            Bank::Sbi,
            &Mapping::default(),
            &BuildOptions {
                bank_ledger: "Synthetic Bank Ledger",
                suspense_ledger: "Suspense",
                account_label: "Synthetic SB xx1234",
                account_number: "00000000001234",
                date_from: None,
                date_to: None,
                cash_answers: answers,
            },
        )
        .unwrap();
        ParsedStatement {
            account_number: "00000000001234".into(),
            statement_rows: rows.len(),
            closing: bridge_tally_core::ExactDecimal::parse("9600.00").unwrap(),
            totals: bridge_bank_statement::money::statement_totals(&rows).unwrap(),
            check: selfcheck(&build, "Synthetic Bank Ledger").unwrap(),
            counterparties: group_counterparties(&build.records).unwrap(),
            build,
        }
    };
    let mut args = json!({
        "statement_path": never_opened("statement.pdf"),
        "password_file": never_opened("statement.password"),
        "bank": "sbi",
        "account_label": "Synthetic SB xx1234",
        "opening_balance": "10,000.00",
        "closing_balance": "9,600.00",
        "total_debits": "500.00",
        "total_credits": "100.00",
        "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense"
    });
    let request = OwnedRequest::from_args(&args).unwrap();
    let open = parsed_with(&request.cash_answers);
    let summary_open = summary(&request, &open, "statement-x", "0", 200_000);
    assert_eq!(summary_open["cash_questions"].as_array().unwrap().len(), 1);
    let id = summary_open["cash_questions"][0]["bridge_txn_id"]
        .as_str()
        .unwrap()
        .to_string();
    let (proposals_id, digest) =
        persist(directory.path(), &request, &open, &"0".repeat(64)).unwrap();
    let build_args = |proposals_id: &str, digest: &str| json!({"company_guid": "00000000-0000-4000-8000-000000000002", "proposals_id": proposals_id, "proposals_sha256": digest});
    assert_eq!(
        resolve_import_arguments(directory.path(), &build_args(&proposals_id, &digest)).err(),
        Some("cash_questions_open".to_string())
    );

    args["cash_answers"] =
        json!([{"bridge_txn_id": id, "answer": "owner_use", "ledger": "Drawings"}]);
    let request = OwnedRequest::from_args(&args).unwrap();
    let answered = parsed_with(&request.cash_answers);
    let (proposals_id, digest) =
        persist(directory.path(), &request, &answered, &"0".repeat(64)).unwrap();
    let resolved =
        resolve_import_arguments(directory.path(), &build_args(&proposals_id, &digest)).unwrap();
    // Drawings is carried to build for the Suspense-group check, not as cash
    // in hand.
    let named = resolved
        .cash_ledgers
        .iter()
        .map(|need| {
            (
                need.bridge_txn_id.as_str(),
                need.ledger.as_str(),
                need.cash_in_hand,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(named, [(id.as_str(), "Drawings", false)]);
    let vouchers = resolved.args["vouchers"].as_array().unwrap();
    assert_eq!(vouchers.len(), 2);
    let drawn = vouchers
        .iter()
        .find(|voucher| voucher["bridge_txn_id"] == id.as_str())
        .unwrap();
    assert_eq!(drawn["voucher_type"], "Payment");
    assert_eq!(drawn["entries"][0]["ledger"], "Drawings");
    assert_eq!(drawn["entries"][0]["side"], "Dr");
    let summary = summary(&request, &answered, "statement-x", "0", 200_000);
    assert_eq!(summary["cash_questions"], json!([]));
    // Only the unidentified transfer went to suspense; the answered cash line
    // did not.
    assert_eq!(
        summary["suspense_by_reason"],
        json!({"cash_purpose_not_confirmed": 0, "party_unmapped_or_mapped_to_suspense": 1}),
        "{summary}"
    );

    // Business cash names a ledger build must find under Cash-in-Hand.
    args["cash_answers"] =
        json!([{"bridge_txn_id": id, "answer": "business_cash", "ledger": "Cash"}]);
    let request = OwnedRequest::from_args(&args).unwrap();
    let kept = parsed_with(&request.cash_answers);
    let (proposals_id, digest) =
        persist(directory.path(), &request, &kept, &"0".repeat(64)).unwrap();
    let resolved =
        resolve_import_arguments(directory.path(), &build_args(&proposals_id, &digest)).unwrap();
    let named = resolved
        .cash_ledgers
        .iter()
        .map(|need| {
            (
                need.bridge_txn_id.as_str(),
                need.ledger.as_str(),
                need.cash_in_hand,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(named, [(id.as_str(), "Cash", true)]);
}

/// A statement with hundreds of open cash lines and suspense lines keeps a
/// result small enough to return: each list is bounded, says how many it left
/// out, and the counts still cover every line.
#[test]
fn every_list_in_the_summary_is_bounded_and_counts_what_it_left_out() {
    use bridge_bank_statement::parse::Row;
    use bridge_bank_statement::proposals::{build, group_counterparties, selfcheck, BuildOptions};
    let row = |narration: String, dr: &str, cr: &str, balance: String| {
        Row::from_pairs([
            ("date", "01Aug2026"),
            ("narr", narration.as_str()),
            ("narr_spaced", narration.as_str()),
            ("ref", ""),
            ("ref_spaced", ""),
            ("dr", dr),
            ("cr", cr),
            ("bal", balance.as_str()),
        ])
    };
    let rows = (0..300)
        .flat_map(|index| {
            [
                row(
                    format!("ATM WDL ATM CASH {index} SYNTHETIC BRANCH"),
                    "1.00",
                    "",
                    format!("{}.00", 100_000 - index),
                ),
                // A distinct payer per row, as SBI's UPI rule names them, so
                // the counterparty list grows with the statement too.
                row(
                    format!("BY TRANSFER-UPI/CR/{index:012}/SYNTHETIC PAYER {index}/XYZ"),
                    "",
                    "1.00",
                    format!("{}.50", 100_000 - index),
                ),
            ]
        })
        .collect::<Vec<_>>();
    let build = build(
        &rows,
        Bank::Sbi,
        &Mapping::default(),
        &BuildOptions {
            bank_ledger: "Synthetic Bank Ledger",
            suspense_ledger: "Suspense",
            account_label: "Synthetic SB xx1234",
            account_number: "00000000001234",
            date_from: None,
            date_to: None,
            cash_answers: CashAnswers::none(),
        },
    )
    .unwrap();
    let parsed = ParsedStatement {
        account_number: "00000000001234".into(),
        statement_rows: rows.len(),
        closing: bridge_tally_core::ExactDecimal::parse("0.00").unwrap(),
        totals: bridge_bank_statement::money::statement_totals(&rows).unwrap(),
        check: selfcheck(&build, "Synthetic Bank Ledger").unwrap(),
        counterparties: group_counterparties(&build.records).unwrap(),
        build,
    };
    let request = OwnedRequest::from_args(&json!({
        "statement_path": never_opened("statement.pdf"),
        "password_file": never_opened("statement.password"),
        "bank": "sbi",
        "account_label": "Synthetic SB xx1234",
        "opening_balance": "0.00",
        "closing_balance": "0.00",
        "total_debits": "300.00",
        "total_credits": "300.00",
        "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense"
    }))
    .unwrap();
    let max_bytes = 40_000;
    let summary = summary(&request, &parsed, "statement-x", "0", max_bytes);
    let listed = |key: &str| summary[key].as_array().unwrap().len();
    let omitted = |key: &str| usize::try_from(summary[key].as_u64().unwrap()).unwrap();
    assert_eq!(summary["cash_questions_open"], 300);
    assert!(listed("cash_questions") > 0 && listed("cash_questions") < 300);
    assert_eq!(
        listed("cash_questions") + omitted("cash_questions_omitted"),
        300
    );
    assert_eq!(summary["suspense_rows"], 300);
    assert_eq!(
        summary["suspense_by_reason"]["party_unmapped_or_mapped_to_suspense"],
        300
    );
    // 300 payers and the one cash party, unmapped payers listed first.
    assert!(omitted("counterparties_omitted") > 0);
    assert_eq!(
        listed("counterparties") + omitted("counterparties_omitted"),
        301
    );
    // The 300 suspense payers fill the budget, so every listed row is one.
    assert!(summary["counterparties"]
        .as_array()
        .unwrap()
        .iter()
        .all(|group| group["suspense"] == true));
    // The MCP frame carries the result twice, so it must fit in half.
    assert!(serde_json::to_vec(&summary).unwrap().len() < max_bytes / 2);
}

/// A customer's deposit is a Receipt crediting the customer: build is asked
/// to check that ledger (not as cash in hand), and a file whose voucher puts
/// it on the other side is refused.
#[test]
fn a_deposit_answer_binds_the_credit_leg_of_its_receipt() {
    let directory = tempfile::tempdir().unwrap();
    let publish = |side: &str| {
        let proposals_id = format!("statement-{}", uuid::Uuid::new_v4());
        let (bank_side, customer_side) = if side == "Cr" {
            ("Dr", "Cr")
        } else {
            ("Cr", "Dr")
        };
        let document = json!({
            "schema": PROPOSALS_SCHEMA,
            "proposals_id": proposals_id,
            "vouchers": [{"bridge_txn_id":"st-1","date":"2026-08-03","voucher_type":"Receipt",
                "narration":"CASH from Synthetic Customer | UBI SB xx7788 | 03-Aug-2026",
                "entries":[{"ledger":"Union Bank","amount":"750.50","side":bank_side},
                           {"ledger":"Synthetic Customer","amount":"750.50","side":customer_side}]}],
            "records": [{"row":1,"date":"2026-08-03","disposition":{"voucher":"Receipt"},"amount":"750.50",
                "party":"CASH DEPOSIT","ledger":"Synthetic Customer","suspense":false,"bridge_txn_id":"st-1",
                "cash_movement":"deposit","cash_answer":"customer_paid_in"}],
        });
        let bytes = serde_json::to_vec_pretty(&document).unwrap();
        let statements = directory.path().join(PROPOSALS_DIRECTORY);
        fs::create_dir_all(&statements).unwrap();
        fs::write(statements.join(format!("{proposals_id}.json")), &bytes).unwrap();
        json!({"company_guid": "00000000-0000-4000-8000-000000000002", "proposals_id": proposals_id, "proposals_sha256": sha256_hex(&bytes)})
    };
    let resolved = resolve_import_arguments(directory.path(), &publish("Cr")).unwrap();
    let named = resolved
        .cash_ledgers
        .iter()
        .map(|need| {
            (
                need.bridge_txn_id.as_str(),
                need.ledger.as_str(),
                need.cash_in_hand,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(named, [("st-1", "Synthetic Customer", false)]);
    assert_eq!(
        resolve_import_arguments(directory.path(), &publish("Dr")).err(),
        Some("proposals_file_invalid".to_string())
    );
}

/// A mapping to hundreds of distinct ledgers keeps ledgers_to_validate
/// bounded too, and counts the ledgers it left out.
#[test]
fn the_ledgers_to_validate_are_bounded_and_counted() {
    use bridge_bank_statement::parse::Row;
    use bridge_bank_statement::proposals::{build, group_counterparties, selfcheck, BuildOptions};
    let rows = (0..300)
        .map(|index| {
            let narration = format!("BY TRANSFER-UPI/CR/{index:012}/SYNTHETIC PAYER {index}/XYZ");
            let balance = format!("{}.50", 100_000 - index);
            Row::from_pairs([
                ("date", "01Aug2026"),
                ("narr", narration.as_str()),
                ("narr_spaced", narration.as_str()),
                ("ref", ""),
                ("ref_spaced", ""),
                ("dr", ""),
                ("cr", "1.00"),
                ("bal", balance.as_str()),
            ])
        })
        .collect::<Vec<_>>();
    let mapping = Mapping::from_rows((0..300).map(|index| MappingRow {
        origin: format!("mapping[{index}]"),
        party: format!("SYNTHETIC PAYER {index}"),
        ledger: format!("Synthetic Payer Ledger {index:03}"),
        treatment: None,
    }))
    .unwrap();
    let build = build(
        &rows,
        Bank::Sbi,
        &mapping,
        &BuildOptions {
            bank_ledger: "Synthetic Bank Ledger",
            suspense_ledger: "Suspense",
            account_label: "Synthetic SB xx1234",
            account_number: "00000000001234",
            date_from: None,
            date_to: None,
            cash_answers: CashAnswers::none(),
        },
    )
    .unwrap();
    assert!(build.records.iter().all(|record| !record.suspense));
    let parsed = ParsedStatement {
        account_number: "00000000001234".into(),
        statement_rows: rows.len(),
        closing: bridge_tally_core::ExactDecimal::parse("0.00").unwrap(),
        totals: bridge_bank_statement::money::statement_totals(&rows).unwrap(),
        check: selfcheck(&build, "Synthetic Bank Ledger").unwrap(),
        counterparties: group_counterparties(&build.records).unwrap(),
        build,
    };
    let request = OwnedRequest::from_args(&json!({
        "statement_path": never_opened("statement.pdf"),
        "password_file": never_opened("statement.password"),
        "bank": "sbi",
        "account_label": "Synthetic SB xx1234",
        "opening_balance": "0.00",
        "closing_balance": "0.00",
        "total_debits": "0.00",
        "total_credits": "300.00",
        "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense"
    }))
    .unwrap();
    let max_bytes = 40_000;
    let summary = summary(&request, &parsed, "statement-x", "0", max_bytes);
    let listed = summary["ledgers_to_validate"].as_array().unwrap().len();
    let omitted =
        usize::try_from(summary["ledgers_to_validate_omitted"].as_u64().unwrap()).unwrap();
    // The bank ledger and the 300 mapped ledgers.
    assert!(
        listed > 0 && omitted > 0,
        "{listed} listed, {omitted} omitted"
    );
    assert_eq!(listed + omitted, 301);
    assert!(serde_json::to_vec(&summary).unwrap().len() < max_bytes / 2);
    // With no cash line, nothing row-level leaves: the same contract the
    // PDFium test pins (no transaction label, date or row identity).
    let text = summary.to_string();
    for private in ["bridge_txn_id", "st-2026", "2026-08-01", "\"row\""] {
        assert!(
            !text.contains(private),
            "{private} left the machine: {text}"
        );
    }
}

/// The one row-level exception (owner ruling b1): an open, recognised cash
/// line's bridge_txn_id, date and amount, and its party as printed (masked
/// under redaction), leave so a person can answer it. Nothing else of any row
/// does, and an answered cash line takes no exception.
#[test]
fn only_an_open_cash_lines_id_date_amount_and_party_leave() {
    use bridge_bank_statement::parse::Row;
    use bridge_bank_statement::proposals::{build, group_counterparties, selfcheck, BuildOptions};
    let sbi = |date: &str, narration: &str, dr: &str, cr: &str, bal: &str| {
        Row::from_pairs([
            ("date", date),
            ("narr", narration),
            ("narr_spaced", narration),
            ("ref", ""),
            ("ref_spaced", ""),
            ("dr", dr),
            ("cr", cr),
            ("bal", bal),
        ])
    };
    // Two rows in every counterparty group, and every row's amount and
    // balance different from every total the summary reports, so a row value
    // cannot hide behind an aggregate (a group of one row reports that row's
    // amount as its total, under the counterparty contract).
    let rows = [
        (
            "01Aug2026",
            "ATM WDL ATM CASH 4417 SYNTHETIC QUAYSIDE",
            "512.00",
            "",
            "9488.00",
        ),
        (
            "03Aug2026",
            "BY TRANSFER-UPI/CR/612345678901/SYNTHETIC PAYER/XYZ",
            "",
            "71.00",
            "9559.00",
        ),
        (
            "04Aug2026",
            "ATM WDL ATM CASH 6639 SYNTHETIC LOCKSIDE",
            "64.00",
            "",
            "9495.00",
        ),
        (
            "05Aug2026",
            "BY TRANSFER-UPI/CR/698765432109/SYNTHETIC PAYER/XYZ",
            "",
            "29.00",
            "9524.00",
        ),
        (
            "07Aug2026",
            "ATM WDL ATM CASH 5528 SYNTHETIC HARBOURSIDE",
            "288.00",
            "",
            "9236.00",
        ),
        (
            "08Aug2026",
            "ATM WDL ATM CASH 7740 SYNTHETIC WHARFSIDE",
            "36.00",
            "",
            "9200.00",
        ),
    ]
    .map(|(date, narration, dr, cr, bal)| sbi(date, narration, dr, cr, bal));
    let mut args = json!({
        "statement_path": never_opened("statement.pdf"),
        "password_file": never_opened("statement.password"),
        "bank": "sbi",
        "account_label": "Synthetic SB xx1234",
        "opening_balance": "10,000.00",
        "closing_balance": "9,200.00",
        "total_debits": "900.00",
        "total_credits": "100.00",
        "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense"
    });
    let parsed_with = |request: &OwnedRequest| {
        let build = build(
            &rows,
            Bank::Sbi,
            &Mapping::default(),
            &BuildOptions {
                bank_ledger: "Synthetic Bank Ledger",
                suspense_ledger: "Suspense",
                account_label: "Synthetic SB xx1234",
                account_number: "00000000001234",
                date_from: None,
                date_to: None,
                cash_answers: &request.cash_answers,
            },
        )
        .unwrap();
        ParsedStatement {
            account_number: "00000000001234".into(),
            statement_rows: rows.len(),
            closing: bridge_tally_core::ExactDecimal::parse("9200.00").unwrap(),
            totals: bridge_bank_statement::money::statement_totals(&rows).unwrap(),
            check: selfcheck(&build, "Synthetic Bank Ledger").unwrap(),
            counterparties: group_counterparties(&build.records).unwrap(),
            build,
        }
    };
    // Answer the last two cash lines; the first two stay open.
    let unanswered = parsed_with(&OwnedRequest::from_args(&args).unwrap());
    args["cash_answers"] = json!(unanswered.build.records[4..]
        .iter()
        .map(|record| json!({"bridge_txn_id": record.bridge_txn_id, "answer": "dont_know"}))
        .collect::<Vec<_>>());
    let request = OwnedRequest::from_args(&args).unwrap();
    let parsed = parsed_with(&request);
    let records = &parsed.build.records;
    assert_eq!(records.len(), 6);
    let open = [&records[0], &records[2]];
    for record in open {
        assert_eq!(record.disposition, Disposition::NeedsAnswer);
    }
    assert!(records[4..]
        .iter()
        .all(|record| record.cash_answer.is_some()));
    let mut summary = summary(&request, &parsed, "statement-x", "0", 200_000);

    // Every key the summary carries, so a new one cannot slip past the scan.
    let mut top: Vec<&str> = summary
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    top.sort_unstable();
    assert_eq!(
        top,
        [
            "account_last4",
            "bank",
            "cash_questions",
            "cash_questions_omitted",
            "cash_questions_open",
            "counterparties",
            "counterparties_omitted",
            "ledgers_to_validate",
            "ledgers_to_validate_omitted",
            "next_step",
            "proposals_id",
            "reconciled",
            "rows_in_window",
            "sha256",
            "skipped",
            "statement_rows",
            "suspense_by_reason",
            "suspense_rows",
            "vouchers"
        ]
    );
    let summary_before = summary.clone();
    // Each open line's entry carries exactly the exempt fields, with the
    // line's own values, and the fixed question and answers.
    let questions = summary["cash_questions"].as_array_mut().unwrap();
    assert_eq!(questions.len(), 2, "only the open lines are asked");
    for (entry, record) in questions.iter_mut().zip(open) {
        let entry = entry.as_object_mut().unwrap();
        let mut keys: Vec<&str> = entry.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "amount",
                "answers",
                "bridge_txn_id",
                "date",
                "movement",
                "printed_as",
                "question"
            ]
        );
        assert_eq!(entry["bridge_txn_id"], record.bridge_txn_id.as_str());
        assert_eq!(entry["date"], record.date.as_str());
        assert_eq!(entry["amount"], record.amount.as_str());
        assert_eq!(
            entry["printed_as"],
            serde_json::to_value(party_name(record.party.clone())).unwrap()
        );
        for exempt in ["bridge_txn_id", "date", "amount", "printed_as"] {
            entry.remove(exempt);
        }
    }
    // The open lines' ids, dates and amounts were present before the
    // removal, and are scanned for below: the scan fires on a leaked row value.
    let whole = summary_before.to_string();
    for record in open {
        for token in [&record.bridge_txn_id, &record.date, &record.amount] {
            assert!(whole.contains(token.as_str()), "{token}");
        }
    }

    // With those four removed, no value of any row is left: no row's id,
    // date, amount or row number, no printed balance, reference or other
    // statement text, and nothing of the answered cash line.
    let text = summary.to_string();
    let mut private: Vec<String> = records
        .iter()
        .flat_map(|record| {
            [
                record.bridge_txn_id.clone(),
                record.date.clone(),
                record.date.replace('-', ""),
                narration_date(&record.date),
                slashed_date(&record.date),
                record.amount.clone(),
                record.amount.trim_end_matches(".00").to_string(),
            ]
        })
        .collect();
    private.extend(
        [
            "\"row\"",
            "st-2026",
            "Aug2026",
            "9488",
            "9,488",
            "9,559",
            "9,495",
            "9,524",
            "9,236",
            "9559",
            "9495",
            "9524",
            "9236",
            "612345678901",
            "698765432109",
            "4417",
            "6639",
            "5528",
            "7740",
            "QUAYSIDE",
            "LOCKSIDE",
            "HARBOURSIDE",
            "WHARFSIDE",
            "ATM WDL",
            "/XYZ",
            "UPI/CR",
            "narration",
            "Bridge: purpose not confirmed",
        ]
        .map(String::from),
    );
    for token in &private {
        assert!(
            !text.contains(token.as_str()),
            "{token} left the machine: {text}"
        );
    }

    // Under mask_parties the open lines' party name is masked like every
    // other party name.
    let plain = super::super::redact_tool_response(
        "parse_bank_statement",
        json!({ "result": summary_before.clone() }),
        Redaction::None,
    );
    let masked = super::super::redact_tool_response(
        "parse_bank_statement",
        json!({ "result": summary_before }),
        Redaction::MaskParties,
    );
    for name in ["ATM CASH WITHDRAWAL", "SYNTHETIC PAYER"] {
        assert!(plain.to_string().contains(name), "{name}: {plain}");
        assert!(!masked.to_string().contains(name), "{name}: {masked}");
    }
    let printed = |value: &Value| value["result"]["cash_questions"][0]["printed_as"].clone();
    assert_eq!(printed(&plain), "ATM CASH WITHDRAWAL");
    assert!(printed(&masked).is_string(), "{masked}");
    assert_ne!(printed(&masked), printed(&plain));
}

/// 2026-08-01 as a statement narration prints it: 01-Aug-2026.
fn narration_date(iso: &str) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let [year, month, day] = [&iso[0..4], &iso[5..7], &iso[8..10]];
    let month = MONTHS[month.parse::<usize>().unwrap() - 1];
    format!("{day}-{month}-{year}")
}

/// 2026-08-01 as 01/08/2026.
fn slashed_date(iso: &str) -> String {
    format!("{}/{}/{}", &iso[8..10], &iso[5..7], &iso[0..4])
}

/// The parse result carries no amount of its own: a counterparty gives its
/// rows only, and reconciled echoes only what the caller supplied. An open
/// cash line's amount is the one exception (owner ruling b1). Checked by
/// structure: every string in the result that reads as an amount sits at one
/// of those places.
#[test]
fn the_parse_result_carries_no_amount_but_the_callers_own() {
    use bridge_bank_statement::parse::Row;
    use bridge_bank_statement::proposals::{build, group_counterparties, selfcheck, BuildOptions};
    let sbi = |date: &str, narration: &str, dr: &str, cr: &str, bal: &str| {
        Row::from_pairs([
            ("date", date),
            ("narr", narration),
            ("narr_spaced", narration),
            ("ref", ""),
            ("ref_spaced", ""),
            ("dr", dr),
            ("cr", cr),
            ("bal", bal),
        ])
    };
    // An answered cash line and a once-paid party (each a group of one row),
    // and an open cash line.
    let rows = [
        (
            "01Aug2026",
            "ATM WDL ATM CASH 4417 SYNTHETIC QUAYSIDE",
            "512.00",
            "",
            "9488.00",
        ),
        (
            "03Aug2026",
            "BY TRANSFER-UPI/CR/612345678901/SYNTHETIC PAYER/XYZ",
            "",
            "71.00",
            "9559.00",
        ),
        (
            "05Aug2026",
            "ATM WDL ATM CASH 6639 SYNTHETIC LOCKSIDE",
            "64.00",
            "",
            "9495.00",
        ),
        // A second unmapped payer, first by name but later in the statement
        // and smaller, so neither appearance nor amount puts it first.
        (
            "06Aug2026",
            "BY TRANSFER-UPI/CR/698765432109/AARDVARK BUYER/XYZ",
            "",
            "9.00",
            "9504.00",
        ),
        // The first payer again, spelled otherwise, and the second payer
        // again, so both groups have two rows and one has two spellings.
        (
            "07Aug2026",
            "BY TRANSFER-UPI/CR/611111111111/Synthetic Payer/XYZ",
            "",
            "10.00",
            "9514.00",
        ),
        (
            "08Aug2026",
            "BY TRANSFER-UPI/CR/622222222222/AARDVARK BUYER/XYZ",
            "",
            "2.00",
            "9516.00",
        ),
    ]
    .map(|(date, narration, dr, cr, bal)| sbi(date, narration, dr, cr, bal));
    let mut args = json!({
        "statement_path": never_opened("statement.pdf"),
        "password_file": never_opened("statement.password"),
        "bank": "sbi",
        "account_label": "Synthetic SB xx1234",
        "opening_balance": "10,000.00",
        "closing_balance": "9,516.00",
        "total_debits": "576.00",
        "total_credits": "92.00",
        "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense"
    });
    let parsed_with = |request: &OwnedRequest| {
        let build = build(
            &rows,
            Bank::Sbi,
            &Mapping::default(),
            &BuildOptions {
                bank_ledger: "Synthetic Bank Ledger",
                suspense_ledger: "Suspense",
                account_label: "Synthetic SB xx1234",
                account_number: "00000000001234",
                date_from: None,
                date_to: None,
                cash_answers: &request.cash_answers,
            },
        )
        .unwrap();
        ParsedStatement {
            account_number: "00000000001234".into(),
            statement_rows: rows.len(),
            closing: bridge_tally_core::ExactDecimal::parse("9516.00").unwrap(),
            totals: bridge_bank_statement::money::statement_totals(&rows).unwrap(),
            check: selfcheck(&build, "Synthetic Bank Ledger").unwrap(),
            counterparties: group_counterparties(&build.records).unwrap(),
            build,
        }
    };
    let unanswered = parsed_with(&OwnedRequest::from_args(&args).unwrap());
    args["cash_answers"] = json!([{
        "bridge_txn_id": unanswered.build.records[0].bridge_txn_id,
        "answer": "owner_use",
        "ledger": "Drawings"
    }]);
    let request = OwnedRequest::from_args(&args).unwrap();
    let parsed = parsed_with(&request);
    let summary = summary(&request, &parsed, "statement-x", "0", 200_000);
    assert_eq!(summary["cash_questions"].as_array().unwrap().len(), 1);

    // Every leaf of the result, by path (array positions as *), so no field
    // can be added anywhere unseen; every number must be a count; and every
    // string holding a digit is listed with where it sits.
    // A path with array positions as *.
    fn shape_of(path: &str) -> String {
        path.split('/')
            .map(|part| {
                if part.parse::<usize>().is_ok() {
                    "*"
                } else {
                    part
                }
            })
            .collect::<Vec<_>>()
            .join("/")
    }
    fn leaves(value: &Value, path: &str, found: &mut Vec<(String, String, Value)>) {
        match value {
            Value::Array(items) => {
                found.push((shape_of(path), path.to_string(), json!([])));
                for (index, item) in items.iter().enumerate() {
                    leaves(item, &format!("{path}/{index}"), found);
                }
            }
            Value::Object(fields) => {
                found.push((shape_of(path), path.to_string(), json!({})));
                for (key, field) in fields {
                    leaves(field, &format!("{path}/{key}"), found);
                }
            }
            leaf => found.push((shape_of(path), path.to_string(), leaf.clone())),
        }
    }
    let mut found = Vec::new();
    leaves(&summary, "", &mut found);
    let shape: std::collections::BTreeSet<&str> =
        found.iter().map(|(shape, _, _)| shape.as_str()).collect();
    let marker = "$bridge_agent_party_name";
    let expected: std::collections::BTreeSet<String> = [
        "",
        "/account_last4",
        "/bank",
        "/cash_questions",
        "/cash_questions/*",
        "/cash_questions/*/amount",
        "/cash_questions/*/answers",
        "/cash_questions/*/answers/*",
        "/cash_questions/*/answers/*/answer",
        "/cash_questions/*/answers/*/ledger_needed",
        "/cash_questions/*/answers/*/not_built",
        "/cash_questions/*/answers/*/text",
        "/cash_questions/*/bridge_txn_id",
        "/cash_questions/*/date",
        "/cash_questions/*/movement",
        "/cash_questions/*/printed_as",
        "/cash_questions/*/printed_as/{marker}",
        "/cash_questions/*/question",
        "/cash_questions_omitted",
        "/cash_questions_open",
        "/counterparties",
        "/counterparties/*",
        "/counterparties/*/also_printed_as",
        "/counterparties/*/also_printed_as/*",
        "/counterparties/*/also_printed_as/*/{marker}",
        "/counterparties/*/disposition",
        "/counterparties/*/ledger",
        "/counterparties/*/ledger/{marker}",
        "/counterparties/*/party",
        "/counterparties/*/party/{marker}",
        "/counterparties/*/rows",
        "/counterparties/*/suspense",
        "/counterparties_omitted",
        "/ledgers_to_validate",
        "/ledgers_to_validate/*",
        "/ledgers_to_validate/*/{marker}",
        "/ledgers_to_validate_omitted",
        "/next_step",
        "/proposals_id",
        "/reconciled",
        "/reconciled/closing_balance",
        "/reconciled/running_balance_every_row",
        "/reconciled/total_credits",
        "/reconciled/total_debits",
        "/reconciled/totals_match_statement",
        "/rows_in_window",
        "/sha256",
        "/skipped",
        "/statement_rows",
        "/suspense_by_reason",
        "/suspense_by_reason/cash_purpose_not_confirmed",
        "/suspense_by_reason/party_unmapped_or_mapped_to_suspense",
        "/suspense_rows",
        "/vouchers",
    ]
    .iter()
    .map(|path| path.replace("{marker}", marker))
    .collect();
    // Every path, containers included, so an empty list's path is pinned too.
    assert_eq!(
        shape,
        expected.iter().map(String::as_str).collect(),
        "{summary}"
    );
    let numbers: std::collections::BTreeSet<&str> = found
        .iter()
        .filter(|(_, _, leaf)| leaf.is_number())
        .map(|(shape, _, _)| shape.as_str())
        .collect();
    // Numbers are counts, and only counts.
    assert_eq!(
        numbers,
        [
            "/cash_questions_omitted",
            "/cash_questions_open",
            "/counterparties/*/rows",
            "/counterparties_omitted",
            "/ledgers_to_validate_omitted",
            "/rows_in_window",
            "/skipped",
            "/statement_rows",
            "/suspense_by_reason/cash_purpose_not_confirmed",
            "/suspense_by_reason/party_unmapped_or_mapped_to_suspense",
            "/suspense_rows",
            "/vouchers",
        ]
        .into_iter()
        .collect(),
        "{summary}"
    );
    let mut digits: Vec<(String, String)> = found
        .iter()
        .filter_map(|(_, path, leaf)| {
            leaf.as_str()
                .filter(|text| text.chars().any(|c| c.is_ascii_digit()))
                .map(|text| (path.clone(), text.to_string()))
        })
        .collect();
    digits.sort();
    // A digit appears only in the caller's echoes, the open cash line's
    // fields (b1), the account's last four digits and the file digest.
    let open_id = summary["cash_questions"][0]["bridge_txn_id"]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(
        digits,
        [
            ("/account_last4", "1234"),
            ("/cash_questions/0/amount", "64.00"),
            ("/cash_questions/0/bridge_txn_id", open_id.as_str()),
            ("/cash_questions/0/date", "2026-08-05"),
            ("/reconciled/closing_balance", "9516.00"),
            ("/reconciled/total_credits", "92.00"),
            ("/reconciled/total_debits", "576.00"),
            ("/sha256", "0"),
        ]
        .map(|(path, text)| (path.to_string(), text.to_string())),
        "{summary}"
    );
    assert!(open_id.starts_with("st-20260805-"), "{open_id}");
    // Ordered by suspense, rows, name and disposition, never by amount: the
    // answered 512.00 line and the open 64.00 line share a spelling, and the
    // open one (NeedsAnswer) comes first by disposition.
    let order: Vec<(Value, Value)> = summary["counterparties"]
        .as_array()
        .unwrap()
        .iter()
        .map(|group| (group["party"].clone(), group["disposition"].clone()))
        .collect();
    let party = |name: &str| serde_json::to_value(party_name(name)).unwrap();
    assert_eq!(
        order,
        [
            (party("AARDVARK BUYER"), json!("Receipt")),
            (party("SYNTHETIC PAYER"), json!("Receipt")),
            (party("ATM CASH WITHDRAWAL"), json!("NeedsAnswer")),
            (party("ATM CASH WITHDRAWAL"), json!("Payment")),
        ]
    );
    // The payer printed two ways is one group of two rows, shown by the first
    // spelling by name, with the other listed.
    assert_eq!(summary["counterparties"][0]["rows"], 2, "{summary}");
    let payer = &summary["counterparties"][1];
    assert_eq!(payer["rows"], 2, "{payer}");
    assert_eq!(payer["also_printed_as"], json!([party("Synthetic Payer")]));
    // A counterparty gives its rows, and no total.
    for group in summary["counterparties"].as_array().unwrap() {
        assert!(group.get("total").is_none(), "{group}");
        assert!(group["rows"].as_u64().unwrap() >= 1, "{group}");
    }
}

#[tokio::test]
async fn a_path_that_is_not_on_a_local_disk_is_refused_before_any_open() {
    let directory = tempfile::tempdir().unwrap();
    let server = server(directory.path(), true, Redaction::None);
    // Every other argument is valid and neither file exists. A path that got
    // as far as an open would therefore answer statement_file_unreadable; the
    // refusal asserted here is the one made on the text, before any open.
    let missing = directory.path().join("absent.pdf");
    let base = arguments(&missing, &directory.path().join("absent.password"));
    assert_eq!(
        error_code(&server.call_tool("parse_bank_statement", base.clone()).await),
        Some("statement_file_unreadable"),
        "the control: a local path that does not exist is opened and fails there"
    );
    for text in [
        r"\\host\share\statement.pdf",
        r"\\host@80\share\statement.pdf",
        r"\\?\UNC\host\share\statement.pdf",
        r"\\?\C:\statement.pdf",
        r"\\.\C:\statement.pdf",
        "//host/share/statement.pdf",
        r"/\host/share/statement.pdf",
    ] {
        for key in ["statement_path", "password_file"] {
            let mut args = base.clone();
            args[key] = json!(text);
            let response = server.call_tool("parse_bank_statement", args).await;
            let expected = format!("argument_invalid:{key}");
            assert_eq!(
                error_code(&response),
                Some(expected.as_str()),
                "{key} {text}"
            );
        }
    }
}

/// A parse of three synthetic rows printed out of date order (so the earliest
/// and latest dates are neither the first nor the last row printed), built
/// without a PDF, optionally through a from/to window.
fn windowed_parse(
    date_from: Option<Date>,
    date_to: Option<Date>,
) -> (OwnedRequest, ParsedStatement) {
    use bridge_bank_statement::parse::Row;
    use bridge_bank_statement::proposals::{build, group_counterparties, selfcheck, BuildOptions};
    let upi = |date: &str, name: &str, reference: &str, dr: &str, balance: &str| {
        let narration = format!("UPI-{name}-9@x-ABCD0001-{reference}-P");
        Row::from_pairs([
            ("date", date),
            ("narr", narration.as_str()),
            ("ref", "1"),
            ("dr", dr),
            ("cr", ""),
            ("bal", balance),
        ])
    };
    let rows = [
        upi("05/08/26", "ALPHA", "111111111111", "10.00", "990.00"),
        upi("09/08/26", "BRAVO", "222222222222", "10.00", "980.00"),
        upi("01/08/26", "CHARLIE", "333333333333", "10.00", "970.00"),
    ];
    let args = json!({
        "statement_path": never_opened("statement.pdf"),
        "password_file": never_opened("statement.password"),
        "bank": "hdfc",
        "account_label": "Synthetic CA xx4321",
        "opening_balance": "1,000.00",
        "closing_balance": "970.00",
        "total_debits": "30.00",
        "total_credits": "0.00",
        "bank_ledger": "Synthetic Bank Ledger",
        "suspense_ledger": "Suspense"
    });
    let request = OwnedRequest::from_args(&args).unwrap();
    let build = build(
        &rows,
        Bank::Hdfc,
        &Mapping::default(),
        &BuildOptions {
            bank_ledger: "Synthetic Bank Ledger",
            suspense_ledger: "Suspense",
            account_label: "Synthetic CA xx4321",
            account_number: "00000000004321",
            date_from,
            date_to,
            cash_answers: &request.cash_answers,
        },
    )
    .unwrap();
    let parsed = ParsedStatement {
        account_number: "00000000004321".into(),
        statement_rows: rows.len(),
        closing: bridge_tally_core::ExactDecimal::parse("970.00").unwrap(),
        totals: bridge_bank_statement::money::statement_totals(&rows).unwrap(),
        check: selfcheck(&build, "Synthetic Bank Ledger").unwrap(),
        counterparties: group_counterparties(&build.records).unwrap(),
        build,
    };
    (request, parsed)
}

fn persisted_document(request: &OwnedRequest, parsed: &ParsedStatement) -> Value {
    let directory = tempfile::tempdir().unwrap();
    let (proposals_id, _) = persist(directory.path(), request, parsed, &"0".repeat(64)).unwrap();
    let path = directory
        .path()
        .join(PROPOSALS_DIRECTORY)
        .join(format!("{proposals_id}.json"));
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

#[test]
fn the_file_records_the_statements_own_span_and_whether_the_window_was_all_of_it() {
    let (request, parsed) = windowed_parse(None, None);
    let document = persisted_document(&request, &parsed);
    assert_eq!(
        document["window"],
        json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-09", "whole_statement": true}),
    );
    assert_eq!(
        document["schema"], PROPOSALS_SCHEMA,
        "an optional field, not a new schema"
    );

    // A window that drops a row: the span is still the statement's own, and
    // the window is no longer the whole of it.
    let (request, parsed) = windowed_parse(Date::new(2026, 8, 2), None);
    let document = persisted_document(&request, &parsed);
    assert_eq!(
        document["window"],
        json!({"first_row_date": "2026-08-01", "last_row_date": "2026-08-09", "whole_statement": false}),
    );
}

#[test]
fn a_build_with_no_span_writes_no_window_field() {
    let (request, mut parsed) = windowed_parse(None, None);
    parsed.build.span = None;
    let document = persisted_document(&request, &parsed);
    assert!(document.get("window").is_none(), "{document}");
}

#[test]
fn the_window_never_reaches_the_summary() {
    let (request, parsed) = windowed_parse(None, None);
    let summary = summary(&request, &parsed, "statement-x", "0", 200_000).to_string();
    for needle in [
        "\"window\"",
        "first_row_date",
        "last_row_date",
        "whole_statement",
        "2026-08-0",
    ] {
        assert!(!summary.contains(needle), "{needle} in {summary}");
    }
}
