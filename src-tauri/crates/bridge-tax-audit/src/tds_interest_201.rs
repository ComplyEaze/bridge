// SPDX-License-Identifier: Apache-2.0
//! s.201(1A) interest on TDS not deducted, or deducted and not paid (Form 3CD clause 34(a) and
//! (c)): for each day a payee's or a partner's credits attracted TDS, the range of interest that
//! may be due until the deduction and deposit dates are known. A port of the reference Python
//! implementation's `tds_interest_201` test module, version 1; its contract is the spec pack in
//! `docs/tax-audit/spec-packs/tds_interest_201/`.
//!
//! It never reads the book. [`assemble_rows`] builds its rows from the results of `tds_payees` and
//! `partners_40b_194t` and from their tranche tables ([`PayeeTranches`], [`PartnerTranches`]); the
//! test reads only those rows, the rules' `[s201_1a]` and `[s206c_7]`, and the as-of date it is
//! given. A row holds no deducted, paid or payee filing date and no TCS section, as no caller
//! supplies one: every row is priced with those dates not supplied, from a minimum of 0 to the
//! largest amount any date of deduction gives.
//!
//! Where this port differs from the reference, in cases no golden reaches:
//!
//! * two rows with one tag are refused, as [`TestResult::fig`] refuses a repeated figure id, where
//!   the reference's later row overwrites the earlier one's figures;
//! * a s.194H row with no `[tds_rates].s194h_bp`, and a s.194J row of a category other than the
//!   four, are refused;
//! * the module check ([`check_invariants`]) recomputes only a row whose deducted date is not
//!   supplied; for a row with one, which no caller builds, it checks only that no
//!   still-not-deducted amount is published;
//! * the module check refuses a result without its as-of date or a rate figure, where the
//!   reference raises.

use std::collections::BTreeMap;

use bridge_tally_primitives::TallyDate;

use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::partners_40b_194t::{self, PartnerTranches, DEFAULT_S194T_RATE_BP};
use crate::read::{iso, iso_date};
use crate::rules::{Rules, TdsRates};
use crate::support::{count, hash8, overflow, py_repr_str};
use crate::tds_payees::{self, Foreseeability, PayeeTranches};
use crate::tds_tranches::RefTranche;

pub const TEST_ID: &str = "tds_interest_201";
pub const VERSION: &str = "1";

const CLAUSES: [&str; 3] = ["s.201(1A)", "3CD-34(a)", "3CD-34(c)"];
/// The value of a deducted or paid date the books and documents do not establish.
const NOT_SUPPLIED: &str = "not supplied";
/// The `tds_payees` sections that give a row, in the order that test walks them.
const PAYEE_SECTIONS: [&str; 4] = ["194C", "194I", "194J", "194H"];
/// `[s201_1a]` when the rules have none: (before, after, authority, status).
const DEFAULT_S201_1A: (i64, i64, &str, &str) = (100, 150, "s.201(1A)", "confirm");
/// `[s206c_7]` when the rules have none: (rate, authority, status).
const DEFAULT_S206C_7: (i64, &str, &str) = (100, "s.206C(7)", "confirm");

const POPULATION_NOTE: &str = "Not walked from the books population: every row is one (section, \
payee) tax amount already identified as a shortfall by 'TDS on payments made' or 'Partners \
(s.40(b), s.194T)', with whatever deduction/payment dates the books or documents establish. An \
unsupplied date is never guessed; it drives a range instead.";
const ASK_CLIENT: [&str; 2] = [
    "Provide TDS/TCS challans and the TAN to confirm actual deduction/collection and deposit dates \
for this payee.",
    "If not deducted/collected, confirm whether Form 26A (or the equivalent TCS relief) is \
available, and the payee's own return-filing date.",
];

const LIMIT_DATES: &str = "Books/documents alone cannot establish the actual deduction/collection \
and deposit dates; challans, the TAN and (for the pre-deduction leg) Form 26A are needed before \
any single interest amount can be confirmed -- this is a range, not a computed liability.";
const LIMIT_QUARTERLY: &str = "Tie this range to the quarterly TDS statement (Form 26Q) filed for \
each credit: TRACES computes s.201(1A) interest from the dates of credit or payment and of \
deduction reported there. Where the statement reports other dates, or omits a credit, the route is \
a correction statement filed through TRACES; the interest then follows the dates as corrected.";
const LIMIT_DEFAULT_RATE: &str = "The s.201(1A) rate used here is a local prototype default \
(status=\"confirm\"), pending confirmation against the bare Act/notification text -- not yet a \
verified rule.";
const LIMIT_NO_FILING: &str = "No payee return-filing date was supplied, so the Form 26A (first \
proviso to s.201(1)) relief scenario could not be computed for this row.";
const LIMIT_ONE_OFF: &str = "Classified as one-off work: the maximum is a range the CA chooses \
from, from the day the year's total crossed the limit (each tranche's interest maximum) to each \
credit's own date (each tranche's own-date maximum).";
const LIMIT_UNCLASSIFIED: &str = "Whether this payee's credits were foreseeable is not classified \
in the client's list of payees, so the maximum runs each credit's tax from its own date. If the \
work was genuinely one-off, the CA may instead run the earlier credits from the day the year's \
total crossed the limit: classify the payee to show that range.";
const LIMIT_FLAG: &str = "Whether this assessee is a deductor at all for the year is itself \
unresolved (previous-year turnover unconfirmed -- see tds_payees' own deductor_status finding); \
this interest range is conditional on deductor status being confirmed, not a standalone \
conclusion.";

const BASIS_194C: &str = "Taken on the credits that attract TDS under s.194C(5) and its proviso \
(as read here, pending the CA's confirmation of that reading): a credit over the single-sum limit \
from its own date; once the year's aggregate exceeds the aggregate limit, every credit, the \
earlier ones from the date the aggregate is crossed. The aggregate is gross and a crossing is \
never undone: a reversal lowers nothing. One the client's list classifies as reversing a \
particular bill, or as an error, is named on the payee's finding but not applied, pending \
bill-wise detail from the books.";
const BASIS_194I: &str = "Taken on the credits of a month whose total exceeds the s.194-I monthly \
limit (as read here, pending the CA's confirmation of that reading), from the day that month's \
total crosses it. The aggregate is gross and a crossing is never undone: a reversal lowers \
nothing. One the client's list classifies as reversing a particular bill, or as an error, is named \
on the payee's finding but not applied, pending bill-wise detail from the books.";
const BASIS_194J: &str = "Taken on the credits that attract TDS under s.194J (as read here, \
pending the CA's confirmation of that reading): once the payee's aggregate in this category \
exceeds the limit, every credit in it, the earlier ones from the date the aggregate is crossed. \
The aggregate is gross and a crossing is never undone: a reversal lowers nothing. One the client's \
list classifies as reversing a particular bill, or as an error, is named on the payee's finding \
but not applied, pending bill-wise detail from the books.";
const BASIS_194H: &str = "Taken on the credits that attract TDS under s.194H (as read here, \
pending the CA's confirmation of that reading): once the payee's aggregate in the financial year \
exceeds the limit, every credit, the earlier ones from the date the aggregate is crossed. The \
aggregate is gross and a crossing is never undone: a reversal lowers nothing. One the client's \
list classifies as reversing a particular bill, or as an error, is named on the payee's finding \
but not applied, pending bill-wise detail from the books.";
const BASIS_194T: &str = "Taken on the interest and remuneration credited that attract TDS under \
s.194T (as read here, pending the CA's confirmation of that reading): once the partner's aggregate \
exceeds the limit, every credit, the earlier ones from the date the aggregate is crossed. The \
aggregate is gross and a crossing is never undone: a reversal lowers nothing.";

/// The section a row's interest is computed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    S194C,
    S194I,
    S194J,
    S194H,
    S194T,
}

impl Section {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::S194C => "194C",
            Self::S194I => "194I",
            Self::S194J => "194J",
            Self::S194H => "194H",
            Self::S194T => "194T",
        }
    }

    /// How the section's credits attract TDS, as read here (the reference's `BASIS_TEXT`).
    fn basis(self) -> &'static str {
        match self {
            Self::S194C => BASIS_194C,
            Self::S194I => BASIS_194I,
            Self::S194J => BASIS_194J,
            Self::S194H => BASIS_194H,
            Self::S194T => BASIS_194T,
        }
    }
}

/// Which of a section's rates a row is taken at: a section with two, where the payee's type is not
/// known, gives a row at each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateTag {
    Lower,
    Higher,
    Single,
}

