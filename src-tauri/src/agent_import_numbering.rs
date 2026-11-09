//! Which voucher types a voucher may go into, by whether it carries its own
//! number (bridge#724).
//!
//! A native post never carries a voucher number, so Tally numbers each voucher
//! by its voucher type's own voucher numbering. Into a type numbered Manual, a
//! second voucher sent without a number took the number already used, or was
//! refused whole with no Tally text; into a type numbered Automatic with
//! "Retain Original Voucher No.", it took the next number (protocol reference
//! §9.8). With a type numbered Automatic set to "Renumber Vouchers", numbers
//! already in the book changed when a voucher with an earlier date was entered
//! by hand, and again when one was sent by import without a number: no
//! voucher's AlterID moved, the company's voucher mark rose by one only, and
//! the import's answer read CREATED 1, ALTERED 0 (one book, one voucher type,
//! `TallyPrime` 7.1 Silver: protocol reference §9.8 records both runs, and
//! `VOUCHER_TYPE_NUMBERING_M8R_CAPTURE_PROVENANCE.md` in the protocol crate's
//! fixtures says where each is kept). A file of two such Receipts was numbered
//! by date and changed the numbers of four, and a Journal imported without a
//! number and dated between two others was numbered by its date and moved the
//! later one (§9.8). Not measured on such a type: a file of more than two
//! vouchers, a mixed file, a Journal that carries a number or is dated after
//! every other, Payment and Contra, Gold, other releases, and the same through
//! this crate's own post (the imports were sent raw, in the shape its builder
//! sends).
//!
//! So a voucher without a number is admitted only into a voucher type whose
//! one series reads Automatic with the sub-method `Auto Retain`, whatever its
//! duplicate setting. A voucher that carries its own number (a build's
//! Journal can) is admitted there too, since there Tally discards a supplied
//! number and assigns its own (protocol reference §9.8),
//! which leaves it a voucher without one; and into a type that reads Manual
//! with duplicate numbers prevented, where Tally kept the number a Journal
//! carried and refused whole a Journal carrying a number that a Journal
//! already held, with no reason in its answer (§9.8). Under Manual with
//! duplicates allowed a Payment carrying a held number was created as a second
//! voucher with it (§9.8; not measured for a Journal), which nothing here or in
//! `verify_import` would notice, so a numbered voucher is refused there. A numbered voucher is refused on a type set to "Renumber Vouchers":
//! there Tally discarded the number a Receipt carried and gave it the number
//! of its place in date order, and the later-dated Receipts' numbers went up
//! by one (§9.8), as for a voucher sent without one.
//! Every other
//! reading, and a type the read does not hold, refuses: a type that does not
//! read Automatic as `import_voucher_numbering_not_automatic`, and an
//! Automatic one whose sub-method reads `Auto Renumber`, is missing or is not
//! recognised as `import_voucher_numbering_not_retain`. A batch that holds
//! both is refused as `import_voucher_numbering_not_automatic`. The
//! sub-method of a type that is not Automatic
//! is not read: a Manual series printed `Auto Retain` too. The
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
    DuplicateSetting, NumberingSubMethod, SeriesNumberingMethod, VoucherTypeNumbering,
};

/// The refusal of a voucher type whose voucher numbering is not Automatic.
pub(in crate::agent) const NUMBERING_NOT_AUTOMATIC: &str = "import_voucher_numbering_not_automatic";

/// The refusal of a voucher type whose voucher numbering is Automatic and
/// whose sub-method did not read `Auto Retain`.
pub(in crate::agent) const NUMBERING_NOT_RETAIN: &str = "import_voucher_numbering_not_retain";

/// The reading that admits, as each next step names it.
macro_rules! admitted {
    () => {
        "whose voucher numbering it read as Automatic with existing numbers retained (the series reads Auto Retain; on the voucher type's screen, Retain Original Voucher No.)."
    };
}

/// What a build writes a voucher into, with or without its own number. It
/// says nothing of what Tally does with a number: that is said only where it
/// was measured, of a voucher that is created.
macro_rules! written_into {
    () => {
        concat!(
            "ComplyEaze Bridge writes a voucher into a voucher type ",
            admitted!(),
            " A voucher that carries its own number is also written into one read as Manual with duplicate numbers prevented; no voucher is written into any other."
        )
    };
}

/// What `observed` holds, then what was seen under each refused reading. One
/// text for the three next steps, so a new measurement is written once.
macro_rules! observed_and_seen {
    () => {
        "observed says what was read, and under automatic its sub_method is auto_renumber, absent (Tally printed none) or unrecognised. Under Manual voucher numbering, a voucher sent without a number was seen to take a number already used, or to be refused. Under Automatic set to Renumber Vouchers, a voucher sent by import without a number and dated before existing vouchers was seen to change the numbers of vouchers already in the book, while the import's answer reported nothing altered; a voucher entered by hand with an earlier date did the same (one book, TallyPrime 7.1 Silver: by import on a Receipt and a Journal, by hand on a Receipt)."
    };
}

