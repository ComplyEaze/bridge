//! Closing stock by godown (#2017): Tally's own Godown Summary, requested by
//! report name, and the Stock Summary with the explode flag.
//!
//! A built-in report like the statements (protocol reference §12a.1): a success
//! carries no `HEADER` or `STATUS`, and nothing in the response identifies the
//! company, so a caller must bracket the read with identity and extent reads.
//!
//! The Godown Summary is the Stock Summary's grammar, one `DSPACCNAME` and
//! `DSPSTKINFO` pair per godown directly under the root and no line for a group
//! or an item (§12a.24). It is read by [`parse_native_godown_summary`] into its
//! own types, so a godown's line can never be taken for a stock group's or an
//! item's: its quantity is Tally's net of every item in that godown
//! ([`GodownNetQuantity`]), and a godown under another godown has no line of
//! its own, being folded into its parent.
//!
//! A value per item per godown comes only from the Stock Summary with the
//! explode flag, which writes a line per godown after an item that sits
//! directly under the root, each at that godown's own cost; an item inside a
//! stock group gets none (§12a.24). [`items_by_godown`] returns those lines only
//! where they add up to the item's own line.
use crate::{
    native_outstandings::NativeLedgerSnapshotPeriod,
    native_statement_reports::{
        render_built_in_report_request, render_exploded_stock_summary_request, BuiltInReport,
    },
    native_stock_summary::{
        parse_native_stock_summary_report, NativeExplodedLine, NativeQuantityRead,
        NativeStockError, NativeStockQuantity, NativeStockReport,
    },
};
use bridge_tally_primitives::ExactDecimal;

/// Renders the only supported request: Tally's own Godown Summary by name over
/// the period, for the named company. No TDL, and no variable that names a
/// godown.
pub fn render_native_godown_summary_request(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_built_in_report_request(BuiltInReport::GodownSummary, company, period)
}

/// The Stock Summary by name over the period with the explode flag, the one
/// request that carries a value per godown below the godown's own line.
pub fn render_native_stock_summary_exploded_request(
    company: &str,
    period: &NativeLedgerSnapshotPeriod,
) -> String {
    render_exploded_stock_summary_request(company, period)
}

/// A godown's closing quantity as the Godown Summary gives it. It is Tally's net
/// of every item in that godown that shares a unit, never one item's closing:
/// `316.667 Kgs` was one item's `366.667 Kgs` and another's `-50.000 Kgs`
/// (§12a.24).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GodownNetQuantity {
    NetAcrossItems(NativeStockQuantity),
    /// Present and empty. On the captures the godown held items in several
    /// units, but the cause is not measured: a stock group whose items were all
    /// in one unit was empty too.
    Empty,
    /// Text that is not `<number> <unit>`.
    Unread,
}

/// One line of the Godown Summary: a godown directly under the root, with any
/// godown under it folded in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeGodownLine {
    pub name: String,
    pub quantity: GodownNetQuantity,
    /// `None` where Tally sent an empty amount: not zero.
    pub amount: Option<ExactDecimal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeGodownSummary {
    Lines {
        /// The sum of the amounts present; `None` when every amount was empty.
        total: Option<ExactDecimal>,
        /// Amounts that were empty: counted, never read as zero.
        empty: usize,
        lines: Vec<NativeGodownLine>,
    },
    /// An empty `ENVELOPE`: not told apart from a report Tally did not render.
    Empty,
}

/// Parses the Godown Summary with the Stock Summary's grammar and refusals,
/// into godown types.
pub fn parse_native_godown_summary(xml: &str) -> Result<NativeGodownSummary, NativeStockError> {
    Ok(match parse_native_stock_summary_report(xml)? {
        NativeStockReport::Empty => NativeGodownSummary::Empty,
        NativeStockReport::Lines {
            total,
            empty,
            lines,
            ..
        } => NativeGodownSummary::Lines {
            total,
            empty,
            lines: lines
                .into_iter()
                .map(|line| NativeGodownLine {
                    name: line.name,
                    quantity: match line.quantity {
                        NativeQuantityRead::Read(quantity) => {
                            GodownNetQuantity::NetAcrossItems(quantity)
                        }
                        NativeQuantityRead::Empty => GodownNetQuantity::Empty,
                        NativeQuantityRead::Unread => GodownNetQuantity::Unread,
                    },
                    amount: line.amount,
                })
                .collect(),
        },
    })
}

