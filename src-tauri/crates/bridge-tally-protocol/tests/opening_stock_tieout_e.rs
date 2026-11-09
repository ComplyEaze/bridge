//! #692 option B: the 5 Nov 2026 opening-stock run on `BRIDGE TIEOUT E`, read through this crate's
//! own parsers (`tests/fixtures/opening-stock-tieout-e/PROVENANCE.md`).
//!
//! The run's verdicts were printed by the lab's `identities.py` and `relations.py`, which find
//! elements with regular expressions. Each test here reaches the same verdict from the typed
//! values the parsers return, so it rests on structure, not on a substring match (AGENTS.md P5):
//! the statement lines from `parse_native_statement`, the ledgers from
//! `parse_native_trial_balance`, and each ledger's primary group from its ancestry chain over
//! `parse_native_group_snapshot`, by `RESERVEDNAME` rather than by display name.
//!
//! O and C are the sub amounts of the Profit and Loss lines `Opening Stock` and
//! `Less: Closing Stock`. Signs are Tally's: a debit is negative. An empty amount is not zero
//! (AGENTS.md P3). The run's scripts read every empty amount they used as zero, so these tests do
//! the same, but only after [`assert_empties`] has asserted, for each read, exactly which of those
//! amounts the captured answer holds empty ([`EMPTIES`]). An empty amount the table does not name
//! fails the test before any sum is taken.

use bridge_tally_primitives::ExactDecimal;
use bridge_tally_protocol::group_ancestry::GroupIndex;
use bridge_tally_protocol::native_outstandings::parse_native_group_snapshot;
use bridge_tally_protocol::native_statement_reports::{
    parse_native_statement, NativeStatement, NativeStatementAmount, NativeStatementKind,
};
use bridge_tally_protocol::native_trial_balance::{
    parse_native_trial_balance, NativeTrialBalance, NativeTrialBalanceAmount,
};
use bridge_tally_protocol::{decode_tally_xml_response_bytes_limited, ExpectedTallyTextEncoding};

const TIEOUT_E_GUID: &str = "ffa928f6-5934-48e2-bbd6-474936e71546";
const NOMINAL: [&str; 6] = [
    "Sales Accounts",
    "Purchase Accounts",
    "Direct Incomes",
    "Direct Expenses",
    "Indirect Incomes",
    "Indirect Expenses",
];
const BALANCE_SHEET_PRIMARIES: [&str; 7] = [
    "Capital Account",
    "Current Liabilities",
    "Loans (Liability)",
    "Fixed Assets",
    "Investments",
    "Suspense A/c",
    "Misc. Expenses (ASSET)",
];

/// The nine (run, window) reads with ledgers: R1 w1 to w4, R5 w5, R2 w1 and w2, R3 w2, R4 w2.
const READS: [&str; 9] = [
    "r1-w1", "r1-w2", "r1-w3", "r1-w4", "r5-w5", "r2-w1", "r2-w2", "r3-w2", "r4-w2",
];

