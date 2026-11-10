//! Lab only, never merged: renders row 2346-TD's bills reads with master's own builders, the party
//! admitted through `LedgerScopeName` as `fetch_party_bills` takes it (row 2377-NB's path).
//!
//! Run (one build slot): `LAB_2346_TD_OUT=<abs path of lab/2346-td> cargo test -p bridge-tally-protocol
//! --test lab_2346_td_render -- --ignored --exact lab_2346_td_renders_the_row_requests --nocapture`.
//! Each request is written as the build sends it: UTF-16LE with a BOM. Before writing, it checks:
//! - the snapshot builder against the request the build sent on another lab book on 29 Oct
//!   (`snapshot-build-request-forex-20250401-20250430.bin`, row 2377-NB's control), byte for byte;
//! - the snapshot builder at row 2346-PB's to-date against PB's `S-snap-20260401-20260801.xml`, byte
//!   for byte;
//! - the report builder at PB's to-date against PB's `L-lo-bc-20260801.xml` (kept in `pb/`) with
//!   PB's customer name turned into this row's, byte for byte;
//! - that `LedgerScopeName` refuses PB's name and admits this row's.
//!
//! It prints whether `NativeLedgerSnapshotPeriod` admits the books' first day to the period's last day
//! under each `DateBoundaryProfile`.

use bridge_tally_primitives::TallyDate;
use bridge_tally_protocol::ledger_scope::{LedgerScopeName, LedgerScopeNameError};
use bridge_tally_protocol::native_outstandings::{
    render_ledger_outstandings_request, render_native_ledger_snapshot_request,
    NativeLedgerSnapshotPeriod,
};
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;

/// The book's name, as the build sends it (`identity.display_name()`).
const BOOK: &str = "BRIDGE PILOT LAB";
/// The book's first day (its BOOKSFROM, `pilot-lab-register-book-extent.utf16le.xml`).
const BOOKS_FROM: &str = "20260401";
/// Row 2346-PB's to-date, the notes' own date.
const PB_TO: &str = "20260801";
/// The last day of the book's current period, 1-Apr-26 to 31-Mar-27 (check 4).
const PERIOD_END: &str = "20270331";
/// Row 2346-PB's customer, which `LedgerScopeName` refuses.
const PB_PARTY: &str = "BRIDGE Walk-in BC";
/// This row's customer.
const PARTY: &str = "BRIDGE Walkin BC";

fn day(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

/// As the build sends a request: a BOM, then UTF-16LE.
fn as_sent(xml: &str) -> Vec<u8> {
    let mut bytes = vec![0xff, 0xfe];
    for unit in xml.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes
}

/// The snapshot `fetch_party_bills` sends (`tally/runtime_party_bills.rs:80-85`): the books' first
/// day to `to`, on a licensed gateway, which is `ModeAgnostic`.
fn snapshot(book: &str, from: &str, to: &str) -> Vec<u8> {
    let period =
        NativeLedgerSnapshotPeriod::new(DateBoundaryProfile::ModeAgnostic, day(from), day(to))
            .unwrap();
    as_sent(&render_native_ledger_snapshot_request(book, &period))
}

/// The party's report `fetch_party_bills` sends (`tally/runtime_party_bills.rs:97`).
fn outstandings(to: &str) -> Vec<u8> {
    let party = LedgerScopeName::new(PARTY).unwrap();
    as_sent(&render_ledger_outstandings_request(BOOK, &party, &day(to)))
}

#[test]
#[ignore = "lab only: writes row 2346-TD's requests"]
fn lab_2346_td_renders_the_row_requests() {
    let out = std::path::PathBuf::from(std::env::var("LAB_2346_TD_OUT").unwrap());
    let read = |name: &str| std::fs::read(out.join(name)).unwrap();

    assert_eq!(
        snapshot("BRIDGE FOREXGROUP LAB", "20250401", "20250430"),
        read("snapshot-build-request-forex-20250401-20250430.bin"),
        "the snapshot builder's render differs from the build's request of 29 Oct"
    );

    assert_eq!(
        LedgerScopeName::new(PB_PARTY),
        Err(LedgerScopeNameError::Character)
    );
    assert_eq!(LedgerScopeName::new(PARTY).unwrap().as_str(), PARTY);

    assert_eq!(
        snapshot(BOOK, BOOKS_FROM, PB_TO),
        read("S-snap-20260401-20260801.xml"),
        "the snapshot builder's render differs from row 2346-PB's request"
    );
    let pb_report = read("pb/L-lo-bc-20260801.xml");
    let pb_report = String::from_utf16(
        &pb_report[2..]
            .chunks(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    assert_eq!(pb_report.matches(PB_PARTY).count(), 1);
    assert_eq!(
        outstandings(PB_TO),
        as_sent(&pb_report.replace(PB_PARTY, PARTY)),
        "the report builder's render differs from row 2346-PB's request, name mapped"
    );

    for profile in [
        DateBoundaryProfile::ModeAgnostic,
        DateBoundaryProfile::EducationRestricted,
    ] {
        let label = match profile {
            DateBoundaryProfile::ModeAgnostic => "ModeAgnostic",
            DateBoundaryProfile::EducationRestricted => "EducationRestricted",
        };
        let admitted =
            NativeLedgerSnapshotPeriod::new(profile, day(BOOKS_FROM), day(PERIOD_END)).is_ok();
        println!(
            "NativeLedgerSnapshotPeriod {BOOKS_FROM} to {PERIOD_END}, {label}: admitted {admitted}"
        );
    }

    for (name, bytes) in [
        ("L-lo-wk-20260801.xml", outstandings(PB_TO)),
        (
            "S-snap-20260401-20270331.xml",
            snapshot(BOOK, BOOKS_FROM, PERIOD_END),
        ),
        ("L-lo-wk-20270331.xml", outstandings(PERIOD_END)),
    ] {
        std::fs::write(out.join(name), &bytes).unwrap();
        println!("{name} {}", bytes.len());
    }
}
