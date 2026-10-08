//! A plain headline for a read result: the company, the exact period and the
//! state in words, built only from typed state.
//!
//! Nothing here reads a result to decide what to say. A tool builds the facts
//! from the same typed value it builds its `state` from, and this module turns
//! them into a sentence. (A byte cap that trims a page reads back only the
//! headline's own `page` numbers, to restate its row sentence.) Two rules are held by the types:
//!
//! - A read is whole only when it has no [`Gap`] ([`Completeness::is_whole`]);
//!   a read with any gap is partial, and its headline names every gap. There
//!   is no way to write "whole" over a gap.
//! - Rows are described by a [`Page`], and a later byte cap that trims a page
//!   restates the row sentence from the rows that are left
//!   ([`restate_rows`]), so the headline never claims rows the response no
//!   longer holds.
//!
//! The company name is Tally's own unrestricted text, so it is written in quotes with
//! control characters removed, and no ledger or party name ever enters a
//! headline: the headline types hold counts, dates and enums, and one
//! [`CompanyName`].
use super::*;
use crate::reports::statements::{Established, NotEstablishedReason};
use serde::Deserialize;

/// The company a headline names, as Tally spells it, made safe to quote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CompanyName(String);

impl CompanyName {
    /// Control characters become spaces (so words are not joined), invisible
    /// formatting characters (bidirectional overrides and isolates,
    /// zero-width marks) are removed so that a name cannot reorder or hide the
    /// words around it, quotes inside it become apostrophes, runs of spaces
    /// collapse, and it is held to [`MAX_COMPANY_NAME_CHARS`].
    pub(super) fn new(name: &str) -> Self {
        let cleaned = name
            .chars()
            .filter(|character| !is_invisible_format(*character))
            .map(|character| match character {
                '\u{201c}' | '\u{201d}' | '"' => '\'',
                other if other.is_control() => ' ',
                other => other,
            })
            .take(MAX_COMPANY_NAME_CHARS)
            .collect::<String>();
        Self(cleaned.split_whitespace().collect::<Vec<_>>().join(" "))
    }

    fn quoted(&self) -> String {
        format!("\u{201c}{}\u{201d}", self.0)
    }
}

/// Longest company name a headline carries. The `company` block beside it
/// has the whole name.
const MAX_COMPANY_NAME_CHARS: usize = 200;

/// Characters that draw nothing but can change how the text around them reads:
/// Unicode General_Category Cf and Default_Ignorable_Code_Point (the same rule
/// the native approval screen applies to the names it shows), so U+061C, the
/// tag characters and every future format character are covered without a list.
fn is_invisible_format(character: char) -> bool {
    use icu_properties::{
        props::{DefaultIgnorableCodePoint, GeneralCategory},
        CodePointMapData, CodePointSetData,
    };
    CodePointSetData::new::<DefaultIgnorableCodePoint>().contains(character)
        || CodePointMapData::<GeneralCategory>::new().get(character) == GeneralCategory::Format
}

/// A date as a person reads it: `1 Apr 2026`, never `01/04/2026`.
pub(super) fn plain_date(date: &bridge_tally_core::TallyDate) -> String {
    NaiveDate::parse_from_str(date.as_str(), "%Y%m%d")
        .map(|parsed| parsed.format("%-d %b %Y").to_string())
        .unwrap_or_else(|_| date.as_str().to_string())
}

/// One way a read does not cover everything it was asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Gap {
    /// A several-currency book read for its plain base-currency ledgers only:
    /// how many ledgers were set aside, in each of the two ways. Even with
    /// none set aside this is partial: the tool's own scope
    /// (`ledgers_scope: base_currency_ledgers_only`) and its first limitation
    /// say so, and the headline agrees with them and says that nothing was
    /// left out, rather than calling a different scope whole.
    BaseCurrencyLedgersOnly { foreign: usize, mixed: usize },
}

impl Gap {
    /// What is partial, in a few words, and what it costs the reader.
    fn short(&self) -> String {
        match self {
            Self::BaseCurrencyLedgersOnly {
                foreign: 0,
                mixed: 0,
            } => "base-currency ledgers only, and no ledger was left out".to_string(),
            Self::BaseCurrencyLedgersOnly { .. } => {
                "base-currency ledgers only, so debit and credit totals are not expected to match"
                    .to_string()
            }
        }
    }

