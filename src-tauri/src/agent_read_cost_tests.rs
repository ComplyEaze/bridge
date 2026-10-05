//! The read-cost block (bridge#1239): what it states, its boundaries, and where it
//! is added.
use super::*;
use crate::agent::voucher_window::{PartTiming, RequestTally};

fn part(rows: usize, ms: u128) -> PartTiming {
    PartTiming {
        from: "20260801".into(),
        to: "20260801".into(),
        after: None,
        through: None,
        served: true,
        bytes: Some(1),
        rows: Some(rows),
        ms,
    }
}

fn timings(
    marks_ms: u128,
    (census_reads, census_ms): (u64, u128),
    parts: Vec<PartTiming>,
) -> WindowReadTimings {
    WindowReadTimings {
        from: "20260801".into(),
        to: "20260801".into(),
        marks: RequestTally {
            requests: 1,
            ms: marks_ms,
        },
        census: RequestTally {
            requests: census_reads,
            ms: census_ms,
        },
        parts,
        failed: None,
    }
}

/// A weekday of about 760 vouchers on a book with a mark near 1.03 million: 126
/// census reads (bridge#595, 28 Sep 2026; every figure rounded).
fn largest_book_day() -> WindowReadTimings {
    timings(3_400, (126, 145_000), vec![part(760, 21_000)])
}

/// A window on the same book that took 440 s (synthetic figures: a long window).
fn a_long_window() -> WindowReadTimings {
    timings(3_000, (126, 300_000), vec![part(5_000, 137_000)])
}

fn block(timings: &WindowReadTimings) -> Value {
    read_cost(timings, Ended::Read).expect("a block")
}

#[test]
fn a_quick_read_on_a_small_book_says_nothing() {
    // The 29,900-mark synthetic day: 4 census reads, 7.7 s in all.
    let small = timings(1_000, (4, 5_200), vec![part(82, 1_540)]);
    assert_eq!(read_cost(&small, Ended::Read), None);
}

#[test]
fn a_read_of_twenty_seconds_is_worth_a_statement_and_one_millisecond_less_is_not() {
    // One census read leaves the floor at nothing, so only time decides.
    let at = timings(0, (1, 19_000), vec![part(1, 1_000)]);
    assert!(read_cost(&at, Ended::Read).is_some());
    let under = timings(0, (1, 19_000), vec![part(1, 999)]);
    assert_eq!(read_cost(&under, Ended::Read), None);
}

#[test]
fn sixteen_census_reads_are_worth_a_statement_even_when_the_read_stopped() {
    // Sixteen reads wait at least fifteen gaps of half a second: 7.5 s.
    let sixteen = timings(0, (16, 0), vec![]);
    assert!(read_cost(&sixteen, Ended::Stopped).is_some());
    let fifteen = timings(0, (15, 0), vec![]);
    assert_eq!(read_cost(&fifteen, Ended::Stopped), None);
}

#[test]
fn the_largest_book_day_states_what_it_did_and_that_it_fitted() {
    let cost = block(&largest_book_day());
    assert_eq!(cost["ended"], "read");
    assert_eq!(cost["census_reads"], 126);
    assert_eq!(cost["vouchers_read"], 760);
    assert_eq!(
        cost["observed_seconds"],
        json!({"marks": 3, "census": 145, "parts": 21, "total": 169})
    );
    // 125 gaps of 0.5 s, rounded down.
    assert_eq!(cost["floor_seconds"], 62);
    assert_eq!(
        cost["host_240"],
        json!({"state": "window_fits", "vouchers_known_to_fit": 760})
    );
    // Nothing is extrapolated from the one book: no estimate field exists.
    assert!(cost.get("estimate").is_none(), "{cost}");
}

#[test]
fn a_window_past_the_desktop_limit_says_so_and_names_no_number() {
    let cost = block(&a_long_window());
    assert_eq!(cost["observed_seconds"]["total"], 440);
    assert_eq!(cost["host_240"], json!({"state": "window_too_long"}));
}

#[test]
fn a_call_of_exactly_the_limit_fitted_and_a_millisecond_more_did_not() {
    let at_limit = timings(0, (1, 0), vec![part(100, 240_000)]);
    assert_eq!(
        block(&at_limit)["host_240"],
        json!({"state": "window_fits", "vouchers_known_to_fit": 100})
    );
    let over = timings(0, (1, 0), vec![part(100, 240_001)]);
    assert_eq!(
        block(&over)["host_240"],
        json!({"state": "window_too_long"})
    );
}

