//! Lab only, never merged: renders row 683-TW's requests with master's own builders.
//!
//! Run (one build slot): `LAB_683_TW_OUT=<abs path of lab/683-tw> cargo test -p bridge --lib
//! lab_683_tw -- --ignored --exact agent::lab_683_tw_render_tests::lab_683_tw_renders_the_row_requests`.
//! Each request is written as sent: UTF-16LE with a BOM. Three renders are first checked byte
//! for byte against requests this book has already been sent, so the encoding and the builders
//! are shown equal: the 1 to 30 April currency Trial Balance (row 683-TB's `tb1`, equal to the
//! build's request of 29 Oct), and the book-scoped and ledger-scoped movement windows of
//! 1 April (row 683-LM's `mv` and `mvl`). Each new request differs from its checked one in its
//! dates alone, and the ledger-scoped ones also in the ledger's name.

use super::read_profiles::{
    render_agent_ledger_vouchers, render_agent_movement_vouchers, LedgerVoucherFields,
};
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::ledger_scope::LedgerScopeName;
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

/// The period `trial_balance` builds for (`from`, `to`) on a licensed gateway, which is
/// `ModeAgnostic` (`tally/runtime_trial_balance.rs:391`).
fn tb(from: &str, to: &str) -> Vec<u8> {
    let period =
        NativeLedgerSnapshotPeriod::new(DateBoundaryProfile::ModeAgnostic, day(from), day(to))
            .unwrap();
    as_sent(&render_native_trial_balance_request_with_currency(
        BOOK, &period,
    ))
}

fn mv(from: &str, to: &str) -> Vec<u8> {
    as_sent(&render_agent_movement_vouchers(BOOK, &day(from), &day(to)).unwrap())
}

fn mvl(ledger: &str, from: &str, to: &str) -> Vec<u8> {
    as_sent(
        &render_agent_ledger_vouchers(
            BOOK,
            &LedgerScopeName::new(ledger).unwrap(),
            LedgerVoucherFields::Movement,
            &day(from),
            &day(to),
        )
        .unwrap(),
    )
}

#[test]
#[ignore = "lab only: writes row 683-TW's requests"]
fn lab_683_tw_renders_the_row_requests() {
    let out = std::path::PathBuf::from(std::env::var("LAB_683_TW_OUT").unwrap());
    let lm = out.join("../683-lm");

    let tb1 = tb("20250401", "20250430");
    assert_eq!(
        tb1,
        std::fs::read(out.join("T-tb1-20250401-20250430.xml")).unwrap(),
        "the 1 to 30 April render differs from row 683-TB's tb1"
    );
    assert_eq!(
        mv("20250401", "20250401"),
        std::fs::read(lm.join("V-mv-20250401.xml")).unwrap(),
        "the book-scoped 1 April render differs from row 683-LM's mv"
    );
    assert_eq!(
        mvl("BRIDGE USD Customer", "20250401", "20250401"),
        std::fs::read(lm.join("V-mvl-usd-customer-20250401.xml")).unwrap(),
        "the ledger-scoped 1 April render differs from row 683-LM's mvl"
    );

    for (name, bytes) in [
        ("T-tb3-20250401-20250401.xml", tb("20250401", "20250401")),
        ("T-tb4-20250401-20260331.xml", tb("20250401", "20260331")),
        ("V-mvy-20250401-20260331.xml", mv("20250401", "20260331")),
        (
            "V-mvly-inr-customer-20250401-20260331.xml",
            mvl("BRIDGE INR Customer", "20250401", "20260331"),
        ),
        (
            "V-mvly-usd-customer-20250401-20260331.xml",
            mvl("BRIDGE USD Customer", "20250401", "20260331"),
        ),
    ] {
        std::fs::write(out.join(name), &bytes).unwrap();
        println!("{name} {}", bytes.len());
    }
}
