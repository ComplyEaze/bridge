//! What a voucher window read cost, stated for the next call (bridge#1239).
//!
//! The census a window read pays is set by the book's voucher mark, not by the
//! window: a one-day read on a book with a mark near 1.03 million is 126 census
//! reads and 169 s, and so is a one-week read's census (bridge#595). An
//! assistant that reads a month day by day pays that census thirty times. This
//! block tells it, from the call's own timings, what is paid on every call and
//! how many vouchers one call can carry under the one host limit that has been
//! measured. It adds no request and decides nothing: it refuses nothing and
//! changes no completeness proof.
//!
//! Every figure carries what it is:
//! - `floor_seconds` is a **measured lower bound**: each census read waits 500 ms
//!   before the next (`SHIPPED_REQUEST_SPACING`), so `reads x 0.5 s` is a floor by
//!   construction. It is rounded down.
//! - everything under `estimate` is **derived** from this call's own timings and
//!   one measured constant, and rounded the cautious way.
//! - the host limits are facts about hosts, each with its basis.
use super::voucher_window::WindowReadTimings;
use serde_json::{json, Value};

/// The wait between two census reads, in ms (`SHIPPED_REQUEST_SPACING`, pinned
/// to the runtime's constant by a test that reads its source).
const SPACING_MS: u64 = 500;

/// What a census row adds to the census, in ms. Measured once, on one book
/// (mark about 1.03M): the same census was 144.5 s for a day of 757 vouchers and
/// 307.6 s for a week of 5,178 (bridge#595, 28 Sep 2026), which is 36.9 ms a
/// row. Used for every book, so it over-states a small-mark book's cost, which
/// errs toward a narrower window, never a wider one.
const CENSUS_MS_PER_ROW: u64 = 37;

/// The one call limit that has been measured: Claude Desktop's chat app on
/// macOS, bundle 2.19675.0, cancelled a silent call at 240 s (protocol reference
/// 11f, one run).
const DESKTOP_CALL_LIMIT_MS: u64 = 240_000;

/// A call this slow is worth a statement: from here, thirty one-day calls take
/// ten minutes.
const NOTABLE_ELAPSED_MS: u64 = 20_000;

/// A call whose certain floor is this long is worth a statement even when it
/// stopped early (a read refused or failed in its census): thirty such calls
/// wait four minutes on the floor alone.
const NOTABLE_FLOOR_MS: u64 = 8_000;

/// Whether the read finished or stopped part-way. The lead sentence says so.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Ended {
    Read,
    Stopped,
}

/// How a window of this book sits against the 240 s limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fit {
    /// This window fitted; `at_most` vouchers is the most one call can carry.
    WindowFits { at_most: u64 },
    /// This window did not fit; `at_most` vouchers would.
    WindowTooLong { at_most: u64 },
    /// What every call pays alone is already at the limit.
    NoWindowFits,
    /// No voucher was read, so no per-voucher rate exists.
    NotEstablished,
}

