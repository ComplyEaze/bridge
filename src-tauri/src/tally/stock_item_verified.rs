//! The last step of creating one stock item (#1381): the import answer to a
//! Create, a fresh read of the item checked against what was shown, and the
//! one type that holds both.
//!
//! A Create's counters alone do not show that the item was stored as sent: row
//! 1381-R1 of `TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md`
//! §12a.13 records a Create the counters reported clean whose stored item a
//! fresh read showed to differ. So `VerifiedItem` is built only from a clean
//! answer together with a fresh read whose new row equals the shown draft:
//! `VerifiedItem::new` takes the shown draft and the fresh read and judges the
//! read itself, so no verdict made for another draft can be passed in.
//! Outside this module a clean answer cannot be made by hand: `CountersClean`
//! holds a `CleanCreate` that only `StockItemImportOutcome::of` makes (and
//! `Equal` an `EqualRead` that only `ReadBackVerdict::of` makes).
//!
//! `VerifiedItem` does not prove:
//! - that the fresh read was taken after the Create was posted (the caller's
//!   job);
//! - that a window showed the draft (`ShownDraft::from_draft` is a plain
//!   constructor);
//! - that the fresh read is the whole book (size and count admission are the
//!   caller's, as the module doc of `stock_item_create.rs` says);
//! - that the answer was Tally's answer to this draft's Create: it takes the
//!   outcome as it is passed.
//!
//! The tests read the captures in
//! `bridge-tally-protocol/tests/fixtures/agent/stock-create-lab/`; see their
//! `PROVENANCE.md` (PARTIAL: one book, one release, one sitting).

// Nothing outside the tests calls this until the stock-item tools exist.
#![cfg_attr(not(test), allow(dead_code))]

use super::master_name::MasterName;
use super::stock_item_create::{
    measured_shape, missing_fields, DraftRefusal, GoodsSibling, NotGoodsSibling, ShownDraft,
    SiblingField, SiblingGst, SiblingRefusal, StockItemCatalogue,
};
use bridge_tally_protocol::{ImportCounters, TallyImportApplicationStatus, TallyImportOutcome};

/// The proof inside `CountersClean`. Its field is private, so only
/// `StockItemImportOutcome::of` makes one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CleanCreate(());

/// What the import answer to one stock item Create says, by the protocol
/// crate's own counter rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StockItemImportOutcome {
    /// The answer's `HEADER/STATUS` reads 0, the application's failure,
    /// whatever its counters say.
    ApplicationFailure,
    /// `CREATED 1`, every other counter reported 0, no `LINEERROR`, and no
    /// failure status. A bare `RESPONSE`, as captured, reports no status.
    CountersClean(CleanCreate),
    /// No item created and at least one altered: the counters report an
    /// existing item altered, not a new one (§12a.13 records such answers to a
    /// Create on a name an item already held, row 1381-N among them).
    AlteredExisting {
        counters: ImportCounters,
        line_errors: u64,
    },
    /// Every counter reported, and the answer is neither of the two above.
    NotClean {
        counters: ImportCounters,
        line_errors: u64,
    },
    /// At least one counter was not reported. A counter Tally did not report
    /// is not a zero.
    Unreported,
}

impl StockItemImportOutcome {
    /// Classifies the answer by its status, then with `reported_counters` and
    /// `is_clean_success_for(1, 0, 0)`; it writes no counter rule of its own.
    /// The status check is the one `import_outcome_is_clean` makes for a
    /// voucher post.
    pub(crate) fn of(outcome: &TallyImportOutcome) -> Self {
        if outcome.application_status() == TallyImportApplicationStatus::Failure {
            return Self::ApplicationFailure;
        }
        let result = outcome.counters();
        let Some(counters) = result.reported_counters() else {
            return Self::Unreported;
        };
        let line_errors = result.line_error_count;
        if result.is_clean_success_for(1, 0, 0) {
            Self::CountersClean(CleanCreate(()))
        } else if counters.created == 0 && counters.altered >= 1 {
            Self::AlteredExisting {
                counters,
                line_errors,
            }
        } else {
            Self::NotClean {
                counters,
                line_errors,
            }
        }
    }
}