#[test]
fn a_census_that_alone_passes_the_limit_is_too_long_not_a_verdict_on_the_book() {
    // A smaller window has a smaller census, so this says only that this window
    // did not fit.
    let census_alone = timings(0, (250, 300_000), vec![]);
    assert_eq!(
        block(&census_alone)["host_240"],
        json!({"state": "window_too_long"})
    );
}

#[test]
fn a_read_with_no_voucher_cannot_say_what_a_call_carries() {
    let empty = timings(0, (21, 21_000), vec![]);
    assert_eq!(
        block(&empty)["host_240"],
        json!({"state": "not_established"})
    );
}

/// A read that stopped states the floor and no verdict: the time of a request that
/// failed or hung is not what a window costs.
#[test]
fn a_read_that_stopped_states_no_verdict_whatever_its_times() {
    for stopped in [
        timings(0, (250, 300_000), vec![]),
        timings(0, (5, 250_000), vec![]),
        timings(0, (30, 25_000), vec![part(100, 1_000)]),
    ] {
        let cost = read_cost(&stopped, Ended::Stopped).expect("a block");
        assert_eq!(cost["ended"], "stopped", "{cost}");
        assert_eq!(
            cost["host_240"],
            json!({"state": "not_established"}),
            "{cost}"
        );
    }
}

#[test]
fn the_floor_is_rounded_down_so_it_stays_a_lower_bound() {
    // Four reads leave three gaps: 1.5 s, so the floor says 1, not 2.
    let four = timings(0, (4, 21_000), vec![part(1, 1)]);
    assert_eq!(block(&four)["floor_seconds"], 1);
}

#[test]
fn only_the_served_parts_count_their_vouchers() {
    let mut unserved = part(900, 1_000);
    unserved.served = false;
    let mixed = timings(0, (30, 25_000), vec![part(100, 1_000), unserved]);
    assert_eq!(block(&mixed)["vouchers_read"], 100);
}

#[test]
fn the_host_limits_name_each_host_and_what_is_known() {
    let cost = block(&largest_book_day());
    assert_eq!(
        cost["host_limits"],
        json!([
            {"host": "claude_desktop_chat_macos", "seconds": 240, "basis": "measured_twice_one_build_silent_calls"},
            {"host": "claude_desktop_chat_windows", "seconds": null, "basis": "unmeasured"},
            {"host": "claude_code", "seconds": null, "basis": "completed_150s_by_default_one_run_per_two_builds_one_earlier_60s_unexplained"},
        ])
    );
}

#[test]
fn the_sentence_leads_with_the_outcome_and_labels_what_is_derived() {
    // Pinned whole: assistant-facing text changes on purpose, never by drift.
    assert_eq!(
        block(&largest_book_day())["say"],
        "The window read took 169 seconds for 760 vouchers. This book's census took 126 reads, so at least 62 seconds of any call go on the wait between them (derived from the 0.5 second gate). Claude Desktop's chat app stops a silent call at 240 seconds (measured twice, on one Mac build); a window of up to 760 vouchers is known to fit this book, and whether a larger one does is not established."
    );
    assert_eq!(
        block(&a_long_window())["say"],
        "The window read took 440 seconds for 5000 vouchers. This book's census took 126 reads, so at least 62 seconds of any call go on the wait between them (derived from the 0.5 second gate). That is past 240 seconds, where Claude Desktop's chat app stops a silent call (measured twice, on one Mac build). Read a shorter window; how short is not established. For totals over a long period read trial_balance, which reads no vouchers."
    );
    // One census read has no wait between reads to state.
    assert_eq!(
        block(&timings(0, (1, 25_000), vec![part(10, 1_000)]))["say"],
        "The window read took 26 seconds for 10 vouchers. Claude Desktop's chat app stops a silent call at 240 seconds (measured twice, on one Mac build); a window of up to 10 vouchers is known to fit this book, and whether a larger one does is not established."
    );
    // A window with no voucher is read once more, which is not in the figures.
    assert_eq!(
        block(&timings(0, (21, 21_000), vec![]))["say"],
        "The window read took 21 seconds for 0 vouchers. This book's census took 21 reads, so at least 10 seconds of any call go on the wait between them (derived from the 0.5 second gate). No voucher was read, so what a call can carry is not established. A window with no voucher is also read once more, a day wider on each side, to confirm it is empty; that read pays its own census and is not in these figures."
    );
    let stopped = read_cost(&timings(0, (250, 300_000), vec![]), Ended::Stopped).unwrap();
    assert_eq!(
        stopped["say"],
        "The window read stopped after 300 seconds, before any voucher was read. This book's census took at least 250 reads, so at least 124 seconds of any call go on the wait between them (derived from the 0.5 second gate). The read stopped, so what a call can carry is not established."
    );
    let with_vouchers = read_cost(
        &timings(0, (30, 25_000), vec![part(100, 1_000)]),
        Ended::Stopped,
    )
    .unwrap();
    assert!(
        with_vouchers["say"].as_str().unwrap().starts_with(
            "The window read stopped after 26 seconds with 100 vouchers read and none returned. "
        ),
        "{with_vouchers}"
    );
}

