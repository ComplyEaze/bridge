//! What a voucher window read cost, stated for the next call (bridge#1239).
//!
//! The census a window read pays is set by the book's voucher mark, not by the
//! window: a one-day read on a book with a mark near 1.03 million is 126 census
//! reads and about 170 s, and an assistant that reads a month day by day pays that
//! census thirty times. This block says, from the call's own timings alone, what
//! the window read did, what is certain of its cost, and whether it fitted under
//! the one host limit that has been measured. It makes no estimate beyond what the
//! call showed: a larger or a smaller window is not extrapolated from one book's
//! two measurements. It adds no request and decides nothing: it refuses nothing
//! and changes no completeness proof.
//!
//! What each figure is:
//! - `observed_seconds` and the counts are what this call did.
//! - `floor_seconds` is **derived from the gate's rule**, not measured: the gate
//!   holds each request back until 500 ms after the previous one ended
//!   (`SHIPPED_REQUEST_SPACING`), so a call that sent N census reads waited at
//!   least (N - 1) x 0.5 s in all. It is rounded down.
//! - `host_240.vouchers_known_to_fit` is what this call carried inside 240 s: a
//!   window of up to that many vouchers is known to fit this book; nothing larger
//!   is claimed.
//! - the host limits are facts about hosts, each with its basis.
//!
//! A read that stopped (a refusal) states the floor and no verdict: the time of a
//! request that failed or hung is not a cost of the window. The figures are for
//! the window read alone; the call's smaller reads (company check, ledger and type
//! lists) are not in them, and a window with no voucher is read once more, a day
//! wider on each side, which is not counted either (`say` states that case).
use super::voucher_window::WindowReadTimings;
use serde_json::{json, Value};

/// The wait between two census reads, in ms (`SHIPPED_REQUEST_SPACING`, pinned
/// to the runtime's constant by a test that reads its source).
const SPACING_MS: u64 = 500;

/// The one call limit that has been measured: Claude Desktop's chat app on
/// macOS, bundle 2.19675.0, cancelled a silent 250 s call at 240 s in two runs
/// (protocol reference 11f). Calls between 130 s and 240 s were not tried, and
/// whether progress notifications would extend the limit is not answered.
const DESKTOP_CALL_LIMIT_MS: u64 = 240_000;

/// A call this slow is worth a statement: from here, thirty one-day calls take
/// ten minutes.
const NOTABLE_ELAPSED_MS: u64 = 20_000;

/// A call whose floor is this long is worth a statement even when it stopped
/// early (a read refused or failed in its census): sixteen census reads wait at
/// least 7.5 s between them, and thirty such calls four minutes.
const NOTABLE_FLOOR_MS: u64 = 7_500;

/// Whether the read finished or stopped part-way. The lead sentence says so.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Ended {
    Read,
    Stopped,
}

/// How this window sits against the 240 s limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fit {
    /// This window fitted: `vouchers` is what it carried.
    Fits { vouchers: u64 },
    /// This window did not fit.
    TooLong,
    /// A read that stopped, or one that read no voucher: no verdict.
    NotEstablished,
}

impl Fit {
    fn state(self) -> &'static str {
        match self {
            Self::Fits { .. } => "window_fits",
            Self::TooLong => "window_too_long",
            Self::NotEstablished => "not_established",
        }
    }
}

struct Cost {
    census_reads: u64,
    vouchers: u64,
    marks_ms: u64,
    census_ms: u64,
    parts_ms: u64,
    floor_ms: u64,
    fit: Fit,
}

impl Cost {
    fn of(timings: &WindowReadTimings, ended: Ended) -> Self {
        let census_reads = timings.census.requests;
        let vouchers = timings
            .parts
            .iter()
            .filter(|part| part.served)
            .filter_map(|part| part.rows)
            .map(|rows| u64::try_from(rows).unwrap_or(u64::MAX))
            .fold(0_u64, u64::saturating_add);
        let marks_ms = ms(timings.marks.ms);
        let census_ms = ms(timings.census.ms);
        let parts_ms = timings
            .parts
            .iter()
            .map(|part| ms(part.ms))
            .fold(0_u64, u64::saturating_add);
        let total_ms = marks_ms.saturating_add(census_ms).saturating_add(parts_ms);
        let fit = match ended {
            // A read that stopped has requests that failed or hung in its times.
            Ended::Stopped => Fit::NotEstablished,
            Ended::Read if total_ms > DESKTOP_CALL_LIMIT_MS => Fit::TooLong,
            Ended::Read if vouchers == 0 => Fit::NotEstablished,
            Ended::Read => Fit::Fits { vouchers },
        };
        Self {
            census_reads,
            vouchers,
            marks_ms,
            census_ms,
            parts_ms,
            floor_ms: census_reads.saturating_sub(1).saturating_mul(SPACING_MS),
            fit,
        }
    }

    fn total_ms(&self) -> u64 {
        self.marks_ms
            .saturating_add(self.census_ms)
            .saturating_add(self.parts_ms)
    }

