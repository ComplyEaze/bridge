//! The verdict function over constructed inputs: every state, and one test per
//! check that flips it. Nothing here reads or sends anything.
use super::*;
use crate::tally::ledger_create_item::{AdmittedDutyHead, MasterName, PartySide, VerifiedGstin};
use bridge_tally_protocol::GstDutyHead;

const TARGET: &str = "61c6de69-1748-461c-ad3f-162cb949df9f";
fn other() -> &'static str {
    crate::synthetic_guid(0x2222)
}
fn cash() -> &'static str {
    crate::synthetic_guid(0xaaaa)
}
fn debtors_group() -> &'static str {
    crate::synthetic_guid(0xbbbb)
}
fn new_ledger() -> &'static str {
    crate::synthetic_guid(0xd0d1)
}
const NAME: &str = "Zeta Test Ledger";
const LAB_GSTIN: &str = "08ZZZZZ0000Z2ZP";

fn name() -> MasterName {
    MasterName::new(NAME).unwrap()
}

fn suspense() -> LedgerCreateItem {
    LedgerCreateItem::new(name(), AdmittedLedgerKind::Suspense)
}

fn party(gstin: bool) -> LedgerCreateItem {
    LedgerCreateItem::new(
        name(),
        AdmittedLedgerKind::Party {
            side: PartySide::SundryDebtors,
            gstin: gstin.then(|| VerifiedGstin::new(LAB_GSTIN).unwrap()),
            bill_wise: true,
        },
    )
}

fn duty() -> LedgerCreateItem {
    LedgerCreateItem::new(
        name(),
        AdmittedLedgerKind::GstDuty {
            head: AdmittedDutyHead::new(GstDutyHead::SgstUtgst).unwrap(),
        },
    )
}

fn master(kind: MasterKind, guid: &str, alter_id: u64, names: &[&str]) -> IndexedMaster {
    IndexedMaster {
        kind,
        guid: guid.into(),
        alter_id,
        names: names.iter().map(ToString::to_string).collect(),
    }
}

fn mark(guid: &str, altmstid: u64) -> MasterMark {
    MasterMark {
        company_guid: guid.into(),
        altmstid,
    }
}

fn counters(created: u64, altered: u64, errors: u64, exceptions: u64) -> MasterCounters {
    MasterCounters {
        created,
        altered,
        deleted: 0,
        ignored: 0,
        errors,
        cancelled: 0,
        exceptions,
    }
}

fn pre_image() -> BTreeMap<String, u64> {
    BTreeMap::from([(cash().to_string(), 9), (debtors_group().to_string(), 4)])
}

/// The row the verified ledger reads back as, for `item`.
fn row_for(item: &LedgerCreateItem) -> PartyView {
    let (tax_type, duty_head, party_gstin) = match item.kind() {
        AdmittedLedgerKind::GstDuty { head } => (
            Some("GST".to_string()),
            Some(head.head().tally_spelling().to_string()),
            None,
        ),
        AdmittedLedgerKind::Party { gstin, .. } => (
            None,
            None,
            gstin.as_ref().map(|gstin| gstin.as_str().to_string()),
        ),
        AdmittedLedgerKind::Suspense => (None, None, None),
    };
    PartyView {
        name: NAME.into(),
        guid: new_ledger().into(),
        parent: Some(item.parent().into()),
        tax_type,
        duty_head,
        party_gstin,
        opening_balance: Some("0.00".into()),
    }
}

/// One scenario: the world after a clean create, every part open to change.
struct World {
    item: LedgerCreateItem,
    pre_image: BTreeMap<String, u64>,
    pre_marks: Vec<MasterMark>,
    counters: Option<MasterCounters>,
    moment: Moment,
    read: ReadBack,
}

impl World {
    fn clean(item: LedgerCreateItem) -> Self {
        let row = row_for(&item);
        Self {
            item,
            pre_image: pre_image(),
            pre_marks: vec![mark(TARGET, 7), mark(other(), 3)],
            counters: Some(counters(1, 0, 0, 0)),
            moment: Moment::JustPosted,
            read: ReadBack {
                marks: Some(vec![mark(TARGET, 8), mark(other(), 3)]),
                masters: Some(vec![
                    master(MasterKind::Ledger, cash(), 9, &["Cash"]),
                    master(MasterKind::Group, debtors_group(), 4, &["Sundry Debtors"]),
                    master(MasterKind::Ledger, new_ledger(), 11, &[NAME]),
                ]),
                party: PartyRead::Rows(vec![row]),
            },
        }
    }

    /// The same world with nothing created: the book as it was.
    fn untouched(item: LedgerCreateItem) -> Self {
        let mut world = Self::clean(item);
        world.moment = Moment::Reconcile;
        world.counters = None;
        world.read.marks = Some(vec![mark(TARGET, 7), mark(other(), 3)]);
        world.read.masters = Some(vec![
            master(MasterKind::Ledger, cash(), 9, &["Cash"]),
            master(MasterKind::Group, debtors_group(), 4, &["Sundry Debtors"]),
        ]);
        world.read.party = PartyRead::NotRead;
        world
    }

    fn judge(&self) -> Judgement {
        judge(
            &Facts {
                item: &self.item,
                company_guid: TARGET,
                pre_image: &self.pre_image,
                pre_marks: &self.pre_marks,
                counters: self.counters,
                moment: self.moment,
            },
            &self.read,
        )
    }

