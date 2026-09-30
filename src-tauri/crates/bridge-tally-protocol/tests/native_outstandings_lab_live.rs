// SPDX-License-Identifier: Apache-2.0

//! What the unallocated residual says about its own composition, on captures of
//! a book seeded to hold each case (#945).
//!
//! The four fixtures are live reads of the synthetic company BRIDGE OUTSTANDINGS
//! LAB (TallyPrime Silver 7.1, 30 Sep 2026), each the raw answer to a request the
//! code renders itself; their provenance files give the rendering function and
//! both hashes. The expected values below are worked out by hand from the
//! vouchers the book was seeded with, NOT from this code's output:
//!
//! | party | closing | named bills | residual (closing - bills) | ledger |
//! |---|---|---|---|---|
//! | P01 | -12,000 | -6,000, -6,000 | 0 | bill-wise |
//! | P02 | -7,000 | -10,000 | +3,000 (on-account receipt 4,000 less payment 1,000) | bill-wise |
//! | P03 | -7,500 | none | -7,500 (sale on a ledger that keeps no bills) | NOT bill-wise |
//! | P04 | +6,000 | +6,000 (an advance, listed as a bill) | 0 | bill-wise |
//! | P05 | -7,500 | -12,000, +3,000 (credit note with a reference) | +1,500 (credit note with none) | bill-wise |
//! | P06 | +6,200 | +9,000, -2,000 (debit note with a reference) | -800 (debit note with none) | bill-wise |
//! | P07 | -33,333 | -33,333 (an opening allocated to a reference) | 0, opening -33,333 | bill-wise |
//! | P08 | -20,000 | none | -20,000, opening -20,000 (an opening with no reference) | bill-wise |
//! | P09 | -4,200 | -4,200 | 0 | bill-wise |
//! | P10 | -6,500 | -6,500 | 0 | bill-wise |
//! | P11 | -5,000 | -5,000 | 0 | bill-wise |
//!
//! The fixtures cannot be hand-edited: change a number here to match a change in
//! the code and the test stops proving anything.

use bridge_tally_primitives::TallyDate;
use bridge_tally_protocol::native_outstandings::{
    compute_native_outstandings, parse_native_bill_rows, parse_native_group_snapshot,
    parse_native_ledger_snapshot_for_company, AgeingAnchor, NativeGroupSnapshot,
    NativeMasterSnapshot, NativeOutstandingsResult,
};

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/agent");
const COMPANY_GUID: &str = "49f1fbda-ee59-4a4b-aacf-b45fe32402d7";

fn fixture(name: &str) -> String {
    let bytes = std::fs::read(format!("{FIXTURES}/{name}.utf16le.xml"))
        .unwrap_or_else(|error| panic!("fixture {name} unreadable: {error}"));
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    String::from_utf16(&units).expect("fixture is UTF-16LE")
}

fn computed() -> NativeOutstandingsResult {
    let books_from = TallyDate::parse("20250401").unwrap();
    let as_of = TallyDate::parse("20260630").unwrap();
    let receivable = parse_native_bill_rows(
        &fixture("native-outstandings-lab-bills-receivable"),
        &books_from,
        &as_of,
    )
    .expect("receivable rows parse");
    let payable = parse_native_bill_rows(
        &fixture("native-outstandings-lab-bills-payable"),
        &books_from,
        &as_of,
    )
    .expect("payable rows parse");
    assert_eq!(
        receivable.len(),
        9,
        "the receivable bills the vouchers sum to"
    );
    assert_eq!(
        payable.len(),
        3,
        "an advance, a credit note and a supplier bill"
    );
    let groups =
        parse_native_group_snapshot(&fixture("native-outstandings-lab-groups"), COMPANY_GUID)
            .expect("groups parse");
    let ledgers = parse_native_ledger_snapshot_for_company(
        &fixture("native-outstandings-lab-ledgers"),
        COMPANY_GUID,
    )
    .expect("the ledger snapshot belongs to the target company");
    compute_native_outstandings(
        "BRIDGE OUTSTANDINGS LAB",
        &receivable,
        &payable,
        NativeMasterSnapshot {
            ledgers: &ledgers,
            groups: NativeGroupSnapshot::Complete(&groups),
        },
        AgeingAnchor::DueDate,
        &as_of,
        0,
    )
    .expect("native computation succeeds")
}

fn residual<'a>(
    result: &'a NativeOutstandingsResult,
    party_prefix: &str,
) -> &'a bridge_tally_protocol::native_outstandings::PartyResidual {
    let matches = result
        .residuals
        .iter()
        .filter(|residual| residual.party.starts_with(party_prefix))
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "exactly one residual row for {party_prefix}"
    );
    matches[0]
}

#[test]
fn each_residual_carries_its_ledgers_bill_wise_flag_and_opening() {
    let result = computed();
    // (prefix, residual, bill-wise, opening)
    let expected = [
        ("OL P01 ", "0", true, "0.00"),
        ("OL P02 ", "3000", true, "0.00"),
        ("OL P03 ", "-7500", false, "0.00"),
        ("OL P04 ", "0", true, "0.00"),
        ("OL P05 ", "1500", true, "0.00"),
        ("OL P06 ", "-800", true, "0.00"),
        ("OL P07 ", "0", true, "-33333.00"),
        ("OL P08 ", "-20000", true, "-20000.00"),
        ("OL P09 ", "0", true, "0.00"),
        ("OL P10 ", "0", true, "0.00"),
        ("OL P11 ", "0", true, "0.00"),
    ];
    for (prefix, amount, bill_wise, opening) in expected {
        let row = residual(&result, prefix);
        assert!(
            row.amount
                .numeric_eq(&bridge_tally_primitives::ExactDecimal::parse(amount).unwrap()),
            "{prefix}: residual {} is not {amount}",
            row.amount.as_str()
        );
        assert_eq!(row.bill_wise_on, bill_wise, "{prefix}: bill-wise flag");
        assert!(
            row.opening_balance
                .numeric_eq(&bridge_tally_primitives::ExactDecimal::parse(opening).unwrap()),
            "{prefix}: opening {} is not {opening}",
            row.opening_balance.as_str()
        );
    }
}

#[test]
fn only_the_ledger_that_keeps_no_bills_is_bill_wise_off_among_the_residual_parties() {
    let result = computed();
    let off = result
        .residuals
        .iter()
        .filter(|row| !row.amount.is_zero() && !row.bill_wise_on)
        .map(|row| row.party.as_str())
        .collect::<Vec<_>>();
    assert_eq!(off, ["OL P03 Not Billwise Debtor"]);
}
