//! The verdict on one master write, decided over what was read back after the
//! POST. Pure: it reads nothing and sends nothing, so every state and every
//! failing check is tested on constructed inputs.
//!
//! The inputs are parsed values, never text: the company marks, the paired
//! ledger and group index as one list of masters, the party-ledger master rows
//! of the item's parent, the pre-image the intent named, the marks the intent
//! recorded, and Tally's counters when it reported them.
//!
//! **No client value reaches the verdict except the item the person
//! approved.** [`PartyView`] has no field for a PAN, a bank detail, an e-mail
//! address, a phone number or an address, so the parser's richer row is
//! projected onto it at the boundary ([`PartyView::project`]) and nothing else
//! of the row survives to be put in an answer or a journal record.
//!
//! Order of decision, first match wins:
//!
//! 1. a read-back unavailable: `masters_after_post_unconfirmed`;
//! 2. a pre-image master's `ALTERID` changed, `ALTERED` above zero, or the name
//!    resolves to a pre-image master: `posted_into_existing_master`;
//! 3. another loaded company's `ALTMSTID` moved:
//!    `master_write_located_elsewhere`;
//! 4. on a reconcile only, the name absent under every key of the index, no pre-image
//!    master changed or missing, no loaded company's mark moved and no counter
//!    claiming a create: `confirmed_absent`;
//! 5. every check of [`Checks`] passed: `posted_verified`;
//! 6. anything else: `reconciliation_required`, with its reasons.
use super::super::master_ledger::{
    is_canonical_guid, MasterCounters, MasterMark, MasterVerdictState,
};
use crate::tally::ledger_create_item::{AdmittedLedgerKind, LedgerCreateItem};
use bridge_tally_core::master_name_index::{
    IndexedMaster, MasterKind, MasterNameIndex, NameCollision, Via,
};
use bridge_tally_protocol::{GstDutyHeadObservation, ParsedSourceRecord, PartyLedgerMasterRecord};
use serde::Serialize;
use std::collections::BTreeMap;

/// When the verdict is asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Moment {
    /// Straight after the POST answered: a ledger missing now is not yet
    /// "absent", it is a doubt for a person.
    JustPosted,
    /// A later call for a dispatched batch (or one whose POST answer was lost).
    /// Only here can a ledger be confirmed absent.
    Reconcile,
}

/// The fields of a party-ledger master row that the verdict reads, and no
/// others. A row of the parser also holds PAN, bank details, e-mail, phone and
/// address; none of them is copied here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PartyView {
    pub(super) name: String,
    /// Lower-case.
    pub(super) guid: String,
    pub(super) parent: Option<String>,
    pub(super) tax_type: Option<String>,
    /// The duty head exactly as Tally spelled it.
    pub(super) duty_head: Option<String>,
    pub(super) party_gstin: Option<String>,
    pub(super) opening_balance: Option<String>,
}

impl PartyView {
    /// Projects one parsed row. A row without a GUID cannot be matched to a
    /// ledger and is dropped by the caller, never guessed.
    pub(super) fn project(row: &ParsedSourceRecord<PartyLedgerMasterRecord>) -> Option<Self> {
        let guid = row.identities.guid.as_deref()?.to_ascii_lowercase();
        let ledger = &row.record.ledger;
        let fields = &row.record.fields;
        let duty_head = match &fields.gst_duty_head {
            GstDutyHeadObservation::Recognized { raw, .. }
            | GstDutyHeadObservation::Unrecognized { raw }
            | GstDutyHeadObservation::Contradictory { raw, .. } => Some(raw.clone()),
            GstDutyHeadObservation::NotTaxLedger { .. } | GstDutyHeadObservation::Absent => None,
        };
        Some(Self {
            name: ledger.name.clone(),
            guid,
            parent: ledger.parent.nonempty_returned_text().map(str::to_owned),
            tax_type: fields.tax_type.returned_text().map(str::to_owned),
            duty_head,
            party_gstin: ledger.party_gstin.returned_text().map(str::to_owned),
            opening_balance: ledger.opening_balance.clone(),
        })
    }
}

/// The party-ledger master read of the item's parent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum PartyRead {
    /// It was needed and could not be read or parsed.
    Unavailable,
    /// It was not made: the paired index showed no ledger it could describe.
    NotRead,
    Rows(Vec<PartyView>),
}

/// What was read back. `None` is an unavailable read, never an empty one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ReadBack {
    pub(super) marks: Option<Vec<MasterMark>>,
    /// The ledgers and the groups of the paired index, as one list.
    pub(super) masters: Option<Vec<IndexedMaster>>,
    pub(super) party: PartyRead,
}

