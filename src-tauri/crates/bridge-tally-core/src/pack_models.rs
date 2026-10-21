use crate::{
    CapabilityPackId, PackSchemaVersion, ReadWindow, SourceIdentity, TallyDate,
    TallyError,
};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

const MAX_CANONICAL_TEXT_BYTES: usize = 512;
const MAX_SOURCE_ID_BYTES: usize = 512;
const MAX_ALTER_ID_BYTES: usize = 128;

/// A structurally valid GSTIN-shaped identifier observed from Tally.
///
/// This type does not claim that the registration exists, is active, belongs to the company, or
/// passed an external GST portal lookup; those are separate evidence states.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct CanonicalText(String);

impl CanonicalText {
    pub fn parse(value: impl Into<String>) -> Result<Self, TallyError> {
        let value = value.into();
        validate_text(&value, "canonical_text_invalid")?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Text copied verbatim from a client's Tally book.
///
/// Bridge has no authority to normalise or reject this text: Tally resolves
/// master names by exact code point.  In particular, a name that contains a
/// C1 control character may be ugly to render, but it remains the only valid
/// token with which to refer to that master in Tally.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct ForeignText(String);

impl ForeignText {
    pub fn from_tally(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    /// A presentation warning, never an instruction to rewrite the source.
    pub fn document_rendering_diagnostic(&self) -> Option<ForeignTextDiagnostic> {
        if !self.0.chars().any(char::is_control) {
            return None;
        }
        Some(ForeignTextDiagnostic {
            contains_control_characters: true,
            likely_intended_spelling: latin1_as_utf8_suggestion(&self.0),
        })
    }
}

impl<'de> Deserialize<'de> for ForeignText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_tally(String::deserialize(deserializer)?))
    }
}

/// Presentation-only metadata for foreign text. `likely_intended_spelling`
/// is an operator suggestion; `ForeignText` always retains the exact Tally
/// value and callers must never substitute this suggestion into a request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ForeignTextDiagnostic {
    pub contains_control_characters: bool,
    pub likely_intended_spelling: Option<String>,
}

fn latin1_as_utf8_suggestion(value: &str) -> Option<String> {
    let bytes = value
        .chars()
        .map(u32::from)
        .map(u8::try_from)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let repaired = String::from_utf8(bytes).ok()?;
    (repaired != value).then_some(repaired)
}

impl<'de> Deserialize<'de> for CanonicalText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceCountScope {
    /// The source explicitly reported the count for the complete object/query
    /// scope, not merely for the records parsed in this response.
    Complete,
    Window,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReportedCountEvidence {
    pub object_type: CanonicalText,
    pub query_profile: CanonicalText,
    /// Stable fingerprint of the exact company/filter/window count scope.
    pub source_scope_fingerprint: CanonicalText,
    pub source_count_scope: SourceCountScope,
    pub source_reported_count: u64,
}

/// Identifies which Tally-origin identity field produced a canonical record's
/// source ID. Consumers must preserve the selected field rather than guessing
/// that every identifier is a remote ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceIdentityKind {
    Guid,
    RemoteId,
    MasterId,
    Fallback,
}

/// SHA-256 of the exact source payload from which the record was parsed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct RawSourceSha256(String);