    fn rows(&mut self) -> &mut PartyView {
        match &mut self.read.party {
            PartyRead::Rows(rows) => &mut rows[0],
            other => panic!("no rows: {other:?}"),
        }
    }

    fn masters(&mut self) -> &mut Vec<IndexedMaster> {
        self.read.masters.as_mut().unwrap()
    }

    fn marks(&mut self) -> &mut Vec<MasterMark> {
        self.read.marks.as_mut().unwrap()
    }
}

fn items() -> [(&'static str, LedgerCreateItem); 4] {
    [
        ("suspense", suspense()),
        ("party", party(false)),
        ("party_gstin", party(true)),
        ("duty", duty()),
    ]
}

fn assert_verified(world: &World, label: &str) {
    let judgement = world.judge();
    assert_eq!(
        judgement.state,
        MasterVerdictState::PostedVerified,
        "{label}: {judgement:?}"
    );
    assert_eq!(judgement.guid.as_deref(), Some(new_ledger()), "{label}");
    assert!(judgement.checks.all_passed(), "{label}");
    assert!(judgement.reasons.is_empty(), "{label}");
}

/// One check flipped: the verdict is `reconciliation_required` with that
/// check failed and its reason given.
fn assert_doubt(world: &World, label: &str, failed: impl Fn(&Checks) -> Check, reason: &str) {
    let judgement = world.judge();
    assert_eq!(
        judgement.state,
        MasterVerdictState::ReconciliationRequired,
        "{label}: {judgement:?}"
    );
    assert_eq!(failed(&judgement.checks), Check::Failed, "{label}");
    assert!(
        judgement.reasons.contains(&reason),
        "{label}: {:?}",
        judgement.reasons
    );
    assert!(!judgement.checks.all_passed(), "{label}");
}

// --- posted_verified -------------------------------------------------------

#[test]
fn a_clean_create_of_each_kind_is_posted_verified_with_every_check_passed() {
    for (label, item) in items() {
        let world = World::clean(item);
        assert_verified(&world, label);
        let checks = world.judge().checks;
        for check in [
            checks.name,
            checks.guid_new,
            checks.parent,
            checks.tax_fields,
            checks.party_gstin,
            checks.opening_balance,
            checks.counters,
            checks.target_mark_moved,
            checks.other_marks_unmoved,
            checks.pre_image_unchanged,
        ] {
            assert_eq!(check, Check::Passed, "{label}");
        }
    }
}

#[test]
fn absent_counters_never_give_posted_verified_whatever_else_holds() {
    for (label, item) in items() {
        let mut world = World::clean(item);
        world.counters = None;
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{label}"
        );
        assert_eq!(judgement.checks.counters, Check::NotChecked, "{label}");
        assert!(
            judgement.reasons.contains(&"counters_unavailable"),
            "{label}: {:?}",
            judgement.reasons
        );
        // Every other check passed: only the counters keep it unverified.
        assert_eq!(judgement.guid.as_deref(), Some(new_ledger()), "{label}");
        assert_eq!(judgement.checks.name, Check::Passed);
        assert_eq!(judgement.checks.parent, Check::Passed);
        // The same on a reconcile, where an unknown outcome has no counters.
        world.moment = Moment::Reconcile;
        assert_eq!(
            world.judge().state,
            MasterVerdictState::ReconciliationRequired,
            "{label} reconcile"
        );
    }
}

#[test]
fn an_absent_or_zero_opening_balance_is_accepted_and_no_other_is() {
    for opening in [
        None,
        Some(""),
        Some("0"),
        Some("0.00"),
        Some("-0.00"),
        Some(" 0 "),
    ] {
        let mut world = World::clean(suspense());
        world.rows().opening_balance = opening.map(str::to_owned);
        assert_verified(&world, &format!("{opening:?}"));
    }
    for opening in [
        "1250.00",
        "-50000.00",
        "0.01",
        "1 Rs",
        "abc",
        "0 @ 1 = 0 x",
        ".",
    ] {
        let mut world = World::clean(suspense());
        world.rows().opening_balance = Some(opening.into());
        assert_doubt(
            &world,
            opening,
            |checks| checks.opening_balance,
            "opening_balance_not_zero",
        );
    }
}

#[test]
fn an_unrelated_master_does_not_make_the_name_ambiguous() {
    // Another master's unrelated alias does not make the name ambiguous.
    let mut world = World::clean(suspense());
    world.masters().push(master(
        MasterKind::Ledger,
        crate::synthetic_guid(0xe001),
        1,
        &["Other", "Another"],
    ));
    assert_verified(&world, "unrelated master");
}

// --- one test per check ----------------------------------------------------

#[test]
fn the_name_check_fails_for_a_name_stored_in_another_spelling() {
    for stored in [
        "zeta test ledger",
        "ZETA TEST LEDGER",
        "Zeta-Test Ledger",
        "Zeta Test  Ledger",
    ] {
        let mut world = World::clean(suspense());
        world.masters().last_mut().unwrap().names = vec![stored.to_string()];
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{stored}"
        );
        assert_eq!(judgement.checks.name, Check::Failed, "{stored}");
        assert!(judgement.guid.is_none(), "{stored}");
        assert!(judgement.reasons.contains(&"name_not_one_ledger_in_book"));
    }
}

