// SPDX-License-Identifier: Apache-2.0
//! GST ledgers by duty head, and the books' monthly outward and reverse-charge figures for a
//! comparison with GSTR-3B. A port of the reference Python implementation's `gst_ledgers_3b`
//! test module, version 1, for an engagement with **no GSTR-3B document**.
//!
//! Per duty head (client configuration: `[roles].tax_ledgers`, never a ledger name), every
//! nonzero population line on a configured tax ledger is put in one bucket by its voucher's
//! category ([`classify_voucher`]): output, input, payment or other. Opening and closing come from
//! the Trial Balance, and the unexplained residual (closing - opening - the four buckets, raw sign)
//! is reported even when zero. Each month the books' output tax (tax lines on output vouchers),
//! taxable turnover (Sales Accounts ledgers' movement), the old residual taxable value
//! ([`crate::gst_outward_books`]) and reverse-charge tax (`[roles].rcm_tax_ledgers`) are figures;
//! the goods-transport question is asked on `[tds].goods_carriage_ledgers` payments with no duty
//! line.
//!
//! **Scope.** With no GSTR-3B row, the reference classifies every month
//! `no_3b_document_for_month` before it reads GSTR-1 rows or the round-off movement
//! (`gst_ledgers_3b.py:355-357` outward, `:202-203` reverse charge), so its dump depends on the
//! book and the roles above alone. [`Inputs`] holds no GSTR-3B or GSTR-1 row, and [`run`] takes a
//! [`NoGstr3b`], which only [`no_gstr3b`] makes, after refusing an engagement or caller that holds
//! a GSTR-3B (`AuditError::DocumentNotPorted`) and an engagement whose `[documents]` is not a
//! table (`AuditError::Config`). The monthly tie to GSTR-3B is a later port.
//!
//! **One owner of the category.** [`classify_voucher`] is this crate's only reading of a
//! voucher as input, output or payment for GST. The one other reading of base type for GST,
//! `crate::gst_outward_books`' Sales/Credit Note/Debit Note population, is blind to direction (a
//! purchase-side note is in it, as in the reference) and feeds only the old residual figure.
//! Nothing here attributes GST to a counter-ledger, and no figure is per ledger beyond the duty
//! head each tax ledger is configured under.
//!
//! `check_invariants` is LEDGER-1 (each head's figures tie to a fresh Trial Balance read),
//! LEDGER-2 (each monthly series' reason counts sum to its months) and LEDGER-3 (no month
//! unclassified), `:736-800`.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::{Book, Voucher};
use crate::error::{AuditError, Result};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::gst_outward_books::{books_outward_rows, year_month};
use crate::ledger_ids::stable_ledger_tag;
use crate::read::{iso, Window};
use crate::registry::CallerData;
use crate::rules::Rules;
use crate::support;
use crate::Engagement;

pub const TEST_ID: &str = "gst_ledgers_3b";
pub const VERSION: &str = "1";

/// LEDGER-1's tolerance, Re 1 (`TOL_PAISE`, :127).
const TOL_PAISE: i128 = 100;

const SALES_GROUP: &str = "Sales Accounts";
const PURCHASE_GROUP: &str = "Purchase Accounts";
const DUTIES_GROUP: &str = "Duties & Taxes";

const NO_3B: &str = "no_3b_document_for_month";
const UNCLASSIFIED: &str = "unclassified";
/// The outward closed set, in the reference's order (`OUTWARD_REASONS`, :154).
const OUTWARD_REASONS: [&str; 8] = [
    "within_tolerance",
    "timing_invoice_reported_other_month",
    "credit_note_reported_other_month",
    "amendment_after_filing",
    "non_gst_charges_on_invoice",
    NO_3B,
    "unreconciled",
    UNCLASSIFIED,
];
/// The reverse-charge closed set (`RCM_REASONS`, :158).
const RCM_REASONS: [&str; 6] = [
    "within_tolerance",
    "month_shift_indicative",
    NO_3B,
    "unreconciled",
    "not_computed_no_rcm_ledger",
    UNCLASSIFIED,
];

/// A voucher's GST category (`CAT_*`, :138).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Output,
    Input,
    Payment,
    Other,
}

/// The test's inputs, from the bound client configuration.
#[derive(Debug, Clone, Default)]
pub struct Inputs {
    /// `[roles].tax_ledgers` flattened: ledger -> GST duty head.
    pub tax_ledgers: BTreeMap<String, String>,
    /// `[roles].payable_ledgers`.
    pub payable_ledgers: BTreeSet<String>,
    /// `[roles].rcm_tax_ledgers`.
    pub rcm_tax_ledgers: BTreeSet<String>,
    /// `[roles].round_off_ledgers`.
    pub round_off_ledgers: BTreeSet<String>,
    /// `[tds].goods_carriage_ledgers`.
    pub goods_carriage_ledgers: BTreeSet<String>,
}

/// Proof that no GSTR-3B reaches this test: [`run`] requires one, and only [`no_gstr3b`] makes one,
/// from an engagement and its caller's data that hold none.
#[derive(Debug, Clone, Copy)]
pub struct NoGstr3b(());

