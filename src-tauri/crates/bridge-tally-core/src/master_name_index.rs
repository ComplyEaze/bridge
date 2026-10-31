//! A company's ledger and group names, indexed so a proposed new ledger name
//! can be checked against every existing master before anything is written.
//!
//! Tally resolves a supplied master name loosely (`TALLY_PROTOCOL_REFERENCE_WRITE_RESPONSES_AND_MASTERS.md`
//! §9.4b, §9.4d): case, `-` and `/` as a space, whitespace runs. That
//! looseness is only partly measured, and a Create that resolves to an existing
//! ledger overwrites it (§9.4g). A false "absent" is therefore the dangerous
//! answer and a false "present" is a loud, safe one, so [`MasterNameIndex::collisions`]
//! reports a match under any of three keys, and a hit on any is a collision:
//! [`canonical_master_key`], [`absence_key`] (the third [`twin_fold_keys`]
//! position) and [`letters_and_digits_key`], which drops every space and
//! symbol: a ledger Create was measured to fold a removed space and removed
//! full stops, and every other symbol is refused conservatively, since a false
//! refusal costs less than an overwrite. The first two do not subsume each
//! other: lowercasing and NFC do not commute,
//! so `J` + caron and `j` + caron share a canonical key but not an absence key,
//! while NFC against NFD, or an en dash against a space, share an absence key
//! but not a canonical key. What is subsumed, and pinned by property tests in
//! `master_binding`, is the other two keys: equality on twin key 0 implies
//! equality on [`absence_key`] (it is a function of key 0), and equality on
//! twin key 1 implies equality on [`canonical_master_key`].
//!
//! A name matches a master through its stored name or any alias. A ledger name also collides with a
//! group's: Tally created a ledger named like an existing group (one synthetic
//! book, `bridge-tally-protocol/tests/fixtures/agent/masters-lab.PROVENANCE.md`),
//! but a person reading either name cannot tell the two apart, so the index
//! refuses it by choice.
//!
//! This module performs no I/O and holds no company identity. It decides
//! nothing about a binding: it only finds names that could be the same master.
//!
//! [`twin_fold_keys`]: crate::master_binding::twin_fold_keys

use std::collections::{BTreeMap, BTreeSet};

use crate::master_binding::{absence_key, canonical_master_key, letters_and_digits_key};

/// What kind of master an indexed name belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MasterKind {
    Ledger,
    Group,
}

/// How a colliding name belongs to its master.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Via {
    /// The master's stored name (`names[0]`).
    Name,
    /// One of the master's aliases (`names[1..]`).
    Alias,
}

/// One existing master as read from the book: `names[0]` is the stored name
/// and the rest are aliases. A ledger's export can also carry a row spelling
/// that is none of its names: a Create Tally resolved to an existing ledger
/// rewrites its stored name and leaves the row's `NAME` attribute as it was
/// until Tally restarts (one synthetic book,
/// `bridge-tally-protocol/tests/fixtures/agent/masters-lab.PROVENANCE.md`).
/// A caller appends it after the aliases, so the book is checked against it
/// too; a match on it alone is reported as [`Via::Alias`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedMaster {
    pub kind: MasterKind,
    pub guid: String,
    pub alter_id: u64,
    pub names: Vec<String>,
}

/// An existing master name the candidate could resolve to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameCollision {
    pub kind: MasterKind,
    pub via: Via,
    pub guid: String,
    /// The existing name that matched, as stored.
    pub name: String,
}

/// Why a set of masters cannot be indexed. `master` is the position in the
/// input list; names are not echoed, as they are client data.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MasterNameIndexError {
    #[error("master {master} has no names")]
    NoNames { master: usize },
    #[error("master {master} has an empty or whitespace-only name at position {position}")]
    BlankName { master: usize, position: usize },
    #[error("master {master} has an empty GUID")]
    EmptyGuid { master: usize },
    #[error("master {master} has a GUID with leading or trailing whitespace")]
    PaddedGuid { master: usize },
    #[error("the candidate name is empty or whitespace-only")]
    BlankCandidate,
    #[error("GUID {guid} appears on more than one master")]
    DuplicateGuid { guid: String },
}

