//! Form 3CD clause 44: the break-up of total expenditure by the GST registration status of the
//! supplier. A port of the reference Python implementation's `clause44` test module, version 2.
//!
//! Every nonzero line in a books-population voucher on a ledger whose own primary group is
//! Purchase Accounts, Direct Expenses or Indirect Expenses is counted once: either in one of the
//! clause's four columns, decided per voucher from its supplier's GSTIN, or under a closed reason
//! outside them (no supplier by nature, a depreciation entry, a money item the CA must judge, no
//! party, or a party that is itself a P&L ledger). A credit on an expense ledger reduces its
//! column.
//!
//! A line is placed by its own ledger before its voucher's party is read, in this order (the first
//! hit wins): the depreciation set, a judgement money category, the no-supplier list, a forced
//! money category (interest to a bank or NBFC to the exempt column, bank charges to "other than
//! composition", whatever the party, a journal with none included). Only a line none of these
//! places is read by the party: no party, a party that is itself a P&L ledger, else the GSTIN rule.
//! A plain round-off line (a round-off ledger with no placement of its own) is shown where the line
//! it rounds is shown: the voucher's first other expenditure line that the party reads, else its
//! first other expenditure line, else (it is alone, or beside only lines that are no expenditure)
//! it is a non-supply expense. It takes that line's column or reason and, when that column was
//! forced, its forced mark.
//!
//! The supplier's GSTIN is the voucher's own `PARTYGSTIN` first, then its party ledger's GSTIN (the
//! master's flat `PARTYGSTIN`, else its registration in force on the period's last day); a
//! `PARTYGSTIN` that is blank once stripped is no GSTIN, so the ledger's is read. Any other text
//! counts as a GSTIN, as in the reference, so "URP" or "NA" reads as registered.
//! Composition status and the money categories are client
//! configuration, never read from a ledger name. Two categories settle the column by statute
//! (interest to a bank or NBFC, bank charges); the rest leave the ordinary rule in place or put
//! the line outside the four columns for the CA.
//!
//! The two client tables are read once, at the boundary ([`Inputs::new`]): a registration type that
//! is not one of Tally's (after strip and case fold) and a money category that is not exactly one
//! of the five are refused, naming the table and the ledger. The reference refuses the same values
//! when it reads the table. It names the first offending entry in the table's written order; this
//! port holds the table sorted by its bound ledger names, so it names the first in that order (a
//! difference only when a table has more than one offending entry, and one no golden can show).
//!
//! The voucher's own PARTYGSTIN is read as the reference reads it, but a populated value has not
//! yet been seen on the wire: the tag is verified present and empty, its populated form is not
//! (`docs/tally/TALLY_PROTOCOL_REFERENCE.md` section 8.2c). The edge books supply populated values.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::{Book, LedgerLine, Voucher, VoucherStatus};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::rules::Rules;
use crate::support;

pub const TEST_ID: &str = "clause44";
pub const VERSION: &str = "2";

const PURCHASE_GROUP: &str = "Purchase Accounts";
const DIRECT_EXPENSES_GROUP: &str = "Direct Expenses";
const INDIRECT_EXPENSES_GROUP: &str = "Indirect Expenses";
const EXPENDITURE_GROUPS: [&str; 3] = [
    PURCHASE_GROUP,
    DIRECT_EXPENSES_GROUP,
    INDIRECT_EXPENSES_GROUP,
];
/// A party ledger under one of these is itself a P&L ledger: no external supplier.
const PL_LEDGER_GROUPS: [&str; 6] = [
    PURCHASE_GROUP,
    DIRECT_EXPENSES_GROUP,
    INDIRECT_EXPENSES_GROUP,
    "Sales Accounts",
    "Direct Incomes",
    "Indirect Incomes",
];

/// The module check's tolerance: Re 1, fired only when exceeded.
const TIE_TOLERANCE_PAISE: i128 = 100;

/// The clause's four columns, in the reference's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bucket {
    Composition,
    OtherRegistered,
    ExemptNonGst,
    Unregistered,
}

pub const BUCKETS: [Bucket; 4] = [
    Bucket::Composition,
    Bucket::OtherRegistered,
    Bucket::ExemptNonGst,
    Bucket::Unregistered,
];

impl Bucket {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Composition => "registered_composition",
            Self::OtherRegistered => "registered_other_than_composition",
            Self::ExemptNonGst => "exempt_or_non_gst_supply",
            Self::Unregistered => "unregistered",
        }
    }

    /// Whether the column requires a supplier GSTIN (CL44-3's re-derivation).
    fn registered(self) -> bool {
        self != Self::Unregistered
    }
}

/// Why a line is outside the four columns: a closed set. `Unclassified` is never produced by
/// [`run`]; CL44-2 requires its count to be zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reason {
    DepreciationLedger,
    NonSupplyExpense,
    JudgementMoneyItem,
    NoParty,
    PartyIsPlLedger,
    Unclassified,
}

pub const REASONS: [Reason; 6] = [
    Reason::DepreciationLedger,
    Reason::NonSupplyExpense,
    Reason::JudgementMoneyItem,
    Reason::NoParty,
    Reason::PartyIsPlLedger,
    Reason::Unclassified,
];

impl Reason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DepreciationLedger => "depreciation_or_provision_expense_ledger",
            Self::NonSupplyExpense => "non_supply_expense",
            Self::JudgementMoneyItem => "judgement_required_money_item",
            Self::NoParty => "no_party_ledger_on_voucher",
            Self::PartyIsPlLedger => "party_ledger_is_itself_a_pl_ledger",
            Self::Unclassified => "unclassified",
        }
    }
}

/// `[clause44].money_category_by_ledger`'s values: exactly one of five spellings, parsed once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoneyCategory {
    /// Interest or discount to a bank or NBFC: forced to the exempt column (N12 entry 27(a)).
    InterestBankNbfc,
    /// Bank charges and fees: forced to the other-registered column (N12 para 2(zk)).
    BankCharges,
    /// Interest to an individual lender with no GSTIN: the ordinary rule, plus a judgement finding.
    InterestIndividualNoGstin,
    /// Partners' interest or remuneration: outside the four columns, for the CA.
    PartnerInterestOrRemuneration,
    /// Interest and charges booked together: outside the four columns, for the CA.
    MixedInterestAndCharges,
}

