// SPDX-License-Identifier: Apache-2.0
//! Item-level scoring of one canonical dump against a sealed answer key (#738).
//!
//! A key names every finding id a test can emit and, for each, the evidence items a reader who
//! saw only the book expects it to list, each with that reader's reason. A key is sealed: it opens
//! only with the sha256 committed for it before any run was scored, so a key edited afterwards
//! does not open. A score compares `kind:id` item sets per finding: the items expected and listed,
//! the items missed (with the key's reason), and the items listed that the key did not expect.
//! Figures are not scored; parity covers them.
//!
//! A score describes one run against one key for one synthetic book, and nothing beyond it.
//!
//! The scorer lives under `tests/` because only the crate's tests run it: a module under `src/`
//! would change `src/lib.rs`, which would select every mutation that targets that file
//! (`parity/mutations.mjs`).
//!
//! The key format, in TOML:
//!
//! ```toml
//! test_id = "stale_balances_41_1"
//!
//! [[finding]]
//! id = "stale_balances_41_1/s41_1/creditor"
//!
//! [[finding.item]]
//! kind = "ledger"
//! id = "A ledger name, exactly as the book spells it"
//! reason = "Why the reader expects it listed"
//! ```
//!
//! A finding with no `item` is one the reader expects to list nothing.

use std::collections::{BTreeMap, BTreeSet};

use bridge_tax_audit::canonical::hex;
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// Why a key does not open.
#[derive(Debug, PartialEq, Eq)]
pub enum KeyError {
    /// The key's bytes do not hash to the sealed sha256.
    SealMismatch {
        sealed: String,
        actual: String,
    },
    NotUtf8,
    /// Not the key format: a missing or unknown field, or a value of the wrong type.
    Parse(String),
    /// A required text is empty or only whitespace. Names are never trimmed otherwise.
    Blank(&'static str),
    NoFindings,
    DuplicateFinding(String),
    DuplicateItem {
        finding: String,
        item: String,
    },
}

/// Why a dump is not scored against a key.
#[derive(Debug, PartialEq, Eq)]
pub enum ScoreError {
    TestIdDiffers {
        key: String,
        dump: String,
    },
    /// The dump is not a canonical dump: a missing field or a value of the wrong type.
    DumpInvalid(String),
    /// The dump holds a finding the key does not name, so the key cannot judge its items.
    DumpFindingNotInKey(String),
    DuplicateListed {
        finding: String,
        item: String,
    },
    /// The key expects nothing and the dump lists nothing: refused, never a perfect score.
    Empty,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawKey {
    test_id: String,
    #[serde(default)]
    finding: Vec<RawFinding>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFinding {
    id: String,
    #[serde(default)]
    item: Vec<RawItem>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawItem {
    kind: String,
    id: String,
    reason: String,
}

/// A key that opened against its seal. Only [`SealedKey::open`] makes one.
#[derive(Debug)]
pub struct SealedKey {
    test_id: String,
    /// finding id -> item (`kind:id`) -> reason.
    findings: BTreeMap<String, BTreeMap<String, String>>,
}

fn non_blank(text: &str, field: &'static str) -> Result<(), KeyError> {
    if text.trim().is_empty() {
        return Err(KeyError::Blank(field));
    }
    Ok(())
}

impl SealedKey {
    /// Open a key from its bytes and the sha256 (lowercase hex) sealed for it.
    pub fn open(bytes: &[u8], sealed_sha256: &str) -> Result<Self, KeyError> {
        let actual = hex(&Sha256::digest(bytes));
        if actual != sealed_sha256 {
            return Err(KeyError::SealMismatch {
                sealed: sealed_sha256.to_string(),
                actual,
            });
        }
        let text = std::str::from_utf8(bytes).map_err(|_| KeyError::NotUtf8)?;
        let raw: RawKey = toml::from_str(text).map_err(|e| KeyError::Parse(e.to_string()))?;
        non_blank(&raw.test_id, "test_id")?;
        if raw.finding.is_empty() {
            return Err(KeyError::NoFindings);
        }
        let mut findings = BTreeMap::new();
        for finding in raw.finding {
            non_blank(&finding.id, "finding.id")?;
            let mut items = BTreeMap::new();
            for item in finding.item {
                non_blank(&item.kind, "item.kind")?;
                non_blank(&item.id, "item.id")?;
                non_blank(&item.reason, "item.reason")?;
                let key = format!("{}:{}", item.kind, item.id);
                if items.insert(key.clone(), item.reason).is_some() {
                    return Err(KeyError::DuplicateItem {
                        finding: finding.id,
                        item: key,
                    });
                }
            }
            if findings.insert(finding.id.clone(), items).is_some() {
                return Err(KeyError::DuplicateFinding(finding.id));
            }
        }
        Ok(Self {
            test_id: raw.test_id,
            findings,
        })
    }
}

/// An expected item the dump did not list, with the key's reason for expecting it.
#[derive(Debug, PartialEq, Eq)]
pub struct Miss {
    pub item: String,
    pub reason: String,
}

/// One finding's comparison. Recall is `matched / expected`, precision `matched / listed`; each is
/// undefined when its denominator is 0.
#[derive(Debug, PartialEq, Eq)]
pub struct FindingScore {
    pub finding_id: String,
    pub expected: usize,
    pub listed: usize,
    pub matched: usize,
    pub missed: Vec<Miss>,
    /// Items listed that the key does not expect, as `kind:id`.
    pub extra: Vec<String>,
}

/// Every finding the key names, in id order.
#[derive(Debug, PartialEq, Eq)]
pub struct Score {
    pub test_id: String,
    pub findings: Vec<FindingScore>,
}

fn field<'a>(doc: &'a Value, key: &str, at: &str) -> Result<&'a Value, ScoreError> {
    doc.get(key)
        .ok_or_else(|| ScoreError::DumpInvalid(format!("{at}: no {key}")))
}

fn string<'a>(doc: &'a Value, key: &str, at: &str) -> Result<&'a str, ScoreError> {
    field(doc, key, at)?
        .as_str()
        .ok_or_else(|| ScoreError::DumpInvalid(format!("{at}: {key} is not a string")))
}

