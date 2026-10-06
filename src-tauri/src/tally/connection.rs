use anyhow::Context as _;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{
    atomic::{AtomicU64, AtomicU8, Ordering},
    Arc,
};
#[cfg(feature = "voucher-scan")]
use std::time::{Duration, Instant};

use super::xml_parser::{TallyLedger, TallyVoucher};
use super::{
    standard_ledger_catalog::{
        parse_standard_ledger_catalog_response, render_standard_ledger_catalog_request,
    },
    tdl_engine,
    validators::{normalize_company_guid, normalize_company_name},
    xml_parser::{self, TallyCompany},
    VerifiedCompanyIdentity,
};
use crate::endpoint_wire::WireGateConfig;
use crate::reports::party_ledger_master::{PartyLedgerMasterRow, PartyLedgerMasterSource};
use crate::tally::runtime::{
    with_read_evidence, PartyLedgerMasterCurrencyAssertion, RuntimeReadEvidence,
};
use bridge_tally_core::{
    CapabilityEvidence, CapabilityFeatureId, CapabilityPackId, CapabilityProfile, CapabilityState,
    EvidenceConfidence, LicenseTier, TransportId,
};
#[cfg(feature = "voucher-scan")]
use bridge_tally_protocol::outstandings::{
    parse_ledger_opening_coverage, verify_empty_partition_witness_pair_with_wire_evidence,
    verify_segment_pair_with_wire_evidence, voucher_empty_partition_witness_request,
    voucher_outstandings_request, AlterIdRange, LedgerOpeningCoverage, NarrowDateWindow,
    PinnedCompany, SegmentVerification, SegmentWireEvidence, VoucherOutstandingsRequestXml,
    WitnessPairVerification,
};
use bridge_tally_protocol::{
    ledger_census::{
        render_ledger_census_slice_request, CensusLimits, LedgerCensus, LedgerCensusError,
        LedgerCensusPlan, LedgerCount,
    },
    native_outstandings::{
        parse_compliance_ledger_snapshot_for_company, parse_native_group_snapshot_with_evidence,
        parse_native_ledger_snapshot_for_company, render_native_group_snapshot_request,
        render_native_ledger_export_request, render_native_ledger_snapshot_request,
        render_native_ledger_snapshot_request_for_parents, render_native_voucher_export_request,
        render_party_ledger_master_request, render_party_ledger_master_request_for_parents,
        BaseCurrencyName, ForeignCurrencyLedger, LedgerSnapshotEntry, NativeLedgerExportPeriod,
        NativeLedgerSnapshotPeriod, NativeOutstandingsError,
    },
    outstandings_shared::{
        parse_company_book_extent_v2, parse_company_ledger_count,
        render_company_ledger_count_request, require_master_witness, CompanyBookExtent,
        DateBoundaryProfile, OutstandingsError,
    },
    parent_partition::{ParentPart, ParentPartition, ParentPartitionError, PartitionLimits},
    parse_companies_for_interactive_discovery, parse_company_gateway_capability_observation,
    parse_ledger_census_slice, parse_native_ledger_source_records_with_evidence,
    parse_native_party_ledger_master_records_leaving_unparsed,
    parse_native_party_ledger_master_structure, parse_native_voucher_source_records_with_evidence,
    parse_standard_ledger_catalog, parse_standard_ledger_identity_observation,
    xml_read_profiles::{ReadOnlyProfile, ValidatedCompanyName},
    ParsedExport, ParsedSourceRecord, PartyLedgerMasterRecord, StandardLedgerCatalogError,
    TallyTextEncoding, MAX_STANDARD_LEDGER_IDENTITY_ROWS,
};
use bridge_tally_transport::{
    canonical_loopback_origin as transport_canonical_origin, TallyEndpointConfig,
    TallyHttpTransport, TallyTransportError, WireHeldTransport,
};

pub type TallyConfig = TallyEndpointConfig;

fn ledger_display_key(name: &str, parent: Option<&str>) -> String {
    let parent = parent.unwrap_or_default();
    format!("{}:{name}{}:{parent}", name.len(), parent.len())
}

fn party_ledger_master_balance_snapshot_error(error: NativeOutstandingsError) -> anyhow::Error {
    match error {
        NativeOutstandingsError::InvalidResponse(
            "ledger_response_company_guid_missing" | "ledger_response_company_guid_mismatch",
        ) => anyhow::Error::new(
            PartyLedgerMasterSourceValidationError::BalanceCompanyIdentityUnverified,
        ),
        error => anyhow::Error::new(error),
    }
}

fn party_ledger_master_group_snapshot_error(error: NativeOutstandingsError) -> anyhow::Error {
    match error {
        NativeOutstandingsError::InvalidResponse(
            "group_response_company_guid_missing" | "group_response_company_guid_mismatch",
        ) => anyhow::Error::new(
            PartyLedgerMasterSourceValidationError::GroupCompanyIdentityUnverified,
        ),
        error => anyhow::Error::new(error),
    }
}

/// The paired master request reached Tally successfully. Any parser failure
/// after that point is response validation, never endpoint reachability.
fn party_ledger_master_master_snapshot_error(source: anyhow::Error) -> anyhow::Error {
    anyhow::Error::new(PartyLedgerMasterSourceValidationError::MasterResponseInvalid { source })
}

fn party_ledger_master_openings_agree(
    master_opening: &str,
    balance_opening: &bridge_tally_core::ExactDecimal,
) -> anyhow::Result<bool> {
    let master_opening = bridge_tally_core::ExactDecimal::parse(master_opening.to_owned())?;
    Ok(master_opening.numeric_eq(balance_opening))
}

/// The paired sources answered successfully but cannot be reconciled into one
/// safe workbook source. This is distinct from endpoint or XML failure.
#[derive(Debug, thiserror::Error)]
pub(crate) enum PartyLedgerMasterSourceValidationError {
    #[error("Tally master ledger export period is unsupported")]
    MasterPeriod,
    #[error("Tally closing-balance period is unsupported")]
    BalancePeriod,
    #[error("Tally ledger master omitted GUID")]
    MasterGuid,
    #[error("Tally ledger master omitted MASTERID")]
    MasterId,
    #[error("Tally ledger master omitted ALTERID")]
    MasterAlterId,
    #[error("Tally ledger master omitted OPENINGBALANCE")]
    MasterOpeningBalance,
    #[error("Tally ledger master repeated a stable source identity")]
    DuplicateMasterIdentity,
    #[error("Tally balance snapshot omitted a ledger master")]
    BalanceMissingMasterLedger,
    #[error("Tally ledger opening balances disagreed across the paired sources")]
    OpeningBalancesDisagreed,
    #[error("Tally balance snapshot contained a ledger absent from master evidence")]
    BalanceLedgerAbsentFromMasterEvidence,
    #[error("Tally balance snapshot repeated a ledger display key")]
    DuplicateBalanceDisplayKey,
    #[error("Tally balance snapshot did not prove the selected company identity")]
    BalanceCompanyIdentityUnverified,
    #[error("Tally Group snapshot did not prove the selected company identity")]
    GroupCompanyIdentityUnverified,
    #[error("Tally party/ledger master response failed validation")]
    MasterResponseInvalid {
        #[source]
        source: anyhow::Error,
    },
    /// The company's master-alteration mark, an upper bound on its ledgers, is
    /// past the largest mark the census counts (#679), and the catalogue that
    /// would count the ledgers instead is estimated beyond the transport's
    /// response cap, so nothing was sent after the opening extent: the mark
    /// times the catalogue's bytes per ledger is over the limit. Numbers only.
    #[error("Tally ledger catalogue is estimated beyond Bridge's response limit")]
    CatalogueTooLarge {
        master_alter_id: u64,
        estimated_bytes: u64,
        limit_bytes: u64,
        /// The largest master mark the census can count.
        mark_limit: u64,
    },
    /// The ledger catalogue that counts a marked book's ledgers before its
    /// master read (#668) failed validation: another company, a damaged
    /// response or a duplicate identity. Nothing was sized from it.
    #[error("Tally ledger catalogue for the ledger count failed validation")]
    LedgerCountInvalid {
        #[source]
        source: StandardLedgerCatalogError,
    },
    /// A book too large to read whole (#668) could not be read as parts by
    /// immediate parent group (#679): a group too large for one part, too many
    /// parts, a ledger or group name the filter cannot carry, or a part whose
    /// rows are not the catalogue's. Nothing is released from a partial read.
    #[error("Tally compliance master read could not be split by parent group")]
    ParentPartition {
        #[source]
        source: ParentPartitionError,
    },
    /// A book read as several parts (#679) reads each part's balances at a
    /// different moment, so its balances only agree if no voucher was written
    /// meanwhile, and only the company's voucher high-water proves that. This
    /// Tally did not report one in its extent, so no part was requested.
    #[error("Tally did not report the voucher high-water a multi-part ledger read needs")]
    VoucherWitnessAbsent,
    /// The ledger census (#679), which counts a book whose master mark is past
    /// what its catalogue can be read for, refused: a slice held more rows than
    /// its span, one ledger was seen twice, no ledger was found, or the
    /// census stopped early. Nothing was sized from it.
    #[error("Tally ledger census could not count the book's ledgers")]
    LedgerSpan {
        #[source]
        source: LedgerCensusError,
    },
    /// The company's own count of its ledgers (`NUMLEDGERS`, #938) is higher
    /// than the census counted: the census missed ledgers (a ledger was added
    /// during the read or, by reasoning only, the company was closed and
    /// reopened while it ran), and a read sized from it would be sized too small.
    /// Nothing was requested after the company-count read. Numbers only.
    #[error("Tally's own ledger count is higher than the ledger census counted")]
    LedgerCountCompanyDiffers { company: u64, census: u64 },
    /// Tally's answer to the company ledger-count request (#938) was damaged,
    /// named another company or none of the loaded ones, or held a count that
    /// is not a plain number. Nothing was sized from it.
    #[error("Tally's answer to the company ledger-count request failed validation")]
    LedgerCountCompanyInvalid {
        #[source]
        source: OutstandingsError,
    },
    /// Tally's answer to the company ledger-count request (#938) ran past the
    /// transport's response cap (#1033), far more than one company's count can
    /// account for. The transport drops the connection with the rest of the
    /// response unread. Nothing was sized from it, and nothing after it was sent.
    #[error("Tally's answer to the company ledger-count request ran past ComplyEaze Bridge's response limit")]
    LedgerCountCompanyResponseTooLarge {
        #[source]
        source: anyhow::Error,
    },
    /// A slice of the ledger census (#679) failed validation: another company,
    /// a damaged response, a foreign field, or a ledger seen twice within the
    /// slice. Nothing was sized from it.
    #[error("Tally ledger census slice failed validation")]
    LedgerSpanSliceInvalid {
        #[source]
        source: StandardLedgerCatalogError,
    },
    /// One slice of the ledger census answered past the transport's response
    /// cap (#679), more than the slice's span can account for: Tally may have
    /// ignored the slice's filter. The transport drops the connection with the
    /// rest of the response unread. Nothing after that response was sent.
    #[error("Tally ledger census slice answered past Bridge's response limit")]
    LedgerSpanSliceResponseTooLarge {
        #[source]
        source: anyhow::Error,
    },
    /// Two counts of one book's ledgers disagreed (#679): the census, or the
    /// catalogue that counted them, against the ledgers the master read
    /// returned or against each other. The book changed under the read, or
    /// Tally answered one of them wrongly; nothing is released. Numbers only.
    #[error("Tally ledger counts disagreed")]
    LedgerCountDiffers { expected: u64, observed: u64 },
    /// The census counted ledgers the whole read cannot hold and the catalogue
    /// that would name their parents is estimated beyond Bridge's response
    /// limit (#679), so nothing was sent after the census. Numbers only.
    #[error("Tally ledger catalogue for the counted ledgers is beyond Bridge's response limit")]
    CountedCatalogueTooLarge {
        ledgers: u64,
        estimated_bytes: u64,
        limit_bytes: u64,
    },
    /// A parent part's answer passed the transport's response cap (#679):
    /// more than the catalogue can account for under the part's parents,
    /// possibly because Tally did not apply the part's filter. The transport
    /// error stays in the chain.
    #[error("Tally answered a parent part beyond Bridge's response limit")]
    ParentPartResponseTooLarge {
        #[source]
        source: anyhow::Error,
    },
}

impl PartyLedgerMasterSourceValidationError {
    /// A stable, data-free name for this refusal, safe to return to an agent.
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::MasterPeriod => "master_period_unsupported",
            Self::BalancePeriod => "balance_period_unsupported",
            Self::MasterGuid => "master_guid_missing",
            Self::MasterId => "master_id_missing",
            Self::MasterAlterId => "master_alter_id_missing",
            Self::MasterOpeningBalance => "master_opening_balance_missing",
            Self::DuplicateMasterIdentity => "duplicate_master_identity",
            Self::BalanceMissingMasterLedger => "balance_missing_master_ledger",
            Self::OpeningBalancesDisagreed => "opening_balances_disagreed",
            Self::BalanceLedgerAbsentFromMasterEvidence => {
                "balance_ledger_absent_from_master_evidence"
            }
            Self::DuplicateBalanceDisplayKey => "duplicate_balance_display_key",
            Self::BalanceCompanyIdentityUnverified => "balance_company_identity_unverified",
            Self::GroupCompanyIdentityUnverified => "group_company_identity_unverified",
            Self::MasterResponseInvalid { .. } => "master_response_invalid",
            Self::CatalogueTooLarge { .. } => "ledger_catalogue_too_large",
            Self::LedgerCountInvalid { source } => source.safe_code(),
            Self::ParentPartition { source } => source.safe_code(),
            Self::VoucherWitnessAbsent => "parent_partition_voucher_witness_absent",
            Self::ParentPartResponseTooLarge { .. } => "parent_part_response_too_large",
            Self::LedgerSpan { source } => source.safe_code(),
            Self::LedgerCountCompanyDiffers { .. } => "ledger_count_company_differs",
            Self::LedgerCountCompanyInvalid { .. } => "ledger_count_company_invalid",
            Self::LedgerCountCompanyResponseTooLarge { .. } => {
                "ledger_count_company_response_too_large"
            }
            Self::LedgerSpanSliceInvalid { source } => match source {
                StandardLedgerCatalogError::DuplicateIdentity => "ledger_span_duplicate_identity",
                StandardLedgerCatalogError::CompanyIdentityMismatch => {
                    "ledger_span_identity_mismatch"
                }
                StandardLedgerCatalogError::BoundsViolation => "ledger_span_slice_over_bound",
                // The census slice has no bill-wise flag to be missing, so the
                // flag errors cannot arrive here; they read as malformed.
                StandardLedgerCatalogError::MalformedResponse
                | StandardLedgerCatalogError::LedgerNameUnusable
                | StandardLedgerCatalogError::BillWiseFlagMissing
                | StandardLedgerCatalogError::BillWiseFlagInvalid
                | StandardLedgerCatalogError::BillWiseFlagRepeated => "ledger_span_slice_malformed",
            },
            Self::LedgerSpanSliceResponseTooLarge { .. } => "ledger_span_slice_response_too_large",
            Self::LedgerCountDiffers { .. } => "ledger_count_differs",
            Self::CountedCatalogueTooLarge { .. } => "ledger_count_catalogue_too_large",
        }
    }
}