impl MoneyCategory {
    fn parse(text: &str) -> Option<Self> {
        match text {
            "interest_bank_nbfc" => Some(Self::InterestBankNbfc),
            "bank_charges" => Some(Self::BankCharges),
            "interest_individual_no_gstin" => Some(Self::InterestIndividualNoGstin),
            "partner_interest_or_remuneration" => Some(Self::PartnerInterestOrRemuneration),
            "mixed_interest_and_charges" => Some(Self::MixedInterestAndCharges),
            _ => None,
        }
    }

    fn forced(self) -> Option<Bucket> {
        match self {
            Self::InterestBankNbfc => Some(Bucket::ExemptNonGst),
            Self::BankCharges => Some(Bucket::OtherRegistered),
            _ => None,
        }
    }

    fn is_judgement(self) -> bool {
        matches!(
            self,
            Self::PartnerInterestOrRemuneration | Self::MixedInterestAndCharges
        )
    }
}

/// The test's inputs, typed once from the bound client configuration.
#[derive(Debug, Clone, Default)]
pub struct Inputs {
    /// `[depreciation].dep_expense_ledgers`.
    pub dep_expense_ledgers: BTreeSet<String>,
    /// Party ledgers whose `[roles.gst_registration_type_by_ledger]` value reads "composition"
    /// once stripped and case-folded.
    pub composition_ledgers: BTreeSet<String>,
    /// Every ledger under `[roles].tax_ledgers`, whatever its head.
    pub tax_ledgers: BTreeSet<String>,
    /// `[roles].no_supplier_expense_ledgers`.
    pub no_supplier_expense_ledgers: BTreeSet<String>,
    /// `[roles].round_off_ledgers`.
    pub round_off_ledgers: BTreeSet<String>,
    /// `[clause44].money_category_by_ledger`.
    pub money_category_by_ledger: BTreeMap<String, MoneyCategory>,
}

/// Tally's registration types as a ledger master writes them, stripped and case-folded, and blank.
const REGISTRATION_TYPES: [&str; 7] = [
    "",
    "regular",
    "regular - sez",
    "composition",
    "unregistered",
    "consumer",
    "unregistered/consumer",
];

fn refusal(table: &'static str, ledger: &str) -> AuditError {
    AuditError::ConfigValueRefused {
        table,
        name: ledger.to_string(),
    }
}

impl Inputs {
    /// Type the two ledger maps (keys already bound), refusing the first offending entry of a
    /// table, registration types first, with [`AuditError::ConfigValueRefused`] naming the table
    /// and the ledger, whether or not a line reaches it. A registration type must be text that is
    /// one of [`REGISTRATION_TYPES`] once stripped and case-folded; a money category must be text
    /// that is exactly one of the five (no strip, no case fold, no blank). The reference refuses
    /// the same values, and before this it read a mistyped one as no status or no category.
    pub fn new(
        registration_type_by_ledger: &BTreeMap<String, toml::Value>,
        money_category_by_ledger: &BTreeMap<String, toml::Value>,
    ) -> Result<Self> {
        let mut composition_ledgers = BTreeSet::new();
        for (ledger, value) in registration_type_by_ledger {
            let table = "roles.gst_registration_type_by_ledger";
            let folded = value
                .as_str()
                .map(|text| support::py_casefold(support::py_strip(text)))
                .filter(|folded| REGISTRATION_TYPES.contains(&folded.as_str()))
                .ok_or_else(|| refusal(table, ledger))?;
            if folded == "composition" {
                composition_ledgers.insert(ledger.clone());
            }
        }
        let mut categories = BTreeMap::new();
        for (ledger, value) in money_category_by_ledger {
            let category = value
                .as_str()
                .and_then(MoneyCategory::parse)
                .ok_or_else(|| refusal("clause44.money_category_by_ledger", ledger))?;
            categories.insert(ledger.clone(), category);
        }
        Ok(Self {
            composition_ledgers,
            money_category_by_ledger: categories,
            ..Self::default()
        })
    }
}

/// The reference's `_norm_gstin`: stripped and upper-cased.
fn norm_gstin(text: &str) -> String {
    support::py_upper(support::py_strip(text))
}

fn primary_in(book: &Book, ledger: &str, groups: &[&str]) -> bool {
    book.ledgers
        .get(ledger)
        .and_then(|l| l.chain.last())
        .is_some_and(|primary| groups.contains(&primary.as_str()))
}

fn is_expenditure_line(book: &Book, ledger: &str, amount_paise: i64) -> bool {
    amount_paise != 0 && primary_in(book, ledger, &EXPENDITURE_GROUPS)
}

/// What one line was classified as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    Bucket { bucket: Bucket, forced: bool },
    NoSupplier(Reason),
}

/// One counted line, in walk order.
struct Row<'a> {
    voucher: &'a Voucher,
    ledger: &'a str,
    amount_paise: i64,
    class: Class,
}

/// Everything [`run`] reports. Kept apart from it so [`check_invariants`] can be exercised
/// without calling it.
struct Computed<'a> {
    rows: Vec<Row<'a>>,
    bucket_totals: BTreeMap<Bucket, (usize, i64)>,
    reason_totals: BTreeMap<Reason, (usize, i64)>,
    total_paise: i64,
    /// Expenditure-group lines on post-dated vouchers only, in read order.
    excluded: Vec<(&'a Voucher, i64)>,
    excluded_paise: i64,
    judgement_candidate: (usize, i64),
    forced: (usize, i64),
}

fn add(a: i64, b: i64) -> Result<i64> {
    a.checked_add(b).ok_or_else(|| support::overflow(TEST_ID))
}

