//! The transfer pairing view: for each bank line skipped as "recorded from the
//! other bank's statement", whether that other side exists as a posted Contra.
//!
//! The view is a pure function of plain typed facts (`read_days` is the one part
//! that sends requests, and only through `read_entry_window_rows_with`, the
//! reader-generic form of `read_entry_window_rows`). It reads no journal, holds no state and
//! persists no pairing, so a pair whose other batch is rolled back, amended or
//! fails verification simply stops being a pair the next time it is computed,
//! and no order of the inputs can change the answer (every candidate set is
//! counted, never consumed in turn).
//!
//! The matching runs in two rounds over "edges" between a skipped row and a
//! Contra:
//!
//! * An edge needs the same company and endpoint, a Contra whose two legs are
//!   exactly the skip's own bank ledger and its other ledger on opposite sides,
//!   the same direction on the skip's own bank ledger (a withdrawal skipped from
//!   its bank is a Contra that credits that bank, so a return transfer never
//!   matches), equal amounts, and a Contra from outside the skip's lineage and
//!   built from the other ledger's own statement (one from the skip's own
//!   statement is that statement's own row; one from a third bank's is another
//!   transfer).
//! * Round 1, by reference: both rows carry a reference and they are equal.
//!   A pair forms only where the skip and the Contra each have exactly one such
//!   edge.
//! * Round 2, without a reference: on what round 1 left, one side has no
//!   reference, and the dates are within k days. A pair forms only where both
//!   ends have exactly one such edge, and only when the other ledger's verified
//!   windows hold the skip's date +/- k and the skip's own ledger's hold the
//!   Contra's: a pair by date claims that nothing else exists. Where a window
//!   is missing the skip is `MatchedUnconfirmed` and keeps that Contra: the
//!   Contra is known, and that it is the only one is not. A row that took
//!   part in round 1 without pairing does not pair here, although it still
//!   counts against the rows that could.
//! * A row with an equal amount going the other way the same day on its own
//!   statement (`reversed_same_day`) may be a reversal or one of two real
//!   transfers, and the date cannot tell them apart. Equal network references
//!   still pair it (round 1 ignores the flag). It never pairs by date (round
//!   2), but it still counts against the rows that could. A skip with no
//!   reference candidate that is flagged, or any of whose candidates is, is
//!   `ReversedSameDay` with those candidates instead of `Ambiguous` or
//!   `NotFound`. A skip with a reference candidate is open for the reason any
//!   row would be (`OtherSideUnverified`, `Ambiguous`), and one with no
//!   candidate beside a Contra of another reference is `ReferenceDiffers`,
//!   flag or none.
//! * A Contra that is double-booked (also booked from a second statement) pairs
//!   in neither round; a skip whose candidate it is stays `Ambiguous`.
//!
//! Contras are grouped once by (company, endpoint, ledger pair, side on each,
//! canonical amount), so a skip reads only the same-amount, same-direction
//! Contras, by reference or as a date range.
//!
//! Facts the view cannot reason about are refused at the boundary with a typed
//! `PairingInputError`: duplicate keys, an amount that is not positive, a skip
//! whose two ledgers are one, and a parse window that ends before it starts.
//!
//! Unverified Contras count as candidates (a second possible match must make a
//! skip ambiguous, not paired) but never pair: a skip whose candidates are all
//! in one batch not yet `posted_verified` is `OtherSideUnverified`.
//!
//! A skip with no candidate is not said to have no Contra while one of its
//! day (+/- k), amount, ledgers and direction carries a reference that differs
//! from the skip's and is neither paired with another row nor matched with one
//! by date (`MatchedUnconfirmed`): it is `ReferenceDiffers` with every such
//! Contra. Which banks print one value on both statements of a transfer is not
//! measured, so two references that differ neither pair two rows nor prove
//! them apart. This changes no pair: the two rounds never read it.
//!
//! Otherwise a skip with no candidate is open for one of three reasons. If the
//! other bank ledger's verified parse windows do not cover its whole date +/- k
//! interval, the absence proves nothing and it is `NotEstablished`. If they do,
//! it is `NotFound` only when a read of Tally has covered the skip's whole date
//! and found no Contra between the two ledgers, plain or not (`TallyDays`).
//! Without such a read it is `TallyUnchecked`: the statements hold no Contra
//! for it, and a Contra entered in Tally by other means is not in their records.
//! The read is `read_days`, which reads at most four of the skips' days (newest
//! first, within 31 days of the newest) and yields a `WholeDay` only from a
//! day whose census counted it, with none withheld, none outside the day and
//! at least one voucher. No caller passes its days to the view yet, so every
//! row that was `NotFound` is `TallyUnchecked` until one does (#1392).
//!
//! Two skips with one `SkipKey`, or two Contras with one `ContraKey`, are refused
//! with a typed error naming the smallest such key, whatever the input order.

use super::voucher_parse::VoucherRow;
use super::voucher_window::{
    AgentReader, VoucherReadShape, WindowPart, WindowReadOutcome, WindowReader,
};
use super::vouchers::{read_entry_window_rows_with, VoucherComposites};
use super::VerifiedCompanyIdentity;
use bridge_bank_statement::bank::BankReference;
use bridge_tally_core::{DateSpan, ExactDecimal, TallyDate};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

