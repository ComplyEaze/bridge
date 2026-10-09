use super::{
    arg_usize, combine_evidence, company_currency_read, company_high_water_read, company_json,
    import_ledger_catalogue_read, native_group_snapshot_read, normalized_date, optional_string,
    parse_company_high_water, party_name, required_string, sha256_hex, sha256_json,
    standard_ledger_catalog_read, Evidence, Server, ToolFailure, ToolOutcome,
    VOUCHER_CHECKPOINT_NOT_OBSERVED,
};
use crate::tally::agent_read_request::AgentReadRequest;
use crate::tally::standard_ledger_catalog::{
    admit_standard_ledger_catalog_request, parse_import_ledger_catalog_response,
    parse_standard_ledger_catalog_response,
};
use bridge_tally_core::master_binding::{
    self, twin_fold_keys, BindingBasis, BindingStatus, Candidates, EntityBinding, MasterCatalog,
    MasterClass, SourceEntity,
};
use bridge_tally_core::ExactDecimal;
use bridge_tally_protocol::native_outstandings::parse_native_group_snapshot;
use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;
use bridge_tally_protocol::xml_text::escape_text as xml_escape;
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Seek, SeekFrom, Write};

#[path = "agent_import_bill_wise.rs"]
mod bill_wise;
#[path = "agent_import_cash_bank.rs"]
mod cash_bank;
use cash_bank::{CashBankState, LegRequirement, ObservedMasters};
#[path = "agent_import_identity.rs"]
mod identity;
#[path = "agent_import_invoice.rs"]
pub(super) mod invoice;
pub(super) use identity::import_identity;
use identity::ImportIdentityScheme;
#[path = "agent_import_schema.rs"]
mod schema;
pub(super) use schema::voucher_input_schema;
#[path = "agent_desktop_journal.rs"]
mod desktop_journal;
#[path = "agent_desktop_journal_review.rs"]
pub(crate) mod desktop_journal_review;
#[cfg(test)]
#[path = "agent_desktop_journal_tests.rs"]
mod desktop_journal_tests;
use crate::endpoint_coordination as dispatch_lease;
use crate::local_files::file::lock_error as import_admission_lock_error;
#[path = "agent_import_ack.rs"]
mod ack;
#[path = "agent_import_amend.rs"]
mod amend;
#[path = "agent_import_approval.rs"]
mod approval;
#[path = "agent_import_ledger.rs"]
pub(super) mod ledger;
#[path = "agent_import_local_data.rs"]
pub(super) mod local_data;
#[path = "agent_import_persistence.rs"]
mod persistence;
#[path = "agent_import_post.rs"]
mod post;
#[path = "agent_import_span_identity.rs"]
mod span_identity;
#[path = "agent_import_stop.rs"]
mod stop;
pub(super) use approval::PostApprovals;
#[path = "agent_import_verification.rs"]
mod verification;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use verification::{
    actual_entry_fingerprint, alter_id_delta, canonical_verification_amount,
    company_high_water_mark, corroborate_verification_window, expected_entry_fingerprint,
    final_verification_status, mark_verification_names, parse_import_voucher_rows,
    parse_import_vouchers, render_proof_markdown, verification_response_page, verification_status,
    verification_window_identities, verify_batch_as, voucher_diffs,
    voucher_is_accounting_effective, Attribution, DeltaBasis, InvoiceIdentity, VerificationStatus,
};
#[cfg(test)]
use verification::{
    batch_duplicate_sets, duplicates, expected_fingerprint, observed_fingerprint,
    observed_voucher_identity, verify_batch, VERIFICATION_NAME_FIELDS,
};

struct ImportProfileObservation {
    qualification: Result<(), ImportProfileRefusal>,
    evidence: Evidence,
    observed_profile: Value,
    admission_key: (String, String),
}

#[derive(Clone, Copy)]
enum ImportProfileRefusal {
    Mode,
}

impl ImportProfileRefusal {
    fn build_code(self) -> &'static str {
        match self {
            Self::Mode => "import_mode_unqualified",
        }
    }

    fn verification_code(self) -> &'static str {
        match self {
            Self::Mode => "verification_mode_unqualified",
        }
    }
}

const MAX_VOUCHERS: usize = 1_000;
pub(super) const MAX_MASTER_NAMES: usize = 100;
pub(super) const MAX_MASTER_NAME_CHARS: usize = 1024;
const MAX_TEXT_CHARS: usize = 2_000;

/// Name the one parse failure a caller can act on, and keep every other cause on
/// the general refusal.
///
/// `parse_company_high_water` already reports which of its causes fired. That
/// detail used to be discarded, so an empty book — a company that has never held
/// a voucher, for which Tally omits ALTVCHID entirely — was indistinguishable
/// from a malformed response, an unmatched company GUID or an ambiguous company
/// row. Only the empty book has a next step the caller can take, so only it is
/// promoted.
///
/// This changes what a refusal is *called*, never whether it refuses. Without a
/// voucher high-water mark there is no "before", so an import cannot be
/// attributed and Bridge must still decline either way.
///
/// Kept pure and separate from `pre_import_mark` so the mapping is provable
/// without a live gateway or a scripted response sequence.
fn pre_import_mark_refusal(parse_error: &str) -> &'static str {
    if parse_error == VOUCHER_CHECKPOINT_NOT_OBSERVED {
        "empty_book_first_import"
    } else {
        "pre_import_mark_unobserved"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ImportPayload {
    company_guid: String,
    vouchers: Vec<ImportVoucher>,
    /// A batch this Bridge built whose vouchers this build corrects in place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    amends_batch_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ImportVoucher {
    bridge_txn_id: String,
    date: String,
    voucher_type: VoucherType,
    #[serde(default)]
    narration: Option<String>,
    #[serde(default)]
    reference: Option<String>,
    #[serde(default)]
    voucher_number: Option<String>,
    /// What a Sales invoice carries beyond its entries (place of supply, and
    /// what the build observed in Tally). Absent on every other type, and on
    /// every record written before invoices existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    invoice: Option<invoice::InvoiceDetail>,
    entries: Vec<ImportEntry>,
}

impl ImportVoucher {
    /// The display name of the voucher type this voucher is filed under in the
    /// book: the caller-named type for an invoice, the class name otherwise.
    /// What a read-back must find, and what a render writes.
    fn filed_type_name(&self) -> &str {
        match &self.invoice {
            Some(detail) if self.voucher_type.is_invoice() => &detail.voucher_type_name,
            _ => self.voucher_type.as_str(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
enum VoucherType {
    Payment,
    Receipt,
    Journal,
    Contra,
    Sales,
}

impl VoucherType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Payment => "Payment",
            Self::Receipt => "Receipt",
            Self::Journal => "Journal",
            Self::Contra => "Contra",
            Self::Sales => "Sales",
        }
    }

    /// A GST invoice: a party leg, a sales or purchase leg and tax legs, with
    /// an invoice view. It shares nothing of the bank shape and is refused
    /// beside any other shape.
    fn is_invoice(&self) -> bool {
        matches!(self, Self::Sales)
    }
}

/// Which sides of a voucher type are constrained, and how.
///
/// This is the whole of what distinguishes Payment, Receipt and Contra from a
/// Journal on the write path: a Journal names no party and constrains no side.
struct BankVoucherShape {
    legs: &'static [(EntrySide, LegRequirement)],
}

impl BankVoucherShape {
    /// The side rendered as `PARTYLEDGERNAME`, which is the counterparty leg by
    /// definition. A Contra moves money between two of the company's own
    /// accounts, so it has no counterparty leg and names no party.
    fn party_side(&self) -> Option<&EntrySide> {
        self.legs
            .iter()
            .find(|(_, requirement)| *requirement == LegRequirement::Counterparty)
            .map(|(side, _)| side)
    }
}

impl VoucherType {
    /// `None` for a Journal, whose qualified file shape predates and does not
    /// carry these elements.
    fn bank_shape(&self) -> Option<BankVoucherShape> {
        match self {
            // Payment: Dr party, Cr bank. Receipt: Dr bank, Cr party.
            Self::Payment => Some(BankVoucherShape {
                legs: &[
                    (EntrySide::Cr, LegRequirement::Money),
                    (EntrySide::Dr, LegRequirement::Counterparty),
                ],
            }),
            Self::Receipt => Some(BankVoucherShape {
                legs: &[
                    (EntrySide::Dr, LegRequirement::Money),
                    (EntrySide::Cr, LegRequirement::Counterparty),
                ],
            }),
            Self::Contra => Some(BankVoucherShape {
                legs: &[
                    (EntrySide::Dr, LegRequirement::Money),
                    (EntrySide::Cr, LegRequirement::Money),
                ],
            }),
            Self::Journal | Self::Sales => None,
        }
    }
}

// Every variant here carries live import/readback evidence for the exact file
// shape this module renders for it: Journal in
// docs/agent/ASSESSMENT-2026-09-06.md, and Payment/Receipt/Contra in
// docs/tally/TALLY_PROTOCOL_REFERENCE.md §9.13. Adding a `VoucherType` variant
// does not qualify it; the build refuses any type absent from this list before
// its first request, and a post refuses a saved one, so evidence has to arrive
// before the file can.
//
// Sales is NOT here. One lab rehearsal (7 Oct 2026, §9.16) showed the
// duplicate-number read finding a known invoice, Tally taking two of the
// element sets, every read-back field coming back, and the company's
// STATENAME equal to the GST registration state of a keyed invoice. It joins
// this list when what that rehearsal left owed is done (ADR 0004). The check
// before a post now recognises an invoice by its number; that has not run
// against Tally.
const LIVE_QUALIFIED_VOUCHER_TYPES: &[VoucherType] = &[
    VoucherType::Journal,
    VoucherType::Payment,
    VoucherType::Receipt,
    VoucherType::Contra,
];

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
enum EntrySide {
    Dr,
    Cr,
}

impl EntrySide {
    fn tally_positive(&self) -> &'static str {
        match self {
            Self::Dr => "Yes",
            Self::Cr => "No",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ImportEntry {
    ledger: String,
    amount: String,
    side: EntrySide,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) struct ImportLedgerLine {
    batch_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    identity_scheme: Option<ImportIdentityScheme>,
    /// The original batch whose wire identity this build reuses. Absent on
    /// every build that is not an amendment, including all older records.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    amends_batch_id: Option<String>,
    company_guid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    endpoint_origin: Option<String>,
    #[serde(default)]
    company: Option<ImportCompanyTuple>,
    txn_ids: Vec<String>,
    date_from: String,
    date_to: String,
    sha256: String,
    built_at: String,
    status: String,
    pre_import_mark: PreImportMark,
    vouchers: Vec<ImportVoucher>,
    /// Each ledger the batch names, with the GUID the build's own catalogue
    /// read bound it to (bridge#239). A post refuses when any of them now
    /// resolves to another GUID. Absent on records built before this field
    /// existed: such a batch is refused for posting and must be rebuilt.
    ///
    /// Written as `ledger_identities_2`. Releases 0.3.0 to 0.4.2 read only
    /// `ledger_identities`, find none, and refuse the batch as built before
    /// this record (`import_batch_predates_ledger_binding`), which is their
    /// one refusal of a record they cannot check. They do not read the two
    /// records below, so without this they would post a batch this build
    /// saved, with neither check. The old name is still read, for a batch one
    /// of them saved. No writer may write both names: a record carrying both
    /// does not parse, and the journal reader refuses the whole history on a
    /// record it cannot parse. When a record is added to or removed from a saved
    /// batch, write this one under a new name again and keep every earlier
    /// name as an alias: `a_saved_batch_holds_exactly_these_records` stops
    /// compiling, or fails, until that is looked at.
    #[serde(
        default,
        rename = "ledger_identities_2",
        alias = "ledger_identities",
        skip_serializing_if = "Option::is_none"
    )]
    ledger_identities: Option<Vec<BoundLedger>>,
    /// Each ledger a bank cash answer named as cash in hand, with the voucher
    /// it was answered for (#815). The build refused any outside Cash-in-Hand,
    /// and a post checks each again before approval and in the queue, since a
    /// ledger or its group can move in between. Empty when no answer named
    /// one. Absent on records built before this field existed: such a batch is
    /// refused for posting and must be rebuilt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cash_in_hand_ledgers: Option<Vec<CashInHandLedger>>,
    /// Each bill-wise ledger (`ISBILLWISEON` Yes) for which the caller supplied
    /// an approval, with the digest that tied it to this batch's exact content
    /// (#1234). Bridge cannot tell whether a person said yes: the digest is a
    /// consistency binding. `Some(vec![])` when the build found
    /// no bill-wise ledger among those it names. Absent on records built
    /// before this field existed: such a batch is refused for posting and must
    /// be rebuilt. A release older than this field does not read it (this
    /// struct does not deny unknown fields); releases 0.3.0 to 0.4.2 refuse
    /// the batch for the ledger binding they cannot find instead (see
    /// `ledger_identities`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    on_account_approved: Option<Vec<bill_wise::OnAccountApproved>>,
}

/// A ledger a bank cash answer named as cash in hand, and the voucher built
/// from that answer (#815).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct CashInHandLedger {
    bridge_txn_id: String,
    ledger: String,
}

/// A requested name and every live ledger whose stored name folds equal to it
/// under either of the binding module's folds (`twin_fold_keys`: case, NFC,
/// dashes, quotes, whitespace including CR and LF, and `/` as a space), when
/// there are at least two (bridge#626). Tally's import lookup also matches
/// names loosely (§9.4d), and which of two such ledgers it would post to is
/// not established, so a build naming either is refused.
#[derive(Clone, Debug, PartialEq, Eq)]
struct FoldedTwins {
    requested: String,
    live: Vec<(String, Option<String>)>,
}

/// The requested names that fold equal to two or more live ledgers, in the
/// order requested. Each lists its live ledgers in catalogue order.
fn folded_twins<'a>(
    requested: &[String],
    catalogue: impl Iterator<Item = (&'a str, Option<&'a str>)>,
) -> Vec<FoldedTwins> {
    let catalogue = catalogue.collect::<Vec<_>>();
    let mut by_key: [BTreeMap<String, Vec<usize>>; 3] = Default::default();
    for (index, (name, _)) in catalogue.iter().enumerate() {
        for (keys, key) in by_key.iter_mut().zip(twin_fold_keys(name)) {
            keys.entry(key).or_default().push(index);
        }
    }
    requested
        .iter()
        .filter_map(|name| {
            let mut family = BTreeSet::new();
            for (keys, key) in by_key.iter().zip(twin_fold_keys(name)) {
                if let Some(found) = keys.get(&key) {
                    family.extend(found.iter().copied());
                }
            }
            (family.len() > 1).then(|| FoldedTwins {
                requested: name.clone(),
                live: family
                    .into_iter()
                    .map(|index| {
                        let (name, parent) = catalogue[index];
                        (name.to_string(), parent.map(str::to_string))
                    })
                    .collect(),
            })
        })
        .collect()
}

fn folded_live_ledgers_json(twins: &FoldedTwins) -> Value {
    json!(twins
        .live
        .iter()
        .map(|(name, parent)| json!({"name": party_name(name.clone()), "parent": parent}))
        .collect::<Vec<_>>())
}

/// Marks each report entry whose requested name folds equal to two or more
/// live ledgers with those ledgers and their groups. Such a name is not
/// importable, whichever ledger it bound to.
fn annotate_folded_twins<'a>(
    report: &mut [Value],
    requested: &[String],
    catalogue: impl Iterator<Item = (&'a str, Option<&'a str>)>,
) {
    let twins = folded_twins(requested, catalogue);
    for (entry, name) in report.iter_mut().zip(requested) {
        if let Some(found) = twins.iter().find(|twins| &twins.requested == name) {
            entry["folded_twins"] = folded_live_ledgers_json(found);
            if entry.get("importable").is_some() {
                entry["importable"] = json!(false);
            }
        }
    }
}

const FOLDED_TWIN_NEXT_STEP: &str = "No file was written. Each ledger in ledger_twins has at least one other live ledger whose name differs from it only by case, spacing, dashes, slashes or quotes, or a trailing line break. Tally's import also matches names loosely, and which of them it would post to is not established, so ComplyEaze Bridge names none of them. Have an operator rename ledgers in Tally until no two in each group fold equal, then run validate_masters and build again.";