/// Names keyed by one fold; each value lists `(master, name)` slots.
type KeyMap = BTreeMap<String, Vec<(usize, usize)>>;

#[derive(Debug, Clone)]
pub struct MasterNameIndex {
    masters: Vec<IndexedMaster>,
    /// One map per key of [`collision_keys`], in its order.
    keys: [KeyMap; 3],
}

/// The keys a name is matched on, in a fixed order.
fn collision_keys(name: &str) -> [String; 3] {
    [
        absence_key(name),
        canonical_master_key(name),
        letters_and_digits_key(name),
    ]
}

impl MasterNameIndex {
    /// Indexes `masters`, refusing a list the index could not answer for: a
    /// master without names, a blank name, a blank or whitespace-padded GUID
    /// (refused as given, never trimmed), or one GUID on two masters (compared
    /// ASCII case-insensitively, across both kinds).
    pub fn new(masters: Vec<IndexedMaster>) -> Result<Self, MasterNameIndexError> {
        let mut seen_guids = BTreeSet::new();
        for (master, entry) in masters.iter().enumerate() {
            if entry.guid.trim().is_empty() {
                return Err(MasterNameIndexError::EmptyGuid { master });
            }
            if entry.guid.trim() != entry.guid {
                return Err(MasterNameIndexError::PaddedGuid { master });
            }
            if entry.names.is_empty() {
                return Err(MasterNameIndexError::NoNames { master });
            }
            if let Some(position) = entry.names.iter().position(|name| name.trim().is_empty()) {
                return Err(MasterNameIndexError::BlankName { master, position });
            }
            if !seen_guids.insert(entry.guid.to_ascii_lowercase()) {
                return Err(MasterNameIndexError::DuplicateGuid {
                    guid: entry.guid.to_ascii_lowercase(),
                });
            }
        }

        let mut keys: [KeyMap; 3] = Default::default();
        for (master, entry) in masters.iter().enumerate() {
            for (position, name) in entry.names.iter().enumerate() {
                for (map, key) in keys.iter_mut().zip(collision_keys(name)) {
                    map.entry(key).or_default().push((master, position));
                }
            }
        }
        Ok(Self { masters, keys })
    }

    /// Every existing name `candidate` could resolve to, in the masters' input
    /// order and then name order, each reported once. Empty means no key
    /// matches any name or alias. A blank candidate is an error, not an empty
    /// answer: it names nothing, so "no collision" would be meaningless.
    pub fn collisions(&self, candidate: &str) -> Result<Vec<NameCollision>, MasterNameIndexError> {
        if candidate.trim().is_empty() {
            return Err(MasterNameIndexError::BlankCandidate);
        }
        let mut slots = BTreeSet::new();
        for (map, key) in self.keys.iter().zip(collision_keys(candidate)) {
            if let Some(found) = map.get(&key) {
                slots.extend(found.iter().copied());
            }
        }
        Ok(slots
            .into_iter()
            .map(|(master, position)| {
                let entry = &self.masters[master];
                NameCollision {
                    kind: entry.kind,
                    via: if position == 0 { Via::Name } else { Via::Alias },
                    guid: entry.guid.clone(),
                    name: entry.names[position].clone(),
                }
            })
            .collect())
    }

    /// Each master's GUID (ASCII-lowercased) with the `ALTERID` it was indexed
    /// at, for a later check that a write changed nothing it did not mean to.
    #[must_use]
    pub fn pre_image(&self) -> BTreeMap<String, u64> {
        self.masters
            .iter()
            .map(|entry| (entry.guid.to_ascii_lowercase(), entry.alter_id))
            .collect()
    }
}

#[cfg(test)]
#[path = "master_name_index_tests.rs"]
mod tests;
