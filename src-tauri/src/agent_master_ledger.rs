//! The master write journal: `agent-master-ledger.jsonl`, beside the voucher
//! import journal and never mixed with it.
//!
//! A master record cannot live in the voucher journal. Its reader takes any
//! line without `record_type` for a voucher batch, admits a request digest only
//! on a voucher dispatch intent, and keys its duplicate-dispatch refusal on
//! that intent. A master line there would either refuse every voucher post or
//! slip past the refusal that stops a second send. A file of its own, with its
//! own typed records, leaves the voucher journal untouched, and a build that
//! predates this file never opens it, so a downgrade cannot send a master
//! write twice.
//!
//! The one door into the journal is [`MasterJournal::admit`], used both to read
//! a line back and to check a record before it is appended: a record the reader
//! would refuse is never written.
//!
//! Every write takes an [`ExclusiveAdmission`]. It is built only by
//! [`ExclusiveAdmission::acquire`], which takes the import admission lock
//! exclusively itself, the lock `Server::lock_import_admission` takes; there is no
//! conversion from a `File`, so a shared lock (`lock_import_admission_shared`) or
//! no lock at all cannot reach a write. Appending an intent returns a
//! [`RecordedIntent`], and the send that follows must consume it: a send cannot be
//! built without an intent recorded under the exclusive lock.
//! [`MasterJournal::dispatched`] reads without the lock and is for reporting,
//! not a gate for a send.
//!
//! What a caller relies on:
//!
//! - An intent's items are its batch's names, and this module derives each
//!   item's `canonical_key` and `absence_key` with two of the master name
//!   index's key functions; a caller building an intent cannot supply them. Whether two
//!   names may be one master, inside a batch and against an open dispatch, is
//!   computed from the names every time ([`NameKeys::may_be_one_master`]), so a
//!   journal written before a fold widened is judged by the wider fold. Against
//!   an open dispatch an equal recorded key also refuses: a key on disk can only
//!   add a refusal. That keeps a fold narrowed by a later build from releasing an
//!   overlap between two lines written before it; a new intent's keys come from
//!   the narrower fold, so it does not cover a new batch.
//! - Widening a fold can make an existing journal unreadable: a batch or a pair
//!   of open intents admitted before may now match, and the whole file is then
//!   refused, which stops every master write until the journal is repaired. That
//!   failure is closed and loud, and a fold change has to weigh it.
//! - An intent with no terminal verdict blocks every overlapping batch. The
//!   terminal verdicts are `posted_verified`, `posted_into_existing_master`,
//!   `master_write_located_elsewhere`, `posted_unattributed` and
//!   `confirmed_absent`; the last two are decided only by a reconcile
//!   (`agent_master_verdict.rs`). `confirmed_absent` is given when its paired
//!   read-back shows the ledger absent, and a new batch for the same ledger then
//!   needs a new approval: nothing is ever sent again automatically.
//!   `posted_unattributed` is given when no answer from Tally is on file and the
//!   ledger was found as sent, written by the first master change after the
//!   mark the intent recorded.
//! - A build that predates a verdict state refuses the whole journal on a line
//!   holding it, which stops every master write and reconcile with no repair in
//!   the product. So a new state ships in the same release as the first build
//!   that writes it, and `posted_unattributed` in the same release as the first
//!   build that writes this file: no build without it has a journal to read.
//! - After a terminal verdict the same ledgers could be sent again under a new
//!   batch id. What stops that is the dispatch slice's absence check against the
//!   books as they are then, made in the same queue as the send. This journal does
//!   not stop it and must not be read as if it did.
//!
//! - A line cut short is refused on read, but a file cut back to a record
//!   boundary reads as a shorter, valid journal: whole trailing records lost to a
//!   crash or a restore are not detectable here.
//!
//! Reuse: bounded record reading ([`ledger::read_record`]), the append that
//! syncs and truncates a failed write back ([`append_private_import_ledger`]),
//! the owner-only file mode ([`set_private_file`]), the batch-id validator
//! ([`valid_batch_id`]) and the staged, synced, exclusive-rename record write
//! ([`persistence::write_record_once`]) all come from the voucher import path.
use super::ledger::{self, MAX_RECORD_BYTES};
use super::persistence::{self, RecordOnce};
use super::{append_private_import_ledger, set_private_file, valid_batch_id};
use crate::agent::{local_file, sha256_hex};
use bridge_tally_core::master_binding::{
    absence_key, canonical_master_key, letters_and_digits_key, twin_fold_keys,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};
use uuid::Uuid;

const JOURNAL_FILE: &str = "agent-master-ledger.jsonl";
const PRE_IMAGE_PREFIX: &str = "master-pre-image-";

/// Proof that the import admission lock is held exclusively. Built only by
/// [`ExclusiveAdmission::acquire`]; it holds the locked file for as long as it
/// lives. It duplicates the few lines of `Server::lock_import_admission` rather
/// than wrapping a `File`, because a `File` may hold a shared lock and carries
/// nothing that says which.
#[derive(Debug)]
pub(super) struct ExclusiveAdmission {
    /// Held for its lock; closing it releases the lock.
    _held: std::fs::File,
}

impl ExclusiveAdmission {
    /// Take the admission lock of `data_dir` exclusively, without waiting:
    /// contention is a refusal, as for a voucher post.
    pub(super) fn acquire(data_dir: &Path) -> Result<Self, String> {
        let path = data_dir.join("agent-import-admission.lock");
        let file = local_file::open_local_file(&path, true)
            .map_err(|_| "import_admission_lock_unavailable".to_string())?;
        file.try_lock()
            .map_err(super::import_admission_lock_error)?;
        persistence::require_settled(&data_dir.join("imports"))?;
        Ok(Self { _held: file })
    }

    /// For tests that exercise a write on a directory of their own.
    #[cfg(test)]
    fn standing_in(file: std::fs::File) -> Self {
        Self { _held: file }
    }
}

/// Whether `name` is a file this module writes: the journal, a sidecar named for
/// a well-formed batch id, or the stage a sidecar write leaves behind if it stops
/// (`<batch id>.<uuid>.next`). The local-data report classes these with the
/// import journal.
pub(super) fn is_master_file(name: &str) -> bool {
    if name == JOURNAL_FILE {
        return true;
    }
    let Some(rest) = name.strip_prefix(PRE_IMAGE_PREFIX) else {
        return false;
    };
    if let Some(batch_id) = rest.strip_suffix(".json") {
        return valid_batch_id(batch_id);
    }
    rest.strip_suffix(".next")
        .and_then(|stem| stem.rsplit_once('.'))
        .is_some_and(|(batch_id, stage)| valid_batch_id(batch_id) && Uuid::parse_str(stage).is_ok())
}

/// What kind of master write a batch is. One class today; the field is in the
/// record so a later class is a new value, not a reinterpreted old record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum MasterClass {
    LedgerCreate,
}

