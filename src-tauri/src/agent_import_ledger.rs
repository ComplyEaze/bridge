//! Legacy full batch records plus compact, hash-bound verification status updates.
use super::*;
use std::io::BufRead;

// The MCP frame is at most 5,000,000 bytes. This also leaves room for worst-case
// JSON escaping, repeated transaction labels and batch metadata in old records.
// Bound individual records, not append-only journal history.
pub(super) const MAX_RECORD_BYTES: usize = 32 * 1024 * 1024;

/// The most vouchers one native post may send, and so the most REMOTEIDs one
/// dispatch intent may record: the writer refuses more and the reader admits
/// no more. Imports of 50 were measured on the raw gateway (protocol reference
/// §11c.5, PARTIAL); through Bridge's own post path it is not yet measured.
pub(super) const MAX_BATCH_POST_VOUCHERS: usize = 50;

/// The name of one saved proof pair: when it was saved and the SHA-256 of
/// its JSON, written `<UTC stamp>.<sha256>`, so a person listing the folder
/// sees the pairs in the order they were saved. Constructed only from a
/// proof's own bytes or parsed whole, so a journal record or a file name
/// can never carry a path (#911).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub(super) struct ProofName {
    stamp: String,
    sha256: String,
}

impl ProofName {
    pub(super) fn of(json: &[u8], saved_at: chrono::DateTime<Utc>) -> Self {
        Self {
            stamp: saved_at.format("%Y%m%dT%H%M%S%3fZ").to_string(),
            sha256: sha256_hex(json),
        }
    }

    pub(super) fn sha256(&self) -> &str {
        &self.sha256
    }

    /// The proof's JSON file name for `batch_id`.
    pub(super) fn json_file(&self, batch_id: &str) -> String {
        format!("{batch_id}.proof.{}.{}.json", self.stamp, self.sha256)
    }

    /// The proof's Markdown file name for `batch_id`.
    pub(super) fn markdown_file(&self, batch_id: &str) -> String {
        format!("{batch_id}.proof.{}.{}.md", self.stamp, self.sha256)
    }
}

impl TryFrom<String> for ProofName {
    type Error = String;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        let invalid = || "import_ledger_invalid".to_string();
        let (stamp, sha256) = text.split_once('.').ok_or_else(invalid)?;
        let digits = |range: std::ops::Range<usize>| {
            stamp
                .get(range)
                .is_some_and(|part| part.bytes().all(|byte| byte.is_ascii_digit()))
        };
        if stamp.len() != 19
            || !digits(0..8)
            || stamp.as_bytes()[8] != b'T'
            || !digits(9..18)
            || stamp.as_bytes()[18] != b'Z'
            || sha256.len() != 64
            || !sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(invalid());
        }
        Ok(Self {
            stamp: stamp.into(),
            sha256: sha256.into(),
        })
    }
}

impl From<ProofName> for String {
    fn from(name: ProofName) -> Self {
        format!("{}.{}", name.stamp, name.sha256)
    }
}

/// Which saved proof is a batch's current one: the pair its latest
/// verification record names, or, for a record an older build wrote (or no
/// record at all), the single `<batch>.proof.json` those builds replaced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum CurrentProof {
    Legacy,
    Saved(ProofName),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum StatusKind {
    VerificationStatus,
    DispatchIntent,
    DispatchResponse,
    /// What binding a native post to its own AlterID span decided: the bound
    /// vouchers, or why it was refused. At most one per batch, after its
    /// response; it never changes the batch's status.
    PostSpanVerdict,
}

/// The status a post-span verdict record carries. It is not a batch status:
/// readers keep the batch's own status when they meet it.
const POST_SPAN_VERDICT_STATUS: &str = "post_span_verdict";

/// What binding a batch's native post decided, as journaled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum PostSpanVerdict {
    Bound(Vec<super::span_identity::PostedVoucherIdentity>),
    /// A definitive refusal, by its code. Permanent: no later bind is tried.
    Refused(String),
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::agent) struct StatusRecord {
    record_type: StatusKind,
    batch_id: String,
    batch_sha256: String,
    status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    response: Option<DispatchResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    native_request_sha256: Option<String>,
    /// The REMOTEID the native post sends. Tally deletes a voucher only by the
    /// client REMOTEID it was created with, and exports its own GUID in that
    /// attribute instead, so this record is the only place it survives
    /// (bridge#579). Written with the dispatch intent, before the POST.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    native_remote_id: Option<String>,
    /// The REMOTEIDs a native batch post sends, one per voucher, in order.
    /// Written with a batch's dispatch intent, before the POST, and never
    /// beside `native_remote_id`. A binary older than this field refuses a
    /// journal holding one (`deny_unknown_fields`): loudly, never by
    /// skipping a record of what was sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    native_remote_ids: Option<Vec<String>>,
    /// The target company's voucher mark (`ALTVCHID`) in the last read before
    /// the POST. Written with a native dispatch intent, before the POST, so a
    /// later binding never rests on a mark read after it. A binary older than
    /// this field refuses a journal holding one (`deny_unknown_fields`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pre_post_voucher_mark: Option<u64>,
    /// A post-span verdict's bound vouchers, in batch order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bindings: Option<Vec<super::span_identity::PostedVoucherIdentity>>,
    /// A post-span verdict's refusal code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    binding_refusal: Option<String>,
    /// The proof pair a verification saved before this record, which names
    /// it current (#911). Only on a verification status record. A binary
    /// older than this field refuses a journal holding one
    /// (`deny_unknown_fields`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    proof: Option<ProofName>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DispatchResponse {
    pub(super) request_sha256: String,
    pub(super) response_sha256: String,
    pub(super) bytes: usize,
    pub(super) outcome: Option<bridge_tally_protocol::TallyImportOutcome>,
}

