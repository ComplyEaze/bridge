//! The HSN/SAC a Sales ledger resolves to before an invoice is posted (#691).
//!
//! Tally stamps a voucher's HSN/SAC when the voucher is saved, from the first
//! of: the sales ledger's own dated row in force on the voucher date, the
//! nearest group carrying one (a sub-group before its parent), the company.
//! A voucher saved while nothing resolves stays uncertain in GSTR-1 even after
//! the master is corrected and Tally's master-change re-check has run; only
//! saving the voucher again repairs it (TALLY_PROTOCOL_REFERENCE section 9.16,
//! the HSN/SAC check before a post). So an invoice whose sales ledger resolves
//! to no code is refused before it is posted.
//!
//! Every answer is read at the edge, and the resolution follows only the
//! shapes and the company settings that were measured: anything else is
//! refused under its own code (`invoice_sales_ledger_hsn_source_not_measured`),
//! never read as "no code" or as a code. Bridge never chooses or suggests a
//! code.

use super::{clean, parse_tree, Node};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The source text of a row that carries its own code.
const SPECIFIED: &str = "Specify Details Here";
/// The source text of a row that defers to the groups and the company.
const INHERITED: &str = "As per Company/Group";
/// The elements the lab's HSN/SAC rows carried, on ledgers, groups and the
/// company alike; any other child marks the row unmeasured.
const ROW_CHILDREN: &[&str] = &[
    "APPLICABLEFROM",
    "HSN",
    "HSNCLASSIFICATIONNAME",
    "HSNCODE",
    "SRCOFHSNDETAILS",
];
/// The groups between a ledger and the top of the chart that were measured:
/// a ledger under a sub-group under `Sales Accounts`. A deeper chain is not
/// followed.
const MAX_GROUP_DEPTH: usize = 2;
/// The company settings the precedence above was measured under
/// (`CMPGSTDETAILS.LIST`): the HSN/SAC source priority and the sections the
/// code applies to.
const MEASURED_PRIORITY: &str = "Ledger";
const MEASURED_SECTIONS: &str = "All Sections";

pub(in crate::agent::agent_import) const UNRESOLVED: &str = "invoice_sales_ledger_hsn_unresolved";
pub(in crate::agent::agent_import) const NOT_MEASURED: &str =
    "invoice_sales_ledger_hsn_source_not_measured";

/// One `HSNDETAILS.LIST` element as Tally sent it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::agent::agent_import) struct HsnRow {
    pub(in crate::agent::agent_import) applicable_from: Option<String>,
    pub(in crate::agent::agent_import) code: Option<String>,
    pub(in crate::agent::agent_import) source: Option<String>,
    pub(in crate::agent::agent_import) classification: Option<String>,
    /// A child outside the measured set came back.
    pub(in crate::agent::agent_import) unmeasured: bool,
    /// No attribute, no child and no text: the placeholder a group with no
    /// code carried in the lab.
    pub(in crate::agent::agent_import) empty: bool,
}

/// The HSN/SAC rows of one master. `None` when the answer carried no
/// `HSNDETAILS.LIST` for it, which no measured answer did.
pub(in crate::agent::agent_import) type HsnRows = Option<Vec<HsnRow>>;

/// One dated `CMPGSTDETAILS.LIST` row of the company, as Tally sent it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::agent::agent_import) struct CompanyGstRow {
    pub(in crate::agent::agent_import) applicable_from: Option<String>,
    /// `HSNSOURCEPRIORITY`.
    pub(in crate::agent::agent_import) priority: Option<String>,
    /// `HSNAPPLICABLESECTION`.
    pub(in crate::agent::agent_import) sections: Option<String>,
    /// `HSNCODELENGTH`: the minimum length of a code, by annual turnover.
    pub(in crate::agent::agent_import) code_length: Option<String>,
}

/// What the company object says of the company-level HSN/SAC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::agent::agent_import) struct CompanyHsn {
    pub(in crate::agent::agent_import) rows: HsnRows,
    pub(in crate::agent::agent_import) settings: Vec<CompanyGstRow>,
    /// `ISGSTCLASSIFON`.
    pub(in crate::agent::agent_import) classification_on: Option<String>,
}

/// Where a resolved code came from.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(in crate::agent::agent_import) enum HsnLevel {
    Ledger,
    Group(String),
    Company,
}

/// One pass of the resolution. A pass that needs the group read says so;
/// the caller makes it and passes again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::agent::agent_import) enum HsnStep {
    Resolved { code: String, level: HsnLevel },
    Refused(&'static str),
    NeedsGroups,
}

/// What one master's rows say on a date.
#[derive(Clone, Debug, PartialEq, Eq)]
enum LevelAnswer {
    Code(String),
    /// The row in force defers to the groups and the company, or (where it
    /// was measured) the master carries only the placeholder.
    NoCode,
    NotMeasured,
}

