//! The first three tests reproduce the tie measured on 6 Oct 2026 on one
//! synthetic company: Tally's own Cash Flow for three windows, parsed from the
//! committed live captures, against the debit and credit totals of the book's
//! cash and bank ledgers as `trial_balance` returned them for the same windows
//! (Cash 5,500.00 and a bank ledger 17,970,481.22 for the year; the bank
//! ledger alone 7,263,013.22 for April to June and 5,472,348.20 for June).
//! The Trial Balance rows here are typed from those figures, not captured
//! XML, and the group tree is a verbatim capture from another company whose
//! predefined groups are the same in every company.
//!
//! Tests marked "synthetic" alter a row or a group to reach a branch the
//! captures do not; they make no claim about Tally output.

use super::*;
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::{
    decode_tally_xml_response_bytes_limited,
    native_cash_flow::{
        parse_native_cash_flow, CashFlowMonth, NativeCashFlowRow, WholeMonthWindow,
    },
    native_outstandings::parse_native_group_snapshot,
    native_trial_balance::{NativeTrialBalance, NativeTrialBalanceAmount, NativeTrialBalanceRow},
    outstandings_shared::DateBoundaryProfile,
    ExpectedTallyTextEncoding, PartyLedgerMasterFieldObservation,
};

const GROUPS: &str = include_str!(
    "../../crates/bridge-tally-protocol/tests/fixtures/native/group_snapshot_aarav_with_computed_company_guid.xml"
);
const GROUPS_GUID: &str = "bb8ad19e-6aef-4239-a917-87fec0c6215e";
const CASH_FLOW_FY: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/builtin_cash_flow_probe_b_fy_live.utf16le.xml"
);
const CASH_FLOW_APR_JUN: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/builtin_cash_flow_probe_b_apr_jun_live.utf16le.xml"
);
const CASH_FLOW_JUNE: &[u8] = include_bytes!(
    "../../crates/bridge-tally-protocol/tests/fixtures/builtin_cash_flow_probe_b_june_live.utf16le.xml"
);

fn groups() -> Vec<TallyNamedMaster> {
    parse_native_group_snapshot(GROUPS, GROUPS_GUID).unwrap()
}

fn decimal(value: &str) -> ExactDecimal {
    ExactDecimal::parse(value).unwrap()
}

fn amount(value: &str) -> NativeTrialBalanceAmount {
    if value.is_empty() {
        NativeTrialBalanceAmount::PresentEmpty
    } else {
        NativeTrialBalanceAmount::Present(decimal(value))
    }
}

/// One Trial Balance row: its parent group and its debit and credit.
fn row(name: &str, parent: &str, debit: &str, credit: &str) -> NativeTrialBalanceRow {
    NativeTrialBalanceRow {
        name: name.to_string(),
        guid: format!("guid-{name}"),
        parent: PartyLedgerMasterFieldObservation::Returned(parent.to_string()),
        opening: amount("0.00"),
        debit: amount(debit),
        credit: amount(credit),
        closing: amount(""),
    }
}

fn admitted(rows: Vec<NativeTrialBalanceRow>) -> SingleCurrencyTrialBalance {
    SingleCurrencyTrialBalance::admitted_for_tests(NativeTrialBalance { rows })
}

fn captured(bytes: &[u8], from: &str, to: &str) -> NativeCashFlow {
    let xml = decode_tally_xml_response_bytes_limited(
        bytes,
        "text/xml; charset=utf-16",
        ExpectedTallyTextEncoding::Utf16Le,
        bytes.len(),
    )
    .unwrap()
    .text;
    let window = WholeMonthWindow::new(
        DateBoundaryProfile::ModeAgnostic,
        TallyDate::parse(from).unwrap(),
        TallyDate::parse(to).unwrap(),
    )
    .unwrap();
    parse_native_cash_flow(&xml, &window).unwrap()
}

/// A typed Cash Flow of one month, for the branches the captures do not reach.
fn typed_cash_flow(closing: &str) -> NativeCashFlow {
    let slot = |value: &str| {
        if value.is_empty() {
            bridge_tally_protocol::native_statement_reports::NativeStatementAmount::Empty
        } else {
            bridge_tally_protocol::native_statement_reports::NativeStatementAmount::Present(
                decimal(value),
            )
        }
    };
    NativeCashFlow {
        rows: vec![NativeCashFlowRow {
            month: CashFlowMonth {
                year: 2025,
                month: 9,
            },
            debit: slot(""),
            credit: slot(""),
            closing: slot(closing),
        }],
    }
}

fn check(rows: Vec<NativeTrialBalanceRow>, cash_flow: &NativeCashFlow) -> CashFlowCheck {
    check_cash_flow(&admitted(rows), &groups(), cash_flow).unwrap()
}

