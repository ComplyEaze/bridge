//! What a statement build records so the transfer pairing view can be computed
//! from the journal: the skipped rows, the Contras booked from the statement,
//! and the dates the statement read in full.
//!
//! Two records carry it. A saved batch holds `statement` beside its vouchers.
//! A build that posts nothing (every row skipped) holds it in a separate
//! append-only file, one `SkipOnlyRecord` per line in the journal's framing: an
//! older release refuses a journal it finds an unknown record in, so the new
//! record never goes there.
//!
//! The reader checks only the shape (strings, dates, the record split). Every
//! semantic check is `validate_statement_facts`, which the writer runs before it
//! appends and the fact builder runs again when it reads, so a value the writer
//! would not have admitted fails the view loudly instead of pairing wrongly.
//!
//! The fact builder is pure: it folds the journal's records into the plain
//! facts `transfer_pairing_view` takes. A fact it cannot establish is an error
//! that fails the whole view, never a dropped row: losing one lineage's Contras
//! could turn an ambiguous skip into a wrong unique pair.
use super::*;
use crate::agent::bank_statement::ResolvedStatement;
use crate::agent::transfer_pairing::{
    BatchId, BatchStatus, CompanyGuid, ContraFact, ContraKey, ContraLeg, Coverage, Direction,
    EndpointOrigin, LedgerGuid, LineageId, PairingInputError, ParseWindow, Side, SkipFact, SkipKey,
    StatementRowId, TxnId,
};
use bridge_bank_statement::bank::BankReference;
use bridge_bank_statement::proposals::{is_statement_txn_id, Direction as FileDirection};
use chrono::{DateTime, FixedOffset};
use std::io::BufRead;

const POSTED_VERIFIED: &str = "posted_verified";

// ---- the journal records ------------------------------------------------------

/// The dates a statement read in full, both ends inclusive.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct StatementWindow {
    pub(super) from: TallyDate,
    pub(super) to: TallyDate,
}

/// A statement Contra of the batch. The reference is opaque text here.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct ContraRow {
    pub(super) bridge_txn_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) reference: Option<String>,
    /// The cheque or number the statement prints. Recorded only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) instrument: Option<String>,
    /// The statement holds another row on the same date, of an equal amount,
    /// the other way.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) reversed_same_day: bool,
}

/// Which way money moved on the statement's own bank ledger.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum RowDirection {
    Outflow,
    Inflow,
}

/// A row skipped as "recorded from the other bank's statement".
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct SkipRow {
    pub(super) bridge_txn_id: String,
    pub(super) date: TallyDate,
    pub(super) amount: String,
    pub(super) direction: RowDirection,
    pub(super) other_ledger_guid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) instrument: Option<String>,
    /// As on [`ContraRow`].
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(super) reversed_same_day: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct StatementFacts {
    pub(super) bank_ledger_guid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) coverage: Option<StatementWindow>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) contra_rows: Vec<ContraRow>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) skip_rows: Vec<SkipRow>,
}

/// The value a saved batch holds as its `statement`. The batch keeps the JSON,
/// not the type, so a statement a later release wrote cannot make the journal
/// unreadable: only the fact builder parses it. Writers build it here.
pub(super) fn statement_record(facts: &StatementFacts) -> Value {
    json!(facts)
}

/// The facts of a build that posts nothing.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct SkipOnlyRecord {
    /// `skips-` and a version 4 UUID in lower-case hyphenated form.
    pub(super) record_id: String,
    pub(super) company_guid: String,
    pub(super) endpoint_origin: String,
    pub(super) built_at: String,
    pub(super) statement: StatementFacts,
}

/// The skip-only file, beside the import journal.
pub(super) const SKIPS_FILE: &str = "agent-import-statement-skips.jsonl";

impl StatementFacts {
    /// The facts a version 2 proposals file gives, each ledger name taken to the
    /// GUID the build's own catalogue read bound it to. A name the binding does
    /// not hold gives an empty GUID, which `validate_statement_facts` refuses.
    pub(super) fn of_statement(statement: &ResolvedStatement, binding: &[BoundLedger]) -> Self {
        let guid = |name: &str| {
            binding
                .iter()
                .find(|bound| bound.name == name)
                .map(|bound| bound.guid.clone())
                .unwrap_or_default()
        };
        Self {
            bank_ledger_guid: guid(&statement.bank_ledger),
            coverage: statement
                .coverage
                .clone()
                .map(|(from, to)| StatementWindow { from, to }),
            contra_rows: statement
                .contras
                .iter()
                .map(|contra| ContraRow {
                    bridge_txn_id: contra.bridge_txn_id.clone(),
                    reference: contra.reference.as_ref().map(ToString::to_string),
                    instrument: contra.instrument.clone(),
                    reversed_same_day: contra.reversed_same_day,
                })
                .collect(),
            skip_rows: statement
                .skips
                .iter()
                .map(|skip| SkipRow {
                    bridge_txn_id: skip.bridge_txn_id.clone(),
                    date: skip.date.clone(),
                    amount: skip.amount.clone(),
                    direction: match skip.direction {
                        FileDirection::Outflow => RowDirection::Outflow,
                        FileDirection::Inflow => RowDirection::Inflow,
                    },
                    other_ledger_guid: guid(&skip.other_ledger),
                    reference: skip.reference.as_ref().map(ToString::to_string),
                    instrument: skip.instrument.clone(),
                    reversed_same_day: skip.reversed_same_day,
                })
                .collect(),
        }
    }
}

/// Read the skip-only file: the journal's framing, one record per line, each
/// ended by a newline.
pub(super) fn read_skip_records(mut reader: impl BufRead) -> Result<Vec<SkipOnlyRecord>, String> {
    let mut records = Vec::new();
    let mut line = Vec::new();
    while ledger::read_record(&mut reader, &mut line)? {
        let text =
            std::str::from_utf8(&line).map_err(|_| "import_ledger_unavailable".to_string())?;
        records.push(serde_json::from_str(text).map_err(|_| "import_ledger_invalid".to_string())?);
    }
    Ok(records)
}

