//! Cash withdrawals and deposits: a person's answer per line, never a rule.
//!
//! The same statement text ("ATM WDL", "BY CASH") covers business cash, the
//! owner's drawings, a customer paying in, and cash handed straight to someone.
//! Only the firm knows which, so each such line is asked one plain question
//! with a closed set of answers, and each answer maps to exactly one entry
//! ([`build`](crate::proposals::build)), except cash paid straight to someone,
//! which takes the cash through the cash book: a Contra into the named cash
//! ledger, then a cash Payment to the named payee. A line with no answer is never
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

/// The narration tag of a cash Payment that takes one payee's cash paid in a
/// day, within this statement's answers, over [`CASH_PAYMENT_LIMIT`]. A flag
/// for the CA, never a decision: an expense paid in cash over the limit can be
/// disallowed under s.40A(3) of the Income-tax Act, Rule 6DD lists exceptions,
/// and a higher limit applies to payments for plying, hiring or leasing goods
/// carriages. The section is cited as the Income-tax Act, 1961 numbers it; the
/// Income-tax Act, 2025, in force from 1 April 2026, carries the rule under its
/// own numbering, which is not confirmed here.
pub const CASH_LIMIT_FLAG: &str = "Bridge: cash to one payee in a day over 10000; check s.40A(3)";

/// The s.40A(3) limit, in rupees, on cash paid to one person in a day.
pub const CASH_PAYMENT_LIMIT: &str = "10000.00";

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
    /// W3: paid straight to someone. Contra, Dr the named cash ledger, Cr
    /// bank; then Payment, Dr the named payee, Cr that cash ledger, on the same
    /// day and for the same amount.
    PaidToSomeone,
    /// D1: from our own cash box. Contra, Dr bank, Cr the named cash ledger.
    /// It can drive the cash book negative, so the build projects that ledger
    /// first and refuses unless the projection is established (#1761).
    OwnCashBox,
    /// D2: cash sales not yet recorded. Refused; `dont_know` instead.
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
            Self::BusinessCash | Self::OwnCashBox => Some("the cash-in-hand ledger"),
            Self::OwnerUse => Some("the drawings or capital ledger"),
            Self::CustomerPaidIn => Some("the customer's ledger"),
            Self::OwnerBroughtIn => Some("the capital ledger"),
            Self::PaidToSomeone => Some(
                "the ledger of the person paid (an expense, a supplier or a contractor), and in cash_ledger the cash-in-hand ledger the cash is paid from",
            ),
            Self::UnbookedCashSales | Self::DontKnow => None,
        }
    }

    /// Why Bridge cannot build this answer yet, when it cannot.
    pub fn not_built(self) -> Option<&'static str> {
        match self {
            Self::UnbookedCashSales => Some(
                "the cash sales or collections behind this deposit are not in the books, and ComplyEaze Bridge does not build those sales yet; answer dont_know and the line is built to the suspense ledger, marked for the CA to move",
            ),
            _ => None,
        }
    }

    /// Whether this answer also names, in `cash_ledger`, the cash-in-hand
    /// ledger the cash passes through.
    pub fn takes_cash_ledger(self) -> bool {
        self == Self::PaidToSomeone
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
    /// `paid_to_someone` only: the cash-in-hand ledger the cash is paid from.
    pub cash_ledger: Option<String>,
}

/// One validated answer. Only [`CashAnswers::from_rows`] builds one, so a
/// `paid_to_someone` answer always carries its cash ledger and no other
/// answer carries one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answered {
    answer: CashAnswer,
    ledger: String,
    cash_ledger: Option<String>,
}

impl Answered {
    pub fn answer(&self) -> CashAnswer {
        self.answer
    }

    /// The ledger the answer names (empty for `dont_know`): for
    /// `paid_to_someone`, the payee.
    pub fn ledger(&self) -> &str {
        &self.ledger
    }

    /// The cash-in-hand ledger a `paid_to_someone` answer pays from.
    pub fn cash_ledger(&self) -> Option<&str> {
        self.cash_ledger.as_deref()
    }
}

/// Validated answers, keyed by `bridge_txn_id`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CashAnswers(BTreeMap<String, Answered>);

static NO_ANSWERS: CashAnswers = CashAnswers(BTreeMap::new());

impl CashAnswers {
    /// No answers at all: every cash line stays open.
    pub fn none() -> &'static Self {
        &NO_ANSWERS
    }

    /// The answer and its ledgers.
    pub fn get(&self, bridge_txn_id: &str) -> Option<&Answered> {
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
            let cash_ledger = squash(row.cash_ledger.as_deref().unwrap_or_default());
            let cash_ledger = match (answer.takes_cash_ledger(), cash_ledger.is_empty()) {
                (true, true) => {
                    return Err(Refusal::new(
                        "cash_answer_without_cash_ledger",
                        format!(
                            "{}: paid_to_someone also needs cash_ledger, the cash-in-hand ledger the cash is paid from",
                            row.origin
                        ),
                    ))
                }
                (false, false) => {
                    return Err(Refusal::new(
                        "cash_ledger_not_used",
                        format!(
                            "{}: only a paid_to_someone answer takes cash_ledger",
                            row.origin
                        ),
                    ))
                }
                (true, false) => Some(cash_ledger),
                (false, true) => None,
            };
            let answered = Answered {
                answer,
                ledger,
                cash_ledger,
            };
            if answers.insert(id, answered).is_some() {
                return Err(Refusal::new(
                    "cash_answer_repeated",
                    format!("{}: a line is answered twice", row.origin),
                ));
            }
        }
        Ok(Self(answers))
    }
}
