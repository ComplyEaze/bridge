//! One stock item's Stock Query answer held against the stock-item collection's closing quantity
//! for the same item and period end (#2018). Stock Query gives the item's quantity per godown and
//! batch ([`crate::native_stock_query`]); the collection gives its closing. The only tie-out is
//! the lines' signed sum against that closing: no line is compared with any voucher, because a
//! Stock Journal moves stock between godowns without an invoice goods line (§12a.23).
//!
//! The empty answer is never zero on its own. Tally sends it for an item with no stock, for one
//! that never moved and for a name it does not know alike, so it is told apart only by the
//! collection's closing beside it: an explicit zero is `NoStock`, an empty or absent closing is
//! `NoMovement` (one sitting of one book, `stock-query/PROVENANCE.md`).
use crate::native_stock_query::{
    parse_stock_query, StockQueryAnswer, StockQueryError, StockQueryLine,
};
use crate::native_stock_summary::{sum_at_scale, NativeQuantityRead, NativeStockQuantity};
use crate::{export_status, TallyExportStatus};

/// Reads a Stock Query answer, taking Tally's shaped failure envelope (`HEADER` with `STATUS` 0)
/// as Tally's failure. The parser admits only the flat envelope Stock Query answers with, so it
/// refuses that shape at its first unknown element, and a person would be told the answer was
/// malformed when Tally said it failed. Tally answered a built-in report it could not find with
/// that envelope (`builtin_unknown_report_refusal_live`); no Stock Query answer has been seen in
/// it, so this mapping is UNVERIFIED for Stock Query.
pub fn read_stock_query(xml: &str) -> Result<StockQueryAnswer, StockQueryError> {
    parse_stock_query(xml).map_err(|error| match export_status(xml) {
        Ok(TallyExportStatus::Failure) => StockQueryError::TallyReportedFailure,
        _ => error,
    })
}

/// Why the lines and the closing could not be compared. Nothing is converted between units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotComparable {
    /// The closing is text the quantity grammar does not read: an item with an alternate unit,
    /// whose closing Tally writes as `<base> = <alternate>`, while Stock Query writes the base
    /// unit only.
    ClosingUnread,
    /// The units differ, among the lines or from the closing, compared as sent (case and dots
    /// included).
    UnitsDiffer,
}

impl NotComparable {
    pub const fn code(self) -> &'static str {
        match self {
            Self::ClosingUnread => "closing_unread",
            Self::UnitsDiffer => "units_differ",
        }
    }
}

/// What one item's answer and closing say together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StockQueryTie {
    /// The lines' signed sum, in their one unit, equals the closing in the same unit.
    Tied(NativeStockQuantity),
    /// The two sides disagree and neither is adjusted: lines whose sum is not the closing, lines
    /// against an empty or absent closing, or no lines against a closing that is not zero.
    /// `lines_total` is the lines' sum where they are in one unit, and `None` for no lines.
    Differs {
        lines_total: Option<NativeStockQuantity>,
    },
    NotComparable(NotComparable),
    /// The empty answer beside the collection's explicit zero for the item.
    NoStock,
    /// The empty answer beside the collection's empty or absent closing for the item.
    NoMovement,
}

impl StockQueryTie {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Tied(_) => "tied",
            Self::Differs { .. } => "differs",
            Self::NotComparable(_) => "not_comparable",
            Self::NoStock => "no_stock",
            Self::NoMovement => "no_movement",
        }
    }
}

/// A sum that `ExactDecimal` cannot hold. No captured quantity comes near it; it refuses that
/// item rather than report a wrong total.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StockSumUnrepresentable;

impl StockSumUnrepresentable {
    pub const fn code(self) -> &'static str {
        "stock_query_sum_unrepresentable"
    }
}

/// The signed sum of `lines`, which must share one unit; `None` for lines in several units.
fn sum(lines: &[&StockQueryLine]) -> Result<Option<NativeStockQuantity>, StockSumUnrepresentable> {
    let Some(first) = lines.first() else {
        return Ok(None);
    };
    let unit = &first.quantity.unit;
    if lines.iter().any(|line| &line.quantity.unit != unit) {
        return Ok(None);
    }
    // At the scale of the widest line, as Tally writes the unit: `200.000` and `200.000` are
    // `400.000`, not `400`.
    let amount = sum_at_scale(lines.iter().map(|line| &line.quantity.amount))
        .map_err(|_| StockSumUnrepresentable)?;
    Ok(Some(NativeStockQuantity {
        amount,
        unit: unit.clone(),
    }))
}

/// Holds `answer` against `closing`, the collection's closing for the same item and period end.
pub fn tie_stock_query(
    answer: &StockQueryAnswer,
    closing: &NativeQuantityRead,
) -> Result<StockQueryTie, StockSumUnrepresentable> {
    let lines = match answer {
        StockQueryAnswer::NoLines => {
            return Ok(match closing {
                NativeQuantityRead::Empty => StockQueryTie::NoMovement,
                NativeQuantityRead::Read(quantity) if quantity.amount.is_zero() => {
                    StockQueryTie::NoStock
                }
                NativeQuantityRead::Read(_) => StockQueryTie::Differs { lines_total: None },
                NativeQuantityRead::Unread => {
                    StockQueryTie::NotComparable(NotComparable::ClosingUnread)
                }
            });
        }
        StockQueryAnswer::Lines(lines) => lines.iter().collect::<Vec<_>>(),
    };
    let Some(total) = sum(&lines)? else {
        return Ok(StockQueryTie::NotComparable(NotComparable::UnitsDiffer));
    };
    Ok(match closing {
        NativeQuantityRead::Empty => StockQueryTie::Differs {
            lines_total: Some(total),
        },
        NativeQuantityRead::Unread => StockQueryTie::NotComparable(NotComparable::ClosingUnread),
        NativeQuantityRead::Read(closing) if closing.unit != total.unit => {
            StockQueryTie::NotComparable(NotComparable::UnitsDiffer)
        }
        NativeQuantityRead::Read(closing) if closing.amount.numeric_eq(&total.amount) => {
            StockQueryTie::Tied(total)
        }
        NativeQuantityRead::Read(_) => StockQueryTie::Differs {
            lines_total: Some(total),
        },
    })
}

/// One godown's lines, by their positions in the answer, and their sum where they share a unit.
/// It carries no name: a caller shows the godown through the first of its lines, so that a name
/// masked on the line is never a key beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GodownSubtotal {
    pub lines: Vec<usize>,
    pub quantity: Option<NativeStockQuantity>,
}

/// The lines grouped by godown, in the order each godown first appears. Godowns are compared as
/// Tally sent them: two names that differ only in case are two godowns, as the parser's repeat
/// check holds them.
pub fn godown_subtotals(
    lines: &[StockQueryLine],
) -> Result<Vec<GodownSubtotal>, StockSumUnrepresentable> {
    let mut groups: Vec<(&str, Vec<usize>)> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        match groups.iter_mut().find(|(godown, _)| *godown == line.godown) {
            Some((_, members)) => members.push(index),
            None => groups.push((&line.godown, vec![index])),
        }
    }
    groups
        .into_iter()
        .map(|(_, members)| {
            let quantity = sum(&members.iter().map(|&at| &lines[at]).collect::<Vec<_>>())?;
            Ok(GodownSubtotal {
                lines: members,
                quantity,
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "native_stock_by_batch_tests.rs"]
mod tests;