// ---- the checks ---------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(super) enum FactsError {
    #[error("a ledger GUID is empty")]
    EmptyLedgerGuid,
    #[error("the statement window ends before it starts")]
    InvertedCoverage,
    #[error("one transaction id is named twice")]
    DuplicateTxnId { txn: String },
    #[error("a skipped row's id is not a statement row id")]
    SkipIdNotStatementId { txn: String },
    #[error("a reference is not one a statement row can carry")]
    ReferenceInadmissible { txn: String },
    #[error("a skipped row's amount is not a positive amount of at most two places")]
    AmountInadmissible { txn: String },
    #[error("a skipped row's two ledgers are one")]
    SkipLedgersEqual { txn: String },
    #[error("a Contra row does not name exactly one Contra voucher of the batch")]
    ContraRowNotOneContraVoucher { txn: String },
    #[error("a Contra voucher of the batch is not among the Contra rows")]
    ContraVoucherNotInFacts { txn: String },
    #[error("a skipped row is also a voucher of the batch")]
    SkipIsVoucher { txn: String },
    #[error("the statement's bank ledger is not bound by the batch")]
    BankLedgerUnbound,
    #[error("a batch's statement is not facts this release reads")]
    Unparsable,
    #[error("a skip-only record's id is not `skips-` and a version 4 UUID")]
    BadRecordId,
    #[error("a skip-only record must hold skipped rows and no Contra row")]
    NotSkipOnly,
    #[error("a withdrawn skip is not a statement row the batch posts")]
    WithdrawnRowNotVoucher { txn: String },
}

impl FactsError {
    /// Which check failed, without the value that failed it.
    pub(super) fn variant(&self) -> &'static str {
        match self {
            Self::EmptyLedgerGuid => "empty_ledger_guid",
            Self::InvertedCoverage => "inverted_coverage",
            Self::DuplicateTxnId { .. } => "duplicate_txn_id",
            Self::SkipIdNotStatementId { .. } => "skip_id_not_statement_id",
            Self::ReferenceInadmissible { .. } => "reference_inadmissible",
            Self::AmountInadmissible { .. } => "amount_inadmissible",
            Self::SkipLedgersEqual { .. } => "skip_ledgers_equal",
            Self::ContraRowNotOneContraVoucher { .. } => "contra_row_not_one_contra_voucher",
            Self::ContraVoucherNotInFacts { .. } => "contra_voucher_not_in_facts",
            Self::SkipIsVoucher { .. } => "skip_is_voucher",
            Self::BankLedgerUnbound => "bank_ledger_unbound",
            Self::BadRecordId => "bad_record_id",
            Self::NotSkipOnly => "not_skip_only",
            Self::WithdrawnRowNotVoucher { .. } => "withdrawn_row_not_voucher",
            Self::Unparsable => "unparsable",
        }
    }
}

/// A batch's vouchers and bound ledgers, which the facts must agree with.
pub(super) struct LineShape<'a> {
    pub(super) vouchers: &'a [BuildVoucher],
    pub(super) ledger_identities: Option<&'a [BoundLedger]>,
}

/// Facts that passed every check, as typed values.
#[derive(Debug)]
pub(super) struct ValidFacts {
    pub(super) bank_ledger: LedgerGuid,
    pub(super) window: Option<ParseWindow>,
    /// Each Contra row's id and reference.
    pub(super) contras: BTreeMap<String, Option<BankReference>>,
    /// The Contra rows whose statement reversed them the same day.
    pub(super) reversed_contras: BTreeSet<String>,
    pub(super) skips: Vec<ValidSkip>,
}

#[derive(Debug)]
pub(super) struct ValidSkip {
    txn: String,
    date: TallyDate,
    amount: ExactDecimal,
    direction: Direction,
    other_ledger: LedgerGuid,
    reference: Option<BankReference>,
    reversed_same_day: bool,
}

fn admitted_reference(txn: &str, text: Option<&str>) -> Result<Option<BankReference>, FactsError> {
    text.map(|text| {
        BankReference::from_recorded(text).ok_or_else(|| FactsError::ReferenceInadmissible {
            txn: txn.to_string(),
        })
    })
    .transpose()
}

/// A positive amount with at most two decimal places, as written.
fn admitted_amount(txn: &str, text: &str) -> Result<ExactDecimal, FactsError> {
    let places = text
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    ExactDecimal::parse(text)
        .ok()
        .filter(|amount| places <= 2 && !amount.is_zero() && !amount.is_negative())
        .ok_or_else(|| FactsError::AmountInadmissible {
            txn: txn.to_string(),
        })
}

/// Every semantic check on one statement's facts. With the batch's own shape
/// the facts must also agree with its vouchers and bound ledgers.
pub(super) fn validate_statement_facts(
    facts: &StatementFacts,
    shape: Option<LineShape<'_>>,
) -> Result<ValidFacts, FactsError> {
    if facts.bank_ledger_guid.is_empty() {
        return Err(FactsError::EmptyLedgerGuid);
    }
    let window = match &facts.coverage {
        Some(window) if window.from > window.to => return Err(FactsError::InvertedCoverage),
        Some(window) => Some(ParseWindow {
            from: window.from.clone(),
            to: window.to.clone(),
        }),
        None => None,
    };
    let mut seen = BTreeSet::new();
    let mut contras = BTreeMap::new();
    let mut reversed_contras = BTreeSet::new();
    for row in &facts.contra_rows {
        let txn = row.bridge_txn_id.as_str();
        if !seen.insert(txn) {
            return Err(FactsError::DuplicateTxnId { txn: txn.into() });
        }
        contras.insert(
            txn.to_string(),
            admitted_reference(txn, row.reference.as_deref())?,
        );
        if row.reversed_same_day {
            reversed_contras.insert(txn.to_string());
        }
    }
    let mut skips = Vec::new();
    for row in &facts.skip_rows {
        let txn = row.bridge_txn_id.as_str();
        if !seen.insert(txn) {
            return Err(FactsError::DuplicateTxnId { txn: txn.into() });
        }
        if !is_statement_txn_id(txn) {
            return Err(FactsError::SkipIdNotStatementId { txn: txn.into() });
        }
        let reference = admitted_reference(txn, row.reference.as_deref())?;
        let amount = admitted_amount(txn, &row.amount)?;
        if row.other_ledger_guid.is_empty() {
            return Err(FactsError::EmptyLedgerGuid);
        }
        if row.other_ledger_guid == facts.bank_ledger_guid {
            return Err(FactsError::SkipLedgersEqual { txn: txn.into() });
        }
        skips.push(ValidSkip {
            txn: txn.to_string(),
            date: row.date.clone(),
            amount,
            direction: match row.direction {
                RowDirection::Outflow => Direction::Outflow,
                RowDirection::Inflow => Direction::Inflow,
            },
            other_ledger: LedgerGuid::new(row.other_ledger_guid.as_str()),
            reference,
            reversed_same_day: row.reversed_same_day,
        });
    }
    if let Some(shape) = shape {
        let contra_vouchers = |txn: &str| {
            shape
                .vouchers
                .iter()
                .filter(|voucher| voucher.kind == VoucherType::Contra && voucher.txn_id == txn)
                .count()
        };
        for txn in contras.keys() {
            if contra_vouchers(txn) != 1 {
                return Err(FactsError::ContraRowNotOneContraVoucher { txn: txn.clone() });
            }
        }
        for voucher in shape.vouchers {
            if voucher.kind == VoucherType::Contra && !contras.contains_key(&voucher.txn_id) {
                return Err(FactsError::ContraVoucherNotInFacts {
                    txn: voucher.txn_id.clone(),
                });
            }
        }
        for skip in &skips {
            if shape
                .vouchers
                .iter()
                .any(|voucher| voucher.txn_id == skip.txn)
            {
                return Err(FactsError::SkipIsVoucher {
                    txn: skip.txn.clone(),
                });
            }
        }
        if !shape.ledger_identities.is_some_and(|identities| {
            identities
                .iter()
                .any(|bound| bound.guid == facts.bank_ledger_guid)
        }) {
            return Err(FactsError::BankLedgerUnbound);
        }
    }
    Ok(ValidFacts {
        bank_ledger: LedgerGuid::new(facts.bank_ledger_guid.as_str()),
        window,
        contras,
        reversed_contras,
        skips,
    })
}

