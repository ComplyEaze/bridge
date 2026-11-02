//! `parse_bank_statement_csv`: the tool over the CSV edge reader (#1320).
//! Every file here is synthetic rows in the layout the edge reads; the people,
//! accounts and references do not exist. Assertions are on the typed error code,
//! never on message text.
use super::*;

const HEADER: &str = "\"Sl. No.\",\"Transaction Date\",\"Value Date\",\"Description\",\"Chq /Ref No.\",\"Amount\",\"Dr / Cr\",\"Balance\",\"Dr / Cr\"";
const CLOSING: &str = "\"Closing Balance\",\"as on 31/03/2026 INR 11,249.50\"";
const ROW_ONE: &str = "\"1\",\"02-04-2025 10:15:00\",\"02-04-2025\",\"UPI/SYNTHETIC PAYER\",\"SYN 000001\",\"1,500.00\",\"CR\",\"11,500.00\",\"CR\"";
const ROW_TWO: &str = "\"2\",\"03-04-2025 09:00:01\",\"03-04-2025\",\"NEFT/SYNTHETIC SUPPLIER\",\"\",\"250.50\",\"DR\",\"11,249.50\",\"CR\"";

const ACCOUNT_LINE: &str = "\"1 SYNTHETIC ROAD\",\"\",\"\",\"\",\"Account No.\",\"000000004321\"";
const OPENING: &str = "10,000.00";
const CLOSING_FIGURE: &str = "11,249.50";

/// A file of the layout: a short preamble, the header, `rows`, then `trailer`.
fn file(header: &str, rows: &[&str], trailer: &[&str]) -> Vec<u8> {
    let mut lines = vec![
        "\"\",\"\",\"Account Statement\"".to_string(),
        "\"SYNTHETIC LAB CUSTOMER\",\"\",\"\",\"\",\"Currency\",\"INR\"".to_string(),
        ACCOUNT_LINE.to_string(),
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
    let with_account = |line: &str| {
        String::from_utf8(good())
            .unwrap()
            .replace(ACCOUNT_LINE, line)
            .into_bytes()
    };
    vec![
        (
            "no_account_number_line",
            with_account("\"1 SYNTHETIC ROAD\""),
            "statement_no_account_number_line",
        ),
        (
            "no_account_number_line",
            with_account("\"1 SYNTHETIC ROAD\",\"\",\"\",\"\",\"Account No.\",\"XXXXXXXX4321\""),
            "statement_no_account_number_line",
        ),
        (
            "no_account_number_line",
            with_account("\"1 SYNTHETIC ROAD\",\"\",\"\",\"\",\"Account No.\",\"\""),
            "statement_no_account_number_line",
        ),
        (
            "ambiguous_account_match",
            with_account(&format!("{ACCOUNT_LINE}\n{ACCOUNT_LINE}")),
            "statement_ambiguous_account_match",
        ),
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
    server_with_budget(directory, import_enabled, 200_000)
}

fn server_with_budget(directory: &Path, import_enabled: bool, max_bytes: usize) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".to_string(),
            port: 9,
        },
        data_dir: directory.join("agent"),
        max_rows: 500,
        max_bytes,
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

/// `args` with the arguments every run needs, each left as `args` gives it.
fn complete(mut args: Value) -> Value {
    let map = args.as_object_mut().unwrap();
    let independent = map
        .get("controls_source")
        .is_some_and(|source| source != "this_file");
    let mut fill = |key: &str, value: &str| {
        map.entry(key).or_insert_with(|| json!(value));
    };
    fill("account_label", "Synthetic CA xx4321");
    fill("bank_ledger", "Synthetic Bank");
    fill("suspense_ledger", "Suspense");
    if independent {
        fill("opening_balance", OPENING);
        fill("closing_balance", CLOSING_FIGURE);
    }
    args
}

async fn call(directory: &Path, args: Value) -> Value {
    server(directory, true)
        .call_tool("parse_bank_statement_csv", complete(args))
        .await
}

fn result_of(response: &Value) -> &Value {
    let result = &response["structuredContent"]["result"];
    assert!(result.get("error").is_none(), "{response}");
    result
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
    }
}

