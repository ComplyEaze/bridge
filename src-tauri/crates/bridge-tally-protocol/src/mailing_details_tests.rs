use super::*;

fn raw(date: Option<&str>, pin: Option<&str>) -> RawMailingDetailsEntry {
    RawMailingDetailsEntry {
        applicable_from: date.map(str::to_string),
        mailing_name: Some("Party".to_string()),
        address_lines: vec!["1 Test Street".to_string(), "Test Area".to_string()],
        state: Some("Karnataka".to_string()),
        country: Some("India".to_string()),
        pin_code: pin.map(str::to_string),
        repeated_field: false,
        other_content_skipped: false,
        has_content: true,
    }
}

fn day(text: &str) -> TallyDate {
    TallyDate::parse(text).unwrap()
}

fn entry(date: &str, pin: Option<&str>) -> MailingDetailsEntry {
    MailingDetailsEntry {
        applicable_from: day(date),
        mailing_name: Some("Party".to_string()),
        address_lines: vec!["1 Test Street".to_string(), "Test Area".to_string()],
        state: Some("Karnataka".to_string()),
        country: Some("India".to_string()),
        pin_code: pin.map(str::to_string),
        other_content_skipped: false,
    }
}

fn unreadable(raw_entries: Vec<RawMailingDetailsEntry>) -> Option<MailingDetailsDefect> {
    match MailingDetailsHistory::from_raw(raw_entries) {
        MailingDetailsHistory::Unreadable { defect } => Some(defect),
        _ => None,
    }
}

#[test]
fn a_placeholder_is_an_observed_empty_history_not_an_unobserved_one() {
    assert_eq!(
        MailingDetailsHistory::from_raw(vec![RawMailingDetailsEntry::default()]),
        MailingDetailsHistory::Entries { entries: vec![] }
    );
    assert_eq!(
        MailingDetailsHistory::from_raw(vec![]),
        MailingDetailsHistory::Entries { entries: vec![] }
    );
    assert_ne!(
        MailingDetailsHistory::from_raw(vec![]),
        MailingDetailsHistory::NotObserved
    );
    assert_eq!(
        MailingDetailsHistory::default(),
        MailingDetailsHistory::NotObserved
    );
}

#[test]
fn a_placeholder_is_skipped_beside_a_real_entry() {
    assert_eq!(
        MailingDetailsHistory::from_raw(vec![
            RawMailingDetailsEntry::default(),
            raw(Some("20260401"), Some("560001")),
        ]),
        MailingDetailsHistory::Entries {
            entries: vec![entry("20260401", Some("560001"))]
        }
    );
}

#[test]
fn an_entry_with_content_but_no_date_is_unreadable() {
    assert_eq!(
        unreadable(vec![raw(None, Some("560001"))]),
        Some(MailingDetailsDefect::EntryWithoutDate)
    );
    // A child was present, even though every value read as empty.
    assert_eq!(
        unreadable(vec![RawMailingDetailsEntry {
            has_content: true,
            ..RawMailingDetailsEntry::default()
        }]),
        Some(MailingDetailsDefect::EntryWithoutDate),
        "a child that read as empty is content, not a placeholder"
    );
    // Values without the content flag are still content.
    assert_eq!(
        unreadable(vec![RawMailingDetailsEntry {
            pin_code: Some("560001".to_string()),
            ..RawMailingDetailsEntry::default()
        }]),
        Some(MailingDetailsDefect::EntryWithoutDate)
    );
    assert_eq!(
        unreadable(vec![RawMailingDetailsEntry {
            address_lines: vec!["1 Test Street".to_string()],
            ..RawMailingDetailsEntry::default()
        }]),
        Some(MailingDetailsDefect::EntryWithoutDate)
    );
}

#[test]
fn a_date_that_is_not_a_calendar_date_is_unreadable() {
    assert_eq!(
        unreadable(vec![raw(Some("2026-04-01"), None)]),
        Some(MailingDetailsDefect::DateInvalid)
    );
    assert_eq!(
        unreadable(vec![raw(Some("20260231"), None)]),
        Some(MailingDetailsDefect::DateInvalid),
        "eight digits that are not a calendar date"
    );
}

#[test]
fn a_repeated_field_is_unreadable_even_when_nothing_else_was_read() {
    assert_eq!(
        unreadable(vec![RawMailingDetailsEntry {
            repeated_field: true,
            ..RawMailingDetailsEntry::default()
        }]),
        Some(MailingDetailsDefect::EntryRepeatsAField)
    );
    assert_eq!(
        unreadable(vec![RawMailingDetailsEntry {
            repeated_field: true,
            ..raw(Some("20260401"), None)
        }]),
        Some(MailingDetailsDefect::EntryRepeatsAField)
    );
}