fn assert_tied(check: &CashFlowCheck, net: &str, money_ledgers: usize) {
    match check {
        CashFlowCheck::Tied {
            net: got,
            money_ledgers: count,
        } => {
            assert!(got.numeric_eq(&decimal(net)), "{} != {net}", got.as_str());
            assert_eq!(*count, money_ledgers);
        }
        other => panic!("expected tied at {net}, got {other:?}"),
    }
}

// ---- the measured tie ----

#[test]
fn the_captured_year_ties_to_the_cash_and_bank_ledgers_of_the_trial_balance() {
    let rows = vec![
        row("Cash", "Cash-in-Hand", "-5500.00", ""),
        row("RO Bank", "Bank Accounts", "", ""),
        row("W1 Bank", "Bank Accounts", "-17970481.22", ""),
        // Not money: a debtor's movement is ignored.
        row(
            "W1 Debtor 01",
            "Sundry Debtors",
            "-5246416.12",
            "1504222.68",
        ),
    ];
    let result = check(rows, &captured(CASH_FLOW_FY, "20250401", "20260331"));
    assert_tied(&result, "-17975981.22", 3);
}

#[test]
fn the_captured_quarter_and_month_tie_to_the_bank_ledger_alone() {
    let quarter = check(
        vec![row("W1 Bank", "Bank Accounts", "-7263013.22", "")],
        &captured(CASH_FLOW_APR_JUN, "20250401", "20250630"),
    );
    assert_tied(&quarter, "-7263013.22", 1);
    let june = check(
        vec![row("W1 Bank", "Bank Accounts", "-5472348.20", "")],
        &captured(CASH_FLOW_JUNE, "20250601", "20250630"),
    );
    assert_tied(&june, "-5472348.20", 1);
}

#[test]
fn a_difference_of_one_paisa_is_not_a_tie_and_both_figures_are_returned() {
    let rows = vec![
        row("Cash", "Cash-in-Hand", "-5500.00", ""),
        row("W1 Bank", "Bank Accounts", "-17970481.21", ""),
    ];
    match check(rows, &captured(CASH_FLOW_FY, "20250401", "20260331")) {
        CashFlowCheck::Differs {
            tally_net,
            ledger_net,
            money_ledgers,
            unclassified_with_movement,
        } => {
            assert!(tally_net.numeric_eq(&decimal("-17975981.22")));
            assert!(ledger_net.numeric_eq(&decimal("-17975981.21")));
            assert_eq!((money_ledgers, unclassified_with_movement), (2, 0));
        }
        other => panic!("expected a difference, got {other:?}"),
    }
}

// ---- branches the captures do not reach ----

#[test]
fn a_ledger_under_a_user_created_group_inside_bank_accounts_counts_as_bank() {
    // synthetic: a group a user created under Bank Accounts (an empty RESERVEDNAME).
    let mut groups = groups();
    groups.push(TallyNamedMaster {
        name: "Branch Banks".to_string(),
        parent: PartyLedgerMasterFieldObservation::Returned("Bank Accounts".to_string()),
        reserved_name: Some(String::new()),
    });
    let rows = vec![row("North Bank", "Branch Banks", "-300.00", "")];
    let result = check_cash_flow(&admitted(rows), &groups, &typed_cash_flow("-300.00")).unwrap();
    assert_tied(&result, "-300.00", 1);
}

#[test]
fn a_net_is_shown_at_the_scale_of_its_terms_not_with_its_trailing_zeros_dropped() {
    let rows = vec![row("Bank", "Bank Accounts", "-100.00", "")];
    match check(rows, &typed_cash_flow("-100.00")) {
        CashFlowCheck::Tied { net, .. } => assert_eq!(net.as_str(), "-100.00"),
        other => panic!("expected a tie, got {other:?}"),
    }
    // A zero net carries the scale too.
    let rows = vec![
        row("Cash", "Cash-in-Hand", "", "2000.50"),
        row("Bank", "Bank Accounts", "-2000.50", ""),
    ];
    match check(rows, &typed_cash_flow("")) {
        CashFlowCheck::Tied { net, .. } => assert_eq!(net.as_str(), "0.00"),
        other => panic!("expected a tie, got {other:?}"),
    }
}

#[test]
fn a_contra_between_two_money_ledgers_counts_on_both_sides_and_nets_to_nothing() {
    // synthetic: cash to bank 2,000.00 moves both ledgers and nothing leaves the money set.
    let rows = vec![
        row("Cash", "Cash-in-Hand", "", "2000.00"),
        row("Bank", "Bank Accounts", "-2000.00", ""),
    ];
    assert_tied(&check(rows, &typed_cash_flow("")), "0.00", 2);
}