/// One ledger name and the GUID it was bound to at build time.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct BoundLedger {
    name: String,
    guid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct ImportCompanyTuple {
    name: String,
    guid: String,
    company_number: String,
    books_from: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct PreImportMark {
    kind: String,
    value: Option<u64>,
    #[serde(default)]
    master_value: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct ReadEntry {
    ledger: String,
    amount: String,
    is_deemed_positive: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct ReadVoucher {
    remote_id: Option<String>,
    guid: Option<String>,
    alter_id: Option<u64>,
    date: Option<String>,
    voucher_type: Option<String>,
    narration: Option<String>,
    voucher_number: Option<String>,
    master_id: Option<String>,
    cancelled: Option<bool>,
    optional: Option<bool>,
    /// Absent when the response carried no `EFFECTIVEDATE` (or an empty one).
    #[serde(default)]
    effective_date: Option<String>,
    #[serde(rename = "amounts")]
    entries: Vec<ReadEntry>,
}

impl super::WindowRow for ReadVoucher {
    fn window_date(&self) -> Option<&str> {
        self.date.as_deref()
    }
    fn window_alter_id(&self) -> Option<u64> {
        self.alter_id
    }
    fn window_guid(&self) -> Option<&str> {
        self.guid.as_deref()
    }
    fn window_master_id(&self) -> Result<Option<u64>, String> {
        super::master_id_of(self.master_id.as_deref())
    }
}

/// The report of how a native post was attributed, with one plain line a
/// person reads first. The codes stay beside it for the assistant.
fn with_post_span_summary(mut report: Value, vouchers: &Value) -> Value {
    let summary = match report["state"].as_str() {
        Some("bound") => "Each voucher was matched to the Tally voucher this post created.",
        Some("refused") => "ComplyEaze Bridge could not confirm which Tally vouchers this post created, so the batch stays open: check its vouchers in Tally before posting any of them again.",
        Some("unsettled")
            if report["code"]
                == span_identity::BindUnsettled::EffectiveDateNotObserved.code() =>
        {
            "ComplyEaze Bridge could not confirm this post yet: Tally's read of its vouchers left out the effective date that ComplyEaze Bridge checks, and a later check may settle it. Until then, check the vouchers in Tally before posting any of them again."
        }
        Some("unsettled") => "ComplyEaze Bridge could not finish matching this post to Tally just now; run the check again.",
        Some("not_bound") => "ComplyEaze Bridge has no readable answer from Tally to this post, so it cannot confirm it: check the vouchers in Tally and do not post them again.",
        Some("book_rolled_back") => "The company's books are older than this post (probably restored from a backup or replaced by another copy), so its vouchers are no longer there: check in Tally before posting again.",
        _ => return report,
    };
    // When Tally's own answer reported every voucher of this post as not
    // created (#1116, #1126), the first line names that cause and agrees with
    // the vouchers' next step, rather than leaving the cause open (#1108). A
    // batch that is partly created never reads so, and keeps the line above.
    let reported_not_created = vouchers.as_array().is_some_and(|vouchers| {
        !vouchers.is_empty()
            && vouchers
                .iter()
                .all(|voucher| voucher["status"] == "tally_reported_not_created")
    });
    let summary = if reported_not_created {
        "Tally reported that this post created none of its vouchers: follow each voucher's next step."
    } else {
        summary
    };
    report["summary"] = json!(summary);
    report
}

/// How one verification attributes a native post by its own span, and what it
/// reports about that (`post_span_binding` in the proof).
struct PostSpanDecision {
    bindings: Option<Vec<span_identity::PostedVoucherIdentity>>,
    /// The journal generation this decision's own verdict record left.
    recorded_at: Option<ledger::VerificationGeneration>,
    /// The book reads below a mark this post left: it was rolled back, so a
    /// voucher not found in it is reported as rolled back, never as absent.
    rolled_back: bool,
    report: Value,
}

impl PostSpanDecision {
    fn not_applicable() -> Self {
        Self::reported(json!({"state":"not_applicable"}))
    }

    fn reported(report: Value) -> Self {
        Self {
            bindings: None,
            recorded_at: None,
            rolled_back: false,
            report,
        }
    }
}

/// A complete collection admitted before either corroboration or attribution.
#[derive(Clone, Debug, PartialEq, Eq)]
struct ImportReadSource {
    rows: Vec<ReadVoucher>,
}

impl ImportReadSource {
    fn admit(mut rows: Vec<ReadVoucher>) -> Result<Self, String> {
        let mut identities = super::VoucherSourceIdentities::default();
        let mut transaction_tags = BTreeSet::new();
        for row in &mut rows {
            if let Some(guid) = row.guid.as_mut() {
                *guid = guid.trim().to_ascii_lowercase();
            }
            let master_id = super::parse_optional_tally_u64(
                row.master_id.as_deref(),
                "import_verification_master_id_invalid",
            )?;
            identities
                .admit(row.guid.as_deref(), master_id)
                .map_err(|_| "import_verification_identity_invalid".to_string())?;
            row.master_id = master_id.map(|id| id.to_string());
            if row.guid.is_none() && row.master_id.is_none() {
                return Err("import_verification_identity_invalid".into());
            }
            let narration = row.narration.as_deref().unwrap_or_default();
            let mut markers = narration_markers(narration);
            if let Some(first) = markers.next() {
                // More than one means the row claims two imports. Taking the
                // first would resolve that silently, which is what the
                // presence contract refuses on the same evidence.
                if markers.next().is_some() {
                    return Err("import_verification_tag_ambiguous".into());
                }
                let tag = first
                    .filter(|id| valid_txn_id(id))
                    .ok_or_else(|| "import_verification_tag_invalid".to_string())?;
                if !transaction_tags.insert(tag.to_string()) {
                    return Err("import_verification_tag_ambiguous".into());
                }
            }
        }
        Ok(Self { rows })
    }
}

impl Server {
    pub(super) fn voucher_schema(&self) -> Result<ToolOutcome, String> {
        let schema = voucher_input_schema();
        Ok(ToolOutcome {
            payload: json!({"result": {"schema": schema, "rules": [
                "bridge_txn_id is client-supplied, unique within this batch, 1-64 ASCII characters from [A-Za-z0-9_-]",
                "new files accept Journal, Payment, Receipt and Contra, the voucher types with recorded live import/readback evidence",
                "a Journal takes any balanced set of entries and may carry a voucher_number",
                "Payment, Receipt and Contra take two or more entries with at least one debit and one credit, no ledger on both sides (for more than two entries, one three-entry Receipt built by ComplyEaze Bridge has been imported over the gateway and verified; no multi-entry Payment or Contra has been, and none of the three, including that Receipt, through Tally's Import menu), and neither voucher_number nor reference: neither element's fate on these types has been observed, and the bank's own reference belongs in the narration, which survives",
                "a Payment credits, and a Receipt debits, a ledger whose live group ancestry reaches Bank Accounts or Cash-in-Hand; both Contra legs must name one, and a leg that cannot be established is refused",
                "the other leg of a Payment or Receipt must be established as holding no money: a ledger under any money group is refused there, because money on both sides is a Contra whatever the type says, and so is one whose group ancestry cannot be resolved at all",
                "each voucher has at least two entries and exact debit total equals credit total",
                "amounts are positive decimal strings with exactly two fractional digits",
                "dates must be within the selected company's BOOKSFROM through today",
                "ledger names must exactly match the live catalogue; validate_masters before build_import_xml",
                "a batch may contain at most 100 distinct ledger names of at most 1024 characters each"
            ], "limits": {"import_mode_qualification": "New files require freshly observed supported TallyPrime product and licence mode before and after the build reads. Release and licence tier are reported as observed facts. Journal, Payment, Receipt and Contra are the voucher types with recorded import/readback evidence, each only in the exact file shape this schema admits, except that a Payment, Receipt or Contra with more than two entries (bridge#466) rests on narrower evidence: hand-built files of that shape were imported and read back over the gateway (a Contra only with a repeated ledger) and one three-entry Receipt built by ComplyEaze Bridge was imported over the gateway and verified, but no multi-entry Payment or Contra has been, and none of the three, including that Receipt, through Tally's Import menu, and its build reports live_evidence hand_built_gateway_readback; every other voucher type is refused. A single-voucher batch is eligible for post_import (and, when BRIDGE_AGENT_ENABLE_BATCH_POST is on, a batch of 2 to 50 such vouchers): an unnumbered Journal, or a Payment, Receipt or Contra, whose legs post_import classifies again before approval and after approval inside the endpoint queue, before the final duplicate check and the post."}}}),
            evidence: local_evidence("voucher_schema"),
            company_guid: None,
            truncated: false,
        })
    }

    pub(super) async fn validate_masters(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let ledgers = args
            .get("ledgers")
            .and_then(Value::as_array)
            .ok_or_else(|| "ledgers_required".to_string())?
            .iter()
            .map(Value::as_str)
            .collect::<Option<Vec<_>>>()
            .filter(|names| !names.is_empty())
            .ok_or_else(|| "ledgers_required".to_string())?;
        // Before either read. A name the core refuses is refused at any
        // catalogue, so verifying the company and reading its ledgers first
        // would spend two live round trips — and retain evidence of them — to
        // reach a failure that was decidable from the request alone.
        let names = ledgers.into_iter().map(str::to_string).collect::<Vec<_>>();
        let requested = requested_masters(&names).map_err(ToolFailure::from)?;
        let (company, identity, identity_evidence) = self.verified_company(guid).await?;
        let (catalogue, ledger_masters, _, evidence) = self
            .read_import_ledger_catalogue(&identity, &company.name)
            .await
            .map_err(|failure| failure.with_prior_evidence(identity_evidence.clone()))?;
        let mut report = requested_master_report(&requested, &catalogue)
            // The catalogue read already succeeded, so its request/response
            // commitments belong in the failure too; attaching identity evidence
            // alone would omit a Tally read that actually happened.
            .map_err(|code| {
                ToolFailure::from(code).with_prior_evidence(combine_evidence(
                    identity_evidence.clone(),
                    evidence.clone(),
                ))
            })?;
        annotate_folded_twins(&mut report, &names, ledger_masters.catalog().parents());
        let hash = sha256_json(&catalogue);
        Ok(ToolOutcome {
            payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {"masters": report, "catalogue_evidence_sha256": hash}}),
            evidence: combine_evidence(identity_evidence, evidence),
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }

    async fn qualified_import_profile(&self) -> Result<ImportProfileObservation, ToolFailure> {
        let observation = self.observe_import_profile().await?;
        if let Err(refusal) = observation.qualification {
            return Err(ToolFailure::from(refusal.build_code().to_string())
                .with_prior_evidence(observation.evidence));
        }
        Ok(observation)
    }

    async fn observe_import_profile(&self) -> Result<ImportProfileObservation, ToolFailure> {
        use bridge_tally_core::{CapabilityFeatureId, CapabilityState, EvidenceConfidence};
        let (probe, wire) = self
            .runtime
            .probe_with_wire_evidence(self.tally_config())
            .await
            .map_err(|error| ToolFailure::from_runtime("import_mode_probe_failed", error))?;
        let evidence = super::evidence_from_runtime_read(wire);
        let product = probe.profile.product.to_ascii_lowercase();
        let mode = probe
            .profile
            .mode
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let qualified = product == "tallyprime"
            && matches!(mode.as_str(), "licensed" | "education" | "educational")
            && probe
                .profile
                .features
                .get(&CapabilityFeatureId::ProductAndMode)
                .is_some_and(|feature| {
                    feature.state == CapabilityState::Supported
                        && feature.confidence == EvidenceConfidence::Observed
                });
        // The literal-date verification filter has its own returned-row and
        // corroboration checks. Product/mode must be observed; a release or
        // tier label is evidence, not a categorical import-file admission gate.
        let qualification = qualified.then_some(()).ok_or(ImportProfileRefusal::Mode);
        Ok(ImportProfileObservation {
            qualification,
            evidence,
            observed_profile: json!({
                "product": probe.profile.product,
                "release": probe.profile.release,
                "license_tier": probe.profile.license_tier,
                "mode": probe.profile.mode,
            }),
            admission_key: (product, mode),
        })
    }

    pub(super) async fn build_import_xml(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        // A proposals file supplies `vouchers`; everything after this line
        // admits them exactly as it admits inline vouchers.
        let resolved =
            super::bank_statement::resolve_import_arguments(&self.settings.data_dir, args)?;
        // The approvals are not part of the payload (`parse_payload` refuses
        // unknown fields, and a post builds `ImportPayload` literals), so the
        // key is taken out here and judged once the flags have been read.
        let mut args = resolved.args.clone();
        let approvals = bill_wise::take_approvals(&mut args).map_err(approval_invalid)?;
        let args = &args;
        let mut payload = parse_payload(args)?;
        // First of all: a type that is not qualified is refused as that, not
        // for whichever of its fields a later check would stop at.
        refuse_unqualified_types(&payload.vouchers, LIVE_QUALIFIED_VOUCHER_TYPES)?;
        invoice::refuse_supplied_observed(&payload.vouchers)?;
        validate_payload(&payload)?;
        // After the whole of `validate_payload`: in a batch with several
        // defects, the first one it finds is reported, not this one (#1055).
        refuse_rewritten_narration(&payload.vouchers)?;
        let (debit, credit) = totals(&payload.vouchers)?;
        normalize_payload_dates(&mut payload)?;
        // Refuse an amendment Bridge could never admit before reading Tally.
        // Admission is repeated under the exclusive lock before publication.
        if payload.amends_batch_id.is_some() {
            invoice::refuse_invoice_amendment(&payload.vouchers)?;
            let _admission_lock = self.lock_import_admission_shared()?;
            self.amendment_lineage_while_admitted(&payload)?;
        }
        // An invoice reaches Tally only through post_import: a file imported by
        // hand skips the stop, the duplicate checks, the reads on either side of
        // the approval and the readback. With posting off, no invoice file is
        // written. Judged after the refusals that need no read, and before the
        // first read.
        if payload
            .vouchers
            .iter()
            .any(|voucher| voucher.voucher_type.is_invoice())
            && !self.settings.writes_enabled
        {
            return Err(INVOICE_POST_NOT_ENABLED.to_string().into());
        }
        let opening_profile = self.qualified_import_profile().await?;
        validate_import_dates_for_profile(&payload, &opening_profile).map_err(|code| {
            ToolFailure::from(code).with_prior_evidence(opening_profile.evidence.clone())
        })?;
        let mode_evidence = opening_profile.evidence.clone();
        let (company, identity, identity_evidence) = self
            .verified_company(&payload.company_guid)
            .await
            .map_err(|failure| failure.with_prior_evidence(mode_evidence.clone()))?;
        let mut accumulated = combine_evidence(mode_evidence, identity_evidence.clone());
        let result: Result<ToolOutcome, ToolFailure> = async {
            let (catalogue, ledger_masters, catalogue_evidence) = self
                .read_import_ledger_catalogue(&identity, &company.name)
                .await
                .map(|(names, catalogue, _, evidence)| (names, catalogue, evidence))?;
            accumulated = combine_evidence(accumulated.clone(), catalogue_evidence.clone());
            let requested_names = requested_ledger_names(&payload);
            let mut report = masters_for_payload(&payload, &catalogue)?;
            annotate_folded_twins(&mut report, &requested_names, ledger_masters.catalog().parents());
            if report.iter().any(|value| value["match_state"] != "exact") {
                let report = in_batch_order(
                    requested_names.iter().zip(report).collect(),
                    &payload.vouchers,
                    |(name, _)| name.as_str(),
                )
                .into_iter()
                .map(|(_, master)| master)
                .collect::<Vec<_>>();
                return Ok(ToolOutcome {
                    payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                        "state":"refused", "reason":"masters_not_exact", "masters":report,
                        "catalogue_evidence_sha256":sha256_json(&catalogue),
                        "next_step":master_recovery_guidance(&report)
                    }}),
                    evidence: accumulated.clone(),
                    company_guid: Some(payload.company_guid),
                    truncated: false,
                });
            }
            let twins = in_batch_order(
                folded_twins(&requested_names, ledger_masters.catalog().parents()),
                &payload.vouchers,
                |twins| twins.requested.as_str(),
            );
            if !twins.is_empty() {
                return Ok(ToolOutcome {
                    payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                        "state":"refused", "reason":"ledger_has_folded_twin",
                        "ledger_twins": twins.iter().map(|twins| json!({
                            "requested": party_name(twins.requested.clone()),
                            "relation": "fold_equal",
                            "live_ledgers": folded_live_ledgers_json(twins),
                        })).collect::<Vec<_>>(),
                        "catalogue_evidence_sha256":sha256_json(&catalogue),
                        "next_step":FOLDED_TWIN_NEXT_STEP
                    }}),
                    evidence: accumulated.clone(),
                    company_guid: Some(payload.company_guid),
                    truncated: false,
                });
            }
            // Bind each named ledger to the GUID this read observed (#239): a
            // post refuses a ledger renamed and replaced under its name since.
            let build_binding = ledger_masters
                .catalog()
                .bind_selected(requested_ledger_names(&payload))
                .map_err(|_| "import_masters_changed".to_string())?
                .pairs()
                .map(|(name, guid)| BoundLedger {
                    name: name.to_string(),
                    guid: guid.to_string(),
                })
                .collect::<Vec<_>>();
            // A Sales invoice reads its own masters (ledger compliance listing,
            // the named voucher type, the party's bill-wise flag, the company's
            // state), classifies every leg from them and records what it saw on
            // the voucher. A journal-only or bank batch never takes this path.
            if payload.vouchers.iter().any(|voucher| voucher.voucher_type.is_invoice()) {
                match self
                    .admit_sales_invoice(&identity, &company, &mut payload.vouchers[0], &ledger_masters)
                    .await
                {
                    Ok(evidence) => {
                        accumulated = combine_evidence(accumulated.clone(), evidence);
                    }
                    Err(invoice::InvoiceAdmission::Failed(failure)) => {
                        return Err(failure.with_prior_evidence(accumulated.clone()));
                    }
                    Err(invoice::InvoiceAdmission::Refused(refusals)) => {
                        return Ok(ToolOutcome {
                            payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                                "state":"refused", "reason":"invoice_not_admitted",
                                "refusals": refusals.iter().map(invoice::InvoiceRefusal::to_json).collect::<Vec<_>>(),
                                "next_step":"No file was written. Each refusal names one defect and the ledger or value it concerns. Fix the voucher, or the master it names in Tally, then build again."
                            }}),
                            evidence: accumulated.clone(),
                            company_guid: Some(payload.company_guid),
                            truncated: false,
                        });
                    }
                }
            }
            // Only a payload carrying a cash/bank voucher reads the group
            // collection. Every payload, a Journal-only one included, reads
            // each named ledger's bill-wise flag (#1234), from the ledger list
            // it already reads, so no request is added.
            let mut group_evidence = None;
            let mut statement_warnings: Vec<Value> = Vec::new();
            if renders_bank_shape(&payload.vouchers) || !resolved.cash_ledgers.is_empty() {
                let (groups, evidence) = self.read_group_collection(&identity, &company.name).await?;
                accumulated = combine_evidence(accumulated.clone(), evidence.clone());
                let observed = ObservedMasters::new(ledger_masters.catalog().parents(), groups);
                let refusals =
                    cash_bank_refusals(&payload, &observed, self.settings.max_bytes);
                if refusals.is_refused() {
                    return Ok(ToolOutcome {
                        payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                            "state":"refused", "reason":"cash_bank_ledger_not_established",
                            "refused_ledgers":refusals.ledgers, "refused_leg_count":refusals.legs,
                            "refused_ledgers_omitted":refusals.omitted,
                            "group_evidence_sha256":evidence.response_sha256,
                            "next_step":"No file was written. Each refused leg says which ledger and why: cash_bank must reach Bank Accounts or Cash-in-Hand, not_cash_bank must reach a group holding no money, and an unresolvable group is refused either way. Fix the payload or the ledger's group, then build again. Raise BRIDGE_AGENT_MAX_BYTES if refused_ledgers_omitted is above zero."
                        }}),
                        evidence: accumulated.clone(),
                        company_guid: Some(payload.company_guid),
                        truncated: false,
                    });
                }
                // A ledger a person named as cash in hand must be one: a bank
                // ledger there would move the cash bank to bank, which the
                // other statement's line then posts a second time.
                if let Some((reason, refused, omitted)) = answered_ledger_refusals(
                    &resolved.cash_ledgers,
                    &observed,
                    self.settings.max_bytes,
                ) {
                    return Ok(ToolOutcome {
                        payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                            "state":"refused", "reason":reason,
                            "refused_ledgers":refused,
                            "refused_ledgers_omitted":omitted,
                            "group_evidence_sha256":evidence.response_sha256,
                            "next_step":"No file was written. Each ledger was named in answering a bank cash line. requires cash_in_hand: the answer named the cash-in-hand ledger, but this one's group reaches the reserved group shown. requires not_suspense: the answer was not dont_know, but the ledger sits under Suspense A/c; only a dont_know answer may post a cash line there, where it is tagged and counted. Re-run parse_bank_statement with the right ledger, then build again. Raise BRIDGE_AGENT_MAX_BYTES if refused_ledgers_omitted is above zero."
                        }}),
                        evidence: accumulated.clone(),
                        company_guid: Some(payload.company_guid),
                        truncated: false,
                    });
                }
                if let Some(ledgers) = &resolved.statement_ledgers {
                    let findings = statement_ledger_findings(ledgers, &observed, |name| {
                        ledger_masters.catalog().parents().any(|(known, _)| known == name)
                    });
                    if findings.bank_in_cash_in_hand {
                        let row = refused_ledger_row(
                            &ledgers.bank_ledger,
                            "bank",
                            &observed.classify(&ledgers.bank_ledger),
                            None,
                        );
                        let mut budget = refusal_diagnostic_budget(self.settings.max_bytes);
                        let (refused, omitted) = super::bank_statement::bounded(vec![row], &mut budget);
                        return Ok(ToolOutcome {
                            payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                                "state":"refused", "reason":"statement_bank_ledger_not_a_bank",
                                "refused_ledgers":refused, "refused_ledgers_omitted":omitted,
                                "group_evidence_sha256":evidence.response_sha256,
                                "next_step":"No file was written. The bank ledger this statement was parsed for is under Cash-in-Hand, and a statement belongs to a bank account. Parse the statement again with the ledger of the bank account that issued it, which must be under Bank Accounts or Bank OD A/c."
                            }}),
                            evidence: accumulated.clone(),
                            company_guid: Some(payload.company_guid),
                            truncated: false,
                        });
                    }
                    statement_warnings = findings.suspense.warning(&ledgers.suspense_ledger);
                }
                group_evidence = Some(evidence);
            }
            validate_dates(&payload, company.books_from.as_deref())?;
            let _admission_lock = self.lock_import_admission()?;
            // Admit the journal before publication; labels in older batches do not
            // collide with this build's independently generated wire identities.
            self.import_snapshot_while_admitted(None)?;
            // The admission above read the stop before this lock; a post may
            // have gone out since. No file is written for a stopped company.
            if payload.vouchers.iter().any(|voucher| voucher.voucher_type.is_invoice())
                && self
                    .import_invoice_stop_while_admitted(&payload.company_guid)?
                    .is_some()
            {
                return Err(ToolFailure::from("invoice_company_stopped".to_string())
                    .with_prior_evidence(accumulated.clone()));
            }
            let lineage = match payload.amends_batch_id {
                Some(_) => Some(self.amendment_lineage_while_admitted(&payload)?),
                None => None,
            };
            // A row another batch of this company already sent to Tally is not
            // written out again, or a hand import of this file would post it a
            // second time (#876). An amendment alters vouchers in place and adds
            // none, so it is exempt. The refusal carries the blocking batch: an
            // agent told only "already posted" rebuilds or hunts for it.
            if lineage.is_none() {
                if let Some(blocking) = self.import_vouchers_already_posted_while_admitted(
                    &canonical_batch_guid(&payload.company_guid),
                    &payload.vouchers,
                )? {
                    return Ok(ToolOutcome {
                        payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {"error": {
                            "code":"import_txn_already_posted",
                            "message":"No file was written: another batch of this company already sent, or was found to have posted, a row of this batch.",
                            "blocking_batch_id":blocking,
                            "next_step":BUILD_TXN_ALREADY_POSTED_NEXT_STEP
                        }}}),
                        evidence: Evidence {
                            state: "partial",
                            reason_code: Some("import_txn_already_posted".to_string()),
                            ..accumulated.clone()
                        },
                        company_guid: Some(payload.company_guid),
                        truncated: false,
                    });
                }
            }
            // Which of the named ledgers keep bills in Tally, as the catalogue
            // read above said it (#1234: the flag rides the
            // catalogue the build already reads, so no further request is made
            // and the catalogue repeat below holds the flags to the same
            // byte-for-byte stability as the names). Entries on such a ledger
            // with no bill allocation land On Account (#1234). Judged after the
            // already-posted check, so nobody is asked to approve a batch that
            // is refused for that reason, and before the mark, so a refusal
            // here costs no further read. A later check can still refuse a
            // batch whose approval was asked.
            let digest_company = import_company_tuple(&company)?;
            let digest_endpoint = super::canonical_loopback_origin(&self.settings.endpoint)
                .map_err(|_| "host_setting_invalid".to_string())?;
            let bill_wise_context = bill_wise::DigestContext {
                company: &digest_company,
                endpoint_origin: &digest_endpoint,
                amends_batch_id: payload.amends_batch_id.as_deref(),
                batch_content: bill_wise::batch_content_digest(&payload.vouchers),
            };
            let observed_flags = bill_wise::ObservedBillWise::from_catalogue(
                &ledger_masters,
                bill_wise::named_ledgers(&payload),
            );
            let parties =
                bill_wise::bill_wise_parties(&payload.vouchers, &observed_flags);
            let verdict = bill_wise::judge_approvals(&approvals, &parties, &bill_wise_context)
                .map_err(approval_invalid)?;
            let names_masked = self.settings.redaction == super::Redaction::MaskParties;
            if !verdict.unapproved.is_empty() {
                let unapproved =
                    bill_wise::in_listing_order(verdict.unapproved, names_masked, |party| &party.1);
                let (refused, omitted) = bill_wise::refused_parties_json(
                    &unapproved,
                    refusal_diagnostic_budget(self.settings.max_bytes),
                );
                return Ok(ToolOutcome {
                    payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                        "state":"refused", "reason":"bill_wise_party_unapproved",
                        "refused_parties":refused,
                        "refused_party_count":unapproved.len(),
                        "refused_parties_omitted":omitted,
                        "bill_wise_response_sha256":[catalogue_evidence.response_sha256.clone()],
                        "next_step":BILL_WISE_UNAPPROVED_NEXT_STEP
                    }}),
                    evidence: accumulated.clone(),
                    company_guid: Some(payload.company_guid),
                    truncated: false,
                });
            }
            let on_account_approved = verdict.approved;
            let (mark, mark_evidence) = self.pre_import_mark(&company, &identity).await?;
            accumulated = combine_evidence(accumulated.clone(), mark_evidence.clone());
            let (_, _, _, repeated_catalogue_evidence) = self
                .read_import_ledger_catalogue(&identity, &company.name)
                .await?;
            accumulated = combine_evidence(accumulated.clone(), repeated_catalogue_evidence.clone());
            // Compare the complete captured catalogue, including identities and parents.
            // This proves stability across these observations, not an atomic snapshot.
            if catalogue_evidence.response_sha256 != repeated_catalogue_evidence.response_sha256 {
                return Err("import_catalogue_changed".to_string().into());
            }
            // The classification that admitted these legs rests on the group
            // collection as much as on the catalogue, so hold it to the same
            // stability requirement rather than trusting a single observation.
            if let Some(group_evidence) = group_evidence {
                let (_, repeated_group_evidence) =
                    self.read_group_collection(&identity, &company.name).await?;
                accumulated = combine_evidence(accumulated.clone(), repeated_group_evidence.clone());
                if group_evidence.response_sha256 != repeated_group_evidence.response_sha256 {
                    return Err("import_groups_changed".to_string().into());
                }
            }
            // The flags came in the catalogue rows, so the byte comparison above
            // already holds them to the stability it holds the names to: a
            // ledger switched to or from bill-wise between the two reads is
            // `import_catalogue_changed`.
            let (date_from, date_to) = match &lineage {
                // The window must hold each voucher where it is now as well as
                // where the amendment moves it, or both checks miss it.
                Some(lineage) => lineage.window(&payload.vouchers),
                None => (
                    payload
                        .vouchers
                        .iter()
                        .map(|voucher| voucher.date.clone())
                        .min()
                        .unwrap_or_default(),
                    payload
                        .vouchers
                        .iter()
                        .map(|voucher| voucher.date.clone())
                        .max()
                        .unwrap_or_default(),
                ),
            };
            // Exercise the exact future readback projection before publishing a file.
            // This observes today's source, not a bound on later Tally mutations.
            // The high-water mark was read just above, so the pre-flight bound
            // costs no further read for a book it already proves small.
            //
            // This read was a single request before the bound. It now also
            // halves a part Tally cannot serve (#485), as verify_import does:
            // it is the same request shape over the same window as the
            // verification it precedes, and a preflight stricter than that
            // verification would refuse to build a batch that could be verified.
            let preflight_read = self
                .read_verification_window(
                    &identity,
                    &company.name,
                    (&date_from, &date_to),
                    super::WindowPlanSource::Estimate {
                        known_marks: mark.value.zip(mark.master_value).map(
                            |(vouchers, masters)| super::CompanyMarks { vouchers, masters },
                        ),
                    },
                )
                .await?;
            if let Some(estimate) = preflight_read.preflight_evidence {
                accumulated = combine_evidence(accumulated.clone(), estimate);
            }
            let (preflight, preflight_evidence) = (preflight_read.source, preflight_read.evidence);
            accumulated = combine_evidence(accumulated.clone(), preflight_evidence.clone());
            if let Some(closing) = preflight_read.closing_evidence {
                accumulated = combine_evidence(accumulated.clone(), closing);
            }
            verification_window_identities(&preflight, &date_from, &date_to)?;
            let amendment = match &lineage {
                Some(lineage) => match lineage.compare_and_swap(
                    &payload.vouchers,
                    &preflight,
                    &verified_baselines(&self.imports_dir()?, lineage),
                )? {
                    Ok(vouchers) => Some(json!({
                        "amends_batch_id": payload.amends_batch_id,
                        "identity_batch_id": lineage.identity_batch_id,
                        "vouchers": vouchers,
                    })),
                    Err(refused) => {
                        return Ok(ToolOutcome {
                            payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                                "state":"refused", "reason":"amended_vouchers_not_as_built",
                                "amends_batch_id": payload.amends_batch_id,
                                "identity_batch_id": lineage.identity_batch_id,
                                "refused_vouchers": refused,
                                "window": {"from": date_from, "to": date_to},
                                "response_sha256": preflight_evidence.response_sha256,
                                "next_step": AMENDMENT_REFUSED_NEXT_STEP
                            }}),
                            evidence: accumulated.clone(),
                            company_guid: Some(payload.company_guid),
                            truncated: false,
                        });
                    }
                },
                None => None,
            };
            let verification_preflight = json!({
                "state":"current_window_readable", "from":date_from, "to":date_to,
                "source_rows":preflight.rows.len(),
                "paired_source_bytes":preflight_evidence.bytes,
                "response_sha256":preflight_evidence.response_sha256
            });
            let closing_profile = self.qualified_import_profile().await?;
            accumulated = combine_evidence(accumulated.clone(), closing_profile.evidence);
            if closing_profile.admission_key != opening_profile.admission_key {
                return Err("import_mode_changed_during_build".to_string().into());
            }
            let batch_id = format!("bridge-{}", Uuid::new_v4());
            let amends_batch_id = lineage
                .as_ref()
                .map(|lineage| lineage.identity_batch_id.clone());
            let xml = render_import_xml(
                &company.name,
                &payload.vouchers,
                amends_batch_id.as_deref().unwrap_or(&batch_id),
            );
            let sha256 = sha256_hex(xml.as_bytes());
            let line = ImportLedgerLine {
                batch_id: batch_id.clone(),
                identity_scheme: Some(ImportIdentityScheme::BatchV1),
                amends_batch_id,
                company_guid: canonical_batch_guid(&payload.company_guid),
                endpoint_origin: Some(super::canonical_loopback_origin(&self.settings.endpoint).map_err(|_| "host_setting_invalid".to_string())?),
                company: Some(import_company_tuple(&company)?),
                txn_ids: payload
                    .vouchers
                    .iter()
                    .map(|voucher| voucher.bridge_txn_id.clone())
                    .collect(),
                date_from,
                date_to,
                sha256: sha256.clone(),
                built_at: now(),
                status: "built".to_string(),
                pre_import_mark: mark,
                vouchers: payload.vouchers,
                ledger_identities: Some(build_binding),
                cash_in_hand_ledgers: Some(
                    resolved
                        .cash_ledgers
                        .iter()
                        .filter(|need| need.cash_in_hand)
                        .map(|need| CashInHandLedger {
                            bridge_txn_id: need.bridge_txn_id.clone(),
                            ledger: need.ledger.clone(),
                        })
                        .collect(),
                ),
                on_account_approved: Some(on_account_approved),
            };
            // An invoice that post_import would refuse writes no file either:
            // it is refused here with the code post_import would give.
            if holds_an_invoice(&line) {
                if let Err(code) = post::admit_saved_voucher(
                    &line,
                    &self.settings.endpoint,
                    post::PostScope::Vouchers,
                    self.post_voucher_limit(post::PostScope::Vouchers),
                ) {
                    return Err(invoice_route_refusal(code).into());
                }
            }
            let imports = self.imports_dir()?;
            let path = imports.join(format!("{batch_id}.xml"));
            if let Some(error) = persistence::persist_build(&imports, &line, xml.as_bytes(), || {
                self.append_import_ledger_while_admitted(&line)
            })? {
                return Ok(ToolOutcome {
                    payload: json!({"result":{"batch_id":batch_id,"error":{"code":error,
                        "message":"Local batch publication is incomplete; reconcile its recovery journal before continuing."}}}),
                    evidence: Evidence {
                        state: "partial",
                        reason_code: Some(error),
                        ..accumulated.clone()
                    },
                    company_guid: Some(line.company_guid.clone()),
                    truncated: false,
                });
            }
            // Reuse the posting admission path to describe only a route that
            // this exact saved batch can take. A manual-only batch is still a
            // successful build.
            // Keep the refusal code: `post_import` returns this same code, and
            // the guidance names it rather than guessing a reason (#866).
            let post_voucher_limit = self.post_voucher_limit(post::PostScope::Vouchers);
            let native_post_refusal = if self.settings.writes_enabled {
                post::admit_saved_voucher(
                    &line,
                    &self.settings.endpoint,
                    post::PostScope::Vouchers,
                    post_voucher_limit,
                )
                .err()
            } else {
                None
            };
            let (mut warnings, next_step) = build_import_guidance(
                self.settings.writes_enabled,
                native_post_refusal.as_deref(),
                post_voucher_limit,
                renders_bank_shape(&line.vouchers),
                line.on_account_approved
                    .as_deref()
                    .is_some_and(|approved| !approved.is_empty()),
                line.vouchers.iter().any(|voucher| {
                    voucher.voucher_type.bank_shape().is_some() && voucher.entries.len() > 2
                }),
                line.vouchers.iter().any(|voucher| invoice::new_ref_party(voucher).is_some()),
            );
            let next_step = if holds_an_invoice(&line) {
                invoice_guidance(&mut warnings)
            } else {
                next_step
            };
            let next_step = match &amendment {
                Some(_) => {
                    if let Some(list) = warnings.as_array_mut() {
                        // Native posting refuses every amendment, so the
                        // generic first message would give the wrong reason.
                        if let Some(first) = list.first_mut() {
                            *first = json!(AMENDMENT_NOT_POSTABLE);
                        }
                        list.insert(0, json!(AMENDMENT_WARNING));
                    }
                    AMENDMENT_NEXT_STEP
                }
                None => next_step,
            };
            Ok(ToolOutcome {
                payload: json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": {
                    "batch_id": batch_id, "path": path, "sha256": sha256,
                    "amendment": amendment,
                    "voucher_count": line.vouchers.len(), "total_debit": debit.as_str(), "total_credit": credit.as_str(),
                    // How many lines a bank import sent to suspense, by tag,
                    // so none sits there uncounted.
                    "suspense_lines": tagged_suspense_vouchers(&line.vouchers),
                    "live_evidence": live_evidence(&line.vouchers),
                    "verification_preflight": verification_preflight,
                    "identity_scheme": line.identity_scheme,
                    // The bill-wise ledgers a person approved, each name
                    // marked as a party name, with the digest the approval
                    // was tied to, and the catalogue response the flags came
                    // from (#1234).
                    "on_account_approved": line.on_account_approved.as_ref().map(|approved| {
                        bill_wise::approved_json(&bill_wise::in_listing_order(
                            approved.clone(),
                            names_masked,
                            |party| &party.party_digest,
                        ))
                    }),
                    "bill_wise_response_sha256": [catalogue_evidence.response_sha256.clone()],
                    // The fifth element of §9.13's identity tuple. It is
                    // recorded on the batch and compared on dispatch, but a
                    // hand import never reaches that check — so the operator
                    // is asked to compare it and must be able to see it.
                    "endpoint_origin": line.endpoint_origin,
                    "observed_profile": opening_profile.observed_profile,
                    "warnings": warnings,
                    "statement_ledger_warnings": statement_warnings,
                    "next_step": next_step
                }}),
                evidence: accumulated.clone(),
                company_guid: Some(line.company_guid.clone()),
                truncated: false,
            })
        }
        .await;
        result.map_err(|failure| failure.with_prior_evidence(accumulated))
    }

    /// The tool: page 1 verifies against Tally; a later page (`proof_sha256`
    /// and `offset`) is served from the proof that verification persisted and
    /// never reads Tally again, so every page describes one verification
    /// (bridge#627).
    pub(super) async fn verify_import(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let offset = arg_usize(args, "offset", 0)?;
        if let Some(proof_sha256) = optional_string(args, "proof_sha256")? {
            return self.verify_import_page(args, &proof_sha256, offset);
        }
        if offset != 0 {
            return Err("verification_page_requires_proof".to_string().into());
        }
        let batch_id = required_string(args, "batch_id")?;
        let mut outcome = self
            .verify_import_with_dispatch(args, false, &mut None, None, &mut None, None)
            .await?;
        let evidence = outcome.evidence.clone();
        let persisted = self
            .read_persisted_proof(batch_id)
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;
        // Page 1 is built from the bytes read back, not from memory, so the
        // page and the hash it names are the same file even if another
        // verification replaced it in between.
        let (proof, page) = served_verification_page(&persisted, 0)
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;
        if proof["batch_id"] != batch_id {
            return Err(
                ToolFailure::from("verification_proof_batch_mismatch".to_string())
                    .with_prior_evidence(evidence),
            );
        }
        self.admit_verification_page(&page)
            .map_err(|failure| failure.with_prior_evidence(evidence))?;
        outcome.payload["result"] = page;
        Ok(outcome)
    }

    /// A later page of a verification, from its persisted proof only (see
    /// [`served_verification_page`]).
    fn verify_import_page(
        &self,
        args: &Value,
        proof_sha256: &str,
        offset: usize,
    ) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let batch_id = required_string(args, "batch_id")?;
        let persisted = self.read_persisted_proof(batch_id)?;
        let sha256 = sha256_hex(&persisted);
        // A newer verification replaced the proof this page belongs to.
        if sha256 != proof_sha256 {
            return Err("verification_proof_changed".to_string().into());
        }
        let (proof, page) = served_verification_page(&persisted, offset)?;
        if proof["batch_id"] != batch_id
            || !proof["company"]["guid"]
                .as_str()
                .is_some_and(|recorded| batch_guid_matches(recorded, guid))
        {
            return Err("verification_proof_batch_mismatch".to_string().into());
        }
        self.admit_verification_page(&page)?;
        Ok(ToolOutcome {
            payload: json!({"company": proof["company"], "result": page}),
            evidence: Evidence {
                request_sha256: sha256_hex(
                    format!("verify_import_page:{batch_id}:{offset}").as_bytes(),
                ),
                response_sha256: sha256,
                bytes: persisted.len(),
                state: "complete",
                read_at: None,
                duration_ms: None,
                reason_code: None,
            },
            company_guid: Some(guid.to_string()),
            truncated: false,
        })
    }

    /// The parts of a page that are never cut must fit on their own: the byte
    /// cap may shorten only the verified rows, so otherwise the refusal is
    /// typed here rather than lost as a generic oversize. Tally's LINEERROR
    /// text is not essential: the cap drops it first, so it is not counted.
    fn admit_verification_page(&self, page: &Value) -> Result<(), ToolFailure> {
        let mut essential = page.clone();
        essential["items"] = json!([]);
        super::drop_tally_line_error_text(&mut essential);
        if essential.to_string().len() > self.settings.max_bytes {
            return Err("verification_too_large_to_report".to_string().into());
        }
        Ok(())
    }

    /// [`Self::verify_import`] for `acknowledge_post_review`, which also needs
    /// the rows the readback observed, to bind the voucher a person reviews.
    async fn verify_for_review(
        &self,
        args: &Value,
        rows: &mut Option<Vec<ReadVoucher>>,
    ) -> Result<ToolOutcome, ToolFailure> {
        self.verify_import_with_dispatch(args, false, &mut None, None, rows, None)
            .await
    }

    /// [`Self::verify_import`] for `post_import`, which then sends the batch's
    /// whole verification window as one request inside the dispatch lease. That
    /// request is admitted here on what this verification read of the same
    /// window measured (§11c), and refused otherwise, before any approval.
    pub(super) async fn verify_import_for_post(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let mut served = None;
        let outcome = self
            .verify_import_with_dispatch(args, false, &mut served, None, &mut None, None)
            .await?;
        post::admit_post_window(served).map_err(|code| {
            ToolFailure::from(code).with_prior_evidence(outcome.evidence.clone())
        })?;
        Ok(outcome)
    }

    /// The readback right after this call's own POST. `masters_after_post`
    /// is the check of the company's masters across the post (#239); it goes
    /// into the proof before it is persisted, so a downgrade is recorded too.
    /// `after_post_mark` is the target's voucher mark read just after the POST,
    /// `None` when that read failed: the post is then bound by the rules for a
    /// mark that was never measured, never by a guess.
    pub(in crate::agent) async fn verify_import_after_current_dispatch(
        &self,
        args: &Value,
        masters_after_post: Value,
        after_post_mark: Option<u64>,
    ) -> Result<ToolOutcome, ToolFailure> {
        self.verify_import_with_dispatch(
            args,
            true,
            &mut None,
            Some(masters_after_post),
            &mut None,
            after_post_mark,
        )
        .await
    }

    async fn verify_import_with_dispatch(
        &self,
        args: &Value,
        current_dispatch: bool,
        served: &mut Option<super::WindowServed>,
        masters_after_post: Option<Value>,
        observed_rows: &mut Option<Vec<ReadVoucher>>,
        after_post_mark: Option<u64>,
    ) -> Result<ToolOutcome, ToolFailure> {
        let guid = required_string(args, "company_guid")?;
        let batch_id = required_string(args, "batch_id")?;
        let ledger::BatchSnapshot {
            batch: line,
            mut generation,
            response: dispatch_response,
            dispatched,
            pre_post_voucher_mark,
            span_verdict,
            ..
        } = self
            .latest_import_snapshot(batch_id)?
            .ok_or_else(|| "import_batch_not_found".to_string())?;
        if !batch_guid_matches(&line.company_guid, guid) {
            return Err("import_batch_company_mismatch".to_string().into());
        }
        validate_dispatched_import_endpoint(&line, dispatched, &self.settings.endpoint)?;
        let opening_mode = self.observe_import_profile().await?;
        let (company, identity, identity_evidence) = self
            .verified_company(guid)
            .await
            .map_err(|failure| failure.with_prior_evidence(opening_mode.evidence.clone()))?;
        let mut accumulated =
            combine_evidence(opening_mode.evidence.clone(), identity_evidence.clone());
        let result: Result<ToolOutcome, ToolFailure> = async {
            if line.company.as_ref() != Some(&import_company_tuple(&company)?) {
                return Err("company_identity_mismatch".to_string().into());
            }
            let window = (line.date_from.as_str(), line.date_to.as_str());
            let observed_read = self
                .read_verification_window(
                    &identity,
                    &company.name,
                    window,
                    super::WindowPlanSource::Estimate { known_marks: None },
                )
                .await?;
            if let Some(preflight) = observed_read.preflight_evidence {
                accumulated = combine_evidence(accumulated.clone(), preflight);
            }
            *served = Some(super::WindowServed::of(
                &observed_read.reads,
                &observed_read.evidence,
                observed_read.refused_a_part,
            ));
            let (observed, observed_evidence) = (observed_read.source, observed_read.evidence);
            accumulated = combine_evidence(accumulated.clone(), observed_evidence.clone());
            if let Some(closing) = observed_read.closing_evidence {
                accumulated = combine_evidence(accumulated.clone(), closing);
            }
            // The corroborating read replays the ranges the first one actually
            // read, rather than planning again: it must observe the same parts.
            let corroboration_read = self
                .read_verification_window(
                    &identity,
                    &company.name,
                    window,
                    super::WindowPlanSource::replay_of(observed_read.reads, observed_read.witness),
                )
                .await?;
            let (corroboration, corroboration_evidence) =
                (corroboration_read.source, corroboration_read.evidence);
            accumulated = combine_evidence(accumulated.clone(), corroboration_evidence.clone());
            if let Some(closing) = corroboration_read.closing_evidence {
                accumulated = combine_evidence(accumulated.clone(), closing);
            }
            // The window may have been served in parts, so there is no single
            // response to hash. The evidence's own response digest already folds
            // every part that was read, which is the honest commitment here.
            let voucher_read_sha256 = observed_evidence.response_sha256.clone();
            corroborate_verification_window(&observed, &corroboration, &line.date_from, &line.date_to)?;
            let span = match pre_post_voucher_mark {
                Some(pre_post_voucher_mark) => {
                    let (current, mark_evidence) =
                        self.current_voucher_mark(&company, &identity).await?;
                    accumulated = combine_evidence(accumulated.clone(), mark_evidence);
                    let decision = self.decide_post_span(
                        &line,
                        span_identity::PreMark::recorded(pre_post_voucher_mark),
                        span_verdict,
                        dispatch_response.as_ref(),
                        &observed,
                        after_post_mark,
                        current,
                        generation,
                    );
                    // This verification's own verdict record moved the journal:
                    // its status is persisted against the record it appended.
                    if let Some(recorded) = decision.recorded_at {
                        generation = recorded;
                    }
                    decision
                }
                None => PostSpanDecision::not_applicable(),
            };
            // A post that recorded its pre-POST mark sent no tag, so only its
            // binding attributes it; a tag in the book is a hand import's.
            let attribution = if pre_post_voucher_mark.is_some() {
                Attribution::Span(span.bindings.as_deref())
            } else {
                Attribution::Tag
            };
            // An invoice is recognised by its number, and by its figures as
            // well while this machine holds an unsettled batch with them.
            let mut result = self.verify_batch_by_journal_identity(&line, &observed, attribution)?;
            // The standard readback sees a voucher's date, type, number, entries
            // and narration. An invoice's party, GST header, reference and bill
            // allocation are read back separately; "posted_verified" is kept
            // only when those match as well. An invoice of a type this build
            // does not qualify is marked not confirmed and is not read back.
            if let Some((invoice_voucher, matched)) = verification::invoice_readback_due(
                &mut result,
                &line.vouchers,
                LIVE_QUALIFIED_VOUCHER_TYPES,
            ) {
                let (mut differences, alter_id, guid, evidence) = self
                    .read_back_sales_invoice(&identity, &company, invoice_voucher)
                    .await?;
                accumulated = combine_evidence(accumulated.clone(), evidence);
                // The voucher found by type and number must be the one the
                // standard readback attributed to this batch.
                if differences.is_empty() {
                    differences = invoice::readback_identity_differences(
                        guid.as_deref(),
                        alter_id.as_deref(),
                        &matched,
                    );
                }
                verification::mark_invoice_readback(
                    &mut result,
                    &invoice_voucher.bridge_txn_id,
                    &differences,
                );
            }
            if span.rolled_back {
                verification::mark_book_rolled_back(&mut result);
            } else if pre_post_voucher_mark.is_some() && span.bindings.is_none() {
                // An untagged native post that is not bound: what its content
                // cannot find is never absent. When the post's own answer said
                // Tally created none of its vouchers and none is found (for a
                // batch, with the voucher mark measured unmoved), say so
                // (bridge#1108); otherwise an edit in Tally is as likely. Only
                // the post's own readback reads that answer: by a later
                // verification someone may have entered a voucher by hand and
                // edited it.
                let counters = current_dispatch
                    .then(|| {
                        dispatch_response
                            .as_ref()
                            .and_then(|response| response.outcome.as_ref())
                            .map(|outcome| outcome.counters())
                    })
                    .flatten();
                let voucher_step =
                    verification::measured_voucher_step(pre_post_voucher_mark, after_post_mark);
                match verification::unmatched_cause(
                    counters,
                    line.vouchers.len(),
                    verification::unmatched_count(&result),
                    voucher_step,
                ) {
                    verification::UnmatchedCause::ReportedNotCreated => {
                        verification::mark_reported_not_created(&mut result)
                    }
                    verification::UnmatchedCause::NotEstablished => {
                        verification::mark_sent_not_attributed(&mut result)
                    }
                }
            }
            let mut closing_mode_evidence = None;
            if result["counts"]["not_found"].as_u64().unwrap_or(0) > 0 {
                // Positive rows are direct observations. Absence additionally requires
                // the product, release and licence tier qualified by the live slice.
                if let Err(refusal) = opening_mode.qualification {
                    return Err(refusal.verification_code().to_string().into());
                }
                let closing_mode = self.observe_import_profile().await?;
                accumulated = combine_evidence(accumulated.clone(), closing_mode.evidence.clone());
                if let Err(refusal) = closing_mode.qualification {
                    return Err(refusal.verification_code().to_string().into());
                }
                if closing_mode.admission_key != opening_mode.admission_key {
                    return Err("verification_mode_changed_during_read".to_string().into());
                }
                closing_mode_evidence = Some(closing_mode.evidence);
            }
            let proof = json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "batch_id": line.batch_id, "batch_sha256": line.sha256,
                "built_at": line.built_at, "verified_at": now(),
                "dispatch_response": dispatch_response,
                "pre_import_mark": line.pre_import_mark,
                "alter_id_delta": alter_id_delta(
                    pre_post_voucher_mark.map_or(DeltaBasis::Build(&line.pre_import_mark), DeltaBasis::PrePost),
                    &observed.rows,
                ),
                "counts": result["counts"], "vouchers": result["vouchers"], "duplicates": result["duplicates"],
                "post_span_binding": with_post_span_summary(span.report, &result["vouchers"]),
                "unrelated_duplicates_in_window": result["unrelated_duplicates_in_window"],
                "evidence": {"mode_opening": opening_mode.evidence, "mode_closing": closing_mode_evidence, "company": identity_evidence, "voucher_read": observed_evidence, "voucher_read_corroboration": corroboration_evidence, "voucher_read_sha256": voucher_read_sha256}
            });
            // This call's own check, or the doubt recorded when this batch was
            // posted: a later readback, which compares by name, never clears it.
            let masters_after_post = match masters_after_post {
                Some(masters) => Some(masters),
                None if dispatched => {
                    match read_masters_check(&self.imports_dir()?, &line.batch_id) {
                        // The check after the post did not finish: finish it
                        // now against the ledgers bound at build, which the
                        // post required to match the approved ones (#616).
                        // Only once the vouchers are found: before the POST
                        // lands, a verdict would vouch for a post not yet made.
                        Some(check)
                            if check["state"] == MASTERS_CHECK_PENDING
                                && verification_status(&proof, line.vouchers.len())
                                    == "posted_verified" =>
                        {
                            let bound = line
                                .ledger_identities
                                .iter()
                                .flatten()
                                .map(|bound| (bound.name.clone(), bound.guid.clone()))
                                .collect::<Vec<_>>();
                            let verdict = if bound.is_empty() {
                                json!({"state":"check_unavailable","trigger":MASTERS_CHECK_PENDING})
                            } else {
                                self.ledgers_still_approved(
                                    &identity,
                                    &company.name,
                                    &bound,
                                    MASTERS_CHECK_PENDING,
                                    &mut accumulated,
                                )
                                .await
                            };
                            Some(self.record_masters_verdict_for(
                                &line.batch_id,
                                verdict,
                                line.vouchers.len() > 1,
                            ))
                        }
                        recorded => recorded,
                    }
                }
                None => None,
            };
            let mut proof = proof;
            if let Some(masters) = &masters_after_post {
                proof["masters_after_post"] =
                    served_masters_verdict(masters.clone(), &line.vouchers);
            }
            // Whether a person's recorded review still covers this doubt and
            // this voucher (#239). Beside the verdict, never instead of it.
            if dispatched {
                // Adds no way for a verification to fail: without the imports
                // directory there is no record to report.
                if let Some(review) = self
                    .imports_dir()
                    .ok()
                    .and_then(|imports| ack::operator_review(&imports, &line, &observed.rows, span.bindings.as_deref()))
                {
                    proof["operator_review"] = review;
                }
            }
            *observed_rows = Some(observed.rows.clone());
            let mut payload = json!({"company": company_json(&company, std::slice::from_ref(&company)), "result": proof});
            if dispatched {
                if current_dispatch {
                    post::finalize_current_dispatch(
                        &mut payload,
                        dispatch_response.as_ref(),
                        masters_after_post.as_ref(),
                        line.vouchers.len(),
                    );
                } else {
                    post::finalize_previous_attempt_reconciliation(
                        &mut payload,
                        dispatch_response.as_ref(),
                        masters_after_post.as_ref(),
                        line.vouchers.len(),
                    );
                }
            }
            let status = final_verification_status(
                dispatched.then(|| &payload["result"]["dispatch"]),
                &result,
                line.vouchers.len(),
            );
            payload["result"]["verification_status"] = json!(status.as_str());
            // Marked before it is saved, so the proof holds the marks too.
            mark_verification_names(&mut payload["result"]);
            self.persist_import_verification(&payload["result"], &line, status, generation)?;
            Ok(ToolOutcome {
                payload,
                evidence: accumulated.clone(),
                company_guid: Some(guid.to_string()),
                truncated: false,
            })
        }
        .await;
        result.map_err(|failure| failure.with_prior_evidence(accumulated))
    }

    /// Publishes the proof files and the ledger line recording `status`, the
    /// verdict, for the batch `line` (bridge#814).
    fn persist_import_verification(
        &self,
        proof: &Value,
        line: &ImportLedgerLine,
        status: VerificationStatus,
        expected_generation: ledger::VerificationGeneration,
    ) -> Result<(), String> {
        let mut update = line.clone();
        update.status = status.as_str().into();
        let update = &update;
        let _admission_lock = self.lock_import_admission()?;
        let current = self.import_snapshot_while_admitted(Some(&update.batch_id))?;
        if current.map(|snapshot| snapshot.generation) != Some(expected_generation) {
            return Err("import_verification_conflict_retry".into());
        }
        let imports = self.imports_dir()?;
        // Saved with its party-name marks, so a page served from it masks
        // names as the response's redaction requires.
        let mut local_proof = proof.clone();
        // The files record the verdict the ledger records, whatever the caller's
        // copy says, so the proof and the ledger status cannot disagree.
        local_proof["verification_status"] = json!(update.status);
        let json = serde_json::to_vec_pretty(&local_proof)
            .map_err(|_| "proof_serialization_failed".to_string())?;
        let markdown = render_proof_markdown(&local_proof);
        persistence::publish_proofs(
            &imports,
            update,
            &json,
            markdown.as_bytes(),
            || self.append_import_record_while_admitted(&ledger::StatusRecord::from(update)),
            |_| Ok(()),
        )?;
        // The first verified ALTERID of each voucher, for a later amendment to
        // compare against (#239). Kept beside the proof, not in the journal, so
        // an older binary still reads the journal after a rollback. A failed
        // write leaves the previous baseline, or none, and an amendment of a
        // voucher it lacks then refuses: the safe direction, so it does not
        // fail this verification.
        let _ = record_verified_baseline(&imports, &update.batch_id, proof);
        Ok(())
    }

    /// The V1 ledger catalogue, for the reads outside the import family that
    /// need row spellings alone (presence, the drift re-read after a voucher
    /// window); `read_resolvable_ledgers` serves the tools that resolve a name.
    pub(super) async fn read_ledger_catalogue(
        &self,
        identity: &super::VerifiedCompanyIdentity,
        company_name: &str,
    ) -> Result<(Vec<String>, Evidence), ToolFailure> {
        let (catalogue, evidence) = self.read_v1_catalogue(identity, company_name).await?;
        Ok((catalogue.names().map(str::to_string).collect(), evidence))
    }

    /// The catalogue's ledgers as a request can reach them, for the tools that
    /// resolve a typed ledger name (#1085). The same V1 read as
    /// `read_ledger_catalogue`, which keeps the row spellings alone.
    pub(super) async fn read_resolvable_ledgers(
        &self,
        identity: &super::VerifiedCompanyIdentity,
        company_name: &str,
    ) -> Result<(Vec<super::ledger_candidates::CatalogueLedger>, Evidence), ToolFailure> {
        let (catalogue, evidence) = self.read_v1_catalogue(identity, company_name).await?;
        Ok((resolvable_ledgers(&catalogue), evidence))
    }

    /// The same catalogue read, with each ledger's immediate parent group as Tally returned it
    /// (`None` when it returned none), for the group summaries of `vouchers` (#1230).
    pub(super) async fn read_ledger_parents(
        &self,
        identity: &super::VerifiedCompanyIdentity,
        company_name: &str,
    ) -> Result<(Vec<(String, Option<String>)>, Evidence), ToolFailure> {
        let (catalogue, evidence) = self.read_v1_catalogue(identity, company_name).await?;
        Ok((owned_parents(&catalogue), evidence))
    }

    /// One catalogue read that serves both a typed ledger name (the spellings it resolves against) and
    /// the group placements (each ledger's parent), so a group summary with `ledger` reads the list once.
    pub(super) async fn read_resolvable_ledgers_with_parents(
        &self,
        identity: &super::VerifiedCompanyIdentity,
        company_name: &str,
    ) -> Result<
        (
            Vec<super::ledger_candidates::CatalogueLedger>,
            Vec<(String, Option<String>)>,
            Evidence,
        ),
        ToolFailure,
    > {
        let (catalogue, evidence) = self.read_v1_catalogue(identity, company_name).await?;
        Ok((
            resolvable_ledgers(&catalogue),
            owned_parents(&catalogue),
            evidence,
        ))
    }

    async fn read_v1_catalogue(
        &self,
        identity: &super::VerifiedCompanyIdentity,
        company_name: &str,
    ) -> Result<(bridge_tally_protocol::StandardLedgerCatalog, Evidence), ToolFailure> {
        let read = standard_ledger_catalog_read(company_name)
            .map_err(|_| "company_name_invalid".to_string())?;
        admit_standard_ledger_catalog_request(read.as_str().to_string())
            .map_err(|_| "ledger_export_invalid".to_string())?;
        let (xml, evidence) = self.post_read(identity, read).await?;
        let catalogue =
            parse_standard_ledger_catalog_response(&xml, company_name, identity.company_guid())
                .map_err(|error| catalogue_failure(error, &evidence))?;
        Ok((catalogue, evidence))
    }

    /// The import family's ledger catalogue: V1's rows, each with its
    /// `ISBILLWISEON` (#1234), so one read answers both which
    /// ledgers exist and which keep bills. Used by the build, `validate_masters`,
    /// the post and the queue's re-read.
    pub(super) async fn read_import_ledger_catalogue(
        &self,
        identity: &super::VerifiedCompanyIdentity,
        company_name: &str,
    ) -> Result<
        (
            Vec<String>,
            bridge_tally_protocol::StandardLedgerCatalogV2,
            AgentReadRequest,
            Evidence,
        ),
        ToolFailure,
    > {
        let read = import_ledger_catalogue_read(company_name)
            .map_err(|_| "company_name_invalid".to_string())?;
        let request = admit_standard_ledger_catalog_request(read.as_str().to_string())
            .map_err(|_| "ledger_export_invalid".to_string())?;
        let (xml, evidence) = self.post_read(identity, read).await?;
        let catalogue =
            parse_import_ledger_catalog_response(&xml, company_name, identity.company_guid())
                .map_err(|error| catalogue_failure(error, &evidence))?;
        Ok((
            catalogue.catalog().names().map(str::to_string).collect(),
            catalogue,
            request,
            evidence,
        ))
    }

    /// The company's group tree, read when a payload needs one leg
    /// classified as cash or bank and for the group summaries of `vouchers`. A
    /// ledger row carries a single `PARENT` hop and no `PARENTSTRUCTURE`, so the
    /// group identities live here.
    pub(super) async fn read_group_collection(
        &self,
        identity: &super::VerifiedCompanyIdentity,
        company_name: &str,
    ) -> Result<(Vec<bridge_tally_protocol::TallyNamedMaster>, Evidence), ToolFailure> {
        let (xml, evidence) = self
            .post_read(identity, native_group_snapshot_read(company_name))
            .await?;
        let groups =
            parse_native_group_snapshot(&xml, identity.company_guid()).map_err(|error| {
                let mut failure = ToolFailure::from("group_export_invalid".to_string())
                    .with_prior_evidence(evidence.clone());
                // The snapshot parser already names each refusal with a data-free
                // code; keep it as the cause instead of dropping it (bridge#676).
                failure.cause = crate::tally::approved_import::group_snapshot_cause(&error);
                failure
            })?;
        Ok((groups, evidence))
    }

    /// Read the whole verification window, divided before it is sent so that no
    /// request is predicted over the budget (protocol reference §11c), and
    /// divided again if Tally still cannot serve a part (#485).
    ///
    /// The window is never narrowed — only divided. Every sub-window is read and
    /// its rows concatenated, so the set of vouchers observed is identical to
    /// what one undivided read would have returned. That distinction matters
    /// because this read feeds an attribution check: filtering it by voucher
    /// identity would make it cheaper by making it see less, which is how a
    /// safety gate quietly stops being one. A date partition costs more requests
    /// and gives up nothing.
    ///
    /// The rows are admitted ONCE over the union. `admit` enforces identity
    /// uniqueness across the whole row set, so admitting each sub-window
    /// separately would check uniqueness only within each one and let a voucher
    /// duplicated across two sub-windows through — a hole that dividing would
    /// have opened and that the undivided read never had.
    async fn read_verification_window(
        &self,
        identity: &super::VerifiedCompanyIdentity,
        company: &str,
        (from, to): (&str, &str),
        source: super::WindowPlanSource,
    ) -> Result<VerificationWindowRead, ToolFailure> {
        // The window comes from the stored batch, not a tool argument: it is
        // parsed here, where it enters the window layer.
        let (from, to) = (
            super::parse_window_date(from)?,
            super::parse_window_date(to)?,
        );
        let shape = super::VoucherReadShape::ImportVerification;
        let read = self
            .read_voucher_window(
                identity,
                company,
                &from,
                &to,
                shape,
                source,
                super::WindowReadLimits::for_shape(shape),
                |xml| parse_import_voucher_rows(xml, identity.company_guid()),
            )
            .await?;
        let all_evidence = read.all_evidence();
        let source = ImportReadSource::admit(read.rows)
            .map_err(|failure| ToolFailure::from(failure).with_prior_evidence(all_evidence))?;
        Ok(VerificationWindowRead {
            source,
            evidence: read.evidence,
            preflight_evidence: read.preflight_evidence,
            closing_evidence: read.closing_evidence,
            reads: read.reads,
            witness: read.witness,
            refused_a_part: read.refused_a_part,
        })
    }

    /// Decides how a native post is attributed in this verification: by the
    /// bindings journaled for it, by binding it now, or not at all, and whether
    /// the book was rolled back after it. `current` is the company's voucher
    /// mark read in this verification (0 for a company holding no voucher).
    ///
    /// A journaled verdict is final. Without one, a clean response binds: on
    /// the span the post measured when `after_post_mark` was read, otherwise on
    /// the span its clean response implies (owner decision, 2026-10-02). A
    /// refusal is journaled and permanent. A failure to read or record is not,
    /// and neither is a span read without a Payment, Receipt or Contra's
    /// `EFFECTIVEDATE`: the next verification decides again, on the span the
    /// clean response implies (the same span the step had accepted). A post
    /// whose response was lost never binds.
    #[allow(clippy::too_many_arguments)]
    fn decide_post_span(
        &self,
        line: &ImportLedgerLine,
        pre_post_voucher_mark: span_identity::PreMark,
        journaled: Option<ledger::PostSpanVerdict>,
        response: Option<&ledger::DispatchResponse>,
        observed: &ImportReadSource,
        after_post_mark: Option<u64>,
        current: u64,
        generation: ledger::VerificationGeneration,
    ) -> PostSpanDecision {
        let before = pre_post_voucher_mark.value();
        let outcome = response.and_then(|response| response.outcome.as_ref());
        // After a create of N, the mark never again reads below before + N, nor
        // below a mark read after the post, unless the book was rolled back
        // (a backup restored, or another copy put in its place).
        let floor = before
            .saturating_add(outcome.map_or(0, |outcome| outcome.counters().created))
            .max(after_post_mark.unwrap_or(0));
        if current < floor {
            return PostSpanDecision {
                rolled_back: true,
                ..PostSpanDecision::reported(
                    json!({"state":"book_rolled_back","current_mark":current,"expected_at_least":floor}),
                )
            };
        }
        match journaled {
            Some(ledger::PostSpanVerdict::Bound(bindings)) => {
                return PostSpanDecision {
                    bindings: Some(bindings),
                    ..PostSpanDecision::reported(json!({"state":"bound"}))
                }
            }
            Some(ledger::PostSpanVerdict::Refused(code)) => {
                return PostSpanDecision::reported(json!({"state":"refused","code":code}))
            }
            None => {}
        }
        let Some(outcome) = outcome else {
            return PostSpanDecision::reported(
                json!({"state":"not_bound","code":"binding_response_not_recorded"}),
            );
        };
        let count = line.vouchers.len();
        let span = match after_post_mark {
            Some(after) => span_identity::PostSpan::after_clean_post(
                pre_post_voucher_mark,
                after,
                outcome,
                count,
            ),
            None => {
                span_identity::PostSpan::after_clean_response(pre_post_voucher_mark, outcome, count)
                    .map_err(span_identity::BindError::Refused)
            }
        };
        let bound = span.and_then(|span| {
            let window = span.alter_id_span();
            let rows = ImportReadSource {
                rows: observed
                    .rows
                    .iter()
                    .filter(|row| {
                        row.alter_id
                            .is_some_and(|id| id > window.after && id <= window.through)
                    })
                    .cloned()
                    .collect(),
            };
            let elsewhere = self.guids_bound_elsewhere(&line.batch_id).map_err(|_| {
                span_identity::BindError::Unsettled(span_identity::BindUnsettled::JournalUnreadable)
            })?;
            span_identity::bind(&span, &line.company_guid, &line.vouchers, &rows, &elsewhere)
        });
        let verdict = match bound {
            Ok(bindings) => ledger::PostSpanVerdict::Bound(bindings),
            Err(span_identity::BindError::Refused(refusal)) => {
                ledger::PostSpanVerdict::Refused(refusal.code().to_string())
            }
            Err(span_identity::BindError::Unsettled(unsettled)) => {
                return PostSpanDecision::reported(
                    json!({"state":"unsettled","code":unsettled.code()}),
                )
            }
        };
        match self.record_post_span_verdict(line, &verdict, generation) {
            Ok((ledger::PostSpanVerdict::Bound(bindings), recorded_at)) => PostSpanDecision {
                bindings: Some(bindings),
                rolled_back: false,
                report: json!({"state":"bound"}),
                recorded_at: Some(recorded_at),
            },
            Ok((ledger::PostSpanVerdict::Refused(code), recorded_at)) => PostSpanDecision {
                recorded_at: Some(recorded_at),
                ..PostSpanDecision::reported(json!({"state":"refused","code":code}))
            },
            Err(_) => PostSpanDecision::reported(
                json!({"state":"unsettled","code":"binding_not_recorded"}),
            ),
        }
    }

    /// Journals a post-span verdict under the exclusive admission lock, and
    /// returns it with the journal generation it left. Refused if the batch's
    /// journal moved since this verification read it (`expected`): something
    /// else wrote, and this verification must not decide on a stale view. A
    /// verdict already journaled is never replaced.
    fn record_post_span_verdict(
        &self,
        line: &ImportLedgerLine,
        verdict: &ledger::PostSpanVerdict,
        expected: ledger::VerificationGeneration,
    ) -> Result<(ledger::PostSpanVerdict, ledger::VerificationGeneration), String> {
        let _lock = self.lock_import_admission()?;
        let current = self
            .import_snapshot_while_admitted(Some(&line.batch_id))?
            .ok_or_else(|| "import_batch_not_found".to_string())?;
        if current.batch.sha256 != line.sha256 || current.generation != expected {
            return Err("import_verification_conflict_retry".into());
        }
        if current.span_verdict.is_some() {
            return Err("import_verification_conflict_retry".into());
        }
        // Another batch may have bound one of these GUIDs since the check
        // before this lock: checked again here, where no other writer can.
        let verdict = match verdict {
            ledger::PostSpanVerdict::Bound(bound) => {
                let elsewhere = match self.import_journal_while_admitted()? {
                    Some(reader) => ledger::guids_bound_elsewhere(reader, &line.batch_id)?,
                    None => BTreeSet::new(),
                };
                match bound
                    .iter()
                    .position(|identity| elsewhere.contains(&identity.guid))
                {
                    Some(position) => ledger::PostSpanVerdict::Refused(
                        span_identity::SpanRefusal::IdentityReused { position }
                            .code()
                            .to_string(),
                    ),
                    None => verdict.clone(),
                }
            }
            ledger::PostSpanVerdict::Refused(_) => verdict.clone(),
        };
        let verdict = &verdict;
        self.append_import_record_while_admitted(&ledger::StatusRecord::post_span_verdict(
            line, verdict,
        ))?;
        let recorded = self
            .import_snapshot_while_admitted(Some(&line.batch_id))?
            .ok_or_else(|| "import_batch_not_found".to_string())?;
        if recorded.span_verdict.as_ref() != Some(verdict) {
            return Err("binding_not_recorded".into());
        }
        Ok((verdict.clone(), recorded.generation))
    }

    /// Every GUID another batch's post-span verdict bound.
    fn guids_bound_elsewhere(&self, own_batch_id: &str) -> Result<BTreeSet<String>, String> {
        let _lock = self.lock_import_admission_shared()?;
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::guids_bound_elsewhere(reader, own_batch_id),
            None => Ok(BTreeSet::new()),
        }
    }

    /// The company's voucher mark now, for telling a book that kept a post from
    /// one rolled back below it. A company holding no voucher omits the mark,
    /// which reads as 0 here (a copy put in place of the book, or one emptied):
    /// unlike before a first import, that is an answer, not a refusal. Any
    /// other failure to read is a failed verification, never a guess.
    async fn current_voucher_mark(
        &self,
        company: &bridge_tally_protocol::TallyCompany,
        identity: &super::VerifiedCompanyIdentity,
    ) -> Result<(u64, Evidence), ToolFailure> {
        let guid = company
            .guid
            .as_deref()
            .ok_or_else(|| "pre_import_mark_unobserved".to_string())?;
        let (xml, evidence) = self
            .post_read(identity, company_high_water_read(&company.name))
            .await?;
        let mark = super::change_parse::company_voucher_high_water(&xml, guid).map_err(|code| {
            ToolFailure::from(pre_import_mark_refusal(&code).to_string())
                .with_prior_evidence(evidence.clone())
        })?;
        Ok((mark, evidence))
    }

    async fn pre_import_mark(
        &self,
        company: &bridge_tally_protocol::TallyCompany,
        identity: &super::VerifiedCompanyIdentity,
    ) -> Result<(PreImportMark, Evidence), ToolFailure> {
        let guid = company
            .guid
            .as_deref()
            .ok_or_else(|| "pre_import_mark_unobserved".to_string())?;
        let (xml, evidence) = self
            .post_read(identity, company_high_water_read(&company.name))
            .await?;
        let high_water = parse_company_high_water(&xml, guid).map_err(|code| {
            ToolFailure::from(pre_import_mark_refusal(&code).to_string())
                .with_prior_evidence(evidence.clone())
        })?;
        let mark = company_high_water_mark(&high_water)
            .map_err(|code| ToolFailure::from(code).with_prior_evidence(evidence.clone()))?;
        Ok((mark, evidence))
    }

    /// The proof the last verification of `batch_id` persisted, read whole.
    /// The batch must be one the import journal records, and the file is
    /// named exactly as `publish_proofs` names it, from the recorded id, so
    /// a caller's argument never becomes a path on its own.
    fn read_persisted_proof(&self, batch_id: &str) -> Result<Vec<u8>, String> {
        const MAX_PERSISTED_PROOF_BYTES: usize = 32 * 1024 * 1024;
        let recorded = self
            .latest_import_snapshot(batch_id)?
            .ok_or_else(|| "import_batch_not_found".to_string())?
            .batch
            .batch_id;
        let path = self.imports_dir()?.join(format!("{recorded}.proof.json"));
        let file = super::local_file::open_local_file(&path, false)
            .map_err(|_| "verification_proof_missing".to_string())?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(
            &mut std::io::Read::take(file, (MAX_PERSISTED_PROOF_BYTES + 1) as u64),
            &mut bytes,
        )
        .map_err(|_| "verification_proof_missing".to_string())?;
        if bytes.len() > MAX_PERSISTED_PROOF_BYTES {
            return Err("verification_proof_too_large".into());
        }
        Ok(bytes)
    }

    /// The XML file Bridge persisted when it built `batch_id`, read whole. The
    /// desktop review and the post path both compare it byte for byte with
    /// what they accept or send, so a journal record that agrees only with
    /// itself cannot stand in for the batch Bridge built (bridge#575).
    pub(super) fn read_persisted_import_xml(&self, batch_id: &str) -> Result<Vec<u8>, String> {
        const MAX_PERSISTED_IMPORT_XML_BYTES: usize = 5_000_000;
        let uuid = batch_id
            .strip_prefix("bridge-")
            .and_then(|value| uuid::Uuid::parse_str(value).ok())
            .ok_or_else(|| "import_batch_identifier_invalid".to_string())?;
        let path = self.imports_dir()?.join(format!("bridge-{uuid}.xml"));
        let mut file = super::local_file::open_local_file(&path, false)
            .map_err(|_| "import_persisted_file_unavailable".to_string())?;
        let length = file
            .metadata()
            .map_err(|_| "import_persisted_file_unavailable".to_string())?
            .len();
        if length > MAX_PERSISTED_IMPORT_XML_BYTES as u64 {
            return Err("import_persisted_file_too_large".into());
        }
        let mut bytes = Vec::with_capacity(length as usize);
        std::io::Read::read_to_end(
            &mut std::io::Read::take(&mut file, (MAX_PERSISTED_IMPORT_XML_BYTES + 1) as u64),
            &mut bytes,
        )
        .map_err(|_| "import_persisted_file_unavailable".to_string())?;
        if bytes.len() > MAX_PERSISTED_IMPORT_XML_BYTES {
            return Err("import_persisted_file_too_large".into());
        }
        Ok(bytes)
    }

    fn imports_dir(&self) -> Result<PathBuf, String> {
        let path = self.settings.data_dir.join("imports");
        super::ensure_private_directory(&path).map_err(|error| match error {
            super::DirectoryAdmissionError::Unavailable => "imports_dir_unavailable".to_string(),
            #[cfg(unix)]
            super::DirectoryAdmissionError::Permissions => {
                "import_file_permissions_failed".to_string()
            }
        })?;
        Ok(path)
    }

    pub(super) fn lock_import_admission(&self) -> Result<std::fs::File, String> {
        let path = self.settings.data_dir.join("agent-import-admission.lock");
        let file = super::local_file::open_local_file(&path, true)
            .map_err(|_| "import_admission_lock_unavailable".to_string())?;
        // Never park the async request loop behind another process's network
        // work. Contention is an in-band refusal, not a deferred posting request.
        file.try_lock().map_err(import_admission_lock_error)?;
        persistence::require_settled(&self.settings.data_dir.join("imports"))?;
        Ok(file)
    }

    fn lock_import_admission_shared(&self) -> Result<std::fs::File, String> {
        let path = self.settings.data_dir.join("agent-import-admission.lock");
        let file = super::local_file::open_local_file(&path, true)
            .map_err(|_| "import_admission_lock_unavailable".to_string())?;
        file.try_lock_shared()
            .map_err(import_admission_lock_error)?;
        persistence::require_settled(&self.settings.data_dir.join("imports"))?;
        Ok(file)
    }

    #[cfg(test)]
    fn import_ledger(&self) -> Result<Vec<ImportLedgerLine>, String> {
        let _admission_lock = self.lock_import_admission_shared()?;
        let history = match self.import_journal_while_admitted()? {
            Some(reader) => ledger::read_history(reader)?,
            None => Vec::new(),
        };
        Ok(history.into_iter().map(|snapshot| snapshot.batch).collect())
    }

    fn import_journal_while_admitted(
        &self,
    ) -> Result<Option<std::io::BufReader<std::fs::File>>, String> {
        let path = self.settings.data_dir.join("agent-import-ledger.jsonl");
        let file = match super::local_file::open_local_file(&path, false) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err("import_ledger_unavailable".to_string()),
        };
        Ok(Some(std::io::BufReader::new(file)))
    }

    fn import_snapshot_while_admitted(
        &self,
        batch_id: Option<&str>,
    ) -> Result<Option<ledger::BatchSnapshot>, String> {
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::read_snapshot(reader, batch_id),
            None => Ok(None),
        }
    }

    /// Whether the journal already records any of `remote_ids` on a dispatch
    /// intent.
    pub(super) fn import_remote_ids_recorded_while_admitted(
        &self,
        remote_ids: &[Uuid],
    ) -> Result<bool, String> {
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::remote_ids_recorded(reader, remote_ids),
            None => Ok(false),
        }
    }

    /// The batch, already sent or found posted, that holds a row of `line`.
    pub(super) fn import_rows_already_posted_while_admitted(
        &self,
        line: &ImportLedgerLine,
    ) -> Result<Option<String>, String> {
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::rows_already_posted(reader, line),
            None => Ok(None),
        }
    }

    /// How an invoice batch's vouchers are recognised in the book: by number,
    /// and by figures as well while this machine holds an unsettled batch with
    /// them. A batch with no invoice takes the journal's lock and reads it not
    /// at all, so Payment, Receipt, Contra and Journal posts and their
    /// readbacks behave as they did before the number joined the identity.
    fn import_invoice_identity(&self, line: &ImportLedgerLine) -> Result<InvoiceIdentity, String> {
        if !holds_an_invoice(line) {
            return Ok(InvoiceIdentity::ByNumber);
        }
        let _lock = self.lock_import_admission_shared()?;
        Ok(InvoiceIdentity::beside(
            self.import_unsettled_invoice_twin_while_admitted(line)?
                .as_deref(),
        ))
    }

    /// A batch's verdict over the rows read, with the invoice identity the
    /// journal calls for (`import_invoice_identity`).
    fn verify_batch_by_journal_identity(
        &self,
        line: &ImportLedgerLine,
        observed: &ImportReadSource,
        attribution: Attribution<'_>,
    ) -> Result<Value, String> {
        let identity = self.import_invoice_identity(line)?;
        verify_batch_as(line, observed, attribution, identity)
    }

    /// What a refusal for rows already in the book says of the journal: `None`
    /// for a batch with no invoice (which neither locks nor reads it), else the
    /// unsettled twin, or why it could not be read.
    pub(super) fn import_unsettled_invoice_twin_for_refusal(
        &self,
        line: &ImportLedgerLine,
    ) -> Option<Result<Option<String>, String>> {
        holds_an_invoice(line).then(|| {
            self.lock_import_admission_shared()
                .and_then(|_lock| self.import_unsettled_invoice_twin_while_admitted(line))
        })
    }

    /// What the journal offers as the control for an invoice number read of
    /// this company (`ledger::invoice_number_control`), read under the shared
    /// admission lock and released at once.
    fn import_invoice_number_control(
        &self,
        company_guid: &str,
        year: (&str, &str),
    ) -> Result<ledger::NumberControl, String> {
        let _lock = self.lock_import_admission_shared()?;
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::invoice_number_control(reader, company_guid, year),
            None => Ok(ledger::NumberControl::NeverSent),
        }
    }

    /// The batch of this company that stops every further Sales post
    /// (`ledger::invoice_stop`), read under the shared admission lock and
    /// released at once.
    pub(super) fn import_invoice_stop(&self, company_guid: &str) -> Result<Option<String>, String> {
        let _lock = self.lock_import_admission_shared()?;
        self.import_invoice_stop_while_admitted(company_guid)
    }

    /// Whether `line` is an invoice that a stop of its company holds back. A
    /// batch with no invoice is never judged by it: a stopped company stops no
    /// Journal, Payment, Receipt or Contra.
    pub(super) fn stopped_company_while_admitted(
        &self,
        line: &ImportLedgerLine,
    ) -> Result<bool, String> {
        if !holds_an_invoice(line) {
            return Ok(false);
        }
        Ok(self
            .import_invoice_stop_while_admitted(&line.company_guid)?
            .is_some())
    }

    /// Every sent, unverified invoice batch of this company that stops it or
    /// stands as its number control after a release (`ledger::invoice_holds`).
    pub(super) fn import_invoice_holds_while_admitted(
        &self,
        company_guid: &str,
    ) -> Result<BTreeMap<String, ledger::InvoiceHold>, String> {
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::invoice_holds(reader, company_guid),
            None => Ok(BTreeMap::new()),
        }
    }

    /// Every batch that stops this company, in id order.
    pub(super) fn import_invoice_stops_while_admitted(
        &self,
        company_guid: &str,
    ) -> Result<Vec<String>, String> {
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::invoice_stops(reader, company_guid),
            None => Ok(Vec::new()),
        }
    }

    /// [`Self::import_invoice_stop`] for a caller that already holds an
    /// admission lock: the build before it writes a file, and a post before it
    /// records its dispatch intent.
    pub(super) fn import_invoice_stop_while_admitted(
        &self,
        company_guid: &str,
    ) -> Result<Option<String>, String> {
        Ok(self
            .import_invoice_stops_while_admitted(company_guid)?
            .into_iter()
            .next())
    }

    /// The batch, sent and whose latest status is not a verified post, that
    /// holds an invoice with the figures of `line`'s invoice
    /// (`ledger::unsettled_invoice_twin`).
    pub(super) fn import_unsettled_invoice_twin_while_admitted(
        &self,
        line: &ImportLedgerLine,
    ) -> Result<Option<String>, String> {
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::unsettled_invoice_twin(reader, line),
            None => Ok(None),
        }
    }

    /// The batch, already sent or found posted, that holds a row of a build's
    /// `vouchers`.
    fn import_vouchers_already_posted_while_admitted(
        &self,
        company_guid: &str,
        vouchers: &[ImportVoucher],
    ) -> Result<Option<String>, String> {
        match self.import_journal_while_admitted()? {
            Some(reader) => ledger::vouchers_already_posted(reader, company_guid, None, vouchers),
            None => Ok(None),
        }
    }

    fn latest_import_snapshot(
        &self,
        batch_id: &str,
    ) -> Result<Option<ledger::BatchSnapshot>, String> {
        let _admission_lock = self.lock_import_admission_shared()?;
        self.import_snapshot_while_admitted(Some(batch_id))
    }

    #[cfg(test)]
    pub(super) fn append_import_ledger(&self, line: &ImportLedgerLine) -> Result<(), String> {
        let _admission_lock = self.lock_import_admission()?;
        self.append_import_ledger_while_admitted(line)
    }

    fn append_import_ledger_while_admitted(&self, line: &ImportLedgerLine) -> Result<(), String> {
        self.append_import_record_while_admitted(line)
    }

    pub(super) fn append_import_record_while_admitted(
        &self,
        line: &impl Serialize,
    ) -> Result<(), String> {
        let path = self.settings.data_dir.join("agent-import-ledger.jsonl");
        let encoded = serde_json::to_string(line)
            .map_err(|_| "import_ledger_serialization_failed".to_string())?;
        let encoded = format!("{encoded}\n");
        if encoded.len() > ledger::MAX_RECORD_BYTES {
            return Err("import_ledger_record_too_large".into());
        }
        append_private_import_ledger(&path, encoded.as_bytes(), set_private_file)
    }
}