macro_rules! text_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub(crate) struct $name(String);

        impl $name {
            pub(crate) fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            // Not every identifier's text is read back.
            #[allow(dead_code)]
            pub(crate) fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

text_id!(
    /// A Tally company GUID.
    CompanyGuid
);
text_id!(
    /// A Tally ledger GUID.
    LedgerGuid
);
text_id!(
    /// The Tally endpoint a batch was built against.
    EndpointOrigin
);
text_id!(
    /// A batch's lineage: every rebuild or amendment of one statement.
    LineageId
);
text_id!(
    /// A saved batch's identity.
    BatchId
);
text_id!(
    /// A statement row's `bridge_txn_id`.
    TxnId
);
text_id!(
    /// A skipped statement row's `st-` id.
    StatementRowId
);

/// A skipped row, whichever batch built it: a rebuilt statement is the same skip.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SkipKey {
    pub(crate) company: CompanyGuid,
    pub(crate) bank_ledger: LedgerGuid,
    /// One statement read through two endpoints is two skips.
    pub(crate) endpoint_origin: EndpointOrigin,
    pub(crate) row: StatementRowId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ContraKey {
    pub(crate) batch: BatchId,
    pub(crate) txn: TxnId,
}

/// Which way money moved on the row's OWN bank ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Outflow,
    Inflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Side {
    Debit,
    Credit,
}

impl Side {
    fn opposite(self) -> Self {
        match self {
            Self::Debit => Self::Credit,
            Self::Credit => Self::Debit,
        }
    }
}

impl Direction {
    /// The side a Contra takes on the skip's own bank ledger: money leaving a
    /// bank credits it, money arriving debits it.
    fn contra_side(self) -> Side {
        match self {
            Self::Outflow => Side::Credit,
            Self::Inflow => Side::Debit,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SkipFact {
    pub(crate) key: SkipKey,
    pub(crate) date: TallyDate,
    pub(crate) amount: ExactDecimal,
    pub(crate) direction: Direction,
    pub(crate) other_ledger: LedgerGuid,
    pub(crate) reference: Option<BankReference>,
    pub(crate) lineage: LineageId,
    /// The statement holds another row on the same date, of an equal amount,
    /// the other way: a reversal and two real transfers look alike, so the
    /// date alone cannot pair the row.
    pub(crate) reversed_same_day: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContraLeg {
    pub(crate) ledger: LedgerGuid,
    pub(crate) side: Side,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BatchStatus {
    PostedVerified,
    NotVerified { batch: BatchId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContraFact {
    pub(crate) key: ContraKey,
    pub(crate) company: CompanyGuid,
    pub(crate) endpoint_origin: EndpointOrigin,
    pub(crate) legs: [ContraLeg; 2],
    pub(crate) amount: ExactDecimal,
    pub(crate) date: TallyDate,
    pub(crate) reference: Option<BankReference>,
    /// The bank ledger of the statement the Contra was built from.
    pub(crate) statement_bank_ledger: LedgerGuid,
    pub(crate) lineage: LineageId,
    pub(crate) status: BatchStatus,
    /// The statement holds another row on the same date, of an equal amount,
    /// the other way: a reversal and two real transfers look alike, so the
    /// date alone cannot pair the row.
    pub(crate) reversed_same_day: bool,
}

/// A verified statement parse window, both ends inclusive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParseWindow {
    pub(crate) from: TallyDate,
    pub(crate) to: TallyDate,
}

/// Per (company, endpoint, bank ledger), the windows of its verified statements.
#[derive(Debug, Clone, Default)]
pub(crate) struct Coverage(BTreeMap<(CompanyGuid, EndpointOrigin, LedgerGuid), Vec<ParseWindow>>);

impl Coverage {
    pub(crate) fn add(
        &mut self,
        company: CompanyGuid,
        endpoint_origin: EndpointOrigin,
        bank_ledger: LedgerGuid,
        window: ParseWindow,
    ) -> Result<(), PairingInputError> {
        if window.from > window.to {
            return Err(PairingInputError::InvertedWindow { bank_ledger });
        }
        self.0
            .entry((company, endpoint_origin, bank_ledger))
            .or_default()
            .push(window);
        Ok(())
    }
}

/// The days of Tally that a read of the whole day has covered and found without
/// any Contra between a skip's two ledgers, plain two-entry or not. Only such a
/// day lets a skip with no candidate be `NotFound`. Outside tests the only
/// constructor is [`TallyDays::from_whole_days`], which takes [`WholeDay`]s, so
/// a day that was not read whole cannot be in it. No caller reads days yet
/// (#1392), so [`TallyDays::none`] is what every caller passes.
#[derive(Debug, Clone, Default)]
pub(crate) struct TallyDays(
    BTreeSet<(
        CompanyGuid,
        EndpointOrigin,
        TallyDate,
        LedgerGuid,
        LedgerGuid,
    )>,
);

impl TallyDays {
    /// No day of Tally has been read.
    pub(crate) fn none() -> Self {
        Self::default()
    }

    /// The skips whose own day was read whole and holds no voucher with an entry
    /// on both of the skip's ledgers, of any type or number of entries: a
    /// Contra "plain or not" is not told from a Journal or a Payment here, so
    /// what is not told apart is counted as found. `name_of` gives the name
    /// Tally prints for a ledger, and must give it as the voucher rows spell it
    /// (case aside): a name spelled otherwise matches no entry and reads as no
    /// Contra, the unsafe direction. A skip with a ledger it cannot name is left
    /// out, as is a skip whose company, Tally or day is not among `days`. Only
    /// the skip's own day is looked at, so it is for a view with `k_days` of 0,
    /// as production passes; with more, a Contra keyed on a neighbouring day,
    /// which was not read, would not be found.
    pub(crate) fn from_whole_days(
        days: &[WholeDay],
        skips: &[SkipFact],
        name_of: impl Fn(&LedgerGuid) -> Option<String>,
    ) -> Self {
        Self(
            skips
                .iter()
                .filter(|skip| {
                    let day = days.iter().find(|day| {
                        day.company == skip.key.company
                            && day.endpoint_origin == skip.key.endpoint_origin
                            && day.date == skip.date
                    });
                    match (
                        day,
                        name_of(&skip.key.bank_ledger),
                        name_of(&skip.other_ledger),
                    ) {
                        (Some(day), Some(bank), Some(other)) => !day.has_entries_on(&bank, &other),
                        _ => false,
                    }
                })
                .map(|skip| {
                    (
                        skip.key.company.clone(),
                        skip.key.endpoint_origin.clone(),
                        skip.date.clone(),
                        skip.key.bank_ledger.clone(),
                        skip.other_ledger.clone(),
                    )
                })
                .collect(),
        )
    }

    /// Whether a whole-day read found no Contra between the skip's two ledgers
    /// on the skip's own date.
    fn found_no_contra(&self, skip: &SkipFact) -> bool {
        self.0.contains(&(
            skip.key.company.clone(),
            skip.key.endpoint_origin.clone(),
            skip.date.clone(),
            skip.key.bank_ledger.clone(),
            skip.other_ledger.clone(),
        ))
    }

    /// Every skip's own day read whole with no Contra between its ledgers, for
    /// the tests of what `NotFound` needs besides a read of Tally.
    #[cfg(test)]
    pub(crate) fn clean_for(skips: &[SkipFact]) -> Self {
        Self(
            skips
                .iter()
                .map(|skip| {
                    (
                        skip.key.company.clone(),
                        skip.key.endpoint_origin.clone(),
                        skip.date.clone(),
                        skip.key.bank_ledger.clone(),
                        skip.other_ledger.clone(),
                    )
                })
                .collect(),
        )
    }
}

/// A voucher of a whole day as the check needs it: the ledgers its entries name,
/// as [`ledger_key`] reads them. No amount, party or narration is kept.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DayVoucher {
    guid: String,
    ledgers: BTreeSet<String>,
}

/// How two spellings of a ledger name are told equal: Tally compares names
/// without regard to case, and a difference missed here would read a day as
/// holding no entry on a ledger it holds one on.
fn ledger_key(name: &str) -> String {
    name.trim().to_lowercase()
}

/// One day of Tally, read whole (#1392): every voucher of the day, counted by a
/// census of its own and admitted against it, with none withheld and none
/// outside the day. It has no public constructor and no `Default`; the only way
/// to one is [`read_days`], and a read that is partial in any way yields a
/// [`NotWhole`] instead, so [`TallyDays::from_whole_days`], and with it
/// `NotFound`, cannot be handed a day that was not read whole. It is a read at
/// one moment: it holds no marks or read time, so it serves the call that read
/// it and is not kept for a later one, after which the book may have changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WholeDay {
    company: CompanyGuid,
    endpoint_origin: EndpointOrigin,
    date: TallyDate,
    vouchers: Vec<DayVoucher>,
}

/// Why a read of one day is not a whole day. Each leaves the day's rows
/// `TallyUnchecked`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NotWhole {
    /// A part of the read is not exactly the day, or a row is dated another day.
    NotTheDay,
    /// No census counted the day's vouchers, so nothing but the response itself
    /// vouches for its rows.
    Uncounted,
    /// Tally stored an amount of some vouchers of the day as a foreign-currency
    /// composite and the read withheld them: the day was not read in full.
    Withheld { vouchers: usize },
    /// The day holds no voucher. An empty window is complete only when a read
    /// of its neighbouring days corroborates it (the `vouchers` tool's rule,
    /// #985), and this read makes none, so an empty day is not said to be whole.
    Empty,
    /// A row without a GUID, or an entry without a ledger.
    Unreadable,
}

impl WholeDay {
    /// The day, or why the read was not one. The window is read off the read's
    /// own parts, not trusted from the caller.
    fn from_read(
        company: CompanyGuid,
        endpoint_origin: EndpointOrigin,
        date: &TallyDate,
        read: &WindowReadOutcome<VoucherRow>,
    ) -> Result<Self, NotWhole> {
        Self::admit(
            company,
            endpoint_origin,
            date,
            &read.reads,
            read.counted(),
            &read.rows,
        )
    }

    fn admit(
        company: CompanyGuid,
        endpoint_origin: EndpointOrigin,
        date: &TallyDate,
        parts: &[WindowPart],
        counted: bool,
        rows: &[VoucherRow],
    ) -> Result<Self, NotWhole> {
        if parts.is_empty()
            || parts
                .iter()
                .any(|part| part.from != *date || part.to != *date)
        {
            return Err(NotWhole::NotTheDay);
        }
        if !counted {
            return Err(NotWhole::Uncounted);
        }
        let mut vouchers = Vec::with_capacity(rows.len());
        let mut withheld = 0;
        for row in rows {
            match row {
                VoucherRow::Withheld(_) => withheld += 1,
                VoucherRow::Read(row) => vouchers.push(day_voucher(row, date)?),
            }
        }
        if withheld > 0 {
            return Err(NotWhole::Withheld { vouchers: withheld });
        }
        if vouchers.is_empty() {
            return Err(NotWhole::Empty);
        }
        Ok(Self {
            company,
            endpoint_origin,
            date: date.clone(),
            vouchers,
        })
    }

    pub(crate) fn date(&self) -> &TallyDate {
        &self.date
    }

    /// The GUIDs of the day's vouchers.
    pub(crate) fn guids(&self) -> BTreeSet<&str> {
        self.vouchers
            .iter()
            .map(|voucher| voucher.guid.as_str())
            .collect()
    }

    /// Whether one voucher of the day has an entry on each of the two ledgers.
    fn has_entries_on(&self, first: &str, second: &str) -> bool {
        let (first, second) = (ledger_key(first), ledger_key(second));
        self.vouchers
            .iter()
            .any(|voucher| voucher.ledgers.contains(&first) && voucher.ledgers.contains(&second))
    }
}

fn day_voucher(row: &Value, date: &TallyDate) -> Result<DayVoucher, NotWhole> {
    if row["date"].as_str() != Some(date.as_str()) {
        return Err(NotWhole::NotTheDay);
    }
    let guid = row["guid"]
        .as_str()
        .filter(|guid| !guid.is_empty())
        .ok_or(NotWhole::Unreadable)?;
    let ledgers = row["amounts"]
        .as_array()
        .ok_or(NotWhole::Unreadable)?
        .iter()
        .map(|entry| {
            entry["ledger"]
                .as_str()
                .map(ledger_key)
                .ok_or(NotWhole::Unreadable)
        })
        .collect::<Result<_, _>>()?;
    Ok(DayVoucher {
        guid: guid.to_string(),
        ledgers,
    })
}

/// The most days one call reads, each as a window of its own.
pub(crate) const MAX_DAY_READS: usize = 4;
/// The days one call looks across, both ends counted: every day it reads is at
/// most `MAX_DAY_SPAN - 1` days before the newest day it could read.
pub(crate) const MAX_DAY_SPAN: u32 = 31;

/// A window reader that knows which Tally it reads, so that a day read can be
/// filed only against that Tally's skips.
pub(crate) trait DayReader: WindowReader {
    fn endpoint_origin(&self) -> Result<EndpointOrigin, String>;
}

impl DayReader for AgentReader<'_> {
    fn endpoint_origin(&self) -> Result<EndpointOrigin, String> {
        super::endpoint_origin(&self.0.settings.endpoint).map(EndpointOrigin::new)
    }
}

/// The days a call may read, of one company on the Tally the reader reads: those
/// of the skipped rows that are `TallyUnchecked` in the view, newest first,
/// within [`MAX_DAY_SPAN`] days of the newest and at most [`MAX_DAY_READS`] of
/// them. Only [`DayPlan::of`] makes one, so no caller can read a day this did not
/// admit, and [`read_days`] reads it only through the reader and identity it was
/// made for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DayPlan {
    company: CompanyGuid,
    endpoint_origin: EndpointOrigin,
    days: Vec<TallyDate>,
}

impl DayPlan {
    pub(crate) fn of<R: DayReader>(
        skips: &[SkipFact],
        view: &TransferView,
        identity: &VerifiedCompanyIdentity,
        reader: &R,
    ) -> Result<Self, String> {
        let company = CompanyGuid::new(identity.company_guid());
        let endpoint_origin = reader.endpoint_origin()?;
        let days: BTreeSet<&TallyDate> = skips
            .iter()
            .filter(|skip| {
                skip.key.company == company
                    && skip.key.endpoint_origin == endpoint_origin
                    && view.skips.get(&skip.key) == Some(&SkipState::TallyUnchecked)
            })
            .map(|skip| &skip.date)
            .collect();
        let mut newest_first = days.into_iter().rev();
        let days = match newest_first.next() {
            None => Vec::new(),
            Some(newest) => std::iter::once(newest)
                .chain(newest_first.filter(|day| within(day, newest, MAX_DAY_SPAN - 1)))
                .take(MAX_DAY_READS)
                .cloned()
                .collect(),
        };
        Ok(Self {
            company,
            endpoint_origin,
            days,
        })
    }

    pub(crate) fn days(&self) -> &[TallyDate] {
        &self.days
    }
}

/// Why a planned day yielded no [`WholeDay`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DayKept {
    NotWhole(NotWhole),
    /// The read failed, with its refusal code. No later day is tried: a gateway
    /// that failed is not asked again in the same call.
    ReadFailed(String),
    NotAttempted,
}

/// Refusal code for a plan read through another company or another Tally than
/// the one it was made for.
pub(crate) const DAY_PLAN_FOR_ANOTHER_READ: &str = "day_plan_for_another_company_or_endpoint";

/// What a call's day reads yielded: the days read whole, and for every other
/// planned day why it was not. A kept day's rows stay `TallyUnchecked`.
#[derive(Debug, Default)]
pub(crate) struct DayReads {
    pub(crate) whole: Vec<WholeDay>,
    pub(crate) kept: Vec<(TallyDate, DayKept)>,
}

/// Read the planned days, newest first, one window of exactly one day each:
/// the company's marks, the day's census and the day's vouchers
/// (`read_entry_window_rows`), with a voucher Tally stored as a foreign-currency
/// composite withheld, not refused, so the day is kept with its cause. A plan
/// made for another company or another Tally than `identity` and `reader` reads
/// nothing.
pub(crate) async fn read_days<R: DayReader>(
    reader: &R,
    identity: &VerifiedCompanyIdentity,
    company: &str,
    plan: &DayPlan,
) -> DayReads {
    let mut reads = DayReads::default();
    let mut failed = match reader.endpoint_origin() {
        Ok(endpoint) => {
            endpoint != plan.endpoint_origin || identity.company_guid() != plan.company.as_str()
        }
        Err(_) => true,
    };
    if failed {
        reads.kept = plan
            .days()
            .iter()
            .map(|date| {
                (
                    date.clone(),
                    DayKept::ReadFailed(DAY_PLAN_FOR_ANOTHER_READ.to_string()),
                )
            })
            .collect();
        return reads;
    }
    for date in plan.days() {
        if failed {
            reads.kept.push((date.clone(), DayKept::NotAttempted));
            continue;
        }
        match read_day(reader, identity, company, &plan.endpoint_origin, date).await {
            Ok(day) => reads.whole.push(day),
            Err(kept) => {
                failed = matches!(kept, DayKept::ReadFailed(_));
                reads.kept.push((date.clone(), kept));
            }
        }
    }
    reads
}

async fn read_day<R: WindowReader>(
    reader: &R,
    identity: &VerifiedCompanyIdentity,
    company: &str,
    endpoint_origin: &EndpointOrigin,
    date: &TallyDate,
) -> Result<WholeDay, DayKept> {
    let read = read_entry_window_rows_with(
        reader,
        identity,
        company,
        date,
        date,
        VoucherReadShape::EntryWildcard,
        VoucherComposites::Withhold,
    )
    .await
    .map_err(|failure| DayKept::ReadFailed(failure.code))?;
    WholeDay::from_read(
        CompanyGuid::new(identity.company_guid()),
        endpoint_origin.clone(),
        date,
        &read,
    )
    .map_err(DayKept::NotWhole)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PairRound {
    Reference,
    NoReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SkipState {
    Paired {
        round: PairRound,
        contra: ContraKey,
    },
    /// The statements hold no Contra for the row, and a read of Tally covered
    /// the row's whole date and found none between the two ledgers.
    NotFound,
    /// The statements hold no Contra for the row, and no read of Tally covered
    /// its whole date: a Contra entered in Tally by other means is not in the
    /// statements' records.
    TallyUnchecked,
    Ambiguous {
        candidates: Vec<ContraKey>,
    },
    OtherSideUnverified {
        batch: BatchId,
    },
    NotEstablished,
    /// One verified Contra matches the row by date and nothing competes with
    /// it, and the verified windows of one of the two ledgers do not hold the
    /// dates the match rests on: the Contra is known, and that it is the only
    /// one is not.
    MatchedUnconfirmed {
        contra: ContraKey,
    },
    /// The skipped row, or a row that could carry it, has an equal amount going
    /// the other way the same day on its own statement, so a reversal and two
    /// real transfers look alike and the date cannot pair it. Listed with the Contras
    /// that could have carried it (none when nothing matched).
    ReversedSameDay {
        candidates: Vec<ContraKey>,
    },
    /// Nothing matches the row by reference or by date, and at least one Contra
    /// of its day, amount, ledgers and direction, neither paired with another
    /// row nor matched with one by date, carries a reference that differs from
    /// the row's: whether such a Contra is the row's other side is not known,
    /// so it is listed and the row is not said to have none.
    ReferenceDiffers {
        candidates: Vec<ContraKey>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DoubleBookedClass {
    /// Equal references: building the second statement is refused.
    RefuseAtBuild,
    /// No reference on one side and the dates within k days: listed only.
    List,
}

/// Two Contras, from two different bank statements, for one transfer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DoubleBooked {
    pub(crate) first: ContraKey,
    pub(crate) second: ContraKey,
    pub(crate) class: DoubleBookedClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TransferView {
    pub(crate) skips: BTreeMap<SkipKey, SkipState>,
    pub(crate) double_booked: Vec<DoubleBooked>,
}

/// Facts that cannot be paired because the caller gave two for one key.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum PairingInputError {
    #[error("two skipped rows share one key")]
    DuplicateSkip { key: SkipKey },
    #[error("two Contras share one key")]
    DuplicateContra { key: ContraKey },
    #[error("a skipped row's amount is not positive")]
    NonPositiveSkipAmount { key: SkipKey },
    #[error("a Contra's amount is not positive")]
    NonPositiveContraAmount { key: ContraKey },
    #[error("a skipped row's own bank ledger is its other ledger")]
    SkipLedgersEqual { key: SkipKey },
    #[error("a verified parse window ends before it starts")]
    InvertedWindow { bank_ledger: LedgerGuid },
}

/// What one transfer shares, so that a skip looks up only the Contras it can
/// match: company, endpoint, the two ledgers (in a fixed order), the side the
/// Contra takes on the first of them, and the amount in canonical form.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct GroupKey {
    company: CompanyGuid,
    endpoint: EndpointOrigin,
    low: LedgerGuid,
    high: LedgerGuid,
    low_side: Side,
    amount: String,
}

/// The Contras of one group, as indices into the sorted Contras.
#[derive(Default)]
struct Group<'a> {
    /// Ordered by date (then key), so the dates within k of a day are a range.
    by_date: Vec<usize>,
    by_reference: BTreeMap<&'a BankReference, Vec<usize>>,
}

/// The amount without scale artefacts, so equal values are equal text: `500`,
/// `500.00` and `0500.0` are one amount. Done once per fact, on an amount the
/// view has already checked is positive.
fn canonical_amount(amount: &ExactDecimal) -> String {
    let text = amount.as_str();
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    let whole = whole.trim_start_matches('0');
    let fraction = fraction.trim_end_matches('0');
    let mut canonical = String::new();
    canonical.push_str(if whole.is_empty() { "0" } else { whole });
    if !fraction.is_empty() {
        canonical.push('.');
        canonical.push_str(fraction);
    }
    canonical
}

fn group_key(
    company: &CompanyGuid,
    endpoint: &EndpointOrigin,
    amount: String,
    first: (&LedgerGuid, Side),
    second: (&LedgerGuid, Side),
) -> Option<GroupKey> {
    if first.0 == second.0 || first.1 == second.1 {
        return None;
    }
    let (low, high) = if first.0 < second.0 {
        (first, second)
    } else {
        (second, first)
    };
    Some(GroupKey {
        company: company.clone(),
        endpoint: endpoint.clone(),
        low: low.0.clone(),
        high: high.0.clone(),
        low_side: low.1,
        amount,
    })
}

/// A Contra has a group only when its legs are two distinct ledgers on
/// opposite sides, which is what a Contra is.
fn contra_group_key(contra: &ContraFact) -> Option<GroupKey> {
    let [first, second] = &contra.legs;
    group_key(
        &contra.company,
        &contra.endpoint_origin,
        canonical_amount(&contra.amount),
        (&first.ledger, first.side),
        (&second.ledger, second.side),
    )
}

/// The group the skip's other side would be in: the Contra's side on the skip's
/// own bank ledger is the one its direction needs, and the other ledger takes
/// the opposite.
fn skip_group_key(skip: &SkipFact) -> Option<GroupKey> {
    let own_side = skip.direction.contra_side();
    group_key(
        &skip.key.company,
        &skip.key.endpoint_origin,
        canonical_amount(&skip.amount),
        (&skip.key.bank_ledger, own_side),
        (&skip.other_ledger, own_side.opposite()),
    )
}

pub(crate) fn transfer_pairing_view(
    skips: &[SkipFact],
    contras: &[ContraFact],
    coverage: &Coverage,
    k_days: u32,
) -> Result<TransferView, PairingInputError> {
    transfer_pairing_view_with(skips, contras, coverage, &TallyDays::none(), k_days)
}

/// The view with the days of Tally a whole-day read has covered. With
/// [`TallyDays::none`] it is [`transfer_pairing_view`].
pub(crate) fn transfer_pairing_view_with(
    skips: &[SkipFact],
    contras: &[ContraFact],
    coverage: &Coverage,
    tally: &TallyDays,
    k_days: u32,
) -> Result<TransferView, PairingInputError> {
    let mut skips: Vec<&SkipFact> = skips.iter().collect();
    skips.sort_by(|left, right| left.key.cmp(&right.key));
    if let Some(pair) = skips.windows(2).find(|pair| pair[0].key == pair[1].key) {
        return Err(PairingInputError::DuplicateSkip {
            key: pair[0].key.clone(),
        });
    }
    let mut contras: Vec<&ContraFact> = contras.iter().collect();
    contras.sort_by(|left, right| left.key.cmp(&right.key));
    if let Some(pair) = contras.windows(2).find(|pair| pair[0].key == pair[1].key) {
        return Err(PairingInputError::DuplicateContra {
            key: pair[0].key.clone(),
        });
    }
    // Bad facts are refused whole, in key order (skips, then Contras), so the
    // error does not depend on the order the facts came in.
    for skip in &skips {
        if skip.amount.is_zero() || skip.amount.is_negative() {
            return Err(PairingInputError::NonPositiveSkipAmount {
                key: skip.key.clone(),
            });
        }
        if skip.key.bank_ledger == skip.other_ledger {
            return Err(PairingInputError::SkipLedgersEqual {
                key: skip.key.clone(),
            });
        }
    }
    for contra in &contras {
        if contra.amount.is_zero() || contra.amount.is_negative() {
            return Err(PairingInputError::NonPositiveContraAmount {
                key: contra.key.clone(),
            });
        }
    }
    let coverage = MergedCoverage::new(coverage);

    let mut groups: BTreeMap<GroupKey, Group> = BTreeMap::new();
    for (index, contra) in contras.iter().enumerate() {
        if let Some(key) = contra_group_key(contra) {
            groups.entry(key).or_default().by_date.push(index);
        }
    }
    for group in groups.values_mut() {
        // stable, so equal dates stay in key order
        group
            .by_date
            .sort_by(|&left, &right| contras[left].date.cmp(&contras[right].date));
        for &index in &group.by_date {
            let contra: &ContraFact = contras[index];
            if let Some(reference) = &contra.reference {
                group.by_reference.entry(reference).or_default().push(index);
            }
        }
    }
    let skip_groups: Vec<Option<&Group>> = skips
        .iter()
        .map(|skip| skip_group_key(skip).and_then(|key| groups.get(&key)))
        .collect();

    // A Contra booked from two statements pairs with nothing: which of the two
    // is the real one is not for this view to say.
    let (double_booked, doubled) = find_double_booked(&contras, &groups, k_days);

    // Round 1: equal references. There is no date bound here: the design pairs
    // a reference whatever the dates, since one transfer can print on different
    // days on the two statements.
    let mut reference_edges: Vec<Vec<usize>> = vec![Vec::new(); skips.len()];
    let mut contra_reference_edges: Vec<Vec<usize>> = vec![Vec::new(); contras.len()];
    for (s, skip) in skips.iter().enumerate() {
        let (Some(group), Some(reference)) = (skip_groups[s], skip.reference.as_ref()) else {
            continue;
        };
        for &c in group.by_reference.get(reference).into_iter().flatten() {
            if is_edge(skip, contras[c]) {
                reference_edges[s].push(c);
                contra_reference_edges[c].push(s);
            }
        }
    }
    let mut pairs: Vec<Option<(PairRound, usize)>> = vec![None; skips.len()];
    let mut contra_taken = vec![false; contras.len()];
    for (s, edges) in reference_edges.iter().enumerate() {
        if let [c] = edges[..] {
            if contra_reference_edges[c].len() == 1 && is_verified(contras[c]) && !doubled[c] {
                pairs[s] = Some((PairRound::Reference, c));
                contra_taken[c] = true;
            }
        }
    }

    // Round 2: on what round 1 left, one side without a reference, the dates
    // within k. Rows that round 1 contested count against the others here but
    // cannot pair themselves. With k = 0 only the same printed day matches; a
    // transfer printed on different days by the two banks is open until the
    // evidence for k is in.
    let mut loose_edges: Vec<Vec<usize>> = vec![Vec::new(); skips.len()];
    let mut contra_loose_edges: Vec<Vec<usize>> = vec![Vec::new(); contras.len()];
    for (s, skip) in skips.iter().enumerate() {
        let Some(group) = skip_groups[s] else {
            continue;
        };
        if pairs[s].is_some() {
            continue;
        }
        for &c in dates_within(group, &contras, &skip.date, k_days) {
            let contra = contras[c];
            if !contra_taken[c]
                && (skip.reference.is_none() || contra.reference.is_none())
                && is_edge(skip, contra)
            {
                loose_edges[s].push(c);
                contra_loose_edges[c].push(s);
            }
        }
    }
    for (s, edges) in loose_edges.iter().enumerate() {
        if let [c] = edges[..] {
            if reference_edges[s].is_empty()
                && contra_reference_edges[c].is_empty()
                && contra_loose_edges[c].len() == 1
                && is_verified(contras[c])
                && !doubled[c]
                && !reversed(skips[s], contras[c])
            {
                pairs[s] = Some((PairRound::NoReference, c));
            }
        }
    }

    // What either round paired is spoken for.
    let mut contra_paired = vec![false; contras.len()];
    for &(_, c) in pairs.iter().flatten() {
        contra_paired[c] = true;
    }

    let states = skips
        .iter()
        .enumerate()
        .map(|(s, skip)| {
            let state = match pairs[s] {
                // A pair by date claims that nothing else exists, so both
                // statements must be known around the dates it rests on.
                Some((PairRound::NoReference, c))
                    if !(coverage.holds(skip, &skip.other_ledger, &skip.date, k_days)
                        && coverage.holds(
                            skip,
                            &skip.key.bank_ledger,
                            &contras[c].date,
                            k_days,
                        )) =>
                {
                    SkipState::MatchedUnconfirmed {
                        contra: contras[c].key.clone(),
                    }
                }
                Some((round, c)) => SkipState::Paired {
                    round,
                    contra: contras[c].key.clone(),
                },
                None => {
                    // every way the skip could still match, so a verified
                    // Contra is not hidden behind an unverified one
                    let candidates: Vec<usize> = reference_edges[s]
                        .iter()
                        .chain(&loose_edges[s])
                        .copied()
                        .collect();
                    // Only a skip with no way to match is asked whether a
                    // Contra of another reference stands where its own would.
                    let differing = if candidates.is_empty() {
                        differing_references(skip, skip_groups[s], &contras, &contra_paired, k_days)
                    } else {
                        Vec::new()
                    };
                    // A Contra of another reference is said first. Then a
                    // reference edge says why the skip is open as it would for
                    // any row; the flag decides only among candidates found by
                    // date.
                    if !differing.is_empty() {
                        SkipState::ReferenceDiffers {
                            candidates: sorted_keys(&differing, &contras),
                        }
                    } else if reference_edges[s].is_empty()
                        && (skip.reversed_same_day
                            || candidates
                                .iter()
                                .any(|&index| contras[index].reversed_same_day))
                    {
                        SkipState::ReversedSameDay {
                            candidates: sorted_keys(&candidates, &contras),
                        }
                    } else {
                        open_state(skip, &candidates, &contras, &coverage, tally, k_days)
                    }
                }
            };
            (skip.key.clone(), state)
        })
        .collect();

    Ok(TransferView {
        skips: states,
        double_booked,
    })
}

/// The Contras of a group a skip dated `date` can reach within k days: a range
/// of the date-ordered members.
fn dates_within<'g>(
    group: &'g Group,
    contras: &[&ContraFact],
    date: &TallyDate,
    k_days: u32,
) -> &'g [usize] {
    let days_apart =
        |from: &TallyDate, to: &TallyDate| DateSpan::new(from, to).map(|span| span.days());
    let start = group.by_date.partition_point(|&index| {
        let before = &contras[index].date;
        before < date && days_apart(before, date).is_some_and(|days| days > k_days)
    });
    let end = group.by_date.partition_point(|&index| {
        let after = &contras[index].date;
        after <= date || days_apart(date, after).is_some_and(|days| days <= k_days)
    });
    &group.by_date[start..end.max(start)]
}