#[test]
fn the_name_check_fails_for_a_second_ledger_or_an_alias_or_a_group() {
    let second = master(
        MasterKind::Ledger,
        crate::synthetic_guid(0xe002),
        12,
        &[NAME],
    );
    let alias = master(
        MasterKind::Ledger,
        crate::synthetic_guid(0xe003),
        12,
        &["Other", NAME],
    );
    let group = master(
        MasterKind::Group,
        crate::synthetic_guid(0xe004),
        12,
        &[NAME],
    );
    for (label, extra) in [("second", second), ("alias", alias), ("group", group)] {
        let mut world = World::clean(suspense());
        world.masters().push(extra);
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{label}"
        );
        assert_eq!(judgement.checks.name, Check::Failed, "{label}");
        assert!(judgement
            .reasons
            .contains(&"name_collides_with_more_than_one_master"));
    }
}

#[test]
fn the_name_check_fails_when_the_parent_read_row_carries_another_name() {
    let mut world = World::clean(suspense());
    world.rows().name = "Zeta Test Ledger ".into();
    assert_doubt(
        &world,
        "row name",
        |checks| checks.name,
        "name_not_one_ledger_in_book",
    );
}

#[test]
fn a_ledger_whose_guid_is_in_the_pre_image_is_into_an_existing_master_never_verified() {
    let mut world = World::clean(suspense());
    // The name now resolves to Cash's GUID.
    *world.masters() = vec![
        master(MasterKind::Ledger, cash(), 9, &[NAME]),
        master(MasterKind::Group, debtors_group(), 4, &["Sundry Debtors"]),
    ];
    let judgement = world.judge();
    assert_eq!(
        judgement.state,
        MasterVerdictState::PostedIntoExistingMaster
    );
    assert_eq!(judgement.guid.as_deref(), Some(cash()));
    assert_eq!(judgement.checks.guid_new, Check::Failed);
    assert!(judgement
        .reasons
        .contains(&"name_resolves_to_existing_master"));
}

#[test]
fn a_name_that_resolves_to_a_pre_image_master_through_an_alias_is_into_an_existing_master() {
    let mut world = World::clean(suspense());
    *world.masters() = vec![
        master(MasterKind::Ledger, cash(), 9, &["Cash", NAME]),
        master(MasterKind::Group, debtors_group(), 4, &["Sundry Debtors"]),
    ];
    assert_eq!(
        world.judge().state,
        MasterVerdictState::PostedIntoExistingMaster
    );
}

/// A pre-image master that changed while nothing ties the change to this
/// create (no altered count, the name not resolving to it) is not called a
/// write into it: the batch is left to a person, holding its name, and is
/// never verified.
#[test]
fn a_changed_pre_image_alter_id_alone_is_for_a_person_never_verified() {
    for changed in [cash(), debtors_group()] {
        let mut world = World::clean(suspense());
        for entry in world.masters().iter_mut() {
            if entry.guid == changed {
                entry.alter_id += 1;
            }
        }
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{changed}"
        );
        assert!(!judgement.state.is_terminal());
        assert_eq!(judgement.checks.pre_image_unchanged, Check::Failed);
        assert!(judgement.reasons.contains(&"pre_image_master_changed"));
    }
}

#[test]
fn altered_above_zero_is_into_an_existing_master() {
    let mut world = World::clean(suspense());
    world.counters = Some(counters(1, 1, 0, 0));
    let judgement = world.judge();
    assert_eq!(
        judgement.state,
        MasterVerdictState::PostedIntoExistingMaster
    );
    assert!(judgement.reasons.contains(&"tally_reported_altered"));
}

#[test]
fn a_pre_image_master_missing_after_the_post_is_a_doubt_not_a_verified_create() {
    let mut world = World::clean(suspense());
    world.masters().retain(|entry| entry.guid != cash());
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert_eq!(judgement.checks.pre_image_unchanged, Check::Failed);
    assert!(judgement.reasons.contains(&"pre_image_master_missing"));
}

#[test]
fn the_parent_check_fails_for_another_parent_or_none() {
    for (label, item) in items() {
        for parent in [Some("Sundry Creditors"), Some("Primary"), Some(""), None] {
            let mut world = World::clean(item.clone());
            world.rows().parent = parent.map(str::to_owned);
            assert_doubt(
                &world,
                &format!("{label} {parent:?}"),
                |checks| checks.parent,
                "parent_differs_from_sent",
            );
        }
    }
}

#[test]
fn the_tax_fields_check_fails_for_a_duty_ledger_whose_head_or_type_differs() {
    for (tax_type, head) in [
        (Some("GST"), Some("CGST")),
        (Some("GST"), Some("sgst/utgst")),
        (Some("GST"), None),
        (Some("Others"), Some("SGST/UTGST")),
        (None, Some("SGST/UTGST")),
    ] {
        let mut world = World::clean(duty());
        world.rows().tax_type = tax_type.map(str::to_owned);
        world.rows().duty_head = head.map(str::to_owned);
        assert_doubt(
            &world,
            &format!("{tax_type:?} {head:?}"),
            |checks| checks.tax_fields,
            "tax_fields_differ_from_sent",
        );
    }
}