/// The shape of a ledger Bridge creates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum StoredLedgerKind {
    PartySundryDebtors,
    PartySundryCreditors,
    Suspense,
    GstDuty,
}

impl StoredLedgerKind {
    fn is_party(self) -> bool {
        match self {
            Self::PartySundryDebtors | Self::PartySundryCreditors => true,
            Self::Suspense | Self::GstDuty => false,
        }
    }

    fn is_gst_duty(self) -> bool {
        match self {
            Self::GstDuty => true,
            Self::PartySundryDebtors | Self::PartySundryCreditors | Self::Suspense => false,
        }
    }
}

/// The company a batch writes into, as verified when the batch was built.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterCompany {
    pub(super) guid: String,
    pub(super) name: String,
    pub(super) number: String,
    pub(super) books_from: String,
}

/// One ledger a batch creates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StoredLedgerCreate {
    pub(super) name: String,
    pub(super) kind: StoredLedgerKind,
    pub(super) bill_wise: bool,
    pub(super) party_gstin: Option<String>,
    /// The duty head in Tally's own spelling.
    pub(super) gst_duty_head: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterBatch {
    pub(super) batch_id: String,
    pub(super) batch_sha256: String,
    pub(super) class: MasterClass,
    pub(super) company: MasterCompany,
    pub(super) items: Vec<StoredLedgerCreate>,
}

/// What the intent records of one item, so a later read can look for it again.
/// Built only from a batch name ([`IntentItem::of`]). A key read back from disk
/// is not checked against the name; it is used only to add an overlap refusal
/// (see the module notes), so no accessor hands it out.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct IntentItem {
    name: String,
    canonical_key: String,
    absence_key: String,
}

impl IntentItem {
    fn of(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            canonical_key: canonical_master_key(name),
            absence_key: absence_key(name),
        }
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }
}

/// A company's master mark (`ALTMSTID`) in the last read before the post.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterMark {
    pub(super) company_guid: String,
    pub(super) altmstid: u64,
}

/// Written before the POST. Its presence is the refusal to send again.
///
/// Serialize only. The one constructor is [`MasterDispatchIntent::new`], which
/// takes the [`WrittenPreImage`] token; the module's own reader parses a line
/// through a private wire type ([`parse_record`]) and admits the result, so no
/// code outside this module can build an intent from JSON. A test pins that this
/// type and [`MasterRecord`] do not implement `Deserialize`.
///
/// Every field is private, so a value cannot be re-pointed at another batch
/// after it was bound to its sidecar. `Clone` stays because [`MasterRecord`] is
/// cloned by callers; a clone is inert, since the journal admits one intent per
/// batch.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(super) struct MasterDispatchIntent {
    batch_id: String,
    batch_sha256: String,
    native_request_sha256: String,
    items: Vec<IntentItem>,
    /// The digest and count of the pre-image sidecar written before this
    /// intent. Private: an intent takes them only from a [`WrittenPreImage`].
    pre_image_sha256: String,
    pre_image_count: u64,
    /// Every loaded company's mark, the batch's own company included, so a
    /// write that landed in another company can be told apart.
    pre_post_master_marks: Vec<MasterMark>,
}

impl MasterDispatchIntent {
    /// The intent of `batch`, bound to the sidecar `pre_image` was written for.
    /// Its items are the batch's names, in order.
    pub(super) fn new(
        batch: &MasterBatch,
        native_request_sha256: String,
        pre_image: WrittenPreImage,
        pre_post_master_marks: Vec<MasterMark>,
    ) -> Result<Self, PreImageError> {
        if pre_image.batch_id != batch.batch_id {
            return Err(PreImageError::OtherBatch);
        }
        Ok(Self {
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.batch_sha256.clone(),
            native_request_sha256,
            items: batch
                .items
                .iter()
                .map(|item| IntentItem::of(&item.name))
                .collect(),
            pre_image_sha256: pre_image.sha256,
            pre_image_count: pre_image.count,
            pre_post_master_marks,
        })
    }

    pub(super) fn batch_id(&self) -> &str {
        &self.batch_id
    }

    pub(super) fn batch_sha256(&self) -> &str {
        &self.batch_sha256
    }

    pub(super) fn native_request_sha256(&self) -> &str {
        &self.native_request_sha256
    }

    pub(super) fn items(&self) -> &[IntentItem] {
        &self.items
    }

    pub(super) fn pre_post_master_marks(&self) -> &[MasterMark] {
        &self.pre_post_master_marks
    }

    pub(super) fn pre_image_sha256(&self) -> &str {
        &self.pre_image_sha256
    }

    pub(super) fn pre_image_count(&self) -> u64 {
        self.pre_image_count
    }
}

/// Tally's import counters, as the response reported them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterCounters {
    pub(super) created: u64,
    pub(super) altered: u64,
    pub(super) deleted: u64,
    pub(super) ignored: u64,
    pub(super) errors: u64,
    pub(super) cancelled: u64,
    pub(super) exceptions: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterDispatchResponse {
    pub(super) batch_id: String,
    pub(super) batch_sha256: String,
    pub(super) response_sha256: String,
    /// `None` when the response carried no counters Bridge could read.
    pub(super) counters: Option<MasterCounters>,
}

/// What a read-back concluded about a batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum MasterVerdictState {
    PostedVerified,
    ReconciliationRequired,
    PostedIntoExistingMaster,
    MastersAfterPostUnconfirmed,
    MasterWriteLocatedElsewhere,
    /// After an outcome was unknown, a paired read-back showed the ledger
    /// absent, no pre-image master changed and no loaded company's master mark
    /// moved since the intent. Nothing was created; the batch is closed and is
    /// never sent again. A new batch needs a new approval.
    ConfirmedAbsent,
    /// A reconcile of a batch with no answer from Tally on file found the
    /// ledger as sent, every check but the counters passed, and the ledger's
    /// `ALTERID` is the target company's mark read before the send plus one:
    /// the first master change after that read wrote it, and none since.
    /// Settled, never verified: with Tally's answer lost, nothing shows that
    /// change was this create rather than a person keying the identical ledger.
    PostedUnattributed,
}

