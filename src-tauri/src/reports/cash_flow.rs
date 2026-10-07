//! The check that stands between Tally's own Cash Flow and a caller: its net
//! total against the movement of the book's cash and bank ledgers in the
//! native Trial Balance for the same window. No Tally I/O belongs here.
//!
//! The Cash Flow is Tally's figure and is returned as Tally printed it; what
//! this module decides is whether Bridge may call that figure checked. The
//! check is on the **net**, and only the net: the sum of the months' closing
//! amounts must equal the sum of `debit + credit` over every ledger whose
//! predefined group is Cash-in-Hand or Bank Accounts (a debit negative, a
//! credit positive: the trial balance's convention, protocol reference §5.6; on
//! every captured month with an amount Tally's closing was the debit plus the
//! credit, but the Cash Flow's own debit and credit columns each differed from
//! the ledgers' totals while the net tied). On
//! the trial balance side a contra between two such ledgers moves both and nets
//! to nothing; how Tally's own Cash Flow prints a contra is not established. Both
//! sides must carry an amount: an empty amount is not a zero, and a side with
//! none is not compared (`Differs` or `NothingToCompare`).
//!
//! What it deliberately does not do:
//! - **Bank OD A/c and Bank OCC A/c.** Tally's Cash Flow was seen counting one
//!   Bank OD ledger (a debit, one book); a credit and a Bank OCC ledger were not
//!   measured. A ledger under either with movement in
//!   the window stops the check (`MoneyGroupUnmeasured`) instead of being
//!   guessed at or counted; one without movement is irrelevant to the net.
//! - **The gross columns and the split by month.** The sum of the months can
//!   tie while one month is wrong, and the debit and credit columns each differed from the ledgers' totals while the
//!   net tied (a contra is the unverified reading); neither is checked here.
//! - **A ledger it cannot classify.** It is left out of the money set and
//!   counted when it carries movement: if it was in fact cash or bank, the tie
//!   fails, and the count says where to look. A tie that holds anyway is not
//!   weakened by it.
use crate::tally::runtime::SingleCurrencyTrialBalance;
use bridge_tally_core::ExactDecimal;
use bridge_tally_protocol::{
    group_ancestry::GroupIndex,
    native_cash_flow::NativeCashFlow,
    native_statement_reports::NativeStatementAmount,
    native_trial_balance::{NativeTrialBalanceAmount, NativeTrialBalanceRow},
    TallyNamedMaster,
};

/// The predefined groups whose ledgers are taken as cash and bank: a ledger under
/// one of them (or under a group a user made inside one) is in the money set. The
/// book measured had its cash and bank ledgers directly under the two groups; a
/// ledger under a user's sub-group is counted by the group tree and was not
/// measured (a wrong guess fails safe as a difference). The same identities the
/// bank import's group table names.
const MEASURED_MONEY_GROUPS: [&str; 2] = ["Bank Accounts", "Cash-in-Hand"];

