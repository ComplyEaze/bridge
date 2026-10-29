//! Every name and GUID here is fabricated from placeholder text; none is
//! evidence about any Tally instance. They establish how the index applies the
//! folds this crate already has, and that it refuses on a match under any of
//! them.

use super::*;

fn master(kind: MasterKind, guid: &str, alter_id: u64, names: &[&str]) -> IndexedMaster {
    IndexedMaster {
        kind,
        guid: guid.to_string(),
        alter_id,
        names: names.iter().map(|name| (*name).to_string()).collect(),
    }
}

fn ledger(guid: &str, names: &[&str]) -> IndexedMaster {
    master(MasterKind::Ledger, guid, 1, names)
}

fn index_of(masters: Vec<IndexedMaster>) -> MasterNameIndex {
    MasterNameIndex::new(masters).expect("fabricated masters are valid")
}

fn single(name: &str) -> MasterNameIndex {
    index_of(vec![ledger("guid-1", &[name])])
}

fn found(index: &MasterNameIndex, candidate: &str) -> Vec<NameCollision> {
    index
        .collisions(candidate)
        .expect("fabricated candidate is not blank")
}

fn hits(index: &MasterNameIndex, candidate: &str) -> usize {
    found(index, candidate).len()
}

#[test]
fn an_exact_name_collides_and_reports_the_stored_name_and_guid() {
    let index = single("Alpha Traders");
    assert_eq!(
        found(&index, "Alpha Traders"),
        vec![NameCollision {
            kind: MasterKind::Ledger,
            via: Via::Name,
            guid: "guid-1".to_string(),
            name: "Alpha Traders".to_string(),
        }]
    );
}

#[test]
fn an_unrelated_name_and_an_empty_book_have_no_collision() {
    assert!(found(&single("Alpha Traders"), "Beta Traders").is_empty());
    assert!(found(&index_of(Vec::new()), "Alpha Traders").is_empty());
}

#[test]
fn case_collides_in_both_directions() {
    assert_eq!(hits(&single("Alpha Traders"), "ALPHA TRADERS"), 1);
    assert_eq!(hits(&single("ALPHA TRADERS"), "alpha traders"), 1);
}

#[test]
fn a_double_space_collides_in_both_directions() {
    assert_eq!(hits(&single("Alpha Traders"), "Alpha  Traders"), 1);
    assert_eq!(hits(&single("Alpha  Traders"), "Alpha Traders"), 1);
}

#[test]
fn leading_and_trailing_space_collide() {
    assert_eq!(hits(&single("Alpha Traders"), "Alpha Traders "), 1);
    assert_eq!(hits(&single("Alpha Traders"), "  Alpha Traders"), 1);
    assert_eq!(hits(&single("Alpha Traders "), "Alpha Traders"), 1);
}

#[test]
fn a_hyphen_and_a_space_collide_in_both_directions() {
    assert_eq!(hits(&single("Alpha-Traders"), "Alpha Traders"), 1);
    assert_eq!(hits(&single("Alpha Traders"), "Alpha-Traders"), 1);
}

#[test]
fn a_slash_and_a_space_collide_in_both_directions() {
    assert_eq!(hits(&single("Alpha/Traders"), "Alpha Traders"), 1);
    assert_eq!(hits(&single("Alpha Traders"), "Alpha/Traders"), 1);
}

#[test]
fn a_tab_or_newline_between_words_collides_with_a_space() {
    assert_eq!(hits(&single("Alpha Traders"), "Alpha\tTraders"), 1);
    assert_eq!(hits(&single("Alpha Traders"), "Alpha\r\nTraders"), 1);
}

#[test]
fn a_hyphen_surrounded_by_spaces_collides_with_a_single_space() {
    // `X - Y` against `X Y` needs the hyphen step and a collapsed run.
    assert_eq!(hits(&single("Alpha - Traders"), "Alpha Traders"), 1);
    assert_eq!(hits(&single("Alpha Traders"), "Alpha - Traders"), 1);
}

#[test]
fn an_alias_hit_reports_alias_and_the_owning_masters_guid() {
    let index = index_of(vec![
        ledger("guid-1", &["Alpha Traders"]),
        ledger("guid-2", &["Beta Stores", "Beta Alias", "Beta-Other"]),
    ]);
    assert_eq!(
        found(&index, "beta alias"),
        vec![NameCollision {
            kind: MasterKind::Ledger,
            via: Via::Alias,
            guid: "guid-2".to_string(),
            name: "Beta Alias".to_string(),
        }]
    );
    // A later alias is also an alias, and the stored name still reads as Name.
    assert_eq!(found(&index, "Beta Other")[0].via, Via::Alias);
    assert_eq!(found(&index, "BETA STORES")[0].via, Via::Name);
}

