//! The read-back after a master POST, and the answer it makes.
//!
//! One function serves both moments: straight after the POST answered
//! ([`Moment::JustPosted`]) and a later call for a batch the journal shows
//! dispatched ([`Moment::Reconcile`]). It only reads: nothing here can send a
//! create, so a reconcile can never repeat one.
//!
//! The reads, in this order, each failing on its own:
//!
//! 1. every loaded company's marks, by the voucher path's own after-POST read
//!    (`read_marks_after_post`);
//! 2. the ledger index and the group index, each one paired, byte-equal read;
//! 3. the party-ledger master of the item's parent group alone, only when the
//!    paired index shows one new ledger of the item's name for it to describe.
//!    Its rows are projected onto [`PartyView`] at once: PAN, bank details,
//!    e-mail, phone and address are never copied out of the parser's row.
//!
//! [`judge`] decides; the verdict is appended to the masters journal (the
//! first terminal verdict of a batch is its last: the journal refuses a later
//! one), and the answer reports the state, the GUID when found and each check.
use super::super::master_ledger::{read_pre_image, MasterMark, MasterVerdict, MasterVerdictState};
use super::verdict::{
    judge, party_read_needed, Facts, Judgement, Moment, OrderCoverage, PartyRead, PartyView,
    ReadBack,
};
use super::*;
use crate::tally::VerifiedCompanyIdentity;
use bridge_tally_core::master_name_index::IndexedMaster;
use bridge_tally_protocol::native_outstandings::NativeLedgerExportPeriod;
use bridge_tally_protocol::parent_partition::{
    ParentObservation, ParentPart, ParentPartition, PartitionLimits,
};
use std::collections::BTreeMap;

