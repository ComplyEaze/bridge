//! Counterparty → ledger mapping, with a suspense fallback and no guessing.
//!
//! This decides only which **name** a statement party is posted under. Whether
//! that name is a real master, and which master a near-miss spelling was meant
//! to be, is `validate_masters`' job; nothing here matches against Tally.

use crate::bank::PARSER_SENTINELS;
use crate::cash::CashMovement;
use crate::refusal::Refusal;
use crate::text::{mapping_key, squash, strip};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Treatment {
    /// Payment when money leaves the bank, Receipt when it arrives.
    Auto,
    /// Both legs are bank or cash ledgers: an own-account or ATM transfer.
    Contra,
    /// Emit no voucher: a transfer between two accounts that are BOTH being
    /// imported, already carried by the other statement's Contra. The mapping
    /// row names the other account's ledger, so that Contra can be found.
    Skip,
}

impl Treatment {
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "auto" => Some(Self::Auto),
            "contra" => Some(Self::Contra),
            "skip" => Some(Self::Skip),
            _ => None,
        }
    }
}

/// One instruction as supplied, before validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingRow {
    /// Where the row came from, for refusal messages (a CSV line, an index).
    pub origin: String,
    pub party: String,
    pub ledger: String,
    /// `None` means "not given" and defaults to `auto`.
    pub treatment: Option<String>,
}

/// A validated mapping, keyed by [`mapping_key`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Mapping(BTreeMap<String, (String, Treatment)>);

impl Mapping {
    pub fn get(&self, party: &str) -> Option<&(String, Treatment)> {
        self.0.get(&mapping_key(party))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Validate instructions (the record loop of `load_mapping`).
    pub fn from_rows(rows: impl IntoIterator<Item = MappingRow>) -> Result<Self, Refusal> {
        let mut mapping = BTreeMap::new();
        let mut first_origin: BTreeMap<String, String> = BTreeMap::new();
        for row in rows {
            let party = squash(&row.party);
            if party.is_empty() {
                continue;
            }
            let treatment_text = match row.treatment.as_deref() {
                None | Some("") => "auto".to_string(),
                Some(text) => strip(text).to_lowercase(),
            };
            let Some(treatment) = Treatment::parse(&treatment_text) else {
                return Err(Refusal::new(
                    "unknown_treatment",
                    format!(
                        "{}: unknown treatment; use auto, contra or skip",
                        row.origin
                    ),
                ));
            };
            let ledger = squash(&row.ledger);
            if treatment == Treatment::Contra && ledger.is_empty() {
                return Err(Refusal::new(
                    "contra_without_ledger",
                    format!(
                        "{}: marked contra with no ledger. A Contra's other leg must be a bank or cash ledger; defaulting it to suspense would misclassify the transaction.",
                        row.origin
                    ),
                ));
            }
            let key = mapping_key(&party);
            // A cash line's entry depends on what happened to that cash, which
            // differs line by line: never one rule for every such line.
            if CashMovement::of_party(&party).is_some() {
                return Err(Refusal::new(
                    "cash_party_in_mapping",
                    format!(
                        "{}: cash withdrawals and deposits are answered line by line in cash_answers, not mapped: the same statement text covers business cash, the owner's drawings, a customer paying in and cash paid straight to someone",
                        row.origin
                    ),
                ));
            }
            if PARSER_SENTINELS.contains(&key.as_str()) {
                return Err(Refusal::new(
                    "mapping_claims_a_sentinel",
                    format!(
                        "{}: this is a value the parser prints when it could NOT identify a counterparty, not a counterparty name. A row for it would send unrelated transactions to one ledger, or with skip drop them all. Map the individual narrations instead.",
                        row.origin
                    ),
                ));
            }
            // After the sentinel and cash checks, so those rows are named as
            // such first.
            if treatment == Treatment::Skip && ledger.is_empty() {
                return Err(Refusal::new(
                    "skip_without_ledger",
                    format!(
                        "{}: marked skip with no ledger. A skipped transfer must name the other bank account's ledger, so its Contra can be found; a transfer with no other side to find cannot be skipped.",
                        row.origin
                    ),
                ));
            }
            if let Some(existing) = mapping.get(&key) {
                if existing != &(ledger.clone(), treatment) {
                    return Err(Refusal::new(
                        "mapping_key_collision",
                        format!(
                            "{} and {} reduce to the same mapping key but give different instructions; every transaction for both would post under whichever came last",
                            row.origin, first_origin[&key]
                        ),
                    ));
                }
            }
            first_origin.entry(key.clone()).or_insert(row.origin);
            mapping.insert(key, (ledger, treatment));
        }
        Ok(Self(mapping))
    }
}
