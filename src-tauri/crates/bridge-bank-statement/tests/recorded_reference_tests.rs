//! A reference read back from a journal is admitted only in a shape
//! `Bank::transfer_reference` can produce. All values are synthetic.

mod common;

use bridge_bank_statement::bank::BankReference;
use common::*;

fn admitted(text: &str) -> Option<String> {
    BankReference::from_recorded(text).map(|found| found.to_string())
}

#[test]
fn twelve_ascii_digits_and_a_utr_are_admitted() {
    assert_eq!(admitted(&p6(1, 12)), Some(p6(1, 12)));
    assert_eq!(
        admitted(&format!("ZZZZ{}", pre("526", 1, 12))),
        Some(format!("ZZZZ{}", pre("526", 1, 12)))
    );
}

#[test]
fn a_shape_no_row_produces_is_refused() {
    for refused in [
        "",
        "  ",
        // a UPI or IMPS value is exactly twelve digits
        &p6(1, 11),
        &p6(11, 13),
        // twelve characters that are not all digits
        &format!("{}a", p6(0, 11)),
        "600000-00001",
        // all-zero placeholders, at both admitted shapes
        &padn(0, 12),
        &padn(0, 16),
        // a UTR is upper case, with no whitespace or punctuation
        &format!("zzzz{}", pre("526", 1, 12)),
        &format!(" ZZZZ{}", pre("526", 1, 12)),
        &format!("ZZZZ {}", pre("526", 1, 12)),
        &format!("ZZZZ{}-1", pre("526", 0, 11)),
        // and carries at least four digits
        "ABCDEFGHIJ",
        "ABCDEFGH123",
        // twelve digits that are not ASCII
        &arabic_indic(&p6(0, 13)),
        // a placeholder, not a reference
        &padn(0, 12),
    ] {
        assert_eq!(admitted(refused), None, "{refused:?}");
    }
}
