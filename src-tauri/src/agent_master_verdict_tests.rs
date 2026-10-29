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
    answer_held: bool,
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
            answer_held: true,
            moment: Moment::JustPosted,
            read: ReadBack {
                marks: Some(vec![mark(TARGET, 8), mark(other(), 3)]),
                masters: Some(vec![
                    master(MasterKind::Ledger, cash(), 9, &["Cash"]),
                    master(MasterKind::Group, debtors_group(), 4, &["Sundry Debtors"]),
                    // The first master change after the intent's mark of 7,
                    // as every measured gateway write was.
                    master(MasterKind::Ledger, new_ledger(), 8, &[NAME]),
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
        world.answer_held = false;
        world.read.marks = Some(vec![mark(TARGET, 7), mark(other(), 3)]);
        world.read.masters = Some(vec![
            master(MasterKind::Ledger, cash(), 9, &["Cash"]),
            master(MasterKind::Group, debtors_group(), 4, &["Sundry Debtors"]),
        ]);
        world.read.party = PartyRead::NotRead;
        world
    }

    /// The clean world read back by a later call whose batch has no counters
    /// on file: Tally's answer was lost.
    fn lost_answer(item: LedgerCreateItem) -> Self {
        let mut world = Self::clean(item);
        world.moment = Moment::Reconcile;
        world.counters = None;
        world.answer_held = false;
        world
    }

    fn new_ledger_alter_id(&mut self, alter_id: u64) {
        self.masters()
            .iter_mut()
            .find(|master| master.guid == new_ledger())
            .unwrap()
            .alter_id = alter_id;
    }

    fn judge(&self) -> Judgement {
        let batch = LedgerCreateBatch::one(self.item.clone());
        judge(
            &Facts {
                batch: &batch,
                company_guid: TARGET,
                pre_image: &self.pre_image,
                pre_marks: &self.pre_marks,
                counters: self.counters,
                answer_held: self.answer_held,
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
            checks.target_mark_step_one,
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
        // The same on a reconcile, while Tally's answer is on file.
        world.moment = Moment::Reconcile;
        assert_eq!(
            world.judge().state,
            MasterVerdictState::ReconciliationRequired,
            "{label} reconcile"
        );
        // With the answer lost it settles, and is still never verified.
        world.answer_held = false;
        assert_eq!(
            world.judge().state,
            MasterVerdictState::PostedUnattributed,
            "{label} lost answer"
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
fn straight_after_the_post_the_target_mark_must_step_by_exactly_one() {
    for after in [7, 6] {
        let mut world = World::clean(suspense());
        world.marks()[0] = mark(TARGET, after);
        assert_doubt(
            &world,
            &format!("{after}"),
            |checks| checks.target_mark_moved,
            "target_mark_not_moved",
        );
        // One reason for the one fact.
        assert!(!world.judge().reasons.contains(&"target_mark_step_not_one"));
    }
    // A step of two: another master change in the company between the aim
    // read and this read-back. The ledger is as sent, but the doubt stands.
    for after in [9, 40] {
        let mut world = World::clean(suspense());
        world.marks()[0] = mark(TARGET, after);
        assert_doubt(
            &world,
            &format!("{after}"),
            |checks| checks.target_mark_step_one,
            "target_mark_step_not_one",
        );
        let judgement = world.judge();
        assert_eq!(judgement.checks.target_mark_moved, Check::Passed);
        assert_eq!(judgement.reasons, vec!["target_mark_step_not_one"]);
        assert!(!judgement.state.is_terminal());
        assert_eq!(judgement.guid.as_deref(), Some(new_ledger()));
    }
}

/// A later read cannot use the step: the book may have moved on since. The
/// ledger's own `ALTERID` decides it, so changes to other masters after the
/// create do not stop a verdict, and a change that came before the create is
/// not forgotten by waiting.
#[test]
fn a_later_read_decides_the_step_by_the_ledgers_alter_id() {
    let mut world = World::clean(suspense());
    world.moment = Moment::Reconcile;
    world.marks()[0] = mark(TARGET, 40);
    assert_verified(&world, "changes to other masters after the create");
    assert_eq!(world.judge().checks.target_mark_step_one, Check::Passed);
    // The step-two case above, read again later: the other change came
    // first, so the ledger is the second change after the intent. Its reason
    // is the ledger's own, not a step that may have been one.
    world.new_ledger_alter_id(9);
    assert_doubt(
        &world,
        "a change before the create",
        |checks| checks.target_mark_step_one,
        "ledger_altered_since_create",
    );
    assert_eq!(world.judge().reasons, vec!["ledger_altered_since_create"]);
    // An ALTERID at or below the intent's mark is not after it at all.
    world.new_ledger_alter_id(7);
    assert_eq!(
        world.judge().reasons,
        vec!["ledger_alter_id_not_after_intent"]
    );
}

/// A clean create that the person then edits in Tally reads, on a later call
/// with Tally's answer on file, as a ledger changed since its create: open,
/// and saying so. Its step straight after the POST was one.
#[test]
fn a_ledger_edited_after_a_clean_create_stays_open_as_altered_since_create() {
    let mut world = World::clean(suspense());
    assert_verified(&world, "straight after the post");
    world.moment = Moment::Reconcile;
    world.marks()[0] = mark(TARGET, 9);
    world.new_ledger_alter_id(9);
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert_eq!(judgement.checks.counters, Check::Passed);
    assert_eq!(
        judgement.checks.first_change_after_intent,
        Check::NotChecked
    );
    assert_eq!(judgement.checks.target_mark_step_one, Check::Failed);
    assert_eq!(judgement.reasons, vec!["ledger_altered_since_create"]);
    assert_eq!(judgement.guid.as_deref(), Some(new_ledger()));
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
        target_mark_step_one: Check::Passed,
        other_marks_unmoved: Check::Passed,
        pre_image_unchanged: Check::Passed,
        first_change_after_intent: Check::NotChecked,
    };
    assert!(all.all_passed());
    // The ALTERID check stands in for the counters only where they are not
    // on file: it is never part of a verified create.
    for value in [Check::Passed, Check::Failed] {
        assert!(Checks {
            first_change_after_intent: value,
            ..all
        }
        .all_passed());
    }
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
            target_mark_step_one: flipped,
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

// --- posted_unattributed ---------------------------------------------------

#[test]
fn a_reconcile_with_no_counters_that_finds_the_first_change_as_sent_is_unattributed() {
    for (label, item) in items() {
        let judgement = World::lost_answer(item).judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::PostedUnattributed,
            "{label}: {judgement:?}"
        );
        assert!(judgement.state.is_terminal(), "{label}");
        assert_eq!(judgement.guid.as_deref(), Some(new_ledger()), "{label}");
        assert_eq!(judgement.reasons, vec!["counters_unavailable"], "{label}");
        assert_eq!(judgement.checks.counters, Check::NotChecked, "{label}");
        assert_eq!(
            judgement.checks.first_change_after_intent,
            Check::Passed,
            "{label}"
        );
        assert!(!judgement.checks.all_passed(), "{label}");
        assert!(judgement.checks.all_but_counters_passed(), "{label}");
    }
}

/// The call that made the POST always holds its answer; the moment alone
/// keeps it out of the state too.
#[test]
fn right_after_a_post_a_create_with_no_counters_is_never_unattributed() {
    for (label, item) in items() {
        let mut world = World::lost_answer(item);
        world.moment = Moment::JustPosted;
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{label}"
        );
        assert_eq!(judgement.checks.first_change_after_intent, Check::Passed);
        assert_eq!(judgement.reasons, vec!["counters_unavailable"], "{label}");
    }
}

/// An answer on file without counters (one whose counters could not all be
/// read) was not lost: it may have been Tally's refusal. The state is not
/// reached and the check is not made.
#[test]
fn an_answer_on_file_without_counters_is_never_unattributed() {
    for (label, item) in items() {
        let mut world = World::lost_answer(item);
        world.answer_held = true;
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{label}"
        );
        assert_eq!(
            judgement.checks.first_change_after_intent,
            Check::NotChecked
        );
        assert_eq!(judgement.reasons, vec!["counters_unavailable"], "{label}");
    }
}

#[test]
fn counters_on_file_decide_and_the_alter_id_check_is_not_made() {
    let mut world = World::lost_answer(suspense());
    world.counters = Some(counters(1, 0, 0, 0));
    world.answer_held = true;
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::PostedVerified);
    assert_eq!(
        judgement.checks.first_change_after_intent,
        Check::NotChecked
    );
    // Counters that fail are never overruled by the mark, even in facts that
    // say no answer is held (the reader never builds those: counters come
    // only from an answer).
    for (failing, answer_held) in [
        counters(0, 0, 0, 0),
        counters(1, 0, 1, 0),
        counters(1, 0, 0, 1),
    ]
    .into_iter()
    .flat_map(|failing| [(failing, true), (failing, false)])
    {
        world.counters = Some(failing);
        world.answer_held = answer_held;
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{failing:?}"
        );
        assert_eq!(
            judgement.checks.first_change_after_intent,
            Check::NotChecked
        );
    }
}

/// A same-name ledger someone made between the aim read and the POST takes
/// the first master change, and the create that alters it moves it one
/// further: the case the counters exist for stays open with its own reason.
#[test]
fn a_ledger_changed_after_the_first_change_stays_open_as_altered_since_create() {
    for (label, item) in items() {
        let mut world = World::lost_answer(item);
        world.new_ledger_alter_id(9);
        world.marks()[0] = mark(TARGET, 9);
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{label}"
        );
        assert!(!judgement.state.is_terminal());
        assert_eq!(
            judgement.checks.first_change_after_intent,
            Check::Failed,
            "{label}"
        );
        assert_eq!(
            judgement.reasons,
            vec!["ledger_altered_since_create", "counters_unavailable"],
            "{label}"
        );
        // The ledger is still named, for the person to compare.
        assert_eq!(judgement.guid.as_deref(), Some(new_ledger()), "{label}");
    }
}