impl RateTag {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lower => "lower_rate",
            Self::Higher => "higher_rate",
            Self::Single => "single_rate",
        }
    }

    /// The sentence that ends a two-rate row's tax definition.
    fn sentence(self) -> &'static str {
        match self {
            Self::Lower => {
                " At the lower of the section's two rates (the payee's type is not \
known)."
            }
            Self::Higher => {
                " At the higher of the section's two rates (the payee's type is not \
known)."
            }
            Self::Single => "",
        }
    }

    /// The phrase a two-rate group's title and tax definition carry.
    fn phrase(self) -> &'static str {
        match self {
            Self::Lower => " (at the lower of its section's two rates)",
            Self::Higher => " (at the higher of its section's two rates)",
            Self::Single => "",
        }
    }

    /// Whether a row at this rate is in the higher scenario (`higher`) or the lower one: a
    /// single-rate row is in both.
    fn in_scenario(self, higher: bool) -> bool {
        match self {
            Self::Single => true,
            Self::Lower => !higher,
            Self::Higher => higher,
        }
    }
}

/// One row: one tranche of one payee or partner at one rate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub section: Section,
    /// `<row id>_<rate tag>_<tranche date, ISO>`.
    pub payee_key: String,
    /// `<row id>_<rate tag>`.
    pub group: String,
    /// The tranche's paise at the rate, half up.
    pub tax: i64,
    /// The tranche's date.
    pub deductible: TallyDate,
    /// The deductor status is unresolved (never set on a partner's row).
    pub flag: bool,
    pub rate_tag: RateTag,
    pub rate_bp: i64,
    /// Each credit of the tranche as (its own date, its own tax, half up).
    pub credits: Vec<(TallyDate, i64)>,
    /// `None` when the client's list does not classify the payee or partner.
    pub foreseeability: Option<Foreseeability>,
    /// The input finding's voucher refs that are the tranche's own.
    pub evidence: Vec<EvidenceRef>,
}

/// The tax on `paise` at `rate_bp`, half up, floored as Python's `//` floors.
fn tax_at(paise: i64, rate_bp: i64) -> Result<i64> {
    let scaled = i128::from(paise)
        .checked_mul(i128::from(rate_bp))
        .and_then(|x| x.checked_add(5000))
        .ok_or_else(|| overflow(TEST_ID))?;
    i64::try_from(scaled.div_euclid(10_000)).map_err(|_| overflow(TEST_ID))
}

/// "Deductor status unresolved": `tds_payees`' deductor status is `unknown`, or the previous
/// year's turnover is a placeholder.
pub fn deductor_uncertain(payees: &TestResult, turnover_is_placeholder: bool) -> Result<bool> {
    Ok(deductor_status(payees)? == "unknown" || turnover_is_placeholder)
}

fn deductor_status(payees: &TestResult) -> Result<&str> {
    let id = format!("{}.deductor_status", tds_payees::TEST_ID);
    match payees.figures.iter().find(|f| f.id == id).map(|f| &f.value) {
        Some(Value::Text(status)) => Ok(status),
        _ => Err(AuditError::Config(format!(
            "{TEST_ID}: the {} result has no {id} figure",
            tds_payees::TEST_ID
        ))),
    }
}

/// A payee row's rates from `[tds_rates]`, in row order. `category` is the row id's second part.
fn payee_rates(
    section: &str,
    category: &str,
    t: &TdsRates,
) -> Result<(Section, Vec<(RateTag, i64)>)> {
    let two = |lower, higher| vec![(RateTag::Lower, lower), (RateTag::Higher, higher)];
    let refuse = |what: String| AuditError::Config(format!("{TEST_ID}: {what}"));
    Ok(match section {
        "194C" => (
            Section::S194C,
            two(t.s194c_individual_huf_bp, t.s194c_other_bp),
        ),
        "194I" => (
            Section::S194I,
            two(t.s194i_plant_machinery_bp, t.s194i_land_building_bp),
        ),
        "194J" => (
            Section::S194J,
            match category {
                "professional" => vec![(RateTag::Single, t.s194j_professional_bp)],
                "technical" => vec![(RateTag::Single, t.s194j_technical_bp)],
                "royalty" | "28va" => two(t.s194j_technical_bp, t.s194j_professional_bp),
                other => {
                    return Err(refuse(format!(
                        "a s.194J row of category {} has no rate",
                        py_repr_str(other)
                    )))
                }
            },
        ),
        "194H" => (
            Section::S194H,
            vec![(
                RateTag::Single,
                t.s194h_bp.ok_or_else(|| {
                    refuse("a s.194H row needs rules [tds_rates].s194h_bp".into())
                })?,
            )],
        ),
        other => return Err(refuse(format!("{other} is not a payee section"))),
    })
}

/// What every row of one payee or partner shares.
struct Source<'a> {
    section: Section,
    row_id: &'a str,
    flag: bool,
    foreseeability: Option<Foreseeability>,
    /// The input finding's evidence.
    evidence: &'a [EvidenceRef],
}

/// One row per tranche, in date order, and per rate, in `rates`' order.
fn push_rows(
    rows: &mut Vec<Row>,
    src: &Source,
    tranches: &[RefTranche],
    rates: &[(RateTag, i64)],
) -> Result<()> {
    for t in tranches {
        let day = iso(&t.day);
        let evidence: Vec<EvidenceRef> = src
            .evidence
            .iter()
            .filter(|e| {
                e.kind == "voucher" && t.refs.iter().any(|r| r.id == e.id && r.label == e.label)
            })
            .cloned()
            .collect();
        for &(rate_tag, rate_bp) in rates {
            let credits = t
                .credits
                .iter()
                .map(|(d, paise)| tax_at(*paise, rate_bp).map(|tax| (d.clone(), tax)))
                .collect::<Result<Vec<_>>>()?;
            rows.push(Row {
                section: src.section,
                payee_key: format!("{}_{}_{day}", src.row_id, rate_tag.as_str()),
                group: format!("{}_{}", src.row_id, rate_tag.as_str()),
                tax: tax_at(t.paise, rate_bp)?,
                deductible: t.day.clone(),
                flag: src.flag,
                rate_tag,
                rate_bp,
                credits,
                foreseeability: src.foreseeability,
                evidence: evidence.clone(),
            });
        }
    }
    Ok(())
}

/// The rows, as the reference's caller assembles them: payee rows from the `tds_payees` findings
/// in the order that test raised them, then partner rows from the `partners_40b_194t` findings in
/// theirs. `flag` is [`deductor_uncertain`]; `foreseeability` is `[tds_payees.foreseeability]`,
/// matched by exact text.
///
/// # Errors
///
/// Refuses without `[tds_rates]`, on every book; for a qualifying finding whose row id is in
/// neither tranche table; and where a row has no rate (see the module docs).
pub fn assemble_rows(
    rules: &Rules,
    payees: &TestResult,
    payee_tranches: &PayeeTranches,
    partners: &TestResult,
    partner_tranches: &PartnerTranches,
    flag: bool,
    foreseeability: &BTreeMap<String, Foreseeability>,
) -> Result<Vec<Row>> {
    let rates = rules
        .tds_rates
        .ok_or_else(|| AuditError::Config(format!("{TEST_ID} needs rules [tds_rates]")))?;
    let not_in_table = |id: &str| {
        AuditError::Config(format!(
            "{TEST_ID}: finding {id} gives a row, and its row id is in neither tranche table"
        ))
    };
    let mut rows = Vec::new();
    if deductor_status(payees)? != "not_deductor" {
        let prefix = format!("{}/", tds_payees::TEST_ID);
        for f in &payees.findings {
            let Some(row_id) = f.id.strip_prefix(prefix.as_str()) else {
                continue;
            };
            let (section, rest) = row_id.split_once('_').unwrap_or((row_id, ""));
            if !PAYEE_SECTIONS.contains(&section) || !f.facts.iter().any(|(n, _)| n == "credited") {
                continue;
            }
            let entry = payee_tranches
                .get(row_id)
                .ok_or_else(|| not_in_table(&f.id))?;
            let category = rest.split('_').next().unwrap_or("");
            let (section, row_rates) = payee_rates(section, category, &rates)?;
            let src = Source {
                section,
                row_id,
                flag,
                foreseeability: foreseeability.get(&entry.entity).copied(),
                evidence: &f.evidence,
            };
            push_rows(&mut rows, &src, &entry.tranches, &row_rates)?;
        }
    }
    let prefix = format!("{}/s194t/", partners_40b_194t::TEST_ID);
    let partner_rate = rules.s194t.map_or(DEFAULT_S194T_RATE_BP, |t| t.rate_bp);
    for f in &partners.findings {
        let Some(tag) = f.id.strip_prefix(prefix.as_str()) else {
            continue;
        };
        if !f.facts.iter().any(|(n, _)| n == "tds_expected") {
            continue;
        }
        let entry = partner_tranches
            .get(tag)
            .ok_or_else(|| not_in_table(&f.id))?;
        let src = Source {
            section: Section::S194T,
            row_id: tag,
            flag: false,
            foreseeability: foreseeability.get(&entry.key).copied(),
            evidence: &f.evidence,
        };
        push_rows(
            &mut rows,
            &src,
            &entry.tranches,
            &[(RateTag::Single, partner_rate)],
        )?;
    }
    Ok(rows)
}

