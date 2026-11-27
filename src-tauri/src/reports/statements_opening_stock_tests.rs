//! Opening stock that Tally carries from a prior year (#692, option B), on the 5 Nov 2026 run's
//! answers from `BRIDGE TIEOUT E` (`crates/bridge-tally-protocol/tests/fixtures/
//! opening-stock-tieout-e/PROVENANCE.md`): a synthetic integrated inventory book, read through
//! the same parsers and the same [`derive_statements`] the tools use.
//!
//! The run did not read the book's inventory flags. The book was created with inventory
//! integrated (PROVENANCE), so these tests pass the flags Tally gave for `BRIDGE TIEOUT B`, an
//! integrated book of the same kind, as [`IntegratedInventory`] reads them.
//!
//! Tests marked "synthetic mutation" edit one captured line to reach a branch the run does not;
//! they make no claim about Tally output.

use super::*;
use bridge_tally_protocol::{
    native_outstandings::parse_native_group_snapshot,
    native_statement_reports::{parse_native_statement, NativeStatementLine},
    native_stock_summary::{parse_company_inventory_flags, parse_native_stock_summary_report},
    native_trial_balance::{parse_native_trial_balance, NativeTrialBalance},
};

const TIEOUT_E_GUID: &str = "ffa928f6-5934-48e2-bbd6-474936e71546";
const TIEOUT_B_GUID: &str = "d64eda55-da7c-4106-a171-3e3cb6cdd331";

/// The reads whose window opens with stock Tally carried from the year before and whose every
/// check holds: R1's 2025-26 and April to September 2025, and 2025-26 after R4's one change.
/// R1's October to March, R2's and R3's 2025-26 are each refused by a rule below.
const CARRIED: [&str; 3] = ["r1-w2", "r1-w3", "r4-w2"];
/// The reads whose window opens at the start of the books, where Tally shows no opening stock.
const FIRST_YEAR: [&str; 3] = ["r1-w1", "r5-w5", "r2-w1"];

fn text(name: &str) -> String {
    let path = format!(
        "{}/crates/bridge-tally-protocol/tests/fixtures/opening-stock-tieout-e/{name}.utf16le.xml",
        env!("CARGO_MANIFEST_DIR")
    );
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{path}: {error}"));
    bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        &bytes,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap()
    .text
}

fn integrated() -> InventoryFlags {
    let bytes = include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_stock_book_live.utf16le.xml"
    );
    let text = bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap()
    .text;
    IntegratedInventory::from_flags(
        &parse_company_inventory_flags(&text, TIEOUT_B_GUID)
            .unwrap()
            .flags,
    )
}

struct Read {
    trial_balance: NativeTrialBalance,
    groups: Vec<TallyNamedMaster>,
    balance_sheet: NativeStatement,
    profit_and_loss: NativeStatement,
    stock_rows: StockRows,
}

fn read(name: &str) -> Read {
    Read {
        trial_balance: parse_native_trial_balance(&text(&format!("{name}-ledgers")), TIEOUT_E_GUID)
            .unwrap(),
        groups: parse_native_group_snapshot(&text("r1-groups"), TIEOUT_E_GUID).unwrap(),
        balance_sheet: parse_native_statement(
            NativeStatementKind::BalanceSheet,
            &text(&format!("{name}-bs")),
        )
        .unwrap(),
        profit_and_loss: parse_native_statement(
            NativeStatementKind::ProfitAndLoss,
            &text(&format!("{name}-pl")),
        )
        .unwrap(),
        stock_rows: StockRows::Report(
            parse_native_stock_summary_report(&text(&format!("{name}-stock-summary"))).unwrap(),
        ),
    }
}

impl Read {
    fn derive(&self, inventory: InventoryFlags) -> DerivedStatements {
        self.try_derive(inventory).unwrap()
    }

    fn try_derive(&self, inventory: InventoryFlags) -> Result<DerivedStatements, StatementsError> {
        derive_statements(
            &SingleCurrencyTrialBalance::admitted_for_tests(self.trial_balance.clone()),
            &self.groups,
            &self.balance_sheet,
            Some(&self.profit_and_loss),
            inventory,
            &self.stock_rows,
        )
    }

