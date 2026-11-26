//! The stock-item kind of the master journal (#1381): its batch line, in a
//! file of its own (`agent-master-stock-item.jsonl`), and
//! [`StockItemReadBackBasis`], the one value a stock-item read-back takes.
//!
//! No shipped path writes this file yet: [`record_stock_item_intent`], its
//! intent writer, has no shipped caller. The intent, the pre-image sidecar and
//! the record tags are the master journal's own, generic over the batch kind;
//! neither the intent nor the sidecar records a kind, because the file is the
//! kind.
//!
//! A stock item and a ledger of the same name do not block each other: each
//! journal's open-dispatch overlap check sees only its own file. That rests on
//! the two files being separate and on nothing else; the protocol reference
//! does not record whether Tally keeps stock items and ledgers apart by name.
use super::super::master_post::StockItemRecheckPassed;
use super::super::master_tools::journal_refusal;
use super::{
    append_kind_intent, check_company, check_ids, read_kind_journal, read_pre_image,
    remove_pre_image, write_pre_image, ExclusiveAdmission, Invalid, Journal, JournalBatch,
    MasterCompany, MasterDispatchIntent, MasterLedgerError, PreImage, PreImageError,
    RecordedIntent,
};
use crate::tally::approved_import::UnderLockRefusal;
use crate::tally::stock_item_create::{
    CreateRenderRefusal, PlainRate, PlainStockItemWrites, StockItemCatalogue, StockItemWrites,
    WritesRebuildRefusal,
};
use crate::tally::stock_item_verified::{
    NotVerified, ReadBackVerdict, StockItemImportOutcome, VerifiedItem,
};
use crate::tally::VerifiedCompanyIdentity;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

/// The one class of stock-item batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum StockItemClass {
    StockItemCreate,
}

/// One stored duty row of [`StoredStockItemCreate`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StoredStockRate {
    duty_head: String,
    rate: Option<String>,
}

/// The stored mirror of `PlainStockItemWrites`, field for field. It is read
/// back only through `StockItemWrites::rebuild`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StoredStockItemCreate {
    name: String,
    sibling_guid: String,
    sibling_source_name: String,
    parent: String,
    unit: String,
    gst_applicable_from: String,
    rates: Vec<StoredStockRate>,
    hsn_applicable_from: String,
    hsn_code: String,
    hsn_description: String,
}

impl StoredStockItemCreate {
    fn of(plain: PlainStockItemWrites) -> Self {
        Self {
            name: plain.name,
            sibling_guid: plain.sibling_guid,
            sibling_source_name: plain.sibling_source_name,
            parent: plain.parent,
            unit: plain.unit,
            gst_applicable_from: plain.gst_applicable_from,
            rates: (plain.rates.into_iter())
                .map(|PlainRate { duty_head, rate }| StoredStockRate { duty_head, rate })
                .collect(),
            hsn_applicable_from: plain.hsn_applicable_from,
            hsn_code: plain.hsn_code,
            hsn_description: plain.hsn_description,
        }
    }

    /// The stored values through `StockItemWrites::rebuild`; the struct
    /// literal names every plain field.
    fn rebuild(&self) -> Result<StockItemWrites, WritesRebuildRefusal> {
        let stored = self.clone();
        StockItemWrites::rebuild(&PlainStockItemWrites {
            name: stored.name,
            sibling_guid: stored.sibling_guid,
            sibling_source_name: stored.sibling_source_name,
            parent: stored.parent,
            unit: stored.unit,
            gst_applicable_from: stored.gst_applicable_from,
            rates: (stored.rates.into_iter())
                .map(|StoredStockRate { duty_head, rate }| PlainRate { duty_head, rate })
                .collect(),
            hsn_applicable_from: stored.hsn_applicable_from,
            hsn_code: stored.hsn_code,
            hsn_description: stored.hsn_description,
        })
    }
}

/// One stock-item Create as its journal records it. Built only by
/// [`StockItemBatch::new`], which takes the digest from the writes; a line read
/// back is admitted by `validate`, which rebuilds the stored item but cannot
/// check the digest without the company as verified afresh. That check is
/// [`StockItemReadBackBasis::from_journal`]'s.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StockItemBatch {
    batch_id: String,
    /// `StockItemWrites::create_sha256` for the verified company.
    batch_sha256: String,
    class: StockItemClass,
    company: MasterCompany,
    item: StoredStockItemCreate,
}