    /// Whether the call is slow enough, or its floor long enough, to say anything
    /// about.
    fn notable(&self) -> bool {
        self.total_ms() >= NOTABLE_ELAPSED_MS || self.floor_ms >= NOTABLE_FLOOR_MS
    }
}

fn ms(value: u128) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

/// Whole seconds, rounded to the nearest (an observed figure).
fn nearest_seconds(value_ms: u64) -> u64 {
    value_ms.saturating_add(500) / 1000
}

/// The block, or `None` when the call was quick enough that nothing needs
/// saying.
pub(super) fn read_cost(timings: &WindowReadTimings, ended: Ended) -> Option<Value> {
    let cost = Cost::of(timings, ended);
    if !cost.notable() {
        return None;
    }
    let mut host_240 = json!({"state": cost.fit.state()});
    if let Fit::Fits { vouchers } = cost.fit {
        host_240["vouchers_known_to_fit"] = json!(vouchers);
    }
    Some(json!({
        "ended": match ended {
            Ended::Read => "read",
            Ended::Stopped => "stopped",
        },
        "census_reads": cost.census_reads,
        "vouchers_read": cost.vouchers,
        "observed_seconds": {
            "marks": nearest_seconds(cost.marks_ms),
            "census": nearest_seconds(cost.census_ms),
            "parts": nearest_seconds(cost.parts_ms),
            "total": nearest_seconds(cost.total_ms()),
        },
        // Derived from the gate's rule, rounded down.
        "floor_seconds": cost.floor_ms / 1000,
        "host_240": host_240,
        "host_limits": [
            {"host": "claude_desktop_chat_macos", "seconds": 240, "basis": "measured_twice_one_build_silent_calls"},
            {"host": "claude_desktop_chat_windows", "seconds": null, "basis": "unmeasured"},
            {"host": "claude_code", "seconds": null, "basis": "completed_150s_by_default_one_run_per_two_builds_one_earlier_60s_unexplained"},
        ],
        "say": say(&cost, ended),
    }))
}

/// Adds the block to a `window` value as its `read_cost` when there is one and
/// the whole response still fits without costing a caller its rows: `container_len`
/// is the serialized length of the payload (or refusal) the window sits in, which
/// a result carries three times over once it is also copied, escaped, into the
/// text content.
pub(super) fn add_read_cost(
    window: &mut Value,
    container_len: usize,
    timings: &WindowReadTimings,
    ended: Ended,
    max_bytes: usize,
) {
    let Some(block) = read_cost(timings, ended) else {
        return;
    };
    let needed = container_len
        .saturating_add(block.to_string().len())
        .saturating_mul(3)
        .saturating_add(1_024);
    if needed <= max_bytes {
        if let Some(object) = window.as_object_mut() {
            object.insert("read_cost".to_string(), block);
        }
    }
}

/// The outcome first, then what is certain of the cost, then whether it fitted.
/// Built from numbers and the states above, never from text of the book.
fn say(cost: &Cost, ended: Ended) -> String {
    let lead = match (ended, cost.vouchers) {
        (Ended::Read, vouchers) => format!(
            "The window read took {} seconds for {vouchers} vouchers.",
            nearest_seconds(cost.total_ms())
        ),
        (Ended::Stopped, 0) => format!(
            "The window read stopped after {} seconds, before any voucher was read.",
            nearest_seconds(cost.total_ms())
        ),
        (Ended::Stopped, vouchers) => format!(
            "The window read stopped after {} seconds with {vouchers} vouchers read and none returned.",
            nearest_seconds(cost.total_ms())
        ),
    };
    let floor = if cost.census_reads > 1 {
        let at_least = if ended == Ended::Stopped {
            "at least "
        } else {
            ""
        };
        format!(
            " This book's census took {at_least}{} reads, so at least {} seconds of any call go on the wait between them (derived from the 0.5 second gate).",
            cost.census_reads,
            cost.floor_ms / 1000
        )
    } else {
        String::new()
    };
    let fit = match cost.fit {
        Fit::Fits { vouchers } => format!(
            "Claude Desktop's chat app stops a silent call at 240 seconds (measured twice, on one Mac build); a window of up to {vouchers} vouchers is known to fit this book, and whether a larger one does is not established."
        ),
        Fit::TooLong => "That is past 240 seconds, where Claude Desktop's chat app stops a silent call (measured twice, on one Mac build). Read a shorter window; how short is not established. For totals over a long period read trial_balance, which reads no vouchers.".to_string(),
        Fit::NotEstablished => match ended {
            Ended::Read => "No voucher was read, so what a call can carry is not established. A window with no voucher is also read once more, a day wider on each side, to confirm it is empty; that read pays its own census and is not in these figures.".to_string(),
            Ended::Stopped => "The read stopped, so what a call can carry is not established.".to_string(),
        },
    };
    format!("{lead}{floor} {fit}")
}

#[cfg(test)]
#[path = "agent_read_cost_tests.rs"]
mod tests;
