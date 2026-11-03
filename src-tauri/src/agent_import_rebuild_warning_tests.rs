//! #2152: a build that reads rows of its batch already in the book says that
//! `post_import` would refuse it. The book is the repository's captured window
//! holding one posted Journal; a rebuild under another batch id matches it by
//! content, as the rebuild of a hand-imported file did on the lab book
//! (`lab/1738-rebuild-before-verify-answers` at 42037d89).
use super::amend_tests::{build_plans_reading, simulated_server};
use super::*;

const TXN: &str = "BRIDGE_MCP_LIVE_20260906_A1";

fn captured_window() -> String {
    String::from_utf16(
        &include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/native-namespaced-journal.utf16le.xml")
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn voucher_json(amount: &str) -> Value {
    json!({"bridge_txn_id":TXN,"date":"20260907","voucher_type":"Journal",
        "narration":"Rebuilt from the same statement row",
        "entries":[{"ledger":"Bridge Nested Debtor WR4","amount":amount,"side":"Dr"},
            {"ledger":"Cash","amount":amount,"side":"Cr"}]})
}

/// The build arguments of the captured window's row, to be built again.
fn rebuild_args() -> Value {
    let mut voucher = voucher_json("12.61");
    voucher["date"] = json!("2026-09-07");
    json!({"company_guid":CAPTURED_GUID,"vouchers":[voucher]})
}

/// A saved batch of the captured row under a batch id of its own.
fn rebuilt_line(amount: &str) -> ImportLedgerLine {
    serde_json::from_value(json!({
        "batch_id":"bridge-00000000-0000-4000-8000-000000002152", "identity_scheme":"batch_v1",
        "company_guid":CAPTURED_GUID, "endpoint_origin":"http://127.0.0.1:9001",
        "company":{"name":"WR2 Unicode Lab","guid":CAPTURED_GUID,"company_number":"100004","books_from":"20260401"},
        "txn_ids":[TXN],"date_from":"20260907","date_to":"20260907",
        "sha256":"", "built_at":"2026-11-02T00:00:00Z", "status":"built", "on_account_approved":[],
        "pre_import_mark":{"kind":"company_high_water","value":8,"master_value":7},
        "vouchers":[voucher_json(amount)]
    }))
    .unwrap()
}

fn captured_source() -> ImportReadSource {
    parse_import_vouchers(&captured_window(), CAPTURED_GUID).unwrap()
}

const WARNING: &str = "1 of 1 rows of this batch already match vouchers in the book as this build read it (bridge_txn_id: BRIDGE_MCP_LIVE_20260906_A1). With Allow voucher posting on, post_import would refuse this batch as import_preexisting_identity before any approval, if the book is unchanged. Nothing in ComplyEaze Bridge checks an import of this file against the book either, so do not import it. Rows with the same date, type, ledgers and amounts match the same voucher: count the vouchers in Tally.";

const NEXT_STEP: &str = "Do not post or import this batch yet: the vouchers named in the first warning are already in the book. Open them in Tally and confirm they are regular vouchers and the same transactions as these rows. If they are, leave those rows out and build the rest again. Never change a row's date, ledger, type or amount to get past this check.";

/// The build over the captured window: the warning leads the others, the next
/// step stops the post and the import, the file is still written, and no request
/// beyond the build's own was made.
#[tokio::test]
async fn a_rebuild_of_a_row_the_book_holds_says_post_import_would_refuse_it() {
    let directory = tempfile::tempdir().unwrap();
    let simulator = SequenceSimulator::spawn(build_plans_reading(Some(captured_window()))).unwrap();
    let server = simulated_server(directory.path(), simulator.address().port());
    let built = server.build_import_xml(&rebuild_args()).await.unwrap();
    assert_eq!(simulator.finish().unwrap().len(), QUALIFIED_BUILD_REQUESTS);
    let result = &built.payload["result"];
    assert_eq!(result["warnings"][0], WARNING, "{result}");
    assert!(
        result["warnings"][1]
            .as_str()
            .is_some_and(|text| text.starts_with("No import XML was sent to Tally.")),
        "the guidance still follows: {result}"
    );
    assert_eq!(result["next_step"], NEXT_STEP, "{result}");
    let batch_id = result["batch_id"].as_str().unwrap();
    assert!(server
        .imports_dir()
        .unwrap()
        .join(format!("{batch_id}.xml"))
        .exists());
    assert!(result.get("error").is_none(), "{result}");
}

/// An empty window says nothing: the guidance is the one it was.
#[tokio::test]
async fn a_rebuild_over_an_empty_window_carries_no_warning() {
    let directory = tempfile::tempdir().unwrap();
    let simulator = SequenceSimulator::spawn(build_plans_reading(None)).unwrap();
    let server = simulated_server(directory.path(), simulator.address().port());
    let built = server.build_import_xml(&rebuild_args()).await.unwrap();
    let _ = simulator.finish().unwrap();
    let result = &built.payload["result"];
    let warnings = result["warnings"].to_string();
    assert!(
        !warnings.contains("already match vouchers in the book"),
        "{warnings}"
    );
    assert!(
        result["warnings"][0]
            .as_str()
            .is_some_and(|text| text.starts_with("No import XML was sent to Tally.")),
        "{result}"
    );
    assert!(
        result["next_step"]
            .as_str()
            .is_some_and(|text| text.starts_with("Tell the user that this connection cannot post.")),
        "{result}"
    );
}

/// The rule that decides, on the captured window: a row under a new batch id is
/// found by its content, an amendment and an invoice are not judged, and a row of
/// other figures or an empty window is not found.
#[test]
fn the_rows_found_are_the_posts_rows_and_only_for_a_plain_new_batch() {
    let book = captured_source();
    let line = rebuilt_line("12.61");
    assert_eq!(
        rows_in_book_of_build(&line, &book, false),
        Some(vec![TXN.to_string()])
    );
    // An amendment alters vouchers meant to be in the book.
    assert_eq!(rows_in_book_of_build(&line, &book, true), None);
    // An invoice reads the journal's unsettled twin as well: not judged here,
    // although the same figures under its own type would be found. The book is
    // the captured window with its row read as a Sales voucher, so that the
    // invoice's type does not by itself keep the row from matching.
    let mut rows =
        verification::parse_import_voucher_rows(&captured_window(), CAPTURED_GUID).unwrap();
    for row in &mut rows {
        row.voucher_type = Some("Sales".to_string());
    }
    let sales_book = ImportReadSource::admit(rows).unwrap();
    let mut invoice = line.clone();
    invoice.vouchers[0].voucher_type = VoucherType::Sales;
    assert!(holds_an_invoice(&invoice));
    let found = verify_batch_as(
        &invoice,
        &sales_book,
        Attribution::Tag,
        InvoiceIdentity::ByNumber,
    )
    .unwrap();
    assert_eq!(
        post::rows_already_in_book(&found, 1),
        Some(vec![TXN.to_string()]),
        "the control: without the gate the invoice's row is found"
    );
    assert_eq!(rows_in_book_of_build(&invoice, &sales_book, false), None);
    // Other figures are another voucher; an empty window holds nothing.
    assert_eq!(
        rows_in_book_of_build(&rebuilt_line("12.62"), &book, false),
        None
    );
    let empty = ImportReadSource::admit(Vec::new()).unwrap();
    assert_eq!(rows_in_book_of_build(&line, &empty, false), None);
}

/// More than ten rows: ten are listed, the count is exact, and no amount, ledger
/// or narration is named.
#[test]
fn more_than_ten_rows_are_counted_and_ten_are_listed() {
    let ids = (1..=12)
        .map(|n| format!("st-row-{n:02}"))
        .collect::<Vec<_>>();
    let warning = rows_in_book_warning(&ids, 15);
    assert!(
        warning.starts_with("12 of 15 rows of this batch already match vouchers in the book as this build read it (bridge_txn_id: st-row-01, st-row-02, st-row-03, st-row-04, st-row-05, st-row-06, st-row-07, st-row-08, st-row-09, st-row-10, and 2 more)."),
        "{warning}"
    );
    assert!(!warning.contains("st-row-11"), "{warning}");
    let one = rows_in_book_warning(&ids[..10], 10);
    assert!(one.contains("st-row-10)."), "{one}");
    assert!(!one.contains("more"), "{one}");
}
