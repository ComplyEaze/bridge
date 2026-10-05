//! Which parties of a batch sit on a bill-wise ledger, which read establishes
//! that, and how an approval of one party is bound to the batch's exact content
//! (#1234 slice 1). No Tally I/O belongs in this module.
//!
//! Tally decides On Account by the ledger's `ISBILLWISEON`, not by the voucher
//! type, so the flag is checked for every ledger on every entry of every
//! voucher: a Journal or Contra leg on a bill-wise ledger is as much a leg as
//! a Payment's counterparty. A party is one such ledger with all its rows.
//!
//! The approval digest binds an approval to this batch's exact content, the
//! company's whole identity tuple, the endpoint and ALL the party's rows (a
//! refusal may list only some). It is a consistency binding, not proof that a
//! person said yes: the assistant holds every input and can supply it without
//! asking anyone. The human gate is the native dialog (slice 3).

use super::{
    party_name, EntrySide, ImportCompanyTuple, ImportEntry, ImportPayload, ImportVoucher,
    VoucherType,
};
use bridge_tally_core::ExactDecimal;
use bridge_tally_protocol::native_outstandings::{
    NativeLedgerBillWiseFlag, NativeLedgerSnapshotPeriod,
};
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;
use bridge_tally_protocol::parent_partition::{
    ParentName, ParentObservation, ParentPart, ParentPartition, ParentPartitionError,
    PartitionLimits,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Most parent parts one bill-wise check may read: Bridge's own bound, and NOT
/// a justified one. A build reads the flags twice (before the approval verdict
/// and again after the catalogue and group repeats), a part is at most 16 MB
/// (4,266 ledgers at the 3,750 bytes a ledger is estimated at), and a 22 MB
/// ledger read of a large book took 7 to 11 s once: at that rate four parts,
/// twice, is about 80 s, past the 45 s one call may take (`CALL_CEILING`). The
/// time of this read on a large book was not measured, and the filtered
/// snapshot has been read live only on small books. See the pull request.
pub(super) const MAX_BILL_WISE_PARTS: usize = 4;

/// The build argument carrying one approval per party.
pub(super) const APPROVALS_KEY: &str = "on_account_approvals";

/// The code a saved batch built before this record existed is refused with,
/// wherever it would be posted.
pub(super) const BILL_WISE_NOT_RECORDED: &str = "import_batch_predates_bill_wise_record";

/// Why the flags of the named ledgers were not established. Every variant is
/// carries no data: it names no ledger, parent or company.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BillWiseError {
    /// The company's `books_from` is absent, so no period can be asked for.
    BooksFromAbsent,
    /// `books_from` is not a date the read's period admits (an Education
    /// licence reads only on the 1st, 2nd or 31st).
    PeriodUnsupported,
    /// A named ledger is not in the catalogue.
    LedgerNotInCatalogue,
    /// A named ledger's parent cannot be named in a filter, and the book is
    /// too large to read whole.
    ParentNotNameable,
    /// One parent holds more ledgers than a part may.
    ParentOverBudget,
    /// The parents need more parts than the limit allows.
    TooManyParts,
    /// The catalogue's rows cannot be planned into parts, for the reason the
    /// protocol names.
    Unplannable(&'static str),
    /// A response answered a different number of ledgers than the catalogue
    /// holds for what was asked.
    RowCountDiffers,
    /// A named ledger is in none of the rows.
    LedgerAbsent,
    /// Two rows carry one ledger name.
    LedgerRepeated,
}

impl BillWiseError {
    /// `bill_wise_read_too_large` when a smaller ask could be made;
    /// `bill_wise_not_established` otherwise.
    pub(super) fn reason(self) -> &'static str {
        match self {
            Self::ParentOverBudget | Self::TooManyParts => "bill_wise_read_too_large",
            _ => "bill_wise_not_established",
        }
    }

    /// One sentence a person can be told, for each cause.
    pub(super) fn plain(self) -> &'static str {
        match self {
            Self::BooksFromAbsent => "Tally did not give this company's books-from date, so the check could not be asked for.",
            Self::PeriodUnsupported => "This company's books-from date could not be used to ask for the check: it is not a valid date, or this licence does not let ComplyEaze Bridge read on it (Tally Education reads only on the 1st, 2nd or 31st of a month).",
            Self::LedgerNotInCatalogue => "A ledger in this batch is not in the company's ledger list, so it cannot be checked.",
            Self::ParentNotNameable => "A ledger in this batch sits under a group whose name ComplyEaze Bridge cannot use to read a large book part by part.",
            Self::ParentOverBudget => "A group holding one of these ledgers has more ledgers than one read can check, and this version cannot split it.",
            Self::TooManyParts => "The ledgers in this batch sit under more groups than this version will read in one build.",
            Self::Unplannable(_) => "The company's ledger list could not be divided safely to read it part by part.",
            Self::RowCountDiffers => "Tally returned a different number of ledgers than its ledger list holds, so the answer could not be trusted. A ledger may have been added or removed during the build.",
            Self::LedgerAbsent => "Tally's answer did not include a ledger this batch names under the same spelling. This can happen for a ledger whose name ends in a line break, which this version cannot check yet.",
            Self::LedgerRepeated => "Tally's answer listed one ledger name twice, so its answer could not be trusted.",
        }
    }

    pub(super) fn cause(self) -> &'static str {
        match self {
            Self::BooksFromAbsent => "books_from_absent",
            Self::PeriodUnsupported => "period_unsupported",
            Self::LedgerNotInCatalogue => "ledger_not_in_catalogue",
            Self::ParentNotNameable => "parent_not_nameable",
            Self::ParentOverBudget => "parent_over_budget",
            Self::TooManyParts => "too_many_parts",
            Self::Unplannable(code) => code,
            Self::RowCountDiffers => "row_count_differs",
            Self::LedgerAbsent => "ledger_absent",
            Self::LedgerRepeated => "ledger_repeated",
        }
    }
}