/// The proof inside `Equal`: the new row's GUID. Its field is private, so only
/// `ReadBackVerdict::of` makes one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EqualRead {
    guid: String,
}

/// What a fresh stock item read says of the item a shown draft proposed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReadBackVerdict {
    /// One new row holds the draft's name as its row name, none of its six
    /// fields is missing, `GoodsSibling` admits its applicability, type of
    /// supply and taxability, and its parent, unit, GST entry and HSN entry
    /// equal the draft's.
    Equal(EqualRead),
    /// When any field is missing, the missing fields only; otherwise the
    /// fields that differ from the draft, up to a refusal (see `of`). In
    /// `SiblingField` order. Never empty.
    Differs { fields: Vec<SiblingField> },
    /// Not exactly one item holds the draft's name as its row name or a name
    /// entry, or the one that does holds it only as a name entry; `rows` is how
    /// many items hold it.
    NotFoundOnce { rows: usize },
    /// The one item holding the name has a GUID the before catalogue held.
    NotNew,
    /// The fresh read was made for another company than the before catalogue.
    OtherCompany,
}

impl ReadBackVerdict {
    /// Checks `fresh` against `shown`. The before catalogue is the one the
    /// shown draft's sibling was found in.
    ///
    /// The six fields are read by the code the sibling is read by
    /// (`missing_fields`, `SiblingGst::named`, `GoodsSibling::from_sibling`
    /// and `measured_shape`), then compared with the draft by `==`. When any
    /// field is missing from the read, only the missing fields are listed and
    /// nothing is compared. A refusal by
    /// `GoodsSibling::from_sibling` or `measured_shape` names the field it is
    /// about and ends the comparison of the GST and HSN entries, so the list
    /// then names every field found to differ up to that refusal and may not
    /// name a later one. Rates are compared as the text read, exactly: a rate
    /// sent as `2.5` and read as `2.50` differs (§12a.13, row 1381-O, o4).
    pub(crate) fn of(shown: &ShownDraft<'_>, fresh: &StockItemCatalogue) -> Self {
        let draft = shown.draft();
        let before = draft.sibling().sibling().catalogue();
        // `VerifiedCompanyIdentity` derives equality over its whole tuple.
        if fresh.company() != before.company() {
            return Self::OtherCompany;
        }

        let name = draft.name().as_str();
        let holding: Vec<_> = fresh
            .items()
            .iter()
            .filter(|item| item.name == name || item.names.iter().any(|entry| entry == name))
            .collect();
        let item = match holding.as_slice() {
            [item] if item.name == name => *item,
            _ => {
                return Self::NotFoundOnce {
                    rows: holding.len(),
                }
            }
        };

        if before
            .items()
            .iter()
            .any(|existing| existing.guid == item.guid)
        {
            return Self::NotNew;
        }

        let missing = missing_fields(item);
        if !missing.is_empty() {
            return Self::Differs { fields: missing };
        }
        let sibling = match SiblingGst::named(fresh, draft.name()) {
            Ok(sibling) => sibling,
            // Not reached after the two checks above, which make the same
            // tests on the same row; each arm still refuses.
            Err(SiblingRefusal::Incomplete(field)) => {
                return Self::Differs {
                    fields: vec![field],
                }
            }
            Err(
                SiblingRefusal::NotFound
                | SiblingRefusal::Ambiguous { .. }
                | SiblingRefusal::OnlyByFold { .. }
                | SiblingRefusal::Unchecked(_),
            ) => {
                return Self::NotFoundOnce {
                    rows: holding.len(),
                }
            }
        };

        let mut fields = Vec::new();
        if sibling.parent() != draft.parent() {
            fields.push(SiblingField::Parent);
        }
        if sibling.unit() != draft.unit() {
            fields.push(SiblingField::Unit);
        }
        match GoodsSibling::from_sibling(sibling) {
            Err(refusal) => fields.push(goods_refusal_field(&refusal)),
            Ok(goods) => match measured_shape(goods.sibling()) {
                Err(refusal) => fields.push(shape_refusal_field(&refusal)),
                Ok((gst, hsn)) => {
                    if gst != *draft.gst() {
                        fields.push(SiblingField::GstDetails);
                    }
                    if hsn != *draft.hsn() {
                        fields.push(SiblingField::HsnDetails);
                    }
                }
            },
        }

        if fields.is_empty() {
            Self::Equal(EqualRead {
                guid: item.guid.clone(),
            })
        } else {
            Self::Differs { fields }
        }
    }
}