/// The next step after each state. The two loud ones tell the person to check
/// Tally and not to retry.
pub(super) const NEXT_VERIFIED: &str = "The read-back found exactly one ledger of this name, new, with the parent, tax fields and GSTIN as sent and no opening balance (bill-wise is not read back), no ledger or group that existed before changed, and no other loaded company's master mark moved. Nothing more is needed for this batch; do not post it again.";
pub(super) const NEXT_INTO_EXISTING: &str = "Open the company in Tally now and check the ledger of this name, its group and the ledgers around it: a ledger that existed before may have been changed. Do not retry and do not post this batch again.";
pub(super) const NEXT_ELSEWHERE: &str = "Open Tally now and check every loaded company: the change may have landed in a company other than the one approved. Do not retry and do not post this batch again.";
pub(super) const NEXT_UNATTRIBUTED: &str = "A later read of the book found the ledger of this name with the parent, tax fields and GSTIN as sent and no opening balance (bill-wise is not read back), and its ALTERID shows it as the first master change in this company after ComplyEaze Bridge's last read of it before sending the create, and not changed since. ComplyEaze Bridge has no answer from Tally to the create on file (it could not be recorded, or Tally never answered), so it cannot show that this change was its own create and not the same ledger made by someone else: check the ledger in Tally. That read also does not show that Tally will not still run a create that ComplyEaze Bridge stopped waiting for, which would change this ledger again. This batch is closed, and nothing is sent again from it; do not post it again.";
pub(super) const NEXT_ABSENT: &str = "A later read of the book found no ledger of this name, and the company's master mark had not moved. This batch is closed, and nothing is sent again from it. That read also fits a create that Tally rejected. It was sent after the create's send ended, and Tally has been seen to serve such a read only after the create ran, but only for requests on new connections: it does not show that Tally will not still run a create that ComplyEaze Bridge stopped waiting for. To create the ledger, first make sure Tally is idle and answering, then call prepare_ledger_creates for a new batch: it needs a new approval.";
/// The read found the name absent after another master change: settled on the
/// same basis as `NEXT_ABSENT`, with what the moved mark does not show.
pub(super) const NEXT_ABSENT_AFTER_CHANGE: &str = "A later read of the book found no ledger of this name and no ledger or group that was not there before the create was sent, while the company's master mark had moved forward since then. Whether the create ran (and its ledger was later deleted) or never ran is not known. This batch is closed, and nothing is sent again from it. The read was sent after the create's send ended, and Tally has been seen to serve such a read only after the create ran, but only for requests on new connections: it does not show that Tally will not still run a create that ComplyEaze Bridge stopped waiting for. To create the ledger, first make sure Tally is idle and answering, then call prepare_ledger_creates for a new batch: it needs a new approval.";
/// The step for a failed post whose own journal read also failed, so that
/// whether a send was recorded is not known. A later call is the right one
/// either way: with a send on file it only reads the book back, and with none
/// it asks for approval again before anything is sent.
pub(super) const NEXT_SEND_NOT_KNOWN: &str = "Call post_ledger_creates with this batch_id again. If a send of this batch was recorded, that call only reads the book back and sends nothing; if none was, it asks for approval again before anything is sent; if ComplyEaze Bridge holds no such batch, it is refused as master_batch_not_found and nothing is sent. Do not create the ledger in Tally by hand meanwhile: a create that has not run yet would overwrite it.";
pub(super) const NEXT_UNCONFIRMED: &str = "The ledger could not be read back this time. Call post_ledger_creates with this batch_id again once Tally is idle to read it back (it sends nothing), or look at the company's ledgers in Tally. Do not create the ledger in Tally by hand meanwhile: a create that has not run yet would overwrite it. Never post this batch again.";
/// The read-back found no ledger of this name, and the outcome is still open.
/// A read that found the name absent and did not settle: nothing more can be
/// settled now, and the name stays held.
pub(super) const NEXT_ABSENT_NOT_SETTLED: &str = "The read-back found no ledger of this name, and the outcome of this batch is not settled (see the reasons). ComplyEaze Bridge cannot settle it now: a create it sent may still be waiting in Tally. While the batch stays open, ComplyEaze Bridge keeps this ledger name held in every company this data folder holds batches for, and will not create it again. Calling post_ledger_creates with this batch_id again, once Tally is idle, sends nothing: it reads the book back, shows whether a ledger of this name now exists, and judges one that has appeared. Never post this batch again.";
/// The read found the name absent on a Tally where what an absent state rests
/// on was not measured: the batch stays open.
pub(super) const NEXT_ABSENT_UNCOVERED: &str = "The read-back found no ledger of this name, but ComplyEaze Bridge closes a batch on an absent ledger only on TallyPrime 7.2 Silver, licensed, the one Tally where the order it rests on (a read sent after the create was served after it) was measured, and the reasons say how this Tally differs. The batch stays open, and ComplyEaze Bridge keeps this ledger name held in every company this data folder holds batches for: a create it sent may still be waiting in Tally. Calling post_ledger_creates with this batch_id again sends nothing and reads the book back again. Check the company's ledgers in Tally. Never post this batch again.";
/// A ledger of this name is in the book and the read-back could not verify it.
pub(super) const NEXT_PRESENT_NOT_VERIFIED: &str = "A ledger of this name is in the book, but ComplyEaze Bridge could not verify that it is the one this batch sent (see the checks and reasons). There is nothing to create: compare the ledger in Tally with this batch by eye. Never post this batch again.";
/// The book was not compared: the batch's company is not loaded, was renamed,
/// or the set of loaded companies is not the one the create was recorded under.
pub(super) const NEXT_LOAD_SAME_COMPANIES: &str = "The book could not be compared with the one the create was recorded under: the batch's company is not loaded in Tally, was renamed, or the set of loaded companies has changed. Load the same companies, under the same names, then call post_ledger_creates with this batch_id again: it reads the book back and sends nothing. Do not create the ledger in Tally by hand meanwhile. Never post this batch again.";
/// The batch's record is not one the read-back can describe.
pub(super) const NEXT_RECORD_NOT_READABLE: &str = "ComplyEaze Bridge cannot read this batch back: its record is not one it can describe. Nothing was sent now. Compare the ledger in Tally with what was approved, by eye, and do not create it by hand from this batch. Never post this batch again.";
pub(super) const NEXT_RECONCILE: &str = "The read-back did not settle the outcome. Check the ledger in Tally against the reasons given, then call post_ledger_creates with this batch_id again to read it back (it sends nothing). Do not create the ledger in Tally by hand meanwhile. Never post this batch again.";
const NOTE_VERIFIED: &str = "ComplyEaze Bridge read the book back after the one create: exactly one ledger of this name, new, with the parent, tax fields and GSTIN as sent and no opening balance (bill-wise is not read back); no ledger or group that existed before changed; and no other loaded company's master mark moved.";
const NOTE_ALERT_EXISTING: &str =
    "ATTENTION: the read-back shows an existing master may have been changed by this create.";
