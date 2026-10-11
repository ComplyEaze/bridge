use super::narration_search_key;

#[test]
fn composed_and_decomposed_accents_meet() {
    assert_eq!(
        narration_search_key("Caf\u{e9}"),
        narration_search_key("Cafe\u{301}")
    );
}

#[test]
fn case_and_whitespace_runs_fold() {
    assert_eq!(
        narration_search_key("  Rent \r\n  FOR\tAug "),
        "rent for aug"
    );
}

#[test]
fn a_dash_or_an_apostrophe_is_not_folded_into_another() {
    assert_ne!(
        narration_search_key("a-b"),
        narration_search_key("a\u{2013}b")
    );
    assert_ne!(
        narration_search_key("it's"),
        narration_search_key("it\u{2019}s")
    );
    assert_ne!(narration_search_key("a-b"), narration_search_key("a b"));
}

#[test]
fn devanagari_survives_the_fold_unchanged() {
    assert_eq!(
        narration_search_key("\u{928}\u{92e}\u{938}\u{94d}\u{924}\u{947}"),
        "\u{928}\u{92e}\u{938}\u{94d}\u{924}\u{947}"
    );
}