#[test]
fn an_alter_id_not_after_the_intent_mark_stays_open_with_its_own_reason() {
    for alter_id in [0, 6, 7] {
        let mut world = World::lost_answer(suspense());
        world.new_ledger_alter_id(alter_id);
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{alter_id}"
        );
        assert_eq!(judgement.checks.first_change_after_intent, Check::Failed);
        assert_eq!(
            judgement.reasons,
            vec!["ledger_alter_id_not_after_intent", "counters_unavailable"],
            "{alter_id}"
        );
    }
}

#[test]
fn the_first_change_is_exactly_the_mark_plus_one_and_a_last_mark_has_none() {
    assert!(FirstChangeAfterIntent::of(7, 8).is_ok());
    for (mark, alter_id, reason) in [
        (7, 9, "ledger_altered_since_create"),
        (7, u64::MAX, "ledger_altered_since_create"),
        (7, 7, "ledger_alter_id_not_after_intent"),
        (7, 6, "ledger_alter_id_not_after_intent"),
        (u64::MAX, u64::MAX, "ledger_alter_id_not_after_intent"),
        (u64::MAX, 0, "ledger_alter_id_not_after_intent"),
    ] {
        assert_eq!(
            FirstChangeAfterIntent::of(mark, alter_id).err(),
            Some(reason),
            "{mark} {alter_id}"
        );
    }
    assert!(FirstChangeAfterIntent::of(u64::MAX - 1, u64::MAX).is_ok());
}