    /// The counts behind it.
    fn detail(&self) -> String {
        match self {
            Self::BaseCurrencyLedgersOnly { foreign: 0, mixed: 0 } => {
                "The book has several currency masters, so only its base-currency ledgers were read."
                    .to_string()
            }
            Self::BaseCurrencyLedgersOnly { foreign, mixed } => format!(
                "{foreign} {} kept in another currency and {mixed} base-currency {} with a value Tally shows in another currency are left out (the result names up to {} of each kind).",
                ledgers(*foreign),
                ledgers(*mixed),
                super::trial_balance::EXCLUDED_TRIAL_BALANCE_LEDGERS_NAMED,
            ),
        }
    }
}

fn ledgers(count: usize) -> &'static str {
    if count == 1 {
        "ledger"
    } else {
        "ledgers"
    }
}

/// Whether a read covers everything it was asked for: the gaps it has, and
/// nothing else. The only way to build one is [`Completeness::from_gaps`] and
/// the field is private, so a read beside a gap cannot be written as whole and
/// a whole read has no gap to name; "whole" is the empty list and is decided
/// here, by [`Completeness::is_whole`], never by a caller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Completeness {
    gaps: Vec<Gap>,
}

impl Completeness {
    pub(super) fn from_gaps(gaps: Vec<Gap>) -> Self {
        Self { gaps }
    }

    pub(super) fn is_whole(&self) -> bool {
        self.gaps.is_empty()
    }
}

/// The rows a result lists, as the words for them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Rows {
    Ledgers,
}

impl Rows {
    fn plural(self) -> &'static str {
        match self {
            Self::Ledgers => "ledgers",
        }
    }
}

/// Which rows of the whole a response lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct Page {
    rows: Rows,
    offset: usize,
    shown: usize,
    total: usize,
}

impl Page {
    pub(super) fn new(rows: Rows, offset: usize, shown: usize, total: usize) -> Self {
        Self {
            rows,
            offset,
            shown,
            total,
        }
    }

    fn sentence(&self) -> String {
        let Self {
            rows,
            offset,
            shown,
            total,
        } = *self;
        let name = rows.plural();
        let first = offset.saturating_add(1);
        let last = offset.saturating_add(shown);
        if total == 0 {
            format!("There are no {name} in this read.")
        } else if shown == 0 {
            format!("No {name} on this page: there are {total} in all, and offset {offset} is at or past the end.")
        } else if offset == 0 && shown >= total {
            format!("All {total} {name} are listed.")
        } else if last >= total {
            format!(
                "{} {first} to {last} of {total}: the last page.",
                capitalised(name)
            )
        } else {
            format!(
                "{} {first} to {last} of {total}; more follow, from offset {last}.",
                capitalised(name)
            )
        }
    }
}

fn capitalised(word: &str) -> String {
    let mut characters = word.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect(),
        None => String::new(),
    }
}

/// The headline a result carries beside `result`: `lead` says what was read
/// and in what state, and `rows` says which rows this response lists.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct Headline {
    lead: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rows: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    page: Option<Page>,
}

/// What a trial balance read covers, held beside its snapshot so a page served
/// from it states the same coverage as the first page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TrialBalanceBasis {
    from: bridge_tally_core::TallyDate,
    to: bridge_tally_core::TallyDate,
    completeness: Completeness,
}

impl TrialBalanceBasis {
    pub(super) fn new(
        from: bridge_tally_core::TallyDate,
        to: bridge_tally_core::TallyDate,
        completeness: Completeness,
    ) -> Self {
        Self {
            from,
            to,
            completeness,
        }
    }