/// The Contras that stand where a skip's own would, with a reference that is
/// not the skip's: of its group, within k days, an edge, a reference on both
/// rows that differs, and neither paired with a skip nor matched with one by
/// date.
fn differing_references(
    skip: &SkipFact,
    group: Option<&Group>,
    contras: &[&ContraFact],
    contra_paired: &[bool],
    k_days: u32,
) -> Vec<usize> {
    let (Some(group), Some(reference)) = (group, skip.reference.as_ref()) else {
        return Vec::new();
    };
    dates_within(group, contras, &skip.date, k_days)
        .iter()
        .copied()
        .filter(|&c| {
            // The reference test states the rule. For a skip with no candidate it changes nothing:
            // a Contra in reach without a reference would be a date edge, and one whose reference
            // equals the skip's a reference edge.
            !contra_paired[c]
                && contras[c]
                    .reference
                    .as_ref()
                    .is_some_and(|other| other != reference)
                && is_edge(skip, contras[c])
        })
        .collect()
}

/// The edge beyond the group (same company, endpoint, ledgers, direction and
/// amount): a Contra from another lineage, built from the OTHER ledger's own
/// statement. One from the skip's own statement is that statement's own row,
/// and one from a third bank's is a different transfer.
fn is_edge(skip: &SkipFact, contra: &ContraFact) -> bool {
    skip.lineage != contra.lineage && contra.statement_bank_ledger == skip.other_ledger
}

