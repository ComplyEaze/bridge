//! Two tests use one company's captures of one window: the Trial Balance,
//! group tree and both of Tally's own statements. They prove the derivation
//! ties, and that the gate passes, on a full year and on a part-year window.
//!
//! The rest pair a captured Trial Balance from one synthetic company with a
//! verbatim group tree captured from another. Every `PARENT` in those Trial
//! Balances names a group Tally creates in every company, so each hop resolves
//! through Tally's own default tree. That pairing crosses companies and proves
//! classification, arithmetic and the gate's clauses only. Tally's Balance
//! Sheet in those tests is synthetic, built to tie or to fail one clause.
//!
//! Tests marked "synthetic mutation" alter a captured row or group to reach a
//! branch the captures do not; they make no claim about Tally output.

use super::*;
use bridge_tally_protocol::{
    native_outstandings::parse_native_group_snapshot,
    native_statement_reports::{parse_native_statement, NativeStatementLine},
    native_stock_summary::parse_company_inventory_flags,
    native_trial_balance::{parse_native_trial_balance, NativeTrialBalance},
    PartyLedgerMasterFieldObservation,
};

const KNOWN_LAB: &str = include_str!(
    "../../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_known_lab.xml"
);
const KNOWN_LAB_GUID: &str = "eebb9a9f-1679-4468-9e8f-814c729674cb";
const OPENING_YEAR: &str = include_str!(
    "../../crates/bridge-tally-protocol/tests/fixtures/native/trial_balance_opening_year.xml"
);
const OPENING_YEAR_GUID: &str = "915d42f8-42ae-4b03-8291-55f596e3a2ea";
const GROUPS: &str = include_str!(
    "../../crates/bridge-tally-protocol/tests/fixtures/native/group_snapshot_aarav_with_computed_company_guid.xml"
);
const GROUPS_GUID: &str = "bb8ad19e-6aef-4239-a917-87fec0c6215e";

fn groups() -> Vec<TallyNamedMaster> {
    parse_native_group_snapshot(GROUPS, GROUPS_GUID).unwrap()
}

fn known_lab() -> NativeTrialBalance {
    parse_native_trial_balance(KNOWN_LAB, KNOWN_LAB_GUID).unwrap()
}

fn admitted(report: NativeTrialBalance) -> SingleCurrencyTrialBalance {
    SingleCurrencyTrialBalance::admitted_for_tests(report)
}

fn decimal(value: &str) -> ExactDecimal {
    ExactDecimal::parse(value).unwrap()
}

fn statement(kind: NativeStatementKind, lines: &[(&str, &str, &str)]) -> NativeStatement {
    let amount = |value: &str| {
        if value.is_empty() {
            NativeStatementAmount::Empty
        } else {
            NativeStatementAmount::Present(decimal(value))
        }
    };
    NativeStatement {
        kind,
        lines: lines
            .iter()
            .map(|(name, sub, main)| NativeStatementLine {
                name: name.to_string(),
                sub: amount(sub),
                main: amount(main),
            })
            .collect(),
    }
}

fn balance_sheet(lines: &[(&str, &str, &str)]) -> NativeStatement {
    statement(NativeStatementKind::BalanceSheet, lines)
}

/// A synthetic Tally Balance Sheet that ties to the known-lab derivation.
fn known_lab_balance_sheet() -> NativeStatement {
    balance_sheet(&[
        ("Capital Account", "", ""),
        ("Current Assets", "", "-11027.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ])
}

fn derive(report: NativeTrialBalance, tally: &NativeStatement) -> DerivedStatements {
    derive_statements(
        &admitted(report),
        &groups(),
        tally,
        None,
        InventoryFlags::NotRead,
    )
    .unwrap()
}

fn assert_sum(sum: &StatementSum, value: &str, present: usize, empty: usize) {
    assert!(
        sum.sum.numeric_eq(&decimal(value)),
        "{} != {value}",
        sum.sum.as_str()
    );
    assert_eq!((sum.present_count, sum.empty_count), (present, empty));
}

fn assert_established(result: &Established, value: &str) {
    match result {
        Established::Established { value: got } => {
            assert!(
                got.numeric_eq(&decimal(value)),
                "{} != {value}",
                got.as_str()
            )
        }
        other => panic!("expected {value}, got {other:?}"),
    }
}

fn blocked(reason: NotEstablishedReason) -> Established {
    Established::NotEstablished {
        reason,
        lines: Vec::new(),
    }
}

fn differs(lines: &[&str]) -> Established {
    Established::NotEstablished {
        reason: NotEstablishedReason::TallyBalanceSheetDiffers,
        lines: lines.iter().map(|line| line.to_string()).collect(),
    }
}

fn assert_every_result(derived: &DerivedStatements, expected: &Established) {
    assert_eq!(&derived.gross_result, expected);
    assert_eq!(&derived.net_result, expected);
    assert_eq!(&derived.balance_sheet_profit_and_loss, expected);
}

fn line<'a>(lines: &'a [PrimaryGroupLine], reserved: &str) -> &'a PrimaryGroupLine {
    lines
        .iter()
        .find(|line| line.reserved_name == reserved)
        .unwrap_or_else(|| panic!("no {reserved} line"))
}

fn root_parent(report: &NativeTrialBalance) -> PartyLedgerMasterFieldObservation {
    report
        .rows
        .iter()
        .find(|row| row.name == "Profit & Loss A/c")
        .unwrap()
        .parent
        .clone()
}

fn row<'a>(report: &'a mut NativeTrialBalance, name: &str) -> &'a mut NativeTrialBalanceRow {
    report.rows.iter_mut().find(|row| row.name == name).unwrap()
}

#[test]
fn a_captured_trial_balance_splits_into_statement_lines() {
    let derived = derive(known_lab(), &known_lab_balance_sheet());

    // The captured tree holds all fifteen reserved primary groups.
    assert_eq!(derived.profit_and_loss.len(), 6);
    assert_eq!(derived.balance_sheet.len(), 9);
    assert_eq!(
        derived
            .profit_and_loss
            .iter()
            .chain(&derived.balance_sheet)
            .filter(|line| line.ledger_count > 0)
            .count(),
        2
    );
    let sales = line(&derived.profit_and_loss, "Sales Accounts");
    // Movement: an empty debit and a 4027.00 credit.
    assert_sum(&sales.amount, "4027.00", 1, 1);
    assert_eq!(sales.ledger_count, 1);

    let current_assets = line(&derived.balance_sheet, "Current Assets");
    // Four closings, one of them empty: debtors and cash.
    assert_sum(&current_assets.amount, "-11027.00", 3, 1);
    assert_eq!(current_assets.ledger_count, 4);

    let root = derived.profit_and_loss_ledger.as_ref().unwrap();
    assert_eq!(root.name, "Profit & Loss A/c");
    assert!(derived.unclassified.is_empty());
    assert_eq!(derived.stock_ledger_count, 0);
    assert_established(&derived.gross_result, "4027.00");
    assert_established(&derived.net_result, "4027.00");
    // 7000.00 carried on the ledger plus the window's 4027.00: §5.6's
    // balance-sheet figure, and the negation of the asset line.
    assert_established(&derived.balance_sheet_profit_and_loss, "11027.00");
}