/// Master changes after the ledger's own, in this company, do not undo what
/// its `ALTERID` shows.
#[test]
fn later_master_changes_in_the_company_do_not_stop_it_settling() {
    let mut world = World::lost_answer(suspense());
    world.marks()[0] = mark(TARGET, 40);
    assert_eq!(world.judge().state, MasterVerdictState::PostedUnattributed);
}

/// Every other check must pass: one failing keeps the batch out of
/// `posted_unattributed`, whatever the ALTERID shows.
#[test]
fn any_other_failing_check_forbids_unattributed() {
    type Change = fn(&mut World);
    let changes: [(&str, Change); 11] = [
        ("parent", |world| {
            world.rows().parent = Some("Cash-in-Hand".into())
        }),
        ("tax fields", |world| {
            world.rows().tax_type = Some("GST".into())
        }),
        ("gstin", |world| {
            world.rows().party_gstin = Some(LAB_GSTIN.into());
        }),
        ("opening balance", |world| {
            world.rows().opening_balance = Some("5.00".into());
        }),
        ("row name", |world| world.rows().name = "Other".into()),
        ("target mark unmoved", |world| {
            world.marks()[0] = mark(TARGET, 7);
        }),
        ("other mark moved", |world| {
            world.marks()[1] = mark(other(), 4)
        }),
        ("companies changed", |world| {
            world.marks().pop();
        }),
        ("pre-image changed", |world| {
            world.masters()[0].alter_id = 10
        }),
        ("second ledger of the name", |world| {
            world.masters().push(master(
                MasterKind::Ledger,
                crate::synthetic_guid(0x5151),
                8,
                &[NAME],
            ));
        }),
        ("party read not made", |world| {
            world.read.party = PartyRead::NotRead
        }),
    ];
    for (label, change) in changes {
        let mut world = World::lost_answer(suspense());
        change(&mut world);
        let judgement = world.judge();
        assert!(
            !matches!(
                judgement.state,
                MasterVerdictState::PostedUnattributed | MasterVerdictState::PostedVerified
            ),
            "{label}: {judgement:?}"
        );
    }
    // A GUID the journal cannot record never settles either.
    let mut world = World::lost_answer(suspense());
    let odd = "not-a-canonical-guid";
    world
        .masters()
        .iter_mut()
        .find(|master| master.guid == new_ledger())
        .unwrap()
        .guid = odd.into();
    world.rows().guid = odd.into();
    assert_ne!(world.judge().state, MasterVerdictState::PostedUnattributed);
}

