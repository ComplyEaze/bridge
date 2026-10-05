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
//! - `fixed_seconds` and `per_voucher_ms` are **derived** from this call's own
//!   timings and one census row cost, measured once on one book.
//! - `host_240` is **derived** advice. It is taken with that row cost moved to the
//!   cautious side (so a window larger than one that fitted is costed more, a
//!   smaller one than one that did not fit is costed more), with 15 s kept back
//!   for the call's other reads, and bounded by the planner's voucher allowance
//!   as well as by the host limit. Its inputs are shown (`basis`) so it can be
//!   reproduced.
//! - the host limits are facts about hosts, each with its basis.
//!
//! A read that stopped (a refusal) gives the certain floor and no estimate: the
//! time of a request that failed or hung is not a cost of the window.
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
/// rounded), which is about 37 ms a row. One run, one host. This is the figure
/// of the central model (`fixed_seconds`, `per_voucher_ms`); the advice is taken
/// at the two figures below.
const CENSUS_MS_PER_ROW: u64 = 37;

/// Taken when the call did not fit and the advice is a smaller window: a census
/// that shrinks less per row than measured would make a narrower window cost
/// more than the model says, so the advice assumes two thirds of the measured
/// figure (24 ms).
const CENSUS_MS_PER_ROW_LOW: u64 = CENSUS_MS_PER_ROW * 2 / 3;

/// Taken when the call fitted and the advice is a larger window: a census that
/// grows more per row than measured would make a wider window cost more than the
/// model says, so the advice assumes one and a half times the measured figure
/// (55 ms).
const CENSUS_MS_PER_ROW_HIGH: u64 = CENSUS_MS_PER_ROW * 3 / 2;

/// The most vouchers one call can plan at the shape's default cost per voucher:
/// the reads the planner may dispatch, each no larger than the data budget. A
/// measured cost can plan more per read later in a window, but no advice is given
/// above what the first plan alone can carry.
fn voucher_allowance() -> u64 {
    let default_part = VoucherReadShape::EntryWildcard.default_wire_bytes_per_voucher();
    (WINDOW_READ_BUDGET_BYTES / default_part.max(1))
        .saturating_mul(u64::try_from(MAX_PLANNED_READS).unwrap_or(u64::MAX))
}

/// The one call limit that has been measured: Claude Desktop's chat app on
/// macOS, bundle 2.19675.0, cancelled a silent 250 s call at 240 s in two runs
/// (protocol reference 11f). Calls between 130 s and 240 s were not tried, and
/// whether progress notifications would extend the limit is not answered.
const DESKTOP_CALL_LIMIT_MS: u64 = 240_000;

/// What is kept back from the limit for the call's reads that this block does not
/// count (company check, ledger and type lists). Not measured on a large book; a
/// stated allowance, not a figure.
const CALL_RESERVE_MS: u64 = 15_000;

/// The limit the advice is planned against.
const PLANNING_LIMIT_MS: u64 = DESKTOP_CALL_LIMIT_MS - CALL_RESERVE_MS;

/// A call this slow is worth a statement: from here, thirty one-day calls take
/// ten minutes.
const NOTABLE_ELAPSED_MS: u64 = 20_000;

/// A call whose certain floor is this long is worth a statement even when it
/// stopped early (a read refused or failed in its census): thirty such calls
/// wait four minutes on the floor alone.
const NOTABLE_FLOOR_MS: u64 = 8_000;

/// The block is left out when it would take more than this share of the response
/// budget, as one part in `BUDGET_SHARE`: a result is carried twice (as
/// structured content and as text), so a sixteenth of the budget is an eighth of
/// what the host receives.
const BUDGET_SHARE: usize = 16;

/// Whether the read finished or stopped part-way. The lead sentence says so.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Ended {
    Read,
    Stopped,
}

/// The inputs of the advice, kept so that it can be reproduced from the result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Basis {
    census_row_ms: u64,
    fixed_ms: u64,
    per_voucher_ms: u64,
}

