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
//! The partial catalogue keeps the verified company it was read for. The
//! complete catalogue owns the 5b-3a proof that its answer came from an admitted
//! read whose rows agree with the company's stock item count (§12a.13). The
//! 5b-3b coordinator owns obtaining the paired reads through the queue; nothing
//! in this transport-independent part reads or sends.
//!
//! `SiblingGst` is the one item that holds a given name exactly, as its row name
//! or a name entry, with its six source fields present as read. A field of only
//! white space counts as missing. `GoodsSibling`
//! is a `SiblingGst` whose type of supply is Goods, whose every dated entry is
//! Taxable and whose applicability is the marked Applicable. Neither is yet a
//! copy that may be sent. A copy sent without Tally's mark on three of its
//! values was stored without its GST applicability and rates, and no counter
//! showed it (one run, PARTIAL; §12a.13). One shape was measured that way. Of
//! the Creates that later changed one variable each (§12a.13), `StockItemDraft`
//! refuses a second dated GST or HSN entry and a Cess rate; it copies the rates
//! and the unit as read, so a sibling with other rates or another unit drafts
//! and renders though no such copy was measured. `SiblingGst`
//! requires the parent, unit, applicability and type of supply to be present and
//! not blank; each dated GST entry needs its taxability and source, at least one
//! state row with a name, and at least one rate row with a duty head and
//! valuation type; each HSN entry needs its code, description and source.
//! Except where `GoodsSibling` checks Goods, Taxable and the marked Applicable,
//! their values are not examined: the unit's value, the sources' values, the
//! state names' values, the duty heads' values and number of rate rows, the
//! valuation types' values, the rates (which may be absent, zero or empty), the
//! HSN code's and description's values, the ineligible-ITC flag, the numbers of
//! dated entries and state rows, and a parent at the root are not examined.
//! `StockItemDraft` admits the structure of the first marked Create measured (one run,
//! PARTIAL, §12a.13), with values read from the sibling, and the
//! `PreparedStockItem` made from it renders it. The name is a `MasterName`; the parent
//! and unit equal a row name in the same company's group or unit read; the dates
//! were parsed by the protocol crate. Each rate is a plain non-zero decimal on
//! the three rows that carry one and absent on the two that do not; the HSN
//! code and description do not hold the root marker. The draft does not hold
//! text to plain text: `PreparedStockItem` does, refusing a parent, unit, HSN code or
//! HSN description that fails the `MasterName` rule or holds one of
//! `& < > " '`. The rate's magnitude and the unit's value are not checked.
//! Bridge does not know how Tally treats values other than the measured ones.

// Nothing outside the tests calls this until the stock-item tools exist.
#![cfg_attr(not(test), allow(dead_code))]

use super::master_name::{MasterName, MasterNameError};
use super::runtime::{AdmittedCandidateRead, AdmittedStockGroups, AdmittedStockUnits};
use super::VerifiedCompanyIdentity;
use bridge_tally_core::master_name_index::{
    IndexedMaster, MasterKind, MasterNameIndex, MasterNameIndexError, NameCollision,
};
use bridge_tally_core::TallyDate;
use bridge_tally_protocol::gst_details::{
    classify_gst_rate, GstDetailsEntry, GstDetailsObservation, GstRate,
};
use bridge_tally_protocol::hsn_details::{HsnDetailsEntry, HsnDetailsObservation};
use bridge_tally_protocol::import_envelope::{render_import_envelope, ImportReport};
#[cfg(test)]
use bridge_tally_protocol::native_masters::{parse_native_masters, NativeMasterKind};
use bridge_tally_protocol::native_masters::{NativeMasterRow, NativeMastersError};
use bridge_tally_protocol::stock_item_candidate::{
    parse_candidate_stock_items, CandidateStockItem, ScalarObservation,
};
use bridge_tally_protocol::{sha256_hex, TALLY_SANITIZED_ROOT_MARKER};