impl Server {
    /// Resolve and admit the lineage a payload amends, under an admission lock
    /// the caller holds, and check the proposal against it.
    fn amendment_lineage_while_admitted(
        &self,
        payload: &ImportPayload,
    ) -> Result<amend::Lineage, String> {
        let named_id = payload
            .amends_batch_id
            .as_deref()
            .filter(|id| valid_batch_id(id))
            .ok_or_else(|| "import_amend_batch_id_invalid".to_string())?;
        let named = self
            .import_snapshot_while_admitted(Some(named_id))?
            .ok_or_else(|| "import_amend_batch_not_found".to_string())?;
        let builds = match self.import_journal_while_admitted()? {
            Some(reader) => ledger::read_lineage(reader, named.batch.identity_batch_id())?,
            None => Vec::new(),
        };
        let origin = super::canonical_loopback_origin(&self.settings.endpoint)
            .map_err(|_| "host_setting_invalid".to_string())?;
        let lineage = amend::admit_lineage(&named, builds, &payload.company_guid, &origin)?;
        lineage.admit_proposal(&payload.vouchers)?;
        Ok(lineage)
    }
}

const AMENDMENT_WARNING: &str = "This file amends an earlier batch. Each voucher carries that batch's REMOTEID, so importing it alters the vouchers already in the book in place instead of creating new ones: Tally should report them as altered, not created. ComplyEaze Bridge compared those vouchers with what it built only as the book stood during this build, and only these fields: the date, a bank voucher's effective date when Tally returned one, the voucher type, the voucher number when the batch set one, each entry's ledger, amount and side, and the narration. It did not compare a voucher's reference, its bill-wise or cost-centre allocations, or which ledger Tally records as its party, because the verification read does not fetch them; instead it refused any voucher whose ALTERID has moved since ComplyEaze Bridge first verified it, which catches an edit to those fields made after that verification, provided a Tally edit advances the voucher's ALTERID (measured over the gateway; not yet for an edit made in Tally's own screens). An edit made before that first verification is not caught, so verify right after every import. An in-place alteration replaces a voucher's entries rather than merging them (measured over the gateway), and this file's entries carry no allocations, so allocations made in Tally, including those ComplyEaze Bridge advises adding after an import, are expected to be lost; that loss, and what happens to a reference, were not measured directly. An edit made in Tally between this build and the import is overwritten without warning. Import promptly, and build the amendment again if anyone may have changed these vouchers. Import and verify each amendment before building the next one for the same voucher: two amendments built from the same state overwrite each other, and the later import wins. In-place alteration with changed content was measured over the XML gateway on licensed TallyPrime 7.1 Silver for Journal, Payment, Receipt and Contra; an import through Tally's own Import menu was not measured.";