    /// The one Profit and Loss line of that name.
    fn line(&mut self, name: &str) -> &mut NativeStatementLine {
        let mut hits = self
            .profit_and_loss
            .lines
            .iter_mut()
            .filter(|line| line.name == name);
        let line = hits.next().unwrap_or_else(|| panic!("no {name:?} line"));
        assert!(hits.next().is_none(), "{name:?} repeated");
        line
    }

    fn sub(&mut self, name: &str) -> ExactDecimal {
        match &self.line(name).sub {
            NativeStatementAmount::Present(value) => value.clone(),
            NativeStatementAmount::Empty => panic!("{name:?} has no sub amount"),
        }
    }

    /// Tally's own Profit & Loss A/c line on its Balance Sheet.
    fn carried(&self) -> ExactDecimal {
        let line = self
            .balance_sheet
            .lines
            .iter()
            .find(|line| line.name == "Profit & Loss A/c")
            .expect("a Profit & Loss A/c line");
        match &line.main {
            NativeStatementAmount::Present(value) => value.clone(),
            NativeStatementAmount::Empty => panic!("an empty Profit & Loss A/c line"),
        }
    }
}

fn established(result: &Established) -> &ExactDecimal {
    match result {
        Established::Established { value } => value,
        other => panic!("not established: {other:?}"),
    }
}

fn declined(derived: &DerivedStatements) -> ClosingStockDeclined {
    match &derived.closing_stock {
        ClosingStockUse::Declined { why, .. } => *why,
        other => panic!("not declined: {other:?}"),
    }
}

/// A declined read keeps the results it would have had with no stock figure at all, which are
/// the ones a book with opening stock had before #692's option B.
fn assert_results_as_without_stock(read: &Read, derived: &DerivedStatements) {
    let without = read.derive(InventoryFlags::NotRead);
    assert_eq!(derived.gross_result, without.gross_result);
    assert_eq!(derived.net_result, without.net_result);
    assert_eq!(
        derived.balance_sheet_profit_and_loss,
        without.balance_sheet_profit_and_loss
    );
    assert!(matches!(
        derived.net_result,
        Established::NotEstablished { .. }
    ));
}

#[test]
fn every_carried_over_read_establishes_with_tallys_own_opening_and_closing_stock() {
    for name in CARRIED {
        let mut read = read(name);
        let derived = read.derive(integrated());
        let (opening, closing) = (read.sub("Opening Stock"), read.sub("Less: Closing Stock"));
        assert_eq!(
            derived.closing_stock,
            ClosingStockUse::Applied {
                value: closing,
                opening: Some(opening),
            },
            "{name}"
        );
        // Identities 1 and 2 are the Balance Sheet gate's Current Assets and Profit & Loss A/c
        // lines; relation 4 is the Profit and Loss gate's heading.
        assert!(established(&derived.balance_sheet_profit_and_loss).numeric_eq(&read.carried()));
        established(&derived.gross_result);
        established(&derived.net_result);
    }
}

#[test]
fn a_first_year_read_establishes_with_its_closing_stock_alone() {
    for name in FIRST_YEAR {
        let mut read = read(name);
        assert_eq!(read.line("Opening Stock").sub, NativeStatementAmount::Empty);
        let derived = read.derive(integrated());
        assert_eq!(
            derived.closing_stock,
            ClosingStockUse::Applied {
                value: read.sub("Less: Closing Stock"),
                opening: None,
            },
            "{name}"
        );
        assert!(established(&derived.balance_sheet_profit_and_loss).numeric_eq(&read.carried()));
    }
}