fn text(name: &str) -> String {
    let path = format!(
        "{}/tests/fixtures/opening-stock-tieout-e/{name}.utf16le.xml",
        env!("CARGO_MANIFEST_DIR")
    );
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{path}: {error}"));
    decode_tally_xml_response_bytes_limited(
        &bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap_or_else(|error| panic!("{name} decodes: {error:?}"))
    .text
}

fn statement(kind: NativeStatementKind, name: &str) -> NativeStatement {
    parse_native_statement(kind, &text(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn ledgers(read: &str) -> NativeTrialBalance {
    parse_native_trial_balance(&text(&format!("{read}-ledgers")), TIEOUT_E_GUID)
        .unwrap_or_else(|error| panic!("{read}-ledgers: {error}"))
}

fn groups() -> GroupIndex {
    GroupIndex::build(
        parse_native_group_snapshot(&text("r1-groups"), TIEOUT_E_GUID).expect("r1-groups parses"),
    )
}

/// For each read: the Profit and Loss amounts the identities use that are empty (`<line> sub` or
/// `<line> main`), the Balance Sheet lines whose main amount is empty, and the ledgers whose
/// closing is empty. Taken from the captured bytes; each is an amount the run's scripts read as zero.
struct Empties {
    read: &'static str,
    pl: &'static [&'static str],
    bs_main: &'static [&'static str],
    tb_closing: &'static [&'static str],
}

const EMPTIES: [Empties; 9] = [
    Empties {
        read: "r1-w1",
        pl: &["Opening Stock sub"],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash"],
    },
    Empties {
        read: "r1-w2",
        pl: &[],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash"],
    },
    Empties {
        read: "r1-w3",
        pl: &[],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash", "Lab Rent"],
    },
    Empties {
        read: "r1-w4",
        pl: &["Add: Purchase Accounts sub", "Cost of Sales : main"],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash", "Lab Purchase", "Lab Sales"],
    },
    Empties {
        read: "r5-w5",
        pl: &["Opening Stock sub"],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash"],
    },
    Empties {
        read: "r2-w1",
        pl: &["Opening Stock sub"],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash"],
    },
    Empties {
        read: "r2-w2",
        pl: &[],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash"],
    },
    Empties {
        read: "r3-w2",
        pl: &[],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash"],
    },
    Empties {
        read: "r4-w2",
        pl: &[],
        bs_main: &["Loans (Liability)"],
        tb_closing: &["Cash"],
    },
];

/// The Profit and Loss amounts the identities and relation 4 read.
const PL_USED: [(&str, bool); 4] = [
    ("Opening Stock", true),
    ("Less: Closing Stock", true),
    ("Add: Purchase Accounts", true),
    ("Cost of Sales :", false),
];

/// Asserts that the read's empty amounts are exactly [`EMPTIES`]'s row for it, so the zeroing that
/// follows touches only those.
fn assert_empties(read: &str) {
    let pl = statement(NativeStatementKind::ProfitAndLoss, &format!("{read}-pl"));
    let bs = statement(NativeStatementKind::BalanceSheet, &format!("{read}-bs"));
    let tb = ledgers(read);
    let mut pl_empty = Vec::new();
    for (name, is_sub) in PL_USED {
        let amount = if is_sub {
            one(&pl, name)
        } else {
            one_main(&pl, name)
        };
        if *amount == NativeStatementAmount::Empty {
            pl_empty.push(format!("{name} {}", if is_sub { "sub" } else { "main" }));
        }
    }
    for line in pl
        .lines
        .iter()
        .filter(|line| line.name.contains("Direct Expenses"))
    {
        if line.sub == NativeStatementAmount::Empty {
            pl_empty.push(format!("{} sub", line.name));
        }
    }
    let bs_empty: Vec<&str> = bs
        .lines
        .iter()
        .filter(|line| line.main == NativeStatementAmount::Empty)
        .map(|line| line.name.as_str())
        .collect();
    let tb_empty: Vec<&str> = tb
        .rows
        .iter()
        .filter(|row| row.closing == NativeTrialBalanceAmount::PresentEmpty)
        .map(|row| row.name.as_str())
        .collect();
    let want = EMPTIES
        .iter()
        .find(|row| row.read == read)
        .unwrap_or_else(|| panic!("{read} has no row in EMPTIES"));
    assert_eq!(pl_empty, want.pl, "{read}: empty Profit and Loss amounts");
    assert_eq!(
        bs_empty, want.bs_main,
        "{read}: empty Balance Sheet main amounts"
    );
    assert_eq!(tb_empty, want.tb_closing, "{read}: empty ledger closings");
}

fn zero_if_empty(amount: &NativeStatementAmount) -> ExactDecimal {
    match amount {
        NativeStatementAmount::Present(value) => value.clone(),
        NativeStatementAmount::Empty => ExactDecimal::zero(),
    }
}

fn tb_zero_if_empty(amount: &NativeTrialBalanceAmount) -> ExactDecimal {
    match amount {
        NativeTrialBalanceAmount::Present(value) => value.clone(),
        NativeTrialBalanceAmount::PresentEmpty => ExactDecimal::zero(),
    }
}

fn sum<'a>(values: impl IntoIterator<Item = &'a ExactDecimal>) -> ExactDecimal {
    values
        .into_iter()
        .fold(ExactDecimal::zero(), |total, value| {
            total.checked_add(value).expect("the sum is exact")
        })
}