fn is_date(text: &str) -> bool {
    bridge_tally_core::TallyDate::parse(text).is_ok()
}

/// The one item in force on `as_of` (`YYYYMMDD`): the latest date not after
/// it. `None` when no date is in force, when a date is not a calendar day in
/// `YYYYMMDD` form or when two items share the date in force.
fn in_force<'a, T>(dated: &'a [(Option<&'a str>, T)], as_of: &str) -> Option<&'a T> {
    if dated.iter().any(|(date, _)| !date.is_some_and(is_date)) {
        return None;
    }
    let date = dated
        .iter()
        .filter_map(|(date, _)| *date)
        .filter(|date| *date <= as_of)
        .max()?;
    let mut matching = dated.iter().filter(|(other, _)| *other == Some(date));
    match (matching.next(), matching.next()) {
        (Some((_, item)), None) => Some(item),
        _ => None,
    }
}

/// The row in force on `as_of`; a row dated later is ignored (the lab: a
/// ledger row from 1 Sep was not used on a 7 Aug voucher). `placeholder_measured`
/// says whether a placeholder-only answer was seen for this kind of master.
fn level_answer(rows: &HsnRows, as_of: &str, placeholder_measured: bool) -> LevelAnswer {
    let Some(rows) = rows else {
        return LevelAnswer::NotMeasured;
    };
    if rows.iter().any(|row| row.unmeasured) {
        return LevelAnswer::NotMeasured;
    }
    if rows.iter().all(|row| row.empty) {
        return if placeholder_measured && rows.len() == 1 {
            LevelAnswer::NoCode
        } else {
            LevelAnswer::NotMeasured
        };
    }
    if rows.iter().any(|row| row.empty) {
        return LevelAnswer::NotMeasured;
    }
    let dated = rows
        .iter()
        .map(|row| (row.applicable_from.as_deref(), row))
        .collect::<Vec<_>>();
    let Some(row) = in_force(&dated, as_of) else {
        return LevelAnswer::NotMeasured;
    };
    if row
        .classification
        .as_deref()
        .is_some_and(|name| !name.is_empty())
    {
        return LevelAnswer::NotMeasured;
    }
    let code = row.code.as_deref().unwrap_or_default();
    match row.source.as_deref() {
        Some(SPECIFIED) if !code.is_empty() => LevelAnswer::Code(code.to_string()),
        // Deferring was measured only on a master whose one row defers.
        Some(INHERITED) if code.is_empty() && rows.len() == 1 => LevelAnswer::NoCode,
        _ => LevelAnswer::NotMeasured,
    }
}

/// The company's minimum code length on `as_of`, when the company is set up
/// as measured: GST classification off, and in force a priority of Ledger,
/// all sections, and a length that is a number.
fn measured_minimum_length(company: &CompanyHsn, as_of: &str) -> Option<usize> {
    if company.classification_on.as_deref() != Some("No") {
        return None;
    }
    let dated = company
        .settings
        .iter()
        .map(|row| (row.applicable_from.as_deref(), row))
        .collect::<Vec<_>>();
    let row = in_force(&dated, as_of)?;
    if row.priority.as_deref() != Some(MEASURED_PRIORITY)
        || row.sections.as_deref() != Some(MEASURED_SECTIONS)
    {
        return None;
    }
    row.code_length.as_deref()?.parse().ok()
}

/// One pass for one sales ledger on the invoice date `as_of`. `chain` is the
/// ledger's groups, nearest first, as the compliance listing resolved them to
/// the top (`None` when it did not); `groups` is `None` until the caller has
/// read them. A code shorter than the company's minimum length is not one
/// Tally was seen to accept.
pub(in crate::agent::agent_import) fn resolve_hsn(
    ledger: &Result<HsnRows, &'static str>,
    chain: Option<&[String]>,
    groups: Option<&BTreeMap<String, HsnRows>>,
    company: &CompanyHsn,
    as_of: &str,
) -> HsnStep {
    let Some(minimum) = measured_minimum_length(company, as_of) else {
        return HsnStep::Refused(NOT_MEASURED);
    };
    let found = |code: String, level: HsnLevel| {
        if code.chars().count() < minimum {
            HsnStep::Refused(NOT_MEASURED)
        } else {
            HsnStep::Resolved { code, level }
        }
    };
    let Ok(rows) = ledger else {
        return HsnStep::Refused(NOT_MEASURED);
    };
    match level_answer(rows, as_of, false) {
        LevelAnswer::Code(code) => return found(code, HsnLevel::Ledger),
        LevelAnswer::NotMeasured => return HsnStep::Refused(NOT_MEASURED),
        LevelAnswer::NoCode => {}
    }
    let Some(chain) = chain else {
        return HsnStep::Refused(NOT_MEASURED);
    };
    let Some(groups) = groups else {
        return HsnStep::NeedsGroups;
    };
    for (depth, name) in chain.iter().enumerate() {
        let Some(rows) = groups.get(name).filter(|_| depth < MAX_GROUP_DEPTH) else {
            return HsnStep::Refused(NOT_MEASURED);
        };
        match level_answer(rows, as_of, true) {
            LevelAnswer::Code(code) => return found(code, HsnLevel::Group(name.clone())),
            LevelAnswer::NotMeasured => return HsnStep::Refused(NOT_MEASURED),
            LevelAnswer::NoCode => {}
        }
    }
    // A company row cannot defer to anything above it: only its own code or
    // the placeholder is read. The company object of a company with no code
    // was not captured; the placeholder a group with no code carried is what
    // is expected, and anything else is refused as not measured.
    match level_answer(&company.rows, as_of, true) {
        LevelAnswer::Code(code) => found(code, HsnLevel::Company),
        LevelAnswer::NoCode if company.rows.iter().flatten().all(|row| row.empty) => {
            HsnStep::Refused(UNRESOLVED)
        }
        _ => HsnStep::Refused(NOT_MEASURED),
    }
}

/// A sales ledger's type of supply (`GSTTYPEOFSUPPLY`), for the warnings only.
/// Anything but the two values the lab's sales ledgers carried, an absent or
/// repeated element included, is not judged: it never fails a read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::agent::agent_import) enum SupplyType {
    Goods,
    Services,
    NotJudged,
}

impl SupplyType {
    pub(super) fn of(master: &Node) -> Self {
        match master.text_of("GSTTYPEOFSUPPLY").ok().flatten().as_deref() {
            Some("Goods") => Self::Goods,
            Some("Services") => Self::Services,
            _ => Self::NotJudged,
        }
    }
}

/// What looks unusual about a code that resolved: a warning naming the
/// ledger, never a block. ComplyEaze Bridge ships no list of codes: these are
/// the prefix and length rules only.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(in crate::agent::agent_import) enum HsnWarningCode {
    /// A character other than a digit; no other rule is judged then.
    #[serde(rename = "hsn_warning_not_digits")]
    NotDigits,
    /// Not six digits: six are preferred, whatever minimum Tally takes.
    #[serde(rename = "hsn_warning_not_six_digits")]
    NotSixDigits,
    /// A services ledger whose code does not start 99, the SAC chapter.
    #[serde(rename = "hsn_warning_not_a_sac_for_services")]
    NotASacForServices,
    /// A goods ledger whose code starts 99.
    #[serde(rename = "hsn_warning_sac_for_goods")]
    SacForGoods,
    /// The ledger's type of supply did not read as Goods or Services, so the
    /// two rules above were not judged.
    #[serde(rename = "hsn_warning_type_of_supply_not_judged")]
    TypeOfSupplyNotJudged,
}

