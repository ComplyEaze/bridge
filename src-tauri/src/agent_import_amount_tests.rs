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

/// A text with two dots is not an import amount, whether it is split at the first dot or at the
/// last: each of these splits differently, and is refused either way (#2058).
#[test]
fn a_text_with_two_dots_is_not_an_import_amount() {
    for text in ["1.0.00", "1.00.0", "1.00.00", "12.50.", "0.0.01", "..00"] {
        assert_eq!(
            ImportAmount::parse(text),
            Err(NotAnImportAmount),
            "{text:?}"
        );
    }
}

/// Letters, signs and non-ASCII digits in either part are refused, a fractional part of one
/// character two bytes long (`13.٥`) included, which passes the length test. `parse` checks the
/// digits of each part itself and `ExactDecimal::parse` checks them again, so these hold the
/// pair, not either alone (see `exact_decimal_refuses_what_parse_relies_on_it_for`).
#[test]
fn a_part_that_is_not_ascii_digits_is_not_an_import_amount() {
    for text in [
        "1a.00",
        "13.a1",
        "13.-5",
        "13.+5",
        "13.5e",
        "--13.00",
        "1e2.00",
        "٣.٠٠",
        "١٢.٥٠",
        "13.٥٠",
        "1٢.50",
        "13.٥",
        "13.²",
    ] {
        assert_eq!(
            ImportAmount::parse(text),
            Err(NotAnImportAmount),
            "{text:?}"
        );
    }
}

/// An amount is read as it is written: whitespace round it is not trimmed away, in the journal
/// (`Deserialize`) or in the tool's text (`PayloadAmount for String`).
#[test]
fn whitespace_round_an_amount_is_refused_where_it_is_read_and_where_it_is_admitted() {
    for text in [" 12.50", "12.50 ", " 12.50 ", "\t12.50", "12.50\n"] {
        assert!(
            serde_json::from_value::<ImportAmount>(json!(text)).is_err(),
            "{text:?}"
        );
        assert_eq!(
            text.to_string().import_amount(),
            Err(NotAnImportAmount),
            "{text:?}"
        );
    }
}

/// An admitted amount parsed again from its received text is itself, so a stage that parses
/// again changes nothing (`PayloadAmount for ImportAmount` returns a clone).
#[test]
fn an_admitted_amount_parsed_again_from_its_received_text_is_itself() {
    for text in ["0012.50", "12.50", "1.00", "0.01", "000.01", "10.10"] {
        let amount = ImportAmount::parse(text).unwrap();
        assert_eq!(ImportAmount::parse(amount.as_str()), Ok(amount.clone()));
        assert_eq!(amount.as_str(), text);
    }
}

/// The canonical form is worked out without a failure, at the longest amount too: adding zero
/// to a text of at most 256 bytes gives a form no longer than it. Subtracting zero gives the
/// same form, which is what lets `parse` take either.
#[test]
fn the_canonical_form_is_worked_out_without_failure_at_the_longest_amount() {
    let longest = format!("{}.99", "9".repeat(253));
    let amount = ImportAmount::parse(&longest).unwrap();
    assert_eq!(amount.as_str().len(), 256);
    assert_eq!(amount.canonical(), longest);
    assert_eq!(ImportAmount::parse(amount.as_str()), Ok(amount.clone()));
    let zero = ExactDecimal::zero();
    for (text, canonical) in [
        ("0012.50", "12.5"),
        ("12.50", "12.5"),
        ("1.00", "1"),
        ("0.01", "0.01"),
        ("000.10", "0.1"),
        ("10.00", "10"),
    ] {
        let lexeme = ExactDecimal::parse(text).unwrap();
        assert_eq!(lexeme.checked_add(&zero).unwrap().as_str(), canonical);
        assert_eq!(lexeme.checked_subtract(&zero).unwrap().as_str(), canonical);
    }
}

/// What `parse` leaves to `ExactDecimal::parse` (#2058): it refuses a text with nothing before
/// the dot, a part with anything but ASCII digits, and (of the signs) every one but a leading
/// minus, which is why `parse` checks the whole part itself. Zero is zero, so an error taken as
/// zero is refused as zero.
#[test]
fn exact_decimal_refuses_what_parse_relies_on_it_for() {
    for text in [
        ".50", "-.50", "1a.00", "13.a1", "13.-5", "13.+5", "13.5e", "+13.00", "1e2.00", "٣.٠٠",
        "13.٥٠", "13.٥", "13.²",
    ] {
        assert!(ExactDecimal::parse(text).is_err(), "{text:?}");
    }
    assert!(ExactDecimal::parse("-13.00").is_ok());
    assert!(ExactDecimal::zero().is_zero());
}