pub(crate) const GOODS: &str = "Goods";
pub(crate) const TAXABLE: &str = "Taxable";
pub(crate) const SPECIFY_DETAILS_HERE: &str = "Specify Details Here";
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

    /// The marked text sent in the measured Create (§12a.13).
    pub(crate) fn render(self) -> &'static str {
        match self {
            Self::Applicable => "&#4; Applicable",
            Self::Any => "&#4; Any",
            Self::NotApplicable => "&#4; Not Applicable",
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

impl RateValuation {
    /// Recognises only the two valuation texts the measured copy read.
    fn from_read(text: Option<&str>) -> Option<Self> {
        match text {
            Some(BASED_ON_VALUE) => Some(Self::BasedOnValue),
            Some(text)
                if MarkedConstant::from_read(text) == Some(MarkedConstant::NotApplicable) =>
            {
                Some(Self::NotApplicable)
            }
            _ => None,
        }
    }

    /// The valuation text sent in the measured Create (§12a.13).
    pub(crate) fn render(self) -> &'static str {
        match self {
            Self::BasedOnValue => BASED_ON_VALUE,
            Self::NotApplicable => MarkedConstant::NotApplicable.render(),
        }
    }
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
pub(crate) struct DraftRate {
    duty_head: &'static str,
    valuation: RateValuation,
    rate: Option<String>,
}

impl DraftRate {
    /// The unmarked duty head at its measured position.
    pub(crate) fn duty_head(&self) -> &'static str {
        self.duty_head
    }

    /// The valuation measured for this duty head.
    pub(crate) fn valuation(&self) -> RateValuation {
        self.valuation
    }

    /// The value is the read's decimal without the read's leading space, as the
    /// measured request sent it; the two Cess rows have none (§12a.13).
    pub(crate) fn rate(&self) -> Option<&str> {
        self.rate.as_deref()
    }
}

/// The one dated GST entry admitted for the measured copy (§12a.13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DraftGstEntry {
    applicable_from: String,
    rates: Vec<DraftRate>,
}

impl DraftGstEntry {
    /// The entry's applicability date as read.
    pub(crate) fn applicable_from(&self) -> &str {
        &self.applicable_from
    }

    /// The five measured duty rows in their measured order.
    pub(crate) fn rates(&self) -> &[DraftRate] {
        &self.rates
    }
}

/// The one dated HSN entry admitted for the measured copy (§12a.13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DraftHsnEntry {
    applicable_from: String,
    code: String,
    description: String,
}

impl DraftHsnEntry {
    /// The entry's applicability date as read.
    pub(crate) fn applicable_from(&self) -> &str {
        &self.applicable_from
    }

    /// The unmarked HSN code as read.
    pub(crate) fn code(&self) -> &str {
        &self.code
    }

    /// The unmarked HSN description as read.
    pub(crate) fn description(&self) -> &str {
        &self.description
    }
}

/// Every value a stock item Create writes from a draft, owned: the proposed
/// name, the sibling it copies (its GUID and the exact row name or alias it was
/// selected by), and the parent, unit, GST entry and HSN entry copied from it.
/// The recheck refuses unless the fresh draft's writes equal the shown draft's.
/// Only a draft or `rebuild` makes one; a rebuilt value is for read-back only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StockItemWrites {
    name: MasterName,
    sibling_guid: String,
    sibling_source_name: MasterName,
    parent: String,
    unit: String,
    gst: DraftGstEntry,
    hsn: DraftHsnEntry,
}

/// One stored duty row: its head and its rate text, `None` where the measured
/// row has no rate. Its valuation is the one measured at its position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlainRate {
    pub(crate) duty_head: String,
    pub(crate) rate: Option<String>,
}

/// The values of a `StockItemWrites` as plain text (`to_plain`), before
/// `rebuild` admits them again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlainStockItemWrites {
    pub(crate) name: String,
    pub(crate) sibling_guid: String,
    pub(crate) sibling_source_name: String,
    pub(crate) parent: String,
    pub(crate) unit: String,
    pub(crate) gst_applicable_from: String,
    pub(crate) rates: Vec<PlainRate>,
    pub(crate) hsn_applicable_from: String,
    pub(crate) hsn_code: String,
    pub(crate) hsn_description: String,
}

/// Why stored plain values are not writes a draft could have made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WritesRebuildRefusal {
    /// The stored proposed name is not a `MasterName`.
    Name(MasterNameError),
    /// The stored source name is not a `MasterName`.
    SourceName(MasterNameError),
    /// The stored sibling GUID is empty or only white space.
    SiblingGuidBlank,
    /// The stored GST entry's date is not one the protocol crate admits.
    GstDateInvalid,
    /// The stored HSN entry's date is not one the protocol crate admits.
    HsnDateInvalid,
    /// A stored duty row, or the HSN code or description, is refused as
    /// `measured_shape` refuses a read one.
    Shape(DraftRefusal),
    /// The renderer's text rule refuses a stored value.
    Text(CreateRenderRefusal),
}