impl HsnWarningCode {
    pub(in crate::agent::agent_import) fn as_str(self) -> &'static str {
        match self {
            Self::NotDigits => "hsn_warning_not_digits",
            Self::NotSixDigits => "hsn_warning_not_six_digits",
            Self::NotASacForServices => "hsn_warning_not_a_sac_for_services",
            Self::SacForGoods => "hsn_warning_sac_for_goods",
            Self::TypeOfSupplyNotJudged => "hsn_warning_type_of_supply_not_judged",
        }
    }
}

/// One warning on one sales ledger, as the build saves it: what the approval
/// says and the post re-checks, so a master change that alters a warning
/// between the build and the post is a changed master. The code itself is
/// not kept: a change that leaves every warning as it was changes nothing the
/// person approved.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(in crate::agent::agent_import) struct HsnWarning {
    pub(in crate::agent::agent_import) code: HsnWarningCode,
    pub(in crate::agent::agent_import) ledger: String,
    /// The master the code came from: a company-level code warns on every
    /// sales ledger that defers to it, and is corrected once, there.
    pub(in crate::agent::agent_import) level: HsnLevel,
    /// The code's length in characters.
    pub(in crate::agent::agent_import) characters: usize,
}

/// The warnings on the code `code` that the sales ledger `ledger` resolved to.
pub(in crate::agent::agent_import) fn hsn_warnings(
    ledger: &str,
    code: &str,
    level: &HsnLevel,
    supply: SupplyType,
) -> Vec<HsnWarning> {
    let codes = if !code.bytes().all(|byte| byte.is_ascii_digit()) {
        vec![HsnWarningCode::NotDigits]
    } else {
        let sac = code.starts_with("99");
        [
            (code.len() != 6, HsnWarningCode::NotSixDigits),
            (
                supply == SupplyType::Services && !sac,
                HsnWarningCode::NotASacForServices,
            ),
            (
                supply == SupplyType::Goods && sac,
                HsnWarningCode::SacForGoods,
            ),
            (
                supply == SupplyType::NotJudged,
                HsnWarningCode::TypeOfSupplyNotJudged,
            ),
        ]
        .into_iter()
        .filter_map(|(applies, code)| applies.then_some(code))
        .collect()
    };
    codes
        .into_iter()
        .map(|warning| HsnWarning {
            code: warning,
            ledger: ledger.to_string(),
            level: level.clone(),
            characters: code.chars().count(),
        })
        .collect()
}

