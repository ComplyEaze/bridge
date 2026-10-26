use super::*;

fn entry(date: Option<&str>, code: Option<&str>) -> RawHsnDetailsEntry {
    RawHsnDetailsEntry {
        applicable_from: date.map(str::to_string),
        hsn_code: code.map(str::to_string),
        description: Some("Disodium Carbonate, Dense".to_string()),
        source: Some("Specify Details Here".to_string()),
        repeated_field: false,
        skipped: false,
    }
}

fn defect(raw: Vec<RawHsnDetailsEntry>) -> Option<HsnDetailsDefect> {
    match HsnDetailsObservation::from_raw(raw) {
        HsnDetailsObservation::Unreadable { defect } => Some(defect),
        _ => None,
    }
}

fn entries(raw: Vec<RawHsnDetailsEntry>) -> Vec<HsnDetailsEntry> {
    match HsnDetailsObservation::from_raw(raw) {
        HsnDetailsObservation::Entries { entries } => entries,
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_placeholder_is_an_empty_list_and_no_element_at_all_is_not_observed() {
    assert_eq!(
        HsnDetailsObservation::from_raw(vec![RawHsnDetailsEntry::default()]),
        HsnDetailsObservation::Entries { entries: vec![] }
    );
    assert_eq!(
        HsnDetailsObservation::from_raw(vec![]),
        HsnDetailsObservation::Entries { entries: vec![] }
    );
    assert_eq!(
        HsnDetailsObservation::default(),
        HsnDetailsObservation::NotObserved
    );
}

#[test]
fn a_code_is_kept_exactly_as_sent_and_an_absent_code_is_not_an_empty_one() {
    let kept = entries(vec![
        entry(Some("20250401"), Some(" 2836 2010 ")),
        entry(Some("20250922"), Some("")),
        entry(Some("20251101"), None),
    ]);
    assert_eq!(kept[0].hsn_code.as_deref(), Some(" 2836 2010 "));
    assert_eq!(kept[1].hsn_code.as_deref(), Some(""));
    assert_eq!(kept[2].hsn_code, None);
}

#[test]
fn entries_are_sorted_by_date_and_an_exact_repeat_is_kept_once() {
    let kept = entries(vec![
        entry(Some("20250922"), Some("28362090")),
        entry(Some("20250401"), Some("28362010")),
        entry(Some("20250401"), Some("28362010")),
    ]);
    let dates = kept
        .iter()
        .map(|entry| entry.applicable_from.as_str())
        .collect::<Vec<_>>();
    assert_eq!(dates, ["20250401", "20250922"]);
}

#[test]
fn each_defect_fails_the_item_with_its_own_defect() {
    assert_eq!(
        defect(vec![entry(None, Some("2836"))]),
        Some(HsnDetailsDefect::EntryWithoutDate)
    );
    assert_eq!(
        defect(vec![entry(Some("20250230"), Some("2836"))]),
        Some(HsnDetailsDefect::DateInvalid)
    );
    assert_eq!(
        defect(vec![entry(Some("1-Apr-2025"), Some("2836"))]),
        Some(HsnDetailsDefect::DateInvalid)
    );
    let mut repeated = entry(Some("20250401"), Some("2836"));
    repeated.repeated_field = true;
    assert_eq!(
        defect(vec![repeated]),
        Some(HsnDetailsDefect::EntryRepeatsAField)
    );
    assert_eq!(
        defect(vec![
            entry(Some("20250401"), Some("2836")),
            entry(Some("20250401"), Some("2837")),
        ]),
        Some(HsnDetailsDefect::ConflictingEntriesOnOneDate)
    );
    let unrecognised = RawHsnDetailsEntry {
        skipped: true,
        ..RawHsnDetailsEntry::default()
    };
    assert_eq!(
        defect(vec![unrecognised]),
        Some(HsnDetailsDefect::UnrecognisedContent)
    );
}

#[test]
fn skipped_content_beside_recognised_fields_is_kept_and_flagged() {
    let mut raw = entry(Some("20250401"), Some("2836"));
    raw.skipped = true;
    let kept = entries(vec![raw]);
    assert!(kept[0].other_content_skipped);
}

#[test]
fn tallys_own_first_row_has_a_date_and_a_source_and_no_code() {
    let own = RawHsnDetailsEntry {
        applicable_from: Some("20250401".to_string()),
        source: Some("As per Company/Stock Group".to_string()),
        ..RawHsnDetailsEntry::default()
    };
    let kept = entries(vec![own]);
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].hsn_code, None);
    assert_eq!(kept[0].description, None);
    assert_eq!(
        kept[0].source.as_deref(),
        Some("As per Company/Stock Group")
    );
}

#[test]
fn a_date_with_padding_is_read_trimmed_and_a_field_alone_is_an_undated_entry() {
    let kept = entries(vec![entry(Some(" 20250401 "), Some("2836"))]);
    assert_eq!(kept[0].applicable_from, "20250401");
    // Each recognised field without a date is content, not a placeholder.
    for raw in [
        RawHsnDetailsEntry {
            hsn_code: Some("2836".to_string()),
            ..RawHsnDetailsEntry::default()
        },
        RawHsnDetailsEntry {
            description: Some("Other".to_string()),
            ..RawHsnDetailsEntry::default()
        },
        RawHsnDetailsEntry {
            source: Some("As per Company/Stock Group".to_string()),
            ..RawHsnDetailsEntry::default()
        },
    ] {
        assert_eq!(defect(vec![raw]), Some(HsnDetailsDefect::EntryWithoutDate));
    }
}