/// What the journal and the intent already hold.
#[derive(Clone, Debug)]
pub(super) struct Facts<'a> {
    pub(super) item: &'a LedgerCreateItem,
    /// Lower-case.
    pub(super) company_guid: &'a str,
    /// Every master's lower-case GUID with its `ALTERID`, before the POST.
    pub(super) pre_image: &'a BTreeMap<String, u64>,
    /// Every loaded company's `ALTMSTID`, before the POST.
    pub(super) pre_marks: &'a [MasterMark],
    /// Tally's counters when it reported all of them.
    pub(super) counters: Option<MasterCounters>,
    pub(super) moment: Moment,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Check {
    Passed,
    Failed,
    /// Not decidable from what was read, or not applicable.
    NotChecked,
}

/// Each check of `posted_verified`, as the answer shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(super) struct Checks {
    /// Exactly one ledger holds the item name, stored byte for byte.
    pub(super) name: Check,
    /// That ledger's GUID is not in the pre-image.
    pub(super) guid_new: Check,
    pub(super) parent: Check,
    /// TAXTYPE and GSTDUTYHEAD as sent (absent when not sent).
    pub(super) tax_fields: Check,
    pub(super) party_gstin: Check,
    pub(super) opening_balance: Check,
    /// CREATED=1, ALTERED=0, ERRORS=0, EXCEPTIONS=0. `NotChecked` when Tally
    /// reported no counters: that can never be `posted_verified`, because a
    /// same-name master created between the aim read and the POST would be
    /// overwritten by the Create and every other check would still pass.
    pub(super) counters: Check,
    pub(super) target_mark_moved: Check,
    pub(super) other_marks_unmoved: Check,
    pub(super) pre_image_unchanged: Check,
}

impl Checks {
    pub(super) const fn none() -> Self {
        Self {
            name: Check::NotChecked,
            guid_new: Check::NotChecked,
            parent: Check::NotChecked,
            tax_fields: Check::NotChecked,
            party_gstin: Check::NotChecked,
            opening_balance: Check::NotChecked,
            counters: Check::NotChecked,
            target_mark_moved: Check::NotChecked,
            other_marks_unmoved: Check::NotChecked,
            pre_image_unchanged: Check::NotChecked,
        }
    }

    /// Whether the write is whole: every check passed, Tally's counters
    /// included. Counters Tally did not report leave the write unverified.
    pub(super) fn all_passed(&self) -> bool {
        let passed = |check: Check| check == Check::Passed;
        passed(self.name)
            && passed(self.guid_new)
            && passed(self.parent)
            && passed(self.tax_fields)
            && passed(self.party_gstin)
            && passed(self.opening_balance)
            && passed(self.counters)
            && passed(self.target_mark_moved)
            && passed(self.other_marks_unmoved)
            && passed(self.pre_image_unchanged)
    }
}

/// A verdict on a sent ledger Create. Only [`judge`], on what was read back,
/// builds one in a terminal state; [`Judgement::open`] builds the two open
/// states a caller may give without a read-back. The private field keeps any
/// other module from writing one by hand, and the journal builds a verdict
/// only from a `Judgement` ([`super::super::master_ledger::MasterVerdict::judged`]),
/// so the record that releases a held name always rests on a read-back (P2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::agent::agent_import) struct Judgement {
    state: MasterVerdictState,
    /// The ledger's GUID, when the verdict names one.
    guid: Option<String>,
    checks: Checks,
    /// Static reasons, carrying no book data, for any verdict other than `posted_verified`.
    reasons: Vec<&'static str>,
    /// A fact about the read, not a verdict: the paired index was read and was
    /// usable, and the item name collides with no master in it under any key.
    /// False where the index was not read or not usable.
    name_absent_in_read_book: bool,
    sealed: (),
}

/// The verdicts a caller may give without a read-back: neither settles the
/// batch, so neither releases the name it holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OpenVerdict {
    ReconciliationRequired,
    MastersAfterPostUnconfirmed,
}

impl Judgement {
    pub(in crate::agent::agent_import) fn state(&self) -> MasterVerdictState {
        self.state
    }

    pub(in crate::agent::agent_import) fn guid(&self) -> Option<&str> {
        self.guid.as_deref()
    }

    pub(super) fn checks(&self) -> &Checks {
        &self.checks
    }

    pub(in crate::agent::agent_import) fn reasons(&self) -> &[&'static str] {
        &self.reasons
    }