/// The measured series behind [`FirstChangeAfterIntent`]: on one synthetic
/// book, the company's master mark after each of 21 gateway writes, and the
/// ledger index reads that show each written ledger's `ALTERID`. Every write
/// that changed a ledger moved the mark by exactly one, to that ledger's
/// `ALTERID`; the one refused write moved nothing
/// (`masters-m1-marks/PROVENANCE.md`).
#[test]
fn the_measured_marks_series_has_each_written_ledger_at_its_writes_mark() {
    const LAB: &str = "f107e523-3d6b-4516-bd24-5b1a14498fa6";
    let text = |bytes: &[u8]| {
        String::from_utf16(
            &bytes
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    };
    let marks = [
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-00-baseline.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-01-n-0a.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-02-n-0b.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-03-n-a.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-04-n-b.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-05-n-c.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-06-n-d.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-07-n-f.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-08-n-e.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-09-m1-party.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-10-m1-susp.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-11-m1-duty-cgst.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-12-m1-duty-igst.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-13-m1-duty-state.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-14-m1-duty-sgstutgst.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-15-m1-duty-ut.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-16-m1-duty-cess.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-17-q-0.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-18-q-r.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-19-q-a.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-20-q-c0.utf16le.xml").as_slice(),
        include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/marks-21-q-c.utf16le.xml").as_slice(),
    ]
    .map(|bytes| {
        let rows = crate::agent::change_parse::parse_all_company_marks(&text(bytes)).unwrap();
        assert_eq!(rows.len(), 1, "one company loaded");
        assert_eq!(rows[0].guid, LAB);
        rows[0].masters
    });
    // Each ledger index read with the mark of the write just before it.
    let reads = [
        (
            include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-lab-ledger-index-after-name-cases.utf16le.xml").as_slice(),
            8,
        ),
        (
            include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/ledger-index-after-m1-shapes.utf16le.xml").as_slice(),
            16,
        ),
        (
            include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-m1-marks/ledger-index-after-probe-create.utf16le.xml").as_slice(),
            20,
        ),
        (
            include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/masters-lab-ledger-index-after-fold-cases.utf16le.xml").as_slice(),
            21,
        ),
    ]
    .map(|(bytes, after)| {
        let rows = bridge_tally_protocol::parse_ledger_name_index(&text(bytes), LAB).unwrap();
        let alter_ids = rows
            .into_iter()
            .map(|row| (row.guid, row.alter_id))
            .collect::<BTreeMap<_, _>>();
        // The last write before each read changed a ledger: no ledger is
        // later than the mark, and one is at it.
        assert_eq!(
            alter_ids.values().max(),
            Some(&marks[after]),
            "read {after}"
        );
        (after, alter_ids)
    });
    // Write k (after marks[k - 1], before marks[k]): the ledger it wrote, by
    // the last two hex digits of its GUID, and the read that shows it.
    let writes: [(usize, Option<&str>, usize); 21] = [
        (1, Some("d5"), 8),
        (2, Some("ce"), 8),
        (3, None, 8),
        (4, Some("d0"), 8),
        (5, Some("d1"), 8),
        (6, Some("d2"), 8),
        (7, Some("d3"), 8),
        (8, Some("d6"), 8),
        (9, Some("d7"), 16),
        (10, Some("d8"), 16),
        (11, Some("d9"), 16),
        (12, Some("da"), 16),
        (13, Some("db"), 16),
        (14, Some("dc"), 16),
        (15, Some("dd"), 16),
        (16, Some("de"), 16),
        (17, Some("ce"), 21),
        (18, Some("cf"), 21),
        (19, Some("d5"), 21),
        (20, Some("df"), 20),
        (21, Some("df"), 21),
    ];
    let mut moved = 0;
    for (k, written, read) in writes {
        let (before, after) = (marks[k - 1], marks[k]);
        let Some(suffix) = written else {
            // The alias name refused as a duplicate: nothing moved.
            assert_eq!(after, before, "write {k}");
            continue;
        };
        moved += 1;
        assert_eq!(after, before + 1, "write {k}");
        let alter_ids = &reads.iter().find(|(at, _)| *at == read).unwrap().1;
        let alter_id = alter_ids[&format!("{LAB}-000000{suffix}")];
        assert_eq!(alter_id, after, "write {k}");
        assert!(
            FirstChangeAfterIntent::of(before, alter_id).is_ok(),
            "write {k}"
        );
    }
    assert_eq!(moved, 20);
    assert_eq!((marks[0], marks[21]), (216, 236));
    // The refused write's ledger kept its keyed ALTERID: not after the mark.
    let refused = reads[0].1[&format!("{LAB}-000000cf")];
    assert_eq!(refused, 211);
    assert!(FirstChangeAfterIntent::of(marks[2], refused).is_err());
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
    for (after, reasons) in [
        (
            8,
            vec!["name_not_one_ledger_in_book", "counters_unavailable"],
        ),
        // A step a later read does not judge: no ledger of the name is found.
        (
            9,
            vec!["name_not_one_ledger_in_book", "counters_unavailable"],
        ),
        (
            6,
            vec![
                "name_not_one_ledger_in_book",
                "target_mark_not_moved",
                "counters_unavailable",
            ],
        ),
    ] {
        let mut world = World::untouched(suspense());
        world.marks()[0] = mark(TARGET, after);
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{after}"
        );
        assert_eq!(judgement.reasons, reasons, "{after}");
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
        PostedIntoExistingMaster, PostedUnattributed, PostedVerified, ReconciliationRequired,
    };
    type Case = (
        &'static str,
        World,
        Change,
        MasterVerdictState,
        Vec<&'static str>,
        bool,
    );
    let cases: [Case; 14] = [
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
            "present, no counters, reconcile",
            World::lost_answer(suspense()),
            |_| {},
            PostedUnattributed,
            vec!["counters_unavailable"],
            false,
        ),
        (
            "present, no counters, reconcile, changed since",
            World::lost_answer(suspense()),
            |world| world.new_ledger_alter_id(9),
            ReconciliationRequired,
            vec!["ledger_altered_since_create", "counters_unavailable"],
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
        &LedgerCreateBatch::one(item.clone()),
        &clean.pre_image,
        clean.read.masters.as_ref().unwrap()
    ));
    let untouched = World::untouched(item.clone());
    assert!(!party_read_needed(
        &LedgerCreateBatch::one(item.clone()),
        &untouched.pre_image,
        untouched.read.masters.as_ref().unwrap()
    ));
    // The name on a pre-image master needs no field check: it is already a doubt.
    let mut existing = World::clean(item.clone());
    *existing.masters() = vec![master(MasterKind::Ledger, cash(), 9, &[NAME])];
    assert!(!party_read_needed(
        &LedgerCreateBatch::one(item.clone()),
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
        &LedgerCreateBatch::one(item.clone()),
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

// --- a batch of several ledgers ----------------------------------------------

const BATCH_NAMES: [&str; 3] = ["Batch Alpha", "Batch Bravo", "Batch Charlie"];

fn batch_guid(index: usize) -> &'static str {
    crate::synthetic_guid(0xb000 + u16::try_from(index).unwrap())
}

/// Three Suspense ledgers created together, every part open to change: the
/// mark steps from 7 to 10, and the ledgers take ALTERIDs 8, 9 and 10.
struct BatchWorld {
    batch: LedgerCreateBatch,
    counters: Option<MasterCounters>,
    answer_held: bool,
    moment: Moment,
    read: ReadBack,
}

impl BatchWorld {
    fn clean() -> Self {
        let batch = LedgerCreateBatch::new(
            BATCH_NAMES
                .iter()
                .map(|name| {
                    LedgerCreateItem::new(
                        MasterName::new(name).unwrap(),
                        AdmittedLedgerKind::Suspense,
                    )
                })
                .collect(),
        )
        .unwrap();
        let mut masters = vec![
            master(MasterKind::Ledger, cash(), 9, &["Cash"]),
            master(MasterKind::Group, debtors_group(), 4, &["Sundry Debtors"]),
        ];
        let mut rows = Vec::new();
        for (index, name) in BATCH_NAMES.iter().enumerate() {
            masters.push(master(
                MasterKind::Ledger,
                batch_guid(index),
                8 + index as u64,
                &[name],
            ));
            rows.push(PartyView {
                name: (*name).into(),
                guid: batch_guid(index).into(),
                parent: Some("Suspense A/c".into()),
                tax_type: None,
                duty_head: None,
                party_gstin: None,
                opening_balance: Some("0.00".into()),
            });
        }
        Self {
            batch,
            counters: Some(counters(3, 0, 0, 0)),
            answer_held: true,
            moment: Moment::JustPosted,
            read: ReadBack {
                marks: Some(vec![mark(TARGET, 10), mark(other(), 3)]),
                masters: Some(masters),
                party: PartyRead::Rows(rows),
            },
        }
    }

    fn judge(&self) -> Judgement {
        let pre_image = pre_image();
        let pre_marks = [mark(TARGET, 7), mark(other(), 3)];
        judge(
            &Facts {
                batch: &self.batch,
                company_guid: TARGET,
                pre_image: &pre_image,
                pre_marks: &pre_marks,
                counters: self.counters,
                answer_held: self.answer_held,
                moment: self.moment,
            },
            &self.read,
        )
    }

    fn masters(&mut self) -> &mut Vec<IndexedMaster> {
        self.read.masters.as_mut().unwrap()
    }

    fn target_mark(&mut self, altmstid: u64) {
        self.read.marks.as_mut().unwrap()[0] = mark(TARGET, altmstid);
    }
}

#[test]
fn a_clean_batch_is_posted_verified_with_each_ledgers_guid_in_order() {
    for moment in [Moment::JustPosted, Moment::Reconcile] {
        let mut world = BatchWorld::clean();
        world.moment = moment;
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::PostedVerified,
            "{moment:?}: {judgement:?}"
        );
        assert!(judgement.checks.all_passed());
        assert!(judgement.reasons.is_empty());
        let expected = (0..3)
            .map(|index| Some(batch_guid(index).to_string()))
            .collect::<Vec<_>>();
        assert_eq!(judgement.guids, expected);
        // A batch names no single ledger.
        assert!(judgement.guid.is_none());
        assert_eq!(judgement.guid_of(2), Some(batch_guid(2)));
    }
}