impl MasterVerdictState {
    /// Whether the outcome of the write is settled: the first terminal verdict
    /// of a batch is its last, and it stops blocking later batches of the same
    /// ledgers. A verdict that asks for a person's reconciliation, or that
    /// could not confirm the masters after the post, leaves the outcome open.
    pub(super) fn is_terminal(self) -> bool {
        match self {
            Self::PostedVerified
            | Self::PostedIntoExistingMaster
            | Self::MasterWriteLocatedElsewhere
            | Self::ConfirmedAbsent
            | Self::PostedUnattributed => true,
            Self::ReconciliationRequired | Self::MastersAfterPostUnconfirmed => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct VerdictItem {
    pub(super) name: String,
    pub(super) guid: Option<String>,
    pub(super) reason: Option<String>,
}

/// A verdict as journaled. Read from a line or built by [`MasterVerdict::judged`]
/// from a judgement: the private field keeps any other module from writing one
/// by hand, so a terminal verdict, the only record that releases a held name,
/// always rests on a read-back's judgement (P2).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterVerdict {
    batch_id: String,
    batch_sha256: String,
    state: MasterVerdictState,
    items: Vec<VerdictItem>,
    #[serde(skip)]
    sealed: (),
}

/// One journal line. Every `match` on it names every variant, so a new record
/// kind is a compile error until each decision site handles it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "record_type")]
pub(super) enum MasterRecord {
    #[serde(rename = "master_batch")]
    Batch(MasterBatch),
    #[serde(rename = "master_dispatch_intent")]
    DispatchIntent(MasterDispatchIntent),
    #[serde(rename = "master_dispatch_response")]
    DispatchResponse(MasterDispatchResponse),
    #[serde(rename = "master_verdict")]
    Verdict(MasterVerdict),
}

/// The shape of an intent on disk. Private, so deserialising one is possible
/// only here.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IntentWire {
    batch_id: String,
    batch_sha256: String,
    native_request_sha256: String,
    items: Vec<IntentItem>,
    pre_image_sha256: String,
    pre_image_count: u64,
    pre_post_master_marks: Vec<MasterMark>,
}

/// A journal line as it is read.
#[derive(Deserialize)]
#[serde(tag = "record_type")]
enum RecordWire {
    #[serde(rename = "master_batch")]
    Batch(MasterBatch),
    #[serde(rename = "master_dispatch_intent")]
    DispatchIntent(IntentWire),
    #[serde(rename = "master_dispatch_response")]
    DispatchResponse(MasterDispatchResponse),
    #[serde(rename = "master_verdict")]
    Verdict(MasterVerdict),
}

/// Parse one journal line. The result is not yet admitted.
fn parse_record(text: &str) -> Result<MasterRecord, serde_json::Error> {
    Ok(match serde_json::from_str::<RecordWire>(text)? {
        RecordWire::Batch(record) => MasterRecord::Batch(record),
        RecordWire::DispatchIntent(wire) => MasterRecord::DispatchIntent(MasterDispatchIntent {
            batch_id: wire.batch_id,
            batch_sha256: wire.batch_sha256,
            native_request_sha256: wire.native_request_sha256,
            items: wire.items,
            pre_image_sha256: wire.pre_image_sha256,
            pre_image_count: wire.pre_image_count,
            pre_post_master_marks: wire.pre_post_master_marks,
        }),
        RecordWire::DispatchResponse(record) => MasterRecord::DispatchResponse(record),
        RecordWire::Verdict(record) => MasterRecord::Verdict(record),
    })
}

impl MasterRecord {
    /// Whether this is a dispatch intent, the record whose durability the
    /// refusal to send twice rests on.
    fn is_dispatch_intent(&self) -> bool {
        match self {
            Self::DispatchIntent(_) => true,
            Self::Batch(_) | Self::DispatchResponse(_) | Self::Verdict(_) => false,
        }
    }

    /// The `record_type` this record is written under, for a test to name the
    /// append it fails.
    #[cfg(test)]
    fn record_type(&self) -> &'static str {
        match self {
            Self::Batch(_) => "master_batch",
            Self::DispatchIntent(_) => "master_dispatch_intent",
            Self::DispatchResponse(_) => "master_dispatch_response",
            Self::Verdict(_) => "master_verdict",
        }
    }

    pub(super) fn batch_id(&self) -> &str {
        match self {
            Self::Batch(record) => &record.batch_id,
            Self::DispatchIntent(record) => &record.batch_id,
            Self::DispatchResponse(record) => &record.batch_id,
            Self::Verdict(record) => &record.batch_id,
        }
    }

    pub(super) fn batch_sha256(&self) -> &str {
        match self {
            Self::Batch(record) => &record.batch_sha256,
            Self::DispatchIntent(record) => &record.batch_sha256,
            Self::DispatchResponse(record) => &record.batch_sha256,
            Self::Verdict(record) => &record.batch_sha256,
        }
    }
}

/// Why a journal or a record was refused as invalid. All of them surface as
/// `master_ledger_invalid`; the cause is for tests and for a person reading a
/// log, so a test of one refusal cannot be satisfied by another.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Invalid {
    /// Not one JSON object of a known record type, or a field it does not allow.
    Parse,
    /// A record with no line delimiter after it.
    Unterminated,
    BadBatchId,
    BadDigest,
    BlankField,
    EmptyItems,
    DuplicateName,
    /// A field that belongs to another kind of ledger, or one missing.
    KindFieldMismatch,
    BadGuid,
    DuplicateMark,
    /// A record names a batch no earlier record introduced.
    NoBatch,
    /// A record carries a different batch digest than its batch.
    ShaMismatch,
    /// A batch id already held by a different batch.
    BatchIdReused,
    ResponseWithoutIntent,
    VerdictWithoutIntent,
    /// A verdict after the batch's terminal verdict.
    VerdictAfterTerminal,
    /// A `posted_verified` verdict with no answer from Tally on record: a
    /// verified create needs Tally's counters, so its answer is appended first.
    VerifiedWithoutResponse,
    /// A `confirmed_absent` verdict against an answer on record whose counters
    /// say Tally created or altered something: absence is never concluded
    /// against Tally's own count of a write.
    AbsentAgainstCounters,
    /// A `posted_unattributed` verdict for a batch with an answer on record,
    /// with or without counters: that state exists only for an answer that was
    /// lost, and Tally's own answer decides one that is on file.
    UnattributedWithAnswer,
    /// A second response for a batch: one POST, one response.
    SecondResponse,
    /// An intent recording no company mark, or none for the batch's company.
    MarksMissing,
    /// A pre-image count of zero.
    ZeroPreImageCount,
    /// An intent or verdict whose items are not the batch's, in order.
    ItemsMismatch,
    /// `posted_verified` or `posted_unattributed` for an item with no GUID.
    VerifiedWithoutGuid,
    /// `confirmed_absent` for an item that names a GUID.
    AbsentWithGuid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MasterLedgerError {
    Unavailable,
    RecordTooLarge,
    SerializationFailed,
    /// An append failed and the journal could not be cut back to its length.
    RollbackFailed,
    /// A second dispatch intent for a batch that already has one.
    DuplicateDispatch,
    /// An intent given to [`append`]; it goes through [`append_intent`].
    UseAppendIntent,
    /// An intent for ledgers an earlier dispatched batch also names, while that
    /// batch's outcome is open: sending them again under a new batch id.
    OpenDispatchOverlaps,
    /// The journal file was created and its directory entry could not be made
    /// durable. The record may be in the file; treat the dispatch as recorded.
    DirectorySyncFailed,
    Invalid(Invalid),
}