    pub(super) fn name_absent_in_read_book(&self) -> bool {
        self.name_absent_in_read_book
    }

    /// An open verdict, with nothing read back: no GUID, no check passed.
    pub(super) fn open(verdict: OpenVerdict, reason: &'static str) -> Self {
        Self {
            state: match verdict {
                OpenVerdict::ReconciliationRequired => MasterVerdictState::ReconciliationRequired,
                OpenVerdict::MastersAfterPostUnconfirmed => {
                    MasterVerdictState::MastersAfterPostUnconfirmed
                }
            },
            guid: None,
            checks: Checks::none(),
            reasons: vec![reason],
            name_absent_in_read_book: false,
            sealed: (),
        }
    }

    /// Any judgement, for a test that needs one [`judge`] would not give.
    #[cfg(test)]
    pub(super) fn for_test(
        state: MasterVerdictState,
        guid: Option<String>,
        checks: Checks,
        reasons: Vec<&'static str>,
        name_absent_in_read_book: bool,
    ) -> Self {
        Self {
            state,
            guid,
            checks,
            reasons,
            name_absent_in_read_book,
            sealed: (),
        }
    }
}

/// Where the item name stands in the paired index.
struct Located {
    index: MasterNameIndex,
    collisions: Vec<NameCollision>,
    /// The lower-case GUID of the one ledger whose stored name is the item
    /// name, when the item name collides with that ledger alone.
    sole_exact_ledger: Option<String>,
    /// How many masters the item name collides with: masters, not spellings,
    /// since one ledger can match through two of its own spellings.
    masters_hit: usize,
}

fn locate(item: &LedgerCreateItem, masters: &[IndexedMaster]) -> Option<Located> {
    let index = MasterNameIndex::new(masters.to_vec()).ok()?;
    let collisions = index.collisions(item.name().as_str()).ok()?;
    let exact = collisions
        .iter()
        .filter(|hit| {
            hit.kind == MasterKind::Ledger
                && hit.via == Via::Name
                && hit.name == item.name().as_str()
        })
        .collect::<Vec<_>>();
    let masters_hit = collisions
        .iter()
        .map(|hit| hit.guid.to_ascii_lowercase())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let sole_exact_ledger = match (masters_hit, exact.as_slice()) {
        (1, [hit]) => Some(hit.guid.to_ascii_lowercase()),
        _ => None,
    };
    Some(Located {
        index,
        collisions,
        sole_exact_ledger,
        masters_hit,
    })
}

/// Whether the party-ledger master read has anything to describe: the paired
/// index shows one ledger holding the item name that the pre-image did not.
/// The reader asks this before it makes the read, so a read that no verdict
/// would use is not made.
pub(super) fn party_read_needed(
    item: &LedgerCreateItem,
    pre_image: &BTreeMap<String, u64>,
    masters: &[IndexedMaster],
) -> bool {
    locate(item, masters)
        .and_then(|located| located.sole_exact_ledger)
        .is_some_and(|guid| !pre_image.contains_key(&guid))
}

/// Whether `text` is no amount or an amount of zero: every digit zero.
pub(super) fn is_zero_amount(text: &str) -> bool {
    let text = text.trim();
    if text.is_empty() {
        return true;
    }
    let amount = text.strip_prefix(['-', '+']).unwrap_or(text);
    amount.matches('.').count() <= 1
        && amount.contains('0')
        && amount.chars().all(|c| c == '0' || c == '.')
}

fn counters_check(counters: Option<MasterCounters>) -> Check {
    match counters {
        None => Check::NotChecked,
        Some(counters) => passed(
            counters.created == 1
                && counters.altered == 0
                && counters.errors == 0
                && counters.exceptions == 0,
        ),
    }
}

fn passed(ok: bool) -> Check {
    if ok {
        Check::Passed
    } else {
        Check::Failed
    }
}

/// The party-row checks of the one ledger found.
fn party_checks(item: &LedgerCreateItem, row: &PartyView, checks: &mut Checks) {
    checks.parent = passed(row.parent.as_deref() == Some(item.parent()));
    let absent = |value: &Option<String>| value.as_deref().is_none_or(str::is_empty);
    checks.tax_fields = match item.kind() {
        AdmittedLedgerKind::GstDuty { head } => passed(
            row.tax_type.as_deref() == Some("GST")
                && row.duty_head.as_deref() == Some(head.head().tally_spelling()),
        ),
        AdmittedLedgerKind::Party { .. } | AdmittedLedgerKind::Suspense => {
            passed(absent(&row.duty_head) && row.tax_type.as_deref() != Some("GST"))
        }
    };
    checks.party_gstin = match item.kind() {
        AdmittedLedgerKind::Party {
            gstin: Some(gstin), ..
        } => passed(row.party_gstin.as_deref() == Some(gstin.as_str())),
        AdmittedLedgerKind::Party { gstin: None, .. }
        | AdmittedLedgerKind::Suspense
        | AdmittedLedgerKind::GstDuty { .. } => passed(absent(&row.party_gstin)),
    };
    checks.opening_balance = passed(row.opening_balance.as_deref().is_none_or(is_zero_amount));
}

