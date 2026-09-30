//! The ledger census: how many ledgers a book holds, counted from the ledgers'
//! GUIDs by AlterID span instead of read as ledgers (#679).
//!
//! A book's ledger count decides how its compliance master is read (whole, in
//! parent parts, or refused). Until now the count came from reading the whole
//! catalogue (name, GUID, parent of every ledger) in one response, which is
//! bounded only by the book's master-alteration mark (`ALTMSTID`), an upper
//! bound that every master of every type and every alteration raises. A book
//! whose mark is large but whose ledgers are few was refused unread.
//!
//! The census asks for the ledgers in slices `(after, through]` of AlterID,
//! fetching each row's GUID only. AlterIDs are taken to be distinct (PARTIAL:
//! no two ledgers shared one on any book read, which is not a proof), so a
//! slice holds at most `through - after` ledgers, and the rows of the slices, taken together,
//! are the book's ledgers if the slices cover `(0, mark]` and the book did not
//! change while they were read (the caller brackets the census with the
//! opening and closing company extent; nothing here proves that).
//!
//! The rules are types rather than reminders (AGENTS.md P2):
//! - a [`LedgerCensusSlice`] can only come from a [`LedgerCensusPlan`], so a
//!   slice with `after >= through`, or one past the width the caller bounded,
//!   is not constructible;
//! - a [`LedgerCensus`] hands out its slices in order, one at a time, and
//!   [`LedgerCensus::finish`] is the only way to obtain a [`LedgerCount`]: it
//!   refuses a census that stopped early, that saw a GUID twice, or that saw
//!   no ledger at all.
//!
//! What is measured (docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md,
//! section 11e, PARTIAL): the `$AlterID > a AND $AlterID <= b` formula, on three
//! books (marks 5,547, 102,161 and 316,028; slices of 800, 1,000, 4,000 and
//! 8,000 AlterIDs), returned rows only within the slice, and the union of the
//! slices' GUIDs equalled the whole catalogue's, with no GUID twice. Nothing
//! else about the slice is measured; every bound here is a [`CensusLimits`]
//! argument chosen by the caller.

use std::collections::HashSet;

use crate::xml_text::escape_text as xml_escape;

/// The name of the `SYSTEM` formula a census slice request adds.
pub const CENSUS_FORMULA_NAME: &str = "BridgeSpan";

/// The bounds a census plan must fit. The caller chooses them from what it has
/// measured; nothing in this module knows a response size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CensusLimits {
    /// The most AlterIDs one slice spans, hence the most ledgers it may return.
    pub slice_width: u64,
    /// The most slices one census may make.
    pub max_slices: usize,
}

/// Why a census could not be planned, or why its slices did not add up to a
/// ledger count. Every variant is data-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerCensusError {
    /// The plan was asked for a mark of zero or a slice width of zero.
    PlanInvalid,
    /// The mark needs more slices than the limit allows.
    TooManySlices { slices: u64 },
    /// A slice returned more rows than its span can hold, so Tally did not
    /// apply the slice's filter.
    SliceOverBound { rows: u64 },
    /// A GUID came back twice, in one slice or in two.
    DuplicateIdentity,
    /// Every slice answered with no ledger. A book of no ledgers is not
    /// distinguishable here from a company that is closed or absent (both
    /// answer an empty slice with the same body), so it is refused.
    CensusEmpty,
    /// `finish` was called before every slice was accepted.
    Incomplete,
}

impl LedgerCensusError {
    /// A stable, data-free name for the failure, for a refusal's `cause`.
    pub const fn safe_code(self) -> &'static str {
        match self {
            Self::PlanInvalid => "ledger_span_plan_invalid",
            Self::TooManySlices { .. } => "ledger_span_too_many_slices",
            Self::SliceOverBound { .. } => "ledger_span_slice_over_bound",
            Self::DuplicateIdentity => "ledger_span_duplicate_identity",
            Self::CensusEmpty => "ledger_span_census_empty",
            Self::Incomplete => "ledger_span_incomplete",
        }
    }
}

impl std::fmt::Display for LedgerCensusError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::PlanInvalid => "the ledger census plan was invalid",
            Self::TooManySlices { .. } => "the ledger census needs more slices than allowed",
            Self::SliceOverBound { .. } => "a ledger census slice returned more rows than it spans",
            Self::DuplicateIdentity => "the ledger census saw one ledger twice",
            Self::CensusEmpty => "the ledger census found no ledger",
            Self::Incomplete => "the ledger census stopped before its last slice",
        })
    }
}

impl std::error::Error for LedgerCensusError {}

/// One slice `(after, through]` of AlterID. Only a [`LedgerCensusPlan`] builds
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedgerCensusSlice {
    after: u64,
    through: u64,
}

impl LedgerCensusSlice {
    /// The AlterID the slice starts after (exclusive).
    pub fn after(&self) -> u64 {
        self.after
    }

    /// The last AlterID the slice includes.
    pub fn through(&self) -> u64 {
        self.through
    }

    /// The most ledgers the slice can return: AlterIDs are distinct.
    pub fn width(&self) -> u64 {
        self.through - self.after
    }
}

