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

/// A weekday of about 760 vouchers on a book with a mark near 1.03 million: 126
/// census reads (bridge#595, 28 Sep 2026; every figure rounded).
fn largest_book_day() -> WindowReadTimings {
    timings(3_400, (126, 145_000), vec![part(760, 21_000)])
}

/// The same book, a week of about 5,200 vouchers.
fn largest_book_week() -> WindowReadTimings {
    timings(3_400, (126, 308_000), vec![part(5_200, 129_000)])
}

/// A response budget no test here comes near.
const AMPLE: usize = 200_000;

fn block(timings: &WindowReadTimings) -> Value {
    read_cost(timings, Ended::Read, AMPLE).expect("a block")
}

#[test]
fn a_quick_read_on_a_small_book_says_nothing() {
    // The 29,900-mark synthetic day: 4 census reads, 7.7 s in all.
    let small = timings(1_000, (4, 5_200), vec![part(82, 1_540)]);
    assert_eq!(read_cost(&small, Ended::Read, AMPLE), None);
}

#[test]
fn a_read_of_twenty_seconds_is_worth_a_statement_and_one_millisecond_less_is_not() {
    // One census read keeps the floor at half a second, so only time decides.
    let at = timings(0, (1, 19_000), vec![part(1, 1_000)]);
    assert!(read_cost(&at, Ended::Read, AMPLE).is_some());
    let under = timings(0, (1, 19_000), vec![part(1, 999)]);
    assert_eq!(read_cost(&under, Ended::Read, AMPLE), None);
}

#[test]
fn a_read_stopped_early_is_worth_a_statement_from_sixteen_census_reads() {
    // Sixteen reads: a certain floor of eight seconds, whatever the clock says.
    let sixteen = timings(0, (16, 0), vec![]);
    assert!(read_cost(&sixteen, Ended::Stopped, AMPLE).is_some());
    let fifteen = timings(0, (15, 0), vec![]);
    assert_eq!(read_cost(&fifteen, Ended::Stopped, AMPLE), None);
}

#[test]
fn the_largest_book_day_states_its_floor_its_estimate_and_what_one_call_can_carry() {
    let cost = block(&largest_book_day());
    assert_eq!(cost["ended"], "read");
    assert_eq!(cost["census_reads"], 126);
    assert_eq!(cost["vouchers_read"], 760);
    assert_eq!(
        cost["observed_seconds"],
        json!({"marks": 3, "census": 145, "parts": 21, "total": 169})
    );
    // 126 reads x 0.5 s, a lower bound by construction.
    assert_eq!(cost["floor_seconds"], 63);
    let estimate = &cost["estimate"];
    assert_eq!(estimate["kind"], "derived");
    // The call fitted, so the census row cost is taken at 55 ms.
    assert_eq!(estimate["census_ms_per_row"], 55);
    // Census 145.0 s less 760 rows at 55 ms (41.8 s) is 103.2 s, above the
    // 63 s floor; plus 3.4 s of marks: 106.6 s, rounded up.
    assert_eq!(estimate["fixed_seconds"], 107);
    // 21 s over 760 vouchers is 27.6 ms, rounded up, plus 55 ms of census.
    assert_eq!(estimate["per_voucher_ms"], 83);
    // (240,000 - 106,600) / 83 = 1,607.
    assert_eq!(
        estimate["host_240"],
        json!({"state": "window_fits", "vouchers_at_most": 1_607})
    );
}

#[test]
fn a_week_past_the_desktop_limit_says_so_and_names_what_would_fit() {
    let cost = block(&largest_book_week());
    assert_eq!(cost["observed_seconds"]["total"], 440);
    // The call did not fit, so the census row cost is taken at 24 ms: census
    // 308.0 s less 5,200 rows at 24 ms (124.8 s) is 183.2 s, plus 3.4 s of
    // marks: 186.6 s, rounded up.
    assert_eq!(cost["estimate"]["census_ms_per_row"], 24);
    assert_eq!(cost["estimate"]["fixed_seconds"], 187);
    // 129 s over 5,200 vouchers is 24.8 ms, rounded up to 25, plus 24.
    assert_eq!(cost["estimate"]["per_voucher_ms"], 49);
    // (240,000 - 186,600) / 49 = 1,089.
    assert_eq!(
        cost["estimate"]["host_240"],
        json!({"state": "window_too_long", "vouchers_at_most": 1_089})
    );
}

