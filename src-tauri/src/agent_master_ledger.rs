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
//! would refuse is never written. Callers hold the import admission lock
//! (`Server::lock_import_admission`), which serializes every writer of this
//! data directory; this module takes no lock of its own.
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
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};

const JOURNAL_FILE: &str = "agent-master-ledger.jsonl";

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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct IntentItem {
    pub(super) name: String,
    pub(super) canonical_key: String,
    pub(super) absence_key: String,
}

/// A company's master mark (`ALTMSTID`) in the last read before the post.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterMark {
    pub(super) company_guid: String,
    pub(super) altmstid: u64,
}

/// Written before the POST. Its presence is the refusal to send again.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterDispatchIntent {
    pub(super) batch_id: String,
    pub(super) batch_sha256: String,
    pub(super) native_request_sha256: String,
    pub(super) items: Vec<IntentItem>,
    /// The digest of the pre-image sidecar written before this intent.
    pub(super) pre_image_sha256: String,
    pub(super) pre_image_count: u64,
    pub(super) pre_post_master_marks: Vec<MasterMark>,
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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct VerdictItem {
    pub(super) name: String,
    pub(super) guid: Option<String>,
    pub(super) reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MasterVerdict {
    pub(super) batch_id: String,
    pub(super) batch_sha256: String,
    pub(super) state: MasterVerdictState,
    pub(super) items: Vec<VerdictItem>,
}

/// One journal line. Every `match` on it names every variant, so a new record
/// kind is a compile error until each decision site handles it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl MasterRecord {
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
    /// An intent or verdict whose items are not the batch's, in order.
    ItemsMismatch,
    /// `posted_verified` for an item with no GUID.
    VerifiedWithoutGuid,
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
    Invalid(Invalid),
}