/// The period the flag read asks for: one day, the company's `books_from`.
/// The flag does not depend on the period; the smallest body is the cheapest
/// read. [verified live 6 Oct 2026 on one synthetic book of 17 ledgers: the
/// one-day answer has the same ledgers, parents and flags as the wide window
/// (lab record); the `live_` tests pin the one-day request and its answer]
pub(super) fn bill_wise_period(
    books_from: Option<&str>,
    profile: DateBoundaryProfile,
) -> Result<NativeLedgerSnapshotPeriod, BillWiseError> {
    let books_from = books_from.ok_or(BillWiseError::BooksFromAbsent)?;
    let date = bridge_tally_core::TallyDate::parse(books_from.replace('-', ""))
        .map_err(|_| BillWiseError::PeriodUnsupported)?;
    NativeLedgerSnapshotPeriod::new(profile, date.clone(), date)
        .map_err(|_| BillWiseError::PeriodUnsupported)
}

/// What to read: the whole snapshot when the catalogue fits one part, else the
/// parts holding the named ledgers' parents. Never a complement part: its
/// formula bound is not verified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BillWiseReadPlan {
    Whole { catalogue_rows: usize },
    Parts(Vec<ParentPart>),
}

/// The read plan for `named`. `rows` is the catalogue's `identified_parents()`,
/// `limits` the build's `parent_partition_limits()`.
pub(super) fn plan_reads<'a>(
    rows: impl IntoIterator<Item = (&'a str, &'a str, ParentObservation<'a>)>,
    limits: PartitionLimits,
    named: &BTreeSet<&str>,
) -> Result<BillWiseReadPlan, BillWiseError> {
    let rows = rows.into_iter().collect::<Vec<_>>();
    let parent_of = rows
        .iter()
        .map(|(name, _, parent)| (*name, *parent))
        .collect::<HashMap<_, _>>();
    // Absence from the catalogue is refused whichever way the book is read,
    // so a whole read never hides a ledger the catalogue lacks.
    if named.iter().any(|name| !parent_of.contains_key(name)) {
        return Err(BillWiseError::LedgerNotInCatalogue);
    }
    if u64::try_from(rows.len()).is_ok_and(|count| count <= limits.max_ledgers_per_part) {
        return Ok(BillWiseReadPlan::Whole {
            catalogue_rows: rows.len(),
        });
    }
    let mut parents = BTreeSet::new();
    for name in named {
        match parent_of[name] {
            ParentObservation::Named(text) if ParentName::parse(text).is_ok() => {
                parents.insert(text);
            }
            _ => return Err(BillWiseError::ParentNotNameable),
        }
    }
    // Every ledger under each such parent, so each part's row count is exact.
    let under_parents = rows.iter().copied().filter(
        |(_, _, parent)| matches!(parent, ParentObservation::Named(text) if parents.contains(text)),
    );
    let partition = ParentPartition::plan(under_parents, limits).map_err(|error| match error {
        ParentPartitionError::ParentOverBudget { .. } => BillWiseError::ParentOverBudget,
        ParentPartitionError::TooManyParts { .. } => BillWiseError::TooManyParts,
        other => BillWiseError::Unplannable(other.safe_code()),
    })?;
    if partition.parts().len() > MAX_BILL_WISE_PARTS {
        return Err(BillWiseError::TooManyParts);
    }
    Ok(BillWiseReadPlan::Parts(partition.parts().to_vec()))
}