/// Whether a batch holds an invoice: the one gate for every read of the journal's
/// unsettled batches, so a bank batch never takes its lock for the number rule.
fn holds_an_invoice(line: &ImportLedgerLine) -> bool {
    line.vouchers
        .iter()
        .any(|voucher| voucher.voucher_type.is_invoice())
}

/// Why no file was written for a row another batch already sent (#876), and what
/// to do instead. It never offers a hand import of this row: that is the second
/// post the refusal exists to stop.
const BUILD_TXN_ALREADY_POSTED_NEXT_STEP: &str = "No file was written and nothing was sent. Another batch of this company already went to Tally with a row of this one, or was found posted; blocking_batch_id names it. Call verify_import with that batch. If it finds the voucher, a row with a statement id (st-, from a bank-statement build) is the same bank row whatever ledger it names: build again without that row (parse the statement again with a narrower from and to; those are whole days, so a day that holds a posted row and an unposted one is left out whole, and the user enters its unposted rows in Tally), and never import a file that carries it. A hand-typed id can repeat: this row matched because the id, date and amounts are equal (or an amount could not be read), and that is either the same transaction, already in the book, or a different real transaction that shares them. Do not decide which yourself: ask the user to open the existing voucher in Tally, compare it with this row, and say which. If it is the same transaction and its ledger or narration is wrong, correct the posted voucher in Tally (or, for a batch that was imported by hand, amend it as described below); if it is a second real transaction that is not in the book, build that voucher under a new bridge_txn_id. ComplyEaze Bridge does not check the user's answer, and for a posted_verified voucher verify_import returns no date, amounts, ledgers or narration. Never rename a statement row this way: a statement row entered under any other id is not seen. To correct a voucher of a batch that was imported by hand, build with amends_batch_id set to that batch. For an invoice, its number identifies it, not its bridge_txn_id. If the blocking batch was released (acknowledge_post_review, doubt invoice_stop) after a read that did not find its invoice, build the invoice again under a new bridge_txn_id and the SAME invoice number: ComplyEaze Bridge refuses the build while Tally's book holds that number in the financial year, and refuses the post, while an earlier batch with the same figures is still unverified, if Tally holds a voucher with those figures under any number. Never also tell the user to enter that invoice in Tally by hand: a retry after a hand entry is how a sale is booked twice. For any other voucher: if Tally rejected that batch and the voucher is not in Tally, ComplyEaze Bridge cannot write this row again: ask the user to enter the voucher in Tally.";