#[test]
fn a_book_whose_census_alone_is_past_the_limit_has_no_window_and_names_none() {
    // 250 census reads taking 300 s, stopped before any voucher: nothing can
    // be read per call on a host that stops at 240 s.
    let stopped = timings(0, (250, 300_000), vec![]);
    let cost = read_cost(&stopped, Ended::Stopped, AMPLE).expect("a block");
    assert_eq!(cost["ended"], "stopped");
    assert_eq!(
        cost["estimate"]["host_240"],
        json!({"state": "no_window_fits"})
    );
    assert_eq!(cost["estimate"]["per_voucher_ms"], Value::Null);
}

#[test]
fn a_call_that_does_not_fit_even_one_voucher_names_no_window() {
    // 100 vouchers read in 1 s (34 ms each with the census row at 24 ms: the
    // call did not fit). A fixed cost of 239,999 ms leaves one millisecond: not
    // one voucher.
    let at_zero = timings(0, (30, 239_999 + 2_400), vec![part(100, 1_000)]);
    assert_eq!(
        block(&at_zero)["estimate"]["host_240"],
        json!({"state": "no_window_fits"})
    );
    // A fixed cost of 239,000 ms leaves 1,000 ms: 29 vouchers at 34 ms.
    let at_twenty_nine = timings(0, (30, 239_000 + 2_400), vec![part(100, 1_000)]);
    assert_eq!(
        block(&at_twenty_nine)["estimate"]["host_240"],
        json!({"state": "window_too_long", "vouchers_at_most": 29})
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
    // model (75 ms each) would allow 2,933, fewer than the call just carried.
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
            {"host": "claude_code", "seconds": null, "basis": "not_cut_at_150s_by_default_one_earlier_60s_unexplained"},
        ])
    );
}

#[test]
fn the_sentence_leads_with_the_outcome_and_labels_what_is_derived() {
    // Pinned whole: assistant-facing text changes on purpose, never by drift.
    assert_eq!(
        block(&largest_book_day())["say"],
        "The window read took 169 seconds for 760 vouchers. About 107 seconds (derived) is paid on every call, however short the window, because this book needs 126 census reads; at least 63 of those seconds are certain (the wait between reads). Claude Desktop's chat app stops a call at 240 seconds (measured once, on one Mac build): one call there can carry about 1607 vouchers (derived), so use the widest window within that, not many short calls."
    );
    assert_eq!(
        block(&largest_book_week())["say"],
        "The window read took 440 seconds for 5200 vouchers. About 187 seconds (derived) is paid on every call, however short the window, because this book needs 126 census reads; at least 63 of those seconds are certain (the wait between reads). That is past 240 seconds, where Claude Desktop's chat app stops a call (measured once, on one Mac build): read about 1089 vouchers or fewer per call (derived). For totals over a long period read trial_balance, which reads no vouchers."
    );
    let stopped = read_cost(&timings(0, (250, 300_000), vec![]), Ended::Stopped, AMPLE).unwrap();
    assert_eq!(
        stopped["say"],
        "The window read stopped after 300 seconds, before any voucher was read. About 300 seconds (derived) is paid on every call, however short the window, because this book needs at least 250 census reads; at least 125 of those seconds are certain (the wait between reads). On a host that stops a call at 240 seconds (Claude Desktop's chat app, measured once on one Mac build) no window of this book fits, because what every call pays is already that long; do not suggest one. For totals over a long period read trial_balance, which reads no vouchers."
    );
}

#[test]
fn a_read_stopped_after_vouchers_were_read_says_none_were_returned() {
    let stopped = read_cost(
        &timings(0, (30, 25_000), vec![part(100, 1_000)]),
        Ended::Stopped,
        AMPLE,
    )
    .expect("a block");
    assert!(
        stopped["say"].as_str().unwrap().starts_with(
            "The window read stopped after 26 seconds with 100 vouchers read and none returned. "
        ),
        "{stopped}"
    );
}