#[test]
fn an_opening_only_book_keeps_its_opening_difference_visible() {
    let report = parse_native_trial_balance(OPENING_YEAR, OPENING_YEAR_GUID).unwrap();
    let tally = balance_sheet(&[
        ("Capital Account", "", "125000.00"),
        ("Current Liabilities", "", "88000.00"),
        ("Profit & Loss A/c", "", ""),
        ("Current Assets", "", "-262833.50"),
    ]);
    // A synthetic statement whose lines tie one by one. Tally's own export
    // leaves out its Difference in opening balances line (measured once, #2063),
    // so its sides differ here by 49,833.50 and no result is established.
    let derived = derive(report, &tally);
    assert_every_result(
        &derived,
        &Established::NotEstablished {
            reason: NotEstablishedReason::TallyBalanceSheetDoesNotBalance,
            lines: Vec::new(),
        },
    );
    let sides = derived.tally_balance_sheet_sides.as_ref().unwrap();
    assert!(sides.credits.numeric_eq(&decimal("213000.00")));
    assert!(sides.debits.numeric_eq(&decimal("262833.50")));

    assert!(derived
        .profit_and_loss
        .iter()
        .all(|line| line.ledger_count == 0));
    assert_sum(
        &line(&derived.balance_sheet, "Capital Account").amount,
        "125000.00",
        1,
        0,
    );
    assert_sum(
        &line(&derived.balance_sheet, "Current Liabilities").amount,
        "88000.00",
        1,
        0,
    );
    assert_sum(
        &line(&derived.balance_sheet, "Current Assets").amount,
        "-262833.50",
        4,
        1,
    );
    // No line is invented to balance: the three lines net to §5.6's -49833.50.
    let net: ExactDecimal = derived
        .balance_sheet
        .iter()
        .try_fold(ExactDecimal::zero(), |total, line| {
            total.checked_add(&line.amount.sum)
        })
        .unwrap();
    assert!(net.numeric_eq(&decimal("-49833.50")));
}

#[test]
fn a_p_and_l_line_is_the_window_movement_not_the_closing() {
    // Synthetic mutation: the sales ledger opens at 100.00, so its closing
    // (4127.00) and its movement (4027.00) differ; a debtor opens and closes
    // 100.00 lower, so the Trial Balance still balances.
    let mut report = known_lab();
    let sales = row(&mut report, "Ageing Sales");
    sales.opening = NativeTrialBalanceAmount::Present(decimal("100.00"));
    sales.closing = NativeTrialBalanceAmount::Present(decimal("4127.00"));
    let debtor = row(&mut report, "Ageing Customer A");
    debtor.opening = NativeTrialBalanceAmount::Present(decimal("-7100.00"));
    debtor.closing = NativeTrialBalanceAmount::Present(decimal("-7377.00"));
    let tally = balance_sheet(&[
        ("Current Assets", "", "-11127.00"),
        ("Profit & Loss A/c", "", "11127.00"),
    ]);
    let derived = derive(report, &tally);

    assert_sum(
        &line(&derived.profit_and_loss, "Sales Accounts").amount,
        "4027.00",
        1,
        1,
    );
    assert_established(&derived.net_result, "4027.00");
    // The carried line is closings: 7000.00 on the ledger and 4127.00 of sales.
    assert_established(&derived.balance_sheet_profit_and_loss, "11127.00");
}

#[test]
fn a_ledger_under_a_user_created_primary_group_blocks_every_result() {
    // Synthetic mutation: a user-created primary group, and one captured
    // ledger moved under it.
    let mut report = known_lab();
    let mut groups = groups();
    groups.push(TallyNamedMaster {
        name: "BRIDGE Synthetic Primary".to_string(),
        parent: root_parent(&report),
        reserved_name: Some(String::new()),
    });
    row(&mut report, "Ageing Sales").parent =
        PartyLedgerMasterFieldObservation::Returned("BRIDGE Synthetic Primary".to_string());

    let derived = derive_statements(
        &admitted(report),
        &groups,
        &known_lab_balance_sheet(),
        None,
        InventoryFlags::NotRead,
    )
    .unwrap();
    assert_eq!(
        line(&derived.profit_and_loss, "Sales Accounts").ledger_count,
        0
    );
    assert_eq!(derived.unclassified.len(), 1);
    assert_eq!(derived.unclassified[0].reason, "primary_group_user_created");
    assert_every_result(
        &derived,
        &blocked(NotEstablishedReason::UnclassifiedLedgerCarriesAnAmount),
    );
}

#[test]
fn an_unclassified_ledger_with_no_amount_blocks_nothing() {
    // Synthetic mutation: the captured all-empty ledger loses its parent group.
    let mut report = known_lab();
    row(&mut report, "Ageing Bank").parent =
        PartyLedgerMasterFieldObservation::Returned("BRIDGE Absent Group".to_string());

    let derived = derive(report, &known_lab_balance_sheet());
    assert_eq!(derived.unclassified.len(), 1);
    assert_eq!(derived.unclassified[0].reason, "group_absent");
    assert_established(&derived.net_result, "4027.00");
}

#[test]
fn a_stock_balance_blocks_every_result_including_the_carried_line() {
    // Synthetic mutation: a captured ledger with a non-zero closing moved under
    // Stock-in-Hand. Tally's carried line includes the change in stock, which
    // the Trial Balance cannot give, so the carried line is blocked too.
    let mut report = known_lab();
    row(&mut report, "Cash").parent =
        PartyLedgerMasterFieldObservation::Returned("Stock-in-Hand".to_string());

    let derived = derive(report, &known_lab_balance_sheet());
    assert_eq!(derived.stock_ledger_count, 1);
    assert_every_result(
        &derived,
        &blocked(NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance),
    );
}

#[test]
fn a_tally_line_that_differs_blocks_every_result_and_is_named() {
    let tally = balance_sheet(&[
        ("Current Assets", "", "-11026.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Current Assets"]));
}

#[test]
fn a_tally_line_with_an_amount_nothing_derived_matches_blocks_every_result() {
    // What an inventory book's statement is expected to add: a line the Trial
    // Balance cannot produce.
    let tally = balance_sheet(&[
        ("Current Assets", "", "-11027.00"),
        ("Closing Stock", "", "500.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Closing Stock"]));

    // Both columns present is uncompared too, and carries an amount.
    let tally = balance_sheet(&[
        ("Current Assets", "-1.00", "-11027.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Current Assets"]));
}