#[test]
fn two_entries_on_one_date_that_differ_are_unreadable_and_identical_ones_are_kept_once() {
    assert_eq!(
        unreadable(vec![
            raw(Some("20260401"), Some("560001")),
            raw(Some("20260401"), Some("560002")),
        ]),
        Some(MailingDetailsDefect::ConflictingEntriesOnOneDate)
    );
    let mut other_lines = raw(Some("20260401"), Some("560001"));
    other_lines.address_lines.reverse();
    assert_eq!(
        unreadable(vec![raw(Some("20260401"), Some("560001")), other_lines]),
        Some(MailingDetailsDefect::ConflictingEntriesOnOneDate),
        "the same lines in another order are another address"
    );
    assert_eq!(
        MailingDetailsHistory::from_raw(vec![
            raw(Some("20260401"), Some("560001")),
            raw(Some("20260401"), Some("560001")),
        ]),
        MailingDetailsHistory::Entries {
            entries: vec![entry("20260401", Some("560001"))]
        }
    );
}

#[test]
fn entries_are_sorted_by_date_whatever_order_tally_sent_them() {
    assert_eq!(
        MailingDetailsHistory::from_raw(vec![
            raw(Some("20260701"), Some("560002")),
            raw(Some("20260401"), Some("560001")),
        ]),
        MailingDetailsHistory::Entries {
            entries: vec![
                entry("20260401", Some("560001")),
                entry("20260701", Some("560002")),
            ]
        }
    );
}

#[test]
fn the_entry_in_force_is_the_latest_on_or_before_the_date_and_none_before_the_first() {
    let history = MailingDetailsHistory::from_raw(vec![
        raw(Some("20260701"), Some("560002")),
        raw(Some("20260401"), Some("560001")),
        raw(Some("20261001"), Some("560003")),
    ]);
    let pin = |as_of| {
        history
            .in_force(&day(as_of))
            .and_then(|e| e.pin_code.as_deref())
    };
    assert_eq!(pin("20260331"), None, "before the first date");
    assert_eq!(pin("20260401"), Some("560001"), "on the first date");
    assert_eq!(pin("20260630"), Some("560001"));
    assert_eq!(pin("20260701"), Some("560002"), "on a later entry's date");
    assert_eq!(pin("20260930"), Some("560002"));
    assert_eq!(
        pin("20270331"),
        Some("560003"),
        "the latest replaces the rest"
    );
}

#[test]
fn nothing_is_in_force_from_an_empty_unobserved_or_unreadable_history() {
    assert_eq!(
        MailingDetailsHistory::Entries { entries: vec![] }.in_force(&day("20260831")),
        None
    );
    assert_eq!(
        MailingDetailsHistory::NotObserved.in_force(&day("20260831")),
        None
    );
    assert_eq!(
        MailingDetailsHistory::Unreadable {
            defect: MailingDetailsDefect::DateInvalid
        }
        .in_force(&day("20260831")),
        None
    );
}

#[test]
fn address_lines_keep_the_order_tally_sent() {
    let MailingDetailsHistory::Entries { entries } =
        MailingDetailsHistory::from_raw(vec![raw(Some("20260401"), None)])
    else {
        panic!("a valid history");
    };
    assert_eq!(entries[0].address_lines, ["1 Test Street", "Test Area"]);
}

#[test]
fn the_history_serialises_with_its_observation_tag() {
    let value = serde_json::to_value(MailingDetailsHistory::from_raw(vec![raw(
        Some("20260401"),
        Some("560001"),
    )]))
    .expect("serialises");
    assert_eq!(value["observation"], "entries");
    assert_eq!(value["entries"][0]["pin_code"], "560001");
    assert_eq!(
        serde_json::to_value(MailingDetailsHistory::NotObserved).expect("serialises")
            ["observation"],
        "not_observed"
    );
    let defect = serde_json::to_value(MailingDetailsHistory::Unreadable {
        defect: MailingDetailsDefect::EntryWithoutDate,
    })
    .expect("serialises");
    assert_eq!(defect["observation"], "unreadable");
    assert_eq!(defect["defect"], "entry_without_date");
}

#[test]
fn a_defect_names_itself_as_it_serialises() {
    for defect in [
        MailingDetailsDefect::EntryWithoutDate,
        MailingDetailsDefect::DateInvalid,
        MailingDetailsDefect::EntryRepeatsAField,
        MailingDetailsDefect::ConflictingEntriesOnOneDate,
    ] {
        assert_eq!(
            serde_json::to_value(defect).expect("serialises"),
            defect.as_str()
        );
    }
}

#[test]
fn skipped_content_is_carried_to_the_entry_and_is_part_of_what_two_entries_must_agree_on() {
    let mut skipping = raw(Some("20260401"), Some("560001"));
    skipping.other_content_skipped = true;
    let MailingDetailsHistory::Entries { entries } =
        MailingDetailsHistory::from_raw(vec![skipping.clone()])
    else {
        panic!("one entry reads");
    };
    assert!(entries[0].other_content_skipped);
    // The same date said once with content unread and once without is a conflict,
    // not a duplicate: the two readings do not agree on what Tally holds.
    assert_eq!(
        unreadable(vec![skipping, raw(Some("20260401"), Some("560001"))]),
        Some(MailingDetailsDefect::ConflictingEntriesOnOneDate)
    );
}
