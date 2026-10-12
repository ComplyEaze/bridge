//! The transfer pairing view: for each bank line skipped as "recorded from the
//! other bank's statement", whether that other side exists as a posted Contra.
//!
//! A pure function of plain typed facts. It reads no journal, holds no state and
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
//!   built from the OTHER bank's statement (one from the skip's own statement
//!   is that statement's own row).
//! * Round 1, by reference: both rows carry a reference and they are equal.
//!   A pair forms only where the skip and the Contra each have exactly one such
//!   edge.
//! * Round 2, without a reference: on what round 1 left, one side has no
//!   reference, and the dates are within k days. A pair forms only where both ends have exactly one such
//!   edge. A row that took part in round 1 without pairing does not pair here,
//!   although it still counts against the rows that could.
//!
//! Unverified Contras count as candidates (a second possible match must make a
//! skip ambiguous, not paired) but never pair: a skip whose candidates are all
//! in one batch not yet `posted_verified` is `OtherSideUnverified`.
//!
//! A skip with no candidate is `NotFound` only when the other bank ledger's
//! verified parse windows cover its whole date +/- k interval; otherwise the
//! absence proves nothing and it is `NotEstablished`.
//!
//! Key uniqueness is the caller's precondition: two skips with one `SkipKey`, or
//! two Contras with one `ContraKey`, are one fact read twice.

use bridge_bank_statement::bank::BankReference;
use bridge_tally_core::{DateSpan, ExactDecimal, TallyDate};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Side {
    Debit,
    Credit,
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
    pub(crate) endpoint_origin: EndpointOrigin,
    pub(crate) lineage: LineageId,
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
    ) {
        self.0
            .entry((company, endpoint_origin, bank_ledger))
            .or_default()
            .push(window);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PairRound {
    Reference,
    NoReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SkipState {
    Paired { round: PairRound, contra: ContraKey },
    NotFound,
    Ambiguous { candidates: Vec<ContraKey> },
    OtherSideUnverified { batch: BatchId },
    NotEstablished,
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

pub(crate) fn transfer_pairing_view(
    skips: &[SkipFact],
    contras: &[ContraFact],
    coverage: &Coverage,
    k_days: u32,
) -> TransferView {
    let mut skips: Vec<&SkipFact> = skips.iter().collect();
    skips.sort_by(|left, right| left.key.cmp(&right.key));
    skips.dedup_by(|left, right| left.key == right.key);
    let mut contras: Vec<&ContraFact> = contras.iter().collect();
    contras.sort_by(|left, right| left.key.cmp(&right.key));
    contras.dedup_by(|left, right| left.key == right.key);

    // Contras bucketed by (company, endpoint, ledger pair), so a skip looks only
    // at the transfers between its own two ledgers.
    let mut buckets: BTreeMap<Bucket, Vec<usize>> = BTreeMap::new();
    for (index, contra) in contras.iter().enumerate() {
        if let Some(bucket) = contra_bucket(contra) {
            buckets.entry(bucket).or_default().push(index);
        }
    }

    // `structural[s]`: the Contras that are an edge of skip `s` before any
    // reference or date is looked at.
    let structural: Vec<Vec<usize>> = skips
        .iter()
        .map(|skip| {
            skip_bucket(skip)
                .and_then(|bucket| buckets.get(&bucket))
                .map(|indices| {
                    indices
                        .iter()
                        .copied()
                        .filter(|&index| is_edge(skip, contras[index]))
                        .collect()
                })
                .unwrap_or_default()
        })
        .collect();

    // Round 1: equal references.
    let mut reference_edges: Vec<Vec<usize>> = Vec::new();
    let mut contra_reference_edges: Vec<Vec<usize>> = vec![Vec::new(); contras.len()];
    for (s, edges) in structural.iter().enumerate() {
        let matching: Vec<usize> = edges
            .iter()
            .copied()
            .filter(|&c| skips[s].reference.is_some() && skips[s].reference == contras[c].reference)
            .collect();
        for &c in &matching {
            contra_reference_edges[c].push(s);
        }
        reference_edges.push(matching);
    }
    let mut pairs: Vec<Option<(PairRound, usize)>> = vec![None; skips.len()];
    let mut contra_taken = vec![false; contras.len()];
    for (s, edges) in reference_edges.iter().enumerate() {
        if let [c] = edges[..] {
            if contra_reference_edges[c].len() == 1 && is_verified(contras[c]) {
                pairs[s] = Some((PairRound::Reference, c));
                contra_taken[c] = true;
            }
        }
    }

    // Round 2: on what round 1 left, one side without a reference, the dates
    // within k. Rows that round 1 contested count
    // against the others here but cannot pair themselves.
    let mut loose_edges: Vec<Vec<usize>> = vec![Vec::new(); skips.len()];
    let mut contra_loose_edges: Vec<Vec<usize>> = vec![Vec::new(); contras.len()];
    for (s, edges) in structural.iter().enumerate() {
        if pairs[s].is_some() {
            continue;
        }
        for &c in edges {
            if !contra_taken[c] && is_loose_edge(skips[s], contras[c], k_days) {
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
            {
                pairs[s] = Some((PairRound::NoReference, c));
            }
        }
    }

    let states = skips
        .iter()
        .enumerate()
        .map(|(s, skip)| {
            let state = match pairs[s] {
                Some((round, c)) => SkipState::Paired {
                    round,
                    contra: contras[c].key.clone(),
                },
                None => {
                    let candidates = if reference_edges[s].is_empty() {
                        &loose_edges[s]
                    } else {
                        &reference_edges[s]
                    };
                    open_state(skip, candidates, &contras, coverage, k_days)
                }
            };
            (skip.key.clone(), state)
        })
        .collect();

    TransferView {
        skips: states,
        double_booked: double_booked(&contras, &buckets, k_days),
    }
}

/// Company, endpoint and the two ledgers, in a fixed order.
type Bucket = (CompanyGuid, EndpointOrigin, LedgerGuid, LedgerGuid);

fn bucket(
    company: &CompanyGuid,
    endpoint: &EndpointOrigin,
    first: &LedgerGuid,
    second: &LedgerGuid,
) -> Option<Bucket> {
    (first != second).then(|| {
        let (low, high) = if first < second {
            (first, second)
        } else {
            (second, first)
        };
        (company.clone(), endpoint.clone(), low.clone(), high.clone())
    })
}

fn skip_bucket(skip: &SkipFact) -> Option<Bucket> {
    bucket(
        &skip.key.company,
        &skip.endpoint_origin,
        &skip.key.bank_ledger,
        &skip.other_ledger,
    )
}

/// A Contra has a bucket only when its legs are two distinct ledgers on
/// opposite sides, which is what a Contra is.
fn contra_bucket(contra: &ContraFact) -> Option<Bucket> {
    let [first, second] = &contra.legs;
    if first.side == second.side {
        return None;
    }
    bucket(
        &contra.company,
        &contra.endpoint_origin,
        &first.ledger,
        &second.ledger,
    )
}

fn side_on<'a>(contra: &'a ContraFact, ledger: &LedgerGuid) -> Option<&'a Side> {
    contra
        .legs
        .iter()
        .find(|leg| &leg.ledger == ledger)
        .map(|leg| &leg.side)
}

/// The structural edge: the same bucket (checked by the caller's lookup), the
/// Contra's side on the skip's own bank ledger the one its direction needs,
/// equal amounts, another lineage, and a Contra built from the OTHER bank's
/// statement: one from the skip's own statement is that statement's own row,
/// whatever reference it carries.
fn is_edge(skip: &SkipFact, contra: &ContraFact) -> bool {
    side_on(contra, &skip.key.bank_ledger) == Some(&skip.direction.contra_side())
        && skip.amount.numeric_eq(&contra.amount)
        && skip.lineage != contra.lineage
        && contra.statement_bank_ledger != skip.key.bank_ledger
}

fn is_loose_edge(skip: &SkipFact, contra: &ContraFact, k_days: u32) -> bool {
    (skip.reference.is_none() || contra.reference.is_none())
        && within(&skip.date, &contra.date, k_days)
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
    coverage: &Coverage,
    k_days: u32,
) -> SkipState {
    if candidates.is_empty() {
        return if covers(coverage, skip, k_days) {
            SkipState::NotFound
        } else {
            SkipState::NotEstablished
        };
    }
    // contras are sorted by key, so are the candidate indices
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
    let mut keys: Vec<ContraKey> = candidates
        .iter()
        .map(|&index| contras[index].key.clone())
        .collect();
    keys.sort();
    SkipState::Ambiguous { candidates: keys }
}

/// Whether the other ledger's verified windows hold the skip's whole
/// date -/+ k interval. Windows that overlap or touch are one window.
fn covers(coverage: &Coverage, skip: &SkipFact, k_days: u32) -> bool {
    let Some(windows) = coverage.0.get(&(
        skip.key.company.clone(),
        skip.endpoint_origin.clone(),
        skip.other_ledger.clone(),
    )) else {
        return false;
    };
    let mut windows: Vec<&ParseWindow> = windows
        .iter()
        .filter(|window| window.from <= window.to)
        .collect();
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
                if window.to > *end {
                    *end = window.to.clone();
                }
            }
            _ => merged.push((window.from.clone(), window.to.clone())),
        }
    }
    let room = |from: &TallyDate, to: &TallyDate| {
        DateSpan::new(from, to).is_some_and(|span| span.days() >= k_days)
    };
    merged
        .iter()
        .any(|(from, to)| room(from, &skip.date) && room(&skip.date, to))
}