impl MasterLedgerError {
    pub(super) const fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "master_ledger_unavailable",
            Self::RecordTooLarge => "master_ledger_record_too_large",
            Self::SerializationFailed => "master_ledger_serialization_failed",
            Self::RollbackFailed => "master_ledger_rollback_failed",
            Self::DuplicateDispatch => "master_ledger_duplicate_dispatch",
            Self::UseAppendIntent => "master_ledger_use_append_intent",
            Self::OpenDispatchOverlaps => "master_ledger_open_dispatch_overlaps",
            Self::DirectorySyncFailed => "master_ledger_directory_sync_failed",
            Self::Invalid(_) => "master_ledger_invalid",
        }
    }
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// A GUID as the journal and the sidecar record it: lower-case hex and
/// hyphens, as Tally exports one.
pub(super) fn is_canonical_guid(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte) || byte == b'-')
}

fn non_blank(value: &str) -> Result<(), Invalid> {
    if value.trim().is_empty() {
        Err(Invalid::BlankField)
    } else {
        Ok(())
    }
}

fn check_ids(batch_id: &str, batch_sha256: &str) -> Result<(), Invalid> {
    if !valid_batch_id(batch_id) {
        return Err(Invalid::BadBatchId);
    }
    if !is_sha256_hex(batch_sha256) {
        return Err(Invalid::BadDigest);
    }
    Ok(())
}

impl StoredLedgerCreate {
    fn validate(&self) -> Result<(), Invalid> {
        non_blank(&self.name)?;
        if let Some(gstin) = &self.party_gstin {
            non_blank(gstin)?;
        }
        if let Some(head) = &self.gst_duty_head {
            non_blank(head)?;
        }
        // A GSTIN belongs to a party; a duty head to a duty ledger, which
        // always has one.
        if self.party_gstin.is_some() && !self.kind.is_party() {
            return Err(Invalid::KindFieldMismatch);
        }
        if self.gst_duty_head.is_some() != self.kind.is_gst_duty() {
            return Err(Invalid::KindFieldMismatch);
        }
        Ok(())
    }
}

impl MasterBatch {
    fn validate(&self) -> Result<(), Invalid> {
        check_ids(&self.batch_id, &self.batch_sha256)?;
        let MasterCompany {
            guid,
            name,
            number,
            books_from,
        } = &self.company;
        for field in [name, number, books_from] {
            non_blank(field)?;
        }
        if !is_canonical_guid(guid) {
            return Err(Invalid::BadGuid);
        }
        if self.items.is_empty() {
            return Err(Invalid::EmptyItems);
        }
        // Two names Tally might take for one master cannot go in one POST.
        let mut seen: Vec<NameKeys> = Vec::with_capacity(self.items.len());
        for item in &self.items {
            item.validate()?;
            let keys = NameKeys::of(&item.name);
            if seen.iter().any(|earlier| earlier.may_be_one_master(&keys)) {
                return Err(Invalid::DuplicateName);
            }
            seen.push(keys);
        }
        match self.class {
            MasterClass::LedgerCreate => Ok(()),
        }
    }

    fn names(&self) -> Vec<&str> {
        self.items.iter().map(|item| item.name.as_str()).collect()
    }
}

impl MasterDispatchIntent {
    fn validate(&self) -> Result<(), Invalid> {
        check_ids(&self.batch_id, &self.batch_sha256)?;
        if !is_sha256_hex(&self.native_request_sha256) || !is_sha256_hex(&self.pre_image_sha256) {
            return Err(Invalid::BadDigest);
        }
        if self.items.is_empty() {
            return Err(Invalid::EmptyItems);
        }
        for item in &self.items {
            non_blank(&item.name)?;
            non_blank(&item.canonical_key)?;
            non_blank(&item.absence_key)?;
        }
        if self.pre_image_count == 0 {
            return Err(Invalid::ZeroPreImageCount);
        }
        if self.pre_post_master_marks.is_empty() {
            return Err(Invalid::MarksMissing);
        }
        let mut companies = BTreeSet::new();
        for mark in &self.pre_post_master_marks {
            if !is_canonical_guid(&mark.company_guid) {
                return Err(Invalid::BadGuid);
            }
            if !companies.insert(mark.company_guid.as_str()) {
                return Err(Invalid::DuplicateMark);
            }
        }
        Ok(())
    }
}

impl MasterDispatchResponse {
    fn validate(&self) -> Result<(), Invalid> {
        check_ids(&self.batch_id, &self.batch_sha256)?;
        if !is_sha256_hex(&self.response_sha256) {
            return Err(Invalid::BadDigest);
        }
        Ok(())
    }
}

impl MasterVerdict {
    pub(super) fn state(&self) -> MasterVerdictState {
        self.state
    }

    pub(super) fn items(&self) -> &[VerdictItem] {
        &self.items
    }

    /// The verdict `judgement` gives on `batch`: each item carries the
    /// judgement's GUID, and its reasons unless the batch is verified.
    pub(super) fn judged(
        batch: &MasterBatch,
        judgement: &super::master_tools::verdict::Judgement,
    ) -> Self {
        let reason = (judgement.state() != MasterVerdictState::PostedVerified)
            .then(|| judgement.reasons().join(","));
        Self {
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.batch_sha256.clone(),
            state: judgement.state(),
            items: batch
                .items
                .iter()
                .map(|item| VerdictItem {
                    name: item.name.clone(),
                    guid: judgement.guid().map(str::to_string),
                    reason: reason.clone(),
                })
                .collect(),
            sealed: (),
        }
    }

    /// Any verdict, for a test of the journal's own admission.
    #[cfg(test)]
    pub(super) fn for_test(
        batch_id: String,
        batch_sha256: String,
        state: MasterVerdictState,
        items: Vec<VerdictItem>,
    ) -> Self {
        Self {
            batch_id,
            batch_sha256,
            state,
            items,
            sealed: (),
        }
    }

    fn validate(&self) -> Result<(), Invalid> {
        check_ids(&self.batch_id, &self.batch_sha256)?;
        if self.items.is_empty() {
            return Err(Invalid::EmptyItems);
        }
        let mut guids = BTreeSet::new();
        for item in &self.items {
            non_blank(&item.name)?;
            if let Some(reason) = &item.reason {
                non_blank(reason)?;
            }
            if let Some(guid) = &item.guid {
                if !is_canonical_guid(guid) || !guids.insert(guid.as_str()) {
                    return Err(Invalid::BadGuid);
                }
            }
        }
        // The states that say the ledger is in the book as sent name every
        // item's GUID.
        let all_found = match self.state {
            MasterVerdictState::PostedVerified | MasterVerdictState::PostedUnattributed => true,
            MasterVerdictState::ReconciliationRequired
            | MasterVerdictState::PostedIntoExistingMaster
            | MasterVerdictState::MastersAfterPostUnconfirmed
            | MasterVerdictState::MasterWriteLocatedElsewhere
            | MasterVerdictState::ConfirmedAbsent => false,
        };
        if all_found && self.items.iter().any(|item| item.guid.is_none()) {
            return Err(Invalid::VerifiedWithoutGuid);
        }
        // A ledger confirmed absent has no GUID to name.
        if self.state == MasterVerdictState::ConfirmedAbsent
            && self.items.iter().any(|item| item.guid.is_some())
        {
            return Err(Invalid::AbsentWithGuid);
        }
        Ok(())
    }
}