impl StockItemWrites {
    /// Admits stored plain values again, in order: the name and source name
    /// as `MasterName`s, a non-blank sibling GUID, the GST date by the protocol
    /// crate's date rule (`TallyDate`), each duty row by the measured row at
    /// its position with its rate classified by `classify_gst_rate` and kept
    /// only when the stored text is already that plain decimal, the HSN date,
    /// an HSN code and description without the root marker, and last
    /// `create_text_refusal`. A stored rate the classifier refuses is refused
    /// at its row before its head is checked. The parent and unit are not
    /// checked against any book, and nothing here reads one, so a rebuilt
    /// value is for read-back only and never reaches a Create.
    pub(crate) fn rebuild(plain: &PlainStockItemWrites) -> Result<Self, WritesRebuildRefusal> {
        use WritesRebuildRefusal as Refusal;
        let name = MasterName::new(&plain.name).map_err(Refusal::Name)?;
        let sibling_source_name =
            MasterName::new(&plain.sibling_source_name).map_err(Refusal::SourceName)?;
        if !non_blank_text(&plain.sibling_guid) {
            return Err(Refusal::SiblingGuidBlank);
        }
        if TallyDate::parse(plain.gst_applicable_from.as_str()).is_err() {
            return Err(Refusal::GstDateInvalid);
        }
        let mut rates = Vec::with_capacity(plain.rates.len());
        for (row, stored) in plain.rates.iter().enumerate() {
            let rate = match classify_gst_rate(stored.rate.clone()) {
                Ok(GstRate::Value { raw, decimal }) if raw != decimal => {
                    return Err(Refusal::Shape(DraftRefusal::RateNotMeasured { row }))
                }
                Ok(rate) => rate,
                Err(_) => return Err(Refusal::Shape(DraftRefusal::RateNotMeasured { row })),
            };
            let valuation = MEASURED_RATE_ROWS
                .get(row)
                .map(|(_, valuation, _)| *valuation);
            rates.push(
                measured_rate(row, Some(&stored.duty_head), valuation, &rate)
                    .map_err(Refusal::Shape)?,
            );
        }
        let rates = all_measured_rows(rates).map_err(Refusal::Shape)?;
        if TallyDate::parse(plain.hsn_applicable_from.as_str()).is_err() {
            return Err(Refusal::HsnDateInvalid);
        }
        if plain.hsn_code.contains(TALLY_SANITIZED_ROOT_MARKER)
            || plain.hsn_description.contains(TALLY_SANITIZED_ROOT_MARKER)
        {
            return Err(Refusal::Shape(DraftRefusal::HsnNotMeasured));
        }
        let writes = Self {
            name,
            sibling_guid: plain.sibling_guid.clone(),
            sibling_source_name,
            parent: plain.parent.clone(),
            unit: plain.unit.clone(),
            gst: DraftGstEntry {
                applicable_from: plain.gst_applicable_from.clone(),
                rates,
            },
            hsn: DraftHsnEntry {
                applicable_from: plain.hsn_applicable_from.clone(),
                code: plain.hsn_code.clone(),
                description: plain.hsn_description.clone(),
            },
        };
        create_text_refusal(&writes).map_err(Refusal::Text)?;
        Ok(writes)
    }

    /// The values as the plain text `rebuild` takes.
    pub(crate) fn to_plain(&self) -> PlainStockItemWrites {
        PlainStockItemWrites {
            name: self.name.as_str().to_owned(),
            sibling_guid: self.sibling_guid.clone(),
            sibling_source_name: self.sibling_source_name.as_str().to_owned(),
            parent: self.parent.clone(),
            unit: self.unit.clone(),
            gst_applicable_from: self.gst.applicable_from.clone(),
            rates: self
                .gst
                .rates
                .iter()
                .map(|rate| PlainRate {
                    duty_head: rate.duty_head.to_owned(),
                    rate: rate.rate.clone(),
                })
                .collect(),
            hsn_applicable_from: self.hsn.applicable_from.clone(),
            hsn_code: self.hsn.code.clone(),
            hsn_description: self.hsn.description.clone(),
        }
    }

    /// The proposed name proved absent from the catalogue it was drafted on.
    pub(crate) fn name(&self) -> &MasterName {
        &self.name
    }

    /// The GUID of the sibling the values were copied from.
    pub(crate) fn sibling_guid(&self) -> &str {
        &self.sibling_guid
    }

    /// The exact row name or alias the sibling was selected by.
    pub(crate) fn sibling_source_name(&self) -> &MasterName {
        &self.sibling_source_name
    }

    /// The parent's exact name.
    pub(crate) fn parent(&self) -> &str {
        &self.parent
    }

    /// The base unit's exact name.
    pub(crate) fn unit(&self) -> &str {
        &self.unit
    }

    /// The one dated GST entry in the measured shape.
    pub(crate) fn gst(&self) -> &DraftGstEntry {
        &self.gst
    }

    /// The one dated HSN entry in the measured shape.
    pub(crate) fn hsn(&self) -> &DraftHsnEntry {
        &self.hsn
    }