impl Fit {
    fn state(self) -> &'static str {
        match self {
            Self::WindowFits { .. } => "window_fits",
            Self::WindowTooLong { .. } => "window_too_long",
            Self::NoWindowFits => "no_window_fits",
            Self::NotEstablished => "not_established",
        }
    }

    fn at_most(self) -> Option<u64> {
        match self {
            Self::WindowFits { at_most } | Self::WindowTooLong { at_most } => Some(at_most),
            Self::NoWindowFits | Self::NotEstablished => None,
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
    fixed_ms: u64,
    per_voucher_ms: Option<u64>,
    fit: Fit,
}

impl Cost {
    fn of(timings: &WindowReadTimings) -> Self {
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
        let floor_ms = census_reads.saturating_mul(SPACING_MS);
        // What the census paid beyond the rows it carried is paid by any
        // window; never less than the certain floor.
        let fixed_ms = census_ms
            .saturating_sub(vouchers.saturating_mul(CENSUS_MS_PER_ROW))
            .max(floor_ms)
            .saturating_add(marks_ms);
        let per_voucher_ms = (vouchers > 0).then(|| {
            parts_ms
                .div_ceil(vouchers)
                .saturating_add(CENSUS_MS_PER_ROW)
        });
        let total_ms = marks_ms.saturating_add(census_ms).saturating_add(parts_ms);
        let fit = fit_of(fixed_ms, per_voucher_ms, vouchers, total_ms);
        Self {
            census_reads,
            vouchers,
            marks_ms,
            census_ms,
            parts_ms,
            floor_ms,
            fixed_ms,
            per_voucher_ms,
            fit,
        }
    }

    fn total_ms(&self) -> u64 {
        self.marks_ms
            .saturating_add(self.census_ms)
            .saturating_add(self.parts_ms)
    }

    /// Whether the call is slow enough, or its certain floor long enough, to say
    /// anything about.
    fn notable(&self) -> bool {
        self.total_ms() >= NOTABLE_ELAPSED_MS || self.floor_ms >= NOTABLE_FLOOR_MS
    }
}

fn ms(value: u128) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn fit_of(fixed_ms: u64, per_voucher_ms: Option<u64>, vouchers: u64, total_ms: u64) -> Fit {
    if fixed_ms >= DESKTOP_CALL_LIMIT_MS {
        return Fit::NoWindowFits;
    }
    let Some(per_voucher_ms) = per_voucher_ms else {
        return Fit::NotEstablished;
    };
    // At most 240,000 / 38 = 6,315 vouchers: below the 10,880 the planner's own
    // allowance admits (a test holds that), so the allowance never needs applying
    // here.
    let at_most = (DESKTOP_CALL_LIMIT_MS - fixed_ms) / per_voucher_ms.max(1);
    if total_ms <= DESKTOP_CALL_LIMIT_MS {
        // What demonstrably fitted is never advised against, however cautious
        // the model.
        return Fit::WindowFits {
            at_most: at_most.max(vouchers),
        };
    }
    if at_most == 0 {
        return Fit::NoWindowFits;
    }
    Fit::WindowTooLong { at_most }
}

/// Whole seconds, rounded to the nearest (an observed figure).
fn nearest_seconds(value_ms: u64) -> u64 {
    value_ms.saturating_add(500) / 1000
}

/// The block, or `None` when the call was quick enough that nothing needs
/// saying.
pub(super) fn read_cost(timings: &WindowReadTimings, ended: Ended) -> Option<Value> {
    let cost = Cost::of(timings);
    if !cost.notable() {
        return None;
    }
    let mut host_240 = json!({"state": cost.fit.state()});
    if let Some(at_most) = cost.fit.at_most() {
        host_240["vouchers_at_most"] = json!(at_most);
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
        // A measured lower bound, rounded down.
        "floor_seconds": cost.floor_ms / 1000,
        "estimate": {
            "kind": "derived",
            // Paid by every call whatever its window, rounded up.
            "fixed_seconds": cost.fixed_ms.div_ceil(1000),
            "per_voucher_ms": cost.per_voucher_ms,
            "host_240": host_240,
        },
        "host_limits": [
            {"host": "claude_desktop_chat_macos", "seconds": 240, "basis": "measured_once_one_build"},
            {"host": "claude_desktop_chat_windows", "seconds": null, "basis": "unmeasured"},
            {"host": "claude_code", "seconds": null, "basis": "none_by_default_user_can_set"},
        ],
        "say": say(&cost, ended),
    }))
}

/// The same block, added to a `window` value as its `read_cost` when there is
/// one to add.
pub(super) fn add_read_cost(window: &mut Value, timings: &WindowReadTimings, ended: Ended) {
    if let (Some(object), Some(block)) = (window.as_object_mut(), read_cost(timings, ended)) {
        object.insert("read_cost".to_string(), block);
    }
}

/// `timings` as the `window` value of a result, with its read cost.
pub(super) fn window_value(timings: &WindowReadTimings, ended: Ended) -> Value {
    let mut window = serde_json::to_value(timings).unwrap_or(Value::Null);
    add_read_cost(&mut window, timings, ended);
    window
}

/// The outcome first, then what every call pays, then what one call can carry.
/// Built from numbers and the states above, never from text of the book.
fn say(cost: &Cost, ended: Ended) -> String {
    let lead = match ended {
        Ended::Read => format!(
            "This read took {} seconds for {} vouchers.",
            nearest_seconds(cost.total_ms()),
            cost.vouchers
        ),
        Ended::Stopped => format!(
            "This read stopped after {} seconds with {} vouchers read.",
            nearest_seconds(cost.total_ms()),
            cost.vouchers
        ),
    };
    let fixed = format!(
        "About {} seconds (derived) is paid on every call, however short the window, because this book needs {} census reads; at least {} of those seconds are certain.",
        cost.fixed_ms.div_ceil(1000),
        cost.census_reads,
        cost.floor_ms / 1000
    );
    let fit = match cost.fit {
        Fit::WindowFits { at_most } => format!(
            "Claude Desktop's chat app stops a call at 240 seconds (measured once, on one Mac build): one call there can carry about {at_most} vouchers (derived), so use the widest window within that, not many short calls."
        ),
        Fit::WindowTooLong { at_most } => format!(
            "That is past 240 seconds, where Claude Desktop's chat app stops a call (measured once, on one Mac build): read about {at_most} vouchers or fewer per call (derived)."
        ),
        Fit::NoWindowFits => "On a host that stops a call at 240 seconds (Claude Desktop's chat app, measured once on one Mac build) no window of this book fits, because what every call pays is already that long; do not suggest one.".to_string(),
        Fit::NotEstablished => "No voucher was read, so how many one call can carry is not established.".to_string(),
    };
    format!("{lead} {fixed} {fit}")
}

#[cfg(test)]
#[path = "agent_read_cost_tests.rs"]
mod tests;