impl MasterRecord {
    fn validate(&self) -> Result<(), Invalid> {
        match self {
            Self::Batch(record) => record.validate(),
            Self::DispatchIntent(record) => record.validate(),
            Self::DispatchResponse(record) => record.validate(),
            Self::Verdict(record) => record.validate(),
        }
    }
}

/// Counts for the local-data report: batches, those dispatched, and of those
/// the ones with no terminal verdict, which block overlapping batches.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct MasterSummary {
    pub(super) batches: usize,
    pub(super) dispatched: usize,
    pub(super) not_settled: usize,
}

/// Everything the journal holds about one batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BatchState {
    pub(super) batch: MasterBatch,
    pub(super) intent: Option<MasterDispatchIntent>,
    pub(super) response: Option<MasterDispatchResponse>,
    /// The latest verdict; once it is terminal, the last.
    pub(super) verdict: Option<MasterVerdict>,
}

impl BatchState {
    /// Whether a terminal verdict has settled the batch's outcome.
    pub(super) fn is_settled(&self) -> bool {
        self.verdict
            .as_ref()
            .is_some_and(|verdict| verdict.state.is_terminal())
    }
}

/// A dispatched batch whose outcome is not settled and whose ledger names block
/// a new dispatch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BlockingBatch {
    pub(super) batch_id: String,
    /// The company the blocking batch was prepared for.
    pub(super) company_guid: String,
}

/// The admitted journal: per-batch state, built record by record.
#[derive(Debug, Default)]
pub(super) struct MasterJournal {
    batches: BTreeMap<String, BatchState>,
}

impl MasterJournal {
    pub(super) fn state(&self, batch_id: &str) -> Option<&BatchState> {
        self.batches.get(batch_id)
    }

    /// Whether a dispatch intent is recorded for the batch: the batch was, or
    /// may have been, sent. A read without the lock, for reporting. It is not a
    /// gate for a send: that is [`append_intent`] under the exclusive lock.
    pub(super) fn dispatched(&self, batch_id: &str) -> bool {
        self.state(batch_id)
            .is_some_and(|state| state.intent.is_some())
    }

    pub(super) fn verdict(&self, batch_id: &str) -> Option<&MasterVerdict> {
        self.state(batch_id)
            .and_then(|state| state.verdict.as_ref())
    }

    /// What the local-data report shows of this journal.
    pub(super) fn summary(&self) -> MasterSummary {
        let dispatched = self
            .batches
            .values()
            .filter(|state| state.intent.is_some())
            .count();
        let not_settled = self
            .batches
            .values()
            .filter(|state| state.intent.is_some() && !state.is_settled())
            .count();
        MasterSummary {
            batches: self.batches.len(),
            dispatched,
            not_settled,
        }
    }

    /// The state of the batch a record belongs to, if that batch is introduced
    /// and the record carries its digest.
    fn batch_of(&self, record: &MasterRecord) -> Result<&BatchState, Invalid> {
        let state = self
            .batches
            .get(record.batch_id())
            .ok_or(Invalid::NoBatch)?;
        if state.batch.batch_sha256 != record.batch_sha256() {
            return Err(Invalid::ShaMismatch);
        }
        Ok(state)
    }

    /// The other batches that were dispatched, have no terminal verdict, and
    /// name a ledger that may be one master with a ledger `items` names, or that
    /// recorded an equal key. Every company's batches are compared: a batch is
    /// listed whatever company it belongs to.
    fn overlaps_open_dispatch(&self, batch_id: &str, items: &[IntentItem]) -> Vec<BlockingBatch> {
        let new_items: Vec<(&IntentItem, NameKeys)> = items
            .iter()
            .map(|item| (item, NameKeys::of(&item.name)))
            .collect();
        self.batches
            .values()
            .filter(|state| !state.is_settled())
            .filter(|state| {
                state.intent.as_ref().is_some_and(|open| {
                    open.batch_id != batch_id
                        && open.items.iter().any(|held| {
                            let held_keys = NameKeys::of(&held.name);
                            new_items.iter().any(|(new, new_keys)| {
                                new.canonical_key == held.canonical_key
                                    || new.absence_key == held.absence_key
                                    || new_keys.may_be_one_master(&held_keys)
                            })
                        })
                })
            })
            .map(|state| BlockingBatch {
                batch_id: state.batch.batch_id.clone(),
                company_guid: state.batch.company.guid.clone(),
            })
            .collect()
    }

    /// The batches that would block a dispatch of `batch` now, by the rule the
    /// journal applies when the intent is appended. The post refuses on it
    /// before the dialog and names the batches from it; the refusal that
    /// decides is the one under the exclusive lock.
    pub(super) fn open_dispatch_blockers(&self, batch: &MasterBatch) -> Vec<BlockingBatch> {
        let items = batch
            .items
            .iter()
            .map(|item| IntentItem::of(&item.name))
            .collect::<Vec<_>>();
        self.overlaps_open_dispatch(&batch.batch_id, &items)
    }

    /// Every refusal the journal would raise for a dispatch intent of the batch
    /// with `batch_sha256`, naming `items` and recording `marks`, except the
    /// ones about the pre-image sidecar. It is the one place the intent's
    /// admission is decided: [`Self::admit`] calls it, and a caller about to
    /// write a sidecar calls it first through [`Self::check_intent_admissible`],
    /// so no sidecar is written for an intent the journal would refuse.
    fn intent_refusal(
        &self,
        batch_id: &str,
        batch_sha256: &str,
        items: &[IntentItem],
        marks: &[MasterMark],
    ) -> Result<(), MasterLedgerError> {
        let state = self
            .batches
            .get(batch_id)
            .ok_or(MasterLedgerError::Invalid(Invalid::NoBatch))?;
        if state.batch.batch_sha256 != batch_sha256 {
            return Err(MasterLedgerError::Invalid(Invalid::ShaMismatch));
        }
        if state.intent.is_some() {
            return Err(MasterLedgerError::DuplicateDispatch);
        }
        let named = items
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();
        if named != state.batch.names() {
            return Err(MasterLedgerError::Invalid(Invalid::ItemsMismatch));
        }
        if !marks
            .iter()
            .any(|mark| mark.company_guid == state.batch.company.guid)
        {
            return Err(MasterLedgerError::Invalid(Invalid::MarksMissing));
        }
        if !self.overlaps_open_dispatch(batch_id, items).is_empty() {
            return Err(MasterLedgerError::OpenDispatchOverlaps);
        }
        Ok(())
    }