    /// Lowercase hex SHA-256 of the UTF-8 text the one renderer gives for
    /// these writes in `company`, after the same text rule; the digest only,
    /// never the request.
    pub(crate) fn create_sha256(
        &self,
        company: &VerifiedCompanyIdentity,
    ) -> Result<String, CreateRenderRefusal> {
        create_text_refusal(self)?;
        Ok(sha256_hex(
            render_writes(company.display_name(), self).as_bytes(),
        ))
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

/// A candidate catalogue whose admitted row count equals Tally's company count,
/// together with the master mark at which its read was sized.
#[derive(Debug)]
pub(crate) struct CompleteStockItemCatalogue {
    catalogue: StockItemCatalogue,
    sized_mark: u64,
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
    /// The item at this position has a GUID that is not lowercase hex digits
    /// and hyphens, the form the master journal records; it is not lowercased.
    GuidNotLowercase { item: usize },
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
            if !is_lowercase_hex_and_hyphens(&row.guid) {
                return Err(StockItemCatalogueError::GuidNotLowercase { item });
            }
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

    fn name_absent_proof(
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

    /// The partial check remains available to its existing unit tests only.
    #[cfg(test)]
    pub(crate) fn name_absent(
        &self,
        name: &MasterName,
    ) -> Result<ItemNameAbsent<'_>, ItemNameNotAbsent> {
        self.name_absent_proof(name)
    }
}

impl CompleteStockItemCatalogue {
    /// Re-parses an admitted candidate body into the catalogue it proves whole.
    /// The read is taken owned or borrowed: the queue lends its own.
    pub(crate) fn from_admitted(
        read: impl std::borrow::Borrow<AdmittedCandidateRead>,
    ) -> Result<Self, StockItemCatalogueError> {
        let read = read.borrow();
        let catalogue = StockItemCatalogue::from_candidate_read(read.company(), read.body())?;
        Ok(Self {
            catalogue,
            sized_mark: read.sized_mark(),
        })
    }

    /// Shows `name` absent from the complete candidate answer, or names the
    /// first item it could be taken for.
    pub(crate) fn name_absent(
        &self,
        name: &MasterName,
    ) -> Result<ItemNameAbsent<'_>, ItemNameNotAbsent> {
        self.catalogue.name_absent_proof(name)
    }

    /// The parsed candidate catalogue, read-only.
    pub(crate) fn catalogue(&self) -> &StockItemCatalogue {
        &self.catalogue
    }

    /// The master mark at which the candidate read was sized.
    pub(crate) const fn sized_mark(&self) -> u64 {
        self.sized_mark
    }

    #[cfg(test)]
    pub(crate) fn from_catalogue_for_test(catalogue: StockItemCatalogue, sized_mark: u64) -> Self {
        Self {
            catalogue,
            sized_mark,
        }
    }
}

impl<'a> ItemNameAbsent<'a> {
    /// The proposed name, shown absent from the catalogue.
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

/// Why no sibling with the six required fields answers to the name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SiblingRefusal {
    /// No item answers to the name.
    NotFound,
    /// More than one item holds the name exactly; this many.
    Ambiguous { items: usize },
    /// No item holds the name exactly, and this many items answer to it by a
    /// fold of the name index. Bridge's rule so far, a choice and not a Tally
    /// fact: an item is a source only when it holds the name exactly, as its row
    /// name or a name entry, and a name that only folds to an item is refused.
    OnlyByFold { items: usize },
    /// The first field that is absent or empty, or whose list holds content the parser does not model.
    /// `Incomplete(SiblingField::GstDetails)` also reports slab rates, a list
    /// the parser cannot read, and a missing state row or rate row, so it does
    /// not by itself mean that the GST details are missing.
    Incomplete(SiblingField),
    /// The index could not check the name.
    Unchecked(MasterNameIndexError),
}

/// The six source fields of the one item that holds the name exactly, with the
/// parent, unit, applicability and type of supply present and not blank; each
/// dated GST entry needs its taxability and source, at least one state row with
/// a name, and at least one rate row with a duty head and valuation type, and
/// each HSN entry needs its code, description and source. A field of only white
/// space counts as missing. Except where `GoodsSibling` checks Goods, Taxable
/// and the marked Applicable, their values are not examined: the unit's value,
/// the sources' values, the state names' values, the duty heads' values and
/// number of rate rows, the valuation types' values, the rates (which may be
/// absent, zero or empty), the HSN code's and description's values, the
/// ineligible-ITC flag, the numbers of dated entries and state rows, and a
/// parent at the root are not examined.
#[derive(Debug)]
pub(crate) struct SiblingGst<'a> {
    catalogue: &'a StockItemCatalogue,
    item: &'a CandidateStockItem,
    source_name: MasterName,
    parent: &'a str,
    unit: &'a str,
    applicability: &'a str,
    supply: &'a str,
    gst: &'a [GstDetailsEntry],
    hsn: &'a [HsnDetailsEntry],
}