/// How a window of this book sits against the 240 s limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fit {
    /// This window fitted; `at_most` vouchers is the most one call should carry.
    WindowFits { at_most: u64, basis: Basis },
    /// This window did not fit; `at_most` vouchers would (none when even the
    /// cautious estimate leaves room for none).
    WindowTooLong { at_most: Option<u64>, basis: Basis },
    /// What every call pays alone is already at the planning limit, even on the
    /// estimate most favourable to a window.
    NoWindowFits,
    /// A read that stopped, or one that read no voucher: no advice.
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
            Self::WindowFits { at_most, .. } => Some(at_most),
            Self::WindowTooLong { at_most, .. } => at_most,
            Self::NoWindowFits | Self::NotEstablished => None,
        }
    }

    fn basis(self) -> Option<Basis> {
        match self {
            Self::WindowFits { basis, .. } | Self::WindowTooLong { basis, .. } => Some(basis),
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
    /// The central model: what every call pays, and one more voucher, at the
    /// measured census row cost. `None` for a read that stopped.
    central: Option<(u64, Option<u64>)>,
    fit: Fit,
}

/// What every call pays whatever its window, at `row_ms` a census row: the census
/// less its rows, never less than the certain floor, plus the marks.
fn fixed_at(census_ms: u64, vouchers: u64, floor_ms: u64, marks_ms: u64, row_ms: u64) -> u64 {
    census_ms
        .saturating_sub(vouchers.saturating_mul(row_ms))
        .max(floor_ms)
        .saturating_add(marks_ms)
}

/// The cost of one more voucher at `row_ms` a census row; `None` when no voucher
/// was read.
fn per_voucher_at(parts_ms: u64, vouchers: u64, row_ms: u64) -> Option<u64> {
    (vouchers > 0).then(|| parts_ms.div_ceil(vouchers).saturating_add(row_ms))
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
        let floor_ms = census_reads.saturating_mul(SPACING_MS);
        let total_ms = marks_ms.saturating_add(census_ms).saturating_add(parts_ms);
        let (central, fit) = match ended {
            // A read that stopped has requests that failed or hung in its times:
            // only the certain floor is stated.
            Ended::Stopped => (None, Fit::NotEstablished),
            Ended::Read => (
                Some((
                    fixed_at(census_ms, vouchers, floor_ms, marks_ms, CENSUS_MS_PER_ROW),
                    per_voucher_at(parts_ms, vouchers, CENSUS_MS_PER_ROW),
                )),
                fit_of(census_ms, parts_ms, vouchers, floor_ms, marks_ms, total_ms),
            ),
        };
        Self {
            census_reads,
            vouchers,
            marks_ms,
            census_ms,
            parts_ms,
            floor_ms,
            central,
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

/// How a window sits against the limit, for a read that finished.
fn fit_of(
    census_ms: u64,
    parts_ms: u64,
    vouchers: u64,
    floor_ms: u64,
    marks_ms: u64,
    total_ms: u64,
) -> Fit {
    let fitted = total_ms <= DESKTOP_CALL_LIMIT_MS;
    // The advice for a larger window costs the census row at the high figure; for
    // a smaller one, at the low figure.
    let census_row_ms = if fitted {
        CENSUS_MS_PER_ROW_HIGH
    } else {
        CENSUS_MS_PER_ROW_LOW
    };
    // No window fits only if even the estimate most favourable to a window (the
    // census row at its high figure, which leaves the least fixed cost) leaves
    // what every call pays at the planning limit or above.
    let most_favourable_fixed = fixed_at(
        census_ms,
        vouchers,
        floor_ms,
        marks_ms,
        CENSUS_MS_PER_ROW_HIGH,
    );
    if most_favourable_fixed >= PLANNING_LIMIT_MS {
        return Fit::NoWindowFits;
    }
    let Some(per_voucher_ms) = per_voucher_at(parts_ms, vouchers, census_row_ms) else {
        return Fit::NotEstablished;
    };
    let basis = Basis {
        census_row_ms,
        fixed_ms: fixed_at(census_ms, vouchers, floor_ms, marks_ms, census_row_ms),
        per_voucher_ms,
    };
    // Bounded by both limits: the host's, and what the planner can carry in one call.
    let room = PLANNING_LIMIT_MS.saturating_sub(basis.fixed_ms);
    let at_most = (room / per_voucher_ms.max(1)).min(voucher_allowance());
    if fitted {
        // What demonstrably fitted is never advised against, however cautious
        // the model.
        return Fit::WindowFits {
            at_most: at_most.max(vouchers),
            basis,
        };
    }
    Fit::WindowTooLong {
        at_most: (at_most > 0).then_some(at_most),
        basis,
    }
}

/// Whole seconds, rounded to the nearest (an observed figure).
fn nearest_seconds(value_ms: u64) -> u64 {
    value_ms.saturating_add(500) / 1000
}

/// The block, or `None` when the call was quick enough that nothing needs
/// saying, or when it would take more than a sixteenth of the response budget
/// (`max_bytes`): guidance is kept out of a small response budget.
pub(super) fn read_cost(
    timings: &WindowReadTimings,
    ended: Ended,
    max_bytes: usize,
) -> Option<Value> {
    let cost = Cost::of(timings, ended);
    if !cost.notable() {
        return None;
    }
    let mut host_240 = json!({"state": cost.fit.state()});
    if let Some(at_most) = cost.fit.at_most() {
        host_240["vouchers_at_most"] = json!(at_most);
    }
    if let Some(basis) = cost.fit.basis() {
        // The inputs of the advice, so that `vouchers_at_most` is
        // (planning limit - fixed) / per voucher, bounded by the allowance.
        host_240["basis"] = json!({
            "census_ms_per_row": basis.census_row_ms,
            "fixed_ms": basis.fixed_ms,
            "per_voucher_ms": basis.per_voucher_ms,
            "planning_limit_ms": PLANNING_LIMIT_MS,
        });
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
        // A lower bound by construction, rounded down.
        "floor_seconds": cost.floor_ms / 1000,
        "estimate": {
            "kind": "derived",
            // The central model at the measured census row cost (about 37 ms): what
            // every call pays whatever its window (rounded up), and the cost of one
            // more voucher. Null for a read that stopped.
            "fixed_seconds": cost.central.map(|(fixed, _)| fixed.div_ceil(1000)),
            "per_voucher_ms": cost.central.and_then(|(_, per_voucher)| per_voucher),
            "host_240": host_240,
        },
        "host_limits": [
            {"host": "claude_desktop_chat_macos", "seconds": 240, "basis": "measured_twice_one_build_silent_calls"},
            {"host": "claude_desktop_chat_windows", "seconds": null, "basis": "unmeasured"},
            {"host": "claude_code", "seconds": null, "basis": "completed_150s_by_default_one_run_per_two_builds_one_earlier_60s_unexplained"},
        ],
        "say": say(&cost, ended),
    });
    (block.to_string().len() <= max_bytes / BUDGET_SHARE).then_some(block)
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
    let paid = match (ended, cost.central) {
        (Ended::Read, Some((fixed_ms, _))) if cost.census_reads > 0 => format!(
            "About {} seconds (derived) is paid on every call, however short the window, because this book needs {} census reads; at least {} of those seconds are certain (the wait between reads).",
            fixed_ms.div_ceil(1000),
            cost.census_reads,
            cost.floor_ms / 1000
        ),
        (Ended::Read, Some((fixed_ms, _))) => format!(
            "About {} seconds (derived) is paid on every call, however short the window.",
            fixed_ms.div_ceil(1000)
        ),
        _ => format!(
            "At least {} census reads were sent, so at least {} seconds is paid on every call (the wait between reads).",
            cost.census_reads,
            cost.floor_ms / 1000
        ),
    };
    let fit = match cost.fit {
        Fit::WindowFits { at_most, .. } => format!(
            "Claude Desktop's chat app stops a silent call at 240 seconds (measured twice, on one Mac build): with 15 seconds kept back for the call's other reads, one call there can carry about {at_most} vouchers (derived), so use the widest window within that, not many short calls."
        ),
        Fit::WindowTooLong { at_most: Some(at_most), .. } => format!(
            "That is past 240 seconds, where Claude Desktop's chat app stops a silent call (measured twice, on one Mac build): read about {at_most} vouchers or fewer per call (derived). For totals over a long period read trial_balance, which reads no vouchers."
        ),
        Fit::WindowTooLong { at_most: None, .. } => "That is past 240 seconds, where Claude Desktop's chat app stops a silent call (measured twice, on one Mac build), and the figures cannot say how many vouchers a call can carry: read a shorter window, or for totals over a long period read trial_balance, which reads no vouchers.".to_string(),
        Fit::NoWindowFits => "On a host that stops a silent call at 240 seconds (Claude Desktop's chat app, measured twice on one Mac build) no window of this book fits, because what every call pays is already that long; do not suggest one. For totals over a long period read trial_balance, which reads no vouchers.".to_string(),
        Fit::NotEstablished => match ended {
            Ended::Read => "No voucher was read, so how many one call can carry is not established. A window with no voucher is also read once more, a day wider on each side, to confirm it is empty; that read is not in these figures and costs about as much again (derived).".to_string(),
            Ended::Stopped => "The read stopped, so how many vouchers one call can carry is not established.".to_string(),
        },
    };
    format!("{lead} {paid} {fit}")
}

#[cfg(test)]
#[path = "agent_read_cost_tests.rs"]
mod tests;
