//! Financial statements derived from the Trial Balance: a port of the reference Python
//! implementation's `financial_statements` test module, version 1.
//!
//! Definitions, as the reference implements them:
//!   * Every TB row is classified once, by its ledger's own primary group (the last element of
//!     [`Ledger::chain`](crate::book::Ledger)): Sales Accounts, Purchase Accounts, Direct
//!     Expenses, Direct Incomes, Indirect Expenses, Indirect Incomes -- the six reserved primary
//!     groups Tally itself uses for a Profit & Loss account. A ledger under any other primary group
//!     is in the Balance Sheet and out of scope here. Every TB row is used (a TB row is already a
//!     full-year total); the books population matters only to the voucher-population figures.
//!   * Sales, Direct Incomes and Indirect Incomes are credit-balance groups: their summed TB
//!     closing (Dr+/Cr-) is reported sign-flipped. The three expense groups are reported as summed.
//!   * Closing stock is never read off a Stock-in-Hand ledger's own TB closing field: when a
//!     company does not integrate accounts with inventory, Tally leaves that field as a copy of the
//!     opening. Closing stock is opening + the year's TB debit - the year's TB credit, per ledger;
//!     the closing field is published only for comparison, with a count of ledgers where it is
//!     stale. Opening stock is the TB opening.
//!   * Gross profit = sales + direct incomes - (opening stock + purchases + direct expenses -
//!     closing stock), one stock basis at both ends; net profit = gross profit - indirect expenses
//!     + other income.
//!   * When `partner_interest_ledgers` (the `[partners]` config's interest ledgers, bound by
//!     identity like every other configured name) is non-empty, profit before partners' interest is
//!     also reported.
//!   * Voucher population figures read [`Book::population`]/[`Book::excluded`] directly.
//!   * Tie to Tally's own report: `report_totals` is caller data (Tally's own Profit & Loss report
//!     export). This crate does not parse that report: a native-report reader needs its own ADR and
//!     attestation before it ships. The totals are republished as figures; FS-1 does the tie.
//!     `report_tie_status` says whether the tie was performed and on what (net profit only, when
//!     the report carries no closing stock); with no totals the finding says the tie was not
//!     performed and needs a document, rather than reading as a computed tie.
//!
//! **Turnover.** `sales` is the engine's turnover: the Sales Accounts TB closing, GST excluded
//! because output tax posts to separate ledgers, returns netted because they post to the same
//! ledgers. The ICAI Guidance Note's s.44AB turnover can differ at the edges (scrap, sales of fixed
//! assets, trade and cash discounts, sales booked outside Sales Accounts). This port reproduces the
//! reference implementation's definition exactly; it does not settle which definition is right.
//!
//! `check_invariants` (FS-1, FS-2) is independent of [`compute`]/[`run`]: it never calls them.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::{
    Book, ClosingStock, ClosingStockUnknown, OpeningStock, OpeningStockUnknown, Voucher,
    VoucherStatus,
};
use crate::error::{AuditError, Result};
use crate::findings::{pct_bp, Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::read::iso;
use crate::rules::Rules;

pub const TEST_ID: &str = "financial_statements";
pub const VERSION: &str = "1";

const SALES_GROUP: &str = "Sales Accounts";
const PURCHASE_GROUP: &str = "Purchase Accounts";
const DIRECT_EXPENSES_GROUP: &str = "Direct Expenses";
const DIRECT_INCOMES_GROUP: &str = "Direct Incomes";
const INDIRECT_EXPENSES_GROUP: &str = "Indirect Expenses";
const INDIRECT_INCOMES_GROUP: &str = "Indirect Incomes";
const CREDIT_BALANCE_GROUPS: [&str; 3] =
    [SALES_GROUP, DIRECT_INCOMES_GROUP, INDIRECT_INCOMES_GROUP];
const PL_GROUPS: [&str; 6] = [
    SALES_GROUP,
    PURCHASE_GROUP,
    DIRECT_EXPENSES_GROUP,
    DIRECT_INCOMES_GROUP,
    INDIRECT_EXPENSES_GROUP,
    INDIRECT_INCOMES_GROUP,
];
/// A subgroup (never itself primary), matched anywhere in a ledger's chain.
const STOCK_IN_HAND_GROUP: &str = "Stock-in-Hand";

/// FS-1's tolerance: Rs 1.
pub const TIE_TOLERANCE_PAISE: i64 = 100;

const POPULATION: &str = "Trial Balance rows, grouped by each ledger's own primary group (all TB \
rows; not filtered to the books population -- a TB row is already a full-year total). \
Voucher-population figures below use the books population (optional, cancelled and post-dated \
vouchers excluded) separately.";

/// Tally's own Profit & Loss report totals, supplied by the caller (see the module docs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportTotals {
    pub net_profit_paise: i64,
    pub closing_stock_paise: Option<i64>,
    /// Where the totals came from; the reference's default when `None`.
    pub source: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StockRow {
    pub ledger: String,
    pub opening_paise: i64,
    pub recomputed_closing_paise: i64,
    pub tb_closing_field_paise: i64,
    pub stale: bool,
}

/// Why a stock term of the gross profit is not known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StockUnknown {
    Opening(OpeningStockUnknown),
    Closing(ClosingStockUnknown),
    /// A Stock-in-Hand ledger carries a value on a book that keeps its stock on the stock items.
    LedgerAlso,
}

impl StockUnknown {
    fn limit(self) -> String {
        let why = match self {
            Self::Opening(r) => format!(
                "Opening stock is held on the stock items, not on a ledger, and was {}",
                r.as_str()
            ),
            Self::Closing(r) => format!(
                "Closing stock is held on the stock items, not on a ledger, and was {}",
                r.as_str()
            ),
            Self::LedgerAlso => {
                "A ledger under Stock-in-Hand carries a value on a book that keeps \
its stock on the stock items, and the two were not added, in case the stock is counted twice"
                    .to_string()
            }
        };
        format!("{why}; no gross profit or net profit is shown.")
    }

    fn ask(self) -> &'static str {
        match self {
            Self::Opening(_) => {
                "Tally's Stock Summary as at the first day of the books, for the opening stock."
            }
            Self::Closing(_) => {
                "Tally's Stock Summary as at the last day of the period, for the closing stock."
            }
            Self::LedgerAlso => {
                "Whether the Stock-in-Hand ledger carries stock that the stock items also hold."
            }
        }
    }
}

/// Where the stock terms of the gross profit come from, decided once from the book's typed terms.
/// Only `Ledgers` and `Items` carry both terms, so a book whose stock is unknown cannot reach the
/// gross profit sum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StockBasis {
    /// The company does not keep inventory integrated with its accounts: Stock-in-Hand ledgers.
    Ledgers,
    /// Integrated inventory, both terms known: the stock items' opening values and the Stock
    /// Summary dated the trial balance's last day.
    Items { opening: i64, closing: i64 },
    /// Integrated inventory with a term unknown, or ledger stock beside it: each known term is
    /// shown, no profit is.
    Unknown {
        opening: Option<i64>,
        closing: Option<i64>,
        why: Vec<StockUnknown>,
    },
}