#[test]
fn the_categories_the_edge_reader_gives_are_exactly_the_variants_the_tool_maps() {
    // Each broken file goes through the crate's own reader, and the set of
    // categories it gives must be the variant list: a variant no file reaches,
    // or one renamed away from the crate's spelling, fails here. The crate
    // exports no list of its categories, so a category it adds later is not
    // caught here; it is refused at run time as unmapped, which is proved by
    // `a_category_the_tool_does_not_know_is_refused_as_unmapped_not_forwarded`.
    let mut reached = Vec::new();
    for (category, bytes, _) in broken() {
        let refusal = csv_edge::read_statement(&bytes).expect_err(category);
        assert_eq!(refusal.category, category);
        reached.push(refusal.category);
    }
    reached.sort_unstable();
    reached.dedup();
    let mut mapped: Vec<&str> = CsvRefusal::ALL.iter().map(|kind| kind.category()).collect();
    mapped.sort_unstable();
    assert_eq!(reached, mapped);
    assert_eq!(mapped.len(), 17);
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
    let absent = absent_in(directory.path());
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

/// The tool's own boundary, called directly: the catalogue's enum check would
/// refuse these values before `from_args` runs, so a call through the tool
/// cannot tell whether `ControlsSource::parse` admits only the three.
#[test]
fn controls_source_parse_admits_exactly_the_three_values_and_nothing_else() {
    for wrong in [
        "",
        "prior",
        "this",
        "THIS_FILE",
        " this_file",
        "this_file ",
        "this_file\n",
        "bank_ledger",
        "someone_said_so",
    ] {
        assert_eq!(ControlsSource::parse(wrong), None, "{wrong:?}");
    }
    let directory = tempfile::tempdir().unwrap();
    let path = absent_in(directory.path());
    for wrong in ["prior", "THIS_FILE", "someone_said_so"] {
        let refused =
            CsvRequest::from_args(&json!({"statement_path": path, "controls_source": wrong}))
                .err()
                .expect("a value outside the three is refused");
        assert_eq!(refused, "argument_invalid:controls_source", "{wrong:?}");
    }
    for (text, source) in [
        ("prior_statement", ControlsSource::PriorStatement),
        ("bank_ledger_in_tally", ControlsSource::BankLedgerInTally),
        ("this_file", ControlsSource::ThisFile),
    ] {
        let request = CsvRequest::from_args(&complete(
            json!({"statement_path": path, "controls_source": text}),
        ))
        .expect("a valid value is admitted");
        assert_eq!(request.controls_source, source);
    }
}

/// A path in the test's own directory where no file exists.
fn absent_in(directory: &Path) -> String {
    directory
        .join("absent-statement.csv")
        .to_str()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn a_file_that_reads_writes_its_proposals_and_says_where_its_figures_came_from() {
    let directory = tempfile::tempdir().unwrap();
    let path = written(directory.path(), "statement.csv", &good());
    for (source, independent) in [
        ("prior_statement", true),
        ("bank_ledger_in_tally", true),
        ("this_file", false),
    ] {
        let response = call(
            directory.path(),
            json!({"statement_path": path, "controls_source": source}),
        )
        .await;
        let result = result_of(&response);
        assert_eq!(result["bank"], "csv_bank_a", "{source}");
        assert_eq!(result["account_last4"], "4321", "{source}");
        assert_eq!(result["statement_rows"], 2, "{source}");
        assert_eq!(result["vouchers"], 2, "{source}");
        assert_eq!(result["controls_source"], source);
        assert_eq!(result["controls_independent"], independent, "{source}");
        assert_eq!(result["reconciled"]["running_balance_every_row"], true);
        assert_eq!(result["reconciled"]["totals_match_statement"], false);
        // `build_import_xml` reads the same file through this loader.
        let document = load_proposals(
            &directory.path().join("agent"),
            &result["proposals_id"],
            Some(&result["sha256"]),
        )
        .unwrap();
        assert_eq!(document["controls_source"], source);
        assert_eq!(document["vouchers"].as_array().unwrap().len(), 2);
    }
}

/// The party is the whole description: nothing is read out of it, so only an exact mapping
/// reaches a ledger, and a line that merely looks like cash is a suspense line here.
#[tokio::test]
async fn a_row_reaches_a_ledger_only_by_a_mapping_of_its_whole_description() {
    let directory = tempfile::tempdir().unwrap();
    let cash = "\"2\",\"03-04-2025 09:00:01\",\"03-04-2025\",\"ATM WDL SYNTHETIC\",\"\",\"250.50\",\"DR\",\"11,249.50\",\"CR\"";
    let path = written(
        directory.path(),
        "statement.csv",
        &file(HEADER, &[ROW_ONE, cash], &[CLOSING]),
    );
    let response = call(
        directory.path(),
        json!({
            "statement_path": path,
            "controls_source": "prior_statement",
            "mapping": [{"party": "UPI/SYNTHETIC PAYER", "ledger": "Synthetic Debtor"}],
        }),
    )
    .await;
    let result = result_of(&response);
    assert_eq!(result["suspense_rows"], 1, "{result}");
    assert_eq!(result["cash_questions_open"], 0, "{result}");
    assert_eq!(result["counterparties"].as_array().unwrap().len(), 2);
    let partly = call(
        directory.path(),
        json!({
            "statement_path": path,
            "controls_source": "prior_statement",
            "mapping": [{"party": "SYNTHETIC PAYER", "ledger": "Synthetic Debtor"}],
        }),
    )
    .await;
    assert_eq!(result_of(&partly)["suspense_rows"], 2, "{partly}");
}

/// A description with a digit may be a reference or an account number, so the row is unnamed
/// and the text never reaches the result.
#[tokio::test]
async fn a_description_with_a_digit_stays_out_of_the_result() {
    let directory = tempfile::tempdir().unwrap();
    let numbered = ROW_ONE.replace("UPI/SYNTHETIC PAYER", "UPI/SYNTHETIC PAYER/000111222");
    let path = written(
        directory.path(),
        "statement.csv",
        &file(HEADER, &[numbered.as_str(), ROW_TWO], &[CLOSING]),
    );
    let response = call(
        directory.path(),
        json!({"statement_path": path, "controls_source": "prior_statement"}),
    )
    .await;
    let result = result_of(&response);
    assert_eq!(result["suspense_rows"], 2, "{result}");
    assert!(!response.to_string().contains("000111222"), "{response}");
}

/// The one cash route left: a row whose whole description is the cash text the proposals
/// already ask about, so it is answerable and never stuck.
#[tokio::test]
async fn a_description_of_exactly_the_cash_text_asks_the_cash_question() {
    let directory = tempfile::tempdir().unwrap();
    let cash = ROW_TWO.replace("NEFT/SYNTHETIC SUPPLIER", "ATM CASH WITHDRAWAL");
    let path = written(
        directory.path(),
        "statement.csv",
        &file(HEADER, &[ROW_ONE, cash.as_str()], &[CLOSING]),
    );
    let response = call(
        directory.path(),
        json!({"statement_path": path, "controls_source": "prior_statement"}),
    )
    .await;
    assert_eq!(result_of(&response)["cash_questions_open"], 1, "{response}");
}

#[tokio::test]
async fn the_file_binds_to_the_account_before_its_arithmetic_is_read() {
    let directory = tempfile::tempdir().unwrap();
    let path = written(directory.path(), "statement.csv", &good());
    for (label, code) in [
        ("Synthetic CA xx9999", "statement_account_not_in_statement"),
        ("xx21", "statement_unbindable_account"),
    ] {
        let response = call(
            directory.path(),
            json!({
                "statement_path": path,
                "controls_source": "prior_statement",
                "account_label": label,
                "closing_balance": "1.00",
            }),
        )
        .await;
        assert_eq!(code_of(&response), Some(code), "{label}");
    }
    assert!(!directory.path().join("agent/bank-statements").exists());
}

/// A source other than the file must give both figures, and they are checked: a row cut from
/// the end of a file with its closing line changed to fit passes only `this_file`.
#[tokio::test]
async fn figures_are_required_from_another_source_refused_from_this_file_and_checked() {
    let directory = tempfile::tempdir().unwrap();
    let path = written(directory.path(), "statement.csv", &good());
    for (args, code) in [
        (
            json!({"controls_source": "this_file", "opening_balance": OPENING}),
            "statement_control_figures_with_this_file",
        ),
        (
            json!({"controls_source": "this_file", "closing_balance": CLOSING_FIGURE}),
            "statement_control_figures_with_this_file",
        ),
    ] {
        let mut args = args;
        args["statement_path"] = json!(path);
        let response = call(directory.path(), args).await;
        assert_eq!(code_of(&response), Some(code));
        assert!(error_of(&response)["remediation"].is_string());
    }
    for args in [
        json!({"account_label": "xx4321", "bank_ledger": "B", "suspense_ledger": "S"}),
        json!({"account_label": "xx4321", "bank_ledger": "B", "suspense_ledger": "S", "opening_balance": OPENING}),
        json!({"account_label": "xx4321", "bank_ledger": "B", "suspense_ledger": "S", "closing_balance": CLOSING_FIGURE}),
    ] {
        let mut args = args;
        args["statement_path"] = json!(path);
        args["controls_source"] = json!("prior_statement");
        let response = server(directory.path(), true)
            .call_tool("parse_bank_statement_csv", args)
            .await;
        assert_eq!(
            code_of(&response),
            Some("statement_control_figures_required")
        );
        assert!(error_of(&response)["remediation"].is_string());
    }
    let short = "\"Closing Balance\",\"as on 31/03/2026 INR 11,500.00\"";
    let cut = written(
        directory.path(),
        "cut.csv",
        &file(HEADER, &[ROW_ONE], &[short]),
    );
    let response = call(
        directory.path(),
        json!({"statement_path": cut, "controls_source": "prior_statement"}),
    )
    .await;
    assert_eq!(code_of(&response), Some("statement_extent_unproven"));
    let response = call(
        directory.path(),
        json!({"statement_path": cut, "controls_source": "this_file"}),
    )
    .await;
    assert_eq!(result_of(&response)["controls_independent"], false);
}

/// The bound is the edge reader's: exactly that many bytes reach the reader
/// (and are refused for their content), one more is too large. The reader's
/// own bound is 32 MiB (`MAX_PDF_BYTES`), so a file read to that bound instead
/// would come back whole and these two assertions would swap.
#[test]
fn the_file_is_read_to_the_edge_readers_bound_and_no_further() {
    let directory = tempfile::tempdir().unwrap();
    let bound = csv_edge::MAX_FILE_BYTES;
    let at_bound = LocalDiskPath::parse(&written(directory.path(), "at.csv", &vec![b'x'; bound]))
        .expect("an absolute local path");
    let past_bound = LocalDiskPath::parse(&written(
        directory.path(),
        "past.csv",
        &vec![b'x'; bound + 1],
    ))
    .expect("an absolute local path");
    assert_eq!(read_csv_file(&at_bound).map(|bytes| bytes.len()), Ok(bound));
    assert_eq!(
        read_csv_file(&past_bound),
        Err("statement_file_too_large".to_string())
    );
}

/// Through the tool: a file of exactly the bound is judged on its content, and
/// one byte more is refused for its size, each under its own typed code.
#[tokio::test]
async fn a_file_at_the_bound_is_read_and_one_byte_more_is_too_large() {
    let directory = tempfile::tempdir().unwrap();
    let bound = csv_edge::MAX_FILE_BYTES;
    for (size, expected) in [
        (bound, "statement_layout_unknown"),
        (bound + 1, "statement_file_too_large"),
    ] {
        let path = written(directory.path(), "sized.csv", &vec![b'x'; size]);
        let response = call(
            directory.path(),
            json!({"statement_path": path, "controls_source": "prior_statement"}),
        )
        .await;
        assert_eq!(code_of(&response), Some(expected), "{size} bytes");
    }
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
            complete(json!({"statement_path": path, "controls_source": "this_file"})),
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
            "statement_path": absent_in(directory.path()),
            "controls_source": "this_file",
            "password_file": "/synthetic/password"
        }),
    )
    .await;
    assert_eq!(code_of(&response), Some("argument_unknown"));
}