/// Refuse a GSTR-3B, or give the proof that there is none. A GSTR-3B is named in the engagement's
/// `[documents].gstr3b`, or held by the caller ([`CallerData::holds_gstr3b`]). The comparison with
/// one is not ported, and the reference reads its rows into every month's figures
/// (`gst_ledgers_3b.py:486-498`), so the test refuses rather than give a part of its result.
///
/// It refuses more than the reference, and so fails closed: the reference reads a document only
/// when `[documents].gstr3b` is truthy (`config.py:93-100`), so `false`, `0`, `[]` or `{}` is no
/// document there and refused here (only an empty string is no document here); and a GSTR-3B
/// loaded with no row in the period gives the reference's no-GSTR-3B dump (`pack.py:496-498`),
/// refused here too. A `[documents]` that is not a table ends the reference's run (`.get` on it
/// raises), and is refused here (`AuditError::Config`).
pub fn no_gstr3b(engagement: &Engagement, caller: &CallerData) -> Result<NoGstr3b> {
    let named = match engagement.raw_cfg.get("documents") {
        None => false,
        Some(toml::Value::Table(documents)) => documents
            .get("gstr3b")
            .is_some_and(|v| v.as_str() != Some("")),
        Some(_) => {
            return Err(AuditError::Config(
                "client config [documents] is not a table".to_string(),
            ))
        }
    };
    if named || caller.holds_gstr3b() {
        return Err(AuditError::DocumentNotPorted {
            test: TEST_ID,
            document: "GSTR-3B",
        });
    }
    Ok(NoGstr3b(()))
}

/// The reference's `_classify_voucher` (:224-250), by base type, and for a note by the groups of
/// its lines (any line, nil or not): output with a Sales Accounts line and no Purchase Accounts
/// line, input the other way round, other when both or neither. A Journal is a payment (a set-off
/// or reclass) when every line off the tax ledgers is a payable ledger or under Duties & Taxes,
/// none included, and input otherwise (tax booked against an expense, as an RCM self-invoice).
pub fn classify_voucher(v: &Voucher, book: &Book, inputs: &Inputs) -> Category {
    let under = |name: &str, group: &str| book.ledgers.get(name).is_some_and(|l| l.under(group));
    match v.base_type.as_str() {
        "Sales" => Category::Output,
        "Purchase" => Category::Input,
        "Credit Note" | "Debit Note" => {
            let sales = v.lines.iter().any(|l| under(&l.ledger, SALES_GROUP));
            let purchase = v.lines.iter().any(|l| under(&l.ledger, PURCHASE_GROUP));
            match (sales, purchase) {
                (true, false) => Category::Output,
                (false, true) => Category::Input,
                _ => Category::Other,
            }
        }
        "Payment" | "Receipt" => Category::Payment,
        "Journal" => {
            let reclass = v
                .lines
                .iter()
                .filter(|l| !inputs.tax_ledgers.contains_key(&l.ledger))
                .all(|l| {
                    inputs.payable_ledgers.contains(&l.ledger) || under(&l.ledger, DUTIES_GROUP)
                });
            if reclass {
                Category::Payment
            } else {
                Category::Input
            }
        }
        _ => Category::Other,
    }
}

/// One duty head's raw-sign buckets and Trial Balance sums (`_head_buckets`, :253-271).
#[derive(Debug, Default)]
struct Head {
    output: i128,
    input: i128,
    payment: i128,
    other: i128,
    opening: i128,
    closing: i128,
    ledgers: Vec<String>,
}

/// A voucher's nonzero lines on the configured tax ledgers (:260, :314).
fn tax_lines<'a>(
    v: &'a Voucher,
    inputs: &'a Inputs,
) -> impl Iterator<Item = &'a crate::book::LedgerLine> {
    v.lines
        .iter()
        .filter(|l| l.amount_paise != 0 && inputs.tax_ledgers.contains_key(&l.ledger))
}

fn head_buckets(book: &Book, pop: &[&Voucher], inputs: &Inputs) -> BTreeMap<String, Head> {
    let mut heads: BTreeMap<String, Head> = inputs
        .tax_ledgers
        .values()
        .map(|h| (h.clone(), Head::default()))
        .collect();
    for v in pop {
        if tax_lines(v, inputs).next().is_none() {
            continue;
        }
        let cat = classify_voucher(v, book, inputs);
        for l in tax_lines(v, inputs) {
            let d = heads
                .get_mut(&inputs.tax_ledgers[&l.ledger])
                .expect("every head is seeded");
            let amount = i128::from(l.amount_paise);
            match cat {
                Category::Output => d.output += amount,
                Category::Input => d.input += amount,
                Category::Payment => d.payment += amount,
                Category::Other => d.other += amount,
            }
        }
    }
    for (name, head) in &inputs.tax_ledgers {
        let d = heads.get_mut(head).expect("every head is seeded");
        if let Some(tb) = book.tb.get(name) {
            d.opening += i128::from(tb.opening_paise);
            d.closing += i128::from(tb.closing_paise);
        }
        d.ledgers.push(name.clone());
    }
    heads
}

