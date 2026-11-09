//! Lab only, never merged: renders row 683-TB's two Trial Balance requests with master's own builder.
//!
//! Run (one build slot): `LAB_683_TB_OUT=<abs path of lab/683-tb> cargo test -p bridge-tally-protocol
//! --test lab_683_tb_render -- --ignored --exact lab_683_tb_renders_the_row_requests`.
//! Each request is written as sent: UTF-16LE with a BOM. The 1 to 30 April render is first checked
//! byte for byte against the request the build sent on this book on 29 Oct (row 2071-forex-rerun,
//! `requests/tb-currency.utf16le.xml`, sha256 d3ca2b38...), kept in the folder as
//! `tb-build-request-20250401-20250430.bin`, so the encoding and the builder are shown equal.

use bridge_tally_primitives::TallyDate;
use bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriod;
use bridge_tally_protocol::native_trial_balance::render_native_trial_balance_request_with_currency;
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;

/// The book's name, as the build sends it (`identity.display_name()`).
const BOOK: &str = "BRIDGE FOREXGROUP LAB";

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

/// The period `trial_balance` builds for (`from`, `to`) on a licensed Silver gateway, which is
/// `ModeAgnostic` (`tally/runtime_trial_balance.rs:391`).
fn tb(from: &str, to: &str) -> Vec<u8> {
    let period =
        NativeLedgerSnapshotPeriod::new(DateBoundaryProfile::ModeAgnostic, day(from), day(to))
            .unwrap();
    as_sent(&render_native_trial_balance_request_with_currency(
        BOOK, &period,
    ))
}

#[test]
#[ignore = "lab only: writes row 683-TB's requests"]
fn lab_683_tb_renders_the_row_requests() {
    let out = std::path::PathBuf::from(std::env::var("LAB_683_TB_OUT").unwrap());

    let tb1 = tb("20250401", "20250430");
    let sent = std::fs::read(out.join("tb-build-request-20250401-20250430.bin")).unwrap();
    assert_eq!(
        tb1, sent,
        "the 1 to 30 April render differs from the build's request of 29 Oct"
    );
    let tb2 = tb("20250402", "20250430");

    for (name, bytes) in [
        ("T-tb1-20250401-20250430.xml", tb1),
        ("T-tb2-20250402-20250430.xml", tb2),
    ] {
        std::fs::write(out.join(name), &bytes).unwrap();
        println!("{name} {}", bytes.len());
    }
}