/// The code of the refusal that stops an invoice build while voucher posting
/// is off (see `build_import_xml`); its text is in agent.rs.
const INVOICE_POST_NOT_ENABLED: &str = "invoice_post_not_enabled";

/// The code an invoice build gives when `post_import` would refuse the saved
/// invoice: the post's own code, except for the refusals of the approval text,
/// which also refuse the other voucher types and so carry a code of their own
/// here, with advice about an invoice.
fn invoice_route_refusal(code: String) -> String {
    match code.as_str() {
        "import_review_layout_text" => "invoice_review_layout_text".to_string(),
        "import_review_format_text" => "invoice_review_format_text".to_string(),
        "import_review_too_large" => "invoice_review_too_large".to_string(),
        _ => code,
    }
}

const INVOICE_POST_ONLY_WARNING: &str = "No import XML was sent to Tally. This invoice is posted by post_import, which needs a separate native approval, and in no other way. Do not import the written file in Tally by hand: a hand import skips the stop on an unverified invoice, the duplicate checks, the reads on either side of the approval and the readback of the invoice.";

const INVOICE_POST_NEXT_STEP: &str = "Call post_import with this company_guid and batch_id; the local user must review and approve it before one posting attempt.";

/// The guidance of a saved invoice batch: the posting route only, never a hand
/// import. Replaces the first warning and the next step the batch kinds share.
fn invoice_guidance(warnings: &mut Value) -> &'static str {
    if let Some(first) = warnings.as_array_mut().and_then(|list| list.first_mut()) {
        *first = json!(INVOICE_POST_ONLY_WARNING);
    }
    INVOICE_POST_NEXT_STEP
}

const AMENDMENT_NOT_POSTABLE: &str = "No import XML was sent to Tally. ComplyEaze Bridge does not post amendments (post_import refuses them), so import the written file by hand, promptly, then use verify_import; do not call post_import for this batch.";

const AMENDMENT_NEXT_STEP: &str = "Import promptly: an edit made in Tally before the import is overwritten, so build the amendment again first if anyone may have changed these vouchers, and re-enter any allocation afterwards. Verify right after importing: that first verification is what a later amendment compares against. Confirm the loaded company matches this batch, import the file in Tally (Gateway of Tally → Import → Vouchers) and check that it reports altered vouchers and none created, then call verify_import with this batch_id. If any voucher was created, do not import again: call verify_import and reconcile the duplicate by hand. Import and verify each amendment before building the next one for the same voucher: two amendments built from the same state overwrite each other, and the later import wins.";

const AMENDMENT_REFUSED_NEXT_STEP: &str = "No file was written. An amendment alters vouchers in place, so it is admitted only while each one is still in the book as a build of this batch wrote it, in the fields ComplyEaze Bridge compares (date, a bank voucher's effective date when Tally returns one, type, number when set, entries' ledger, amount and side, narration). not_in_book means no voucher in the window carries this batch's marker: it was never imported, was deleted, or had its narration edited, so reconcile with verify_import instead. book_voucher_diverged means the voucher changed after ComplyEaze Bridge built it, and an amendment would overwrite that change, so a person must decide what the voucher should hold. voucher_cancelled_or_optional is refused because importing over such a voucher was not measured. voucher_altered_since_verified means the voucher's ALTERID is not the one ComplyEaze Bridge recorded when it first verified a build the book matches, or was not read: Tally has altered the voucher since, which can be an edit to a field ComplyEaze Bridge does not compare, such as a reference or an allocation. Correct the voucher in Tally directly; a fresh batch would duplicate it unless the existing voucher is first cancelled or deleted in Tally. voucher_never_verified means no build the book matches has a verification ComplyEaze Bridge recorded for this voucher: most often the last import was never verified, or it was verified before ComplyEaze Bridge kept these records. Verifying now records this voucher exactly as it stands in Tally, including any changes made since ComplyEaze Bridge built it. Check the voucher in Tally first; if someone has edited it, correct it there instead of amending. If it is unchanged, verify the batch named in book_matches_batch_ids and build the amendment again.";

/// A native-dispatched batch is tied to the Tally endpoint used for its saved
/// admission. Older manual imports retain their original verification path.
fn validate_dispatched_import_endpoint(
    line: &ImportLedgerLine,
    dispatched: bool,
    endpoint: &super::TallyEndpointConfig,
) -> Result<(), String> {
    if !dispatched {
        return Ok(());
    }
    let origin = super::canonical_loopback_origin(endpoint)
        .map_err(|_| "host_setting_invalid".to_string())?;
    if line.endpoint_origin.as_deref() != Some(origin.as_str()) {
        return Err("import_post_endpoint_mismatch".into());
    }
    Ok(())
}

fn append_private_import_ledger(
    path: &Path,
    bytes: &[u8],
    prepare_permissions: impl FnOnce(&std::fs::File) -> Result<(), String>,
) -> Result<(), String> {
    // Admission serializes writers; read/write access also permits Windows rollback.
    let mut file = super::local_file::open_local_file(path, true)
        .map_err(|_| "import_ledger_unavailable".to_string())?;
    prepare_permissions(&file)?;
    file.seek(SeekFrom::End(0))
        .map_err(|_| "import_ledger_unavailable".to_string())?;
    append_import_ledger_bytes(&mut file, bytes)
}

trait ImportLedgerWriter {
    fn length(&mut self) -> std::io::Result<u64>;
    fn append(&mut self, bytes: &[u8]) -> std::io::Result<()>;
    fn sync(&mut self) -> std::io::Result<()>;
    fn truncate(&mut self, length: u64) -> std::io::Result<()>;
}

impl ImportLedgerWriter for std::fs::File {
    fn length(&mut self) -> std::io::Result<u64> {
        self.metadata().map(|metadata| metadata.len())
    }

    fn append(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.write_all(bytes)
    }

    fn sync(&mut self) -> std::io::Result<()> {
        self.sync_data()
    }

    fn truncate(&mut self, length: u64) -> std::io::Result<()> {
        self.set_len(length)
    }
}

fn append_import_ledger_bytes(
    writer: &mut impl ImportLedgerWriter,
    bytes: &[u8],
) -> Result<(), String> {
    let original_length = writer
        .length()
        .map_err(|_| "import_ledger_unavailable".to_string())?;
    if writer.append(bytes).and_then(|_| writer.sync()).is_err() {
        writer
            .truncate(original_length)
            .and_then(|_| writer.sync())
            .map_err(|_| "import_ledger_rollback_failed".to_string())?;
        return Err("import_ledger_unavailable".to_string());
    }
    Ok(())
}

fn canonical_batch_guid(guid: &str) -> String {
    guid.to_ascii_lowercase()
}

fn batch_guid_matches(stored: &str, supplied: &str) -> bool {
    stored.eq_ignore_ascii_case(supplied)
}

fn parse_payload(args: &Value) -> Result<ImportPayload, String> {
    serde_json::from_value(args.clone()).map_err(|_| "voucher_schema_invalid".to_string())
}

fn import_company_tuple(
    company: &bridge_tally_protocol::TallyCompany,
) -> Result<ImportCompanyTuple, String> {
    Ok(ImportCompanyTuple {
        name: nonempty_company_field(&company.name)?,
        guid: nonempty_company_field(
            company
                .guid
                .as_deref()
                .ok_or_else(|| "company_identity_incomplete".to_string())?,
        )?,
        company_number: nonempty_company_field(
            company
                .company_number
                .as_deref()
                .ok_or_else(|| "company_identity_incomplete".to_string())?,
        )?,
        books_from: normalized_date(
            company
                .books_from
                .as_deref()
                .ok_or_else(|| "company_identity_incomplete".to_string())?,
        )?
        .as_str()
        .to_string(),
    })
}

fn nonempty_company_field(value: &str) -> Result<String, String> {
    (!value.trim().is_empty())
        .then(|| value.to_string())
        .ok_or_else(|| "company_identity_incomplete".to_string())
}

/// A catalogue answer that cannot be used: `code` keeps naming what failed; the
/// cause says why, which every catalogue refusal used to leave out (bridge#634).
fn catalogue_failure(
    error: bridge_tally_protocol::StandardLedgerCatalogError,
    evidence: &Evidence,
) -> ToolFailure {
    let mut failure = ToolFailure::from("ledger_export_invalid".to_string())
        .with_prior_evidence(evidence.clone());
    failure.cause = Some(error.safe_code());
    failure
}

fn approval_invalid(error: bill_wise::ApprovalError) -> ToolFailure {
    let mut failure = ToolFailure::from("on_account_approval_invalid".to_string());
    failure.cause = Some(error.cause());
    failure
}

const BILL_WISE_UNAPPROVED_NEXT_STEP: &str = "No file was written. Each party listed is a ledger that keeps bills in Tally. An entry on it with no bill allocation lands On Account, and the person must then match it to a bill in Tally by hand. Show the person each party with its row_count, its debit_total and credit_total, and the rows listed, and say how many more rows there are (rows_omitted, refused_parties_omitted); raise BRIDGE_AGENT_MAX_BYTES to list them all. Ask whether each party's entries may be posted On Account, one party per question. Only for the parties the person says yes to, build again with on_account_approvals: a list with one {party_digest} for each, the digest copied from this answer (a ledger copied beside it is not read, so a masked name does no harm). The digest ties the approval to this exact batch, this company and this endpoint, and changing any row changes every party's digest, so the person is asked again. It does not prove that a person said yes, and a hand import of the file is not checked at all: never approve on the person's behalf. When the batch is posted, the native approval dialog marks each approved party On Account, from this build's record: before its name on each of its entries for one voucher, and on its totals line for a batch (not on a batch's per-voucher lines). That dialog is one answer for the whole batch and asks nothing about any one party, so it does not replace these questions. If this batch amends an earlier one, importing it also replaces any bill allocations the person made in Tally.";

const INVOICE_NEW_REF_NOTE: &str = "The customer on this invoice is a bill-wise ledger, as read from Tally's ledger list during this build: its entry is written with a New Ref named by the invoice number, so it does not land On Account, and verify_import reads the allocation back. Every other ledger this batch names is not bill-wise. ComplyEaze Bridge reads the invoice's masters again before the approval dialog and again before the approval is spent, and refuses the post if the invoice would no longer be admitted or if what it recorded of the customer, the voucher type or the state of the company's GST registration has changed. A hand import of the file is not checked at all.";
const BILL_WISE_NONE_NOTE: &str = "Checked: none of the ledgers this batch names is a bill-wise ledger, as read from Tally in the ledger list during this build. ComplyEaze Bridge reads the ledger list again before posting and refuses the post (import_bill_wise_changed) if a named ledger has become bill-wise since. This reads each ledger's own bill-wise setting, not the company's bill-wise feature. A hand import of the file is not checked at all.";

const BILL_WISE_APPROVED_NOTE: &str = "Entries on the bill-wise ledgers listed in on_account_approved carry no bill allocation, so each amount lands On Account and must be matched to bills in Tally afterwards. Each has an approval digest that matches this batch; ComplyEaze Bridge cannot tell whether a person said yes. If this batch is posted natively, the approval dialog marks each of these ledgers On Account, from this build's record: before its name on each of its entries for one voucher, and on its totals line for a batch (not on a batch's per-voucher lines); a hand import of the file shows no dialog. Any other ledger this batch names that has become bill-wise by the time of posting is refused (import_bill_wise_changed).";

/// Why `post_import` would refuse a saved batch, for the build's warning. The
/// code is the one `post_import` returns; the text only explains it. A code
/// with no entry here keeps its name and a pointer to the same refusal.
fn native_post_refusal_reason(code: &str, voucher_limit: usize) -> String {
    let count = if voucher_limit > 1 {
        format!("1 to {voucher_limit} vouchers")
    } else {
        "one voucher (the limit while batch posting is off)".to_string()
    };
    match code {
        "import_post_requires_one_voucher" => format!(
            "native posting takes {count}, each a Journal, Payment, Receipt or Contra, from a batch built by this version of ComplyEaze Bridge"
        ),
        "import_post_batch_too_large" => format!(
            "this batch has more vouchers than native posting takes at once ({voucher_limit})"
        ),
        "import_post_numbered_journal_unsupported" => {
            "a voucher carries its own number, and native posting lets Tally assign it".to_string()
        }
        "import_review_layout_text" | "import_review_format_text" => {
            "the company name, a ledger name or a voucher's own text holds a line break or another character the approval dialog cannot show faithfully".to_string()
        }
        "import_review_too_large" => {
            "the approval text does not fit in one native dialog".to_string()
        }
        "import_review_on_account_unmarked" => {
            "the approval text would leave an approved bill-wise ledger without its On Account mark".to_string()
        }
        _ => "post_import refuses this batch for the same reason".to_string(),
    }
}

