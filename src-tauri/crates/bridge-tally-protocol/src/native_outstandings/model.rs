use std::fmt;

use bridge_tally_primitives::{ExactDecimal, TallyDate};

use crate::outstandings_shared::OutstandingsReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeOutstandingsError {
    /// A two-digit display date could not be resolved to a valid calendar
    /// date, or its lexeme did not match the observed `D-MMM-YY` shape.
    InvalidDate(&'static str),
    InvalidAmount,
    /// A ledger's `CLOSINGBALANCE` was a foreign-currency display expression
    /// rather than the base-currency decimal this read requires.
    ForeignCurrencyLedgerBalance {
        ledger_name: String,
    },
    /// Tally's response did not match the documented grammar. The code
    /// identifies which structural rule was violated.
    InvalidResponse(&'static str),
    ArithmeticOverflow,
    /// Tally answered with failure. Two rules apply here (bridge#717): the
    /// flat Bills Receivable/Payable report's verification is INVERTED (any
    /// `STATUS` is failure, and none at all is success), while the collection
    /// exports (ledger, group, currency and company) must carry `STATUS`
    /// reading `1`, so for them this is a `STATUS` with another value.
    TallyReportedFailure,
    /// A collection export (ledger, group, currency or company) was a complete
    /// `ENVELOPE` whose `HEADER` carried no `STATUS`, an empty one or a
    /// self-closing one: structural, and not yet captured from Tally. Not used
    /// by the inverted report, where absence is success (bridge#717).
    StatusAbsent,
    /// The ledgers' own currencies could not be classified against the base
    /// currency (bridge#551): see [`super::LedgerCurrencyRefusal`].
    LedgerCurrency(super::LedgerCurrencyRefusal),
    /// A row of a Bills Receivable or Payable report that could not be read:
    /// which row, in the order Tally sent them, and why. The row number and the
    /// report kind are numbers and a word, never the bill's party or reference
    /// (bridge#1091). `report` is set by the caller that knows which report it
    /// parsed ([`NativeOutstandingsError::in_report`]).
    BillRow {
        report: Option<&'static str>,
        row: u32,
        cause: Box<NativeOutstandingsError>,
    },
}

impl NativeOutstandingsError {
    /// A typed code that carries no data for every variant, for a refusal's `cause`
    /// (bridge#1091: the whole read used to fail with no cause).
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidDate(code) | Self::InvalidResponse(code) => code,
            Self::InvalidAmount => "native_amount_invalid",
            Self::ForeignCurrencyLedgerBalance { .. } => "native_foreign_currency_ledger_balance",
            Self::ArithmeticOverflow => "native_arithmetic_overflow",
            Self::TallyReportedFailure => "native_tally_reported_failure",
            Self::StatusAbsent => "native_status_absent",
            Self::LedgerCurrency(refusal) => refusal.code(),
            Self::BillRow { cause, .. } => cause.code(),
        }
    }

    /// The row refusal's report kind and 1-based row number, when this is one.
    pub fn bill_row(&self) -> Option<(Option<&'static str>, u32)> {
        match self {
            Self::BillRow { report, row, .. } => Some((*report, *row)),
            _ => None,
        }
    }

    /// Names the report a row refusal came from; any other error is unchanged.
    #[must_use]
    pub fn in_report(self, kind: &'static str) -> Self {
        match self {
            Self::BillRow { row, cause, .. } => Self::BillRow {
                report: Some(kind),
                row,
                cause,
            },
            other => other,
        }
    }

    pub(super) fn in_row(self, row: usize) -> Self {
        // The book window is the same for every row, so its refusals are not about this one.
        if matches!(
            self,
            Self::InvalidDate(
                "native_date_book_window_invalid" | "native_date_year_ambiguous_book_window"
            )
        ) {
            return self;
        }
        Self::BillRow {
            report: None,
            row: u32::try_from(row).unwrap_or(u32::MAX),
            cause: Box::new(self),
        }
    }
}

impl fmt::Display for NativeOutstandingsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDate(code) => {
                write!(formatter, "native outstandings date invalid ({code})")
            }
            Self::InvalidAmount => formatter.write_str("Tally returned an invalid native amount"),
            Self::ForeignCurrencyLedgerBalance { ledger_name } => write!(
                formatter,
                "Tally reported a foreign-currency closing balance for ledger {ledger_name}"
            ),
            Self::InvalidResponse(code) => {
                write!(formatter, "native outstandings response invalid ({code})")
            }
            Self::ArithmeticOverflow => formatter
                .write_str("native outstandings arithmetic exceeded the exact-decimal bound"),
            Self::TallyReportedFailure => {
                formatter.write_str("Tally reported failure for the native outstandings request")
            }
            Self::StatusAbsent => {
                formatter.write_str("the native collection carried no STATUS answer")
            }
            Self::LedgerCurrency(refusal) => formatter.write_str(refusal.code()),
            Self::BillRow { report, row, cause } => write!(
                formatter,
                "a bill row could not be read ({}, {} row {row}): {cause}",
                cause.code(),
                report.unwrap_or("bills report")
            ),
        }
    }
}