impl StatusRecord {
    #[cfg(test)]
    pub(in crate::agent) fn dispatch(batch: &ImportLedgerLine) -> Self {
        let mut record = Self::dispatch_native(batch, String::new(), Uuid::nil());
        record.native_request_sha256 = None;
        record.native_remote_id = None;
        record
    }

    pub(super) fn dispatch_native(
        batch: &ImportLedgerLine,
        request_sha256: String,
        remote_id: Uuid,
    ) -> Self {
        Self {
            record_type: StatusKind::DispatchIntent,
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.sha256.clone(),
            status: "dispatch_started".into(),
            response: None,
            native_request_sha256: Some(request_sha256),
            native_remote_id: Some(remote_id.hyphenated().to_string()),
            native_remote_ids: None,
            pre_post_voucher_mark: None,
            bindings: None,
            binding_refusal: None,
            proof: None,
        }
    }
    /// The dispatch intent of one native post, bound to the request it sends:
    /// its wire digest and the REMOTEID it carries come from the same value,
    /// so the record cannot name a different request (bridge#579).
    pub(super) fn dispatch_for(
        batch: &ImportLedgerLine,
        request: &super::post::NativePostRequest,
        pre_post_voucher_mark: Option<u64>,
    ) -> Self {
        let record = match request.remote_ids.as_slice() {
            // One voucher keeps the single-id shape. A build that predates
            // the pre-POST mark refuses this intent whatever its shape.
            [remote_id] => Self::dispatch_native(batch, request.request_sha256.clone(), *remote_id),
            remote_ids => Self {
                native_remote_id: None,
                native_remote_ids: Some(
                    remote_ids
                        .iter()
                        .map(|remote_id| remote_id.hyphenated().to_string())
                        .collect(),
                ),
                ..Self::dispatch_native(batch, request.request_sha256.clone(), Uuid::nil())
            },
        };
        Self {
            pre_post_voucher_mark,
            ..record
        }
    }

    /// The post-span verdict of one batch.
    pub(super) fn post_span_verdict(batch: &ImportLedgerLine, verdict: &PostSpanVerdict) -> Self {
        let (bindings, binding_refusal) = match verdict {
            PostSpanVerdict::Bound(bound) => (Some(bound.clone()), None),
            PostSpanVerdict::Refused(code) => (None, Some(code.clone())),
        };
        Self {
            record_type: StatusKind::PostSpanVerdict,
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.sha256.clone(),
            status: POST_SPAN_VERDICT_STATUS.into(),
            response: None,
            native_request_sha256: None,
            native_remote_id: None,
            native_remote_ids: None,
            pre_post_voucher_mark: None,
            bindings,
            binding_refusal,
            proof: None,
        }
    }

    /// Whether this record sets the batch's status. A post-span verdict does
    /// not: it is a fact about the post, beside whatever status the batch has.
    fn sets_status(&self) -> bool {
        !matches!(self.record_type, StatusKind::PostSpanVerdict)
    }

    /// The proof this record names current: a verification record names its
    /// own, or the legacy file when an older build wrote it; others none.
    fn current_proof(&self) -> Option<CurrentProof> {
        matches!(self.record_type, StatusKind::VerificationStatus).then(|| {
            self.proof
                .clone()
                .map_or(CurrentProof::Legacy, CurrentProof::Saved)
        })
    }

    fn span_verdict(&self) -> Option<PostSpanVerdict> {
        match (&self.bindings, &self.binding_refusal) {
            (Some(bound), None) => Some(PostSpanVerdict::Bound(bound.clone())),
            (None, Some(code)) => Some(PostSpanVerdict::Refused(code.clone())),
            _ => None,
        }
    }