#[test]
fn straight_after_the_post_a_batchs_mark_must_step_by_its_size() {
    for after in [9, 11] {
        let mut world = BatchWorld::clean();
        world.target_mark(after);
        let judgement = world.judge();
        assert_eq!(
            judgement.state,
            MasterVerdictState::ReconciliationRequired,
            "{after}"
        );
        assert_eq!(judgement.checks.target_mark_step_one, Check::Failed);
        assert_eq!(
            judgement.reasons,
            vec!["target_mark_step_not_one"],
            "{after}"
        );
    }
}

#[test]
fn a_later_read_needs_the_batchs_ledgers_to_hold_the_span_after_the_intent() {
    let mut world = BatchWorld::clean();
    world.moment = Moment::Reconcile;
    // Changes after the batch do not stop it settling.
    world.target_mark(40);
    assert_eq!(world.judge().state, MasterVerdictState::PostedVerified);
    // A ledger outside 8 to 10: another change came between the creates.
    world.masters()[3].alter_id = 11;
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert_eq!(judgement.checks.target_mark_step_one, Check::Failed);
}

#[test]
fn counters_must_count_one_create_for_each_ledger() {
    let mut world = BatchWorld::clean();
    world.counters = Some(counters(2, 0, 0, 0));
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement.reasons.contains(&"counters_not_one_created"));
    // An alteration among them is a write into an existing master.
    let mut world = BatchWorld::clean();
    world.counters = Some(counters(2, 1, 0, 0));
    assert_eq!(
        world.judge().state,
        MasterVerdictState::PostedIntoExistingMaster
    );
}