    /// Whether a dispatch intent for this batch, recording `marks`, would be
    /// admitted as far as the journal's own rules go. Its items are the batch's
    /// names, as [`MasterDispatchIntent::new`] builds them. For a caller holding
    /// the exclusive lock that is about to write the sidecar the intent must
    /// name; the append checks again.
    pub(super) fn check_intent_admissible(
        &self,
        batch: &MasterBatch,
        marks: &[MasterMark],
    ) -> Result<(), MasterLedgerError> {
        let items = batch
            .items
            .iter()
            .map(|item| IntentItem::of(&item.name))
            .collect::<Vec<_>>();
        self.intent_refusal(&batch.batch_id, &batch.batch_sha256, &items, marks)
    }

    /// Admit one record, or refuse it and leave the journal as it was.
    pub(super) fn admit(&mut self, record: MasterRecord) -> Result<(), MasterLedgerError> {
        record.validate().map_err(MasterLedgerError::Invalid)?;
        match &record {
            MasterRecord::Batch(batch) => {
                // A batch id is introduced once. The same batch again is
                // refused too: the caller reads the state instead of writing.
                if self.batches.contains_key(&batch.batch_id) {
                    return Err(MasterLedgerError::Invalid(Invalid::BatchIdReused));
                }
                self.batches.insert(
                    batch.batch_id.clone(),
                    BatchState {
                        batch: batch.clone(),
                        intent: None,
                        response: None,
                        verdict: None,
                    },
                );
                Ok(())
            }
            MasterRecord::DispatchIntent(intent) => {
                self.intent_refusal(
                    &intent.batch_id,
                    &intent.batch_sha256,
                    &intent.items,
                    &intent.pre_post_master_marks,
                )?;
                if let Some(state) = self.batches.get_mut(&intent.batch_id) {
                    state.intent = Some(intent.clone());
                }
                Ok(())
            }
            MasterRecord::DispatchResponse(response) => {
                let state = self.batch_of(&record).map_err(MasterLedgerError::Invalid)?;
                if state.intent.is_none() {
                    return Err(MasterLedgerError::Invalid(Invalid::ResponseWithoutIntent));
                }
                if state.response.is_some() {
                    return Err(MasterLedgerError::Invalid(Invalid::SecondResponse));
                }
                if let Some(state) = self.batches.get_mut(&response.batch_id) {
                    state.response = Some(response.clone());
                }
                Ok(())
            }
            MasterRecord::Verdict(verdict) => {
                let state = self.batch_of(&record).map_err(MasterLedgerError::Invalid)?;
                if state.intent.is_none() {
                    return Err(MasterLedgerError::Invalid(Invalid::VerdictWithoutIntent));
                }
                if state.is_settled() {
                    return Err(MasterLedgerError::Invalid(Invalid::VerdictAfterTerminal));
                }
                if verdict.state == MasterVerdictState::PostedVerified && state.response.is_none() {
                    return Err(MasterLedgerError::Invalid(Invalid::VerifiedWithoutResponse));
                }
                let counted_a_write = state
                    .response
                    .as_ref()
                    .and_then(|response| response.counters)
                    .is_some_and(|counters| counters.created > 0 || counters.altered > 0);
                if verdict.state == MasterVerdictState::ConfirmedAbsent && counted_a_write {
                    return Err(MasterLedgerError::Invalid(Invalid::AbsentAgainstCounters));
                }
                if verdict.state == MasterVerdictState::PostedUnattributed
                    && state.response.is_some()
                {
                    return Err(MasterLedgerError::Invalid(Invalid::UnattributedWithAnswer));
                }
                let named = verdict
                    .items
                    .iter()
                    .map(|item| item.name.as_str())
                    .collect::<Vec<_>>();
                if named != state.batch.names() {
                    return Err(MasterLedgerError::Invalid(Invalid::ItemsMismatch));
                }
                if let Some(state) = self.batches.get_mut(&verdict.batch_id) {
                    state.verdict = Some(verdict.clone());
                }
                Ok(())
            }
        }
    }

    /// Admit a whole journal. Every line must be one admitted record.
    pub(super) fn read(mut reader: impl BufRead) -> Result<Self, MasterLedgerError> {
        let mut journal = Self::default();
        let mut line = Vec::new();
        while ledger::read_record(&mut reader, &mut line).map_err(read_error)? {
            let text = std::str::from_utf8(&line)
                .map_err(|_| MasterLedgerError::Invalid(Invalid::Parse))?;
            let record =
                parse_record(text).map_err(|_| MasterLedgerError::Invalid(Invalid::Parse))?;
            journal.admit(record)?;
        }
        Ok(journal)
    }
}

fn read_error(code: String) -> MasterLedgerError {
    match code.as_str() {
        "import_ledger_invalid" => MasterLedgerError::Invalid(Invalid::Unterminated),
        "import_ledger_record_too_large" => MasterLedgerError::RecordTooLarge,
        _ => MasterLedgerError::Unavailable,
    }
}

/// The keys of one name for deciding whether Tally might take two names for one
/// master, computed once per name.
struct NameKeys {
    canonical: String,
    twin: [String; 3],
    letters_and_digits: String,
}

impl NameKeys {
    fn of(name: &str) -> Self {
        Self {
            canonical: canonical_master_key(name),
            twin: twin_fold_keys(name),
            letters_and_digits: letters_and_digits_key(name),
        }
    }

    /// Two names may be one master when they agree on either key the master name
    /// index matches on, or on any `twin_fold_keys` fold in any position. The
    /// index's `absence_key` is twin key 2, so the twin comparison covers it;
    /// the canonical key is compared on its own, because lowercasing and NFC do
    /// not commute and a name can agree on it and on no twin key. The index's
    /// letters-and-digits key is compared too, so two names in one batch that
    /// differ only by a space or a symbol are refused as the book check would.
    fn may_be_one_master(&self, other: &Self) -> bool {
        self.canonical == other.canonical
            || self.letters_and_digits == other.letters_and_digits
            || self.twin.iter().any(|key| other.twin.contains(key))
    }
}

fn journal_path(data_dir: &Path) -> PathBuf {
    data_dir.join(JOURNAL_FILE)
}

