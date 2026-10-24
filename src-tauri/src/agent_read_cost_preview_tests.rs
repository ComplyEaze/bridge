//! `voucher_read_cost` through `call_tool`, against the simulator replaying the
//! reads of a whole call: the identity read (four requests) and one paired,
//! identity-bracketed marks read (six). The marks are the live capture of the
//! Company collection (`native-company-book-extents-with-number`) with the
//! requested company's voucher mark rewritten, so the parser reads a real shape.
//! The census counts are pinned to counts measured live: a mark of about 250,000
//! is 31 census reads (protocol reference 11c.5).
use super::super::*;
use super::{measured, CensusCost};
use crate::agent::read_cost::census_floor_ms;
use crate::agent::voucher_window::{
    census_read_count, VoucherReadShape, WindowReadLimits, BOOK_TOO_LARGE, MAX_CENSUS_READS,
};
use tally_protocol_simulator::{
    Fixture, ProductStatus, ResponseFraming, ScenarioPlan, SequenceSimulator, WireEncoding,
};

/// `Aarav Trading Company Demo`, the first row of the captured marks.
const GUID: &str = "bb8ad19e-6aef-4239-a917-87fec0c6215e";
/// That row's captured voucher mark, as the capture spells it.
const CAPTURED_MARK: &str = "<ALTVCHID TYPE=\"Number\"> 101605</ALTVCHID>";

fn decode(bytes: &[u8]) -> String {
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

fn xml(text: String) -> ScenarioPlan {
    ScenarioPlan::new(Fixture::SyntheticXml(text))
        .with_encoding(WireEncoding::Utf16Le)
        .with_framing(ResponseFraming::ContentLength)
}

fn company_plan() -> ScenarioPlan {
    xml(decode(include_bytes!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-companies.utf16le.xml"
    )))
}

fn status_plan() -> ScenarioPlan {
    ScenarioPlan::new(Fixture::ProductStatus(ProductStatus::TallyPrime))
        .with_framing(ResponseFraming::ContentLength)
}

fn captured_marks() -> String {
    let captured = include_str!(
        "../crates/bridge-tally-protocol/tests/fixtures/agent/native-company-book-extents-with-number.utf8.xml"
    )
    .to_string();
    assert_eq!(captured.matches(CAPTURED_MARK).count(), 1);
    captured
}

/// The captured marks with the requested company's voucher mark set to `mark`.
fn marks_with(mark: u64) -> String {
    let rewritten = captured_marks().replacen(
        CAPTURED_MARK,
        &format!("<ALTVCHID TYPE=\"Number\"> {mark}</ALTVCHID>"),
        1,
    );
    assert!(rewritten.contains(&format!("> {mark}</ALTVCHID>")));
    rewritten
}

/// The identity read every call opens with, then one paired marks read.
fn plans(marks: &str) -> Vec<ScenarioPlan> {
    let marks = xml(marks.to_string());
    vec![
        company_plan(),
        status_plan(),
        company_plan(),
        status_plan(),
        company_plan(),
        marks.clone(),
        status_plan(),
        marks,
        status_plan(),
        company_plan(),
    ]
}

fn server_at(port: u16, directory: &std::path::Path) -> Server {
    Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port,
        },
        data_dir: directory.into(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        posting: Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    })
}

/// The response and the requests the simulator saw, each as method, path and
/// body fingerprint, in order.
async fn run(plans: Vec<ScenarioPlan>, tool: &str, args: Value) -> (Value, Vec<String>) {
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = server_at(simulator.address().port(), directory.path());
    let response = server.call_tool(tool, args).await;
    let seen = simulator
        .finish()
        .unwrap()
        .into_iter()
        .filter(|request| !request.method.is_empty())
        .map(|request| {
            format!(
                "{} {} {}",
                request.method, request.path, request.request_body_sha256
            )
        })
        .collect();
    (response, seen)
}

async fn preview(marks: &str) -> (Value, Vec<String>) {
    run(
        plans(marks),
        "voucher_read_cost",
        json!({"company_guid": GUID}),
    )
    .await
}

fn result(response: &Value) -> &Value {
    assert_ne!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]
}

fn error(response: &Value) -> &Value {
    assert_eq!(response["isError"], true, "{response}");
    &response["structuredContent"]["result"]["error"]
}

const TOOLS: [&str; 3] = ["vouchers", "ledger_movement", "verify_import"];

/// (mark, census reads for `vouchers`, for `ledger_movement` and `verify_import`).
/// `vouchers` counts a small book (#1314); the other two skip the census when the
/// mark bounds the book: 170 x 96 KiB fits the 16 MiB budget, 171 does not.
const COUNTS: [(u64, u64, u64); 12] = [
    (1, 1, 0),
    (170, 1, 0),
    (171, 1, 1),
    (8_192, 1, 1),
    (8_193, 2, 2),
    (101_605, 13, 13),
    (249_948, 31, 31),
    (1_030_000, 126, 126),
    (2_097_151, 256, 256),
    (2_097_152, 256, 256),
    (4_096, 1, 1),
    (42, 1, 0),
];