#[test]
fn a_group_name_hit_reports_a_group() {
    let index = index_of(vec![
        ledger("guid-1", &["Alpha Traders"]),
        master(MasterKind::Group, "guid-2", 7, &["Sundry Debtors"]),
    ]);
    assert_eq!(
        found(&index, "sundry debtors"),
        vec![NameCollision {
            kind: MasterKind::Group,
            via: Via::Name,
            guid: "guid-2".to_string(),
            name: "Sundry Debtors".to_string(),
        }]
    );
}

#[test]
fn a_group_alias_hit_reports_a_group_alias() {
    let index = index_of(vec![master(
        MasterKind::Group,
        "guid-2",
        7,
        &["Sundry Debtors", "Debtors"],
    )]);
    let found = found(&index, "DEBTORS");
    assert_eq!(found.len(), 1);
    assert_eq!(
        (found[0].kind, found[0].via),
        (MasterKind::Group, Via::Alias)
    );
}

#[test]
fn nfc_and_nfd_spellings_of_an_accented_name_collide() {
    // The wide fold normalises to NFC, so the two spellings share a key. Tally
    // itself rejected NFD against NFC (§9.4d), but refusing is the safe side.
    let nfc = "Caf\u{e9} Traders";
    let nfd = "Cafe\u{301} Traders";
    assert_ne!(nfc, nfd);
    assert_eq!(hits(&single(nfc), nfd), 1);
    assert_eq!(hits(&single(nfd), nfc), 1);
}

#[test]
fn an_ampersand_and_the_word_and_do_not_collide() {
    // §9.4b: `AND` for `&` was rejected by Tally; no fold here treats them alike.
    assert_eq!(hits(&single("Ram & Sons"), "Ram and Sons"), 0);
    assert_eq!(hits(&single("Ram and Sons"), "Ram & Sons"), 0);
    assert_eq!(hits(&single("Ram & Sons"), "Ram AND Sons"), 0);
}

#[test]
fn an_added_or_missing_word_or_a_plural_does_not_collide() {
    let index = single("Ram & Sons Pvt Ltd");
    assert_eq!(hits(&index, "Ram & Sons"), 0);
    assert_eq!(hits(&index, "Ram & Son Pvt Ltd"), 0);
    assert_eq!(hits(&index, "Ram & Sons Pvt Ltd Two"), 0);
}

#[test]
fn an_underscore_collides_with_a_space_or_a_hyphen_conservatively() {
    // §9.4d: Tally rejected an underscore where the master has a space on the
    // voucher path, and the folds keep them apart; the letters-and-digits key
    // does not, so the index refuses. A false refusal is the safe direction.
    let underscore = "Alpha_Traders";
    assert_ne!(
        crate::master_binding::canonical_master_key(underscore),
        crate::master_binding::canonical_master_key("Alpha Traders")
    );
    assert_eq!(hits(&single("Alpha Traders"), underscore), 1);
    assert_eq!(hits(&single(underscore), "Alpha Traders"), 1);
    assert_eq!(hits(&single("Alpha-Traders"), underscore), 1);
}

#[test]
fn an_en_dash_collides_with_a_space_and_a_hyphen_through_the_wide_fold() {
    // §9.4d: Tally rejected an en dash where the master has a space, and the
    // narrow folds (`verified_fold`, `canonical_master_key`) keep them apart.
    // The wide fold (`comparison_key`) unifies Unicode dash variants into `-`,
    // which `master_identity_key` then reads as a space, so the index refuses.
    // That is deliberate here: a refusal on a name Tally would keep apart is
    // loud and safe, a miss is not.
    let en_dash = "Alpha\u{2013}Traders";
    assert_ne!(
        crate::master_binding::canonical_master_key(en_dash),
        crate::master_binding::canonical_master_key("Alpha Traders")
    );
    assert_eq!(hits(&single("Alpha Traders"), en_dash), 1);
    assert_eq!(hits(&single("Alpha-Traders"), en_dash), 1);
    assert_eq!(hits(&single(en_dash), "Alpha Traders"), 1);
}

#[test]
fn a_candidate_matching_two_masters_reports_both_in_input_order() {
    let index = index_of(vec![
        ledger("guid-1", &["Alpha-Traders"]),
        ledger("guid-2", &["Beta"]),
        master(MasterKind::Group, "guid-3", 3, &["Other", "alpha traders"]),
        ledger("guid-4", &["ALPHA TRADERS"]),
    ]);
    let found = found(&index, "Alpha Traders");
    assert_eq!(
        found
            .iter()
            .map(|c| (c.guid.as_str(), c.via, c.kind))
            .collect::<Vec<_>>(),
        vec![
            ("guid-1", Via::Name, MasterKind::Ledger),
            ("guid-3", Via::Alias, MasterKind::Group),
            ("guid-4", Via::Name, MasterKind::Ledger),
        ]
    );
}

