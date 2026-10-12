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
use super::super::master_ledger::{
    read_pre_image, MasterMark, MasterVerdict, MasterVerdictState, VerdictItem,
};
use super::verdict::{
    judge, party_read_needed, Facts, Judgement, Moment, PartyRead, PartyView, ReadBack,
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
const NEXT_VERIFIED: &str = "The ledger exists as sent and nothing else changed. Nothing more is needed for this batch; never post it again.";
const NEXT_INTO_EXISTING: &str = "Open the company in Tally now and check the ledger of this name, its group and the ledgers around it: a ledger that existed before may have been changed. Do not retry and do not post this batch again.";
const NEXT_ELSEWHERE: &str = "Open Tally now and check every loaded company: the change may have landed in a company other than the one approved. Do not retry and do not post this batch again.";
const NEXT_ABSENT: &str = "Nothing was created in the book and this batch is closed. Nothing is sent again automatically. To create the ledger, call prepare_ledger_creates for a new batch: it needs a new approval.";
const NEXT_UNCONFIRMED: &str = "The ledger could not be read back this time. Call post_ledger_creates with this batch_id again to read it back (it sends nothing), or check the company's ledgers in Tally. Never post this batch again.";
const NEXT_RECONCILE: &str = "The read-back did not settle the outcome. Check the ledger in Tally against the reasons given, then call post_ledger_creates with this batch_id again to read it back (it sends nothing). Never post this batch again.";
const NOTE_VERIFIED: &str = "ComplyEaze Bridge read the book back after the one create: exactly one ledger of this name, new, with the parent, tax fields and GSTIN as sent and no opening balance, and no other master or company changed.";
const NOTE_ALERT_EXISTING: &str =
    "ATTENTION: the read-back shows an existing master may have been changed by this create.";
const NOTE_ALERT_ELSEWHERE: &str = "ATTENTION: the read-back shows a master mark moved in a loaded company other than the one approved.";
const NOTE_ABSENT: &str = "The read-back found no ledger of this name, no existing master changed and no loaded company's master mark moved since the create was recorded.";
const NOTE_UNCONFIRMED: &str = "The book could not be read back, so the create is NOT verified.";
const NOTE_RECONCILE: &str =
    "The read-back did not settle the outcome: the create is NOT verified.";
const COUNTERS_ABSENT_NOTE: &str = "Tally's counters were not available for this create, so it cannot be verified: a ledger of the same name made by someone else just before this create could have been overwritten with every other check still passing. Check the ledger in Tally.";

/// What a caller does next, by verdict.
pub(super) fn settled_next_step(state: MasterVerdictState) -> &'static str {
    match state {
        MasterVerdictState::PostedVerified => NEXT_VERIFIED,
        MasterVerdictState::PostedIntoExistingMaster => NEXT_INTO_EXISTING,
        MasterVerdictState::MasterWriteLocatedElsewhere => NEXT_ELSEWHERE,
        MasterVerdictState::ConfirmedAbsent => NEXT_ABSENT,
        MasterVerdictState::MastersAfterPostUnconfirmed => NEXT_UNCONFIRMED,
        MasterVerdictState::ReconciliationRequired => NEXT_RECONCILE,
    }
}

