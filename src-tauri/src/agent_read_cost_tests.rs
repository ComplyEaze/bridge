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
    // The central model, at the measured 37 ms a census row: census 145.0 s less
    // 760 rows (28.1 s) is 116.9 s, above the 63 s floor; plus 3.4 s of marks:
    // 120.3 s, rounded up. One more voucher: 21 s over 760 is 27.6 ms, rounded up
    // to 28, plus 37.
    assert_eq!(estimate["fixed_seconds"], 121);
    assert_eq!(estimate["per_voucher_ms"], 65);
    // The advice: the call fitted, so the census row is taken at 55 ms: 145.0 s
    // less 41.8 s is 103.2 s, plus 3.4 s of marks is 106.6 s; 28 + 55 = 83 ms a
    // voucher; (225,000 - 106,600) / 83 = 1,426.
    assert_eq!(
        estimate["host_240"],
        json!({"state": "window_fits", "vouchers_at_most": 1_426,
               "basis": {"census_ms_per_row": 55, "fixed_ms": 106_600, "per_voucher_ms": 83, "planning_limit_ms": 225_000}})
    );
}

#[test]
fn a_week_past_the_desktop_limit_says_so_and_names_what_would_fit() {
    let cost = block(&largest_book_week());
    assert_eq!(cost["observed_seconds"]["total"], 440);
    // The same book gives the same central model from the week as from the day:
    // 308.0 s less 5,200 rows at 37 ms (192.4 s) is 115.6 s, plus 3.4 s of marks,
    // 119.0 s (the day gave 120.3 s); 129 s over 5,200 is 24.8 ms, 25, plus 37.
    assert_eq!(cost["estimate"]["fixed_seconds"], 119);
    assert_eq!(cost["estimate"]["per_voucher_ms"], 62);
    // The advice: the call did not fit, so the census row is taken at 24 ms:
    // 308.0 s less 124.8 s is 183.2 s, plus 3.4 s is 186.6 s; 25 + 24 = 49 ms a
    // voucher; (225,000 - 186,600) / 49 = 783.
    assert_eq!(
        cost["estimate"]["host_240"],
        json!({"state": "window_too_long", "vouchers_at_most": 783,
               "basis": {"census_ms_per_row": 24, "fixed_ms": 186_600, "per_voucher_ms": 49, "planning_limit_ms": 225_000}})
    );
}

#[test]
fn a_book_whose_census_alone_is_past_the_limit_has_no_window_and_names_none() {
    // 250 census reads taking 300 s and no voucher read, finished: nothing can be
    // read per call on a host that stops at 240 s.
    let read = timings(0, (250, 300_000), vec![]);
    let cost = block(&read);
    assert_eq!(cost["ended"], "read");
    assert_eq!(
        cost["estimate"]["host_240"],
        json!({"state": "no_window_fits"})
    );
    assert_eq!(cost["estimate"]["per_voucher_ms"], Value::Null);
}

/// A read that stopped gives the certain floor and no estimate: the time of a
/// request that failed or hung is not what a window costs, so five census reads
/// with 250 s of failed time never become a verdict about the book.
#[test]
fn a_read_that_stopped_states_no_estimate_whatever_its_times() {
    for stopped in [
        timings(0, (250, 300_000), vec![]),
        timings(0, (5, 250_000), vec![]),
        timings(0, (30, 25_000), vec![part(100, 1_000)]),
    ] {
        let cost = read_cost(&stopped, Ended::Stopped, AMPLE).expect("a block");
        assert_eq!(cost["ended"], "stopped", "{cost}");
        assert_eq!(
            cost["estimate"]["host_240"],
            json!({"state": "not_established"}),
            "{cost}"
        );
        assert_eq!(cost["estimate"]["fixed_seconds"], Value::Null, "{cost}");
        assert_eq!(cost["estimate"]["per_voucher_ms"], Value::Null, "{cost}");
    }
}

#[test]
fn a_call_that_does_not_fit_even_one_voucher_names_no_window() {
    // 100 vouchers read in 20 s of parts (224 ms each with the census row at 24
    // ms: the call did not fit). A fixed cost of 224,900 ms leaves 100 ms of the
    // planning limit: not one voucher, and the figures cannot say how many fit.
    let none = timings(0, (30, 224_900 + 2_400), vec![part(100, 20_000)]);
    let cost = block(&none);
    assert_eq!(cost["estimate"]["host_240"]["state"], "window_too_long");
    assert!(
        cost["estimate"]["host_240"]
            .get("vouchers_at_most")
            .is_none(),
        "{cost}"
    );
    // A fixed cost of 224,000 ms leaves 1,000 ms: 4 vouchers at 224 ms.
    let four = timings(0, (30, 224_000 + 2_400), vec![part(100, 20_000)]);
    assert_eq!(block(&four)["estimate"]["host_240"]["vouchers_at_most"], 4);
}

