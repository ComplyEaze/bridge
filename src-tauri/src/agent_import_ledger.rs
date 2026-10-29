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
/// can never carry a path (#911). The digest is the JSON's, and only the
/// JSON is read back and checked against it; nothing in ComplyEaze Bridge
/// reads the `.md` back.
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
    /// A person's release of the Sales stop one batch holds (slice 4, ADR
    /// 0004): written only after a native dialog, only for a batch that was
    /// sent, and never a batch status. A later `posted_verified` of that batch
    /// voids it.
    StopRelease,
    /// A verification of a native batch read the company's voucher mark below
    /// that batch's floor: the book was rolled back (a backup restored) after
    /// the post (#1050). Recorded once per batch at a given mark, never a batch
    /// status. From then on no native batch of the same company and endpoint
    /// that it covers reads `posted_verified` ([`rollback_cover`]).
    RollbackSeen,
    /// A person's record that they checked in Tally a native batch matched by
    /// content only (#1039): its span binding was refused, or its answer from
    /// Tally was never recorded, so no later check binds it. Written only after
    /// a native dialog, and never a batch status: it changes no verdict, binds
    /// no voucher and lifts no stop. A later `posted_verified` of the batch, a
    /// later answer with an outcome, or any rollback cover voids it
    /// ([`ContentReviews`]). A binary older than this kind refuses the whole
    /// journal holding one, as for a stop release.
    ContentReview,
    /// A person's settlement of a native batch Tally answered as creating
    /// none of its vouchers (#1709): written only after a native dialog, in
    /// which the person said each voucher is in the book or will not be
    /// posted. It closes the batch in the journal's settlement count, and is
    /// never a batch status: it changes no verdict, sends nothing, releases no
    /// row (#876) and lifts no invoice stop. A later `posted_verified`, a
    /// later answer with an outcome, a binding, or any rollback cover voids it
    /// ([`PersonSettlements`]). A binary older than this kind refuses the
    /// whole journal holding one, as for a stop release.
    PersonSettlement,
}

/// The status a stop release record carries. It is not a batch status.
const STOP_RELEASE_STATUS: &str = "stop_released";

/// The status a content review record carries. It is not a batch status.
const CONTENT_REVIEW_STATUS: &str = "content_reviewed";

/// The status a person settlement record carries. It is not a batch status.
const PERSON_SETTLEMENT_STATUS: &str = "settled_by_person";

/// Which content-only state a person reviewed: the span binding refused by a
/// code other than `span_identity_reused`, or a post with no outcome recorded
/// from Tally's answer, which never binds. A value this build does not know is
/// refused with its record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ReviewedBinding {
    Refused,
    ResponseNotRecorded,
}

impl ReviewedBinding {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::Refused => "refused",
            Self::ResponseNotRecorded => "response_not_recorded",
        }
    }
}

/// The refusal a content review never covers: another batch's voucher was
/// bound in this post's span, which a person's check cannot settle.
pub(super) const IDENTITY_REUSED: &str = "span_identity_reused";

/// The status a rollback-seen record carries. It is not a batch status.
const ROLLBACK_SEEN_STATUS: &str = "rollback_seen";

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
    /// A stop release's record of what the fresh read at the release saw of
    /// the batch's invoice: evidence of what the person was shown, read by
    /// nothing. `Some` only on a stop release. A binary older than this field
    /// refuses a journal holding one (`deny_unknown_fields`), as for every
    /// field before it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    release_seen: Option<ReleaseSeen>,
    /// A rollback-seen record's company voucher mark as the verification read
    /// it (present in Tally's answer, never an omitted mark read as zero).
    /// `Some` only on a rollback-seen record. A binary older than this field
    /// refuses a journal holding one (`deny_unknown_fields`, and the record
    /// kind is unknown to it): the release notes say not to downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    mark_read: Option<u64>,
    /// A rollback-seen record's count of journal records the verification
    /// had read before it read the mark. Only what the journal held within
    /// that count decides which batches the record covers, so a post written
    /// meanwhile is judged on nothing it proved after the read. `Some` only on
    /// a rollback-seen record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    records_before_read: Option<u64>,
    /// A content review's record of the content-only state the person was
    /// shown. `Some` only on a content review. A binary older than this field
    /// refuses a journal holding one (`deny_unknown_fields`, and the record
    /// kind is unknown to it).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reviewed_binding: Option<ReviewedBinding>,
}

