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
//! The catalogue keeps the verified company it was read for. The proof holds
//! for the answer it was built from, and nothing here shows that answer is the
//! whole book. Before any caller acts on a name shown absent, the answer must
//! come from the one request builder, be admitted by size, and agree with the
//! company's stock item count (§12a.13). That caller is a later pull request.
//!
//! `SiblingGst` is the one item that holds a given name exactly, as its row name
//! or a name entry, with its six source fields present as read. `GoodsSibling`
//! is a `SiblingGst` whose type of supply is Goods, whose every dated entry is
//! Taxable and whose applicability is the marked Applicable. Neither is yet a
//! copy that may be sent. A copy sent without Tally's mark on three of its
//! values was stored without its GST applicability and rates, and no counter
//! showed it (one run, PARTIAL; §12a.13). One shape was measured, and these two
//! types do not examine the rates, the sources, the state names, the valuation
//! types, the ineligible-ITC flag, the number of dated entries or state rows, or
//! a parent at the root. The type that holds a copy to send must admit the
//! measured shape only.

// Nothing outside the tests calls this until the stock-item tools exist.
#![cfg_attr(not(test), allow(dead_code))]

use super::ledger_create_item::MasterName;
use super::VerifiedCompanyIdentity;
use bridge_tally_core::master_name_index::{
    IndexedMaster, MasterKind, MasterNameIndex, MasterNameIndexError, NameCollision,
};
use bridge_tally_protocol::gst_details::{GstDetailsEntry, GstDetailsObservation};
use bridge_tally_protocol::hsn_details::{HsnDetailsEntry, HsnDetailsObservation};
use bridge_tally_protocol::native_masters::NativeMastersError;
use bridge_tally_protocol::stock_item_candidate::{
    parse_candidate_stock_items, CandidateStockItem, ScalarObservation,
};
use bridge_tally_protocol::TALLY_SANITIZED_ROOT_MARKER;

const GOODS: &str = "Goods";
const TAXABLE: &str = "Taxable";
const APPLICABLE: &str = " Applicable";

/// The stock items of one answer to the candidate read, each bound to one
/// verified company, with their names indexed. It does not show that the
/// answer is the whole book (see the module doc).
#[derive(Debug)]
pub(crate) struct StockItemCatalogue {
    company: VerifiedCompanyIdentity,
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
        Ok(Self {
            company: company.clone(),
            items,
            index,
        })
    }

    /// The company the read was made for and checked against on every row.
    pub(crate) fn company(&self) -> &VerifiedCompanyIdentity {
        &self.company
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

/// The six fields a stock item is copied by, in the order they are checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SiblingField {
    Parent,
    Unit,
    Applicability,
    TypeOfSupply,
    GstDetails,
    HsnDetails,
}

/// Why no complete sibling answers to the name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SiblingRefusal {
    /// No item answers to the name.
    NotFound,
    /// More than one item holds the name exactly; this many.
    Ambiguous { items: usize },
    /// No item holds the name exactly, and this many items answer to it by a
    /// fold of the name index. Those folds exist to refuse a proposed name, and
    /// one keeps only letters and digits, so a fold never names a sibling.
    OnlyByFold { items: usize },
    /// The first field that is absent or empty, or whose list holds content the parser does not model.
    Incomplete(SiblingField),
    /// The index could not check the name.
    Unchecked(MasterNameIndexError),
}

/// The six source fields of the one item that holds the name exactly, each
/// present and not blank as read. Presence only: the rates, the sources, the
/// state names, the valuation types and the ineligible-ITC flag are not examined.
#[derive(Debug)]
pub(crate) struct SiblingGst<'a> {
    catalogue: &'a StockItemCatalogue,
    item: &'a CandidateStockItem,
    parent: &'a str,
    unit: &'a str,
    applicability: &'a str,
    supply: &'a str,
    gst: &'a [GstDetailsEntry],
    hsn: &'a [HsnDetailsEntry],
}

fn non_blank(text: &Option<String>) -> bool {
    text.as_deref().is_some_and(|text| !text.trim().is_empty())
}