impl StockItemBatch {
    pub(super) fn new(
        batch_id: String,
        identity: &VerifiedCompanyIdentity,
        writes: &StockItemWrites,
    ) -> Result<Self, CreateRenderRefusal> {
        Ok(Self {
            batch_sha256: writes.create_sha256(identity)?,
            batch_id,
            class: StockItemClass::StockItemCreate,
            company: MasterCompany {
                guid: identity.company_guid().to_owned(),
                name: identity.display_name().to_owned(),
                number: identity.company_number().to_owned(),
                books_from: identity.books_from_yyyymmdd().to_owned(),
            },
            item: StoredStockItemCreate::of(writes.to_plain()),
        })
    }
}

impl JournalBatch for StockItemBatch {
    const JOURNAL_FILE: &'static str = "agent-master-stock-item.jsonl";

    fn validate(&self) -> Result<(), Invalid> {
        check_ids(&self.batch_id, &self.batch_sha256)?;
        check_company(&self.company)?;
        match self.class {
            StockItemClass::StockItemCreate => {}
        }
        self.item
            .rebuild()
            .map(|_| ())
            .map_err(|_| Invalid::ItemNotRebuilt)
    }

    fn batch_id(&self) -> &str {
        &self.batch_id
    }

    fn batch_sha256(&self) -> &str {
        &self.batch_sha256
    }

    fn company_guid(&self) -> &str {
        &self.company.guid
    }

    fn names(&self) -> Vec<&str> {
        vec![self.item.name.as_str()]
    }
}

/// The GUIDs of the company's masters before the Create. Built only from a
/// [`PreImage`], the sidecar the intent's digest and count were taken from.
#[derive(Debug)]
pub(super) struct BeforeGuids(BTreeSet<String>);

impl BeforeGuids {
    fn of(pre_image: &PreImage) -> Self {
        Self(pre_image.entries().keys().cloned().collect())
    }
}

/// Why no [`StockItemReadBackBasis`] was built, one variant per step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum BasisRefusal {
    NoBatch,
    NoIntent,
    OtherCompanyName,
    OtherCompanyGuid,
    OtherCompanyNumber,
    OtherBooksFrom,
    NotRebuilt(WritesRebuildRefusal),
    NotRendered(CreateRenderRefusal),
    /// The stored writes, rendered for the company as verified now, do not
    /// give the batch's digest.
    DigestMismatch,
    /// The intent records another request digest than the batch's.
    RequestDigestMismatch,
    PreImage(PreImageError),
    /// A sidecar GUID that is not one of the batch's company.
    BeforeOfOtherCompany,
}

/// Whether `guid` is a master GUID of the company `company_guid`: the company's
/// GUID, a hyphen, then the master's own part. Both are lower-case canonical
/// here, the sidecar's by `PreImage::new` and the batch's by its admission.
fn of_company(guid: &str, company_guid: &str) -> bool {
    guid.strip_prefix(company_guid)
        .is_some_and(|rest| rest.starts_with('-'))
}

/// The company, the writes and the before GUIDs of one stock-item read-back,
/// all taken from one admitted batch, its intent and its sidecar, so no
/// read-back can pair one batch's writes with another's company or before set.
#[derive(Debug)]
pub(super) struct StockItemReadBackBasis {
    company: VerifiedCompanyIdentity,
    writes: StockItemWrites,
    before: BeforeGuids,
}

impl StockItemReadBackBasis {
    /// The basis of `batch_id` in `journal`, for `identity` as verified afresh.
    pub(super) fn from_journal(
        data_dir: &Path,
        journal: &Journal<StockItemBatch>,
        batch_id: &str,
        identity: &VerifiedCompanyIdentity,
    ) -> Result<Self, BasisRefusal> {
        let state = journal.state(batch_id).ok_or(BasisRefusal::NoBatch)?;
        let intent = state.intent.as_ref().ok_or(BasisRefusal::NoIntent)?;
        let company = &state.batch.company;
        if identity.display_name() != company.name {
            return Err(BasisRefusal::OtherCompanyName);
        }
        if !identity.company_guid().eq_ignore_ascii_case(&company.guid) {
            return Err(BasisRefusal::OtherCompanyGuid);
        }
        if identity.company_number() != company.number {
            return Err(BasisRefusal::OtherCompanyNumber);
        }
        if identity.books_from_yyyymmdd() != company.books_from {
            return Err(BasisRefusal::OtherBooksFrom);
        }
        let writes = state
            .batch
            .item
            .rebuild()
            .map_err(BasisRefusal::NotRebuilt)?;
        let digest = writes
            .create_sha256(identity)
            .map_err(BasisRefusal::NotRendered)?;
        if digest != state.batch.batch_sha256 {
            return Err(BasisRefusal::DigestMismatch);
        }
        if intent.native_request_sha256() != state.batch.batch_sha256 {
            return Err(BasisRefusal::RequestDigestMismatch);
        }
        let pre_image = read_pre_image(
            data_dir,
            batch_id,
            intent.pre_image_sha256(),
            intent.pre_image_count(),
        )
        .map_err(BasisRefusal::PreImage)?;
        // A sidecar built from another company's read would leave this
        // company's item of the same name out of the before set, so a row that
        // is not new could read as new.
        if !(pre_image.entries().keys()).all(|guid| of_company(guid, &company.guid)) {
            return Err(BasisRefusal::BeforeOfOtherCompany);
        }
        Ok(Self {
            company: identity.clone(),
            writes,
            before: BeforeGuids::of(&pre_image),
        })
    }

