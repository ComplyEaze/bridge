//! Exact summaries of the native amounts already captured by the runtime.
use bridge_tally_core::ExactDecimal;
use bridge_tally_protocol::{
    native_trial_balance::{
        NativeTrialBalance, NativeTrialBalanceAmount, NativeTrialBalanceMovements,
        NativeTrialBalanceRow,
    },
    PartyLedgerMasterFieldObservation,
};
use std::borrow::Cow;

use serde::Serialize;

const MAX_PARENT_OPTIONS: usize = 100;
const PARENT_OPTION_OVERFLOW: usize = 1;
const MAX_RETAINED_PARENT_CANDIDATES: usize = MAX_PARENT_OPTIONS + PARENT_OPTION_OVERFLOW;
// TrialBalanceExportStore admits no retained cell text beyond this budget. A
// returned parent can therefore have this many bytes; its explicit selector
// label adds the longest display prefix below.
const MAX_RETAINED_PARENT_BYTES: usize = 4_096;
const GROUP_LABEL_PREFIX: &str = "Group: ";
const MAX_PARENT_DISPLAY_BYTES: usize = MAX_RETAINED_PARENT_BYTES + GROUP_LABEL_PREFIX.len();
const NOT_RETURNED_PARENT_LABEL: &str = "Missing field: Parent not returned";
const RETURNED_EMPTY_PARENT_LABEL: &str = "Empty field: Parent returned empty";

/// What a ledger whose closing is not its opening plus its debit and credit totals means
/// (#2068), with `{count}` for the number of such ledgers. The MCP answer, the desktop screen
/// and its workbook all say it in these words. Measured on a Closing Stock ledger with a closing
/// balance keyed on Tally's screen, on two synthetic TallyPrime 7.2 books (one integrated with
/// inventory, one not).
pub(crate) const MOVEMENTS_DIFFER_LIMITATION: &str = "{count} ledger(s) carry movements_differ_by: their closing is not their opening plus their debit and credit totals, counting an empty amount as zero. Each is returned as Tally sent it and nothing is corrected. On the books measured this was a Closing Stock ledger whose closing balance was keyed on Tally's screen: Tally kept the opening as the closing and showed the change only in the debit total. The closing of such a ledger is not necessarily the figure Tally's statements use: on the book not integrated with inventory they used the keyed figure, and on the integrated one they valued the stock from the stock item. The column totals include the closing as sent.";

/// The limitation for `rows`, or `None` when every row adds up.
pub(crate) fn movements_differ_limitation<'a>(
    rows: impl IntoIterator<Item = &'a NativeTrialBalanceRow>,
) -> Option<String> {
    let count = rows
        .into_iter()
        .filter(|row| matches!(row.movements(), NativeTrialBalanceMovements::Differ(_)))
        .count();
    (count > 0).then(|| MOVEMENTS_DIFFER_LIMITATION.replace("{count}", &count.to_string()))
}

/// The desktop's only way to write a Trial Balance row (the protocol row is not `Serialize`): its
/// own fields, in the order and form they had before #2068, plus `movements_differ_by` (closing
/// minus opening, debit and credit) on a row whose columns do not add up.
#[derive(Serialize)]
struct RowWithMovements<'a> {
    name: &'a str,
    guid: &'a str,
    parent: &'a PartyLedgerMasterFieldObservation,
    opening: &'a NativeTrialBalanceAmount,
    debit: &'a NativeTrialBalanceAmount,
    credit: &'a NativeTrialBalanceAmount,
    closing: &'a NativeTrialBalanceAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    movements_differ_by: Option<String>,
}

fn rows_with_movements<'a>(
    rows: &'a [NativeTrialBalanceRow],
) -> impl Iterator<Item = RowWithMovements<'a>> {
    rows.iter().map(|row| RowWithMovements {
        name: &row.name,
        guid: &row.guid,
        parent: &row.parent,
        opening: &row.opening,
        debit: &row.debit,
        credit: &row.credit,
        closing: &row.closing,
        movements_differ_by: match row.movements() {
            NativeTrialBalanceMovements::Tie => None,
            NativeTrialBalanceMovements::Differ(gap) => Some(gap),
        },
    })
}

