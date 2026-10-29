//! `parse_bank_statement_csv`: the tool shell over the CSV edge reader (#1320).
//! Every file here is synthetic rows in the layout the edge reads; the people,
//! accounts and references do not exist. Assertions are on the typed error code
//! (and cause), never on message text.
use super::*;

const HEADER: &str = "\"Sl. No.\",\"Transaction Date\",\"Value Date\",\"Description\",\"Chq /Ref No.\",\"Amount\",\"Dr / Cr\",\"Balance\",\"Dr / Cr\"";
const CLOSING: &str = "\"Closing Balance\",\"as on 31/03/2026 INR 11,249.50\"";
const ROW_ONE: &str = "\"1\",\"02-04-2025 10:15:00\",\"02-04-2025\",\"UPI/SYNTHETIC PAYER\",\"SYN 000001\",\"1,500.00\",\"CR\",\"11,500.00\",\"CR\"";
const ROW_TWO: &str = "\"2\",\"03-04-2025 09:00:01\",\"03-04-2025\",\"NEFT/SYNTHETIC SUPPLIER\",\"\",\"250.50\",\"DR\",\"11,249.50\",\"CR\"";

/// A file of the layout: a short preamble, the header, `rows`, then `trailer`.
fn file(header: &str, rows: &[&str], trailer: &[&str]) -> Vec<u8> {
    let mut lines = vec![
        "\"\",\"\",\"Account Statement\"".to_string(),
        "\"SYNTHETIC LAB CUSTOMER\",\"\",\"\",\"\",\"Currency\",\"INR\"".to_string(),
        String::new(),
        header.to_string(),
    ];
    lines.extend(rows.iter().map(|row| row.to_string()));
    lines.extend(trailer.iter().map(|line| line.to_string()));
    lines.push(String::new());
    lines.join("\n").into_bytes()
}

fn good() -> Vec<u8> {
    file(HEADER, &[ROW_ONE, ROW_TWO], &[CLOSING])
}

/// The refusal each broken file must give, as the category and the row it is at.
fn broken() -> Vec<(&'static str, Vec<u8>, &'static str)> {
    let with_row_two = |row: &str| file(HEADER, &[ROW_ONE, row], &[CLOSING]);
    let mut bom = vec![0xEF, 0xBB, 0xBF];
    bom.extend(good());
    vec![
        (
            "encoding_unsupported",
            bom,
            "statement_encoding_unsupported",
        ),
        (
            "layout_unknown",
            file(
                &HEADER.replace("Value Date", "Val Date"),
                &[ROW_ONE, ROW_TWO],
                &[CLOSING],
            ),
            "statement_layout_unknown",
        ),
        (
            "malformed_row",
            with_row_two("2,03-04-2025,not quoted"),
            "statement_malformed_row:row_2",
        ),
        (
            "serial_broken",
            with_row_two(&ROW_TWO.replacen("\"2\"", "\"3\"", 1)),
            "statement_serial_broken:row_2",
        ),
        (
            "unparseable_date",
            with_row_two(&ROW_TWO.replacen("\"03-04-2025\"", "\"31-02-2025\"", 1)),
            "statement_unparseable_date:row_2",
        ),
        (
            "date_order_broken",
            with_row_two(&ROW_TWO.replacen("03-04-2025 09:00:01", "01-04-2025 09:00:01", 1)),
            "statement_date_order_broken:row_2",
        ),
        (
            "malformed_amount",
            with_row_two(&ROW_TWO.replacen("\"250.50\"", "\"250.5\"", 1)),
            "statement_malformed_amount:row_2",
        ),
        (
            "malformed_balance",
            with_row_two(&ROW_TWO.replacen("\"11,249.50\"", "\"11249.50\"", 1)),
            "statement_malformed_balance:row_2",
        ),
        (
            "direction_unknown",
            with_row_two(&ROW_TWO.replacen("\"DR\"", "\"XX\"", 1)),
            "statement_direction_unknown:row_2",
        ),
        (
            "closing_balance_unreadable",
            file(
                HEADER,
                &[ROW_ONE, ROW_TWO],
                &["\"Closing Balance\",\"as on 31/03/2026\""],
            ),
            "statement_closing_balance_unreadable",
        ),
        (
            "closing_balance_differs",
            file(
                HEADER,
                &[ROW_ONE, ROW_TWO],
                &["\"Closing Balance\",\"as on 31/03/2026 INR 11,249.51\""],
            ),
            "statement_closing_balance_differs",
        ),
        (
            "trailer_missing",
            file(HEADER, &[ROW_ONE, ROW_TWO], &[]),
            "statement_trailer_missing",
        ),
        (
            "trailer_unexpected",
            file(HEADER, &[ROW_ONE, ROW_TWO], &[CLOSING, ROW_TWO]),
            "statement_trailer_unexpected",
        ),
        (
            "empty_statement",
            file(HEADER, &[], &[CLOSING]),
            "statement_empty_statement",
        ),
        (
            // Over the reader's own cap, so the file reader refuses it first,
            // under the same code the edge's category maps to.
            "file_too_large",
            vec![b'x'; csv_edge::MAX_FILE_BYTES + 1],
            "statement_file_too_large",
        ),
    ]
}