/// A date's month counted from year 0: a `TallyDate` is eight checked digits.
fn month_index(d: &TallyDate) -> i64 {
    let digits = |s: &str| s.bytes().fold(0_i64, |a, c| a * 10 + i64::from(c - b'0'));
    let s = d.as_str();
    digits(&s[0..4]) * 12 + digits(&s[4..6])
}

/// Months, "month or part of a month", from `start` to `end`: the calendar months the span
/// touches, 0 when `end` is on or before `start`.
fn months(start: &TallyDate, end: &TallyDate) -> i64 {
    if end <= start {
        0
    } else {
        month_index(end) - month_index(start) + 1
    }
}

/// Interest on `tax` at `rate_bp` a month for `months` months, half up; 0 when any of the three is
/// 0 or less.
fn interest(tax: i64, rate_bp: i64, months: i64) -> Result<i64> {
    if tax <= 0 || rate_bp <= 0 || months <= 0 {
        return Ok(0);
    }
    let scaled = i128::from(tax)
        .checked_mul(i128::from(rate_bp))
        .and_then(|x| x.checked_mul(i128::from(months)))
        .and_then(|x| x.checked_add(5000))
        .ok_or_else(|| overflow(TEST_ID))?;
    i64::try_from(scaled / 10_000).map_err(|_| overflow(TEST_ID))
}

/// The `[s201_1a]` rates and the as-of date every row is priced with.
struct Pricing<'a> {
    before: i64,
    after: i64,
    as_of: &'a TallyDate,
}

impl Pricing<'_> {
    /// The largest interest on `tax` over every date of deduction from `start` to the as-of date,
    /// both included: at the pre-deduction rate to that date, and the post-deduction rate from it,
    /// each leg rounded on its own. 0 when `start` is after the as-of date.
    fn maximum(&self, tax: i64, start: &TallyDate) -> Result<i64> {
        let mut best = 0_i64;
        let mut day = start.clone();
        while day <= *self.as_of {
            let both = interest(tax, self.before, months(start, &day))?
                .checked_add(interest(tax, self.after, months(&day, self.as_of))?)
                .ok_or_else(|| overflow(TEST_ID))?;
            best = best.max(both);
            day = day
                .next_day()
                .map_err(|e| AuditError::Config(format!("{TEST_ID}: {e}")))?;
        }
        Ok(best)
    }
}