/// Contra pairs between two different bank statements for one transfer.
fn double_booked(
    contras: &[&ContraFact],
    buckets: &BTreeMap<Bucket, Vec<usize>>,
    k_days: u32,
) -> Vec<DoubleBooked> {
    let mut found = Vec::new();
    for (bucket, indices) in buckets {
        let low = &bucket.2;
        for (position, &left) in indices.iter().enumerate() {
            for &right in &indices[position + 1..] {
                let (first, second) = (contras[left], contras[right]);
                let same_transfer = side_on(first, low) == side_on(second, low)
                    && first.amount.numeric_eq(&second.amount)
                    && first.statement_bank_ledger != second.statement_bank_ledger
                    && first.lineage != second.lineage;
                if !same_transfer {
                    continue;
                }
                let class = match (&first.reference, &second.reference) {
                    (Some(left_ref), Some(right_ref)) => {
                        (left_ref == right_ref).then_some(DoubleBookedClass::RefuseAtBuild)
                    }
                    _ => {
                        within(&first.date, &second.date, k_days).then_some(DoubleBookedClass::List)
                    }
                };
                if let Some(class) = class {
                    found.push(DoubleBooked {
                        first: first.key.clone(),
                        second: second.key.clone(),
                        class,
                    });
                }
            }
        }
    }
    found.sort_by(|left, right| (&left.first, &left.second).cmp(&(&right.first, &right.second)));
    found
}

#[cfg(test)]
#[path = "agent_transfer_pairing_tests.rs"]
mod tests;