/// The field a `GoodsSibling` refusal is about.
fn goods_refusal_field(refusal: &NotGoodsSibling) -> SiblingField {
    match refusal {
        NotGoodsSibling::NotGoods => SiblingField::TypeOfSupply,
        NotGoodsSibling::NotTaxable { .. } => SiblingField::GstDetails,
        NotGoodsSibling::GstNotApplicable => SiblingField::Applicability,
    }
}

/// The field a `measured_shape` refusal is about.
fn shape_refusal_field(refusal: &DraftRefusal) -> SiblingField {
    match refusal {
        DraftRefusal::HsnEntriesNotMeasured { .. } | DraftRefusal::HsnNotMeasured => {
            SiblingField::HsnDetails
        }
        DraftRefusal::GstEntriesNotMeasured { .. }
        | DraftRefusal::GstSourceNotMeasured
        | DraftRefusal::ItcNotMeasured
        | DraftRefusal::StateNotAny
        | DraftRefusal::DutyHeadNotMeasured { .. }
        | DraftRefusal::ValuationNotMeasured { .. }
        | DraftRefusal::RateNotMeasured { .. } => SiblingField::GstDetails,
        // `measured_shape` does not return these; listed so that a new
        // variant must be placed here, and placed where it still refuses.
        DraftRefusal::OtherCatalogue
        | DraftRefusal::GroupsUnreadable(_)
        | DraftRefusal::ParentNotInBook => SiblingField::Parent,
        DraftRefusal::UnitsUnreadable(_) | DraftRefusal::UnitNotInBook => SiblingField::Unit,
    }
}

/// A stock item whose Create was answered clean and whose fresh read equals
/// the shown draft. See the module doc for what it does not prove.
#[derive(Debug)]
pub(crate) struct VerifiedItem {
    guid: String,
    name: MasterName,
}

/// Why no `VerifiedItem` was built: the outcome as given and the verdict
/// `new` reached.
#[derive(Debug)]
pub(crate) struct NotVerified {
    pub(crate) outcome: StockItemImportOutcome,
    pub(crate) verdict: ReadBackVerdict,
}

impl VerifiedItem {
    /// Judges `fresh` against `shown` with `ReadBackVerdict::of`, and is `Ok`
    /// only for `CountersClean` with `Equal`; the GUID is the read's and the
    /// name the shown draft's.
    pub(crate) fn new(
        outcome: StockItemImportOutcome,
        shown: &ShownDraft<'_>,
        fresh: &StockItemCatalogue,
    ) -> Result<Self, NotVerified> {
        match (outcome, ReadBackVerdict::of(shown, fresh)) {
            (StockItemImportOutcome::CountersClean(_), ReadBackVerdict::Equal(read)) => Ok(Self {
                guid: read.guid,
                name: shown.draft().name().clone(),
            }),
            (outcome, verdict) => Err(NotVerified { outcome, verdict }),
        }
    }

    /// The created item's GUID, as the fresh read gave it.
    pub(crate) fn guid(&self) -> &str {
        &self.guid
    }

    /// The created item's name, as the shown draft held it.
    pub(crate) fn name(&self) -> &MasterName {
        &self.name
    }
}

#[cfg(test)]
#[path = "stock_item_verified_tests.rs"]
mod tests;