const NOTE_ALERT_ELSEWHERE: &str = "ATTENTION: the read-back shows a master mark moved in a loaded company other than the one approved.";
const NOTE_UNATTRIBUTED: &str = "A later read of the book found the ledger as sent (bill-wise is not read back), its ALTERID showing it as the first master change in this company after ComplyEaze Bridge's last read of it before sending the create, and no ledger or group that existed before changed. No answer from Tally to the create is on file, so the create is NOT verified: nothing shows the change was ComplyEaze Bridge's own.";
const NOTE_ABSENT: &str = "A later read of the book found no ledger of this name, no ledger or group that existed before changed and no loaded company's master mark moved since the create was recorded. A create that Tally rejected reaches this state too. The read's basis is the order Tally serves requests in, seen only for requests on new connections, on TallyPrime 7.2 Silver, for ledger creates: it does not show that Tally will not still run a create that ComplyEaze Bridge stopped waiting for.";
const NOTE_ABSENT_AFTER_CHANGE: &str = "A later read of the book found no ledger of this name, no ledger or group that existed before changed or went missing, and no ledger or group that was not there before the create was sent; no other loaded company's master mark moved. This company's master mark moved forward since the create was recorded, so checks.target_mark_moved reads passed: that shows a master in this company changed, not that this create ran. The read's basis is the order Tally serves requests in, seen only for requests on new connections, on TallyPrime 7.2 Silver, for ledger creates: it does not show that Tally will not still run a create that ComplyEaze Bridge stopped waiting for.";
const NOTE_UNCONFIRMED: &str = "The book could not be read back, so the create is NOT verified.";
const NOTE_RECONCILE: &str =
    "The read-back did not settle the outcome: the create is NOT verified.";
/// The step added when the verdict could not be written to the journal, after
/// the step the state has of its own. A later call reads the book back again
/// and sends nothing; that is not posting the batch again.
pub(super) const NEXT_VERDICT_NOT_RECORDED: &str = "ComplyEaze Bridge could not record this verdict, so this batch is not settled. Call post_ledger_creates with this batch_id again to read the book back and record it: that call sends nothing and is not posting the batch again, which must never be done.";
/// Added to that step when Tally's answer is not recorded either: a later call
/// may have no counters, so this answer's are the only record to rely on.
pub(super) const NEXT_ANSWER_NOT_RECORDED: &str = " Tally's answer is not on file either (if its bytes reached the journal after all, a later call will have the counters, but do not rely on it): keep the counters this answer reports, which are the only record to rely on, and check the ledger in Tally.";
/// The step when Tally answered and the call could not go on to read the book
/// back, or to read the journal at all. A later call reads and sends nothing.
pub(super) const NEXT_NOT_READ_BACK_AFTER_ANSWER: &str = "Tally answered the one create and the book was not read back. Call post_ledger_creates with this batch_id again later: it reads the book back and sends nothing. Do not create the ledger in Tally by hand meanwhile. Posting this batch again must never be done.";
/// What a read-back that has no recorded answer to take counters from says.
pub(super) const NO_ANSWER_RECORDED_NOTE: &str = "ComplyEaze Bridge holds no record of Tally's answer for this batch, so this read-back has no counters and cannot verify the create. If an earlier call reported counters for it, those counters stand. Check the ledger in Tally.";
pub(super) const COUNTERS_ABSENT_NOTE: &str = "Tally's counters were not available for this create, so it cannot be verified: a ledger of the same name made by someone else just before this create could have been overwritten with every other check still passing. Check the ledger in Tally.";

/// What a caller does next, by verdict.
pub(super) fn settled_next_step(state: MasterVerdictState) -> &'static str {
    match state {
        MasterVerdictState::PostedVerified => NEXT_VERIFIED,
        MasterVerdictState::PostedIntoExistingMaster => NEXT_INTO_EXISTING,
        MasterVerdictState::MasterWriteLocatedElsewhere => NEXT_ELSEWHERE,
        MasterVerdictState::ConfirmedAbsent => NEXT_ABSENT,
        MasterVerdictState::PostedUnattributed => NEXT_UNATTRIBUTED,
        MasterVerdictState::ConfirmedAbsentAfterChange => NEXT_ABSENT_AFTER_CHANGE,
        MasterVerdictState::MastersAfterPostUnconfirmed => NEXT_UNCONFIRMED,
        MasterVerdictState::ReconciliationRequired => NEXT_RECONCILE,
    }
}