/// What no next step allows.
macro_rules! never_to_get_past {
    () => {
        "Do not change a voucher to another voucher type, do not add a voucher number to it, and do not change a voucher type's voucher numbering to get past this: that is the user's decision, and it can change the numbers of vouchers already entered."
    };
}

/// The post's next step under either refusal code: a batch can hold a voucher
/// type of each.
pub(in crate::agent) const POST_NEXT_STEP: &str = concat!(
    "Nothing was sent. ComplyEaze Bridge posts vouchers without a voucher number, so Tally numbers each one by its voucher type, and it posts only into voucher types ",
    admitted!(),
    " The voucher types in error.refused_types did not read so; ",
    observed_and_seen!(),
    " ComplyEaze Bridge does not yet number vouchers itself, so it cannot post them into those voucher types. Leave the vouchers of those voucher types out: build the other vouchers again without them and post that batch, or build them again once ComplyEaze Bridge supports their numbering. A voucher type that was not among those Tally returned, or whose numbering settings could not be read, is not cured by that: check its name and settings with a masters read of voucher_types. Do not have this batch's written file imported through Tally's Import menu. ",
    never_to_get_past!()
);

/// The build's next step under either refusal code, for vouchers that are not
/// yet in the book.
pub(in crate::agent) const BUILD_NEXT_STEP: &str = concat!(
    "No file was written. Tally numbers a voucher that carries no number by its voucher type; a number a voucher carries it keeps under Manual numbering, and under Automatic numbering discards, assigning its own. For that reason ",
    written_into!(),
    " The voucher types in refused_types did not read so; ",
    observed_and_seen!(),
    " ComplyEaze Bridge does not yet number vouchers itself, so it cannot write them for those voucher types. Leave the vouchers of those voucher types out and build the other vouchers again without them (from a parse_bank_statement proposals file, parse the statement again with from and to that leave out the days holding them; never rename a statement row), or build them again once ComplyEaze Bridge supports their numbering. A voucher type that was not among those Tally returned, or whose numbering settings could not be read, is not cured by that: check its name and settings with a masters read of voucher_types. ",
    never_to_get_past!()
);

/// The build's next step when the refused build is an amendment. An amendment
/// alters vouchers in place, and what an alter does to a voucher's number is
/// not measured, so the step does not say how Tally numbers them. The refusal
/// comes before the amendment's own check of the book, so whether the earlier
/// batch's vouchers are in the book is not known here: the step says so and
/// sends the caller to `verify_import`, which says which of them are in the
/// book; neither is entered again by hand.
pub(in crate::agent) const BUILD_AMENDMENT_NEXT_STEP: &str = concat!(
    "No file was written. This build amends an earlier batch (amends_batch_id), and whether that batch's vouchers are in the book was not checked before this refusal. The vouchers of this amendment that carry no number are written without one. ",
    written_into!(),
    " The voucher types in refused_types did not read so; ",
    observed_and_seen!(),
    " Call verify_import for the batch this build amends. A voucher it finds in the book is never entered again: a second entry duplicates it. A voucher it does not find may still be in the book (its narration edited in Tally, for one), so it is not built as a new voucher either. ComplyEaze Bridge does not yet number vouchers itself: leave the vouchers of those voucher types out of this amendment, or build it again once ComplyEaze Bridge supports their numbering. A voucher type that was not among those Tally returned, or whose numbering settings could not be read, is not cured by that: check its name and settings with a masters read of voucher_types. ",
    never_to_get_past!()
);

/// The least response budget at which a numbering refusal carries its next
/// step and, on a post, the message that names the refused voucher types.
/// The general floor holds a step of a few hundred bytes; these are longer,
/// and an answer over its budget is replaced whole, so under this budget the
/// refusal goes out with its code and refused types only. The budget tests
/// hold an answer naming all four voucher types to it.
pub(in crate::agent) const GUIDANCE_MIN_RESPONSE_BUDGET: usize =
    2 * crate::agent::REMEDIATION_MIN_RESPONSE_BUDGET;

/// What one numbering read admitted: the voucher types every voucher of the
/// batch goes into, each read as its vouchers may go into (Automatic with
/// `Auto Retain`; Manual too, for a voucher that carries a number), for one company,
/// under one master mark. Neither `Clone` nor serialisable, so it cannot
/// outlive the pass whose read made it.
#[derive(Debug)]
pub(in crate::agent) struct NumberingAdmission {
    company_guid: String,
    master_mark: u64,
}

/// Whether a voucher carries its own voucher number. A blank one is none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::agent) enum OwnNumber {
    Carried,
    Absent,
}