/// What every call pays is at the planning limit or above only on the estimate
/// most favourable to a window (the census row at 55 ms): at exactly that, no
/// window fits; a millisecond less and what fitted is never advised against.
#[test]
fn no_window_fits_only_when_even_the_favourable_estimate_says_so() {
    let at_limit = timings(0, (30, 230_500), vec![part(100, 1_000)]);
    assert_eq!(
        block(&at_limit)["estimate"]["host_240"],
        json!({"state": "no_window_fits"})
    );
    let under = timings(0, (30, 230_499), vec![part(100, 1_000)]);
    assert_eq!(
        block(&under)["estimate"]["host_240"]["state"],
        "window_fits"
    );
    assert_eq!(
        block(&under)["estimate"]["host_240"]["vouchers_at_most"],
        100
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
    // model (75 ms each) would allow 2,733, fewer than the call just carried.
    let fitted = timings(0, (40, 20_000), vec![part(5_000, 100_000)]);
    let host = &block(&fitted)["estimate"]["host_240"];
    assert_eq!(host["state"], "window_fits");
    assert_eq!(host["vouchers_at_most"], 5_000);
    // The cautious figures themselves are shown: 20 ms + 55 ms a voucher, and the
    // census (20 s) is all fixed cost.
    assert_eq!(host["basis"]["per_voucher_ms"], 75);
    assert_eq!(host["basis"]["fixed_ms"], 20_000);
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
        "The window read took 169 seconds for 760 vouchers. About 121 seconds (derived) is paid on every call, however short the window, because this book needs 126 census reads; at least 63 of those seconds are certain (the wait between reads). Claude Desktop's chat app stops a silent call at 240 seconds (measured twice, on one Mac build): with 15 seconds kept back for the call's other reads, one call there can carry about 1426 vouchers (derived), so use the widest window within that, not many short calls."
    );
    assert_eq!(
        block(&largest_book_week())["say"],
        "The window read took 440 seconds for 5200 vouchers. About 119 seconds (derived) is paid on every call, however short the window, because this book needs 126 census reads; at least 63 of those seconds are certain (the wait between reads). That is past 240 seconds, where Claude Desktop's chat app stops a silent call (measured twice, on one Mac build): read about 783 vouchers or fewer per call (derived). For totals over a long period read trial_balance, which reads no vouchers."
    );
    assert_eq!(
        block(&timings(0, (250, 300_000), vec![]))["say"],
        "The window read took 300 seconds for 0 vouchers. About 300 seconds (derived) is paid on every call, however short the window, because this book needs 250 census reads; at least 125 of those seconds are certain (the wait between reads). On a host that stops a silent call at 240 seconds (Claude Desktop's chat app, measured twice on one Mac build) no window of this book fits, because what every call pays is already that long; do not suggest one. For totals over a long period read trial_balance, which reads no vouchers."
    );
    let none = timings(0, (30, 224_900 + 2_400), vec![part(100, 20_000)]);
    assert!(
        block(&none)["say"].as_str().unwrap().ends_with(
            "That is past 240 seconds, where Claude Desktop's chat app stops a silent call (measured twice, on one Mac build), and the figures cannot say how many vouchers a call can carry: read a shorter window, or for totals over a long period read trial_balance, which reads no vouchers."
        ),
        "{}", block(&none)
    );
    let stopped = read_cost(&timings(0, (250, 300_000), vec![]), Ended::Stopped, AMPLE).unwrap();
    assert_eq!(
        stopped["say"],
        "The window read stopped after 300 seconds, before any voucher was read. At least 250 census reads were sent, so at least 125 seconds is paid on every call (the wait between reads). The read stopped, so how many vouchers one call can carry is not established."
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
    assert_eq!(stopped["ended"], "stopped");
    assert_eq!(stopped["vouchers_read"], 100);
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
        stopped["say"].as_str().unwrap().ends_with(
            "The read stopped, so how many vouchers one call can carry is not established."
        ),
        "{stopped}"
    );
}

#[test]
fn a_call_of_exactly_the_limit_fitted_and_a_millisecond_more_did_not() {
    // 100 vouchers in parts alone: 240,000 ms fitted (never advised against), and
    // 240,001 ms did not, so the advice is a smaller window.
    let at_limit = timings(0, (1, 0), vec![part(100, 240_000)]);
    let host = &block(&at_limit)["estimate"]["host_240"];
    assert_eq!(host["state"], "window_fits");
    assert_eq!(host["vouchers_at_most"], 100);
    let over = timings(0, (1, 0), vec![part(100, 240_001)]);
    let host = &block(&over)["estimate"]["host_240"];
    assert_eq!(host["state"], "window_too_long");
    // fixed 500 ms (the floor), 2,401 + 24 = 2,425 ms a voucher:
    // (225,000 - 500) / 2,425 = 92.
    assert_eq!(host["vouchers_at_most"], 92);
}

#[test]
fn the_block_is_left_out_past_a_sixteenth_of_the_response_budget() {
    // A result is carried twice, so a sixteenth of the budget is an eighth of
    // what the host receives.
    let day = largest_book_day();
    let size = block(&day).to_string().len();
    assert!(read_cost(&day, Ended::Read, size * 16).is_some());
    assert_eq!(read_cost(&day, Ended::Read, size * 16 - 1), None);
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

#[test]
fn the_advice_never_exceeds_the_planners_allowance() {
    // What the first plan can carry: 16 MiB of data budget at 384 KiB a voucher
    // is 42 vouchers a read, over 128 reads (reference 11c.3). If the planner's
    // constants move, this must be looked at on purpose.
    let allowance = voucher_allowance();
    assert_eq!(allowance, 5_376);
    // The cautious rate alone would allow far more (225,000 ms at 24 ms), so the
    // bound is what holds here.
    assert!(allowance < PLANNING_LIMIT_MS / CENSUS_MS_PER_ROW_LOW);
    assert_eq!(
        fit_of(0, 0, 5, 0, 0, DESKTOP_CALL_LIMIT_MS + 1),
        Fit::WindowTooLong {
            at_most: Some(allowance),
            basis: Basis {
                census_row_ms: CENSUS_MS_PER_ROW_LOW,
                fixed_ms: 0,
                per_voucher_ms: CENSUS_MS_PER_ROW_LOW,
            },
        }
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
fn a_census_that_alone_takes_the_planning_limit_leaves_no_window() {
    // 250 reads, 225.000 s exactly (the planning limit), no voucher read:
    // what every call pays is the limit itself.
    let read = timings(0, (250, 225_000), vec![]);
    assert_eq!(
        block(&read)["estimate"]["host_240"],
        json!({"state": "no_window_fits"})
    );
}