/// Whether `guid` is non-empty and holds only lowercase hex digits and hyphens,
/// as the captured reads hold a GUID and the master journal's pre-image records
/// one, so a read-back compares like with like. Its characters only: a stock
/// GUID is the company GUID, `-` and a hex suffix, and no layout is checked.
fn is_lowercase_hex_and_hyphens(guid: &str) -> bool {
    !guid.is_empty()
        && guid
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f' | b'-'))
}

fn non_blank(text: &Option<String>) -> bool {
    text.as_deref().is_some_and(non_blank_text)
}

fn non_blank_text(text: &str) -> bool {
    !text.trim().is_empty()
}

// The six readers of the sibling rule, one per field. Each gives `None` for a
// field `SiblingGst::named` refuses; `named` and `missing_fields` both use them.

fn read_parent(item: &CandidateStockItem) -> Option<&str> {
    // The parser gives no parent for one that is empty after `str::trim`
    // (`native_masters.rs`), so `Some` is never blank here.
    item.parent.as_deref()
}

fn read_unit(item: &CandidateStockItem) -> Option<&str> {
    match &item.base_units {
        ScalarObservation::Text(unit) if non_blank_text(unit) => Some(unit.as_str()),
        _ => None,
    }
}

fn read_applicability(item: &CandidateStockItem) -> Option<&str> {
    match &item.gst_applicable {
        ScalarObservation::Text(applicability) if non_blank_text(applicability) => {
            Some(applicability.as_str())
        }
        _ => None,
    }
}

fn read_supply(item: &CandidateStockItem) -> Option<&str> {
    match &item.gst_type_of_supply {
        ScalarObservation::Text(supply) if non_blank_text(supply) => Some(supply.as_str()),
        _ => None,
    }
}

fn read_gst(item: &CandidateStockItem) -> Option<&[GstDetailsEntry]> {
    let gst = match &item.gst {
        GstDetailsObservation::Entries { entries } if !entries.is_empty() => entries.as_slice(),
        _ => return None,
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
                    && state
                        .rates
                        .iter()
                        .all(|rate| non_blank(&rate.duty_head) && non_blank(&rate.valuation_type))
            })
    });
    gst_complete.then_some(gst)
}

fn read_hsn(item: &CandidateStockItem) -> Option<&[HsnDetailsEntry]> {
    let hsn = match &item.hsn {
        HsnDetailsObservation::Entries { entries } if !entries.is_empty() => entries.as_slice(),
        _ => return None,
    };
    let hsn_complete = hsn.iter().all(|entry| {
        non_blank(&entry.hsn_code)
            && non_blank(&entry.description)
            && non_blank(&entry.source)
            && !entry.other_content_skipped
    });
    hsn_complete.then_some(hsn)
}

/// Every field of `item` that `SiblingGst::named` refuses, in the order it
/// checks them; `named` refuses with the first.
pub(super) fn missing_fields(item: &CandidateStockItem) -> Vec<SiblingField> {
    [
        (SiblingField::Parent, read_parent(item).is_none()),
        (SiblingField::Unit, read_unit(item).is_none()),
        (
            SiblingField::Applicability,
            read_applicability(item).is_none(),
        ),
        (SiblingField::TypeOfSupply, read_supply(item).is_none()),
        (SiblingField::GstDetails, read_gst(item).is_none()),
        (SiblingField::HsnDetails, read_hsn(item).is_none()),
    ]
    .into_iter()
    .filter_map(|(field, missing)| missing.then_some(field))
    .collect()
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
        use SiblingRefusal::Incomplete;
        let parent = read_parent(item).ok_or(Incomplete(SiblingField::Parent))?;
        let unit = read_unit(item).ok_or(Incomplete(SiblingField::Unit))?;
        let applicability =
            read_applicability(item).ok_or(Incomplete(SiblingField::Applicability))?;
        let supply = read_supply(item).ok_or(Incomplete(SiblingField::TypeOfSupply))?;
        let gst = read_gst(item).ok_or(Incomplete(SiblingField::GstDetails))?;
        let hsn = read_hsn(item).ok_or(Incomplete(SiblingField::HsnDetails))?;
        Ok(Self {
            catalogue,
            item,
            source_name: name.clone(),
            parent,
            unit,
            applicability,
            supply,
            gst,
            hsn,
        })
    }

    /// The catalogue the sibling was found in.
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

    /// The exact row name or alias by which this sibling was selected.
    pub(crate) fn source_name(&self) -> &MasterName {
        &self.source_name
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

/// Why a sibling whose six fields are present is not the measured Goods case.
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

    /// The sibling whose six fields are present, admitted as Goods.
    pub(crate) fn sibling(&self) -> &SiblingGst<'a> {
        &self.sibling
    }
}

