// SPDX-License-Identifier: Apache-2.0
//! The reference implementation's `tds_payees` test: payees credited under a s.194C
//! (contractors), s.194-I (rent), s.194J (professional/technical fees, royalty, s.28(va)) or s.194H
//! (commission or brokerage) mapped expense ledger, grouped by payee ENTITY and tested against each
//! section's limits, plus the assessee's deductor status and clause 21(b). A line-for-line port;
//! the reference module's docstring is the design record, summarised here.
//!
//! * The client config maps expense ledgers to a nature (`nature_by_ledger`), merges payee ledgers
//!   into entities (`payee_aliases`), and, for 194J only, maps a ledger to its category
//!   (`s194j_category_by_ledger`). No ledger name is interpreted here.
//! * For each books-population, non-Contra voucher with a line on a mapped expense ledger, the
//!   payees are its CREDIT lines on ledgers that are neither mapped expense ledgers nor under
//!   `Duties & Taxes`, except that a voucher also carrying a `Purchase Accounts`/`Sales Accounts`
//!   line puts the mapped line's OWN amount into the goods-invoice bucket, and a credit on a cash or
//!   bank ledger goes to the payee-not-named bucket (never for a section that only reverses).
//! * A row keeps each voucher by its [`VoucherKey`], unique per voucher, so two vouchers sharing a
//!   GUID (blank, or repeated) are two bills (#1243); a citation, and the client's list of
//!   reversals, name a voucher by its GUID.
//! * A reversal (payee Dr, mapped expense Cr) lowers nothing: it is recorded against the payee's
//!   row and named on its finding, with the CA's classification from `[tds_payees.reversals]`.
//! * Each credit is read GROSS of the TDS on its own bill where the payee is the one party the bill
//!   credits (a bill booked net of TDS credits the payee the net), and less its GST only where the
//!   payee's agreement states GST separately (`[tds_payees].gst_separate_by_agreement`).
//! * 194C trips on the largest total within one voucher or on the year's aggregate; 194-I on any
//!   calendar month; 194J on each category's aggregate separately; 194H on the aggregate. A payee
//!   under a limit only because a Duties & Taxes credit on its bills may be netted TDS is "possibly
//!   over"; one over only with TDS it shares with another party on a bill is listed in full.
//! * Clause 21(b): a non-deductor lists nothing. With no TDS seen on the vouchers touching a payee,
//!   every payment that attracts TDS ([`crate::tds_tranches`]) is a 21(b)(ii)(A) row. With TDS seen,
//!   every payment is listed and one question asks the CA which it covers; a payment whose own bill
//!   carries TDS in a month the recorded challans show nothing deposited for by the due date is a
//!   21(b)(ii)(B) row. A plain bill (only the section's expense, the payee and its TDS) is
//!   rate-tested on its value before TDS: under the section's rate it is a short-deduction finding;
//!   at or above the lower rate its own voucher deducted the tax, so it is not a 21(b)(ii)(A) row
//!   but is listed on the TDS question for its deposit, unless a voucher touching the payee debits
//!   a TDS ledger (a reversal is not judged). A bill carrying TDS that is not rate-tested is counted.
//! * An individual/HUF deducts only on a supplied previous-year turnover over the business limit,
//!   or receipts over the profession limit, as `[deductor].activity` says; otherwise the status is
//!   `unknown`, never assumed.
//!
//! Divergences from the reference:
//! * a malformed `[tds]`, `[tds_payees]`, `[tds.challans]`, `[tds.form_26a]`, `[deductor].activity`
//!   or `[client].state` is refused when the engagement is read ([`crate::TdsConfig`]), which
//!   refuses every test on that engagement; the reference refuses its whole pack there too, since
//!   the pack reads them unconditionally, but a malformed `[tds]` value of some other kinds it
//!   takes by truthiness (a non-string nature) or compares (a float turnover);
//! * a challan month written with non-ASCII digits is refused, where the reference's `isdigit`
//!   admits them and the month then matches no deduction;
//! * with more than one `[tds_payees.reversals]` entry naming a bill credited to no payee, the
//!   first in key order is named, where the reference names the first in file order;
//! * a `[tds].previous_year_turnover_paise` that is a TOML boolean is refused, where the reference
//!   reads `true` as 1 and `false` as 0;
//! * without `[roles].tax_ledgers` no ledger is read as GST (the module's own default), where the
//!   reference's pack refuses its whole run;
//! * with a 194H mapping and no `[s194h]` rules as well as a goods-carriage ledger not mapped to
//!   194C, the 194H refusal is reported, where the reference's config reader reports the other;
//! * `[tds_payees]` without `[tds]` is neither read nor bound here;
//! * a missing `[client].entity_type` is refused here; the reference's engagement always has one;
//! * two over-limit entities sharing a figure id are refused, as the reference raises;
//! * a total that overflows i64 paise is refused, where Python's integers are unbounded.

use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_primitives::TallyDate;

use crate::book::{voucher_keys, Book, Voucher, VoucherKey};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::ledger_ids::stable_ledger_tag;
use crate::rules::Rules;
use crate::support::{count, hash8, overflow, py_lower, py_strip, rupees, voucher_label};
use crate::tds_tranches::crossing_tranches;
use crate::TdsConfig;

pub const TEST_ID: &str = "tds_payees";
pub const VERSION: &str = "1";

// Tally's own reserved/standard group names (not client data), matched through the whole chain.
const CASH_GROUP: &str = "Cash-in-Hand";
const BANK_GROUPS: [&str; 2] = ["Bank Accounts", "Bank OD A/c"];
const CASH_BANK: [&str; 3] = [CASH_GROUP, BANK_GROUPS[0], BANK_GROUPS[1]];
const DUTIES_TAXES_GROUP: &str = "Duties & Taxes";
const PURCHASE_ACCOUNTS_GROUP: &str = "Purchase Accounts";
const SALES_ACCOUNTS_GROUP: &str = "Sales Accounts";

pub const WITHIN_GOODS_INVOICE: &str = "(within supplier goods invoices)";
pub const PAYEE_NOT_NAMED: &str = "(payee not named)";

const NATURES: [&str; 4] = ["194C", "194I", "194J", "194H"];

/// s.194J first proviso, clause (B): four categories, each with its own limit.
const CATEGORIES_194J: [&str; 4] = ["professional", "technical", "royalty", "28va"];
/// A 194J-mapped ledger with no entry, or an unrecognised one, in `s194j_category_by_ledger`.
pub const CATEGORY_UNMAPPED: &str = "unmapped";

/// The reference's `DEFAULT_S194J["aggregate_paise"]`, used only when the rules carry no
/// `[s194j]` table.
const DEFAULT_S194J_AGGREGATE_PAISE: i64 = 5_000_000;

/// The sections a TDS challan may pay, as the reference's `CHALLAN_SECTIONS`.
pub const CHALLAN_SECTIONS: [&str; 4] = ["194C", "194I", "194J", "194H"];

/// The CA's classification of a voucher that reverses a credit to a payee
/// (`[tds_payees.reversals]`). None lowers the credits; a bill-specific or duplicate/error one is
/// named with its bill and not applied until the read carries bill-wise detail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reversal {
    CreditNote,
    /// Names the bill (voucher GUID) it reverses.
    BillSpecific(String),
    /// Names the bill (voucher GUID) it duplicates or corrects.
    DuplicateOrError(String),
}

/// One `[[tds.challans]]` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Challan {
    /// The day the challan paid the tax.
    pub date: TallyDate,
    /// One of [`CHALLAN_SECTIONS`].
    pub section: String,
    /// "YYYY-MM": the month of deduction the challan pays.
    pub month: String,
    /// Positive.
    pub amount_paise: i64,
}

/// `[deductor].activity`: what an individual/HUF client carries on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeductorActivity {
    Business,
    Profession,
    Both,
}

impl DeductorActivity {
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "business" => Some(Self::Business),
            "profession" => Some(Self::Profession),
            "both" => Some(Self::Both),
            _ => None,
        }
    }
    fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Profession => "profession",
            Self::Both => "both",
        }
    }
}

/// What `tds_payees` reads from tables other than `[tds]`/`[tds_payees]`, bound, as the
/// reference's `pack._tds_payees` passes it.
#[derive(Debug, Clone, Default)]
pub struct Inputs {
    /// `[statutory_dues].nature_by_ledger`'s ledgers classified `tds_payable`.
    pub tds_ledgers: BTreeSet<String>,
    /// `[roles].tax_ledgers`' ledgers (every head); empty when the table is absent.
    pub gst_ledgers: BTreeSet<String>,
    /// `[partners]`' keys: names a foreseeability entry may use besides a payee.
    pub other_names: BTreeSet<String>,
    /// `[client].state`.
    pub client_state: Option<String>,
    /// `[deductor].activity`.
    pub deductor_activity: Option<DeductorActivity>,
}

const REVERSAL_KINDS: &str = "('bill_specific', 'duplicate_or_error', 'credit_note')";

/// The `[tds]`/`[tds_payees]` lists the reference's config readers give `tds_payees`
/// (`tds_payees_reversals`, `tds_payees_gst_separate`, `tds_payees_foreseeability`, `tds_challans`,
/// `tds_form_26a`, `turnover_is_placeholder`), each refused on a malformed value as they refuse it.
pub struct ConfigLists {
    pub reversals: BTreeMap<String, Reversal>,
    pub gst_separate: BTreeSet<String>,
    pub foreseeability_names: BTreeSet<String>,
    pub challans: Option<Vec<Challan>>,
    pub form_26a: BTreeMap<String, bool>,
    pub turnover_is_placeholder: bool,
}

fn config_error(message: String) -> AuditError {
    AuditError::Config(message)
}

/// Read the lists from `[tds]` and the optional `[tds_payees]`.
pub fn read_config_lists(
    tds: &toml::Table,
    tds_payees: Option<&toml::Table>,
) -> Result<ConfigLists> {
    use crate::support::py_repr_str;
    let sub_table = |key: &str| -> Result<Option<&toml::Table>> {
        tds_payees
            .and_then(|t| t.get(key))
            .map(|v| {
                v.as_table()
                    .ok_or_else(|| config_error(format!("[tds_payees.{key}] is not a table")))
            })
            .transpose()
    };
    let mut reversals = BTreeMap::new();
    for (guid, entry) in sub_table("reversals")?.into_iter().flatten() {
        let entry = entry.as_table();
        let kind = entry
            .and_then(|t| t.get("kind"))
            .and_then(toml::Value::as_str);
        let bill = entry
            .and_then(|t| t.get("bill"))
            .and_then(toml::Value::as_str);
        let need_bill = |kind: &str| {
            config_error(format!(
                "[tds_payees.reversals] {}: kind {} needs the bill's voucher GUID",
                py_repr_str(guid),
                py_repr_str(kind)
            ))
        };
        let reversal = match kind {
            Some("credit_note") => Reversal::CreditNote,
            Some(k @ "bill_specific") => {
                Reversal::BillSpecific(bill.ok_or_else(|| need_bill(k))?.to_string())
            }
            Some(k @ "duplicate_or_error") => {
                Reversal::DuplicateOrError(bill.ok_or_else(|| need_bill(k))?.to_string())
            }
            _ => {
                return Err(config_error(format!(
                    "[tds_payees.reversals] {}: kind must be one of {REVERSAL_KINDS}",
                    py_repr_str(guid)
                )))
            }
        };
        reversals.insert(guid.clone(), reversal);
    }
    let gst_separate = match tds_payees.and_then(|t| t.get("gst_separate_by_agreement")) {
        None => BTreeSet::new(),
        Some(v) => v
            .as_array()
            .and_then(|a| {
                a.iter()
                    .map(|n| n.as_str().map(str::to_string))
                    .collect::<Option<BTreeSet<_>>>()
            })
            .ok_or_else(|| {
                config_error(
                    "[tds_payees].gst_separate_by_agreement must be a list of payee names"
                        .to_string(),
                )
            })?,
    };
    let mut foreseeability_names = BTreeSet::new();
    for (name, mode) in sub_table("foreseeability")?.into_iter().flatten() {
        if !matches!(mode.as_str(), Some("foreseeable" | "one_off")) {
            return Err(config_error(format!(
                "[tds_payees.foreseeability] {}: must be one of ('foreseeable', 'one_off')",
                py_repr_str(name)
            )));
        }
        foreseeability_names.insert(name.clone());
    }
    let challans = match tds.get("challans") {
        None => None,
        Some(raw) => {
            let raw = raw.as_array().ok_or_else(|| {
                config_error("[[tds.challans]] must be an array of tables".to_string())
            })?;
            let mut out = Vec::new();
            for (i, c) in raw.iter().enumerate() {
                let c = c.as_table().ok_or_else(|| {
                    config_error(format!("[[tds.challans]] entry {i}: must be a table"))
                })?;
                let date = match c.get("date") {
                    Some(toml::Value::Datetime(d)) if d.time.is_none() && d.offset.is_none() => {
                        d.date.and_then(|d| {
                            TallyDate::parse(format!("{:04}{:02}{:02}", d.year, d.month, d.day))
                                .ok()
                        })
                    }
                    _ => None,
                }
                .ok_or_else(|| {
                    config_error(format!(
                        "[[tds.challans]] entry {i}: date must be a TOML date"
                    ))
                })?;
                let section = c
                    .get("section")
                    .and_then(toml::Value::as_str)
                    .filter(|s| CHALLAN_SECTIONS.contains(s))
                    .ok_or_else(|| {
                        config_error(format!(
                            "[[tds.challans]] entry {i}: section must be one of ('194C', '194I', \
'194J', '194H')"
                        ))
                    })?;
                // "YYYY-MM", month 1..=12. ASCII digits only: the reference's `isdigit` would admit
                // other digits, and the month would then match no deduction (a divergence).
                let month = c
                    .get("month")
                    .and_then(toml::Value::as_str)
                    .filter(|m| {
                        let b = m.as_bytes();
                        b.len() == 7
                            && b[4] == b'-'
                            && b[..4].iter().all(u8::is_ascii_digit)
                            && b[5..].iter().all(u8::is_ascii_digit)
                            && (1..=12).contains(&m[5..].parse::<u32>().unwrap_or(0))
                    })
                    .ok_or_else(|| {
                        config_error(format!(
                            "[[tds.challans]] entry {i}: month must be \"YYYY-MM\""
                        ))
                    })?;
                let amount_paise = c
                    .get("amount_paise")
                    .and_then(toml::Value::as_integer)
                    .filter(|a| *a > 0)
                    .ok_or_else(|| {
                        config_error(format!(
                            "[[tds.challans]] entry {i}: amount_paise must be a positive integer"
                        ))
                    })?;
                out.push(Challan {
                    date,
                    section: section.to_string(),
                    month: month.to_string(),
                    amount_paise,
                });
            }
            Some(out)
        }
    };
    let mut form_26a = BTreeMap::new();
    if let Some(raw) = tds.get("form_26a") {
        let raw = raw.as_array().ok_or_else(|| {
            config_error("[[tds.form_26a]] must be an array of tables".to_string())
        })?;
        for (i, e) in raw.iter().enumerate() {
            let t = e.as_table();
            let payee = t
                .and_then(|t| t.get("payee"))
                .and_then(toml::Value::as_str)
                .filter(|p| !p.is_empty());
            let held = t.and_then(|t| t.get("held")).and_then(toml::Value::as_bool);
            let (Some(payee), Some(held)) = (payee, held) else {
                return Err(config_error(format!(
                    "[[tds.form_26a]] entry {i}: needs payee (a name) and held (true or false)"
                )));
            };
            if form_26a.insert(payee.to_string(), held).is_some() {
                return Err(config_error(format!(
                    "[[tds.form_26a]] entry {i}: {} is listed twice",
                    py_repr_str(payee)
                )));
            }
        }
    }
    let turnover_is_placeholder = match tds.get("previous_year_turnover_status") {
        None => false,
        Some(v) => match v.as_str() {
            Some("placeholder") => true,
            Some("confirmed") => false,
            _ => {
                return Err(config_error(
                    "[tds].previous_year_turnover_status must be one of ('placeholder', \
'confirmed')"
                        .to_string(),
                ))
            }
        },
    };
    Ok(ConfigLists {
        reversals,
        gst_separate,
        foreseeability_names,
        challans,
        form_26a,
        turnover_is_placeholder,
    })
}

