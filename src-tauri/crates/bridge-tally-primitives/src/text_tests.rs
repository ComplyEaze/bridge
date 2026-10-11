use super::*;

#[test]
fn default_ignorable_and_format_characters_draw_nothing() {
    for character in [
        '\u{202A}',  // left-to-right embedding
        '\u{202E}',  // right-to-left override
        '\u{2066}',  // left-to-right isolate
        '\u{2069}',  // pop directional isolate
        '\u{200E}',  // left-to-right mark
        '\u{061C}',  // Arabic letter mark
        '\u{200B}',  // zero width space
        '\u{2060}',  // word joiner
        '\u{2062}',  // invisible times
        '\u{FEFF}',  // byte-order mark
        '\u{00AD}',  // soft hyphen
        '\u{FE0F}',  // variation selector
        '\u{E0041}', // tag latin capital letter A
        '\u{FFF9}',  // interlinear annotation anchor (Cf, not default-ignorable)
    ] {
        assert!(draws_nothing(character), "{:04X}", u32::from(character));
    }
}

#[test]
fn letters_spaces_the_indic_joiners_and_printed_marks_are_read() {
    for character in [
        'A', '7', ' ', '-', '|', '\u{0915}', // Devanagari KA
        '\u{094D}', // Devanagari virama
        '\u{200C}', // zero width non-joiner
        '\u{200D}', // zero width joiner
        '\u{20B9}', // rupee sign
        '\u{FFFD}', // replacement character
        '\u{0600}', // Arabic number sign: Cf, but it prints
    ] {
        assert!(!draws_nothing(character), "{:04X}", u32::from(character));
    }
}
