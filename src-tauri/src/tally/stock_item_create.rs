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
//! or a name entry, with its six source fields present as read. A field of only
//! white space counts as missing. `GoodsSibling`
//! is a `SiblingGst` whose type of supply is Goods, whose every dated entry is
//! Taxable and whose applicability is the marked Applicable. Neither is yet a
//! copy that may be sent. A copy sent without Tally's mark on three of its
//! values was stored without its GST applicability and rates, and no counter
//! showed it (one run, PARTIAL; §12a.13). One shape was measured, and these two
//! types do not examine the rates, the sources, the state names, the valuation
//! types, the ineligible-ITC flag, the number of dated entries or state rows, or
//! a parent at the root. `StockItemDraft` is the type that holds a copy to
//! send: it admits the measured shape only, and a later builder renders it
//! (§12a.13).

// Nothing outside the tests calls this until the stock-item tools exist.
#![cfg_attr(not(test), allow(dead_code))]

use super::master_name::MasterName;
use super::VerifiedCompanyIdentity;
use bridge_tally_core::master_name_index::{
    IndexedMaster, MasterKind, MasterNameIndex, MasterNameIndexError, NameCollision,
};
use bridge_tally_protocol::gst_details::{GstDetailsEntry, GstDetailsObservation, GstRate};
use bridge_tally_protocol::hsn_details::{HsnDetailsEntry, HsnDetailsObservation};
use bridge_tally_protocol::native_masters::{
    parse_native_masters, NativeMasterKind, NativeMastersError,
};
use bridge_tally_protocol::stock_item_candidate::{
    parse_candidate_stock_items, CandidateStockItem, ScalarObservation,
};
use bridge_tally_protocol::TALLY_SANITIZED_ROOT_MARKER;

const GOODS: &str = "Goods";
const TAXABLE: &str = "Taxable";
const SPECIFY_DETAILS_HERE: &str = "Specify Details Here";
const BASED_ON_VALUE: &str = "Based on Value";

/// A control-reference value that the measured copy kept marked (§12a.13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MarkedConstant {
    /// GST applicability in the measured marked form.
    Applicable,
    /// The one state row in the measured marked form.
    Any,
    /// The Cess valuation in the measured marked form.
    NotApplicable,
}

impl MarkedConstant {
    /// Recognises only the exact marked text returned by the read.
    pub(crate) fn from_read(text: &str) -> Option<Self> {
        match text.strip_prefix(TALLY_SANITIZED_ROOT_MARKER) {
            Some(" Applicable") => Some(Self::Applicable),
            Some(" Any") => Some(Self::Any),
            Some(" Not Applicable") => Some(Self::NotApplicable),
            _ => None,
        }
    }
}

/// The two valuation values the measured copy sent (§12a.13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RateValuation {
    /// The valuation sent on CGST, SGST/UTGST, IGST and State Cess.
    BasedOnValue,
    /// The marked valuation sent on Cess.
    NotApplicable,
}

/// The five duty rows of the measured copy, in order: duty head, valuation and
/// whether a rate value is required (§12a.13).
const MEASURED_RATE_ROWS: [(&str, RateValuation, bool); 5] = [
    ("CGST", RateValuation::BasedOnValue, true),
    ("SGST/UTGST", RateValuation::BasedOnValue, true),
    ("IGST", RateValuation::BasedOnValue, true),
    ("Cess", RateValuation::NotApplicable, false),
    ("State Cess", RateValuation::BasedOnValue, false),
];

/// One duty row admitted for the measured copy (§12a.13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DraftRate<'a> {
    duty_head: &'a str,
    valuation: RateValuation,
    rate: Option<&'a str>,
}

impl<'a> DraftRate<'a> {
    /// The unmarked duty head at its measured position.
    pub(crate) fn duty_head(&self) -> &'a str {
        self.duty_head
    }

    /// The valuation measured for this duty head.
    pub(crate) fn valuation(&self) -> RateValuation {
        self.valuation
    }

    /// The value is the read's decimal without the read's leading space, as the
    /// measured request sent it; the two Cess rows have none (§12a.13).
    pub(crate) fn rate(&self) -> Option<&'a str> {
        self.rate
    }
}

/// The one dated GST entry admitted for the measured copy (§12a.13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DraftGstEntry<'a> {
    applicable_from: &'a str,
    rates: Vec<DraftRate<'a>>,
}

impl<'a> DraftGstEntry<'a> {
    /// The entry's applicability date as read.
    pub(crate) fn applicable_from(&self) -> &'a str {
        self.applicable_from
    }

    /// The five measured duty rows in their measured order.
    pub(crate) fn rates(&self) -> &[DraftRate<'a>] {
        &self.rates
    }
}

/// The one dated HSN entry admitted for the measured copy (§12a.13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DraftHsnEntry<'a> {
    applicable_from: &'a str,
    code: &'a str,
    description: &'a str,
}

