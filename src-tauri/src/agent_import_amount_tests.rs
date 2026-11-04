use super::*;
use serde_json::json;

/// The rule table `valid_2dp_amount` was held to (#1872), now on the type that carries it.
#[test]
fn an_import_amount_is_a_positive_two_decimal_number_in_at_most_256_bytes() {
    for (text, admitted) in [
        ("0.00".to_string(), false),
        ("000.00".to_string(), false),
        ("0.01".to_string(), true),
        ("000.01".to_string(), true),
        (format!("{}.99", "9".repeat(253)), true),
        (format!("{}.99", "9".repeat(254)), false),
    ] {
        assert_eq!(ImportAmount::parse(&text).is_ok(), admitted, "{text}");
    }
}

/// A signed amount is not an import amount: the sign is the entry's side. These are the
/// refusals the batch tests used to reach by putting `-13.00` in an entry.
#[test]
fn a_signed_or_misshapen_amount_is_not_an_import_amount() {
    for text in [
        "", "-13.00", "+13.00", "13", "13.", ".50", "13.5", "13.505", "1,300.00", " 13.00",
        "13.00 ", "1e2.00", "٣.٠٠", "1..00", "-0.00",
    ] {
        assert_eq!(
            ImportAmount::parse(text),
            Err(NotAnImportAmount),
            "{text:?}"
        );
    }
}

/// The text is kept as received and the canonical form is worked out from it, once.
#[test]
fn the_received_text_and_the_canonical_form_are_both_held() {
    for (text, canonical) in [
        ("0012.50", "12.5"),
        ("12.50", "12.5"),
        ("1.00", "1"),
        ("0.01", "0.01"),
        ("000.01", "0.01"),
        ("10.10", "10.1"),
    ] {
        let amount = ImportAmount::parse(text).unwrap();
        assert_eq!(amount.as_str(), text);
        assert_eq!(amount.canonical(), canonical);
        assert_eq!(amount.decimal().as_str(), text);
        assert_eq!(amount.to_string(), text);
    }
}

#[test]
fn a_debit_is_signed_in_the_form_each_caller_uses() {
    let amount = ImportAmount::parse("0012.50").unwrap();
    assert_eq!(amount.signed_lexeme(&EntrySide::Dr), "-0012.50");
    assert_eq!(amount.signed_lexeme(&EntrySide::Cr), "0012.50");
    assert_eq!(amount.signed_canonical(&EntrySide::Dr), "-12.5");
    assert_eq!(amount.signed_canonical(&EntrySide::Cr), "12.5");
}

/// Written as received and read by `parse`: a journal line keeps `0012.50`, and a line whose
/// amount is not an import amount does not deserialise.
#[test]
fn an_amount_is_written_as_received_and_read_by_parse() {
    let amount = ImportAmount::parse("0012.50").unwrap();
    assert_eq!(serde_json::to_value(&amount).unwrap(), json!("0012.50"));
    let read: ImportAmount = serde_json::from_value(json!("0012.50")).unwrap();
    assert_eq!(read, amount);
    for bad in [
        json!("-13.00"),
        json!("0.00"),
        json!("13"),
        json!(13.0),
        json!(null),
    ] {
        assert!(
            serde_json::from_value::<ImportAmount>(bad.clone()).is_err(),
            "{bad}"
        );
    }
}

/// The stage trait: the caller's text is parsed, an admitted amount passes through.
#[test]
fn both_stages_of_an_amount_reach_an_import_amount() {
    assert_eq!(
        "12.50".to_string().import_amount(),
        Ok(ImportAmount::parse("12.50").unwrap())
    );
    assert_eq!("-12.50".to_string().import_amount(), Err(NotAnImportAmount));
    let admitted = ImportAmount::parse("12.50").unwrap();
    assert_eq!(admitted.import_amount(), Ok(admitted.clone()));
}