impl OwnNumber {
    pub(in crate::agent) fn of(voucher_number: Option<&str>) -> Self {
        match voucher_number {
            Some(number) if !number.trim().is_empty() => Self::Carried,
            Some(_) | None => Self::Absent,
        }
    }
}

/// Which of the two refusals a voucher type's reading is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::agent) enum RefusalReason {
    /// Not read as Automatic at all, or not held by the read. A voucher that
    /// carries its own number is not refused so for a type that reads Manual.
    NotAutomatic,
    /// Automatic, with a sub-method that is not `Auto Retain`.
    NotRetain,
}

/// One refused voucher type and what the read said of it. `observed` states
/// the reading, never a cause; `reason` decides the refusal's code and is not
/// returned, since `observed` already says it. `in_words` is the same reading
/// for a person, to follow the voucher type's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::agent) struct RefusedType {
    pub(in crate::agent) voucher_type: String,
    pub(in crate::agent) reason: RefusalReason,
    pub(in crate::agent) observed: Value,
    pub(in crate::agent) in_words: &'static str,
}

/// The voucher types that were not admitted, in batch order; never empty.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::agent) struct NumberingRefusal(pub(in crate::agent) Vec<RefusedType>);

impl NumberingRefusal {
    /// One code for the batch: `not_retain` only when every refused type is
    /// refused for its sub-method.
    pub(in crate::agent) fn code(&self) -> &'static str {
        if self
            .0
            .iter()
            .any(|row| row.reason == RefusalReason::NotAutomatic)
        {
            NUMBERING_NOT_AUTOMATIC
        } else {
            NUMBERING_NOT_RETAIN
        }
    }

    /// The refusal for a person who is shown only a message (the desktop's
    /// Journal screen): each refused voucher type with what was read of it,
    /// then what the person can do inside ComplyEaze Bridge: it never sends
    /// the person to Tally's voucher entry screen. It is said of a post,
    /// which sends no voucher number. The next steps above say the same to an
    /// assistant, at length.
    pub(in crate::agent) fn message(&self) -> String {
        let read = self
            .0
            .iter()
            .map(|row| format!("{} {}", row.voucher_type, row.in_words))
            .collect::<Vec<_>>()
            .join("; ");
        format!(
            "Nothing was posted. ComplyEaze Bridge sends a voucher without a voucher number, so it \
             posts only into a voucher type numbered Automatic with Retain Original Voucher No. \
             Read from Tally: {read}. ComplyEaze Bridge does not yet number vouchers itself, so \
             these were not posted: leave them out of this batch and post the others, if any, or \
             post them once ComplyEaze Bridge supports their numbering. Do not change a voucher type's \
             voucher numbering only to post them: that can change the numbers of vouchers already \
             entered."
        )
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

/// One reading in the three forms a refusal needs.
struct Reading {
    /// Why the reading refuses the voucher, or `None` where it admits it.
    reason: Option<RefusalReason>,
    /// What the read said, as a result lists it.
    observed: Value,
    /// The same for a person, to follow the voucher type's name.
    in_words: &'static str,
}

/// What one voucher type's reading means for a voucher that does or does not
/// carry its own number: its verdict, what a result says of it and what a
/// person is told. Every reading is named here once, so a reading added to
/// the read's types does not compile until it is given all three, and none
/// admits by default. The number decides under Manual only: a voucher without
/// one is refused there, and one with a number is admitted only where
/// duplicate numbers are prevented. Under any other method the duplicate
/// setting does not decide.
/// A method or sub-method Tally printed outside the measured set is named
/// only as unrecognised: its raw text is Tally's, and nothing is read from
/// it. `sub_method` is given under `automatic` only, the one method it is
/// read under.
fn reading(numbering: Option<&VoucherTypeNumbering>, number: OwnNumber) -> Reading {
    let not_automatic = |observed: Value, in_words: &'static str| Reading {
        reason: Some(RefusalReason::NotAutomatic),
        observed,
        in_words,
    };
    match numbering {
        None => not_automatic(
            json!({"observation": "absent"}),
            "was not among the voucher types Tally returned",
        ),
        Some(VoucherTypeNumbering::Observed { method, duplicates }) => {
            let prevented = matches!(duplicates, DuplicateSetting::Prevented);
            let duplicates = match duplicates {
                DuplicateSetting::Prevented => "prevented",
                DuplicateSetting::NotPrevented => "not_prevented",
            };
            let observed = |method: &str| json!({"observation": "observed", "method": method, "duplicates": duplicates});
            match method {
                SeriesNumberingMethod::Automatic(sub_method) => {
                    let not_retain = Some(RefusalReason::NotRetain);
                    let (reason, sub_method, in_words) = match sub_method {
                        NumberingSubMethod::AutoRetain => (
                            None,
                            "auto_retain",
                            "is numbered Automatic with Retain Original Voucher No",
                        ),
                        NumberingSubMethod::AutoRenumber => (
                            not_retain,
                            "auto_renumber",
                            "is numbered Automatic with Renumber Vouchers",
                        ),
                        NumberingSubMethod::Unrecognised(_) => (
                            not_retain,
                            "unrecognised",
                            "is numbered Automatic, with a setting for existing numbers that ComplyEaze Bridge does not recognise",
                        ),
                        NumberingSubMethod::Absent => (
                            not_retain,
                            "absent",
                            "is numbered Automatic, and Tally did not say what it does with existing numbers",
                        ),
                    };
                    let mut said = observed("automatic");
                    said["sub_method"] = json!(sub_method);
                    Reading {
                        reason,
                        observed: said,
                        in_words,
                    }
                }
                SeriesNumberingMethod::Manual => match (number, prevented) {
                    (OwnNumber::Carried, true) => Reading {
                        reason: None,
                        observed: observed("manual"),
                        in_words: "is numbered Manual",
                    },
                    (OwnNumber::Carried, false) => not_automatic(
                        observed("manual"),
                        "is numbered Manual and does not prevent duplicate voucher numbers",
                    ),
                    (OwnNumber::Absent, _) => {
                        not_automatic(observed("manual"), "is numbered Manual")
                    }
                },
                SeriesNumberingMethod::AutomaticManualOverride => not_automatic(
                    observed("automatic_manual_override"),
                    "is numbered Automatic (Manual Override)",
                ),
                SeriesNumberingMethod::Unrecognised(_) => not_automatic(
                    observed("unrecognised"),
                    "has a numbering method ComplyEaze Bridge does not recognise",
                ),
            }
        }
        Some(VoucherTypeNumbering::SeriesCount(count)) => not_automatic(
            json!({"observation": "series_count", "count": count}),
            "does not have exactly one number series",
        ),
        Some(VoucherTypeNumbering::FieldInvalid(code)) => not_automatic(
            json!({"observation": "field_invalid", "code": code}),
            "has numbering settings ComplyEaze Bridge could not read",
        ),
    }
}

