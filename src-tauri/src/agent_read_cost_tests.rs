//! The read-cost block (bridge#1239): the figures, the boundaries, and what each
//! one is labelled.
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

/// One weekday of 757 vouchers on the book with a mark near 1.03 million: 126
/// census reads (bridge#595, 28 Sep 2026; marks and the rest rounded to 3.4 s).
fn largest_book_day() -> WindowReadTimings {
    timings(3_400, (126, 144_500), vec![part(757, 21_000)])
}

/// The same book, a week of 5,178 vouchers: census 307.6 s, parts 129.1 s.
fn largest_book_week() -> WindowReadTimings {
    timings(3_400, (126, 307_600), vec![part(5_178, 129_100)])
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
    // One census read keeps the floor at half a second, so only time decides.
    let at = timings(0, (1, 19_000), vec![part(1, 1_000)]);
    assert!(read_cost(&at, Ended::Read).is_some());
    let under = timings(0, (1, 19_000), vec![part(1, 999)]);
    assert_eq!(read_cost(&under, Ended::Read), None);
}

#[test]
fn a_read_stopped_early_is_worth_a_statement_from_sixteen_census_reads() {
    // Sixteen reads: a certain floor of eight seconds, whatever the clock says.
    let sixteen = timings(0, (16, 0), vec![]);
    assert!(read_cost(&sixteen, Ended::Stopped).is_some());
    let fifteen = timings(0, (15, 0), vec![]);
    assert_eq!(read_cost(&fifteen, Ended::Stopped), None);
}

#[test]
fn the_largest_book_day_states_its_floor_its_estimate_and_what_one_call_can_carry() {
    let cost = block(&largest_book_day());
    assert_eq!(cost["ended"], "read");
    assert_eq!(cost["census_reads"], 126);
    assert_eq!(cost["vouchers_read"], 757);
    assert_eq!(
        cost["observed_seconds"],
        json!({"marks": 3, "census": 145, "parts": 21, "total": 169})
    );
    // 126 reads x 0.5 s, a floor by construction.
    assert_eq!(cost["floor_seconds"], 63);
    let estimate = &cost["estimate"];
    assert_eq!(estimate["kind"], "derived");
    // Census 144.5 s less 757 rows at 37 ms (28.0 s) is 116.5 s, above the
    // 63 s floor; plus 3.4 s of marks: 119.9 s, rounded up.
    assert_eq!(estimate["fixed_seconds"], 120);
    // 21 s over 757 vouchers is 27.7 ms, rounded up, plus 37 ms of census.
    assert_eq!(estimate["per_voucher_ms"], 65);
    // (240,000 - 119,891) / 65 = 1,847.
    assert_eq!(
        estimate["host_240"],
        json!({"state": "window_fits", "vouchers_at_most": 1_847})
    );
}

#[test]
fn a_week_past_the_desktop_limit_says_so_and_names_what_would_fit() {
    let cost = block(&largest_book_week());
    assert_eq!(cost["observed_seconds"]["total"], 440);
    // Fixed 119.4 s; 129.1 s over 5,178 is 24.9 ms, rounded up to 25, plus 37.
    assert_eq!(cost["estimate"]["fixed_seconds"], 120);
    assert_eq!(cost["estimate"]["per_voucher_ms"], 62);
    assert_eq!(
        cost["estimate"]["host_240"],
        json!({"state": "window_too_long", "vouchers_at_most": 1_944})
    );
}

#[test]
fn a_book_whose_census_alone_is_past_the_limit_has_no_window_and_names_none() {
    // 250 census reads taking 300 s, stopped before any voucher: nothing can
    // be read per call on a host that stops at 240 s.
    let stopped = timings(0, (250, 300_000), vec![]);
    let cost = read_cost(&stopped, Ended::Stopped).expect("a block");
    assert_eq!(cost["ended"], "stopped");
    assert_eq!(
        cost["estimate"]["host_240"],
        json!({"state": "no_window_fits"})
    );
    assert_eq!(cost["estimate"]["per_voucher_ms"], Value::Null);
}

#[test]
fn a_call_that_does_not_fit_even_one_voucher_names_no_window() {
    // 100 vouchers read in 1 s (47 ms each with the census row). A fixed cost
    // of 239,999 ms leaves one millisecond: not one voucher.
    let at_zero = timings(0, (30, 239_999 + 3_700), vec![part(100, 1_000)]);
    assert_eq!(
        block(&at_zero)["estimate"]["host_240"],
        json!({"state": "no_window_fits"})
    );
    // A fixed cost of 239,000 ms leaves 1,000 ms: 21 vouchers at 47 ms.
    let at_twenty_one = timings(0, (30, 239_000 + 3_700), vec![part(100, 1_000)]);
    assert_eq!(
        block(&at_twenty_one)["estimate"]["host_240"],
        json!({"state": "window_too_long", "vouchers_at_most": 21})
    );
}

#[test]
fn a_read_with_no_voucher_cannot_say_how_many_a_call_carries() {
    // Twenty-one reads, a clean read of an empty window: the floor is long
    // enough to speak, the rate is unknown.
    let empty = timings(0, (21, 21_000), vec![]);
    let cost = block(&empty);
    assert_eq!(
        cost["estimate"]["host_240"],
        json!({"state": "not_established"})
    );
    assert_eq!(cost["estimate"]["per_voucher_ms"], Value::Null);
}