/// The `HSNDETAILS.LIST` children of a master, or `None` when there are none.
pub(super) fn hsn_rows(master: &Node) -> Result<HsnRows, &'static str> {
    let mut rows = Vec::new();
    for details in master.all("HSNDETAILS.LIST") {
        rows.push(HsnRow {
            applicable_from: details.text_of("APPLICABLEFROM")?,
            code: details.text_of("HSNCODE")?,
            source: details.text_of("SRCOFHSNDETAILS")?,
            classification: details.text_of("HSNCLASSIFICATIONNAME")?,
            unmeasured: details
                .children
                .iter()
                .any(|child| !ROW_CHILDREN.contains(&child.name.as_str())),
            empty: details.attributes.is_empty()
                && details.children.is_empty()
                && clean(&details.text).is_empty(),
        });
    }
    Ok((!rows.is_empty()).then_some(rows))
}

/// Each group's HSN/SAC rows by name: every row bound to the verified company
/// by GUID and named, no name twice. A listing with no group is a read that
/// went wrong (every company has the reserved groups). A group whose rows
/// cannot be read is kept as having none, which the resolution refuses only
/// when it reaches that group.
pub(in crate::agent::agent_import) fn parse_group_hsn(
    xml: &str,
    company_guid: &str,
) -> Result<BTreeMap<String, HsnRows>, &'static str> {
    let envelope = parse_tree(xml)?;
    let collection = envelope
        .all("BODY")
        .next()
        .and_then(|body| body.all("DATA").next())
        .and_then(|data| data.all("COLLECTION").next())
        .ok_or("invoice_read_collection_absent")?;
    if collection.all("GROUP").next().is_none() {
        return Err("invoice_group_hsn_listing_empty");
    }
    let mut out = BTreeMap::new();
    for group in collection.all("GROUP") {
        let bound = group.text_of("BRIDGECOMPANYGUID")?;
        if !bound.is_some_and(|guid| guid.eq_ignore_ascii_case(company_guid)) {
            return Err("invoice_group_hsn_company_mismatch");
        }
        let name = group
            .attributes
            .get("NAME")
            .cloned()
            .ok_or("invoice_group_hsn_row_unnamed")?;
        if out.insert(name, hsn_rows(group).unwrap_or(None)).is_some() {
            return Err("invoice_group_hsn_name_repeated");
        }
    }
    Ok(out)
}

/// The company object (`TYPE` Object, `SUBTYPE` Company, every field): its
/// one `COMPANY` must carry the verified company's GUID. Only the HSN/SAC
/// rows and settings are kept; the rest of the object (the company's own
/// contact and tax identifiers among it) is dropped with the answer.
pub(in crate::agent::agent_import) fn parse_company_hsn(
    xml: &str,
    company_guid: &str,
) -> Result<CompanyHsn, &'static str> {
    let envelope = parse_tree(xml)?;
    let mut companies = envelope
        .all("BODY")
        .flat_map(|body| body.all("DATA"))
        .flat_map(|data| data.all("TALLYMESSAGE"))
        .flat_map(|message| message.all("COMPANY"));
    let (Some(company), None) = (companies.next(), companies.next()) else {
        return Err("invoice_company_object_absent");
    };
    let guid = company.text_of("GUID")?;
    if !guid.is_some_and(|guid| guid.eq_ignore_ascii_case(company_guid)) {
        return Err("invoice_company_object_mismatch");
    }
    let mut settings = Vec::new();
    for details in company.all("CMPGSTDETAILS.LIST") {
        settings.push(CompanyGstRow {
            applicable_from: details.text_of("APPLICABLEFROM")?,
            priority: details.text_of("HSNSOURCEPRIORITY")?,
            sections: details.text_of("HSNAPPLICABLESECTION")?,
            code_length: details.text_of("HSNCODELENGTH")?,
        });
    }
    Ok(CompanyHsn {
        rows: hsn_rows(company)?,
        settings,
        classification_on: company.text_of("ISGSTCLASSIFON")?,
    })
}

#[cfg(test)]
#[path = "agent_import_invoice_hsn_tests.rs"]
mod tests;