impl<'a> SiblingGst<'a> {
    /// Finds the one item that holds the name exactly, as its row name or a name
    /// entry, then requires all six fields. A name that only folds to an item is refused.
    pub(crate) fn named(
        catalogue: &'a StockItemCatalogue,
        name: &MasterName,
    ) -> Result<Self, SiblingRefusal> {
        let exact: Vec<_> = catalogue
            .items
            .iter()
            .filter(|item| {
                item.name == name.as_str() || item.names.iter().any(|alias| alias == name.as_str())
            })
            .collect();
        let item = match exact.as_slice() {
            [item] => *item,
            [] => {
                let collisions = catalogue
                    .index
                    .collisions(name.as_str())
                    .map_err(SiblingRefusal::Unchecked)?;
                let mut guids = Vec::with_capacity(collisions.len());
                for collision in &collisions {
                    if !guids.contains(&collision.guid.as_str()) {
                        guids.push(collision.guid.as_str());
                    }
                }
                if guids.is_empty() {
                    return Err(SiblingRefusal::NotFound);
                }
                return Err(SiblingRefusal::OnlyByFold { items: guids.len() });
            }
            _ => return Err(SiblingRefusal::Ambiguous { items: exact.len() }),
        };
        let parent = match &item.parent {
            // The parser gives no parent for a blank one, so `Some` is never blank here.
            Some(parent) => parent.as_str(),
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::Parent)),
        };
        let unit = match &item.base_units {
            ScalarObservation::Text(unit) => unit.as_str(),
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::Unit)),
        };
        let applicability = match &item.gst_applicable {
            ScalarObservation::Text(applicability) => applicability.as_str(),
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::Applicability)),
        };
        let supply = match &item.gst_type_of_supply {
            ScalarObservation::Text(supply) => supply.as_str(),
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::TypeOfSupply)),
        };
        let gst = match &item.gst {
            GstDetailsObservation::Entries { entries } if !entries.is_empty() => entries.as_slice(),
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::GstDetails)),
        };
        let gst_complete = gst.iter().all(|entry| {
            non_blank(&entry.taxability)
                && non_blank(&entry.source)
                && !entry.other_content_skipped
                && !entry.states.is_empty()
                && entry.states.iter().all(|state| {
                    non_blank(&state.state_name)
                        && !state.slab_rates_present
                        && !state.rates.is_empty()
                        && state.rates.iter().all(|rate| {
                            non_blank(&rate.duty_head) && non_blank(&rate.valuation_type)
                        })
                })
        });
        if !gst_complete {
            return Err(SiblingRefusal::Incomplete(SiblingField::GstDetails));
        }
        let hsn = match &item.hsn {
            HsnDetailsObservation::Entries { entries } if !entries.is_empty() => entries.as_slice(),
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::HsnDetails)),
        };
        let hsn_complete = hsn.iter().all(|entry| {
            non_blank(&entry.hsn_code)
                && non_blank(&entry.description)
                && non_blank(&entry.source)
                && !entry.other_content_skipped
        });
        if !hsn_complete {
            return Err(SiblingRefusal::Incomplete(SiblingField::HsnDetails));
        }
        Ok(Self {
            catalogue,
            item,
            parent,
            unit,
            applicability,
            supply,
            gst,
            hsn,
        })
    }

    /// The catalogue this complete read belongs to.
    pub(crate) fn catalogue(&self) -> &'a StockItemCatalogue {
        self.catalogue
    }

    /// The item's GUID as read.
    pub(crate) fn guid(&self) -> &'a str {
        &self.item.guid
    }

    /// The item's row name as read.
    pub(crate) fn name(&self) -> &'a str {
        &self.item.name
    }

    /// The item's parent as read.
    pub(crate) fn parent(&self) -> &'a str {
        self.parent
    }

    /// The item's base unit as read.
    pub(crate) fn unit(&self) -> &'a str {
        self.unit
    }

    /// The item's GST applicability as read.
    pub(crate) fn applicability(&self) -> &'a str {
        self.applicability
    }

    /// The item's type of supply as read.
    pub(crate) fn supply(&self) -> &'a str {
        self.supply
    }

    /// Every dated GST entry as read.
    pub(crate) fn gst(&self) -> &'a [GstDetailsEntry] {
        self.gst
    }

    /// Every dated HSN entry as read.
    pub(crate) fn hsn(&self) -> &'a [HsnDetailsEntry] {
        self.hsn
    }
}

/// Why a complete sibling is not the measured Goods case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NotGoodsSibling {
    /// The type of supply is not Goods.
    NotGoods,
    /// The GST entry at this position, in date order, is not Taxable.
    NotTaxable { entry: usize },
    /// GST applicability is not the marked `Applicable` of the measured copy (reference section 12a.13).
    GstNotApplicable,
}

/// A sibling whose three deciding values are those of the measured copy: Goods,
/// Taxable on every dated entry, and the marked Applicable. The rest of its
/// shape is not checked here; see the module doc.
#[derive(Debug)]
pub(crate) struct GoodsSibling<'a> {
    sibling: SiblingGst<'a>,
}

impl<'a> GoodsSibling<'a> {
    /// Admits only the exact Goods, Taxable and marked Applicable values, and
    /// checks nothing else.
    pub(crate) fn from_sibling(sibling: SiblingGst<'a>) -> Result<Self, NotGoodsSibling> {
        if sibling.supply() != GOODS {
            return Err(NotGoodsSibling::NotGoods);
        }
        for (entry, details) in sibling.gst().iter().enumerate() {
            if details.taxability.as_deref() != Some(TAXABLE) {
                return Err(NotGoodsSibling::NotTaxable { entry });
            }
        }
        if sibling
            .applicability()
            .strip_prefix(TALLY_SANITIZED_ROOT_MARKER)
            != Some(APPLICABLE)
        {
            return Err(NotGoodsSibling::GstNotApplicable);
        }
        Ok(Self { sibling })
    }

    /// The complete read admitted as a Goods sibling.
    pub(crate) fn sibling(&self) -> &SiblingGst<'a> {
        &self.sibling
    }
}

#[cfg(test)]
#[path = "stock_item_create_tests.rs"]
mod tests;