/// The checks on a skip-only record: its id, that it posts nothing, and its
/// facts. Its endpoint and time are checked where they are used.
pub(super) fn validate_skip_record(record: &SkipOnlyRecord) -> Result<ValidFacts, FactsError> {
    let canonical_id = record
        .record_id
        .strip_prefix("skips-")
        .and_then(|rest| Uuid::parse_str(rest).ok())
        .is_some_and(|id| {
            id.get_version_num() == 4 && format!("skips-{}", id.hyphenated()) == record.record_id
        });
    if !canonical_id {
        return Err(FactsError::BadRecordId);
    }
    if !record.statement.contra_rows.is_empty() || record.statement.skip_rows.is_empty() {
        return Err(FactsError::NotSkipOnly);
    }
    validate_statement_facts(&record.statement, None)
}

// ---- what the journal says ----------------------------------------------------

/// What a batch record keeps of a voucher: the rows a transfer can be made of.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BuildVoucher {
    pub(super) txn_id: String,
    pub(super) date: TallyDate,
    pub(super) kind: VoucherType,
    pub(super) entries: Vec<BuildEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BuildEntry {
    pub(super) ledger: String,
    pub(super) amount: String,
    pub(super) side: EntrySide,
}

/// A full batch record, cut down to what the facts need: its Contras, its
/// Journals and its statement rows. Narrations and every other voucher are left
/// behind, since the journal can hold years of batches.
#[derive(Clone, Debug)]
pub(super) struct BuildLine {
    pub(super) batch_id: String,
    pub(super) lineage: String,
    pub(super) company_guid: String,
    pub(super) endpoint_origin: Option<String>,
    pub(super) built_at: String,
    pub(super) status: String,
    pub(super) vouchers: Vec<BuildVoucher>,
    pub(super) ledger_identities: Option<Vec<BoundLedger>>,
    pub(super) statement: Option<Value>,
    /// The statement rows the batch posts although a skip names them.
    pub(super) withdrawn: BTreeSet<String>,
}

impl BuildLine {
    pub(super) fn of(line: &ImportLedgerLine) -> Self {
        Self {
            batch_id: line.batch_id.clone(),
            lineage: line.identity_batch_id().to_string(),
            company_guid: line.company_guid.clone(),
            endpoint_origin: line.endpoint_origin.clone(),
            built_at: line.built_at.clone(),
            status: line.status.clone(),
            vouchers: BuildVoucher::of_vouchers(&line.vouchers),
            ledger_identities: line.ledger_identities.clone(),
            statement: line.statement.clone(),
            withdrawn: line.withdrawn_skips.iter().flatten().cloned().collect(),
        }
    }

    /// The same check at write and at read: a withdrawn row is a statement row
    /// the batch holds as a voucher.
    fn check_withdrawn(&self) -> Result<(), TransferFactsError> {
        match self.withdrawn.iter().find(|row| {
            !is_statement_txn_id(row)
                || !self.vouchers.iter().any(|voucher| &voucher.txn_id == *row)
        }) {
            Some(row) => Err(TransferFactsError::Facts {
                source_id: self.batch_id.clone(),
                error: FactsError::WithdrawnRowNotVoucher { txn: row.clone() },
            }),
            None => Ok(()),
        }
    }
}

impl BuildVoucher {
    /// The vouchers a transfer can be made of: Contras, Journals and statement
    /// rows. Reference §9.13 records one Journal between two bank ledgers
    /// accepted (PARTIAL, one run), so a Journal that names two statement banks
    /// is treated as a possible transfer.
    pub(super) fn of_vouchers(vouchers: &[ImportVoucher]) -> Vec<Self> {
        vouchers
            .iter()
            .filter(|voucher| {
                matches!(
                    voucher.voucher_type,
                    VoucherType::Contra | VoucherType::Journal
                ) || is_statement_txn_id(&voucher.bridge_txn_id)
            })
            .map(|voucher| Self {
                txn_id: voucher.bridge_txn_id.clone(),
                date: voucher.date.clone(),
                kind: voucher.voucher_type.clone(),
                entries: voucher
                    .entries
                    .iter()
                    .map(|entry| BuildEntry {
                        ledger: entry.ledger.clone(),
                        amount: entry.amount.clone(),
                        side: entry.side.clone(),
                    })
                    .collect(),
            })
            .collect()
    }
}

/// A record about to be written, read as the newest one.
pub(super) enum Candidate {
    Build(Box<BuildLine>),
    SkipOnly(Box<SkipOnlyRecord>),
}