impl RawSourceSha256 {
    pub fn parse(value: impl Into<String>) -> Result<Self, TallyError> {
        let value = value.into();
        if !is_lower_sha256(&value) {
            return Err(invalid_data("raw_source_sha256_invalid"));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for RawSourceSha256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

/// Source alter identifier observed with a record, retained as an opaque
/// printable token rather than interpreted as a number.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct SourceAlterId(String);

impl SourceAlterId {
    pub fn parse(value: impl Into<String>) -> Result<Self, TallyError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > MAX_ALTER_ID_BYTES
            || value.trim() != value
            || !value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b':')
            })
        {
            return Err(invalid_data("source_alter_id_invalid"));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SourceAlterId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

/// Per-record provenance supplied by the connector. The object type and
/// source ID bind this evidence to exactly one canonical record.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRecordEvidence {
    pub object_type: CanonicalText,
    pub source_id: SourceRecordId,
    pub identity_kind: SourceIdentityKind,
    #[serde(default)]
    pub observed_identities: ObservedSourceIdentities,
    pub raw_source_sha256: RawSourceSha256,
    pub alter_id: Option<SourceAlterId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObservedSourceIdentities {
    pub guid: Option<SourceRecordId>,
    pub remote_id: Option<SourceRecordId>,
    pub master_id: Option<SourceRecordId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceCountScopeDescriptor {
    pub source_identity: SourceIdentity,
    pub pack: CapabilityPackId,
    pub pack_schema_version: PackSchemaVersion,
    pub object_type: CanonicalText,
    pub query_profile: CanonicalText,
    pub filters_sha256: CanonicalText,
    /// Must be `None` for `Complete` and the exact requested window for
    /// `Window`; the helper rejects the opposite combinations.
    pub window: Option<ReadWindow>,
}

#[derive(Serialize)]
struct SourceCountScopeFingerprintPreimage<'a> {
    contract: &'static str,
    source_identity: &'a SourceIdentity,
    pack: CapabilityPackId,
    pack_schema_version: PackSchemaVersion,
    object_type: &'a CanonicalText,
    query_profile: &'a CanonicalText,
    filters_sha256: &'a CanonicalText,
    window: &'a Option<ReadWindow>,
}

pub fn source_count_scope_fingerprint(
    descriptor: &SourceCountScopeDescriptor,
    scope: SourceCountScope,
) -> Result<CanonicalText, TallyError> {
    validate_scope_descriptor(descriptor, scope)?;
    let preimage = SourceCountScopeFingerprintPreimage {
        contract: "bridge_tally_source_count_scope_v1",
        source_identity: &descriptor.source_identity,
        pack: descriptor.pack,
        pack_schema_version: descriptor.pack_schema_version,
        object_type: &descriptor.object_type,
        query_profile: &descriptor.query_profile,
        filters_sha256: &descriptor.filters_sha256,
        window: &descriptor.window,
    };
    let canonical = serde_json::to_vec(&preimage)
        .map_err(|_| invalid_data("source_count_scope_serialization_failed"))?;
    let digest = Sha256::digest(canonical);
    CanonicalText::parse(hex_lower(&digest))
}

impl SourceReportedCountEvidence {
    pub fn matches_scope_descriptor(
        &self,
        descriptor: &SourceCountScopeDescriptor,
    ) -> Result<bool, TallyError> {
        if self.object_type != descriptor.object_type
            || self.query_profile != descriptor.query_profile
        {
            return Ok(false);
        }
        Ok(self.source_scope_fingerprint
            == source_count_scope_fingerprint(descriptor, self.source_count_scope)?)
    }
}

/// Canonical response envelope for a single requested pack window. Counts and
/// per-record provenance are optional because Bridge must never invent source
/// evidence. Missing record evidence is valid for synthetic/test producers,
/// but production reconciliation must classify that window as partial.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PackWindow<T> {
    pub batch: T,
    pub source_counts: Option<Vec<SourceReportedCountEvidence>>,
    pub record_evidence: Option<Vec<SourceRecordEvidence>>,
}

impl<T> PackWindow<T> {
    pub fn without_source_count_evidence(batch: T) -> Self {
        Self {
            batch,
            source_counts: None,
            record_evidence: None,
        }
    }

    pub fn validate_source_count_evidence(&self) -> Result<(), TallyError> {
        let Some(source_counts) = &self.source_counts else {
            return Ok(());
        };
        if source_counts.is_empty() {
            return Err(invalid_data("source_count_evidence_empty"));
        }
        let mut scopes = BTreeSet::new();
        for evidence in source_counts {
            let key = (
                evidence.object_type.as_str(),
                evidence.query_profile.as_str(),
                evidence.source_scope_fingerprint.as_str(),
                evidence.source_count_scope,
            );
            if !scopes.insert(key) {
                return Err(invalid_data("source_count_evidence_duplicate_scope"));
            }
        }
        Ok(())
    }