#[test]
fn the_tax_fields_check_fails_for_a_ledger_that_sent_no_tax_fields_but_reads_back_gst() {
    for item in [suspense(), party(false), party(true)] {
        let mut with_head = World::clean(item.clone());
        with_head.rows().duty_head = Some("CGST".into());
        assert_doubt(
            &with_head,
            "duty head",
            |checks| checks.tax_fields,
            "tax_fields_differ_from_sent",
        );
        let mut with_type = World::clean(item);
        with_type.rows().tax_type = Some("GST".into());
        assert_doubt(
            &with_type,
            "tax type",
            |checks| checks.tax_fields,
            "tax_fields_differ_from_sent",
        );
    }
}

#[test]
fn the_gstin_check_fails_for_another_gstin_or_none_or_one_never_sent() {
    let mut other = World::clean(party(true));
    other.rows().party_gstin = Some("08ZZZZZ0000Z1ZQ".into());
    assert_doubt(
        &other,
        "other",
        |checks| checks.party_gstin,
        "party_gstin_differs_from_sent",
    );
    let mut none = World::clean(party(true));
    none.rows().party_gstin = None;
    assert_doubt(
        &none,
        "none",
        |checks| checks.party_gstin,
        "party_gstin_differs_from_sent",
    );
    let mut empty = World::clean(party(true));
    empty.rows().party_gstin = Some(String::new());
    assert_doubt(
        &empty,
        "empty",
        |checks| checks.party_gstin,
        "party_gstin_differs_from_sent",
    );
    for item in [suspense(), party(false), duty()] {
        let mut stray = World::clean(item);
        stray.rows().party_gstin = Some(LAB_GSTIN.into());
        assert_doubt(
            &stray,
            "stray",
            |checks| checks.party_gstin,
            "party_gstin_differs_from_sent",
        );
        let mut empty = World::clean(suspense());
        empty.rows().party_gstin = Some(String::new());
        assert_verified(&empty, "an explicit empty GSTIN is absent");
    }
}

#[test]
fn each_counter_outside_one_created_none_altered_no_errors_no_exceptions_fails() {
    for (label, counters) in [
        ("none created", counters(0, 0, 0, 0)),
        ("two created", counters(2, 0, 0, 0)),
        ("errors", counters(1, 0, 1, 0)),
        ("exceptions", counters(1, 0, 0, 1)),
    ] {
        let mut world = World::clean(suspense());
        world.counters = Some(counters);
        assert_doubt(
            &world,
            label,
            |checks| checks.counters,
            "counters_not_one_created",
        );
    }
}

#[test]
fn the_target_mark_must_have_moved_forward() {
    for after in [7, 6] {
        let mut world = World::clean(suspense());
        world.marks()[0] = mark(TARGET, after);
        assert_doubt(
            &world,
            &format!("{after}"),
            |checks| checks.target_mark_moved,
            "target_mark_not_moved",
        );
    }
}

#[test]
fn another_companys_mark_moving_is_located_elsewhere_whatever_else_holds() {
    let mut world = World::clean(suspense());
    world.marks()[1] = mark(other(), 4);
    let judgement = world.judge();
    assert_eq!(
        judgement.state,
        MasterVerdictState::MasterWriteLocatedElsewhere
    );
    assert_eq!(judgement.checks.other_marks_unmoved, Check::Failed);
    assert!(judgement.reasons.contains(&"other_company_mark_moved"));
    // The same when the target did not move at all.
    let mut world = World::clean(suspense());
    world.marks()[0] = mark(TARGET, 7);
    world.marks()[1] = mark(other(), 2);
    assert_eq!(
        world.judge().state,
        MasterVerdictState::MasterWriteLocatedElsewhere
    );
}

#[test]
fn a_company_loaded_or_unloaded_across_the_post_is_a_doubt() {
    let mut added = World::clean(suspense());
    added.marks().push(mark(crate::synthetic_guid(0x3333), 1));
    let judgement = added.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement.reasons.contains(&"company_scope_changed"));
    let mut removed = World::clean(suspense());
    removed.marks().pop();
    let judgement = removed.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement.reasons.contains(&"company_scope_changed"));
    // Two rows for one company cannot be told apart.
    let mut doubled = World::clean(suspense());
    doubled.marks().push(mark(other(), 3));
    assert_eq!(
        doubled.judge().state,
        MasterVerdictState::ReconciliationRequired
    );
}

#[test]
fn the_ledger_missing_from_the_parent_read_or_repeated_in_it_is_a_doubt() {
    let mut missing = World::clean(suspense());
    missing.read.party = PartyRead::Rows(Vec::new());
    let judgement = missing.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement
        .reasons
        .contains(&"ledger_missing_from_parent_read"));
    let mut repeated = World::clean(suspense());
    let row = row_for(&suspense());
    repeated.read.party = PartyRead::Rows(vec![row.clone(), row]);
    assert!(repeated
        .judge()
        .reasons
        .contains(&"ledger_repeated_in_parent_read"));
    let mut not_read = World::clean(suspense());
    not_read.read.party = PartyRead::NotRead;
    assert!(not_read.judge().reasons.contains(&"party_read_not_made"));
    assert_eq!(
        not_read.judge().state,
        MasterVerdictState::ReconciliationRequired
    );
    // A row for another ledger does not describe this one.
    let mut other = World::clean(suspense());
    other.rows().guid = crate::synthetic_guid(0xe009).into();
    assert!(other
        .judge()
        .reasons
        .contains(&"ledger_missing_from_parent_read"));
}