/// `[deductor].activity`, the reference's `deductor_activity`: `None` when absent; any other value
/// than "business", "profession" or "both" is refused.
pub fn read_deductor_activity(cfg: &toml::Table) -> Result<Option<DeductorActivity>> {
    let Some(deductor) = cfg.get("deductor") else {
        return Ok(None);
    };
    let deductor = deductor
        .as_table()
        .ok_or_else(|| config_error("[deductor] is not a table".to_string()))?;
    deductor
        .get("activity")
        .map(|v| {
            v.as_str().and_then(DeductorActivity::parse).ok_or_else(|| {
                config_error(
                    "[deductor].activity must be one of ('business', 'profession', 'both')"
                        .to_string(),
                )
            })
        })
        .transpose()
}

/// `[client].state`, the reference's `client_state`: `None` when absent; a value that is not a
/// name (not a string, or blank) is refused.
pub fn read_client_state(client: &toml::Table) -> Result<Option<String>> {
    client
        .get("state")
        .map(|v| {
            v.as_str()
                .filter(|s| !py_strip(s).is_empty())
                .map(str::to_string)
                .ok_or_else(|| config_error("[client].state must be the state's name".to_string()))
        })
        .transpose()
}

fn month_key(v: &Voucher) -> String {
    let s = v.date.as_str();
    format!("{}-{}", &s[0..4], &s[4..6])
}

fn under_any(book: &Book, ledger: &str, groups: &[&str]) -> bool {
    book.ledgers
        .get(ledger)
        .is_some_and(|l| groups.iter().any(|g| l.under(g)))
}

fn alias<'a>(cfg: &'a TdsConfig, ledger: &'a str) -> &'a str {
    cfg.payee_aliases.get(ledger).map_or(ledger, String::as_str)
}

fn sum(values: impl IntoIterator<Item = i64>) -> Result<i64> {
    values
        .into_iter()
        .try_fold(0_i64, i64::checked_add)
        .ok_or_else(|| overflow(TEST_ID))
}

/// `(nature, subcat)`: subcat is empty for 194C/194I/194H and, for 194J, the ledger's own category
/// or [`CATEGORY_UNMAPPED`]. Decided per LEDGER, so one voucher with two 194J ledgers of different
/// categories lands in two rows.
fn nature_key(nature: &str, ledger: &str, cfg: &TdsConfig) -> (String, String) {
    if nature != "194J" {
        return (nature.to_string(), String::new());
    }
    let cat = match cfg.s194j_category_by_ledger.get(ledger) {
        Some(Some(cat)) if CATEGORIES_194J.contains(&cat.as_str()) => cat.clone(),
        _ => CATEGORY_UNMAPPED.to_string(),
    };
    (nature.to_string(), cat)
}

/// One (nature key, entity) row: what each voucher credited, the vouchers, and the reversals
/// recorded against it (the voucher and the payee's debit on it), each by the voucher's own key.
#[derive(Default, Clone)]
struct Row<'a> {
    by_voucher: BTreeMap<&'a VoucherKey, i64>,
    vouchers: BTreeMap<&'a VoucherKey, &'a Voucher>,
    reversals: BTreeMap<&'a VoucherKey, (&'a Voucher, i64)>,
}

impl<'a> Row<'a> {
    fn add(&mut self, k: &'a VoucherKey, v: &'a Voucher, amount: i64) -> Result<()> {
        let slot = self.by_voucher.entry(k).or_insert(0);
        *slot = slot.checked_add(amount).ok_or_else(|| overflow(TEST_ID))?;
        self.vouchers.insert(k, v);
        Ok(())
    }

    /// The GUIDs of the vouchers credited on this row: the client's list names a bill by GUID.
    fn credited_guids(&self) -> BTreeSet<&'a str> {
        self.by_voucher
            .keys()
            .map(|k| self.vouchers[k].guid.as_str())
            .collect()
    }
}

type NatureKey = (String, String);
type RowKey = (NatureKey, String);

/// The reference's `compute_payee_rows`, over the population with its voucher keys.
fn compute_payee_rows<'a>(
    pop: &'a [(VoucherKey, &'a Voucher)],
    book: &Book,
    cfg: &TdsConfig,
) -> Result<BTreeMap<RowKey, Row<'a>>> {
    let mut rows: BTreeMap<RowKey, Row<'a>> = BTreeMap::new();
    let mut reversals: BTreeMap<RowKey, BTreeMap<&'a VoucherKey, (&'a Voucher, i64)>> =
        BTreeMap::new();
    for (k, v) in pop.iter().map(|(k, v)| (k, *v)) {
        if v.base_type == "Contra" {
            continue;
        }
        let mut expense_lines: BTreeMap<NatureKey, Vec<i64>> = BTreeMap::new();
        let mut expense_ledgers_here: BTreeSet<&str> = BTreeSet::new();
        for l in &v.lines {
            // The reference maps a line only on a truthy nature: an empty one maps nothing.
            if let Some(nature) = cfg
                .nature_by_ledger
                .get(&l.ledger)
                .filter(|n| !n.is_empty())
            {
                expense_lines
                    .entry(nature_key(nature, &l.ledger, cfg))
                    .or_default()
                    .push(l.amount_paise);
                expense_ledgers_here.insert(l.ledger.as_str());
            }
        }
        if expense_lines.is_empty() {
            continue;
        }
        let has_goods_line = v.lines.iter().any(|l| {
            under_any(
                book,
                &l.ledger,
                &[PURCHASE_ACCOUNTS_GROUP, SALES_ACCOUNTS_GROUP],
            )
        });
        if has_goods_line {
            // The charge itself (the mapped line), never the supplier's full invoice credit.
            for (key, amounts) in &expense_lines {
                rows.entry((key.clone(), WITHIN_GOODS_INVOICE.to_string()))
                    .or_default()
                    .add(k, v, sum(amounts.iter().copied())?)?;
            }
            continue;
        }
        // A reversal (payee Dr, mapped expense Cr), decided per section: recorded against the
        // payee's row, never a credit, lowering nothing.
        let mut reversing: BTreeSet<&NatureKey> = BTreeSet::new();
        let mut pure_reversal: BTreeSet<&NatureKey> = BTreeSet::new();
        for (key, amounts) in &expense_lines {
            if sum(amounts.iter().copied())? < 0 {
                reversing.insert(key);
            }
            if amounts.iter().all(|a| *a < 0) {
                pure_reversal.insert(key);
            }
        }
        for key in &reversing {
            for l in &v.lines {
                if l.amount_paise > 0
                    && !expense_ledgers_here.contains(l.ledger.as_str())
                    && !under_any(
                        book,
                        &l.ledger,
                        &[
                            DUTIES_TAXES_GROUP,
                            CASH_GROUP,
                            BANK_GROUPS[0],
                            BANK_GROUPS[1],
                        ],
                    )
                {
                    let rv = reversals
                        .entry(((*key).clone(), alias(cfg, &l.ledger).to_string()))
                        .or_default();
                    let prior = rv.get(k).map_or(0, |(_, a)| *a);
                    let total = prior
                        .checked_add(l.amount_paise)
                        .ok_or_else(|| overflow(TEST_ID))?;
                    rv.insert(k, (v, total));
                }
            }
        }
        let credit_lines: Vec<_> = v
            .lines
            .iter()
            .filter(|l| {
                l.amount_paise < 0
                    && !expense_ledgers_here.contains(l.ledger.as_str())
                    && !under_any(book, &l.ledger, &[DUTIES_TAXES_GROUP])
            })
            .collect();
        if credit_lines.is_empty() {
            continue;
        }
        // A voucher whose mapped lines span more than one nature key attributes the full credit
        // total to EACH one present.
        for key in expense_lines.keys() {
            for l in &credit_lines {
                let amount = l
                    .amount_paise
                    .checked_neg()
                    .ok_or_else(|| overflow(TEST_ID))?;
                let entity = if under_any(book, &l.ledger, &CASH_BANK) {
                    // A section that only reverses: a cash or bank credit is its payment, never a
                    // "payee not named" credit.
                    if pure_reversal.contains(key) {
                        continue;
                    }
                    PAYEE_NOT_NAMED.to_string()
                } else {
                    alias(cfg, &l.ledger).to_string()
                };
                rows.entry((key.clone(), entity))
                    .or_default()
                    .add(k, v, amount)?;
            }
        }
    }
    for (key, rv) in reversals {
        if let Some(row) = rows.get_mut(&key) {
            row.reversals = rv;
        }
    }
    Ok(rows)
}

/// One payee row's reversals against the CA's classifications (the reference's `read_reversals`):
/// none lowers the credits. A classification counts only on the row whose credits hold its bill.
struct ReadReversals<'a> {
    unclassified: Vec<&'a Voucher>,
    credit_notes: Vec<&'a Voucher>,
    /// (reversal voucher, "bill_specific" | "duplicate_or_error", bill GUID)
    not_applied: Vec<(&'a Voucher, &'static str, String)>,
}

fn read_reversals<'a>(row: &Row<'a>, cfg: &TdsConfig) -> ReadReversals<'a> {
    let mut out = ReadReversals {
        unclassified: Vec::new(),
        credit_notes: Vec::new(),
        not_applied: Vec::new(),
    };
    // The client's list names a voucher by its GUID: it reads on every voucher holding it.
    let credited = row.credited_guids();
    for (v, _) in row.reversals.values() {
        let v: &'a Voucher = v;
        match cfg.reversals.get(&v.guid) {
            Some(Reversal::CreditNote) => out.credit_notes.push(v),
            Some(Reversal::BillSpecific(bill)) if credited.contains(bill.as_str()) => {
                out.not_applied.push((v, "bill_specific", bill.clone()));
            }
            Some(Reversal::DuplicateOrError(bill)) if credited.contains(bill.as_str()) => {
                out.not_applied
                    .push((v, "duplicate_or_error", bill.clone()));
            }
            _ => out.unclassified.push(v),
        }
    }
    out
}

/// The reference's `Adjusted`: one row as the threshold test and the tranches read it.
struct Adjusted<'a> {
    row: Row<'a>,
    reversals: ReadReversals<'a>,
    gst_excluded: i64,
    gst_counted: i64,
    tds_grossed_up: i64,
    possibly_netted: i64,
    netted_by_voucher: BTreeMap<&'a VoucherKey, i64>,
    shared_tds: i64,
    shared_by_voucher: BTreeMap<&'a VoucherKey, i64>,
    also_debited: i64,
}

/// The parties a voucher credits (GST, TDS and Duties & Taxes lines aside), as payee entities.
fn credited_parties<'a>(
    v: &'a Voucher,
    cfg: &'a TdsConfig,
    inputs: &Inputs,
    book: &Book,
) -> BTreeSet<&'a str> {
    v.lines
        .iter()
        .filter(|l| {
            l.amount_paise < 0
                && !inputs.gst_ledgers.contains(&l.ledger)
                && !inputs.tds_ledgers.contains(&l.ledger)
                && !under_any(book, &l.ledger, &[DUTIES_TAXES_GROUP])
        })
        .map(|l| alias(cfg, &l.ledger))
        .collect()
}

/// The TDS a voucher deducts: its credits to the ledgers classified as TDS payable.
fn tds_on(v: &Voucher, tds_ledgers: &BTreeSet<String>) -> Result<i64> {
    sum(v
        .lines
        .iter()
        .filter(|l| tds_ledgers.contains(&l.ledger))
        .map(|l| l.amount_paise))?
    .checked_neg()
    .ok_or_else(|| overflow(TEST_ID))
}

fn has_tds_line(v: &Voucher, tds_ledgers: &BTreeSet<String>) -> bool {
    v.lines
        .iter()
        .any(|l| tds_ledgers.contains(&l.ledger) && l.amount_paise != 0)
}

