//! The first step of creating one stock item (#1381): the stock items of one
//! answer to the candidate read, and the proof that a proposed name is none of
//! them.
//!
//! A Create for a name an item already holds does not fail. For an exact name
//! Tally answered ALTERED and replaced that item's GST and HSN details (one
//! run, PARTIAL). A name that differs only by case, or by a doubled space,
//! resolved to the existing item the same way (one run each; what became of
//! its GST details was not recorded). Hyphen, slash, removed spaces, full
//! stops and aliases were not measured for items, so every fold of the ledger
//! index is refused, as the safe answer. See
//! `TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md` §12a.13.
//!
//! The tests read the captures in
//! `bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/`; see their
//! `PROVENANCE.md` (PARTIAL: one book, one release, one sitting).
//!
//! The company is bound by its GUID on every row. The proof holds for the
//! answer it was built from, and nothing here shows that answer is the whole
//! book. Before any caller acts on a name shown absent, the answer must come
//! from the one request builder, be admitted by size, and agree with the
//! company's stock item count (§12a.13). That caller is a later pull request.

// Nothing outside the tests calls this until the stock-item tools exist.
#![cfg_attr(not(test), allow(dead_code))]

use super::ledger_create_item::MasterName;
use super::VerifiedCompanyIdentity;
use bridge_tally_core::master_name_index::{
    IndexedMaster, MasterKind, MasterNameIndex, MasterNameIndexError, NameCollision,
};
use bridge_tally_protocol::native_masters::NativeMastersError;
use bridge_tally_protocol::stock_item_candidate::{
    parse_candidate_stock_items, CandidateStockItem,
};

/// The stock items of one answer to the candidate read, each bound to one
/// verified company, with their names indexed. It does not show that the
/// answer is the whole book (see the module doc).
#[derive(Debug)]
pub(crate) struct StockItemCatalogue {
    items: Vec<CandidateStockItem>,
    index: MasterNameIndex,
}

/// Why a candidate read gives no catalogue. Carries no name from the book.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StockItemCatalogueError {
    /// The parser refused the answer, a foreign company's included.
    Unreadable(NativeMastersError),
    /// The answer held no stock item, so it binds no company.
    NoItems,
    /// The item at this position carried no language name, so its aliases
    /// cannot be seen.
    NoNameEntry { item: usize },
    /// The name index refused the rows.
    Index(MasterNameIndexError),
}

/// Why a proposed name is not shown absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ItemNameNotAbsent {
    /// The first existing name or alias the proposed name could resolve to.
    Taken(NameCollision),
    /// The index could not check the name.
    Unchecked(MasterNameIndexError),
}

/// A name no item of the catalogue it borrows can be taken for. It says
/// nothing of an item the catalogue's answer did not hold.
#[derive(Debug)]
pub(crate) struct ItemNameAbsent<'a> {
    catalogue: &'a StockItemCatalogue,
    name: MasterName,
}

impl StockItemCatalogue {
    /// Parses the candidate read of `company` and indexes every item under
    /// its language names, then its row name when that is none of them.
    pub(crate) fn from_candidate_read(
        company: &VerifiedCompanyIdentity,
        response: &str,
    ) -> Result<Self, StockItemCatalogueError> {
        let items = parse_candidate_stock_items(response, company.company_guid())
            .map_err(StockItemCatalogueError::Unreadable)?;
        if items.is_empty() {
            return Err(StockItemCatalogueError::NoItems);
        }
        let mut masters = Vec::with_capacity(items.len());
        for (item, row) in items.iter().enumerate() {
            if row.names.is_empty() {
                return Err(StockItemCatalogueError::NoNameEntry { item });
            }
            let mut names = row.names.clone();
            if !names.contains(&row.name) {
                names.push(row.name.clone());
            }
            masters.push(IndexedMaster {
                kind: MasterKind::StockItem,
                guid: row.guid.clone(),
                alter_id: row.alter_id,
                names,
            });
        }
        let index = MasterNameIndex::new(masters).map_err(StockItemCatalogueError::Index)?;
        Ok(Self { items, index })
    }

    /// The items as read, in the answer's order.
    pub(crate) fn items(&self) -> &[CandidateStockItem] {
        &self.items
    }

    /// Shows `name` absent, or names the first item it could be taken for.
    pub(crate) fn name_absent(
        &self,
        name: &MasterName,
    ) -> Result<ItemNameAbsent<'_>, ItemNameNotAbsent> {
        let collisions = self
            .index
            .collisions(name.as_str())
            .map_err(ItemNameNotAbsent::Unchecked)?;
        if let Some(first) = collisions.into_iter().next() {
            return Err(ItemNameNotAbsent::Taken(first));
        }
        Ok(ItemNameAbsent {
            catalogue: self,
            name: name.clone(),
        })
    }
}

impl<'a> ItemNameAbsent<'a> {
    pub(crate) fn name(&self) -> &MasterName {
        &self.name
    }

    /// The catalogue the name was checked against.
    pub(crate) fn catalogue(&self) -> &'a StockItemCatalogue {
        self.catalogue
    }
}

#[cfg(test)]
#[path = "stock_item_create_tests.rs"]
mod tests;
