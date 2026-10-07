//! The read-cost block (bridge#1239): what it states, its boundaries, and where it
//! is added.
use super::*;
use crate::agent::voucher_window::{PartTiming, RequestTally};
use bridge_tally_core::TallyDate;

fn day() -> TallyDate {
    TallyDate::parse("20260801").expect("a Tally date")
}

fn part(rows: usize, ms: u128) -> PartTiming {
    PartTiming {
        from: day(),
        to: day(),
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
        from: day(),
        to: day(),
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

/// A weekday of about 760 vouchers on a book with a mark near 1.03 million: over a
/// hundred census reads and about 170 s in all (bridge#595, 28 Sep 2026; the 120
/// reads here are a synthetic round count, the times rounded).
fn largest_book_day() -> WindowReadTimings {
    timings(3_400, (120, 145_000), vec![part(760, 21_000)])
}

/// A window on the same book that took 440 s (synthetic figures: a long window).
fn a_long_window() -> WindowReadTimings {
    timings(3_000, (120, 300_000), vec![part(5_000, 137_000)])
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
    assert_eq!(cost["census_reads"], 120);
    assert_eq!(cost["vouchers_read"], 760);
    assert_eq!(
        cost["observed_seconds"],
        json!({"marks": 3, "census": 145, "parts": 21, "total": 169})
    );
    // 119 gaps of 0.5 s, rounded down.
    assert_eq!(cost["floor_seconds"], 59);
    assert_eq!(
        cost["host_240"],
        json!({"state": "window_fits", "vouchers_that_fitted": 760})
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
        json!({"state": "window_fits", "vouchers_that_fitted": 100})
    );
    let over = timings(0, (1, 0), vec![part(100, 240_001)]);
    assert_eq!(
        block(&over)["host_240"],
        json!({"state": "window_too_long"})
    );
}

#[test]
fn a_window_with_no_voucher_is_never_judged_by_its_time() {
    // It is read twice (the second read, wider, pays its own census), so its own
    // time says nothing about a call, whichever side of 240 s it falls.
    let census_alone = timings(0, (250, 300_000), vec![]);
    assert_eq!(
        block(&census_alone)["host_240"],
        json!({"state": "not_established"})
    );
}

/// The lead sentence rounds toward the verdict, so it can never say 240 seconds of
/// a window that fitted-and-did-not or contradict the state beside it.
#[test]
fn the_lead_never_contradicts_the_verdict_at_the_limit() {
    let fits = block(&timings(0, (1, 0), vec![part(100, 239_600)]));
    assert_eq!(fits["host_240"]["state"], "window_fits");
    assert!(
        fits["say"]
            .as_str()
            .unwrap()
            .starts_with("The window read took 239 seconds for 100 vouchers."),
        "{fits}"
    );
    let over = block(&timings(0, (1, 0), vec![part(100, 240_400)]));
    assert_eq!(over["host_240"]["state"], "window_too_long");
    assert!(
        over["say"]
            .as_str()
            .unwrap()
            .starts_with("The window read took 241 seconds for 100 vouchers."),
        "{over}"
    );
}

/// One voucher is "1 voucher" in every sentence that counts vouchers, and a stopped
/// read of one second is "1 second", not "1 seconds".
#[test]
fn a_single_voucher_and_a_single_second_are_named_in_the_singular() {
    let one = timings(0, (16, 16_000), vec![part(1, 1_000)]);
    let read = block(&one);
    let say = read["say"].as_str().unwrap();
    assert!(
        say.starts_with("The window read took 17 seconds for 1 voucher."),
        "{say}"
    );
    assert!(say.contains("This window of 1 voucher fitted"), "{say}");
    let stopped = read_cost(&one, Ended::Stopped).unwrap();
    assert!(
        stopped["say"].as_str().unwrap().starts_with(
            "The window read stopped after 17 seconds with 1 voucher read and none returned."
        ),
        "{stopped}"
    );
    let quick_stop = read_cost(&timings(0, (16, 500), vec![]), Ended::Stopped).unwrap();
    assert!(
        quick_stop["say"]
            .as_str()
            .unwrap()
            .starts_with("The window read stopped after 1 second, before any voucher was read."),
        "{quick_stop}"
    );
}

/// One second is "1 second".
#[test]
fn the_floor_says_one_second_in_the_singular() {
    let three = block(&timings(0, (3, 21_000), vec![part(1, 1)]));
    assert_eq!(three["floor_seconds"], 1);
    assert!(
        three["say"]
            .as_str()
            .unwrap()
            .contains("at least 1 second of this call"),
        "{three}"
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
            {"host": "claude_code", "seconds": null, "basis": "default_completed_150s_two_builds_configured_60s_cut_at_60s_earlier_60s_unexplained"},
        ])
    );
}