/// A priced row: what its group's finding and the totals read.
struct Priced {
    minimum: i64,
    maximum: i64,
    maximum_own_date: i64,
    /// The finding's per-tranche fact names (after `tranche_<n>_`) and figure ids.
    facts: Vec<(&'static str, String)>,
    limits: Vec<String>,
}

/// A row's limits, in the reference's order.
fn row_limits(row: &Row, as_of: &str, default_rate: bool) -> Vec<String> {
    let mut limits = vec![LIMIT_DATES.to_string(), LIMIT_QUARTERLY.to_string()];
    if default_rate {
        limits.push(LIMIT_DEFAULT_RATE.to_string());
    }
    limits.push(format!(
        "Where the date of deduction is not known, the interest is computed two \
ways as at {as_of}. If the tax is still not deducted: at the pre-deduction rate to {as_of}. If it \
was deducted and not deposited: the largest amount over every possible date of deduction up to \
{as_of}, at the pre-deduction rate to that date and the post-deduction rate from it; a month that \
both periods touch is counted at both rates. Both count calendar months or part of one, and no \
month after {as_of}; a count by periods of thirty days is not computed and may give a higher \
figure."
    ));
    limits.push(LIMIT_NO_FILING.to_string());
    limits.push(row.section.basis().to_string());
    match row.foreseeability {
        Some(Foreseeability::Foreseeable) => {}
        Some(Foreseeability::OneOff) => limits.push(LIMIT_ONE_OFF.to_string()),
        None => limits.push(LIMIT_UNCLASSIFIED.to_string()),
    }
    if row.flag {
        limits.push(LIMIT_FLAG.to_string());
    }
    limits
}

/// A row's fifteen figures; `p` is its position in the list of rows.
#[allow(clippy::too_many_lines)] // one figure each, as the reference lays them out
fn price_row(
    r: &mut TestResult,
    row: &Row,
    p: usize,
    pricing: &Pricing,
    as_of: &str,
    default_rate: bool,
) -> Result<Priced> {
    let section = row.section.as_str();
    let t = hash8(&format!("{section}:{}:{p}", row.payee_key));
    let k = py_repr_str(&row.payee_key);
    let (basis, sentence) = (row.section.basis(), row.rate_tag.sentence());
    let stage_max = months(&row.deductible, pricing.as_of);
    let minimum = 0_i64;
    let not_deducted = interest(row.tax, pricing.before, stage_max)?;
    let maximum = pricing.maximum(row.tax, &row.deductible)?;
    let (mut not_deducted_own_date, mut maximum_own_date) = (0_i64, 0_i64);
    for (day, tax) in &row.credits {
        not_deducted_own_date = not_deducted_own_date
            .checked_add(interest(*tax, pricing.before, months(day, pricing.as_of))?)
            .ok_or_else(|| overflow(TEST_ID))?;
        maximum_own_date = maximum_own_date
            .checked_add(pricing.maximum(*tax, day)?)
            .ok_or_else(|| overflow(TEST_ID))?;
    }
    let ev = || row.evidence.clone();
    r.fig(
        &format!("section_{t}"),
        Value::Text(section.to_string()),
        Unit::Text,
        &format!("Section this row's interest is computed under (row {p}, payee {k}, tag {t})."),
        Vec::new(),
    )?;
    let f_tax = r.fig(
        &format!("tax_{t}"),
        Value::Int(row.tax),
        Unit::Paise,
        &format!(
            "Tax this row's interest is computed on (row {p}, payee {k}, tag {t}) -- carried \
in from tds_payees/partners_40b_194t, not re-derived here. {basis}{sentence}"
        ),
        ev(),
    )?;
    r.fig(
        &format!("rate_bp_{t}"),
        Value::Int(row.rate_bp),
        Unit::BasisPoints,
        &format!("The rate this row's tax is taken at (row {p}, tag {t})."),
        Vec::new(),
    )?;
    let f_deductible = r.fig(
        &format!("deductible_date_{t}"),
        Value::Text(iso(&row.deductible)),
        Unit::Text,
        &format!(
            "Date tax became deductible/collectible (row {p}, tag {t}): the earlier of credit \
and payment for most sections."
        ),
        Vec::new(),
    )?;
    r.fig(
        &format!("deducted_date_{t}"),
        Value::Text(NOT_SUPPLIED.to_string()),
        Unit::Text,
        &format!(
            "Date tax was actually deducted/collected (row {p}, tag {t}); 'not supplied' when \
books/documents do not establish it -- drives the range below, never guessed."
        ),
        Vec::new(),
    )?;
    r.fig(
        &format!("paid_date_{t}"),
        Value::Text(NOT_SUPPLIED.to_string()),
        Unit::Text,
        &format!(
            "Date the deducted/collected tax was actually paid/deposited (row {p}, tag {t}); \
'not supplied' when books/documents do not establish it."
        ),
        Vec::new(),
    )?;
    r.fig(
        &format!("months_stage1_min_{t}"),
        Value::Int(0),
        Unit::Count,
        &format!("s.201(1A) pre-deduction months (or part), minimum bound, row {p} tag {t}."),
        Vec::new(),
    )?;
    r.fig(
        &format!("months_stage1_max_{t}"),
        Value::Int(stage_max),
        Unit::Count,
        &format!("s.201(1A) pre-deduction months (or part), maximum bound, row {p} tag {t}."),
        Vec::new(),
    )?;
    r.fig(
        &format!("months_stage2_min_{t}"),
        Value::Int(0),
        Unit::Count,
        &format!("s.201(1A) post-deduction months (or part), minimum bound, row {p} tag {t}."),
        Vec::new(),
    )?;
    r.fig(
        &format!("months_stage2_max_{t}"),
        Value::Int(stage_max),
        Unit::Count,
        &format!("s.201(1A) post-deduction months (or part), maximum bound, row {p} tag {t}."),
        Vec::new(),
    )?;
    let f_min = r.fig(
        &format!("interest_min_{t}"),
        Value::Int(minimum),
        Unit::Paise,
        &format!(
            "s.201(1A) interest, minimum bound (row {p} tag {t}): known dates used where \
supplied; any unsupplied leg is assumed complete with zero further delay for this floor."
        ),
        ev(),
    )?;
    let f_not_deducted = r.fig(
        &format!("interest_if_not_deducted_{t}"),
        Value::Int(not_deducted),
        Unit::Paise,
        &format!(
            "s.201(1A) interest if the tax is still not deducted at as_of ({as_of}) (row {p} \
tag {t}): the pre-deduction rate for the months from the deductible date."
        ),
        ev(),
    )?;
    let f_max = r.fig(
        &format!("interest_max_{t}"),
        Value::Int(maximum),
        Unit::Paise,
        &format!(
            "s.201(1A) interest, maximum bound (row {p} tag {t}), if the tax was deducted and \
not deposited: the largest over every deduction date from the deductible date to as_of ({as_of}), \
at the pre-deduction rate to that date and the post-deduction rate from it."
        ),
        ev(),
    )?;
    let f_not_deducted_own_date = r.fig(
        &format!("interest_if_not_deducted_own_date_{t}"),
        Value::Int(not_deducted_own_date),
        Unit::Paise,
        &format!(
            "s.201(1A) interest if the tax is still not deducted at as_of ({as_of}) and each \
credit's tax ran from that credit's own date (row {p} tag {t}): each credit's share of the tax at \
the pre-deduction rate, rounded on its own."
        ),
        ev(),
    )?;
    let f_max_own_date = r.fig(
        &format!("interest_max_own_date_{t}"),
        Value::Int(maximum_own_date),
        Unit::Paise,
        &format!(
            "s.201(1A) interest, maximum bound, if each credit's tax ran from that credit's \
own date rather than from the day the aggregate was crossed (row {p} tag {t}): each credit's share \
of the tax, the largest over every deduction date from the credit's own date to as_of ({as_of}). \
Each credit's share is rounded on its own, so this may differ from the row's maximum where the two \
readings date the credits alike."
        ),
        ev(),
    )?;
    Ok(Priced {
        minimum,
        maximum,
        maximum_own_date,
        facts: vec![
            ("deductible_date", f_deductible),
            ("tax", f_tax),
            ("interest_min", f_min),
            ("interest_max", f_max),
            ("interest_if_not_deducted", f_not_deducted),
            ("interest_max_own_date", f_max_own_date),
            ("interest_if_not_deducted_own_date", f_not_deducted_own_date),
        ],
        limits: row_limits(row, as_of, default_rate),
    })
}

/// Run the test on `rows` ([`assemble_rows`]) at `as_of`, the rules' `[due_dates].audit_report`
/// (ISO). The book is not read.
#[allow(clippy::too_many_lines)] // one section per figure and finding, as the reference has them
pub fn run(rules: &Rules, rows: &[Row], as_of: &str) -> Result<TestResult> {
    let as_of_date = iso_date(as_of, "rules: [due_dates].audit_report")?;
    let (before, after, authority, status) = match &rules.s201_1a {
        Some(t) => (
            t.rate_before_deduction_bp,
            t.rate_after_deduction_bp,
            t.authority.as_str(),
            t.status.as_str(),
        ),
        None => DEFAULT_S201_1A,
    };
    let (rate_206c, authority_206c, status_206c) = match &rules.s206c_7 {
        Some(t) => (t.rate_bp, t.authority.as_str(), t.status.as_str()),
        None => DEFAULT_S206C_7,
    };
    let default_rate = rules.s201_1a.is_none();
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.population_note = POPULATION_NOTE.to_string();
    r.fig(
        "as_of",
        Value::Text(as_of.to_string()),
        Unit::Text,
        "Reference date for every maximum bound: the return due date from rules[due_dates], or the \
report date, as supplied by the caller -- this test never reads rules[due_dates] itself.",
        Vec::new(),
    )?;
    r.fig(
        "rate_before_deduction_bp",
        Value::Int(before),
        Unit::BasisPoints,
        &format!(
            "s.201(1A) pre-deduction rate, per month or part of a month ({authority}, \
status={status})."
        ),
        Vec::new(),
    )?;
    r.fig(
        "rate_after_deduction_bp",
        Value::Int(after),
        Unit::BasisPoints,
        &format!(
            "s.201(1A) post-deduction rate, per month or part of a month ({authority}, \
status={status})."
        ),
        Vec::new(),
    )?;
    r.fig(
        "rate_206c_bp",
        Value::Int(rate_206c),
        Unit::BasisPoints,
        &format!(
            "s.206C(7) rate, per month or part of a month ({authority}, status={status}).",
            authority = authority_206c,
            status = status_206c
        ),
        Vec::new(),
    )?;

    let pricing = Pricing {
        before,
        after,
        as_of: &as_of_date,
    };
    let mut priced = Vec::with_capacity(rows.len());
    for (p, row) in rows.iter().enumerate() {
        priced.push(price_row(&mut r, row, p, &pricing, as_of, default_rate)?);
    }

    // The groups, in the order each first appears.
    let mut groups: Vec<(&str, Vec<usize>)> = Vec::new();
    let mut slot_of: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, row) in rows.iter().enumerate() {
        let slot = *slot_of.entry(row.group.as_str()).or_insert_with(|| {
            groups.push((row.group.as_str(), Vec::new()));
            groups.len() - 1
        });
        groups[slot].1.push(i);
    }
    for (group, members) in &groups {
        if !members.iter().any(|&i| priced[i].maximum > 0) {
            continue;
        }
        let first = &rows[members[0]];
        let section = first.section.as_str();
        let phrase = first.rate_tag.phrase();
        let g = hash8(&format!("{section}:{group}"));
        let group_tax = members
            .iter()
            .try_fold(0_i64, |a, &i| a.checked_add(rows[i].tax))
            .ok_or_else(|| overflow(TEST_ID))?;
        let f_group_tax = r.fig(
            &format!("group_tax_{g}"),
            Value::Int(group_tax),
            Unit::Paise,
            &format!(
                "Tax across every tranche of this payee{phrase} (group {g}): the sum of its \
rows' tax."
            ),
            Vec::new(),
        )?;
        let listed: Vec<usize> = members
            .iter()
            .copied()
            .filter(|&i| rows[i].tax > 0)
            .collect();
        let mut facts = vec![("tax".to_string(), f_group_tax)];
        for (n, &i) in listed.iter().enumerate() {
            for (name, id) in &priced[i].facts {
                facts.push((format!("tranche_{}_{name}", n + 1), id.clone()));
            }
        }
        let mut limits: Vec<String> = Vec::new();
        let mut evidence: Vec<EvidenceRef> = Vec::new();
        for &i in members {
            for l in &priced[i].limits {
                if !limits.contains(l) {
                    limits.push(l.clone());
                }
            }
            for e in &rows[i].evidence {
                if !evidence.contains(e) {
                    evidence.push(e.clone());
                }
            }
        }
        let n = listed.len();
        let s = if n > 1 { "s" } else { "" };
        r.findings.push(Finding {
            id: format!("{TEST_ID}/{g}"),
            clauses: CLAUSES.iter().map(|c| (*c).to_string()).collect(),
            title: format!(
                "Possible s.201(1A) interest on a {section} shortfall for one \
payee{phrase} -- {n} tranche{s} -- range pending deposit evidence"
            ),
            facts,
            evidence,
            confidence: Confidence::NeedsDocument,
            limits,
            ask_client: ASK_CLIENT.iter().map(|a| (*a).to_string()).collect(),
        });
    }