fn server(directory: &Path, import_enabled: bool) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".to_string(),
            port: 9,
        },
        data_dir: directory.join("agent"),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled,
        posting: Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    })
}

fn error_of(response: &Value) -> &Value {
    &response["structuredContent"]["result"]["error"]
}

fn code_of(response: &Value) -> Option<&str> {
    error_of(response)["code"].as_str()
}

fn written(directory: &Path, name: &str, bytes: &[u8]) -> String {
    let path = directory.join(name);
    fs::write(&path, bytes).unwrap();
    path.to_str().unwrap().to_string()
}

async fn call(directory: &Path, args: Value) -> Value {
    server(directory, true)
        .call_tool("parse_bank_statement_csv", args)
        .await
}

#[tokio::test]
async fn each_refusal_category_is_its_own_typed_code() {
    let directory = tempfile::tempdir().unwrap();
    for (category, bytes, expected) in broken() {
        let path = written(directory.path(), "statement.csv", &bytes);
        let response = call(
            directory.path(),
            json!({"statement_path": path, "controls_source": "prior_statement"}),
        )
        .await;
        assert_eq!(code_of(&response), Some(expected), "{category}");
        // A refusal about the file says nothing of the controls.
        assert!(error_of(&response).get("cause").is_none(), "{category}");
    }
}

#[test]
fn the_categories_the_edge_reader_gives_are_exactly_the_variants_the_tool_maps() {
    // The crate is the source of truth: each broken file goes through its
    // reader, and the set of categories it gives must be the variant list, so
    // a category added there without a variant here fails on the next fixture
    // written for it, and a variant no file reaches fails here.
    let mut reached = Vec::new();
    for (category, bytes, _) in broken() {
        let refusal = csv_edge::read_statement(&bytes).expect_err(category);
        assert_eq!(refusal.category, category);
        reached.push(refusal.category);
    }
    reached.sort_unstable();
    let mut mapped: Vec<&str> = CsvRefusal::ALL.iter().map(|kind| kind.category()).collect();
    mapped.sort_unstable();
    assert_eq!(reached, mapped);
    assert_eq!(mapped.len(), 15);
    for kind in CsvRefusal::ALL {
        assert_eq!(CsvRefusal::from_category(kind.category()), Some(*kind));
    }
}

#[test]
fn a_category_the_tool_does_not_know_is_refused_as_unmapped_not_forwarded() {
    let failure = csv_failure(&Refusal::new("category_added_later", "synthetic"));
    assert_eq!(failure.code, CSV_REFUSAL_UNMAPPED);
    let at_row = csv_failure(&Refusal::at_row("category_added_later", 3, "synthetic"));
    assert_eq!(at_row.code, CSV_REFUSAL_UNMAPPED);
    // The edge's own over-size category maps too, though the file reader's cap
    // normally gets there first.
    let too_large = csv_failure(&Refusal::new("file_too_large", "synthetic"));
    assert_eq!(too_large.code, "statement_file_too_large");
}

#[tokio::test]
async fn an_unknown_header_is_refused_not_guessed() {
    let directory = tempfile::tempdir().unwrap();
    let columns_swapped = HEADER.replacen("\"Amount\",\"Dr / Cr\"", "\"Dr / Cr\",\"Amount\"", 1);
    let another_bank = "\"Date\",\"Narration\",\"Withdrawal\",\"Deposit\",\"Balance\"";
    for header in [columns_swapped.as_str(), another_bank] {
        let path = written(
            directory.path(),
            "statement.csv",
            &file(header, &[ROW_ONE, ROW_TWO], &[CLOSING]),
        );
        let response = call(
            directory.path(),
            json!({"statement_path": path, "controls_source": "this_file"}),
        )
        .await;
        assert_eq!(code_of(&response), Some("statement_layout_unknown"));
    }
}