/// Money groups the check does not count in the net although Tally's Cash Flow was
/// seen counting a Bank OD ledger. In the group
/// tree captured on 7.1 `Bank OCC A/c` is a language alias of the group whose
/// reserved name is `Bank OD A/c`, so a lookup by reserved name returns the
/// latter; the second entry covers a build that names it separately.
const UNMEASURED_MONEY_GROUPS: [&str; 2] = ["Bank OD A/c", "Bank OCC A/c"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CashFlowCheck {
    /// The months' closing amounts add up to the cash and bank ledgers' movement.
    Tied {
        net: ExactDecimal,
        /// How many ledgers made up the money set (at least one).
        money_ledgers: usize,
    },
    /// The two figures differ, or only one side carries an amount. Nothing about
    /// the Cash Flow is checked.
    Differs {
        tally_net: ExactDecimal,
        ledger_net: ExactDecimal,
        /// Whether the months of Tally's Cash Flow carried any amount; when not,
        /// `tally_net` is a zero standing for nothing.
        tally_amounts: bool,
        /// Whether any cash or bank ledger carried an amount; when not,
        /// `ledger_net` is a zero standing for nothing.
        ledger_amounts: bool,
        money_ledgers: usize,
        /// Ledgers whose group could not be resolved to a predefined identity
        /// and that carry movement: where to look when the figures differ.
        unclassified_with_movement: usize,
    },
    /// A ledger under Bank OD A/c or Bank OCC A/c has movement in the window,
    /// and Tally's Cash Flow was seen counting one such ledger, which this check
    /// does not count.
    MoneyGroupUnmeasured { ledgers: usize },
    /// Nothing to tie: neither side carries an amount (no cash or bank ledger
    /// with an amount, and every month of Tally's Cash Flow empty; an empty
    /// amount is not zero, §7), or both sides net to zero, which would be equal
    /// under any sign convention and any meaning of the columns.
    NothingToCompare,
}

impl CashFlowCheck {
    /// The code a refusal carries; `None` when the check held.
    pub(crate) fn refusal_code(&self) -> Option<&'static str> {
        match self {
            Self::Tied { .. } => None,
            Self::Differs { .. } => Some("cash_flow_differs_from_trial_balance"),
            Self::MoneyGroupUnmeasured { .. } => Some("cash_flow_money_group_unmeasured"),
            Self::NothingToCompare => Some("cash_flow_nothing_to_compare"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub(crate) enum CashFlowCheckError {
    #[error("cash_flow_sum_invalid")]
    SumInvalid,
}

impl CashFlowCheckError {
    pub(crate) fn code(self) -> &'static str {
        match self {
            Self::SumInvalid => "cash_flow_sum_invalid",
        }
    }
}

fn present_nonzero(amount: &NativeTrialBalanceAmount) -> bool {
    matches!(amount, NativeTrialBalanceAmount::Present(value) if !value.is_zero())
}

fn has_movement(row: &NativeTrialBalanceRow) -> bool {
    present_nonzero(&row.debit) || present_nonzero(&row.credit)
}

fn decimals(text: &str) -> usize {
    text.split_once('.')
        .map_or(0, |(_, fraction)| fraction.len())
}

/// Adds `value` to `sum`, keeping the most decimals any term carried and
/// counting the amount as observed on its side.
fn add(
    sum: &mut ExactDecimal,
    scale: &mut usize,
    observed: &mut usize,
    value: &ExactDecimal,
) -> Result<(), CashFlowCheckError> {
    *observed += 1;
    *scale = (*scale).max(decimals(value.as_str()));
    *sum = sum
        .checked_add(value)
        .map_err(|_| CashFlowCheckError::SumInvalid)?;
    Ok(())
}

/// A sum shown at the scale of its terms: the arithmetic drops trailing zeros,
/// and a money total reads as `-4950.00`, not `-4950`.
fn at_scale(sum: ExactDecimal, scale: usize) -> Result<ExactDecimal, CashFlowCheckError> {
    let text = sum.as_str();
    let missing = scale.saturating_sub(decimals(text));
    if missing == 0 {
        return Ok(sum);
    }
    let point = if text.contains('.') { "" } else { "." };
    ExactDecimal::parse(format!("{text}{point}{}", "0".repeat(missing)))
        .map_err(|_| CashFlowCheckError::SumInvalid)
}

/// Checks Tally's Cash Flow for a window against the Trial Balance and group
/// tree read for the same window inside one identity and book-extent bracket.
pub(crate) fn check_cash_flow(
    trial_balance: &SingleCurrencyTrialBalance,
    groups: &[TallyNamedMaster],
    cash_flow: &NativeCashFlow,
) -> Result<CashFlowCheck, CashFlowCheckError> {
    let index = GroupIndex::build(groups.iter().cloned());
    let mut ledger_net = ExactDecimal::zero();
    let mut scale = 0_usize;
    // Amounts that were present on each side; empty ones are not counted.
    let mut ledger_observed = 0_usize;
    let mut tally_observed = 0_usize;
    let mut money_ledgers = 0_usize;
    let mut unmeasured = 0_usize;
    let mut unclassified_with_movement = 0_usize;
    for row in &trial_balance.report().rows {
        match index.reserved_ancestor(row.parent.nonempty_returned_text()) {
            Ok(reserved) if MEASURED_MONEY_GROUPS.contains(&reserved.trim()) => {
                money_ledgers += 1;
                for amount in [&row.debit, &row.credit] {
                    if let NativeTrialBalanceAmount::Present(value) = amount {
                        add(&mut ledger_net, &mut scale, &mut ledger_observed, value)?;
                    }
                }
            }
            Ok(reserved) if UNMEASURED_MONEY_GROUPS.contains(&reserved.trim()) => {
                if has_movement(row) {
                    unmeasured += 1;
                }
            }
            Ok(_) => {}
            Err(_) => {
                if has_movement(row) {
                    unclassified_with_movement += 1;
                }
            }
        }
    }
    if unmeasured > 0 {
        return Ok(CashFlowCheck::MoneyGroupUnmeasured {
            ledgers: unmeasured,
        });
    }
    let mut tally_net = ExactDecimal::zero();
    for row in &cash_flow.rows {
        if let NativeStatementAmount::Present(value) = &row.closing {
            add(&mut tally_net, &mut scale, &mut tally_observed, value)?;
        }
    }
    let tally_net = at_scale(tally_net, scale)?;
    let ledger_net = at_scale(ledger_net, scale)?;
    // Two sides with no amount at all agree about nothing: it is not a tie.
    if tally_observed == 0 && ledger_observed == 0 {
        return Ok(CashFlowCheck::NothingToCompare);
    }
    // A tie needs an amount on both sides: an empty side is not a zero, and months
    // that net to zero over no cash or bank ledger at all (no ledger amount) are
    // Tally printing figures nothing accounts for.
    let both_sides_have_amounts = tally_observed > 0 && ledger_observed > 0;
    // Two zero nets are equal under any sign convention and any meaning of the
    // columns, so they show nothing about either: not a tie.
    if both_sides_have_amounts && tally_net.is_zero() && ledger_net.is_zero() {
        return Ok(CashFlowCheck::NothingToCompare);
    }
    if both_sides_have_amounts && tally_net.numeric_eq(&ledger_net) {
        Ok(CashFlowCheck::Tied {
            net: tally_net,
            money_ledgers,
        })
    } else {
        Ok(CashFlowCheck::Differs {
            tally_net,
            ledger_net,
            tally_amounts: tally_observed > 0,
            ledger_amounts: ledger_observed > 0,
            money_ledgers,
            unclassified_with_movement,
        })
    }
}

#[cfg(test)]
#[path = "cash_flow_tests.rs"]
mod tests;