/// Names a parent part's read that ran past the response cap as such, so the
/// caller learns the part's filter was probably ignored; a whole-book read has
/// no filter to blame and any other failure is left as it came.
fn parent_part_response_error(part: Option<&ParentPart>, error: anyhow::Error) -> anyhow::Error {
    let over_the_cap = error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<TallyTransportError>(),
            Some(TallyTransportError::ResponseTooLarge { .. })
        )
    });
    if part.is_some() && over_the_cap {
        // The read evidence a failed pair carries must stay the chain's root,
        // where the caller's `with_read_evidence` merges it.
        let completed = error
            .downcast_ref::<super::runtime::RuntimeReadFailure>()
            .map(|failure| failure.evidence.clone());
        let named = anyhow::Error::new(
            PartyLedgerMasterSourceValidationError::ParentPartResponseTooLarge { source: error },
        );
        match completed {
            Some(evidence) => with_read_evidence(named, evidence),
            None => named,
        }
    } else {
        error
    }
}

/// Bytes one ledger is estimated to add to the compliance master response
/// (#637). UNVERIFIED as a bound for real books: the only field observation is
/// a book of about 9,500 ledgers (2026-09-24) that read about 35.6 MB over
/// about 44 s before it was abandoned. That exceeds both the 32 MiB response
/// cap and the 20 s request deadline, so it spans more than one request, most
/// likely both halves of the paired master read. Attributing the whole 35.6 MB
/// to one master response (about 3.75 KB per ledger) overstates it, probably
/// about twofold. PARTIAL: a synthetic book of 1,989 ledgers, the parties
/// carrying every compliance field, read 2,875 bytes per ledger (5.7 MB,
/// 0.9 s; 2026-09-29), so 3,750 is a budget above that book's mean, not a bound
/// on a row: party rows cost 3.2 to 3.3 KB there.
const COMPLIANCE_MASTER_BYTES_PER_LEDGER_UNVERIFIED: u64 = 3_750;

/// The largest estimated compliance master response Bridge will request
/// (#637). UNVERIFIED: 0.8 MB/s is that book's 35.6 MB over about 44 s,
/// averaged over more than one request, so 16 MB is about one 20 s request at
/// that average, right at the deadline. A basic ledger read of about 22 MB
/// completed on the same book in 7-11 s.
///
/// One constant with the masters read's budget, in the protocol crate.
const COMPLIANCE_MASTER_RESPONSE_BUDGET_BYTES_UNVERIFIED: u64 =
    bridge_tally_protocol::native_masters::MASTERS_RESPONSE_BUDGET_BYTES as u64;

/// Bytes one ledger is estimated to add to the balance-free ledger catalogue
/// response that counts a marked book's ledgers (#668). PARTIAL: a synthetic
/// book of 1,989 ledgers read 1,104 bytes per ledger (2.2 MB, 0.35 s;
/// 2026-09-29) and a real book of about 9,500 ledgers read about 1,221 (about
/// 11.6 MB in about 1.5 s), so 1,400 leaves about 15% over the larger.
const LEDGER_CATALOGUE_BYTES_PER_LEDGER_PARTIAL: u64 = 1_400;

/// The largest estimated catalogue response Bridge will request (#679). A
/// catalogue is not read in parts, and a response over the transport's cap
/// (`XML_RESPONSE_MAX_BYTES`, 32 MiB) makes the transport return as soon as the
/// running total passes it, dropping the connection with the rest of the
/// response unread: an abandoned read, which can leave Tally's gateway busy.
/// So the catalogue is bounded before it is sent, not left to the cap. 32 MB
/// is under the cap by about 1.5 MB. UNVERIFIED as a margin: it is Bridge's
/// own choice. At 1,400 bytes per unit of mark it admits a mark of 22,857.
const LEDGER_CATALOGUE_RESPONSE_LIMIT_BYTES_UNVERIFIED: u64 = 32_000_000;

/// AlterIDs one census slice spans (#679), hence the most ledgers it may
/// return. UNVERIFIED as a size bound: chosen so that a slice of ledgers with
/// the longest name Bridge assumes fits [`COMPLIANCE_MASTER_RESPONSE_BUDGET_BYTES_UNVERIFIED`]
/// (asserted by a test), which is about half the transport's cap. Measured at
/// 800, 1,000, 4,000 and 8,000 wide, on three books (section 11e, PARTIAL).
const LEDGER_CENSUS_SLICE_WIDTH_UNVERIFIED: u64 = 4_000;

/// Most slices one census makes (#679): Bridge's own bound on the serial
/// requests a census may spend, not a measured limit. With the slice width it
/// admits a master mark of 400,000, so a book whose mark is above that is
/// refused before anything is sent after the extent.
const LEDGER_CENSUS_MAX_SLICES_UNVERIFIED: usize = 100;

/// Characters of a GUID-only census row that do not depend on the ledger's
/// name, counting the row's separators: 391 in the committed eight-row capture
/// (a row is 2 x name + about 390 characters). An earlier run whose rows are not
/// committed averaged 846 bytes a row, 423 characters, with names whose length
/// was not recorded. 415 is above the capture's figure, so a change that
/// lowered it below the capture fails a test; it is a margin, not a
/// measurement. PARTIAL: one capture, and no row with aliases.
const LEDGER_CENSUS_ROW_FIXED_CHARS_PARTIAL: u64 = 415;

/// The longest ledger name (in characters) a census row is assumed to carry.
/// UNVERIFIED: the longest name observed on any book is 88 characters (#917
/// tracks measuring it); Tally's own limit is not known to be lower. A ledger
/// with aliases carries each alias in the row too, which this does not bound.
const LEDGER_CENSUS_NAME_CHARS_UNVERIFIED: u64 = 128;

/// The bytes of the largest census row assumed: the row's fixed characters plus
/// the name twice, each character at most six characters when escaped
/// (`&#NNN;`), all in UTF-16 (2 bytes a character).
const fn ledger_census_worst_row_bytes() -> u64 {
    2 * (LEDGER_CENSUS_ROW_FIXED_CHARS_PARTIAL + 12 * LEDGER_CENSUS_NAME_CHARS_UNVERIFIED)
}

// A full slice of the longest names assumed fits the budget the master read is
// held to (#679): raising the width or the assumed name length past it does not
// compile.
const _: () = assert!(
    LEDGER_CENSUS_SLICE_WIDTH_UNVERIFIED * ledger_census_worst_row_bytes()
        <= COMPLIANCE_MASTER_RESPONSE_BUDGET_BYTES_UNVERIFIED
);

fn ledger_census_limits() -> CensusLimits {
    CensusLimits {
        slice_width: LEDGER_CENSUS_SLICE_WIDTH_UNVERIFIED,
        max_slices: LEDGER_CENSUS_MAX_SLICES_UNVERIFIED,
    }
}

/// The largest master mark a census covers.
const fn ledger_census_mark_limit() -> u64 {
    LEDGER_CENSUS_SLICE_WIDTH_UNVERIFIED * LEDGER_CENSUS_MAX_SLICES_UNVERIFIED as u64
}

/// Most immediate parent groups one part's `$Parent = "A" OR $Parent = "B"`
/// formula names (#679). Measured on a synthetic book of 4,339 ledgers: an `OR`
/// of 1, 8, 50 and 200 parents answered in 0.1 to 0.6 s with every row
/// returned. 200 is the most measured; above it nothing is measured and a
/// longer formula may not be answered, so 200 is the cap and it is UNVERIFIED
/// above 200. With `PARENT_PART_MAX_PARTS_UNVERIFIED` parts that bounds a book
/// at 2,400 parents (typed `parent_partition_too_many_parts` above that), and a
/// book has about one parent per ten ledgers, so a book of about 1,350 parents
/// fits in seven parts.
const PARENT_PART_MAX_PARENTS_UNVERIFIED: usize = 200;

/// The ceiling on one parent group is the whole-read bound: a group with more
/// ledgers than one part may carry (4,266) is refused as `parent_over_budget`,
/// never split, because a group cannot be read in pieces by `$Parent`.
///
/// Most parts one book may be read as (#679). UNVERIFIED: Bridge's own bound on
/// the serial requests one call may spend (four per part), not a measured
/// limit. It admits about 51,000 ledgers, far past what the mark bound admits.
const PARENT_PART_MAX_PARTS_UNVERIFIED: usize = 12;

/// Most bytes of `NOT` formula text the complement part's request may carry
/// (#679). UNVERIFIED: Bridge's own bound. One live probe sent 54 KB of
/// request for 1,354 parents; the 12-part, 200-parents-per-part limits already
/// cap the parents at 2,400, so this is a backstop against very long names.
const PARENT_COMPLEMENT_MAX_FORMULA_BYTES_UNVERIFIED: usize = 262_144;

/// The limits a book's parent parts must fit (#679): each part's estimated
/// master response is inside the same budget a whole read is admitted by, so
/// the per-part ledger bound is the whole-read bound.
fn parent_partition_limits() -> PartitionLimits {
    PartitionLimits {
        max_ledgers_per_part: COMPLIANCE_MASTER_RESPONSE_BUDGET_BYTES_UNVERIFIED
            / COMPLIANCE_MASTER_BYTES_PER_LEDGER_UNVERIFIED,
        max_parents_per_part: PARENT_PART_MAX_PARENTS_UNVERIFIED,
        max_parts: PARENT_PART_MAX_PARTS_UNVERIFIED,
        max_complement_formula_bytes: PARENT_COMPLEMENT_MAX_FORMULA_BYTES_UNVERIFIED,
    }
}

/// A compliance master response estimate for a ledger count, or an upper
/// bound on one, and whether it
/// fits the budget. An estimate exactly at the budget fits: the budget is the
/// largest response Bridge will request, not the first it refuses. Takes its
/// figures as arguments so the boundary is tested exactly, whatever the
/// measured constants become.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ComplianceEstimate {
    pub(crate) estimated_bytes: u64,
    pub(crate) fits: bool,
}

/// Also sizes the masters read (`runtime_masters.rs`), which shares this
/// budget.
pub(crate) fn compliance_estimate(
    count: u64,
    bytes_per_ledger: u64,
    budget_bytes: u64,
) -> ComplianceEstimate {
    let estimated_bytes = count.saturating_mul(bytes_per_ledger);
    ComplianceEstimate {
        estimated_bytes,
        fits: estimated_bytes <= budget_bytes,
    }
}

/// [`compliance_estimate`] at the two UNVERIFIED constants.
fn compliance_estimate_unverified(count: u64) -> ComplianceEstimate {
    compliance_estimate(
        count,
        COMPLIANCE_MASTER_BYTES_PER_LEDGER_UNVERIFIED,
        COMPLIANCE_MASTER_RESPONSE_BUDGET_BYTES_UNVERIFIED,
    )
}

/// What the census's count was checked against (#938), kept so a reader can see
/// whether the check ran. The company's own count never admits or sizes a read:
/// it can only refuse a census that counted fewer ledgers than Tally says the
/// company holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CountCrossCheck {
    /// Tally's own count equals the census's.
    Matched,
    /// Tally's own count is below the census's: not the hazardous direction,
    /// so the read goes on (the other direction is checked by the count against
    /// the rows the read returns).
    CompanyCountLower,
    /// The company's answer carried no ledger count, so the check did not run.
    Unavailable,
}

impl CountCrossCheck {
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::CompanyCountLower => "company_count_lower",
            Self::Unavailable => "unavailable",
        }
    }
}

/// What one master-and-balance pair of a compliance read covers: the whole
/// book, with the number of ledgers a count already said it holds (#679), or
/// one parent part, whose own ledger count is checked instead.
#[derive(Clone, Copy)]
enum PartyLedgerRead<'a> {
    Whole { expected_ledgers: Option<u64> },
    Part(&'a ParentPart),
}

/// What sizing a compliance master read decided before any ledger request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComplianceAdmission {
    Admitted,
    /// The mark does not fit whole but its catalogue can be read: count the
    /// ledgers, then admit again with the count (#668).
    CountFirst,
    /// The mark is past what the catalogue can be read for, but within what
    /// the census covers: count the ledgers by AlterID span (#679), then admit
    /// again with the count.
    CensusFirst,
    /// The counted ledgers do not fit one read: read them in parts by parent
    /// group (#679).
    InParts,
}

/// Sizes a compliance read before any master request is sent (#637, #668, #679).
///
/// With `counted` ledgers the count decides: the estimate is `counted` times
/// the per-ledger constant, and a count that does not fit is read in parts.
/// Without one, the company's master-alteration mark (`ALTMSTID`, from the
/// opening extent; the extent read already fails closed without it,
/// `require_master_witness`) stands in: it fits, or the catalogue that counts
/// the book fits the transport ([`ComplianceAdmission::CountFirst`]), or the
/// census can count it by AlterID span ([`ComplianceAdmission::CensusFirst`]),
/// or the read is refused before anything is sent after the extent.
///
/// The mark is an UPPER BOUND on ledgers, not a count: every master of every
/// type (stock items, units, groups and the rest) raises it, and so does every
/// alteration. It bounds the ledger count on the assumption that every ledger
/// holds its own distinct `ALTERID`, no greater than the mark; deletions only
/// loosen it. PARTIAL: that held on every captured company with both an extent
/// and a ledger capture (8 small lab companies, 6-88 ledgers against marks of
/// 213-328, not captured at the same moment; and a synthetic book of 1,989
/// ledgers against a mark of 2,197), which is not a proof. If the assumption
/// is ever false, a book this admits is read as it was before #637, and the
/// census (which covers `(0, mark]` only) undercounts: the count is compared
/// with the ledgers the read returns, and a difference is refused
/// (`ledger_count_differs`).
fn admit_compliance_master_read(
    master_alter_id: u64,
    counted: Option<u64>,
) -> Result<ComplianceAdmission, PartyLedgerMasterSourceValidationError> {
    if compliance_estimate_unverified(counted.unwrap_or(master_alter_id)).fits {
        return Ok(ComplianceAdmission::Admitted);
    }
    if counted.is_some() {
        return Ok(ComplianceAdmission::InParts);
    }
    let catalogue = compliance_estimate(
        master_alter_id,
        LEDGER_CATALOGUE_BYTES_PER_LEDGER_PARTIAL,
        LEDGER_CATALOGUE_RESPONSE_LIMIT_BYTES_UNVERIFIED,
    );
    if catalogue.fits {
        return Ok(ComplianceAdmission::CountFirst);
    }
    if master_alter_id <= ledger_census_mark_limit() {
        return Ok(ComplianceAdmission::CensusFirst);
    }
    Err(PartyLedgerMasterSourceValidationError::CatalogueTooLarge {
        master_alter_id,
        estimated_bytes: catalogue.estimated_bytes,
        limit_bytes: LEDGER_CATALOGUE_RESPONSE_LIMIT_BYTES_UNVERIFIED,
        mark_limit: ledger_census_mark_limit(),
    })
}