    // The two scenarios: a single-rate row is in both, and they are never added.
    let one_off = rows
        .iter()
        .any(|row| row.foreseeability == Some(Foreseeability::OneOff));
    let (mut lower_minimum, mut higher_maximum, mut higher_one_off) = (0_i64, 0_i64, 0_i64);
    for (sc, higher) in [("lower", false), ("higher", true)] {
        let scenario = format!(
            "the {sc}-rate scenario: every row at a payee's {sc} rate, and \
every row at a single rate -- never a payee's row at the other rate as well"
        );
        let members: Vec<usize> = (0..rows.len())
            .filter(|&i| rows[i].rate_tag.in_scenario(higher))
            .collect();
        let sum = |part: &dyn Fn(usize) -> i64| {
            members
                .iter()
                .try_fold(0_i64, |a, &i| a.checked_add(part(i)))
                .ok_or_else(|| overflow(TEST_ID))
        };
        let total_tax = sum(&|i| rows[i].tax)?;
        let total_minimum = sum(&|i| priced[i].minimum)?;
        let total_maximum = sum(&|i| priced[i].maximum_own_date)?;
        r.fig(
            &format!("total_tax_{sc}_rates_paise"),
            Value::Int(total_tax),
            Unit::Paise,
            &format!("Sum of tax_<row> in {scenario}."),
            Vec::new(),
        )?;
        r.fig(
            &format!("total_interest_min_{sc}_rates_paise"),
            Value::Int(total_minimum),
            Unit::Paise,
            &format!("Sum of interest_min_<row> in {scenario}."),
            Vec::new(),
        )?;
        r.fig(
            &format!("total_interest_max_{sc}_rates_paise"),
            Value::Int(total_maximum),
            Unit::Paise,
            &format!(
                "Sum of each row's maximum with every credit's tax from its own date \
(interest_max_own_date_<row>, or interest_max_<row> where a row has no such figure) in \
{scenario}."
            ),
            Vec::new(),
        )?;
        r.fig(
            &format!("total_interest_26a_relief_{sc}_rates_paise"),
            Value::Int(0),
            Unit::Paise,
            &format!(
                "Sum of interest_26a_relief_<row> in {scenario}, over only the rows where a \
payee filing date was supplied -- a partial sum over a subset, never compared directly to the \
range."
            ),
            Vec::new(),
        )?;
        if one_off {
            let at_crossing = sum(&|i| {
                if rows[i].foreseeability == Some(Foreseeability::OneOff) {
                    priced[i].maximum
                } else {
                    priced[i].maximum_own_date
                }
            })?;
            r.fig(
                &format!("total_interest_max_one_off_at_crossing_{sc}_rates_paise"),
                Value::Int(at_crossing),
                Unit::Paise,
                &format!(
                    "As total_interest_max_{sc}_rates_paise, but with the rows of payees \
classified as one-off run from the day the year's total crossed the limit, in {scenario}."
                ),
                Vec::new(),
            )?;
            if higher {
                higher_one_off = at_crossing;
            }
        }
        if higher {
            higher_maximum = total_maximum;
        } else {
            lower_minimum = total_minimum;
        }
    }
    r.fig(
        "total_interest_min_paise",
        Value::Int(lower_minimum),
        Unit::Paise,
        "The floor of the combined range: the lower-rate scenario's minimum.",
        Vec::new(),
    )?;
    r.fig(
        "total_interest_max_paise",
        Value::Int(higher_maximum),
        Unit::Paise,
        "The ceiling of the combined range: the higher-rate scenario's maximum, each credit's tax \
running from its own date (one reading of s.194C(5) and its proviso, pending the CA's \
confirmation).",
        Vec::new(),
    )?;
    if one_off {
        r.fig(
            "total_interest_max_one_off_at_crossing_paise",
            Value::Int(higher_one_off),
            Unit::Paise,
            "The same ceiling with the payees classified as one-off run from the day the year's \
total crossed the limit: the other end of their range, the CA's to choose.",
            Vec::new(),
        )?;
    }
    let unclassified = rows
        .iter()
        .filter(|row| row.foreseeability.is_none())
        .count();
    r.fig(
        "foreseeability_unclassified_row_count",
        count(TEST_ID, unclassified)?,
        Unit::Count,
        "Rows whose payee's foreseeability is not classified in the client's list of payees: \
priced from each credit's own date and flagged on their finding.",
        Vec::new(),
    )?;
    r.fig(
        "s201_1a_defaults_with_26a_relief_date_count",
        Value::Int(0),
        Unit::Count,
        "Rows (deducted_date unknown, s.201(1A)) where a payee return-filing date was supplied for \
the Form 26A relief scenario.",
        Vec::new(),
    )?;
    r.fig(
        "s201_1a_defaults_without_26a_relief_date_count",
        count(TEST_ID, rows.len())?,
        Unit::Count,
        "Rows (deducted_date unknown, s.201(1A)) where no payee return-filing date was supplied -- \
the Form 26A relief scenario could not be computed.",
        Vec::new(),
    )?;
    Ok(r)
}

// The module check keeps its own date arithmetic, on the figures' ISO texts, so that it does not
// repeat the test's.

type Ymd = (i64, i64, i64);

/// An ISO `YYYY-MM-DD` text as (year, month, day); `None` for any other shape.
fn check_ymd(text: &str) -> Option<Ymd> {
    let b = text.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    let number = |from: usize, to: usize| {
        b[from..to].iter().try_fold(0_i64, |a, c| {
            c.is_ascii_digit().then(|| a * 10 + i64::from(c - b'0'))
        })
    };
    Some((number(0, 4)?, number(5, 7)?, number(8, 10)?))
}

fn check_next_day((y, m, d): Ymd) -> Ymd {
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let last = match m {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if d < last {
        (y, m, d + 1)
    } else if m < 12 {
        (y, m + 1, 1)
    } else {
        (y + 1, 1, 1)
    }
}

fn check_months(start: Ymd, end: Ymd) -> i64 {
    if end <= start {
        0
    } else {
        (end.0 - start.0) * 12 + (end.1 - start.1) + 1
    }
}

fn check_interest(tax: i64, rate_bp: i64, months: i64) -> Option<i64> {
    if tax <= 0 || rate_bp <= 0 || months <= 0 {
        return Some(0);
    }
    let scaled = i128::from(tax)
        .checked_mul(i128::from(rate_bp))?
        .checked_mul(i128::from(months))?
        .checked_add(5000)?;
    i64::try_from(scaled / 10_000).ok()
}

/// The maximum by the reference check's own search: the start date, the day after it when the
/// as-of date is later, and the first day of every later month up to the as-of date's. Each other
/// day counts the same months as one of these, so the search gives the test's maximum.
fn check_maximum(tax: i64, start: Ymd, as_of: Ymd, before: i64, after: i64) -> Option<i64> {
    if start > as_of {
        return Some(0);
    }
    let mut days = vec![start];
    if as_of > start {
        days.push(check_next_day(start));
    }
    let (mut y, mut m) = (start.0, start.1);
    loop {
        (y, m) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
        if (y, m) > (as_of.0, as_of.1) {
            break;
        }
        days.push((y, m, 1));
    }
    days.into_iter().try_fold(0_i64, |best, d| {
        let both = check_interest(tax, before, check_months(start, d))?
            .checked_add(check_interest(tax, after, check_months(d, as_of))?)?;
        Some(best.max(both))
    })
}

fn int_figure(figures: &BTreeMap<&str, &Value>, name: &str) -> Option<i64> {
    match figures.get(name).copied() {
        Some(Value::Int(v)) => Some(*v),
        _ => None,
    }
}

fn text_figure<'v>(figures: &BTreeMap<&str, &'v Value>, name: &str) -> Option<&'v str> {
    match figures.get(name).copied() {
        Some(Value::Text(v)) => Some(v.as_str()),
        _ => None,
    }
}

/// An optional int as Python prints it.
fn py_int(v: Option<i64>) -> String {
    v.map_or_else(|| "None".to_string(), |v| v.to_string())
}