#[test]
fn a_guid_the_journal_cannot_record_is_never_verified() {
    let mut world = World::clean(suspense());
    world.masters().last_mut().unwrap().guid = "NOT A GUID!".into();
    world.rows().guid = "not a guid!".into();
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement.reasons.contains(&"ledger_guid_not_recordable"));
    assert!(judgement.guid.is_none());
}

#[test]
fn an_index_that_cannot_be_built_is_a_doubt() {
    let mut world = World::clean(suspense());
    let duplicate = world.masters()[0].clone();
    world.masters().push(duplicate);
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement.reasons.contains(&"master_index_not_usable"));
}

// --- the checks type itself ------------------------------------------------

#[test]
fn all_passed_needs_every_check_counters_included() {
    let all = Checks {
        name: Check::Passed,
        guid_new: Check::Passed,
        parent: Check::Passed,
        tax_fields: Check::Passed,
        party_gstin: Check::Passed,
        opening_balance: Check::Passed,
        counters: Check::Passed,
        target_mark_moved: Check::Passed,
        other_marks_unmoved: Check::Passed,
        pre_image_unchanged: Check::Passed,
    };
    assert!(all.all_passed());
    assert!(!Checks {
        counters: Check::NotChecked,
        ..all
    }
    .all_passed());
    assert!(!Checks {
        counters: Check::Failed,
        ..all
    }
    .all_passed());
    for flipped in [Check::Failed, Check::NotChecked] {
        assert!(!Checks {
            name: flipped,
            ..all
        }
        .all_passed());
        assert!(!Checks {
            guid_new: flipped,
            ..all
        }
        .all_passed());
        assert!(!Checks {
            parent: flipped,
            ..all
        }
        .all_passed());
        assert!(!Checks {
            tax_fields: flipped,
            ..all
        }
        .all_passed());
        assert!(!Checks {
            party_gstin: flipped,
            ..all
        }
        .all_passed());
        assert!(!Checks {
            opening_balance: flipped,
            ..all
        }
        .all_passed());
        assert!(!Checks {
            target_mark_moved: flipped,
            ..all
        }
        .all_passed());
        assert!(!Checks {
            other_marks_unmoved: flipped,
            ..all
        }
        .all_passed());
        assert!(!Checks {
            pre_image_unchanged: flipped,
            ..all
        }
        .all_passed());
    }
    assert!(!Checks::none().all_passed());
}

// --- confirmed_absent ------------------------------------------------------

#[test]
fn a_reconcile_that_finds_nothing_and_no_mark_moved_is_confirmed_absent() {
    for (label, item) in items() {
        let world = World::untouched(item);
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ConfirmedAbsent,
            "{label}: {judgement:?}"
        );
        assert!(judgement.guid.is_none(), "{label}");
        assert!(!judgement.checks.all_passed());
    }
}

#[test]
fn confirmed_absent_with_counters_that_created_nothing_is_still_absent() {
    let mut world = World::untouched(suspense());
    world.counters = Some(counters(0, 0, 1, 1));
    assert_eq!(world.judge().state, MasterVerdictState::ConfirmedAbsent);
}

#[test]
fn right_after_a_post_a_missing_ledger_is_never_confirmed_absent() {
    let mut world = World::untouched(suspense());
    world.moment = Moment::JustPosted;
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement.reasons.contains(&"ledger_absent_after_post"));
}

#[test]
fn a_moved_target_mark_forbids_confirmed_absent() {
    for after in [8, 6] {
        let mut world = World::untouched(suspense());
        world.marks()[0] = mark(TARGET, after);
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{after}"
        );
    }
}

#[test]
fn a_moved_other_mark_on_a_reconcile_is_located_elsewhere_not_absent() {
    let mut world = World::untouched(suspense());
    world.marks()[1] = mark(other(), 4);
    assert_eq!(
        world.judge().state,
        MasterVerdictState::MasterWriteLocatedElsewhere
    );
}

#[test]
fn a_changed_or_missing_pre_image_master_forbids_confirmed_absent() {
    let mut changed = World::untouched(suspense());
    changed.masters()[0].alter_id += 1;
    assert_eq!(
        changed.judge().state,
        MasterVerdictState::ReconciliationRequired
    );
    let mut missing = World::untouched(suspense());
    missing.masters().remove(0);
    assert_eq!(
        missing.judge().state,
        MasterVerdictState::ReconciliationRequired
    );
}

#[test]
fn a_name_present_under_either_fold_forbids_confirmed_absent() {
    for stored in [
        "zeta test ledger",
        "Zeta-Test Ledger",
        "Zeta Test  Ledger",
        // NFD against NFC and an en dash against a space share the absence key.
        "Zeta\u{2013}Test Ledger",
    ] {
        let mut world = World::untouched(suspense());
        world
            .masters()
            .push(master(MasterKind::Ledger, new_ledger(), 11, &[stored]));
        let judgement = world.judge();
        assert_ne!(
            judgement.state,
            MasterVerdictState::ConfirmedAbsent,
            "{stored}"
        );
    }
    // A group or an alias holding the name is a presence too.
    let mut group = World::untouched(suspense());
    group
        .masters()
        .push(master(MasterKind::Group, new_ledger(), 11, &[NAME]));
    assert_ne!(group.judge().state, MasterVerdictState::ConfirmedAbsent);
    let mut alias = World::untouched(suspense());
    alias.masters().push(master(
        MasterKind::Ledger,
        new_ledger(),
        11,
        &["Other", NAME],
    ));
    assert_ne!(alias.judge().state, MasterVerdictState::ConfirmedAbsent);
}