impl<'a> DraftHsnEntry<'a> {
    /// The entry's applicability date as read.
    pub(crate) fn applicable_from(&self) -> &'a str {
        self.applicable_from
    }

    /// The unmarked HSN code as read.
    pub(crate) fn code(&self) -> &'a str {
        self.code
    }

    /// The unmarked HSN description as read.
    pub(crate) fn description(&self) -> &'a str {
        self.description
    }
}

/// Why a Goods sibling cannot be drafted as the measured copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DraftRefusal {
    /// The proposed name and sibling came from different catalogue reads.
    OtherCatalogue,
    /// The stock-group read could not be parsed for the sibling's company.
    GroupsUnreadable(NativeMastersError),
    /// No stock group in the same company's read has the parent's exact name.
    ParentNotInBook,
    /// The unit read could not be parsed for the sibling's company.
    UnitsUnreadable(NativeMastersError),
    /// No unit in the same company's read has the base unit's exact name.
    UnitNotInBook,
    /// A copy with this many dated GST entries was not measured (§12a.13).
    GstEntriesNotMeasured { entries: usize },
    /// This GST source was not sent in the measured copy (§12a.13).
    GstSourceNotMeasured,
    /// An ineligible-ITC flag was not sent in the measured copy (§12a.13).
    ItcNotMeasured,
    /// The measured copy had one state row marked `Any` (§12a.13).
    StateNotAny,
    /// The duty head at this row is absent, extra or not the measured head (§12a.13).
    DutyHeadNotMeasured { row: usize },
    /// The valuation at this row is not the one measured for its head (§12a.13).
    ValuationNotMeasured { row: usize },
    /// The rate at this row is not present or absent as measured (§12a.13).
    RateNotMeasured { row: usize },
    /// A copy with this many dated HSN entries was not measured (§12a.13).
    HsnEntriesNotMeasured { entries: usize },
    /// The HSN source, code or description is not in the measured form (§12a.13).
    HsnNotMeasured,
}

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
/// present and not blank as read. A field of only white space counts as missing.
/// Presence only: the rates, the sources, the
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
    text.as_deref().is_some_and(non_blank_text)
}

fn non_blank_text(text: &str) -> bool {
    !text.trim().is_empty()
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
            // The parser gives no parent for one that is empty after `str::trim`
            // (`native_masters.rs`), so `Some` is never blank here.
            Some(parent) => parent.as_str(),
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::Parent)),
        };
        let unit = match &item.base_units {
            ScalarObservation::Text(unit) if non_blank_text(unit) => unit.as_str(),
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::Unit)),
        };
        let applicability = match &item.gst_applicable {
            ScalarObservation::Text(applicability) if non_blank_text(applicability) => {
                applicability.as_str()
            }
            _ => return Err(SiblingRefusal::Incomplete(SiblingField::Applicability)),
        };
        let supply = match &item.gst_type_of_supply {
            ScalarObservation::Text(supply) if non_blank_text(supply) => supply.as_str(),
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
        if MarkedConstant::from_read(sibling.applicability()) != Some(MarkedConstant::Applicable) {
            return Err(NotGoodsSibling::GstNotApplicable);
        }
        Ok(Self { sibling })
    }

    /// The complete read admitted as a Goods sibling.
    pub(crate) fn sibling(&self) -> &SiblingGst<'a> {
        &self.sibling
    }
}

/// A copy in the measured shape only. It refuses unmeasured GST and HSN entry
/// counts, sources, ITC flags, state rows, duty heads, valuations, rates and
/// marked HSN values (§12a.13).
#[derive(Debug)]
pub(crate) struct StockItemDraft<'a> {
    sibling: GoodsSibling<'a>,
    name: MasterName,
    parent: &'a str,
    unit: &'a str,
    gst: DraftGstEntry<'a>,
    hsn: DraftHsnEntry<'a>,
}