/// The reference's `adjusted_row`.
fn adjusted_row<'a>(
    d: &Row<'a>,
    entity: &str,
    cfg: &'a TdsConfig,
    inputs: &Inputs,
    book: &Book,
) -> Result<Adjusted<'a>> {
    let separate = cfg.gst_separate.contains(entity);
    let mut by_voucher = d.by_voucher.clone();
    let (mut excluded, mut counted, mut grossed, mut possibly_netted) =
        (0_i64, 0_i64, 0_i64, 0_i64);
    let (mut shared, mut also_debited) = (0_i64, 0_i64);
    let mut netted_by_voucher = BTreeMap::new();
    let mut shared_by_voucher = BTreeMap::new();
    let add = |a: i64, b: i64| a.checked_add(b).ok_or_else(|| overflow(TEST_ID));
    for (g, credit) in &d.by_voucher {
        let v = d.vouchers[g];
        let mut credit = *credit;
        let mut parties = credited_parties(v, cfg, inputs, book);
        if entity == PAYEE_NOT_NAMED {
            // A payment with no named payee credits a cash or bank ledger: its own line.
            parties.retain(|p| !under_any(book, p, &CASH_BANK));
        }
        let alone = parties.iter().all(|p| *p == entity);
        let tds = tds_on(v, &inputs.tds_ledgers)?;
        let debited = sum(v
            .lines
            .iter()
            .filter(|l| l.amount_paise > 0 && alias(cfg, &l.ledger) == entity)
            .map(|l| l.amount_paise))?;
        if tds > 0 && alone {
            credit = add(credit, tds)?;
            grossed = add(grossed, tds)?;
            if debited != 0 {
                also_debited += 1;
            }
        } else if tds > 0 {
            shared = add(shared, tds)?;
            shared_by_voucher.insert(*g, tds);
        } else if alone {
            let netted = sum(v
                .lines
                .iter()
                .filter(|l| {
                    l.amount_paise < 0
                        && !inputs.tds_ledgers.contains(&l.ledger)
                        && !inputs.gst_ledgers.contains(&l.ledger)
                        && under_any(book, &l.ledger, &[DUTIES_TAXES_GROUP])
                })
                .map(|l| l.amount_paise))?
            .checked_neg()
            .ok_or_else(|| overflow(TEST_ID))?;
            if netted != 0 {
                possibly_netted = add(possibly_netted, netted)?;
                netted_by_voucher.insert(*g, netted);
            }
        }
        // The voucher's NET GST: input GST less any reverse-charge payable credited on it.
        let gst = sum(v
            .lines
            .iter()
            .filter(|l| inputs.gst_ledgers.contains(&l.ledger))
            .map(|l| l.amount_paise))?
        .max(0);
        if gst != 0 && separate && alone {
            let cut = gst.min(credit);
            credit -= cut;
            excluded = add(excluded, cut)?;
        } else if gst != 0 {
            counted = add(counted, gst)?;
        }
        by_voucher.insert(*g, credit);
    }
    by_voucher.retain(|_, p| *p > 0);
    let vouchers = d
        .vouchers
        .iter()
        .filter(|(g, _)| by_voucher.contains_key(*g))
        .map(|(g, v)| (*g, *v))
        .collect();
    Ok(Adjusted {
        row: Row {
            by_voucher,
            vouchers,
            reversals: d.reversals.clone(),
        },
        reversals: read_reversals(d, cfg),
        gst_excluded: excluded,
        gst_counted: counted,
        tds_grossed_up: grossed,
        possibly_netted,
        netted_by_voucher,
        shared_tds: shared,
        shared_by_voucher,
        also_debited,
    })
}

/// The reference's `summarise_row`.
struct Summary<'a> {
    credited: i64,
    max_single: i64,
    months: BTreeMap<String, i64>,
    vouchers: BTreeMap<&'a VoucherKey, &'a Voucher>,
}

fn summarise<'a>(row: &Row<'a>) -> Result<Summary<'a>> {
    let credited = sum(row.by_voucher.values().copied())?;
    let max_single = row.by_voucher.values().copied().max().unwrap_or(0);
    let mut months: BTreeMap<String, i64> = BTreeMap::new();
    for (k, amount) in &row.by_voucher {
        let slot = months.entry(month_key(row.vouchers[k])).or_insert(0);
        *slot = slot.checked_add(*amount).ok_or_else(|| overflow(TEST_ID))?;
    }
    Ok(Summary {
        credited,
        max_single,
        months,
        vouchers: row.vouchers.clone(),
    })
}

/// The reference's `deductor_status`: "deductor" | "not_deductor" | "unknown". An individual/HUF
/// deducts only if its preceding year's business turnover exceeded the business limit, or its
/// profession's gross receipts the (lower) profession limit, as the activity says. Fails closed:
/// no turnover, a placeholder turnover, or an unrecorded activity between the two limits is
/// "unknown"; both activities (receipts not split) never conclude a deductor. Refuses without
/// `[deductor]`, or without the profession limit where it is read, as the reference raises.
pub(crate) fn deductor_status(
    entity_type: &str,
    rules: &Rules,
    turnover: Option<i64>,
    activity: Option<DeductorActivity>,
    placeholder: bool,
) -> Result<&'static str> {
    let business = rules
        .deductor_individual_huf_prev_year_turnover_paise
        .ok_or_else(|| AuditError::Config(format!("{TEST_ID} needs rules [deductor]")))?;
    if entity_type != "individual" && entity_type != "huf" {
        return Ok("deductor");
    }
    let Some(turnover) = turnover else {
        return Ok("unknown");
    };
    let profession = rules
        .deductor_individual_huf_prev_year_receipts_profession_paise
        .ok_or_else(|| {
            AuditError::Config(format!(
                "{TEST_ID} needs rules [deductor].individual_huf_prev_year_receipts_profession_paise"
            ))
        })?;
    let status = match activity {
        Some(DeductorActivity::Both) => {
            if turnover <= profession {
                "not_deductor"
            } else {
                "unknown"
            }
        }
        None => {
            if turnover > business {
                "deductor"
            } else if turnover <= profession {
                "not_deductor"
            } else {
                "unknown"
            }
        }
        Some(a) => {
            let limit = if a == DeductorActivity::Business {
                business
            } else {
                profession
            };
            if turnover > limit {
                "deductor"
            } else {
                "not_deductor"
            }
        }
    };
    Ok(if placeholder && status == "not_deductor" {
        "unknown"
    } else {
        status
    })
}

/// The reference's `deductor_question`: the "unknown" status's limit (`loans_interest` asks it too).
pub(crate) fn deductor_question(
    rules: &Rules,
    sections: &str,
    activity: Option<DeductorActivity>,
    placeholder: bool,
) -> Result<String> {
    let missing = || AuditError::Config(format!("{TEST_ID} needs rules [deductor]"));
    let business = rules
        .deductor_individual_huf_prev_year_turnover_paise
        .ok_or_else(missing)?;
    let profession = rules
        .deductor_individual_huf_prev_year_receipts_profession_paise
        .ok_or_else(missing)?;
    // i64 -> f64 is exact below 2^53 and the division correctly rounded, as Python's is.
    #[allow(clippy::cast_precision_loss)]
    let (crore, lakh) = (
        business as f64 / 1_000_000_000_f64,
        profession as f64 / 10_000_000_f64,
    );
    let recorded = match activity {
        Some(a @ (DeductorActivity::Business | DeductorActivity::Profession)) => {
            format!("the client is recorded as carrying on a {}; ", a.as_str())
        }
        Some(DeductorActivity::Both) => {
            "the client is recorded as carrying on both, its receipts not split between them; "
                .to_string()
        }
        None => {
            "whether the client carries on a business or a profession is not recorded; ".to_string()
        }
    };
    Ok(format!(
        "An individual/HUF is a {sections} deductor only if the immediately preceding year's \
turnover from a business exceeded ₹{} crore, or its gross receipts from a profession exceeded ₹{} \
lakh (s.194A(1) proviso; s.194C Explanation (i)(l)(B)); {recorded}the current year's books alone \
cannot establish this.{}",
        py_format_g(crore),
        py_format_g(lakh),
        if placeholder {
            " The previous-year turnover on file is a placeholder, so it is not relied on."
        } else {
            ""
        }
    ))
}

/// Python's `format(x, "g")`: six significant digits, trailing zeros dropped, scientific notation
/// below 1e-4 or from 1e6. The reference formats the deductor limits and rates this way.
pub(crate) fn py_format_g(x: f64) -> String {
    if x == 0.0 {
        return if x.is_sign_negative() { "-0" } else { "0" }.to_string();
    }
    let sci = format!("{x:.5e}");
    let (mantissa, exponent) = sci.split_once('e').expect("{:e} always has an exponent");
    let exponent: i32 = exponent.parse().expect("an integer exponent");
    let strip = |s: &str| -> String {
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s.to_string()
        }
    };
    if (-4..6).contains(&exponent) {
        let decimals = usize::try_from(5 - exponent).expect("0..=9 decimals");
        strip(&format!("{x:.decimals$}"))
    } else {
        let sign = if exponent < 0 { '-' } else { '+' };
        format!("{}e{sign}{:02}", strip(mantissa), exponent.abs())
    }
}

/// The section limits a row's tranches read: (single, aggregate, per month, s.194J, s.194H).
#[derive(Clone, Copy)]
struct Limits {
    single: i64,
    aggregate: i64,
    per_month: i64,
    s194j: i64,
    s194h: Option<i64>,
}

/// The reference's `_row_tranches`: the vouchers whose credits attract TDS, by their keys.
fn row_tranche_keys<'a>(
    nature: &str,
    row: &Row<'a>,
    limits: Limits,
) -> Result<BTreeSet<&'a VoucherKey>> {
    let credits: Vec<(TallyDate, &'a VoucherKey, i64)> = row
        .by_voucher
        .iter()
        .map(|(k, p)| (row.vouchers[k].date.clone(), *k, *p))
        .collect();
    let tranches = match nature {
        "194C" => crossing_tranches(
            &credits,
            Some(limits.single),
            Some(limits.aggregate),
            TEST_ID,
        )?,
        "194I" => {
            let mut by_month: BTreeMap<String, Vec<(TallyDate, &'a VoucherKey, i64)>> =
                BTreeMap::new();
            for c in credits {
                let s = c.0.as_str();
                by_month
                    .entry(format!("{}-{}", &s[0..4], &s[4..6]))
                    .or_default()
                    .push(c);
            }
            let mut all = Vec::new();
            for month in by_month.values() {
                all.extend(crossing_tranches(
                    month,
                    None,
                    Some(limits.per_month),
                    TEST_ID,
                )?);
            }
            all
        }
        "194H" => crossing_tranches(&credits, None, limits.s194h, TEST_ID)?,
        _ => crossing_tranches(&credits, None, Some(limits.s194j), TEST_ID)?,
    };
    Ok(tranches.into_iter().flat_map(|t| t.ids).collect())
}

/// Sort keys by (voucher date, key), as the reference's `key=lambda g: (date, g)`.
fn by_date<'a>(
    keys: impl IntoIterator<Item = &'a VoucherKey>,
    row: &Row<'a>,
) -> Vec<&'a VoucherKey> {
    let mut out: Vec<&'a VoucherKey> = keys.into_iter().collect();
    out.sort_by(|a, b| (&row.vouchers[a].date, a).cmp(&(&row.vouchers[b].date, b)));
    out
}

/// A voucher as evidence: its GUID and label, never the key a row holds it under (the
/// reference's `_ref`).
fn voucher_ref(v: &Voucher) -> EvidenceRef {
    EvidenceRef::with_label("voucher", &v.guid, &voucher_label(v))
}

/// The distinct refs of some vouchers, sorted by (id, label): vouchers sharing a GUID are each
/// cited unless their refs are identical too (the reference's `_refs`).
fn voucher_refs<'a>(vouchers: impl IntoIterator<Item = &'a Voucher>) -> Vec<EvidenceRef> {
    let mut ev: Vec<EvidenceRef> = vouchers.into_iter().map(voucher_ref).collect();
    ev.sort_by(|a, b| (&a.id, &a.label).cmp(&(&b.id, &b.label)));
    ev.dedup();
    ev
}

/// The reference's `_sections_on`: every mapped line's nature key, an empty nature included
/// (membership, not truthiness, as the reference reads it here).
fn sections_on(v: &Voucher, cfg: &TdsConfig) -> BTreeSet<NatureKey> {
    v.lines
        .iter()
        .filter_map(|l| {
            cfg.nature_by_ledger
                .get(&l.ledger)
                .map(|n| nature_key(n, &l.ledger, cfg))
        })
        .collect()
}

/// One payee row going into clause 21(b): the reference's `_Row21b`.
struct Row21b<'r, 'a> {
    prefix: &'r str,
    rid: &'r str,
    h: &'r str,
    nature: &'r str,
    cat_note: &'r str,
    entity: &'r str,
    row: &'r Row<'a>,
    pair: Option<(i64, i64)>,
    list_all: bool,
}

/// Everything clause 21(b) reads besides the row.
struct Ctx<'r> {
    rules: &'r Rules,
    cfg: &'r TdsConfig,
    inputs: &'r Inputs,
    book: &'r Book,
    deductor: &'static str,
    limits: Limits,
    deposited: Option<&'r BTreeMap<(String, String), i64>>,
}

/// The reference's `_tds_seen_limit`: publishes `tds_seen_<rid>` when TDS ledgers are classified.
fn tds_seen_limit(
    r: &mut TestResult,
    prefix: &str,
    rid: &str,
    h: &str,
    seen: &BTreeMap<&VoucherKey, &Voucher>,
    tds_ledgers: &BTreeSet<String>,
) -> Result<String> {
    if tds_ledgers.is_empty() {
        return Ok(
            "Whether TDS on this payee covers the whole year's aggregate is not judged: the \
client's statutory dues classify no ledger as TDS payable."
                .to_string(),
        );
    }
    let tds = sum(seen
        .values()
        .map(|v| tds_on(v, tds_ledgers))
        .collect::<Result<Vec<_>>>()?)?;
    r.fig(
        &format!("{prefix}_row_tds_seen_{rid}"),
        Value::Int(tds),
        Unit::Paise,
        &format!(
            "TDS on the vouchers touching this payee entity (tag {h}): the lines on ledgers in the \
client's statutory dues classified as TDS payable, net, on every voucher that carries one and names \
the payee or credits it."
        ),
        voucher_refs(seen.values().copied()),
    )?;
    if seen.is_empty() {
        return Ok(
            "No TDS line (a ledger the client's statutory dues classify as TDS payable) is \
seen on this payee's vouchers."
                .to_string(),
        );
    }
    Ok(format!(
        "TDS lines are seen on {} voucher(s) touching this payee (net {}): which payments they \
cover, and whether the tax was deposited by the s.139(1) due date, is the CA's to determine (see \
the question on this payee's TDS).",
        seen.len(),
        rupees(i128::from(tds))
    ))
}

/// The reference's `_form_26a_note`.
fn form_26a_note(held: bool) -> String {
    if held {
        "Form 26A is recorded as held for this payee: the CA confirms which payments it relieves \
(a failure to deduct, second proviso to s.40(a)(ia)); nothing is taken out of clause 21(b) here."
            .to_string()
    } else {
        "Form 26A is recorded for this payee but not yet held; nothing is taken out of clause \
21(b)."
            .to_string()
    }
}