/// The reason each failed check gives, except the pre-image's, which `judge`
/// words itself (a changed master and a missing one are different doubts).
fn reasons_of(checks: &Checks) -> Vec<&'static str> {
    [
        (checks.name, "name_not_one_ledger_in_book"),
        (checks.guid_new, "ledger_guid_in_pre_image"),
        (checks.parent, "parent_differs_from_sent"),
        (checks.tax_fields, "tax_fields_differ_from_sent"),
        (checks.party_gstin, "party_gstin_differs_from_sent"),
        (checks.opening_balance, "opening_balance_not_zero"),
        (checks.counters, "counters_not_one_created"),
        (checks.target_mark_moved, "target_mark_not_moved"),
        (checks.other_marks_unmoved, "other_company_mark_moved"),
    ]
    .into_iter()
    .filter(|(check, _)| *check == Check::Failed)
    .map(|(_, reason)| reason)
    .collect()
}

/// Decides the verdict. See the module documentation for the order.
pub(super) fn judge(facts: &Facts<'_>, read: &ReadBack) -> Judgement {
    let mut checks = Checks::none();
    checks.counters = counters_check(facts.counters);
    let mut reasons = Vec::<&'static str>::new();

    // 1. A read-back that could not be made leaves the outcome open.
    if read.marks.is_none() {
        reasons.push("company_marks_unavailable");
    }
    if read.masters.is_none() {
        reasons.push("master_index_unavailable");
    }
    if read.party == PartyRead::Unavailable {
        reasons.push("party_read_unavailable");
    }
    let (Some(now_marks), Some(masters), false) = (
        &read.marks,
        &read.masters,
        read.party == PartyRead::Unavailable,
    ) else {
        return Judgement {
            state: MasterVerdictState::MastersAfterPostUnconfirmed,
            guid: None,
            checks,
            reasons,
            name_absent_in_read_book: false,
            sealed: (),
        };
    };

    // The marks: who moved, and whether the set of companies is the same.
    let pre = facts
        .pre_marks
        .iter()
        .map(|mark| (mark.company_guid.as_str(), mark.altmstid))
        .collect::<BTreeMap<_, _>>();
    let now = now_marks
        .iter()
        .map(|mark| (mark.company_guid.as_str(), mark.altmstid))
        .collect::<BTreeMap<_, _>>();
    let same_companies = now.len() == now_marks.len()
        && pre.len() == facts.pre_marks.len()
        && pre.keys().eq(now.keys());
    if !same_companies {
        reasons.push("company_scope_changed");
    }
    let others_moved = pre
        .iter()
        .filter(|(guid, _)| **guid != facts.company_guid)
        .any(|(guid, before)| now.get(guid).is_some_and(|after| after != before));
    let target_marks = pre.get(facts.company_guid).zip(now.get(facts.company_guid));
    if let Some((before, after)) = target_marks {
        checks.target_mark_moved = passed(after > before);
    }
    if others_moved {
        checks.other_marks_unmoved = Check::Failed;
    } else if same_companies {
        checks.other_marks_unmoved = Check::Passed;
    }
    let target_moved = target_marks.is_some_and(|(before, after)| before != after);

    // The index: the pre-image across the POST, and where the name stands.
    let Some(located) = locate(facts.item, masters) else {
        reasons.push("master_index_not_usable");
        return Judgement {
            state: MasterVerdictState::ReconciliationRequired,
            guid: None,
            checks,
            reasons,
            name_absent_in_read_book: false,
            sealed: (),
        };
    };
    let name_absent = located.collisions.is_empty();
    let current = located.index.pre_image();
    let mut changed = false;
    let mut missing = false;
    for (guid, before) in facts.pre_image {
        match current.get(guid) {
            Some(after) if after != before => changed = true,
            None => missing = true,
            Some(_) => {}
        }
    }
    checks.pre_image_unchanged = passed(!changed && !missing);
    if changed {
        reasons.push("pre_image_master_changed");
    }
    if missing {
        reasons.push("pre_image_master_missing");
    }
    checks.name = passed(located.sole_exact_ledger.is_some());
    checks.guid_new = located
        .sole_exact_ledger
        .as_ref()
        .map_or(Check::NotChecked, |guid| {
            passed(!facts.pre_image.contains_key(guid))
        });
    let into_pre_image = located
        .collisions
        .iter()
        .find(|hit| facts.pre_image.contains_key(&hit.guid.to_ascii_lowercase()));
    let altered = facts.counters.is_some_and(|counters| counters.altered > 0);

    // 2. Something that existed before was written to, by this create: Tally
    // counted an alteration, or the name resolves to a master that existed
    // before. A pre-image master that changed while nothing ties the change
    // to this create (another person's edit during the window) is not called
    // a write into it: it fails the pre-image check and is left to a person
    // below, holding the name, and it never reads absent (step 4).
    if altered || into_pre_image.is_some() {
        if altered {
            reasons.push("tally_reported_altered");
        }
        if into_pre_image.is_some() {
            reasons.push("name_resolves_to_existing_master");
        }
        return Judgement {
            state: MasterVerdictState::PostedIntoExistingMaster,
            guid: into_pre_image
                .map(|hit| hit.guid.to_ascii_lowercase())
                .filter(|guid| is_canonical_guid(guid)),
            checks,
            reasons,
            name_absent_in_read_book: name_absent,
            sealed: (),
        };
    }

    // 3. Another loaded company's master mark moved.
    if others_moved {
        reasons.push("other_company_mark_moved");
        return Judgement {
            state: MasterVerdictState::MasterWriteLocatedElsewhere,
            guid: None,
            checks,
            reasons,
            name_absent_in_read_book: name_absent,
            sealed: (),
        };
    }

    // 4. After an unknown outcome: nothing was written anywhere we can see.
    let counters_claim_a_write = facts
        .counters
        .is_some_and(|counters| counters.created > 0 || counters.altered > 0);
    if facts.moment == Moment::Reconcile
        && name_absent
        && !changed
        && !missing
        && same_companies
        && target_marks.is_some()
        && !target_moved
        && !counters_claim_a_write
    {
        return Judgement {
            state: MasterVerdictState::ConfirmedAbsent,
            guid: None,
            checks,
            reasons: vec!["not_in_book"],
            name_absent_in_read_book: name_absent,
            sealed: (),
        };
    }

    // 5. The ledger the write should have made, described field by field.
    let found = located.sole_exact_ledger.clone();
    if let Some(guid) = &found {
        match &read.party {
            PartyRead::Rows(rows) => {
                let mut matching = rows.iter().filter(|row| &row.guid == guid);
                match (matching.next(), matching.next()) {
                    (Some(row), None) => {
                        // The row's own name must be the one the index stored.
                        if row.name != facts.item.name().as_str() {
                            checks.name = Check::Failed;
                        }
                        party_checks(facts.item, row, &mut checks);
                    }
                    (None, _) => reasons.push("ledger_missing_from_parent_read"),
                    (Some(_), Some(_)) => reasons.push("ledger_repeated_in_parent_read"),
                }
            }
            PartyRead::NotRead => reasons.push("party_read_not_made"),
            PartyRead::Unavailable => {}
        }
    }
    let recordable = found.as_deref().is_some_and(is_canonical_guid);
    if found.is_some() && !recordable {
        reasons.push("ledger_guid_not_recordable");
    }
    if checks.all_passed() && recordable {
        return Judgement {
            state: MasterVerdictState::PostedVerified,
            guid: found,
            checks,
            reasons: Vec::new(),
            name_absent_in_read_book: name_absent,
            sealed: (),
        };
    }

    // 6. Anything else is for a person.
    reasons.extend(reasons_of(&checks));
    if checks.counters == Check::NotChecked {
        reasons.push("counters_unavailable");
    }
    if facts.moment == Moment::JustPosted && name_absent {
        reasons.push("ledger_absent_after_post");
    }
    if located.masters_hit > 1 {
        reasons.push("name_collides_with_more_than_one_master");
    }
    if reasons.is_empty() {
        reasons.push("unclassified");
    }
    Judgement {
        state: MasterVerdictState::ReconciliationRequired,
        guid: found.filter(|guid| is_canonical_guid(guid)),
        checks,
        reasons,
        name_absent_in_read_book: name_absent,
        sealed: (),
    }
}

#[cfg(test)]
#[path = "agent_master_verdict_tests.rs"]
mod tests;