    /// The headline of one page of this trial balance.
    pub(super) fn headline(&self, company: &CompanyName, page: Page) -> Headline {
        let subject = format!(
            "trial balance for {}, {} to {}",
            company.quoted(),
            plain_date(&self.from),
            plain_date(&self.to)
        );
        let lead = if self.completeness.is_whole() {
            format!("{}: read for every ledger.", capitalised(&subject))
        } else {
            // The first sentence says it is partial and what that costs; the
            // counts follow it.
            let gaps = &self.completeness.gaps;
            let short = gaps.iter().map(Gap::short).collect::<Vec<_>>().join("; ");
            let detail = gaps.iter().map(Gap::detail).collect::<Vec<_>>().join(" ");
            format!("Partial {subject}: {short}. {detail}")
        };
        Headline {
            lead,
            rows: Some(page.sentence()),
            page: Some(page),
        }
    }
}

/// Which statement a headline is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StatementKind {
    ProfitAndLoss,
    BalanceSheet,
}

/// One result a statement carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StatementPart {
    GrossResult,
    NetResult,
    BalanceSheetProfitAndLoss,
}

impl StatementPart {
    fn words(self) -> &'static str {
        match self {
            Self::GrossResult => "the gross result",
            Self::NetResult => "the net result",
            Self::BalanceSheetProfitAndLoss => "the profit and loss line of the balance sheet",
        }
    }
}

/// Whether one result of a statement is established, and if not why.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PartOutcome {
    Established,
    NotEstablished {
        reason: NotEstablishedReason,
        /// How many lines did not tie: a Tally line that differs, a Tally line
        /// carrying an amount that nothing derived was compared with, or a
        /// derived line Tally has no counterpart for; for a profit and loss,
        /// also the Cost of Sales heading when it is off the derived cost of
        /// sales (a count; no names).
        differing_lines: usize,
    },
}

impl PartOutcome {
    /// The outcome of one derived result, by an exhaustive match.
    pub(super) fn of(result: &Established) -> Self {
        match result {
            Established::Established { .. } => Self::Established,
            Established::NotEstablished { reason, lines } => Self::NotEstablished {
                reason: *reason,
                differing_lines: lines.len(),
            },
        }
    }
}

fn reason_words(reason: NotEstablishedReason) -> &'static str {
    match reason {
        NotEstablishedReason::UnclassifiedLedgerCarriesAnAmount => {
            "a ledger the derivation cannot classify carries an amount"
        }
        NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance => {
            "the book has a Stock-in-Hand balance and closing stock is not derived"
        }
        NotEstablishedReason::ProfitAndLossLedgerNotReturned => {
            "Tally did not return its Profit & Loss A/c ledger"
        }
        NotEstablishedReason::TallyBalanceSheetDiffers => {
            "Tally's own Balance Sheet differs from the derived lines"
        }
        NotEstablishedReason::TallyProfitAndLossDiffers => {
            "Tally's own Profit and Loss differs from the derived lines"
        }
    }
}

/// What to do about it, for each reason, by an exhaustive match.
fn reason_next_step(reason: NotEstablishedReason) -> &'static str {
    match reason {
        NotEstablishedReason::UnclassifiedLedgerCarriesAnAmount => {
            "Ask the user to look at the ledgers the result lists as unclassified."
        }
        NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance => {
            "Tell the user to read Tally's own Profit and Loss and Balance Sheet for this period."
        }
        NotEstablishedReason::ProfitAndLossLedgerNotReturned => {
            "Tell the user Tally did not return the ledger the carried line needs, and ask them to check the book in Tally."
        }
        NotEstablishedReason::TallyBalanceSheetDiffers => {
            "See balance_sheet_gate in the result for the lines that differ, and tell the user."
        }
        NotEstablishedReason::TallyProfitAndLossDiffers => {
            "See tie_out in the result for the lines that differ, and tell the user."
        }
    }
}

/// What a profit and loss or balance sheet read established, held for its
/// headline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct StatementBasis {
    kind: StatementKind,
    from: bridge_tally_core::TallyDate,
    to: bridge_tally_core::TallyDate,
    parts: Vec<(StatementPart, PartOutcome)>,
    /// Whether Tally's own Profit and Loss was read as well. It only counts
    /// for a profit and loss: only its results are gated on it.
    read_tally_profit_and_loss: bool,
}

impl StatementBasis {
    pub(super) fn new(
        kind: StatementKind,
        from: bridge_tally_core::TallyDate,
        to: bridge_tally_core::TallyDate,
        parts: Vec<(StatementPart, PartOutcome)>,
        read_tally_profit_and_loss: bool,
    ) -> Self {
        Self {
            kind,
            from,
            to,
            parts,
            read_tally_profit_and_loss,
        }
    }

