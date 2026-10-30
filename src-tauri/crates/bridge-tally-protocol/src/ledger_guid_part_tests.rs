use super::*;

const COMPANY: &str = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";

fn guid(suffix: &str) -> LedgerGuid {
    LedgerGuid::parse(&format!("{COMPANY}-{suffix}")).unwrap()
}

#[test]
fn a_ledger_guid_admits_hex_digits_and_hyphens_only() {
    assert!(LedgerGuid::parse(&format!("{COMPANY}-000000d0")).is_ok());
    assert_eq!(LedgerGuid::parse(""), Err(LedgerGuidError::Empty));
    for refused in [
        "a\"b",
        "a b",
        "a&b",
        "a\nb",
        "a OR $Name = \"x\"",
        "abcg",
        "a\u{0661}b",
    ] {
        assert_eq!(
            LedgerGuid::parse(refused),
            Err(LedgerGuidError::Character),
            "{refused:?}"
        );
    }
    assert_eq!(
        LedgerGuid::parse(&"a".repeat(LEDGER_GUID_MAX_CHARS + 1)),
        Err(LedgerGuidError::TooLong)
    );
    assert!(LedgerGuid::parse(&"a".repeat(LEDGER_GUID_MAX_CHARS)).is_ok());
}

#[test]
fn the_formula_names_each_ledger_once_in_guid_order() {
    let part = LedgerGuidPart::new([
        guid("000000d8"),
        guid("000000d0"),
        guid("000000d7"),
        guid("000000d3"),
        guid("000000d5"),
        guid("000000d0"),
    ])
    .unwrap();
    let formula = part.formula();
    assert_eq!(formula.name, "BridgeInvoiceLedgers");
    // The lab sheet's `g1` filter, byte for byte.
    assert_eq!(
        formula.text,
        format!(
            "$GUID = \"{COMPANY}-000000d0\" OR $GUID = \"{COMPANY}-000000d3\" OR $GUID = \"{COMPANY}-000000d5\" OR $GUID = \"{COMPANY}-000000d7\" OR $GUID = \"{COMPANY}-000000d8\""
        )
    );
    assert_eq!(part.guids().len(), 5);
}

#[test]
fn a_part_names_at_least_one_and_at_most_the_measured_count() {
    assert_eq!(
        LedgerGuidPart::new(Vec::new()),
        Err(LedgerGuidPartError::Empty)
    );
    let many = |count: usize| (0..count).map(|index| guid(&format!("{index:08x}")));
    assert!(LedgerGuidPart::new(many(LEDGER_GUID_PART_MAX_GUIDS)).is_ok());
    assert_eq!(
        LedgerGuidPart::new(many(LEDGER_GUID_PART_MAX_GUIDS + 1)),
        Err(LedgerGuidPartError::TooMany)
    );
}