#[test]
fn a_read_that_fitted_is_never_advised_against() {
    // 5,000 vouchers in 100 s of parts at a 20 s fixed census: the cautious
    // model (57 ms each) would allow 3,859, fewer than the call just carried.
    let fitted = timings(0, (40, 20_000), vec![part(5_000, 100_000)]);
    assert_eq!(
        block(&fitted)["estimate"]["host_240"],
        json!({"state": "window_fits", "vouchers_at_most": 5_000})
    );
}

#[test]
fn the_floor_is_rounded_down_so_it_stays_a_lower_bound() {
    // Three reads are 1.5 s: the floor says 1, not 2. (Slow enough to speak.)
    let three = timings(0, (3, 21_000), vec![part(1, 1)]);
    assert_eq!(block(&three)["floor_seconds"], 1);
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
            {"host": "claude_desktop_chat_macos", "seconds": 240, "basis": "measured_once_one_build"},
            {"host": "claude_desktop_chat_windows", "seconds": null, "basis": "unmeasured"},
            {"host": "claude_code", "seconds": null, "basis": "none_by_default_user_can_set"},
        ])
    );
}

#[test]
fn the_sentence_leads_with_the_outcome_and_labels_what_is_derived() {
    // Pinned whole: assistant-facing text changes on purpose, never by drift.
    assert_eq!(
        block(&largest_book_day())["say"],
        "This read took 169 seconds for 757 vouchers. About 120 seconds (derived) is paid on every call, however short the window, because this book needs 126 census reads; at least 63 of those seconds are certain. Claude Desktop's chat app stops a call at 240 seconds (measured once, on one Mac build): one call there can carry about 1847 vouchers (derived), so use the widest window within that, not many short calls."
    );
    assert_eq!(
        read_cost(&largest_book_week(), Ended::Read).unwrap()["say"],
        "This read took 440 seconds for 5178 vouchers. About 120 seconds (derived) is paid on every call, however short the window, because this book needs 126 census reads; at least 63 of those seconds are certain. That is past 240 seconds, where Claude Desktop's chat app stops a call (measured once, on one Mac build): read about 1944 vouchers or fewer per call (derived). For totals over a long period read trial_balance, which reads no vouchers."
    );
    let stopped = read_cost(&timings(0, (250, 300_000), vec![]), Ended::Stopped).unwrap();
    assert_eq!(
        stopped["say"],
        "This read stopped after 300 seconds with 0 vouchers read. About 300 seconds (derived) is paid on every call, however short the window, because this book needs 250 census reads; at least 125 of those seconds are certain. On a host that stops a call at 240 seconds (Claude Desktop's chat app, measured once on one Mac build) no window of this book fits, because what every call pays is already that long; do not suggest one. For totals over a long period read trial_balance, which reads no vouchers."
    );
}

#[test]
fn the_window_value_carries_the_block_only_when_there_is_one() {
    let slow = window_value(&largest_book_day(), Ended::Read);
    assert_eq!(slow["census"]["requests"], 126);
    assert_eq!(slow["read_cost"]["census_reads"], 126);
    let quick = timings(1_000, (4, 5_200), vec![part(82, 1_540)]);
    let plain = window_value(&quick, Ended::Read);
    assert_eq!(plain, serde_json::to_value(&quick).unwrap());
    assert!(plain.get("read_cost").is_none());
}

/// The floor is the runtime's own spacing: if that constant moves, this block's
/// "certain" seconds would be wrong, so the two are held together here.
#[test]
fn the_floor_spacing_is_the_runtimes_shipped_spacing() {
    assert_eq!(SPACING_MS, 500);
    assert!(include_str!("tally/runtime_control.rs")
        .contains("const SHIPPED_REQUEST_SPACING: Duration = Duration::from_millis(500);"));
}

/// The most vouchers the model can advise is below what the planner's own
/// allowance admits, so the allowance never needs applying in the block.
#[test]
fn the_model_never_advises_more_vouchers_than_the_planners_allowance_admits() {
    use crate::agent::voucher_window::{
        VoucherReadShape, MAX_PLANNED_READS, WINDOW_READ_BUDGET_BYTES,
    };
    let smallest_part = VoucherReadShape::EntryWildcard.default_wire_bytes_per_voucher() / 2;
    let allowance = (WINDOW_READ_BUDGET_BYTES / smallest_part) * MAX_PLANNED_READS as u64;
    // The fastest rate the model can state is the census row term plus 1 ms.
    let most_advised = DESKTOP_CALL_LIMIT_MS / (CENSUS_MS_PER_ROW + 1);
    assert!(most_advised < allowance, "{most_advised} vs {allowance}");
}

fn server() -> (crate::agent::Server, tempfile::TempDir) {
    use crate::agent::{Redaction, Server, Settings, TallyEndpointConfig};
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: 9,
        },
        data_dir: directory.path().into(),
        max_rows: 500,
        max_bytes: 200_000,
        redaction: Redaction::None,
        import_enabled: false,
        writes_enabled: false,
        batch_post_enabled: false,
    });
    (server, directory)
}

/// A refused `vouchers` read reports what it had cost, so the caller learns the
/// fixed cost of the book from the failure too. The other tools that report a
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