/// The reference's `_months` (:177-183): every `YYYY-MM` from the period's first month to its last.
fn months(p: &Window) -> Vec<String> {
    let ym = |s: &str| -> (u32, u32) {
        (
            s[0..4].parse().expect("a validated date"),
            s[4..6].parse().expect("a validated date"),
        )
    };
    let (mut y, mut m) = ym(p.from.as_str());
    let end = ym(p.to.as_str());
    let mut out = Vec::new();
    while (y, m) <= end {
        out.push(format!("{y:04}-{m:02}"));
        (y, m) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
    }
    out
}

fn paise(x: i128) -> Result<Value> {
    i64::try_from(x)
        .map(Value::Int)
        .map_err(|_| support::overflow(TEST_ID))
}

/// Vouchers as evidence, one ref per voucher, repeats kept: the reference builds these lists
/// from its own voucher lists (:589, :637), not from a set.
fn voucher_refs(vs: &[&Voucher]) -> Vec<EvidenceRef> {
    vs.iter()
        .map(|v| EvidenceRef::with_label("voucher", &v.guid, &support::voucher_label(v)))
        .collect()
}

/// Run the test: the reference's `run()` (:392-732) with no GSTR-3B and no GSTR-1 row.
pub fn run(
    book: &Book,
    rules: &Rules,
    period: &Window,
    inputs: &Inputs,
    _no_gstr3b: NoGstr3b,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let pop = book.population()?;
    let months = months(period);
    let count = |n: usize| support::count(TEST_ID, n);
    let no_ev = Vec::new;
    r.population_note = "Books population (optional, cancelled and post-dated vouchers excluded). \
Per-head figures classify every population voucher line touching a configured tax ledger into \
output/input/payment/other by voucher base type and (for a Credit/Debit Note) which ledger group \
its other lines fall under -- never by ledger name."
        .into();

    for (name, head) in &inputs.tax_ledgers {
        r.fig(
            &format!("tax_ledger_{}", stable_ledger_tag(book, name)?),
            Value::Text(head.clone()),
            Unit::Text,
            "GST duty head for one ledger.",
            vec![EvidenceRef::new("ledger", name)],
        )?;
    }
    for name in &inputs.rcm_tax_ledgers {
        r.fig(
            &format!("rcm_tax_ledger_{}", stable_ledger_tag(book, name)?),
            Value::Text("rcm".to_string()),
            Unit::Text,
            "Ledger role 'rcm_tax_ledger' (caller-supplied; see the report's proposed config).",
            vec![EvidenceRef::new("ledger", name)],
        )?;
    }

    // Per-head ledger movement (:433-462).
    let heads = head_buckets(book, &pop, inputs);
    for (head, d) in &heads {
        let ev: Vec<EvidenceRef> = d
            .ledgers
            .iter()
            .map(|n| EvidenceRef::new("ledger", n))
            .collect();
        let rows: [(&str, i128, String); 6] = [
            ("opening_paise", d.opening, format!("TB opening balance summed across every '{head}' tax ledger (Dr+/Cr-).")),
            ("output_tax_paise", -d.output, format!("Output tax charged this FY on '{head}' ledgers: Sales/sales-side Credit or Debit Note population vouchers, negated from the ledger's own Cr- to a positive tax-charged figure.")),
            ("input_tax_paise", d.input, format!("Input tax booked this FY on '{head}' ledgers: Purchase/purchase-side Credit or Debit Note vouchers, plus a Journal carrying genuine input tax (ledger's own Dr+ sign, not negated).")),
            ("payment_paise", d.payment, format!("Payment/set-off movement this FY on '{head}' ledgers: Payment/Receipt vouchers, and a Journal reclassifying entirely among Duties & Taxes/payable ledgers (ledger's own Dr+/Cr- sign, not negated).")),
            ("other_paise", d.other, format!("Movement this FY on '{head}' ledgers not classified output/input/payment (e.g. a Contra, or an ambiguous note with no Sales/Purchase line) -- reported, never hidden (ledger's own sign, not negated).")),
            ("closing_paise", d.closing, format!("TB closing balance summed across every '{head}' tax ledger (Dr+/Cr-).")),
        ];
        for (suffix, value, definition) in rows {
            r.fig(
                &format!("head_{head}_{suffix}"),
                paise(value)?,
                Unit::Paise,
                &definition,
                ev.clone(),
            )?;
        }
        r.fig(
            &format!("head_{head}_movement_unexplained_residual_paise"),
            paise(d.closing - d.opening - (d.output + d.input + d.payment + d.other))?,
            Unit::Paise,
            &format!("'{head}' TB movement (closing-opening) minus (raw output + input + payment + other) -- must be 0 (LEDGER-1); reported even when zero, never hidden."),
            no_ev(),
        )?;
    }
    if !heads.is_empty() {
        r.findings.push(Finding {
            id: format!("{TEST_ID}/needs_electronic_ledgers"),
            clauses: vec!["GSTR-9C-9".to_string()],
            title: "GST duty-head ledger balances are FY-opening-plus-movement accumulations, not \
confirmed against the GST portal's own Electronic Cash/Credit Ledger"
                .to_string(),
            facts: heads
                .keys()
                .map(|h| {
                    (
                        format!("{h}_closing"),
                        format!("{TEST_ID}.head_{h}_closing_paise"),
                    )
                })
                .collect(),
            evidence: Vec::new(),
            confidence: Confidence::NeedsDocument,
            limits: vec![
                "A duty-head ledger balance carried from an opening balance and this FY's own \
movement cannot, by itself, show the position the GST portal holds at year end when the monthly \
set-off is not journalised (see the GST set-off item on the Accounting Quality sheet for the same \
point at the coarser Trial-Balance level)."
                    .to_string(),
            ],
            ask_client: vec![
                "Electronic Cash Ledger and Electronic Credit Ledger from the GST portal, one \
per GSTIN, as at the year end, to reconcile against the closing figures above."
                    .to_string(),
            ],
        });
    }

    // Monthly outward figures (:482-568). Every month is no_3b_document_for_month with the books'
    // own figure as its difference (:355-357), so each series' total difference is its books total.
    let sales = book.ledgers_under_any(&[SALES_GROUP.to_string()]);
    let mut taxable_by_month: BTreeMap<String, i128> = BTreeMap::new();
    let mut tax_by_month: BTreeMap<String, i128> = BTreeMap::new();
    for v in &pop {
        let m = year_month(&v.date);
        let amt: i128 = v
            .lines
            .iter()
            .filter(|l| sales.contains(&l.ledger))
            .map(|l| i128::from(l.amount_paise))
            .sum();
        if amt != 0 {
            *taxable_by_month.entry(m.clone()).or_default() -= amt;
        }
        if tax_lines(v, inputs).next().is_some()
            && classify_voucher(v, book, inputs) == Category::Output
        {
            *tax_by_month.entry(m).or_default() -= tax_lines(v, inputs)
                .map(|l| i128::from(l.amount_paise))
                .sum::<i128>();
        }
    }
    let tax_keys: BTreeSet<String> = inputs.tax_ledgers.keys().cloned().collect();
    let (residual_rows, _excluded) =
        books_outward_rows(book, &tax_keys, &inputs.round_off_ledgers)?;
    let mut residual_by_month: BTreeMap<String, i128> = BTreeMap::new();
    for row in &residual_rows {
        *residual_by_month.entry(row.month.clone()).or_default() += row.taxable_paise;
    }
    let at = |map: &BTreeMap<String, i128>, m: &str| map.get(m).copied().unwrap_or(0);
    for m in &months {
        r.fig(
            &format!("monthly_output_books_taxable_residual_paise_{m}"),
            paise(at(&residual_by_month, m))?,
            Unit::Paise,
            &format!("OLD definition (party credit - tax - round-off, gst_outward_gstr1's own residual) for {m} -- NOT comparable to the 3B column above; kept only for cross-reference against GSTR-1."),
            no_ev(),
        )?;
    }
    // Every month the rows fall in, inside the period or not (:501).
    r.fig(
        "annual_output_books_taxable_residual_paise",
        paise(residual_by_month.values().sum())?,
        Unit::Paise,
        "Sum of the OLD residual definition above across the year -- NOT the 3B comparison basis (see annual_books_taxable_paise for that); reported for the record, not hidden.",
        no_ev(),
    )?;
    let outward_months_id = r.fig(
        "gstr3b_outward_months_available_count",
        count(0)?,
        Unit::Count,
        "Distinct months with a GSTR-3B offset/summary row for outward supply/output tax (0 when this client has no such document -- see module docstring).",
        no_ev(),
    )?;
    let annual_taxable: i128 = months.iter().map(|m| at(&taxable_by_month, m)).sum();
    let annual_tax: i128 = months.iter().map(|m| at(&tax_by_month, m)).sum();
    r.fig(
        "annual_books_taxable_paise",
        paise(annual_taxable)?,
        Unit::Paise,
        "Sum of the sales-ledger-movement monthly figures below across the year.",
        no_ev(),
    )?;
    r.fig(
        "annual_gstr3b_taxable_paise",
        paise(0)?,
        Unit::Paise,
        "Sum of the 3B sheet's own monthly Total (NRC+RC+Non-Taxable) column.",
        no_ev(),
    )?;
    r.fig(
        "annual_taxable_tie_diff_paise",
        paise(annual_taxable)?,
        Unit::Paise,
        "annual_books_taxable_paise minus annual_gstr3b_taxable_paise; reported even when zero.",
        no_ev(),
    )?;
    for m in &months {
        let (bt, btx) = (at(&tax_by_month, m), at(&taxable_by_month, m));
        let figs: [(String, Value, Unit, String); 8] = [
            (format!("monthly_output_books_tax_{m}_paise"), paise(bt)?, Unit::Paise, format!("Books output tax for {m}: tax-ledger lines on CAT_OUTPUT-classified population vouchers.")),
            (format!("monthly_output_gstr3b_tax_{m}_paise"), paise(0)?, Unit::Paise, format!("GSTR-3B output tax filed for {m}.")),
            (format!("monthly_output_books_taxable_{m}_paise"), paise(btx)?, Unit::Paise, format!("Books taxable outward turnover for {m}: Sales-Accounts-group ledger movement.")),
            (format!("monthly_output_gstr3b_taxable_{m}_paise"), paise(0)?, Unit::Paise, format!("GSTR-3B outward turnover filed for {m} -- the sheet's own Total (NRC+RC+Non-Taxable) column.")),
            (format!("monthly_output_tax_diff_{m}_paise"), paise(bt)?, Unit::Paise, format!("Books output tax minus GSTR-3B output tax for {m}; reason '{NO_3B}'.")),
            (format!("monthly_output_tax_diff_reason_{m}"), Value::Text(NO_3B.to_string()), Unit::Text, format!("Closed-set reason for the {m} output-tax gap.")),
            (format!("monthly_output_taxable_diff_{m}_paise"), paise(btx)?, Unit::Paise, format!("Books taxable minus GSTR-3B taxable for {m}; reason '{NO_3B}'.")),
            (format!("monthly_output_taxable_diff_reason_{m}"), Value::Text(NO_3B.to_string()), Unit::Text, format!("Closed-set reason for the {m} taxable gap.")),
        ];
        for (id, value, unit, definition) in figs {
            r.fig(&id, value, unit, &definition, no_ev())?;
        }
    }
    // Months per reason, the same in all three series: every month is no_3b_document_for_month.
    let months_with = |reason: &str| if reason == NO_3B { months.len() } else { 0 };
    let reason_count = |reason: &str| count(months_with(reason));
    for reason in OUTWARD_REASONS {
        r.fig(
            &format!("monthly_output_tax_reason_{reason}_count"),
            reason_count(reason)?,
            Unit::Count,
            &format!("Months whose output-tax gap classifies '{reason}'."),
            no_ev(),
        )?;
        r.fig(
            &format!("monthly_output_taxable_reason_{reason}_count"),
            reason_count(reason)?,
            Unit::Count,
            &format!("Months whose taxable-turnover gap classifies '{reason}'."),
            no_ev(),
        )?;
    }
    r.fig("annual_output_tax_diff_paise", paise(annual_tax)?, Unit::Paise, "Sum of every month's books-minus-3B output-tax gap; reported even when the components net close to it, never hidden.", no_ev())?;
    r.fig(
        "annual_output_taxable_diff_paise",
        paise(annual_taxable)?,
        Unit::Paise,
        "Sum of every month's books-minus-3B taxable gap.",
        no_ev(),
    )?;

    // Monthly reverse-charge figures (:571-612). With no RCM tax ledger the books' figure and the
    // difference are not emitted; with no 3B row every month is no_3b_document_for_month (:202-203).
    let mut rcm_by_month: BTreeMap<String, (i128, Vec<&Voucher>)> = BTreeMap::new();
    for v in &pop {
        let amt: i128 = v
            .lines
            .iter()
            .filter(|l| inputs.rcm_tax_ledgers.contains(&l.ledger))
            .map(|l| i128::from(l.amount_paise))
            .sum();
        if amt != 0 {
            let slot = rcm_by_month.entry(year_month(&v.date)).or_default();
            slot.0 += amt;
            slot.1.push(v);
        }
    }
    r.fig("gstr3b_rcm_months_available_count", count(0)?, Unit::Count, "Distinct months with a GSTR-3B reverse-charge liability row (0 when this document has no such block, or the client has none at all).", no_ev())?;
    let rcm_count_id = r.fig("rcm_tax_ledger_count", count(inputs.rcm_tax_ledgers.len())?, Unit::Count, "RCM tax ledgers set for this client (none means this section reports nothing to compare, not a missing RCM liability).", no_ev())?;
    let books_read = !inputs.rcm_tax_ledgers.is_empty();
    let mut annual_rcm = 0i128;
    for m in &months {
        let (b, vs) = rcm_by_month
            .get(m)
            .map_or((0, &[][..]), |(b, vs)| (*b, vs.as_slice()));
        annual_rcm += b;
        if books_read {
            r.fig(
                &format!("monthly_rcm_books_tax_{m}_paise"),
                paise(b)?,
                Unit::Paise,
                &format!("Books RCM tax recognised for {m}."),
                voucher_refs(vs),
            )?;
        }
        r.fig(
            &format!("monthly_rcm_gstr3b_tax_{m}_paise"),
            paise(0)?,
            Unit::Paise,
            &format!("GSTR-3B reverse-charge tax filed for {m}."),
            no_ev(),
        )?;
        if books_read {
            r.fig(
                &format!("monthly_rcm_tax_diff_{m}_paise"),
                paise(b)?,
                Unit::Paise,
                &format!("Books RCM tax minus GSTR-3B RCM tax for {m}; reason '{NO_3B}'."),
                no_ev(),
            )?;
        }
        r.fig(
            &format!("monthly_rcm_tax_diff_reason_{m}"),
            Value::Text(NO_3B.to_string()),
            Unit::Text,
            &format!("Closed-set reason for the {m} RCM gap."),
            no_ev(),
        )?;
    }
    for reason in RCM_REASONS {
        r.fig(
            &format!("monthly_rcm_tax_reason_{reason}_count"),
            reason_count(reason)?,
            Unit::Count,
            &format!("Months whose RCM-tax gap classifies '{reason}'."),
            no_ev(),
        )?;
    }
    r.fig(
        "annual_rcm_tax_diff_paise",
        paise(annual_rcm)?,
        Unit::Paise,
        if books_read {
            "Sum of every month's books-minus-3B RCM-tax gap."
        } else {
            "Not computed: no RCM tax ledger is set, so the books' reverse-charge tax was not read; this 0 is not a measurement."
        },
        no_ev(),
    )?;
    // LEDGER-3's figure: the unclassified counts of the three series (:614-617).
    r.fig("monthly_unclassified_count", count(3 * months_with(UNCLASSIFIED))?, Unit::Count, "Output-tax + taxable + RCM monthly gaps with no reason from the closed set (LEDGER-3: must be 0).", no_ev())?;

    goods_carriage(&mut r, book, &pop, inputs, &rcm_count_id)?;

    // :718-730: no GSTR-3B document, by construction of `Inputs`.
    r.findings.push(Finding {
        id: format!("{TEST_ID}/no_3b_document"),
        clauses: Vec::new(),
        title: "No GSTR-3B offset/summary document was available for this engagement; the monthly \
comparisons of the books with GSTR-3B, reverse charge included, are not performed"
            .to_string(),
        facts: vec![(
            "gstr3b_outward_months_available_count".to_string(),
            outward_months_id,
        )],
        evidence: Vec::new(),
        confidence: Confidence::NeedsDocument,
        limits: vec![
            "The per-head opening/output/input/payment/closing figures above still stand \
(TB- and voucher-based, no GSTR-3B document required); only the monthly comparison to what was \
actually filed is unavailable."
                .to_string(),
        ],
        ask_client: vec![format!(
            "The firm's own GSTR-3B offset/summary workbook (or the filed GSTR-3B returns \
themselves), month by month, for FY {} to {}.",
            iso(&period.from),
            iso(&period.to)
        )],
    });
    Ok(r)
}