    /// Whether the result's own comparison included Tally's Profit and Loss:
    /// only a profit and loss that read it.
    fn compared_with_tally_profit_and_loss(&self) -> bool {
        self.kind == StatementKind::ProfitAndLoss && self.read_tally_profit_and_loss
    }

    /// Whether the derived lines are withheld from the result: the tool shows
    /// them only once the net result (or the balance sheet's carried line) is
    /// established.
    pub(super) fn lines_withheld(&self) -> bool {
        self.parts.iter().any(|(part, outcome)| {
            matches!(
                part,
                StatementPart::NetResult | StatementPart::BalanceSheetProfitAndLoss
            ) && !matches!(outcome, PartOutcome::Established)
        })
    }

    pub(super) fn headline(&self, company: &CompanyName) -> Headline {
        let name = match self.kind {
            StatementKind::ProfitAndLoss => "profit and loss",
            StatementKind::BalanceSheet => "balance sheet",
        };
        let subject = format!(
            "{name} for {}, {} to {}",
            company.quoted(),
            plain_date(&self.from),
            plain_date(&self.to)
        );
        let every_part_established = self
            .parts
            .iter()
            .all(|(_, outcome)| matches!(outcome, PartOutcome::Established));
        let lead = if every_part_established {
            let established = self
                .parts
                .iter()
                .map(|(part, _)| part.words())
                .collect::<Vec<_>>()
                .join(" and ");
            let against = if self.compared_with_tally_profit_and_loss() {
                "Tally's own Balance Sheet and Profit and Loss"
            } else {
                "Tally's own Balance Sheet"
            };
            let verb = if self.parts.len() == 1 { "is" } else { "are" };
            format!(
                "{}: {established} {verb} established, after the derived lines passed the comparison with {against}.",
                capitalised(&subject)
            )
        } else {
            let sentences = self
                .parts
                .iter()
                .map(|(part, outcome)| match outcome {
                    PartOutcome::Established => {
                        format!("{} is established.", capitalised(part.words()))
                    }
                    PartOutcome::NotEstablished {
                        reason,
                        differing_lines,
                    } => format!(
                        "{} is not established because {}{}.",
                        capitalised(part.words()),
                        reason_words(*reason),
                        match differing_lines {
                            0 => String::new(),
                            1 => " (on 1 line)".to_string(),
                            count => format!(" (on {count} lines)"),
                        }
                    ),
                })
                .collect::<Vec<_>>()
                .join(" ");
            let withheld = if self.lines_withheld() {
                " The derived lines are withheld."
            } else {
                ""
            };
            // One next step for each distinct reason, in the order they appear.
            let mut reasons = Vec::new();
            for (_, outcome) in &self.parts {
                if let PartOutcome::NotEstablished { reason, .. } = outcome {
                    if !reasons.contains(reason) {
                        reasons.push(*reason);
                    }
                }
            }
            let next = reasons
                .iter()
                .map(|reason| reason_next_step(*reason))
                .collect::<Vec<_>>()
                .join(" ");
            format!("Not established: the {subject}. {sentences}{withheld} {next}")
        };
        Headline {
            lead,
            rows: None,
            page: None,
        }
    }
}

/// What a cash flow read established, held for its headline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CashFlowOutcome {
    /// The net total equals the trial balance's cash and bank ledgers.
    Tied,
    /// The two differ.
    Differs,
    /// A Bank OD A/c or Bank OCC A/c ledger has movement in the period, and
    /// Tally's Cash Flow was seen counting one such ledger, which the check does not count.
    MoneyGroupUnmeasured,
    /// No amount on either side (no cash or bank ledger with an amount, and an
    /// empty Cash Flow): nothing was compared.
    NothingToCompare,
}

/// What a cash flow read covered and found, held for its headline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CashFlowBasis {
    from: bridge_tally_core::TallyDate,
    to: bridge_tally_core::TallyDate,
    outcome: CashFlowOutcome,
    /// The answer holds something not yet compared with Tally's own Cash Flow
    /// screen (a credit amount, a positive closing, a window across March into April).
    unmeasured_shape: bool,
}