/// What a caller does next, by what the read-back judged. No step tells the
/// person to create the ledger in Tally by hand.
pub(super) fn judged_next_step(judgement: &Judgement, moment: Moment) -> &'static str {
    if judgement.state() != MasterVerdictState::ReconciliationRequired {
        return settled_next_step(judgement.state());
    }
    if judgement.reasons().contains(&"company_scope_changed") {
        NEXT_LOAD_SAME_COMPANIES
    } else if judgement.reasons().iter().any(|reason| {
        matches!(
            *reason,
            "absent_not_measured_in_education"
                | "absent_not_measured_on_gold"
                | "absent_not_measured_on_this_release"
                | "absent_tally_release_not_known"
        )
    }) {
        NEXT_ABSENT_UNCOVERED
    } else if moment == Moment::Reconcile && judgement.name_absent_in_read_book() {
        NEXT_ABSENT_NOT_SETTLED
    } else if judgement.guid().is_some() {
        NEXT_PRESENT_NOT_VERIFIED
    } else {
        NEXT_RECONCILE
    }
}

/// What a caller does next when the book could not be read back at all, by the
/// code of what stopped it.
pub(super) fn not_read_back_next_step(code: &str) -> &'static str {
    match code {
        "company_identity_not_found"
        | "company_identity_ambiguous"
        | "company_identity_incomplete"
        | "company_identity_mismatch"
        | "company_number_invalid" => NEXT_LOAD_SAME_COMPANIES,
        "master_batch_invalid" | "master_intent_missing" => NEXT_RECORD_NOT_READABLE,
        _ => NEXT_UNCONFIRMED,
    }
}

/// What the read-back says about the outcome, by verdict.
pub(super) fn settled_note(state: MasterVerdictState) -> &'static str {
    match state {
        MasterVerdictState::PostedVerified => NOTE_VERIFIED,
        MasterVerdictState::PostedIntoExistingMaster => NOTE_ALERT_EXISTING,
        MasterVerdictState::MasterWriteLocatedElsewhere => NOTE_ALERT_ELSEWHERE,
        MasterVerdictState::ConfirmedAbsent => NOTE_ABSENT,
        MasterVerdictState::PostedUnattributed => NOTE_UNATTRIBUTED,
        MasterVerdictState::ConfirmedAbsentAfterChange => NOTE_ABSENT_AFTER_CHANGE,
        MasterVerdictState::MastersAfterPostUnconfirmed => NOTE_UNCONFIRMED,
        MasterVerdictState::ReconciliationRequired => NOTE_RECONCILE,
    }
}

/// What a caller does next when the verdict was judged but not written. A loud
/// state keeps its own step first: the person still has to look at Tally.
pub(super) fn unrecorded_next_step(
    judgement: &Judgement,
    moment: Moment,
    answer_recorded: bool,
) -> String {
    // A state that is open, or loud, has a step of its own that is true whether
    // or not its verdict was written: the check to make in Tally. A quiet final
    // state (verified, absent) has none that is true while unsettled.
    let own = !judgement.state().is_terminal() || settled_alert(judgement.state()).is_some();
    let mut step = if own {
        format!(
            "{} {NEXT_VERDICT_NOT_RECORDED}",
            judged_next_step(judgement, moment)
        )
    } else {
        NEXT_VERDICT_NOT_RECORDED.to_string()
    };
    if moment == Moment::JustPosted && !answer_recorded {
        step.push_str(NEXT_ANSWER_NOT_RECORDED);
    }
    step
}

/// The loud states lead the answer with an alert.
pub(super) fn settled_alert(state: MasterVerdictState) -> Option<&'static str> {
    match state {
        MasterVerdictState::PostedIntoExistingMaster => Some(NOTE_ALERT_EXISTING),
        MasterVerdictState::MasterWriteLocatedElsewhere => Some(NOTE_ALERT_ELSEWHERE),
        MasterVerdictState::PostedVerified
        | MasterVerdictState::ConfirmedAbsent
        | MasterVerdictState::PostedUnattributed
        | MasterVerdictState::ConfirmedAbsentAfterChange
        | MasterVerdictState::MastersAfterPostUnconfirmed
        | MasterVerdictState::ReconciliationRequired => None,
    }
}

