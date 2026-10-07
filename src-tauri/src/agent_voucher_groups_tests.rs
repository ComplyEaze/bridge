//! Group placement over a live capture (#1230). The ledger catalogue and the group snapshot of the
//! synthetic book `BRIDGE SHAPE LAB` are the answers Tally gave on 6 Oct 2026, and the requests are
//! the bytes the read code sent (see `shape-lab-group-chain.PROVENANCE.md`); they go through the
//! production parsers. A test that needs a case the book lacks (a renamed predefined group, a missing
//! group, a repeated name, a cycle) edits the parsed groups and says so: those are derived, not captured.
use super::*;
use crate::tally::standard_ledger_catalog::parse_standard_ledger_catalog_response;
use bridge_tally_protocol::native_outstandings::parse_native_group_snapshot;
use bridge_tally_protocol::PartyLedgerMasterFieldObservation;
use serde_json::Value;

const COMPANY: &str = "BRIDGE SHAPE LAB";
const COMPANY_GUID: &str = "3a6bd6e1-b835-4bff-89dd-8a6af138c346";
const CATALOGUE_REQUEST: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/shape-lab-standard-ledger-catalogue.request.utf16le.xml"
);
const CATALOGUE: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/shape-lab-standard-ledger-catalogue.utf16le.xml"
);
const GROUPS_REQUEST: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/shape-lab-group-snapshot.request.utf16le.xml"
);
const GROUPS: &[u8] = include_bytes!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/shape-lab-group-snapshot.utf16le.xml"
);
const TRIAL_BALANCE: &str = include_str!(
    "../crates/bridge-tally-protocol/tests/fixtures/agent/shape-lab-fy.trial-balance-ledgers.json"
);

fn utf16(bytes: &[u8]) -> String {
    let bytes = bytes.strip_prefix(&[0xff, 0xfe]).unwrap_or(bytes);
    String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>(),
    )
    .expect("UTF-16LE")
}

fn catalogue() -> bridge_tally_protocol::StandardLedgerCatalog {
    parse_standard_ledger_catalog_response(&utf16(CATALOGUE), COMPANY, COMPANY_GUID)
        .expect("the captured catalogue parses")
}

fn groups() -> Vec<TallyNamedMaster> {
    parse_native_group_snapshot(&utf16(GROUPS), COMPANY_GUID)
        .expect("the captured group snapshot parses")
}

/// The placements of the captured live book, for the summary tests.
pub(in crate::agent) fn live_placements() -> Placements {
    placements_of(groups())
}

/// The captured live book's ledgers with their parents, and its groups, for the summary tests.
pub(in crate::agent) fn live_parents() -> Vec<(String, Option<String>)> {
    catalogue()
        .parents()
        .map(|(ledger, parent)| (ledger.to_string(), parent.map(str::to_string)))
        .collect()
}

pub(in crate::agent) fn live_groups() -> Vec<TallyNamedMaster> {
    groups()
}

fn placements_of(groups: Vec<TallyNamedMaster>) -> Placements {
    let catalogue = catalogue();
    Placements::build(catalogue.parents(), groups)
}

fn names(placement: &Placement) -> Vec<(&str, &str)> {
    placement
        .chain
        .iter()
        .map(|hop| (hop.name.as_str(), hop.reserved_name.as_str()))
        .collect()
}

fn primary(key: &str, name: &str, reserved: Option<&str>) -> Primary {
    Primary {
        key: key.to_string(),
        name: name.to_string(),
        reserved_name: reserved.map(str::to_string),
    }
}

fn placed<'a>(placements: &'a Placements, ledger: &str) -> &'a Placement {
    placements
        .get(ledger)
        .unwrap_or_else(|| panic!("no placement for {ledger}"))
        .as_ref()
        .unwrap_or_else(|gap| panic!("{ledger} has a gap: {gap}"))
}

#[test]
fn the_captured_requests_are_what_the_read_code_renders() {
    // The captured bytes are the requests as sent: the same text the read code renders now.
    let catalogue = crate::agent::read_profiles::standard_ledger_catalog_read(COMPANY).unwrap();
    assert_eq!(utf16(CATALOGUE_REQUEST), catalogue.as_str());
    let groups = crate::agent::read_profiles::native_group_snapshot_read(COMPANY);
    assert_eq!(utf16(GROUPS_REQUEST), groups.as_str());
}