#[tokio::test]
async fn each_tool_states_the_census_reads_its_own_read_would_send() {
    for (mark, vouchers, others) in COUNTS {
        let (response, seen) = preview(&marks_with(mark)).await;
        let result = result(&response);
        assert_eq!(result["voucher_mark"], mark, "{result}");
        assert_eq!(
            seen.len(),
            10,
            "mark {mark}: the identity read and one marks read"
        );
        for (tool, expected) in TOOLS.iter().zip([vouchers, others, others]) {
            let entry = &result["per_call"][*tool];
            assert_eq!(entry["state"], "counted", "{tool} at {mark}: {entry}");
            assert_eq!(entry["census_reads"], expected, "{tool} at {mark}: {entry}");
            // The floor is (N - 1) x 0.5 s, rounded down: 31 reads is 15 s.
            assert_eq!(
                entry["floor_seconds"],
                expected.saturating_sub(1) / 2,
                "{tool} at {mark}: {entry}"
            );
        }
    }
}

#[tokio::test]
async fn a_book_that_never_held_a_voucher_sends_no_census_and_states_zero_only_then() {
    // The live never-held-a-voucher shape (#550): the mark is omitted, ALTMSTID kept.
    let never_held = captured_marks().replacen(CAPTURED_MARK, "", 1);
    assert_ne!(never_held, captured_marks());
    let (response, _) = preview(&never_held).await;
    let result = result(&response);
    assert_eq!(result["voucher_mark"], 0, "{result}");
    for tool in TOOLS {
        assert_eq!(result["per_call"][tool]["census_reads"], 0, "{tool}");
        assert_eq!(result["per_call"][tool]["floor_seconds"], 0, "{tool}");
    }
}

#[tokio::test]
async fn a_mark_that_cannot_be_read_is_an_error_with_no_figure_never_zero() {
    // Both marks removed from the requested company's row: the same refusal the
    // windowed read gives, at the same read, by its typed code.
    let neither = captured_marks().replacen(CAPTURED_MARK, "", 1).replacen(
        "<ALTMSTID TYPE=\"Number\"> 328</ALTMSTID>",
        "",
        1,
    );
    let (response, _) = preview(&neither).await;
    assert_eq!(error(&response)["code"], "master_checkpoint_not_observed");
    let text = response.to_string();
    for absent in ["census_reads", "floor_seconds", "per_call", "measured"] {
        assert!(!text.contains(absent), "{absent} in {text}");
    }
}

#[tokio::test]
async fn the_floor_is_its_own_derived_figure_and_there_is_no_estimate() {
    let (response, _) = preview(&marks_with(1_030_000)).await;
    let result = result(&response);
    // 126 reads leave 125 gaps of 0.5 s: 62.5 s, stated as 62. A figure that took
    // the measured seconds per read for the floor would not equal this.
    assert_eq!(
        result["per_call"]["vouchers"],
        json!({"state": "counted", "census_reads": 126, "floor_seconds": 62, "floor_basis": "derived"})
    );
    assert_eq!(census_floor_ms(126), 62_500);
    assert_eq!(census_floor_ms(0), 0);
    assert_eq!(census_floor_ms(1), 0);
    assert_eq!(census_floor_ms(2), 500);
    let keys: Vec<_> = result.as_object().unwrap().keys().cloned().collect();
    for key in &keys {
        assert!(!key.contains("estimate"), "an estimate field: {key}");
    }
    for tool in TOOLS {
        let entry = result["per_call"][tool].as_object().unwrap();
        assert!(
            entry.keys().all(|key| !key.contains("estimate")),
            "{entry:?}"
        );
    }
    assert_eq!(result["request_spacing_ms"], 500);
}

#[tokio::test]
async fn the_only_refusal_it_states_is_the_one_the_read_makes_from_the_same_function() {
    let limits = WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard);
    let largest = MAX_CENSUS_READS as u64 * limits.census_capacity();
    assert_eq!(largest, 2_097_152);
    let (response, _) = preview(&marks_with(largest + 1)).await;
    // The preview itself answers: nothing is refused on it.
    let result = result(&response);
    for tool in TOOLS {
        let entry = &result["per_call"][tool];
        assert_eq!(entry["state"], "refused_by_allowance", "{tool}: {entry}");
        assert_eq!(entry["code"], BOOK_TOO_LARGE, "{tool}: {entry}");
        assert_eq!(entry["limit_census_reads"], MAX_CENSUS_READS, "{tool}");
        // A read refused before any census has no floor to state.
        assert!(entry.get("floor_seconds").is_none(), "{tool}: {entry}");
        assert!(entry.get("census_reads").is_none(), "{tool}: {entry}");
    }
    // The same function the read uses, on the same limits.
    assert_eq!(census_read_count(largest + 1, limits), Err(BOOK_TOO_LARGE));
    assert_eq!(census_read_count(largest, limits), Ok(256));
    assert_eq!(
        CensusCost::of(largest + 1, limits),
        CensusCost::Refused(BOOK_TOO_LARGE)
    );
}

