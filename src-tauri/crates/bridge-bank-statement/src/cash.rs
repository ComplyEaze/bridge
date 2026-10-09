//! Cash withdrawals and deposits: a person's answer per line, never a rule.
//!
//! The same statement text ("ATM WDL", "BY CASH") covers business cash, the
//! owner's drawings, a customer paying in, and cash handed straight to someone.
//! Only the firm knows which, so each such line is asked one plain question
//! with a closed set of answers, and each answer maps to exactly one entry
//! ([`build`](crate::proposals::build)). A line with no answer is never
//! defaulted: it stays open, and `build_import_xml` refuses the proposals.
//!
//! An explicit "don't know" posts the line to the suspense ledger with
//! [`PURPOSE_NOT_CONFIRMED`] in its narration, and it is counted in the parse
//! and build results, so the CA can find it in the books by that tag and move
//! it.
//! That is the owner's
//! decision of 27-Sep-2026: every bank line reaches the books, and a line whose
//! purpose nobody knows is visible rather than held back.
//!
//! Recognition is by the party name the parser gives a row: SBI's `ATM WDL`
//! and the `BY CASH` rules of Union Bank and Bank of Baroda, each from a measured
//! statement, name the two cash parties ([`crate::bank`]). A parser that extracts one of those two
//! names from other text makes that row a cash line too, which fails safe: it
//! is asked, never defaulted.

use crate::refusal::Refusal;
use crate::text::{mapping_key, squash, strip};
use serde::Serialize;
use std::collections::BTreeMap;

/// The party the SBI parser names an `ATM WDL` row.
pub const CASH_WITHDRAWAL: &str = "ATM CASH WITHDRAWAL";
/// The party the Union Bank and Bank of Baroda parsers name a `BY CASH` row.
pub const CASH_DEPOSIT: &str = "CASH DEPOSIT";

/// The narration tag of a line posted to suspense because a person answered
/// that its purpose is not known.
pub const PURPOSE_NOT_CONFIRMED: &str = "Bridge: purpose not confirmed; reclassify";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CashMovement {
    Withdrawal,
    Deposit,
}

impl CashMovement {
    /// The movement a parsed party names, by the mapping key's fold.
    pub fn of_party(party: &str) -> Option<Self> {
        let key = mapping_key(party);
        if key == mapping_key(CASH_WITHDRAWAL) {
            Some(Self::Withdrawal)
        } else if key == mapping_key(CASH_DEPOSIT) {
            Some(Self::Deposit)
        } else {
            None
        }
    }

    /// Whether money leaves the bank.
    pub fn outward(self) -> bool {
        self == Self::Withdrawal
    }

    /// The question, and every answer it may take, in the order shown.
    pub fn question(self) -> (&'static str, &'static [CashAnswer]) {
        match self {
            Self::Withdrawal => (
                "This cash was withdrawn from the bank. What happened to it?",
                &[
                    CashAnswer::BusinessCash,
                    CashAnswer::OwnerUse,
                    CashAnswer::PaidToSomeone,
                    CashAnswer::DontKnow,
                ],
            ),
            Self::Deposit => (
                "This cash was deposited into the bank. Where did it come from?",
                &[
                    CashAnswer::OwnCashBox,
                    CashAnswer::UnbookedCashSales,
                    CashAnswer::CustomerPaidIn,
                    CashAnswer::OwnerBroughtIn,
                    CashAnswer::DontKnow,
                ],
            ),
        }
    }
}

/// One answer to a cash line's question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CashAnswer {
    /// W1: kept as business cash. Contra, Dr the named cash ledger, Cr bank.
    BusinessCash,
    /// W2: the owner or a partner took it. Payment, Dr the named drawings or
    /// capital ledger, Cr bank.
    OwnerUse,
    /// W3: paid straight to someone. Needs a cash Payment to that party too;
    /// not built yet, so refused.
    PaidToSomeone,
    /// D1: from our own cash box. A Contra that can drive the cash book
    /// negative, so refused until the cash-book projection is built.
    OwnCashBox,
    /// D2: cash sales not yet recorded. Refused: record them first.
    UnbookedCashSales,
    /// D3: a customer paid it in. Receipt, Dr bank, Cr the named customer.
    CustomerPaidIn,
    /// D4: the owner or a partner brought it in. Receipt, Dr bank, Cr the
    /// named capital ledger.
    OwnerBroughtIn,
    /// Nobody knows yet. Posted to the suspense ledger, tagged
    /// [`PURPOSE_NOT_CONFIRMED`].
    DontKnow,
}

