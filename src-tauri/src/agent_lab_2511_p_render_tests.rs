//! Lab only, never merged: renders row 2511-P's read requests with master's own builders.
//!
//! Run (one build slot): `LAB_2511_P_OUT=<abs path of lab/2511-p> cargo test -p bridge --lib
//! lab_2511_p -- --ignored --exact agent::lab_2511_p_render_tests::lab_2511_p_renders_the_row_requests`.
//! Each request is written as sent: UTF-16LE with a BOM. The book-scoped year window is first
//! checked byte for byte against `V-mvy-20250401-20260331.xml`, which rows 683-TW, 683-LD and
//! 683-VD sent on this book, so the encoding and the builder are shown equal. The three day
//! windows differ from it in their dates alone.
//!
//! `C-cat2.xml` is the import's ledger catalogue as master sends it (V2,
//! `render_import_ledger_catalog_request`, reached from `agent_import.rs` through
//! `import_ledger_catalogue_read`). `C-cat4.xml` is that render with one line added after the
//! `IsBillWiseOn` line: the same indentation, `<NATIVEMETHOD>CurrencyName</NATIVEMETHOD>`. It is a
//! new shape. The builder it is meant to become is a `StandardLedgerCatalogV4` profile, unbuilt.

use super::read_profiles::render_agent_movement_vouchers;
use bridge_tally_core::TallyDate;

/// The book's name, as the build sends it (`identity.display_name()`).
const BOOK: &str = "BRIDGE FOREXGROUP LAB";

/// V2's own line for the bill-wise flag (`xml_read_profiles.rs`, the V2 render).
const BILL_WISE_LINE: &str = "\n                        <NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>";
/// The one line this row adds after it.
const CURRENCY_LINE: &str = "\n                        <NATIVEMETHOD>CurrencyName</NATIVEMETHOD>";

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

fn mv(from: &str, to: &str) -> Vec<u8> {
    as_sent(&render_agent_movement_vouchers(BOOK, &day(from), &day(to)).unwrap())
}

#[test]
#[ignore = "lab only: writes row 2511-P's requests"]
fn lab_2511_p_renders_the_row_requests() {
    let out = std::path::PathBuf::from(std::env::var("LAB_2511_P_OUT").unwrap());

    assert_eq!(
        mv("20250401", "20260331"),
        std::fs::read(out.join("V-mvy-20250401-20260331.xml")).unwrap(),
        "the book-scoped year render differs from the window rows 683-TW, 683-LD and 683-VD sent"
    );

    let cat2 =
        crate::tally::standard_ledger_catalog::render_import_ledger_catalog_request(BOOK).unwrap();
    assert_eq!(
        cat2.matches(BILL_WISE_LINE).count(),
        1,
        "V2 must hold the bill-wise line exactly once"
    );
    assert!(
        !cat2.contains("CurrencyName"),
        "V2 already fetches CurrencyName"
    );
    let cat4 = cat2.replacen(
        BILL_WISE_LINE,
        &format!("{BILL_WISE_LINE}{CURRENCY_LINE}"),
        1,
    );
    assert_eq!(cat4.len(), cat2.len() + CURRENCY_LINE.len());

    for (name, bytes) in [
        ("V-day-20250402.xml", mv("20250402", "20250402")),
        ("V-day-20250403.xml", mv("20250403", "20250403")),
        ("V-day-20250404.xml", mv("20250404", "20250404")),
        ("C-cat2.xml", as_sent(&cat2)),
        ("C-cat4.xml", as_sent(&cat4)),
    ] {
        std::fs::write(out.join(name), &bytes).unwrap();
        println!("{name} {}", bytes.len());
    }
}