/// How the godowns' amounts compare with the plain Stock Summary's total for the
/// same period.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeGodownTie {
    /// Every godown carries an amount, and they add up to the Stock Summary's
    /// total.
    Tied { total: ExactDecimal },
    /// Both totals are present and differ: the read refuses, with both.
    Differs {
        godown_total: ExactDecimal,
        stock_summary_total: ExactDecimal,
    },
    /// A total is missing, or a godown's amount is empty, so nothing ties.
    NotComparable {
        godown_total: Option<ExactDecimal>,
        stock_summary_total: Option<ExactDecimal>,
        godown_empty_amounts: usize,
    },
}

/// Ties the godowns to the Stock Summary. An empty godown amount is not zero, so
/// a summary with one never ties, whatever the sums say.
pub fn tie_godowns(godowns: &NativeGodownSummary, stock: &NativeStockReport) -> NativeGodownTie {
    let (godown_total, godown_empty_amounts) = match godowns {
        NativeGodownSummary::Lines { total, empty, .. } => (total.clone(), *empty),
        NativeGodownSummary::Empty => (None, 0),
    };
    let stock_summary_total = match stock {
        NativeStockReport::Lines { total, .. } => total.clone(),
        NativeStockReport::Empty => None,
    };
    match (godown_total, stock_summary_total) {
        (Some(godown_total), Some(stock_summary_total)) if godown_empty_amounts == 0 => {
            if godown_total.numeric_eq(&stock_summary_total) {
                NativeGodownTie::Tied {
                    total: stock_summary_total,
                }
            } else {
                NativeGodownTie::Differs {
                    godown_total,
                    stock_summary_total,
                }
            }
        }
        (godown_total, stock_summary_total) => NativeGodownTie::NotComparable {
            godown_total,
            stock_summary_total,
            godown_empty_amounts,
        },
    }
}

/// An item's closing in one godown, at that godown's own cost as Tally values
/// it: not the item's average rate times the quantity (§12a.24).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeItemGodownLine {
    pub godown: String,
    pub quantity: NativeQuantityRead,
    pub value: ExactDecimal,
}

/// Whether the godown quantities were added and compared with the item's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GodownQuantityCheck {
    /// Every quantity was read in the item's unit, and they add up to it.
    Added,
    /// A quantity was empty, unread or in another unit: not compared.
    NotCompared,
}

/// What one exploded line with godown sub-lines established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeItemByGodown {
    /// The godown values add up to the item's own value, and the plain Stock
    /// Summary has exactly one line of this name with that value.
    Tied {
        value: ExactDecimal,
        quantity: GodownQuantityCheck,
        godowns: Vec<NativeItemGodownLine>,
    },
    /// The godown values do not add up to the item's value: no line is returned.
    ValuesDiffer {
        item_value: ExactDecimal,
        godown_sum: ExactDecimal,
    },
    /// The godown quantities, all in the item's unit, do not add up to it.
    QuantitiesDiffer {
        item_quantity: NativeStockQuantity,
        godown_sum: ExactDecimal,
    },
    /// The item's value or a godown's value was empty: not zero, so not added.
    ValueEmpty,
    /// A sub-line names a batch. Every capture's batch was empty, so a batch
    /// line's meaning is not measured.
    BatchNamed,
    /// Two sub-lines name the same godown.
    GodownRepeated,
    /// The plain Stock Summary has no line of this name with this value, or more
    /// than one: the line is not shown to be an item under the root.
    NotInPlainReport,
}