/// The reference's `_clause_21b`: returns (limits, clauses), adding figures, facts and findings.
fn clause_21b(
    r: &mut TestResult,
    ctx: &Ctx,
    x: &Row21b,
    mut limits: Vec<String>,
    clauses: Vec<String>,
    facts: &mut Vec<(String, String)>,
    seen: &BTreeMap<&VoucherKey, &Voucher>,
) -> Result<(Vec<String>, Vec<String>)> {
    let drop_21b: Vec<String> = clauses
        .iter()
        .filter(|c| c.as_str() != "3CD-21(b)")
        .cloned()
        .collect();
    if ctx.deductor == "not_deductor" {
        limits.push(
            "The assessee is not required to deduct tax under these sections on its previous \
year's turnover (its deductor status), so nothing is listed in clause 21(b)."
                .to_string(),
        );
        return Ok((limits, drop_21b));
    }
    let mut clauses = clauses;
    let (row, e) = (x.row, x.entity);
    let tds_ledgers = &ctx.inputs.tds_ledgers;
    // TDS seen, or over only with shared TDS: EVERY credit is listed.
    let taxable = if x.list_all || !seen.is_empty() {
        by_date(row.by_voucher.keys().copied(), row)
    } else {
        by_date(row_tranche_keys(x.nature, row, ctx.limits)?, row)
    };
    let mut own: Vec<&VoucherKey> = Vec::new();
    for g in &taxable {
        let v = row.vouchers[g];
        if tds_on(v, tds_ledgers)? > 0
            && e != PAYEE_NOT_NAMED
            && credited_parties(v, ctx.cfg, ctx.inputs, ctx.book)
                .iter()
                .all(|p| *p == e)
            && sections_on(v, ctx.cfg).len() == 1
        {
            own.push(g);
        }
    }
    let held = ctx.cfg.form_26a.get(e).copied();
    let low_high = x.pair.map(|(a, b)| (a.min(b), a.max(b)));
    // A plain bill: every line is the section's expense, the payee (a credit, or a debit equal to the
    // bill's own TDS: booked gross with its TDS debited back) or a TDS ledger. Only its value is read
    // off the books without a judgement, so only it is rate-tested.
    let plain = |v: &Voucher| -> Result<bool> {
        let tds = tds_on(v, tds_ledgers)?;
        let mut debit = 0_i64;
        for l in v
            .lines
            .iter()
            .filter(|l| l.amount_paise > 0 && alias(ctx.cfg, &l.ledger) == e)
        {
            debit = debit
                .checked_add(l.amount_paise)
                .ok_or_else(|| overflow(TEST_ID))?;
        }
        Ok((debit == 0 || debit == tds)
            && v.lines.iter().filter(|l| l.amount_paise != 0).all(|l| {
                ctx.cfg.nature_by_ledger.contains_key(&l.ledger)
                    || tds_ledgers.contains(&l.ledger)
                    || alias(ctx.cfg, &l.ledger) == e
            }))
    };
    let mut rated: Vec<&VoucherKey> = Vec::new();
    if low_high.is_some() {
        for g in &own {
            if plain(row.vouchers[g])? {
                rated.push(*g);
            }
        }
    }
    // Q2-d, as a fact per bill: its own voucher's TDS against the section's rates, on the bill's
    // value before TDS -- the payee's net credit plus its own TDS.
    let (mut short, mut mid) = (Vec::new(), Vec::new());
    let mut bases: BTreeMap<&VoucherKey, i64> = BTreeMap::new();
    if let Some((low, high)) = low_high {
        let floor_rate = |base: i64, bp: i64| -> i128 {
            (i128::from(base) * i128::from(bp) + 5000).div_euclid(10_000) - 100
        };
        for g in &rated {
            let v = row.vouchers[g];
            let tds = tds_on(v, tds_ledgers)?;
            let mut payee = 0_i64;
            for l in v.lines.iter().filter(|l| alias(ctx.cfg, &l.ledger) == e) {
                payee = payee
                    .checked_add(l.amount_paise)
                    .ok_or_else(|| overflow(TEST_ID))?;
            }
            let base = tds.checked_sub(payee).ok_or_else(|| overflow(TEST_ID))?;
            bases.insert(*g, base);
            if i128::from(tds) < floor_rate(base, low) {
                short.push(*g);
            } else if low != high && i128::from(tds) < floor_rate(base, high) {
                mid.push(*g);
            }
        }
    }
    let reversal = seen.values().any(|v| {
        v.lines
            .iter()
            .any(|l| tds_ledgers.contains(&l.ledger) && l.amount_paise > 0)
    });
    let candidates: Vec<&VoucherKey> = rated
        .iter()
        .copied()
        .filter(|g| SHORT_LEAVES_A || !short.contains(g))
        .collect();
    // P2-2: tax deducted on its own voucher, so not (ii)(A).
    let deducted: Vec<&VoucherKey> = if reversal {
        Vec::new()
    } else {
        candidates.clone()
    };
    let b_rows: Vec<&VoucherKey> = match ctx.deposited {
        Some(deposited) if !seen.is_empty() => own
            .iter()
            .copied()
            .filter(|g| {
                deposited
                    .get(&(x.nature.to_string(), month_key(row.vouchers[g])))
                    .copied()
                    .unwrap_or(0)
                    == 0
            })
            .collect(),
        _ => Vec::new(),
    };
    // Listed on the TDS question, for their deposit.
    let off_a: Vec<&VoucherKey> = deducted
        .iter()
        .copied()
        .filter(|g| !b_rows.contains(g))
        .collect();
    let listed: Vec<&VoucherKey> = taxable
        .iter()
        .copied()
        .filter(|g| !b_rows.contains(g) && !off_a.contains(g))
        .collect();

    for (n, g) in listed.iter().enumerate() {
        let id = r.fig(
            &format!("{}_row_21b_a_{}_{:03}", x.prefix, x.rid, n + 1),
            Value::Int(row.by_voucher[g]),
            Unit::Paise,
            &format!(
                "Clause 21(b)(ii)(A), s.40(a)(ia): one payment to this payee entity (tag {}) under \
{}{}, attracting TDS, as credited to the payee (the return applies the 30%). The voucher cited gives \
its date.",
                x.h, x.nature, x.cat_note
            ),
            vec![voucher_ref(row.vouchers[g])],
        )?;
        facts.push((format!("payment_a:{:03}", n + 1), id));
    }
    if listed.is_empty() {
        clauses = drop_21b;
    } else {
        let total = sum(listed.iter().map(|g| row.by_voucher[g]))?;
        let id = r.fig(
            &format!("{}_row_21b_a_total_{}", x.prefix, x.rid),
            Value::Int(total),
            Unit::Paise,
            &format!(
                "Clause 21(b)(ii)(A): the payments to this payee entity (tag {}) listed there, \
summed. Each is its own item in clause 21(b); this total is not.",
                x.h
            ),
            Vec::new(),
        )?;
        facts.push(("inadmissible_payments_total".to_string(), id));
    }

    if seen.is_empty() {
        if !listed.is_empty() {
            limits.push(format!(
                "{} payment(s) that attract TDS are listed in clause 21(b)(ii)(A) at their amounts \
as credited{}",
                listed.len(),
                if tds_ledgers.is_empty() {
                    "; whether TDS was deducted on them is not judged: the client's statutory dues \
classify no ledger as TDS payable."
                } else {
                    ": no TDS line is seen on the vouchers touching this payee."
                }
            ));
        }
        if let Some(held) = held {
            limits.push(form_26a_note(held));
        }
        return Ok((limits, clauses));
    }

    // TDS seen: listed, and ONE question.
    if !listed.is_empty() {
        limits.push(format!(
            "{} payment(s) that attract TDS are listed in clause 21(b)(ii)(A) at their amounts as \
credited: TDS is seen on vouchers touching this payee, so which payments it covers is not concluded \
(see the question on this payee's TDS).",
            listed.len()
        ));
    }
    if !off_a.is_empty() {
        limits.push(format!(
            "{} payment(s) are listed on this payee's TDS question as deducted on their own vouchers, \
not in clause 21(b)(ii)(A) in this draft: tax was deducted on them. Whether it was deposited by the \
s.139(1) due date is asked there.",
            off_a.len()
        ));
    }
    if let Some(held) = held {
        limits.push(form_26a_note(held));
    }
    if !b_rows.is_empty() {
        not_deposited_finding(r, ctx.rules, x, &b_rows, tds_ledgers)?;
        limits.push(format!(
            "{} payment(s), whose own bill carries this payee's TDS in a month for which the \
challans recorded show nothing deposited by the s.139(1) due date, are listed in clause \
21(b)(ii)(B).",
            b_rows.len()
        ));
    }
    let tds_total = sum(seen
        .values()
        .map(|v| tds_on(v, tds_ledgers))
        .collect::<Result<Vec<_>>>()?)?;
    let mut ordered: Vec<(&VoucherKey, &Voucher)> = seen.iter().map(|(g, v)| (*g, *v)).collect();
    ordered.sort_by(|a, b| (&a.1.date, a.0).cmp(&(&b.1.date, b.0)));
    let labels: Vec<String> = ordered.iter().map(|(_, v)| voucher_label(v)).collect();
    let shown = labels
        .iter()
        .take(12)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    let more = if labels.len() > 12 {
        format!(", and {} more", labels.len() - 12)
    } else {
        String::new()
    };
    let tds_seen_id = r.fig(
        &format!("{}_row_tds_seen_net_{}", x.prefix, x.rid),
        Value::Int(tds_total),
        Unit::Paise,
        &format!(
            "TDS, net, on the vouchers touching this payee entity (tag {}).",
            x.h
        ),
        Vec::new(),
    )?;
    let mut q_facts = vec![("tds_seen".to_string(), tds_seen_id)];
    for (n, g) in off_a.iter().enumerate() {
        let v = row.vouchers[g];
        let ev1 = vec![voucher_ref(v)];
        let id = r.fig(
            &format!("{}_row_21b_deducted_{}_{:03}", x.prefix, x.rid, n + 1),
            Value::Int(row.by_voucher[g]),
            Unit::Paise,
            &format!(
                "One payment to this payee entity (tag {}) under {}{} whose own voucher deducts its \
TDS, as credited to the payee: listed on this question rather than in clause 21(b)(ii)(A) in this \
draft. It is in (ii)(B) only if that tax was not deposited by the s.139(1) due date, which the CA \
determines. The voucher cited gives its date.",
                x.h, x.nature, x.cat_note
            ),
            ev1.clone(),
        )?;
        q_facts.push((format!("deducted_payment:{:03}", n + 1), id));
        let id = r.fig(
            &format!("{}_row_21b_deducted_tds_{}_{:03}", x.prefix, x.rid, n + 1),
            Value::Int(tds_on(v, tds_ledgers)?),
            Unit::Paise,
            &format!("The TDS on that payment's own voucher (tag {}).", x.h),
            ev1,
        )?;
        q_facts.push((format!("deducted_tds:{:03}", n + 1), id));
    }
    if !off_a.is_empty() {
        let id = r.fig(
            &format!("{}_row_21b_deducted_total_{}", x.prefix, x.rid),
            Value::Int(sum(off_a.iter().map(|g| row.by_voucher[g]))?),
            Unit::Paise,
            &format!(
                "The payments to this payee entity (tag {}) listed on this question as deducted on \
their own vouchers (not in clause 21(b)(ii)(A) or (ii)(B) in this draft), summed.",
                x.h
            ),
            Vec::new(),
        )?;
        q_facts.push(("deducted_payments_total".to_string(), id));
    }
    let mut guard = if reversal && !candidates.is_empty() {
        " A voucher touching this payee debits a ledger classified as TDS payable (a reversal or \
correction of a deduction, or another entry): what it undoes is not judged, so no payment is listed \
on this question as deducted on its own voucher; each stays in clause 21(b)(ii)(A) or (ii)(B)."
            .to_string()
    } else {
        String::new()
    };
    if !off_a.is_empty() {
        guard.push_str(
            " A reversal of a deduction booked without this payee's ledger (against the expense, \
say) is not seen here: the CA confirms that each deduction listed on this question stands.",
        );
    }
    r.findings.push(Finding {
        id: format!("{TEST_ID}/tds_seen/{}", x.rid),
        clauses: vec!["3CD-21(b)".to_string(), "3CD-34(a)".to_string()],
        title: format!(
            "TDS is seen on vouchers touching a payee over the {}{} limit: which payments it \
covers, and its deposit, are the CA's to determine",
            x.nature, x.cat_note
        ),
        facts: q_facts,
        evidence: voucher_refs(seen.values().copied()),
        confidence: Confidence::JudgementRequired,
        limits: vec![format!(
            "TDS of {} (net) is seen on {} voucher(s) touching this payee: {shown}{more}. The CA \
determines which payments it covers and whether it was deposited by the s.139(1) due date; until \
then every payment that attracts TDS is listed in clause 21(b){}{}.{guard}{}",
            rupees(i128::from(tds_total)),
            seen.len(),
            if b_rows.is_empty() {
                ""
            } else {
                " (in (B) where its own bill's TDS month shows nothing deposited on the challans \
recorded)"
            },
            if off_a.is_empty() {
                String::new()
            } else {
                format!(
                    ", except the {} payment(s) listed on this question as deducted on their own \
vouchers: not in clause 21(b)(ii)(A) or (ii)(B) in this draft, they are listed here for whether the \
tax was deposited by the due date",
                    off_a.len()
                )
            },
            if held == Some(true) {
                " The payee's Form 26A is recorded as held: which payments it relieves (a failure \
to deduct) is part of that determination."
            } else {
                ""
            }
        )],
        ask_client: vec![
            "For each payment to this payee, the TDS deducted on it and the challan that deposited \
it."
            .to_string(),
        ],
    });
    if low_high.is_some() {
        // A bill carrying TDS whose rate is not tested is counted and said, never dropped silently.
        let mut untested: Vec<&VoucherKey> = Vec::new();
        for g in &taxable {
            if !rated.contains(g) && tds_on(row.vouchers[g], tds_ledgers)? > 0 {
                untested.push(*g);
            }
        }
        if !untested.is_empty() {
            let why = if e == PAYEE_NOT_NAMED {
                "no payee is named on them"
            } else {
                "each carries lines other than the section's expense, the payee and its TDS (GST, \
another tax, another party or ledger), or more than one section"
            };
            r.fig(
                &format!("{}_row_tds_bills_not_rate_tested_{}", x.prefix, x.rid),
                count(TEST_ID, untested.len())?,
                Unit::Count,
                &format!(
                    "Bills to this payee entity (tag {}) under {}{} that carry TDS but whose rate is \
not tested, because {why}.",
                    x.h, x.nature, x.cat_note
                ),
                untested.iter().map(|g| voucher_ref(row.vouchers[g])).collect(),
            )?;
            limits.push(format!(
                "{} bill(s) carrying TDS are not tested against the section's rate: {why}.",
                untested.len()
            ));
        }
    }
    if let Some((low, high)) = low_high.filter(|_| !short.is_empty() || !mid.is_empty()) {
        short_deduction_finding(r, x, &short, &mid, (low, high), ctx, &bases, &listed)?;
    }
    Ok((limits, clauses))
}

