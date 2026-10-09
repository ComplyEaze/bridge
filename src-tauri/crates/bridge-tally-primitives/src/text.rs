//! Which characters a reader sees, for every crate that reads text in or writes
//! it to Tally.

/// Whether `character` draws nothing: Unicode Default_Ignorable_Code_Point or
/// General_Category Cf, every future one included without a list. Not counted:
/// ZERO WIDTH JOINER and ZERO WIDTH NON-JOINER, which Devanagari and other Indic
/// spellings need to shape a word, and the prepended concatenation marks
/// (U+0600 to U+0605 and their kind), which are Cf but print a sign.
pub fn draws_nothing(character: char) -> bool {
    use icu_properties::{
        props::{DefaultIgnorableCodePoint, GeneralCategory, PrependedConcatenationMark},
        CodePointMapData, CodePointSetData,
    };
    !matches!(character, '\u{200C}' | '\u{200D}')
        && !CodePointSetData::new::<PrependedConcatenationMark>().contains(character)
        && (CodePointSetData::new::<DefaultIgnorableCodePoint>().contains(character)
            || CodePointMapData::<GeneralCategory>::new().get(character) == GeneralCategory::Format)
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
