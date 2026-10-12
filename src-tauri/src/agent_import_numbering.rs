//! Which voucher types a voucher sent without a number may go into (bridge#724).
//!
//! A native post never carries a voucher number, so Tally numbers each voucher
//! by its voucher type's own voucher numbering. Into a type numbered Manual, a
//! second voucher sent without a number took the number already used, or was
//! refused whole with no Tally text; into a type numbered Automatic with
//! "Retain Original Voucher No.", it took the next number (protocol reference
//! §9.8). So a voucher type is admitted only when its one series reads
//! Automatic, whatever its duplicate setting; every other reading, and a type
//! the read does not hold, refuses. The read does not see the insertion
//! setting, so Automatic set to renumber is admitted too, unmeasured. The
//! company's master mark is read first; the voucher-type masters rows must sit
//! at or under it, and the post compares it again just before it is sent, so a
//! master saved after it, a voucher type included, refuses the post
//! (`post_masters_moved`). A build has no later compare: its check holds for
//! the moment of the read.
use super::location::{self, LoadedCompanyMarks};
use super::*;
use crate::tally::approved_import::ApprovedImportAdmissionError;
use crate::tally::runtime::VoucherNumberingRead;
use crate::tally::VerifiedCompanyIdentity;
use bridge_tally_protocol::native_voucher_type_numbering::{
    DuplicateSetting, SeriesNumberingMethod, VoucherTypeNumbering,
};

/// The refusal of a voucher type whose voucher numbering is not Automatic.
pub(in crate::agent) const NUMBERING_NOT_AUTOMATIC: &str = "import_voucher_numbering_not_automatic";

/// What one numbering read admitted: the voucher types every voucher of the
/// batch goes into, each read Automatic, for one company, under one master
/// mark. Neither `Clone` nor serialisable, so it cannot outlive the pass whose
/// read made it.
#[derive(Debug)]
pub(in crate::agent) struct NumberingAdmission {
    company_guid: String,
    master_mark: u64,
}

/// One refused voucher type and what the read said of it. `observed` states
/// the reading, never a cause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::agent) struct RefusedType {
    pub(in crate::agent) voucher_type: String,
    pub(in crate::agent) observed: Value,
}

/// The voucher types that did not read Automatic, in batch order; never empty.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::agent) struct NumberingRefusal(pub(in crate::agent) Vec<RefusedType>);

impl NumberingRefusal {
    pub(in crate::agent) const fn code(&self) -> &'static str {
        NUMBERING_NOT_AUTOMATIC
    }

    /// The refused voucher types as the result lists them.
    pub(in crate::agent) fn refused_types(&self) -> Value {
        Value::Array(
            self.0
                .iter()
                .map(|row| json!({"voucher_type": row.voucher_type, "observed": row.observed}))
                .collect(),
        )
    }
}

/// What the read said of one voucher type. A method Tally printed outside the
/// measured set is named only as unrecognised: its raw text is Tally's, and
/// nothing is read from it.
fn observed_json(numbering: Option<&VoucherTypeNumbering>) -> Value {
    match numbering {
        None => json!({"observation": "absent"}),
        Some(VoucherTypeNumbering::Observed { method, duplicates }) => json!({
            "observation": "observed",
            "method": match method {
                SeriesNumberingMethod::Automatic => "automatic",
                SeriesNumberingMethod::Manual => "manual",
                SeriesNumberingMethod::AutomaticManualOverride => "automatic_manual_override",
                SeriesNumberingMethod::Unrecognised(_) => "unrecognised",
            },
            "duplicates": match duplicates {
                DuplicateSetting::Prevented => "prevented",
                DuplicateSetting::NotPrevented => "not_prevented",
            },
        }),
        Some(VoucherTypeNumbering::SeriesCount(count)) => {
            json!({"observation": "series_count", "count": count})
        }
        Some(VoucherTypeNumbering::FieldInvalid(code)) => {
            json!({"observation": "field_invalid", "code": code})
        }
    }
}