    pub(super) fn response(batch: &ImportLedgerLine, response: DispatchResponse) -> Self {
        Self {
            record_type: StatusKind::DispatchResponse,
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.sha256.clone(),
            status: "response_received".into(),
            response: Some(response),
            native_request_sha256: None,
            native_remote_id: None,
            native_remote_ids: None,
            pre_post_voucher_mark: None,
            bindings: None,
            binding_refusal: None,
            proof: None,
        }
    }
}

impl StatusRecord {
    /// A verification's status record, naming the proof pair it saved.
    pub(super) fn verified(batch: &ImportLedgerLine, proof: ProofName) -> Self {
        Self {
            proof: Some(proof),
            ..Self::from(batch)
        }
    }
}

impl From<&ImportLedgerLine> for StatusRecord {
    fn from(batch: &ImportLedgerLine) -> Self {
        Self {
            record_type: StatusKind::VerificationStatus,
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.sha256.clone(),
            status: batch.status.clone(),
            response: None,
            native_request_sha256: None,
            native_remote_id: None,
            native_remote_ids: None,
            pre_post_voucher_mark: None,
            bindings: None,
            binding_refusal: None,
            proof: None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct VerificationGeneration(usize);

pub(super) struct BatchSnapshot {
    pub(super) batch: ImportLedgerLine,
    pub(super) dispatched: bool,
    pub(super) response: Option<DispatchResponse>,
    /// The REMOTEID recorded with the native dispatch intent, if any.
    pub(super) native_remote_id: Option<String>,
    /// The voucher mark recorded with the native dispatch intent, if any.
    pub(super) pre_post_voucher_mark: Option<u64>,
    /// What binding the native post to its own span decided, if journaled.
    pub(super) span_verdict: Option<PostSpanVerdict>,
    /// The proof the latest verification record names current.
    pub(super) current_proof: CurrentProof,
    // Last matching physical journal record, including identical status appends.
    pub(super) generation: VerificationGeneration,
}

enum Record {
    Batch(Box<ImportLedgerLine>),
    Status(Box<StatusRecord>),
}

/// Validate the entire journal, retaining only the requested batch payload.
/// `None` performs build admission without retaining any voucher payload.
pub(super) fn read_snapshot(
    reader: impl BufRead,
    batch_id: Option<&str>,
) -> Result<Option<BatchSnapshot>, String> {
    let mut selected: Option<BatchSnapshot> = None;
    scan_records(reader, |record, generation| match record {
        Record::Batch(batch) if batch_id == Some(batch.batch_id.as_str()) => {
            selected = Some(BatchSnapshot {
                response: selected
                    .as_ref()
                    .and_then(|snapshot| snapshot.response.clone()),
                native_remote_id: selected
                    .as_ref()
                    .and_then(|snapshot| snapshot.native_remote_id.clone()),
                pre_post_voucher_mark: selected
                    .as_ref()
                    .and_then(|snapshot| snapshot.pre_post_voucher_mark),
                span_verdict: selected
                    .as_ref()
                    .and_then(|snapshot| snapshot.span_verdict.clone()),
                current_proof: selected.as_ref().map_or(CurrentProof::Legacy, |snapshot| {
                    snapshot.current_proof.clone()
                }),
                dispatched: selected
                    .as_ref()
                    .is_some_and(|snapshot| snapshot.dispatched),
                batch: *batch,
                generation,
            });
        }
        Record::Status(update) if batch_id == Some(update.batch_id.as_str()) => {
            // Whole-journal admission already established the preceding batch.
            let snapshot = selected
                .as_mut()
                .expect("status refers to an admitted batch");
            if let Some(response) = update.response.clone() {
                snapshot.response = Some(response);
            }
            snapshot.dispatched |= matches!(update.record_type, StatusKind::DispatchIntent);
            if update.native_remote_id.is_some() {
                snapshot.native_remote_id = update.native_remote_id.clone();
            }
            if update.pre_post_voucher_mark.is_some() {
                snapshot.pre_post_voucher_mark = update.pre_post_voucher_mark;
            }
            if let Some(verdict) = update.span_verdict() {
                snapshot.span_verdict = Some(verdict);
            }
            if let Some(proof) = update.current_proof() {
                snapshot.current_proof = proof;
            }
            if update.sets_status() {
                snapshot.batch.status = update.status;
            }
            snapshot.generation = generation;
        }
        _ => {}
    })?;
    Ok(selected)
}

/// Validate the entire journal, retaining every build that carries one wire
/// identity: the original batch and each amendment of it, in journal order.
pub(super) fn read_lineage(
    reader: impl BufRead,
    identity_batch_id: &str,
) -> Result<Vec<BatchSnapshot>, String> {
    let mut builds: Vec<BatchSnapshot> = Vec::new();
    let mut latest: BTreeMap<String, usize> = BTreeMap::new();
    scan_records(reader, |record, generation| match record {
        Record::Batch(batch) if batch.identity_batch_id() == identity_batch_id => {
            // A repeated full record replaces its earlier snapshot, keeping
            // any dispatch already recorded against that batch.
            let prior = latest.get(&batch.batch_id).map(|index| &builds[*index]);
            let snapshot = BatchSnapshot {
                response: prior.and_then(|snapshot| snapshot.response.clone()),
                native_remote_id: prior.and_then(|snapshot| snapshot.native_remote_id.clone()),
                pre_post_voucher_mark: prior.and_then(|snapshot| snapshot.pre_post_voucher_mark),
                span_verdict: prior.and_then(|snapshot| snapshot.span_verdict.clone()),
                current_proof: prior.map_or(CurrentProof::Legacy, |snapshot| {
                    snapshot.current_proof.clone()
                }),
                dispatched: prior.is_some_and(|snapshot| snapshot.dispatched),
                batch: *batch,
                generation,
            };
            match latest.get(&snapshot.batch.batch_id) {
                Some(index) => builds[*index] = snapshot,
                None => {
                    latest.insert(snapshot.batch.batch_id.clone(), builds.len());
                    builds.push(snapshot);
                }
            }
        }
        Record::Status(update) => {
            if let Some(index) = latest.get(&update.batch_id) {
                let snapshot = &mut builds[*index];
                if let Some(response) = update.response.clone() {
                    snapshot.response = Some(response);
                }
                snapshot.dispatched |= matches!(update.record_type, StatusKind::DispatchIntent);
                if update.native_remote_id.is_some() {
                    snapshot.native_remote_id = update.native_remote_id.clone();
                }
                if update.pre_post_voucher_mark.is_some() {
                    snapshot.pre_post_voucher_mark = update.pre_post_voucher_mark;
                }
                if let Some(verdict) = update.span_verdict() {
                    snapshot.span_verdict = Some(verdict);
                }
                if let Some(proof) = update.current_proof() {
                    snapshot.current_proof = proof;
                }
                if update.sets_status() {
                    snapshot.batch.status = update.status;
                }
                snapshot.generation = generation;
            }
        }
        _ => {}
    })?;
    Ok(builds)
}

/// Finds the sole batch identity bound to a persisted XML digest.  The caller
/// still reads that batch's snapshot afterwards, so status updates remain part
/// of the normal snapshot admission path.
pub(super) fn find_batch_id_by_sha256(
    reader: impl BufRead,
    wanted_sha256: &str,
) -> Result<Option<String>, String> {
    let mut batch_id = None;
    scan_records(reader, |record, _| {
        if let Record::Batch(batch) = record {
            if batch.sha256 == wanted_sha256 {
                match &batch_id {
                    Some(existing) if existing != &batch.batch_id => {
                        // A digest is an identity only while it identifies one
                        // local batch. Do not choose between two history entries.
                        batch_id = Some(String::new());
                    }
                    Some(_) => {}
                    None => batch_id = Some(batch.batch_id),
                }
            }
        }
    })?;
    match batch_id.as_deref() {
        Some("") => Err("import_batch_digest_ambiguous".into()),
        Some(id) => Ok(Some(id.to_string())),
        None => Ok(None),
    }
}

/// A REMOTEID as the journal records it: a canonical, lower-case, hyphenated
/// UUID that is not nil.
fn is_canonical_remote_id(remote_id: &str) -> bool {
    Uuid::parse_str(remote_id)
        .is_ok_and(|id| !id.is_nil() && id.hyphenated().to_string() == remote_id)
}

/// Whether any dispatch intent in the journal already records any of
/// `remote_ids`, as a single post's REMOTEID or as one of a batch's.
/// The whole journal is admitted on the way, as for every other read.
pub(super) fn remote_ids_recorded(
    reader: impl BufRead,
    remote_ids: &[Uuid],
) -> Result<bool, String> {
    let wanted = remote_ids
        .iter()
        .map(|remote_id| remote_id.hyphenated().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let mut recorded = false;
    scan_records(reader, |record, _| {
        if let Record::Status(update) = record {
            recorded |= update
                .native_remote_id
                .iter()
                .chain(update.native_remote_ids.iter().flatten())
                .any(|recorded_id| wanted.contains(recorded_id));
        }
    })?;
    Ok(recorded)
}

/// How settled the journal's batches are, for the local-data report: which
/// batches a later post or verification still leans on. Nothing is retained
/// but counts.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Settlement {
    /// Distinct batches in the journal.
    pub(super) batches: usize,
    /// Batches Bridge sent to Tally (a dispatch intent) or that a readback
    /// found posted at some point.
    pub(super) sent_or_found: usize,
    /// Batches with a dispatch intent that are not settled: no recorded
    /// response, or a response but the latest status is not `posted_verified`.
    pub(super) unsettled: usize,
    /// Of `unsettled`, the batches with no recorded response.
    pub(super) unsettled_no_response: usize,
    /// Batches with no recorded dispatch that were never found posted. That
    /// includes a batch imported by hand whose verification is incomplete, which
    /// may well be in Tally: this is what the journal holds, not what Tally
    /// holds, and no deletion may rest on it. Their saved file is what
    /// `post_import` would send, so moving or deleting the folder strands them.
    pub(super) no_dispatch_never_verified: usize,
}

/// Validate the whole journal and count its batches by settlement (#local-data).
pub(super) fn settlement(reader: impl BufRead) -> Result<Settlement, String> {
    #[derive(Default)]
    struct Progress {
        dispatched: bool,
        responded: bool,
        /// The latest status is `posted_verified`.
        verified: bool,
        /// A readback found the batch posted at some point.
        found: bool,
    }
    let mut batches: BTreeMap<String, Progress> = BTreeMap::new();
    scan_records(reader, |record, _| match record {
        Record::Batch(batch) => {
            let progress = batches.entry(batch.batch_id.clone()).or_default();
            progress.verified = batch.status == "posted_verified";
            progress.found |= progress.verified;
        }
        Record::Status(update) => {
            let progress = batches.entry(update.batch_id.clone()).or_default();
            match update.record_type {
                StatusKind::DispatchIntent => progress.dispatched = true,
                StatusKind::DispatchResponse => progress.responded = true,
                StatusKind::VerificationStatus | StatusKind::PostSpanVerdict => {}
            }
            // The latest status of every kind is the batch's status, as
            // `read_snapshot` takes it: a dispatch intent or a response after a
            // hand-import's `posted_verified` makes the batch unverified again.
            // A post-span verdict sets no status.
            if update.sets_status() {
                progress.verified = update.status == "posted_verified";
                progress.found |= progress.verified;
            }
        }
    })?;
    let sent_or_found = batches
        .values()
        .filter(|progress| progress.dispatched || progress.found)
        .count();
    let unsettled = batches
        .values()
        .filter(|progress| progress.dispatched && !(progress.responded && progress.verified));
    Ok(Settlement {
        batches: batches.len(),
        sent_or_found,
        no_dispatch_never_verified: batches.len() - sent_or_found,
        unsettled_no_response: unsettled
            .clone()
            .filter(|progress| !progress.responded)
            .count(),
        unsettled: unsettled.count(),
    })
}

/// A voucher's date and its entries as (amount, is debit), sorted, so two
/// vouchers compare equal whatever order their ledgers were listed in and
/// whichever ledgers they name. `None` when an amount cannot be read, which a
/// caller treats as a match.
type RowShape = (bridge_tally_core::TallyDate, Vec<(String, bool)>);

fn row_shape(voucher: &ImportVoucher) -> Option<RowShape> {
    let mut entries = voucher
        .entries
        .iter()
        .map(|entry| {
            Some((
                super::verification::canonical_verification_amount(&entry.amount).ok()?,
                matches!(entry.side, EntrySide::Dr),
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    entries.sort();
    Some((voucher.date.clone(), entries))
}

/// The id of another batch of the same company that Bridge sent to Tally (a
/// dispatch intent) or that a readback found posted (`posted_verified`) and that
/// holds a voucher `batch` would post again (#876); the first in id order.
///
/// Two vouchers are the same row when they share a transaction id and either
/// the id has the form a bank-statement build derives, which hashes the row's
/// date, amounts, running balance and narration and so survives a change of
/// ledger, or their date and amounts agree. Order of the two batches does not
/// matter: a batch built earlier but dispatched later still counts. A batch
/// that was only built, or whose journal record was replaced by one without the
/// row, does not. The batch's own id is never matched. The whole journal is
/// admitted on the way, as for every other read.
pub(super) fn rows_already_posted(
    reader: impl BufRead,
    batch: &ImportLedgerLine,
) -> Result<Option<String>, String> {
    vouchers_already_posted(
        reader,
        &batch.company_guid,
        Some(&batch.batch_id),
        &batch.vouchers,
    )
}

/// The same check for vouchers not yet in a batch (a build), where no batch id
/// of their own exists to skip.
pub(super) fn vouchers_already_posted(
    reader: impl BufRead,
    company_guid: &str,
    own_batch_id: Option<&str>,
    vouchers: &[ImportVoucher],
) -> Result<Option<String>, String> {
    let mut wanted: BTreeMap<&str, Vec<Option<RowShape>>> = BTreeMap::new();
    for voucher in vouchers {
        wanted
            .entry(voucher.bridge_txn_id.as_str())
            .or_default()
            .push(row_shape(voucher));
    }
    let mut holds_a_row = BTreeSet::new();
    let mut sent_or_found = BTreeSet::new();
    scan_records(reader, |record, _| match record {
        Record::Batch(other) if Some(other.batch_id.as_str()) != own_batch_id => {
            let same_row = other.company_guid.eq_ignore_ascii_case(company_guid)
                && other.vouchers.iter().any(|voucher| {
                    wanted
                        .get(voucher.bridge_txn_id.as_str())
                        .is_some_and(|shapes| {
                            bridge_bank_statement::proposals::is_statement_txn_id(
                                &voucher.bridge_txn_id,
                            ) || row_shape(voucher).is_none_or(|shape| {
                                shapes
                                    .iter()
                                    .any(|wanted| wanted.as_ref().is_none_or(|w| *w == shape))
                            })
                        })
                });
            if same_row {
                holds_a_row.insert(other.batch_id.clone());
            } else {
                holds_a_row.remove(&other.batch_id);
            }
            if other.status == "posted_verified" {
                sent_or_found.insert(other.batch_id.clone());
            }
        }
        Record::Status(update) => {
            if matches!(update.record_type, StatusKind::DispatchIntent)
                || update.status == "posted_verified"
            {
                sent_or_found.insert(update.batch_id.clone());
            }
        }
        Record::Batch(_) => {}
    })?;
    Ok(holds_a_row.intersection(&sent_or_found).next().cloned())
}

fn scan_records(
    mut reader: impl BufRead,
    mut visit: impl FnMut(Record, VerificationGeneration),
) -> Result<(), String> {
    // Compact records must be checked even for unrelated batches. Retain only
    // the latest hash per distinct ID, not every historical voucher payload.
    // Memory therefore still grows with distinct IDs, not with status history.
    // Each batch's latest hash, with its voucher count for its REMOTEIDs.
    let mut latest: BTreeMap<String, (String, usize)> = BTreeMap::new();
    let mut dispatched: BTreeMap<String, Option<String>> = BTreeMap::new();
    // For post-span verdicts: each batch's transaction ids, the batches with a
    // recorded response, and those with a verdict.
    let mut txn_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    // Batches whose response carries a parsed outcome, and whose intent
    // recorded a pre-POST mark: only such a batch can have a post-span verdict
    // (the writer binds only from both).
    let mut responded: BTreeSet<String> = BTreeSet::new();
    let mut marked: BTreeSet<String> = BTreeSet::new();
    let mut with_verdict: BTreeSet<String> = BTreeSet::new();
    let mut line = Vec::new();
    let mut ordinal = 0_usize;
    while read_record(&mut reader, &mut line)? {
        let text =
            std::str::from_utf8(&line).map_err(|_| "import_ledger_unavailable".to_string())?;
        let value: Value =
            serde_json::from_str(text).map_err(|_| "import_ledger_invalid".to_string())?;
        let record = if value.get("record_type").is_some() {
            let update: StatusRecord =
                serde_json::from_value(value).map_err(|_| "import_ledger_invalid".to_string())?;
            let Some((batch_sha256, voucher_count)) = latest.get(&update.batch_id) else {
                return Err("import_ledger_invalid".into());
            };
            if *batch_sha256 != update.batch_sha256 {
                return Err("import_ledger_invalid".into());
            }
            if let Some(hash) = &update.native_request_sha256 {
                if !matches!(update.record_type, StatusKind::DispatchIntent)
                    || hash.len() != 64
                    || !hash.bytes().all(|byte| byte.is_ascii_hexdigit())
                {
                    return Err("import_ledger_invalid".into());
                }
            }
            // A recorded REMOTEID belongs only to a native dispatch intent,
            // beside its request hash, and must be a canonical UUID.
            if let Some(remote_id) = &update.native_remote_id {
                if update.native_request_sha256.is_none() || !is_canonical_remote_id(remote_id) {
                    return Err("import_ledger_invalid".into());
                }
            }
            // A batch's REMOTEIDs follow the same rule, and are distinct, one
            // per voucher of the batch, at least two (one is `native_remote_id`)
            // and at most the batch cap.
            if let Some(remote_ids) = &update.native_remote_ids {
                let distinct = remote_ids.iter().collect::<std::collections::BTreeSet<_>>();
                if update.native_request_sha256.is_none()
                    || update.native_remote_id.is_some()
                    || remote_ids.len() != *voucher_count
                    || !(2..=MAX_BATCH_POST_VOUCHERS).contains(&remote_ids.len())
                    || distinct.len() != remote_ids.len()
                    || !remote_ids
                        .iter()
                        .all(|remote_id| is_canonical_remote_id(remote_id))
                {
                    return Err("import_ledger_invalid".into());
                }
            }
            if matches!(update.record_type, StatusKind::DispatchIntent)
                && dispatched
                    .insert(
                        update.batch_id.clone(),
                        update.native_request_sha256.clone(),
                    )
                    .is_some()
            {
                return Err("import_ledger_duplicate_dispatch".into());
            }
            // A pre-POST mark belongs only to a native dispatch intent.
            if update.pre_post_voucher_mark.is_some()
                && !(matches!(update.record_type, StatusKind::DispatchIntent)
                    && update.native_request_sha256.is_some())
            {
                return Err("import_ledger_invalid".into());
            }
            if update.pre_post_voucher_mark.is_some() {
                marked.insert(update.batch_id.clone());
            }
            // A saved proof belongs only to a verification status record.
            if update.proof.is_some()
                && !matches!(update.record_type, StatusKind::VerificationStatus)
            {
                return Err("import_ledger_invalid".into());
            }
            if matches!(update.record_type, StatusKind::PostSpanVerdict) {
                admit_post_span_verdict(
                    &update,
                    *voucher_count,
                    txn_ids.get(&update.batch_id),
                    responded.contains(&update.batch_id) && marked.contains(&update.batch_id),
                    &mut with_verdict,
                )?;
            } else if update.bindings.is_some() || update.binding_refusal.is_some() {
                return Err("import_ledger_invalid".into());
            }
            if matches!(update.record_type, StatusKind::DispatchResponse) {
                let request_hash = dispatched
                    .get(&update.batch_id)
                    .ok_or("import_ledger_invalid")?;
                let response = update.response.as_ref().ok_or("import_ledger_invalid")?;
                if response.outcome.is_some() {
                    responded.insert(update.batch_id.clone());
                }
                if request_hash
                    .as_ref()
                    .is_some_and(|hash| hash != &response.request_sha256)
                {
                    return Err("import_ledger_invalid".into());
                }
            } else if update.response.is_some() {
                return Err("import_ledger_invalid".into());
            }
            Record::Status(Box::new(update))
        } else {
            let batch: ImportLedgerLine =
                serde_json::from_value(value).map_err(|_| "import_ledger_invalid".to_string())?;
            if dispatched.contains_key(&batch.batch_id)
                && latest.get(&batch.batch_id).map(|(sha256, _)| sha256) != Some(&batch.sha256)
            {
                return Err("import_ledger_invalid".into());
            }
            latest.insert(
                batch.batch_id.clone(),
                (batch.sha256.clone(), batch.vouchers.len()),
            );
            txn_ids.insert(
                batch.batch_id.clone(),
                batch
                    .vouchers
                    .iter()
                    .map(|voucher| voucher.bridge_txn_id.clone())
                    .collect(),
            );
            Record::Batch(Box::new(batch))
        };
        visit(record, VerificationGeneration(ordinal));
        ordinal = ordinal
            .checked_add(1)
            .ok_or_else(|| "import_ledger_invalid".to_string())?;
    }
    Ok(())
}

/// Every GUID that a batch other than `own_batch_id` has bound, the whole
/// journal admitted on the way. A bind refuses to claim one of these again.
pub(super) fn guids_bound_elsewhere(
    reader: impl BufRead,
    own_batch_id: &str,
) -> Result<BTreeSet<String>, String> {
    let mut guids = BTreeSet::new();
    scan_records(reader, |record, _| {
        if let Record::Status(update) = record {
            if update.batch_id != own_batch_id {
                guids.extend(
                    update
                        .bindings
                        .iter()
                        .flatten()
                        .map(|bound| bound.guid.clone()),
                );
            }
        }
    })?;
    Ok(guids)
}

/// Admits one post-span verdict, or refuses the journal. It must follow the
/// batch's recorded response, be the batch's only verdict, carry exactly one
/// of bindings and a refusal code, and set no status. Bindings name each of the
/// batch's transaction ids once, with distinct GUIDs and MasterIDs. A GUID that
/// another batch also bound is not a reason to refuse the whole journal (a split
/// company keeps its parent's GUID); the bind itself refuses to reuse one.
fn admit_post_span_verdict(
    update: &StatusRecord,
    voucher_count: usize,
    batch_txn_ids: Option<&BTreeSet<String>>,
    responded: bool,
    with_verdict: &mut BTreeSet<String>,
) -> Result<(), String> {
    let invalid = || "import_ledger_invalid".to_string();
    if update.status != POST_SPAN_VERDICT_STATUS
        || !responded
        || update.native_request_sha256.is_some()
        || update.native_remote_id.is_some()
        || update.native_remote_ids.is_some()
        || !with_verdict.insert(update.batch_id.clone())
    {
        return Err(invalid());
    }
    match (&update.bindings, &update.binding_refusal) {
        (Some(bindings), None) => {
            let batch_txn_ids = batch_txn_ids.ok_or_else(invalid)?;
            let named = bindings
                .iter()
                .map(|bound| bound.bridge_txn_id.as_str())
                .collect::<BTreeSet<_>>();
            if bindings.len() != voucher_count
                || named.len() != bindings.len()
                || !named.iter().all(|txn_id| batch_txn_ids.contains(*txn_id))
            {
                return Err(invalid());
            }
            let mut guids = BTreeSet::new();
            let mut master_ids = BTreeSet::new();
            for bound in bindings {
                let canonical = bound.guid.len() <= 128
                    && !bound.guid.is_empty()
                    && bound.guid.bytes().all(|byte| {
                        byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte) || byte == b'-'
                    });
                if !canonical || !guids.insert(&bound.guid) || !master_ids.insert(bound.master_id) {
                    return Err(invalid());
                }
            }
            Ok(())
        }
        (None, Some(code))
            if (1..=64).contains(&code.len())
                && code.bytes().all(|byte| {
                    byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'
                }) =>
        {
            Ok(())
        }
        _ => Err(invalid()),
    }
}

fn read_record(reader: &mut impl BufRead, line: &mut Vec<u8>) -> Result<bool, String> {
    line.clear();
    loop {
        let available = reader
            .fill_buf()
            .map_err(|_| "import_ledger_unavailable".to_string())?;
        if available.is_empty() {
            return if line.is_empty() {
                Ok(false)
            } else {
                // Even a complete JSON value needs its record delimiter;
                // otherwise the next append would concatenate two objects.
                Err("import_ledger_invalid".into())
            };
        }
        let newline = available.iter().position(|byte| *byte == b'\n');
        let length = newline.map_or(available.len(), |index| index + 1);
        if length > MAX_RECORD_BYTES.saturating_sub(line.len()) {
            return Err("import_ledger_record_too_large".into());
        }
        line.extend_from_slice(&available[..length]);
        reader.consume(length);
        if newline.is_some() {
            return Ok(true);
        }
    }
}

#[cfg(test)]
pub(super) fn parse_snapshots(text: &str) -> Result<Vec<BatchSnapshot>, String> {
    read_history(std::io::Cursor::new(text.as_bytes()))
}

#[cfg(test)]
pub(super) fn read_history(reader: impl BufRead) -> Result<Vec<BatchSnapshot>, String> {
    let mut batches: Vec<BatchSnapshot> = Vec::new();
    let mut latest: BTreeMap<String, usize> = BTreeMap::new();
    scan_records(reader, |record, generation| match record {
        Record::Batch(batch) => {
            // Keep legacy full-record history readable without rewriting it.
            let dispatched = latest
                .get(&batch.batch_id)
                .is_some_and(|index| batches[*index].dispatched);
            let response = latest
                .get(&batch.batch_id)
                .and_then(|index| batches.get(*index))
                .and_then(|snapshot| snapshot.response.clone());
            let prior = latest
                .get(&batch.batch_id)
                .and_then(|index| batches.get(*index));
            let native_remote_id = prior.and_then(|snapshot| snapshot.native_remote_id.clone());
            let pre_post_voucher_mark = prior.and_then(|snapshot| snapshot.pre_post_voucher_mark);
            let span_verdict = prior.and_then(|snapshot| snapshot.span_verdict.clone());
            let current_proof = prior.map_or(CurrentProof::Legacy, |snapshot| {
                snapshot.current_proof.clone()
            });
            latest.insert(batch.batch_id.clone(), batches.len());
            batches.push(BatchSnapshot {
                response,
                native_remote_id,
                pre_post_voucher_mark,
                span_verdict,
                current_proof,
                dispatched,
                batch: *batch,
                generation,
            });
        }
        Record::Status(update) => {
            let snapshot = &mut batches[latest[&update.batch_id]];
            if let Some(response) = update.response.clone() {
                snapshot.response = Some(response);
            }
            snapshot.dispatched |= matches!(update.record_type, StatusKind::DispatchIntent);
            if update.native_remote_id.is_some() {
                snapshot.native_remote_id = update.native_remote_id.clone();
            }
            if update.pre_post_voucher_mark.is_some() {
                snapshot.pre_post_voucher_mark = update.pre_post_voucher_mark;
            }
            if let Some(verdict) = update.span_verdict() {
                snapshot.span_verdict = Some(verdict);
            }
            if let Some(proof) = update.current_proof() {
                snapshot.current_proof = proof;
            }
            if update.sets_status() {
                snapshot.batch.status = update.status;
            }
            snapshot.generation = generation;
        }
    })?;
    Ok(batches)
}

#[cfg(test)]
#[path = "agent_import_ledger_stream_tests.rs"]
mod stream_tests;
