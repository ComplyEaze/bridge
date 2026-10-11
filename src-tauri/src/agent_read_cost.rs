//! What a voucher window read cost, stated for the next call (bridge#1239).
//!
//! The census a window read pays is set by the book's voucher mark, not by the
//! window: a one-day read on a book with a mark near 1.03 million is over a hundred
//! census reads and about 170 s, and an assistant that reads a month day by day pays that
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
//!   (`SHIPPED_REQUEST_SPACING`), so consecutive census reads are at least 0.5 s
//!   apart and a call that sent N of them took at least (N - 1) x 0.5 s in
//!   gaps. It is time between reads, not time spent sleeping (the gate sleeps
//!   only what is left of the half second after Bridge's own work). Rounded down.
//! - `host_240.vouchers_that_fitted` is what this window carried inside 240 s. It
//!   claims nothing about a larger or a smaller window: fewer vouchers are not
//!   cheaper (a window with no voucher is read twice).
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
            // A window with no voucher is read twice, so neither verdict is its own.
            Ended::Read if vouchers == 0 => Fit::NotEstablished,
            Ended::Read if total_ms > DESKTOP_CALL_LIMIT_MS => Fit::TooLong,
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

    /// A call that reads its window twice, a planned read and a replay of its parts
    /// (`ledger_movement`): the vouchers are the window's, counted once; the times
    /// and the census reads of both are added; the floor is each read's own gaps
    /// between consecutive census reads, so it never counts a gap that was not
    /// between two of them.
    fn of_replayed(first: &WindowReadTimings, second: &WindowReadTimings, ended: Ended) -> Self {
        let a = Self::of(first, ended);
        let b = Self::of(second, ended);
        let marks_ms = a.marks_ms.saturating_add(b.marks_ms);
        let census_ms = a.census_ms.saturating_add(b.census_ms);
        let parts_ms = a.parts_ms.saturating_add(b.parts_ms);
        let total_ms = marks_ms.saturating_add(census_ms).saturating_add(parts_ms);
        let fit = match ended {
            Ended::Stopped => Fit::NotEstablished,
            Ended::Read if a.vouchers == 0 => Fit::NotEstablished,
            Ended::Read if total_ms > DESKTOP_CALL_LIMIT_MS => Fit::TooLong,
            Ended::Read => Fit::Fits {
                vouchers: a.vouchers,
            },
        };
        Self {
            census_reads: a.census_reads.saturating_add(b.census_reads),
            vouchers: a.vouchers,
            marks_ms,
            census_ms,
            parts_ms,
            floor_ms: a.floor_ms.saturating_add(b.floor_ms),
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

/// The call's total in whole seconds as the lead sentence says it: rounded toward
/// the verdict, so that a window that fitted never reads as 240 s or more and one
/// that did not never reads as 240 s or less.
fn lead_seconds(cost: &Cost) -> u64 {
    match cost.fit {
        Fit::Fits { .. } => cost.total_ms() / 1000,
        Fit::TooLong => cost.total_ms().div_ceil(1000),
        Fit::NotEstablished => nearest_seconds(cost.total_ms()),
    }
}

/// "1 second", "2 seconds".
fn seconds(count: u64) -> String {
    if count == 1 {
        "1 second".to_string()
    } else {
        format!("{count} seconds")
    }
}

/// "1 voucher", "3 vouchers": the count with its noun.
fn vouchers_phrase(count: u64) -> String {
    if count == 1 {
        "1 voucher".to_string()
    } else {
        format!("{count} vouchers")
    }
}

/// The block, or `None` when the call was quick enough that nothing needs
/// saying.
pub(super) fn read_cost(timings: &WindowReadTimings, ended: Ended) -> Option<Value> {
    block(Cost::of(timings, ended), ended, 1)
}

/// [`read_cost`] for a call that read its window twice, a planned read and a replay
/// of it (`ledger_movement`): `first` and `second` are the two reads' timings.
pub(super) fn read_cost_of_replayed(
    first: &WindowReadTimings,
    second: &WindowReadTimings,
    ended: Ended,
) -> Option<Value> {
    block(Cost::of_replayed(first, second, ended), ended, 2)
}

fn block(cost: Cost, ended: Ended, window_reads: u64) -> Option<Value> {
    if !cost.notable() {
        return None;
    }
    let mut host_240 = json!({"state": cost.fit.state()});
    if let Fit::Fits { vouchers } = cost.fit {
        host_240["vouchers_that_fitted"] = json!(vouchers);
    }
    let mut block = json!({
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
        // Derived from the gate's rule (the gaps between reads), rounded down.
        "floor_seconds": cost.floor_ms / 1000,
        "host_240": host_240,
        "host_limits": [
            {"host": "claude_desktop_chat_macos", "seconds": 240, "basis": "measured_twice_one_build_silent_calls"},
            {"host": "claude_desktop_chat_windows", "seconds": null, "basis": "unmeasured"},
            {"host": "claude_code", "seconds": null, "basis": "default_completed_150s_two_builds_configured_60s_cut_at_60s_earlier_60s_unexplained"},
        ],
        "say": say(&cost, ended, window_reads),
    });
    // Said only when it is not the usual one read, so the `vouchers` block keeps its shape.
    if window_reads > 1 {
        block["window_reads"] = json!(window_reads);
    }
    Some(block)
}

/// Adds the block to a `window` value as its `read_cost` when there is one and
/// the response can still carry it: `container_len` is the serialized length of the
/// smallest payload (or refusal) the window sits in, which a result carries three
/// times over once it is also copied, escaped, into the text content. When the
/// block is dropped for want of room the window says so (`read_cost_left_out`).
pub(super) fn add_read_cost(
    window: &mut Value,
    container_len: usize,
    timings: &WindowReadTimings,
    ended: Ended,
    max_bytes: usize,
) {
    place_block(window, container_len, read_cost(timings, ended), max_bytes);
}

/// [`add_read_cost`] for a call that read its window twice (`ledger_movement`).
pub(super) fn add_read_cost_of_replayed(
    window: &mut Value,
    container_len: usize,
    (first, second): (&WindowReadTimings, &WindowReadTimings),
    ended: Ended,
    max_bytes: usize,
) {
    place_block(
        window,
        container_len,
        read_cost_of_replayed(first, second, ended),
        max_bytes,
    );
}

fn place_block(window: &mut Value, container_len: usize, block: Option<Value>, max_bytes: usize) {
    let Some(block) = block else {
        return;
    };
    let needed = container_len
        .saturating_add(block.to_string().len())
        .saturating_mul(3)
        .saturating_add(1_024);
    if let Some(object) = window.as_object_mut() {
        if needed <= max_bytes {
            object.insert("read_cost".to_string(), block);
        } else {
            // Said, so that a caller can tell a quick call from a slow one whose
            // block would not fit.
            object.insert("read_cost_left_out".to_string(), json!("response_budget"));
        }
    }
}

/// The serialized length of `payload` with its `result.items` cut to the first
/// one: what the smallest page that can still be returned would weigh. The block
/// is judged against that, since a page that is trimmed to fit its budget gives up
/// rows, never the block's own refusal code or its last row.
pub(super) fn smallest_page_len(payload: &Value) -> usize {
    smallest_page_len_of(payload, "items")
}

/// [`smallest_page_len`] for a result whose page is the array `key` (`ledgers`).
pub(super) fn smallest_page_len_of(payload: &Value, key: &str) -> usize {
    let whole = payload.to_string().len();
    let Some(items) = payload["result"][key].as_array() else {
        return whole;
    };
    let all: usize = items.iter().map(|item| item.to_string().len() + 1).sum();
    let first = items.first().map_or(0, |item| item.to_string().len() + 1);
    whole.saturating_sub(all).saturating_add(first)
}

/// The outcome first, then what is certain of the cost, then whether it fitted.
/// Built from numbers and the states above, never from text of the book.
fn say(cost: &Cost, ended: Ended, window_reads: u64) -> String {
    let lead = match (ended, cost.vouchers) {
        (Ended::Read, vouchers) => format!(
            "The {} took {} for {}.",
            if window_reads > 1 {
                "two window reads"
            } else {
                "window read"
            },
            seconds(lead_seconds(cost)),
            vouchers_phrase(vouchers)
        ),
        (Ended::Stopped, 0) => format!(
            "The window read stopped after {}, before any voucher was read.",
            seconds(nearest_seconds(cost.total_ms()))
        ),
        (Ended::Stopped, vouchers) => format!(
            "The window read stopped after {} with {} read and none returned.",
            seconds(nearest_seconds(cost.total_ms())),
            vouchers_phrase(vouchers)
        ),
    };
    let floor = if cost.census_reads > 1 {
        let at_least = if ended == Ended::Stopped {
            "at least "
        } else {
            ""
        };
        let census = if window_reads > 1 {
            "Across the two reads the census took"
        } else {
            "This book's census took"
        };
        format!(
            " {census} {at_least}{} reads, and the 0.5 second gate keeps consecutive reads that far apart, so at least {} of this call went on the gaps between them (derived).",
            cost.census_reads,
            seconds(cost.floor_ms / 1000)
        )
    } else {
        String::new()
    };
    let fit = match cost.fit {
        Fit::Fits { vouchers } => format!(
            "This window of {} fitted inside the 240 seconds at which Claude Desktop's chat app stops a silent call (measured twice, on one Mac build; calls between 130 and 240 seconds were not tried, and the call's other reads are not in these figures). Whether a larger or a smaller window fits is not established.",
            vouchers_phrase(vouchers)
        ),
        Fit::TooLong => "That is past 240 seconds, where Claude Desktop's chat app stops a silent call (measured twice, on one Mac build). A shorter window saves the time of its vouchers but pays the same census reads, so how short is enough is not established. For totals over a long period read trial_balance, which reads no vouchers.".to_string(),
        Fit::NotEstablished => match ended {
            Ended::Read if window_reads > 1 => "No voucher was read, so what a call can carry is not established. A window with no voucher is also read once more by each of the two reads, a day wider on each side, to confirm it is empty; those reads pay their own census when the book is large, and are not in these figures.".to_string(),
            Ended::Read => "No voucher was read, so what a call can carry is not established. A window with no voucher is also read once more, a day wider on each side, to confirm it is empty; that read pays its own census when the book is large, and is not in these figures.".to_string(),
            Ended::Stopped => "The read stopped, so what a call can carry is not established.".to_string(),
        },
    };
    let twice = if window_reads > 1 {
        " This call reads the window twice, a planned read and a replay of it that proves nothing moved, and these figures add both."
    } else {
        ""
    };
    format!("{lead}{floor}{twice} {fit}")
}

#[cfg(test)]
#[path = "agent_read_cost_tests.rs"]
mod tests;