fn compute<'a>(book: &'a Book, inputs: &Inputs) -> Result<Computed<'a>> {
    // The voucher's own PARTYGSTIN, unless it is blank once stripped; then its party ledger's.
    let supplier_gstin = |v: &Voucher| -> String {
        let own = norm_gstin(&v.party_gstin);
        if !own.is_empty() {
            return own;
        }
        book.ledgers
            .get(&v.party_field)
            .map_or_else(String::new, |l| norm_gstin(&l.gstin))
    };
    let has_gst = |v: &Voucher| {
        v.lines
            .iter()
            .any(|l| l.amount_paise != 0 && inputs.tax_ledgers.contains(&l.ledger))
    };
    // Where the line's own ledger places it, whatever its voucher's party; `None` when the
    // ledger says nothing and the party reads the line. The order is the precedence.
    let own_placement = |ledger: &str| -> Option<Class> {
        let category = inputs.money_category_by_ledger.get(ledger).copied();
        if inputs.dep_expense_ledgers.contains(ledger) {
            Some(Class::NoSupplier(Reason::DepreciationLedger))
        } else if category.is_some_and(MoneyCategory::is_judgement) {
            Some(Class::NoSupplier(Reason::JudgementMoneyItem))
        } else if inputs.no_supplier_expense_ledgers.contains(ledger) {
            Some(Class::NoSupplier(Reason::NonSupplyExpense))
        } else {
            category
                .and_then(MoneyCategory::forced)
                .map(|bucket| Class::Bucket {
                    bucket,
                    forced: true,
                })
        }
    };
    // A round-off ledger with a placement of its own is placed like any line.
    let plain_round_off =
        |ledger: &str| inputs.round_off_ledgers.contains(ledger) && own_placement(ledger).is_none();
    // What the voucher's party makes of a line its own ledger leaves open.
    let read_by_party = |v: &Voucher| -> Class {
        if v.party_field.is_empty() {
            return Class::NoSupplier(Reason::NoParty);
        }
        if primary_in(book, &v.party_field, &PL_LEDGER_GROUPS) {
            return Class::NoSupplier(Reason::PartyIsPlLedger);
        }
        let bucket = if supplier_gstin(v).is_empty() {
            Bucket::Unregistered
        } else if inputs.composition_ledgers.contains(&v.party_field) {
            Bucket::Composition
        } else if has_gst(v) {
            Bucket::OtherRegistered
        } else {
            Bucket::ExemptNonGst
        };
        Class::Bucket {
            bucket,
            forced: false,
        }
    };

    let mut rows = Vec::new();
    let mut bucket_totals: BTreeMap<Bucket, (usize, i64)> =
        BUCKETS.iter().map(|b| (*b, (0, 0))).collect();
    let mut reason_totals: BTreeMap<Reason, (usize, i64)> =
        REASONS.iter().map(|r| (*r, (0, 0))).collect();
    let mut judgement_candidate = (0usize, 0i64);

    for v in book.population()? {
        let lines: Vec<&LedgerLine> = v
            .lines
            .iter()
            .filter(|l| is_expenditure_line(book, &l.ledger, l.amount_paise))
            .collect();
        let by_party = read_by_party(v);
        let class_of = |ledger: &str| own_placement(ledger).unwrap_or(by_party);
        // A plain round-off line is shown where the line it rounds is shown: the voucher's first
        // other expenditure line that the party reads, else its first other expenditure line,
        // else (nothing to round) a non-supply expense.
        let others: Vec<&&LedgerLine> = lines
            .iter()
            .filter(|l| !plain_round_off(&l.ledger))
            .collect();
        let round_off_class = others
            .iter()
            .find(|l| own_placement(&l.ledger).is_none())
            .or(others.first())
            .map_or(Class::NoSupplier(Reason::NonSupplyExpense), |l| {
                class_of(&l.ledger)
            });
        for l in lines {
            if inputs.money_category_by_ledger.get(&l.ledger)
                == Some(&MoneyCategory::InterestIndividualNoGstin)
            {
                judgement_candidate.0 += 1;
                judgement_candidate.1 = add(judgement_candidate.1, l.amount_paise)?;
            }
            let class = if plain_round_off(&l.ledger) {
                round_off_class
            } else {
                class_of(&l.ledger)
            };
            let (count, paise) = match class {
                Class::NoSupplier(reason) => reason_totals.get_mut(&reason).expect("every reason"),
                Class::Bucket { bucket, .. } => {
                    bucket_totals.get_mut(&bucket).expect("every bucket")
                }
            };
            *count += 1;
            *paise = add(*paise, l.amount_paise)?;
            rows.push(Row {
                voucher: v,
                ledger: &l.ledger,
                amount_paise: l.amount_paise,
                class,
            });
        }
    }

    let mut total_paise = 0i64;
    for (_, paise) in bucket_totals.values().chain(reason_totals.values()) {
        total_paise = add(total_paise, *paise)?;
    }

    // CL44-1's named reconciling item: post-dated vouchers only. The Trial Balance leaves out an
    // optional or cancelled voucher as the books population does, so adding those back would
    // overstate the tie.
    let mut excluded = Vec::new();
    let mut excluded_paise = 0i64;
    for v in book.excluded() {
        if v.status != VoucherStatus::Postdated {
            continue;
        }
        for l in &v.lines {
            if is_expenditure_line(book, &l.ledger, l.amount_paise) {
                excluded.push((v, l.amount_paise));
                excluded_paise = add(excluded_paise, l.amount_paise)?;
            }
        }
    }

    let mut forced = (0usize, 0i64);
    for row in &rows {
        if matches!(row.class, Class::Bucket { forced: true, .. }) {
            forced.0 += 1;
            forced.1 = add(forced.1, row.amount_paise)?;
        }
    }

    Ok(Computed {
        rows,
        bucket_totals,
        reason_totals,
        total_paise,
        excluded,
        excluded_paise,
        judgement_candidate,
        forced,
    })
}