fn is_verified(contra: &ContraFact) -> bool {
    contra.status == BatchStatus::PostedVerified
}

fn within(left: &TallyDate, right: &TallyDate, k_days: u32) -> bool {
    DateSpan::new(left, right)
        .or_else(|| DateSpan::new(right, left))
        .is_some_and(|span| span.days() <= k_days)
}

/// A skip that did not pair: the candidates it has, or the absence of any.
fn open_state(
    skip: &SkipFact,
    candidates: &[usize],
    contras: &[&ContraFact],
    coverage: &MergedCoverage,
    tally: &TallyDays,
    k_days: u32,
) -> SkipState {
    if candidates.is_empty() {
        return if !coverage.holds(skip, &skip.other_ledger, &skip.date, k_days) {
            SkipState::NotEstablished
        } else if tally.found_no_contra(skip) {
            SkipState::NotFound
        } else {
            SkipState::TallyUnchecked
        };
    }
    let unverified: BTreeSet<&BatchId> = candidates
        .iter()
        .filter_map(|&index| match &contras[index].status {
            BatchStatus::PostedVerified => None,
            BatchStatus::NotVerified { batch } => Some(batch),
        })
        .collect();
    if unverified.len() == 1 && candidates.iter().all(|&index| !is_verified(contras[index])) {
        if let Some(batch) = unverified.into_iter().next() {
            return SkipState::OtherSideUnverified {
                batch: batch.clone(),
            };
        }
    }
    SkipState::Ambiguous {
        candidates: sorted_keys(candidates, contras),
    }
}