/// The goods-transport question (:619-665): a population voucher debiting a goods-carriage
/// ledger with no line at all (nil included) on a configured tax or RCM ledger or any ledger
/// under Duties & Taxes.
fn goods_carriage(
    r: &mut TestResult,
    book: &Book,
    pop: &[&Voucher],
    inputs: &Inputs,
    rcm_count_id: &str,
) -> Result<()> {
    let goods = &inputs.goods_carriage_ledgers;
    r.fig(
        "goods_carriage_ledgers_count",
        support::count(TEST_ID, goods.len())?,
        Unit::Count,
        if goods.is_empty() {
            "Ledgers the CA marks as payments for goods carriage: none, so the goods transport agency question is not asked here."
        } else {
            "Ledgers the CA marks as payments for goods carriage; the goods transport agency question is asked on their payments."
        },
        Vec::new(),
    )?;
    if goods.is_empty() {
        return Ok(());
    }
    let is_duty = |name: &str| {
        inputs.tax_ledgers.contains_key(name)
            || inputs.rcm_tax_ledgers.contains(name)
            || book
                .ledgers
                .get(name)
                .is_some_and(|l| l.under(DUTIES_GROUP))
    };
    let debits = |v: &Voucher| -> Vec<i128> {
        v.lines
            .iter()
            .filter(|l| goods.contains(&l.ledger) && l.amount_paise > 0)
            .map(|l| i128::from(l.amount_paise))
            .collect()
    };
    let bare: Vec<&Voucher> = pop
        .iter()
        .copied()
        .filter(|v| !debits(v).is_empty() && !v.lines.iter().any(|l| is_duty(&l.ledger)))
        .collect();
    let total: i128 = bare.iter().flat_map(|v| debits(v)).sum();
    let ev = voucher_refs(&bare);
    let n_id = r.fig(
        "goods_carriage_no_gst_count",
        support::count(TEST_ID, bare.len())?,
        Unit::Count,
        "Vouchers debiting a ledger the CA marks as goods carriage that carry no line on any duties ledger (no GST and no reverse-charge tax).",
        ev.clone(),
    )?;
    let t_id = r.fig(
        "goods_carriage_no_gst_total_paise",
        paise(total)?,
        Unit::Paise,
        "Their debits to those ledgers, added together.",
        ev.clone(),
    )?;
    if bare.is_empty() {
        return Ok(());
    }
    r.findings.push(Finding {
        id: format!("{TEST_ID}/gta_question"),
        clauses: Vec::new(),
        title: "Goods-carriage payments with no GST on the voucher: whether each supplier is a goods \
transport agency is not known"
            .to_string(),
        facts: vec![
            ("vouchers".to_string(), n_id),
            ("amount".to_string(), t_id),
            ("rcm_tax_ledgers".to_string(), rcm_count_id.to_string()),
        ],
        evidence: ev,
        confidence: Confidence::JudgementRequired,
        limits: vec![
            "A goods transport agency's service to a specified recipient (such as a registered person, \
a factory, a firm or a body corporate) is taxed under reverse charge, the recipient paying the tax, \
unless the agency opted to charge GST itself. Transport by a truck owner who issues no consignment \
note is exempt; a truck hired with its driver under the hirer's control is a rental, taxed as such. \
The books show neither a consignment note, nor what the supplier is, nor the terms of hire."
                .to_string(),
            "A voucher carrying any line on a duties ledger is not listed, even where that tax is for \
goods on the same voucher rather than for the carriage."
                .to_string(),
            if inputs.rcm_tax_ledgers.is_empty() {
                "No reverse-charge tax ledger is set for this client, so any reverse-charge tax the books \
hold is not read here (see the reverse-charge question)."
            } else {
                "Reverse-charge tax booked by a separate journal is not tied to these payments; any such \
tax the books hold on the reverse-charge tax ledgers set for this client is in this test's \
reverse-charge figures."
            }
            .to_string(),
        ],
        ask_client: vec!["For each supplier listed, whether it is a goods transport agency issuing \
consignment notes, a truck owner, or a hire of a vehicle with its driver; and, for an agency, whether \
it charged GST or reverse-charge tax was paid on its services."
            .to_string()],
    });
    Ok(())
}