/// One journal record, in journal order, as far as the facts need it.
pub(super) enum JournalEvent {
    Build(Box<BuildLine>),
    /// A record that sets a batch's status; `sent` for a dispatch intent.
    Status {
        batch_id: String,
        status: String,
        sent: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(super) enum TransferFactsError {
    #[error("the import journal could not be read")]
    JournalUnreadable { code: String },
    #[error("the statement skip file could not be read")]
    SkipFileUnreadable { code: String },
    #[error("a record's facts failed a check")]
    Facts {
        source_id: String,
        error: FactsError,
    },
    #[error("a record that holds facts or decides a Contra has no endpoint")]
    MissingEndpoint { source_id: String },
    #[error("a build time is not an RFC 3339 time")]
    BuiltAtUnparsable { source_id: String },
    #[error("a later full record of a batch differs from its first")]
    BatchRecordChanged { batch_id: String },
    #[error("two skip-only records share an id")]
    DuplicateSkipRecordId { record_id: String },
    #[error("a skipped row is also a voucher of its own lineage")]
    SkipIsVoucherInLineage { lineage: String, txn: String },
    #[error("a Contra names a ledger its build did not bind")]
    LegLedgerUnbound { batch_id: String, ledger: String },
    #[error(transparent)]
    Pairing(#[from] PairingInputError),
}

impl TransferFactsError {
    /// Which failure, without any value the record held.
    pub(super) fn variant(&self) -> &'static str {
        match self {
            Self::JournalUnreadable { .. } => "journal_unreadable",
            Self::SkipFileUnreadable { .. } => "skip_file_unreadable",
            Self::Facts { .. } => "facts",
            Self::MissingEndpoint { .. } => "missing_endpoint",
            Self::BuiltAtUnparsable { .. } => "built_at_unparsable",
            Self::BatchRecordChanged { .. } => "batch_record_changed",
            Self::DuplicateSkipRecordId { .. } => "duplicate_skip_record_id",
            Self::SkipIsVoucherInLineage { .. } => "skip_is_voucher_in_lineage",
            Self::LegLedgerUnbound { .. } => "leg_ledger_unbound",
            Self::Pairing(error) => match error {
                PairingInputError::DuplicateSkip { .. } => "pairing_duplicate_skip",
                PairingInputError::DuplicateContra { .. } => "pairing_duplicate_contra",
                PairingInputError::NonPositiveSkipAmount { .. } => "pairing_non_positive_skip",
                PairingInputError::NonPositiveContraAmount { .. } => "pairing_non_positive_contra",
                PairingInputError::SkipLedgersEqual { .. } => "pairing_skip_ledgers_equal",
                PairingInputError::InvertedWindow { .. } => "pairing_inverted_window",
            },
        }
    }

    /// The batch or skip-only record whose facts failed, when one did. A
    /// journal-wide failure, or one in a ledger or a row, names none.
    pub(super) fn record_id(&self) -> Option<&str> {
        match self {
            Self::Facts { source_id, .. }
            | Self::MissingEndpoint { source_id }
            | Self::BuiltAtUnparsable { source_id } => Some(source_id),
            Self::BatchRecordChanged { batch_id } | Self::LegLedgerUnbound { batch_id, .. } => {
                Some(batch_id)
            }
            Self::DuplicateSkipRecordId { record_id } => Some(record_id),
            Self::SkipIsVoucherInLineage { lineage, .. } => Some(lineage),
            Self::JournalUnreadable { .. } | Self::SkipFileUnreadable { .. } | Self::Pairing(_) => {
                None
            }
        }
    }

    /// The failed check of a facts failure.
    pub(super) fn check(&self) -> Option<&'static str> {
        match self {
            Self::Facts { error, .. } => Some(error.variant()),
            _ => None,
        }
    }
}

/// The facts `transfer_pairing_view` takes, and what was set aside on the way.
#[derive(Debug)]
pub(super) struct TransferFacts {
    pub(super) skips: Vec<SkipFact>,
    pub(super) contras: Vec<ContraFact>,
    pub(super) coverage: Coverage,
    /// Skips dropped because a batch found posted holds the row and recorded
    /// that a person withdrew the skip.
    pub(super) retired_skips: usize,
    /// Skips whose row a batch holds as a voucher, and that are not retired.
    pub(super) held: Vec<HeldSkip>,
    /// The company's batches a recorded rollback covers (#1050): any row that
    /// names one is marked restored.
    pub(super) restored: BTreeSet<String>,
    /// Statement Contras left out because they are not two ledgers with equal
    /// positive amounts, or have no leg on the statement's bank.
    // Counted for the tests; no tool reports it, since the coverage rule already
    // withholds `not_found` and `tally_unchecked` (the row reads `not_established`)
    // where such a Contra sits.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) contras_not_two_leg: usize,
    /// Contras of a lineage that holds statement facts, with no facts entry in
    /// any of its builds: not seen, and the days they sit on are not cover.
    // Counted for the tests; no tool reports it, since the coverage rule already
    // withholds `not_found` and `tally_unchecked` (the row reads `not_established`)
    // where such a Contra sits.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) contras_without_facts: usize,
    /// Contras of a build never sent and never found, whose row a sent or found
    /// batch of another lineage holds.
    // Counted for the tests; no tool reports it, since the coverage rule already
    // withholds `not_found` and `tally_unchecked` (the row reads `not_established`)
    // where such a Contra sits.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) contras_superseded: usize,
    /// The skips, kept or held elsewhere, that some build of the company which
    /// mentions them was sent or found posted, or that a skip-only record holds:
    /// a skip a person has acted on. A skip only built is superseded by the
    /// next build of its statement and is not live.
    pub(super) live_skips: BTreeSet<SkipKey>,
    /// Every skip the company still has, live or not, kept or held: the skips a
    /// build that posts the row may withdraw. A retired skip is not one.
    pub(super) skip_mentions: BTreeSet<SkipKey>,
    /// When the newest record of each skip in `skip_mentions` was built.
    pub(super) skip_built: BTreeMap<SkipKey, DateTime<FixedOffset>>,
}

/// What is true of the batch that holds a skipped row as a voucher.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HeldBy {
    /// No batch holding the row was found posted: it may yet be, and until then
    /// the skip is not established.
    UnverifiedBatch,
    /// A batch found posted holds the row, and no build recorded that a person
    /// withdrew the skip: the posted voucher contradicts the skip, and the
    /// other account's Contra may carry the same transfer.
    PostedBatch,
}

/// A skipped row that a batch holds as a voucher.
#[derive(Debug, Clone)]
pub(super) struct HeldSkip {
    pub(super) fact: SkipFact,
    /// The batch that holds the row: the latest one, or for a posted holder
    /// the first found posted that did not record the withdrawal.
    pub(super) holder: BatchId,
    pub(super) by: HeldBy,
    /// Whether the skip is judged against the Contras when a person names it
    /// in `withdraw_skips`. Not for a skip held only because a recorded
    /// rollback covers the batch that would have retired it (#1050): master
    /// retired it, so it pairs with nothing and is no skip to withdraw.
    pub(super) pairs: bool,
}

/// A batch as the journal leaves it.
struct Batch {
    line: BuildLine,
    /// The checked facts of a batch that holds them, and when it was built.
    facts: Option<(ValidFacts, DateTime<FixedOffset>)>,
    status: String,
    sent: bool,
    /// Ever read `posted_verified`, and not covered by a recorded rollback.
    found: bool,
    /// Ever read `posted_verified`, covered or not.
    ever_found: bool,
    /// A recorded rollback covers it (#1050): its vouchers may have been
    /// removed, so it is neither found nor verified, whatever its statuses.
    covered: bool,
}

impl Batch {
    fn is_verified(&self) -> bool {
        self.status == POSTED_VERIFIED && !self.covered
    }