/// The loud states lead the answer with an alert.
pub(super) fn settled_alert(state: MasterVerdictState) -> Option<&'static str> {
    match state {
        MasterVerdictState::PostedIntoExistingMaster => Some(NOTE_ALERT_EXISTING),
        MasterVerdictState::MasterWriteLocatedElsewhere => Some(NOTE_ALERT_ELSEWHERE),
        MasterVerdictState::PostedVerified
        | MasterVerdictState::ConfirmedAbsent
        | MasterVerdictState::MastersAfterPostUnconfirmed
        | MasterVerdictState::ReconciliationRequired => None,
    }
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
        let ledgers = match ledger_name_index_read(company) {
            Ok(request) => self.post_read_observing_boundary(identity, request).await,
            Err(_) => Err(ToolFailure::from("master_request_invalid".to_string())),
        };
        match ledgers {
            Err(failure) => status["ledger_index"] = unavailable(&failure.code),
            Ok((ledgers_xml, ledger_evidence, boundary)) => {
                status["ledger_index"] = json!("read");
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
    ) -> Result<Value, String> {
        let item = item_from_batch(&state.batch)?;
        let intent = state
            .intent
            .as_ref()
            .ok_or_else(|| "master_intent_missing".to_string())?;
        let counters = state
            .response
            .as_ref()
            .and_then(|response| response.counters);
        let pre_image = read_pre_image(
            &self.settings.data_dir,
            &state.batch.batch_id,
            intent.pre_image_sha256(),
            intent.pre_image_count(),
        );
        let (judgement, status) = match pre_image {
            // Without the pre-image nothing can be said about what changed.
            Err(_) => (
                Judgement {
                    state: MasterVerdictState::ReconciliationRequired,
                    guid: None,
                    checks: super::verdict::Checks::none(),
                    reasons: vec!["pre_image_unreadable"],
                },
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
                        moment,
                    },
                    &back.read,
                );
                (judgement, back.status)
            }
        };
        let recorded = self.record_master_verdict(state, &judgement);
        Ok(verification_result(
            state, &item, &judgement, status, &recorded, moment,
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
        let judgement = Judgement {
            state: MasterVerdictState::MastersAfterPostUnconfirmed,
            guid: None,
            checks: super::verdict::Checks::none(),
            reasons: vec!["endpoint_dispatch_busy"],
        };
        let recorded = self.record_master_verdict(state, &judgement);
        Ok(verification_result(
            state,
            &item,
            &judgement,
            json!({"state": "not_attempted", "observation_lease": unavailable(lease_code)}),
            &recorded,
            Moment::Reconcile,
        ))
    }

    /// Appends the verdict to the masters journal. An open verdict the journal
    /// already holds in the same words is not written again.
    fn record_master_verdict(
        &self,
        state: &BatchState,
        judgement: &Judgement,
    ) -> Result<bool, String> {
        let reason = (judgement.state != MasterVerdictState::PostedVerified)
            .then(|| judgement.reasons.join(","));
        let verdict = MasterVerdict {
            batch_id: state.batch.batch_id.clone(),
            batch_sha256: state.batch.batch_sha256.clone(),
            state: judgement.state,
            items: state
                .batch
                .items
                .iter()
                .map(|item| VerdictItem {
                    name: item.name.clone(),
                    guid: judgement.guid.clone(),
                    reason: reason.clone(),
                })
                .collect(),
        };
        if state.verdict.as_ref() == Some(&verdict) {
            return Ok(false);
        }
        let lock = ExclusiveAdmission::acquire(&self.settings.data_dir)?;
        append(
            &self.settings.data_dir,
            &lock,
            MasterRecord::Verdict(verdict),
        )
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
    recorded: &Result<bool, String>,
    moment: Moment,
) -> Value {
    let next_step = settled_next_step(judgement.state);
    let alert = settled_alert(judgement.state);
    let note = match judgement.state {
        MasterVerdictState::PostedVerified => NOTE_VERIFIED,
        MasterVerdictState::PostedIntoExistingMaster => NOTE_ALERT_EXISTING,
        MasterVerdictState::MasterWriteLocatedElsewhere => NOTE_ALERT_ELSEWHERE,
        MasterVerdictState::ConfirmedAbsent => NOTE_ABSENT,
        MasterVerdictState::MastersAfterPostUnconfirmed => NOTE_UNCONFIRMED,
        MasterVerdictState::ReconciliationRequired => NOTE_RECONCILE,
    };
    let counters = state
        .response
        .as_ref()
        .and_then(|response| response.counters.as_ref());
    let mut result = json!({
        "batch_id": state.batch.batch_id,
        "state": verdict_state_code(judgement.state),
        "attempt_recorded": true,
        "dispatch": {"state": "dispatched", "resent": false},
        "verification": {
            "moment": match moment {
                Moment::JustPosted => "after_post",
                Moment::Reconcile => "reconcile",
            },
            "settled": judgement.state.is_terminal(),
            "guid": judgement.guid,
            "checks": judgement.checks,
            "reasons": judgement.reasons,
            "read_back": read_back,
            // Recorded: the journal holds this verdict. Written: this call
            // appended it (an open verdict already held in the same words is not
            // written again).
            "verdict_recorded": recorded.is_ok(),
            "verdict_written": matches!(recorded, Ok(true)),
        },
        "counters": counters_json(counters),
        "next_step": next_step,
        "note": note,
    });
    if let Err(code) = recorded {
        result["verification"]["verdict_record_error"] = json!(code);
    }
    if counters.is_none() {
        result["verification"]["counters_note"] = json!(COUNTERS_ABSENT_NOTE);
    }
    if let AdmittedLedgerKind::Party {
        bill_wise: true, ..
    } = item.kind()
    {
        // The party read does not fetch ISBILLWISEON: said, not assumed.
        result["verification"]["not_read_back"] = json!(["bill_wise"]);
    }
    // "alert" sorts first among the keys: the loud states lead the answer.
    if let Some(alert) = alert {
        result["alert"] = json!(alert);
    }
    json!({ "result": result })
}