impl CashAnswer {
    pub fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "business_cash" => Self::BusinessCash,
            "owner_use" => Self::OwnerUse,
            "paid_to_someone" => Self::PaidToSomeone,
            "own_cash_box" => Self::OwnCashBox,
            "unbooked_cash_sales" => Self::UnbookedCashSales,
            "customer_paid_in" => Self::CustomerPaidIn,
            "owner_brought_in" => Self::OwnerBroughtIn,
            "dont_know" => Self::DontKnow,
            _ => return None,
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessCash => "business_cash",
            Self::OwnerUse => "owner_use",
            Self::PaidToSomeone => "paid_to_someone",
            Self::OwnCashBox => "own_cash_box",
            Self::UnbookedCashSales => "unbooked_cash_sales",
            Self::CustomerPaidIn => "customer_paid_in",
            Self::OwnerBroughtIn => "owner_brought_in",
            Self::DontKnow => "dont_know",
        }
    }

    /// The answer as the person reads it.
    pub fn text(self) -> &'static str {
        match self {
            Self::BusinessCash => "It was kept as business cash (cash box or petty cash).",
            Self::OwnerUse => "The owner or a partner took it for personal use.",
            Self::PaidToSomeone => {
                "It was paid straight to someone (wages, a supplier, a contractor)."
            }
            Self::OwnCashBox => "Our own cash box, from cash already recorded in the books.",
            Self::UnbookedCashSales => "Cash sales or collections that are not yet recorded.",
            Self::CustomerPaidIn => "A customer paid it straight into our account.",
            Self::OwnerBroughtIn => "The owner or a partner brought it in.",
            Self::DontKnow => "I don't know yet. It goes to suspense, marked for the CA to move.",
        }
    }

    /// What the answer needs besides itself: the ledger its one other leg
    /// posts to, named in the answer.
    pub fn ledger_prompt(self) -> Option<&'static str> {
        match self {
            Self::BusinessCash => Some("the cash-in-hand ledger"),
            Self::OwnerUse => Some("the drawings or capital ledger"),
            Self::CustomerPaidIn => Some("the customer's ledger"),
            Self::OwnerBroughtIn => Some("the capital ledger"),
            Self::PaidToSomeone | Self::OwnCashBox | Self::UnbookedCashSales | Self::DontKnow => {
                None
            }
        }
    }

    /// Why Bridge cannot build this answer yet, when it cannot.
    pub fn not_built(self) -> Option<&'static str> {
        match self {
            Self::PaidToSomeone => Some(
                "cash paid straight to someone needs a cash Payment to that person as well, which ComplyEaze Bridge does not build yet; record it by hand, or answer dont_know so it posts to suspense for the CA",
            ),
            Self::OwnCashBox => Some(
                "a deposit from the cash box is a Contra that can drive the cash book negative, and ComplyEaze Bridge cannot yet check the cash book's balance first; record it by hand, or answer dont_know so it posts to suspense for the CA",
            ),
            Self::UnbookedCashSales => Some(
                "record those cash sales or collections in Tally first; the deposit itself is then a Contra from the cash box, which ComplyEaze Bridge does not build yet",
            ),
            _ => None,
        }
    }

    /// Whether the ledger this answer names must be a cash-in-hand ledger in
    /// the book: a bank ledger there would move the cash bank to bank.
    pub fn names_cash_in_hand(self) -> bool {
        matches!(self, Self::BusinessCash | Self::OwnCashBox)
    }

    /// Whether this answer can be given to a line of this movement.
    pub fn answers(self, movement: CashMovement) -> bool {
        movement.question().1.contains(&self)
    }
}

/// One answer as supplied, before validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CashAnswerRow {
    /// Where the answer came from, for refusal messages.
    pub origin: String,
    pub bridge_txn_id: String,
    pub answer: String,
    pub ledger: Option<String>,
}

/// Validated answers, keyed by `bridge_txn_id`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CashAnswers(BTreeMap<String, (CashAnswer, String)>);

static NO_ANSWERS: CashAnswers = CashAnswers(BTreeMap::new());

impl CashAnswers {
    /// No answers at all: every cash line stays open.
    pub fn none() -> &'static Self {
        &NO_ANSWERS
    }

    /// The answer and its ledger (empty for `dont_know`).
    pub fn get(&self, bridge_txn_id: &str) -> Option<&(CashAnswer, String)> {
        self.0.get(bridge_txn_id)
    }

    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }

    pub fn from_rows(rows: impl IntoIterator<Item = CashAnswerRow>) -> Result<Self, Refusal> {
        let mut answers = BTreeMap::new();
        for row in rows {
            let id = strip(&row.bridge_txn_id).to_string();
            let Some(answer) = CashAnswer::parse(strip(&row.answer)) else {
                return Err(Refusal::new(
                    "unknown_cash_answer",
                    format!(
                        "{}: not one of the answers the line's question offers",
                        row.origin
                    ),
                ));
            };
            let ledger = squash(row.ledger.as_deref().unwrap_or_default());
            match (answer.ledger_prompt(), ledger.is_empty()) {
                (Some(prompt), true) => {
                    return Err(Refusal::new(
                        "cash_answer_without_ledger",
                        format!("{}: this answer needs {prompt}", row.origin),
                    ))
                }
                (None, false) => {
                    return Err(Refusal::new(
                        "cash_answer_ledger_not_used",
                        format!(
                            "{}: this answer takes no ledger (dont_know always posts to suspense_ledger)",
                            row.origin
                        ),
                    ))
                }
                _ => {}
            }
            if answers.insert(id, (answer, ledger)).is_some() {
                return Err(Refusal::new(
                    "cash_answer_repeated",
                    format!("{}: a line is answered twice", row.origin),
                ));
            }
        }
        Ok(Self(answers))
    }
}