    fn endpoint(&self) -> Result<&str, TransferFactsError> {
        self.line
            .endpoint_origin
            .as_deref()
            .ok_or_else(|| TransferFactsError::MissingEndpoint {
                source_id: self.line.batch_id.clone(),
            })
    }
}

pub(super) fn built_instant(
    source_id: &str,
    text: &str,
) -> Result<DateTime<FixedOffset>, TransferFactsError> {
    DateTime::parse_from_rfc3339(text).map_err(|_| TransferFactsError::BuiltAtUnparsable {
        source_id: source_id.to_string(),
    })
}

/// Whether a withdrawal built at `recorded` counts against a skip whose newest
/// record was built at `mention_built`: only a withdrawal built strictly after
/// it does. One built at the very instant cannot be told from one made before
/// the skip, so it does not count. The facts apply this to a posted holder; the
/// check at send applies it to the batch being sent.
pub(super) fn withdrawal_counts(
    recorded: &DateTime<FixedOffset>,
    mention_built: &DateTime<FixedOffset>,
) -> bool {
    recorded > mention_built
}

/// The facts a batch's record holds, checked against that record, and the
/// instant it was built.
fn admit_facts(
    line: &BuildLine,
) -> Result<Option<(ValidFacts, DateTime<FixedOffset>)>, TransferFactsError> {
    let Some(recorded) = &line.statement else {
        return Ok(None);
    };
    let refused = |error| TransferFactsError::Facts {
        source_id: line.batch_id.clone(),
        error,
    };
    let statement: StatementFacts =
        serde_json::from_value(recorded.clone()).map_err(|_| refused(FactsError::Unparsable))?;
    let facts = validate_statement_facts(
        &statement,
        Some(LineShape {
            vouchers: &line.vouchers,
            ledger_identities: line.ledger_identities.as_deref(),
        }),
    )
    .map_err(refused)?;
    Ok(Some((
        facts,
        built_instant(&line.batch_id, &line.built_at)?,
    )))
}

/// The company's batches in the order of their first journal record, each
/// holding its latest full record and its status as `read_snapshot` folds it.
fn fold_batches(
    events: Vec<JournalEvent>,
    company_guid: &str,
    covered: &BTreeSet<String>,
) -> Result<Vec<Batch>, TransferFactsError> {
    let mut batches: Vec<Batch> = Vec::new();
    let mut index: BTreeMap<String, usize> = BTreeMap::new();
    for event in events {
        match event {
            JournalEvent::Build(line) if line.company_guid.eq_ignore_ascii_case(company_guid) => {
                match index.get(&line.batch_id) {
                    Some(&at) => {
                        let batch = &mut batches[at];
                        if batch.line.vouchers != line.vouchers
                            || batch.line.statement != line.statement
                            || batch.line.withdrawn != line.withdrawn
                        {
                            return Err(TransferFactsError::BatchRecordChanged {
                                batch_id: line.batch_id,
                            });
                        }
                        // the batch keeps its place, and its content is the latest record's
                        batch.found |= line.status == POSTED_VERIFIED;
                        batch.ever_found |= line.status == POSTED_VERIFIED;
                        batch.status = line.status.clone();
                        batch.facts = admit_facts(&line)?;
                        batch.line = *line;
                    }
                    None => {
                        let line = *line;
                        line.check_withdrawn()?;
                        let facts = admit_facts(&line)?;
                        index.insert(line.batch_id.clone(), batches.len());
                        batches.push(Batch {
                            status: line.status.clone(),
                            found: line.status == POSTED_VERIFIED,
                            ever_found: line.status == POSTED_VERIFIED,
                            line,
                            facts,
                            sent: false,
                            covered: false,
                        });
                    }
                }
            }
            JournalEvent::Build(_) => {}
            JournalEvent::Status {
                batch_id,
                status,
                sent,
            } => {
                if let Some(&at) = index.get(&batch_id) {
                    let batch = &mut batches[at];
                    batch.found |= status == POSTED_VERIFIED;
                    batch.ever_found |= status == POSTED_VERIFIED;
                    batch.sent |= sent;
                    batch.status = status;
                }
            }
        }
    }
    for batch in &mut batches {
        if covered.contains(&batch.line.batch_id) {
            batch.covered = true;
            batch.found = false;
        }
    }
    Ok(batches)
}

/// The batches that hold one row as a voucher, in journal order.
struct Held {
    latest: usize,
    batches: Vec<usize>,
}

/// A skip as one source recorded it, with what orders it against the others.
struct SkipMention {
    fact: SkipFact,
    /// The row's `st-` id, which a voucher of a batch can also carry.
    row: String,
    built: DateTime<FixedOffset>,
    source_id: String,
    /// Some source that mentions the row was sent, found posted, or is a
    /// skip-only record.
    live: bool,
}

impl SkipMention {
    /// Keep the newer of two mentions of one row. Liveness is a property of
    /// the row, not of the mention kept: if either was acted on, it is.
    fn merge_into(self, mentions: &mut BTreeMap<SkipKey, SkipMention>) {
        let key = self.fact.key.clone();
        match mentions.get_mut(&key) {
            Some(old) => {
                let live = old.live || self.live;
                if (&self.built, &self.source_id) > (&old.built, &old.source_id) {
                    *old = self;
                }
                old.live = live;
            }
            None => {
                mentions.insert(key, self);
            }
        }
    }
}

/// A statement Contra with the build that decided it.
struct DecidedContra {
    fact: ContraFact,
    txn: String,
    deciding: usize,
}

/// What one lineage contributes.
#[derive(Default)]
struct LineageFacts {
    contras: Vec<DecidedContra>,
    skips: BTreeMap<SkipKey, SkipMention>,
    left_out: usize,
    /// Contras with no facts entry in a lineage that holds facts.
    without_facts: usize,
    /// Each build's voucher of such a Contra, by batch and voucher position:
    /// it blocks every ledger it names, as a batch without facts does.
    unseen: Vec<(usize, usize)>,
    /// Every build that decides one of the lineage's voucher rows is verified.
    all_verified: bool,
    /// The days of every row the facts cannot vouch for: decided by a build
    /// not verified, a Contra left out, or a Contra with no facts entry. A row
    /// counts on every day any build of the lineage gave it.
    blocked: BTreeSet<TallyDate>,
    /// The endpoint and bank ledger of each statement the lineage holds.
    ledgers: BTreeSet<(EndpointOrigin, LedgerGuid)>,
}