#[test]
fn an_empty_window_says_its_second_read_is_not_counted() {
    // Twenty-one reads and no voucher: the window is then read once more, wider.
    let empty = timings(0, (21, 21_000), vec![]);
    assert_eq!(
        block(&empty)["say"],
        "The window read took 21 seconds for 0 vouchers. About 21 seconds (derived) is paid on every call, however short the window, because this book needs 21 census reads; at least 10 of those seconds are certain (the wait between reads). No voucher was read, so how many one call can carry is not established. A window with no voucher is also read once more, a day wider on each side, to confirm it is empty; that read is not in these figures and costs about as much again (derived)."
    );
    // A read that stopped never reached that second read.
    let stopped = read_cost(&empty, Ended::Stopped, AMPLE).unwrap();
    assert!(
        stopped["say"]
            .as_str()
            .unwrap()
            .ends_with("No voucher was read, so how many one call can carry is not established."),
        "{stopped}"
    );
}

#[test]
fn a_call_of_exactly_the_limit_fitted_and_a_millisecond_more_did_not() {
    // 100 vouchers in parts alone: 240,000 ms fitted (never advised against), and
    // 240,001 ms did not, so the advice is a smaller window.
    let at_limit = timings(0, (1, 0), vec![part(100, 240_000)]);
    assert_eq!(
        block(&at_limit)["estimate"]["host_240"],
        json!({"state": "window_fits", "vouchers_at_most": 100})
    );
    let over = timings(0, (1, 0), vec![part(100, 240_001)]);
    assert_eq!(
        block(&over)["estimate"]["host_240"],
        json!({"state": "window_too_long", "vouchers_at_most": 98})
    );
}

#[test]
fn the_block_is_left_out_past_an_eighth_of_the_response_budget() {
    // Guidance never costs the caller a rows page or a refusal code.
    let day = largest_book_day();
    let size = block(&day).to_string().len();
    assert!(read_cost(&day, Ended::Read, size * 8).is_some());
    assert_eq!(read_cost(&day, Ended::Read, size * 8 - 1), None);
    assert_eq!(read_cost(&day, Ended::Stopped, 4_096), None);
}

#[test]
fn the_window_gets_the_block_only_when_there_is_one() {
    let mut slow = serde_json::to_value(largest_book_day()).unwrap();
    add_read_cost(&mut slow, &largest_book_day(), Ended::Read, AMPLE);
    assert_eq!(slow["census"]["requests"], 126);
    assert_eq!(slow["read_cost"]["census_reads"], 126);
    let quick = timings(1_000, (4, 5_200), vec![part(82, 1_540)]);
    let mut plain = serde_json::to_value(&quick).unwrap();
    add_read_cost(&mut plain, &quick, Ended::Read, AMPLE);
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

/// The advice is bounded by the planner's own allowance as well as by the host's
/// limit: with a fixed cost of nothing and a millisecond a voucher, the host limit
/// alone would allow 240,000 vouchers, far more than one call admits.
#[test]
fn the_advice_never_exceeds_the_planners_allowance() {
    let allowance = voucher_allowance();
    assert!(
        allowance > 0 && allowance < DESKTOP_CALL_LIMIT_MS,
        "{allowance}"
    );
    assert_eq!(
        fit_of(0, Some(1), 5, 100),
        Fit::WindowFits { at_most: allowance }
    );
    assert_eq!(
        fit_of(0, Some(1), 5, DESKTOP_CALL_LIMIT_MS + 1),
        Fit::WindowTooLong { at_most: allowance }
    );
}

fn server() -> (crate::agent::Server, tempfile::TempDir) {
    server_with_budget(AMPLE)
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

#[test]
fn what_every_call_pays_is_never_less_than_the_certain_floor() {
    // 30 census reads (a certain 15 s) that took 20 s while carrying 500 rows:
    // the rows' share (27.5 s at 55 ms) would leave nothing, below the floor, so
    // the floor stands.
    let rows_heavy = timings(0, (30, 20_000), vec![part(500, 1_000)]);
    assert_eq!(block(&rows_heavy)["estimate"]["fixed_seconds"], 15);
}

#[test]
fn a_census_that_alone_takes_the_whole_limit_leaves_no_window() {
    // 250 reads, 240.000 s exactly, no voucher read: what every call pays is the
    // limit itself.
    let stopped = timings(0, (250, 240_000), vec![]);
    let cost = read_cost(&stopped, Ended::Stopped, AMPLE).expect("a block");
    assert_eq!(
        cost["estimate"]["host_240"],
        json!({"state": "no_window_fits"})
    );
}