#[test]
fn a_derived_line_that_tally_does_not_show_blocks_every_result() {
    let tally = balance_sheet(&[("Profit & Loss A/c", "", "11027.00")]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Current Assets"]));
}

#[test]
fn a_missing_profit_and_loss_ledger_blocks_every_result_and_says_so() {
    // Synthetic mutation: the reserved-root ledger removed from the capture.
    let mut report = known_lab();
    report.rows.retain(|row| row.name != "Profit & Loss A/c");
    let derived = derive(report, &known_lab_balance_sheet());
    assert_every_result(
        &derived,
        &blocked(NotEstablishedReason::ProfitAndLossLedgerNotReturned),
    );
}

#[test]
fn a_tally_line_without_a_counterpart_passes_only_while_it_carries_no_amount() {
    let tally = balance_sheet(&[
        ("Sources of Funds :", "", ""),
        ("Diff. in Opening Balances", "", "0.00"),
        ("Current Assets", "", "-11027.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_established(&derived.net_result, "4027.00");

    let tally = balance_sheet(&[
        ("Diff. in Opening Balances", "", "0.01"),
        ("Current Assets", "", "-11027.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Diff. in Opening Balances"]));
}

#[test]
fn a_row_whose_columns_do_not_add_up_is_refused() {
    // Synthetic mutation: the one captured row with all four amounts present.
    let mut report = known_lab();
    row(&mut report, "Ageing Customer A").closing =
        NativeTrialBalanceAmount::Present(decimal("-7277.01"));
    assert_eq!(
        derive_statements(
            &admitted(report),
            &groups(),
            &known_lab_balance_sheet(),
            None,
            InventoryFlags::NotRead
        )
        .unwrap_err(),
        StatementsError::RowInconsistent(RowGap {
            ledger: "Ageing Customer A".to_string(),
            gap: "-0.01".to_string(),
        })
    );
}

#[test]
fn a_closing_that_no_movement_explains_is_refused_even_with_empty_columns() {
    // Synthetic mutation: the sales ledger keeps its 4027.00 closing but loses
    // its credit, so its closing is in the carried line and not in the P&L.
    let mut report = known_lab();
    row(&mut report, "Ageing Sales").credit = NativeTrialBalanceAmount::PresentEmpty;
    assert_eq!(
        derive_statements(
            &admitted(report),
            &groups(),
            &known_lab_balance_sheet(),
            None,
            InventoryFlags::NotRead
        )
        .unwrap_err(),
        StatementsError::RowInconsistent(RowGap {
            ledger: "Ageing Sales".to_string(),
            gap: "4027.00".to_string(),
        })
    );
}

#[test]
fn a_second_root_ledger_is_refused() {
    // Synthetic mutation: another captured ledger moved to the root.
    let mut report = known_lab();
    let root = root_parent(&report);
    row(&mut report, "Cash").parent = root;
    assert_eq!(
        derive_statements(
            &admitted(report),
            &groups(),
            &known_lab_balance_sheet(),
            None,
            InventoryFlags::NotRead
        )
        .unwrap_err(),
        StatementsError::RootLedgerRepeated
    );
}

#[test]
fn each_gate_takes_only_its_own_statement() {
    let profit_and_loss = statement(
        NativeStatementKind::ProfitAndLoss,
        &[("Sales Accounts", "", "4027.00")],
    );
    assert_eq!(
        derive_statements(
            &admitted(known_lab()),
            &groups(),
            &profit_and_loss,
            None,
            InventoryFlags::NotRead
        )
        .unwrap_err(),
        StatementsError::GateNotABalanceSheet
    );
    assert_eq!(
        derive_statements(
            &admitted(known_lab()),
            &groups(),
            &known_lab_balance_sheet(),
            Some(&known_lab_balance_sheet()),
            InventoryFlags::NotRead
        )
        .unwrap_err(),
        StatementsError::GateNotAProfitAndLoss
    );
}

fn profit_and_loss(lines: &[(&str, &str, &str)]) -> NativeStatement {
    statement(NativeStatementKind::ProfitAndLoss, lines)
}

/// Known lab with Tally's Balance Sheet tying and `tally` as its P&L.
fn derive_with_profit_and_loss(
    report: NativeTrialBalance,
    tally: &NativeStatement,
) -> DerivedStatements {
    derive_statements(
        &admitted(report),
        &groups(),
        &known_lab_balance_sheet(),
        Some(tally),
        InventoryFlags::NotRead,
    )
    .unwrap()
}

fn assert_movement_refused(derived: &DerivedStatements, lines: &[&str]) {
    let expected = Established::NotEstablished {
        reason: NotEstablishedReason::TallyProfitAndLossDiffers,
        lines: lines.iter().map(|line| line.to_string()).collect(),
    };
    assert_eq!(derived.gross_result, expected);
    assert_eq!(derived.net_result, expected);
    // The carried line is tied to the Balance Sheet, which still ties.
    assert_established(&derived.balance_sheet_profit_and_loss, "11027.00");
}

#[test]
fn a_profit_and_loss_that_ties_establishes_gross_and_net() {
    let derived = derive_with_profit_and_loss(
        known_lab(),
        &profit_and_loss(&[("Sales Accounts", "", "4027.00")]),
    );
    assert_established(&derived.gross_result, "4027.00");
    assert_established(&derived.net_result, "4027.00");
}

#[test]
fn a_stock_line_on_tallys_profit_and_loss_refuses_gross_and_net() {
    let derived = derive_with_profit_and_loss(
        known_lab(),
        &profit_and_loss(&[
            ("Closing Stock", "", "100.00"),
            ("Sales Accounts", "", "4027.00"),
        ]),
    );
    assert_movement_refused(&derived, &["Closing Stock"]);
}

#[test]
fn stock_held_at_the_window_start_refuses_even_when_the_balance_sheet_ties() {
    // The worked case: stock bought before the window and sold inside it, none
    // left at `to`. Tally's Balance Sheet ties; its P&L carries Opening Stock
    // and a cost of sales the derivation's purchases (none) do not explain.
    let derived = derive_with_profit_and_loss(
        known_lab(),
        &profit_and_loss(&[
            ("Opening Stock", "", "-100.00"),
            ("Cost of Sales :", "", "-100.00"),
            ("Sales Accounts", "", "4027.00"),
        ]),
    );
    assert_movement_refused(&derived, &["Opening Stock", "Cost of Sales :"]);
}

#[test]
fn a_renamed_derived_line_is_derived_only_and_refuses() {
    let derived =
        derive_with_profit_and_loss(known_lab(), &profit_and_loss(&[("Sales", "", "4027.00")]));
    let sales = line(&derived.profit_and_loss, "Sales Accounts")
        .display_name
        .clone();
    assert_movement_refused(&derived, &["Sales", &sales]);
}

/// Decoded as production decodes a response: BOM-less UTF-16LE under the
/// `charset=utf-16` content type Tally sent.
fn utf16(bytes: &[u8]) -> String {
    bridge_tally_protocol::decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        bridge_tally_protocol::ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap()
    .text
}

struct Capture {
    guid: &'static str,
    trial_balance: &'static [u8],
    groups: &'static [u8],
    balance_sheet: &'static [u8],
    profit_and_loss: &'static [u8],
}

impl Capture {
    fn parts(
        &self,
    ) -> (
        NativeTrialBalance,
        Vec<TallyNamedMaster>,
        NativeStatement,
        NativeStatement,
    ) {
        (
            parse_native_trial_balance(&utf16(self.trial_balance), self.guid).unwrap(),
            parse_native_group_snapshot(&utf16(self.groups), self.guid).unwrap(),
            parse_native_statement(
                NativeStatementKind::BalanceSheet,
                &utf16(self.balance_sheet),
            )
            .unwrap(),
            parse_native_statement(
                NativeStatementKind::ProfitAndLoss,
                &utf16(self.profit_and_loss),
            )
            .unwrap(),
        )
    }

    fn derive(&self) -> (DerivedStatements, TieOut) {
        let (report, groups, balance_sheet, profit_and_loss) = self.parts();
        let derived = derive_statements(
            &admitted(report),
            &groups,
            &balance_sheet,
            Some(&profit_and_loss),
            InventoryFlags::NotRead,
        )
        .unwrap();
        let tie = derived.profit_and_loss_tie.clone().unwrap();
        (derived, tie)
    }
}

const READS_LAB: Capture = Capture {
    guid: "de2e15f2-6d42-4715-b6e7-b7a95a68abe8",
    trial_balance: include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/statement_trial_balance_fy_live.utf16le.xml"
    ),
    groups: include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/statement_groups_fy_live.utf16le.xml"
    ),
    balance_sheet: include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/statement_balance_sheet_fy_live.utf16le.xml"
    ),
    profit_and_loss: include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/statement_profit_and_loss_fy_live.utf16le.xml"
    ),
};