#[test]
fn one_name_on_an_existing_master_makes_the_batch_a_write_into_it_and_names_that_master() {
    let mut world = BatchWorld::clean();
    // Bravo's name now resolves to the pre-image's Cash ledger, as when it
    // was its alias.
    world
        .masters()
        .retain(|master| master.guid != batch_guid(1));
    world.masters()[0].names.push(BATCH_NAMES[1].into());
    world.counters = Some(counters(2, 1, 0, 0));
    let judgement = world.judge();
    assert_eq!(
        judgement.state,
        MasterVerdictState::PostedIntoExistingMaster
    );
    assert_eq!(judgement.guids, vec![None, Some(cash().to_string()), None]);
}

#[test]
fn one_ledger_missing_after_the_post_keeps_the_batch_open() {
    let mut world = BatchWorld::clean();
    world
        .masters()
        .retain(|master| master.guid != batch_guid(2));
    world.target_mark(9);
    world.counters = Some(counters(2, 0, 0, 0));
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement.reasons.contains(&"ledger_absent_after_post"));
    assert_eq!(judgement.guid_of(2), None);
    assert_eq!(judgement.guid_of(0), Some(batch_guid(0)));
}

#[test]
fn a_batch_with_its_answer_lost_is_never_unattributed() {
    let mut world = BatchWorld::clean();
    world.moment = Moment::Reconcile;
    world.counters = None;
    world.answer_held = false;
    let judgement = world.judge();
    assert_eq!(judgement.state, MasterVerdictState::ReconciliationRequired);
    assert!(judgement.reasons.contains(&"counters_unavailable"));
}