fn sub(left: &ExactDecimal, right: &ExactDecimal) -> ExactDecimal {
    left.checked_subtract(right)
        .expect("the difference is exact")
}

/// The one line of that name; a missing or repeated line fails, never reads as zero.
fn one<'a>(statement: &'a NativeStatement, name: &str) -> &'a NativeStatementAmount {
    one_line(statement, name, |line| &line.sub)
}

fn one_main<'a>(statement: &'a NativeStatement, name: &str) -> &'a NativeStatementAmount {
    one_line(statement, name, |line| &line.main)
}

fn one_line<'a>(
    statement: &'a NativeStatement,
    name: &str,
    column: impl Fn(
        &'a bridge_tally_protocol::native_statement_reports::NativeStatementLine,
    ) -> &'a NativeStatementAmount,
) -> &'a NativeStatementAmount {
    let hits: Vec<_> = statement
        .lines
        .iter()
        .filter(|line| line.name == name)
        .collect();
    assert_eq!(hits.len(), 1, "{name:?} lines: {}", hits.len());
    column(hits[0])
}

/// Where a ledger sits: under the reserved root itself, or under a primary group, named by the
/// last hop's `RESERVEDNAME`. An incomplete chain fails.
enum Place {
    Root,
    Primary(String),
}

fn place(index: &GroupIndex, parent: Option<&str>) -> Place {
    let chain = index.ancestry_chain(parent);
    assert!(chain.is_complete(), "an incomplete chain: {:?}", chain.gap);
    match chain.hops.last() {
        None => Place::Root,
        Some(hop) => Place::Primary(hop.reserved_name.clone()),
    }
}

#[test]
fn the_ten_ledger_and_group_answers_parse_with_the_book_guid() {
    assert!(!groups().is_empty());
    for read in READS {
        // R4 added one ledger, Lab Freight In under Direct Expenses.
        let expected = if read == "r4-w2" { 10 } else { 9 };
        assert_eq!(ledgers(read).rows.len(), expected, "{read}: ledger rows");
    }
}

#[test]
fn identities_1_to_3_hold_in_every_read_as_the_run_recorded() {
    let index = groups();
    for read in READS {
        assert_empties(read);
        let pl = statement(NativeStatementKind::ProfitAndLoss, &format!("{read}-pl"));
        let bs = statement(NativeStatementKind::BalanceSheet, &format!("{read}-bs"));
        let tb = ledgers(read);
        let o = zero_if_empty(one(&pl, "Opening Stock"));
        let c = zero_if_empty(one(&pl, "Less: Closing Stock"));

        // O and C are debits (or zero).
        assert!(
            o.is_negative() || o.is_zero(),
            "{read}: the opening stock line is a debit"
        );
        assert!(
            c.is_negative() || c.is_zero(),
            "{read}: the closing stock line is a debit"
        );

        // The Balance Sheet export balances: its main amounts sum to zero.
        let mains: Vec<ExactDecimal> = bs.lines.iter().map(|l| zero_if_empty(&l.main)).collect();
        assert!(sum(&mains).is_zero(), "{read}: the Balance Sheet balances");

        // Identity 3: the ledgers' closings sum to -O.
        let closings: Vec<ExactDecimal> = tb
            .rows
            .iter()
            .map(|r| tb_zero_if_empty(&r.closing))
            .collect();
        assert!(
            sum(&closings).checked_add(&o).expect("exact").is_zero(),
            "{read}: identity 3"
        );

        let (mut current_assets, mut nominal, mut profit_and_loss) = (vec![], vec![], vec![]);
        let mut at_root = 0;
        for row in &tb.rows {
            let closing = tb_zero_if_empty(&row.closing);
            match place(&index, row.parent.nonempty_returned_text()) {
                Place::Root => {
                    at_root += 1;
                    profit_and_loss.push(closing);
                }
                Place::Primary(primary) if primary == "Current Assets" => {
                    current_assets.push(closing)
                }
                Place::Primary(primary) if NOMINAL.contains(&primary.as_str()) => {
                    nominal.push(closing)
                }
                Place::Primary(primary) => assert!(
                    BALANCE_SHEET_PRIMARIES.contains(&primary.as_str()),
                    "{read}: a ledger under {primary:?}, which the run's script did not classify"
                ),
            }
        }
        assert_eq!(
            at_root, 1,
            "{read}: one ledger, Profit & Loss A/c, under the root"
        );

        // Identity 1: Tally's Current Assets less the ledgers' Current Assets is C.
        let tally_ca = zero_if_empty(one_main(&bs, "Current Assets"));
        assert!(
            sub(&tally_ca, &sum(&current_assets)).numeric_eq(&c),
            "{read}: identity 1"
        );

        // Identity 2: the carried P&L line's gap from the ledgers is O - C.
        let tally_pl = zero_if_empty(one_main(&bs, "Profit & Loss A/c"));
        let ledgers_pl = sum(&profit_and_loss)
            .checked_add(&sum(&nominal))
            .expect("exact");
        assert!(
            sub(&tally_pl, &ledgers_pl).numeric_eq(&sub(&o, &c)),
            "{read}: identity 2"
        );
    }
}

