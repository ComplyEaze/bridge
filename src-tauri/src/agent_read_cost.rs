//! What a voucher window read cost, stated for the next call (bridge#1239).
//!
//! The census a window read pays is set by the book's voucher mark, not by the
//! window: a one-day read on a book with a mark near 1.03 million is 126 census
//! reads and about 170 s, and a one-week read pays the same census (bridge#595). An
//! assistant that reads a month day by day pays that census thirty times. This
//! block tells it, from the call's own timings, what is paid on every call and
//! how many vouchers one call can carry under the one host limit that has been
//! measured. It adds no request and decides nothing: it refuses nothing and
//! changes no completeness proof.
//!
//! Every figure carries what it is:
//! - `floor_seconds` is a **lower bound by construction**: each census read waits
//!   500 ms before the next (`SHIPPED_REQUEST_SPACING`), so `reads x 0.5 s` can
//!   never be more than the wait. It is rounded down.
//! - everything under `estimate` is **derived** from this call's own timings and
//!   a census row cost measured once, taken at two thirds or one and a half times it so the
//!   advice errs toward the narrower window, and bounded by the voucher
//!   allowance as well as the host limit.
//! - the host limits are facts about hosts, each with its basis.
//!
//! The figures are for the window read alone. The call's smaller reads (company
//! check, ledger and type lists) are not in them, and a window with no voucher is
//! read once more, a day wider on each side, which is not counted either; `say`
//! states that last case.
use super::voucher_window::{
    VoucherReadShape, WindowReadTimings, MAX_PLANNED_READS, WINDOW_READ_BUDGET_BYTES,
};
use serde_json::{json, Value};

/// The wait between two census reads, in ms (`SHIPPED_REQUEST_SPACING`, pinned
/// to the runtime's constant by a test that reads its source).
const SPACING_MS: u64 = 500;

/// What a census row adds to the census, in ms, measured once on one book (mark
/// about 1.03M): the same census took about 145 s for a day of about 760
/// vouchers and about 308 s for a week of about 5,200 (bridge#595, 28 Sep 2026,
/// rounded), which is about 37 ms a row. One run, one host: the advice is not
/// taken at this figure but at two thirds or one and a half times it, whichever
/// errs toward the narrower window.
const CENSUS_MS_PER_ROW: u64 = 37;

/// Taken when the call did not fit and the advice is a smaller window: a census
/// that shrinks less per row than measured would make a narrower window cost
/// more than the model says, so the model assumes two thirds of the measured
/// figure (24 ms).
const CENSUS_MS_PER_ROW_LOW: u64 = CENSUS_MS_PER_ROW * 2 / 3;

/// Taken when the call fitted and the advice is a larger window: a census that
/// grows more per row than measured would make a wider window cost more than the
/// model says, so the model assumes one and a half times the measured figure
/// (55 ms).
const CENSUS_MS_PER_ROW_HIGH: u64 = CENSUS_MS_PER_ROW * 3 / 2;