/// The vouchers of the rows `pred` selects, one reference per GUID, labelled by the first row
/// with that GUID, as the reference's `_evidence_for` keeps them.
fn evidence_for(rows: &[Row], pred: impl Fn(&Row) -> bool) -> Vec<EvidenceRef> {
    let mut seen = BTreeSet::new();
    rows.iter()
        .filter(|row| pred(row) && seen.insert(row.voucher.guid.as_str()))
        .map(|row| {
            EvidenceRef::with_label(
                "voucher",
                &row.voucher.guid,
                &support::voucher_label(row.voucher),
            )
        })
        .collect()
}

pub fn run(book: &Book, rules: &Rules, inputs: &Inputs) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.population_note = "Books population (optional, cancelled and post-dated vouchers excluded): \
every line with an amount on a ledger under Purchase Accounts, Direct Expenses or Indirect Expenses \
(the expense groups of the financial statements' profit and loss account), attributed to the \
voucher's own PARTYLEDGERNAME; a credit on such a ledger (a purchase return, a debit note) reduces \
its column. Expenditure-group lines on POST-DATED excluded vouchers are reported separately \
as population_excluded_expense_paise (optional/cancelled vouchers are NOT -- see that figure's own \
definition), never silently dropped from the TB tie (CL44-1)."
        .into();
    let d = compute(book, inputs)?;
    let count = |n: usize| support::count(TEST_ID, n);
    let mut facts: Vec<(String, String)> = Vec::new();

    for bucket in BUCKETS {
        let b = bucket.as_str();
        let (n, paise) = d.bucket_totals[&bucket];
        let evidence = evidence_for(
            &d.rows,
            |row| matches!(row.class, Class::Bucket { bucket: x, .. } if x == bucket),
        );
        let count_id = r.fig(
            &format!("{b}_count"),
            count(n)?,
            Unit::Count,
            &format!("Expense lines classified '{b}'."),
            evidence,
        )?;
        let paise_id = r.fig(
            &format!("{b}_paise"),
            Value::Int(paise),
            Unit::Paise,
            &format!("Sum of expenditure, category '{b}'."),
            Vec::new(),
        )?;
        facts.push((format!("{b}_count"), count_id));
        facts.push((format!("{b}_paise"), paise_id));
    }

    let mut reason_ids: BTreeMap<Reason, (String, String)> = BTreeMap::new();
    for reason in REASONS {
        let rs = reason.as_str();
        let (n, paise) = d.reason_totals[&reason];
        let evidence = evidence_for(&d.rows, |row| row.class == Class::NoSupplier(reason));
        let count_id = r.fig(
            &format!("no_supplier_{rs}_count"),
            count(n)?,
            Unit::Count,
            &format!("Expense lines outside the four GST-registration columns, reason '{rs}'."),
            evidence,
        )?;
        let paise_id = r.fig(
            &format!("no_supplier_{rs}_paise"),
            Value::Int(paise),
            Unit::Paise,
            &format!(
                "Sum of expenditure outside the four GST-registration columns, reason '{rs}'."
            ),
            Vec::new(),
        )?;
        facts.push((format!("no_supplier_{rs}_count"), count_id.clone()));
        facts.push((format!("no_supplier_{rs}_paise"), paise_id.clone()));
        reason_ids.insert(reason, (count_id, paise_id));
    }

    let total_id = r.fig(
        "total_expenditure_paise",
        Value::Int(d.total_paise),
        Unit::Paise,
        "Sum of every bucket and no-supplier/judgement-reason figure above (books-POPULATION \
only); tied in check_invariants (CL44-1), TOGETHER WITH population_excluded_expense_paise below, \
to the financial_statements test's own TB-based expenditure definition (purchases + \
direct_expenses + indirect_expenses), independent of this walk.",
        Vec::new(),
    )?;
    facts.push(("total_expenditure_paise".into(), total_id));

    // One reference per excluded voucher, labelled by its last line's row, sorted by GUID, as the
    // reference's dict comprehension keeps them.
    let mut excluded_by_guid: BTreeMap<&str, &Voucher> = BTreeMap::new();
    for (v, _) in &d.excluded {
        excluded_by_guid.insert(v.guid.as_str(), v);
    }
    let excluded_evidence = excluded_by_guid
        .values()
        .map(|v| EvidenceRef::with_label("excluded_voucher", &v.guid, &support::voucher_label(v)))
        .collect();
    let excluded_count_id = r.fig(
        "population_excluded_expense_count",
        count(d.excluded.len())?,
        Unit::Count,
        "Expenditure-group lines on POST-DATED vouchers (excluded from the books population) -- \
a named reconciling item: the raw TB export is not always population-filtered for a post-dated \
voucher (see check CL44-1), so these must be added back before tying to the TB. Optional and \
cancelled vouchers are NOT included here -- checked on data, the TB excludes both just as the \
books population does, so adding them back would overstate this figure.",
        excluded_evidence,
    )?;
    let excluded_paise_id = r.fig(
        "population_excluded_expense_paise",
        Value::Int(d.excluded_paise),
        Unit::Paise,
        "Sum of expenditure-group lines on post-dated, population-excluded vouchers.",
        Vec::new(),
    )?;
    facts.push(("post_dated_voucher_expense_count".into(), excluded_count_id));
    facts.push(("post_dated_voucher_expense_paise".into(), excluded_paise_id));

    // Overlays: these lines are already inside total_expenditure_paise through their own bucket.
    let forced_evidence = evidence_for(&d.rows, |row| {
        matches!(row.class, Class::Bucket { forced: true, .. })
    });
    r.fig(
        "money_category_forced_lines_count",
        count(d.forced.0)?,
        Unit::Count,
        "Population expenditure lines whose column was FORCED by a client-configured \
money-category classification (interest or discount to a bank/NBFC, or bank charges/fees) rather \
than derived from the voucher's own GSTIN/tax-line signals -- CL44-3 excludes every voucher named \
here from its own GSTIN-precedence re-derivation (module docstring).",
        forced_evidence,
    )?;
    r.fig(
        "money_category_forced_lines_paise",
        Value::Int(d.forced.1),
        Unit::Paise,
        "Sum of the lines above; already included in total_expenditure_paise via its own forced \
bucket, not a separate addition.",
        Vec::new(),
    )?;

    let judgement_evidence = evidence_for(&d.rows, |row| {
        inputs.money_category_by_ledger.get(row.ledger)
            == Some(&MoneyCategory::InterestIndividualNoGstin)
    });
    let judgement_count_id = r.fig(
        "interest_individual_no_gstin_lines_count",
        count(d.judgement_candidate.0)?,
        Unit::Count,
        "Population expenditure lines on a ledger classified as interest to an individual \
lender with no GSTIN, counted through the ordinary GSTIN-derived bucket rule above (normally \
'unregistered') -- not pulled out.",
        judgement_evidence.clone(),
    )?;
    let judgement_paise_id = r.fig(
        "interest_individual_no_gstin_lines_paise",
        Value::Int(d.judgement_candidate.1),
        Unit::Paise,
        "Sum of the lines above; already included in total_expenditure_paise via its own bucket, \
not a separate addition.",
        Vec::new(),
    )?;

    r.findings.push(Finding {
        id: format!("{TEST_ID}/breakup"),
        clauses: vec!["3CD-44".into()],
        title: "Break-up of total expenditure by GST registration status of the supplier".into(),
        facts,
        evidence: evidence_for(&d.rows, |_| true),
        confidence: Confidence::Indicative,
        limits: vec![
            "A voucher's party field names only the first party on a multi-party voucher (same \
limit as the GST-ITC-vs-2B test); clause 44 is defined per voucher here, so a voucher with more \
than one true counterparty is attributed wholly to its named party."
                .into(),
            "'Exempt or non-GST supply' is a books-only indicator for a GSTIN-derived row (GSTIN \
present, no input-tax line on the voucher); the books cannot show whether the supply was exempt, \
nil-rated, non-GST-rated, or simply a registered supplier who did not charge GST it should have. A \
row FORCED into this column because it is interest or discount paid to a bank or NBFC is not a \
proxy: it is exempt by N12 entry 27(a) regardless of what the voucher's own GSTIN/tax-line \
signals show."
                .into(),
            "Composition status comes from a caller-supplied ledger map read off the ledger \
master's own GSTREGISTRATIONTYPE; a ledger missing from that map is treated as not composition \
(the overwhelmingly common case). A composition supplier missing from it is then shown under \
'other than composition' when the voucher carries input tax and under 'exempt or non-GST supply' \
when it carries none, as a composition dealer's bill does: confirm the list of composition \
suppliers."
                .into(),
            "Total expenditure is NOT reduced for depreciation or other non-cash items -- GN 79.2: \
depreciation and bad debts are not divided across columns 3 to 7 but stay 'included in the amount \
reported in column 2'."
                .into(),
            "The expense ledgers treated as having no GST supplier by nature (salary/wages to \
employees, interest/late fees on taxes and similar) are listed for this client; a ledger of that \
kind not on the list is classified by its GSTIN instead and can appear as 'unregistered' rather \
than 'no supplier'. Review the list."
                .into(),
            "Money-category classification (2026-09-17 correction, from our review of clause \
44's money items): interest/discount to a bank or NBFC is FORCED to column 3 (N12 entry 27(a), \
exempt) and bank charges/fees are FORCED to column 5 (N12 para 2(zk), taxable), whatever the \
voucher's party (a journal with none included) -- a ledger of either kind not on that list instead falls through to the ordinary GSTIN-derived path, which can \
misclassify it (e.g. a bank ledger with no GSTIN in the master would otherwise read \
'unregistered'). A ledger mixing interest and charges that cannot be shown separately from the \
books, and partners' interest/remuneration, are NEVER placed in column 3 or 5 -- see the \
judgement finding below."
                .into(),
        ],
        ask_client: vec![
            "Confirmation of GST registration status (regular / composition / unregistered) for \
every supplier in the 'unregistered' and 'exempt or non-GST supply' columns."
                .into(),
            "For any row where the PARTY LEDGER itself carries no GSTIN and registration rests \
entirely on the voucher's own recorded PARTYGSTIN (a cash-party purchase, most visibly on \
GTA/freight-under-RCM ledgers, where an unregistered transporter is common) -- confirm that \
transaction GSTIN is a genuine, currently-active registration for that specific supplier; the \
books alone cannot verify it."
                .into(),
        ],
    });

    if d.judgement_candidate.0 > 0 {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/interest_individual_no_gstin_candidate"),
            clauses: vec!["3CD-44".into()],
            title: "Interest to an individual lender with no GSTIN: whether this is even a GST \
supply is not settled"
                .into(),
            facts: vec![
                ("count".into(), judgement_count_id),
                ("amount".into(), judgement_paise_id),
            ],
            evidence: judgement_evidence,
            confidence: Confidence::JudgementRequired,
            limits: vec![
                "These lines are counted where the voucher's party puts them: in the 'unregistered' \
column above (the conservative default) when that party has no GSTIN, and under 'no party ledger \
on voucher' on a journal that names none. Whether a non-business individual lender's interest \
is a 'supply' at all under s.7(1)(a) is not settled from primary text -- if it is not a supply, \
the correct clause-44 treatment is to leave the item out of the base entirely, not report it as an \
unregistered supply."
                    .into(),
                "Source: our review of clause 44's money items, Q2 (row: interest on unsecured \
loans from individuals/relatives/directors), confidence S/U."
                    .into(),
            ],
            ask_client: vec![
                "Confirm whether these lenders extend this interest in the course or furtherance \
of a business (a 'supply') or purely as an individual, and disclose the treatment taken (GN \
79.21)."
                    .into(),
            ],
        });
    }

    let judgement_money = Reason::JudgementMoneyItem;
    if d.reason_totals[&judgement_money].0 > 0 {
        let (count_id, paise_id) = reason_ids[&judgement_money].clone();
        r.findings.push(Finding {
            id: format!("{TEST_ID}/judgement_money_item_candidate"),
            clauses: vec!["3CD-44".into()],
            title: "Partner interest/remuneration or a mixed interest-and-charges ledger: clause \
44 treatment is a CA call, never column 3 or 5"
                .into(),
            facts: vec![("count".into(), count_id), ("amount".into(), paise_id)],
            evidence: evidence_for(&d.rows, |row| {
                row.class == Class::NoSupplier(judgement_money)
            }),
            confidence: Confidence::JudgementRequired,
            limits: vec![
                "Interest on a partner's capital and a partner's remuneration are not plainly \
either an exempt supply (N12 27(a)) or a Schedule III item (a partner is not plainly an \
'employee') -- s.7(1)(aa) deems a firm and its partners separate persons, but no circular \
addresses this specifically. A ledger that books both interest and charges together cannot be \
divided between column 3 and column 5 from the books alone. Neither is placed in any of the four \
registration columns here -- the CA must pick one treatment (column 7 or excluded from the base) \
and disclose it (GN 79.21)."
                    .into(),
                "Source: our review of clause 44's money items, Q2 (row: interest on partners' \
capital) and the final table's own instruction to show a ledger mixing interest and charges \
separately, both confidence U."
                    .into(),
            ],
            ask_client: vec![
                "Confirm and disclose the treatment taken for partner interest/remuneration for \
clause 44, and identify whether any listed ledger's interest and charges can be shown \
separately."
                    .into(),
            ],
        });
    }

    Ok(r)
}