#[test]
fn counters_that_claim_a_create_forbid_confirmed_absent() {
    let mut world = World::untouched(suspense());
    world.counters = Some(counters(1, 0, 0, 0));
    assert_eq!(
        world.judge().state,
        MasterVerdictState::ReconciliationRequired
    );
}

#[test]
fn a_changed_set_of_companies_forbids_confirmed_absent() {
    let mut world = World::untouched(suspense());
    world.marks().pop();
    assert_eq!(
        world.judge().state,
        MasterVerdictState::ReconciliationRequired
    );
    let mut world = World::untouched(suspense());
    world.marks().remove(0);
    assert_eq!(
        world.judge().state,
        MasterVerdictState::ReconciliationRequired
    );
}

// --- masters_after_post_unconfirmed ---------------------------------------

#[test]
fn each_unavailable_read_back_is_masters_after_post_unconfirmed() {
    type Unavailable = fn(&mut ReadBack);
    let cases: [(&str, Unavailable, &str); 3] = [
        (
            "marks",
            |read| read.marks = None,
            "company_marks_unavailable",
        ),
        (
            "index",
            |read| read.masters = None,
            "master_index_unavailable",
        ),
        (
            "party",
            |read| read.party = PartyRead::Unavailable,
            "party_read_unavailable",
        ),
    ];
    for moment in [Moment::JustPosted, Moment::Reconcile] {
        for (label, unavailable, reason) in cases {
            let mut world = World::clean(suspense());
            world.moment = moment;
            unavailable(&mut world.read);
            let judgement = world.judge();
            assert_eq!(
                judgement.state,
                MasterVerdictState::MastersAfterPostUnconfirmed,
                "{label}"
            );
            assert_eq!(judgement.reasons, vec![reason], "{label}");
            assert!(judgement.guid.is_none());
        }
    }
    // Unavailable beats everything the rest would have said, loud states too.
    let mut world = World::clean(suspense());
    world.marks()[1] = mark(other(), 4);
    world.read.party = PartyRead::Unavailable;
    assert_eq!(
        world.judge().state,
        MasterVerdictState::MastersAfterPostUnconfirmed
    );
}

#[test]
fn an_unavailable_read_back_is_not_absence() {
    let mut world = World::untouched(suspense());
    world.read.masters = None;
    assert_eq!(
        world.judge().state,
        MasterVerdictState::MastersAfterPostUnconfirmed
    );
}

// --- the read fact the next step is chosen by ------------------------------

/// Every scenario the verdict tests above build, with the state and reasons it
/// gave before `name_absent_in_read_book` was added and still gives, and what
/// the new fact says of its read.
#[test]
fn every_verdict_state_and_reason_is_unchanged_and_the_fact_beside_them_is_exact() {
    type Change = fn(&mut World);
    use MasterVerdictState::{
        ConfirmedAbsent, MasterWriteLocatedElsewhere, MastersAfterPostUnconfirmed,
        PostedIntoExistingMaster, PostedVerified, ReconciliationRequired,
    };
    type Case = (
        &'static str,
        World,
        Change,
        MasterVerdictState,
        Vec<&'static str>,
        bool,
    );
    let cases: [Case; 12] = [
        (
            "clean",
            World::clean(suspense()),
            |_| {},
            PostedVerified,
            vec![],
            false,
        ),
        (
            "untouched reconcile",
            World::untouched(suspense()),
            |_| {},
            ConfirmedAbsent,
            vec!["not_in_book"],
            true,
        ),
        (
            "untouched just posted",
            World::untouched(suspense()),
            |world| world.moment = Moment::JustPosted,
            ReconciliationRequired,
            vec![
                "name_not_one_ledger_in_book",
                "target_mark_not_moved",
                "counters_unavailable",
                "ledger_absent_after_post",
            ],
            true,
        ),
        (
            "absent, target mark moved",
            World::untouched(suspense()),
            |world| world.marks()[0] = mark(TARGET, 8),
            ReconciliationRequired,
            vec!["name_not_one_ledger_in_book", "counters_unavailable"],
            true,
        ),
        (
            "absent, other mark moved",
            World::untouched(suspense()),
            |world| world.marks()[1] = mark(other(), 4),
            MasterWriteLocatedElsewhere,
            vec!["other_company_mark_moved"],
            true,
        ),
        (
            "absent, companies changed",
            World::untouched(suspense()),
            |world| {
                world.marks().pop();
            },
            ReconciliationRequired,
            vec![
                "company_scope_changed",
                "name_not_one_ledger_in_book",
                "target_mark_not_moved",
                "counters_unavailable",
            ],
            true,
        ),
        (
            "present, no counters",
            World::clean(suspense()),
            |world| world.counters = None,
            ReconciliationRequired,
            vec!["counters_unavailable"],
            false,
        ),
        (
            "altered",
            World::clean(suspense()),
            |world| world.counters = Some(counters(1, 1, 0, 0)),
            PostedIntoExistingMaster,
            vec!["tally_reported_altered"],
            false,
        ),
        (
            "pre-image master missing",
            World::clean(suspense()),
            |world| world.masters().retain(|entry| entry.guid != cash()),
            ReconciliationRequired,
            vec!["pre_image_master_missing"],
            false,
        ),
        (
            "index not usable",
            World::clean(suspense()),
            |world| {
                let duplicate = world.masters()[0].clone();
                world.masters().push(duplicate);
            },
            ReconciliationRequired,
            vec!["master_index_not_usable"],
            false,
        ),
        (
            "marks unavailable",
            World::untouched(suspense()),
            |world| world.read.marks = None,
            MastersAfterPostUnconfirmed,
            vec!["company_marks_unavailable"],
            false,
        ),
        (
            "index unavailable",
            World::untouched(suspense()),
            |world| world.read.masters = None,
            MastersAfterPostUnconfirmed,
            vec!["master_index_unavailable"],
            false,
        ),
    ];
    for (label, mut world, change, state, reasons, absent) in cases {
        change(&mut world);
        let judgement = world.judge();
        assert_eq!(judgement.state, state, "{label}: {judgement:?}");
        assert_eq!(judgement.reasons, reasons, "{label}");
        assert_eq!(judgement.name_absent_in_read_book, absent, "{label}");
    }
}

