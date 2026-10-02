use super::*;

/// The refusal of `requested` against `names`, with what it lists.
fn refusal(
    names: &[&str],
    requested: &str,
    redaction: Redaction,
) -> (String, LedgerMiss, Vec<Value>) {
    let failure =
        resolve_ledger_or_refuse(names.iter().copied(), requested, redaction).unwrap_err();
    let candidates = failure
        .candidates
        .expect("a ledger refusal carries candidates");
    (
        failure.code,
        candidates.miss.expect("and says what the listing means"),
        candidates.items,
    )
}

/// The names a refusal lists, in order, read back out of their party markers.
fn listed_names(items: &[Value]) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            redact_value(item.clone(), Redaction::None)["name"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

#[test]
fn a_resolvable_name_is_still_resolved_and_nothing_is_listed() {
    assert_eq!(
        resolve_ledger_or_refuse(["Cash", "HDFC Bank"].into_iter(), "cash", Redaction::None)
            .unwrap(),
        "Cash"
    );
}

#[test]
fn a_truncated_name_lists_the_ledger_it_could_be_and_resolves_nothing() {
    let (code, miss, items) = refusal(&["HDFC Bank", "Cash"], "hdfc", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(miss.listing, Listing::Listed);
    assert_eq!(miss.found, 1);
    assert_eq!(listed_names(&items), ["HDFC Bank"]);
    assert_eq!(items[0]["rule"], "catalog_prefix");
}

#[test]
fn a_name_that_resembles_nothing_says_none_and_not_missing() {
    let (code, miss, items) = refusal(&["Cash", "HDFC Bank"], "Zzqx", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(miss.listing, Listing::None);
    assert_eq!(miss.found, 0);
    assert!(items.is_empty());
}

/// A number inside the name points at one ledger. It is shown for the user to
/// confirm; it is never the resolution, which stays the resolver's refusal.
#[test]
fn an_embedded_number_is_listed_for_confirmation_and_never_resolves() {
    let (code, miss, items) = refusal(
        &["Ramesh Kumar 9876543210", "Cash"],
        "Ramesh 9876543210",
        Redaction::None,
    );
    assert_eq!(code, "ledger_not_found");
    assert_eq!(miss.listing, Listing::Listed);
    assert_eq!(listed_names(&items), ["Ramesh Kumar 9876543210"]);
    assert_eq!(items[0]["rule"], "shared_identifier");
}

/// `a_b` and `AB` and `A-B` share one lookup key; the ambiguity lists exactly
/// the ledgers that collide, sorted, with an exact count.
#[test]
fn an_ambiguous_name_lists_exactly_the_colliding_ledgers() {
    let (code, miss, items) = refusal(&["Cash", "AB", "A-B"], "a_b", Redaction::None);
    assert_eq!(code, "ledger_ambiguous");
    assert_eq!(
        (miss.listing, miss.found, miss.found_is_lower_bound),
        (Listing::Listed, 2, false)
    );
    assert_eq!(listed_names(&items), ["A-B", "AB"]);
    assert!(items.iter().all(|item| item["rule"] == "lookup_key_equal"));
}

/// A name that reaches a whole family and separates none of it is counted and
/// not listed: an empty list there is not "nothing resembles it".
#[test]
fn a_family_of_matches_is_counted_and_withheld() {
    let names = (1..=30)
        .map(|number| format!("DN Party 0{number:02}"))
        .collect::<Vec<_>>();
    let refs = names.iter().map(String::as_str).collect::<Vec<_>>();
    let (code, miss, items) = refusal(&refs, "DN Party 0", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(miss.listing, Listing::Withheld);
    assert_eq!(miss.found, 30);
    assert!(items.is_empty());
}

/// A search that cannot run is `unavailable`, never `none`, and the refusal
/// keeps its code.
#[test]
fn a_search_that_cannot_run_is_unavailable_and_keeps_the_code() {
    let (code, miss, items) = refusal(&["Cash", "Bad\u{202e}Name"], "Cashh", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(miss.listing, Listing::Unavailable);
    assert_eq!(miss.reason, Some("master_name_unsafe"));
    assert!(items.is_empty());
}

/// Under `mask_parties` nothing is searched and nothing is counted: the names
/// are not listed (a masked name cannot be sent back as a ledger name), and a
/// count would answer "does a ledger start with this?" for every prefix tried.
#[test]
fn masking_withholds_the_names_and_the_count() {
    for (names, requested, code) in [
        (&["HDFC Bank", "Cash"][..], "hdfc", "ledger_not_found"),
        (&["A-B", "AB"][..], "a_b", "ledger_ambiguous"),
    ] {
        let (got, miss, items) = refusal(names, requested, Redaction::MaskParties);
        assert_eq!(got, code);
        assert_eq!(
            (miss.listing, miss.found, miss.reason),
            (Listing::NamesMasked, 0, None)
        );
        assert!(items.is_empty());
    }
}

/// A masked spelling is `Ra…rs`, and the lookup key drops everything that is
/// not a letter or a digit: any retyping of it (`...`, `..`, `**`, a space, a
/// hyphen, a lookalike mark) would resolve to a ledger named `RARS`. Refused
/// instead, but only under masking, and a ledger spelled exactly as asked is
/// still reached.
#[test]
fn a_masked_spelling_is_refused_and_not_resolved_to_another_ledger() {
    let book = ["RARS", "Ramesh Traders"];
    for spelling in [
        "Ra…rs", "Ra...rs", "Ra..rs", "Ra**rs", "Ra rs", "Ra-rs", "Ra⋯rs", "rars",
    ] {
        let failure = resolve_ledger_or_refuse(book.into_iter(), spelling, Redaction::MaskParties)
            .unwrap_err();
        assert_eq!(failure.code, "ledger_name_masked", "{spelling}");
        assert!(failure.candidates.is_none());
    }
    // The ledger itself, spelled as it is, is reached; and without masking
    // the same retypings resolve as they always did.
    assert_eq!(
        resolve_ledger_or_refuse(book.into_iter(), "RARS", Redaction::MaskParties).unwrap(),
        "RARS"
    );
    assert_eq!(
        resolve_ledger_or_refuse(book.into_iter(), "Ra…rs", Redaction::None).unwrap(),
        "RARS"
    );
    assert_eq!(
        resolve_ledger_or_refuse(
            ["Misc... Expenses", "RARS"].into_iter(),
            "Misc... Expenses",
            Redaction::MaskParties
        )
        .unwrap(),
        "Misc... Expenses"
    );
    // The shortened form of the very ledger it resolves to is not another
    // party's: it resolves, as it always did.
    assert_eq!(
        resolve_ledger_or_refuse(["Ra-Rs"].into_iter(), "Ra..Rs", Redaction::MaskParties).unwrap(),
        "Ra-Rs"
    );
    // The mark alone is enough when the request resolves and no other ledger's
    // masked form has its key: `Ab…` finds `Ab`, and is refused all the same.
    let failure =
        resolve_ledger_or_refuse(["Ab"].into_iter(), "Ab…", Redaction::MaskParties).unwrap_err();
    assert_eq!(failure.code, "ledger_name_masked");
    // An ordinary typed name, long enough to share no key with a masked form,
    // is left alone under masking.
    assert_eq!(
        resolve_ledger_or_refuse(
            ["HDFC Bank", "Ramesh Traders"].into_iter(),
            "hdfc bank",
            Redaction::MaskParties
        )
        .unwrap(),
        "HDFC Bank"
    );
    // A request that matches no ledger and carries the mark is refused as
    // masked too, with no search and no names.
    let failure =
        resolve_ledger_or_refuse(book.into_iter(), "Zz…qq", Redaction::MaskParties).unwrap_err();
    assert_eq!(failure.code, "ledger_name_masked");
}

#[test]
fn the_remediation_for_each_code_says_to_ask_the_user_and_never_to_pick() {
    for code in ["ledger_not_found", "ledger_ambiguous"] {
        let text = refusal_remediation(code).expect(code);
        assert!(
            text.contains("ask the user") || text.contains("ask which one"),
            "{code}"
        );
        assert!(text.contains("ComplyEaze Bridge chose none"), "{code}");
    }
    let not_found = refusal_remediation("ledger_not_found").unwrap();
    for needle in [
        "even one candidate needs the user's confirmation",
        "none is marked best: the order is by rule and then name, not by likelihood",
        "do not say the ledger does not exist",
        "If `candidates_listing` is absent, no candidates were attached because the response budget is small",
    ] {
        assert!(not_found.contains(needle), "missing: {needle}");
    }
    let masked = refusal_remediation("ledger_name_masked").unwrap();
    assert!(masked.contains("exactly as spelled in Tally"));
    assert!(masked.contains("refused it"));
}

/// A list cut to fit the byte budget says `truncated`, with the full count
/// kept; a listing that was already something else keeps its word.
#[test]
fn a_list_cut_by_the_budget_is_truncated_and_other_words_stay() {
    let miss = |listing| LedgerMiss {
        listing,
        found: 20,
        found_is_lower_bound: false,
        reason: None,
    };
    assert_eq!(miss(Listing::Listed).listing_word(true), "truncated");
    assert_eq!(miss(Listing::Listed).listing_word(false), "listed");
    for listing in [
        Listing::None,
        Listing::Withheld,
        Listing::NamesMasked,
        Listing::Unavailable,
        Listing::Truncated,
    ] {
        assert_eq!(miss(listing).listing_word(true), listing.as_str());
    }
}

/// Twenty-five colliding ledgers are all listed; a twenty-sixth makes the list
/// `truncated`, with the full count.
#[test]
fn the_collision_list_is_cut_after_the_core_cap_and_counts_them_all() {
    for (count, listing, listed) in [(25, Listing::Listed, 25), (26, Listing::Truncated, 25)] {
        let names = (0..count)
            .map(|index| format!("Dup {}", "-".repeat(index + 1)))
            .collect::<Vec<_>>();
        let refs = names.iter().map(String::as_str).collect::<Vec<_>>();
        let (code, miss, items) = refusal(&refs, "dup", Redaction::None);
        assert_eq!(code, "ledger_ambiguous");
        assert_eq!(
            (miss.listing, miss.found, items.len()),
            (listing, count, listed)
        );
    }
}