/// The legs of a Contra that is two ledgers, one Dr and one Cr, with equal
/// positive amounts and one leg on the statement's bank. `Ok(None)` for one
/// that is not; an error for a ledger the build did not bind.
fn contra_legs(
    batch: &Batch,
    voucher: &BuildVoucher,
    bank_ledger: &LedgerGuid,
) -> Result<Option<([ContraLeg; 2], ExactDecimal)>, TransferFactsError> {
    let [first, second] = voucher.entries.as_slice() else {
        return Ok(None);
    };
    if voucher.kind != VoucherType::Contra || first.side == second.side {
        return Ok(None);
    }
    let amount = match (
        canonical_verification_amount(&first.amount),
        canonical_verification_amount(&second.amount),
    ) {
        (Ok(left), Ok(right)) if left == right => ExactDecimal::parse(left).ok(),
        _ => None,
    };
    let Some(amount) = amount.filter(|amount| !amount.is_zero() && !amount.is_negative()) else {
        return Ok(None);
    };
    let leg = |entry: &BuildEntry| {
        batch
            .line
            .ledger_identities
            .iter()
            .flatten()
            .find(|bound| bound.name == entry.ledger)
            .map(|bound| ContraLeg {
                ledger: LedgerGuid::new(bound.guid.as_str()),
                side: match entry.side {
                    EntrySide::Dr => Side::Debit,
                    EntrySide::Cr => Side::Credit,
                },
            })
            .ok_or_else(|| TransferFactsError::LegLedgerUnbound {
                batch_id: batch.line.batch_id.clone(),
                ledger: entry.ledger.clone(),
            })
    };
    let legs = [leg(first)?, leg(second)?];
    Ok(
        (legs[0].ledger != legs[1].ledger && legs.iter().any(|leg| &leg.ledger == bank_ledger))
            .then_some((legs, amount)),
    )
}

/// One lineage that holds facts: its builds in journal order.
fn lineage_facts(
    batches: &[Batch],
    members: &[usize],
    company: &CompanyGuid,
) -> Result<LineageFacts, TransferFactsError> {
    let lineage_text = batches[members[0]].line.lineage.clone();
    let lineage = LineageId::new(lineage_text.as_str());
    // the newest build to mention a row as a voucher decides its content
    let mut deciding: BTreeMap<&str, (usize, &BuildVoucher)> = BTreeMap::new();
    for &at in members {
        for voucher in &batches[at].line.vouchers {
            deciding.insert(voucher.txn_id.as_str(), (at, voucher));
        }
    }
    // the newest facts entry for a Contra row gives its reference and bank
    let mut entries: BTreeMap<&str, (&LedgerGuid, &Option<BankReference>, bool)> = BTreeMap::new();
    let mut skips: BTreeMap<SkipKey, SkipMention> = BTreeMap::new();
    let mut ledgers = BTreeSet::new();
    for &at in members {
        let batch = &batches[at];
        let Some((facts, built)) = &batch.facts else {
            continue;
        };
        ledgers.insert((
            EndpointOrigin::new(batch.endpoint()?),
            facts.bank_ledger.clone(),
        ));
        for (txn, reference) in &facts.contras {
            entries.insert(
                txn.as_str(),
                (
                    &facts.bank_ledger,
                    reference,
                    facts.reversed_contras.contains(txn),
                ),
            );
        }
        for skip in &facts.skips {
            if deciding.contains_key(skip.txn.as_str()) {
                return Err(TransferFactsError::SkipIsVoucherInLineage {
                    lineage: lineage_text,
                    txn: skip.txn.clone(),
                });
            }
            let key = SkipKey {
                company: company.clone(),
                bank_ledger: facts.bank_ledger.clone(),
                endpoint_origin: EndpointOrigin::new(batch.endpoint()?),
                row: StatementRowId::new(skip.txn.as_str()),
            };
            let live =
                batch.sent || batch.found || skips.get(&key).is_some_and(|earlier| earlier.live);
            skips.insert(
                key.clone(),
                SkipMention {
                    live,
                    fact: SkipFact {
                        key,
                        date: skip.date.clone(),
                        amount: skip.amount.clone(),
                        direction: skip.direction,
                        other_ledger: skip.other_ledger.clone(),
                        reference: skip.reference.clone(),
                        lineage: lineage.clone(),
                        reversed_same_day: skip.reversed_same_day,
                    },
                    row: skip.txn.clone(),
                    built: *built,
                    source_id: batch.line.batch_id.clone(),
                },
            );
        }
    }
    let mut uncertain: BTreeSet<&str> = deciding
        .iter()
        .filter(|(_, (at, _))| !batches[*at].is_verified())
        .map(|(&txn, _)| txn)
        .collect();
    let mut unseen: BTreeSet<&str> = BTreeSet::new();
    let mut found = LineageFacts {
        skips,
        all_verified: uncertain.is_empty(),
        ledgers,
        ..LineageFacts::default()
    };
    for (&txn, &(decider, voucher)) in &deciding {
        let batch = &batches[decider];
        // a Contra with no facts entry in any build is not a statement Contra,
        // in a lineage that holds facts it is one the facts do not see
        let Some(&(bank_ledger, reference, reversed_same_day)) = entries.get(txn) else {
            if voucher.kind == VoucherType::Contra {
                found.without_facts += 1;
                uncertain.insert(txn);
                unseen.insert(txn);
            } else if voucher.kind == VoucherType::Journal {
                // No facts entry ever records a Journal, and one can move money
                // between two banks. A lineage Bridge built cannot reach this
                // (the statement parser writes no Journal, and an amendment adds
                // no row and changes no type); it guards a journal written
                // otherwise.
                unseen.insert(txn);
            }
            continue;
        };
        let Some((legs, amount)) = contra_legs(batch, voucher, bank_ledger)? else {
            // its ledgers need not include the statement's bank: they are
            // unseen like a Contra without an entry
            found.left_out += 1;
            uncertain.insert(txn);
            unseen.insert(txn);
            continue;
        };
        found.contras.push(DecidedContra {
            fact: ContraFact {
                key: ContraKey {
                    batch: BatchId::new(batch.line.batch_id.as_str()),
                    txn: TxnId::new(txn),
                },
                company: company.clone(),
                endpoint_origin: EndpointOrigin::new(batch.endpoint()?),
                legs,
                amount,
                date: voucher.date.clone(),
                reference: reference.clone(),
                statement_bank_ledger: bank_ledger.clone(),
                lineage: lineage.clone(),
                status: if batch.is_verified() {
                    BatchStatus::PostedVerified
                } else {
                    BatchStatus::NotVerified {
                        batch: BatchId::new(batch.line.batch_id.as_str()),
                    }
                },
                reversed_same_day,
            },
            txn: txn.to_string(),
            deciding: decider,
        });
    }
    for &at in members {
        for (position, voucher) in batches[at].line.vouchers.iter().enumerate() {
            if uncertain.contains(voucher.txn_id.as_str()) {
                found.blocked.insert(voucher.date.clone());
            }
            if unseen.contains(voucher.txn_id.as_str()) {
                found.unseen.push((at, position));
            }
        }
    }
    Ok(found)
}