impl std::error::Error for NativeOutstandingsError {}

/// Which of a bill's two dates ageing is measured from.
///
/// `DueDate` is the verified default (TALLY_PROTOCOL_REFERENCE ground truth
/// captured 2026-08-07): Tally's own `BILLOVERDUE` counter ages from
/// `BILLDUE`, not `BILLDATE`, whenever a bill carries a credit period that
/// makes the two differ. `BillDate` remains selectable for callers that want
/// it explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgeingAnchor {
    BillDate,
    DueDate,
}

/// One outstanding bill row from the flat Bills Receivable/Payable report.
///
/// `tally_overdue_days` is Tally's own `BILLOVERDUE` counter, measured
/// against the requested `SVTODATE`. Tally leaves it empty when the counter
/// is not applicable, including a future-due bill. It is retained only as an
/// independent cross-check against Bridge's own ageing computation and must
/// never be used as ageing's source of truth (it is not recomputed for an
/// as-of date other than the one that was requested).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeBillRow {
    pub party: String,
    pub reference: String,
    pub bill_date: TallyDate,
    pub due_date: TallyDate,
    pub closing_balance: ExactDecimal,
    pub tally_overdue_days: Option<i64>,
}

/// One ledger master row from the `List of Ledgers` collection snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerSnapshotEntry {
    pub name: String,
    pub parent: Option<String>,
    /// `None` means Tally returned an empty `<CLOSINGBALANCE>` element. It is
    /// distinct from an established numeric zero so each consumer can retain
    /// its own documented treatment of that observed wire shape.
    pub closing_balance: Option<ExactDecimal>,
    pub opening_balance: ExactDecimal,
    /// The opening as Tally sent it: `None` when the element was empty, which is
    /// unknown and not zero. `opening_balance` above reads an empty element as
    /// zero for the callers that established that reading; anything that shows
    /// the opening uses this field.
    pub opening_balance_observed: Option<ExactDecimal>,
    pub bill_wise_on: bool,
    /// The ledger's own `CURRENCYNAME` (bridge#551), `None` when the element
    /// was absent or empty. Compared with the base master's NAME by
    /// [`classify_ledger_currencies`](super::classify_ledger_currencies).
    pub currency_name: Option<String>,
}

/// A party's unallocated residual: the gap between the ledger's own
/// `CLOSINGBALANCE` and the sum of everything the Bills Receivable/Payable
/// reports show as open bills for that party. The reports list only named
/// bills, so the residual is money the ledger balance carries with no bill
/// reference at all, and therefore no truthful bill age.
///
/// It is NOT "the on-account amount". On a ledger whose bill-wise flag is off it
/// is what the ledger carries (seen on one ledger that never had bills; a flag
/// switched off after bills existed is unmeasured); on a bill-wise ledger it is the net of the on-account
/// entries and of any opening balance not allocated to a reference, and the
/// bills reports alone cannot separate those (a live book, #945). `bill_wise_on`
/// and `opening_balance` are the ledger's own values, carried so a reader can
/// see which case it is; nothing here infers the components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartyResidual {
    pub party: String,
    pub amount: ExactDecimal,
    /// The ledger's `ISBILLWISEON`.
    pub bill_wise_on: bool,
    /// The ledger's opening balance as of the start of the snapshot period (the
    /// books-from date, not the current year's). `None` when Tally sent an empty
    /// element: unknown, never zero.
    pub opening_balance: Option<ExactDecimal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeOutstandingsResult {
    pub report: OutstandingsReport,
    pub residuals: Vec<PartyResidual>,
    /// Sum of the absolute magnitude of every party residual: the total
    /// unallocated exposure the bill-level reports cannot see (not only
    /// on-account: see [`PartyResidual`]).
    pub residual_total: ExactDecimal,
    /// The outcome of independently comparing Tally's `BILLOVERDUE` values
    /// with Bridge's due-date ageing. It is never used as ageing's source of
    /// truth, but a refused as-of date is materially different from scattered
    /// source-data disagreement and must reach the operator distinctly.
    pub overdue_crosscheck: NativeOverdueCrosscheck,
    /// Ledgers kept in another currency and left out of every figure above,
    /// with their bills (bridge#551). Empty when nothing was excluded, which
    /// is the only case in which the figures describe the whole book.
    pub foreign_currency_ledgers_excluded: Vec<super::ForeignCurrencyLedger>,
    /// Base-currency ledgers with a value Tally wrote as a currency composite,
    /// left out of every figure above with their bills (bridge#642). Their
    /// bills are plain rupee amounts, but the ledger's balance was not read,
    /// so nothing could reconcile them.
    pub mixed_currency_ledgers_excluded: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeOverdueCrosscheck {
    Honored,
    Inconsistent,
    RefusedAsOf {
        tally_as_of: TallyDate,
    },
    /// No bill row can corroborate the requested date, while the separately
    /// read ledger snapshot carries an unallocated balance whose period could
    /// have moved. The report must remain partial rather than claiming the
    /// requested as-of date was honored.
    UnconfirmedAsOfWithoutBillReferences,
    /// The response carried no positive overdue counter that can identify
    /// Tally's effective date. This includes no bill rows, empty counters,
    /// and zero-only counters; none can establish that Tally honored the
    /// requested as-of date.
    UnconfirmedAsOfWithoutEffectiveDateEvidence,
}

/// What Tally reports about a company's currencies.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct CompanyCurrency {
    pub symbol: String,
    pub mailing_name: String,
    /// How many currency masters the company defines. The parser sets
    /// `is_inr` only when there is exactly one: with several defined, this
    /// read cannot tell which is the BASE currency, and guessing would put a
    /// wrong currency symbol in front of a real balance. Identifying the base
    /// among several needs the company's own `CURRENCYNAME`
    /// (TALLY_PROTOCOL_REFERENCE §9.10a.2), and its result is never carried by
    /// this type.
    pub currency_count: usize,
    /// The base currency's display precision reported by Tally. Consumers
    /// must carry this to their rendering boundary rather than silently
    /// assuming paise precision.
    pub decimal_places: u8,
    pub is_inr: bool,
    /// Every master's NAME, in read order (`symbol` is the first). Kept out
    /// of serialization so every output that carries this struct is unchanged;
    /// it exists so a refusal can name the masters it saw.
    #[serde(skip)]
    pub names: Vec<String>,
}