fn build_import_guidance(
    writes_enabled: bool,
    native_post_refusal: Option<&str>,
    voucher_limit: usize,
    bank_types: bool,
    on_account_approved: bool,
    multi_entry_bank: bool,
    new_ref_invoice: bool,
) -> (Value, &'static str) {
    let preflight_warning =
        "The preflight observes the current verification window. The import or subsequent changes can make later readback exceed the source limits.";
    // §9.11d and §9.13: a mismatched SVCURRENTCOMPANY is verified to post into
    // whatever company Tally has loaded, CREATED=1 and no error. Deliberately
    // NOT gated on bank_types — the hazard is general to any import, and
    // warning only on the bank path would imply the Journal path is safe. The
    // wording stays neutral between a hand import and a native post because
    // this is emitted at build time, before which one happens is known.
    let company_identity_warning =
        "Confirm the loaded company before importing. A mismatched SVCURRENTCOMPANY is verified to post into whatever company Tally has loaded, with CREATED=1 and no error, so naming a company does not aim the write. Compare the whole identity immediately before importing — endpoint origin, name, GUID, company number and books-from, all five. Not the GUID alone, because a year-end split gives the child its parent's GUID; and not the company fields alone, because a second local Tally can hold a copy of the same company and a hand import is never checked against the endpoint this batch was built from. The endpoint_origin this batch recorded is returned beside it. Prefer an instance with no other company loaded.";
    // §9.13: every party amount in the observed import landed On Account.
    // The build now reads each named ledger's bill-wise flag and has a person
    // approve each bill-wise party (#1234), so what is stated is what was
    // checked and what that check cannot show.
    // The controlled repeat observed in §9.8 does not establish unknown-outcome
    // recovery. Every voucher type retains its original identity and uses
    // read-only reconciliation; Journal is not an exception.
    let repeat_warning =
        "Do not re-import or rebuild this business event if the outcome is uncertain, including a Journal. Preserve the original batch and saved file, then call verify_import for read-only reconciliation. A controlled repeat observation does not qualify unknown-outcome recovery.";
    // agent_import_cash_bank.rs's module header documents this gap: the build
    // proves master stability across the build only, and says nothing about
    // afterwards, so a regroup between build and hand import is invisible to
    // verify_import.
    // post_import closes the gap for its own path: it classifies every leg
    // again before approval and again after approval inside the endpoint
    // queue, before the final duplicate check and the post. A hand import of
    // the file has no such check, so the warning keeps saying so.
    let stale_classification_warning = bank_types.then_some(
        "This file's Payment, Receipt and Contra split came from the group collection read during this build. Regrouping a ledger afterwards is an ordinary Tally operation and would silently make the voucher type wrong — a counterparty moved under a cash or bank group should have become a Contra. post_import classifies every leg again before approval and after approval inside the endpoint queue, before the final duplicate check and the post, and refuses a changed one, but a manual import of this file is not checked, and verify_import compares the entries as built, not current ancestry. If any master changed since this batch was built, discard it and build again.",
    );
    // The qualified slice is §9.13's, measured on one licensed instance. This
    // repo's settled position — see `observe_import_profile`'s own comment —
    // is that a release or tier label is evidence recorded on this path, not
    // a categorical admission gate, so a build against a different release or
    // tier is reported here rather than refused or vouched for.
    let release_evidence_warning = bank_types.then_some(
        "The Payment, Receipt and Contra file shapes were measured on licensed TallyPrime 7.1 Gold only. This endpoint's observed product, release, tier and mode are returned beside this batch. Release and tier are recorded as evidence on this path rather than used as an admission gate, so a build against a different release is neither refused nor proven.",
    );
    let allocation_warning = if on_account_approved {
        BILL_WISE_APPROVED_NOTE
    } else if new_ref_invoice {
        INVOICE_NEW_REF_NOTE
    } else {
        BILL_WISE_NONE_NOTE
    };
    // bridge#466: the shape rests on narrower live evidence than two entries.
    // Comments and docs are not what an operator reads, so the result says so
    // itself, beside the party choice it made.
    let multi_entry_warning = multi_entry_bank.then_some(
        "A Payment, Receipt or Contra with more than two entries rests on narrower evidence than a two-entry one (bridge#466). Hand-built files of this shape were imported and read back with every entry over the gateway on licensed TallyPrime 7.1 Silver (for a Contra, only with a ledger repeated; three distinct ledgers not observed), and one three-entry Receipt built by ComplyEaze Bridge was imported over the gateway and verified, but no multi-entry Payment or Contra has been, and none of the three, including that Receipt, through Tally's Import menu. Where such a voucher names several counterparties, the file names the first as the voucher's party; on two-entry Payments and Receipts and on that three-entry Receipt, Tally 7.1 Silver read back the bank ledger as the party rather than the counterparty written, and verify_import does not compare the party. verify_import still compares every entry.",
    );
    let warnings = |first: &str| {
        json!(std::iter::once(first)
            .chain(std::iter::once(preflight_warning))
            .chain(std::iter::once(company_identity_warning))
            .chain(std::iter::once(repeat_warning))
            .chain(stale_classification_warning)
            .chain(release_evidence_warning)
            .chain(std::iter::once(allocation_warning))
            .chain(multi_entry_warning)
            .collect::<Vec<_>>())
    };
    let manual_import_next_step = "Confirm the loaded company matches this batch, import the file in Tally (Gateway of Tally → Import → Vouchers), then call verify_import right away: its first verification records each voucher's state for any later amendment";
    if writes_enabled && native_post_refusal.is_none() {
        (
            warnings(
                "No import XML was sent to Tally. To post this saved batch, call post_import; it requires a separate native approval. If you import the file manually, call verify_import right after importing and do not call post_import for that batch.",
            ),
            "Call post_import with this company_guid and batch_id; the local user must review and approve it before one posting attempt.",
        )
    } else if let Some(code) = native_post_refusal {
        (
            warnings(&format!(
                "No import XML was sent to Tally. This saved batch is not eligible for native posting: post_import would refuse it as {code} ({}). Import the written file manually, then use verify_import; do not call post_import for this batch.",
                native_post_refusal_reason(code, voucher_limit),
            )),
            manual_import_next_step,
        )
    } else {
        (
            warnings(
                "No import XML was sent to Tally. Import the written file manually, then use verify_import.",
            ),
            manual_import_next_step,
        )
    }
}

/// The live observation each voucher type in this batch actually rests on.
///
/// A Journal file rests on the synthetic-lab import/readback recorded in the
/// 2026-09-06 assessment — a report that in the same breath records Payment,
/// Receipt and Contra being refused. Those three rest on the licensed 7.1
/// import recorded as reference §9.13 instead. Citing either for the other
/// would look auditable and be wrong. The two never share a file — mixing the
/// rendered shapes is refused — but the map stays general so a batch of several
/// bank types reports the one source they share, once.
fn live_evidence(vouchers: &[ImportVoucher]) -> Vec<Value> {
    let mut sources = BTreeMap::<(&str, &str), BTreeSet<&str>>::new();
    for voucher in vouchers {
        let source = match voucher.voucher_type.bank_shape() {
            None if voucher.voucher_type.is_invoice() => (
                "hand_keyed_reads_and_hand_imports_not_bridge_posted",
                "docs/tally/TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md",
            ),
            None => (
                "synthetic_lab_readback",
                "docs/agent/ASSESSMENT-2026-09-06.md",
            ),
            // §9.13 imported two-entry vouchers only. A bank voucher with more
            // entries (bridge#466) must not borrow that. It keeps the weaker
            // label even after one Bridge-built three-entry Receipt was imported
            // over the gateway and verified: that is one type, one sample, and
            // not Tally's Import menu. §9.3's correction table records hand-built
            // XML of this shape imported and read back over the gateway.
            Some(_) if voucher.entries.len() > 2 => (
                "hand_built_gateway_readback",
                "docs/tally/TALLY_PROTOCOL_REFERENCE_WRITE_RESPONSES_AND_MASTERS.md",
            ),
            Some(_) => (
                "licensed_bank_voucher_import",
                "docs/tally/TALLY_PROTOCOL_REFERENCE.md",
            ),
        };
        sources
            .entry(source)
            .or_default()
            .insert(voucher.voucher_type.as_str());
    }
    sources
        .into_iter()
        .map(|((observation, report), voucher_types)| {
            json!({"observation":observation, "report":report,
                "voucher_types":voucher_types.into_iter().collect::<Vec<_>>()})
        })
        .collect()
}

/// The first gate of a build, and a gate of every post before any request: a
/// voucher type absent from the qualified list never reaches a live request,
/// let alone a written file.
/// `qualified` is a parameter so the guard can be exercised against a
/// narrowed list as well as the real one.
fn refuse_unqualified_types(
    vouchers: &[ImportVoucher],
    qualified: &[VoucherType],
) -> Result<(), String> {
    vouchers
        .iter()
        .all(|voucher| qualified.contains(&voucher.voucher_type))
        .then_some(())
        .ok_or_else(|| "import_voucher_type_unqualified".to_string())
}

/// Whether this batch renders the bank shape at all.
///
/// After `validate_payload` a batch is homogeneous by shape, so this is both
/// "any" and "all" — the group read, the guidance and the evidence record can
/// each ask it once and get a whole-batch answer.
fn renders_bank_shape(vouchers: &[ImportVoucher]) -> bool {
    vouchers
        .iter()
        .any(|voucher| voucher.voucher_type.bank_shape().is_some())
}

/// A file may carry more than one voucher type — the measured statement files
/// mixed Payment and Receipt freely, 61+54 in one and 20+8 in another, both
/// imported clean. What it may not do is mix two rendered *shapes*.
///
/// The three bank types share one shape: `EFFECTIVEDATE` beside `DATE`, a party
/// on the counterparty side, never a number or reference. A Journal's shape
/// carries none of that and comes from a separate qualification lineage
/// (§9.8 against §9.13). No file mixing the two has been imported — the
/// reallocation Journals went in on their own — so the union is refused rather
/// than assumed from holding both citations at once.
fn refuse_mixed_shapes(vouchers: &[ImportVoucher]) -> Result<(), String> {
    // An invoice is its own shape and is built alone: its masters, its type's
    // numbering and its party are read for that one voucher.
    let invoices = vouchers
        .iter()
        .filter(|voucher| voucher.voucher_type.is_invoice())
        .count();
    if invoices > 0 && invoices != vouchers.len() {
        return Err("voucher_type_shapes_mixed".to_string());
    }
    if invoices > 1 {
        return Err("invoice_one_per_batch".to_string());
    }
    let bank = vouchers
        .iter()
        .filter(|voucher| voucher.voucher_type.bank_shape().is_some())
        .count();
    (bank == 0 || bank == vouchers.len())
        .then_some(())
        .ok_or_else(|| "voucher_type_shapes_mixed".to_string())
}

fn validate_payload(payload: &ImportPayload) -> Result<(), String> {
    refuse_mixed_shapes(&payload.vouchers)?;
    if payload.company_guid.trim().is_empty()
        || payload.vouchers.is_empty()
        || payload.vouchers.len() > MAX_VOUCHERS
    {
        return Err("voucher_count_invalid".to_string());
    }
    let mut txn_ids = BTreeSet::new();
    let mut ledger_names = BTreeSet::new();
    for voucher in &payload.vouchers {
        if !valid_txn_id(&voucher.bridge_txn_id) || !txn_ids.insert(&voucher.bridge_txn_id) {
            return Err("bridge_txn_id_invalid_or_duplicate".to_string());
        }
        normalized_date(&voucher.date)?;
        if voucher.entries.len() < 2 {
            return Err("voucher_entries_too_few".to_string());
        }
        for text in [voucher.narration.as_deref(), voucher.reference.as_deref()]
            .into_iter()
            .flatten()
        {
            if contains_reserved_marker(text) {
                return Err("narration_reserved_marker".to_string());
            }
        }
        for text in [voucher.narration.as_deref(), voucher.reference.as_deref()]
            .into_iter()
            .flatten()
        {
            // JSON Schema minLength/maxLength count Unicode code points, not UTF-8 bytes.
            if text.is_empty()
                || text.chars().count() > MAX_TEXT_CHARS
                || text.chars().any(char::is_control)
            {
                return Err("voucher_text_invalid".to_string());
            }
        }
        if let Some(number) = voucher.voucher_number.as_deref() {
            // Unlike narration/reference, `voucher_diffs` compares the
            // voucher number verbatim, so a value that would read back
            // rewritten must be refused here — verification could never
            // confirm it as posted.
            if number.is_empty()
                || number.chars().count() > MAX_TEXT_CHARS
                || number.chars().any(char::is_control)
                || reads_back_as_other_text(number)
            {
                return Err("voucher_text_invalid".to_string());
            }
        }
        if voucher
            .voucher_number
            .as_deref()
            .is_some_and(|number| number.chars().count() > 32 || number.contains('$'))
        {
            return Err("voucher_number_invalid".to_string());
        }
        for entry in &voucher.entries {
            if entry.ledger.trim().is_empty()
                || entry.ledger.chars().count() > MAX_MASTER_NAME_CHARS
                || without_trailing_crlf(&entry.ledger)
                    .chars()
                    .any(char::is_control)
                || reads_back_as_other_text(&entry.ledger)
                || !valid_2dp_amount(&entry.amount)
            {
                return Err("voucher_entry_invalid".to_string());
            }
            ledger_names.insert(entry.ledger.as_str());
            if ledger_names.len() > MAX_MASTER_NAMES {
                return Err("voucher_unique_ledger_limit_exceeded".to_string());
            }
        }
        let (debit, credit) = totals(std::slice::from_ref(voucher))?;
        if !debit.numeric_eq(&credit) {
            return Err("voucher_not_balanced".to_string());
        }
        if voucher.voucher_type.bank_shape().is_some() {
            validate_bank_voucher_shape(voucher)?;
        }
        invoice::validate_invoice_voucher(voucher)?;
    }
    Ok(())
}

/// Payment, Receipt and Contra take two or more entries with at least one on
/// each side, and neither a supplied voucher number nor a reference.
///
/// More than two entries is bridge#466 (one bank line settling two parties,
/// or a payment funded from two accounts). Tally stores and reads back a
/// multi-entry bank voucher with every entry (§9.3 correction table), and
/// verify_import pairs entries as a sorted multiset, so a repeated ledger on
/// one side pairs as two entries. What keeps a disguised Contra out is that
/// every leg is classified (`constrained_legs`), not only the first on each
/// side. The every-leg rule and the party choice (`render_voucher_xml`) are
/// the owner's decisions of 2026-09-22. One Bridge-built three-entry Receipt
/// has been imported over the gateway and verified live; no multi-entry
/// Payment or Contra has been, and none of the three, including that Receipt,
/// through Tally's Import menu.
///
/// One ledger on both sides would net inside the voucher, so it is refused.
fn validate_bank_voucher_shape(voucher: &ImportVoucher) -> Result<(), String> {
    let debits = voucher
        .entries
        .iter()
        .filter(|entry| entry.side == EntrySide::Dr)
        .collect::<Vec<_>>();
    let credits = voucher
        .entries
        .iter()
        .filter(|entry| entry.side == EntrySide::Cr)
        .collect::<Vec<_>>();
    // Unreachable while validate_payload's entry-count, positive-amount and
    // balance checks run first; kept so this function stays correct on its own
    // if that order changes.
    if debits.is_empty() || credits.is_empty() {
        return Err("voucher_entry_pair_required".to_string());
    }
    if debits
        .iter()
        .any(|debit| credits.iter().any(|credit| credit.ledger == debit.ledger))
    {
        return Err("voucher_entry_ledger_repeated".to_string());
    }
    // A supplied VOUCHERNUMBER's fate is decided by the *voucher type's*
    // numbering method (§9.8), which is per-type configuration this build has
    // never read: under Automatic, Tally discards the number without reporting
    // it; under Manual it keeps it. The observed book numbered these types
    // automatically, and that is one book — so the number is refused because
    // its fate is unobserved, not because every company is automatic. Reading
    // the method would need its own qualified voucher-type read contract,
    // which §9.8 says cannot be inferred; native posting refuses a supplied
    // number for exactly this reason. The bank's own reference belongs in the
    // narration, which survives either way.
    if voucher.voucher_number.is_some() {
        return Err("voucher_number_unqualified_for_type".to_string());
    }
    // §9.13's measured shape carries no REFERENCE element. Tally may well
    // accept one here, but no file carrying it has been imported and read
    // back on these types, and verify_import compares accounting entries
    // rather than this annotation, so nothing downstream would notice if it
    // were dropped or rewritten. Refuse it rather than write an unmeasured
    // variant while claiming the qualified shape.
    if voucher.reference.is_some() {
        return Err("voucher_reference_unqualified_for_type".to_string());
    }
    Ok(())
}

fn entry_for_side<'a>(voucher: &'a ImportVoucher, side: &EntrySide) -> Option<&'a ImportEntry> {
    voucher.entries.iter().find(|entry| &entry.side == side)
}

/// Every constrained (voucher, side, ledger) in the payload, with what that
/// leg must be. Empty for a Journal-only batch, which is what keeps the group
/// read off that path entirely.
fn constrained_legs(
    payload: &ImportPayload,
) -> Vec<(&ImportVoucher, &EntrySide, &str, LegRequirement)> {
    payload
        .vouchers
        .iter()
        .flat_map(|voucher| {
            voucher
                .voucher_type
                .bank_shape()
                .into_iter()
                .flat_map(move |shape| {
                    // Every entry on a constrained side, not only the first:
                    // money at any counterparty position is a disguised
                    // Contra (bridge#466).
                    shape.legs.iter().flat_map(move |(side, requirement)| {
                        voucher
                            .entries
                            .iter()
                            .filter(move |entry| &entry.side == side)
                            .map(move |entry| (voucher, side, entry.ledger.as_str(), *requirement))
                    })
                })
        })
        .collect()
}

/// One row per constrained leg, in payload order, whether or not it passed.
/// A refusal names every failing leg at once: a caller fixing them one build
/// at a time pays a full live read cycle for each.
/// What a batch's constrained legs refuse, and why.
///
/// Empty `ledgers` means every constrained leg was admitted.
struct CashBankRefusals {
    /// One row per distinct failing (ledger, requirement), not per leg. A
    /// ledger in the wrong group fails identically in every voucher that names
    /// it, and 400 copies of one problem is not 400 problems.
    ///
    /// This is what bounds the result. `validate_payload` already caps a batch
    /// at `MAX_MASTER_NAMES` distinct ledger names, rows are deduplicated by
    /// (ledger, requirement), and there are only two requirements (money and
    /// counterparty) — a ledger repeated on one side of a multi-entry voucher
    /// still yields one row — so these rows cannot exceed 200
    /// however many vouchers the batch carries. Emitting one row per leg had no
    /// such bound: a 1,000-voucher batch produced up to 2,000 rows, and once
    /// that passed the response cap the whole actionable refusal collapsed into
    /// a generic size error.
    ledgers: Vec<Value>,
    /// Failing legs before deduplication, so a caller can tell one misfiled
    /// ledger from one that poisons the entire batch.
    legs: usize,
    /// Distinct failures the byte budget left out. Deduplication bounds the
    /// row *count*; it does not bound their size, and a ledger name may be
    /// 1,024 characters.
    omitted: usize,
}

/// How many serialized bytes the refusal diagnostics may occupy, given the
/// caller's configured response cap.
///
/// Row count alone is not a bound on size: a batch may carry
/// `MAX_MASTER_NAMES` names of `MAX_MASTER_NAME_CHARS` each, which passes even
/// the default cap on names alone before any detail text. The share is a
/// quarter because the diagnostics are one field among several in the refusal
/// and the transport repeats structured content as text, so the frame carries
/// roughly twice what is measured here. `max_bytes` is configurable down to
/// 256, where a quarter leaves room for nothing — which is why one row always
/// goes out regardless, and the rest are counted rather than dropped silently.
fn refusal_diagnostic_budget(max_bytes: usize) -> usize {
    (max_bytes / 4).min(32 * 1024)
}

impl CashBankRefusals {
    /// Whether the batch is refused.
    ///
    /// This is `legs`, never `ledgers`. The row vector is presentation and is
    /// bounded by a *display* budget, so at a small configured response cap it
    /// can be empty while legs still failed. Gating on it once let a batch with
    /// failing cash/bank legs write its file, which made an accounting check
    /// switchable by `BRIDGE_AGENT_MAX_BYTES`.
    fn is_refused(&self) -> bool {
        self.legs > 0
    }
}

/// What a statement file's two ledgers are in this book's groups.
struct StatementLedgerFindings {
    /// Refused: a statement belongs to a bank account.
    bank_in_cash_in_hand: bool,
    /// Warned, not refused: some books keep a bank suspense ledger elsewhere.
    suspense: SuspenseFinding,
}

/// Where a statement file's suspense ledger sits. Only a group that is
/// established and is not Suspense A/c is "outside"; a ledger the book lacks,
/// or one whose group cannot be established, is said to be exactly that.
enum SuspenseFinding {
    Inside,
    OutsideGroup(String),
    GroupNotEstablished,
    NotInBook,
}

impl SuspenseFinding {
    /// The typed warning for the build result, or none.
    fn warning(&self, ledger: &str) -> Vec<Value> {
        let (code, message, group) = match self {
            Self::Inside => return Vec::new(),
            Self::OutsideGroup(group) => (
                "suspense_ledger_outside_suspense_group",
                "The suspense ledger this statement was parsed for is under another reserved group, not Suspense A/c. The lines it receives are tagged in their narration and counted in suspense_lines, but a review that reads the Suspense A/c group will not see them. Some books keep a bank suspense ledger elsewhere on purpose; check that this is the one intended.",
                Some(group.as_str()),
            ),
            Self::GroupNotEstablished => (
                "suspense_ledger_group_not_established",
                "The suspense ledger this statement was parsed for is in the book, but its group does not lead to a reserved group, so it is not established whether it is under Suspense A/c.",
                None,
            ),
            Self::NotInBook => (
                "suspense_ledger_not_in_book",
                "The suspense ledger this statement was parsed for is not in the book's ledger catalogue, so it is not under Suspense A/c. No voucher in this file uses it, or the build would have refused the file.",
                None,
            ),
        };
        vec![json!({
            "code": code,
            "ledger": party_name(ledger),
            "reserved_group": group,
            "message": message,
        })]
    }
}

fn statement_ledger_findings(
    ledgers: &super::bank_statement::StatementLedgers,
    observed: &ObservedMasters,
    in_catalogue: impl Fn(&str) -> bool,
) -> StatementLedgerFindings {
    let suspense = observed.classify(&ledgers.suspense_ledger);
    StatementLedgerFindings {
        bank_in_cash_in_hand: observed.classify(&ledgers.bank_ledger).is_cash_in_hand(),
        suspense: if suspense.is_suspense() {
            SuspenseFinding::Inside
        } else if let Some(group) = suspense.reserved_group() {
            SuspenseFinding::OutsideGroup(group.to_string())
        } else if in_catalogue(&ledgers.suspense_ledger) {
            SuspenseFinding::GroupNotEstablished
        } else {
            SuspenseFinding::NotInBook
        },
    }
}

/// One refused ledger, as every ledger-group refusal of a build reports it.
fn refused_ledger_row(
    ledger: &str,
    requires: &str,
    state: &CashBankState,
    first_bridge_txn_id: Option<&str>,
) -> Value {
    json!({
        "ledger": party_name(ledger),
        "requires": requires,
        "state": state.state(),
        "reserved_group": state.reserved_group(),
        "first_bridge_txn_id": first_bridge_txn_id,
    })
}