#[test]
fn the_sentence_leads_with_the_outcome_and_labels_what_is_derived() {
    // Pinned whole: assistant-facing text changes on purpose, never by drift.
    const FITS: &str = "This window of {} vouchers fitted inside the 240 seconds at which Claude Desktop's chat app stops a silent call (measured twice, on one Mac build; calls between 130 and 240 seconds were not tried, and the call's other reads are not in these figures). Whether a larger or a smaller window fits is not established.";
    const TOO_LONG: &str = "That is past 240 seconds, where Claude Desktop's chat app stops a silent call (measured twice, on one Mac build). A shorter window saves the time of its vouchers but pays the same census reads, so how short is enough is not established. For totals over a long period read trial_balance, which reads no vouchers.";
    let gaps = |reads: &str, seconds: u64| {
        format!(" This book's census took {reads} reads, and the 0.5 second gate keeps consecutive reads that far apart, so at least {seconds} seconds of this call went on the gaps between them (derived).")
    };
    assert_eq!(
        block(&largest_book_day())["say"],
        format!(
            "The window read took 169 seconds for 760 vouchers.{} {}",
            gaps("120", 59),
            FITS.replace("{}", "760")
        )
    );
    assert_eq!(
        block(&a_long_window())["say"],
        format!(
            "The window read took 440 seconds for 5000 vouchers.{} {TOO_LONG}",
            gaps("120", 59)
        )
    );
    // One census read has no gap between reads to state.
    assert_eq!(
        block(&timings(0, (1, 25_000), vec![part(10, 1_000)]))["say"],
        format!(
            "The window read took 26 seconds for 10 vouchers. {}",
            FITS.replace("{}", "10")
        )
    );
    // A window with no voucher is read once more, which is not in the figures.
    assert_eq!(
        block(&timings(0, (21, 21_000), vec![]))["say"],
        format!(
            "The window read took 21 seconds for 0 vouchers.{} No voucher was read, so what a call can carry is not established. A window with no voucher is also read once more, a day wider on each side, to confirm it is empty; that read pays its own census and is not in these figures.",
            gaps("21", 10)
        )
    );
    let stopped = read_cost(&timings(0, (250, 300_000), vec![]), Ended::Stopped).unwrap();
    assert_eq!(
        stopped["say"],
        format!(
            "The window read stopped after 300 seconds, before any voucher was read.{} The read stopped, so what a call can carry is not established.",
            gaps("at least 250", 124)
        )
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

/// The block is added only when the response can still carry it: a result is
/// carried three times over once it is also copied, escaped, into the text
/// content, plus a kilobyte of envelope. When it is dropped the window says so.
#[test]
fn the_block_is_added_when_the_response_can_carry_it_and_the_window_says_when_not() {
    let day = largest_book_day();
    let block_len = block(&day).to_string().len();
    let container = 5_000;
    let needed = (container + block_len) * 3 + 1_024;
    let window = || serde_json::to_value(&day).unwrap();
    let mut at = window();
    add_read_cost(&mut at, container, &day, Ended::Read, needed);
    assert_eq!(at["read_cost"]["census_reads"], 120);
    assert!(at.get("read_cost_left_out").is_none(), "{at}");
    let mut under = window();
    add_read_cost(&mut under, container, &day, Ended::Read, needed - 1);
    assert!(under.get("read_cost").is_none(), "{under}");
    assert_eq!(under["read_cost_left_out"], "response_budget", "{under}");
    // A quick call has nothing to add, and nothing to say it left out.
    let quick = timings(1_000, (4, 5_200), vec![part(82, 1_540)]);
    let mut plain = serde_json::to_value(&quick).unwrap();
    add_read_cost(&mut plain, 1, &quick, Ended::Read, usize::MAX);
    assert_eq!(plain, serde_json::to_value(&quick).unwrap());
}

/// A page is judged by the smallest page it could be trimmed to (one item), not by
/// the page as asked for: a first page of five hundred vouchers must not lose the
/// block because the page is large.
#[test]
fn a_page_is_judged_by_its_smallest_form() {
    let items = |n: usize| -> Vec<Value> {
        (0..n)
            .map(|i| json!({"n": i, "text": "x".repeat(900)}))
            .collect()
    };
    let page = |n: usize| json!({"company": {"name": "c"}, "result": {"state": "complete", "items": items(n), "total": n}});
    // Fifty items are judged as the one-item page with the same other fields.
    let mut fifty = page(50);
    fifty["result"]["total"] = json!(1);
    assert_eq!(smallest_page_len(&fifty), page(1).to_string().len());
    // A payload with no items array is judged whole.
    let refusal = json!({"error": {"code": "x"}});
    assert_eq!(smallest_page_len(&refusal), refusal.to_string().len());
}

/// The floor is the runtime's own spacing: if that constant moves, this block's
/// "certain" seconds would be wrong, so the two are held together here.
#[test]
fn the_floor_spacing_is_the_runtimes_shipped_spacing() {
    assert_eq!(SPACING_MS, 500);
    let runtime = include_str!("tally/runtime_control.rs");
    assert!(
        runtime.contains("const SHIPPED_REQUEST_SPACING: Duration = Duration::from_millis(500);")
    );
    // The shipped build's default is that constant (tests alone use zero).
    assert!(runtime.contains(
        "#[cfg(not(test))]\nconst DEFAULT_REQUEST_SPACING: Duration = SHIPPED_REQUEST_SPACING;"
    ));
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
    assert_eq!(window["census"]["requests"], 120, "{window}");
    assert_eq!(window["read_cost"]["ended"], "stopped", "{window}");
    assert_eq!(window["read_cost"]["census_reads"], 120, "{window}");
    let outstandings = server.finish_tool_response(
        "outstandings",
        &json!({}),
        chrono::Utc::now(),
        Err(refusal()),
    );
    let window = &outstandings.value["structuredContent"]["result"]["error"]["window"];
    assert_eq!(window["census"]["requests"], 120, "{window}");
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
    assert_eq!(error["window"]["census"]["requests"], 120, "{error}");
    assert!(error["window"].get("read_cost").is_none(), "{error}");
    assert_eq!(
        error["window"]["read_cost_left_out"], "response_budget",
        "{error}"
    );
}

// -- a call that reads its window twice (`ledger_movement`, #1239) ----------------------------------

/// The replay of the largest book's day: the same part again, no census, one more
/// marks read (synthetic times).
fn its_replay() -> WindowReadTimings {
    timings(1_000, (0, 0), vec![part(760, 21_000)])
}

fn replayed_block(first: &WindowReadTimings, second: &WindowReadTimings) -> Value {
    read_cost_of_replayed(first, second, Ended::Read).expect("a block")
}

/// The vouchers are the window's, counted once; the times of both reads add; the
/// census floor is the first read's gaps, and the block says it covers two reads.
#[test]
fn two_reads_of_a_window_count_its_vouchers_once_and_add_their_times() {
    let cost = replayed_block(&largest_book_day(), &its_replay());
    assert_eq!(cost["window_reads"], 2, "{cost}");
    assert_eq!(cost["vouchers_read"], 760, "{cost}");
    assert_eq!(cost["census_reads"], 120, "{cost}");
    assert_eq!(
        cost["observed_seconds"],
        json!({"marks": 4, "census": 145, "parts": 42, "total": 191}),
        "{cost}"
    );
    assert_eq!(cost["floor_seconds"], 59, "{cost}");
    let say = cost["say"].as_str().unwrap();
    assert!(say.contains("reads the window twice"), "{say}");
    assert!(
        say.starts_with("The window read took 191 seconds for 760 vouchers."),
        "{say}"
    );
}

/// The verdict is the call's: a first read that fitted alone does not fit once the
/// replay's time is added.
#[test]
fn a_replay_can_take_a_call_past_the_host_limit_its_first_read_was_inside() {
    let first = largest_book_day();
    assert_eq!(block(&first)["host_240"]["state"], "window_fits");
    let slow_replay = timings(1_000, (0, 0), vec![part(760, 90_000)]);
    let cost = replayed_block(&first, &slow_replay);
    assert_eq!(
        cost["host_240"],
        json!({"state": "window_too_long"}),
        "{cost}"
    );
}

/// A pair of quick reads says nothing, as one quick read does, and a one-read block
/// keeps its shape (no `window_reads`).
#[test]
fn a_quick_pair_says_nothing_and_a_single_read_has_no_window_reads_field() {
    let small = timings(1_000, (4, 5_200), vec![part(82, 1_540)]);
    let replay = timings(1_000, (0, 0), vec![part(82, 1_540)]);
    assert_eq!(read_cost_of_replayed(&small, &replay, Ended::Read), None);
    assert!(block(&largest_book_day()).get("window_reads").is_none());
}

/// A pair that stopped states the floor and no verdict, like a single read.
#[test]
fn a_pair_that_stopped_names_no_verdict() {
    let cost =
        read_cost_of_replayed(&largest_book_day(), &its_replay(), Ended::Stopped).expect("a block");
    assert_eq!(cost["ended"], "stopped", "{cost}");
    assert_eq!(
        cost["host_240"],
        json!({"state": "not_established"}),
        "{cost}"
    );
}

/// The block is placed in the result beside the ledgers, and left out, saying so,
/// when the response cannot carry it.
#[test]
fn the_pair_block_is_placed_or_marked_left_out_by_the_response_budget() {
    let mut roomy = json!({});
    add_read_cost_of_replayed(
        &mut roomy,
        1_000,
        (&largest_book_day(), &its_replay()),
        Ended::Read,
        1_000_000,
    );
    assert!(roomy.get("read_cost").is_some(), "{roomy}");
    let mut tight = json!({});
    add_read_cost_of_replayed(
        &mut tight,
        1_000,
        (&largest_book_day(), &its_replay()),
        Ended::Read,
        1_000,
    );
    assert_eq!(tight, json!({"read_cost_left_out": "response_budget"}));
}

/// Each read's own census gaps add: a second read that did send census reads (the
/// replay sends none today) adds its gaps to the floor, and never a gap between the
/// two reads, which is not between two census reads.
#[test]
fn the_floor_of_two_reads_adds_each_reads_own_census_gaps() {
    let second = timings(1_000, (5, 2_000), vec![part(760, 21_000)]);
    let cost = replayed_block(&largest_book_day(), &second);
    // 119 gaps in the first read and 4 in the second, 0.5 s each: 61.5 s, rounded down.
    assert_eq!(cost["floor_seconds"], 61, "{cost}");
    assert_eq!(cost["census_reads"], 125, "{cost}");
}
