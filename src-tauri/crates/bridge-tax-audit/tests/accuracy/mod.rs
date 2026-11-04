// SPDX-License-Identifier: Apache-2.0
//! Item-level scoring of one canonical dump against a sealed answer key (#738).
//!
//! A key names every finding id a test can emit and, for each, the evidence items a reader who
//! saw only the book expects it to list, each with that reader's reason. A key is sealed: it opens
//! only with the sha256 given for it, so a key whose bytes changed after that hash was taken does
//! not open (when the hash was taken is outside what the scorer can see). A score compares the
//! items expected and listed per finding: the items matched, the items missed (with the key's
//! reason), and the items listed that the key did not expect. Figures are not scored; parity
//! covers them.
//!
//! An item is its kind, id and label, as the canonical dump keeps them (`canonical::evidence`): a
//! dump may list two refs with one kind and id that differ by label (vouchers with an empty id
//! among them), and even one ref twice. So items are counted with their repeats: an item the key
//! names twice matches at most two listed copies. A key's id and label are NFC-normalised, as the
//! dump's are; nothing is trimmed.
//!
//! A score describes one run against one key for one synthetic book, and nothing beyond it.
//!
//! The scorer lives under `tests/` because only the crate's tests run it: a module under `src/`
//! would change `src/lib.rs`, which would select every mutation that targets that file (the
//! mutation runner under `parity/`).
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
//! label = ""                       # optional; the dump's label for the ref, exactly
//! reason = "Why the reader expects it listed"
//! ```
//!
//! A finding with no `item` is one the reader expects to list nothing.

use std::collections::BTreeMap;

use bridge_tax_audit::canonical::{hex, nfc};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// The evidence kinds the crate's tests emit (`EvidenceRef::new` and `with_label` across
/// `src/`). A key's kind outside this list does not open, and a dump's kind outside it is not
/// scored, so a misspelt kind is refused rather than counted as a miss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Config,
    DocumentRow,
    ExcludedVoucher,
    Ledger,
    Pan,
    PayeeName,
    Row,
    Rule,
    StockItem,
    Voucher,
}

/// One evidence item: a ref's kind, id and label.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    pub kind: Kind,
    pub id: String,
    pub label: String,
}

/// Why a key does not open.
#[derive(Debug, PartialEq, Eq)]
pub enum KeyError {
    /// The key's bytes do not hash to the sealed sha256.
    SealMismatch {
        sealed: String,
        actual: String,
    },
    NotUtf8,
    /// Not the key format: a missing or unknown field, an unknown kind, or a value of the wrong
    /// type.
    Parse(String),
    /// A required text is empty or only whitespace. Names are never trimmed otherwise.
    Blank(&'static str),
    NoFindings,
    DuplicateFinding(String),
}

/// Why a dump is not scored against a key.
#[derive(Debug, PartialEq, Eq)]
pub enum ScoreError {
    TestIdDiffers {
        key: String,
        dump: String,
    },
    /// The dump is not a canonical dump: a missing field, an unknown kind, or a value of the
    /// wrong type.
    DumpInvalid(String),
    /// The dump holds a finding the key does not name, so the key cannot judge its items.
    DumpFindingNotInKey(String),
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
    kind: Kind,
    id: String,
    #[serde(default)]
    label: String,
    reason: String,
}

/// A key that opened against its seal. Only [`SealedKey::open`] makes one.
#[derive(Debug)]
pub struct SealedKey {
    test_id: String,
    /// finding id -> item -> the reason for each time the key names it.
    findings: BTreeMap<String, BTreeMap<Item, Vec<String>>>,
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
            let mut items: BTreeMap<Item, Vec<String>> = BTreeMap::new();
            for item in finding.item {
                // A voucher ref may have an empty id and only a label; one with neither cannot be
                // told from any other.
                if item.id.trim().is_empty() && item.label.trim().is_empty() {
                    return Err(KeyError::Blank("item.id and item.label"));
                }
                non_blank(&item.reason, "item.reason")?;
                let key = Item {
                    kind: item.kind,
                    id: nfc(&item.id),
                    label: nfc(&item.label),
                };
                items.entry(key).or_default().push(item.reason);
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
    pub item: Item,
    pub reason: String,
}

/// One finding's comparison, each count with repeats. Recall is `matched / expected`, precision
/// `matched / listed`; each is undefined when its denominator is 0.
#[derive(Debug, PartialEq, Eq)]
pub struct FindingScore {
    pub finding_id: String,
    pub expected: usize,
    pub listed: usize,
    pub matched: usize,
    pub missed: Vec<Miss>,
    /// Each listed copy the key does not expect.
    pub extra: Vec<Item>,
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

/// Each finding's listed items, each with how many times it is listed, from a canonical dump.
fn listed_items(dump: &Value) -> Result<BTreeMap<String, BTreeMap<Item, usize>>, ScoreError> {
    let mut listed = BTreeMap::new();
    for finding in array(dump, "findings", "dump")? {
        let id = string(finding, "id", "finding")?.to_string();
        let mut items: BTreeMap<Item, usize> = BTreeMap::new();
        for evidence in array(finding, "evidence", &id)? {
            let kind = string(evidence, "kind", &id)?;
            let kind = serde_json::from_value::<Kind>(Value::from(kind))
                .map_err(|_| ScoreError::DumpInvalid(format!("{id}: unknown kind {kind}")))?;
            let item = Item {
                kind,
                id: string(evidence, "id", &id)?.to_string(),
                label: string(evidence, "label", &id)?.to_string(),
            };
            *items.entry(item).or_default() += 1;
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
        let mut missed = Vec::new();
        let mut matched = 0;
        for (item, reasons) in expected {
            let found = listed.get(item).copied().unwrap_or(0).min(reasons.len());
            matched += found;
            missed.extend(reasons[found..].iter().map(|reason| Miss {
                item: item.clone(),
                reason: reason.clone(),
            }));
        }
        let mut extra = Vec::new();
        for (item, &count) in &listed {
            let wanted = expected.get(item).map_or(0, Vec::len);
            extra.extend(std::iter::repeat_n(
                item.clone(),
                count.saturating_sub(wanted),
            ));
        }
        findings.push(FindingScore {
            finding_id: finding_id.clone(),
            expected: expected.values().map(Vec::len).sum(),
            listed: listed.values().sum(),
            matched,
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