/// What one response was asked for.
pub(super) enum BillWiseScope<'a> {
    Whole { catalogue_rows: usize },
    Part(&'a ParentPart),
}

/// One snapshot response's rows, and what was asked for.
pub(super) struct BillWiseRead<'a> {
    pub(super) scope: BillWiseScope<'a>,
    pub(super) rows: &'a [NativeLedgerBillWiseFlag],
}

/// Each observed ledger's `ISBILLWISEON`, by exact name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ObservedBillWise {
    flags: BTreeMap<String, bool>,
}

impl ObservedBillWise {
    /// Refuses when a response's row count differs from the catalogue's, when
    /// a name repeats across the rows, or when a requested ledger is in none.
    pub(super) fn new<'a>(
        requested: impl IntoIterator<Item = &'a str>,
        reads: &[BillWiseRead<'_>],
    ) -> Result<Self, BillWiseError> {
        let mut flags = BTreeMap::new();
        for read in reads {
            let count_ok = match &read.scope {
                BillWiseScope::Whole { catalogue_rows } => read.rows.len() == *catalogue_rows,
                BillWiseScope::Part(part) => part.check_row_count(read.rows.len()).is_ok(),
            };
            if !count_ok {
                return Err(BillWiseError::RowCountDiffers);
            }
            for row in read.rows {
                if flags.insert(row.name.clone(), row.bill_wise_on).is_some() {
                    return Err(BillWiseError::LedgerRepeated);
                }
            }
        }
        if requested.into_iter().any(|name| !flags.contains_key(name)) {
            return Err(BillWiseError::LedgerAbsent);
        }
        Ok(Self { flags })
    }

    /// The flags of the named ledgers alone, for comparing two observations:
    /// a ledger the batch does not name changing between two reads does not
    /// matter to it.
    pub(super) fn flags_of<'a>(
        &self,
        named: &BTreeSet<&'a str>,
    ) -> BTreeMap<&'a str, Option<bool>> {
        named
            .iter()
            .map(|name| (*name, self.flags.get(*name).copied()))
            .collect()
    }

    /// A name that was never observed counts as bill-wise: `new` refuses a
    /// requested ledger that is absent, so this only answers for a name the
    /// caller did not request, and refusing is the safe error.
    fn is_bill_wise(&self, ledger: &str) -> bool {
        self.flags.get(ledger).copied().unwrap_or(true)
    }
}

/// Every ledger name the payload's entries carry.
pub(super) fn named_ledgers(payload: &ImportPayload) -> BTreeSet<&str> {
    payload
        .vouchers
        .iter()
        .flat_map(|voucher| voucher.entries.iter())
        .map(|entry| entry.ledger.as_str())
        .collect()
}