/// CL44-1..CL44-3, independent of [`run`]'s walk: the total is re-derived from the Trial Balance
/// through `financial_statements`, and each cited voucher's GSTIN straight from the book.
pub fn check_invariants(book: &Book, result: &TestResult) -> Result<Vec<String>> {
    let figure = |name: &str| {
        let id = format!("{}.{name}", result.test_id);
        result.figures.iter().find(|f| f.id == id)
    };
    let value = |name: &str| -> Option<i128> {
        figure(name).map(|f| match f.value {
            Value::Int(n) => i128::from(n),
            _ => 0,
        })
    };
    let mut out = Vec::new();

    // CL44-1: the columns sum to the total, and the total with the post-dated item ties the TB.
    let mut col_sum = 0i128;
    for b in BUCKETS {
        col_sum += value(&format!("{}_paise", b.as_str())).unwrap_or(0);
    }
    for rs in REASONS {
        col_sum += value(&format!("no_supplier_{}_paise", rs.as_str())).unwrap_or(0);
    }
    if let Some(total) = value("total_expenditure_paise") {
        if (col_sum - total).abs() > TIE_TOLERANCE_PAISE {
            out.push(format!(
                "CL44-1: published columns sum ({col_sum}p) != total_expenditure_paise \
({total}p); difference {}p",
                col_sum - total
            ));
        }
        let excluded = value("population_excluded_expense_paise").unwrap_or(0);
        let (fs, _) = crate::financial_statements::compute(book, &BTreeSet::new())?;
        let fs_expenditure = i128::from(fs.purchases)
            + i128::from(fs.direct_expenses)
            + i128::from(fs.indirect_expenses);
        let reconciled = total + excluded;
        if (reconciled - fs_expenditure).abs() > TIE_TOLERANCE_PAISE {
            out.push(format!(
                "CL44-1: total_expenditure_paise ({total}p) + population_excluded_expense_paise \
({excluded}p) = {reconciled}p does not tie the financial_statements expenditure definition \
(purchases + direct_expenses + indirect_expenses = {fs_expenditure}p) within \
{TIE_TOLERANCE_PAISE}p; unexplained difference {}p",
                reconciled - fs_expenditure
            ));
        }
    }

    // CL44-2: no line went unclassified.
    let unclassified = value(&format!(
        "no_supplier_{}_count",
        Reason::Unclassified.as_str()
    ));
    if let Some(n) = unclassified.filter(|n| *n != 0) {
        out.push(format!(
            "CL44-2: {n} expense line(s) have no classification reason"
        ));
    }

    // CL44-3: each column's cited vouchers, but those whose column was forced by statute, agree
    // with a fresh transaction-first, ledger-second GSTIN lookup. The last voucher with a GUID
    // answers for it, as the reference's index keeps it.
    let mut by_guid: BTreeMap<&str, &Voucher> = BTreeMap::new();
    for v in &book.vouchers {
        by_guid.insert(v.guid.as_str(), v);
    }
    let forced: BTreeSet<&str> = figure("money_category_forced_lines_count")
        .map(|f| f.evidence.iter().map(|e| e.id.as_str()).collect())
        .unwrap_or_default();
    for bucket in BUCKETS {
        let name = format!("{}_count", bucket.as_str());
        let Some(f) = figure(&name) else {
            continue;
        };
        for e in &f.evidence {
            if e.kind != "voucher" || forced.contains(e.id.as_str()) {
                continue;
            }
            let Some(v) = by_guid.get(e.id.as_str()) else {
                out.push(format!(
                    "CL44-3: cannot resolve voucher {} (figure {})",
                    e.id, f.id
                ));
                continue;
            };
            let mut gstin = norm_gstin(&v.party_gstin);
            if gstin.is_empty() {
                if let Some(l) = book.ledgers.get(&v.party_field) {
                    gstin = norm_gstin(&l.gstin);
                }
            }
            if bucket == Bucket::Unregistered && !gstin.is_empty() {
                out.push(format!(
                    "CL44-3: voucher {} classified 'unregistered' but carries a \
transaction/ledger GSTIN ({gstin}) on fresh re-derivation",
                    e.id
                ));
            }
            if bucket.registered() && gstin.is_empty() {
                out.push(format!(
                    "CL44-3: voucher {} classified '{}' but has no GSTIN on fresh \
re-derivation",
                    e.id,
                    bucket.as_str()
                ));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{Ledger, LedgerLine, TbRow};
    use bridge_tally_primitives::TallyDate;

    fn text(map: &[(&str, toml::Value)]) -> BTreeMap<String, toml::Value> {
        map.iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect()
    }

    const REGISTRATION_TABLE: &str = "roles.gst_registration_type_by_ledger";
    const MONEY_TABLE: &str = "clause44.money_category_by_ledger";

    /// The table and the ledger a refusal names, read from the typed variant.
    fn refused(err: AuditError) -> (&'static str, String) {
        match err {
            AuditError::ConfigValueRefused { table, name } => (table, name),
            other => panic!("not a refused config value: {other}"),
        }
    }

    fn not_text() -> Vec<toml::Value> {
        vec![
            toml::Value::Integer(0),
            toml::Value::Float(1.5),
            toml::Value::Boolean(true),
            toml::Value::Array(vec!["composition".into()]),
            toml::Value::Table(toml::map::Map::new()),
        ]
    }

    #[test]
    fn a_registration_type_that_is_not_one_of_tallys_is_refused_naming_the_ledger() {
        let none = BTreeMap::new();
        let mistyped: Vec<toml::Value> = [
            "composition scheme",
            "Composit ion",
            "Regular Dealer",
            "unknown",
            "regular\u{0}",
            "regular - sez.",
        ]
        .iter()
        .map(|t| (*t).into())
        .chain(not_text())
        .collect();
        for value in mistyped {
            let err = Inputs::new(&text(&[("Beta", value.clone())]), &none).unwrap_err();
            assert_eq!(
                refused(err),
                (REGISTRATION_TABLE, "Beta".to_string()),
                "{value:?}"
            );
        }
    }

    /// Every registration type Tally writes is accepted once stripped and case-folded, blank
    /// included; only a composition one is a composition supplier. "Unregi\u{17f}tered" folds to
    /// "unregistered" (a case fold, not a lower-casing).
    #[test]
    fn each_registration_type_is_accepted_stripped_and_case_folded() {
        let inputs = Inputs::new(
            &text(&[
                ("A", " Composition\t".into()),
                ("B", "\u{a0}COMPOSITION".into()),
                ("C", "Regular".into()),
                ("D", "".into()),
                ("E", "  ".into()),
                ("F", " REGULAR - SEZ\t".into()),
                ("G", "Unregi\u{17f}tered".into()),
                ("H", "CONSUMER".into()),
                ("I", " Unregistered/Consumer ".into()),
            ]),
            &BTreeMap::new(),
        )
        .unwrap();
        assert_eq!(
            inputs.composition_ledgers,
            BTreeSet::from(["A".to_string(), "B".to_string()])
        );
    }

    #[test]
    fn a_money_category_that_is_not_exactly_one_of_the_five_is_refused_naming_the_ledger() {
        let none = BTreeMap::new();
        let mistyped: Vec<toml::Value> = [
            " bank_charges",
            "bank_charges ",
            "Bank_Charges",
            "INTEREST_BANK_NBFC",
            "",
            "courier_fees",
            "bank charges",
        ]
        .iter()
        .map(|t| (*t).into())
        .chain(not_text())
        .collect();
        for value in mistyped {
            let err = Inputs::new(&none, &text(&[("Fees", value.clone())])).unwrap_err();
            assert_eq!(refused(err), (MONEY_TABLE, "Fees".to_string()), "{value:?}");
        }
    }

    #[test]
    fn each_of_the_five_money_categories_is_accepted_as_written() {
        let inputs = Inputs::new(
            &BTreeMap::new(),
            &text(&[
                ("A", "interest_bank_nbfc".into()),
                ("B", "bank_charges".into()),
                ("C", "interest_individual_no_gstin".into()),
                ("D", "partner_interest_or_remuneration".into()),
                ("E", "mixed_interest_and_charges".into()),
            ]),
        )
        .unwrap();
        assert_eq!(
            inputs.money_category_by_ledger,
            BTreeMap::from([
                ("A".to_string(), MoneyCategory::InterestBankNbfc),
                ("B".to_string(), MoneyCategory::BankCharges),
                ("C".to_string(), MoneyCategory::InterestIndividualNoGstin),
                (
                    "D".to_string(),
                    MoneyCategory::PartnerInterestOrRemuneration
                ),
                ("E".to_string(), MoneyCategory::MixedInterestAndCharges),
            ])
        );
    }

    /// The first offending entry of a table names the refusal; the registration table is read
    /// before the money one, as the reference's pack reads them.
    #[test]
    fn the_first_offending_entry_is_the_one_named_and_registration_types_come_first() {
        let registration = text(&[
            ("A", "Regular".into()),
            ("B", "Composit".into()),
            ("C", toml::Value::Integer(5)),
        ]);
        let money = text(&[
            ("P", "bank_charges".into()),
            ("Q", toml::Value::Integer(5)),
            ("R", "Bank_Charges".into()),
        ]);
        assert_eq!(
            refused(Inputs::new(&registration, &money).unwrap_err()),
            (REGISTRATION_TABLE, "B".to_string())
        );
        let sound = text(&[("A", "Regular".into())]);
        assert_eq!(
            refused(Inputs::new(&sound, &money).unwrap_err()),
            (MONEY_TABLE, "Q".to_string())
        );
    }

    /// One purchase of 1,000 on Rent from a supplier with no GSTIN, tied in the Trial Balance.
    fn book() -> Book {
        let ledger = |name: &str, group: &str| Ledger {
            name: name.to_string(),
            parent: group.to_string(),
            chain: vec![group.to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: format!("guid-{name}"),
            masterid: None,
        };
        let row = |closing: i64| TbRow {
            opening_paise: 0,
            debit_paise: closing.max(0),
            credit_paise: (-closing).max(0),
            closing_paise: closing,
        };
        Book {
            ledgers: [
                ledger("Rent", INDIRECT_EXPENSES_GROUP),
                ledger("Landlord", "Sundry Creditors"),
            ]
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect(),
            vouchers: vec![Voucher {
                guid: "v1".to_string(),
                date: TallyDate::parse("20250601").unwrap(),
                vtype: "Purchase".to_string(),
                base_type: "Purchase".to_string(),
                status: VoucherStatus::Regular,
                party_field: "Landlord".to_string(),
                lines: vec![
                    LedgerLine {
                        ledger: "Rent".to_string(),
                        amount_paise: 100_000,
                    },
                    LedgerLine {
                        ledger: "Landlord".to_string(),
                        amount_paise: -100_000,
                    },
                ],
                ..Default::default()
            }],
            tb: BTreeMap::from([
                ("Rent".to_string(), row(100_000)),
                ("Landlord".to_string(), row(-100_000)),
            ]),
            ..Default::default()
        }
    }

    fn result(book: &Book) -> TestResult {
        run(book, &Rules::vendored().unwrap(), &Inputs::default()).unwrap()
    }

    fn set(r: &mut TestResult, name: &str, value: i64) {
        let id = format!("{TEST_ID}.{name}");
        r.figures.iter_mut().find(|f| f.id == id).unwrap().value = Value::Int(value);
    }

    /// The party ledger's own GSTIN is read stripped, as the reference reads it (measured on it):
    /// one that is blank once stripped is no GSTIN, so the supplier is unregistered; a padded one
    /// is a GSTIN, so the purchase with no tax line is exempt or non-GST.
    #[test]
    fn the_party_ledgers_gstin_is_read_stripped() {
        let count = |gstin: &str, column: &str| {
            let mut b = book();
            b.ledgers.get_mut("Landlord").unwrap().gstin = gstin.to_string();
            let id = format!("{TEST_ID}.{column}_count");
            let r = result(&b);
            r.figures.iter().find(|f| f.id == id).unwrap().value.clone()
        };
        assert_eq!(count("  \u{a0} ", "unregistered"), Value::Int(1));
        assert_eq!(
            count("  \u{a0} ", "exempt_or_non_gst_supply"),
            Value::Int(0)
        );
        assert_eq!(count(" 27landl0000a1y9 ", "unregistered"), Value::Int(0));
        assert_eq!(
            count(" 27landl0000a1y9 ", "exempt_or_non_gst_supply"),
            Value::Int(1)
        );
    }

    /// A ledger in both the depreciation set and the no-supplier list is a depreciation line, as
    /// the reference places it (measured on it): the depreciation set is read first.
    #[test]
    fn a_ledger_in_the_depreciation_set_and_the_no_supplier_list_is_depreciation() {
        let b = book();
        let rent = BTreeSet::from(["Rent".to_string()]);
        let inputs = Inputs {
            dep_expense_ledgers: rent.clone(),
            no_supplier_expense_ledgers: rent,
            ..Inputs::default()
        };
        let r = run(&b, &Rules::vendored().unwrap(), &inputs).unwrap();
        let count = |reason: &str| {
            let id = format!("{TEST_ID}.no_supplier_{reason}_count");
            r.figures.iter().find(|f| f.id == id).unwrap().value.clone()
        };
        assert_eq!(
            count("depreciation_or_provision_expense_ledger"),
            Value::Int(1)
        );
        assert_eq!(count("non_supply_expense"), Value::Int(0));
    }

    #[test]
    fn the_module_check_holds_on_the_tests_own_result() {
        let b = book();
        assert_eq!(
            check_invariants(&b, &result(&b)).unwrap(),
            Vec::<String>::new()
        );
    }

    /// A published total off its columns by more than Re 1 fires CL44-1 twice: once against the
    /// columns, once against the Trial Balance; off by exactly Re 1, neither.
    #[test]
    fn cl44_1_fires_past_re_1_between_the_columns_and_the_total() {
        let b = book();
        let mut r = result(&b);
        set(&mut r, "total_expenditure_paise", 100_100);
        assert_eq!(check_invariants(&b, &r).unwrap(), Vec::<String>::new());
        set(&mut r, "total_expenditure_paise", 100_101);
        assert_eq!(
            check_invariants(&b, &r).unwrap(),
            vec![
                "CL44-1: published columns sum (100000p) != total_expenditure_paise (100101p); \
difference -101p"
                    .to_string(),
                "CL44-1: total_expenditure_paise (100101p) + population_excluded_expense_paise \
(0p) = 100101p does not tie the financial_statements expenditure definition (purchases + \
direct_expenses + indirect_expenses = 100000p) within 100p; unexplained difference 101p"
                    .to_string(),
            ]
        );
    }

    /// A total past i64 refuses, naming the test, rather than wrapping or panicking.
    #[test]
    fn an_overflowing_total_is_refused() {
        let mut b = book();
        let lines = &mut b.vouchers[0].lines;
        lines[0].amount_paise = i64::MAX;
        lines.push(LedgerLine {
            ledger: "Rent".to_string(),
            amount_paise: 1,
        });
        let err = run(&b, &Rules::vendored().unwrap(), &Inputs::default()).unwrap_err();
        assert!(
            matches!(&err, AuditError::Config(m) if m == "clause44: a total overflowed i64 paise"),
            "{err}"
        );
    }

    #[test]
    fn cl44_2_fires_on_any_unclassified_line() {
        let b = book();
        let mut r = result(&b);
        set(&mut r, "no_supplier_unclassified_count", 1);
        assert_eq!(
            check_invariants(&b, &r).unwrap(),
            vec!["CL44-2: 1 expense line(s) have no classification reason".to_string()]
        );
    }

    #[test]
    fn cl44_3_names_a_cited_voucher_the_book_does_not_hold() {
        let b = book();
        let mut r = result(&b);
        let id = format!("{TEST_ID}.unregistered_count");
        let f = r.figures.iter_mut().find(|f| f.id == id).unwrap();
        f.evidence.push(EvidenceRef::new("voucher", "missing"));
        assert_eq!(
            check_invariants(&b, &r).unwrap(),
            vec![
                "CL44-3: cannot resolve voucher missing (figure clause44.unregistered_count)"
                    .to_string()
            ]
        );
    }
}
