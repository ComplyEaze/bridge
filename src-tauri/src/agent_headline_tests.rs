use super::*;

fn date(text: &str) -> bridge_tally_core::TallyDate {
    bridge_tally_core::TallyDate::parse(text.to_string()).unwrap()
}

fn company() -> CompanyName {
    CompanyName::new("Synthetic Traders")
}

fn basis(gaps: Vec<Gap>) -> TrialBalanceBasis {
    TrialBalanceBasis::new(
        date("20260401"),
        date("20260902"),
        Completeness::from_gaps(gaps),
    )
}

fn page(offset: usize, shown: usize, total: usize) -> Page {
    Page::new(Rows::Ledgers, offset, shown, total)
}

#[test]
fn a_read_with_no_gap_is_whole_and_says_the_company_and_the_exact_period() {
    let headline = basis(vec![]).headline(&company(), page(0, 14, 14));
    assert_eq!(
        headline.lead,
        "Trial balance for \u{201c}Synthetic Traders\u{201d}, 1 Apr 2026 to 2 Sep 2026: read for every ledger."
    );
    assert_eq!(headline.rows.as_deref(), Some("All 14 ledgers are listed."));
    assert!(!headline.lead.contains("Partial"));
}

/// The state is in the lead, in words: the first sentence says it is partial
/// and what that costs, and the counts of what was left out follow it.
#[test]
fn a_gap_makes_the_read_partial_and_the_lead_says_so_and_counts_it() {
    for (foreign, mixed, counts) in [
        (3, 1, "3 ledgers kept in another currency and 1 base-currency ledger with a value Tally shows in another currency are left out"),
        (1, 0, "1 ledger kept in another currency and 0 base-currency ledgers with a value Tally shows in another currency are left out"),
    ] {
        let headline = basis(vec![Gap::BaseCurrencyLedgersOnly { foreign, mixed }])
            .headline(&company(), page(0, 5, 5));
        let (first, rest) = headline.lead.split_once(". ").unwrap();
        assert!(first.starts_with("Partial trial balance for "), "{first}");
        assert!(
            first.ends_with("base-currency ledgers only, so debit and credit totals are not expected to match"),
            "{first}"
        );
        assert!(rest.contains(counts), "{rest}");
        assert!(first.split_whitespace().count() < 30, "a short first sentence: {first}");
    }
}

/// A several-currency book with nothing set aside is still read for its
/// base-currency ledgers only, and says so without claiming ledgers were left
/// out or that the totals will differ.
#[test]
fn a_several_currency_book_with_nothing_set_aside_says_so() {
    let headline = basis(vec![Gap::BaseCurrencyLedgersOnly {
        foreign: 0,
        mixed: 0,
    }])
    .headline(&company(), page(0, 5, 5));
    assert_eq!(
        headline.lead,
        "Partial trial balance for \u{201c}Synthetic Traders\u{201d}, 1 Apr 2026 to 2 Sep 2026: base-currency ledgers only, and no ledger was left out. The book has several currency masters, so only its base-currency ledgers were read."
    );
    assert!(!headline.lead.contains("0 ledgers"), "{}", headline.lead);
}

/// Every gap is named, however many there are: none is dropped for another.
#[test]
fn every_gap_is_named_in_the_lead() {
    let completeness = Completeness::from_gaps(vec![
        Gap::BaseCurrencyLedgersOnly {
            foreign: 2,
            mixed: 0,
        },
        Gap::BaseCurrencyLedgersOnly {
            foreign: 7,
            mixed: 4,
        },
    ]);
    let headline = TrialBalanceBasis::new(date("20260401"), date("20260902"), completeness)
        .headline(&company(), page(0, 1, 1));
    assert!(
        headline
            .lead
            .contains("2 ledgers kept in another currency and 0 base-currency"),
        "{}",
        headline.lead
    );
    assert!(
        headline
            .lead
            .contains("7 ledgers kept in another currency and 4 base-currency"),
        "{}",
        headline.lead
    );
}

#[test]
fn only_an_empty_list_of_gaps_is_whole() {
    assert!(Completeness::from_gaps(Vec::new()).is_whole());
    assert!(!Completeness::from_gaps(vec![Gap::BaseCurrencyLedgersOnly {
        foreign: 0,
        mixed: 0
    }])
    .is_whole());
}

#[test]
fn the_page_sentence_says_which_rows_and_where_the_rest_begin() {
    let words = |offset, shown, total| page(offset, shown, total).sentence();
    assert_eq!(words(0, 14, 14), "All 14 ledgers are listed.");
    assert_eq!(
        words(0, 3, 14),
        "Ledgers 1 to 3 of 14; more follow, from offset 3."
    );
    assert_eq!(
        words(3, 3, 14),
        "Ledgers 4 to 6 of 14; more follow, from offset 6."
    );
    assert_eq!(words(12, 2, 14), "Ledgers 13 to 14 of 14: the last page.");
    assert_eq!(words(0, 0, 0), "There are no ledgers in this read.");
    assert!(words(20, 0, 14).starts_with("No ledgers on this page: there are 14 in all"));
}