/// The admitted journal of `data_dir`; a journal not yet written is empty.
pub(super) fn read_journal(data_dir: &Path) -> Result<MasterJournal, MasterLedgerError> {
    match local_file::open_local_file(&journal_path(data_dir), false) {
        Ok(file) => MasterJournal::read(std::io::BufReader::new(file)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(MasterJournal::default()),
        Err(_) => Err(MasterLedgerError::Unavailable),
    }
}

/// Make the entries of `directory` durable.
#[cfg(unix)]
fn sync_directory(directory: &Path) -> std::io::Result<()> {
    std::fs::File::open(directory)?.sync_all()
}

/// Windows has no directory handle to sync; the file's own sync is what holds.
#[cfg(not(unix))]
fn sync_directory(_directory: &Path) -> std::io::Result<()> {
    Ok(())
}

/// Counts of the journal in `reader`, for the local-data report.
pub(super) fn read_summary(reader: impl BufRead) -> Result<MasterSummary, MasterLedgerError> {
    MasterJournal::read(reader).map(|journal| journal.summary())
}

/// Append one record, after admitting it against the journal as it stands: a
/// record the reader would refuse (a second intent included) is never written.
/// The caller holds the admission lock. The data directory is synced after the
/// write, and a failure to do so is an error, not ignored, when the journal was
/// empty or absent before the write and for every dispatch intent: a journal file
/// a power failure could make vanish is no record that a dispatch was recorded.
/// The state is read afresh on each append, so an earlier append that created the
/// file and then failed does not excuse a later one.
/// An intent does not go through here: see [`append_intent`].
pub(super) fn append(
    data_dir: &Path,
    lock: &ExclusiveAdmission,
    record: MasterRecord,
) -> Result<(), MasterLedgerError> {
    if record.is_dispatch_intent() {
        return Err(MasterLedgerError::UseAppendIntent);
    }
    #[cfg(test)]
    if test_fault_fires(record.record_type()) {
        return Err(MasterLedgerError::Unavailable);
    }
    #[cfg(test)]
    let lost_after_write = test_fault_fires(&format!("{}_lost_after_write", record.record_type()));
    let appended = append_syncing(data_dir, lock, record, sync_directory);
    #[cfg(test)]
    if lost_after_write && appended.is_ok() {
        return Err(MasterLedgerError::Unavailable);
    }
    appended
}

#[cfg(test)]
tokio::task_local! {
    /// The faults a test arms around the call under test: each entry names one
    /// point, a `record_type` whose next [`append`] (for an intent,
    /// [`append_intent`]) fails (or, with
    /// `_lost_after_write`, is written and then reported as failed) or a read
    /// the answered call makes, and is used up by the first time that point is
    /// reached.
    /// Absent, nothing fails.
    pub(super) static TEST_FAULTS: std::cell::RefCell<Vec<&'static str>>;
    /// Run by the post just before it reads the journal again for the check
    /// that precedes the dialog, as a holder settling or a rival's send
    /// landing in the seconds of reads before it would.
    pub(super) static BEFORE_HOLD_CHECK: std::sync::Arc<dyn Fn() + Send + Sync>;
    /// Run by a post refused under the journal's lock just before it reads the
    /// journal to say why, as a holder settling or another call's send landing
    /// in that moment would.
    pub(super) static BEFORE_FAILURE_READ: std::sync::Arc<dyn Fn() + Send + Sync>;
}

/// Whether `point` is armed in the task running this, using the arming up.
#[cfg(test)]
pub(super) fn test_fault_fires(point: &str) -> bool {
    TEST_FAULTS
        .try_with(|armed| {
            let mut armed = armed.borrow_mut();
            armed
                .iter()
                .position(|armed| *armed == point)
                .map(|at| armed.remove(at))
                .is_some()
        })
        .unwrap_or(false)
}

pub(super) use crate::tally::approved_import::RecordedIntent;

/// Append a dispatch intent and return the token for its send. An error, a
/// failed directory sync included, returns no token: the intent may be in the
/// file, and the batch must be treated as dispatched.
pub(super) fn append_intent(
    data_dir: &Path,
    lock: &ExclusiveAdmission,
    intent: MasterDispatchIntent,
) -> Result<RecordedIntent, MasterLedgerError> {
    // The faults a test arms for [`append`] reach the intent the same way: one
    // refused before any byte is written, and one written and then reported as
    // a failed directory sync, which leaves the intent in the file.
    #[cfg(test)]
    if test_fault_fires("master_dispatch_intent") {
        return Err(MasterLedgerError::Invalid(Invalid::BadDigest));
    }
    #[cfg(test)]
    let intent_lost_after_write = test_fault_fires("master_dispatch_intent_lost_after_write");
    let recorded = append_intent_syncing(data_dir, lock, intent, sync_directory);
    #[cfg(test)]
    if intent_lost_after_write && recorded.is_ok() {
        return Err(MasterLedgerError::DirectorySyncFailed);
    }
    recorded
}

fn append_intent_syncing(
    data_dir: &Path,
    lock: &ExclusiveAdmission,
    intent: MasterDispatchIntent,
    sync: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<RecordedIntent, MasterLedgerError> {
    let (batch_id, native_request_sha256) = (
        intent.batch_id.clone(),
        intent.native_request_sha256.clone(),
    );
    append_syncing(data_dir, lock, MasterRecord::DispatchIntent(intent), sync)?;
    Ok(RecordedIntent::issue(batch_id, native_request_sha256))
}

/// [`append`] for any record, with the directory sync given so a test can fail
/// it.
fn append_syncing(
    data_dir: &Path,
    _lock: &ExclusiveAdmission,
    record: MasterRecord,
    sync: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<(), MasterLedgerError> {
    let path = journal_path(data_dir);
    // A journal file that is absent or empty has not yet been made durable by an
    // earlier append that got that far, and an intent must never rest on a file
    // a power failure could take back: sync for either, and for every intent.
    let empty_before = std::fs::symlink_metadata(&path)
        .ok()
        .is_none_or(|meta| meta.len() == 0);
    let sync_after = empty_before || record.is_dispatch_intent();
    let mut journal = read_journal(data_dir)?;
    let encoded =
        serde_json::to_string(&record).map_err(|_| MasterLedgerError::SerializationFailed)?;
    let encoded = format!("{encoded}\n");
    if encoded.len() > MAX_RECORD_BYTES {
        return Err(MasterLedgerError::RecordTooLarge);
    }
    journal.admit(record)?;
    append_private_import_ledger(&path, encoded.as_bytes(), set_private_file).map_err(|code| {
        match code.as_str() {
            "import_ledger_rollback_failed" => MasterLedgerError::RollbackFailed,
            _ => MasterLedgerError::Unavailable,
        }
    })?;
    if sync_after {
        sync(data_dir).map_err(|_| MasterLedgerError::DirectorySyncFailed)?;
    }
    Ok(())
}

/// Why the pre-image sidecar could not be written or read back.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PreImageError {
    BadBatchId,
    /// An empty map: a book read that returned nothing is a failed read.
    Empty,
    /// A key that is not a lower-case GUID.
    BadGuid,
    /// One GUID given two different marks.
    ConflictingEntry,
    Missing,
    Unreadable,
    TooLarge,
    /// Not the sorted map this module writes.
    Malformed,
    ShaMismatch,
    CountMismatch,
    /// The sidecar's name already holds different bytes.
    ExistsDifferent,
    /// More bytes than the reader admits: it would refuse what was written.
    TooLargeToWrite,
    /// A token written for another batch.
    OtherBatch,
    /// The sidecar is in place but its directory entry could not be made durable.
    DirectorySyncFailed,
    WriteFailed,
    /// A sidecar written for a dispatch that was then refused could not be
    /// removed again.
    RemoveFailed,
}

impl PreImageError {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::BadBatchId => "master_pre_image_batch_id_invalid",
            Self::Empty
            | Self::BadGuid
            | Self::ConflictingEntry
            | Self::Malformed
            | Self::ShaMismatch
            | Self::CountMismatch => "master_pre_image_invalid",
            Self::Missing => "master_pre_image_missing",
            Self::Unreadable | Self::TooLarge => "master_pre_image_unavailable",
            Self::OtherBatch => "master_pre_image_invalid",
            Self::RemoveFailed => "master_pre_image_remove_failed",
            Self::ExistsDifferent
            | Self::TooLargeToWrite
            | Self::DirectorySyncFailed
            | Self::WriteFailed => "master_pre_image_write_failed",
        }
    }
}

