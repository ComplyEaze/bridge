//! What one call of `vouchers`, `ledger_movement` or `verify_import` spends on its
//! census, stated before the call runs (bridge#1465, after #1239).
//!
//! The census a window read pays is set by the book's voucher mark, not by the
//! window (`census_spans`): a narrower window does not lower it, and N one-day calls
//! pay it N times. So the figure here has no window. It is the company's voucher
//! mark, read once through the marks read every windowed call opens with, put
//! through the same two rules the read applies (`census_read_count`), so what is
//! said here cannot differ from what the read does.
//!
//! Two kinds of figure, kept apart:
//! - the census-read count and the floor are **derived** from the mark and the
//!   gate's rule, and the floor is the one `read_cost.floor_seconds` states
//!   (`census_floor_ms`, one rule for both). The floor is the least time spent on
//!   the gaps between census reads of a call that is not refused earlier. It leaves
//!   out every data part and the wider re-read an empty window gets in `vouchers`
//!   and `ledger_movement`;
//! - the `measured` block is **constant text**: whole calls recorded in the public
//!   protocol reference, each with its host, date, confidence and "not a bound".
//!   Bridge computes no estimate from them. If one is wanted later it gets its own
//!   type; until then there is none, and no field is ever 0 for "not known".
//!
//! Nothing here refuses. The only refusal it can state is the one the read already
//! makes, `voucher_window_book_too_large`, and then no floor is given: a refused read sends no
//! census.
use super::read_cost::{census_floor_ms, host_limits, SPACING_MS};
use super::voucher_window::{
    census_read_count, VoucherReadShape, WindowReadLimits, MAX_CENSUS_READS,
};
use super::*;

/// What one call of a tool spends on census.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CensusCost {
    /// The call sends this many census reads (0 when the mark bounds the book).
    Reads(u64),
    /// The read refuses the book before any census, with this code.
    Refused(&'static str),
}

impl CensusCost {
    fn of(high_water: u64, limits: WindowReadLimits) -> Self {
        match census_read_count(high_water, limits) {
            Ok(reads) => Self::Reads(reads),
            Err(code) => Self::Refused(code),
        }
    }

    fn to_json(self) -> Value {
        match self {
            Self::Reads(reads) => json!({
                "state": "counted",
                "census_reads": reads,
                "floor_seconds": census_floor_ms(reads) / 1000,
                "floor_basis": "derived",
            }),
            Self::Refused(code) => json!({
                "state": "refused_by_allowance",
                "code": code,
                "limit_census_reads": MAX_CENSUS_READS,
            }),
        }
    }
}