/// Admits `vouchers`, each voucher's voucher type and whether it carries its
/// own number, in batch order, against `read`: each type must be held by the
/// read (by ASCII case fold, as the read keys its names) and read Automatic
/// with `Auto Retain`, or Manual with duplicate numbers prevented for a voucher
/// that carries a number. Every
/// refused type is named once, in the order it first appears.
pub(in crate::agent) fn admit_numbering<'a>(
    read: VoucherNumberingRead,
    vouchers: impl IntoIterator<Item = (&'a str, OwnNumber)>,
) -> Result<NumberingAdmission, NumberingRefusal> {
    let mut seen: Vec<(&str, OwnNumber)> = Vec::new();
    let mut refused: Vec<RefusedType> = Vec::new();
    for (voucher_type, number) in vouchers {
        if seen
            .iter()
            .any(|(known, of)| *of == number && known.eq_ignore_ascii_case(voucher_type))
        {
            continue;
        }
        seen.push((voucher_type, number));
        let numbering = read
            .types()
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(voucher_type))
            .map(|(_, numbering)| numbering);
        let reading = reading(numbering, number);
        // A type both a numbered and a numberless voucher go into is named
        // once: where both refuse, they refuse alike.
        let named = refused
            .iter()
            .any(|row| row.voucher_type.eq_ignore_ascii_case(voucher_type));
        if let (Some(reason), false) = (reading.reason, named) {
            refused.push(RefusedType {
                voucher_type: voucher_type.to_string(),
                reason,
                observed: reading.observed,
                in_words: reading.in_words,
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
    /// An admission read under `master_mark` for `company_guid`, for a test
    /// that never reaches the numbering check or sets its mark by hand.
    #[cfg(test)]
    pub(in crate::agent) fn for_test(company_guid: &str, master_mark: u64) -> Self {
        Self {
            company_guid: company_guid.to_string(),
            master_mark,
        }
    }

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
    /// `ALTMSTID` read before this call, and admits `vouchers` against it.
    pub(in crate::agent) async fn admit_voucher_numbering<'a>(
        &self,
        identity: &VerifiedCompanyIdentity,
        master_mark: u64,
        vouchers: impl IntoIterator<Item = (&'a str, OwnNumber)>,
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
        admit_numbering(read, vouchers).map_err(NumberingFailure::Refused)
    }

    /// Before the approval: the company's master mark first, then the voucher
    /// numbering under it, for every voucher of a native post, each under the
    /// type it is filed in: without a number, except an invoice, which is
    /// posted with its own (`ImportVoucher::numbering_at_post`).
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
                .map(|voucher| voucher.numbering_at_post()),
            accumulated,
        )
        .await
    }
}