fn sorted_keys(candidates: &[usize], contras: &[&ContraFact]) -> Vec<ContraKey> {
    let mut keys: Vec<ContraKey> = candidates
        .iter()
        .map(|&index| contras[index].key.clone())
        .collect();
    keys.sort();
    keys
}

/// A flagged row never pairs by date: a reversal and two real transfers look
/// alike. It still counts against the rows that could pair.
fn reversed(skip: &SkipFact, contra: &ContraFact) -> bool {
    skip.reversed_same_day || contra.reversed_same_day
}

/// The verified windows of each ledger, with those that overlap or touch merged
/// into one, built once.
struct MergedCoverage(
    BTreeMap<(CompanyGuid, EndpointOrigin, LedgerGuid), Vec<(TallyDate, TallyDate)>>,
);

impl MergedCoverage {
    fn new(coverage: &Coverage) -> Self {
        let merged = coverage
            .0
            .iter()
            .map(|(key, windows)| {
                let mut windows: Vec<&ParseWindow> = windows.iter().collect();
                windows.sort_by(|left, right| left.from.cmp(&right.from));
                let mut merged: Vec<(TallyDate, TallyDate)> = Vec::new();
                for window in windows {
                    match merged.last_mut() {
                        Some((_, end))
                            if window.from <= *end
                                || end
                                    .next_day()
                                    .is_ok_and(|following| window.from <= following) =>
                        {
                            // a window nested in the last one must not shorten it
                            if window.to > *end {
                                *end = window.to.clone();
                            }
                        }
                        _ => merged.push((window.from.clone(), window.to.clone())),
                    }
                }
                (key.clone(), merged)
            })
            .collect();
        Self(merged)
    }