/// `serialize_with` for a Trial Balance's rows wherever the desktop receives them.
pub(crate) fn serialize_rows_with_movements<S: serde::Serializer>(
    rows: &[NativeTrialBalanceRow],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(rows_with_movements(rows))
}

/// `serialize_with` for a whole report, as `{"rows": [...]}` with each row as above.
pub(crate) fn serialize_report_with_movements<S: serde::Serializer>(
    report: &NativeTrialBalance,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    #[derive(Serialize)]
    struct Report<'a> {
        #[serde(serialize_with = "serialize_rows_with_movements")]
        rows: &'a [NativeTrialBalanceRow],
    }
    Report { rows: &report.rows }.serialize(serializer)
}

#[derive(Debug, Clone, Serialize)]
pub struct ObservedAmountTotal {
    pub sum: ExactDecimal,
    pub empty_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrialBalanceTotals {
    pub opening: ObservedAmountTotal,
    pub debit: ObservedAmountTotal,
    pub credit: ObservedAmountTotal,
    pub closing: ObservedAmountTotal,
}

/// A case-preserving follow-up over one already retained Trial Balance capture.
/// It is a row subset, not a qualified financial group balance.
#[derive(Debug, Clone, Serialize)]
pub struct TrialBalanceParentQuery {
    pub parent: PartyLedgerMasterFieldObservation,
    #[serde(serialize_with = "serialize_rows_with_movements")]
    pub selected_rows: Vec<NativeTrialBalanceRow>,
    pub totals: TrialBalanceTotals,
    pub source_row_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialBalanceParentQueryError {
    ParentNotInCapture,
    TotalsUnavailable,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrialBalanceCaptureParentOption {
    pub parent: PartyLedgerMasterFieldObservation,
    pub row_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrialBalanceCaptureParentOptions {
    pub options: Vec<TrialBalanceCaptureParentOption>,
    pub has_more: bool,
    pub source_row_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialBalanceCaptureParentOptionsError {
    SearchInvalid,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ParentMatchRank {
    Exact = 0,
    FoldedExact = 1,
    FoldedSubstring = 2,
}

struct ParentSearch {
    raw: String,
    lowercase: String,
}

impl ParentSearch {
    fn new(raw: String) -> Result<Self, TrialBalanceCaptureParentOptionsError> {
        if raw.len() > MAX_PARENT_DISPLAY_BYTES {
            return Err(TrialBalanceCaptureParentOptionsError::SearchInvalid);
        }
        Ok(Self {
            lowercase: raw.to_lowercase(),
            raw,
        })
    }

    fn rank(&self, parent: &PartyLedgerMasterFieldObservation) -> Option<ParentMatch> {
        // Initial discovery preserves distinct-observation source order and
        // avoids display/lowercase allocations for every retained row.
        if self.raw.is_empty() {
            return Some(ParentMatch::source_order());
        }
        let display = parent_display_label(parent);
        if parent.returned_text() == Some(self.raw.as_str()) || display == self.raw {
            let display_owned_len = match display {
                Cow::Borrowed(_) => 0,
                Cow::Owned(ref value) => value.len(),
            };
            return Some(ParentMatch::exact(display_owned_len));
        }
        let raw_lowercase = parent.returned_text().map(str::to_lowercase);
        let display_lowercase = display.to_lowercase();
        #[cfg(test)]
        let temporary_text_bytes = match &display {
            Cow::Borrowed(_) => 0,
            Cow::Owned(value) => value.len(),
        } + raw_lowercase.as_ref().map_or(0, String::len)
            + display_lowercase.len();
        if raw_lowercase.as_deref() == Some(self.lowercase.as_str())
            || display_lowercase == self.lowercase
        {
            return Some(ParentMatch {
                rank: ParentMatchRank::FoldedExact,
                #[cfg(test)]
                temporary_text_bytes,
            });
        }
        (raw_lowercase
            .as_deref()
            .is_some_and(|raw| raw.contains(&self.lowercase))
            || display_lowercase.contains(&self.lowercase))
        .then_some(ParentMatch {
            rank: ParentMatchRank::FoldedSubstring,
            #[cfg(test)]
            temporary_text_bytes,
        })
    }
}

struct ParentMatch {
    rank: ParentMatchRank,
    #[cfg(test)]
    temporary_text_bytes: usize,
}

impl ParentMatch {
    fn source_order() -> Self {
        Self {
            rank: ParentMatchRank::FoldedSubstring,
            #[cfg(test)]
            temporary_text_bytes: 0,
        }
    }

    fn exact(_display_owned_len: usize) -> Self {
        Self {
            rank: ParentMatchRank::Exact,
            #[cfg(test)]
            temporary_text_bytes: _display_owned_len,
        }
    }
}

fn parent_display_label(parent: &PartyLedgerMasterFieldObservation) -> Cow<'static, str> {
    match parent {
        PartyLedgerMasterFieldObservation::NotObserved => Cow::Borrowed(NOT_RETURNED_PARENT_LABEL),
        PartyLedgerMasterFieldObservation::Returned(value) if value.is_empty() => {
            Cow::Borrowed(RETURNED_EMPTY_PARENT_LABEL)
        }
        PartyLedgerMasterFieldObservation::Returned(value) => {
            Cow::Owned(format!("{GROUP_LABEL_PREFIX}{value}"))
        }
    }
}

struct ParentCandidate<'a> {
    parent: &'a PartyLedgerMasterFieldObservation,
    rank: ParentMatchRank,
    first_index: usize,
    row_count: usize,
}

impl ParentCandidate<'_> {
    fn is_better_than(&self, other: &Self) -> bool {
        (self.rank, self.first_index) < (other.rank, other.first_index)
    }
}

struct ParentOptionScan<'a> {
    candidates: Vec<ParentCandidate<'a>>,
    has_more: bool,
    #[cfg(test)]
    max_retained_candidates: usize,
    #[cfg(test)]
    max_retained_parent_text_bytes: usize,
    #[cfg(test)]
    max_temporary_text_bytes: usize,
}

impl<'a> ParentOptionScan<'a> {
    fn new() -> Self {
        Self {
            candidates: Vec::with_capacity(MAX_RETAINED_PARENT_CANDIDATES),
            has_more: false,
            #[cfg(test)]
            max_retained_candidates: 0,
            #[cfg(test)]
            max_retained_parent_text_bytes: 0,
            #[cfg(test)]
            max_temporary_text_bytes: 0,
        }
    }