#[test]
fn rule_1_a_carried_line_whose_closing_departs_from_the_ledgers_own_opening_is_refused() {
    // R2 changed one item's costing method from Avg. Cost to FIFO. Year 1 then reads with a new
    // result, while the Profit & Loss A/c ledger still opens 2025-26 with the result Tally carried
    // before the change. Its closing for 2025-26 moved with year 1 although the window has no
    // movement on it, so the row does not add up and the read is refused, as for any such row.
    let before = established(
        &read("r1-w1")
            .derive(integrated())
            .balance_sheet_profit_and_loss,
    )
    .clone();
    let after = established(
        &read("r2-w1")
            .derive(integrated())
            .balance_sheet_profit_and_loss,
    )
    .clone();
    assert!(!before.numeric_eq(&after));
    let year_two = read("r2-w2");
    let ledger = year_two
        .trial_balance
        .rows
        .iter()
        .find(|row| row.name == "Profit & Loss A/c")
        .expect("the Profit & Loss A/c row");
    assert!(
        matches!(&ledger.opening, NativeTrialBalanceAmount::Present(opening)
        if opening.numeric_eq(&before))
    );
    let Err(StatementsError::RowInconsistent(gap)) = year_two.try_derive(integrated()) else {
        panic!("the row is not refused");
    };
    assert_eq!(gap.ledger, "Profit & Loss A/c");
    let gap = ExactDecimal::parse(&gap.gap).unwrap();
    assert!(gap.numeric_eq(&after.checked_subtract(&before).unwrap()));
}

#[test]
fn rule_2_a_stock_summary_row_without_a_value_refuses_the_stock_and_keeps_todays_results() {
    // R3 sold an item with no purchase: its Stock Summary row shows a negative quantity and no
    // value, while every identity still holds.
    let read = read("r3-w2");
    let StockRows::Report(NativeStockReport::Lines { empty, .. }) = &read.stock_rows else {
        panic!("not a report with lines");
    };
    assert_eq!(*empty, 1);
    let derived = read.derive(integrated());
    assert_eq!(
        declined(&derived),
        ClosingStockDeclined::StockSummaryRowWithoutValue
    );
    assert_results_as_without_stock(&read, &derived);
}

#[test]
fn rule_2_each_stock_summary_the_read_could_not_use_has_its_own_reason() {
    for (rows, why) in [
        (
            StockRows::WindowNotMeasured,
            ClosingStockDeclined::StockSummaryWindowNotMeasured,
        ),
        (
            StockRows::BookTooLarge,
            ClosingStockDeclined::StockSummaryTooLarge,
        ),
        (
            StockRows::Unreadable,
            ClosingStockDeclined::StockSummaryNotRead,
        ),
        (
            StockRows::NotRead,
            ClosingStockDeclined::StockSummaryNotRead,
        ),
        (
            StockRows::Report(NativeStockReport::Empty),
            ClosingStockDeclined::StockSummaryNotRead,
        ),
    ] {
        let mut read = read("r1-w2");
        read.stock_rows = rows;
        let derived = read.derive(integrated());
        assert_eq!(declined(&derived), why);
        assert_results_as_without_stock(&read, &derived);
    }
}

#[test]
fn rule_2_a_first_year_read_needs_no_stock_summary() {
    let mut read = read("r1-w1");
    read.stock_rows = StockRows::NotRead;
    assert!(matches!(
        read.derive(integrated()).closing_stock,
        ClosingStockUse::Applied { opening: None, .. }
    ));
}

#[test]
fn rule_3_an_empty_cost_of_sales_heading_refuses_the_stock_and_keeps_todays_results() {
    // R1's October to March: no purchase or sale, opening and closing stock equal, and the heading
    // with no amount, which would otherwise read as a zero cost of sales.
    let read = read("r1-w4");
    let derived = read.derive(integrated());
    assert_eq!(
        declined(&derived),
        ClosingStockDeclined::CostOfSalesHeadingWithoutAmount
    );
    assert_results_as_without_stock(&read, &derived);
}