/// One voucher's entries on one ledger: a row is (`bridge_txn_id`, ledger), so
/// several entries of one ledger in one voucher fold into one row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PartyRow {
    pub(super) bridge_txn_id: String,
    pub(super) voucher_type: VoucherType,
    pub(super) date: String,
    pub(super) entries: Vec<(EntrySide, String)>,
}

/// A bill-wise ledger and every row of the batch that touches it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BillWiseParty {
    pub(super) ledger: String,
    pub(super) rows: Vec<PartyRow>,
}

/// Every bill-wise ledger the vouchers touch, by ledger name, each with all
/// its rows in batch order, on any side of any voucher type.
pub(super) fn bill_wise_parties(
    vouchers: &[ImportVoucher],
    observed: &ObservedBillWise,
) -> Vec<BillWiseParty> {
    let mut parties = BTreeMap::<&str, Vec<PartyRow>>::new();
    for voucher in vouchers {
        let mut seen = BTreeSet::new();
        for entry in &voucher.entries {
            let ledger = entry.ledger.as_str();
            if !observed.is_bill_wise(ledger) || !seen.insert(ledger) {
                continue;
            }
            parties.entry(ledger).or_default().push(PartyRow {
                bridge_txn_id: voucher.bridge_txn_id.clone(),
                voucher_type: voucher.voucher_type.clone(),
                date: voucher.date.clone(),
                entries: voucher
                    .entries
                    .iter()
                    .filter(|candidate| candidate.ledger == ledger)
                    .map(|candidate| (candidate.side.clone(), candidate.amount.clone()))
                    .collect(),
            });
        }
    }
    parties
        .into_iter()
        .map(|(ledger, rows)| BillWiseParty {
            ledger: ledger.to_string(),
            rows,
        })
        .collect()
}

/// A length-prefixed encoding: every field is its u32 big-endian length, then
/// its bytes. Never serde output, so a serializer change cannot move a digest.
struct Encoder(Sha256);

impl Encoder {
    fn new(domain: &str) -> Self {
        let mut encoder = Self(Sha256::new());
        encoder.field(domain.as_bytes());
        encoder
    }

    fn field(&mut self, bytes: &[u8]) {
        let length = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
        self.0.update(length.to_be_bytes());
        self.0.update(bytes);
    }

    fn count(&mut self, count: usize) {
        self.field(&u32::try_from(count).unwrap_or(u32::MAX).to_be_bytes());
    }

    fn optional(&mut self, value: Option<&str>) {
        match value {
            None => self.field(b"absent"),
            Some(text) => {
                self.field(b"present");
                self.field(text.as_bytes());
            }
        }
    }

    fn finish(self) -> [u8; 32] {
        self.0.finalize().into()
    }
}

fn side_text(side: &EntrySide) -> &'static [u8] {
    match side {
        EntrySide::Dr => b"Dr",
        EntrySide::Cr => b"Cr",
    }
}

/// A digest of every field of every voucher and entry. Both destructurings are
/// exhaustive on purpose: a new field on either stops this compiling until it
/// is bound.
pub(super) fn batch_content_digest(vouchers: &[ImportVoucher]) -> [u8; 32] {
    let mut encoder = Encoder::new("bridge-on-account-batch-v1");
    encoder.count(vouchers.len());
    for voucher in vouchers {
        let ImportVoucher {
            bridge_txn_id,
            date,
            voucher_type,
            narration,
            reference,
            voucher_number,
            entries,
        } = voucher;
        encoder.field(bridge_txn_id.as_bytes());
        encoder.field(date.as_bytes());
        encoder.field(voucher_type.as_str().as_bytes());
        encoder.optional(narration.as_deref());
        encoder.optional(reference.as_deref());
        encoder.optional(voucher_number.as_deref());
        encoder.count(entries.len());
        for entry in entries {
            let ImportEntry {
                ledger,
                amount,
                side,
            } = entry;
            encoder.field(ledger.as_bytes());
            encoder.field(amount.as_bytes());
            encoder.field(side_text(side));
        }
    }
    encoder.finish()
}