/// Tally's answer to the one create, as the call that made the POST holds it.
pub(super) struct PostedAnswer<'a> {
    /// The counters to judge on and to report.
    pub(super) counters: Option<MasterCounters>,
    /// The answer, when appending it to the journal did not succeed: the verdict
    /// is written only after it, under the one lock.
    pub(super) unrecorded: Option<&'a MasterDispatchResponse>,
}

impl PostedAnswer<'_> {
    /// What the journal holds: a later call has nothing but this to go on.
    pub(super) fn recorded(state: &BatchState) -> Self {
        Self {
            counters: state
                .response
                .as_ref()
                .and_then(|response| response.counters),
            unrecorded: None,
        }
    }
}

/// What one verification left in the journal.
struct Recording {
    /// `Ok(true)`: this call appended the verdict. `Ok(false)`: the journal
    /// already held it in the same words. `Err`: it is not recorded, with the
    /// code of what stopped it.
    verdict: Result<bool, String>,
    /// Whether the journal holds Tally's answer for the batch.
    answer_recorded: bool,
}

fn marks_failure(error: &anyhow::Error) -> String {
    if crate::endpoint_wire::wire_refusal(error) == Some(bridge_tally_transport::WireRefusal::Busy)
    {
        "tally_endpoint_busy".to_string()
    } else {
        "read_failed".to_string()
    }
}

/// The part that reads the ledgers of one parent group, and no other.
fn single_parent_part(parent: &str) -> Option<ParentPart> {
    let limits = PartitionLimits {
        max_ledgers_per_part: 1,
        max_parents_per_part: 1,
        max_parts: 1,
        max_complement_formula_bytes: 0,
    };
    ParentPartition::plan(
        [("probe", "probe", ParentObservation::Named(parent))],
        limits,
    )
    .ok()?
    .parts()
    .first()
    .cloned()
}

#[cfg(test)]
tokio::task_local! {
    /// The coverage a tool test says the read-back's Tally has, standing in
    /// for a captured company list of that release. Absent, the coverage is
    /// read from the answer as it is in a shipped build.
    pub(super) static TEST_ORDER_COVERAGE: OrderCoverage;
}

/// What a read-back came to, with a status for each read that carries no book data.
struct BookBack {
    read: ReadBack,
    status: Value,
}

fn unavailable(code: &str) -> Value {
    json!(format!("unavailable:{code}"))
}