/// The most vouchers one call can admit, from the planner's own limits: the
/// reads it may dispatch, each no larger than the data budget at half the
/// shape's default cost per voucher (a measured cost never plans below half the
/// default). No advice is given above it.
fn voucher_allowance() -> u64 {
    let smallest_part = VoucherReadShape::EntryWildcard.default_wire_bytes_per_voucher() / 2;
    (WINDOW_READ_BUDGET_BYTES / smallest_part.max(1))
        .saturating_mul(u64::try_from(MAX_PLANNED_READS).unwrap_or(u64::MAX))
}

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
    /// The census row cost the estimate assumed (see the constants).
    census_row_ms: u64,
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
        let total_ms = marks_ms.saturating_add(census_ms).saturating_add(parts_ms);
        // A call that fitted is advised toward a larger window, a call that did
        // not toward a smaller one; each takes the census row cost that makes
        // the other side of the observed point cost more, not less.
        let census_row_ms = if total_ms <= DESKTOP_CALL_LIMIT_MS {
            CENSUS_MS_PER_ROW_HIGH
        } else {
            CENSUS_MS_PER_ROW_LOW
        };
        // What the census paid beyond the rows it carried is paid by any
        // window; never less than the certain floor.
        let fixed_ms = census_ms
            .saturating_sub(vouchers.saturating_mul(census_row_ms))
            .max(floor_ms)
            .saturating_add(marks_ms);
        let per_voucher_ms =
            (vouchers > 0).then(|| parts_ms.div_ceil(vouchers).saturating_add(census_row_ms));
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
            census_row_ms,
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
    // Bounded by both limits: the host's, and what the planner admits in one call.
    let at_most =
        ((DESKTOP_CALL_LIMIT_MS - fixed_ms) / per_voucher_ms.max(1)).min(voucher_allowance());
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
/// saying, or when it would take more than an eighth of the response budget
/// (`max_bytes`): guidance never costs the caller a rows page or a refusal code,
/// so at a deliberately small cap it is left out.
pub(super) fn read_cost(
    timings: &WindowReadTimings,
    ended: Ended,
    max_bytes: usize,
) -> Option<Value> {
    let cost = Cost::of(timings);
    if !cost.notable() {
        return None;
    }
    let mut host_240 = json!({"state": cost.fit.state()});
    if let Some(at_most) = cost.fit.at_most() {
        host_240["vouchers_at_most"] = json!(at_most);
    }
    let block = json!({
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
            // The census row cost the figures below assume, in ms: one and a half
            // times the measured one when the call fitted, two thirds when it
            // did not.
            "census_ms_per_row": cost.census_row_ms,
            // Paid by every call whatever its window, rounded up.
            "fixed_seconds": cost.fixed_ms.div_ceil(1000),
            "per_voucher_ms": cost.per_voucher_ms,
            "host_240": host_240,
        },
        "host_limits": [
            {"host": "claude_desktop_chat_macos", "seconds": 240, "basis": "measured_once_one_build"},
            {"host": "claude_desktop_chat_windows", "seconds": null, "basis": "unmeasured"},
            {"host": "claude_code", "seconds": null, "basis": "not_cut_at_150s_by_default_one_earlier_60s_unexplained"},
        ],
        "say": say(&cost, ended),
    });
    (block.to_string().len() <= max_bytes / 8).then_some(block)
}

/// The same block, added to a `window` value as its `read_cost` when there is
/// one to add.
pub(super) fn add_read_cost(
    window: &mut Value,
    timings: &WindowReadTimings,
    ended: Ended,
    max_bytes: usize,
) {
    if let (Some(object), Some(block)) =
        (window.as_object_mut(), read_cost(timings, ended, max_bytes))
    {
        object.insert("read_cost".to_string(), block);
    }
}

/// The outcome first, then what every call pays, then what one call can carry.
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
    // A read that stopped part-way has sent only the census reads so far.
    let at_least = match ended {
        Ended::Read => "",
        Ended::Stopped => "at least ",
    };
    let fixed = format!(
        "About {} seconds (derived) is paid on every call, however short the window, because this book needs {at_least}{} census reads; at least {} of those seconds are certain (the wait between reads).",
        cost.fixed_ms.div_ceil(1000),
        cost.census_reads,
        cost.floor_ms / 1000
    );
    let fit = match cost.fit {
        Fit::WindowFits { at_most } => format!(
            "Claude Desktop's chat app stops a call at 240 seconds (measured once, on one Mac build): one call there can carry about {at_most} vouchers (derived), so use the widest window within that, not many short calls."
        ),
        Fit::WindowTooLong { at_most } => format!(
            "That is past 240 seconds, where Claude Desktop's chat app stops a call (measured once, on one Mac build): read about {at_most} vouchers or fewer per call (derived). For totals over a long period read trial_balance, which reads no vouchers."
        ),
        Fit::NoWindowFits => "On a host that stops a call at 240 seconds (Claude Desktop's chat app, measured once on one Mac build) no window of this book fits, because what every call pays is already that long; do not suggest one. For totals over a long period read trial_balance, which reads no vouchers.".to_string(),
        Fit::NotEstablished => match ended {
            Ended::Read => "No voucher was read, so how many one call can carry is not established. A window with no voucher is also read once more, a day wider on each side, to confirm it is empty; that read is not in these figures and costs about as much again (derived).".to_string(),
            Ended::Stopped => "No voucher was read, so how many one call can carry is not established.".to_string(),
        },
    };
    format!("{lead} {fixed} {fit}")
}

#[cfg(test)]
#[path = "agent_read_cost_tests.rs"]
mod tests;