#[test]
fn a_batch_is_confirmed_absent_only_when_every_name_is() {
    let mut world = BatchWorld::clean();
    world.moment = Moment::Reconcile;
    world.counters = None;
    world.answer_held = false;
    world.target_mark(7);
    world.read.party = PartyRead::NotRead;
    world.masters().truncate(2);
    assert_eq!(world.judge().state, MasterVerdictState::ConfirmedAbsent);
    // One of the three present: not absent.
    let mut world = BatchWorld::clean();
    world.moment = Moment::Reconcile;
    world.counters = None;
    world.answer_held = false;
    world.target_mark(7);
    world.masters().truncate(3);
    assert_ne!(world.judge().state, MasterVerdictState::ConfirmedAbsent);
}

#[test]
fn the_party_read_is_needed_when_any_ledger_of_the_batch_is_new() {
    let world = BatchWorld::clean();
    let mut masters = world.read.masters.clone().unwrap();
    assert!(party_read_needed(&world.batch, &pre_image(), &masters));
    masters.truncate(3);
    assert!(party_read_needed(&world.batch, &pre_image(), &masters));
    masters.truncate(2);
    assert!(!party_read_needed(&world.batch, &pre_image(), &masters));
}

// --- the lab capture of several creates in one request (L0) ------------------
//
// The values below are read from the answers of lab sheet L0 (branch
// `lab/690-batch-ledger-create-l0-answers` at 078db03e, TallyPrime 7.2 Silver,
// one synthetic company, one sitting): the company GUID, each ledger's GUID and
// `ALTERID`, the marks and Tally's counters. The party rows are not captured:
// they are built as the request sent them.

const L0_COMPANY: &str = "5848d0ca-ed85-4c97-ab20-e045893a9803";

fn l0_guid(suffix: &str) -> String {
    format!("{L0_COMPANY}-{suffix}")
}

fn l0_creditor(name: &str) -> LedgerCreateItem {
    LedgerCreateItem::new(
        MasterName::new(name).unwrap(),
        AdmittedLedgerKind::Party {
            side: PartySide::SundryCreditors,
            gstin: None,
            bill_wise: false,
        },
    )
}

/// The book's ledgers as a step of L0 read them: name, GUID suffix, ALTERID.
fn l0_ledgers(rows: &[(&str, &str, u64)]) -> Vec<IndexedMaster> {
    rows.iter()
        .map(|(name, suffix, alter_id)| {
            master(MasterKind::Ledger, &l0_guid(suffix), *alter_id, &[name])
        })
        .collect()
}

const L0_STEP_0: [(&str, &str, u64); 3] = [
    ("Cash", "0000001f", 32),
    ("LBox Suspense", "000000ce", 208),
    ("Profit & Loss A/c", "0000001e", 31),
];

const L0_STEP_2: [(&str, &str, u64); 6] = [
    ("Cash", "0000001f", 32),
    ("L0 Batch Alpha", "000000cf", 209),
    ("L0 Batch Bravo", "000000d0", 210),
    ("L0 Batch Charlie", "000000d1", 211),
    ("LBox Suspense", "000000ce", 208),
    ("Profit & Loss A/c", "0000001e", 31),
];

const L0_STEP_4: [(&str, &str, u64); 8] = [
    ("Cash", "0000001f", 32),
    ("L0 Batch Alpha", "000000cf", 209),
    ("L0 Batch Bravo", "000000d0", 213),
    ("L0 Batch Charlie", "000000d1", 211),
    ("L0 Batch Delta", "000000d2", 212),
    ("L0 Batch Echo", "000000d3", 214),
    ("LBox Suspense", "000000ce", 208),
    ("Profit & Loss A/c", "0000001e", 31),
];

fn l0_pre_image(rows: &[(&str, &str, u64)]) -> BTreeMap<String, u64> {
    rows.iter()
        .map(|(_, suffix, alter_id)| (l0_guid(suffix), *alter_id))
        .collect()
}

fn l0_rows(names: &[&str], step: &[(&str, &str, u64)]) -> Vec<PartyView> {
    names
        .iter()
        .map(|name| {
            let (_, suffix, _) = step.iter().find(|(row, _, _)| row == name).unwrap();
            PartyView {
                name: (*name).into(),
                guid: l0_guid(suffix),
                parent: Some("Sundry Creditors".into()),
                tax_type: None,
                duty_head: None,
                party_gstin: None,
                opening_balance: Some("0.00".into()),
            }
        })
        .collect()
}

