//! Where each ledger of a window sits in the group tree, for `summarise_by` `group` and
//! `primary_group` (#1230).
//!
//! Built from two reads the server already makes elsewhere, with no request of its own: the ledger
//! catalogue (each ledger's `PARENT`) and the group snapshot (each group's parent and
//! `RESERVEDNAME`). A ledger whose chain cannot be walked to the reserved root is kept as a typed
//! gap, never guessed, and a summary that touches one is refused (see `voucher_summary`).
//!
//! The placement is today's masters, not the grouping in force on each voucher's date.
use bridge_tally_protocol::group_ancestry::{AncestryGap, GroupIndex};
use bridge_tally_protocol::{
    is_tally_reserved_root, TallyNamedMaster, TALLY_SANITIZED_ROOT_MARKER,
};
use std::collections::BTreeMap;

/// One group on a ledger's chain: its own (renamable) name and its `RESERVEDNAME`, which is
/// empty for a group the book's user created.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct GroupHop {
    pub(super) name: String,
    pub(super) reserved_name: String,
}

/// The group directly under the root that a ledger sits under: what identifies its bucket and what it
/// is called now. A predefined group is identified by its reserved name, which a rename of the group
/// does not change; a group the book's user made has none, so it is identified by its name under a
/// prefix no reserved name can begin with, and two such groups never share a bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Primary {
    pub(super) key: String,
    pub(super) name: String,
    /// `None` for the reserved root itself, which has no reserved name; empty for a user group.
    pub(super) reserved_name: Option<String>,
}

/// A ledger's place in the tree: every group from its immediate parent up to, not including,
/// the reserved root, nearest first. A ledger directly under the root has an empty chain and the
/// root's own label.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Placement {
    pub(super) chain: Vec<GroupHop>,
    /// The root's label (`Primary`), set only for a ledger directly under the root.
    pub(super) root_label: Option<String>,
}

impl Placement {
    /// The immediate group's name, or the root's label for a ledger directly under it.
    pub(super) fn group_name(&self) -> &str {
        match (self.chain.first(), &self.root_label) {
            (Some(hop), _) => &hop.name,
            (None, Some(label)) => label,
            (None, None) => "",
        }
    }

    /// The immediate group's reserved name: empty for a user group, `None` for the root.
    pub(super) fn group_reserved_name(&self) -> Option<&str> {
        self.chain.first().map(|hop| hop.reserved_name.as_str())
    }

    pub(super) fn primary(&self) -> Primary {
        match (self.chain.last(), &self.root_label) {
            (Some(hop), _) => Primary {
                key: if hop.reserved_name.trim().is_empty() {
                    format!("user:{}", hop.name)
                } else {
                    format!("reserved:{}", hop.reserved_name)
                },
                name: hop.name.clone(),
                reserved_name: Some(hop.reserved_name.clone()),
            },
            (None, label) => Primary {
                key: "root".to_string(),
                name: label.clone().unwrap_or_default(),
                reserved_name: None,
            },
        }
    }
}

/// Why a ledger has no placement: the typed reason the walk stopped, as a code.
pub(super) fn gap_code(gap: AncestryGap) -> &'static str {
    match gap {
        AncestryGap::NoParent => "no_parent",
        AncestryGap::ReachedRoot => "reached_root",
        AncestryGap::GroupAbsent => "group_absent",
        AncestryGap::GroupNameRepeated => "group_name_repeated",
        AncestryGap::ReservedNameMissing => "reserved_name_missing",
        AncestryGap::Cycle => "cycle",
        AncestryGap::Exhausted => "exhausted",
    }
}

/// Every ledger of the catalogue with its placement or the code of the gap that stopped the walk.
/// Two reads that produced equal values saw the same ledgers under the same chains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Placements {
    ledgers: BTreeMap<String, Result<Placement, &'static str>>,
}

impl Placements {
    pub(super) fn build<'a>(
        parents: impl IntoIterator<Item = (&'a str, Option<&'a str>)>,
        groups: Vec<TallyNamedMaster>,
    ) -> Self {
        // A walk that ends because a group has no parent is not a walk that reached the root: the
        // shared walker treats an empty parent as the root, so the top group's own parent is checked
        // here against the root marker.
        let top_parents: BTreeMap<String, Option<String>> = groups
            .iter()
            .map(|group| {
                (
                    group.name.clone(),
                    group.parent.returned_text().map(str::to_string),
                )
            })
            .collect();
        let index = GroupIndex::build(groups);
        let ledgers = parents
            .into_iter()
            .map(|(ledger, parent)| (ledger.to_string(), place(&index, &top_parents, parent)))
            .collect();
        Self { ledgers }
    }

    pub(super) fn get(&self, ledger: &str) -> Option<&Result<Placement, &'static str>> {
        self.ledgers.get(ledger)
    }

    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.ledgers.len()
    }

    /// An estimate of the memory the placements hold, in bytes, from the text they carry.
    pub(super) fn approx_bytes(&self) -> usize {
        self.ledgers
            .iter()
            .map(|(ledger, placement)| {
                ledger.len()
                    + 64
                    + placement.as_ref().map_or(0, |placement| {
                        placement
                            .chain
                            .iter()
                            .map(|hop| hop.name.len() + hop.reserved_name.len() + 48)
                            .sum::<usize>()
                    })
            })
            .sum()
    }

    /// Whether two reads listed exactly the same ledgers, whatever their placements.
    pub(super) fn same_ledgers(&self, other: &Self) -> bool {
        self.ledgers.keys().eq(other.ledgers.keys())
    }
}

/// The static form of a code carried through a summary refusal as text, for its typed cause.
pub(super) fn static_gap_code(code: &str) -> &'static str {
    [
        "no_parent",
        "reached_root",
        "group_absent",
        "group_name_repeated",
        "reserved_name_missing",
        "cycle",
        "exhausted",
        "top_group_not_under_root",
        "ledger_not_in_catalogue",
        "no_placements",
    ]
    .into_iter()
    .find(|known| *known == code)
    .unwrap_or("unknown")
}

fn place(
    index: &GroupIndex,
    parents: &BTreeMap<String, Option<String>>,
    parent: Option<&str>,
) -> Result<Placement, &'static str> {
    // Blankness is judged on the trimmed text; the walk matches the text exactly, as the shared walker
    // requires (a trimmed hop could resolve an incoherent pair against a real group).
    let Some(parent) = parent.filter(|text| !text.trim().is_empty()) else {
        return Err(gap_code(AncestryGap::NoParent));
    };
    if is_tally_reserved_root(parent) {
        let label = parent
            .trim()
            .strip_prefix(TALLY_SANITIZED_ROOT_MARKER)
            .unwrap_or(parent)
            .trim();
        return Ok(Placement {
            chain: Vec::new(),
            root_label: Some(label.to_string()),
        });
    }
    let chain = index.ancestry_chain(Some(parent));
    if let Some(gap) = chain.gap {
        return Err(gap_code(gap));
    }
    let Some(top) = chain.hops.last() else {
        return Err("top_group_not_under_root");
    };
    let under_root = parents
        .get(&top.name)
        .and_then(|parent| parent.as_deref())
        .is_some_and(is_tally_reserved_root);
    if !under_root {
        return Err("top_group_not_under_root");
    }
    Ok(Placement {
        chain: chain
            .hops
            .into_iter()
            .map(|hop| GroupHop {
                name: hop.name,
                reserved_name: hop.reserved_name,
            })
            .collect(),
        root_label: None,
    })
}

#[cfg(test)]
#[path = "agent_voucher_groups_tests.rs"]
pub(super) mod tests;