/// What the read at a stop release saw of the batch's invoice, as journaled
/// (`stop::Seen` without its words). A value this build does not know is
/// refused with its record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ReleaseSeen {
    Found,
    NotEstablished,
    Unreadable,
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
            release_seen: None,
            mark_read: None,
            records_before_read: None,
            reviewed_binding: None,
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
            release_seen: None,
            mark_read: None,
            records_before_read: None,
            reviewed_binding: None,
        }
    }

    /// Whether this record sets the batch's status. A post-span verdict does
    /// not: it is a fact about the post, beside whatever status the batch has.
    fn sets_status(&self) -> bool {
        !matches!(
            self.record_type,
            StatusKind::PostSpanVerdict
                | StatusKind::StopRelease
                | StatusKind::RollbackSeen
                | StatusKind::ContentReview
                | StatusKind::PersonSettlement
        )
    }

    /// The record that `batch`'s verification read the company's voucher
    /// mark at `mark_read`, below the batch's floor, having read
    /// `records_before_read` journal records before that read.
    pub(super) fn rollback_seen(
        batch: &ImportLedgerLine,
        mark_read: u64,
        records_before_read: u64,
    ) -> Self {
        Self {
            record_type: StatusKind::RollbackSeen,
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.sha256.clone(),
            status: ROLLBACK_SEEN_STATUS.into(),
            response: None,
            native_request_sha256: None,
            native_remote_id: None,
            native_remote_ids: None,
            pre_post_voucher_mark: None,
            bindings: None,
            binding_refusal: None,
            proof: None,
            release_seen: None,
            mark_read: Some(mark_read),
            records_before_read: Some(records_before_read),
            reviewed_binding: None,
        }
    }

    /// A person's release of the stop `batch` holds. `seen` is what the fresh
    /// read at the release saw of its invoice.
    pub(super) fn stop_release(batch: &ImportLedgerLine, seen: ReleaseSeen) -> Self {
        Self {
            record_type: StatusKind::StopRelease,
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.sha256.clone(),
            status: STOP_RELEASE_STATUS.into(),
            response: None,
            native_request_sha256: None,
            native_remote_id: None,
            native_remote_ids: None,
            pre_post_voucher_mark: None,
            bindings: None,
            binding_refusal: None,
            proof: None,
            release_seen: Some(seen),
            mark_read: None,
            records_before_read: None,
            reviewed_binding: None,
        }
    }

    /// A person's review of `batch`, matched by content only in the state
    /// `reviewed` (#1039).
    pub(super) fn content_review(batch: &ImportLedgerLine, reviewed: ReviewedBinding) -> Self {
        Self {
            record_type: StatusKind::ContentReview,
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.sha256.clone(),
            status: CONTENT_REVIEW_STATUS.into(),
            response: None,
            native_request_sha256: None,
            native_remote_id: None,
            native_remote_ids: None,
            pre_post_voucher_mark: None,
            bindings: None,
            binding_refusal: None,
            proof: None,
            release_seen: None,
            mark_read: None,
            records_before_read: None,
            reviewed_binding: Some(reviewed),
        }
    }

    /// A person's settlement of `batch`, which Tally answered as creating
    /// none of its vouchers (#1709).
    pub(super) fn person_settlement(batch: &ImportLedgerLine) -> Self {
        Self {
            record_type: StatusKind::PersonSettlement,
            batch_id: batch.batch_id.clone(),
            batch_sha256: batch.sha256.clone(),
            status: PERSON_SETTLEMENT_STATUS.into(),
            response: None,
            native_request_sha256: None,
            native_remote_id: None,
            native_remote_ids: None,
            pre_post_voucher_mark: None,
            bindings: None,
            binding_refusal: None,
            proof: None,
            release_seen: None,
            mark_read: None,
            records_before_read: None,
            reviewed_binding: None,
        }
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
            release_seen: None,
            mark_read: None,
            records_before_read: None,
            reviewed_binding: None,
        }
    }
}

impl StatusRecord {
    /// A verification's status record, naming the proof pair it saved: the
    /// only verification record this build writes (#911).
    pub(super) fn verified(batch: &ImportLedgerLine, proof: ProofName) -> Self {
        Self {
            proof: Some(proof),
            ..Self::verification(batch)
        }
    }

    fn verification(batch: &ImportLedgerLine) -> Self {
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
            release_seen: None,
            mark_read: None,
            records_before_read: None,
            reviewed_binding: None,
        }
    }
}