impl Server {
    /// Reads the book back. Never sends a create.
    async fn read_master_book_back(
        &self,
        identity: &VerifiedCompanyIdentity,
        item: &LedgerCreateItem,
        pre_image: &BTreeMap<String, u64>,
        call_started: std::time::Instant,
        evidence: &mut Evidence,
    ) -> BookBack {
        let company = identity.display_name();
        let mut status = json!({
            "marks": "not_attempted",
            "ledger_index": "not_attempted",
            "group_index": "not_attempted",
            "index_parse": "not_attempted",
            "party_read": "not_attempted",
        });
        // 1. Marks, first: the closest read to the POST.
        let marks: Option<Vec<MasterMark>> = async {
            let request = crate::tally::agent_read_request::AgentReadRequest::parse(
                crate::agent::read_profiles::render_agent_company_high_water(company),
            )
            .map_err(|_| "read_failed".to_string())?;
            let text = self
                .read_marks_after_post(request, call_started)
                .await
                .map_err(|error| marks_failure(&error))?;
            super::super::master_post::aim_master_marks(&text).map_err(|_| "unparsed".to_string())
        }
        .await
        .map_err(|code| status["marks"] = unavailable(&code))
        .ok();
        if marks.is_some() {
            status["marks"] = json!("read");
        }
        // 2. The paired indexes.
        let mut masters: Option<Vec<IndexedMaster>> = None;
        let mut profile = None;
        // Not measured until the index read says which Tally answered.
        let mut order_coverage = OrderCoverage::NotMeasured("absent_tally_release_not_known");
        let ledgers = match ledger_name_index_read(company) {
            Ok(request) => self.post_read_observing_tally(identity, request).await,
            Err(_) => Err(ToolFailure::from("master_request_invalid".to_string())),
        };
        match ledgers {
            Err(failure) => status["ledger_index"] = unavailable(&failure.code),
            Ok((ledgers_xml, ledger_evidence, boundary, tally)) => {
                status["ledger_index"] = json!("read");
                order_coverage = OrderCoverage::of(&tally);
                // The one seam: a tool test says which Tally answered, where
                // its captured company list is of another release. It is
                // compiled under bare `cfg(test)`, so no shipped build holds it.
                #[cfg(test)]
                if let Ok(forced) = TEST_ORDER_COVERAGE.try_with(|coverage| *coverage) {
                    order_coverage = forced;
                }
                *evidence = combine_evidence(evidence.clone(), ledger_evidence);
                profile = Some(boundary);
                match self
                    .post_read(identity, native_group_snapshot_read(company))
                    .await
                {
                    Err(failure) => status["group_index"] = unavailable(&failure.code),
                    Ok((groups_xml, group_evidence)) => {
                        status["group_index"] = json!("read");
                        *evidence = combine_evidence(evidence.clone(), group_evidence);
                        match super::super::master_post::parse_readback_indexes(
                            &ledgers_xml,
                            &groups_xml,
                            identity.company_guid(),
                        ) {
                            Err(error) => {
                                status["index_parse"] = unavailable(refusal_code(&error));
                            }
                            Ok((ledgers, groups)) => {
                                status["index_parse"] = json!("parsed");
                                masters = Some([ledgers, groups].concat());
                            }
                        }
                    }
                }
            }
        }
        // 3. The party master of the item's parent, if there is a ledger for it
        // to describe.
        let party = match (&masters, profile) {
            (Some(masters), Some(boundary)) => {
                if party_read_needed(item, pre_image, masters) {
                    let rows = self
                        .read_party_rows(identity, item, boundary, evidence)
                        .await;
                    match rows {
                        Ok(rows) => {
                            status["party_read"] = json!("read");
                            PartyRead::Rows(rows)
                        }
                        Err(code) => {
                            status["party_read"] = unavailable(&code);
                            PartyRead::Unavailable
                        }
                    }
                } else {
                    status["party_read"] = json!("not_needed");
                    PartyRead::NotRead
                }
            }
            // The index is the only reason to read it: without one, the verdict
            // is already unconfirmed.
            _ => PartyRead::NotRead,
        };
        BookBack {
            read: ReadBack {
                marks,
                masters,
                party,
                order_coverage,
            },
            status,
        }
    }

    async fn read_party_rows(
        &self,
        identity: &VerifiedCompanyIdentity,
        item: &LedgerCreateItem,
        boundary: DateBoundaryProfile,
        evidence: &mut Evidence,
    ) -> Result<Vec<PartyView>, String> {
        let books_from =
            bridge_tally_core::TallyDate::parse(identity.books_from_yyyymmdd().to_string())
                .map_err(|_| "period_invalid".to_string())?;
        let period = NativeLedgerExportPeriod::new(boundary, books_from.clone(), books_from)
            .map_err(|_| "period_unsupported".to_string())?;
        let part = single_parent_part(item.parent()).ok_or("parent_unsupported".to_string())?;
        let request = crate::agent::read_profiles::party_ledger_master_for_parent_read(
            identity.display_name(),
            &period,
            &part,
        );
        let (xml, read_evidence) = self
            .post_read(identity, request)
            .await
            .map_err(|failure| failure.code)?;
        *evidence = combine_evidence(evidence.clone(), read_evidence);
        let parsed = bridge_tally_protocol::parse_native_party_ledger_master_records_with_evidence(
            &xml,
            identity.company_guid(),
        )
        .map_err(|_| "unparsed".to_string())?;
        Ok(parsed
            .records
            .iter()
            .filter_map(PartyView::project)
            .collect())
    }