#[test]
fn an_inflow_and_an_outflow_in_one_ledger_net_in_the_tie() {
    // synthetic: the credit column is positive, so -700.00 in and 300.00 out is -400.00 net.
    let rows = vec![row("Bank", "Bank Accounts", "-700.00", "300.00")];
    assert_tied(&check(rows, &typed_cash_flow("-400.00")), "-400.00", 1);
}

#[test]
fn a_bank_od_ledger_with_movement_is_refused_until_it_has_been_measured() {
    // synthetic: Tally's cash flow may or may not count a Bank OD A/c ledger; unmeasured.
    let rows = vec![
        row("W1 Bank", "Bank Accounts", "-100.00", ""),
        row("HDFC CC", "Bank OD A/c", "-250.00", ""),
    ];
    assert_eq!(
        check(rows, &typed_cash_flow("-100.00")),
        CashFlowCheck::MoneyGroupUnmeasured { ledgers: 1 }
    );
}

#[test]
fn a_bank_od_ledger_without_movement_does_not_stop_the_tie() {
    // synthetic: empty and zero amounts are no movement.
    let rows = vec![
        row("W1 Bank", "Bank Accounts", "-100.00", ""),
        row("HDFC CC", "Bank OD A/c", "", ""),
        row("Old CC", "Bank OD A/c", "0.00", "0.00"),
    ];
    assert_tied(&check(rows, &typed_cash_flow("-100.00")), "-100.00", 1);
}

#[test]
fn a_ledger_that_cannot_be_classified_is_counted_when_the_tie_fails() {
    // synthetic: a parent that is not in the group tree, with movement.
    let rows = vec![
        row("W1 Bank", "Bank Accounts", "-100.00", ""),
        row("Orphan", "Not A Group", "-50.00", ""),
    ];
    match check(rows, &typed_cash_flow("-150.00")) {
        CashFlowCheck::Differs {
            unclassified_with_movement,
            ..
        } => assert_eq!(unclassified_with_movement, 1),
        other => panic!("expected a difference, got {other:?}"),
    }
}

#[test]
fn a_book_with_no_cash_or_bank_ledger_and_an_empty_cash_flow_has_nothing_to_compare() {
    // synthetic: two zeros over no ledger agree about nothing, and are not called a tie.
    let rows = vec![row("A Debtor", "Sundry Debtors", "-10.00", "")];
    assert_eq!(
        check(rows, &typed_cash_flow("")),
        CashFlowCheck::NothingToCompare
    );
}

#[test]
fn a_book_with_no_cash_or_bank_ledger_and_a_cash_flow_with_a_figure_differs() {
    // synthetic: Tally printed a figure the trial balance's money ledgers cannot account for.
    let rows = vec![row("A Debtor", "Sundry Debtors", "-10.00", "")];
    match check(rows, &typed_cash_flow("-10.00")) {
        CashFlowCheck::Differs {
            money_ledgers,
            tally_net,
            ledger_net,
            ..
        } => {
            assert_eq!(money_ledgers, 0);
            assert!(tally_net.numeric_eq(&decimal("-10.00")));
            assert!(ledger_net.is_zero());
        }
        other => panic!("expected a difference, got {other:?}"),
    }
}

#[test]
fn money_ledgers_with_no_movement_and_an_empty_cash_flow_tie_at_zero() {
    // A checked "no movement": both sides were compared and both are empty.
    let rows = vec![row("Cash", "Cash-in-Hand", "", "")];
    assert_tied(&check(rows, &typed_cash_flow("")), "0", 1);
}

#[test]
fn empty_amounts_are_left_out_of_both_sums() {
    let rows = vec![
        row("Cash", "Cash-in-Hand", "", ""),
        row("W1 Bank", "Bank Accounts", "-100.00", ""),
    ];
    assert_tied(&check(rows, &typed_cash_flow("-100.00")), "-100.00", 2);
}

#[test]
fn the_refusal_codes_are_stable_and_distinct() {
    assert_eq!(
        CashFlowCheck::MoneyGroupUnmeasured { ledgers: 1 }.refusal_code(),
        Some("cash_flow_money_group_unmeasured")
    );
    let differs = CashFlowCheck::Differs {
        tally_net: decimal("1"),
        ledger_net: decimal("2"),
        money_ledgers: 1,
        unclassified_with_movement: 0,
    };
    assert_eq!(
        differs.refusal_code(),
        Some("cash_flow_differs_from_trial_balance")
    );
    assert_eq!(
        CashFlowCheck::NothingToCompare.refusal_code(),
        Some("cash_flow_no_money_ledger")
    );
    let tied = CashFlowCheck::Tied {
        net: decimal("0"),
        money_ledgers: 1,
    };
    assert_eq!(tied.refusal_code(), None);
}
