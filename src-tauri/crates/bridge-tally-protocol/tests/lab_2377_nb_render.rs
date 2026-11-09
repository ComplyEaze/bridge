//! Lab only, never merged: renders row 2377-NB's four requests with master's own builders.
//!
//! Run (one build slot): `LAB_2377_NB_OUT=<abs path of lab/2377-nb> cargo test -p bridge-tally-protocol
//! --test lab_2377_nb_render -- --ignored --exact lab_2377_nb_renders_the_row_requests`.
//! Each request is written as the build sends it: UTF-16LE with a BOM. First the snapshot builder is
//! rendered for the request the build sent on another lab book on 29 Oct (row 2071-forex-rerun,
//! `requests/ledger-snapshot.utf16le.xml`, sha256 4df96fa3...), kept in the folder as
//! `snapshot-build-request-forex-20250401-20250430.bin`, and checked byte for byte, so the encoding
//! and the builder are shown equal to what the build sends.

use bridge_tally_primitives::TallyDate;
use bridge_tally_protocol::ledger_scope::LedgerScopeName;
use bridge_tally_protocol::native_outstandings::{
    render_ledger_outstandings_request, render_native_ledger_snapshot_request,
    NativeLedgerSnapshotPeriod,
};
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;

/// The book's name, as the build sends it (`identity.display_name()`).
const BOOK: &str = "BRIDGE PILOT LAB";
/// The book's first day (its BOOKSFROM, `pilot-lab-register-book-extent.utf16le.xml`).
const BOOKS_FROM: &str = "20260401";
/// The party: the book's one bill-wise ledger.
const PARTY: &str = "BRIDGE Supplier RJ";

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

/// The snapshot `fetch_party_bills` sends (`tally/runtime_party_bills.rs:81-85`): the books' first
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
#[ignore = "lab only: writes row 2377-NB's requests"]
fn lab_2377_nb_renders_the_row_requests() {
    let out = std::path::PathBuf::from(std::env::var("LAB_2377_NB_OUT").unwrap());

    let control = snapshot("BRIDGE FOREXGROUP LAB", "20250401", "20250430");
    let sent = std::fs::read(out.join("snapshot-build-request-forex-20250401-20250430.bin")).unwrap();
    assert_eq!(
        control, sent,
        "the snapshot builder's render differs from the build's request of 29 Oct"
    );

    for (name, bytes) in [
        ("S-snap-20260401-20260804.xml", snapshot(BOOK, BOOKS_FROM, "20260804")),
        ("L-lo-rj-20260804.xml", outstandings("20260804")),
        ("S-snap-20260401-20260731.xml", snapshot(BOOK, BOOKS_FROM, "20260731")),
        ("L-lo-rj-20260731.xml", outstandings("20260731")),
    ] {
        std::fs::write(out.join(name), &bytes).unwrap();
        println!("{name} {}", bytes.len());
    }
}