#[test]
fn every_ledger_of_the_live_catalogue_is_placed_under_a_complete_chain() {
    let (catalogue, placements) = (catalogue(), placements_of(groups()));
    assert_eq!(placements.len(), 44);
    assert_eq!(catalogue.parents().count(), 44);
    for (ledger, _) in catalogue.parents() {
        assert!(matches!(placements.get(ledger), Some(Ok(_))), "{ledger}");
    }
}

#[test]
fn live_chains_run_through_user_made_groups_to_the_group_under_the_root() {
    let placements = placements_of(groups());
    // A user group (empty reserved name) under a predefined group under a primary group.
    let buyer = placed(&placements, "Shape Buyer 1");
    assert_eq!(
        names(buyer),
        vec![
            ("Trade Debtors - Local", ""),
            ("Sundry Debtors", "Sundry Debtors"),
            ("Current Assets", "Current Assets"),
        ]
    );
    assert_eq!(buyer.group_name(), "Trade Debtors - Local");
    assert_eq!(buyer.group_reserved_name(), Some(""));
    assert_eq!(
        buyer.primary(),
        primary(
            "reserved:Current Assets",
            "Current Assets",
            Some("Current Assets")
        )
    );
    // Two user groups deep.
    let power = placed(&placements, "Power Charges");
    assert_eq!(
        names(power),
        vec![
            ("Power and Fuel", ""),
            ("Factory Overheads", ""),
            ("Indirect Expenses", "Indirect Expenses"),
        ]
    );
    assert_eq!(
        power.primary(),
        primary(
            "reserved:Indirect Expenses",
            "Indirect Expenses",
            Some("Indirect Expenses")
        )
    );
    // A predefined group directly under a primary group.
    let tax = placed(&placements, "CGST");
    assert_eq!(
        names(tax),
        vec![
            ("Duties & Taxes", "Duties & Taxes"),
            ("Current Liabilities", "Current Liabilities")
        ]
    );
    let bank = placed(&placements, "Bank of Baroda CA");
    assert_eq!(bank.group_name(), "Bank Accounts");
    assert_eq!(bank.primary().key, "reserved:Current Assets");
    // A primary group itself.
    assert_eq!(
        names(placed(&placements, "Sales - Local")),
        vec![("Sales Accounts", "Sales Accounts")]
    );
}

#[test]
fn a_ledger_directly_under_the_reserved_root_is_placed_under_the_root_itself() {
    let placements = placements_of(groups());
    let profit = placed(&placements, "Profit & Loss A/c");
    assert!(profit.chain.is_empty());
    assert_eq!(profit.root_label.as_deref(), Some("Primary"));
    assert_eq!(profit.group_name(), "Primary");
    assert_eq!(profit.primary(), primary("root", "Primary", None));
    assert_eq!(profit.group_reserved_name(), None);
}

#[test]
fn the_catalogue_and_the_trial_balance_agree_on_every_ledgers_immediate_group() {
    let placements = placements_of(groups());
    let rows: Vec<Value> = serde_json::from_str(TRIAL_BALANCE).unwrap();
    assert_eq!(rows.len(), 44);
    for row in &rows {
        let ledger = row["ledger"].as_str().unwrap();
        let parent = row["parent"].as_str().unwrap();
        let expected = parent
            .strip_prefix(TALLY_SANITIZED_ROOT_MARKER)
            .map_or(parent, str::trim);
        assert_eq!(
            placed(&placements, ledger).group_name(),
            expected,
            "{ledger}"
        );
    }
}

#[test]
fn a_renamed_predefined_group_keeps_its_bucket_under_its_reserved_name() {
    // Derived: the live group snapshot with the primary group `Current Assets` renamed in the book.
    let mut renamed = groups();
    for group in renamed
        .iter_mut()
        .filter(|group| group.name == "Current Assets")
    {
        group.name = "Working Assets".to_string();
    }
    for group in renamed
        .iter_mut()
        .filter(|group| group.parent.returned_text() == Some("Current Assets"))
    {
        group.parent = PartyLedgerMasterFieldObservation::Returned("Working Assets".to_string());
    }
    let placements = placements_of(renamed);
    let buyer = placed(&placements, "Shape Buyer 1");
    // The bucket is keyed by the reserved name and shows the name as the book has it now.
    assert_eq!(
        buyer.primary(),
        primary(
            "reserved:Current Assets",
            "Working Assets",
            Some("Current Assets")
        )
    );
}

