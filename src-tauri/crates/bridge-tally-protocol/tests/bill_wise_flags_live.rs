// SPDX-License-Identifier: Apache-2.0

//! The bill-wise flag reads of #1234 slice 1, on captures of a live Tally.
//!
//! The two fixtures are raw answers from the synthetic company BRIDGE
//! OUTSTANDINGS LAB (TallyPrime Silver 7.1, 6 Oct 2026) to requests this crate
//! renders: the unfiltered outstandings ledger snapshot over one day (the
//! company's books_from), and the same snapshot filtered to the ledgers under
//! two parents. Their provenance files give both hashes.
//!
//! The expected flags are the book's seeding, worked out by hand (the table in
//! `native_outstandings_lab_live.rs`: every party but P03 keeps bills; the table
//! lists parties only, and the flag of the other six ledgers is No by their
//! groups), NOT read back from this code. The hashes in a fixture's provenance
//! file pin it only against an accidental edit: the live answers themselves are
//! a lab record, not something a test can re-derive.

use bridge_tally_primitives::TallyDate;
use bridge_tally_protocol::native_outstandings::{
    parse_native_ledger_bill_wise_flags_for_company, render_native_ledger_snapshot_request,
    render_native_ledger_snapshot_request_for_parents, NativeLedgerBillWiseFlag,
    NativeLedgerSnapshotPeriod,
};
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;
use bridge_tally_protocol::parent_partition::{
    ParentObservation, ParentPartition, PartitionLimits,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/agent");
const COMPANY: &str = "BRIDGE OUTSTANDINGS LAB";
const COMPANY_GUID: &str = "49f1fbda-ee59-4a4b-aacf-b45fe32402d7";
const WHOLE: &str = "bill-wise-snapshot-oneday-outstandings-lab";
const PARTS: &str = "bill-wise-snapshot-parents-oneday-outstandings-lab";

/// Every party the book was seeded with keeps bills except P03 (a ledger that
/// keeps none); the six non-party ledgers (cash, bank, income, purchases, sales,
/// the profit and loss ledger) do not.
const BILL_WISE: [&str; 10] = [
    "OL P01 Named Bills Debtor",
    "OL P02 On Account Debtor",
    "OL P04 Advance Debtor",
    "OL P05 Credit Note Debtor",
    "OL P06 Debit Note Creditor",
    "OL P07 Referenced Opening Debtor",
    "OL P08 Unreferenced Opening Debtor",
    "OL P09 Reuse Debtor A",
    "OL P10 Reuse Debtor B",
    "OL P11 Journal Debtor",
];

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// The fixture's text, after checking its bytes against its provenance file.
fn fixture(name: &str) -> (String, serde_json::Value) {
    let bytes = std::fs::read(format!("{FIXTURES}/{name}.utf16le.xml"))
        .unwrap_or_else(|error| panic!("fixture {name} unreadable: {error}"));
    let provenance: serde_json::Value = serde_json::from_slice(
        &std::fs::read(format!("{FIXTURES}/{name}.json"))
            .unwrap_or_else(|error| panic!("provenance {name} unreadable: {error}")),
    )
    .expect("provenance is JSON");
    assert_eq!(provenance["fixture_sha256"], sha256_hex(&bytes), "{name}");
    assert_eq!(provenance["source_response_sha256"], sha256_hex(&bytes));
    assert_eq!(provenance["fixture_bytes"], bytes.len());
    assert_eq!(bytes.len() % 2, 0, "a UTF-16LE fixture has an even length");
    let units = bytes
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    (
        String::from_utf16(&units).expect("fixture is UTF-16LE"),
        provenance,
    )
}

fn flags(name: &str) -> Vec<NativeLedgerBillWiseFlag> {
    parse_native_ledger_bill_wise_flags_for_company(&fixture(name).0, COMPANY_GUID)
        .unwrap_or_else(|error| panic!("{name} parses: {error:?}"))
}

fn by_name(rows: &[NativeLedgerBillWiseFlag]) -> BTreeMap<&str, (Option<&str>, bool)> {
    rows.iter()
        .map(|row| (row.name.as_str(), (row.parent.as_deref(), row.bill_wise_on)))
        .collect()
}

fn one_day() -> NativeLedgerSnapshotPeriod {
    let books_from = TallyDate::parse("20250401").unwrap();
    NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        books_from.clone(),
        books_from,
    )
    .unwrap()
}

fn wire_sha256(xml: &str) -> String {
    sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
        xml,
    ))
}

#[test]
fn the_one_day_snapshot_answers_every_ledger_with_the_seeded_flag() {
    let rows = flags(WHOLE);
    assert_eq!(rows.len(), 17, "every ledger of the book");
    let seen = by_name(&rows);
    assert_eq!(seen.len(), 17, "no name repeats");
    for (name, (_, on)) in &seen {
        assert_eq!(*on, BILL_WISE.contains(name), "{name}");
    }
    assert!(!seen["OL P03 Not Billwise Debtor"].1);
    assert!(!seen["OL Bank"].1);
}

#[test]
fn the_parent_filtered_snapshot_is_exactly_the_ledgers_under_those_parents() {
    let whole = flags(WHOLE);
    let part = flags(PARTS);
    let under = |row: &&NativeLedgerBillWiseFlag| {
        matches!(
            row.parent.as_deref(),
            Some("Sundry Debtors" | "Sundry Creditors")
        )
    };
    let expected = whole.iter().filter(under).cloned().collect::<Vec<_>>();
    assert_eq!(expected.len(), 11);
    assert_eq!(by_name(&part), by_name(&expected));
}

#[test]
fn the_renderers_still_produce_the_requests_that_were_sent() {
    let (_, whole) = fixture(WHOLE);
    let xml = render_native_ledger_snapshot_request(COMPANY, &one_day());
    assert_eq!(wire_sha256(&xml), whole["source_request_sha256"]);
    assert_eq!(
        bridge_tally_protocol::encode_tally_xml_request_utf16le(&xml).len(),
        whole["source_request_bytes"]
    );
    // The part a plan makes over the ledgers under those parents, from the whole
    // answer's own names and parents. The limits are a copy of the build's
    // (`parent_partition_limits`), so a change there does not move this test.
    let rows = flags(WHOLE);
    let under = rows
        .iter()
        .filter(|row| {
            matches!(
                row.parent.as_deref(),
                Some("Sundry Debtors" | "Sundry Creditors")
            )
        })
        .map(|row| {
            (
                row.name.as_str(),
                row.name.as_str(),
                ParentObservation::Named(row.parent.as_deref().unwrap()),
            )
        });
    let plan = ParentPartition::plan(
        under,
        PartitionLimits {
            max_ledgers_per_part: 16_000_000 / 3_750,
            max_parents_per_part: 200,
            max_parts: 12,
            max_complement_formula_bytes: 262_144,
        },
    )
    .unwrap();
    assert_eq!(plan.parts().len(), 1);
    let (_, parts) = fixture(PARTS);
    let xml =
        render_native_ledger_snapshot_request_for_parents(COMPANY, &one_day(), &plan.parts()[0]);
    assert_eq!(wire_sha256(&xml), parts["source_request_sha256"]);
    assert_eq!(plan.parts()[0].check_row_count(flags(PARTS).len()), Ok(()));
}