    /// Reads the batch's book back, judges it, records the verdict and returns
    /// the `result` object of the answer. `Err` is a code for a batch the
    /// journal itself cannot describe (no intent, no admitted item): the caller
    /// then reports the journal alone.
    pub(super) async fn verify_master_batch(
        &self,
        state: &BatchState,
        moment: Moment,
        identity: &VerifiedCompanyIdentity,
        call_started: std::time::Instant,
        evidence: &mut Evidence,
        answer: PostedAnswer<'_>,
    ) -> Result<Value, String> {
        let item = item_from_batch(&state.batch)?;
        let intent = state
            .intent
            .as_ref()
            .ok_or_else(|| "master_intent_missing".to_string())?;
        let counters = answer.counters;
        let pre_image = read_pre_image(
            &self.settings.data_dir,
            &state.batch.batch_id,
            intent.pre_image_sha256(),
            intent.pre_image_count(),
        );
        let (judgement, status) = match pre_image {
            // Without the pre-image nothing can be said about what changed.
            Err(_) => (
                Judgement::open(
                    super::verdict::OpenVerdict::ReconciliationRequired,
                    "pre_image_unreadable",
                ),
                json!({"state": "not_attempted"}),
            ),
            Ok(pre_image) => {
                let back = self
                    .read_master_book_back(
                        identity,
                        &item,
                        pre_image.entries(),
                        call_started,
                        evidence,
                    )
                    .await;
                let company_guid = identity.company_guid().to_ascii_lowercase();
                let judgement = judge(
                    &Facts {
                        item: &item,
                        company_guid: &company_guid,
                        pre_image: pre_image.entries(),
                        pre_marks: intent.pre_post_master_marks(),
                        counters,
                        // The call that made the POST holds Tally's answer
                        // whether or not it could record it.
                        answer_held: moment == Moment::JustPosted
                            || state.response.is_some()
                            || answer.unrecorded.is_some(),
                        moment,
                    },
                    &back.read,
                );
                (judgement, back.status)
            }
        };
        let recorded = self.record_master_verdict(state, &judgement, answer.unrecorded);
        Ok(verification_result(
            state, &item, &judgement, status, &recorded, moment, counters,
        ))
    }

    /// The answer for a reconcile that could not take the observation lease:
    /// another process is dispatching on this endpoint, and a create of this
    /// very batch may be on its way to the wire. Nothing is read; the outcome
    /// is unconfirmed, never absent.
    pub(super) fn unconfirmed_while_dispatch_is_busy(
        &self,
        state: &BatchState,
        lease_code: &str,
    ) -> Result<Value, String> {
        let item = item_from_batch(&state.batch)?;
        let judgement = Judgement::open(
            super::verdict::OpenVerdict::MastersAfterPostUnconfirmed,
            "endpoint_dispatch_busy",
        );
        let recorded = self.record_master_verdict(state, &judgement, None);
        Ok(verification_result(
            state,
            &item,
            &judgement,
            json!({"state": "not_attempted", "observation_lease": unavailable(lease_code)}),
            &recorded,
            Moment::Reconcile,
            PostedAnswer::recorded(state).counters,
        ))
    }

    /// Appends the verdict to the masters journal, under the one exclusive lock
    /// that is tried and never waited for. An open verdict the journal already
    /// holds in the same words is not written again. When Tally's answer is
    /// still unrecorded and the journal holds none, it is appended first, and a
    /// verdict is never written without it: the journal never holds a verdict
    /// judged on counters it does not hold.
    fn record_master_verdict(
        &self,
        state: &BatchState,
        judgement: &Judgement,
        unrecorded: Option<&MasterDispatchResponse>,
    ) -> Recording {
        let verdict = MasterVerdict::judged(&state.batch, judgement);
        let mut answer_recorded = state.response.is_some();
        let verdict = self.append_under_lock(state, verdict, unrecorded, &mut answer_recorded);
        Recording {
            verdict,
            answer_recorded,
        }
    }