#[test]
fn a_chain_that_cannot_be_walked_is_a_typed_gap_not_a_guess() {
    let parent_of = |ledger: &str| {
        catalogue()
            .parents()
            .find(|(name, _)| *name == ledger)
            .and_then(|(_, parent)| parent.map(str::to_string))
    };
    assert_eq!(
        parent_of("Shape Buyer 1").as_deref(),
        Some("Trade Debtors - Local")
    );
    let gap = |groups: Vec<TallyNamedMaster>| {
        *placements_of(groups)
            .get("Shape Buyer 1")
            .unwrap()
            .as_ref()
            .unwrap_err()
    };
    // Derived: a group missing from the snapshot.
    let mut missing = groups();
    missing.retain(|group| group.name != "Sundry Debtors");
    assert_eq!(gap(missing), "group_absent");
    // Derived: a group name that occurs twice.
    let mut repeated = groups();
    let copy = repeated
        .iter()
        .find(|group| group.name == "Sundry Debtors")
        .unwrap()
        .clone();
    repeated.push(copy);
    assert_eq!(gap(repeated), "group_name_repeated");
    // Derived: a group whose reserved name was never captured.
    let mut unnamed = groups();
    for group in unnamed
        .iter_mut()
        .filter(|group| group.name == "Trade Debtors - Local")
    {
        group.reserved_name = None;
    }
    assert_eq!(gap(unnamed), "reserved_name_missing");
    // Derived: two groups that are each other's parent.
    let mut looped = groups();
    for group in looped.iter_mut() {
        if group.name == "Sundry Debtors" {
            group.parent =
                PartyLedgerMasterFieldObservation::Returned("Trade Debtors - Local".to_string());
        }
    }
    assert_eq!(gap(looped), "cycle");
    // A ledger the catalogue gave no parent for.
    let none = Placements::build([("Orphan", None::<&str>)], groups());
    assert_eq!(
        *none.get("Orphan").unwrap().as_ref().unwrap_err(),
        "no_parent"
    );
    let blank = Placements::build([("Blank", Some("  "))], groups());
    assert_eq!(
        *blank.get("Blank").unwrap().as_ref().unwrap_err(),
        "no_parent"
    );
}

#[test]
fn two_reads_of_the_same_masters_are_equal_and_a_moved_ledger_or_group_makes_them_differ() {
    let first = placements_of(groups());
    assert_eq!(first, placements_of(groups()));
    // A ledger moved to another group.
    let moved = Placements::build(
        catalogue().parents().map(|(ledger, parent)| {
            (
                ledger,
                if ledger == "Shape Buyer 1" {
                    Some("Sundry Creditors")
                } else {
                    parent
                },
            )
        }),
        groups(),
    );
    assert_ne!(first, moved);
    // A group moved under another one.
    let mut regrouped = groups();
    for group in regrouped
        .iter_mut()
        .filter(|group| group.name == "Trade Debtors - Local")
    {
        group.parent = PartyLedgerMasterFieldObservation::Returned("Sundry Creditors".to_string());
    }
    assert_ne!(first, placements_of(regrouped));
    // A ledger gone from the catalogue.
    let fewer = Placements::build(
        catalogue()
            .parents()
            .filter(|(ledger, _)| *ledger != "Cash"),
        groups(),
    );
    assert_ne!(first, fewer);
}

#[test]
fn two_user_groups_directly_under_the_root_are_two_primary_buckets_not_one() {
    // Derived: two user-made groups (an empty reserved name) placed directly under the root, each with a
    // ledger. Keyed by their empty reserved names they would merge; they are keyed by their names.
    let mut derived = groups();
    let root_parent = derived
        .iter()
        .find(|group| group.name == "Current Assets")
        .unwrap()
        .parent
        .clone();
    for name in ["Branch Alpha", "Branch Beta"] {
        derived.push(TallyNamedMaster {
            name: name.to_string(),
            parent: root_parent.clone(),
            reserved_name: Some(String::new()),
        });
    }
    let placements = Placements::build(
        [
            ("Ledger A", Some("Branch Alpha")),
            ("Ledger B", Some("Branch Beta")),
        ],
        derived,
    );
    let (a, b) = (
        placed(&placements, "Ledger A").primary(),
        placed(&placements, "Ledger B").primary(),
    );
    assert_eq!(a, primary("user:Branch Alpha", "Branch Alpha", Some("")));
    assert_eq!(b, primary("user:Branch Beta", "Branch Beta", Some("")));
    assert_ne!(a.key, b.key);
    // A whitespace-only reserved name is a user group too.
    let mut spaced = groups();
    for group in spaced
        .iter_mut()
        .filter(|group| group.name == "Current Assets")
    {
        group.reserved_name = Some("  ".to_string());
    }
    let placements = Placements::build([("Cash", Some("Cash-in-Hand"))], spaced);
    assert_eq!(
        placed(&placements, "Cash").primary().key,
        "user:Current Assets"
    );
}

