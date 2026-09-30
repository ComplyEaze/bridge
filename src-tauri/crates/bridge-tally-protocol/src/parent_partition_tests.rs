use super::*;

const LIMITS: PartitionLimits = PartitionLimits {
    max_ledgers_per_part: 10,
    max_parents_per_part: 3,
    max_parts: 4,
};

fn rows(spec: &[(&str, u64)]) -> Vec<(String, String, Option<String>)> {
    let mut out = Vec::new();
    for (parent, count) in spec {
        for index in 0..*count {
            out.push((
                format!("{parent}-ledger-{index}"),
                format!("guid-{parent}-{index}"),
                Some((*parent).to_owned()),
            ));
        }
    }
    out
}

fn plan(
    rows: &[(String, String, Option<String>)],
    limits: PartitionLimits,
) -> Result<ParentPartition, ParentPartitionError> {
    ParentPartition::plan(
        rows.iter().map(|(name, guid, parent)| {
            (
                name.as_str(),
                guid.as_str(),
                ParentObservation::from(parent.as_deref()),
            )
        }),
        limits,
    )
}

fn parents_of(partition: &ParentPartition) -> Vec<Vec<&str>> {
    partition
        .parts()
        .iter()
        .map(|part| {
            part.parents()
                .iter()
                .map(ParentName::as_catalogue_text)
                .collect()
        })
        .collect()
}

#[test]
fn packs_largest_parent_first_into_the_first_part_with_room() {
    let partition = plan(&rows(&[("A", 6), ("B", 5), ("C", 4), ("D", 1)]), LIMITS).unwrap();
    assert_eq!(parents_of(&partition), vec![vec!["A", "C"], vec!["B", "D"]]);
    let counts = partition
        .parts()
        .iter()
        .map(ParentPart::ledger_count)
        .collect::<Vec<_>>();
    assert_eq!(counts, vec![10, 6]);
}

#[test]
fn plan_is_independent_of_input_order() {
    let mut forward = rows(&[("A", 6), ("B", 5), ("C", 4)]);
    let first = plan(&forward, LIMITS).unwrap();
    forward.reverse();
    let second = plan(&forward, LIMITS).unwrap();
    assert_eq!(first, second);
}

#[test]
fn a_part_never_exceeds_the_ledger_or_parent_limits() {
    let spec = (0..9)
        .map(|index| (format!("P{index}"), 1u64))
        .collect::<Vec<_>>();
    let spec_ref = spec
        .iter()
        .map(|(name, count)| (name.as_str(), *count))
        .collect::<Vec<_>>();
    let partition = plan(&rows(&spec_ref), LIMITS).unwrap();
    assert_eq!(partition.parts().len(), 3);
    for part in partition.parts() {
        assert!(part.parents().len() <= LIMITS.max_parents_per_part);
        assert!(part.ledger_count() <= LIMITS.max_ledgers_per_part);
    }
}

#[test]
fn a_parent_over_the_ledger_limit_is_refused_not_split() {
    let error = plan(&rows(&[("A", 11), ("B", 1)]), LIMITS).unwrap_err();
    assert_eq!(
        error,
        ParentPartitionError::ParentOverBudget { ledgers: 11 }
    );
    assert_eq!(error.safe_code(), "parent_over_budget");
}

#[test]
fn a_parent_exactly_at_the_ledger_limit_is_one_part() {
    let partition = plan(&rows(&[("A", 10)]), LIMITS).unwrap();
    assert_eq!(partition.parts().len(), 1);
}

#[test]
fn too_many_parts_is_refused() {
    let spec = (0..5)
        .map(|index| (format!("P{index}"), 10u64))
        .collect::<Vec<_>>();
    let spec_ref = spec
        .iter()
        .map(|(name, count)| (name.as_str(), *count))
        .collect::<Vec<_>>();
    assert_eq!(
        plan(&rows(&spec_ref), LIMITS).unwrap_err(),
        ParentPartitionError::TooManyParts { parts: 5 }
    );
}

#[test]
fn a_ledger_without_a_parent_is_refused() {
    for parent in [None, Some(String::new())] {
        let mut all = rows(&[("A", 1)]);
        all.push(("orphan".into(), "guid-orphan".into(), parent));
        assert_eq!(
            plan(&all, LIMITS).unwrap_err(),
            ParentPartitionError::LedgerWithoutParent
        );
    }
}

