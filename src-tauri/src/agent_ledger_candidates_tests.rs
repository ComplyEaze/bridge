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
fn a_name_in_another_ascii_case_is_resolved_and_says_so() {
    assert_eq!(
        resolve_ledger_or_refuse(["Cash", "HDFC Bank"].into_iter(), "cash", Redaction::None)
            .unwrap(),
        LedgerMatch::CaseOrSpacing {
            name: "Cash".to_string()
        }
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
    assert_eq!(items[0]["rule"], "identifier_match");
    // The core skipped its name search here, so the one name is not "every
    // ledger like this": the count is a floor, and the reason says why.
    assert!(miss.found_is_lower_bound);
    assert_eq!(miss.reason, Some("name_search_not_run"));
}

/// Two ledgers that differ only in case are both what `sales local` spells:
/// the ambiguity lists exactly those, sorted, with an exact count, and not the
/// hyphenated ledger, which only a looser fold would reach.
#[test]
fn an_ambiguous_name_lists_exactly_the_colliding_ledgers() {
    let (code, miss, items) = refusal(
        &["Cash", "Sales Local", "SALES LOCAL", "Sales - Local"],
        "sales local",
        Redaction::None,
    );
    assert_eq!(code, "ledger_ambiguous");
    assert_eq!(
        (miss.listing, miss.found, miss.found_is_lower_bound),
        (Listing::Listed, 2, false)
    );
    assert_eq!(listed_names(&items), ["SALES LOCAL", "Sales Local"]);
    assert!(items
        .iter()
        .all(|item| item["rule"] == "case_or_spacing_equal"));
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
        (
            &["Sales Local", "SALES LOCAL"][..],
            "sales local",
            "ledger_ambiguous",
        ),
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
    // The ledger itself, spelled as it is, is reached.
    assert_eq!(
        resolve_ledger_or_refuse(book.into_iter(), "RARS", Redaction::MaskParties)
            .unwrap()
            .name(),
        "RARS"
    );
    // The mark itself is refused whatever the setting: a name copied while
    // masking was on must not be read as another party after it is turned off.
    for spelling in ["Ra…rs", "Ra...rs"] {
        let failure =
            resolve_ledger_or_refuse(book.into_iter(), spelling, Redaction::None).unwrap_err();
        assert_eq!(failure.code, "ledger_name_masked", "{spelling}");
    }
    // Without masking, a retyping without the mark is an ordinary miss:
    // Bridge wrote no masked name to retype. It no longer resolves (#1076),
    // and the ledger it used to read is offered for the user to confirm.
    let (code, _, items) = refusal(&book, "Ra rs", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert!(listed_names(&items).contains(&"RARS".to_string()));
    assert_eq!(
        resolve_ledger_or_refuse(
            ["Misc... Expenses", "RARS"].into_iter(),
            "Misc... Expenses",
            Redaction::MaskParties
        )
        .unwrap()
        .name(),
        "Misc... Expenses"
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
        .unwrap()
        .name(),
        "HDFC Bank"
    );
    // A request that matches no ledger and carries the mark is refused as
    // masked too, with no search and no names.
    for redaction in [Redaction::MaskParties, Redaction::None] {
        let failure = resolve_ledger_or_refuse(book.into_iter(), "Zz…qq", redaction).unwrap_err();
        assert_eq!(failure.code, "ledger_name_masked", "{redaction:?}");
    }
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
        "none is marked best: the order is by rule strength and then name, not by likelihood",
        "do not say the ledger does not exist",
        "If `candidates_listing` is absent, no candidates were attached because the response budget is small",
    ] {
        assert!(not_found.contains(needle), "missing: {needle}");
    }
    let masked = refusal_remediation("ledger_name_masked").unwrap();
    assert!(masked.contains("exactly as spelled in Tally"));
    assert!(masked.starts_with("Refused: ask the user to type the full ledger name"));
    assert!(masked.contains("did not use it"));
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

/// Twenty-five ledgers that differ only in case are all listed; a twenty-sixth
/// makes the list `truncated`, with the full count.
#[test]
fn the_collision_list_is_cut_after_the_core_cap_and_counts_them_all() {
    for (count, listing, listed) in [(25, Listing::Listed, 25), (26, Listing::Truncated, 25)] {
        let names = case_variants("dupxy", count);
        let refs = names.iter().map(String::as_str).collect::<Vec<_>>();
        let (code, miss, items) = refusal(&refs, "dupxy", Redaction::None);
        assert_eq!(code, "ledger_ambiguous");
        assert_eq!(
            (miss.listing, miss.found, items.len()),
            (listing, count, listed)
        );
    }
}

/// `count` distinct spellings of `word` that differ only in ASCII case, none of
/// them all lower case.
fn case_variants(word: &str, count: usize) -> Vec<String> {
    let letters = word.chars().collect::<Vec<_>>();
    (1..=count)
        .map(|mask| {
            letters
                .iter()
                .enumerate()
                .map(|(index, letter)| {
                    if mask >> index & 1 == 1 {
                        letter.to_ascii_uppercase()
                    } else {
                        *letter
                    }
                })
                .collect()
        })
        .collect()
}

// #1076 Part 1. Each test below has a mutant it kills, named in its comment.

/// `A & B` once read a lone `AB`, because the loose key drops `&`. It is a
/// different spelling, so it asks, and `AB` is offered, never chosen.
/// Mutant killed: resolving on the loose key again.
#[test]
fn a_dropped_symbol_no_longer_resolves_and_is_offered() {
    let (code, miss, items) = refusal(&["AB", "Cash"], "A & B", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(miss.listing, Listing::Listed);
    assert_eq!(listed_names(&items), ["AB"]);
    assert_eq!(items[0]["rule"], "lookup_key_equal");
}

/// Names that differ only in an Indic virama or vowel sign once read as one
/// ledger, because the loose key drops every mark that is not a letter or a
/// digit (#1076). They are different spellings, so each asks and offers the
/// ledger, and none is chosen. The first drops the virama (a key match, the
/// rule that offers the old loose match); the second drops the vowel sign, so
/// it is also a prefix of the ledger's name.
/// Mutant killed: resolving on the loose key again.
#[test]
fn a_dropped_virama_or_vowel_sign_asks_and_offers_the_ledger() {
    for requested in ["खरचा", "खर्च"] {
        let (code, miss, items) = refusal(&["खर्चा", "Cash"], requested, Redaction::None);
        assert_eq!(code, "ledger_not_found", "{requested}");
        assert_eq!(miss.listing, Listing::Listed, "{requested}");
        assert_eq!(listed_names(&items), ["खर्चा"], "{requested}");
    }
    let (_, _, items) = refusal(&["खर्चा", "Cash"], "खरचा", Redaction::None);
    assert_eq!(items[0]["rule"], "lookup_key_equal");
}

/// The measured wrong ledger: a truncated `Input Cess (` read `Input Cess`,
/// not `Input Cess (M2)` (both names from a captured lab catalogue).
/// Mutant killed: resolving on the loose key again.
#[test]
fn a_truncated_name_does_not_read_a_different_ledger() {
    let failure = resolve_ledger_or_refuse(
        ["Input Cess", "Input Cess (M2)"].into_iter(),
        "Input Cess (",
        Redaction::None,
    )
    .unwrap_err();
    assert_eq!(failure.code, "ledger_not_found");
}

/// Case and spaces resolve, ASCII only.
/// Mutants killed: an exact-only key; a key that does not collapse spaces.
#[test]
fn ascii_case_and_spaces_resolve() {
    for requested in ["cash", "CASH", "  Cash  ", "Sales   Local", " sales local"] {
        let found = resolve_ledger_or_refuse(
            ["Cash", "Sales Local"].into_iter(),
            requested,
            Redaction::None,
        )
        .unwrap();
        assert!(
            matches!(found, LedgerMatch::CaseOrSpacing { .. }),
            "{requested}"
        );
    }
}

/// Decision A and reference 9.4f: the case of a non-ASCII letter is not
/// folded, so `CAFÉ NAÏVE TRADERS` asks, with the ledger offered.
/// Mutant killed: Unicode lower-casing in the spelling key.
#[test]
fn a_non_ascii_case_difference_asks_and_offers_the_ledger() {
    let (code, _, items) = refusal(
        &["Café Naïve Traders", "Cash"],
        "CAFÉ NAÏVE TRADERS",
        Redaction::None,
    );
    assert_eq!(code, "ledger_not_found");
    assert!(listed_names(&items).contains(&"Café Naïve Traders".to_string()));
}

/// A folded match whose ledger has a twin that differs only by a trailing
/// CR LF (reference 9.4e) is ambiguous: no one can type the CR LF, and either
/// could be meant. Mutant killed: dropping the twin check.
#[test]
fn a_folded_match_with_a_twin_is_ambiguous() {
    let (code, miss, items) = refusal(&["LF708 C", "lf708 c\r\n"], "lf708 c", Redaction::None);
    assert_eq!(code, "ledger_ambiguous");
    assert_eq!(miss.found, 2);
    assert_eq!(listed_names(&items), ["LF708 C", "lf708 c\r\n"]);
}

/// An exact spelling resolves even when it has a twin, and names the twin in
/// the answer: refusing would make the exact ledger unreachable.
/// Mutant killed: leaving `similar` empty.
#[test]
fn an_exact_spelling_with_a_twin_resolves_and_names_the_twin() {
    let found = resolve_ledger_or_refuse(
        ["LF708 A", "LF708 A\r\n", "Cash"].into_iter(),
        "LF708 A",
        Redaction::None,
    )
    .unwrap();
    assert_eq!(
        found,
        LedgerMatch::Exact {
            name: "LF708 A".to_string(),
            similar: vec!["LF708 A\r\n".to_string()]
        }
    );
    let echo = found.to_json(Redaction::None);
    assert_eq!(echo["matched"], "exact");
    assert_eq!(echo["similar_ledgers"].as_array().unwrap().len(), 1);
}

/// A name typed run together resembles nothing to the binding rules; the
/// ledger the old key read is still offered, so no spelling that used to
/// resolve meets an empty list. Mutant killed: dropping the loose candidates.
#[test]
fn a_run_together_name_is_offered_the_ledger_it_used_to_read() {
    let (code, miss, items) = refusal(
        &["Bank of Baroda CA", "Cash"],
        "BankofBarodaCA",
        Redaction::None,
    );
    assert_eq!(code, "ledger_not_found");
    assert_eq!((miss.listing, miss.found), (Listing::Listed, 1));
    // The rules' "nothing resembles it" no longer describes this list.
    assert_eq!(miss.reason, None);
    assert_eq!(listed_names(&items), ["Bank of Baroda CA"]);
    assert_eq!(items[0]["rule"], "lookup_key_equal");
}

/// #1085: a catalogue's name can be the stored name re-cased or reformatted.
/// Re-cased still resolves; reformatted asks, with the catalogue's spelling
/// offered.
#[test]
fn a_re_cased_name_resolves_and_a_reformatted_one_asks() {
    assert_eq!(
        resolve_ledger_or_refuse(["Round Off"].into_iter(), "ROUND OFF", Redaction::None)
            .unwrap()
            .name(),
        "Round Off"
    );
    let (code, _, items) = refusal(&["N.S.C.", "Cash"], "NSC", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert!(listed_names(&items).contains(&"N.S.C.".to_string()));
}

/// Under masking, a retyped masked name that no longer resolves is still
/// refused as masked, with its own next step, not as an unknown name.
/// Mutant killed: the masked-form guard only on the resolved path.
#[test]
fn a_retyped_masked_name_is_refused_as_masked_when_nothing_resolves() {
    let failure = resolve_ledger_or_refuse(
        ["RARS", "Ramesh Traders"].into_iter(),
        "Ra-rs",
        Redaction::MaskParties,
    )
    .unwrap_err();
    assert_eq!(failure.code, "ledger_name_masked");
}

/// The binding rules cannot search a name holding a control character: the
/// ledger the old key read is still offered, and the count is a floor, since
/// nothing else was searched.
/// Mutant killed: offering the loose ledgers only after a `none` or `listed`.
#[test]
fn a_search_that_cannot_run_still_offers_the_ledger_it_used_to_read() {
    let (code, miss, items) = refusal(&["Cash", "Bank"], "Cash\t", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(
        (miss.listing, miss.found, miss.found_is_lower_bound),
        (Listing::Listed, 1, true)
    );
    assert_eq!(listed_names(&items), ["Cash"]);
    assert_eq!(items[0]["rule"], "lookup_key_equal");
}

/// A no-break space, a figure space, a narrow no-break space and an
/// ideographic space are not spaces to the spelling key, which folds only the
/// ASCII space: a name holding one never resolves to the ledger it looks like.
/// That ledger is listed as a candidate instead, for the person to choose
/// (#1095). A name copied from a document or a web page can carry these.
/// Mutant killed: `ledger_spelling_key` splitting on each of them.
#[test]
fn a_name_holding_a_unicode_space_asks_rather_than_resolves() {
    for space in ['\u{a0}', '\u{2007}', '\u{202f}', '\u{3000}'] {
        let requested = format!("Cash{space}Book");
        let (code, miss, items) = refusal(&["Cash Book", "Bank"], &requested, Redaction::None);
        let space = format!("U+{:04X}", space as u32);
        assert_eq!(code, "ledger_not_found", "{space}");
        assert_eq!(
            (miss.listing, miss.found, miss.found_is_lower_bound),
            (Listing::Listed, 1, false),
            "{space}"
        );
        assert_eq!(listed_names(&items), ["Cash Book"], "{space}");
        assert_eq!(items[0]["rule"], "normalized_equal", "{space}");
    }
}

/// A book holding a CR LF twin (9.4e) is still searched by the binding rules,
/// which list both twins themselves; the loose ledgers add no second copy.
#[test]
fn a_book_with_a_cr_lf_twin_lists_each_twin_once() {
    let (code, miss, items) = refusal(
        &["LF708 C", "lf708 c\r\n", "Cash"],
        "LF708-C",
        Redaction::None,
    );
    assert_eq!(code, "ledger_not_found");
    assert_eq!((miss.listing, miss.found), (Listing::Listed, 2));
    assert_eq!(listed_names(&items), ["LF708 C", "lf708 c\r\n"]);
}

/// A withheld family is still counted, and the ledger the old key read is
/// listed beside the count: `truncated`, with the family's count as a floor.
/// Mutant killed: leaving a withheld result as it was.
#[test]
fn a_withheld_family_still_offers_the_ledger_it_used_to_read() {
    let mut names = (1..=30)
        .map(|number| format!("DN Party 0{number:02}"))
        .collect::<Vec<_>>();
    names.push("DN.Party 0".to_string());
    let refs = names.iter().map(String::as_str).collect::<Vec<_>>();
    let (code, miss, items) = refusal(&refs, "DN Party 0", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(
        (miss.listing, miss.found, miss.found_is_lower_bound),
        (Listing::Truncated, 30, true)
    );
    assert_eq!(listed_names(&items), ["DN.Party 0"]);
}

/// A full list makes room for the ledger the old key read, listed first so a
/// small response budget does not cut it either: the list stays at the cap
/// and says `truncated`. Mutants killed: no room made; listed last.
#[test]
fn a_full_list_makes_room_for_the_ledger_it_used_to_read() {
    let mut names = (1..=25)
        .map(|number| format!("Q Zork {number:02}"))
        .collect::<Vec<_>>();
    names.push("QZork".to_string());
    let refs = names.iter().map(String::as_str).collect::<Vec<_>>();
    let (code, miss, items) = refusal(&refs, "Q Zork", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!((miss.listing, items.len()), (Listing::Truncated, 25));
    assert_eq!(miss.found, 26);
    assert_eq!(listed_names(&items)[0], "QZork");
    assert_eq!(items[0]["rule"], "lookup_key_equal");
}

/// A name the catalogue lists twice is one ledger, not a clash of one.
/// Mutant killed: not de-duplicating the names.
#[test]
fn a_name_listed_twice_resolves_as_one_ledger() {
    assert_eq!(
        resolve_ledger_or_refuse(["Cash", "Cash"].into_iter(), "cash", Redaction::None)
            .unwrap()
            .name(),
        "Cash"
    );
}

/// A request with no letters or digits is no ledger's masked form.
/// Mutant killed: dropping the empty-key check from the masked-form guard.
#[test]
fn a_request_with_no_letters_is_not_a_masked_name() {
    let failure = resolve_ledger_or_refuse(["Ab", "Cash"].into_iter(), "-", Redaction::MaskParties)
        .unwrap_err();
    assert_eq!(failure.code, "ledger_not_found");
}

/// `similar_ledgers` is bounded like a candidate list, with a total.
/// Mutant killed: listing every twin.
#[test]
fn similar_ledgers_are_bounded_and_counted() {
    let twins = (1..=30)
        .map(|spaces| format!("Twin{}A", " ".repeat(spaces)))
        .collect::<Vec<_>>();
    let mut names = twins.iter().map(String::as_str).collect::<Vec<_>>();
    names.push("Cash");
    let found = resolve_ledger_or_refuse(names.into_iter(), "Twin A", Redaction::None).unwrap();
    let echo = found.to_json(Redaction::None);
    assert_eq!(echo["similar_ledgers"].as_array().unwrap().len(), 25);
    assert_eq!(echo["similar_ledgers_total"], 29);
}

/// Under masking an exact answer names no similar ledger and counts none, as
/// a masked refusal counts none. Mutant killed: twins listed under masking.
#[test]
fn a_masked_answer_carries_no_similar_ledgers() {
    let found = resolve_ledger_or_refuse(
        ["LF708 A", "LF708 A\r\n", "Cash"].into_iter(),
        "LF708 A",
        Redaction::MaskParties,
    )
    .unwrap();
    let echo = found.to_json(Redaction::MaskParties);
    assert_eq!(echo["matched"], "exact");
    assert!(echo.get("similar_ledgers").is_none(), "{echo}");
    assert!(echo.get("similar_ledgers_total").is_none(), "{echo}");
}

/// More ledgers share the old key than a list holds: the first twenty-five
/// are listed and the whole count is kept. Mutant killed: counting after
/// the cap.
#[test]
fn more_loose_matches_than_the_cap_are_counted_and_cut() {
    let names = (0..30_u32)
        .map(|pattern| {
            "dupxyz"
                .chars()
                .enumerate()
                .map(|(index, letter)| {
                    if index > 0 && pattern >> (index - 1) & 1 == 1 {
                        format!(".{letter}")
                    } else {
                        letter.to_string()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    let refs = names.iter().map(String::as_str).collect::<Vec<_>>();
    let (code, miss, items) = refusal(&refs, "DUP XYZ", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(miss.listing, Listing::Truncated);
    assert_eq!(items.len(), 25);
    assert!(miss.found >= 30, "{}", miss.found);
}

/// The binding rules' list was already cut: the ledger the old key read is
/// listed first, and the count stays theirs, as a floor, since it may already
/// include it. Mutant killed: adding to a cut list's count.
#[test]
fn a_cut_list_keeps_its_count_as_a_floor_and_lists_the_old_match_first() {
    let mut names = (1..=300)
        .map(|number| format!("Filler {number:03}"))
        .collect::<Vec<_>>();
    names.extend((1..=26).map(|number| format!("Zork Kappa {number:02}")));
    names.push("KappaZork".to_string());
    let refs = names.iter().map(String::as_str).collect::<Vec<_>>();
    let (code, miss, items) = refusal(&refs, "Kappa Zork", Redaction::None);
    assert_eq!(code, "ledger_not_found");
    assert_eq!(miss.listing, Listing::Truncated);
    assert!(miss.found_is_lower_bound);
    assert_eq!(listed_names(&items)[0], "KappaZork");
}