/// The fact is about the name in the index that was read, and nothing else.
#[test]
fn the_name_absent_fact_follows_the_index_whatever_the_moment_or_the_marks() {
    for (label, item) in items() {
        for moment in [Moment::JustPosted, Moment::Reconcile] {
            let mut absent = World::untouched(item.clone());
            absent.moment = moment;
            assert!(
                absent.judge().name_absent_in_read_book,
                "{label} {moment:?}"
            );
            let mut found = World::clean(item.clone());
            found.moment = moment;
            assert!(
                !found.judge().name_absent_in_read_book,
                "{label} {moment:?}"
            );
        }
    }
    // A name present under another spelling is a collision, not an absence.
    let mut twin = World::untouched(suspense());
    twin.masters().push(master(
        MasterKind::Ledger,
        new_ledger(),
        11,
        &["zeta test ledger"],
    ));
    assert!(!twin.judge().name_absent_in_read_book);
}

// --- the boundary helpers --------------------------------------------------

#[test]
fn the_party_read_is_needed_only_for_one_new_ledger_of_the_item_name() {
    let item = suspense();
    let clean = World::clean(item.clone());
    assert!(party_read_needed(
        &item,
        &clean.pre_image,
        clean.read.masters.as_ref().unwrap()
    ));
    let untouched = World::untouched(item.clone());
    assert!(!party_read_needed(
        &item,
        &untouched.pre_image,
        untouched.read.masters.as_ref().unwrap()
    ));
    // The name on a pre-image master needs no field check: it is already a doubt.
    let mut existing = World::clean(item.clone());
    *existing.masters() = vec![master(MasterKind::Ledger, cash(), 9, &[NAME])];
    assert!(!party_read_needed(
        &item,
        &existing.pre_image,
        existing.read.masters.as_ref().unwrap()
    ));
    // Two holders of the name: nothing one row could settle.
    let mut two = World::clean(item.clone());
    two.masters().push(master(
        MasterKind::Ledger,
        crate::synthetic_guid(0xe002),
        1,
        &[NAME],
    ));
    assert!(!party_read_needed(
        &item,
        &two.pre_image,
        two.read.masters.as_ref().unwrap()
    ));
}

#[test]
fn zero_amounts_are_recognised_exactly() {
    for zero in ["", " ", "0", "00", "0.0", "0.00", "-0", "+0.000", " 0 "] {
        assert!(is_zero_amount(zero), "{zero:?}");
    }
    for other in [
        "1", "0.01", "10", "-1.5", ".", "0.0.0", "0 Rs", "x", "--0", "1e0",
    ] {
        assert!(!is_zero_amount(other), "{other:?}");
    }
}

// --- the projection keeps no client value ----------------------------------

fn party_collection(row_fields: &str) -> String {
    format!(
        "<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER><BODY><DATA><COLLECTION><LEDGER NAME=\"{NAME}\"><GUID>{TARGET}-000000ce</GUID><BRIDGECOMPANYGUID>{TARGET}</BRIDGECOMPANYGUID><MASTERID>206</MASTERID><ALTERID>208</ALTERID><PARENT>Sundry Debtors</PARENT><OPENINGBALANCE>0.00</OPENINGBALANCE>{row_fields}</LEDGER></COLLECTION></DATA></BODY></ENVELOPE>"
    )
}

