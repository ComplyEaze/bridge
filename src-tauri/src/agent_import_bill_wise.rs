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
use bridge_tally_protocol::{BillWiseFlag, StandardLedgerCatalogV2};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

/// The build argument carrying one approval per party.
pub(super) const APPROVALS_KEY: &str = "on_account_approvals";

/// The code a saved batch built before this record existed is refused with,
/// wherever it would be posted.
pub(super) const BILL_WISE_NOT_RECORDED: &str = "import_batch_predates_bill_wise_record";

/// Each named ledger's `ISBILLWISEON`, by exact name, as the ledger catalogue
/// read said it (#1234: the flag rides the catalogue the build
/// already reads). A V2 catalogue holds a typed flag for every ledger, so a
/// ledger never lacks one here; a name that was not asked for is treated as
/// bill-wise, the refusing direction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ObservedBillWise {
    flags: BTreeMap<String, BillWiseFlag>,
}

impl ObservedBillWise {
    /// Only the requested ledgers are kept. The build has already refused a
    /// name the catalogue does not hold exactly (`masters_for_payload`), and a
    /// name missing here would count as bill-wise anyway, the refusing direction.
    pub(super) fn from_catalogue<'a>(
        catalogue: &StandardLedgerCatalogV2,
        requested: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        let requested = requested.into_iter().collect::<BTreeSet<_>>();
        let flags = catalogue
            .bill_wise_flags()
            .filter(|(name, _, _)| requested.contains(name))
            .map(|(name, _, flag)| (name.to_string(), flag))
            .collect::<BTreeMap<_, _>>();
        Self { flags }
    }

    /// A name that was never observed counts as bill-wise, so refusing is the
    /// safe error for a name the caller did not request or the catalogue lacks.
    fn is_bill_wise(&self, ledger: &str) -> bool {
        !matches!(self.flags.get(ledger), Some(BillWiseFlag::Off))
    }
}

/// Whether every ledger the batch names still reads as it did when the person
/// was asked: a named ledger with no approval was not bill-wise at the build, so
/// it must not be bill-wise now, or an entry on it would land On Account unseen.
/// An approved ledger that is no longer bill-wise needs no approval and passes.
/// A named ledger the catalogue no longer holds fails.
pub(super) fn flags_still_as_approved(
    catalogue: &StandardLedgerCatalogV2,
    named: &BTreeSet<&str>,
    approved: &[OnAccountApproved],
) -> bool {
    let approved = approved
        .iter()
        .map(|item| item.ledger.as_str())
        .collect::<BTreeSet<_>>();
    let now = catalogue
        .bill_wise_flags()
        .filter(|(name, _, _)| named.contains(name))
        .map(|(name, _, flag)| (name, flag))
        .collect::<BTreeMap<_, _>>();
    named.iter().all(|name| match now.get(name) {
        Some(BillWiseFlag::Off) => true,
        Some(BillWiseFlag::On) => approved.contains(name),
        None => false,
    })
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