/// What every party's digest binds besides its own rows.
pub(super) struct DigestContext<'a> {
    pub(super) company: &'a ImportCompanyTuple,
    pub(super) endpoint_origin: &'a str,
    pub(super) amends_batch_id: Option<&'a str>,
    pub(super) batch_content: [u8; 32],
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::new(), |mut text, byte| {
        let _ = write!(text, "{byte:02x}");
        text
    })
}

/// One digest over ALL of a party's rows in the batch, never over the rows a
/// refusal had room to show.
pub(super) fn party_digest(context: &DigestContext<'_>, party: &BillWiseParty) -> String {
    let mut encoder = Encoder::new("bridge-on-account-party-v1");
    let ImportCompanyTuple {
        name,
        guid,
        company_number,
        books_from,
    } = context.company;
    for field in [name, guid, company_number, books_from] {
        encoder.field(field.as_bytes());
    }
    encoder.field(context.endpoint_origin.as_bytes());
    encoder.field(context.amends_batch_id.unwrap_or_default().as_bytes());
    encoder.field(&context.batch_content);
    encoder.field(party.ledger.as_bytes());
    encoder.count(party.rows.len());
    for row in &party.rows {
        encoder.field(row.bridge_txn_id.as_bytes());
        encoder.field(row.voucher_type.as_str().as_bytes());
        encoder.field(row.date.as_bytes());
        encoder.count(row.entries.len());
        for (side, amount) in &row.entries {
            encoder.field(side_text(side));
            encoder.field(amount.as_bytes());
        }
    }
    hex(&encoder.finish())
}

/// One party the person approved, as recorded on the saved batch.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct OnAccountApproved {
    pub(super) ledger: String,
    pub(super) party_digest: String,
}

/// Why an approval argument was refused. Carries no data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ApprovalError {
    /// Not an array of `{ledger, party_digest}` with a 64-hex digest.
    Malformed,
    /// One ledger approved twice.
    Duplicate,
    /// The ledger is not a bill-wise party of this batch.
    UnknownLedger,
    /// The digest is not the one this batch's party has.
    DigestDiffers,
}

impl ApprovalError {
    pub(super) fn cause(self) -> &'static str {
        match self {
            Self::Malformed => "malformed",
            Self::Duplicate => "duplicate",
            Self::UnknownLedger => "unknown_ledger",
            Self::DigestDiffers => "digest_differs",
        }
    }
}

/// Removes `on_account_approvals` from `args` and parses it. The key must go
/// before the payload is parsed, which refuses unknown fields. A missing key
/// is no approvals.
pub(super) fn take_approvals(args: &mut Value) -> Result<Vec<OnAccountApproved>, ApprovalError> {
    let Some(value) = args
        .as_object_mut()
        .and_then(|map| map.remove(APPROVALS_KEY))
    else {
        return Ok(Vec::new());
    };
    let Value::Array(items) = value else {
        return Err(ApprovalError::Malformed);
    };
    let mut approvals = Vec::<OnAccountApproved>::with_capacity(items.len());
    for item in items {
        // serde's derived `Deserialize` also reads a struct from a JSON array,
        // so the shape is checked first: only an object is an approval.
        if !item.is_object() {
            return Err(ApprovalError::Malformed);
        }
        let approved: OnAccountApproved =
            serde_json::from_value(item).map_err(|_| ApprovalError::Malformed)?;
        let hex_digest = approved.party_digest.len() == 64
            && approved
                .party_digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
        if !hex_digest || approved.ledger.is_empty() {
            return Err(ApprovalError::Malformed);
        }
        if approvals.iter().any(|seen| seen.ledger == approved.ledger) {
            return Err(ApprovalError::Duplicate);
        }
        approvals.push(approved);
    }
    Ok(approvals)
}