/// `ledger_has_value`: a Stock-in-Hand ledger carries a nonzero opening or closing.
fn stock_basis(book: &Book, ledger_has_value: bool) -> StockBasis {
    let opening = match book.opening_stock {
        OpeningStock::NotApplicable => return StockBasis::Ledgers,
        OpeningStock::Valued(v) => Ok(v),
        OpeningStock::Unknown(r) => Err(StockUnknown::Opening(r)),
    };
    let closing = match book.closing_stock {
        ClosingStock::Valued(v) => Ok(v),
        ClosingStock::Unknown(r) => Err(StockUnknown::Closing(r)),
        // `load_book` sets both terms from one flag; a book built by hand with only one is not read.
        ClosingStock::NotApplicable => Err(StockUnknown::Closing(ClosingStockUnknown::NotRead)),
    };
    match (opening, closing) {
        (Ok(opening), Ok(closing)) if !ledger_has_value => StockBasis::Items { opening, closing },
        (opening, closing) => {
            let mut why: Vec<StockUnknown> = [opening.err(), closing.err()]
                .into_iter()
                .flatten()
                .collect();
            if why.is_empty() {
                why.push(StockUnknown::LedgerAlso);
            }
            StockBasis::Unknown {
                opening: opening.ok(),
                closing: closing.ok(),
                why,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Statements {
    pub sales: i64,
    pub purchases: i64,
    pub direct_expenses: i64,
    pub direct_incomes: i64,
    pub stock_basis: StockBasis,
    /// `None` when the term is not known.
    pub opening_stock: Option<i64>,
    pub closing_stock: Option<i64>,
    pub closing_stock_tb_field: i64,
    /// `None` when a stock term is not known: never computed as if stock were zero.
    pub gross_profit: Option<i64>,
    pub indirect_expenses: i64,
    pub other_income: i64,
    pub net_profit: Option<i64>,
    pub partner_interest_paise: Option<i64>,
    pub profit_before_partner_interest: Option<i64>,
    /// Per P&L group, the ledgers with a non-zero TB closing (for evidence).
    pub group_lines: BTreeMap<String, Vec<String>>,
    pub stock_rows: Vec<StockRow>,
    pub exported_count: i64,
    pub in_books_count: i64,
    pub excluded_count: i64,
}

/// Excluded vouchers by (status, voucher type), in read order.
pub type ExcludedByGroup<'b> = BTreeMap<(String, String), Vec<&'b Voucher>>;

fn overflow() -> AuditError {
    AuditError::Config("financial_statements: a total overflowed i64 paise".to_string())
}

fn add(a: i64, b: i64) -> Result<i64> {
    a.checked_add(b).ok_or_else(overflow)
}

fn sub(a: i64, b: i64) -> Result<i64> {
    a.checked_sub(b).ok_or_else(overflow)
}

fn status_value(s: VoucherStatus) -> &'static str {
    match s {
        VoucherStatus::Regular => "regular",
        VoucherStatus::Optional => "optional",
        VoucherStatus::Cancelled => "cancelled",
        VoucherStatus::Postdated => "postdated",
        VoucherStatus::Unknown => "unknown",
    }
}

use crate::support::{hash8, voucher_label};

/// The excluded vouchers' refs carry the "excluded_voucher" kind: listing them is this figure's
/// purpose, and POP-4 checks that each really is outside the books population.
fn voucher_evidence(vouchers: &[&Voucher]) -> Vec<EvidenceRef> {
    let mut sorted: Vec<&Voucher> = vouchers.to_vec();
    sorted.sort_by(|a, b| (iso(&a.date), &a.guid).cmp(&(iso(&b.date), &b.guid)));
    sorted
        .into_iter()
        .map(|v| EvidenceRef::with_label("excluded_voucher", &v.guid, &voucher_label(v)))
        .collect()
}

/// Everything [`run`] reports, computed once, plus the excluded vouchers by (status, voucher
/// type). Kept separate so the tests can exercise it and so [`check_invariants`] can be exercised
/// without calling it.
pub fn compute<'b>(
    book: &'b Book,
    partner_interest_ledgers: &BTreeSet<String>,
) -> Result<(Statements, ExcludedByGroup<'b>)> {
    let mut group_totals: BTreeMap<&str, i64> = BTreeMap::new();
    let mut group_lines: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, ledger) in &book.ledgers {
        let Some(primary) = ledger.chain.last() else {
            continue;
        };
        let Some(&group) = PL_GROUPS.iter().find(|g| **g == primary.as_str()) else {
            continue;
        };
        let Some(row) = book.tb.get(name) else {
            continue;
        };
        let t = group_totals.entry(group).or_insert(0);
        *t = add(*t, row.closing_paise)?;
        if row.closing_paise != 0 {
            group_lines
                .entry(group.to_string())
                .or_default()
                .push(name.clone());
        }
    }
    let pl = |group: &str| -> Result<i64> {
        let v = group_totals.get(group).copied().unwrap_or(0);
        if CREDIT_BALANCE_GROUPS.contains(&group) {
            v.checked_neg().ok_or_else(overflow)
        } else {
            Ok(v)
        }
    };
    let sales = pl(SALES_GROUP)?;
    let purchases = pl(PURCHASE_GROUP)?;
    let direct_expenses = pl(DIRECT_EXPENSES_GROUP)?;
    let direct_incomes = pl(DIRECT_INCOMES_GROUP)?;
    let indirect_expenses = pl(INDIRECT_EXPENSES_GROUP)?;
    let other_income = pl(INDIRECT_INCOMES_GROUP)?;

    let stock_ledgers = book.ledgers_under_any(&[STOCK_IN_HAND_GROUP.to_string()]);
    let mut stock_rows = Vec::new();
    let (mut opening_stock, mut closing_stock, mut closing_stock_tb_field) = (0i64, 0i64, 0i64);
    for name in &stock_ledgers {
        let Some(row) = book.tb.get(name) else {
            continue;
        };
        let recomputed = sub(add(row.opening_paise, row.debit_paise)?, row.credit_paise)?;
        opening_stock = add(opening_stock, row.opening_paise)?;
        closing_stock = add(closing_stock, recomputed)?;
        closing_stock_tb_field = add(closing_stock_tb_field, row.closing_paise)?;
        stock_rows.push(StockRow {
            ledger: name.clone(),
            opening_paise: row.opening_paise,
            recomputed_closing_paise: recomputed,
            tb_closing_field_paise: row.closing_paise,
            stale: recomputed != row.closing_paise,
        });
    }

    let ledger_has_value = stock_rows.iter().any(|r| {
        r.opening_paise != 0 || r.recomputed_closing_paise != 0 || r.tb_closing_field_paise != 0
    });
    let stock_basis = stock_basis(book, ledger_has_value);
    let stock_terms = match &stock_basis {
        StockBasis::Ledgers => Some((opening_stock, closing_stock)),
        StockBasis::Items { opening, closing } => Some((*opening, *closing)),
        StockBasis::Unknown { .. } => None,
    };
    let (opening_stock, closing_stock) = match &stock_basis {
        StockBasis::Unknown {
            opening, closing, ..
        } => (*opening, *closing),
        _ => (stock_terms.map(|t| t.0), stock_terms.map(|t| t.1)),
    };
    let (gross_profit, net_profit) = match stock_terms {
        Some((opening, closing)) => {
            let cost = sub(add(add(opening, purchases)?, direct_expenses)?, closing)?;
            let gross_profit = sub(add(sales, direct_incomes)?, cost)?;
            let net_profit = add(sub(gross_profit, indirect_expenses)?, other_income)?;
            (Some(gross_profit), Some(net_profit))
        }
        None => (None, None),
    };

    let (partner_interest_paise, profit_before_partner_interest) =
        if partner_interest_ledgers.is_empty() {
            (None, None)
        } else {
            let mut pi = 0i64;
            for n in partner_interest_ledgers {
                if let Some(row) = book.tb.get(n) {
                    pi = add(pi, row.closing_paise)?;
                }
            }
            (Some(pi), net_profit.map(|np| add(np, pi)).transpose()?)
        };

    let in_books = book.population()?.len();
    let mut excluded: ExcludedByGroup = BTreeMap::new();
    let mut excluded_count = 0usize;
    for v in book.excluded() {
        excluded_count += 1;
        excluded
            .entry((status_value(v.status).to_string(), v.vtype.clone()))
            .or_default()
            .push(v);
    }
    let count = |n: usize| i64::try_from(n).map_err(|_| overflow());
    Ok((
        Statements {
            sales,
            purchases,
            direct_expenses,
            direct_incomes,
            stock_basis,
            opening_stock,
            closing_stock,
            closing_stock_tb_field,
            gross_profit,
            indirect_expenses,
            other_income,
            net_profit,
            partner_interest_paise,
            profit_before_partner_interest,
            group_lines,
            stock_rows,
            exported_count: count(book.vouchers.len())?,
            in_books_count: count(in_books)?,
            excluded_count: count(excluded_count)?,
        },
        excluded,
    ))
}

pub fn run(
    book: &Book,
    rules: &Rules,
    partner_interest_ledgers: &BTreeSet<String>,
    report_totals: Option<&ReportTotals>,
) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    r.population_note = POPULATION.to_string();
    let (d, excluded) = compute(book, partner_interest_ledgers)?;

    let group_ev = |group: &str| -> Vec<EvidenceRef> {
        d.group_lines
            .get(group)
            .map(|names| {
                names
                    .iter()
                    .map(|n| EvidenceRef::new("ledger", n))
                    .collect()
            })
            .unwrap_or_default()
    };
    let f_sales = r.fig(
        "sales",
        Value::Int(d.sales),
        Unit::Paise,
        "Sales Accounts: sum of TB closing balances of every ledger under the primary group \
'Sales Accounts' (Dr+/Cr- convention; credit-balance group, sign-flipped to a positive revenue \
figure).",
        group_ev(SALES_GROUP),
    )?;
    let f_direct_inc = r.fig(
        "direct_incomes",
        Value::Int(d.direct_incomes),
        Unit::Paise,
        "Direct Incomes: sum of TB closing balances under the primary group 'Direct Incomes', \
sign-flipped to a positive figure.",
        group_ev(DIRECT_INCOMES_GROUP),
    )?;
    let f_purchases = r.fig(
        "purchases",
        Value::Int(d.purchases),
        Unit::Paise,
        "Purchase Accounts: sum of TB closing balances under the primary group 'Purchase \
Accounts' (debit-balance group, no sign flip).",
        group_ev(PURCHASE_GROUP),
    )?;
    let f_direct_exp = r.fig(
        "direct_expenses",
        Value::Int(d.direct_expenses),
        Unit::Paise,
        "Direct Expenses: sum of TB closing balances under the primary group 'Direct Expenses', \
no sign flip.",
        group_ev(DIRECT_EXPENSES_GROUP),
    )?;

    let on_items = !matches!(d.stock_basis, StockBasis::Ledgers);
    let stock_ev: Vec<EvidenceRef> = d
        .stock_rows
        .iter()
        .map(|row| EvidenceRef::new("ledger", &row.ledger))
        .collect();
    let f_open_stock = d
        .opening_stock
        .map(|v| {
            if on_items {
                r.fig(
                    "opening_stock",
                    Value::Int(v),
                    Unit::Paise,
                    "Opening stock: the stock items' opening values (Dr positive), as at the books' \
first day. Tally holds it on the stock items, not on a ledger.",
                    Vec::new(),
                )
            } else {
                r.fig(
                    "opening_stock",
                    Value::Int(v),
                    Unit::Paise,
                    "Opening stock: sum of TB opening balances of every ledger under 'Stock-in-Hand' (a \
masters value, read directly; not subject to the quirk in the Trial Balance's own closing field \
for Stock-in-Hand ledgers).",
                    stock_ev.clone(),
                )
            }
        })
        .transpose()?;
    let f_close_stock = d
        .closing_stock
        .map(|v| {
            if on_items {
                r.fig(
                    "closing_stock",
                    Value::Int(v),
                    Unit::Paise,
                    "Closing stock: the closing values (Dr positive) of Tally's Stock Summary dated \
the last day of the period. Tally holds it on the stock items, not on a ledger.",
                    Vec::new(),
                )
            } else {
                r.fig(
                    "closing_stock",
                    Value::Int(v),
                    Unit::Paise,
                    "Closing stock: sum, per Stock-in-Hand ledger, of TB opening + the year's TB debit \
movement - the year's TB credit movement (recomputed; the ledger's own TB closing field is NOT \
used -- it is shown separately, for comparison, with a count of the ledgers where it is stale).",
                    stock_ev.clone(),
                )
            }
        })
        .transpose()?;
    if !on_items {
        r.fig(
            "closing_stock_tb_field",
            Value::Int(d.closing_stock_tb_field),
            Unit::Paise,
            "Sum, per Stock-in-Hand ledger, of the TB's OWN closing field -- shown only for comparison; \
the recomputed closing stock (opening plus net debit movement) is the figure used everywhere else in \
this test.",
            stock_ev,
        )?;
        let stale: Vec<&StockRow> = d.stock_rows.iter().filter(|row| row.stale).collect();
        r.fig(
            "stock_in_hand_ledgers_with_stale_tb_closing_field_count",
            Value::Int(i64::try_from(stale.len()).map_err(|_| overflow())?),
            Unit::Count,
            "Stock-in-Hand ledgers where the TB's own closing field does not equal opening + net \
debit movement -- Tally leaves the closing field as a static copy of the opening on such a ledger \
when \
the company does not integrate accounts with inventory; verified nonzero on data (not merely \
asserted) wherever this quirk is present.",
            stale
                .iter()
                .map(|row| EvidenceRef::new("ledger", &row.ledger))
                .collect(),
        )?;
    }

    let f_gp = d
        .gross_profit
        .map(|gp| {
            r.fig(
                "gross_profit",
                Value::Int(gp),
                Unit::Paise,
                if on_items {
                    "Sales plus direct incomes, less (opening stock plus purchases plus direct expenses \
less closing stock); the stock taken from the stock items at both ends, direct expenses included, so \
this ties to Form 3CD Clause 40."
                } else {
                    "Sales plus direct incomes, less (opening stock plus purchases plus direct expenses less \
closing stock); one stock basis (Trial Balance / balance sheet, with the recomputed closing stock) \
used at BOTH ends, direct expenses included, so this ties to Form 3CD Clause 40."
                },
                Vec::new(),
            )
        })
        .transpose()?;
    let f_gp_pct = d
        .gross_profit
        .map(|gp| {
            r.fig(
                "gross_profit_pct_bp",
                pct_bp(gp, d.sales).ok_or_else(overflow)?,
                Unit::BasisPoints,
                if on_items {
                    "Gross profit as a share of sales, on the stock items' values."
                } else {
                    "Gross profit as a share of sales, on the Trial Balance stock basis (recomputed \
closing stock)."
                },
                Vec::new(),
            )
        })
        .transpose()?;
    if let Some(cs) = d.closing_stock {
        r.fig(
            "stock_to_turnover_pct_bp",
            pct_bp(cs, d.sales).ok_or_else(overflow)?,
            Unit::BasisPoints,
            "Closing stock as a share of sales.",
            Vec::new(),
        )?;
    }
    let f_indirect_exp = r.fig(
        "indirect_expenses",
        Value::Int(d.indirect_expenses),
        Unit::Paise,
        "Indirect Expenses: sum of TB closing balances under the primary group 'Indirect \
Expenses', no sign flip.",
        group_ev(INDIRECT_EXPENSES_GROUP),
    )?;
    let f_other_inc = r.fig(
        "other_income",
        Value::Int(d.other_income),
        Unit::Paise,
        "Indirect Incomes: sum of TB closing balances under the primary group 'Indirect Incomes', \
sign-flipped to a positive figure.",
        group_ev(INDIRECT_INCOMES_GROUP),
    )?;
    let f_np = d
        .net_profit
        .map(|np| {
            r.fig(
                "net_profit",
                Value::Int(np),
                Unit::Paise,
                "Gross profit less indirect expenses plus indirect incomes.",
                Vec::new(),
            )
        })
        .transpose()?;
    if let Some(np) = d.net_profit {
        r.fig(
            "net_profit_pct_bp",
            pct_bp(np, d.sales).ok_or_else(overflow)?,
            Unit::BasisPoints,
            "Net profit as a share of sales.",
            Vec::new(),
        )?;
    }

    let mut facts = vec![
        ("sales".to_string(), f_sales),
        ("purchases".to_string(), f_purchases),
        ("direct_expenses".to_string(), f_direct_exp),
        ("direct_incomes".to_string(), f_direct_inc),
    ];
    // A term that is not known has no figure, so it has no fact either.
    for (name, fact) in [
        ("opening_stock", f_open_stock),
        ("closing_stock", f_close_stock),
        ("gross_profit", f_gp),
        ("gross_profit_pct_bp", f_gp_pct),
    ] {
        facts.extend(fact.map(|f| (name.to_string(), f)));
    }
    facts.push(("indirect_expenses".to_string(), f_indirect_exp));
    facts.push(("other_income".to_string(), f_other_inc));
    facts.extend(f_np.map(|f| ("net_profit".to_string(), f)));

    if let Some(pi) = d.partner_interest_paise {
        let ev: Vec<EvidenceRef> = partner_interest_ledgers
            .iter()
            .map(|n| EvidenceRef::new("ledger", n))
            .collect();
        let f_pi = r.fig(
            "partner_interest",
            Value::Int(pi),
            Unit::Paise,
            "Sum of TB closing balances of the client's own partners'-interest ledgers (the client's \
setup; already included inside indirect expenses).",
            ev.clone(),
        )?;
        facts.push(("partner_interest".to_string(), f_pi));
        if let Some(pbi) = d.profit_before_partner_interest {
            let f_pbi = r.fig(
                "profit_before_partner_interest",
                Value::Int(pbi),
                Unit::Paise,
                "Net profit plus the partners' interest (net profit is AFTER partners' interest, since it \
is charged as an indirect expense).",
                ev,
            )?;
            facts.push(("profit_before_partner_interest".to_string(), f_pbi));
        }
    }

    let limits = if on_items {
        vec![
            "The gross profit ratio uses the stock items' values: opening stock as at the books' first \
day and closing stock from the Stock Summary dated the last day of the period, at the costing method \
Tally applies, which was not read."
                .to_string(),
            "Stock is Tally's own valuation of the stock items, not an independently counted or valued \
physical stock figure; the books cannot show whether it was actually verified."
                .to_string(),
            "A stock item with no value counts as nothing, so a Stock Summary whose items all lack a \
closing value shows a closing stock of zero; a Summary that lists no item at all is not taken as zero."
                .to_string(),
        ]
    } else {
        vec![
        "The gross profit ratio uses the stock values in the books (balance-sheet basis) at both \
ends. A ratio on Tally's item-level Stock Summary values is not shown here; if one is wanted, both \
opening and closing stock must be taken on that same basis."
            .to_string(),
        "Closing stock is the Stock-in-Hand ledger balance the client recorded in the books \
(accounts not integrated with inventory), not an independently counted or valued physical stock \
figure; the books cannot show whether it was actually verified."
            .to_string(),
        ]
    };
    let clauses = vec!["3CD-40".to_string(), "3CD-14".to_string()];
    // Whether the tie to Tally's own report was performed, as a figure: the finding below is about
    // the tie, and a reader takes its confidence as the tie's status, so "not performed" must be
    // said, not implied. The report's closing stock is optional, so "performed" names what was tied.
    let tie_status = match (report_totals, d.net_profit) {
        (None, _) => "not performed: no report part in this read",
        (Some(rep), Some(_)) if rep.closing_stock_paise.is_some() => {
            "performed: net profit and closing stock"
        }
        (Some(_), Some(_)) => "performed: net profit only",
        (Some(rep), None) if rep.closing_stock_paise.is_some() && d.closing_stock.is_some() => {
            "performed: closing stock only"
        }
        (Some(_), None) => "not performed: the stock is not known, so no net profit was derived",
    };
    let f_tie_status = r.fig(
        "report_tie_status",
        Value::Text(tie_status.to_string()),
        Unit::Text,
        "Whether net profit and closing stock were tied to Tally's own Profit & Loss report: \
'performed' and which of the two, when the report's totals were supplied (from the read's report \
part), else 'not performed' and why.",
        Vec::new(),
    )?;
    facts.push(("report_tie_status".to_string(), f_tie_status));
    let mut diff_np = None;
    if let Some(rep) = report_totals {
        let source = rep
            .source
            .as_deref()
            .unwrap_or("Tally's own Profit & Loss / Balance Sheet report export");
        let f_rep_np = r.fig(
            "report_net_profit",
            Value::Int(rep.net_profit_paise),
            Unit::Paise,
            &format!(
                "Net profit per {source} (Tally's own report; never recomputed by this test)."
            ),
            Vec::new(),
        )?;
        if let Some(np) = d.net_profit {
            let diff = sub(np, rep.net_profit_paise)?;
            r.fig(
                "report_net_profit_diff",
                Value::Int(diff),
                Unit::Paise,
                "Net profit less the net profit in Tally's own report.",
                Vec::new(),
            )?;
            diff_np = Some(diff);
        }
        facts.push(("report_net_profit".to_string(), f_rep_np));
        if let Some(cs) = rep.closing_stock_paise {
            let f_rep_cs = r.fig(
                "report_closing_stock",
                Value::Int(cs),
                Unit::Paise,
                &format!(
                    "Closing stock per {source} (Tally's own report; never recomputed by this \
test)."
                ),
                Vec::new(),
            )?;
            if let Some(derived) = d.closing_stock {
                r.fig(
                    "report_closing_stock_diff",
                    Value::Int(sub(derived, cs)?),
                    Unit::Paise,
                    "Closing stock less the closing stock in Tally's own report.",
                    Vec::new(),
                )?;
            }
            facts.push(("report_closing_stock".to_string(), f_rep_cs));
        }
    }
    if let StockBasis::Unknown { why, .. } = &d.stock_basis {
        let mut limits = limits;
        limits.extend(why.iter().map(|u| u.limit()));
        r.findings.push(Finding {
            id: format!("{TEST_ID}/report_tie"),
            clauses,
            title: "Report tie not performed on net profit: the stock is not known, so no gross \
profit or net profit was derived"
                .to_string(),
            facts,
            evidence: Vec::new(),
            confidence: Confidence::NeedsDocument,
            limits,
            ask_client: why.iter().map(|u| u.ask().to_string()).collect(),
        });
    } else if let Some(diff_np) = diff_np {
        let tie_ok = diff_np.checked_abs().ok_or_else(overflow)? <= TIE_TOLERANCE_PAISE;
        r.findings.push(Finding {
            id: format!("{TEST_ID}/report_tie"),
            clauses,
            title: "Net profit (and closing stock, where the report carries it) tied to Tally's \
own P&L/Balance Sheet report"
                .to_string(),
            facts,
            evidence: Vec::new(),
            confidence: if tie_ok {
                Confidence::Computed
            } else {
                Confidence::JudgementRequired
            },
            limits,
            ask_client: Vec::new(),
        });
    } else {
        let mut limits = limits;
        limits.push(
            "Tally's Profit & Loss and Balance Sheet reports were not supplied for this \
engagement; net profit and closing stock are not independently tied to a Tally report printout in \
this pack."
                .to_string(),
        );
        r.findings.push(Finding {
            id: format!("{TEST_ID}/report_tie"),
            clauses,
            title: format!(
                "Report tie not performed: no Tally Profit & Loss report part in this read; net \
profit and closing stock are from {} only",
                if on_items {
                    "the Trial Balance and the stock items"
                } else {
                    "the Trial Balance"
                }
            ),
            facts,
            evidence: Vec::new(),
            confidence: Confidence::NeedsDocument,
            limits,
            ask_client: vec![
                "Tally's own Profit & Loss and Balance Sheet printouts (or an export of them) as \
at the period end, to independently tie net profit and closing stock."
                    .to_string(),
            ],
        });
    }

    r.fig(
        "exported_voucher_count",
        Value::Int(d.exported_count),
        Unit::Count,
        "Every voucher exported, all statuses.",
        Vec::new(),
    )?;
    r.fig(
        "in_books_voucher_count",
        Value::Int(d.in_books_count),
        Unit::Count,
        "Books population: exported vouchers with status regular (optional, cancelled and \
post-dated excluded).",
        Vec::new(),
    )?;
    r.fig(
        "excluded_voucher_count",
        Value::Int(d.excluded_count),
        Unit::Count,
        "Exported vouchers excluded from the books population (vouchers exported less vouchers in \
the books population).",
        Vec::new(),
    )?;
    for ((status, vtype), vouchers) in &excluded {
        let h = hash8(&format!("{status}:{vtype}"));
        r.fig(
            &format!("excluded_{h}"),
            Value::Int(i64::try_from(vouchers.len()).map_err(|_| overflow())?),
            Unit::Count,
            &format!(
                "Excluded vouchers with status '{status}' and voucher type '{vtype}' (tag {h})."
            ),
            voucher_evidence(vouchers),
        )?;
    }
    Ok(r)
}

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

