//! The cash-book projection a deposit from the firm's own cash box is built on
//! (#1761). A deposit lowers its cash ledger on its date and on every later day,
//! so the build reads the ledger's day-end balances and admits the deposit only
//! if none of them goes below zero.
//!
//! Tally's Trial Balance gives the ledger's opening at the first day and its
//! closing at the last; the ledger's vouchers give the days between. They must
//! agree. Tally counts a post-dated voucher in a closing dated after it, whatever
//! its current date, and the voucher window lists it on its own date, so the two
//! tie (`docs/tally/TALLY_PROTOCOL_REFERENCE.md` §12a.26).

use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_core::{ExactDecimal, TallyDate};
use bridge_tally_protocol::{
    native_trial_balance::{NativeTrialBalanceAmount, NativeTrialBalanceRow},
    outstandings_shared::CompanyBookExtent,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::super::bank_statement::AnsweredCashLedger;
use super::super::{evidence_from_runtime_read, VerifiedCompanyIdentity};
use super::{
    bill_wise::hex, combine_evidence, normalized_date, party_name, EntrySide, Evidence,
    ImportVoucher, Server, ToolFailure,
};
use crate::tally::runtime::{TrialBalanceCurrencyScope, TrialBalancePeriod, TrialBalanceRead};

/// The book a deposit was projected on has moved.
pub(super) const CASH_PROJECTION_STALE: &str = "cash_projection_stale";

/// All a built projection claims, in the build's answer.
const CLAIM: &str = "At the build, no day-end balance of these cash ledgers from the deposit's date to the last date goes below zero after these deposits. Proposals not yet posted, later entries, movements within a day and anything dated after the last date are not counted.";

const REFUSED_NEXT_STEP: &str = "No file was written. A balance is debit positive, so a negative one is a credit balance. Correct the cash book in Tally, or answer this line again with parse_bank_statement if the deposit did not come from this ledger, then build again.";

/// One deposit a batch takes out of a cash ledger.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CashOutflow {
    ledger: String,
    date: TallyDate,
    amount: ExactDecimal,
}

/// The file's own cash outflows: each voucher answered as a deposit from the cash
/// box, for its leg out of the named ledger. Its cash inflows are not counted.
pub(super) fn outflows_of(
    vouchers: &[ImportVoucher],
    answered: &[AnsweredCashLedger],
) -> Result<Vec<CashOutflow>, String> {
    let mut outflows = Vec::new();
    for need in answered.iter().filter(|need| need.cash_out) {
        let leg = vouchers
            .iter()
            .filter(|voucher| voucher.bridge_txn_id == need.bridge_txn_id)
            .find_map(|voucher| {
                voucher
                    .entries
                    .iter()
                    .find(|entry| entry.side == EntrySide::Cr && entry.ledger == need.ledger)
                    .map(|entry| (voucher, entry))
            });
        let Some((voucher, entry)) = leg else {
            return Err("proposals_file_invalid".to_string());
        };
        outflows.push(CashOutflow {
            ledger: need.ledger.clone(),
            date: voucher.date.clone(),
            amount: entry.amount.decimal().clone(),
        });
    }
    Ok(outflows)
}

/// Why the book does not establish a deposit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum CashProjectionRefusal {
    /// The ledger is not in the Trial Balance, or Tally left an amount empty.
    BalanceNotRead {
        ledger: String,
    },
    /// Opening and vouchers do not reach Tally's closing: a voucher Tally counts
    /// was not read, or one it does not count was.
    NotTied {
        ledger: String,
        from_vouchers: ExactDecimal,
        tally_closing: ExactDecimal,
    },
    /// Below zero on a day from the deposit's date with no deposit counted.
    AlreadyNegative {
        ledger: String,
        date: String,
        balance: ExactDecimal,
    },
    /// Below zero on a day once this file's deposits are counted.
    WouldGoNegative {
        ledger: String,
        date: String,
        balance: ExactDecimal,
    },
    AmountInvalid,
}