/// The reference's `_not_deposited_finding`: clause 21(b)(ii)(B) for one payee.
fn not_deposited_finding(
    r: &mut TestResult,
    rules: &Rules,
    x: &Row21b,
    b_rows: &[&VoucherKey],
    tds_ledgers: &BTreeSet<String>,
) -> Result<()> {
    let row = x.row;
    let mut facts = Vec::new();
    for (n, g) in b_rows.iter().enumerate() {
        let v = row.vouchers[g];
        let ev1 = vec![voucher_ref(v)];
        let id = r.fig(
            &format!("{}_row_21b_b_{}_{:03}", x.prefix, x.rid, n + 1),
            Value::Int(row.by_voucher[g]),
            Unit::Paise,
            &format!(
                "Clause 21(b)(ii)(B), s.40(a)(ia): one payment to this payee entity (tag {}) under \
{}{} whose own bill carries its TDS, in a month for which the challans recorded show nothing \
deposited by the s.139(1) due date; its full amount. The voucher cited gives its date.",
                x.h, x.nature, x.cat_note
            ),
            ev1.clone(),
        )?;
        facts.push((format!("payment_b:{:03}", n + 1), id));
        let id = r.fig(
            &format!("{}_row_21b_b_tds_{}_{:03}", x.prefix, x.rid, n + 1),
            Value::Int(tds_on(v, tds_ledgers)?),
            Unit::Paise,
            &format!(
                "The TDS on that payment's own bill (tag {}); deposited for its month by the due \
date: nothing.",
                x.h
            ),
            ev1,
        )?;
        facts.push((format!("tax_deducted_b:{:03}", n + 1), id));
    }
    let id = r.fig(
        &format!("{}_row_21b_b_total_{}", x.prefix, x.rid),
        Value::Int(sum(b_rows.iter().map(|g| row.by_voucher[g]))?),
        Unit::Paise,
        &format!(
            "Clause 21(b)(ii)(B): the payments to this payee entity (tag {}) listed there, summed. \
Each is its own item in clause 21(b); this total is not.",
            x.h
        ),
        Vec::new(),
    )?;
    facts.push(("inadmissible_payments_total".to_string(), id));
    r.findings.push(Finding {
        id: format!("{TEST_ID}/not_deposited/{}", x.rid),
        clauses: vec!["3CD-21(b)".to_string(), "3CD-34(a)".to_string()],
        title: format!(
            "TDS deducted under {}{} but not deposited by the s.139(1) due date",
            x.nature, x.cat_note
        ),
        facts,
        evidence: voucher_refs(b_rows.iter().map(|g| row.vouchers[g])),
        confidence: Confidence::NeedsDocument,
        limits: vec![
            format!(
                "On the challans recorded, the months of these deductions show nothing deposited \
by the due date, so each of the {} payment(s) is listed in clause 21(b)(ii)(B) at its full amount, \
with the tax on its own bill and nothing deposited.",
                b_rows.len()
            ),
            format!(
                "The s.139(1) due date used is {} (an audit case, from the rules table{}",
                rules.due_date_return_audit_case,
                if rules.due_dates_status == "verified" {
                    ")."
                } else {
                    ", pending confirmation)."
                }
            ),
        ],
        ask_client: vec![
            "Confirm the challans for these months: a deposit not recorded reads as none."
                .to_string(),
        ],
    });
    Ok(())
}

/// A bill whose own voucher deducts TDS short of the section's lower rate: false keeps it in
/// clause 21(b)(ii)(A), with its short-deduction finding; true would move it onto the deposit
/// question. False errs loud: a token deduction never takes a bill out of (A). As the reference
/// sets it (28-Sep); the owner is asked.
const SHORT_LEAVES_A: bool = false;

/// Q2-d: the High Court split on whether s.40(a)(ia) reaches a short deduction, as the reference
/// states it.
const SHORT_DEDUCTION_SPLIT: &str = "Whether s.40(a)(ia) reaches a short deduction is not \
settled, and listing on that ground is the CA's decision. For no disallowance: PCIT v Media Worldwide Ltd, Bombay HC, ITA 19/2020, 24 April 2026 (order read), \
which agrees with four High Courts known only as quoted there: CIT v S.K. Tekriwal (2014) 361 ITR \
432 (Calcutta); Future First Info Services (2023) 290 Taxman 490 (Delhi); Kishore Rao & Others \
(HUF) (2016) 387 ITR 196 (Karnataka); Samsung Heavy Industries (2025) (Uttarakhand). For \
disallowance: CIT v PVS Memorial Hospital, Kerala HC, ITA 16/2014, 20 July 2015 (order read): a \
deduction under the wrong provision does not save the payer from s.40(a)(ia). The Supreme Court \
appeal against PVS Memorial (CA 10915-10916/2018) was pending at last report (as quoted in Media \
Worldwide); its current status was not checked.";

/// Python's `s.casefold() in ("kerala", "lakshadweep")`. Full case folding maps to one of these
/// two only through ASCII letters, U+212A KELVIN SIGN (to "k") and U+017F LATIN SMALL LETTER LONG S
/// (to "s"): the other folds that reach ASCII letters are multi-letter ("ss", "i" with a dot,
/// ligatures such as "ff" and "fi"), and neither word holds any of their sequences.
fn in_kerala_hc(state: &str) -> bool {
    let fold = |c: char| match c {
        '\u{212A}' => 'k',
        '\u{017F}' => 's',
        c => c.to_ascii_lowercase(),
    };
    ["kerala", "lakshadweep"]
        .iter()
        .any(|w| state.chars().count() == w.len() && state.chars().map(fold).eq(w.chars()))
}

/// The reference's `_short_deduction_finding`: `bases` holds each tested bill's value before
/// TDS, and `listed_a` the payments listed in clause 21(b)(ii)(A).
#[allow(clippy::too_many_arguments)]
fn short_deduction_finding(
    r: &mut TestResult,
    x: &Row21b,
    short: &[&VoucherKey],
    mid: &[&VoucherKey],
    (low, high): (i64, i64),
    ctx: &Ctx,
    bases: &BTreeMap<&VoucherKey, i64>,
    listed_a: &[&VoucherKey],
) -> Result<()> {
    let row = x.row;
    // i64 -> f64 is exact below 2^53, as Python's `bp / 100` is.
    #[allow(clippy::cast_precision_loss)]
    let pct = |bp: i64| format!("{}%", py_format_g(bp as f64 / 100.0));
    let rows = by_date(short.iter().chain(mid).copied(), row);
    let mut facts = Vec::new();
    for (n, g) in rows.iter().enumerate() {
        let v = row.vouchers[g];
        let ev1 = vec![voucher_ref(v)];
        let id = r.fig(
            &format!("{}_row_short_{}_{:03}", x.prefix, x.rid, n + 1),
            Value::Int(row.by_voucher[g]),
            Unit::Paise,
            &format!(
                "One payment to this payee entity (tag {}) under {}{} whose own TDS is less than \
the section's rate; its full amount. The voucher cited gives its date.",
                x.h, x.nature, x.cat_note
            ),
            ev1.clone(),
        )?;
        facts.push((format!("short_payment:{:03}", n + 1), id));
        let id = r.fig(
            &format!("{}_row_short_tds_{}_{:03}", x.prefix, x.rid, n + 1),
            Value::Int(tds_on(v, &ctx.inputs.tds_ledgers)?),
            Unit::Paise,
            &format!("The TDS on that payment's own voucher (tag {}).", x.h),
            ev1.clone(),
        )?;
        facts.push((format!("short_tds:{:03}", n + 1), id));
        let id = r.fig(
            &format!("{}_row_short_base_{}_{:03}", x.prefix, x.rid, n + 1),
            Value::Int(bases[g]),
            Unit::Paise,
            &format!(
                "The base that payment's TDS is tested against (tag {}): the payee's net credit on \
the bill plus the bill's own TDS, its value before TDS (a bill tested carries only the section's \
expense, the payee and its TDS).",
                x.h
            ),
            ev1,
        )?;
        facts.push((format!("short_base:{:03}", n + 1), id));
    }
    let id = r.fig(
        &format!("{}_row_short_total_{}", x.prefix, x.rid),
        Value::Int(sum(rows.iter().map(|g| row.by_voucher[g]))?),
        Unit::Paise,
        &format!(
            "The payments to this payee entity (tag {}) whose own TDS is less than the section's \
rate, summed.",
            x.h
        ),
        Vec::new(),
    )?;
    facts.push(("short_deducted_payments_total".to_string(), id));
    let mut limits = Vec::new();
    if !short.is_empty() {
        limits.push(format!(
            "{} payment(s) carry TDS on their own vouchers less than the section's lower rate \
({}).",
            short.len(),
            pct(low)
        ));
    }
    if !mid.is_empty() {
        let unless = match x.nature {
            "194C" => "short unless the payee is an individual or HUF",
            "194I" => "short unless the rent is for plant, machinery or equipment",
            _ => "short at the higher rate",
        };
        limits.push(format!(
            "{} payment(s) carry TDS at the section's lower rate ({}) but less than its higher \
({}): {unless}.",
            mid.len(),
            pct(low),
            pct(high)
        ));
    }
    limits.push(
        "Each bill tested carries only the section's expense, the payee and its TDS; its rate is \
tested on the bill's value before TDS (the payee's net credit plus its own TDS)."
            .to_string(),
    );
    let in_a = rows.iter().filter(|g| listed_a.contains(g)).count();
    limits.push(
        "TDS elsewhere on this payee's vouchers may cover them: see the question on this payee's \
TDS."
            .to_string(),
    );
    if in_a > 0 {
        limits.push(format!(
            "{in_a} of these payment(s) are listed in clause 21(b)(ii)(A) in this draft; whether to \
keep them there is the CA's decision."
        ));
    }
    if in_a < rows.len() {
        limits.push(format!(
            "{} of these payment(s) are not listed in clause 21(b)(ii)(A) in this draft (they are in \
(ii)(B), or on the question on this payee's TDS): tax was deducted on them; whether to list them in \
(ii)(A) is the CA's decision.",
            rows.len() - in_a
        ));
    }
    limits.push(SHORT_DEDUCTION_SPLIT.to_string());
    if let Some(state) = ctx.inputs.client_state.as_deref() {
        let state = py_strip(state);
        if in_kerala_hc(state) {
            limits.push(format!(
                "The client is recorded as in {state}, within the Kerala High Court's \
jurisdiction, where PVS Memorial applies: listing these payments in clause 21(b) is the \
conservative choice there."
            ));
        }
    }
    r.findings.push(Finding {
        id: format!("{TEST_ID}/short_deduction/{}", x.rid),
        clauses: vec!["3CD-34(a)".to_string(), "3CD-21(b)".to_string()],
        title: format!(
            "TDS deducted at less than the {}{} rate on payments to this payee",
            x.nature, x.cat_note
        ),
        facts,
        evidence: voucher_refs(rows.iter().map(|g| row.vouchers[g])),
        confidence: Confidence::JudgementRequired,
        limits,
        ask_client: vec![
            "Confirm the payee's type (or, for rent, what is let) and so the rate that applies; \
decide whether to list these payments in clause 21(b)."
                .to_string(),
        ],
    });
    Ok(())
}

/// The reference's `_s194c6_question`: a payee listed in clause 21(b) with vouchers on ledgers the
/// CA marks as goods carriage.
fn s194c6_question(
    r: &mut TestResult,
    rid: &str,
    h: &str,
    vouchers: &BTreeMap<&VoucherKey, &Voucher>,
    goods_carriage: &BTreeSet<String>,
    book: &Book,
) -> Result<()> {
    if vouchers.is_empty() {
        return Ok(());
    }
    let mut tags = BTreeSet::new();
    for v in vouchers.values() {
        for l in &v.lines {
            if goods_carriage.contains(&l.ledger) {
                tags.insert(stable_ledger_tag(book, &l.ledger)?);
            }
        }
    }
    let tags = tags.into_iter().collect::<Vec<_>>().join(", ");
    r.findings.push(Finding {
        id: format!("{TEST_ID}/s194c6/{rid}"),
        clauses: vec!["3CD-21(b)".to_string()],
        title: "Payments on ledgers marked as goods carriage: they stay listed unless the s.194C(6) \
conditions are evidenced"
            .to_string(),
        facts: Vec::new(),
        evidence: voucher_refs(vouchers.values().copied()),
        confidence: Confidence::JudgementRequired,
        limits: vec![
            format!(
                "{} of this payee's vouchers (tag {h}) are on ledgers the client marks as goods \
carriage (tags {tags}). Its payments stay where this draft lists them: in clause 21(b), or on the \
question on this payee's TDS.",
                vouchers.len()
            ),
            "They leave the list only if the CA holds evidence that s.194C(6) applied: the payee \
was in the business of plying, hiring or leasing goods carriages, owned ten or fewer goods carriages \
at any time during the previous year, and furnished a declaration to that effect along with its PAN \
(s.194C(6) as it stands from 1-6-2015; CBDT Circular 19/2015, para 43). The books show none of \
this."
                .to_string(),
        ],
        ask_client: vec![
            "For this payee: its s.194C(6) declaration with PAN, if any, and the number of goods \
carriages it owned in the year."
                .to_string(),
        ],
    });
    Ok(())
}