/// A copy with the structure of the first marked Create measured (one run, PARTIAL,
/// §12a.13), with values read from the sibling. It refuses unmeasured GST and
/// HSN entry counts, sources, ITC flags, state rows, duty heads, valuations and
/// rates. The name is a `MasterName`; the parent and unit equal a row name in
/// the same company's group or unit read; the dates were parsed by the protocol
/// crate; and the HSN code and description do not hold the root marker. Each
/// rate is a plain non-zero decimal on the three rows that carry one and absent
/// on the two that do not; its magnitude is not checked, and neither is the
/// unit's value. The draft does not hold text to plain text; the renderer does.
/// Bridge does not know how Tally treats values other than the measured ones.
#[derive(Debug)]
pub(crate) struct StockItemDraft<'a> {
    sibling: GoodsSibling<'a>,
    writes: StockItemWrites,
}

impl<'a> StockItemDraft<'a> {
    /// Checks the proposed name and sibling against the same company's group
    /// and unit reads, then admits the first measured copy's structure (§12a.13).
    #[cfg(test)]
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
        let units = parse_native_masters(NativeMasterKind::Units, units, company_guid)
            .map_err(DraftRefusal::UnitsUnreadable)?;
        Self::from_rows(name, sibling, &groups.rows, &units.rows)
    }

    /// Builds a draft only from the admitted, already parsed support reads.
    pub(crate) fn new_admitted(
        name: ItemNameAbsent<'a>,
        sibling: GoodsSibling<'a>,
        groups: &AdmittedStockGroups,
        units: &AdmittedStockUnits,
    ) -> Result<Self, DraftRefusal> {
        let company = name.catalogue().company();
        if groups.company() != company || units.company() != company {
            return Err(DraftRefusal::OtherCatalogue);
        }
        Self::from_rows(name, sibling, groups.rows(), units.rows())
    }

    fn from_rows(
        name: ItemNameAbsent<'a>,
        sibling: GoodsSibling<'a>,
        groups: &[NativeMasterRow],
        units: &[NativeMasterRow],
    ) -> Result<Self, DraftRefusal> {
        if !std::ptr::eq(name.catalogue(), sibling.sibling().catalogue()) {
            return Err(DraftRefusal::OtherCatalogue);
        }
        let parent = sibling.sibling().parent();
        if !groups.iter().any(|row| row.name == parent) {
            return Err(DraftRefusal::ParentNotInBook);
        }
        let unit = sibling.sibling().unit();
        if !units.iter().any(|row| row.name == unit) {
            return Err(DraftRefusal::UnitNotInBook);
        }
        let (gst, hsn) = measured_shape(sibling.sibling())?;
        let writes = StockItemWrites {
            name: name.name().clone(),
            sibling_guid: sibling.sibling().guid().to_owned(),
            sibling_source_name: sibling.sibling().source_name().clone(),
            parent: parent.to_owned(),
            unit: unit.to_owned(),
            gst,
            hsn,
        };
        Ok(Self { sibling, writes })
    }

    /// Every value the Create writes, owned.
    pub(crate) fn writes(&self) -> &StockItemWrites {
        &self.writes
    }

    /// The proposed name proved absent from this catalogue.
    pub(crate) fn name(&self) -> &MasterName {
        self.writes.name()
    }

    /// The parent exact name proved present in the same company's group read.
    pub(crate) fn parent(&self) -> &str {
        self.writes.parent()
    }

    /// The base-unit exact name proved present in the same company's unit read.
    pub(crate) fn unit(&self) -> &str {
        self.writes.unit()
    }

    /// The one dated GST entry in the measured shape.
    pub(crate) fn gst(&self) -> &DraftGstEntry {
        self.writes.gst()
    }

    /// The one dated HSN entry in the measured shape.
    pub(crate) fn hsn(&self) -> &DraftHsnEntry {
        self.writes.hsn()
    }

    /// The Goods sibling whose six fields are present, from which this draft was built.
    pub(crate) fn sibling(&self) -> &GoodsSibling<'a> {
        &self.sibling
    }
}