#[test]
fn ledgers_whose_parent_cannot_be_carried_are_counted_not_called_parentless() {
    let mut all = rows(&[("A", 1)]);
    let unsupported = ["u1", "u2", "u3"].map(|name| (name, format!("guid-{name}")));
    let mut observed = all
        .iter()
        .map(|(name, guid, parent)| {
            (
                name.as_str(),
                guid.as_str(),
                ParentObservation::from(parent.as_deref()),
            )
        })
        .collect::<Vec<_>>();
    for (name, guid) in &unsupported {
        observed.push((name, guid.as_str(), ParentObservation::Unsupported));
    }
    let error = ParentPartition::plan(observed, LIMITS).unwrap_err();
    assert_eq!(
        error,
        ParentPartitionError::ParentNameUnsupported { ledgers: 3 }
    );
    assert_eq!(error.safe_code(), "parent_name_unsupported");
    all.push(("orphan".into(), "guid-orphan".into(), None));
    assert_eq!(
        plan(&all, LIMITS).unwrap_err(),
        ParentPartitionError::LedgerWithoutParent
    );
}

#[test]
fn unsupported_named_parents_are_counted_across_the_whole_catalogue() {
    let mut all = rows(&[("A", 2)]);
    all.push(("t1".into(), "guid-t1".into(), Some("tab\there".into())));
    all.push(("t2".into(), "guid-t2".into(), Some("tab\there".into())));
    all.push(("q1".into(), "guid-q1".into(), Some("a\"b".into())));
    assert_eq!(
        plan(&all, LIMITS).unwrap_err(),
        ParentPartitionError::ParentNameUnsupported { ledgers: 3 }
    );
}

#[test]
fn a_repeated_guid_is_refused_ignoring_ascii_case() {
    let mut all = rows(&[("A", 1)]);
    all.push(("other".into(), "GUID-A-0".into(), Some("A".into())));
    assert_eq!(
        plan(&all, LIMITS).unwrap_err(),
        ParentPartitionError::DuplicateLedgerIdentity
    );
}

#[test]
fn parent_names_that_cannot_sit_in_a_literal_are_refused() {
    for name in [
        "Sundry \"Debtors\"",
        "a\"b",
        "line\nbreak",
        "tab\there",
        "nul\u{0}",
        "  ",
        "\u{fffd}#5; Primary",
        "\u{fffd}#4; Primary Extra",
        "x\u{fffd}#4; Primary",
        "A \u{fffd} B",
    ] {
        assert_eq!(
            ParentName::parse(name).unwrap_err(),
            ParentPartitionError::ParentNameUnsupported { ledgers: 1 },
            "{name:?}"
        );
    }
}

#[test]
fn ordinary_parent_names_are_accepted_as_they_are() {
    for name in [
        "Sundry Debtors",
        "Duties & Taxes",
        "Bank <A/C>",
        "O'Neil",
        "  Sales",
    ] {
        assert_eq!(ParentName::parse(name).unwrap().as_catalogue_text(), name);
    }
}

#[test]
fn formula_escapes_xml_once_and_joins_with_or() {
    let partition = plan(
        &[
            ("l1".into(), "g1".into(), Some("Duties & Taxes".into())),
            ("l2".into(), "g2".into(), Some("Bank <A>".into())),
        ],
        LIMITS,
    )
    .unwrap();
    assert_eq!(partition.parts().len(), 1);
    assert_eq!(
        partition.parts()[0].formula(),
        "$Parent = \"Bank &lt;A&gt;\" OR $Parent = \"Duties &amp; Taxes\""
    );
}

#[test]
fn the_reserved_root_is_written_as_tally_writes_it() {
    let root = format!("{TALLY_SANITIZED_ROOT_MARKER} Primary");
    let partition = plan(&[("l1".into(), "g1".into(), Some(root.clone()))], LIMITS).unwrap();
    assert_eq!(partition.parts()[0].formula(), "$Parent = \"&#4; Primary\"");
    let mut coverage = partition.coverage();
    coverage.accept(0, "g1", "l1", Some(&root)).unwrap();
    coverage.finish().unwrap();
}

#[test]
fn a_single_parent_formula_has_no_or() {
    let partition = plan(&rows(&[("A", 1)]), LIMITS).unwrap();
    assert_eq!(partition.parts()[0].formula(), "$Parent = \"A\"");
}