fn cash_bank_refusals(
    payload: &ImportPayload,
    observed: &ObservedMasters,
    max_bytes: usize,
) -> CashBankRefusals {
    let mut classified = BTreeMap::<&str, CashBankState>::new();
    // One row per ledger and requirement, in the order the batch's legs are
    // first refused, never by name, with or without masking. The order also
    // decides which rows the budget below keeps.
    let mut refused = Vec::<((&str, &'static str), Value)>::new();
    let mut legs = 0_usize;
    for (voucher, side, ledger, requirement) in constrained_legs(payload) {
        let state = classified
            .entry(ledger)
            .or_insert_with(|| observed.classify(ledger))
            .clone();
        let admitted = requirement.admits(&state);
        if admitted {
            continue;
        }
        legs += 1;
        let requires = match requirement {
            LegRequirement::Money => "cash_bank",
            LegRequirement::Counterparty => "not_cash_bank",
        };
        if refused.iter().any(|(row, _)| *row == (ledger, requires)) {
            continue;
        }
        refused.push((
            (ledger, requires),
            json!({
                "ledger": party_name(ledger),
                "requires": requires,
                "side": side,
                "state": state.state(),
                "refused_because": requirement.refusal(&state, voucher.voucher_type.as_str()),
                // One voucher a caller can open to see the problem, rather
                // than every voucher that repeats it.
                "first_bridge_txn_id": voucher.bridge_txn_id,
            }),
        ));
    }
    let mut budget = refusal_diagnostic_budget(max_bytes);
    let distinct = refused.len();
    let ledgers = refused
        .into_iter()
        .map(|(_, row)| row)
        // Filter rather than stop at the first row that will not fit: an
        // oversized row is one long ledger name, not a reason to discard every
        // shorter refusal behind it. Budget is only spent on rows that are
        // kept, so the remainder stays available.
        .filter(|row| {
            let cost = serde_json::to_string(row).map_or(usize::MAX, |text| text.len());
            let affordable = cost <= budget;
            if affordable {
                budget -= cost;
            }
            affordable
        })
        .collect::<Vec<_>>();
    CashBankRefusals {
        omitted: distinct.saturating_sub(ledgers.len()),
        ledgers,
        legs,
    }
}

/// Whether a value Bridge writes would read back as different text.
///
/// Every agent reader marks forbidden numeric references before parsing
/// (`TALLY_PROTOCOL_REFERENCE.md` §1.1(d)), and to keep that rewrite
/// reversible it also rewrites a literal U+FFFD directly followed by `#`,
/// digits and `;` to `U+FFFD#65533;`. A posted ledger name or voucher number
/// holding that sequence would therefore read back changed.
///
/// Call this only on a field something compares as text: a ledger name
/// (checked in `validate_payload`'s entry loop) and the voucher number
/// (checked above), which `voucher_diffs` (agent_import_verification.rs)
/// compares, and the narration, which a native post's span binding compares
/// byte for byte (`agent_import_span_identity.rs`) and the build and a native
/// post refuse (`refuse_rewritten_narration`). The reference is never
/// compared, so refusing it would refuse a value nothing downstream would
/// ever notice as changed, and `validate_payload` does not.
///
/// The value's five XML characters are escaped first (`quick_xml`'s escape),
/// so a literal `&#4;` in it is text, not a reference, and is not refused.
/// The writer (`xml_text::escape_text`) also writes CR and LF as `&#13;` and
/// `&#10;`; this check does not apply that step, so it judges only the text
/// itself, not the references the writer adds for a line ending.
fn reads_back_as_other_text(value: &str) -> bool {
    matches!(
        bridge_tally_protocol::mark_forbidden_numeric_references(&quick_xml::escape::escape(value)),
        std::borrow::Cow::Owned(_)
    )
}

/// A native post is bound to its span only if each voucher's narration reads
/// back byte for byte (`agent_import_span_identity.rs`), so a narration that
/// would read back rewritten is refused when the batch is built, and before a
/// native POST of a batch saved before this check, rather than refusing that
/// post's binding for good. `validate_payload` still admits such a saved batch
/// for review and reconciliation. The reference is never compared, so it is
/// not refused.
fn refuse_rewritten_narration(vouchers: &[ImportVoucher]) -> Result<(), String> {
    if vouchers.iter().any(|voucher| {
        voucher
            .narration
            .as_deref()
            .is_some_and(reads_back_as_other_text)
    }) {
        return Err("voucher_text_invalid".to_string());
    }
    Ok(())
}

fn contains_reserved_marker(value: &str) -> bool {
    quick_xml::escape::unescape(value)
        .map(|decoded| decoded.to_ascii_uppercase().contains("[BRIDGE:"))
        .unwrap_or_else(|_| value.to_ascii_uppercase().contains("[BRIDGE:"))
}

/// Dates cross the tool boundary in the human-friendly form but are persisted
/// in the exact Tally form used in the generated XML and verification window.
fn normalize_payload_dates(payload: &mut ImportPayload) -> Result<(), String> {
    for voucher in &mut payload.vouchers {
        voucher.date = normalized_date(&voucher.date)?.as_str().to_string();
    }
    Ok(())
}

fn validate_dates(payload: &ImportPayload, books_from: Option<&str>) -> Result<(), String> {
    let from =
        normalized_date(books_from.ok_or_else(|| "company_identity_incomplete".to_string())?)?;
    let today = super::tally_host_today();
    for voucher in &payload.vouchers {
        let date = normalized_date(&voucher.date)?;
        if date < from || date.as_str() > today.as_str() {
            return Err("voucher_date_outside_company_extent".to_string());
        }
    }
    Ok(())
}

/// The date boundaries this licence's mode admits.
fn boundary_profile_for(profile: &ImportProfileObservation) -> DateBoundaryProfile {
    if matches!(
        profile.admission_key.1.as_str(),
        "education" | "educational"
    ) {
        DateBoundaryProfile::EducationRestricted
    } else {
        DateBoundaryProfile::ModeAgnostic
    }
}

fn validate_import_dates_for_profile(
    payload: &ImportPayload,
    profile: &ImportProfileObservation,
) -> Result<(), String> {
    let boundary_profile = boundary_profile_for(profile);
    for voucher in &payload.vouchers {
        let date = bridge_tally_core::TallyDate::parse(voucher.date.clone())
            .map_err(|_| "voucher_date_invalid".to_string())?;
        if !boundary_profile.accepts_boundary(&date) {
            return Err("education_voucher_date_unsupported".to_string());
        }
    }
    Ok(())
}

/// The reserved marker this module appends to each narration of the file
/// `build_import_xml` writes, for a person to import by hand. A native post
/// sends the narration without it (#864; `render_native_vouchers_xml`).
pub(super) const NARRATION_MARKER_PREFIX: &str = "[BRIDGE:";

/// Every reserved marker occurrence in a narration, in the order written.
///
/// `None` is an occurrence that never closed -- a malformed marker is still a
/// marker, and a reader that silently dropped it would report a narration
/// Bridge plainly touched as carrying nothing. Callers decide what more than
/// one, or a malformed one, means for them; this only reports what is there.
pub(super) fn narration_markers(narration: &str) -> impl Iterator<Item = Option<&str>> {
    narration
        .match_indices(NARRATION_MARKER_PREFIX)
        .map(|(start, _)| {
            narration[start + NARRATION_MARKER_PREFIX.len()..]
                .split_once(']')
                .map(|(identity, _)| identity)
        })
}

/// The shape `build_import_xml` generates for a batch id: `bridge-` and a
/// canonical UUID. Read at presence time for the same reason `valid_txn_id`
/// is -- a batch id the writer could not have produced cannot have written a
/// marker, so hashing it derives an identity no book holds and the run
/// reports `absent` where it should have reported bad input.
///
/// Canonical spelling alone is not enough: it admits a nil, v1 or v7 UUID
/// that this writer -- `Uuid::new_v4()`, line below -- could never have
/// generated. Presence would hash such a value, find it in no book, and
/// report `absent` for input the writer could not have produced, which is
/// exactly the wrong answer under automatic numbering and invites a
/// duplicate import. `is_batch_derived` in `agent_presence.rs` checks the
/// version its writer stamps for the same reason; this checks version 4.
pub(in crate::agent) fn valid_batch_id(value: &str) -> bool {
    value
        .strip_prefix("bridge-")
        .and_then(|uuid| Uuid::parse_str(uuid).ok().map(|parsed| (uuid, parsed)))
        .is_some_and(|(spelled, parsed)| {
            parsed.to_string() == spelled
                && parsed.get_version() == Some(uuid::Version::Random)
                && parsed.get_variant() == uuid::Variant::RFC4122
        })
}

/// The character rule `build_import_xml` enforces on a caller's transaction
/// label. Presence reads it too: a label the writer would have refused cannot
/// have produced a narration marker, so hashing one would derive an identity
/// no book can hold. One rule, so read time and write time cannot drift.
pub(in crate::agent) fn valid_txn_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}
fn valid_2dp_amount(value: &str) -> bool {
    let Some((whole, fractional)) = value.split_once('.') else {
        return false;
    };
    !whole.is_empty()
        && whole.bytes().all(|byte| byte.is_ascii_digit())
        && fractional.len() == 2
        && fractional.bytes().all(|byte| byte.is_ascii_digit())
        && ExactDecimal::parse(value.to_string())
            .is_ok_and(|amount| !amount.is_zero() && !amount.is_negative())
}

fn totals(vouchers: &[ImportVoucher]) -> Result<(ExactDecimal, ExactDecimal), String> {
    let mut debit = ExactDecimal::zero();
    let mut credit = ExactDecimal::zero();
    for entry in vouchers.iter().flat_map(|voucher| &voucher.entries) {
        let amount = ExactDecimal::parse(entry.amount.clone())
            .map_err(|_| "voucher_amount_invalid".to_string())?;
        match entry.side {
            EntrySide::Dr => {
                debit = debit
                    .checked_add(&amount)
                    .map_err(|_| "voucher_amount_overflow".to_string())?
            }
            EntrySide::Cr => {
                credit = credit
                    .checked_add(&amount)
                    .map_err(|_| "voucher_amount_overflow".to_string())?
            }
        }
    }
    Ok((debit, credit))
}

fn masters_for_payload(
    payload: &ImportPayload,
    catalogue: &[String],
) -> Result<Vec<Value>, String> {
    requested_master_report(
        &requested_masters(&requested_ledger_names(payload))?,
        catalogue,
    )
}

/// A ledger name as a caller requested it, parsed at the boundary (bridge#626).
enum RequestedMaster {
    /// Bound by the core's rules, which refuse every control character.
    Named(SourceEntity),
    /// Ends in one CR LF, as some books store a ledger name; the core refuses
    /// that as input. Such a name is only ever bound byte for byte, to a live
    /// ledger holding exactly these bytes: no fold, candidate or identifier can
    /// select it, because the fold that would find it also finds its twin.
    ExactOnly(String),
}

/// A name without one trailing CR LF. Only that spelling has been observed to
/// import onto a stored ledger (bridge#626); a lone CR or LF, a repeated CR LF
/// or a line break anywhere else stays refused.
fn without_trailing_crlf(name: &str) -> &str {
    name.strip_suffix("\r\n").unwrap_or(name)
}

/// Whether a live catalogue name can be sent back as an import's ledger name:
/// either the core admits it, or it is the core-admitted name plus one trailing
/// CR LF, which `requested_masters` admits as exact-only.
fn live_spelling_importable(position: usize, name: &str) -> bool {
    SourceEntity::new(position, without_trailing_crlf(name)).is_ok()
}

/// Parses requested names at the boundary, which is where the core's own
/// bounds are enforced: a control character, or more identifiers than one name
/// may carry. A name that ends in one CR LF is admitted as
/// [`RequestedMaster::ExactOnly`] when the rest of it passes those bounds.
///
/// Kept separate from the report so a caller can run it **before** it reads
/// Tally. A request the core will refuse cannot succeed at any catalogue, so
/// spending a live read and collecting evidence of it first buys nothing and
/// costs an external round trip against the operator's books.
fn requested_masters(requested: &[String]) -> Result<Vec<RequestedMaster>, String> {
    let mut named = 0_usize;
    requested
        .iter()
        .map(|name| {
            let base = without_trailing_crlf(name);
            let entity = SourceEntity::new(named, base)
                .map_err(|error| error.safe_reason_code().to_string())?;
            if base.len() == name.len() {
                named += 1;
                Ok(RequestedMaster::Named(entity))
            } else {
                Ok(RequestedMaster::ExactOnly(name.clone()))
            }
        })
        .collect()
}

/// [`master_report`] for requested names that may include exact-only ones,
/// in the order they were requested. An exact-only name is `exact` when a live
/// ledger holds exactly its bytes and `missing` otherwise, with no candidates.
fn requested_master_report(
    requested: &[RequestedMaster],
    catalogue: &[String],
) -> Result<Vec<Value>, String> {
    let named = requested
        .iter()
        .filter_map(|master| match master {
            RequestedMaster::Named(entity) => Some(entity.clone()),
            RequestedMaster::ExactOnly(_) => None,
        })
        .collect::<Vec<_>>();
    let mut named_report = master_report(&named, catalogue)?.into_iter();
    requested
        .iter()
        .map(|master| match master {
            RequestedMaster::Named(_) => named_report
                .next()
                .ok_or_else(|| "master_report_incomplete".to_string()),
            RequestedMaster::ExactOnly(name) => Ok(if catalogue.contains(name) {
                json!({
                    "requested": party_name(name.clone()),
                    "match_state": "exact",
                    "exact_live_spelling": party_name(name.clone()),
                    "importable": true,
                })
            } else {
                json!({
                    "requested": party_name(name.clone()),
                    "match_state": "missing",
                    "reason": "master_binding_no_candidate",
                    "listing": "none",
                    "candidate_count": 0,
                    "candidate_count_is_lower_bound": false,
                    "candidates_truncated": false,
                    "candidates": [],
                    "unresolved_identity": [],
                })
            }),
        })
        .collect()
}

/// The ledgers named in bank cash answers that the book's groups refuse: a
/// cash-in-hand answer's ledger outside Cash-in-Hand
/// (`cash_ledger_not_cash_in_hand`), else another answer's ledger under
/// Suspense A/c (`cash_answer_ledger_in_suspense`). One row per ledger with the
/// reserved group it reaches, in the order the answers first name them and
/// never by name, bounded like the cash/bank refusal.
fn answered_ledger_refusals(
    required: &[super::bank_statement::AnsweredCashLedger],
    observed: &ObservedMasters,
    max_bytes: usize,
) -> Option<(&'static str, Vec<Value>, usize)> {
    let mut not_cash = Vec::<(&str, Value)>::new();
    let mut in_suspense = Vec::<(&str, Value)>::new();
    for need in required {
        let state = observed.classify(&need.ledger);
        let (refused, requires) = if need.cash_in_hand && !state.is_cash_in_hand() {
            (&mut not_cash, "cash_in_hand")
        } else if !need.cash_in_hand && state.is_suspense() {
            (&mut in_suspense, "not_suspense")
        } else {
            continue;
        };
        if !refused.iter().any(|(ledger, _)| *ledger == need.ledger) {
            refused.push((
                need.ledger.as_str(),
                refused_ledger_row(&need.ledger, requires, &state, Some(&need.bridge_txn_id)),
            ));
        }
    }
    let (reason, refused) = if !not_cash.is_empty() {
        ("cash_ledger_not_cash_in_hand", not_cash)
    } else if !in_suspense.is_empty() {
        ("cash_answer_ledger_in_suspense", in_suspense)
    } else {
        return None;
    };
    let mut budget = refusal_diagnostic_budget(max_bytes);
    let (rows, omitted) = super::bank_statement::bounded(
        refused.into_iter().map(|(_, row)| row).collect(),
        &mut budget,
    );
    Some((reason, rows, omitted))
}

/// The recorded cash-in-hand ledgers that no longer reach Cash-in-Hand (#815),
/// as the build's `cash_ledger_not_cash_in_hand` refusal reports them: one row
/// per ledger with the reserved group it reaches, bounded by `max_bytes`, and
/// how many rows were left out.
fn cash_in_hand_refusals(
    recorded: &[CashInHandLedger],
    observed: &ObservedMasters,
    max_bytes: usize,
) -> Option<(Vec<Value>, usize)> {
    let required = recorded
        .iter()
        .map(|need| super::bank_statement::AnsweredCashLedger {
            bridge_txn_id: need.bridge_txn_id.clone(),
            ledger: need.ledger.clone(),
            cash_in_hand: true,
        })
        .collect::<Vec<_>>();
    answered_ledger_refusals(&required, observed, max_bytes)
        .map(|(_, refused, omitted)| (refused, omitted))
}

/// How many vouchers Bridge's bank import sent to suspense, by the tag it
/// writes at the end of their narration
/// ([`bridge_bank_statement::proposals::suspense_tag`]), for a build's result.
/// Counts only: a line's date and amounts stay local, as in the parse result.
fn tagged_suspense_vouchers(vouchers: &[ImportVoucher]) -> Value {
    use bridge_bank_statement::proposals::SuspenseTag;
    let (mut purpose_not_confirmed, mut unidentified) = (0_usize, 0_usize);
    for voucher in vouchers {
        let tag = voucher.narration.as_deref().and_then(|narration| {
            bridge_bank_statement::proposals::suspense_tag(
                narration,
                voucher.entries.iter().map(|entry| entry.ledger.as_str()),
            )
        });
        match tag {
            Some(SuspenseTag::PurposeNotConfirmed) => purpose_not_confirmed += 1,
            Some(SuspenseTag::Unidentified) => unidentified += 1,
            None => {}
        }
    }
    json!({
        "count": purpose_not_confirmed + unidentified,
        "purpose_not_confirmed": purpose_not_confirmed,
        "unidentified": unidentified,
    })
}

/// `rows` in the order the batch first names each row's ledger. A list of the
/// batch's ledgers goes out in this order and never by name, with or without
/// masking. A ledger the batch does not name goes last, in its incoming order:
/// the lists built from the batch hold none, and a recorded verdict names the
/// ledgers the batch was bound to.
fn in_batch_order<T>(
    mut rows: Vec<T>,
    vouchers: &[ImportVoucher],
    ledger: impl for<'a> Fn(&'a T) -> &'a str,
) -> Vec<T> {
    rows.sort_by_cached_key(|row| {
        vouchers
            .iter()
            .flat_map(|voucher| &voucher.entries)
            .position(|entry| entry.ledger == ledger(row))
            .unwrap_or(usize::MAX)
    });
    rows
}

/// A masters verdict as an answer carries it: the ledgers it names in the order
/// the batch names them (`in_batch_order`), not the order they were recorded
/// in. The recorded verdict is left as it was saved: a recorded review is bound
/// to its bytes.
fn served_masters_verdict(mut verdict: Value, vouchers: &[ImportVoucher]) -> Value {
    if let Some(ledgers) = verdict.get_mut("ledgers").and_then(Value::as_array_mut) {
        *ledgers = in_batch_order(std::mem::take(ledgers), vouchers, |ledger| {
            ledger.as_str().unwrap_or_default()
        });
    }
    verdict
}