fn array<'a>(doc: &'a Value, key: &str, at: &str) -> Result<&'a [Value], ScoreError> {
    field(doc, key, at)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| ScoreError::DumpInvalid(format!("{at}: {key} is not a list")))
}

/// Each finding's listed items (`kind:id`), from a canonical dump.
fn listed_items(dump: &Value) -> Result<BTreeMap<String, BTreeSet<String>>, ScoreError> {
    let mut listed = BTreeMap::new();
    for finding in array(dump, "findings", "dump")? {
        let id = string(finding, "id", "finding")?.to_string();
        let mut items = BTreeSet::new();
        for evidence in array(finding, "evidence", &id)? {
            let item = format!(
                "{}:{}",
                string(evidence, "kind", &id)?,
                string(evidence, "id", &id)?
            );
            if !items.insert(item.clone()) {
                return Err(ScoreError::DuplicateListed { finding: id, item });
            }
        }
        if listed.insert(id.clone(), items).is_some() {
            return Err(ScoreError::DumpInvalid(format!("finding {id} repeats")));
        }
    }
    Ok(listed)
}

/// Score one canonical dump (`canonical::canonical_test_result`'s form) against a sealed key.
/// A finding the key names and the dump lacks lists nothing.
pub fn score(key: &SealedKey, dump: &Value) -> Result<Score, ScoreError> {
    let dump_test = string(dump, "test_id", "dump")?;
    if dump_test != key.test_id {
        return Err(ScoreError::TestIdDiffers {
            key: key.test_id.clone(),
            dump: dump_test.to_string(),
        });
    }
    let mut listed = listed_items(dump)?;
    if let Some(id) = listed.keys().find(|id| !key.findings.contains_key(*id)) {
        return Err(ScoreError::DumpFindingNotInKey(id.clone()));
    }
    let mut findings = Vec::new();
    for (finding_id, expected) in &key.findings {
        let listed = listed.remove(finding_id).unwrap_or_default();
        let missed = expected
            .iter()
            .filter(|(item, _)| !listed.contains(*item))
            .map(|(item, reason)| Miss {
                item: item.clone(),
                reason: reason.clone(),
            })
            .collect::<Vec<_>>();
        let extra = listed
            .iter()
            .filter(|item| !expected.contains_key(*item))
            .cloned()
            .collect::<Vec<_>>();
        findings.push(FindingScore {
            finding_id: finding_id.clone(),
            expected: expected.len(),
            listed: listed.len(),
            matched: expected.len() - missed.len(),
            missed,
            extra,
        });
    }
    if findings.iter().all(|f| f.expected == 0 && f.listed == 0) {
        return Err(ScoreError::Empty);
    }
    Ok(Score {
        test_id: key.test_id.clone(),
        findings,
    })
}