impl MasterLedgerError {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "master_ledger_unavailable",
            Self::RecordTooLarge => "master_ledger_record_too_large",
            Self::SerializationFailed => "master_ledger_serialization_failed",
            Self::RollbackFailed => "master_ledger_rollback_failed",
            Self::DuplicateDispatch => "master_ledger_duplicate_dispatch",
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
fn is_canonical_guid(value: &str) -> bool {
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

/// Tally folds case in a master's name, so two names that differ only in case
/// are one name.
fn name_key(name: &str) -> String {
    name.trim().to_lowercase()
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
        for field in [guid, name, number, books_from] {
            non_blank(field)?;
        }
        if self.items.is_empty() {
            return Err(Invalid::EmptyItems);
        }
        let mut names = BTreeSet::new();
        for item in &self.items {
            item.validate()?;
            if !names.insert(name_key(&item.name)) {
                return Err(Invalid::DuplicateName);
            }
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
        let mut companies = BTreeSet::new();
        for mark in &self.pre_post_master_marks {
            non_blank(&mark.company_guid)?;
            if !companies.insert(mark.company_guid.to_lowercase()) {
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
        // The one state that says the write is whole names every item's GUID.
        let all_found = match self.state {
            MasterVerdictState::PostedVerified => true,
            MasterVerdictState::ReconciliationRequired
            | MasterVerdictState::PostedIntoExistingMaster
            | MasterVerdictState::MastersAfterPostUnconfirmed
            | MasterVerdictState::MasterWriteLocatedElsewhere => false,
        };
        if all_found && self.items.iter().any(|item| item.guid.is_none()) {
            return Err(Invalid::VerifiedWithoutGuid);
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

/// Everything the journal holds about one batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BatchState {
    pub(super) batch: MasterBatch,
    pub(super) intent: Option<MasterDispatchIntent>,
    pub(super) response: Option<MasterDispatchResponse>,
    /// The latest verdict.
    pub(super) verdict: Option<MasterVerdict>,
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
    /// may have been, sent.
    pub(super) fn dispatched(&self, batch_id: &str) -> bool {
        self.state(batch_id)
            .is_some_and(|state| state.intent.is_some())
    }

    pub(super) fn verdict(&self, batch_id: &str) -> Option<&MasterVerdict> {
        self.state(batch_id)
            .and_then(|state| state.verdict.as_ref())
    }

    /// The state of the batch a record belongs to, if that batch is introduced
    /// and the record carries its digest.
    fn batch_of(&mut self, record: &MasterRecord) -> Result<&mut BatchState, Invalid> {
        let state = self
            .batches
            .get_mut(record.batch_id())
            .ok_or(Invalid::NoBatch)?;
        if state.batch.batch_sha256 != record.batch_sha256() {
            return Err(Invalid::ShaMismatch);
        }
        Ok(state)
    }

    /// Admit one record, or refuse it and leave the journal as it was.
    pub(super) fn admit(&mut self, record: MasterRecord) -> Result<(), MasterLedgerError> {
        record.validate().map_err(MasterLedgerError::Invalid)?;
        match &record {
            MasterRecord::Batch(batch) => {
                match self.batches.get(&batch.batch_id) {
                    // The same batch written twice is one batch.
                    Some(held) if held.batch == *batch => {}
                    Some(_) => return Err(MasterLedgerError::Invalid(Invalid::BatchIdReused)),
                    None => {
                        self.batches.insert(
                            batch.batch_id.clone(),
                            BatchState {
                                batch: batch.clone(),
                                intent: None,
                                response: None,
                                verdict: None,
                            },
                        );
                    }
                }
                Ok(())
            }
            MasterRecord::DispatchIntent(intent) => {
                let state = self.batch_of(&record).map_err(MasterLedgerError::Invalid)?;
                if state.intent.is_some() {
                    return Err(MasterLedgerError::DuplicateDispatch);
                }
                let named = intent
                    .items
                    .iter()
                    .map(|item| item.name.as_str())
                    .collect::<Vec<_>>();
                if named != state.batch.names() {
                    return Err(MasterLedgerError::Invalid(Invalid::ItemsMismatch));
                }
                state.intent = Some(intent.clone());
                Ok(())
            }
            MasterRecord::DispatchResponse(response) => {
                let state = self.batch_of(&record).map_err(MasterLedgerError::Invalid)?;
                if state.intent.is_none() {
                    return Err(MasterLedgerError::Invalid(Invalid::ResponseWithoutIntent));
                }
                state.response = Some(response.clone());
                Ok(())
            }
            MasterRecord::Verdict(verdict) => {
                let state = self.batch_of(&record).map_err(MasterLedgerError::Invalid)?;
                if state.intent.is_none() {
                    return Err(MasterLedgerError::Invalid(Invalid::VerdictWithoutIntent));
                }
                let named = verdict
                    .items
                    .iter()
                    .map(|item| item.name.as_str())
                    .collect::<Vec<_>>();
                if named != state.batch.names() {
                    return Err(MasterLedgerError::Invalid(Invalid::ItemsMismatch));
                }
                state.verdict = Some(verdict.clone());
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
            let record: MasterRecord = serde_json::from_str(text)
                .map_err(|_| MasterLedgerError::Invalid(Invalid::Parse))?;
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

/// Append one record, after admitting it against the journal as it stands: a
/// record the reader would refuse (a second intent included) is never written.
pub(super) fn append(data_dir: &Path, record: MasterRecord) -> Result<(), MasterLedgerError> {
    let mut journal = read_journal(data_dir)?;
    let encoded =
        serde_json::to_string(&record).map_err(|_| MasterLedgerError::SerializationFailed)?;
    let encoded = format!("{encoded}\n");
    if encoded.len() > MAX_RECORD_BYTES {
        return Err(MasterLedgerError::RecordTooLarge);
    }
    journal.admit(record)?;
    append_private_import_ledger(
        &journal_path(data_dir),
        encoded.as_bytes(),
        set_private_file,
    )
    .map_err(|code| match code.as_str() {
        "import_ledger_rollback_failed" => MasterLedgerError::RollbackFailed,
        _ => MasterLedgerError::Unavailable,
    })
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
    WriteFailed,
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
            Self::ExistsDifferent | Self::WriteFailed => "master_pre_image_write_failed",
        }
    }
}

/// The company's masters before the post: each GUID, lower-case, with its
/// ALTERID. Sorted, so equal books give equal bytes and an equal digest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

/// Write the sidecar for `batch_id`, whole and synced, and return the digest
/// the dispatch intent records. A sidecar already there with the same bytes is
/// the same write repeated; with other bytes it is left alone and refused.
pub(super) fn write_pre_image(
    data_dir: &Path,
    batch_id: &str,
    pre_image: &PreImage,
) -> Result<String, PreImageError> {
    let path = pre_image_path(data_dir, batch_id)?;
    let bytes = pre_image.bytes()?;
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
    Ok(sha256_hex(&bytes))
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