impl CashFlowBasis {
    pub(super) fn new(
        from: bridge_tally_core::TallyDate,
        to: bridge_tally_core::TallyDate,
        outcome: CashFlowOutcome,
        unmeasured_shape: bool,
    ) -> Self {
        Self {
            from,
            to,
            outcome,
            unmeasured_shape,
        }
    }

    /// Whether the months are withheld: they are shown only once the net total
    /// is checked, so a month's figure is never shown beside a total Tally's own
    /// ledgers contradict.
    pub(super) fn months_withheld(&self) -> bool {
        self.outcome != CashFlowOutcome::Tied
    }

    pub(super) fn headline(&self, company: &CompanyName) -> Headline {
        let subject = format!(
            "cash flow for {}, {} to {}",
            company.quoted(),
            plain_date(&self.from),
            plain_date(&self.to)
        );
        let lead = match self.outcome {
            CashFlowOutcome::Tied => format!(
                "{}: Tally's net cash and bank movement for the whole period equals the cash and bank ledgers of its trial balance. The split into months is Tally's own and has not been checked. Tally's debit and credit columns are not shown, and a negative amount is a debit (cash and bank growing).{} This is Tally's month-wise cash and bank movement, not a cash flow statement under AS 3.",
                capitalised(&subject),
                if self.unmeasured_shape {
                    " This answer holds a credit amount, a positive closing or a window from March into April, a shape of which a credit amount (two windows, one with a positive closing) tied to the trial balance in live runs on one synthetic book, a window from March into April was not run, and no month was compared with Tally's own Cash Flow screen: the net total agrees, but treat each month's figure as unverified and compare it with Tally's own Cash Flow."
                } else {
                    " No month's figure has been checked against Tally's own Cash Flow screen."
                }
            ),
            CashFlowOutcome::Differs => format!(
                "Not established: the {subject}. Tally's net cash and bank figure for the period differs from the cash and bank ledgers of its trial balance, so the months are withheld. Do not give either figure as the cash movement: ask the user to open Cash Flow in Tally for the same period."
            ),
            CashFlowOutcome::MoneyGroupUnmeasured => format!(
                "Not established: the {subject}. A ledger under Bank OD A/c or Bank OCC A/c has movement in the period, and Tally's Cash Flow was seen counting such a ledger, which this check does not count, so the months are withheld. Tell the user that cash flow for this period has to be read in Tally."
            ),
            CashFlowOutcome::NothingToCompare => format!(
                "Not established: the {subject}. Nothing could be tied: either neither the trial balance's cash and bank ledgers (Cash-in-Hand and Bank Accounts) nor Tally's Cash Flow carry any amount for the period (an empty month is not a zero), or both add up to zero, which would agree under any reading of the columns. The months are withheld. Tell the user that cash flow for this period has to be read in Tally."
            ),
        };
        Headline {
            lead,
            rows: None,
            page: None,
        }
    }
}

impl Headline {
    /// The same headline after a byte cap left `shown` rows of its page.
    fn restated(mut self, shown: usize) -> Self {
        if let Some(page) = self.page.as_mut() {
            page.shown = shown.min(page.shown);
            self.rows = Some(page.sentence());
        }
        self
    }
}

/// Restates a response's headline for the `shown` rows a byte cap left, in
/// place. A response without a headline, or a headline that lists no rows, is
/// left as it is.
pub(super) fn restate_rows(response: &mut Value, shown: usize) {
    let Some(headline) = response.get("headline").cloned() else {
        return;
    };
    if let Ok(current) = serde_json::from_value::<Headline>(headline) {
        if let Ok(value) = serde_json::to_value(current.restated(shown)) {
            response["headline"] = value;
            return;
        }
    }
    // A headline that cannot be restated fails closed: its rows sentence goes,
    // rather than staying beside rows it no longer describes.
    if let Some(fields) = response["headline"].as_object_mut() {
        fields.remove("rows");
        fields.remove("page");
    }
}

#[cfg(test)]
#[path = "agent_headline_tests.rs"]
mod tests;
