// SPDX-License-Identifier: Apache-2.0
//! Form 3CD clause 16(a) and (d), the credits to capital, reserve, loan, liability, tax and
//! suspense ledgers that may be amounts not credited to the profit and loss account, listed for
//! the CA and never judged. A crate-first test: the reference engine has no such module, so there
//! is no reference golden to compare it with (`docs/tax-audit/parity-spec-v1.md`, section 12).
//!
//! The population is limited by structure in the book only: which head a ledger sits under, and
//! what each voucher debits and credits. Nothing is left out for its size or for what a text says,
//! and whatever is left out is counted in a figure of its own:
//! - a voucher none of whose credits is listed, because every debit is on a profit and loss,
//!   asset or investment head, or on Duties & Taxes, with at least one on the former;
//! - a Duties & Taxes credit on a voucher that also credits an income ledger or debits a creditor
//!   (tax collected on a sale, or withheld on a payment);
//! - a ledger whose head this cannot tell, because its chain is incomplete or ends at a group
//!   that is not one of Tally's reserved primary groups.
//!
//! Every other credit to a head ledger is one row per voucher and ledger. Whether it is income,
//! capital, a loan, a liability or tax, the books do not say; each row is the CA's to decide.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::{Book, Ledger, Voucher, PRIMARY_GROUPS};
use crate::error::Result;
use crate::financial_statements::{CREDIT_BALANCE_GROUPS, PL_GROUPS};
use crate::findings::{Confidence, EvidenceRef, Finding, TestResult, Unit, Value};
use crate::rules::Rules;
use crate::support::{count, overflow, voucher_label, voucher_refs};

pub const TEST_ID: &str = "clause16_credits";
pub const VERSION: &str = "1";

const POPULATION: &str = "Books population (optional, cancelled and post-dated vouchers excluded)";

/// The primary groups whose ledgers' credits are listed, with every group under them.
const HEAD_GROUPS: [&str; 4] = [
    "Capital Account",
    "Loans (Liability)",
    "Current Liabilities",
    "Suspense A/c",
];
/// The heads, beside the profit and loss groups, a debit to which can exclude a voucher.
const ASSET_GROUPS: [&str; 3] = ["Fixed Assets", "Investments", "Misc. Expenses (ASSET)"];
/// Tally's reserved ledger, directly under the reserved root (book.rs, `chain`).
const RESERVED_PL_LEDGER: &str = "Profit & Loss A/c";
const TAX_GROUP: &str = "Duties & Taxes";
/// An overdraft is a money account, though Tally keeps it under Loans (Liability).
const OVERDRAFT_GROUP: &str = "Bank OD A/c";
const CREDITOR_GROUP: &str = "Sundry Creditors";

/// What a ledger is to this test, decided once per ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// The engagement's cash and bank ledgers, and an overdraft.
    Money,
    /// A profit and loss, asset or investment head, or Tally's reserved `Profit & Loss A/c`.
    Excluded,
    /// Under Duties & Taxes: its debits are neutral to the exclusion.
    Tax,
    /// Under Capital Account, Loans (Liability), Current Liabilities or Suspense A/c.
    Head,
    /// A head this cannot tell: an incomplete chain, or one ending at a user group under Primary.
    Untested,
    /// Everything else: debtors, current assets, and the like.
    Other,
}

/// A ledger's role. `money` holds the engagement's cash and bank ledgers and any overdraft.
pub fn role(ledger: &Ledger, money: &BTreeSet<String>) -> Role {
    let reserved_pl = ledger.name == RESERVED_PL_LEDGER
        && matches!(ledger.chain.as_slice(), [root] if crate::xml::is_reserved_root(root));
    if money.contains(&ledger.name) {
        Role::Money
    } else if reserved_pl {
        Role::Excluded
    } else if !ledger.chain_complete
        || !ledger
            .chain
            .last()
            .is_some_and(|g| PRIMARY_GROUPS.contains(&g.as_str()))
    {
        Role::Untested
    } else if PL_GROUPS
        .iter()
        .chain(&ASSET_GROUPS)
        .any(|g| ledger.under(g))
    {
        Role::Excluded
    } else if ledger.under(TAX_GROUP) {
        Role::Tax
    } else if HEAD_GROUPS.iter().any(|g| ledger.under(g)) {
        Role::Head
    } else {
        Role::Other
    }
}