/// The slices that cover `(0, mark]`, each at most the limit's width.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerCensusPlan {
    slices: Vec<LedgerCensusSlice>,
}

impl LedgerCensusPlan {
    pub fn new(mark: u64, limits: CensusLimits) -> Result<Self, LedgerCensusError> {
        if mark == 0 || limits.slice_width == 0 || limits.max_slices == 0 {
            return Err(LedgerCensusError::PlanInvalid);
        }
        // Whole slices plus a remainder, without the `mark + width - 1` that
        // could overflow.
        let slices =
            mark / limits.slice_width + u64::from(!mark.is_multiple_of(limits.slice_width));
        if slices > limits.max_slices as u64 {
            return Err(LedgerCensusError::TooManySlices { slices });
        }
        let mut planned = Vec::with_capacity(slices as usize);
        let mut after = 0_u64;
        while after < mark {
            let through = after.saturating_add(limits.slice_width).min(mark);
            planned.push(LedgerCensusSlice { after, through });
            after = through;
        }
        Ok(Self { slices: planned })
    }

    pub fn slices(&self) -> &[LedgerCensusSlice] {
        &self.slices
    }
}

/// A ledger count that only a complete, duplicate-free, non-empty census
/// produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedgerCount(u64);

impl LedgerCount {
    pub fn get(self) -> u64 {
        self.0
    }
}

/// A census in progress: its slices are taken in order and each slice's GUIDs
/// are accepted before the next is offered.
#[derive(Debug)]
pub struct LedgerCensus {
    plan: LedgerCensusPlan,
    accepted: usize,
    seen: HashSet<String>,
}

impl LedgerCensus {
    pub fn new(plan: LedgerCensusPlan) -> Self {
        Self {
            plan,
            accepted: 0,
            seen: HashSet::new(),
        }
    }

    /// The slice to ask for next, or `None` once every slice was accepted.
    pub fn next_slice(&self) -> Option<LedgerCensusSlice> {
        self.plan.slices.get(self.accepted).copied()
    }

    /// Takes the GUIDs one slice returned. A slice holding more rows than its
    /// span, or a GUID already seen, refuses the census; nothing is kept from a
    /// refused slice.
    pub fn accept(&mut self, guids: Vec<String>) -> Result<(), LedgerCensusError> {
        let slice = self.next_slice().ok_or(LedgerCensusError::Incomplete)?;
        let rows = guids.len() as u64;
        if rows > slice.width() {
            return Err(LedgerCensusError::SliceOverBound { rows });
        }
        let mut fresh = HashSet::with_capacity(guids.len());
        for guid in guids {
            let guid = guid.to_ascii_lowercase();
            if self.seen.contains(&guid) || !fresh.insert(guid) {
                return Err(LedgerCensusError::DuplicateIdentity);
            }
        }
        self.seen.extend(fresh);
        self.accepted += 1;
        Ok(())
    }

    /// The ledger count, once every slice was accepted and at least one ledger
    /// was seen.
    pub fn finish(self) -> Result<LedgerCount, LedgerCensusError> {
        if self.accepted != self.plan.slices.len() {
            return Err(LedgerCensusError::Incomplete);
        }
        if self.seen.is_empty() {
            return Err(LedgerCensusError::CensusEmpty);
        }
        Ok(LedgerCount(self.seen.len() as u64))
    }
}

/// The census request for one slice: the `List of Ledgers` collection fetching
/// each ledger's GUID and the company's GUID, restricted to the slice's AlterID
/// span. These are the bytes measured on the synthetic book (section 11e,
/// PARTIAL); the request has no period variables because AlterID is not
/// period-scoped.
///
/// The formula holds only digits and `$AlterID`, no `$$` function, so section
/// 6.1's space hazard does not apply. Tally's row still carries the ledger's
/// name twice whatever is fetched; the response's size is what the caller's
/// `slice_width` bounds.
pub fn render_ledger_census_slice_request(company: &str, slice: &LedgerCensusSlice) -> String {
    format!(
        r#"<ENVELOPE>
    <HEADER>
        <VERSION>1</VERSION>
        <TALLYREQUEST>EXPORT</TALLYREQUEST>
        <TYPE>COLLECTION</TYPE>
        <ID>List of Ledgers</ID>
    </HEADER>
    <BODY>
        <DESC>
            <STATICVARIABLES>
                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
                <SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>
            </STATICVARIABLES>
            <TDL>
                <TDLMESSAGE>
                    <SYSTEM TYPE="Formulae" NAME="{formula}">$AlterID &gt; {after} AND $AlterID &lt;= {through}</SYSTEM>
                    <COLLECTION NAME="List of Ledgers" ISMODIFY="Yes">
                        <NATIVEMETHOD>GUID</NATIVEMETHOD>
                        <COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>
                        <FILTERS>{formula}</FILTERS>
                    </COLLECTION>
                </TDLMESSAGE>
            </TDL>
        </DESC>
    </BODY>
</ENVELOPE>"#,
        company = xml_escape(company),
        formula = CENSUS_FORMULA_NAME,
        after = slice.after,
        through = slice.through,
    )
}

#[cfg(test)]
#[path = "ledger_census_tests.rs"]
mod tests;