#[test]
fn the_projection_of_a_parsed_row_holds_no_pan_bank_email_phone_or_address() {
    let xml = party_collection(
        "<PARTYGSTIN>08ZZZZZ0000Z2ZP</PARTYGSTIN><INCOMETAXNUMBER>SYNTHPAN01</INCOMETAXNUMBER>\
         <NAMEONPAN>Pan Holder Name</NAMEONPAN><BANKACCHOLDERNAME>Bank Holder Name</BANKACCHOLDERNAME>\
         <BANKDETAILS>111111111111</BANKDETAILS><IFSCODE>ZZZZ0000001</IFSCODE>\
         <EMAIL>someone@example.invalid</EMAIL><LEDGERPHONE>9999999999</LEDGERPHONE>\
         <LEDPINCODE>560001</LEDPINCODE><STATENAME>Synthetic State</STATENAME>\
         <LEDADDRESS.LIST><LEDADDRESS>1 Synthetic Road</LEDADDRESS></LEDADDRESS.LIST>\
         <TAXTYPE>GST</TAXTYPE><GSTDUTYHEAD>SGST/UTGST</GSTDUTYHEAD>",
    );
    let parsed =
        bridge_tally_protocol::parse_native_party_ledger_master_records_with_evidence(&xml, TARGET)
            .unwrap();
    let views = parsed
        .records
        .iter()
        .filter_map(PartyView::project)
        .collect::<Vec<_>>();
    assert_eq!(views.len(), 1);
    let view = &views[0];
    assert_eq!(view.name, NAME);
    assert_eq!(view.guid, format!("{TARGET}-000000ce"));
    assert_eq!(view.parent.as_deref(), Some("Sundry Debtors"));
    assert_eq!(view.tax_type.as_deref(), Some("GST"));
    assert_eq!(view.duty_head.as_deref(), Some("SGST/UTGST"));
    assert_eq!(view.party_gstin.as_deref(), Some("08ZZZZZ0000Z2ZP"));
    assert_eq!(view.opening_balance.as_deref(), Some("0.00"));
    let shown = format!("{views:?}");
    for private in [
        "SYNTHPAN01",
        "Pan Holder Name",
        "Bank Holder Name",
        "111111111111",
        "ZZZZ0000001",
        "someone@example.invalid",
        "9999999999",
        "560001",
        "Synthetic State",
        "1 Synthetic Road",
    ] {
        assert!(!shown.contains(private), "{private} reached the view");
    }
}

/// A positive control for the test above: the parser's own row does hold the
/// private values, so their absence from the view is the projection's doing.
#[test]
fn the_parsed_row_holds_the_private_values_the_projection_leaves_behind() {
    let xml = party_collection(
        "<INCOMETAXNUMBER>SYNTHPAN01</INCOMETAXNUMBER><EMAIL>someone@example.invalid</EMAIL>\
         <BANKDETAILS>111111111111</BANKDETAILS><LEDGERPHONE>9999999999</LEDGERPHONE>",
    );
    let parsed =
        bridge_tally_protocol::parse_native_party_ledger_master_records_with_evidence(&xml, TARGET)
            .unwrap();
    let held = format!("{:?}", parsed.records);
    for private in [
        "SYNTHPAN01",
        "someone@example.invalid",
        "111111111111",
        "9999999999",
    ] {
        assert!(held.contains(private), "{private}");
    }
    let view = format!("{:?}", PartyView::project(&parsed.records[0]).unwrap());
    for private in [
        "SYNTHPAN01",
        "someone@example.invalid",
        "111111111111",
        "9999999999",
    ] {
        assert!(!view.contains(private), "{private}");
    }
}

#[test]
fn a_contradictory_duty_head_keeps_its_raw_text_and_a_tax_free_row_has_none() {
    let xml = party_collection("<TAXTYPE>Others</TAXTYPE><GSTDUTYHEAD>CGST</GSTDUTYHEAD>");
    let parsed =
        bridge_tally_protocol::parse_native_party_ledger_master_records_with_evidence(&xml, TARGET)
            .unwrap();
    let view = PartyView::project(&parsed.records[0]).unwrap();
    assert_eq!(view.duty_head.as_deref(), Some("CGST"));
    assert_eq!(view.tax_type.as_deref(), Some("Others"));
    let plain = party_collection("");
    let parsed = bridge_tally_protocol::parse_native_party_ledger_master_records_with_evidence(
        &plain, TARGET,
    )
    .unwrap();
    let view = PartyView::project(&parsed.records[0]).unwrap();
    assert_eq!(view.duty_head, None);
    assert_eq!(view.party_gstin, None);
}

/// One ledger that matches the item name through two of its own spellings
/// (its stored name and a spelling that folds alike) is still one ledger, so
/// the create verifies.
#[test]
fn one_ledger_matching_through_two_of_its_spellings_is_still_verified() {
    let mut world = World::clean(suspense());
    let new = world
        .masters()
        .iter_mut()
        .find(|master| master.guid == new_ledger())
        .unwrap();
    new.names.push(NAME.to_uppercase());
    assert_verified(&world, "two spellings");
}

/// The judgement a caller may give without a read-back never settles a batch,
/// so it never releases the name the batch holds.
#[test]
fn an_open_judgement_is_never_terminal() {
    for verdict in [
        OpenVerdict::ReconciliationRequired,
        OpenVerdict::MastersAfterPostUnconfirmed,
    ] {
        let judgement = Judgement::open(verdict, "a_reason");
        assert!(!judgement.state.is_terminal(), "{verdict:?}");
        assert_eq!(judgement.guid, None);
        assert_eq!(judgement.reasons, vec!["a_reason"]);
    }
}