#[test]
fn several_aliases_of_one_master_each_report_in_name_order() {
    let index = index_of(vec![ledger("guid-1", &["Alpha", "ALPHA", "alpha"])]);
    let found = found(&index, "Alpha");
    assert_eq!(
        found
            .iter()
            .map(|c| (c.via, c.name.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (Via::Name, "Alpha"),
            (Via::Alias, "ALPHA"),
            (Via::Alias, "alpha")
        ]
    );
}

#[test]
fn one_name_matching_under_several_folds_is_reported_once() {
    // `Alpha-Traders` matches `Alpha Traders` under the wide key, the narrow
    // key and the canonical key all at once.
    assert_eq!(hits(&single("Alpha Traders"), "alpha-traders"), 1);
}

#[test]
fn a_match_on_only_the_wide_folds_still_collides() {
    let keys = crate::master_binding::twin_fold_keys;
    let canonical = crate::master_binding::canonical_master_key;

    // NFC against NFD: the wide key and the both-folds key agree, the narrow
    // key and the canonical key (which do not normalise) do not.
    let nfd = "Cafe\u{301} Traders";
    let nfc = "Caf\u{e9} Traders";
    assert_eq!(keys(nfd)[0], keys(nfc)[0]);
    assert_ne!(keys(nfd)[1], keys(nfc)[1]);
    assert_eq!(keys(nfd)[2], keys(nfc)[2]);
    assert_ne!(canonical(nfd), canonical(nfc));
    assert_eq!(hits(&single(nfc), nfd), 1);

    // A slash beside an NFD spelling needs both steps: only the both-folds
    // key (position 2) agrees; the wide key keeps the slash and the narrow
    // key keeps the decomposed accent.
    let both = "Cafe\u{301}/Traders";
    let target = "Caf\u{e9} Traders";
    assert_ne!(keys(both)[0], keys(target)[0]);
    assert_ne!(keys(both)[1], keys(target)[1]);
    assert_eq!(keys(both)[2], keys(target)[2]);
    assert_ne!(canonical(both), canonical(target));
    assert_eq!(hits(&single(target), both), 1);
    assert_eq!(hits(&single(both), target), 1);
}

#[test]
fn a_non_ascii_capital_collides_with_its_lowercase() {
    // The narrow fold lowercases ASCII only, so position 1 differs here; the
    // wide fold and the canonical key lowercase by Unicode rules and agree.
    // (The canonical key adds no match the twin folds miss: see the property
    // test in `master_binding`.)
    let keys = crate::master_binding::twin_fold_keys;
    assert_ne!(keys("\u{c9}A")[1], keys("\u{e9}a")[1]);
    assert_eq!(hits(&single("\u{e9}a"), "\u{c9}A"), 1);
}

#[test]
fn a_candidate_that_folds_empty_collides_only_with_a_name_that_folds_empty() {
    assert_eq!(hits(&single("-"), "/"), 1);
    assert_eq!(hits(&single("Alpha"), "/"), 0);
}

#[test]
fn pre_image_maps_lowercased_guids_to_alter_ids_for_both_kinds() {
    let index = index_of(vec![
        master(MasterKind::Ledger, "ABCD-0001", 12, &["Alpha"]),
        master(MasterKind::Group, "abcd-0002", 99, &["Beta"]),
        master(MasterKind::Ledger, "Ef-03", 0, &["Gamma", "G"]),
    ]);
    let expected: BTreeMap<String, u64> = [
        ("abcd-0001".to_string(), 12),
        ("abcd-0002".to_string(), 99),
        ("ef-03".to_string(), 0),
    ]
    .into_iter()
    .collect();
    assert_eq!(index.pre_image(), expected);
    assert!(index_of(Vec::new()).pre_image().is_empty());
}

#[test]
fn an_empty_name_list_is_refused() {
    assert_eq!(
        MasterNameIndex::new(vec![ledger("guid-1", &["Alpha"]), ledger("guid-2", &[])])
            .unwrap_err(),
        MasterNameIndexError::NoNames { master: 1 }
    );
}

#[test]
fn an_empty_or_whitespace_only_name_is_refused_even_as_an_alias() {
    for blank in ["", " ", "\t", "  \r\n ", "\u{a0}"] {
        assert_eq!(
            MasterNameIndex::new(vec![ledger("guid-1", &[blank])]).unwrap_err(),
            MasterNameIndexError::BlankName {
                master: 0,
                position: 0
            },
            "name {blank:?}"
        );
        assert_eq!(
            MasterNameIndex::new(vec![ledger("guid-1", &["Alpha", "Beta", blank])]).unwrap_err(),
            MasterNameIndexError::BlankName {
                master: 0,
                position: 2
            },
            "alias {blank:?}"
        );
    }
}

#[test]
fn an_empty_or_blank_guid_is_refused() {
    for guid in ["", " ", "\t"] {
        assert_eq!(
            MasterNameIndex::new(vec![ledger("guid-1", &["Alpha"]), ledger(guid, &["Beta"])])
                .unwrap_err(),
            MasterNameIndexError::EmptyGuid { master: 1 },
            "guid {guid:?}"
        );
    }
}

#[test]
fn a_duplicate_guid_is_refused_across_a_ledger_and_a_group() {
    assert_eq!(
        MasterNameIndex::new(vec![
            ledger("guid-1", &["Alpha"]),
            master(MasterKind::Group, "guid-1", 2, &["Beta"]),
        ])
        .unwrap_err(),
        MasterNameIndexError::DuplicateGuid {
            guid: "guid-1".to_string()
        }
    );
}

#[test]
fn a_duplicate_guid_is_refused_within_one_kind() {
    assert!(matches!(
        MasterNameIndex::new(vec![
            ledger("guid-1", &["Alpha"]),
            ledger("guid-2", &["Beta"]),
            ledger("guid-1", &["Gamma"]),
        ]),
        Err(MasterNameIndexError::DuplicateGuid { .. })
    ));
}

#[test]
fn guids_differing_only_in_ascii_case_are_duplicates() {
    assert_eq!(
        MasterNameIndex::new(vec![
            ledger("AbCd-01", &["Alpha"]),
            master(MasterKind::Group, "aBcD-01", 2, &["Beta"]),
        ])
        .unwrap_err(),
        MasterNameIndexError::DuplicateGuid {
            guid: "abcd-01".to_string()
        }
    );
}

#[test]
fn a_blank_candidate_is_an_error_not_an_empty_answer() {
    let index = single("Alpha");
    for blank in ["", " ", "\t", "  \r\n ", "\u{a0}"] {
        assert_eq!(
            index.collisions(blank).unwrap_err(),
            MasterNameIndexError::BlankCandidate,
            "candidate {blank:?}"
        );
    }
}

#[test]
fn a_guid_with_surrounding_whitespace_is_refused_not_trimmed() {
    for guid in [" guid-1", "guid-1 ", "\tguid-1", "guid-1\n"] {
        assert_eq!(
            MasterNameIndex::new(vec![ledger("guid-0", &["Alpha"]), ledger(guid, &["Beta"])])
                .unwrap_err(),
            MasterNameIndexError::PaddedGuid { master: 1 },
            "guid {guid:?}"
        );
    }
}

#[test]
fn guids_differing_beyond_ascii_case_are_distinct() {
    // Only ASCII case is folded: these two are different GUID strings.
    assert!(MasterNameIndex::new(vec![
        ledger("g\u{e9}", &["Alpha"]),
        ledger("g\u{c9}", &["Beta"]),
    ])
    .is_ok());
}

#[test]
fn error_messages_do_not_echo_master_names() {
    let message = MasterNameIndex::new(vec![ledger("guid-1", &["Secret Client Name", " "])])
        .unwrap_err()
        .to_string();
    assert!(!message.contains("Secret"), "{message}");
}

/// Lowercasing and NFC do not commute: for these pairs the canonical key
/// (lowercase, no normalisation) is equal, while `absence_key` (NFC first,
/// then lowercase) differs, because the lowercase form composes and the
/// uppercase form has no precomposed letter. Only the canonical key catches
/// them, so the index must match on it too.
const LOWERCASE_NFC_PAIRS: &[(&str, &str)] = &[
    ("j\u{30c}", "J\u{30c}"),
    ("h\u{331}", "H\u{331}"),
    ("t\u{308}", "T\u{308}"),
    ("t\u{344}", "T\u{344}"),
    ("w\u{30a}", "W\u{30a}"),
    ("y\u{30a}", "Y\u{30a}"),
];

#[test]
fn a_letter_that_lowercases_and_composes_differently_still_collides() {
    use crate::master_binding::{absence_key, canonical_master_key};
    for (lower, upper) in LOWERCASE_NFC_PAIRS {
        let existing = format!("Alpha {lower}");
        let candidate = format!("ALPHA {upper}");
        assert_eq!(
            canonical_master_key(&existing),
            canonical_master_key(&candidate),
            "{lower:?} / {upper:?}"
        );
        assert_ne!(
            absence_key(&existing),
            absence_key(&candidate),
            "{lower:?} / {upper:?}: this pair is meant to need the canonical key"
        );
        assert_eq!(hits(&single(&existing), &candidate), 1, "{lower:?}");
        assert_eq!(hits(&single(&candidate), &existing), 1, "{upper:?}");
    }
}

#[test]
fn a_dotted_capital_i_with_a_low_mark_collides_with_its_lowercase_spelling() {
    use crate::master_binding::canonical_master_key;
    // U+0130 lowercases to i + U+0307. With a mark of combining class below
    // 230 (U+0316..U+0329) NFC reorders the mark ahead of the dot in one
    // spelling and not the other; only the canonical key sees them as equal.
    for mark in '\u{316}'..='\u{329}' {
        let existing = format!("Alpha \u{130}{mark}");
        let candidate = format!("alpha i\u{307}{mark}");
        let code = mark as u32;
        assert_eq!(
            canonical_master_key(&existing),
            canonical_master_key(&candidate),
            "U+{code:04X}"
        );
        assert_eq!(hits(&single(&existing), &candidate), 1, "U+{code:04X}");
        assert_eq!(hits(&single(&candidate), &existing), 1, "U+{code:04X}");
    }
}

#[test]
fn neither_the_canonical_key_nor_the_absence_key_subsumes_the_other() {
    use crate::master_binding::{absence_key, canonical_master_key};
    // Only the canonical key: lowercase and NFC do not commute.
    let (lower, upper) = ("Raj\u{30c}", "RAJ\u{30c}");
    assert_eq!(canonical_master_key(lower), canonical_master_key(upper));
    assert_ne!(absence_key(lower), absence_key(upper));
    assert_eq!(hits(&single(lower), upper), 1);

    // Only the absence key: NFC against NFD, and an en dash against a space.
    for (a, b) in [
        ("Caf\u{e9} Traders", "Cafe\u{301} Traders"),
        ("Alpha Traders", "Alpha\u{2013}Traders"),
    ] {
        assert_ne!(canonical_master_key(a), canonical_master_key(b), "{a:?}");
        assert_eq!(absence_key(a), absence_key(b), "{a:?}");
        assert_eq!(hits(&single(a), b), 1, "{a:?}");
    }
}

/// Tally folds a removed space and removed full stops when it resolves a
/// Create, so the index refuses on them; any other symbol (here brackets and
/// an underscore) is unmeasured and refused conservatively: a false "present"
/// is the safe answer. A changed letter or digit is still a
/// different name.
#[test]
fn a_name_differing_only_by_spaces_or_symbols_collides() {
    let index = single("Alpha Traders Pvt. Ltd.");
    for candidate in [
        "Alpha Traders Pvt Ltd",
        "AlphaTraders Pvt. Ltd.",
        "ALPHA_TRADERS (PVT) LTD",
    ] {
        assert_eq!(hits(&index, candidate), 1, "{candidate}");
    }
    for candidate in ["Alpha Trader Pvt Ltd", "Alpha Traders Pvt Ltd 2"] {
        assert_eq!(hits(&index, candidate), 0, "{candidate}");
    }
}

/// A letter that lower-casing leaves alone but upper-casing maps still meets
/// its ASCII spelling, so an existing name holding one is not missed.
#[test]
fn a_dotless_i_or_long_s_meets_its_ascii_spelling() {
    assert_eq!(hits(&single("K\u{131}ran Traders"), "KIRAN TRADERS"), 1);
    assert_eq!(hits(&single("A\u{17f}hok"), "Ashok"), 1);
}

/// A stored name holding Tally's illegal character reference (`&#4;`, read
/// as U+FFFD `#4;`) meets its plain spelling: the reference is dropped whole,
/// digit and all. A U+FFFD that opens no whole marker is only a symbol, and
/// the digits of a name stay.
#[test]
fn a_stored_reference_marker_meets_the_plain_spelling() {
    assert_eq!(
        hits(&single("\u{fffd}#4; Not Applicable"), "Not Applicable"),
        1
    );
    assert_eq!(hits(&single("Primary\u{fffd}#65533;"), "Primary"), 1);
    assert_eq!(hits(&single("Godown \u{fffd}# 4"), "Godown 4"), 1);
    assert_eq!(hits(&single("\u{fffd}#4; Branch 7"), "Branch 4"), 0);
    assert_eq!(hits(&single("\u{fffd}#4; Branch 7"), "Branch 7"), 1);
}