/// The block is added only when the whole response still fits: a result is carried
/// three times over once it is also copied, escaped, into the text content, plus a
/// kilobyte of envelope.
#[test]
fn the_block_is_added_only_when_the_response_still_fits() {
    let day = largest_book_day();
    let block_len = block(&day).to_string().len();
    let container = 5_000;
    let needed = (container + block_len) * 3 + 1_024;
    let window = || serde_json::to_value(&day).unwrap();
    let mut at = window();
    add_read_cost(&mut at, container, &day, Ended::Read, needed);
    assert_eq!(at["read_cost"]["census_reads"], 126);
    let mut under = window();
    add_read_cost(&mut under, container, &day, Ended::Read, needed - 1);
    assert!(under.get("read_cost").is_none(), "{under}");
    // A quick call has nothing to add whatever the budget.
    let quick = timings(1_000, (4, 5_200), vec![part(82, 1_540)]);
    let mut plain = serde_json::to_value(&quick).unwrap();
    add_read_cost(&mut plain, 1, &quick, Ended::Read, usize::MAX);
    assert_eq!(plain, serde_json::to_value(&quick).unwrap());
}

/// The floor is the runtime's own spacing: if that constant moves, this block's
/// "certain" seconds would be wrong, so the two are held together here.
#[test]
fn the_floor_spacing_is_the_runtimes_shipped_spacing() {
    assert_eq!(SPACING_MS, 500);
    assert!(include_str!("tally/runtime_control.rs")
        .contains("const SHIPPED_REQUEST_SPACING: Duration = Duration::from_millis(500);"));
}

fn server() -> (crate::agent::Server, tempfile::TempDir) {
    server_with_budget(200_000)
}

fn server_with_budget(max_bytes: usize) -> (crate::agent::Server, tempfile::TempDir) {
    use crate::agent::{Redaction, Server, Settings, TallyEndpointConfig};
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    (server, directory)
}

/// A refused `vouchers` read reports what it had cost, so the caller learns the
/// census cost of the book from the failure too. The other tools that report a
/// window keep the shape they had.
#[test]
fn a_refused_vouchers_read_reports_its_cost_and_other_tools_keep_their_shape() {
    use crate::agent::ToolFailure;
    let (server, _directory) = server();
    let refusal = || {
        let mut failure = ToolFailure::from("voucher_window_part_not_admitted".to_string());
        failure.window_timings = Some(Box::new(largest_book_day()));
        failure
    };
    let vouchers =
        server.finish_tool_response("vouchers", &json!({}), chrono::Utc::now(), Err(refusal()));
    let window = &vouchers.value["structuredContent"]["result"]["error"]["window"];
    assert_eq!(window["census"]["requests"], 126, "{window}");
    assert_eq!(window["read_cost"]["ended"], "stopped", "{window}");
    assert_eq!(window["read_cost"]["census_reads"], 126, "{window}");
    let outstandings = server.finish_tool_response(
        "outstandings",
        &json!({}),
        chrono::Utc::now(),
        Err(refusal()),
    );
    let window = &outstandings.value["structuredContent"]["result"]["error"]["window"];
    assert_eq!(window["census"]["requests"], 126, "{window}");
    assert!(window.get("read_cost").is_none(), "{window}");
}

/// At a deliberately small response cap the refusal keeps its code and carries
/// the timings it always did, without the block.
#[test]
fn a_refusal_at_a_small_cap_keeps_its_code_and_drops_the_block() {
    use crate::agent::ToolFailure;
    let (server, _directory) = server_with_budget(4_096);
    let mut failure = ToolFailure::from("voucher_window_part_not_admitted".to_string());
    failure.window_timings = Some(Box::new(largest_book_day()));
    let refused =
        server.finish_tool_response("vouchers", &json!({}), chrono::Utc::now(), Err(failure));
    let error = &refused.value["structuredContent"]["result"]["error"];
    assert_eq!(error["code"], "voucher_window_part_not_admitted", "{error}");
    assert_eq!(error["window"]["census"]["requests"], 126, "{error}");
    assert!(error["window"].get("read_cost").is_none(), "{error}");
}