/// The parties not approved, and the approvals recorded, once every approval
/// named a real party with the party's own digest.
pub(super) struct ApprovalVerdict<'a> {
    pub(super) approved: Vec<OnAccountApproved>,
    pub(super) unapproved: Vec<(&'a BillWiseParty, String)>,
}

pub(super) fn judge_approvals<'a>(
    approvals: &[OnAccountApproved],
    parties: &'a [BillWiseParty],
    context: &DigestContext<'_>,
) -> Result<ApprovalVerdict<'a>, ApprovalError> {
    let digests = parties
        .iter()
        .map(|party| (party.ledger.as_str(), party_digest(context, party)))
        .collect::<BTreeMap<_, _>>();
    for approval in approvals {
        match digests.get(approval.ledger.as_str()) {
            None => return Err(ApprovalError::UnknownLedger),
            Some(digest) if *digest != approval.party_digest => {
                return Err(ApprovalError::DigestDiffers)
            }
            Some(_) => {}
        }
    }
    let unapproved = parties
        .iter()
        .filter(|party| {
            !approvals
                .iter()
                .any(|approval| approval.ledger == party.ledger)
        })
        .map(|party| (party, digests[party.ledger.as_str()].clone()))
        .collect();
    let mut approved = approvals.to_vec();
    approved.sort_by(|left, right| left.ledger.cmp(&right.ledger));
    Ok(ApprovalVerdict {
        approved,
        unapproved,
    })
}

/// One party's per-side totals, as exact decimals in text. A total that
/// cannot be added is reported as absent rather than guessed.
fn side_totals(party: &BillWiseParty) -> (Option<String>, Option<String>) {
    let mut debit = Some(ExactDecimal::zero());
    let mut credit = Some(ExactDecimal::zero());
    for (side, amount) in party.rows.iter().flat_map(|row| &row.entries) {
        let total = match side {
            EntrySide::Dr => &mut debit,
            EntrySide::Cr => &mut credit,
        };
        *total = total.take().and_then(|sum| {
            ExactDecimal::parse(amount.clone())
                .ok()
                .and_then(|value| sum.checked_add(&value).ok())
        });
    }
    (
        debit.map(|total| total.as_str().to_string()),
        credit.map(|total| total.as_str().to_string()),
    )
}

fn row_json(row: &PartyRow) -> Value {
    json!({
        "bridge_txn_id": row.bridge_txn_id,
        "voucher_type": row.voucher_type.as_str(),
        "date": row.date,
        "entries": row.entries.iter().map(|(side, amount)| json!({
            "side": std::str::from_utf8(side_text(side)).unwrap_or_default(),
            "amount": amount,
        })).collect::<Vec<_>>(),
    })
}

/// The refusal's party list inside `budget` bytes of JSON, and how many whole
/// parties and rows did not fit. The gate counts parties, never this list: a
/// small response cap can empty it while parties still need approval.
pub(super) fn refused_parties_json(
    unapproved: &[(&BillWiseParty, String)],
    budget: usize,
) -> (Vec<Value>, usize) {
    let mut remaining = budget;
    let mut shown = Vec::new();
    let mut omitted = 0;
    for (party, digest) in unapproved {
        let (debit, credit) = side_totals(party);
        let header = json!({
            "ledger": party_name(party.ledger.clone()),
            "party_digest": digest,
            "row_count": party.rows.len(),
            "debit_total": debit,
            "credit_total": credit,
            "rows": [],
            "rows_omitted": party.rows.len(),
        });
        let cost = serde_json::to_string(&header).map_or(usize::MAX, |text| text.len());
        if cost > remaining {
            omitted += 1;
            continue;
        }
        remaining -= cost;
        let rows = party.rows.iter().map(row_json).collect::<Vec<_>>();
        let (kept, rows_omitted) = super::super::bank_statement::bounded(rows, &mut remaining);
        let mut entry = header;
        entry["rows"] = Value::Array(kept);
        entry["rows_omitted"] = json!(rows_omitted);
        shown.push(entry);
    }
    (shown, omitted)
}

#[cfg(test)]
#[path = "agent_import_bill_wise_tests.rs"]
mod tests;