/// A byte cap that trims the rows restates the row sentence from the rows that
/// are left, in the response itself; a whole page cut to part of it stops
/// saying "all".
#[test]
fn a_trimmed_page_restates_its_rows_and_never_claims_more_than_it_holds() {
    let mut response = json!({
        "headline": basis(vec![]).headline(&company(), page(0, 14, 14)),
        "result": {"ledgers": []},
    });
    assert_eq!(response["headline"]["rows"], "All 14 ledgers are listed.");
    restate_rows(&mut response, 9);
    assert_eq!(
        response["headline"]["rows"],
        "Ledgers 1 to 9 of 14; more follow, from offset 9."
    );
    // The lead is not touched, and a second cut restates again from what is left.
    assert!(response["headline"]["lead"]
        .as_str()
        .unwrap()
        .contains("read for every ledger"));
    restate_rows(&mut response, 4);
    assert_eq!(
        response["headline"]["rows"],
        "Ledgers 1 to 4 of 14; more follow, from offset 4."
    );
    // More rows than the page holds is not a claim.
    restate_rows(&mut response, 99);
    assert_eq!(
        response["headline"]["rows"],
        "Ledgers 1 to 4 of 14; more follow, from offset 4."
    );
}

#[test]
fn a_response_without_a_headline_is_left_alone() {
    let mut response = json!({"result": {"ledgers": [1, 2]}});
    restate_rows(&mut response, 1);
    assert_eq!(response, json!({"result": {"ledgers": [1, 2]}}));
}

/// Tally's company name is free text: control characters, bidirectional
/// overrides and zero-width marks are removed so that it cannot reorder or
/// hide the words around it, quotes inside it become apostrophes, it is always
/// quoted, and it is held to a bound.
#[test]
fn the_company_name_is_quoted_cleaned_and_bounded() {
    let name = CompanyName::new("  Odd\u{7}\nName \u{201c}Ltd\u{201d} \"x\" ");
    assert_eq!(name.quoted(), "\u{201c}Odd Name 'Ltd' 'x'\u{201d}");
    // A name written to close the quote and read as a sentence cannot: its
    // own quotes are apostrophes, so the state after it stays outside.
    let sentence =
        CompanyName::new("X\u{201d}, 1 Apr 2025 to 31 Mar 2026: read for every ledger. \u{201c}");
    assert_eq!(sentence.quoted().matches('\u{201d}').count(), 1);
    assert_eq!(sentence.quoted().matches('\u{201c}').count(), 1);
    let spoof = CompanyName::new("A\u{202e}B\u{200b}C\u{2066}D\u{feff}E");
    assert_eq!(spoof.quoted(), "\u{201c}ABCDE\u{201d}");
    // By category, not by list: the Arabic letter mark, the tag characters and
    // a soft hyphen are format characters too.
    let more = CompanyName::new("A\u{61c}B\u{e0041}C\u{ad}D");
    assert_eq!(more.quoted(), "\u{201c}ABCD\u{201d}");
    let long = CompanyName::new(&"n".repeat(5_000));
    assert_eq!(long.0.chars().count(), MAX_COMPANY_NAME_CHARS);
}

/// An offset at the top of the range is a sentence, not an overflow.
#[test]
fn an_offset_at_the_top_of_the_range_does_not_overflow() {
    let sentence = page(usize::MAX, 0, 14).sentence();
    assert!(
        sentence.starts_with("No ledgers on this page"),
        "{sentence}"
    );
    let sentence = page(usize::MAX - 1, 5, 14).sentence();
    assert!(sentence.contains("of 14"), "{sentence}");
}

/// A headline that cannot be restated fails closed: its rows sentence goes
/// rather than staying beside rows it no longer describes.
#[test]
fn a_headline_that_cannot_be_restated_loses_its_rows_sentence() {
    let mut response = json!({
        "headline": {"lead": "x", "rows": "All 14 ledgers are listed.", "page": {"rows": "vouchers", "offset": 0, "shown": 14, "total": 14}},
    });
    restate_rows(&mut response, 3);
    assert_eq!(response["headline"], json!({"lead": "x"}));
}

#[test]
fn dates_are_written_for_a_person() {
    assert_eq!(plain_date(&date("20260401")), "1 Apr 2026");
    assert_eq!(plain_date(&date("20251231")), "31 Dec 2025");
}
