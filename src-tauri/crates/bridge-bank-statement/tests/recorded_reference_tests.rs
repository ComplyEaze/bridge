//! A reference read back from a journal is admitted only in a shape
//! `Bank::transfer_reference` can produce. All values are synthetic.

use bridge_bank_statement::bank::BankReference;

fn admitted(text: &str) -> Option<String> {
    BankReference::from_recorded(text).map(|found| found.to_string())
}

#[test]
fn twelve_ascii_digits_and_a_utr_are_admitted() {
    assert_eq!(admitted("600000000001"), Some("600000000001".to_string()));
    assert_eq!(
        admitted("SBIN526000000001"),
        Some("SBIN526000000001".to_string())
    );
}

#[test]
fn a_shape_no_row_produces_is_refused() {
    for refused in [
        "",
        "  ",
        // a UPI or IMPS value is exactly twelve digits
        "60000000001",
        "6000000000011",
        // twelve characters that are not all digits
        "60000000000a",
        "600000-00001",
        // all-zero placeholders, at both admitted shapes
        "000000000000",
        "0000000000000000",
        // a UTR is upper case, with no whitespace or punctuation
        "sbin526000000001",
        " SBIN526000000001",
        "SBIN 526000000001",
        "SBIN52600000000-1",
        // and carries at least four digits
        "ABCDEFGHIJ",
        "ABCDEFGH123",
        // twelve digits that are not ASCII
        "٦٠٠٠٠٠٠٠٠٠٠٠٠",
        // a placeholder, not a reference
        "000000000000",
    ] {
        assert_eq!(admitted(refused), None, "{refused:?}");
    }
}