/// The company's masters before the post: each GUID, lower-case, with its
/// ALTERID. Sorted, so equal books give equal bytes and an equal digest.
/// Serialize only: a sidecar is read as a plain map and rebuilt through
/// [`PreImage::new`], so a key the writer would refuse is refused on read too.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub(super) struct PreImage(BTreeMap<String, u64>);

impl PreImage {
    pub(super) fn new(
        entries: impl IntoIterator<Item = (String, u64)>,
    ) -> Result<Self, PreImageError> {
        let mut map = BTreeMap::new();
        for (guid, alter_id) in entries {
            let guid = guid.to_lowercase();
            if !is_canonical_guid(&guid) {
                return Err(PreImageError::BadGuid);
            }
            if map
                .insert(guid, alter_id)
                .is_some_and(|prior| prior != alter_id)
            {
                return Err(PreImageError::ConflictingEntry);
            }
        }
        if map.is_empty() {
            return Err(PreImageError::Empty);
        }
        Ok(Self(map))
    }

    pub(super) fn len(&self) -> usize {
        self.0.len()
    }

    pub(super) fn entries(&self) -> &BTreeMap<String, u64> {
        &self.0
    }

    fn bytes(&self) -> Result<Vec<u8>, PreImageError> {
        serde_json::to_vec(self).map_err(|_| PreImageError::WriteFailed)
    }
}

fn pre_image_path(data_dir: &Path, batch_id: &str) -> Result<PathBuf, PreImageError> {
    // A valid id is `bridge-` and a UUID, so no path separator can reach the name.
    if !valid_batch_id(batch_id) {
        return Err(PreImageError::BadBatchId);
    }
    Ok(data_dir.join(format!("master-pre-image-{batch_id}.json")))
}

/// A sidecar written for a batch: the digest and count its dispatch intent
/// records, and nothing else can supply them. Only [`write_pre_image`] builds
/// one, so an intent cannot name a digest or a count no sidecar was written
/// with.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct WrittenPreImage {
    batch_id: String,
    sha256: String,
    count: u64,
}

impl WrittenPreImage {
    pub(super) fn sha256(&self) -> &str {
        &self.sha256
    }

    pub(super) fn count(&self) -> u64 {
        self.count
    }
}

/// Write the sidecar for `batch`, whole and synced, under the admission lock.
/// A sidecar the reader would refuse for its size is never written. One already
/// there with the same bytes is the same write repeated; with other bytes it is
/// left alone and refused.
pub(super) fn write_pre_image(
    data_dir: &Path,
    lock: &ExclusiveAdmission,
    batch: &MasterBatch,
    pre_image: &PreImage,
) -> Result<WrittenPreImage, PreImageError> {
    write_pre_image_syncing(data_dir, lock, batch, pre_image, sync_directory)
}

/// [`write_pre_image`] with the directory sync given. The shared record write
/// fails when the sync after a new placement fails, but a sidecar already there
/// may be an earlier attempt's whose sync failed, so the directory is synced
/// here as well, whether the sidecar was just placed or was there already.
fn write_pre_image_syncing(
    data_dir: &Path,
    _lock: &ExclusiveAdmission,
    batch: &MasterBatch,
    pre_image: &PreImage,
    sync: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<WrittenPreImage, PreImageError> {
    let path = pre_image_path(data_dir, &batch.batch_id)?;
    let bytes = pre_image.bytes()?;
    if bytes.len() > MAX_RECORD_BYTES {
        return Err(PreImageError::TooLargeToWrite);
    }
    match persistence::write_record_once(&path, &bytes) {
        Ok(()) => {}
        Err(RecordOnce::Exists) => {
            let held = read_sidecar_bytes(&path)?;
            if held != bytes {
                return Err(PreImageError::ExistsDifferent);
            }
        }
        Err(RecordOnce::Failed) => return Err(PreImageError::WriteFailed),
    }
    sync(data_dir).map_err(|_| PreImageError::DirectorySyncFailed)?;
    Ok(WrittenPreImage {
        batch_id: batch.batch_id.clone(),
        sha256: sha256_hex(&bytes),
        count: pre_image.len() as u64,
    })
}

/// Remove the sidecar of `batch_id`, under the admission lock: for a dispatch
/// the journal refused after its sidecar was written, which no intent names. An
/// absent sidecar is already removed. Never call it for a batch that may have an
/// intent recorded: the sidecar is what that intent's digest was taken from.
pub(super) fn remove_pre_image(
    data_dir: &Path,
    _lock: &ExclusiveAdmission,
    batch_id: &str,
) -> Result<(), PreImageError> {
    match std::fs::remove_file(pre_image_path(data_dir, batch_id)?) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(PreImageError::RemoveFailed),
    }
    sync_directory(data_dir).map_err(|_| PreImageError::RemoveFailed)
}

fn read_sidecar_bytes(path: &Path) -> Result<Vec<u8>, PreImageError> {
    let file = match local_file::open_local_file(path, false) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(PreImageError::Missing)
        }
        Err(_) => return Err(PreImageError::Unreadable),
    };
    let mut bytes = Vec::new();
    file.take(MAX_RECORD_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| PreImageError::Unreadable)?;
    if bytes.len() > MAX_RECORD_BYTES {
        return Err(PreImageError::TooLarge);
    }
    Ok(bytes)
}

/// Read the sidecar back, refusing it unless its bytes carry the digest and
/// its map the count the dispatch intent recorded.
pub(super) fn read_pre_image(
    data_dir: &Path,
    batch_id: &str,
    expected_sha256: &str,
    expected_count: u64,
) -> Result<PreImage, PreImageError> {
    let bytes = read_sidecar_bytes(&pre_image_path(data_dir, batch_id)?)?;
    if sha256_hex(&bytes) != expected_sha256 {
        return Err(PreImageError::ShaMismatch);
    }
    let map: BTreeMap<String, u64> =
        serde_json::from_slice(&bytes).map_err(|_| PreImageError::Malformed)?;
    // Parse through the one constructor, so a key this module would not write
    // is refused here as well.
    let pre_image = PreImage::new(map.clone())?;
    if pre_image.entries() != &map {
        return Err(PreImageError::Malformed);
    }
    if pre_image.len() as u64 != expected_count {
        return Err(PreImageError::CountMismatch);
    }
    Ok(pre_image)
}

#[cfg(test)]
#[path = "agent_master_ledger_tests.rs"]
mod tests;
