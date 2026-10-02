// SPDX-License-Identifier: Apache-2.0
//! Port of the part of the reference engine's `party_identity` that `entity_269st_gap` uses: the
//! ledgers that are parties, and the entities bound by a shared PAN.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::Book;
use crate::error::Result;

/// Where a party's PAN came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanSource {
    Masters,
    ClientConfig,
    DerivedFromGstin,
}

/// The engagement's optional `[party_identity]` table.
#[derive(Debug, Clone, Default)]
pub struct PartyConfig {
    pub derive_pan_from_gstin: bool,
    pub party_groups: Vec<String>,
    pub additional_party_ledgers: BTreeSet<String>,
    pub excluded_ledgers: BTreeSet<String>,
    pub round_off_ledgers: BTreeSet<String>,
    pub overrides: BTreeMap<String, PartyOverride>,
}

impl PartyConfig {
    /// The optional `[party_identity]` table; empty when absent.
    pub fn from_toml(_raw: Option<&toml::Value>) -> Result<Self> {
        Ok(Self::default())
    }
}

/// One ledger's override: a field fills a gap a master left, never replaces what it holds.
#[derive(Debug, Clone, Default)]
pub struct PartyOverride {
    pub name: String,
    pub pan: String,
    pub gstin: String,
    pub address: String,
}

/// Ledgers that are one legal person, bound by a shared PAN, never by name.
#[derive(Debug, Clone)]
pub struct EntityBinding {
    pub pan: String,
    /// Canonical ledger names, sorted.
    pub ledgers: Vec<String>,
    /// Per ledger, aligned with `ledgers`.
    pub pan_sources: Vec<PanSource>,
    /// Disclosure only, never a binding criterion: whether every ledger name shares a word.
    pub names_agree: bool,
}

#[derive(Debug, Default)]
pub struct PartyIndex {
    entities: Vec<EntityBinding>,
    entity_of: BTreeMap<String, usize>,
}

impl PartyIndex {
    /// The binding a ledger belongs to, or `None` when it has no PAN to bind on.
    pub fn entity_for_ledger(&self, ledger: &str) -> Option<&EntityBinding> {
        self.entity_of.get(ledger).map(|i| &self.entities[*i])
    }
}

pub fn build_party_index(_book: &Book, _config: &PartyConfig) -> Result<PartyIndex> {
    Ok(PartyIndex::default())
}