    /// The appends of [`Self::record_master_verdict`]. `answer_recorded` ends as
    /// what the journal read under the lock holds, once the answer is appended.
    pub(super) fn append_under_lock(
        &self,
        state: &BatchState,
        verdict: MasterVerdict,
        unrecorded: Option<&MasterDispatchResponse>,
        answer_recorded: &mut bool,
    ) -> Result<bool, String> {
        let data_dir = &self.settings.data_dir;
        // An open verdict the journal already held, in the same words, stays
        // recorded when the lock cannot be taken or the journal cannot be read:
        // nothing is left to write. Only a failure before the journal is read
        // under the lock is taken so; a failed append is never.
        let held_in_state = unrecorded.is_none() && state.verdict.as_ref() == Some(&verdict);
        let locked = (|| {
            #[cfg(test)]
            if super::super::master_ledger::test_fault_fires("verdict_lock_busy") {
                return Err("import_admission_busy".to_string());
            }
            let lock = ExclusiveAdmission::acquire(data_dir)?;
            let journal = read_journal(data_dir).map_err(|error| error.code().to_string())?;
            Ok((lock, journal))
        })();
        let (lock, journal) = match locked {
            Ok(locked) => locked,
            Err(_) if held_in_state => return Ok(false),
            Err(code) => return Err(code),
        };
        let held = journal
            .state(&state.batch.batch_id)
            .ok_or_else(|| "master_batch_not_found".to_string())?;
        *answer_recorded = held.response.is_some();
        if let (Some(response), false) = (unrecorded, *answer_recorded) {
            append(
                data_dir,
                &lock,
                MasterRecord::DispatchResponse(response.clone()),
            )
            .map_err(|error| error.code().to_string())?;
            *answer_recorded = true;
        }
        if held.verdict.as_ref() == Some(&verdict) {
            return Ok(false);
        }
        append(data_dir, &lock, MasterRecord::Verdict(verdict))
            .map_err(|error| error.code().to_string())?;
        Ok(true)
    }
}

/// The `result` object of a verified (or doubted) batch. Nothing in it is a
/// client value but the batch the person approved: states, GUIDs, check
/// outcomes, counters and static reasons.
fn verification_result(
    state: &BatchState,
    item: &LedgerCreateItem,
    judgement: &Judgement,
    read_back: Value,
    recorded: &Recording,
    moment: Moment,
    counters: Option<MasterCounters>,
) -> Value {
    let alert = settled_alert(judgement.state());
    let counters = counters.as_ref();
    let verdict_recorded = recorded.verdict.is_ok();
    let next_step = if verdict_recorded {
        judged_next_step(judgement, moment).to_string()
    } else {
        unrecorded_next_step(judgement, moment, recorded.answer_recorded)
    };
    let mut result = json!({
        "batch_id": state.batch.batch_id,
        "state": verdict_state_code(judgement.state()),
        "attempt_recorded": true,
        // Whether the journal holds Tally's answer for this batch.
        "answer_recorded": recorded.answer_recorded,
        "dispatch": {"state": "dispatched", "resent": false},
        "verification": {
            "moment": match moment {
                Moment::JustPosted => "after_post",
                Moment::Reconcile => "reconcile",
            },
            // Settled is the journal's word: a terminal state whose verdict is
            // not recorded is judged, and not yet settled.
            "settled": judgement.state().is_terminal() && verdict_recorded,
            "guid": judgement.guid(),
            "checks": judgement.checks(),
            "reasons": judgement.reasons(),
            "read_back": read_back,
            // Recorded: the journal holds this verdict. Written: this call
            // appended it (an open verdict already held in the same words is not
            // written again).
            "verdict_recorded": verdict_recorded,
            "verdict_written": matches!(recorded.verdict, Ok(true)),
        },
        "counters": counters_json(counters),
        "next_step": next_step,
        "note": settled_note(judgement.state()),
    });
    if let Err(code) = &recorded.verdict {
        result["verification"]["verdict_record_error"] = json!(code);
    }
    if counters.is_none() {
        // A read-back with no recorded answer has no counters of its own, and
        // an earlier call may have reported them.
        result["verification"]["counters_note"] =
            if moment == Moment::Reconcile && !recorded.answer_recorded {
                json!(NO_ANSWER_RECORDED_NOTE)
            } else {
                json!(COUNTERS_ABSENT_NOTE)
            };
    }
    if let AdmittedLedgerKind::Party {
        bill_wise: true, ..
    } = item.kind()
    {
        // The party read does not fetch ISBILLWISEON: said, not assumed.
        result["verification"]["not_read_back"] = json!(["bill_wise"]);
    }
    if judgement.state().is_absent() {
        // What an absent state rests on: a read sent after the create's send
        // ended was served after the create (the order Tally serves requests
        // in), not the name being absent, which says only what the book holds.
        result["verification"]["basis"] = json!("read_after_send_order");
    }
    // "alert" sorts first among the keys: the loud states lead the answer.
    if let Some(alert) = alert {
        result["alert"] = json!(alert);
    }
    json!({ "result": result })
}