impl<'a> StockItemDraft<'a> {
    /// Checks the proposed name and sibling against the same company's group
    /// and unit reads, then admits only the measured copy shape (§12a.13).
    pub(crate) fn new(
        name: ItemNameAbsent<'a>,
        sibling: GoodsSibling<'a>,
        groups: &str,
        units: &str,
    ) -> Result<Self, DraftRefusal> {
        if !std::ptr::eq(name.catalogue(), sibling.sibling().catalogue()) {
            return Err(DraftRefusal::OtherCatalogue);
        }
        let company_guid = name.catalogue().company().company_guid();
        let groups = parse_native_masters(NativeMasterKind::StockGroups, groups, company_guid)
            .map_err(DraftRefusal::GroupsUnreadable)?;
        let parent = sibling.sibling().parent();
        if !groups.rows.iter().any(|row| row.name == parent) {
            return Err(DraftRefusal::ParentNotInBook);
        }
        let units = parse_native_masters(NativeMasterKind::Units, units, company_guid)
            .map_err(DraftRefusal::UnitsUnreadable)?;
        let unit = sibling.sibling().unit();
        if !units.rows.iter().any(|row| row.name == unit) {
            return Err(DraftRefusal::UnitNotInBook);
        }

        let gst = match sibling.sibling().gst() {
            [details] => details,
            entries => {
                return Err(DraftRefusal::GstEntriesNotMeasured {
                    entries: entries.len(),
                });
            }
        };
        let hsn = match sibling.sibling().hsn() {
            [details] => details,
            entries => {
                return Err(DraftRefusal::HsnEntriesNotMeasured {
                    entries: entries.len(),
                });
            }
        };

        if gst.source.as_deref() != Some(SPECIFY_DETAILS_HERE) {
            return Err(DraftRefusal::GstSourceNotMeasured);
        }
        if gst.ineligible_itc.is_some() {
            return Err(DraftRefusal::ItcNotMeasured);
        }
        let state = match gst.states.as_slice() {
            [state]
                if state
                    .state_name
                    .as_deref()
                    .and_then(MarkedConstant::from_read)
                    == Some(MarkedConstant::Any) =>
            {
                state
            }
            _ => return Err(DraftRefusal::StateNotAny),
        };
        let mut rates = Vec::with_capacity(MEASURED_RATE_ROWS.len());
        for (row, (duty_head, expected_valuation, rate_required)) in
            MEASURED_RATE_ROWS.iter().enumerate()
        {
            let read = match state.rates.get(row) {
                Some(read) => read,
                None => return Err(DraftRefusal::DutyHeadNotMeasured { row }),
            };
            if read.duty_head.as_deref() != Some(*duty_head) {
                return Err(DraftRefusal::DutyHeadNotMeasured { row });
            }
            let valuation = match read.valuation_type.as_deref() {
                Some(BASED_ON_VALUE) => RateValuation::BasedOnValue,
                Some(text)
                    if MarkedConstant::from_read(text) == Some(MarkedConstant::NotApplicable) =>
                {
                    RateValuation::NotApplicable
                }
                _ => return Err(DraftRefusal::ValuationNotMeasured { row }),
            };
            if valuation != *expected_valuation {
                return Err(DraftRefusal::ValuationNotMeasured { row });
            }
            let rate = match (&read.rate, rate_required) {
                (GstRate::Value { decimal, .. }, true) => Some(decimal.as_str()),
                (GstRate::Absent, false) => None,
                _ => return Err(DraftRefusal::RateNotMeasured { row }),
            };
            rates.push(DraftRate {
                duty_head,
                valuation,
                rate,
            });
        }
        if state.rates.get(MEASURED_RATE_ROWS.len()).is_some() {
            return Err(DraftRefusal::DutyHeadNotMeasured {
                row: MEASURED_RATE_ROWS.len(),
            });
        }
        let gst = DraftGstEntry {
            applicable_from: &gst.applicable_from,
            rates,
        };

        let (code, description) = match (&hsn.hsn_code, &hsn.description) {
            (Some(code), Some(description))
                if hsn.source.as_deref() == Some(SPECIFY_DETAILS_HERE)
                    && !code.contains(TALLY_SANITIZED_ROOT_MARKER)
                    && !description.contains(TALLY_SANITIZED_ROOT_MARKER) =>
            {
                (code.as_str(), description.as_str())
            }
            _ => return Err(DraftRefusal::HsnNotMeasured),
        };
        let hsn = DraftHsnEntry {
            applicable_from: &hsn.applicable_from,
            code,
            description,
        };

        Ok(Self {
            sibling,
            name: name.name().clone(),
            parent,
            unit,
            gst,
            hsn,
        })
    }

    /// The proposed name proved absent from this catalogue.
    pub(crate) fn name(&self) -> &MasterName {
        &self.name
    }

    /// The parent exact name proved present in the same company's group read.
    pub(crate) fn parent(&self) -> &'a str {
        self.parent
    }

    /// The base-unit exact name proved present in the same company's unit read.
    pub(crate) fn unit(&self) -> &'a str {
        self.unit
    }

    /// The one dated GST entry in the measured shape.
    pub(crate) fn gst(&self) -> &DraftGstEntry<'a> {
        &self.gst
    }

    /// The one dated HSN entry in the measured shape.
    pub(crate) fn hsn(&self) -> &DraftHsnEntry<'a> {
        &self.hsn
    }

    /// The complete Goods sibling from which this draft was built.
    pub(crate) fn sibling(&self) -> &GoodsSibling<'a> {
        &self.sibling
    }
}

#[cfg(test)]
#[path = "stock_item_create_tests.rs"]
mod tests;
