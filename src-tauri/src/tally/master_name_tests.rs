use super::*;

fn name(value: &str) -> MasterName {
    MasterName::new(value).expect("a valid name")
}

#[test]
fn a_plain_ascii_name_is_accepted_and_kept_verbatim() {
    assert_eq!(
        name("Acme Traders - Delhi (No. 1)/A").as_str(),
        "Acme Traders - Delhi (No. 1)/A"
    );
}

#[test]
fn an_empty_name_is_refused() {
    assert_eq!(MasterName::new(""), Err(MasterNameError::Empty));
}

#[test]
fn a_non_ascii_name_is_refused() {
    for value in [
        "Caf\u{e9}",
        "\u{928}\u{92e}",
        "A\u{a0}B",
        "A\u{200b}B",
        "\u{feff}A",
    ] {
        assert_eq!(
            MasterName::new(value),
            Err(MasterNameError::NonAscii),
            "{value:?}"
        );
    }
}

#[test]
fn every_ascii_control_character_is_refused() {
    for code in (0x00u8..=0x1F).chain([0x7F]) {
        let value = format!("A{}B", char::from(code));
        assert_eq!(
            MasterName::new(&value),
            Err(MasterNameError::ControlCharacter),
            "0x{code:02X}"
        );
    }
}

#[test]
fn leading_or_trailing_whitespace_is_refused() {
    for value in [" A", "A ", " ", "  "] {
        assert_eq!(
            MasterName::new(value),
            Err(MasterNameError::SurroundingWhitespace),
            "{value:?}"
        );
    }
}

#[test]
fn a_run_of_two_or_more_spaces_is_refused() {
    for value in ["A  B", "A   B"] {
        assert_eq!(
            MasterName::new(value),
            Err(MasterNameError::RepeatedSpaces),
            "{value:?}"
        );
    }
}

#[test]
fn the_length_cap_is_the_master_binding_cap_and_is_inclusive() {
    assert_eq!(MAX_NAME_CHARS, 16_384);
    assert!(MasterName::new(&"A".repeat(MAX_NAME_CHARS)).is_ok());
    assert_eq!(
        MasterName::new(&"A".repeat(MAX_NAME_CHARS + 1)),
        Err(MasterNameError::TooLong)
    );
}