#[test]
fn relation_4_holds_where_the_run_checked_it_and_r1_w4_has_no_heading_amount() {
    let (mut checked, mut without_amount) = (0, Vec::new());
    for read in READS {
        assert_empties(read);
        let pl = statement(NativeStatementKind::ProfitAndLoss, &format!("{read}-pl"));
        let heading = one_main(&pl, "Cost of Sales :");
        let o = zero_if_empty(one(&pl, "Opening Stock"));
        let p = zero_if_empty(one(&pl, "Add: Purchase Accounts"));
        let c = zero_if_empty(one(&pl, "Less: Closing Stock"));
        // As relations.py: the sub amounts of the lines naming Direct Expenses, where present.
        let direct: Vec<_> = pl
            .lines
            .iter()
            .filter(|line| line.name.contains("Direct Expenses"))
            .collect();
        let expected = if read == "r4-w2" { 1 } else { 0 };
        assert_eq!(
            direct.len(),
            expected,
            "{read}: lines naming Direct Expenses"
        );
        let direct_subs: Vec<ExactDecimal> = direct
            .iter()
            .filter_map(|line| match &line.sub {
                NativeStatementAmount::Present(value) => Some(value.clone()),
                NativeStatementAmount::Empty => None,
            })
            .collect();
        assert_eq!(
            direct_subs.len(),
            direct.len(),
            "{read}: each has a sub amount"
        );
        let parts = sum(&[o, p, sum(&direct_subs)]);
        match heading {
            NativeStatementAmount::Empty => without_amount.push(read),
            NativeStatementAmount::Present(value) => {
                assert!(value.numeric_eq(&sub(&parts, &c)), "{read}: relation 4");
                checked += 1;
            }
        }
    }
    assert_eq!(
        without_amount,
        ["r1-w4"],
        "the reads whose heading has no amount"
    );
    assert_eq!(checked, 8, "relation 4 is checked on the other eight reads");
}

#[test]
fn each_after_restore_profit_and_loss_answer_equals_r1_w2_byte_for_byte() {
    let read = |name: &str| {
        let path = format!(
            "{}/tests/fixtures/opening-stock-tieout-e/{name}.utf16le.xml",
            env!("CARGO_MANIFEST_DIR")
        );
        std::fs::read(&path).unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    let first = read("r1-w2-pl");
    for name in ["r2-after-w2-pl", "r3-after-w2-pl", "r4-after-w2-pl"] {
        assert!(read(name) == first, "{name} equals r1-w2-pl");
    }
}