/// The Go on #1465: the preview's requests are the opening of the real call, by
/// structure. A `vouchers` call on a book too large to count stops right after the
/// identity read and the marks read it opens with, before any census, so its
/// requests are the whole of what the preview sends.
#[tokio::test]
async fn the_preview_sends_exactly_the_opening_of_the_real_call_and_no_census() {
    let too_large = marks_with(2_097_153);
    let (preview_response, preview_seen) = preview(&too_large).await;
    result(&preview_response);
    let (vouchers_response, vouchers_seen) = run(
        plans(&too_large),
        "vouchers",
        json!({"company_guid": GUID, "from": "2026-04-01", "to": "2026-04-30"}),
    )
    .await;
    assert_eq!(
        error(&vouchers_response)["code"],
        BOOK_TOO_LARGE,
        "{vouchers_response}"
    );
    assert_eq!(preview_seen.len(), 10);
    assert_eq!(preview_seen, vouchers_seen);

    // On a book that is counted the same ten requests are all it sends: the call
    // that follows would go on to a census, the preview never does.
    let (response, seen) = preview(&marks_with(249_948)).await;
    result(&response);
    assert_eq!(seen, preview_seen, "the requests do not depend on the mark");
}

#[tokio::test]
async fn the_measured_block_is_constant_text_with_its_markers() {
    let (small, _) = preview(&marks_with(8_193)).await;
    let (large, _) = preview(&marks_with(1_030_000)).await;
    // Not a function of the mark: the same data whatever the book.
    assert_eq!(result(&small)["measured"], measured());
    assert_eq!(result(&large)["measured"], measured());
    let block = measured();
    let points = block["points"].as_array().unwrap();
    assert_eq!(points.len(), 2);
    for point in points {
        for key in [
            "call",
            "window",
            "voucher_mark",
            "census_reads",
            "data_parts",
            "seconds",
            "book",
            "host",
            "date",
            "confidence",
            "source",
            "not_a_bound",
        ] {
            assert!(point.get(key).is_some(), "{key} in {point}");
        }
        assert_eq!(point["not_a_bound"], true);
        assert_eq!(point["confidence"], "verified");
        assert_eq!(point["date"], "2026-09-21");
        assert_eq!(point["source"], "protocol reference 11c.5");
        // The book's mark is rounded: it is not a lab company.
        assert_eq!(point["voucher_mark"], "about 250,000");
        assert_eq!(point["book"], "an inventory-heavy book");
    }
    assert_eq!(block["kind"], "measured_calls_not_a_bound");
    // The figures are the ones protocol reference 11c.5 records: one-day `vouchers`
    // 34.3 s (31 census spans, 2 data parts), one-month `ledger_movement` 104.7 s
    // (31, 5). This test does not read that document: it is outside the native
    // job's scope, so an edit to it would never run this test (the consistency
    // check refuses the include); the block's `source` names the section.
    assert_eq!(points[0]["seconds"], 34.3);
    assert_eq!(points[0]["census_reads"], 31);
    assert_eq!(points[0]["data_parts"], 2);
    assert_eq!(points[1]["seconds"], 104.7);
    assert_eq!(points[1]["census_reads"], 31);
    assert_eq!(points[1]["data_parts"], 5);
}

#[tokio::test]
async fn the_answer_says_it_is_for_one_call_and_the_host_limits_are_the_read_cost_blocks() {
    let (response, _) = preview(&marks_with(1_030_000)).await;
    let result = result(&response);
    let say = result["say"].as_str().unwrap();
    assert!(say.contains("one call"), "{say}");
    assert!(say.contains("a narrower window does not lower it"), "{say}");
    assert!(say.contains("as of this read of the mark"), "{say}");
    assert!(say.contains("Nothing is refused on it"), "{say}");
    assert!(say.contains("126 census reads"), "{say}");
    assert_eq!(
        result["host_limits"],
        crate::agent::read_cost::host_limits()
    );
    assert_eq!(result["basis"], "voucher_mark_read_by_this_call");
}

#[tokio::test]
async fn it_takes_no_window() {
    // A refused argument sends nothing: the endpoint is one nothing listens on.
    let directory = tempfile::tempdir().unwrap();
    let response = server_at(9, directory.path())
        .call_tool(
            "voucher_read_cost",
            json!({"company_guid": GUID, "from": "2026-04-01", "to": "2026-04-30"}),
        )
        .await;
    assert_eq!(error(&response)["code"], "argument_unknown", "{response}");
}