/// The ledgers a batch names, each once, in name order. A list of them sent to
/// the caller is put `in_batch_order` first.
fn requested_ledger_names(payload: &ImportPayload) -> Vec<String> {
    payload
        .vouchers
        .iter()
        .flat_map(|voucher| &voucher.entries)
        .map(|entry| entry.ledger.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Bounds the candidate names one unbound entity may copy into a tool result.
/// The binding contract bounds the candidate *count*; this bounds their bytes,
/// which is an egress concern rather than a matching one.
const MAX_CANDIDATE_RESULT_BYTES: usize = 8_192;

/// Binds requested ledger names against the observed catalogue.
///
/// The rules live in `bridge_tally_core::master_binding` so this tool and the
/// desktop preparation screen cannot drift apart; see
/// `docs/adr/0016-master-binding-authority.md`. This function only renders the
/// report, and it never promotes a candidate into a spelling.
fn master_report(entities: &[SourceEntity], catalogue: &[String]) -> Result<Vec<Value>, String> {
    let catalog = MasterCatalog::new(MasterClass::Ledger, catalogue)
        .map_err(|error| error.safe_reason_code().to_string())?;
    let report = master_binding::bind(&catalog, entities)
        .map_err(|error| error.safe_reason_code().to_string())?;
    Ok(report.entities().iter().map(master_match_json).collect())
}

fn master_match_json(binding: &EntityBinding) -> Value {
    let requested = party_name(binding.source_name.clone());
    match &binding.status {
        BindingStatus::Bound {
            catalog_name,
            basis,
        } => {
            // Only byte-exact equality may be reported as `exact`: the import
            // file carries the name verbatim, and build_import_xml admits
            // nothing else.
            let match_state = match basis {
                BindingBasis::ExactName => "exact",
                BindingBasis::Identifier => "identifier",
            };
            // A catalogue name can now hold characters the *proposal* side
            // refuses -- a ledger genuinely named across two lines, say. That
            // asymmetry is deliberate: the book's name is a fact, a caller's
            // proposed name is input. But it means the live spelling is not
            // always something the caller can send back, and the guidance below
            // used to tell them to copy it regardless. Following that failed the
            // whole batch on `master_name_unsafe`, because `requested_masters`
            // collects into one Result and refuses on the first bad name.
            //
            // Ask the proposal constructor rather than restating its rule, so
            // the two can never disagree about what is admissible. A trailing
            // line break is the one exception it does not know (bridge#626).
            let importable = live_spelling_importable(binding.position, catalog_name);
            json!({
                "requested": requested,
                "match_state": match_state,
                "exact_live_spelling": party_name(catalog_name.clone()),
                "importable": importable,
            })
        }
        BindingStatus::Ambiguous(unresolved) | BindingStatus::Unmatched(unresolved) => {
            let mut bytes = 0_usize;
            let candidates = unresolved
                .candidates
                .listed()
                .iter()
                .take_while(|candidate| {
                    bytes = bytes.saturating_add(candidate.catalog_name.len());
                    bytes <= MAX_CANDIDATE_RESULT_BYTES
                })
                .map(|candidate| {
                    json!({
                        "name": party_name(candidate.catalog_name.clone()),
                        "rule": candidate.rule,
                    })
                })
                .collect::<Vec<_>>();
            // The listing state is carried explicitly rather than left to be
            // inferred from an empty array. A model is exactly the caller that
            // would read "no candidates" as "no such ledger exists", and for
            // `withheld` that is false: masters were found and deliberately not
            // listed because none of them separates the requested name.
            let found = unresolved.candidates.found();
            let listing = match unresolved.candidates {
                Candidates::None => "none",
                Candidates::Withheld { .. } => "withheld",
                _ if candidates.len() < found => "truncated",
                _ => "listed",
            };
            // No `exact_live_spelling`. Naming one candidate as the live
            // spelling is the auto-resolution that rejected a batch once.
            json!({
                "requested": requested,
                "match_state": match binding.status {
                    BindingStatus::Unmatched(_) => "missing",
                    _ => "near_miss",
                },
                "reason": unresolved.reason.safe_reason_code(),
                "listing": listing,
                "candidate_count": found,
                "candidate_count_is_lower_bound": unresolved
                    .candidates
                    .count_is_lower_bound(),
                "candidates_truncated": listing != "listed" && listing != "none",
                "candidates": candidates,
                "unresolved_identity": unresolved
                    .unresolved_identity
                    .iter()
                    .map(|identifier| json!({
                        "kind": identifier.kind,
                        "value": party_name(identifier.value.clone()),
                    }))
                    .collect::<Vec<_>>(),
            })
        }
    }
}

fn master_recovery_guidance(report: &[Value]) -> String {
    let mut guidance = Vec::new();
    if report.iter().any(|master| {
        master["match_state"] == "identifier" && master["importable"] != Value::Bool(false)
    }) {
        guidance
            .push("For identifier-bound entries, copy exact_live_spelling from this fresh result.");
    }
    // Said separately, because the remedy is the opposite one: this spelling
    // cannot be copied back at all, and no retry of this payload will post
    // against that ledger.
    if report.iter().any(|master| {
        master["importable"] == Value::Bool(false) && master.get("folded_twins").is_none()
    }) {
        guidance.push("One matched ledger is named with a character imports do not accept, so its exact_live_spelling cannot be sent back; have an operator rename it in Tally, then run validate_masters again.");
    }
    // Its own remedy: the spelling is admissible, but another live ledger folds
    // equal to it, and Tally's loose import lookup could post to either (#626).
    if report
        .iter()
        .any(|master| master.get("folded_twins").is_some())
    {
        guidance.push("A requested ledger folds equal to another live ledger (folded_twins: the same name apart from case, spacing, dashes, slashes or quotes, or a trailing line break), and which of them Tally's import would post to is not established, so neither is importable; have an operator rename one of them in Tally, then run validate_masters again.");
    }
    if report
        .iter()
        .any(|master| master["match_state"] == "missing")
    {
        guidance.push("For missing ledgers, correct the source spelling or have an operator create the legitimate ledger externally, then run validate_masters again.");
    }
    if report
        .iter()
        .any(|master| master["match_state"] == "near_miss")
    {
        guidance.push("For near-misses, have an operator explicitly select the intended ledger and run validate_masters again; do not copy a candidate automatically.");
    }
    guidance.push("After operator review, update the payload to each confirmed exact live spelling and run validate_masters again before building. No file was written.");
    guidance.join(" ")
}

fn render_import_xml(company: &str, vouchers: &[ImportVoucher], batch_id: &str) -> String {
    let messages = vouchers
        .iter()
        .map(|voucher| {
            let identity = import_identity(batch_id, &voucher.bridge_txn_id);
            render_voucher_xml(voucher, identity, NarrationAttribution::Tagged(identity))
        })
        .collect::<String>();
    render_import_envelope(company, &messages)
}

/// The native post's request: each voucher paired with its own REMOTEID. The
/// caller pairs them, after checking there is one id per voucher. Every id
/// must be fresh for every attempt: a public file may already have been
/// imported and edited, and reusing its client REMOTEID for a native Create
/// can make Tally treat it as an upsert. The caller records the ids with the
/// dispatch intent before sending, because Tally deletes only by them and
/// never exports them (bridge#579). The narration carries no attribution tag:
/// a client's narration is print-ready (owner decision, 2026-09-28), and a
/// native post is attributed by its own AlterID span instead
/// (`agent_import_span_identity.rs`).
fn render_native_vouchers_xml<'a>(
    company: &str,
    vouchers_with_remote_ids: impl Iterator<Item = (&'a ImportVoucher, Uuid)>,
) -> String {
    let messages: String = vouchers_with_remote_ids
        .map(|(voucher, remote_id)| {
            render_voucher_xml(voucher, remote_id, NarrationAttribution::Untagged)
        })
        .collect();
    render_import_envelope(company, &messages)
}

/// Whether a rendered voucher's narration carries Bridge's `[BRIDGE:…]` tag.
/// A file a person imports by hand keeps it, because Bridge never sees that
/// import and the tag is its only attribution; a native post does not.
#[derive(Clone, Copy)]
enum NarrationAttribution {
    Tagged(Uuid),
    Untagged,
}

fn render_import_envelope(company: &str, messages: &str) -> String {
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{messages}</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>", xml_escape(company))
}

/// The narration a post sends for `voucher`, and the one its approval dialog
/// shows (#1055 point 5): the saved text without leading or trailing spaces, so
/// none reaches Tally and the person approves exactly the text that is sent.
/// `None` when the voucher has no narration.
fn posted_narration(voucher: &ImportVoucher) -> Option<&str> {
    voucher.narration.as_deref().map(str::trim)
}

fn render_voucher_xml(
    voucher: &ImportVoucher,
    remote_id: Uuid,
    attribution: NarrationAttribution,
) -> String {
    let text = posted_narration(voucher).unwrap_or("");
    let narration = match attribution {
        NarrationAttribution::Tagged(attribution_id) => format!(
            "<NARRATION>{}</NARRATION>",
            xml_escape(format!("{text} [BRIDGE:{attribution_id}]").trim())
        ),
        NarrationAttribution::Untagged => format!("<NARRATION>{}</NARRATION>", xml_escape(text)),
    };
    // REFERENCE is retained because it is part of the agent input contract. Its effect is not used as posting evidence; verify_import compares the accounting entries, not this annotation.
    let reference = voucher
        .reference
        .as_deref()
        .map(|value| format!("<REFERENCE>{}</REFERENCE>", xml_escape(value)))
        .unwrap_or_default();
    let voucher_number = voucher
        .voucher_number
        .as_deref()
        .map(|value| format!("<VOUCHERNUMBER>{}</VOUCHERNUMBER>", xml_escape(value)))
        .unwrap_or_default();
    if voucher.voucher_type.is_invoice() {
        let date = normalized_date(&voucher.date)
            .map(|date| date.as_str().to_string())
            .unwrap_or_default();
        // An invoice reaches here only after the build recorded what it
        // observed (`admit_saved_voucher_integrity` refuses one that has not).
        // An unobserved one renders as nothing rather than as a guess.
        return invoice::render_sales_invoice_xml(voucher, remote_id, &date, &narration)
            .unwrap_or_default();
    }
    let shape = voucher.voucher_type.bank_shape();
    // §9.13's measured files put the debit first in every voucher, and a
    // caller's ordering is not a fact about the batch. Canonicalise rather than
    // refuse: it costs the caller nothing and removes the variance entirely.
    // A Journal keeps the caller's order, since its own measured file did.
    let mut ordered = voucher.entries.iter().collect::<Vec<_>>();
    if shape.is_some() {
        ordered.sort_by_key(|entry| match entry.side {
            EntrySide::Dr => 0,
            EntrySide::Cr => 1,
        });
    }
    let entries = ordered.iter().map(|entry| {
        let amount = match entry.side { EntrySide::Dr => format!("-{}", entry.amount), EntrySide::Cr => entry.amount.clone() };
        format!("<ALLLEDGERENTRIES.LIST><LEDGERNAME>{}</LEDGERNAME><ISDEEMEDPOSITIVE>{}</ISDEEMEDPOSITIVE><AMOUNT>{}</AMOUNT></ALLLEDGERENTRIES.LIST>", xml_escape(&entry.ledger), entry.side.tally_positive(), amount)
    }).collect::<String>();
    let date = normalized_date(&voucher.date)
        .map(|date| date.as_str().to_string())
        .unwrap_or_default();
    // §9.13: the imported Payment/Receipt/Contra files carried EFFECTIVEDATE
    // beside DATE, and named the party on the side opposite the money. The
    // Journal shape qualified in §9.8 carries neither element, and is left
    // byte-identical to the file that measurement actually ran on.
    let effective_date = shape
        .as_ref()
        .map(|_| format!("<EFFECTIVEDATE>{date}</EFFECTIVEDATE>"))
        .unwrap_or_default();
    // PARTYLEDGERNAME is the first entry on the counterparty side, in the
    // voucher's own order: the single counterparty when there is one, and a
    // deterministic choice when several parties share a voucher (bridge#466,
    // owner decision 2026-09-22; Tally 7.1 Silver read the bank ledger back as
    // the party on a three-entry Receipt, and omitting the element was not
    // tried).
    let party = shape
        .as_ref()
        .and_then(BankVoucherShape::party_side)
        .and_then(|side| entry_for_side(voucher, side))
        .map(|entry| {
            format!(
                "<PARTYLEDGERNAME>{}</PARTYLEDGERNAME>",
                xml_escape(&entry.ledger)
            )
        })
        .unwrap_or_default();
    // The qualified human-import slice uses Create + stable client REMOTEID;
    // native posting uses a separate private REMOTEID and no supplied number.
    // See docs/tally/TALLY_PROTOCOL_REFERENCE.md §9.8 for scope and limits.
    format!("<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"{}\" VCHTYPE=\"{}\" ACTION=\"Create\" OBJVIEW=\"Accounting Voucher View\"><DATE>{date}</DATE>{effective_date}<VOUCHERTYPENAME>{}</VOUCHERTYPENAME>{party}{voucher_number}{narration}{reference}{entries}</VOUCHER></TALLYMESSAGE>", remote_id, voucher.voucher_type.as_str(), voucher.voucher_type.as_str())
}

/// One verification window as read: the admitted rows, the evidence for the
/// data reads alone, the pre-flight reads kept apart from it, and the ranges
/// read so that a corroborating read can replay them.
struct VerificationWindowRead {
    source: ImportReadSource,
    evidence: Evidence,
    preflight_evidence: Option<Evidence>,
    /// The closing bracket, read after the data parts.
    closing_evidence: Option<Evidence>,
    reads: Vec<super::WindowPart>,
    /// What a corroborating replay of this read must carry.
    witness: Option<super::WindowWitness>,
    /// Tally refused one of this read's data requests as too large or timed out.
    refused_a_part: bool,
}

pub(super) fn render_import_verification_read(
    company: &str,
    from: &bridge_tally_core::TallyDate,
    to: &bridge_tally_core::TallyDate,
) -> String {
    render_import_verification_in_span(company, from, to, None)
}

/// [`render_import_verification_read`], optionally narrowed to an AlterID span.
/// `None` renders the unnarrowed request byte for byte; `Some` is one part of
/// a day too heavy for one read (protocol reference §11c). Every part is read
/// and verified against; none is discarded.
pub(super) fn render_import_verification_in_span(
    company: &str,
    from: &bridge_tally_core::TallyDate,
    to: &bridge_tally_core::TallyDate,
    span: Option<super::AlterIdSpan>,
) -> String {
    let span_filter = span.map(super::AlterIdSpan::filter).unwrap_or_default();
    // A quoted `$$Date:"…"` literal takes only a date: XML escaping cannot
    // protect it, since Tally decodes `&quot;` before evaluating (#861).
    let (from, to) = (from.as_str(), to.as_str());
    format!("<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Agent Import Verification</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY><SVFROMDATE TYPE=\"Date\">{from}</SVFROMDATE><SVTODATE TYPE=\"Date\">{to}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeImportWindow\">$Date &gt;= $$Date:\"{from}\" AND $Date &lt;= $$Date:\"{to}\"{span_filter}</SYSTEM><COLLECTION NAME=\"Bridge Agent Import Verification\" ISMODIFY=\"No\"><TYPE>Voucher</TYPE><FETCH>DATE,VOUCHERNUMBER,VOUCHERTYPENAME,REMOTEID,GUID,MASTERID,ALTERID,NARRATION,ISCANCELLED,ISOPTIONAL,ALLLEDGERENTRIES.LEDGERNAME,ALLLEDGERENTRIES.AMOUNT,ALLLEDGERENTRIES.ISDEEMEDPOSITIVE,EFFECTIVEDATE</FETCH><FILTERS>BridgeImportWindow</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>", xml_escape(company))
}

pub(super) fn local_evidence(label: &str) -> Evidence {
    Evidence {
        request_sha256: sha256_hex(label.as_bytes()),
        response_sha256: sha256_hex(label.as_bytes()),
        bytes: 0,
        state: "complete",
        read_at: None,
        duration_ms: None,
        reason_code: None,
    }
}
fn now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
/// The state of a masters check that has not finished (#239).
pub(super) const MASTERS_CHECK_PENDING: &str = "check_pending";

// The masters-check pair (`*.masters_check.json`, `*.masters_doubt.json`)
// holds a post's durable doubts: the masters verdict (#239) and, for a batch
// of more than one voucher, the batch step verdict (`batch_step`), whose own
// doubt is kept in `*.batch_step_doubt.json`. Neither verdict overwrites or
// masks the other. The names stay as they were, so older records still read;
// a record with no `batch_step` has no step verdict, which is right for a
// one-voucher post.

fn masters_check_path(imports: &Path, batch_id: &str) -> PathBuf {
    imports.join(format!("{batch_id}.masters_check.json"))
}

fn masters_doubt_path(imports: &Path, batch_id: &str) -> PathBuf {
    imports.join(format!("{batch_id}.masters_doubt.json"))
}

fn batch_step_doubt_path(imports: &Path, batch_id: &str) -> PathBuf {
    imports.join(format!("{batch_id}.batch_step_doubt.json"))
}

/// Write an observed doubt to its own file. When that fails, the verdict that
/// goes into the check record says so (`doubt_record: unavailable`, #722):
/// it still holds the doubt, and it says in-band why no review can find it.
/// The readers decide from the file's absence, not from this mark, so a file
/// lost later is refused the same way.
fn record_doubt(path: &Path, verdict: &mut Value) {
    if write_masters_record(path, verdict).is_err() {
        verdict["doubt_record"] = json!("unavailable");
    }
}

/// The durable checks recorded for this batch: the masters verdict (#239),
/// with the batch step verdict beside it as `batch_step` when the post was a
/// batch. An observed doubt of either kind is kept in a file of its own that
/// nothing removes or replaces, and it overrides that kind's verdict in the
/// check record. Absent only for a batch dispatched before these records
/// existed.
fn read_masters_check(imports: &Path, batch_id: &str) -> Option<Value> {
    let check = read_masters_record(&masters_check_path(imports, batch_id));
    let mut masters =
        read_masters_record(&masters_doubt_path(imports, batch_id)).or_else(|| check.clone())?;
    let step = read_masters_record(&batch_step_doubt_path(imports, batch_id)).or_else(|| {
        check
            .as_ref()
            .and_then(|check| check.get("batch_step").cloned())
    });
    if let (Some(step), Some(fields)) = (step, masters.as_object_mut()) {
        fields.insert("batch_step".into(), step);
    }
    Some(masters)
}

/// A record that exists but cannot be opened, read or parsed reads as a
/// pending check: a doubt, never an admission.
fn read_masters_record(path: &Path) -> Option<Value> {
    let unreadable = || Some(json!({"state": MASTERS_CHECK_PENDING}));
    let mut file = match super::local_file::open_local_file(path, false) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return None,
        Err(_) => return unreadable(),
    };
    let Some(bytes) = read_capped_record(&mut file) else {
        return unreadable();
    };
    serde_json::from_slice(&bytes).ok().or_else(unreadable)
}

/// A persisted record read whole, or `None` when it cannot be read or is
/// larger than `MAX_RECORD_BYTES`, the bound every persisted record has
/// (#837). One byte past the bound is read, so a larger record is refused,
/// never truncated.
fn read_capped_record(file: &mut fs::File) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(
        &mut std::io::Read::take(file, (ledger::MAX_RECORD_BYTES + 1) as u64),
        &mut bytes,
    )
    .ok()?;
    (bytes.len() <= ledger::MAX_RECORD_BYTES).then_some(bytes)
}

/// Staged under a name no other writer uses, then renamed into place, so
/// writers need no lock and never collide.
fn write_masters_record(path: &Path, record: &Value) -> Result<(), String> {
    let staged = path.with_extension(format!("{}.next", Uuid::new_v4()));
    let bytes =
        serde_json::to_vec_pretty(record).map_err(|_| "proof_serialization_failed".to_string())?;
    write_private(&staged, &bytes)?;
    fs::rename(&staged, path).map_err(|_| {
        let _ = fs::remove_file(&staged);
        "import_file_write_failed".to_string()
    })
}

impl Server {
    /// Mark this batch's masters check pending before it can be dispatched. A
    /// crash before the check finishes, a concurrent reader, or a failed later
    /// write then reads a doubt, never an absent record; a post whose record
    /// cannot be written is not sent. It never touches an observed doubt.
    #[cfg(test)]
    pub(super) fn record_masters_check_pending(&self, batch_id: &str) -> Result<(), String> {
        self.record_post_checks_pending(batch_id, false)
    }

    /// As [`Self::record_masters_check_pending`], and for a batch the step
    /// verdict pending too, so a crash before it is recorded reads as doubt.
    pub(super) fn record_post_checks_pending(
        &self,
        batch_id: &str,
        batch: bool,
    ) -> Result<(), String> {
        let unavailable = |_| "post_masters_record_unavailable".to_string();
        let imports = self.imports_dir().map_err(unavailable)?;
        let mut pending = json!({"state": MASTERS_CHECK_PENDING});
        if batch {
            pending["batch_step"] = json!({"state": MASTERS_CHECK_PENDING});
        }
        write_masters_record(&masters_check_path(&imports, batch_id), &pending).map_err(unavailable)
    }

    /// Record a batch's step verdict: `matched` only when the target's
    /// voucher mark moved by exactly Tally's CREATED. Anything else is doubt,
    /// which goes first to its own file. The check record keeps its masters
    /// verdict as it is. A verdict that cannot be written leaves the step
    /// pending, which is doubt.
    #[cfg(test)]
    pub(super) fn record_batch_step_verdict(&self, batch_id: &str, target_voucher_step: &Value) {
        self.record_batch_step_verdict_caused(batch_id, target_voucher_step, None);
    }

    /// As [`Self::record_batch_step_verdict`], with `cause` naming why the
    /// step could not be read when the marks readback failed (#884). It is
    /// recorded beside the verdict and never changes it: a step not read is
    /// still doubt.
    pub(super) fn record_batch_step_verdict_caused(
        &self,
        batch_id: &str,
        target_voucher_step: &Value,
        cause: Option<&str>,
    ) {
        let Ok(imports) = self.imports_dir() else {
            return;
        };
        let mut verdict = if target_voucher_step["matches_created"] == true {
            json!({"state": "matched", "target_voucher_step": target_voucher_step})
        } else {
            json!({"state": "unmatched", "target_voucher_step": target_voucher_step})
        };
        if let Some(cause) = cause {
            verdict["cause"] = json!(cause);
        }
        if verdict["state"] != "matched" {
            record_doubt(&batch_step_doubt_path(&imports, batch_id), &mut verdict);
        }
        let path = masters_check_path(&imports, batch_id);
        if let Some(mut check) = read_masters_record(&path) {
            if check.is_object() {
                check["batch_step"] = verdict;
                let _ = write_masters_record(&path, &check);
            }
        }
    }

    /// Record a finished check's verdict and return what the batch's records
    /// now say. An observed doubt goes first to its own file, which then
    /// outranks any later verdict. A check that could not run
    /// (`check_unavailable`) is not recorded, so a later readback checks
    /// again; a verdict that cannot be written leaves the check pending.
    #[cfg(test)]
    pub(super) fn record_masters_verdict(&self, batch_id: &str, verdict: Value) -> Value {
        self.record_masters_verdict_for(batch_id, verdict, false)
    }

    /// As [`Self::record_masters_verdict`], for a batch (`batch`) keeping its
    /// step verdict beside: pending when none can be read, never dropped.
    pub(super) fn record_masters_verdict_for(
        &self,
        batch_id: &str,
        verdict: Value,
        batch: bool,
    ) -> Value {
        if verdict["state"] == "check_unavailable" {
            return verdict;
        }
        let pending = json!({"state": MASTERS_CHECK_PENDING});
        let Ok(imports) = self.imports_dir() else {
            return pending;
        };
        let mut verdict = verdict;
        if verdict["state"] == "posted_under_changed_masters" {
            record_doubt(&masters_doubt_path(&imports, batch_id), &mut verdict);
        }
        // The batch step verdict beside it is kept, never overwritten; for a
        // batch whose step verdict cannot be read, it stays pending (doubt).
        let path = masters_check_path(&imports, batch_id);
        let step = read_masters_record(&path)
            .and_then(|check| check.get("batch_step").cloned())
            .or_else(|| batch.then(|| json!({"state": MASTERS_CHECK_PENDING})));
        if let (Some(step), Some(fields)) = (step, verdict.as_object_mut()) {
            fields.insert("batch_step".into(), step);
        }
        let _ = write_masters_record(&path, &verdict);
        read_masters_check(&imports, batch_id).unwrap_or(pending)
    }
}

fn verified_baseline_path(imports: &Path, batch_id: &str) -> PathBuf {
    imports.join(format!("{batch_id}.baseline.json"))
}

/// A build's verified baseline, or `None` when it has none or it cannot be
/// read. Either way an amendment of that build refuses.
fn read_verified_baseline(imports: &Path, batch_id: &str) -> Option<amend::VerifiedBaseline> {
    read_verified_baseline_for(imports, batch_id, 1)
}

/// As [`read_verified_baseline`], for a build of `voucher_count` vouchers: a
/// batch whose step verdict is doubted, or was never recorded, is no
/// baseline either.
fn read_verified_baseline_for(
    imports: &Path,
    batch_id: &str,
    voucher_count: usize,
) -> Option<amend::VerifiedBaseline> {
    // A batch in doubt about its ledgers, or whose check is still pending
    // (#239), is no baseline, whenever its baseline was written. Only a
    // native post has these records, so a build imported by file has none
    // and no doubt; a native batch's step must have matched as well.
    if let Some(check) = read_masters_check(imports, batch_id) {
        if post::post_doubt(Some(&check), voucher_count).is_some() {
            return None;
        }
    }
    let mut file =
        super::local_file::open_local_file(&verified_baseline_path(imports, batch_id), false)
            .ok()?;
    let bytes = read_capped_record(&mut file)?;
    serde_json::from_slice(&bytes).ok()
}

/// Record each voucher's first verified ALTERID; a voucher already recorded
/// keeps its value. Called under the import admission lock.
fn record_verified_baseline(imports: &Path, batch_id: &str, proof: &Value) -> Result<(), String> {
    let path = verified_baseline_path(imports, batch_id);
    let mut baseline = if path.exists() {
        // An unreadable baseline is never rewritten: nothing proves which
        // values were first, so amendments of this build stay refused.
        read_verified_baseline(imports, batch_id)
            .ok_or_else(|| "verified_baseline_unreadable".to_string())?
    } else {
        amend::VerifiedBaseline::default()
    };
    if amend::record_first_verified(&mut baseline, proof) {
        let bytes = serde_json::to_vec_pretty(&baseline)
            .map_err(|_| "verified_baseline_serialization_failed".to_string())?;
        // Staged and renamed, so a failed write leaves the previous file whole
        // rather than a truncated one that would refuse every amendment.
        let staged = imports.join(format!("{batch_id}.baseline.json.next"));
        write_private(&staged, &bytes)?;
        fs::rename(&staged, &path).map_err(|_| "verified_baseline_publish_failed".to_string())?;
    }
    Ok(())
}

fn verified_baselines(imports: &Path, lineage: &amend::Lineage) -> amend::VerifiedBaselines {
    amend::VerifiedBaselines(
        lineage
            .builds
            .iter()
            .filter_map(|build| {
                read_verified_baseline_for(
                    imports,
                    &build.batch.batch_id,
                    build.batch.vouchers.len(),
                )
                .map(|baseline| (build.batch.batch_id.clone(), baseline))
            })
            .collect(),
    )
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = super::local_file::open_local_file(path, true)
        .map_err(|_| "import_file_write_failed".to_string())?;
    set_private_file(&file)?;
    file.set_len(0)
        .and_then(|_| file.write_all(bytes))
        .and_then(|_| file.sync_data())
        .map_err(|_| "import_file_write_failed".to_string())
}
fn set_private_file(file: &std::fs::File) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(fs::Permissions::from_mode(0o600))
            .map_err(|_| "import_file_permissions_failed".to_string())?;
    }
    #[cfg(not(unix))]
    let _ = file;
    Ok(())
}
#[cfg(test)]
#[path = "agent_import_tests.rs"]
mod tests;

/// A verification page as it is served, from the saved proof's bytes: the
/// proof, with every ledger name marked so the response's redaction applies,
/// and the page from `offset`. Page 1 and every later page are served
/// through this, so they cannot differ in what they mark.
fn served_verification_page(persisted: &[u8], offset: usize) -> Result<(Value, Value), String> {
    let mut proof: Value = serde_json::from_slice(persisted)
        .map_err(|_| "verification_proof_unreadable".to_string())?;
    mark_verification_names(&mut proof);
    let page = verification_response_page(&proof, &sha256_hex(persisted), offset);
    Ok((proof, page))
}

#[cfg(test)]
#[path = "agent_import_file_tests.rs"]
mod file_tests;

/// The catalogue's ledgers as a request can reach them: each row spelling with its stored name (#1085).
fn resolvable_ledgers(
    catalogue: &bridge_tally_protocol::StandardLedgerCatalog,
) -> Vec<super::ledger_candidates::CatalogueLedger> {
    catalogue
        .spellings()
        .map(|(row, stored)| super::ledger_candidates::CatalogueLedger::new(row, stored))
        .collect()
}

/// Each ledger of a catalogue with its immediate parent group as Tally returned it (#1230).
fn owned_parents(
    catalogue: &bridge_tally_protocol::StandardLedgerCatalog,
) -> Vec<(String, Option<String>)> {
    catalogue
        .parents()
        .map(|(ledger, parent)| (ledger.to_string(), parent.map(str::to_string)))
        .collect()
}