    pub fn validate_record_evidence(&self) -> Result<(), TallyError> {
        let Some(record_evidence) = &self.record_evidence else {
            return Ok(());
        };
        // Empty evidence is meaningful for a proven-empty pack window; the pack-specific
        // binding check still requires it to match an empty canonical record set exactly.
        let mut records = BTreeSet::new();
        for evidence in record_evidence {
            let selected = match evidence.identity_kind {
                SourceIdentityKind::Guid => evidence.observed_identities.guid.as_ref(),
                SourceIdentityKind::RemoteId => evidence.observed_identities.remote_id.as_ref(),
                SourceIdentityKind::MasterId => evidence.observed_identities.master_id.as_ref(),
                SourceIdentityKind::Fallback => {
                    if evidence.observed_identities != ObservedSourceIdentities::default() {
                        return Err(invalid_data("fallback_identity_claimed_native_ids"));
                    }
                    Some(&evidence.source_id)
                }
            };
            if selected != Some(&evidence.source_id) {
                return Err(invalid_data("source_record_primary_identity_mismatch"));
            }
            if !records.insert((evidence.object_type.as_str(), evidence.source_id.as_str())) {
                return Err(invalid_data("source_record_evidence_duplicate_record"));
            }
        }
        Ok(())
    }
}

/// Stable Tally-origin identity or reference. It is intentionally serialized
/// as a string so the wire contract stays compact while construction and
/// deserialization remain fail-closed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct SourceRecordId(String);

impl SourceRecordId {
    pub fn parse(value: impl Into<String>) -> Result<Self, TallyError> {
        let value = value.into();
        if value.is_empty()
            || value.len() > MAX_SOURCE_ID_BYTES
            || value.trim() != value
            || value.chars().any(char::is_control)
        {
            return Err(invalid_data("invalid_source_record_id"));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SourceRecordId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

fn validate_text(value: &str, code: &'static str) -> Result<(), TallyError> {
    if value.is_empty()
        || value.len() > MAX_CANONICAL_TEXT_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(invalid_data(code));
    }
    Ok(())
}

fn invalid_data(code: &'static str) -> TallyError {
    TallyError::InvalidData {
        code: code.to_string(),
    }
}

fn is_lower_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_scope_descriptor(
    descriptor: &SourceCountScopeDescriptor,
    scope: SourceCountScope,
) -> Result<(), TallyError> {
    let identity = &descriptor.source_identity;
    for value in [
        identity.bridge_source_lineage.as_str(),
        identity.company_guid.as_str(),
        identity.observed_fingerprint.as_str(),
    ] {
        validate_text(value, "source_count_source_identity_invalid")?;
    }
    if descriptor.pack_schema_version.major == 0 {
        return Err(invalid_data("source_count_pack_schema_invalid"));
    }
    validate_sha256(
        descriptor.filters_sha256.as_str(),
        "source_count_filters_sha256_invalid",
    )?;
    match (scope, descriptor.window.as_ref()) {
        (SourceCountScope::Complete, None) | (SourceCountScope::Window, Some(_)) => {}
        _ => return Err(invalid_data("source_count_window_scope_mismatch")),
    }
    if let Some(window) = &descriptor.window {
        TallyDate::parse(window.from_yyyymmdd.clone())?;
        TallyDate::parse(window.to_yyyymmdd.clone())?;
        if window.from_yyyymmdd > window.to_yyyymmdd {
            return Err(invalid_data("source_count_window_reversed"));
        }
    }
    Ok(())
}

fn validate_sha256(value: &str, code: &'static str) -> Result<(), TallyError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid_data(code));
    }
    Ok(())
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(test)]
#[path = "pack_models_tests.rs"]
mod tests;