/// A verification record that names no proof, as an older build wrote one.
#[cfg(test)]
impl From<&ImportLedgerLine> for StatusRecord {
    fn from(batch: &ImportLedgerLine) -> Self {
        Self::verification(batch)
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
    /// Of `unsettled`, the batches with a response whose binding to their own
    /// vouchers was refused: matched by content only, they stay unsettled for
    /// good (#1039).
    pub(super) unsettled_binding_refused: usize,
    /// Of `unsettled`, the batches a person recorded checking in Tally while
    /// they were matched by content only, with the review standing (#1039).
    /// They count in no other part of `unsettled`: still unsettled, never
    /// verified, and their rows stay held.
    pub(super) unsettled_reviewed: usize,
    /// Batches with a dispatch intent that a person settled after Tally
    /// answered that their post created none of their vouchers, with the
    /// settlement standing (#1709). They are not in `unsettled`, and their
    /// rows stay held.
    pub(super) settled_by_person: usize,
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
        binding_refused: bool,
    }
    let mut batches: BTreeMap<String, Progress> = BTreeMap::new();
    let mut reviews = ContentReviews::default();
    let mut settlements = PersonSettlements::default();
    let mut rollbacks = RollbackFacts::empty();
    scan_records(reader, |record, VerificationGeneration(at)| {
        reviews.feed(&record);
        settlements.feed(&record);
        rollbacks.feed(&record, at);
        match record {
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
                    StatusKind::PostSpanVerdict => {
                        progress.binding_refused = update.binding_refusal.is_some();
                    }
                    StatusKind::VerificationStatus
                    | StatusKind::StopRelease
                    | StatusKind::RollbackSeen
                    | StatusKind::ContentReview
                    | StatusKind::PersonSettlement => {}
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
        }
    })?;
    let covered = rollbacks.covered();
    let reviewed = reviews.standing(&covered);
    let settled = settlements.standing(&covered);
    let sent_or_found = batches
        .values()
        .filter(|progress| progress.dispatched || progress.found)
        .count();
    let open = batches
        .iter()
        .filter(|(_, progress)| progress.dispatched && !(progress.responded && progress.verified));
    // A batch a person settled leaves `unsettled` and every part of it.
    let unsettled = open
        .clone()
        .filter(|(batch_id, _)| !settled.contains(*batch_id));
    // A batch a person reviewed counts in its own bucket and in no other
    // part of `unsettled`.
    let unreviewed = unsettled
        .clone()
        .filter(|(batch_id, _)| !reviewed.contains_key(*batch_id))
        .map(|(_, progress)| progress);
    Ok(Settlement {
        batches: batches.len(),
        sent_or_found,
        no_dispatch_never_verified: batches.len() - sent_or_found,
        unsettled_no_response: unreviewed
            .clone()
            .filter(|progress| !progress.responded)
            .count(),
        unsettled_binding_refused: unreviewed
            .clone()
            .filter(|progress| progress.responded && progress.binding_refused)
            .count(),
        unsettled_reviewed: unsettled.clone().count() - unreviewed.count(),
        settled_by_person: open.count() - unsettled.clone().count(),
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

/// An invoice's date, filed type and entries, as two saved invoices are
/// compared.
enum InvoiceFigures {
    /// Not an invoice: the guard reads nothing of it.
    NotInvoice,
    /// An invoice whose amount cannot be read (its date is a `TallyDate`, so
    /// always readable). The guard never takes this for "no figures": on a
    /// saved batch it counts as a match.
    Unreadable,
    Of((String, String, Vec<String>)),
}

fn invoice_figures(voucher: &ImportVoucher) -> InvoiceFigures {
    if !voucher.voucher_type.is_invoice() {
        return InvoiceFigures::NotInvoice;
    }
    let mut canonical = voucher.clone();
    for entry in &mut canonical.entries {
        match super::verification::canonical_verification_amount(&entry.amount) {
            Ok(amount) => entry.amount = amount,
            Err(_) => return InvoiceFigures::Unreadable,
        }
    }
    InvoiceFigures::Of((
        voucher.date.as_str().to_string(),
        voucher.filed_type_name().to_string(),
        super::verification::expected_entry_fingerprint(&canonical),
    ))
}

/// The id of another batch of the same company that Bridge sent to Tally (a
/// dispatch intent), whose latest status is not a verified post, and that holds an
/// invoice with the date, filed type and entries of `batch`'s invoice; the
/// first in id order. A post whose answer was lost never binds, and such a
/// batch may be this invoice under the number it was first given, so while it
/// stands the invoice is matched by its figures as well as its number
/// (`InvoiceIdentity::ByNumberOrFigures`). A batch that was only built, or one
/// whose latest status is `posted_verified`, does not count: two invoices with
/// the same figures are otherwise two documents. A batch with no invoice reads
/// nothing here. It fails closed: an invoice of the batch itself that cannot be
/// read is an error, and one of a saved batch that cannot be read counts as a
/// twin.
pub(super) fn unsettled_invoice_twin(
    reader: impl BufRead,
    batch: &ImportLedgerLine,
) -> Result<Option<String>, String> {
    let mut wanted = Vec::new();
    for figures in batch.vouchers.iter().map(invoice_figures) {
        match figures {
            InvoiceFigures::NotInvoice => {}
            InvoiceFigures::Unreadable => return Err("import_invoice_figures_unreadable".into()),
            InvoiceFigures::Of(figures) => wanted.push(figures),
        }
    }
    if wanted.is_empty() {
        return Ok(None);
    }
    let mut holds_a_twin = BTreeSet::new();
    let mut sent = BTreeSet::new();
    // A batch is settled when its LATEST status is `posted_verified`, as
    // `settlement` takes it: a dispatch intent after a hand import's
    // verification makes it unverified again.
    let mut verified = BTreeSet::new();
    scan_records(reader, |record, _| match record {
        Record::Batch(other) if other.batch_id != batch.batch_id => {
            let twin = other.company_guid.eq_ignore_ascii_case(&batch.company_guid)
                && other
                    .vouchers
                    .iter()
                    .map(invoice_figures)
                    .any(|figures| match figures {
                        InvoiceFigures::NotInvoice => false,
                        InvoiceFigures::Unreadable => true,
                        InvoiceFigures::Of(figures) => wanted.contains(&figures),
                    });
            if twin {
                holds_a_twin.insert(other.batch_id.clone());
            } else {
                holds_a_twin.remove(&other.batch_id);
            }
            if other.status == "posted_verified" {
                verified.insert(other.batch_id.clone());
            } else {
                verified.remove(&other.batch_id);
            }
        }
        Record::Status(update) => {
            if matches!(update.record_type, StatusKind::DispatchIntent) {
                sent.insert(update.batch_id.clone());
            }
            if update.sets_status() {
                if update.status == "posted_verified" {
                    verified.insert(update.batch_id.clone());
                } else {
                    verified.remove(&update.batch_id);
                }
            }
        }
        Record::Batch(_) => {}
    })?;
    Ok(holds_a_twin
        .intersection(&sent)
        .find(|id| !verified.contains(*id))
        .cloned())
}

/// What stands as the control for a build's "this invoice number is not in
/// use" read, from the journal of one company: a known invoice to read for
/// alongside it, or why there is none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum NumberControl {
    /// The newest invoice of a batch whose LATEST status is `posted_verified`,
    /// preferring one dated inside `year`: its batch, its number and its date.
    Known {
        batch_id: String,
        number: String,
        date: String,
    },
    /// No invoice of this company stands as sent: none was sent, or every one
    /// sent was released by a person.
    NoneStanding,
    /// One stands as sent and none is verified posted now (an answer lost, a
    /// post declined, or a later verification that no longer finds it).
    NoneVerified,
}

/// The control the journal offers for an invoice number read of this
/// company (`NumberControl`). A batch is verified while its latest status is
/// `posted_verified`, as `settlement` takes it; a dispatch intent makes it
/// sent. A person's release makes it neither until it next reads verified:
/// whatever the read at the release saw, it did not show that the number read
/// works on this Tally. An invoice whose number or date cannot be read is not a
/// control.
pub(super) fn invoice_number_control(
    reader: impl BufRead,
    company_guid: &str,
    (from, to): (&str, &str),
) -> Result<NumberControl, String> {
    // batch id -> (journal order, the batch's invoices as (date, number))
    let mut invoices: BTreeMap<String, (usize, Vec<(String, String)>)> = BTreeMap::new();
    let mut sent = BTreeSet::new();
    let mut verified = BTreeSet::new();
    // Batches a person released, while the release stands.
    let mut released = BTreeSet::new();
    let mut order = 0usize;
    scan_records(reader, |record, _| match record {
        Record::Batch(batch) => {
            order += 1;
            let held: Vec<(String, String)> =
                if batch.company_guid.eq_ignore_ascii_case(company_guid) {
                    batch
                        .vouchers
                        .iter()
                        .filter(|voucher| voucher.voucher_type.is_invoice())
                        .filter_map(|voucher| {
                            let date = voucher.date.as_str().to_string();
                            Some((date, voucher.voucher_number.clone()?))
                        })
                        .collect()
                } else {
                    Vec::new()
                };
            if held.is_empty() {
                invoices.remove(&batch.batch_id);
            } else {
                invoices.insert(batch.batch_id.clone(), (order, held));
            }
            if batch.status == "posted_verified" {
                verified.insert(batch.batch_id.clone());
            } else {
                verified.remove(&batch.batch_id);
            }
        }
        Record::Status(update) => {
            if matches!(update.record_type, StatusKind::DispatchIntent) {
                sent.insert(update.batch_id.clone());
            }
            if matches!(update.record_type, StatusKind::StopRelease) {
                released.insert(update.batch_id.clone());
            }
            if update.sets_status() {
                if update.status == "posted_verified" {
                    verified.insert(update.batch_id.clone());
                    released.remove(&update.batch_id);
                } else {
                    verified.remove(&update.batch_id);
                }
            }
        }
    })?;
    let known = invoices
        .iter()
        .filter(|(id, _)| verified.contains(*id))
        .flat_map(|(id, (order, held))| {
            held.iter()
                .map(move |invoice| (id.as_str(), *order, invoice))
        })
        .max_by_key(|(_, order, (date, _))| {
            (
                date.as_str() >= from && date.as_str() <= to,
                date.clone(),
                *order,
            )
        });
    Ok(match known {
        Some((batch_id, _, (date, number))) => NumberControl::Known {
            batch_id: batch_id.to_string(),
            number: number.clone(),
            date: date.clone(),
        },
        None if invoices
            .keys()
            .any(|id| sent.contains(id) && !released.contains(id)) =>
        {
            NumberControl::NoneVerified
        }
        None => NumberControl::NoneStanding,
    })
}

/// The ids of the batches of this company that stop every further Sales post
/// (slice 4, ADR 0004), in id order: each one holding an invoice that was sent to Tally (it has a
/// dispatch intent), whose LATEST status is not `posted_verified`, and that no
/// person has released since it last read verified.
///
/// It reads what the journal holds and fails closed: a batch counts when ANY of
/// its vouchers is an invoice, whether or not its number or date can be read.
/// A batch that was only built, or whose approval a person declined, was never
/// sent and does not stop anything; a lost answer, a post Tally declined and a
/// readback that does not match all do. A later `posted_verified` lifts the
/// stop by itself and voids a release made before it, so a divergence found
/// after that is a new stop.
pub(super) fn invoice_stops(
    reader: impl BufRead,
    company_guid: &str,
) -> Result<Vec<String>, String> {
    let mut holds_invoice: BTreeSet<String> = BTreeSet::new();
    let mut sent = BTreeSet::new();
    let mut verified = BTreeSet::new();
    let mut released = BTreeSet::new();
    scan_records(reader, |record, _| match record {
        Record::Batch(batch) => {
            if batch.company_guid.eq_ignore_ascii_case(company_guid)
                && batch
                    .vouchers
                    .iter()
                    .any(|voucher| voucher.voucher_type.is_invoice())
            {
                holds_invoice.insert(batch.batch_id.clone());
            } else {
                holds_invoice.remove(&batch.batch_id);
            }
            if batch.status == "posted_verified" {
                verified.insert(batch.batch_id.clone());
            } else {
                verified.remove(&batch.batch_id);
            }
        }
        Record::Status(update) => {
            match update.record_type {
                StatusKind::DispatchIntent => {
                    sent.insert(update.batch_id.clone());
                }
                StatusKind::StopRelease => {
                    released.insert(update.batch_id.clone());
                }
                _ => {}
            }
            if update.sets_status() {
                if update.status == "posted_verified" {
                    verified.insert(update.batch_id.clone());
                    released.remove(&update.batch_id);
                } else {
                    verified.remove(&update.batch_id);
                }
            }
        }
    })?;
    Ok(holds_invoice
        .into_iter()
        .filter(|id| sent.contains(id) && !verified.contains(id) && !released.contains(id))
        .collect())
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

/// The first batch, in id order, that holds as a voucher some of `rows` and
/// either belongs to the company and was sent or found posted, whatever its
/// lineage, or belongs to `own_lineage` at any status; with the rows of `rows`
/// it holds. A build that skips such a row cannot take it out of Tally. It reads
/// the journal alone: the transfer view is not consulted.
pub(super) fn rows_held_as_vouchers(
    reader: impl BufRead,
    company_guid: &str,
    own_lineage: Option<&str>,
    rows: &BTreeSet<String>,
) -> Result<Option<(String, BTreeSet<String>)>, String> {
    // each batch's latest full record decides what it holds
    let mut held: BTreeMap<String, (bool, BTreeSet<String>)> = BTreeMap::new();
    let mut sent_or_found = BTreeSet::new();
    scan_records(reader, |record, _| match record {
        Record::Batch(batch) => {
            if batch.company_guid.eq_ignore_ascii_case(company_guid) {
                let mine = batch
                    .vouchers
                    .iter()
                    .filter(|voucher| rows.contains(&voucher.bridge_txn_id))
                    .map(|voucher| voucher.bridge_txn_id.clone())
                    .collect();
                let own = own_lineage == Some(batch.identity_batch_id());
                held.insert(batch.batch_id.clone(), (own, mine));
            }
            if batch.status == "posted_verified" {
                sent_or_found.insert(batch.batch_id.clone());
            }
        }
        Record::Status(update) => {
            if matches!(update.record_type, StatusKind::DispatchIntent)
                || update.status == "posted_verified"
            {
                sent_or_found.insert(update.batch_id.clone());
            }
        }
    })?;
    Ok(held
        .into_iter()
        .find(|(id, (own, mine))| !mine.is_empty() && (*own || sent_or_found.contains(id)))
        .map(|(id, (_, mine))| (id, mine)))
}

/// The facts the transfer pairing view takes for one company: the journal
/// (every record admitted, as for every other read) and the skip-only file,
/// with a build about to be written, if any, read as the newest record.
///
/// `subject` is a batch already in the journal that is being judged again (the
/// one about to be sent): every record of it is left out, so the journal reads
/// as it did when that batch was built, and it is given as `candidate`.
pub(super) fn transfer_facts(
    journal: impl BufRead,
    skips_file: impl BufRead,
    company_guid: &str,
    candidate: Option<statement_facts::Candidate>,
    subject: Option<&str>,
) -> Result<statement_facts::TransferFacts, statement_facts::TransferFactsError> {
    use statement_facts::{BuildLine, Candidate, JournalEvent, TransferFactsError};
    let mut events = Vec::new();
    // The same pass gathers what a recorded rollback covers (#1050), so the
    // facts never call a batch found that a restore may have emptied. It reads
    // every record, the subject's too, since a cover depends on positions.
    let mut rollbacks = RollbackFacts::empty();
    scan_records(journal, |record, VerificationGeneration(at)| {
        rollbacks.feed(&record, at);
        match record {
            Record::Batch(batch) if Some(batch.batch_id.as_str()) == subject => {}
            Record::Batch(batch) => {
                events.push(JournalEvent::Build(Box::new(BuildLine::of(&batch))));
            }
            Record::Status(update) if Some(update.batch_id.as_str()) == subject => {}
            Record::Status(update) if update.sets_status() => events.push(JournalEvent::Status {
                sent: matches!(update.record_type, StatusKind::DispatchIntent),
                batch_id: update.batch_id,
                status: update.status,
            }),
            Record::Status(_) => {}
        }
    })
    .map_err(|code| TransferFactsError::JournalUnreadable { code })?;
    let covered = rollbacks.covered();
    let mut skip_records = statement_facts::read_skip_records(skips_file)
        .map_err(|code| TransferFactsError::SkipFileUnreadable { code })?;
    match candidate {
        Some(Candidate::Build(line)) => events.push(JournalEvent::Build(line)),
        Some(Candidate::SkipOnly(record)) => skip_records.push(*record),
        None => {}
    }
    statement_facts::build_transfer_facts(events, skip_records, company_guid, &covered)
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
    // Batches whose latest status is `posted_verified`: a stop release is
    // admitted only while its batch is not.
    let mut verified: BTreeSet<String> = BTreeSet::new();
    // Batches whose journaled span refusal a content review can cover (any
    // code but `span_identity_reused`), and those with a review admitted since
    // the batch last read verified or was answered: a second is refused while
    // one is in this set (a rollback cover does not clear it).
    let mut refused: BTreeSet<String> = BTreeSet::new();
    let mut reviewed: BTreeSet<String> = BTreeSet::new();
    // Batches whose latest answer with an outcome reported none of their
    // vouchers created (#1709), those bound, and those with a settlement
    // admitted since the batch last read verified or was answered: a second
    // is refused while one is in this set.
    let mut not_created: BTreeSet<String> = BTreeSet::new();
    let mut bound: BTreeSet<String> = BTreeSet::new();
    let mut settled: BTreeSet<String> = BTreeSet::new();
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
            // A stop release follows its batch's dispatch intent, is made
            // only while the batch is not verified, carries nothing else, and
            // records what the release read saw.
            if matches!(update.record_type, StatusKind::StopRelease) {
                if !dispatched.contains_key(&update.batch_id)
                    || verified.contains(&update.batch_id)
                    || update.status != STOP_RELEASE_STATUS
                    || update.release_seen.is_none()
                    || update.response.is_some()
                    || update.native_request_sha256.is_some()
                    || update.native_remote_id.is_some()
                    || update.native_remote_ids.is_some()
                    || update.pre_post_voucher_mark.is_some()
                    || update.bindings.is_some()
                    || update.binding_refusal.is_some()
                {
                    return Err("import_ledger_invalid".into());
                }
            } else if update.release_seen.is_some() {
                return Err("import_ledger_invalid".into());
            }
            // A content review follows a native dispatch intent that recorded
            // a pre-POST mark, is made only while the batch is not verified and
            // its content-only state holds (a refusal it can cover, or no
            // outcome), carries nothing else, and is refused while another
            // stands: one admitted since the batch last read verified or was
            // answered. A rollback cover does not clear that: the batch then
            // reads rolled back and is never reviewed again.
            if matches!(update.record_type, StatusKind::ContentReview) {
                let state_holds = match update.reviewed_binding {
                    Some(ReviewedBinding::Refused) => refused.contains(&update.batch_id),
                    // A verdict follows an answer with an outcome, so none
                    // stands without one.
                    Some(ReviewedBinding::ResponseNotRecorded) => {
                        !responded.contains(&update.batch_id)
                    }
                    None => false,
                };
                if !marked.contains(&update.batch_id)
                    || verified.contains(&update.batch_id)
                    || update.status != CONTENT_REVIEW_STATUS
                    || !state_holds
                    || update.response.is_some()
                    || update.native_request_sha256.is_some()
                    || update.native_remote_id.is_some()
                    || update.native_remote_ids.is_some()
                    || update.pre_post_voucher_mark.is_some()
                    || update.bindings.is_some()
                    || update.binding_refusal.is_some()
                    || !reviewed.insert(update.batch_id.clone())
                {
                    return Err("import_ledger_invalid".into());
                }
            } else if update.reviewed_binding.is_some() {
                return Err("import_ledger_invalid".into());
            }
            // A person settlement follows a native dispatch intent that
            // recorded a pre-POST mark and an answer that reported none of the
            // batch's vouchers created, is made only while the batch is not
            // verified and not bound, and is refused while another stands.
            // Every field of another record kind is refused by that kind's own
            // rule above, so it carries nothing else.
            if matches!(update.record_type, StatusKind::PersonSettlement)
                && (!marked.contains(&update.batch_id)
                    || !not_created.contains(&update.batch_id)
                    || verified.contains(&update.batch_id)
                    || bound.contains(&update.batch_id)
                    || update.status != PERSON_SETTLEMENT_STATUS
                    || !settled.insert(update.batch_id.clone()))
            {
                return Err("import_ledger_invalid".into());
            }
            // A rollback-seen record follows a native dispatch intent that
            // recorded a pre-POST mark, names a count of records it had read no
            // larger than its own position, and carries nothing else. Its mark
            // is not checked against a floor: the floor a writer used may rest
            // on a mark read after the post, which the journal does not hold,
            // and a record a writer emits must never be refused.
            if matches!(update.record_type, StatusKind::RollbackSeen) {
                let within = update
                    .records_before_read
                    .and_then(|count| usize::try_from(count).ok())
                    .is_some_and(|count| count <= ordinal);
                if !marked.contains(&update.batch_id)
                    || update.status != ROLLBACK_SEEN_STATUS
                    || update.mark_read.is_none()
                    || !within
                    || update.response.is_some()
                    || update.native_request_sha256.is_some()
                    || update.native_remote_id.is_some()
                    || update.native_remote_ids.is_some()
                    || update.pre_post_voucher_mark.is_some()
                    || update.bindings.is_some()
                    || update.binding_refusal.is_some()
                {
                    return Err("import_ledger_invalid".into());
                }
            } else if update.mark_read.is_some() || update.records_before_read.is_some() {
                return Err("import_ledger_invalid".into());
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
                if update
                    .binding_refusal
                    .as_ref()
                    .is_some_and(|code| code != IDENTITY_REUSED)
                {
                    refused.insert(update.batch_id.clone());
                }
                // A bound batch is never settled again, so its settlement
                // needs no clearing here; the reader voids it.
                if update.bindings.is_some() {
                    bound.insert(update.batch_id.clone());
                }
            } else if update.bindings.is_some() || update.binding_refusal.is_some() {
                return Err("import_ledger_invalid".into());
            }
            if matches!(update.record_type, StatusKind::DispatchResponse) {
                let request_hash = dispatched
                    .get(&update.batch_id)
                    .ok_or("import_ledger_invalid")?;
                let response = update.response.as_ref().ok_or("import_ledger_invalid")?;
                if let Some(outcome) = &response.outcome {
                    responded.insert(update.batch_id.clone());
                    reviewed.remove(&update.batch_id);
                    settled.remove(&update.batch_id);
                    if u64::try_from(*voucher_count).is_ok_and(|sent| {
                        super::verification::answer_reported_none_created(outcome.counters(), sent)
                    }) {
                        not_created.insert(update.batch_id.clone());
                    } else {
                        not_created.remove(&update.batch_id);
                    }
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
            if update.sets_status() {
                if update.status == "posted_verified" {
                    verified.insert(update.batch_id.clone());
                    reviewed.remove(&update.batch_id);
                    settled.remove(&update.batch_id);
                } else {
                    verified.remove(&update.batch_id);
                }
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
            if batch.status == "posted_verified" {
                verified.insert(batch.batch_id.clone());
                reviewed.remove(&batch.batch_id);
                settled.remove(&batch.batch_id);
            } else {
                verified.remove(&batch.batch_id);
            }
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

/// The content reviews standing in the journal (#1039), by batch: what each
/// reviewed. A later `posted_verified` of the batch, or a later answer with an
/// outcome, voids a review; so does any rollback cover of the batch, which
/// [`Self::standing`] takes from the caller's [`RollbackFacts`] of the same scan.
#[derive(Default)]
struct ContentReviews(BTreeMap<String, ReviewedBinding>);

impl ContentReviews {
    fn feed(&mut self, record: &Record) {
        match record {
            Record::Batch(batch) => {
                if batch.status == "posted_verified" {
                    self.0.remove(&batch.batch_id);
                }
            }
            Record::Status(update) => {
                if let (StatusKind::ContentReview, Some(reviewed)) =
                    (&update.record_type, update.reviewed_binding)
                {
                    self.0.insert(update.batch_id.clone(), reviewed);
                }
                let answered = update
                    .response
                    .as_ref()
                    .is_some_and(|response| response.outcome.is_some());
                if answered || (update.sets_status() && update.status == "posted_verified") {
                    self.0.remove(&update.batch_id);
                }
            }
        }
    }

    /// The reviews that stand: those no rollback in `covered` voids.
    fn standing(self, covered: &BTreeSet<String>) -> BTreeMap<String, ReviewedBinding> {
        self.0
            .into_iter()
            .filter(|(batch_id, _)| !covered.contains(batch_id))
            .collect()
    }
}

/// The content review of `batch_id` that stands now, if any, with what it
/// reviewed, the whole journal admitted on the way (#1039).
pub(super) fn content_review(
    reader: impl BufRead,
    batch_id: &str,
) -> Result<Option<ReviewedBinding>, String> {
    let mut reviews = ContentReviews::default();
    let mut rollbacks = RollbackFacts::empty();
    scan_records(reader, |record, VerificationGeneration(at)| {
        reviews.feed(&record);
        rollbacks.feed(&record, at);
    })?;
    Ok(reviews.standing(&rollbacks.covered()).remove(batch_id))
}

/// The review `record` would stand as, were it appended to the journal `reader`
/// holds: `None` when the journal would hold no standing review of its batch
/// after it (a rollback covers the batch), and the journal's refusal when it
/// would not admit the record (the batch's state is not content-only, it reads
/// verified, or a review of it already stands). The writer appends only what
/// this admits, so a record it writes never makes the journal unreadable.
pub(super) fn content_review_after(
    reader: impl BufRead,
    record: &StatusRecord,
) -> Result<Option<ReviewedBinding>, String> {
    let mut appended =
        serde_json::to_vec(record).map_err(|_| "import_ledger_serialization_failed".to_string())?;
    appended.push(b'\n');
    content_review(
        reader.chain(std::io::Cursor::new(appended)),
        &record.batch_id,
    )
}

/// The person settlements standing in the journal (#1709), by batch. A later
/// `posted_verified` of the batch, a later answer with an outcome, or a
/// binding voids a settlement; so does any rollback cover of the batch, which
/// [`Self::standing`] takes from the caller's [`RollbackFacts`] of the same scan.
#[derive(Default)]
struct PersonSettlements(BTreeSet<String>);

impl PersonSettlements {
    fn feed(&mut self, record: &Record) {
        match record {
            Record::Batch(batch) => {
                if batch.status == "posted_verified" {
                    self.0.remove(&batch.batch_id);
                }
            }
            Record::Status(update) => {
                if matches!(update.record_type, StatusKind::PersonSettlement) {
                    self.0.insert(update.batch_id.clone());
                }
                let answered = update
                    .response
                    .as_ref()
                    .is_some_and(|response| response.outcome.is_some());
                if answered
                    || update.bindings.is_some()
                    || (update.sets_status() && update.status == "posted_verified")
                {
                    self.0.remove(&update.batch_id);
                }
            }
        }
    }

    /// The settlements that stand: those no rollback in `covered` voids.
    fn standing(self, covered: &BTreeSet<String>) -> BTreeSet<String> {
        self.0.difference(covered).cloned().collect()
    }
}

/// Whether a person settlement of `batch_id` stands now, the whole journal
/// admitted on the way (#1709).
pub(super) fn person_settlement(reader: impl BufRead, batch_id: &str) -> Result<bool, String> {
    let mut settlements = PersonSettlements::default();
    let mut rollbacks = RollbackFacts::empty();
    scan_records(reader, |record, VerificationGeneration(at)| {
        settlements.feed(&record);
        rollbacks.feed(&record, at);
    })?;
    Ok(settlements
        .standing(&rollbacks.covered())
        .contains(batch_id))
}

/// Whether `record` would stand, were it appended to the journal `reader`
/// holds: `false` when a rollback covers its batch, and the journal's refusal
/// when it would not admit the record. The writer appends only what this
/// admits, so a record it writes never makes the journal unreadable.
pub(super) fn person_settlement_after(
    reader: impl BufRead,
    record: &StatusRecord,
) -> Result<bool, String> {
    let mut appended =
        serde_json::to_vec(record).map_err(|_| "import_ledger_serialization_failed".to_string())?;
    appended.push(b'\n');
    person_settlement(
        reader.chain(std::io::Cursor::new(appended)),
        &record.batch_id,
    )
}

/// What the rollback-seen records say about one batch (#1050), from one scan.
pub(super) struct RollbackScan {
    /// How many records the journal held when it was read: the count a
    /// rollback-seen record carries when the mark it names is read after this
    /// scan.
    pub(super) records: u64,
    /// The record that covers the batch, if any (the lowest mark among them).
    pub(super) covered_by: Option<RollbackCover>,
}

/// A rollback-seen record that covers a batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct RollbackCover {
    pub(super) mark_read: u64,
    /// The batch whose verification read the mark.
    pub(super) seen_by: String,
}

/// What one batch had journaled, and where, for judging a record's cover.
#[derive(Default)]
struct NativePost {
    intent_at: Option<usize>,
    pre_post_voucher_mark: Option<u64>,
    /// Where its answer with a parsed outcome was recorded, and the vouchers
    /// that answer proves landed ([`landed_count`]).
    created_at: Option<(usize, u64)>,
    /// Where its binding was recorded, and how many vouchers it bound.
    bound_at: Option<(usize, u64)>,
}

impl NativePost {
    /// The floor the post had proved within the first `prefix` records: the
    /// vouchers it bound (the bind read exactly those AlterIDs above the
    /// pre-POST mark), else the vouchers Tally's recorded answer counted
    /// created (a recorded answer means they had landed, unless it reported
    /// `STATUS=0`: [`landed_count`]), else the pre-POST
    /// mark alone. `None` for a post not dispatched within the prefix or with
    /// no pre-POST mark.
    fn floor_within(&self, prefix: usize) -> Option<u64> {
        let before = self.pre_post_voucher_mark?;
        if self.intent_at.is_none_or(|at| at >= prefix) {
            return None;
        }
        let proved = match (self.bound_at, self.created_at) {
            (Some((at, bound)), _) if at < prefix => bound,
            (_, Some((at, created))) if at < prefix => created,
            _ => 0,
        };
        Some(before.saturating_add(proved))
    }
}

/// The vouchers a recorded answer proves landed: its `CREATED`, unless it
/// reported `STATUS=0`, which failed whatever its counters say
/// (IMPLEMENTATION_GUIDE.md §3.2, I6; #2035).
pub(super) fn landed_count(outcome: &bridge_tally_protocol::TallyImportOutcome) -> u64 {
    if outcome.application_status() == bridge_tally_protocol::TallyImportApplicationStatus::Failure
    {
        0
    } else {
        outcome.counters().created
    }
}

/// The rollback-seen records that cover `batch_id`, the whole journal
/// admitted on the way. A record covers a batch of the same company (all four
/// fields) and endpoint when, within the records the record's verification
/// had read, the batch's dispatch intent was recorded and the floor it had
/// proved there is above the mark the record read. Only facts journaled before
/// that read count, so a post whose answer or binding came later is judged on
/// its pre-POST mark alone, which a mark read after it cannot fall below.
pub(super) fn rollback_cover(reader: impl BufRead, batch_id: &str) -> Result<RollbackScan, String> {
    let facts = RollbackFacts::scan(reader)?;
    Ok(RollbackScan {
        records: facts.records as u64,
        covered_by: facts.cover_of(batch_id),
    })
}

/// Whether a rollback-seen record that `seen_by`'s verification would write,
/// for a mark read at `mark_read` after reading `records_before_read` records,
/// covers a native batch of its company and address that no record on file
/// covers (#1050). Only such a record is written: a check of a batch already
/// covered still records a second restore that reaches a batch posted since,
/// and a record that would cover nothing new is not written.
pub(super) fn rollback_record_extends(
    reader: impl BufRead,
    seen_by: &str,
    mark_read: u64,
    records_before_read: u64,
) -> Result<bool, String> {
    let facts = RollbackFacts::scan(reader)?;
    let extends = facts
        .newly_covered(seen_by, mark_read, records_before_read)
        .next()
        .is_some();
    Ok(extends)
}

/// The batch a rollback-seen record written for a post's own reading names
/// (#1050, seen at the post): the first native batch, in journal order, of
/// `poster`'s company and address that such a record would cover and no record
/// on file covers. The post that read the mark has no dispatch intent yet, so
/// a record may not name it; it names a batch it covers, which admission
/// accepts. `None` when the record would cover nothing new.
pub(super) fn first_newly_covered(
    reader: impl BufRead,
    poster: &str,
    mark_read: u64,
    records_before_read: u64,
) -> Result<Option<String>, String> {
    let facts = RollbackFacts::scan(reader)?;
    let first = facts
        .newly_covered(poster, mark_read, records_before_read)
        .next()
        .cloned();
    Ok(first)
}

/// The native batches the journal's last record covers when it is a
/// rollback-seen record, in journal order, the batch it names included, for
/// the answer that just wrote it to name: their own status is unchanged until
/// each is verified again. Empty when the last record is another kind. One
/// record, not every record its batch names: a record written for a post's
/// reading names a batch it covers, and that batch may write its own later.
pub(super) fn covered_by_last_record(reader: impl BufRead) -> Result<Vec<String>, String> {
    let facts = RollbackFacts::scan(reader)?;
    let Some(record) = facts
        .seen
        .last()
        .filter(|record| record.at + 1 == facts.records)
    else {
        return Ok(Vec::new());
    };
    let Some(place) = facts.place_of(&record.by) else {
        return Ok(Vec::new());
    };
    Ok(facts
        .order
        .iter()
        .filter(|batch_id| facts.place_of(batch_id) == Some(place))
        .filter(|batch_id| {
            facts.posts.get(*batch_id).is_some_and(|post| {
                post.floor_within(record.prefix)
                    .is_some_and(|floor| floor > record.mark_read)
            })
        })
        .cloned()
        .collect())
}

/// The batch's company and Tally address, as its full record names them.
type Place = (Option<ImportCompanyTuple>, Option<String>);

/// One rollback-seen record: the batch it names, the mark read, how many
/// records the reading had read before the mark, and its own position.
struct SeenRecord {
    by: String,
    mark_read: u64,
    prefix: usize,
    at: usize,
}

/// What one scan of the journal says about native posts and rollback-seen
/// records, for [`rollback_cover`] and [`covered_by_last_record`].
struct RollbackFacts {
    places: BTreeMap<String, Place>,
    posts: BTreeMap<String, NativePost>,
    /// Batch ids in the order their first full record appeared.
    order: Vec<String>,
    /// Each rollback-seen record, in journal order.
    seen: Vec<SeenRecord>,
    records: usize,
}

impl RollbackFacts {
    fn empty() -> Self {
        Self {
            places: BTreeMap::new(),
            posts: BTreeMap::new(),
            order: Vec::new(),
            seen: Vec::new(),
            records: 0,
        }
    }

    fn scan(reader: impl BufRead) -> Result<Self, String> {
        let mut facts = Self::empty();
        scan_records(reader, |record, VerificationGeneration(at)| {
            facts.feed(&record, at);
        })?;
        Ok(facts)
    }

    /// Take one admitted record, the `at`-th of the journal.
    fn feed(&mut self, record: &Record, at: usize) {
        let facts = self;
        facts.records = at + 1;
        match record {
            Record::Batch(batch) => {
                if !facts.places.contains_key(&batch.batch_id) {
                    facts.order.push(batch.batch_id.clone());
                }
                facts.places.insert(
                    batch.batch_id.clone(),
                    (batch.company.clone(), batch.endpoint_origin.clone()),
                );
            }
            Record::Status(update) => {
                let post = facts.posts.entry(update.batch_id.clone()).or_default();
                match update.record_type {
                    StatusKind::DispatchIntent => {
                        post.intent_at = Some(at);
                        post.pre_post_voucher_mark = update.pre_post_voucher_mark;
                    }
                    StatusKind::DispatchResponse => {
                        if let Some(outcome) = update
                            .response
                            .as_ref()
                            .and_then(|response| response.outcome.as_ref())
                        {
                            post.created_at = Some((at, landed_count(outcome)));
                        }
                    }
                    StatusKind::PostSpanVerdict => {
                        if let Some(bound) = &update.bindings {
                            post.bound_at = Some((at, bound.len() as u64));
                        }
                    }
                    StatusKind::RollbackSeen => {
                        if let (Some(mark), Some(count)) =
                            (update.mark_read, update.records_before_read)
                        {
                            // Admission bounds the count by the record's position.
                            let count = usize::try_from(count).unwrap_or(at);
                            facts.seen.push(SeenRecord {
                                by: update.batch_id.clone(),
                                mark_read: mark,
                                prefix: count,
                                at,
                            });
                        }
                    }
                    StatusKind::VerificationStatus
                    | StatusKind::StopRelease
                    | StatusKind::ContentReview
                    | StatusKind::PersonSettlement => {}
                }
            }
        }
    }

    /// Every batch some recorded rollback covers.
    fn covered(&self) -> BTreeSet<String> {
        self.order
            .iter()
            .filter(|batch_id| self.cover_of(batch_id).is_some())
            .cloned()
            .collect()
    }

    /// The record that covers `batch_id`, if any (the lowest mark among them):
    /// one written by a batch of the same place, within whose prefix the batch
    /// was dispatched and had proved a floor above the mark read.
    fn cover_of(&self, batch_id: &str) -> Option<RollbackCover> {
        let own_place = self.place_of(batch_id)?;
        let post = self.posts.get(batch_id)?;
        self.seen
            .iter()
            .filter(|record| {
                self.place_of(&record.by) == Some(own_place)
                    && post
                        .floor_within(record.prefix)
                        .is_some_and(|floor| floor > record.mark_read)
            })
            .min_by_key(|record| record.mark_read)
            .map(|record| RollbackCover {
                mark_read: record.mark_read,
                seen_by: record.by.clone(),
            })
    }

    /// The native batches of `reader`'s company and address, in journal order,
    /// that a record of `mark_read`, read after `records_before_read` records,
    /// would cover and that no record on file covers.
    fn newly_covered<'a>(
        &'a self,
        reader: &str,
        mark_read: u64,
        records_before_read: u64,
    ) -> impl Iterator<Item = &'a String> + 'a {
        let place = self.place_of(reader);
        let prefix = usize::try_from(records_before_read).unwrap_or(usize::MAX);
        self.order.iter().filter(move |batch_id| {
            place.is_some()
                && self.place_of(batch_id) == place
                && self
                    .posts
                    .get(*batch_id)
                    .and_then(|post| post.floor_within(prefix))
                    .is_some_and(|floor| floor > mark_read)
                && self.cover_of(batch_id).is_none()
        })
    }

    /// The batch's place, when its record names both a company and an address.
    fn place_of(&self, batch_id: &str) -> Option<&Place> {
        self.places
            .get(batch_id)
            .filter(|(company, endpoint)| company.is_some() && endpoint.is_some())
    }
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

/// Shared with the master journal, which bounds its records the same way.
pub(super) fn read_record(reader: &mut impl BufRead, line: &mut Vec<u8>) -> Result<bool, String> {
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