/// A sibling's GST and HSN lists as the first measured copy's entries
/// (§12a.13), refusing in the order `StockItemDraft::new` refuses them.
pub(super) fn measured_shape(
    sibling: &SiblingGst<'_>,
) -> Result<(DraftGstEntry, DraftHsnEntry), DraftRefusal> {
    let gst = match sibling.gst() {
        [details] => details,
        entries => {
            return Err(DraftRefusal::GstEntriesNotMeasured {
                entries: entries.len(),
            });
        }
    };
    let hsn = match sibling.hsn() {
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
    let mut rates = Vec::with_capacity(state.rates.len());
    for (row, read) in state.rates.iter().enumerate() {
        let valuation = RateValuation::from_read(read.valuation_type.as_deref());
        rates.push(measured_rate(
            row,
            read.duty_head.as_deref(),
            valuation,
            &read.rate,
        )?);
    }
    let rates = all_measured_rows(rates)?;
    let gst = DraftGstEntry {
        applicable_from: gst.applicable_from.clone(),
        rates,
    };

    let (code, description) = match (&hsn.hsn_code, &hsn.description) {
        (Some(code), Some(description))
            if hsn.source.as_deref() == Some(SPECIFY_DETAILS_HERE)
                && !code.contains(TALLY_SANITIZED_ROOT_MARKER)
                && !description.contains(TALLY_SANITIZED_ROOT_MARKER) =>
        {
            (code.clone(), description.clone())
        }
        _ => return Err(DraftRefusal::HsnNotMeasured),
    };
    let hsn = DraftHsnEntry {
        applicable_from: hsn.applicable_from.clone(),
        code,
        description,
    };
    Ok((gst, hsn))
}

/// Admits the duty row at `row` as the measured row at that position
/// (§12a.13), refusing its head, then its valuation, then its rate: a plain
/// non-zero decimal where the measured row carries one, absent where not.
/// `measured_shape` and `StockItemWrites::rebuild` share it.
fn measured_rate(
    row: usize,
    duty_head: Option<&str>,
    valuation: Option<RateValuation>,
    rate: &GstRate,
) -> Result<DraftRate, DraftRefusal> {
    let Some((measured_head, measured_valuation, rate_required)) = MEASURED_RATE_ROWS.get(row)
    else {
        return Err(DraftRefusal::DutyHeadNotMeasured { row });
    };
    if duty_head != Some(*measured_head) {
        return Err(DraftRefusal::DutyHeadNotMeasured { row });
    }
    if valuation != Some(*measured_valuation) {
        return Err(DraftRefusal::ValuationNotMeasured { row });
    }
    let rate = match (rate, rate_required) {
        (GstRate::Value { decimal, .. }, true) => Some(decimal.clone()),
        (GstRate::Absent, false) => None,
        _ => return Err(DraftRefusal::RateNotMeasured { row }),
    };
    Ok(DraftRate {
        duty_head: measured_head,
        valuation: *measured_valuation,
        rate,
    })
}

/// Refuses fewer rows than the five measured, at the first missing row; an
/// extra row is refused by `measured_rate`.
fn all_measured_rows(rates: Vec<DraftRate>) -> Result<Vec<DraftRate>, DraftRefusal> {
    if rates.len() < MEASURED_RATE_ROWS.len() {
        return Err(DraftRefusal::DutyHeadNotMeasured { row: rates.len() });
    }
    Ok(rates)
}

pub(crate) use prepared::PreparedStockItem;

/// A private module so that the only code able to build a `PreparedStockItem` is
/// `from_draft`, which takes the draft by value: no constructor or conversion
/// takes bare writes, so writes rebuilt from storage never reach a Create.
mod prepared {
    use super::{
        create_text_refusal, CreateRenderRefusal, StockItemDraft, StockItemWrites,
        VerifiedCompanyIdentity,
    };

    /// A stock-item Create made from a draft, owned: the verified company its
    /// catalogue was read for and every value it writes, held to the renderer's
    /// text rule. It proves no approval; the approval is the `ApprovedImport`
    /// that holds it.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct PreparedStockItem {
        company: VerifiedCompanyIdentity,
        writes: StockItemWrites,
    }

    impl PreparedStockItem {
        /// Takes the draft's writes and its catalogue's company, refusing what
        /// `create_text_refusal` refuses.
        pub(crate) fn from_draft(draft: StockItemDraft<'_>) -> Result<Self, CreateRenderRefusal> {
            create_text_refusal(&draft.writes)?;
            let company = draft.sibling.sibling().catalogue().company().clone();
            Ok(Self {
                company,
                writes: draft.writes,
            })
        }

        pub(crate) fn company(&self) -> &VerifiedCompanyIdentity {
            &self.company
        }

        pub(crate) fn writes(&self) -> &StockItemWrites {
            &self.writes
        }
    }
}

/// The measured Create wrote these five values as plain text (§12a.13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DraftText {
    Name,
    Parent,
    Unit,
    HsnCode,
    HsnDescription,
}

/// Why a value cannot be written as the measured Create's plain text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CreateRenderRefusal {
    /// The value is not plain ASCII text of the kind `MasterName` admits: it is
    /// empty, non-ASCII, holds a control character, has a space at either end,
    /// has two spaces together, or is too long. No Create with such a value was
    /// measured (§12a.13).
    TextNotPlain { field: DraftText },
    /// No Create with one of these five characters in the value was measured,
    /// so the renderer writes no escape and refuses it (§12a.13).
    TextNeedsEscaping { field: DraftText },
}