fn covered(all: &[(String, String, Option<String>)]) -> (ParentPartition, PartitionCoverage) {
    let partition = plan(all, LIMITS).unwrap();
    let coverage = partition.coverage();
    (partition, coverage)
}

fn part_index(partition: &ParentPartition, parent: &str) -> usize {
    partition
        .parts()
        .iter()
        .position(|part| {
            part.parents()
                .iter()
                .any(|candidate| candidate.as_catalogue_text() == parent)
        })
        .unwrap()
}

#[test]
fn coverage_accepts_every_row_once_in_its_own_part() {
    let all = rows(&[("A", 6), ("B", 5), ("C", 4)]);
    let (partition, mut coverage) = covered(&all);
    for (name, guid, parent) in &all {
        let parent = parent.as_deref().unwrap();
        coverage
            .accept(part_index(&partition, parent), guid, name, Some(parent))
            .unwrap();
    }
    coverage.finish().unwrap();
}

#[test]
fn coverage_matches_guids_ignoring_ascii_case() {
    let all = rows(&[("A", 1)]);
    let (partition, mut coverage) = covered(&all);
    coverage
        .accept(
            part_index(&partition, "A"),
            "GUID-A-0",
            "A-ledger-0",
            Some("A"),
        )
        .unwrap();
    coverage.finish().unwrap();
}

#[test]
fn a_row_from_the_wrong_part_is_refused() {
    let all = rows(&[("A", 10), ("B", 10)]);
    let (partition, mut coverage) = covered(&all);
    let wrong = 1 - part_index(&partition, "A");
    assert_eq!(
        coverage
            .accept(wrong, "guid-A-0", "A-ledger-0", Some("A"))
            .unwrap_err(),
        ParentPartitionError::RowOutsideParts
    );
}

#[test]
fn a_row_the_catalogue_lacks_is_refused() {
    let (_, mut coverage) = covered(&rows(&[("A", 1)]));
    assert_eq!(
        coverage
            .accept(0, "guid-new", "new", Some("A"))
            .unwrap_err(),
        ParentPartitionError::RowNotInCatalogue
    );
}

#[test]
fn a_row_whose_name_or_parent_differs_is_refused() {
    let (_, mut coverage) = covered(&rows(&[("A", 1)]));
    for (name, parent) in [
        ("renamed", Some("A")),
        ("A-ledger-0", Some("B")),
        ("A-ledger-0", None),
    ] {
        assert_eq!(
            coverage.accept(0, "guid-A-0", name, parent).unwrap_err(),
            ParentPartitionError::RowDiffersFromCatalogue
        );
    }
}

#[test]
fn a_row_returned_twice_is_refused() {
    let (_, mut coverage) = covered(&rows(&[("A", 2)]));
    coverage
        .accept(0, "guid-A-0", "A-ledger-0", Some("A"))
        .unwrap();
    assert_eq!(
        coverage
            .accept(0, "GUID-A-0", "A-ledger-0", Some("A"))
            .unwrap_err(),
        ParentPartitionError::RowRepeated
    );
}

#[test]
fn a_missing_row_fails_finish() {
    let (_, mut coverage) = covered(&rows(&[("A", 2)]));
    coverage
        .accept(0, "guid-A-0", "A-ledger-0", Some("A"))
        .unwrap();
    assert_eq!(
        coverage.finish().unwrap_err(),
        ParentPartitionError::RowsMissing
    );
}

#[test]
fn an_empty_catalogue_plans_no_parts_and_finishes() {
    let partition = plan(&[], LIMITS).unwrap();
    assert!(partition.parts().is_empty());
    partition.coverage().finish().unwrap();
}

#[test]
fn every_error_has_a_distinct_safe_code() {
    let codes = [
        ParentPartitionError::LedgerWithoutParent,
        ParentPartitionError::ParentNameUnsupported { ledgers: 1 },
        ParentPartitionError::DuplicateLedgerIdentity,
        ParentPartitionError::ParentOverBudget { ledgers: 1 },
        ParentPartitionError::TooManyParts { parts: 1 },
        ParentPartitionError::RowOutsideParts,
        ParentPartitionError::RowNotInCatalogue,
        ParentPartitionError::RowDiffersFromCatalogue,
        ParentPartitionError::RowRepeated,
        ParentPartitionError::RowsMissing,
    ]
    .map(ParentPartitionError::safe_code);
    let unique = codes.iter().collect::<HashSet<_>>();
    assert_eq!(unique.len(), codes.len());
}