/// The days no window may vouch for, by endpoint and bank ledger; `None` is any.
type Blocked = BTreeMap<(Option<EndpointOrigin>, Option<LedgerGuid>), BTreeSet<TallyDate>>;

/// The ledgers some statement of the company was read for. A window covers
/// only such a ledger, and a skipped row is on such a ledger, so a voucher
/// moving a skipped row's money to a bank whose window could vouch for its
/// absence names two of them.
fn statement_banks<'a>(
    batches: &[Batch],
    skip_records: impl Iterator<Item = &'a SkipOnlyRecord>,
) -> BTreeSet<LedgerGuid> {
    let mut banks: BTreeSet<LedgerGuid> = batches
        .iter()
        .filter_map(|batch| batch.facts.as_ref())
        .map(|(facts, _)| facts.bank_ledger.clone())
        .collect();
    banks.extend(
        skip_records.map(|record| LedgerGuid::new(record.statement.bank_ledger_guid.as_str())),
    );
    banks
}

/// A batch of a lineage that holds no facts (built inline, or by a release
/// before them) can still book a transfer: a withdrawn skip posted as a
/// Contra, a Contra keyed by hand, a Journal between two banks. Each Contra and
/// statement row it holds, and each Journal that may be a transfer, is one the
/// facts cannot see, so its date, and for a statement row the date its id
/// carries, are not cover for any ledger it names; where the endpoint or a
/// ledger is not bound, for any.
fn block_unseen(batch: &Batch, banks: &BTreeSet<LedgerGuid>, blocked: &mut Blocked) {
    for voucher in &batch.line.vouchers {
        block_voucher(batch, voucher, banks, blocked);
    }
}

fn block_voucher(
    batch: &Batch,
    voucher: &BuildVoucher,
    banks: &BTreeSet<LedgerGuid>,
    blocked: &mut Blocked,
) {
    let ledgers: Vec<Option<LedgerGuid>> = voucher
        .entries
        .iter()
        .map(|entry| {
            batch
                .line
                .ledger_identities
                .iter()
                .flatten()
                .find(|bound| bound.name == entry.ledger)
                .map(|bound| LedgerGuid::new(bound.guid.as_str()))
        })
        .collect();
    // A Journal with one bank leg (charges, interest, TDS) is no transfer: it
    // blocks only when it names two statement banks, or a ledger not bound,
    // which may be one.
    if voucher.kind == VoucherType::Journal {
        let named: BTreeSet<&LedgerGuid> = ledgers
            .iter()
            .flatten()
            .filter(|ledger| banks.contains(*ledger))
            .collect();
        if named.len() < 2 && ledgers.iter().all(Option::is_some) {
            return;
        }
    }
    let endpoint = batch
        .line
        .endpoint_origin
        .as_deref()
        .map(EndpointOrigin::new);
    let mut days = vec![voucher.date.clone()];
    if is_statement_txn_id(&voucher.txn_id) {
        // The id was checked to be `st-`, eight digits and the rest. Eight
        // digits that are not a date add nothing: the voucher's date blocks.
        days.extend(TallyDate::parse(voucher.txn_id[3..11].to_string()).ok());
    }
    for ledger in ledgers {
        blocked
            .entry((endpoint.clone(), ledger))
            .or_default()
            .extend(days.iter().cloned());
    }
}