/// The three tools and the limits each plans its window read under: the ones the
/// tool's own read uses. `vouchers` counts a small book (#1314), so it sends at
/// least one census read on any book that has held a voucher; the other two do
/// not (`SmallBooks::Skip`). A `vouchers` call with a type filter plans under the
/// class shape, whose default is the same, so its count is the same.
fn tools() -> [(&'static str, WindowReadLimits); 3] {
    [
        (
            "vouchers",
            WindowReadLimits::for_shape(VoucherReadShape::EntryWildcard).counting_small_books(),
        ),
        (
            "ledger_movement",
            WindowReadLimits::for_shape(VoucherReadShape::Movement),
        ),
        (
            "verify_import",
            WindowReadLimits::for_shape(VoucherReadShape::ImportVerification),
        ),
    ]
}

/// The measured calls the public protocol reference records, as data. Each
/// carries its source section, host, date and confidence, and says it is not a
/// bound. Nothing here is computed, and nothing is added to or scaled from these
/// figures. The book's mark is rounded, as the reference rounds it: an exact mark
/// identifies a book.
fn measured() -> Value {
    json!({
        "kind": "measured_calls_not_a_bound",
        "say": "These are whole calls recorded in the protocol reference, each on one book, with the census reads and data parts each made. The seconds are the whole call, not the census alone. Bridge computes no estimate from them. A different book, host or load can take a different time per census read. They are not a bound.",
        "points": [
            {
                "call": "vouchers",
                "window": "one day",
                "voucher_mark": "about 250,000",
                "census_reads": 31,
                "data_parts": 2,
                "seconds": 34.3,
                "book": "an inventory-heavy book",
                "host": "TallyPrime 7.1 Silver, licensed lab host; operating system not recorded",
                "date": "2026-09-21",
                "confidence": "verified",
                "source": "protocol reference 11c.5",
                "not_a_bound": true,
            },
            {
                "call": "ledger_movement",
                "window": "one month",
                "voucher_mark": "about 250,000",
                "census_reads": 31,
                "data_parts": 5,
                "seconds": 104.7,
                "book": "an inventory-heavy book",
                "host": "TallyPrime 7.1 Silver, licensed lab host; operating system not recorded",
                "date": "2026-09-21",
                "confidence": "verified",
                "source": "protocol reference 11c.5",
                "not_a_bound": true,
            },
        ],
    })
}

const LIMITATIONS: [&str; 7] = [
    "The figure is for one call, and it is as of this call's own read of the company's voucher mark. The call that follows reads the mark again, and a book that changed in between can differ.",
    "A narrower window does not lower the census count: the census follows the book's mark, not the window. Reading N days in N calls pays it N times. Only the data parts follow the window.",
    "The census count and the floor are derived from the mark and the gate's rules, not measured on this book. The floor is the least time spent on the half-second gap between census reads, for a call that is not refused earlier; it leaves out every data part, and the recorded runs under measured took more than twice their floor.",
    "No estimate is made. The measured block is constant text from the protocol reference and is not a bound.",
    "A `vouchers` or `ledger_movement` window with no row is read once more, a day wider on each side, which pays its own census on a large book and is not counted here. `ledger_movement` also reads its window twice; the second read sends no census.",
    "Nothing here is a limit or a refusal except `refused_by_allowance`, which is the refusal the read itself makes. Bridge is not told which host timeout applies: host_limits states what has been measured.",
    "The mark is read from Tally's company record; an absent voucher mark on a verified company means it has never held a voucher, and then no census is sent.",
];

/// The sentence the answer opens with, from numbers only.
fn say(mark: u64, costs: &[(&'static str, CensusCost)]) -> String {
    let parts = costs
        .iter()
        .map(|(tool, cost)| match cost {
            CensusCost::Reads(0) => format!("{tool} sends no census"),
            CensusCost::Reads(reads) => {
                let floor = census_floor_ms(*reads) / 1000;
                format!(
                    "{tool} sends at most {reads} census read{}, and a call that is not refused earlier spends at least {floor} second{} on the gaps between them (derived)",
                    if *reads == 1 { "" } else { "s" },
                    if floor == 1 { "" } else { "s" },
                )
            }
            CensusCost::Refused(code) => format!("{tool} is refused as {code}"),
        })
        .collect::<Vec<_>>()
        .join("; ");
    format!(
        "On a book with a voucher mark of {mark}, one call: {parts}. This is for one call whatever its window: a narrower window does not lower it, and N calls pay it N times. It is as of this read of the mark. Nothing is refused on it."
    )
}

impl Server {
    pub(super) async fn voucher_read_cost(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let (company, identity, prior) = self.verified_company(guid).await?;
        // The marks read every windowed call opens with: one paired read with an
        // identity check on either side, and no census or voucher request.
        let (marks, marks_evidence) = self
            .read_company_marks_once(&identity, &company.name)
            .await
            .map_err(|failure| failure.with_prior_evidence(prior.clone()))?;
        let evidence = combine_evidence(prior, marks_evidence);
        let costs = tools().map(|(tool, limits)| (tool, CensusCost::of(marks.vouchers(), limits)));
        let per_tool: serde_json::Map<String, Value> = costs
            .iter()
            .map(|(tool, cost)| ((*tool).to_string(), cost.to_json()))
            .collect();
        let payload = json!({
            "company": company_json(&company, std::slice::from_ref(&company)),
            "result": {
                "state": "observed",
                "basis": "voucher_mark_read_by_this_call",
                "voucher_mark": marks.vouchers(),
                "per_call": per_tool,
                "request_spacing_ms": SPACING_MS,
                "measured": measured(),
                "host_limits": host_limits(),
                "say": say(marks.vouchers(), &costs),
                "limitations": LIMITATIONS,
            },
        });
        Ok(ToolOutcome {
            payload,
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }
}

#[cfg(test)]
#[path = "agent_read_cost_preview_tests.rs"]
mod tests;