/// The reference module's own check (TDSI-1): every row's bounds recomputed from its published
/// figures alone, with this check's own months and interest rules, in ascending order of tag.
///
/// # Errors
///
/// Refuses a result without its `as_of` date or a rate figure, and an amount that overflows.
pub fn check_invariants(result: &TestResult) -> Result<Vec<String>> {
    let prefix = format!("{TEST_ID}.");
    let figures: BTreeMap<&str, &Value> = result
        .figures
        .iter()
        .filter_map(|f| Some((f.id.strip_prefix(prefix.as_str())?, &f.value)))
        .collect();
    let refuse = |name: &str| {
        AuditError::Config(format!(
            "{TEST_ID}.check_invariants: the result has no {name} figure to recompute from"
        ))
    };
    let as_of = text_figure(&figures, "as_of")
        .and_then(check_ymd)
        .ok_or_else(|| refuse("as_of"))?;
    let before = int_figure(&figures, "rate_before_deduction_bp")
        .ok_or_else(|| refuse("rate_before_deduction_bp"))?;
    let after = int_figure(&figures, "rate_after_deduction_bp")
        .ok_or_else(|| refuse("rate_after_deduction_bp"))?;
    int_figure(&figures, "rate_206c_bp").ok_or_else(|| refuse("rate_206c_bp"))?;
    let mut out = Vec::new();
    for tag in figures
        .keys()
        .filter_map(|n| n.strip_prefix("interest_min_"))
    {
        let name = |what: &str| format!("{what}_{tag}");
        let (Some(tax), Some(_), Some(deductible), Some(min), Some(max)) = (
            int_figure(&figures, &name("tax")),
            text_figure(&figures, &name("section")),
            text_figure(&figures, &name("deductible_date")).and_then(check_ymd),
            int_figure(&figures, &name("interest_min")),
            int_figure(&figures, &name("interest_max")),
        ) else {
            out.push(format!(
                "TDSI-1: {tag}: missing a figure needed for independent recomputation"
            ));
            continue;
        };
        if min < 0 || max < 0 {
            out.push(format!(
                "TDSI-1: {tag}: negative bound (min={min}, max={max})"
            ));
        }
        if min > max {
            out.push(format!("TDSI-1: {tag}: min {min} > max {max}"));
        }
        let deducted = text_figure(&figures, &name("deducted_date")).unwrap_or(NOT_SUPPLIED);
        let expected_not_deducted = if deducted == NOT_SUPPLIED {
            let want_max = check_maximum(tax, deductible, as_of, before, after)
                .ok_or_else(|| overflow(TEST_ID))?;
            if min != 0 {
                out.push(format!(
                    "TDSI-1: {tag}: published interest_min {min} != independently recomputed 0"
                ));
            }
            if max != want_max {
                out.push(format!(
                    "TDSI-1: {tag}: published interest_max {max} != independently recomputed \
{want_max}"
                ));
            }
            Some(
                check_interest(tax, before, check_months(deductible, as_of))
                    .ok_or_else(|| overflow(TEST_ID))?,
            )
        } else {
            None
        };
        let published = int_figure(&figures, &name("interest_if_not_deducted"));
        if published != expected_not_deducted {
            out.push(format!(
                "TDSI-1: {tag}: published interest_if_not_deducted {} != independently recomputed \
{}",
                py_int(published),
                py_int(expected_not_deducted)
            ));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::S2011a;

    fn d(s: &str) -> TallyDate {
        TallyDate::parse(s.to_string()).unwrap()
    }

    fn pricing(as_of: &TallyDate, before: i64, after: i64) -> Pricing<'_> {
        Pricing {
            before,
            after,
            as_of,
        }
    }

    fn row(section: Section, rid: &str, rate_tag: RateTag, day: &str, tax: i64) -> Row {
        let iso_day = iso(&d(day));
        Row {
            section,
            payee_key: format!("{rid}_{}_{iso_day}", rate_tag.as_str()),
            group: format!("{rid}_{}", rate_tag.as_str()),
            tax,
            deductible: d(day),
            flag: false,
            rate_tag,
            rate_bp: 100,
            credits: vec![(d(day), tax)],
            foreseeability: None,
            evidence: Vec::new(),
        }
    }

    fn value<'a>(r: &'a TestResult, name: &str) -> &'a Value {
        let id = format!("{TEST_ID}.{name}");
        &r.figures.iter().find(|f| f.id == id).unwrap().value
    }

    fn has(r: &TestResult, name: &str) -> bool {
        let id = format!("{TEST_ID}.{name}");
        r.figures.iter().any(|f| f.id == id)
    }

    #[test]
    fn months_count_the_calendar_months_a_span_touches() {
        let as_of = d("20260930");
        assert_eq!(months(&d("20260929"), &as_of), 1);
        assert_eq!(months(&d("20260831"), &as_of), 2);
        assert_eq!(months(&as_of, &as_of), 0);
        assert_eq!(months(&d("20261001"), &as_of), 0);
        assert_eq!(months(&d("20251231"), &d("20260101")), 2);
        assert_eq!(months(&d("20251001"), &as_of), 12);
        assert_eq!(months(&d("20250930"), &as_of), 13);
    }

    #[test]
    fn interest_rounds_half_up_and_is_zero_on_each_guard() {
        // 50 * 100 * 1 = 5000: half a paisa rounds up; 4999 rounds down.
        assert_eq!(interest(50, 100, 1).unwrap(), 1);
        assert_eq!(interest(49, 100, 1).unwrap(), 0);
        assert_eq!(interest(2000, 150, 1).unwrap(), 30);
        for (tax, rate, m) in [
            (0, 100, 1),
            (-5, 100, 1),
            (100, 0, 1),
            (100, -1, 1),
            (100, 100, 0),
        ] {
            assert_eq!(interest(tax, rate, m).unwrap(), 0, "{tax} {rate} {m}");
        }
        assert_eq!(tax_at(5, 1000).unwrap(), 1, "0.5 paisa rounds up");
        assert_eq!(tax_at(4, 1000).unwrap(), 0);
    }

    /// The maxima the spec pack works through (`ti_month_edges` m03, m04, m05; `ti_rounding` x11).
    #[test]
    fn the_maximum_can_fall_after_the_start_date() {
        let as_of = d("20260930");
        let p = pricing(&as_of, 100, 150);
        assert_eq!(p.maximum(2000, &d("20260831")).unwrap(), 70);
        assert_eq!(p.maximum(2000, &d("20260901")).unwrap(), 50);
        assert_eq!(p.maximum(2000, &d("20260929")).unwrap(), 30);
        assert_eq!(p.maximum(9, &d("20260122")).unwrap(), 2);
        assert_eq!(
            pricing(&as_of, 200, 300)
                .maximum(9, &d("20260122"))
                .unwrap(),
            3
        );
        assert_eq!(p.maximum(2000, &d("20261001")).unwrap(), 0);
    }

    /// The check's month-candidate search gives the test's day-by-day maximum: leap days, month
    /// and year ends, taxes from 1 paisa, several rate pairs.
    #[test]
    fn the_checks_search_agrees_with_the_day_loop() {
        let as_of_dates = ["20260930", "20240229", "20251231", "20260301"];
        let starts = [
            "20240131", "20240228", "20240229", "20240301", "20241231", "20250101", "20250430",
            "20250815", "20251130", "20251231", "20260131", "20260228", "20260831", "20260901",
            "20260929", "20260930",
        ];
        for as_of in as_of_dates {
            let as_of_date = d(as_of);
            let as_of_ymd = check_ymd(&iso(&as_of_date)).unwrap();
            for (before, after) in [(100, 150), (150, 100), (100, 100), (1, 10_000), (200, 300)] {
                let p = pricing(&as_of_date, before, after);
                for start in starts {
                    let start_ymd = check_ymd(&iso(&d(start))).unwrap();
                    for tax in [1, 9, 33, 2000, 123_457] {
                        assert_eq!(
                            p.maximum(tax, &d(start)).unwrap(),
                            check_maximum(tax, start_ymd, as_of_ymd, before, after).unwrap(),
                            "{start} to {as_of}, tax {tax}, {before}/{after}"
                        );
                    }
                }
            }
        }
    }

    /// The tag and group id of the spec pack's own example: `ti_placeholder`'s first row.
    #[test]
    fn a_tag_hashes_section_key_and_position() {
        let key = "194H_7b02487b_single_rate_2025-07-14";
        let rules = Rules::vendored().unwrap();
        let x = row(
            Section::S194H,
            "194H_7b02487b",
            RateTag::Single,
            "20250714",
            2000,
        );
        assert_eq!(x.payee_key, key);
        let r = run(&rules, &[x], "2026-09-30").unwrap();
        let tag = hash8(&format!("194H:{key}:0"));
        assert!(has(&r, &format!("interest_min_{tag}")));
        let g = hash8("194H:194H_7b02487b_single_rate");
        assert_eq!(r.findings[0].id, format!("{TEST_ID}/{g}"));
    }

    #[test]
    fn a_single_rate_row_is_in_both_scenarios_and_a_two_rate_row_in_one() {
        let rules = Rules::vendored().unwrap();
        let rows = [
            row(Section::S194C, "194C_aa", RateTag::Lower, "20250701", 100),
            row(Section::S194C, "194C_aa", RateTag::Higher, "20250701", 200),
            row(Section::S194H, "194H_bb", RateTag::Single, "20250701", 1000),
        ];
        let r = run(&rules, &rows, "2026-09-30").unwrap();
        assert_eq!(value(&r, "total_tax_lower_rates_paise"), &Value::Int(1100));
        assert_eq!(value(&r, "total_tax_higher_rates_paise"), &Value::Int(1200));
        assert_eq!(
            value(&r, "s201_1a_defaults_without_26a_relief_date_count"),
            &Value::Int(3)
        );
        assert_eq!(
            value(&r, "foreseeability_unclassified_row_count"),
            &Value::Int(3)
        );
        // No one-off row: none of the three one-off figures.
        assert!(!has(&r, "total_interest_max_one_off_at_crossing_paise"));
        assert!(!has(
            &r,
            "total_interest_max_one_off_at_crossing_lower_rates_paise"
        ));
    }

    #[test]
    fn a_one_off_row_takes_its_plain_maximum_in_the_one_off_totals() {
        let rules = Rules::vendored().unwrap();
        let mut x = row(Section::S194C, "194C_aa", RateTag::Higher, "20250701", 2000);
        // The aggregate crossed on 1 July, the credit itself from 1 April: its own date is earlier.
        x.credits = vec![(d("20250401"), 2000)];
        x.foreseeability = Some(Foreseeability::OneOff);
        let r = run(&rules, &[x], "2026-09-30").unwrap();
        let tag = hash8("194C:194C_aa_higher_rate_2025-07-01:0");
        let plain = value(&r, &format!("interest_max_{tag}")).clone();
        let own = value(&r, &format!("interest_max_own_date_{tag}")).clone();
        assert_ne!(plain, own);
        assert_eq!(value(&r, "total_interest_max_higher_rates_paise"), &own);
        assert_eq!(
            value(
                &r,
                "total_interest_max_one_off_at_crossing_higher_rates_paise"
            ),
            &plain
        );
        assert_eq!(
            value(&r, "total_interest_max_one_off_at_crossing_paise"),
            &plain
        );
        assert_eq!(value(&r, "total_interest_max_paise"), &own);
    }

    #[test]
    fn a_title_counts_the_listed_tranches_and_skips_a_tax_of_zero() {
        let rules = Rules::vendored().unwrap();
        let one = run(
            &rules,
            &[row(
                Section::S194H,
                "194H_aa",
                RateTag::Single,
                "20250701",
                1000,
            )],
            "2026-09-30",
        )
        .unwrap();
        assert!(
            one.findings[0].title.contains("-- 1 tranche --"),
            "{}",
            one.findings[0].title
        );
        let rows = [
            row(Section::S194H, "194H_aa", RateTag::Single, "20250701", 1000),
            row(Section::S194H, "194H_aa", RateTag::Single, "20250801", 0),
            row(Section::S194H, "194H_aa", RateTag::Single, "20250901", 1000),
        ];
        let three = run(&rules, &rows, "2026-09-30").unwrap();
        let f = &three.findings[0];
        assert!(f.title.contains("-- 2 tranches --"), "{}", f.title);
        assert!(f.facts.iter().any(|(n, _)| n == "tranche_2_tax"));
        assert!(!f.facts.iter().any(|(n, _)| n == "tranche_3_tax"));
        // The group tax is every row's, the tax-0 row included.
        let g = f.facts.iter().find(|(n, _)| n == "tax").unwrap().1.clone();
        let g = g.strip_prefix(&format!("{TEST_ID}.")).unwrap();
        assert_eq!(value(&three, g), &Value::Int(2000));
    }

    #[test]
    fn a_group_whose_rows_all_start_on_or_after_the_as_of_date_has_no_finding() {
        let rules = Rules::vendored().unwrap();
        let rows = [row(
            Section::S194H,
            "194H_aa",
            RateTag::Single,
            "20260930",
            1000,
        )];
        let r = run(&rules, &rows, "2026-09-30").unwrap();
        assert!(r.findings.is_empty());
        assert_eq!(value(&r, "total_tax_lower_rates_paise"), &Value::Int(1000));
    }

    #[test]
    fn a_rows_limits_come_in_order_with_the_flag_last() {
        let mut x = row(Section::S194C, "194C_aa", RateTag::Lower, "20250701", 1000);
        x.flag = true;
        let limits = row_limits(&x, "2026-09-30", true);
        assert_eq!(limits.len(), 8);
        assert_eq!(limits[0], LIMIT_DATES);
        assert_eq!(limits[1], LIMIT_QUARTERLY);
        assert_eq!(limits[2], LIMIT_DEFAULT_RATE);
        assert_eq!(limits[3].matches("2026-09-30").count(), 4);
        assert_eq!(limits[4], LIMIT_NO_FILING);
        assert_eq!(limits[5], BASIS_194C);
        assert_eq!(limits[6], LIMIT_UNCLASSIFIED);
        assert_eq!(limits[7], LIMIT_FLAG);
        x.flag = false;
        x.foreseeability = Some(Foreseeability::Foreseeable);
        let limits = row_limits(&x, "2026-09-30", false);
        assert_eq!(limits.len(), 5);
        assert_eq!(limits[4], BASIS_194C);
        x.foreseeability = Some(Foreseeability::OneOff);
        assert_eq!(row_limits(&x, "2026-09-30", false)[5], LIMIT_ONE_OFF);
    }

    #[test]
    fn the_default_rates_are_used_and_said_without_s201_1a() {
        let mut rules = Rules::vendored().unwrap();
        rules.s201_1a = None;
        rules.s206c_7 = None;
        let r = run(&rules, &[], "2026-09-30").unwrap();
        assert_eq!(value(&r, "rate_before_deduction_bp"), &Value::Int(100));
        assert_eq!(value(&r, "rate_after_deduction_bp"), &Value::Int(150));
        assert_eq!(value(&r, "rate_206c_bp"), &Value::Int(100));
        let def = &r
            .figures
            .iter()
            .find(|f| f.id == format!("{TEST_ID}.rate_206c_bp"))
            .unwrap()
            .definition;
        assert!(def.ends_with("(s.206C(7), status=confirm)."), "{def}");
        rules.s201_1a = Some(S2011a {
            rate_before_deduction_bp: 7,
            rate_after_deduction_bp: 9,
            authority: "a".to_string(),
            status: "b".to_string(),
        });
        let r = run(&rules, &[], "2026-09-30").unwrap();
        assert_eq!(value(&r, "rate_after_deduction_bp"), &Value::Int(9));
        // No row: the seventeen figures, every total and count 0, and no finding.
        assert_eq!(r.figures.len(), 17);
        assert!(r.findings.is_empty());
    }

    fn payees_result(status: &str, findings: Vec<Finding>) -> TestResult {
        let mut r = TestResult::new(tds_payees::TEST_ID, "1", "r");
        r.fig(
            "deductor_status",
            Value::Text(status.to_string()),
            Unit::Text,
            "d",
            Vec::new(),
        )
        .unwrap();
        r.findings = findings;
        r
    }

    fn finding(id: &str, fact: &str) -> Finding {
        Finding {
            id: id.to_string(),
            clauses: Vec::new(),
            title: String::new(),
            facts: vec![(fact.to_string(), "x".to_string())],
            evidence: vec![
                EvidenceRef::with_label("voucher", "g1", "Journal 1 on 2025-07-01"),
                EvidenceRef::with_label("voucher", "g2", "Journal 2 on 2025-08-01"),
            ],
            confidence: Confidence::NeedsDocument,
            limits: Vec::new(),
            ask_client: Vec::new(),
        }
    }

    fn tranche(day: &str, paise: i64, refs: &[(&str, &str)]) -> RefTranche {
        RefTranche {
            day: d(day),
            paise,
            refs: refs
                .iter()
                .map(|(g, l)| EvidenceRef::with_label("voucher", g, l))
                .collect(),
            credits: vec![(d(day), paise)],
        }
    }

    fn payee_table(rid: &str, entity: &str) -> PayeeTranches {
        [(
            rid.to_string(),
            tds_payees::PayeeRow {
                entity: entity.to_string(),
                tranches: vec![
                    tranche("20250701", 10_000_000, &[("g1", "Journal 1 on 2025-07-01")]),
                    tranche("20250801", 3_333, &[("g2", "Journal 2 on 2025-08-01")]),
                ],
            },
        )]
        .into()
    }

    #[test]
    fn rows_are_assembled_per_tranche_and_rate_payees_first() {
        let rules = Rules::vendored().unwrap();
        let rates = rules.tds_rates.unwrap();
        let payees = payees_result(
            "deductor",
            vec![
                finding("tds_payees/deductor_status", "deductor_status"),
                finding("tds_payees/194C_aa", "credited"),
                finding("tds_payees/194J_unmapped_bb", "unmapped_category_credited"),
            ],
        );
        let mut partners = TestResult::new(partners_40b_194t::TEST_ID, "1", "r");
        partners.findings = vec![finding("partners_40b_194t/s194t/cc", "tds_expected")];
        let partner_table: PartnerTranches = [(
            "cc".to_string(),
            partners_40b_194t::PartnerRow {
                key: "P".to_string(),
                tranches: vec![tranche("20250901", 3_000_000, &[])],
            },
        )]
        .into();
        let foreseeability: BTreeMap<String, Foreseeability> =
            [("Payee A".to_string(), Foreseeability::OneOff)].into();
        let rows = assemble_rows(
            &rules,
            &payees,
            &payee_table("194C_aa", "Payee A"),
            &partners,
            &partner_table,
            true,
            &foreseeability,
        )
        .unwrap();
        let keys: Vec<&str> = rows.iter().map(|r| r.payee_key.as_str()).collect();
        assert_eq!(
            keys,
            [
                "194C_aa_lower_rate_2025-07-01",
                "194C_aa_higher_rate_2025-07-01",
                "194C_aa_lower_rate_2025-08-01",
                "194C_aa_higher_rate_2025-08-01",
                "cc_single_rate_2025-09-01",
            ]
        );
        assert_eq!(rows[0].rate_bp, rates.s194c_individual_huf_bp);
        assert_eq!(rows[1].rate_bp, rates.s194c_other_bp);
        assert_eq!(
            rows[0].tax,
            tax_at(10_000_000, rates.s194c_individual_huf_bp).unwrap()
        );
        // Each tranche's evidence is its own vouchers' among the finding's.
        assert_eq!(rows[0].evidence.len(), 1);
        assert_eq!(rows[0].evidence[0].id, "g1");
        assert_eq!(rows[2].evidence[0].id, "g2");
        assert!(
            rows[0].flag && !rows[4].flag,
            "the flag is never set on a partner's row"
        );
        assert_eq!(rows[0].foreseeability, Some(Foreseeability::OneOff));
        assert_eq!(
            rows[4].foreseeability, None,
            "the partner's key P is not classified"
        );
        assert_eq!(rows[4].rate_bp, rules.s194t.map_or(1000, |t| t.rate_bp));

        // Not a deductor: no payee row, the partner's still.
        let not = payees_result(
            "not_deductor",
            vec![finding("tds_payees/194C_aa", "credited")],
        );
        let rows = assemble_rows(
            &rules,
            &not,
            &payee_table("194C_aa", "Payee A"),
            &partners,
            &partner_table,
            false,
            &foreseeability,
        )
        .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].section, Section::S194T);
    }

    #[test]
    fn s194j_categories_take_their_rates() {
        let rates = Rules::vendored().unwrap().tds_rates.unwrap();
        let rate_list = |cat: &str| payee_rates("194J", cat, &rates).unwrap().1;
        assert_eq!(
            rate_list("professional"),
            [(RateTag::Single, rates.s194j_professional_bp)]
        );
        assert_eq!(
            rate_list("technical"),
            [(RateTag::Single, rates.s194j_technical_bp)]
        );
        for cat in ["royalty", "28va"] {
            assert_eq!(
                rate_list(cat),
                [
                    (RateTag::Lower, rates.s194j_technical_bp),
                    (RateTag::Higher, rates.s194j_professional_bp)
                ]
            );
        }
        assert!(payee_rates("194J", "unmapped", &rates).is_err());
        assert_eq!(
            payee_rates("194I", "", &rates).unwrap().1,
            [
                (RateTag::Lower, rates.s194i_plant_machinery_bp),
                (RateTag::Higher, rates.s194i_land_building_bp)
            ]
        );
    }

    #[test]
    fn assembly_refuses_without_tds_rates_a_missing_entry_and_194h_without_a_rate() {
        let mut rules = Rules::vendored().unwrap();
        let empty = TestResult::new(partners_40b_194t::TEST_ID, "1", "r");
        let none_p = PartnerTranches::new();
        let refusal =
            |rules: &Rules, payees: &TestResult, table: &PayeeTranches| match assemble_rows(
                rules,
                payees,
                table,
                &empty,
                &none_p,
                false,
                &BTreeMap::new(),
            ) {
                Err(AuditError::Config(m)) => m,
                other => panic!("{other:?}"),
            };
        let missing = payees_result("deductor", vec![finding("tds_payees/194C_zz", "credited")]);
        let m = refusal(&rules, &missing, &PayeeTranches::new());
        assert!(m.contains("in neither tranche table"), "{m}");
        let h = payees_result("deductor", vec![finding("tds_payees/194H_aa", "credited")]);
        let mut no_h = rules.tds_rates.unwrap();
        no_h.s194h_bp = None;
        rules.tds_rates = Some(no_h);
        assert!(refusal(&rules, &h, &payee_table("194H_aa", "A")).contains("s194h_bp"));
        rules.tds_rates = None;
        let no_rows = payees_result("deductor", Vec::new());
        assert_eq!(
            refusal(&rules, &no_rows, &PayeeTranches::new()),
            "tds_interest_201 needs rules [tds_rates]"
        );
    }

    fn checked_result() -> (TestResult, String) {
        let rules = Rules::vendored().unwrap();
        let rows = [
            row(Section::S194H, "194H_aa", RateTag::Single, "20250701", 1000),
            row(Section::S194H, "194H_bb", RateTag::Single, "20260122", 9),
        ];
        let r = run(&rules, &rows, "2026-09-30").unwrap();
        let tag = hash8("194H:194H_aa_single_rate_2025-07-01:0");
        (r, tag)
    }

    fn set(r: &mut TestResult, name: &str, v: Value) {
        let id = format!("{TEST_ID}.{name}");
        r.figures.iter_mut().find(|f| f.id == id).unwrap().value = v;
    }

    #[test]
    fn each_tdsi_1_message_fires_on_its_own_tampering() {
        let (r, t) = checked_result();
        assert!(check_invariants(&r).unwrap().is_empty());
        let max = match value(&r, &format!("interest_max_{t}")) {
            Value::Int(v) => *v,
            other => panic!("{other:?}"),
        };
        let tampered = |f: &dyn Fn(&mut TestResult)| {
            let mut x = r.clone();
            f(&mut x);
            check_invariants(&x).unwrap()
        };
        assert_eq!(
            tampered(&|x| x.figures.retain(|f| f.id != format!("{TEST_ID}.tax_{t}"))),
            [format!(
                "TDSI-1: {t}: missing a figure needed for independent recomputation"
            )]
        );
        let v = tampered(&|x| set(x, &format!("interest_min_{t}"), Value::Int(-1)));
        assert!(
            v.contains(&format!("TDSI-1: {t}: negative bound (min=-1, max={max})")),
            "{v:?}"
        );
        let v = tampered(&|x| set(x, &format!("interest_min_{t}"), Value::Int(max + 1)));
        assert!(
            v.contains(&format!("TDSI-1: {t}: min {} > max {max}", max + 1)),
            "{v:?}"
        );
        assert!(v.contains(&format!(
            "TDSI-1: {t}: published interest_min {} != independently recomputed 0",
            max + 1
        )));
        let v = tampered(&|x| set(x, &format!("interest_max_{t}"), Value::Int(max + 1)));
        assert_eq!(
            v,
            [format!(
                "TDSI-1: {t}: published interest_max {} != independently recomputed {max}",
                max + 1
            )]
        );
        let v = tampered(&|x| {
            x.figures
                .retain(|f| f.id != format!("{TEST_ID}.interest_if_not_deducted_{t}"));
        });
        assert_eq!(v.len(), 1);
        assert!(v[0].starts_with(&format!(
            "TDSI-1: {t}: published interest_if_not_deducted None != independently recomputed "
        )));
        // A supplied deducted date: no still-not-deducted amount is expected.
        let v = tampered(&|x| {
            set(
                x,
                &format!("deducted_date_{t}"),
                Value::Text("2026-01-01".into()),
            );
        });
        assert_eq!(v.len(), 1);
        assert!(v[0].ends_with(" != independently recomputed None"), "{v:?}");
    }

    #[test]
    fn an_absent_deducted_date_reads_as_not_supplied_and_tags_come_in_order() {
        let (mut r, t) = checked_result();
        r.figures
            .retain(|f| f.id != format!("{TEST_ID}.deducted_date_{t}"));
        assert!(check_invariants(&r).unwrap().is_empty());
        for f in &mut r.figures {
            if f.id.starts_with(&format!("{TEST_ID}.interest_max_")) && !f.id.contains("own_date") {
                f.value = Value::Int(-1);
            }
        }
        let v = check_invariants(&r).unwrap();
        let tags: Vec<&str> = v
            .iter()
            .filter(|m| m.contains("negative bound"))
            .map(|m| &m["TDSI-1: ".len().."TDSI-1: ".len() + 8])
            .collect();
        assert_eq!(tags.len(), 2);
        assert!(tags[0] < tags[1], "{tags:?}");
        let mut no_as_of = r.clone();
        no_as_of
            .figures
            .retain(|f| f.id != format!("{TEST_ID}.as_of"));
        assert!(matches!(
            check_invariants(&no_as_of),
            Err(AuditError::Config(_))
        ));
    }
}