    fn retain(
        &mut self,
        parent: &'a PartyLedgerMasterFieldObservation,
        parent_match: ParentMatch,
        index: usize,
    ) {
        #[cfg(test)]
        {
            self.max_temporary_text_bytes = self
                .max_temporary_text_bytes
                .max(parent_match.temporary_text_bytes);
        }
        if let Some(existing) = self
            .candidates
            .iter_mut()
            .find(|candidate| candidate.parent == parent)
        {
            existing.row_count += 1;
            return;
        }
        let candidate = ParentCandidate {
            parent,
            rank: parent_match.rank,
            first_index: index,
            row_count: 1,
        };
        if self.candidates.len() < MAX_RETAINED_PARENT_CANDIDATES {
            self.candidates.push(candidate);
        } else {
            let worst = self
                .candidates
                .iter()
                .enumerate()
                .max_by_key(|(_, candidate)| (candidate.rank, candidate.first_index))
                .expect("nonempty bounded candidate set")
                .0;
            if candidate.is_better_than(&self.candidates[worst]) {
                self.candidates[worst] = candidate;
            }
            self.has_more = true;
        }
        #[cfg(test)]
        {
            self.max_retained_candidates = self.max_retained_candidates.max(self.candidates.len());
            self.max_retained_parent_text_bytes = self.max_retained_parent_text_bytes.max(
                self.candidates
                    .iter()
                    .map(|candidate| candidate.parent.workbook_text().len())
                    .sum(),
            );
        }
    }