#[test]
fn a_parent_is_matched_exactly_not_trimmed_into_a_real_group() {
    let gap = |parent: &str| {
        *Placements::build([("Ledger", Some(parent))], groups())
            .get("Ledger")
            .unwrap()
            .as_ref()
            .unwrap_err()
    };
    assert_eq!(gap("Sundry Debtors "), "group_absent");
    assert_eq!(gap(" Sundry Debtors"), "group_absent");
    assert_eq!(gap("sundry debtors"), "group_absent");
    assert!(
        Placements::build([("Ledger", Some("Sundry Debtors"))], groups())
            .get("Ledger")
            .unwrap()
            .is_ok()
    );
}

#[test]
fn a_chain_whose_top_group_has_no_parent_is_not_taken_for_one_under_the_root() {
    // Derived: the snapshot never gave `Current Assets` a parent. The shared walker reads an empty parent
    // as the root; the placement does not.
    let mut derived = groups();
    for group in derived
        .iter_mut()
        .filter(|group| group.name == "Current Assets")
    {
        group.parent = PartyLedgerMasterFieldObservation::NotObserved;
    }
    let placements = Placements::build([("Cash", Some("Cash-in-Hand"))], derived);
    assert_eq!(
        *placements.get("Cash").unwrap().as_ref().unwrap_err(),
        "top_group_not_under_root"
    );
    let mut blank = groups();
    for group in blank
        .iter_mut()
        .filter(|group| group.name == "Current Assets")
    {
        group.parent = PartyLedgerMasterFieldObservation::Returned(String::new());
    }
    let placements = Placements::build([("Cash", Some("Cash-in-Hand"))], blank);
    assert_eq!(
        *placements.get("Cash").unwrap().as_ref().unwrap_err(),
        "top_group_not_under_root"
    );
}

#[test]
fn each_walk_gap_has_its_own_code() {
    for (gap, code) in [
        (AncestryGap::NoParent, "no_parent"),
        (AncestryGap::ReachedRoot, "reached_root"),
        (AncestryGap::GroupAbsent, "group_absent"),
        (AncestryGap::GroupNameRepeated, "group_name_repeated"),
        (AncestryGap::ReservedNameMissing, "reserved_name_missing"),
        (AncestryGap::Cycle, "cycle"),
        (AncestryGap::Exhausted, "exhausted"),
    ] {
        assert_eq!(gap_code(gap), code);
    }
}

#[test]
fn every_code_a_refusal_can_carry_survives_the_trip_through_text() {
    // A refusal's cause travels as text and comes back through `static_gap_code`; a code missing
    // from that list would read as `unknown`.
    for gap in [
        AncestryGap::NoParent,
        AncestryGap::ReachedRoot,
        AncestryGap::GroupAbsent,
        AncestryGap::GroupNameRepeated,
        AncestryGap::ReservedNameMissing,
        AncestryGap::Cycle,
        AncestryGap::Exhausted,
    ] {
        assert_eq!(static_gap_code(gap_code(gap)), gap_code(gap));
    }
    for code in [
        "top_group_not_under_root",
        "ledger_not_in_catalogue",
        "no_placements",
    ] {
        assert_eq!(static_gap_code(code), code);
    }
    assert_eq!(static_gap_code("not_a_code"), "unknown");
}

#[test]
fn the_memory_estimate_of_held_placements_grows_with_what_they_hold() {
    let empty = Placements::build(std::iter::empty::<(&str, Option<&str>)>(), groups());
    let full = placements_of(groups());
    assert_eq!(empty.approx_bytes(), 0);
    assert!(
        full.approx_bytes() > full.len() * 64,
        "{}",
        full.approx_bytes()
    );
}