/// READS LAB's own sources with its captured P&L replaced by `lines`: the
/// heading allowance measured against the one book it was observed on.
fn reads_lab_with_profit_and_loss(lines: &[(&str, &str, &str)]) -> DerivedStatements {
    let (report, groups, balance_sheet, _) = READS_LAB.parts();
    derive_statements(
        &admitted(report),
        &groups,
        &balance_sheet,
        Some(&profit_and_loss(lines)),
        InventoryFlags::NotRead,
    )
    .unwrap()
}

#[test]
fn the_cost_of_sales_heading_passes_only_at_exactly_the_derived_cost_of_sales() {
    let holds = reads_lab_with_profit_and_loss(&[
        ("Cost of Sales :", "", "-4250.00"),
        ("Purchase Accounts", "-4250.00", ""),
    ]);
    assert_established(&holds.net_result, "-4250.00");

    let refused = |lines: &[(&str, &str, &str)], name: &str| {
        let derived = reads_lab_with_profit_and_loss(lines);
        let expected = Established::NotEstablished {
            reason: NotEstablishedReason::TallyProfitAndLossDiffers,
            lines: vec![name.to_string()],
        };
        assert_eq!(derived.gross_result, expected);
        assert_eq!(derived.net_result, expected);
    };
    // Off by one paisa.
    refused(
        &[
            ("Cost of Sales :", "", "-4250.01"),
            ("Purchase Accounts", "-4250.00", ""),
        ],
        "Cost of Sales :",
    );
    // Any other spelling is not the heading.
    refused(
        &[
            ("Cost of sales :", "", "-4250.00"),
            ("Purchase Accounts", "-4250.00", ""),
        ],
        "Cost of sales :",
    );
    refused(
        &[
            ("Cost of Sales", "", "-4250.00"),
            ("Purchase Accounts", "-4250.00", ""),
        ],
        "Cost of Sales",
    );
}

/// A heading that reads zero or empty is compared like any other line (#1070):
/// over a non-zero derived cost of sales it refuses, naming the heading, and
/// over a zero one it ties. Before, the no-amount arm let it pass whatever the
/// cost of sales was.
#[test]
fn a_zero_or_empty_cost_of_sales_heading_is_compared_with_the_cost_of_sales() {
    let expected = Established::NotEstablished {
        reason: NotEstablishedReason::TallyProfitAndLossDiffers,
        lines: vec!["Cost of Sales :".to_string()],
    };
    for (sub, main) in [("", "0.00"), ("0.00", "0.00"), ("", "")] {
        let derived = reads_lab_with_profit_and_loss(&[
            ("Cost of Sales :", sub, main),
            ("Purchase Accounts", "-4250.00", ""),
        ]);
        assert_eq!(derived.gross_result, expected, "{sub:?} {main:?}");
        assert_eq!(derived.net_result, expected, "{sub:?} {main:?}");
    }
    // Known lab has no Purchase Accounts or Direct Expenses: its cost of
    // sales is zero, so a zero or empty heading ties.
    for (sub, main) in [("", "0.00"), ("0.00", "0.00"), ("", "")] {
        let derived = derive_with_profit_and_loss(
            known_lab(),
            &profit_and_loss(&[
                ("Cost of Sales :", sub, main),
                ("Sales Accounts", "", "4027.00"),
            ]),
        );
        assert_established(&derived.net_result, "4027.00");
    }
}

fn statuses(tie: &TieOut) -> Vec<(&str, TieStatus)> {
    tie.lines
        .iter()
        .map(|line| (line.name.as_str(), line.status.clone()))
        .collect()
}

/// One company, one window, four captures read within two minutes: the Trial
/// Balance and group tree derive the statements, Tally's own Balance Sheet
/// passes the gate, and Tally's own Profit and Loss ties.
#[test]
fn a_same_company_full_year_passes_the_gate() {
    let (derived, tie) = READS_LAB.derive();

    assert!(derived.unclassified.is_empty());
    assert_sum(
        &line(&derived.profit_and_loss, "Purchase Accounts").amount,
        "-4250.00",
        1,
        1,
    );
    assert_sum(
        &line(&derived.balance_sheet, "Current Liabilities").amount,
        "4250.00",
        1,
        3,
    );
    assert_established(&derived.gross_result, "-4250.00");
    assert_established(&derived.net_result, "-4250.00");
    assert_established(&derived.balance_sheet_profit_and_loss, "-4250.00");
    assert_eq!(
        statuses(&derived.balance_sheet_tie),
        vec![
            ("Capital Account", TieStatus::MatchedEmptyAsZero),
            ("Loans (Liability)", TieStatus::MatchedEmptyAsZero),
            ("Current Liabilities", TieStatus::Matched),
            ("Profit & Loss A/c", TieStatus::Matched),
            ("Current Assets", TieStatus::MatchedEmptyAsZero),
        ]
    );
    assert!(derived.balance_sheet_tie.derived_only.is_empty());
    assert_eq!(
        statuses(&tie),
        vec![
            (
                "Cost of Sales :",
                TieStatus::NotCompared {
                    reason: "no_derived_line_of_that_name"
                }
            ),
            ("Purchase Accounts", TieStatus::Matched),
        ]
    );
    assert!(tie.derived_only.is_empty());
}