impl NativeItemByGodown {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Tied { .. } => "tied",
            Self::ValuesDiffer { .. } => "values_differ",
            Self::QuantitiesDiffer { .. } => "quantities_differ",
            Self::ValueEmpty => "value_empty",
            Self::BatchNamed => "batch_named",
            Self::GodownRepeated => "godown_repeated",
            Self::NotInPlainReport => "plain_report_differs",
        }
    }
}

/// One exploded line that carried godown sub-lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeItemGodowns {
    pub item: String,
    pub outcome: NativeItemByGodown,
}

/// The exploded lines that carry godown sub-lines, each tied to its own line and
/// to the plain Stock Summary. A line without sub-lines (a stock group, or an
/// item inside one) is left out: it has no value per godown.
pub fn items_by_godown(
    exploded: &[NativeExplodedLine],
    plain: &NativeStockReport,
) -> Result<Vec<NativeItemGodowns>, NativeStockError> {
    let plain_lines = match plain {
        NativeStockReport::Lines { lines, .. } => lines.as_slice(),
        NativeStockReport::Empty => &[],
    };
    exploded
        .iter()
        .filter(|exploded| !exploded.sub_lines.is_empty())
        .map(|exploded| {
            let line = &exploded.line;
            let mut named = plain_lines.iter().filter(|plain| {
                plain.name == line.name
                    && match (&plain.amount, &line.amount) {
                        (Some(plain), Some(own)) => plain.numeric_eq(own),
                        (None, None) => true,
                        _ => false,
                    }
            });
            let in_plain_report = named.next().is_some() && named.next().is_none();
            Ok(NativeItemGodowns {
                item: line.name.clone(),
                outcome: if in_plain_report {
                    item_outcome(exploded)?
                } else {
                    NativeItemByGodown::NotInPlainReport
                },
            })
        })
        .collect()
}

fn item_outcome(exploded: &NativeExplodedLine) -> Result<NativeItemByGodown, NativeStockError> {
    let subs = &exploded.sub_lines;
    if subs.iter().any(|sub| sub.batch.is_some()) {
        return Ok(NativeItemByGodown::BatchNamed);
    }
    if subs
        .iter()
        .enumerate()
        .any(|(index, sub)| subs[..index].iter().any(|seen| seen.godown == sub.godown))
    {
        return Ok(NativeItemByGodown::GodownRepeated);
    }
    let (Some(item_value), Some(values)) = (
        exploded.line.amount.as_ref(),
        subs.iter()
            .map(|sub| sub.amount.as_ref())
            .collect::<Option<Vec<_>>>(),
    ) else {
        return Ok(NativeItemByGodown::ValueEmpty);
    };
    let godown_sum = crate::native_stock_summary::sum_at_scale(values.iter().copied())?;
    if !godown_sum.numeric_eq(item_value) {
        return Ok(NativeItemByGodown::ValuesDiffer {
            item_value: item_value.clone(),
            godown_sum,
        });
    }
    let quantity = match (
        exploded.line.quantity.read(),
        subs.iter()
            .map(|sub| sub.quantity.read())
            .collect::<Option<Vec<_>>>(),
    ) {
        (Some(own), Some(quantities)) if quantities.iter().all(|q| q.unit == own.unit) => {
            let sum = crate::native_stock_summary::sum_at_scale(
                quantities.iter().map(|quantity| &quantity.amount),
            )?;
            if !sum.numeric_eq(&own.amount) {
                return Ok(NativeItemByGodown::QuantitiesDiffer {
                    item_quantity: own.clone(),
                    godown_sum: sum,
                });
            }
            GodownQuantityCheck::Added
        }
        _ => GodownQuantityCheck::NotCompared,
    };
    Ok(NativeItemByGodown::Tied {
        value: item_value.clone(),
        quantity,
        godowns: subs
            .iter()
            .zip(values)
            .map(|(sub, value)| NativeItemGodownLine {
                godown: sub.godown.clone(),
                quantity: sub.quantity.clone(),
                value: value.clone(),
            })
            .collect(),
    })
}

#[cfg(test)]
#[path = "native_godown_summary_tests.rs"]
mod tests;
