use std::collections::{BTreeMap, BTreeSet};

use bridge_tally_core::report_tie_out::TieOutState;
use bridge_tally_core::{
    CanonicalPackWindow, CanonicalText, CapabilityPackId, CapabilityState, CoreAccountingBatch,
    Freshness, Gap, PackBatch, PackSchemaVersion, ProofManifest, ReadWindow,
    RunOutcome as CoreRunOutcome, SourceCountScope, SourceCountScopeDescriptor, SourceIdentity,
    SourceIdentityKind, SourceRecordEvidence, TallyDate,
    VerificationState as CoreVerificationState, PROOF_CONTRACT_VERSION,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::db::tally_mirror::{
    Confidence, ObservedRecordInput, RunOutcome, SourceIdentityInput, VerificationState,
};
use crate::warning_codes::WarningCode;

const MAX_SAFE_DRILL_DOWN_IDS: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonScope {
    Complete,
    Window,
    Unavailable,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EndProfileCheck {
    Passed,
    Mismatch,
    #[default]
    Unavailable,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceStabilityCheck {
    Passed,
    Mismatch,
    #[default]
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ObjectCountEvidence {
    pub scope: ComparisonScope,
    pub source_reported_count: Option<u64>,
    pub parsed_count: u64,
    pub accepted_count: u64,
    pub deduped_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "scope", rename_all = "snake_case")]
pub enum ExternalReferenceCatalog {
    Unavailable,
    Complete {
        company_ids: BTreeSet<String>,
        voucher_ids: BTreeSet<String>,
        ledger_ids: BTreeSet<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReconciliationMismatch {
    pub safe_reason_code: String,
    pub safe_record_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReportTieOutEvidence {
    pub source_identity: SourceIdentity,
    pub pack: CapabilityPackId,
    pub pack_schema_version: PackSchemaVersion,
    pub query_profile: CanonicalText,
    pub filters_sha256: CanonicalText,
    pub from_yyyymmdd: String,
    pub to_yyyymmdd: String,
    pub report_sha256: String,
    pub state: TieOutState,
    pub compared_ledger_count: u64,
    pub source_reported_count: u64,
    pub core_ledger_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct WindowEvidence {
    pub window_id: String,
    pub from_yyyymmdd: String,
    pub to_yyyymmdd: String,
    pub canonical_sha256: String,
    #[serde(default)]
    pub query_profile: String,
    #[serde(default)]
    pub filters_sha256: String,
    /// Complete only when every canonical record has connector-supplied raw
    /// provenance bound one-to-one. Unavailable is an explicit proof gap.
    #[serde(default = "unavailable_comparison_scope")]
    pub record_provenance_scope: ComparisonScope,
    pub source_count_scope: ComparisonScope,
    pub source_count: u64,
    pub parsed_count: u64,
    pub accepted_count: u64,
    pub deduped_count: u64,
    pub rejected_count: u64,
    pub duplicate_identity_count: u64,
    pub missing_identity_count: u64,
    pub out_of_range_count: u64,
    pub record_counts: BTreeMap<String, u64>,
    pub accepted_record_counts: BTreeMap<String, u64>,
    pub object_counts: BTreeMap<String, ObjectCountEvidence>,
    /// Commitment to the complete ordered membership stored in the encrypted
    /// mirror. The full identity map is hydrated only for reconciliation and
    /// is never embedded in durable snapshot-state JSON.
    #[serde(default)]
    pub record_set_sha256: Option<String>,
    #[serde(default, skip_serializing)]
    pub canonical_records: BTreeMap<String, String>,
    pub accounting_scope: ComparisonScope,
    /// Fine-grained accounting checks that could not be evaluated for this
    /// window. These codes are proof gaps, never warnings.
    #[serde(default)]
    pub accounting_gap_codes: BTreeSet<String>,
    pub report_tie_out_scope: ComparisonScope,
    #[serde(default)]
    pub report_tie_out: Option<ReportTieOutEvidence>,
    pub mismatches: Vec<ReconciliationMismatch>,
}

fn unavailable_comparison_scope() -> ComparisonScope {
    ComparisonScope::Unavailable
}

#[derive(Debug, Clone)]
pub struct CanonicalObservation {
    pub object_type: String,
    pub source_id: String,
    pub display_name: Option<String>,
    pub canonical_payload: Value,
    pub exact_decimals: BTreeMap<String, String>,
    pub canonical_sha256: String,
    pub provenance: Option<SourceRecordEvidence>,
}

impl CanonicalObservation {
    pub fn mirror_input(
        &self,
        batch_id: &str,
        observed_at_unix_ms: i64,
    ) -> Result<ObservedRecordInput, ReconciliationError> {
        let provenance = self
            .provenance
            .as_ref()
            .ok_or(ReconciliationError::RecordProvenanceUnavailable)?;
        let identity = SourceIdentityInput {
            guid: provenance
                .observed_identities
                .guid
                .as_ref()
                .map(|value| value.as_str().to_string()),
            remote_id: provenance
                .observed_identities
                .remote_id
                .as_ref()
                .map(|value| value.as_str().to_string()),
            master_id: provenance
                .observed_identities
                .master_id
                .as_ref()
                .map(|value| value.as_str().to_string()),
            fallback_fingerprint: (provenance.identity_kind == SourceIdentityKind::Fallback)
                .then(|| self.source_id.clone()),
            confidence: Some(
                if provenance.identity_kind == SourceIdentityKind::Fallback {
                    Confidence::Inferred
                } else {
                    Confidence::Observed
                },
            ),
        };
        Ok(ObservedRecordInput {
            batch_id: batch_id.to_string(),
            object_type: self.object_type.clone(),
            display_name: self.display_name.clone(),
            identity,
            observed_at_unix_ms,
            raw_source_sha256: provenance.raw_source_sha256.as_str().to_string(),
            canonical_sha256: self.canonical_sha256.clone(),
            canonical_payload: self.canonical_payload.clone(),
            exact_decimals: self.exact_decimals.clone(),
            observed_alter_id: provenance
                .alter_id
                .as_ref()
                .map(|alter_id| alter_id.as_str().to_string()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CanonicalWindow {
    pub observations: Vec<CanonicalObservation>,
    pub evidence: WindowEvidence,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ReconciliationError {
    #[error("the returned pack does not match the requested pack")]
    PackMismatch,
    #[error("canonical serialization failed")]
    Serialization,
    #[error("canonical typed pack validation failed")]
    InvalidTypedPack,
    #[error("source count evidence is invalid")]
    InvalidSourceCountEvidence,
    #[error("source count evidence does not match the immutable request scope")]
    SourceCountScopeMismatch,
    #[error("record provenance evidence does not bind to the canonical batch")]
    RecordEvidenceMismatch,
    #[error("record provenance is unavailable")]
    RecordProvenanceUnavailable,
    #[error("invalid snapshot proof input ({0})")]
    InvalidInput(&'static str),
}

#[derive(Debug, Clone)]
pub struct ReconciliationInput {
    pub batch_id: String,
    pub run_id: String,
    pub source_identity: SourceIdentity,
    pub pack: CapabilityPackId,
    pub pack_schema_version: PackSchemaVersion,
    pub started_at_unix_ms: i64,
    pub completed_at_unix_ms: i64,
    pub freshness_before: Freshness,
    pub freshness_target_seconds: i64,
    pub planned_window_ids: BTreeSet<String>,
    pub completed_windows: BTreeMap<String, WindowEvidence>,
    pub end_profile_check: EndProfileCheck,
    pub source_stability_check: SourceStabilityCheck,
    pub explicit_gap_codes: BTreeSet<String>,
    pub warning_codes: BTreeSet<WarningCode>,
}

#[derive(Debug, Clone)]
pub struct ReconciliationDecision {
    pub proof: ProofManifest,
    pub mirror_commit: CommitBatchInput,
}

/// Sealed commit authority. Production code can only obtain this value from
/// the fail-closed reconciliation builders in this module.
#[derive(Debug, Clone)]
pub struct CommitBatchInput {
    parts: CommitBatchParts,
}

#[derive(Debug, Clone)]
pub(crate) struct CommitBatchParts {
    pub batch_id: String,
    pub proof_contract_version: u16,
    pub outcome: RunOutcome,
    pub verification: VerificationState,
    pub completed_at_unix_ms: i64,
    pub record_counts_sha256: Option<String>,
    pub snapshot_sha256: Option<String>,
    pub expected_checkpoint_before: Option<String>,
    pub checkpoint_after: Option<String>,
    pub freshness_target_seconds: i64,
    pub gap_codes: Vec<String>,
    pub warning_codes: Vec<String>,
}

impl CommitBatchInput {
    pub(in crate::sync) fn reconciled(parts: CommitBatchParts) -> Self {
        Self { parts }
    }

    pub(in crate::sync) fn parts(&self) -> &CommitBatchParts {
        &self.parts
    }

    pub(in crate::sync) fn bind_expected_checkpoint(&mut self, checkpoint: Option<String>) {
        self.parts.expected_checkpoint_before = checkpoint;
    }

    pub(crate) fn into_parts(self) -> CommitBatchParts {
        self.parts
    }

    #[cfg(test)]
    pub(crate) fn test_only(parts: CommitBatchParts) -> Self {
        Self { parts }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalKind {
    Failed,
    Cancelled,
}

pub struct CanonicalWindowContext<'a> {
    pub requested_pack: CapabilityPackId,
    pub schema_version: PackSchemaVersion,
    pub source_identity: &'a SourceIdentity,
    pub query_profile: &'a CanonicalText,
    pub filters_sha256: &'a CanonicalText,
    pub window_id: &'a str,
    pub requested_window: &'a ReadWindow,
}

pub fn canonicalize_window(
    context: &CanonicalWindowContext<'_>,
    window: &CanonicalPackWindow,
) -> Result<CanonicalWindow, ReconciliationError> {
    window
        .validate_source_count_evidence()
        .map_err(|_| ReconciliationError::InvalidSourceCountEvidence)?;
    window
        .validate_record_evidence_binding()
        .map_err(|_| ReconciliationError::RecordEvidenceMismatch)?;
    if pack_id(&window.batch) != context.requested_pack {
        return Err(ReconciliationError::PackMismatch);
    }

    let PackBatch::CoreAccounting(core) = &window.batch;
    let mut canonical =
        canonicalize_core_window(context.window_id, context.requested_window, core)?;
    canonical.evidence.query_profile = context.query_profile.as_str().to_string();
    canonical.evidence.filters_sha256 = context.filters_sha256.as_str().to_string();
    bind_record_provenance(&mut canonical, window)?;
    bind_source_count_evidence(
        &mut canonical.evidence,
        window,
        context.source_identity,
        context.requested_pack,
        context.schema_version,
        context.query_profile,
        context.filters_sha256,
        context.requested_window,
    )?;
    Ok(canonical)
}

fn bind_record_provenance(
    canonical: &mut CanonicalWindow,
    window: &CanonicalPackWindow,
) -> Result<(), ReconciliationError> {
    let Some(record_evidence) = &window.record_evidence else {
        canonical.evidence.record_provenance_scope = ComparisonScope::Unavailable;
        return Ok(());
    };
    let mut evidence_by_record = record_evidence
        .iter()
        .cloned()
        .map(|evidence| {
            (
                (
                    evidence.object_type.as_str().to_string(),
                    evidence.source_id.as_str().to_string(),
                ),
                evidence,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut inferred_ids = Vec::new();
    for observation in &mut canonical.observations {
        observation.provenance = evidence_by_record.remove(&(
            observation.object_type.clone(),
            observation.source_id.clone(),
        ));
        if observation.provenance.is_none() {
            return Err(ReconciliationError::RecordEvidenceMismatch);
        }
        if observation
            .provenance
            .as_ref()
            .is_some_and(|evidence| evidence.identity_kind == SourceIdentityKind::Fallback)
        {
            inferred_ids.push(observation.source_id.clone());
        }
    }
    if !evidence_by_record.is_empty() {
        return Err(ReconciliationError::RecordEvidenceMismatch);
    }
    canonical.evidence.record_provenance_scope = ComparisonScope::Complete;
    if !inferred_ids.is_empty() {
        canonical
            .evidence
            .mismatches
            .push(safe_mismatch("inferred_record_identity", inferred_ids));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn bind_source_count_evidence(
    evidence: &mut WindowEvidence,
    window: &CanonicalPackWindow,
    source_identity: &SourceIdentity,
    pack: CapabilityPackId,
    schema_version: PackSchemaVersion,
    query_profile: &CanonicalText,
    filters_sha256: &CanonicalText,
    requested_window: &ReadWindow,
) -> Result<(), ReconciliationError> {
    let expected_object_types = pack_object_types(pack);
    let supplied = window.source_counts.as_deref().unwrap_or_default();
    if supplied
        .iter()
        .any(|count| !expected_object_types.contains(&count.object_type.as_str()))
    {
        return Err(ReconciliationError::SourceCountScopeMismatch);
    }

    let mut object_counts = BTreeMap::new();
    for &object_type in expected_object_types {
        let parsed_count = evidence
            .record_counts
            .get(object_type)
            .copied()
            .unwrap_or(0);
        let accepted_count = evidence
            .accepted_record_counts
            .get(object_type)
            .copied()
            .unwrap_or(0);
        let deduped_count = evidence
            .canonical_records
            .keys()
            .filter(|identity| {
                identity
                    .split_once('\0')
                    .is_some_and(|(kind, _)| kind == object_type)
            })
            .count() as u64;
        let mut complete = None;
        let mut window_only = None;
        for count in supplied
            .iter()
            .filter(|count| count.object_type.as_str() == object_type)
        {
            let descriptor = SourceCountScopeDescriptor {
                source_identity: source_identity.clone(),
                pack,
                pack_schema_version: schema_version,
                object_type: count.object_type.clone(),
                query_profile: query_profile.clone(),
                filters_sha256: filters_sha256.clone(),
                window: match count.source_count_scope {
                    SourceCountScope::Complete => None,
                    SourceCountScope::Window => Some(requested_window.clone()),
                },
            };
            let matches = count
                .matches_scope_descriptor(&descriptor)
                .map_err(|_| ReconciliationError::InvalidSourceCountEvidence)?;
            if !matches {
                return Err(ReconciliationError::SourceCountScopeMismatch);
            }
            match count.source_count_scope {
                SourceCountScope::Complete => complete = Some(count.source_reported_count),
                SourceCountScope::Window => window_only = Some(count.source_reported_count),
            }
        }
        let (scope, source_reported_count) = if let Some(count) = complete {
            (ComparisonScope::Complete, Some(count))
        } else if let Some(count) = window_only {
            (ComparisonScope::Window, Some(count))
        } else {
            (ComparisonScope::Unavailable, None)
        };
        object_counts.insert(
            object_type.to_string(),
            ObjectCountEvidence {
                scope,
                source_reported_count,
                parsed_count,
                accepted_count,
                deduped_count,
            },
        );
    }
    evidence.source_count_scope = if object_counts
        .values()
        .all(|count| count.scope == ComparisonScope::Complete)
    {
        ComparisonScope::Complete
    } else if object_counts
        .values()
        .any(|count| count.scope == ComparisonScope::Unavailable)
    {
        ComparisonScope::Unavailable
    } else {
        ComparisonScope::Window
    };
    evidence.source_count = object_counts
        .values()
        .filter_map(|count| count.source_reported_count)
        .sum();
    evidence.object_counts = object_counts;
    Ok(())
}

pub fn build_reconciliation(
    input: ReconciliationInput,
) -> Result<ReconciliationDecision, ReconciliationError> {
    validate_reconciliation_input(&input)?;

    let mut gaps = input.explicit_gap_codes;
    insert_run_drift_gaps(
        input.source_stability_check,
        input.end_profile_check,
        &mut gaps,
    );
    let warnings = input.warning_codes;
    let completed_ids = input.completed_windows.keys().cloned().collect();
    if input.planned_window_ids != completed_ids {
        gaps.insert("missing_snapshot_window".to_string());
    }

    let mut totals = SnapshotTotals::default();
    for evidence in input.completed_windows.values() {
        insert_window_count_gaps(evidence, &mut gaps);
        if let Some(code) = report_tie_out_gap(
            evidence,
            &input.source_identity,
            input.pack,
            input.pack_schema_version,
        ) {
            gaps.insert(code.to_string());
        }
        if !evidence.mismatches.is_empty() {
            gaps.insert("reconciliation_mismatch".to_string());
        }
        for (record_type, count) in &evidence.record_counts {
            *totals.record_counts.entry(record_type.clone()).or_insert(0) += count;
        }
        totals.add_object_counts(evidence, &mut gaps);
        totals.add_canonical_records(evidence, &mut gaps);
    }
    let record_counts = totals.finish(&mut gaps);

    let verification = if gaps.is_empty() {
        CoreVerificationState::Verified
    } else {
        CoreVerificationState::Partial
    };
    let snapshot_sha256 = snapshot_sha256(&input.completed_windows)?;
    let checkpoint_after = (verification == CoreVerificationState::Verified)
        .then(|| format!("full:{snapshot_sha256}"));

    let proof_gaps = gaps
        .iter()
        .map(|code| Gap {
            pack: input.pack,
            field_or_invariant: code.clone(),
            state: CapabilityState::Unknown,
            safe_reason_code: code.clone(),
        })
        .collect();
    let record_counts_sha256 = proof_record_counts_sha256(&record_counts);
    let proof = ProofManifest {
        proof_contract_version: PROOF_CONTRACT_VERSION,
        run_id: input.run_id,
        source_identity: input.source_identity,
        pack: input.pack,
        pack_schema_version: input.pack_schema_version,
        outcome: CoreRunOutcome::Completed,
        verification,
        freshness: if verification == CoreVerificationState::Verified {
            Freshness::Fresh
        } else {
            input.freshness_before
        },
        started_at_unix_ms: input.started_at_unix_ms,
        completed_at_unix_ms: Some(input.completed_at_unix_ms),
        record_counts,
        snapshot_sha256: Some(snapshot_sha256.clone()),
        gaps: proof_gaps,
    };
    let mirror_commit = CommitBatchInput::reconciled(CommitBatchParts {
        batch_id: input.batch_id,
        proof_contract_version: PROOF_CONTRACT_VERSION,
        outcome: RunOutcome::Completed,
        verification: if verification == CoreVerificationState::Verified {
            VerificationState::Verified
        } else {
            VerificationState::Partial
        },
        completed_at_unix_ms: input.completed_at_unix_ms,
        record_counts_sha256: Some(record_counts_sha256),
        snapshot_sha256: Some(snapshot_sha256),
        expected_checkpoint_before: None,
        checkpoint_after,
        freshness_target_seconds: input.freshness_target_seconds,
        gap_codes: gaps.into_iter().collect(),
        warning_codes: warnings
            .into_iter()
            .map(|warning| warning.as_str().to_string())
            .collect(),
    });

    Ok(ReconciliationDecision {
        proof,
        mirror_commit,
    })
}

/// Gaps for anything that may have changed in the source or the capability
/// profile while the run was reading it.
fn insert_run_drift_gaps(
    source_stability_check: SourceStabilityCheck,
    end_profile_check: EndProfileCheck,
    gaps: &mut BTreeSet<String>,
) {
    // The current XML transport reads multiple reports sequentially and does
    // not yet bracket them with an independently observed source watermark.
    // That remains a proof gap even when every internal invariant checked in
    // `build_reconciliation` and a fresh end-of-run capability-profile
    // comparison pass.
    match source_stability_check {
        SourceStabilityCheck::Passed => {
            // Full semantic reread equality is strong drift evidence, but
            // Tally documents no cross-request snapshot-isolation contract.
            gaps.insert("source_cut_atomicity_unavailable".to_string());
        }
        SourceStabilityCheck::Mismatch => {
            gaps.insert("source_changed_during_run".to_string());
        }
        SourceStabilityCheck::Unavailable => {
            gaps.insert("source_cut_consistency_unavailable".to_string());
        }
    }
    match end_profile_check {
        EndProfileCheck::Passed => {}
        EndProfileCheck::Mismatch => {
            gaps.insert("capability_profile_changed_during_run".to_string());
        }
        EndProfileCheck::Unavailable => {
            gaps.insert("capability_profile_drift_check_unavailable".to_string());
        }
    }
}

/// Gaps a single window's own parse, accept, dedupe and accounting counters
/// show, before anything is compared across windows.
fn insert_window_count_gaps(evidence: &WindowEvidence, gaps: &mut BTreeSet<String>) {
    if evidence.record_provenance_scope == ComparisonScope::Unavailable {
        gaps.insert("record_provenance_unavailable".to_string());
    }
    if evidence.parsed_count != evidence.accepted_count + evidence.rejected_count {
        gaps.insert("parse_accept_count_mismatch".to_string());
    }
    if evidence.accepted_count < evidence.deduped_count
        || evidence
            .accepted_count
            .saturating_sub(evidence.deduped_count)
            != evidence.duplicate_identity_count
    {
        gaps.insert("accept_dedupe_count_mismatch".to_string());
    }
    if evidence.rejected_count > 0 {
        gaps.insert("rejected_snapshot_records".to_string());
    }
    if evidence.duplicate_identity_count > 0 {
        gaps.insert("duplicate_source_identity".to_string());
    }
    if evidence.missing_identity_count > 0 {
        gaps.insert("missing_source_identity".to_string());
    }
    if evidence.out_of_range_count > 0 {
        gaps.insert("response_date_outside_window".to_string());
    }
    if evidence.accounting_scope == ComparisonScope::Unavailable {
        gaps.insert("accounting_reconciliation_unavailable".to_string());
    }
    gaps.extend(evidence.accounting_gap_codes.iter().cloned());
}

/// The gap a window's report tie-out leaves, or `None` when it passed for this
/// exact source, pack, period, query profile and filters.
fn report_tie_out_gap(
    evidence: &WindowEvidence,
    source_identity: &SourceIdentity,
    pack: CapabilityPackId,
    pack_schema_version: PackSchemaVersion,
) -> Option<&'static str> {
    match &evidence.report_tie_out {
        Some(report)
            if report.source_identity == *source_identity
                && report.pack == pack
                && report.pack_schema_version == pack_schema_version
                && report.from_yyyymmdd == evidence.from_yyyymmdd
                && report.to_yyyymmdd == evidence.to_yyyymmdd
                && report.query_profile.as_str() == evidence.query_profile
                && report.filters_sha256.as_str() == evidence.filters_sha256
                && is_lower_sha256(&report.report_sha256)
                && report.state == TieOutState::Passed
                && report.compared_ledger_count == report.source_reported_count
                && report.compared_ledger_count == report.core_ledger_count =>
        {
            None
        }
        Some(report) if report.state == TieOutState::Mismatch => Some("report_tie_out_mismatch"),
        Some(report) if report.state == TieOutState::Unavailable => {
            Some("period_report_profile_unobserved")
        }
        Some(_) => Some("report_tie_out_evidence_invalid"),
        None => Some("report_tie_out_unavailable"),
    }
}

/// Counts and identities accumulated across every completed window, so that
/// cross-window agreement can be judged once all windows are in.
#[derive(Default)]
struct SnapshotTotals {
    record_counts: BTreeMap<String, u64>,
    records_across_windows: BTreeMap<String, (String, ComparisonScope)>,
    complete_source_counts: BTreeMap<String, u64>,
    unique_identities_by_object: BTreeMap<String, BTreeSet<String>>,
    expected_count_objects: BTreeSet<String>,
    window_count_objects: BTreeSet<String>,
}

impl SnapshotTotals {
    /// Adds one window's per-object parse, accept and dedupe counts, and checks
    /// its source-reported count against the window or the complete scope.
    fn add_object_counts(&mut self, evidence: &WindowEvidence, gaps: &mut BTreeSet<String>) {
        for (object_type, count) in &evidence.object_counts {
            self.expected_count_objects.insert(object_type.clone());
            *self
                .record_counts
                .entry(format!("{object_type}.parsed"))
                .or_insert(0) += count.parsed_count;
            *self
                .record_counts
                .entry(format!("{object_type}.accepted"))
                .or_insert(0) += count.accepted_count;
            *self
                .record_counts
                .entry(format!("{object_type}.deduped"))
                .or_insert(0) += count.deduped_count;
            match count.scope {
                ComparisonScope::Unavailable => {}
                ComparisonScope::Window => {
                    self.window_count_objects.insert(object_type.clone());
                    if count.source_reported_count != Some(count.deduped_count) {
                        gaps.insert("window_source_accepted_count_mismatch".to_string());
                    }
                }
                ComparisonScope::Complete => {
                    let reported = count
                        .source_reported_count
                        .expect("complete evidence has a source count");
                    match self
                        .complete_source_counts
                        .insert(object_type.clone(), reported)
                    {
                        Some(previous) if previous != reported => {
                            gaps.insert("complete_source_count_disagreement".to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Records one window's canonical identities, flagging an identity seen in
    /// an earlier window with the same content (a duplicate, unless a complete
    /// repeatable master) or with different content (the source changed).
    fn add_canonical_records(&mut self, evidence: &WindowEvidence, gaps: &mut BTreeSet<String>) {
        for (identity, content_sha256) in &evidence.canonical_records {
            let identity_parts = identity.split_once('\0');
            let object_scope = identity_parts
                .and_then(|(object_type, _)| evidence.object_counts.get(object_type))
                .map_or(ComparisonScope::Unavailable, |count| count.scope);
            if let Some((object_type, source_id)) = identity_parts {
                self.unique_identities_by_object
                    .entry(object_type.to_string())
                    .or_default()
                    .insert(source_id.to_string());
            }
            match self
                .records_across_windows
                .insert(identity.clone(), (content_sha256.clone(), object_scope))
            {
                Some((previous, previous_scope)) if previous == *content_sha256 => {
                    if previous_scope != ComparisonScope::Complete
                        || object_scope != ComparisonScope::Complete
                        || !identity_parts
                            .is_some_and(|(object_type, _)| repeatable_complete_master(object_type))
                    {
                        gaps.insert("duplicate_record_across_windows".to_string());
                    }
                }
                Some(_) => {
                    gaps.insert("source_changed_during_snapshot".to_string());
                }
                None => {}
            }
        }
    }

    /// Compares complete source counts with the unique identities accepted and
    /// records both; then marks every object type whose count was only
    /// window-scoped or never reported. Returns the record counts for the proof.
    fn finish(self, gaps: &mut BTreeSet<String>) -> BTreeMap<String, u64> {
        let Self {
            mut record_counts,
            complete_source_counts,
            unique_identities_by_object,
            mut expected_count_objects,
            mut window_count_objects,
            records_across_windows: _,
        } = self;
        for (object_type, count) in complete_source_counts {
            let accepted_unique = unique_identities_by_object
                .get(&object_type)
                .map_or(0, |identities| identities.len() as u64);
            if count != accepted_unique {
                gaps.insert("source_accepted_count_mismatch".to_string());
            }
            record_counts.insert(format!("{object_type}.source_reported_complete"), count);
            record_counts.insert(format!("{object_type}.accepted_unique"), accepted_unique);
            expected_count_objects.remove(&object_type);
            window_count_objects.remove(&object_type);
        }
        for object_type in expected_count_objects {
            if window_count_objects.contains(&object_type) {
                gaps.insert("source_count_window_only".to_string());
            } else {
                gaps.insert("source_count_unavailable".to_string());
            }
        }
        record_counts
    }
}

fn repeatable_complete_master(object_type: &str) -> bool {
    matches!(object_type, "group" | "ledger" | "voucher_type")
}

#[allow(clippy::too_many_arguments)]
pub fn build_terminal_proof(
    batch_id: String,
    run_id: String,
    source_identity: SourceIdentity,
    pack: CapabilityPackId,
    pack_schema_version: PackSchemaVersion,
    started_at_unix_ms: i64,
    completed_at_unix_ms: i64,
    freshness_before: Freshness,
    freshness_target_seconds: i64,
    kind: TerminalKind,
    safe_reason_code: String,
    mut gap_codes: BTreeSet<String>,
    warning_codes: BTreeSet<WarningCode>,
    record_counts: BTreeMap<String, u64>,
) -> ReconciliationDecision {
    let clock_moved_backwards = completed_at_unix_ms < started_at_unix_ms;
    let completed_at_unix_ms = completed_at_unix_ms.max(started_at_unix_ms);
    if clock_moved_backwards {
        gap_codes.insert("local_clock_moved_backwards".to_string());
    }
    let (core_outcome, mirror_outcome) = match kind {
        TerminalKind::Failed => (CoreRunOutcome::Failed, RunOutcome::Failed),
        TerminalKind::Cancelled => (CoreRunOutcome::Cancelled, RunOutcome::Cancelled),
    };
    gap_codes.insert(safe_reason_code);
    let gaps = gap_codes
        .iter()
        .map(|code| Gap {
            pack,
            field_or_invariant: code.clone(),
            state: CapabilityState::Unknown,
            safe_reason_code: code.clone(),
        })
        .collect();
    let record_counts_sha256 = proof_record_counts_sha256(&record_counts);
    ReconciliationDecision {
        proof: ProofManifest {
            proof_contract_version: PROOF_CONTRACT_VERSION,
            run_id,
            source_identity,
            pack,
            pack_schema_version,
            outcome: core_outcome,
            verification: CoreVerificationState::Unverified,
            freshness: freshness_before,
            started_at_unix_ms,
            completed_at_unix_ms: Some(completed_at_unix_ms),
            record_counts,
            snapshot_sha256: None,
            gaps,
        },
        mirror_commit: CommitBatchInput::reconciled(CommitBatchParts {
            batch_id,
            proof_contract_version: PROOF_CONTRACT_VERSION,
            outcome: mirror_outcome,
            verification: VerificationState::Unverified,
            completed_at_unix_ms,
            record_counts_sha256: Some(record_counts_sha256),
            snapshot_sha256: None,
            expected_checkpoint_before: None,
            checkpoint_after: None,
            freshness_target_seconds,
            gap_codes: gap_codes.into_iter().collect(),
            warning_codes: warning_codes
                .into_iter()
                .map(|warning| warning.as_str().to_string())
                .collect(),
        }),
    }
}

pub(crate) fn proof_record_counts_sha256(record_counts: &BTreeMap<String, u64>) -> String {
    let mut digest = Sha256::new();
    digest.update(b"bridge-tally-proof-record-counts-v1\0");
    digest.update((record_counts.len() as u64).to_be_bytes());
    for (key, count) in record_counts {
        hash_framed(&mut digest, key.as_bytes());
        digest.update(count.to_be_bytes());
    }
    hex_digest(digest.finalize())
}

fn canonicalize_core_window(
    window_id: &str,
    requested_window: &ReadWindow,
    core: &CoreAccountingBatch,
) -> Result<CanonicalWindow, ReconciliationError> {
    if core
        .vouchers
        .iter()
        .any(|voucher| TallyDate::parse(voucher.date_yyyymmdd.clone()).is_err())
    {
        return Err(ReconciliationError::InvalidTypedPack);
    }
    let mut observations = Vec::new();
    let mut duplicate_identity_count = 0_u64;
    let mut missing_identity_count = 0_u64;
    let mut record_counts = BTreeMap::new();
    let mut accepted_record_counts = BTreeMap::new();
    let mut seen = BTreeSet::new();
    let mut source_count = 0_u64;

    macro_rules! push_records {
        ($records:expr, $object_type:literal, $name:expr, $decimals:expr) => {{
            let mut records = $records.iter().collect::<Vec<_>>();
            records.sort_by(|left, right| {
                left.source_id.cmp(&right.source_id).then_with(|| {
                    canonical_sha256(*left)
                        .unwrap_or_default()
                        .cmp(&canonical_sha256(*right).unwrap_or_default())
                })
            });
            for record in records {
                source_count += 1;
                *record_counts.entry($object_type.to_string()).or_insert(0) += 1;
                if !mirror_safe_source_id(&record.source_id) {
                    missing_identity_count += 1;
                    continue;
                }
                *accepted_record_counts
                    .entry($object_type.to_string())
                    .or_insert(0) += 1;
                if !seen.insert(($object_type, record.source_id.as_str())) {
                    duplicate_identity_count += 1;
                    continue;
                }
                let canonical_payload = canonical_value(
                    serde_json::to_value(record).map_err(|_| ReconciliationError::Serialization)?,
                );
                observations.push(CanonicalObservation {
                    object_type: $object_type.to_string(),
                    source_id: record.source_id.clone(),
                    display_name: $name(record),
                    canonical_sha256: canonical_sha256(&canonical_payload)?,
                    canonical_payload,
                    exact_decimals: $decimals(record),
                    provenance: None,
                });
            }
        }};
    }

    push_records!(
        &core.groups,
        "group",
        |record: &bridge_tally_core::GroupRecord| Some(record.name.clone()),
        |_record: &bridge_tally_core::GroupRecord| BTreeMap::new()
    );
    push_records!(
        &core.ledgers,
        "ledger",
        |record: &bridge_tally_core::LedgerRecord| Some(record.name.clone()),
        |record: &bridge_tally_core::LedgerRecord| {
            let mut values = BTreeMap::new();
            if let Some(value) = &record.opening_balance {
                values.insert("opening_balance".to_string(), value.as_str().to_string());
            }
            values
        }
    );
    push_records!(
        &core.voucher_types,
        "voucher_type",
        |record: &bridge_tally_core::VoucherTypeRecord| Some(record.name.clone()),
        |_record: &bridge_tally_core::VoucherTypeRecord| BTreeMap::new()
    );
    push_records!(
        &core.vouchers,
        "voucher",
        |_record: &bridge_tally_core::VoucherRecord| None,
        |_record: &bridge_tally_core::VoucherRecord| BTreeMap::new()
    );
    push_records!(
        &core.ledger_entries,
        "ledger_entry",
        |_record: &bridge_tally_core::LedgerEntryRecord| None,
        |record: &bridge_tally_core::LedgerEntryRecord| {
            BTreeMap::from([("amount".to_string(), record.amount.as_str().to_string())])
        }
    );

    observations.sort_by(|left, right| {
        left.object_type
            .cmp(&right.object_type)
            .then(left.source_id.cmp(&right.source_id))
            .then(left.canonical_sha256.cmp(&right.canonical_sha256))
    });
    let out_of_range_ids = core
        .vouchers
        .iter()
        .filter(|voucher| {
            voucher.date_yyyymmdd < requested_window.from_yyyymmdd
                || voucher.date_yyyymmdd > requested_window.to_yyyymmdd
        })
        .map(|voucher| voucher.source_id.clone())
        .collect::<Vec<_>>();
    let out_of_range_count = out_of_range_ids.len() as u64;
    let accounting = reconcile_core_accounting(core);
    let mut mismatches = accounting.mismatches;
    if !out_of_range_ids.is_empty() {
        mismatches.push(safe_mismatch(
            "response_date_outside_window",
            out_of_range_ids,
        ));
    }
    let canonical_sha256 = hash_observations(&observations);
    let canonical_records = observations
        .iter()
        .map(|observation| {
            (
                format!("{}\0{}", observation.object_type, observation.source_id),
                observation.canonical_sha256.clone(),
            )
        })
        .collect();
    let accepted_count = observations.len() as u64;
    let validated_count = accepted_record_counts.values().sum();
    let rejected_count = source_count.saturating_sub(validated_count);

    Ok(CanonicalWindow {
        observations,
        evidence: WindowEvidence {
            window_id: window_id.to_string(),
            from_yyyymmdd: requested_window.from_yyyymmdd.clone(),
            to_yyyymmdd: requested_window.to_yyyymmdd.clone(),
            canonical_sha256,
            query_profile: String::new(),
            filters_sha256: String::new(),
            record_provenance_scope: ComparisonScope::Unavailable,
            source_count_scope: ComparisonScope::Unavailable,
            source_count,
            parsed_count: source_count,
            accepted_count: validated_count,
            deduped_count: accepted_count,
            rejected_count,
            duplicate_identity_count,
            missing_identity_count,
            out_of_range_count,
            record_counts,
            accepted_record_counts,
            object_counts: BTreeMap::new(),
            record_set_sha256: None,
            canonical_records,
            accounting_scope: ComparisonScope::Window,
            accounting_gap_codes: accounting.gap_codes,
            report_tie_out_scope: ComparisonScope::Unavailable,
            report_tie_out: None,
            mismatches,
        },
    })
}

fn mirror_safe_source_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

struct CoreAccountingReconciliation {
    mismatches: Vec<ReconciliationMismatch>,
    gap_codes: BTreeSet<String>,
}

fn reconcile_core_accounting(core: &CoreAccountingBatch) -> CoreAccountingReconciliation {
    let assessment = bridge_tally_core::reconciliation::assess_core_accounting(core);
    let mismatches = assessment
        .issues
        .into_iter()
        .map(|issue| safe_mismatch(issue.safe_reason_code, issue.source_ids))
        .collect();
    let mut gap_codes = BTreeSet::new();
    if assessment.checks.voucher_entry_applicability
        == bridge_tally_core::reconciliation::CheckState::Unavailable
    {
        gap_codes.insert("voucher_entry_applicability_unavailable".to_string());
    }
    if assessment.checks.voucher_header_entry_total
        == bridge_tally_core::reconciliation::CheckState::Unavailable
    {
        gap_codes.insert("voucher_header_entry_total_unavailable".to_string());
    }
    if assessment.checks.voucher_entry_polarity
        == bridge_tally_core::reconciliation::CheckState::Unavailable
    {
        gap_codes.insert("voucher_entry_polarity_unavailable".to_string());
    }
    CoreAccountingReconciliation {
        mismatches,
        gap_codes,
    }
}

fn safe_mismatch(code: &str, ids: Vec<String>) -> ReconciliationMismatch {
    let mut safe_record_ids = ids
        .into_iter()
        .map(|source_id| safe_record_token(&source_id))
        .collect::<Vec<_>>();
    safe_record_ids.sort();
    safe_record_ids.dedup();
    safe_record_ids.truncate(MAX_SAFE_DRILL_DOWN_IDS);
    ReconciliationMismatch {
        safe_reason_code: code.to_string(),
        safe_record_ids,
    }
}

fn safe_record_token(source_id: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(b"bridge-tally-safe-record-id-v1\0");
    hash_framed(&mut digest, source_id.as_bytes());
    format!("rid:{}", hex_digest(digest.finalize()))
}

fn validate_reconciliation_input(input: &ReconciliationInput) -> Result<(), ReconciliationError> {
    if input.batch_id.is_empty()
        || input.run_id.is_empty()
        || input.planned_window_ids.is_empty()
        || input.freshness_target_seconds <= 0
        || input.completed_at_unix_ms < input.started_at_unix_ms
    {
        return Err(ReconciliationError::InvalidInput("run_metadata"));
    }
    for code in &input.explicit_gap_codes {
        if !is_safe_code(code) {
            return Err(ReconciliationError::InvalidInput("safe_code"));
        }
    }
    Ok(())
}

fn snapshot_sha256(
    windows: &BTreeMap<String, WindowEvidence>,
) -> Result<String, ReconciliationError> {
    #[derive(Serialize)]
    struct SnapshotHashInput<'a> {
        windows: &'a BTreeMap<String, WindowEvidence>,
    }
    canonical_sha256(&SnapshotHashInput { windows })
}

fn hash_observations(observations: &[CanonicalObservation]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"bridge-tally-window-v1\0");
    for observation in observations {
        hash_framed(&mut digest, observation.object_type.as_bytes());
        hash_framed(&mut digest, observation.source_id.as_bytes());
        hash_framed(&mut digest, observation.canonical_sha256.as_bytes());
    }
    hex_digest(digest.finalize())
}

fn hash_framed(digest: &mut Sha256, value: &[u8]) {
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
}

fn canonical_sha256(value: &impl Serialize) -> Result<String, ReconciliationError> {
    let value = serde_json::to_value(value).map_err(|_| ReconciliationError::Serialization)?;
    let bytes = serde_json::to_vec(&canonical_value(value))
        .map_err(|_| ReconciliationError::Serialization)?;
    Ok(hex_digest(Sha256::digest(bytes)))
}

fn canonical_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonical_value).collect()),
        Value::Object(values) => {
            let ordered = values
                .into_iter()
                .map(|(key, value)| (key, canonical_value(value)))
                .collect::<BTreeMap<_, _>>();
            serde_json::to_value(ordered).expect("BTreeMap JSON serialization cannot fail")
        }
        other => other,
    }
}

fn hex_digest(digest: impl AsRef<[u8]>) -> String {
    digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn is_lower_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_safe_code(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'_' | b'-' | b'.' | b':')
        })
}

fn pack_id(batch: &PackBatch) -> CapabilityPackId {
    match batch {
        PackBatch::CoreAccounting(_) => CapabilityPackId::CoreAccounting,
    }
}

fn pack_object_types(pack: CapabilityPackId) -> &'static [&'static str] {
    match pack {
        CapabilityPackId::CoreAccounting => {
            &["group", "ledger", "voucher_type", "voucher", "ledger_entry"]
        }
        CapabilityPackId::IndiaTax => &["tax_registration", "voucher_tax"],
        CapabilityPackId::BillsAndPayments => {
            &["party_outstanding", "bill_allocation", "bill_outstanding"]
        }
        CapabilityPackId::Inventory => &["stock_item", "godown", "inventory_entry"],
    }
}

#[cfg(test)]
#[path = "reconciliation_tests.rs"]
mod tests;
