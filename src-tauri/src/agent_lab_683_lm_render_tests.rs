//! Lab only, never merged: renders row 683-LM's requests with master's own builders.
//!
//! Run (one build slot): `LAB_683_LM_OUT=<abs path of lab/683-lm> cargo test -p bridge --lib
//! lab_683_lm -- --ignored --exact agent::lab_683_lm_render_tests::lab_683_lm_renders_the_row_requests`.
//! Each request is written as sent: UTF-16LE with a BOM. Two renders are first checked byte
//! for byte against bytes already sent, so the encoding and the builders are shown equal:
//! the currency read against the build's own request on this book (row 683-register,
//! exchange 58, body sha256 df262192...), and the ledger-scoped movement form against the
//! fixture `movement-dn-party-month` that
//! `the_movement_builder_renders_the_request_the_lab_sent_byte_for_byte` pins.

use super::read_profiles::{
    render_agent_ledger_vouchers, render_agent_movement_vouchers, LedgerVoucherFields,
};
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::ledger_scope::LedgerScopeName;
use bridge_tally_protocol::native_outstandings::{
    render_company_currency_request, render_native_ledger_export_request,
    NativeLedgerExportPeriod,
};
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;

/// The book's name, as the build sent it in exchange 58 (`identity.display_name()`).
const BOOK: &str = "BRIDGE FOREXGROUP LAB";
/// The book's extent, as `forex-register-book-extent` read it: BOOKSFROM and LASTVOUCHERDATE.
const BOOKS_FROM: &str = "20250401";
const LAST_VOUCHER: &str = "20250401";

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

/// `ledger_opening_period` (`tally/runtime.rs:2078`) for a requested `from`: `to` is the later
/// of the last voucher date and `from`. A licensed Silver gateway is `ModeAgnostic`
/// (`runtime.rs:1840`).
fn export_period(from: &str) -> NativeLedgerExportPeriod {
    assert!(from >= BOOKS_FROM);
    let to = if from > LAST_VOUCHER { from } else { LAST_VOUCHER };
    NativeLedgerExportPeriod::new(DateBoundaryProfile::ModeAgnostic, day(from), day(to)).unwrap()
}

#[test]
#[ignore = "lab only: writes row 683-LM's requests"]
fn lab_683_lm_renders_the_row_requests() {
    let out = std::path::PathBuf::from(std::env::var("LAB_683_LM_OUT").unwrap());

    let cur = as_sent(&render_company_currency_request(BOOK));
    let built = std::fs::read(out.join("cur-build-body.bin")).unwrap();
    assert_eq!(cur, built, "the currency read differs from the build's exchange 58");

    let pinned = as_sent(
        &render_agent_ledger_vouchers(
            "BRIDGE CORPUS DENSE",
            &LedgerScopeName::new("DN Party 001").unwrap(),
            LedgerVoucherFields::Movement,
            &day("20250501"),
            &day("20250531"),
        )
        .unwrap(),
    );
    assert_eq!(
        pinned,
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/ledger-scoped-vouchers/movement-dn-party-month.request.utf16le.xml").to_vec(),
        "the ledger-scoped movement form differs from its pinned fixture"
    );

    let mv = render_agent_movement_vouchers(BOOK, &day("20250401"), &day("20250401")).unwrap();
    let mvl = render_agent_ledger_vouchers(
        BOOK,
        &LedgerScopeName::new("BRIDGE USD Customer").unwrap(),
        LedgerVoucherFields::Movement,
        &day("20250401"),
        &day("20250401"),
    )
    .unwrap();
    let lx1 = render_native_ledger_export_request(BOOK, &export_period("20250401"));
    let lx2 = render_native_ledger_export_request(BOOK, &export_period("20250402"));

    for (name, bytes) in [
        ("C-currency.xml", cur),
        ("V-mv-20250401.xml", as_sent(&mv)),
        ("V-mvl-usd-customer-20250401.xml", as_sent(&mvl)),
        ("X-lx1-20250401.xml", as_sent(&lx1)),
        ("X-lx2-20250402.xml", as_sent(&lx2)),
    ] {
        std::fs::write(out.join(name), &bytes).unwrap();
        println!("{name} {}", bytes.len());
    }
}