    fn finish(mut self, source_row_count: usize) -> TrialBalanceCaptureParentOptions {
        self.candidates
            .sort_by_key(|candidate| (candidate.rank, candidate.first_index));
        let has_more = self.has_more || self.candidates.len() > MAX_PARENT_OPTIONS;
        TrialBalanceCaptureParentOptions {
            options: self
                .candidates
                .into_iter()
                .take(MAX_PARENT_OPTIONS)
                .map(|candidate| TrialBalanceCaptureParentOption {
                    parent: candidate.parent.clone(),
                    row_count: candidate.row_count,
                })
                .collect(),
            has_more,
            source_row_count,
        }
    }
}

/// Lists at most 100 distinct observed parents from one retained capture. The
/// scan borrows retained row text and keeps at most one overflow candidate;
/// it never builds a parent index or acquires a new Tally response.
pub fn list_observed_capture_parents(
    report: &NativeTrialBalance,
    search: String,
) -> Result<TrialBalanceCaptureParentOptions, TrialBalanceCaptureParentOptionsError> {
    let search = ParentSearch::new(search)?;
    Ok(scan_observed_capture_parents(report, &search).finish(report.rows.len()))
}

fn scan_observed_capture_parents<'a>(
    report: &'a NativeTrialBalance,
    search: &ParentSearch,
) -> ParentOptionScan<'a> {
    let mut scan = ParentOptionScan::new();
    for (index, row) in report.rows.iter().enumerate() {
        if let Some(parent_match) = search.rank(&row.parent) {
            scan.retain(&row.parent, parent_match, index);
        }
    }
    scan
}

/// These are sums of numeric observations. Empty native fields remain counted
/// separately, so a column sum cannot masquerade as fully observed arithmetic.
pub fn observed_totals(
    report: &NativeTrialBalance,
) -> Result<TrialBalanceTotals, bridge_tally_core::TallyError> {
    observed_totals_for_rows(report.rows.iter())
}

fn observed_totals_for_rows<'a>(
    rows: impl IntoIterator<Item = &'a NativeTrialBalanceRow>,
) -> Result<TrialBalanceTotals, bridge_tally_core::TallyError> {
    let mut totals = std::array::from_fn::<_, 4, _>(|_| ObservedAmountTotal {
        sum: ExactDecimal::zero(),
        empty_count: 0,
    });
    for row in rows {
        for (total, amount) in
            totals
                .iter_mut()
                .zip([&row.opening, &row.debit, &row.credit, &row.closing])
        {
            match amount {
                NativeTrialBalanceAmount::Present(value) => {
                    total.sum = total.sum.checked_add(value)?
                }
                NativeTrialBalanceAmount::PresentEmpty => total.empty_count += 1,
            }
        }
    }
    let [opening, debit, credit, closing] = totals;
    Ok(TrialBalanceTotals {
        opening,
        debit,
        credit,
        closing,
    })
}

/// Selects only rows whose source parent observation exactly matches `parent`.
/// `NotObserved` and a returned empty string intentionally remain distinct.
/// See `docs/tally/TALLY_PROTOCOL_REFERENCE.md` §5.6 for the capture contract.
pub fn query_observed_parent(
    report: &NativeTrialBalance,
    parent: &PartyLedgerMasterFieldObservation,
) -> Result<TrialBalanceParentQuery, TrialBalanceParentQueryError> {
    let selected_rows = report
        .rows
        .iter()
        .filter(|row| row.parent == *parent)
        .cloned()
        .collect::<Vec<_>>();
    if selected_rows.is_empty() {
        return Err(TrialBalanceParentQueryError::ParentNotInCapture);
    }
    let totals = observed_totals_for_rows(selected_rows.iter())
        .map_err(|_| TrialBalanceParentQueryError::TotalsUnavailable)?;
    Ok(TrialBalanceParentQuery {
        parent: parent.clone(),
        selected_rows,
        totals,
        source_row_count: report.rows.len(),
    })
}

#[cfg(test)]
#[path = "trial_balance_tests.rs"]
pub(crate) mod tests;