/// One Currency master as the currency read returns it (bridge#551).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct CurrencyMaster {
    /// `NAME`: the symbol a ledger's `CURRENCYNAME` carries (`I₹`, `Rs.`, `$`).
    pub name: String,
    /// `ORIGINALNAME`, when the response carries it, exactly as received (not
    /// trimmed): on the books measured, the value the company's own
    /// `CURRENCYNAME` carries when this master is its base (`₹` for a master
    /// named `I₹`). It identifies the base and never decides INR. `Some("")`
    /// when the element is present but empty, which is not the same as absent.
    /// Only the outstandings reads fetch it, and only on a book with several
    /// masters (bridge#551).
    pub original_name: Option<String>,
    pub mailing_name: String,
    pub decimal_places: u8,
}

impl CurrencyMaster {
    /// The INR rule for an identified base master (bridge#551): its
    /// `MAILINGNAME` is `Indian Rupees` or `INR`, ignoring case.
    ///
    /// The symbol is not an arm. `Rs.` is shared by the Pakistani, Nepali and
    /// Sri Lankan rupees, and whether `ORIGINALNAME` `₹` survives a Company
    /// Alteration that renames the base currency is unmeasured
    /// (TALLY_PROTOCOL_REFERENCE §9.10a.2).
    pub(crate) fn is_inr(&self) -> bool {
        self.mailing_name.eq_ignore_ascii_case("Indian Rupees")
            || self.mailing_name.eq_ignore_ascii_case("INR")
    }
}

impl CompanyCurrency {
    /// The company's currency from its Currency masters, as before bridge#551:
    /// `symbol`, `mailing_name` and `decimal_places` are the first master
    /// read, and `is_inr` holds only for a book with exactly one master that
    /// passes [`CurrencyMaster::is_inr`].
    pub(crate) fn from_masters(masters: &[CurrencyMaster]) -> Self {
        let first = masters.first().cloned().unwrap_or_default();
        Self {
            is_inr: matches!(masters, [only] if only.is_inr()),
            symbol: first.name,
            mailing_name: first.mailing_name,
            currency_count: masters.len(),
            decimal_places: first.decimal_places,
            names: masters.iter().map(|master| master.name.clone()).collect(),
        }
    }
}

/// The base among a book's Currency masters (bridge#551): the only master,
/// or, among several, the unique master whose `ORIGINALNAME` equals the
/// company's own `CURRENCYNAME` character for character
/// (TALLY_PROTOCOL_REFERENCE §9.10a.2). A blank company value never matches.
/// It identifies the base only; whether that base is INR is
/// [`CurrencyMaster::is_inr`]'s.
pub(crate) fn identify_base_master<'a>(
    masters: &'a [CurrencyMaster],
    company_currency_name: Option<&str>,
) -> Option<&'a CurrencyMaster> {
    if let [only] = masters {
        return Some(only);
    }
    let name = company_currency_name.filter(|name| !name.trim().is_empty())?;
    let mut matching = masters
        .iter()
        .filter(|master| master.original_name.as_deref() == Some(name));
    match (matching.next(), matching.next()) {
        (Some(base), None) => Some(base),
        _ => None,
    }
}