impl CashProjectionRefusal {
    pub(super) fn code(&self) -> &'static str {
        match self {
            Self::BalanceNotRead { .. } => "cash_projection_balance_not_read",
            Self::NotTied { .. } => "cash_projection_not_tied",
            Self::AlreadyNegative { .. } => "cash_book_already_negative",
            Self::WouldGoNegative { .. } => "cash_projection_negative",
            Self::AmountInvalid => "voucher_amount_invalid",
        }
    }

    pub(super) fn to_json(&self) -> Value {
        let mut result = json!({
            "state": "refused",
            "reason": self.code(),
            "next_step": REFUSED_NEXT_STEP,
        });
        match self {
            Self::BalanceNotRead { ledger } => result["ledger"] = json!(party_name(ledger)),
            Self::NotTied {
                ledger,
                from_vouchers,
                tally_closing,
            } => {
                result["ledger"] = json!(party_name(ledger));
                result["from_vouchers"] = json!(from_vouchers.as_str());
                result["tally_closing"] = json!(tally_closing.as_str());
            }
            Self::AlreadyNegative {
                ledger,
                date,
                balance,
            }
            | Self::WouldGoNegative {
                ledger,
                date,
                balance,
            } => {
                result["ledger"] = json!(party_name(ledger));
                result["date"] = json!(date);
                result["balance"] = json!(balance.as_str());
            }
            Self::AmountInvalid => {}
        }
        result
    }
}

/// A projection that failed: the read did not complete, or the book refused it.
pub(super) enum CashProjectionFailure {
    Read(ToolFailure),
    Refused(CashProjectionRefusal),
}

impl From<CashProjectionFailure> for ToolFailure {
    fn from(failure: CashProjectionFailure) -> Self {
        match failure {
            CashProjectionFailure::Read(failure) => failure,
            CashProjectionFailure::Refused(refusal) => refusal.code().to_string().into(),
        }
    }
}

impl From<ToolFailure> for CashProjectionFailure {
    fn from(failure: ToolFailure) -> Self {
        Self::Read(failure)
    }
}

impl From<CashProjectionRefusal> for CashProjectionFailure {
    fn from(refusal: CashProjectionRefusal) -> Self {
        Self::Refused(refusal)
    }
}

/// A cash ledger's balance at the end of each day it moved, debit positive. It
/// exists only once its opening and vouchers reach Tally's own closing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct BookDayEnds {
    ledger: String,
    opening: ExactDecimal,
    days: BTreeMap<String, ExactDecimal>,
}

impl BookDayEnds {
    pub(super) fn tied(
        ledger: &str,
        opening: ExactDecimal,
        moves: &BTreeMap<String, ExactDecimal>,
        tally_closing: ExactDecimal,
    ) -> Result<Self, CashProjectionRefusal> {
        let mut balance = opening.clone();
        let mut days = BTreeMap::new();
        for (day, change) in moves {
            balance = balance
                .checked_add(change)
                .map_err(|_| CashProjectionRefusal::AmountInvalid)?;
            days.insert(day.clone(), balance.clone());
        }
        if !balance.numeric_eq(&tally_closing) {
            return Err(CashProjectionRefusal::NotTied {
                ledger: ledger.to_string(),
                from_vouchers: balance,
                tally_closing,
            });
        }
        Ok(Self {
            ledger: ledger.to_string(),
            opening,
            days,
        })
    }
}

/// What a saved batch keeps of the book it was built on.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct CashProjectionRecord {
    pub(super) ledgers: Vec<String>,
    pub(super) from: TallyDate,
    pub(super) to: TallyDate,
    /// Of the day-end table, not of the responses: the reads may fall in other
    /// parts, and a voucher away from cash is not a change to this table.
    pub(super) book_sha256: String,
}

fn book_sha256(from: &TallyDate, to: &TallyDate, books: &[BookDayEnds]) -> String {
    let table = serde_json::to_vec(&(from, to, books)).expect("a day-end table serialises");
    hex(&Sha256::digest(table))
}

/// The projection a Contra from the cash box is built from: only [`established`]
/// makes one, and it fails on any day that goes below zero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct EstablishedCashProjection {
    record: CashProjectionRecord,
    lowest: Vec<LowestDayEnd>,
}

/// A ledger's lowest day-end once its deposits are counted.
#[derive(Clone, Debug, PartialEq, Eq)]
struct LowestDayEnd {
    ledger: String,
    date: String,
    balance: ExactDecimal,
}

impl EstablishedCashProjection {
    pub(super) fn record(&self) -> &CashProjectionRecord {
        &self.record
    }

    pub(super) fn to_json(&self) -> Value {
        json!({
            "from": self.record.from,
            "to": self.record.to,
            "lowest_day_end_after_deposits": self.lowest.iter().map(|low| json!({
                "ledger": party_name(&low.ledger),
                "date": low.date,
                "balance": low.balance.as_str(),
            })).collect::<Vec<_>>(),
            "claim": CLAIM,
        })
    }
}