/// The renderer's text rule, in its order: Name, Parent, Unit, HsnCode and
/// HsnDescription, returning the first refusal. Name receives only the
/// five-character check because it is a `MasterName`. Each other value must
/// first pass the `MasterName` rule, then hold none of `& < > " '`. The
/// preview applies the same rule, so a draft the renderer refuses is never
/// shown for approval.
pub(crate) fn create_text_refusal(writes: &StockItemWrites) -> Result<(), CreateRenderRefusal> {
    for (field, text) in [
        (DraftText::Name, writes.name().as_str()),
        (DraftText::Parent, writes.parent()),
        (DraftText::Unit, writes.unit()),
        (DraftText::HsnCode, writes.hsn().code()),
        (DraftText::HsnDescription, writes.hsn().description()),
    ] {
        // The closed rule also refuses an HSN description or a group or unit
        // name with two spaces together, non-ASCII text or over 16,384 bytes;
        // this is Bridge's choice, not a Tally fact.
        if field != DraftText::Name && MasterName::new(text).is_err() {
            return Err(CreateRenderRefusal::TextNotPlain { field });
        }
        if text
            .chars()
            .any(|c| matches!(c, '&' | '<' | '>' | '"' | '\''))
        {
            return Err(CreateRenderRefusal::TextNeedsEscaping { field });
        }
    }
    Ok(())
}

/// The one renderer of a stock item Create: it takes only a `PreparedStockItem`,
/// whose one maker has applied `create_text_refusal`, sends nothing, and renders
/// the structure of the first marked Create measured (one run, PARTIAL,
/// §12a.13), with values read from the sibling. The rate's magnitude is not
/// checked, and Bridge does not know how Tally treats values other than the
/// measured ones.
pub(crate) fn render_stock_item_create(prepared: &PreparedStockItem) -> String {
    render_writes(prepared.company().display_name(), prepared.writes())
}

/// The body of the one renderer, over the company's name and writes the text
/// rule admitted; `create_sha256` takes only its digest.
fn render_writes(company: &str, writes: &StockItemWrites) -> String {
    let name = writes.name().as_str();
    let parent = writes.parent();
    let unit = writes.unit();
    let applicability = MarkedConstant::Applicable.render();
    let gst = writes.gst();
    let applicable_from = gst.applicable_from();
    let state = MarkedConstant::Any.render();
    let mut message = format!(
        "<TALLYMESSAGE><STOCKITEM NAME=\"{name}\" ACTION=\"Create\"><NAME>{name}</NAME><PARENT>{parent}</PARENT><BASEUNITS>{unit}</BASEUNITS><GSTAPPLICABLE>{applicability}</GSTAPPLICABLE><GSTTYPEOFSUPPLY>{GOODS}</GSTTYPEOFSUPPLY><GSTDETAILS.LIST><APPLICABLEFROM>{applicable_from}</APPLICABLEFROM><TAXABILITY>{TAXABLE}</TAXABILITY><SRCOFGSTDETAILS>{SPECIFY_DETAILS_HERE}</SRCOFGSTDETAILS><STATEWISEDETAILS.LIST><STATENAME>{state}</STATENAME>"
    );
    for rate in gst.rates() {
        message.push_str("<RATEDETAILS.LIST><GSTRATEDUTYHEAD>");
        message.push_str(rate.duty_head());
        message.push_str("</GSTRATEDUTYHEAD><GSTRATEVALUATIONTYPE>");
        message.push_str(rate.valuation().render());
        message.push_str("</GSTRATEVALUATIONTYPE>");
        if let Some(value) = rate.rate() {
            message.push_str("<GSTRATE>");
            message.push_str(value);
            message.push_str("</GSTRATE>");
        }
        message.push_str("</RATEDETAILS.LIST>");
    }
    message.push_str(
        "<GSTSLABRATES.LIST></GSTSLABRATES.LIST></STATEWISEDETAILS.LIST></GSTDETAILS.LIST>",
    );
    let hsn = writes.hsn();
    let hsn_from = hsn.applicable_from();
    let code = hsn.code();
    let description = hsn.description();
    message.push_str(&format!(
        "<HSNDETAILS.LIST><APPLICABLEFROM>{hsn_from}</APPLICABLEFROM><HSNCODE>{code}</HSNCODE><HSN>{description}</HSN><SRCOFHSNDETAILS>{SPECIFY_DETAILS_HERE}</SRCOFHSNDETAILS></HSNDETAILS.LIST></STOCKITEM></TALLYMESSAGE>"
    ));

    render_import_envelope(company, ImportReport::AllMasters, &message)
}

#[cfg(test)]
#[path = "stock_item_create_tests.rs"]
pub(crate) mod tests;