    pub(super) fn read_back(&self, fresh: &StockItemCatalogue) -> ReadBackVerdict {
        ReadBackVerdict::of(&self.company, &self.writes, &self.before.0, fresh)
    }

    pub(super) fn verify(
        &self,
        outcome: StockItemImportOutcome,
        fresh: &StockItemCatalogue,
    ) -> Result<VerifiedItem, NotVerified> {
        VerifiedItem::new(outcome, &self.company, &self.writes, &self.before.0, fresh)
    }
}

/// Why [`record_stock_item_intent`] recorded no intent. Every variant but
/// `Other` was decided before any byte of the intent was written. Two can
/// leave a sidecar that names no intent, as the ledger writer's can:
/// `Refused(PreImageWriteFailed)` when the directory sync failed after the
/// file was placed or another sidecar was already in place (`ExistsDifferent`),
/// and `Refused(PreImageRemoveFailed)`.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum StockIntentRefusal {
    /// The recheck passed for another batch than the one to record.
    OtherBatch,
    /// The recheck's digest is not the batch's: an intent that no read-back
    /// could verify is not written.
    OtherDigest,
    Refused(UnderLockRefusal),
    /// The intent may be in the file: the batch is treated as sent.
    Other(String),
}

/// The stock twin of the ledger post's `record_master_intent`, under the
/// exclusive lock and in its order: the batch undispatched and unchanged, the
/// token of that batch and digest, the journal's admission, then the spend, the
/// sidecar and the intent. Nothing shipped calls it yet.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) fn record_stock_item_intent(
    data_dir: &Path,
    lock: &ExclusiveAdmission,
    batch: &StockItemBatch,
    passed: StockItemRecheckPassed,
    spend: impl FnOnce() -> Result<(), UnderLockRefusal>,
) -> Result<RecordedIntent, StockIntentRefusal> {
    use StockIntentRefusal::{Other, Refused};
    let journal = read_kind_journal::<StockItemBatch>(data_dir)
        .map_err(|_| Refused(UnderLockRefusal::MasterJournalUnavailable))?;
    let current =
        (journal.state(&batch.batch_id)).ok_or(Refused(UnderLockRefusal::BatchNotFound))?;
    if current.intent.is_some() {
        return Err(Refused(UnderLockRefusal::AlreadyAttempted));
    }
    if current.batch != *batch {
        return Err(Refused(UnderLockRefusal::BatchChanged));
    }
    let (create_sha256, pre_image, marks, batch_id) = passed.into_intent_parts();
    if batch_id != batch.batch_id {
        return Err(StockIntentRefusal::OtherBatch);
    }
    if create_sha256 != batch.batch_sha256 {
        return Err(StockIntentRefusal::OtherDigest);
    }
    let refused = |error| match journal_refusal(error) {
        Ok(refusal) => Refused(refusal),
        Err(code) => Other(code),
    };
    journal
        .check_intent_admissible(&current.batch, &marks)
        .map_err(refused)?;
    spend().map_err(Refused)?;
    let written = write_pre_image(data_dir, lock, &current.batch, &pre_image)
        .map_err(|_| Refused(UnderLockRefusal::PreImageWriteFailed))?;
    let intent = MasterDispatchIntent::new(&current.batch, create_sha256, written, marks)
        .map_err(|_| MasterLedgerError::Invalid(Invalid::BadDigest));
    let appended =
        intent.and_then(|intent| append_kind_intent::<StockItemBatch>(data_dir, lock, intent));
    appended.map_err(|error| match refused(error) {
        // Refused before any byte of the intent was written: the sidecar names
        // no intent, so it is removed while the lock is held.
        Refused(refusal) => match remove_pre_image(data_dir, lock, &batch.batch_id) {
            Ok(()) => Refused(refusal),
            Err(_) => Refused(UnderLockRefusal::PreImageRemoveFailed),
        },
        other => other,
    })
}

#[cfg(test)]
#[path = "agent_master_stock_item_journal_tests.rs"]
mod tests;