/// The cash and bank ledgers, and every ledger under Bank OD A/c.
pub fn money_ledgers(
    book: &Book,
    cash: &BTreeSet<String>,
    bank: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut money = book.ledgers_under_any(&[OVERDRAFT_GROUP.to_string()]);
    money.extend(cash.iter().cloned());
    money.extend(bank.iter().cloned());
    money
}

/// One listed credit: a head ledger, the credits to it on one voucher summed, and the voucher's
/// debit ledgers in the order its lines name them.
struct Row<'a> {
    voucher: &'a Voucher,
    ledger: &'a str,
    credit: i64,
    debits: Vec<&'a str>,
}

/// A credit's amount, positive; `None` past the 64-bit range.
fn credit_of(amount_paise: i64) -> Option<i64> {
    amount_paise.checked_neg()
}

fn total(values: impl Iterator<Item = i64>) -> Result<i64> {
    let mut sum = 0;
    for paise in values {
        add(&mut sum, paise)?;
    }
    Ok(sum)
}

fn add(sum: &mut i64, paise: i64) -> Result<()> {
    *sum = sum.checked_add(paise).ok_or_else(|| overflow(TEST_ID))?;
    Ok(())
}

pub fn run(book: &Book, rules: &Rules, money: &BTreeSet<String>) -> Result<TestResult> {
    let mut r = TestResult::new(TEST_ID, VERSION, &rules.version);
    let role_of = |name: &str| {
        book.ledgers
            .get(name)
            .map_or(Role::Untested, |l| role(l, money))
    };
    let mut rows: Vec<Row> = Vec::new();
    let (mut excluded, mut excluded_credit): (Vec<&Voucher>, i64) = (Vec::new(), 0);
    let (mut taxed, mut taxed_credit): (Vec<&Voucher>, i64) = (Vec::new(), 0);
    let mut untested: BTreeMap<&str, i64> = BTreeMap::new();
    for v in book.population()? {
        let debits: Vec<_> = v.lines.iter().filter(|l| l.amount_paise > 0).collect();
        let is_excluded = debits
            .iter()
            .all(|l| matches!(role_of(&l.ledger), Role::Excluded | Role::Tax))
            && debits.iter().any(|l| role_of(&l.ledger) == Role::Excluded);
        let under_any = |ledger: &str, groups: &[&str]| {
            book.ledgers
                .get(ledger)
                .is_some_and(|l| groups.iter().any(|g| l.under(g)))
        };
        let tax_left_out = v
            .lines
            .iter()
            .any(|l| l.amount_paise < 0 && under_any(&l.ledger, &CREDIT_BALANCE_GROUPS))
            || debits
                .iter()
                .any(|l| under_any(&l.ledger, &[CREDITOR_GROUP]));
        let mut debit_names: Vec<&str> = Vec::new();
        for l in &debits {
            if !debit_names.contains(&l.ledger.as_str()) {
                debit_names.push(&l.ledger);
            }
        }
        let (mut excluded_here, mut taxed_here) = (false, false);
        let mut listed: BTreeMap<&str, i64> = BTreeMap::new();
        for l in v.lines.iter().filter(|l| l.amount_paise < 0) {
            let credit = credit_of(l.amount_paise).ok_or_else(|| overflow(TEST_ID))?;
            match role_of(&l.ledger) {
                Role::Untested => add(untested.entry(&l.ledger).or_insert(0), credit)?,
                Role::Head | Role::Tax if is_excluded => {
                    excluded_here = true;
                    add(&mut excluded_credit, credit)?;
                }
                Role::Tax if tax_left_out => {
                    taxed_here = true;
                    add(&mut taxed_credit, credit)?;
                }
                Role::Head | Role::Tax => add(listed.entry(&l.ledger).or_insert(0), credit)?,
                Role::Money | Role::Excluded | Role::Other => {}
            }
        }
        if excluded_here {
            excluded.push(v);
        }
        if taxed_here {
            taxed.push(v);
        }
        rows.extend(listed.into_iter().map(|(ledger, credit)| Row {
            voucher: v,
            ledger,
            credit,
            debits: debit_names.clone(),
        }));
    }
    rows.sort_by(|a, b| {
        (
            a.voucher.date.as_str(),
            voucher_label(a.voucher),
            &a.voucher.guid,
            a.ledger,
        )
            .cmp(&(
                b.voucher.date.as_str(),
                voucher_label(b.voucher),
                &b.voucher.guid,
                b.ledger,
            ))
    });

    r.population_note = POPULATION.to_string();
    let rows_id = r.fig(
        "rows",
        count(TEST_ID, rows.len())?,
        Unit::Count,
        &format!(
            "Credits to capital, reserve, loan, liability, tax and suspense ledgers, one row per voucher and ledger. {POPULATION}"
        ),
        voucher_refs(rows.iter().map(|row| row.voucher)),
    )?;
    let row_credit_id = r.fig(
        "row_credit",
        Value::Int(total(rows.iter().map(|row| row.credit))?),
        Unit::Paise,
        "The listed credits, each row's own.",
        voucher_refs(rows.iter().map(|row| row.voucher)),
    )?;
    for (index, row) in rows.iter().enumerate() {
        let debits = if row.debits.is_empty() {
            "no ledger".to_string()
        } else {
            row.debits.join(", ")
        };
        let mut evidence = voucher_refs([row.voucher]);
        evidence.push(EvidenceRef::new("ledger", row.ledger));
        r.fig(
            &format!("row_{}", index + 1),
            Value::Int(row.credit),
            Unit::Paise,
            &format!(
                "Credits to {} on {}; the voucher debits {debits}.",
                row.ledger,
                voucher_label(row.voucher)
            ),
            evidence,
        )?;
    }
    let excluded_id = r.fig(
        "excluded_vouchers",
        count(TEST_ID, excluded.len())?,
        Unit::Count,
        "Vouchers whose head credits are not listed: every debit is on a profit and loss, asset or investment head or on Duties & Taxes, and at least one on the former.",
        voucher_refs(excluded.iter().copied()),
    )?;
    let excluded_credit_id = r.fig(
        "excluded_credit",
        Value::Int(excluded_credit),
        Unit::Paise,
        "The head credits on those vouchers.",
        voucher_refs(excluded.iter().copied()),
    )?;
    let taxed_id = r.fig(
        "tax_left_out",
        count(TEST_ID, taxed.len())?,
        Unit::Count,
        "Vouchers whose Duties & Taxes credits are not listed: the voucher also credits a sales or income ledger, or debits a creditor.",
        voucher_refs(taxed.iter().copied()),
    )?;
    let taxed_credit_id = r.fig(
        "tax_left_out_credit",
        Value::Int(taxed_credit),
        Unit::Paise,
        "The Duties & Taxes credits on those vouchers.",
        voucher_refs(taxed.iter().copied()),
    )?;
    let names: Vec<&str> = untested.keys().copied().collect();
    let ledger_refs = || -> Vec<EvidenceRef> {
        names
            .iter()
            .map(|n| EvidenceRef::new("ledger", n))
            .collect()
    };
    let named = if names.is_empty() {
        String::new()
    } else {
        format!(" Ledgers: {}.", names.join(", "))
    };
    let untested_id = r.fig(
        "untested_ledgers",
        count(TEST_ID, names.len())?,
        Unit::Count,
        &format!(
            "Credited ledgers whose head is not told: an incomplete group chain, or one ending at a group that is not one of Tally's reserved primary groups. Their credits are not listed.{named}"
        ),
        ledger_refs(),
    )?;
    let untested_credit_id = r.fig(
        "untested_credit",
        Value::Int(total(untested.values().copied())?),
        Unit::Paise,
        "The credits to those ledgers.",
        ledger_refs(),
    )?;
    r.findings.push(Finding {
        id: format!("{TEST_ID}/candidates"),
        clauses: vec!["3CD-16(a)".to_string(), "3CD-16(d)".to_string()],
        title: "Credits to capital, reserve, loan, liability, tax and suspense ledgers".to_string(),
        facts: vec![
            ("rows".to_string(), rows_id),
            ("row_credit".to_string(), row_credit_id),
            ("excluded_vouchers".to_string(), excluded_id),
            ("excluded_credit".to_string(), excluded_credit_id),
            ("tax_left_out".to_string(), taxed_id),
            ("tax_left_out_credit".to_string(), taxed_credit_id),
            ("untested_ledgers".to_string(), untested_id),
            ("untested_credit".to_string(), untested_credit_id),
        ],
        evidence: voucher_refs(rows.iter().map(|row| row.voucher)),
        confidence: Confidence::JudgementRequired,
        limits: vec![
            "Books only: whether a credit is income, capital, a loan, a liability or tax is not established."
                .to_string(),
            "Group nature is not read, so a ledger under a group the user made under Primary is counted, not tested."
                .to_string(),
            "Bill-wise allocations are not read.".to_string(),
            "Receipts in Form 26AS or the AIS are not compared here.".to_string(),
            "Credits to Fixed Assets or Investments are not listed.".to_string(),
            "A journal whose every line is on Duties & Taxes, such as a GST set-off, is listed."
                .to_string(),
        ],
        ask_client: Vec::new(),
    });
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::{LedgerLine, VoucherStatus};
    use crate::error::AuditError;
    use bridge_tally_primitives::TallyDate;

    /// Tally's reserved root as `chain` keeps it: the decoded `&#4; Primary`.
    const RESERVED_ROOT: &str = "\u{fffd}#4; Primary";

    fn ledger(name: &str, chain: &[&str]) -> Ledger {
        Ledger {
            name: name.to_string(),
            parent: chain.first().unwrap_or(&"").to_string(),
            chain: chain.iter().map(|g| g.to_string()).collect(),
            chain_complete: true,
            master_opening_paise: 0,
            guid: String::new(),
            masterid: None,
            pan: String::new(),
            gstin: String::new(),
        }
    }

    /// One ledger per role, as the lab book has them.
    fn ledgers() -> Vec<Ledger> {
        vec![
            ledger("Cash", &["Cash-in-Hand", "Current Assets"]),
            ledger("Bank", &["Bank Accounts", "Current Assets"]),
            ledger("OD", &["Bank OD A/c", "Loans (Liability)"]),
            ledger("Capital", &["Capital Account"]),
            ledger("Reserve", &["Reserves & Surplus", "Capital Account"]),
            ledger("Loan", &["Loans (Liability)"]),
            ledger("Advance", &["Current Liabilities"]),
            ledger("Refund", &["Current Liabilities"]),
            ledger("Creditor", &["Sundry Creditors", "Current Liabilities"]),
            ledger("Suspense", &["Suspense A/c"]),
            ledger("GST", &["Duties & Taxes", "Current Liabilities"]),
            ledger("Input GST", &["Duties & Taxes", "Current Liabilities"]),
            ledger("TDS", &["Duties & Taxes", "Current Liabilities"]),
            ledger("Expense", &["Indirect Expenses"]),
            ledger("Sales", &["Sales Accounts"]),
            ledger("Machine", &["Fixed Assets"]),
            ledger("Debtor", &["Sundry Debtors", "Current Assets"]),
            ledger(RESERVED_PL_LEDGER, &[RESERVED_ROOT]),
            ledger("Grant", &["Grants"]),
        ]
    }

    fn line(ledger: &str, amount_paise: i64) -> LedgerLine {
        LedgerLine {
            ledger: ledger.to_string(),
            amount_paise,
        }
    }

    fn voucher(guid: &str, date: &str, lines: Vec<LedgerLine>) -> Voucher {
        Voucher {
            guid: guid.to_string(),
            date: TallyDate::parse(date).unwrap(),
            vtype: "Journal".to_string(),
            base_type: "Journal".to_string(),
            number: guid.to_string(),
            status: VoucherStatus::Regular,
            lines,
            ..Voucher::default()
        }
    }

    fn book_with(ledgers: Vec<Ledger>, vouchers: Vec<Voucher>) -> Book {
        Book {
            company_name: "Synthetic".to_string(),
            company_guid: "test-guid".to_string(),
            read_at: String::new(),
            groups: BTreeMap::new(),
            group_masters: BTreeMap::new(),
            ledgers: ledgers.into_iter().map(|l| (l.name.clone(), l)).collect(),
            stock: None,
            currency_read: false,
            opening_stock: crate::book::OpeningStock::NotApplicable,
            closing_stock: crate::book::ClosingStock::NotApplicable,
            vouchers,
            tb: BTreeMap::new(),
        }
    }

    fn book(vouchers: Vec<Voucher>) -> Book {
        book_with(ledgers(), vouchers)
    }

    fn run_on(book: &Book) -> Result<TestResult> {
        let one = |name: &str| BTreeSet::from([name.to_string()]);
        let money = money_ledgers(book, &one("Cash"), &one("Bank"));
        run(book, &Rules::vendored().unwrap(), &money)
    }

    fn figure(r: &TestResult, name: &str) -> i64 {
        let id = format!("{TEST_ID}.{name}");
        match r.figures.iter().find(|f| f.id == id).map(|f| &f.value) {
            Some(Value::Int(n)) => *n,
            other => panic!("{id}: {other:?}"),
        }
    }

    /// Each row as (voucher GUID, ledger, paise), in the result's order.
    fn rows(r: &TestResult) -> Vec<(String, String, i64)> {
        (1..=figure(r, "rows"))
            .map(|n| {
                let f = r
                    .figures
                    .iter()
                    .find(|f| f.id == format!("{TEST_ID}.row_{n}"))
                    .unwrap();
                let Value::Int(paise) = f.value else {
                    panic!("{}", f.id)
                };
                (f.evidence[0].id.clone(), f.evidence[1].id.clone(), paise)
            })
            .collect()
    }

    fn row(guid: &str, ledger: &str, paise: i64) -> (String, String, i64) {
        (guid.to_string(), ledger.to_string(), paise)
    }

    /// The counted figures, in the finding's order after the rows.
    fn counted(r: &TestResult) -> [i64; 6] {
        [
            "excluded_vouchers",
            "excluded_credit",
            "tax_left_out",
            "tax_left_out_credit",
            "untested_ledgers",
            "untested_credit",
        ]
        .map(|name| figure(r, name))
    }

    #[test]
    fn each_ledger_takes_the_first_role_that_fits() {
        let b = book(Vec::new());
        let money = money_ledgers(
            &b,
            &BTreeSet::from(["Cash".to_string()]),
            &BTreeSet::from(["Bank".to_string()]),
        );
        let role_of = |name: &str| role(&b.ledgers[name], &money);
        for (name, expected) in [
            ("Cash", Role::Money),
            ("Bank", Role::Money),
            ("OD", Role::Money),
            ("Capital", Role::Head),
            ("Reserve", Role::Head),
            ("Loan", Role::Head),
            ("Creditor", Role::Head),
            ("Suspense", Role::Head),
            ("GST", Role::Tax),
            ("Expense", Role::Excluded),
            ("Sales", Role::Excluded),
            ("Machine", Role::Excluded),
            (RESERVED_PL_LEDGER, Role::Excluded),
            ("Debtor", Role::Other),
            ("Grant", Role::Untested),
        ] {
            assert_eq!(role_of(name), expected, "{name}");
        }
        let mut broken = ledger("Broken", &["Indirect Expenses"]);
        broken.chain_complete = false;
        assert_eq!(role(&broken, &money), Role::Untested);
        let bare_primary = ledger("Odd", &["Primary"]);
        assert_eq!(role(&bare_primary, &money), Role::Untested);
    }

    /// Capital by bank, a liability by cash, a refund by bank, and a loan: each one row.
    #[test]
    fn a_head_credit_against_money_is_one_row() {
        let r = run_on(&book(vec![
            voucher(
                "c1",
                "20250630",
                vec![line("Bank", 500), line("Capital", -500)],
            ),
            voucher(
                "s15",
                "20250701",
                vec![line("Cash", 220), line("Advance", -220)],
            ),
            voucher(
                "c4",
                "20251020",
                vec![line("Bank", 800), line("Refund", -800)],
            ),
            voucher(
                "loan",
                "20251101",
                vec![line("Cash", 230), line("Loan", -230)],
            ),
        ]))
        .unwrap();
        assert_eq!(
            rows(&r),
            [
                row("c1", "Capital", 500),
                row("s15", "Advance", 220),
                row("c4", "Refund", 800),
                row("loan", "Loan", 230),
            ]
        );
        assert_eq!(figure(&r, "row_credit"), 1750);
        assert_eq!(counted(&r), [0; 6]);
        assert_eq!(r.findings[0].confidence, Confidence::JudgementRequired);
        assert!(r.findings[0].ask_client.is_empty());
    }

    /// An expense debit beside a bank debit does not exclude, and the row names each debit ledger
    /// once, in line order.
    #[test]
    fn a_money_debit_beside_an_expense_keeps_the_row_and_names_every_debit() {
        let r = run_on(&book(vec![voucher(
            "c16",
            "20260210",
            vec![
                line("Bank", 100),
                line("Expense", 50),
                line("Bank", 50),
                line("Advance", -200),
            ],
        )]))
        .unwrap();
        assert_eq!(rows(&r), [row("c16", "Advance", 200)]);
        let row_1 = r.figures.iter().find(|f| f.id.ends_with(".row_1")).unwrap();
        assert!(
            row_1
                .definition
                .ends_with("the voucher debits Bank, Expense."),
            "{}",
            row_1.definition
        );
        assert_eq!(counted(&r), [0; 6]);
    }

    /// Every debit on an expense, or on an expense and input tax, excludes the voucher; every debit
    /// on tax alone does not, and a set-off journal (every line tax) is listed.
    #[test]
    fn the_exclusion_needs_every_debit_excluded_or_tax_and_one_excluded() {
        let r = run_on(&book(vec![
            voucher(
                "c6",
                "20250801",
                vec![line("Expense", 400), line("Creditor", -400)],
            ),
            voucher(
                "c19",
                "20250802",
                vec![
                    line("Expense", 100),
                    line("Input GST", 18),
                    line("Creditor", -118),
                ],
            ),
            voucher(
                "tax-only",
                "20250803",
                vec![line("Input GST", 18), line("Creditor", -18)],
            ),
            voucher(
                "set-off",
                "20250804",
                vec![line("GST", 18), line("Input GST", -18)],
            ),
            voucher(
                "machine",
                "20250805",
                vec![line("Machine", 900), line("Loan", -900)],
            ),
            voucher(
                "pl",
                "20250806",
                vec![line(RESERVED_PL_LEDGER, 70), line("Reserve", -70)],
            ),
            voucher(
                "no-head",
                "20250807",
                vec![line("Expense", 5), line("Debtor", -5)],
            ),
            voucher(
                "withheld",
                "20250808",
                vec![line("Expense", 100), line("Creditor", -95), line("TDS", -5)],
            ),
        ]))
        .unwrap();
        assert_eq!(
            rows(&r),
            [
                row("tax-only", "Creditor", 18),
                row("set-off", "Input GST", 18),
            ]
        );
        // c6, c19, machine, pl and withheld, its tax credit too; no-head credits nothing that
        // would be listed, so it is not counted.
        assert_eq!(counted(&r), [5, 400 + 118 + 900 + 70 + 100, 0, 0, 0, 0]);
    }

    /// Tax on a voucher that credits income or debits a creditor is counted; tax beside a debtor
    /// or an income debit is listed; and a creditor paid with no tax credit is not counted.
    #[test]
    fn tax_on_a_sale_or_withheld_from_a_creditor_is_counted() {
        let r = run_on(&book(vec![
            voucher(
                "c17",
                "20250901",
                vec![line("Creditor", 100), line("Bank", -95), line("TDS", -5)],
            ),
            voucher(
                "c18",
                "20250902",
                vec![line("Debtor", 118), line("Sales", -100), line("GST", -18)],
            ),
            voucher(
                "plain",
                "20250903",
                vec![line("Debtor", 18), line("GST", -18)],
            ),
            voucher(
                "return",
                "20250904",
                vec![line("Sales", 10), line("Debtor", 8), line("GST", -18)],
            ),
            voucher(
                "paid",
                "20250905",
                vec![line("Creditor", 50), line("Bank", -50)],
            ),
        ]))
        .unwrap();
        assert_eq!(
            rows(&r),
            [row("plain", "GST", 18), row("return", "GST", 18)]
        );
        assert_eq!(counted(&r), [0, 0, 2, 23, 0, 0]);
    }

    /// Only the reserved ledger counts as the reserved `Profit & Loss A/c`: a user ledger of the
    /// same name under a group is what its group makes it.
    #[test]
    fn a_user_ledger_named_like_the_reserved_one_is_not_it() {
        let mut ledgers = ledgers();
        ledgers.retain(|l| l.name != RESERVED_PL_LEDGER);
        ledgers.push(ledger(
            RESERVED_PL_LEDGER,
            &["Sundry Debtors", "Current Assets"],
        ));
        let vouchers = vec![voucher(
            "c10",
            "20251001",
            vec![line(RESERVED_PL_LEDGER, 70), line("Reserve", -70)],
        )];
        let r = run_on(&book_with(ledgers, vouchers.clone())).unwrap();
        assert_eq!(rows(&r), [row("c10", "Reserve", 70)]);
        let r = run_on(&book(vouchers)).unwrap();
        assert!(rows(&r).is_empty());
        assert_eq!(counted(&r), [1, 70, 0, 0, 0, 0]);
    }

    /// A credit to a ledger whose head is not told is counted by ledger, never listed, and a debit
    /// to one is an "other" debit (no voucher in the lab read has one).
    #[test]
    fn an_untested_ledger_is_counted_and_its_debit_does_not_exclude() {
        let mut broken = ledger("Broken", &["Capital Account"]);
        broken.chain_complete = false;
        let mut ledgers = ledgers();
        ledgers.push(broken);
        let r = run_on(&book_with(
            ledgers,
            vec![
                voucher(
                    "c13",
                    "20251101",
                    vec![line("Bank", 150), line("Grant", -150)],
                ),
                voucher(
                    "again",
                    "20251102",
                    vec![line("Bank", 10), line("Grant", -10)],
                ),
                voucher(
                    "broken",
                    "20251103",
                    vec![line("Bank", 7), line("Broken", -7)],
                ),
                voucher(
                    "missing",
                    "20251104",
                    vec![line("Bank", 3), line("Nowhere", -3)],
                ),
                voucher(
                    "debit",
                    "20251105",
                    vec![
                        line("Grant", 40),
                        line("Expense", 60),
                        line("Advance", -100),
                    ],
                ),
            ],
        ))
        .unwrap();
        assert_eq!(rows(&r), [row("debit", "Advance", 100)]);
        assert_eq!(counted(&r), [0, 0, 0, 0, 3, 170]);
        let untested = r
            .figures
            .iter()
            .find(|f| f.id.ends_with(".untested_ledgers"))
            .unwrap();
        assert!(untested
            .definition
            .ends_with(" Ledgers: Broken, Grant, Nowhere."));
    }

    /// An overdraft credit is money: nothing listed or counted.
    #[test]
    fn an_overdraft_credit_is_money() {
        let r = run_on(&book(vec![voucher(
            "c9",
            "20251201",
            vec![line("Bank", 900), line("OD", -900)],
        )]))
        .unwrap();
        assert!(rows(&r).is_empty());
        assert_eq!(counted(&r), [0; 6]);
    }

    /// One ledger credited on two lines of a voucher is one row of their sum; two ledgers on one
    /// voucher are two rows, by ledger name; rows run by date, then label, whatever the read order.
    #[test]
    fn rows_are_summed_per_voucher_and_ledger_and_sorted() {
        let numbered = |guid: &str, number: &str, date: &str, lines| Voucher {
            number: number.to_string(),
            ..voucher(guid, date, lines)
        };
        // The numbers order the labels against the GUIDs, so the label key is what decides.
        let r = run_on(&book(vec![
            numbered(
                "a-late",
                "1",
                "20260301",
                vec![line("Bank", 9), line("Capital", -9)],
            ),
            numbered(
                "b-c15",
                "2",
                "20260205",
                vec![
                    line("Bank", 300),
                    line("Capital", -100),
                    line("Capital", -200),
                ],
            ),
            numbered(
                "a-two",
                "3",
                "20260205",
                vec![line("Bank", 30), line("Suspense", -10), line("Loan", -20)],
            ),
            numbered(
                "c-first",
                "1",
                "20260205",
                vec![line("Bank", 1), line("Capital", -1)],
            ),
        ]))
        .unwrap();
        assert_eq!(
            rows(&r),
            [
                row("c-first", "Capital", 1),
                row("b-c15", "Capital", 300),
                row("a-two", "Loan", 20),
                row("a-two", "Suspense", 10),
                row("a-late", "Capital", 9),
            ]
        );
    }

    #[test]
    fn a_total_past_i64_is_refused() {
        let over = |lines| {
            let err = run_on(&book(vec![voucher("v", "20250601", lines)])).unwrap_err();
            assert!(
                matches!(&err, AuditError::Config(m) if m.starts_with("clause16_credits:")),
                "{err}"
            );
        };
        over(vec![line("Capital", -i64::MAX), line("Capital", -i64::MAX)]);
        over(vec![line("Capital", i64::MIN)]);
        over(vec![line("Grant", -i64::MAX), line("Grant", -1)]);
    }

    /// Vouchers outside the books population are not read, and an unknown status refuses.
    #[test]
    fn the_population_is_the_books_population() {
        let mut optional = voucher(
            "opt",
            "20250601",
            vec![line("Bank", 5), line("Capital", -5)],
        );
        optional.status = VoucherStatus::Optional;
        let r = run_on(&book(vec![optional.clone()])).unwrap();
        assert!(rows(&r).is_empty());
        optional.status = VoucherStatus::Unknown;
        assert!(matches!(
            run_on(&book(vec![optional])),
            Err(AuditError::UnknownVoucherStatus(1))
        ));
    }
}
