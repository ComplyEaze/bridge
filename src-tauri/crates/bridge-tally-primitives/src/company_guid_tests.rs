use super::*;

const LOWER: &str = "0a1b2c3d-4e5f-4a6b-8c7d-9e0f1a2b3c4d";

#[test]
fn accepts_lowercase_uppercase_and_mixed_case_and_stores_lowercase() {
    for spelling in [
        LOWER.to_string(),
        LOWER.to_ascii_uppercase(),
        "0A1b2C3d-4E5f-4A6b-8C7d-9E0f1A2b3C4d".to_string(),
    ] {
        let guid = CompanyGuid::parse(&spelling).expect("strict spelling parses");
        assert_eq!(guid.as_str(), LOWER);
        assert_eq!(guid.to_string(), LOWER);
    }
}

#[test]
fn any_case_spelling_of_one_guid_is_equal() {
    assert_eq!(
        CompanyGuid::parse(LOWER).unwrap(),
        CompanyGuid::parse(&LOWER.to_ascii_uppercase()).unwrap()
    );
}

#[test]
fn matches_observed_ignores_ascii_case_only() {
    let guid = CompanyGuid::parse(LOWER).unwrap();
    assert!(guid.matches_observed(LOWER));
    assert!(guid.matches_observed(&LOWER.to_ascii_uppercase()));
    assert!(!guid.matches_observed(&format!(" {LOWER}")));
    assert!(!guid.matches_observed(&LOWER[..35]));
    assert!(!guid.matches_observed(""));
}

#[test]
fn rejects_35_and_37_bytes() {
    assert_eq!(
        CompanyGuid::parse(&LOWER[..35]),
        Err(CompanyGuidError::WrongLength)
    );
    assert_eq!(
        CompanyGuid::parse(&format!("{LOWER}0")),
        Err(CompanyGuidError::WrongLength)
    );
    assert_eq!(CompanyGuid::parse(""), Err(CompanyGuidError::WrongLength));
}

#[test]
fn rejects_a_hyphen_moved() {
    // Same length, same digit count, hyphen one byte to the left.
    let moved = "0a1b2c3-d4e5f-4a6b-8c7d-9e0f1a2b3c4d";
    assert_eq!(moved.len(), 36);
    assert_eq!(
        CompanyGuid::parse(moved),
        Err(CompanyGuidError::MisplacedHyphen)
    );
    // A hyphen replaced by a digit.
    let missing = "0a1b2c3d04e5f-4a6b-8c7d-9e0f1a2b3c4d";
    assert_eq!(missing.len(), 36);
    assert_eq!(
        CompanyGuid::parse(missing),
        Err(CompanyGuidError::MisplacedHyphen)
    );
}

#[test]
fn rejects_a_non_hex_character_at_every_digit_position() {
    for offset in (0..36).filter(|offset| ![8, 13, 18, 23].contains(offset)) {
        let mut bytes = LOWER.as_bytes().to_vec();
        bytes[offset] = b'g';
        let text = String::from_utf8(bytes).unwrap();
        assert_eq!(
            CompanyGuid::parse(&text),
            Err(CompanyGuidError::NotHexadecimal),
            "offset {offset}"
        );
    }
}

#[test]
fn never_trims() {
    assert_eq!(
        CompanyGuid::parse(&format!(" {}", &LOWER[..35])),
        Err(CompanyGuidError::NotHexadecimal)
    );
    assert_eq!(
        CompanyGuid::parse(&format!("{} ", &LOWER[..35])),
        Err(CompanyGuidError::NotHexadecimal)
    );
    assert_eq!(
        CompanyGuid::parse(&format!(" {LOWER}")),
        Err(CompanyGuidError::WrongLength)
    );
    assert_eq!(
        CompanyGuid::parse(&format!("{LOWER}\n")),
        Err(CompanyGuidError::WrongLength)
    );
}

#[test]
fn rejects_braces_simple_and_urn_forms() {
    assert_eq!(
        CompanyGuid::parse(&format!("{{{LOWER}}}")),
        Err(CompanyGuidError::WrongLength)
    );
    assert_eq!(
        CompanyGuid::parse(&LOWER.replace('-', "")),
        Err(CompanyGuidError::WrongLength)
    );
    assert_eq!(
        CompanyGuid::parse(&format!("urn:uuid:{LOWER}")),
        Err(CompanyGuidError::WrongLength)
    );
    // Braces swallowing the first and last digit keep the 36-byte length.
    let braced = format!("{{{}}}", &LOWER[1..35]);
    assert_eq!(braced.len(), 36);
    assert_eq!(
        CompanyGuid::parse(&braced),
        Err(CompanyGuidError::NotHexadecimal)
    );
}

#[test]
fn rejects_non_ascii_text() {
    // Fullwidth digit: 3 bytes, so the total byte length is 38.
    let fullwidth = LOWER.replacen('0', "\u{FF10}", 1);
    assert_eq!(
        CompanyGuid::parse(&fullwidth),
        Err(CompanyGuidError::WrongLength)
    );
    // Two-byte letter in place of two digits keeps exactly 36 bytes.
    let two_byte = format!("\u{00e9}{}", &LOWER[2..]);
    assert_eq!(two_byte.len(), 36);
    assert_eq!(
        CompanyGuid::parse(&two_byte),
        Err(CompanyGuidError::NotHexadecimal)
    );
    // Kelvin sign lowercases to ASCII 'k' under Unicode rules; it is not hex
    // either way and must not be folded into the accepted form.
    let kelvin = LOWER.replacen('a', "\u{212A}", 1);
    // The 3-byte sign replaces a 1-byte digit, so the input is 38 bytes and
    // the length check refuses it before any character is examined.
    assert_eq!(kelvin.len(), 38);
    assert_eq!(
        CompanyGuid::parse(&kelvin),
        Err(CompanyGuidError::WrongLength)
    );
}