/// One month of a 29,900-voucher book. In a part-year window a P&L ledger's
/// Trial Balance covers the window only, and the year's earlier result sits in
/// the Profit & Loss A/c ledger's opening. The gate still passes.
#[test]
fn a_same_company_part_year_window_on_a_heavy_book_passes_the_gate() {
    let (derived, tie) = Capture {
        guid: "d45bc1b0-e5e3-4261-b3b2-cce3915f42d3",
        trial_balance: include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/statement_trial_balance_dense_month_live.utf16le.xml"
        ),
        groups: include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/statement_groups_dense_live.utf16le.xml"
        ),
        balance_sheet: include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/statement_balance_sheet_dense_month_live.utf16le.xml"
        ),
        profit_and_loss: include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/statement_profit_and_loss_dense_month_live.utf16le.xml"
        ),
    }
    .derive();

    assert!(derived.unclassified.is_empty());
    assert_sum(
        &line(&derived.profit_and_loss, "Sales Accounts").amount,
        "113726661.73",
        1,
        1,
    );
    assert_eq!(
        line(&derived.balance_sheet, "Current Assets").ledger_count,
        121
    );
    let root = derived.profit_and_loss_ledger.as_ref().unwrap();
    assert_eq!(
        root.closing,
        NativeTrialBalanceAmount::Present(decimal("109235760.65"))
    );
    assert_established(&derived.net_result, "113726661.73");
    assert_established(&derived.balance_sheet_profit_and_loss, "222962422.38");
    assert_eq!(
        statuses(&derived.balance_sheet_tie),
        vec![
            ("Capital Account", TieStatus::MatchedEmptyAsZero),
            ("Loans (Liability)", TieStatus::MatchedEmptyAsZero),
            ("Current Liabilities", TieStatus::MatchedEmptyAsZero),
            ("Profit & Loss A/c", TieStatus::Matched),
            ("Current Assets", TieStatus::Matched),
        ]
    );
    assert!(derived.balance_sheet_tie.derived_only.is_empty());
    assert_eq!(statuses(&tie), vec![("Sales Accounts", TieStatus::Matched)]);
    assert!(tie.derived_only.is_empty());
}

/// A Tally line that reads 0.00 in both columns is a zero amount: over a derived
/// line with an amount it is a difference and names the line, on both
/// statements (#1067). Before the fix it was "not compared" and no failure, so
/// the results were established.
#[test]
fn a_present_zero_tally_line_over_a_derived_amount_is_a_difference() {
    // Balance Sheet: Tally's "Current Assets" reads 0.00 / 0.00; derived is -11027.00.
    let tally = balance_sheet(&[
        ("Current Assets", "0.00", "0.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Current Assets"]));

    // Profit and Loss: Tally's "Sales Accounts" reads 0.00 / 0.00; derived is 4027.00.
    let derived = derive_with_profit_and_loss(
        known_lab(),
        &profit_and_loss(&[("Sales Accounts", "0.00", "0.00")]),
    );
    assert_movement_refused(&derived, &["Sales Accounts"]);
}

/// A present zero over a derived zero still ties, and a line with both columns
/// present and an amount is still uncompared and still refuses.
#[test]
fn a_present_zero_over_a_derived_zero_ties_and_a_present_amount_still_refuses() {
    let tally = balance_sheet(&[
        ("Capital Account", "0.00", "0.00"),
        ("Current Assets", "", "-11027.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_established(&derived.net_result, "4027.00");
    assert!(derived
        .balance_sheet_tie
        .lines
        .iter()
        .any(|line| line.name == "Capital Account" && line.status == TieStatus::Matched));

    let tally = balance_sheet(&[
        ("Current Assets", "-1.00", "-11027.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Current Assets"]));
    // One zero column beside an amount, in either column, is not a zero line:
    // it stays uncompared, and refuses.
    let tally = balance_sheet(&[
        ("Current Assets", "-11027.00", "0.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Current Assets"]));
    assert!(derived
        .balance_sheet_tie
        .lines
        .iter()
        .any(|line| line.name == "Current Assets"
            && line.status
                == TieStatus::NotCompared {
                    reason: "both_tally_columns_present"
                }));
    // The Profit and Loss gate, the same: an amount in one column of a pair
    // refuses, in either order.
    for (sub, main) in [
        ("4027.00", "0.00"),
        ("0.00", "4027.00"),
        ("1.00", "4026.00"),
    ] {
        let derived = derive_with_profit_and_loss(
            known_lab(),
            &profit_and_loss(&[("Sales Accounts", sub, main)]),
        );
        assert_movement_refused(&derived, &["Sales Accounts"]);
    }
    let tally = balance_sheet(&[
        ("Current Assets", "0.00", "-11027.00"),
        ("Profit & Loss A/c", "", "11027.00"),
    ]);
    let derived = derive(known_lab(), &tally);
    assert_every_result(&derived, &differs(&["Current Assets"]));
    assert!(derived
        .balance_sheet_tie
        .lines
        .iter()
        .any(|line| line.name == "Current Assets"
            && line.status
                == TieStatus::NotCompared {
                    reason: "both_tally_columns_present"
                }));
}

// Tally's own closing stock on an integrated inventory book (#692). The four
// sources are the captures of `BRIDGE TIEOUT B`: no opening stock, three items,
// one purchase and one sale. Tests marked "synthetic mutation" edit a captured
// Profit and Loss line to reach a branch the capture does not; they make no
// claim about Tally output.

const TIEOUT_B: Capture = Capture {
    guid: "d64eda55-da7c-4106-a171-3e3cb6cdd331",
    trial_balance: include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/statement_trial_balance_stock_book_fy_live.utf16le.xml"
    ),
    groups: include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/statement_groups_stock_book_live.utf16le.xml"
    ),
    balance_sheet: include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/statement_balance_sheet_stock_book_fy_live.utf16le.xml"
    ),
    profit_and_loss: include_bytes!(
        "../../crates/bridge-tally-protocol/tests/fixtures/statement_profit_and_loss_stock_book_fy_live.utf16le.xml"
    ),
};

/// The flags Tally gave for the book (its captured answer), as
/// `IntegratedInventory` reads them.
fn integrated() -> InventoryFlags {
    let flags = parse_company_inventory_flags(
        &utf16(include_bytes!(
            "../../crates/bridge-tally-protocol/tests/fixtures/company_inventory_flags_stock_book_live.utf16le.xml"
        )),
        TIEOUT_B.guid,
    )
    .unwrap()
    .flags;
    IntegratedInventory::from_flags(&flags)
}