#[tokio::test]
async fn controls_source_is_required_and_only_the_three_values_are_admitted() {
    let directory = tempfile::tempdir().unwrap();
    let path = written(directory.path(), "statement.csv", &good());
    let response = call(directory.path(), json!({"statement_path": path})).await;
    assert_eq!(code_of(&response), Some("controls_source_required"));
    for wrong in [
        json!(""),
        json!("prior"),
        json!("THIS_FILE"),
        json!(" this_file"),
        json!("this_file "),
        json!("bank_ledger"),
        json!(5),
        json!(null),
        json!(["this_file"]),
    ] {
        let response = call(
            directory.path(),
            json!({"statement_path": path, "controls_source": wrong}),
        )
        .await;
        assert_eq!(
            code_of(&response),
            Some("argument_invalid:controls_source"),
            "{wrong}"
        );
    }
    // Refused before the file is looked at: the path does not exist.
    let absent = never_opened_csv();
    let response = call(
        directory.path(),
        json!({"statement_path": absent, "controls_source": "someone_said_so"}),
    )
    .await;
    assert_eq!(code_of(&response), Some("argument_invalid:controls_source"));
    // Each admitted value gets as far as the file.
    for source in ["prior_statement", "bank_ledger_in_tally", "this_file"] {
        let response = call(
            directory.path(),
            json!({"statement_path": absent, "controls_source": source}),
        )
        .await;
        assert_eq!(code_of(&response), Some("statement_file_unreadable"));
    }
    assert_eq!(
        ControlsSource::parse("prior_statement"),
        Some(ControlsSource::PriorStatement)
    );
    assert_eq!(
        ControlsSource::parse("bank_ledger_in_tally"),
        Some(ControlsSource::BankLedgerInTally)
    );
    assert_eq!(
        ControlsSource::parse("this_file"),
        Some(ControlsSource::ThisFile)
    );
}

fn never_opened_csv() -> String {
    std::env::temp_dir()
        .join("bridge-never-opened-statement.csv")
        .to_string_lossy()
        .into_owned()
}

#[tokio::test]
async fn a_file_that_reads_is_refused_for_want_of_an_account_line_and_this_file_is_labelled() {
    let directory = tempfile::tempdir().unwrap();
    let path = written(directory.path(), "statement.csv", &good());
    for (source, label) in [
        ("prior_statement", None),
        ("bank_ledger_in_tally", None),
        ("this_file", Some("this_file: derived, not independent")),
    ] {
        let response = call(
            directory.path(),
            json!({"statement_path": path, "controls_source": source}),
        )
        .await;
        assert_eq!(code_of(&response), Some(NO_ACCOUNT_NUMBER_LINE), "{source}");
        assert_eq!(error_of(&response)["cause"].as_str(), label, "{source}");
        assert!(
            error_of(&response)["remediation"].is_string(),
            "{source}: the refusal says what happens next"
        );
    }
    // Nothing was proposed or written.
    assert!(!directory.path().join("agent").exists());
}

#[tokio::test]
async fn the_tool_is_hidden_and_refused_without_the_import_opt_in() {
    for (enabled, listed) in [(false, false), (true, true)] {
        assert_eq!(
            tool_definitions(enabled, false)
                .as_array()
                .unwrap()
                .iter()
                .any(|tool| tool["name"] == "parse_bank_statement_csv"),
            listed
        );
    }
    let directory = tempfile::tempdir().unwrap();
    let path = written(directory.path(), "statement.csv", &good());
    let response = server(directory.path(), false)
        .call_tool(
            "parse_bank_statement_csv",
            json!({"statement_path": path, "controls_source": "this_file"}),
        )
        .await;
    assert_eq!(code_of(&response), Some("import_unverified_on_live_tally"));
}

#[tokio::test]
async fn the_path_rules_are_the_pdf_tools_and_unknown_arguments_are_refused() {
    let directory = tempfile::tempdir().unwrap();
    for path in [
        "relative/statement.csv",
        "\\\\server\\share\\statement.csv",
        "",
    ] {
        let response = call(
            directory.path(),
            json!({"statement_path": path, "controls_source": "this_file"}),
        )
        .await;
        assert_eq!(
            code_of(&response),
            Some("argument_invalid:statement_path"),
            "{path}"
        );
    }
    let response = call(
        directory.path(),
        json!({
            "statement_path": never_opened_csv(),
            "controls_source": "this_file",
            "account_label": "Synthetic CA xx4321"
        }),
    )
    .await;
    assert_eq!(code_of(&response), Some("argument_unknown"));
}
