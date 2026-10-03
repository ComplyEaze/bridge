//! A plain headline for a read result: the company, the exact period and the
//! state in words, built only from typed state.
//!
//! Nothing here reads a result to decide what to say. A tool builds the facts
//! from the same typed value it builds its `state` from, and this module turns
//! them into a sentence. (A byte cap that trims a page reads back only the
//! headline's own `page` numbers, to restate its row sentence.) Two rules are held by the types:
//!
//! - A read is [`Completeness::Whole`] only when it has no [`Gap`]; a read
//!   with any gap is [`Completeness::Partial`], and its headline names every
//!   gap. There is no way to write "whole" over a gap.
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
    /// how many ledgers were set aside, in each of the two ways.
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
