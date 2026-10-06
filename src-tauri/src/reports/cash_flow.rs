//! The check that stands between Tally's own Cash Flow and a caller: its net
//! total against the movement of the book's cash and bank ledgers in the
//! native Trial Balance for the same window. No Tally I/O belongs here.
//!
//! The Cash Flow is Tally's figure and is returned as Tally printed it; what
//! this module decides is whether Bridge may call that figure checked. The
//! check is on the **net**, and only the net: the sum of the months' closing
//! amounts must equal the sum of `debit + credit` over every ledger whose
//! predefined group is Cash-in-Hand or Bank Accounts (a debit negative, a
//! credit positive: the trial balance's convention, protocol reference §5.6; the
//! sign and meaning of the Cash Flow's own credit column are not measured, and
//! its closing figure was measured only where the credit column was empty). A
//! contra between two such ledgers moves both and nets to nothing, so it cannot
//! make the tie fail or pass.
//!
//! What it deliberately does not do:
//! - **Bank OD A/c and Bank OCC A/c.** Whether Tally's Cash Flow counts a
//!   ledger under them is not measured. A ledger under either with movement in
//!   the window stops the check (`MoneyGroupUnmeasured`) instead of being
//!   guessed at; one without movement is irrelevant to the net.
//! - **The gross columns and the split by month.** The sum of the months can
//!   tie while one month is wrong, and the debit and credit columns depend on
//!   how Tally treats a contra; neither is checked here.
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

/// The predefined groups whose ledgers Tally's Cash Flow was measured to count:
/// a ledger under one of them (or under a group a user made inside one) is in
/// the money set. The same identities the bank import's group table names.
const MEASURED_MONEY_GROUPS: [&str; 2] = ["Bank Accounts", "Cash-in-Hand"];

/// Money groups whose treatment in the Cash Flow is not measured.
const UNMEASURED_MONEY_GROUPS: [&str; 2] = ["Bank OD A/c", "Bank OCC A/c"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CashFlowCheck {
    /// The months' closing amounts add up to the cash and bank ledgers' movement.
    Tied {
        net: ExactDecimal,
        /// How many ledgers made up the money set; zero says there was nothing
        /// to compare, not that something was checked.
        money_ledgers: usize,
    },
    /// The two figures differ. Nothing about the Cash Flow is checked.
    Differs {
        tally_net: ExactDecimal,
        ledger_net: ExactDecimal,
        money_ledgers: usize,
        /// Ledgers whose group could not be resolved to a predefined identity
        /// and that carry movement: where to look when the figures differ.
        unclassified_with_movement: usize,
    },
    /// A ledger under Bank OD A/c or Bank OCC A/c has movement in the window,
    /// and whether Tally's Cash Flow counts it is not measured.
    MoneyGroupUnmeasured { ledgers: usize },
    /// The trial balance holds no ledger under Cash-in-Hand or Bank Accounts and
    /// Tally's Cash Flow is empty: both sides are zero, and nothing was compared.
    NothingToCompare,
}

impl CashFlowCheck {
    /// The code a refusal carries; `None` when the check held.
    pub(crate) fn refusal_code(&self) -> Option<&'static str> {
        match self {
            Self::Tied { .. } => None,
            Self::Differs { .. } => Some("cash_flow_differs_from_trial_balance"),
            Self::MoneyGroupUnmeasured { .. } => Some("cash_flow_money_group_unmeasured"),
            Self::NothingToCompare => Some("cash_flow_no_money_ledger"),
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

/// Adds `value` to `sum`, keeping the most decimals any term carried.
fn add(
    sum: &mut ExactDecimal,
    scale: &mut usize,
    value: &ExactDecimal,
) -> Result<(), CashFlowCheckError> {
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
    let mut money_ledgers = 0_usize;
    let mut unmeasured = 0_usize;
    let mut unclassified_with_movement = 0_usize;
    for row in &trial_balance.report().rows {
        match index.reserved_ancestor(row.parent.nonempty_returned_text()) {
            Ok(reserved) if MEASURED_MONEY_GROUPS.contains(&reserved.trim()) => {
                money_ledgers += 1;
                for amount in [&row.debit, &row.credit] {
                    if let NativeTrialBalanceAmount::Present(value) = amount {
                        add(&mut ledger_net, &mut scale, value)?;
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
            add(&mut tally_net, &mut scale, value)?;
        }
    }
    let tally_net = at_scale(tally_net, scale)?;
    let ledger_net = at_scale(ledger_net, scale)?;
    // Two zeros over no ledger at all agree about nothing: it is not a tie.
    if money_ledgers == 0 && tally_net.is_zero() {
        return Ok(CashFlowCheck::NothingToCompare);
    }
    if tally_net.numeric_eq(&ledger_net) {
        Ok(CashFlowCheck::Tied {
            net: tally_net,
            money_ledgers,
        })
    } else {
        Ok(CashFlowCheck::Differs {
            tally_net,
            ledger_net,
            money_ledgers,
            unclassified_with_movement,
        })
    }
}

#[cfg(test)]
#[path = "cash_flow_tests.rs"]
mod tests;