    /// Whether the ledger's verified windows (same company and endpoint as the
    /// skip) hold the whole `date` -/+ k interval.
    fn holds(&self, skip: &SkipFact, ledger: &LedgerGuid, date: &TallyDate, k_days: u32) -> bool {
        let key = (
            skip.key.company.clone(),
            skip.key.endpoint_origin.clone(),
            ledger.clone(),
        );
        let room = |from: &TallyDate, to: &TallyDate| {
            DateSpan::new(from, to).is_some_and(|span| span.days() >= k_days)
        };
        self.0.get(&key).is_some_and(|windows| {
            windows
                .iter()
                .any(|(from, to)| room(from, date) && room(date, to))
        })
    }
}

/// Contra pairs between two different bank statements for one transfer, and
/// which Contras appear in one. Within a group everything but the statements,
/// the lineages, the references and the dates is already equal.
fn find_double_booked(
    contras: &[&ContraFact],
    groups: &BTreeMap<GroupKey, Group>,
    k_days: u32,
) -> (Vec<DoubleBooked>, Vec<bool>) {
    let two_statements = |left: usize, right: usize| {
        contras[left].statement_bank_ledger != contras[right].statement_bank_ledger
            && contras[left].lineage != contras[right].lineage
    };
    let mut found = Vec::new();
    let mut doubled = vec![false; contras.len()];
    let mut record = |left: usize, right: usize, class: DoubleBookedClass| {
        doubled[left] = true;
        doubled[right] = true;
        let (first, second) = if contras[left].key < contras[right].key {
            (left, right)
        } else {
            (right, left)
        };
        found.push(DoubleBooked {
            first: contras[first].key.clone(),
            second: contras[second].key.clone(),
            class,
        });
    };
    for group in groups.values() {
        // equal references, whatever the dates
        for members in group.by_reference.values() {
            for (position, &left) in members.iter().enumerate() {
                for &right in &members[position + 1..] {
                    if two_statements(left, right) {
                        record(left, right, DoubleBookedClass::RefuseAtBuild);
                    }
                }
            }
        }
        // a reference missing on one side, the dates within k
        for (position, &left) in group.by_date.iter().enumerate() {
            for &right in &group.by_date[position + 1..] {
                if !within(&contras[left].date, &contras[right].date, k_days) {
                    break;
                }
                if (contras[left].reference.is_none() || contras[right].reference.is_none())
                    && two_statements(left, right)
                {
                    record(left, right, DoubleBookedClass::List);
                }
            }
        }
    }
    found.sort_by(|left, right| (&left.first, &left.second).cmp(&(&right.first, &right.second)));
    (found, doubled)
}

#[cfg(test)]
#[path = "agent_transfer_pairing_tests.rs"]
mod tests;