/// Whether the catalogue that names a counted book's parents fits the
/// transport (#679): the census counted more ledgers than one read holds, so
/// the parts are planned from the named catalogue, and that catalogue is bounded
/// by the count instead of by the mark. A refusal here sends nothing more.
fn admit_counted_catalogue(ledgers: u64) -> Result<(), PartyLedgerMasterSourceValidationError> {
    let catalogue = compliance_estimate(
        ledgers,
        LEDGER_CATALOGUE_BYTES_PER_LEDGER_PARTIAL,
        LEDGER_CATALOGUE_RESPONSE_LIMIT_BYTES_UNVERIFIED,
    );
    if catalogue.fits {
        return Ok(());
    }
    Err(
        PartyLedgerMasterSourceValidationError::CountedCatalogueTooLarge {
            ledgers,
            estimated_bytes: catalogue.estimated_bytes,
            limit_bytes: LEDGER_CATALOGUE_RESPONSE_LIMIT_BYTES_UNVERIFIED,
        },
    )
}

/// Names a census slice's read that ran past the response cap as such, so the
/// caller learns the slice's filter was probably ignored: a slice holds at most
/// its width in ledgers, far under the cap. Any other failure is left as it
/// came.
fn ledger_span_response_error(error: anyhow::Error) -> anyhow::Error {
    named_past_the_cap(error, |source| {
        PartyLedgerMasterSourceValidationError::LedgerSpanSliceResponseTooLarge { source }
    })
}

/// Names the company ledger-count read (#938) that ran past the response cap
/// as such (#1033), so the refusal carries its own cause rather than none. Any
/// other failure is left as it came.
fn company_count_response_error(error: anyhow::Error) -> anyhow::Error {
    named_past_the_cap(error, |source| {
        PartyLedgerMasterSourceValidationError::LedgerCountCompanyResponseTooLarge { source }
    })
}

/// `error` wrapped by `name` when the transport refused the response as past
/// its cap; any other failure unchanged.
fn named_past_the_cap(
    error: anyhow::Error,
    name: fn(anyhow::Error) -> PartyLedgerMasterSourceValidationError,
) -> anyhow::Error {
    let over_the_cap = error.chain().any(|cause| {
        matches!(
            cause.downcast_ref::<TallyTransportError>(),
            Some(TallyTransportError::ResponseTooLarge { .. })
        )
    });
    if over_the_cap {
        anyhow::Error::new(name(error))
    } else {
        error
    }
}

/// A paired or bracketed read observed movement in the endpoint's data. This
/// is response validation, not an endpoint failure: Tally answered, but
/// Bridge must withhold the unstable result.
#[derive(Debug, thiserror::Error)]
pub(crate) enum PairedReadValidationError {
    #[error("Tally native ledger collection changed between paired reads")]
    NativeLedgerCollection,
    #[error("Tally company book changed during native ledger read")]
    NativeLedgerExtent,
    #[error("Tally group hierarchy changed between paired reads beside a ledger read")]
    NativeLedgerGroup,
    #[error("Tally ledger master changed between paired reads")]
    PartyLedgerMaster,
    #[error("Tally ledger balances changed between paired reads")]
    PartyLedgerBalance,
    #[error("Tally group hierarchy changed between paired reads")]
    PartyLedgerGroup,
    #[error("Tally ledger catalogue changed between paired reads")]
    PartyLedgerCatalogue,
    #[error("Tally company book changed during party/ledger master read")]
    PartyLedgerExtent,
    #[error("Tally company book extent changed between paired reads")]
    CompanyBookExtent,
    #[error("Tally currency masters changed between paired reads")]
    CurrencyMaster,
    #[error("Tally company currency name changed between paired reads")]
    CompanyCurrencyName,
    #[error("Tally's own statement changed between paired reads")]
    NativeStatement,
    #[error("Tally company book changed during currency detection")]
    CurrencyExtent,
    #[error("Tally company changed between the currency read and the master read")]
    CurrencyToMasterExtent,
    #[error("Tally masters collection changed between paired reads")]
    MastersCollection,
    #[error("Tally company book changed during masters read")]
    MastersExtent,
    #[error("Tally stock collection changed between paired reads")]
    StockSummaryCollection,
    #[error("Tally company book changed during stock summary read")]
    StockSummaryExtent,
}