fn l0_judge(
    names: &[&str],
    before: (&[(&str, &str, u64)], u64),
    after: (&[(&str, &str, u64)], u64),
    counters: MasterCounters,
    moment: Moment,
) -> Judgement {
    let batch =
        LedgerCreateBatch::new(names.iter().map(|name| l0_creditor(name)).collect()).unwrap();
    let pre_image = l0_pre_image(before.0);
    let pre_marks = [mark(L0_COMPANY, before.1)];
    judge(
        &Facts {
            batch: &batch,
            company_guid: L0_COMPANY,
            pre_image: &pre_image,
            pre_marks: &pre_marks,
            counters: Some(counters),
            answer_held: true,
            moment,
        },
        &ReadBack {
            marks: Some(vec![mark(L0_COMPANY, after.1)]),
            masters: Some(l0_ledgers(after.0)),
            party: PartyRead::Rows(l0_rows(names, after.0)),
        },
    )
}

/// L0's request A: three new ledgers in one request counted CREATED 3, the
/// mark stepped 208 to 211, and the ledgers took 209 to 211 in request order.
/// The batch reads `posted_verified`, straight after the send and later.
#[test]
fn the_captured_batch_of_three_new_ledgers_is_posted_verified() {
    let names = ["L0 Batch Alpha", "L0 Batch Bravo", "L0 Batch Charlie"];
    for moment in [Moment::JustPosted, Moment::Reconcile] {
        let judgement = l0_judge(
            &names,
            (&L0_STEP_0, 208),
            (&L0_STEP_2, 211),
            counters(3, 0, 0, 0),
            moment,
        );
        assert_eq!(
            judgement.state,
            MasterVerdictState::PostedVerified,
            "{moment:?}: {judgement:?}"
        );
        assert_eq!(
            judgement.guids,
            ["000000cf", "000000d0", "000000d1"].map(|suffix| Some(l0_guid(suffix)))
        );
    }
}

/// L0's request B: Delta, Bravo (already a ledger) and Echo in one request
/// counted CREATED 2, ALTERED 1. Bravo kept its GUID and its ALTERID went 210
/// to 213 with no field changed; the mark stepped 211 to 214, each master
/// taking the next value in request order. The batch is a write into an
/// existing master, and the verdict names Bravo against its item.
#[test]
fn the_captured_batch_with_one_existing_name_is_a_write_into_that_ledger_naming_it() {
    let judgement = l0_judge(
        &["L0 Batch Delta", "L0 Batch Bravo", "L0 Batch Echo"],
        (&L0_STEP_2, 211),
        (&L0_STEP_4, 214),
        counters(2, 1, 0, 0),
        Moment::JustPosted,
    );
    assert_eq!(
        judgement.state,
        MasterVerdictState::PostedIntoExistingMaster
    );
    assert_eq!(judgement.guids, vec![None, Some(l0_guid("000000d0")), None]);
    assert!(judgement.reasons.contains(&"tally_reported_altered"));
    assert!(judgement
        .reasons
        .contains(&"name_resolves_to_existing_master"));
    // Bravo's alteration moved its ALTERID although no field changed.
    assert_eq!(judgement.checks.pre_image_unchanged, Check::Failed);
    // The step was the batch's size: the alteration took a mark value too.
    assert_eq!(judgement.checks.target_mark_step_one, Check::Passed);
}

/// L0b: ten new ledgers in one request counted CREATED 10, the mark stepped
/// 208 to 218, and the ledgers took 209 to 218 in request order (branch
/// `lab/690-batch-ledger-create-l0b-answers` at 5efb36d4, the same company).
#[test]
fn the_captured_batch_of_ten_new_ledgers_is_posted_verified() {
    let names: Vec<String> = (1..=10).map(|n| format!("L0b Batch {n:02}")).collect();
    let suffixes: Vec<String> = (0xcf..=0xd8).map(|n| format!("{n:08x}")).collect();
    let mut after: Vec<(&str, &str, u64)> = L0_STEP_0.to_vec();
    for (index, (name, suffix)) in names.iter().zip(&suffixes).enumerate() {
        after.push((name.as_str(), suffix.as_str(), 209 + index as u64));
    }
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    for moment in [Moment::JustPosted, Moment::Reconcile] {
        let judgement = l0_judge(
            &names,
            (&L0_STEP_0, 208),
            (&after, 218),
            counters(10, 0, 0, 0),
            moment,
        );
        assert_eq!(
            judgement.state,
            MasterVerdictState::PostedVerified,
            "{moment:?}: {judgement:?}"
        );
        assert_eq!(
            judgement.guids,
            suffixes
                .iter()
                .map(|suffix| Some(l0_guid(suffix)))
                .collect::<Vec<_>>()
        );
    }
}