/// The inventory book with `profit_and_loss` as Tally's own P&L (`None`: not read).
fn tieout_b_with(
    profit_and_loss: Option<&NativeStatement>,
    inventory: InventoryFlags,
) -> DerivedStatements {
    let (report, groups, balance_sheet, _) = TIEOUT_B.parts();
    derive_statements(
        &admitted(report),
        &groups,
        &balance_sheet,
        profit_and_loss,
        inventory,
    )
    .unwrap()
}

/// The inventory book's captured P&L lines, edited by `edit`.
fn captured_stock_lines() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("Sales Accounts", "", "1400.00"),
        ("Cost of Sales :", "", "-900.00"),
        ("Opening Stock", "", ""),
        ("Add: Purchase Accounts", "-3000.00", ""),
        ("Less: Closing Stock", "-2100.00", ""),
        ("Indirect Expenses", "", "-500.00"),
    ]
}

fn tieout_b_edited(
    edit: impl FnOnce(&mut Vec<(&'static str, &'static str, &'static str)>),
    inventory: InventoryFlags,
) -> DerivedStatements {
    let mut lines = captured_stock_lines();
    edit(&mut lines);
    tieout_b_with(Some(&profit_and_loss(&lines)), inventory)
}

fn replace_line(
    lines: &mut [(&'static str, &'static str, &'static str)],
    name: &str,
    sub: &'static str,
    main: &'static str,
) {
    let slot = lines.iter_mut().find(|line| line.0 == name).unwrap();
    slot.1 = sub;
    slot.2 = main;
}

fn declined(derived: &DerivedStatements) -> ClosingStockDeclined {
    match &derived.closing_stock {
        ClosingStockUse::Declined { why, .. } => *why,
        other => panic!("expected a declined closing stock, got {other:?}"),
    }
}

#[test]
fn the_edited_lines_are_the_captured_ones() {
    // The mutation tests below start from these lines; this ties them to the capture.
    let (_, _, _, captured) = TIEOUT_B.parts();
    assert_eq!(captured, profit_and_loss(&captured_stock_lines()));
}

#[test]
fn tallys_closing_stock_closes_every_gap_on_the_inventory_book() {
    let (_, _, _, tally) = TIEOUT_B.parts();
    let derived = tieout_b_with(Some(&tally), integrated());
    match &derived.closing_stock {
        ClosingStockUse::Applied { value } => assert!(value.numeric_eq(&decimal("-2100.00"))),
        other => panic!("expected Applied, got {other:?}"),
    }
    // Gross 500 and net 0 as Tally's screen shows them, the carried line 0.
    assert_established(&derived.gross_result, "500.00");
    assert_established(&derived.net_result, "0");
    assert_established(&derived.balance_sheet_profit_and_loss, "0");
    // The derived Current Assets line is still the Trial Balance's: the amount
    // is a line of its own, not folded into a group.
    assert!(line(&derived.balance_sheet, "Current Assets")
        .amount
        .sum
        .numeric_eq(&decimal("-50900.00")));
    let tie = derived.profit_and_loss_tie.unwrap();
    let stock = tie
        .lines
        .iter()
        .find(|line| line.name == "Less: Closing Stock")
        .unwrap();
    assert_eq!(
        stock.status,
        TieStatus::NotCompared {
            reason: "tally_closing_stock_used"
        }
    );
    let purchase = tie
        .lines
        .iter()
        .find(|line| line.name == "Add: Purchase Accounts")
        .unwrap();
    assert_eq!(purchase.status, TieStatus::Matched);
    assert!(tie.derived_only.is_empty());
}

#[test]
fn without_the_flags_the_inventory_book_keeps_todays_refusal() {
    let (_, _, _, tally) = TIEOUT_B.parts();
    let derived = tieout_b_with(Some(&tally), InventoryFlags::NotShownIntegrated);
    assert_eq!(
        declined(&derived),
        ClosingStockDeclined::BookNotShownIntegrated
    );
    // A declined read is the result of a book without the figure.
    let expected = differs(&["Profit & Loss A/c", "Current Assets"]);
    assert_every_result(&derived, &expected);
}

#[test]
fn without_a_profit_and_loss_the_inventory_book_keeps_todays_refusal() {
    let derived = tieout_b_with(None, integrated());
    assert_eq!(derived.closing_stock, ClosingStockUse::NotApplicable);
    assert_every_result(&derived, &differs(&["Profit & Loss A/c", "Current Assets"]));
}

#[test]
fn a_stock_book_whose_figure_does_not_close_every_gap_names_the_lines() {
    // synthetic mutation: closing stock off by a paisa closes the Cost of Sales
    // heading only approximately and Current Assets not at all.
    let derived = tieout_b_edited(
        |lines| replace_line(lines, "Less: Closing Stock", "-2100.01", ""),
        integrated(),
    );
    let ClosingStockUse::Declined { why, lines } = &derived.closing_stock else {
        panic!("expected a declined closing stock");
    };
    assert_eq!(*why, ClosingStockDeclined::DoesNotCloseEveryGap);
    assert!(lines.contains(&"Current Assets".to_string()), "{lines:?}");
    assert!(lines.contains(&"Cost of Sales :".to_string()), "{lines:?}");
    assert_every_result(&derived, &differs(&["Profit & Loss A/c", "Current Assets"]));

    // synthetic mutation: only the heading is off.
    let derived = tieout_b_edited(
        |lines| replace_line(lines, "Cost of Sales :", "", "-900.01"),
        integrated(),
    );
    let ClosingStockUse::Declined { why, lines } = &derived.closing_stock else {
        panic!("expected a declined closing stock");
    };
    assert_eq!(*why, ClosingStockDeclined::DoesNotCloseEveryGap);
    assert_eq!(lines, &vec!["Cost of Sales :".to_string()]);
}

/// The Profit and Loss of `BRIDGE TIEOUT D` (a book with opening stock and a matching capital entry),
/// as Tally answered it, for the year and for 1 October to 31 March.
const OPENING_STOCK_BOOK_YEAR: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/statement_profit_and_loss_opening_stock_book_fy_live.utf16le.xml"
);
const OPENING_STOCK_BOOK_OCT_MAR: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/statement_profit_and_loss_opening_stock_book_oct_mar_live.utf16le.xml"
);

#[test]
fn a_captured_book_with_opening_stock_is_declined_by_its_opening_stock_line() {
    // Real bytes, not a mutation: the year (read before the October sale) carries Opening Stock
    // -1000.00 and Closing Stock -2640.00; the second half-year carries -2640.00 and -2040.00. The slice is for a book whose
    // Opening Stock Tally shows empty, so both decline, and the reason is the opening stock.
    for bytes in [OPENING_STOCK_BOOK_YEAR, OPENING_STOCK_BOOK_OCT_MAR] {
        let statement =
            parse_native_statement(NativeStatementKind::ProfitAndLoss, &utf16(bytes)).unwrap();
        assert_eq!(
            TallyClosingStock::from_profit_and_loss(&statement),
            Err(ClosingStockDeclined::OpeningStockNotShownEmpty)
        );
    }
}

#[test]
fn a_closing_stock_not_as_captured_is_declined_by_its_shape() {
    // synthetic mutation: opening stock carrying an amount.
    let derived = tieout_b_edited(
        |lines| replace_line(lines, "Opening Stock", "-100.00", ""),
        integrated(),
    );
    assert_eq!(
        declined(&derived),
        ClosingStockDeclined::OpeningStockNotShownEmpty
    );
    // synthetic mutation: no Opening Stock line at all.
    let derived = tieout_b_edited(
        |lines| lines.retain(|line| line.0 != "Opening Stock"),
        integrated(),
    );
    assert_eq!(
        declined(&derived),
        ClosingStockDeclined::OpeningStockNotShownEmpty
    );
    // synthetic mutation: a credit.
    let derived = tieout_b_edited(
        |lines| replace_line(lines, "Less: Closing Stock", "2100.00", ""),
        integrated(),
    );
    assert_eq!(declined(&derived), ClosingStockDeclined::NotADebit);
    // synthetic mutation: the amount in the main column.
    let derived = tieout_b_edited(
        |lines| replace_line(lines, "Less: Closing Stock", "", "-2100.00"),
        integrated(),
    );
    assert_eq!(declined(&derived), ClosingStockDeclined::ShapeNotAsSeen);
    // synthetic mutation: both columns.
    let derived = tieout_b_edited(
        |lines| replace_line(lines, "Less: Closing Stock", "-2100.00", "-2100.00"),
        integrated(),
    );
    assert_eq!(declined(&derived), ClosingStockDeclined::ShapeNotAsSeen);
}

#[test]
fn only_the_captured_spelling_of_the_line_is_the_figure() {
    // synthetic mutation: another spelling is not the line, so nothing is offered.
    for spelling in [
        "Closing Stock",
        "less: closing stock",
        "Less: Closing Stock ",
    ] {
        let mut lines = captured_stock_lines();
        lines
            .iter_mut()
            .find(|line| line.0 == "Less: Closing Stock")
            .unwrap()
            .0 = spelling;
        let derived = tieout_b_with(Some(&profit_and_loss(&lines)), integrated());
        assert_eq!(
            derived.closing_stock,
            ClosingStockUse::NotApplicable,
            "{spelling}"
        );
    }
    // An empty or zero line offers nothing either.
    for sub in ["", "0.00"] {
        let derived = tieout_b_edited(
            |lines| replace_line(lines, "Less: Closing Stock", sub, ""),
            integrated(),
        );
        assert_eq!(
            derived.closing_stock,
            ClosingStockUse::NotApplicable,
            "{sub:?}"
        );
    }
}

#[test]
fn a_stock_in_hand_balance_still_blocks_before_the_figure_is_considered() {
    // synthetic mutation: a ledger under Stock-in-Hand carrying an amount.
    let (mut report, groups, balance_sheet, tally) = TIEOUT_B.parts();
    let stock_group = groups
        .iter()
        .find(|group| group.reserved_name.as_deref().map(str::trim) == Some("Stock-in-Hand"))
        .expect("the group tree holds Stock-in-Hand");
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.name == "Lab Freight")
        .unwrap();
    row.parent = PartyLedgerMasterFieldObservation::Returned(stock_group.name.clone());
    let derived = derive_statements(
        &admitted(report),
        &groups,
        &balance_sheet,
        Some(&tally),
        integrated(),
    )
    .unwrap();
    assert_eq!(derived.closing_stock, ClosingStockUse::NotApplicable);
    assert_every_result(
        &derived,
        &blocked(NotEstablishedReason::ClosingStockNotDerivableFromTrialBalance),
    );
}

#[test]
fn a_book_with_no_stock_is_untouched_by_the_flags() {
    // READS LAB has no closing-stock line: the flags change nothing.
    let (report, groups, balance_sheet, tally) = READS_LAB.parts();
    let with = derive_statements(
        &admitted(report.clone()),
        &groups,
        &balance_sheet,
        Some(&tally),
        integrated(),
    )
    .unwrap();
    let without = derive_statements(
        &admitted(report),
        &groups,
        &balance_sheet,
        Some(&tally),
        InventoryFlags::NotRead,
    )
    .unwrap();
    assert_eq!(with, without);
    assert_eq!(with.closing_stock, ClosingStockUse::NotApplicable);
}

#[test]
fn the_stock_books_purchase_line_name_is_not_an_alias_without_closing_stock() {
    // synthetic mutation: READS LAB's purchase line under the name a stock book
    // uses. With no closing stock used it names no derived line, so it is an
    // uncompared line with an amount, and the results are refused.
    let derived = reads_lab_with_profit_and_loss(&[
        ("Cost of Sales :", "", "-4250.00"),
        ("Add: Purchase Accounts", "-4250.00", ""),
    ]);
    let Established::NotEstablished { reason, lines } = &derived.net_result else {
        panic!("expected a refusal, got {:?}", derived.net_result);
    };
    assert_eq!(*reason, NotEstablishedReason::TallyProfitAndLossDiffers);
    assert!(
        lines.contains(&"Add: Purchase Accounts".to_string()),
        "{lines:?}"
    );
}

#[test]
fn only_both_flags_yes_show_an_integrated_inventory() {
    let flags = |integrated, inventory_on| NativeInventoryFlags {
        integrated,
        inventory_on,
        batchwise: NativeFlag::No,
    };
    assert!(matches!(
        IntegratedInventory::from_flags(&flags(NativeFlag::Yes, NativeFlag::Yes)),
        InventoryFlags::Integrated(_)
    ));
    for (integrated, inventory_on) in [
        (NativeFlag::No, NativeFlag::Yes),
        (NativeFlag::Unknown, NativeFlag::Yes),
        (NativeFlag::Yes, NativeFlag::No),
        (NativeFlag::Yes, NativeFlag::Unknown),
    ] {
        assert!(
            matches!(
                IntegratedInventory::from_flags(&flags(integrated, inventory_on)),
                InventoryFlags::NotShownIntegrated
            ),
            "{integrated:?} {inventory_on:?}"
        );
    }
}

#[test]
fn a_closing_stock_without_the_cost_of_sales_heading_is_declined() {
    // synthetic mutation: the heading the figure is checked against is missing.
    let derived = tieout_b_edited(
        |lines| lines.retain(|line| line.0 != "Cost of Sales :"),
        integrated(),
    );
    assert_eq!(declined(&derived), ClosingStockDeclined::ShapeNotAsSeen);
}

#[test]
fn flags_that_could_not_be_read_are_their_own_reason() {
    let (_, _, _, tally) = TIEOUT_B.parts();
    let derived = tieout_b_with(Some(&tally), InventoryFlags::NotRead);
    assert_eq!(
        declined(&derived),
        ClosingStockDeclined::InventoryFlagsNotRead
    );
    assert_every_result(&derived, &differs(&["Profit & Loss A/c", "Current Assets"]));
}

#[test]
fn a_book_whose_trial_balance_does_not_balance_does_not_use_the_figure() {
    // synthetic mutation: the Bank ledger opens 1,000 higher and Tally's Current
    // Assets are made to tie to it, so every compared line ties with the figure
    // while the derived lines no longer sum to zero: a difference in opening
    // balances this book's Balance Sheet export would not show.
    let (mut report, groups, mut balance_sheet, tally) = TIEOUT_B.parts();
    let bank = row(&mut report, "Lab Bank");
    bank.opening = NativeTrialBalanceAmount::Present(decimal("-49000.00"));
    bank.closing = NativeTrialBalanceAmount::Present(decimal("-48500.00"));
    let current_assets = balance_sheet
        .lines
        .iter_mut()
        .find(|line| line.name == "Current Assets")
        .unwrap();
    current_assets.main = NativeStatementAmount::Present(decimal("-52000.00"));
    let derived = derive_statements(
        &admitted(report),
        &groups,
        &balance_sheet,
        Some(&tally),
        integrated(),
    )
    .unwrap();
    assert_eq!(
        declined(&derived),
        ClosingStockDeclined::BalanceSheetDoesNotBalance
    );
}

// A Balance Sheet export that does not balance (#2063). The three sources are the captures of
// `BRIDGE TIEOUT D` after one stock item's opening value was raised by 100.00 (TallyPrime 7.2,
// 27 Oct 2026): Tally's own screen shows a `Difference in opening balances` of 100.00, and the
// export has no line for it. The group tree is TIEOUT B's (the default tree every company has),
// as in the tests above that cross companies.

const OPENING_DIFFERENCE_GUID: &str = "6a9518b2-a88c-417c-a700-40243f9d9119";

fn opening_difference() -> (NativeTrialBalance, NativeStatement, NativeStatement) {
    (
        parse_native_trial_balance(
            &utf16(include_bytes!(
                "../../crates/bridge-tally-protocol/tests/fixtures/statement_trial_balance_opening_difference_fy_live.utf16le.xml"
            )),
            OPENING_DIFFERENCE_GUID,
        )
        .unwrap(),
        parse_native_statement(
            NativeStatementKind::BalanceSheet,
            &utf16(include_bytes!(
                "../../crates/bridge-tally-protocol/tests/fixtures/statement_balance_sheet_opening_difference_fy_live.utf16le.xml"
            )),
        )
        .unwrap(),
        parse_native_statement(
            NativeStatementKind::ProfitAndLoss,
            &utf16(include_bytes!(
                "../../crates/bridge-tally-protocol/tests/fixtures/statement_profit_and_loss_opening_difference_fy_live.utf16le.xml"
            )),
        )
        .unwrap(),
    )
}

#[test]
fn the_captured_export_that_leaves_out_the_opening_difference_says_its_sides_and_establishes_nothing(
) {
    let (report, balance_sheet, profit_and_loss) = opening_difference();
    let (_, groups, _, _) = TIEOUT_B.parts();
    let derived = derive_statements(
        &admitted(report),
        &groups,
        &balance_sheet,
        Some(&profit_and_loss),
        integrated(),
    )
    .unwrap();
    let sides = derived.tally_balance_sheet_sides.as_ref().unwrap();
    assert!(sides.credits.numeric_eq(&decimal("4846.67")));
    assert!(sides.debits.numeric_eq(&decimal("4946.67")));
    for result in [
        &derived.gross_result,
        &derived.net_result,
        &derived.balance_sheet_profit_and_loss,
    ] {
        assert!(
            matches!(result, Established::NotEstablished { .. }),
            "{result:?}"
        );
    }
}

#[test]
fn a_balance_sheet_that_balances_has_no_sides_to_report() {
    // Every captured Balance Sheet of a balanced book, and the synthetic ones that tie.
    for capture in [&READS_LAB, &TIEOUT_B] {
        let (report, groups, balance_sheet, _) = capture.parts();
        let derived = derive_statements(
            &admitted(report),
            &groups,
            &balance_sheet,
            None,
            InventoryFlags::NotRead,
        )
        .unwrap();
        assert_eq!(derived.tally_balance_sheet_sides, None);
    }
    assert_eq!(
        derive(known_lab(), &known_lab_balance_sheet()).tally_balance_sheet_sides,
        None
    );
}

#[test]
fn the_sides_are_each_lines_amount_by_sign_and_one_paisa_is_a_difference() {
    let sides =
        |lines: &[(&str, &str, &str)]| BalanceSheetSides::of(&balance_sheet(lines)).unwrap();
    // An empty line is no side; an amount in the sub column alone counts, as the gate
    // reads it; with both columns present the main column stands for the line.
    assert_eq!(
        sides(&[
            ("A", "", "100.00"),
            ("B", "", ""),
            ("C", "-30.00", ""),
            ("D", "", "-70.00"),
        ]),
        None
    );
    assert_eq!(
        sides(&[("A", "", "100.00"), ("E", "-1.00", "-100.00")]),
        None
    );
    let sub_only = sides(&[("A", "", "100.00"), ("C", "-5.00", "")]).unwrap();
    assert!(sub_only.credits.numeric_eq(&decimal("100.00")));
    assert!(sub_only.debits.numeric_eq(&decimal("5.00")));
    let differing = sides(&[("A", "", "100.00"), ("D", "", "-100.01")]).unwrap();
    assert!(differing.credits.numeric_eq(&decimal("100.00")));
    assert!(differing.debits.numeric_eq(&decimal("100.01")));
}

/// The sides come last: a Profit and Loss that differs from Tally's keeps its own reason
/// for gross and net, and the Balance Sheet's carried line, which that statement does not
/// gate, takes the new one.
#[test]
fn a_profit_and_loss_that_differs_keeps_its_reason_beside_unbalanced_sides() {
    let report = parse_native_trial_balance(OPENING_YEAR, OPENING_YEAR_GUID).unwrap();
    let tally = balance_sheet(&[
        ("Capital Account", "", "125000.00"),
        ("Current Liabilities", "", "88000.00"),
        ("Profit & Loss A/c", "", ""),
        ("Current Assets", "", "-262833.50"),
    ]);
    let derived = derive_statements(
        &admitted(report),
        &groups(),
        &tally,
        Some(&profit_and_loss(&[("Sales Accounts", "", "1.00")])),
        InventoryFlags::NotRead,
    )
    .unwrap();
    let expected = Established::NotEstablished {
        reason: NotEstablishedReason::TallyProfitAndLossDiffers,
        lines: vec!["Sales Accounts".to_string()],
    };
    assert_eq!(derived.gross_result, expected);
    assert_eq!(derived.net_result, expected);
    assert_eq!(
        derived.balance_sheet_profit_and_loss,
        blocked(NotEstablishedReason::TallyBalanceSheetDoesNotBalance)
    );
    assert!(derived.tally_balance_sheet_sides.is_some());
}