/// Fold the journal's records and the skip-only file into the facts of one
/// company. Records of any other company are neither read nor checked.
pub(super) fn build_transfer_facts(
    events: Vec<JournalEvent>,
    skip_records: Vec<SkipOnlyRecord>,
    company_guid: &str,
    covered: &BTreeSet<String>,
) -> Result<TransferFacts, TransferFactsError> {
    let company = CompanyGuid::new(canonical_batch_guid(company_guid));
    let batches = fold_batches(events, company_guid, covered)?;
    let mut lineages: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (at, batch) in batches.iter().enumerate() {
        lineages
            .entry(batch.line.lineage.as_str())
            .or_default()
            .push(at);
    }
    // every batch holding a row as a voucher, whatever its lineage
    let mut holders: BTreeMap<&str, Held> = BTreeMap::new();
    for (at, batch) in batches.iter().enumerate() {
        for voucher in &batch.line.vouchers {
            let held = holders
                .entry(voucher.txn_id.as_str())
                .or_insert_with(|| Held {
                    latest: at,
                    batches: Vec::new(),
                });
            held.latest = at;
            held.batches.push(at);
        }
    }

    let mut contras: Vec<DecidedContra> = Vec::new();
    let mut mentions: BTreeMap<SkipKey, SkipMention> = BTreeMap::new();
    let mut windows: Vec<(EndpointOrigin, LedgerGuid, ParseWindow)> = Vec::new();
    let mut contras_not_two_leg = 0;
    let mut contras_without_facts = 0;
    let mut blocked = Blocked::new();
    let banks = statement_banks(
        &batches,
        skip_records
            .iter()
            .filter(|record| record.company_guid.eq_ignore_ascii_case(company_guid)),
    );
    for members in lineages.values() {
        if members.iter().all(|&at| batches[at].facts.is_none()) {
            for &at in members {
                block_unseen(&batches[at], &banks, &mut blocked);
            }
            continue;
        }
        let found = lineage_facts(&batches, members, &company)?;
        for &(at, position) in &found.unseen {
            block_voucher(
                &batches[at],
                &batches[at].line.vouchers[position],
                &banks,
                &mut blocked,
            );
        }
        contras_not_two_leg += found.left_out;
        contras_without_facts += found.without_facts;
        for (endpoint, ledger) in &found.ledgers {
            blocked
                .entry((Some(endpoint.clone()), Some(ledger.clone())))
                .or_default()
                .extend(found.blocked.iter().cloned());
        }
        if found.all_verified && found.left_out == 0 && found.without_facts == 0 {
            for &at in members {
                let batch = &batches[at];
                if let Some((facts, _)) = &batch.facts {
                    if let Some(window) = &facts.window {
                        windows.push((
                            EndpointOrigin::new(batch.endpoint()?),
                            facts.bank_ledger.clone(),
                            window.clone(),
                        ));
                    }
                }
            }
        }
        contras.extend(found.contras);
        for mention in found.skips.into_values() {
            mention.merge_into(&mut mentions);
        }
    }

    let mut record_ids = BTreeSet::new();
    for record in skip_records
        .iter()
        .filter(|record| record.company_guid.eq_ignore_ascii_case(company_guid))
    {
        let source_id = || record.record_id.clone();
        let facts = validate_skip_record(record).map_err(|error| TransferFactsError::Facts {
            source_id: source_id(),
            error,
        })?;
        if !record_ids.insert(record.record_id.as_str()) {
            return Err(TransferFactsError::DuplicateSkipRecordId {
                record_id: source_id(),
            });
        }
        if record.endpoint_origin.is_empty() {
            return Err(TransferFactsError::MissingEndpoint {
                source_id: source_id(),
            });
        }
        let built = built_instant(&record.record_id, &record.built_at)?;
        let endpoint = EndpointOrigin::new(record.endpoint_origin.as_str());
        if let Some(window) = facts.window {
            windows.push((endpoint.clone(), facts.bank_ledger.clone(), window));
        }
        for skip in facts.skips {
            let key = SkipKey {
                company: company.clone(),
                bank_ledger: facts.bank_ledger.clone(),
                endpoint_origin: endpoint.clone(),
                row: StatementRowId::new(skip.txn.as_str()),
            };
            SkipMention {
                live: true,
                fact: SkipFact {
                    key,
                    date: skip.date,
                    amount: skip.amount,
                    direction: skip.direction,
                    other_ledger: skip.other_ledger,
                    reference: skip.reference,
                    lineage: LineageId::new(record.record_id.as_str()),
                    reversed_same_day: skip.reversed_same_day,
                },
                row: skip.txn,
                built,
                source_id: source_id(),
            }
            .merge_into(&mut mentions);
        }
    }

    // A skipped row that a batch posted, or may yet post, is not a skip. One a
    // batch found posted holds is retired only when every such batch's lineage
    // recorded the withdrawal, in a build that was sent or found posted and was
    // built after the newest mention of the skip (a withdrawal built at the very
    // instant of the skip cannot be told from one made before it, so it does not
    // count); otherwise the posted voucher contradicts the skip and it stays open.
    let mut withdrawn_at: BTreeMap<(&str, &str), DateTime<FixedOffset>> = BTreeMap::new();
    for batch in &batches {
        if !(batch.sent || batch.found) {
            continue;
        }
        for row in &batch.line.withdrawn {
            let built = built_instant(&batch.line.batch_id, &batch.line.built_at)?;
            let newest = withdrawn_at
                .entry((batch.line.lineage.as_str(), row.as_str()))
                .or_insert(built);
            *newest = (*newest).max(built);
        }
    }
    let mut skips = Vec::new();
    let mut retired_skips = 0;
    let mut held = Vec::new();
    let mut live_skips = BTreeSet::new();
    let mut skip_mentions = BTreeSet::new();
    let mut skip_built = BTreeMap::new();
    for (key, mention) in mentions {
        let live = mention.live;
        match holders.get(mention.row.as_str()) {
            None => skips.push(mention.fact),
            Some(holding) => {
                let unwithdrawn = |at: usize| {
                    withdrawn_at
                        .get(&(batches[at].line.lineage.as_str(), mention.row.as_str()))
                        .is_none_or(|recorded| !withdrawal_counts(recorded, &mention.built))
                };
                let contradicting = holding
                    .batches
                    .iter()
                    .copied()
                    .find(|&at| batches[at].found && unwithdrawn(at));
                let any_posted = holding.batches.iter().any(|&at| batches[at].found);
                // What the skip would be with no rollback covering a holder,
                // every batch ever read posted counting as found (#1050): a
                // rollback never retires a skip that would otherwise be held,
                // and a skip retired then stays out of every admission.
                let retired_without_cover =
                    holding.batches.iter().any(|&at| batches[at].ever_found)
                        && !holding
                            .batches
                            .iter()
                            .any(|&at| batches[at].ever_found && unwithdrawn(at));
                let retired_but_for_cover = retired_without_cover && !any_posted;
                match (contradicting, retired_without_cover && any_posted) {
                    (_, true) => {
                        retired_skips += 1;
                        continue;
                    }
                    (Some(at), _) => held.push(HeldSkip {
                        fact: mention.fact,
                        holder: BatchId::new(batches[at].line.batch_id.as_str()),
                        by: HeldBy::PostedBatch,
                        pairs: true,
                    }),
                    // A batch a recorded rollback covers is named first, so the
                    // row's restored marker and its batch agree (#1050).
                    (None, false) => held.push(HeldSkip {
                        fact: mention.fact,
                        holder: BatchId::new(
                            batches[holding
                                .batches
                                .iter()
                                .rev()
                                .copied()
                                .find(|&at| batches[at].covered)
                                .unwrap_or(holding.latest)]
                            .line
                            .batch_id
                            .as_str(),
                        ),
                        by: HeldBy::UnverifiedBatch,
                        pairs: !retired_but_for_cover,
                    }),
                }
                if retired_but_for_cover {
                    continue;
                }
            }
        }
        skip_built.insert(key.clone(), mention.built);
        skip_mentions.insert(key.clone());
        if live {
            live_skips.insert(key);
        }
    }

    // a Contra built again and never sent is not one while another lineage's
    // batch holding the row was sent or found
    let mut contras_superseded = 0;
    let mut kept = Vec::new();
    for decided in contras {
        let batch = &batches[decided.deciding];
        let superseded = !batch.sent
            && !batch.found
            && is_statement_txn_id(&decided.txn)
            && holders
                .get(decided.txn.as_str())
                .into_iter()
                .flat_map(|held| &held.batches)
                .any(|&at| {
                    batches[at].line.lineage != batch.line.lineage
                        && (batches[at].sent || batches[at].found)
                });
        if superseded {
            contras_superseded += 1;
        } else {
            kept.push(decided.fact);
        }
    }
    kept.sort_by(|left, right| left.key.cmp(&right.key));

    let mut coverage = Coverage::default();
    for (endpoint, bank_ledger, window) in windows {
        // a window over a day some row cannot be vouched for says nothing of that day
        let blocked_inside = [
            (Some(endpoint.clone()), Some(bank_ledger.clone())),
            (Some(endpoint.clone()), None),
            (None, Some(bank_ledger.clone())),
            (None, None),
        ]
        .iter()
        .filter_map(|key| blocked.get(key))
        .any(|days| {
            days.range(window.from.clone()..=window.to.clone())
                .next()
                .is_some()
        });
        if !blocked_inside {
            coverage.add(company.clone(), endpoint, bank_ledger, window)?;
        }
    }
    let restored = batches
        .iter()
        .filter(|batch| batch.covered)
        .map(|batch| batch.line.batch_id.clone())
        .collect();
    Ok(TransferFacts {
        skips,
        contras: kept,
        coverage,
        retired_skips,
        held,
        restored,
        contras_not_two_leg,
        contras_without_facts,
        contras_superseded,
        live_skips,
        skip_mentions,
        skip_built,
    })
}

#[cfg(test)]
#[path = "agent_import_statement_facts_tests.rs"]
mod tests;