#[test]
fn rule_4_direct_expenses_count_in_the_cost_of_sales_only_from_the_sub_column() {
    let mut read = read("r4-w2");
    let direct = read.sub("Direct Expenses");
    assert!(!direct.is_zero());
    assert!(matches!(
        read.derive(integrated()).closing_stock,
        ClosingStockUse::Applied { .. }
    ));
    // synthetic mutation: the same amount in the main column instead.
    let line = read.line("Direct Expenses");
    line.sub = NativeStatementAmount::Empty;
    line.main = NativeStatementAmount::Present(direct);
    let derived = read.derive(integrated());
    let ClosingStockUse::Declined { why, lines } = &derived.closing_stock else {
        panic!("not declined: {:?}", derived.closing_stock);
    };
    assert_eq!(*why, ClosingStockDeclined::DoesNotCloseEveryGap);
    assert_eq!(lines, &["Direct Expenses".to_string()]);
    assert_results_as_without_stock(&read, &derived);
}

/// A named change to Tally's Opening Stock line.
type LineEdit = (&'static str, fn(&mut NativeStatementLine));

#[test]
fn an_opening_stock_that_is_not_one_debit_in_its_sub_column_is_refused() {
    let edits: [LineEdit; 4] = [
        ("a credit", |line| {
            line.sub = NativeStatementAmount::Present(ExactDecimal::parse("2120.00").unwrap())
        }),
        ("a zero", |line| {
            line.sub = NativeStatementAmount::Present(ExactDecimal::zero())
        }),
        ("the main column", |line| {
            line.main = std::mem::replace(&mut line.sub, NativeStatementAmount::Empty)
        }),
        ("both columns", |line| line.main = line.sub.clone()),
    ];
    for (what, edit) in edits {
        // synthetic mutation: the captured opening stock line edited as named.
        let mut read = read("r1-w2");
        edit(read.line("Opening Stock"));
        let derived = read.derive(integrated());
        assert_eq!(
            declined(&derived),
            ClosingStockDeclined::OpeningStockNotAsSeen,
            "{what}"
        );
        assert_results_as_without_stock(&read, &derived);
    }
    // synthetic mutation: the line repeated.
    let mut read = read("r1-w2");
    let repeated = read.line("Opening Stock").clone();
    read.profit_and_loss.lines.push(repeated);
    assert_eq!(
        declined(&read.derive(integrated())),
        ClosingStockDeclined::OpeningStockNotAsSeen
    );
}

#[test]
fn a_trial_balance_that_does_not_sum_to_minus_the_opening_stock_is_refused() {
    // synthetic mutation: the capital ledger's opening and closing, and Tally's Capital Account
    // line, each one rupee higher. Every line still ties, but the ledgers' closings no longer
    // sum to minus the opening stock (identity 3).
    let mut read = read("r1-w2");
    let one = ExactDecimal::parse("1.00").unwrap();
    let raised = |amount: &NativeTrialBalanceAmount| match amount {
        NativeTrialBalanceAmount::Present(value) => {
            NativeTrialBalanceAmount::Present(value.checked_add(&one).unwrap())
        }
        NativeTrialBalanceAmount::PresentEmpty => panic!("an empty capital amount"),
    };
    let capital = read
        .trial_balance
        .rows
        .iter_mut()
        .find(|row| row.name == "Lab Capital")
        .expect("the capital ledger");
    capital.opening = raised(&capital.opening);
    capital.closing = raised(&capital.closing);
    let line = read
        .balance_sheet
        .lines
        .iter_mut()
        .find(|line| line.name == "Capital Account")
        .expect("the Capital Account line");
    line.main = match &line.main {
        NativeStatementAmount::Present(value) => {
            NativeStatementAmount::Present(value.checked_add(&one).unwrap())
        }
        NativeStatementAmount::Empty => panic!("an empty Capital Account line"),
    };
    let derived = read.derive(integrated());
    assert_eq!(
        declined(&derived),
        ClosingStockDeclined::BalanceSheetDoesNotBalance
    );
    assert_results_as_without_stock(&read, &derived);
}

#[test]
fn without_the_flags_a_carried_over_read_keeps_todays_refusal() {
    for name in CARRIED {
        let read = read(name);
        let derived = read.derive(InventoryFlags::NotRead);
        assert_eq!(
            declined(&derived),
            ClosingStockDeclined::InventoryFlagsNotRead
        );
        assert!(matches!(
            derived.net_result,
            Established::NotEstablished { .. }
        ));
    }
}