/// Run the test. `entity_type` is `[client].entity_type`; `cfg` is `[tds]`/`[tds_payees]`, bound;
/// `inputs` is what the reference's pack passes from the other tables.
#[allow(clippy::too_many_lines)]
pub fn run(
    book: &Book,
    rules: &Rules,
    entity_type: &str,
    cfg: &TdsConfig,
    inputs: &Inputs,
) -> Result<TestResult> {
    let missing = |table: &str| AuditError::Config(format!("{TEST_ID} needs rules [{table}]"));
    let s194c = rules.s194c.ok_or_else(|| missing("s194c"))?;
    let per_month_limit = rules
        .s194i_per_month_per_payee_paise
        .ok_or_else(|| missing("s194i"))?;
    let s194j_is_default = rules.s194j_aggregate_paise.is_none();
    let s194j_limit = rules
        .s194j_aggregate_paise
        .unwrap_or(DEFAULT_S194J_AGGREGATE_PAISE);
    // s.194H: tested only where a ledger is mapped to it, and never without the rules' own limit.
    let s194h_mapped = cfg.nature_by_ledger.values().any(|n| n == "194H");
    if s194h_mapped && rules.s194h_aggregate_paise.is_none() {
        return Err(AuditError::Config(format!(
            "{TEST_ID}: a ledger is mapped to 194H but the rules table has no [s194h]"
        )));
    }
    let limits_paise = Limits {
        single: s194c.single_sum_paise,
        aggregate: s194c.aggregate_paise,
        per_month: per_month_limit,
        s194j: s194j_limit,
        s194h: rules.s194h_aggregate_paise,
    };
    // `[tds].goods_carriage_ledgers` must each be mapped to 194C, as the reference's config
    // reader refuses otherwise.
    let bad: Vec<&String> = cfg
        .goods_carriage_ledgers
        .iter()
        .filter(|l| cfg.nature_by_ledger.get(*l).map(String::as_str) != Some("194C"))
        .collect();
    if !bad.is_empty() {
        return Err(AuditError::Config(format!(
            "[tds].goods_carriage_ledgers lists ledgers not mapped to 194C in \
[tds].nature_by_ledger: [{}]",
            bad.iter()
                .map(|s| crate::support::py_repr_str(s))
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }

    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let pop = book.population()?;
    r.population_note = "Books population (optional, cancelled and post-dated vouchers \
excluded); Contra excluded throughout. A voucher whose mapped expense lines span more than one \
TDS nature (rare) attributes its full credit-line total to each nature present, not divided or \
netted."
        .to_string();

    // ---------------------------------------------------------------- deductor status
    let status = deductor_status(
        entity_type,
        rules,
        cfg.previous_year_turnover_paise,
        inputs.deductor_activity,
        cfg.turnover_is_placeholder,
    )?;
    let f_status = r.fig(
        "deductor_status",
        Value::Text(status.to_string()),
        Unit::Text,
        "Whether the assessee must deduct TDS under s.194A/194C/194H/194-I/194J for the year: \
firm/LLP/company always; individual/HUF only if previous-year business turnover exceeded the \
rules' turnover limit for an individual or HUF -- never assumed from the current year's books \
alone.",
        Vec::new(),
    )?;
    if status == "unknown" {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/deductor_status"),
            clauses: vec!["3CD-21(b)".to_string(), "3CD-34(a)".to_string()],
            title: "Deductor status depends on previous-year turnover".to_string(),
            facts: vec![("deductor_status".to_string(), f_status)],
            evidence: Vec::new(),
            confidence: Confidence::JudgementRequired,
            limits: vec![deductor_question(
                rules,
                "s.194A/194C/194H/194-I/194J",
                inputs.deductor_activity,
                cfg.turnover_is_placeholder,
            )?],
            ask_client: vec![
                "Confirm previous-year turnover or gross receipts, and whether from a business or \
a profession."
                    .to_string(),
            ],
        });
    }

    // ---------------------------------------------------------------- firm-level: TDS ledger figure
    let named_tds_ledgers: Vec<&String> = book
        .ledgers
        .iter()
        .filter(|(name, l)| l.under(DUTIES_TAXES_GROUP) && py_lower(name).contains("tds"))
        .map(|(name, _)| name)
        .collect();
    r.fig(
        "tds_ledger_under_duties_taxes_count",
        count(TEST_ID, named_tds_ledgers.len())?,
        Unit::Count,
        "Ledgers under 'Duties & Taxes' whose name contains 'tds' (case-insensitive) -- a books \
figure only; existence or absence of such a ledger is not itself a conclusion about TDS compliance.",
        named_tds_ledgers
            .iter()
            .map(|n| EvidenceRef::with_label("ledger", n, n))
            .collect(),
    )?;

    // ---------------------------------------------------------------- payee rows
    // Each voucher's own key: a row keeps a voucher's credit, date and count by it, so two vouchers
    // sharing a GUID are two bills (#1243).
    let keyed = voucher_keys(&pop)?;
    let rows = compute_payee_rows(&keyed, book, cfg)?;
    // A classification counts only on the row holding its bill; one whose bill is a credit to no
    // payee at all is a mistake in the client's list, refused rather than ignored.
    let credited: BTreeSet<&str> = rows.values().flat_map(Row::credited_guids).collect();
    for (guid, c) in &cfg.reversals {
        let bill = match c {
            Reversal::CreditNote => continue,
            Reversal::BillSpecific(b) | Reversal::DuplicateOrError(b) => b,
        };
        if !credited.contains(bill.as_str()) {
            return Err(AuditError::Config(format!(
                "[tds_payees.reversals] {}: bill {} is not a credit to any payee",
                crate::support::py_repr_str(guid),
                crate::support::py_repr_str(bill)
            )));
        }
    }
    let mut adjusted: BTreeMap<&RowKey, Adjusted> = BTreeMap::new();
    for (key, d) in &rows {
        adjusted.insert(key, adjusted_row(d, &key.1, cfg, inputs, book)?);
    }
    // The TDS seen for a payee is the TDS on ANY voucher touching it.
    let entities: BTreeSet<&str> = rows
        .keys()
        .map(|(_, e)| e.as_str())
        .filter(|e| *e != WITHIN_GOODS_INVOICE && *e != PAYEE_NOT_NAMED)
        .collect();
    let mut tds_touching: BTreeMap<&str, BTreeMap<&VoucherKey, &Voucher>> = BTreeMap::new();
    if !inputs.tds_ledgers.is_empty() {
        for (k, v) in &keyed {
            if v.base_type == "Contra" || !has_tds_line(v, &inputs.tds_ledgers) {
                continue;
            }
            let touched: BTreeSet<&str> = v.lines.iter().map(|l| alias(cfg, &l.ledger)).collect();
            for e in touched.intersection(&entities) {
                tds_touching.entry(e).or_default().insert(k, v);
            }
        }
    }
    let deposited: Option<BTreeMap<(String, String), i64>> = match &cfg.challans {
        None => None,
        Some(challans) => {
            let due = TallyDate::parse(rules.due_date_return_audit_case.replace('-', ""))
                .map_err(|e| AuditError::Config(format!("rules: [due_dates]: {e}")))?;
            let mut out: BTreeMap<(String, String), i64> = BTreeMap::new();
            for c in challans {
                if c.date <= due {
                    let slot = out.entry((c.section.clone(), c.month.clone())).or_insert(0);
                    *slot = slot
                        .checked_add(c.amount_paise)
                        .ok_or_else(|| overflow(TEST_ID))?;
                }
            }
            Some(out)
        }
    };
    // Entries in the client's lists that match nothing in these books.
    let reversal_vouchers: BTreeSet<&str> = rows
        .values()
        .flat_map(|d| d.reversals.values().map(|(v, _)| v.guid.as_str()))
        .collect();
    let mut unmatched: Vec<String> = Vec::new();
    unmatched.extend(
        cfg.reversals
            .keys()
            .filter(|g| !reversal_vouchers.contains(g.as_str()))
            .map(|g| format!("reversal {g}")),
    );
    unmatched.extend(
        cfg.gst_separate
            .iter()
            .filter(|n| !entities.contains(n.as_str()))
            .map(|n| format!("GST by agreement: {n}")),
    );
    unmatched.extend(
        cfg.foreseeability_names
            .iter()
            .filter(|n| !entities.contains(n.as_str()) && !inputs.other_names.contains(*n))
            .map(|n| format!("foreseeability: {n}")),
    );
    unmatched.extend(
        cfg.form_26a
            .keys()
            .filter(|n| !entities.contains(n.as_str()))
            .map(|n| format!("Form 26A: {n}")),
    );
    unmatched.sort();
    if !unmatched.is_empty() {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/config_unmatched"),
            clauses: Vec::new(),
            title: "Entries in the client's TDS lists that match nothing in these books"
                .to_string(),
            facts: Vec::new(),
            evidence: Vec::new(),
            confidence: Confidence::JudgementRequired,
            limits: vec![format!(
                "Each entry below names a payee or voucher these books do not hold, so it has no \
effect: {}.",
                unmatched.join("; ")
            )],
            ask_client: vec![
                "Check each entry's spelling against the ledger or voucher it is meant to name."
                    .to_string(),
            ],
        });
    }
    let rate = |f: fn(&crate::rules::TdsRates) -> Option<i64>| rules.tds_rates.as_ref().and_then(f);
    let rate_pair = |prefix: &str| -> Option<(Option<i64>, Option<i64>)> {
        match prefix {
            "194C" => Some((
                rate(|t| Some(t.s194c_individual_huf_bp)),
                rate(|t| Some(t.s194c_other_bp)),
            )),
            "194I" => Some((
                rate(|t| Some(t.s194i_plant_machinery_bp)),
                rate(|t| Some(t.s194i_land_building_bp)),
            )),
            "194J_professional" => {
                let p = rate(|t| Some(t.s194j_professional_bp));
                Some((p, p))
            }
            "194J_technical" => {
                let p = rate(|t| Some(t.s194j_technical_bp));
                Some((p, p))
            }
            "194H" => {
                let p = rate(|t| t.s194h_bp);
                Some((p, p))
            }
            _ => None,
        }
    };
    let clauses_by_nature = |nature: &str| -> Vec<String> {
        let section = match nature {
            "194C" => "s.194C(5)",
            "194I" => "s.194-I",
            "194H" => "s.194H",
            _ => "s.194J",
        };
        vec![
            section.to_string(),
            "3CD-21(b)".to_string(),
            "3CD-34(a)".to_string(),
        ]
    };

    // (nature, subcat, prefix): one row per 194J category plus unmapped, each tested separately.
    let mut nature_subcats: Vec<(&str, &str, String)> = Vec::new();
    for nature in NATURES {
        if nature == "194H" && !s194h_mapped {
            continue; // no 194H figures on a book that maps no ledger to it
        }
        if nature == "194J" {
            for cat in CATEGORIES_194J.iter().copied().chain([CATEGORY_UNMAPPED]) {
                nature_subcats.push((nature, cat, format!("194J_{cat}")));
            }
        } else {
            nature_subcats.push((nature, "", nature.to_string()));
        }
    }

    for (nature, subcat, prefix) in &nature_subcats {
        let (nature, subcat, prefix) = (*nature, *subcat, prefix.as_str());
        let mut summaries: BTreeMap<&str, Summary> = BTreeMap::new();
        for key in rows.keys() {
            if key.0 .0 == nature && key.0 .1 == subcat {
                summaries.insert(key.1.as_str(), summarise(&adjusted[key].row)?);
            }
        }
        let row_key =
            |e: &str| -> RowKey { ((nature.to_string(), subcat.to_string()), e.to_string()) };

        let goods_total = summaries
            .get(WITHIN_GOODS_INVOICE)
            .map_or(0, |s| s.credited);
        let cat_note = if subcat.is_empty() {
            String::new()
        } else {
            format!(", category '{subcat}'")
        };
        r.fig(
            &format!("{prefix}_within_supplier_goods_invoices_total"),
            Value::Int(goods_total),
            Unit::Paise,
            &format!(
                "Sum of the {nature}-mapped expense line(s) themselves (the charge -- e.g. freight \
-- not the supplier's full invoice credit) on vouchers that also carry a Purchase Accounts/Sales \
Accounts line{cat_note}: inside a goods invoice, not a separate contract with the payee; excluded \
from the per-payee tests."
            ),
            Vec::new(),
        )?;

        let payee_entities: BTreeMap<&str, &Summary> = summaries
            .iter()
            .filter(|(e, _)| **e != WITHIN_GOODS_INVOICE)
            .map(|(e, s)| (*e, s))
            .collect();
        let credited_total = sum(payee_entities.values().map(|s| s.credited))?;
        r.fig(
            &format!("{prefix}_payee_entities_count"),
            count(TEST_ID, payee_entities.len())?,
            Unit::Count,
            &format!(
                "Distinct payee entities (ledgers the client's setup names as one payee counted \
once; 'payee not named' counted as one entity, 'within supplier goods invoices' excluded) credited on a voucher with a {nature}-mapped \
expense ledger line{cat_note}."
            ),
            Vec::new(),
        )?;
        r.fig(
            &format!("{prefix}_credited_total"),
            Value::Int(credited_total),
            Unit::Paise,
            &format!(
                "Sum credited to every payee entity credited on a voucher with a {nature}-mapped \
expense ledger line{cat_note} (excludes the goods-invoice bucket). Never summed with any other category's total before a threshold test."
            ),
            Vec::new(),
        )?;

        let is_unmapped_194j = nature == "194J" && subcat == CATEGORY_UNMAPPED;
        let s194h_limit = limits_paise.s194h.unwrap_or(i64::MAX);
        let mut over: BTreeMap<&str, &Summary> = payee_entities
            .iter()
            .filter(|(_, s)| {
                if is_unmapped_194j {
                    s.credited > 0
                } else if nature == "194C" {
                    s.credited > s194c.aggregate_paise || s.max_single > s194c.single_sum_paise
                } else if nature == "194I" {
                    s.months
                        .values()
                        .max()
                        .is_some_and(|m| *m > per_month_limit)
                } else if nature == "194H" {
                    s.credited > s194h_limit
                } else {
                    s.credited > s194j_limit
                }
            })
            .map(|(e, s)| (*e, *s))
            .collect();
        // A payee under the limit only because a Duties & Taxes credit on its bills (not classified
        // as TDS payable) may be netted TDS is raised too, as "possibly over".
        let agg_limit = match nature {
            "194C" => Some(s194c.aggregate_paise),
            "194J" => Some(s194j_limit),
            "194H" => limits_paise.s194h,
            _ => None,
        };
        let mut possibly_over: BTreeSet<&str> = BTreeSet::new();
        if nature == "194I" {
            for (e, s) in &payee_entities {
                let adj = &adjusted[&row_key(e)];
                if over.contains_key(e) || adj.netted_by_voucher.is_empty() {
                    continue;
                }
                let original = &rows[&row_key(e)];
                let mut extra: BTreeMap<String, i64> = BTreeMap::new();
                for (g, x) in &adj.netted_by_voucher {
                    let slot = extra.entry(month_key(original.vouchers[g])).or_insert(0);
                    *slot = slot.checked_add(*x).ok_or_else(|| overflow(TEST_ID))?;
                }
                let mut crosses = false;
                for (m, x) in &extra {
                    let total = s
                        .months
                        .get(m)
                        .copied()
                        .unwrap_or(0)
                        .checked_add(*x)
                        .ok_or_else(|| overflow(TEST_ID))?;
                    crosses |= total > per_month_limit;
                }
                if crosses {
                    over.insert(e, s);
                    possibly_over.insert(e);
                }
            }
        }
        if let (Some(agg), false) = (agg_limit, is_unmapped_194j) {
            for (e, s) in &payee_entities {
                let extra = adjusted[&row_key(e)].possibly_netted;
                if !over.contains_key(e)
                    && extra != 0
                    && s.credited
                        .checked_add(extra)
                        .ok_or_else(|| overflow(TEST_ID))?
                        > agg
                {
                    over.insert(e, s);
                    possibly_over.insert(e);
                }
            }
        }
        // A TDS line on a bill that also credits another payee cannot be divided, so it counts
        // whole toward this payee's limit (loud): over with it, the payee is listed and asked.
        let mut shared_over: BTreeSet<&str> = BTreeSet::new();
        if !is_unmapped_194j {
            for (e, s) in &payee_entities {
                let adj = &adjusted[&row_key(e)];
                if over.contains_key(e) || adj.shared_tds == 0 {
                    continue;
                }
                let credit_of = |g: &VoucherKey| {
                    adj.row.by_voucher.get(g).copied().ok_or_else(|| {
                        AuditError::Config(format!(
                            "{TEST_ID}: a shared-TDS bill {} is not among the payee's credits",
                            rows[&row_key(e)].vouchers[g].guid
                        ))
                    })
                };
                let crosses = if nature == "194I" {
                    let mut extra: BTreeMap<String, i64> = BTreeMap::new();
                    for (g, x) in &adj.shared_by_voucher {
                        credit_of(g)?;
                        let slot = extra.entry(month_key(adj.row.vouchers[g])).or_insert(0);
                        *slot = slot.checked_add(*x).ok_or_else(|| overflow(TEST_ID))?;
                    }
                    let mut crosses = false;
                    for (m, x) in &extra {
                        let total = s
                            .months
                            .get(m)
                            .copied()
                            .unwrap_or(0)
                            .checked_add(*x)
                            .ok_or_else(|| overflow(TEST_ID))?;
                        crosses |= total > per_month_limit;
                    }
                    crosses
                } else {
                    let agg = agg_limit.expect("an aggregate limit for every nature but 194-I");
                    let mut crosses = s
                        .credited
                        .checked_add(adj.shared_tds)
                        .ok_or_else(|| overflow(TEST_ID))?
                        > agg;
                    if !crosses && nature == "194C" {
                        // Python's `any` stops at the first bill over the single-sum limit.
                        for (g, x) in &adj.shared_by_voucher {
                            let total = credit_of(g)?
                                .checked_add(*x)
                                .ok_or_else(|| overflow(TEST_ID))?;
                            if total > s194c.single_sum_paise {
                                crosses = true;
                                break;
                            }
                        }
                    }
                    crosses
                };
                if crosses {
                    over.insert(e, s);
                    shared_over.insert(e);
                }
            }
        }

        r.fig(
            &format!("{prefix}_over_limit_payee_count"),
            count(TEST_ID, over.len())?,
            Unit::Count,
            &format!(
                "Payee entities credited on a voucher with a {nature}-mapped expense ledger line \
whose {nature}{cat_note} test trips (single sum/aggregate for \
194C, any month for 194-I, per-category aggregate for 194J; every nonzero-credit payee for category \
'unmapped', which is never threshold-tested)."
            ),
            Vec::new(),
        )?;

        let mut ordered: Vec<(&str, &Summary)> = over.into_iter().collect();
        ordered.sort_by_key(|(e, _)| hash8(e));
        for (entity, s) in ordered {
            let h = hash8(&format!("{prefix}:{}", stable_ledger_tag(book, entity)?));
            let rid = format!("{prefix}_{h}");
            let evidence = voucher_refs(s.vouchers.values().copied());
            // Two over-limit entities with one tag repeat this figure id: the reference raises
            // there, and `fig` refuses.
            let f_credited = r.fig(
                &format!("{prefix}_row_credited_{rid}"),
                Value::Int(s.credited),
                Unit::Paise,
                &format!(
                    "Total credited to one payee entity (tag {h}) under {nature}{cat_note}, summed \
across every population voucher touching a mapped expense ledger."
                ),
                evidence.clone(),
            )?;
            let f_max_single = r.fig(
                &format!("{prefix}_row_max_single_{rid}"),
                Value::Int(s.max_single),
                Unit::Paise,
                &format!(
                    "Largest total credited to this payee entity (tag {h}) within one voucher, \
under {nature}{cat_note}."
                ),
                Vec::new(),
            )?;
            // An unmapped-category or "possibly over" amount is never a computed TDS default: its
            // fact key keeps it out of the clause amount sums while the finding still counts.
            let credited_key = if is_unmapped_194j {
                "unmapped_category_credited"
            } else if possibly_over.contains(entity) {
                "possibly_over_credited"
            } else {
                "credited"
            };
            let mut facts = vec![
                (credited_key.to_string(), f_credited),
                ("max_single".to_string(), f_max_single),
            ];
            if nature == "194I" {
                let max_month = s.months.values().copied().max().unwrap_or(0);
                let f_month = r.fig(
                    &format!("{prefix}_row_max_month_{rid}"),
                    Value::Int(max_month),
                    Unit::Paise,
                    &format!(
                        "Highest single calendar-month credit total to this payee entity (tag {h}) \
under 194-I."
                    ),
                    Vec::new(),
                )?;
                facts.push(("max_month".to_string(), f_month));
            }

            let (title, mut limits, ask_client): (String, Vec<String>, Vec<String>) =
                if is_unmapped_194j {
                    (
                        "Payee credited under a 194J-mapped ledger with no known s.194J category"
                            .to_string(),
                        vec![
                            "This ledger is treated as a s.194J payment, but its fee category \
(professional, technical, royalty, or s.28(va)) has not been recorded, so which of the first \
proviso's four separate ₹50,000 tests applies cannot be determined from the books alone."
                                .to_string(),
                            "This amount is NEVER summed with any known-category total for this \
payee before a threshold test -- doing so would silently re-create the over-flagging bug this \
correction fixes (gap register section 7)."
                                .to_string(),
                        ],
                        vec![
                            "Confirm which s.194J category (professional fees, technical fees, \
royalty, or s.28(va)) this ledger's payments belong to, then re-test against that category's own \
₹50,000 aggregate."
                                .to_string(),
                        ],
                    )
                } else if entity == PAYEE_NOT_NAMED {
                    (
                        format!(
                            "Cash/bank paid under a {nature}-mapped expense, payee not \
identified, over the limit"
                        ),
                        vec![
                            "The credit line is a cash or bank ledger, not a named payee; the \
actual payee must be identified from narration, vouchers or the bank statement before any TDS \
conclusion can be drawn for this amount."
                                .to_string(),
                            "s.40(a)(ia) disallowance and its Form 26A relief cannot be assessed \
until the payee is known."
                                .to_string(),
                        ],
                        vec![
                            "Identify the payee(s) behind this cash/bank credit from narration, \
vouchers or the bank statement, then re-test."
                                .to_string(),
                        ],
                    )
                } else if possibly_over.contains(entity) {
                    (
                        format!(
                            "Payee possibly over the {nature}{cat_note} limit: a Duties & Taxes \
credit on its bills may be TDS netted"
                        ),
                        vec![
                            "Counted as booked, this payee's credits are within the limit; \
counted gross of the Duties & Taxes credit on its bills, they are over it. Whether that credit is \
TDS is not known from the books."
                                .to_string(),
                        ],
                        vec![
                            "Confirm what the Duties & Taxes credit on this payee's bills is; if \
it is TDS, classify the ledger as TDS payable in the client's statutory dues and re-test."
                                .to_string(),
                        ],
                    )
                } else {
                    (
                        format!("Payee over the {nature}{cat_note} limit"),
                        vec![
                            "Books only: the contract's nature, any lower/nil-deduction \
certificate (s.197) and, for 194C, a s.194C(6) declaration (a contractor plying, hiring or leasing \
goods carriages that owned ten or fewer goods carriages at any time during the previous year, with \
its PAN) are not visible from vouchers -- contract nature and any certificate/declaration are a CA \
judgement call, not a books fact."
                                .to_string(),
                            "s.40(a)(ia) disallows 30% of the sum on TDS default; the second \
proviso removes this if the payee's Form 26A (Rule 31ACB) shows the income was returned -- \
s.201(1A) interest still runs either way."
                                .to_string(),
                        ],
                        vec![
                            "Confirm the nature of the contract/service and whether any \
lower/nil-deduction certificate or s.194C(6) declaration applies."
                                .to_string(),
                            "If no TDS was deducted, confirm whether Form 26A is available for \
this payee."
                                .to_string(),
                        ],
                    )
                };
            if nature == "194J" && !is_unmapped_194j && s194j_is_default {
                limits.push(format!(
                    "The s.194J aggregate limit used here ({s194j_limit} paise) is a local \
prototype default (status=\"confirm\"), pending confirmation in the rules table -- not yet a \
verified rule."
                ));
            }

            let key = row_key(entity);
            let adj = &adjusted[&key];
            let row_adj = &adj.row;
            // By GUID, as the client's list of reversals names a bill; a GUID's bills in the books'
            // order, as the reference reads its row, which is their keys' order (the place is
            // zero-padded).
            let mut bills: BTreeMap<&str, Vec<&Voucher>> = BTreeMap::new();
            for v in rows[&key].vouchers.values() {
                bills.entry(v.guid.as_str()).or_default().push(v);
            }
            if adj.tds_grossed_up != 0 {
                limits.push(format!(
                    "TDS of {} deducted on this payee's own bills is added back: the credits are \
counted gross of it (a bill booked net of TDS credits the payee the net).{}",
                    rupees(i128::from(adj.tds_grossed_up)),
                    if adj.also_debited != 0 {
                        format!(
                            " {} of those bills also debit the payee (the TDS, or an advance): \
their amounts may overstate by the TDS.",
                            adj.also_debited
                        )
                    } else {
                        String::new()
                    }
                ));
            }
            if adj.possibly_netted != 0 {
                limits.push(format!(
                    "A Duties & Taxes ledger not classified as TDS payable is credited on this \
payee's bills ({}): if it is TDS, these credits are counted net of it. Classify the ledger as TDS \
payable in the client's statutory dues to count them gross.",
                    rupees(i128::from(adj.possibly_netted))
                ));
            }
            let pair = rate_pair(prefix);
            let mut seen: BTreeMap<&VoucherKey, &Voucher> =
                tds_touching.get(entity).cloned().unwrap_or_default();
            for (g, v) in &row_adj.vouchers {
                if has_tds_line(v, &inputs.tds_ledgers) {
                    seen.insert(*g, *v);
                }
            }
            let pair = match pair {
                Some((Some(a), Some(b))) => Some((a, b)),
                _ => None,
            };
            if !is_unmapped_194j && entity != PAYEE_NOT_NAMED && pair.is_some() {
                limits.push(tds_seen_limit(
                    &mut r,
                    prefix,
                    &rid,
                    &h,
                    &seen,
                    &inputs.tds_ledgers,
                )?);
            }
            let mut clauses = clauses_by_nature(nature);
            if !is_unmapped_194j && !possibly_over.contains(entity) {
                if shared_over.contains(entity) {
                    limits.push(format!(
                        "Counted with the TDS on its bills that also credit another party -- a \
payee, a bank, a round-off ({}), which the books do not divide -- this payee's credits exceed the \
limit: every credit to it is listed (the list may overstate).",
                        rupees(i128::from(adj.shared_tds))
                    ));
                }
                let ctx = Ctx {
                    rules,
                    cfg,
                    inputs,
                    book,
                    deductor: status,
                    limits: limits_paise,
                    deposited: deposited.as_ref(),
                };
                let x = Row21b {
                    prefix,
                    rid: &rid,
                    h: &h,
                    nature,
                    cat_note: &cat_note,
                    entity,
                    row: row_adj,
                    pair,
                    list_all: shared_over.contains(entity),
                };
                (limits, clauses) =
                    clause_21b(&mut r, &ctx, &x, limits, clauses, &mut facts, &seen)?;
                let listed = facts.iter().any(|(k, _)| k.starts_with("payment_a:"))
                    || r.findings
                        .iter()
                        .any(|f| f.id == format!("{TEST_ID}/not_deposited/{rid}"))
                    || r.findings.iter().any(|f| {
                        f.id == format!("{TEST_ID}/tds_seen/{rid}")
                            && f.facts
                                .iter()
                                .any(|(k, _)| k.starts_with("deducted_payment:"))
                    });
                if nature == "194C" && !cfg.goods_carriage_ledgers.is_empty() && listed {
                    let on_goods_carriage: BTreeMap<&VoucherKey, &Voucher> = row_adj
                        .vouchers
                        .iter()
                        .filter(|(_, v)| {
                            v.lines
                                .iter()
                                .any(|l| cfg.goods_carriage_ledgers.contains(&l.ledger))
                        })
                        .map(|(g, v)| (*g, *v))
                        .collect();
                    s194c6_question(
                        &mut r,
                        &rid,
                        &h,
                        &on_goods_carriage,
                        &cfg.goods_carriage_ledgers,
                        book,
                    )?;
                }
            }
            if adj.gst_excluded != 0 {
                limits.push(format!(
                    "GST of {} is left out of this payee's credits: the client's list records that \
the agreement states GST separately (CBDT Circular 23/2017 speaks of the agreement, not the \
invoice).",
                    rupees(i128::from(adj.gst_excluded))
                ));
            }
            if adj.gst_counted != 0 {
                limits.push(format!(
                    "GST of {} on this payee's vouchers is counted in its credits: no agreement \
stating GST separately is recorded for it (or the voucher credits another payee too). If the \
agreement states it separately, record that and GST is left out.",
                    rupees(i128::from(adj.gst_counted))
                ));
            }
            let rv = &adj.reversals;
            if !rv.unclassified.is_empty() {
                limits.push(format!(
                    "{} reversal(s) of credits to this payee are not classified in the client's \
list of reversals, so they are counted gross (the aggregate is not lowered): {}.",
                    rv.unclassified.len(),
                    rv.unclassified
                        .iter()
                        .map(|v| voucher_label(v))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            if !rv.not_applied.is_empty() {
                limits.push(format!(
                    "{} reversal(s) the client's list of reversals classifies as reversing a \
particular bill, or as a duplicate or error, are NOT applied: the credits are counted gross, pending \
bill-wise detail from the books, because the part of each reversal that falls on the bill it names \
cannot yet be read. Adjust for them in judging this payee: {}.",
                    rv.not_applied.len(),
                    rv.not_applied
                        .iter()
                        .map(|(v, kind, bill)| format!(
                            "{} ({}, against {})",
                            voucher_label(v),
                            if *kind == "bill_specific" {
                                "bill-specific"
                            } else {
                                "duplicate or error"
                            },
                            bills[bill.as_str()]
                                .iter()
                                .map(|b| voucher_label(b))
                                .collect::<Vec<_>>()
                                .join(" or ")
                        ))
                        .collect::<Vec<_>>()
                        .join("; ")
                ));
            }
            if !rv.credit_notes.is_empty() {
                limits.push(format!(
                    "Reversals the client's list of reversals classifies as credit notes, kept \
gross: {}.",
                    rv.credit_notes
                        .iter()
                        .map(|v| voucher_label(v))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            let confidence =
                if is_unmapped_194j || possibly_over.contains(entity) || !rv.not_applied.is_empty()
                {
                    Confidence::JudgementRequired
                } else {
                    Confidence::NeedsDocument
                };
            r.findings.push(Finding {
                id: format!("{TEST_ID}/{rid}"),
                clauses,
                title,
                facts,
                evidence,
                confidence,
                limits,
                ask_client,
            });
        }
    }
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Python 3.13's `format(x, "g")`, case by case.
    #[test]
    fn py_format_g_matches_python() {
        for (x, want) in [
            (1.0, "1"),
            (0.5, "0.5"),
            (12.5, "12.5"),
            (100_000.0, "100000"),
            (1_000_000.0, "1e+06"),
            (1_234_567.0, "1.23457e+06"),
            (0.0001, "0.0001"),
            (0.000_01, "1e-05"),
            (2.5e-7, "2.5e-07"),
            (0.1 + 0.2, "0.3"),
            (123_456.5, "123456"),
            (99_999.95, "99999.9"),
            (0.0, "0"),
            (-1.5, "-1.5"),
        ] {
            assert_eq!(py_format_g(x), want, "{x}");
        }
    }

    /// An alias spelled as another over-limit payee's GUID gives both entities one tag, so one
    /// figure id twice: the reference raises, and this refuses instead of panicking.
    #[test]
    fn two_payees_sharing_a_figure_id_are_refused() {
        use crate::book::{Ledger, LedgerLine, VoucherStatus};
        const GUID_C: &str = "aaaaaaaa-0000-4000-8000-000000000001";
        let ledger = |name: &str, group: &str, guid: &str| Ledger {
            name: name.to_string(),
            parent: group.to_string(),
            chain: vec![group.to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: guid.to_string(),
            masterid: None,
        };
        let voucher = |guid: &str, payee: &str| Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse("20250610".to_string()).unwrap(),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            number: guid.to_string(),
            status: VoucherStatus::Regular,
            lines: vec![
                LedgerLine {
                    ledger: "Freight".to_string(),
                    amount_paise: 20_000_000,
                },
                LedgerLine {
                    ledger: payee.to_string(),
                    amount_paise: -20_000_000,
                },
            ],
            narration: String::new(),
            ..Default::default()
        };
        let book = Book {
            company_name: "Invented".to_string(),
            company_guid: "invented".to_string(),
            read_at: String::new(),
            groups: BTreeMap::new(),
            group_masters: BTreeMap::new(),
            ledgers: [
                ledger("Freight", "Direct Expenses", ""),
                ledger("Contractor C", "Sundry Creditors", GUID_C),
                ledger("Contractor E", "Sundry Creditors", ""),
            ]
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect(),
            vouchers: vec![voucher("v1", "Contractor C"), voucher("v2", "Contractor E")],
            tb: BTreeMap::new(),
            ..Default::default()
        };
        let mut cfg = TdsConfig::default();
        cfg.nature_by_ledger
            .insert("Freight".to_string(), "194C".to_string());
        cfg.payee_aliases
            .insert("Contractor E".to_string(), GUID_C.to_string());
        let rules = Rules::vendored().unwrap();
        let inputs = Inputs::default();
        let err = run(&book, &rules, "firm", &cfg, &inputs).unwrap_err();
        assert!(
            matches!(&err, AuditError::DuplicateFigureId(id) if id.contains("_row_credited_")),
            "{err}"
        );
        // The control: without the colliding alias both payees are reported.
        cfg.payee_aliases.clear();
        let r = run(&book, &rules, "firm", &cfg, &inputs).unwrap();
        assert_eq!(
            r.findings
                .iter()
                .filter(|f| f.id.starts_with("tds_payees/194C_"))
                .count(),
            2
        );
    }

    /// The reference's `deductor_status` at its current head, row by row: the business limit (1
    /// crore) and the profession limit (50 lakh), each activity, and a placeholder turnover.
    #[test]
    fn deductor_status_is_never_assumed_for_an_individual_or_huf() {
        use DeductorActivity::{Both, Business, Profession};
        let rules = Rules::vendored().unwrap();
        let (crore, lakh50) = (1_000_000_000, 500_000_000);
        for (entity, turnover, activity, placeholder, want) in [
            ("individual", None, None, false, "unknown"),
            ("huf", None, Some(Business), false, "unknown"),
            ("individual", Some(crore + 1), None, false, "deductor"),
            ("individual", Some(crore), None, false, "unknown"),
            ("individual", Some(lakh50 + 1), None, false, "unknown"),
            ("individual", Some(lakh50), None, false, "not_deductor"),
            ("individual", Some(lakh50), None, true, "unknown"),
            (
                "individual",
                Some(crore),
                Some(Business),
                false,
                "not_deductor",
            ),
            (
                "individual",
                Some(crore + 1),
                Some(Business),
                true,
                "deductor",
            ),
            ("huf", Some(lakh50 + 1), Some(Profession), false, "deductor"),
            ("huf", Some(lakh50), Some(Profession), false, "not_deductor"),
            (
                "individual",
                Some(lakh50),
                Some(Both),
                false,
                "not_deductor",
            ),
            ("individual", Some(crore + 1), Some(Both), false, "unknown"),
            ("Individual", None, None, false, "deductor"),
            ("firm", None, None, false, "deductor"),
            ("company", Some(0), None, true, "deductor"),
        ] {
            assert_eq!(
                deductor_status(entity, &rules, turnover, activity, placeholder).unwrap(),
                want,
                "{entity} {turnover:?} {activity:?} {placeholder}"
            );
        }
        // Python's casefold reaches "kerala" through the Kelvin sign and "lakshadweep" through a
        // long s; any other spelling does not.
        assert!(in_kerala_hc("Kerala") && in_kerala_hc("\u{212A}ERALA"));
        assert!(in_kerala_hc("lak\u{017F}hadweep") && !in_kerala_hc("Keralam"));
    }

    /// The config readers refuse what the reference's `tae.config` readers refuse, each by its own
    /// message, and read the well-formed control.
    #[test]
    fn the_config_readers_refuse_what_the_reference_refuses() {
        let read = |tds: &str, tds_payees: &str| {
            let tds: toml::Table = toml::from_str(tds).unwrap();
            let tp: toml::Table = toml::from_str(tds_payees).unwrap();
            read_config_lists(&tds, Some(&tp)).map(|_| ())
        };
        let refused = |tds: &str, tds_payees: &str, message: &str| {
            let err = read(tds, tds_payees).unwrap_err();
            assert!(
                matches!(&err, AuditError::Config(m) if m.contains(message)),
                "{tds} {tds_payees}: {err}"
            );
        };
        refused(
            "",
            "[reversals]\ng1 = { kind = \"undo\" }",
            "g1': kind must be one of",
        );
        refused(
            "",
            "[reversals]\ng1 = \"credit_note\"",
            "g1': kind must be one of",
        );
        refused(
            "",
            "[reversals]\ng1 = { kind = \"bill_specific\" }",
            "kind 'bill_specific' needs the bill's voucher GUID",
        );
        refused(
            "",
            "gst_separate_by_agreement = \"P\"",
            "must be a list of payee names",
        );
        refused("", "[foreseeability]\nP = \"often\"", "'P': must be one of");
        refused("challans = 1", "", "must be an array of tables");
        refused("challans = [1]", "", "entry 0: must be a table");
        let challan = |fields: &str| format!("[[challans]]\n{fields}");
        let ok = "date = 2025-08-07\nsection = \"194C\"\nmonth = \"2025-07\"\namount_paise = 1";
        refused(
            &challan(&ok.replace("2025-08-07", "\"2025-08-07\"")),
            "",
            "date must be a TOML date",
        );
        refused(
            &challan(&ok.replace("2025-08-07", "2025-08-07T10:00:00")),
            "",
            "date must be",
        );
        refused(
            &challan(&ok.replace("194C", "194Q")),
            "",
            "section must be one of",
        );
        for month in ["2025-13", "2025-00", "2025-7", "２０２５-07"] {
            refused(&challan(&ok.replace("2025-07", month)), "", "month must be");
        }
        refused(
            &challan(&ok.replace("= 1", "= 0")),
            "",
            "amount_paise must be a positive",
        );
        refused(
            "[[form_26a]]\npayee = \"P\"",
            "",
            "needs payee (a name) and held",
        );
        refused("[[form_26a]]\npayee = \"\"\nheld = true", "", "needs payee");
        refused(
            "[[form_26a]]\npayee = \"P\"\nheld = true\n[[form_26a]]\npayee = \"P\"\nheld = false",
            "",
            "entry 1: 'P' is listed twice",
        );
        refused(
            "previous_year_turnover_status = \"Placeholder\"",
            "",
            "must be one of",
        );
        // The control: every list well formed is read.
        read(
            &format!(
                "{}\nprevious_year_turnover_status = \"placeholder\"",
                challan(ok)
            ),
            "gst_separate_by_agreement = [\"P\"]\n[reversals]\ng1 = { kind = \"credit_note\" }",
        )
        .unwrap();
        let table = |text: &str| toml::from_str::<toml::Table>(text).unwrap();
        assert!(matches!(
            read_deductor_activity(&table("[deductor]\nactivity = \"trade\"")),
            Err(AuditError::Config(m)) if m.contains("[deductor].activity must be one of")
        ));
        assert_eq!(
            read_deductor_activity(&table("[deductor]\nactivity = \"both\"")).unwrap(),
            Some(DeductorActivity::Both)
        );
        for bad in ["state = \" \\u001C \"", "state = 1"] {
            assert!(matches!(
                read_client_state(&table(bad)),
                Err(AuditError::Config(m)) if m == "[client].state must be the state's name"
            ));
        }
        assert_eq!(
            read_client_state(&table("state = \" Kerala \""))
                .unwrap()
                .as_deref(),
            Some(" Kerala ")
        );
    }

    /// `run` refuses where the reference raises: a reversal naming a bill credited to no payee,
    /// a ledger mapped to 194H with no `[s194h]` rules, and a goods-carriage ledger not mapped to
    /// 194C. The control runs.
    #[test]
    fn run_refuses_a_bill_no_payee_holds_and_a_section_without_its_rules() {
        use crate::book::{Ledger, LedgerLine, VoucherStatus};
        let ledger = |name: &str, group: &str| Ledger {
            name: name.to_string(),
            parent: group.to_string(),
            chain: vec![group.to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: String::new(),
            masterid: None,
        };
        let book = Book {
            company_name: "Invented".to_string(),
            company_guid: "invented".to_string(),
            ledgers: [
                ledger("Freight", "Direct Expenses"),
                ledger("Commission", "Indirect Expenses"),
                ledger("Carrier", "Sundry Creditors"),
            ]
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect(),
            vouchers: vec![Voucher {
                guid: "v1".to_string(),
                date: TallyDate::parse("20250610".to_string()).unwrap(),
                vtype: "Journal".to_string(),
                base_type: "Journal".to_string(),
                number: "1".to_string(),
                status: VoucherStatus::Regular,
                lines: vec![
                    LedgerLine {
                        ledger: "Freight".to_string(),
                        amount_paise: 100,
                    },
                    LedgerLine {
                        ledger: "Carrier".to_string(),
                        amount_paise: -100,
                    },
                ],
                ..Default::default()
            }],
            ..Default::default()
        };
        let rules = Rules::vendored().unwrap();
        let inputs = Inputs::default();
        let mut cfg = TdsConfig::default();
        cfg.nature_by_ledger
            .insert("Freight".to_string(), "194C".to_string());
        run(&book, &rules, "firm", &cfg, &inputs).unwrap();
        let refused = |cfg: &TdsConfig, rules: &Rules, message: &str| {
            let err = run(&book, rules, "firm", cfg, &inputs).unwrap_err();
            assert!(
                matches!(&err, AuditError::Config(m) if m.contains(message)),
                "{err}"
            );
        };
        let mut bad = cfg.clone();
        bad.reversals.insert(
            "r1".to_string(),
            Reversal::DuplicateOrError("v9".to_string()),
        );
        refused(&bad, &rules, "'r1': bill 'v9' is not a credit to any payee");
        // The same reversal naming the credited bill is read (as unmatched: r1 reverses nothing).
        bad.reversals
            .insert("r1".to_string(), Reversal::BillSpecific("v1".to_string()));
        run(&book, &rules, "firm", &bad, &inputs).unwrap();
        let mut bad = cfg.clone();
        bad.nature_by_ledger
            .insert("Commission".to_string(), "194H".to_string());
        run(&book, &rules, "firm", &bad, &inputs).unwrap();
        let no_194h = Rules {
            s194h_aggregate_paise: None,
            ..rules.clone()
        };
        refused(
            &bad,
            &no_194h,
            "mapped to 194H but the rules table has no [s194h]",
        );
        let mut bad = cfg.clone();
        bad.goods_carriage_ledgers.insert("Commission".to_string());
        refused(
            &bad,
            &rules,
            "not mapped to 194C in [tds].nature_by_ledger: ['Commission']",
        );
    }
}