impl PairedReadValidationError {
    /// A stable, data-free name for this refusal, safe to return to an agent.
    pub(crate) fn safe_code(&self) -> &'static str {
        match self {
            Self::NativeLedgerCollection => "native_ledger_collection_changed",
            Self::NativeLedgerExtent => "native_ledger_extent_changed",
            Self::NativeLedgerGroup => "native_ledger_group_changed",
            Self::PartyLedgerMaster => "party_ledger_master_changed",
            Self::PartyLedgerBalance => "party_ledger_balance_changed",
            Self::PartyLedgerGroup => "party_ledger_group_changed",
            Self::PartyLedgerCatalogue => "party_ledger_catalogue_changed",
            Self::PartyLedgerExtent => "party_ledger_extent_changed",
            Self::CompanyBookExtent => "company_book_extent_changed",
            Self::CurrencyMaster => "currency_master_changed",
            Self::CompanyCurrencyName => "company_currency_name_changed",
            Self::NativeStatement => "native_statement_changed",
            Self::CurrencyExtent => "currency_extent_changed",
            Self::CurrencyToMasterExtent => "currency_to_master_extent_changed",
            Self::MastersCollection => "masters_collection_changed",
            Self::MastersExtent => "masters_extent_changed",
            Self::StockSummaryCollection => "stock_summary_collection_changed",
            Self::StockSummaryExtent => "stock_summary_extent_changed",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum DirectCompanyBootstrapError {
    #[error("Tally direct company identity did not match its enumerated candidate")]
    CandidateGuidMismatch,
    #[error("Tally direct company candidate omitted a complete identity tuple")]
    IncompleteTuple,
}

#[cfg(feature = "voucher-scan")]
#[derive(Debug)]
pub(crate) struct OutstandingsSegmentObservation {
    pub(crate) verification: SegmentVerification,
    pub(crate) first_read_elapsed: Duration,
    pub(crate) second_read_elapsed: Duration,
}

#[cfg(feature = "voucher-scan")]
pub(crate) enum LedgerOpeningCoverageRead {
    Stable(LedgerOpeningCoverage),
    Drifted,
}

/// An HTTP response entity kept byte for byte, with its decoded text.
#[derive(Debug, Clone)]
pub(crate) struct RawTallyResponse {
    pub(crate) text: String,
    pub(crate) encoded_body: Vec<u8>,
    pub(crate) encoded_sha256: String,
}

/// A Tally client holding its endpoint's wire lock for exactly one send (#697):
/// the import POST, whose attempt is recorded between taking the lock and
/// sending. Spending it on that send, or dropping it, releases the lock.
pub(super) struct WireHeldClient<'a> {
    client: &'a TallyClient,
    wire: WireHeldTransport<'a>,
}

impl WireHeldClient<'_> {
    /// [`TallyClient::post_probe_xml`] under the held lock: one send, which
    /// spends it. It does not wait for the lock again.
    pub(super) async fn post_probe_xml(
        self,
        xml: String,
        evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<String> {
        let response = self.wire.post_xml_decoded(xml).await?;
        self.client.probe_response(response, evidence)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Tally native report changed between paired reads")]
pub(crate) struct NativeReportPairDrift;

impl NativeReportPairDrift {
    /// A stable, data-free name for this refusal, safe to return to an agent.
    pub(crate) const SAFE_CODE: &'static str = "native_report_pair_changed";
}

/// Tags a failure as coming from one of the two POST responses inside
/// `fetch_native_report_paired_with_evidence` -- the paired native report
/// request itself, never the health checks bracketing it or any stage
/// outside this function. A catalogue read's classifier applies its
/// bounds/malformed split only when this marker is present in the error
/// chain; everything untagged -- the identity bracket, both health checks,
/// and any stage added later -- falls back to the conservative `Transport`
/// code by default. That inversion is deliberate: tagging every non-response
/// stage is unbounded (there is always another stage to remember), while a
/// positive marker on the one response that is actually being classified
/// means a stage added later inherits the safe code automatically instead of
/// silently inheriting a confidently wrong one.
///
/// Deliberately `transparent`: `tally_runtime_command_error` classifies some
/// failures by substring-matching `error.to_string()`, which is the *top-level*
/// message, so any wrapper with a message of its own silently rewrites how
/// every reader's errors classify. A message naming this stage would have
/// contained "report", whose "port" substring routes straight to
/// `endpoint_configuration_invalid` -- telling operators their endpoint is
/// misconfigured when a connection merely dropped. Forwarding Display leaves
/// every existing message byte-identical and adds only a type to downcast to.
#[derive(Debug)]
pub(crate) struct PairedNativeReportResponseFailure(anyhow::Error);

// Display and Error are written out rather than derived because this marker has
// to be invisible in two different ways at once, and no single derive gives
// both.
//
// Display forwards: `tally_runtime_command_error` classifies some failures by
// substring-matching `error.to_string()`, which is the *top-level* message, so a
// marker with a message of its own silently rewrites how every reader's errors
// classify. A message naming this stage would have contained "report", whose
// "port" substring routes to `endpoint_configuration_invalid` -- blaming the
// endpoint configuration for a dropped connection.
//
// `source` returns the wrapped error's own head rather than delegating to its
// source. `#[error(transparent)]` would delegate, which skips the head and hides
// the `TallyTransportError` from everything that downcasts while walking the
// chain. Returning the head keeps the chain exactly as it was, with this marker
// inserted ahead of it rather than replacing anything.
impl std::fmt::Display for PairedNativeReportResponseFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::error::Error for PairedNativeReportResponseFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}

impl PairedNativeReportResponseFailure {
    /// Only constructor: forces every paired-report response failure through
    /// this one marking point rather than each call site improvising its own
    /// wrap. `pub(crate)` so tests outside this module can construct a
    /// tagged failure directly rather than driving a real request.
    pub(crate) fn new(error: anyhow::Error) -> Self {
        Self(error)
    }

    /// The `TallyTransportError` behind this marker, if any. Looked up
    /// through the wrapped `anyhow::Error`'s own chain rather than the outer
    /// error's chain: nesting an `anyhow::Error` behind `#[source]` does not
    /// expose the wrapped error as its own link when walked through
    /// `std::error::Error::source()`, only through `anyhow::Error::chain()`
    /// on that inner value directly.
    pub(crate) fn transport_error(&self) -> Option<&TallyTransportError> {
        self.0
            .chain()
            .find_map(|cause| cause.downcast_ref::<TallyTransportError>())
    }
}

/// Outcome of a paired native-report read. `Drifted` means the two reads
/// disagreed, so the book moved between them and no total may be reported.
pub(crate) enum NativePairedRead {
    Stable {
        body: String,
        encoded_bytes: usize,
        encoded_sha256: String,
    },
    Drifted(RuntimeReadEvidence),
}

impl NativePairedRead {
    pub(crate) fn require_stable(
        self,
        error: PairedReadValidationError,
    ) -> anyhow::Result<(String, usize, String)> {
        match self {
            Self::Stable {
                body,
                encoded_bytes,
                encoded_sha256,
            } => Ok((body, encoded_bytes, encoded_sha256)),
            Self::Drifted(evidence) => {
                Err(super::runtime::with_read_evidence(error.into(), evidence))
            }
        }
    }
}

#[cfg(feature = "voucher-scan")]
struct OutstandingsWireResponse {
    text: String,
    encoded_bytes: usize,
    encoded_sha256: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum TallyProduct {
    TallyPrime,
    #[serde(rename = "Tally ERP 9")]
    TallyErp9,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionStatus {
    pub reachable: bool,
    pub compatible: bool,
    pub server_text: String,
    pub product: TallyProduct,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TallyProbeResult {
    pub connection: ConnectionStatus,
    pub companies: Vec<TallyCompany>,
    pub profile: CapabilityProfile,
    pub passport_snapshot_id: Option<String>,
}

struct GatewayProductModeEvidence {
    product: String,
    release: Option<String>,
    license_tier: Option<LicenseTier>,
    mode: Option<String>,
    capability: CapabilityEvidence,
}

impl GatewayProductModeEvidence {
    fn unavailable() -> Self {
        Self {
            product: "Unknown".to_string(),
            release: None,
            license_tier: None,
            mode: None,
            capability: CapabilityEvidence {
                state: CapabilityState::Unknown,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: Some("product_mode_evidence_unavailable".to_string()),
            },
        }
    }

    fn from_observation(
        observation: bridge_tally_protocol::CompanyGatewayCapabilityObservation,
    ) -> Self {
        // Observed live on 22 Sep 2026 (a TallyPrime 7.1 lab instance in
        // Education): `EDUMODE=Yes` with `SILVER=Yes` and `GOLD=No`. Education
        // still reports Silver, so the mode is read from `EDUMODE` first, and
        // no licence tier is inferred when it is set (bridge#581).
        let mode = if observation.educational_mode {
            Some("Education".to_string())
        } else if observation.silver || observation.gold {
            Some("Licensed".to_string())
        } else {
            None
        };
        let capability = if mode.is_some() {
            CapabilityEvidence {
                state: CapabilityState::Supported,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: None,
            }
        } else {
            CapabilityEvidence {
                state: CapabilityState::Unknown,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: Some("license_mode_not_established".to_string()),
            }
        };
        let license_tier = match (
            observation.educational_mode,
            observation.silver,
            observation.gold,
        ) {
            (false, true, false) => Some(LicenseTier::Silver),
            (false, false, true) => Some(LicenseTier::Gold),
            _ => None,
        };
        Self {
            product: observation.product,
            release: observation.release,
            license_tier,
            mode,
            capability,
        }
    }
}

#[derive(Clone)]
pub struct TallyClient {
    config: TallyConfig,
    http: TallyHttpTransport,
    observed_body_bytes: Arc<AtomicU64>,
    observed_encoding: Arc<AtomicU8>,
}

const BODY_BYTES_UNAVAILABLE: u64 = u64::MAX;
const ENCODING_UNAVAILABLE: u8 = 0;
const ENCODING_UTF8: u8 = 1;
const ENCODING_UTF8_BOM: u8 = 2;
const ENCODING_UTF16_LE: u8 = 3;
const ENCODING_UTF16_LE_BOM: u8 = 4;
const ENCODING_UTF16_BE_BOM: u8 = 5;

impl TallyClient {
    /// Every send takes the endpoint's wire lock (`endpoint_wire`), at the
    /// shared per-user coordination root.
    pub fn new(config: TallyConfig) -> anyhow::Result<Self> {
        Self::with_wire(config, &WireGateConfig::default())
    }

    /// As [`Self::new`], gating every send on `wire`'s root and retry bound.
    pub(crate) fn with_wire(config: TallyConfig, wire: &WireGateConfig) -> anyhow::Result<Self> {
        let http = TallyHttpTransport::new(config.clone())?
            .with_wire_gate(wire.gate_for(&config), wire.retry())
            .with_send_observer(crate::request_trail::observer());
        Ok(Self {
            config,
            http,
            observed_body_bytes: Arc::new(AtomicU64::new(BODY_BYTES_UNAVAILABLE)),
            observed_encoding: Arc::new(AtomicU8::new(ENCODING_UNAVAILABLE)),
        })
    }

    /// A clone for one runtime operation (#697 item (a)): every send it and
    /// its clones make draws on `budget` for the wire lock, so the operation
    /// waits at most that long however many sends it makes. The observation
    /// counters stay shared, as for any clone.
    pub(crate) fn for_operation(&self, budget: bridge_tally_transport::WireWaitBudget) -> Self {
        Self {
            http: self.http.for_operation(budget),
            ..self.clone()
        }
    }

    pub fn canonical_origin(&self) -> anyhow::Result<String> {
        canonical_loopback_origin(&self.config)
    }

    #[cfg(test)]
    fn with_http_builder(config: TallyConfig, builder: reqwest::ClientBuilder) -> Self {
        let wire = WireGateConfig::default();
        let http = TallyHttpTransport::with_builder(
            config.clone(),
            bridge_tally_transport::TransportPolicy::default(),
            builder,
        )
        .expect("build synthetic Tally HTTP transport")
        .with_wire_gate(wire.gate_for(&config), wire.retry())
        .with_send_observer(crate::request_trail::observer());
        Self {
            config,
            http,
            observed_body_bytes: Arc::new(AtomicU64::new(BODY_BYTES_UNAVAILABLE)),
            observed_encoding: Arc::new(AtomicU8::new(ENCODING_UNAVAILABLE)),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_transport_policy(
        config: TallyConfig,
        policy: bridge_tally_transport::TransportPolicy,
        wire: &WireGateConfig,
    ) -> anyhow::Result<Self> {
        let http =
            TallyHttpTransport::with_builder(config.clone(), policy, reqwest::Client::builder())?
                .with_wire_gate(wire.gate_for(&config), wire.retry())
                .with_send_observer(crate::request_trail::observer());
        Ok(Self {
            config,
            http,
            observed_body_bytes: Arc::new(AtomicU64::new(BODY_BYTES_UNAVAILABLE)),
            observed_encoding: Arc::new(AtomicU8::new(ENCODING_UNAVAILABLE)),
        })
    }

    pub async fn check_connection(&self) -> anyhow::Result<ConnectionStatus> {
        match self.check_connection_strict().await {
            Ok(status) => Ok(status),
            Err(error) => Ok(ConnectionStatus {
                reachable: false,
                compatible: false,
                server_text: String::new(),
                product: TallyProduct::Unknown,
                error: Some(safe_connection_failure_code(&error).to_string()),
            }),
        }
    }

    pub(crate) async fn check_connection_strict(&self) -> anyhow::Result<ConnectionStatus> {
        self.check_connection_strict_with_wire_evidence()
            .await
            .map(|(status, _)| status)
    }

    async fn check_connection_strict_with_wire_evidence(
        &self,
    ) -> anyhow::Result<(ConnectionStatus, RuntimeReadEvidence)> {
        let response = self.http.get_status_decoded().await?;
        let wire_evidence = RuntimeReadEvidence {
            // GET /status has an empty request body. This is a body commitment,
            // matching POST request_body_sha256, not an invented operation label.
            request_sha256: Sha256::digest([])
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect(),
            response_sha256: response.encoded_sha256().to_string(),
            bytes: response.encoded_bytes(),
        };
        self.record_observed_body_bytes(response.encoded_bytes());
        self.record_observed_encoding(response.encoding());
        let response_text = response.into_text();
        let product = detect_product(&response_text);
        let compatible = matches!(product, TallyProduct::TallyPrime | TallyProduct::TallyErp9);
        let server_text = match product {
            TallyProduct::TallyPrime => "TallyPrime Server is Running",
            TallyProduct::TallyErp9 => "Tally ERP 9 Server is Running",
            TallyProduct::Unknown => "Endpoint responded with an unrecognized status document",
        };
        Ok((
            ConnectionStatus {
                reachable: true,
                compatible,
                product,
                server_text: server_text.to_string(),
                error: None,
            },
            wire_evidence,
        ))
    }

    pub async fn probe(&self) -> anyhow::Result<TallyProbeResult> {
        self.probe_with_wire_evidence()
            .await
            .map(|(probe, _)| probe)
    }

    pub(crate) async fn probe_with_wire_evidence(
        &self,
    ) -> anyhow::Result<(TallyProbeResult, RuntimeReadEvidence)> {
        // `/status` is useful local diagnostics but is not part of Tally's
        // documented third-party XML contract. Never gate the POST probe or
        // authoritative product metadata on this unauthenticated heuristic.
        let (mut connection, mut wire_evidence) =
            match self.check_connection_strict_with_wire_evidence().await {
                Ok(observation) => observation,
                Err(error) => (
                    ConnectionStatus {
                        reachable: false,
                        compatible: false,
                        server_text: String::new(),
                        product: TallyProduct::Unknown,
                        error: Some(safe_connection_failure_code(&error).to_string()),
                    },
                    RuntimeReadEvidence::empty(),
                ),
            };
        let mut transports = BTreeMap::new();
        let mut features = BTreeMap::new();
        let mut packs = BTreeMap::new();
        let mut companies = Vec::new();

        let mut gateway_product_mode = GatewayProductModeEvidence::unavailable();
        let xml_evidence = self
            .company_discovery_evidence(
                &mut connection,
                &mut companies,
                &mut gateway_product_mode,
                &mut wire_evidence,
            )
            .await
            .map_err(|error| super::runtime::with_read_evidence(error, wire_evidence.clone()))?;
        transports.insert(TransportId::XmlHttp, xml_evidence.clone());
        transports.insert(
            TransportId::JsonEx,
            CapabilityEvidence {
                state: CapabilityState::Unknown,
                confidence: EvidenceConfidence::Unknown,
                safe_reason_code: Some("transport_not_probed".to_string()),
            },
        );
        for transport in [TransportId::TdlCompanion, TransportId::Odbc] {
            transports.insert(
                transport,
                CapabilityEvidence {
                    state: CapabilityState::Unknown,
                    confidence: EvidenceConfidence::Unknown,
                    safe_reason_code: Some("configuration_not_observed".to_string()),
                },
            );
        }

        features.insert(
            CapabilityFeatureId::EndpointReachability,
            CapabilityEvidence {
                state: CapabilityState::Supported,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: Some("xml_endpoint_responded".to_string()),
            },
        );
        features.insert(
            CapabilityFeatureId::ProductAndMode,
            gateway_product_mode.capability.clone(),
        );
        let empty_company_reason = || {
            if xml_evidence.state == CapabilityState::Supported {
                "company_not_loaded".to_string()
            } else {
                xml_evidence
                    .safe_reason_code
                    .clone()
                    .unwrap_or_else(|| "company_list_not_established".to_string())
            }
        };
        let company_state = if companies.is_empty() {
            CapabilityEvidence {
                state: if xml_evidence.state == CapabilityState::Supported {
                    CapabilityState::NotConfigured
                } else {
                    CapabilityState::Unknown
                },
                confidence: xml_evidence.confidence,
                safe_reason_code: Some(empty_company_reason()),
            }
        } else {
            CapabilityEvidence {
                state: CapabilityState::Supported,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: Some("loaded_company_observed".to_string()),
            }
        };
        features.insert(CapabilityFeatureId::LoadedCompanies, company_state);
        let identity_evidence = if companies.is_empty() {
            CapabilityEvidence {
                state: if xml_evidence.state == CapabilityState::Supported {
                    CapabilityState::NotConfigured
                } else {
                    CapabilityState::Unknown
                },
                confidence: xml_evidence.confidence,
                safe_reason_code: Some(empty_company_reason()),
            }
        } else if has_presentation_equivalent_guid_siblings(&companies) {
            CapabilityEvidence {
                state: CapabilityState::Unknown,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: Some("company_identity_display_scope_ambiguous".to_string()),
            }
        } else if unique_company_identities(&companies) {
            CapabilityEvidence {
                state: CapabilityState::Supported,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: Some("stable_company_identity_observed".to_string()),
            }
        } else if companies.iter().all(has_complete_company_identity) {
            CapabilityEvidence {
                state: CapabilityState::Unknown,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: Some("company_identity_ambiguous".to_string()),
            }
        } else {
            CapabilityEvidence {
                state: CapabilityState::Unknown,
                confidence: EvidenceConfidence::Observed,
                safe_reason_code: Some("stable_company_identity_not_observed".to_string()),
            }
        };
        features.insert(
            CapabilityFeatureId::StableCompanyIdentity,
            identity_evidence,
        );
        features.insert(
            CapabilityFeatureId::EncodingBehaviour,
            self.observed_encoding_evidence(),
        );
        features.insert(
            CapabilityFeatureId::PracticalResponseLimit,
            CapabilityEvidence {
                state: CapabilityState::Unknown,
                confidence: EvidenceConfidence::Unknown,
                safe_reason_code: Some("practical_limit_not_measured".to_string()),
            },
        );
        features.insert(CapabilityFeatureId::CompanyRead, xml_evidence);
        for feature in [
            CapabilityFeatureId::LedgerRead,
            CapabilityFeatureId::VoucherRead,
            CapabilityFeatureId::SelectedLedgerRead,
            CapabilityFeatureId::SelectedVoucherWindowRead,
        ] {
            features.insert(
                feature,
                CapabilityEvidence {
                    state: CapabilityState::Unknown,
                    confidence: EvidenceConfidence::Unknown,
                    safe_reason_code: Some("selected_read_probe_not_run".to_string()),
                },
            );
        }
        features.insert(
            CapabilityFeatureId::Write,
            CapabilityEvidence {
                state: CapabilityState::Unknown,
                confidence: EvidenceConfidence::Unknown,
                safe_reason_code: Some("write_probe_not_run".to_string()),
            },
        );

        for pack in [
            CapabilityPackId::CoreAccounting,
            CapabilityPackId::IndiaTax,
            CapabilityPackId::BillsAndPayments,
            CapabilityPackId::Inventory,
        ] {
            packs.insert(
                pack,
                CapabilityEvidence {
                    state: CapabilityState::Unknown,
                    confidence: EvidenceConfidence::Unknown,
                    safe_reason_code: Some("verified_snapshot_not_run".to_string()),
                },
            );
        }

        Ok((
            TallyProbeResult {
                connection,
                companies,
                profile: CapabilityProfile {
                    // Version 4 adds observed release and licence tier, invalidating
                    // reuse of version-3 snapshots without those observations.
                    profile_version: 4,
                    product: gateway_product_mode.product,
                    release: gateway_product_mode.release,
                    license_tier: gateway_product_mode.license_tier,
                    mode: gateway_product_mode.mode,
                    transports,
                    features,
                    packs,
                },
                passport_snapshot_id: None,
            },
            wire_evidence,
        ))
    }

    /// Discovers companies through Tally's documented `Company` collection
    /// (`ReadOnlyProfile::CompanyListV2`). Unlike the legacy custom TDL
    /// report, its response is Tally's ordinary shaped `HEADER/STATUS=1`
    /// success envelope, so a successful parse directly satisfies the export
    /// trust check instead of requiring the narrower, explicitly-untrusted
    /// interactive compatibility parse.
    ///
    /// Responders that reject the collection outright — a shaped failure, an
    /// unrecognized shape, or anything else the collection parser cannot
    /// read — fall back to `legacy_company_discovery_evidence`, off the happy
    /// path but otherwise unchanged.
    async fn company_discovery_evidence(
        &self,
        connection: &mut ConnectionStatus,
        companies: &mut Vec<TallyCompany>,
        gateway_product_mode: &mut GatewayProductModeEvidence,
        wire_evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<CapabilityEvidence> {
        let xml = self
            .post_probe_xml(ReadOnlyProfile::CompanyListV2.render(), wire_evidence)
            .await?;
        match xml_parser::parse_companies_from_collection(&xml) {
            Ok(discovered) => {
                *gateway_product_mode = parse_company_gateway_capability_observation(&xml)
                    .map(GatewayProductModeEvidence::from_observation)
                    .unwrap_or_else(|_| GatewayProductModeEvidence::unavailable());
                connection.reachable = true;
                if connection.error.is_some() {
                    connection.error = Some("status_heuristic_unavailable".to_string());
                }
                Ok(match normalize_discovered_companies(discovered) {
                    Ok(normalized) => {
                        *companies = normalized;
                        CapabilityEvidence {
                            state: CapabilityState::Supported,
                            confidence: EvidenceConfidence::Observed,
                            safe_reason_code: None,
                        }
                    }
                    Err(()) => CapabilityEvidence {
                        state: CapabilityState::Unknown,
                        confidence: EvidenceConfidence::Observed,
                        safe_reason_code: Some("company_identity_invalid".to_string()),
                    },
                })
            }
            Err(_) => {
                self.legacy_company_discovery_evidence(connection, companies, wire_evidence)
                    .await
            }
        }
    }

    /// The pre-`CompanyListV2` company discovery path: the custom
    /// `CompanyListV1` TDL report, which most Tally responders answer with a
    /// bare `<ENVELOPE><COMPANYINFO>...` document carrying no
    /// `HEADER`/`STATUS` at all. That bare shape is accepted only through the
    /// narrow, explicitly-untrusted interactive discovery parse; it can never
    /// promote `CapabilityState::Supported`.
    async fn legacy_company_discovery_evidence(
        &self,
        connection: &mut ConnectionStatus,
        companies: &mut Vec<TallyCompany>,
        wire_evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<CapabilityEvidence> {
        let xml = self
            .post_probe_xml(tdl_engine::company_list_request(), wire_evidence)
            .await?;
        Ok(match xml_parser::parse_companies(&xml) {
            Ok(discovered) => {
                connection.reachable = true;
                if connection.error.is_some() {
                    connection.error = Some("status_heuristic_unavailable".to_string());
                }
                match normalize_discovered_companies(discovered) {
                    Ok(normalized) => {
                        *companies = normalized;
                        CapabilityEvidence {
                            state: CapabilityState::Supported,
                            confidence: EvidenceConfidence::Observed,
                            safe_reason_code: None,
                        }
                    }
                    Err(()) => CapabilityEvidence {
                        state: CapabilityState::Unknown,
                        confidence: EvidenceConfidence::Observed,
                        safe_reason_code: Some("company_identity_invalid".to_string()),
                    },
                }
            }
            Err(_) => match xml_parser::export_status(&xml) {
                Ok(xml_parser::TallyExportStatus::Failure) => CapabilityEvidence {
                    // A shaped failure is an endpoint claim, not responder
                    // authenticity or proof that the read profile works.
                    state: CapabilityState::Unknown,
                    confidence: EvidenceConfidence::Observed,
                    safe_reason_code: Some(
                        xml_parser::export_failure_reason_code(&xml).to_string(),
                    ),
                },
                _ if parse_companies_for_interactive_discovery(&xml).is_ok() => {
                    connection.reachable = true;
                    if connection.error.is_some() {
                        connection.error = Some("status_heuristic_unavailable".to_string());
                    }
                    CapabilityEvidence {
                        state: CapabilityState::Unknown,
                        confidence: EvidenceConfidence::Observed,
                        safe_reason_code: Some("direct_company_report_untrusted".to_string()),
                    }
                }
                _ => CapabilityEvidence {
                    state: CapabilityState::Unknown,
                    confidence: EvidenceConfidence::Observed,
                    safe_reason_code: Some("xml_export_shape_unrecognized".to_string()),
                },
            },
        })
    }

    pub(super) async fn post_probe_xml(
        &self,
        xml: String,
        evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<String> {
        let response = self.http.post_xml_decoded(xml).await?;
        self.probe_response(response, evidence)
    }

    /// Take this endpoint's wire lock for one send, once (a taken lock is
    /// refused, not waited for, so no wait can land between the caller's own
    /// checks): before an import POST's attempt is recorded. The lock is spent
    /// on that one send by [`WireHeldClient`]; nothing that waits on another
    /// process may run while it is held.
    pub(super) async fn acquire_wire(&self) -> anyhow::Result<WireHeldClient<'_>> {
        let wire = self.http.acquire_wire_lock().await?;
        Ok(WireHeldClient { client: self, wire })
    }

    fn probe_response(
        &self,
        response: bridge_tally_transport::TallyDecodedHttpResponse,
        evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<String> {
        let wire = RuntimeReadEvidence {
            request_sha256: response
                .request_body_sha256()
                .ok_or_else(|| anyhow::anyhow!("Tally POST omitted request wire commitment"))?
                .to_string(),
            response_sha256: response.encoded_sha256().to_string(),
            bytes: response.encoded_bytes(),
        };
        self.record_observed_body_bytes(response.encoded_bytes());
        self.record_observed_encoding(response.encoding());
        *evidence = evidence.clone().combine(wire);
        Ok(response.into_text())
    }

    pub(super) async fn post_xml(&self, xml: String) -> anyhow::Result<String> {
        self.post_xml_with_encoded_bytes(xml)
            .await
            .map(|(xml, _, _)| xml)
    }

    async fn post_xml_with_encoded_bytes(
        &self,
        xml: String,
    ) -> anyhow::Result<(String, usize, String)> {
        let response = self.http.post_xml_decoded(xml).await?;
        let encoded_bytes = response.encoded_bytes();
        let encoded_sha256 = response.encoded_sha256().to_string();
        self.record_observed_body_bytes(encoded_bytes);
        self.record_observed_encoding(response.encoding());
        Ok((response.into_text(), encoded_bytes, encoded_sha256))
    }

    #[cfg(feature = "voucher-scan")]
    async fn post_outstandings_xml_with_encoded_bytes(
        &self,
        request: VoucherOutstandingsRequestXml,
    ) -> anyhow::Result<OutstandingsWireResponse> {
        let response = self.http.post_outstandings_xml_decoded(request).await?;
        let encoded_bytes = response.encoded_bytes();
        let encoded_sha256 = response.encoded_sha256().to_string();
        self.record_observed_body_bytes(encoded_bytes);
        self.record_observed_encoding(response.encoding());
        Ok(OutstandingsWireResponse {
            text: response.into_text(),
            encoded_bytes,
            encoded_sha256,
        })
    }

    /// Uses the ordinary 32 MiB XML cap. Only the wildcard outstandings
    /// profile is allowed through `post_outstandings_xml_decoded`.
    #[cfg(feature = "voucher-scan")]
    async fn post_xml_with_wire_evidence(
        &self,
        request: String,
    ) -> anyhow::Result<OutstandingsWireResponse> {
        let response = self.http.post_xml_decoded(request).await?;
        let encoded_bytes = response.encoded_bytes();
        let encoded_sha256 = response.encoded_sha256().to_string();
        self.record_observed_body_bytes(encoded_bytes);
        self.record_observed_encoding(response.encoding());
        Ok(OutstandingsWireResponse {
            text: response.into_text(),
            encoded_bytes,
            encoded_sha256,
        })
    }

    /// Discovers companies through Tally's documented `Company` collection
    /// (`ReadOnlyProfile::CompanyListV2`) rather than the legacy `CompanyListV1`
    /// custom TDL report. Unlike the legacy report -- which one Tally instance
    /// answers with a bare, unwrapped `<COMPANYINFO>` document and another is
    /// known to simply hang on -- the collection always answers with the
    /// ordinary shaped `HEADER/STATUS=1` envelope, so `parse_companies_from_collection`
    /// can require that shape outright.
    pub async fn fetch_companies(&self) -> anyhow::Result<Vec<TallyCompany>> {
        self.fetch_companies_with_wire_evidence()
            .await
            .map(|(companies, _)| companies)
    }

    /// Enumerates the complete Company collection together with the exact
    /// request/response commitment. Write admission retains this observation
    /// instead of treating a parsed tuple as sufficient evidence.
    pub(crate) async fn fetch_companies_with_wire_evidence(
        &self,
    ) -> anyhow::Result<(Vec<TallyCompany>, RuntimeReadEvidence)> {
        self.fetch_companies_observing_education_mode()
            .await
            .map(|(companies, evidence, _)| (companies, evidence))
    }

    /// As [`Self::fetch_companies_with_wire_evidence`], also returning whether
    /// the same `CompanyListV2` response may come from an Education-mode
    /// endpoint: by the full capability observation, or by any `EDUMODE` field
    /// saying anything but `No` when that observation does not parse. No
    /// further request is made.
    pub(crate) async fn fetch_companies_observing_education_mode(
        &self,
    ) -> anyhow::Result<(Vec<TallyCompany>, RuntimeReadEvidence, bool)> {
        let mut evidence = RuntimeReadEvidence::empty();
        let xml = self
            .post_probe_xml(ReadOnlyProfile::CompanyListV2.render(), &mut evidence)
            .await?;
        let education = parse_company_gateway_capability_observation(&xml)
            .map(|observation| observation.educational_mode)
            .unwrap_or(false)
            || bridge_tally_protocol::company_list_may_be_in_educational_mode(&xml);
        let discovered = xml_parser::parse_companies_from_collection(&xml)
            .map_err(|error| with_read_evidence(error, evidence.clone()))?;
        let companies = normalize_discovered_companies(discovered).map_err(|_| {
            with_read_evidence(
                anyhow::anyhow!(
                    "Tally returned an invalid company identity for interactive discovery"
                ),
                evidence.clone(),
            )
        })?;
        Ok((companies, evidence, education))
    }

    /// Re-enumerates the trusted `Company` collection, then proves one
    /// user-chosen name with a separate shaped standard collection response.
    /// The collection's GUID is deliberately discarded; only the standard
    /// ledger identity collection's computed context may construct the
    /// returned company identity -- that binding proves Tally will actually
    /// scope subsequent reads to this exact company, which matching a name
    /// in a list can never prove by itself.
    pub async fn bootstrap_direct_company(
        &self,
        candidate_name: &str,
    ) -> anyhow::Result<TallyCompany> {
        let candidate_name = normalize_company_name(candidate_name)
            .map_err(|_| anyhow::anyhow!("Tally direct company candidate was invalid"))?;
        let discovered = self.fetch_companies().await?;
        let candidates = discovered
            .into_iter()
            .filter(|company| company.name == candidate_name)
            .collect::<Vec<_>>();
        let [candidate] = candidates.as_slice() else {
            anyhow::bail!("Tally direct company candidate was absent or ambiguous");
        };
        let xml = self
            .post_xml(tdl_engine::standard_ledger_identity_request(
                &candidate.name,
            ))
            .await?;
        let observed = parse_standard_ledger_identity_observation(&xml, &candidate.name)?;
        let guid = normalize_company_guid(&observed.company_guid)
            .map_err(|_| anyhow::anyhow!("Tally standard ledger identity was invalid"))?;
        if candidate
            .guid
            .as_deref()
            .is_none_or(|listed_guid| !listed_guid.eq_ignore_ascii_case(&guid))
        {
            return Err(DirectCompanyBootstrapError::CandidateGuidMismatch.into());
        }
        let Some(company_number) = candidate.company_number.clone() else {
            return Err(DirectCompanyBootstrapError::IncompleteTuple.into());
        };
        let Some(books_from) = candidate.books_from.clone() else {
            return Err(DirectCompanyBootstrapError::IncompleteTuple.into());
        };
        Ok(TallyCompany {
            name: candidate.name.clone(),
            guid: Some(guid),
            company_number: Some(company_number),
            books_from: Some(books_from),
        })
    }

    pub async fn fetch_ledgers(
        &self,
        identity: &VerifiedCompanyIdentity,
        boundary_profile: DateBoundaryProfile,
    ) -> anyhow::Result<Vec<TallyLedger>> {
        let opening_extent = self.fetch_company_book_extent(identity).await?;
        let period = NativeLedgerExportPeriod::new(
            boundary_profile,
            opening_extent.books_from().clone(),
            opening_extent.last_voucher_date().clone(),
        )
        .map_err(|_| {
            anyhow::anyhow!(
                "Tally master ledger export period is not supported by the endpoint compatibility profile"
            )
        })?;
        let paired = self
            .fetch_native_report_paired(render_native_ledger_export_request(
                identity.display_name(),
                &period,
            ))
            .await?;
        let (body, _, _) =
            paired.require_stable(PairedReadValidationError::NativeLedgerCollection)?;
        let parsed =
            parse_native_ledger_source_records_with_evidence(&body, identity.company_guid())?;
        let closing_extent = self.fetch_company_book_extent(identity).await?;
        if closing_extent != opening_extent {
            return Err(anyhow::Error::new(
                PairedReadValidationError::NativeLedgerExtent,
            ));
        }
        Ok(parsed
            .records
            .into_iter()
            .map(|record| record.record)
            .collect())
    }

    /// Reads the identity-bearing ledger master and the existing period-bound
    /// balance snapshot as one bracketed source for a customer workbook. The
    /// balance parser requires row GUID evidence for the selected company
    /// before any `(name, parent)` join can attach money to a master. The second
    /// value is the evidence of the reads that counted a marked book's ledgers:
    /// the catalogue pair (#668), the census's slices (#679), or both, empty
    /// when the mark alone admitted the read. `today` is
    /// the host's calendar date: the balance snapshot ends no later than it
    /// (the next admissible boundary at or after), whatever the extent's last
    /// voucher date says (#875).
    pub(crate) async fn fetch_party_ledger_master_source(
        &self,
        identity: &VerifiedCompanyIdentity,
        boundary_profile: DateBoundaryProfile,
        currency_assertion: PartyLedgerMasterCurrencyAssertion,
        today: &bridge_tally_core::TallyDate,
    ) -> anyhow::Result<(PartyLedgerMasterSource, RuntimeReadEvidence)> {
        let mut evidence = RuntimeReadEvidence::empty();
        let mut count_evidence = RuntimeReadEvidence::empty();
        let result = async {
            let opening_extent = self.fetch_company_book_extent(identity).await?;
            let ledger_currency_base = currency_assertion.ledger_currency_base().cloned();
            let master_mark = opening_extent
                .master_alter_id_high_water()
                .ok_or(OutstandingsError::MasterWitnessAbsent)?
                .get();
            // Sized before any ledger request is sent (#637): by the mark when
            // it fits, otherwise by the count of a balance-free catalogue read
            // once the local checks below have passed (#668).
            let admission = admit_compliance_master_read(master_mark, None)?;
            let currency = currency_assertion.require_opening_extent(&opening_extent)?;
            let master_period = NativeLedgerExportPeriod::new(
                boundary_profile,
                opening_extent.books_from().clone(),
                opening_extent.last_voucher_date().clone(),
            )
            .map_err(|_| {
                anyhow::Error::new(PartyLedgerMasterSourceValidationError::MasterPeriod)
            })?;
            let balance_period = party_ledger_master_balance_period(
                boundary_profile,
                opening_extent.books_from().clone(),
                opening_extent.last_voucher_date().clone(),
                today,
            )
            .map_err(|_| {
                anyhow::Error::new(PartyLedgerMasterSourceValidationError::BalancePeriod)
            })?;
            let mut partition = None;
            // How many ledgers the whole read must return, once a count says.
            let mut expected_ledgers = None;
            // A census counts a book whose mark is past what its catalogue can
            // be read for (#679); its count admits the read like a catalogue's.
            let mut census_count = None;
            // Whether the census's count was checked against the company's own.
            let mut count_cross_check = None;
            if admission == ComplianceAdmission::CensusFirst {
                let (census, cross_check) = self
                    .count_ledgers_by_span(
                        identity,
                        &opening_extent,
                        master_mark,
                        &mut evidence,
                        &mut count_evidence,
                    )
                    .await?;
                let counted = census.get();
                count_cross_check = Some(cross_check);
                census_count = Some(counted);
                if admit_compliance_master_read(master_mark, Some(counted))?
                    == ComplianceAdmission::InParts
                {
                    // Their parents are named by the catalogue, which the
                    // count now bounds.
                    admit_counted_catalogue(counted)?;
                } else {
                    expected_ledgers = Some(counted);
                }
            }
            if admission == ComplianceAdmission::CountFirst
                || (census_count.is_some() && expected_ledgers.is_none())
            {
                let catalogue_request =
                    render_standard_ledger_catalog_request(identity.display_name())?;
                let catalogue_pair = self
                    .fetch_native_report_paired(catalogue_request.clone())
                    .await?;
                let (catalogue_body, catalogue_bytes, catalogue_sha256) = catalogue_pair
                    .require_stable(PairedReadValidationError::PartyLedgerCatalogue)?;
                let catalogue_read = RuntimeReadEvidence::paired(
                    &catalogue_request,
                    catalogue_sha256,
                    catalogue_bytes,
                );
                count_evidence = count_evidence.clone().combine(catalogue_read.clone());
                evidence = evidence.clone().combine(catalogue_read);
                let catalogue = parse_standard_ledger_catalog_response(
                    &catalogue_body,
                    identity.display_name(),
                    identity.company_guid(),
                )
                .map_err(|source| {
                    PartyLedgerMasterSourceValidationError::LedgerCountInvalid { source }
                })?;
                let counted = catalogue.names().count() as u64;
                // Two counts of one book, taken by different requests: they
                // agree, or one of them is wrong.
                if let Some(census) = census_count {
                    if census != counted {
                        return Err(anyhow::Error::new(
                            PartyLedgerMasterSourceValidationError::LedgerCountDiffers {
                                expected: census,
                                observed: counted,
                            },
                        ));
                    }
                }
                // A count the whole read cannot fit is read as parts by parent
                // group (#679); the catalogue that counted the book also
                // names each ledger's parent, so nothing more is asked first.
                if admit_compliance_master_read(master_mark, Some(counted))?
                    == ComplianceAdmission::InParts
                {
                    let planned = ParentPartition::plan(
                        catalogue.identified_parents(),
                        parent_partition_limits(),
                    )
                    .map_err(|source| {
                        PartyLedgerMasterSourceValidationError::ParentPartition { source }
                    })?;
                    // Each part's balances are read at a different moment; the
                    // closing extent equalling the opening only proves nothing
                    // was written between them when it carries the voucher
                    // high-water, so several parts need it to be there.
                    if planned.parts().len() > 1
                        && opening_extent.voucher_alter_id_high_water().is_none()
                    {
                        return Err(anyhow::Error::new(
                            PartyLedgerMasterSourceValidationError::VoucherWitnessAbsent,
                        ));
                    }
                    partition = Some(planned);
                } else {
                    expected_ledgers = Some(counted);
                }
            }
            let group_request = render_native_group_snapshot_request(identity.display_name());
            let part_requests = match &partition {
                None => vec![(
                    render_party_ledger_master_request(identity.display_name(), &master_period),
                    render_native_ledger_snapshot_request(identity.display_name(), &balance_period),
                )],
                Some(partition) => partition
                    .parts()
                    .iter()
                    .map(|part| {
                        (
                            render_party_ledger_master_request_for_parents(
                                identity.display_name(),
                                &master_period,
                                part,
                            ),
                            render_native_ledger_snapshot_request_for_parents(
                                identity.display_name(),
                                &balance_period,
                                part,
                            ),
                        )
                    })
                    .collect(),
            };
            let committed_requests = part_requests
                .iter()
                .flat_map(|(master, balance)| [master.clone(), balance.clone()])
                .chain(std::iter::once(group_request.clone()))
                .collect::<Vec<_>>();
            let request_sha256 = party_ledger_request_commitment(&committed_requests);
            let mut reads = Vec::with_capacity(part_requests.len());
            for (index, (master_request, balance_request)) in part_requests.iter().enumerate() {
                reads.push(
                    self.read_party_ledger_master_part(
                        identity,
                        master_request,
                        balance_request,
                        match &partition {
                            Some(partition) => PartyLedgerRead::Part(&partition.parts()[index]),
                            None => PartyLedgerRead::Whole { expected_ledgers },
                        },
                        ledger_currency_base.as_ref(),
                        &mut evidence,
                    )
                    .await?,
                );
            }
            let group_pair = self
                .fetch_native_report_paired(group_request.clone())
                .await?;
            let (group_body, group_response_bytes, group_response_sha256) =
                group_pair.require_stable(PairedReadValidationError::PartyLedgerGroup)?;
            evidence = evidence.clone().combine(RuntimeReadEvidence::paired(
                &group_request,
                group_response_sha256.clone(),
                group_response_bytes,
            ));
            let groups =
                parse_native_group_snapshot_with_evidence(&group_body, identity.company_guid())
                    .map_err(party_ledger_master_group_snapshot_error)?
                    .into_iter()
                    .map(|entry| entry.record)
                    .collect();
            let closing_extent = self.fetch_company_book_extent(identity).await?;
            if closing_extent != opening_extent {
                return Err(anyhow::Error::new(
                    PairedReadValidationError::PartyLedgerExtent,
                ));
            }
            // The parts together must be the catalogue's ledgers, each once, in
            // the part its parent names, and with the catalogue's name and
            // parent: a part that lost, doubled or renamed a row is withheld.
            if let Some(partition) = &partition {
                let mut coverage = partition.coverage();
                for (index, read) in reads.iter().enumerate() {
                    for source in &read.master.records {
                        let guid = source.identities.guid.as_deref().ok_or_else(|| {
                            anyhow::Error::new(PartyLedgerMasterSourceValidationError::MasterGuid)
                        })?;
                        coverage
                            .accept(
                                index,
                                guid,
                                &source.record.ledger.name,
                                source.record.ledger.parent.nonempty_returned_text(),
                            )
                            .map_err(|source| {
                                PartyLedgerMasterSourceValidationError::ParentPartition { source }
                            })?;
                    }
                }
                coverage.finish().map_err(|source| {
                    PartyLedgerMasterSourceValidationError::ParentPartition { source }
                })?;
            }

            let (master_response_sha256, master_response_bytes) =
                aggregate_part_evidence(reads.iter().map(|read| {
                    (
                        read.master_response_sha256.as_str(),
                        read.master_response_bytes,
                    )
                }));
            let (balance_response_sha256, balance_response_bytes) =
                aggregate_part_evidence(reads.iter().map(|read| {
                    (
                        read.balance_response_sha256.as_str(),
                        read.balance_response_bytes,
                    )
                }));
            let several_parts = reads.len() > 1;
            let mut rows = Vec::new();
            let mut foreign_currency_ledgers_excluded = Vec::new();
            let mut mixed_currency_ledgers_excluded = Vec::new();
            for read in reads {
                foreign_currency_ledgers_excluded.extend(read.foreign.iter().cloned());
                mixed_currency_ledgers_excluded.extend(read.mixed.iter().cloned());
                rows.extend(join_party_ledger_master_part(read)?);
            }
            if several_parts {
                foreign_currency_ledgers_excluded
                    .sort_by(|left, right| left.ledger.cmp(&right.ledger));
                mixed_currency_ledgers_excluded.sort();
            }
            rows.sort_by(|left, right| left.name.cmp(&right.name).then(left.guid.cmp(&right.guid)));
            let source = PartyLedgerMasterSource {
                count_cross_check,
                company: identity.display_name().to_string(),
                company_guid: identity.company_guid().to_string(),
                currency_assertion: currency.assertion,
                currency_decimal_places: currency.decimal_places,
                from: master_period.from().clone(),
                // The snapshot period is the balance evidence. Its derived end is
                // the date Tally was actually asked to honor, not merely the last
                // voucher date used by the identity/master read.
                to: balance_period.to().clone(),
                last_voucher_date: opening_extent.last_voucher_date().clone(),
                rows,
                request_sha256,
                master_response_sha256,
                balance_response_sha256,
                group_response_sha256,
                master_response_bytes,
                balance_response_bytes,
                group_response_bytes,
                groups,
                foreign_currency_ledgers_excluded,
                mixed_currency_ledgers_excluded,
            };
            Ok((source, count_evidence))
        }
        .await;
        result.map_err(|error| crate::tally::runtime::with_read_evidence(error, evidence))
    }

    /// Counts a marked book's ledgers by AlterID span (#679): one single read
    /// per slice of `(0, mark]`, each asking for the ledgers' GUIDs only, and
    /// no slice is read twice. Stability of the book across the census is not
    /// proven here: the caller's opening and closing company extent, which
    /// carry the master mark and the company GUID, are the bracket (and the
    /// extent is read again here, before the count is returned, after Tally's
    /// own count of the company's ledgers was read once and compared, #938), and an empty
    /// slice is the same body a closed or absent company answers with, so a
    /// census that found no ledger at all is refused rather than counted.
    /// Each slice whose answer was received in full is added to `evidence` and
    /// to `count_evidence` as it completes, so a refusal still accounts for
    /// what was received; a slice whose request failed, or whose answer passed
    /// the response cap, adds nothing.
    async fn count_ledgers_by_span(
        &self,
        identity: &VerifiedCompanyIdentity,
        opening_extent: &CompanyBookExtent,
        master_mark: u64,
        evidence: &mut RuntimeReadEvidence,
        count_evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<(LedgerCount, CountCrossCheck)> {
        let plan = LedgerCensusPlan::new(master_mark, ledger_census_limits())
            .map_err(|source| PartyLedgerMasterSourceValidationError::LedgerSpan { source })?;
        let mut census = LedgerCensus::new(plan);
        while let Some(slice) = census.next_slice() {
            let request = render_ledger_census_slice_request(identity.display_name(), &slice);
            let (body, bytes, sha256) = self
                .post_xml_with_encoded_bytes(request.clone())
                .await
                .map_err(ledger_span_response_error)?;
            let read = RuntimeReadEvidence::single(&request, sha256, bytes);
            *evidence = evidence.clone().combine(read.clone());
            *count_evidence = count_evidence.clone().combine(read);
            // The parser only bounds what it will hold; the slice's span is
            // enforced once, by the census, under its own typed refusal.
            let guids = parse_ledger_census_slice(
                &body,
                identity.company_guid(),
                MAX_STANDARD_LEDGER_IDENTITY_ROWS as u64,
            )
            .map_err(|source| {
                PartyLedgerMasterSourceValidationError::LedgerSpanSliceInvalid { source }
            })?;
            census
                .accept(guids)
                .map_err(|source| PartyLedgerMasterSourceValidationError::LedgerSpan { source })?;
        }
        let counted = census
            .finish()
            .map_err(|source| PartyLedgerMasterSourceValidationError::LedgerSpan { source })?;
        // A company closed, reopened or switched during the census answers the
        // remaining slices with the same empty body as a slice past every
        // ledger, so the count can be low, and the count sizes the next read.
        // Two checks run before the count leaves this function, so no caller
        // can use a count without them. Tally's own count of the company's
        // ledgers is read and must not be higher than the census's: that is
        // meant to catch a company closed and reopened with equal marks, which
        // the extent cannot see (by reasoning: no live reproduction), and it
        // also refuses a ledger added during the read (#938), and, by reasoning
        // only (#965), one altered during it before its slice was read: its new
        // AlterID is past every slice, while Tally still counts it. A ledger
        // altered after its slice was read was counted; the extent read below
        // refuses that case instead. Then the extent (company GUID and marks) is
        // read again, and a change in either refuses the call.
        let cross_check = self
            .cross_check_census_count(identity, counted.get(), evidence, count_evidence)
            .await?;
        if self.fetch_company_book_extent(identity).await? != *opening_extent {
            return Err(anyhow::Error::new(
                PairedReadValidationError::PartyLedgerExtent,
            ));
        }
        Ok((counted, cross_check))
    }

    /// Reads Tally's own count of the company's ledgers once and compares it
    /// with the census's (#938). Refuse-only: a count higher than the census's
    /// refuses the call; equal, lower, or absent never admits or sizes anything.
    /// A response for another company, or a value that is not a plain integer,
    /// is refused as any damaged answer is. The request is one read, not a
    /// pair: the extent bracket around the census is what proves the book did
    /// not move, and this count is compared, never relied on.
    async fn cross_check_census_count(
        &self,
        identity: &VerifiedCompanyIdentity,
        census: u64,
        evidence: &mut RuntimeReadEvidence,
        count_evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<CountCrossCheck> {
        let request = render_company_ledger_count_request(identity.display_name());
        let (body, bytes, sha256) = self
            .post_xml_with_encoded_bytes(request.clone())
            .await
            .map_err(company_count_response_error)?;
        let read = RuntimeReadEvidence::single(&request, sha256, bytes);
        *evidence = evidence.clone().combine(read.clone());
        *count_evidence = count_evidence.clone().combine(read);
        let company =
            parse_company_ledger_count(&body, identity.display_name(), identity.company_guid())
                .map_err(|source| {
                    PartyLedgerMasterSourceValidationError::LedgerCountCompanyInvalid { source }
                })?;
        Ok(match company.map(|count| count.get()) {
            None => CountCrossCheck::Unavailable,
            Some(company) if company > census => {
                return Err(anyhow::Error::new(
                    PartyLedgerMasterSourceValidationError::LedgerCountCompanyDiffers {
                        company,
                        census,
                    },
                ));
            }
            Some(company) if company == census => CountCrossCheck::Matched,
            Some(_) => CountCrossCheck::CompanyCountLower,
        })
    }

    /// One master-and-balance pair of a compliance ledger read: the whole book,
    /// or one parent part of it (#679). Each response is checked against the
    /// selected company and classified before any other request is sent.
    async fn read_party_ledger_master_part(
        &self,
        identity: &VerifiedCompanyIdentity,
        master_request: &str,
        balance_request: &str,
        scope: PartyLedgerRead<'_>,
        ledger_currency_base: Option<&BaseCurrencyName>,
        evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<PartyLedgerMasterPartRead> {
        let part = match scope {
            PartyLedgerRead::Part(part) => Some(part),
            PartyLedgerRead::Whole { .. } => None,
        };
        let master_pair = self
            .fetch_native_report_paired(master_request.to_owned())
            .await
            .map_err(|error| parent_part_response_error(part, error))?;
        let (master_body, master_response_bytes, master_response_sha256) =
            master_pair.require_stable(PairedReadValidationError::PartyLedgerMaster)?;
        *evidence = evidence.clone().combine(RuntimeReadEvidence::paired(
            master_request,
            master_response_sha256.clone(),
            master_response_bytes,
        ));
        // Refuse a wrong or damaged master before any further request; its
        // amounts are admitted below, once the snapshot names the ledgers
        // set aside (bridge#551).
        let structure =
            parse_native_party_ledger_master_structure(&master_body, identity.company_guid())
                .map_err(party_ledger_master_master_snapshot_error)?;
        if !structure.evidence.duplicate_identities.is_empty() {
            return Err(anyhow::Error::new(
                PartyLedgerMasterSourceValidationError::DuplicateMasterIdentity,
            ));
        }
        // A part that came back short or long means Tally did not apply its
        // filter as asked. Its master pair is already sent, the complement's
        // too; this keeps back its balance and every later request, the next
        // part's among them, whose filter rests on the same assumption.
        if let Some(part) = part {
            part.check_row_count(structure.records.len())
                .map_err(
                    |source| PartyLedgerMasterSourceValidationError::ParentPartition { source },
                )?;
        } else if let PartyLedgerRead::Whole {
            expected_ledgers: Some(expected),
        } = scope
        {
            // A whole read after a count: the count was taken by another
            // request, and the master read returns the same collection.
            let observed = structure.records.len() as u64;
            if observed != expected {
                return Err(anyhow::Error::new(
                    PartyLedgerMasterSourceValidationError::LedgerCountDiffers {
                        expected,
                        observed,
                    },
                ));
            }
        }
        let balance_pair = self
            .fetch_native_report_paired(balance_request.to_owned())
            .await
            .map_err(|error| parent_part_response_error(part, error))?;
        let (balance_body, balance_response_bytes, balance_response_sha256) =
            balance_pair.require_stable(PairedReadValidationError::PartyLedgerBalance)?;
        *evidence = evidence.clone().combine(RuntimeReadEvidence::paired(
            balance_request,
            balance_response_sha256.clone(),
            balance_response_bytes,
        ));
        // Each ledger's own currency is compared with the base before any
        // balance is parsed (bridge#551): a foreign ledger's balance is a
        // composite display string, never rupees, so the ledger leaves the
        // source, its master row included, and is named instead. So does a
        // base-currency ledger with any composite balance: a rupee ledger a
        // foreign-currency entry touched. A base of one master refuses a
        // ledger in another currency outright. An assertion with no base
        // (one master whose NAME was not read) keeps the unclassified read.
        let (balances, foreign, mixed) = match ledger_currency_base {
            Some(base) => {
                let classified = parse_compliance_ledger_snapshot_for_company(
                    &balance_body,
                    identity.company_guid(),
                    base,
                )
                .map_err(party_ledger_master_balance_snapshot_error)?;
                (classified.base, classified.foreign, classified.mixed)
            }
            None => (
                parse_native_ledger_snapshot_for_company(&balance_body, identity.company_guid())
                    .map_err(party_ledger_master_balance_snapshot_error)?,
                Vec::new(),
                Vec::new(),
            ),
        };
        // Ledger names are unique within a Tally company, so a ledger set
        // aside is the master row with its name. The master is parsed only
        // now, leaving those rows' openings unparsed.
        let set_aside = foreign
            .iter()
            .map(|ledger| ledger.ledger.clone())
            .chain(mixed.iter().cloned())
            .collect::<BTreeSet<_>>();
        let master = parse_native_party_ledger_master_records_leaving_unparsed(
            &master_body,
            identity.company_guid(),
            &set_aside,
        )
        .map_err(party_ledger_master_master_snapshot_error)?;
        if !master.evidence.duplicate_identities.is_empty() {
            return Err(anyhow::Error::new(
                PartyLedgerMasterSourceValidationError::DuplicateMasterIdentity,
            ));
        }
        Ok(PartyLedgerMasterPartRead {
            master,
            balances,
            foreign,
            mixed,
            set_aside,
            master_response_sha256,
            master_response_bytes,
            balance_response_sha256,
            balance_response_bytes,
        })
    }

    /// Reads the documented standard ledger collection as an explicitly limited
    /// compatibility catalog. It is not a fallback for Bridge's custom export
    /// and cannot establish snapshot, voucher, or write capability.
    pub async fn fetch_standard_ledger_catalog(
        &self,
        company: &str,
        expected_company_guid: &str,
    ) -> anyhow::Result<Vec<TallyLedger>> {
        let xml = self
            .post_xml(tdl_engine::standard_ledger_catalog_request(company))
            .await?;
        Ok(parse_standard_ledger_catalog(
            &xml,
            company,
            expected_company_guid,
        )?)
    }

    /// One extra paired read per scan: bill-wise OPENING balances live on
    /// ledger masters, so a voucher-only scan is blind to them.
    ///
    /// Takes the already GUID-verified `PinnedCompany` rather than a bare name.
    /// The ledger profile fetches every master GUID and verifies its company
    /// GUID prefix, so a name-only selection cannot make another loaded
    /// company's coverage look like the pinned book.
    #[cfg(feature = "voucher-scan")]
    pub(crate) async fn fetch_ledger_opening_coverage(
        &self,
        company: &PinnedCompany,
    ) -> anyhow::Result<LedgerOpeningCoverageRead> {
        let company_name = ValidatedCompanyName::new(company.name().to_string())?;
        let request = ReadOnlyProfile::LedgerOpeningCoverageV1 {
            company: &company_name,
        }
        .render();
        let first = self.post_xml(request.clone()).await?;
        self.http
            .get_status_decoded()
            .await
            .context("Tally health check between ledger opening reads failed")?;
        let second = self.post_xml(request).await?;
        self.http
            .get_status_decoded()
            .await
            .context("Tally health check after ledger opening reads failed")?;
        let first = parse_ledger_opening_coverage(&first, company)?;
        let second = parse_ledger_opening_coverage(&second, company)?;
        if first != second {
            return Ok(LedgerOpeningCoverageRead::Drifted);
        }
        Ok(LedgerOpeningCoverageRead::Stable(first))
    }

    pub async fn fetch_company_book_extent(
        &self,
        identity: &VerifiedCompanyIdentity,
    ) -> anyhow::Result<CompanyBookExtent> {
        self.fetch_company_book_extent_with_evidence(identity, &mut RuntimeReadEvidence::empty())
            .await
    }

    /// [`Self::fetch_company_book_extent`], adding each of its two reads to
    /// `evidence` as it completes, so a failure still accounts for what was sent.
    pub(crate) async fn fetch_company_book_extent_with_evidence(
        &self,
        identity: &VerifiedCompanyIdentity,
        evidence: &mut RuntimeReadEvidence,
    ) -> anyhow::Result<CompanyBookExtent> {
        let expectation = identity.company_book_extent_expectation()?;
        let company_name = ValidatedCompanyName::new(identity.display_name().to_owned())?;
        let request = ReadOnlyProfile::CompanyBookExtentV2 {
            company: &company_name,
        }
        .render();
        let (first, first_bytes, first_sha256) =
            self.post_xml_with_encoded_bytes(request.clone()).await?;
        *evidence = evidence.clone().combine(RuntimeReadEvidence::single(
            &request,
            first_sha256,
            first_bytes,
        ));
        self.http
            .get_status_decoded()
            .await
            .context("Tally health check between company extent reads failed")?;
        let (second, second_bytes, second_sha256) =
            self.post_xml_with_encoded_bytes(request.clone()).await?;
        *evidence = evidence.clone().combine(RuntimeReadEvidence::single(
            &request,
            second_sha256,
            second_bytes,
        ));
        self.http
            .get_status_decoded()
            .await
            .context("Tally health check after company extent reads failed")?;
        let first = parse_company_book_extent_v2(&first, &expectation)?;
        let second = parse_company_book_extent_v2(&second, &expectation)?;
        if first != second {
            return Err(anyhow::Error::new(
                PairedReadValidationError::CompanyBookExtent,
            ));
        }
        // The parser stays tolerant of an absent ALTMSTID (older captures still parse), but this
        // is the outstandings bracket itself: fail closed here so a witness-less pair -- which
        // would otherwise compare equal regardless of a mid-window master edit -- can never be
        // mistaken for a stable one. See `require_master_witness` for why.
        require_master_witness(&first)?;
        Ok(first)
    }

    /// Paired read for the native `TYPE=Data` bills reports and the ledger
    /// closing snapshot.
    ///
    /// These responses are small — measured 11 KB for 48 bills and 41 KB for 88
    /// ledgers — so the whole-response byte comparison this performs is cheap,
    /// and it replaces the date/AlterID partition-completeness machinery the
    /// voucher scan needs. A drift between the two reads means the book moved
    /// mid-sequence; the caller must treat that as Partial rather than pick a
    /// side.
    ///
    /// Health checks bracket both requests and sit between them, so a gateway
    /// that stalls mid-pair is distinguishable from a clean pair. See the
    /// identical discipline in `fetch_company_book_extent`.
    pub(crate) async fn fetch_native_report_paired(
        &self,
        request_xml: String,
    ) -> anyhow::Result<NativePairedRead> {
        match self
            .fetch_native_report_paired_with_evidence(request_xml)
            .await
        {
            Ok((body, encoded_bytes, encoded_sha256)) => Ok(NativePairedRead::Stable {
                body,
                encoded_bytes,
                encoded_sha256,
            }),
            // Preserve existing financial callers' explicit Partial verdict.
            Err(error)
                if error
                    .chain()
                    .any(|cause| cause.is::<NativeReportPairDrift>()) =>
            {
                let failure = error.downcast::<super::runtime::RuntimeReadFailure>()?;
                Ok(NativePairedRead::Drifted(failure.evidence))
            }
            Err(error) => Err(error),
        }
    }

    // Both adapters and the financial verdict wrapper retain completed source
    // commitments on pair drift; the wrapper preserves its Partial classification.
    pub(crate) async fn fetch_native_report_paired_with_evidence(
        &self,
        request_xml: String,
    ) -> anyhow::Result<(String, usize, String)> {
        let (first, first_bytes, first_sha256) = self
            .post_xml_with_encoded_bytes(request_xml.clone())
            .await
            .map_err(|error| anyhow::Error::new(PairedNativeReportResponseFailure::new(error)))?;
        let mut evidence =
            RuntimeReadEvidence::single(&request_xml, first_sha256.clone(), first_bytes);
        let result = async {
            self.http
                .get_status_decoded()
                .await
                .context("Tally health check between paired native report reads failed")?;
            let (second, second_bytes, second_sha256) = self
                .post_xml_with_encoded_bytes(request_xml.clone())
                .await
                .map_err(|error| {
                    anyhow::Error::new(PairedNativeReportResponseFailure::new(error))
                })?;
            if first_bytes == second_bytes && first_sha256 == second_sha256 {
                evidence.bytes = evidence.bytes.saturating_add(second_bytes);
            } else {
                evidence = evidence.clone().combine(RuntimeReadEvidence::single(
                    &request_xml,
                    second_sha256.clone(),
                    second_bytes,
                ));
            }
            self.http
                .get_status_decoded()
                .await
                .context("Tally health check after paired native report reads failed")?;
            if first != second || first_bytes != second_bytes || first_sha256 != second_sha256 {
                return Err(NativeReportPairDrift.into());
            }
            Ok((first, first_bytes, first_sha256))
        }
        .await;
        result.map_err(|error| super::runtime::with_read_evidence(error, evidence))
    }

    /// One XML POST whose HTTP response entity is kept byte for byte, for
    /// audit_read, which seals those bytes rather than a re-encoding of the
    /// decoded text. The entity is what the transport received after chunked
    /// framing is removed; content encodings other than identity are refused.
    /// The decoded text is returned beside it for admission.
    pub(crate) async fn post_xml_raw(&self, xml: String) -> anyhow::Result<RawTallyResponse> {
        let response = self.http.post_xml(xml).await?;
        self.record_observed_body_bytes(response.encoded_bytes());
        self.record_observed_encoding(response.encoding());
        let encoded_body = response.encoded_body().to_vec();
        let encoded_sha256 = sha256_hex(&encoded_body);
        Ok(RawTallyResponse {
            text: response.into_text(),
            encoded_body,
            encoded_sha256,
        })
    }

    /// [`Self::post_xml_raw`] twice, with the same health checks as
    /// [`Self::fetch_native_report_paired_with_evidence`], admitted only when
    /// the two response entities are byte-identical. On drift both completed
    /// bodies are accounted for and neither is released, and the refusal is
    /// returned before the trailing health check, so a drift is never reported
    /// as that check's failure.
    pub(crate) async fn post_xml_raw_paired(
        &self,
        xml: String,
    ) -> anyhow::Result<RawTallyResponse> {
        let first = self.post_xml_raw(xml.clone()).await?;
        let mut evidence = RuntimeReadEvidence::single(
            &xml,
            first.encoded_sha256.clone(),
            first.encoded_body.len(),
        );
        let result = async {
            self.http
                .get_status_decoded()
                .await
                .context("Tally health check between paired audit reads failed")?;
            let second = self.post_xml_raw(xml.clone()).await?;
            if second.encoded_body == first.encoded_body {
                evidence.bytes = evidence.bytes.saturating_add(second.encoded_body.len());
            } else {
                evidence = evidence.clone().combine(RuntimeReadEvidence::single(
                    &xml,
                    second.encoded_sha256.clone(),
                    second.encoded_body.len(),
                ));
            }
            if second.encoded_body != first.encoded_body {
                return Err(NativeReportPairDrift.into());
            }
            self.http
                .get_status_decoded()
                .await
                .context("Tally health check after paired audit reads failed")?;
            Ok(())
        }
        .await;
        result.map_err(|error| super::runtime::with_read_evidence(error, evidence))?;
        Ok(first)
    }

    /// The smallest request Tally answers: its status page. Used to learn
    /// whether a responder that was left building an abandoned response has
    /// finished; it proves nothing else.
    pub(crate) async fn status_probe(&self) -> anyhow::Result<()> {
        self.http.get_status_decoded().await?;
        Ok(())
    }

    #[cfg(feature = "voucher-scan")]
    pub(crate) async fn fetch_outstandings_segment_pair(
        &self,
        company: &PinnedCompany,
        segment_window: NarrowDateWindow,
        alter_id_range: AlterIdRange,
    ) -> anyhow::Result<OutstandingsSegmentObservation> {
        let request = voucher_outstandings_request(company, &segment_window, alter_id_range);
        let range_label = format!(
            "{}..{}",
            alter_id_range.exclusive_start(),
            alter_id_range.inclusive_end()
        );
        let first_started = Instant::now();
        let first = self
            .post_outstandings_xml_with_encoded_bytes(request.clone())
            .await
            .with_context(|| {
                format!("outstandings first segment read failed for AlterID {range_label}")
            })?;
        let first_read_elapsed = first_started.elapsed();
        self.http.get_status_decoded().await.with_context(|| {
            format!(
                "Tally health check between outstandings reads failed for AlterID {range_label}"
            )
        })?;
        let second_started = Instant::now();
        let second = self
            .post_outstandings_xml_with_encoded_bytes(request)
            .await
            .with_context(|| {
                format!("outstandings second segment read failed for AlterID {range_label}")
            })?;
        let second_read_elapsed = second_started.elapsed();
        self.http.get_status_decoded().await.with_context(|| {
            format!("Tally health check after outstandings reads failed for AlterID {range_label}")
        })?;
        let verification = verify_segment_pair_with_wire_evidence(
            SegmentWireEvidence::new(&first.text, first.encoded_bytes, &first.encoded_sha256),
            SegmentWireEvidence::new(&second.text, second.encoded_bytes, &second.encoded_sha256),
            company,
            segment_window.into_date_window(),
            alter_id_range,
        )?;
        Ok(OutstandingsSegmentObservation {
            verification,
            first_read_elapsed,
            second_read_elapsed,
        })
    }

    /// Executes one paired, date-only I5 witness read. This is intentionally
    /// separate from `fetch_outstandings_segment_pair`: it has no AlterID
    /// predicate and uses the ordinary 32 MiB transport cap. Its supervised
    /// live qualification is recorded in TALLY_PROTOCOL_REFERENCE.md §12.7;
    /// runtime may use it only for a primary-empty partition's control or
    /// shifted cover.
    #[cfg(feature = "voucher-scan")]
    pub(crate) async fn fetch_empty_partition_witness_pair(
        &self,
        company: &PinnedCompany,
        window: NarrowDateWindow,
    ) -> anyhow::Result<WitnessPairVerification> {
        let request = voucher_empty_partition_witness_request(company, &window).into_xml();
        let label = format!("{}..{}", window.from().as_str(), window.to().as_str());
        let first = self
            .post_xml_with_wire_evidence(request.clone())
            .await
            .with_context(|| format!("empty-date witness first read failed for {label}"))?;
        self.http.get_status_decoded().await.with_context(|| {
            format!("Tally health check between empty-date witness reads for {label}")
        })?;
        let second = self
            .post_xml_with_wire_evidence(request)
            .await
            .with_context(|| format!("empty-date witness second read failed for {label}"))?;
        self.http.get_status_decoded().await.with_context(|| {
            format!("Tally health check after empty-date witness reads for {label}")
        })?;
        verify_empty_partition_witness_pair_with_wire_evidence(
            SegmentWireEvidence::new(&first.text, first.encoded_bytes, &first.encoded_sha256),
            SegmentWireEvidence::new(&second.text, second.encoded_bytes, &second.encoded_sha256),
            company,
            window.into_date_window(),
        )
        .map_err(anyhow::Error::from)
    }

    pub async fn fetch_vouchers(
        &self,
        identity: &VerifiedCompanyIdentity,
        from: &str,
        to: &str,
    ) -> anyhow::Result<Vec<TallyVoucher>> {
        // Fail closed: `from`/`to` feed a quoted `$$Date:"..."` TDL formula
        // argument, where XML escaping alone cannot contain an embedded
        // quote (Tally decodes `&quot;` back to `"` before evaluating the
        // formula). Requiring a validated `TallyDate` -- exactly 8 ASCII
        // digits -- closes that off at the source instead of sanitising.
        let from = bridge_tally_core::TallyDate::parse(from)
            .context("voucher export from-date must be a valid YYYYMMDD date")?;
        let to = bridge_tally_core::TallyDate::parse(to)
            .context("voucher export to-date must be a valid YYYYMMDD date")?;
        let xml = self
            .post_xml(render_native_voucher_export_request(
                identity.display_name(),
                &from,
                &to,
            ))
            .await?;
        let parsed =
            parse_native_voucher_source_records_with_evidence(&xml, identity.company_guid())?;
        if parsed.records.is_empty() {
            // A native Voucher collection carries no envelope company GUID,
            // so a zero-row response has no per-row identity to bind to the
            // pinned company either -- `parse_native_voucher_source_records_with_evidence`
            // accepts it unauthenticated. Since the voucher request is now
            // filtered by date, a refused period boundary yields the exact
            // same zero-row, byte-identical response as a genuinely empty
            // window. Confirm the pinned company out-of-band with the same
            // GUID-verified, paired book-extent bracket the core window uses
            // for exactly this situation (see `RuntimeTallyConnector::extract_core_window`
            // in connector.rs), instead of accepting the empty result as-is.
            // Paid only here: a non-empty response keeps its existing
            // row-GUID binding and issues no extra request.
            self.fetch_company_book_extent(identity).await.context(
                "empty voucher response could not confirm the pinned company book extent",
            )?;
        }
        Ok(parsed
            .records
            .into_iter()
            .map(|record| record.record)
            .collect())
    }

    pub(crate) fn reset_observed_body_bytes(&self) {
        self.observed_body_bytes
            .store(BODY_BYTES_UNAVAILABLE, Ordering::Release);
    }

    pub(crate) fn observed_body_bytes(&self) -> Option<u64> {
        match self.observed_body_bytes.load(Ordering::Acquire) {
            BODY_BYTES_UNAVAILABLE => None,
            bytes => Some(bytes),
        }
    }

    fn record_observed_body_bytes(&self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX - 1);
        let _ = self.observed_body_bytes.fetch_update(
            Ordering::AcqRel,
            Ordering::Acquire,
            |observed| {
                Some(if observed == BODY_BYTES_UNAVAILABLE {
                    bytes
                } else {
                    observed.max(bytes)
                })
            },
        );
    }

    fn record_observed_encoding(&self, encoding: TallyTextEncoding) {
        let value = match encoding {
            TallyTextEncoding::Utf8 => ENCODING_UTF8,
            TallyTextEncoding::Utf8Bom => ENCODING_UTF8_BOM,
            TallyTextEncoding::Utf16Le => ENCODING_UTF16_LE,
            TallyTextEncoding::Utf16LeBom => ENCODING_UTF16_LE_BOM,
            TallyTextEncoding::Utf16BeBom => ENCODING_UTF16_BE_BOM,
        };
        self.observed_encoding.store(value, Ordering::Release);
    }

    fn observed_encoding_evidence(&self) -> CapabilityEvidence {
        let reason = match self.observed_encoding.load(Ordering::Acquire) {
            ENCODING_UTF8 => "utf8_observed",
            ENCODING_UTF8_BOM => "utf8_bom_observed",
            ENCODING_UTF16_LE => "utf16_le_observed",
            ENCODING_UTF16_LE_BOM => "utf16_le_bom_observed",
            ENCODING_UTF16_BE_BOM => "utf16_be_bom_observed",
            _ => {
                return CapabilityEvidence {
                    state: CapabilityState::Unknown,
                    confidence: EvidenceConfidence::Unknown,
                    safe_reason_code: Some("encoding_not_observed".to_string()),
                };
            }
        };
        CapabilityEvidence {
            state: CapabilityState::Supported,
            confidence: EvidenceConfidence::Observed,
            safe_reason_code: Some(reason.to_string()),
        }
    }
}

fn party_ledger_master_balance_period(
    boundary_profile: DateBoundaryProfile,
    books_from: bridge_tally_core::TallyDate,
    last_voucher_date: bridge_tally_core::TallyDate,
    today: &bridge_tally_core::TallyDate,
) -> Result<
    NativeLedgerSnapshotPeriod,
    bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriodError,
> {
    // A voucher saved under a mistyped far-future date makes the extent's last
    // voucher date that date, and Tally applies a far-future `SVTODATE` as given
    // (protocol reference 11e), so the closing balance would take in everything
    // up to it. The snapshot therefore ends no later than the host's today, like
    // the outstandings read; the source keeps the last voucher date so the
    // workbook can say a later-dated voucher exists. A host clock before
    // `books_from` (a book for a coming year) falls back to `books_from`, never
    // an inverted period.
    let reference = last_voucher_date.min(today.clone()).max(books_from.clone());
    // This workbook must be safe when a capability profile is not cached.
    // The strict Education boundary set is the known common admissible set;
    // choosing the next such date includes the reference rather than silently
    // requesting a refused boundary or shrinking the period.
    let closing_boundary = DateBoundaryProfile::EducationRestricted
        .earliest_boundary_at_or_after(&reference)
        .ok_or(
            bridge_tally_protocol::native_outstandings::NativeLedgerSnapshotPeriodError::UnsupportedBoundary,
        )?;
    NativeLedgerSnapshotPeriod::new(boundary_profile, books_from, closing_boundary)
}

fn normalize_discovered_companies(companies: Vec<TallyCompany>) -> Result<Vec<TallyCompany>, ()> {
    companies
        .into_iter()
        .map(|company| {
            let name = normalize_company_name(&company.name).map_err(|_| ())?;
            let guid = company
                .guid
                .as_deref()
                .map(normalize_company_guid)
                .transpose()
                .map_err(|_| ())?;
            let company_number = company
                .company_number
                .as_deref()
                .map(normalize_company_number)
                .transpose()
                .map_err(|_| ())?;
            let books_from = company
                .books_from
                .as_deref()
                .map(normalize_books_from)
                .transpose()
                .map_err(|_| ())?;
            Ok(TallyCompany {
                name,
                guid,
                company_number,
                books_from,
            })
        })
        .collect()
}

fn normalize_company_number(value: &str) -> Result<String, ()> {
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.len() > 16
        || !trimmed.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(());
    }
    Ok(trimmed.to_string())
}

fn normalize_books_from(value: &str) -> Result<String, ()> {
    let trimmed = value.trim();
    bridge_tally_core::TallyDate::parse(trimmed)
        .map(|_| trimmed.to_string())
        .map_err(|_| ())
}

fn has_complete_company_identity(company: &TallyCompany) -> bool {
    company
        .guid
        .as_deref()
        .is_some_and(|value| !value.is_empty())
        && company
            .company_number
            .as_deref()
            .is_some_and(|value| !value.is_empty())
        && company
            .books_from
            .as_deref()
            .is_some_and(|value| !value.is_empty())
        && !company.name.is_empty()
}

fn unique_company_identities(companies: &[TallyCompany]) -> bool {
    let mut seen = BTreeSet::new();
    companies.iter().all(|company| {
        let (Some(guid), Some(company_number), Some(books_from)) = (
            company.guid.as_deref(),
            company.company_number.as_deref(),
            company.books_from.as_deref(),
        ) else {
            return false;
        };
        seen.insert((
            guid.to_ascii_lowercase(),
            company_number.to_string(),
            company.name.clone(),
            books_from.to_string(),
        ))
    })
}

/// Tally scopes reads by display name, so presentation-equivalent same-GUID
/// books with distinct observed tuples cannot be safely selected.
fn has_presentation_equivalent_guid_siblings(companies: &[TallyCompany]) -> bool {
    companies.iter().enumerate().any(|(index, company)| {
        let Some(guid) = company.guid.as_deref() else {
            return false;
        };
        companies[..index].iter().any(|other| {
            other
                .guid
                .as_deref()
                .is_some_and(|other_guid| other_guid.eq_ignore_ascii_case(guid))
                && company.name.trim().eq_ignore_ascii_case(other.name.trim())
                && (company.name != other.name
                    || company.company_number != other.company_number
                    || company.books_from != other.books_from)
        })
    })
}

/// One master-and-balance pair as read and classified, before it is joined
/// (#679): the whole book's, or one parent part's.
struct PartyLedgerMasterPartRead {
    master: ParsedExport<ParsedSourceRecord<PartyLedgerMasterRecord>>,
    balances: Vec<LedgerSnapshotEntry>,
    foreign: Vec<ForeignCurrencyLedger>,
    mixed: Vec<String>,
    set_aside: BTreeSet<String>,
    master_response_sha256: String,
    master_response_bytes: usize,
    balance_response_sha256: String,
    balance_response_bytes: usize,
}

/// A single read's own response hash and size, unchanged. For several parts the
/// hash is NOT a response hash: it is the SHA-256 of the parts' response hashes
/// joined in part order, so it identifies the set of parts read, and the size is
/// their sum.
fn aggregate_part_evidence<'a>(parts: impl Iterator<Item = (&'a str, usize)>) -> (String, usize) {
    let parts = parts.collect::<Vec<_>>();
    if let [(sha256, bytes)] = parts.as_slice() {
        return ((*sha256).to_owned(), *bytes);
    }
    let joined = parts
        .iter()
        .map(|(sha256, _)| *sha256)
        .collect::<Vec<_>>()
        .join(":");
    let bytes = parts
        .iter()
        .fold(0usize, |total, (_, bytes)| total.saturating_add(*bytes));
    (sha256_hex(joined.as_bytes()), bytes)
}

/// Joins one read's master rows to its balances by `(name, parent)`, leaving out
/// the ledgers set aside for currency, and refuses any row or balance the other
/// side lacks.
fn join_party_ledger_master_part(
    read: PartyLedgerMasterPartRead,
) -> anyhow::Result<Vec<PartyLedgerMasterRow>> {
    let mut balances_by_key = HashMap::new();
    for balance in read.balances {
        let key = ledger_display_key(&balance.name, balance.parent.as_deref());
        if balances_by_key.insert(key, balance).is_some() {
            return Err(anyhow::Error::new(
                PartyLedgerMasterSourceValidationError::DuplicateBalanceDisplayKey,
            ));
        }
    }
    let mut rows = Vec::with_capacity(read.master.records.len());
    for source in read
        .master
        .records
        .into_iter()
        .filter(|source| !read.set_aside.contains(&source.record.ledger.name))
    {
        let key = ledger_display_key(
            &source.record.ledger.name,
            source.record.ledger.parent.nonempty_returned_text(),
        );
        let balance = balances_by_key.remove(&key).ok_or_else(|| {
            anyhow::Error::new(PartyLedgerMasterSourceValidationError::BalanceMissingMasterLedger)
        })?;
        let guid = source.identities.guid.ok_or_else(|| {
            anyhow::Error::new(PartyLedgerMasterSourceValidationError::MasterGuid)
        })?;
        let master_id = source
            .identities
            .master_id
            .ok_or_else(|| anyhow::Error::new(PartyLedgerMasterSourceValidationError::MasterId))?;
        let alter_id = source.alter_id.ok_or_else(|| {
            anyhow::Error::new(PartyLedgerMasterSourceValidationError::MasterAlterId)
        })?;
        let master_opening = source
            .record
            .ledger
            .opening_balance
            .as_deref()
            .ok_or_else(|| {
                anyhow::Error::new(PartyLedgerMasterSourceValidationError::MasterOpeningBalance)
            })?;
        if !party_ledger_master_openings_agree(master_opening, &balance.opening_balance)? {
            return Err(anyhow::Error::new(
                PartyLedgerMasterSourceValidationError::OpeningBalancesDisagreed,
            ));
        }
        rows.push(PartyLedgerMasterRow {
            name: source.record.ledger.name,
            parent: source.record.ledger.parent,
            party_gstin: source.record.ledger.party_gstin,
            fields: source.record.fields,
            guid,
            master_id,
            alter_id,
            opening_balance: balance.opening_balance,
            closing_balance: balance.closing_balance,
        });
    }
    if !balances_by_key.is_empty() {
        return Err(anyhow::Error::new(
            PartyLedgerMasterSourceValidationError::BalanceLedgerAbsentFromMasterEvidence,
        ));
    }
    Ok(rows)
}

fn party_ledger_request_commitment(requests: &[String]) -> String {
    let hashes = requests
        .iter()
        .map(|request| {
            sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
                request,
            ))
        })
        .collect::<Vec<_>>();
    sha256_hex(hashes.join(":").as_bytes())
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn safe_connection_failure_code(error: &anyhow::Error) -> &'static str {
    if let Some(transport) = error.downcast_ref::<TallyTransportError>() {
        return transport.safe_code();
    }
    let message = error.to_string().to_ascii_lowercase();
    if message.contains("cancel") {
        "request_cancelled"
    } else if message.contains("queue deadline") {
        "endpoint_queue_deadline_exceeded"
    } else if message.contains("circuit") {
        "endpoint_circuit_open"
    } else if message.contains("response exceeded") {
        "response_size_limit_exceeded"
    } else if message.contains("decode") || message.contains("utf") {
        "response_encoding_invalid"
    } else {
        "endpoint_unreachable"
    }
}

pub(crate) fn canonical_loopback_origin(config: &TallyConfig) -> anyhow::Result<String> {
    Ok(transport_canonical_origin(config)?)
}

#[cfg(test)]
fn tally_endpoint(config: &TallyConfig, path: &str) -> anyhow::Result<reqwest::Url> {
    let mut url = reqwest::Url::parse(&canonical_loopback_origin(config)?)?;
    url.set_path(path);
    Ok(url)
}

#[cfg(test)]
fn decode_xml_bytes(bytes: Vec<u8>) -> anyhow::Result<String> {
    bridge_tally_protocol::decode_xml_bytes(bytes)
}

fn detect_product(text: &str) -> TallyProduct {
    let trimmed = text.trim();
    let marker = |expected: &str| {
        trimmed.eq_ignore_ascii_case(expected)
            || trimmed.eq_ignore_ascii_case(&format!("<RESPONSE>{expected}</RESPONSE>"))
    };
    if marker("TallyPrime Server is Running") {
        TallyProduct::TallyPrime
    } else if marker("Tally ERP 9 Server is Running") || marker("Tally.ERP 9 Server is Running") {
        TallyProduct::TallyErp9
    } else {
        TallyProduct::Unknown
    }
}

#[cfg(test)]
#[path = "connection_tests.rs"]
pub(crate) mod tests;