/// A figure's integer value, by full id.
fn int_figure(result: &TestResult, id: &str) -> Option<i64> {
    result
        .figures
        .iter()
        .find(|f| f.id == id)
        .and_then(|f| match f.value {
            Value::Int(n) => Some(n),
            _ => None,
        })
}

/// LEDGER-1 to LEDGER-3 (:736-800), re-read from the Trial Balance and the result's own figures.
pub fn check_invariants(book: &Book, result: &TestResult) -> Result<Vec<String>> {
    let prefix = format!("{}.", result.test_id);
    let val = |name: &str| int_figure(result, &format!("{prefix}{name}"));
    let mut out = Vec::new();

    // LEDGER-1: heads are read off the closing figures' ids, ledgers off their evidence (:748-753).
    let (start, suffix) = (prefix.len() + "head_".len(), "_closing_paise");
    let heads: BTreeSet<&str> = result
        .figures
        .iter()
        .filter(|f| f.id.starts_with(&format!("{prefix}head_")) && f.id.ends_with(suffix))
        .map(|f| f.id.get(start..f.id.len() - suffix.len()).unwrap_or(""))
        .collect();
    for head in heads {
        let closing_id = format!("{prefix}head_{head}_closing_paise");
        let ledgers: Vec<&str> = result
            .figures
            .iter()
            .filter(|f| f.id == closing_id)
            .flat_map(|f| {
                f.evidence
                    .iter()
                    .filter(|e| e.kind == "ledger")
                    .map(|e| e.id.as_str())
            })
            .collect();
        let tb_sum = |pick: fn(&crate::book::TbRow) -> i64| -> i128 {
            ledgers
                .iter()
                .filter_map(|n| book.tb.get(*n))
                .map(|t| i128::from(pick(t)))
                .sum()
        };
        let (tb_opening, tb_closing) = (tb_sum(|t| t.opening_paise), tb_sum(|t| t.closing_paise));
        let get = |s: &str| val(&format!("head_{head}_{s}")).map(i128::from);
        let all = [
            get("opening_paise"),
            get("closing_paise"),
            get("output_tax_paise"),
            get("input_tax_paise"),
            get("payment_paise"),
            get("other_paise"),
        ];
        // The reference reports a missing figure after the two Trial Balance comparisons; on this
        // test's own result every figure is present, so a missing one is reported alone here.
        let [Some(opening), Some(closing), Some(output), Some(input), Some(payment), Some(other)] =
            all
        else {
            out.push(format!(
                "LEDGER-1: head '{head}' is missing one or more required figures"
            ));
            continue;
        };
        if opening != tb_opening {
            out.push(format!("LEDGER-1: head '{head}' reported opening {opening}p != fresh TB opening {tb_opening}p"));
        }
        if closing != tb_closing {
            out.push(format!("LEDGER-1: head '{head}' reported closing {closing}p != fresh TB closing {tb_closing}p"));
        }
        let moved = -output + input + payment + other;
        let residual = (closing - opening) - moved;
        if residual.abs() > TOL_PAISE {
            out.push(format!(
                "LEDGER-1: head '{head}' movement {}p != output+input+payment+other {moved}p (residual {residual}p)",
                closing - opening
            ));
        }
    }

    // LEDGER-2 (:773-784): over the union of the two closed sets.
    let reasons: BTreeSet<&str> = OUTWARD_REASONS
        .iter()
        .chain(RCM_REASONS.iter())
        .copied()
        .collect();
    for series in [
        "monthly_output_tax",
        "monthly_output_taxable",
        "monthly_rcm_tax",
    ] {
        let reason_prefix = format!("{prefix}{series}_diff_reason_");
        let month_count = result
            .figures
            .iter()
            .filter(|f| f.id.starts_with(&reason_prefix))
            .count();
        let total: i128 = reasons
            .iter()
            .filter_map(|reason| val(&format!("{series}_reason_{reason}_count")))
            .map(i128::from)
            .sum();
        if month_count > 0 && total != i128::try_from(month_count).unwrap_or(i128::MAX) {
            out.push(format!("LEDGER-2: {series} reason counts sum to {total} but {month_count} months have a diff figure"));
        }
    }

    // LEDGER-3 (:787-791).
    if let Some(n) = val("monthly_unclassified_count").filter(|n| *n != 0) {
        out.push(format!(
            "LEDGER-3: {n} monthly gap(s) have no classification reason"
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{LedgerLine, TbRow, VoucherStatus};
    use bridge_tally_primitives::TallyDate;

    fn window(from: &str, to: &str) -> Window {
        Window {
            from: TallyDate::parse(from).unwrap(),
            to: TallyDate::parse(to).unwrap(),
        }
    }

    #[test]
    fn months_run_from_the_first_month_to_the_last_across_a_year_end() {
        assert_eq!(
            months(&window("20251115", "20260201")),
            ["2025-11", "2025-12", "2026-01", "2026-02"]
        );
        assert!(months(&window("20250501", "20250430")).is_empty());
    }

    /// A one-head result whose checks pass, for the tampering below.
    fn result() -> (Book, TestResult) {
        let tb = TbRow {
            opening_paise: -50,
            debit_paise: 0,
            credit_paise: 90,
            closing_paise: -140,
        };
        let book = Book {
            tb: BTreeMap::from([("Cgst".to_string(), tb)]),
            vouchers: vec![Voucher {
                guid: "s1".to_string(),
                date: TallyDate::parse("20250410").unwrap(),
                base_type: "Sales".to_string(),
                status: VoucherStatus::Regular,
                lines: vec![
                    LedgerLine {
                        ledger: "Party".to_string(),
                        amount_paise: 90,
                    },
                    LedgerLine {
                        ledger: "Cgst".to_string(),
                        amount_paise: -90,
                    },
                ],
                ..Voucher::default()
            }],
            ..Book::default()
        };
        let inputs = Inputs {
            tax_ledgers: BTreeMap::from([("Cgst".to_string(), "cgst".to_string())]),
            ..Inputs::default()
        };
        let mut rules = Rules::vendored().unwrap();
        rules.version = "v".to_string();
        let e = Engagement::from_toml_for_read(
            "[client]\nlabel = \"T\"\nassessment_year = \"2026-27\"\n[period]\n\
             start = \"2025-04-01\"\nend = \"2026-03-31\"\n[roles]\ncash_groups = []\n\
             bank_groups = []\n",
        )
        .unwrap();
        let none = no_gstr3b(&e, &CallerData::default()).unwrap();
        let r = run(
            &book,
            &rules,
            &window("20250401", "20250531"),
            &inputs,
            none,
        )
        .unwrap();
        assert!(check_invariants(&book, &r).unwrap().is_empty());
        (book, r)
    }

    fn set(r: &mut TestResult, name: &str, value: i64) {
        let id = format!("{TEST_ID}.{name}");
        r.figures.iter_mut().find(|f| f.id == id).unwrap().value = Value::Int(value);
    }

    /// Each LEDGER check fires on its own tampering, with the reference's words.
    #[test]
    fn each_ledger_check_fires_on_its_own_tampering() {
        let (book, base) = result();
        let mut r = base.clone();
        set(&mut r, "head_cgst_opening_paise", -49);
        set(&mut r, "head_cgst_closing_paise", -339);
        assert_eq!(
            check_invariants(&book, &r).unwrap(),
            [
                "LEDGER-1: head 'cgst' reported opening -49p != fresh TB opening -50p",
                "LEDGER-1: head 'cgst' reported closing -339p != fresh TB closing -140p",
                "LEDGER-1: head 'cgst' movement -290p != output+input+payment+other -90p (residual -200p)",
            ]
        );
        let mut r = base.clone();
        r.figures
            .retain(|f| f.id != format!("{TEST_ID}.head_cgst_payment_paise"));
        assert_eq!(
            check_invariants(&book, &r).unwrap(),
            ["LEDGER-1: head 'cgst' is missing one or more required figures"]
        );
        let mut r = base.clone();
        set(
            &mut r,
            "monthly_rcm_tax_reason_no_3b_document_for_month_count",
            1,
        );
        set(
            &mut r,
            "monthly_output_taxable_reason_unclassified_count",
            1,
        );
        set(&mut r, "monthly_unclassified_count", 2);
        assert_eq!(
            check_invariants(&book, &r).unwrap(),
            [
                "LEDGER-2: monthly_output_taxable reason counts sum to 3 but 2 months have a diff figure",
                "LEDGER-2: monthly_rcm_tax reason counts sum to 1 but 2 months have a diff figure",
                "LEDGER-3: 2 monthly gap(s) have no classification reason",
            ]
        );
        // A series with no month figure is not checked.
        let mut r = base;
        r.figures
            .retain(|f| !f.id.contains("monthly_output_tax_diff_reason_"));
        set(
            &mut r,
            "monthly_output_tax_reason_within_tolerance_count",
            5,
        );
        assert!(check_invariants(&book, &r).unwrap().is_empty());
    }
}