/// FS-1 and FS-2, independent of [`compute`]/[`run`]: this function never calls them.
///
/// FS-1 compares the published `report_net_profit`/`report_closing_stock` (Tally's own report,
/// never derived here) with the derived `net_profit`/`closing_stock`, within
/// [`TIE_TOLERANCE_PAISE`]; a side with no report figure is skipped, not a violation. FS-2
/// re-derives net profit and closing stock in a separate walk of the TB and compares them with the
/// published figures.
pub fn check_invariants(book: &Book, result: &TestResult) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let id = |name: &str| format!("{}.{name}", result.test_id);

    let np = int_figure(result, &id("net_profit"));
    let cs = int_figure(result, &id("closing_stock"));
    if let (Some(rep), Some(derived)) = (int_figure(result, &id("report_net_profit")), np) {
        let diff = sub(derived, rep)?;
        if diff.checked_abs().ok_or_else(overflow)? > TIE_TOLERANCE_PAISE {
            out.push(format!(
                "FS-1: derived net_profit ({derived}p) does not tie the Tally report's own net \
profit ({rep}p) within {TIE_TOLERANCE_PAISE}p; difference {diff}p"
            ));
        }
    }
    if let (Some(rep), Some(derived)) = (int_figure(result, &id("report_closing_stock")), cs) {
        let diff = sub(derived, rep)?;
        if diff.checked_abs().ok_or_else(overflow)? > TIE_TOLERANCE_PAISE {
            out.push(format!(
                "FS-1: derived closing_stock ({derived}p) does not tie the Tally report's own \
closing stock ({rep}p) within {TIE_TOLERANCE_PAISE}p; difference {diff}p"
            ));
        }
    }

    // FS-2: a fresh walk, written separately from `compute`.
    let mut fresh: BTreeMap<String, i64> = BTreeMap::new();
    for (name, ledger) in &book.ledgers {
        let Some(primary) = ledger.chain.last() else {
            continue;
        };
        if !PL_GROUPS.contains(&primary.as_str()) {
            continue;
        }
        if let Some(row) = book.tb.get(name) {
            let t = fresh.entry(primary.clone()).or_insert(0);
            *t = add(*t, row.closing_paise)?;
        }
    }
    let g = |name: &str| fresh.get(name).copied().unwrap_or(0);
    let neg = |v: i64| v.checked_neg().ok_or_else(overflow);
    let fresh_sales = neg(g(SALES_GROUP))?;
    let fresh_direct_inc = neg(g(DIRECT_INCOMES_GROUP))?;
    let fresh_other_inc = neg(g(INDIRECT_INCOMES_GROUP))?;
    let (mut open_stock, mut close_stock) = (0i64, 0i64);
    let mut ledger_has_value = false;
    for (name, ledger) in &book.ledgers {
        if !ledger.under(STOCK_IN_HAND_GROUP) {
            continue;
        }
        if let Some(row) = book.tb.get(name) {
            let closing = sub(add(row.opening_paise, row.debit_paise)?, row.credit_paise)?;
            ledger_has_value |= row.opening_paise != 0 || closing != 0 || row.closing_paise != 0;
            open_stock = add(open_stock, row.opening_paise)?;
            close_stock = add(close_stock, closing)?;
        }
    }
    // The stock terms as the book's typed terms give them, then the same sum written out again.
    let profit = |open: i64, close: i64| -> Result<i64> {
        let gp = sub(
            add(fresh_sales, fresh_direct_inc)?,
            sub(
                add(add(open, g(PURCHASE_GROUP))?, g(DIRECT_EXPENSES_GROUP))?,
                close,
            )?,
        )?;
        add(sub(gp, g(INDIRECT_EXPENSES_GROUP))?, fresh_other_inc)
    };
    let (fresh_close, fresh_np) = match stock_basis(book, ledger_has_value) {
        StockBasis::Ledgers => (Some(close_stock), Some(profit(open_stock, close_stock)?)),
        StockBasis::Items { opening, closing } => (Some(closing), Some(profit(opening, closing)?)),
        StockBasis::Unknown { closing, .. } => (closing, None),
    };
    match (np, fresh_np) {
        (Some(derived), Some(fresh_np)) if fresh_np != derived => out.push(format!(
            "FS-2: fresh recompute of net_profit ({fresh_np}p) does not equal the published \
figure ({derived}p); difference {}p",
            sub(fresh_np, derived)?
        )),
        (Some(derived), None) => out.push(format!(
            "FS-2: net_profit ({derived}p) is published although a stock term is not known"
        )),
        (None, Some(fresh_np)) => out.push(format!(
            "FS-2: net_profit is not published although the stock terms are known (fresh \
recompute {fresh_np}p)"
        )),
        _ => {}
    }
    match (cs, fresh_close) {
        (Some(derived), Some(close)) if close != derived => out.push(format!(
            "FS-2: fresh recompute of closing_stock ({close}p) does not equal the published \
figure ({derived}p); difference {}p",
            sub(close, derived)?
        )),
        (Some(derived), None) => out.push(format!(
            "FS-2: closing_stock ({derived}p) is published although it is not known"
        )),
        _ => {}
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{Ledger, LedgerLine, TbRow};
    use bridge_tally_primitives::TallyDate;

    fn ledger(name: &str, chain: &[&str]) -> Ledger {
        Ledger {
            name: name.to_string(),
            parent: chain[0].to_string(),
            chain: chain.iter().map(|s| (*s).to_string()).collect(),
            chain_complete: true,
            master_opening_paise: 0,
            pan: String::new(),
            gstin: String::new(),
            guid: format!("guid-{name}"),
            masterid: None,
        }
    }

    fn voucher(guid: &str, status: VoucherStatus, vtype: &str) -> Voucher {
        Voucher {
            narration: String::new(),
            party_field: String::new(),
            guid: guid.to_string(),
            date: TallyDate::parse("20250601").unwrap(),
            vtype: vtype.to_string(),
            base_type: vtype.to_string(),
            number: String::new(),
            status,
            lines: vec![LedgerLine {
                ledger: "Sales".to_string(),
                amount_paise: 0,
            }],
            ..Default::default()
        }
    }

    /// Sales 10,000 Cr; purchases 6,000 Dr; freight (Direct Expenses, one subgroup down) 500 Dr;
    /// job work (Direct Incomes) 300 Cr; rent 1,200 Dr; interest to partners 400 Dr; interest
    /// received (Indirect Incomes) 100 Cr; one stale stock ledger (opening 2,000, debit 2,500,
    /// closing field 2,000) and one without movement (opening 1,000); a P&L ledger with no TB row;
    /// a Balance Sheet ledger with a balance.
    fn book() -> Book {
        let ledgers = vec![
            ledger("Sales", &["Sales Accounts"]),
            ledger("Purchases", &["Purchase Accounts"]),
            ledger("Freight", &["Carriage", "Direct Expenses"]),
            ledger("Job Work", &["Direct Incomes"]),
            ledger("Rent", &["Indirect Expenses"]),
            ledger("Interest to Partners", &["Indirect Expenses"]),
            ledger("Interest Received", &["Indirect Incomes"]),
            ledger("Stale Stock", &["Stock-in-Hand", "Current Assets"]),
            ledger("Still Stock", &["Stock-in-Hand", "Current Assets"]),
            ledger("No TB Row", &["Indirect Expenses"]),
            ledger("Debtor", &["Sundry Debtors", "Current Assets"]),
        ];
        let tb = [
            ("Sales", 0, 0, 1_000_000, -1_000_000),
            ("Purchases", 0, 600_000, 0, 600_000),
            ("Freight", 0, 50_000, 0, 50_000),
            ("Job Work", 0, 0, 30_000, -30_000),
            ("Rent", 0, 120_000, 0, 120_000),
            ("Interest to Partners", 0, 40_000, 0, 40_000),
            ("Interest Received", 0, 0, 10_000, -10_000),
            ("Stale Stock", 200_000, 250_000, 0, 200_000),
            ("Still Stock", 100_000, 0, 0, 100_000),
            ("Debtor", 0, 500_000, 0, 500_000),
        ];
        Book {
            company_name: "Synthetic".to_string(),
            company_guid: "test-guid".to_string(),
            read_at: String::new(),
            groups: BTreeMap::new(),
            group_masters: BTreeMap::new(),
            ledgers: ledgers.into_iter().map(|l| (l.name.clone(), l)).collect(),
            vouchers: vec![
                voucher("v1", VoucherStatus::Regular, "Sales"),
                voucher("v2", VoucherStatus::Optional, "Journal"),
                voucher("v3", VoucherStatus::Optional, "Journal"),
                voucher("v4", VoucherStatus::Cancelled, "Sales"),
            ],
            tb: tb
                .into_iter()
                .map(
                    |(n, opening_paise, debit_paise, credit_paise, closing_paise)| {
                        (
                            n.to_string(),
                            TbRow {
                                opening_paise,
                                debit_paise,
                                credit_paise,
                                closing_paise,
                            },
                        )
                    },
                )
                .collect(),
            ..Default::default()
        }
    }

    fn rules() -> Rules {
        Rules::vendored().unwrap()
    }

    fn fig(r: &TestResult, name: &str) -> Value {
        r.figures
            .iter()
            .find(|f| f.id == format!("{TEST_ID}.{name}"))
            .unwrap_or_else(|| panic!("no figure {name}"))
            .value
            .clone()
    }

    fn interest() -> BTreeSet<String> {
        BTreeSet::from(["Interest to Partners".to_string()])
    }

    #[test]
    fn groups_signs_and_stock_basis() {
        let r = run(&book(), &rules(), &BTreeSet::new(), None).unwrap();
        assert_eq!(fig(&r, "sales"), Value::Int(1_000_000)); // credit group, sign-flipped
        assert_eq!(fig(&r, "purchases"), Value::Int(600_000));
        assert_eq!(fig(&r, "direct_expenses"), Value::Int(50_000)); // primary is the chain's last
        assert_eq!(fig(&r, "direct_incomes"), Value::Int(30_000));
        assert_eq!(fig(&r, "indirect_expenses"), Value::Int(160_000)); // no-TB-row ledger skipped
        assert_eq!(fig(&r, "other_income"), Value::Int(10_000));
        assert_eq!(fig(&r, "opening_stock"), Value::Int(300_000));
        // Closing stock is opening + debit - credit, never the stale closing field.
        assert_eq!(fig(&r, "closing_stock"), Value::Int(550_000));
        assert_eq!(fig(&r, "closing_stock_tb_field"), Value::Int(300_000));
        assert_eq!(
            fig(
                &r,
                "stock_in_hand_ledgers_with_stale_tb_closing_field_count"
            ),
            Value::Int(1)
        );
        // GP = 10,000 + 300 - (3,000 + 6,000 + 500 - 5,500) = 6,300; NP = 6,300 - 1,600 + 100.
        assert_eq!(fig(&r, "gross_profit"), Value::Int(630_000));
        assert_eq!(fig(&r, "net_profit"), Value::Int(480_000));
        assert_eq!(fig(&r, "gross_profit_pct_bp"), Value::Int(6300));
        assert!(!r
            .figures
            .iter()
            .any(|f| f.id.ends_with(".partner_interest")));
    }

    /// Closing stock subtracts the year's TB credit too: a stock ledger drawn down during the year
    /// closes lower than opening + debit.
    #[test]
    fn stock_credit_movement_reduces_closing_stock() {
        let mut b = book();
        *b.tb.get_mut("Still Stock").unwrap() = TbRow {
            opening_paise: 100_000,
            debit_paise: 0,
            credit_paise: 30_000,
            closing_paise: 70_000,
        };
        let r = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
        assert_eq!(fig(&r, "closing_stock"), Value::Int(520_000)); // 4,500 + 700
        assert_eq!(
            fig(
                &r,
                "stock_in_hand_ledgers_with_stale_tb_closing_field_count"
            ),
            Value::Int(1) // the drawn-down ledger's field is current, not stale
        );
        assert!(check_invariants(&b, &r).unwrap().is_empty());
    }

    #[test]
    fn partner_interest_is_reported_only_when_configured() {
        let r = run(&book(), &rules(), &interest(), None).unwrap();
        assert_eq!(fig(&r, "partner_interest"), Value::Int(40_000));
        assert_eq!(
            fig(&r, "profit_before_partner_interest"),
            Value::Int(520_000)
        );
        // A configured interest ledger with no TB row counts as zero, as the reference sums it.
        let absent = BTreeSet::from(["Nowhere".to_string()]);
        let r = run(&book(), &rules(), &absent, None).unwrap();
        assert_eq!(fig(&r, "partner_interest"), Value::Int(0));
    }

    #[test]
    fn zero_sales_leaves_the_ratios_undefined_not_zero() {
        let mut b = book();
        b.tb.get_mut("Sales").unwrap().closing_paise = 0;
        let r = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
        assert_eq!(fig(&r, "gross_profit_pct_bp"), Value::Undefined);
        assert_eq!(fig(&r, "net_profit_pct_bp"), Value::Undefined);
        assert_eq!(fig(&r, "stock_to_turnover_pct_bp"), Value::Undefined);
    }

    #[test]
    fn excluded_vouchers_grouped_by_status_and_type() {
        let r = run(&book(), &rules(), &BTreeSet::new(), None).unwrap();
        assert_eq!(fig(&r, "exported_voucher_count"), Value::Int(4));
        assert_eq!(fig(&r, "in_books_voucher_count"), Value::Int(1));
        assert_eq!(fig(&r, "excluded_voucher_count"), Value::Int(3));
        let tag = hash8("optional:Journal");
        assert_eq!(fig(&r, &format!("excluded_{tag}")), Value::Int(2));
        let tag = hash8("cancelled:Sales");
        assert_eq!(fig(&r, &format!("excluded_{tag}")), Value::Int(1));
        // The reference's tag, computed independently: python3 hashlib.sha1(b"optional:Journal").hexdigest()[:8].
        assert_eq!(hash8("optional:Journal"), "8cdf9d3d");
    }

    fn report(net_profit_paise: i64, closing: Option<i64>) -> ReportTotals {
        ReportTotals {
            net_profit_paise,
            closing_stock_paise: closing,
            source: None,
        }
    }

    /// FS-1's tolerance is inclusive: exactly Rs 1 apart ties, one paisa more does not.
    #[test]
    fn the_report_tie_boundary_is_one_rupee_inclusive() {
        let b = book();
        let at = run(
            &b,
            &rules(),
            &BTreeSet::new(),
            Some(&report(480_100, Some(549_900))),
        )
        .unwrap();
        assert_eq!(at.findings[0].confidence, Confidence::Computed);
        assert!(check_invariants(&b, &at).unwrap().is_empty());

        let over = run(
            &b,
            &rules(),
            &BTreeSet::new(),
            Some(&report(480_101, Some(549_899))),
        )
        .unwrap();
        assert_eq!(over.findings[0].confidence, Confidence::JudgementRequired);
        let v = check_invariants(&b, &over).unwrap();
        assert_eq!(v.len(), 2, "{v:?}");
        assert!(v[0].starts_with("FS-1: derived net_profit (480000p)"));
        assert!(v[1].starts_with("FS-1: derived closing_stock (550000p)"));
    }

    #[test]
    fn a_report_without_closing_stock_ties_net_profit_only() {
        let b = book();
        let r = run(&b, &rules(), &BTreeSet::new(), Some(&report(480_000, None))).unwrap();
        assert!(r
            .figures
            .iter()
            .any(|f| f.id.ends_with(".report_net_profit")));
        assert!(!r
            .figures
            .iter()
            .any(|f| f.id.ends_with(".report_closing_stock")));
        assert!(check_invariants(&b, &r).unwrap().is_empty());
    }

    /// A read with no Profit & Loss report part: the report-tie row must not read as a performed,
    /// computed tie (a CA takes the row's confidence as the tie's status). It says the tie was not
    /// performed, needs a document, and carries the status as a figure.
    #[test]
    fn no_report_part_says_the_tie_was_not_performed() {
        let b = book();
        let r = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
        assert_eq!(
            fig(&r, "report_tie_status"),
            Value::Text("not performed: no report part in this read".to_string())
        );
        let f = &r.findings[0];
        assert_eq!(f.id, format!("{TEST_ID}/report_tie"));
        assert!(f.title.starts_with(
            "Report tie not performed: no Tally Profit & Loss report part in this read"
        ));
        assert_eq!(f.confidence, Confidence::NeedsDocument);
        assert!(
            f.facts
                .iter()
                .any(|(k, v)| k == "report_tie_status"
                    && *v == format!("{TEST_ID}.report_tie_status"))
        );
        assert_eq!(f.ask_client.len(), 1);
        assert!(check_invariants(&b, &r).unwrap().is_empty()); // FS-1 skipped, not violated
    }

    /// The with-part path is unchanged apart from the status figure: same title, same confidence rule.
    #[test]
    fn with_a_report_part_the_tie_is_unchanged_and_says_performed() {
        let b = book();
        let r = run(
            &b,
            &rules(),
            &BTreeSet::new(),
            Some(&report(480_000, Some(550_000))),
        )
        .unwrap();
        assert_eq!(
            fig(&r, "report_tie_status"),
            Value::Text("performed: net profit and closing stock".to_string())
        );
        let f = &r.findings[0];
        assert_eq!(
            f.title,
            "Net profit (and closing stock, where the report carries it) tied to Tally's own \
P&L/Balance Sheet report"
        );
        assert_eq!(f.confidence, Confidence::Computed);
        assert!(
            f.facts
                .iter()
                .any(|(k, v)| k == "report_tie_status"
                    && *v == format!("{TEST_ID}.report_tie_status"))
        );
        let off = run(
            &b,
            &rules(),
            &BTreeSet::new(),
            Some(&report(490_000, Some(550_000))),
        )
        .unwrap();
        assert_eq!(
            fig(&off, "report_tie_status"),
            Value::Text("performed: net profit and closing stock".to_string())
        );
        assert_eq!(off.findings[0].confidence, Confidence::JudgementRequired);
    }

    /// The report's closing stock is optional; the status must not claim a stock tie it did not do.
    #[test]
    fn a_report_without_closing_stock_says_only_net_profit_was_tied() {
        let b = book();
        let r = run(&b, &rules(), &BTreeSet::new(), Some(&report(480_000, None))).unwrap();
        assert_eq!(
            fig(&r, "report_tie_status"),
            Value::Text("performed: net profit only".to_string())
        );
        assert_eq!(r.findings[0].confidence, Confidence::Computed);
    }

    /// FS-2 re-derives on its own: a published figure that does not match the book is caught,
    /// even though FS-1 (with no report) has nothing to compare.
    #[test]
    fn fs2_catches_a_published_figure_the_book_does_not_support() {
        let b = book();
        let mut r = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
        for f in &mut r.figures {
            if f.id.ends_with(".net_profit") {
                f.value = Value::Int(480_001);
            }
            if f.id.ends_with(".closing_stock") {
                f.value = Value::Int(300_000); // the stale field's total, the classic mistake
            }
        }
        let v = check_invariants(&b, &r).unwrap();
        assert_eq!(v.len(), 2, "{v:?}");
        assert!(v[0].contains("FS-2: fresh recompute of net_profit (480000p)"));
        assert!(v[1].contains("FS-2: fresh recompute of closing_stock (550000p)"));
    }

    // --- #1505: stock from the stock items on a book that keeps inventory integrated ---

    use crate::book::{ClosingStock, ClosingStockUnknown, OpeningStock, OpeningStockUnknown};

    /// `book()` without its Stock-in-Hand ledgers, as Tally holds a book with integrated
    /// inventory: stock only on the stock items. Sales 10,000, purchases 6,000, freight 500, job
    /// work 300, rent 1,200, partners' interest 400, interest received 100 (rupees).
    fn item_book(opening: OpeningStock, closing: ClosingStock) -> Book {
        let mut b = book();
        for name in ["Stale Stock", "Still Stock"] {
            b.ledgers.remove(name);
            b.tb.remove(name);
        }
        b.opening_stock = opening;
        b.closing_stock = closing;
        b
    }

    fn has_figure(r: &TestResult, name: &str) -> bool {
        r.figures
            .iter()
            .any(|f| f.id == format!("{TEST_ID}.{name}"))
    }

    const NO_PROFIT: [&str; 5] = [
        "gross_profit",
        "gross_profit_pct_bp",
        "net_profit",
        "net_profit_pct_bp",
        "profit_before_partner_interest",
    ];

    /// The issue's example in rupees: opening stock 2,000 and closing stock 3,000 on the items.
    /// GP = 10,000 + 300 - (2,000 + 6,000 + 500 - 3,000) = 4,800; NP = 4,800 - 1,600 + 100 = 3,300.
    /// With both stock terms read as zero, as before, GP would be 3,800.
    #[test]
    fn an_integrated_book_takes_both_stock_terms_from_the_stock_items() {
        let b = item_book(OpeningStock::Valued(200_000), ClosingStock::Valued(300_000));
        let r = run(&b, &rules(), &interest(), None).unwrap();
        assert_eq!(fig(&r, "opening_stock"), Value::Int(200_000));
        assert_eq!(fig(&r, "closing_stock"), Value::Int(300_000));
        assert_eq!(fig(&r, "gross_profit"), Value::Int(480_000));
        assert_eq!(fig(&r, "gross_profit_pct_bp"), Value::Int(4800));
        assert_eq!(fig(&r, "net_profit"), Value::Int(330_000));
        assert_eq!(fig(&r, "net_profit_pct_bp"), Value::Int(3300));
        assert_eq!(fig(&r, "stock_to_turnover_pct_bp"), Value::Int(3000));
        assert_eq!(
            fig(&r, "profit_before_partner_interest"),
            Value::Int(370_000)
        );
        // The ledger-only figures are not shown for stock that sits on no ledger.
        assert!(!has_figure(&r, "closing_stock_tb_field"));
        assert!(!has_figure(
            &r,
            "stock_in_hand_ledgers_with_stale_tb_closing_field_count"
        ));
        assert!(r.figures.iter().all(|f| {
            !["opening_stock", "closing_stock"]
                .iter()
                .any(|n| f.id == format!("{TEST_ID}.{n}"))
                || f.evidence.is_empty()
        }));
        assert!(check_invariants(&b, &r).unwrap().is_empty());
    }

    /// The lab's inventory book (#692, Book B) in paise: sales 1,400, purchase 3,000, freight 500,
    /// no opening stock, closing stock 2,100. Tally's own P&L showed gross profit 500 and nett
    /// profit 0; with no stock term the crate gave -1,600 and -2,100.
    #[test]
    fn the_lab_book_b_shape_ties_the_gross_and_net_profit_tally_showed() {
        let mut b = item_book(OpeningStock::Valued(0), ClosingStock::Valued(210_000));
        for name in [
            "Freight",
            "Job Work",
            "Rent",
            "Interest to Partners",
            "Interest Received",
        ] {
            b.ledgers.remove(name);
            b.tb.remove(name);
        }
        b.tb.get_mut("Sales").unwrap().closing_paise = -140_000;
        b.tb.get_mut("Purchases").unwrap().closing_paise = 300_000;
        b.ledgers.insert(
            "Freight".to_string(),
            ledger("Freight", &["Indirect Expenses"]),
        );
        b.tb.insert(
            "Freight".to_string(),
            TbRow {
                opening_paise: 0,
                debit_paise: 50_000,
                credit_paise: 0,
                closing_paise: 50_000,
            },
        );
        let r = run(
            &b,
            &rules(),
            &BTreeSet::new(),
            Some(&report(0, Some(210_000))),
        )
        .unwrap();
        assert_eq!(fig(&r, "gross_profit"), Value::Int(50_000));
        assert_eq!(fig(&r, "net_profit"), Value::Int(0));
        assert_eq!(r.findings[0].confidence, Confidence::Computed);
        assert!(check_invariants(&b, &r).unwrap().is_empty());
    }

    /// A book that does not keep inventory integrated is read exactly as before, whatever the
    /// closing term says: the whole result equals the one the ledgers alone give.
    #[test]
    fn a_book_that_is_not_integrated_keeps_the_ledger_path() {
        let mut b = book();
        b.closing_stock = ClosingStock::Valued(999_999);
        let r = run(&b, &rules(), &interest(), None).unwrap();
        assert_eq!(
            format!("{r:?}"),
            format!("{:?}", run(&book(), &rules(), &interest(), None).unwrap())
        );
        assert_eq!(fig(&r, "closing_stock"), Value::Int(550_000));
        assert_eq!(fig(&r, "gross_profit"), Value::Int(630_000));
    }

    /// On the ledger basis the two limits are the balance-sheet basis and the closing-stock
    /// sentence, whole and in that order, with or without a report part to tie to. This is the test
    /// book of the closing-stock wording difference declared in `tests/fixtures/PROVENANCE.md`.
    #[test]
    fn a_ledger_stock_book_says_its_closing_stock_is_what_the_client_recorded() {
        let ledger_limits = [
            "The gross profit ratio uses the stock values in the books (balance-sheet basis) at both \
ends. A ratio on Tally's item-level Stock Summary values is not shown here; if one is wanted, both \
opening and closing stock must be taken on that same basis.",
            "Closing stock is the Stock-in-Hand ledger balance the client recorded in the books \
(accounts not integrated with inventory), not an independently counted or valued physical stock \
figure; the books cannot show whether it was actually verified.",
        ];
        let b = book();
        assert_eq!(stock_basis(&b, false), StockBasis::Ledgers);

        let without = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
        assert_eq!(without.findings[0].limits.len(), 3);
        assert_eq!(without.findings[0].limits[..2], ledger_limits);

        let with = run(
            &b,
            &rules(),
            &BTreeSet::new(),
            Some(&report(480_000, Some(550_000))),
        )
        .unwrap();
        assert_eq!(with.findings[0].limits, ledger_limits);
    }

    /// A term that is not known is reported and never read as zero: no gross or net profit, no
    /// ratio on them, and a limit that names the term and why. The known term and the rest stay.
    #[test]
    fn an_unknown_stock_term_shows_no_profit_and_says_which() {
        let report_less = |opening, closing| {
            let b = item_book(opening, closing);
            let r = run(&b, &rules(), &interest(), None).unwrap();
            for name in NO_PROFIT {
                assert!(!has_figure(&r, name), "{name} must not be shown");
            }
            assert_eq!(fig(&r, "sales"), Value::Int(1_000_000));
            assert_eq!(fig(&r, "partner_interest"), Value::Int(40_000));
            assert!(check_invariants(&b, &r).unwrap().is_empty());
            r
        };
        let opening_unknown = report_less(
            OpeningStock::Unknown(OpeningStockUnknown::NotAtBooksStart),
            ClosingStock::Valued(300_000),
        );
        assert!(!has_figure(&opening_unknown, "opening_stock"));
        assert_eq!(fig(&opening_unknown, "closing_stock"), Value::Int(300_000));
        assert_eq!(
            fig(&opening_unknown, "stock_to_turnover_pct_bp"),
            Value::Int(3000)
        );
        let f = &opening_unknown.findings[0];
        assert_eq!(
            f.title,
            "Report tie not performed on net profit: the stock is not known, so no gross profit or \
net profit was derived"
        );
        assert_eq!(f.confidence, Confidence::NeedsDocument);
        assert_eq!(f.limits.len(), 4);
        assert_eq!(
            f.limits[3],
            "Opening stock is held on the stock items, not on a ledger, and was not taken: the read \
does not show the trial balance starting on the books' first day; no gross profit or net profit is \
shown."
        );
        assert_eq!(
            f.ask_client,
            ["Tally's Stock Summary as at the first day of the books, for the opening stock."]
        );
        assert!(!f
            .facts
            .iter()
            .any(|(k, _)| k == "opening_stock" || k == "gross_profit" || k == "net_profit"));

        let closing_unknown = report_less(
            OpeningStock::Valued(200_000),
            ClosingStock::Unknown(ClosingStockUnknown::NoSummaryAtPeriodEnd),
        );
        assert_eq!(fig(&closing_unknown, "opening_stock"), Value::Int(200_000));
        assert!(!has_figure(&closing_unknown, "closing_stock"));
        assert!(!has_figure(&closing_unknown, "stock_to_turnover_pct_bp"));
        let f = &closing_unknown.findings[0];
        assert_eq!(
            f.limits[3],
            "Closing stock is held on the stock items, not on a ledger, and was not taken: the read \
has no Stock Summary dated the trial balance's last day; no gross profit or net profit is shown."
        );
        assert_eq!(
            f.ask_client,
            ["Tally's Stock Summary as at the last day of the period, for the closing stock."]
        );

        let both = report_less(
            OpeningStock::Unknown(OpeningStockUnknown::NotRead),
            ClosingStock::Unknown(ClosingStockUnknown::Unreadable),
        );
        assert!(!has_figure(&both, "opening_stock"));
        assert!(!has_figure(&both, "closing_stock"));
        let f = &both.findings[0];
        assert_eq!(f.limits.len(), 5);
        assert!(f.limits[3].starts_with(
            "Opening stock is held on the stock items, not on a ledger, and was not read;"
        ));
        assert!(f.limits[4].starts_with(
            "Closing stock is held on the stock items, not on a ledger, and was unreadable;"
        ));
        assert_eq!(f.ask_client.len(), 2);

        // A hand-built book with only an opening term is not read as having a closing one.
        let half = item_book(OpeningStock::Valued(200_000), ClosingStock::NotApplicable);
        let r = run(&half, &rules(), &BTreeSet::new(), None).unwrap();
        assert!(!has_figure(&r, "gross_profit"));
        assert!(r.findings[0].limits[3].starts_with(
            "Closing stock is held on the stock items, not on a ledger, and was not read;"
        ));
    }

    /// Stock on a Stock-in-Hand ledger beside stock on the items is never added to it: the profit
    /// is not shown and the limit says why. The known terms still show.
    #[test]
    fn a_stock_in_hand_ledger_beside_the_items_is_not_added() {
        let mut b = book();
        b.opening_stock = OpeningStock::Valued(200_000);
        b.closing_stock = ClosingStock::Valued(300_000);
        let r = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
        for name in NO_PROFIT {
            assert!(!has_figure(&r, name), "{name} must not be shown");
        }
        assert_eq!(fig(&r, "opening_stock"), Value::Int(200_000));
        assert_eq!(fig(&r, "closing_stock"), Value::Int(300_000));
        assert_eq!(
            r.findings[0].limits[3],
            "A ledger under Stock-in-Hand carries a value on a book that keeps its stock on the \
stock items, and the two were not added, in case the stock is counted twice; no gross profit or net \
profit is shown."
        );
        assert_eq!(
            r.findings[0].ask_client,
            ["Whether the Stock-in-Hand ledger carries stock that the stock items also hold."]
        );
        assert!(check_invariants(&b, &r).unwrap().is_empty());
        // A Stock-in-Hand ledger that carries nothing does not stop the items being used.
        for name in ["Stale Stock", "Still Stock"] {
            *b.tb.get_mut(name).unwrap() = TbRow {
                opening_paise: 0,
                debit_paise: 0,
                credit_paise: 0,
                closing_paise: 0,
            };
        }
        let r = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
        assert_eq!(fig(&r, "gross_profit"), Value::Int(480_000));
    }

    /// FS-1 ties the item-based figures to Tally's report as it tied the ledger-based ones.
    #[test]
    fn the_report_tie_works_on_the_item_basis() {
        let b = item_book(OpeningStock::Valued(200_000), ClosingStock::Valued(300_000));
        let r = run(
            &b,
            &rules(),
            &BTreeSet::new(),
            Some(&report(330_000, Some(300_000))),
        )
        .unwrap();
        assert_eq!(
            fig(&r, "report_tie_status"),
            Value::Text("performed: net profit and closing stock".to_string())
        );
        assert_eq!(fig(&r, "report_net_profit_diff"), Value::Int(0));
        assert_eq!(fig(&r, "report_closing_stock_diff"), Value::Int(0));
        assert_eq!(r.findings[0].confidence, Confidence::Computed);
        assert!(r.findings[0].limits[0]
            .starts_with("The gross profit ratio uses the stock items' values"));
        // How a Summary row with no value, and a Summary with no row, read: said beside the rule.
        assert_eq!(r.findings[0].limits.len(), 3);
        assert_eq!(
            r.findings[0].limits[2],
            "A stock item with no value counts as nothing, so a Stock Summary whose items all lack a \
closing value shows a closing stock of zero; a Summary that lists no item at all is not taken as zero."
        );
        assert!(check_invariants(&b, &r).unwrap().is_empty());

        let off = run(
            &b,
            &rules(),
            &BTreeSet::new(),
            Some(&report(430_000, Some(250_000))),
        )
        .unwrap();
        assert_eq!(off.findings[0].confidence, Confidence::JudgementRequired);
        let v = check_invariants(&b, &off).unwrap();
        assert_eq!(v.len(), 2, "{v:?}");
        assert!(v[0].starts_with("FS-1: derived net_profit (330000p)"));
        assert!(v[1].starts_with("FS-1: derived closing_stock (300000p)"));

        // Without a report, the tie is not performed and the title does not say the stock is a
        // ledger's.
        let r = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
        assert!(r.findings[0]
            .title
            .ends_with("from the Trial Balance and the stock items only"));
    }

    /// With the net profit unknown, only a closing stock tie can be performed, and the status
    /// says exactly that.
    #[test]
    fn a_report_beside_an_unknown_term_ties_only_what_is_known() {
        let opening_unknown = item_book(
            OpeningStock::Unknown(OpeningStockUnknown::NotRead),
            ClosingStock::Valued(300_000),
        );
        let r = run(
            &opening_unknown,
            &rules(),
            &BTreeSet::new(),
            Some(&report(330_000, Some(300_000))),
        )
        .unwrap();
        assert_eq!(
            fig(&r, "report_tie_status"),
            Value::Text("performed: closing stock only".to_string())
        );
        assert_eq!(fig(&r, "report_net_profit"), Value::Int(330_000));
        assert!(!has_figure(&r, "report_net_profit_diff"));
        assert_eq!(fig(&r, "report_closing_stock_diff"), Value::Int(0));
        assert!(check_invariants(&opening_unknown, &r).unwrap().is_empty());

        for (b, rep) in [
            (&opening_unknown, report(330_000, None)),
            (
                &item_book(
                    OpeningStock::Valued(200_000),
                    ClosingStock::Unknown(ClosingStockUnknown::NotRead),
                ),
                report(330_000, Some(300_000)),
            ),
        ] {
            let r = run(b, &rules(), &BTreeSet::new(), Some(&rep)).unwrap();
            assert_eq!(
                fig(&r, "report_tie_status"),
                Value::Text(
                    "not performed: the stock is not known, so no net profit was derived"
                        .to_string()
                )
            );
            assert!(!has_figure(&r, "report_closing_stock_diff"));
            assert_eq!(r.findings[0].confidence, Confidence::NeedsDocument);
        }
    }

    /// FS-2 follows the book's own terms: a profit published for a book whose stock is unknown,
    /// or one the item terms do not support, is caught.
    #[test]
    fn fs2_follows_the_stock_basis() {
        let items = item_book(OpeningStock::Valued(200_000), ClosingStock::Valued(300_000));
        let mut r = run(&items, &rules(), &BTreeSet::new(), None).unwrap();
        for f in &mut r.figures {
            if f.id.ends_with(".net_profit") {
                f.value = Value::Int(380_000); // the figure with both stock terms read as zero
            }
            if f.id.ends_with(".closing_stock") {
                f.value = Value::Int(0);
            }
        }
        let v = check_invariants(&items, &r).unwrap();
        assert_eq!(v.len(), 2, "{v:?}");
        assert!(v[0].contains("FS-2: fresh recompute of net_profit (330000p)"));
        assert!(v[1].contains("FS-2: fresh recompute of closing_stock (300000p)"));

        let unknown = item_book(
            OpeningStock::Valued(200_000),
            ClosingStock::Unknown(ClosingStockUnknown::NotRead),
        );
        let honest = run(&unknown, &rules(), &BTreeSet::new(), None).unwrap();
        let mut forged = honest.clone();
        forged.figures.extend(
            run(&items, &rules(), &BTreeSet::new(), None)
                .unwrap()
                .figures
                .into_iter()
                .filter(|f| f.id.ends_with(".net_profit") || f.id.ends_with(".closing_stock")),
        );
        let v = check_invariants(&unknown, &forged).unwrap();
        assert_eq!(v.len(), 2, "{v:?}");
        assert!(v[0].contains(
            "FS-2: net_profit (330000p) is published although a stock term is not known"
        ));
        assert!(
            v[1].contains("FS-2: closing_stock (300000p) is published although it is not known")
        );

        // And a profit left out of a book whose terms are all known.
        let mut dropped = run(&items, &rules(), &BTreeSet::new(), None).unwrap();
        dropped.figures.retain(|f| !f.id.ends_with(".net_profit"));
        let v = check_invariants(&items, &dropped).unwrap();
        assert_eq!(v.len(), 1, "{v:?}");
        assert!(v[0].contains("FS-2: net_profit is not published although the stock terms are known (fresh recompute 330000p)"));
    }

    /// A Stock-in-Hand ledger counts as carrying a value by any one of its opening, its
    /// recomputed closing (opening plus debit less credit) or the Trial Balance's own closing
    /// field. Each alone keeps the stock items from being used beside it, and FS-2 agrees.
    #[test]
    fn each_value_a_stock_in_hand_ledger_can_carry_keeps_the_items_from_being_used() {
        let cases = [
            ("only the opening", 5_000, 0, 0, 0),
            ("an opening wholly drawn down", 5_000, 0, 5_000, 0),
            ("only a debit movement", 0, 5_000, 0, 0),
            ("only a credit movement", 0, 0, 5_000, 0),
            ("only the TB's own closing field", 0, 0, 0, 5_000),
        ];
        for (what, opening_paise, debit_paise, credit_paise, closing_paise) in cases {
            let mut b = item_book(OpeningStock::Valued(200_000), ClosingStock::Valued(300_000));
            b.ledgers.insert(
                "Stock Ledger".to_string(),
                ledger("Stock Ledger", &["Stock-in-Hand", "Current Assets"]),
            );
            b.tb.insert(
                "Stock Ledger".to_string(),
                TbRow {
                    opening_paise,
                    debit_paise,
                    credit_paise,
                    closing_paise,
                },
            );
            let r = run(&b, &rules(), &BTreeSet::new(), None).unwrap();
            for name in NO_PROFIT {
                assert!(!has_figure(&r, name), "{what}: {name} must not be shown");
            }
            assert!(
                r.findings[0].limits[3].starts_with("A ledger under Stock-in-Hand carries a value"),
                "{what}: {:?}",
                r.findings[0].limits
            );
            assert!(check_invariants(&b, &r).unwrap().is_empty(), "{what}");
        }
    }

    /// The registry's figure floor is exactly what a book shows when neither stock term is known,
    /// and a book on the ledger basis shows at least the 18 the floor used to be, so a ledger
    /// book that loses figures does not hide behind the lower floor.
    #[test]
    fn the_figure_floor_is_what_a_book_with_no_stock_term_shows() {
        let floor = crate::registry::find(TEST_ID).unwrap().min_figures;
        let mut none = item_book(
            OpeningStock::Unknown(OpeningStockUnknown::NotRead),
            ClosingStock::Unknown(ClosingStockUnknown::NotRead),
        );
        none.vouchers.clear(); // no excluded-voucher figures on top
        let r = run(&none, &rules(), &BTreeSet::new(), None).unwrap();
        assert_eq!(
            r.figures.len(),
            floor,
            "{:?}",
            r.figures.iter().map(|f| &f.id).collect::<Vec<_>>()
        );

        let mut ledgers = book();
        ledgers.vouchers.clear();
        let r = run(&ledgers, &rules(), &BTreeSet::new(), None).unwrap();
        assert!(r.figures.len() >= 18, "{}", r.figures.len());
    }
}