/// Each ledger's day-ends with the deposits taken off, from the ledger's first
/// deposit on: a deposit lowers its date and every later day. A book already
/// below zero says so whatever the deposits would do.
pub(super) fn established(
    from: TallyDate,
    to: TallyDate,
    books: &[BookDayEnds],
    outflows: &[CashOutflow],
) -> Result<EstablishedCashProjection, CashProjectionRefusal> {
    let invalid = |_| CashProjectionRefusal::AmountInvalid;
    let mut lowest = Vec::with_capacity(books.len());
    for book in books {
        let mut deposits = BTreeMap::<&str, ExactDecimal>::new();
        for outflow in outflows
            .iter()
            .filter(|outflow| outflow.ledger == book.ledger)
        {
            let day = deposits
                .entry(outflow.date.as_str())
                .or_insert_with(ExactDecimal::zero);
            *day = day.checked_add(&outflow.amount).map_err(invalid)?;
        }
        let Some(first) = deposits.keys().next().copied() else {
            continue;
        };
        let days = book
            .days
            .keys()
            .map(String::as_str)
            .chain(deposits.keys().copied())
            .collect::<BTreeSet<_>>();
        let mut balance = book.opening.clone();
        let mut deposited = ExactDecimal::zero();
        let mut below_zero = None;
        let mut low: Option<(&str, ExactDecimal)> = None;
        for day in days {
            if let Some(end) = book.days.get(day) {
                balance = end.clone();
            }
            if let Some(amount) = deposits.get(day) {
                deposited = deposited.checked_add(amount).map_err(invalid)?;
            }
            if day < first {
                continue;
            }
            if balance.is_negative() {
                return Err(CashProjectionRefusal::AlreadyNegative {
                    ledger: book.ledger.clone(),
                    date: day.to_string(),
                    balance,
                });
            }
            let after = balance.checked_subtract(&deposited).map_err(invalid)?;
            if after.is_negative() && below_zero.is_none() {
                below_zero = Some(CashProjectionRefusal::WouldGoNegative {
                    ledger: book.ledger.clone(),
                    date: day.to_string(),
                    balance: after.clone(),
                });
            }
            let lower = low.as_ref().is_none_or(|(_, held)| {
                held.checked_subtract(&after)
                    .is_ok_and(|gap| !gap.is_negative() && !gap.is_zero())
            });
            if lower {
                low = Some((day, after));
            }
        }
        if let Some(refusal) = below_zero {
            return Err(refusal);
        }
        if let Some((date, balance)) = low {
            lowest.push(LowestDayEnd {
                ledger: book.ledger.clone(),
                date: date.to_string(),
                balance,
            });
        }
    }
    Ok(EstablishedCashProjection {
        record: CashProjectionRecord {
            ledgers: books.iter().map(|book| book.ledger.clone()).collect(),
            book_sha256: book_sha256(&from, &to, books),
            from,
            to,
        },
        lowest,
    })
}

/// The 31 March on or after `date`. A book's last entry leaves out a post-dated
/// voucher, so it cannot end the window alone; this bound does, and the result
/// names it.
fn year_end(date: &TallyDate) -> Result<TallyDate, String> {
    let (year, month_day) = date.as_str().split_at(4);
    let year = year
        .parse::<u32>()
        .map_err(|_| "invalid_date_range".to_string())?;
    let year = if month_day <= "0331" { year } else { year + 1 };
    normalized_date(&format!("{year}0331"))
}

fn debit_positive(
    ledger: &str,
    amount: &NativeTrialBalanceAmount,
) -> Result<ExactDecimal, CashProjectionRefusal> {
    let NativeTrialBalanceAmount::Present(amount) = amount else {
        return Err(CashProjectionRefusal::BalanceNotRead {
            ledger: ledger.to_string(),
        });
    };
    // Tally's Trial Balance signs a debit negative.
    ExactDecimal::zero()
        .checked_subtract(amount)
        .map_err(|_| CashProjectionRefusal::AmountInvalid)
}

/// Whether the ledger's vouchers must be read: unless Tally states zero on both
/// sides. An older capture leaves one side of a live ledger empty, and empty is
/// not zero, so that goes to the vouchers and the tie.
fn stated_unmoved(row: &NativeTrialBalanceRow) -> bool {
    let zero = |amount: &NativeTrialBalanceAmount| matches!(amount, NativeTrialBalanceAmount::Present(amount) if amount.is_zero());
    zero(&row.debit) && zero(&row.credit)
}