/// Admits `voucher_types`, the voucher types of the vouchers sent without a
/// number, in batch order, against `read`: each must be held by the read (by
/// ASCII case fold, as the read keys its names) and read Automatic. Every
/// refused type is named once, in the order it first appears.
pub(in crate::agent) fn admit_numbering<'a>(
    read: VoucherNumberingRead,
    voucher_types: impl IntoIterator<Item = &'a str>,
) -> Result<NumberingAdmission, NumberingRefusal> {
    let mut seen: Vec<&str> = Vec::new();
    let mut refused = Vec::new();
    for voucher_type in voucher_types {
        if seen
            .iter()
            .any(|known| known.eq_ignore_ascii_case(voucher_type))
        {
            continue;
        }
        seen.push(voucher_type);
        let numbering = read
            .types()
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(voucher_type))
            .map(|(_, numbering)| numbering);
        if !matches!(
            numbering,
            Some(VoucherTypeNumbering::Observed {
                method: SeriesNumberingMethod::Automatic,
                ..
            })
        ) {
            refused.push(RefusedType {
                voucher_type: voucher_type.to_string(),
                observed: observed_json(numbering),
            });
        }
    }
    if !refused.is_empty() {
        return Err(NumberingRefusal(refused));
    }
    Ok(NumberingAdmission {
        company_guid: read.company_guid().to_string(),
        master_mark: read.master_mark(),
    })
}

impl NumberingAdmission {
    /// As the post is sent: the target's master mark in the snapshot the
    /// queue's binding reads began with must equal the mark the numbering was
    /// read under. The queue then holds that snapshot equal to the last one
    /// before the POST, so no master, a voucher type included, was saved
    /// between the numbering read and the POST. A move is `MastersMoved`; a
    /// snapshot without exactly one target row, or an admission of another
    /// company, cannot be compared and is `MastersUnconfirmed`.
    pub(in crate::agent) fn admit_at_binding(
        &self,
        at_binding: &[LoadedCompanyMarks],
        company_guid: &str,
        company_name: &str,
    ) -> Result<(), ApprovedImportAdmissionError> {
        if !self.company_guid.eq_ignore_ascii_case(company_guid) {
            return Err(ApprovedImportAdmissionError::MastersUnconfirmed);
        }
        match location::target_master_mark(at_binding, company_guid, company_name) {
            Some(mark) if mark == self.master_mark => Ok(()),
            Some(_) => Err(ApprovedImportAdmissionError::MastersMoved),
            None => Err(ApprovedImportAdmissionError::MastersUnconfirmed),
        }
    }
}

/// Why a batch's voucher numbering was not admitted. A read that failed keeps
/// its own code (a busy endpoint, a pair that changed, a numbering answer that
/// does not join): it says nothing about the numbering.
pub(in crate::agent) enum NumberingFailure {
    Read(ToolFailure),
    Refused(NumberingRefusal),
}

impl From<NumberingFailure> for ToolFailure {
    fn from(failure: NumberingFailure) -> Self {
        match failure {
            NumberingFailure::Read(failure) => failure,
            NumberingFailure::Refused(refusal) => ToolFailure::from(refusal.code().to_string()),
        }
    }
}

impl Server {
    /// Reads the voucher numbering under `master_mark`, the company's
    /// `ALTMSTID` read before this call, and admits `voucher_types` against it.
    pub(in crate::agent) async fn admit_voucher_numbering<'a>(
        &self,
        identity: &VerifiedCompanyIdentity,
        master_mark: u64,
        voucher_types: impl IntoIterator<Item = &'a str>,
        accumulated: &mut Evidence,
    ) -> Result<NumberingAdmission, NumberingFailure> {
        let read = self
            .runtime
            .fetch_voucher_type_numbering(self.tally_config(), identity, master_mark)
            .await
            // The caller adds the evidence gathered before this read, as it
            // does for every failure.
            .map_err(|error| {
                NumberingFailure::Read(ToolFailure::from_runtime("masters_read_failed", error))
            })?;
        *accumulated = combine_evidence(
            accumulated.clone(),
            evidence_from_runtime_read(read.evidence().clone()),
        );
        admit_numbering(read, voucher_types).map_err(NumberingFailure::Refused)
    }

    /// Before the approval: the company's master mark first, then the voucher
    /// numbering under it, for every voucher of a native post (none carries a
    /// number).
    pub(super) async fn admit_numbering_for_post(
        &self,
        company: &bridge_tally_protocol::TallyCompany,
        identity: &VerifiedCompanyIdentity,
        payload: &ImportPayload,
        accumulated: &mut Evidence,
    ) -> Result<NumberingAdmission, NumberingFailure> {
        let (mark, evidence) = self
            .pre_import_mark(company, identity)
            .await
            .map_err(NumberingFailure::Read)?;
        *accumulated = combine_evidence(accumulated.clone(), evidence);
        let master_mark = mark.master_value.ok_or_else(|| {
            NumberingFailure::Read(ToolFailure::from("pre_import_mark_unobserved".to_string()))
        })?;
        self.admit_voucher_numbering(
            identity,
            master_mark,
            payload
                .vouchers
                .iter()
                .map(|voucher| voucher.voucher_type.as_str()),
            accumulated,
        )
        .await
    }
}
