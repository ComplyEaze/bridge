//! The texts a person sees when a grouping-decision save is refused or the
//! store fails (#737).
use super::*;
use crate::db::tally_mirror::MirrorError;
use crate::reports::schedule_iii::{
    Derivation, DerivedOutcome, FinancialYear, LedgerGuid, ScheduleIIIError, Undetermined,
};

fn missing(index: usize) -> SeenMismatch {
    SeenMismatch::LedgerMissing {
        ledger: LedgerGuid::new(&format!("ledger-{index:02}")).unwrap(),
    }
}

fn moved(name: &str) -> SeenMismatch {
    let derivation = |group: &str| Derivation {
        outcome: DerivedOutcome::Undetermined(Undetermined::UnmappedReservedGroup(
            group.to_string(),
        )),
        ancestry: vec![group.to_string()],
    };
    SeenMismatch::Moved {
        ledger: LedgerGuid::new("ledger-moved").unwrap(),
        ledger_name: name.to_string(),
        seen: derivation("seen group"),
        now: derivation("new group"),
    }
}

#[test]
fn a_moved_read_names_each_changed_ledger_up_to_ten_and_always_counts_them() {
    let few = grouping_refusal_error(GroupingRequestRefusal::Moved(vec![
        missing(1),
        moved("Synthetic Rent"),
    ]));
    assert_eq!(few.code, "grouping_read_changed_since_export");
    assert_eq!(
        few.message,
        "The books changed after the export, so nothing was saved. 2 changed ledger(s): \
         ledger-01 (no longer in the books); Synthetic Rent."
    );

    let ten = grouping_refusal_error(GroupingRequestRefusal::Moved(
        (1..=MISMATCHES_NAMED).map(missing).collect(),
    ));
    assert!(ten.message.contains("10 changed ledger(s)"));
    assert!(ten.message.contains("ledger-10 (no longer in the books)."));
    assert!(!ten.message.contains("and more"));

    let eleven = grouping_refusal_error(GroupingRequestRefusal::Moved(
        (1..=MISMATCHES_NAMED + 1).map(missing).collect(),
    ));
    assert!(eleven.message.contains("11 changed ledger(s)"));
    assert!(eleven
        .message
        .ends_with("ledger-10 (no longer in the books); and more."));
    assert!(!eleven.message.contains("ledger-11"));
}

#[test]
fn a_year_change_names_both_years_and_an_invalid_field_keeps_its_code() {
    let year = grouping_refusal_error(GroupingRequestRefusal::YearChanged {
        seen: FinancialYear::beginning_in(2025),
        now: FinancialYear::beginning_in(2026),
    });
    assert_eq!(year.code, "grouping_year_changed_since_export");
    assert_eq!(
        year.message,
        "The save read the books as of a date in the financial year beginning April 2026, \
         and the export it follows was made for the year beginning April 2025, so nothing was saved."
    );

    let invalid = grouping_refusal_error(GroupingRequestRefusal::Invalid("grouping_reason_empty"));
    assert_eq!(invalid.code, "grouping_reason_empty");
    assert!(invalid.message.ends_with("Nothing was saved."));
    for refusal in [&year, &invalid] {
        assert!(!refusal.local_state_changed);
    }
}

#[test]
fn each_store_failure_has_its_own_code() {
    let code = |error| grouping_store_error(error).code;
    assert_eq!(
        code(GroupingStoreError::Refused("grouping_event_target_unknown")),
        "grouping_event_target_unknown"
    );
    assert_eq!(
        code(GroupingStoreError::Unreadable),
        "grouping_decisions_unreadable"
    );
    assert_eq!(
        code(GroupingStoreError::Invalid(
            ScheduleIIIError::DecisionRepeated
        )),
        "grouping_decisions_unreadable"
    );
    assert_eq!(
        code(GroupingStoreError::Mirror(MirrorError::InvalidInput(
            "synthetic"
        ))),
        "grouping_store_failed"
    );
}

#[test]
fn the_register_view_shows_a_stored_company_name_by_the_one_rule() {
    // #2449: the stored name keeps a zero-width space as given; the view writes it as its code.
    let stored = "Synthetic\u{200b}Traders";
    let record = GroupingEventRecord {
        sequence: 1,
        id: crate::db::grouping_decisions::GroupingEventId::parse(&uuid::Uuid::nil().to_string())
            .expect("synthetic id"),
        event: RecordedEvent::Review {
            target: crate::db::grouping_decisions::GroupingEventId::parse(
                &uuid::Uuid::nil().to_string(),
            )
            .expect("synthetic target"),
        },
        company_display_name: stored.to_string(),
        year: FinancialYear::beginning_in(2025),
        ledger: LedgerGuid::new("synthetic-ledger").expect("synthetic ledger"),
        ledger_name: "Synthetic Ledger".to_string(),
        declared_by: DeclaredPerson::new("Synthetic Preparer").expect("synthetic person"),
        recorded_at_unix_ms: 0,
    };
    let view = serde_json::to_value(event_view(record)).expect("view serializes");
    assert_eq!(view["company_display_name"], "Synthetic\\u{200b}Traders");
}