impl Server {
    async fn cash_trial_balance(
        &self,
        identity: &VerifiedCompanyIdentity,
        from: &TallyDate,
        to: &TallyDate,
    ) -> Result<(TrialBalanceRead, CompanyBookExtent), ToolFailure> {
        let period = TrialBalancePeriod::new(from.clone(), to.clone())
            .map_err(|_| "invalid_date_range".to_string())?;
        self.runtime
            .fetch_trial_balance_with_extent(
                self.tally_config(),
                identity,
                period,
                TrialBalanceCurrencyScope::SingleCurrency,
            )
            .await
            .map_err(|error| ToolFailure::from_runtime("trial_balance_read_failed", error))
    }

    /// `ledgers`' day-end tables over the Trial Balance's window. A ledger Tally
    /// states zero movement for reads no vouchers: a scoped answer of none says
    /// nothing, and Tally's own totals do.
    async fn cash_day_ends(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        ledgers: &[String],
        read: TrialBalanceRead,
    ) -> Result<(Vec<BookDayEnds>, Evidence), CashProjectionFailure> {
        let mut evidence = evidence_from_runtime_read(read.evidence.clone());
        let mut books = Vec::with_capacity(ledgers.len());
        for ledger in ledgers {
            let row = read
                .report
                .rows
                .iter()
                .find(|row| row.name == *ledger)
                .ok_or_else(|| CashProjectionRefusal::BalanceNotRead {
                    ledger: ledger.clone(),
                })?;
            let moves = if stated_unmoved(row) {
                BTreeMap::new()
            } else {
                let (moves, read_evidence) = self
                    .read_ledger_day_moves(
                        identity,
                        company,
                        ledger,
                        (read.from.clone(), read.to.clone()),
                    )
                    .await
                    .map_err(|failure| failure.with_prior_evidence(evidence.clone()))?;
                evidence = combine_evidence(evidence, read_evidence);
                moves
            };
            books.push(BookDayEnds::tied(
                ledger,
                debit_positive(ledger, &row.opening)?,
                &moves,
                debit_positive(ledger, &row.closing)?,
            )?);
        }
        Ok((books, evidence))
    }

    /// The projection a batch's deposits are built on, from the first deposit to
    /// the end of the book's year. Single-currency books only, as the Trial
    /// Balance read admits them.
    pub(super) async fn project_cash_book(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        outflows: &[CashOutflow],
    ) -> Result<(EstablishedCashProjection, Evidence), CashProjectionFailure> {
        let dates = outflows.iter().map(|outflow| &outflow.date);
        let (Some(from), Some(last)) = (dates.clone().min(), dates.max()) else {
            return Err(ToolFailure::from("proposals_file_invalid".to_string()).into());
        };
        let from = from.clone();
        let first_to = year_end(last).map_err(ToolFailure::from)?;
        let (read, extent) = self.cash_trial_balance(identity, &from, &first_to).await?;
        let to = year_end(extent.last_voucher_date().max(&first_to)).map_err(ToolFailure::from)?;
        let read = if to == first_to {
            read
        } else {
            self.cash_trial_balance(identity, &from, &to).await?.0
        };
        let ledgers = outflows
            .iter()
            .map(|outflow| outflow.ledger.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let (books, evidence) = self
            .cash_day_ends(identity, company, &ledgers, read)
            .await?;
        Ok((established(from, to, &books, outflows)?, evidence))
    }

    /// At post: the same reads again, and the table must be the build's.
    pub(super) async fn cash_book_unmoved(
        &self,
        identity: &VerifiedCompanyIdentity,
        company: &str,
        record: &CashProjectionRecord,
    ) -> Result<Evidence, CashProjectionFailure> {
        let (read, _) = self
            .cash_trial_balance(identity, &record.from, &record.to)
            .await?;
        let (books, evidence) = self
            .cash_day_ends(identity, company, &record.ledgers, read)
            .await?;
        if book_sha256(&record.from, &record.to, &books) != record.book_sha256 {
            return Err(ToolFailure::from(CASH_PROJECTION_STALE.to_string())
                .with_prior_evidence(evidence)
                .into());
        }
        Ok(evidence)
    }
}

#[cfg(test)]
#[path = "agent_cash_projection_tests.rs"]
pub(super) mod tests;
